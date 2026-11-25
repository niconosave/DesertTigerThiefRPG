//! Elementos físicos locales y sus límites observables.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AparienciaLocal {
    Puerta,
    Obstaculo,
    Indefinida,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnvolventeLocal {
    pub min: Posicion,
    pub max: Posicion,
}

impl ElementoLocal {
    pub fn envolvente(&self) -> Option<EnvolventeLocal> {
        Some(EnvolventeLocal {
            min: self.posicion,
            max: Posicion {
                z: self.posicion.z.checked_add(self.alto.checked_sub(1)?)?,
                ..self.posicion
            },
        })
    }
    pub fn apariencia(&self) -> AparienciaLocal {
        match self.clase {
            ClaseElemento::PuertaManual { .. } => AparienciaLocal::Puerta,
            ClaseElemento::Obstaculo => AparienciaLocal::Obstaculo,
            ClaseElemento::PresenciaProbable => AparienciaLocal::Indefinida,
        }
    }
}
impl Celda {
    pub fn material(&self) -> Option<MaterialLocal> {
        match self {
            Self::Suelo | Self::Muro => Some(MaterialLocal::Piedra),
            Self::Solido(m) => Some(*m),
            _ => None,
        }
    }
}
