use std::num::NonZeroUsize;

use desert_tiger_core::acciones::{
    CierreTramo, EmisionAccion, ErrorMotor, FalloMotor, FragmentoNarrativo, RechazoAccion,
    RespuestaDecision, ResultadoSemantico, SolicitudAccion, iniciar, responder,
};
use desert_tiger_core::azar::{ErrorFuenteAzar, FuenteAzar};
use desert_tiger_core::{Duracion, IdActor, IdLugar, Mundo};

use crate::contenido::CatalogoClaves;
use crate::lua::{
    ClaseErrorLua, ErrorPoliticaLua, EtapaLua, PoliticaLuaValidada, ResultadoPoliticaLua,
    evaluar_politica,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumenNpc {
    entradas: Vec<EntradaResumenNpc>,
    cronologias: Vec<CronologiaNpc>,
}

impl ResumenNpc {
    pub fn entradas(&self) -> &[EntradaResumenNpc] {
        &self.entradas
    }

    pub fn cronologias(&self) -> &[CronologiaNpc] {
        &self.cronologias
    }

    fn uno(entrada: EntradaResumenNpc) -> Self {
        Self {
            entradas: vec![entrada],
            cronologias: Vec::new(),
        }
    }

    fn con_cronologia(entrada: EntradaResumenNpc, cronologia: CronologiaNpc) -> Self {
        Self {
            entradas: vec![entrada],
            cronologias: vec![cronologia],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CronologiaNpc {
    pub actor: IdActor,
    pub registros: Vec<FragmentoNarrativo>,
    pub decisiones_automaticas: usize,
    pub duracion: Duracion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntradaResumenNpc {
    ViajeCompletado {
        actor: IdActor,
        origen: IdLugar,
        destino: IdLugar,
    },
    ViajeInterrumpido {
        actor: IdActor,
        lugar: IdLugar,
    },
    SolicitudViajeRechazada {
        actor: IdActor,
        destino: IdLugar,
        causa: RechazoAccion,
    },
    ErrorPolitica {
        diagnostico: DiagnosticoPoliticaNpc,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtapaFalloPoliticaNpc {
    Preparacion,
    Ejecucion,
    ConversionRespuesta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoFalloPoliticaNpc {
    LimiteInstrucciones,
    LimiteMemoria,
    ArgumentoInvalido,
    ClaveVariableGlobalInvalida,
    VariableGlobalInexistente,
    TipoVariableGlobalIncorrecto,
    CaracteristicaInvalida,
    ClaveLugarInvalida,
    LugarInexistente,
    RespuestaInvalida,
    Ejecucion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticoPoliticaNpc {
    pub actor: IdActor,
    pub clave_actor: String,
    pub archivo_relativo: String,
    pub etapa: EtapaFalloPoliticaNpc,
    pub codigo: CodigoFalloPoliticaNpc,
    pub linea: Option<u32>,
    pub detalle: String,
}

#[derive(Debug)]
pub(crate) enum FalloFaseNpc {
    Integracion(ErrorPoliticaLua),
    FuenteAzar(ErrorFuenteAzar),
    IndiceAzarFueraDeRango {
        indice: usize,
        limite_exclusivo: NonZeroUsize,
    },
    Motor(FalloMotor),
    RespuestaAutomaticaRechazada(RechazoAccion),
    InvarianteRota,
}

pub(crate) fn ejecutar_fase_npc(
    mundo: &mut Mundo,
    politicas: &[PoliticaLuaValidada],
    claves: &CatalogoClaves,
    azar: &mut dyn FuenteAzar,
) -> Result<Option<ResumenNpc>, FalloFaseNpc> {
    let elegibles: Vec<_> = politicas
        .iter()
        .filter(|politica| {
            politica.actor() != mundo.actor_jugador() && !mundo.grupo().contiene(politica.actor())
        })
        .collect();
    let [politica] = elegibles.as_slice() else {
        return Err(FalloFaseNpc::InvarianteRota);
    };
    let fecha_antes = mundo.fecha();
    let solicitud = match evaluar_politica(politica, mundo, claves) {
        Ok(ResultadoPoliticaLua::Ninguna) => return Ok(None),
        Ok(ResultadoPoliticaLua::Solicitud(solicitud)) => solicitud,
        Err(error) if es_recuperable(&error) => {
            return Ok(Some(ResumenNpc::uno(EntradaResumenNpc::ErrorPolitica {
                diagnostico: proyectar_error(error),
            })));
        }
        Err(error) => return Err(FalloFaseNpc::Integracion(error)),
    };
    let (actor_solicitud, destino_solicitud) = match solicitud {
        SolicitudAccion::Viajar { actor, destino } => (actor, destino),
    };
    let origen_solicitud = mundo
        .ubicacion_actor(actor_solicitud)
        .ok_or(FalloFaseNpc::InvarianteRota)?;
    let inicio = match iniciar(mundo, solicitud, azar) {
        Ok(inicio) => inicio,
        Err(ErrorMotor::Rechazo(causa)) => {
            return Ok(Some(ResumenNpc::uno(
                EntradaResumenNpc::SolicitudViajeRechazada {
                    actor: actor_solicitud,
                    destino: destino_solicitud,
                    causa,
                },
            )));
        }
        Err(ErrorMotor::Fatal(fallo)) => return Err(FalloFaseNpc::Motor(fallo)),
    };

    let mut ejecucion = inicio.ejecucion;
    let mut resultados = Vec::new();
    let mut registros = Vec::new();
    incorporar_emisiones(inicio.tramo.emisiones, &mut resultados, &mut registros);
    let mut cierre = inicio.tramo.cierre;
    let mut decisiones_automaticas = 0;
    while let CierreTramo::EsperandoDecision(decision) = cierre {
        decisiones_automaticas += 1;
        let limite =
            NonZeroUsize::new(decision.opciones.len()).ok_or(FalloFaseNpc::InvarianteRota)?;
        let indice = azar
            .elegir_indice(limite)
            .map_err(FalloFaseNpc::FuenteAzar)?;
        let opcion = decision
            .opciones
            .get(indice)
            .ok_or(FalloFaseNpc::IndiceAzarFueraDeRango {
                indice,
                limite_exclusivo: limite,
            })?;
        let tramo = responder(
            mundo,
            &mut ejecucion,
            RespuestaDecision {
                decision: decision.id,
                opcion: opcion.id,
            },
            azar,
        )
        .map_err(|error| match error {
            ErrorMotor::Rechazo(rechazo) => FalloFaseNpc::RespuestaAutomaticaRechazada(rechazo),
            ErrorMotor::Fatal(fallo) => FalloFaseNpc::Motor(fallo),
        })?;
        incorporar_emisiones(tramo.emisiones, &mut resultados, &mut registros);
        cierre = tramo.cierre;
    }
    let CierreTramo::Completada(finalizacion) = cierre else {
        return Err(FalloFaseNpc::InvarianteRota);
    };
    if mundo.fecha() != fecha_antes {
        return Err(FalloFaseNpc::InvarianteRota);
    }
    let mut lugar_confirmado = origen_solicitud;
    for resultado in resultados {
        match resultado {
            ResultadoSemantico::ActoresTrasladados {
                actores,
                origen,
                destino,
            } => {
                if actores.as_slice() != [politica.actor()] || origen != lugar_confirmado {
                    return Err(FalloFaseNpc::InvarianteRota);
                }
                lugar_confirmado = destino;
            }
            ResultadoSemantico::CaracteristicaActorAnadida { actor, .. }
            | ResultadoSemantico::CaracteristicaActorQuitada { actor, .. } => {
                if actor != politica.actor() {
                    return Err(FalloFaseNpc::InvarianteRota);
                }
            }
        }
    }
    match lugar_confirmado {
        lugar if lugar == destino_solicitud => Ok(Some(ResumenNpc::con_cronologia(
            EntradaResumenNpc::ViajeCompletado {
                actor: politica.actor(),
                origen: origen_solicitud,
                destino: destino_solicitud,
            },
            CronologiaNpc {
                actor: politica.actor(),
                registros,
                decisiones_automaticas,
                duracion: finalizacion.duracion,
            },
        ))),
        lugar if mundo.ubicacion_actor(politica.actor()) == Some(lugar) => {
            Ok(Some(ResumenNpc::con_cronologia(
                EntradaResumenNpc::ViajeInterrumpido {
                    actor: politica.actor(),
                    lugar,
                },
                CronologiaNpc {
                    actor: politica.actor(),
                    registros,
                    decisiones_automaticas,
                    duracion: finalizacion.duracion,
                },
            )))
        }
        _ => Err(FalloFaseNpc::InvarianteRota),
    }
}

fn incorporar_emisiones(
    emisiones: Vec<EmisionAccion>,
    resultados: &mut Vec<ResultadoSemantico>,
    registros: &mut Vec<FragmentoNarrativo>,
) {
    for emision in emisiones {
        match emision {
            EmisionAccion::Narrativa(fragmento) => registros.push(fragmento),
            EmisionAccion::ResultadoSemantico(resultado) => resultados.push(resultado),
        }
    }
}

fn es_recuperable(error: &ErrorPoliticaLua) -> bool {
    error.etapa != EtapaLua::Validacion
        && !matches!(
            error.clase,
            ClaseErrorLua::Sintaxis | ClaseErrorLua::Infraestructura
        )
}

fn proyectar_error(error: ErrorPoliticaLua) -> DiagnosticoPoliticaNpc {
    let etapa = match error.etapa {
        EtapaLua::Preparacion => EtapaFalloPoliticaNpc::Preparacion,
        EtapaLua::Ejecucion => EtapaFalloPoliticaNpc::Ejecucion,
        EtapaLua::ConversionRespuesta => EtapaFalloPoliticaNpc::ConversionRespuesta,
        EtapaLua::Validacion => unreachable!("un error de validación no es recuperable"),
    };
    let codigo = match error.clase {
        ClaseErrorLua::LimiteInstrucciones => CodigoFalloPoliticaNpc::LimiteInstrucciones,
        ClaseErrorLua::LimiteMemoria => CodigoFalloPoliticaNpc::LimiteMemoria,
        ClaseErrorLua::ArgumentoInvalido => CodigoFalloPoliticaNpc::ArgumentoInvalido,
        ClaseErrorLua::ClaveVariableGlobalInvalida => {
            CodigoFalloPoliticaNpc::ClaveVariableGlobalInvalida
        }
        ClaseErrorLua::VariableGlobalInexistente => {
            CodigoFalloPoliticaNpc::VariableGlobalInexistente
        }
        ClaseErrorLua::TipoVariableGlobalIncorrecto => {
            CodigoFalloPoliticaNpc::TipoVariableGlobalIncorrecto
        }
        ClaseErrorLua::CaracteristicaInvalida => CodigoFalloPoliticaNpc::CaracteristicaInvalida,
        ClaseErrorLua::ClaveLugarInvalida => CodigoFalloPoliticaNpc::ClaveLugarInvalida,
        ClaseErrorLua::LugarInexistente => CodigoFalloPoliticaNpc::LugarInexistente,
        ClaseErrorLua::RespuestaInvalida => CodigoFalloPoliticaNpc::RespuestaInvalida,
        ClaseErrorLua::Ejecucion => CodigoFalloPoliticaNpc::Ejecucion,
        ClaseErrorLua::Sintaxis | ClaseErrorLua::Infraestructura => {
            unreachable!("estas clases no son recuperables")
        }
    };
    DiagnosticoPoliticaNpc {
        actor: error.actor,
        clave_actor: error.clave_actor,
        archivo_relativo: error.archivo_relativo.to_string_lossy().into_owned(),
        etapa,
        codigo,
        linea: error.linea,
        detalle: error.detalle,
    }
}
