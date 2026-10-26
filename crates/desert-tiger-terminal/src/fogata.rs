use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

use crate::colores::PaletaColores;
use crate::recursos::ConfiguracionFogata;

pub(crate) const ANCHO: u16 = 12;
pub(crate) const ALTO: u16 = 10;
const REALCE_CONTORNO: f32 = 1.75;

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
    x: i64,
    y: i64,
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
    let direccion = muestra(100 + emisor, ciclo) * std::f32::consts::PI;
    let angulo = if muestra(110 + emisor, ciclo) < 0.85 {
        -direccion
    } else {
        direccion
    };
    let alcance = 2.8 + 1.2 * muestra(80 + emisor, ciclo);
    // El impulso se agota pronto; después quedan la deriva y un bamboleo
    // pequeño. Se muestrea directamente, sin simular fotogramas atrasados.
    let impulso = 1.0 - (-8.0 * edad).exp();
    let flotacion = (edad - 0.25).max(0.0);
    let frecuencia = 9.0 + 5.0 * muestra(90 + emisor, ciclo);
    let fase_zigzag = std::f32::consts::TAU * muestra(120 + emisor, ciclo);
    let zigzag = (fase_zigzag + edad * frecuencia).sin() - fase_zigzag.sin();
    let ondulacion =
        (fase_zigzag * 0.7 + edad * frecuencia * 0.6).sin() - (fase_zigzag * 0.7).sin();
    let vertical = angulo.sin();
    let x = 5.0
        + muestra(70 + emisor, ciclo)
        + angulo.cos() * alcance * impulso
        + 0.22 * flotacion
        + 0.4 * zigzag * impulso;
    let y = 7.0 + vertical * alcance * if vertical < 0.0 { 1.2 } else { 0.25 } * impulso
        - 0.9 * flotacion
        + 0.2 * ondulacion * impulso;
    let (x, y) = (x.round(), y.round());
    if x < 0.0 || x >= f32::from(ANCHO) || y < 0.0 || y >= f32::from(ALTO) {
        return None;
    }
    Some(Chispa {
        x: x as i64,
        y: y as i64,
        brillo: 1.0 - edad_ms / duracion,
    })
}

fn chispa_larga(emisor: u64, t: u64) -> Option<Chispa> {
    let fase = u128::from(t) + u128::from(emisor * 3000);
    let ciclo = (fase / 9000) as u64;
    let edad_ms = (fase % 9000) as f32;
    let duracion = 5000.0 + 3000.0 * muestra(150 + emisor, ciclo);
    if edad_ms >= duracion {
        return None;
    }
    let edad = edad_ms / 1000.0;
    let fase_zigzag = std::f32::consts::TAU * muestra(160 + emisor, ciclo);
    let frecuencia = 2.2 + 1.6 * muestra(170 + emisor, ciclo);
    let zigzag = (fase_zigzag + edad * frecuencia).sin() - fase_zigzag.sin();
    let ondulacion =
        (fase_zigzag * 0.7 + edad * frecuencia * 0.6).sin() - (fase_zigzag * 0.7).sin();
    let x = 5.0
        + muestra(180 + emisor, ciclo)
        + edad * (1.4 + 0.8 * muestra(190 + emisor, ciclo))
        + 0.8 * zigzag;
    let y = 7.0
        - edad * (0.9 + 0.45 * muestra(200 + emisor, ciclo))
        - 0.7 * (1.0 - (-4.0 * edad).exp())
        + 0.3 * ondulacion;
    // Su vida depende solo de la edad. Las coordenadas pueden salir del dibujo
    // de la llama; el compositor aplica el recorte visible al dibujar.
    Some(Chispa {
        x: x.round() as i64,
        y: y.round() as i64,
        brillo: 0.75 * (1.0 - edad_ms / duracion).sqrt(),
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
            // Los glifos todavía pertenecen solo a los retratos: el fuego se
            // dibuja después. No confundimos el recorte con un borde real.
            let borde_derecho = x < lx
                && x + 1 < i64::from(recorte.right())
                && buffer
                    .cell((x as u16, y as u16))
                    .is_some_and(|c| c.symbol() != " ")
                && buffer
                    .cell(((x + 1) as u16, y as u16))
                    .is_some_and(|c| c.symbol() == " ");
            let intensidad =
                (brillo * caida * if borde_derecho { REALCE_CONTORNO } else { 1.0 }).min(1.0);
            if caida > 0.0
                && let Some(celda) = buffer.cell_mut((x as u16, y as u16))
            {
                celda.fg = iluminar(celda.fg, configuracion.luz(), intensidad, paleta);
                celda.bg = iluminar(celda.bg, configuracion.luz(), intensidad, paleta);
            }
        }
    }
    let chispas = std::array::from_fn::<_, 9, _>(|i| {
        if i < 6 {
            chispa(i as u64, instante_ms)
        } else {
            chispa_larga((i - 6) as u64, instante_ms)
        }
    });
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
                let celda_fuego = fuego(x, y, instante_ms);
                if let Some((glifo, color)) = celda_fuego {
                    celda.set_char(glifo).set_fg(color);
                }
            }
        }
    }
    for (indice, chispa) in chispas
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|c| (i, c)))
    {
        if chispas.iter().enumerate().any(|(i, c)| {
            c.is_some_and(|c| {
                c.x == chispa.x
                    && c.y == chispa.y
                    && (c.brillo > chispa.brillo || (c.brillo == chispa.brillo && i < indice))
            })
        }) {
            continue;
        }
        let px = ox.saturating_add(chispa.x);
        let py = oy.saturating_add(chispa.y);
        if px < i64::from(recorte.x)
            || px >= i64::from(recorte.right())
            || py < i64::from(recorte.y)
            || py >= i64::from(recorte.bottom())
        {
            continue;
        }
        if let Some(celda) = buffer.cell_mut((px as u16, py as u16)) {
            celda.set_char(if chispa.brillo > 0.5 { '·' } else { '.' });
            let celda_fuego = if (0..i64::from(ANCHO)).contains(&chispa.x)
                && (0..i64::from(ALTO)).contains(&chispa.y)
            {
                fuego(chispa.x as u16, chispa.y as u16, instante_ms)
            } else {
                None
            };
            if let Some((_, color)) = celda_fuego {
                celda.set_fg(Color::Rgb(255, 255, 255)).set_bg(color);
            } else {
                celda.set_fg(interpolar([96, 35, 0], [255, 224, 96], chispa.brillo));
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
    fn borde_derecho_recibe_mas_luz_sin_aumentar_el_alcance() {
        let config = configuracion();
        let area = Rect::new(0, 0, 160, 24);
        let mut borde = Buffer::empty(area);
        for x in 5..10 {
            borde[(x, 10)]
                .set_char('#')
                .set_fg(Color::Rgb(60, 60, 60))
                .set_bg(Color::Rgb(0, 0, 0));
        }
        borde[(159, 10)]
            .set_char('#')
            .set_fg(Color::Rgb(60, 60, 60));
        let mut interior = borde.clone();
        interior[(10, 10)]
            .set_char('#')
            .set_fg(Color::Rgb(60, 60, 60));
        let lejano = borde[(159, 10)].clone();
        for b in [&mut borde, &mut interior] {
            dibujar(b, area, (15, 3), 0, &config, &PaletaColores::default());
        }
        let Color::Rgb(realce, _, _) = borde[(9, 10)].fg else {
            panic!("borde iluminado");
        };
        let Color::Rgb(normal, _, _) = interior[(9, 10)].fg else {
            panic!("interior iluminado");
        };
        assert!(realce > normal);
        assert_eq!(borde[(9, 10)].symbol(), "#");
        assert_eq!(borde[(9, 10)].bg, Color::Rgb(0, 0, 0));
        assert_eq!(borde[(159, 10)], lejano);
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
                    assert!(
                        c.x >= 0
                            && c.x < i64::from(ANCHO)
                            && c.y >= 0
                            && c.y < i64::from(ALTO)
                            && (0.0..=1.0).contains(&c.brillo)
                    );
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
        assert_ne!((posterior.x, posterior.y), (inicial.x, inicial.y));
        assert!(posterior.brillo < inicial.brillo);
        let (mut izquierda, mut derecha, mut asciende) = (false, false, false);
        let (mut recorrido_inicial, mut recorrido_final) = (0u64, 0u64);
        for ciclo in 1..=4 {
            for emisor in 0..6 {
                let inicio = ciclo * 1800 - emisor * 300;
                let a = chispa(emisor, inicio).unwrap();
                let b = chispa(emisor, inicio + 200).unwrap();
                let c = chispa(emisor, inicio + 600).unwrap();
                let d = chispa(emisor, inicio + 800).unwrap();
                izquierda |= b.x < a.x;
                derecha |= b.x > a.x;
                asciende |= b.y < a.y;
                recorrido_inicial += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
                recorrido_final += c.x.abs_diff(d.x) + c.y.abs_diff(d.y);
            }
        }
        assert!(izquierda && derecha && asciende);
        assert!(
            recorrido_inicial > recorrido_final,
            "el impulso debe ceder a la flotación"
        );
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
    #[test]
    fn chispas_largas_viven_fuera_de_la_llama_y_se_recortan_al_area_de_arte() {
        let a = chispa_larga(0, 0).unwrap();
        let b = chispa_larga(0, 4900).unwrap();
        assert!(b.x > a.x && b.y < a.y);
        assert!(b.x >= i64::from(ANCHO) || b.y < 0);
        assert!(b.brillo < a.brillo);
        assert!(chispa_larga(0, 8000).is_none());
        let area = Rect::new(0, 0, 40, 24);
        let recorte = Rect::new(2, 2, 30, 20);
        let mut buffer = Buffer::empty(area);
        dibujar(
            &mut buffer,
            recorte,
            (5, 10),
            4900,
            &configuracion(),
            &PaletaColores::default(),
        );
        let x = (5 + b.x) as u16;
        let y = (10 + b.y) as u16;
        assert!(recorte.contains((x, y).into()));
        assert!(["·", "."].contains(&buffer[(x, y)].symbol()));
        for y in 0..24 {
            for x in 0..40 {
                if !recorte.contains((x, y).into()) {
                    assert_eq!(buffer[(x, y)].symbol(), " ");
                }
            }
        }
        assert!(chispa_larga(2, u64::MAX).is_none_or(|c| c.brillo.is_finite()));
    }
}
