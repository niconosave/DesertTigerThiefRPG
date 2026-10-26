mod configuracion;
#[cfg(feature = "audio")]
mod dispositivo;
#[cfg(feature = "audio")]
mod fogata;
#[cfg(feature = "audio")]
mod motor;

use configuracion::ConfiguracionFogataAudio;
use std::path::Path;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AmbienteSonoro {
    #[default]
    Silencio,
    Fogata,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(crate) enum AvisoAudio {
    SinDispositivo,
    Configuracion,
    Creacion,
    Arranque,
    Flujo,
    MuestraNoFinita,
}

impl AvisoAudio {
    pub(crate) fn mensaje(self) -> &'static str {
        "Audio no disponible. El juego continúa sin sonido."
    }
}

pub(crate) struct PreparacionAudio {
    config: Option<ConfiguracionFogataAudio>,
}

pub(crate) fn preparar(habilitado: bool, raiz: &Path) -> Result<PreparacionAudio, String> {
    let config = if habilitado && cfg!(feature = "audio") {
        Some(ConfiguracionFogataAudio::cargar(&raiz.join("fogata.ron"))?)
    } else {
        None
    };
    Ok(PreparacionAudio { config })
}

#[derive(Default)]
pub(crate) struct ControlAudio {
    #[cfg(feature = "audio")]
    salida: Option<dispositivo::SalidaAudio>,
    aviso: Option<AvisoAudio>,
    ambiente: AmbienteSonoro,
}

impl ControlAudio {
    pub(crate) fn iniciar(preparacion: PreparacionAudio) -> Self {
        #[cfg(feature = "audio")]
        if let Some(config) = preparacion.config {
            return Self::desde_apertura(dispositivo::SalidaAudio::abrir(config));
        }
        #[cfg(not(feature = "audio"))]
        let _ = preparacion.config;
        Self::default()
    }

    #[cfg(feature = "audio")]
    fn desde_apertura(apertura: Result<dispositivo::SalidaAudio, AvisoAudio>) -> Self {
        match apertura {
            Ok(salida) => Self {
                salida: Some(salida),
                ..Self::default()
            },
            Err(aviso) => Self {
                salida: None,
                aviso: Some(aviso),
                ambiente: AmbienteSonoro::Silencio,
            },
        }
    }

    pub(crate) fn actualizar(&mut self, ambiente: AmbienteSonoro) {
        #[cfg(feature = "audio")]
        {
            if let Some(fallo) = self.salida.as_ref().and_then(|s| s.fallo()) {
                self.salida = None;
                self.aviso = Some(fallo);
            }
            if self.ambiente != ambiente
                && let Some(salida) = &self.salida
            {
                salida.ordenar(match ambiente {
                    AmbienteSonoro::Silencio => motor::SILENCIO,
                    AmbienteSonoro::Fogata => motor::FOGATA,
                });
            }
        }
        self.ambiente = ambiente;
    }

    pub(crate) fn tomar_aviso(&mut self) -> Option<AvisoAudio> {
        self.aviso.take()
    }

    pub(crate) fn cerrar(&mut self) {
        #[cfg(feature = "audio")]
        if let Some(salida) = self.salida.take() {
            salida.cerrar();
        }
        self.ambiente = AmbienteSonoro::Silencio;
    }
}

impl Drop for ControlAudio {
    fn drop(&mut self) {
        self.cerrar();
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn deshabilitado_no_carga_recursos_ni_abre_dispositivo() {
        let preparacion = preparar(false, Path::new("/ruta/ausente")).unwrap();
        assert!(preparacion.config.is_none());
        let mut control = ControlAudio::iniciar(preparacion);
        control.actualizar(AmbienteSonoro::Fogata);
        control.actualizar(AmbienteSonoro::Fogata);
        assert!(control.tomar_aviso().is_none());
        control.cerrar();
        control.cerrar();
    }

    #[cfg(not(feature = "audio"))]
    #[test]
    fn compilacion_sin_audio_omite_recursos_aunque_esten_habilitados() {
        assert!(
            preparar(true, Path::new("/ruta/ausente"))
                .unwrap()
                .config
                .is_none()
        );
    }

    #[cfg(feature = "audio")]
    #[test]
    fn fallo_de_inicio_avisa_una_vez_y_no_reintenta() {
        for aviso in [
            AvisoAudio::SinDispositivo,
            AvisoAudio::Configuracion,
            AvisoAudio::Creacion,
            AvisoAudio::Arranque,
        ] {
            let mut control = ControlAudio::desde_apertura(Err(aviso));
            assert_eq!(control.tomar_aviso(), Some(aviso));
            for ambiente in [
                AmbienteSonoro::Fogata,
                AmbienteSonoro::Silencio,
                AmbienteSonoro::Fogata,
            ] {
                control.actualizar(ambiente);
                assert!(control.salida.is_none());
                assert!(control.tomar_aviso().is_none());
            }
            control.cerrar();
        }
        assert!(preparar(true, Path::new("/ruta/ausente")).is_err());
    }

    #[cfg(feature = "audio")]
    #[test]
    fn procesar_sonido_no_cambia_el_cierre_del_campamento_ni_el_viaje() {
        use desert_tiger_app::{
            AccionDisponible, ConfiguracionPartida, IntencionFrontend, Partida, VistaSesion,
            cargar_contenido,
        };
        let contenido =
            cargar_contenido(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"))
                .unwrap();
        let recorrer = |con_audio: bool| {
            let mut partida =
                Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) }).unwrap();
            let mut motor =
                motor::MotorAudio::nuevo(configuracion::fixture(0.25, 0.18, [8.0, 0.6]), 48000)
                    .unwrap();
            let campamento = match partida
                .procesar(IntencionFrontend::ComenzarCampamento)
                .unwrap()
            {
                VistaSesion::Acampando(c) => c,
                _ => panic!("campamento esperado"),
            };
            if con_audio {
                motor.ordenar(motor::FOGATA);
                for _ in 0..48000 {
                    motor.siguiente();
                }
                motor.ordenar(motor::SILENCIO);
                for _ in 0..12000 {
                    motor.siguiente();
                }
            }
            let mundo = match partida
                .procesar(IntencionFrontend::AbandonarCampamento {
                    campamento: campamento.id,
                })
                .unwrap()
            {
                VistaSesion::Disponible(m) => m,
                _ => panic!("mundo esperado"),
            };
            let destino = mundo
                .acciones
                .iter()
                .find_map(|a| match a {
                    AccionDisponible::Viajar { destinos } => destinos.first().copied(),
                    _ => None,
                })
                .unwrap();
            let mut actual = partida
                .procesar(IntencionFrontend::ComenzarViaje { destino })
                .unwrap();
            let mut vistas = vec![VistaSesion::Disponible(mundo)];
            for _ in 0..500 {
                vistas.push(actual.clone());
                actual = match actual {
                    VistaSesion::Disponible(_) => return vistas,
                    VistaSesion::PresentandoAccion(f) => partida
                        .procesar(IntencionFrontend::ContinuarPresentacion { fragmento: f.id })
                        .unwrap(),
                    VistaSesion::EsperandoDecision(d) => partida
                        .procesar(IntencionFrontend::ResponderDecision {
                            decision: d.id,
                            opcion: d.opciones[0].id,
                        })
                        .unwrap(),
                    _ => panic!("estado inesperado"),
                };
            }
            panic!("el viaje no terminó");
        };
        assert_eq!(recorrer(false), recorrer(true));
    }
}
