#[cfg(feature = "audio")]
mod ambientes;
mod configuracion;
#[cfg(feature = "audio")]
mod dispositivo;
#[cfg(feature = "audio")]
mod fogata;
#[cfg(feature = "audio")]
mod fuentes;
#[cfg(feature = "audio")]
mod motor;
mod paisaje;
#[cfg(feature = "audio")]
mod partitura;
#[cfg(feature = "audio")]
mod preparacion_musica;
#[cfg(feature = "audio")]
mod reproductor;
#[cfg(feature = "audio")]
mod riachuelo;
#[cfg(feature = "audio")]
mod transicion;

use configuracion::{
    ConfiguracionAmbientes, ConfiguracionAudio, ConfiguracionFogataAudio, ConfiguracionMusica,
};
use std::path::Path;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct AmbienteSonoro {
    pub exterior: bool,
    pub fogata: bool,
    pub rio: bool,
    pub mar: bool,
    pub bosque: bool,
}

impl AmbienteSonoro {
    pub(crate) const SILENCIO: Self = Self {
        exterior: false,
        fogata: false,
        rio: false,
        mar: false,
        bosque: false,
    };
    #[cfg(test)]
    const FOGATA: Self = Self {
        fogata: true,
        ..Self::SILENCIO
    };
    pub(crate) fn del_lugar(
        mundo: &desert_tiger_app::VistaMundo,
        lugar: desert_tiger_app::IdLugar,
        fogata: bool,
    ) -> Self {
        paisaje::seleccionar(mundo, lugar, fogata)
    }
    #[cfg(feature = "audio")]
    fn codigo(self) -> u8 {
        u8::from(self.fogata)
            | (u8::from(self.exterior) << 2)
            | (u8::from(self.rio) << 3)
            | (u8::from(self.mar) << 4)
            | (u8::from(self.bosque) << 5)
    }
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
    Musica,
}

impl AvisoAudio {
    pub(crate) fn mensaje(self) -> &'static str {
        match self {
            Self::Musica => "Música no disponible. Los demás sonidos continúan.",
            _ => "Audio no disponible. El juego continúa sin sonido.",
        }
    }
}

pub(crate) struct PreparacionAudio {
    config: Option<ConfiguracionAudio>,
}

pub(crate) fn preparar(habilitado: bool, raiz: &Path) -> Result<PreparacionAudio, String> {
    let config = if habilitado && cfg!(feature = "audio") {
        Some(ConfiguracionAudio {
            fogata: ConfiguracionFogataAudio::cargar(&raiz.join("fogata.ron"))?,
            ambientes: ConfiguracionAmbientes::cargar(&raiz.join("ambientes.ron"))?,
            musica: ConfiguracionMusica::cargar(&raiz.join("musica"))?,
        })
    } else {
        None
    };
    Ok(PreparacionAudio { config })
}

#[derive(Default)]
pub(crate) struct ControlAudio {
    #[cfg(feature = "audio")]
    salida: Option<dispositivo::SalidaAudio>,
    #[cfg(feature = "audio")]
    preparacion_musica: Option<preparacion_musica::TrabajoMusica>,
    #[cfg(feature = "audio")]
    repertorio: Option<preparacion_musica::RepertorioCompartido>,
    aviso: Option<AvisoAudio>,
    ambiente: AmbienteSonoro,
    ajustes_musica: Option<crate::musica_campamento::AgendaMusica>,
}

impl ControlAudio {
    pub(crate) fn iniciar(preparacion: PreparacionAudio) -> Self {
        #[cfg(feature = "audio")]
        if let Some(mut config) = preparacion.config {
            let ajustes = (config.musica.volumen > 0.0).then_some(config.musica.agenda);
            let repertorio = config.musica.piezas.clone();
            let pendientes = std::mem::take(&mut config.musica.pendientes);
            let mut control = Self::desde_apertura(dispositivo::SalidaAudio::abrir(config));
            control.ajustes_musica = ajustes;
            control.repertorio = Some(repertorio.clone());
            if control.salida.is_some() && ajustes.is_some() && !pendientes.is_empty() {
                match preparacion_musica::TrabajoMusica::iniciar(pendientes, repertorio) {
                    Ok(trabajo) => control.preparacion_musica = Some(trabajo),
                    Err(_) => control.aviso = Some(AvisoAudio::Musica),
                }
            }
            return control;
        }
        #[cfg(not(feature = "audio"))]
        let _ = preparacion.config;
        Self::default()
    }

    #[cfg(feature = "audio")]
    fn desde_apertura(apertura: Result<dispositivo::SalidaAudio, AvisoAudio>) -> Self {
        let mut control = Self::default();
        match apertura {
            Ok(salida) => control.salida = Some(salida),
            Err(aviso) => control.aviso = Some(aviso),
        }
        control
    }

    pub(crate) fn actualizar(&mut self, ambiente: AmbienteSonoro) {
        #[cfg(feature = "audio")]
        {
            if let Some(fallo) = self.salida.as_ref().and_then(|s| s.fallo()) {
                self.salida = None;
                self.preparacion_musica = None;
                self.aviso = Some(fallo);
            }
            if let Some(trabajo) = &mut self.preparacion_musica
                && let Some(Err(_)) = trabajo.revisar()
            {
                self.aviso = Some(AvisoAudio::Musica);
            }
            if self.ambiente != ambiente
                && let Some(salida) = &self.salida
            {
                salida.ordenar(ambiente.codigo());
            }
        }
        self.ambiente = ambiente;
    }

    pub(crate) fn tomar_aviso(&mut self) -> Option<AvisoAudio> {
        self.aviso.take()
    }

    pub(crate) fn ajustes_musica(&self) -> Option<crate::musica_campamento::AgendaMusica> {
        #[cfg(feature = "audio")]
        {
            self.salida.as_ref()?;
            let repertorio = self.repertorio.as_ref()?.get()?.as_ref().ok()?;
            repertorio.audible.then_some(self.ajustes_musica?)
        }
        #[cfg(not(feature = "audio"))]
        {
            self.ajustes_musica
        }
    }

    pub(crate) fn disponible(&self) -> bool {
        #[cfg(feature = "audio")]
        {
            self.salida.is_some()
        }
        #[cfg(not(feature = "audio"))]
        {
            false
        }
    }

    pub(crate) fn solicitar_musica(&self, evento: u32) {
        #[cfg(feature = "audio")]
        if let Some(salida) = &self.salida {
            salida.solicitar_musica(evento);
        }
        #[cfg(not(feature = "audio"))]
        let _ = evento;
    }

    pub(crate) fn estado_musica(&self) -> Option<crate::musica_campamento::ProgresoMusica> {
        #[cfg(feature = "audio")]
        {
            self.salida.as_ref().and_then(|s| s.estado_musica())
        }
        #[cfg(not(feature = "audio"))]
        {
            None
        }
    }

    pub(crate) fn cerrar(&mut self) {
        #[cfg(feature = "audio")]
        {
            self.preparacion_musica = None;
        }
        #[cfg(feature = "audio")]
        if let Some(salida) = self.salida.take() {
            salida.cerrar();
        }
        self.ambiente = AmbienteSonoro::SILENCIO;
        self.ajustes_musica = None;
    }
}

#[cfg(feature = "audio")]
pub(crate) fn previsualizar(entrada: &Path, salida: &Path) -> Result<f64, String> {
    partitura::previsualizar(entrada, salida)
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
        control.actualizar(AmbienteSonoro::FOGATA);
        control.actualizar(AmbienteSonoro::FOGATA);
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
                AmbienteSonoro::FOGATA,
                AmbienteSonoro::SILENCIO,
                AmbienteSonoro::FOGATA,
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
