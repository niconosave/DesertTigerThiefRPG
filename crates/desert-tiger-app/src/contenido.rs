use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use desert_tiger_core::{
    ActorInicial, Caracteristica, ClaveVariableGlobal, DatosInicialesMundo, ExtremoRelacion, Fecha,
    IdActor, IdLugar, IdMapa, LugarInicial, Mes, Mundo, RelacionInicial, TipoRelacion,
    ValorVariableGlobal, VariableGlobalInicial,
};
use serde::de::{DeserializeOwned, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use unicode_width::UnicodeWidthChar;

use crate::lua::{PoliticaLuaValidada, ResolucionClavesLua, validar_politica};

pub const MAX_BYTES_CLAVE_TIPO_TILE: usize = 64;
const MAX_BYTES_CLAVE_EXTERNA: usize = 64;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaveTipoTile(String);

impl ClaveTipoTile {
    pub fn nueva(valor: String) -> Result<Self, ErrorClaveTipoTile> {
        if valor.len() > MAX_BYTES_CLAVE_TIPO_TILE {
            return Err(ErrorClaveTipoTile::DemasiadoLarga {
                longitud_bytes: valor.len(),
                maximo_bytes: MAX_BYTES_CLAVE_TIPO_TILE,
            });
        }
        if !es_lower_camel(&valor) {
            return Err(ErrorClaveTipoTile::SintaxisInvalida);
        }
        Ok(Self(valor))
    }

    pub fn como_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorClaveTipoTile {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}

impl Display for ErrorClaveTipoTile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SintaxisInvalida => formatter.write_str("sintaxis de tipo de tile inválida"),
            Self::DemasiadoLarga {
                longitud_bytes,
                maximo_bytes,
            } => write!(
                formatter,
                "tipo de tile demasiado largo: {longitud_bytes} bytes; máximo {maximo_bytes}"
            ),
        }
    }
}

impl Error for ErrorClaveTipoTile {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoordenadaMapa {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraficoRetrato {
    ancho: usize,
    alto: usize,
    celdas: Vec<CeldaRetrato>,
}

impl GraficoRetrato {
    pub fn ancho(&self) -> usize {
        self.ancho
    }

    pub fn alto(&self) -> usize {
        self.alto
    }

    pub fn celdas(&self) -> &[CeldaRetrato] {
        &self.celdas
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CeldaRetrato {
    pub glifo: char,
    pub estilo: Option<EstiloCeldaRetrato>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstiloCeldaRetrato {
    pub frente: ColorRetrato,
    pub fondo: Option<ColorRetrato>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorRetrato {
    Negro,
    Rojo,
    Verde,
    Amarillo,
    Azul,
    Magenta,
    Cian,
    Gris,
    GrisOscuro,
    GrisMuyOscuro,
    RojoClaro,
    VerdeClaro,
    AmarilloClaro,
    AzulClaro,
    MagentaClaro,
    CianClaro,
    Blanco,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetratoActor {
    base: GraficoRetrato,
    rostro: Option<RostroRetrato>,
    parpadeo: Option<ParpadeoRetrato>,
}

impl RetratoActor {
    pub fn base(&self) -> &GraficoRetrato {
        &self.base
    }

    pub fn rostro(&self) -> Option<&RostroRetrato> {
        self.rostro.as_ref()
    }

    pub fn parpadeo(&self) -> Option<&ParpadeoRetrato> {
        self.parpadeo.as_ref()
    }

    pub fn capas(
        &self,
        expresion: Option<&str>,
        parpadeando: bool,
    ) -> [Option<(PosicionRetrato, &GraficoRetrato)>; 3] {
        let rostro = self.rostro.as_ref().and_then(|rostro| {
            expresion.and_then(|clave| {
                rostro
                    .expresion(clave)
                    .map(|grafico| (rostro.origen, grafico))
            })
        });
        let ojo = self
            .parpadeo
            .as_ref()
            .filter(|_| parpadeando)
            .map(|parpadeo| (parpadeo.origen, &parpadeo.ojo_cerrado));
        [
            Some((PosicionRetrato { x: 0, y: 0 }, &self.base)),
            rostro,
            ojo,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PosicionRetrato {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RostroRetrato {
    origen: PosicionRetrato,
    expresiones: BTreeMap<String, GraficoRetrato>,
}

impl RostroRetrato {
    pub fn origen(&self) -> PosicionRetrato {
        self.origen
    }

    pub fn expresion(&self, clave: &str) -> Option<&GraficoRetrato> {
        self.expresiones.get(clave)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParpadeoRetrato {
    origen: PosicionRetrato,
    ojo_cerrado: GraficoRetrato,
    intervalo_ms: u64,
    duracion_ms: u64,
}

impl ParpadeoRetrato {
    pub fn origen(&self) -> PosicionRetrato {
        self.origen
    }

    pub fn ojo_cerrado(&self) -> &GraficoRetrato {
        &self.ojo_cerrado
    }

    pub fn intervalo_ms(&self) -> u64 {
        self.intervalo_ms
    }

    pub fn duracion_ms(&self) -> u64 {
        self.duracion_ms
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaseDiagnosticoContenido {
    AccesoFilesystem,
    RutaFueraDeRaiz,
    FormatoRonInvalido,
    ClaveInvalida,
    Duplicado,
    ReferenciaInvalida,
    ValorInvalido,
    PoliticaLuaInvalida,
    InvarianteDominio,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoElementoContenido {
    Mundo,
    Mapa,
    Lugar,
    Actor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrigenDiagnosticoContenido {
    Raiz { ruta: PathBuf },
    Archivo { ruta_relativa: PathBuf },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementoDiagnosticoContenido {
    pub tipo: TipoElementoContenido,
    pub clave: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PosicionDiagnosticoContenido {
    Fuente { linea: usize, columna: usize },
    Matriz { x: usize, y: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticoContenido {
    clase: ClaseDiagnosticoContenido,
    origen: OrigenDiagnosticoContenido,
    elemento: Option<ElementoDiagnosticoContenido>,
    posicion: Option<PosicionDiagnosticoContenido>,
    detalle: String,
}

impl DiagnosticoContenido {
    pub fn clase(&self) -> ClaseDiagnosticoContenido {
        self.clase
    }
    pub fn origen(&self) -> &OrigenDiagnosticoContenido {
        &self.origen
    }
    pub fn elemento(&self) -> Option<&ElementoDiagnosticoContenido> {
        self.elemento.as_ref()
    }
    pub fn posicion(&self) -> Option<PosicionDiagnosticoContenido> {
        self.posicion
    }
    pub fn detalle(&self) -> &str {
        &self.detalle
    }
}

#[derive(Debug)]
pub struct ErrorContenido {
    diagnosticos: Vec<DiagnosticoContenido>,
}

impl ErrorContenido {
    pub fn diagnosticos(&self) -> &[DiagnosticoContenido] {
        &self.diagnosticos
    }

    fn uno(diagnostico: DiagnosticoContenido) -> Self {
        Self {
            diagnosticos: vec![diagnostico],
        }
    }
}

impl Display for ErrorContenido {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "contenido inválido: {} diagnóstico(s)",
            self.diagnosticos.len()
        )
    }
}

impl Error for ErrorContenido {}

#[derive(Clone)]
pub struct ContenidoValidado {
    interno: Arc<ContenidoValidadoInterno>,
}

struct ContenidoValidadoInterno {
    plantilla_mundo: DatosInicialesMundo,
    presentacion: CatalogoPresentacion,
    claves: CatalogoClaves,
    politicas: Vec<PoliticaLuaValidada>,
}

impl ContenidoValidado {
    pub(crate) fn instanciar_estado(
        &self,
    ) -> Result<(Mundo, EstadoMapa), ErrorInstanciarContenido> {
        let mundo = Mundo::nuevo(self.interno.plantilla_mundo.clone())
            .map_err(ErrorInstanciarContenido::Mundo)?;
        let mapa = self.interno.presentacion.instanciar_mapa();
        if mundo.id_mapa() != mapa.id() {
            return Err(ErrorInstanciarContenido::MapaNoCorresponde {
                esperado: mundo.id_mapa(),
                encontrado: mapa.id(),
            });
        }
        Ok((mundo, mapa))
    }

    pub(crate) fn presentacion(&self) -> &CatalogoPresentacion {
        &self.interno.presentacion
    }
    pub(crate) fn claves(&self) -> &CatalogoClaves {
        &self.interno.claves
    }
    pub(crate) fn politicas(&self) -> &[PoliticaLuaValidada] {
        &self.interno.politicas
    }
}

#[derive(Debug)]
pub(crate) enum ErrorInstanciarContenido {
    Mundo(desert_tiger_core::ErrorConstruirMundo),
    MapaNoCorresponde {
        esperado: IdMapa,
        encontrado: IdMapa,
    },
}

impl Display for ErrorInstanciarContenido {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mundo(error) => write!(formatter, "el Mundo no pudo construirse: {error}"),
            Self::MapaNoCorresponde {
                esperado,
                encontrado,
            } => write!(
                formatter,
                "el mapa no corresponde al Mundo: esperado {esperado:?}, encontrado {encontrado:?}"
            ),
        }
    }
}

#[derive(Clone)]
struct PlantillaMapa {
    id: IdMapa,
    ancho: usize,
    alto: usize,
    celdas_iniciales: Vec<ClaveTipoTile>,
    tipos_declarados: BTreeSet<ClaveTipoTile>,
}

pub(crate) struct CatalogoPresentacion {
    mapa: PlantillaMapa,
    lugares: Vec<PresentacionLugar>,
    actores: Vec<PresentacionActor>,
}

pub(crate) struct PresentacionLugar {
    id: IdLugar,
    nombre: String,
    coordenada_central: CoordenadaMapa,
    coordenadas_rotulo: Vec<CoordenadaMapa>,
}

pub(crate) struct PresentacionActor {
    id: IdActor,
    retrato: Option<RetratoActor>,
}

impl CatalogoPresentacion {
    pub(crate) fn lugares(&self) -> &[PresentacionLugar] {
        &self.lugares
    }
    #[allow(dead_code)]
    pub(crate) fn actores(&self) -> &[PresentacionActor] {
        &self.actores
    }
    #[allow(dead_code)]
    pub(crate) fn lugar(&self, id: IdLugar) -> Option<&PresentacionLugar> {
        self.lugares.iter().find(|lugar| lugar.id == id)
    }
    pub(crate) fn actor(&self, id: IdActor) -> Option<&PresentacionActor> {
        self.actores.iter().find(|actor| actor.id == id)
    }
    fn instanciar_mapa(&self) -> EstadoMapa {
        EstadoMapa {
            id: self.mapa.id,
            ancho: self.mapa.ancho,
            alto: self.mapa.alto,
            celdas: self.mapa.celdas_iniciales.clone(),
            tipos_declarados: self.mapa.tipos_declarados.clone(),
        }
    }
}

impl PresentacionLugar {
    pub(crate) fn id(&self) -> IdLugar {
        self.id
    }
    pub(crate) fn nombre(&self) -> &str {
        &self.nombre
    }
    pub(crate) fn coordenada_central(&self) -> CoordenadaMapa {
        self.coordenada_central
    }
    pub(crate) fn coordenadas_rotulo(&self) -> &[CoordenadaMapa] {
        &self.coordenadas_rotulo
    }
}

impl PresentacionActor {
    #[allow(dead_code)]
    pub(crate) fn id(&self) -> IdActor {
        self.id
    }
    pub(crate) fn retrato(&self) -> Option<&RetratoActor> {
        self.retrato.as_ref()
    }
}

pub(crate) struct EstadoMapa {
    id: IdMapa,
    ancho: usize,
    alto: usize,
    celdas: Vec<ClaveTipoTile>,
    tipos_declarados: BTreeSet<ClaveTipoTile>,
}

impl EstadoMapa {
    pub(crate) fn id(&self) -> IdMapa {
        self.id
    }
    pub(crate) fn ancho(&self) -> usize {
        self.ancho
    }
    pub(crate) fn alto(&self) -> usize {
        self.alto
    }
    pub(crate) fn celdas(&self) -> &[ClaveTipoTile] {
        &self.celdas
    }
    pub(crate) fn tipos_declarados(&self) -> impl ExactSizeIterator<Item = &ClaveTipoTile> + '_ {
        self.tipos_declarados.iter()
    }
    #[allow(dead_code)]
    pub(crate) fn tipo_en(&self, coordenada: CoordenadaMapa) -> Option<&ClaveTipoTile> {
        if coordenada.x >= self.ancho || coordenada.y >= self.alto {
            return None;
        }
        let indice = coordenada
            .y
            .checked_mul(self.ancho)?
            .checked_add(coordenada.x)?;
        self.celdas.get(indice)
    }
    #[allow(dead_code)]
    pub(crate) fn cambiar_tipo(
        &mut self,
        coordenada: CoordenadaMapa,
        nuevo_tipo: ClaveTipoTile,
    ) -> Result<(), ErrorCambiarTile> {
        if coordenada.x >= self.ancho || coordenada.y >= self.alto {
            return Err(ErrorCambiarTile::CoordenadaFueraDeLimites {
                coordenada,
                ancho: self.ancho,
                alto: self.alto,
            });
        }
        if !self.tipos_declarados.contains(&nuevo_tipo) {
            return Err(ErrorCambiarTile::TipoNoDeclarado { tipo: nuevo_tipo });
        }
        let indice = coordenada
            .y
            .checked_mul(self.ancho)
            .and_then(|base| base.checked_add(coordenada.x))
            .ok_or(ErrorCambiarTile::CoordenadaFueraDeLimites {
                coordenada,
                ancho: self.ancho,
                alto: self.alto,
            })?;
        self.celdas[indice] = nuevo_tipo;
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ErrorCambiarTile {
    CoordenadaFueraDeLimites {
        coordenada: CoordenadaMapa,
        ancho: usize,
        alto: usize,
    },
    TipoNoDeclarado {
        tipo: ClaveTipoTile,
    },
}

pub(crate) struct CatalogoClaves {
    actores_por_clave: BTreeMap<String, IdActor>,
    claves_por_actor: BTreeMap<IdActor, String>,
    lugares_por_clave: BTreeMap<String, IdLugar>,
    claves_por_lugar: BTreeMap<IdLugar, String>,
}

impl CatalogoClaves {
    pub(crate) fn clave_actor(&self, actor: IdActor) -> Option<&str> {
        self.claves_por_actor.get(&actor).map(String::as_str)
    }

    pub(crate) fn clave_lugar(&self, lugar: IdLugar) -> Option<&str> {
        self.claves_por_lugar.get(&lugar).map(String::as_str)
    }
}

impl ResolucionClavesLua for CatalogoClaves {
    fn clave_actor(&self, actor: IdActor) -> Option<&str> {
        self.clave_actor(actor)
    }

    fn clave_lugar(&self, lugar: IdLugar) -> Option<&str> {
        self.clave_lugar(lugar)
    }
    fn lugar_por_clave(&self, clave: &str) -> Option<IdLugar> {
        self.lugares_por_clave.get(clave).copied()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MundoDto {
    mapa: String,
    lugares: Vec<String>,
    actores: Vec<String>,
    actor_jugador: String,
    companeros_iniciales: Vec<String>,
    fecha_inicial: FechaDto,
    variables_globales: VariablesGlobalesDto,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FechaDto {
    anio: u64,
    mes: u64,
    dia: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VariablesGlobalesDto {
    booleanas: Vec<GlobalBooleanaDto>,
    enteras: Vec<GlobalEnteraDto>,
    textos: Vec<GlobalTextoDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GlobalBooleanaDto {
    clave: String,
    valor: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GlobalEnteraDto {
    clave: String,
    valor: i128,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GlobalTextoDto {
    clave: String,
    valor: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MapaDto {
    clave: String,
    leyenda: LeyendaDto,
    matriz: String,
}

struct LeyendaDto {
    entradas: Vec<(char, String)>,
}

impl<'de> Deserialize<'de> for LeyendaDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitante;
        impl<'de> Visitor<'de> for Visitante {
            type Value = LeyendaDto;
            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("un mapa de símbolos a tipos visuales")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut acceso: A) -> Result<Self::Value, A::Error> {
                let mut entradas = Vec::new();
                while let Some(entrada) = acceso.next_entry::<char, String>()? {
                    entradas.push(entrada);
                }
                Ok(LeyendaDto { entradas })
            }
        }
        deserializer.deserialize_map(Visitante)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LugarDto {
    clave: String,
    nombre: String,
    caracteristicas: Vec<String>,
    relaciones: Vec<DeclaracionRelacionDto>,
    coordenada_central: CoordenadaDto,
    coordenadas_rotulo: Vec<CoordenadaDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CoordenadaDto {
    x: u64,
    y: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ActorDto {
    clave: String,
    lugar_inicial: String,
    caracteristicas: Vec<String>,
    relaciones: Vec<DeclaracionRelacionDto>,
    #[serde(deserialize_with = "deserializar_opcion_requerida")]
    retrato: Option<RetratoDto>,
    politica: PoliticaDto,
}

fn deserializar_opcion_requerida<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RetratoDto {
    paleta: PaletaRetratoDto,
    base: GraficoRetratoDto,
    #[serde(default)]
    rostro: Option<RostroRetratoDto>,
    #[serde(default)]
    parpadeo: Option<ParpadeoRetratoDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RostroRetratoDto {
    origen: CoordenadaDto,
    expresiones: ExpresionesRetratoDto,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ParpadeoRetratoDto {
    origen: CoordenadaDto,
    ojo_cerrado: GraficoRetratoDto,
    #[serde(default = "intervalo_parpadeo")]
    intervalo_ms: u64,
    #[serde(default = "duracion_parpadeo")]
    duracion_ms: u64,
}

fn intervalo_parpadeo() -> u64 {
    4500
}

fn duracion_parpadeo() -> u64 {
    250
}

struct ExpresionesRetratoDto(Vec<(String, GraficoRetratoDto)>);

impl<'de> Deserialize<'de> for ExpresionesRetratoDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitante;
        impl<'de> Visitor<'de> for Visitante {
            type Value = ExpresionesRetratoDto;
            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("un mapa de expresiones de rostro")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut acceso: A) -> Result<Self::Value, A::Error> {
                let mut entradas = Vec::new();
                while let Some(entrada) = acceso.next_entry()? {
                    entradas.push(entrada);
                }
                Ok(ExpresionesRetratoDto(entradas))
            }
        }
        deserializer.deserialize_map(Visitante)
    }
}

struct PaletaRetratoDto(Vec<(char, EstiloCeldaRetratoDto)>);

impl<'de> Deserialize<'de> for PaletaRetratoDto {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitante;
        impl<'de> Visitor<'de> for Visitante {
            type Value = PaletaRetratoDto;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("un mapa de claves de estilo de retrato")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut acceso: A) -> Result<Self::Value, A::Error> {
                let mut entradas = Vec::new();
                while let Some(entrada) = acceso.next_entry::<char, EstiloCeldaRetratoDto>()? {
                    entradas.push(entrada);
                }
                Ok(PaletaRetratoDto(entradas))
            }
        }
        deserializer.deserialize_map(Visitante)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EstiloCeldaRetratoDto {
    frente: String,
    fondo: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GraficoRetratoDto {
    arte: String,
    estilos: Option<String>,
}

#[derive(Deserialize)]
enum PoliticaDto {
    Ninguna,
    Lua(String),
}

#[derive(Deserialize)]
enum TipoRelacionDto {
    Conectado,
    Conoce,
}

#[derive(Deserialize)]
enum DestinoRelacionDto {
    Lugar(String),
    Actor(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeclaracionRelacionDto {
    tipo: TipoRelacionDto,
    destino: DestinoRelacionDto,
}

struct MapaValidado {
    plantilla: PlantillaMapa,
}
struct LugarValidado {
    inicial: LugarInicial,
    presentacion: PresentacionLugar,
    relaciones: Vec<RelacionDeclarada>,
}
struct ActorValidado {
    inicial: ActorInicial,
    presentacion: PresentacionActor,
    politica: Option<PoliticaLuaValidada>,
    relaciones: Vec<RelacionDeclarada>,
}

struct RelacionDeclarada {
    inicial: RelacionInicial,
    ruta: PathBuf,
    tipo_elemento: TipoElementoContenido,
    clave_elemento: String,
    posicion: usize,
}

pub fn cargar_contenido(raiz: &Path) -> Result<ContenidoValidado, ErrorContenido> {
    let raiz_canonica = fs::canonicalize(raiz).map_err(|error| {
        ErrorContenido::uno(diagnostico_raiz(
            raiz,
            ClaseDiagnosticoContenido::AccesoFilesystem,
            format!(
                "no se pudo resolver la raíz: {}",
                sanear(&error.to_string())
            ),
        ))
    })?;
    if !raiz_canonica.is_dir() {
        return Err(ErrorContenido::uno(diagnostico_raiz(
            raiz,
            ClaseDiagnosticoContenido::AccesoFilesystem,
            "la raíz de contenido no es un directorio".to_owned(),
        )));
    }

    let mundo: MundoDto = leer_ron(
        &raiz_canonica,
        Path::new("world.ron"),
        TipoElementoContenido::Mundo,
        None,
    )?;
    validar_lista_claves(&mundo.lugares, "lugares", &raiz_canonica)?;
    validar_lista_claves(&mundo.actores, "actores", &raiz_canonica)?;
    validar_lista_claves(
        &mundo.companeros_iniciales,
        "companeros_iniciales",
        &raiz_canonica,
    )?;
    validar_clave_externa(&mundo.mapa)
        .map_err(|detalle| error_mundo(ClaseDiagnosticoContenido::ClaveInvalida, detalle))?;

    let id_mapa = IdMapa::desde_posicion(0).map_err(|error| {
        error_mundo(
            ClaseDiagnosticoContenido::InvarianteDominio,
            error.to_string(),
        )
    })?;
    let mapa_ruta = PathBuf::from("maps").join(format!("{}.ron", mundo.mapa));
    let mapa_dto: MapaDto = leer_ron(
        &raiz_canonica,
        &mapa_ruta,
        TipoElementoContenido::Mapa,
        Some(&mundo.mapa),
    )?;
    let mapa = validar_mapa(mapa_dto, &mundo.mapa, id_mapa, &mapa_ruta)?;

    let mut lugares_por_clave = BTreeMap::new();
    let mut claves_por_lugar = BTreeMap::new();
    for (posicion, clave) in mundo.lugares.iter().enumerate() {
        let id = IdLugar::desde_posicion(posicion).map_err(|error| {
            error_mundo(
                ClaseDiagnosticoContenido::InvarianteDominio,
                error.to_string(),
            )
        })?;
        lugares_por_clave.insert(clave.clone(), id);
        claves_por_lugar.insert(id, clave.clone());
    }
    let mut actores_por_clave = BTreeMap::new();
    let mut claves_por_actor = BTreeMap::new();
    for (posicion, clave) in mundo.actores.iter().enumerate() {
        let id = IdActor::desde_posicion(posicion).map_err(|error| {
            error_mundo(
                ClaseDiagnosticoContenido::InvarianteDominio,
                error.to_string(),
            )
        })?;
        actores_por_clave.insert(clave.clone(), id);
        claves_por_actor.insert(id, clave.clone());
    }
    let claves = CatalogoClaves {
        actores_por_clave,
        claves_por_actor,
        lugares_por_clave,
        claves_por_lugar,
    };

    let mut lugares = Vec::with_capacity(mundo.lugares.len());
    for (posicion, clave) in mundo.lugares.iter().enumerate() {
        let ruta = PathBuf::from("places").join(format!("{clave}.ron"));
        let dto: LugarDto = leer_ron(
            &raiz_canonica,
            &ruta,
            TipoElementoContenido::Lugar,
            Some(clave),
        )?;
        lugares.push(validar_lugar(
            dto,
            clave,
            IdLugar::desde_posicion(posicion).expect("la posición ya se validó"),
            mapa.plantilla.ancho,
            mapa.plantilla.alto,
            &claves,
            &ruta,
        )?);
    }

    let jugador = claves
        .actores_por_clave
        .get(&mundo.actor_jugador)
        .copied()
        .ok_or_else(|| {
            error_mundo(
                ClaseDiagnosticoContenido::ReferenciaInvalida,
                format!(
                    "actor_jugador desconocido: {}",
                    sanear(&mundo.actor_jugador)
                ),
            )
        })?;
    if mundo.companeros_iniciales.is_empty() {
        return Err(error_mundo(
            ClaseDiagnosticoContenido::ValorInvalido,
            "companeros_iniciales no puede estar vacío".to_owned(),
        ));
    }
    let companeros = resolver_actores(&mundo.companeros_iniciales, &claves)?;
    if companeros.contains(&jugador) {
        return Err(error_mundo(
            ClaseDiagnosticoContenido::ReferenciaInvalida,
            "el jugador no puede integrar companeros_iniciales".to_owned(),
        ));
    }

    let mut actores = Vec::with_capacity(mundo.actores.len());
    for (posicion, clave) in mundo.actores.iter().enumerate() {
        let ruta = PathBuf::from("actors").join(format!("{clave}.ron"));
        let dto: ActorDto = leer_ron(
            &raiz_canonica,
            &ruta,
            TipoElementoContenido::Actor,
            Some(clave),
        )?;
        let id = IdActor::desde_posicion(posicion).expect("la posición ya se validó");
        let requiere_retrato = id == jugador || companeros.contains(&id);
        actores.push(validar_actor(
            dto,
            clave,
            id,
            requiere_retrato,
            &claves,
            &ruta,
        )?);
    }

    let fecha = validar_fecha(mundo.fecha_inicial)?;
    let variables_globales = validar_globales(mundo.variables_globales)?;
    let relaciones = normalizar_relaciones(
        lugares
            .iter()
            .flat_map(|lugar| lugar.relaciones.iter())
            .chain(actores.iter().flat_map(|actor| actor.relaciones.iter())),
    )?;
    let datos = DatosInicialesMundo {
        mapa: id_mapa,
        fecha,
        lugares: lugares.iter().map(|lugar| lugar.inicial.clone()).collect(),
        actores: actores.iter().map(|actor| actor.inicial.clone()).collect(),
        actor_jugador: jugador,
        companeros: companeros.clone(),
        variables_globales,
        relaciones,
    };
    Mundo::nuevo(datos.clone()).map_err(|error| {
        error_mundo(
            ClaseDiagnosticoContenido::InvarianteDominio,
            error.to_string(),
        )
    })?;

    let politicas: Vec<_> = actores
        .iter_mut()
        .filter_map(|actor| actor.politica.take())
        .collect();
    let contenido = ContenidoValidado {
        interno: Arc::new(ContenidoValidadoInterno {
            plantilla_mundo: datos,
            presentacion: CatalogoPresentacion {
                mapa: mapa.plantilla,
                lugares: lugares
                    .into_iter()
                    .map(|lugar| lugar.presentacion)
                    .collect(),
                actores: actores
                    .into_iter()
                    .map(|actor| actor.presentacion)
                    .collect(),
            },
            claves,
            politicas,
        }),
    };
    let (mundo, _) = contenido
        .instanciar_estado()
        .map_err(|e| error_mundo(ClaseDiagnosticoContenido::InvarianteDominio, e.to_string()))?;
    crate::contexto_viaje::construir(&mundo, contenido.presentacion())
        .map_err(|e| error_mundo(ClaseDiagnosticoContenido::ValorInvalido, e))?;
    Ok(contenido)
}

fn leer_ron<T: DeserializeOwned>(
    raiz: &Path,
    relativa: &Path,
    tipo: TipoElementoContenido,
    clave: Option<&str>,
) -> Result<T, ErrorContenido> {
    let canonica = fs::canonicalize(raiz.join(relativa)).map_err(|error| {
        error_archivo(
            relativa,
            tipo,
            clave,
            ClaseDiagnosticoContenido::AccesoFilesystem,
            None,
            format!(
                "no se pudo abrir el archivo: {}",
                sanear(&error.to_string())
            ),
        )
    })?;
    if !canonica.starts_with(raiz) {
        return Err(error_archivo(
            relativa,
            tipo,
            clave,
            ClaseDiagnosticoContenido::RutaFueraDeRaiz,
            None,
            "el archivo resuelve fuera de la raíz de contenido".to_owned(),
        ));
    }
    let texto = fs::read_to_string(&canonica).map_err(|error| {
        error_archivo(
            relativa,
            tipo,
            clave,
            ClaseDiagnosticoContenido::AccesoFilesystem,
            None,
            format!("no se pudo leer UTF-8: {}", sanear(&error.to_string())),
        )
    })?;
    let mut deserializador = ron::de::Deserializer::from_str(&texto)
        .map_err(|error| error_ron(relativa, tipo, clave, error))?;
    if !deserializador.extensions().is_empty() {
        return Err(error_archivo(
            relativa,
            tipo,
            clave,
            ClaseDiagnosticoContenido::FormatoRonInvalido,
            None,
            "no se permiten extensiones RON".to_owned(),
        ));
    }
    let valor = T::deserialize(&mut deserializador)
        .map_err(|error| error_ron_sin_posicion(relativa, tipo, clave, error))?;
    deserializador
        .end()
        .map_err(|error| error_ron_sin_posicion(relativa, tipo, clave, error))?;
    Ok(valor)
}

fn error_ron_sin_posicion(
    relativa: &Path,
    tipo: TipoElementoContenido,
    clave: Option<&str>,
    error: ron::error::Error,
) -> ErrorContenido {
    error_archivo(
        relativa,
        tipo,
        clave,
        ClaseDiagnosticoContenido::FormatoRonInvalido,
        None,
        sanear(&error.to_string()),
    )
}

fn error_ron(
    relativa: &Path,
    tipo: TipoElementoContenido,
    clave: Option<&str>,
    error: ron::error::SpannedError,
) -> ErrorContenido {
    error_archivo(
        relativa,
        tipo,
        clave,
        ClaseDiagnosticoContenido::FormatoRonInvalido,
        Some(PosicionDiagnosticoContenido::Fuente {
            linea: error.span.start.line,
            columna: error.span.start.col,
        }),
        sanear(&error.code.to_string()),
    )
}

fn validar_mapa(
    dto: MapaDto,
    esperada: &str,
    id: IdMapa,
    ruta: &Path,
) -> Result<MapaValidado, ErrorContenido> {
    validar_clave_y_archivo(&dto.clave, esperada, ruta, TipoElementoContenido::Mapa)?;
    if dto.leyenda.entradas.is_empty() {
        return Err(error_archivo(
            ruta,
            TipoElementoContenido::Mapa,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "la leyenda está vacía".to_owned(),
        ));
    }
    let mut simbolos = BTreeMap::new();
    let mut tipos = BTreeSet::new();
    for (simbolo, texto_tipo) in dto.leyenda.entradas {
        if !simbolo.is_ascii()
            || !simbolo.is_ascii_graphic()
            || matches!(simbolo, '\'' | '"' | '\\')
        {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Mapa,
                Some(esperada),
                ClaseDiagnosticoContenido::ValorInvalido,
                None,
                format!("símbolo de leyenda inválido: {}", simbolo.escape_default()),
            ));
        }
        if simbolos.contains_key(&simbolo) {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Mapa,
                Some(esperada),
                ClaseDiagnosticoContenido::Duplicado,
                None,
                format!("símbolo duplicado: {}", simbolo.escape_default()),
            ));
        }
        let tipo = ClaveTipoTile::nueva(texto_tipo).map_err(|error| {
            error_archivo(
                ruta,
                TipoElementoContenido::Mapa,
                Some(esperada),
                ClaseDiagnosticoContenido::ClaveInvalida,
                None,
                error.to_string(),
            )
        })?;
        if !tipos.insert(tipo.clone()) {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Mapa,
                Some(esperada),
                ClaseDiagnosticoContenido::Duplicado,
                None,
                format!("tipo visual duplicado: {}", tipo.como_str()),
            ));
        }
        simbolos.insert(simbolo, tipo);
    }
    let cuerpo = normalizar_bloque(&dto.matriz).map_err(|detalle| {
        error_archivo(
            ruta,
            TipoElementoContenido::Mapa,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            detalle,
        )
    })?;
    let filas: Vec<&str> = cuerpo.split('\n').collect();
    if filas.is_empty()
        || filas[0].is_empty()
        || filas
            .iter()
            .any(|fila| fila.is_empty() || fila.contains('\t'))
    {
        return Err(error_archivo(
            ruta,
            TipoElementoContenido::Mapa,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "la matriz contiene una fila vacía o tabulación".to_owned(),
        ));
    }
    let ancho = filas[0].len();
    if filas
        .iter()
        .any(|fila| fila.len() != ancho || !fila.is_ascii())
    {
        return Err(error_archivo(
            ruta,
            TipoElementoContenido::Mapa,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "la matriz no es ASCII rectangular".to_owned(),
        ));
    }
    let alto = filas.len();
    let mut celdas = Vec::with_capacity(ancho.checked_mul(alto).unwrap_or(0));
    for (y, fila) in filas.iter().enumerate() {
        for (x, simbolo) in fila.chars().enumerate() {
            let Some(tipo) = simbolos.get(&simbolo) else {
                return Err(error_archivo(
                    ruta,
                    TipoElementoContenido::Mapa,
                    Some(esperada),
                    ClaseDiagnosticoContenido::ReferenciaInvalida,
                    Some(PosicionDiagnosticoContenido::Matriz { x, y }),
                    format!("símbolo sin declarar: {}", simbolo.escape_default()),
                ));
            };
            celdas.push(tipo.clone());
        }
    }
    Ok(MapaValidado {
        plantilla: PlantillaMapa {
            id,
            ancho,
            alto,
            celdas_iniciales: celdas,
            tipos_declarados: tipos,
        },
    })
}

fn validar_lugar(
    dto: LugarDto,
    esperada: &str,
    id: IdLugar,
    ancho: usize,
    alto: usize,
    claves: &CatalogoClaves,
    ruta: &Path,
) -> Result<LugarValidado, ErrorContenido> {
    validar_clave_y_archivo(&dto.clave, esperada, ruta, TipoElementoContenido::Lugar)?;
    if dto.nombre.is_empty() {
        return Err(error_archivo(
            ruta,
            TipoElementoContenido::Lugar,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "el nombre está vacío".to_owned(),
        ));
    }
    for (indice, caracter) in dto.nombre.chars().enumerate() {
        if caracter.is_control() || UnicodeWidthChar::width(caracter) != Some(1) {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Lugar,
                Some(esperada),
                ClaseDiagnosticoContenido::ValorInvalido,
                None,
                format!(
                    "carácter de nombre no presentable en posición {indice}: {}",
                    caracter.escape_default()
                ),
            ));
        }
    }
    if dto.coordenadas_rotulo.len() != dto.nombre.chars().count() {
        return Err(error_archivo(
            ruta,
            TipoElementoContenido::Lugar,
            Some(esperada),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "el rótulo no tiene una coordenada por carácter".to_owned(),
        ));
    }
    let central = convertir_coordenada(
        dto.coordenada_central,
        ancho,
        alto,
        ruta,
        TipoElementoContenido::Lugar,
        esperada,
    )?;
    let mut vistas = BTreeSet::new();
    vistas.insert(central);
    let mut rotulo = Vec::with_capacity(dto.coordenadas_rotulo.len());
    for coordenada in dto.coordenadas_rotulo {
        let coordenada = convertir_coordenada(
            coordenada,
            ancho,
            alto,
            ruta,
            TipoElementoContenido::Lugar,
            esperada,
        )?;
        if !vistas.insert(coordenada) {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Lugar,
                Some(esperada),
                ClaseDiagnosticoContenido::Duplicado,
                None,
                "coordenada repetida dentro del lugar".to_owned(),
            ));
        }
        rotulo.push(coordenada);
    }
    let caracteristicas = validar_caracteristicas(
        dto.caracteristicas,
        ruta,
        TipoElementoContenido::Lugar,
        esperada,
    )?;
    let relaciones = validar_relaciones(
        dto.relaciones,
        ExtremoRelacion::Lugar(id),
        claves,
        ruta,
        TipoElementoContenido::Lugar,
        esperada,
    )?;
    Ok(LugarValidado {
        inicial: LugarInicial {
            id,
            caracteristicas,
        },
        presentacion: PresentacionLugar {
            id,
            nombre: dto.nombre,
            coordenada_central: central,
            coordenadas_rotulo: rotulo,
        },
        relaciones,
    })
}

fn validar_actor(
    dto: ActorDto,
    esperada: &str,
    id: IdActor,
    requiere_retrato: bool,
    claves: &CatalogoClaves,
    ruta: &Path,
) -> Result<ActorValidado, ErrorContenido> {
    validar_clave_y_archivo(&dto.clave, esperada, ruta, TipoElementoContenido::Actor)?;
    let lugar = claves
        .lugares_por_clave
        .get(&dto.lugar_inicial)
        .copied()
        .ok_or_else(|| {
            error_archivo(
                ruta,
                TipoElementoContenido::Actor,
                Some(esperada),
                ClaseDiagnosticoContenido::ReferenciaInvalida,
                None,
                format!("lugar inicial desconocido: {}", sanear(&dto.lugar_inicial)),
            )
        })?;
    let caracteristicas = validar_caracteristicas(
        dto.caracteristicas,
        ruta,
        TipoElementoContenido::Actor,
        esperada,
    )?;
    let relaciones = validar_relaciones(
        dto.relaciones,
        ExtremoRelacion::Actor(id),
        claves,
        ruta,
        TipoElementoContenido::Actor,
        esperada,
    )?;
    let retrato = match dto.retrato {
        Some(retrato) => Some(validar_retrato(retrato, ruta, esperada)?),
        None if requiere_retrato => {
            return Err(error_archivo(
                ruta,
                TipoElementoContenido::Actor,
                Some(esperada),
                ClaseDiagnosticoContenido::ValorInvalido,
                None,
                "el jugador y sus compañeros requieren retrato".to_owned(),
            ));
        }
        None => None,
    };
    let politica = match dto.politica {
        PoliticaDto::Ninguna => None,
        PoliticaDto::Lua(codigo) => Some(
            validar_politica(id, esperada.to_owned(), ruta.to_path_buf(), codigo).map_err(
                |error| {
                    error_archivo(
                        ruta,
                        TipoElementoContenido::Actor,
                        Some(esperada),
                        ClaseDiagnosticoContenido::PoliticaLuaInvalida,
                        None,
                        error.to_string(),
                    )
                },
            )?,
        ),
    };
    Ok(ActorValidado {
        inicial: ActorInicial {
            id,
            lugar,
            caracteristicas,
        },
        presentacion: PresentacionActor { id, retrato },
        politica,
        relaciones,
    })
}

fn validar_relaciones(
    declaraciones: Vec<DeclaracionRelacionDto>,
    origen: ExtremoRelacion,
    claves: &CatalogoClaves,
    ruta: &Path,
    tipo_elemento: TipoElementoContenido,
    clave_elemento: &str,
) -> Result<Vec<RelacionDeclarada>, ErrorContenido> {
    let mut resultado = Vec::with_capacity(declaraciones.len());
    for (posicion, declaracion) in declaraciones.into_iter().enumerate() {
        let tipo = match declaracion.tipo {
            TipoRelacionDto::Conectado => TipoRelacion::Conectado,
            TipoRelacionDto::Conoce => TipoRelacion::Conoce,
        };
        let destino = match declaracion.destino {
            DestinoRelacionDto::Lugar(clave) => {
                validar_clave_externa(&clave).map_err(|detalle| {
                    error_archivo(
                        ruta,
                        tipo_elemento,
                        Some(clave_elemento),
                        ClaseDiagnosticoContenido::ClaveInvalida,
                        None,
                        format!("relaciones[{posicion}].destino: {detalle}"),
                    )
                })?;
                ExtremoRelacion::Lugar(claves.lugares_por_clave.get(&clave).copied().ok_or_else(
                    || {
                        error_archivo(
                            ruta,
                            tipo_elemento,
                            Some(clave_elemento),
                            ClaseDiagnosticoContenido::ReferenciaInvalida,
                            None,
                            format!(
                                "relaciones[{posicion}].destino: Lugar desconocido: {}",
                                sanear(&clave)
                            ),
                        )
                    },
                )?)
            }
            DestinoRelacionDto::Actor(clave) => {
                validar_clave_externa(&clave).map_err(|detalle| {
                    error_archivo(
                        ruta,
                        tipo_elemento,
                        Some(clave_elemento),
                        ClaseDiagnosticoContenido::ClaveInvalida,
                        None,
                        format!("relaciones[{posicion}].destino: {detalle}"),
                    )
                })?;
                ExtremoRelacion::Actor(claves.actores_por_clave.get(&clave).copied().ok_or_else(
                    || {
                        error_archivo(
                            ruta,
                            tipo_elemento,
                            Some(clave_elemento),
                            ClaseDiagnosticoContenido::ReferenciaInvalida,
                            None,
                            format!(
                                "relaciones[{posicion}].destino: Actor desconocido: {}",
                                sanear(&clave)
                            ),
                        )
                    },
                )?)
            }
        };
        let extremos_validos = matches!(
            (tipo, origen, destino),
            (
                TipoRelacion::Conectado,
                ExtremoRelacion::Lugar(_),
                ExtremoRelacion::Lugar(_)
            ) | (
                TipoRelacion::Conoce,
                ExtremoRelacion::Actor(_),
                ExtremoRelacion::Lugar(_)
            )
        );
        if !extremos_validos {
            let requisito = match tipo {
                TipoRelacion::Conectado => "Conectado exige dos Lugares",
                TipoRelacion::Conoce => "Conoce exige un Actor de origen y un Lugar de destino",
            };
            return Err(error_archivo(
                ruta,
                tipo_elemento,
                Some(clave_elemento),
                ClaseDiagnosticoContenido::ValorInvalido,
                None,
                format!("relaciones[{posicion}]: {requisito}"),
            ));
        }
        if origen == destino {
            return Err(error_archivo(
                ruta,
                tipo_elemento,
                Some(clave_elemento),
                ClaseDiagnosticoContenido::ValorInvalido,
                None,
                format!("relaciones[{posicion}]: una Relación no puede ser reflexiva"),
            ));
        }
        resultado.push(RelacionDeclarada {
            inicial: RelacionInicial {
                tipo,
                origen,
                destino,
            },
            ruta: ruta.to_path_buf(),
            tipo_elemento,
            clave_elemento: clave_elemento.to_owned(),
            posicion,
        });
    }
    Ok(resultado)
}

fn normalizar_relaciones<'a>(
    declaraciones: impl Iterator<Item = &'a RelacionDeclarada>,
) -> Result<Vec<RelacionInicial>, ErrorContenido> {
    let mut vistas: BTreeMap<(TipoRelacion, ExtremoRelacion, ExtremoRelacion), (PathBuf, usize)> =
        BTreeMap::new();
    let mut resultado = Vec::new();
    for declaracion in declaraciones {
        let par = match (
            declaracion.inicial.tipo,
            declaracion.inicial.origen,
            declaracion.inicial.destino,
        ) {
            (TipoRelacion::Conectado, origen, destino) if origen < destino => {
                (declaracion.inicial.tipo, origen, destino)
            }
            (TipoRelacion::Conectado, origen, destino) => {
                (declaracion.inicial.tipo, destino, origen)
            }
            (TipoRelacion::Conoce, origen, destino) => (declaracion.inicial.tipo, origen, destino),
        };
        if let Some((ruta_primera, posicion_primera)) = vistas.get(&par) {
            return Err(error_archivo(
                &declaracion.ruta,
                declaracion.tipo_elemento,
                Some(&declaracion.clave_elemento),
                ClaseDiagnosticoContenido::Duplicado,
                None,
                format!(
                    "relaciones[{}] duplica la declaración {} de {}",
                    declaracion.posicion,
                    posicion_primera,
                    ruta_primera.display()
                ),
            ));
        }
        vistas.insert(par, (declaracion.ruta.clone(), declaracion.posicion));
        resultado.push(declaracion.inicial.clone());
    }
    Ok(resultado)
}

fn validar_retrato(
    dto: RetratoDto,
    ruta: &Path,
    clave: &str,
) -> Result<RetratoActor, ErrorContenido> {
    let error = |detalle| {
        error_archivo(
            ruta,
            TipoElementoContenido::Actor,
            Some(clave),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            detalle,
        )
    };
    let mut paleta = BTreeMap::new();
    for (codigo, estilo) in dto.paleta.0 {
        if !codigo.is_ascii_graphic() || matches!(codigo, '\'' | '"' | '\\') {
            return Err(error(format!(
                "clave de paleta de retrato no admitida: {}",
                codigo.escape_default()
            )));
        }
        let frente = color_retrato(&estilo.frente).ok_or_else(|| {
            error(format!(
                "color frontal de retrato desconocido: {}",
                sanear(&estilo.frente)
            ))
        })?;
        let fondo = estilo
            .fondo
            .as_deref()
            .map(|texto| {
                color_retrato(texto).ok_or_else(|| {
                    error(format!(
                        "color de fondo de retrato desconocido: {}",
                        sanear(texto)
                    ))
                })
            })
            .transpose()?;
        if paleta
            .insert(codigo, EstiloCeldaRetrato { frente, fondo })
            .is_some()
        {
            return Err(error(format!(
                "clave de paleta de retrato duplicada: {codigo}"
            )));
        }
    }

    let base = validar_grafico_retrato(dto.base, &paleta, &error, "base")?;
    let rostro = dto
        .rostro
        .map(|rostro| {
            let origen = validar_posicion_retrato(rostro.origen, &base, &error)?;
            let mut expresiones = BTreeMap::new();
            for (nombre, grafico) in rostro.expresiones.0 {
                if nombre.len() > MAX_BYTES_CLAVE_EXTERNA || !es_lower_camel(&nombre) {
                    return Err(error(format!(
                        "clave de expresión inválida: {}",
                        sanear(&nombre)
                    )));
                }
                let grafico = validar_grafico_retrato(grafico, &paleta, &error, &nombre)?;
                validar_limites_capa(origen, &grafico, &base, &error)?;
                if expresiones.insert(nombre.clone(), grafico).is_some() {
                    return Err(error(format!("expresión duplicada: {nombre}")));
                }
            }
            Ok(RostroRetrato {
                origen,
                expresiones,
            })
        })
        .transpose()?;
    let parpadeo = dto.parpadeo.map(|parpadeo| {
        if parpadeo.duracion_ms == 0 || parpadeo.intervalo_ms <= parpadeo.duracion_ms
            || parpadeo.intervalo_ms % 250 != 0 || parpadeo.duracion_ms % 250 != 0 {
            return Err(error("parpadeo: intervalo y duración deben ser múltiplos positivos de 250 ms y la duración debe ser menor que el intervalo".to_owned()));
        }
        let origen = validar_posicion_retrato(parpadeo.origen, &base, &error)?;
        let ojo_cerrado = validar_grafico_retrato(parpadeo.ojo_cerrado, &paleta, &error, "ojo_cerrado")?;
        validar_limites_capa(origen, &ojo_cerrado, &base, &error)?;
        Ok(ParpadeoRetrato { origen, ojo_cerrado, intervalo_ms: parpadeo.intervalo_ms, duracion_ms: parpadeo.duracion_ms })
    }).transpose()?;
    Ok(RetratoActor {
        base,
        rostro,
        parpadeo,
    })
}

fn validar_posicion_retrato(
    dto: CoordenadaDto,
    base: &GraficoRetrato,
    error: &impl Fn(String) -> ErrorContenido,
) -> Result<PosicionRetrato, ErrorContenido> {
    let x = usize::try_from(dto.x)
        .map_err(|_| error("coordenada de retrato fuera de rango".to_owned()))?;
    let y = usize::try_from(dto.y)
        .map_err(|_| error("coordenada de retrato fuera de rango".to_owned()))?;
    if x >= base.ancho || y >= base.alto {
        return Err(error("origen de capa fuera del retrato base".to_owned()));
    }
    Ok(PosicionRetrato { x, y })
}

fn validar_limites_capa(
    origen: PosicionRetrato,
    grafico: &GraficoRetrato,
    base: &GraficoRetrato,
    error: &impl Fn(String) -> ErrorContenido,
) -> Result<(), ErrorContenido> {
    if !origen
        .x
        .checked_add(grafico.ancho)
        .is_some_and(|x| x <= base.ancho)
        || !origen
            .y
            .checked_add(grafico.alto)
            .is_some_and(|y| y <= base.alto)
    {
        return Err(error(
            "la capa excede los límites del retrato base".to_owned(),
        ));
    }
    Ok(())
}

fn validar_grafico_retrato(
    grafico: GraficoRetratoDto,
    paleta: &BTreeMap<char, EstiloCeldaRetrato>,
    error: &impl Fn(String) -> ErrorContenido,
    contexto: &str,
) -> Result<GraficoRetrato, ErrorContenido> {
    let fallo = |detalle| error(format!("{contexto}: {detalle}"));
    let cuerpo = normalizar_bloque(&grafico.arte).map_err(fallo)?;
    let filas: Vec<Vec<char>> = cuerpo.lines().map(|fila| fila.chars().collect()).collect();
    if filas.is_empty() || filas[0].is_empty() || filas.iter().any(Vec::is_empty) {
        return Err(fallo("gráfico vacío".to_owned()));
    }
    let ancho = filas[0].len();
    let alto = filas.len();
    if filas.iter().any(|fila| fila.len() != ancho) {
        return Err(fallo("arte no rectangular".to_owned()));
    }
    for (y, fila) in filas.iter().enumerate() {
        for (x, caracter) in fila.iter().copied().enumerate() {
            if caracter.is_control() || caracter.width() != Some(1) {
                return Err(fallo(format!(
                    "glifo de retrato no admitido en ({x}, {y}): {}",
                    caracter.escape_default()
                )));
            }
        }
    }
    let estilos = match grafico.estilos {
        None => vec![None; ancho * alto],
        Some(texto) => {
            let cuerpo = normalizar_bloque(&texto).map_err(fallo)?;
            let filas_estilo: Vec<Vec<char>> =
                cuerpo.lines().map(|fila| fila.chars().collect()).collect();
            if filas_estilo.len() != alto || filas_estilo.iter().any(|fila| fila.len() != ancho) {
                return Err(fallo(
                    "matriz de estilos no coincide con el arte".to_owned(),
                ));
            }
            let mut estilos = Vec::with_capacity(ancho * alto);
            for (y, fila) in filas_estilo.into_iter().enumerate() {
                for (x, codigo) in fila.into_iter().enumerate() {
                    let estilo = paleta.get(&codigo).copied().ok_or_else(|| {
                        fallo(format!(
                            "estilo de retrato no declarado en ({x}, {y}): {}",
                            codigo.escape_default()
                        ))
                    })?;
                    estilos.push(Some(estilo));
                }
            }
            estilos
        }
    };
    let celdas = filas
        .into_iter()
        .flatten()
        .zip(estilos)
        .map(|(glifo, estilo)| CeldaRetrato { glifo, estilo })
        .collect();
    Ok(GraficoRetrato {
        ancho,
        alto,
        celdas,
    })
}

fn color_retrato(texto: &str) -> Option<ColorRetrato> {
    match texto {
        "negro" => Some(ColorRetrato::Negro),
        "rojo" => Some(ColorRetrato::Rojo),
        "verde" => Some(ColorRetrato::Verde),
        "amarillo" => Some(ColorRetrato::Amarillo),
        "azul" => Some(ColorRetrato::Azul),
        "magenta" => Some(ColorRetrato::Magenta),
        "cian" => Some(ColorRetrato::Cian),
        "gris" => Some(ColorRetrato::Gris),
        "grisOscuro" => Some(ColorRetrato::GrisOscuro),
        "grisMuyOscuro" => Some(ColorRetrato::GrisMuyOscuro),
        "rojoClaro" => Some(ColorRetrato::RojoClaro),
        "verdeClaro" => Some(ColorRetrato::VerdeClaro),
        "amarilloClaro" => Some(ColorRetrato::AmarilloClaro),
        "azulClaro" => Some(ColorRetrato::AzulClaro),
        "magentaClaro" => Some(ColorRetrato::MagentaClaro),
        "cianClaro" => Some(ColorRetrato::CianClaro),
        "blanco" => Some(ColorRetrato::Blanco),
        _ => None,
    }
}

fn validar_fecha(dto: FechaDto) -> Result<Fecha, ErrorContenido> {
    let anio = u16::try_from(dto.anio)
        .ok()
        .filter(|anio| (1..=9999).contains(anio));
    let mes = u8::try_from(dto.mes)
        .ok()
        .and_then(|numero| Mes::desde_numero(numero).ok());
    let dia = u8::try_from(dto.dia).ok();
    match (anio, mes, dia) {
        (Some(anio), Some(mes), Some(dia)) => Fecha::nueva(anio, mes, dia).map_err(|error| {
            error_mundo(ClaseDiagnosticoContenido::ValorInvalido, error.to_string())
        }),
        _ => Err(error_mundo(
            ClaseDiagnosticoContenido::ValorInvalido,
            "fecha inicial fuera de rango".to_owned(),
        )),
    }
}

fn validar_globales(
    dto: VariablesGlobalesDto,
) -> Result<Vec<VariableGlobalInicial>, ErrorContenido> {
    let mut vistas = BTreeSet::new();
    let mut resultado = Vec::new();
    for entrada in dto.booleanas {
        insertar_global(
            &mut resultado,
            &mut vistas,
            entrada.clave,
            ValorVariableGlobal::Booleana(entrada.valor),
        )?;
    }
    for entrada in dto.enteras {
        let valor = i64::try_from(entrada.valor).map_err(|_| {
            error_mundo(
                ClaseDiagnosticoContenido::ValorInvalido,
                "entero global fuera de i64".to_owned(),
            )
        })?;
        insertar_global(
            &mut resultado,
            &mut vistas,
            entrada.clave,
            ValorVariableGlobal::Entera(valor),
        )?;
    }
    for entrada in dto.textos {
        insertar_global(
            &mut resultado,
            &mut vistas,
            entrada.clave,
            ValorVariableGlobal::Texto(entrada.valor),
        )?;
    }
    Ok(resultado)
}

fn insertar_global(
    resultado: &mut Vec<VariableGlobalInicial>,
    vistas: &mut BTreeSet<String>,
    texto: String,
    valor: ValorVariableGlobal,
) -> Result<(), ErrorContenido> {
    if !vistas.insert(texto.clone()) {
        return Err(error_mundo(
            ClaseDiagnosticoContenido::Duplicado,
            format!("variable global duplicada: {}", sanear(&texto)),
        ));
    }
    let clave = ClaveVariableGlobal::nueva(texto).map_err(|error| {
        error_mundo(ClaseDiagnosticoContenido::ClaveInvalida, error.to_string())
    })?;
    resultado.push(VariableGlobalInicial { clave, valor });
    Ok(())
}

fn validar_caracteristicas(
    valores: Vec<String>,
    ruta: &Path,
    tipo: TipoElementoContenido,
    clave: &str,
) -> Result<Vec<Caracteristica>, ErrorContenido> {
    let mut vistas = BTreeSet::new();
    let mut resultado = Vec::new();
    for valor in valores {
        let caracteristica = Caracteristica::nueva(valor).map_err(|error| {
            error_archivo(
                ruta,
                tipo,
                Some(clave),
                ClaseDiagnosticoContenido::ClaveInvalida,
                None,
                error.to_string(),
            )
        })?;
        if !vistas.insert(caracteristica.clone()) {
            return Err(error_archivo(
                ruta,
                tipo,
                Some(clave),
                ClaseDiagnosticoContenido::Duplicado,
                None,
                format!("característica duplicada: {}", caracteristica.como_str()),
            ));
        }
        resultado.push(caracteristica);
    }
    Ok(resultado)
}

fn validar_lista_claves(
    valores: &[String],
    campo: &str,
    _raiz: &Path,
) -> Result<(), ErrorContenido> {
    if valores.is_empty() {
        return Err(error_mundo(
            ClaseDiagnosticoContenido::ValorInvalido,
            format!("{campo} no puede estar vacío"),
        ));
    }
    let mut vistas = BTreeSet::new();
    for valor in valores {
        validar_clave_externa(valor)
            .map_err(|detalle| error_mundo(ClaseDiagnosticoContenido::ClaveInvalida, detalle))?;
        if !vistas.insert(valor) {
            return Err(error_mundo(
                ClaseDiagnosticoContenido::Duplicado,
                format!("clave repetida en {campo}: {}", sanear(valor)),
            ));
        }
    }
    Ok(())
}

fn resolver_actores(
    valores: &[String],
    claves: &CatalogoClaves,
) -> Result<Vec<IdActor>, ErrorContenido> {
    valores
        .iter()
        .map(|clave| {
            claves.actores_por_clave.get(clave).copied().ok_or_else(|| {
                error_mundo(
                    ClaseDiagnosticoContenido::ReferenciaInvalida,
                    format!("actor desconocido: {}", sanear(clave)),
                )
            })
        })
        .collect()
}

fn validar_clave_y_archivo(
    valor: &str,
    esperado: &str,
    ruta: &Path,
    tipo: TipoElementoContenido,
) -> Result<(), ErrorContenido> {
    validar_clave_externa(valor).map_err(|detalle| {
        error_archivo(
            ruta,
            tipo,
            Some(esperado),
            ClaseDiagnosticoContenido::ClaveInvalida,
            None,
            detalle,
        )
    })?;
    if valor != esperado {
        return Err(error_archivo(
            ruta,
            tipo,
            Some(esperado),
            ClaseDiagnosticoContenido::ReferenciaInvalida,
            None,
            format!(
                "la clave interna {} no coincide con el archivo {esperado}",
                sanear(valor)
            ),
        ));
    }
    Ok(())
}

fn validar_clave_externa(valor: &str) -> Result<(), String> {
    if valor.len() > MAX_BYTES_CLAVE_EXTERNA || valor.is_empty() {
        return Err(format!(
            "clave externa de longitud inválida: {}",
            sanear(valor)
        ));
    }
    let bytes = valor.as_bytes();
    if !bytes[0].is_ascii_lowercase()
        || bytes
            .iter()
            .any(|byte| !byte.is_ascii_lowercase() && !byte.is_ascii_digit() && *byte != b'_')
        || valor.split('_').any(str::is_empty)
    {
        return Err(format!("clave externa inválida: {}", sanear(valor)));
    }
    Ok(())
}

pub(crate) fn clave_externa_valida(valor: &str) -> bool {
    validar_clave_externa(valor).is_ok()
}

fn convertir_coordenada(
    dto: CoordenadaDto,
    ancho: usize,
    alto: usize,
    ruta: &Path,
    tipo: TipoElementoContenido,
    clave: &str,
) -> Result<CoordenadaMapa, ErrorContenido> {
    let x = usize::try_from(dto.x).ok();
    let y = usize::try_from(dto.y).ok();
    match (x, y) {
        (Some(x), Some(y)) if x < ancho && y < alto => Ok(CoordenadaMapa { x, y }),
        _ => Err(error_archivo(
            ruta,
            tipo,
            Some(clave),
            ClaseDiagnosticoContenido::ValorInvalido,
            None,
            "coordenada fuera del mapa".to_owned(),
        )),
    }
}

fn normalizar_bloque(texto: &str) -> Result<String, String> {
    let mut normalizado = String::with_capacity(texto.len());
    let mut caracteres = texto.chars().peekable();
    while let Some(caracter) = caracteres.next() {
        if caracter == '\r' {
            if caracteres.next_if_eq(&'\n').is_some() {
                normalizado.push('\n');
                continue;
            }
            return Err("el bloque contiene un retorno de carro aislado".to_owned());
        }
        normalizado.push(caracter);
    }
    if normalizado.starts_with('\n') {
        normalizado.remove(0);
    }
    if normalizado.ends_with('\n') {
        normalizado.pop();
    }
    Ok(normalizado)
}

fn diagnostico_raiz(
    raiz: &Path,
    clase: ClaseDiagnosticoContenido,
    detalle: String,
) -> DiagnosticoContenido {
    DiagnosticoContenido {
        clase,
        origen: OrigenDiagnosticoContenido::Raiz {
            ruta: raiz.to_path_buf(),
        },
        elemento: None,
        posicion: None,
        detalle: sanear(&detalle),
    }
}

fn error_mundo(clase: ClaseDiagnosticoContenido, detalle: String) -> ErrorContenido {
    ErrorContenido::uno(DiagnosticoContenido {
        clase,
        origen: OrigenDiagnosticoContenido::Archivo {
            ruta_relativa: PathBuf::from("world.ron"),
        },
        elemento: Some(ElementoDiagnosticoContenido {
            tipo: TipoElementoContenido::Mundo,
            clave: None,
        }),
        posicion: None,
        detalle: sanear(&detalle),
    })
}

fn error_archivo(
    ruta: &Path,
    tipo: TipoElementoContenido,
    clave: Option<&str>,
    clase: ClaseDiagnosticoContenido,
    posicion: Option<PosicionDiagnosticoContenido>,
    detalle: String,
) -> ErrorContenido {
    ErrorContenido::uno(DiagnosticoContenido {
        clase,
        origen: OrigenDiagnosticoContenido::Archivo {
            ruta_relativa: ruta.to_path_buf(),
        },
        elemento: Some(ElementoDiagnosticoContenido {
            tipo,
            clave: clave.map(str::to_owned),
        }),
        posicion,
        detalle: sanear(&detalle),
    })
}

pub(crate) fn sanear(texto: &str) -> String {
    texto
        .chars()
        .flat_map(|caracter| {
            if caracter == '\u{1b}' || caracter.is_control() {
                caracter.escape_default().collect::<Vec<_>>()
            } else {
                vec![caracter]
            }
        })
        .collect()
}

fn es_lower_camel(valor: &str) -> bool {
    let mut bytes = valor.bytes();
    matches!(bytes.next(), Some(inicial) if inicial.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_alphabetic())
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::lua::{ResultadoPoliticaLua, evaluar_politica};
    use desert_tiger_core::Duracion;
    use desert_tiger_core::acciones::SolicitudAccion;

    fn raiz_base() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base")
    }

    #[test]
    fn carga_e_instancia_el_fixture_oficial() {
        assert_eq!(
            color_retrato("grisMuyOscuro"),
            Some(ColorRetrato::GrisMuyOscuro)
        );
        assert_eq!(color_retrato("GrisMuyOscuro"), None);

        let contenido = cargar_contenido(&raiz_base()).expect("el fixture debe ser válido");
        let (mundo, mapa) = contenido
            .instanciar_estado()
            .expect("el contenido validado debe instanciarse");

        assert_eq!(mapa.ancho(), 192);
        assert_eq!(mapa.alto(), 64);
        assert_eq!(mapa.celdas().len(), 192 * 64);
        assert_eq!(mundo.fecha().anio(), 2025);
        assert_eq!(mundo.fecha().mes(), Mes::Enero);
        assert_eq!(mundo.fecha().dia(), 31);
        assert_eq!(
            mundo.grupo().integrantes(),
            contenido.interno.plantilla_mundo.companeros.as_slice()
        );
        assert_eq!(mundo.relaciones().len(), 42);
        assert_eq!(
            mundo.lugares_conectados(IdLugar::desde_posicion(1).expect("ID válido")),
            Some(
                [
                    IdLugar::desde_posicion(6).expect("ID válido"),
                    IdLugar::desde_posicion(9).expect("ID válido"),
                    IdLugar::desde_posicion(13).expect("ID válido"),
                    IdLugar::desde_posicion(20).expect("ID válido"),
                    IdLugar::desde_posicion(21).expect("ID válido"),
                ]
                .as_slice()
            )
        );
        assert_eq!(contenido.presentacion().lugares().len(), 26);
        for (origen, intermedio, destino) in [
            (0, 20, 1),
            (1, 21, 2),
            (2, 22, 10),
            (10, 23, 11),
            (11, 24, 14),
            (14, 25, 17),
        ] {
            let origen = IdLugar::desde_posicion(origen).expect("ID válido");
            let intermedio = IdLugar::desde_posicion(intermedio).expect("ID válido");
            let destino = IdLugar::desde_posicion(destino).expect("ID válido");
            assert!(
                mundo
                    .lugares_conectados(origen)
                    .expect("Lugar válido")
                    .contains(&intermedio)
            );
            assert!(
                mundo
                    .lugares_conectados(intermedio)
                    .expect("Lugar válido")
                    .contains(&destino)
            );
            assert!(
                !mundo
                    .lugares_conectados(origen)
                    .expect("Lugar válido")
                    .contains(&destino)
            );
        }
        assert_eq!(contenido.presentacion().actores().len(), 6);
        let bandido = IdActor::desde_posicion(1).expect("ID válido");
        let cazador = IdActor::desde_posicion(2).expect("ID válido");
        let id_lugar = |posicion| IdLugar::desde_posicion(posicion).expect("ID válido");
        assert_eq!(
            mundo.lugares_conocidos(bandido),
            Some([id_lugar(0), id_lugar(1), id_lugar(20), id_lugar(21)].as_slice())
        );
        assert!(
            mundo
                .actor_conoce_lugar(cazador, id_lugar(10))
                .expect("consulta válida")
        );
        assert_eq!(contenido.politicas().len(), 1);
        assert_eq!(
            contenido.presentacion().lugares()[0].nombre(),
            "Hamada Roja"
        );
        assert_eq!(
            contenido.presentacion().lugares()[2].nombre(),
            "Valle del Drâa"
        );
        let retrato = contenido.presentacion().actores()[0]
            .retrato()
            .expect("el príncipe tiene retrato");
        assert_eq!((retrato.base().ancho(), retrato.base().alto()), (24, 14));
        let ojo = 7 * 24 + 14;
        assert_eq!(
            retrato.parpadeo().unwrap().ojo_cerrado().celdas()[1].glifo,
            '▀'
        );
        assert_eq!(retrato.base().celdas()[ojo].glifo, 'o');
        assert_eq!(
            retrato.base().celdas()[ojo].estilo,
            Some(EstiloCeldaRetrato {
                frente: ColorRetrato::Cian,
                fondo: Some(ColorRetrato::Negro),
            })
        );
    }

    fn retrato_de_prueba(
        arte: &str,
        estilos: Option<&str>,
        paleta: Vec<(char, EstiloCeldaRetratoDto)>,
    ) -> RetratoDto {
        RetratoDto {
            paleta: PaletaRetratoDto(paleta),
            base: GraficoRetratoDto {
                arte: arte.to_owned(),
                estilos: estilos.map(str::to_owned),
            },
            rostro: None,
            parpadeo: None,
        }
    }

    fn estilo_dto() -> EstiloCeldaRetratoDto {
        EstiloCeldaRetratoDto {
            frente: "blanco".to_owned(),
            fondo: None,
        }
    }

    fn retrato_capas_de_prueba() -> RetratoDto {
        ron::from_str(
            r#"(
            paleta: {
                'b': (frente: "blanco", fondo: Some("negro")),
                'e': (frente: "rojo", fondo: Some("verde")),
                'p': (frente: "cian", fondo: None),
            },
            base: (arte: "abcd\nefgh\nijkl", estilos: Some("bbbb\nbbbb\nbbbb")),
            rostro: Some((origen: (x: 0, y: 0), expresiones: {
                "enojado": (arte: "X ZW\n Q  ", estilos: Some("eeee\neeee")),
            })),
            parpadeo: Some((origen: (x: 1, y: 0),
                intervalo_ms: 1000, duracion_ms: 250,
                ojo_cerrado: (arte: " C ", estilos: Some("ppp")),
            )),
        )"#,
        )
        .expect("retrato de prueba válido")
    }

    #[test]
    fn capas_tienen_origen_y_orden_sin_imponer_un_ojo_de_dos_celdas() {
        let retrato = validar_retrato(
            retrato_capas_de_prueba(),
            Path::new("actors/prueba.ron"),
            "prueba",
        )
        .unwrap();
        let capas = retrato.capas(Some("enojado"), true);
        assert_eq!(capas[0].unwrap().1.celdas()[0].glifo, 'a');
        assert_eq!(capas[1].unwrap().1.celdas()[0].glifo, 'X');
        assert_eq!(capas[1].unwrap().1.celdas()[1].glifo, ' ');
        let (origen, ojo) = capas[2].unwrap();
        assert_eq!(origen, PosicionRetrato { x: 1, y: 0 });
        assert_eq!((ojo.ancho(), ojo.alto()), (3, 1));
        assert!(retrato.capas(Some("desconocida"), false)[1].is_none());
        assert!(retrato.capas(None, false)[2].is_none());
    }

    #[test]
    fn rechaza_capas_fuera_del_retrato_y_expresiones_duplicadas() {
        let mut dto = retrato_capas_de_prueba();
        dto.parpadeo.as_mut().unwrap().origen.x = 2;
        let error = validar_retrato(dto, Path::new("actors/prueba.ron"), "prueba").unwrap_err();
        assert!(error.diagnosticos()[0].detalle().contains("límites"));
        let mut dto = retrato_capas_de_prueba();
        dto.rostro.as_mut().unwrap().expresiones.0.push((
            "enojado".into(),
            GraficoRetratoDto {
                arte: "x".into(),
                estilos: None,
            },
        ));
        let error = validar_retrato(dto, Path::new("actors/prueba.ron"), "prueba").unwrap_err();
        assert!(error.diagnosticos()[0].detalle().contains("duplicada"));
        let mut dto = retrato_capas_de_prueba();
        dto.rostro.as_mut().unwrap().origen.x = u64::MAX;
        assert!(validar_retrato(dto, Path::new("actors/prueba.ron"), "prueba").is_err());
    }

    #[test]
    fn rechaza_tiempos_invalidos_y_admite_retratos_sin_capas() {
        for (intervalo, duracion) in [
            (0, 250),
            (1000, 0),
            (1000, 1000),
            (1000, 1250),
            (1100, 250),
            (1000, 100),
        ] {
            let mut dto = retrato_capas_de_prueba();
            let parpadeo = dto.parpadeo.as_mut().unwrap();
            parpadeo.intervalo_ms = intervalo;
            parpadeo.duracion_ms = duracion;
            assert!(validar_retrato(dto, Path::new("actors/prueba.ron"), "prueba").is_err());
        }
        let retrato = validar_retrato(
            retrato_de_prueba("x", None, vec![]),
            Path::new("actors/prueba.ron"),
            "prueba",
        )
        .unwrap();
        assert!(retrato.rostro().is_none());
        assert!(retrato.parpadeo().is_none());
    }

    #[test]
    fn retrato_rechaza_glifos_anchos_y_matrices_de_estilo_incompatibles() {
        let ruta = Path::new("actors/prueba.ron");
        let ancho = validar_retrato(retrato_de_prueba("界", None, vec![]), ruta, "prueba")
            .expect_err("un glifo ancho debe rechazarse");
        assert!(ancho.diagnosticos()[0].detalle().contains("glifo"));

        let matriz = validar_retrato(
            retrato_de_prueba("xx\nxx", Some("0"), vec![('0', estilo_dto())]),
            ruta,
            "prueba",
        )
        .expect_err("las matrices deben coincidir");
        assert!(matriz.diagnosticos()[0].detalle().contains("no coincide"));
    }

    #[test]
    fn retrato_rechaza_estilos_ausentes_y_claves_duplicadas() {
        let ruta = Path::new("actors/prueba.ron");
        let ausente = validar_retrato(retrato_de_prueba("x", Some("X"), vec![]), ruta, "prueba")
            .expect_err("un estilo no declarado debe rechazarse");
        assert!(ausente.diagnosticos()[0].detalle().contains("no declarado"));

        let duplicada = validar_retrato(
            retrato_de_prueba(
                "x",
                Some("0"),
                vec![('0', estilo_dto()), ('0', estilo_dto())],
            ),
            ruta,
            "prueba",
        )
        .expect_err("una clave duplicada debe rechazarse");
        assert!(duplicada.diagnosticos()[0].detalle().contains("duplicada"));
    }

    #[test]
    fn dos_instancias_no_comparten_estado_mutable() {
        let contenido = cargar_contenido(&raiz_base()).expect("fixture válido");
        let (mut mundo_uno, mut mapa_uno) = contenido.instanciar_estado().expect("instancia uno");
        let (mundo_dos, mapa_dos) = contenido.instanciar_estado().expect("instancia dos");
        let coordenada = CoordenadaMapa { x: 0, y: 0 };
        let tipo_original = mapa_dos
            .tipo_en(coordenada)
            .expect("celda existente")
            .clone();
        let quemado = ClaveTipoTile::nueva("bosqueQuemado".to_owned()).expect("clave válida");

        mapa_uno
            .cambiar_tipo(coordenada, quemado.clone())
            .expect("tipo declarado");
        mundo_uno
            .avanzar_fecha(Duracion::desde_dias(1))
            .expect("fecha válida");

        assert_eq!(mapa_uno.tipo_en(coordenada), Some(&quemado));
        assert_eq!(mapa_dos.tipo_en(coordenada), Some(&tipo_original));
        assert_eq!(mundo_dos.fecha().dia(), 31);
        assert_eq!(
            mapa_uno.tipos_declarados().count(),
            mapa_dos.tipos_declarados().count()
        );
    }

    #[test]
    fn politica_oficial_observa_la_fecha_sin_mutar_el_mundo() {
        let contenido = cargar_contenido(&raiz_base()).expect("fixture válido");
        let (mut mundo, _) = contenido.instanciar_estado().expect("instancia válida");
        let politica = &contenido.politicas()[0];

        assert_eq!(
            evaluar_politica(politica, &mundo, contenido.claves()).expect("evaluación de enero"),
            ResultadoPoliticaLua::Ninguna
        );
        mundo
            .avanzar_fecha(Duracion::desde_meses(1))
            .expect("fecha válida");
        let esperado = SolicitudAccion::Viajar {
            actor: IdActor::desde_posicion(2).expect("ID válido"),
            destino: IdLugar::desde_posicion(0).expect("ID válido"),
        };
        assert_eq!(
            evaluar_politica(politica, &mundo, contenido.claves()).expect("evaluación de febrero"),
            ResultadoPoliticaLua::Solicitud(esperado)
        );
        assert_eq!(mundo.fecha().mes(), Mes::Febrero);
        assert_eq!(
            mundo.ubicacion_actor(IdActor::desde_posicion(2).expect("ID válido")),
            Some(IdLugar::desde_posicion(2).expect("ID válido"))
        );
    }

    #[test]
    fn cambiar_tile_rechaza_sin_mutacion() {
        let contenido = cargar_contenido(&raiz_base()).expect("fixture válido");
        let (_, mut mapa) = contenido.instanciar_estado().expect("instancia válida");
        let coordenada = CoordenadaMapa { x: 0, y: 0 };
        let antes = mapa.tipo_en(coordenada).expect("celda existente").clone();
        let desconocido = ClaveTipoTile::nueva("nieve".to_owned()).expect("clave válida");
        assert!(mapa.cambiar_tipo(coordenada, desconocido).is_err());
        assert_eq!(mapa.tipo_en(coordenada), Some(&antes));
    }

    #[test]
    fn relaciones_de_contenido_rechazan_referencias_tipos_y_duplicados() {
        let lugar_cero = IdLugar::desde_posicion(0).expect("ID válido");
        let lugar_uno = IdLugar::desde_posicion(1).expect("ID válido");
        let actor_cero = IdActor::desde_posicion(0).expect("ID válido");
        let claves = CatalogoClaves {
            actores_por_clave: BTreeMap::from([("actor".to_owned(), actor_cero)]),
            claves_por_actor: BTreeMap::from([(actor_cero, "actor".to_owned())]),
            lugares_por_clave: BTreeMap::from([
                ("origen".to_owned(), lugar_cero),
                ("destino".to_owned(), lugar_uno),
            ]),
            claves_por_lugar: BTreeMap::from([
                (lugar_cero, "origen".to_owned()),
                (lugar_uno, "destino".to_owned()),
            ]),
        };
        let declaracion = || DeclaracionRelacionDto {
            tipo: TipoRelacionDto::Conectado,
            destino: DestinoRelacionDto::Lugar("destino".to_owned()),
        };
        let primera = validar_relaciones(
            vec![declaracion()],
            ExtremoRelacion::Lugar(lugar_cero),
            &claves,
            Path::new("places/origen.ron"),
            TipoElementoContenido::Lugar,
            "origen",
        )
        .unwrap_or_else(|_| panic!("Relación válida"));
        let inversa = validar_relaciones(
            vec![DeclaracionRelacionDto {
                tipo: TipoRelacionDto::Conectado,
                destino: DestinoRelacionDto::Lugar("origen".to_owned()),
            }],
            ExtremoRelacion::Lugar(lugar_uno),
            &claves,
            Path::new("places/destino.ron"),
            TipoElementoContenido::Lugar,
            "destino",
        )
        .unwrap_or_else(|_| panic!("Relación inversa aislada válida"));
        let duplicada = normalizar_relaciones(primera.iter().chain(inversa.iter()))
            .expect_err("la orientación inversa también duplica");
        assert_eq!(
            duplicada.diagnosticos()[0].clase(),
            ClaseDiagnosticoContenido::Duplicado
        );

        let actor = validar_relaciones(
            vec![declaracion()],
            ExtremoRelacion::Actor(actor_cero),
            &claves,
            Path::new("actors/actor.ron"),
            TipoElementoContenido::Actor,
            "actor",
        )
        .err()
        .expect("Conectado no admite Actores");
        assert_eq!(
            actor.diagnosticos()[0].clase(),
            ClaseDiagnosticoContenido::ValorInvalido
        );

        let desconocida = validar_relaciones(
            vec![DeclaracionRelacionDto {
                tipo: TipoRelacionDto::Conectado,
                destino: DestinoRelacionDto::Lugar("ausente".to_owned()),
            }],
            ExtremoRelacion::Lugar(lugar_cero),
            &claves,
            Path::new("places/origen.ron"),
            TipoElementoContenido::Lugar,
            "origen",
        )
        .err()
        .expect("una referencia inexistente se rechaza");
        assert_eq!(
            desconocida.diagnosticos()[0].clase(),
            ClaseDiagnosticoContenido::ReferenciaInvalida
        );
    }
}
