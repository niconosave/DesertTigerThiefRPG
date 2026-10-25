use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;

pub trait FuenteAzar {
    fn elegir_indice(&mut self, limite_exclusivo: NonZeroUsize) -> Result<usize, ErrorFuenteAzar>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ErrorFuenteAzar;

impl Display for ErrorFuenteAzar {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("la fuente de azar no pudo producir un índice")
    }
}

impl Error for ErrorFuenteAzar {}
