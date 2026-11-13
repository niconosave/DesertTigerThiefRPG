use super::{
    reservas::{LoteTentativo, PESO, ReservasViaje, multiplicar, sumar},
    rutas,
    tiempo::HorasViaje,
};
use crate::acciones::{Resultado, fallo};
use crate::azar::FuenteAzar;
use crate::{IdActor, IdLugar, Mundo};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub(crate) struct Configuracion {
    pub reservas: ReservasViaje,
    pub animales: u64,
    pub exceso: u64,
    pub referencia: u64,
    pub suplemento: ReservasViaje,
    pub demoras: Vec<[HorasViaje; 2]>,
}

pub(crate) fn exceso(reservas: &ReservasViaje, actores: usize, animales: u64) -> Resultado<u64> {
    let capacidad = sumar(
        multiplicar(actores as u64, 10_000)?,
        multiplicar(animales, 100_000)?,
    )?;
    Ok(reservas.peso()?.saturating_sub(capacidad))
}
pub(crate) fn plan(tiempos: &[[u64; 2]], gramos: u64) -> Resultado<Vec<[HorasViaje; 2]>> {
    tiempos
        .iter()
        .map(|p| {
            Ok([
                HorasViaje::demora(p[0], gramos)?,
                HorasViaje::demora(p[1], gramos)?,
            ])
        })
        .collect()
}
pub(crate) fn suma_plan(plan: &[[HorasViaje; 2]]) -> Resultado<HorasViaje> {
    plan.iter()
        .flatten()
        .try_fold(HorasViaje::CERO, |s, &h| s.sumar(h))
}

pub(crate) fn preparar(
    mundo: &Mundo,
    ruta: &[IdLugar],
    actores: &[IdActor],
    multiplicador: u64,
    azar: &mut dyn FuenteAzar,
) -> Resultado<(Configuracion, Option<Configuracion>)> {
    let a = actores.len() as u64;
    let mut lote = LoteTentativo::default();
    let t0 = rutas::tiempos(mundo, ruta, actores, a, a, 0)?;
    let crear = |lote: &mut LoteTentativo,
                 t: &[[u64; 2]],
                 azar: &mut dyn FuenteAzar|
     -> Resultado<ReservasViaje> {
        let horas = ruta
            .windows(2)
            .zip(t)
            .filter(|(p, _)| !mundo.ofrece_pasaje(p[0], p[1]) && !mundo.es_tramo_pasaje(p[0], p[1]))
            .try_fold(0u64, |total, (_, mitades)| {
                sumar(total, sumar(mitades[0], mitades[1])?)
            })?;
        let dias = sumar(horas, 24)? / 24;
        lote.materializar(
            multiplicar(multiplicar(multiplicar(4, a)?, dias)?, multiplicador)?,
            multiplicar(multiplicar(multiplicar(500, a)?, dias)?, multiplicador)?,
            azar,
        )
    };
    let mut t = t0.clone();
    let mut reservas = crear(&mut lote, &t, azar)?;
    let mut n = exceso(&reservas, actores.len(), 0)?.div_ceil(100_000);
    if n > 0 {
        t = rutas::tiempos(mundo, ruta, actores, a, a, n)?;
        reservas = crear(&mut lote, &t, azar)?;
        n = exceso(&reservas, actores.len(), 0)?.div_ceil(100_000);
    }
    let base = Configuracion {
        reservas,
        animales: n,
        exceso: 0,
        referencia: rutas::sumar_horas(&t)?,
        suplemento: ReservasViaje::default(),
        demoras: plan(&t, 0)?,
    };
    if n == 0 {
        return Ok((base, None));
    }
    let m = n - 1;
    let tiempos = if m == 0 { t0 } else { t };
    let referencia = rutas::sumar_horas(&tiempos)?;
    let inicial = crear(&mut lote, &tiempos, azar)?;
    let mut reservas = inicial.clone();
    let mut q = 0_u64;
    let mut x = exceso(&reservas, actores.len(), m)?;
    let mut demora = HorasViaje::demora(referencia, x)?;
    loop {
        let requeridas = u64::try_from(HorasViaje::horas(referencia).sumar(demora)?.jornadas())
            .map_err(|_| fallo("suplemento fuera de rango"))?
            - referencia / 24;
        if requeridas == q {
            let mut suplemento = ReservasViaje::default();
            suplemento.comidas[0] = multiplicar(multiplicar(2, a)?, q)?;
            suplemento.agregar_agua(multiplicar(multiplicar(500, a)?, q)?)?;
            return Ok((
                base,
                Some(Configuracion {
                    reservas,
                    animales: m,
                    exceso: x,
                    referencia,
                    suplemento,
                    demoras: plan(&tiempos, x)?,
                }),
            ));
        }
        if requeridas < q {
            return Err(fallo("suplemento no monótono"));
        }
        q = requeridas;
        let mut suplemento = ReservasViaje::default();
        suplemento.comidas[0] = multiplicar(multiplicar(2, a)?, q)?;
        suplemento.agregar_agua(multiplicar(multiplicar(500, a)?, q)?)?;
        reservas = inicial.clone();
        reservas.incorporar(&suplemento)?;
        x = exceso(&reservas, actores.len(), m)?;
        let nueva = HorasViaje::demora(referencia, x)?;
        if nueva
            .0
            .checked_sub(demora.0)
            .ok_or_else(|| fallo("demora no monótona"))?
            > HorasViaje::horas(24).0
        {
            return Ok((base, None));
        }
        demora = nueva;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct LoteDescarte {
    pub comidas: [u64; 4],
    pub botas: Vec<usize>,
}
impl LoteDescarte {
    fn cantidad(&self) -> u64 {
        self.comidas.iter().sum::<u64>() + self.botas.len() as u64
    }
    pub(crate) fn quitar(&self, reservas: &mut ReservasViaje) -> Resultado<()> {
        for (n, &quitar) in reservas.comidas.iter_mut().zip(&self.comidas) {
            *n = n
                .checked_sub(quitar)
                .ok_or_else(|| fallo("descarte inválido"))?;
        }
        for &i in self.botas.iter().rev() {
            if i >= reservas.botas.len() {
                return Err(fallo("bota de descarte inexistente"));
            }
            reservas.botas.remove(i);
        }
        Ok(())
    }
}
fn mejor(a: &LoteDescarte, b: &LoteDescarte) -> bool {
    (a.cantidad(), a) < (b.cantidad(), b)
}
fn tabla(grupos: &[(u64, LoteDescarte)], limite: u64) -> Resultado<BTreeMap<u64, LoteDescarte>> {
    let mut tabla = BTreeMap::from([(0, LoteDescarte::default())]);
    for (peso, lote) in grupos {
        let previo: Vec<_> = tabla.iter().map(|(&p, l)| (p, l.clone())).collect();
        for (p, mut l) in previo {
            let nuevo = sumar(p, *peso)?;
            if nuevo > limite {
                continue;
            }
            for (n, &extra) in l.comidas.iter_mut().zip(&lote.comidas) {
                *n = sumar(*n, extra)?;
            }
            l.botas.extend(&lote.botas);
            l.botas.sort_unstable();
            if tabla.get(&nuevo).is_none_or(|anterior| mejor(&l, anterior)) {
                tabla.insert(nuevo, l);
            }
        }
    }
    Ok(tabla)
}
pub(crate) fn descarte(reservas: &ReservasViaje, exceso: u64) -> Resultado<LoteDescarte> {
    if exceso == 0 {
        return Ok(LoteDescarte::default());
    }
    let mut comida = Vec::new();
    let mut agua = Vec::new();
    let mut maximo = 0;
    for (tipo, &n) in reservas.comidas.iter().enumerate() {
        let mut restantes = n;
        let mut bloque = 1_u64;
        while restantes > 0 {
            let usar = restantes.min(bloque);
            let mut l = LoteDescarte::default();
            l.comidas[tipo] = usar;
            comida.push((multiplicar(usar, PESO[tipo])?, l));
            restantes -= usar;
            bloque = bloque.checked_mul(2).unwrap_or(restantes);
        }
        if n > 0 {
            maximo = maximo.max(PESO[tipo]);
        }
    }
    let mut volumenes: BTreeMap<u16, Vec<usize>> = BTreeMap::new();
    for (id, &v) in reservas.botas.iter().enumerate() {
        volumenes.entry(v).or_default().push(id);
        maximo = maximo.max(100 + u64::from(v));
    }
    for (v, ids) in volumenes {
        let mut inicio = 0;
        let mut bloque = 1;
        while inicio < ids.len() {
            let fin = (inicio + bloque).min(ids.len());
            agua.push((
                multiplicar((fin - inicio) as u64, 100 + u64::from(v))?,
                LoteDescarte {
                    comidas: [0; 4],
                    botas: ids[inicio..fin].to_vec(),
                },
            ));
            inicio = fin;
            bloque = bloque.saturating_mul(2);
        }
    }
    let limite = sumar(exceso, maximo.saturating_sub(1))?;
    let tc = tabla(&comida, limite)?;
    let ta = tabla(&agua, limite)?;
    let mut elegido: Option<((u64, u64, u64), LoteDescarte)> = None;
    for (&pc, lc) in &tc {
        let falta = exceso.saturating_sub(pc);
        if let Some((&pa, la)) = ta.range(falta..).next() {
            let peso = sumar(pc, pa)?;
            let mut l = lc.clone();
            l.botas = la.botas.clone();
            // Los bloques binarios representan cantidades, no una preferencia
            // por los IDs incluidos en cada bloque. Elegir los primeros IDs
            // del volumen correspondiente conserva el desempate canónico.
            let mut cantidades = BTreeMap::<u16, usize>::new();
            for &i in &l.botas {
                *cantidades.entry(reservas.botas[i]).or_default() += 1;
            }
            l.botas.clear();
            for (i, &volumen) in reservas.botas.iter().enumerate() {
                if let Some(n) = cantidades.get_mut(&volumen)
                    && *n > 0
                {
                    *n -= 1;
                    l.botas.push(i);
                }
            }
            let clave = (peso, pc.abs_diff(pa), l.cantidad());
            if elegido
                .as_ref()
                .is_none_or(|(k, prev)| clave < *k || (clave == *k && l < *prev))
            {
                elegido = Some((clave, l));
            }
        }
    }
    elegido
        .map(|(_, l)| l)
        .ok_or_else(|| fallo("no existe descarte suficiente"))
}
