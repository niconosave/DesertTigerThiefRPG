use super::{
    carga::LoteDescarte,
    reservas::{RegistroConsumo, ReservasViaje},
    rutas::Medio,
};
use crate::acciones::ResultadoChequeo;
use crate::{CambioConfirmado, IdActor, IdLugar, InstanteAccion, TiempoAccion};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaveTarjeta {
    SinPlan,
    PropuestaRuta,
    Preparacion,
    CargaInicial,
    Incompleto,
    Andar,
    Caballo,
    Camello,
    Navegar,
    PasoMontanoso,
    Forrajeo,
    Sobrecarga,
    Llegada,
}
impl ClaveTarjeta {
    pub fn clave(self) -> &'static str {
        match self {
            Self::SinPlan => "VIA-RUT-01",
            Self::PropuestaRuta => "VIA-RUT-02",
            Self::Preparacion => "VIA-APR-01",
            Self::CargaInicial => "VIA-APR-05",
            Self::Incompleto => "VIA-APR-03",
            Self::Andar => "VIA-DES-AND",
            Self::Caballo => "VIA-DES-CAB",
            Self::Camello => "VIA-DES-CAM",
            Self::Navegar => "VIA-DES-NAV",
            Self::PasoMontanoso => "VIA-DES-PAS",
            Self::Forrajeo => "VIA-APR-02",
            Self::Sobrecarga => "VIA-APR-06",
            Self::Llegada => "VIA-LLE-01",
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Incidente {
    Agotamiento,
    Esguince,
    RumboPerdido,
    Separacion,
    Enfermedad,
    Obstinada,
    Detenida,
    Asustada,
    CrisisMontura,
    MuerteMontura,
    Olas,
    Lluvia,
    SinViento,
    Naufragio,
    PorLaBorda,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncidenteResuelto {
    pub tipo: Incidente,
    pub dado: u8,
    pub lugar: IdLugar,
    pub afectado: Option<IdActor>,
    pub recuperacion: Option<u8>,
    pub monturas_restantes: Option<u64>,
    pub demora: TiempoAccion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MitadRecorrida {
    pub lugar: IdLugar,
    pub medio: Medio,
    pub horas_base: u64,
    pub pequena: bool,
    pub vasta: bool,
    pub terreno_lento: bool,
    pub animal_limitante: bool,
    pub estado_horas: u64,
    pub duracion: TiempoAccion,
    pub demora_carga: TiempoAccion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Desplazamiento {
    pub medio: Medio,
    pub origen: IdLugar,
    pub destino: IdLugar,
    pub chequeos: [ResultadoChequeo; 2],
    pub mitades: Vec<MitadRecorrida>,
    pub incidente: Option<IncidenteResuelto>,
    pub viento: Option<(u8, u8, i16)>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfiguracionVista {
    pub reservas: ReservasViaje,
    pub animales: u64,
    pub exceso_gramos: u64,
    pub peso_actores: u64,
    pub limite_actores: u64,
    pub tiempo: TiempoAccion,
    pub demora: TiempoAccion,
    pub suplemento: ReservasViaje,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContenidoEscena {
    Plan {
        propuestas: Vec<PropuestaRuta>,
    },
    Preparacion {
        chequeo: ResultadoChequeo,
        configuracion: ConfiguracionVista,
    },
    CargaInicial {
        base: ConfiguracionVista,
        alternativa: ConfiguracionVista,
    },
    Desplazamiento(Desplazamiento),
    PasoMontanoso(RegistroPasoMontanoso),
    Forrajeo {
        chequeos: Vec<ResultadoChequeo>,
        obtenido: ReservasViaje,
        busca_comida: bool,
        busca_agua: bool,
    },
    Incompleto {
        inicial: bool,
        falta_comida: bool,
        falta_agua: bool,
    },
    Sobrecarga {
        configuracion: ConfiguracionVista,
        lote: LoteDescarte,
    },
    Llegada {
        lugares: usize,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IdPaso(pub(crate) u64);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistroEscena {
    pub id: IdPaso,
    pub tarjeta: ClaveTarjeta,
    pub contenido: ContenidoEscena,
    pub participantes: Vec<IdActor>,
    pub composicion_resultante: Vec<IdActor>,
    pub estados_iniciales: Vec<(IdActor, Vec<String>)>,
    pub caballos: u64,
    pub camellos: u64,
    pub animales_carga: u64,
    pub lugar: IdLugar,
    pub destino_final: IdLugar,
    pub siguiente: Option<IdLugar>,
    pub plan_visible: bool,
    pub duracion: TiempoAccion,
    pub total: TiempoAccion,
    pub instante: InstanteAccion,
    pub consumo: RegistroConsumo,
    pub horas_desde_racion: TiempoAccion,
    pub cierre: Option<CierreViaje>,
    pub reservas: Option<ReservasViaje>,
    pub cambios: Vec<CambioConfirmado>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpcionViaje {
    ElegirRuta(IdPropuestaRuta),
    Improvisar,
    Cancelar,
    AceptarDemora,
    EvitarDemora,
    Continuar,
    Buscar,
    Descartar,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RespuestaDecision {
    pub paso: IdPaso,
    pub opcion: OpcionViaje,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolicitudDecision {
    pub escena: RegistroEscena,
    pub opciones: Vec<OpcionViaje>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CierreViaje {
    Llegada,
    Cancelado,
    SinRecursos,
    InterrumpidoPorSeparacion,
    Naufragio,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinalizacionAccion {
    pub tiempo: TiempoAccion,
    pub cierre: CierreViaje,
    pub lugar: IdLugar,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PasoAccion {
    Escena(RegistroEscena),
    Decision(SolicitudDecision),
    Finalizada(FinalizacionAccion),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct IdPropuestaRuta(pub(crate) u32);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PerfilRuta {
    Rapida,
    Conocidos,
    Montana,
    Caballo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RespaldoRuta {
    pub actor: IdActor,
    pub perfil: PerfilRuta,
    pub caracteristicas_habilitantes: Vec<crate::Caracteristica>,
    pub lugares_relevantes: Vec<IdLugar>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropuestaRuta {
    pub id: IdPropuestaRuta,
    pub ruta: Vec<IdLugar>,
    pub previsto: TiempoAccion,
    pub respaldos: Vec<RespaldoRuta>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MomentoPaso {
    Panorama,
    CaminoDificil,
    Cornisa,
    Rodeos,
    AbrirPaso,
    Campamento,
    OtraVertiente,
    Interrupcion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistroPasoMontanoso {
    pub momento: MomentoPaso,
    pub origen: IdLugar,
    pub destino: IdLugar,
    pub cantidad_lugares: usize,
    pub lugar_confirmado: IdLugar,
    pub previsto: TiempoAccion,
    pub traslado_acumulado: TiempoAccion,
    pub otras_actividades: TiempoAccion,
    pub chequeo: Option<ResultadoChequeo>,
    pub demora: TiempoAccion,
    pub animales_perdidos: u64,
    pub comida_perdida: [u64; 4],
}
