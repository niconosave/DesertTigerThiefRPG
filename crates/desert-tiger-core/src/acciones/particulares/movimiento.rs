use std::collections::BTreeSet;

use crate::azar::FuenteAzar;
use crate::{Caracteristica, Duracion, IdActor, IdLugar, Mundo, OperacionMundo};

use super::resolucion::{
    self, AccionParticular, BandaResolucion, CambioTransitorio, ContextoTraza, EjeCausal,
    ErrorResolucion, Evaluacion, FactorAplicado, ProcedenciaFactor, PronosticoParticular,
    PruebaD20, Resolucion, ResolucionAplicada, TrazaAccionParticular,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SolicitudEscalar {
    pub responsable: IdActor,
    pub ayudante: Option<IdActor>,
    pub viajeros: Vec<IdActor>,
    pub origen: IdLugar,
    pub destino: IdLugar,
    pub carga_voluminosa: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CausaImposibleEscalar {
    SinSuperficieEscalable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RechazoEscalar {
    ActorDesconocido { actor: IdActor },
    LugarDesconocido { lugar: IdLugar },
    LugaresNoConectados { origen: IdLugar, destino: IdLugar },
    SinViajeros,
    ViajeroDuplicado { actor: IdActor },
    ResponsableNoEsViajero { actor: IdActor },
    AyudanteInvalido { actor: IdActor },
    ViajerosSeparados,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvaluacionEscalar {
    modo: Evaluacion,
    causa_imposible: Option<CausaImposibleEscalar>,
    responsable: IdActor,
    expuesto: IdActor,
    ayudante: Option<IdActor>,
    origen: IdLugar,
    destino: IdLugar,
    factores: Vec<FactorAplicado>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ConclusionEscalar {
    Imposible(CausaImposibleEscalar),
    Superada(BandaResolucion),
    Fallida,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoEscalar {
    pub conclusion: ConclusionEscalar,
    pub duracion: Duracion,
    pub operaciones: Vec<OperacionMundo>,
    pub traza: TrazaAccionParticular,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TipoMovimientoTerrestre {
    Orientarse,
    Marchar,
    Descender,
    Cabalgar,
    GuiarCaravana,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RitmoMovimiento {
    Prudente,
    Normal,
    Forzado,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ContextoMovimiento {
    pub ritmo: RitmoMovimiento,
    pub carga_voluminosa: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResultadoMovimientoTerrestre {
    pub banda: BandaResolucion,
    pub avanza: bool,
    pub duracion: Duracion,
    pub operaciones: Vec<OperacionMundo>,
    pub consumo_agua_extra: u32,
    pub consumo_alimento_extra: u32,
    pub traza: TrazaAccionParticular,
}

pub(crate) fn resolver_movimiento_terrestre(
    mundo: &Mundo,
    solicitud: &SolicitudEscalar,
    tipo: TipoMovimientoTerrestre,
    contexto: ContextoMovimiento,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoMovimientoTerrestre, ErrorResolucion> {
    validar_solicitud(mundo, solicitud).map_err(|_| ErrorResolucion::ContextoInvalido)?;
    let responsable = mundo
        .actor(solicitud.responsable)
        .ok_or(ErrorResolucion::ContextoInvalido)?;
    let adversidad = match tipo {
        TipoMovimientoTerrestre::Orientarse => {
            if tiene_en_etapa(mundo, solicitud, "camino")
                || tiene_en_etapa(mundo, solicitud, "calzada")
                || tiene_en_etapa(mundo, solicitud, "sendero")
            {
                0
            } else if tiene_en_etapa(mundo, solicitud, "desertico") {
                4
            } else if tiene_en_etapa(mundo, solicitud, "boscoso")
                || tiene_en_etapa(mundo, solicitud, "pantanoso")
            {
                3
            } else {
                0
            }
        }
        TipoMovimientoTerrestre::Marchar => {
            if tiene_en_etapa(mundo, solicitud, "pantanoso") {
                5
            } else if tiene_en_etapa(mundo, solicitud, "empinado")
                || tiene_en_etapa(mundo, solicitud, "arido")
            {
                4
            } else if tiene_en_etapa(mundo, solicitud, "rocoso")
                || tiene_en_etapa(mundo, solicitud, "boscoso")
            {
                3
            } else {
                0
            }
        }
        TipoMovimientoTerrestre::Descender => {
            if tiene_en_etapa(mundo, solicitud, "pared")
                || tiene_en_etapa(mundo, solicitud, "acantilado")
            {
                3
            } else if tiene_en_etapa(mundo, solicitud, "rocoso")
                || tiene_en_etapa(mundo, solicitud, "estrecho")
            {
                2
            } else {
                0
            }
        }
        TipoMovimientoTerrestre::Cabalgar => {
            if tiene_en_etapa(mundo, solicitud, "pantanoso")
                || tiene_en_etapa(mundo, solicitud, "boscoso")
            {
                5
            } else if tiene_en_etapa(mundo, solicitud, "rocoso")
                || tiene_en_etapa(mundo, solicitud, "empinado")
            {
                3
            } else {
                0
            }
        }
        TipoMovimientoTerrestre::GuiarCaravana => {
            if tiene_en_etapa(mundo, solicitud, "estrecho")
                || tiene_en_etapa(mundo, solicitud, "empinado")
            {
                3
            } else {
                0
            }
        }
    };
    let via = if tiene_en_etapa(mundo, solicitud, "camino")
        || tiene_en_etapa(mundo, solicitud, "calzada")
    {
        5
    } else if tiene_en_etapa(mundo, solicitud, "sendero") {
        3
    } else {
        0
    };
    let capacidad = match tipo {
        TipoMovimientoTerrestre::Orientarse if tiene(responsable, "guia") => 5,
        TipoMovimientoTerrestre::Orientarse if tiene(responsable, "rastreador") => 2,
        TipoMovimientoTerrestre::Marchar if tiene(responsable, "resistente") => 4,
        TipoMovimientoTerrestre::Marchar if tiene(responsable, "superviviente") => 2,
        TipoMovimientoTerrestre::Descender if tiene(responsable, "escalador") => 5,
        TipoMovimientoTerrestre::Descender if tiene(responsable, "fuerte") => 2,
        TipoMovimientoTerrestre::Cabalgar if tiene(responsable, "jinete") => 4,
        TipoMovimientoTerrestre::Cabalgar if tiene(responsable, "camellero") => 4,
        TipoMovimientoTerrestre::GuiarCaravana if tiene(responsable, "camellero") => 5,
        TipoMovimientoTerrestre::GuiarCaravana if tiene(responsable, "arriero") => 5,
        TipoMovimientoTerrestre::GuiarCaravana if tiene(responsable, "guia") => 3,
        _ => 0,
    };
    let conocimiento_especifico = solicitud.viajeros.iter().copied().any(|actor| {
        mundo
            .actor_conoce_lugar(actor, solicitud.origen)
            .unwrap_or(false)
            || mundo
                .actor_conoce_lugar(actor, solicitud.destino)
                .unwrap_or(false)
    });
    let conocimiento = if conocimiento_especifico {
        4
    } else if tiene_en_etapa(mundo, solicitud, "desertico")
        && solicitud
            .viajeros
            .iter()
            .filter_map(|&actor| mundo.actor(actor))
            .any(|actor| tiene(actor, "conoceDesiertos"))
    {
        3
    } else {
        0
    };
    let estado = if tiene(responsable, "agotado") {
        -4
    } else if tiene(responsable, "herido") || tiene(responsable, "fatigado") {
        -2
    } else {
        0
    };
    let carga = contexto.carga_voluminosa
        || solicitud.viajeros.iter().any(|&actor| {
            mundo
                .actor(actor)
                .is_some_and(|actor| tiene(actor, "cargaVoluminosa"))
        });
    let via_danada = tiene_en_etapa(mundo, solicitud, "derrumbado")
        || tiene_en_etapa(mundo, solicitud, "inundado");
    let automatico = estado == 0
        && match tipo {
            TipoMovimientoTerrestre::Orientarse => via > 0 && !via_danada,
            TipoMovimientoTerrestre::Marchar => via > 0 && !via_danada && !carga,
            TipoMovimientoTerrestre::Descender => adversidad == 0,
            TipoMovimientoTerrestre::Cabalgar => via > 0 && adversidad == 0,
            TipoMovimientoTerrestre::GuiarCaravana => via > 0 && adversidad == 0,
        };
    let base = match tipo {
        TipoMovimientoTerrestre::Orientarse => 10,
        TipoMovimientoTerrestre::Marchar => 9,
        TipoMovimientoTerrestre::Descender => 10,
        TipoMovimientoTerrestre::Cabalgar => 9,
        TipoMovimientoTerrestre::GuiarCaravana => 11,
    };
    let ajuste_ritmo = match (tipo, contexto.ritmo) {
        (TipoMovimientoTerrestre::Marchar, RitmoMovimiento::Prudente) => 2,
        (TipoMovimientoTerrestre::Marchar, RitmoMovimiento::Forzado) => -3,
        _ => 0,
    };
    let evaluacion = if automatico {
        Evaluacion::Determinista(BandaResolucion::Holgada)
    } else {
        Evaluacion::Incierta(PruebaD20 {
            dificultad: base + adversidad - via + i16::from(carga) * 2,
            modificador: capacidad + conocimiento + estado + ajuste_ritmo,
        })
    };
    let resolucion = resolucion::resolver(evaluacion, azar)?;
    let banda = match resolucion {
        Resolucion::Determinista { banda } | Resolucion::Incierta { banda, .. } => banda,
        Resolucion::Imposible => return Err(ErrorResolucion::ContextoInvalido),
    };
    let avanza = match tipo {
        TipoMovimientoTerrestre::Orientarse => banda != BandaResolucion::Fallida,
        TipoMovimientoTerrestre::Marchar => banda != BandaResolucion::Fallida,
        TipoMovimientoTerrestre::Descender => true,
        TipoMovimientoTerrestre::Cabalgar | TipoMovimientoTerrestre::GuiarCaravana => {
            banda != BandaResolucion::Fallida
        }
    };
    let duracion = match (tipo, banda) {
        (
            TipoMovimientoTerrestre::Orientarse,
            BandaResolucion::Holgada | BandaResolucion::Lograda,
        ) => Duracion::CERO,
        (TipoMovimientoTerrestre::Marchar, _)
            if contexto.ritmo == RitmoMovimiento::Prudente && adversidad > 0 =>
        {
            Duracion::desde_dias(2)
        }
        (TipoMovimientoTerrestre::Cabalgar | TipoMovimientoTerrestre::GuiarCaravana, _) => {
            Duracion::desde_dias(1)
        }
        _ => Duracion::desde_dias(1),
    };
    let operaciones = match (tipo, banda) {
        (TipoMovimientoTerrestre::Marchar, BandaResolucion::Costosa | BandaResolucion::Fallida) => {
            vec![OperacionMundo::AnadirCaracteristicaActor {
                actor: solicitud.responsable,
                caracteristica: caracteristica("fatigado"),
            }]
        }
        (TipoMovimientoTerrestre::Descender, BandaResolucion::Fallida) => {
            vec![OperacionMundo::AnadirCaracteristicaActor {
                actor: solicitud.responsable,
                caracteristica: caracteristica("herido"),
            }]
        }
        (
            TipoMovimientoTerrestre::Cabalgar | TipoMovimientoTerrestre::GuiarCaravana,
            BandaResolucion::Costosa | BandaResolucion::Fallida,
        ) => vec![OperacionMundo::AnadirCaracteristicaActor {
            actor: solicitud.responsable,
            caracteristica: caracteristica("fatigado"),
        }],
        _ => vec![],
    };
    let consumo_agua_extra =
        if tipo == TipoMovimientoTerrestre::Marchar && contexto.ritmo == RitmoMovimiento::Forzado {
            u32::try_from(solicitud.viajeros.len()).map_err(|_| ErrorResolucion::Desbordamiento)?
        } else {
            0
        };
    let consumo_alimento_extra = if matches!(
        tipo,
        TipoMovimientoTerrestre::Cabalgar | TipoMovimientoTerrestre::GuiarCaravana
    ) && !tiene_en_etapa(mundo, solicitud, "pastizal")
    {
        u32::try_from(solicitud.viajeros.len()).map_err(|_| ErrorResolucion::Desbordamiento)?
    } else {
        0
    };
    let mut factores = vec![resolucion::factor(
        "dificultadBase",
        "dificultad propia de la acción",
        EjeCausal::Base,
        ProcedenciaFactor::Regla,
        base,
    )];
    for (valor, clave, explicacion, eje, procedencia) in [
        (
            adversidad,
            "adversidadTerreno",
            "el terreno exige esfuerzo",
            EjeCausal::Entorno,
            ProcedenciaFactor::Contexto,
        ),
        (
            -via,
            "viaTransitable",
            "una vía reconocible facilita el avance",
            EjeCausal::Via,
            ProcedenciaFactor::Contexto,
        ),
        (
            i16::from(carga) * 2,
            "cargaVoluminosa",
            "la carga dificulta el movimiento",
            EjeCausal::Carga,
            ProcedenciaFactor::Grupo,
        ),
        (
            capacidad,
            "capacidadResponsable",
            "la capacidad del responsable ayuda",
            EjeCausal::Capacidad,
            ProcedenciaFactor::Actor(solicitud.responsable),
        ),
        (
            conocimiento,
            "conocimientoRuta",
            "el conocimiento del recorrido ayuda",
            EjeCausal::Conocimiento,
            ProcedenciaFactor::Grupo,
        ),
        (
            estado,
            "estadoResponsable",
            "el estado del responsable afecta el intento",
            EjeCausal::Estado,
            ProcedenciaFactor::Actor(solicitud.responsable),
        ),
        (
            ajuste_ritmo,
            "ritmoViaje",
            "el ritmo elegido modifica el esfuerzo",
            EjeCausal::Ritmo,
            ProcedenciaFactor::Contexto,
        ),
    ] {
        if valor != 0 {
            factores.push(resolucion::factor(
                clave,
                explicacion,
                eje,
                procedencia,
                valor,
            ));
        }
    }
    let accion = match tipo {
        TipoMovimientoTerrestre::Orientarse => AccionParticular::Orientarse,
        TipoMovimientoTerrestre::Marchar => AccionParticular::Marchar,
        TipoMovimientoTerrestre::Descender => AccionParticular::Descender,
        TipoMovimientoTerrestre::Cabalgar => AccionParticular::Cabalgar,
        TipoMovimientoTerrestre::GuiarCaravana => AccionParticular::GuiarCaravana,
    };
    let traza = TrazaAccionParticular {
        accion,
        responsable: Some(solicitud.responsable),
        ayudante: solicitud.ayudante,
        contexto: ContextoTraza::Trayecto {
            origen: solicitud.origen,
            destino: solicitud.destino,
        },
        factores,
        resoluciones: vec![ResolucionAplicada {
            sujeto: Some(solicitud.responsable),
            modo: resolucion::modo_resuelto(evaluacion, resolucion, false),
        }],
        banda: Some(banda),
        desenlace: if avanza {
            "avanceConfirmado"
        } else {
            "sinAvance"
        },
        duracion,
        cambios_transitorios: vec![
            CambioTransitorio::Avance { confirmado: avanza },
            CambioTransitorio::Reservas {
                agua: -i64::from(consumo_agua_extra),
                alimento: -i64::from(consumo_alimento_extra),
            },
        ],
        operaciones_solicitadas: operaciones.clone(),
        operaciones_confirmadas: None,
    };
    Ok(ResultadoMovimientoTerrestre {
        banda,
        avanza,
        duracion,
        operaciones,
        consumo_agua_extra,
        consumo_alimento_extra,
        traza,
    })
}

pub(crate) fn evaluar_escalar(
    mundo: &Mundo,
    solicitud: &SolicitudEscalar,
) -> Result<EvaluacionEscalar, RechazoEscalar> {
    validar_solicitud(mundo, solicitud)?;

    let tiene_superficie = tiene_en_etapa(mundo, solicitud, "empinado")
        || tiene_en_etapa(mundo, solicitud, "pared")
        || tiene_en_etapa(mundo, solicitud, "acantilado");
    if !tiene_superficie {
        return Ok(EvaluacionEscalar {
            modo: Evaluacion::Imposible,
            causa_imposible: Some(CausaImposibleEscalar::SinSuperficieEscalable),
            responsable: solicitud.responsable,
            expuesto: solicitud.responsable,
            ayudante: solicitud.ayudante,
            origen: solicitud.origen,
            destino: solicitud.destino,
            factores: vec![resolucion::factor(
                "sinSuperficieEscalable",
                "no existe una superficie escalable",
                EjeCausal::Entorno,
                ProcedenciaFactor::Contexto,
                0,
            )],
        });
    }
    let responsable =
        mundo
            .actor(solicitud.responsable)
            .ok_or(RechazoEscalar::ActorDesconocido {
                actor: solicitud.responsable,
            })?;
    let carga = solicitud.carga_voluminosa
        || solicitud.viajeros.iter().any(|&actor| {
            mundo
                .actor(actor)
                .is_some_and(|actual| tiene(actual, "cargaVoluminosa"))
        });
    let formacion_adversa =
        tiene_en_etapa(mundo, solicitud, "pared") || tiene_en_etapa(mundo, solicitud, "acantilado");
    let estado_fisico_adverso = tiene_en_etapa(mundo, solicitud, "rocaHumeda")
        || tiene_en_etapa(mundo, solicitud, "rocaSuelta")
        || tiene_en_etapa(mundo, solicitud, "viento");
    let estado_personal_adverso = tiene(responsable, "agotado")
        || tiene(responsable, "herido")
        || tiene(responsable, "fatigado");
    let experto_equipado = tiene(responsable, "escalador") && tiene(responsable, "equipoMontana");

    let mut factores = vec![resolucion::factor(
        "dificultadBase",
        "dificultad propia de escalar",
        EjeCausal::Base,
        ProcedenciaFactor::Regla,
        12,
    )];
    if experto_equipado {
        factores.push(resolucion::factor(
            "escalador",
            "la pericia del responsable permite un ascenso seguro",
            EjeCausal::Capacidad,
            ProcedenciaFactor::Actor(solicitud.responsable),
            5,
        ));
        factores.push(resolucion::factor(
            "equipoMontana",
            "el equipo adecuado permite resolver el ascenso",
            EjeCausal::Medio,
            ProcedenciaFactor::Actor(solicitud.responsable),
            4,
        ));
    }
    let modo = if experto_equipado
        && !carga
        && !formacion_adversa
        && !estado_fisico_adverso
        && !estado_personal_adverso
    {
        Evaluacion::Determinista(BandaResolucion::Holgada)
    } else {
        let dificultad = 12
            + if formacion_adversa { 3 } else { 0 }
            + if estado_fisico_adverso { 2 } else { 0 }
            + if carga { 2 } else { 0 };
        let capacidad = if tiene(responsable, "escalador") {
            5
        } else if tiene(responsable, "fuerte") {
            2
        } else {
            0
        };
        let equipo = if tiene(responsable, "equipoMontana") {
            4
        } else {
            0
        };
        let ayuda = solicitud
            .ayudante
            .and_then(|actor| mundo.actor(actor))
            .filter(|actor| es_ayudante_competente(actor))
            .map_or(0, |_| 2);
        let estado = if tiene(responsable, "agotado") {
            -4
        } else if tiene(responsable, "herido") {
            -3
        } else if tiene(responsable, "fatigado") {
            -2
        } else {
            0
        };
        for (valor, clave, explicacion, eje, procedencia) in [
            (
                if formacion_adversa { 3 } else { 0 },
                "formacionAdversa",
                "la formación aumenta la dificultad",
                EjeCausal::Entorno,
                ProcedenciaFactor::Contexto,
            ),
            (
                if estado_fisico_adverso { 2 } else { 0 },
                "superficieAdversa",
                "la superficie o el viento dificultan el ascenso",
                EjeCausal::Entorno,
                ProcedenciaFactor::Contexto,
            ),
            (
                if carga { 2 } else { 0 },
                "cargaVoluminosa",
                "la carga dificulta el ascenso",
                EjeCausal::Carga,
                ProcedenciaFactor::Grupo,
            ),
            (
                capacidad,
                "capacidadResponsable",
                "la capacidad del responsable ayuda",
                EjeCausal::Capacidad,
                ProcedenciaFactor::Actor(solicitud.responsable),
            ),
            (
                equipo,
                "equipoMontana",
                "el equipo de montaña ayuda",
                EjeCausal::Medio,
                ProcedenciaFactor::Actor(solicitud.responsable),
            ),
            (
                ayuda,
                "ayudaCompetente",
                "un compañero competente ayuda",
                EjeCausal::Grupo,
                ProcedenciaFactor::Grupo,
            ),
            (
                estado,
                "estadoResponsable",
                "el estado del responsable afecta el ascenso",
                EjeCausal::Estado,
                ProcedenciaFactor::Actor(solicitud.responsable),
            ),
        ] {
            if valor != 0 {
                factores.push(resolucion::factor(
                    clave,
                    explicacion,
                    eje,
                    procedencia,
                    valor,
                ));
            }
        }
        Evaluacion::Incierta(PruebaD20 {
            dificultad,
            modificador: capacidad + equipo + ayuda + estado,
        })
    };

    Ok(EvaluacionEscalar {
        modo,
        causa_imposible: None,
        responsable: solicitud.responsable,
        expuesto: solicitud.responsable,
        ayudante: solicitud.ayudante,
        origen: solicitud.origen,
        destino: solicitud.destino,
        factores,
    })
}

pub(crate) fn resolver_escalar(
    evaluacion: EvaluacionEscalar,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoEscalar, ErrorResolucion> {
    let resolucion = resolucion::resolver(evaluacion.modo, azar)?;
    let duracion = match resolucion {
        Resolucion::Imposible => Duracion::CERO,
        Resolucion::Determinista { .. } | Resolucion::Incierta { .. } => Duracion::desde_dias(1),
    };
    let (conclusion, operaciones) = match resolucion {
        Resolucion::Imposible => (
            ConclusionEscalar::Imposible(
                evaluacion
                    .causa_imposible
                    .expect("una evaluación imposible conserva su causa"),
            ),
            vec![],
        ),
        Resolucion::Determinista { banda } | Resolucion::Incierta { banda, .. } => match banda {
            BandaResolucion::Holgada | BandaResolucion::Lograda => {
                (ConclusionEscalar::Superada(banda), vec![])
            }
            BandaResolucion::Costosa => (
                ConclusionEscalar::Superada(banda),
                vec![OperacionMundo::AnadirCaracteristicaActor {
                    actor: evaluacion.responsable,
                    caracteristica: caracteristica("fatigado"),
                }],
            ),
            BandaResolucion::Fallida => (
                ConclusionEscalar::Fallida,
                vec![OperacionMundo::AnadirCaracteristicaActor {
                    actor: evaluacion.expuesto,
                    caracteristica: caracteristica("herido"),
                }],
            ),
        },
    };
    let banda = resolucion.banda();
    let traza = TrazaAccionParticular {
        accion: AccionParticular::Escalar,
        responsable: Some(evaluacion.responsable),
        ayudante: evaluacion.ayudante,
        contexto: ContextoTraza::Trayecto {
            origen: evaluacion.origen,
            destino: evaluacion.destino,
        },
        factores: evaluacion.factores,
        resoluciones: vec![ResolucionAplicada {
            sujeto: Some(evaluacion.responsable),
            modo: resolucion::modo_resuelto(evaluacion.modo, resolucion, false),
        }],
        banda,
        desenlace: match conclusion {
            ConclusionEscalar::Imposible(_) => "imposible",
            ConclusionEscalar::Superada(_) => "ascensoSuperado",
            ConclusionEscalar::Fallida => "ascensoFallido",
        },
        duracion,
        cambios_transitorios: vec![CambioTransitorio::Avance {
            confirmado: matches!(conclusion, ConclusionEscalar::Superada(_)),
        }],
        operaciones_solicitadas: operaciones.clone(),
        operaciones_confirmadas: None,
    };
    Ok(ResultadoEscalar {
        conclusion,
        duracion,
        operaciones,
        traza,
    })
}

pub(crate) fn pronosticar_escalar(
    evaluacion: &EvaluacionEscalar,
) -> Result<PronosticoParticular, ErrorResolucion> {
    resolucion::pronosticar(evaluacion.modo, Duracion::desde_dias(1), 0)
}

fn validar_solicitud(mundo: &Mundo, solicitud: &SolicitudEscalar) -> Result<(), RechazoEscalar> {
    if mundo.lugar(solicitud.origen).is_none() {
        return Err(RechazoEscalar::LugarDesconocido {
            lugar: solicitud.origen,
        });
    }
    if mundo.lugar(solicitud.destino).is_none() {
        return Err(RechazoEscalar::LugarDesconocido {
            lugar: solicitud.destino,
        });
    }
    if !mundo
        .lugares_conectados(solicitud.origen)
        .is_some_and(|vecinos| vecinos.contains(&solicitud.destino))
    {
        return Err(RechazoEscalar::LugaresNoConectados {
            origen: solicitud.origen,
            destino: solicitud.destino,
        });
    }
    if solicitud.viajeros.is_empty() {
        return Err(RechazoEscalar::SinViajeros);
    }
    let mut vistos = BTreeSet::new();
    let mut ubicacion = None;
    for &viajero in &solicitud.viajeros {
        if !vistos.insert(viajero) {
            return Err(RechazoEscalar::ViajeroDuplicado { actor: viajero });
        }
        let actor = mundo
            .actor(viajero)
            .ok_or(RechazoEscalar::ActorDesconocido { actor: viajero })?;
        match ubicacion {
            Some(lugar) if lugar != actor.lugar() => return Err(RechazoEscalar::ViajerosSeparados),
            None => ubicacion = Some(actor.lugar()),
            Some(_) => {}
        }
    }
    if !vistos.contains(&solicitud.responsable) {
        return Err(RechazoEscalar::ResponsableNoEsViajero {
            actor: solicitud.responsable,
        });
    }
    if let Some(ayudante) = solicitud.ayudante
        && (ayudante == solicitud.responsable || !vistos.contains(&ayudante))
    {
        return Err(RechazoEscalar::AyudanteInvalido { actor: ayudante });
    }
    Ok(())
}

fn tiene_en_etapa(mundo: &Mundo, solicitud: &SolicitudEscalar, valor: &str) -> bool {
    let caracteristica = caracteristica(valor);
    [solicitud.origen, solicitud.destino]
        .into_iter()
        .filter_map(|lugar| mundo.lugar(lugar))
        .any(|lugar| lugar.tiene_caracteristica(&caracteristica))
}

fn tiene(actor: &crate::Actor, valor: &str) -> bool {
    actor.tiene_caracteristica(&caracteristica(valor))
}

fn es_ayudante_competente(actor: &crate::Actor) -> bool {
    tiene(actor, "escalador") || tiene(actor, "fuerte") || tiene(actor, "equipoMontana")
}

fn caracteristica(valor: &str) -> Caracteristica {
    Caracteristica::nueva(valor.to_owned())
        .expect("las Características literales de Escalar son válidas")
}

#[cfg(test)]
mod pruebas {
    use std::num::NonZeroUsize;

    use super::*;
    use crate::acciones::ModoResolucion;
    use crate::azar::ErrorFuenteAzar;
    use crate::{
        ActorInicial, DatosInicialesMundo, ExtremoRelacion, Fecha, IdMapa, LoteOperaciones,
        LugarInicial, Mes, RelacionInicial, TipoRelacion,
    };

    struct AzarUnico {
        indice: Option<usize>,
        limites: Vec<usize>,
    }

    impl AzarUnico {
        fn nuevo(indice: usize) -> Self {
            Self {
                indice: Some(indice),
                limites: vec![],
            }
        }
    }

    impl FuenteAzar for AzarUnico {
        fn elegir_indice(
            &mut self,
            limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            self.limites.push(limite_exclusivo.get());
            self.indice.take().ok_or(ErrorFuenteAzar)
        }
    }

    fn id_actor(posicion: usize) -> IdActor {
        IdActor::desde_posicion(posicion).expect("ID de Actor válido")
    }

    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID de Lugar válido")
    }

    fn caracteristicas(valores: &[&str]) -> Vec<Caracteristica> {
        valores.iter().map(|valor| caracteristica(valor)).collect()
    }

    fn mundo_escalada(responsable: &[&str], ayudante: &[&str], destino: &[&str]) -> Mundo {
        Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID de mapa válido"),
            fecha: Fecha::nueva(2026, Mes::Octubre, 18).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: id_lugar(0),
                    caracteristicas: vec![],
                },
                LugarInicial {
                    id: id_lugar(1),
                    caracteristicas: caracteristicas(destino),
                },
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor(0),
                    lugar: id_lugar(0),
                    caracteristicas: caracteristicas(responsable),
                },
                ActorInicial {
                    id: id_actor(1),
                    lugar: id_lugar(0),
                    caracteristicas: caracteristicas(ayudante),
                },
            ],
            actor_jugador: id_actor(0),
            companeros: vec![id_actor(1)],
            variables_globales: vec![],
            relaciones: vec![RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id_lugar(0)),
                destino: ExtremoRelacion::Lugar(id_lugar(1)),
            }],
        })
        .expect("Mundo de escalada válido")
    }

    fn solicitud() -> SolicitudEscalar {
        SolicitudEscalar {
            responsable: id_actor(0),
            ayudante: Some(id_actor(1)),
            viajeros: vec![id_actor(0), id_actor(1)],
            origen: id_lugar(0),
            destino: id_lugar(1),
            carga_voluminosa: false,
        }
    }

    fn contexto_normal() -> ContextoMovimiento {
        ContextoMovimiento {
            ritmo: RitmoMovimiento::Normal,
            carga_voluminosa: false,
        }
    }

    #[test]
    fn distingue_imposible_determinista_e_incierta() {
        let plano = mundo_escalada(&[], &[], &[]);
        let imposible = evaluar_escalar(&plano, &solicitud()).expect("solicitud válida");
        assert_eq!(imposible.modo, Evaluacion::Imposible);
        assert_eq!(
            imposible.causa_imposible,
            Some(CausaImposibleEscalar::SinSuperficieEscalable)
        );

        let experto = mundo_escalada(&["escalador", "equipoMontana"], &[], &["empinado"]);
        let automatica = evaluar_escalar(&experto, &solicitud()).expect("solicitud válida");
        assert_eq!(
            automatica.modo,
            Evaluacion::Determinista(BandaResolucion::Holgada)
        );

        let ordinario = mundo_escalada(&[], &[], &["empinado"]);
        let incierta = evaluar_escalar(&ordinario, &solicitud()).expect("solicitud válida");
        assert!(matches!(incierta.modo, Evaluacion::Incierta(_)));
    }

    #[test]
    fn deriva_un_solo_factor_por_eje_causal() {
        let mundo = mundo_escalada(
            &["fuerte", "cargaVoluminosa", "herido"],
            &["escalador", "fuerte"],
            &["empinado", "pared", "acantilado", "rocaHumeda", "viento"],
        );
        let evaluacion = evaluar_escalar(&mundo, &solicitud()).expect("solicitud válida");
        assert_eq!(
            evaluacion.modo,
            Evaluacion::Incierta(PruebaD20 {
                dificultad: 19,
                modificador: 1,
            })
        );
    }

    #[test]
    fn fallo_solicita_herida_sin_mutar_hasta_confirmar_el_lote() {
        let mut mundo = mundo_escalada(&[], &[], &["empinado", "pared"]);
        let evaluacion = evaluar_escalar(&mundo, &solicitud()).expect("solicitud válida");
        let pronostico_antes = pronosticar_escalar(&evaluacion).expect("pronóstico válido");
        assert_eq!(pronostico_antes.duracion_minima, Duracion::desde_dias(1));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));

        let mut azar = AzarUnico::nuevo(0);
        let resultado = resolver_escalar(evaluacion, &mut azar).expect("resolución válida");
        assert_eq!(resultado.conclusion, ConclusionEscalar::Fallida);
        assert_eq!(resultado.duracion, Duracion::desde_dias(1));
        assert_eq!(azar.limites, vec![20]);
        let herido = caracteristica("herido");
        assert!(
            !mundo
                .actor(id_actor(0))
                .expect("Actor existente")
                .tiene_caracteristica(&herido)
        );

        let mut traza = resultado.traza;
        let lote = LoteOperaciones::nuevo(resultado.operaciones).expect("lote no vacío");
        let cambios = mundo.aplicar_lote(lote).expect("lote válido");
        traza.confirmar_operaciones(cambios.como_slice());
        assert_eq!(
            traza.operaciones_confirmadas.as_deref(),
            Some(cambios.como_slice())
        );
        assert!(
            mundo
                .actor(id_actor(0))
                .expect("Actor existente")
                .tiene_caracteristica(&herido)
        );
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
    }

    #[test]
    fn marcha_por_camino_es_automatica_y_orientarse_en_bosque_usa_d20() {
        let camino = mundo_escalada(&[], &[], &["camino", "arido", "rocoso"]);
        let mut azar = AzarUnico::nuevo(19);
        let marcha = resolver_movimiento_terrestre(
            &camino,
            &solicitud(),
            TipoMovimientoTerrestre::Marchar,
            contexto_normal(),
            &mut azar,
        )
        .expect("marcha válida");
        assert!(marcha.avanza);
        assert_eq!(marcha.banda, BandaResolucion::Holgada);
        assert_eq!(marcha.traza.accion, AccionParticular::Marchar);
        assert_eq!(
            marcha.traza.resoluciones,
            vec![ResolucionAplicada {
                sujeto: Some(id_actor(0)),
                modo: ModoResolucion::Determinista,
            }]
        );
        assert_eq!(marcha.traza.duracion, marcha.duracion);
        assert!(azar.limites.is_empty());

        let bosque = mundo_escalada(&["guia"], &[], &["boscoso"]);
        let mut azar = AzarUnico::nuevo(10);
        let orientacion = resolver_movimiento_terrestre(
            &bosque,
            &solicitud(),
            TipoMovimientoTerrestre::Orientarse,
            contexto_normal(),
            &mut azar,
        )
        .expect("orientación válida");
        assert!(orientacion.avanza);
        assert_eq!(azar.limites, vec![20]);
    }

    #[test]
    fn conserva_mas_de_seis_trazas_iguales_en_orden() {
        let mundo = mundo_escalada(&[], &[], &["camino"]);
        let mut trazas = Vec::new();
        for _ in 0..7 {
            let mut azar = AzarUnico::nuevo(19);
            let resultado = resolver_movimiento_terrestre(
                &mundo,
                &solicitud(),
                TipoMovimientoTerrestre::Marchar,
                contexto_normal(),
                &mut azar,
            )
            .expect("marcha automática válida");
            trazas.push(resultado.traza);
        }
        assert_eq!(trazas.len(), 7);
        assert!(trazas.windows(2).all(|par| par[0] == par[1]));
        assert!(
            trazas
                .iter()
                .all(|traza| traza.accion == AccionParticular::Marchar)
        );
    }

    #[test]
    fn descenso_fallido_avanza_y_solicita_herida() {
        let mundo = mundo_escalada(&[], &[], &["empinado", "estrecho", "rocoso"]);
        let mut azar = AzarUnico::nuevo(0);
        let descenso = resolver_movimiento_terrestre(
            &mundo,
            &solicitud(),
            TipoMovimientoTerrestre::Descender,
            contexto_normal(),
            &mut azar,
        )
        .expect("descenso válido");
        assert!(descenso.avanza);
        assert_eq!(descenso.banda, BandaResolucion::Fallida);
        assert!(matches!(
            descenso.operaciones.as_slice(),
            [OperacionMundo::AnadirCaracteristicaActor { actor, .. }] if *actor == id_actor(0)
        ));
    }

    #[test]
    fn camellos_y_caravana_sustituyen_la_marcha_y_consumen_alimento() {
        let mundo = mundo_escalada(&["camellero"], &[], &["camino", "arido"]);
        let mut azar = AzarUnico::nuevo(19);
        let cabalgata = resolver_movimiento_terrestre(
            &mundo,
            &solicitud(),
            TipoMovimientoTerrestre::Cabalgar,
            contexto_normal(),
            &mut azar,
        )
        .expect("cabalgata válida");
        assert!(cabalgata.avanza);
        assert_eq!(cabalgata.consumo_alimento_extra, 2);

        let mut solicitud_cargada = solicitud();
        solicitud_cargada.carga_voluminosa = true;
        let mut azar = AzarUnico::nuevo(19);
        let caravana = resolver_movimiento_terrestre(
            &mundo,
            &solicitud_cargada,
            TipoMovimientoTerrestre::GuiarCaravana,
            ContextoMovimiento {
                ritmo: RitmoMovimiento::Normal,
                carga_voluminosa: true,
            },
            &mut azar,
        )
        .expect("caravana válida");
        assert!(caravana.avanza);
        assert_eq!(caravana.consumo_alimento_extra, 2);
    }
}
