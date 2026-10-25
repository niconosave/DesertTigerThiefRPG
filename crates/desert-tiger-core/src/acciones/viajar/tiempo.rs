use crate::acciones::{Resultado, fallo};
use crate::calendario::{PARTES_POR_DIA, PARTES_POR_HORA};
use crate::{Duracion, FraccionDia, TiempoAccion};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HorasViaje(pub(crate) u128);

impl HorasViaje {
    pub const CERO: Self = Self(0);
    pub fn horas(horas: u64) -> Self {
        Self(u128::from(horas) * PARTES_POR_HORA)
    }
    pub fn sumar(self, otra: Self) -> Resultado<Self> {
        self.0
            .checked_add(otra.0)
            .map(Self)
            .ok_or_else(|| fallo("desbordamiento de tiempo"))
    }
    pub fn demora(horas: u64, gramos: u64) -> Resultado<Self> {
        u128::from(gramos)
            .checked_mul(u128::from(gramos))
            .and_then(|x| x.checked_mul(u128::from(horas)))
            .map(Self)
            .ok_or_else(|| fallo("desbordamiento de carga"))
    }
    pub fn jornadas(self) -> u128 {
        self.0 / PARTES_POR_DIA
    }
    pub fn resto(self) -> Self {
        Self(self.0 % PARTES_POR_DIA)
    }
    pub fn tiempo(self) -> Resultado<TiempoAccion> {
        Ok(TiempoAccion {
            duracion: Duracion::desde_dias(
                u32::try_from(self.jornadas())
                    .map_err(|_| fallo("duración fuera de calendario"))?,
            ),
            fraccion: FraccionDia::nueva(
                u32::try_from(self.resto().0).map_err(|_| fallo("fracción inválida"))?,
            )
            .map_err(|_| fallo("fracción inválida"))?,
        })
    }
}
