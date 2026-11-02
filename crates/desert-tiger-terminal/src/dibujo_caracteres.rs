//! Composición compartida de gráficos de caracteres validados.
use crate::recursos::{NEGRO_PURO, RecursosVisuales};
use desert_tiger_app::{CeldaRetrato, ColorRetrato, EstiloCeldaRetrato};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
};

pub(crate) fn dibujar_celdas(
    frame: &mut Frame<'_>,
    recorte: Rect,
    x: i64,
    y: i64,
    ancho: usize,
    celdas: &[CeldaRetrato],
    recursos: &RecursosVisuales,
) {
    if ancho == 0 || !celdas.len().is_multiple_of(ancho) {
        return;
    }
    let (Ok(w), Ok(h)) = (i64::try_from(ancho), i64::try_from(celdas.len() / ancho)) else {
        return;
    };
    let left = x.max(i64::from(recorte.x)).max(i64::from(frame.area().x));
    let top = y.max(i64::from(recorte.y)).max(i64::from(frame.area().y));
    let right = x
        .saturating_add(w)
        .min(i64::from(recorte.right()))
        .min(i64::from(frame.area().right()));
    let bottom = y
        .saturating_add(h)
        .min(i64::from(recorte.bottom()))
        .min(i64::from(frame.area().bottom()));
    for py in top..bottom {
        let fila = (py - y) as usize;
        for px in left..right {
            let celda = &celdas[fila * ancho + (px - x) as usize];
            if celda.glifo == ' ' {
                continue;
            }
            let estilo = celda
                .estilo
                .map(estilo_retrato)
                .unwrap_or(recursos.ui.destacado);
            // El fondo ausente se conserva, pero los atributos de la capa inferior no.
            let estilo = estilo.remove_modifier(Modifier::all().difference(estilo.add_modifier));
            if let Some(destino) = frame.buffer_mut().cell_mut((px as u16, py as u16)) {
                destino.set_char(celda.glifo).set_style(estilo);
            }
        }
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

pub(crate) fn color_retrato(color: ColorRetrato) -> Color {
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
    use ratatui::{Terminal, backend::TestBackend};
    use std::path::Path;

    fn recursos() -> RecursosVisuales {
        RecursosVisuales::cargar(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap()
    }

    #[test]
    fn espacio_no_pinta_y_el_fondo_ausente_se_conserva_sin_heredar_atributos() {
        let estilo = |fondo| {
            Some(EstiloCeldaRetrato {
                frente: ColorRetrato::Rojo,
                fondo,
            })
        };
        let celdas = [
            CeldaRetrato {
                glifo: 'x',
                estilo: estilo(None),
            },
            CeldaRetrato {
                glifo: ' ',
                estilo: estilo(Some(ColorRetrato::Negro)),
            },
            CeldaRetrato {
                glifo: 'y',
                estilo: estilo(Some(ColorRetrato::Negro)),
            },
        ];
        let mut t = Terminal::new(TestBackend::new(3, 1)).unwrap();
        t.draw(|f| {
            for c in &mut f.buffer_mut().content {
                c.set_char('?').set_style(
                    Style::default()
                        .fg(Color::Green)
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD | Modifier::DIM),
                );
            }
            dibujar_celdas(f, Rect::new(0, 0, 3, 1), 0, 0, 3, &celdas, &recursos());
        })
        .unwrap();
        let b = t.backend().buffer();
        assert_eq!(b[(0, 0)].symbol(), "x");
        assert_eq!(b[(0, 0)].bg, Color::Blue);
        assert_eq!(b[(0, 0)].fg, Color::Red);
        assert!(b[(0, 0)].modifier.is_empty());
        assert_eq!(b[(1, 0)].symbol(), "?");
        assert_eq!(b[(1, 0)].bg, Color::Blue);
        assert_eq!(b[(1, 0)].modifier, Modifier::BOLD | Modifier::DIM);
        assert_eq!(b[(2, 0)].bg, NEGRO_PURO);
    }

    #[test]
    fn recorta_filas_y_columnas_sin_escribir_fuera_del_area() {
        let celdas: Vec<_> = "ABCDEFGHI"
            .chars()
            .map(|glifo| CeldaRetrato {
                glifo,
                estilo: None,
            })
            .collect();
        let mut t = Terminal::new(TestBackend::new(4, 4)).unwrap();
        t.draw(|f| {
            dibujar_celdas(f, Rect::new(1, 1, 2, 2), 0, 0, 3, &celdas, &recursos());
            dibujar_celdas(
                f,
                Rect::new(0, 0, 4, 4),
                i64::MIN,
                i64::MAX,
                3,
                &celdas,
                &recursos(),
            );
        })
        .unwrap();
        for y in 0..4 {
            for x in 0..4 {
                let esperado = match (x, y) {
                    (1, 1) => "E",
                    (2, 1) => "F",
                    (1, 2) => "H",
                    (2, 2) => "I",
                    _ => " ",
                };
                assert_eq!(t.backend().buffer()[(x, y)].symbol(), esperado);
            }
        }
    }
}
