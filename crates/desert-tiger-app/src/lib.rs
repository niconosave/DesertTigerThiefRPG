//! Composición de partidas y fachada independiente del frontend.

mod contenido;
mod fase_npc;
mod lua;
mod sesion;

pub use contenido::{
    CeldaRetrato, ClaseDiagnosticoContenido, ClaveTipoTile, ColorRetrato, ContenidoValidado,
    CoordenadaMapa, DiagnosticoContenido, ElementoDiagnosticoContenido, ErrorClaveTipoTile,
    ErrorContenido, EstiloCeldaRetrato, FotogramaRetrato, OrigenDiagnosticoContenido,
    PosicionDiagnosticoContenido, RetratoActor, TipoElementoContenido, cargar_contenido,
};
pub use desert_tiger_core::acciones::{
    ClaveEscena, RechazoAccion, RegistroProyecto, RolPresentacion,
};
pub use desert_tiger_core::{Duracion, Fecha, IdActor, IdLugar, Mes};
pub use fase_npc::{
    CodigoFalloPoliticaNpc, CronologiaNpc, DiagnosticoPoliticaNpc, EntradaResumenNpc,
    EtapaFalloPoliticaNpc, ResumenNpc,
};
pub use sesion::{
    AccionDisponible, CausaIndisponibilidad, CodigoErrorCrearPartida, CodigoFatalSesion,
    ConfiguracionPartida, ErrorCrearPartida, ErrorIntencion, IdDecisionVista, IdFragmento,
    IdOpcionVista, IntencionFrontend, Partida, VistaActor, VistaDecision, VistaFatal,
    VistaFragmento, VistaLugar, VistaMapa, VistaMundo, VistaOpcionDecision, VistaProgresoViaje,
    VistaSesion,
};
