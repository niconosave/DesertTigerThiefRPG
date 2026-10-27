use super::configuracion::ConfiguracionAudio;
#[cfg(test)]
use super::configuracion::ConfiguracionFogataAudio;
use super::{ambientes::GeneradorAmbientes, fogata::GeneradorFogata};
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

pub(super) const SILENCIO: u8 = 0;
pub(super) const FOGATA: u8 = 1;
pub(super) const EXTERIOR: u8 = 4;
pub(super) const RIO: u8 = 8;
pub(super) const MAR: u8 = 16;
pub(super) const BOSQUE: u8 = 32;
pub(super) const CERRAR: u8 = 128;
pub(super) const FALLO_STREAM: u8 = 1;
pub(super) const FALLO_MUESTRA: u8 = 2;

#[derive(Clone, Default)]
pub(super) struct Comunicacion {
    pub orden: Arc<AtomicU8>,
    pub fallo: Arc<AtomicU8>,
    pub silencio: Arc<AtomicU8>,
}
impl Comunicacion {
    pub(super) fn informar_fallo(&self, codigo: u8) {
        let _ = self
            .fallo
            .compare_exchange(0, codigo, Ordering::AcqRel, Ordering::Acquire);
    }
}

#[derive(Clone, Copy, Default)]
struct Rampa {
    ganancia: f32,
    origen: f32,
    destino: f32,
    paso: u64,
    duracion: u64,
}
impl Rampa {
    fn dirigir(&mut self, activo: bool, ms: u32, frecuencia: u32) {
        self.origen = self.ganancia;
        self.destino = if activo { 1.0 } else { 0.0 };
        self.paso = 0;
        self.duracion = (ms as u64 * frecuencia as u64).div_ceil(1000);
    }
    fn avanzar(&mut self) -> f32 {
        if self.paso < self.duracion {
            self.paso += 1;
            self.ganancia = if self.paso == self.duracion {
                self.destino
            } else {
                self.origen + (self.destino - self.origen) * self.paso as f32 / self.duracion as f32
            };
        }
        self.ganancia
    }
    fn silencio(&self) -> bool {
        self.ganancia == 0.0 && self.destino == 0.0
    }
}

pub(super) struct MotorAudio {
    generador: GeneradorFogata,
    ambientes: GeneradorAmbientes,
    frecuencia: u32,
    entrada_ms: u32,
    salida_ms: u32,
    orden: u8,
    fogata: Rampa,
    capas: [Rampa; 4],
}
impl MotorAudio {
    pub(super) fn con_configuracion(
        config: ConfiguracionAudio,
        frecuencia: u32,
    ) -> Result<Self, String> {
        let entrada_ms = config.fogata.entrada_ms();
        let salida_ms = config.fogata.salida_ms();
        let generador = GeneradorFogata::nuevo(config.fogata, frecuencia, 0x6465736572746669)?;
        let ambientes = GeneradorAmbientes::nuevo(config.ambientes, frecuencia)?;
        Ok(Self {
            generador,
            ambientes,
            frecuencia,
            entrada_ms,
            salida_ms,
            orden: SILENCIO,
            fogata: Rampa::default(),
            capas: [Rampa::default(); 4],
        })
    }
    #[cfg(test)]
    pub(super) fn nuevo(config: ConfiguracionFogataAudio, frecuencia: u32) -> Result<Self, String> {
        Self::con_configuracion(
            ConfiguracionAudio {
                fogata: config,
                ambientes: super::configuracion::ConfiguracionAmbientes::fixture(),
            },
            frecuencia,
        )
    }
    pub(super) fn ordenar(&mut self, orden: u8) {
        if orden == self.orden {
            return;
        }
        let anterior = self.orden;
        self.orden = orden;
        let cierre = orden == CERRAR;
        if cierre || (orden & FOGATA != anterior & FOGATA) {
            let activo = !cierre && orden & FOGATA != 0;
            let ms = if cierre {
                20
            } else if activo {
                self.entrada_ms
            } else {
                self.salida_ms
            };
            self.fogata.dirigir(activo, ms, self.frecuencia);
        }
        for (capa, mascara) in self.capas.iter_mut().zip([EXTERIOR, RIO, MAR, BOSQUE]) {
            if cierre || (orden & mascara != anterior & mascara) {
                capa.dirigir(
                    !cierre && orden & mascara != 0,
                    if cierre { 20 } else { 1500 },
                    self.frecuencia,
                );
            }
        }
    }
    pub(super) fn siguiente(&mut self) -> f32 {
        let ganancia = self.fogata.avanzar();
        let mut suma = if self.fogata.silencio() {
            0.0
        } else {
            self.generador.siguiente_muestra() * ganancia
        };
        for (i, capa) in self.capas.iter_mut().enumerate() {
            let ganancia = capa.avanzar();
            if !capa.silencio() {
                suma += self.ambientes.siguiente(i) * ganancia;
            }
        }
        0.8 * (suma / 0.8).tanh()
    }
    pub(super) fn en_silencio(&self) -> bool {
        self.fogata.silencio() && self.capas.iter().all(Rampa::silencio)
    }
}
#[cfg(test)]
mod pruebas {
    use super::super::configuracion::fixture;
    use super::*;

    #[test]
    fn cambiar_entorno_no_reinicia_capas_comunes_y_el_cierre_silencia_todo() {
        let mut m = MotorAudio::nuevo(fixture(0.25, 0.18, [8.0, 0.6]), 8000).unwrap();
        m.ordenar(EXTERIOR | RIO);
        for _ in 0..4000 {
            m.siguiente();
        }
        let paso_viento = m.capas[0].paso;
        m.ordenar(EXTERIOR | RIO | BOSQUE);
        assert_eq!(m.capas[0].paso, paso_viento);
        assert_eq!(m.capas[1].paso, 4000);
        assert_eq!(m.capas[3].paso, 0);
        for _ in 0..12000 {
            assert!(m.siguiente().is_finite());
        }
        m.ordenar(EXTERIOR | MAR | FOGATA);
        for _ in 0..24000 {
            assert!(m.siguiente().abs() <= 0.8);
        }
        assert!(m.capas[1].silencio() && m.capas[3].silencio());
        m.ordenar(CERRAR);
        for _ in 0..160 {
            m.siguiente();
        }
        assert!(m.en_silencio());
        assert_eq!(m.siguiente(), 0.0);
    }

    #[test]
    fn rampas_se_invierten_sin_saltos_y_cierre_termina_en_cero() {
        let mut motor = MotorAudio::nuevo(fixture(1.0, 1.0, [8.0, 0.6]), 48000).unwrap();
        for _ in 0..100 {
            assert_eq!(motor.siguiente(), 0.0);
        }
        motor.ordenar(FOGATA);
        for _ in 0..4800 {
            motor.siguiente();
        }
        assert!((motor.fogata.ganancia - 0.5).abs() < 0.0001);
        motor.ordenar(FOGATA);
        assert_eq!(motor.fogata.paso, 4800); // Repetir el ambiente no reinicia la rampa.
        let antes = motor.fogata.ganancia;
        motor.ordenar(SILENCIO);
        assert_eq!(motor.fogata.ganancia, antes);
        motor.siguiente();
        assert!((motor.fogata.ganancia - antes).abs() < 0.001);
        for _ in 0..1000 {
            motor.siguiente();
        }
        let antes = motor.fogata.ganancia;
        motor.ordenar(FOGATA);
        assert_eq!(motor.fogata.ganancia, antes);
        for _ in 0..9600 {
            motor.siguiente();
        }
        assert_eq!(motor.fogata.ganancia, 1.0);
        motor.ordenar(CERRAR);
        for _ in 0..960 {
            motor.siguiente();
        }
        assert!(motor.en_silencio());
        assert_eq!(motor.siguiente(), 0.0);
    }

    #[test]
    fn silencio_suspende_el_generador_y_bloques_conservan_la_senal() {
        let crear = || MotorAudio::nuevo(fixture(0.25, 0.18, [8.0, 0.6]), 44100).unwrap();
        let mut a = crear();
        let mut b = crear();
        for _ in 0..1000 {
            a.siguiente();
        }
        a.ordenar(FOGATA);
        b.ordenar(FOGATA);
        for _ in 0..10000 {
            assert_eq!(a.siguiente(), b.siguiente());
        }
        a.ordenar(SILENCIO);
        b.ordenar(SILENCIO);
        for _ in 0..11025 {
            assert_eq!(a.siguiente(), b.siguiente());
        }
        assert!(a.en_silencio());
        for _ in 0..1000 {
            a.siguiente();
        }
        a.ordenar(FOGATA);
        b.ordenar(FOGATA);
        for _ in 0..10000 {
            assert_eq!(a.siguiente(), b.siguiente());
        }
    }
}
