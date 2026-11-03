use super::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

fn habilita_rapida(mundo: &Mundo, actor: IdActor, ruta: &[IdLugar]) -> bool {
    tiene(mundo, actor, "viajero")
        || tiene(mundo, actor, "explorador")
        || ruta
            .iter()
            .filter(|&&l| mundo.actor_conoce_lugar(actor, l).unwrap_or(false))
            .count()
            >= ruta.len().div_ceil(2)
}

fn mitades(mundo: &Mundo, viajeros: &[IdActor], a: IdLugar, b: IdLugar) -> Resultado<[u64; 2]> {
    let n = viajeros.len() as u64;
    Ok(rutas::tiempos(mundo, &[a, b], viajeros, n, n, 0)?[0])
}

pub(super) fn tiempo(mundo: &Mundo, viajeros: &[IdActor], ruta: &[IdLugar]) -> Resultado<u64> {
    let n = viajeros.len() as u64;
    rutas::sumar_horas(&rutas::tiempos(mundo, ruta, viajeros, n, n, 0)?)
}

fn favorece(
    mundo: &Mundo,
    conocidos: &BTreeSet<IdLugar>,
    perfil: PerfilRuta,
    lugar: IdLugar,
    medio: Medio,
) -> bool {
    match perfil {
        PerfilRuta::Rapida => false,
        PerfilRuta::Conocidos => conocidos.contains(&lugar),
        PerfilRuta::Montana => rutas::lugar_tiene(mundo, lugar, "montanoso"),
        PerfilRuta::Caballo => {
            medio == Medio::Caballo
                && !["montanoso", "boscoso", "pantanoso"]
                    .iter()
                    .any(|c| rutas::lugar_tiene(mundo, lugar, c))
        }
    }
}

// Todos los costes son positivos: los predecesores nunca forman ciclos.
pub(super) fn buscar(
    mundo: &Mundo,
    viajeros: &[IdActor],
    origen: IdLugar,
    destino: IdLugar,
    perfil: PerfilRuta,
    conocidos: &BTreeSet<IdLugar>,
    factor: u64,
) -> Resultado<Vec<IdLugar>> {
    let mut dist = BTreeMap::from([(origen, (0_u64, 0_u64))]);
    let mut prev = BTreeMap::new();
    let mut cola = BinaryHeap::from([Reverse(((0_u64, 0_u64), origen))]);
    while let Some(Reverse((d, a))) = cola.pop() {
        if dist.get(&a) != Some(&d) {
            continue;
        }
        for &b in mundo
            .lugares_conectados(a)
            .ok_or_else(|| fallo("índice de conexiones ausente"))?
        {
            let medio = rutas::medio(
                mundo,
                a,
                b,
                viajeros.len(),
                viajeros.len() as u64,
                viajeros.len() as u64,
            );
            let h = mitades(mundo, viajeros, a, b)?;
            let mut peso = 0_u64;
            for (l, horas) in [(a, h[0]), (b, h[1])] {
                let multiplicador = if favorece(mundo, conocidos, perfil, l, medio) {
                    factor
                } else {
                    4
                };
                peso = peso
                    .checked_add(
                        horas
                            .checked_mul(multiplicador)
                            .ok_or_else(|| fallo("peso de ruta desbordado"))?,
                    )
                    .ok_or_else(|| fallo("peso de ruta desbordado"))?;
            }
            let nd = (
                d.0.checked_add(peso)
                    .ok_or_else(|| fallo("peso de ruta desbordado"))?,
                d.1.checked_add(h[0])
                    .and_then(|s| s.checked_add(h[1]))
                    .ok_or_else(|| fallo("tiempo de ruta desbordado"))?,
            );
            match dist.get(&b) {
                Some(&v) if v < nd => {}
                Some(&v) if v == nd => {
                    if prev.get(&b).is_none_or(|&p| a < p) {
                        prev.insert(b, a);
                    }
                }
                _ => {
                    dist.insert(b, nd);
                    prev.insert(b, a);
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
        actual = *prev
            .get(&actual)
            .ok_or_else(|| fallo("predecesor de ruta ausente"))?;
        ruta.push(actual);
    }
    ruta.reverse();
    Ok(ruta)
}

pub(super) fn generar(
    mundo: &Mundo,
    viajeros: &[IdActor],
    rapida: &[IdLugar],
    larga: &[IdLugar],
) -> Resultado<Vec<PropuestaRuta>> {
    let (&origen, &destino) = rapida
        .first()
        .zip(rapida.last())
        .ok_or_else(|| fallo("ruta rápida vacía"))?;
    let minimo = tiempo(mundo, viajeros, rapida)?;
    let maximo = tiempo(mundo, viajeros, larga)?;
    let mut agrupadas: BTreeMap<Vec<IdLugar>, Vec<RespaldoRuta>> = BTreeMap::new();
    let mut cache = BTreeMap::new();
    for &actor in viajeros {
        let conocidos: BTreeSet<_> = mundo
            .lugares_conocidos(actor)
            .unwrap_or(&[])
            .iter()
            .copied()
            .collect();
        for perfil in [
            PerfilRuta::Rapida,
            PerfilRuta::Conocidos,
            PerfilRuta::Montana,
            PerfilRuta::Caballo,
        ] {
            let rasgos: &[&str] = match perfil {
                PerfilRuta::Rapida => &["viajero", "explorador"],
                PerfilRuta::Montana => &["montaraz"],
                PerfilRuta::Caballo => &["jinete"],
                PerfilRuta::Conocidos => &[],
            };
            if (perfil == PerfilRuta::Rapida && !habilita_rapida(mundo, actor, rapida))
                || (matches!(perfil, PerfilRuta::Montana | PerfilRuta::Caballo)
                    && !tiene(mundo, actor, rasgos[0]))
                || (perfil == PerfilRuta::Conocidos && conocidos.is_empty())
            {
                continue;
            }
            let clave = (
                perfil,
                if perfil == PerfilRuta::Conocidos {
                    conocidos.clone()
                } else {
                    BTreeSet::new()
                },
            );
            if !cache.contains_key(&clave) {
                let rutas = if perfil == PerfilRuta::Rapida {
                    vec![rapida.to_vec()]
                } else {
                    [3, 1]
                        .iter()
                        .map(|&f| buscar(mundo, viajeros, origen, destino, perfil, &conocidos, f))
                        .collect::<Resultado<Vec<_>>>()?
                };
                cache.insert(clave.clone(), rutas);
            }
            for ruta in &cache[&clave] {
                if ruta.first() != Some(&origen)
                    || ruta.last() != Some(&destino)
                    || ruta.iter().collect::<BTreeSet<_>>().len() != ruta.len()
                    || ruta.windows(2).any(|p| {
                        !mundo
                            .lugares_conectados(p[0])
                            .unwrap_or(&[])
                            .contains(&p[1])
                    })
                {
                    return Err(fallo("propuesta de ruta inconsistente"));
                }
                let t = tiempo(mundo, viajeros, ruta)?;
                if t < minimo || t > maximo || (t == minimo && !habilita_rapida(mundo, actor, ruta))
                {
                    continue;
                }
                let mut relevantes = BTreeSet::new();
                for p in ruta.windows(2) {
                    let medio = rutas::medio(
                        mundo,
                        p[0],
                        p[1],
                        viajeros.len(),
                        viajeros.len() as u64,
                        viajeros.len() as u64,
                    );
                    for &l in p {
                        if perfil == PerfilRuta::Rapida {
                            if conocidos.contains(&l) {
                                relevantes.insert(l);
                            }
                        } else if l != origen
                            && l != destino
                            && favorece(mundo, &conocidos, perfil, l, medio)
                        {
                            relevantes.insert(l);
                        }
                    }
                }
                if perfil != PerfilRuta::Rapida && relevantes.is_empty() {
                    continue;
                }
                let respaldo = RespaldoRuta {
                    actor,
                    perfil,
                    caracteristicas_habilitantes: rasgos
                        .iter()
                        .filter(|&&c| tiene(mundo, actor, c))
                        .map(|&c| {
                            crate::Caracteristica::nueva(c.to_owned())
                                .map_err(|_| fallo("rasgo de perfil inválido"))
                        })
                        .collect::<Resultado<Vec<_>>>()?,
                    lugares_relevantes: ruta
                        .iter()
                        .filter(|l| relevantes.contains(l))
                        .copied()
                        .collect(),
                };
                let respaldos = agrupadas.entry(ruta.clone()).or_default();
                if !respaldos.contains(&respaldo) {
                    respaldos.push(respaldo);
                }
            }
        }
    }
    let mut salida = agrupadas
        .into_iter()
        .map(|(ruta, mut respaldos)| {
            respaldos.sort_by_key(|r| (r.actor, r.perfil));
            Ok((tiempo(mundo, viajeros, &ruta)?, ruta, respaldos))
        })
        .collect::<Resultado<Vec<_>>>()?;
    salida.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    salida
        .into_iter()
        .enumerate()
        .map(|(i, (t, ruta, respaldos))| {
            Ok(PropuestaRuta {
                id: IdPropuestaRuta(u32::try_from(i).map_err(|_| fallo("demasiadas propuestas"))?),
                ruta,
                respaldos,
                previsto: HorasViaje::horas(t).tiempo()?,
            })
        })
        .collect()
}
