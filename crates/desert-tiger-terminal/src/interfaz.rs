use std::cell::Cell;
use std::collections::BTreeSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::animacion::{PASO_ANIMACION, PASO_FOGATA, RelojVisual};
use crate::audio::{AmbienteSonoro, AvisoAudio};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use desert_tiger_app::{
    AccionDisponible, CausaIndisponibilidad, ColorRetrato, EntradaResumenNpc, ErrorIntencion,
    EstiloCeldaRetrato, GraficoRetrato, IdLugar, IntencionFrontend, Partida, VistaActor,
    VistaDecision, VistaFragmento, VistaMundo, VistaSesion,
};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use unicode_width::UnicodeWidthChar;

use crate::recursos::{NEGRO_PURO, RecursosVisuales};

const ANCHO_MINIMO: u16 = 80;
const ALTO_MINIMO: u16 = 24;
const ANCHO_TEXTO_VINETA: usize = 66;
const LINEAS_POR_PAGINA: usize = 8;
const OPCIONES_POR_PAGINA: usize = 1;

#[derive(Clone)]
enum ModoLocal {
    Normal,
    Seleccion { destinos: Vec<IdLugar>, foco: usize },
}

pub(crate) struct Interfaz {
    partida: Partida,
    vista: VistaSesion,
    ultima_vista_mundo: VistaMundo,
    modo: ModoLocal,
    confirmar_salida: bool,
    salir: bool,
    error_recuperable: Option<String>,
    mostrar_detalle: bool,
    pagina: usize,
    pagina_antes_detalle: usize,
    reloj: RelojVisual,
    campamento: Option<crate::campamento::EstadoCampamento>,
    musica: crate::musica_campamento::MusicaCampamento,
    aviso_audio: Option<AvisoAudio>,
    aviso_audio_mostrado: Cell<bool>,
}

impl Interfaz {
    pub(crate) fn nueva(partida: Partida) -> Result<Self, String> {
        let vista = partida.vista();
        let VistaSesion::Disponible(mundo) = &vista else {
            return Err("la partida no comenzó en una vista disponible".to_owned());
        };
        let ultima_vista_mundo = mundo.clone();
        Ok(Self {
            partida,
            vista,
            ultima_vista_mundo,
            modo: ModoLocal::Normal,
            confirmar_salida: false,
            salir: false,
            error_recuperable: None,
            mostrar_detalle: false,
            pagina: 0,
            pagina_antes_detalle: 0,
            reloj: RelojVisual::default(),
            campamento: None,
            musica: crate::musica_campamento::MusicaCampamento::default(),
            aviso_audio: None,
            aviso_audio_mostrado: Cell::new(false),
        })
    }

    pub(crate) fn debe_salir(&self) -> bool {
        self.salir
    }

    pub(crate) fn ambiente_sonoro(&self) -> AmbienteSonoro {
        if self.salir || matches!(self.vista, VistaSesion::Fatal(_)) {
            return AmbienteSonoro::SILENCIO;
        }
        let lugar = match &self.vista {
            VistaSesion::PresentandoAccion(f) => f.progreso.lugar_confirmado,
            VistaSesion::EsperandoDecision(d) => d.progreso.lugar_confirmado,
            _ => self.ultima_vista_mundo.lugar_jugador,
        };
        AmbienteSonoro::del_lugar(
            &self.ultima_vista_mundo,
            lugar,
            matches!(self.vista, VistaSesion::Acampando(_)),
        )
    }

    pub(crate) fn actualizar_musica(
        &mut self,
        ahora: Duration,
        progreso: Option<crate::musica_campamento::ProgresoMusica>,
        disponible: bool,
        ajustes: Option<crate::musica_campamento::AgendaMusica>,
    ) {
        use crate::musica_campamento::candidatos;
        if let (VistaSesion::Acampando(vista), Some(ajustes)) = (&self.vista, ajustes)
            && disponible
            && !self.salir
        {
            if self.musica.campamento() != Some(vista.id) {
                self.musica
                    .abrir(vista.id, candidatos(&self.partida, vista), ahora, ajustes);
            }
            self.musica
                .actualizar(ahora, progreso, self.confirmar_salida, ajustes);
        } else {
            self.musica.cancelar();
        }
        if let Some(campamento) = &mut self.campamento {
            campamento.musica = self.musica.notas();
        }
    }
    pub(crate) fn solicitud_musica(&self) -> u32 {
        self.musica.solicitud()
    }

    pub(crate) fn recibir_aviso_audio(&mut self, aviso: AvisoAudio) {
        if self.aviso_audio.is_none() {
            self.aviso_audio = Some(aviso);
            self.aviso_audio_mostrado.set(false);
        }
    }

    fn aviso_audio_visible(&self) -> bool {
        self.aviso_audio.is_some()
            && !self.confirmar_salida
            && self.error_recuperable.is_none()
            && matches!(self.modo, ModoLocal::Normal)
            && matches!(
                self.vista,
                VistaSesion::Disponible(_) | VistaSesion::Acampando(_)
            )
    }

    pub(crate) fn diagnostico_fatal(&self) -> Option<String> {
        match &self.vista {
            VistaSesion::Fatal(fatal) => Some(fatal.mensaje.clone()),
            _ => None,
        }
    }

    pub(crate) fn avanzar_animacion(&mut self, transcurrido: Duration) {
        if !self.confirmar_salida {
            self.reloj.avanzar(transcurrido);
        }
    }

    pub(crate) fn espera_animacion(&self, recursos: &RecursosVisuales) -> Duration {
        if self.confirmar_salida {
            return PASO_ANIMACION;
        }
        let intervalo = if self.campamento.is_some()
            && (recursos.efecto_fogata.is_some() || self.musica.notas().is_some())
        {
            PASO_FOGATA
        } else {
            PASO_ANIMACION
        };
        self.reloj.hasta_siguiente(intervalo)
    }

    pub(crate) fn manejar_tecla(&mut self, evento: KeyEvent) {
        if evento.kind != KeyEventKind::Press {
            return;
        }
        if self.aviso_audio_mostrado.replace(false) {
            self.aviso_audio = None;
        }
        if self.confirmar_salida {
            match normalizar_letra(evento.code) {
                Some('s') => self.salir = true,
                Some('n') => self.confirmar_salida = false,
                _ if evento.code == KeyCode::Esc => self.confirmar_salida = false,
                _ => {}
            }
            return;
        }
        if normalizar_letra(evento.code) == Some('q') {
            self.confirmar_salida = true;
            return;
        }
        if let Some(campamento) = &mut self.campamento {
            if let Some(intencion) = campamento.manejar(evento.code, &self.partida) {
                self.enviar(intencion);
            }
            return;
        }
        if normalizar_letra(evento.code) == Some('d') && self.vista_tiene_detalle() {
            if self.mostrar_detalle {
                self.mostrar_detalle = false;
                self.pagina = self.pagina_antes_detalle;
            } else {
                self.mostrar_detalle = true;
                self.pagina_antes_detalle = self.pagina;
                self.pagina = 0;
            }
            return;
        }
        match &mut self.modo {
            ModoLocal::Seleccion { destinos, foco } => match evento.code {
                KeyCode::Up => *foco = foco.saturating_sub(1),
                KeyCode::Down if *foco + 1 < destinos.len() => *foco += 1,
                KeyCode::Esc => {
                    self.modo = ModoLocal::Normal;
                    self.error_recuperable = None;
                }
                KeyCode::Enter => {
                    let destino = destinos[*foco];
                    self.enviar(IntencionFrontend::ComenzarViaje { destino });
                    if !matches!(self.vista, VistaSesion::Disponible(_)) {
                        self.modo = ModoLocal::Normal;
                    }
                }
                _ => {}
            },
            ModoLocal::Normal => self.manejar_modo_semantico(evento.code),
        }
    }

    fn manejar_modo_semantico(&mut self, tecla: KeyCode) {
        if matches!(tecla, KeyCode::Left | KeyCode::PageUp) {
            self.pagina = self.pagina.saturating_sub(1);
            return;
        }
        if matches!(tecla, KeyCode::Right | KeyCode::PageDown) {
            let paginas = self.paginas_vigentes();
            self.pagina = (self.pagina + 1).min(paginas.saturating_sub(1));
            return;
        }
        // Consultar el desglose nunca responde una decisión ni confirma una escena.
        if self.mostrar_detalle {
            if tecla == KeyCode::Enter {
                self.pagina = (self.pagina + 1).min(self.paginas_vigentes().saturating_sub(1));
            }
            return;
        }
        match &self.vista {
            VistaSesion::Disponible(mundo)
                if normalizar_letra(tecla) == Some('a')
                    && mundo
                        .acciones
                        .iter()
                        .any(|a| matches!(a, AccionDisponible::Acampar)) =>
            {
                self.enviar(IntencionFrontend::ComenzarCampamento);
            }
            VistaSesion::Disponible(mundo) if normalizar_letra(tecla) == Some('v') => {
                if let Some(destinos) =
                    destinos_viaje(mundo).filter(|destinos| !destinos.is_empty())
                {
                    let mut destinos = destinos.to_vec();
                    destinos.sort_by_cached_key(|id| {
                        mundo
                            .lugares
                            .iter()
                            .find(|l| l.id == *id)
                            .map(|l| (l.nombre_abreviado.to_lowercase(), *id))
                    });
                    self.modo = ModoLocal::Seleccion {
                        foco: mundo
                            .lugares
                            .iter()
                            .find(|lugar| lugar.id == mundo.lugar_jugador)
                            .and_then(|lugar| {
                                foco_mas_cercano(
                                    &destinos,
                                    &mundo.lugares,
                                    lugar.coordenada_central,
                                )
                            })
                            .unwrap_or(0),
                        destinos,
                    };
                    self.error_recuperable = None;
                }
            }
            VistaSesion::PresentandoAccion(fragmento) if tecla == KeyCode::Enter => {
                let paginas = self.paginas_vigentes();
                if self.pagina + 1 < paginas {
                    self.pagina += 1;
                } else if !self.mostrar_detalle {
                    self.enviar(IntencionFrontend::ContinuarPresentacion {
                        fragmento: fragmento.id,
                    });
                }
            }
            VistaSesion::EsperandoDecision(decision) => {
                let Some(inicio) = inicio_opciones_decision(decision, self.pagina) else {
                    if tecla == KeyCode::Enter {
                        self.pagina += 1;
                    }
                    return;
                };
                if let Some(letra) = normalizar_letra(tecla)
                    && let Some((_, opcion)) = atajos_decision(decision)
                        .into_iter()
                        .skip(inicio)
                        .take(OPCIONES_POR_PAGINA)
                        .find(|(atajo, _)| *atajo == letra)
                {
                    self.enviar(IntencionFrontend::ResponderDecision {
                        decision: decision.id,
                        opcion,
                    });
                }
            }
            VistaSesion::Acampando(_)
            | VistaSesion::Fatal(_)
            | VistaSesion::Disponible(_)
            | VistaSesion::PresentandoAccion(_) => {}
        }
    }

    fn enviar(&mut self, intencion: IntencionFrontend) {
        match self.partida.procesar(intencion) {
            Ok(vista) => {
                self.vista = vista;
                self.error_recuperable = None;
                self.mostrar_detalle = false;
                self.pagina = 0;
                self.pagina_antes_detalle = 0;
                self.actualizar_mundo();
                self.campamento = match &self.vista {
                    VistaSesion::Acampando(v) => Some(crate::campamento::EstadoCampamento::nuevo(
                        v.clone(),
                        &self.partida,
                    )),
                    VistaSesion::Disponible(_)
                    | VistaSesion::PresentandoAccion(_)
                    | VistaSesion::EsperandoDecision(_)
                    | VistaSesion::Fatal(_) => None,
                };
            }
            Err(error) => self.error_recuperable = Some(texto_error_intencion(error)),
        }
    }

    fn actualizar_mundo(&mut self) {
        self.ultima_vista_mundo = self.partida.mundo_visible();
        if let VistaSesion::Disponible(mundo) = &self.vista {
            self.ultima_vista_mundo = mundo.clone();
            self.modo = ModoLocal::Normal;
        }
    }

    pub(crate) fn dibujar(&self, frame: &mut Frame<'_>, recursos: &RecursosVisuales) {
        self.dibujar_contenido(frame, recursos);
        recursos.paleta.aplicar(frame.buffer_mut());
    }

    fn dibujar_contenido(&self, frame: &mut Frame<'_>, recursos: &RecursosVisuales) {
        let area = frame.area();
        let fondo = recursos.paleta.fondo_pantalla(recursos.ui.pantalla.bg);
        frame.render_widget(Block::default().style(recursos.ui.pantalla.bg(fondo)), area);
        if area.width < ANCHO_MINIMO || area.height < ALTO_MINIMO {
            let texto = format!(
                "Terminal insuficiente: se requieren 80 x 24; tamaño actual {} x {}. [Q] Salir",
                area.width, area.height
            );
            frame.render_widget(Paragraph::new(texto).style(recursos.ui.error), area);
            if self.confirmar_salida {
                self.dibujar_confirmacion_compacta(frame, recursos);
            }
            return;
        }
        let mundo = &self.ultima_vista_mundo;
        let region_mundo = Rect::new(area.x, area.y, area.width, area.height - 2);
        if let Some(campamento) = &self.campamento {
            campamento.dibujar(frame, region_mundo, mundo, recursos, &self.reloj);
        } else if let ModoLocal::Seleccion { destinos, foco } = &self.modo {
            let ancho_nombre = destinos
                .iter()
                .filter_map(|id| mundo.lugares.iter().find(|l| l.id == *id))
                .map(|l| l.nombre_abreviado.chars().count())
                .max()
                .unwrap_or(0);
            let ancho_lista = (ancho_nombre + 4)
                .clamp(18, usize::from((region_mundo.width / 3).clamp(26, 40)))
                as u16;
            let mapa = Rect::new(
                region_mundo.x,
                region_mundo.y,
                region_mundo.width - ancho_lista,
                region_mundo.height,
            );
            let lista = Rect::new(
                mapa.right(),
                region_mundo.y,
                ancho_lista,
                region_mundo.height,
            );
            self.dibujar_mundo(frame, mapa, mundo, recursos);
            self.dibujar_destinos(frame, lista, destinos, *foco, recursos);
        } else {
            self.dibujar_mundo(frame, region_mundo, mundo, recursos);
        }
        let contextual = Rect::new(area.x, area.y + area.height - 2, area.width, 1);
        if self.aviso_audio_visible() {
            self.aviso_audio_mostrado.set(true);
        }
        let acciones = Rect::new(area.x, area.y + area.height - 1, area.width, 1);
        frame.render_widget(
            Paragraph::new(self.texto_contextual()).style(self.estilo_contextual(recursos)),
            contextual,
        );
        frame.render_widget(
            Paragraph::new(self.texto_acciones()).style(recursos.ui.barra_acciones),
            acciones,
        );
        match &self.vista {
            VistaSesion::PresentandoAccion(fragmento) => {
                let paginas = paginas_fragmento(fragmento, self.mostrar_detalle);
                let pagina = self.pagina.min(paginas.len().saturating_sub(1));
                let titulo = titulo_paginado(&fragmento.titulo, pagina, paginas.len());
                self.dibujar_ventana(
                    frame,
                    region_mundo,
                    &titulo,
                    &paginas[pagina],
                    fragmento.escena.filter(|_| !self.mostrar_detalle),
                    recursos,
                )
            }
            VistaSesion::EsperandoDecision(decision) => {
                let paginas = if self.mostrar_detalle {
                    paginas_detalle_decision(decision, self.pagina_antes_detalle)
                } else {
                    paginas_decision(decision)
                };
                let pagina = self.pagina.min(paginas.len().saturating_sub(1));
                let titulo = titulo_paginado(&decision.titulo, pagina, paginas.len());
                self.dibujar_ventana(
                    frame,
                    region_mundo,
                    &titulo,
                    &paginas[pagina],
                    decision.escena.filter(|_| !self.mostrar_detalle),
                    recursos,
                )
            }
            VistaSesion::Fatal(fatal) => {
                self.dibujar_ventana(frame, region_mundo, "Error", &fatal.mensaje, None, recursos)
            }
            VistaSesion::Disponible(_) | VistaSesion::Acampando(_) => {}
        }
        if self.confirmar_salida {
            self.dibujar_confirmacion(frame, region_mundo, recursos);
        }
    }

    fn dibujar_destinos(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        destinos: &[IdLugar],
        foco: usize,
        recursos: &RecursosVisuales,
    ) {
        let bloque = Block::default()
            .borders(Borders::ALL)
            .title(" Destinos ")
            .title_bottom(format!(" {}/{} ", foco + 1, destinos.len()))
            .style(recursos.ui.pantalla)
            .border_style(recursos.ui.marco)
            .title_style(recursos.ui.titulo);
        let interior = bloque.inner(area);
        frame.render_widget(bloque, area);
        let filas = usize::from(interior.height);
        let inicio = foco
            .saturating_sub(filas / 2)
            .min(destinos.len().saturating_sub(filas));
        let lineas = destinos
            .iter()
            .enumerate()
            .skip(inicio)
            .take(filas)
            .map(|(indice, id)| {
                let nombre = self
                    .ultima_vista_mundo
                    .lugares
                    .iter()
                    .find(|l| l.id == *id)
                    .map(|l| l.nombre_abreviado.as_str())
                    .unwrap_or("Lugar desconocido");
                let (prefijo, estilo) = if indice == foco {
                    ("> ", recursos.ui.foco)
                } else {
                    ("  ", recursos.ui.texto_normal)
                };
                Line::styled(format!("{prefijo}{nombre}"), estilo)
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lineas), interior);
    }

    fn dibujar_mundo(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        mundo: &VistaMundo,
        recursos: &RecursosVisuales,
    ) {
        let objetivo = self.objetivo_camara(mundo);
        let (origen_x, margen_x) =
            origen_eje(mundo.mapa.ancho, usize::from(area.width), objetivo.x);
        let (origen_y, margen_y) =
            origen_eje(mundo.mapa.alto, usize::from(area.height), objetivo.y);
        for vy in 0..usize::from(area.height) {
            for vx in 0..usize::from(area.width) {
                let Some(mx) = vx
                    .checked_sub(margen_x)
                    .and_then(|x| x.checked_add(origen_x))
                else {
                    continue;
                };
                let Some(my) = vy
                    .checked_sub(margen_y)
                    .and_then(|y| y.checked_add(origen_y))
                else {
                    continue;
                };
                if mx >= mundo.mapa.ancho || my >= mundo.mapa.alto {
                    continue;
                }
                let clave = &mundo.mapa.celdas[my * mundo.mapa.ancho + mx];
                let apariencia = recursos.tile(clave, mx, my, self.reloj.paso_250_ms());
                poner_celda(
                    frame,
                    area.x + vx as u16,
                    area.y + vy as u16,
                    apariencia.glifo,
                    apariencia.estilo,
                );
            }
        }
        let foco = match &self.modo {
            ModoLocal::Seleccion { destinos, foco } => destinos.get(*foco).copied(),
            ModoLocal::Normal => None,
        };
        for lugar in &mundo.lugares {
            for (caracter, coordenada) in lugar.nombre.chars().zip(&lugar.coordenadas_rotulo) {
                if let Some((x, y)) =
                    proyectar(*coordenada, area, origen_x, origen_y, margen_x, margen_y)
                {
                    let estilo = if foco == Some(lugar.id) {
                        recursos.ui.foco
                    } else {
                        recursos.ui.destacado
                    };
                    poner_celda(frame, x, y, caracter, estilo);
                }
            }
        }
        let lugar_grupo = match &self.vista {
            VistaSesion::PresentandoAccion(fragmento) => fragmento.progreso.lugar_confirmado,
            VistaSesion::EsperandoDecision(decision) => decision.progreso.lugar_confirmado,
            _ => mundo.lugar_jugador,
        };
        if let Some(coordenada) = mundo
            .lugares
            .iter()
            .find(|lugar| lugar.id == lugar_grupo)
            .map(|lugar| lugar.coordenada_central)
            && let Some((x, y)) =
                proyectar(coordenada, area, origen_x, origen_y, margen_x, margen_y)
        {
            poner_celda(frame, x, y, '@', recursos.ui.foco);
        }
        dibujar_retratos(
            frame,
            area,
            area,
            mundo,
            recursos,
            self.reloj.paso_250_ms(),
            false,
        );
    }

    fn objetivo_camara(&self, mundo: &VistaMundo) -> desert_tiger_app::CoordenadaMapa {
        let id = match &self.modo {
            ModoLocal::Seleccion { destinos, foco } => destinos[*foco],
            ModoLocal::Normal => match &self.vista {
                VistaSesion::PresentandoAccion(fragmento) => fragmento.progreso.lugar_confirmado,
                VistaSesion::EsperandoDecision(decision) => decision.progreso.lugar_confirmado,
                _ => mundo.lugar_jugador,
            },
        };
        mundo
            .lugares
            .iter()
            .find(|lugar| lugar.id == id)
            .map(|lugar| lugar.coordenada_central)
            .unwrap_or(desert_tiger_app::CoordenadaMapa { x: 0, y: 0 })
    }

    fn texto_contextual(&self) -> String {
        if self.aviso_audio_visible()
            && let Some(aviso) = self.aviso_audio
        {
            return aviso.mensaje().to_owned();
        }
        if self.confirmar_salida {
            return "La partida actual no se guardará. Confirma si quieres salir.".to_owned();
        }
        if let Some(error) = &self.error_recuperable {
            return format!("Error: {error}");
        }
        if let ModoLocal::Seleccion { destinos, foco } = &self.modo {
            let nombre = self
                .ultima_vista_mundo
                .lugares
                .iter()
                .find(|lugar| lugar.id == destinos[*foco])
                .map(|lugar| lugar.nombre.as_str())
                .unwrap_or("destino");
            return format!("> Destino: {nombre}. Elige adónde viajar.");
        }
        match &self.vista {
            VistaSesion::PresentandoAccion(fragmento) => texto_progreso(&fragmento.progreso),
            VistaSesion::EsperandoDecision(decision) => texto_progreso(&decision.progreso),
            VistaSesion::Acampando(_) => self
                .campamento
                .as_ref()
                .map(|c| c.contexto())
                .unwrap_or_default(),
            VistaSesion::Fatal(_) => "La sesión encontró un error fatal.".to_owned(),
            VistaSesion::Disponible(mundo) => contexto_mundo(mundo),
        }
    }

    fn estilo_contextual(&self, recursos: &RecursosVisuales) -> Style {
        if self.aviso_audio_visible() {
            return recursos.ui.secundario;
        }
        if self.error_recuperable.is_some() || matches!(self.vista, VistaSesion::Fatal(_)) {
            recursos.ui.error
        } else {
            recursos.ui.barra_contextual
        }
    }

    fn texto_acciones(&self) -> String {
        if self.campamento.is_some() && !self.confirmar_salida {
            return " ←→ Grupo | ↑↓ Info | Enter Abrir/Salir | PgUp/PgDn Leer | Esc Salir | Q Juego".into();
        }
        if self.confirmar_salida {
            return " [S] Sí  [N] No  [Esc] Cancelar".to_owned();
        }
        match &self.modo {
            ModoLocal::Seleccion { .. } => {
                " [↑/↓] Destino  [Enter] Confirmar  [Esc] Cancelar  [Q] Salir".to_owned()
            }
            ModoLocal::Normal => match &self.vista {
                VistaSesion::Disponible(mundo) => match destinos_viaje(mundo) {
                    Some([]) => " Viajar (no disponible)  [A] Acampar  [Q] Salir".to_owned(),
                    Some(_) => " [V] Viajar  [A] Acampar  [Q] Salir".to_owned(),
                    None => " [Q] Salir".to_owned(),
                },
                VistaSesion::PresentandoAccion(fragmento) => {
                    let detalle = if fragmento.detalle.is_empty() {
                        ""
                    } else if self.mostrar_detalle {
                        "  [D] Ocultar detalles"
                    } else {
                        "  [D] Detalles"
                    };
                    let paginas = self.paginas_vigentes();
                    let navegar = if paginas > 1 {
                        "  [←/→] Página"
                    } else {
                        ""
                    };
                    let accion = if self.mostrar_detalle {
                        "[Enter] Página siguiente"
                    } else {
                        "[Enter] Continuar"
                    };
                    format!(" {accion}{navegar}{detalle}  [Q] Salir")
                }
                VistaSesion::EsperandoDecision(decision) => {
                    let inicio = inicio_opciones_decision(decision, self.pagina);
                    let opciones = if self.mostrar_detalle {
                        "[Enter] Página siguiente".to_owned()
                    } else if let Some(inicio) = inicio {
                        atajos_decision(decision)
                            .into_iter()
                            .skip(inicio)
                            .take(OPCIONES_POR_PAGINA)
                            .zip(decision.opciones.iter().skip(inicio))
                            .map(|((tecla, _), opcion)| {
                                format!("[{}] {}", tecla.to_ascii_uppercase(), opcion.texto)
                            })
                            .collect::<Vec<_>>()
                            .join("  ")
                    } else {
                        "[Enter] Continuar".to_owned()
                    };
                    let detalle = if self.vista_tiene_detalle() {
                        if self.mostrar_detalle {
                            "  [D] Ocultar detalles"
                        } else {
                            "  [D] Detalles"
                        }
                    } else {
                        ""
                    };
                    let navegar = if self.paginas_vigentes() > 1 {
                        "  [←/→] Página"
                    } else {
                        ""
                    };
                    format!(" {opciones}{navegar}{detalle}  [Q] Salir")
                }
                VistaSesion::Acampando(_) | VistaSesion::Fatal(_) => String::new(),
            },
        }
    }

    fn vista_tiene_detalle(&self) -> bool {
        match &self.vista {
            VistaSesion::PresentandoAccion(fragmento) => !fragmento.detalle.is_empty(),
            VistaSesion::EsperandoDecision(decision) => {
                !decision.detalle.is_empty()
                    || decision
                        .opciones
                        .iter()
                        .any(|opcion| opcion.detalle.is_some())
            }
            _ => false,
        }
    }

    fn paginas_vigentes(&self) -> usize {
        match &self.vista {
            VistaSesion::PresentandoAccion(fragmento) => {
                paginas_fragmento(fragmento, self.mostrar_detalle).len()
            }
            VistaSesion::EsperandoDecision(decision) => {
                if self.mostrar_detalle {
                    paginas_detalle_decision(decision, self.pagina_antes_detalle).len()
                } else {
                    paginas_decision(decision).len()
                }
            }
            _ => 1,
        }
    }

    fn dibujar_ventana(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        titulo: &str,
        texto: &str,
        escena: Option<desert_tiger_app::ClaveEscena>,
        recursos: &RecursosVisuales,
    ) {
        let ancho = 72.min(area.width.saturating_sub(4)).max(40);
        // Reservamos espacio real para las ocho líneas paginadas y el arte:
        // una ventana fija de doce filas ocultaba las últimas líneas del relato.
        let arte = escena.and_then(|clave| recursos.escena(clave));
        let filas_arte = arte.map_or(0, |filas| filas.len() + 1);
        let alto = (filas_arte + texto.lines().count() + 2)
            .max(12)
            .min(usize::from(area.height.saturating_sub(2)))
            .max(7) as u16;
        let ventana = Rect::new(
            area.x + (area.width - ancho) / 2,
            area.y + (area.height - alto) / 2,
            ancho,
            alto,
        );
        frame.render_widget(Clear, ventana);
        let bloque = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(titulo, recursos.ui.titulo))
            .style(recursos.ui.marco);
        let interior = bloque.inner(ventana);
        frame.render_widget(bloque, ventana);
        let mut lineas = Vec::new();
        if let Some(arte) = arte {
            lineas.extend(
                arte.iter()
                    .map(|fila| Line::styled(fila.clone(), recursos.ui.destacado)),
            );
            lineas.push(Line::default());
        }
        lineas.extend(
            texto
                .lines()
                .map(|linea| Line::styled(linea.to_owned(), recursos.ui.texto_normal)),
        );
        frame.render_widget(
            Paragraph::new(lineas).wrap(ratatui::widgets::Wrap { trim: false }),
            interior,
        );
    }

    fn dibujar_confirmacion(&self, frame: &mut Frame<'_>, area: Rect, recursos: &RecursosVisuales) {
        self.dibujar_ventana(
            frame,
            area,
            "Salir",
            "La partida actual no se guardará. ¿Quieres salir del juego?",
            None,
            recursos,
        );
    }

    fn dibujar_confirmacion_compacta(&self, frame: &mut Frame<'_>, recursos: &RecursosVisuales) {
        frame.render_widget(
            Paragraph::new("Salir: [S] Sí  [N] No  [Esc] Cancelar").style(recursos.ui.error),
            frame.area(),
        );
    }
}

// Las coordenadas solo ordenan el foco visual; los destinos permitidos siguen
// siendo exactamente los que entregó la aplicación.
fn foco_mas_cercano(
    destinos: &[IdLugar],
    lugares: &[desert_tiger_app::VistaLugar],
    origen: desert_tiger_app::CoordenadaMapa,
) -> Option<usize> {
    destinos
        .iter()
        .enumerate()
        .filter_map(|(indice, id)| {
            let punto = lugares
                .iter()
                .find(|lugar| lugar.id == *id)?
                .coordenada_central;
            let dx = punto.x.abs_diff(origen.x) as f64;
            let dy = punto.y.abs_diff(origen.y) as f64;
            let distancia = dx * dx + dy * dy;
            Some((indice, *id, distancia))
        })
        .min_by(|a, b| a.2.total_cmp(&b.2).then_with(|| a.1.cmp(&b.1)))
        .map(|(indice, _, _)| indice)
}

fn destinos_viaje(mundo: &VistaMundo) -> Option<&[IdLugar]> {
    mundo.acciones.iter().find_map(|a| match a {
        AccionDisponible::Viajar { destinos } => Some(destinos.as_slice()),
        AccionDisponible::Acampar => None,
    })
}

fn atajos_decision(decision: &VistaDecision) -> Vec<(char, desert_tiger_app::IdOpcionVista)> {
    let mut usados = BTreeSet::from(['d', 'q']);
    decision
        .opciones
        .iter()
        .enumerate()
        .map(|(indice, opcion)| {
            if indice > 0 && indice % 24 == 0 {
                usados = BTreeSet::from(['d', 'q']);
            }
            let preferida = opcion
                .texto
                .chars()
                .find(|c| c.is_ascii_alphabetic() && !usados.contains(&c.to_ascii_lowercase()))
                .map(|c| c.to_ascii_lowercase());
            let tecla = preferida
                .or_else(|| ('a'..='z').find(|c| !usados.contains(c)))
                .unwrap_or('?');
            usados.insert(tecla);
            (tecla, opcion.id)
        })
        .collect()
}

fn paginas_fragmento(fragmento: &VistaFragmento, detalle: bool) -> Vec<String> {
    if detalle {
        paginar_texto(&format!("DETALLES\n{}", fragmento.detalle.join("\n\n")))
    } else {
        paginar_texto(&fragmento.texto)
    }
}

fn inicio_opciones_decision(decision: &VistaDecision, pagina: usize) -> Option<usize> {
    let paginas_relato = paginar_texto(&decision.texto).len();
    let inicio = pagina.checked_sub(paginas_relato.saturating_sub(1))? * OPCIONES_POR_PAGINA;
    (inicio < decision.opciones.len()).then_some(inicio)
}

fn paginas_decision(decision: &VistaDecision) -> Vec<String> {
    let mut paginas = paginar_texto(&decision.texto);
    if let Some(ultima) = paginas.last().cloned() {
        // La primera opción aparece al terminar de leer; las restantes conservan
        // ese contexto, cada una con su página de la barra de acciones.
        paginas.extend(std::iter::repeat_n(
            ultima,
            decision
                .opciones
                .len()
                .div_ceil(OPCIONES_POR_PAGINA)
                .saturating_sub(1),
        ));
    }
    paginas
}

fn paginas_detalle_decision(decision: &VistaDecision, pagina_origen: usize) -> Vec<String> {
    let mut texto = format!("DETALLES\n{}", decision.detalle.join("\n\n"));
    if let Some(inicio) = inicio_opciones_decision(decision, pagina_origen) {
        let atajos = atajos_decision(decision);
        for (i, opcion) in decision
            .opciones
            .iter()
            .enumerate()
            .skip(inicio)
            .take(OPCIONES_POR_PAGINA)
        {
            if let Some(detalle) = &opcion.detalle {
                texto.push_str(&format!(
                    "\n\n[{}] {}\n{detalle}",
                    atajos[i].0.to_ascii_uppercase(),
                    opcion.texto
                ));
            }
        }
    }
    paginar_texto(&texto)
}

fn titulo_paginado(titulo: &str, pagina: usize, total: usize) -> String {
    if total > 1 {
        format!("{titulo} · {}/{}", pagina + 1, total)
    } else {
        titulo.to_owned()
    }
}

fn paginar_texto(texto: &str) -> Vec<String> {
    let mut lineas = Vec::new();
    for parrafo in texto.split('\n') {
        if parrafo.is_empty() {
            lineas.push(String::new());
            continue;
        }
        let mut linea = String::new();
        for palabra in parrafo.split_whitespace() {
            let separador = usize::from(!linea.is_empty());
            let ancho_linea = linea
                .chars()
                .map(|caracter| caracter.width().unwrap_or(0))
                .sum::<usize>();
            let ancho_palabra = palabra
                .chars()
                .map(|caracter| caracter.width().unwrap_or(0))
                .sum::<usize>();
            if ancho_linea + separador + ancho_palabra > ANCHO_TEXTO_VINETA && !linea.is_empty() {
                lineas.push(std::mem::take(&mut linea));
            }
            if ancho_palabra > ANCHO_TEXTO_VINETA {
                let mut fragmento = String::new();
                let mut ancho_fragmento = 0;
                for caracter in palabra.chars() {
                    let ancho = caracter.width().unwrap_or(0);
                    if ancho_fragmento + ancho > ANCHO_TEXTO_VINETA && !fragmento.is_empty() {
                        lineas.push(std::mem::take(&mut fragmento));
                        ancho_fragmento = 0;
                    }
                    fragmento.push(caracter);
                    ancho_fragmento += ancho;
                }
                linea = fragmento;
                continue;
            }
            if !linea.is_empty() {
                linea.push(' ');
            }
            linea.push_str(palabra);
        }
        lineas.push(linea);
    }
    let mut paginas = lineas
        .chunks(LINEAS_POR_PAGINA)
        .map(|lineas| lineas.join("\n"))
        .collect::<Vec<_>>();
    if paginas.is_empty() {
        paginas.push(String::new());
    }
    paginas
}

fn contexto_mundo(mundo: &VistaMundo) -> String {
    if mundo.causa_indisponibilidad == Some(CausaIndisponibilidad::JugadorMuerto) {
        return "El personaje jugador está muerto y no puede actuar.".to_owned();
    }
    if let Some(resumen) = &mundo.resumen_npc_reciente
        && let Some(entrada) = resumen.entradas().first()
    {
        let actividad = resumen
            .cronologias()
            .first()
            .map(|cronologia| {
                format!(
                    " {} escenas; {} simulados.",
                    cronologia.escenas, cronologia.tiempo_visible
                )
            })
            .unwrap_or_default();
        return match entrada {
            EntradaResumenNpc::ViajeCompletado { .. } => {
                format!("Un actor completó un viaje mientras transcurría el tiempo.{actividad}")
            }
            EntradaResumenNpc::ViajeInterrumpido { .. } => {
                format!("Un actor intentó viajar, pero interrumpió el recorrido.{actividad}")
            }
            EntradaResumenNpc::SolicitudViajeRechazada { .. } => {
                "Error: una acción de un NPC fue rechazada.".to_owned()
            }
            EntradaResumenNpc::ErrorPolitica { .. } => {
                "Error: la política de un NPC no pudo evaluarse.".to_owned()
            }
        };
    }
    if destinos_viaje(mundo).is_some_and(<[IdLugar]>::is_empty) {
        "No hay destinos disponibles para viajar.".to_owned()
    } else {
        "Elige una acción en el mundo.".to_owned()
    }
}

fn texto_progreso(progreso: &desert_tiger_app::VistaProgresoViaje) -> String {
    let siguiente = progreso
        .nombre_siguiente_lugar
        .as_deref()
        .map(|nombre| format!(" · siguiente: {nombre}"))
        .unwrap_or_default();
    format!(
        "{} · {:02}/{:02}/{} · {}{siguiente}",
        progreso.nombre_lugar_confirmado,
        progreso.fecha_proyectada.dia(),
        progreso.fecha_proyectada.mes().numero(),
        progreso.fecha_proyectada.anio(),
        progreso.tiempo_transcurrido,
    )
}

fn texto_error_intencion(error: ErrorIntencion) -> String {
    match error {
        ErrorIntencion::CampamentoNoVigente => "el campamento ya no está vigente",
        ErrorIntencion::AcampadaRechazada(_) => "no se puede abrir el campamento",
        ErrorIntencion::FaseIncompatible => "esa orden no está disponible ahora",
        ErrorIntencion::FragmentoNoVigente => "esa viñeta ya no está vigente",
        ErrorIntencion::DecisionNoVigente => "esa decisión ya no está vigente",
        ErrorIntencion::OpcionNoVigente => "esa opción ya no está vigente",
        ErrorIntencion::AccionRechazada(_) => "la acción fue rechazada por el mundo",
    }
    .to_owned()
}

fn normalizar_letra(tecla: KeyCode) -> Option<char> {
    match tecla {
        KeyCode::Char(c) if c.is_ascii_alphabetic() => Some(c.to_ascii_lowercase()),
        _ => None,
    }
}

fn origen_eje(mapa: usize, vista: usize, objetivo: usize) -> (usize, usize) {
    if mapa <= vista {
        (0, (vista - mapa) / 2)
    } else {
        (objetivo.saturating_sub(vista / 2).min(mapa - vista), 0)
    }
}

fn proyectar(
    coordenada: desert_tiger_app::CoordenadaMapa,
    area: Rect,
    origen_x: usize,
    origen_y: usize,
    margen_x: usize,
    margen_y: usize,
) -> Option<(u16, u16)> {
    let vx = coordenada.x.checked_sub(origen_x)?.checked_add(margen_x)?;
    let vy = coordenada.y.checked_sub(origen_y)?.checked_add(margen_y)?;
    if vx >= usize::from(area.width) || vy >= usize::from(area.height) {
        return None;
    }
    Some((
        area.x + u16::try_from(vx).ok()?,
        area.y + u16::try_from(vy).ok()?,
    ))
}

pub(crate) fn disposicion_retratos(
    mundo: &VistaMundo,
    area: Rect,
) -> impl Iterator<Item = (&VistaActor, i64, i64)> {
    mundo
        .companeros
        .iter()
        .chain(std::iter::once(&mundo.jugador))
        .scan(
            (i64::from(area.x), None::<usize>),
            move |(x, anterior), actor| {
                let base = actor.retrato.base();
                if let Some(ancho) = *anterior {
                    let solapamiento = 1.max(ancho.min(base.ancho()) / 4);
                    *x = x.saturating_add(i64::try_from(ancho - solapamiento).unwrap_or(i64::MAX));
                }
                *anterior = Some(base.ancho());
                let y = i64::from(area.bottom())
                    .saturating_sub(i64::try_from(base.alto()).unwrap_or(i64::MAX));
                Some((actor, *x, y))
            },
        )
}

pub(crate) fn dibujar_retratos(
    frame: &mut Frame<'_>,
    area: Rect,
    recorte: Rect,
    mundo: &VistaMundo,
    recursos: &RecursosVisuales,
    paso: usize,
    musica: bool,
) -> i64 {
    let mut fin = i64::from(area.x);
    for (actor, x, inicio_y) in disposicion_retratos(mundo, area) {
        let retrato = &actor.retrato;
        for (origen, grafico) in retrato
            .capas(expresion_retrato(actor, musica), parpadeando(actor, paso))
            .into_iter()
            .flatten()
        {
            let px = x.saturating_add(i64::try_from(origen.x).unwrap_or(i64::MAX));
            let py = inicio_y.saturating_add(i64::try_from(origen.y).unwrap_or(i64::MAX));
            dibujar_grafico_retrato(frame, recorte, px, py, grafico, recursos);
        }
        fin = x.saturating_add(i64::try_from(retrato.base().ancho()).unwrap_or(i64::MAX));
    }
    fin
}

fn expresion_retrato(actor: &VistaActor, musica: bool) -> Option<&str> {
    if musica
        && actor
            .retrato
            .rostro()
            .is_some_and(|r| r.expresion("music").is_some())
    {
        Some("music")
    } else {
        actor.expresion.as_deref()
    }
}

fn parpadeando(actor: &VistaActor, paso: usize) -> bool {
    let Some(parpadeo) = actor.retrato.parpadeo() else {
        return false;
    };
    let intervalo = u128::from(parpadeo.intervalo_ms() / 250);
    let duracion = u128::from(parpadeo.duracion_ms() / 250);
    // Es un desfase visual estable por ID. Nunca consume azar de la Partida.
    let mut hash = DefaultHasher::new();
    actor.id.hash(&mut hash);
    let desfase = u128::from(hash.finish()) % intervalo;
    (paso as u128 + desfase) % intervalo >= intervalo - duracion
}

fn dibujar_grafico_retrato(
    frame: &mut Frame<'_>,
    area: Rect,
    x: i64,
    y: i64,
    grafico: &GraficoRetrato,
    recursos: &RecursosVisuales,
) {
    for (indice, celda) in grafico.celdas().iter().enumerate() {
        if celda.glifo == ' ' {
            continue;
        }
        let px = x.saturating_add(i64::try_from(indice % grafico.ancho()).unwrap_or(i64::MAX));
        let py = y.saturating_add(i64::try_from(indice / grafico.ancho()).unwrap_or(i64::MAX));
        if px >= i64::from(area.x)
            && px < i64::from(area.right())
            && py >= i64::from(area.y)
            && py < i64::from(area.bottom())
        {
            let estilo = celda
                .estilo
                .map(estilo_retrato)
                .unwrap_or(recursos.ui.destacado);
            // El fondo puede ser transparente; la negrita/tenue del terreno no.
            let estilo = estilo.remove_modifier(Modifier::all().difference(estilo.add_modifier));
            if let (Ok(px), Ok(py)) = (u16::try_from(px), u16::try_from(py)) {
                poner_celda(frame, px, py, celda.glifo, estilo);
            }
        }
    }
}

fn poner_celda(frame: &mut Frame<'_>, x: u16, y: u16, caracter: char, estilo: Style) {
    if let Some(celda) = frame.buffer_mut().cell_mut((x, y)) {
        celda.set_char(caracter).set_style(estilo);
    }
}

fn estilo_retrato(estilo: EstiloCeldaRetrato) -> Style {
    let mut resultado = Style::default().fg(color_retrato(estilo.frente));
    if let Some(fondo) = estilo.fondo {
        resultado = resultado.bg(if fondo == ColorRetrato::Negro {
            NEGRO_PURO
        } else {
            color_retrato(fondo)
        });
    }
    resultado
}

fn color_retrato(color: ColorRetrato) -> Color {
    match color {
        ColorRetrato::Negro => Color::Black,
        ColorRetrato::Rojo => Color::Red,
        ColorRetrato::Verde => Color::Green,
        ColorRetrato::Amarillo => Color::Yellow,
        ColorRetrato::Azul => Color::Blue,
        ColorRetrato::Magenta => Color::Magenta,
        ColorRetrato::Cian => Color::Cyan,
        ColorRetrato::Gris => Color::Gray,
        ColorRetrato::GrisOscuro => Color::DarkGray,
        ColorRetrato::GrisMuyOscuro => Color::Rgb(32, 32, 32),
        ColorRetrato::RojoClaro => Color::LightRed,
        ColorRetrato::VerdeClaro => Color::LightGreen,
        ColorRetrato::AmarilloClaro => Color::LightYellow,
        ColorRetrato::AzulClaro => Color::LightBlue,
        ColorRetrato::MagentaClaro => Color::LightMagenta,
        ColorRetrato::CianClaro => Color::LightCyan,
        ColorRetrato::Blanco => Color::White,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crossterm::event::KeyModifiers;
    use desert_tiger_app::{ConfiguracionPartida, cargar_contenido};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use std::path::PathBuf;

    #[test]
    fn retratos_conservan_color_y_modificadores_sobre_cada_terreno_al_redimensionar() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let interfaz = Interfaz::nueva(partida).unwrap();
        let mut recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        for fija in [false, true] {
            recursos.paleta = if fija {
                crate::colores::PaletaColores::fija()
            } else {
                crate::colores::PaletaColores::default()
            };
            for ancho in [80, 97, 120] {
                let area = Rect::new(0, 0, ancho, 22);
                let mut mundo = interfaz.ultima_vista_mundo.clone();
                for lugar in &mut mundo.lugares {
                    lugar.coordenadas_rotulo.clear();
                }
                let mut terminal = Terminal::new(TestBackend::new(ancho, 24)).unwrap();
                terminal
                    .draw(|f| {
                        dibujar_retratos(f, area, area, &mundo, &recursos, 0, false);
                        recursos.paleta.aplicar(f.buffer_mut());
                    })
                    .unwrap();
                let retratos = terminal.backend().buffer().clone();
                for tipo in ["agua", "montana", "bosque", "arido", "bosqueQuemado"] {
                    let clave = desert_tiger_app::ClaveTipoTile::nueva(tipo.into()).unwrap();
                    mundo.mapa.celdas.fill(clave);
                    terminal
                        .draw(|f| {
                            interfaz.dibujar_mundo(f, area, &mundo, &recursos);
                            recursos.paleta.aplicar(f.buffer_mut());
                        })
                        .unwrap();
                    let mapa = terminal.backend().buffer();
                    for y in 0..22 {
                        for x in 0..ancho {
                            let esperado = &retratos[(x, y)];
                            if esperado.symbol() == " " {
                                continue;
                            }
                            let actual = &mapa[(x, y)];
                            assert_eq!(
                                actual.symbol(),
                                esperado.symbol(),
                                "glifo en {tipo}, ancho {ancho}, ({x},{y})"
                            );
                            assert_eq!(
                                actual.fg, esperado.fg,
                                "color en {tipo}, ancho {ancho}, ({x},{y})"
                            );
                            assert_eq!(
                                actual.modifier, esperado.modifier,
                                "atributos en {tipo}, ancho {ancho}, ({x},{y})"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn retrato_no_hereda_atributos_pero_conserva_fondos_espacios_y_estilo_propio() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let p = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let mundo = p.mundo_visible();
        let base = mundo
            .companeros
            .iter()
            .find(|a| a.nombre == "Sirviente")
            .unwrap()
            .retrato
            .base();
        let sin_estilo = partida_con_retrato_capas().mundo_visible();
        let sin_estilo = sin_estilo
            .jugador
            .retrato
            .rostro()
            .unwrap()
            .expresion("sinEstilo")
            .unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let heredados = Modifier::BOLD | Modifier::DIM | Modifier::REVERSED;
        for grafico in [base, sin_estilo] {
            let render = |modificadores| {
                let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
                t.draw(|f| {
                    for c in &mut f.buffer_mut().content {
                        c.set_char('?').set_style(
                            Style::default()
                                .fg(Color::Green)
                                .bg(Color::Blue)
                                .add_modifier(modificadores),
                        );
                    }
                    dibujar_grafico_retrato(f, Rect::new(0, 0, 80, 24), 0, 0, grafico, &recursos);
                })
                .unwrap();
                t.backend().buffer().clone()
            };
            let plano = render(Modifier::empty());
            let decorado = render(heredados);
            for (i, celda) in grafico.celdas().iter().enumerate() {
                let posicion = ((i % grafico.ancho()) as u16, (i / grafico.ancho()) as u16);
                if celda.glifo == ' ' {
                    assert_eq!(decorado[posicion].symbol(), "?");
                    assert_eq!(decorado[posicion].modifier, heredados);
                    assert_eq!(decorado[posicion].bg, Color::Blue);
                } else {
                    assert_eq!(decorado[posicion], plano[posicion]);
                    match celda.estilo {
                        None => {
                            assert_eq!(
                                decorado[posicion].modifier,
                                recursos.ui.destacado.add_modifier
                            );
                            assert_eq!(decorado[posicion].bg, Color::Blue);
                        }
                        Some(estilo) if estilo.fondo.is_none() => {
                            assert_eq!(decorado[posicion].bg, Color::Blue)
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn notas_musicales_siguen_al_sirviente_solo_con_audio_confirmado_y_respetan_recorte() {
        use crate::musica_campamento::{AgendaMusica, ProgresoMusica};
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut i =
            Interfaz::nueva(Partida::nueva(&c, ConfiguracionPartida { semilla: Some(7) }).unwrap())
                .unwrap();
        let mut r = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        r.efecto_fogata = None;
        i.enviar(IntencionFrontend::ComenzarCampamento);
        // Esta prueba aísla las notas: la misma cara musical en ambos cuadros.
        for actor in i
            .ultima_vista_mundo
            .companeros
            .iter_mut()
            .chain(std::iter::once(&mut i.ultima_vista_mundo.jugador))
        {
            actor.expresion = Some("music".into());
        }
        let ajustes = Some(AgendaMusica {
            inicial: (1, 1),
            pausa: (2, 2),
        });
        i.actualizar_musica(Duration::ZERO, None, true, ajustes);
        i.actualizar_musica(Duration::from_secs(1), None, true, ajustes);
        let evento = i.solicitud_musica();
        assert!(evento > 0 && i.musica.notas().is_none());
        let base = cuadro_campamento(&i, &r, 80, 24);
        let mut vistos = 0;
        let mut cuadros = Vec::new();
        for ms in [0, 450, 900, 1400] {
            i.actualizar_musica(
                Duration::from_secs(2),
                Some(ProgresoMusica {
                    evento,
                    transcurrido: Duration::from_millis(ms),
                    finalizada: false,
                }),
                true,
                ajustes,
            );
            let notas = i.musica.notas().unwrap();
            assert_eq!(
                i.ultima_vista_mundo
                    .companeros
                    .iter()
                    .find(|a| a.id == notas.actor)
                    .unwrap()
                    .nombre,
                "Sirviente"
            );
            let cuadro = cuadro_campamento(&i, &r, 80, 24);
            for y in 0..24 {
                for x in 0..80 {
                    if cuadro[(x, y)] != base[(x, y)] {
                        assert_eq!(base[(x, y)].symbol(), " ");
                        assert_eq!(cuadro[(x, y)].bg, base[(x, y)].bg);
                        assert!(["♪", "♫", "♩"].contains(&cuadro[(x, y)].symbol()));
                        assert!((6..22).contains(&y));
                        vistos += 1;
                    }
                }
            }
            cuadros.push(cuadro);
            let _ = cuadro_campamento(&i, &r, 79, 23);
        }
        assert!(
            vistos > 0,
            "debe verse alguna nota en los retratos oficiales"
        );
        assert_ne!(cuadros[0], cuadros[2]);
        for glifo in ['♪', '♫', '♩'] {
            assert_eq!(UnicodeWidthChar::width(glifo), Some(1));
        }
        i.actualizar_musica(Duration::from_secs(3), None, false, ajustes);
        assert!(i.musica.notas().is_none() && i.solicitud_musica() == 0);
        assert_eq!(base, cuadro_campamento(&i, &r, 80, 24));
    }

    #[test]
    fn rostro_music_es_para_todo_el_grupo_y_restaura_las_expresiones_previas() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        let p = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let antes = p.vista();
        let mut mundo = p.mundo_visible();
        for actor in mundo
            .companeros
            .iter_mut()
            .chain(std::iter::once(&mut mundo.jugador))
        {
            actor.expresion = Some("angry".into());
        }
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let paso = (0..40)
            .find(|&p| {
                mundo
                    .companeros
                    .iter()
                    .chain(std::iter::once(&mundo.jugador))
                    .all(|a| !parpadeando(a, p))
            })
            .unwrap();
        let render = |mundo: &VistaMundo, musica| {
            let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
            t.draw(|f| {
                let area = f.area();
                dibujar_retratos(f, area, area, mundo, &recursos, paso, musica);
            })
            .unwrap();
            t.backend().buffer().clone()
        };
        let base = render(&mundo, false);
        let disfrutando = render(&mundo, true);
        let rectangulos: Vec<_> = disposicion_retratos(&mundo, Rect::new(0, 0, 80, 24))
            .map(|(actor, x, y)| {
                let rostro = actor.retrato.rostro().unwrap();
                let grafico = rostro
                    .expresion("music")
                    .expect("el grupo inicial tiene cara musical");
                (
                    x + rostro.origen().x as i64,
                    y + rostro.origen().y as i64,
                    grafico.ancho() as i64,
                    grafico.alto() as i64,
                )
            })
            .collect();
        for &(x, y, ancho, alto) in &rectangulos {
            assert!((y..y + alto).any(|py| {
                (x..x + ancho)
                    .any(|px| base[(px as u16, py as u16)] != disfrutando[(px as u16, py as u16)])
            }));
        }
        for py in 0..24 {
            for px in 0..80 {
                if base[(px, py)] != disfrutando[(px, py)] {
                    assert!(
                        rectangulos
                            .iter()
                            .any(|&(x, y, w, h)| (x..x + w).contains(&i64::from(px))
                                && (y..y + h).contains(&i64::from(py)))
                    );
                }
            }
        }
        assert_eq!(render(&mundo, false), base);
        assert!(
            mundo
                .companeros
                .iter()
                .chain(std::iter::once(&mundo.jugador))
                .all(|a| a.expresion.as_deref() == Some("angry"))
        );
        let mut sin_variante = partida_con_retrato_capas().mundo_visible();
        sin_variante.companeros.clear();
        sin_variante.jugador.expresion = Some("enojado".into());
        assert_eq!(render(&sin_variante, true), render(&sin_variante, false));
        assert_eq!(p.vista(), antes);
    }

    #[test]
    fn paisaje_del_viaje_sigue_lugar_confirmado_y_no_foco_o_siguiente_lugar() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        let inicial = i.ambiente_sonoro();
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
        i.manejar_tecla(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(i.ambiente_sonoro(), inicial);
        i.manejar_tecla(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        let bosque = i
            .ultima_vista_mundo
            .lugares
            .iter()
            .find(|l| l.nombre == "Bosque de Tamrut")
            .unwrap()
            .id;
        let mar = i
            .ultima_vista_mundo
            .lugares
            .iter()
            .find(|l| l.nombre == "Mar de Poniente")
            .unwrap()
            .id;
        i.enviar(IntencionFrontend::ComenzarViaje { destino: mar });
        let mut hubo_fragmento = false;
        let mut hubo_decision = false;
        for _ in 0..500 {
            let vista_real = i.vista.clone();
            match &mut i.vista {
                VistaSesion::PresentandoAccion(f) => {
                    f.progreso.lugar_confirmado = bosque;
                    f.progreso.siguiente_lugar = Some(mar);
                    hubo_fragmento = true;
                }
                VistaSesion::EsperandoDecision(d) => {
                    d.progreso.lugar_confirmado = bosque;
                    d.progreso.siguiente_lugar = Some(mar);
                    hubo_decision = true;
                }
                _ => break,
            }
            let paisaje = i.ambiente_sonoro();
            assert!(paisaje.exterior && paisaje.bosque && paisaje.rio && !paisaje.mar);
            i.vista = vista_real;
            if hubo_fragmento && hubo_decision {
                break;
            }
            match i.vista.clone() {
                VistaSesion::PresentandoAccion(f) => {
                    i.enviar(IntencionFrontend::ContinuarPresentacion { fragmento: f.id })
                }
                VistaSesion::EsperandoDecision(d) => {
                    i.enviar(IntencionFrontend::ResponderDecision {
                        decision: d.id,
                        opcion: d.opciones[0].id,
                    })
                }
                _ => break,
            }
        }
        assert!(hubo_fragmento && hubo_decision);
    }

    #[test]
    fn aviso_audio_respeta_prioridades_y_solo_se_retira_despues_de_dibujarse() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        let tecla = |c| KeyEvent::new(c, KeyModifiers::NONE);
        i.manejar_tecla(tecla(KeyCode::Char('q')));
        i.recibir_aviso_audio(AvisoAudio::Configuracion);
        assert!(!i.aviso_audio_visible());
        i.manejar_tecla(tecla(KeyCode::Char('n')));
        assert!(i.aviso_audio.is_some());
        let mut pequena = Terminal::new(TestBackend::new(79, 23)).unwrap();
        pequena.draw(|f| i.dibujar(f, &recursos)).unwrap();
        i.manejar_tecla(tecla(KeyCode::Char('x')));
        assert!(i.aviso_audio.is_some());
        i.error_recuperable = Some("problema de juego".into());
        assert!(!i.aviso_audio_visible());
        assert!(i.texto_contextual().contains("problema de juego"));
        i.error_recuperable = None;
        i.manejar_tecla(tecla(KeyCode::Char('v')));
        assert!(!i.aviso_audio_visible());
        i.manejar_tecla(tecla(KeyCode::Esc));
        i.manejar_tecla(tecla(KeyCode::Char('a')));
        assert!(i.aviso_audio_visible());
        let mut normal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        normal.draw(|f| i.dibujar(f, &recursos)).unwrap();
        assert!(i.texto_contextual().contains("Audio no disponible"));
        assert_eq!(i.estilo_contextual(&recursos), recursos.ui.secundario);
        i.manejar_tecla(tecla(KeyCode::Right));
        assert!(i.aviso_audio.is_none());
        assert!(i.ambiente_sonoro().fogata);
    }

    #[test]
    fn ambiente_se_conserva_en_fichas_y_cierre_rechazado_pero_no_en_salida_o_fatal() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        let tecla = |c| KeyEvent::new(c, KeyModifiers::NONE);
        i.manejar_tecla(tecla(KeyCode::Char('a')));
        for codigo in [
            KeyCode::Enter,
            KeyCode::PageDown,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Down,
        ] {
            i.manejar_tecla(tecla(codigo));
            assert!(i.ambiente_sonoro().fogata);
        }
        let id = match &i.vista {
            VistaSesion::Acampando(v) => v.id,
            _ => panic!("campamento esperado"),
        };
        i.enviar(IntencionFrontend::AbandonarCampamento { campamento: id });
        i.enviar(IntencionFrontend::ComenzarCampamento);
        // Un token del campamento anterior no puede cerrar el actual.
        i.enviar(IntencionFrontend::AbandonarCampamento { campamento: id });
        assert!(i.ambiente_sonoro().fogata);
        assert!(i.error_recuperable.is_some());
        i.manejar_tecla(tecla(KeyCode::Char('q')));
        i.manejar_tecla(tecla(KeyCode::Char('s')));
        assert_eq!(i.ambiente_sonoro(), AmbienteSonoro::SILENCIO);
        i.salir = false;
        i.vista = VistaSesion::Fatal(desert_tiger_app::VistaFatal {
            codigo: desert_tiger_app::CodigoFatalSesion::Motor,
            mensaje: "fallo de prueba".into(),
        });
        assert_eq!(i.ambiente_sonoro(), AmbienteSonoro::SILENCIO);
    }

    #[test]
    fn campamento_por_teclas_conserva_ficha_al_cancelar_q_y_vuelve_a_viajar() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let mut interfaz = Interfaz::nueva(partida).unwrap();
        let tecla = |c| KeyEvent::new(c, KeyModifiers::NONE);
        assert!(!interfaz.ambiente_sonoro().fogata);
        interfaz.manejar_tecla(tecla(KeyCode::Char('a')));
        assert!(interfaz.ambiente_sonoro().fogata);
        assert!(matches!(interfaz.vista, VistaSesion::Acampando(_)));
        let antes = interfaz.texto_contextual();
        interfaz.manejar_tecla(tecla(KeyCode::Char('q')));
        assert!(interfaz.confirmar_salida);
        assert!(interfaz.ambiente_sonoro().fogata);
        interfaz.manejar_tecla(tecla(KeyCode::Char('n')));
        assert_eq!(interfaz.texto_contextual(), antes);
        let mut terminal = Terminal::new(TestBackend::new(79, 24)).unwrap();
        terminal.draw(|f| interfaz.dibujar(f, &recursos)).unwrap();
        assert!(interfaz.ambiente_sonoro().fogata);
        assert_eq!(interfaz.texto_contextual(), antes);
        terminal.backend_mut().resize(80, 24);
        terminal.resize(Rect::new(0, 0, 80, 24)).unwrap();
        terminal.draw(|f| interfaz.dibujar(f, &recursos)).unwrap();
        assert!(interfaz.campamento.is_some());
        interfaz.manejar_tecla(tecla(KeyCode::Esc));
        assert!(matches!(interfaz.vista, VistaSesion::Acampando(_)));
        interfaz.manejar_tecla(tecla(KeyCode::Enter));
        assert!(matches!(interfaz.vista, VistaSesion::Disponible(_)));
        assert!(interfaz.campamento.is_none());
        assert!(!interfaz.ambiente_sonoro().fogata);
        interfaz.manejar_tecla(tecla(KeyCode::Char('v')));
        assert!(matches!(interfaz.modo, ModoLocal::Seleccion { .. }));
    }

    fn partida_con_retrato_capas() -> Partida {
        use std::fs;
        use std::sync::atomic::{AtomicU64, Ordering};
        static NUMERO: AtomicU64 = AtomicU64::new(0);
        let destino = std::env::temp_dir().join(format!(
            "desert-tiger-retrato-capas-{}-{}",
            std::process::id(),
            NUMERO.fetch_add(1, Ordering::Relaxed)
        ));
        struct Temporal(PathBuf);
        impl Drop for Temporal {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
        fn copiar(origen: &std::path::Path, destino: &std::path::Path) {
            fs::create_dir_all(destino).unwrap();
            for entrada in fs::read_dir(origen).unwrap() {
                let entrada = entrada.unwrap();
                let siguiente = destino.join(entrada.file_name());
                if entrada.file_type().unwrap().is_dir() {
                    copiar(&entrada.path(), &siguiente);
                } else {
                    fs::copy(entrada.path(), siguiente).unwrap();
                }
            }
        }
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base");
        fs::create_dir(&destino).unwrap();
        let _temporal = Temporal(destino.clone());
        copiar(&raiz, &destino);
        fs::write(
            destino.join("actors/principe_perdido.ron"),
            r#"(
            clave: "principe_perdido", nombre: "Príncipe Perdido", ubicacion_inicial: EnLugar("zona_arida"),
            caracteristicas: [], relaciones: [], politica: Ninguna,
            retrato: Some((
                paleta: {
                    'b': (frente: "blanco", fondo: Some("negro")),
                    'e': (frente: "rojo", fondo: Some("verde")),
                    'p': (frente: "cian", fondo: None),
                },
                base: (arte: "abcd\nefgh\nijkl", estilos: Some("bbbb\nbbbb\nbbbb")),
                rostro: Some((origen: (x: 0, y: 0), expresiones: {
                    "enojado": (arte: "X ZW\n Q  ", estilos: Some("eeee\neeee")),
                    "sinEstilo": (arte: "a b", estilos: None),
                })),
                parpadeo: Some((origen: (x: 1, y: 0), intervalo_ms: 1000, duracion_ms: 250,
                    ojo_cerrado: (arte: " C ", estilos: Some("ppp")),
                )),
            )),
        )"#,
        )
        .unwrap();
        let contenido = cargar_contenido(&destino).unwrap();
        Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap()
    }

    #[test]
    fn expresion_y_ojo_se_superponen_con_transparencia_y_prioridad() {
        let mut interfaz = Interfaz::nueva(partida_con_retrato_capas()).unwrap();
        interfaz.ultima_vista_mundo.companeros.clear();
        let antes = interfaz.partida.vista();
        let actor = &interfaz.ultima_vista_mundo.jugador;
        let abierto = (0..4).find(|&p| !parpadeando(actor, p)).unwrap();
        let cerrado = (0..4).find(|&p| parpadeando(actor, p)).unwrap();
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal");
        let recursos = RecursosVisuales::cargar(&raiz).unwrap();
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        interfaz.reloj = RelojVisual::default();
        interfaz.reloj.avanzar(PASO_ANIMACION * abierto as u32);
        interfaz.ultima_vista_mundo.jugador.expresion = Some("enojado".into());
        terminal.draw(|f| interfaz.dibujar(f, &recursos)).unwrap();
        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(0, 19)].symbol(), "X");
        assert_eq!(buffer[(1, 19)].symbol(), "b");
        assert_eq!(buffer[(2, 19)].symbol(), "Z");
        assert_eq!(buffer[(1, 20)].symbol(), "Q");
        interfaz.reloj = RelojVisual::default();
        interfaz.reloj.avanzar(PASO_ANIMACION * cerrado as u32);
        terminal.draw(|f| interfaz.dibujar(f, &recursos)).unwrap();
        let buffer = terminal.backend().buffer();
        assert_eq!(buffer[(0, 19)].symbol(), "X");
        assert_eq!(buffer[(1, 19)].symbol(), "b");
        assert_eq!(buffer[(2, 19)].symbol(), "C");
        assert_eq!(buffer[(3, 19)].symbol(), "W");
        assert_eq!(buffer[(2, 19)].fg, Color::Cyan);
        assert_eq!(buffer[(2, 19)].bg, Color::Green);
        interfaz.reloj = RelojVisual::default();
        interfaz.reloj.avanzar(PASO_ANIMACION * abierto as u32);
        interfaz.ultima_vista_mundo.jugador.expresion = Some("inexistente".into());
        terminal.draw(|f| interfaz.dibujar(f, &recursos)).unwrap();
        assert_eq!(terminal.backend().buffer()[(0, 19)].symbol(), "a");
        assert_eq!(interfaz.partida.vista(), antes);
    }

    #[test]
    fn los_actores_no_comparten_todos_la_fase_de_parpadeo() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base");
        let contenido = cargar_contenido(&raiz).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let interfaz = Interfaz::nueva(partida).unwrap();
        let mut actor = interfaz.ultima_vista_mundo.jugador.clone();
        let periodo =
            usize::try_from(actor.retrato.parpadeo().unwrap().intervalo_ms() / 250).unwrap();
        let secuencias: Vec<Vec<bool>> = (0..5)
            .map(|id| {
                actor.id = desert_tiger_app::IdActor::desde_posicion(id).unwrap();
                (0..periodo).map(|p| parpadeando(&actor, p)).collect()
            })
            .collect();
        assert!(
            secuencias
                .iter()
                .all(|s| s.iter().any(|&cerrado| cerrado) && s.iter().any(|&cerrado| !cerrado))
        );
        assert!(secuencias[1..].iter().any(|s| s != &secuencias[0]));
    }

    #[test]
    fn acampar_antes_de_preparar_musica_espera_sin_notas_y_arranca_la_agenda_al_estar_lista() {
        use crate::musica_campamento::{AgendaMusica, ProgresoMusica};
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let mut i = Interfaz::nueva(partida).unwrap();
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        let estado = i.partida.vista();
        for segundos in [0, 1, 5, 10] {
            i.actualizar_musica(Duration::from_secs(segundos), None, true, None);
            assert_eq!(i.solicitud_musica(), 0);
            assert!(i.musica.notas().is_none());
            assert!(i.ambiente_sonoro().fogata);
        }
        let agenda = Some(AgendaMusica {
            inicial: (1, 1),
            pausa: (20, 180),
        });
        i.actualizar_musica(Duration::from_secs(10), None, true, agenda);
        assert_eq!(i.solicitud_musica(), 0);
        i.actualizar_musica(Duration::from_secs(11), None, true, agenda);
        let evento = i.solicitud_musica();
        assert_ne!(evento, 0);
        assert!(i.musica.notas().is_none());
        i.actualizar_musica(
            Duration::from_millis(11100),
            Some(ProgresoMusica {
                evento,
                transcurrido: Duration::from_millis(100),
                finalizada: false,
            }),
            true,
            agenda,
        );
        assert!(i.musica.notas().is_some());
        assert_eq!(i.partida.vista(), estado);
    }

    #[test]
    fn confirmacion_congela_y_reanuda_la_animacion() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida válida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        interfaz.avanzar_animacion(PASO_ANIMACION * 1);
        assert_eq!(interfaz.reloj.paso_250_ms(), 1);
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        interfaz.avanzar_animacion(PASO_ANIMACION * 3);
        assert_eq!(interfaz.reloj.paso_250_ms(), 1);
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        interfaz.avanzar_animacion(PASO_ANIMACION * 1);
        assert_eq!(interfaz.reloj.paso_250_ms(), 2);
    }

    #[test]
    fn detalle_largo_se_pagina_y_enter_no_confirma_la_vineta() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida válida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        // La primera vista puede ser una decisión de ruta o carga. Avanzamos
        // por la fachada hasta una escena automática, sin suponer su número.
        while let VistaSesion::EsperandoDecision(decision) = interfaz.vista.clone() {
            interfaz.enviar(IntencionFrontend::ResponderDecision {
                decision: decision.id,
                opcion: decision.opciones[0].id,
            });
        }
        let id = match &mut interfaz.vista {
            VistaSesion::PresentandoAccion(fragmento) => {
                fragmento.texto = "El grupo considera el propósito del viaje. ".repeat(20);
                fragmento.detalle = vec!["factor visible y consecuencia comprobada ".repeat(20)];
                fragmento.id
            }
            _ => panic!("se esperaba una viñeta"),
        };
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert!(interfaz.mostrar_detalle);
        assert!(interfaz.paginas_vigentes() > 1);
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(interfaz.pagina, 1);
        assert!(matches!(
            &interfaz.vista,
            VistaSesion::PresentandoAccion(fragmento) if fragmento.id == id
        ));
        interfaz.pagina = interfaz.paginas_vigentes() - 1;
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(matches!(
            &interfaz.vista,
            VistaSesion::PresentandoAccion(fragmento) if fragmento.id == id
        ));
    }

    #[test]
    fn decisiones_conservan_detalle_comun_paginan_y_restauran_su_opcion() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let mut i = Interfaz::nueva(partida).unwrap();
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
        i.manejar_tecla(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        while let VistaSesion::PresentandoAccion(f) = i.vista.clone() {
            i.enviar(IntencionFrontend::ContinuarPresentacion { fragmento: f.id });
        }
        let VistaSesion::EsperandoDecision(d) = &i.vista else {
            panic!("se esperaba decisión");
        };
        assert!(
            !d.detalle.is_empty(),
            "la aplicación conserva el detalle común"
        );
        assert!(d.opciones.iter().all(|o| o.detalle.is_none()));
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert!(i.mostrar_detalle, "D funciona aun sin detalles de opciones");
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        let VistaSesion::EsperandoDecision(d) = &mut i.vista else {
            panic!();
        };
        d.texto = (0..16)
            .map(|n| format!("relato_{n:02}"))
            .collect::<Vec<_>>()
            .join("\n");
        d.detalle = vec![
            (0..24)
                .map(|n| format!("factor_{n:02}"))
                .collect::<Vec<_>>()
                .join("\n"),
        ];
        d.opciones[0].detalle = Some("Detalle exclusivo de la primera alternativa".into());
        d.opciones[1].detalle = Some("Detalle exclusivo de la segunda alternativa".into());
        let atajo = atajos_decision(d)[1].0;
        let paginas_relato = paginar_texto(&d.texto).len();
        let vista = i.vista.clone();
        let sesion = i.partida.vista();
        // Antes de completar el relato, las teclas de opciones no actúan.
        i.manejar_tecla(KeyEvent::new(KeyCode::Char(atajo), KeyModifiers::NONE));
        assert_eq!(i.vista, vista);
        let mut leido = String::new();
        for pagina in 0..paginas_relato {
            i.pagina = pagina;
            let cuadro = cuadro_campamento(&i, &recursos, 80, 24);
            leido.extend(cuadro.content.iter().flat_map(|c| c.symbol().chars()));
        }
        assert!(
            leido.contains("relato_15"),
            "arte y texto caben en la ventana"
        );
        // Seleccionamos la segunda página de opciones sin responderla.
        i.pagina = paginas_relato;
        let pagina_opcion = i.pagina;
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert_eq!(i.pagina, 0);
        assert!(!i.texto_acciones().contains("segunda alternativa"));
        leido.clear();
        for pagina in 0..i.paginas_vigentes() {
            i.pagina = pagina;
            for (ancho, alto) in [(80, 24), (110, 35)] {
                let cuadro = cuadro_campamento(&i, &recursos, ancho, alto);
                leido.extend(cuadro.content.iter().flat_map(|c| c.symbol().chars()));
            }
            i.manejar_tecla(KeyEvent::new(KeyCode::Char(atajo), KeyModifiers::NONE));
            assert_eq!(i.vista, vista);
            assert_eq!(i.partida.vista(), sesion);
        }
        for n in 0..24 {
            assert!(leido.contains(&format!("factor_{n:02}")));
        }
        assert!(leido.contains("Detalle exclusivo de la segunda alternativa"));
        assert!(!leido.contains("Detalle exclusivo de la primera alternativa"));
        assert!(
            !leido.contains("relato_00"),
            "el desglose no duplica el relato"
        );
        let ultima = i.pagina;
        i.manejar_tecla(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(i.pagina, ultima);
        assert_eq!(i.partida.vista(), sesion);
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert_eq!(i.pagina, pagina_opcion);
        assert!(!i.mostrar_detalle);
        i.manejar_tecla(KeyEvent::new(KeyCode::Char(atajo), KeyModifiers::NONE));
        assert_ne!(
            i.partida.vista(),
            sesion,
            "el atajo recupera la opción correcta"
        );
    }

    #[test]
    fn paginacion_de_texto_no_descarta_contenido() {
        let texto = (0..30)
            .map(|indice| format!("línea causal {indice}: intención, esfuerzo y consecuencia"))
            .collect::<Vec<_>>()
            .join("\n");
        let paginas = paginar_texto(&texto);
        assert!(paginas.len() > 3);
        for indice in 0..30 {
            assert!(
                paginas
                    .iter()
                    .any(|pagina| pagina.contains(&format!("línea causal {indice}:")))
            );
        }
    }

    #[test]
    fn viaje_completo_renderiza_cada_paso_y_redimensiona_sin_avanzar() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let mut interfaz = Interfaz::nueva(partida).unwrap();
        let destino = interfaz
            .ultima_vista_mundo
            .lugares
            .iter()
            .find(|l| l.nombre == "Mar de Levante")
            .unwrap()
            .id;
        interfaz.enviar(IntencionFrontend::ComenzarViaje { destino });
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let mut pasos = 0;
        loop {
            terminal
                .draw(|frame| interfaz.dibujar(frame, &recursos))
                .unwrap();
            pasos += 1;
            assert!(pasos < 500);
            match interfaz.vista.clone() {
                VistaSesion::PresentandoAccion(r) => {
                    let anterior = interfaz.vista.clone();
                    terminal.backend_mut().resize(79, 24);
                    terminal.resize(Rect::new(0, 0, 79, 24)).unwrap();
                    terminal
                        .draw(|frame| interfaz.dibujar(frame, &recursos))
                        .unwrap();
                    assert_eq!(interfaz.vista, anterior);
                    terminal.backend_mut().resize(80, 24);
                    terminal.resize(Rect::new(0, 0, 80, 24)).unwrap();
                    interfaz.enviar(IntencionFrontend::ContinuarPresentacion { fragmento: r.id });
                }
                VistaSesion::EsperandoDecision(d) => {
                    interfaz.enviar(IntencionFrontend::ResponderDecision {
                        decision: d.id,
                        opcion: d.opciones[0].id,
                    })
                }
                VistaSesion::Disponible(_) => break,
                VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
            }
        }
        assert!(pasos > 4);
    }

    #[test]
    fn lista_destinos_ordena_nombres_y_solo_confirma_al_pulsar_enter() {
        use desert_tiger_app::CoordenadaMapa;
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let mut i = Interfaz::nueva(partida).unwrap();
        let mut mundo = i.ultima_vista_mundo.clone();
        let destinos = destinos_viaje(&mundo).unwrap()[..3].to_vec();
        for ((id, nombre), (x, y)) in destinos
            .iter()
            .copied()
            .zip(["Zoco", "Abadía", "Monte"])
            .zip([(50, 50), (55, 50), (55, 55)])
        {
            let lugar = mundo.lugares.iter_mut().find(|l| l.id == id).unwrap();
            lugar.nombre = format!("Nombre completo de {nombre}");
            lugar.nombre_abreviado = nombre.into();
            lugar.coordenada_central = CoordenadaMapa { x, y };
        }
        mundo
            .lugares
            .iter_mut()
            .find(|l| l.id == mundo.lugar_jugador)
            .unwrap()
            .coordenada_central = CoordenadaMapa { x: 49, y: 50 };
        mundo.acciones = vec![AccionDisponible::Viajar {
            destinos: destinos.clone(),
        }];
        i.ultima_vista_mundo = mundo.clone();
        i.vista = VistaSesion::Disponible(mundo);
        let antes = i.partida.vista();
        let tecla = |code| KeyEvent::new(code, KeyModifiers::NONE);
        i.manejar_tecla(tecla(KeyCode::Char('v')));
        let ordenados = vec![destinos[1], destinos[2], destinos[0]];
        assert!(
            matches!(&i.modo, ModoLocal::Seleccion { destinos, foco: 2 } if destinos == &ordenados)
        );
        assert!(i.texto_acciones().contains("↑/↓"));
        for (direccion, esperado) in [
            (KeyCode::Down, 2),
            (KeyCode::Up, 1),
            (KeyCode::Up, 0),
            (KeyCode::Up, 0),
            (KeyCode::Down, 1),
        ] {
            i.manejar_tecla(tecla(direccion));
            assert!(matches!(i.modo, ModoLocal::Seleccion { foco, .. } if foco == esperado));
            assert_eq!(i.partida.vista(), antes);
            assert_eq!(
                i.objetivo_camara(&i.ultima_vista_mundo),
                i.ultima_vista_mundo
                    .lugares
                    .iter()
                    .find(|l| l.id == ordenados[esperado])
                    .unwrap()
                    .coordenada_central
            );
        }
        i.manejar_tecla(tecla(KeyCode::Esc));
        assert!(matches!(i.modo, ModoLocal::Normal));
        assert_eq!(i.partida.vista(), antes);
        i.manejar_tecla(tecla(KeyCode::Char('v')));
        i.manejar_tecla(tecla(KeyCode::Up));
        i.manejar_tecla(tecla(KeyCode::Enter));
        let progreso = match &i.vista {
            VistaSesion::PresentandoAccion(v) => &v.progreso,
            VistaSesion::EsperandoDecision(v) => &v.progreso,
            otra => panic!("vista inesperada: {otra:?}"),
        };
        assert_eq!(progreso.destino_final, destinos[2]);
    }

    #[test]
    fn lista_lateral_desplaza_el_foco_y_conserva_mapa_y_barras_al_redimensionar() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let mut i = Interfaz::nueva(partida).unwrap();
        let destinos = destinos_viaje(&i.ultima_vista_mundo).unwrap().to_vec();
        assert!(
            destinos.len() > 20,
            "el contenido permite probar desplazamiento"
        );
        for (n, id) in destinos.iter().enumerate() {
            let lugar = i
                .ultima_vista_mundo
                .lugares
                .iter_mut()
                .find(|l| l.id == *id)
                .unwrap();
            lugar.nombre = format!("Nombre completo del destino {n:02}");
            lugar.nombre_abreviado = format!("Destino {n:02}");
        }
        i.modo = ModoLocal::Seleccion {
            destinos: destinos.clone(),
            foco: 0,
        };
        let antes = i.partida.vista();
        for foco in 0..destinos.len() {
            for (ancho, alto) in [(80, 24), (120, 35)] {
                let cuadro = cuadro_campamento(&i, &recursos, ancho, alto);
                let fila = |y| {
                    (0..ancho)
                        .map(|x| cuadro[(x, y)].symbol())
                        .collect::<String>()
                };
                let inicio_lista = ancho - 18;
                let lista = (0..alto - 2)
                    .map(|y| {
                        (inicio_lista..ancho)
                            .map(|x| cuadro[(x, y)].symbol())
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                assert!(lista.contains(&format!("> Destino {foco:02}")), "{lista}");
                assert!(!lista.contains("Nombre completo"));
                assert!(lista.contains(&format!("{}/{}", foco + 1, destinos.len())));
                assert!(fila(alto - 1).contains("[↑/↓] Destino"));
                assert!(fila(alto - 2).contains(&format!("Nombre completo del destino {foco:02}")));
                assert!(
                    (0..alto - 2)
                        .any(|y| (0..inicio_lista).any(|x| cuadro[(x, y)].symbol() != " "))
                );
            }
            assert_eq!(i.partida.vista(), antes);
            i.manejar_tecla(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        }
        assert!(matches!(i.modo, ModoLocal::Seleccion { foco, .. } if foco == destinos.len() - 1));
    }

    #[test]
    fn mosaico_conserva_la_celda_global_al_mover_la_camara() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida válida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos válidos");
        let bosque = desert_tiger_app::ClaveTipoTile::nueva("bosque".to_owned()).unwrap();
        let recursos = recursos.con_fotogramas_prueba(bosque.clone(), &["ab\ncd", "wx\nyz"]);
        let mut mundo = interfaz.ultima_vista_mundo.clone();
        mundo.mapa.ancho = 120;
        mundo.mapa.alto = 60;
        mundo.mapa.celdas = vec![bosque.clone(); 120 * 60];
        mundo.mapa.tipos_declarados = vec![bosque.clone()];
        for lugar in &mut mundo.lugares {
            lugar.coordenadas_rotulo.clear();
        }
        mundo.lugares[0].coordenada_central = desert_tiger_app::CoordenadaMapa { x: 50, y: 30 };
        mundo.lugares[1].coordenada_central = desert_tiger_app::CoordenadaMapa { x: 51, y: 31 };
        let destinos = vec![mundo.lugares[0].id, mundo.lugares[1].id];
        mundo.lugar_jugador = destinos[0];
        let area = Rect::new(0, 0, 80, 22);
        let esperado = recursos.tile(&bosque, 40, 21, 0);
        // Estas fases del mosaico distinguen coordenadas de mapa y de pantalla.
        assert_ne!(esperado.glifo, recursos.tile(&bosque, 30, 2, 0).glifo);
        assert_ne!(esperado.glifo, recursos.tile(&bosque, 29, 1, 0).glifo);
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        interfaz.modo = ModoLocal::Seleccion {
            destinos: destinos.clone(),
            foco: 0,
        };
        terminal
            .draw(|frame| interfaz.dibujar_mundo(frame, area, &mundo, &recursos))
            .unwrap();
        // La celda global (40, 21) queda en una fila libre de retratos.
        let primera = terminal.backend().buffer()[(30, 2)].clone();
        assert_eq!(primera.symbol(), esperado.glifo.to_string());
        assert_eq!(primera.fg, esperado.estilo.fg.unwrap_or(Color::Reset));
        assert_eq!(primera.bg, esperado.estilo.bg.unwrap_or(Color::Reset));
        assert_eq!(primera.modifier, esperado.estilo.add_modifier);

        interfaz.modo = ModoLocal::Seleccion { destinos, foco: 1 };
        terminal
            .draw(|frame| interfaz.dibujar_mundo(frame, area, &mundo, &recursos))
            .unwrap();
        let segunda = &terminal.backend().buffer()[(29, 1)];
        assert_eq!(
            &primera, segunda,
            "mover la cámara en ambos ejes debe conservar el glifo y estilo de la celda global"
        );
        let vista_antes = interfaz.partida.vista();
        interfaz.avanzar_animacion(PASO_ANIMACION * 2);
        terminal
            .draw(|frame| interfaz.dibujar_mundo(frame, area, &mundo, &recursos))
            .unwrap();
        let animada = &terminal.backend().buffer()[(29, 1)];
        assert_eq!(animada.symbol(), "y");
        assert_eq!(animada.fg, primera.fg);
        assert_eq!(animada.bg, primera.bg);
        assert_eq!(interfaz.partida.vista(), vista_antes);
    }

    #[test]
    fn jugador_a_la_derecha_conserva_transparencia_colores_y_parpadeo() {
        assert_eq!(color_retrato(ColorRetrato::Negro), Color::Black);
        assert_eq!(
            color_retrato(ColorRetrato::GrisMuyOscuro),
            Color::Rgb(32, 32, 32)
        );

        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido valido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida valida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz valida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos validos");
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("backend valido");
        let actor = &interfaz.ultima_vista_mundo.jugador;
        let periodo =
            usize::try_from(actor.retrato.parpadeo().unwrap().intervalo_ms() / 250).unwrap();
        let abierto = (0..periodo).find(|&p| !parpadeando(actor, p)).unwrap();
        let cerrado = (0..periodo).find(|&p| parpadeando(actor, p)).unwrap();
        interfaz.reloj = RelojVisual::default();
        interfaz.reloj.avanzar(PASO_ANIMACION * cerrado as u32);
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("primer fotograma valido");
        let cerrado_buffer = terminal.backend().buffer().clone();
        interfaz.reloj = RelojVisual::default();
        interfaz.reloj.avanzar(PASO_ANIMACION * abierto as u32);
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("segundo fotograma valido");
        let abierto_buffer = terminal.backend().buffer();
        // El ojo del príncipe identifica su posición sin fijar cuántos compañeros
        // eligió el diseñador ni reproducir el algoritmo de solapamiento.
        let ojo_x = (1..80_u16)
            .find(|&x| {
                let ojo = &abierto_buffer[(x, 15)];
                // El contorno de la nariz es contenido artístico editable.
                ojo.symbol() == "o" && ojo.fg == Color::Cyan
            })
            .expect("el ojo del príncipe es visible");
        let mundo = &interfaz.ultima_vista_mundo;
        if !mundo.companeros.is_empty() {
            assert!(
                ojo_x > 14,
                "el jugador debe aparecer después de sus compañeros"
            );
        }
        let inicio_jugador = ojo_x - 14;
        let area_mundo = Rect::new(0, 0, 80, 22);
        let objetivo = interfaz.objetivo_camara(mundo);
        let (origen_x, margen_x) =
            origen_eje(mundo.mapa.ancho, usize::from(area_mundo.width), objetivo.x);
        let (origen_y, margen_y) =
            origen_eje(mundo.mapa.alto, usize::from(area_mundo.height), objetivo.y);

        let mapa_x = usize::from(ojo_x).checked_sub(margen_x).unwrap() + origen_x;
        let mapa_y = 15usize.checked_sub(margen_y).unwrap() + origen_y;
        let fondo_mapa = recursos
            .tile(
                &mundo.mapa.celdas[mapa_y * mundo.mapa.ancho + mapa_x],
                mapa_x,
                mapa_y,
                interfaz.reloj.paso_250_ms(),
            )
            .estilo
            .bg
            .or(recursos.ui.pantalla.bg)
            .unwrap_or(Color::Reset);

        let cerrado = &cerrado_buffer[(ojo_x, 15)];
        assert_eq!(cerrado.symbol(), "▀");
        assert_eq!(cerrado.fg, Color::LightMagenta);
        // Base y parpadeo heredan el lienzo, como el retrato del sirviente.
        assert_eq!(cerrado.bg, fondo_mapa);
        let transparente = terminal.backend().buffer()[(inicio_jugador, 8)].symbol();
        let mapa_transparente_x = usize::from(inicio_jugador)
            .checked_sub(margen_x)
            .and_then(|x| x.checked_add(origen_x))
            .expect("la celda transparente se proyecta dentro del mapa");
        let mapa_transparente_y = 8usize
            .checked_sub(margen_y)
            .and_then(|y| y.checked_add(origen_y))
            .expect("la celda transparente se proyecta dentro del mapa");
        let clave_transparente =
            &mundo.mapa.celdas[mapa_transparente_y * mundo.mapa.ancho + mapa_transparente_x];
        assert_eq!(
            transparente,
            recursos
                .tile(
                    clave_transparente,
                    mapa_transparente_x,
                    mapa_transparente_y,
                    interfaz.reloj.paso_250_ms(),
                )
                .glifo
                .to_string(),
            "una celda transparente debe conservar el glifo vigente del mapa"
        );

        let abierto = &abierto_buffer[(ojo_x, 15)];
        assert_eq!(abierto.symbol(), "o");
        assert_eq!(abierto.fg, Color::Cyan);
        assert_eq!(abierto.bg, fondo_mapa);
    }

    fn cuadro_campamento(
        interfaz: &Interfaz,
        recursos: &RecursosVisuales,
        ancho: u16,
        alto: u16,
    ) -> ratatui::buffer::Buffer {
        let mut t = Terminal::new(TestBackend::new(ancho, alto)).unwrap();
        t.draw(|f| interfaz.dibujar(f, recursos)).unwrap();
        t.backend().buffer().clone()
    }

    #[test]
    fn fogata_cambia_solo_el_arte_y_conserva_pausa_resize_y_estilos_de_la_ficha() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        let tecla = |c| KeyEvent::new(c, KeyModifiers::NONE);
        i.manejar_tecla(tecla(KeyCode::Char('a')));
        assert_eq!(i.espera_animacion(&recursos), PASO_FOGATA);
        i.avanzar_animacion(Duration::from_millis(249));
        assert_eq!(i.reloj.paso_250_ms(), 0);
        assert_eq!(i.espera_animacion(&recursos), Duration::from_millis(1));
        let antes = i.partida.vista();
        let animado = cuadro_campamento(&i, &recursos, 80, 24);
        assert_eq!(animado, cuadro_campamento(&i, &recursos, 80, 24));
        let config = recursos.efecto_fogata.take();
        assert_eq!(i.espera_animacion(&recursos), Duration::from_millis(1));
        let estatico = cuadro_campamento(&i, &recursos, 80, 24);
        let alto = i
            .ultima_vista_mundo
            .companeros
            .iter()
            .chain(std::iter::once(&i.ultima_vista_mundo.jugador))
            .map(|a| a.retrato.base().alto())
            .max()
            .unwrap();
        assert!(alto >= 10);
        let fin_ficha = 22 - alto.min(16) as u16;
        for y in (0..fin_ficha).chain(22..24) {
            for x in 0..80 {
                assert_eq!(animado[(x, y)], estatico[(x, y)]);
            }
        }
        assert_ne!(animado, estatico);
        recursos.efecto_fogata = config;
        i.manejar_tecla(tecla(KeyCode::Char('q')));
        let pausa = cuadro_campamento(&i, &recursos, 80, 24);
        i.avanzar_animacion(Duration::from_secs(30));
        assert_eq!(i.espera_animacion(&recursos), PASO_ANIMACION);
        assert_eq!(pausa, cuadro_campamento(&i, &recursos, 80, 24));
        i.manejar_tecla(tecla(KeyCode::Esc));
        assert_eq!(animado, cuadro_campamento(&i, &recursos, 80, 24));
        let _ = cuadro_campamento(&i, &recursos, 120, 40);
        assert_eq!(animado, cuadro_campamento(&i, &recursos, 80, 24));
        let _ = cuadro_campamento(&i, &recursos, 79, 23);
        assert_eq!(animado, cuadro_campamento(&i, &recursos, 80, 24));
        i.avanzar_animacion(Duration::from_millis(1));
        assert_eq!(i.reloj.paso_250_ms(), 1);
        assert_eq!(i.reloj.instante_ms(), 250);
        i.avanzar_animacion(Duration::from_millis(500));
        assert_ne!(animado, cuadro_campamento(&i, &recursos, 80, 24));
        assert_eq!(i.partida.vista(), antes);
    }

    #[test]
    fn animar_campamento_no_cambia_el_cierre_ni_el_azar_del_viaje_siguiente() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let recorrer = |animar: bool| {
            let mut i = Interfaz::nueva(
                Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
            )
            .unwrap();
            let tecla = |c| KeyEvent::new(c, KeyModifiers::NONE);
            assert_eq!(i.espera_animacion(&recursos), PASO_ANIMACION);
            i.manejar_tecla(tecla(KeyCode::Char('a')));
            let inicial = i.partida.vista();
            if animar {
                for _ in 0..30 {
                    i.avanzar_animacion(Duration::from_millis(53));
                    let _ = cuadro_campamento(&i, &recursos, 80, 24);
                }
            }
            assert_eq!(i.partida.vista(), inicial);
            i.manejar_tecla(tecla(KeyCode::Esc));
            i.manejar_tecla(tecla(KeyCode::Enter));
            assert!(matches!(i.vista, VistaSesion::Disponible(_)));
            let mut vistas = vec![i.vista.clone()];
            let destino = i
                .ultima_vista_mundo
                .acciones
                .iter()
                .find_map(|a| match a {
                    AccionDisponible::Viajar { destinos } => destinos.first().copied(),
                    AccionDisponible::Acampar => None,
                })
                .unwrap();
            i.enviar(IntencionFrontend::ComenzarViaje { destino });
            for _ in 0..500 {
                vistas.push(i.vista.clone());
                match i.vista.clone() {
                    VistaSesion::PresentandoAccion(f) => {
                        i.enviar(IntencionFrontend::ContinuarPresentacion { fragmento: f.id })
                    }
                    VistaSesion::EsperandoDecision(d) => {
                        i.enviar(IntencionFrontend::ResponderDecision {
                            decision: d.id,
                            opcion: d.opciones[0].id,
                        })
                    }
                    VistaSesion::Disponible(_) => return vistas,
                    otra => panic!("vista inesperada: {otra:?}"),
                }
            }
            panic!("el viaje no terminó");
        };
        assert_eq!(recorrer(false), recorrer(true));
    }
    #[test]
    fn paleta_consultada_solo_afecta_luz_y_modo_fijo_cubre_la_pantalla_completa() {
        use crate::colores::PaletaColores;
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        let normal = cuadro_campamento(&i, &recursos, 80, 24);
        recursos.paleta.registrar(13, [187, 136, 170]);
        assert_eq!(normal, cuadro_campamento(&i, &recursos, 80, 24));
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        let parcial = cuadro_campamento(&i, &recursos, 80, 24);
        recursos.paleta = PaletaColores::default();
        let desconocida = cuadro_campamento(&i, &recursos, 80, 24);
        assert!(
            parcial
                .content
                .iter()
                .zip(&desconocida.content)
                .any(|(a, b)| a.fg != b.fg || a.bg != b.bg)
        );
        for (a, b) in parcial.content.iter().zip(&desconocida.content) {
            assert_eq!(a.symbol(), b.symbol());
            if b.fg == Color::Blue {
                assert_eq!(a.fg, Color::Blue);
            }
            if b.bg == Color::Blue {
                assert_eq!(a.bg, Color::Blue);
            }
        }
        recursos.paleta.registrar(13, [187, 136, 170]);
        recursos.paleta.registrar(4, [80, 90, 140]);
        assert_ne!(parcial, cuadro_campamento(&i, &recursos, 80, 24));
        recursos.paleta = PaletaColores::fija();
        for (ancho, alto) in [(80, 24), (79, 23)] {
            for celda in &cuadro_campamento(&i, &recursos, ancho, alto).content {
                assert!(matches!(celda.fg, Color::Rgb(..)));
                assert!(matches!(celda.bg, Color::Rgb(..)));
            }
        }
        i.manejar_tecla(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        for celda in &cuadro_campamento(&i, &recursos, 80, 24).content {
            assert!(matches!(celda.fg, Color::Rgb(..)) && matches!(celda.bg, Color::Rgb(..)));
        }
    }
    #[test]
    fn fondo_negro_solo_cambia_el_lienzo_y_conserva_arte_y_colores() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut r = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let mut i = Interfaz::nueva(
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap(),
        )
        .unwrap();
        for (acampar, modal, ancho, alto) in [
            (false, false, 80, 24),
            (true, false, 80, 24),
            (true, true, 80, 24),
            (true, true, 79, 23),
        ] {
            if acampar && i.campamento.is_none() {
                i.manejar_tecla(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
            }
            if modal && !i.confirmar_salida {
                i.manejar_tecla(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
            }
            r.paleta.forzar_fondo_negro(false);
            let nativo = cuadro_campamento(&i, &r, ancho, alto);
            r.paleta.forzar_fondo_negro(true);
            let negro = cuadro_campamento(&i, &r, ancho, alto);
            if ancho >= ANCHO_MINIMO && alto >= ALTO_MINIMO {
                assert!(nativo.content.iter().any(|c| c.bg == Color::Reset));
            }
            for (n, b) in nativo.content.iter().zip(&negro.content) {
                assert_eq!(n.symbol(), b.symbol());
                assert_eq!(n.fg, b.fg);
                assert_eq!(n.modifier, b.modifier);
                assert_eq!(
                    b.bg,
                    if n.bg == Color::Reset {
                        NEGRO_PURO
                    } else {
                        n.bg
                    }
                );
            }
        }
    }
}
