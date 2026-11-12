use super::{
    contenido::{Catalogo, Control, Definicion, Modo, Observacion, Paso},
    lua::{Respuesta, TipoPrograma},
};
use crate::contenido::CatalogoClaves;
use desert_tiger_core::acciones::intervenciones::ContextoIntervencion;
use desert_tiger_core::{Fecha, IdColectivo, IdLugar, InstanteAccion, Mundo};
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
    pub hablante: Option<desert_tiger_core::IdActor>,
    pub momento: usize,
    pub total_momentos: usize,
    pendientes: VecDeque<super::lua::Momento>,
    opciones_finales: Vec<OpcionEvento>,
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
    pub fecha_suceso: Option<Fecha>,
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
    agenda: bool,
    candidatos: VecDeque<String>,
    observado: bool,
    fecha_suceso: Option<Fecha>,
    pub actividades: BTreeSet<IdColectivo>,
    pub posiciones_observadas: Vec<IdLugar>,
    futuras: Vec<IdColectivo>,
    indice_punto: usize,
}
pub enum Salida {
    Pendiente,
    Vista(Box<VistaEvento>),
    Fin { omitir: bool, interrumpir: bool },
}
fn observar(e: &Definicion, m: &Mundo, c: &CatalogoClaves) -> bool {
    match &e.observacion {
        Observacion::DelPunto | Observacion::Predeterminada => true,
        Observacion::Silenciosa => false,
        Observacion::Local { lugar } => c.lugar_por_clave(lugar) == Some(m.ubicacion_grupo()),
        Observacion::EnColectivo { colectivo } => {
            c.colectivo_por_clave(colectivo) == m.colectivo_de(m.actor_jugador())
        }
    }
}
fn sede(e: &Definicion, m: &Mundo, c: &CatalogoClaves) -> Option<IdLugar> {
    match &e.observacion {
        Observacion::Local { lugar } => c.lugar_por_clave(lugar),
        Observacion::EnColectivo { colectivo } => c
            .colectivo_por_clave(colectivo)
            .and_then(|id| m.colectivo(id))
            .map(|c| c.lugar()),
        _ => None,
    }
}
fn coincide(
    e: &Definicion,
    m: &Mundo,
    c: &CatalogoClaves,
    ctx: &ContextoIntervencion,
    fecha: Option<Fecha>,
    futuras: &[IdColectivo],
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
    match e.condicion.ejecutar_con_agenda(
        TipoPrograma::Condicion,
        m,
        c,
        ctx,
        (fecha, futuras, sede(e, m, c), observar(e, m, c)),
    )? {
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
        self.agenda = false;
        self.indice_punto = 0;
        self.sustituir = false;
        self.interrumpir = false;
        Ok(())
    }
    pub fn abrir_fechas(
        &mut self,
        ctx: ContextoIntervencion,
        inicio: InstanteAccion,
        inclusiva: bool,
        cat: &Catalogo,
    ) -> Result<(), String> {
        self.abrir(ctx.clone())?;
        self.agenda = true;
        self.candidatos = cat
            .fechas
            .range(inicio.fecha..=ctx.instante.fecha)
            .filter(|(f, _)| inclusiva || **f > inicio.fecha)
            .flat_map(|(_, ids)| ids.iter().cloned())
            .collect();
        // Las citas ocurren a fracción cero; ninguna de la fecha inicial vuelve
        // a pertenecer a un intervalo abierto por la izquierda.
        self.candidatos.retain(|k| !self.terminados.contains(k));
        Ok(())
    }
    pub fn futuras(
        &self,
        cat: &Catalogo,
        c: &CatalogoClaves,
        ahora: InstanteAccion,
    ) -> Vec<IdColectivo> {
        cat.fechas
            .range((
                std::ops::Bound::Excluded(ahora.fecha),
                std::ops::Bound::Unbounded,
            ))
            .flat_map(|(_, ids)| ids)
            .filter(|k| !self.terminados.contains(*k))
            .filter_map(|k| match &cat.eventos[k].observacion {
                Observacion::EnColectivo { colectivo } => c.colectivo_por_clave(colectivo),
                _ => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
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
        if !v.pendientes.is_empty() {
            if opcion.is_some() {
                return Err("opciones disponibles solo al final de la viñeta".into());
            }
            let mut vista = v.clone();
            let momento = vista.pendientes.pop_front().ok_or("momento ausente")?;
            let ordinal = self.ordinal.checked_add(1).ok_or("IDs agotados")?;
            vista.id = IdPresentacionEvento(ordinal);
            vista.momento += 1;
            vista.hablante = momento.actor;
            vista.texto = momento.texto;
            if vista.pendientes.is_empty() {
                vista.opciones = std::mem::take(&mut vista.opciones_finales);
            }
            self.ordinal = ordinal;
            self.vista = Some(vista);
            return Ok(());
        }
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
        if !self.observado {
            return Ok(None);
        }
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
        let Respuesta::Presentacion(p) = presentacion.ejecutar_con_agenda(
            TipoPrograma::Presentacion,
            m,
            c,
            ctx,
            (
                self.fecha_suceso,
                &self.futuras,
                sede(e, m, c),
                self.observado,
            ),
        )?
        else {
            return Err("presentación inválida".into());
        };
        let mut momentos: VecDeque<_> = p.momentos.into();
        let total_momentos = momentos.len();
        let primero = momentos.pop_front().ok_or("presentación vacía")?;
        let opciones_finales: Vec<_> = opciones
            .iter()
            .enumerate()
            .map(|(i, o)| OpcionEvento {
                id: IdOpcionEvento(i),
                clave: o.clave.clone(),
                texto: o.texto.clone(),
            })
            .collect();
        Ok(Some(VistaEvento {
            hablante: primero.actor,
            momento: 0,
            total_momentos,
            opciones: if momentos.is_empty() {
                opciones_finales.clone()
            } else {
                vec![]
            },
            pendientes: momentos,
            opciones_finales,
            id: IdPresentacionEvento(self.ordinal.checked_add(1).ok_or("IDs agotados")?),
            evento: e.clave.clone(),
            paso: paso.into(),
            titulo: p.titulo,
            texto: primero.texto,
            detalle: p.detalle,
            ilustracion: ilustracion.clone(),
            expresion: expresion.clone(),
            lugar: m.ubicacion_grupo(),
            fecha: ctx.instante.fecha,
            fecha_suceso: self.fecha_suceso,
        }))
    }
    pub fn conducir(
        &mut self,
        mundo: &mut Mundo,
        c: &CatalogoClaves,
        cat: &Catalogo,
        validar: impl Fn(&Mundo, bool) -> Result<(), String>,
    ) -> Result<Salida, String> {
        self.futuras = self.futuras(
            cat,
            c,
            self.contexto.as_ref().ok_or("sin contexto")?.instante,
        );
        for _ in 0..64 {
            // Cada paso confirma conjuntamente reserva, cola y efectos. Un fallo
            // posterior conserva los pasos ya confirmados, nunca medio paso.
            let mut siguiente = self.clone();
            let salida = siguiente.paso(mundo, c, cat, &validar)?;
            *self = siguiente;
            if let Some(salida) = salida {
                return Ok(salida);
            }
        }
        Ok(Salida::Pendiente)
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
            let solicitado = !self.agenda && !self.cola.is_empty();
            let candidato = if self.agenda {
                self.candidatos.pop_front()
            } else if let Some(k) = self.cola.pop_front() {
                Some(k)
            } else {
                let k = cat
                    .indice
                    .get(&ctx.clase)
                    .and_then(|ids| ids.get(self.indice_punto))
                    .cloned();
                self.indice_punto += usize::from(k.is_some());
                k
            };
            let Some(k) = candidato else {
                let salida = Salida::Fin {
                    omitir: self.sustituir,
                    interrumpir: self.interrumpir,
                };
                self.contexto = None;
                return Ok(Some(salida));
            };
            let e = &cat.eventos[&k];
            if self.terminados.contains(&k)
                || (self.sustituir && e.modo == Modo::SustituirRelato)
                || (!self.agenda && self.interrumpir && !solicitado)
            {
                return Ok(None);
            }
            if !self.agenda && !e.permite(ctx.clase) {
                return Err("control incompatible".into());
            }
            self.fecha_suceso = e.fecha;
            if !coincide(e, mundo, c, ctx, e.fecha, &self.futuras)? {
                if self.agenda {
                    self.terminados.insert(k);
                }
                return Ok(None);
            }
            self.observado = observar(e, mundo, c);
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
            Paso::Vigneta { siguiente, .. } if !self.observado => {
                self.activo = Some((
                    k,
                    siguiente
                        .clone()
                        .ok_or("evento remoto requiere siguiente")?,
                ));
            }
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
                let Respuesta::Efectos(r) = resolver.ejecutar_con_agenda(
                    TipoPrograma::Efectos,
                    mundo,
                    c,
                    ctx,
                    (
                        self.fecha_suceso,
                        &self.futuras,
                        sede(e, mundo, c),
                        self.observado,
                    ),
                )?
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
                let actividades_traslado = r
                    .operaciones
                    .iter()
                    .filter_map(|op| match op {
                        desert_tiger_core::OperacionMundo::TrasladarColectivo {
                            colectivo,
                            destino,
                        } if mundo
                            .colectivo(*colectivo)
                            .is_some_and(|x| x.lugar() != *destino) =>
                        {
                            Some(*colectivo)
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let preparado = mundo
                    .preparar_operaciones(r.operaciones)
                    .map_err(|err| format!("{}:{p}: {err}", e.archivo.display()))?;
                validar(preparado.mundo(), cerrar)?;
                let vista = self.preparar_vista(e, &r.siguiente, preparado.mundo(), c)?;
                let posicion_anterior = mundo.ubicacion_grupo();
                preparado.confirmar(mundo);
                if mundo.ubicacion_grupo() != posicion_anterior {
                    self.posiciones_observadas.push(mundo.ubicacion_grupo());
                }
                self.actividades.extend(actividades_traslado);
                self.actividades.extend(r.acciones_colectivas);
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
                if !self.agenda {
                    self.indice_punto = 0;
                }
            }
        }
        Ok(None)
    }
}
