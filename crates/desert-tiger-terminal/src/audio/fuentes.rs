use fundsp::prelude32::{AttoHash, AudioUnit, highpass_hz, lowpass_hz, pink, white};

// Azar cosmético propio: nunca comparte estado con las reglas del juego.
pub(super) struct Azar(pub(super) u64);
impl Azar {
    pub(super) fn siguiente(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub(super) fn unidad(&mut self) -> f64 {
        // 52 bits más medio paso: ambos extremos quedan excluidos.
        ((self.siguiente() >> 12) as f64 + 0.5) / 4503599627370496.0
    }
    pub(super) fn rango(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.unidad() as f32
    }
}

pub(super) fn fuente(
    rosa: bool,
    minimo: f32,
    maximo: f32,
    frecuencia: u32,
    semilla: u64,
) -> Box<dyn AudioUnit> {
    let q = std::f32::consts::FRAC_1_SQRT_2;
    let maximo = maximo.min(frecuencia as f32 * 0.45);
    let mut nodo: Box<dyn AudioUnit> = if rosa {
        Box::new(pink() >> highpass_hz(minimo, q) >> lowpass_hz(maximo, q))
    } else {
        Box::new(white() >> highpass_hz(minimo, q) >> lowpass_hz(maximo, q))
    };
    nodo.set_sample_rate(frecuencia as f64);
    nodo.ping(false, AttoHash::new(semilla));
    nodo.allocate();
    nodo
}
