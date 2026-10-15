//! Composición de partidas y fachada independiente del frontend.

mod contenido;
mod fase_npc;
mod lua;
mod sesion;

pub use contenido::{
    ClaseDiagnosticoContenido, ClaveTipoTile, ContenidoValidado, CoordenadaMapa,
    DiagnosticoContenido, ElementoDiagnosticoContenido, ErrorClaveTipoTile, ErrorContenido,
    FotogramaRetrato, OrigenDiagnosticoContenido, PosicionDiagnosticoContenido, RetratoActor,
    TipoElementoContenido, cargar_contenido,
};
pub use desert_tiger_core::acciones::{ClaveEscena, RechazoAccion};
pub use desert_tiger_core::{IdActor, IdLugar};
pub use fase_npc::{
    CodigoFalloPoliticaNpc, DiagnosticoPoliticaNpc, EntradaResumenNpc, EtapaFalloPoliticaNpc,
    ResumenNpc,
};
pub use sesion::{
    AccionDisponible, CausaIndisponibilidad, CodigoErrorCrearPartida, CodigoFatalSesion,
    ConfiguracionPartida, ErrorCrearPartida, ErrorIntencion, IdDecisionVista, IdFragmento,
    IdOpcionVista, IntencionFrontend, Partida, VistaActor, VistaDecision, VistaFatal,
    VistaFragmento, VistaLugar, VistaMapa, VistaMundo, VistaOpcionDecision, VistaSesion,
};
