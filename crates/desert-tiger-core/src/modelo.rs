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

impl IdActor {
    fn indice(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

impl IdLugar {
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
    pub lugar: IdLugar,
    pub caracteristicas: Vec<Caracteristica>,
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

    pub fn tiene_caracteristica(&self, caracteristica: &Caracteristica) -> bool {
        self.caracteristicas.contains(caracteristica)
    }

    pub fn caracteristicas(&self) -> impl ExactSizeIterator<Item = &Caracteristica> + '_ {
        self.caracteristicas.iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Actor {
    id: IdActor,
    lugar: IdLugar,
    caracteristicas: BTreeSet<Caracteristica>,
}

impl Actor {
    pub fn id(&self) -> IdActor {
        self.id
    }

    pub fn lugar(&self) -> IdLugar {
        self.lugar
    }

    pub fn tiene_caracteristica(&self, caracteristica: &Caracteristica) -> bool {
        self.caracteristicas.contains(caracteristica)
    }

    pub fn caracteristicas(&self) -> impl ExactSizeIterator<Item = &Caracteristica> + '_ {
        self.caracteristicas.iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
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
}

impl Mundo {
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
            if resolver_lugar(&lugares, inicial.lugar).is_none() {
                return Err(ErrorConstruirMundo::LugarDeActorDesconocido {
                    actor: inicial.id,
                    lugar: inicial.lugar,
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
                lugar: inicial.lugar,
                caracteristicas,
            });
        }

        let jugador = resolver_actor(&actores, datos.actor_jugador).ok_or(
            ErrorConstruirMundo::ActorJugadorDesconocido {
                actor: datos.actor_jugador,
            },
        )?;
        let lugar_jugador = jugador.lugar;

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
            if actor.lugar != lugar_jugador {
                return Err(ErrorConstruirMundo::IntegranteGrupoSeparado {
                    actor: integrante,
                    lugar_actor: actor.lugar,
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

        Ok(Self {
            mapa: datos.mapa,
            fecha: datos.fecha,
            lugares,
            actores,
            actor_jugador: datos.actor_jugador,
            grupo: Grupo {
                integrantes: datos.companeros,
            },
            variables_globales,
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
            Some(jugador) => jugador.lugar,
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
        self.actor(id).map(Actor::lugar)
    }

    pub fn actores(&self) -> &[Actor] {
        &self.actores
    }

    pub fn lugares(&self) -> &[Lugar] {
        &self.lugares
    }

    pub fn actores_en_lugar(&self, lugar: IdLugar) -> Option<impl Iterator<Item = &Actor> + '_> {
        self.lugar(lugar)?;
        Some(
            self.actores
                .iter()
                .filter(move |actor| actor.lugar == lugar),
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
        let indice_actor = actor
            .indice()
            .filter(|&indice| {
                self.actores
                    .get(indice)
                    .is_some_and(|actual| actual.id == actor)
            })
            .ok_or(ErrorTraslado::ActorDesconocido { actor })?;
        if self.lugar(destino).is_none() {
            return Err(ErrorTraslado::DestinoDesconocido { destino });
        }
        let origen = self.actores[indice_actor].lugar;
        if self.lugar(origen).is_none() {
            return Err(ErrorTraslado::EstadoInconsistente);
        }
        if destino == origen {
            return Err(ErrorTraslado::DestinoEsLugarActual { lugar: origen });
        }
        if actor != self.actor_jugador && self.grupo.contiene(actor) {
            return Err(ErrorTraslado::IntegranteGrupoNoPuedeTrasladarseSolo { actor });
        }

        let actores = if actor == self.actor_jugador {
            let mut actores = Vec::with_capacity(self.grupo.integrantes.len() + 1);
            actores.push(actor);
            actores.extend(self.grupo.integrantes.iter().copied());
            actores
        } else {
            vec![actor]
        };

        let mut indices = Vec::with_capacity(actores.len());
        let mut vistos = BTreeSet::new();
        for &id in &actores {
            if !vistos.insert(id) {
                return Err(ErrorTraslado::EstadoInconsistente);
            }
            let Some(indice) = id.indice() else {
                return Err(ErrorTraslado::EstadoInconsistente);
            };
            let Some(actual) = self.actores.get(indice) else {
                return Err(ErrorTraslado::EstadoInconsistente);
            };
            if actual.id != id || actual.lugar != origen {
                return Err(ErrorTraslado::EstadoInconsistente);
            }
            indices.push(indice);
        }

        for indice in indices {
            self.actores[indice].lugar = destino;
        }

        Ok(TrasladoConfirmado {
            actores,
            origen,
            destino,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrasladoConfirmado {
    pub actores: Vec<IdActor>,
    pub origen: IdLugar,
    pub destino: IdLugar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConstruirMundo {
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
}

impl Display for ErrorConstruirMundo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo construir el Mundo: {self:?}")
    }
}

impl Error for ErrorConstruirMundo {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorTraslado {
    ActorDesconocido { actor: IdActor },
    DestinoDesconocido { destino: IdLugar },
    DestinoEsLugarActual { lugar: IdLugar },
    IntegranteGrupoNoPuedeTrasladarseSolo { actor: IdActor },
    EstadoInconsistente,
}

impl Display for ErrorTraslado {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo trasladar: {self:?}")
    }
}

impl Error for ErrorTraslado {}

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
                    lugar: id_lugar(0),
                    caracteristicas: vec![],
                },
                ActorInicial {
                    id: id_actor(1),
                    lugar: id_lugar(0),
                    caracteristicas: vec![caracteristica("fuerte")],
                },
                ActorInicial {
                    id: id_actor(2),
                    lugar: id_lugar(1),
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
        datos.actores[2].lugar = id_lugar(8);
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
        datos.actores[1].lugar = id_lugar(1);
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
        mundo.actores[1].lugar = id_lugar(1);
        assert_eq!(
            mundo.trasladar(id_actor(0), id_lugar(1)),
            Err(ErrorTraslado::EstadoInconsistente)
        );
        assert_eq!(mundo.ubicacion_actor(id_actor(0)), Some(id_lugar(0)));
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
