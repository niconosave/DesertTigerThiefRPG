use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;

use crate::azar::{ErrorFuenteAzar, FuenteAzar};
use crate::{Caracteristica, Duracion, IdActor, IdLugar, Mundo, TrasladoConfirmado};

const TEXTO_MARCHA: &str =
    "El viaje avanza sin contratiempos. La marcha mantiene un ritmo constante.";
const TEXTO_LLEGADA_TRANQUILA: &str =
    "Tras una marcha tranquila, el destino aparece al final del camino.";
const TEXTO_OBSTACULO: &str =
    "Un derrumbe bloquea el paso. Los restos impiden continuar por el trayecto más directo.";
const TEXTO_DECISION_OBSTACULO: &str =
    "El derrumbe cierra el paso directo. ¿Cómo debe continuar el viaje?";
const TEXTO_ABRIR_FUERTE: &str = "El esfuerzo abre un paso entre los restos. El viaje continúa sin una gran demora hasta su destino.";
const TEXTO_ABRIR_SIN_FUERZA: &str = "Abrir un paso entre los restos exige mucho más tiempo de lo previsto, pero el viaje finalmente alcanza su destino.";
const TEXTO_RODEAR: &str = "Un rodeo evita el derrumbe a costa de una marcha más larga. El viaje finalmente alcanza su destino.";
const TEXTO_RUMBO_INCIERTO: &str =
    "Las señales del camino se vuelven confusas. No resulta claro cuál es el rumbo correcto.";
const TEXTO_DECISION_RUMBO: &str = "El rumbo es incierto. ¿Cómo debe continuar el viaje?";
const TEXTO_INTUICION_ACERTADA: &str = "La intuición resulta acertada y permite retomar el rumbo sin una gran demora. El viaje finalmente alcanza su destino.";
const TEXTO_INTUICION_DEMORA: &str = "La intuición conduce por un desvío equivocado. Corregir el rumbo demora la marcha, pero el viaje finalmente alcanza su destino.";
const TEXTO_REVISAR_RUMBO: &str = "Una comprobación cuidadosa permite confirmar el rumbo. La marcha continúa por un trayecto seguro hasta su destino.";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolicitudAccion {
    Viajar { actor: IdActor, destino: IdLugar },
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
    pub opciones: Vec<OpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpcionDecision {
    pub id: IdOpcionDecision,
    pub texto: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaveEscena {
    ViajeMarcha,
    ViajeObstaculo,
    ViajeRumboIncierto,
    ViajeLlegada,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentoNarrativo {
    pub texto: String,
    pub escena: Option<ClaveEscena>,
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
    EsperandoObstaculo(EsperaViajar),
    EsperandoRumbo(EsperaViajar),
}

struct EsperaViajar {
    actor: IdActor,
    destino: IdLugar,
    duracion: Duracion,
    decision: IdDecision,
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

    let inalcanzable = caracteristica_regla("inalcanzable");
    Ok(mundo
        .lugares()
        .iter()
        .filter(|lugar| {
            lugar.id() != actor_actual.lugar() && !lugar.tiene_caracteristica(&inalcanzable)
        })
        .map(|lugar| lugar.id())
        .collect())
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
    let espera = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::EsperandoObstaculo(espera))
        | EstadoEjecucion::Viajar(EstadoViajar::EsperandoRumbo(espera)) => espera,
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

    let (texto, incremento) = match &ejecucion.estado {
        EstadoEjecucion::Viajar(EstadoViajar::EsperandoObstaculo(espera)) => {
            resolver_obstaculo(mundo, espera.actor, respuesta.opcion)?
        }
        EstadoEjecucion::Viajar(EstadoViajar::EsperandoRumbo(_)) => {
            resolver_rumbo(respuesta.opcion, azar)?
        }
        EstadoEjecucion::Finalizada => unreachable!("la ejecución se validó al comenzar"),
    };

    let (actor, destino, duracion) = match ejecucion_viajar(&ejecucion.estado) {
        Some(espera) => (
            espera.actor,
            espera.destino,
            agregar_duracion(espera.duracion, incremento)?,
        ),
        None => return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
    };

    let emisiones = finalizar_viaje(mundo, actor, destino, duracion, texto)?;
    ejecucion.estado = EstadoEjecucion::Finalizada;
    Ok(TramoAccion {
        emisiones,
        cierre: CierreTramo::Completada(FinalizacionAccion { duracion }),
    })
}

fn iniciar_viaje(
    mundo: &mut Mundo,
    actor: IdActor,
    destino: IdLugar,
    azar: &mut dyn FuenteAzar,
) -> Result<InicioAccion, ErrorMotor> {
    validar_viaje(mundo, actor, destino).map_err(ErrorMotor::Rechazo)?;
    let rama = elegir(azar, 3)?;

    match rama {
        0 => {
            let duracion = agregar_duracion(Duracion::CERO, Duracion::desde_dias(1))?;
            let duracion = agregar_duracion(duracion, Duracion::desde_dias(1))?;
            let mut emisiones = vec![narrativa(TEXTO_MARCHA, ClaveEscena::ViajeMarcha)?];
            emisiones.extend(finalizar_viaje(
                mundo,
                actor,
                destino,
                duracion,
                TEXTO_LLEGADA_TRANQUILA,
            )?);
            Ok(InicioAccion {
                ejecucion: EjecucionAccion {
                    estado: EstadoEjecucion::Finalizada,
                },
                tramo: TramoAccion {
                    emisiones,
                    cierre: CierreTramo::Completada(FinalizacionAccion { duracion }),
                },
            })
        }
        1 => iniciar_espera(
            actor,
            destino,
            TEXTO_OBSTACULO,
            ClaveEscena::ViajeObstaculo,
            TEXTO_DECISION_OBSTACULO,
            ["Abrirse paso", "Rodear el obstáculo"],
            EstadoViajar::EsperandoObstaculo,
        ),
        2 => iniciar_espera(
            actor,
            destino,
            TEXTO_RUMBO_INCIERTO,
            ClaveEscena::ViajeRumboIncierto,
            TEXTO_DECISION_RUMBO,
            ["Confiar en la intuición", "Revisar el rumbo"],
            EstadoViajar::EsperandoRumbo,
        ),
        _ => Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
    }
}

fn iniciar_espera(
    actor: IdActor,
    destino: IdLugar,
    texto_fragmento: &str,
    escena: ClaveEscena,
    texto_decision: &str,
    opciones: [&str; 2],
    construir_estado: fn(EsperaViajar) -> EstadoViajar,
) -> Result<InicioAccion, ErrorMotor> {
    let duracion = agregar_duracion(Duracion::CERO, Duracion::desde_dias(1))?;
    let decision = IdDecision(0);
    let solicitud = solicitud_decision(decision, texto_decision, escena, opciones)?;
    Ok(InicioAccion {
        ejecucion: EjecucionAccion {
            estado: EstadoEjecucion::Viajar(construir_estado(EsperaViajar {
                actor,
                destino,
                duracion,
                decision,
            })),
        },
        tramo: TramoAccion {
            emisiones: vec![narrativa(texto_fragmento, escena)?],
            cierre: CierreTramo::EsperandoDecision(solicitud),
        },
    })
}

fn validar_viaje(mundo: &Mundo, actor: IdActor, destino: IdLugar) -> Result<(), RechazoAccion> {
    let actor_actual = mundo
        .actor(actor)
        .ok_or(RechazoAccion::ActorDesconocido { actor })?;
    let lugar_destino = mundo
        .lugar(destino)
        .ok_or(RechazoAccion::DestinoDesconocido { destino })?;
    validar_jugador_vivo(mundo, actor, actor_actual)?;
    if actor_actual.lugar() == destino {
        return Err(RechazoAccion::DestinoEsLugarActual { lugar: destino });
    }
    if lugar_destino.tiene_caracteristica(&caracteristica_regla("inalcanzable")) {
        return Err(RechazoAccion::DestinoInalcanzable { destino });
    }
    if actor != mundo.actor_jugador() && mundo.grupo().contiene(actor) {
        return Err(RechazoAccion::ViajeIndependienteDeIntegranteGrupo { actor });
    }
    Ok(())
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

fn resolver_obstaculo(
    mundo: &Mundo,
    actor: IdActor,
    opcion: IdOpcionDecision,
) -> Result<(&'static str, Duracion), ErrorMotor> {
    match opcion.0 {
        0 => {
            let fuerte = caracteristica_regla("fuerte");
            let tiene_fuerza = if actor == mundo.actor_jugador() {
                std::iter::once(actor)
                    .chain(mundo.grupo().integrantes().iter().copied())
                    .try_fold(false, |encontrada, viajero| {
                        mundo
                            .actor(viajero)
                            .map(|actual| encontrada || actual.tiene_caracteristica(&fuerte))
                            .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))
                    })?
            } else {
                mundo
                    .actor(actor)
                    .map(|actual| actual.tiene_caracteristica(&fuerte))
                    .ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?
            };
            if tiene_fuerza {
                Ok((TEXTO_ABRIR_FUERTE, Duracion::desde_dias(1)))
            } else {
                Ok((TEXTO_ABRIR_SIN_FUERZA, Duracion::desde_dias(3)))
            }
        }
        1 => Ok((TEXTO_RODEAR, Duracion::desde_dias(2))),
        _ => Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
    }
}

fn resolver_rumbo(
    opcion: IdOpcionDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<(&'static str, Duracion), ErrorMotor> {
    match opcion.0 {
        0 => match elegir(azar, 2)? {
            0 => Ok((TEXTO_INTUICION_ACERTADA, Duracion::desde_dias(1))),
            1 => Ok((TEXTO_INTUICION_DEMORA, Duracion::desde_dias(3))),
            _ => Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
        },
        1 => Ok((TEXTO_REVISAR_RUMBO, Duracion::desde_dias(2))),
        _ => Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota)),
    }
}

fn finalizar_viaje(
    mundo: &mut Mundo,
    actor: IdActor,
    destino: IdLugar,
    _duracion: Duracion,
    texto: &str,
) -> Result<Vec<EmisionAccion>, ErrorMotor> {
    let fragmento = narrativa(texto, ClaveEscena::ViajeLlegada)?;
    let traslado = mundo
        .trasladar(actor, destino)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    Ok(vec![fragmento, resultado_traslado(traslado)])
}

fn resultado_traslado(traslado: TrasladoConfirmado) -> EmisionAccion {
    EmisionAccion::ResultadoSemantico(ResultadoSemantico::ActoresTrasladados {
        actores: traslado.actores,
        origen: traslado.origen,
        destino: traslado.destino,
    })
}

fn narrativa(texto: &str, escena: ClaveEscena) -> Result<EmisionAccion, ErrorMotor> {
    if texto.is_empty() {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    Ok(EmisionAccion::Narrativa(FragmentoNarrativo {
        texto: texto.to_owned(),
        escena: Some(escena),
    }))
}

fn solicitud_decision(
    id: IdDecision,
    texto: &str,
    escena: ClaveEscena,
    textos_opciones: [&str; 2],
) -> Result<SolicitudDecision, ErrorMotor> {
    if texto.is_empty() || textos_opciones.iter().any(|texto| texto.is_empty()) {
        return Err(ErrorMotor::Fatal(FalloMotor::InvarianteRota));
    }
    let mut opciones = Vec::with_capacity(textos_opciones.len());
    for (posicion, texto) in textos_opciones.into_iter().enumerate() {
        let indice =
            u32::try_from(posicion).map_err(|_| ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
        opciones.push(OpcionDecision {
            id: IdOpcionDecision(indice),
            texto: texto.to_owned(),
        });
    }
    Ok(SolicitudDecision {
        id,
        texto: texto.to_owned(),
        escena: Some(escena),
        opciones,
    })
}

fn elegir(azar: &mut dyn FuenteAzar, limite: usize) -> Result<usize, ErrorMotor> {
    let limite_exclusivo =
        NonZeroUsize::new(limite).ok_or(ErrorMotor::Fatal(FalloMotor::InvarianteRota))?;
    let indice = azar
        .elegir_indice(limite_exclusivo)
        .map_err(|error| ErrorMotor::Fatal(FalloMotor::FuenteAzar(error)))?;
    if indice >= limite_exclusivo.get() {
        return Err(ErrorMotor::Fatal(FalloMotor::IndiceAzarFueraDeRango {
            indice,
            limite_exclusivo,
        }));
    }
    Ok(indice)
}

fn agregar_duracion(actual: Duracion, incremento: Duracion) -> Result<Duracion, ErrorMotor> {
    actual
        .checked_add(incremento)
        .map_err(|_| ErrorMotor::Fatal(FalloMotor::DesbordamientoDuracion))
}

fn ejecucion_viajar(estado: &EstadoEjecucion) -> Option<&EsperaViajar> {
    match estado {
        EstadoEjecucion::Viajar(EstadoViajar::EsperandoObstaculo(espera))
        | EstadoEjecucion::Viajar(EstadoViajar::EsperandoRumbo(espera)) => Some(espera),
        EstadoEjecucion::Finalizada => None,
    }
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
    DestinoInalcanzable {
        destino: IdLugar,
    },
    ViajeIndependienteDeIntegranteGrupo {
        actor: IdActor,
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
        ActorInicial, DatosInicialesMundo, Fecha, IdMapa, LugarInicial, Mes, VariableGlobalInicial,
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
                    caracteristicas: caracteristicas(destino),
                },
                LugarInicial {
                    id: id_lugar(2),
                    caracteristicas: caracteristicas(&["inalcanzable"]),
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
                    lugar: id_lugar(1),
                    caracteristicas: caracteristicas(npc),
                },
            ],
            actor_jugador: id_actor(0),
            companeros: vec![id_actor(1)],
            variables_globales: Vec::<VariableGlobalInicial>::new(),
        })
        .expect("Mundo de prueba válido")
    }

    fn solicitud_jugador() -> SolicitudAccion {
        SolicitudAccion::Viajar {
            actor: id_actor(0),
            destino: id_lugar(1),
        }
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
            Ok(vec![id_lugar(1)])
        );
        assert_eq!(
            destinos_disponibles(&mundo, id_actor(2)),
            Ok(vec![id_lugar(0)])
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
                    actor: id_actor(0),
                    destino: id_lugar(2),
                },
                ErrorMotor::Rechazo(RechazoAccion::DestinoInalcanzable {
                    destino: id_lugar(2),
                }),
            ),
            (
                SolicitudAccion::Viajar {
                    actor: id_actor(1),
                    destino: id_lugar(1),
                },
                ErrorMotor::Rechazo(RechazoAccion::ViajeIndependienteDeIntegranteGrupo {
                    actor: id_actor(1),
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
    fn completa_trayecto_tranquilo_con_textos_duracion_y_traslado_exactos() {
        let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        let fecha = mundo.fecha();
        let mut azar = AzarProgramado::nuevo([0]);
        let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje válido");

        assert_eq!(azar.limites, vec![3]);
        assert_eq!(
            textos_narrativos(&inicio.tramo),
            vec![
                (TEXTO_MARCHA, Some(ClaveEscena::ViajeMarcha)),
                (TEXTO_LLEGADA_TRANQUILA, Some(ClaveEscena::ViajeLlegada)),
            ]
        );
        assert_eq!(duracion_final(&inicio.tramo), Duracion::desde_dias(2));
        assert_eq!(mundo.fecha(), fecha);
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(1)));
        assert_eq!(mundo.ubicacion_actor(id_actor(1)), Some(id_lugar(1)));
        assert_eq!(
            inicio.tramo.emisiones.last(),
            Some(&EmisionAccion::ResultadoSemantico(
                ResultadoSemantico::ActoresTrasladados {
                    actores: vec![id_actor(0), id_actor(1)],
                    origen: id_lugar(0),
                    destino: id_lugar(1),
                }
            ))
        );
    }

    #[test]
    fn obstaculo_conserva_espera_ante_respuesta_invalida_y_usa_fuerza_del_grupo() {
        let mut mundo = mundo_con(&[], &["fuerte"], &[], &[]);
        let mut azar = AzarProgramado::nuevo([1]);
        let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje válido");
        assert_eq!(
            textos_narrativos(&inicio.tramo),
            vec![(TEXTO_OBSTACULO, Some(ClaveEscena::ViajeObstaculo))]
        );
        let CierreTramo::EsperandoDecision(decision) = &inicio.tramo.cierre else {
            panic!("se esperaba decisión")
        };
        assert_eq!(decision.texto, TEXTO_DECISION_OBSTACULO);
        assert_eq!(decision.escena, Some(ClaveEscena::ViajeObstaculo));
        assert_eq!(
            decision
                .opciones
                .iter()
                .map(|opcion| opcion.texto.as_str())
                .collect::<Vec<_>>(),
            vec!["Abrirse paso", "Rodear el obstáculo"]
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
        assert_eq!(azar.limites, vec![3]);
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
        assert_eq!(
            textos_narrativos(&tramo),
            vec![(TEXTO_ABRIR_FUERTE, Some(ClaveEscena::ViajeLlegada))]
        );
        assert_eq!(azar.limites, vec![3]);
    }

    #[test]
    fn obstaculo_cubre_abrir_sin_fuerza_y_rodear() {
        for (indice_opcion, dias, texto) in [(0, 4, TEXTO_ABRIR_SIN_FUERZA), (1, 3, TEXTO_RODEAR)] {
            let mut mundo = mundo_con(&[], &[], &[], &[]);
            let mut azar = AzarProgramado::nuevo([1]);
            let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje válido");
            let (decision, primera, segunda) = ids_decision(&inicio.tramo);
            let opcion = if indice_opcion == 0 { primera } else { segunda };
            let mut ejecucion = inicio.ejecucion;
            let tramo = responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision { decision, opcion },
                &mut azar,
            )
            .expect("respuesta válida");
            assert_eq!(duracion_final(&tramo), Duracion::desde_dias(dias));
            assert_eq!(textos_narrativos(&tramo)[0].0, texto);
            assert_eq!(azar.limites, vec![3]);
        }
    }

    #[test]
    fn npc_solo_consulta_su_fuerza_y_se_traslada_solo() {
        for (npc_fuerte, dias) in [(false, 4), (true, 2)] {
            let rasgos_npc: &[&str] = if npc_fuerte { &["fuerte"] } else { &[] };
            let mut mundo = mundo_con(&["fuerte"], &["fuerte"], rasgos_npc, &[]);
            let mut azar = AzarProgramado::nuevo([1]);
            let inicio = iniciar(
                &mut mundo,
                SolicitudAccion::Viajar {
                    actor: id_actor(2),
                    destino: id_lugar(0),
                },
                &mut azar,
            )
            .expect("viaje NPC válido");
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
            assert_eq!(duracion_final(&tramo), Duracion::desde_dias(dias));
            assert_eq!(mundo.ubicacion_actor(id_actor(2)), Some(id_lugar(0)));
            assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
            assert!(matches!(
                tramo.emisiones.last(),
                Some(EmisionAccion::ResultadoSemantico(
                    ResultadoSemantico::ActoresTrasladados { actores, .. }
                )) if actores == &vec![id_actor(2)]
            ));
        }
    }

    #[test]
    fn rumbo_incierto_cubre_intuicion_y_revision_con_orden_de_azar() {
        for (valores, opcion_indice, dias, texto, limites) in [
            (vec![2, 0], 0, 2, TEXTO_INTUICION_ACERTADA, vec![3, 2]),
            (vec![2, 1], 0, 4, TEXTO_INTUICION_DEMORA, vec![3, 2]),
            (vec![2], 1, 3, TEXTO_REVISAR_RUMBO, vec![3]),
        ] {
            let mut mundo = mundo_con(&[], &[], &[], &[]);
            let mut azar = AzarProgramado::nuevo(valores);
            let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje válido");
            assert_eq!(
                textos_narrativos(&inicio.tramo),
                vec![(TEXTO_RUMBO_INCIERTO, Some(ClaveEscena::ViajeRumboIncierto))]
            );
            let CierreTramo::EsperandoDecision(solicitud) = &inicio.tramo.cierre else {
                panic!("se esperaba decisión")
            };
            assert_eq!(solicitud.texto, TEXTO_DECISION_RUMBO);
            assert_eq!(
                solicitud
                    .opciones
                    .iter()
                    .map(|opcion| opcion.texto.as_str())
                    .collect::<Vec<_>>(),
                vec!["Confiar en la intuición", "Revisar el rumbo"]
            );
            let (decision, primera, segunda) = ids_decision(&inicio.tramo);
            let opcion = if opcion_indice == 0 { primera } else { segunda };
            let mut ejecucion = inicio.ejecucion;
            let tramo = responder(
                &mut mundo,
                &mut ejecucion,
                RespuestaDecision { decision, opcion },
                &mut azar,
            )
            .expect("respuesta válida");
            assert_eq!(duracion_final(&tramo), Duracion::desde_dias(dias));
            assert_eq!(textos_narrativos(&tramo)[0].0, texto);
            assert_eq!(azar.limites, limites);
        }
    }

    #[test]
    fn opcion_no_vigente_no_consume_azar_y_la_espera_sigue_utilizable() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut azar = AzarProgramado::nuevo([2, 0]);
        let inicio = iniciar(&mut mundo, solicitud_jugador(), &mut azar).expect("viaje válido");
        let (decision, intuicion, _) = ids_decision(&inicio.tramo);
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
        assert_eq!(azar.limites, vec![3]);
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
        let tramo = responder(
            &mut mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision,
                opcion: intuicion,
            },
            &mut azar,
        )
        .expect("la espera continúa vigente");
        assert_eq!(duracion_final(&tramo), Duracion::desde_dias(2));
    }

    #[test]
    fn convierte_fallos_de_azar_e_indices_fuera_de_rango() {
        let mut mundo = mundo_con(&[], &[], &[], &[]);
        let mut error = AzarProgramado::con_error();
        assert!(matches!(
            iniciar(&mut mundo, solicitud_jugador(), &mut error),
            Err(ErrorMotor::Fatal(FalloMotor::FuenteAzar(ErrorFuenteAzar)))
        ));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));

        let mut fuera = AzarProgramado::nuevo([3]);
        assert!(matches!(
            iniciar(&mut mundo, solicitud_jugador(), &mut fuera),
            Err(ErrorMotor::Fatal(FalloMotor::IndiceAzarFueraDeRango {
                indice: 3,
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
