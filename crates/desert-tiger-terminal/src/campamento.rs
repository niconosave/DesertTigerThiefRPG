use std::cell::Cell;

use crossterm::event::KeyCode;
use desert_tiger_app::{
    EntradaFicha, ExtremoRelacion, IntencionFrontend, Partida, VistaCampamento, VistaFicha,
    VistaMundo,
};
use ratatui::{Frame, layout::Rect, widgets::Paragraph};
use unicode_width::UnicodeWidthChar;

use crate::{interfaz::dibujar_retratos, recursos::RecursosVisuales};

pub(crate) struct EstadoCampamento {
    vista: VistaCampamento,
    horizontal: usize,
    ficha: Option<VistaFicha>,
    entrada: Option<usize>,
    pagina: Cell<usize>,
    dimensiones_detalle: Cell<(u16, u16)>,
    error: Option<String>,
    pub(crate) musica: Option<crate::musica_campamento::NotasMusicales>,
}

impl EstadoCampamento {
    pub(crate) fn nuevo(vista: VistaCampamento, partida: &Partida) -> Self {
        let horizontal = vista.integrantes.len().saturating_sub(1);
        let mut estado = Self {
            vista,
            horizontal,
            ficha: None,
            entrada: None,
            pagina: Cell::new(0),
            dimensiones_detalle: Cell::new((53, 6)),
            error: None,
            musica: None,
        };
        estado.seleccionar(horizontal, partida);
        estado
    }

    fn saliendo(&self) -> bool {
        self.horizontal == self.vista.integrantes.len()
    }

    fn abrir(&mut self, elemento: ExtremoRelacion, partida: &Partida) -> bool {
        match partida.consultar_ficha(self.vista.id, elemento) {
            Ok(ficha) => {
                self.entrada = (!ficha.entradas.is_empty()).then_some(0);
                self.ficha = Some(ficha);
                self.pagina.set(0);
                self.error = None;
                true
            }
            Err(error) => {
                self.error = Some(error.to_string());
                false
            }
        }
    }

    fn seleccionar(&mut self, indice: usize, partida: &Partida) {
        if indice == self.vista.integrantes.len() {
            self.horizontal = indice;
            self.entrada = None;
            self.error = None;
        } else if let Some(actor) = self.vista.integrantes.get(indice).map(|a| a.actor)
            && self.abrir(ExtremoRelacion::Actor(actor), partida)
        {
            self.horizontal = indice;
        }
    }

    pub(crate) fn manejar(
        &mut self,
        tecla: KeyCode,
        partida: &Partida,
    ) -> Option<IntencionFrontend> {
        match tecla {
            KeyCode::Left => {
                let n = self.vista.integrantes.len() + 1;
                self.seleccionar((self.horizontal + n - 1) % n, partida);
            }
            KeyCode::Right => self.seleccionar(
                (self.horizontal + 1) % (self.vista.integrantes.len() + 1),
                partida,
            ),
            KeyCode::Esc => self.seleccionar(self.vista.integrantes.len(), partida),
            KeyCode::Enter if self.saliendo() => {
                return Some(IntencionFrontend::AbandonarCampamento {
                    campamento: self.vista.id,
                });
            }
            KeyCode::Enter => {
                let destino = self
                    .ficha
                    .as_ref()
                    .and_then(|f| self.entrada.and_then(|i| f.entradas.get(i)))
                    .and_then(|e| match e {
                        EntradaFicha::Relacion(r) => Some(r.referencia.elemento),
                        EntradaFicha::Caracteristica { .. } => None,
                    });
                if let Some(destino) = destino {
                    self.abrir(destino, partida);
                }
            }
            KeyCode::Up | KeyCode::Down if !self.saliendo() => {
                if let (Some(i), Some(f)) = (self.entrada, &self.ficha) {
                    self.entrada = Some(if tecla == KeyCode::Up {
                        i.saturating_sub(1)
                    } else {
                        (i + 1).min(f.entradas.len().saturating_sub(1))
                    });
                    self.pagina.set(0);
                }
            }
            KeyCode::PageUp if !self.saliendo() => {
                self.pagina.set(self.pagina.get().saturating_sub(1))
            }
            KeyCode::PageDown if !self.saliendo() => self
                .pagina
                .set((self.pagina.get() + 1).min(self.paginas().len().saturating_sub(1))),
            _ => {}
        }
        None
    }

    fn detalle(&self) -> String {
        let Some(ficha) = &self.ficha else {
            return "No se pudo abrir la ficha. Esc permite enfocar Salir.".into();
        };
        let entrada = self.entrada.and_then(|i| ficha.entradas.get(i));
        let mut texto = match entrada {
            Some(EntradaFicha::Caracteristica {
                clave,
                nombre,
                descripcion,
            }) => format!(
                "{nombre} ({clave})\n{}",
                descripcion
                    .as_deref()
                    .unwrap_or("Sin descripción disponible")
            ),
            Some(EntradaFicha::Relacion(r)) => format!(
                "{}\n{}\n{}\nEnter: abrir ficha",
                r.etiqueta, r.referencia.nombre, r.referencia.texto
            ),
            None => "Sin características ni relaciones".into(),
        };
        if let Some(EntradaFicha::Relacion(r)) = entrada
            && let Some(explicacion) = &r.explicacion
        {
            texto.push_str(&format!("\n{explicacion}"));
        }
        texto.push_str(&format!("\n\nFicha: {}", ficha.nombre));
        if let Some(ubicacion) = &ficha.ubicacion {
            texto.push_str(&format!(
                "\nUbicación: {}",
                match ubicacion {
                    desert_tiger_app::VistaUbicacionActor::EnLugar { nombre, .. } =>
                        nombre.as_str(),
                    desert_tiger_app::VistaUbicacionActor::FueraDelEscenario =>
                        "Fuera de la región",
                }
            ));
        }
        if let Some(bio) = &ficha.biografia {
            let titulo = match ficha.elemento {
                ExtremoRelacion::Actor(_) => "Biografía",
                ExtremoRelacion::Lugar(_) | ExtremoRelacion::Concepto(_) => "Descripción",
            };
            texto.push_str(&format!(
                "\n\n{titulo} de {}\n{}",
                ficha.nombre,
                if bio.trim().is_empty() {
                    "Sin biografía disponible"
                } else {
                    bio
                }
            ));
        }
        texto
    }

    fn paginas(&self) -> Vec<String> {
        let (ancho, alto) = self.dimensiones_detalle.get();
        envolver(&self.detalle(), usize::from(ancho.max(1)))
            .chunks(usize::from(alto.max(1)))
            .map(|p| p.join("\n"))
            .collect()
    }

    pub(crate) fn contexto(&self) -> String {
        if let Some(error) = &self.error {
            return format!("Error: {error}");
        }
        if self.saliendo() {
            return "Abandonar el campamento hará pasar un día. Enter confirma.".into();
        }
        format!(
            "Ficha: {} | Selector: {}",
            self.ficha
                .as_ref()
                .map(|f| f.nombre.as_str())
                .unwrap_or("Sin ficha"),
            self.vista
                .integrantes
                .get(self.horizontal)
                .map(|a| a.nombre.as_str())
                .unwrap_or("Salir")
        )
    }

    pub(crate) fn dibujar(
        &self,
        frame: &mut Frame<'_>,
        area: Rect,
        mundo: &VistaMundo,
        recursos: &RecursosVisuales,
        reloj: &crate::animacion::RelojVisual,
    ) {
        let alto = mundo
            .companeros
            .iter()
            .chain(std::iter::once(&mundo.jugador))
            .map(|a| a.retrato.base().alto())
            .chain(std::iter::once(recursos.alto_fogata()))
            .max()
            .unwrap_or(0);
        let alto_arte = u16::try_from(alto)
            .unwrap_or(u16::MAX)
            .min(area.height.saturating_sub(6));
        let ficha_alto = area.height - alto_arte;
        let arte = Rect::new(area.x, area.y + ficha_alto, area.width, alto_arte);
        let fin_retratos = dibujar_retratos(
            frame,
            area,
            arte,
            mundo,
            recursos,
            reloj.paso_250_ms(),
            self.musica.is_some(),
        );
        if let Some(musica) = self.musica {
            crate::musica_campamento::dibujar_notas(
                frame,
                area,
                arte,
                mundo,
                musica,
                recursos.ui.destacado,
            );
        }
        let fogata_x = fin_retratos.saturating_add(1);
        let fogata_y = i64::from(area.bottom())
            .saturating_sub(i64::try_from(recursos.alto_fogata()).unwrap_or(i64::MAX));
        if let Some(configuracion) = &recursos.efecto_fogata {
            crate::fogata::dibujar(
                frame.buffer_mut(),
                arte,
                (fogata_x, fogata_y),
                reloj.instante_ms(),
                configuracion,
                &recursos.paleta,
            );
        } else {
            for (y, fila) in recursos.fogata.iter().enumerate() {
                for (x, c) in fila.chars().enumerate() {
                    if c == ' ' {
                        continue;
                    }
                    let px = fogata_x.saturating_add(i64::try_from(x).unwrap_or(i64::MAX));
                    let py = fogata_y.saturating_add(i64::try_from(y).unwrap_or(i64::MAX));
                    if px >= i64::from(arte.x)
                        && px < i64::from(arte.right())
                        && py >= i64::from(arte.y)
                        && py < i64::from(arte.bottom())
                        && let (Ok(x), Ok(y)) = (u16::try_from(px), u16::try_from(py))
                        && let Some(celda) = frame.buffer_mut().cell_mut((x, y))
                    {
                        celda.set_char(c).set_style(recursos.estilo_fogata);
                    }
                }
            }
        }
        if self.saliendo() {
            frame.render_widget(
                Paragraph::new("Abandonar campamento — Enter confirma (1 día)")
                    .style(recursos.ui.foco),
                Rect::new(area.x, area.y, area.width, 1),
            );
            return;
        }
        let titulo = self
            .ficha
            .as_ref()
            .map(|f| {
                format!(
                    "{}: {}",
                    match f.elemento {
                        ExtremoRelacion::Actor(_) => "Actor",
                        ExtremoRelacion::Lugar(_) => "Lugar",
                        ExtremoRelacion::Concepto(_) => "Concepto",
                    },
                    f.nombre
                )
            })
            .unwrap_or_else(|| "Campamento".into());
        frame.render_widget(
            Paragraph::new(titulo).style(recursos.ui.titulo),
            Rect::new(area.x, area.y, area.width, 1),
        );
        let ancho_lista = (area.width / 3).clamp(24, 36);
        let cuerpo_alto = ficha_alto.saturating_sub(2).max(1);
        let detalle = Rect::new(
            area.x + ancho_lista + 1,
            area.y + 1,
            area.width - ancho_lista - 1,
            cuerpo_alto,
        );
        self.dimensiones_detalle
            .set((detalle.width, detalle.height));
        let paginas = self.paginas();
        let pagina = self.pagina.get().min(paginas.len().saturating_sub(1));
        self.pagina.set(pagina);
        if let Some(texto) = paginas.get(pagina) {
            frame.render_widget(
                Paragraph::new(texto.as_str()).style(recursos.ui.texto_normal),
                detalle,
            );
        }
        let total = self.ficha.as_ref().map(|f| f.entradas.len()).unwrap_or(0);
        let primero = self
            .entrada
            .unwrap_or(0)
            .saturating_sub(usize::from(cuerpo_alto) - 1);
        if let Some(ficha) = &self.ficha {
            for (fila, entrada) in ficha
                .entradas
                .iter()
                .enumerate()
                .skip(primero)
                .take(usize::from(cuerpo_alto))
            {
                let seleccionado = self.entrada == Some(fila);
                let (tipo, nombre) = match entrada {
                    EntradaFicha::Caracteristica { nombre, .. } => ("C", nombre),
                    EntradaFicha::Relacion(r) => ("R", &r.etiqueta),
                };
                frame.render_widget(
                    Paragraph::new(format!(
                        "{} [{tipo}] {nombre}",
                        if seleccionado { ">" } else { " " }
                    ))
                    .style(if seleccionado {
                        recursos.ui.foco
                    } else {
                        recursos.ui.texto_normal
                    }),
                    Rect::new(area.x, area.y + 1 + (fila - primero) as u16, ancho_lista, 1),
                );
            }
        }
        frame.render_widget(
            Paragraph::new(format!(
                "Información {}/{} | Texto {}/{} — PgUp/PgDn para leer",
                self.entrada.map(|i| i + 1).unwrap_or(0),
                total,
                pagina + 1,
                paginas.len()
            ))
            .style(recursos.ui.secundario),
            Rect::new(area.x, area.y + ficha_alto - 1, area.width, 1),
        );
    }
}

fn envolver(texto: &str, ancho: usize) -> Vec<String> {
    let mut filas = Vec::new();
    for parrafo in texto.split('\n') {
        let mut fila = String::new();
        let mut usado = 0;
        for c in parrafo.chars() {
            let n = UnicodeWidthChar::width(c).unwrap_or(0);
            if usado + n > ancho && !fila.is_empty() {
                filas.push(std::mem::take(&mut fila));
                usado = 0;
            }
            fila.push(c);
            usado += n;
        }
        filas.push(fila);
    }
    filas
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_app::{ConfiguracionPartida, IdActor, IdLugar, VistaSesion, cargar_contenido};
    use ratatui::{Terminal, backend::TestBackend};
    use std::path::PathBuf;

    fn escenario() -> (Partida, EstadoCampamento, RecursosVisuales) {
        let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut partida =
            Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let VistaSesion::Acampando(vista) = partida
            .procesar(IntencionFrontend::ComenzarCampamento)
            .unwrap()
        else {
            panic!()
        };
        let estado = EstadoCampamento::nuevo(vista, &partida);
        (
            partida,
            estado,
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap(),
        )
    }

    #[test]
    fn la_referencia_de_deseo_abre_y_dibuja_un_concepto() {
        let (p, mut e, recursos) = escenario();
        e.entrada = e.ficha.as_ref().unwrap().entradas.iter().position(|entrada| {
            matches!(entrada, EntradaFicha::Relacion(r) if r.etiqueta.starts_with("Desea:"))
        });
        assert!(e.entrada.is_some());
        e.manejar(KeyCode::Enter, &p);
        let ficha = e.ficha.as_ref().unwrap();
        assert!(matches!(ficha.elemento, ExtremoRelacion::Concepto(_)));
        assert_eq!(ficha.nombre, "Recuperar el trono del Drâa");
        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 100, 28),
                    &p.mundo_visible(),
                    &recursos,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        let texto: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(texto.contains("Recuperar el trono del Drâa"));
        assert!(texto.contains("Concepto"));
    }

    #[test]
    fn ficha_real_del_rey_navega_por_asesinato_y_hermandad_con_explicacion() {
        let (p, mut e, recursos) = escenario();
        let antes = p.mundo_visible();
        e.entrada = e.ficha.as_ref().unwrap().entradas.iter().position(
            |entrada| matches!(entrada, EntradaFicha::Relacion(r) if r.etiqueta == "Padre: Rey"),
        );
        e.manejar(KeyCode::Enter, &p);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Rey");
        assert!(e.detalle().contains("Fuera de la región"));
        let asesino = e
            .ficha
            .as_ref()
            .unwrap()
            .entradas
            .iter()
            .position(
                |x| matches!(x,EntradaFicha::Relacion(r) if r.etiqueta.starts_with("Matado por:")),
            )
            .unwrap();
        e.entrada = Some(asesino);
        e.manejar(KeyCode::Enter, &p);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Hermano del rey");
        let parentesco=e.ficha.as_ref().unwrap().entradas.iter().position(|x|matches!(x,EntradaFicha::Relacion(r) if matches!(r.vinculo,desert_tiger_app::VinculoFicha::Hermandad{..}))).unwrap();
        e.entrada = Some(parentesco);
        assert!(
            e.detalle()
                .contains("Progenitores en común: Abuelo paterno, Abuela paterna")
        );
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 80, 22),
                    &p.mundo_visible(),
                    &recursos,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        let paginas = e.paginas().len();
        let mut explicacion = false;
        for _ in 0..paginas {
            terminal
                .draw(|f| {
                    e.dibujar(
                        f,
                        Rect::new(0, 0, 80, 22),
                        &p.mundo_visible(),
                        &recursos,
                        &crate::animacion::RelojVisual::default(),
                    )
                })
                .unwrap();
            let texto: String = terminal
                .backend()
                .buffer()
                .content
                .iter()
                .map(|c| c.symbol())
                .collect();
            explicacion |= texto.contains("Progenitores en común");
            e.manejar(KeyCode::PageDown, &p);
        }
        assert!(explicacion);
        e.manejar(KeyCode::Enter, &p);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Rey");
        e.manejar(KeyCode::Left, &p);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Sirviente");
        assert_eq!(p.mundo_visible(), antes);
    }

    #[test]
    fn focos_referencias_externas_ciclos_y_salida_no_alteran_el_grupo() {
        let (p, mut e, _) = escenario();
        let antes = p.mundo_visible();
        assert_eq!(e.horizontal, e.vista.integrantes.len() - 1);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Príncipe Perdido");
        assert_eq!(e.entrada, Some(0));
        e.manejar(KeyCode::Right, &p);
        assert!(e.saliendo());
        e.manejar(KeyCode::Right, &p);
        assert_eq!(e.horizontal, 0);
        for _ in 0..200 {
            e.manejar(KeyCode::Down, &p);
        }
        assert_eq!(
            e.entrada,
            Some(e.ficha.as_ref().unwrap().entradas.len() - 1)
        );
        for _ in 0..200 {
            e.manejar(KeyCode::Up, &p);
        }
        assert_eq!(e.entrada, Some(0));
        // Lugar con Conoce entrante del cazador, un Actor externo sin retrato.
        assert!(e.abrir(
            ExtremoRelacion::Lugar(IdLugar::desde_posicion(2).unwrap()),
            &p
        ));
        let cazador = ExtremoRelacion::Actor(IdActor::desde_posicion(2).unwrap());
        for _ in 0..30 {
            let indice = e
                .ficha
                .as_ref()
                .unwrap()
                .entradas
                .iter()
                .position(
                    |x| matches!(x,EntradaFicha::Relacion(r) if r.referencia.elemento == cazador),
                )
                .unwrap();
            e.entrada = Some(indice);
            e.manejar(KeyCode::Enter, &p);
            assert_eq!(e.ficha.as_ref().unwrap().elemento, cazador);
            let indice = e.ficha.as_ref().unwrap().entradas.iter().position(|x| matches!(x,EntradaFicha::Relacion(r) if r.referencia.elemento == ExtremoRelacion::Lugar(IdLugar::desde_posicion(2).unwrap()))).unwrap();
            e.entrada = Some(indice);
            e.manejar(KeyCode::Enter, &p);
        }
        e.manejar(KeyCode::Right, &p);
        assert_eq!(e.horizontal, 1);
        assert_eq!(e.ficha.as_ref().unwrap().nombre, "Sirviente");
        let ficha = e.ficha.clone();
        let indice = e.entrada;
        let horizontal = e.horizontal;
        assert!(!e.abrir(
            ExtremoRelacion::Actor(IdActor::desde_posicion(999).unwrap()),
            &p
        ));
        assert_eq!(e.ficha, ficha);
        assert_eq!(e.entrada, indice);
        assert_eq!(e.horizontal, horizontal);
        e.manejar(KeyCode::Esc, &p);
        assert!(matches!(
            e.manejar(KeyCode::Enter, &p),
            Some(IntencionFrontend::AbandonarCampamento { .. })
        ));
        assert_eq!(p.mundo_visible(), antes);
    }

    #[test]
    fn render_minimo_pagina_biografia_y_conserva_lista_al_redimensionar() {
        let (p, mut e, mut r) = escenario();
        r.efecto_fogata = None;
        let ficha = e.ficha.as_mut().unwrap();
        ficha.biografia = Some(format!(
            "{}\nFINBIO",
            "Una historia para leer completa. ".repeat(100)
        ));
        ficha.entradas = (0..100)
            .map(|i| EntradaFicha::Caracteristica {
                clave: format!("entrada{i}"),
                nombre: format!("Entrada {i}"),
                descripcion: Some("Una explicación".into()),
            })
            .collect();
        e.entrada = Some(99);
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 80, 22),
                    &p.mundo_visible(),
                    &r,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        assert_eq!(terminal.backend().buffer()[(59, 18)].symbol(), "/");
        let lineas: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(lineas.contains("Entrada 99"));
        let mut fin = false;
        for _ in 0..e.paginas().len() {
            terminal
                .draw(|f| {
                    e.dibujar(
                        f,
                        Rect::new(0, 0, 80, 22),
                        &p.mundo_visible(),
                        &r,
                        &crate::animacion::RelojVisual::default(),
                    )
                })
                .unwrap();
            let texto: String = terminal
                .backend()
                .buffer()
                .content
                .iter()
                .map(|c| c.symbol())
                .collect();
            fin |= texto.contains("FINBIO");
            e.manejar(KeyCode::PageDown, &p);
        }
        assert!(fin, "toda la biografía debe poder leerse");
        terminal.backend_mut().resize(120, 40);
        terminal.resize(Rect::new(0, 0, 120, 40)).unwrap();
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 120, 38),
                    &p.mundo_visible(),
                    &r,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        terminal.backend_mut().resize(80, 24);
        terminal.resize(Rect::new(0, 0, 80, 24)).unwrap();
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 80, 22),
                    &p.mundo_visible(),
                    &r,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        assert_eq!(e.entrada, Some(99));
        assert!(e.pagina.get() < e.paginas().len());
    }

    #[test]
    fn texto_unicode_y_palabras_largas_no_pierden_caracteres() {
        let texto = "áéíóú verylongwordwithoutspaces 海水 texto";
        let filas = envolver(texto, 7);
        assert_eq!(filas.concat(), texto);
        assert!(filas.iter().all(|f| {
            f.chars()
                .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
                .sum::<usize>()
                <= 7
        }));
    }

    #[test]
    fn arte_alto_y_grupo_ancho_se_recortan_sin_tapar_ficha() {
        let (p, e, r) = escenario();
        let mut mundo = p.mundo_visible();
        mundo.companeros.clear();
        let mut terminal = Terminal::new(TestBackend::new(80, 12)).unwrap();
        // Una región más baja que la base fuerza un origen lógico negativo.
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 80, 10),
                    &mundo,
                    &r,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        assert_eq!(terminal.backend().buffer()[(3, 9)].symbol(), "█");
        assert_eq!(terminal.backend().buffer()[(0, 0)].symbol(), "A");
        mundo.companeros = vec![mundo.jugador.clone(); 100];
        terminal
            .draw(|f| {
                e.dibujar(
                    f,
                    Rect::new(0, 0, 80, 10),
                    &mundo,
                    &r,
                    &crate::animacion::RelojVisual::default(),
                )
            })
            .unwrap();
        assert_eq!(terminal.backend().buffer()[(0, 0)].symbol(), "A");
        assert_eq!(terminal.backend().buffer()[(0, 10)].symbol(), " ");
    }
}
