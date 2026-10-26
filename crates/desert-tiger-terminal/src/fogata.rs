use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

use crate::colores::PaletaColores;
use crate::recursos::ConfiguracionFogata;

pub(crate) const ANCHO: u16 = 12;
pub(crate) const ALTO: u16 = 10;

// Hash cosmético explícito: ni la semilla ni el RNG de la Partida intervienen.
fn muestra(clave: u64, ciclo: u64) -> f32 {
    let mut v = ciclo.wrapping_add(clave.wrapping_mul(0x9e3779b97f4a7c15)) ^ 0xf17e2026;
    v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    v ^= v >> 31;
    (v >> 40) as f32 / 16_777_215.0
}

fn ruido(clave: u64, t: u64, periodo: u64) -> f32 {
    let ciclo = t / periodo;
    let s = (t % periodo) as f32 / periodo as f32;
    let s = s * s * (3.0 - 2.0 * s);
    let a = muestra(clave, ciclo);
    a + (muestra(clave, ciclo.wrapping_add(1)) - a) * s
}

fn interpolar(a: [u8; 3], b: [u8; 3], factor: f32) -> Color {
    let c = std::array::from_fn::<_, 3, _>(|i| {
        (f32::from(a[i]) + (f32::from(b[i]) - f32::from(a[i])) * factor)
            .round()
            .clamp(0.0, 255.0) as u8
    });
    Color::Rgb(c[0], c[1], c[2])
}

fn color_llama(intensidad: f32) -> Color {
    let (a, b, factor) = if intensidad < 0.35 {
        ([120, 24, 0], [255, 96, 0], intensidad / 0.35)
    } else if intensidad < 0.70 {
        ([255, 96, 0], [255, 200, 64], (intensidad - 0.35) / 0.35)
    } else {
        ([255, 200, 64], [255, 244, 192], (intensidad - 0.70) / 0.30)
    };
    interpolar(a, b, factor)
}

fn fuego(x: u16, y: u16, t: u64) -> Option<(char, Color)> {
    if y == 9 && (2..=8).contains(&x) {
        return Some((
            '▄',
            interpolar([70, 32, 14], [116, 58, 20], ruido(40, t, 900)),
        ));
    }
    if y == 8 && (3..=7).contains(&x) {
        return Some((
            '▓',
            interpolar([150, 30, 0], [255, 125, 16], ruido(u64::from(x), t, 300)),
        ));
    }
    if !(1..=7).contains(&y) {
        return None;
    }
    let h = f32::from(8 - y) / 7.0;
    let centro = 5.0 + h * (0.8 + 1.2 * (2.0 * ruido(0, t, 400) - 1.0));
    let ancho = (3.2 * (1.0 - h) + 0.45) * (0.85 + 0.3 * ruido(u64::from(y), t, 200));
    let perfil = (1.0 - (f32::from(x) - centro).abs() / ancho).max(0.0);
    let intensidad = (perfil * (1.0 - 0.45 * h) * (0.8 + 0.2 * ruido(20 + u64::from(y), t, 150)))
        .clamp(0.0, 1.0);
    let glifo = match intensidad {
        v if v < 0.12 => return None,
        v if v < 0.35 => '░',
        v if v < 0.60 => '▒',
        v if v < 0.82 => '▓',
        _ => '█',
    };
    Some((glifo, color_llama(intensidad)))
}

#[derive(Clone, Copy)]
struct Chispa {
    x: u16,
    y: u16,
    brillo: f32,
}

fn chispa(emisor: u64, t: u64) -> Option<Chispa> {
    // u128 evita desbordes aun al muestrear el extremo del reloj visual.
    let fase = u128::from(t) + u128::from(emisor * 300);
    let ciclo = (fase / 1800) as u64;
    let edad_ms = (fase % 1800) as f32;
    let duracion = 900.0 + 500.0 * muestra(60 + emisor, ciclo);
    if edad_ms >= duracion {
        return None;
    }
    let edad = edad_ms / 1000.0;
    let x = 5.0 + muestra(70 + emisor, ciclo) + edad * (1.0 + muestra(80 + emisor, ciclo));
    let y = 7.0 - edad * (3.0 + muestra(90 + emisor, ciclo));
    let (x, y) = (x.round(), y.round());
    if x < 0.0 || x >= f32::from(ANCHO) || y < 0.0 || y >= f32::from(ALTO) {
        return None;
    }
    Some(Chispa {
        x: x as u16,
        y: y as u16,
        brillo: 1.0 - edad_ms / duracion,
    })
}

fn iluminar(color: Color, luz: [u8; 3], intensidad: f32, paleta: &PaletaColores) -> Color {
    if intensidad <= 0.0 {
        return color;
    }
    let Some(original) = paleta.para_iluminar(color) else {
        return color;
    };
    if original == [0, 0, 0] {
        return color;
    }
    let c = std::array::from_fn::<_, 3, _>(|i| {
        let c = f32::from(original[i]);
        (c + intensidad * (255.0 - c) * f32::from(luz[i]) / 255.0)
            .round()
            .clamp(0.0, 255.0) as u8
    });
    Color::Rgb(c[0], c[1], c[2])
}

pub(crate) fn dibujar(
    buffer: &mut Buffer,
    recorte: Rect,
    origen: (i64, i64),
    instante_ms: u64,
    configuracion: &ConfiguracionFogata,
    paleta: &PaletaColores,
) {
    let recorte = recorte.intersection(buffer.area);
    let (ox, oy) = origen;
    let (lx, ly) = (ox.saturating_add(5), oy.saturating_add(7));
    let radio = i64::from(configuracion.radio_luz());
    let brillo = configuracion.intensidad_luz() * (0.85 + 0.15 * ruido(0, instante_ms, 400));
    // Recortamos el halo antes del recorrido. Un origen fuera de la pantalla
    // todavía puede iluminar una parte visible del retrato.
    for y in i64::from(recorte.y).max(ly.saturating_sub(radio / 2))
        ..i64::from(recorte.bottom()).min(ly.saturating_add(radio / 2 + 1))
    {
        for x in i64::from(recorte.x).max(lx.saturating_sub(radio))
            ..i64::from(recorte.right()).min(lx.saturating_add(radio + 1))
        {
            let dx = (x - lx) as f32;
            let dy = (y - ly) as f32 * 2.0;
            let caida = (1.0 - dx.hypot(dy) / radio as f32).max(0.0).powi(2);
            if caida > 0.0
                && let Some(celda) = buffer.cell_mut((x as u16, y as u16))
            {
                celda.fg = iluminar(celda.fg, configuracion.luz(), brillo * caida, paleta);
                celda.bg = iluminar(celda.bg, configuracion.luz(), brillo * caida, paleta);
            }
        }
    }
    let chispas = std::array::from_fn::<_, 6, _>(|i| chispa(i as u64, instante_ms));
    for y in 0..ALTO {
        for x in 0..ANCHO {
            let px = ox.saturating_add(i64::from(x));
            let py = oy.saturating_add(i64::from(y));
            if px < i64::from(recorte.x)
                || px >= i64::from(recorte.right())
                || py < i64::from(recorte.y)
                || py >= i64::from(recorte.bottom())
            {
                continue;
            }
            if let Some(celda) = buffer.cell_mut((px as u16, py as u16)) {
                if let Some((glifo, color)) = fuego(x, y, instante_ms) {
                    celda.set_char(glifo).set_fg(color);
                }
                if let Some(chispa) = chispas
                    .iter()
                    .flatten()
                    .filter(|c| c.x == x && c.y == y)
                    .max_by(|a, b| a.brillo.total_cmp(&b.brillo))
                {
                    celda
                        .set_char(if chispa.brillo > 0.5 { '·' } else { '.' })
                        .set_fg(interpolar([96, 35, 0], [255, 224, 96], chispa.brillo));
                }
            }
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::recursos::RecursosVisuales;
    use ratatui::style::{Modifier, Style};
    use std::path::Path;
    use unicode_width::UnicodeWidthChar;

    fn configuracion() -> ConfiguracionFogata {
        RecursosVisuales::cargar(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap()
        .efecto_fogata
        .unwrap()
    }

    #[test]
    fn fuego_y_chispas_son_reproducibles_y_glifos_de_una_columna() {
        let area = Rect::new(0, 0, ANCHO, ALTO);
        let config = configuracion();
        let generar = |t| {
            let mut b = Buffer::empty(area);
            dibujar(&mut b, area, (0, 0), t, &config, &PaletaColores::fija());
            b
        };
        assert_eq!(generar(0), generar(0));
        assert_ne!(generar(0), generar(450));
        let mut colores = std::collections::BTreeSet::new();
        let mut vio_chispa = false;
        for t in (0..6000).step_by(50).chain([u64::MAX]) {
            let b = generar(t);
            for celda in &b.content {
                for c in celda.symbol().chars() {
                    assert_eq!(UnicodeWidthChar::width(c), Some(1));
                }
                if let Color::Rgb(r, g, b) = celda.fg {
                    colores.insert((r, g, b));
                }
                vio_chispa |= celda.symbol() == "·" || celda.symbol() == ".";
            }
            for emisor in 0..6 {
                if let Some(c) = chispa(emisor, t) {
                    assert!(c.x < ANCHO && c.y < ALTO && (0.0..=1.0).contains(&c.brillo));
                }
            }
        }
        assert!(vio_chispa);
        assert!(
            colores.len() > 100,
            "el degradado debe producir más que una paleta nominal"
        );
        let inicial = chispa(0, 0).unwrap();
        let posterior = chispa(0, 500).unwrap();
        assert!(posterior.y < inicial.y && posterior.x >= inicial.x);
        assert!(posterior.brillo < inicial.brillo);
    }

    #[test]
    fn luz_preserva_negro_simbolos_y_modificadores_y_decrece_con_distancia() {
        let config = configuracion();
        let area = Rect::new(0, 0, 40, 20);
        let mut b = Buffer::empty(area);
        for c in &mut b.content {
            c.set_char('@').set_style(
                Style::default()
                    .fg(Color::Blue)
                    .bg(Color::Rgb(0, 0, 0))
                    .add_modifier(Modifier::BOLD),
            );
        }
        let antes = b.clone();
        // La llama queda fuera; solo su halo llega al lado derecho del retrato.
        dibujar(&mut b, area, (40, 6), 0, &config, &PaletaColores::fija());
        for (nuevo, original) in b.content.iter().zip(&antes.content) {
            assert_eq!(nuevo.symbol(), original.symbol());
            assert_eq!(nuevo.bg, original.bg);
            assert_eq!(nuevo.modifier, original.modifier);
        }
        assert_eq!(b[(0, 13)], antes[(0, 13)]);
        let Color::Rgb(lejos, _, _) = b[(25, 13)].fg else {
            panic!("luz lejana");
        };
        let Color::Rgb(cerca, _, _) = b[(39, 13)].fg else {
            panic!("luz cercana");
        };
        assert!(cerca > lejos);
        assert_eq!(
            iluminar(Color::Black, config.luz(), 0.9, &PaletaColores::fija()),
            Color::Black
        );
        assert_eq!(
            iluminar(
                Color::Rgb(0, 0, 0),
                config.luz(),
                0.9,
                &PaletaColores::fija()
            ),
            Color::Rgb(0, 0, 0)
        );
        assert_eq!(
            iluminar(Color::Green, config.luz(), 0.0, &PaletaColores::fija()),
            Color::Green
        );
        assert_eq!(
            iluminar(Color::Reset, config.luz(), 0.9, &PaletaColores::fija()),
            Color::Reset
        );
        assert_eq!(
            iluminar(
                Color::Indexed(7),
                config.luz(),
                0.9,
                &PaletaColores::default()
            ),
            Color::Indexed(7)
        );
        assert_ne!(
            iluminar(
                Color::Rgb(20, 30, 40),
                config.luz(),
                0.45,
                &PaletaColores::fija()
            ),
            Color::Rgb(20, 30, 40)
        );
    }

    #[test]
    fn recorte_y_origenes_extremos_no_escriben_fuera_del_arte() {
        let config = configuracion();
        let area = Rect::new(0, 0, 40, 20);
        let recorte = Rect::new(5, 5, 20, 10);
        for origen in [
            (10, 8),
            (-4, -2),
            (i64::MAX, i64::MIN),
            (i64::MIN, i64::MAX),
        ] {
            let mut b = Buffer::filled(area, ratatui::buffer::Cell::new("#"));
            let antes = b.clone();
            dibujar(
                &mut b,
                recorte,
                origen,
                u64::MAX,
                &config,
                &PaletaColores::fija(),
            );
            for y in 0..20 {
                for x in 0..40 {
                    if !recorte.contains((x, y).into()) {
                        assert_eq!(b[(x, y)], antes[(x, y)]);
                    }
                }
            }
        }
        let mut b = Buffer::empty(area);
        let antes = b.clone();
        dibujar(
            &mut b,
            Rect::default(),
            (0, 0),
            0,
            &config,
            &PaletaColores::fija(),
        );
        assert_eq!(b, antes);
    }
    #[test]
    fn luz_nativa_usa_solo_los_canales_consultados_y_conserva_el_resto() {
        let mut paleta = PaletaColores::default();
        assert_eq!(
            iluminar(Color::LightMagenta, [255, 180, 72], 0.1, &paleta),
            Color::LightMagenta
        );
        paleta.registrar(13, [187, 136, 170]);
        let iluminado = iluminar(Color::LightMagenta, [255, 180, 72], 0.1, &paleta);
        assert_eq!(iluminado, Color::Rgb(194, 144, 172));
        assert_eq!(
            iluminar(Color::Blue, [255, 180, 72], 0.1, &paleta),
            Color::Blue
        );
        assert_eq!(
            iluminar(Color::Rgb(32, 32, 32), [255, 180, 72], 0.1, &paleta),
            Color::Rgb(54, 48, 38)
        );
    }
}
