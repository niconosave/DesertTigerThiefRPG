mod campamento;
mod interfaz;
mod recursos;

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, stdout};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use desert_tiger_app::{ConfiguracionPartida, Partida, VistaSesion, cargar_contenido};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use interfaz::Interfaz;
use recursos::RecursosVisuales;

const INTERVALO_ANIMACION: Duration = Duration::from_millis(250);

#[derive(Debug)]
pub struct ErrorTerminal(String);

impl Display for ErrorTerminal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for ErrorTerminal {}

pub fn ejecutar<I, S>(argumentos: I) -> Result<(), ErrorTerminal>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let contenido = analizar_argumentos(argumentos)?;
    let recursos = RecursosVisuales::cargar(Path::new("assets/terminal"))
        .map_err(|error| ErrorTerminal(format!("recursos visuales: {error}")))?;
    let contenido = cargar_contenido(&contenido).map_err(|error| {
        let detalle = error
            .diagnosticos()
            .first()
            .map(|d| d.detalle())
            .unwrap_or("sin diagnóstico");
        ErrorTerminal(format!("contenido: {detalle}"))
    })?;
    let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: None })
        .map_err(|error| ErrorTerminal(format!("partida: {error}")))?;
    if let VistaSesion::Disponible(mundo) = partida.vista()
        && let Some(faltante) = mundo
            .mapa
            .tipos_declarados
            .iter()
            .find(|clave| !recursos.contiene_tile(clave))
    {
        return Err(ErrorTerminal(format!(
            "recursos visuales: falta una apariencia oficial para {}",
            faltante.como_str()
        )));
    }
    ejecutar_interactivo(partida, recursos)
}

fn analizar_argumentos<I, S>(argumentos: I) -> Result<PathBuf, ErrorTerminal>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut iterador = argumentos.into_iter().map(Into::into);
    let _programa = iterador.next();
    let mut contenido = None;
    while let Some(argumento) = iterador.next() {
        if argumento != "--content" {
            return Err(ErrorTerminal(format!("opción desconocida: {argumento}")));
        }
        if contenido.is_some() {
            return Err(ErrorTerminal("--content no puede repetirse".to_owned()));
        }
        let ruta = iterador
            .next()
            .ok_or_else(|| ErrorTerminal("--content requiere una ruta".to_owned()))?;
        if ruta == "--content" {
            return Err(ErrorTerminal("--content requiere una ruta".to_owned()));
        }
        contenido = Some(PathBuf::from(ruta));
    }
    Ok(contenido.unwrap_or_else(|| PathBuf::from("content/base")))
}

fn ejecutar_interactivo(partida: Partida, recursos: RecursosVisuales) -> Result<(), ErrorTerminal> {
    let _guardia = GuardiaTerminal::entrar()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).map_err(error_io)?;
    terminal.clear().map_err(error_io)?;
    let mut interfaz = Interfaz::nueva(partida).map_err(ErrorTerminal)?;
    let mut ultimo_paso = Instant::now();
    loop {
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .map_err(error_io)?;
        if let Some(mensaje) = interfaz.diagnostico_fatal() {
            return Err(ErrorTerminal(format!("sesión: {mensaje}")));
        }
        if interfaz.debe_salir() {
            return Ok(());
        }
        let transcurrido = ultimo_paso.elapsed();
        let espera = INTERVALO_ANIMACION.saturating_sub(transcurrido);
        if event::poll(espera).map_err(error_io)? {
            match event::read().map_err(error_io)? {
                Event::Key(tecla) => interfaz.manejar_tecla(tecla),
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        let transcurrido = ultimo_paso.elapsed();
        let pasos = transcurrido.as_millis() / INTERVALO_ANIMACION.as_millis();
        if pasos > 0 {
            interfaz.avanzar_animacion(usize::try_from(pasos).unwrap_or(usize::MAX));
            ultimo_paso +=
                INTERVALO_ANIMACION.saturating_mul(u32::try_from(pasos).unwrap_or(u32::MAX));
        }
    }
}

struct GuardiaTerminal;

impl GuardiaTerminal {
    fn entrar() -> Result<Self, ErrorTerminal> {
        enable_raw_mode().map_err(error_io)?;
        if let Err(error) = execute!(stdout(), EnterAlternateScreen, Hide) {
            let _ = disable_raw_mode();
            return Err(error_io(error));
        }
        Ok(Self)
    }
}

impl Drop for GuardiaTerminal {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn error_io(error: io::Error) -> ErrorTerminal {
    ErrorTerminal(format!("terminal: {error}"))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use ratatui::backend::TestBackend;

    fn raiz_proyecto() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn argumentos_respetan_default_y_rechazan_ambiguedades() {
        assert_eq!(
            analizar_argumentos(["juego"]).expect("default válido"),
            PathBuf::from("content/base")
        );
        assert!(analizar_argumentos(["juego", "--content"]).is_err());
        assert!(
            analizar_argumentos([
                "juego",
                "--content",
                "content/base",
                "--content",
                "content/base"
            ])
            .is_err()
        );
    }

    #[test]
    fn render_minimo_reserva_mundo_y_dos_barras() {
        let raiz = raiz_proyecto();
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(3) })
            .expect("partida válida");
        let interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos válidos");
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("terminal de prueba");
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("dibujo válido");
        let buffer = terminal.backend().buffer();
        let contextual = (0..80)
            .map(|x| buffer[(x, 22)].symbol())
            .collect::<String>();
        let acciones = (0..80)
            .map(|x| buffer[(x, 23)].symbol())
            .collect::<String>();
        assert!(contextual.contains("Elige una acción"));
        assert!(acciones.contains("[V] Viajar"));
    }

    #[test]
    fn tamanos_menores_muestran_solo_el_aviso() {
        let raiz = raiz_proyecto();
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(3) })
            .expect("partida válida");
        let interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos válidos");
        let backend = TestBackend::new(79, 24);
        let mut terminal = Terminal::new(backend).expect("terminal de prueba");
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("dibujo válido");
        let primera = (0..79)
            .map(|x| terminal.backend().buffer()[(x, 0)].symbol())
            .collect::<String>();
        assert!(primera.contains("Terminal insuficiente"));
    }

    #[test]
    fn recorrido_humo_con_decision_es_reproducible_y_renderizable() {
        let raiz = raiz_proyecto();
        let contenido = cargar_contenido(&raiz.join("content/base")).expect("contenido válido");
        fn recorrer(contenido: &desert_tiger_app::ContenidoValidado) -> Vec<VistaSesion> {
            let mut partida = Partida::nueva(contenido, ConfiguracionPartida { semilla: Some(0) })
                .expect("partida válida");
            let inicial = partida.vista();
            let destino = match &inicial {
                VistaSesion::Disponible(mundo) => match &mundo.acciones[0] {
                    desert_tiger_app::AccionDisponible::Viajar { destinos } => destinos[0],
                    desert_tiger_app::AccionDisponible::Acampar => panic!("se esperaba Viajar"),
                },
                _ => panic!("vista inicial inesperada"),
            };
            let mut vistas = vec![inicial];
            let mut actual = partida
                .procesar(desert_tiger_app::IntencionFrontend::ComenzarViaje { destino })
                .expect("inicio aceptado");
            let mut hubo_decision = false;
            for _ in 0..12 {
                vistas.push(actual.clone());
                actual = match actual {
                    VistaSesion::PresentandoAccion(fragmento) => partida
                        .procesar(desert_tiger_app::IntencionFrontend::ContinuarPresentacion {
                            fragmento: fragmento.id,
                        })
                        .expect("continuación aceptada"),
                    VistaSesion::EsperandoDecision(decision) => {
                        hubo_decision = true;
                        partida
                            .procesar(desert_tiger_app::IntencionFrontend::ResponderDecision {
                                decision: decision.id,
                                opcion: decision.opciones[0].id,
                            })
                            .expect("decisión aceptada")
                    }
                    VistaSesion::Disponible(mundo) => {
                        assert!(mundo.resumen_npc_reciente.is_some());
                        assert_eq!(mundo.lugar_jugador, destino);
                        break;
                    }
                    VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                    VistaSesion::Fatal(fatal) => panic!("sesión fatal: {}", fatal.mensaje),
                };
            }
            assert!(hubo_decision, "la semilla 0 debe cubrir una decisión");
            vistas.push(partida.vista());
            vistas
        }

        let primera = recorrer(&contenido);
        let segunda = recorrer(&contenido);
        assert_eq!(primera, segunda);

        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(0) })
            .expect("partida válida");
        let interfaz = Interfaz::nueva(partida).expect("interfaz válida");
        let recursos =
            RecursosVisuales::cargar(&raiz.join("assets/terminal")).expect("recursos válidos");
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).expect("backend válido");
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .expect("el recorrido se puede presentar");
    }
}
