use super::tiempo::HorasViaje;
use crate::acciones::{Resultado, fallo, sortear};
use crate::azar::FuenteAzar;
use crate::{IdActor, IdLugar, Mundo};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Medio {
    Pie,
    Caballo,
    Camello,
    Barco,
}

pub(crate) fn lugar_tiene(mundo: &Mundo, id: IdLugar, clave: &str) -> bool {
    mundo
        .lugar(id)
        .is_some_and(|l| l.caracteristicas().any(|c| c.como_str() == clave))
}
pub(crate) fn medio(
    mundo: &Mundo,
    a: IdLugar,
    b: IdLugar,
    actores: usize,
    caballos: u64,
    camellos: u64,
) -> Medio {
    if [a, b].iter().any(|&l| lugar_tiene(mundo, l, "acuatico")) {
        return Medio::Barco;
    }
    if caballos >= actores as u64
        && ![a, b]
            .iter()
            .any(|&l| lugar_tiene(mundo, l, "desierto") && lugar_tiene(mundo, l, "vasto"))
    {
        return Medio::Caballo;
    }
    if camellos >= actores as u64 {
        Medio::Camello
    } else {
        Medio::Pie
    }
}
pub(crate) fn estado_pie(mundo: &Mundo, actores: &[IdActor]) -> u64 {
    actores
        .iter()
        .filter_map(|&id| mundo.actor(id))
        .map(|a| {
            let tiene = |s| a.caracteristicas().any(|c| c.como_str() == s);
            u64::from(tiene("esguince")) * 4 + u64::from(tiene("cansado")) * 6
        })
        .max()
        .unwrap_or(0)
}
pub(crate) fn mitad(
    mundo: &Mundo,
    lugar: IdLugar,
    medio: Medio,
    animales: u64,
    estado: u64,
) -> Resultado<u64> {
    if mundo.lugar(lugar).is_none() {
        return Err(fallo("Lugar de mitad desconocido"));
    }
    let lento = ["montanoso", "pantanoso", "boscoso"]
        .iter()
        .any(|c| lugar_tiene(mundo, lugar, c));
    let base = match medio {
        Medio::Pie => 84,
        Medio::Caballo if !lento => 48,
        Medio::Camello if !lento => 60,
        Medio::Barco => 24,
        _ => 84,
    };
    let base = if animales > 0 && medio != Medio::Barco {
        base.max(60)
    } else {
        base
    };
    let pequeno = lugar_tiene(mundo, lugar, "pequeno");
    let vasto = lugar_tiene(mundo, lugar, "vasto");
    if pequeno && vasto {
        return Err(fallo("tamaño contradictorio"));
    }
    let horas = if pequeno {
        base / 2
    } else if vasto {
        base * 2
    } else {
        base
    };
    Ok(horas + if medio == Medio::Pie { estado } else { 0 })
}
pub(crate) fn tiempos(
    mundo: &Mundo,
    ruta: &[IdLugar],
    actores: &[IdActor],
    caballos: u64,
    camellos: u64,
    animales: u64,
) -> Resultado<Vec<[u64; 2]>> {
    ruta.windows(2)
        .map(|p| {
            let m = medio(mundo, p[0], p[1], actores.len(), caballos, camellos);
            Ok([
                mitad(mundo, p[0], m, animales, estado_pie(mundo, actores))?,
                mitad(mundo, p[1], m, animales, estado_pie(mundo, actores))?,
            ])
        })
        .collect()
}
pub(crate) fn sumar_horas(t: &[[u64; 2]]) -> Resultado<u64> {
    t.iter().flatten().try_fold(0_u64, |s, &h| {
        s.checked_add(h)
            .ok_or_else(|| fallo("previsión desbordada"))
    })
}
pub(crate) fn rapida(
    mundo: &Mundo,
    origen: IdLugar,
    destino: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Resultado<Vec<IdLugar>> {
    let mut dist = BTreeMap::from([(origen, 0_u64)]);
    let mut prev: BTreeMap<IdLugar, Vec<IdLugar>> = BTreeMap::new();
    let mut cola = BinaryHeap::from([Reverse((0_u64, origen))]);
    while let Some(Reverse((d, a))) = cola.pop() {
        if dist.get(&a) != Some(&d) {
            continue;
        }
        for &b in mundo
            .lugares_conectados(a)
            .ok_or_else(|| fallo("índice de conexiones ausente"))?
        {
            let m = medio(mundo, a, b, 1, 1, 1);
            let costo = mitad(mundo, a, m, 0, 0)? + mitad(mundo, b, m, 0, 0)?;
            let nd = d
                .checked_add(costo)
                .ok_or_else(|| fallo("ruta desbordada"))?;
            match dist.get(&b).copied() {
                Some(v) if v < nd => {}
                Some(v) if v == nd => {
                    prev.entry(b).or_default().push(a);
                }
                _ => {
                    dist.insert(b, nd);
                    prev.insert(b, vec![a]);
                    cola.push(Reverse((nd, b)));
                }
            }
        }
    }
    if !dist.contains_key(&destino) {
        return Ok(Vec::new());
    }
    let mut ruta = vec![destino];
    let mut actual = destino;
    while actual != origen {
        let padres = prev
            .get(&actual)
            .ok_or_else(|| fallo("ruta inconsistente"))?;
        actual = padres[sortear(azar, padres.len())?];
        ruta.push(actual);
    }
    ruta.reverse();
    Ok(ruta)
}
pub(crate) fn larga(
    mundo: &Mundo,
    corta: &[IdLugar],
    azar: &mut dyn FuenteAzar,
) -> Resultado<Vec<IdLugar>> {
    let c = corta.len();
    let min = c + c / 4 + 1;
    let max = c.saturating_mul(2).saturating_sub(1);
    let mut ruta = corta.to_vec();
    while ruta.len() < min {
        let mut opciones = Vec::new();
        for i in 0..ruta.len().saturating_sub(1) {
            let a = ruta[i];
            let b = ruta[i + 1];
            let excluidos: BTreeSet<_> =
                ruta.iter().copied().filter(|&x| x != a && x != b).collect();
            let mut padres = BTreeMap::new();
            let mut vistos = BTreeSet::from([a]);
            let mut cola = VecDeque::from([a]);
            while let Some(x) = cola.pop_front() {
                let mut vecinos = mundo.lugares_conectados(x).unwrap_or(&[]).to_vec();
                for j in (1..vecinos.len()).rev() {
                    let k = sortear(azar, j + 1)?;
                    vecinos.swap(j, k);
                }
                for y in vecinos {
                    if (x == a && y == b)
                        || (x == b && y == a)
                        || excluidos.contains(&y)
                        || !vistos.insert(y)
                    {
                        continue;
                    }
                    padres.insert(y, x);
                    cola.push_back(y);
                }
            }
            if !padres.contains_key(&b) {
                continue;
            }
            let mut desvio = vec![b];
            let mut x = b;
            while x != a {
                x = *padres
                    .get(&x)
                    .ok_or_else(|| fallo("desvío inconsistente"))?;
                desvio.push(x);
            }
            desvio.reverse();
            if ruta.len() + desvio.len() - 2 <= max {
                opciones.push((i, desvio));
            }
        }
        if opciones.is_empty() {
            break;
        }
        let (i, desvio) = opciones.swap_remove(sortear(azar, opciones.len())?);
        ruta.splice(i..=i + 1, desvio);
    }
    Ok(ruta)
}
pub(crate) fn total(t: &[[u64; 2]]) -> Resultado<HorasViaje> {
    Ok(HorasViaje::horas(sumar_horas(t)?))
}
