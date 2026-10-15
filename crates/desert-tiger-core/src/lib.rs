//! Reglas y estado reglado de Desert Tiger Thief RPG.

pub mod acciones;
pub mod azar;
mod calendario;
mod modelo;

pub use calendario::{Duracion, ErrorCalendario, Fecha, Mes};
pub use modelo::{
    Actor, ActorInicial, Caracteristica, ClaveVariableGlobal, DatosInicialesMundo,
    ErrorCaracteristica, ErrorClaveVariableGlobal, ErrorConstruirMundo,
    ErrorConsultaVariableGlobal, ErrorIdentificador, ErrorTraslado, Grupo, IdActor, IdLugar,
    IdMapa, Lugar, LugarInicial, MAX_BYTES_CARACTERISTICA, MAX_BYTES_CLAVE_VARIABLE_GLOBAL, Mundo,
    TipoVariableGlobal, TrasladoConfirmado, ValorVariableGlobal, VariableGlobalInicial,
};
