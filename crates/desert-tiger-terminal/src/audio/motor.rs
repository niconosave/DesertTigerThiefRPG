use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use super::configuracion::ConfiguracionFogataAudio;
use super::fogata::GeneradorFogata;

pub(super) const SILENCIO: u8 = 0;
pub(super) const FOGATA: u8 = 1;
pub(super) const CERRAR: u8 = 2;
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

pub(super) struct MotorAudio {
    generador: GeneradorFogata,
    frecuencia: u32,
    entrada_ms: u32,
    salida_ms: u32,
    orden: u8,
    ganancia: f32,
    origen: f32,
    destino: f32,
    paso: u64,
    duracion: u64,
}

impl MotorAudio {
    pub(super) fn nuevo(config: ConfiguracionFogataAudio, frecuencia: u32) -> Result<Self, String> {
        let entrada_ms = config.entrada_ms();
        let salida_ms = config.salida_ms();
        let generador = GeneradorFogata::nuevo(config, frecuencia, 0x6465736572746669)?;
        Ok(Self {
            generador,
            frecuencia,
            entrada_ms,
            salida_ms,
            orden: SILENCIO,
            ganancia: 0.0,
            origen: 0.0,
            destino: 0.0,
            paso: 0,
            duracion: 0,
        })
    }

    pub(super) fn ordenar(&mut self, orden: u8) {
        if orden == self.orden {
            return;
        }
        self.orden = orden;
        self.origen = self.ganancia;
        self.destino = if orden == FOGATA { 1.0 } else { 0.0 };
        let ms = match orden {
            FOGATA => self.entrada_ms,
            CERRAR => 20,
            _ => self.salida_ms,
        };
        self.duracion = (ms as u64 * self.frecuencia as u64).div_ceil(1000);
        self.paso = 0;
    }

    pub(super) fn siguiente(&mut self) -> f32 {
        if self.paso < self.duracion {
            self.paso += 1;
            self.ganancia = if self.paso == self.duracion {
                self.destino
            } else {
                self.origen + (self.destino - self.origen) * self.paso as f32 / self.duracion as f32
            };
        }
        if self.en_silencio() {
            0.0
        } else {
            self.generador.siguiente_muestra() * self.ganancia
        }
    }

    pub(super) fn en_silencio(&self) -> bool {
        self.ganancia == 0.0 && self.destino == 0.0
    }
}

#[cfg(test)]
mod pruebas {
    use super::super::configuracion::fixture;
    use super::*;

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
        assert!((motor.ganancia - 0.5).abs() < 0.0001);
        motor.ordenar(FOGATA);
        assert_eq!(motor.paso, 4800); // Repetir el ambiente no reinicia la rampa.
        let antes = motor.ganancia;
        motor.ordenar(SILENCIO);
        assert_eq!(motor.ganancia, antes);
        motor.siguiente();
        assert!((motor.ganancia - antes).abs() < 0.001);
        for _ in 0..1000 {
            motor.siguiente();
        }
        let antes = motor.ganancia;
        motor.ordenar(FOGATA);
        assert_eq!(motor.ganancia, antes);
        for _ in 0..9600 {
            motor.siguiente();
        }
        assert_eq!(motor.ganancia, 1.0);
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
