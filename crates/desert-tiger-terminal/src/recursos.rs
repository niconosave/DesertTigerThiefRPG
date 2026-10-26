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
const VERDE_OSCURO: Color = Color::Rgb(32, 96, 78);

#[derive(Clone, Copy, Debug)]
pub(crate) struct ConfiguracionFogata {
    luz: [u8; 3],
    intensidad_luz: f32,
    radio_luz: u16,
}

impl ConfiguracionFogata {
    pub(crate) fn luz(&self) -> [u8; 3] {
        self.luz
    }
    pub(crate) fn intensidad_luz(&self) -> f32 {
        self.intensidad_luz
    }
    pub(crate) fn radio_luz(&self) -> u16 {
        self.radio_luz
    }
}

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

struct PatronTile {
    ancho: usize,
    alto: usize,
    fotogramas: Vec<Vec<char>>,
    duracion_fotograma_ms: u64,
    estilo: Style,
}

impl PatronTile {
    fn en(&self, x: usize, y: usize, paso_animacion: usize) -> AparienciaTile {
        let transcurrido_ms = paso_animacion as u128 * crate::PASO_ANIMACION.as_millis();
        let indice = (transcurrido_ms / u128::from(self.duracion_fotograma_ms)
            % self.fotogramas.len() as u128) as usize;
        AparienciaTile {
            glifo: self.fotogramas[indice][(y % self.alto) * self.ancho + x % self.ancho],
            estilo: self.estilo,
        }
    }
}

pub(crate) struct RecursosVisuales {
    pub(crate) paleta: crate::colores::PaletaColores,
    reserva: AparienciaTile,
    apariencias: BTreeMap<ClaveTipoTile, PatronTile>,
    pub ui: PaletaUi,
    escenas: Vec<(ClaveEscena, Vec<String>)>,
    pub(crate) fogata: Vec<String>,
    pub(crate) estilo_fogata: Style,
    pub(crate) efecto_fogata: Option<ConfiguracionFogata>,
}

impl RecursosVisuales {
    pub(crate) fn cargar(raiz: &Path) -> Result<Self, ErrorRecursos> {
        let tiles: TilesDto = leer_ron(&raiz.join("tiles.ron"))?;
        let ui: UiDto = leer_ron(&raiz.join("ui.ron"))?;
        let escenas: EscenasDto = leer_ron(&raiz.join("scenes.ron"))?;
        let campamento: CampamentoDto = leer_ron(&raiz.join("campamento.ron"))
            .map_err(|e| ErrorRecursos(format!("{e} (fogata, incluido fogata.efecto)")))?;
        let efecto_fogata = campamento.fogata.efecto.map(convertir_efecto).transpose()?;
        let fogata = normalizar_arte(&campamento.fogata.arte)
            .map_err(|e| ErrorRecursos(format!("campamento.ron: {e}")))?;
        let estilo_fogata = convertir_estilo(campamento.fogata.estilo)
            .map_err(|e| ErrorRecursos(format!("campamento.ron: {e}")))?;
        let reserva = convertir_apariencia(tiles.reserva)?;
        if reserva.glifo != '?' {
            return Err(ErrorRecursos(
                "la apariencia de reserva debe usar '?'".to_owned(),
            ));
        }
        let mut apariencias = BTreeMap::new();
        for (texto, dto) in tiles.apariencias.0 {
            let patron = convertir_patron(dto)
                .map_err(|error| ErrorRecursos(format!("tiles.ron, tipo {texto}: {error}")))?;
            let clave = ClaveTipoTile::nueva(texto)
                .map_err(|error| ErrorRecursos(format!("clave de tile inválida: {error}")))?;
            if apariencias.insert(clave, patron).is_some() {
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
            paleta: crate::colores::PaletaColores::default(),
            reserva,
            apariencias,
            ui: convertir_ui(ui)?,
            escenas: ilustraciones,
            fogata,
            estilo_fogata,
            efecto_fogata,
        })
    }

    pub(crate) fn alto_fogata(&self) -> usize {
        if self.efecto_fogata.is_some() {
            usize::from(crate::fogata::ALTO)
        } else {
            self.fogata.len()
        }
    }

    pub(crate) fn tile(
        &self,
        clave: &ClaveTipoTile,
        x: usize,
        y: usize,
        paso_animacion: usize,
    ) -> AparienciaTile {
        self.apariencias
            .get(clave)
            .map(|patron| patron.en(x, y, paso_animacion))
            .unwrap_or_else(|| self.reserva.clone())
    }

    pub(crate) fn escena(&self, clave: ClaveEscena) -> Option<&[String]> {
        self.escenas
            .iter()
            .find_map(|(actual, arte)| (*actual == clave).then_some(arte.as_slice()))
    }

    pub(crate) fn contiene_tile(&self, clave: &ClaveTipoTile) -> bool {
        self.apariencias.contains_key(clave)
    }

    #[cfg(test)]
    pub(crate) fn con_fotogramas_prueba(
        mut self,
        clave: ClaveTipoTile,
        fotogramas: &[&str],
    ) -> Self {
        let patron = convertir_patron(DefinicionTileDto {
            glifo: None,
            patron: None,
            fotogramas: Some(fotogramas.iter().map(|f| (*f).to_owned()).collect()),
            duracion_fotograma_ms: 500,
            estilo: EstiloDto {
                frente: "verde".to_owned(),
                fondo: Some("negro".to_owned()),
                modificadores: vec!["negrita".to_owned()],
            },
        })
        .expect("patrón controlado válido");
        self.apariencias.insert(clave, patron);
        self
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

struct MapaAparienciasDto(Vec<(String, DefinicionTileDto)>);

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
struct DefinicionTileDto {
    #[serde(default, deserialize_with = "campo_presente")]
    glifo: Option<char>,
    #[serde(default, deserialize_with = "campo_presente")]
    patron: Option<String>,
    #[serde(default, deserialize_with = "campo_presente")]
    fotogramas: Option<Vec<String>>,
    #[serde(default = "duracion_predeterminada")]
    duracion_fotograma_ms: u64,
    estilo: EstiloDto,
}

fn duracion_predeterminada() -> u64 {
    500
}

fn campo_presente<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
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

fn convertir_patron(dto: DefinicionTileDto) -> Result<PatronTile, ErrorRecursos> {
    let textos = match (dto.glifo, dto.patron, dto.fotogramas) {
        (Some(glifo), None, None) => vec![glifo.to_string()],
        (None, Some(patron), None) => vec![patron],
        (None, None, Some(fotogramas)) => fotogramas,
        _ => {
            return Err(ErrorRecursos(
                "se requiere exactamente uno de glifo, patron o fotogramas".to_owned(),
            ));
        }
    };
    if textos.is_empty() {
        return Err(ErrorRecursos("fotogramas no puede estar vacío".to_owned()));
    }
    if dto.duracion_fotograma_ms == 0
        || u128::from(dto.duracion_fotograma_ms) % crate::PASO_ANIMACION.as_millis() != 0
    {
        return Err(ErrorRecursos(
            "duracion_fotograma_ms debe ser un múltiplo positivo de 250".to_owned(),
        ));
    }
    let mut ancho = 0;
    let mut alto = 0;
    let mut fotogramas = Vec::with_capacity(textos.len());
    for (indice, texto) in textos.iter().enumerate() {
        let filas = normalizar_arte(texto)
            .map_err(|error| ErrorRecursos(format!("fotograma {}: {error}", indice + 1)))?;
        let dimensiones = (filas[0].chars().count(), filas.len());
        if indice == 0 {
            (ancho, alto) = dimensiones;
        } else if dimensiones != (ancho, alto) {
            return Err(ErrorRecursos(format!(
                "fotograma {}: todos los fotogramas deben tener el mismo ancho y alto",
                indice + 1
            )));
        }
        fotogramas.push(filas.iter().flat_map(|fila| fila.chars()).collect());
    }
    Ok(PatronTile {
        ancho,
        alto,
        fotogramas,
        duracion_fotograma_ms: dto.duracion_fotograma_ms,
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
        "verdeOscuro" => Ok(VERDE_OSCURO),
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
            fila.chars().count() != filas[0].chars().count()
                || !fila
                    .chars()
                    .all(|c| !c.is_control() && c.width() == Some(1))
        })
    {
        return Err(ErrorRecursos(
            "el arte debe ser rectangular y usar glifos de una columna".to_owned(),
        ));
    }
    Ok(filas)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn patron_desde_ron(texto: &str) -> Result<PatronTile, ErrorRecursos> {
        let dto = ron::from_str(texto).map_err(|error| ErrorRecursos(error.to_string()))?;
        convertir_patron(dto)
    }

    #[test]
    fn patron_repite_glifos_y_estilo_y_admite_el_formato_de_glifo_unico() {
        let patron = patron_desde_ron(
            r#"(patron: "ab\ncd", estilo: (frente: "rojo", fondo: Some("negro"), modificadores: ["tenue"]))"#,
        )
        .unwrap();
        for (x, y, glifo) in [
            (0, 0, 'a'),
            (1, 0, 'b'),
            (0, 1, 'c'),
            (1, 1, 'd'),
            (4, 6, 'a'),
            (usize::MAX, usize::MAX, 'd'),
        ] {
            let celda = patron.en(x, y, 0);
            assert_eq!(celda.glifo, glifo);
            assert_eq!(celda.estilo.fg, Some(Color::Red));
            assert_eq!(celda.estilo.bg, Some(NEGRO_PURO));
            assert_eq!(celda.estilo.add_modifier, Modifier::DIM);
        }
        let fijo = patron_desde_ron(
            r#"(glifo: '♣', estilo: (frente: "verde", fondo: None, modificadores: []))"#,
        )
        .unwrap();
        assert_eq!(fijo.en(0, 0, 0).glifo, '♣');
        assert_eq!(fijo.en(19, 21, usize::MAX).glifo, '♣');
    }

    #[test]
    fn patrones_rechazan_campos_ambiguos_y_glifos_de_ancho_incompatible() {
        for campos in [
            "",
            r#"glifo: 'x', patron: "ab","#,
            r#"glifo: 'x', glifo: 'y',"#,
            r#"patron: "","#,
            r#"patron: "ab\nc","#,
            r#"patron: "界","#,
            r#"patron: "e\u{0301}","#,
            r#"patron: "a\tb","#,
            r#"fotogramas: [],"#,
            r#"fotogramas: [""],"#,
            r#"fotogramas: ["ab", "c"],"#,
            r#"fotogramas: ["ab\ncd", "ef"],"#,
            r#"fotogramas: ["ab", "界x"],"#,
            r#"glifo: 'x', fotogramas: ["ab"],"#,
            r#"patron: "ab", fotogramas: ["ab"],"#,
            r#"fotogramas: ["a", "b"], duracion_fotograma_ms: 0,"#,
            r#"fotogramas: ["a", "b"], duracion_fotograma_ms: 125,"#,
            r#"fotogramas: ["a", "b"], duracion_fotograma_ms: 375,"#,
        ] {
            let texto =
                format!("({campos} estilo: (frente: \"blanco\", fondo: None, modificadores: []))");
            assert!(patron_desde_ron(&texto).is_err(), "aceptó {texto}");
        }
    }

    #[test]
    fn animacion_repite_el_ciclo_y_cada_tipo_conserva_su_velocidad() {
        let rapido = patron_desde_ron(
            r#"(fotogramas: ["ab\ncd", "ef\ngh", "ij\nkl"], estilo: (frente: "azul", fondo: Some("negro"), modificadores: []))"#,
        )
        .unwrap();
        for (paso, esperado) in [(0, 'a'), (1, 'a'), (2, 'e'), (3, 'e'), (4, 'i'), (6, 'a')] {
            assert_eq!(rapido.en(0, 0, paso).glifo, esperado);
        }
        assert_eq!(rapido.en(5, 5, 2).glifo, 'h');
        let lento = patron_desde_ron(
            r#"(fotogramas: ["a", "b"], duracion_fotograma_ms: 1000, estilo: (frente: "azul", fondo: None, modificadores: []))"#,
        )
        .unwrap();
        assert_eq!(lento.en(0, 0, 2).glifo, 'a');
        assert_eq!(lento.en(0, 0, 4).glifo, 'b');
        assert_eq!(lento.en(0, 0, 8).glifo, 'a');
        let estatico = patron_desde_ron(
            r#"(patron: "ab\ncd", estilo: (frente: "verde", fondo: None, modificadores: []))"#,
        )
        .unwrap();
        assert_eq!(estatico.en(0, 0, usize::MAX).glifo, 'a');
    }

    #[test]
    fn tipo_desconocido_conserva_la_reserva_en_cualquier_coordenada() {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal");
        let recursos = RecursosVisuales::cargar(&raiz).unwrap();
        let clave = ClaveTipoTile::nueva("terrenoDeUnMod".to_owned()).unwrap();
        assert!(!recursos.contiene_tile(&clave));
        for (x, y) in [(0, 0), (37, 89), (usize::MAX, usize::MAX)] {
            let tile = recursos.tile(&clave, x, y, usize::MAX);
            assert_eq!(tile.glifo, '?');
            assert_eq!(tile.estilo.bg, None);
            assert!(tile.estilo.add_modifier.contains(Modifier::REVERSED));
        }
    }

    #[test]
    fn ilustraciones_admiten_bloques_de_una_columna_y_rechazan_ancho_incompatible() {
        assert_eq!(normalizar_arte("\n██\nxx\n").unwrap(), vec!["██", "xx"]);
        assert_eq!(normalizar_arte("░▓\r\n▌▐\r\n").unwrap(), vec!["░▓", "▌▐"]);
        for arte in ["", "██\nx", "界", "e\u{0301}", "a\tb", "\u{1b}[31m", "a\rb"] {
            assert!(normalizar_arte(arte).is_err(), "aceptó {arte:?}");
        }
    }

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
        assert_eq!(color("verdeOscuro").unwrap(), Color::Rgb(32, 96, 78));
    }

    #[test]
    fn efecto_fogata_es_opcional_y_valida_limites_con_contexto() {
        let original = include_str!("../../../assets/terminal/campamento.ron");
        let oficial: CampamentoDto = ron::from_str(original).unwrap();
        oficial
            .fogata
            .efecto
            .map(convertir_efecto)
            .transpose()
            .unwrap();
        let ejemplo =
            ron::from_str("(luz: (r: 255, g: 180, b: 72), intensidad_luz: 0.45, radio_luz: 28)")
                .unwrap();
        let config = convertir_efecto(ejemplo).unwrap();
        assert_eq!(config.luz(), [255, 180, 72]);
        assert_eq!(config.radio_luz(), 28);
        for campo in ["", "efecto: None,"] {
            let texto = format!(
                r#"(fogata: (arte: "x", estilo: (frente: "amarillo", fondo: None, modificadores: []), {campo}))"#
            );
            let dto: CampamentoDto = ron::from_str(&texto).unwrap();
            assert!(dto.fogata.efecto.is_none());
        }
        for (intensidad, radio, rojo, campo) in [
            ("-0.1", 28, 255, "intensidad_luz"),
            ("1.1", 28, 255, "intensidad_luz"),
            ("NaN", 28, 255, "intensidad_luz"),
            ("inf", 28, 255, "intensidad_luz"),
            ("0.45", 0, 255, "radio_luz"),
            ("0.45", 129, 255, "radio_luz"),
            ("0.45", 28, 256, "luz.r"),
        ] {
            let texto = format!(
                "(luz: (r: {rojo}, g: 180, b: 72), intensidad_luz: {intensidad}, radio_luz: {radio})"
            );
            let dto = ron::from_str(&texto).unwrap();
            let error = convertir_efecto(dto).unwrap_err().to_string();
            assert!(
                error.contains(&format!("campamento.ron: fogata.efecto.{campo}")),
                "{error}"
            );
        }
        for (intensidad, radio) in [(0.0, 1), (1.0, 128)] {
            assert!(
                convertir_efecto(EfectoFogataDto {
                    luz: RgbFogataDto {
                        r: 0,
                        g: 255,
                        b: 255
                    },
                    intensidad_luz: intensidad,
                    radio_luz: radio,
                })
                .is_ok()
            );
        }
        for texto in [
            "(luz: (r: 1, g: 2, b: 3), radio_luz: 28)",
            "(luz: (r: 1, g: 2), intensidad_luz: 0.5, radio_luz: 28)",
        ] {
            assert!(ron::from_str::<EfectoFogataDto>(texto).is_err());
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CampamentoDto {
    fogata: FogataDto,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FogataDto {
    arte: String,
    estilo: EstiloDto,
    #[serde(default)]
    efecto: Option<EfectoFogataDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EfectoFogataDto {
    luz: RgbFogataDto,
    intensidad_luz: f32,
    radio_luz: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RgbFogataDto {
    r: u16,
    g: u16,
    b: u16,
}

fn convertir_efecto(dto: EfectoFogataDto) -> Result<ConfiguracionFogata, ErrorRecursos> {
    let error =
        |campo, detalle| ErrorRecursos(format!("campamento.ron: fogata.efecto.{campo}: {detalle}"));
    let canal =
        |valor, nombre| u8::try_from(valor).map_err(|_| error(nombre, "debe estar entre 0 y 255"));
    let luz = [
        canal(dto.luz.r, "luz.r")?,
        canal(dto.luz.g, "luz.g")?,
        canal(dto.luz.b, "luz.b")?,
    ];
    if !dto.intensidad_luz.is_finite() || !(0.0..=1.0).contains(&dto.intensidad_luz) {
        return Err(error(
            "intensidad_luz",
            "debe ser finita y estar entre 0 y 1",
        ));
    }
    if !(1..=128).contains(&dto.radio_luz) {
        return Err(error("radio_luz", "debe estar entre 1 y 128"));
    }
    Ok(ConfiguracionFogata {
        luz,
        intensidad_luz: dto.intensidad_luz,
        radio_luz: dto.radio_luz,
    })
}
