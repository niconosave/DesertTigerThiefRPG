use crate::recursos::RecursosVisuales;
use desert_tiger_app::{
    CoordenadaMapa, VisibilidadTile, VistaDecision, VistaMundo, VistaRutaPropuesta,
};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
};

#[derive(Default)]
pub(crate) struct EstadoPlanificacion {
    pub foco: usize,
    pub eligiendo: bool,
    pub desplazamiento: std::cell::Cell<(isize, isize)>,
    pub tamano: std::cell::Cell<Option<(u16, u16)>>,
}

impl EstadoPlanificacion {
    pub fn ruta<'a>(&self, decision: &'a VistaDecision) -> Option<&'a VistaRutaPropuesta> {
        if !self.eligiendo {
            return None;
        }
        decision.opciones.get(self.foco)?.ruta.as_ref()
    }
    pub fn objetivo(
        &self,
        decision: &VistaDecision,
        mundo: &VistaMundo,
        area: Rect,
    ) -> CoordenadaMapa {
        let centros: Vec<_> = self
            .ruta(decision)
            .into_iter()
            .flat_map(|r| &r.lugares)
            .filter_map(|id| {
                mundo
                    .lugares
                    .iter()
                    .find(|l| l.id == *id)
                    .map(|l| l.coordenada_central)
            })
            .collect();
        let centro = if centros.is_empty() {
            mundo
                .lugares
                .iter()
                .find(|l| l.id == mundo.lugar_jugador)
                .map(|l| l.coordenada_central)
                .unwrap_or(CoordenadaMapa { x: 0, y: 0 })
        } else {
            let (mut min, mut max) = (centros[0], centros[0]);
            for c in centros {
                min.x = min.x.min(c.x);
                min.y = min.y.min(c.y);
                max.x = max.x.max(c.x);
                max.y = max.y.max(c.y);
            }
            CoordenadaMapa {
                x: min.x + (max.x - min.x) / 2,
                y: min.y + (max.y - min.y) / 2,
            }
        };
        if self.tamano.replace(Some((area.width, area.height))) != Some((area.width, area.height)) {
            self.desplazamiento.set((0, 0));
        }
        let desplazamiento = self.desplazamiento.get();
        CoordenadaMapa {
            x: centro
                .x
                .saturating_add_signed(desplazamiento.0)
                .min(mundo.mapa.ancho.saturating_sub(1)),
            y: centro
                .y
                .saturating_add_signed(desplazamiento.1)
                .min(mundo.mapa.alto.saturating_sub(1)),
        }
    }
}

pub(crate) fn detalle(decision: &VistaDecision, foco: usize) -> String {
    format!(
        "{}\n\n{}\n{}",
        decision.texto,
        decision.opciones.get(foco).map_or("", |o| o.texto.as_str()),
        decision
            .opciones
            .get(foco)
            .and_then(|o| o.detalle.as_deref())
            .unwrap_or("")
    )
}

pub(crate) fn color_punto(
    vision: VisibilidadTile,
    frente: Color,
    recursos: &RecursosVisuales,
) -> Color {
    if vision != VisibilidadTile::Visible {
        return Color::Yellow;
    }
    match recursos.paleta.para_iluminar(frente) {
        Some([r, g, b]) if r != g || g != b => Color::Rgb(255 - r, 255 - g, 255 - b),
        _ => Color::Yellow,
    }
}

pub(crate) fn dibujar_ruta(
    frame: &mut Frame<'_>,
    mundo: &VistaMundo,
    ruta: &VistaRutaPropuesta,
    recursos: &RecursosVisuales,
    paso: usize,
    proyectar: impl Fn(CoordenadaMapa) -> Option<(u16, u16)>,
) {
    for par in ruta.lugares.windows(2) {
        let centros: Vec<_> = par
            .iter()
            .filter_map(|id| {
                mundo
                    .lugares
                    .iter()
                    .find(|l| l.id == *id)
                    .map(|l| l.coordenada_central)
            })
            .collect();
        if centros.len() != 2 {
            continue;
        }
        crate::trazado::recorrer_linea(centros[0], centros[1], |c| {
            let Some(posicion) = proyectar(c) else {
                return;
            };
            let vision = mundo.mapa.visibilidad_en(c);
            let estilo = match vision {
                VisibilidadTile::Visible => {
                    let Some(clave) =
                        c.y.checked_mul(mundo.mapa.ancho)
                            .and_then(|i| i.checked_add(c.x))
                            .and_then(|i| mundo.mapa.celdas.get(i))
                    else {
                        return;
                    };
                    recursos.tile(clave, c.x, c.y, paso).estilo
                }
                VisibilidadTile::Recordado => recursos.estilo_recordado(),
                VisibilidadTile::Desconocido => recursos.ui.pantalla,
            };
            let color = color_punto(vision, estilo.fg.unwrap_or(Color::Reset), recursos);
            if let Some(celda) = frame.buffer_mut().cell_mut(posicion) {
                let fondo = estilo
                    .bg
                    .or(recursos.ui.pantalla.bg)
                    .unwrap_or(Color::Reset);
                celda.set_char('.').set_style(
                    Style::default()
                        .fg(color)
                        .bg(fondo)
                        .remove_modifier(Modifier::all()),
                );
            }
        });
    }
}

/// Los números se pintan después de rótulos y @ para identificar también el origen.
pub(crate) fn dibujar_numeros(
    frame: &mut Frame<'_>,
    mundo: &VistaMundo,
    ruta: &VistaRutaPropuesta,
    recursos: &RecursosVisuales,
    paso: usize,
    proyectar: impl Fn(CoordenadaMapa) -> Option<(u16, u16)>,
) {
    for (indice, id) in ruta.lugares.iter().enumerate() {
        let Some(lugar) = mundo.lugares.iter().find(|l| l.id == *id) else {
            continue;
        };
        let centro = lugar.coordenada_central;
        let vision = mundo.mapa.visibilidad_en(centro);
        let frente = if vision == VisibilidadTile::Visible {
            centro
                .y
                .checked_mul(mundo.mapa.ancho)
                .and_then(|i| i.checked_add(centro.x))
                .and_then(|i| mundo.mapa.celdas.get(i))
                .and_then(|c| recursos.tile(c, centro.x, centro.y, paso).estilo.fg)
                .unwrap_or(Color::Reset)
        } else {
            Color::Reset
        };
        let fondo = color_punto(vision, frente, recursos);
        let texto = match recursos.paleta.para_iluminar(fondo) {
            Some([r, g, b])
                if u32::from(r) * 299 + u32::from(g) * 587 + u32::from(b) * 114 < 128_000 =>
            {
                Color::Rgb(255, 255, 255)
            }
            _ => Color::Rgb(0, 0, 0),
        };
        let estilo = Style::default()
            .fg(texto)
            .bg(fondo)
            .remove_modifier(Modifier::all());
        for (columna, digito) in (indice + 1).to_string().chars().enumerate() {
            let Some(x) = centro.x.checked_add(columna) else {
                break;
            };
            if let Some(posicion) = proyectar(CoordenadaMapa { x, y: centro.y })
                && let Some(celda) = frame.buffer_mut().cell_mut(posicion)
            {
                celda.set_char(digito).set_style(estilo);
            }
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn colores_complementarios_respetan_niebla_grises_y_paleta_desconocida() {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal");
        let mut recursos = RecursosVisuales::cargar(&raiz).unwrap();
        assert_eq!(
            color_punto(VisibilidadTile::Visible, Color::Rgb(10, 80, 200), &recursos),
            Color::Rgb(245, 175, 55)
        );
        for vision in [VisibilidadTile::Desconocido, VisibilidadTile::Recordado] {
            assert_eq!(
                color_punto(vision, Color::Rgb(10, 80, 200), &recursos),
                Color::Yellow
            );
        }
        assert_eq!(
            color_punto(VisibilidadTile::Visible, Color::Rgb(90, 90, 90), &recursos),
            Color::Yellow
        );
        assert_eq!(
            color_punto(VisibilidadTile::Visible, Color::Reset, &recursos),
            Color::Yellow
        );
        recursos.paleta = crate::colores::PaletaColores::default();
        assert_eq!(
            color_punto(VisibilidadTile::Visible, Color::Blue, &recursos),
            Color::Yellow
        );
        recursos.paleta = crate::colores::PaletaColores::fija();
        assert_eq!(
            color_punto(VisibilidadTile::Visible, Color::Blue, &recursos),
            Color::Rgb(255, 255, 127)
        );
    }

    #[test]
    fn puntos_desconocidos_no_revelan_terreno_y_se_recortan_al_viewport() {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = desert_tiger_app::cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida = desert_tiger_app::Partida::nueva(
            &contenido,
            desert_tiger_app::ConfiguracionPartida { semilla: Some(0) },
        )
        .unwrap();
        let mut mundo = partida.mundo_visible();
        mundo.mapa.visibilidad.fill(VisibilidadTile::Desconocido);
        mundo.lugares[0].coordenada_central = CoordenadaMapa { x: 2, y: 2 };
        mundo.lugares[1].coordenada_central = CoordenadaMapa { x: 18, y: 2 };
        let ruta = VistaRutaPropuesta {
            proponentes: vec![],
            lugares: vec![mundo.lugares[0].id, mundo.lugares[1].id],
            tiempo_previsto: String::new(),
            respaldos: vec![],
        };
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let antes = mundo.clone();
        let render = |m: &VistaMundo| {
            let mut terminal = Terminal::new(TestBackend::new(20, 6)).unwrap();
            terminal
                .draw(|f| {
                    dibujar_ruta(f, m, &ruta, &recursos, 0, |c| {
                        (c.x < 10 && c.y < 5).then_some((c.x as u16, c.y as u16))
                    })
                })
                .unwrap();
            terminal.backend().buffer().clone()
        };
        let buffer = render(&mundo);
        for x in 2..10 {
            assert_eq!(buffer[(x, 2)].symbol(), ".");
            assert_eq!(buffer[(x, 2)].fg, Color::Yellow);
        }
        assert!((10..20).all(|x| buffer[(x, 2)].symbol() == " "));
        assert_eq!(mundo, antes);
        let otra = mundo.mapa.celdas[100].clone();
        mundo.mapa.celdas.fill(otra);
        assert_eq!(
            render(&mundo),
            buffer,
            "el contenido oculto no determina el trazo ni el color"
        );
    }
    #[test]
    fn centros_numerados_incluyen_origen_dos_cifras_y_recorte() {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = desert_tiger_app::cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida = desert_tiger_app::Partida::nueva(
            &contenido,
            desert_tiger_app::ConfiguracionPartida { semilla: Some(0) },
        )
        .unwrap();
        let mut mundo = partida.mundo_visible();
        mundo.mapa.visibilidad.fill(VisibilidadTile::Desconocido);
        let lugares = mundo
            .lugares
            .iter_mut()
            .take(12)
            .enumerate()
            .map(|(i, l)| {
                l.coordenada_central = CoordenadaMapa { x: 2 + 3 * i, y: 2 };
                l.id
            })
            .collect();
        let ruta = VistaRutaPropuesta {
            lugares,
            proponentes: vec![],
            tiempo_previsto: String::new(),
            respaldos: vec![],
        };
        let recursos = RecursosVisuales::cargar(&raiz.join("assets/terminal")).unwrap();
        let antes = mundo.clone();
        let mut terminal = Terminal::new(TestBackend::new(40, 6)).unwrap();
        terminal
            .draw(|f| {
                dibujar_numeros(f, &mundo, &ruta, &recursos, 0, |c| {
                    (c.x < 36 && c.y < 6).then_some((c.x as u16, c.y as u16))
                })
            })
            .unwrap();
        let buffer = terminal.backend().buffer();
        for n in 1..=12 {
            for (i, digito) in n.to_string().chars().enumerate() {
                let x = 2 + 3 * (n - 1) + i;
                if x >= 36 {
                    continue;
                }
                let celda = &buffer[(x as u16, 2)];
                assert_eq!(celda.symbol(), digito.to_string());
                assert_eq!(celda.bg, Color::Yellow);
                assert_eq!(celda.fg, Color::Rgb(0, 0, 0));
            }
        }
        assert_eq!(
            buffer[(36, 2)].symbol(),
            " ",
            "la segunda cifra de 12 queda fuera del viewport"
        );
        assert_eq!(mundo, antes);
    }
}
