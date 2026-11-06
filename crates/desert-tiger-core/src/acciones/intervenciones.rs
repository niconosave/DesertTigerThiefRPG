//! Adaptador reanudable: conserva la API de viaje anterior para consumidores sin eventos.
use super::*;
use crate::{ExtremoRelacion, TiempoAccion};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClasePunto {
    Disponible,
    ViajarInicio,
    EntreLugares,
    RelatoLlegada,
    AcamparInicio,
    AccionTerminada,
    FaseNpcTerminada,
}
impl ClasePunto {
    pub fn clave(self) -> &'static str {
        match self {
            Self::Disponible => "partida.disponible",
            Self::ViajarInicio => "viajar.inicio",
            Self::EntreLugares => "viajar.entre_lugares",
            Self::RelatoLlegada => "viajar.relato_llegada",
            Self::AcamparInicio => "acampar.inicio",
            Self::AccionTerminada => "accion.terminada",
            Self::FaseNpcTerminada => "fase_npc.terminada",
        }
    }
    pub fn desde_clave(s: &str) -> Option<Self> {
        [
            Self::Disponible,
            Self::ViajarInicio,
            Self::EntreLugares,
            Self::RelatoLlegada,
            Self::AcamparInicio,
            Self::AccionTerminada,
            Self::FaseNpcTerminada,
        ]
        .into_iter()
        .find(|p| p.clave() == s)
    }
    pub fn permite_interrumpir(self) -> bool {
        matches!(self, Self::ViajarInicio | Self::EntreLugares)
    }
}
#[derive(Clone, Debug)]
pub struct ContextoIntervencion {
    pub clase: ClasePunto,
    pub actor: IdActor,
    pub accion: Option<&'static str>,
    pub objetivo: Option<ExtremoRelacion>,
    pub lugar: IdLugar,
    pub participantes: Vec<IdActor>,
    pub instante: InstanteAccion,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdIntervencion(u64);
#[derive(Clone, Debug)]
pub struct PuntoIntervencion {
    pub id: IdIntervencion,
    pub contexto: ContextoIntervencion,
}
#[derive(Clone, Copy, Debug)]
pub enum ResolucionPunto {
    EjecutarGenerico,
    OmitirRelato,
    Interrumpir,
}
pub enum SalidaMotor {
    Punto(PuntoIntervencion),
    Paso(Box<PasoAccion>),
}
enum Continuacion {
    Inicio,
    Avanzar(IdPaso),
    Publicar(Box<PasoAccion>),
}
pub struct EjecucionIntervenida {
    solicitud: SolicitudAccion,
    interior: Option<EjecucionAccion>,
    contexto: ContextoAccionPropio,
    pendiente: Option<(IdIntervencion, Continuacion)>,
    ordinal: u64,
    ultimo: Option<RegistroEscena>,
    lugar: IdLugar,
    participantes_iniciales: Vec<IdActor>,
}
struct ContextoAccionPropio {
    inicio: InstanteAccion,
    rumbos: CatalogoRumbos,
}
impl EjecucionIntervenida {
    fn validar_contexto(&self, c: &ContextoAccion<'_>) -> Resultado<()> {
        if self.contexto.inicio != c.inicio || &self.contexto.rumbos != c.rumbos {
            return Err(fallo("contexto intervenido modificado"));
        }
        Ok(())
    }

    fn punto(&mut self, clase: ClasePunto, continuar: Continuacion) -> Resultado<SalidaMotor> {
        self.ordinal = self
            .ordinal
            .checked_add(1)
            .ok_or_else(|| fallo("token de intervención agotado"))?;
        let id = IdIntervencion(self.ordinal);
        self.pendiente = Some((id, continuar));
        Ok(SalidaMotor::Punto(PuntoIntervencion {
            id,
            contexto: self.contexto(clase),
        }))
    }
    pub fn contexto(&self, clase: ClasePunto) -> ContextoIntervencion {
        let SolicitudAccion::Viajar { actor, destino } = self.solicitud;
        ContextoIntervencion {
            clase,
            actor,
            accion: Some("viajar"),
            objetivo: Some(ExtremoRelacion::Lugar(destino)),
            lugar: self.lugar,
            participantes: self
                .ultimo
                .as_ref()
                .map(|r| r.composicion_resultante.clone())
                .unwrap_or_else(|| self.participantes_iniciales.clone()),
            instante: self
                .ultimo
                .as_ref()
                .map(|r| r.instante)
                .unwrap_or(self.contexto.inicio),
        }
    }
    pub fn validar_continuacion(&self, mundo: &Mundo) -> Resultado<()> {
        if self
            .ultimo
            .as_ref()
            .is_some_and(|r| r.tarjeta == ClaveTarjeta::Llegada)
        {
            return Ok(());
        }
        let SolicitudAccion::Viajar { actor, .. } = self.solicitud;
        if mundo.ubicacion_actor(actor) != Some(self.lugar) {
            return Err(fallo("ubicación de continuación modificada"));
        }
        for id in std::iter::once(actor).chain(mundo.grupo().integrantes().iter().copied()) {
            if mundo
                .actor(id)
                .is_none_or(|a| a.caracteristicas().any(|c| c.como_str() == "muerto"))
            {
                return Err(fallo("participante muerto en continuación"));
            }
        }
        Ok(())
    }
    fn aceptar(&mut self, paso: PasoAccion) -> Resultado<SalidaMotor> {
        if let PasoAccion::Escena(r) | PasoAccion::Decision(SolicitudDecision { escena: r, .. }) =
            &paso
        {
            self.lugar = r.lugar;
            self.ultimo = Some(r.clone());
        }
        if matches!(&paso,PasoAccion::Escena(r) if r.tarjeta==ClaveTarjeta::Llegada) {
            self.punto(
                ClasePunto::RelatoLlegada,
                Continuacion::Publicar(Box::new(paso)),
            )
        } else {
            Ok(SalidaMotor::Paso(Box::new(paso)))
        }
    }
}
pub fn iniciar(
    mundo: &Mundo,
    solicitud: SolicitudAccion,
    c: &ContextoAccion<'_>,
) -> Resultado<(EjecucionIntervenida, SalidaMotor)> {
    let SolicitudAccion::Viajar { actor, destino } = solicitud;
    if mundo.ubicacion_actor(actor) == Some(destino)
        || mundo.lugar(destino).is_none()
        || (actor == mundo.actor_jugador() && super::lugar_oculto(mundo, destino))
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::DestinoInvalido));
    }
    if !destinos_disponibles(mundo, actor)
        .map_err(ErrorMotor::Rechazo)?
        .contains(&destino)
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::SinRuta));
    }
    let lugar = mundo
        .ubicacion_actor(actor)
        .ok_or_else(|| fallo("actor sin ubicación"))?;
    let mut e = EjecucionIntervenida {
        solicitud,
        interior: None,
        contexto: ContextoAccionPropio {
            inicio: c.inicio,
            rumbos: c.rumbos.clone(),
        },
        pendiente: None,
        ordinal: 0,
        ultimo: None,
        lugar,
        participantes_iniciales: std::iter::once(actor)
            .chain(mundo.grupo().integrantes().iter().copied())
            .collect(),
    };
    let salida = e.punto(ClasePunto::ViajarInicio, Continuacion::Inicio)?;
    Ok((e, salida))
}
pub fn avanzar(
    mundo: &mut Mundo,
    e: &mut EjecucionIntervenida,
    paso: IdPaso,
    c: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<SalidaMotor> {
    e.validar_contexto(c)?;
    e.interior
        .as_ref()
        .ok_or_else(|| fallo("viaje ausente"))?
        .validar_avance_intervenido(paso, c)?;
    if e.pendiente.is_some() || e.ultimo.as_ref().is_none_or(|r| r.id != paso) {
        return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente));
    }
    let r = e.ultimo.as_ref().ok_or_else(|| fallo("escena ausente"))?;
    let movio = r
        .cambios
        .iter()
        .any(|c| matches!(c, crate::CambioConfirmado::ActoresTrasladados { .. }));
    if movio && r.cierre.is_none() {
        return e.punto(ClasePunto::EntreLugares, Continuacion::Avanzar(paso));
    }
    let p = super::avanzar(
        mundo,
        e.interior.as_mut().ok_or_else(|| fallo("viaje ausente"))?,
        paso,
        c,
        azar,
    )?;
    e.aceptar(p)
}
pub fn responder(
    mundo: &mut Mundo,
    e: &mut EjecucionIntervenida,
    r: RespuestaDecision,
    c: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<SalidaMotor> {
    e.validar_contexto(c)?;
    if e.pendiente.is_some() {
        return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente));
    }
    let p = super::responder(
        mundo,
        e.interior.as_mut().ok_or_else(|| fallo("viaje ausente"))?,
        r,
        c,
        azar,
    )?;
    e.aceptar(p)
}
pub fn resolver_punto(
    mundo: &mut Mundo,
    e: &mut EjecucionIntervenida,
    id: IdIntervencion,
    resolucion: ResolucionPunto,
    c: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<SalidaMotor> {
    e.validar_contexto(c)?;
    if !e
        .pendiente
        .as_ref()
        .is_some_and(|(actual, _)| *actual == id)
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente));
    }
    let clase = match e.pendiente.as_ref().map(|(_, p)| p) {
        Some(Continuacion::Inicio) => ClasePunto::ViajarInicio,
        Some(Continuacion::Avanzar(_)) => ClasePunto::EntreLugares,
        _ => ClasePunto::RelatoLlegada,
    };
    if matches!(resolucion, ResolucionPunto::Interrumpir) {
        if !clase.permite_interrumpir() {
            return Err(fallo("punto no interrumpible"));
        }
        e.pendiente = None;
        let tiempo = match &e.ultimo {
            Some(r) => r.total,
            None => TiempoAccion {
                duracion: crate::Duracion::desde_dias(0),
                fraccion: crate::FraccionDia::CERO,
            },
        };
        return Ok(SalidaMotor::Paso(Box::new(PasoAccion::Finalizada(
            FinalizacionAccion {
                tiempo,
                cierre: CierreViaje::InterrumpidoPorEvento,
                lugar: e.lugar,
            },
        ))));
    }
    if matches!(resolucion, ResolucionPunto::OmitirRelato) && clase != ClasePunto::RelatoLlegada {
        return Err(fallo("sustitución no autorizada"));
    }
    let (_, continuar) = e.pendiente.take().ok_or_else(|| fallo("punto ausente"))?;
    let p = match continuar {
        Continuacion::Inicio => {
            let inicio = super::iniciar(mundo, e.solicitud, c, azar)?;
            e.interior = Some(inicio.ejecucion);
            inicio.paso
        }
        Continuacion::Avanzar(paso) => super::avanzar(
            mundo,
            e.interior.as_mut().ok_or_else(|| fallo("viaje ausente"))?,
            paso,
            c,
            azar,
        )?,
        Continuacion::Publicar(p) => {
            if matches!(resolucion, ResolucionPunto::OmitirRelato) {
                let PasoAccion::Escena(r) = *p else {
                    return Err(fallo("relato inválido"));
                };
                return avanzar(mundo, e, r.id, c, azar);
            } else {
                return Ok(SalidaMotor::Paso(p));
            }
        }
    };
    e.aceptar(p)
}
