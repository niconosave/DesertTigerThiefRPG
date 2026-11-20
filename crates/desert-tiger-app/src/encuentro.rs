use crate::IdActor;
use crate::contenido::CatalogoClaves;
pub use desert_tiger_core::encuentros::Escenario as EscenarioEncuentro;
use serde::Deserialize;
use std::path::Path;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConfigEncuentros {
    #[serde(default)]
    pub rescates: Vec<RescateConfigurado>,
    #[serde(default)]
    pub recuperaciones: Vec<RecuperacionConfigurada>,
    pub objetivos: Vec<ObjetivoConfigurado>,
}
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub(crate) enum EscenarioConfigurado {
    #[default]
    Automatico,
    Recinto,
    Campamento,
    Palacio,
    Ciudad,
    Bosque,
    Montana,
    Playa,
}
impl EscenarioConfigurado {
    pub fn motor(
        self,
        mundo: &desert_tiger_core::Mundo,
    ) -> desert_tiger_core::encuentros::Escenario {
        use desert_tiger_core::encuentros::Escenario;
        match self {
            Self::Automatico => {
                let lugar = mundo.lugar(mundo.ubicacion_grupo());
                let tiene = |tag: &str| {
                    lugar.is_some_and(|l| l.caracteristicas().any(|c| c.como_str() == tag))
                };
                if tiene("civilizado") {
                    Escenario::Ciudad
                } else if tiene("boscoso") {
                    Escenario::Bosque
                } else if tiene("montanoso") {
                    Escenario::Montana
                } else if tiene("costero") {
                    Escenario::Playa
                } else {
                    Escenario::Campamento
                }
            }
            Self::Palacio => Escenario::Palacio,
            Self::Ciudad => Escenario::Ciudad,
            Self::Bosque => Escenario::Bosque,
            Self::Montana => Escenario::Montana,
            Self::Playa => Escenario::Playa,
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
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RescateConfigurado {
    pub actor: String,
    pub captor: String,
    #[serde(default)]
    pub defensores: Vec<String>,
    #[serde(default)]
    pub escenario: EscenarioConfigurado,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecuperacionConfigurada {
    pub objeto: String,
    pub custodio: String,
    #[serde(default)]
    pub defensores: Vec<String>,
    #[serde(default)]
    pub escenario: EscenarioConfigurado,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjetivoObjeto {
    pub objeto: crate::IdObjeto,
    pub nombre: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjetoEncuentro {
    pub objeto: crate::IdObjeto,
    pub nombre: String,
    pub x: u16,
    pub y: u16,
    pub recogido: bool,
    pub fuera: bool,
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
        let mut vistos = std::collections::BTreeSet::new();
        for r in &c.rescates {
            if !vistos.insert(&r.actor)
                || r.actor == r.captor
                || claves.actor_por_clave(&r.actor).is_none()
                || claves.actor_por_clave(&r.captor).is_none()
                || r.defensores.len() > 8
            {
                return Err("encounters.ron: rescate inválido".into());
            }
            let mut defensa = std::collections::BTreeSet::new();
            for d in &r.defensores {
                if d == &r.actor
                    || d == &r.captor
                    || !defensa.insert(d)
                    || claves.actor_por_clave(d).is_none()
                {
                    return Err("encounters.ron: defensor de rescate inválido".into());
                }
            }
        }
        let mut objetos = std::collections::BTreeSet::new();
        for r in &c.recuperaciones {
            if !objetos.insert(&r.objeto)
                || claves.objeto_por_clave(&r.objeto).is_none()
                || claves.actor_por_clave(&r.custodio).is_none()
                || r.defensores.len() > 8
            {
                return Err("encounters.ron: recuperación inválida".into());
            }
            let mut defensa = std::collections::BTreeSet::new();
            for d in &r.defensores {
                if d == &r.custodio || !defensa.insert(d) || claves.actor_por_clave(d).is_none() {
                    return Err("encounters.ron: defensor de tesoro inválido".into());
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
    pub cautivo: bool,
    pub rescatado: bool,
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
    pub recuperacion: bool,
    pub objeto: Option<ObjetoEncuentro>,
    pub rescate: bool,
    pub escenario: EscenarioEncuentro,
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
