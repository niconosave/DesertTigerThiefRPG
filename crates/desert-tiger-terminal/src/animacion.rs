use std::time::Duration;

pub(crate) const PASO_ANIMACION: Duration = Duration::from_millis(250);
pub(crate) const PASO_FOGATA: Duration = Duration::from_millis(50);

#[derive(Default)]
pub(crate) struct RelojVisual {
    activo: Duration,
}

impl RelojVisual {
    pub(crate) fn avanzar(&mut self, transcurrido: Duration) {
        self.activo = self.activo.saturating_add(transcurrido);
    }

    pub(crate) fn instante_ms(&self) -> u64 {
        let ms = self.activo.as_millis() / PASO_FOGATA.as_millis() * PASO_FOGATA.as_millis();
        u64::try_from(ms).unwrap_or(u64::MAX)
    }

    pub(crate) fn paso_250_ms(&self) -> usize {
        usize::try_from(self.activo.as_millis() / PASO_ANIMACION.as_millis()).unwrap_or(usize::MAX)
    }

    pub(crate) fn hasta_siguiente(&self, intervalo: Duration) -> Duration {
        // Los únicos intervalos del bucle son 50 y 250 ms. Conservamos también
        // la fracción inferior al milisegundo al calcular la espera.
        let resto = self.activo.as_nanos() % intervalo.as_nanos();
        intervalo.saturating_sub(Duration::from_nanos(resto as u64))
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn conserva_fracciones_y_las_dos_resoluciones() {
        let mut reloj = RelojVisual::default();
        assert_eq!(reloj.hasta_siguiente(PASO_FOGATA), PASO_FOGATA);
        reloj.avanzar(Duration::from_micros(49_500));
        assert_eq!(reloj.instante_ms(), 0);
        assert_eq!(
            reloj.hasta_siguiente(PASO_FOGATA),
            Duration::from_micros(500)
        );
        reloj.avanzar(Duration::from_micros(500));
        assert_eq!(reloj.instante_ms(), 50);
        assert_eq!(reloj.paso_250_ms(), 0);
        reloj.avanzar(Duration::from_millis(199));
        assert_eq!(reloj.paso_250_ms(), 0);
        reloj.avanzar(Duration::from_millis(1));
        assert_eq!(reloj.paso_250_ms(), 1);
        assert_eq!(reloj.hasta_siguiente(PASO_ANIMACION), PASO_ANIMACION);
    }

    #[test]
    fn demora_larga_muestrea_el_presente_y_satura_sin_fallar() {
        let mut reloj = RelojVisual::default();
        reloj.avanzar(Duration::from_millis(65_029));
        assert_eq!(reloj.instante_ms(), 65_000);
        assert_eq!(reloj.paso_250_ms(), 260);
        assert_eq!(
            reloj.hasta_siguiente(PASO_FOGATA),
            Duration::from_millis(21)
        );
        reloj.avanzar(Duration::MAX);
        reloj.avanzar(Duration::MAX);
        assert_eq!(reloj.instante_ms(), u64::MAX);
    }
}
