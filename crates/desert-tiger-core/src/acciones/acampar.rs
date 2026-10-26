use crate::{Duracion, FraccionDia, IdActor, Mundo, TiempoAccion};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct Acampada {
    actor: IdActor,
}

#[derive(Debug)]
pub struct ResultadoAcampada {
    pub tiempo: TiempoAccion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorAcampada {
    ActorFueraDelEscenario,
    ActorDesconocido,
    ActorMuerto,
    IntegranteDelGrupo,
}

impl fmt::Display for ErrorAcampada {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ActorFueraDelEscenario => "el actor está fuera del escenario",
            Self::ActorDesconocido => "actor desconocido",
            Self::ActorMuerto => "un actor muerto no puede acampar",
            Self::IntegranteDelGrupo => "un compañero no actúa independientemente del grupo",
        })
    }
}
impl Error for ErrorAcampada {}

pub fn iniciar_acampada(mundo: &Mundo, actor: IdActor) -> Result<Acampada, ErrorAcampada> {
    let actual = mundo.actor(actor).ok_or(ErrorAcampada::ActorDesconocido)?;
    if actual.caracteristicas().any(|c| c.como_str() == "muerto") {
        return Err(ErrorAcampada::ActorMuerto);
    }
    if actor != mundo.actor_jugador() && mundo.grupo().contiene(actor) {
        return Err(ErrorAcampada::IntegranteDelGrupo);
    }
    if actual.lugar().is_none() {
        return Err(ErrorAcampada::ActorFueraDelEscenario);
    }
    Ok(Acampada { actor })
}

pub fn finalizar_acampada(acampada: Acampada) -> ResultadoAcampada {
    let Acampada { actor: _actor } = acampada;
    ResultadoAcampada {
        tiempo: TiempoAccion {
            duracion: Duracion::desde_dias(1),
            fraccion: FraccionDia::CERO,
        },
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{
        ActorInicial, Caracteristica, DatosInicialesMundo, Fecha, IdLugar, IdMapa, LugarInicial,
        Mes,
    };

    #[test]
    fn valida_actores_y_cierra_sin_modificar_el_mundo() {
        let id = |i| IdActor::desde_posicion(i).unwrap();
        let lugar = IdLugar::desde_posicion(0).unwrap();
        let mundo = Mundo::nuevo(DatosInicialesMundo {
            mapa: IdMapa::desde_posicion(0).unwrap(),
            fecha: Fecha::nueva(2026, Mes::Enero, 1).unwrap(),
            lugares: vec![LugarInicial {
                id: lugar,
                caracteristicas: vec![],
            }],
            actores: (0..4)
                .map(|i| ActorInicial {
                    id: id(i),
                    ubicacion: crate::UbicacionActor::EnLugar(lugar),
                    caracteristicas: if i == 3 {
                        vec![Caracteristica::nueva("muerto".into()).unwrap()]
                    } else {
                        vec![]
                    },
                })
                .collect(),
            actor_jugador: id(0),
            companeros: vec![id(1)],
            variables_globales: vec![],
            relaciones: vec![],
        })
        .unwrap();
        let antes = format!("{mundo:?}");
        assert_eq!(
            iniciar_acampada(&mundo, id(99)).unwrap_err(),
            ErrorAcampada::ActorDesconocido
        );
        assert_eq!(
            iniciar_acampada(&mundo, id(3)).unwrap_err(),
            ErrorAcampada::ActorMuerto
        );
        assert_eq!(
            iniciar_acampada(&mundo, id(1)).unwrap_err(),
            ErrorAcampada::IntegranteDelGrupo
        );
        for actor in [id(0), id(2)] {
            let fin = finalizar_acampada(iniciar_acampada(&mundo, actor).unwrap());
            assert_eq!(fin.tiempo.duracion, Duracion::desde_dias(1));
            assert_eq!(fin.tiempo.fraccion, FraccionDia::CERO);
        }
        assert_eq!(format!("{mundo:?}"), antes);
    }
}
