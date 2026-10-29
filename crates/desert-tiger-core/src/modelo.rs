use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::{Duracion, ErrorCalendario, Fecha};

pub const MAX_BYTES_CARACTERISTICA: usize = 64;
pub const MAX_BYTES_CLAVE_VARIABLE_GLOBAL: usize = 64;

macro_rules! identificador {
    ($nombre:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $nombre(u32);

        impl $nombre {
            pub fn desde_posicion(posicion: usize) -> Result<Self, ErrorIdentificador> {
                u32::try_from(posicion)
                    .map(Self)
                    .map_err(|_| ErrorIdentificador::PosicionFueraDeRango { posicion })
            }
        }
    };
}

identificador!(IdActor);
identificador!(IdLugar);
identificador!(IdMapa);
identificador!(IdRelacion);

impl IdActor {
    pub fn indice(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

impl IdLugar {
    pub fn indice(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

impl IdRelacion {
    fn indice(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorIdentificador {
    PosicionFueraDeRango { posicion: usize },
}

impl Display for ErrorIdentificador {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PosicionFueraDeRango { posicion } => {
                write!(formatter, "posición fuera del rango de u32: {posicion}")
            }
        }
    }
}

impl Error for ErrorIdentificador {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Caracteristica(String);

impl Caracteristica {
    pub fn nueva(valor: String) -> Result<Self, ErrorCaracteristica> {
        if valor.len() > MAX_BYTES_CARACTERISTICA {
            return Err(ErrorCaracteristica::DemasiadoLarga {
                longitud_bytes: valor.len(),
                maximo_bytes: MAX_BYTES_CARACTERISTICA,
            });
        }
        if !es_lower_camel_case(&valor) {
            return Err(ErrorCaracteristica::SintaxisInvalida);
        }
        Ok(Self(valor))
    }

    pub fn como_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCaracteristica {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}

impl Display for ErrorCaracteristica {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SintaxisInvalida => formatter.write_str("sintaxis de Característica inválida"),
            Self::DemasiadoLarga {
                longitud_bytes,
                maximo_bytes,
            } => write!(
                formatter,
                "Característica demasiado larga: {longitud_bytes} bytes; máximo {maximo_bytes}"
            ),
        }
    }
}

impl Error for ErrorCaracteristica {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaveVariableGlobal(String);

impl ClaveVariableGlobal {
    pub fn nueva(valor: String) -> Result<Self, ErrorClaveVariableGlobal> {
        if valor.len() > MAX_BYTES_CLAVE_VARIABLE_GLOBAL {
            return Err(ErrorClaveVariableGlobal::DemasiadoLarga {
                longitud_bytes: valor.len(),
                maximo_bytes: MAX_BYTES_CLAVE_VARIABLE_GLOBAL,
            });
        }
        if !es_lower_camel_case(&valor) {
            return Err(ErrorClaveVariableGlobal::SintaxisInvalida);
        }
        Ok(Self(valor))
    }

    pub fn como_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoVariableGlobal {
    Booleana,
    Entera,
    Texto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValorVariableGlobal {
    Booleana(bool),
    Entera(i64),
    Texto(String),
}

impl ValorVariableGlobal {
    pub fn tipo(&self) -> TipoVariableGlobal {
        match self {
            Self::Booleana(_) => TipoVariableGlobal::Booleana,
            Self::Entera(_) => TipoVariableGlobal::Entera,
            Self::Texto(_) => TipoVariableGlobal::Texto,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableGlobalInicial {
    pub clave: ClaveVariableGlobal,
    pub valor: ValorVariableGlobal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorClaveVariableGlobal {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}

impl Display for ErrorClaveVariableGlobal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SintaxisInvalida => formatter.write_str("sintaxis de clave global inválida"),
            Self::DemasiadoLarga {
                longitud_bytes,
                maximo_bytes,
            } => write!(
                formatter,
                "clave global demasiado larga: {longitud_bytes} bytes; máximo {maximo_bytes}"
            ),
        }
    }
}

impl Error for ErrorClaveVariableGlobal {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConsultaVariableGlobal {
    Inexistente {
        clave: ClaveVariableGlobal,
    },
    TipoIncompatible {
        clave: ClaveVariableGlobal,
        esperado: TipoVariableGlobal,
        encontrado: TipoVariableGlobal,
    },
}

impl Display for ErrorConsultaVariableGlobal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inexistente { clave } => {
                write!(
                    formatter,
                    "variable global inexistente: {}",
                    clave.como_str()
                )
            }
            Self::TipoIncompatible {
                clave,
                esperado,
                encontrado,
            } => write!(
                formatter,
                "tipo incompatible para {}: se esperaba {esperado:?} y se encontró {encontrado:?}",
                clave.como_str()
            ),
        }
    }
}

impl Error for ErrorConsultaVariableGlobal {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LugarInicial {
    pub id: IdLugar,
    pub caracteristicas: Vec<Caracteristica>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorInicial {
    pub id: IdActor,
    pub ubicacion: UbicacionActor,
    pub caracteristicas: Vec<Caracteristica>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UbicacionActor {
    EnLugar(IdLugar),
    FueraDelEscenario,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Progenitores {
    pub padre: Option<IdActor>,
    pub madre: Option<IdActor>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HermanoDerivado {
    pub actor: IdActor,
    pub progenitores_compartidos: Vec<IdActor>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExtremoRelacion {
    Actor(IdActor),
    Lugar(IdLugar),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TipoRelacion {
    Sirviente,
    Padre,
    Madre,
    MatadoPor,
    Conectado,
    Abarca,
    OrigenDe,
    AfluenteDe,
    Bloquea,
    Conoce,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelacionInicial {
    pub tipo: TipoRelacion,
    pub origen: ExtremoRelacion,
    pub destino: ExtremoRelacion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relacion {
    id: IdRelacion,
    tipo: TipoRelacion,
    origen: ExtremoRelacion,
    destino: ExtremoRelacion,
}

impl Relacion {
    pub fn id(&self) -> IdRelacion {
        self.id
    }

    pub fn tipo(&self) -> TipoRelacion {
        self.tipo
    }

    pub fn origen(&self) -> ExtremoRelacion {
        self.origen
    }

    pub fn destino(&self) -> ExtremoRelacion {
        self.destino
    }

    pub fn extremos(&self) -> (ExtremoRelacion, ExtremoRelacion) {
        (self.origen, self.destino)
    }

    pub fn contiene(&self, extremo: ExtremoRelacion) -> bool {
        self.origen == extremo || self.destino == extremo
    }

    pub fn otro_extremo(&self, extremo: ExtremoRelacion) -> Option<ExtremoRelacion> {
        if extremo == self.origen {
            Some(self.destino)
        } else if extremo == self.destino {
            Some(self.origen)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatosInicialesMundo {
    pub mapa: IdMapa,
    pub fecha: Fecha,
    pub lugares: Vec<LugarInicial>,
    pub actores: Vec<ActorInicial>,
    pub actor_jugador: IdActor,
    pub companeros: Vec<IdActor>,
    pub variables_globales: Vec<VariableGlobalInicial>,
    pub relaciones: Vec<RelacionInicial>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Lugar {
    id: IdLugar,
    caracteristicas: BTreeSet<Caracteristica>,
}

impl Lugar {
    pub fn id(&self) -> IdLugar {
        self.id
    }

    pub fn es_region(&self) -> bool {
        self.caracteristicas().any(|c| c.como_str() == "region")
    }

    pub fn tiene_caracteristica(&self, caracteristica: &Caracteristica) -> bool {
        self.caracteristicas.contains(caracteristica)
    }

    pub fn caracteristicas(&self) -> impl ExactSizeIterator<Item = &Caracteristica> + '_ {
        self.caracteristicas.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    id: IdActor,
    ubicacion: UbicacionActor,
    caracteristicas: BTreeSet<Caracteristica>,
}

impl Actor {
    pub fn id(&self) -> IdActor {
        self.id
    }

    pub fn ubicacion(&self) -> UbicacionActor {
        self.ubicacion
    }
    pub fn lugar(&self) -> Option<IdLugar> {
        match self.ubicacion {
            UbicacionActor::EnLugar(id) => Some(id),
            UbicacionActor::FueraDelEscenario => None,
        }
    }

    pub fn tiene_caracteristica(&self, caracteristica: &Caracteristica) -> bool {
        self.caracteristicas.contains(caracteristica)
    }

    pub fn caracteristicas(&self) -> impl ExactSizeIterator<Item = &Caracteristica> + '_ {
        self.caracteristicas.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grupo {
    integrantes: Vec<IdActor>,
}

impl Grupo {
    pub fn integrantes(&self) -> &[IdActor] {
        &self.integrantes
    }

    pub fn contiene(&self, actor: IdActor) -> bool {
        self.integrantes.contains(&actor)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Mundo {
    mapa: IdMapa,
    fecha: Fecha,
    lugares: Vec<Lugar>,
    actores: Vec<Actor>,
    actor_jugador: IdActor,
    grupo: Grupo,
    variables_globales: BTreeMap<ClaveVariableGlobal, ValorVariableGlobal>,
    relaciones: Vec<Relacion>,
    relaciones_por_lugar: Vec<Vec<IdRelacion>>,
    relaciones_por_actor: Vec<Vec<IdRelacion>>,
    conectados_por_lugar: Vec<Vec<IdLugar>>,
    lugares_conocidos_por_actor: Vec<Vec<IdLugar>>,
    progenitores_por_actor: Vec<Progenitores>,
    hijos_por_actor: Vec<Vec<IdActor>>,
}

impl Mundo {
    pub fn progenitores(&self, actor: IdActor) -> Option<Progenitores> {
        self.actor(actor)?;
        self.progenitores_por_actor.get(actor.indice()?).copied()
    }

    pub fn hijos_de(&self, actor: IdActor) -> Option<&[IdActor]> {
        self.actor(actor)?;
        self.hijos_por_actor.get(actor.indice()?).map(Vec::as_slice)
    }

    pub fn hermanos_de(&self, actor: IdActor) -> Option<Vec<HermanoDerivado>> {
        let padres = self.progenitores(actor)?;
        let mut encontrados: BTreeMap<IdActor, BTreeSet<IdActor>> = BTreeMap::new();
        for progenitor in [padres.padre, padres.madre].into_iter().flatten() {
            for &hijo in self.hijos_de(progenitor)? {
                if hijo != actor {
                    encontrados.entry(hijo).or_default().insert(progenitor);
                }
            }
        }
        Some(
            encontrados
                .into_iter()
                .map(|(actor, compartidos)| HermanoDerivado {
                    actor,
                    progenitores_compartidos: compartidos.into_iter().collect(),
                })
                .collect(),
        )
    }

    pub fn nuevo(datos: DatosInicialesMundo) -> Result<Self, ErrorConstruirMundo> {
        if datos.lugares.is_empty() {
            return Err(ErrorConstruirMundo::SinLugares);
        }
        if datos.actores.is_empty() {
            return Err(ErrorConstruirMundo::SinActores);
        }

        let mut lugares = Vec::with_capacity(datos.lugares.len());
        for (posicion, inicial) in datos.lugares.into_iter().enumerate() {
            let esperado = IdLugar::desde_posicion(posicion)
                .map_err(|_| ErrorConstruirMundo::PosicionLugarFueraDeRango { posicion })?;
            if inicial.id != esperado {
                return Err(ErrorConstruirMundo::IdLugarNoCanonico {
                    posicion,
                    esperado,
                    encontrado: inicial.id,
                });
            }
            let caracteristicas =
                conjunto_caracteristicas(inicial.caracteristicas).map_err(|caracteristica| {
                    ErrorConstruirMundo::CaracteristicaLugarDuplicada {
                        lugar: inicial.id,
                        caracteristica,
                    }
                })?;
            lugares.push(Lugar {
                id: inicial.id,
                caracteristicas,
            });
        }

        let mut actores = Vec::with_capacity(datos.actores.len());
        for (posicion, inicial) in datos.actores.into_iter().enumerate() {
            let esperado = IdActor::desde_posicion(posicion)
                .map_err(|_| ErrorConstruirMundo::PosicionActorFueraDeRango { posicion })?;
            if inicial.id != esperado {
                return Err(ErrorConstruirMundo::IdActorNoCanonico {
                    posicion,
                    esperado,
                    encontrado: inicial.id,
                });
            }
            if let UbicacionActor::EnLugar(lugar) = inicial.ubicacion
                && resolver_lugar(&lugares, lugar).is_none()
            {
                return Err(ErrorConstruirMundo::LugarDeActorDesconocido {
                    actor: inicial.id,
                    lugar,
                });
            }
            let caracteristicas =
                conjunto_caracteristicas(inicial.caracteristicas).map_err(|caracteristica| {
                    ErrorConstruirMundo::CaracteristicaActorDuplicada {
                        actor: inicial.id,
                        caracteristica,
                    }
                })?;
            actores.push(Actor {
                id: inicial.id,
                ubicacion: inicial.ubicacion,
                caracteristicas,
            });
        }

        let jugador = resolver_actor(&actores, datos.actor_jugador).ok_or(
            ErrorConstruirMundo::ActorJugadorDesconocido {
                actor: datos.actor_jugador,
            },
        )?;
        let lugar_jugador =
            jugador
                .lugar()
                .ok_or(ErrorConstruirMundo::JugadorFueraDelEscenario {
                    actor: datos.actor_jugador,
                })?;

        if datos.companeros.is_empty() {
            return Err(ErrorConstruirMundo::GrupoVacio);
        }
        let mut vistos = BTreeSet::new();
        for &integrante in &datos.companeros {
            if integrante == datos.actor_jugador {
                return Err(ErrorConstruirMundo::JugadorIncluidoEnGrupo { actor: integrante });
            }
            if !vistos.insert(integrante) {
                return Err(ErrorConstruirMundo::IntegranteGrupoDuplicado { actor: integrante });
            }
            let actor = resolver_actor(&actores, integrante)
                .ok_or(ErrorConstruirMundo::IntegranteGrupoDesconocido { actor: integrante })?;
            let lugar_actor =
                actor
                    .lugar()
                    .ok_or(ErrorConstruirMundo::IntegranteGrupoFueraDelEscenario {
                        actor: integrante,
                    })?;
            if lugar_actor != lugar_jugador {
                return Err(ErrorConstruirMundo::IntegranteGrupoSeparado {
                    actor: integrante,
                    lugar_actor,
                    lugar_jugador,
                });
            }
        }

        let mut variables_globales = BTreeMap::new();
        for variable in datos.variables_globales {
            if variables_globales.contains_key(&variable.clave) {
                return Err(ErrorConstruirMundo::VariableGlobalDuplicada {
                    clave: variable.clave,
                });
            }
            variables_globales.insert(variable.clave, variable.valor);
        }

        let mut relaciones = Vec::with_capacity(datos.relaciones.len());
        let mut relaciones_por_lugar = vec![Vec::new(); lugares.len()];
        let mut relaciones_por_actor = vec![Vec::new(); actores.len()];
        let mut conectados_por_lugar = vec![Vec::new(); lugares.len()];
        let mut lugares_conocidos_por_actor = vec![Vec::new(); actores.len()];
        let mut relaciones_vistas = BTreeMap::new();
        for (posicion, inicial) in datos.relaciones.into_iter().enumerate() {
            let id = IdRelacion::desde_posicion(posicion)
                .map_err(|_| ErrorConstruirMundo::PosicionRelacionFueraDeRango { posicion })?;
            for extremo in [inicial.origen, inicial.destino] {
                let existe = match extremo {
                    ExtremoRelacion::Actor(actor) => resolver_actor(&actores, actor).is_some(),
                    ExtremoRelacion::Lugar(lugar) => resolver_lugar(&lugares, lugar).is_some(),
                };
                if !existe {
                    return Err(ErrorConstruirMundo::ExtremoRelacionDesconocido {
                        posicion,
                        extremo,
                    });
                }
            }
            if inicial.origen == inicial.destino {
                return Err(ErrorConstruirMundo::RelacionReflexiva {
                    posicion,
                    tipo: inicial.tipo,
                    extremo: inicial.origen,
                });
            }
            if inicial.tipo == TipoRelacion::Conectado {
                for extremo in [inicial.origen, inicial.destino] {
                    if let ExtremoRelacion::Lugar(lugar) = extremo
                        && resolver_lugar(&lugares, lugar).is_some_and(Lugar::es_region)
                    {
                        return Err(ErrorConstruirMundo::ConexionConRegion { posicion, lugar });
                    }
                }
            }
            if inicial.tipo == TipoRelacion::Abarca
                && let ExtremoRelacion::Lugar(lugar) = inicial.origen
                && !resolver_lugar(&lugares, lugar).is_some_and(Lugar::es_region)
            {
                return Err(ErrorConstruirMundo::OrigenAbarcaNoEsRegion { posicion, lugar });
            }
            let clave_duplicado = match (inicial.tipo, inicial.origen, inicial.destino) {
                (
                    TipoRelacion::Abarca
                    | TipoRelacion::OrigenDe
                    | TipoRelacion::AfluenteDe
                    | TipoRelacion::Bloquea,
                    ExtremoRelacion::Lugar(_),
                    ExtremoRelacion::Lugar(_),
                ) => (inicial.tipo, inicial.origen, inicial.destino),
                (
                    TipoRelacion::Conectado,
                    ExtremoRelacion::Lugar(origen),
                    ExtremoRelacion::Lugar(destino),
                ) => {
                    if origen == destino {
                        return Err(ErrorConstruirMundo::RelacionReflexiva {
                            posicion,
                            tipo: inicial.tipo,
                            extremo: inicial.origen,
                        });
                    }
                    if origen < destino {
                        (
                            inicial.tipo,
                            ExtremoRelacion::Lugar(origen),
                            ExtremoRelacion::Lugar(destino),
                        )
                    } else {
                        (
                            inicial.tipo,
                            ExtremoRelacion::Lugar(destino),
                            ExtremoRelacion::Lugar(origen),
                        )
                    }
                }
                (
                    TipoRelacion::Conoce,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Lugar(destino),
                ) => (
                    inicial.tipo,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Lugar(destino),
                ),
                (
                    TipoRelacion::Padre
                    | TipoRelacion::Madre
                    | TipoRelacion::MatadoPor
                    | TipoRelacion::Sirviente,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Actor(destino),
                ) => (
                    inicial.tipo,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Actor(destino),
                ),
                _ => {
                    return Err(ErrorConstruirMundo::ExtremosRelacionIncompatibles {
                        posicion,
                        tipo: inicial.tipo,
                        origen: inicial.origen,
                        destino: inicial.destino,
                    });
                }
            };
            if let Some(&primera) = relaciones_vistas.get(&clave_duplicado) {
                return Err(ErrorConstruirMundo::RelacionDuplicada { posicion, primera });
            }
            relaciones_vistas.insert(clave_duplicado, id);

            match (inicial.tipo, inicial.origen, inicial.destino) {
                (
                    TipoRelacion::Abarca
                    | TipoRelacion::OrigenDe
                    | TipoRelacion::AfluenteDe
                    | TipoRelacion::Bloquea,
                    ExtremoRelacion::Lugar(origen),
                    ExtremoRelacion::Lugar(destino),
                ) => {
                    relaciones_por_lugar[origen.indice().expect("Lugar validado")].push(id);
                    relaciones_por_lugar[destino.indice().expect("Lugar validado")].push(id);
                }
                (
                    TipoRelacion::Conectado,
                    ExtremoRelacion::Lugar(origen),
                    ExtremoRelacion::Lugar(destino),
                ) => {
                    relaciones_por_lugar[origen.indice().expect("Lugar validado")].push(id);
                    relaciones_por_lugar[destino.indice().expect("Lugar validado")].push(id);
                    conectados_por_lugar[origen.indice().expect("Lugar validado")].push(destino);
                    conectados_por_lugar[destino.indice().expect("Lugar validado")].push(origen);
                }
                (
                    TipoRelacion::Conoce,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Lugar(destino),
                ) => {
                    relaciones_por_actor[origen.indice().expect("Actor validado")].push(id);
                    relaciones_por_lugar[destino.indice().expect("Lugar validado")].push(id);
                    lugares_conocidos_por_actor[origen.indice().expect("Actor validado")]
                        .push(destino);
                }
                (
                    TipoRelacion::Padre
                    | TipoRelacion::Madre
                    | TipoRelacion::MatadoPor
                    | TipoRelacion::Sirviente,
                    ExtremoRelacion::Actor(origen),
                    ExtremoRelacion::Actor(destino),
                ) => {
                    relaciones_por_actor[origen.indice().expect("Actor validado")].push(id);
                    relaciones_por_actor[destino.indice().expect("Actor validado")].push(id);
                }
                _ => unreachable!("los extremos ya fueron validados"),
            }
            relaciones.push(Relacion {
                id,
                tipo: inicial.tipo,
                origen: inicial.origen,
                destino: inicial.destino,
            });
        }
        for vecinos in &mut conectados_por_lugar {
            vecinos.sort_unstable();
        }
        for conocidos in &mut lugares_conocidos_por_actor {
            conocidos.sort_unstable();
        }

        let (progenitores_por_actor, hijos_por_actor) =
            construir_parentesco(&actores, &relaciones)?;
        Ok(Self {
            progenitores_por_actor,
            hijos_por_actor,
            mapa: datos.mapa,
            fecha: datos.fecha,
            lugares,
            actores,
            actor_jugador: datos.actor_jugador,
            grupo: Grupo {
                integrantes: datos.companeros,
            },
            variables_globales,
            relaciones,
            relaciones_por_lugar,
            relaciones_por_actor,
            conectados_por_lugar,
            lugares_conocidos_por_actor,
        })
    }

    pub fn id_mapa(&self) -> IdMapa {
        self.mapa
    }

    pub fn actor_jugador(&self) -> IdActor {
        self.actor_jugador
    }

    pub fn grupo(&self) -> &Grupo {
        &self.grupo
    }

    pub fn ubicacion_grupo(&self) -> IdLugar {
        match self.actor(self.actor_jugador) {
            Some(jugador) => match jugador.ubicacion {
                UbicacionActor::EnLugar(lugar) => lugar,
                UbicacionActor::FueraDelEscenario => {
                    unreachable!("el jugador siempre tiene ubicación local validada")
                }
            },
            None => unreachable!("un Mundo válido siempre contiene a su jugador"),
        }
    }

    pub fn actor(&self, id: IdActor) -> Option<&Actor> {
        resolver_actor(&self.actores, id)
    }

    pub fn lugar(&self, id: IdLugar) -> Option<&Lugar> {
        resolver_lugar(&self.lugares, id)
    }

    pub fn ubicacion_actor(&self, id: IdActor) -> Option<IdLugar> {
        self.actor(id).and_then(Actor::lugar)
    }

    pub fn actores(&self) -> &[Actor] {
        &self.actores
    }

    pub fn lugares(&self) -> &[Lugar] {
        &self.lugares
    }

    pub fn relaciones(&self) -> &[Relacion] {
        &self.relaciones
    }

    pub fn relacion(&self, id: IdRelacion) -> Option<&Relacion> {
        self.relaciones
            .get(id.indice()?)
            .filter(|relacion| relacion.id == id)
    }

    pub fn relaciones_de(&self, extremo: ExtremoRelacion) -> Option<&[IdRelacion]> {
        match extremo {
            ExtremoRelacion::Actor(actor) => {
                self.actor(actor)?;
                self.relaciones_por_actor
                    .get(actor.indice()?)
                    .map(Vec::as_slice)
            }
            ExtremoRelacion::Lugar(lugar) => {
                self.lugar(lugar)?;
                self.relaciones_por_lugar
                    .get(lugar.indice()?)
                    .map(Vec::as_slice)
            }
        }
    }

    pub fn lugares_conectados(&self, lugar: IdLugar) -> Option<&[IdLugar]> {
        self.lugar(lugar)?;
        self.conectados_por_lugar
            .get(lugar.indice()?)
            .map(Vec::as_slice)
    }

    pub fn lugares_conocidos(&self, actor: IdActor) -> Option<&[IdLugar]> {
        self.actor(actor)?;
        self.lugares_conocidos_por_actor
            .get(actor.indice()?)
            .map(Vec::as_slice)
    }

    pub fn actor_conoce_lugar(
        &self,
        actor: IdActor,
        lugar: IdLugar,
    ) -> Result<bool, ErrorConsultaRelacion> {
        if self.actor(actor).is_none() {
            return Err(ErrorConsultaRelacion::ActorDesconocido { actor });
        }
        if self.lugar(lugar).is_none() {
            return Err(ErrorConsultaRelacion::LugarDesconocido { lugar });
        }
        Ok(self
            .lugares_conocidos(actor)
            .is_some_and(|lugares| lugares.binary_search(&lugar).is_ok()))
    }

    pub fn lugares_conocidos_por_viajeros(
        &self,
        viajeros: &[IdActor],
    ) -> Result<Vec<IdLugar>, ErrorConsultaRelacion> {
        let mut resultado = BTreeSet::new();
        for &actor in viajeros {
            let lugares = self
                .lugares_conocidos(actor)
                .ok_or(ErrorConsultaRelacion::ActorDesconocido { actor })?;
            resultado.extend(lugares.iter().copied());
        }
        Ok(resultado.into_iter().collect())
    }

    pub fn actores_en_lugar(&self, lugar: IdLugar) -> Option<impl Iterator<Item = &Actor> + '_> {
        self.lugar(lugar)?;
        Some(
            self.actores
                .iter()
                .filter(move |actor| actor.lugar() == Some(lugar)),
        )
    }

    pub fn fecha(&self) -> Fecha {
        self.fecha
    }

    pub fn avanzar_fecha(&mut self, duracion: Duracion) -> Result<Fecha, ErrorCalendario> {
        let nueva = self.fecha.sumar(duracion)?;
        self.fecha = nueva;
        Ok(nueva)
    }

    pub fn variable_global(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<&ValorVariableGlobal, ErrorConsultaVariableGlobal> {
        self.variables_globales
            .get(clave)
            .ok_or_else(|| ErrorConsultaVariableGlobal::Inexistente {
                clave: clave.clone(),
            })
    }

    pub fn variable_booleana(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<bool, ErrorConsultaVariableGlobal> {
        match self.variable_global(clave)? {
            ValorVariableGlobal::Booleana(valor) => Ok(*valor),
            encontrado => Err(error_tipo(clave, TipoVariableGlobal::Booleana, encontrado)),
        }
    }

    pub fn variable_entera(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<i64, ErrorConsultaVariableGlobal> {
        match self.variable_global(clave)? {
            ValorVariableGlobal::Entera(valor) => Ok(*valor),
            encontrado => Err(error_tipo(clave, TipoVariableGlobal::Entera, encontrado)),
        }
    }

    pub fn variable_texto(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<&str, ErrorConsultaVariableGlobal> {
        match self.variable_global(clave)? {
            ValorVariableGlobal::Texto(valor) => Ok(valor),
            encontrado => Err(error_tipo(clave, TipoVariableGlobal::Texto, encontrado)),
        }
    }

    pub fn variables_globales(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ClaveVariableGlobal, &ValorVariableGlobal)> + '_ {
        self.variables_globales.iter()
    }

    pub fn trasladar(
        &mut self,
        actor: IdActor,
        destino: IdLugar,
    ) -> Result<TrasladoConfirmado, ErrorTraslado> {
        let lote = LoteOperaciones::uno(OperacionMundo::Trasladar { actor, destino });
        let cambios = self
            .aplicar_lote(lote)
            .map_err(error_operacion_como_traslado)?;
        let [
            CambioConfirmado::ActoresTrasladados {
                actores,
                origen,
                destino,
            },
        ] = cambios.como_slice()
        else {
            return Err(ErrorTraslado::EstadoInconsistente);
        };
        Ok(TrasladoConfirmado {
            actores: actores.clone(),
            origen: *origen,
            destino: *destino,
        })
    }

    pub fn aplicar_lote(
        &mut self,
        lote: LoteOperaciones,
    ) -> Result<CambiosConfirmados, ErrorOperacionMundo> {
        let mut actores_virtuales = self.actores.clone();
        let mut grupo_virtual = self.grupo.clone();
        let mut relaciones_virtuales = self.relaciones.clone();
        let mut por_actor = self.relaciones_por_actor.clone();
        let mut por_lugar = self.relaciones_por_lugar.clone();
        let mut conocidos = self.lugares_conocidos_por_actor.clone();
        let mut cambios = Vec::with_capacity(lote.operaciones.len());

        for operacion in lote.operaciones {
            match operacion {
                OperacionMundo::Trasladar { actor, destino } => {
                    let traslado = trasladar_en_estado(
                        &mut actores_virtuales,
                        &self.lugares,
                        self.actor_jugador,
                        &grupo_virtual,
                        actor,
                        destino,
                    )?;
                    cambios.push(CambioConfirmado::ActoresTrasladados {
                        actores: traslado.actores,
                        origen: traslado.origen,
                        destino: traslado.destino,
                    });
                }
                OperacionMundo::AnadirCaracteristicaActor {
                    actor,
                    caracteristica,
                } => {
                    let actual = resolver_actor_mut(&mut actores_virtuales, actor)
                        .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
                    if actual.caracteristicas.insert(caracteristica.clone()) {
                        cambios.push(CambioConfirmado::CaracteristicaActorAnadida {
                            actor,
                            caracteristica,
                        });
                    }
                }
                OperacionMundo::QuitarCaracteristicaActor {
                    actor,
                    caracteristica,
                } => {
                    let actual = resolver_actor_mut(&mut actores_virtuales, actor)
                        .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
                    if actual.caracteristicas.remove(&caracteristica) {
                        cambios.push(CambioConfirmado::CaracteristicaActorQuitada {
                            actor,
                            caracteristica,
                        });
                    }
                }
                OperacionMundo::SepararCompanero { actor, lugar } => {
                    if actor == self.actor_jugador || !grupo_virtual.contiene(actor) {
                        return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
                    }
                    if resolver_lugar(&self.lugares, lugar).is_none() {
                        return Err(ErrorOperacionMundo::DestinoDesconocido { destino: lugar });
                    }
                    let actual = resolver_actor_mut(&mut actores_virtuales, actor)
                        .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
                    actual.ubicacion = UbicacionActor::EnLugar(lugar);
                    grupo_virtual.integrantes.retain(|&id| id != actor);
                    cambios.push(CambioConfirmado::CompaneroSeparado { actor, lugar });
                }
                OperacionMundo::RegistrarConocimiento { actor, lugar } => {
                    let ai = actor
                        .indice()
                        .filter(|&i| actores_virtuales.get(i).is_some_and(|a| a.id == actor))
                        .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
                    let li = lugar
                        .indice()
                        .filter(|&i| self.lugares.get(i).is_some_and(|l| l.id == lugar))
                        .ok_or(ErrorOperacionMundo::DestinoDesconocido { destino: lugar })?;
                    if conocidos[ai].binary_search(&lugar).is_err() {
                        let id = IdRelacion::desde_posicion(relaciones_virtuales.len())
                            .map_err(|_| ErrorOperacionMundo::EstadoGrupoInconsistente)?;
                        relaciones_virtuales.push(Relacion {
                            id,
                            tipo: TipoRelacion::Conoce,
                            origen: ExtremoRelacion::Actor(actor),
                            destino: ExtremoRelacion::Lugar(lugar),
                        });
                        por_actor[ai].push(id);
                        por_lugar[li].push(id);
                        conocidos[ai].push(lugar);
                        conocidos[ai].sort_unstable();
                        cambios.push(CambioConfirmado::ConocimientoRegistrado { actor, lugar });
                    }
                }
            }
        }

        self.actores = actores_virtuales;
        self.grupo = grupo_virtual;
        self.relaciones = relaciones_virtuales;
        self.relaciones_por_actor = por_actor;
        self.relaciones_por_lugar = por_lugar;
        self.lugares_conocidos_por_actor = conocidos;
        Ok(CambiosConfirmados { cambios })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrasladoConfirmado {
    pub actores: Vec<IdActor>,
    pub origen: IdLugar,
    pub destino: IdLugar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperacionMundo {
    SepararCompanero {
        actor: IdActor,
        lugar: IdLugar,
    },
    RegistrarConocimiento {
        actor: IdActor,
        lugar: IdLugar,
    },
    Trasladar {
        actor: IdActor,
        destino: IdLugar,
    },
    AnadirCaracteristicaActor {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    QuitarCaracteristicaActor {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoteOperaciones {
    operaciones: Vec<OperacionMundo>,
}

impl LoteOperaciones {
    pub fn nuevo(operaciones: Vec<OperacionMundo>) -> Result<Self, ErrorOperacionMundo> {
        if operaciones.is_empty() {
            return Err(ErrorOperacionMundo::LoteVacio);
        }
        Ok(Self { operaciones })
    }

    pub fn uno(operacion: OperacionMundo) -> Self {
        Self {
            operaciones: vec![operacion],
        }
    }

    pub fn operaciones(&self) -> &[OperacionMundo] {
        &self.operaciones
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CambiosConfirmados {
    cambios: Vec<CambioConfirmado>,
}

impl CambiosConfirmados {
    pub fn como_slice(&self) -> &[CambioConfirmado] {
        &self.cambios
    }

    pub fn en_vec(self) -> Vec<CambioConfirmado> {
        self.cambios
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CambioConfirmado {
    CompaneroSeparado {
        actor: IdActor,
        lugar: IdLugar,
    },
    ConocimientoRegistrado {
        actor: IdActor,
        lugar: IdLugar,
    },
    ActoresTrasladados {
        actores: Vec<IdActor>,
        origen: IdLugar,
        destino: IdLugar,
    },
    CaracteristicaActorAnadida {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    CaracteristicaActorQuitada {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorOperacionMundo {
    ActorFueraDelEscenario { actor: IdActor },
    LoteVacio,
    ActorDesconocido { actor: IdActor },
    DestinoDesconocido { destino: IdLugar },
    DestinoEsLugarActual { lugar: IdLugar },
    IntegranteGrupoNoPuedeTrasladarseSolo { actor: IdActor },
    EstadoGrupoInconsistente,
}

impl Display for ErrorOperacionMundo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo aplicar el lote del Mundo: {self:?}")
    }
}

impl Error for ErrorOperacionMundo {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConstruirMundo {
    JugadorFueraDelEscenario {
        actor: IdActor,
    },
    IntegranteGrupoFueraDelEscenario {
        actor: IdActor,
    },
    ProgenitorMultiplicado {
        actor: IdActor,
        tipo: TipoRelacion,
    },
    ProgenitoresNoDistintos {
        actor: IdActor,
        progenitor: IdActor,
    },
    CicloAscendencia {
        actores: Vec<IdActor>,
    },
    SinLugares,
    SinActores,
    PosicionLugarFueraDeRango {
        posicion: usize,
    },
    PosicionActorFueraDeRango {
        posicion: usize,
    },
    IdLugarNoCanonico {
        posicion: usize,
        esperado: IdLugar,
        encontrado: IdLugar,
    },
    IdActorNoCanonico {
        posicion: usize,
        esperado: IdActor,
        encontrado: IdActor,
    },
    CaracteristicaLugarDuplicada {
        lugar: IdLugar,
        caracteristica: Caracteristica,
    },
    CaracteristicaActorDuplicada {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    LugarDeActorDesconocido {
        actor: IdActor,
        lugar: IdLugar,
    },
    ActorJugadorDesconocido {
        actor: IdActor,
    },
    GrupoVacio,
    JugadorIncluidoEnGrupo {
        actor: IdActor,
    },
    IntegranteGrupoDuplicado {
        actor: IdActor,
    },
    IntegranteGrupoDesconocido {
        actor: IdActor,
    },
    IntegranteGrupoSeparado {
        actor: IdActor,
        lugar_actor: IdLugar,
        lugar_jugador: IdLugar,
    },
    VariableGlobalDuplicada {
        clave: ClaveVariableGlobal,
    },
    PosicionRelacionFueraDeRango {
        posicion: usize,
    },
    ExtremoRelacionDesconocido {
        posicion: usize,
        extremo: ExtremoRelacion,
    },
    ExtremosRelacionIncompatibles {
        posicion: usize,
        tipo: TipoRelacion,
        origen: ExtremoRelacion,
        destino: ExtremoRelacion,
    },
    ConexionConRegion {
        posicion: usize,
        lugar: IdLugar,
    },
    OrigenAbarcaNoEsRegion {
        posicion: usize,
        lugar: IdLugar,
    },
    RelacionReflexiva {
        posicion: usize,
        tipo: TipoRelacion,
        extremo: ExtremoRelacion,
    },
    RelacionDuplicada {
        posicion: usize,
        primera: IdRelacion,
    },
}

impl Display for ErrorConstruirMundo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo construir el Mundo: {self:?}")
    }
}

impl Error for ErrorConstruirMundo {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorTraslado {
    ActorFueraDelEscenario { actor: IdActor },
    ActorDesconocido { actor: IdActor },
    DestinoDesconocido { destino: IdLugar },
    DestinoEsLugarActual { lugar: IdLugar },
    IntegranteGrupoNoPuedeTrasladarseSolo { actor: IdActor },
    EstadoInconsistente,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorConsultaRelacion {
    ActorDesconocido { actor: IdActor },
    LugarDesconocido { lugar: IdLugar },
}

impl Display for ErrorConsultaRelacion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo consultar la Relación: {self:?}")
    }
}

impl Error for ErrorConsultaRelacion {}

impl Display for ErrorTraslado {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo trasladar: {self:?}")
    }
}

impl Error for ErrorTraslado {}

// Los Actores son canónicos y todos los extremos ya fueron resueltos por
// Mundo::nuevo antes de construir estos índices. Sus índices son posiciones
// válidas de los vectores siguientes; los expect expresan esa invariante interna.
fn construir_parentesco(
    actores: &[Actor],
    relaciones: &[Relacion],
) -> Result<(Vec<Progenitores>, Vec<Vec<IdActor>>), ErrorConstruirMundo> {
    let mut padres = vec![Progenitores::default(); actores.len()];
    let mut hijos = vec![Vec::new(); actores.len()];
    for relacion in relaciones {
        if !matches!(relacion.tipo, TipoRelacion::Padre | TipoRelacion::Madre) {
            continue;
        }
        let (ExtremoRelacion::Actor(actor), ExtremoRelacion::Actor(progenitor)) =
            relacion.extremos()
        else {
            unreachable!("los extremos parentales fueron validados");
        };
        let p = &mut padres[actor.indice().expect("Actor validado")];
        let lugar = if relacion.tipo == TipoRelacion::Padre {
            &mut p.padre
        } else {
            &mut p.madre
        };
        if lugar.replace(progenitor).is_some() {
            return Err(ErrorConstruirMundo::ProgenitorMultiplicado {
                actor,
                tipo: relacion.tipo,
            });
        }
        if p.padre.is_some() && p.padre == p.madre {
            return Err(ErrorConstruirMundo::ProgenitoresNoDistintos { actor, progenitor });
        }
        hijos[progenitor.indice().expect("progenitor validado")].push(actor);
    }
    let mut colores = vec![0u8; actores.len()];
    for raiz in 0..actores.len() {
        if colores[raiz] != 0 {
            continue;
        }
        colores[raiz] = 1;
        let mut pila = vec![(raiz, 0usize)];
        while let Some(&(nodo, siguiente)) = pila.last() {
            if siguiente == 2 {
                colores[nodo] = 2;
                pila.pop();
                continue;
            }
            if let Some(tope) = pila.last_mut() {
                tope.1 += 1;
            }
            let Some(padre) = [padres[nodo].padre, padres[nodo].madre][siguiente] else {
                continue;
            };
            let indice = padre.indice().expect("progenitor validado");
            match colores[indice] {
                0 => {
                    colores[indice] = 1;
                    pila.push((indice, 0));
                }
                1 => {
                    let inicio = pila
                        .iter()
                        .position(|&(i, _)| i == indice)
                        .expect("un nodo en curso está en la pila");
                    let mut recorrido = pila[inicio..]
                        .iter()
                        .map(|&(i, _)| actores[i].id)
                        .collect::<Vec<_>>();
                    recorrido.push(padre);
                    return Err(ErrorConstruirMundo::CicloAscendencia { actores: recorrido });
                }
                _ => {}
            }
        }
    }
    for lista in &mut hijos {
        lista.sort_unstable();
    }
    Ok((padres, hijos))
}

fn es_lower_camel_case(valor: &str) -> bool {
    let mut bytes = valor.bytes();
    matches!(bytes.next(), Some(inicial) if inicial.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_alphabetic())
}

fn conjunto_caracteristicas(
    caracteristicas: Vec<Caracteristica>,
) -> Result<BTreeSet<Caracteristica>, Caracteristica> {
    let mut resultado = BTreeSet::new();
    for caracteristica in caracteristicas {
        if !resultado.insert(caracteristica.clone()) {
            return Err(caracteristica);
        }
    }
    Ok(resultado)
}

fn resolver_actor(actores: &[Actor], id: IdActor) -> Option<&Actor> {
    actores.get(id.indice()?).filter(|actor| actor.id == id)
}

fn resolver_actor_mut(actores: &mut [Actor], id: IdActor) -> Option<&mut Actor> {
    actores.get_mut(id.indice()?).filter(|actor| actor.id == id)
}

fn trasladar_en_estado(
    actores_estado: &mut [Actor],
    lugares: &[Lugar],
    actor_jugador: IdActor,
    grupo: &Grupo,
    actor: IdActor,
    destino: IdLugar,
) -> Result<TrasladoConfirmado, ErrorOperacionMundo> {
    let indice_actor = actor
        .indice()
        .filter(|&indice| {
            actores_estado
                .get(indice)
                .is_some_and(|actual| actual.id == actor)
        })
        .ok_or(ErrorOperacionMundo::ActorDesconocido { actor })?;
    if resolver_lugar(lugares, destino).is_none() {
        return Err(ErrorOperacionMundo::DestinoDesconocido { destino });
    }
    let origen = actores_estado[indice_actor]
        .lugar()
        .ok_or(ErrorOperacionMundo::ActorFueraDelEscenario { actor })?;
    if resolver_lugar(lugares, origen).is_none() {
        return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
    }
    if destino == origen {
        return Err(ErrorOperacionMundo::DestinoEsLugarActual { lugar: origen });
    }
    if actor != actor_jugador && grupo.contiene(actor) {
        return Err(ErrorOperacionMundo::IntegranteGrupoNoPuedeTrasladarseSolo { actor });
    }

    let actores = if actor == actor_jugador {
        let mut actores = Vec::with_capacity(grupo.integrantes.len() + 1);
        actores.push(actor);
        actores.extend(grupo.integrantes.iter().copied());
        actores
    } else {
        vec![actor]
    };

    let mut indices = Vec::with_capacity(actores.len());
    let mut vistos = BTreeSet::new();
    for &id in &actores {
        if !vistos.insert(id) {
            return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
        }
        let Some(indice) = id.indice() else {
            return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
        };
        let Some(actual) = actores_estado.get(indice) else {
            return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
        };
        if actual.id != id || actual.lugar() != Some(origen) {
            return Err(ErrorOperacionMundo::EstadoGrupoInconsistente);
        }
        indices.push(indice);
    }

    for indice in indices {
        actores_estado[indice].ubicacion = UbicacionActor::EnLugar(destino);
    }

    Ok(TrasladoConfirmado {
        actores,
        origen,
        destino,
    })
}

fn error_operacion_como_traslado(error: ErrorOperacionMundo) -> ErrorTraslado {
    match error {
        ErrorOperacionMundo::ActorFueraDelEscenario { actor } => {
            ErrorTraslado::ActorFueraDelEscenario { actor }
        }
        ErrorOperacionMundo::ActorDesconocido { actor } => {
            ErrorTraslado::ActorDesconocido { actor }
        }
        ErrorOperacionMundo::DestinoDesconocido { destino } => {
            ErrorTraslado::DestinoDesconocido { destino }
        }
        ErrorOperacionMundo::DestinoEsLugarActual { lugar } => {
            ErrorTraslado::DestinoEsLugarActual { lugar }
        }
        ErrorOperacionMundo::IntegranteGrupoNoPuedeTrasladarseSolo { actor } => {
            ErrorTraslado::IntegranteGrupoNoPuedeTrasladarseSolo { actor }
        }
        ErrorOperacionMundo::LoteVacio | ErrorOperacionMundo::EstadoGrupoInconsistente => {
            ErrorTraslado::EstadoInconsistente
        }
    }
}

fn resolver_lugar(lugares: &[Lugar], id: IdLugar) -> Option<&Lugar> {
    lugares.get(id.indice()?).filter(|lugar| lugar.id == id)
}

fn error_tipo(
    clave: &ClaveVariableGlobal,
    esperado: TipoVariableGlobal,
    encontrado: &ValorVariableGlobal,
) -> ErrorConsultaVariableGlobal {
    ErrorConsultaVariableGlobal::TipoIncompatible {
        clave: clave.clone(),
        esperado,
        encontrado: encontrado.tipo(),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::Mes;

    fn id_actor(posicion: usize) -> IdActor {
        IdActor::desde_posicion(posicion).expect("ID de prueba válido")
    }

    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID de prueba válido")
    }

    fn caracteristica(valor: &str) -> Caracteristica {
        Caracteristica::nueva(valor.to_owned()).expect("Característica de prueba válida")
    }

    fn clave(valor: &str) -> ClaveVariableGlobal {
        ClaveVariableGlobal::nueva(valor.to_owned()).expect("clave de prueba válida")
    }

    fn datos_familia() -> DatosInicialesMundo {
        let mut datos = datos_validos();
        for i in 3..9 {
            datos.actores.push(ActorInicial {
                id: id_actor(i),
                ubicacion: UbicacionActor::FueraDelEscenario,
                caracteristicas: if i == 7 {
                    vec![caracteristica("muerto")]
                } else {
                    vec![]
                },
            });
        }
        for (tipo, hijo, progenitor) in [
            (TipoRelacion::Padre, 0, 3),
            (TipoRelacion::Madre, 0, 4),
            (TipoRelacion::Padre, 2, 3),
            (TipoRelacion::Padre, 5, 3),
            (TipoRelacion::Madre, 5, 4),
            (TipoRelacion::MatadoPor, 7, 8),
        ] {
            datos.relaciones.push(RelacionInicial {
                tipo,
                origen: ExtremoRelacion::Actor(id_actor(hijo)),
                destino: ExtremoRelacion::Actor(id_actor(progenitor)),
            });
        }
        datos
    }

    #[test]
    fn actores_externos_existen_sin_ocupar_lugares_y_no_pueden_ser_jugador_o_grupo() {
        let datos = datos_familia();
        let mundo = Mundo::nuevo(datos.clone()).unwrap();
        assert_eq!(
            mundo.actor(id_actor(3)).unwrap().ubicacion(),
            UbicacionActor::FueraDelEscenario
        );
        assert_eq!(mundo.ubicacion_actor(id_actor(3)), None);
        assert_eq!(mundo.ubicacion_actor(id_actor(99)), None);
        assert!(mundo.actor(id_actor(3)).is_some());
        assert!(mundo.actor(id_actor(99)).is_none());
        assert_eq!(mundo.actores_en_lugar(id_lugar(0)).unwrap().count(), 2);
        let mut erroneos = datos.clone();
        erroneos.actores[0].ubicacion = UbicacionActor::FueraDelEscenario;
        assert!(matches!(
            Mundo::nuevo(erroneos),
            Err(ErrorConstruirMundo::JugadorFueraDelEscenario { .. })
        ));
        let mut erroneos = datos;
        erroneos.actores[1].ubicacion = UbicacionActor::FueraDelEscenario;
        assert!(matches!(
            Mundo::nuevo(erroneos),
            Err(ErrorConstruirMundo::IntegranteGrupoFueraDelEscenario { .. })
        ));
    }

    #[test]
    fn actor_externo_vivo_rechaza_acciones_y_lotes_sin_azar_ni_cambios_parciales() {
        use crate::{
            FraccionDia, InstanteAccion, acciones,
            azar::{ErrorFuenteAzar, FuenteAzar},
        };
        struct SinAzar;
        impl FuenteAzar for SinAzar {
            fn elegir_indice(
                &mut self,
                _: std::num::NonZeroUsize,
            ) -> Result<usize, ErrorFuenteAzar> {
                panic!("no debe sortear")
            }
        }
        let mut mundo = Mundo::nuevo(datos_familia()).unwrap();
        let antes = Mundo::nuevo(datos_familia()).unwrap();
        let actor = id_actor(8);
        assert_eq!(
            acciones::destinos_disponibles(&mundo, actor),
            Err(acciones::RechazoAccion::ActorFueraDelEscenario)
        );
        assert_eq!(
            acciones::iniciar_acampada(&mundo, actor).unwrap_err(),
            acciones::ErrorAcampada::ActorFueraDelEscenario
        );
        let contexto = acciones::ContextoAccion {
            inicio: InstanteAccion {
                fecha: mundo.fecha(),
                fraccion: FraccionDia::CERO,
            },
            rumbos: &acciones::CatalogoRumbos::default(),
        };
        assert!(matches!(
            acciones::iniciar(
                &mut mundo,
                acciones::SolicitudAccion::Viajar {
                    actor,
                    destino: id_lugar(1)
                },
                &contexto,
                &mut SinAzar
            ),
            Err(acciones::ErrorMotor::Rechazo(
                acciones::RechazoAccion::ActorFueraDelEscenario
            ))
        ));
        assert_eq!(
            mundo.trasladar(actor, id_lugar(1)),
            Err(ErrorTraslado::ActorFueraDelEscenario { actor })
        );
        assert_eq!(
            mundo.aplicar_lote(
                LoteOperaciones::nuevo(vec![
                    OperacionMundo::AnadirCaracteristicaActor {
                        actor: id_actor(0),
                        caracteristica: caracteristica("herido")
                    },
                    OperacionMundo::Trasladar {
                        actor,
                        destino: id_lugar(1)
                    },
                ])
                .unwrap()
            ),
            Err(ErrorOperacionMundo::ActorFueraDelEscenario { actor })
        );
        assert_eq!(mundo, antes);
    }

    #[test]
    fn hermanos_se_deducen_con_evidencia_sin_materializar_parejas() {
        let mut mundo = Mundo::nuevo(datos_familia()).unwrap();
        let antes = Mundo::nuevo(datos_familia()).unwrap();
        assert_eq!(
            mundo.progenitores(id_actor(0)),
            Some(Progenitores {
                padre: Some(id_actor(3)),
                madre: Some(id_actor(4))
            })
        );
        assert_eq!(
            mundo.hijos_de(id_actor(3)),
            Some([id_actor(0), id_actor(2), id_actor(5)].as_slice())
        );
        let esperado = vec![
            HermanoDerivado {
                actor: id_actor(2),
                progenitores_compartidos: vec![id_actor(3)],
            },
            HermanoDerivado {
                actor: id_actor(5),
                progenitores_compartidos: vec![id_actor(3), id_actor(4)],
            },
        ];
        assert_eq!(mundo.hermanos_de(id_actor(0)), Some(esperado.clone()));
        assert_eq!(mundo.hermanos_de(id_actor(1)), Some(vec![]));
        assert_eq!(mundo.hermanos_de(id_actor(99)), None);
        assert_eq!(mundo, antes);
        mundo
            .aplicar_lote(LoteOperaciones::uno(
                OperacionMundo::RegistrarConocimiento {
                    actor: id_actor(0),
                    lugar: id_lugar(1),
                },
            ))
            .unwrap();
        assert_eq!(mundo.hermanos_de(id_actor(0)), Some(esperado));
        assert_eq!(mundo.relaciones().len(), antes.relaciones().len() + 1);
        assert_eq!(
            mundo
                .relaciones_de(ExtremoRelacion::Actor(id_actor(3)))
                .unwrap()
                .len(),
            3
        );
    }

    #[test]
    fn filiacion_rechaza_cardinalidad_igualdad_autorreferencias_y_extremos_invalidos() {
        for tipo in [TipoRelacion::Padre, TipoRelacion::Madre] {
            let mut datos = datos_validos();
            for padre in [1, 2] {
                datos.relaciones.push(RelacionInicial {
                    tipo,
                    origen: ExtremoRelacion::Actor(id_actor(0)),
                    destino: ExtremoRelacion::Actor(id_actor(padre)),
                });
            }
            assert!(matches!(
                Mundo::nuevo(datos),
                Err(ErrorConstruirMundo::ProgenitorMultiplicado { .. })
            ));
        }
        let mut datos = datos_validos();
        for tipo in [TipoRelacion::Padre, TipoRelacion::Madre] {
            datos.relaciones.push(RelacionInicial {
                tipo,
                origen: ExtremoRelacion::Actor(id_actor(0)),
                destino: ExtremoRelacion::Actor(id_actor(2)),
            });
        }
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::ProgenitoresNoDistintos { .. })
        ));
        for tipo in [
            TipoRelacion::Padre,
            TipoRelacion::Madre,
            TipoRelacion::MatadoPor,
        ] {
            let mut datos = datos_validos();
            datos.relaciones.push(RelacionInicial {
                tipo,
                origen: ExtremoRelacion::Actor(id_actor(0)),
                destino: ExtremoRelacion::Actor(id_actor(0)),
            });
            assert!(matches!(
                Mundo::nuevo(datos),
                Err(ErrorConstruirMundo::RelacionReflexiva { .. })
            ));
            let mut datos = datos_validos();
            datos.relaciones.push(RelacionInicial {
                tipo,
                origen: ExtremoRelacion::Actor(id_actor(0)),
                destino: ExtremoRelacion::Lugar(id_lugar(0)),
            });
            assert!(matches!(
                Mundo::nuevo(datos),
                Err(ErrorConstruirMundo::ExtremosRelacionIncompatibles { .. })
            ));
        }
    }

    #[test]
    fn ascendencia_detecta_ciclos_mezclados_pero_no_prohibe_ciclos_historicos() {
        let mut datos = datos_validos();
        for (hijo, padre, tipo) in [
            (0, 1, TipoRelacion::Padre),
            (1, 2, TipoRelacion::Madre),
            (2, 0, TipoRelacion::Padre),
        ] {
            datos.relaciones.push(RelacionInicial {
                tipo,
                origen: ExtremoRelacion::Actor(id_actor(hijo)),
                destino: ExtremoRelacion::Actor(id_actor(padre)),
            });
        }
        let Err(ErrorConstruirMundo::CicloAscendencia { actores }) = Mundo::nuevo(datos) else {
            panic!("no detectó el ciclo")
        };
        assert_eq!(
            actores,
            vec![id_actor(0), id_actor(1), id_actor(2), id_actor(0)]
        );
        let mut datos = datos_validos();
        for (a, b) in [(0, 2), (2, 0)] {
            datos.relaciones.push(RelacionInicial {
                tipo: TipoRelacion::MatadoPor,
                origen: ExtremoRelacion::Actor(id_actor(a)),
                destino: ExtremoRelacion::Actor(id_actor(b)),
            });
        }
        assert!(Mundo::nuevo(datos).is_ok());
    }

    #[test]
    fn genealogia_profunda_no_depende_de_la_pila_de_llamadas() {
        let mut datos = datos_validos();
        for i in 3..20_000 {
            datos.actores.push(ActorInicial {
                id: id_actor(i),
                ubicacion: UbicacionActor::FueraDelEscenario,
                caracteristicas: vec![],
            });
        }
        for i in 2..19_999 {
            datos.relaciones.push(RelacionInicial {
                tipo: TipoRelacion::Padre,
                origen: ExtremoRelacion::Actor(id_actor(i)),
                destino: ExtremoRelacion::Actor(id_actor(i + 1)),
            });
        }
        let mundo = Mundo::nuevo(datos).unwrap();
        assert_eq!(
            mundo.progenitores(id_actor(19_998)).unwrap().padre,
            Some(id_actor(19_999))
        );
        assert_eq!(mundo.hermanos_de(id_actor(2)), Some(vec![]));
    }

    fn datos_validos() -> DatosInicialesMundo {
        DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).expect("ID de mapa válido"),
            fecha: Fecha::nueva(2025, Mes::Enero, 31).expect("fecha válida"),
            lugares: vec![
                LugarInicial {
                    id: id_lugar(0),
                    caracteristicas: vec![caracteristica("arido")],
                },
                LugarInicial {
                    id: id_lugar(1),
                    caracteristicas: vec![caracteristica("fertil")],
                },
            ],
            actores: vec![
                ActorInicial {
                    id: id_actor(0),
                    ubicacion: crate::UbicacionActor::EnLugar(id_lugar(0)),
                    caracteristicas: vec![],
                },
                ActorInicial {
                    id: id_actor(1),
                    ubicacion: crate::UbicacionActor::EnLugar(id_lugar(0)),
                    caracteristicas: vec![caracteristica("fuerte")],
                },
                ActorInicial {
                    id: id_actor(2),
                    ubicacion: crate::UbicacionActor::EnLugar(id_lugar(1)),
                    caracteristicas: vec![],
                },
            ],
            actor_jugador: id_actor(0),
            companeros: vec![id_actor(1)],
            variables_globales: vec![
                VariableGlobalInicial {
                    clave: clave("eventoOcurrio"),
                    valor: ValorVariableGlobal::Booleana(true),
                },
                VariableGlobalInicial {
                    clave: clave("contador"),
                    valor: ValorVariableGlobal::Entera(3),
                },
                VariableGlobalInicial {
                    clave: clave("estado"),
                    valor: ValorVariableGlobal::Texto("inicial".to_owned()),
                },
            ],
            relaciones: vec![RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id_lugar(0)),
                destino: ExtremoRelacion::Lugar(id_lugar(1)),
            }],
        }
    }

    #[test]
    fn abarcar_indexa_ambos_extremos_sin_crear_rutas_para_jugador_ni_npcs() {
        let mut datos = datos_validos();
        datos.lugares.push(LugarInicial {
            id: id_lugar(2),
            caracteristicas: vec![caracteristica("region")],
        });
        datos.relaciones.push(RelacionInicial {
            tipo: TipoRelacion::Abarca,
            origen: ExtremoRelacion::Lugar(id_lugar(2)),
            destino: ExtremoRelacion::Lugar(id_lugar(0)),
        });
        let mundo = Mundo::nuevo(datos.clone()).unwrap();
        let id = IdRelacion::desde_posicion(1).unwrap();
        assert!(mundo.lugar(id_lugar(2)).unwrap().es_region());
        assert_eq!(mundo.lugares_conectados(id_lugar(2)), Some([].as_slice()));
        assert_eq!(
            mundo.relaciones_de(ExtremoRelacion::Lugar(id_lugar(2))),
            Some([id].as_slice())
        );
        assert!(
            mundo
                .relaciones_de(ExtremoRelacion::Lugar(id_lugar(0)))
                .unwrap()
                .contains(&id)
        );
        assert_eq!(
            crate::acciones::destinos_disponibles(&mundo, id_actor(0)).unwrap(),
            vec![id_lugar(1)]
        );
        assert_eq!(
            crate::acciones::destinos_disponibles(&mundo, id_actor(2)).unwrap(),
            vec![id_lugar(0)]
        );
        for (a, b) in [(0, 2), (2, 0)] {
            let mut invalido = datos.clone();
            invalido.relaciones.push(RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id_lugar(a)),
                destino: ExtremoRelacion::Lugar(id_lugar(b)),
            });
            assert!(matches!(
                Mundo::nuevo(invalido),
                Err(ErrorConstruirMundo::ConexionConRegion { .. })
            ));
        }
        let mut invalido = datos.clone();
        invalido.relaciones[1].origen = ExtremoRelacion::Lugar(id_lugar(1));
        assert!(matches!(
            Mundo::nuevo(invalido),
            Err(ErrorConstruirMundo::OrigenAbarcaNoEsRegion { .. })
        ));
        datos.relaciones.push(datos.relaciones[1].clone());
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::RelacionDuplicada { .. })
        ));
    }

    #[test]
    fn relaciones_geograficas_son_dirigidas_y_no_alteran_los_caminos() {
        for tipo in [
            TipoRelacion::OrigenDe,
            TipoRelacion::AfluenteDe,
            TipoRelacion::Bloquea,
        ] {
            let mut datos = datos_validos();
            datos.lugares.push(LugarInicial {
                id: id_lugar(2),
                caracteristicas: vec![caracteristica("region")],
            });
            // Una relación descriptiva puede coexistir con Conectado, y una
            // región puede participar sin convertirse en destino transitable.
            for (origen, destino) in [(0, 1), (0, 2), (2, 0)] {
                datos.relaciones.push(RelacionInicial {
                    tipo,
                    origen: ExtremoRelacion::Lugar(id_lugar(origen)),
                    destino: ExtremoRelacion::Lugar(id_lugar(destino)),
                });
            }
            let mundo = Mundo::nuevo(datos).unwrap();
            let ids: Vec<_> = (0..4)
                .map(|i| IdRelacion::desde_posicion(i).unwrap())
                .collect();
            assert_eq!(
                mundo.relaciones_de(ExtremoRelacion::Lugar(id_lugar(0))),
                Some(ids.as_slice())
            );
            assert_eq!(
                mundo.relaciones_de(ExtremoRelacion::Lugar(id_lugar(2))),
                Some(&ids[2..])
            );
            assert_eq!(
                mundo.relacion(ids[2]).unwrap().extremos(),
                (
                    ExtremoRelacion::Lugar(id_lugar(0)),
                    ExtremoRelacion::Lugar(id_lugar(2))
                )
            );
            assert_eq!(
                mundo.relacion(ids[3]).unwrap().extremos(),
                (
                    ExtremoRelacion::Lugar(id_lugar(2)),
                    ExtremoRelacion::Lugar(id_lugar(0))
                )
            );
            assert_eq!(mundo.lugares_conectados(id_lugar(2)), Some([].as_slice()));
            assert_eq!(
                crate::acciones::destinos_disponibles(&mundo, id_actor(0)).unwrap(),
                vec![id_lugar(1)]
            );
            assert_eq!(
                crate::acciones::destinos_disponibles(&mundo, id_actor(2)).unwrap(),
                vec![id_lugar(0)]
            );
        }
    }

    #[test]
    fn relaciones_geograficas_rechazan_extremos_invalidos_y_duplicados() {
        for tipo in [
            TipoRelacion::OrigenDe,
            TipoRelacion::AfluenteDe,
            TipoRelacion::Bloquea,
        ] {
            let mut datos = datos_validos();
            datos.relaciones[0].tipo = tipo;

            let mut desconocida = datos.clone();
            desconocida.relaciones[0].destino = ExtremoRelacion::Lugar(id_lugar(9));
            assert!(matches!(
                Mundo::nuevo(desconocida),
                Err(ErrorConstruirMundo::ExtremoRelacionDesconocido { posicion: 0, .. })
            ));

            for origen_actor in [false, true] {
                let mut incompatible = datos.clone();
                if origen_actor {
                    incompatible.relaciones[0].origen = ExtremoRelacion::Actor(id_actor(0));
                } else {
                    incompatible.relaciones[0].destino = ExtremoRelacion::Actor(id_actor(0));
                }
                assert!(matches!(
                    Mundo::nuevo(incompatible),
                    Err(ErrorConstruirMundo::ExtremosRelacionIncompatibles { posicion: 0, .. })
                ));
            }

            let mut reflexiva = datos.clone();
            reflexiva.relaciones[0].destino = ExtremoRelacion::Lugar(id_lugar(0));
            assert!(matches!(
                Mundo::nuevo(reflexiva),
                Err(ErrorConstruirMundo::RelacionReflexiva { posicion: 0, .. })
            ));

            datos.relaciones.push(datos.relaciones[0].clone());
            assert!(matches!(
                Mundo::nuevo(datos),
                Err(ErrorConstruirMundo::RelacionDuplicada { posicion: 1, .. })
            ));
        }
    }

    #[test]
    fn construye_y_consulta_un_mundo_valido_en_orden_canonico() {
        let mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        assert_eq!(mundo.actor_jugador(), id_actor(0));
        assert_eq!(mundo.grupo().integrantes(), &[id_actor(1)]);
        assert_eq!(mundo.ubicacion_grupo(), id_lugar(0));
        assert_eq!(
            mundo.actores().iter().map(Actor::id).collect::<Vec<_>>(),
            vec![id_actor(0), id_actor(1), id_actor(2)]
        );
        assert_eq!(
            mundo
                .actores_en_lugar(id_lugar(0))
                .expect("Lugar existente")
                .map(Actor::id)
                .collect::<Vec<_>>(),
            vec![id_actor(0), id_actor(1)]
        );
        assert!(mundo.actores_en_lugar(id_lugar(9)).is_none());
        assert!(
            mundo
                .lugar(id_lugar(0))
                .expect("Lugar existente")
                .tiene_caracteristica(&caracteristica("arido"))
        );
        let id_relacion = IdRelacion::desde_posicion(0).expect("ID válido");
        let relacion = mundo.relacion(id_relacion).expect("Relación existente");
        assert_eq!(relacion.tipo(), TipoRelacion::Conectado);
        assert_eq!(
            relacion.extremos(),
            (
                ExtremoRelacion::Lugar(id_lugar(0)),
                ExtremoRelacion::Lugar(id_lugar(1))
            )
        );
        assert_eq!(
            mundo.lugares_conectados(id_lugar(0)),
            Some([id_lugar(1)].as_slice())
        );
        assert_eq!(
            mundo.lugares_conectados(id_lugar(1)),
            Some([id_lugar(0)].as_slice())
        );
        assert_eq!(
            mundo.relaciones_de(ExtremoRelacion::Lugar(id_lugar(1))),
            Some([id_relacion].as_slice())
        );
        assert_eq!(
            mundo.relaciones_de(ExtremoRelacion::Actor(id_actor(0))),
            Some([].as_slice())
        );
        assert!(mundo.lugares_conectados(id_lugar(9)).is_none());
    }

    #[test]
    fn rechaza_relaciones_invalidas_sin_publicar_indices() {
        let mut desconocida = datos_validos();
        desconocida.relaciones[0].destino = ExtremoRelacion::Lugar(id_lugar(9));
        assert!(matches!(
            Mundo::nuevo(desconocida),
            Err(ErrorConstruirMundo::ExtremoRelacionDesconocido { posicion: 0, .. })
        ));

        let mut incompatible = datos_validos();
        incompatible.relaciones[0].destino = ExtremoRelacion::Actor(id_actor(2));
        assert!(matches!(
            Mundo::nuevo(incompatible),
            Err(ErrorConstruirMundo::ExtremosRelacionIncompatibles { posicion: 0, .. })
        ));

        let mut reflexiva = datos_validos();
        reflexiva.relaciones[0].destino = ExtremoRelacion::Lugar(id_lugar(0));
        assert!(matches!(
            Mundo::nuevo(reflexiva),
            Err(ErrorConstruirMundo::RelacionReflexiva { posicion: 0, .. })
        ));

        let mut duplicada = datos_validos();
        duplicada.relaciones.push(RelacionInicial {
            tipo: TipoRelacion::Conectado,
            origen: ExtremoRelacion::Lugar(id_lugar(1)),
            destino: ExtremoRelacion::Lugar(id_lugar(0)),
        });
        assert_eq!(
            Mundo::nuevo(duplicada),
            Err(ErrorConstruirMundo::RelacionDuplicada {
                posicion: 1,
                primera: IdRelacion::desde_posicion(0).expect("ID válido"),
            })
        );
    }

    #[test]
    fn valida_ids_y_sintaxis_de_etiquetas_sin_truncar() {
        assert_eq!(
            IdActor::desde_posicion(usize::MAX),
            Err(ErrorIdentificador::PosicionFueraDeRango {
                posicion: usize::MAX
            })
        );
        for invalida in ["", "CargaTesoro", "carga_tesoro", "carga2", "rápido"] {
            assert_eq!(
                Caracteristica::nueva(invalida.to_owned()),
                Err(ErrorCaracteristica::SintaxisInvalida)
            );
            assert_eq!(
                ClaveVariableGlobal::nueva(invalida.to_owned()),
                Err(ErrorClaveVariableGlobal::SintaxisInvalida)
            );
        }
        assert!(matches!(
            Caracteristica::nueva("a".repeat(65)),
            Err(ErrorCaracteristica::DemasiadoLarga { .. })
        ));
        assert_eq!(caracteristica("cargaTesoro").como_str(), "cargaTesoro");
    }

    #[test]
    fn respeta_precedencia_y_rechaza_invariantes_de_construccion() {
        let mut datos = datos_validos();
        datos.lugares.clear();
        datos.actores.clear();
        assert_eq!(Mundo::nuevo(datos), Err(ErrorConstruirMundo::SinLugares));

        let mut datos = datos_validos();
        datos.actores.clear();
        assert_eq!(Mundo::nuevo(datos), Err(ErrorConstruirMundo::SinActores));

        let mut datos = datos_validos();
        datos.lugares[0].id = id_lugar(1);
        datos.lugares[0].caracteristicas = vec![caracteristica("arido"); 2];
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::IdLugarNoCanonico { posicion: 0, .. })
        ));

        let mut datos = datos_validos();
        datos.actores[2].ubicacion = UbicacionActor::EnLugar(id_lugar(8));
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::LugarDeActorDesconocido { actor, .. })
                if actor == id_actor(2)
        ));

        let mut datos = datos_validos();
        datos.actores[0].id = id_actor(1);
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::IdActorNoCanonico { posicion: 0, .. })
        ));

        let mut datos = datos_validos();
        datos.actor_jugador = id_actor(8);
        assert_eq!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::ActorJugadorDesconocido { actor: id_actor(8) })
        );

        let mut datos = datos_validos();
        datos.companeros = vec![];
        assert_eq!(Mundo::nuevo(datos), Err(ErrorConstruirMundo::GrupoVacio));

        let mut datos = datos_validos();
        datos.companeros = vec![id_actor(0)];
        assert_eq!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::JugadorIncluidoEnGrupo { actor: id_actor(0) })
        );

        let mut datos = datos_validos();
        datos.companeros = vec![id_actor(1), id_actor(1)];
        assert_eq!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::IntegranteGrupoDuplicado { actor: id_actor(1) })
        );

        let mut datos = datos_validos();
        datos.companeros = vec![id_actor(8)];
        assert_eq!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::IntegranteGrupoDesconocido { actor: id_actor(8) })
        );

        let mut datos = datos_validos();
        datos.actores[1].ubicacion = UbicacionActor::EnLugar(id_lugar(1));
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::IntegranteGrupoSeparado { actor, .. })
                if actor == id_actor(1)
        ));
    }

    #[test]
    fn rechaza_caracteristicas_y_globales_duplicadas() {
        let mut datos = datos_validos();
        datos.lugares[0].caracteristicas = vec![caracteristica("arido"); 2];
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::CaracteristicaLugarDuplicada { .. })
        ));

        let mut datos = datos_validos();
        datos.actores[0].caracteristicas = vec![caracteristica("herido"); 2];
        assert!(matches!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::CaracteristicaActorDuplicada { .. })
        ));

        let mut datos = datos_validos();
        datos.variables_globales.push(VariableGlobalInicial {
            clave: clave("contador"),
            valor: ValorVariableGlobal::Texto(String::new()),
        });
        assert_eq!(
            Mundo::nuevo(datos),
            Err(ErrorConstruirMundo::VariableGlobalDuplicada {
                clave: clave("contador")
            })
        );
    }

    #[test]
    fn consulta_globales_tipadas_y_en_orden_lexicografico() {
        let mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        assert_eq!(mundo.variable_booleana(&clave("eventoOcurrio")), Ok(true));
        assert_eq!(mundo.variable_entera(&clave("contador")), Ok(3));
        assert_eq!(mundo.variable_texto(&clave("estado")), Ok("inicial"));
        assert_eq!(
            mundo.variable_entera(&clave("eventoOcurrio")),
            Err(ErrorConsultaVariableGlobal::TipoIncompatible {
                clave: clave("eventoOcurrio"),
                esperado: TipoVariableGlobal::Entera,
                encontrado: TipoVariableGlobal::Booleana,
            })
        );
        assert_eq!(
            mundo.variable_booleana(&clave("inventadaEnContenido")),
            Err(ErrorConsultaVariableGlobal::Inexistente {
                clave: clave("inventadaEnContenido")
            })
        );
        assert_eq!(
            mundo
                .variables_globales()
                .map(|(clave, _)| clave.como_str())
                .collect::<Vec<_>>(),
            vec!["contador", "estado", "eventoOcurrio"]
        );
    }

    #[test]
    fn traslada_npc_o_jugador_con_grupo_y_devuelve_el_resultado() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        let npc = mundo
            .trasladar(id_actor(2), id_lugar(0))
            .expect("traslado válido");
        assert_eq!(
            npc,
            TrasladoConfirmado {
                actores: vec![id_actor(2)],
                origen: id_lugar(1),
                destino: id_lugar(0),
            }
        );

        let grupo = mundo
            .trasladar(id_actor(0), id_lugar(1))
            .expect("traslado válido");
        assert_eq!(grupo.actores, vec![id_actor(0), id_actor(1)]);
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(1)));
        assert_eq!(mundo.ubicacion_actor(id_actor(1)), Some(id_lugar(1)));
    }

    #[test]
    fn rechazos_de_traslado_no_cambian_el_mundo() {
        let casos = [
            (
                id_actor(9),
                id_lugar(1),
                ErrorTraslado::ActorDesconocido { actor: id_actor(9) },
            ),
            (
                id_actor(0),
                id_lugar(9),
                ErrorTraslado::DestinoDesconocido {
                    destino: id_lugar(9),
                },
            ),
            (
                id_actor(0),
                id_lugar(0),
                ErrorTraslado::DestinoEsLugarActual { lugar: id_lugar(0) },
            ),
            (
                id_actor(1),
                id_lugar(1),
                ErrorTraslado::IntegranteGrupoNoPuedeTrasladarseSolo { actor: id_actor(1) },
            ),
        ];

        for (actor, destino, esperado) in casos {
            let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
            let antes = mundo.actores().iter().map(Actor::lugar).collect::<Vec<_>>();
            assert_eq!(mundo.trasladar(actor, destino), Err(esperado));
            assert_eq!(
                mundo.actores().iter().map(Actor::lugar).collect::<Vec<_>>(),
                antes
            );
        }
    }

    #[test]
    fn detecta_grupo_inconsistente_antes_de_escribir() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        mundo.actores[1].ubicacion = UbicacionActor::EnLugar(id_lugar(1));
        assert_eq!(
            mundo.trasladar(id_actor(0), id_lugar(1)),
            Err(ErrorTraslado::EstadoInconsistente)
        );
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
    }

    #[test]
    fn lote_aplica_cambios_en_orden_y_omite_operaciones_idempotentes() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        let fatigado = caracteristica("fatigado");
        let herido = caracteristica("herido");
        let lote = LoteOperaciones::nuevo(vec![
            OperacionMundo::AnadirCaracteristicaActor {
                actor: id_actor(0),
                caracteristica: fatigado.clone(),
            },
            OperacionMundo::AnadirCaracteristicaActor {
                actor: id_actor(0),
                caracteristica: fatigado.clone(),
            },
            OperacionMundo::QuitarCaracteristicaActor {
                actor: id_actor(0),
                caracteristica: fatigado.clone(),
            },
            OperacionMundo::AnadirCaracteristicaActor {
                actor: id_actor(0),
                caracteristica: herido.clone(),
            },
        ])
        .expect("lote no vacío");

        let cambios = mundo.aplicar_lote(lote).expect("lote válido");
        assert_eq!(
            cambios.como_slice(),
            [
                CambioConfirmado::CaracteristicaActorAnadida {
                    actor: id_actor(0),
                    caracteristica: fatigado.clone(),
                },
                CambioConfirmado::CaracteristicaActorQuitada {
                    actor: id_actor(0),
                    caracteristica: fatigado.clone(),
                },
                CambioConfirmado::CaracteristicaActorAnadida {
                    actor: id_actor(0),
                    caracteristica: herido.clone(),
                },
            ]
        );
        let actor = mundo.actor(id_actor(0)).expect("Actor existente");
        assert!(!actor.tiene_caracteristica(&fatigado));
        assert!(actor.tiene_caracteristica(&herido));
    }

    #[test]
    fn lote_fallido_no_confirma_ninguna_operacion_anterior() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        let antes = mundo.actores.clone();
        let lote = LoteOperaciones::nuevo(vec![
            OperacionMundo::AnadirCaracteristicaActor {
                actor: id_actor(0),
                caracteristica: caracteristica("fatigado"),
            },
            OperacionMundo::Trasladar {
                actor: id_actor(9),
                destino: id_lugar(1),
            },
        ])
        .expect("lote no vacío");

        assert_eq!(
            mundo.aplicar_lote(lote),
            Err(ErrorOperacionMundo::ActorDesconocido { actor: id_actor(9) })
        );
        assert_eq!(mundo.actores, antes);
    }

    #[test]
    fn lote_combina_traslado_de_grupo_y_caracteristicas_atomicamente() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        let herido = caracteristica("herido");
        let lote = LoteOperaciones::nuevo(vec![
            OperacionMundo::Trasladar {
                actor: id_actor(0),
                destino: id_lugar(1),
            },
            OperacionMundo::AnadirCaracteristicaActor {
                actor: id_actor(1),
                caracteristica: herido.clone(),
            },
        ])
        .expect("lote no vacío");
        let cambios = mundo.aplicar_lote(lote).expect("lote válido");

        assert!(matches!(
            cambios.como_slice(),
            [
                CambioConfirmado::ActoresTrasladados { actores, .. },
                CambioConfirmado::CaracteristicaActorAnadida { actor, caracteristica }
            ] if actores == &vec![id_actor(0), id_actor(1)]
                && *actor == id_actor(1)
                && caracteristica == &herido
        ));
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(1)));
        assert_eq!(mundo.ubicacion_actor(id_actor(1)), Some(id_lugar(1)));
        assert!(
            mundo
                .actor(id_actor(1))
                .expect("Actor existente")
                .tiene_caracteristica(&herido)
        );
    }

    #[test]
    fn lote_vacio_se_rechaza_antes_de_aplicar() {
        assert_eq!(
            LoteOperaciones::nuevo(vec![]),
            Err(ErrorOperacionMundo::LoteVacio)
        );
    }

    #[test]
    fn avanza_fecha_atomicamente() {
        let mut mundo = Mundo::nuevo(datos_validos()).expect("Mundo válido");
        assert_eq!(
            mundo.avanzar_fecha(Duracion::desde_dias(2)),
            Fecha::nueva(2025, Mes::Febrero, 2)
        );
        let limite = Fecha::nueva(9999, Mes::Diciembre, 31).expect("fecha válida");
        mundo.fecha = limite;
        assert_eq!(
            mundo.avanzar_fecha(Duracion::desde_dias(1)),
            Err(ErrorCalendario::Desbordamiento)
        );
        assert_eq!(mundo.fecha(), limite);
    }
}
