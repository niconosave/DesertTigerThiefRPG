use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColectivoInicial {
    pub id: IdColectivo,
    pub lugar: IdLugar,
    pub caracteristicas: Vec<Caracteristica>,
    pub lider: Option<IdActor>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorColectivo {
    pub(super) id: IdColectivo,
    pub(super) lugar: IdLugar,
    caracteristicas: BTreeSet<Caracteristica>,
    lider: Option<IdActor>,
}
impl ActorColectivo {
    pub fn id(&self) -> IdColectivo {
        self.id
    }
    pub fn lugar(&self) -> IdLugar {
        self.lugar
    }
    pub fn lider(&self) -> Option<IdActor> {
        self.lider
    }
    pub fn caracteristicas(&self) -> impl ExactSizeIterator<Item = &Caracteristica> {
        self.caracteristicas.iter()
    }
}
impl Mundo {
    pub fn colectivo(&self, id: IdColectivo) -> Option<&ActorColectivo> {
        self.colectivos.get(id.indice()?).filter(|c| c.id == id)
    }
    pub fn colectivos(&self) -> &[ActorColectivo] {
        &self.colectivos
    }
    pub fn colectivo_de(&self, actor: IdActor) -> Option<IdColectivo> {
        match self.actor(actor)?.ubicacion {
            UbicacionActor::EnColectivo(id) => Some(id),
            _ => None,
        }
    }
    pub fn miembros_colectivo(&self, id: IdColectivo) -> Option<&[IdActor]> {
        self.colectivo(id)?;
        self.miembros_por_colectivo
            .get(id.indice()?)
            .map(Vec::as_slice)
    }
}
pub(super) fn construir(
    datos: Vec<ColectivoInicial>,
    lugares: &[Lugar],
) -> Result<Vec<ActorColectivo>, ErrorConstruirMundo> {
    datos
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            if c.id.indice() != Some(i)
                || !resolver_lugar(lugares, c.lugar).is_some_and(|l| !l.es_region())
            {
                return Err(ErrorConstruirMundo::ColectivoInvalido {
                    detalle: format!("ID o Lugar inválido: {:?}", c.id),
                });
            }
            let caracteristicas = conjunto_caracteristicas(c.caracteristicas).map_err(|_| {
                ErrorConstruirMundo::ColectivoInvalido {
                    detalle: "característica duplicada".into(),
                }
            })?;
            Ok(ActorColectivo {
                id: c.id,
                lugar: c.lugar,
                caracteristicas,
                lider: c.lider,
            })
        })
        .collect()
}
pub(super) fn ubicacion(u: UbicacionActor, cs: &[ActorColectivo]) -> Option<IdLugar> {
    match u {
        UbicacionActor::EnLugar(l) => Some(l),
        UbicacionActor::EnColectivo(c) => {
            cs.get(c.indice()?).filter(|x| x.id == c).map(|x| x.lugar)
        }
        UbicacionActor::FueraDelEscenario => None,
    }
}
pub(super) fn indexar(actores: &[Actor], cantidad: usize) -> Vec<Vec<IdActor>> {
    let mut indice = vec![vec![]; cantidad];
    for a in actores {
        if let UbicacionActor::EnColectivo(c) = a.ubicacion {
            indice[c.indice().expect("ID validado")].push(a.id);
        }
    }
    indice
}
fn lider_valido(c: &ActorColectivo, actores: &[Actor]) -> bool {
    c.lider.is_none_or(|id| {
        resolver_actor(actores, id).is_some_and(|a| {
            a.ubicacion == UbicacionActor::EnColectivo(c.id)
                && !a.caracteristicas.iter().any(|x| x.como_str() == "muerto")
        })
    })
}
pub(super) fn validar_lideres(
    cs: &[ActorColectivo],
    actores: &[Actor],
) -> Result<(), ErrorOperacionMundo> {
    if cs.iter().all(|c| lider_valido(c, actores)) {
        Ok(())
    } else {
        Err(ErrorOperacionMundo::LiderColectivoInvalido)
    }
}
pub(super) fn limpiar_lideres(cs: &mut [ActorColectivo], actores: &[Actor]) {
    for c in cs {
        if !lider_valido(c, actores) {
            c.lider = None;
        }
    }
}
pub(super) fn aplicar(
    op: OperacionMundo,
    actores: &mut [Actor],
    cs: &mut [ActorColectivo],
    lugares: &[Lugar],
    jugador: IdActor,
    grupo: &Grupo,
    cambios: &mut Vec<CambioConfirmado>,
) -> Result<(), ErrorOperacionMundo> {
    use OperacionMundo::*;
    let (id, individual, ingreso) = match &op {
        IngresarActorEnColectivo { actor, colectivo } => (*colectivo, Some(*actor), Some(true)),
        SalirActorDeColectivo { actor, colectivo } => (*colectivo, Some(*actor), Some(false)),
        IngresarGrupoEnColectivo { colectivo } => (*colectivo, None, Some(true)),
        SalirGrupoDeColectivo { colectivo } => (*colectivo, None, Some(false)),
        TrasladarColectivo { colectivo, .. } | EstablecerLiderColectivo { colectivo, .. } => {
            (*colectivo, None, None)
        }
        _ => unreachable!("solo operaciones colectivas"),
    };
    let c = cs
        .get_mut(
            id.indice()
                .ok_or(ErrorOperacionMundo::ColectivoDesconocido { colectivo: id })?,
        )
        .filter(|c| c.id == id)
        .ok_or(ErrorOperacionMundo::ColectivoDesconocido { colectivo: id })?;
    if let Some(ingreso) = ingreso {
        let ids = if let Some(actor) = individual {
            if actor == jugador || grupo.contiene(actor) {
                return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
            }
            vec![actor]
        } else {
            std::iter::once(jugador)
                .chain(grupo.integrantes.iter().copied())
                .collect()
        };
        for &actor in &ids {
            let a = resolver_actor(actores, actor)
                .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
            let valida = if ingreso {
                matches!(a.ubicacion,UbicacionActor::EnLugar(l) if l==c.lugar)
                    || a.ubicacion == UbicacionActor::EnColectivo(id)
            } else {
                a.ubicacion == UbicacionActor::EnColectivo(id)
            };
            if !valida {
                return Err(ErrorOperacionMundo::PertenenciaIncompatible);
            }
        }
        let nueva = if ingreso {
            UbicacionActor::EnColectivo(id)
        } else {
            UbicacionActor::EnLugar(c.lugar)
        };
        if ids
            .iter()
            .any(|&a| resolver_actor(actores, a).is_some_and(|a| a.ubicacion != nueva))
        {
            for &actor in &ids {
                resolver_actor_mut(actores, actor)
                    .expect("Actor validado")
                    .ubicacion = nueva;
            }
            cambios.push(CambioConfirmado::PertenenciaColectivaCambiada {
                actores: ids,
                colectivo: id,
                ingreso,
            });
        }
        return Ok(());
    }
    match op {
        TrasladarColectivo { destino, .. } => {
            if !resolver_lugar(lugares, destino).is_some_and(|l| !l.es_region()) {
                return Err(ErrorOperacionMundo::LugarColectivoInvalido);
            }
            if c.lugar != destino {
                let origen = c.lugar;
                c.lugar = destino;
                cambios.push(CambioConfirmado::ColectivoTrasladado {
                    colectivo: id,
                    origen,
                    destino,
                });
            }
        }
        EstablecerLiderColectivo { lider, .. } => {
            let anterior = c.lider;
            c.lider = lider;
            if !lider_valido(c, actores) {
                return Err(ErrorOperacionMundo::LiderColectivoInvalido);
            }
            if anterior != lider {
                cambios.push(CambioConfirmado::LiderColectivoCambiado {
                    colectivo: id,
                    lider,
                });
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[cfg(test)]
mod pruebas {
    use super::*;
    fn actor(i: usize) -> IdActor {
        IdActor::desde_posicion(i).unwrap()
    }
    fn lugar(i: usize) -> IdLugar {
        IdLugar::desde_posicion(i).unwrap()
    }
    fn compania() -> IdColectivo {
        IdColectivo::desde_posicion(0).unwrap()
    }
    fn datos() -> DatosInicialesMundo {
        DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).unwrap(),
            fecha: Fecha::nueva(2025, crate::Mes::Enero, 1).unwrap(),
            lugares: (0..3)
                .map(|i| LugarInicial {
                    id: lugar(i),
                    caracteristicas: if i == 2 {
                        vec![Caracteristica::nueva("region".into()).unwrap()]
                    } else {
                        vec![]
                    },
                })
                .collect(),
            actores: (0..4)
                .map(|i| ActorInicial {
                    id: actor(i),
                    ubicacion: UbicacionActor::EnLugar(lugar(0)),
                    caracteristicas: vec![],
                })
                .collect(),
            colectivos: vec![ColectivoInicial {
                id: compania(),
                lugar: lugar(0),
                caracteristicas: vec![],
                lider: None,
            }],
            actor_jugador: actor(0),
            companeros: vec![actor(1)],
            variables_globales: vec![],
            relaciones: vec![],
        }
    }
    fn aplicar(
        m: &mut Mundo,
        ops: Vec<OperacionMundo>,
    ) -> Result<CambiosConfirmados, ErrorOperacionMundo> {
        m.aplicar_lote(LoteOperaciones::nuevo(ops).unwrap())
    }
    #[test]
    fn grupo_ingresa_viaja_y_sale_junto_con_indices_y_rollback() {
        use OperacionMundo::*;
        let mut m = Mundo::nuevo(datos()).unwrap();
        aplicar(
            &mut m,
            vec![IngresarGrupoEnColectivo {
                colectivo: compania(),
            }],
        )
        .unwrap();
        assert_eq!(
            m.miembros_colectivo(compania()).unwrap(),
            &[actor(0), actor(1)]
        );
        assert!(
            aplicar(
                &mut m,
                vec![IngresarGrupoEnColectivo {
                    colectivo: compania()
                }]
            )
            .unwrap()
            .como_slice()
            .is_empty()
        );
        let antes = m.clone();
        assert!(
            aplicar(
                &mut m,
                vec![
                    TrasladarColectivo {
                        colectivo: compania(),
                        destino: lugar(1)
                    },
                    SalirGrupoDeColectivo {
                        colectivo: IdColectivo::desde_posicion(99).unwrap()
                    }
                ]
            )
            .is_err()
        );
        assert_eq!(m, antes);
        aplicar(
            &mut m,
            vec![TrasladarColectivo {
                colectivo: compania(),
                destino: lugar(1),
            }],
        )
        .unwrap();
        assert_eq!(m.ubicacion_grupo(), lugar(1));
        assert_eq!(m.ubicacion_actor(actor(1)), Some(lugar(1)));
        assert_eq!(m.ubicacion_actor(actor(2)), Some(lugar(0)));
        assert_eq!(m.actores_en_lugar(lugar(1)).unwrap().count(), 2);
        assert!(m.trasladar(actor(0), lugar(0)).is_err());
        assert!(crate::acciones::destinos_disponibles(&m, actor(0)).is_err());
        assert!(
            aplicar(
                &mut m,
                vec![SepararCompanero {
                    actor: actor(1),
                    lugar: lugar(1)
                }]
            )
            .is_err()
        );
        aplicar(
            &mut m,
            vec![SalirGrupoDeColectivo {
                colectivo: compania(),
            }],
        )
        .unwrap();
        assert_eq!(
            m.actor(actor(0)).unwrap().ubicacion(),
            UbicacionActor::EnLugar(lugar(1))
        );
        assert!(m.miembros_colectivo(compania()).unwrap().is_empty());
        assert_eq!(m.grupo().integrantes(), &[actor(1)]);
    }
    #[test]
    fn liderazgo_exige_miembro_vivo_y_no_reaparece_al_resucitar() {
        use OperacionMundo::*;
        let mut m = Mundo::nuevo(datos()).unwrap();
        assert!(
            aplicar(
                &mut m,
                vec![EstablecerLiderColectivo {
                    colectivo: compania(),
                    lider: Some(actor(2))
                }]
            )
            .is_err()
        );
        aplicar(
            &mut m,
            vec![
                IngresarActorEnColectivo {
                    actor: actor(2),
                    colectivo: compania(),
                },
                EstablecerLiderColectivo {
                    colectivo: compania(),
                    lider: Some(actor(2)),
                },
            ],
        )
        .unwrap();
        assert!(m.trasladar(actor(2), lugar(1)).is_err());
        let muerto = Caracteristica::nueva("muerto".into()).unwrap();
        aplicar(
            &mut m,
            vec![
                AnadirCaracteristicaActor {
                    actor: actor(2),
                    caracteristica: muerto.clone(),
                },
                QuitarCaracteristicaActor {
                    actor: actor(2),
                    caracteristica: muerto,
                },
            ],
        )
        .unwrap();
        assert_eq!(m.colectivo(compania()).unwrap().lider(), None);
        aplicar(
            &mut m,
            vec![
                EstablecerLiderColectivo {
                    colectivo: compania(),
                    lider: Some(actor(2)),
                },
                SalirActorDeColectivo {
                    actor: actor(2),
                    colectivo: compania(),
                },
            ],
        )
        .unwrap();
        assert_eq!(m.colectivo(compania()).unwrap().lider(), None);
    }
    #[test]
    fn carga_y_operaciones_rechazan_colectivos_inconsistentes() {
        let mut d = datos();
        d.colectivos[0].lider = Some(actor(2));
        assert!(Mundo::nuevo(d).is_err());
        let mut d = datos();
        d.colectivos[0].lugar = lugar(2);
        assert!(Mundo::nuevo(d).is_err());
        let mut d = datos();
        d.actores[0].ubicacion = UbicacionActor::EnColectivo(compania());
        assert!(Mundo::nuevo(d).is_err());
        let mut m = Mundo::nuevo(datos()).unwrap();
        assert!(
            aplicar(
                &mut m,
                vec![OperacionMundo::IngresarActorEnColectivo {
                    actor: actor(1),
                    colectivo: compania()
                }]
            )
            .is_err()
        );
        assert!(
            aplicar(
                &mut m,
                vec![OperacionMundo::TrasladarColectivo {
                    colectivo: compania(),
                    destino: lugar(2)
                }]
            )
            .is_err()
        );
        assert!(m.colectivo_de(actor(0)).is_none());
    }
}
