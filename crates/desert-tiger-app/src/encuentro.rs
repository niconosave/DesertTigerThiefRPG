use crate::IdActor;
use crate::contenido::CatalogoClaves;
use serde::Deserialize;
use std::path::Path;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConfigEncuentros {
    pub objetivos: Vec<ObjetivoConfigurado>,
}
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub(crate) enum EscenarioConfigurado {
    #[default]
    Recinto,
    Campamento,
}
impl EscenarioConfigurado {
    pub fn motor(self) -> desert_tiger_core::encuentros::Escenario {
        match self {
            Self::Recinto => desert_tiger_core::encuentros::Escenario::Recinto,
            Self::Campamento => desert_tiger_core::encuentros::Escenario::Campamento,
        }
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ObjetivoConfigurado {
    #[serde(default)]
    pub escenario: EscenarioConfigurado,
    pub actor: String,
    #[serde(default)]
    pub defensores: Vec<String>,
}
impl ConfigEncuentros {
    pub(crate) fn cargar(raiz: &Path, claves: &CatalogoClaves) -> Result<Option<Self>, String> {
        let ruta = raiz.join("encounters.ron");
        let texto = match std::fs::read_to_string(&ruta) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("{}: {e}", ruta.display())),
        };
        let c: Self = ron::from_str(&texto).map_err(|e| format!("{}: {e}", ruta.display()))?;
        let mut vistos = std::collections::BTreeSet::new();
        for o in &c.objetivos {
            if !vistos.insert(&o.actor)
                || claves.actor_por_clave(&o.actor).is_none()
                || o.defensores.len() > 8
            {
                return Err("encounters.ron: objetivo inválido o repetido".into());
            }
            let mut defensa = std::collections::BTreeSet::new();
            for d in &o.defensores {
                if d == &o.actor || !defensa.insert(d) || claves.actor_por_clave(d).is_none() {
                    return Err("encounters.ron: defensor inválido o repetido".into());
                }
            }
        }
        Ok(Some(c))
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjetivoAtaque {
    pub actor: IdActor,
    pub nombre: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerrenoEncuentro {
    Desconocido,
    Suelo,
    Muro,
    Vacio,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FichaEncuentro {
    pub id: usize,
    pub impacto: Option<bool>,
    pub nombre: String,
    pub x: u16,
    pub y: u16,
    pub aliado: bool,
    pub golpes: u8,
    pub salio: bool,
    pub transportado: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaEncuentro {
    pub id: u64,
    pub paso: u64,
    pub ancho: u16,
    pub alto: u16,
    pub celdas: Vec<TerrenoEncuentro>,
    pub fichas: Vec<FichaEncuentro>,
    pub texto: String,
    pub detalle: Vec<String>,
    pub ronda: u64,
    pub finalizado: bool,
    pub puede_retirar: bool,
    pub pausa_decision: bool,
    pub retirando: bool,
}
