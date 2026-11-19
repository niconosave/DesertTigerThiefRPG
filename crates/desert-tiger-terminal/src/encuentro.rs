//! Reproducción local de estados ya resueltos; no contiene reglas de combate.
use crate::recursos::RecursosVisuales;
use desert_tiger_app::{ObjetivoAtaque, TerrenoEncuentro, VistaEncuentro};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Clear, Paragraph},
};
pub(crate) fn dibujar(
    frame: &mut Frame<'_>,
    v: &VistaEncuentro,
    r: &RecursosVisuales,
    pausa: bool,
    detalle: bool,
) {
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
            "ATAQUE · ronda {} · {}",
            v.ronda,
            if pausa { "pausa" } else { "en curso" }
        ))
        .style(r.ui.titulo),
        Rect::new(1, 0, a.width - 2, 1),
    );
    let alto = a.height.saturating_sub(9).min(v.alto).max(1);
    let ancho = (a.width.saturating_sub(2) / 2).min(v.ancho).max(1);
    let centro = v
        .fichas
        .iter()
        .find(|f| f.aliado && !f.salio && !f.transportado)
        .map(|f| (f.x, f.y))
        .unwrap_or((v.ancho / 2, v.alto / 2));
    let ox = centro
        .0
        .saturating_sub(ancho / 2)
        .min(v.ancho.saturating_sub(ancho));
    let oy = centro
        .1
        .saturating_sub(alto / 2)
        .min(v.alto.saturating_sub(alto));
    let izquierda = (a.width - ancho * 2) / 2;
    for y in 0..alto {
        for x in 0..ancho {
            let i = usize::from((y + oy) * v.ancho + x + ox);
            let (s, c) = match v.celdas[i] {
                TerrenoEncuentro::Desconocido => (" ", Color::Reset),
                TerrenoEncuentro::Suelo => (".", Color::DarkGray),
                TerrenoEncuentro::Muro => ("#", Color::Gray),
                TerrenoEncuentro::Vacio => ("~", Color::Blue),
            };
            if let Some(celda) = frame.buffer_mut().cell_mut((izquierda + x * 2, 2 + y)) {
                celda.set_symbol(s).set_style(Style::default().fg(c));
            }
        }
    }
    for (i, f) in v.fichas.iter().enumerate().rev() {
        if f.salio
            || f.transportado
            || f.x < ox
            || f.y < oy
            || f.x >= ox + ancho
            || f.y >= oy + alto
        {
            continue;
        }
        let glifo = if f.golpes >= 3 {
            '%'
        } else if f.golpes == 2 {
            'x'
        } else if f.aliado {
            (b'A' + (i % 26) as u8) as char
        } else {
            'E'
        };
        if let Some(celda) = frame
            .buffer_mut()
            .cell_mut((izquierda + (f.x - ox) * 2, 2 + f.y - oy))
        {
            celda.set_char(glifo).set_style(
                Style::default()
                    .fg(if f.golpes > 0 {
                        Color::LightRed
                    } else if f.aliado {
                        Color::Cyan
                    } else {
                        Color::Yellow
                    })
                    .add_modifier(Modifier::BOLD),
            );
        }
    }
    // Efecto breve junto a la ficha: acierto (*) o fallo (-).
    for f in &v.fichas {
        if let Some(acierto) = f.impacto
            && !f.salio
            && !f.transportado
            && f.x >= ox
            && f.x < ox + ancho
            && f.y >= oy
            && f.y < oy + alto
            && let Some(c) = frame
                .buffer_mut()
                .cell_mut((izquierda + (f.x - ox) * 2 + 1, 2 + f.y - oy))
        {
            c.set_symbol(if acierto { "*" } else { "-" })
                .set_fg(if acierto {
                    Color::LightRed
                } else {
                    Color::White
                });
        }
    }
    let pie = 2 + alto;
    frame.render_widget(
        Paragraph::new(v.texto.as_str())
            .wrap(ratatui::widgets::Wrap { trim: true })
            .style(r.ui.texto_normal),
        Rect::new(1, pie, a.width - 2, 3.min(a.height.saturating_sub(pie))),
    );
    let roster = v
        .fichas
        .iter()
        .enumerate()
        .filter(|(_, f)| f.aliado)
        .map(|(i, f)| {
            format!(
                "{} {}{}",
                (b'A' + (i % 26) as u8) as char,
                f.nombre,
                if f.golpes >= 3 {
                    " (muerto)"
                } else if f.golpes == 2 {
                    " (incapacitado)"
                } else if f.golpes == 1 {
                    " (herido)"
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
    let controles = if v.finalizado {
        "Enter: cerrar episodio · D: detalles · Q: salir"
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
    opciones: &[ObjetivoAtaque],
    foco: usize,
    r: &RecursosVisuales,
) {
    let a = frame.area();
    let w = a.width.min(48);
    let h = (opciones.len() as u16 + 3).min(a.height);
    let area = Rect::new(a.width.saturating_sub(w), 0, w, h);
    frame.render_widget(Clear, area);
    let texto = std::iter::once("Elegí a quién atacar (Esc cancela)".into())
        .chain(
            opciones
                .iter()
                .enumerate()
                .map(|(i, o)| format!("{} {}", if i == foco { ">" } else { " " }, o.nombre)),
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
    #[test]
    fn encuentro_terminal_dibuja_fichas_impactos_y_admite_resize() {
        let recursos = RecursosVisuales::cargar(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
        )
        .unwrap();
        let mut v = VistaEncuentro {
            id: 1,
            paso: 0,
            ancho: 9,
            alto: 9,
            celdas: vec![TerrenoEncuentro::Suelo; 81],
            fichas: vec![FichaEncuentro {
                id: 0,
                impacto: Some(true),
                nombre: "Prueba".into(),
                x: 4,
                y: 4,
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
            retirando: false,
        };
        let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
        t.draw(|f| dibujar(f, &v, &recursos, true, false)).unwrap();
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
        t.draw(|f| dibujar(f, &v, &recursos, false, false)).unwrap();
        let texto = t
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect::<String>();
        assert!(texto.contains("A-"));
        t.backend_mut().resize(120, 40);
        t.resize(Rect::new(0, 0, 120, 40)).unwrap();
        t.draw(|f| dibujar(f, &v, &recursos, true, true)).unwrap();
        t.backend_mut().resize(12, 5);
        t.resize(Rect::new(0, 0, 12, 5)).unwrap();
        t.draw(|f| dibujar(f, &v, &recursos, true, false)).unwrap();
    }
}
