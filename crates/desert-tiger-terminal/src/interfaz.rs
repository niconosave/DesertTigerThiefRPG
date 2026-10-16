use std::collections::BTreeSet;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use desert_tiger_app::{
    AccionDisponible, CausaIndisponibilidad, ColorRetrato, EntradaResumenNpc, ErrorIntencion,
    EstiloCeldaRetrato, IdLugar, IntencionFrontend, Partida, VistaDecision, VistaMundo,
    VistaSesion,
};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::recursos::RecursosVisuales;

const ANCHO_MINIMO: u16 = 80;
const ALTO_MINIMO: u16 = 24;

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
    fotograma: usize,
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
            fotograma: 0,
        })
    }

    pub(crate) fn debe_salir(&self) -> bool {
        self.salir
    }

    pub(crate) fn diagnostico_fatal(&self) -> Option<String> {
        match &self.vista {
            VistaSesion::Fatal(fatal) => Some(fatal.mensaje.clone()),
            _ => None,
        }
    }

    pub(crate) fn avanzar_animacion(&mut self, pasos: usize) {
        if !self.confirmar_salida {
            self.fotograma = self.fotograma.wrapping_add(pasos);
        }
    }

    pub(crate) fn manejar_tecla(&mut self, evento: KeyEvent) {
        if evento.kind != KeyEventKind::Press {
            return;
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
        match &mut self.modo {
            ModoLocal::Seleccion { destinos, foco } => match evento.code {
                KeyCode::Left => *foco = foco.saturating_sub(1),
                KeyCode::Right if *foco + 1 < destinos.len() => *foco += 1,
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
        match &self.vista {
            VistaSesion::Disponible(mundo) if normalizar_letra(tecla) == Some('v') => {
                if let Some(destinos) =
                    destinos_viaje(mundo).filter(|destinos| !destinos.is_empty())
                {
                    self.modo = ModoLocal::Seleccion {
                        destinos: destinos.to_vec(),
                        foco: 0,
                    };
                    self.error_recuperable = None;
                }
            }
            VistaSesion::PresentandoAccion(fragmento) if tecla == KeyCode::Enter => {
                self.enviar(IntencionFrontend::ContinuarPresentacion {
                    fragmento: fragmento.id,
                });
            }
            VistaSesion::EsperandoDecision(decision) => {
                if let Some(letra) = normalizar_letra(tecla)
                    && let Some((_, opcion)) = atajos_decision(decision)
                        .into_iter()
                        .find(|(atajo, _)| *atajo == letra)
                {
                    self.enviar(IntencionFrontend::ResponderDecision {
                        decision: decision.id,
                        opcion,
                    });
                }
            }
            VistaSesion::Fatal(_)
            | VistaSesion::Disponible(_)
            | VistaSesion::PresentandoAccion(_) => {}
        }
    }

    fn enviar(&mut self, intencion: IntencionFrontend) {
        match self.partida.procesar(intencion) {
            Ok(vista) => {
                self.vista = vista;
                self.error_recuperable = None;
                self.actualizar_mundo();
            }
            Err(error) => self.error_recuperable = Some(texto_error_intencion(error)),
        }
    }

    fn actualizar_mundo(&mut self) {
        if let VistaSesion::Disponible(mundo) = &self.vista {
            self.ultima_vista_mundo = mundo.clone();
            self.modo = ModoLocal::Normal;
        }
    }

    pub(crate) fn dibujar(&self, frame: &mut Frame<'_>, recursos: &RecursosVisuales) {
        let area = frame.area();
        frame.render_widget(Block::default().style(recursos.ui.pantalla), area);
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
        self.dibujar_mundo(frame, region_mundo, mundo, recursos);
        let contextual = Rect::new(area.x, area.y + area.height - 2, area.width, 1);
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
            VistaSesion::PresentandoAccion(fragmento) => self.dibujar_ventana(
                frame,
                region_mundo,
                "Suceso",
                &fragmento.texto,
                fragmento.escena,
                recursos,
            ),
            VistaSesion::EsperandoDecision(decision) => self.dibujar_ventana(
                frame,
                region_mundo,
                "Decisión",
                &decision.texto,
                decision.escena,
                recursos,
            ),
            VistaSesion::Fatal(fatal) => {
                self.dibujar_ventana(frame, region_mundo, "Error", &fatal.mensaje, None, recursos)
            }
            VistaSesion::Disponible(_) => {}
        }
        if self.confirmar_salida {
            self.dibujar_confirmacion(frame, region_mundo, recursos);
        }
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
                let apariencia = recursos.tile(clave);
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
        let mut retratos = Vec::with_capacity(mundo.companeros.len() + 1);
        retratos.push(&mundo.jugador.retrato);
        retratos.extend(mundo.companeros.iter().map(|actor| &actor.retrato));
        let mut x = area.x;
        let base_y = area.y + area.height;
        let mut anterior: Option<usize> = None;
        for retrato in retratos {
            let fotograma = &retrato.fotogramas()[self.fotograma % retrato.fotogramas().len()];
            if let Some(ancho_anterior) = anterior {
                let solapamiento = 1.max(ancho_anterior.min(fotograma.ancho()) / 4);
                x = x.saturating_add(
                    u16::try_from(ancho_anterior - solapamiento).unwrap_or(u16::MAX),
                );
            }
            let alto = u16::try_from(fotograma.alto()).unwrap_or(u16::MAX);
            let inicio_y = base_y.saturating_sub(alto);
            for (indice, celda) in fotograma.celdas().iter().enumerate() {
                if celda.glifo == ' ' {
                    continue;
                }
                let fila = indice / fotograma.ancho();
                let columna = indice % fotograma.ancho();
                let px = x.saturating_add(u16::try_from(columna).unwrap_or(u16::MAX));
                let py = inicio_y.saturating_add(u16::try_from(fila).unwrap_or(u16::MAX));
                if px < area.right() && py >= area.y && py < area.bottom() {
                    let estilo = celda
                        .estilo
                        .map(estilo_retrato)
                        .unwrap_or(recursos.ui.destacado);
                    poner_celda(frame, px, py, celda.glifo, estilo);
                }
            }
            anterior = Some(fotograma.ancho());
        }
    }

    fn objetivo_camara(&self, mundo: &VistaMundo) -> desert_tiger_app::CoordenadaMapa {
        let id = match &self.modo {
            ModoLocal::Seleccion { destinos, foco } => destinos[*foco],
            ModoLocal::Normal => mundo.lugar_jugador,
        };
        mundo
            .lugares
            .iter()
            .find(|lugar| lugar.id == id)
            .map(|lugar| lugar.coordenada_central)
            .unwrap_or(desert_tiger_app::CoordenadaMapa { x: 0, y: 0 })
    }

    fn texto_contextual(&self) -> String {
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
            VistaSesion::PresentandoAccion(_) => {
                "Lee lo sucedido y continúa cuando estés listo.".to_owned()
            }
            VistaSesion::EsperandoDecision(_) => "Elige cómo continuar.".to_owned(),
            VistaSesion::Fatal(_) => "La sesión encontró un error fatal.".to_owned(),
            VistaSesion::Disponible(mundo) => contexto_mundo(mundo),
        }
    }

    fn estilo_contextual(&self, recursos: &RecursosVisuales) -> Style {
        if self.error_recuperable.is_some() || matches!(self.vista, VistaSesion::Fatal(_)) {
            recursos.ui.error
        } else {
            recursos.ui.barra_contextual
        }
    }

    fn texto_acciones(&self) -> String {
        if self.confirmar_salida {
            return " [S] Sí  [N] No  [Esc] Cancelar".to_owned();
        }
        match &self.modo {
            ModoLocal::Seleccion { .. } => {
                " [←/→] Destino  [Enter] Confirmar  [Esc] Cancelar  [Q] Salir".to_owned()
            }
            ModoLocal::Normal => match &self.vista {
                VistaSesion::Disponible(mundo) => match destinos_viaje(mundo) {
                    Some([]) => " Viajar (no disponible)  [Q] Salir".to_owned(),
                    Some(_) => " [V] Viajar  [Q] Salir".to_owned(),
                    None => " [Q] Salir".to_owned(),
                },
                VistaSesion::PresentandoAccion(_) => " [Enter] Continuar  [Q] Salir".to_owned(),
                VistaSesion::EsperandoDecision(decision) => {
                    let opciones = atajos_decision(decision)
                        .into_iter()
                        .zip(&decision.opciones)
                        .map(|((tecla, _), opcion)| {
                            format!("[{}] {}", tecla.to_ascii_uppercase(), opcion.texto)
                        })
                        .collect::<Vec<_>>()
                        .join("  ");
                    format!(" {opciones}  [Q] Salir")
                }
                VistaSesion::Fatal(_) => String::new(),
            },
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
        let alto = 12.min(area.height.saturating_sub(2)).max(7);
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
        if let Some(arte) = escena.and_then(|clave| recursos.escena(clave)) {
            lineas.extend(
                arte.iter()
                    .map(|fila| Line::styled(fila.clone(), recursos.ui.destacado)),
            );
            lineas.push(Line::default());
        }
        lineas.push(Line::styled(texto.to_owned(), recursos.ui.texto_normal));
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

fn destinos_viaje(mundo: &VistaMundo) -> Option<&[IdLugar]> {
    match mundo.acciones.first()? {
        AccionDisponible::Viajar { destinos } => Some(destinos.as_slice()),
    }
}

fn atajos_decision(decision: &VistaDecision) -> Vec<(char, desert_tiger_app::IdOpcionVista)> {
    let mut usados = BTreeSet::from(['q']);
    decision
        .opciones
        .iter()
        .map(|opcion| {
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

fn contexto_mundo(mundo: &VistaMundo) -> String {
    if mundo.causa_indisponibilidad == Some(CausaIndisponibilidad::JugadorMuerto) {
        return "El personaje jugador está muerto y no puede actuar.".to_owned();
    }
    if let Some(resumen) = &mundo.resumen_npc_reciente
        && let Some(entrada) = resumen.entradas().first()
    {
        return match entrada {
            EntradaResumenNpc::ViajeCompletado { .. } => {
                "Un actor completó un viaje mientras transcurría el tiempo.".to_owned()
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

fn texto_error_intencion(error: ErrorIntencion) -> String {
    match error {
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

fn poner_celda(frame: &mut Frame<'_>, x: u16, y: u16, caracter: char, estilo: Style) {
    if let Some(celda) = frame.buffer_mut().cell_mut((x, y)) {
        celda.set_char(caracter).set_style(estilo);
    }
}

fn estilo_retrato(estilo: EstiloCeldaRetrato) -> Style {
    let mut resultado = Style::default().fg(color_retrato(estilo.frente));
    if let Some(fondo) = estilo.fondo {
        resultado = resultado.bg(color_retrato(fondo));
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
    fn confirmacion_congela_y_reanuda_la_animacion() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida válida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        interfaz.avanzar_animacion(1);
        assert_eq!(interfaz.fotograma, 1);
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        interfaz.avanzar_animacion(3);
        assert_eq!(interfaz.fotograma, 1);
        interfaz.manejar_tecla(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        interfaz.avanzar_animacion(1);
        assert_eq!(interfaz.fotograma, 2);
    }

    #[test]
    fn retrato_del_principe_conserva_transparencia_colores_y_parpadeo() {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido valido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida valida");
        let mut interfaz = Interfaz::nueva(partida).expect("interfaz valida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos validos");
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("backend valido");

        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("primer fotograma valido");
        let cerrado = &terminal.backend().buffer()[(14, 15)];
        assert_eq!(cerrado.symbol(), "▀");
        assert_eq!(cerrado.fg, Color::LightMagenta);
        assert_eq!(cerrado.bg, Color::Gray);
        let transparente = terminal.backend().buffer()[(0, 8)].symbol();
        assert!(
            ['.', ':', '^', ',', '~', '*', ';'].contains(&transparente.chars().next().unwrap()),
            "una celda transparente debe conservar el mapa, pero contiene {transparente:?}"
        );

        interfaz.avanzar_animacion(1);
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("segundo fotograma valido");
        let abierto = &terminal.backend().buffer()[(14, 15)];
        assert_eq!(abierto.symbol(), "o");
        assert_eq!(abierto.fg, Color::Cyan);
        assert_eq!(abierto.bg, Color::Gray);
    }
}
