//! Composición de partidas y fachada independiente del frontend.

mod contenido;
mod contexto_viaje;
mod fase_npc;
mod fichas;
pub use desert_tiger_core::acciones::ErrorAcampada;
pub use desert_tiger_core::{ExtremoRelacion, IdRelacion, TipoRelacion};
pub use fichas::{
    EntradaFicha, ErrorConsultaFicha, ResumenElemento, SentidoRelacionFicha, VistaFicha,
    VistaRelacionFicha,
};
mod lua;
mod narracion_viaje;
mod sesion;
mod tiempo;
mod vision_mapa;
pub use vision_mapa::VisibilidadTile;

pub use contenido::{
    ArteLugar, CeldaRetrato, ClaseDiagnosticoContenido, ClaveTipoTile, ColorRetrato,
    ContenidoValidado, CoordenadaMapa, DiagnosticoContenido, ElementoDiagnosticoContenido,
    ErrorClaveTipoTile, ErrorContenido, EstiloCeldaRetrato, GraficoRetrato,
    OrigenDiagnosticoContenido, ParpadeoRetrato, PosicionDiagnosticoContenido, PosicionRetrato,
    RetratoActor, RostroRetrato, TipoElementoContenido, cargar_contenido, cargar_retrato,
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

pub use sesion::{IdCampamento, IntegranteCampamento, VistaCampamento};

pub use desert_tiger_core::{HermanoDerivado, Progenitores, UbicacionActor};
pub use fichas::{VinculoFicha, VistaUbicacionActor};
