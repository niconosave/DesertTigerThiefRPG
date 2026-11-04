use crate::trazado::recorrer_linea;
use std::collections::BTreeMap;

use desert_tiger_app::{CoordenadaMapa, VistaMundo};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::recursos::RecursosVisuales;

#[derive(Default)]
pub(crate) struct InspectorCoordenadas {
    pub activo: bool,
    pub puntero: Option<(u16, u16)>,
    pub fijada: Option<CoordenadaMapa>,
}

impl InspectorCoordenadas {
    pub fn alternar(&mut self) {
        self.activo = !self.activo;
        self.puntero = None;
        self.fijada = None;
    }
}

const ARCOIRIS: [Color; 6] = [
    Color::LightRed,
    Color::LightYellow,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightBlue,
    Color::LightMagenta,
];

pub(crate) fn dibujar_menu(
    frame: &mut Frame<'_>,
    area: Rect,
    conexiones: bool,
    coordenadas: bool,
    mapa_completo: bool,
    foco: usize,
    recursos: &RecursosVisuales,
) {
    let ancho = 68.min(area.width);
    let alto = 10.min(area.height);
    let ventana = Rect::new(
        area.x + (area.width - ancho) / 2,
        area.y + (area.height - alto) / 2,
        ancho,
        alto,
    );
    frame.render_widget(Clear, ventana);
    let bloque = Block::default()
        .borders(Borders::ALL)
        .title(" Desarrollo ")
        .style(recursos.ui.marco);
    let interior = bloque.inner(ventana);
    frame.render_widget(bloque, ventana);
    let marca = if conexiones { "x" } else { " " };
    let marca_coordenadas = if coordenadas { "x" } else { " " };
    let marca_mapa = if mapa_completo { "x" } else { " " };
    let cursor = |opcion| if foco == opcion { ">" } else { " " };
    let texto = format!(
        "{} [{marca}] Mostrar conexiones de lugares [C]\n{} [{marca_coordenadas}] Inspeccionar coordenadas [I]\n{} [{marca_mapa}] Mostrar mapa completo [M]\n\n↑/↓: elegir | Enter / Espacio: alternar\nEsc / D: volver al mapa",
        cursor(0),
        cursor(1),
        cursor(2),
    );
    frame.render_widget(
        Paragraph::new(texto).style(recursos.ui.texto_normal),
        interior,
    );
}

pub(crate) fn dibujar_conexiones(
    frame: &mut Frame<'_>,
    mundo: &VistaMundo,
    paso_animacion: usize,
    recordado: Style,
    proyectar: impl Fn(CoordenadaMapa) -> Option<(u16, u16)>,
) {
    let centros: BTreeMap<_, _> = mundo
        .lugares
        .iter()
        .filter(|l| !l.es_region())
        .map(|l| (l.id, l.coordenada_central))
        .collect();
    let estilo = Style::default()
        .fg(ARCOIRIS[paso_animacion % ARCOIRIS.len()])
        .remove_modifier(Modifier::BOLD | Modifier::DIM);
    for lugar in mundo.lugares.iter().filter(|l| !l.es_region()) {
        // El índice de conexiones es simétrico; cada par se dibuja una sola vez.
        for destino in lugar.conectados.iter().filter(|&&id| lugar.id < id) {
            if let Some(&centro) = centros.get(destino) {
                recorrer_linea(lugar.coordenada_central, centro, |coordenada| {
                    let estilo = match mundo.mapa.visibilidad_en(coordenada) {
                        desert_tiger_app::VisibilidadTile::Desconocido => return,
                        desert_tiger_app::VisibilidadTile::Recordado => recordado,
                        desert_tiger_app::VisibilidadTile::Visible => estilo,
                    };
                    if let Some(posicion) = proyectar(coordenada)
                        && let Some(celda) = frame.buffer_mut().cell_mut(posicion)
                    {
                        celda.set_char('.').set_style(estilo);
                    }
                });
            }
        }
    }
    // Los centros se dibujan al final para que otra conexión no los tape con puntos.
    for lugar in mundo.lugares.iter().filter(|l| !l.es_region()) {
        let estilo = match mundo.mapa.visibilidad_en(lugar.coordenada_central) {
            desert_tiger_app::VisibilidadTile::Desconocido => continue,
            desert_tiger_app::VisibilidadTile::Recordado => recordado,
            desert_tiger_app::VisibilidadTile::Visible => estilo,
        };
        if lugar.conectados.iter().any(|id| centros.contains_key(id))
            && let Some(posicion) = proyectar(lugar.coordenada_central)
            && let Some(celda) = frame.buffer_mut().cell_mut(posicion)
        {
            celda.set_char('O').set_style(estilo);
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn conexiones_recordadas_son_grises_y_las_desconocidas_no_se_dibujan() {
        use desert_tiger_app::VisibilidadTile::{Desconocido, Recordado, Visible};
        let c = desert_tiger_app::cargar_contenido(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let p = desert_tiger_app::Partida::nueva(
            &c,
            desert_tiger_app::ConfiguracionPartida { semilla: Some(0) },
        )
        .unwrap();
        let mut m = p.mundo_visible();
        m.mapa.ancho = 5;
        m.mapa.alto = 1;
        m.mapa.visibilidad = vec![Visible, Recordado, Desconocido, Visible, Recordado];
        for l in &mut m.lugares {
            l.conectados.clear();
        }
        m.lugares[0].coordenada_central = CoordenadaMapa { x: 0, y: 0 };
        m.lugares[1].coordenada_central = CoordenadaMapa { x: 4, y: 0 };
        m.lugares[0].conectados = vec![m.lugares[1].id];
        m.lugares[1].conectados = vec![m.lugares[0].id];
        let pintar = |paso| {
            let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(5, 1)).unwrap();
            t.draw(|f| {
                dibujar_conexiones(f, &m, paso, Style::default().fg(Color::DarkGray), |c| {
                    Some((c.x as u16, c.y as u16))
                })
            })
            .unwrap();
            t.backend().buffer().clone()
        };
        let a = pintar(0);
        let b = pintar(1);
        assert_eq!(a[(1, 0)].symbol(), ".");
        assert_eq!(a[(1, 0)].fg, Color::DarkGray);
        assert_eq!(a[(1, 0)], b[(1, 0)]);
        assert_eq!(a[(2, 0)].symbol(), " ");
        assert_eq!(a[(4, 0)].symbol(), "O");
        assert_eq!(a[(4, 0)], b[(4, 0)]);
        assert_ne!(a[(3, 0)].fg, b[(3, 0)].fg);
    }

    #[test]
    fn conexiones_cruzan_la_camara_sin_pintar_fuera_ni_incluir_regiones() {
        let raiz = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let contenido = desert_tiger_app::cargar_contenido(&raiz.join("content/base")).unwrap();
        let partida = desert_tiger_app::Partida::nueva(
            &contenido,
            desert_tiger_app::ConfiguracionPartida { semilla: Some(0) },
        )
        .unwrap();
        let mut mundo = partida.mundo_visible();
        mundo
            .mapa
            .visibilidad
            .fill(desert_tiger_app::VisibilidadTile::Visible);
        for lugar in &mut mundo.lugares {
            lugar.conectados.clear();
        }
        mundo.lugares[0].coordenada_central = CoordenadaMapa { x: 0, y: 23 };
        mundo.lugares[1].coordenada_central = CoordenadaMapa { x: 30, y: 23 };
        mundo.lugares[2].coordenada_central = CoordenadaMapa { x: 15, y: 23 };
        mundo.lugares[26].coordenada_central = CoordenadaMapa { x: 15, y: 20 };
        mundo.lugares[0].conectados = vec![mundo.lugares[1].id];
        mundo.lugares[1].conectados = vec![
            mundo.lugares[0].id,
            mundo.lugares[2].id,
            mundo.lugares[26].id,
        ];
        mundo.lugares[2].conectados = vec![mundo.lugares[1].id];
        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(30, 14)).unwrap();
        for paso in 0..=ARCOIRIS.len() {
            terminal
                .draw(|frame| {
                    for celda in &mut frame.buffer_mut().content {
                        celda.set_char('#').set_style(
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        );
                    }
                    dibujar_conexiones(
                        frame,
                        &mundo,
                        paso,
                        Style::default().fg(Color::DarkGray),
                        |p| {
                            let x = p.x.checked_sub(10)?;
                            let y = p.y.checked_sub(20)?;
                            (x < 10 && y < 7).then_some((x as u16 + 5, y as u16 + 3))
                        },
                    );
                })
                .unwrap();
            let buffer = terminal.backend().buffer();
            for y in 0..14 {
                for x in 0..30 {
                    let punto = y == 6 && (5..15).contains(&x);
                    let esperado = if punto && x == 10 {
                        "O"
                    } else if punto {
                        "."
                    } else {
                        "#"
                    };
                    assert_eq!(buffer[(x, y)].symbol(), esperado);
                    if punto {
                        assert_eq!(buffer[(x, y)].fg, ARCOIRIS[paso % ARCOIRIS.len()]);
                        assert!(
                            !buffer[(x, y)]
                                .modifier
                                .intersects(Modifier::BOLD | Modifier::DIM)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn lineas_continuas_incluyen_extremos_en_todas_las_direcciones() {
        let origen = CoordenadaMapa { x: 8, y: 8 };
        for (x, y) in [
            (0, 0),
            (0, 8),
            (0, 10),
            (8, 0),
            (8, 8),
            (8, 16),
            (10, 0),
            (16, 8),
            (16, 16),
        ] {
            let destino = CoordenadaMapa { x, y };
            let mut puntos = Vec::new();
            recorrer_linea(origen, destino, |p| puntos.push(p));
            assert_eq!(puntos.first(), Some(&origen));
            assert_eq!(puntos.last(), Some(&destino));
            assert_eq!(
                puntos.len(),
                origen.x.abs_diff(x).max(origen.y.abs_diff(y)) + 1
            );
            assert!(puntos.windows(2).all(|p| p[0].x.abs_diff(p[1].x) <= 1
                && p[0].y.abs_diff(p[1].y) <= 1
                && p[0] != p[1]));
        }
    }
}
