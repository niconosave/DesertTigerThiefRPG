use super::{
    configuracion::ConfiguracionMusica, fuentes::Azar, partitura::FRECUENCIA, transicion::Rampa,
};

pub(super) const FINALIZADA: u32 = 1 << 31;

pub(super) struct ReproductorMusica {
    piezas: super::preparacion_musica::RepertorioCompartido,
    seleccion: usize,
    anterior: Option<usize>,
    azar: Azar,
    volumen: f32,
    frecuencia: u32,
    paso: f64,
    posicion: f64,
    ultima_orden: u32,
    evento: u32,
    finalizada: bool,
    ganancia: Rampa,
}

impl ReproductorMusica {
    pub(super) fn nuevo(config: ConfiguracionMusica, frecuencia: u32) -> Self {
        Self {
            piezas: config.piezas,
            seleccion: 0,
            anterior: None,
            azar: Azar(0x7265706572746f72),
            volumen: config.volumen,
            frecuencia,
            paso: f64::from(FRECUENCIA) / f64::from(frecuencia),
            posicion: 0.0,
            ultima_orden: 0,
            evento: 0,
            finalizada: false,
            ganancia: Rampa::default(),
        }
    }
    pub(super) fn ordenar(&mut self, evento: u32) {
        if evento == self.ultima_orden {
            return;
        }
        let piezas = self.piezas.get().and_then(|r| r.as_ref().ok());
        if evento != 0 && piezas.is_none_or(|r| r.piezas.is_empty() || !r.audible) {
            return;
        }
        self.ultima_orden = evento;
        if evento == 0 {
            self.cancelar(150);
        } else {
            let cantidad = piezas.map_or(0, |r| r.piezas.len());
            self.seleccion = match (cantidad, self.anterior) {
                (0 | 1, _) | (_, None) => 0,
                (cantidad, Some(anterior)) => {
                    let elegida = (self.azar.siguiente() % (cantidad - 1) as u64) as usize;
                    elegida + usize::from(elegida >= anterior)
                }
            };
            self.anterior = Some(self.seleccion);
            self.evento = evento;
            self.posicion = 0.0;
            self.finalizada = false;
            self.ganancia = Rampa::default();
            self.ganancia.dirigir(true, 40, self.frecuencia);
        }
    }
    pub(super) fn cancelar(&mut self, ms: u32) {
        self.ultima_orden = 0;
        self.ganancia.dirigir(false, ms, self.frecuencia);
        if self.en_silencio() {
            self.evento = 0;
        }
    }
    pub(super) fn siguiente(&mut self) -> f32 {
        let ganancia = self.ganancia.avanzar();
        if self.ganancia.silencio() {
            self.evento = 0;
            return 0.0;
        }
        if self.evento == 0 || self.finalizada {
            return 0.0;
        }
        let indice = self.posicion as usize;
        let Some(muestras) = self
            .piezas
            .get()
            .and_then(|r| r.as_ref().ok())
            .and_then(|r| r.piezas.get(self.seleccion))
        else {
            self.finalizada = true;
            return 0.0;
        };
        let Some(&a) = muestras.get(indice) else {
            self.finalizada = true;
            return 0.0;
        };
        let b = muestras.get(indice + 1).copied().unwrap_or(0.0);
        let muestra = a + (b - a) * self.posicion.fract() as f32;
        self.posicion += self.paso;
        if self.posicion >= muestras.len() as f64 {
            self.finalizada = true;
        }
        muestra * self.volumen * ganancia
    }
    pub(super) fn estado(&self) -> u64 {
        if self.evento == 0 {
            return 0;
        }
        let posicion = if self.finalizada {
            FINALIZADA
        } else {
            self.posicion as u32
        };
        (u64::from(self.evento) << 32) | u64::from(posicion)
    }
    pub(super) fn en_silencio(&self) -> bool {
        self.evento == 0 || self.finalizada || self.ganancia.silencio()
    }
}

pub(super) fn decodificar(estado: u64) -> Option<crate::musica_campamento::ProgresoMusica> {
    let evento = (estado >> 32) as u32;
    if evento == 0 {
        return None;
    }
    let posicion = estado as u32;
    Some(crate::musica_campamento::ProgresoMusica {
        evento,
        finalizada: posicion & FINALIZADA != 0,
        transcurrido: std::time::Duration::from_micros(
            u64::from(posicion & !FINALIZADA) * 1_000_000 / u64::from(FRECUENCIA),
        ),
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn repertorio_varia_sin_repetir_y_no_cambia_durante_una_pieza() {
        for cantidad in [2, 3] {
            let mut config = ConfiguracionMusica::fixture();
            config.piezas = super::super::preparacion_musica::Repertorio::listo(
                [0.1, -0.2, 0.3]
                    .into_iter()
                    .take(cantidad)
                    .map(|amplitud| std::sync::Arc::from(vec![amplitud; 4800]))
                    .collect(),
            );
            let mut r = ReproductorMusica::nuevo(config, 48000);
            let mut anterior = None;
            for evento in 1..=16 {
                r.ordenar(evento);
                let elegida = r.seleccion;
                assert_ne!(anterior, Some(elegida));
                let mut muestra = 0.0;
                for _ in 0..2048 {
                    muestra = r.siguiente();
                    r.ordenar(evento);
                    assert_eq!(r.seleccion, elegida);
                }
                assert_eq!(muestra, [0.05, -0.1, 0.15][elegida]);
                for _ in 0..4800 {
                    r.siguiente();
                }
                assert!(decodificar(r.estado()).unwrap().finalizada);
                r.ordenar(0); // Se conserva la última pieza también al salir/reentrar.
                anterior = Some(elegida);
            }
        }
    }

    #[test]
    fn remuestreo_fin_confirmado_y_orden_repetida_no_reinicia() {
        for frecuencia in [8000, 44100, 48000, 192000] {
            let mut r = ReproductorMusica::nuevo(ConfiguracionMusica::fixture(), frecuencia);
            r.ordenar(7);
            let mut cuadros = 0;
            while !r.finalizada {
                let x = r.siguiente();
                assert!(x.is_finite() && x.abs() <= 0.8);
                cuadros += 1;
                assert!(cuadros <= frecuencia / 10 + 2);
                r.ordenar(7);
            }
            assert!((i64::from(cuadros) - i64::from(frecuencia / 10)).abs() <= 1);
            let estado = decodificar(r.estado()).unwrap();
            assert!(estado.finalizada && estado.evento == 7);
            for _ in 0..100 {
                assert_eq!(r.siguiente(), 0.0);
            }
            assert_eq!(r.estado(), (7_u64 << 32) | u64::from(FINALIZADA));
            r.ordenar(8);
            assert_eq!(decodificar(r.estado()).unwrap().evento, 8);
            assert!(!r.finalizada);
        }
    }

    #[test]
    fn cancelacion_y_cierre_suaves_silencian_la_pieza() {
        for ms in [150, 20] {
            let mut r = ReproductorMusica::nuevo(ConfiguracionMusica::fixture(), 48000);
            r.ordenar(1);
            for _ in 0..1000 {
                r.siguiente();
            }
            r.cancelar(ms);
            for _ in 0..48000 * ms / 1000 + 1 {
                r.siguiente();
            }
            assert!(r.en_silencio());
            assert_eq!(r.estado(), 0);
            assert_eq!(r.siguiente(), 0.0);
        }
    }
}
