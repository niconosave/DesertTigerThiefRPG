use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use desert_tiger_core::acciones::SolicitudAccion;
use desert_tiger_core::{
    Caracteristica, ClaveVariableGlobal, IdActor, IdLugar, Mundo, ValorVariableGlobal,
};
use mlua::{
    Error as ErrorMlua, HookTriggers, Lua, LuaOptions, MultiValue, StdLib, Table, UserData, Value,
    VmState,
};

use crate::contenido::{clave_externa_valida, sanear};

pub(crate) const LIMITE_MEMORIA_LUA_BYTES: usize = 2 * 1024 * 1024;
pub(crate) const INTERVALO_HOOK_LUA: u32 = 1_000;
pub(crate) const LIMITE_INSTRUCCIONES_LUA: u32 = 100_000;

pub(crate) trait ResolucionClavesLua {
    fn clave_actor(&self, actor: IdActor) -> Option<&str>;
    fn clave_lugar(&self, lugar: IdLugar) -> Option<&str>;
    fn lugar_por_clave(&self, clave: &str) -> Option<IdLugar>;
}

#[derive(Debug)]
pub(crate) struct PoliticaLuaValidada {
    actor: IdActor,
    clave_actor: String,
    archivo_relativo: PathBuf,
    codigo: String,
}

impl PoliticaLuaValidada {
    pub(crate) fn actor(&self) -> IdActor {
        self.actor
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EtapaLua {
    Validacion,
    Preparacion,
    Ejecucion,
    ConversionRespuesta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaseErrorLua {
    Sintaxis,
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
    Infraestructura,
}

#[derive(Debug)]
pub(crate) struct ErrorPoliticaLua {
    pub actor: IdActor,
    pub clave_actor: String,
    pub archivo_relativo: PathBuf,
    pub etapa: EtapaLua,
    pub clase: ClaseErrorLua,
    pub linea: Option<u32>,
    pub detalle: String,
}

impl Display for ErrorPoliticaLua {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "política Lua inválida para {} en {}: {}",
            self.clave_actor,
            self.archivo_relativo.display(),
            self.detalle
        )
    }
}

impl Error for ErrorPoliticaLua {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResultadoPoliticaLua {
    Ninguna,
    Solicitud(SolicitudAccion),
}

#[derive(Clone)]
enum SolicitudLua {
    Viajar { destino: String },
}

impl UserData for SolicitudLua {}

pub(crate) fn validar_politica(
    actor: IdActor,
    clave_actor: String,
    archivo_relativo: PathBuf,
    codigo: String,
) -> Result<PoliticaLuaValidada, ErrorPoliticaLua> {
    let lua = crear_maquina().map_err(|error| {
        convertir_error(
            actor,
            &clave_actor,
            &archivo_relativo,
            EtapaLua::Validacion,
            error,
        )
    })?;
    let nombre = format!("{}:politica", archivo_relativo.display());
    lua.load(&codigo)
        .set_name(nombre)
        .into_function()
        .map_err(|error| {
            convertir_error(
                actor,
                &clave_actor,
                &archivo_relativo,
                EtapaLua::Validacion,
                error,
            )
        })?;
    Ok(PoliticaLuaValidada {
        actor,
        clave_actor,
        archivo_relativo,
        codigo,
    })
}

pub(crate) fn evaluar_politica<R: ResolucionClavesLua>(
    politica: &PoliticaLuaValidada,
    mundo: &Mundo,
    claves: &R,
) -> Result<ResultadoPoliticaLua, ErrorPoliticaLua> {
    let actor = mundo.actor(politica.actor).ok_or_else(|| {
        error_directo(
            politica,
            EtapaLua::Preparacion,
            ClaseErrorLua::Infraestructura,
            "el actor asociado no existe en el Mundo".to_owned(),
        )
    })?;
    let Some(lugar) = actor.lugar() else {
        return Ok(ResultadoPoliticaLua::Ninguna);
    };
    let clave_actor = claves.clave_actor(actor.id()).ok_or_else(|| {
        error_directo(
            politica,
            EtapaLua::Preparacion,
            ClaseErrorLua::Infraestructura,
            "el actor asociado no resuelve a una clave externa".to_owned(),
        )
    })?;
    if clave_actor != politica.clave_actor {
        return Err(error_directo(
            politica,
            EtapaLua::Preparacion,
            ClaseErrorLua::Infraestructura,
            "la clave del actor no coincide con la política".to_owned(),
        ));
    }
    let clave_lugar = claves.clave_lugar(lugar).ok_or_else(|| {
        error_directo(
            politica,
            EtapaLua::Preparacion,
            ClaseErrorLua::Infraestructura,
            "el lugar del actor no resuelve a una clave externa".to_owned(),
        )
    })?;

    let fecha = mundo.fecha();
    let caracteristicas: Vec<String> = actor
        .caracteristicas()
        .map(|valor| valor.como_str().to_owned())
        .collect();
    let globales: BTreeMap<String, ValorVariableGlobal> = mundo
        .variables_globales()
        .map(|(clave, valor)| (clave.como_str().to_owned(), valor.clone()))
        .collect();
    let lua = crear_maquina()
        .map_err(|error| convertir_error_politica(politica, EtapaLua::Preparacion, error))?;
    let entorno = crear_entorno(
        &lua,
        fecha.anio(),
        fecha.mes().numero(),
        fecha.dia(),
        clave_actor.to_owned(),
        clave_lugar.to_owned(),
        caracteristicas,
        globales,
    )
    .map_err(|error| convertir_error_politica(politica, EtapaLua::Preparacion, error))?;

    let consumidas = Arc::new(AtomicU32::new(0));
    let contador = Arc::clone(&consumidas);
    lua.set_hook(
        HookTriggers::new().every_nth_instruction(INTERVALO_HOOK_LUA),
        move |_, _| {
            let total =
                contador.fetch_add(INTERVALO_HOOK_LUA, Ordering::Relaxed) + INTERVALO_HOOK_LUA;
            if total >= LIMITE_INSTRUCCIONES_LUA {
                Err(ErrorMlua::runtime("DT_LIMITE_INSTRUCCIONES"))
            } else {
                Ok(VmState::Continue)
            }
        },
    )
    .map_err(|error| convertir_error_politica(politica, EtapaLua::Preparacion, error))?;

    let nombre = format!("{}:politica", politica.archivo_relativo.display());
    let funcion = lua
        .load(&politica.codigo)
        .set_name(nombre)
        .set_environment(entorno)
        .into_function()
        .map_err(|error| convertir_error_politica(politica, EtapaLua::Ejecucion, error))?;
    let respuesta: MultiValue = funcion
        .call(())
        .map_err(|error| convertir_error_politica(politica, EtapaLua::Ejecucion, error))?;
    convertir_respuesta(politica, respuesta, claves)
}

pub(crate) fn crear_maquina() -> mlua::Result<Lua> {
    let lua = Lua::new_with(StdLib::NONE, LuaOptions::new())?;
    lua.set_memory_limit(LIMITE_MEMORIA_LUA_BYTES)?;
    Ok(lua)
}

#[allow(clippy::too_many_arguments)]
fn crear_entorno(
    lua: &Lua,
    anio: u16,
    mes: u8,
    dia: u8,
    clave_actor: String,
    clave_lugar: String,
    caracteristicas: Vec<String>,
    globales: BTreeMap<String, ValorVariableGlobal>,
) -> mlua::Result<Table> {
    let fecha_datos = lua.create_table()?;
    fecha_datos.set("anio", anio)?;
    fecha_datos.set("mes", mes)?;
    fecha_datos.set("dia", dia)?;
    let fecha = proxy_solo_lectura(lua, fecha_datos)?;

    let mundo_datos = lua.create_table()?;
    mundo_datos.set("fecha", fecha)?;
    for (nombre, tipo) in [
        ("booleano", TipoConsultaGlobal::Booleana),
        ("entero", TipoConsultaGlobal::Entera),
        ("texto", TipoConsultaGlobal::Texto),
    ] {
        let copia = globales.clone();
        mundo_datos.set(
            nombre,
            lua.create_function(move |lua, argumentos: MultiValue| {
                let clave = argumento_texto_exacto(argumentos)?;
                let clave_validada = ClaveVariableGlobal::nueva(clave.clone())
                    .map_err(|_| ErrorMlua::runtime("DT_CLAVE_GLOBAL_INVALIDA"))?;
                let valor = copia
                    .get(clave_validada.como_str())
                    .ok_or_else(|| ErrorMlua::runtime("DT_GLOBAL_INEXISTENTE"))?;
                convertir_global(lua, valor, tipo)
            })?,
        )?;
    }
    let mundo = proxy_solo_lectura(lua, mundo_datos)?;

    let actor_datos = lua.create_table()?;
    actor_datos.set("clave", clave_actor)?;
    actor_datos.set("lugar", clave_lugar)?;
    actor_datos.set(
        "tiene_caracteristica",
        lua.create_function(move |_, argumentos: MultiValue| {
            let texto = argumento_texto_exacto(argumentos)?;
            let caracteristica = Caracteristica::nueva(texto)
                .map_err(|_| ErrorMlua::runtime("DT_CARACTERISTICA_INVALIDA"))?;
            Ok(caracteristicas
                .iter()
                .any(|actual| actual == caracteristica.como_str()))
        })?,
    )?;
    let actor = proxy_solo_lectura(lua, actor_datos)?;

    let acciones_datos = lua.create_table()?;
    acciones_datos.set(
        "viajar",
        lua.create_function(|lua, argumentos: MultiValue| {
            let destino = argumento_texto_exacto(argumentos)?;
            lua.create_userdata(SolicitudLua::Viajar { destino })
        })?,
    )?;
    let acciones = proxy_solo_lectura(lua, acciones_datos)?;

    let entorno_datos = lua.create_table()?;
    entorno_datos.set("mundo", mundo)?;
    entorno_datos.set("actor", actor)?;
    entorno_datos.set("acciones", acciones)?;
    proxy_solo_lectura(lua, entorno_datos)
}

pub(crate) fn proxy_solo_lectura(lua: &Lua, datos: Table) -> mlua::Result<Table> {
    let proxy = lua.create_table()?;
    let metatabla = lua.create_table()?;
    let longitud = datos.raw_len();
    metatabla.set(
        "__len",
        lua.create_function(move |_, _: MultiValue| Ok(longitud))?,
    )?;
    metatabla.set("__index", datos)?;
    metatabla.set(
        "__newindex",
        lua.create_function(|_, _: MultiValue| -> mlua::Result<()> {
            Err(ErrorMlua::runtime("DT_ESCRITURA_PROHIBIDA"))
        })?,
    )?;
    metatabla.set("__metatable", "protegida")?;
    proxy.set_metatable(Some(metatabla))?;
    Ok(proxy)
}

#[derive(Clone, Copy)]
enum TipoConsultaGlobal {
    Booleana,
    Entera,
    Texto,
}

fn convertir_global(
    lua: &Lua,
    valor: &ValorVariableGlobal,
    tipo: TipoConsultaGlobal,
) -> mlua::Result<Value> {
    match (valor, tipo) {
        (ValorVariableGlobal::Booleana(valor), TipoConsultaGlobal::Booleana) => {
            Ok(Value::Boolean(*valor))
        }
        (ValorVariableGlobal::Entera(valor), TipoConsultaGlobal::Entera) => {
            Ok(Value::Integer(*valor))
        }
        (ValorVariableGlobal::Texto(valor), TipoConsultaGlobal::Texto) => {
            Ok(Value::String(lua.create_string(valor)?))
        }
        _ => Err(ErrorMlua::runtime("DT_TIPO_GLOBAL_INCORRECTO")),
    }
}

fn argumento_texto_exacto(argumentos: MultiValue) -> mlua::Result<String> {
    if argumentos.len() != 1 {
        return Err(ErrorMlua::runtime("DT_ARGUMENTO_INVALIDO"));
    }
    let Some(Value::String(texto)) = argumentos.into_iter().next() else {
        return Err(ErrorMlua::runtime("DT_ARGUMENTO_INVALIDO"));
    };
    texto
        .to_str()
        .map(|texto| texto.as_ref().to_owned())
        .map_err(|_| ErrorMlua::runtime("DT_ARGUMENTO_INVALIDO"))
}

fn convertir_respuesta<R: ResolucionClavesLua>(
    politica: &PoliticaLuaValidada,
    respuesta: MultiValue,
    claves: &R,
) -> Result<ResultadoPoliticaLua, ErrorPoliticaLua> {
    if respuesta.is_empty() || respuesta.len() == 1 && matches!(respuesta[0], Value::Nil) {
        return Ok(ResultadoPoliticaLua::Ninguna);
    }
    if respuesta.len() != 1 {
        return Err(error_directo(
            politica,
            EtapaLua::ConversionRespuesta,
            ClaseErrorLua::RespuestaInvalida,
            "la política devolvió más de un valor".to_owned(),
        ));
    }
    let Value::UserData(datos) = &respuesta[0] else {
        return Err(error_directo(
            politica,
            EtapaLua::ConversionRespuesta,
            ClaseErrorLua::RespuestaInvalida,
            "la política devolvió un tipo no admitido".to_owned(),
        ));
    };
    let solicitud = datos.borrow::<SolicitudLua>().map_err(|_| {
        error_directo(
            politica,
            EtapaLua::ConversionRespuesta,
            ClaseErrorLua::RespuestaInvalida,
            "la política devolvió userdata ajeno".to_owned(),
        )
    })?;
    match &*solicitud {
        SolicitudLua::Viajar { destino } => {
            if !clave_externa_valida(destino) {
                return Err(error_directo(
                    politica,
                    EtapaLua::ConversionRespuesta,
                    ClaseErrorLua::ClaveLugarInvalida,
                    "la clave de destino es inválida".to_owned(),
                ));
            }
            let destino = claves.lugar_por_clave(destino).ok_or_else(|| {
                error_directo(
                    politica,
                    EtapaLua::ConversionRespuesta,
                    ClaseErrorLua::LugarInexistente,
                    "el destino solicitado no existe".to_owned(),
                )
            })?;
            Ok(ResultadoPoliticaLua::Solicitud(SolicitudAccion::Viajar {
                actor: politica.actor,
                destino,
            }))
        }
    }
}

fn convertir_error_politica(
    politica: &PoliticaLuaValidada,
    etapa: EtapaLua,
    error: ErrorMlua,
) -> ErrorPoliticaLua {
    convertir_error(
        politica.actor,
        &politica.clave_actor,
        &politica.archivo_relativo,
        etapa,
        error,
    )
}

fn convertir_error(
    actor: IdActor,
    clave_actor: &str,
    archivo: &Path,
    etapa: EtapaLua,
    error: ErrorMlua,
) -> ErrorPoliticaLua {
    let texto = error.to_string();
    let clase = if matches!(error, ErrorMlua::SyntaxError { .. }) {
        ClaseErrorLua::Sintaxis
    } else if matches!(error, ErrorMlua::MemoryError(_)) {
        ClaseErrorLua::LimiteMemoria
    } else if texto.contains("DT_LIMITE_INSTRUCCIONES") {
        ClaseErrorLua::LimiteInstrucciones
    } else if texto.contains("DT_ARGUMENTO_INVALIDO") {
        ClaseErrorLua::ArgumentoInvalido
    } else if texto.contains("DT_CLAVE_GLOBAL_INVALIDA") {
        ClaseErrorLua::ClaveVariableGlobalInvalida
    } else if texto.contains("DT_GLOBAL_INEXISTENTE") {
        ClaseErrorLua::VariableGlobalInexistente
    } else if texto.contains("DT_TIPO_GLOBAL_INCORRECTO") {
        ClaseErrorLua::TipoVariableGlobalIncorrecto
    } else if texto.contains("DT_CARACTERISTICA_INVALIDA") {
        ClaseErrorLua::CaracteristicaInvalida
    } else {
        ClaseErrorLua::Ejecucion
    };
    ErrorPoliticaLua {
        actor,
        clave_actor: clave_actor.to_owned(),
        archivo_relativo: archivo.to_path_buf(),
        etapa,
        clase,
        linea: None,
        detalle: sanear(&texto),
    }
}

fn error_directo(
    politica: &PoliticaLuaValidada,
    etapa: EtapaLua,
    clase: ClaseErrorLua,
    detalle: String,
) -> ErrorPoliticaLua {
    ErrorPoliticaLua {
        actor: politica.actor,
        clave_actor: politica.clave_actor.clone(),
        archivo_relativo: politica.archivo_relativo.clone(),
        etapa,
        clase,
        linea: None,
        detalle: sanear(&detalle),
    }
}

#[cfg(test)]
mod pruebas {
    use std::collections::BTreeMap;

    use super::*;
    use desert_tiger_core::{
        ActorInicial, DatosInicialesMundo, Fecha, IdMapa, LugarInicial, Mes, VariableGlobalInicial,
    };

    struct ClavesPrueba {
        actores: BTreeMap<IdActor, String>,
        lugares: BTreeMap<IdLugar, String>,
    }

    impl ResolucionClavesLua for ClavesPrueba {
        fn clave_actor(&self, actor: IdActor) -> Option<&str> {
            self.actores.get(&actor).map(String::as_str)
        }
        fn clave_lugar(&self, lugar: IdLugar) -> Option<&str> {
            self.lugares.get(&lugar).map(String::as_str)
        }
        fn lugar_por_clave(&self, clave: &str) -> Option<IdLugar> {
            self.lugares
                .iter()
                .find_map(|(id, actual)| (actual == clave).then_some(*id))
        }
    }

    fn id_actor(posicion: usize) -> IdActor {
        IdActor::desde_posicion(posicion).expect("ID válido")
    }
    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID válido")
    }

    fn mundo_y_claves() -> (Mundo, ClavesPrueba) {
        let mundo = Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID válido"),
            fecha: Fecha::nueva(2025, Mes::Febrero, 2).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: id_lugar(0),
                    caracteristicas: vec![],
                },
                LugarInicial {
                    id: id_lugar(1),
                    caracteristicas: vec![],
                },
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor(0),
                    ubicacion: desert_tiger_core::UbicacionActor::EnLugar(id_lugar(0)),
                    caracteristicas: vec![],
                },
                ActorInicial {
                    id: id_actor(1),
                    ubicacion: desert_tiger_core::UbicacionActor::EnLugar(id_lugar(0)),
                    caracteristicas: vec![],
                },
                ActorInicial {
                    id: id_actor(2),
                    ubicacion: desert_tiger_core::UbicacionActor::EnLugar(id_lugar(1)),
                    caracteristicas: vec![
                        Caracteristica::nueva("rapido".to_owned()).expect("característica válida"),
                    ],
                },
            ],
            actor_jugador: id_actor(0),
            companeros: vec![id_actor(1)],
            variables_globales: vec![
                VariableGlobalInicial {
                    clave: ClaveVariableGlobal::nueva("eventoOcurrio".to_owned())
                        .expect("clave válida"),
                    valor: ValorVariableGlobal::Booleana(true),
                },
                VariableGlobalInicial {
                    clave: ClaveVariableGlobal::nueva("contador".to_owned()).expect("clave válida"),
                    valor: ValorVariableGlobal::Entera(4),
                },
                VariableGlobalInicial {
                    clave: ClaveVariableGlobal::nueva("estado".to_owned()).expect("clave válida"),
                    valor: ValorVariableGlobal::Texto("abierto".to_owned()),
                },
            ],
            relaciones: vec![],
        })
        .expect("mundo válido");
        let claves = ClavesPrueba {
            actores: [
                (id_actor(0), "jugador"),
                (id_actor(1), "companero"),
                (id_actor(2), "cazador"),
            ]
            .into_iter()
            .map(|(id, clave)| (id, clave.to_owned()))
            .collect(),
            lugares: [(id_lugar(0), "zona_arida"), (id_lugar(1), "valle_fertil")]
                .into_iter()
                .map(|(id, clave)| (id, clave.to_owned()))
                .collect(),
        };
        (mundo, claves)
    }

    fn politica(codigo: &str) -> PoliticaLuaValidada {
        validar_politica(
            id_actor(2),
            "cazador".to_owned(),
            PathBuf::from("actors/cazador.ron"),
            codigo.to_owned(),
        )
        .expect("sintaxis válida")
    }

    #[test]
    fn consulta_fecha_globales_y_actor_con_api_minima() {
        let (mundo, claves) = mundo_y_claves();
        let politica = politica(
            r#"
if mundo.fecha.mes == 2
    and mundo.booleano("eventoOcurrio")
    and mundo.entero("contador") == 4
    and mundo.texto("estado") == "abierto"
    and actor.clave == "cazador"
    and actor.lugar == "valle_fertil"
    and actor.tiene_caracteristica("rapido")
then
    return acciones.viajar("zona_arida")
end
return nil
"#,
        );
        assert_eq!(
            evaluar_politica(&politica, &mundo, &claves).expect("evaluación válida"),
            ResultadoPoliticaLua::Solicitud(SolicitudAccion::Viajar {
                actor: id_actor(2),
                destino: id_lugar(0),
            })
        );
    }

    #[test]
    fn entorno_no_expone_bibliotecas_y_rechaza_escritura() {
        let (mundo, claves) = mundo_y_claves();
        let sin_biblioteca = politica("return os.time()");
        assert_eq!(
            evaluar_politica(&sin_biblioteca, &mundo, &claves)
                .expect_err("os debe estar ausente")
                .clase,
            ClaseErrorLua::Ejecucion
        );
        let escritura = politica("mundo.fecha.mes = 3; return nil");
        assert!(evaluar_politica(&escritura, &mundo, &claves).is_err());
        assert_eq!(mundo.fecha().mes(), Mes::Febrero);
    }

    #[test]
    fn limita_instrucciones_y_valida_respuesta_exacta() {
        let (mundo, claves) = mundo_y_claves();
        let bucle = politica("while true do end");
        assert_eq!(
            evaluar_politica(&bucle, &mundo, &claves)
                .expect_err("el bucle debe interrumpirse")
                .clase,
            ClaseErrorLua::LimiteInstrucciones
        );
        let multiple = politica("return nil, nil");
        assert_eq!(
            evaluar_politica(&multiple, &mundo, &claves)
                .expect_err("varios resultados deben rechazarse")
                .clase,
            ClaseErrorLua::RespuestaInvalida
        );
    }

    #[test]
    fn validacion_compila_sin_ejecutar() {
        let valida_pero_falla_al_ejecutar = validar_politica(
            id_actor(2),
            "cazador".to_owned(),
            PathBuf::from("actors/cazador.ron"),
            "error_inexistente()".to_owned(),
        );
        assert!(valida_pero_falla_al_ejecutar.is_ok());
        let invalida = validar_politica(
            id_actor(2),
            "cazador".to_owned(),
            PathBuf::from("actors/cazador.ron"),
            "if then".to_owned(),
        )
        .expect_err("la sintaxis inválida debe rechazarse");
        assert_eq!(invalida.etapa, EtapaLua::Validacion);
        assert_eq!(invalida.clase, ClaseErrorLua::Sintaxis);
    }
}
