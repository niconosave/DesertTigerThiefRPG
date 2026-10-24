use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;

mod particulares;
mod viajar;

use crate::azar::{ErrorFuenteAzar, FuenteAzar};
use crate::{
    CambioConfirmado, Caracteristica, Duracion, Fecha, IdActor, IdLugar, LoteOperaciones, Mundo,
    OperacionMundo,
};
use particulares::movimiento::{
    ConclusionEscalar, ContextoMovimiento, RitmoMovimiento, SolicitudEscalar,
    TipoMovimientoTerrestre, evaluar_escalar, pronosticar_escalar, resolver_escalar,
    resolver_movimiento_terrestre,
};
use particulares::navegacion::{ManiobraNautica, resolver_navegacion};
use particulares::resolucion::ErrorResolucion;
pub use particulares::resolucion::{
    AccionParticular, BandaResolucion, CambioTransitorio, ContextoTraza, DatoResolucion, EjeCausal,
    FactorAplicado, ModoResolucion, ProcedenciaFactor, ResolucionAplicada, TrazaAccionParticular,
    VisibilidadFactor,
};
use particulares::supervivencia::{
    AccionRecurso, CalidadAgua, descansar, montar_campamento, resolver_recurso, tratar_agua,
};
use viajar::expedicion::{
    EstadoEmbarcacion, MedioAcuatico, MedioTerrestre, PreparacionViaje, RitmoViaje,
};
pub use viajar::proyecto::{
    EstrategiaViaje, FuenteMedio, MedioAcuaticoProyecto, MedioTerrestreProyecto, PlanViaje,
    PreparacionProyecto, PropuestaViaje, ProyectoViaje, RitmoProyecto, SubobjetivoViaje,
};

const TEXTO_PASO_EMPINADO: &str =
    "El camino atraviesa una pendiente empinada. El terreno vuelve difícil mantener el ritmo.";
const TEXTO_DECISION_PASO: &str = "La pendiente exige elegir cómo continuar el viaje.";
const TEXTO_ASCENSO_HOLGADO: &str =
    "Los viajeros superan la pendiente con seguridad y sin perder el ritmo.";
const TEXTO_ASCENSO_LOGRADO: &str = "Los viajeros completan el ascenso y continúan la marcha.";
const TEXTO_ASCENSO_COSTOSO: &str =
    "Los viajeros completan el ascenso, pero el esfuerzo deja fatigado al responsable.";
const TEXTO_ASCENSO_FALLIDO: &str =
    "El ascenso fracasa. Un viajero resulta herido y el viaje queda interrumpido.";
const TEXTO_PASO_MENOS_EMPINADO: &str =
    "Los viajeros encuentran un paso menos empinado a costa de una demora.";
const TEXTO_LLEGADA: &str = "El viaje finalmente alcanza su destino.";
const TEXTO_DESCENSO: &str = "El grupo desciende unido por el terreno escarpado.";
const TEXTO_ETAPA_COSTOSA: &str =
    "La etapa queda atrás, aunque el esfuerzo y el terreno pasan factura.";
const TEXTO_ETAPA_INTERRUMPIDA: &str = "Las condiciones impiden completar la etapa. La expedición se detiene en el último lugar alcanzado.";
const TEXTO_NAVEGACION: &str = "La expedición recorre por agua la siguiente etapa de su ruta.";
const TEXTO_ATRAQUE: &str = "La embarcación alcanza tierra y el grupo completa el desembarco.";
const TEXTO_RECURSOS: &str =
    "El grupo aprovecha los recursos del lugar para sostener la expedición.";
const TEXTO_CARENCIA: &str =
    "Las reservas no alcanzan para toda la jornada y el desgaste del grupo empeora.";
const TEXTO_CRISIS_NAUTICA: &str =
    "La embarcación está al límite. El grupo debe responder antes de continuar.";
const TEXTO_CRISIS_CONTENIDA: &str =
    "El grupo achica el agua y recupera control suficiente para continuar.";
const TEXTO_FONDEO_EMERGENCIA: &str =
    "La expedición fondea y detiene el viaje para no arriesgar la embarcación.";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolicitudAccion {
    Viajar { actor: IdActor, destino: IdLugar },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PronosticoViaje {
    pub destino: IdLugar,
    pub dias_minimos: u32,
    pub dias_maximos: u32,
    pub exposicion_conocida: u32,
    pub hay_incertidumbre: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdDecision(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdOpcionDecision(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RespuestaDecision {
    pub decision: IdDecision,
    pub opcion: IdOpcionDecision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolicitudDecision {
    pub id: IdDecision,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub rol: RolPresentacion,
    pub proyecto: Option<RegistroProyecto>,
    pub progreso: ProgresoViaje,
    pub opciones: Vec<OpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpcionDecision {
    pub id: IdOpcionDecision,
    pub texto: String,
    pub detalle: Option<String>,
    pub propuesta_viaje: Option<PropuestaViaje>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaveEscena {
    ViajeMarcha,
    ViajeObstaculo,
    ViajeRumboIncierto,
    ViajeLlegada,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RolPresentacion {
    Apertura,
    Deliberacion,
    Compromiso,
    PlanteamientoSituacion,
    DesarrolloSituacion,
    DesenlaceSituacion,
    Replanteamiento,
    Conclusion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistroProyecto {
    Proposito,
    Propuesta,
    Compromiso,
    Subobjetivo,
    Interpretacion,
    Replanteamiento,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgresoViaje {
    pub lugar_confirmado: IdLugar,
    pub siguiente_lugar: Option<IdLugar>,
    pub destino_final: IdLugar,
    pub duracion_transcurrida: Duracion,
    pub fecha_proyectada: Fecha,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdMomentoCausal(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MomentoCausal {
    pub id: IdMomentoCausal,
    pub subobjetivo: Option<SubobjetivoViaje>,
    pub traza: TrazaAccionParticular,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentoNarrativo {
    pub titulo: String,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub rol: RolPresentacion,
    pub proyecto: Option<RegistroProyecto>,
    pub progreso: ProgresoViaje,
    pub momentos: Vec<MomentoCausal>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TramoAccion {
    pub emisiones: Vec<EmisionAccion>,
    pub cierre: CierreTramo,
}

#[derive(Debug, Eq, PartialEq)]
pub enum EmisionAccion {
    Narrativa(FragmentoNarrativo),
    ResultadoSemantico(ResultadoSemantico),
}

#[derive(Debug, Eq, PartialEq)]
pub enum CierreTramo {
    EsperandoDecision(SolicitudDecision),
    Completada(FinalizacionAccion),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResultadoSemantico {
    ActoresTrasladados {
        actores: Vec<IdActor>,
        origen: IdLugar,
        destino: IdLugar,
    },
    CaracteristicaActorAnadida {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    CaracteristicaActorQuitada {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FinalizacionAccion {
    pub duracion: Duracion,
}

pub struct EjecucionAccion {
    estado: EstadoEjecucion,
}

enum EstadoEjecucion {
    Viajar(EstadoViajar),
    Finalizada,
}

enum EstadoViajar {
    Preparacion(EsperaConfiguracion),
    Ritmo(EsperaConfiguracion),
    Medio(EsperaConfiguracion),
    CrisisNautica(EsperaNavegar),
    PasoEmpinado(EsperaViajar),
}

struct EsperaConfiguracion {
    expedicion: viajar::expedicion::Expedicion,
    decision: IdDecision,
    agua_disponible: bool,
    alimento_disponible: bool,
    camellos_disponibles: bool,
}

struct EsperaViajar {
    expedicion: viajar::expedicion::Expedicion,
    decision: IdDecision,
    solicitud_escalar: SolicitudEscalar,
}

struct EsperaNavegar {
    expedicion: viajar::expedicion::Expedicion,
    decision: IdDecision,
    lugar_nautico: IdLugar,
}

pub struct InicioAccion {
    pub ejecucion: EjecucionAccion,
    pub tramo: TramoAccion,
}

pub fn destinos_disponibles(mundo: &Mundo, actor: IdActor) -> Result<Vec<IdLugar>, RechazoAccion> {
    let actor_actual = mundo
        .actor(actor)
        .ok_or(RechazoAccion::ActorDesconocido { actor })?;
    validar_jugador_vivo(mundo, actor, actor_actual)?;
    if actor != mundo.actor_jugador() && mundo.grupo().contiene(actor) {
        return Err(RechazoAccion::ViajeIndependienteDeIntegranteGrupo { actor });
    }

    let viajeros = viajeros_para_consulta(mundo, actor);
    Ok(mundo
        .lugares()
        .iter()
        .filter(|lugar| {
            lugar.id() != actor_actual.lugar()
                && viajar::planificacion::ruta_ponderada(
                    mundo,
                    &viajeros,
                    actor_actual.lugar(),
                    lugar.id(),
                )
                .is_some_and(|ruta| ruta_admite_propuesta(mundo, &viajeros, &ruta))
        })
        .map(|lugar| lugar.id())
        .collect())
}

pub fn pronosticos_viaje(
    mundo: &Mundo,
    actor: IdActor,
) -> Result<Vec<PronosticoViaje>, RechazoAccion> {
    let actual = mundo
        .actor(actor)
        .ok_or(RechazoAccion::ActorDesconocido { actor })?;
    let viajeros = viajeros_para_consulta(mundo, actor);
    destinos_disponibles(mundo, actor)?
        .into_iter()
        .map(|destino| {
            let ruta =
                viajar::planificacion::ruta_ponderada(mundo, &viajeros, actual.lugar(), destino)
                    .ok_or(RechazoAccion::DestinoSinRutaConectada {
                        origen: actual.lugar(),
                        destino,
                    })?;
            let (dias, exposicion, incertidumbre) = viajar::planificacion::coste_ruta(
                mundo, &viajeros, &ruta,
            )
            .ok_or(RechazoAccion::DestinoSinRutaConectada {
                origen: actual.lugar(),
                destino,
            })?;
            Ok(PronosticoViaje {
                destino,
                dias_minimos: dias,
                dias_maximos: dias,
                exposicion_conocida: exposicion,
                hay_incertidumbre: incertidumbre > 0,
            })
        })
        .collect()
}

pub fn iniciar(
    mundo: &mut Mundo,
    solicitud: SolicitudAccion,
    azar: &mut dyn FuenteAzar,
) -> Result<InicioAccion, ErrorMotor> {
    match solicitud {
        SolicitudAccion::Viajar { actor, destino } => iniciar_viaje(mundo, actor, destino, azar),
    }
}

pub fn responder(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor> {
    if matches!(
        ejecucion.estado,
        EstadoEjecucion::Viajar(EstadoViajar::Preparacion(_))
    ) {
        return responder_preparacion(mundo, ejecucion, respuesta, azar);
    }
    if matches!(
        ejecucion.estado,
        EstadoEjecucion::Viajar(EstadoViajar::Ritmo(_))
    ) {
        return responder_ritmo(mundo, ejecucion, respuesta, azar);
    }
    if matches!(
        ejecucion.estado,
        EstadoEjecucion::Viajar(EstadoViajar::Medio(_))
    ) {
        return responder_medio(mundo, ejecucion, respuesta, azar);
    }
    if matches!(
        ejecucion.estado,
        EstadoEjecucion::Viajar(EstadoViajar::CrisisNautica(_))
    ) {
        return responder_crisis_nautica(mundo, ejecucion, respuesta, azar);
    }
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::PasoEmpinado(espera)) => espera,
        EstadoEjecucion::Viajar(
            EstadoViajar::Preparacion(_)
            | EstadoViajar::Ritmo(_)
            | EstadoViajar::Medio(_)
            | EstadoViajar::CrisisNautica(_),
        ) => unreachable!("las decisiones de configuración se resolvieron antes"),
        EstadoEjecucion::Finalizada => {
            return Err(ErrorMotor::Rechazo(RechazoAccion::DecisionNoVigente {
                decision: respuesta.decision,
            }));
        }
    };

    if respuesta.decision != espera.decision {
        return Err(ErrorMotor::Rechazo(RechazoAccion::DecisionNoVigente {
            decision: respuesta.decision,
        }));
    }
    if respuesta.opcion.0 >= 2 {
        return Err(ErrorMotor::Rechazo(RechazoAccion::OpcionNoVigente {
            decision: respuesta.decision,
            opcion: respuesta.opcion,
        }));
    }

    if !contexto_viaje_valido(espera) {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    let estado = std::mem::replace(&mut ejecucion.estado, EstadoEjecucion::Finalizada);
    let EstadoEjecucion::Viajar(EstadoViajar::PasoEmpinado(mut espera)) = estado else {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    };
    let actor = espera.expedicion.dirigente;
    let solicitud_escalar = espera.solicitud_escalar.clone();

    let (texto, duracion, mut operaciones, avanza, traza) = match respuesta.opcion.0 {
        0 => {
            let evaluacion = evaluar_escalar(mundo, &solicitud_escalar)
                .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
            let pronostico = pronosticar_escalar(&evaluacion).map_err(error_resolucion)?;
            let resultado = resolver_escalar(evaluacion, azar).map_err(error_resolucion)?;
            if pronostico.duracion_minima == pronostico.duracion_maxima
                && resultado.duracion != pronostico.duracion_minima
            {
                return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
            }
            match resultado.conclusion {
                ConclusionEscalar::Imposible(_) => {
                    return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
                }
                ConclusionEscalar::Superada(banda) => {
                    let texto = match banda {
                        BandaResolucion::Holgada => TEXTO_ASCENSO_HOLGADO,
                        BandaResolucion::Lograda => TEXTO_ASCENSO_LOGRADO,
                        BandaResolucion::Costosa => TEXTO_ASCENSO_COSTOSO,
                        BandaResolucion::Fallida => {
                            return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
                        }
                    };
                    (
                        texto,
                        resultado.duracion,
                        resultado.operaciones,
                        true,
                        Some(resultado.traza),
                    )
                }
                ConclusionEscalar::Fallida => (
                    TEXTO_ASCENSO_FALLIDO,
                    resultado.duracion,
                    resultado.operaciones,
                    false,
                    Some(resultado.traza),
                ),
            }
        }
        1 => (
            TEXTO_PASO_MENOS_EMPINADO,
            Duracion::desde_dias(2),
            vec![],
            true,
            None,
        ),
        _ => return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
    };

    let mut emisiones = vec![if let Some(traza) = traza {
        narrativa_con_traza(
            texto,
            ClaveEscena::ViajeObstaculo,
            &mut espera.expedicion,
            traza,
        )?
    } else {
        narrativa(texto, ClaveEscena::ViajeObstaculo, &espera.expedicion)?
    }];
    if avanza {
        let siguiente = espera
            .expedicion
            .siguiente()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        operaciones.push(OperacionMundo::Trasladar {
            actor,
            destino: siguiente,
        });
        let reservas_antes = espera.expedicion.reservas;
        let duracion_movimiento = duracion;
        let supervivencia = atender_supervivencia(
            mundo,
            &mut espera.expedicion,
            solicitud_escalar.origen,
            duracion,
            0,
            0,
            azar,
        )?;
        let duracion = duracion_movimiento
            .checked_add(supervivencia.duracion)
            .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        emisiones.extend(narrar_supervivencia(
            &mut espera.expedicion,
            supervivencia.trazas,
            duracion_movimiento,
        )?);
        if espera.expedicion.reservas.agua > reservas_antes.agua
            || espera.expedicion.reservas.alimento > reservas_antes.alimento
        {
            emisiones.push(narrativa_con_progreso(
                TEXTO_RECURSOS,
                ClaveEscena::ViajeMarcha,
                &espera.expedicion,
                duracion,
            )?);
        }
        let (carencias, hay_carencia) =
            consumir_reservas(mundo, &mut espera.expedicion, duracion, 0, 0)?;
        if hay_carencia {
            emisiones.push(narrativa_con_progreso(
                TEXTO_CARENCIA,
                ClaveEscena::ViajeObstaculo,
                &espera.expedicion,
                duracion,
            )?);
        }
        operaciones.extend(carencias);
        emisiones.extend(aplicar_operaciones(mundo, operaciones)?);
        espera
            .expedicion
            .confirmar_avance(duracion)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        emisiones.push(registro_proyecto(
            "El paso ha quedado superado. El grupo acepta el nuevo terreno como posición confirmada y vuelve a orientar el plan hacia el destino.",
            "El paso superado",
            RolPresentacion::DesenlaceSituacion,
            RegistroProyecto::Interpretacion,
            &espera.expedicion,
        )?);
        recalcular_ruta(mundo, &mut espera.expedicion)?;
        let (estado, mut tramo) = continuar_expedicion(mundo, espera.expedicion, azar)?;
        emisiones.append(&mut tramo.emisiones);
        ejecucion.estado = estado;
        Ok(TramoAccion {
            emisiones,
            cierre: tramo.cierre,
        })
    } else {
        emisiones.extend(aplicar_operaciones(mundo, operaciones)?);
        espera
            .expedicion
            .acumular_sin_avance(duracion)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        Ok(TramoAccion {
            emisiones,
            cierre: CierreTramo::Completada(FinalizacionAccion {
                duracion: espera.expedicion.duracion,
            }),
        })
    }
}

fn iniciar_viaje(
    mundo: &mut Mundo,
    actor: IdActor,
    destino: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<InicioAccion, ErrorMotor> {
    let ruta = validar_viaje(mundo, actor, destino).map_err(ErrorMotor::Rechazo)?;
    let viajeros = viajeros_de(mundo, actor)?;
    let mut expedicion =
        viajar::expedicion::Expedicion::nueva(actor, viajeros, destino, ruta, mundo.fecha())
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let proyecto = crear_proyecto(mundo, &expedicion)?;
    expedicion.fijar_proyecto(proyecto);
    let (agua_disponible, alimento_disponible) = abastecimiento_en_origen(mundo, actor)?;
    let camellos_disponibles = expedicion.viajeros.iter().copied().any(|viajero| {
        mundo
            .actor(viajero)
            .is_some_and(|actor| actor.tiene_caracteristica(&caracteristica_regla("conCamellos")))
    });
    let mut emisiones = vec![registro_proyecto(
        "El grupo se propone abandonar su lugar actual y alcanzar el destino elegido.",
        "Un propósito compartido",
        RolPresentacion::Apertura,
        RegistroProyecto::Proposito,
        &expedicion,
    )?];
    let propuestas = expedicion
        .proyecto
        .as_ref()
        .map(|proyecto| proyecto.propuestas.clone())
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let (estado, cierre) = if propuestas.len() > 1 {
        emisiones.push(registro_proyecto(
            "Al deliberar, los viajeros comparan todas las maneras realizables de alcanzar el destino y la procedencia concreta de cada medio.",
            "Deliberación",
            RolPresentacion::Deliberacion,
            RegistroProyecto::Propuesta,
            &expedicion,
        )?);
        let decision = IdDecision(u32::MAX - 2);
        let etiquetas = propuestas
            .iter()
            .map(etiqueta_propuesta)
            .collect::<Vec<_>>();
        let mut solicitud = solicitud_decision(
            decision,
            "El dirigente debe elegir qué plan compromete al grupo.",
            ClaveEscena::ViajeMarcha,
            &etiquetas,
            &expedicion,
        )?;
        solicitud.rol = RolPresentacion::Deliberacion;
        solicitud.proyecto = Some(RegistroProyecto::Propuesta);
        for (opcion, propuesta) in solicitud.opciones.iter_mut().zip(propuestas) {
            opcion.detalle = Some(detalle_propuesta(&propuesta));
            opcion.propuesta_viaje = Some(propuesta);
        }
        (
            EstadoEjecucion::Viajar(EstadoViajar::Medio(EsperaConfiguracion {
                expedicion,
                decision,
                agua_disponible,
                alimento_disponible,
                camellos_disponibles,
            })),
            CierreTramo::EsperandoDecision(solicitud),
        )
    } else {
        let referencia = propuestas
            .first()
            .map(|propuesta| propuesta.referencia)
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        expedicion
            .proyecto
            .as_mut()
            .and_then(|proyecto| proyecto.comprometer(referencia))
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        aplicar_medios_comprometidos(&mut expedicion)?;
        emisiones.push(registro_proyecto(
            "No hay otro medio terrestre asegurado. El dirigente adopta un plan directo a pie y el grupo se dispone a precisar cómo ejecutarlo.",
            "Plan adoptado",
            RolPresentacion::Compromiso,
            RegistroProyecto::Compromiso,
            &expedicion,
        )?);
        let (estado, mut tramo) = configurar_ritmo(
            mundo,
            expedicion,
            agua_disponible,
            alimento_disponible,
            camellos_disponibles,
            azar,
        )?;
        emisiones.append(&mut tramo.emisiones);
        (estado, tramo.cierre)
    };
    let tramo = TramoAccion { emisiones, cierre };
    Ok(InicioAccion {
        ejecucion: EjecucionAccion { estado },
        tramo,
    })
}

fn configurar_ritmo(
    mundo: &mut Mundo,
    expedicion: viajar::expedicion::Expedicion,
    agua_disponible: bool,
    alimento_disponible: bool,
    camellos_disponibles: bool,
    azar: &mut dyn FuenteAzar,
) -> Result<(EstadoEjecucion, TramoAccion), ErrorMotor> {
    if !ruta_tiene_movimiento_terrestre(mundo, &expedicion) {
        return configurar_preparacion(
            mundo,
            expedicion,
            agua_disponible,
            alimento_disponible,
            camellos_disponibles,
            azar,
        );
    }
    let decision = IdDecision(u32::MAX);
    let mut solicitud = solicitud_decision(
        decision,
        "Elige el ritmo con el que el grupo intentará cumplir el plan.",
        ClaveEscena::ViajeMarcha,
        &["Ritmo prudente", "Ritmo normal", "Ritmo forzado"],
        &expedicion,
    )?;
    solicitud.rol = RolPresentacion::Compromiso;
    solicitud.proyecto = Some(RegistroProyecto::Compromiso);
    Ok((
        EstadoEjecucion::Viajar(EstadoViajar::Ritmo(EsperaConfiguracion {
            expedicion,
            decision,
            agua_disponible,
            alimento_disponible,
            camellos_disponibles,
        })),
        TramoAccion {
            emisiones: vec![],
            cierre: CierreTramo::EsperandoDecision(solicitud),
        },
    ))
}

fn configurar_preparacion(
    mundo: &mut Mundo,
    mut expedicion: viajar::expedicion::Expedicion,
    agua_disponible: bool,
    alimento_disponible: bool,
    camellos_disponibles: bool,
    azar: &mut dyn FuenteAzar,
) -> Result<(EstadoEjecucion, TramoAccion), ErrorMotor> {
    let bien_aprovisionado = mundo.actor(expedicion.dirigente).is_some_and(|actor| {
        actor.tiene_caracteristica(&caracteristica_regla("bienAprovisionado"))
    });
    if !agua_disponible && !alimento_disponible && !bien_aprovisionado {
        expedicion
            .preparar(PreparacionViaje::Ligera, 0, 0)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
        if let Some(plan) = expedicion
            .proyecto
            .as_mut()
            .and_then(|proyecto| proyecto.plan.as_mut())
        {
            plan.preparacion = PreparacionProyecto::Ligera;
        }
        return continuar_expedicion(mundo, expedicion, azar);
    }
    let decision = IdDecision(u32::MAX - 1);
    let dias_previstos = u32::try_from(expedicion.ruta.len().saturating_sub(1))
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    let bono = u32::from(bien_aprovisionado) * 2;
    let describir = |nombre: &str, jornadas: u32| {
        let agua = if agua_disponible || bien_aprovisionado {
            jornadas.saturating_add(bono)
        } else {
            0
        };
        let alimento = if alimento_disponible || bien_aprovisionado {
            jornadas.saturating_add(bono)
        } else {
            0
        };
        format!("{nombre} — agua {agua}, alimento {alimento} jornadas-grupo")
    };
    let detalles = [
        describir("Ligera", 1),
        describir("Suficiente", dias_previstos.saturating_add(1).max(2)),
        describir("Cargada", dias_previstos.saturating_add(3).max(4)),
    ];
    let mut solicitud = solicitud_decision(
        decision,
        "Elige cuántas provisiones comprometer con el plan.",
        ClaveEscena::ViajeMarcha,
        &[
            "Preparación ligera",
            "Preparación suficiente",
            "Preparación cargada",
        ],
        &expedicion,
    )?;
    for (opcion, detalle) in solicitud.opciones.iter_mut().zip(detalles) {
        opcion.detalle = Some(detalle);
    }
    solicitud.rol = RolPresentacion::Compromiso;
    solicitud.proyecto = Some(RegistroProyecto::Compromiso);
    Ok((
        EstadoEjecucion::Viajar(EstadoViajar::Preparacion(EsperaConfiguracion {
            expedicion,
            decision,
            agua_disponible,
            alimento_disponible,
            camellos_disponibles,
        })),
        TramoAccion {
            emisiones: vec![],
            cierre: CierreTramo::EsperandoDecision(solicitud),
        },
    ))
}

fn responder_preparacion(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor> {
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::Preparacion(espera)) => espera,
        _ => unreachable!(),
    };
    validar_respuesta(espera.decision, 3, respuesta)?;
    let estado = std::mem::replace(&mut ejecucion.estado, EstadoEjecucion::Finalizada);
    let EstadoEjecucion::Viajar(EstadoViajar::Preparacion(mut espera)) = estado else {
        unreachable!()
    };
    let preparacion = match respuesta.opcion.0 {
        0 => PreparacionViaje::Ligera,
        1 => PreparacionViaje::Suficiente,
        2 => PreparacionViaje::Cargada,
        _ => unreachable!(),
    };
    let dias_previstos = u32::try_from(espera.expedicion.ruta.len().saturating_sub(1))
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    let jornadas = match preparacion {
        PreparacionViaje::Ligera => 1,
        PreparacionViaje::Suficiente => dias_previstos.saturating_add(1).max(2),
        PreparacionViaje::Cargada => dias_previstos.saturating_add(3).max(4),
    };
    let bien_aprovisionado = mundo
        .actor(espera.expedicion.dirigente)
        .is_some_and(|actor| {
            actor.tiene_caracteristica(&caracteristica_regla("bienAprovisionado"))
        });
    let bono = u32::from(bien_aprovisionado) * 2;
    espera
        .expedicion
        .preparar(
            preparacion,
            if espera.agua_disponible || bien_aprovisionado {
                jornadas
            } else {
                0
            }
            .saturating_add(bono),
            if espera.alimento_disponible || bien_aprovisionado {
                jornadas
            } else {
                0
            }
            .saturating_add(bono),
        )
        .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
    if let Some(plan) = espera
        .expedicion
        .proyecto
        .as_mut()
        .and_then(|proyecto| proyecto.plan.as_mut())
    {
        plan.preparacion = match preparacion {
            PreparacionViaje::Ligera => PreparacionProyecto::Ligera,
            PreparacionViaje::Suficiente => PreparacionProyecto::Suficiente,
            PreparacionViaje::Cargada => PreparacionProyecto::Cargada,
        };
    }
    let mut emisiones = Vec::new();
    if bien_aprovisionado {
        emisiones.extend(aplicar_operaciones(
            mundo,
            vec![OperacionMundo::QuitarCaracteristicaActor {
                actor: espera.expedicion.dirigente,
                caracteristica: caracteristica_regla("bienAprovisionado"),
            }],
        )?);
    }
    emisiones.push(registro_proyecto(
        "El grupo reúne las provisiones elegidas. El plan ya tiene medios, ritmo y preparación concretos; comienza la marcha.",
        "Preparativos concluidos",
        RolPresentacion::Compromiso,
        RegistroProyecto::Compromiso,
        &espera.expedicion,
    )?);
    let (estado, mut tramo) = continuar_expedicion(mundo, espera.expedicion, azar)?;
    emisiones.append(&mut tramo.emisiones);
    ejecucion.estado = estado;
    Ok(TramoAccion {
        emisiones,
        cierre: tramo.cierre,
    })
}

fn ruta_tiene_movimiento_terrestre(
    mundo: &Mundo,
    expedicion: &viajar::expedicion::Expedicion,
) -> bool {
    expedicion
        .ruta
        .windows(2)
        .any(|etapa| !es_lugar_acuatico(mundo, etapa[0]) && !es_lugar_acuatico(mundo, etapa[1]))
}

fn responder_ritmo(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor> {
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::Ritmo(espera)) => espera,
        _ => unreachable!(),
    };
    validar_respuesta(espera.decision, 3, respuesta)?;
    let estado = std::mem::replace(&mut ejecucion.estado, EstadoEjecucion::Finalizada);
    let EstadoEjecucion::Viajar(EstadoViajar::Ritmo(mut espera)) = estado else {
        unreachable!()
    };
    espera.expedicion.fijar_ritmo(match respuesta.opcion.0 {
        0 => RitmoViaje::Prudente,
        1 => RitmoViaje::Normal,
        2 => RitmoViaje::Forzado,
        _ => unreachable!(),
    });
    if let Some(plan) = espera
        .expedicion
        .proyecto
        .as_mut()
        .and_then(|proyecto| proyecto.plan.as_mut())
    {
        plan.ritmo = match respuesta.opcion.0 {
            0 => RitmoProyecto::Prudente,
            1 => RitmoProyecto::Normal,
            2 => RitmoProyecto::Forzado,
            _ => unreachable!(),
        };
    }
    let (estado, tramo) = configurar_preparacion(
        mundo,
        espera.expedicion,
        espera.agua_disponible,
        espera.alimento_disponible,
        espera.camellos_disponibles,
        azar,
    )?;
    ejecucion.estado = estado;
    Ok(tramo)
}

fn responder_medio(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor> {
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::Medio(espera)) => espera,
        _ => unreachable!(),
    };
    let cantidad = espera
        .expedicion
        .proyecto
        .as_ref()
        .and_then(|proyecto| u32::try_from(proyecto.propuestas.len()).ok())
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    validar_respuesta(espera.decision, cantidad, respuesta)?;
    let estado = std::mem::replace(&mut ejecucion.estado, EstadoEjecucion::Finalizada);
    let EstadoEjecucion::Viajar(EstadoViajar::Medio(mut espera)) = estado else {
        unreachable!()
    };
    let referencia = espera
        .expedicion
        .proyecto
        .as_ref()
        .and_then(|proyecto| proyecto.propuestas.get(respuesta.opcion.0 as usize))
        .map(|propuesta| propuesta.referencia)
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    espera
        .expedicion
        .proyecto
        .as_mut()
        .and_then(|proyecto| proyecto.comprometer(referencia))
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    aplicar_medios_comprometidos(&mut espera.expedicion)?;
    let compromiso = registro_proyecto(
        "El dirigente adopta una de las propuestas realizables. Sus medios y procedencias quedan comprometidos antes de comenzar la marcha.",
        "Plan adoptado",
        RolPresentacion::Compromiso,
        RegistroProyecto::Compromiso,
        &espera.expedicion,
    )?;
    let (estado, mut tramo) = configurar_ritmo(
        mundo,
        espera.expedicion,
        espera.agua_disponible,
        espera.alimento_disponible,
        espera.camellos_disponibles,
        azar,
    )?;
    tramo.emisiones.insert(0, compromiso);
    ejecucion.estado = estado;
    Ok(tramo)
}

fn aplicar_medios_comprometidos(
    expedicion: &mut viajar::expedicion::Expedicion,
) -> Result<(), ErrorMotor> {
    let propuesta = expedicion
        .proyecto
        .as_ref()
        .and_then(|proyecto| proyecto.plan.as_ref())
        .map(|plan| plan.propuesta.clone())
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    expedicion.fijar_medio_terrestre(match propuesta.medio_terrestre {
        MedioTerrestreProyecto::APie => MedioTerrestre::APie,
        MedioTerrestreProyecto::Camellos => MedioTerrestre::Camellos,
    });
    expedicion.fijar_medio_acuatico(match propuesta.medio_acuatico {
        MedioAcuaticoProyecto::Ninguno => MedioAcuatico::Ninguno,
        MedioAcuaticoProyecto::Bote => MedioAcuatico::Bote,
        MedioAcuaticoProyecto::Barco => MedioAcuatico::Barco,
    });
    Ok(())
}

fn responder_crisis_nautica(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor> {
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::CrisisNautica(espera)) => espera,
        _ => unreachable!(),
    };
    validar_respuesta(espera.decision, 2, respuesta)?;
    let estado = std::mem::replace(&mut ejecucion.estado, EstadoEjecucion::Finalizada);
    let EstadoEjecucion::Viajar(EstadoViajar::CrisisNautica(mut espera)) = estado else {
        unreachable!()
    };
    let lugar = espera.lugar_nautico;
    let responsable = espera
        .expedicion
        .viajeros
        .iter()
        .copied()
        .find(|&id| actor_tiene(mundo, id, "fuerte"))
        .unwrap_or(espera.expedicion.dirigente);
    let maniobra = if respuesta.opcion.0 == 0 {
        ManiobraNautica::Achicar
    } else {
        ManiobraNautica::Fondear
    };
    let resultado = resolver_navegacion(
        mundo,
        maniobra,
        espera.expedicion.medio_acuatico,
        espera.expedicion.estado_embarcacion,
        responsable,
        &espera.expedicion.viajeros,
        lugar,
        lugar,
        azar,
    )
    .map_err(error_resolucion)?;
    espera.expedicion.fijar_estado_embarcacion(resultado.estado);
    let mut emisiones = vec![narrativa_con_traza(
        if maniobra == ManiobraNautica::Achicar {
            TEXTO_CRISIS_CONTENIDA
        } else {
            TEXTO_FONDEO_EMERGENCIA
        },
        ClaveEscena::ViajeObstaculo,
        &mut espera.expedicion,
        resultado.traza,
    )?];
    if !resultado.operaciones.is_empty() {
        emisiones.extend(aplicar_operaciones(mundo, resultado.operaciones)?);
    }
    if maniobra == ManiobraNautica::Achicar
        && resultado.estado != EstadoEmbarcacion::NaufragioInminente
    {
        let (estado, mut tramo) = continuar_expedicion(mundo, espera.expedicion, azar)?;
        emisiones.append(&mut tramo.emisiones);
        ejecucion.estado = estado;
        Ok(TramoAccion {
            emisiones,
            cierre: tramo.cierre,
        })
    } else {
        Ok(TramoAccion {
            emisiones,
            cierre: CierreTramo::Completada(FinalizacionAccion {
                duracion: espera.expedicion.duracion,
            }),
        })
    }
}

fn validar_respuesta(
    decision: IdDecision,
    opciones: u32,
    respuesta: RespuestaDecision,
) -> Result<(), ErrorMotor> {
    if respuesta.decision != decision {
        return Err(ErrorMotor::Rechazo(RechazoAccion::DecisionNoVigente {
            decision: respuesta.decision,
        }));
    }
    if respuesta.opcion.0 >= opciones {
        return Err(ErrorMotor::Rechazo(RechazoAccion::OpcionNoVigente {
            decision,
            opcion: respuesta.opcion,
        }));
    }
    Ok(())
}

fn abastecimiento_en_origen(mundo: &Mundo, actor: IdActor) -> Result<(bool, bool), ErrorMotor> {
    let lugar = mundo
        .ubicacion_actor(actor)
        .and_then(|id| mundo.lugar(id))
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let tiene = |rasgo| lugar.tiene_caracteristica(&caracteristica_regla(rasgo));
    let civilizado = tiene("poblado") || tiene("posada") || tiene("mercado");
    Ok((
        civilizado || tiene("aguaDulce") || tiene("pozo") || tiene("manantial"),
        civilizado,
    ))
}

fn continuar_expedicion(
    mundo: &mut Mundo,
    mut expedicion: viajar::expedicion::Expedicion,
    azar: &mut dyn FuenteAzar,
) -> Result<(EstadoEjecucion, TramoAccion), ErrorMotor> {
    let mut emisiones = Vec::new();
    while !expedicion.llego() {
        let origen = expedicion
            .actual()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        let destino = expedicion
            .siguiente()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        let movimiento = viajar::situaciones::movimiento_etapa(mundo, origen, destino)
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        if let Some(proyecto) = expedicion.proyecto.as_mut() {
            proyecto.subobjetivo = Some(
                if movimiento == viajar::situaciones::MovimientoEtapa::Escalar {
                    SubobjetivoViaje::SuperarPaso(destino)
                } else {
                    SubobjetivoViaje::Alcanzar(destino)
                },
            );
        }
        emisiones.push(registro_proyecto(
            "Con el plan decidido, los viajeros concentran ahora su esfuerzo en alcanzar el siguiente lugar sin separar al grupo.",
            "El siguiente esfuerzo",
            RolPresentacion::PlanteamientoSituacion,
            RegistroProyecto::Subobjetivo,
            &expedicion,
        )?);
        if es_lugar_acuatico(mundo, origen) || es_lugar_acuatico(mundo, destino) {
            if expedicion.medio_acuatico == MedioAcuatico::Ninguno {
                return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
            }
            let (estado, tramo, siguiente_expedicion) =
                resolver_etapa_acuatica(mundo, expedicion, origen, destino, azar)?;
            emisiones.extend(tramo.emisiones);
            if let Some(expedicion_continua) = siguiente_expedicion {
                expedicion = expedicion_continua;
                if !expedicion.llego() {
                    recalcular_ruta(mundo, &mut expedicion)?;
                }
                continue;
            }
            return Ok((
                estado,
                TramoAccion {
                    emisiones,
                    cierre: tramo.cierre,
                },
            ));
        }
        if movimiento == viajar::situaciones::MovimientoEtapa::Escalar {
            let posicion = u32::try_from(expedicion.posicion)
                .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
            let decision = IdDecision(posicion);
            let solicitud_escalar = SolicitudEscalar {
                responsable: expedicion.dirigente,
                ayudante: expedicion
                    .viajeros
                    .iter()
                    .copied()
                    .find(|&viajero| viajero != expedicion.dirigente),
                viajeros: expedicion.viajeros.clone(),
                origen,
                destino,
                carga_voluminosa: expedicion.preparacion == PreparacionViaje::Cargada,
            };
            emisiones.push(narrativa(
                TEXTO_PASO_EMPINADO,
                ClaveEscena::ViajeObstaculo,
                &expedicion,
            )?);
            let solicitud = solicitud_decision(
                decision,
                TEXTO_DECISION_PASO,
                ClaveEscena::ViajeObstaculo,
                &["Ascender directamente", "Buscar un paso menos empinado"],
                &expedicion,
            )?;
            return Ok((
                EstadoEjecucion::Viajar(EstadoViajar::PasoEmpinado(EsperaViajar {
                    expedicion,
                    decision,
                    solicitud_escalar,
                })),
                TramoAccion {
                    emisiones,
                    cierre: CierreTramo::EsperandoDecision(solicitud),
                },
            ));
        }

        let usa_camellos = expedicion.medio_terrestre == MedioTerrestre::Camellos
            && !etapa_tiene(mundo, origen, destino, "pantanoso")
            && !etapa_tiene(mundo, origen, destino, "boscoso")
            && movimiento == viajar::situaciones::MovimientoEtapa::Marchar;
        let responsable = if usa_camellos {
            expedicion
                .viajeros
                .iter()
                .copied()
                .find(|&viajero| actor_tiene(mundo, viajero, "camellero"))
                .unwrap_or(expedicion.dirigente)
        } else {
            expedicion.dirigente
        };
        let solicitud = SolicitudEscalar {
            responsable,
            ayudante: expedicion
                .viajeros
                .iter()
                .copied()
                .find(|&viajero| viajero != responsable),
            viajeros: expedicion.viajeros.clone(),
            origen,
            destino,
            carga_voluminosa: expedicion.preparacion == PreparacionViaje::Cargada,
        };
        let contexto_movimiento = ContextoMovimiento {
            ritmo: match expedicion.ritmo {
                RitmoViaje::Prudente => RitmoMovimiento::Prudente,
                RitmoViaje::Normal => RitmoMovimiento::Normal,
                RitmoViaje::Forzado => RitmoMovimiento::Forzado,
            },
            carga_voluminosa: expedicion.preparacion == PreparacionViaje::Cargada,
        };
        if movimiento == viajar::situaciones::MovimientoEtapa::Marchar
            && requiere_orientarse(mundo, origen, destino)
        {
            let orientacion = resolver_movimiento_terrestre(
                mundo,
                &solicitud,
                TipoMovimientoTerrestre::Orientarse,
                contexto_movimiento,
                azar,
            )
            .map_err(error_resolucion)?;
            expedicion
                .acumular_sin_avance(orientacion.duracion)
                .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
            emisiones.push(narrativa_con_traza(
                if orientacion.avanza {
                    "El grupo contrasta el terreno con lo que conoce y mantiene un rumbo seguro."
                } else {
                    "Las referencias no bastan: el grupo pierde el rumbo y debe detener el proyecto."
                },
                ClaveEscena::ViajeRumboIncierto,
                &mut expedicion,
                orientacion.traza.clone(),
            )?);
            if !orientacion.avanza {
                emisiones.push(narrativa(
                    TEXTO_ETAPA_INTERRUMPIDA,
                    ClaveEscena::ViajeRumboIncierto,
                    &expedicion,
                )?);
                return Ok((
                    EstadoEjecucion::Finalizada,
                    TramoAccion {
                        emisiones,
                        cierre: CierreTramo::Completada(FinalizacionAccion {
                            duracion: expedicion.duracion,
                        }),
                    },
                ));
            }
        }
        let tipo = match movimiento {
            viajar::situaciones::MovimientoEtapa::Marchar if usa_camellos => {
                if expedicion.preparacion == PreparacionViaje::Cargada {
                    TipoMovimientoTerrestre::GuiarCaravana
                } else {
                    TipoMovimientoTerrestre::Cabalgar
                }
            }
            viajar::situaciones::MovimientoEtapa::Marchar => TipoMovimientoTerrestre::Marchar,
            viajar::situaciones::MovimientoEtapa::Descender => TipoMovimientoTerrestre::Descender,
            viajar::situaciones::MovimientoEtapa::Escalar => unreachable!(),
        };
        let resultado =
            resolver_movimiento_terrestre(mundo, &solicitud, tipo, contexto_movimiento, azar)
                .map_err(error_resolucion)?;
        let traza_movimiento = resultado.traza.clone();
        let mut operaciones = resultado.operaciones;
        if resultado.avanza {
            operaciones.push(OperacionMundo::Trasladar {
                actor: expedicion.dirigente,
                destino,
            });
        }
        let reservas_antes = expedicion.reservas;
        let duracion_supervivencia = atender_supervivencia(
            mundo,
            &mut expedicion,
            origen,
            resultado.duracion,
            resultado.consumo_agua_extra,
            resultado.consumo_alimento_extra,
            azar,
        )?;
        emisiones.extend(narrar_supervivencia(
            &mut expedicion,
            duracion_supervivencia.trazas,
            resultado.duracion,
        )?);
        let duracion_etapa = resultado
            .duracion
            .checked_add(duracion_supervivencia.duracion)
            .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        if expedicion.reservas.agua > reservas_antes.agua
            || expedicion.reservas.alimento > reservas_antes.alimento
        {
            emisiones.push(narrativa_con_progreso(
                TEXTO_RECURSOS,
                ClaveEscena::ViajeMarcha,
                &expedicion,
                duracion_etapa,
            )?);
        }
        let (carencias, hay_carencia) = consumir_reservas(
            mundo,
            &mut expedicion,
            duracion_etapa,
            resultado.consumo_agua_extra,
            resultado.consumo_alimento_extra,
        )?;
        if hay_carencia {
            emisiones.push(narrativa_con_progreso(
                TEXTO_CARENCIA,
                ClaveEscena::ViajeObstaculo,
                &expedicion,
                duracion_etapa,
            )?);
        }
        operaciones.extend(carencias);
        if !operaciones.is_empty() {
            emisiones.extend(aplicar_operaciones(mundo, operaciones)?);
        }
        if !resultado.avanza {
            expedicion
                .acumular_sin_avance(duracion_etapa)
                .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
            emisiones.push(narrativa_con_traza(
                "El esfuerzo no abre paso. El grupo reconoce que no puede confirmar la siguiente etapa y detiene el viaje.",
                ClaveEscena::ViajeObstaculo,
                &mut expedicion,
                traza_movimiento,
            )?);
            return Ok((
                EstadoEjecucion::Finalizada,
                TramoAccion {
                    emisiones,
                    cierre: CierreTramo::Completada(FinalizacionAccion {
                        duracion: expedicion.duracion,
                    }),
                },
            ));
        }
        expedicion
            .confirmar_avance(duracion_etapa)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        emisiones.push(narrativa_con_traza(
            match tipo {
                TipoMovimientoTerrestre::Marchar => {
                    "El grupo sostiene la marcha y alcanza el siguiente lugar de la ruta."
                }
                TipoMovimientoTerrestre::Descender => {
                    "El grupo asegura el descenso y confirma terreno firme al final de la etapa."
                }
                TipoMovimientoTerrestre::Cabalgar => {
                    "Los camellos aportados al plan permiten cubrir la etapa con mayor soltura."
                }
                TipoMovimientoTerrestre::GuiarCaravana => {
                    "El grupo conduce los camellos y la carga hasta el siguiente lugar confirmado."
                }
                TipoMovimientoTerrestre::Orientarse => unreachable!(),
            },
            ClaveEscena::ViajeMarcha,
            &mut expedicion,
            traza_movimiento,
        )?);
        emisiones.push(registro_proyecto(
            "El grupo reconoce la etapa como cumplida: el terreno recién alcanzado pasa a ser su nueva posición confirmada y el proyecto puede continuar desde allí.",
            "La etapa queda atrás",
            RolPresentacion::DesenlaceSituacion,
            RegistroProyecto::Interpretacion,
            &expedicion,
        )?);
        if tipo == TipoMovimientoTerrestre::Descender {
            emisiones.push(narrativa(
                TEXTO_DESCENSO,
                ClaveEscena::ViajeObstaculo,
                &expedicion,
            )?);
        } else if matches!(
            resultado.banda,
            BandaResolucion::Costosa | BandaResolucion::Fallida
        ) {
            emisiones.push(narrativa(
                TEXTO_ETAPA_COSTOSA,
                ClaveEscena::ViajeMarcha,
                &expedicion,
            )?);
        }
        if !expedicion.llego() {
            recalcular_ruta(mundo, &mut expedicion)?;
        }
    }

    if expedicion.jornadas_completas_restantes() >= 2 {
        emisiones.extend(aplicar_operaciones(
            mundo,
            vec![OperacionMundo::AnadirCaracteristicaActor {
                actor: expedicion.dirigente,
                caracteristica: caracteristica_regla("bienAprovisionado"),
            }],
        )?);
    }
    emisiones.push(narrativa(
        TEXTO_LLEGADA,
        ClaveEscena::ViajeLlegada,
        &expedicion,
    )?);
    Ok((
        EstadoEjecucion::Finalizada,
        TramoAccion {
            emisiones,
            cierre: CierreTramo::Completada(FinalizacionAccion {
                duracion: expedicion.duracion,
            }),
        },
    ))
}

fn resolver_etapa_acuatica(
    mundo: &mut Mundo,
    mut expedicion: viajar::expedicion::Expedicion,
    origen: IdLugar,
    destino: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<
    (
        EstadoEjecucion,
        TramoAccion,
        Option<viajar::expedicion::Expedicion>,
    ),
    ErrorMotor,
> {
    let origen_agua = es_lugar_acuatico(mundo, origen);
    let destino_agua = es_lugar_acuatico(mundo, destino);
    if expedicion.medio_acuatico == MedioAcuatico::Ninguno {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    let maniobra = if origen_agua && !destino_agua {
        ManiobraNautica::Atracar
    } else if expedicion.medio_acuatico == MedioAcuatico::Bote {
        ManiobraNautica::Remar
    } else {
        ManiobraNautica::Navegar
    };
    let responsable = expedicion
        .viajeros
        .iter()
        .copied()
        .find(|&id| {
            ["navegante", "remero", "timonel", "marino"]
                .into_iter()
                .any(|rasgo| actor_tiene(mundo, id, rasgo))
        })
        .unwrap_or(expedicion.dirigente);
    let maniobra_previa = if expedicion.estado_embarcacion != EstadoEmbarcacion::Estable {
        Some(ManiobraNautica::Achicar)
    } else if etapa_tiene(mundo, origen, destino, "rocoso")
        || etapa_tiene(mundo, origen, destino, "estrecho")
        || etapa_tiene(mundo, origen, destino, "corrienteRapida")
    {
        Some(ManiobraNautica::GobernarTimon)
    } else {
        None
    };
    let mut operaciones_previas = Vec::new();
    let mut trazas_previas = Vec::new();
    if let Some(maniobra_previa) = maniobra_previa {
        let previa = resolver_navegacion(
            mundo,
            maniobra_previa,
            expedicion.medio_acuatico,
            expedicion.estado_embarcacion,
            responsable,
            &expedicion.viajeros,
            origen,
            destino,
            azar,
        )
        .map_err(error_resolucion)?;
        expedicion.fijar_estado_embarcacion(previa.estado);
        trazas_previas.push(previa.traza.clone());
        operaciones_previas.extend(previa.operaciones);
    }
    let resultado = resolver_navegacion(
        mundo,
        maniobra,
        expedicion.medio_acuatico,
        expedicion.estado_embarcacion,
        responsable,
        &expedicion.viajeros,
        origen,
        destino,
        azar,
    )
    .map_err(error_resolucion)?;
    expedicion.fijar_estado_embarcacion(resultado.estado);
    let mut operaciones = operaciones_previas;
    let mut trazas_posteriores = Vec::new();
    operaciones.extend(resultado.operaciones);
    if !resultado.avanza {
        let fondeo = resolver_navegacion(
            mundo,
            ManiobraNautica::Fondear,
            expedicion.medio_acuatico,
            resultado.estado,
            responsable,
            &expedicion.viajeros,
            origen,
            destino,
            azar,
        )
        .map_err(error_resolucion)?;
        expedicion.fijar_estado_embarcacion(fondeo.estado);
        trazas_posteriores.push(fondeo.traza.clone());
        operaciones.extend(fondeo.operaciones);
    }
    if resultado.avanza {
        operaciones.push(OperacionMundo::Trasladar {
            actor: expedicion.dirigente,
            destino,
        });
    }
    let reservas_antes = expedicion.reservas;
    let supervivencia = atender_supervivencia(
        mundo,
        &mut expedicion,
        origen,
        resultado.duracion,
        0,
        0,
        azar,
    )?;
    let duracion = resultado
        .duracion
        .checked_add(supervivencia.duracion)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    let (carencias, hay_carencia) = consumir_reservas(mundo, &mut expedicion, duracion, 0, 0)?;
    operaciones.extend(carencias);
    let mut emisiones = trazas_previas
        .into_iter()
        .map(|traza| {
            narrativa_con_traza(
                "Antes de avanzar, la tripulación maniobra para recuperar control y abrir una derrota segura.",
                ClaveEscena::ViajeObstaculo,
                &mut expedicion,
                traza,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    emisiones.push(narrativa_con_traza(
        if maniobra == ManiobraNautica::Atracar {
            TEXTO_ATRAQUE
        } else {
            TEXTO_NAVEGACION
        },
        ClaveEscena::ViajeMarcha,
        &mut expedicion,
        resultado.traza.clone(),
    )?);
    for traza in trazas_posteriores {
        emisiones.push(narrativa_con_traza(
            "La maniobra no logra sostener el avance; la tripulación fondea para impedir que el peligro crezca.",
            ClaveEscena::ViajeObstaculo,
            &mut expedicion,
            traza,
        )?);
    }
    emisiones.extend(narrar_supervivencia(
        &mut expedicion,
        supervivencia.trazas,
        resultado.duracion,
    )?);
    if expedicion.reservas.agua > reservas_antes.agua
        || expedicion.reservas.alimento > reservas_antes.alimento
    {
        emisiones.push(narrativa_con_progreso(
            TEXTO_RECURSOS,
            ClaveEscena::ViajeMarcha,
            &expedicion,
            duracion,
        )?);
    }
    if hay_carencia {
        emisiones.push(narrativa_con_progreso(
            TEXTO_CARENCIA,
            ClaveEscena::ViajeObstaculo,
            &expedicion,
            duracion,
        )?);
    }
    if !operaciones.is_empty() {
        emisiones.extend(aplicar_operaciones(mundo, operaciones)?);
    }
    if resultado.avanza {
        expedicion
            .confirmar_avance(duracion)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        actualizar_progreso(&mut emisiones, &expedicion)?;
        emisiones.push(registro_proyecto(
            "La maniobra deja atrás la etapa acuática. La tripulación reconoce el lugar alcanzado como nueva posición segura para continuar el proyecto.",
            "Derrota confirmada",
            RolPresentacion::DesenlaceSituacion,
            RegistroProyecto::Interpretacion,
            &expedicion,
        )?);
        if expedicion.estado_embarcacion == EstadoEmbarcacion::NaufragioInminente {
            return esperar_crisis_nautica(
                expedicion,
                if destino_agua { destino } else { origen },
                emisiones,
            );
        }
        if !destino_agua
            && !expedicion
                .viajeros
                .iter()
                .copied()
                .any(|id| actor_tiene(mundo, id, "conBarco") || actor_tiene(mundo, id, "conBote"))
        {
            expedicion.fijar_medio_acuatico(MedioAcuatico::Ninguno);
        }
        Ok((
            EstadoEjecucion::Finalizada,
            TramoAccion {
                emisiones,
                cierre: CierreTramo::Completada(FinalizacionAccion {
                    duracion: expedicion.duracion,
                }),
            },
            Some(expedicion),
        ))
    } else {
        expedicion
            .acumular_sin_avance(duracion)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
        actualizar_progreso(&mut emisiones, &expedicion)?;
        if expedicion.estado_embarcacion == EstadoEmbarcacion::NaufragioInminente {
            return esperar_crisis_nautica(expedicion, origen, emisiones);
        }
        emisiones.push(narrativa(
            TEXTO_ETAPA_INTERRUMPIDA,
            ClaveEscena::ViajeObstaculo,
            &expedicion,
        )?);
        Ok((
            EstadoEjecucion::Finalizada,
            TramoAccion {
                emisiones,
                cierre: CierreTramo::Completada(FinalizacionAccion {
                    duracion: expedicion.duracion,
                }),
            },
            None,
        ))
    }
}

fn crear_proyecto(
    mundo: &Mundo,
    expedicion: &viajar::expedicion::Expedicion,
) -> Result<ProyectoViaje, ErrorMotor> {
    let (dias, exposicion, incertidumbre) =
        viajar::planificacion::coste_ruta(mundo, &expedicion.viajeros, &expedicion.ruta)
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let entrada_acuatica = expedicion.ruta.windows(2).find_map(|etapa| {
        (!es_lugar_acuatico(mundo, etapa[0]) && es_lugar_acuatico(mundo, etapa[1]))
            .then_some(etapa[0])
    });
    let requiere_mar = expedicion
        .ruta
        .iter()
        .copied()
        .any(|lugar| etapa_tiene(mundo, lugar, lugar, "marAbierto"));
    let usa_agua = expedicion
        .ruta
        .iter()
        .copied()
        .any(|lugar| es_lugar_acuatico(mundo, lugar));
    let mut medios_acuaticos = Vec::new();
    if !usa_agua {
        medios_acuaticos.push((MedioAcuaticoProyecto::Ninguno, None));
    } else {
        if expedicion
            .actual()
            .is_some_and(|lugar| es_lugar_acuatico(mundo, lugar))
        {
            let origen = expedicion
                .actual()
                .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
            medios_acuaticos.push((
                if requiere_mar {
                    MedioAcuaticoProyecto::Barco
                } else {
                    MedioAcuaticoProyecto::Bote
                },
                Some(FuenteMedio::EmbarcacionActual(origen)),
            ));
        }
        if !requiere_mar {
            if let Some(lugar) = entrada_acuatica.filter(|&id| etapa_tiene(mundo, id, id, "muelle"))
            {
                medios_acuaticos
                    .push((MedioAcuaticoProyecto::Bote, Some(FuenteMedio::Lugar(lugar))));
            }
            for actor in expedicion.viajeros.iter().copied() {
                if actor_tiene(mundo, actor, "conBote") {
                    medios_acuaticos
                        .push((MedioAcuaticoProyecto::Bote, Some(FuenteMedio::Actor(actor))));
                }
            }
        }
        if let Some(lugar) = entrada_acuatica.filter(|&id| etapa_tiene(mundo, id, id, "puerto")) {
            medios_acuaticos.push((
                MedioAcuaticoProyecto::Barco,
                Some(FuenteMedio::Lugar(lugar)),
            ));
        }
        for actor in expedicion.viajeros.iter().copied() {
            if actor_tiene(mundo, actor, "conBarco") {
                medios_acuaticos.push((
                    MedioAcuaticoProyecto::Barco,
                    Some(FuenteMedio::Actor(actor)),
                ));
            }
        }
    }
    let mut medios_terrestres = vec![(
        MedioTerrestreProyecto::APie,
        FuenteMedio::CapacidadHumana,
        expedicion.dirigente,
    )];
    if let Some(aportante) = expedicion
        .viajeros
        .iter()
        .copied()
        .find(|actor| actor_tiene(mundo, *actor, "conCamellos"))
    {
        medios_terrestres.push((
            MedioTerrestreProyecto::Camellos,
            FuenteMedio::Actor(aportante),
            aportante,
        ));
    }
    let mut propuestas = Vec::new();
    for (medio_terrestre, fuente_terrestre, proponente) in medios_terrestres {
        for &(medio_acuatico, fuente_acuatica) in &medios_acuaticos {
            let referencia = u32::try_from(propuestas.len())
                .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
            propuestas.push(PropuestaViaje {
                referencia,
                proponente,
                estrategia: EstrategiaViaje::Directa,
                medio_terrestre: medio_terrestre.clone(),
                fuente_terrestre,
                medio_acuatico,
                fuente_acuatica,
                ruta: expedicion.ruta.clone(),
                dias_previstos: dias,
                exposicion_conocida: exposicion,
                hay_incertidumbre: incertidumbre > 0,
            });
        }
    }
    ProyectoViaje::nuevo(
        expedicion.dirigente,
        expedicion.viajeros.clone(),
        expedicion
            .actual()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?,
        expedicion.destino_final,
        expedicion.fecha_inicio,
        propuestas,
    )
    .ok_or_else(|| {
        ErrorMotor::Rechazo(RechazoAccion::SinPropuestaViajeViable {
            origen: expedicion.actual().unwrap_or(expedicion.destino_final),
            destino: expedicion.destino_final,
        })
    })
}

fn etiqueta_propuesta(propuesta: &PropuestaViaje) -> String {
    let terrestre = match propuesta.medio_terrestre {
        MedioTerrestreProyecto::APie => "A pie",
        MedioTerrestreProyecto::Camellos => "Con camellos",
    };
    match propuesta.medio_acuatico {
        MedioAcuaticoProyecto::Ninguno => terrestre.to_owned(),
        MedioAcuaticoProyecto::Bote => format!("{terrestre} y en bote"),
        MedioAcuaticoProyecto::Barco => format!("{terrestre} y en barco"),
    }
}

fn detalle_propuesta(propuesta: &PropuestaViaje) -> String {
    let terrestre = match propuesta.fuente_terrestre {
        FuenteMedio::CapacidadHumana => "marcha por capacidad humana ordinaria".to_owned(),
        FuenteMedio::Actor(actor) => format!("medio terrestre aportado por {actor:?}"),
        FuenteMedio::Lugar(lugar) => format!("medio terrestre disponible en {lugar:?}"),
        FuenteMedio::EmbarcacionActual(lugar) => {
            format!("medio conservado desde la posición actual en {lugar:?}")
        }
    };
    let acuatico = match (propuesta.medio_acuatico, propuesta.fuente_acuatica) {
        (MedioAcuaticoProyecto::Ninguno, _) => None,
        (MedioAcuaticoProyecto::Bote, Some(FuenteMedio::Actor(actor))) => {
            Some(format!("bote aportado por {actor:?}"))
        }
        (MedioAcuaticoProyecto::Barco, Some(FuenteMedio::Actor(actor))) => {
            Some(format!("barco aportado por {actor:?}"))
        }
        (MedioAcuaticoProyecto::Bote, Some(FuenteMedio::Lugar(lugar))) => {
            Some(format!("servicio de bote en {lugar:?}"))
        }
        (MedioAcuaticoProyecto::Barco, Some(FuenteMedio::Lugar(lugar))) => {
            Some(format!("servicio de barco en {lugar:?}"))
        }
        (medio, Some(FuenteMedio::EmbarcacionActual(lugar))) => Some(format!(
            "{} en el que el grupo ya se encuentra en {lugar:?}",
            match medio {
                MedioAcuaticoProyecto::Bote => "bote",
                MedioAcuaticoProyecto::Barco => "barco",
                MedioAcuaticoProyecto::Ninguno => "medio acuático",
            }
        )),
        (_, Some(FuenteMedio::CapacidadHumana) | None) => None,
    };
    let incertidumbre = if propuesta.hay_incertidumbre {
        "con tramos todavía inciertos"
    } else {
        "sin incertidumbre geográfica conocida"
    };
    match acuatico {
        Some(acuatico) => format!(
            "{terrestre}; {acuatico}; {} días previstos; {incertidumbre}",
            propuesta.dias_previstos
        ),
        None => format!(
            "{terrestre}; {} días previstos; {incertidumbre}",
            propuesta.dias_previstos
        ),
    }
}

fn esperar_crisis_nautica(
    expedicion: viajar::expedicion::Expedicion,
    lugar_nautico: IdLugar,
    mut emisiones: Vec<EmisionAccion>,
) -> Result<
    (
        EstadoEjecucion,
        TramoAccion,
        Option<viajar::expedicion::Expedicion>,
    ),
    ErrorMotor,
> {
    let decision = IdDecision(u32::MAX - 4);
    emisiones.push(narrativa(
        TEXTO_CRISIS_NAUTICA,
        ClaveEscena::ViajeObstaculo,
        &expedicion,
    )?);
    let solicitud = solicitud_decision(
        decision,
        TEXTO_CRISIS_NAUTICA,
        ClaveEscena::ViajeObstaculo,
        &["Achicar el agua", "Fondear y detener el viaje"],
        &expedicion,
    )?;
    Ok((
        EstadoEjecucion::Viajar(EstadoViajar::CrisisNautica(EsperaNavegar {
            expedicion,
            decision,
            lugar_nautico,
        })),
        TramoAccion {
            emisiones,
            cierre: CierreTramo::EsperandoDecision(solicitud),
        },
        None,
    ))
}

fn es_lugar_acuatico(mundo: &Mundo, id: IdLugar) -> bool {
    etapa_tiene(mundo, id, id, "acuatico")
}

fn recalcular_ruta(
    mundo: &Mundo,
    expedicion: &mut viajar::expedicion::Expedicion,
) -> Result<(), ErrorMotor> {
    if expedicion.llego() {
        return Ok(());
    }
    let actual = expedicion
        .actual()
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let ruta = viajar::planificacion::ruta_ponderada(
        mundo,
        &expedicion.viajeros,
        actual,
        expedicion.destino_final,
    )
    .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    expedicion
        .reemplazar_ruta_desde_actual(ruta)
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))
}

fn requiere_orientarse(mundo: &Mundo, origen: IdLugar, destino: IdLugar) -> bool {
    [origen, destino].into_iter().any(|lugar| {
        mundo.lugar(lugar).is_some_and(|lugar| {
            ["desertico", "boscoso", "pantanoso"]
                .into_iter()
                .any(|rasgo| lugar.tiene_caracteristica(&caracteristica_regla(rasgo)))
        })
    })
}

fn etapa_tiene(mundo: &Mundo, origen: IdLugar, destino: IdLugar, rasgo: &str) -> bool {
    [origen, destino].into_iter().any(|id| {
        mundo
            .lugar(id)
            .is_some_and(|lugar| lugar.tiene_caracteristica(&caracteristica_regla(rasgo)))
    })
}

fn actor_tiene(mundo: &Mundo, actor: IdActor, rasgo: &str) -> bool {
    mundo
        .actor(actor)
        .is_some_and(|actor| actor.tiene_caracteristica(&caracteristica_regla(rasgo)))
}

fn atender_supervivencia(
    mundo: &mut Mundo,
    expedicion: &mut viajar::expedicion::Expedicion,
    lugar: IdLugar,
    duracion_movimiento: Duracion,
    agua_extra: u32,
    alimento_extra: u32,
    azar: &mut dyn FuenteAzar,
) -> Result<ResultadoSupervivencia, ErrorMotor> {
    if duracion_movimiento.meses() != 0 {
        return Err(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion));
    }
    let cantidad = u32::try_from(expedicion.viajeros.len())
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
    let agua_necesaria = duracion_movimiento
        .dias()
        .checked_mul(cantidad)
        .and_then(|valor| valor.checked_add(agua_extra))
        .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
    let alimento_necesario = duracion_movimiento
        .dias()
        .checked_mul(cantidad)
        .and_then(|valor| valor.checked_add(alimento_extra))
        .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
    let viajeros = expedicion.viajeros.clone();
    let responsable = viajeros
        .iter()
        .copied()
        .find(|&id| {
            [
                "superviviente",
                "cazador",
                "rastreador",
                "pescador",
                "recolector",
            ]
            .into_iter()
            .any(|rasgo| actor_tiene(mundo, id, rasgo))
        })
        .unwrap_or(expedicion.dirigente);
    let mut duracion = Duracion::CERO;
    let mut trazas = Vec::new();

    if expedicion.reservas.agua < agua_necesaria
        && expedicion
            .registrar_intento_supervivencia(lugar, AccionRecurso::BuscarAgua.clave_intento())
    {
        let resultado = resolver_recurso(
            mundo,
            AccionRecurso::BuscarAgua,
            responsable,
            &viajeros,
            lugar,
            azar,
        )
        .map_err(error_resolucion)?;
        trazas.push(resultado.traza.clone());
        let agua = match resultado.calidad_agua {
            Some(CalidadAgua::Potable) => resultado.agua,
            Some(calidad) if resultado.agua > 0 => {
                let tratamiento =
                    tratar_agua(calidad, resultado.agua, true, azar).map_err(error_resolucion)?;
                trazas.push(tratamiento.traza.clone());
                tratamiento.cantidad_potable
            }
            _ => 0,
        };
        expedicion
            .agregar_agua(agua)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
        duracion = duracion
            .checked_add(resultado.duracion)
            .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    }

    for accion in [
        AccionRecurso::Cazar,
        AccionRecurso::Pescar,
        AccionRecurso::Recolectar,
    ] {
        if expedicion.reservas.alimento >= alimento_necesario {
            break;
        }
        if !expedicion.registrar_intento_supervivencia(lugar, accion.clave_intento()) {
            continue;
        }
        let resultado = resolver_recurso(mundo, accion, responsable, &viajeros, lugar, azar)
            .map_err(error_resolucion)?;
        trazas.push(resultado.traza.clone());
        expedicion
            .agregar_alimento(resultado.alimento)
            .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
        duracion = duracion
            .checked_add(resultado.duracion)
            .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    }

    let necesita_descanso = viajeros
        .iter()
        .copied()
        .any(|id| actor_tiene(mundo, id, "fatigado") || actor_tiene(mundo, id, "agotado"));
    let campista = viajeros.iter().copied().find(|&id| {
        actor_tiene(mundo, id, "superviviente") || actor_tiene(mundo, id, "equipoCampamento")
    });
    if necesita_descanso && let Some(campista) = campista {
        let campamento =
            montar_campamento(mundo, campista, lugar, azar).map_err(error_resolucion)?;
        trazas.push(campamento.traza.clone());
        let descanso = descansar(
            mundo,
            &viajeros,
            campamento.calidad,
            expedicion.reservas.agua >= agua_necesaria
                && expedicion.reservas.alimento >= alimento_necesario,
            ["abierto", "viento", "pantanoso", "inundado"]
                .into_iter()
                .any(|rasgo| etapa_tiene(mundo, lugar, lugar, rasgo)),
            azar,
        )
        .map_err(error_resolucion)?;
        trazas.push(descanso.traza.clone());
        if !descanso.operaciones.is_empty() {
            let _ = aplicar_operaciones(mundo, descanso.operaciones)?;
        }
    }
    Ok(ResultadoSupervivencia { duracion, trazas })
}

struct ResultadoSupervivencia {
    duracion: Duracion,
    trazas: Vec<TrazaAccionParticular>,
}

fn narrar_supervivencia(
    expedicion: &mut viajar::expedicion::Expedicion,
    trazas: Vec<TrazaAccionParticular>,
    duracion_previa: Duracion,
) -> Result<Vec<EmisionAccion>, ErrorMotor> {
    let mut transcurrida = expedicion
        .duracion
        .checked_add(duracion_previa)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    trazas
        .into_iter()
        .map(|traza| {
            transcurrida = transcurrida
                .checked_add(traza.duracion)
                .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
            let texto = match traza.accion {
                AccionParticular::BuscarAgua => {
                    "La necesidad de agua aparta al grupo de la marcha: los viajeros buscan una fuente que permita sostener el plan."
                }
                AccionParticular::TratarAgua => {
                    "El agua encontrada no es segura por sí sola; el grupo se esfuerza por volverla potable antes de continuar."
                }
                AccionParticular::Cazar => {
                    "Las reservas escasean y un viajero sale a cazar para que la expedición pueda seguir avanzando."
                }
                AccionParticular::Pescar => {
                    "El grupo aprovecha las aguas cercanas y pesca para recomponer sus provisiones."
                }
                AccionParticular::Recolectar => {
                    "Los viajeros registran el terreno y reúnen alimento disponible para sostener la jornada."
                }
                AccionParticular::MontarCampamento => {
                    "El cansancio obliga a buscar resguardo y preparar un campamento antes de exigir otro esfuerzo al grupo."
                }
                AccionParticular::Descansar => {
                    "Con el campamento dispuesto, cada integrante intenta recuperar fuerzas para conservar unido al grupo."
                }
                _ => "El grupo atiende una necesidad concreta antes de retomar el viaje.",
            };
            let mut emision =
                narrativa_con_traza(texto, ClaveEscena::ViajeObstaculo, expedicion, traza)?;
            let EmisionAccion::Narrativa(fragmento) = &mut emision else {
                unreachable!()
            };
            fragmento.progreso.duracion_transcurrida = transcurrida;
            fragmento.progreso.fecha_proyectada = expedicion
                .fecha_inicio
                .sumar(transcurrida)
                .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
            Ok(emision)
        })
        .collect()
}

fn consumir_reservas(
    mundo: &Mundo,
    expedicion: &mut viajar::expedicion::Expedicion,
    duracion: Duracion,
    agua_extra: u32,
    alimento_extra: u32,
) -> Result<(Vec<OperacionMundo>, bool), ErrorMotor> {
    if duracion.meses() != 0 {
        return Err(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion));
    }
    let (falta_agua, falta_alimento) = expedicion
        .consumir(duracion.dias(), agua_extra, alimento_extra)
        .ok_or(ErrorMotor::Fatal(FalloMotor::DesbordamientoResolucion))?;
    let mut operaciones = Vec::new();
    for &actor in &expedicion.viajeros {
        if falta_agua > 0 {
            operaciones.extend(progresion_carencia(mundo, actor, "sediento"));
            operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                actor,
                caracteristica: caracteristica_regla("sediento"),
            });
        } else if actor_tiene(mundo, actor, "sediento") {
            operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                actor,
                caracteristica: caracteristica_regla("sediento"),
            });
        }
        if falta_alimento > 0 {
            operaciones.extend(progresion_carencia(mundo, actor, "hambriento"));
            operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                actor,
                caracteristica: caracteristica_regla("hambriento"),
            });
        } else if actor_tiene(mundo, actor, "hambriento") {
            operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                actor,
                caracteristica: caracteristica_regla("hambriento"),
            });
        }
    }
    Ok((operaciones, falta_agua > 0 || falta_alimento > 0))
}

fn progresion_carencia(mundo: &Mundo, actor: IdActor, causa: &str) -> Vec<OperacionMundo> {
    if !actor_tiene(mundo, actor, causa) {
        return vec![];
    }
    let (quitar, anadir) = if actor_tiene(mundo, actor, "agotado") {
        (Some("agotado"), Some("herido"))
    } else if actor_tiene(mundo, actor, "fatigado") {
        (Some("fatigado"), Some("agotado"))
    } else if actor_tiene(mundo, actor, "herido") {
        (None, None)
    } else {
        (None, Some("fatigado"))
    };
    let mut operaciones = Vec::new();
    if let Some(rasgo) = quitar {
        operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
            actor,
            caracteristica: caracteristica_regla(rasgo),
        });
    }
    if let Some(rasgo) = anadir {
        operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
            actor,
            caracteristica: caracteristica_regla(rasgo),
        });
    }
    operaciones
}

fn validar_viaje(
    mundo: &Mundo,
    actor: IdActor,
    destino: IdLugar,
) -> Result<Vec<IdLugar>, RechazoAccion> {
    let actor_actual = mundo
        .actor(actor)
        .ok_or(RechazoAccion::ActorDesconocido { actor })?;
    mundo
        .lugar(destino)
        .ok_or(RechazoAccion::DestinoDesconocido { destino })?;
    validar_jugador_vivo(mundo, actor, actor_actual)?;
    if actor_actual.lugar() == destino {
        return Err(RechazoAccion::DestinoEsLugarActual { lugar: destino });
    }
    if actor != mundo.actor_jugador() && mundo.grupo().contiene(actor) {
        return Err(RechazoAccion::ViajeIndependienteDeIntegranteGrupo { actor });
    }
    let viajeros = viajeros_para_consulta(mundo, actor);
    let ruta =
        viajar::planificacion::ruta_ponderada(mundo, &viajeros, actor_actual.lugar(), destino)
            .ok_or_else(|| {
                if viajar::planificacion::hay_ruta_topologica(mundo, actor_actual.lugar(), destino)
                {
                    RechazoAccion::SinPropuestaViajeViable {
                        origen: actor_actual.lugar(),
                        destino,
                    }
                } else {
                    RechazoAccion::DestinoSinRutaConectada {
                        origen: actor_actual.lugar(),
                        destino,
                    }
                }
            })?;
    if !ruta_admite_propuesta(mundo, &viajeros, &ruta) {
        return Err(RechazoAccion::SinPropuestaViajeViable {
            origen: actor_actual.lugar(),
            destino,
        });
    }
    Ok(ruta)
}

fn ruta_admite_propuesta(mundo: &Mundo, viajeros: &[IdActor], ruta: &[IdLugar]) -> bool {
    let usa_agua = ruta
        .iter()
        .copied()
        .any(|lugar| es_lugar_acuatico(mundo, lugar));
    if !usa_agua {
        return true;
    }
    let requiere_barco = ruta
        .iter()
        .copied()
        .any(|lugar| etapa_tiene(mundo, lugar, lugar, "marAbierto"));
    let entrada = ruta.windows(2).find_map(|etapa| {
        (!es_lugar_acuatico(mundo, etapa[0]) && es_lugar_acuatico(mundo, etapa[1]))
            .then_some(etapa[0])
    });
    let origen_acuatico = ruta
        .first()
        .copied()
        .is_some_and(|lugar| es_lugar_acuatico(mundo, lugar));
    let actor_aporta = viajeros.iter().copied().any(|actor| {
        actor_tiene(mundo, actor, "conBarco")
            || (!requiere_barco && actor_tiene(mundo, actor, "conBote"))
    });
    let lugar_ofrece = entrada.is_some_and(|lugar| {
        etapa_tiene(mundo, lugar, lugar, "puerto")
            || (!requiere_barco && etapa_tiene(mundo, lugar, lugar, "muelle"))
    });
    origen_acuatico || actor_aporta || lugar_ofrece
}

fn validar_jugador_vivo(
    mundo: &Mundo,
    actor: IdActor,
    actor_actual: &crate::Actor,
) -> Result<(), RechazoAccion> {
    if actor == mundo.actor_jugador()
        && actor_actual.tiene_caracteristica(&caracteristica_regla("muerto"))
    {
        return Err(RechazoAccion::JugadorMuerto { actor });
    }
    Ok(())
}

fn viajeros_de(mundo: &Mundo, actor: IdActor) -> Result<Vec<IdActor>, ErrorMotor> {
    if actor == mundo.actor_jugador() {
        let mut viajeros = Vec::with_capacity(mundo.grupo().integrantes().len() + 1);
        viajeros.push(actor);
        viajeros.extend(mundo.grupo().integrantes().iter().copied());
        Ok(viajeros)
    } else if mundo.actor(actor).is_some() {
        Ok(vec![actor])
    } else {
        Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota))
    }
}

fn viajeros_para_consulta(mundo: &Mundo, actor: IdActor) -> Vec<IdActor> {
    if actor == mundo.actor_jugador() {
        let mut viajeros = vec![actor];
        viajeros.extend_from_slice(mundo.grupo().integrantes());
        viajeros
    } else {
        vec![actor]
    }
}

fn contexto_viaje_valido(espera: &EsperaViajar) -> bool {
    espera.expedicion.ruta.last() == Some(&espera.expedicion.destino_final)
        && espera.expedicion.viajeros.first() == Some(&espera.expedicion.dirigente)
        && espera.expedicion.actual() == Some(espera.solicitud_escalar.origen)
        && espera.expedicion.siguiente() == Some(espera.solicitud_escalar.destino)
}

fn aplicar_operaciones(
    mundo: &mut Mundo,
    operaciones: Vec<OperacionMundo>,
) -> Result<Vec<EmisionAccion>, ErrorMotor> {
    let lote = LoteOperaciones::nuevo(operaciones)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let cambios = mundo
        .aplicar_lote(lote)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    Ok(cambios.en_vec().into_iter().map(resultado_cambio).collect())
}

fn resultado_cambio(cambio: CambioConfirmado) -> EmisionAccion {
    let resultado = match cambio {
        CambioConfirmado::ActoresTrasladados {
            actores,
            origen,
            destino,
        } => ResultadoSemantico::ActoresTrasladados {
            actores,
            origen,
            destino,
        },
        CambioConfirmado::CaracteristicaActorAnadida {
            actor,
            caracteristica,
        } => ResultadoSemantico::CaracteristicaActorAnadida {
            actor,
            caracteristica,
        },
        CambioConfirmado::CaracteristicaActorQuitada {
            actor,
            caracteristica,
        } => ResultadoSemantico::CaracteristicaActorQuitada {
            actor,
            caracteristica,
        },
    };
    EmisionAccion::ResultadoSemantico(resultado)
}

fn narrativa(
    texto: &str,
    escena: ClaveEscena,
    expedicion: &viajar::expedicion::Expedicion,
) -> Result<EmisionAccion, ErrorMotor> {
    if texto.is_empty() {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    Ok(EmisionAccion::Narrativa(FragmentoNarrativo {
        titulo: match escena {
            ClaveEscena::ViajeMarcha => "La jornada",
            ClaveEscena::ViajeObstaculo => "Una dificultad",
            ClaveEscena::ViajeRumboIncierto => "El rumbo",
            ClaveEscena::ViajeLlegada => "Llegada",
        }
        .to_owned(),
        texto: texto.to_owned(),
        escena: Some(escena),
        rol: if escena == ClaveEscena::ViajeLlegada {
            RolPresentacion::Conclusion
        } else {
            RolPresentacion::DesarrolloSituacion
        },
        proyecto: None,
        progreso: expedicion
            .progreso()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?,
        momentos: Vec::new(),
    }))
}

fn narrativa_con_progreso(
    texto: &str,
    escena: ClaveEscena,
    expedicion: &viajar::expedicion::Expedicion,
    incremento: Duracion,
) -> Result<EmisionAccion, ErrorMotor> {
    let mut emision = narrativa(texto, escena, expedicion)?;
    let EmisionAccion::Narrativa(fragmento) = &mut emision else {
        unreachable!()
    };
    let transcurrida = expedicion
        .duracion
        .checked_add(incremento)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    fragmento.progreso.duracion_transcurrida = transcurrida;
    fragmento.progreso.fecha_proyectada = expedicion
        .fecha_inicio
        .sumar(transcurrida)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))?;
    Ok(emision)
}

fn registro_proyecto(
    texto: &str,
    titulo: &str,
    rol: RolPresentacion,
    registro: RegistroProyecto,
    expedicion: &viajar::expedicion::Expedicion,
) -> Result<EmisionAccion, ErrorMotor> {
    let mut emision = narrativa(texto, ClaveEscena::ViajeMarcha, expedicion)?;
    let EmisionAccion::Narrativa(fragmento) = &mut emision else {
        unreachable!()
    };
    fragmento.titulo = titulo.to_owned();
    fragmento.rol = rol;
    fragmento.proyecto = Some(registro);
    fragmento.escena = None;
    Ok(emision)
}

fn narrativa_con_traza(
    texto: &str,
    escena: ClaveEscena,
    expedicion: &mut viajar::expedicion::Expedicion,
    traza: TrazaAccionParticular,
) -> Result<EmisionAccion, ErrorMotor> {
    let subobjetivo = match traza.accion {
        AccionParticular::BuscarAgua
        | AccionParticular::TratarAgua
        | AccionParticular::Cazar
        | AccionParticular::Pescar
        | AccionParticular::Recolectar
        | AccionParticular::MontarCampamento
        | AccionParticular::Descansar => expedicion.actual().map(SubobjetivoViaje::SostenerGrupo),
        AccionParticular::Fondear | AccionParticular::Achicar => expedicion
            .actual()
            .map(SubobjetivoViaje::EstabilizarEmbarcacion),
        _ => expedicion
            .proyecto
            .as_ref()
            .and_then(|proyecto| proyecto.subobjetivo),
    };
    let mut emision = narrativa(texto, escena, expedicion)?;
    let EmisionAccion::Narrativa(fragmento) = &mut emision else {
        unreachable!()
    };
    fragmento.momentos.push(MomentoCausal {
        id: expedicion
            .siguiente_momento()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?,
        subobjetivo,
        traza,
    });
    fragmento.rol = RolPresentacion::DesenlaceSituacion;
    Ok(emision)
}

fn actualizar_progreso(
    emisiones: &mut [EmisionAccion],
    expedicion: &viajar::expedicion::Expedicion,
) -> Result<(), ErrorMotor> {
    let progreso = expedicion
        .progreso()
        .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    for emision in emisiones {
        if let EmisionAccion::Narrativa(fragmento) = emision {
            fragmento.progreso = progreso;
        }
    }
    Ok(())
}

fn solicitud_decision<T: AsRef<str>>(
    id: IdDecision,
    texto: &str,
    escena: ClaveEscena,
    textos_opciones: &[T],
    expedicion: &viajar::expedicion::Expedicion,
) -> Result<SolicitudDecision, ErrorMotor> {
    if texto.is_empty()
        || textos_opciones
            .iter()
            .any(|texto| texto.as_ref().is_empty())
    {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    let mut opciones = Vec::with_capacity(textos_opciones.len());
    for (posicion, texto) in textos_opciones.iter().enumerate() {
        let indice =
            u32::try_from(posicion).map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        opciones.push(OpcionDecision {
            id: IdOpcionDecision(indice),
            texto: texto.as_ref().to_owned(),
            detalle: None,
            propuesta_viaje: None,
        });
    }
    Ok(SolicitudDecision {
        id,
        texto: texto.to_owned(),
        escena: Some(escena),
        rol: if escena == ClaveEscena::ViajeObstaculo {
            RolPresentacion::DesarrolloSituacion
        } else {
            RolPresentacion::Compromiso
        },
        proyecto: None,
        progreso: expedicion
            .progreso()
            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?,
        opciones,
    })
}

#[cfg(test)]
fn agregar_duracion(actual: Duracion, incremento: Duracion) -> Result<Duracion, ErrorMotor> {
    actual
        .checked_add(incremento)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))
}

fn error_resolucion(error: ErrorResolucion) -> ErrorMotor {
    let fallo = match error {
        ErrorResolucion::ContextoInvalido => FalloMotor::InvarianteRota,
        ErrorResolucion::FuenteAzar(error) => FalloMotor::FuenteAzar(error),
        ErrorResolucion::IndiceFueraDeRango {
            indice,
            limite_exclusivo,
        } => FalloMotor::IndiceAzarFueraDeRango {
            indice,
            limite_exclusivo,
        },
        ErrorResolucion::Desbordamiento => FalloMotor::DesbordamientoResolucion,
    };
    ErrorMotor::Fatal(fallo)
}

fn caracteristica_regla(texto: &str) -> Caracteristica {
    match Caracteristica::nueva(texto.to_owned()) {
        Ok(caracteristica) => caracteristica,
        Err(_) => unreachable!("las Características literales del motor son válidas"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorMotor {
    Rechazo(RechazoAccion),
    Fatal(FalloMotor),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RechazoAccion {
    ActorDesconocido {
        actor: IdActor,
    },
    DestinoDesconocido {
        destino: IdLugar,
    },
    JugadorMuerto {
        actor: IdActor,
    },
    DestinoEsLugarActual {
        lugar: IdLugar,
    },
    ViajeIndependienteDeIntegranteGrupo {
        actor: IdActor,
    },
    DestinoSinRutaConectada {
        origen: IdLugar,
        destino: IdLugar,
    },
    SinPropuestaViajeViable {
        origen: IdLugar,
        destino: IdLugar,
    },
    DecisionNoVigente {
        decision: IdDecision,
    },
    OpcionNoVigente {
        decision: IdDecision,
        opcion: IdOpcionDecision,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FalloMotor {
    FuenteAzar(ErrorFuenteAzar),
    IndiceAzarFueraDeRango {
        indice: usize,
        limite_exclusivo: NonZeroUsize,
    },
    DesbordamientoDuracion,
    DesbordamientoResolucion,
    InvarianteRota,
}

impl Display for ErrorMotor {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "error del motor: {self:?}")
    }
}

impl Error for ErrorMotor {}

impl Display for RechazoAccion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "acción rechazada: {self:?}")
    }
}

impl Error for RechazoAccion {}

impl Display for FalloMotor {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "fallo fatal del motor: {self:?}")
    }
}

impl Error for FalloMotor {}

#[cfg(test)]
mod pruebas {
    use std::collections::VecDeque;

    use super::*;
    use crate::{
        ActorInicial, DatosInicialesMundo, ExtremoRelacion, Fecha, IdMapa, LugarInicial, Mes,
        RelacionInicial, TipoRelacion, VariableGlobalInicial,
    };

    #[derive(Default)]
    struct AzarProgramado {
        valores: VecDeque<Result<usize, ErrorFuenteAzar>>,
        limites: Vec<usize>,
    }

    impl AzarProgramado {
        fn nuevo(valores: impl IntoIterator<Item = usize>) -> Self {
            Self {
                valores: valores.into_iter().map(Ok).collect(),
                limites: Vec::new(),
            }
        }

        fn con_error() -> Self {
            Self {
                valores: VecDeque::from([Err(ErrorFuenteAzar)]),
                limites: Vec::new(),
            }
        }
    }

    impl FuenteAzar for AzarProgramado {
        fn elegir_indice(
            &mut self,
            limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            self.limites.push(limite_exclusivo.get());
            self.valores.pop_front().unwrap_or(Err(ErrorFuenteAzar))
        }
    }

    fn id_actor(posicion: usize) -> IdActor {
        IdActor::desde_posicion(posicion).expect("ID de Actor válido")
    }

    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID de Lugar válido")
    }

    fn caracteristicas(valores: &[&str]) -> Vec<Caracteristica> {
        valores
            .iter()
            .map(|valor| Caracteristica::nueva((*valor).to_owned()).expect("etiqueta válida"))
            .collect()
    }

    fn mundo_con(jugador: &[&str], companero: &[&str], npc: &[&str], destino: &[&str]) -> Mundo {
        Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID de Mapa válido"),
            fecha: Fecha::nueva(2025, Mes::Enero, 31).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: id_lugar(0),
                    caracteristicas: vec![],
                },
                LugarInicial {
                    id: id_lugar(1),
                    caracteristicas: caracteristicas(&["empinado"]),
                },
                LugarInicial {
                    id: id_lugar(2),
                    caracteristicas: caracteristicas(destino),
                },
                LugarInicial {
                    id: id_lugar(3),
                    caracteristicas: vec![],
                },
                LugarInicial {
                    id: id_lugar(4),
                    caracteristicas: vec![],
                },
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor(0),
                    lugar: id_lugar(0),
                    caracteristicas: caracteristicas(jugador),
                },
                ActorInicial {
                    id: id_actor(1),
                    lugar: id_lugar(0),
                    caracteristicas: caracteristicas(companero),
                },
                ActorInicial {
                    id: id_actor(2),
                    lugar: id_lugar(2),
                    caracteristicas: caracteristicas(npc),
                },
            ],
            actor_jugador: id_actor(0),
            companeros: vec![id_actor(1)],
            variables_globales: Vec::<VariableGlobalInicial>::new(),
            relaciones: vec![
                RelacionInicial {
                    tipo: TipoRelacion::Conectado,
                    origen: ExtremoRelacion::Lugar(id_lugar(0)),
                    destino: ExtremoRelacion::Lugar(id_lugar(1)),
                },
                RelacionInicial {
                    tipo: TipoRelacion::Conectado,
                    origen: ExtremoRelacion::Lugar(id_lugar(1)),
                    destino: ExtremoRelacion::Lugar(id_lugar(2)),
                },
            ],
        })
        .expect("Mundo de prueba válido")
    }

    fn solicitud_jugador() -> SolicitudAccion {
        SolicitudAccion::Viajar {
            actor: id_actor(0),
            destino: id_lugar(2),
        }
    }

    fn iniciar_configurado(
        mundo: &mut Mundo,
        solicitud: SolicitudAccion,
        azar: &mut dyn FuenteAzar,
    ) -> InicioAccion {
        let inicio = iniciar(mundo, solicitud, azar).expect("inicio válido");
        let mut ejecucion = inicio.ejecucion;
        let mut tramo = inicio.tramo;
        let CierreTramo::EsperandoDecision(primera) = &tramo.cierre else {
            panic!("se esperaba configuración")
        };
        if primera.texto.contains("cuánto preparar") {
            tramo = responder(
                mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision: primera.id,
                    opcion: primera.opciones[0].id,
                },
                azar,
            )
            .expect("preparación válida");
        }
        let CierreTramo::EsperandoDecision(ritmo) = &tramo.cierre else {
            panic!("se esperaba ritmo")
        };
        let tramo = responder(
            mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision: ritmo.id,
                opcion: ritmo.opciones[1].id,
            },
            azar,
        )
        .expect("ritmo válido");
        InicioAccion { ejecucion, tramo }
    }

    fn ids_decision(tramo: &TramoAccion) -> (IdDecision, IdOpcionDecision, IdOpcionDecision) {
        let CierreTramo::EsperandoDecision(decision) = &tramo.cierre else {
            panic!("se esperaba una decisión")
        };
        (
            decision.id,
            decision.opciones[0].id,
            decision.opciones[1].id,
        )
    }

    fn duracion_final(tramo: &TramoAccion) -> Duracion {
        let CierreTramo::Completada(finalizacion) = tramo.cierre else {
            panic!("se esperaba una finalización")
        };
        finalizacion.duracion
    }

    fn textos_narrativos(tramo: &TramoAccion) -> Vec<(&str, Option<ClaveEscena>)> {
        tramo
            .emisiones
            .iter()
            .filter_map(|emision| match emision {
                EmisionAccion::Narrativa(fragmento) => {
                    Some((fragmento.texto.as_str(), fragmento.escena))
                }
                EmisionAccion::ResultadoSemantico(_) => None,
            })
            .collect()
    }

    #[test]
    fn enumera_destinos_sin_consumir_azar_ni_usar_el_mapa() {
        let mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        assert_eq!(
            destinos_disponibles(&mundo, id_actor(0)),
            Ok(vec![id_lugar(1), id_lugar(2)])
        );
        assert_eq!(
            destinos_disponibles(&mundo, id_actor(2)),
            Ok(vec![id_lugar(0), id_lugar(1)])
        );
        assert_eq!(
            destinos_disponibles(&mundo, id_actor(1)),
            Err(RechazoAccion::ViajeIndependienteDeIntegranteGrupo { actor: id_actor(1) })
        );
    }

    #[test]
    fn rechaza_solicitudes_en_orden_sin_consumir_azar_ni_mutar() {
        let casos = [
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(9),
                    destino: id_lugar(9),
                },
                ErrorMotor::Rechazo(RechazoAccion::ActorDesconocido { actor: id_actor(9) }),
            ),
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(0),
                    destino: id_lugar(9),
                },
                ErrorMotor::Rechazo(RechazoAccion::DestinoDesconocido {
                    destino: id_lugar(9),
                }),
            ),
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(0),
                    destino: id_lugar(0),
                },
                ErrorMotor::Rechazo(RechazoAccion::DestinoEsLugarActual { lugar: id_lugar(0) }),
            ),
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(1),
                    destino: id_lugar(2),
                },
                ErrorMotor::Rechazo(RechazoAccion::ViajeIndependienteDeIntegranteGrupo {
                    actor: id_actor(1),
                }),
            ),
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(0),
                    destino: id_lugar(4),
                },
                ErrorMotor::Rechazo(RechazoAccion::DestinoSinRutaConectada {
                    origen: id_lugar(0),
                    destino: id_lugar(4),
                }),
            ),
        ];

        for (solicitud, esperado) in casos {
            let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
            let ubicaciones = mundo
                .actores()
                .iter()
                .map(|actor| actor.lugar())
                .collect::<Vec<_>>();
            let mut azar = AzarProgramado::nuevo([0]);
            assert_eq!(
                iniciar(&mut mundo, solicitud, &mut azar).err(),
                Some(esperado)
            );
            assert!(azar.limites.is_empty());
            assert_eq!(
                mundo
                    .actores()
                    .iter()
                    .map(|actor| actor.lugar())
                    .collect::<Vec<_>>(),
                ubicaciones
            );
        }

        let mut mundo = mundo_con(&["muerto"], &["fuerte"], &[], &[]);
        let mut azar = AzarProgramado::nuevo([0]);
        assert!(matches!(
            iniciar(&mut mundo, solicitud_jugador(), &mut azar),
            Err(ErrorMotor::Rechazo(RechazoAccion::JugadorMuerto { .. }))
        ));
        assert!(azar.limites.is_empty());
    }

    #[test]
    fn distingue_conexion_geografica_sin_medio_viable() {
        let mut mundo = mundo_con(&[], &[], &[], &["acuatico"]);
        let ubicacion = mundo.ubicacion_actor(id_actor(0));
        let mut azar = AzarProgramado::nuevo([19]);
        assert_eq!(
            iniciar(&mut mundo, solicitud_jugador(), &mut azar).err(),
            Some(ErrorMotor::Rechazo(
                RechazoAccion::SinPropuestaViajeViable {
                    origen: id_lugar(0),
                    destino: id_lugar(2),
                }
            ))
        );
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), ubicacion);
        assert!(azar.limites.is_empty());
    }

    #[test]
    fn deliberacion_conserva_propuestas_y_procedencias_semanticas() {
        let mut mundo = mundo_con(&[], &["conCamellos"], &[], &[]);
        let mut azar = AzarProgramado::nuevo([19]);
        let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje viable");
        let CierreTramo::EsperandoDecision(decision) = inicio.tramo.cierre else {
            panic!("se esperaba deliberación")
        };
        assert_eq!(decision.opciones.len(), 2);
        let propuestas = decision
            .opciones
            .iter()
            .map(|opcion| opcion.propuesta_viaje.as_ref().expect("carga semántica"))
            .collect::<Vec<_>>();
        assert_eq!(propuestas[0].fuente_terrestre, FuenteMedio::CapacidadHumana);
        assert_eq!(
            propuestas[1].fuente_terrestre,
            FuenteMedio::Actor(id_actor(1))
        );
        assert!(
            decision
                .opciones
                .iter()
                .all(|opcion| opcion.detalle.is_some())
        );
    }

    #[test]
    fn cronologia_asigna_momentos_unicos_y_progreso_no_decreciente() {
        let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        let mut azar = AzarProgramado::nuevo(std::iter::repeat_n(19, 64));
        let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("inicio válido");
        let mut ejecucion = inicio.ejecucion;
        let mut tramo = inicio.tramo;
        let mut ids = Vec::new();
        let mut progresos = Vec::new();
        loop {
            for emision in &tramo.emisiones {
                if let EmisionAccion::Narrativa(fragmento) = emision {
                    progresos.push(fragmento.progreso.duracion_transcurrida);
                    ids.extend(fragmento.momentos.iter().map(|momento| momento.id));
                }
            }
            let CierreTramo::EsperandoDecision(decision) = &tramo.cierre else {
                break;
            };
            let opcion = if decision.texto == TEXTO_DECISION_PASO {
                decision.opciones[1].id
            } else {
                decision.opciones[0].id
            };
            tramo = responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision: decision.id,
                    opcion,
                },
                &mut azar,
            )
            .expect("continuación válida");
        }
        assert!(!ids.is_empty());
        let unicos = ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(unicos.len(), ids.len());
        assert!(
            progresos
                .windows(2)
                .all(|par| { (par[0].meses(), par[0].dias()) <= (par[1].meses(), par[1].dias()) }),
            "progreso fuera de orden: {progresos:?}"
        );
    }

    #[test]
    fn confirma_cada_etapa_y_conserva_el_mundo_sin_avanzar_fecha() {
        let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        let fecha = mundo.fecha();
        let mut azar = AzarProgramado::default();
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, _, rodear) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: rodear,
            },
            &mut azar,
        )
        .expect("rodeo válido");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(3));
        assert_eq!(mundo.fecha(), fecha);
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(2)));
        assert_eq!(mundo.ubicacion_actor(id_actor(1)), Some(id_lugar(2)));
        assert_eq!(
            tramo
                .emisiones
                .iter()
                .filter(|emision| matches!(
                    emision,
                    EmisionAccion::ResultadoSemantico(
                        ResultadoSemantico::ActoresTrasladados { .. }
                    )
                ))
                .count(),
            2
        );
    }

    #[test]
    fn obstaculo_conserva_espera_y_escalar_usa_la_ayuda_del_grupo() {
        let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        let mut azar = AzarProgramado::nuevo([14]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        assert_eq!(
            textos_narrativos(&inicio.tramo).last().copied(),
            Some((TEXTO_PASO_EMPINADO, Some(ClaveEscena::ViajeObstaculo)))
        );
        let CierreTramo::EsperandoDecision(decision) = &inicio.tramo.cierre else {
            panic!("se esperaba decisión")
        };
        assert_eq!(decision.texto, TEXTO_DECISION_PASO);
        assert_eq!(decision.escena, Some(ClaveEscena::ViajeObstaculo));
        assert_eq!(
            decision
                .opciones
                .iter()
                .map(|opcion| opcion.texto.as_str())
                .collect::<Vec<_>>(),
            vec!["Ascender directamente", "Buscar un paso menos empinado"]
        );

        let (id, abrir, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let error = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision: IdDecision(99),
                opcion: abrir,
            },
            &mut azar,
        );
        assert!(matches!(
            error,
            Err(ErrorMotor::Rechazo(RechazoAccion::DecisionNoVigente { .. }))
        ));
        assert!(azar.limites.is_empty());
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));

        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision: id,
                opcion: abrir,
            },
            &mut azar,
        )
        .expect("respuesta válida");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
        let textos = textos_narrativos(&tramo);
        assert_eq!(
            textos.first().map(|(texto, _)| *texto),
            Some(TEXTO_ASCENSO_HOLGADO)
        );
        assert_eq!(textos.last().map(|(texto, _)| *texto), Some(TEXTO_LLEGADA));
        assert_eq!(azar.limites, vec![20]);
    }

    #[test]
    fn obstaculo_cubre_ascenso_incierto_y_rodeo_determinista() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::nuevo([11]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: ascender,
            },
            &mut azar,
        )
        .expect("ascenso válido");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
        assert_eq!(textos_narrativos(&tramo)[0].0, TEXTO_ASCENSO_LOGRADO);
        assert_eq!(azar.limites, vec![20]);

        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::default();
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, _, rodear) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: rodear,
            },
            &mut azar,
        )
        .expect("rodeo válido");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(3));
        assert_eq!(textos_narrativos(&tramo)[0].0, TEXTO_PASO_MENOS_EMPINADO);
        assert!(azar.limites.is_empty());
    }

    #[test]
    fn escalar_costoso_confirma_desgaste_y_traslado_en_el_mismo_lote() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::nuevo([7, 19]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: ascender,
            },
            &mut azar,
        )
        .expect("ascenso costoso válido");

        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(2)));
        assert!(
            mundo
                .actor(id_actor(0))
                .expect("Actor existente")
                .tiene_caracteristica(&caracteristica_regla("agotado"))
        );
        assert!(tramo.emisiones.iter().any(|emision| matches!(
            emision,
            EmisionAccion::ResultadoSemantico(
                ResultadoSemantico::CaracteristicaActorAnadida { actor, .. }
            ) if *actor == id_actor(0)
        )));
        assert_eq!(azar.limites, vec![20, 20]);
    }

    #[test]
    fn escalar_fallido_hiere_e_interrumpe_sin_trasladar() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::nuevo([0]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: ascender,
            },
            &mut azar,
        )
        .expect("fallo reglado válido");

        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(1));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
        assert_eq!(mundo.ubicacion_actor(id_actor(1)), Some(id_lugar(0)));
        assert!(
            mundo
                .actor(id_actor(0))
                .expect("Actor existente")
                .tiene_caracteristica(&caracteristica_regla("herido"))
        );
        assert_eq!(
            textos_narrativos(&tramo),
            vec![(TEXTO_ASCENSO_FALLIDO, Some(ClaveEscena::ViajeObstaculo))]
        );
        assert!(!tramo.emisiones.iter().any(|emision| matches!(
            emision,
            EmisionAccion::ResultadoSemantico(ResultadoSemantico::ActoresTrasladados { .. })
        )));
        assert_eq!(azar.limites, vec![20]);
    }

    #[test]
    fn npc_resuelve_la_misma_escalada_y_se_traslada_solo() {
        for (npc_fuerte, cara) in [(false, 11), (true, 9)] {
            let rasgos_npc: &[&str] = if npc_fuerte { &["fuerte"] } else { &[] };
            let mut mundo = mundo_con(&["fuerte"], &["fuerte"], rasgos_npc, &[]);
            let mut azar = AzarProgramado::nuevo([cara]);
            let inicio = iniciar_configurado(
                &mut mundo,
                SolicitudAccion::Viajar {
                    actor: id_actor(2),
                    destino: id_lugar(0),
                },
                &mut azar,
            );
            let (decision, abrir, _) = ids_decision(&inicio.tramo);
            let mut ejecucion = inicio.ejecucion;
            let tramo = responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision,
                    opcion: abrir,
                },
                &mut azar,
            )
            .expect("respuesta válida");
            assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
            assert_eq!(mundo.ubicacion_actor(id_actor(2)), Some(id_lugar(0)));
            assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
            assert!(tramo.emisiones.iter().any(|emision| matches!(
                emision,
                EmisionAccion::ResultadoSemantico(
                    ResultadoSemantico::ActoresTrasladados { actores, .. }
                ) if actores == &vec![id_actor(2)]
            )));
            assert_eq!(azar.limites, vec![20]);
        }
    }

    #[test]
    fn opcion_no_vigente_no_consume_azar_y_la_espera_sigue_utilizable() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::nuevo([11]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut azar);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        assert!(matches!(
            responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision,
                    opcion: IdOpcionDecision(9),
                },
                &mut azar,
            ),
            Err(ErrorMotor::Rechazo(RechazoAccion::OpcionNoVigente { .. }))
        ));
        assert!(azar.limites.is_empty());
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: ascender,
            },
            &mut azar,
        )
        .expect("la espera continúa vigente");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
        assert_eq!(azar.limites, vec![20]);
    }

    #[test]
    fn convierte_fallos_de_azar_e_indices_fuera_de_rango() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut error = AzarProgramado::con_error();
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut error);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        assert!(matches!(
            responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision,
                    opcion: ascender
                },
                &mut error
            ),
            Err(ErrorMotor::Fatal(FalloMotor::FuenteAzar(ErrorFuenteAzar)))
        ));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));

        let mut fuera = AzarProgramado::nuevo([20]);
        let inicio = iniciar_configurado(&mut mundo, solicitud_jugador(), &mut fuera);
        let (decision, ascender, _) = ids_decision(&inicio.tramo);
        let mut ejecucion = inicio.ejecucion;
        assert!(matches!(
            responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision {
                    decision,
                    opcion: ascender
                },
                &mut fuera
            ),
            Err(ErrorMotor::Fatal(FalloMotor::IndiceAzarFueraDeRango {
                indice: 20,
                ..
            }))
        ));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
    }

    #[test]
    fn convierte_desbordamiento_de_duracion_en_fallo_fatal() {
        assert_eq!(
            agregar_duracion(Duracion::desde_dias(u32::MAX), Duracion::desde_dias(1)),
            Err(ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))
        );
    }
}
