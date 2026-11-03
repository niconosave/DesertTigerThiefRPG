use super::*;

#[derive(Clone)]
pub(super) enum Fase {
    Plan,
    ElegirRuta,
    Preparar,
    ElegirCarga(Preparacion),
    ConfirmarPreparacion(ResultadoChequeo, Configuracion),
    Aviso { inicial: bool },
    ElegirAviso { inicial: bool },
    Mover,
    CerrarMovimiento(Box<tarjetas::Movimiento>),
    ElegirSeparacion(Box<tarjetas::Movimiento>),
    Entre { revisar: bool, busco: bool },
    Forrajear,
    Descartar(LoteDescarte),
    Llegar,
    Terminar(CierreViaje),
    Finalizada,
}
#[derive(Clone)]
pub(super) struct Preparacion {
    pub(super) chequeo: ResultadoChequeo,
    pub(super) base: Configuracion,
    pub(super) alternativa: Configuracion,
}
pub(super) struct EfectosEscena {
    pub(super) participantes: Vec<IdActor>,
    pub(super) operaciones: Vec<OperacionMundo>,
}
#[derive(Clone)]
pub struct EjecucionAccion {
    pub(super) dirigente: IdActor,
    pub(super) viajeros: Vec<IdActor>,
    pub(super) destino: IdLugar,
    pub(super) lugar: IdLugar,
    pub(super) ruta: Vec<IdLugar>,
    pub(super) propuestas: Vec<PropuestaRuta>,
    pub(super) larga: Vec<IdLugar>,
    pub(super) posicion: usize,
    pub(super) inicio: InstanteAccion,
    pub(super) rumbos: CatalogoRumbos,
    pub(super) total: HorasViaje,
    pub(super) h: HorasViaje,
    pub(super) carencia_comida: Option<HorasViaje>,
    pub(super) carencia_agua: Option<HorasViaje>,
    pub(super) reservas: Option<ReservasViaje>,
    pub(super) caballos: u64,
    pub(super) camellos: u64,
    pub(super) animales: u64,
    pub(super) demoras: Vec<[HorasViaje; 2]>,
    pub(super) visible: bool,
    pub(super) fase: Fase,
    pub(super) ordinal: u64,
    pub(super) espera: Option<(IdPaso, Vec<OpcionViaje>)>,
}
pub struct InicioAccion {
    pub ejecucion: EjecucionAccion,
    pub paso: PasoAccion,
}
