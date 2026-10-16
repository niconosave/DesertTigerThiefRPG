use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::Path;

use desert_tiger_app::{ClaveEscena, ClaveTipoTile};
use ratatui::style::{Color, Modifier, Style};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use unicode_width::UnicodeWidthChar;

pub(crate) const NEGRO_PURO: Color = Color::Rgb(0, 0, 0);
const AZUL_OSCURO: Color = Color::Rgb(0, 24, 64);

#[derive(Debug)]
pub struct ErrorRecursos(String);

impl Display for ErrorRecursos {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for ErrorRecursos {}

#[derive(Clone)]
pub(crate) struct AparienciaTile {
    pub glifo: char,
    pub estilo: Style,
}

pub(crate) struct RecursosVisuales {
    reserva: AparienciaTile,
    apariencias: BTreeMap<ClaveTipoTile, AparienciaTile>,
    pub ui: PaletaUi,
    escenas: Vec<(ClaveEscena, Vec<String>)>,
}

impl RecursosVisuales {
    pub(crate) fn cargar(raiz: &Path) -> Result<Self, ErrorRecursos> {
        let tiles: TilesDto = leer_ron(&raiz.join("tiles.ron"))?;
        let ui: UiDto = leer_ron(&raiz.join("ui.ron"))?;
        let escenas: EscenasDto = leer_ron(&raiz.join("scenes.ron"))?;
        let reserva = convertir_apariencia(tiles.reserva)?;
        if reserva.glifo != '?' {
            return Err(ErrorRecursos(
                "la apariencia de reserva debe usar '?'".to_owned(),
            ));
        }
        let mut apariencias = BTreeMap::new();
        for (texto, dto) in tiles.apariencias.0 {
            let clave = ClaveTipoTile::nueva(texto)
                .map_err(|error| ErrorRecursos(format!("clave de tile inválida: {error}")))?;
            if apariencias
                .insert(clave, convertir_apariencia(dto)?)
                .is_some()
            {
                return Err(ErrorRecursos("clave de tile duplicada".to_owned()));
            }
        }
        let mut ilustraciones = Vec::new();
        for entrada in escenas.ilustraciones {
            let clave = escena_desde_texto(&entrada.escena)
                .ok_or_else(|| ErrorRecursos(format!("escena desconocida: {}", entrada.escena)))?;
            let arte = normalizar_arte(&entrada.arte)?;
            if ilustraciones
                .iter()
                .any(|(existente, _)| *existente == clave)
            {
                return Err(ErrorRecursos(format!(
                    "escena duplicada: {}",
                    entrada.escena
                )));
            }
            ilustraciones.push((clave, arte));
        }
        Ok(Self {
            reserva,
            apariencias,
            ui: convertir_ui(ui)?,
            escenas: ilustraciones,
        })
    }

    pub(crate) fn tile(&self, clave: &ClaveTipoTile) -> &AparienciaTile {
        self.apariencias.get(clave).unwrap_or(&self.reserva)
    }

    pub(crate) fn escena(&self, clave: ClaveEscena) -> Option<&[String]> {
        self.escenas
            .iter()
            .find_map(|(actual, arte)| (*actual == clave).then_some(arte.as_slice()))
    }

    pub(crate) fn contiene_tile(&self, clave: &ClaveTipoTile) -> bool {
        self.apariencias.contains_key(clave)
    }
}

pub(crate) struct PaletaUi {
    pub pantalla: Style,
    pub texto_normal: Style,
    pub destacado: Style,
    #[allow(dead_code)]
    pub secundario: Style,
    pub marco: Style,
    pub titulo: Style,
    pub foco: Style,
    #[allow(dead_code)]
    pub atajo: Style,
    pub error: Style,
    #[allow(dead_code)]
    pub deshabilitado: Style,
    pub barra_contextual: Style,
    pub barra_acciones: Style,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TilesDto {
    reserva: AparienciaDto,
    apariencias: MapaAparienciasDto,
}

struct MapaAparienciasDto(Vec<(String, AparienciaDto)>);

impl<'de> Deserialize<'de> for MapaAparienciasDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitante;
        impl<'de> Visitor<'de> for Visitante {
            type Value = MapaAparienciasDto;
            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("un mapa de apariencias")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut acceso: A) -> Result<Self::Value, A::Error> {
                let mut entradas = Vec::new();
                while let Some(entrada) = acceso.next_entry()? {
                    entradas.push(entrada);
                }
                Ok(MapaAparienciasDto(entradas))
            }
        }
        deserializer.deserialize_map(Visitante)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AparienciaDto {
    glifo: char,
    estilo: EstiloDto,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EstiloDto {
    frente: String,
    fondo: Option<String>,
    modificadores: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UiDto {
    pantalla: EstiloDto,
    texto_normal: EstiloDto,
    destacado: EstiloDto,
    secundario: EstiloDto,
    marco: EstiloDto,
    titulo: EstiloDto,
    foco: EstiloDto,
    atajo: EstiloDto,
    error: EstiloDto,
    deshabilitado: EstiloDto,
    barra_contextual: EstiloDto,
    barra_acciones: EstiloDto,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EscenasDto {
    ilustraciones: Vec<EscenaDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EscenaDto {
    escena: String,
    arte: String,
}

fn leer_ron<T: for<'de> Deserialize<'de>>(ruta: &Path) -> Result<T, ErrorRecursos> {
    let texto = fs::read_to_string(ruta)
        .map_err(|error| ErrorRecursos(format!("{}: {error}", ruta.display())))?;
    ron::from_str(&texto).map_err(|error| ErrorRecursos(format!("{}: {error}", ruta.display())))
}

fn convertir_apariencia(dto: AparienciaDto) -> Result<AparienciaTile, ErrorRecursos> {
    if dto.glifo.is_control() || UnicodeWidthChar::width(dto.glifo) != Some(1) {
        return Err(ErrorRecursos("glifo de tile inválido".to_owned()));
    }
    Ok(AparienciaTile {
        glifo: dto.glifo,
        estilo: convertir_estilo(dto.estilo)?,
    })
}

fn convertir_ui(dto: UiDto) -> Result<PaletaUi, ErrorRecursos> {
    Ok(PaletaUi {
        pantalla: convertir_estilo(dto.pantalla)?,
        texto_normal: convertir_estilo(dto.texto_normal)?,
        destacado: convertir_estilo(dto.destacado)?,
        secundario: convertir_estilo(dto.secundario)?,
        marco: convertir_estilo(dto.marco)?,
        titulo: convertir_estilo(dto.titulo)?,
        foco: convertir_estilo(dto.foco)?,
        atajo: convertir_estilo(dto.atajo)?,
        error: convertir_estilo(dto.error)?,
        deshabilitado: convertir_estilo(dto.deshabilitado)?,
        barra_contextual: convertir_estilo(dto.barra_contextual)?,
        barra_acciones: convertir_estilo(dto.barra_acciones)?,
    })
}

fn convertir_estilo(dto: EstiloDto) -> Result<Style, ErrorRecursos> {
    let mut vistos = BTreeSet::new();
    let mut modificadores = Modifier::empty();
    for texto in dto.modificadores {
        if !vistos.insert(texto.clone()) {
            return Err(ErrorRecursos(format!("modificador duplicado: {texto}")));
        }
        modificadores |= match texto.as_str() {
            "negrita" => Modifier::BOLD,
            "tenue" => Modifier::DIM,
            "invertido" => Modifier::REVERSED,
            _ => return Err(ErrorRecursos(format!("modificador desconocido: {texto}"))),
        };
    }
    if modificadores.contains(Modifier::BOLD) && modificadores.contains(Modifier::DIM) {
        return Err(ErrorRecursos(
            "negrita y tenue no pueden combinarse".to_owned(),
        ));
    }
    let mut estilo = Style::default()
        .fg(color(&dto.frente)?)
        .add_modifier(modificadores);
    if let Some(fondo) = dto.fondo {
        estilo = estilo.bg(color(&fondo)?);
    }
    Ok(estilo)
}

fn color(texto: &str) -> Result<Color, ErrorRecursos> {
    match texto {
        "negro" => Ok(NEGRO_PURO),
        "rojo" => Ok(Color::Red),
        "verde" => Ok(Color::Green),
        "amarillo" => Ok(Color::Yellow),
        "azul" => Ok(Color::Blue),
        "azulOscuro" => Ok(AZUL_OSCURO),
        "magenta" => Ok(Color::Magenta),
        "cian" => Ok(Color::Cyan),
        "gris" => Ok(Color::Gray),
        "grisOscuro" => Ok(Color::DarkGray),
        "rojoClaro" => Ok(Color::LightRed),
        "verdeClaro" => Ok(Color::LightGreen),
        "amarilloClaro" => Ok(Color::LightYellow),
        "azulClaro" => Ok(Color::LightBlue),
        "magentaClaro" => Ok(Color::LightMagenta),
        "cianClaro" => Ok(Color::LightCyan),
        "blanco" => Ok(Color::White),
        _ => Err(ErrorRecursos(format!("color desconocido: {texto}"))),
    }
}

fn escena_desde_texto(texto: &str) -> Option<ClaveEscena> {
    match texto {
        "viajeMarcha" => Some(ClaveEscena::ViajeMarcha),
        "viajeObstaculo" => Some(ClaveEscena::ViajeObstaculo),
        "viajeRumboIncierto" => Some(ClaveEscena::ViajeRumboIncierto),
        "viajeLlegada" => Some(ClaveEscena::ViajeLlegada),
        _ => None,
    }
}

fn normalizar_arte(texto: &str) -> Result<Vec<String>, ErrorRecursos> {
    let texto = texto.replace("\r\n", "\n");
    if texto.contains('\r') {
        return Err(ErrorRecursos("el arte contiene CR aislado".to_owned()));
    }
    let texto = texto.strip_prefix('\n').unwrap_or(&texto);
    let texto = texto.strip_suffix('\n').unwrap_or(texto);
    let filas: Vec<String> = texto.split('\n').map(str::to_owned).collect();
    if filas.is_empty()
        || filas[0].is_empty()
        || filas.iter().any(|fila| {
            fila.len() != filas[0].len() || !fila.chars().all(|c| c == ' ' || c.is_ascii_graphic())
        })
    {
        return Err(ErrorRecursos(
            "el arte de escena no es ASCII rectangular".to_owned(),
        ));
    }
    Ok(filas)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn apariencia(glifo: char) -> AparienciaDto {
        AparienciaDto {
            glifo,
            estilo: EstiloDto {
                frente: "blanco".to_owned(),
                fondo: None,
                modificadores: Vec::new(),
            },
        }
    }

    #[test]
    fn admite_glifos_unicode_de_una_celda_para_tiles() {
        assert_eq!(convertir_apariencia(apariencia('♣')).unwrap().glifo, '♣');
        assert_eq!(convertir_apariencia(apariencia('⛰')).unwrap().glifo, '⛰');
    }

    #[test]
    fn rechaza_glifos_de_ancho_incompatible_para_tiles() {
        for glifo in ['\n', '\u{0301}', '🍀'] {
            assert!(convertir_apariencia(apariencia(glifo)).is_err());
        }
    }

    #[test]
    fn colores_rgb_fijos_no_dependen_de_la_paleta_ansi() {
        assert_eq!(color("negro").unwrap(), Color::Rgb(0, 0, 0));
        assert_eq!(color("azulOscuro").unwrap(), Color::Rgb(0, 24, 64));
    }
}
