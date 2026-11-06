use super::lua::Programa;
use crate::contenido::{CatalogoClaves, clave_externa_valida, validar_texto_presentacion};
use desert_tiger_core::ExtremoRelacion;
use desert_tiger_core::acciones::intervenciones::ClasePunto;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Default, Debug, Deserialize, PartialEq, Eq)]
pub enum Modo {
    #[default]
    Insertar,
    SustituirRelato,
}
#[derive(Clone, Copy, Default, Debug, Deserialize)]
pub enum Repeticion {
    #[default]
    UnaVezPorPartida,
}
#[derive(Clone, Debug, Deserialize)]
pub enum Objetivo {
    Actor(String),
    Lugar(String),
}
impl Objetivo {
    pub fn resolver(&self, c: &CatalogoClaves) -> Result<ExtremoRelacion, String> {
        match self {
            Self::Actor(s) => c.actor_por_clave(s).map(ExtremoRelacion::Actor),
            Self::Lugar(s) => c.lugar_por_clave(s).map(ExtremoRelacion::Lugar),
        }
        .ok_or_else(|| format!("referencia inexistente: {self:?}"))
    }
}
#[derive(Clone, Default, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Filtros {
    pub actor: Option<String>,
    pub lugar: Option<String>,
    pub objetivo: Option<Objetivo>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definicion {
    pub clave: String,
    pub puntos: Vec<String>,
    #[serde(default)]
    pub modo: Modo,
    #[serde(default)]
    pub prioridad: i32,
    #[serde(default)]
    pub repeticion: Repeticion,
    #[serde(default)]
    pub filtros: Filtros,
    pub condicion: Programa,
    pub inicio: String,
    #[serde(default)]
    pub encadena: Vec<String>,
    pub pasos: Vec<Paso>,
    #[serde(skip)]
    pub archivo: PathBuf,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Paso {
    Vigneta {
        clave: String,
        presentacion: Programa,
        ilustracion: Option<String>,
        expresion: Option<String>,
        #[serde(default)]
        opciones: Vec<Opcion>,
        siguiente: Option<String>,
    },
    Efectos {
        clave: String,
        salidas: Vec<String>,
        resolver: Programa,
    },
    Fin {
        clave: String,
        control: Control,
    },
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Control {
    Continuar,
    InterrumpirAccion,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opcion {
    pub clave: String,
    pub texto: String,
    pub siguiente: String,
}
impl Paso {
    pub fn clave(&self) -> &str {
        match self {
            Self::Vigneta { clave, .. } | Self::Efectos { clave, .. } | Self::Fin { clave, .. } => {
                clave
            }
        }
    }
    fn salidas(&self) -> Vec<&str> {
        match self {
            Self::Vigneta {
                opciones,
                siguiente,
                ..
            } => siguiente
                .iter()
                .map(String::as_str)
                .chain(opciones.iter().map(|o| o.siguiente.as_str()))
                .collect(),
            Self::Efectos { salidas, .. } => salidas.iter().map(String::as_str).collect(),
            Self::Fin { .. } => vec![],
        }
    }
}
impl Definicion {
    pub fn paso(&self, clave: &str) -> Result<&Paso, String> {
        self.pasos
            .iter()
            .find(|p| p.clave() == clave)
            .ok_or_else(|| format!("paso inexistente {clave}"))
    }
    pub fn interrumpe(&self) -> bool {
        self.pasos.iter().any(|p| {
            matches!(
                p,
                Paso::Fin {
                    control: Control::InterrumpirAccion,
                    ..
                }
            )
        })
    }
    pub fn permite(&self, punto: ClasePunto) -> bool {
        (!self.interrumpe() || punto.permite_interrumpir())
            && (self.modo != Modo::SustituirRelato || punto == ClasePunto::RelatoLlegada)
    }
}
#[derive(Debug, Default)]
pub struct Catalogo {
    pub eventos: BTreeMap<String, Definicion>,
    pub indice: BTreeMap<ClasePunto, Vec<String>>,
}
fn clave(s: &str) -> Result<(), String> {
    if clave_externa_valida(s) {
        Ok(())
    } else {
        Err(format!("clave inválida: {s:?}"))
    }
}
fn unicos<'a>(items: impl Iterator<Item = &'a str>) -> Result<(), String> {
    let mut vistos = BTreeSet::new();
    for s in items {
        clave(s)?;
        if !vistos.insert(s) {
            return Err(format!("clave repetida {s}"));
        }
    }
    Ok(())
}
fn grafo(g: &BTreeMap<String, Vec<String>>, inicio: &str) -> Result<BTreeSet<String>, String> {
    let mut visitados = BTreeSet::new();
    let mut activos = BTreeSet::new();
    let mut pila = vec![(inicio.to_owned(), false)];
    while let Some((n, salir)) = pila.pop() {
        if salir {
            activos.remove(&n);
            visitados.insert(n);
            continue;
        }
        if activos.contains(&n) {
            return Err(format!("ciclo en {n}"));
        }
        if visitados.contains(&n) {
            continue;
        }
        let destinos = g
            .get(&n)
            .ok_or_else(|| format!("referencia inexistente {n}"))?;
        activos.insert(n.clone());
        pila.push((n, true));
        for d in destinos.iter().rev() {
            pila.push((d.clone(), false));
        }
    }
    Ok(visitados)
}
fn archivos(dir: &Path, salida: &mut Vec<PathBuf>) -> Result<(), String> {
    for entrada in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entrada = entrada.map_err(|e| e.to_string())?;
        let p = entrada.path();
        let t = entrada.file_type().map_err(|e| e.to_string())?;
        if t.is_symlink() {
            return Err(format!("{}: no se admiten enlaces simbólicos", p.display()));
        }
        if t.is_dir() {
            archivos(&p, salida)?;
        } else if t.is_file() && p.extension().is_some_and(|e| e == "ron") {
            salida.push(p);
        }
    }
    Ok(())
}
pub fn cargar(raiz: &Path, claves: &CatalogoClaves) -> Result<Catalogo, String> {
    let dir = raiz.join("events");
    if !dir.exists() {
        return Ok(Catalogo::default());
    }
    if std::fs::symlink_metadata(&dir)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err("events: enlace simbólico no admitido".into());
    }
    let mut rutas = vec![];
    archivos(&dir, &mut rutas)?;
    rutas.sort();
    let mut c = Catalogo::default();
    for ruta in rutas {
        let texto =
            std::fs::read_to_string(&ruta).map_err(|e| format!("{}: {e}", ruta.display()))?;
        let mut e: Definicion =
            ron::from_str(&texto).map_err(|err| format!("{}: {err}", ruta.display()))?;
        let validar = |e: &mut Definicion| -> Result<(), String> {
            let Repeticion::UnaVezPorPartida = e.repeticion;
            clave(&e.clave)?;
            if ruta.file_stem().and_then(|s| s.to_str()) != Some(&e.clave) {
                return Err("clave no coincide con archivo".into());
            }
            if e.puntos.is_empty() || e.pasos.is_empty() {
                return Err("puntos/pasos vacíos".into());
            }
            let mut puntos = BTreeSet::new();
            for p in &e.puntos {
                let p =
                    ClasePunto::desde_clave(p).ok_or_else(|| format!("punto no soportado {p}"))?;
                if !puntos.insert(p) || !e.permite(p) {
                    return Err("punto repetido o control no admitido".into());
                }
            }
            if let Some(a) = &e.filtros.actor
                && claves.actor_por_clave(a).is_none()
            {
                return Err(format!("Actor inexistente {a}"));
            }
            if let Some(l) = &e.filtros.lugar
                && claves.lugar_por_clave(l).is_none()
            {
                return Err(format!("Lugar inexistente {l}"));
            }
            if let Some(o) = &e.filtros.objetivo {
                o.resolver(claves)?;
            }
            unicos(e.pasos.iter().map(Paso::clave))?;
            unicos(e.encadena.iter().map(String::as_str))?;
            e.condicion
                .compilar(&format!("{}:condicion", ruta.display()))?;
            for p in &mut e.pasos {
                match p {
                    Paso::Vigneta {
                        clave,
                        presentacion,
                        opciones,
                        siguiente,
                        ilustracion,
                        expresion,
                    } => {
                        if opciones.is_empty() == siguiente.is_none() {
                            return Err(format!("{clave}: opciones/siguiente incompatibles"));
                        }
                        unicos(opciones.iter().map(|o| o.clave.as_str()))?;
                        for o in opciones {
                            validar_texto_presentacion(&o.texto, false, true)?;
                        }
                        for k in ilustracion.iter().chain(expresion.iter()) {
                            crate::eventos::contenido::clave(k)?;
                        }
                        presentacion
                            .compilar(&format!("{}:{clave}:presentacion", ruta.display()))?;
                    }
                    Paso::Efectos {
                        clave,
                        salidas,
                        resolver,
                    } => {
                        if salidas.is_empty() {
                            return Err(format!("{clave}: salidas vacías"));
                        }
                        unicos(salidas.iter().map(String::as_str))?;
                        resolver.compilar(&format!("{}:{clave}:efectos", ruta.display()))?;
                    }
                    Paso::Fin { .. } => {}
                }
            }
            let g = e
                .pasos
                .iter()
                .map(|p| {
                    (
                        p.clave().to_owned(),
                        p.salidas().into_iter().map(str::to_owned).collect(),
                    )
                })
                .collect();
            if grafo(&g, &e.inicio)?.len() != e.pasos.len() {
                return Err("pasos inalcanzables".into());
            }
            Ok(())
        };
        validar(&mut e).map_err(|err| format!("{}: {err}", ruta.display()))?;
        e.archivo = ruta;
        if c.eventos.insert(e.clave.clone(), e).is_some() {
            return Err("clave de evento repetida".into());
        }
    }
    let g = c
        .eventos
        .iter()
        .map(|(k, e)| (k.clone(), e.encadena.clone()))
        .collect();
    for k in c.eventos.keys() {
        grafo(&g, k)?;
    }
    for (k, e) in &c.eventos {
        for p in &e.puntos {
            c.indice
                .entry(ClasePunto::desde_clave(p).ok_or("punto inválido")?)
                .or_default()
                .push(k.clone());
        }
    }
    for v in c.indice.values_mut() {
        v.sort_by(|a, b| {
            c.eventos[b]
                .prioridad
                .cmp(&c.eventos[a].prioridad)
                .then(a.cmp(b))
        });
    }
    Ok(c)
}
