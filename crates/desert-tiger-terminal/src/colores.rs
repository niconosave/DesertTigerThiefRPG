use ratatui::{buffer::Buffer, style::Color};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum EsquemaColores {
    #[default]
    Terminal,
    Fijo,
}

const FIJOS: [[u8; 3]; 16] = [
    [0, 0, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 128, 0],
    [0, 0, 128],
    [128, 0, 128],
    [0, 128, 128],
    [192, 192, 192],
    [128, 128, 128],
    [255, 0, 0],
    [0, 255, 0],
    [255, 255, 0],
    [0, 0, 255],
    [255, 0, 255],
    [0, 255, 255],
    [255, 255, 255],
];

#[derive(Clone, Debug, Default)]
pub(crate) struct PaletaColores {
    esquema: EsquemaColores,
    consultados: [Option<[u8; 3]>; 16],
    fondo_negro: bool,
}

impl PaletaColores {
    pub(crate) fn fija() -> Self {
        Self {
            esquema: EsquemaColores::Fijo,
            consultados: FIJOS.map(Some),
            fondo_negro: false,
        }
    }

    pub(crate) fn registrar(&mut self, indice: usize, rgb: [u8; 3]) {
        if self.esquema == EsquemaColores::Terminal
            && let Some(canal) = self.consultados.get_mut(indice)
        {
            *canal = Some(rgb);
        }
    }

    pub(crate) fn completa(&self) -> bool {
        self.consultados.iter().all(Option::is_some)
    }

    pub(crate) fn forzar_fondo_negro(&mut self, forzar: bool) {
        self.fondo_negro = forzar;
    }

    pub(crate) fn fondo_pantalla(&self, configurado: Option<Color>) -> Color {
        if self.fondo_negro {
            Color::Rgb(0, 0, 0)
        } else {
            configurado.unwrap_or(Color::Reset)
        }
    }

    pub(crate) fn para_iluminar(&self, color: Color) -> Option<[u8; 3]> {
        match color {
            Color::Rgb(r, g, b) => Some([r, g, b]),
            Color::Reset => None,
            Color::Indexed(i) if i >= 16 => {
                (self.esquema == EsquemaColores::Fijo).then(|| indexado(i))
            }
            _ => indice(color).and_then(|i| self.consultados[i]),
        }
    }

    pub(crate) fn aplicar(&self, buffer: &mut Buffer) {
        if self.esquema != EsquemaColores::Fijo && !self.fondo_negro {
            return;
        }
        for celda in &mut buffer.content {
            if self.esquema == EsquemaColores::Fijo {
                celda.fg = self.fijar(celda.fg, FIJOS[7]);
                celda.bg = self.fijar(celda.bg, FIJOS[0]);
            } else if self.fondo_negro && celda.bg == Color::Reset {
                celda.bg = Color::Rgb(0, 0, 0);
            }
        }
    }

    fn fijar(&self, color: Color, reserva: [u8; 3]) -> Color {
        let [r, g, b] = self.para_iluminar(color).unwrap_or(reserva);
        Color::Rgb(r, g, b)
    }
}

fn indice(color: Color) -> Option<usize> {
    Some(match color {
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Magenta => 5,
        Color::Cyan => 6,
        Color::Gray => 7,
        Color::DarkGray => 8,
        Color::LightRed => 9,
        Color::LightGreen => 10,
        Color::LightYellow => 11,
        Color::LightBlue => 12,
        Color::LightMagenta => 13,
        Color::LightCyan => 14,
        Color::White => 15,
        Color::Indexed(i) if i < 16 => usize::from(i),
        _ => return None,
    })
}

fn indexado(i: u8) -> [u8; 3] {
    if i >= 232 {
        return [8 + (i - 232) * 10; 3];
    }
    let i = usize::from(i - 16);
    let escalas = [0, 95, 135, 175, 215, 255];
    [escalas[i / 36], escalas[i / 6 % 6], escalas[i % 6]]
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use ratatui::{
        layout::Rect,
        style::{Modifier, Style},
    };

    #[test]
    fn modo_terminal_conserva_el_buffer_y_solo_resuelve_colores_conocidos() {
        let mut paleta = PaletaColores::default();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        buffer[(0, 0)].set_char('@').set_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
        let antes = buffer.clone();
        assert_eq!(paleta.para_iluminar(Color::Magenta), None);
        paleta.registrar(5, [180, 120, 160]);
        assert_eq!(paleta.para_iluminar(Color::Magenta), Some([180, 120, 160]));
        assert_eq!(paleta.para_iluminar(Color::Blue), None);
        assert_eq!(paleta.para_iluminar(Color::Rgb(0, 0, 0)), Some([0, 0, 0]));
        paleta.aplicar(&mut buffer);
        assert_eq!(buffer, antes);
    }

    #[test]
    fn modo_fijo_resuelve_todo_sin_cambiar_simbolos_ni_rgb_explicito() {
        let paleta = PaletaColores::fija();
        assert!(paleta.completa());
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 1));
        buffer[(0, 0)]
            .set_char('@')
            .set_fg(Color::LightMagenta)
            .set_bg(Color::Yellow);
        buffer[(1, 0)]
            .set_char('x')
            .set_fg(Color::Rgb(20, 30, 40))
            .set_bg(Color::Rgb(0, 0, 0));
        paleta.aplicar(&mut buffer);
        assert_eq!(buffer[(0, 0)].fg, Color::Rgb(255, 0, 255));
        assert_eq!(buffer[(0, 0)].bg, Color::Rgb(128, 128, 0));
        assert_eq!(buffer[(0, 0)].symbol(), "@");
        assert_eq!(buffer[(1, 0)].fg, Color::Rgb(20, 30, 40));
        assert_eq!(buffer[(1, 0)].bg, Color::Rgb(0, 0, 0));
        assert_eq!(paleta.para_iluminar(Color::Indexed(196)), Some([255, 0, 0]));
        assert_eq!(
            paleta.para_iluminar(Color::Indexed(255)),
            Some([238, 238, 238])
        );
    }
    #[test]
    fn forzar_negro_es_independiente_de_los_colores_de_frente_y_fondos_pintados() {
        let mut p = PaletaColores::default();
        assert_eq!(p.fondo_pantalla(None), Color::Reset);
        p.forzar_fondo_negro(true);
        assert_eq!(p.fondo_pantalla(Some(Color::Blue)), Color::Rgb(0, 0, 0));
        let mut b = Buffer::empty(Rect::new(0, 0, 3, 1));
        b[(0, 0)].set_char('@').set_fg(Color::Magenta);
        b[(1, 0)]
            .set_char('~')
            .set_fg(Color::Cyan)
            .set_bg(Color::Blue);
        b[(2, 0)]
            .set_char('o')
            .set_fg(Color::Yellow)
            .set_bg(Color::Rgb(0, 0, 0));
        p.aplicar(&mut b);
        assert_eq!(b[(0, 0)].fg, Color::Magenta);
        assert_eq!(b[(0, 0)].bg, Color::Rgb(0, 0, 0));
        assert_eq!(b[(1, 0)].fg, Color::Cyan);
        assert_eq!(b[(1, 0)].bg, Color::Blue);
        assert_eq!(b[(2, 0)].bg, Color::Rgb(0, 0, 0));
    }
}
