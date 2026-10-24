use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

use crate::{IdActor, IdLugar, Mundo};

use super::situaciones::exposicion_conocida;

type ClaveCosteRuta = (u32, u32, usize, Vec<IdLugar>);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Candidato {
    dias_maximos: u32,
    exposicion: u32,
    ruta: Vec<IdLugar>,
}

impl Ord for Candidato {
    fn cmp(&self, otro: &Self) -> Ordering {
        otro.dias_maximos
            .cmp(&self.dias_maximos)
            .then_with(|| otro.exposicion.cmp(&self.exposicion))
            .then_with(|| otro.ruta.len().cmp(&self.ruta.len()))
            .then_with(|| otro.ruta.cmp(&self.ruta))
    }
}

impl PartialOrd for Candidato {
    fn partial_cmp(&self, otro: &Self) -> Option<Ordering> {
        Some(self.cmp(otro))
    }
}

pub(crate) fn ruta_ponderada(
    mundo: &Mundo,
    viajeros: &[IdActor],
    origen: IdLugar,
    destino: IdLugar,
) -> Option<Vec<IdLugar>> {
    mundo.lugar(origen)?;
    mundo.lugar(destino)?;
    let mut mejores: Vec<Option<ClaveCosteRuta>> = vec![None; mundo.lugares().len()];
    let mut pendientes = BinaryHeap::new();
    pendientes.push(Candidato {
        dias_maximos: 0,
        exposicion: 0,
        ruta: vec![origen],
    });

    while let Some(candidato) = pendientes.pop() {
        let actual = *candidato.ruta.last()?;
        let indice = actual.indice()?;
        let clave = (
            candidato.dias_maximos,
            candidato.exposicion,
            candidato.ruta.len(),
            candidato.ruta.clone(),
        );
        if mejores[indice]
            .as_ref()
            .is_some_and(|mejor| mejor <= &clave)
        {
            continue;
        }
        mejores[indice] = Some(clave);
        if actual == destino {
            return Some(candidato.ruta);
        }
        for &vecino in mundo.lugares_conectados(actual)? {
            if candidato.ruta.contains(&vecino) {
                continue;
            }
            if !etapa_viable_por_medio(mundo, viajeros, actual, vecino) {
                continue;
            }
            let mut ruta = candidato.ruta.clone();
            ruta.push(vecino);
            let (exposicion, incertidumbre) = coste_etapa(mundo, viajeros, actual, vecino)?;
            pendientes.push(Candidato {
                dias_maximos: candidato.dias_maximos.checked_add(1)?,
                exposicion: candidato
                    .exposicion
                    .checked_add(exposicion.checked_add(incertidumbre)?)?,
                ruta,
            });
        }
    }
    None
}

pub(crate) fn hay_ruta_topologica(mundo: &Mundo, origen: IdLugar, destino: IdLugar) -> bool {
    if mundo.lugar(origen).is_none() || mundo.lugar(destino).is_none() {
        return false;
    }
    let mut visitados = vec![false; mundo.lugares().len()];
    let mut pendientes = VecDeque::from([origen]);
    while let Some(actual) = pendientes.pop_front() {
        if actual == destino {
            return true;
        }
        let Some(indice) = actual.indice() else {
            continue;
        };
        if visitados.get(indice).copied().unwrap_or(true) {
            continue;
        }
        visitados[indice] = true;
        if let Some(vecinos) = mundo.lugares_conectados(actual) {
            pendientes.extend(vecinos.iter().copied());
        }
    }
    false
}

pub(crate) fn coste_ruta(
    mundo: &Mundo,
    viajeros: &[IdActor],
    ruta: &[IdLugar],
) -> Option<(u32, u32, u32)> {
    let etapas = ruta.len().checked_sub(1)?;
    let dias = u32::try_from(etapas).ok()?;
    let mut exposicion = 0_u32;
    let mut incertidumbre = 0_u32;
    for etapa in ruta.windows(2) {
        if !etapa_viable_por_medio(mundo, viajeros, etapa[0], etapa[1]) {
            return None;
        }
        let (riesgo, incierto) = coste_etapa(mundo, viajeros, etapa[0], etapa[1])?;
        exposicion = exposicion.checked_add(riesgo)?;
        incertidumbre = incertidumbre.checked_add(incierto)?;
    }
    Some((dias, exposicion, incertidumbre))
}

fn etapa_viable_por_medio(
    mundo: &Mundo,
    viajeros: &[IdActor],
    origen: IdLugar,
    destino: IdLugar,
) -> bool {
    let Some(origen_lugar) = mundo.lugar(origen) else {
        return false;
    };
    let Some(destino_lugar) = mundo.lugar(destino) else {
        return false;
    };
    let origen_agua = tiene(origen_lugar, "acuatico");
    let destino_agua = tiene(destino_lugar, "acuatico");
    if !destino_agua || origen_agua {
        return true;
    }
    tiene(origen_lugar, "puerto")
        || tiene(origen_lugar, "muelle")
        || viajeros.iter().copied().any(|id| {
            mundo.actor(id).is_some_and(|actor| {
                actor.tiene_caracteristica(&caracteristica("conBote"))
                    || actor.tiene_caracteristica(&caracteristica("conBarco"))
            })
        })
}

fn tiene(lugar: &crate::Lugar, rasgo: &str) -> bool {
    lugar.tiene_caracteristica(&caracteristica(rasgo))
}

fn caracteristica(rasgo: &str) -> crate::Caracteristica {
    crate::Caracteristica::nueva(rasgo.to_owned()).unwrap_or_else(|_| unreachable!())
}

fn coste_etapa(
    mundo: &Mundo,
    viajeros: &[IdActor],
    origen: IdLugar,
    destino: IdLugar,
) -> Option<(u32, u32)> {
    let conocido = viajeros.iter().copied().any(|actor| {
        mundo.actor_conoce_lugar(actor, origen).unwrap_or(false)
            || mundo.actor_conoce_lugar(actor, destino).unwrap_or(false)
    });
    if conocido {
        Some((exposicion_conocida(mundo, origen, destino)?, 0))
    } else {
        Some((0, 1))
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{
        ActorInicial, Caracteristica, DatosInicialesMundo, ExtremoRelacion, Fecha, IdActor, IdMapa,
        LugarInicial, Mes, RelacionInicial, TipoRelacion,
    };

    fn lugar(indice: usize, rasgos: &[&str]) -> LugarInicial {
        LugarInicial {
            id: IdLugar::desde_posicion(indice).expect("ID válido"),
            caracteristicas: rasgos
                .iter()
                .map(|rasgo| Caracteristica::nueva((*rasgo).to_owned()).expect("rasgo válido"))
                .collect(),
        }
    }

    #[test]
    fn minimiza_exposicion_y_desempata_por_ids() {
        let id_actor = IdActor::desde_posicion(0).expect("ID válido");
        let id_companero = IdActor::desde_posicion(1).expect("ID válido");
        let id_lugar = |indice| IdLugar::desde_posicion(indice).expect("ID válido");
        let mut relaciones: Vec<_> = [(0, 1), (1, 3), (0, 2), (2, 3)]
            .into_iter()
            .map(|(origen, destino)| RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id_lugar(origen)),
                destino: ExtremoRelacion::Lugar(id_lugar(destino)),
            })
            .collect();
        relaciones.extend((0..4).map(|destino| RelacionInicial {
            tipo: TipoRelacion::Conoce,
            origen: ExtremoRelacion::Actor(id_actor),
            destino: ExtremoRelacion::Lugar(id_lugar(destino)),
        }));
        let mundo = Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID válido"),
            fecha: Fecha::nueva(2026, Mes::Octubre, 18).expect("fecha válida"),
            lugares: vec![
                lugar(0, &[]),
                lugar(1, &["rocoso"]),
                lugar(2, &[]),
                lugar(3, &[]),
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor,
                    lugar: id_lugar(0),
                    caracteristicas: vec![],
                },
                ActorInicial {
                    id: id_companero,
                    lugar: id_lugar(0),
                    caracteristicas: vec![],
                },
            ],
            actor_jugador: id_actor,
            companeros: vec![id_companero],
            variables_globales: vec![],
            relaciones,
        })
        .expect("Mundo válido");

        assert_eq!(
            ruta_ponderada(&mundo, &[id_actor, id_companero], id_lugar(0), id_lugar(3),),
            Some(vec![id_lugar(0), id_lugar(2), id_lugar(3)])
        );
        assert_eq!(
            mundo.lugares_conocidos(id_actor),
            Some([id_lugar(0), id_lugar(1), id_lugar(2), id_lugar(3)].as_slice())
        );
        assert_eq!(
            mundo
                .lugares_conocidos_por_viajeros(&[id_companero, id_actor])
                .expect("viajeros válidos"),
            vec![id_lugar(0), id_lugar(1), id_lugar(2), id_lugar(3)]
        );
        assert!(
            mundo
                .actor_conoce_lugar(id_actor, id_lugar(1))
                .expect("consulta válida")
        );
    }

    #[test]
    fn no_presume_embarcacion_al_entrar_al_agua() {
        let id_actor = IdActor::desde_posicion(0).expect("ID válido");
        let id_lugar = |indice| IdLugar::desde_posicion(indice).expect("ID válido");
        let construir = |origen: &[&str]| {
            Mundo::nuevo(DatosInicialesMundo {
                mapa: IdMapa::desde_posicion(0).expect("ID válido"),
                fecha: Fecha::nueva(2026, Mes::Octubre, 18).expect("fecha válida"),
                lugares: vec![lugar(0, origen), lugar(1, &["acuatico", "maritimo"])],
                actores: vec![
                    ActorInicial {
                        id: id_actor,
                        lugar: id_lugar(0),
                        caracteristicas: vec![],
                    },
                    ActorInicial {
                        id: IdActor::desde_posicion(1).expect("ID válido"),
                        lugar: id_lugar(0),
                        caracteristicas: vec![],
                    },
                ],
                actor_jugador: id_actor,
                companeros: vec![IdActor::desde_posicion(1).expect("ID válido")],
                variables_globales: vec![],
                relaciones: vec![RelacionInicial {
                    tipo: TipoRelacion::Conectado,
                    origen: ExtremoRelacion::Lugar(id_lugar(0)),
                    destino: ExtremoRelacion::Lugar(id_lugar(1)),
                }],
            })
            .expect("Mundo válido")
        };
        assert_eq!(
            ruta_ponderada(
                &construir(&["costero"]),
                &[id_actor],
                id_lugar(0),
                id_lugar(1)
            ),
            None
        );
        assert_eq!(
            ruta_ponderada(
                &construir(&["puerto"]),
                &[id_actor],
                id_lugar(0),
                id_lugar(1)
            ),
            Some(vec![id_lugar(0), id_lugar(1)])
        );
    }
}
