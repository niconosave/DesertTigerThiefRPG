use super::fuentes::{Azar, fuente};
use fundsp::prelude32::AudioUnit;

use super::configuracion::ConfiguracionFogataAudio;

struct Voz {
    ruido: Box<dyn AudioUnit>,
    edad: u32,
    ataque: u32,
    caida: u32,
    amplitud: f32,
    activa: bool,
}

impl Voz {
    fn muestra(&mut self) -> f32 {
        if !self.activa {
            return 0.0;
        }
        let envolvente = if self.edad < self.ataque {
            self.edad as f32 / self.ataque as f32
        } else {
            let u = (self.edad - self.ataque) as f32 / self.caida as f32;
            (-6.0 * u).exp() * (1.0 - u)
        };
        let muestra = self.ruido.get_mono() * self.amplitud * envolvente;
        self.edad += 1;
        if self.edad >= self.ataque + self.caida {
            self.activa = false;
        }
        muestra
    }
}

struct Familia {
    voces: Vec<Voz>,
    agenda: Azar,
    parametros: Azar,
    tasa: f32,
    espera: u64,
    frecuencia: u32,
    grande: bool,
}

impl Familia {
    fn nueva(grande: bool, tasa: f32, frecuencia: u32, semillas: &mut Azar) -> Self {
        let voces = (0..if grande { 2 } else { 6 })
            .map(|_| Voz {
                ruido: fuente(
                    false,
                    if grande { 250.0 } else { 1200.0 },
                    if grande { 2800.0 } else { 6000.0 },
                    frecuencia,
                    semillas.siguiente(),
                ),
                edad: 0,
                ataque: 1,
                caida: 1,
                amplitud: 0.0,
                activa: false,
            })
            .collect();
        let mut familia = Self {
            voces,
            agenda: Azar(semillas.siguiente()),
            parametros: Azar(semillas.siguiente()),
            tasa,
            espera: 0,
            frecuencia,
            grande,
        };
        familia.agendar();
        familia
    }

    fn agendar(&mut self) {
        self.espera = if self.tasa == 0.0 {
            u64::MAX
        } else {
            // La conversión float→entero de Rust satura para tasas diminutas.
            ((-self.agenda.unidad().ln() / self.tasa as f64 * self.frecuencia as f64).ceil() as u64)
                .max(1)
        };
    }

    fn muestra(&mut self) -> f32 {
        if self.tasa != 0.0 {
            self.espera = self.espera.saturating_sub(1);
            if self.espera == 0 {
                if let Some(voz) = self.voces.iter_mut().find(|v| !v.activa) {
                    voz.edad = 0;
                    voz.ataque = (self.frecuencia as f32 * if self.grande { 0.002 } else { 0.001 })
                        .ceil() as u32;
                    let duracion = if self.grande {
                        self.parametros.rango(40.0, 120.0)
                    } else {
                        self.parametros.rango(15.0, 60.0)
                    };
                    voz.caida = (duracion * self.frecuencia as f32 / 1000.0).ceil() as u32;
                    voz.amplitud = if self.grande {
                        self.parametros.rango(0.5, 0.9)
                    } else {
                        self.parametros.rango(0.2, 0.6)
                    };
                    voz.activa = true;
                }
                self.agendar();
            }
        }
        self.voces.iter_mut().map(Voz::muestra).sum()
    }
}

pub(super) struct GeneradorFogata {
    fondo: Box<dyn AudioUnit>,
    familias: [Familia; 2],
    azar_modulacion: Azar,
    modulacion: [f32; 2],
    paso_modulacion: u32,
    duracion_modulacion: u32,
    indice: u64,
    volumen: f32,
    nivel: f32,
}

impl GeneradorFogata {
    pub(super) fn nuevo(
        config: ConfiguracionFogataAudio,
        frecuencia: u32,
        semilla: u64,
    ) -> Result<Self, String> {
        if !(8000..=192000).contains(&frecuencia) {
            return Err("frecuencia de audio fuera de 8000–192000 Hz".into());
        }
        let mut semillas = Azar(semilla);
        let fondo = fuente(true, 80.0, 1500.0, frecuencia, semillas.siguiente());
        let [pequenos, grandes] = config.tasas();
        let familias = [
            Familia::nueva(false, pequenos, frecuencia, &mut semillas),
            Familia::nueva(true, grandes, frecuencia, &mut semillas),
        ];
        let mut azar_modulacion = Azar(semillas.siguiente());
        let modulacion = [
            azar_modulacion.rango(0.7, 1.0),
            azar_modulacion.rango(0.7, 1.0),
        ];
        Ok(Self {
            fondo,
            familias,
            azar_modulacion,
            modulacion,
            paso_modulacion: 0,
            duracion_modulacion: frecuencia / 2,
            indice: 0,
            volumen: config.volumen(),
            nivel: config.nivel_brasa(),
        })
    }

    pub(super) fn siguiente_muestra(&mut self) -> f32 {
        let s = self.paso_modulacion as f32 / self.duracion_modulacion as f32;
        let modulacion = self.modulacion[0]
            + (self.modulacion[1] - self.modulacion[0]) * s * s * (3.0 - 2.0 * s);
        let mezcla = self.fondo.get_mono() * self.nivel * modulacion
            + self.familias.iter_mut().map(Familia::muestra).sum::<f32>();
        self.paso_modulacion += 1;
        if self.paso_modulacion >= self.duracion_modulacion {
            self.paso_modulacion = 0;
            self.modulacion = [self.modulacion[1], self.azar_modulacion.rango(0.7, 1.0)];
        }
        self.indice = self.indice.saturating_add(1);
        self.volumen * 0.8 * mezcla.tanh()
    }
}

#[cfg(test)]
mod pruebas {
    use super::super::configuracion::fixture;
    use super::*;

    #[test]
    fn senal_finita_acotada_y_reproducible_en_distintas_frecuencias() {
        for frecuencia in [8000, 44100, 48000, 192000] {
            let config = fixture(1.0, 1.0, [40.0, 5.0]);
            let mut a = GeneradorFogata::nuevo(config.clone(), frecuencia, 42).unwrap();
            let mut b = GeneradorFogata::nuevo(config.clone(), frecuencia, 42).unwrap();
            let mut c = GeneradorFogata::nuevo(config, frecuencia, 43).unwrap();
            let mut energia = 0.0_f64;
            let mut diferentes = 0;
            for _ in 0..frecuencia * 2 {
                let x = a.siguiente_muestra();
                assert_eq!(x, b.siguiente_muestra());
                diferentes += usize::from(x != c.siguiente_muestra());
                assert!(x.is_finite() && x.abs() <= 0.8);
                energia += (x as f64).powi(2);
            }
            assert!(energia > 0.01);
            assert!(diferentes > 100);
        }
    }

    #[test]
    fn fuentes_desactivadas_y_volumen_cero_producen_silencio() {
        for config in [
            fixture(0.0, 1.0, [40.0, 5.0]),
            fixture(1.0, 0.0, [0.0, 0.0]),
        ] {
            let mut g = GeneradorFogata::nuevo(config, 48000, 1).unwrap();
            for _ in 0..48000 {
                assert_eq!(g.siguiente_muestra(), 0.0);
            }
        }
        assert!(GeneradorFogata::nuevo(fixture(1.0, 1.0, [1.0, 1.0]), 0, 0).is_err());
    }

    #[test]
    fn agendas_diminutas_contadores_y_voces_no_desbordan() {
        let mut g =
            GeneradorFogata::nuevo(fixture(1.0, 1.0, [f32::MIN_POSITIVE, 0.0]), 48000, 1).unwrap();
        assert_eq!(g.familias[0].espera, u64::MAX);
        g.indice = u64::MAX;
        for _ in 0..100 {
            assert!(g.siguiente_muestra().is_finite());
        }
        assert_eq!(g.indice, u64::MAX);
        let f = &mut g.familias[0];
        f.tasa = 40.0;
        for v in &mut f.voces {
            v.activa = true;
            v.edad = 0;
            v.ataque = 10;
            v.caida = 20;
        }
        f.espera = 1;
        f.muestra();
        assert_eq!(f.voces.len(), 6);
        assert!(f.voces.iter().all(|v| v.edad == 1));
        for _ in 0..30 {
            f.muestra();
        }
        assert!(f.voces.iter().all(|v| !v.activa));
        assert_eq!(f.muestra(), 0.0);
    }

    #[test]
    fn bloques_no_cambian_la_secuencia() {
        let config = fixture(0.25, 0.18, [8.0, 0.6]);
        let render = |bloque: usize| {
            let mut g = GeneradorFogata::nuevo(config.clone(), 48000, 5).unwrap();
            let mut muestras = Vec::new();
            while muestras.len() < 48000 * 20 {
                for _ in 0..bloque.min(48000 * 20 - muestras.len()) {
                    muestras.push(g.siguiente_muestra());
                }
            }
            muestras
        };
        let referencia = render(1_usize);
        for bloque in [64, 256, 1024] {
            assert_eq!(referencia, render(bloque));
        }
    }
}
