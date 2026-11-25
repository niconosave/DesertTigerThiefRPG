//! Reproducción local de estados ya resueltos; no contiene reglas de combate.
use crate::recursos::RecursosVisuales;
use desert_tiger_app::{EscenarioEncuentro, ObjetivoAtaque, TerrenoEncuentro, VistaEncuentro};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Clear, Paragraph},
};
#[derive(Clone)]
pub(crate) enum ObjetivoSeleccion {
    Actor(ObjetivoAtaque),
    Objeto(desert_tiger_app::ObjetivoObjeto),
}
impl ObjetivoSeleccion {
    pub fn nombre(&self) -> &str {
        match self {
            Self::Actor(a) => &a.nombre,
            Self::Objeto(o) => &o.nombre,
        }
    }
}

fn solido(t: TerrenoEncuentro) -> bool {
    matches!(
        t,
        TerrenoEncuentro::Suelo | TerrenoEncuentro::Muro | TerrenoEncuentro::Obstaculo
    )
}

// Sombra ficticia sobre suelo: tres casillas por tramo de altura hacia -X,+Y.
fn suelo_en_sombra(v: &VistaEncuentro, x: u16, y: u16, z: u16) -> bool {
    let celda = |x: u16, y: u16, z: u16| {
        if x >= v.ancho || y >= v.alto || z >= v.altura {
            return None;
        }
        v.celdas
            .get(usize::from((z * v.alto + y) * v.ancho + x))
            .copied()
    };
    if celda(x, y, z) != Some(TerrenoEncuentro::Suelo)
        || !matches!(
            celda(x, y, z + 1),
            Some(TerrenoEncuentro::Aire | TerrenoEncuentro::Vacio)
        )
    {
        return false;
    }
    for paso in 1..=v.altura.saturating_sub(1).saturating_sub(z) {
        for (dx, dy) in [(paso, paso), (paso, paso - 1), (paso - 1, paso)] {
            let Some(sx) = x.checked_add(dx) else {
                continue;
            };
            let Some(sy) = y.checked_sub(dy) else {
                continue;
            };
            // Cada bloque conocido proyecta su huella, incluso sobre un hueco.
            if celda(sx, sy, z + paso).is_some_and(solido) {
                return true;
            }
        }
    }
    false
}

fn apariencia_solido(
    v: &VistaEncuentro,
    x: u16,
    y: u16,
    z: u16,
    corte: u16,
) -> (&'static str, Color) {
    let bloque = if z == corte {
        Color::Gray
    } else if z.is_multiple_of(2) {
        Color::DarkGray
    } else {
        crate::dibujo_caracteres::color_retrato(desert_tiger_app::ColorRetrato::GrisMuyOscuro)
    };
    let celda = |x: u16, y: u16, z: u16| {
        if x >= v.ancho || y >= v.alto || z >= v.altura {
            return None;
        }
        v.celdas
            .get(usize::from((z * v.alto + y) * v.ancho + x))
            .copied()
    };
    // El corte atraviesa el sólido. Desconocido encima no demuestra una cara expuesta.
    if z >= corte
        || !matches!(
            celda(x, y, z + 1),
            Some(TerrenoEncuentro::Aire | TerrenoEncuentro::Vacio)
        )
    {
        return (" ", bloque);
    }
    let interior = x > 0
        && y > 0
        && [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .all(|&(nx, ny)| celda(nx, ny, z).is_some_and(solido));
    if interior {
        (".", Color::Reset)
    } else {
        ("+", bloque)
    }
}

pub(crate) fn foco(v: &VistaEncuentro) -> Option<&desert_tiger_app::FichaEncuentro> {
    v.fichas
        .iter()
        .filter(|f| f.aliado && !f.salio && !f.transportado)
        .min_by_key(|f| (f.id != 0, f.id))
}
#[derive(Clone, Copy)]
pub(crate) struct CamaraEncuentro {
    pub corte: u16,
    pub manual: bool,
    pub centro: Option<(u16, u16)>,
}

pub(crate) fn dibujar(
    frame: &mut Frame<'_>,
    v: &VistaEncuentro,
    r: &RecursosVisuales,
    pausa: bool,
    detalle: bool,
    camara: CamaraEncuentro,
) {
    let CamaraEncuentro {
        corte,
        manual: corte_manual,
        centro: centro_guardado,
    } = camara;
    let a = frame.area();
    frame.render_widget(Clear, a);
    frame.render_widget(Paragraph::new("").style(r.ui.pantalla), a);
    if a.width < 20 || a.height < 8 {
        frame.render_widget(
            Paragraph::new("Ampliá la terminal para ver el encuentro."),
            a,
        );
        return;
    }
    frame.render_widget(
        Paragraph::new(format!(
            "{} · {} · ronda {} · {} · z≤{} {}",
            if v.recuperacion {
                "RECUPERAR"
            } else if v.rescate {
                "RESCATE"
            } else {
                "ATAQUE"
            },
            match v.escenario {
                EscenarioEncuentro::Recinto => "recinto",
                EscenarioEncuentro::Campamento => "campamento",
                EscenarioEncuentro::Palacio => "palacio",
                EscenarioEncuentro::Ciudad => "ciudad",
                EscenarioEncuentro::Bosque => "bosque",
                EscenarioEncuentro::Montana => "montaña",
                EscenarioEncuentro::Playa => "playa",
                EscenarioEncuentro::DosAlturas => "dos alturas",
            },
            v.ronda,
            if pausa { "pausa" } else { "en curso" },
            corte.min(v.altura.saturating_sub(1)),
            if corte_manual {
                "manual"
            } else {
                "seguimiento"
            }
        ))
        .style(r.ui.titulo),
        Rect::new(1, 0, a.width - 2, 1),
    );
    let alto = a.height.saturating_sub(9).min(v.alto).max(1);
    let ancho = (a.width.saturating_sub(2) / 2).min(v.ancho).max(1);
    let centro = foco(v)
        .map(|f| (f.x, f.y))
        .or(centro_guardado)
        .unwrap_or((v.ancho / 2, v.alto / 2));
    let ox = centro
        .0
        .saturating_sub(ancho / 2)
        .min(v.ancho.saturating_sub(ancho));
    let oy = centro
        .1
        .saturating_sub(alto / 2)
        .min(v.alto.saturating_sub(alto));
    let mut desconocidos = std::collections::BTreeSet::new();
    let izquierda = (a.width - ancho * 2) / 2;
    let mut propietarios = std::collections::BTreeMap::new();
    // Composición de fondo a frente; Z solo desempata dentro de una fila física.
    for y in 0..v.alto {
        for z in 0..=corte.min(v.altura.saturating_sub(1)) {
            for x in ox..ox + ancho {
                let fila = 2 + i32::from(y) - i32::from(oy) - i32::from(z);
                let sx = izquierda + (x - ox) * 2;
                let t = v.celdas[usize::from((z * v.alto + y) * v.ancho + x)];
                let color = if z % 2 == 0 { Color::Cyan } else { Color::Blue };
                let (glifo, mut fondo) = match t {
                    TerrenoEncuentro::Aire | TerrenoEncuentro::Vacio => continue,
                    TerrenoEncuentro::Desconocido => (" ", Color::Reset),
                    TerrenoEncuentro::Suelo
                    | TerrenoEncuentro::Muro
                    | TerrenoEncuentro::Obstaculo => apariencia_solido(v, x, y, z, corte),
                    TerrenoEncuentro::PuertaAbierta => ("/", Color::Reset),
                    TerrenoEncuentro::PuertaCerrada => ("+", Color::DarkGray),
                    TerrenoEncuentro::Indefinido => (";", Color::Reset),
                };
                let piso = t == TerrenoEncuentro::Suelo && glifo == ".";
                if piso {
                    fondo = if suelo_en_sombra(v, x, y, z) {
                        Color::Reset
                    } else {
                        match (z % 2, (x + y) % 2) {
                            (0, 0) => Color::Rgb(18, 26, 36),
                            (0, _) => Color::Rgb(25, 35, 46),
                            (_, 0) => Color::Rgb(24, 34, 25),
                            (_, _) => Color::Rgb(33, 44, 31),
                        }
                    };
                }
                let superior = solido(t) && z < corte && matches!(glifo, "." | "+");
                // La cara superior está en z+1; la cara vertical permanece en z.
                let mut muestras = vec![];
                if superior {
                    if !piso {
                        muestras.push((fila, [" ", " "]));
                    }
                    muestras.push((fila - 1, if piso { [" ", " "] } else { [glifo, " "] }));
                } else {
                    muestras.push((fila, [glifo, " "]));
                }
                for (fila, simbolos) in muestras {
                    if fila < 2 || fila >= i32::from(2 + alto) {
                        continue;
                    }
                    let sy = fila as u16;
                    for dx in 0..2 {
                        if t == TerrenoEncuentro::Desconocido {
                            desconocidos.insert((sx + dx, sy));
                        }
                        propietarios
                            .insert((sx + dx, sy), (t == TerrenoEncuentro::Desconocido, None));
                        if let Some(c) = frame.buffer_mut().cell_mut((sx + dx, sy)) {
                            c.set_symbol(simbolos[usize::from(dx)])
                                .set_style(Style::default().fg(color).bg(fondo));
                        }
                    }
                }
            }
            for &(x, ey, ez) in &v.escaleras {
                if ey != y || ez != z || x < ox || x >= ox + ancho {
                    continue;
                }
                let fila = 2 + i32::from(y) - i32::from(oy) - i32::from(z);
                if fila >= 2 && fila < i32::from(2 + alto) {
                    frame.buffer_mut()[(izquierda + (x - ox) * 2, fila as u16)]
                        .set_symbol(if r.ascii { ">" } else { "△" })
                        .set_fg(Color::Blue);
                }
            }
            let mut fichas: Vec<_> = v.fichas.iter().collect();
            fichas.sort_by_key(|f| std::cmp::Reverse(f.id));
            for f in fichas {
                if f.salio || f.transportado || f.y != y || f.x < ox || f.x >= ox + ancho {
                    continue;
                }
                let torso = f.golpes < 2 && z == f.z + 1;
                if z != f.z && !torso {
                    continue;
                }
                let fila = 2 + i32::from(y) - i32::from(oy) - i32::from(z);
                if fila < 2 || fila >= i32::from(2 + alto) {
                    continue;
                }
                let glifo = if f.golpes >= 3 {
                    '%'
                } else if f.golpes == 2 {
                    'x'
                } else if !torso {
                    '^'
                } else if f.rescatado {
                    '@'
                } else if f.aliado {
                    (b'A' + (f.id % 26) as u8) as char
                } else {
                    'E'
                };
                let color = if f.golpes > 0 {
                    Color::LightRed
                } else if f.aliado {
                    Color::Cyan
                } else {
                    Color::Yellow
                };
                propietarios.insert(
                    (izquierda + (f.x - ox) * 2, fila as u16),
                    (false, Some(f.id)),
                );
                frame.buffer_mut()[(izquierda + (f.x - ox) * 2, fila as u16)]
                    .set_char(glifo)
                    .set_fg(color);
                if torso && let Some(acierto) = f.impacto {
                    let sx = izquierda + (f.x - ox) * 2 + 1;
                    frame.buffer_mut()[(sx, fila as u16)]
                        .set_symbol(if acierto { "*" } else { "-" })
                        .set_fg(color);
                    propietarios.insert((sx, fila as u16), (false, Some(f.id)));
                }
            }
            if let Some(o) = &v.objeto
                && !o.fuera
                && !o.recogido
                && o.y == y
                && o.z == z
                && o.x >= ox
                && o.x < ox + ancho
            {
                let fila = 2 + i32::from(y) - i32::from(oy) - i32::from(z);
                if fila >= 2 && fila < i32::from(2 + alto) {
                    frame.buffer_mut()[(izquierda + (o.x - ox) * 2, fila as u16)]
                        .set_symbol("$")
                        .set_fg(Color::Yellow);
                }
            }
        }
    }
    for f in &v.fichas {
        if f.salio || f.transportado || f.z > corte || f.x < ox || f.x >= ox + ancho {
            continue;
        }
        let z = (f.z + u16::from(f.golpes < 2)).min(corte);
        let fila = 2 + i32::from(f.y) - i32::from(oy) - i32::from(z);
        if fila < 2 || fila >= i32::from(2 + alto) {
            continue;
        }
        let punto = (izquierda + (f.x - ox) * 2, fila as u16);
        if propietarios.get(&punto) == Some(&(false, None)) && !desconocidos.contains(&punto) {
            let letra = if f.golpes >= 3 {
                '%'
            } else if f.golpes == 2 {
                'x'
            } else if f.aliado {
                (b'A' + (f.id % 26) as u8) as char
            } else {
                'E'
            };
            frame.buffer_mut()[punto]
                .set_char(letra)
                .set_fg(Color::Rgb(0, 0, 0))
                .set_bg(if f.aliado { Color::Cyan } else { Color::Yellow });
        }
    }
    let pie = 2 + alto;
    frame.render_widget(
        Paragraph::new(if v.evacuacion_pendiente.is_empty() { v.texto.clone() } else {
                format!("No se pudo evacuar a: {}. Enter: continuar el auxilio. A: confirmar que quedan atrás.", v.evacuacion_pendiente.iter().map(|p| format!("{} ({})", p.nombre, p.motivo)).collect::<Vec<_>>().join(", "))
            })
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(r.ui.texto_normal),
        Rect::new(1, pie, a.width - 2, 3.min(a.height.saturating_sub(pie))),
    );
    let roster = v
        .fichas
        .iter()
        .enumerate()
        .filter(|(_, f)| f.aliado)
        .map(|(_, f)| {
            format!(
                "{} {}{}",
                if f.rescatado {
                    '@'
                } else {
                    (b'A' + (f.id % 26) as u8) as char
                },
                f.nombre,
                if f.golpes >= 3 {
                    " (muerto)"
                } else if f.golpes == 2 {
                    " (incapacitado)"
                } else if f.golpes == 1 {
                    " (herido)"
                } else if f.cautivo {
                    " (cautivo)"
                } else if f.rescatado {
                    " (acompañando)"
                } else {
                    ""
                }
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    frame.render_widget(
        Paragraph::new(roster).style(r.ui.secundario),
        Rect::new(1, a.height - 3, a.width - 2, 1),
    );
    if let Some(o) = &v.objeto {
        frame.render_widget(
            Paragraph::new(format!(
                "$ {} · {}",
                o.nombre,
                if o.fuera {
                    "fuera del escenario"
                } else if o.recogido {
                    "transportado"
                } else {
                    "en el lugar"
                }
            ))
            .style(r.ui.secundario),
            Rect::new(1, a.height - 2, a.width - 2, 1),
        );
    }
    let controles = if v.finalizado {
        "Enter: cerrar episodio · D: detalles · Q: salir"
    } else if !v.evacuacion_pendiente.is_empty() {
        "Evacuación incompleta: Enter auxiliar · A abandonar · D detalles · Q salir"
    } else if v.pausa_decision {
        "Sin avance: Enter reintentar · R retirarse · D detalles · Q salir"
    } else {
        "Enter/Espacio pausa · R retirada · +/- velocidad · S omitir · D detalles · Q salir"
    };
    frame.render_widget(
        Paragraph::new(controles).style(r.ui.barra_acciones),
        Rect::new(0, a.height - 1, a.width, 1),
    );
    if detalle {
        let h = a.height.saturating_sub(4).min(12);
        let area = Rect::new(2, 2, a.width - 4, h);
        frame.render_widget(Clear, area);
        let t = if v.detalle.is_empty() {
            "No hubo intercambios en este paso.".into()
        } else {
            v.detalle.join("\n")
        };
        frame.render_widget(
            Paragraph::new(t)
                .wrap(ratatui::widgets::Wrap { trim: true })
                .style(r.ui.texto_normal),
            area,
        );
    }
}
pub(crate) fn objetivos(
    frame: &mut Frame<'_>,
    opciones: &[ObjetivoSeleccion],
    foco: usize,
    r: &RecursosVisuales,
    verbo: &str,
) {
    let a = frame.area();
    let w = a.width.min(48);
    let h = (opciones.len() as u16 + 3).min(a.height);
    let area = Rect::new(a.width.saturating_sub(w), 0, w, h);
    frame.render_widget(Clear, area);
    let texto = std::iter::once(if verbo == "recuperar" {
        "Elegí qué recuperar (Esc cancela)".into()
    } else {
        format!("Elegí a quién {verbo} (Esc cancela)")
    })
    .chain(
        opciones
            .iter()
            .enumerate()
            .map(|(i, o)| format!("{} {}", if i == foco { ">" } else { " " }, o.nombre())),
    )
    .collect::<Vec<_>>()
    .join("\n");
    frame.render_widget(
        Paragraph::new(texto).style(r.ui.texto_normal.fg(Color::Cyan)),
        area,
    );
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_app::FichaEncuentro;
    use ratatui::{Terminal, backend::TestBackend};
    fn fixture() -> VistaEncuentro {
        VistaEncuentro {
            escaleras: vec![],
            recuperacion: false,
            objeto: None,
            rescate: false,
            escenario: EscenarioEncuentro::Recinto,
            id: 1,
            paso: 0,
            ancho: 9,
            alto: 9,
            altura: 3,
            celdas: {
                let mut c = vec![TerrenoEncuentro::Aire; 9 * 9 * 3];
                c[..81].fill(TerrenoEncuentro::Suelo);
                c
            },
            fichas: vec![FichaEncuentro {
                cautivo: false,
                rescatado: false,
                id: 0,
                impacto: Some(true),
                nombre: "Prueba".into(),
                x: 4,
                y: 4,
                z: 1,
                aliado: true,
                golpes: 1,
                salio: false,
                transportado: false,
            }],
            texto: "El grupo entra en combate.".into(),
            detalle: vec![],
            ronda: 1,
            finalizado: false,
            puede_retirar: true,
            pausa_decision: false,
            evacuacion_pendiente: Vec::new(),
            retirando: false,
        }
    }

    #[test]
    fn sombra_fondo_negro_y_longitud_independiente_del_corte() {
        use ratatui::{Terminal, backend::TestBackend};
        let recursos = RecursosVisuales::cargar(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap();
        let mut v = fixture();
        v.fichas.clear();
        v.altura = 5;
        v.celdas.resize(9 * 9 * 5, TerrenoEncuentro::Aire);
        for z in 1..=3 {
            v.celdas[(z * 9 + 2) * 9 + 5] = TerrenoEncuentro::Muro;
        }
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        for corte in [1, 2, 4] {
            terminal
                .draw(|f| {
                    dibujar(
                        f,
                        &v,
                        &recursos,
                        false,
                        false,
                        CamaraEncuentro {
                            corte,
                            manual: true,
                            centro: None,
                        },
                    )
                })
                .unwrap();
            // Piso en (2,5): extremo lejano de la sombra de altura tres.
            assert_eq!(terminal.backend().buffer()[(35, 6)].bg, Color::Reset);
            assert_eq!(
                terminal.backend().buffer()[(33, 7)].bg,
                Color::Rgb(25, 35, 46)
            );
        }
    }
    #[test]
    fn sombra_bloque_elevado_con_aire_debajo() {
        let mut v = fixture();
        v.altura = 5;
        v.celdas.resize(9 * 9 * 5, TerrenoEncuentro::Aire);
        let bloque = (3 * 9 + 2) * 9 + 5;
        v.celdas[bloque] = TerrenoEncuentro::Suelo;
        for (x, y) in [(2, 4), (2, 5), (3, 5)] {
            assert!(suelo_en_sombra(&v, x, y, 0));
        }
        // Los niveles vacíos no rellenan los tramos primero y segundo.
        assert!(!suelo_en_sombra(&v, 4, 3, 0));
        assert!(!suelo_en_sombra(&v, 3, 4, 0));
        v.celdas[bloque] = TerrenoEncuentro::Desconocido;
        assert!(!suelo_en_sombra(&v, 2, 5, 0));
        v.celdas[bloque] = TerrenoEncuentro::Aire;
        assert!(!suelo_en_sombra(&v, 2, 5, 0));
    }
    #[test]
    fn sombra_diagonal_tres_niveles_solo_en_suelo_conocido() {
        let mut v = fixture();
        v.altura = 5;
        v.celdas.resize(9 * 9 * 5, TerrenoEncuentro::Aire);
        for z in 1..=3 {
            v.celdas[(z * 9 + 2) * 9 + 5] = TerrenoEncuentro::Muro;
        }
        for (x, y) in [
            (4, 2),
            (4, 3),
            (5, 3),
            (3, 3),
            (3, 4),
            (4, 4),
            (2, 4),
            (2, 5),
            (3, 5),
        ] {
            assert!(suelo_en_sombra(&v, x, y, 0), "{x},{y}");
        }
        assert!(!suelo_en_sombra(&v, 1, 6, 0));
        assert!(!suelo_en_sombra(&v, 5, 1, 0));
        assert!(suelo_en_sombra(&v, 2, 5, 0));
        v.celdas[5 * 9 + 2] = TerrenoEncuentro::Muro;
        assert!(!suelo_en_sombra(&v, 2, 5, 0));
        v.celdas[(3 * 9 + 2) * 9 + 5] = TerrenoEncuentro::Desconocido;
        assert!(!suelo_en_sombra(&v, 3, 5, 0));
    }
    #[test]
    fn solidos_corte_superficie_y_coronamiento() {
        let mut v = fixture();
        assert_eq!(apariencia_solido(&v, 4, 4, 0, 0), (" ", Color::Gray));
        assert_eq!(apariencia_solido(&v, 4, 4, 0, 1), (".", Color::Reset));
        v.celdas[81 + 4 * 9 + 4] = TerrenoEncuentro::Muro;
        assert_eq!(apariencia_solido(&v, 4, 4, 0, 2), (" ", Color::DarkGray));
        assert_eq!(apariencia_solido(&v, 4, 4, 1, 1), (" ", Color::Gray));
        assert_eq!(
            apariencia_solido(&v, 4, 4, 1, 2),
            ("+", Color::Rgb(32, 32, 32))
        );
        v.celdas[162 + 4 * 9 + 4] = TerrenoEncuentro::Desconocido;
        assert_eq!(
            apariencia_solido(&v, 4, 4, 1, 2),
            (" ", Color::Rgb(32, 32, 32))
        );
    }
    #[test]
    fn vista_oblicua_cuerpo_escalera_y_corte_estable() {
        use ratatui::{Terminal, backend::TestBackend};
        let recursos = RecursosVisuales::cargar(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap();
        let mut v = fixture();
        v.escaleras.push((6, 6, 1));
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let dibujar_vista = |t: &mut Terminal<TestBackend>, v: &VistaEncuentro, corte| {
            t.draw(|f| {
                dibujar(
                    f,
                    v,
                    &recursos,
                    false,
                    false,
                    CamaraEncuentro {
                        corte,
                        manual: true,
                        centro: None,
                    },
                )
            })
            .unwrap();
        };
        dibujar_vista(&mut terminal, &v, 2);
        let cuerpo = terminal.backend().buffer().clone();
        let buscar = |b: &ratatui::buffer::Buffer, signo: &str| {
            b.content
                .iter()
                .enumerate()
                .find_map(|(i, c)| (i >= 160 && i < 880 && c.symbol() == signo).then_some(i))
                .unwrap_or_else(|| panic!("símbolo ausente: {signo}"))
        };
        let torso = buscar(&cuerpo, "A");
        let piernas = buscar(&cuerpo, "^");
        assert_eq!(piernas, torso + 80);
        assert_eq!(cuerpo.content[torso].bg, Color::Rgb(25, 35, 46));
        assert_eq!(cuerpo.content[piernas].bg, Color::Rgb(18, 26, 36));
        buscar(&cuerpo, "△");
        // Una columna detrás comparte píxel con los pies, no su casilla física.
        let sin_columna = v.clone();
        for z in 0..=2 {
            v.celdas[(z * 9 + 3) * 9 + 4] = TerrenoEncuentro::Muro;
        }
        dibujar_vista(&mut terminal, &v, 2);
        assert_eq!(terminal.backend().buffer().content[piernas].symbol(), "^");
        assert_eq!(
            terminal.backend().buffer().content[piernas].bg,
            Color::Reset
        );
        v = sin_columna;

        v.altura = 4;
        v.celdas.resize(9 * 9 * 4, TerrenoEncuentro::Aire);
        let pared = usize::from((3 * v.alto + 5) * v.ancho + 4);
        v.celdas[pared] = TerrenoEncuentro::Muro;
        dibujar_vista(&mut terminal, &v, 3);
        assert_eq!(
            terminal.backend().buffer().content[torso].fg,
            Color::Rgb(0, 0, 0)
        );
        assert_eq!(terminal.backend().buffer().content[torso].bg, Color::Cyan);
        v.celdas[pared] = TerrenoEncuentro::Desconocido;
        dibujar_vista(&mut terminal, &v, 3);
        assert_ne!(terminal.backend().buffer().content[torso].symbol(), "A");

        dibujar_vista(&mut terminal, &v, 1);
        assert_eq!(buscar(terminal.backend().buffer(), "^"), piernas);
        assert!(
            !terminal
                .backend()
                .buffer()
                .content
                .iter()
                .enumerate()
                .any(|(i, c)| i >= 160 && i < 880 && c.symbol() == "A")
        );
    }
    #[test]
    fn consecuencias_locales_muestra_persona_y_opciones_de_evacuacion() {
        use ratatui::{Terminal, backend::TestBackend};
        let recursos = RecursosVisuales::cargar(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap();
        let mut v = fixture();
        v.evacuacion_pendiente = vec![desert_tiger_app::ParticipanteEvacuacion {
            id: 1,
            nombre: "Nadine".into(),
            motivo: "incapacitado, sin evacuar".into(),
        }];
        v.pausa_decision = true;
        let antes = v.clone();
        let mut t = Terminal::new(TestBackend::new(100, 30)).unwrap();
        t.draw(|f| {
            dibujar(
                f,
                &v,
                &recursos,
                false,
                false,
                CamaraEncuentro {
                    corte: 2,
                    manual: false,
                    centro: None,
                },
            )
        })
        .unwrap();
        let texto = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect::<String>();
        assert!(texto.contains("Nadine"));
        assert!(texto.contains("Enter auxiliar"));
        assert!(texto.contains("A abandonar"));
        assert_eq!(v, antes);
    }
    #[test]
    fn corte_altura_dibuja_fichas_impactos_y_admite_resize() {
        let recursos = RecursosVisuales::cargar(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap();
        let mut v = fixture();
        let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
        t.draw(|f| {
            dibujar(
                f,
                &v,
                &recursos,
                true,
                false,
                CamaraEncuentro {
                    corte: 2,
                    manual: false,
                    centro: None,
                },
            )
        })
        .unwrap();
        let texto = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect::<String>();
        assert!(texto.contains("A*"));
        assert!(texto.contains("El grupo entra en combate."));
        assert!(texto.contains("herido"));
        v.fichas[0].impacto = Some(false);
        t.draw(|f| {
            dibujar(
                f,
                &v,
                &recursos,
                false,
                false,
                CamaraEncuentro {
                    corte: 2,
                    manual: false,
                    centro: None,
                },
            )
        })
        .unwrap();
        let texto = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect::<String>();
        assert!(texto.contains("A-"));
        v.celdas[4 * 9 + 3] = TerrenoEncuentro::Muro;
        for (escenario, nombre) in [
            (EscenarioEncuentro::Bosque, "bosque"),
            (EscenarioEncuentro::Montana, "montaña"),
            (EscenarioEncuentro::Playa, "playa"),
            (EscenarioEncuentro::Ciudad, "ciudad"),
            (EscenarioEncuentro::Palacio, "palacio"),
        ] {
            v.escenario = escenario;
            t.draw(|f| {
                dibujar(
                    f,
                    &v,
                    &recursos,
                    true,
                    false,
                    CamaraEncuentro {
                        corte: 2,
                        manual: false,
                        centro: None,
                    },
                )
            })
            .unwrap();
            let texto = t
                .backend()
                .buffer()
                .content
                .iter()
                .map(|c| c.symbol())
                .collect::<String>();
            assert!(texto.contains(nombre));
        }
        t.backend_mut().resize(120, 40);
        t.resize(Rect::new(0, 0, 120, 40)).unwrap();
        t.draw(|f| {
            dibujar(
                f,
                &v,
                &recursos,
                true,
                true,
                CamaraEncuentro {
                    corte: 2,
                    manual: false,
                    centro: None,
                },
            )
        })
        .unwrap();
        t.backend_mut().resize(12, 5);
        t.resize(Rect::new(0, 0, 12, 5)).unwrap();
        t.draw(|f| {
            dibujar(
                f,
                &v,
                &recursos,
                true,
                false,
                CamaraEncuentro {
                    corte: 2,
                    manual: false,
                    centro: None,
                },
            )
        })
        .unwrap();
    }
}
