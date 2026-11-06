use super::{
    contenido::{Catalogo, Control, Definicion, Modo, Paso},
    lua::{Respuesta, TipoPrograma},
};
use crate::contenido::CatalogoClaves;
use desert_tiger_core::acciones::intervenciones::ContextoIntervencion;
use desert_tiger_core::{Fecha, IdLugar, Mundo};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdPresentacionEvento(u64);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpcionEvento {
    pub id: IdOpcionEvento,
    pub clave: String,
    pub texto: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdOpcionEvento(usize);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VistaEvento {
    pub id: IdPresentacionEvento,
    pub evento: String,
    pub paso: String,
    pub titulo: String,
    pub texto: String,
    pub detalle: Option<String>,
    pub ilustracion: Option<String>,
    pub expresion: Option<String>,
    pub opciones: Vec<OpcionEvento>,
    pub lugar: IdLugar,
    pub fecha: Fecha,
}
#[derive(Clone, Default)]
pub struct Coordinador {
    terminados: BTreeSet<String>,
    activo: Option<(String, String)>,
    cola: VecDeque<String>,
    pub contexto: Option<ContextoIntervencion>,
    pub vista: Option<VistaEvento>,
    ordinal: u64,
    sustituir: bool,
    interrumpir: bool,
}
pub enum Salida {
    Vista(Box<VistaEvento>),
    Fin { omitir: bool, interrumpir: bool },
}
fn coincide(
    e: &Definicion,
    m: &Mundo,
    c: &CatalogoClaves,
    ctx: &ContextoIntervencion,
) -> Result<bool, String> {
    if e.filtros
        .actor
        .as_ref()
        .is_some_and(|s| c.actor_por_clave(s) != Some(ctx.actor))
        || e.filtros
            .lugar
            .as_ref()
            .is_some_and(|s| c.lugar_por_clave(s) != Some(ctx.lugar))
    {
        return Ok(false);
    }
    if let Some(o) = &e.filtros.objetivo
        && Some(o.resolver(c)?) != ctx.objetivo
    {
        return Ok(false);
    }
    match e.condicion.ejecutar(TipoPrograma::Condicion, m, c, ctx)? {
        Respuesta::Condicion(b) => Ok(b),
        _ => Err("retorno condición inválido".into()),
    }
}
impl Coordinador {
    pub fn abrir(&mut self, ctx: ContextoIntervencion) -> Result<(), String> {
        if self.contexto.is_some() {
            return Err("punto narrativo ya abierto".into());
        }
        self.contexto = Some(ctx);
        self.sustituir = false;
        self.interrumpir = false;
        Ok(())
    }
    pub fn responder(
        &mut self,
        id: IdPresentacionEvento,
        opcion: Option<IdOpcionEvento>,
        cat: &Catalogo,
    ) -> Result<(), String> {
        let v = self
            .vista
            .as_ref()
            .filter(|v| v.id == id)
            .ok_or("presentación de evento vencida")?;
        let e = cat.eventos.get(&v.evento).ok_or("evento ausente")?;
        let Paso::Vigneta {
            opciones,
            siguiente,
            ..
        } = e.paso(&v.paso)?
        else {
            return Err("paso no es viñeta".into());
        };
        let siguiente = if opciones.is_empty() {
            if opcion.is_some() {
                return Err("viñeta sin opciones".into());
            }
            siguiente.clone().ok_or("destino ausente")?
        } else {
            opcion
                .and_then(|i| opciones.get(i.0))
                .ok_or("opción de evento inválida")?
                .siguiente
                .clone()
        };
        self.activo = Some((v.evento.clone(), siguiente));
        self.vista = None;
        Ok(())
    }
    fn preparar_vista(
        &self,
        e: &Definicion,
        paso: &str,
        m: &Mundo,
        c: &CatalogoClaves,
    ) -> Result<Option<VistaEvento>, String> {
        let Paso::Vigneta {
            presentacion,
            opciones,
            ilustracion,
            expresion,
            ..
        } = e.paso(paso)?
        else {
            return Ok(None);
        };
        let ctx = self.contexto.as_ref().ok_or("contexto ausente")?;
        let Respuesta::Presentacion(p) =
            presentacion.ejecutar(TipoPrograma::Presentacion, m, c, ctx)?
        else {
            return Err("presentación inválida".into());
        };
        Ok(Some(VistaEvento {
            id: IdPresentacionEvento(self.ordinal.checked_add(1).ok_or("IDs agotados")?),
            evento: e.clave.clone(),
            paso: paso.into(),
            titulo: p.titulo,
            texto: p.texto,
            detalle: p.detalle,
            ilustracion: ilustracion.clone(),
            expresion: expresion.clone(),
            opciones: opciones
                .iter()
                .enumerate()
                .map(|(i, o)| OpcionEvento {
                    id: IdOpcionEvento(i),
                    clave: o.clave.clone(),
                    texto: o.texto.clone(),
                })
                .collect(),
            lugar: ctx.lugar,
            fecha: ctx.instante.fecha,
        }))
    }
    pub fn conducir(
        &mut self,
        mundo: &mut Mundo,
        c: &CatalogoClaves,
        cat: &Catalogo,
        validar: impl Fn(&Mundo, bool) -> Result<(), String>,
    ) -> Result<Salida, String> {
        loop {
            // Cada paso confirma conjuntamente reserva, cola y efectos. Un fallo
            // posterior conserva los pasos ya confirmados, nunca medio paso.
            let mut siguiente = self.clone();
            let salida = siguiente.paso(mundo, c, cat, &validar)?;
            *self = siguiente;
            if let Some(salida) = salida {
                return Ok(salida);
            }
        }
    }
    fn paso(
        &mut self,
        mundo: &mut Mundo,
        c: &CatalogoClaves,
        cat: &Catalogo,
        validar: &impl Fn(&Mundo, bool) -> Result<(), String>,
    ) -> Result<Option<Salida>, String> {
        if let Some(v) = &self.vista {
            return Ok(Some(Salida::Vista(Box::new(v.clone()))));
        }
        let ctx = self.contexto.as_ref().ok_or("sin punto narrativo")?;
        if self.activo.is_none() {
            let mut elegido = None;
            while let Some(k) = self.cola.pop_front() {
                let e = &cat.eventos[&k];
                if self.terminados.contains(&k)
                    || (self.sustituir && e.modo == Modo::SustituirRelato)
                {
                    continue;
                }
                if !e.permite(ctx.clase) {
                    return Err(format!(
                        "{}: control incompatible con {}",
                        k,
                        ctx.clase.clave()
                    ));
                }
                if coincide(e, mundo, c, ctx)? {
                    elegido = Some(k);
                    break;
                }
            }
            if elegido.is_none() && !self.interrumpir {
                for k in cat.indice.get(&ctx.clase).into_iter().flatten() {
                    let e = &cat.eventos[k];
                    if !self.terminados.contains(k)
                        && !(self.sustituir && e.modo == Modo::SustituirRelato)
                        && coincide(e, mundo, c, ctx)?
                    {
                        elegido = Some(k.clone());
                        break;
                    }
                }
            }
            let Some(k) = elegido else {
                let salida = Salida::Fin {
                    omitir: self.sustituir,
                    interrumpir: self.interrumpir,
                };
                self.contexto = None;
                return Ok(Some(salida));
            };
            let e = &cat.eventos[&k];
            let vista = self.preparar_vista(e, &e.inicio, mundo, c)?;
            if e.modo == Modo::SustituirRelato {
                self.sustituir = true;
            }
            self.activo = Some((k, e.inicio.clone()));
            if let Some(v) = vista {
                self.ordinal += 1;
                self.vista = Some(v);
            }
            if self.vista.is_some() {
                return Ok(None);
            }
        }
        let (k, p) = self.activo.clone().ok_or("evento ausente")?;
        let e = &cat.eventos[&k];
        match e.paso(&p)? {
            Paso::Vigneta { .. } => {
                let v = self
                    .preparar_vista(e, &p, mundo, c)?
                    .ok_or("vista ausente")?;
                self.ordinal += 1;
                self.vista = Some(v);
            }
            Paso::Efectos {
                resolver, salidas, ..
            } => {
                let Respuesta::Efectos(r) =
                    resolver.ejecutar(TipoPrograma::Efectos, mundo, c, ctx)?
                else {
                    return Err("efectos inválidos".into());
                };
                if !salidas.contains(&r.siguiente) {
                    return Err(format!("{}:{p}: salida no declarada", k));
                }
                for s in &r.solicitar {
                    if !e.encadena.contains(s) {
                        return Err(format!("solicitud no declarada {s}"));
                    }
                    if !cat.eventos[s].permite(ctx.clase) {
                        return Err(format!("evento {s} no permite punto {}", ctx.clase.clave()));
                    }
                }
                let cerrar = self.interrumpir
                    || matches!(
                        e.paso(&r.siguiente)?,
                        Paso::Fin {
                            control: Control::InterrumpirAccion,
                            ..
                        }
                    );
                let preparado = mundo
                    .preparar_operaciones(r.operaciones)
                    .map_err(|err| format!("{}:{p}: {err}", e.archivo.display()))?;
                validar(preparado.mundo(), cerrar)?;
                let vista = self.preparar_vista(e, &r.siguiente, preparado.mundo(), c)?;
                preparado.confirmar(mundo);
                self.activo = Some((k, r.siguiente));
                for s in r.solicitar {
                    if !self.terminados.contains(&s) && s != e.clave && !self.cola.contains(&s) {
                        self.cola.push_back(s);
                    }
                }
                if let Some(v) = vista {
                    self.ordinal += 1;
                    self.vista = Some(v);
                }
            }
            Paso::Fin { control, .. } => {
                self.interrumpir |= *control == Control::InterrumpirAccion;
                self.terminados.insert(k);
                self.activo = None;
            }
        }
        Ok(None)
    }
}
