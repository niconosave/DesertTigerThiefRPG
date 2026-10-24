use crate::azar::FuenteAzar;
use crate::{Caracteristica, Duracion, IdActor, IdLugar, Mundo, OperacionMundo};

use super::resolucion::{
    self, AccionParticular, BandaResolucion, CambioTransitorio, ContextoTraza, EjeCausal,
    ErrorResolucion, Evaluacion, ModoResolucion, ProcedenciaFactor, PruebaD20, Resolucion,
    ResolucionAplicada, TrazaAccionParticular, VisibilidadFactor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccionRecurso {
    BuscarAgua,
    Cazar,
    Pescar,
    Recolectar,
}

impl AccionRecurso {
    pub fn clave_intento(self) -> u8 {
        match self {
            Self::BuscarAgua => 0,
            Self::Cazar => 1,
            Self::Pescar => 2,
            Self::Recolectar => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CalidadAgua {
    Potable,
    Insegura,
    Salada,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CalidadCampamento {
    Protegido,
    Adecuado,
    Precario,
    Expuesto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoRecurso {
    pub agua: u32,
    pub alimento: u32,
    pub calidad_agua: Option<CalidadAgua>,
    pub duracion: Duracion,
    pub banda: Option<BandaResolucion>,
    pub traza: TrazaAccionParticular,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoTratamientoAgua {
    pub cantidad_potable: u32,
    pub traza: TrazaAccionParticular,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoCampamento {
    pub calidad: CalidadCampamento,
    pub traza: TrazaAccionParticular,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoDescanso {
    pub operaciones: Vec<OperacionMundo>,
    pub traza: TrazaAccionParticular,
}

pub(crate) fn resolver_recurso(
    mundo: &Mundo,
    accion: AccionRecurso,
    responsable: IdActor,
    viajeros: &[IdActor],
    lugar: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoRecurso, ErrorResolucion> {
    let actor = mundo
        .actor(responsable)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let lugar_actual = mundo
        .lugar(lugar)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    if viajeros.is_empty() || !viajeros.contains(&responsable) {
        return Err(ErrorResolucion::ContextoInvalido);
    }
    let cantidad_grupo =
        u32::try_from(viajeros.len()).map_err(|_| ErrorResolucion::Desbordamiento)?;

    let (evaluacion, calidad) = match accion {
        AccionRecurso::BuscarAgua if tiene_lugar(lugar_actual, "aguaDulce") => (
            Evaluacion::Determinista(BandaResolucion::Holgada),
            Some(CalidadAgua::Potable),
        ),
        AccionRecurso::BuscarAgua if tiene_lugar(lugar_actual, "aguaContaminada") => (
            prueba_agua(mundo, actor, responsable, lugar, lugar_actual),
            Some(CalidadAgua::Insegura),
        ),
        AccionRecurso::BuscarAgua
            if tiene_lugar(lugar_actual, "aguaSalada")
                || tiene_lugar(lugar_actual, "aguaSalobre") =>
        {
            (
                Evaluacion::Determinista(BandaResolucion::Fallida),
                Some(CalidadAgua::Salada),
            )
        }
        AccionRecurso::BuscarAgua => (Evaluacion::Imposible, None),
        AccionRecurso::Cazar if tiene_lugar(lugar_actual, "faunaAbundante") => (
            prueba(8, capacidad_caza(mundo, actor, responsable, lugar)),
            None,
        ),
        AccionRecurso::Cazar if tiene_lugar(lugar_actual, "fauna") => (
            prueba(12, capacidad_caza(mundo, actor, responsable, lugar)),
            None,
        ),
        AccionRecurso::Cazar if tiene_lugar(lugar_actual, "faunaEscasa") => (
            prueba(16, capacidad_caza(mundo, actor, responsable, lugar)),
            None,
        ),
        AccionRecurso::Cazar => (Evaluacion::Imposible, None),
        AccionRecurso::Pescar if tiene_lugar(lugar_actual, "pecesAbundantes") => (
            prueba(9, capacidad_pesca(mundo, actor, responsable, lugar)),
            None,
        ),
        AccionRecurso::Pescar if tiene_lugar(lugar_actual, "peces") => (
            prueba(
                13 + i16::from(tiene_lugar(lugar_actual, "corrienteRapida")) * 3,
                capacidad_pesca(mundo, actor, responsable, lugar),
            ),
            None,
        ),
        AccionRecurso::Pescar => (Evaluacion::Imposible, None),
        AccionRecurso::Recolectar if tiene_lugar(lugar_actual, "vegetacionComestible") => (
            prueba(
                if tiene_lugar(lugar_actual, "vegetacionAbundante") {
                    8
                } else {
                    12
                },
                capacidad_recoleccion(mundo, actor, responsable, lugar),
            ),
            None,
        ),
        AccionRecurso::Recolectar => (Evaluacion::Imposible, None),
    };
    let resolucion = resolucion::resolver(evaluacion, azar)?;
    let banda = match resolucion {
        Resolucion::Imposible => None,
        Resolucion::Determinista { banda } | Resolucion::Incierta { banda, .. } => Some(banda),
    };
    let jornadas: u32 = match (accion, banda) {
        (_, None | Some(BandaResolucion::Fallida)) => 0,
        (
            AccionRecurso::BuscarAgua | AccionRecurso::Cazar | AccionRecurso::Pescar,
            Some(BandaResolucion::Holgada),
        ) => 2,
        (
            AccionRecurso::BuscarAgua | AccionRecurso::Cazar | AccionRecurso::Pescar,
            Some(BandaResolucion::Lograda),
        ) => 1,
        (
            AccionRecurso::BuscarAgua | AccionRecurso::Cazar | AccionRecurso::Pescar,
            Some(BandaResolucion::Costosa),
        ) => 0,
        (AccionRecurso::Recolectar, Some(BandaResolucion::Holgada)) => 1,
        (AccionRecurso::Recolectar, Some(BandaResolucion::Lograda | BandaResolucion::Costosa)) => 0,
    };
    let parcial = match (accion, banda) {
        (
            AccionRecurso::BuscarAgua | AccionRecurso::Cazar | AccionRecurso::Pescar,
            Some(BandaResolucion::Costosa),
        ) => cantidad_grupo / 2,
        (AccionRecurso::Recolectar, Some(BandaResolucion::Lograda)) => cantidad_grupo / 2,
        (AccionRecurso::Recolectar, Some(BandaResolucion::Costosa)) => {
            cantidad_grupo.div_ceil(2).max(1)
        }
        _ => 0,
    };
    let total = jornadas
        .checked_mul(cantidad_grupo)
        .and_then(|v| v.checked_add(parcial))
        .ok_or(ErrorResolucion::Desbordamiento)?;
    let duracion = if matches!(resolucion, Resolucion::Imposible)
        || (accion == AccionRecurso::BuscarAgua
            && matches!(resolucion, Resolucion::Determinista { .. }))
    {
        Duracion::CERO
    } else {
        Duracion::desde_dias(1)
    };
    let (accion_traza, clave_recurso) = match accion {
        AccionRecurso::BuscarAgua => (AccionParticular::BuscarAgua, "fuenteAgua"),
        AccionRecurso::Cazar => (AccionParticular::Cazar, "fauna"),
        AccionRecurso::Pescar => (AccionParticular::Pescar, "peces"),
        AccionRecurso::Recolectar => (AccionParticular::Recolectar, "vegetacionComestible"),
    };
    let conocido = mundo
        .actor_conoce_lugar(responsable, lugar)
        .unwrap_or(false);
    let mut factor_recurso = resolucion::factor(
        clave_recurso,
        "el recurso presente determina la viabilidad",
        EjeCausal::Recurso,
        ProcedenciaFactor::Lugar(lugar),
        0,
    );
    if matches!(evaluacion, Evaluacion::Imposible) && !conocido {
        factor_recurso.visibilidad = VisibilidadFactor::Oculta;
    }
    let mut factores = vec![factor_recurso];
    if let Evaluacion::Incierta(prueba) = evaluacion {
        factores.push(resolucion::factor(
            "dificultadRecurso",
            "la abundancia y el entorno fijan la dificultad",
            EjeCausal::Entorno,
            ProcedenciaFactor::Lugar(lugar),
            prueba.dificultad,
        ));
        if prueba.modificador != 0 {
            factores.push(resolucion::factor(
                "capacidadResponsable",
                "la experiencia y el conocimiento ayudan",
                EjeCausal::Capacidad,
                ProcedenciaFactor::Actor(responsable),
                prueba.modificador,
            ));
        }
    }
    let agua = if accion == AccionRecurso::BuscarAgua {
        total
    } else {
        0
    };
    let alimento = if accion != AccionRecurso::BuscarAgua {
        total
    } else {
        0
    };
    let traza = TrazaAccionParticular {
        accion: accion_traza,
        responsable: Some(responsable),
        ayudante: None,
        contexto: ContextoTraza::Lugar { lugar },
        factores,
        resoluciones: vec![ResolucionAplicada {
            sujeto: Some(responsable),
            modo: resolucion::modo_resuelto(
                evaluacion,
                resolucion,
                !conocido && matches!(evaluacion, Evaluacion::Imposible),
            ),
        }],
        banda,
        desenlace: if total > 0 {
            "recursoObtenido"
        } else if matches!(resolucion, Resolucion::Imposible) {
            "imposible"
        } else {
            "sinRecurso"
        },
        duracion,
        cambios_transitorios: vec![CambioTransitorio::Reservas {
            agua: i64::from(agua),
            alimento: i64::from(alimento),
        }],
        operaciones_solicitadas: vec![],
        operaciones_confirmadas: Some(vec![]),
    };
    Ok(ResultadoRecurso {
        agua,
        alimento,
        calidad_agua: calidad,
        duracion,
        banda,
        traza,
    })
}

pub(crate) fn tratar_agua(
    calidad: CalidadAgua,
    cantidad: u32,
    improvisado: bool,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoTratamientoAgua, ErrorResolucion> {
    let evaluacion = match calidad {
        CalidadAgua::Potable => Evaluacion::Determinista(BandaResolucion::Holgada),
        CalidadAgua::Insegura if !improvisado => Evaluacion::Determinista(BandaResolucion::Holgada),
        CalidadAgua::Salada => Evaluacion::Imposible,
        CalidadAgua::Insegura => Evaluacion::Incierta(PruebaD20 {
            dificultad: 10,
            modificador: 0,
        }),
    };
    let resuelta = resolucion::resolver(evaluacion, azar)?;
    let cantidad_potable = match resuelta {
        Resolucion::Determinista { .. } => cantidad,
        Resolucion::Imposible => 0,
        Resolucion::Incierta {
            banda: BandaResolucion::Holgada | BandaResolucion::Lograda,
            ..
        } => cantidad,
        Resolucion::Incierta {
            banda: BandaResolucion::Costosa,
            ..
        } => cantidad / 2,
        Resolucion::Incierta { .. } => 0,
    };
    Ok(ResultadoTratamientoAgua {
        cantidad_potable,
        traza: TrazaAccionParticular {
            accion: AccionParticular::TratarAgua,
            responsable: None,
            ayudante: None,
            contexto: ContextoTraza::SinLugar,
            factores: vec![resolucion::factor(
                "calidadAgua",
                "la calidad determina el tratamiento",
                EjeCausal::Calidad,
                ProcedenciaFactor::Contexto,
                0,
            )],
            resoluciones: vec![ResolucionAplicada {
                sujeto: None,
                modo: resolucion::modo_resuelto(evaluacion, resuelta, false),
            }],
            banda: resuelta.banda(),
            desenlace: if cantidad_potable == cantidad {
                "aguaPotabilizada"
            } else if cantidad_potable == 0 {
                "tratamientoFallido"
            } else {
                "aguaPotabilizadaParcial"
            },
            duracion: Duracion::CERO,
            cambios_transitorios: vec![CambioTransitorio::Reservas {
                agua: i64::from(cantidad_potable),
                alimento: 0,
            }],
            operaciones_solicitadas: vec![],
            operaciones_confirmadas: Some(vec![]),
        },
    })
}

pub(crate) fn montar_campamento(
    mundo: &Mundo,
    responsable: IdActor,
    lugar: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoCampamento, ErrorResolucion> {
    let actor = mundo
        .actor(responsable)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let lugar = mundo
        .lugar(lugar)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    if tiene_lugar(lugar, "cueva") || tiene_lugar(lugar, "refugio") {
        return Ok(ResultadoCampamento {
            calidad: CalidadCampamento::Protegido,
            traza: TrazaAccionParticular {
                accion: AccionParticular::MontarCampamento,
                responsable: Some(responsable),
                ayudante: None,
                contexto: ContextoTraza::Lugar { lugar: lugar.id() },
                factores: vec![resolucion::factor(
                    "refugio",
                    "un refugio existente protege el campamento",
                    EjeCausal::Entorno,
                    ProcedenciaFactor::Lugar(lugar.id()),
                    -3,
                )],
                resoluciones: vec![ResolucionAplicada {
                    sujeto: Some(responsable),
                    modo: ModoResolucion::Determinista,
                }],
                banda: Some(BandaResolucion::Holgada),
                desenlace: "campamentoProtegido",
                duracion: Duracion::CERO,
                cambios_transitorios: vec![CambioTransitorio::Calidad { clave: "protegido" }],
                operaciones_solicitadas: vec![],
                operaciones_confirmadas: Some(vec![]),
            },
        });
    }
    let dificultad = 10
        + i16::from(tiene_lugar(lugar, "abierto") || tiene_lugar(lugar, "viento")) * 3
        + i16::from(tiene_lugar(lugar, "pantanoso") || tiene_lugar(lugar, "inundado")) * 4
        - i16::from(tiene_lugar(lugar, "cueva") || tiene_lugar(lugar, "refugio")) * 3;
    let modificador = i16::from(tiene_actor(actor, "superviviente")) * 4
        + i16::from(tiene_actor(actor, "equipoCampamento")) * 4;
    let evaluacion = prueba(dificultad, modificador);
    let resuelta = resolucion::resolver(evaluacion, azar)?;
    let banda = match resuelta {
        Resolucion::Incierta { banda, .. } | Resolucion::Determinista { banda } => banda,
        Resolucion::Imposible => return Err(ErrorResolucion::ContextoInvalido),
    };
    let calidad = match banda {
        BandaResolucion::Holgada => CalidadCampamento::Protegido,
        BandaResolucion::Lograda => CalidadCampamento::Adecuado,
        BandaResolucion::Costosa => CalidadCampamento::Precario,
        BandaResolucion::Fallida => CalidadCampamento::Expuesto,
    };
    let clave = match calidad {
        CalidadCampamento::Protegido => "protegido",
        CalidadCampamento::Adecuado => "adecuado",
        CalidadCampamento::Precario => "precario",
        CalidadCampamento::Expuesto => "expuesto",
    };
    Ok(ResultadoCampamento {
        calidad,
        traza: TrazaAccionParticular {
            accion: AccionParticular::MontarCampamento,
            responsable: Some(responsable),
            ayudante: None,
            contexto: ContextoTraza::Lugar { lugar: lugar.id() },
            factores: vec![
                resolucion::factor(
                    "dificultadCampamento",
                    "el entorno determina la dificultad",
                    EjeCausal::Entorno,
                    ProcedenciaFactor::Lugar(lugar.id()),
                    dificultad,
                ),
                resolucion::factor(
                    "capacidadCampamento",
                    "la capacidad y el equipo ayudan",
                    EjeCausal::Capacidad,
                    ProcedenciaFactor::Actor(responsable),
                    modificador,
                ),
            ],
            resoluciones: vec![ResolucionAplicada {
                sujeto: Some(responsable),
                modo: resolucion::modo_resuelto(evaluacion, resuelta, false),
            }],
            banda: Some(banda),
            desenlace: "campamentoMontado",
            duracion: Duracion::CERO,
            cambios_transitorios: vec![CambioTransitorio::Calidad { clave }],
            operaciones_solicitadas: vec![],
            operaciones_confirmadas: Some(vec![]),
        },
    })
}

pub(crate) fn descansar(
    mundo: &Mundo,
    viajeros: &[IdActor],
    calidad: CalidadCampamento,
    con_agua_y_alimento: bool,
    condicion_adversa: bool,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoDescanso, ErrorResolucion> {
    if viajeros.is_empty() {
        return Err(ErrorResolucion::ContextoInvalido);
    }
    let mut operaciones = Vec::new();
    let mut resoluciones = Vec::new();
    let mut factores = Vec::new();
    for &id in viajeros {
        let actor = mundo.actor(id).ok_or(ErrorResolucion::ContextoInvalido)?;
        if matches!(
            calidad,
            CalidadCampamento::Protegido | CalidadCampamento::Adecuado
        ) && con_agua_y_alimento
        {
            resoluciones.push(ResolucionAplicada {
                sujeto: Some(id),
                modo: ModoResolucion::Determinista,
            });
            if tiene_actor(actor, "agotado") {
                operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                    actor: id,
                    caracteristica: caracteristica("agotado"),
                });
                operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                    actor: id,
                    caracteristica: caracteristica("fatigado"),
                });
            } else if tiene_actor(actor, "fatigado") {
                operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                    actor: id,
                    caracteristica: caracteristica("fatigado"),
                });
            }
        } else if calidad == CalidadCampamento::Expuesto && condicion_adversa {
            let modificador = i16::from(tiene_actor(actor, "resistente")) * 4
                - i16::from(tiene_actor(actor, "sediento")) * 3
                - i16::from(tiene_actor(actor, "hambriento")) * 2;
            let evaluacion = Evaluacion::Incierta(PruebaD20 {
                dificultad: 10,
                modificador,
            });
            let resuelta = resolucion::resolver(evaluacion, azar)?;
            resoluciones.push(ResolucionAplicada {
                sujeto: Some(id),
                modo: resolucion::modo_resuelto(evaluacion, resuelta, false),
            });
            factores.push(resolucion::factor(
                "estadoDescanso",
                "el estado del viajero modifica su descanso",
                EjeCausal::Estado,
                ProcedenciaFactor::Actor(id),
                modificador,
            ));
            let banda = match resuelta {
                Resolucion::Incierta { banda, .. } => banda,
                _ => return Err(ErrorResolucion::ContextoInvalido),
            };
            match banda {
                BandaResolucion::Holgada if tiene_actor(actor, "fatigado") => {
                    operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                        actor: id,
                        caracteristica: caracteristica("fatigado"),
                    });
                }
                BandaResolucion::Costosa if !tiene_actor(actor, "fatigado") => {
                    operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                        actor: id,
                        caracteristica: caracteristica("fatigado"),
                    });
                }
                BandaResolucion::Fallida if tiene_actor(actor, "fatigado") => {
                    operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                        actor: id,
                        caracteristica: caracteristica("fatigado"),
                    });
                    operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                        actor: id,
                        caracteristica: caracteristica("agotado"),
                    });
                }
                BandaResolucion::Fallida => {
                    operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                        actor: id,
                        caracteristica: caracteristica("fatigado"),
                    });
                }
                BandaResolucion::Holgada | BandaResolucion::Lograda | BandaResolucion::Costosa => {}
            }
        } else {
            resoluciones.push(ResolucionAplicada {
                sujeto: Some(id),
                modo: ModoResolucion::Determinista,
            });
        }
    }
    Ok(ResultadoDescanso {
        traza: TrazaAccionParticular {
            accion: AccionParticular::Descansar,
            responsable: viajeros.first().copied(),
            ayudante: None,
            contexto: ContextoTraza::SinLugar,
            factores,
            resoluciones,
            banda: None,
            desenlace: if operaciones.is_empty() {
                "descansoSinCambios"
            } else {
                "descansoCompletado"
            },
            duracion: Duracion::CERO,
            cambios_transitorios: vec![],
            operaciones_solicitadas: operaciones.clone(),
            operaciones_confirmadas: None,
        },
        operaciones,
    })
}

fn prueba(dificultad: i16, modificador: i16) -> Evaluacion {
    Evaluacion::Incierta(PruebaD20 {
        dificultad,
        modificador,
    })
}

fn prueba_agua(
    mundo: &Mundo,
    actor: &crate::Actor,
    id: IdActor,
    lugar: IdLugar,
    entorno: &crate::Lugar,
) -> Evaluacion {
    let dificultad = 11
        + i16::from(tiene_lugar(entorno, "arido") || tiene_lugar(entorno, "desertico")) * 3
        + i16::from(tiene_lugar(entorno, "rocoso")) * 2;
    let modificador = i16::from(mundo.actor_conoce_lugar(id, lugar).unwrap_or(false)) * 5
        + i16::from(tiene_actor(actor, "conoceDesiertos")) * 4
        + i16::from(tiene_actor(actor, "superviviente")) * 3
        + i16::from(tiene_actor(actor, "rastreador")) * 2;
    prueba(dificultad, modificador)
}

fn capacidad_caza(mundo: &Mundo, actor: &crate::Actor, id: IdActor, lugar: IdLugar) -> i16 {
    i16::from(tiene_actor(actor, "cazador")) * 5
        + i16::from(tiene_actor(actor, "rastreador")) * 3
        + i16::from(mundo.actor_conoce_lugar(id, lugar).unwrap_or(false)) * 2
}

fn capacidad_pesca(mundo: &Mundo, actor: &crate::Actor, id: IdActor, lugar: IdLugar) -> i16 {
    i16::from(tiene_actor(actor, "pescador")) * 5
        + i16::from(mundo.actor_conoce_lugar(id, lugar).unwrap_or(false)) * 2
}

fn capacidad_recoleccion(mundo: &Mundo, actor: &crate::Actor, id: IdActor, lugar: IdLugar) -> i16 {
    i16::from(tiene_actor(actor, "recolector")) * 4
        + i16::from(tiene_actor(actor, "superviviente")) * 2
        + i16::from(mundo.actor_conoce_lugar(id, lugar).unwrap_or(false)) * 3
}

fn tiene_actor(actor: &crate::Actor, valor: &str) -> bool {
    actor.tiene_caracteristica(&caracteristica(valor))
}

fn tiene_lugar(lugar: &crate::Lugar, valor: &str) -> bool {
    lugar.tiene_caracteristica(&caracteristica(valor))
}

fn caracteristica(valor: &str) -> Caracteristica {
    Caracteristica::nueva(valor.to_owned()).unwrap_or_else(|_| unreachable!())
}

#[cfg(test)]
mod pruebas {
    use std::num::NonZeroUsize;

    use super::*;
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

    fn id_actor() -> IdActor {
        IdActor::desde_posicion(0).expect("ID válido")
    }

    fn id_lugar(indice: usize) -> IdLugar {
        IdLugar::desde_posicion(indice).expect("ID válido")
    }

    fn mundo() -> Mundo {
        Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID válido"),
            fecha: Fecha::nueva(2026, Mes::Octubre, 18).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: id_lugar(0),
                    caracteristicas: vec![caracteristica("aguaDulce"), caracteristica("fauna")],
                },
                LugarInicial {
                    id: id_lugar(1),
                    caracteristicas: vec![caracteristica("pantanoso")],
                },
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor(),
                    lugar: id_lugar(0),
                    caracteristicas: vec![caracteristica("cazador"), caracteristica("fatigado")],
                },
                ActorInicial {
                    id: IdActor::desde_posicion(1).expect("ID válido"),
                    lugar: id_lugar(0),
                    caracteristicas: vec![],
                },
            ],
            actor_jugador: id_actor(),
            companeros: vec![IdActor::desde_posicion(1).expect("ID válido")],
            variables_globales: vec![],
            relaciones: vec![RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id_lugar(0)),
                destino: ExtremoRelacion::Lugar(id_lugar(1)),
            }],
        })
        .expect("Mundo válido")
    }

    #[test]
    fn agua_visible_es_automatica_y_cazar_respeta_las_bandas() {
        let mundo = mundo();
        let mut azar = AzarFijo(19);
        let agua = resolver_recurso(
            &mundo,
            AccionRecurso::BuscarAgua,
            id_actor(),
            &[id_actor()],
            id_lugar(0),
            &mut azar,
        )
        .expect("búsqueda válida");
        assert_eq!(agua.agua, 2);
        assert_eq!(agua.calidad_agua, Some(CalidadAgua::Potable));
        assert_eq!(agua.duracion, Duracion::CERO);
        assert_eq!(agua.traza.accion, AccionParticular::BuscarAgua);
        assert_eq!(
            agua.traza.resoluciones,
            vec![ResolucionAplicada {
                sujeto: Some(id_actor()),
                modo: ModoResolucion::Determinista,
            }]
        );

        let caza = resolver_recurso(
            &mundo,
            AccionRecurso::Cazar,
            id_actor(),
            &[id_actor()],
            id_lugar(0),
            &mut azar,
        )
        .expect("caza válida");
        assert_eq!(caza.alimento, 2);
        assert_eq!(caza.duracion, Duracion::desde_dias(1));
        assert!(matches!(
            caza.traza.resoluciones.as_slice(),
            [ResolucionAplicada {
                modo: ModoResolucion::PruebaD20 { .. },
                ..
            }]
        ));
    }

    #[test]
    fn tratamiento_y_descanso_no_inventan_curacion_de_heridas() {
        let mundo = mundo();
        let mut azar = AzarFijo(19);
        assert_eq!(
            tratar_agua(CalidadAgua::Salada, 4, true, &mut azar)
                .expect("tratamiento válido")
                .cantidad_potable,
            0
        );
        let operaciones = descansar(
            &mundo,
            &[id_actor()],
            CalidadCampamento::Adecuado,
            true,
            false,
            &mut azar,
        )
        .expect("descanso válido");
        assert!(matches!(
            operaciones.operaciones.as_slice(),
            [OperacionMundo::QuitarCaracteristicaActor { actor, .. }] if *actor == id_actor()
        ));
        assert_eq!(operaciones.traza.accion, AccionParticular::Descansar);
    }
}
