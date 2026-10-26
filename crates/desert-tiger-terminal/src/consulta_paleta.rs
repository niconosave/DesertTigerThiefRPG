use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::colores::{EsquemaColores, PaletaColores};

const PLAZO_CONSULTA: Duration = Duration::from_millis(300);
const PLAZO_FRAGMENTO: Duration = Duration::from_millis(100);
const PREFIJO: &[u8] = b"\x1b]4;";

#[derive(Default)]
struct Filtro {
    bytes: Vec<u8>,
    originales: Vec<Event>,
    ultima: Option<Instant>,
    respuesta: bool,
    excedida: bool,
}

impl Filtro {
    fn limpiar(&mut self) {
        self.bytes.clear();
        self.originales.clear();
        self.ultima = None;
        self.respuesta = false;
        self.excedida = false;
    }

    fn expirar(&mut self, ahora: Instant, cola: &mut VecDeque<Event>) {
        if self
            .ultima
            .is_some_and(|ultima| ahora.duration_since(ultima) >= PLAZO_FRAGMENTO)
        {
            if !self.respuesta {
                cola.extend(self.originales.drain(..));
            }
            self.limpiar();
        }
    }

    fn espera(&self, ahora: Instant) -> Option<Duration> {
        self.ultima
            .map(|ultima| PLAZO_FRAGMENTO.saturating_sub(ahora.duration_since(ultima)))
    }

    fn recibir(
        &mut self,
        evento: Event,
        ahora: Instant,
        cola: &mut VecDeque<Event>,
    ) -> Option<(usize, [u8; 3])> {
        self.expirar(ahora, cola);
        let token = bytes_evento(&evento);
        let Some(token) = token else {
            if !self.respuesta {
                cola.extend(self.originales.drain(..));
                self.limpiar();
            }
            cola.push_back(evento);
            return None;
        };
        if self.bytes.is_empty() && !self.respuesta && token[0] != 0x1b {
            cola.push_back(evento);
            return None;
        }
        self.ultima = Some(ahora);
        if !self.respuesta {
            self.originales.push(evento);
        }
        if self.respuesta && self.bytes.len() + token.len() > 256 {
            let esc = self.bytes.last() == Some(&0x1b);
            self.bytes.clear();
            if esc {
                self.bytes.push(0x1b);
            }
            self.excedida = true;
        }
        self.bytes.extend_from_slice(&token);
        if !self.respuesta {
            if self.bytes.starts_with(PREFIJO) {
                self.respuesta = true;
                self.originales.clear();
            } else if !PREFIJO.starts_with(&self.bytes) {
                cola.extend(self.originales.drain(..));
                self.limpiar();
                return None;
            }
        }
        if self.respuesta {
            let termina = self.bytes.ends_with(b"\x07") || self.bytes.ends_with(b"\x1b\\");
            if termina {
                let resultado = if self.excedida {
                    None
                } else {
                    analizar_respuesta(&self.bytes)
                };
                self.limpiar();
                return resultado;
            }
        }
        None
    }
}

// Crossterm 0.29 decodifica OSC como Alt+] y caracteres de teclado.
// Este adaptador consume únicamente respuestas OSC 4, preservando los eventos
// originales cuando la secuencia resulta ser entrada del usuario.
fn bytes_evento(evento: &Event) -> Option<Vec<u8>> {
    let Event::Key(tecla) = evento else {
        return None;
    };
    if tecla.kind != KeyEventKind::Press {
        return None;
    }
    match (tecla.code, tecla.modifiers) {
        (KeyCode::Esc, m) if m.is_empty() => Some(vec![0x1b]),
        (KeyCode::Char(']'), m) if m == KeyModifiers::ALT => Some(b"\x1b]".to_vec()),
        (KeyCode::Char('\\'), m) if m == KeyModifiers::ALT => Some(b"\x1b\\".to_vec()),
        (KeyCode::Char('g'), m) if m == KeyModifiers::CONTROL => Some(vec![7]),
        (KeyCode::Char(c), m) if c.is_ascii() && (m.is_empty() || m == KeyModifiers::SHIFT) => {
            Some(vec![c as u8])
        }
        _ => None,
    }
}

fn analizar_respuesta(bytes: &[u8]) -> Option<(usize, [u8; 3])> {
    let cuerpo = bytes.strip_prefix(PREFIJO)?;
    let cuerpo = cuerpo
        .strip_suffix(b"\x07")
        .or_else(|| cuerpo.strip_suffix(b"\x1b\\"))?;
    let cuerpo = std::str::from_utf8(cuerpo).ok()?;
    let (indice, color) = cuerpo.split_once(';')?;
    if indice.is_empty() || !indice.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let indice: usize = indice.parse().ok()?;
    if indice >= 16 {
        return None;
    }
    let mut canales = color.strip_prefix("rgb:")?.split('/');
    let mut rgb = [0; 3];
    for canal in &mut rgb {
        let texto = canales.next()?;
        if texto.is_empty() || texto.len() > 4 || !texto.bytes().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let valor = u32::from_str_radix(texto, 16).ok()?;
        let maximo = (1u32 << (texto.len() * 4)) - 1;
        *canal = ((valor * 255 + maximo / 2) / maximo) as u8;
    }
    if canales.next().is_some() {
        return None;
    }
    Some((indice, rgb))
}

#[derive(Default)]
pub(crate) struct EntradaTerminal {
    filtro: Filtro,
    cola: VecDeque<Event>,
}

impl EntradaTerminal {
    pub(crate) fn consultar(esquema: EsquemaColores) -> (PaletaColores, Self) {
        if esquema == EsquemaColores::Fijo {
            return (PaletaColores::fija(), Self::default());
        }
        let mut paleta = PaletaColores::default();
        let mut entrada = Self::default();
        if std::env::var("TERM").is_ok_and(|t| t == "dumb") {
            return (paleta, entrada);
        }
        let mut salida = io::stdout().lock();
        for i in 0..16 {
            if write!(salida, "\x1b]4;{i};?\x1b\\").is_err() {
                return (paleta, entrada);
            }
        }
        if salida.flush().is_err() {
            return (paleta, entrada);
        }
        drop(salida);
        let inicio = Instant::now();
        while !paleta.completa() {
            let restante = PLAZO_CONSULTA.saturating_sub(inicio.elapsed());
            if restante.is_zero() {
                break;
            }
            match event::poll(restante) {
                Ok(true) => match event::read() {
                    Ok(e) => {
                        if let Some((i, rgb)) =
                            entrada.filtro.recibir(e, Instant::now(), &mut entrada.cola)
                        {
                            paleta.registrar(i, rgb);
                        }
                    }
                    Err(_) => break,
                },
                Ok(false) | Err(_) => break,
            }
        }
        (paleta, entrada)
    }

    pub(crate) fn leer(&mut self, espera: Duration) -> io::Result<Option<Event>> {
        let inicio = Instant::now();
        loop {
            let ahora = Instant::now();
            self.filtro.expirar(ahora, &mut self.cola);
            if let Some(evento) = self.cola.pop_front() {
                return Ok(Some(evento));
            }
            let restante = espera.saturating_sub(inicio.elapsed());
            if restante.is_zero() {
                return Ok(None);
            }
            let plazo = self
                .filtro
                .espera(ahora)
                .map_or(restante, |p| p.min(restante));
            if event::poll(plazo)? {
                // Los informes tardíos se consumen sin cambiar la instantánea
                // de paleta obtenida durante el inicio.
                self.filtro
                    .recibir(event::read()?, Instant::now(), &mut self.cola);
            }
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crossterm::event::KeyEvent;

    fn tecla(c: char) -> Event {
        Event::Key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE))
    }
    fn eventos(texto: &str, separados: bool) -> Vec<Event> {
        let mut eventos = Vec::new();
        let mut chars = texto.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' && !separados && chars.peek().is_some() {
                eventos.push(Event::Key(KeyEvent::new(
                    KeyCode::Char(chars.next().unwrap()),
                    KeyModifiers::ALT,
                )));
            } else if c == '\x1b' {
                eventos.push(Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)));
            } else if c == '\x07' {
                eventos.push(Event::Key(KeyEvent::new(
                    KeyCode::Char('g'),
                    KeyModifiers::CONTROL,
                )));
            } else {
                eventos.push(tecla(c));
            }
        }
        eventos
    }

    #[test]
    fn rgb_admite_anchos_hex_escala_y_rechaza_respuestas_invalidas() {
        for (texto, esperado) in [
            ("\x1b]4;13;rgb:b/8/a\x07", (13, [187, 136, 170])),
            ("\x1b]4;4;rgb:12/34/56\x1b\\", (4, [18, 52, 86])),
            ("\x1b]4;5;rgb:800/000/fff\x07", (5, [128, 0, 255])),
            ("\x1b]4;0;rgb:ffff/8080/0000\x07", (0, [255, 128, 0])),
        ] {
            assert_eq!(analizar_respuesta(texto.as_bytes()), Some(esperado));
        }
        for texto in [
            "\x1b]4;16;rgb:ff/ff/ff\x07",
            "\x1b]4;-1;rgb:ff/ff/ff\x07",
            "\x1b]4;5;rgb:fffff/00/00\x07",
            "\x1b]4;5;rgb:xx/00/00\x07",
            "\x1b]4;5;rgb:/00/00\x07",
            "\x1b]4;5;rgb:00/00\x07",
            "\x1b]4;5;rgb:00/00/00/00\x07",
            "\x1b]4;5;rgb:00/00/00",
        ] {
            assert_eq!(analizar_respuesta(texto.as_bytes()), None, "{texto:?}");
        }
    }

    #[test]
    fn fragmentos_y_teclas_se_conservan_en_orden_sin_exponer_respuestas() {
        for separados in [false, true] {
            let mut filtro = Filtro::default();
            let mut cola = VecDeque::new();
            let ahora = Instant::now();
            filtro.recibir(tecla('a'), ahora, &mut cola);
            let mut encontrados = Vec::new();
            for (paso, e) in eventos("\x1b]4;13;rgb:bbbb/8888/aaaa\x1b\\", separados)
                .into_iter()
                .enumerate()
            {
                if let Some(rgb) =
                    filtro.recibir(e, ahora + Duration::from_millis(paso as u64), &mut cola)
                {
                    encontrados.push(rgb);
                }
            }
            filtro.recibir(tecla('v'), ahora + Duration::from_millis(50), &mut cola);
            assert_eq!(encontrados, vec![(13, [187, 136, 170])]);
            assert_eq!(cola, VecDeque::from([tecla('a'), tecla('v')]));
        }
    }

    #[test]
    fn expiracion_restituye_escape_y_descarta_respuestas_truncadas_o_excesivas() {
        let ahora = Instant::now();
        let mut filtro = Filtro::default();
        let mut cola = VecDeque::new();
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        filtro.recibir(esc.clone(), ahora, &mut cola);
        filtro.expirar(ahora + PLAZO_FRAGMENTO, &mut cola);
        assert_eq!(cola.pop_front(), Some(esc));
        let alt = Event::Key(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::ALT));
        filtro.recibir(alt.clone(), ahora, &mut cola);
        filtro.recibir(tecla('x'), ahora, &mut cola);
        assert_eq!(cola.drain(..).collect::<Vec<_>>(), vec![alt, tecla('x')]);
        for e in eventos("\x1b]4;5;rgb:ab", false) {
            filtro.recibir(e, ahora, &mut cola);
        }
        filtro.expirar(ahora + PLAZO_FRAGMENTO, &mut cola);
        assert!(cola.is_empty());
        let largo = format!("\x1b]4;5;rgb:{}\x07", "a".repeat(700));
        for e in eventos(&largo, false) {
            assert!(filtro.recibir(e, ahora, &mut cola).is_none());
            assert!(filtro.bytes.len() <= 256);
        }
        filtro.recibir(tecla('q'), ahora, &mut cola);
        assert_eq!(cola, VecDeque::from([tecla('q')]));
    }
}
