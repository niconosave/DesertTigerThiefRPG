use crate::acciones::viajar::expedicion::{EstadoEmbarcacion, MedioAcuatico};
use crate::azar::FuenteAzar;
use crate::{Caracteristica, Duracion, IdActor, IdLugar, Mundo, OperacionMundo};

use super::resolucion::{
    self, AccionParticular, BandaResolucion, CambioTransitorio, ContextoTraza, EjeCausal,
    ErrorResolucion, Evaluacion, ProcedenciaFactor, PruebaD20, Resolucion, ResolucionAplicada,
    TrazaAccionParticular,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ManiobraNautica {
    Remar,
    Navegar,
    GobernarTimon,
    Fondear,
    Atracar,
    Achicar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoNavegacion {
    pub banda: BandaResolucion,
    pub avanza: bool,
    pub duracion: Duracion,
    pub estado: EstadoEmbarcacion,
    pub operaciones: Vec<OperacionMundo>,
    pub traza: TrazaAccionParticular,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolver_navegacion(
    mundo: &Mundo,
    maniobra: ManiobraNautica,
    medio: MedioAcuatico,
    estado: EstadoEmbarcacion,
    responsable: IdActor,
    viajeros: &[IdActor],
    origen: IdLugar,
    destino: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoNavegacion, ErrorResolucion> {
    if medio == MedioAcuatico::Ninguno || viajeros.is_empty() || !viajeros.contains(&responsable) {
        return Err(ErrorResolucion::ContextoInvalido);
    }
    let actor = mundo
        .actor(responsable)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let destino_id = destino;
    let origen = mundo
        .lugar(origen)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let destino = mundo
        .lugar(destino)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let agua = es_agua(origen) || es_agua(destino);
    if !agua {
        return Err(ErrorResolucion::ContextoInvalido);
    }
    let mar_abierto = tiene(origen, "marAbierto") || tiene(destino, "marAbierto");
    let aguas_someras = tiene(origen, "aguasSomeras") || tiene(destino, "aguasSomeras");
    let canal_navegable = tiene(origen, "canalNavegable") || tiene(destino, "canalNavegable");
    if (medio == MedioAcuatico::Bote && mar_abierto)
        || (medio == MedioAcuatico::Barco && aguas_someras && !canal_navegable)
    {
        return Err(ErrorResolucion::ContextoInvalido);
    }
    let adverso = tiene(origen, "corrienteRapida")
        || tiene(destino, "corrienteRapida")
        || tiene(origen, "aguaAgitada")
        || tiene(destino, "aguaAgitada");
    let peligro = tiene(origen, "rocoso")
        || tiene(destino, "rocoso")
        || tiene(origen, "estrecho")
        || tiene(destino, "estrecho");
    let conoce = mundo
        .actor_conoce_lugar(responsable, destino_id)
        .unwrap_or(false);
    let (base, modificador, automatico) = match maniobra {
        ManiobraNautica::Remar => (
            9 + i16::from(adverso) * 4,
            i16::from(tiene_actor(actor, "remero")) * 5
                + i16::from(tiene_actor(actor, "fuerte")) * 3
                + i16::from(viajeros.len() > 1) * 3,
            medio == MedioAcuatico::Bote && !adverso && estado == EstadoEmbarcacion::Estable,
        ),
        ManiobraNautica::Navegar => (
            if mar_abierto { 15 } else { 10 },
            i16::from(tiene_actor(actor, "navegante")) * 5 + i16::from(conoce) * 4,
            medio == MedioAcuatico::Barco
                && !mar_abierto
                && conoce
                && estado == EstadoEmbarcacion::Estable,
        ),
        ManiobraNautica::GobernarTimon => (
            12 + i16::from(peligro) * 3,
            i16::from(tiene_actor(actor, "timonel")) * 5
                + i16::from(tiene_actor(actor, "navegante")) * 3
                + i16::from(conoce) * 2,
            false,
        ),
        ManiobraNautica::Fondear => (
            10 + i16::from(tiene(origen, "costaAbierta")) * 3
                - i16::from(tiene(origen, "bahiaProtegida")) * 3,
            i16::from(tiene_actor(actor, "marino")) * 4
                + i16::from(tiene_actor(actor, "navegante")) * 3,
            tiene(origen, "bahiaProtegida") && conoce,
        ),
        ManiobraNautica::Atracar => (
            10 + i16::from(adverso || peligro) * 3,
            i16::from(tiene_actor(actor, "timonel")) * 4 + i16::from(conoce) * 3,
            (tiene(destino, "muelle") || tiene(destino, "puerto")) && !adverso,
        ),
        ManiobraNautica::Achicar => (
            10 + grados_despues_agua(estado) * 3,
            i16::from(tiene_actor(actor, "fuerte")) * 3 + i16::from(viajeros.len() > 1) * 2,
            estado == EstadoEmbarcacion::AguaEmbarcada && viajeros.len() > 1,
        ),
    };
    let evaluacion = if automatico {
        Evaluacion::Determinista(BandaResolucion::Holgada)
    } else {
        Evaluacion::Incierta(PruebaD20 {
            dificultad: base,
            modificador,
        })
    };
    let resuelta = resolucion::resolver(evaluacion, azar)?;
    let banda = match resuelta {
        Resolucion::Determinista { banda } | Resolucion::Incierta { banda, .. } => banda,
        Resolucion::Imposible => return Err(ErrorResolucion::ContextoInvalido),
    };
    let dominante = matches!(
        maniobra,
        ManiobraNautica::Remar | ManiobraNautica::Navegar | ManiobraNautica::Atracar
    );
    let avanza = dominante && banda != BandaResolucion::Fallida;
    let mut nuevo_estado = estado;
    if maniobra == ManiobraNautica::Achicar {
        nuevo_estado = match banda {
            BandaResolucion::Holgada => EstadoEmbarcacion::Estable,
            BandaResolucion::Lograda => mejorar(estado),
            BandaResolucion::Costosa => estado,
            BandaResolucion::Fallida => empeorar(estado),
        };
    } else if matches!(banda, BandaResolucion::Costosa | BandaResolucion::Fallida)
        && (adverso || peligro)
    {
        nuevo_estado = empeorar(estado);
    }
    let operaciones = if matches!(banda, BandaResolucion::Costosa)
        && matches!(maniobra, ManiobraNautica::Remar | ManiobraNautica::Navegar)
    {
        vec![OperacionMundo::AnadirCaracteristicaActor {
            actor: responsable,
            caracteristica: caracteristica("fatigado"),
        }]
    } else {
        vec![]
    };
    let duracion = if matches!(
        maniobra,
        ManiobraNautica::GobernarTimon | ManiobraNautica::Fondear | ManiobraNautica::Achicar
    ) {
        Duracion::CERO
    } else if maniobra == ManiobraNautica::Navegar && banda == BandaResolucion::Costosa {
        Duracion::desde_dias(2)
    } else {
        Duracion::desde_dias(1)
    };
    let accion = match maniobra {
        ManiobraNautica::Remar => AccionParticular::Remar,
        ManiobraNautica::Navegar => AccionParticular::Navegar,
        ManiobraNautica::GobernarTimon => AccionParticular::GobernarTimon,
        ManiobraNautica::Fondear => AccionParticular::Fondear,
        ManiobraNautica::Atracar => AccionParticular::Atracar,
        ManiobraNautica::Achicar => AccionParticular::Achicar,
    };
    let mut factores = vec![resolucion::factor(
        "dificultadManiobra",
        "la maniobra y el agua fijan la dificultad",
        EjeCausal::Base,
        ProcedenciaFactor::Regla,
        base,
    )];
    if modificador != 0 {
        factores.push(resolucion::factor(
            "capacidadTripulacion",
            "la capacidad y composición del grupo ayudan",
            EjeCausal::Capacidad,
            ProcedenciaFactor::Actor(responsable),
            modificador,
        ));
    }
    if adverso {
        factores.push(resolucion::factor(
            "aguaAdversa",
            "el agua adversa dificulta la maniobra",
            EjeCausal::Entorno,
            ProcedenciaFactor::Contexto,
            4,
        ));
    }
    if peligro {
        factores.push(resolucion::factor(
            "peligroNautico",
            "el paso presenta un peligro náutico",
            EjeCausal::Entorno,
            ProcedenciaFactor::Contexto,
            3,
        ));
    }
    let traza = TrazaAccionParticular {
        accion,
        responsable: Some(responsable),
        ayudante: viajeros.iter().copied().find(|&id| id != responsable),
        contexto: ContextoTraza::Trayecto {
            origen: origen.id(),
            destino: destino.id(),
        },
        factores,
        resoluciones: vec![ResolucionAplicada {
            sujeto: Some(responsable),
            modo: resolucion::modo_resuelto(evaluacion, resuelta, false),
        }],
        banda: Some(banda),
        desenlace: if avanza {
            "avanceNautico"
        } else {
            "maniobraSinAvance"
        },
        duracion,
        cambios_transitorios: vec![
            CambioTransitorio::Avance { confirmado: avanza },
            CambioTransitorio::EstadoMedio {
                anterior: estado_embarcacion_indice(estado),
                posterior: estado_embarcacion_indice(nuevo_estado),
            },
        ],
        operaciones_solicitadas: operaciones.clone(),
        operaciones_confirmadas: None,
    };
    Ok(ResultadoNavegacion {
        banda,
        avanza,
        duracion,
        estado: nuevo_estado,
        operaciones,
        traza,
    })
}

fn estado_embarcacion_indice(estado: EstadoEmbarcacion) -> u8 {
    match estado {
        EstadoEmbarcacion::Estable => 0,
        EstadoEmbarcacion::AguaEmbarcada => 1,
        EstadoEmbarcacion::Anegada => 2,
        EstadoEmbarcacion::NaufragioInminente => 3,
    }
}

fn es_agua(lugar: &crate::Lugar) -> bool {
    tiene(lugar, "acuatico")
}

fn grados_despues_agua(estado: EstadoEmbarcacion) -> i16 {
    match estado {
        EstadoEmbarcacion::Estable | EstadoEmbarcacion::AguaEmbarcada => 0,
        EstadoEmbarcacion::Anegada => 1,
        EstadoEmbarcacion::NaufragioInminente => 2,
    }
}

fn empeorar(estado: EstadoEmbarcacion) -> EstadoEmbarcacion {
    match estado {
        EstadoEmbarcacion::Estable => EstadoEmbarcacion::AguaEmbarcada,
        EstadoEmbarcacion::AguaEmbarcada => EstadoEmbarcacion::Anegada,
        EstadoEmbarcacion::Anegada | EstadoEmbarcacion::NaufragioInminente => {
            EstadoEmbarcacion::NaufragioInminente
        }
    }
}

fn mejorar(estado: EstadoEmbarcacion) -> EstadoEmbarcacion {
    match estado {
        EstadoEmbarcacion::NaufragioInminente => EstadoEmbarcacion::Anegada,
        EstadoEmbarcacion::Anegada => EstadoEmbarcacion::AguaEmbarcada,
        EstadoEmbarcacion::AguaEmbarcada | EstadoEmbarcacion::Estable => EstadoEmbarcacion::Estable,
    }
}

fn tiene_actor(actor: &crate::Actor, valor: &str) -> bool {
    actor.tiene_caracteristica(&caracteristica(valor))
}

fn tiene(lugar: &crate::Lugar, valor: &str) -> bool {
    lugar.tiene_caracteristica(&caracteristica(valor))
}

fn caracteristica(valor: &str) -> Caracteristica {
    Caracteristica::nueva(valor.to_owned()).unwrap_or_else(|_| unreachable!())
}

#[cfg(test)]
mod pruebas {
    use std::num::NonZeroUsize;

    use super::*;
    use crate::acciones::ModoResolucion;
    use crate::azar::ErrorFuenteAzar;
    use crate::{
        ActorInicial, DatosInicialesMundo, ExtremoRelacion, Fecha, IdMapa, LugarInicial, Mes,
        RelacionInicial, TipoRelacion,
    };

    struct AzarFijo(usize);

    impl FuenteAzar for AzarFijo {
        fn elegir_indice(
            &mut self,
            limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            Ok(self.0.min(limite_exclusivo.get() - 1))
        }
    }

    fn actor() -> IdActor {
        IdActor::desde_posicion(0).expect("ID válido")
    }

    fn lugar(indice: usize) -> IdLugar {
        IdLugar::desde_posicion(indice).expect("ID válido")
    }

    fn mundo() -> Mundo {
        Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID válido"),
            fecha: Fecha::nueva(2026, Mes::Octubre, 18).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: lugar(0),
                    caracteristicas: vec![caracteristica("acuatico"), caracteristica("costero")],
                },
                LugarInicial {
                    id: lugar(1),
                    caracteristicas: vec![caracteristica("acuatico"), caracteristica("marAbierto")],
                },
            ],
            actores: vec![
                ActorInicial {
                    id: actor(),
                    lugar: lugar(0),
                    caracteristicas: vec![caracteristica("navegante"), caracteristica("fuerte")],
                },
                ActorInicial {
                    id: IdActor::desde_posicion(1).expect("ID válido"),
                    lugar: lugar(0),
                    caracteristicas: vec![],
                },
            ],
            actor_jugador: actor(),
            companeros: vec![IdActor::desde_posicion(1).expect("ID válido")],
            variables_globales: vec![],
            relaciones: vec![RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(lugar(0)),
                destino: ExtremoRelacion::Lugar(lugar(1)),
            }],
        })
        .expect("Mundo válido")
    }

    #[test]
    fn navegar_y_achicar_progresan_sin_naufragio_instantaneo() {
        let mundo = mundo();
        let mut azar = AzarFijo(0);
        let resultado = resolver_navegacion(
            &mundo,
            ManiobraNautica::Navegar,
            MedioAcuatico::Barco,
            EstadoEmbarcacion::Estable,
            actor(),
            &[actor()],
            lugar(0),
            lugar(1),
            &mut azar,
        )
        .expect("navegación válida");
        assert!(!resultado.avanza);
        assert_ne!(resultado.estado, EstadoEmbarcacion::NaufragioInminente);
        assert_eq!(resultado.traza.accion, AccionParticular::Navegar);
        assert!(matches!(
            resultado.traza.resoluciones.as_slice(),
            [ResolucionAplicada {
                modo: ModoResolucion::PruebaD20 { .. },
                ..
            }]
        ));

        let mut azar = AzarFijo(19);
        let achique = resolver_navegacion(
            &mundo,
            ManiobraNautica::Achicar,
            MedioAcuatico::Barco,
            EstadoEmbarcacion::Anegada,
            actor(),
            &[actor()],
            lugar(0),
            lugar(1),
            &mut azar,
        )
        .expect("achique válido");
        assert!(achique.estado < EstadoEmbarcacion::Anegada);
        assert_eq!(achique.traza.accion, AccionParticular::Achicar);
    }
}
