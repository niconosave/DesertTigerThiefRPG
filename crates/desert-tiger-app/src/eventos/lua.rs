use crate::{
    contenido::{CatalogoClaves, validar_texto_presentacion},
    lua::{INTERVALO_HOOK_LUA, LIMITE_INSTRUCCIONES_LUA, crear_maquina, proxy_solo_lectura},
};
use desert_tiger_core::acciones::intervenciones::ContextoIntervencion;
use desert_tiger_core::{
    Caracteristica, ClaveVariableGlobal, ExtremoRelacion, Fecha, IdActor, IdColectivo, IdLugar,
    Mundo, OperacionMundo, TipoRelacion, ValorVariableGlobal,
};
use mlua::{HookTriggers, Lua, MultiValue, Scope, Table, UserData, Value, VmState};
use serde::{Deserialize, Deserializer};
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

#[derive(Clone, Debug)]
pub struct Programa {
    fuente: String,
    compilado: Vec<u8>,
    nombre: String,
}
impl<'de> Deserialize<'de> for Programa {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self {
            fuente: String::deserialize(d)?,
            compilado: vec![],
            nombre: String::new(),
        })
    }
}
#[derive(Clone)]
struct Operacion(OperacionMundo);
impl UserData for Operacion {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Momento {
    pub actor: Option<IdActor>,
    pub texto: String,
}
impl UserData for Momento {}
pub struct Presentacion {
    pub momentos: Vec<Momento>,
    pub titulo: String,
    pub detalle: Option<String>,
}
pub struct Efectos {
    pub acciones_colectivas: Vec<IdColectivo>,
    pub operaciones: Vec<OperacionMundo>,
    pub siguiente: String,
    pub solicitar: Vec<String>,
}
pub enum Respuesta {
    Condicion(bool),
    Presentacion(Presentacion),
    Efectos(Efectos),
}
#[derive(Clone, Copy)]
pub enum TipoPrograma {
    Condicion,
    Presentacion,
    Efectos,
}
fn err(s: impl ToString) -> mlua::Error {
    mlua::Error::runtime(s.to_string())
}
fn string(v: &Value) -> mlua::Result<String> {
    match v {
        Value::String(s) => Ok(s.to_str()?.to_owned()),
        _ => Err(err("se requiere cadena UTF-8")),
    }
}
fn entero(v: &Value) -> mlua::Result<i64> {
    match v {
        Value::Integer(n) => Ok(*n),
        _ => Err(err("se requiere entero i64")),
    }
}
fn booleano(v: &Value) -> mlua::Result<bool> {
    match v {
        Value::Boolean(n) => Ok(*n),
        _ => Err(err("se requiere booleano")),
    }
}
fn campo(t: &Table, k: &str) -> mlua::Result<String> {
    string(&t.get::<Value>(k)?)
}
fn campos(t: &Table, validos: &[&str]) -> mlua::Result<()> {
    for p in t.clone().pairs::<Value, Value>() {
        let (k, _) = p?;
        let k = string(&k)?;
        if !validos.contains(&k.as_str()) {
            return Err(err(format!("campo desconocido {k}")));
        }
    }
    Ok(())
}
fn lista(t: Table) -> mlua::Result<Vec<Value>> {
    let n = t.raw_len();
    let mut cantidad = 0;
    for p in t.clone().pairs::<Value, Value>() {
        let (k, _) = p?;
        let i = entero(&k)?;
        if i < 1 || i as usize > n {
            return Err(err("lista no contigua"));
        }
        cantidad += 1;
    }
    if cantidad != n {
        return Err(err("lista con huecos"));
    }
    (1..=n).map(|i| t.raw_get(i)).collect()
}
fn actor(c: &CatalogoClaves, s: &Value) -> mlua::Result<IdActor> {
    let s = string(s)?;
    c.actor_por_clave(&s)
        .ok_or_else(|| err(format!("Actor inexistente {s}")))
}
fn colectivo(c: &CatalogoClaves, v: &Value) -> mlua::Result<IdColectivo> {
    let k = string(v)?;
    c.colectivo_por_clave(&k)
        .ok_or_else(|| err(format!("colectivo inexistente {k}")))
}
fn lugar(c: &CatalogoClaves, s: &Value) -> mlua::Result<IdLugar> {
    let s = string(s)?;
    c.lugar_por_clave(&s)
        .ok_or_else(|| err(format!("Lugar inexistente {s}")))
}
fn valor(lua: &Lua, v: &ValorVariableGlobal) -> mlua::Result<Value> {
    Ok(match v {
        ValorVariableGlobal::Booleana(b) => Value::Boolean(*b),
        ValorVariableGlobal::Entera(n) => Value::Integer(*n),
        ValorVariableGlobal::Texto(s) => Value::String(lua.create_string(s)?),
    })
}
fn texto(lua: &Lua, s: Option<&str>) -> mlua::Result<Value> {
    s.map(|s| lua.create_string(s).map(Value::String))
        .unwrap_or(Ok(Value::Nil))
}
fn funcion<'s, 'e>(
    scope: &'s Scope<'s, 'e>,
    tabla: &Table,
    nombre: &str,
    min: usize,
    max: usize,
    metodo: bool,
    f: impl Fn(&Lua, Vec<Value>) -> mlua::Result<Value> + 's,
) -> mlua::Result<()> {
    tabla.set(
        nombre,
        scope.create_function(move |lua, v: MultiValue| {
            let mut v = v.into_vec();
            if metodo {
                if !matches!(v.first(), Some(Value::Table(_))) {
                    return Err(err("llamada requiere ':'"));
                }
                v.remove(0);
            }
            if v.len() < min || v.len() > max {
                return Err(err("cantidad de argumentos inválida"));
            }
            f(lua, v)
        })?,
    )
}
impl Programa {
    pub fn compilar(&mut self, nombre: &str) -> Result<(), String> {
        let lua = crear_maquina().map_err(|e| e.to_string())?;
        self.compilado = lua
            .load(&self.fuente)
            .set_name(nombre)
            .into_function()
            .map_err(|e| e.to_string())?
            .dump(false);
        self.nombre = nombre.into();
        Ok(())
    }
    #[cfg(test)]
    pub fn ejecutar(
        &self,
        tipo: TipoPrograma,
        mundo: &Mundo,
        c: &CatalogoClaves,
        ctx: &ContextoIntervencion,
    ) -> Result<Respuesta, String> {
        self.ejecutar_con_agenda(tipo, mundo, c, ctx, (None, &[], None, true))
    }
    pub fn ejecutar_con_agenda(
        &self,
        tipo: TipoPrograma,
        mundo: &Mundo,
        c: &CatalogoClaves,
        ctx: &ContextoIntervencion,
        agenda: (Option<Fecha>, &[IdColectivo], Option<IdLugar>, bool),
    ) -> Result<Respuesta, String> {
        let (fecha_suceso, futuras, sede, presente) = agenda;
        let run = || -> mlua::Result<Respuesta> {
            let lua = crear_maquina()?;
            let contador = Arc::new(AtomicU32::new(0));
            lua.set_hook(
                HookTriggers::new().every_nth_instruction(INTERVALO_HOOK_LUA),
                move |_, _| {
                    if contador.fetch_add(INTERVALO_HOOK_LUA, Ordering::Relaxed)
                        >= LIMITE_INSTRUCCIONES_LUA
                    {
                        return Err(err("límite de instrucciones Lua"));
                    }
                    Ok(VmState::Continue)
                },
            )?;
            lua.scope(|scope| {
                let env = lua.create_table()?;
                let consultas = lua.create_table()?;
                let contexto = lua.create_table()?;
                for (nombre, s) in [
                    (
                        "punto",
                        Some(if fecha_suceso.is_some() {
                            "mundo.fecha_programada"
                        } else {
                            ctx.clase.clave()
                        }),
                    ),
                    (
                        "accion",
                        if fecha_suceso.is_some() {
                            None
                        } else {
                            ctx.accion
                        },
                    ),
                    (
                        "actor",
                        if fecha_suceso.is_some() {
                            None
                        } else {
                            c.clave_actor(ctx.actor)
                        },
                    ),
                    (
                        "lugar",
                        if fecha_suceso.is_some() {
                            sede.and_then(|l| c.clave_lugar(l))
                        } else {
                            c.clave_lugar(ctx.lugar)
                        },
                    ),
                ] {
                    funcion(scope, &contexto, nombre, 0, 0, true, move |lua, _| {
                        texto(lua, s)
                    })?;
                }
                funcion(scope, &contexto, "objetivo", 0, 0, true, move |lua, _| {
                    let Some(o) = ctx.objetivo.filter(|_| fecha_suceso.is_none()) else {
                        return Ok(Value::Nil);
                    };
                    let t = lua.create_table()?;
                    match o {
                        ExtremoRelacion::Colectivo(id) => {
                            t.set("tipo", "Colectivo")?;
                            t.set("clave", c.clave_colectivo(id))?;
                        }
                        ExtremoRelacion::Concepto(id) => {
                            t.set("tipo", "Concepto")?;
                            t.set("clave", c.clave_concepto(id))?;
                        }

                        ExtremoRelacion::Actor(a) => {
                            t.set("tipo", "Actor")?;
                            t.set("clave", c.clave_actor(a))?;
                        }
                        ExtremoRelacion::Lugar(l) => {
                            t.set("tipo", "Lugar")?;
                            t.set("clave", c.clave_lugar(l))?;
                        }
                    }
                    Ok(Value::Table(proxy_solo_lectura(lua, t)?))
                })?;
                funcion(
                    scope,
                    &contexto,
                    "participantes",
                    0,
                    0,
                    true,
                    move |lua, _| {
                        let t = lua.create_sequence_from(
                            ctx.participantes
                                .iter()
                                .filter(|_| fecha_suceso.is_none())
                                .map(|&a| c.clave_actor(a)),
                        )?;
                        Ok(Value::Table(proxy_solo_lectura(lua, t)?))
                    },
                )?;
                funcion(scope, &contexto, "ahora", 0, 0, true, move |lua, _| {
                    let t = lua.create_table()?;
                    t.set("dia_absoluto", ctx.instante.fecha.ordinal())?;
                    t.set("anio", ctx.instante.fecha.anio())?;
                    t.set("mes", ctx.instante.fecha.mes().numero())?;
                    t.set("dia", ctx.instante.fecha.dia())?;
                    t.set("fraccion_numerador", ctx.instante.fraccion.partes())?;
                    t.set("fraccion_denominador", 2_400_000_000u64)?;
                    Ok(Value::Table(proxy_solo_lectura(lua, t)?))
                })?;
                funcion(
                    scope,
                    &contexto,
                    "fecha_programada",
                    0,
                    0,
                    true,
                    move |lua, _| {
                        let Some(f) = fecha_suceso else {
                            return Ok(Value::Nil);
                        };
                        let t = lua.create_table()?;
                        t.set("anio", f.anio())?;
                        t.set("mes", f.mes().numero())?;
                        t.set("dia", f.dia())?;
                        Ok(Value::Table(proxy_solo_lectura(lua, t)?))
                    },
                )?;
                funcion(
                    scope,
                    &contexto,
                    "jugador_presente",
                    0,
                    0,
                    true,
                    move |_, _| Ok(Value::Boolean(presente)),
                )?;
                funcion(
                    scope,
                    &consultas,
                    "actor_jugador",
                    0,
                    0,
                    true,
                    move |lua, _| texto(lua, c.clave_actor(mundo.actor_jugador())),
                )?;
                funcion(
                    scope,
                    &consultas,
                    "colectivo_de",
                    1,
                    1,
                    true,
                    move |lua, v| {
                        texto(
                            lua,
                            mundo
                                .colectivo_de(actor(c, &v[0])?)
                                .and_then(|id| c.clave_colectivo(id)),
                        )
                    },
                )?;
                funcion(
                    scope,
                    &consultas,
                    "ubicacion_colectivo",
                    1,
                    1,
                    true,
                    move |lua, v| {
                        texto(
                            lua,
                            mundo
                                .colectivo(colectivo(c, &v[0])?)
                                .and_then(|x| c.clave_lugar(x.lugar())),
                        )
                    },
                )?;
                funcion(
                    scope,
                    &consultas,
                    "lider_colectivo",
                    1,
                    1,
                    true,
                    move |lua, v| {
                        texto(
                            lua,
                            mundo
                                .colectivo(colectivo(c, &v[0])?)
                                .and_then(|x| x.lider())
                                .and_then(|id| c.clave_actor(id)),
                        )
                    },
                )?;
                funcion(
                    scope,
                    &consultas,
                    "miembros_colectivo",
                    1,
                    1,
                    true,
                    move |lua, v| {
                        let id = colectivo(c, &v[0])?;
                        let t = lua.create_sequence_from(
                            mundo
                                .miembros_colectivo(id)
                                .unwrap_or(&[])
                                .iter()
                                .map(|&a| c.clave_actor(a)),
                        )?;
                        Ok(Value::Table(proxy_solo_lectura(lua, t)?))
                    },
                )?;
                funcion(
                    scope,
                    &consultas,
                    "hay_citas_futuras_del_colectivo",
                    1,
                    1,
                    true,
                    move |_, v| Ok(Value::Boolean(futuras.contains(&colectivo(c, &v[0])?))),
                )?;
                funcion(scope, &consultas, "actor_tiene", 2, 2, true, move |_, v| {
                    let id = actor(c, &v[0])?;
                    let s = string(&v[1])?;
                    Caracteristica::nueva(s.clone()).map_err(err)?;
                    Ok(Value::Boolean(mundo.actor(id).is_some_and(|a| {
                        a.caracteristicas().any(|x| x.como_str() == s)
                    })))
                })?;
                funcion(scope, &consultas, "lugar_tiene", 2, 2, true, move |_, v| {
                    let id = lugar(c, &v[0])?;
                    let s = string(&v[1])?;
                    Caracteristica::nueva(s.clone()).map_err(err)?;
                    Ok(Value::Boolean(mundo.lugar(id).is_some_and(|a| {
                        a.caracteristicas().any(|x| x.como_str() == s)
                    })))
                })?;
                funcion(
                    scope,
                    &consultas,
                    "ubicacion_actor",
                    1,
                    1,
                    true,
                    move |lua, v| {
                        texto(
                            lua,
                            mundo
                                .ubicacion_actor(actor(c, &v[0])?)
                                .and_then(|l| c.clave_lugar(l)),
                        )
                    },
                )?;
                funcion(scope, &consultas, "en_grupo", 1, 1, true, move |_, v| {
                    Ok(Value::Boolean(mundo.grupo().contiene(actor(c, &v[0])?)))
                })?;
                for (nombre, tipo) in [("booleana", 0), ("entera", 1), ("texto", 2)] {
                    funcion(scope, &consultas, nombre, 2, 2, true, move |lua, v| {
                        let k = ClaveVariableGlobal::nueva(string(&v[0])?).map_err(err)?;
                        let defecto = match tipo {
                            0 => ValorVariableGlobal::Booleana(booleano(&v[1])?),
                            1 => ValorVariableGlobal::Entera(entero(&v[1])?),
                            _ => ValorVariableGlobal::Texto(string(&v[1])?),
                        };
                        let encontrado = mundo.variable_global(&k).unwrap_or(&defecto);
                        if encontrado.tipo() != defecto.tipo() {
                            return Err(err("tipo global incompatible"));
                        }
                        valor(lua, encontrado)
                    })?;
                }
                funcion(
                    scope,
                    &consultas,
                    "hay_relacion",
                    3,
                    4,
                    true,
                    move |_, v| {
                        let tipo = string(&v[0])?;
                        let tipos = [
                            ("Desea", TipoRelacion::Desea),
                            ("Debe", TipoRelacion::Debe),
                            ("Necesita", TipoRelacion::Necesita),
                            ("EsLlamadoA", TipoRelacion::EsLlamadoA),
                            ("Gobierna", TipoRelacion::Gobierna),
                            ("EnemigoDe", TipoRelacion::EnemigoDe),
                            ("PactoCon", TipoRelacion::PactoCon),
                            ("PrisioneroDe", TipoRelacion::PrisioneroDe),
                            ("SalvadoPor", TipoRelacion::SalvadoPor),
                            ("TratosCon", TipoRelacion::TratosCon),
                            ("ComerciaCon", TipoRelacion::ComerciaCon),
                            ("Padre", TipoRelacion::Padre),
                            ("Madre", TipoRelacion::Madre),
                            ("EsclavoDe", TipoRelacion::EsclavoDe),
                            ("Sirviente", TipoRelacion::Sirviente),
                            ("MatadoPor", TipoRelacion::MatadoPor),
                            ("Conoce", TipoRelacion::Conoce),
                            ("Conectado", TipoRelacion::Conectado),
                            ("Abarca", TipoRelacion::Abarca),
                            ("OrigenDe", TipoRelacion::OrigenDe),
                            ("AfluenteDe", TipoRelacion::AfluenteDe),
                            ("Bloquea", TipoRelacion::Bloquea),
                            ("Vende", TipoRelacion::Vende),
                            ("Compra", TipoRelacion::Vende),
                        ];
                        let tipo_r = tipos
                            .into_iter()
                            .find(|(n, _)| *n == tipo)
                            .ok_or_else(|| err("tipo de relación desconocido"))?
                            .1;
                        let extremo = |v: &Value| -> mlua::Result<ExtremoRelacion> {
                            let Value::Table(t) = v else {
                                return Err(err("extremo requiere tabla"));
                            };
                            campos(t, &["tipo", "clave"])?;
                            let k = t.get::<Value>("clave")?;
                            match campo(t, "tipo")?.as_str() {
                                "Concepto" => c
                                    .concepto_por_clave(&string(&k)?)
                                    .map(ExtremoRelacion::Concepto)
                                    .ok_or_else(|| err("concepto desconocido")),
                                "Colectivo" => Ok(ExtremoRelacion::Colectivo(colectivo(c, &k)?)),
                                "Actor" => Ok(ExtremoRelacion::Actor(actor(c, &k)?)),
                                "Lugar" => Ok(ExtremoRelacion::Lugar(lugar(c, &k)?)),
                                _ => Err(err("extremo inválido")),
                            }
                        };
                        let mut a = extremo(&v[1])?;
                        let mut b = extremo(&v[2])?;
                        if tipo == "Compra" {
                            std::mem::swap(&mut a, &mut b);
                        }
                        let mercancia = v.get(3).map(string).transpose()?;
                        if (tipo_r == TipoRelacion::Vende) != mercancia.is_some() {
                            return Err(err("mercancía incompatible con tipo de relación"));
                        }
                        let valido = tipo_r.admite_extremos(a, b);
                        if !valido {
                            return Err(err("extremos incompatibles"));
                        }
                        Ok(Value::Boolean(
                            mundo
                                .relaciones_de(a)
                                .into_iter()
                                .flatten()
                                .filter_map(|&id| mundo.relacion(id))
                                .any(|r| {
                                    r.tipo() == tipo_r
                                        && r.mercancia() == mercancia.as_deref()
                                        && ((r.origen() == a && r.destino() == b)
                                            || (tipo_r.es_simetrica()
                                                && r.origen() == b
                                                && r.destino() == a))
                                }),
                        ))
                    },
                )?;
                if matches!(tipo, TipoPrograma::Presentacion) {
                    let ui = lua.create_table()?;
                    funcion(scope, &ui, "narrar", 1, 1, false, move |lua, v| {
                        let texto = validar_texto_presentacion(&string(&v[0])?, true, false)
                            .map_err(err)?;
                        Ok(Value::UserData(
                            lua.create_userdata(Momento { actor: None, texto })?,
                        ))
                    })?;
                    funcion(scope, &ui, "decir", 2, 2, false, move |lua, v| {
                        let texto = validar_texto_presentacion(&string(&v[1])?, true, false)
                            .map_err(err)?;
                        Ok(Value::UserData(lua.create_userdata(Momento {
                            actor: Some(actor(c, &v[0])?),
                            texto,
                        })?))
                    })?;
                    env.set("ui", proxy_solo_lectura(&lua, ui)?)?;
                }
                env.set("ctx", proxy_solo_lectura(&lua, contexto)?)?;
                env.set("mundo", proxy_solo_lectura(&lua, consultas)?)?;
                if matches!(tipo, TipoPrograma::Efectos) {
                    let ops = lua.create_table()?;
                    funcion(scope, &ops, "esclavizar", 2, 2, false, move |lua, v| {
                        Ok(Value::UserData(lua.create_userdata(Operacion(
                            OperacionMundo::RegistrarEsclavitud {
                                actor: actor(c, &v[0])?,
                                amo: actor(c, &v[1])?,
                            },
                        ))?))
                    })?;
                    for (nombre, quitar) in [
                        ("anadir_caracteristica", false),
                        ("quitar_caracteristica", true),
                    ] {
                        funcion(scope, &ops, nombre, 2, 2, false, move |lua, v| {
                            let actor = actor(c, &v[0])?;
                            let caracteristica =
                                Caracteristica::nueva(string(&v[1])?).map_err(err)?;
                            let op = if quitar {
                                OperacionMundo::QuitarCaracteristicaActor {
                                    actor,
                                    caracteristica,
                                }
                            } else {
                                OperacionMundo::AnadirCaracteristicaActor {
                                    actor,
                                    caracteristica,
                                }
                            };
                            Ok(Value::UserData(lua.create_userdata(Operacion(op))?))
                        })?;
                    }
                    funcion(
                        scope,
                        &ops,
                        "registrar_conocimiento",
                        2,
                        2,
                        false,
                        move |lua, v| {
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::RegistrarConocimiento {
                                    actor: actor(c, &v[0])?,
                                    lugar: lugar(c, &v[1])?,
                                },
                            ))?))
                        },
                    )?;
                    for (nombre, tipo) in [
                        ("establecer_booleana", 0),
                        ("establecer_entera", 1),
                        ("establecer_texto", 2),
                    ] {
                        funcion(scope, &ops, nombre, 2, 2, false, move |lua, v| {
                            let clave = ClaveVariableGlobal::nueva(string(&v[0])?).map_err(err)?;
                            let valor = match tipo {
                                0 => ValorVariableGlobal::Booleana(booleano(&v[1])?),
                                1 => ValorVariableGlobal::Entera(entero(&v[1])?),
                                _ => ValorVariableGlobal::Texto(string(&v[1])?),
                            };
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::EstablecerGlobal { clave, valor },
                            ))?))
                        })?;
                    }
                    for (nombre, ingreso) in [
                        ("ingresar_grupo_en_colectivo", true),
                        ("salir_grupo_de_colectivo", false),
                    ] {
                        funcion(scope, &ops, nombre, 1, 1, false, move |lua, v| {
                            let colectivo = colectivo(c, &v[0])?;
                            let op = if ingreso {
                                OperacionMundo::IngresarGrupoEnColectivo { colectivo }
                            } else {
                                OperacionMundo::SalirGrupoDeColectivo { colectivo }
                            };
                            Ok(Value::UserData(lua.create_userdata(Operacion(op))?))
                        })?;
                    }
                    for (nombre, ingreso) in [
                        ("ingresar_actor_en_colectivo", true),
                        ("salir_actor_de_colectivo", false),
                    ] {
                        funcion(scope, &ops, nombre, 2, 2, false, move |lua, v| {
                            let actor = actor(c, &v[0])?;
                            let colectivo = colectivo(c, &v[1])?;
                            let op = if ingreso {
                                OperacionMundo::IngresarActorEnColectivo { actor, colectivo }
                            } else {
                                OperacionMundo::SalirActorDeColectivo { actor, colectivo }
                            };
                            Ok(Value::UserData(lua.create_userdata(Operacion(op))?))
                        })?;
                    }
                    funcion(
                        scope,
                        &ops,
                        "trasladar_colectivo",
                        2,
                        2,
                        false,
                        move |lua, v| {
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::TrasladarColectivo {
                                    colectivo: colectivo(c, &v[0])?,
                                    destino: lugar(c, &v[1])?,
                                },
                            ))?))
                        },
                    )?;
                    funcion(
                        scope,
                        &ops,
                        "establecer_lider_colectivo",
                        2,
                        2,
                        false,
                        move |lua, v| {
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::EstablecerLiderColectivo {
                                    colectivo: colectivo(c, &v[0])?,
                                    lider: if matches!(v[1], Value::Nil) {
                                        None
                                    } else {
                                        Some(actor(c, &v[1])?)
                                    },
                                },
                            ))?))
                        },
                    )?;
                    funcion(scope, &ops, "trasladar_npc", 2, 2, false, move |lua, v| {
                        let actor = actor(c, &v[0])?;
                        if actor == mundo.actor_jugador() || mundo.grupo().contiene(actor) {
                            return Err(err("trasladar_npc no acepta al Grupo del jugador"));
                        }
                        Ok(Value::UserData(lua.create_userdata(Operacion(
                            OperacionMundo::Trasladar {
                                actor,
                                destino: lugar(c, &v[1])?,
                            },
                        ))?))
                    })?;
                    funcion(
                        scope,
                        &ops,
                        "incorporar_companero",
                        1,
                        1,
                        false,
                        move |lua, v| {
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::IncorporarCompanero {
                                    actor: actor(c, &v[0])?,
                                },
                            ))?))
                        },
                    )?;
                    funcion(
                        scope,
                        &ops,
                        "separar_companero",
                        2,
                        2,
                        false,
                        move |lua, v| {
                            Ok(Value::UserData(lua.create_userdata(Operacion(
                                OperacionMundo::SepararCompanero {
                                    actor: actor(c, &v[0])?,
                                    lugar: lugar(c, &v[1])?,
                                },
                            ))?))
                        },
                    )?;
                    env.set("op", proxy_solo_lectura(&lua, ops)?)?;
                }
                let f = lua
                    .load(&self.compilado)
                    .set_environment(env)
                    .into_function()?;
                let mut valores: MultiValue = f.call(())?;
                if valores.len() != 1 {
                    return Err(err("se requiere exactamente un retorno"));
                }
                let v = valores.pop_front().ok_or_else(|| err("retorno ausente"))?;
                if matches!(tipo, TipoPrograma::Condicion) {
                    return Ok(Respuesta::Condicion(booleano(&v)?));
                }
                let Value::Table(t) = v else {
                    return Err(err("se requiere tabla de retorno"));
                };
                match tipo {
                    TipoPrograma::Presentacion => {
                        campos(&t, &["titulo", "texto", "detalle", "momentos"])?;
                        let titulo = validar_texto_presentacion(&campo(&t, "titulo")?, false, true)
                            .map_err(err)?;
                        let texto_original = t.get::<Value>("texto")?;
                        let secuencia = t.get::<Value>("momentos")?;
                        let momentos = match (texto_original, secuencia) {
                            (Value::Nil, Value::Table(lista_momentos)) => {
                                let mut momentos = Vec::new();
                                for (indice, v) in lista(lista_momentos)?.into_iter().enumerate() {
                                    let Value::UserData(u) = v else {
                                        return Err(err(format!(
                                            "momento {} no autorizado",
                                            indice + 1
                                        )));
                                    };
                                    momentos.push(
                                        u.borrow::<Momento>()
                                            .map_err(|_| {
                                                err(format!("momento {} no autorizado", indice + 1))
                                            })?
                                            .clone(),
                                    );
                                }
                                if momentos.is_empty() {
                                    return Err(err("momentos no puede estar vacío"));
                                }
                                momentos
                            }
                            (v, Value::Nil) if !matches!(v, Value::Nil) => vec![Momento {
                                actor: None,
                                texto: validar_texto_presentacion(&string(&v)?, true, true)
                                    .map_err(err)?,
                            }],
                            _ => {
                                return Err(err(
                                    "debe especificarse exactamente uno de texto o momentos",
                                ));
                            }
                        };
                        let detalle = match t.get::<Value>("detalle")? {
                            Value::Nil => None,
                            v => Some(
                                validar_texto_presentacion(&string(&v)?, true, false)
                                    .map_err(err)?,
                            ),
                        };
                        Ok(Respuesta::Presentacion(Presentacion {
                            momentos,
                            titulo,
                            detalle,
                        }))
                    }
                    TipoPrograma::Efectos => {
                        campos(
                            &t,
                            &[
                                "operaciones",
                                "siguiente",
                                "solicitar",
                                "acciones_colectivas",
                            ],
                        )?;
                        let mut operaciones = vec![];
                        for v in lista(t.get("operaciones")?)? {
                            let Value::UserData(u) = v else {
                                return Err(err("operación no autorizada"));
                            };
                            operaciones.push(u.borrow::<Operacion>()?.0.clone());
                        }
                        let solicitar = match t.get::<Value>("solicitar")? {
                            Value::Nil => vec![],
                            Value::Table(t) => lista(t)?
                                .iter()
                                .map(string)
                                .collect::<mlua::Result<Vec<_>>>()?,
                            _ => return Err(err("solicitar requiere lista")),
                        };
                        let acciones_colectivas = match t.get::<Value>("acciones_colectivas")? {
                            Value::Nil => vec![],
                            Value::Table(t) => lista(t)?
                                .iter()
                                .map(|v| colectivo(c, v))
                                .collect::<mlua::Result<Vec<_>>>()?,
                            _ => return Err(err("acciones_colectivas requiere lista")),
                        };
                        Ok(Respuesta::Efectos(Efectos {
                            acciones_colectivas,
                            operaciones,
                            siguiente: campo(&t, "siguiente")?,
                            solicitar,
                        }))
                    }
                    TipoPrograma::Condicion => Err(err("retorno inválido")),
                }
            })
        };
        run().map_err(|e| format!("{}: {e}", self.nombre))
    }
}

#[cfg(test)]
mod pruebas_relaciones_personales {
    use super::*;
    use desert_tiger_core::{FraccionDia, InstanteAccion, acciones::intervenciones::ClasePunto};

    #[test]
    fn dialogos_lua_valida_secuencia_completa_y_constructores() {
        let contenido = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = contenido.instanciar_estado().unwrap();
        let ctx = ContextoIntervencion {
            clase: ClasePunto::Disponible,
            actor: mundo.actor_jugador(),
            accion: None,
            objetivo: None,
            lugar: mundo.ubicacion_grupo(),
            participantes: vec![mundo.actor_jugador()],
            instante: InstanteAccion {
                fecha: mundo.fecha(),
                fraccion: FraccionDia::CERO,
            },
        };
        for (fuente, valido) in [
            (
                r#"return {titulo="Escena",momentos={ui.narrar("Llegás al puerto."),ui.decir("sirviente","Vamos."),ui.decir("zahrim","Bienvenidos.")}}"#,
                true,
            ),
            (r#"return {titulo="Escena",texto="Legado"}"#, true),
            (r#"return {titulo="Escena",momentos={}}"#, false),
            (
                r#"return {titulo="Escena",texto="Otro",momentos={ui.narrar("Relato")}}"#,
                false,
            ),
            (
                r#"return {titulo="Escena",momentos={ui.narrar("Relato"),ui.decir("no_existe","Hola")}}"#,
                false,
            ),
            (
                r#"return {titulo="Escena",momentos={[1]=ui.narrar("Uno"),[3]=ui.narrar("Tres")}}"#,
                false,
            ),
            (
                r#"return {titulo="Escena",momentos={{actor="sirviente",texto="Sin constructor"}}}"#,
                false,
            ),
        ] {
            let mut programa: Programa =
                ron::from_str(&ron::ser::to_string(fuente).unwrap()).unwrap();
            programa.compilar("dialogos_prueba").unwrap();
            let resultado =
                programa.ejecutar(TipoPrograma::Presentacion, &mundo, contenido.claves(), &ctx);
            assert_eq!(resultado.is_ok(), valido, "{fuente}");
        }
    }

    #[test]
    fn guiones_consultan_conceptos_y_pactos_sin_inferir_cumplimiento() {
        let contenido = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = contenido.instanciar_estado().unwrap();
        let antes = mundo.clone();
        let ctx = ContextoIntervencion {
            clase: ClasePunto::Disponible,
            actor: mundo.actor_jugador(),
            accion: None,
            objetivo: None,
            lugar: mundo.ubicacion_grupo(),
            participantes: vec![mundo.actor_jugador()],
            instante: InstanteAccion {
                fecha: mundo.fecha(),
                fraccion: FraccionDia::CERO,
            },
        };
        for (fuente, esperado) in [
            (
                r#"return mundo:hay_relacion("Desea", {tipo="Actor", clave="zahrim"}, {tipo="Concepto", clave="controlar_salida_mar_del_viento"})"#,
                true,
            ),
            (
                r#"return mundo:hay_relacion("Debe", {tipo="Actor", clave="dahran"}, {tipo="Concepto", clave="proteger_tripulacion"})"#,
                true,
            ),
            (
                r#"return mundo:hay_relacion("PactoCon", {tipo="Actor", clave="miraz"}, {tipo="Actor", clave="sahmun"})"#,
                true,
            ),
            (
                r#"return mundo:hay_relacion("Necesita", {tipo="Actor", clave="farid"}, {tipo="Concepto", clave="abrir_casa_comidas_shahraz"})"#,
                false,
            ),
        ] {
            let mut p: Programa = ron::from_str(&ron::ser::to_string(&fuente).unwrap()).unwrap();
            p.compilar("relaciones_personales").unwrap();
            assert!(
                matches!(p.ejecutar(TipoPrograma::Condicion, &mundo, contenido.claves(), &ctx).unwrap(), Respuesta::Condicion(v) if v == esperado)
            );
        }
        let fuente = r#"return mundo:hay_relacion("Desea", {tipo="Actor", clave="zahrim"}, {tipo="Actor", clave="miraz"})"#;
        let mut p: Programa = ron::from_str(&ron::ser::to_string(&fuente).unwrap()).unwrap();
        p.compilar("extremos_invalidos").unwrap();
        assert!(
            p.ejecutar(TipoPrograma::Condicion, &mundo, contenido.claves(), &ctx)
                .is_err()
        );
        assert_eq!(mundo, antes);
    }
}
