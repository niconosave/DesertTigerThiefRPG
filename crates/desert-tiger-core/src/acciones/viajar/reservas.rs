use super::tiempo::HorasViaje;
use crate::acciones::{Resultado, fallo, sortear};
use crate::azar::FuenteAzar;

pub const APORTE: [u64; 4] = [2, 4, 1, 8];
pub const PESO: [u64; 4] = [10, 12, 15, 7];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReservasViaje {
    pub comidas: [u64; 4],
    pub botas: Vec<u16>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RegistroConsumo {
    pub comidas: [u64; 4],
    pub agua_ml: u64,
    pub jornadas: u64,
    pub sin_alimento: Option<crate::TiempoAccion>,
    pub sin_agua: Option<crate::TiempoAccion>,
}

pub(crate) fn sumar(a: u64, b: u64) -> Resultado<u64> {
    a.checked_add(b).ok_or_else(|| fallo("cantidad desbordada"))
}
pub(crate) fn multiplicar(a: u64, b: u64) -> Resultado<u64> {
    a.checked_mul(b).ok_or_else(|| fallo("cantidad desbordada"))
}

impl ReservasViaje {
    pub fn alimento(&self) -> Resultado<u64> {
        self.comidas
            .iter()
            .zip(APORTE)
            .try_fold(0, |s, (&n, a)| sumar(s, multiplicar(n, a)?))
    }
    pub fn agua(&self) -> Resultado<u64> {
        self.botas
            .iter()
            .try_fold(0, |s, &v| sumar(s, u64::from(v)))
    }
    pub fn peso(&self) -> Resultado<u64> {
        let comida = self
            .comidas
            .iter()
            .zip(PESO)
            .try_fold(0, |s, (&n, p)| sumar(s, multiplicar(n, p)?))?;
        sumar(
            comida,
            sumar(
                self.agua()?,
                multiplicar(
                    u64::try_from(self.botas.len()).map_err(|_| fallo("demasiadas botas"))?,
                    100,
                )?,
            )?,
        )
    }
    pub fn agregar_agua(&mut self, ml: u64) -> Resultado<()> {
        let cantidad = ml.div_ceil(3000);
        let cantidad = usize::try_from(cantidad).map_err(|_| fallo("demasiadas botas"))?;
        self.botas
            .try_reserve(cantidad)
            .map_err(|_| fallo("sin memoria para botas"))?;
        self.botas.extend(std::iter::repeat_n(3000, cantidad));
        Ok(())
    }
    pub fn incorporar(&mut self, otras: &Self) -> Resultado<()> {
        for (n, &extra) in self.comidas.iter_mut().zip(&otras.comidas) {
            *n = sumar(*n, extra)?;
        }
        self.botas
            .try_reserve(otras.botas.len())
            .map_err(|_| fallo("sin memoria para botas"))?;
        self.botas.extend(&otras.botas);
        Ok(())
    }
    pub fn alcanza(&self, actores: u64, jornadas: u64) -> Resultado<bool> {
        Ok(
            self.alimento()? >= multiplicar(multiplicar(4, actores)?, jornadas)?
                && self.agua()? >= multiplicar(multiplicar(500, actores)?, jornadas)?,
        )
    }
    pub fn agotada(&self) -> Resultado<bool> {
        Ok(self.alimento()? == 0 || self.agua()? == 0)
    }
}

pub(crate) fn consumir(
    reservas: &mut ReservasViaje,
    resto: HorasViaje,
    duracion: HorasViaje,
    actores: usize,
    azar: &mut dyn FuenteAzar,
) -> Resultado<(HorasViaje, RegistroConsumo)> {
    let total = resto.sumar(duracion)?;
    let jornadas = u64::try_from(total.jornadas()).map_err(|_| fallo("demasiadas jornadas"))?;
    let mut registro = RegistroConsumo {
        jornadas,
        ..RegistroConsumo::default()
    };
    for jornada in 1..=jornadas {
        let mut falta_comida = false;
        let mut falta_agua = false;
        for _ in 0..actores {
            let mut pagado = 0;
            while pagado < 4 {
                let tipos: Vec<_> = reservas
                    .comidas
                    .iter()
                    .enumerate()
                    .filter(|(_, n)| **n > 0)
                    .map(|(i, _)| i)
                    .collect();
                if tipos.is_empty() {
                    falta_comida = true;
                    break;
                }
                let tipo = tipos[sortear(azar, tipos.len())?];
                reservas.comidas[tipo] -= 1;
                registro.comidas[tipo] = sumar(registro.comidas[tipo], 1)?;
                pagado += APORTE[tipo];
            }
            let mut faltante = 500_u16;
            for bota in &mut reservas.botas {
                let tomar = (*bota).min(faltante);
                *bota -= tomar;
                faltante -= tomar;
                registro.agua_ml = sumar(registro.agua_ml, u64::from(tomar))?;
                if faltante == 0 {
                    break;
                }
            }
            falta_agua |= faltante > 0;
        }
        let desde_frontera = HorasViaje(total.0 - HorasViaje::horas(jornada * 24).0).tiempo()?;
        if falta_comida && registro.sin_alimento.is_none() {
            registro.sin_alimento = Some(desde_frontera);
        }
        if falta_agua && registro.sin_agua.is_none() {
            registro.sin_agua = Some(desde_frontera);
        }
    }
    Ok((total.resto(), registro))
}

#[derive(Default)]
pub(crate) struct LoteTentativo {
    piezas: Vec<usize>,
}
impl LoteTentativo {
    pub fn materializar(
        &mut self,
        alimento_x10: u64,
        agua_x10: u64,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<ReservasViaje> {
        let mut lote = ReservasViaje::default();
        let mut aporte = 0;
        let mut posicion = 0;
        while multiplicar(aporte, 10)? < alimento_x10 {
            if posicion == self.piezas.len() {
                self.piezas.push(sortear(azar, 4)?);
            }
            let tipo = self.piezas[posicion];
            posicion += 1;
            lote.comidas[tipo] = sumar(lote.comidas[tipo], 1)?;
            aporte = sumar(aporte, APORTE[tipo])?;
        }
        // Comparación racional: cada bota contiene 30.000 décimas de ml.
        let botas = agua_x10.div_ceil(30_000);
        lote.agregar_agua(multiplicar(botas, 3000)?)?;
        Ok(lote)
    }
}
