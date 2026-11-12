use crate::contenido::CatalogoClaves;
use crate::lua::{
    ClaseErrorLua, ErrorPoliticaLua, EtapaLua, PoliticaLuaValidada, ResultadoPoliticaLua,
    evaluar_politica,
};
use desert_tiger_core::acciones::{
    self, CatalogoRumbos, CierreViaje, ContextoAccion, ErrorMotor, PasoAccion, RechazoAccion,
    RespuestaDecision,
};
use desert_tiger_core::azar::FuenteAzar;
use desert_tiger_core::{IdActor, IdLugar, InstanteAccion, Mundo, TiempoAccion};
use std::num::NonZeroUsize;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
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
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CronologiaNpc {
    pub actor: IdActor,
    pub escenas: usize,
    pub decisiones_automaticas: usize,
    pub tiempo: TiempoAccion,
    pub tiempo_visible: String,
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
pub(crate) struct FalloFaseNpc(pub String);

pub(crate) fn ejecutar_fase_npc(
    mundo: &mut Mundo,
    politicas: &[PoliticaLuaValidada],
    claves: &CatalogoClaves,
    rumbos: &CatalogoRumbos,
    inicio: InstanteAccion,
    azar: &mut dyn FuenteAzar,
) -> Result<Option<ResumenNpc>, FalloFaseNpc> {
    let mut politicas: Vec<_> = politicas.iter().collect();
    politicas.sort_by_key(|p| p.actor());
    let mut resumen = ResumenNpc::default();
    for politica in politicas {
        let actor = politica.actor();
        if actor == mundo.actor_jugador()
            || mundo.grupo().contiene(actor)
            || mundo.actor(actor).is_none_or(|a| {
                !matches!(a.ubicacion(), desert_tiger_core::UbicacionActor::EnLugar(_))
                    || a.caracteristicas().any(|c| c.como_str() == "muerto")
            })
        {
            continue;
        }
        let solicitud = match evaluar_politica(politica, mundo, claves) {
            Ok(ResultadoPoliticaLua::Ninguna) => continue,
            Ok(ResultadoPoliticaLua::Solicitud(s)) => s,
            Err(e)
                if e.etapa != EtapaLua::Validacion
                    && !matches!(
                        e.clase,
                        ClaseErrorLua::Sintaxis | ClaseErrorLua::Infraestructura
                    ) =>
            {
                resumen.entradas.push(EntradaResumenNpc::ErrorPolitica {
                    diagnostico: diagnostico(e),
                });
                continue;
            }
            Err(e) => return Err(FalloFaseNpc(format!("política Lua inválida: {e:?}"))),
        };
        let acciones::SolicitudAccion::Viajar { destino, .. } = solicitud;
        let origen = mundo
            .ubicacion_actor(actor)
            .ok_or_else(|| FalloFaseNpc("NPC sin Lugar".into()))?;
        let contexto = ContextoAccion { inicio, rumbos };
        let iniciado = match acciones::iniciar(mundo, solicitud, &contexto, azar) {
            Ok(v) => v,
            Err(ErrorMotor::Rechazo(causa)) => {
                resumen
                    .entradas
                    .push(EntradaResumenNpc::SolicitudViajeRechazada {
                        actor,
                        destino,
                        causa,
                    });
                continue;
            }
            Err(e) => return Err(FalloFaseNpc(e.to_string())),
        };
        let mut ejecucion = iniciado.ejecucion;
        let mut paso = iniciado.paso;
        let mut escenas = 0;
        let mut decisiones = 0;
        let fin = loop {
            paso = match paso {
                PasoAccion::Escena(r) => {
                    escenas += 1;
                    acciones::avanzar(mundo, &mut ejecucion, r.id, &contexto, azar)
                        .map_err(|e| FalloFaseNpc(e.to_string()))?
                }
                PasoAccion::Decision(d) => {
                    decisiones += 1;
                    let n = NonZeroUsize::new(d.opciones.len())
                        .ok_or_else(|| FalloFaseNpc("decisión vacía".into()))?;
                    let i = azar
                        .elegir_indice(n)
                        .map_err(|_| FalloFaseNpc("azar NPC fallido".into()))?;
                    let opcion = *d
                        .opciones
                        .get(i)
                        .ok_or_else(|| FalloFaseNpc("índice NPC inválido".into()))?;
                    acciones::responder(
                        mundo,
                        &mut ejecucion,
                        RespuestaDecision {
                            paso: d.escena.id,
                            opcion,
                        },
                        &contexto,
                        azar,
                    )
                    .map_err(|e| FalloFaseNpc(e.to_string()))?
                }
                PasoAccion::Finalizada(f) => break f,
            };
        };
        if mundo.fecha() != inicio.fecha {
            return Err(FalloFaseNpc("un NPC modificó la fecha".into()));
        }
        resumen
            .entradas
            .push(if fin.cierre == CierreViaje::Llegada {
                EntradaResumenNpc::ViajeCompletado {
                    actor,
                    origen,
                    destino,
                }
            } else {
                EntradaResumenNpc::ViajeInterrumpido {
                    actor,
                    lugar: fin.lugar,
                }
            });
        resumen.cronologias.push(CronologiaNpc {
            actor,
            escenas,
            decisiones_automaticas: decisiones,
            tiempo: fin.tiempo,
            tiempo_visible: crate::tiempo::formatear(fin.tiempo),
        });
    }
    Ok(if resumen.entradas.is_empty() {
        None
    } else {
        Some(resumen)
    })
}
fn diagnostico(e: ErrorPoliticaLua) -> DiagnosticoPoliticaNpc {
    let etapa = match e.etapa {
        EtapaLua::Preparacion => EtapaFalloPoliticaNpc::Preparacion,
        EtapaLua::ConversionRespuesta => EtapaFalloPoliticaNpc::ConversionRespuesta,
        _ => EtapaFalloPoliticaNpc::Ejecucion,
    };
    let codigo = match e.clase {
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
        _ => CodigoFalloPoliticaNpc::Ejecucion,
    };
    DiagnosticoPoliticaNpc {
        actor: e.actor,
        clave_actor: e.clave_actor,
        archivo_relativo: e.archivo_relativo.to_string_lossy().into_owned(),
        etapa,
        codigo,
        linea: e.linea,
        detalle: e.detalle,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn actor_externo_vivo_omite_lua_incluso_en_una_llamada_directa() {
        struct SinAzar;
        impl FuenteAzar for SinAzar {
            fn elegir_indice(
                &mut self,
                _: NonZeroUsize,
            ) -> Result<usize, desert_tiger_core::azar::ErrorFuenteAzar> {
                panic!("un externo no puede consumir azar")
            }
        }
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mut mundo, _) = c.instanciar_estado().unwrap();
        let actor = IdActor::desde_posicion(8).unwrap();
        assert!(mundo.actor(actor).unwrap().lugar_directo().is_none());
        assert!(
            !mundo
                .actor(actor)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "muerto")
        );
        let politica = crate::lua::validar_politica(
            actor,
            "hermano_del_rey".into(),
            "actors/hermano_del_rey.ron".into(),
            "error('esta política no debe ejecutarse')".into(),
        )
        .unwrap();
        assert!(matches!(
            evaluar_politica(&politica, &mundo, c.claves()).unwrap(),
            ResultadoPoliticaLua::Ninguna
        ));
        let rumbos = crate::contexto_viaje::construir(&mundo, c.presentacion()).unwrap();
        let inicio = InstanteAccion {
            fecha: mundo.fecha(),
            fraccion: desert_tiger_core::FraccionDia::CERO,
        };
        assert!(
            ejecutar_fase_npc(
                &mut mundo,
                &[politica],
                c.claves(),
                &rumbos,
                inicio,
                &mut SinAzar
            )
            .unwrap()
            .is_none()
        );
    }
    struct Maximo;
    impl FuenteAzar for Maximo {
        fn elegir_indice(
            &mut self,
            n: NonZeroUsize,
        ) -> Result<usize, desert_tiger_core::azar::ErrorFuenteAzar> {
            Ok(n.get() - 1)
        }
    }
    #[test]
    fn cero_y_varios_npc_incluido_un_companero_separado() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mut mundo, _) = c.instanciar_estado().unwrap();
        let bandido = IdActor::desde_posicion(1).unwrap();
        let cazador = IdActor::desde_posicion(2).unwrap();
        let lugar = mundo.ubicacion_actor(bandido).unwrap();
        mundo
            .aplicar_lote(desert_tiger_core::LoteOperaciones::uno(
                desert_tiger_core::OperacionMundo::SepararCompanero {
                    actor: bandido,
                    lugar,
                },
            ))
            .unwrap();
        let r = crate::contexto_viaje::construir(&mundo, c.presentacion()).unwrap();
        let inicio = InstanteAccion {
            fecha: mundo.fecha(),
            fraccion: desert_tiger_core::FraccionDia::CERO,
        };
        let mut azar = Maximo;
        assert!(
            ejecutar_fase_npc(&mut mundo, &[], c.claves(), &r, inicio, &mut azar)
                .unwrap()
                .is_none()
        );
        let politicas = vec![
            crate::lua::validar_politica(
                bandido,
                "bandido".into(),
                "actors/bandido.ron".into(),
                "return acciones.viajar('valle_fertil')".into(),
            )
            .unwrap(),
            crate::lua::validar_politica(
                cazador,
                "cazador".into(),
                "actors/cazador.ron".into(),
                "return acciones.viajar('zona_arida')".into(),
            )
            .unwrap(),
        ];
        let resumen = ejecutar_fase_npc(&mut mundo, &politicas, c.claves(), &r, inicio, &mut azar)
            .unwrap()
            .unwrap();
        assert_eq!(resumen.entradas().len(), 2);
        assert_eq!(resumen.cronologias().len(), 2);
        assert_eq!(mundo.fecha(), inicio.fecha);
        assert!(
            resumen
                .cronologias()
                .iter()
                .all(|c| c.escenas + c.decisiones_automaticas > 0)
        );
    }
}
