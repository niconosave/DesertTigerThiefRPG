use fundsp::prelude32::{AudioUnit, sine};

use super::fuentes::{Azar, fuente};

struct Gota {
    tono: Box<dyn AudioUnit>,
    ruido: Box<dyn AudioUnit>,
    edad: u32,
    duracion: u32,
    frecuencia: f32,
    amplitud: f32,
}

impl Gota {
    fn nueva(frecuencia: u32, semilla: u64) -> Self {
        let mut tono: Box<dyn AudioUnit> = Box::new(sine());
        tono.set_sample_rate(frecuencia as f64);
        tono.allocate();
        Self {
            tono,
            ruido: fuente(false, 600.0, 4200.0, frecuencia, semilla),
            edad: 0,
            duracion: 0,
            frecuencia: 0.0,
            amplitud: 0.0,
        }
    }

    fn siguiente(&mut self, muestreo: u32) -> f32 {
        if self.edad >= self.duracion {
            return 0.0;
        }
        let u = self.edad as f32 / self.duracion as f32;
        let ataque = (self.edad as f32 / (muestreo as f32 * 0.0015)).min(1.0);
        let envolvente = ataque * (-7.0 * u).exp() * (1.0 - u);
        // Resonancia breve de burbuja, de tono ascendente; el ruido aporta el
        // chapoteo y evita una colección de notas sostenidas o campanitas.
        let frecuencia = (self.frecuencia * (1.0 + 0.6 * u)).min(muestreo as f32 * 0.4);
        let mut tono = [0.0];
        self.tono.tick(&[frecuencia], &mut tono);
        let muestra = (tono[0] * 0.28 + self.ruido.get_mono() * 0.72) * envolvente * self.amplitud;
        self.edad += 1;
        muestra
    }
}

/// Agua fragmentada entre piedras: un hilo suave y muchos pequeños borboteos.
/// Comparte la capa del río; todo el estado se prepara antes del callback.
pub(super) struct GeneradorRiachuelo {
    fondo: Box<dyn AudioUnit>,
    gotas: [Gota; 8],
    agenda: Azar,
    parametros: Azar,
    espera: u64,
    frecuencia: u32,
}

impl GeneradorRiachuelo {
    pub(super) fn nuevo(frecuencia: u32) -> Self {
        // Semilla propia: elegir esta variante no cambia viento, mar o aves.
        let mut semillas = Azar(0x726961636875656c);
        let fondo = fuente(false, 450.0, 5200.0, frecuencia, semillas.siguiente());
        let gotas = std::array::from_fn(|_| Gota::nueva(frecuencia, semillas.siguiente()));
        let mut generador = Self {
            fondo,
            gotas,
            agenda: Azar(semillas.siguiente()),
            parametros: Azar(semillas.siguiente()),
            espera: 0,
            frecuencia,
        };
        generador.agendar();
        generador
    }

    fn agendar(&mut self) {
        self.espera =
            ((-self.agenda.unidad().ln() / 32.0 * self.frecuencia as f64).ceil() as u64).max(1);
    }

    pub(super) fn siguiente(&mut self) -> f32 {
        self.espera = self.espera.saturating_sub(1);
        if self.espera == 0 {
            if let Some(gota) = self.gotas.iter_mut().find(|g| g.edad >= g.duracion) {
                gota.edad = 0;
                gota.duracion = (self.parametros.rango(30.0, 95.0) * self.frecuencia as f32
                    / 1000.0)
                    .ceil() as u32;
                gota.frecuencia = self.parametros.rango(300.0, 1900.0);
                gota.amplitud = self.parametros.rango(0.25, 0.65);
            }
            self.agendar();
        }
        self.fondo.get_mono() * 0.08
            + self
                .gotas
                .iter_mut()
                .map(|g| g.siguiente(self.frecuencia))
                .sum::<f32>()
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn riachuelo_finito_reproducible_y_con_voces_acotadas() {
        for frecuencia in [8000, 44100, 48000, 192000] {
            let mut a = GeneradorRiachuelo::nuevo(frecuencia);
            let mut b = GeneradorRiachuelo::nuevo(frecuencia);
            let mut energia = 0.0_f64;
            for _ in 0..frecuencia * 2 {
                let x = a.siguiente();
                assert_eq!(x, b.siguiente());
                assert!(x.is_finite());
                energia += f64::from(x).powi(2);
                assert_eq!(a.gotas.len(), 8);
            }
            assert!(energia > 0.01);
        }
    }
}
