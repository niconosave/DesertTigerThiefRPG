//! Motor semántico: una escena o decisión por paso, sin presentación ni reloj real.
use crate::azar::FuenteAzar;
use crate::{IdActor, IdLugar, InstanteAccion, Mundo};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt,
    num::NonZeroUsize,
};
mod acampar;
mod particulares;
pub use acampar::{
    Acampada, ErrorAcampada, ResultadoAcampada, finalizar_acampada, iniciar_acampada,
};
mod viajar;
pub use particulares::chequeos::{
    AccionParticular, ModificadorAplicado, ResultadoChequeo, evaluar_capacidad, resolver_chequeo,
};
pub use viajar::registros::*;
pub use viajar::reservas::{RegistroConsumo, ReservasViaje};
pub use viajar::rutas::Medio;
pub use viajar::{EjecucionAccion, InicioAccion, avanzar, iniciar, responder};

pub type Resultado<T> = Result<T, ErrorMotor>;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolicitudAccion {
    Viajar { actor: IdActor, destino: IdLugar },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RechazoAccion {
    ActorDesconocido,
    JugadorMuerto,
    DestinoInvalido,
    SinRuta,
    IntegranteDelGrupo,
    PasoNoVigente,
    OpcionNoVigente,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FalloMotor {
    Invariante(&'static str),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorMotor {
    Rechazo(RechazoAccion),
    Fatal(FalloMotor),
}
impl fmt::Display for ErrorMotor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ErrorMotor {}
impl fmt::Display for RechazoAccion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for RechazoAccion {}
pub(crate) fn fallo(causa: &'static str) -> ErrorMotor {
    ErrorMotor::Fatal(FalloMotor::Invariante(causa))
}
pub(crate) fn sortear(azar: &mut dyn FuenteAzar, n: usize) -> Resultado<usize> {
    let limite = NonZeroUsize::new(n).ok_or_else(|| fallo("sorteo sin candidatos"))?;
    if n == 1 {
        return Ok(0);
    }
    let i = azar
        .elegir_indice(limite)
        .map_err(|_| fallo("fuente de azar fallida"))?;
    if i >= n {
        Err(fallo("índice aleatorio fuera de rango"))
    } else {
        Ok(i)
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CatalogoRumbos(pub BTreeMap<(IdLugar, IdLugar), u8>);
pub struct ContextoAccion<'a> {
    pub inicio: InstanteAccion,
    pub rumbos: &'a CatalogoRumbos,
}

pub fn destinos_disponibles(mundo: &Mundo, actor: IdActor) -> Result<Vec<IdLugar>, RechazoAccion> {
    let actual = mundo.actor(actor).ok_or(RechazoAccion::ActorDesconocido)?;
    if actual.caracteristicas().any(|c| c.como_str() == "muerto") {
        return Err(RechazoAccion::JugadorMuerto);
    }
    if actor != mundo.actor_jugador() && mundo.grupo().contiene(actor) {
        return Err(RechazoAccion::IntegranteDelGrupo);
    }
    let mut vistos = BTreeSet::from([actual.lugar()]);
    let mut cola = VecDeque::from([actual.lugar()]);
    while let Some(l) = cola.pop_front() {
        for &vecino in mundo.lugares_conectados(l).unwrap_or(&[]) {
            if vistos.insert(vecino) {
                cola.push_back(vecino);
            }
        }
    }
    vistos.remove(&actual.lugar());
    Ok(vistos.into_iter().collect())
}

// Claves de presentación neutrales que el frontend ya asocia a su arte local.
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
