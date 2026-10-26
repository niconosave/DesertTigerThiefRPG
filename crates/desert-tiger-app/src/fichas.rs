use crate::contenido::ContenidoValidado;
use desert_tiger_core::{ExtremoRelacion, IdRelacion, Mundo, TipoRelacion};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFicha {
    pub elemento: ExtremoRelacion,
    pub nombre: String,
    pub biografia: Option<String>,
    pub entradas: Vec<EntradaFicha>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntradaFicha {
    Caracteristica {
        clave: String,
        nombre: String,
        descripcion: Option<String>,
    },
    Relacion(VistaRelacionFicha),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaRelacionFicha {
    pub id: IdRelacion,
    pub tipo: TipoRelacion,
    pub sentido: SentidoRelacionFicha,
    pub etiqueta: String,
    pub referencia: ResumenElemento,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SentidoRelacionFicha {
    Saliente,
    Entrante,
    Simetrica,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumenElemento {
    pub elemento: ExtremoRelacion,
    pub nombre: String,
    pub texto: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConsultaFicha {
    FaseIncompatible,
    CampamentoNoVigente,
    ElementoInexistente,
    EstadoInconsistente(String),
}
impl std::fmt::Display for ErrorConsultaFicha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FaseIncompatible => f.write_str("la consulta requiere un campamento abierto"),
            Self::CampamentoNoVigente => f.write_str("el campamento ya no está vigente"),
            Self::ElementoInexistente => f.write_str("el elemento no existe"),
            Self::EstadoInconsistente(s) => write!(f, "datos inconsistentes: {s}"),
        }
    }
}
impl std::error::Error for ErrorConsultaFicha {}

fn datos<'a>(
    mundo: &'a Mundo,
    contenido: &'a ContenidoValidado,
    elemento: ExtremoRelacion,
) -> Result<(&'a str, Option<&'a str>, Vec<&'a str>), ErrorConsultaFicha> {
    let falta =
        || ErrorConsultaFicha::EstadoInconsistente("falta la presentación del elemento".into());
    let mut resultado = match elemento {
        ExtremoRelacion::Actor(id) => {
            let actor = mundo
                .actor(id)
                .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
            let p = contenido.presentacion().actor(id).ok_or_else(falta)?;
            (
                p.nombre(),
                Some(p.biografia()),
                actor
                    .caracteristicas()
                    .map(|c| c.como_str())
                    .collect::<Vec<_>>(),
            )
        }
        ExtremoRelacion::Lugar(id) => {
            let lugar = mundo
                .lugar(id)
                .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
            let p = contenido.presentacion().lugar(id).ok_or_else(falta)?;
            (
                p.nombre(),
                None,
                lugar
                    .caracteristicas()
                    .map(|c| c.como_str())
                    .collect::<Vec<_>>(),
            )
        }
    };
    resultado.2.sort_unstable();
    Ok(resultado)
}
fn resumen(
    mundo: &Mundo,
    contenido: &ContenidoValidado,
    elemento: ExtremoRelacion,
) -> Result<ResumenElemento, ErrorConsultaFicha> {
    let (nombre, bio, caracteristicas) = datos(mundo, contenido, elemento)?;
    let texto = if let Some(bio) = bio {
        if bio.trim().is_empty() {
            "Sin biografía disponible".into()
        } else {
            let mut texto: String = bio.chars().take(160).collect();
            if bio.chars().nth(160).is_some() {
                texto.push('…');
            }
            texto
        }
    } else if caracteristicas.is_empty() {
        "Sin características".into()
    } else {
        let mut texto = caracteristicas
            .iter()
            .take(3)
            .map(|c| {
                contenido
                    .presentacion()
                    .caracteristicas
                    .get(*c)
                    .map(|d| d.nombre.as_str())
                    .unwrap_or(c)
            })
            .collect::<Vec<_>>()
            .join(", ");
        if caracteristicas.len() > 3 {
            texto.push_str(&format!(" y {} más", caracteristicas.len() - 3));
        }
        texto
    };
    Ok(ResumenElemento {
        elemento,
        nombre: nombre.into(),
        texto,
    })
}
pub(crate) fn proyectar(
    mundo: &Mundo,
    contenido: &ContenidoValidado,
    elemento: ExtremoRelacion,
) -> Result<VistaFicha, ErrorConsultaFicha> {
    let (nombre, bio, caracteristicas) = datos(mundo, contenido, elemento)?;
    let mut entradas = caracteristicas
        .into_iter()
        .map(|clave| {
            let descripcion = contenido.presentacion().caracteristicas.get(clave);
            EntradaFicha::Caracteristica {
                clave: clave.into(),
                nombre: descripcion
                    .map(|d| d.nombre.as_str())
                    .unwrap_or(clave)
                    .into(),
                descripcion: descripcion.map(|d| d.descripcion.clone()),
            }
        })
        .collect::<Vec<_>>();
    let mut ids = mundo
        .relaciones_de(elemento)
        .ok_or(ErrorConsultaFicha::ElementoInexistente)?
        .to_vec();
    ids.sort_unstable();
    for id in ids {
        let error = || {
            ErrorConsultaFicha::EstadoInconsistente("relación o extremo ausente del índice".into())
        };
        let relacion = mundo.relacion(id).ok_or_else(error)?;
        let otro = relacion.otro_extremo(elemento).ok_or_else(error)?;
        let referencia = resumen(mundo, contenido, otro)?;
        let (sentido, prefijo) = match relacion.tipo() {
            TipoRelacion::Conectado => (SentidoRelacionFicha::Simetrica, "Conectado con"),
            TipoRelacion::Conoce if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Conoce")
            }
            TipoRelacion::Conoce => (SentidoRelacionFicha::Entrante, "Conocido por"),
        };
        entradas.push(EntradaFicha::Relacion(VistaRelacionFicha {
            id,
            tipo: relacion.tipo(),
            sentido,
            etiqueta: format!("{prefijo} {}", referencia.nombre),
            referencia,
        }));
    }
    Ok(VistaFicha {
        elemento,
        nombre: nombre.into(),
        biografia: bio.map(str::to_owned),
        entradas,
    })
}
