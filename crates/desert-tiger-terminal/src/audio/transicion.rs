#[derive(Clone, Copy, Default)]
pub(super) struct Rampa {
    pub(super) ganancia: f32,
    origen: f32,
    destino: f32,
    pub(super) paso: u64,
    duracion: u64,
}
impl Rampa {
    pub(super) fn dirigir(&mut self, activo: bool, ms: u32, frecuencia: u32) {
        self.origen = self.ganancia;
        self.destino = if activo { 1.0 } else { 0.0 };
        self.paso = 0;
        self.duracion = (ms as u64 * frecuencia as u64).div_ceil(1000);
    }
    pub(super) fn avanzar(&mut self) -> f32 {
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
    pub(super) fn silencio(&self) -> bool {
        self.ganancia == 0.0 && self.destino == 0.0
    }
}
