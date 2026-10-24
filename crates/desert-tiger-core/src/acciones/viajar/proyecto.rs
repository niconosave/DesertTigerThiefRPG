use crate::{Duracion, Fecha, IdActor, IdLugar};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstrategiaViaje {
    Directa,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FuenteMedio {
    CapacidadHumana,
    Actor(IdActor),
    Lugar(IdLugar),
    EmbarcacionActual(IdLugar),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MedioTerrestreProyecto {
    APie,
    Camellos,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MedioAcuaticoProyecto {
    Ninguno,
    Bote,
    Barco,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RitmoProyecto {
    Prudente,
    Normal,
    Forzado,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreparacionProyecto {
    Ligera,
    Suficiente,
    Cargada,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropuestaViaje {
    pub referencia: u32,
    pub proponente: IdActor,
    pub estrategia: EstrategiaViaje,
    pub medio_terrestre: MedioTerrestreProyecto,
    pub fuente_terrestre: FuenteMedio,
    pub medio_acuatico: MedioAcuaticoProyecto,
    pub fuente_acuatica: Option<FuenteMedio>,
    pub ruta: Vec<IdLugar>,
    pub dias_previstos: u32,
    pub exposicion_conocida: u32,
    pub hay_incertidumbre: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubobjetivoViaje {
    PrepararPartida,
    Alcanzar(IdLugar),
    SuperarPaso(IdLugar),
    SostenerGrupo(IdLugar),
    EstabilizarEmbarcacion(IdLugar),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanViaje {
    pub propuesta: PropuestaViaje,
    pub ritmo: RitmoProyecto,
    pub preparacion: PreparacionProyecto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProyectoViaje {
    pub dirigente: IdActor,
    pub participantes: Vec<IdActor>,
    pub origen_inicial: IdLugar,
    pub ultimo_lugar_confirmado: IdLugar,
    pub destino_final: IdLugar,
    pub fecha_inicio: Fecha,
    pub propuestas: Vec<PropuestaViaje>,
    pub plan: Option<PlanViaje>,
    pub subobjetivo: Option<SubobjetivoViaje>,
    pub duracion: Duracion,
}

impl ProyectoViaje {
    pub fn nuevo(
        dirigente: IdActor,
        participantes: Vec<IdActor>,
        origen: IdLugar,
        destino: IdLugar,
        fecha_inicio: Fecha,
        propuestas: Vec<PropuestaViaje>,
    ) -> Option<Self> {
        if participantes.first() != Some(&dirigente) || propuestas.is_empty() {
            return None;
        }
        Some(Self {
            dirigente,
            participantes,
            origen_inicial: origen,
            ultimo_lugar_confirmado: origen,
            destino_final: destino,
            fecha_inicio,
            propuestas,
            plan: None,
            subobjetivo: None,
            duracion: Duracion::CERO,
        })
    }

    pub fn comprometer(&mut self, referencia: u32) -> Option<&PlanViaje> {
        let propuesta = self
            .propuestas
            .iter()
            .find(|propuesta| propuesta.referencia == referencia)?
            .clone();
        self.plan = Some(PlanViaje {
            propuesta,
            ritmo: RitmoProyecto::Normal,
            preparacion: PreparacionProyecto::Ligera,
        });
        self.subobjetivo = Some(SubobjetivoViaje::PrepararPartida);
        self.plan.as_ref()
    }
}
