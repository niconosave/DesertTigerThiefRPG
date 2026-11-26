//! Diagnósticos locales tipados; la app decide cómo presentarlos.
use super::{ErrorGeometria, IdElementoLocal, IdFicha, Posicion};
use crate::ExtremoRelacion;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorEncuentro {
    PresupuestoDireccionAgotado,
    IdentidadesAgotadas {
        dominio: &'static str,
    },
    Geometria(ErrorGeometria),
    PosicionFueraDeLimites(Posicion),
    EstadoIncompatible {
        operacion: &'static str,
    },
    ReferenciaInvalida {
        referencia: ExtremoRelacion,
        detalle: &'static str,
    },
    ReferenciaLocal {
        ficha: IdFicha,
        elemento: IdElementoLocal,
    },
    GeneracionAgotada {
        intentos: u8,
        ultimo: Box<ErrorEncuentro>,
    },
    Azar(String),
    ReglaInvalida(String),
}
impl fmt::Display for ErrorEncuentro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentidadesAgotadas { dominio } => write!(f, "identidades de {dominio} agotadas"),
            Self::PresupuestoDireccionAgotado => f.write_str("presupuesto de dirección agotado"),
            Self::PosicionFueraDeLimites(p) => write!(f, "posición fuera del encuentro: {p:?}"),
            Self::Geometria(e) => write!(f, "geometría del encuentro: {e:?}"),
            Self::EstadoIncompatible { operacion } => {
                write!(f, "estado incompatible al {operacion}")
            }
            Self::ReferenciaInvalida {
                referencia,
                detalle,
            } => write!(f, "{detalle}: {referencia:?}"),
            Self::ReferenciaLocal { ficha, elemento } => {
                write!(f, "interacción local inválida: {ficha:?}, {elemento:?}")
            }
            Self::GeneracionAgotada { intentos, ultimo } => {
                write!(f, "generación agotada tras {intentos} intentos: {ultimo}")
            }
            Self::Azar(e) => write!(f, "azar del encuentro: {e}"),
            Self::ReglaInvalida(e) => f.write_str(e),
        }
    }
}
impl std::error::Error for ErrorEncuentro {}
impl From<ErrorGeometria> for ErrorEncuentro {
    fn from(e: ErrorGeometria) -> Self {
        Self::Geometria(e)
    }
}
impl From<&str> for ErrorEncuentro {
    fn from(e: &str) -> Self {
        Self::ReglaInvalida(e.into())
    }
}
impl From<String> for ErrorEncuentro {
    fn from(e: String) -> Self {
        Self::ReglaInvalida(e)
    }
}
