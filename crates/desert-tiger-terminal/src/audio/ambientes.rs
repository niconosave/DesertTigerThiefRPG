use super::{
    configuracion::ConfiguracionAmbientes,
    fuentes::{Azar, fuente},
};
use fundsp::prelude32::{AudioUnit, sine};

struct Canto {
    oscilador: Box<dyn AudioUnit>,
    azar: Azar,
    clase: usize,
    espera: u64,
    edad: u32,
    duracion: u32,
    frecuencia: u32,
    tono: f32,
    activo: bool,
}

impl Canto {
    fn nuevo(clase: usize, frecuencia: u32, semilla: u64) -> Self {
        let mut oscilador: Box<dyn AudioUnit> = Box::new(sine());
        oscilador.set_sample_rate(frecuencia as f64);
        oscilador.allocate();
        let segundos = [0.7, 0.9, 1.1][clase];
        let mut canto = Self {
            oscilador,
            azar: Azar(semilla),
            clase,
            espera: 0,
            edad: 0,
            duracion: (segundos * frecuencia as f32) as u32,
            frecuencia,
            tono: 1.0,
            activo: false,
        };
        canto.agendar();
        canto
    }

    fn agendar(&mut self) {
        let (min, max) = [(12.0, 28.0), (2.5, 6.0), (4.0, 9.0)][self.clase];
        self.espera = (self.azar.rango(min, max) * self.frecuencia as f32).ceil() as u64;
    }

    fn siguiente(&mut self) -> f32 {
        if !self.activo {
            self.espera = self.espera.saturating_sub(1);
            if self.espera > 0 {
                return 0.0;
            }
            self.edad = 0;
            self.tono = self.azar.rango(0.92, 1.08);
            self.activo = true;
        }
        let t = self.edad as f32 / self.frecuencia as f32;
        let (frecuencia, pulso) = match self.clase {
            0 => (
                2100.0 - t * 1100.0,
                (std::f32::consts::PI * self.edad as f32 / self.duracion as f32)
                    .sin()
                    .powi(2),
            ),
            1 => {
                let nota = (t / 0.3).floor();
                let u = (t % 0.3) / 0.22;
                (
                    2500.0 + nota * 260.0 + u.min(1.0) * 350.0,
                    if u < 1.0 {
                        (std::f32::consts::PI * u).sin().powi(2)
                    } else {
                        0.0
                    },
                )
            }
            _ => {
                let u = (t % 0.55) / 0.4;
                (
                    1450.0 + 180.0 * (std::f32::consts::TAU * 6.0 * t).sin(),
                    if u < 1.0 {
                        (std::f32::consts::PI * u).sin().powi(2)
                    } else {
                        0.0
                    },
                )
            }
        };
        let mut muestra = [0.0];
        self.oscilador.tick(
            &[(frecuencia * self.tono).min(self.frecuencia as f32 * 0.4)],
            &mut muestra,
        );
        self.edad += 1;
        if self.edad >= self.duracion {
            self.activo = false;
            self.agendar();
        }
        muestra[0] * pulso
    }
}

pub(super) struct GeneradorAmbientes {
    ruidos: [Box<dyn AudioUnit>; 5],
    aves: [Canto; 3],
    fases: [f64; 6],
    frecuencia: u32,
    volumen: f32,
    niveles: [f32; 6],
}

impl GeneradorAmbientes {
    pub(super) fn nuevo(config: ConfiguracionAmbientes, frecuencia: u32) -> Result<Self, String> {
        if !(8000..=192000).contains(&frecuencia) {
            return Err("frecuencia ambiental fuera de 8000–192000 Hz".into());
        }
        let mut azar = Azar(0x70616973616a6573);
        Ok(Self {
            ruidos: [
                fuente(true, 90.0, 950.0, frecuencia, azar.siguiente()),
                fuente(false, 180.0, 4800.0, frecuencia, azar.siguiente()),
                fuente(true, 120.0, 700.0, frecuencia, azar.siguiente()),
                fuente(true, 70.0, 2400.0, frecuencia, azar.siguiente()),
                fuente(false, 400.0, 2400.0, frecuencia, azar.siguiente()),
            ],
            aves: std::array::from_fn(|i| Canto::nuevo(i, frecuencia, azar.siguiente())),
            fases: [0.31, 0.73, 0.12, 0.57, 0.42, 0.86],
            frecuencia,
            volumen: config.volumen(),
            niveles: config.niveles(),
        })
    }

    fn onda(&mut self, indice: usize, segundos: f64) -> f32 {
        self.fases[indice] =
            (self.fases[indice] + 1.0 / (segundos * self.frecuencia as f64)).fract();
        (std::f64::consts::TAU * self.fases[indice]).sin() as f32
    }

    // Una llamada avanza solo la capa que se escucha o está desvaneciéndose.
    pub(super) fn siguiente(&mut self, capa: usize) -> f32 {
        let muestra = match capa {
            0 => {
                let rafaga = 0.65 + 0.20 * self.onda(0, 7.3) + 0.15 * self.onda(1, 13.7);
                self.ruidos[0].get_mono() * rafaga * self.niveles[0]
                    + self.aves[0].siguiente() * self.niveles[1]
            }
            1 => {
                let burbujeo = 0.75 + 0.15 * self.onda(2, 0.17) + 0.1 * self.onda(3, 0.29);
                (self.ruidos[1].get_mono() * 0.7 + self.ruidos[2].get_mono() * 0.3)
                    * burbujeo
                    * self.niveles[2]
            }
            2 => {
                let oleaje =
                    (0.5 + 0.5 * (0.7 * self.onda(4, 6.7) + 0.3 * self.onda(5, 10.9))).powi(2);
                (self.ruidos[3].get_mono() * 0.6 + self.ruidos[4].get_mono() * 0.4)
                    * (0.08 + 0.92 * oleaje)
                    * self.niveles[3]
            }
            3 => {
                self.aves[1].siguiente() * self.niveles[4]
                    + self.aves[2].siguiente() * self.niveles[5]
            }
            _ => 0.0,
        };
        muestra * self.volumen
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn volumen_cero_silencia_todas_las_capas() {
        let mut g =
            GeneradorAmbientes::nuevo(ConfiguracionAmbientes::fixture_silenciosa(), 8000).unwrap();
        for _ in 0..8000 * 32 {
            for capa in 0..4 {
                assert_eq!(g.siguiente(capa), 0.0);
            }
        }
        assert!(GeneradorAmbientes::nuevo(ConfiguracionAmbientes::fixture(), 0).is_err());
    }
    #[test]
    fn capas_son_reproducibles_audibles_y_acotadas() {
        for frecuencia in [8000, 44100, 48000, 192000] {
            let mut a =
                GeneradorAmbientes::nuevo(ConfiguracionAmbientes::fixture(), frecuencia).unwrap();
            let mut b =
                GeneradorAmbientes::nuevo(ConfiguracionAmbientes::fixture(), frecuencia).unwrap();
            for capa in 0..4 {
                let mut energia = 0.0;
                // Incluye la primera frase de ambas aves de bosque y del ave lejana.
                let cantidad = if frecuencia == 8000 {
                    frecuencia * 32
                } else {
                    frecuencia
                };
                for _ in 0..cantidad {
                    let x = a.siguiente(capa);
                    assert_eq!(x, b.siguiente(capa));
                    assert!(x.is_finite() && x.abs() <= 0.8);
                    energia += x * x;
                }
                if frecuencia == 8000 || capa < 3 {
                    assert!(energia > 0.01, "capa {capa}");
                }
            }
        }
    }

    #[test]
    fn cantos_tienen_agendas_y_frases_distintas() {
        for (clase, (min, max)) in [(12.0, 28.0), (2.5, 6.0), (4.0, 9.0)]
            .into_iter()
            .enumerate()
        {
            let mut c = Canto::nuevo(clase, 8000, 42);
            assert!((min * 8000.0..=max * 8000.0).contains(&(c.espera as f64)));
            c.espera = 1;
            let muestras: Vec<_> = (0..c.duracion).map(|_| c.siguiente()).collect();
            assert!(!c.activo);
            assert_eq!(c.siguiente(), 0.0);
            assert!(muestras.iter().any(|m| m.abs() > 0.1));
            assert!(muestras[0].abs() < 0.001);
        }
    }
}
