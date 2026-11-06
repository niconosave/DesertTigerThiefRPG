#[cfg(test)]
use desert_tiger_app::VistaSesion;
mod animacion;
mod arte_lugares;
mod audio;
mod barcos;
mod campamento;
mod caravanas;
mod colores;
mod consulta_paleta;
mod desarrollo;
mod dibujo_caracteres;
mod fogata;
mod graficos;
mod interfaz;
mod musica_campamento;
mod planificacion_rutas;
mod recursos;
mod trafico_decorativo;
mod trazado;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, stdout};
use std::path::{Path, PathBuf};
use std::time::Instant;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use desert_tiger_app::{ConfiguracionPartida, IdActor, Partida, RetratoActor, cargar_contenido};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use interfaz::Interfaz;
use recursos::RecursosVisuales;

use animacion::PASO_ANIMACION;
use colores::EsquemaColores;
use consulta_paleta::EntradaTerminal;
use graficos::ModoGraficos;

#[derive(Debug)]
pub struct ErrorTerminal(String);

impl Display for ErrorTerminal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for ErrorTerminal {}

#[cfg(feature = "audio")]
pub fn previsualizar_melodia(entrada: &Path, salida: &Path) -> Result<f64, ErrorTerminal> {
    audio::previsualizar(entrada, salida).map_err(ErrorTerminal)
}

pub fn ejecutar<I, S>(argumentos: I) -> Result<(), ErrorTerminal>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let opciones = analizar_argumentos(argumentos)?;
    let raiz_visual = Path::new("assets/terminal");
    let recursos = RecursosVisuales::cargar_modo(raiz_visual, opciones.graficos)
        .map_err(|error| ErrorTerminal(format!("recursos visuales: {error}")))?;
    let contenido = cargar_contenido(&opciones.contenido).map_err(|error| {
        let detalle = error
            .diagnosticos()
            .first()
            .map(|d| d.detalle())
            .unwrap_or("sin diagnóstico");
        ErrorTerminal(format!("contenido: {detalle}"))
    })?;
    let retratos = graficos::cargar_retratos(raiz_visual, opciones.graficos, &contenido)
        .map_err(|error| ErrorTerminal(format!("retratos alternativos: {error}")))?;
    let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: None })
        .map_err(|error| ErrorTerminal(format!("partida: {error}")))?;
    if let Some(faltante) = partida
        .mundo_visible()
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
    let audio = audio::preparar(!opciones.sin_audio, Path::new("assets/audio"))
        .map_err(|error| ErrorTerminal(format!("recursos sonoros: {error}")))?;
    ejecutar_interactivo(
        partida,
        recursos,
        retratos,
        opciones.esquema,
        opciones.fondo_negro,
        audio,
    )
}

#[derive(Debug, PartialEq, Eq)]
struct Opciones {
    contenido: PathBuf,
    esquema: EsquemaColores,
    graficos: ModoGraficos,
    fondo_negro: bool,
    sin_audio: bool,
}

fn analizar_argumentos<I, S>(argumentos: I) -> Result<Opciones, ErrorTerminal>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut iterador = argumentos.into_iter().map(Into::into);
    let _programa = iterador.next();
    let mut contenido = None;
    let mut esquema = None;
    let mut graficos = None;
    let mut fondo_negro = false;
    let mut sin_audio = false;
    while let Some(argumento) = iterador.next() {
        if argumento == "--no-audio" {
            if sin_audio {
                return Err(ErrorTerminal("--no-audio no puede repetirse".into()));
            }
            sin_audio = true;
            continue;
        }
        if argumento == "-B" {
            if fondo_negro {
                return Err(ErrorTerminal("-B no puede repetirse".into()));
            }
            fondo_negro = true;
            continue;
        }
        if !matches!(
            argumento.as_str(),
            "--content" | "--color-scheme" | "--graphics"
        ) {
            return Err(ErrorTerminal(format!("opción desconocida: {argumento}")));
        }
        if (argumento == "--content" && contenido.is_some())
            || (argumento == "--color-scheme" && esquema.is_some())
            || (argumento == "--graphics" && graficos.is_some())
        {
            return Err(ErrorTerminal(format!("{argumento} no puede repetirse")));
        }
        let valor = iterador
            .next()
            .filter(|s| !s.starts_with("--") && s != "-B")
            .ok_or_else(|| ErrorTerminal(format!("{argumento} requiere un valor")))?;
        if argumento == "--content" {
            contenido = Some(PathBuf::from(valor));
        } else if argumento == "--graphics" {
            graficos = Some(match valor.as_str() {
                "default" => ModoGraficos::Predeterminado,
                "custom-font" => ModoGraficos::Fuente,
                "ascii" => ModoGraficos::Ascii,
                _ => {
                    return Err(ErrorTerminal(format!(
                        "--graphics: valor desconocido {valor}; se espera default, custom-font o ascii"
                    )));
                }
            });
        } else {
            esquema = Some(match valor.as_str() {
                "terminal" => EsquemaColores::Terminal,
                "fixed" => EsquemaColores::Fijo,
                _ => {
                    return Err(ErrorTerminal(format!(
                        "--color-scheme: valor desconocido {valor}; se espera terminal o fixed"
                    )));
                }
            });
        }
    }
    Ok(Opciones {
        contenido: contenido.unwrap_or_else(|| PathBuf::from("content/base")),
        esquema: esquema.unwrap_or_default(),
        graficos: graficos.unwrap_or_default(),
        fondo_negro,
        sin_audio,
    })
}

fn ejecutar_interactivo(
    partida: Partida,
    mut recursos: RecursosVisuales,
    retratos: HashMap<IdActor, RetratoActor>,
    esquema: EsquemaColores,
    fondo_negro: bool,
    preparacion_audio: audio::PreparacionAudio,
) -> Result<(), ErrorTerminal> {
    let mut guardia = GuardiaTerminal::entrar()?;
    let (mut paleta, mut entrada) = EntradaTerminal::consultar(esquema);
    paleta.forzar_fondo_negro(fondo_negro);
    recursos.paleta = paleta;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).map_err(error_io)?;
    terminal.clear().map_err(error_io)?;
    let mut interfaz = Interfaz::nueva(partida)
        .map_err(ErrorTerminal)?
        .con_retratos(retratos);
    // El mapa ya es visible cuando se abre audio y empieza la síntesis diferida.
    terminal
        .draw(|frame| interfaz.dibujar(frame, &recursos))
        .map_err(error_io)?;
    let mut audio = audio::ControlAudio::iniciar(preparacion_audio);
    let inicio_musica = Instant::now();
    let mut ultimo_paso = Instant::now();
    loop {
        guardia.actualizar_mouse(interfaz.capturar_mouse())?;
        sincronizar_audio(&mut interfaz, &mut audio, inicio_musica.elapsed());
        terminal
            .draw(|frame| interfaz.dibujar(frame, &recursos))
            .map_err(error_io)?;
        if let Some(mensaje) = interfaz.diagnostico_fatal() {
            return Err(ErrorTerminal(format!("sesión: {mensaje}")));
        }
        if interfaz.debe_salir() {
            return Ok(());
        }
        // Contamos también el tiempo de composición antes de esperar.
        let ahora = Instant::now();
        interfaz.avanzar_animacion(ahora.duration_since(ultimo_paso));
        ultimo_paso = ahora;
        let evento = entrada
            .leer(interfaz.espera_animacion(&recursos))
            .map_err(error_io)?;
        // Se aplica el estado de pausa anterior al evento, incluso si no llegó
        // entrada. Al cancelar Q no recuperamos el tiempo del modal.
        let ahora = Instant::now();
        interfaz.avanzar_animacion(ahora.duration_since(ultimo_paso));
        ultimo_paso = ahora;
        if let Some(evento) = evento {
            match evento {
                Event::Key(tecla) => interfaz.manejar_tecla(tecla),
                Event::Mouse(mouse) => interfaz.manejar_mouse(mouse),
                Event::Resize(_, _) => interfaz.invalidar_geometria_mouse(),
                _ => {}
            }
        }
        sincronizar_audio(&mut interfaz, &mut audio, inicio_musica.elapsed());
    }
}

fn sincronizar_audio(
    interfaz: &mut Interfaz,
    audio: &mut audio::ControlAudio,
    ahora: std::time::Duration,
) {
    audio.actualizar(interfaz.ambiente_sonoro());
    interfaz.actualizar_musica(
        ahora,
        audio.estado_musica(),
        audio.disponible(),
        audio.ajustes_musica(),
    );
    audio.solicitar_musica(interfaz.solicitud_musica());
    if let Some(aviso) = audio.tomar_aviso() {
        interfaz.recibir_aviso_audio(aviso);
    }
}

struct GuardiaTerminal {
    captura_mouse: bool,
}

impl GuardiaTerminal {
    fn entrar() -> Result<Self, ErrorTerminal> {
        enable_raw_mode().map_err(error_io)?;
        if let Err(error) = execute!(stdout(), EnterAlternateScreen, Hide) {
            let _ = disable_raw_mode();
            return Err(error_io(error));
        }
        Ok(Self {
            captura_mouse: false,
        })
    }

    fn actualizar_mouse(&mut self, activar: bool) -> Result<(), ErrorTerminal> {
        if activar != self.captura_mouse {
            if activar {
                execute!(stdout(), EnableMouseCapture).map_err(error_io)?;
            } else {
                execute!(stdout(), DisableMouseCapture).map_err(error_io)?;
            }
            self.captura_mouse = activar;
        }
        Ok(())
    }
}

impl Drop for GuardiaTerminal {
    fn drop(&mut self) {
        let _ = execute!(stdout(), DisableMouseCapture, Show, LeaveAlternateScreen);
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
            Opciones {
                contenido: PathBuf::from("content/base"),
                esquema: EsquemaColores::Terminal,
                graficos: ModoGraficos::Predeterminado,
                fondo_negro: false,
                sin_audio: false,
            }
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
    fn opcion_sin_audio_se_combina_y_rechaza_duplicados_o_valores() {
        let opciones = analizar_argumentos([
            "juego",
            "--no-audio",
            "-B",
            "--content",
            "mundo",
            "--color-scheme",
            "fixed",
        ])
        .unwrap();
        assert!(opciones.sin_audio && opciones.fondo_negro);
        assert_eq!(opciones.contenido, PathBuf::from("mundo"));
        assert_eq!(opciones.esquema, EsquemaColores::Fijo);
        assert!(
            analizar_argumentos(["juego", "--color-scheme", "terminal", "--no-audio"])
                .unwrap()
                .sin_audio
        );
        for args in [
            vec!["juego", "--no-audio", "--no-audio"],
            vec!["juego", "--no-audio", "true"],
            vec!["juego", "--content", "--no-audio"],
        ] {
            assert!(analizar_argumentos(args).is_err());
        }
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
                // Un destino no adyacente siempre ofrece la elección de planificación.
                // Elegir el primer destino del atlas dependía de incidentes aleatorios.
                VistaSesion::Disponible(mundo) => {
                    mundo
                        .lugares
                        .iter()
                        .find(|lugar| lugar.nombre == "Paso Escarpado")
                        .expect("el contenido oficial incluye el territorio conocido")
                        .id
                }
                _ => panic!("vista inicial inesperada"),
            };
            let mut vistas = vec![inicial];
            let mut actual = partida
                .procesar(desert_tiger_app::IntencionFrontend::ComenzarViaje { destino })
                .expect("inicio aceptado");
            let mut hubo_decision = false;
            let mut termino = false;
            for _ in 0..128 {
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
                        // Un incidente puede terminar el viaje antes del destino.
                        // La vista final debe conservar el último lugar confirmado.
                        let confirmado = vistas
                            .iter()
                            .rev()
                            .find_map(|vista| match vista {
                                VistaSesion::PresentandoAccion(f) => {
                                    Some(f.progreso.lugar_confirmado)
                                }
                                VistaSesion::EsperandoDecision(d) => {
                                    Some(d.progreso.lugar_confirmado)
                                }
                                _ => None,
                            })
                            .expect("el viaje presentó su progreso");
                        assert_eq!(mundo.lugar_jugador, confirmado);
                        termino = true;
                        break;
                    }
                    VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                    VistaSesion::PresentandoEvento(_) | VistaSesion::EsperandoDecisionEvento(_) => {
                        panic!("catálogo vacío inesperado")
                    }
                    VistaSesion::Fatal(fatal) => panic!("sesión fatal: {}", fatal.mensaje),
                };
            }
            assert!(
                hubo_decision,
                "el recorrido conocido debe ofrecer una decisión"
            );
            assert!(termino, "el recorrido de humo debe terminar");
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
    #[test]
    fn argumentos_colores_validan_valores_duplicados_y_combinacion_con_contenido() {
        let opciones =
            analizar_argumentos(["juego", "--color-scheme", "fixed", "--content", "mundo"])
                .unwrap();
        assert_eq!(
            opciones,
            Opciones {
                contenido: PathBuf::from("mundo"),
                esquema: EsquemaColores::Fijo,
                graficos: ModoGraficos::Predeterminado,
                fondo_negro: false,
                sin_audio: false,
            }
        );
        assert_eq!(
            analizar_argumentos(["juego", "--content", "mundo", "--color-scheme", "terminal"])
                .unwrap()
                .esquema,
            EsquemaColores::Terminal
        );
        for args in [
            vec!["juego", "--color-scheme"],
            vec!["juego", "--color-scheme", "otro"],
            vec![
                "juego",
                "--color-scheme",
                "fixed",
                "--color-scheme",
                "terminal",
            ],
            vec!["juego", "--content", "--color-scheme", "fixed"],
            vec!["juego", "--otra"],
        ] {
            assert!(analizar_argumentos(args).is_err());
        }
    }
    #[test]
    fn argumentos_graficos_son_independientes_y_rechazan_opciones_invalidas() {
        assert_eq!(
            analizar_argumentos(["juego"]).unwrap().graficos,
            ModoGraficos::Predeterminado
        );
        for (valor, esperado) in [
            ("default", ModoGraficos::Predeterminado),
            ("custom-font", ModoGraficos::Fuente),
            ("ascii", ModoGraficos::Ascii),
        ] {
            let opciones = analizar_argumentos([
                "juego",
                "--graphics",
                valor,
                "--content",
                "mundo",
                "--color-scheme",
                "fixed",
                "-B",
                "--no-audio",
            ])
            .unwrap();
            assert_eq!(opciones.graficos, esperado);
            assert_eq!(opciones.contenido, PathBuf::from("mundo"));
            assert_eq!(opciones.esquema, EsquemaColores::Fijo);
            assert!(opciones.fondo_negro && opciones.sin_audio);
        }
        for args in [
            vec!["juego", "--graphics"],
            vec!["juego", "--graphics", "otro"],
            vec!["juego", "--graphics", "--no-audio"],
            vec!["juego", "--graphics", "-B"],
            vec!["juego", "--graphics", "ascii", "--graphics", "default"],
        ] {
            assert!(
                analizar_argumentos(args)
                    .unwrap_err()
                    .0
                    .contains("--graphics")
            );
        }
    }

    #[test]
    fn opcion_b_mayuscula_no_cambia_el_esquema_y_admite_combinaciones() {
        let opciones = analizar_argumentos(["juego", "-B"]).unwrap();
        assert!(opciones.fondo_negro);
        assert_eq!(opciones.esquema, EsquemaColores::Terminal);
        let opciones = analizar_argumentos([
            "juego",
            "--content",
            "mundo",
            "-B",
            "--color-scheme",
            "fixed",
        ])
        .unwrap();
        assert!(opciones.fondo_negro);
        assert_eq!(opciones.esquema, EsquemaColores::Fijo);
        assert_eq!(opciones.contenido, PathBuf::from("mundo"));
        for argumentos in [
            vec!["juego", "-B", "-B"],
            vec!["juego", "-b"],
            vec!["juego", "--content", "-B"],
            vec!["juego", "--color-scheme", "-B"],
        ] {
            assert!(analizar_argumentos(argumentos).is_err());
        }
    }
}
