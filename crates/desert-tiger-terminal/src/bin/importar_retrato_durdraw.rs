use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use serde::Deserialize;
use unicode_width::UnicodeWidthChar;

const CLAVES_PALETA: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!$%&()*+,-./:;<=>?@[]^_`{|}~";

#[derive(Debug)]
struct ErrorImportacion(String);

impl Display for ErrorImportacion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for ErrorImportacion {}

#[derive(Deserialize)]
struct ArchivoDurdraw {
    #[serde(rename = "DurMovie")]
    pelicula: PeliculaDurdraw,
}

#[derive(Deserialize)]
struct PeliculaDurdraw {
    #[serde(rename = "formatVersion")]
    version_formato: u32,
    #[serde(rename = "colorFormat")]
    formato_color: String,
    encoding: String,
    columns: usize,
    lines: usize,
    frames: Vec<FotogramaDurdraw>,
}

#[derive(Deserialize)]
struct FotogramaDurdraw {
    #[serde(rename = "frameNumber")]
    numero: usize,
    delay: f64,
    contents: Vec<String>,
    #[serde(rename = "colorMap")]
    colores: Vec<Vec<[u8; 2]>>,
}

struct FotogramaConvertido {
    arte: String,
    estilos: String,
}

fn main() {
    if let Err(error) = ejecutar(std::env::args()) {
        let _ = writeln!(io::stderr().lock(), "Error: {error}");
        std::process::exit(1);
    }
}

fn ejecutar<I, S>(argumentos: I) -> Result<(), ErrorImportacion>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut argumentos = argumentos.into_iter().map(Into::into);
    let programa = argumentos
        .next()
        .unwrap_or_else(|| "importar-retrato-durdraw".to_owned());
    let ruta = argumentos.next().ok_or_else(|| {
        ErrorImportacion(format!("uso: {programa} <archivo.dur> [--fotograma N]"))
    })?;
    let numero = match argumentos.next().as_deref() {
        None => 1,
        Some("--fotograma") => argumentos
            .next()
            .and_then(|n| n.parse::<usize>().ok())
            .filter(|&n| n > 0)
            .ok_or_else(|| {
                ErrorImportacion("--fotograma requiere un número positivo".to_owned())
            })?,
        _ => {
            return Err(ErrorImportacion(format!(
                "uso: {programa} <archivo.dur> [--fotograma N]"
            )));
        }
    };
    if argumentos.next().is_some() {
        return Err(ErrorImportacion(format!(
            "uso: {programa} <archivo.dur> [--fotograma N]"
        )));
    }
    let salida = convertir_archivo(Path::new(&ruta), numero)?;
    io::stdout()
        .lock()
        .write_all(salida.as_bytes())
        .map_err(|error| ErrorImportacion(format!("no se pudo escribir la salida: {error}")))
}

fn convertir_archivo(ruta: &Path, numero: usize) -> Result<String, ErrorImportacion> {
    let bytes =
        fs::read(ruta).map_err(|error| ErrorImportacion(format!("{}: {error}", ruta.display())))?;
    convertir_bytes(&bytes, numero)
}

fn convertir_bytes(bytes: &[u8], numero: usize) -> Result<String, ErrorImportacion> {
    let mut json = String::new();
    GzDecoder::new(bytes)
        .read_to_string(&mut json)
        .map_err(|error| ErrorImportacion(format!("gzip o UTF-8 inválido: {error}")))?;
    let archivo: ArchivoDurdraw = serde_json::from_str(&json)
        .map_err(|error| ErrorImportacion(format!("JSON Durdraw inválido: {error}")))?;
    convertir_pelicula(archivo.pelicula, numero)
}

fn convertir_pelicula(
    pelicula: PeliculaDurdraw,
    numero: usize,
) -> Result<String, ErrorImportacion> {
    if pelicula.version_formato != 8 {
        return Err(ErrorImportacion(format!(
            "versión Durdraw no admitida: {}; se requiere 8",
            pelicula.version_formato
        )));
    }
    if pelicula.formato_color != "16" {
        return Err(ErrorImportacion(format!(
            "formato de color no admitido: {}; se requiere 16",
            pelicula.formato_color
        )));
    }
    if pelicula.encoding != "utf-8" {
        return Err(ErrorImportacion(format!(
            "codificación no admitida: {}; se requiere utf-8",
            pelicula.encoding
        )));
    }
    if pelicula.columns == 0 || pelicula.lines == 0 {
        return Err(ErrorImportacion(
            "el lienzo Durdraw debe tener ancho y alto positivos".to_owned(),
        ));
    }
    if pelicula.frames.is_empty() {
        return Err(ErrorImportacion(
            "el archivo Durdraw no contiene fotogramas".to_owned(),
        ));
    }

    let mut paleta: Vec<[u8; 2]> = Vec::new();
    let mut fotogramas = Vec::with_capacity(pelicula.frames.len());
    for (indice, fotograma) in pelicula.frames.into_iter().enumerate() {
        let esperado = indice + 1;
        if fotograma.numero != esperado {
            return Err(ErrorImportacion(format!(
                "numeración de fotogramas inválida: se esperaba {esperado} y apareció {}",
                fotograma.numero
            )));
        }
        if fotograma.delay != 0.0 {
            return Err(ErrorImportacion(format!(
                "el fotograma {esperado} tiene una demora propia no admitida"
            )));
        }
        if fotograma.contents.len() != pelicula.lines || fotograma.colores.len() != pelicula.lines {
            return Err(ErrorImportacion(format!(
                "el fotograma {esperado} no tiene {} filas de arte y color",
                pelicula.lines
            )));
        }

        let mut arte = String::new();
        let mut estilos = String::new();
        for (y, (fila, colores)) in fotograma
            .contents
            .into_iter()
            .zip(fotograma.colores)
            .enumerate()
        {
            let glifos: Vec<char> = fila.chars().collect();
            if glifos.len() != pelicula.columns || colores.len() != pelicula.columns {
                return Err(ErrorImportacion(format!(
                    "el fotograma {esperado}, fila {y}, no tiene {} columnas",
                    pelicula.columns
                )));
            }
            for (x, glifo) in glifos.into_iter().enumerate() {
                if glifo.is_control() || glifo.width() != Some(1) {
                    return Err(ErrorImportacion(format!(
                        "glifo no admitido en fotograma {esperado}, ({x}, {y}): {}",
                        glifo.escape_default()
                    )));
                }
                arte.push(glifo);
            }
            for (x, par) in colores.into_iter().enumerate() {
                validar_par_color(par).map_err(|detalle| {
                    ErrorImportacion(format!("fotograma {esperado}, ({x}, {y}): {detalle}"))
                })?;
                let posicion =
                    if let Some(posicion) = paleta.iter().position(|actual| *actual == par) {
                        posicion
                    } else {
                        paleta.push(par);
                        paleta.len() - 1
                    };
                let codigo = CLAVES_PALETA.get(posicion).copied().ok_or_else(|| {
                    ErrorImportacion("el retrato requiere demasiados estilos distintos".to_owned())
                })?;
                estilos.push(char::from(codigo));
            }
            if y + 1 < pelicula.lines {
                arte.push('\n');
                estilos.push('\n');
            }
        }
        fotogramas.push(FotogramaConvertido { arte, estilos });
    }

    let elegido = numero
        .checked_sub(1)
        .and_then(|i| fotogramas.get(i))
        .ok_or_else(|| {
            ErrorImportacion(format!(
                "fotograma {numero} inexistente; el archivo tiene {}",
                fotogramas.len()
            ))
        })?;
    escribir_ron(&paleta, elegido)
}

fn validar_par_color([frente, fondo]: [u8; 2]) -> Result<(), String> {
    if frente > 16 {
        return Err(format!("color frontal fuera de 0..=16: {frente}"));
    }
    if fondo > 8 {
        return Err(format!("color de fondo fuera de 0..=8: {fondo}"));
    }
    Ok(())
}

fn escribir_ron(
    paleta: &[[u8; 2]],
    grafico: &FotogramaConvertido,
) -> Result<String, ErrorImportacion> {
    let mut salida = String::from("(\n    paleta: {\n");
    for (indice, [frente, fondo]) in paleta.iter().copied().enumerate() {
        let codigo = CLAVES_PALETA.get(indice).copied().ok_or_else(|| {
            ErrorImportacion("el retrato requiere demasiados estilos distintos".to_owned())
        })?;
        let frente = nombre_color_frontal_durdraw(frente)
            .ok_or_else(|| ErrorImportacion(format!("color frontal no admitido: {frente}")))?;
        let fondo = match nombre_fondo_durdraw(fondo) {
            Some(Some(nombre)) => format!("Some(\"{nombre}\")"),
            Some(None) => "None".to_owned(),
            None => {
                return Err(ErrorImportacion(format!(
                    "color de fondo no admitido: {fondo}"
                )));
            }
        };
        salida.push_str(&format!(
            "        '{}': (frente: \"{}\", fondo: {}),\n",
            char::from(codigo),
            frente,
            fondo
        ));
    }
    salida.push_str("    },\n    base: (\n        arte: ");
    salida.push_str(&literal_raw(&grafico.arte));
    salida.push_str(",\n        estilos: Some(");
    salida.push_str(&literal_raw(&grafico.estilos));
    salida.push_str("),\n    ),\n    rostro: None,\n    parpadeo: None,\n)\n");
    Ok(salida)
}

fn literal_raw(contenido: &str) -> String {
    let mut cantidad = 1;
    while contenido.contains(&format!("\"{}", "#".repeat(cantidad))) {
        cantidad += 1;
    }
    let marcas = "#".repeat(cantidad);
    format!("r{marcas}\"\n{contenido}\n\"{marcas}")
}

fn nombre_color_frontal_durdraw(indice: u8) -> Option<&'static str> {
    match indice {
        // Durdraw exporta negro con 0 o 1 en su paleta ANSI de 16 colores.
        0 | 1 => Some("negro"),
        2 => Some("azul"),
        3 => Some("verde"),
        4 => Some("cian"),
        5 => Some("rojo"),
        6 => Some("magenta"),
        7 => Some("amarillo"),
        8 => Some("gris"),
        9 => Some("grisOscuro"),
        10 => Some("azulClaro"),
        11 => Some("verdeClaro"),
        12 => Some("cianClaro"),
        13 => Some("rojoClaro"),
        14 => Some("magentaClaro"),
        15 => Some("amarilloClaro"),
        16 => Some("blanco"),
        17 => Some("grisMuyOscuro"),
        _ => None,
    }
}

fn nombre_fondo_durdraw(indice: u8) -> Option<Option<&'static str>> {
    match indice {
        0 | 8 => Some(None),
        1 => Some(Some("azul")),
        2 => Some(Some("verde")),
        3 => Some(Some("cian")),
        4 => Some(Some("rojo")),
        5 => Some(Some("magenta")),
        6 => Some(Some("amarillo")),
        7 => Some(Some("gris")),
        _ => None,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn convierte_el_retrato_oficial_de_forma_reproducible() {
        let bytes = include_bytes!("../../../../assets/principe.dur");
        let primera = convertir_bytes(bytes, 1).expect("fixture Durdraw válido");
        let segunda = convertir_bytes(bytes, 1).expect("segunda conversión válida");

        assert_eq!(primera, segunda);
        assert_eq!(primera.matches("arte:").count(), 1);
        assert!(primera.contains("base:"));
        assert!(!primera.contains("fotogramas:"));
        assert!(primera.contains('█'));
        assert!(primera.contains("frente: \"magentaClaro\""));
        assert!(primera.contains("'1': (frente: \"gris\", fondo: None)"));
        assert!(primera.contains("'7': (frente: \"grisOscuro\", fondo: Some(\"gris\"))"));
    }

    #[test]
    fn traduce_la_paleta_de_fondo_y_usa_negro_como_transparencia() {
        assert_eq!(nombre_color_frontal_durdraw(0), Some("negro"));
        assert_eq!(nombre_color_frontal_durdraw(1), Some("negro"));
        assert!(validar_par_color([0, 8]).is_ok());
        assert!(validar_par_color([17, 8]).is_err());
        assert_eq!(nombre_fondo_durdraw(0), Some(None));
        assert_eq!(nombre_fondo_durdraw(8), Some(None));
        assert_eq!(nombre_fondo_durdraw(1), Some(Some("azul")));
        assert_eq!(nombre_fondo_durdraw(4), Some(Some("rojo")));
        assert_eq!(nombre_fondo_durdraw(7), Some(Some("gris")));
        assert_eq!(nombre_fondo_durdraw(9), None);
        assert!(validar_par_color([8, 9]).is_err());
    }

    #[test]
    fn selecciona_un_grafico_base_sin_adivinar_el_parpadeo() {
        let bytes = include_bytes!("../../../../assets/principe.dur");
        let primero = convertir_bytes(bytes, 1).unwrap();
        let segundo = convertir_bytes(bytes, 2).unwrap();
        assert_ne!(primero, segundo);
        assert_eq!(segundo.matches("arte:").count(), 1);
        assert!(segundo.contains("parpadeo: None"));
        assert!(convertir_bytes(bytes, 0).is_err());
        assert!(convertir_bytes(bytes, 3).is_err());
    }

    #[test]
    fn rechaza_glifos_anchos_y_demoras_propias() {
        let pelicula = PeliculaDurdraw {
            version_formato: 8,
            formato_color: "16".to_owned(),
            encoding: "utf-8".to_owned(),
            columns: 1,
            lines: 1,
            frames: vec![FotogramaDurdraw {
                numero: 1,
                delay: 0.0,
                contents: vec!["界".to_owned()],
                colores: vec![vec![[8, 0]]],
            }],
        };
        assert!(convertir_pelicula(pelicula, 1).is_err());

        let pelicula = PeliculaDurdraw {
            version_formato: 8,
            formato_color: "16".to_owned(),
            encoding: "utf-8".to_owned(),
            columns: 1,
            lines: 1,
            frames: vec![FotogramaDurdraw {
                numero: 1,
                delay: 0.5,
                contents: vec!["x".to_owned()],
                colores: vec![vec![[8, 0]]],
            }],
        };
        assert!(convertir_pelicula(pelicula, 1).is_err());
    }
}
