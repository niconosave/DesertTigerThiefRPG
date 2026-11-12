use crate::contenido::ContenidoValidado;
use desert_tiger_core::{ExtremoRelacion, IdActor, IdLugar, IdRelacion, Mundo, TipoRelacion};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFicha {
    pub elemento: ExtremoRelacion,
    pub nombre: String,
    pub biografia: Option<String>,
    pub ubicacion: Option<VistaUbicacionActor>,
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
pub enum VistaUbicacionActor {
    EnLugar { lugar: IdLugar, nombre: String },
    FueraDelEscenario,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VinculoFicha {
    Registrado {
        id: IdRelacion,
        tipo: TipoRelacion,
        sentido: SentidoRelacionFicha,
    },
    Incorporacion,
    Integrante,
    Liderazgo,
    Hermandad {
        progenitores_compartidos: Vec<IdActor>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaRelacionFicha {
    pub vinculo: VinculoFicha,
    pub etiqueta: String,
    pub referencia: ResumenElemento,
    pub explicacion: Option<String>,
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
        ExtremoRelacion::Colectivo(id) => {
            let c = mundo
                .colectivo(id)
                .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
            let p = contenido.presentacion().colectivo(id).ok_or_else(falta)?;
            (
                p.nombre(),
                Some(p.descripcion()),
                c.caracteristicas().map(|c| c.como_str()).collect(),
            )
        }
        ExtremoRelacion::Concepto(id) => {
            mundo
                .relaciones_de(elemento)
                .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
            let p = contenido.presentacion().concepto(id).ok_or_else(falta)?;
            (p.nombre(), Some(p.descripcion()), Vec::new())
        }

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
                (!p.descripcion().is_empty()).then(|| p.descripcion()),
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
    let mut texto = if let Some(bio) = bio {
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
    if let ExtremoRelacion::Actor(id) = elemento
        && mundo.ubicacion_actor(id).is_none()
    {
        texto = format!("Fuera de la región. {texto}");
    }
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
            TipoRelacion::Desea if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Desea:")
            }
            TipoRelacion::Desea => (SentidoRelacionFicha::Entrante, "Deseado por:"),
            TipoRelacion::Debe if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Debe:")
            }
            TipoRelacion::Debe => (SentidoRelacionFicha::Entrante, "Deber de:"),
            TipoRelacion::Necesita if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Necesita:")
            }
            TipoRelacion::Necesita => (SentidoRelacionFicha::Entrante, "Necesario para:"),
            TipoRelacion::EsLlamadoA if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Es llamado a:")
            }
            TipoRelacion::EsLlamadoA => (SentidoRelacionFicha::Entrante, "Llama a:"),
            TipoRelacion::Gobierna if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Gobierna:")
            }
            TipoRelacion::Gobierna => (SentidoRelacionFicha::Entrante, "Gobernado por:"),
            TipoRelacion::PrisioneroDe if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Prisionero de:")
            }
            TipoRelacion::PrisioneroDe => (SentidoRelacionFicha::Entrante, "Tiene prisionero a:"),
            TipoRelacion::SalvadoPor if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Fue salvado por:")
            }
            TipoRelacion::SalvadoPor => (SentidoRelacionFicha::Entrante, "Salvó a:"),
            TipoRelacion::ComerciaCon if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Comercia con:")
            }
            TipoRelacion::ComerciaCon => (SentidoRelacionFicha::Entrante, "Recibe comercio de:"),
            TipoRelacion::EnemigoDe => (SentidoRelacionFicha::Simetrica, "Enemigo de:"),
            TipoRelacion::PactoCon => (SentidoRelacionFicha::Simetrica, "Tiene un pacto con:"),
            TipoRelacion::TratosCon => (SentidoRelacionFicha::Simetrica, "Tiene tratos con:"),

            TipoRelacion::EsclavoDe if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Esclavo de:")
            }
            TipoRelacion::EsclavoDe => (SentidoRelacionFicha::Entrante, "Tiene como esclavo a:"),
            TipoRelacion::Sirviente if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Sirviente de:")
            }
            TipoRelacion::Sirviente => (SentidoRelacionFicha::Entrante, "Servido por:"),
            TipoRelacion::Conectado => (SentidoRelacionFicha::Simetrica, "Conectado con"),
            TipoRelacion::Abarca if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Abarca:")
            }
            TipoRelacion::Abarca => (SentidoRelacionFicha::Entrante, "Dentro de:"),
            TipoRelacion::OrigenDe if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Origen de:")
            }
            TipoRelacion::OrigenDe => (SentidoRelacionFicha::Entrante, "Nace de:"),
            TipoRelacion::AfluenteDe if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Afluente de:")
            }
            TipoRelacion::AfluenteDe => (SentidoRelacionFicha::Entrante, "Recibe al afluente:"),
            TipoRelacion::Bloquea if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Bloquea:")
            }
            TipoRelacion::Bloquea => (SentidoRelacionFicha::Entrante, "Bloqueado por:"),
            TipoRelacion::Vende if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Vende a:")
            }
            TipoRelacion::Vende => (SentidoRelacionFicha::Entrante, "Compra a:"),
            TipoRelacion::Conoce if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Conoce")
            }
            TipoRelacion::Conoce => (SentidoRelacionFicha::Entrante, "Conocido por"),
            TipoRelacion::Padre if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Padre:")
            }
            TipoRelacion::Padre => (SentidoRelacionFicha::Entrante, "Padre de:"),
            TipoRelacion::Madre if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Madre:")
            }
            TipoRelacion::Madre => (SentidoRelacionFicha::Entrante, "Madre de:"),
            TipoRelacion::MatadoPor if relacion.origen() == elemento => {
                (SentidoRelacionFicha::Saliente, "Matado por:")
            }
            TipoRelacion::MatadoPor => (SentidoRelacionFicha::Entrante, "Mató a:"),
        };
        entradas.push(EntradaFicha::Relacion(VistaRelacionFicha {
            vinculo: VinculoFicha::Registrado {
                id,
                tipo: relacion.tipo(),
                sentido,
            },
            explicacion: None,
            etiqueta: if let Some(mercancia) = relacion.mercancia() {
                let verbo = if relacion.origen() == elemento {
                    "Vende"
                } else {
                    "Compra"
                };
                format!("{verbo} {mercancia} a: {}", referencia.nombre)
            } else {
                format!("{prefijo} {}", referencia.nombre)
            },
            referencia,
        }));
    }
    let mut enlace = |destino, vinculo, etiqueta: &str| -> Result<(), ErrorConsultaFicha> {
        let referencia = resumen(mundo, contenido, destino)?;
        entradas.push(EntradaFicha::Relacion(VistaRelacionFicha {
            vinculo,
            etiqueta: format!("{etiqueta}: {}", referencia.nombre),
            referencia,
            explicacion: None,
        }));
        Ok(())
    };
    match elemento {
        ExtremoRelacion::Actor(id) => {
            if let Some(c) = mundo.colectivo_de(id) {
                enlace(
                    ExtremoRelacion::Colectivo(c),
                    VinculoFicha::Incorporacion,
                    "Incorporado a",
                )?;
            }
        }
        ExtremoRelacion::Colectivo(id) => {
            let c = mundo
                .colectivo(id)
                .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
            enlace(
                ExtremoRelacion::Lugar(c.lugar()),
                VinculoFicha::Incorporacion,
                "Ubicación",
            )?;
            if let Some(l) = c.lider() {
                enlace(
                    ExtremoRelacion::Actor(l),
                    VinculoFicha::Liderazgo,
                    "Liderado por",
                )?;
            }
            for &a in mundo.miembros_colectivo(id).unwrap_or(&[]) {
                enlace(
                    ExtremoRelacion::Actor(a),
                    VinculoFicha::Integrante,
                    "Integrante",
                )?;
            }
        }
        _ => {}
    }
    let ubicacion = if let ExtremoRelacion::Actor(id) = elemento {
        let hermanos = mundo
            .hermanos_de(id)
            .ok_or(ErrorConsultaFicha::ElementoInexistente)?;
        for hermano in hermanos {
            let referencia = resumen(mundo, contenido, ExtremoRelacion::Actor(hermano.actor))?;
            let nombres = hermano
                .progenitores_compartidos
                .iter()
                .map(|&p| {
                    contenido
                        .presentacion()
                        .actor(p)
                        .map(|a| a.nombre())
                        .ok_or_else(|| {
                            ErrorConsultaFicha::EstadoInconsistente(
                                "progenitor sin presentación".into(),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let explicacion = format!(
                "{}: {}",
                if nombres.len() == 1 {
                    "Progenitor en común"
                } else {
                    "Progenitores en común"
                },
                nombres.join(", ")
            );
            entradas.push(EntradaFicha::Relacion(VistaRelacionFicha {
                vinculo: VinculoFicha::Hermandad {
                    progenitores_compartidos: hermano.progenitores_compartidos,
                },
                etiqueta: format!("Hermano/a de: {}", referencia.nombre),
                referencia,
                explicacion: Some(explicacion),
            }));
        }
        Some(match mundo.ubicacion_actor(id) {
            None => VistaUbicacionActor::FueraDelEscenario,
            Some(lugar) => VistaUbicacionActor::EnLugar {
                lugar,
                nombre: contenido
                    .presentacion()
                    .lugar(lugar)
                    .ok_or_else(|| {
                        ErrorConsultaFicha::EstadoInconsistente("ubicación sin presentación".into())
                    })?
                    .nombre()
                    .into(),
            },
        })
    } else {
        None
    };
    Ok(VistaFicha {
        ubicacion,
        elemento,
        nombre: nombre.into(),
        biografia: bio.map(str::to_owned),
        entradas,
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn comercio_se_explora_en_ambos_sentidos_y_muestra_descripciones_exteriores() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = c.instanciar_estado().unwrap();
        let lugar = |nombre| {
            c.presentacion()
                .lugares()
                .iter()
                .find(|l| l.nombre() == nombre)
                .unwrap()
                .id()
        };
        let zarabad = ExtremoRelacion::Lugar(lugar("Zarabad la Bienaventurada"));
        let compra = proyectar(&mundo, &c, zarabad)
            .unwrap()
            .entradas
            .into_iter()
            .find_map(|e| match e {
                EntradaFicha::Relacion(r) if r.etiqueta == "Compra cobre a: Vardanes" => Some(r),
                _ => None,
            })
            .unwrap();
        let vardanes = proyectar(&mundo, &c, compra.referencia.elemento).unwrap();
        assert!(
            vardanes
                .biografia
                .as_deref()
                .unwrap()
                .contains("al otro lado del mar")
        );
        assert!(vardanes.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r)
            if r.etiqueta == "Vende cobre a: Zarabad la Bienaventurada" && r.referencia.elemento == zarabad)));
        assert!(compra.referencia.texto.contains("Vardanes"));
    }

    #[test]
    fn fichas_permiten_explorar_abarca_y_dentro_de_en_ambos_sentidos() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = c.instanciar_estado().unwrap();
        let hamada = ExtremoRelacion::Lugar(IdLugar::desde_posicion(0).unwrap());
        let region = ExtremoRelacion::Lugar(IdLugar::desde_posicion(25).unwrap());
        let ficha_region = proyectar(&mundo, &c, region).unwrap();
        assert!(ficha_region.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if r.etiqueta == "Abarca: Hamada Roja" && r.referencia.elemento == hamada)));
        let ficha_hamada = proyectar(&mundo, &c, hamada).unwrap();
        assert!(ficha_hamada.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if r.etiqueta == "Dentro de: TIERRAS DE SAHRAN" && r.referencia.elemento == region)));
    }

    #[test]
    fn geografia_se_explora_en_ambos_sentidos_sin_crear_destinos_fluviales() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = c.instanciar_estado().unwrap();
        let id = |nombre: &str| {
            c.presentacion()
                .lugares()
                .iter()
                .find(|l| l.nombre() == nombre)
                .unwrap()
                .id()
        };
        for (tipo, origen, destino, ida, vuelta) in [
            (
                TipoRelacion::OrigenDe,
                "Monte Tazir",
                "Río Tazir",
                "Origen de:",
                "Nace de:",
            ),
            (
                TipoRelacion::OrigenDe,
                "Pico de Izir",
                "Río Izir",
                "Origen de:",
                "Nace de:",
            ),
            (
                TipoRelacion::AfluenteDe,
                "Río Izir",
                "Río Tamrut",
                "Afluente de:",
                "Recibe al afluente:",
            ),
            (
                TipoRelacion::AfluenteDe,
                "Río Tazir",
                "Río Tamrut",
                "Afluente de:",
                "Recibe al afluente:",
            ),
            (
                TipoRelacion::Bloquea,
                "Espinazo Gris",
                "Hamada de Zahrun",
                "Bloquea:",
                "Bloqueado por:",
            ),
            (
                TipoRelacion::Bloquea,
                "Risco de Adrar",
                "Hamada Roja",
                "Bloquea:",
                "Bloqueado por:",
            ),
            (
                TipoRelacion::Bloquea,
                "Alcázar Gris",
                "Valle Gris",
                "Bloquea:",
                "Bloqueado por:",
            ),
            (
                TipoRelacion::Bloquea,
                "Monte Tazir",
                "Valle del Drâa",
                "Bloquea:",
                "Bloqueado por:",
            ),
        ] {
            let mut ids = Vec::new();
            for (desde, hacia, prefijo, sentido) in [
                (origen, destino, ida, SentidoRelacionFicha::Saliente),
                (destino, origen, vuelta, SentidoRelacionFicha::Entrante),
            ] {
                let ficha = proyectar(&mundo, &c, ExtremoRelacion::Lugar(id(desde))).unwrap();
                let encontrados: Vec<_> = ficha
                    .entradas
                    .iter()
                    .filter_map(|e| match e {
                        EntradaFicha::Relacion(r) if r.etiqueta == format!("{prefijo} {hacia}") => {
                            Some(r)
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(encontrados.len(), 1, "{desde}: {prefijo} {hacia}");
                let r = encontrados[0];
                assert_eq!(r.referencia.elemento, ExtremoRelacion::Lugar(id(hacia)));
                match r.vinculo {
                    VinculoFicha::Registrado {
                        id,
                        tipo: t,
                        sentido: s,
                    } => {
                        assert_eq!((t, s), (tipo, sentido));
                        ids.push(id);
                    }
                    _ => panic!("la relación geográfica debe estar registrada"),
                }
            }
            assert_eq!(ids[0], ids[1], "ambas fichas consultan la misma relación");
        }
        let destinos =
            desert_tiger_core::acciones::destinos_disponibles(&mundo, mundo.actor_jugador())
                .unwrap();
        for nombre in ["Río Izir", "Río Tazir", "Río Tamrut"] {
            assert!(mundo.lugares_conectados(id(nombre)).unwrap().is_empty());
            assert!(!destinos.contains(&id(nombre)));
        }
    }

    #[test]
    fn puertos_se_exploran_desde_el_grupo_hasta_los_conceptos_y_la_familia() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = c.instanciar_estado().unwrap();
        let actor = |k| c.claves().actor_por_clave(k).unwrap();
        let ficha = |k| proyectar(&mundo, &c, ExtremoRelacion::Actor(actor(k))).unwrap();
        let relacion = |f: &VistaFicha, etiqueta: &str| {
            f.entradas
                .iter()
                .find_map(|e| match e {
                    EntradaFicha::Relacion(r) if r.etiqueta == etiqueta => {
                        Some(r.referencia.elemento)
                    }
                    _ => None,
                })
                .unwrap()
        };
        let rey = relacion(&ficha("bandido"), "Tiene tratos con: Zahrim");
        let rey = proyectar(&mundo, &c, rey).unwrap();
        assert_eq!(
            relacion(&rey, "Enemigo de: Miraz"),
            ExtremoRelacion::Actor(actor("miraz"))
        );
        let capitan = relacion(&rey, "Servido por: Dahran");
        let capitan = proyectar(&mundo, &c, capitan).unwrap();
        let deber = relacion(&capitan, "Debe: Proteger a su tripulación");
        let concepto = proyectar(&mundo, &c, deber).unwrap();
        assert!(concepto.ubicacion.is_none());
        assert!(concepto.biografia.as_deref().unwrap().contains("personas"));
        assert_eq!(relacion(&concepto, "Deber de: Dahran"), capitan.elemento);
        let libertad = relacion(&ficha("farid"), "Necesita: Recuperar la libertad");
        assert_eq!(
            relacion(&ficha("rima"), "Necesita: Recuperar la libertad"),
            libertad
        );
        let compartido = proyectar(&mundo, &c, libertad).unwrap();
        assert_eq!(compartido.entradas.len(), 2);
        assert!(
            mundo
                .hermanos_de(actor("farid"))
                .unwrap()
                .iter()
                .any(|h| h.actor == actor("samir"))
        );
        assert_eq!(
            relacion(&ficha("farid"), "Prisionero de: Dahran"),
            capitan.elemento
        );
        assert!(!ficha("bandido").entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if r.etiqueta == "Sirviente de: Príncipe Perdido")));
    }

    #[test]
    fn familia_del_principe_expone_hechos_y_deducciones_sin_ids_ficticios() {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, _) = c.instanciar_estado().unwrap();
        let (antes, _) = c.instanciar_estado().unwrap();
        let id = |i| IdActor::desde_posicion(i).unwrap();
        let ficha = |i| proyectar(&mundo, &c, ExtremoRelacion::Actor(id(i))).unwrap();
        assert_eq!(mundo.actores().len(), 21);
        {
            let actor = 3;
            let sirviente = ficha(actor);
            assert!(sirviente.entradas.iter().any(|e| matches!(e,
                EntradaFicha::Relacion(r) if r.etiqueta == "Sirviente de: Príncipe Perdido"
                && r.referencia.elemento == ExtremoRelacion::Actor(id(0))
                && matches!(r.vinculo, VinculoFicha::Registrado { tipo: TipoRelacion::Sirviente, sentido: SentidoRelacionFicha::Saliente, .. }))));
            assert!(ficha(0).entradas.iter().any(|e| matches!(e,
                EntradaFicha::Relacion(r) if r.referencia.elemento == ExtremoRelacion::Actor(id(actor))
                && r.etiqueta.starts_with("Servido por:")
                && matches!(r.vinculo, VinculoFicha::Registrado { tipo: TipoRelacion::Sirviente, sentido: SentidoRelacionFicha::Entrante, .. }))));
            assert!(mundo.hermanos_de(id(actor)).unwrap().is_empty());
        }
        assert!(ficha(0).biografia.unwrap().contains("exiliado"));
        assert!(
            !mundo
                .actor(id(0))
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "exiliado")
        );
        for _ in 0..50 {
            let rey = ficha(6);
            assert_eq!(rey.ubicacion, Some(VistaUbicacionActor::FueraDelEscenario));
            let al_hermano = rey
                .entradas
                .iter()
                .filter_map(|e| match e {
                    EntradaFicha::Relacion(r)
                        if r.referencia.elemento == ExtremoRelacion::Actor(id(8)) =>
                    {
                        Some(r)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(al_hermano.len(), 2);
            assert!(al_hermano.iter().any(|r| matches!(
                r.vinculo,
                VinculoFicha::Registrado {
                    tipo: TipoRelacion::MatadoPor,
                    sentido: SentidoRelacionFicha::Saliente,
                    ..
                }
            )));
            let derivado = al_hermano
                .iter()
                .find(|r| matches!(r.vinculo, VinculoFicha::Hermandad { .. }))
                .unwrap();
            assert_eq!(
                derivado.vinculo,
                VinculoFicha::Hermandad {
                    progenitores_compartidos: vec![id(9), id(10)]
                }
            );
            let explicacion = derivado.explicacion.as_ref().unwrap();
            assert!(
                explicacion.contains("Abuelo paterno") && explicacion.contains("Abuela paterna")
            );
            assert!(rey.entradas.iter().any(|e|matches!(e,EntradaFicha::Relacion(r) if r.etiqueta=="Padre de: Príncipe Perdido")));
            assert!(ficha(7).entradas.iter().any(|e|matches!(e,EntradaFicha::Relacion(r) if r.etiqueta=="Madre de: Príncipe Perdido")));
            let hermano = ficha(8);
            assert!(
                hermano
                    .entradas
                    .iter()
                    .any(|e| matches!(e,EntradaFicha::Relacion(r) if r.etiqueta=="Mató a: Rey"))
            );
        }
        assert_eq!(mundo, antes);
        assert_eq!(mundo.relaciones().len(), 266);
    }
}
