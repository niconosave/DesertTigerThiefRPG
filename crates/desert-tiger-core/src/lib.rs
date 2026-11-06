//! Reglas y estado reglado de Desert Tiger Thief RPG.

pub mod acciones;
pub mod azar;
mod calendario;
mod modelo;

pub use calendario::{
    Duracion, ErrorCalendario, Fecha, FraccionDia, InstanteAccion, Mes, TiempoAccion,
    proyectar_tiempo,
};
pub use modelo::{
    Actor, ActorInicial, CambioConfirmado, CambiosConfirmados, Caracteristica, ClaveVariableGlobal,
    DatosInicialesMundo, ErrorCaracteristica, ErrorClaveVariableGlobal, ErrorConstruirMundo,
    ErrorConsultaRelacion, ErrorConsultaVariableGlobal, ErrorIdentificador, ErrorOperacionMundo,
    ErrorTraslado, ExtremoRelacion, Grupo, IdActor, IdLugar, IdMapa, IdRelacion, LoteOperaciones,
    Lugar, LugarInicial, MAX_BYTES_CARACTERISTICA, MAX_BYTES_CLAVE_VARIABLE_GLOBAL, Mundo,
    MundoPreparado, OperacionMundo, Relacion, RelacionInicial, TipoRelacion, TipoVariableGlobal,
    TrasladoConfirmado, ValorVariableGlobal, VariableGlobalInicial,
};

pub use modelo::{HermanoDerivado, Progenitores, UbicacionActor};
