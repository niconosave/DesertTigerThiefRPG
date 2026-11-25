//! Encuentro local efímero. Cada pulso elige intenciones desde una sola instantánea.
mod comportamientos;
mod conocimiento;
mod elementos;
mod error;
mod espacio;
pub use error::ErrorEncuentro;
mod generacion;
mod planes;
mod puertas;
mod resultado;
pub use conocimiento::ContextoNavegacion;
pub use elementos::{AparienciaLocal, EnvolventeLocal};
pub use planes::{Cometido, DireccionFicha, MotivoBloqueo, RolEncuentro};
pub use resultado::{CausaFin, ResultadoBien, ResultadoEncuentro, ResultadoFicha};

use crate::azar::FuenteAzar;
use crate::{ExtremoRelacion, IdActor, IdObjeto, Mundo};
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

macro_rules! id_local {
    ($n:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $n(pub u16);
    };
}
id_local!(IdFicha);
id_local!(IdElementoLocal);
id_local!(IdEspacio);
id_local!(IdConexion);
id_local!(IdPista);
id_local!(IdPlan);
id_local!(IdComportamiento);
pub const MAX_ANCHO: u16 = 64;
pub const MAX_PROFUNDIDAD: u16 = 64;
pub const MAX_ALTURA: u16 = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorGeometria {
    VolumenInvalido,
    PerfilNoCabe(Posicion),
    VolumenOcupado(IdFicha),
    ReferenciaInvalida,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImpedimentoOcupacion {
    FueraDeLimites,
    SinApoyoOAltura,
    PuertaCerrada(IdElementoLocal),
    Obstaculo(IdElementoLocal),
    Ficha(IdFicha),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Posicion {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
pub type PosicionLocal = Posicion;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstadoPuerta {
    Abierta,
    Cerrada,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClaseElemento {
    PuertaManual {
        conexion: IdConexion,
        estado: EstadoPuerta,
    },
    Obstaculo,
    PresenciaProbable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementoLocal {
    pub id: IdElementoLocal,
    pub clase: ClaseElemento,
    pub posicion: Posicion,
    pub alto: u16,
    pub definido: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EspacioLocal {
    pub id: IdEspacio,
    pub nombre: String,
    pub min: Posicion,
    pub max: Posicion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConexionLocal {
    pub id: IdConexion,
    pub desde: IdEspacio,
    pub hasta: IdEspacio,
    pub recorrido: Vec<Posicion>,
    pub control: Option<IdElementoLocal>,
    pub salida: bool,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContextoLocal {
    pub espacios: Vec<EspacioLocal>,
    pub conexiones: Vec<ConexionLocal>,
    pub revision_geometria: u64,
    pub revision_conocimiento: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropositoPlan {
    Atacar,
    Rescatar,
    Recuperar,
    Retirarse,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstadoPlan {
    Activo,
    Suspendido,
    Bloqueado,
    Cumplido,
    Cancelado,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanLocal {
    pub id: IdPlan,
    pub proposito: PropositoPlan,
    pub prioridad: u8,
    pub participantes: Vec<IdFicha>,
    pub estado: EstadoPlan,
    pub bloqueo: Option<MotivoBloqueo>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialLocal {
    Piedra,
    Madera,
    Metal,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Celda {
    Aire,
    Suelo,
    Muro,
    Solido(MaterialLocal),
    Vacio,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ficha {
    pub id: IdFicha,
    pub cautivo: bool,
    pub actor: IdActor,
    pub posicion: Posicion,
    pub aliado: bool,
    pub golpes: u8,
    pub ataque: i16,
    pub defensa: i16,
    pub salio: bool,
    pub transportado_por: Option<IdFicha>,
}
impl Ficha {
    pub fn puede_actuar(&self) -> bool {
        !self.cautivo && self.golpes < 2 && !self.salio && self.transportado_por.is_none()
    }
    pub fn muerto(&self) -> bool {
        self.golpes >= 3
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Desenlace {
    Exito,
    Retirada,
    Captura { captor: IdActor },
    Derrota,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Suceso {
    PuertaCambiada {
        elemento: IdElementoLocal,
        por: IdFicha,
        estado: EstadoPuerta,
    },
    ObjetoRecogido {
        por: IdFicha,
    },
    Liberado {
        ficha: IdFicha,
        por: IdFicha,
    },
    Paso {
        ficha: IdFicha,
        desde: Posicion,
        hasta: Posicion,
    },
    Golpe {
        atacante: IdFicha,
        blanco: IdFicha,
        acierto: bool,
        reaccion: bool,
    },
    Recogido {
        ficha: IdFicha,
        por: IdFicha,
    },
}
/// Transformaciones explícitas de autor, sin simular su causa narrativa.
#[derive(Clone, Debug)]
pub enum ComandoEncuentro {
    Voxel { posicion: Posicion, terreno: Celda },
    Despejar { posicion: Posicion, alto: u16 },
    Reubicar { actor: IdActor, posicion: Posicion },
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Escenario {
    #[default]
    Recinto,
    Campamento,
    Palacio,
    Ciudad,
    Bosque,
    Montana,
    Playa,
    DosAlturas,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Botin {
    pub ubicacion_inicial: ExtremoRelacion,
    pub objeto: IdObjeto,
    pub posicion: Posicion,
    pub portador: Option<IdFicha>,
    pub fuera: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encuentro {
    pub lugar: crate::IdLugar,
    pub botin: Option<Botin>,
    pub rescate: bool,
    pub escenario: Escenario,
    pub referencias: BTreeMap<String, Posicion>,
    pub ancho: u16,
    pub alto: u16,
    pub altura: u16,
    celdas: Vec<Celda>,
    pub elementos: Vec<ElementoLocal>,
    pub contexto: ContextoLocal,
    pub planes: Vec<PlanLocal>,
    interacciones_puerta: BTreeMap<IdFicha, (IdPlan, IdElementoLocal, EstadoPuerta)>,
    puertas_inicio: BTreeMap<IdElementoLocal, EstadoPuerta>,
    direcciones: BTreeMap<IdFicha, DireccionFicha>,
    proximo_comportamiento: u16,
    rutas_decision: BTreeMap<IdFicha, BTreeMap<Posicion, (u32, Posicion)>>,
    pub fichas: Vec<Ficha>,
    pub objetivo: Option<IdFicha>,
    pub entrada: Posicion,
    pub pulso: u64,
    pub retirando: bool,
    pub descubierta: bool,
    espacios_descubiertos: BTreeSet<IdEspacio>,
    memoria_terreno: BTreeMap<Posicion, Celda>,
    memoria_elementos: BTreeMap<IdElementoLocal, ElementoLocal>,
    pub sucesos: Vec<Suceso>,
    pub desenlace: Option<Desenlace>,
    pub capturas: BTreeMap<IdFicha, IdFicha>,
    pub resultado: Option<ResultadoEncuentro>,
    pub decision_evacuacion: Vec<IdFicha>,
    abandonados_confirmados: BTreeSet<IdFicha>,
    evacuados: BTreeMap<IdFicha, IdFicha>,
    prioridad: Vec<usize>,
    sin_progreso: u8,
    pub bloqueado: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Intento {
    Esperar,
    Mover(Posicion),
    Atacar(usize),
    Recoger(usize),
    Liberar,
    TomarObjeto,
    Salir,
    Abrir(IdElementoLocal),
    Cerrar(IdElementoLocal),
}
fn sortear(azar: &mut dyn FuenteAzar, n: usize) -> Result<usize, ErrorEncuentro> {
    azar.elegir_indice(NonZeroUsize::new(n).ok_or("sorteo vacío")?)
        .map_err(|e| ErrorEncuentro::Azar(e.to_string()))
}
fn distancia(a: Posicion, b: Posicion) -> u16 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}
impl Encuentro {
    pub fn espacio_en(&self, p: Posicion) -> Option<IdEspacio> {
        self.contexto
            .espacios
            .iter()
            .find(|e| {
                (e.min.x..=e.max.x).contains(&p.x)
                    && (e.min.y..=e.max.y).contains(&p.y)
                    && (e.min.z..=e.max.z).contains(&p.z)
            })
            .map(|e| e.id)
    }

    pub fn conexiones_desde(&self, espacio: IdEspacio) -> impl Iterator<Item = &ConexionLocal> {
        self.contexto
            .conexiones
            .iter()
            .filter(move |c| c.desde == espacio || c.hasta == espacio)
    }
    pub fn terreno_descubierto(&self, p: Posicion) -> bool {
        self.espacio_en(p)
            .is_some_and(|id| self.espacios_descubiertos.contains(&id))
    }

    fn conexion_abierta(&self, c: &ConexionLocal) -> bool {
        c.control.is_none_or(|id| {
            self.elementos.iter().any(|e| {
                e.id == id
                    && matches!(
                        e.clase,
                        ClaseElemento::PuertaManual {
                            estado: EstadoPuerta::Abierta,
                            ..
                        }
                    )
            })
        })
    }

    pub fn ficha_observable(&self, f: &Ficha) -> bool {
        if f.aliado && !f.cautivo {
            return true;
        }
        let Some(destino) = self.espacio_en(f.posicion) else {
            return false;
        };
        self.espacio_observable(destino)
    }

    fn espacio_observable(&self, destino: IdEspacio) -> bool {
        if !self.espacios_descubiertos.contains(&destino) {
            return false;
        }
        self.fichas
            .iter()
            .filter(|a| a.aliado && !a.cautivo && !a.salio && !a.muerto())
            .filter_map(|a| self.espacio_en(a.posicion))
            .any(|origen| {
                origen == destino
                    || self.conexiones_desde(origen).any(|c| {
                        !c.salida
                            && (c.desde == destino || c.hasta == destino)
                            && self.conexion_abierta(c)
                    })
            })
    }

    fn recordar_terreno(&mut self) {
        let observables: BTreeSet<_> = self
            .espacios_descubiertos
            .iter()
            .copied()
            .filter(|&id| self.espacio_observable(id))
            .collect();
        let muestras: Vec<_> = self
            .voxeles()
            .filter(|(p, _)| {
                self.espacio_en(*p)
                    .is_some_and(|id| observables.contains(&id))
            })
            .collect();
        self.memoria_terreno.extend(muestras);
        for e in &self.elementos {
            if self
                .espacio_en(e.posicion)
                .is_some_and(|id| observables.contains(&id))
            {
                self.memoria_elementos.insert(e.id, e.clone());
            }
        }
    }

    pub fn terreno_recordado(&self, p: Posicion) -> Option<Celda> {
        self.memoria_terreno.get(&p).copied()
    }

    pub fn indice_objetivo(&self) -> Option<usize> {
        let id = self.objetivo?;
        self.fichas.iter().position(|f| f.id == id)
    }

    pub fn ficha(&self, id: IdFicha) -> Option<&Ficha> {
        self.fichas.iter().find(|f| f.id == id)
    }

    pub fn nuevo(
        mundo: &Mundo,
        grupo: &[IdActor],
        objetivo: IdActor,
        defensores: &[IdActor],
        azar: &mut dyn FuenteAzar,
    ) -> Result<Self, ErrorEncuentro> {
        Self::nuevo_con_escenario(mundo, grupo, objetivo, defensores, azar, Escenario::Recinto)
    }
    pub fn nuevo_con_escenario(
        mundo: &Mundo,
        grupo: &[IdActor],
        objetivo: IdActor,
        defensores: &[IdActor],
        azar: &mut dyn FuenteAzar,
        escenario: Escenario,
    ) -> Result<Self, ErrorEncuentro> {
        Self::construir(mundo, grupo, Some(objetivo), defensores, azar, escenario)
    }
    pub fn nuevo_recuperacion(
        mundo: &Mundo,
        grupo: &[IdActor],
        objeto: IdObjeto,
        guardias: &[IdActor],
        azar: &mut dyn FuenteAzar,
        escenario: Escenario,
    ) -> Result<Self, ErrorEncuentro> {
        if mundo.objeto(objeto).is_none()
            || mundo.ubicacion_objeto(objeto) != Some(mundo.ubicacion_grupo())
        {
            return Err(ErrorEncuentro::ReferenciaInvalida {
                referencia: ExtremoRelacion::Objeto(objeto),
                detalle: "objeto fuera del lugar",
            });
        }
        let mut e = Self::construir(mundo, grupo, None, guardias, azar, escenario)?;
        let posicion = Posicion { z: 1, x: 27, y: 8 };
        e.celdas[usize::from(posicion.y * e.ancho + posicion.x)] = Celda::Suelo;
        e.botin = Some(Botin {
            ubicacion_inicial: mundo
                .destino_ubicacion(ExtremoRelacion::Objeto(objeto))
                .ok_or("objeto sin ubicación")?,
            objeto,
            posicion,
            portador: None,
            fuera: false,
        });
        Ok(e)
    }
    pub fn objetivo_mundo(&self) -> ExtremoRelacion {
        if let Some(b) = &self.botin {
            ExtremoRelacion::Objeto(b.objeto)
        } else {
            ExtremoRelacion::Actor(
                self.fichas[self.indice_objetivo().expect("objetivo actor")].actor,
            )
        }
    }
    pub fn objetivo_actor(&self) -> Option<IdActor> {
        self.indice_objetivo().map(|i| self.fichas[i].actor)
    }
    pub fn objetivo_muerto(&self) -> bool {
        self.indice_objetivo()
            .is_some_and(|i| self.fichas[i].muerto())
    }
    pub fn extraccion_fuera(&self) -> bool {
        self.rescatado_fuera() || self.botin.as_ref().is_some_and(|b| b.fuera)
    }
    pub fn nuevo_rescate(
        mundo: &Mundo,
        grupo: &[IdActor],
        cautivo: IdActor,
        guardias: &[IdActor],
        azar: &mut dyn FuenteAzar,
        escenario: Escenario,
    ) -> Result<Self, ErrorEncuentro> {
        let mut e = Self::nuevo_con_escenario(mundo, grupo, cautivo, guardias, azar, escenario)?;
        e.rescate = true;
        e.planes[0].proposito = PropositoPlan::Rescatar;
        let cautivo_id = e.objetivo.expect("objetivo del rescate");
        for plan in e.planes.iter_mut().skip(1) {
            plan.participantes.retain(|id| *id != cautivo_id);
        }
        let i = e.indice_objetivo().unwrap();
        e.fichas[i].aliado = true;
        e.fichas[i].cautivo = true;
        Ok(e)
    }
    pub fn rescatado_fuera(&self) -> bool {
        self.rescate
            && self.fichas[self.indice_objetivo().unwrap()].salio
            && !self.fichas[self.indice_objetivo().unwrap()].muerto()
    }
    pub fn aplicar_comandos(
        &mut self,
        comandos: &[ComandoEncuentro],
        mundo: &Mundo,
    ) -> Result<(), ErrorEncuentro> {
        let mut candidato = self.clone();
        for comando in comandos {
            let posicion = match comando {
                ComandoEncuentro::Voxel { posicion, .. }
                | ComandoEncuentro::Despejar { posicion, .. }
                | ComandoEncuentro::Reubicar { posicion, .. } => *posicion,
            };
            if candidato.terreno(posicion).is_none() {
                return Err(ErrorEncuentro::PosicionFueraDeLimites(posicion));
            }
            let alto_editado = match *comando {
                ComandoEncuentro::Voxel { .. } => 1,
                ComandoEncuentro::Despejar { alto, .. } => alto,
                ComandoEncuentro::Reubicar { .. } => 0,
            };
            if alto_editado > 0
                && candidato.elementos.iter().any(|e| {
                    e.posicion.x == posicion.x
                        && e.posicion.y == posicion.y
                        && posicion.z < e.posicion.z.saturating_add(e.alto)
                        && e.posicion.z < posicion.z.saturating_add(alto_editado)
                })
            {
                return Err("la edición de terreno intersecta un elemento local".into());
            }
            match *comando {
                ComandoEncuentro::Voxel { terreno, .. } => {
                    if terreno != Celda::Aire
                        && (posicion == candidato.entrada
                            || candidato
                                .botin
                                .as_ref()
                                .is_some_and(|b| !b.fuera && b.posicion == posicion)
                            || candidato
                                .fichas
                                .iter()
                                .any(|f| !f.salio && f.posicion == posicion))
                    {
                        return Err("terreno bloquea una ficha o la salida".into());
                    }
                    let indice = usize::from(
                        ((posicion.z * candidato.alto + posicion.y) * candidato.ancho) + posicion.x,
                    );
                    candidato.celdas[indice] = terreno;
                }
                ComandoEncuentro::Despejar { alto, .. } => {
                    if alto == 0
                        || posicion.z == 0
                        || posicion.z.saturating_add(alto) > candidato.altura
                    {
                        return Err("despeje inválido".into());
                    }
                    for z in posicion.z..posicion.z + alto {
                        let p = Posicion { z, ..posicion };
                        if candidato
                            .fichas
                            .iter()
                            .any(|f| f.puede_actuar() && f.posicion == p)
                        {
                            return Err("despeje intersecta ficha".into());
                        }
                        let indice = usize::from(
                            ((z * candidato.alto + posicion.y) * candidato.ancho) + posicion.x,
                        );
                        candidato.celdas[indice] = Celda::Aire;
                    }
                }
                ComandoEncuentro::Reubicar { actor, .. } => {
                    if !candidato.suelo(posicion) {
                        return Err("destino no transitable".into());
                    }
                    let indice = candidato
                        .fichas
                        .iter()
                        .position(|f| f.actor == actor)
                        .ok_or("actor fuera del encuentro")?;
                    if !candidato.fichas[indice].puede_actuar()
                        || candidato
                            .fichas
                            .iter()
                            .any(|f| f.transportado_por == Some(candidato.fichas[indice].id))
                        || candidato
                            .fichas
                            .iter()
                            .enumerate()
                            .any(|(i, f)| i != indice && f.puede_actuar() && f.posicion == posicion)
                    {
                        return Err("reubicación incompatible con ocupación o estado".into());
                    }
                    candidato.fichas[indice].posicion = posicion;
                }
            }
        }
        // La pertenencia y estados mundiales no pueden desincronizar las fichas.
        for f in &candidato.fichas {
            let a = mundo
                .actor(f.actor)
                .ok_or(ErrorEncuentro::ReferenciaInvalida {
                    referencia: ExtremoRelacion::Actor(f.actor),
                    detalle: "actor ausente",
                })?;
            let tiene = |tag: &str| a.caracteristicas().any(|c| c.como_str() == tag);
            let golpes = if tiene("muerto") {
                3
            } else if tiene("incapacitado") {
                2
            } else if tiene("herido") {
                1
            } else {
                0
            };
            if mundo.ubicacion_actor(f.actor) != Some(mundo.ubicacion_grupo())
                || golpes != f.golpes
                || f.ataque != if tiene("fuerte") { 2 } else { 0 }
                || f.defensa != if tiene("agil") { 2 } else { 0 }
            {
                return Err("el evento desincroniza el actor y su ficha".into());
            }
        }
        if let Some(b) = &mut candidato.botin
            && let Some(i) = b.portador
        {
            b.posicion = candidato
                .fichas
                .iter()
                .find(|f| f.id == i)
                .ok_or("portador inexistente")?
                .posicion;
        }
        if let Some(b) = &candidato.botin {
            let esperado = if b.fuera {
                ExtremoRelacion::Actor(mundo.actor_jugador())
            } else {
                b.ubicacion_inicial
            };
            if mundo.destino_ubicacion(ExtremoRelacion::Objeto(b.objeto)) != Some(esperado) {
                return Err("el evento desincroniza el tesoro".into());
            }
        }
        candidato
            .validar_geometria()
            .map_err(ErrorEncuentro::Geometria)?;
        if candidato.celdas != self.celdas {
            candidato.contexto.revision_geometria = self.contexto.revision_geometria + 1;
        }
        candidato.recordar_terreno();
        candidato.sucesos.clear();
        candidato.bloqueado = false;
        candidato.sin_progreso = 0;
        *self = candidato;
        Ok(())
    }

    pub fn avanzar(&mut self, azar: &mut dyn FuenteAzar) -> Result<(), ErrorEncuentro> {
        if self.desenlace.is_some() || !self.decision_evacuacion.is_empty() {
            return Err(ErrorEncuentro::EstadoIncompatible {
                operacion: "avanzar un encuentro terminado o pendiente de decisión",
            });
        }
        let mut candidato = self.clone();
        candidato.resolver(azar)?;
        *self = candidato;
        Ok(())
    }
    fn resolver(&mut self, azar: &mut dyn FuenteAzar) -> Result<(), ErrorEncuentro> {
        self.pulso = self.pulso.checked_add(1).ok_or("contador agotado")?;
        self.sucesos.clear();
        self.puertas_inicio = self
            .elementos
            .iter()
            .filter_map(|e| match e.clase {
                ClaseElemento::PuertaManual { estado, .. } => Some((e.id, estado)),
                _ => None,
            })
            .collect();
        self.coordinar_planes();
        let antes = self.fichas.clone();
        let mut intentos = (0..antes.len()).map(|i| self.elegir(i)).collect::<Vec<_>>();
        for intento in &mut intentos {
            if let Intento::Mover(p) = *intento
                && let Some(puerta) = self.puerta_cerrada_en(p)
            {
                *intento = Intento::Abrir(puerta);
            }
        }
        for (ficha, intento) in antes.iter().zip(&intentos) {
            if let Some(d) = self.direcciones.get_mut(&ficha.id) {
                d.cometido = match intento {
                    Intento::Atacar(_) => Cometido::AfrontarOposicion,
                    Intento::Liberar | Intento::TomarObjeto => Cometido::ActuarSobreObjetivo,
                    Intento::Recoger(_) => Cometido::Auxiliar,
                    Intento::Salir => Cometido::Evacuar,
                    Intento::Esperar => Cometido::Esperar,
                    _ => d.cometido,
                };
            }
        }
        let mut golpes: BTreeMap<(usize, usize), bool> = BTreeMap::new();
        for (i, intento) in intentos.iter().enumerate() {
            if let Intento::Atacar(j) = intento {
                golpes.insert((i, *j), false);
            }
        }
        // Una reacción por defensor y pulso. Toda salida amenazada cuenta, incluso dentro del alcance.
        for &i in &self.prioridad {
            if !antes[i].puede_actuar() {
                continue;
            }
            if let Some(j) = self.prioridad.iter().copied().find(|&j| {
                antes[i].aliado != antes[j].aliado
                    && matches!(intentos[j], Intento::Mover(_) | Intento::Salir)
                    && self.contacto(antes[i].posicion, antes[j].posicion)
            }) {
                golpes.insert((i, j), true);
            }
        }
        let mut danos = vec![0u8; antes.len()];
        let mut detenidos = BTreeSet::new();
        for ((i, j), reaccion) in golpes {
            let dado = (sortear(azar, 20)? + 1) as i16;
            let acierto = antes[j].golpes == 2
                || dado == 20
                || (dado != 1
                    && dado + antes[i].ataque - if antes[i].golpes > 0 { 2 } else { 0 }
                        >= 10 + antes[j].defensa);
            self.sucesos.push(Suceso::Golpe {
                atacante: antes[i].id,
                blanco: antes[j].id,
                acierto,
                reaccion,
            });
            if acierto {
                danos[j] = danos[j].saturating_add(1);
                if reaccion {
                    detenidos.insert(j);
                }
            }
        }
        // Se aplican todos los golpes calculados desde la instantánea: muerte mutua posible.
        for (i, dano) in danos.into_iter().enumerate() {
            self.fichas[i].golpes = self.fichas[i].golpes.saturating_add(dano).min(3);
        }
        for &i in &self.prioridad {
            if !self.fichas[i].puede_actuar() {
                continue;
            }
            let abrir = match intentos[i] {
                Intento::Abrir(id)
                    if self.puertas_inicio.get(&id) == Some(&EstadoPuerta::Cerrada)
                        && self.elementos.iter().any(|e| {
                            e.id == id && self.contacto(antes[i].posicion, e.posicion)
                        }) =>
                {
                    Some(id)
                }
                _ => None,
            };
            if let Some(id) = abrir
                && let Some(elemento) = self.elementos.iter_mut().find(|e| e.id == id)
                && let ClaseElemento::PuertaManual { estado, .. } = &mut elemento.clase
                && *estado == EstadoPuerta::Cerrada
            {
                *estado = EstadoPuerta::Abierta;
                self.contexto.revision_geometria += 1;
                self.sucesos.push(Suceso::PuertaCambiada {
                    elemento: id,
                    por: antes[i].id,
                    estado: EstadoPuerta::Abierta,
                });
            }
            if matches!(intentos[i], Intento::TomarObjeto)
                && let Some(b) = &mut self.botin
                && b.portador.is_none()
                && !b.fuera
            {
                b.portador = Some(self.fichas[i].id);
                self.sucesos
                    .push(Suceso::ObjetoRecogido { por: antes[i].id });
            }
            if matches!(intentos[i], Intento::Liberar)
                && self.rescate
                && self.fichas[self.indice_objetivo().unwrap()].cautivo
            {
                let objetivo = self.indice_objetivo().unwrap();
                self.fichas[objetivo].cautivo = false;
                let liberado = self.fichas[objetivo].id;
                for plan in &mut self.planes {
                    if matches!(
                        plan.proposito,
                        PropositoPlan::Rescatar | PropositoPlan::Retirarse
                    ) && !matches!(plan.estado, EstadoPlan::Cancelado | EstadoPlan::Cumplido)
                        && !plan.participantes.contains(&liberado)
                    {
                        plan.participantes.push(liberado);
                    }
                }
                self.sucesos.push(Suceso::Liberado {
                    ficha: self.objetivo.unwrap(),
                    por: antes[i].id,
                });
            }
            if let Intento::Recoger(j) = intentos[i]
                && self.fichas[j].golpes == 2
                && self.fichas[j].transportado_por.is_none()
            {
                self.fichas[j].transportado_por = Some(self.fichas[i].id);
                self.sucesos.push(Suceso::Recogido {
                    ficha: antes[j].id,
                    por: antes[i].id,
                });
            }
        }
        let mut destinos = BTreeMap::new();
        let mut reservados = BTreeSet::new();
        for &i in &self.prioridad {
            if self.fichas[i].puede_actuar() && !detenidos.contains(&i) {
                if let Intento::Mover(p) = intentos[i]
                    && reservados.insert(p)
                {
                    destinos.insert(i, p);
                }
                if matches!(intentos[i], Intento::Salir) {
                    let id = self.fichas[i].id;
                    for f in &mut self.fichas {
                        if f.transportado_por == Some(id) {
                            f.salio = true;
                            self.evacuados.insert(f.id, id);
                            f.transportado_por = None;
                        }
                    }
                    let quedan = self.fichas.iter().any(|f| {
                        f.aliado
                            && !f.cautivo
                            && f.golpes == 2
                            && !f.salio
                            && f.transportado_por.is_none()
                            && !self.abandonados_confirmados.contains(&f.id)
                    });
                    if !quedan {
                        self.fichas[i].salio = true;
                    }
                }
            }
        }
        fn disponible(
            i: usize,
            d: &BTreeMap<usize, Posicion>,
            f: &[Ficha],
            visitados: &mut BTreeSet<usize>,
        ) -> bool {
            if !visitados.insert(i) {
                return false;
            }
            let Some(p) = d.get(&i) else {
                return false;
            };
            if let Some(j) = f
                .iter()
                .enumerate()
                .find_map(|(j, f)| (j != i && f.puede_actuar() && f.posicion == *p).then_some(j))
            {
                disponible(j, d, f, visitados)
            } else {
                true
            }
        }
        let permitidos = destinos
            .keys()
            .copied()
            .filter(|&i| disponible(i, &destinos, &self.fichas, &mut BTreeSet::new()))
            .collect::<Vec<_>>();
        for i in permitidos {
            let p = destinos[&i];
            let desde = self.fichas[i].posicion;
            self.fichas[i].posicion = p;
            if antes[i].aliado
                && let (Some(origen), Some(destino)) = (self.espacio_en(desde), self.espacio_en(p))
                && origen != destino
                && self.conexiones_desde(origen).any(|c| {
                    !c.salida
                        && (c.desde == destino || c.hasta == destino)
                        && self.conexion_abierta(c)
                })
                && self.espacios_descubiertos.insert(destino)
            {
                self.contexto.revision_conocimiento += 1;
                self.descubierta = true;
            }
            self.sucesos.push(Suceso::Paso {
                ficha: antes[i].id,
                desde,
                hasta: p,
            });
        }
        self.cerrar_puertas_del_pulso(&antes, &intentos);
        for i in 0..self.fichas.len() {
            if let Some(id) = self.fichas[i].transportado_por {
                let j = self
                    .fichas
                    .iter()
                    .position(|f| f.id == id)
                    .ok_or("transportista inexistente")?;
                if self.fichas[j].puede_actuar() || self.fichas[j].salio {
                    self.fichas[i].posicion = self.fichas[j].posicion;
                    self.fichas[i].salio = self.fichas[j].salio;
                } else {
                    self.fichas[i].transportado_por = None;
                }
            }
        }
        if let Some(b) = &mut self.botin
            && let Some(i) = b.portador
        {
            let i = self
                .fichas
                .iter()
                .position(|f| f.id == i)
                .ok_or("portador inexistente")?;
            b.posicion = self.fichas[i].posicion;
            if self.fichas[i].salio {
                b.fuera = true;
            } else if !self.fichas[i].puede_actuar() {
                b.portador = None;
            }
        }
        self.recordar_terreno();
        self.evaluar_conclusion()?;
        if self
            .sucesos
            .iter()
            .any(|s| !matches!(s, Suceso::Golpe { acierto: false, .. }))
        {
            self.sin_progreso = 0;
        } else {
            self.sin_progreso = self.sin_progreso.saturating_add(1);
        }
        self.bloqueado = self.sin_progreso >= 8;
        if self.bloqueado
            && self.desenlace.is_none()
            && (self.retirando || self.extraccion_fuera() || self.objetivo_muerto())
        {
            self.decision_evacuacion = self.pendientes_evacuacion();
        }
        if self.bloqueado {
            for plan in self
                .planes
                .iter_mut()
                .filter(|p| p.estado == EstadoPlan::Activo)
            {
                plan.estado = EstadoPlan::Bloqueado;
                plan.bloqueo = Some(MotivoBloqueo::SinProgreso);
            }
        }
        if let Some(resultado) = self.desenlace {
            for plan in &mut self.planes {
                if plan.estado == EstadoPlan::Cancelado {
                    continue;
                }
                let cumplido = match plan.proposito {
                    PropositoPlan::Retirarse => {
                        matches!(resultado, Desenlace::Retirada | Desenlace::Exito)
                    }
                    _ => resultado == Desenlace::Exito,
                };
                plan.estado = if cumplido {
                    EstadoPlan::Cumplido
                } else {
                    EstadoPlan::Cancelado
                };
                plan.bloqueo = None;
            }
        }
        Ok(())
    }
    pub fn reconsiderar(&mut self, retirada: bool) {
        self.retirando = retirada;
        if retirada
            && !self
                .planes
                .iter()
                .any(|p| p.proposito == PropositoPlan::Retirarse)
        {
            self.planes.insert(
                0,
                PlanLocal {
                    id: IdPlan(1),
                    proposito: PropositoPlan::Retirarse,
                    prioridad: 20,
                    participantes: self
                        .fichas
                        .iter()
                        .enumerate()
                        .filter(|(_, f)| f.aliado)
                        .map(|(_, f)| f.id)
                        .collect(),
                    estado: EstadoPlan::Activo,
                    bloqueo: None,
                },
            );
        }
        for plan in &mut self.planes {
            if plan
                .participantes
                .iter()
                .all(|id| self.fichas.iter().any(|f| f.id == *id && !f.aliado))
            {
                continue;
            }
            if matches!(plan.estado, EstadoPlan::Cancelado | EstadoPlan::Cumplido) {
                continue;
            }
            plan.estado = if (plan.proposito == PropositoPlan::Retirarse) == retirada {
                EstadoPlan::Activo
            } else {
                EstadoPlan::Suspendido
            };
        }
        self.bloqueado = false;
        self.sin_progreso = 0;
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{
        ActorInicial, Caracteristica, DatosInicialesMundo, Fecha, IdLugar, IdMapa, LugarInicial,
        Mes, UbicacionActor,
    };
    use std::collections::VecDeque;
    struct Dado(usize);
    impl FuenteAzar for Dado {
        fn elegir_indice(
            &mut self,
            n: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            Ok(self.0.min(n.get() - 1))
        }
    }
    fn actor(i: usize) -> IdActor {
        IdActor::desde_posicion(i).unwrap()
    }
    fn mundo(n: usize, grupo: usize) -> Mundo {
        Mundo::nuevo(datos(n, grupo)).unwrap()
    }
    fn datos(n: usize, grupo: usize) -> DatosInicialesMundo {
        let lugar = IdLugar::desde_posicion(0).unwrap();
        DatosInicialesMundo {
            colectivos: vec![],
            mapa: IdMapa::desde_posicion(0).unwrap(),
            fecha: Fecha::nueva(2026, Mes::Enero, 1).unwrap(),
            lugares: vec![LugarInicial {
                id: lugar,
                caracteristicas: vec![Caracteristica::nueva("civilizado".into()).unwrap()],
            }],
            actores: (0..n.max(3))
                .map(|i| ActorInicial {
                    id: actor(i),
                    ubicacion: UbicacionActor::EnLugar(lugar),
                    caracteristicas: vec![],
                })
                .collect(),
            actor_jugador: actor(0),
            companeros: (1..grupo.max(2)).map(actor).collect(),
            variables_globales: vec![],
            relaciones: vec![],
        }
    }
    #[test]
    fn errores_locales_conservan_ids_y_rechazan_antes_de_mutar() {
        let mundo = mundo(3, 2);
        let mut e = Encuentro::nuevo(&mundo, &[actor(0)], actor(2), &[], &mut Dado(0)).unwrap();
        let posicion = Posicion {
            x: e.ancho,
            y: 0,
            z: 1,
        };
        let terreno = e.celdas.clone();
        assert_eq!(
            e.aplicar_comandos(
                &[ComandoEncuentro::Voxel {
                    posicion,
                    terreno: Celda::Muro
                }],
                &mundo
            ),
            Err(ErrorEncuentro::PosicionFueraDeLimites(posicion))
        );
        assert_eq!(e.celdas, terreno);
        assert_eq!(e.pulso, 0);
        let ficha = IdFicha(99);
        let elemento = IdElementoLocal(99);
        assert_eq!(
            e.solicitar_puerta(ficha, elemento, EstadoPuerta::Abierta),
            Err(ErrorEncuentro::ReferenciaLocal { ficha, elemento })
        );
        e.decision_evacuacion.push(e.fichas[0].id);
        assert!(matches!(
            e.avanzar(&mut Dado(0)),
            Err(ErrorEncuentro::EstadoIncompatible { .. })
        ));
        assert_eq!(e.pulso, 0);
    }
    #[test]
    fn consecuencias_locales_decision_abandono_y_contabilidad_completa() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(3, 2),
            &[actor(0), actor(1)],
            actor(2),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[1].golpes = 2;
        e.fichas[2].golpes = 3;
        e.reconsiderar(true);
        e.planes.clear(); // Un plan sin progreso debe devolver el control, no girar eternamente.
        for _ in 0..8 {
            e.avanzar(&mut Dado(0)).unwrap();
        }
        assert_eq!(e.decision_evacuacion, vec![e.fichas[1].id]);
        let pulso = e.pulso;
        assert!(e.avanzar(&mut Dado(0)).is_err());
        e.responder_evacuacion(false).unwrap();
        assert_eq!(e.pulso, pulso);
        assert!(e.abandonados_confirmados.is_empty());
        e.planes.clear();
        for _ in 0..8 {
            e.avanzar(&mut Dado(0)).unwrap();
        }
        e.responder_evacuacion(true).unwrap();
        for _ in 0..100 {
            if e.resultado.is_some() {
                break;
            }
            e.avanzar(&mut Dado(0)).unwrap();
        }
        let r = e.resultado.as_ref().unwrap();
        assert_eq!(r.causa, CausaFin::SalidaResuelta);
        assert_eq!(r.fichas[&e.fichas[0].id], ResultadoFicha::Salio);
        assert_eq!(
            r.fichas[&e.fichas[1].id],
            ResultadoFicha::Permanece(e.lugar)
        );
        assert_eq!(r.fichas[&e.fichas[2].id], ResultadoFicha::Muerto);
        assert!(!e.fichas[1].salio);
        e.validar_conclusion(r).unwrap();
        let mut incompleto = r.clone();
        incompleto.fichas.remove(&e.fichas[1].id);
        assert!(e.validar_conclusion(&incompleto).is_err());
        let mut causa_falsa = r.clone();
        causa_falsa.causa = CausaFin::Derrota;
        assert!(e.validar_conclusion(&causa_falsa).is_err());
    }
    #[test]
    fn consecuencias_locales_captura_parcial_no_atraviesa_espacios() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(3, 2),
            &[actor(0), actor(1)],
            actor(2),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].golpes = 2;
        e.fichas[1].golpes = 2;
        let enemigo = e.fichas[2].posicion;
        e.fichas[1].posicion = Posicion {
            x: enemigo.x - 1,
            ..enemigo
        };
        e.contexto.conexiones.clear();
        // Aislar físicamente al primer herido; quitar pistas no levanta paredes.
        let aislado = e.fichas[0].posicion;
        for y in aislado.y - 1..=aislado.y + 1 {
            for x in aislado.x - 1..=aislado.x + 1 {
                if (x, y) != (aislado.x, aislado.y) {
                    for z in aislado.z..=aislado.z + 1 {
                        let indice = usize::from((z * e.alto + y) * e.ancho + x);
                        e.celdas[indice] = Celda::Muro;
                    }
                }
            }
        }
        e.planes.clear();
        e.avanzar(&mut Dado(0)).unwrap();
        let r = e.resultado.as_ref().unwrap();
        assert_eq!(r.causa, CausaFin::Captura);
        assert_eq!(
            r.fichas[&e.fichas[0].id],
            ResultadoFicha::Permanece(e.lugar)
        );
        assert_eq!(
            r.fichas[&e.fichas[1].id],
            ResultadoFicha::Capturado(e.fichas[2].id)
        );
        e.validar_conclusion(r).unwrap();
    }
    #[test]
    fn robo_palacio_companeros_cruzan_umbral() {
        use crate::{ObjetoInicial, RelacionInicial, TipoRelacion};
        let mut d = datos(4, 3);
        let objeto = IdObjeto::desde_posicion(0).unwrap();
        d.relaciones.push(RelacionInicial {
            mercancia: None,
            tipo: TipoRelacion::GuardadoEn,
            origen: ExtremoRelacion::Objeto(objeto),
            destino: ExtremoRelacion::Lugar(IdLugar::desde_posicion(0).unwrap()),
        });
        let m = Mundo::nuevo_con_elementos(
            d,
            0,
            vec![ObjetoInicial {
                id: objeto,
                caracteristicas: vec![],
            }],
        )
        .unwrap();
        let mut e = Encuentro::nuevo_recuperacion(
            &m,
            &[actor(0), actor(1), actor(2)],
            objeto,
            &[actor(3)],
            &mut Dado(19),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas.iter_mut().find(|f| !f.aliado).unwrap().ataque = -1000;
        let mut cruzaron = BTreeSet::new();
        for _ in 0..300 {
            if e.desenlace.is_some() {
                break;
            }
            e.avanzar(&mut Dado(10)).unwrap();
            for f in e.fichas.iter().filter(|f| f.aliado && f.posicion.x > 11) {
                cruzaron.insert(f.id);
            }
            if e.bloqueado {
                panic!(
                    "bloqueo: {:?}; intentos {:?}; conocidos {:?}",
                    e.fichas,
                    (0..e.fichas.len()).map(|i| e.elegir(i)).collect::<Vec<_>>(),
                    e.espacios_descubiertos
                );
            }
        }
        assert_eq!(cruzaron.len(), 3, "{:?}", e.fichas);
        assert_eq!(e.desenlace, Some(Desenlace::Exito));
    }
    #[test]
    fn objetos_encuentro_extrae_y_recoge_de_nuevo_si_cae_el_portador() {
        use crate::{ObjetoInicial, RelacionInicial, TipoRelacion};
        let mut d = datos(3, 2);
        let objeto = IdObjeto::desde_posicion(0).unwrap();
        d.relaciones.push(RelacionInicial {
            mercancia: None,
            tipo: TipoRelacion::GuardadoEn,
            origen: ExtremoRelacion::Objeto(objeto),
            destino: ExtremoRelacion::Lugar(IdLugar::desde_posicion(0).unwrap()),
        });
        let m = Mundo::nuevo_con_elementos(
            d,
            0,
            vec![ObjetoInicial {
                id: objeto,
                caracteristicas: vec![],
            }],
        )
        .unwrap();
        for caer in [false, true] {
            let mut e = Encuentro::nuevo_recuperacion(
                &m,
                &[actor(0), actor(1)],
                objeto,
                &[],
                &mut Dado(19),
                Escenario::Campamento,
            )
            .unwrap();
            assert_eq!(e.fichas.len(), 2);
            assert!(e.objetivo.is_none());
            let mut cayo = false;
            let mut recogidas = 0;
            for _ in 0..500 {
                if e.desenlace.is_some() {
                    break;
                }
                if caer
                    && !cayo
                    && let Some(i) = e.botin.as_ref().unwrap().portador
                {
                    e.fichas.iter_mut().find(|f| f.id == i).unwrap().golpes = 2;
                    cayo = true;
                }
                e.avanzar(&mut Dado(19)).unwrap();
                recogidas += e
                    .sucesos
                    .iter()
                    .filter(|s| matches!(s, Suceso::ObjetoRecogido { .. }))
                    .count();
                assert!(!e.bloqueado);
            }
            assert_eq!(e.desenlace, Some(Desenlace::Exito));
            assert!(e.botin.as_ref().unwrap().fuera);
            assert_eq!(recogidas, if caer { 2 } else { 1 });
        }
    }
    #[test]
    fn rescate_relaciones_conserva_ids_y_no_duplica_al_recapturar() {
        use crate::{ExtremoRelacion, LoteOperaciones, OperacionMundo as Op, TipoRelacion};
        let mut m = mundo(3, 2);
        let confirmar = Op::ConfirmarRescate {
            actor: actor(1),
            captor: actor(2),
            rescatador: actor(0),
        };
        for _ in 0..2 {
            m.aplicar_lote(LoteOperaciones::uno(Op::RegistrarCautiverio {
                actor: actor(1),
                captor: actor(2),
            }))
            .unwrap();
            let id = m
                .relaciones()
                .iter()
                .find(|r| r.tipo() == TipoRelacion::PrisioneroDe)
                .unwrap()
                .id();
            m.aplicar_lote(LoteOperaciones::uno(confirmar.clone()))
                .unwrap();
            assert_eq!(m.relacion(id).unwrap().tipo(), TipoRelacion::LiberadoDe);
            m.aplicar_lote(LoteOperaciones::uno(confirmar.clone()))
                .unwrap();
            assert_eq!(m.relaciones().len(), 3);
            let ids = m.relaciones_de(ExtremoRelacion::Actor(actor(0))).unwrap();
            assert_eq!(ids.len(), 1);
            assert_eq!(m.relacion(ids[0]).unwrap().tipo(), TipoRelacion::SalvadoPor);
        }
    }
    #[test]
    fn rescate_libera_y_extrae_incluso_incapacitado() {
        let m = mundo(3, 2);
        for golpes in [0, 2] {
            let mut e = Encuentro::nuevo_rescate(
                &m,
                &[actor(0), actor(1)],
                actor(2),
                &[],
                &mut Dado(19),
                Escenario::Campamento,
            )
            .unwrap();
            let objetivo = e.indice_objetivo().unwrap();
            e.fichas[objetivo].golpes = golpes;
            let mut liberaciones = 0;
            for _ in 0..300 {
                if e.desenlace.is_some() {
                    break;
                }
                e.avanzar(&mut Dado(19)).unwrap();
                liberaciones += e
                    .sucesos
                    .iter()
                    .filter(|s| matches!(s, Suceso::Liberado { .. }))
                    .count();
                assert!(!e.bloqueado);
            }
            assert_eq!(e.desenlace, Some(Desenlace::Exito));
            assert_eq!(liberaciones, 1);
            assert!(e.rescatado_fuera());
            assert_eq!(e.fichas[e.indice_objetivo().unwrap()].golpes, golpes);
        }
    }
    #[test]
    fn rescate_retirada_sin_liberar_no_es_exito() {
        let m = mundo(3, 2);
        let mut e = Encuentro::nuevo_rescate(
            &m,
            &[actor(0), actor(1)],
            actor(2),
            &[],
            &mut Dado(19),
            Escenario::Campamento,
        )
        .unwrap();
        e.reconsiderar(true);
        for _ in 0..40 {
            if e.desenlace.is_some() {
                break;
            }
            e.avanzar(&mut Dado(19)).unwrap();
        }
        assert_eq!(e.desenlace, Some(Desenlace::Retirada));
        assert!(e.fichas[e.indice_objetivo().unwrap()].cautivo);
        assert!(!e.rescatado_fuera());
    }
    #[test]
    fn escenarios_simples_conectados_distintos_y_jugables() {
        let m = mundo(7, 3);
        let mut formas = Vec::new();
        for escenario in [
            Escenario::Palacio,
            Escenario::Ciudad,
            Escenario::Bosque,
            Escenario::Montana,
            Escenario::Playa,
        ] {
            let mut e = Encuentro::nuevo_con_escenario(
                &m,
                &[actor(0), actor(1), actor(2)],
                actor(3),
                &[actor(4), actor(5), actor(6)],
                &mut Dado(19),
                escenario,
            )
            .unwrap();
            let otra = Encuentro::nuevo_con_escenario(
                &m,
                &[actor(0), actor(1), actor(2)],
                actor(3),
                &[actor(4), actor(5), actor(6)],
                &mut Dado(19),
                escenario,
            )
            .unwrap();
            assert_eq!(e, otra);
            assert!(
                !formas.contains(&e.celdas),
                "geometría repetida: {escenario:?}"
            );
            formas.push(e.celdas.clone());
            let mut vistos = BTreeSet::from([e.entrada]);
            let mut cola = VecDeque::from([e.entrada]);
            while let Some(p) = cola.pop_front() {
                for vecino in e.vecinos(p) {
                    if vistos.insert(vecino) {
                        cola.push_back(vecino);
                    }
                }
            }
            assert!(
                e.fichas.iter().all(|f| vistos.contains(&f.posicion)),
                "inicio aislado: {escenario:?}"
            );
            for _ in 0..400 {
                if e.desenlace.is_some() {
                    break;
                }
                e.avanzar(&mut Dado(19)).unwrap();
                assert!(!e.bloqueado, "atasco: {escenario:?}");
            }
            assert!(e.desenlace.is_some(), "no termina: {escenario:?}");
        }
    }
    #[test]
    fn ciclo3_campamento_conectado_reproducible_con_dos_pasos() {
        let m = mundo(7, 3);
        for dado in [0, 19] {
            let e = Encuentro::nuevo_con_escenario(
                &m,
                &[actor(0), actor(1), actor(2)],
                actor(3),
                &[actor(4), actor(5), actor(6)],
                &mut Dado(dado),
                Escenario::Campamento,
            )
            .unwrap();
            let otra = Encuentro::nuevo_con_escenario(
                &m,
                &[actor(0), actor(1), actor(2)],
                actor(3),
                &[actor(4), actor(5), actor(6)],
                &mut Dado(dado),
                Escenario::Campamento,
            )
            .unwrap();
            assert_eq!(e, otra);
            assert!(e.descubierta);
            assert!(!e.contacto_hostil());
            assert!(!e.referencias.contains_key("brecha_norte"));
            let mut vistos = BTreeSet::from([e.entrada]);
            let mut cola = VecDeque::from([e.entrada]);
            while let Some(p) = cola.pop_front() {
                for vecino in e.vecinos(p) {
                    if vistos.insert(vecino) {
                        cola.push_back(vecino);
                    }
                }
            }
            assert!(e.fichas.iter().all(|f| vistos.contains(&f.posicion)));
            for nombre in ["paso_norte", "paso_sur"] {
                let paso = e.referencias[nombre];
                assert!(vistos.contains(&paso));
                assert_eq!(e.terreno(paso), Some(Celda::Aire));
            }
            assert_eq!(
                (1..16)
                    .filter(|&y| e.suelo(Posicion { z: 1, x: 20, y }))
                    .count(),
                4
            );
            let mut retirada = e.clone();
            retirada.reconsiderar(true);
            for _ in 0..50 {
                if retirada.desenlace.is_some() {
                    break;
                }
                retirada.avanzar(&mut Dado(19)).unwrap();
            }
            assert_eq!(retirada.desenlace, Some(Desenlace::Retirada));
        }
    }
    #[test]
    fn ciclo2_comandos_locales_validan_y_no_confirman_prefijos_invalidos() {
        let m = mundo(3, 2);
        let mut e =
            Encuentro::nuevo(&m, &[actor(0), actor(1)], actor(2), &[], &mut Dado(19)).unwrap();
        let inicial = e.clone();
        let abrir = ComandoEncuentro::Despejar {
            posicion: Posicion { z: 1, x: 15, y: 7 },
            alto: 2,
        };
        let invalido = ComandoEncuentro::Reubicar {
            actor: actor(0),
            posicion: e.fichas[1].posicion,
        };
        assert!(e.aplicar_comandos(&[abrir.clone(), invalido], &m).is_err());
        assert_eq!(e, inicial);
        let destino = Posicion { z: 1, x: 5, y: 8 };
        e.aplicar_comandos(
            &[
                abrir,
                ComandoEncuentro::Reubicar {
                    actor: actor(0),
                    posicion: destino,
                },
            ],
            &m,
        )
        .unwrap();
        assert_eq!(e.fichas[0].posicion, destino);
        assert_eq!(e.terreno(Posicion { z: 1, x: 15, y: 7 }), Some(Celda::Aire));
        let inicial = e.clone();
        assert!(
            e.aplicar_comandos(
                &[ComandoEncuentro::Voxel {
                    posicion: e.entrada,
                    terreno: Celda::Muro
                }],
                &m
            )
            .is_err()
        );
        assert_eq!(e, inicial);
    }
    #[test]
    fn companeros_rodean_al_aliado_que_ya_combate() {
        let m = mundo(4, 3);
        let mut e = Encuentro::nuevo(
            &m,
            &[actor(0), actor(1), actor(2)],
            actor(3),
            &[],
            &mut Dado(19),
        )
        .unwrap();
        for (i, x) in [20, 19, 18, 21].into_iter().enumerate() {
            e.fichas[i].posicion = Posicion { z: 1, x, y: 8 };
        }
        e.descubierta = true;
        // Todos los golpes fallan: comprobamos posicionamiento, sin que una baja abra espacio.
        e.espacios_descubiertos.insert(IdEspacio(1));
        for _ in 0..12 {
            e.avanzar(&mut Dado(0)).unwrap();
        }
        assert!(
            e.fichas[..3]
                .iter()
                .all(|f| e.contacto(f.posicion, e.fichas[3].posicion))
        );
        let posiciones = e.fichas.iter().map(|f| f.posicion).collect::<BTreeSet<_>>();
        assert_eq!(posiciones.len(), 4);
    }
    #[test]
    fn ataques_simultaneos_no_anulan_la_intencion_del_rival() {
        let m = mundo(2, 1);
        let mut e = Encuentro::nuevo(&m, &[actor(0)], actor(1), &[], &mut Dado(19)).unwrap();
        e.fichas[0].posicion = Posicion { z: 1, x: 20, y: 8 };
        e.fichas[1].posicion = Posicion { z: 1, x: 21, y: 8 };
        e.fichas[0].golpes = 1;
        e.fichas[1].golpes = 1;
        e.descubierta = true;
        e.espacios_descubiertos.insert(IdEspacio(1));
        e.avanzar(&mut Dado(19)).unwrap();
        assert_eq!((e.fichas[0].golpes, e.fichas[1].golpes), (2, 2));
        assert_eq!(
            e.sucesos
                .iter()
                .filter(|s| matches!(s, Suceso::Golpe { acierto: true, .. }))
                .count(),
            2
        );
    }
    #[test]
    fn reaccion_al_rodear_acierta_detiene_y_fallo_permite_paso() {
        for (dado, se_mueve) in [(19, false), (0, true)] {
            let m = mundo(2, 1);
            let mut e = Encuentro::nuevo(&m, &[actor(0)], actor(1), &[], &mut Dado(19)).unwrap();
            e.fichas[0].posicion = Posicion { z: 1, x: 20, y: 8 };
            e.fichas[1].posicion = Posicion { z: 1, x: 21, y: 8 };
            e.entrada = Posicion { z: 1, x: 21, y: 7 };
            e.reconsiderar(true);
            e.descubierta = true;
            e.espacios_descubiertos.insert(IdEspacio(1));
            e.recordar_terreno();
            assert_eq!(e.elegir(0), Intento::Mover(e.entrada));
            e.avanzar(&mut Dado(dado)).unwrap();
            assert_eq!(e.fichas[0].posicion == e.entrada, se_mueve);
            assert_eq!(
                e.sucesos
                    .iter()
                    .filter(|s| matches!(
                        s,
                        Suceso::Golpe {
                            atacante: IdFicha(1),
                            reaccion: true,
                            ..
                        }
                    ))
                    .count(),
                1
            );
        }
    }
    #[test]
    fn diagonales_sin_cortar_esquinas_y_movimiento_de_ambos_bandos() {
        let m = mundo(10, 5);
        let grupo = (0..5).map(actor).collect::<Vec<_>>();
        let defensa = (6..10).map(actor).collect::<Vec<_>>();
        let mut e = Encuentro::nuevo(&m, &grupo, actor(5), &defensa, &mut Dado(0)).unwrap();
        e.descubierta = true;
        for (i, f) in e.fichas.iter_mut().enumerate() {
            f.posicion = Posicion {
                z: 1,
                x: if i < 5 { 4 } else { 25 },
                y: 2 + ((i % 5) * 3) as u16,
            };
        }
        assert!(
            e.vecinos(Posicion { z: 1, x: 4, y: 8 })
                .contains(&Posicion { z: 1, x: 5, y: 9 })
        );
        assert!(!e.contacto(
            Posicion { z: 1, x: 14, y: 7 },
            Posicion { z: 1, x: 15, y: 8 }
        ));
        let antes = e.fichas.clone();
        e.avanzar(&mut Dado(0)).unwrap();
        assert!(
            e.sucesos
                .iter()
                .any(|s| matches!(s,Suceso::Paso{ficha,..} if ficha.0<5))
        );
        assert!(
            e.sucesos
                .iter()
                .any(|s| matches!(s,Suceso::Paso{ficha,..} if ficha.0>=5))
        );
        for (a, b) in antes.iter().zip(&e.fichas) {
            assert!(distancia(a.posicion, b.posicion) <= 1);
        }
        let posiciones = e
            .fichas
            .iter()
            .filter(|f| f.puede_actuar())
            .map(|f| f.posicion)
            .collect::<BTreeSet<_>>();
        assert_eq!(posiciones.len(), 10);
    }
    #[test]
    fn generacion_reproducible_y_retirada_carga_a_los_incapacitados() {
        let m = mundo(4, 3);
        let grupo = [actor(0), actor(1), actor(2)];
        let mut e = Encuentro::nuevo(&m, &grupo, actor(3), &[], &mut Dado(0)).unwrap();
        assert_eq!(
            e,
            Encuentro::nuevo(&m, &grupo, actor(3), &[], &mut Dado(0)).unwrap()
        );
        e.fichas[1].golpes = 2;
        e.fichas[2].golpes = 2;
        e.reconsiderar(true);
        for _ in 0..150 {
            if e.desenlace.is_some() {
                break;
            }
            e.avanzar(&mut Dado(0)).unwrap();
            if e.bloqueado {
                e.reconsiderar(true);
            }
        }
        assert_eq!(e.desenlace, Some(Desenlace::Retirada));
        assert!(e.fichas.iter().filter(|f| f.aliado).all(|f| f.salio));
    }
    #[test]
    fn contexto_local_rechaza_puerta_compartida_por_conexiones_distintas() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        let mut falsa = e
            .contexto
            .conexiones
            .iter()
            .find(|c| c.control.is_some())
            .unwrap()
            .clone();
        falsa.id = IdConexion(200);
        e.contexto.conexiones.push(falsa);
        assert_eq!(
            e.validar_geometria(),
            Err(ErrorGeometria::ReferenciaInvalida)
        );
    }

    #[test]
    fn espacio_local_rechaza_botin_sin_apoyo() {
        let mut e =
            Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        e.botin = Some(Botin {
            objeto: IdObjeto::desde_posicion(0).unwrap(),
            ubicacion_inicial: ExtremoRelacion::Lugar(IdLugar::desde_posicion(0).unwrap()),
            posicion: Posicion { x: 2, y: 2, z: 7 },
            portador: None,
            fuera: false,
        });
        assert_eq!(
            e.validar_geometria(),
            Err(ErrorGeometria::ReferenciaInvalida)
        );
    }

    #[test]
    fn contexto_local_navegacion_no_publica_el_blanco_oculto() {
        let e = Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        let contexto = e.contexto_navegacion(e.fichas[0].id).unwrap();
        assert_eq!(contexto.voxel(e.fichas[1].posicion), None);
        assert!(!contexto.participantes().contains(&e.fichas[1].id));
        assert!(contexto.voxel(e.fichas[0].posicion).is_some());
    }

    #[test]
    fn espacio_local_presencia_probable_tiene_envolvente_estable() {
        let e = ElementoLocal {
            id: IdElementoLocal(99),
            clase: ClaseElemento::PresenciaProbable,
            posicion: Posicion { x: 2, y: 2, z: 1 },
            alto: 2,
            definido: false,
        };
        assert_eq!(e.apariencia(), AparienciaLocal::Indefinida);
        assert_eq!(e.envolvente().unwrap().max.z, 2);
        assert_eq!(
            Celda::Solido(MaterialLocal::Metal).material(),
            Some(MaterialLocal::Metal)
        );
    }

    #[test]
    fn espacio_local_rechaza_ciclo_de_transportistas() {
        let mut e = Encuentro::nuevo(
            &mundo(3, 2),
            &[actor(0), actor(1)],
            actor(2),
            &[],
            &mut Dado(0),
        )
        .unwrap();
        let primero = e.fichas[0].id;
        let segundo = e.fichas[1].id;
        e.fichas[0].transportado_por = Some(segundo);
        e.fichas[1].transportado_por = Some(primero);
        assert_eq!(
            e.validar_geometria(),
            Err(ErrorGeometria::ReferenciaInvalida)
        );
    }

    #[test]
    fn espacio_local_dos_alturas_y_cuerpos_de_altura_dos() {
        let e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::DosAlturas,
        )
        .unwrap();
        assert_eq!((e.ancho, e.alto, e.altura), (31, 17, 8));
        for z in 1..=2 {
            assert_eq!(e.terreno(Posicion { x: 18, y: 5, z }), Some(Celda::Muro));
        }
        for z in 4..=5 {
            assert_eq!(e.terreno(Posicion { x: 18, y: 5, z }), Some(Celda::Muro));
            assert_eq!(e.terreno(Posicion { x: 18, y: 8, z }), Some(Celda::Aire));
        }
        assert!(e.puede_ocupar(Posicion { x: 18, y: 8, z: 4 }, None).is_ok());
        let ruta = e
            .contexto
            .conexiones
            .iter()
            .find(|c| c.id == IdConexion(2))
            .unwrap();
        for paso in ruta.recorrido.windows(2) {
            assert_eq!(
                paso[0].x.abs_diff(paso[1].x) + paso[0].y.abs_diff(paso[1].y),
                1
            );
            assert!(paso[0].z.abs_diff(paso[1].z) <= 1);
            assert!(e.vecinos_caminables(paso[0]).contains(&paso[1]));
            assert!(e.vecinos_caminables(paso[1]).contains(&paso[0]));
        }
        assert!(
            e.vecinos_caminables(Posicion { x: 18, y: 8, z: 4 })
                .contains(&Posicion { x: 19, y: 8, z: 4 })
        );
        assert!(!e.contacto(
            Posicion { x: 18, y: 8, z: 1 },
            Posicion { x: 19, y: 8, z: 4 }
        ));
    }
    #[test]
    fn contexto_local_rechaza_escalera_sin_altura_de_transicion() {
        let m = mundo(2, 1);
        let mut e = Encuentro::nuevo_con_escenario(
            &m,
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::DosAlturas,
        )
        .unwrap();
        let anterior = e.clone();
        let techo = ComandoEncuentro::Voxel {
            posicion: Posicion { x: 15, y: 8, z: 3 },
            terreno: Celda::Muro,
        };
        assert!(e.aplicar_comandos(&[techo], &m).is_err());
        assert_eq!(e, anterior);
    }

    #[test]
    fn consecuencias_locales_captor_en_otra_altura_necesita_acceso() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(19),
            Escenario::DosAlturas,
        )
        .unwrap();
        e.fichas[0].golpes = 2;
        let mut aislado = e.clone();
        aislado
            .contexto
            .conexiones
            .retain(|c| c.recorrido.len() == 1);
        aislado.avanzar(&mut Dado(19)).unwrap();
        assert_eq!(aislado.desenlace, Some(Desenlace::Derrota));
        assert!(aislado.capturas.is_empty());
        e.avanzar(&mut Dado(19)).unwrap();
        assert_eq!(e.capturas.get(&e.fichas[0].id), Some(&e.fichas[1].id));
    }

    #[test]
    fn vista_local_memoria_de_puerta_no_revela_cierre_fuera_de_observacion() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Abierta, 0)
            .unwrap();
        e.recordar_terreno();
        e.fichas[0].posicion = Posicion { x: 25, y: 8, z: 1 };
        e.espacios_descubiertos.extend([IdEspacio(1), IdEspacio(2)]);
        e.fichas[1].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Cerrada, 1)
            .unwrap();
        e.recordar_terreno();
        assert!(!e.punto_observable(e.elementos[0].posicion));
        assert!(matches!(
            e.elementos_recordados().next().unwrap().clase,
            ClaseElemento::PuertaManual {
                estado: EstadoPuerta::Abierta,
                ..
            }
        ));
    }

    #[test]
    fn contexto_local_puerta_oculta_cambios_del_espacio_recordado() {
        let m = mundo(2, 1);
        let mut e = Encuentro::nuevo_con_escenario(
            &m,
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        let p = Posicion { x: 17, y: 8, z: 1 };
        assert_eq!(e.terreno_recordado(p), None);
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.espacios_descubiertos.insert(IdEspacio(1));
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Abierta, 0)
            .unwrap();
        e.recordar_terreno();
        assert_eq!(e.terreno_recordado(p), Some(Celda::Aire));
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Cerrada, 0)
            .unwrap();
        e.aplicar_comandos(
            &[ComandoEncuentro::Voxel {
                posicion: p,
                terreno: Celda::Muro,
            }],
            &m,
        )
        .unwrap();
        assert_eq!(e.terreno(p), Some(Celda::Muro));
        assert_eq!(e.terreno_recordado(p), Some(Celda::Aire));
        assert!(!e.ficha_observable(&e.fichas[1]));
    }

    #[test]
    fn contexto_local_rechaza_control_de_puerta_ajeno() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        assert_eq!(e.espacio_en(e.entrada), Some(IdEspacio(0)));
        assert!(e.conexiones_desde(IdEspacio(0)).any(|c| c.salida));
        e.contexto.conexiones[0].control = Some(IdElementoLocal(99));
        assert_eq!(
            e.validar_geometria(),
            Err(ErrorGeometria::ReferenciaInvalida)
        );
    }

    #[test]
    fn espacio_local_obstaculo_en_la_cabeza_impide_el_paso() {
        let mut e =
            Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        let p = Posicion { x: 3, y: 8, z: 1 };
        let id = IdElementoLocal(42);
        e.elementos.push(ElementoLocal {
            id,
            clase: ClaseElemento::Obstaculo,
            posicion: Posicion { z: 2, ..p },
            alto: 1,
            definido: true,
        });
        assert_eq!(
            e.puede_ocupar(p, None),
            Err(ImpedimentoOcupacion::Obstaculo(id))
        );
        assert!(!e.vecinos_caminables(e.entrada).contains(&p));
    }

    #[test]
    fn contexto_local_invalida_revision_al_cambiar_puerta() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        let revision = e.contexto.revision_geometria;
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Abierta, 0)
            .unwrap();
        assert!(e.contexto.revision_geometria > revision);
        assert!(e.puede_ocupar(Posicion { x: 11, y: 8, z: 1 }, None).is_ok());
    }
    #[test]
    fn planes_locales_comportamiento_estable_y_reanudacion_sin_reinicio() {
        let mut e =
            Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        let actor = e.fichas[0].id;
        e.coordinar_planes();
        let original = e.direccion(actor).unwrap().id;
        e.coordinar_planes();
        assert_eq!(e.direccion(actor).unwrap().id, original);
        e.avanzar(&mut Dado(0)).unwrap();
        let posicion = e.ficha(actor).unwrap().posicion;
        e.reconsiderar(true);
        e.coordinar_planes();
        assert_ne!(e.direccion(actor).unwrap().id, original);
        e.reconsiderar(false);
        e.coordinar_planes();
        assert_eq!(e.direccion(actor).unwrap().plan, IdPlan(0));
        assert_eq!(e.ficha(actor).unwrap().posicion, posicion);
    }

    #[test]
    fn planes_locales_rutas_no_revelan_destino_oculto() {
        let e = Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        let rutas = e.mapa_rutas(0);
        assert!(!rutas.contains_key(&e.fichas[1].posicion));
        assert!(rutas.contains_key(&Posicion { x: 16, y: 8, z: 1 }));
    }

    #[test]
    fn planes_locales_auxilio_unico_y_sustitucion_por_incapacidad() {
        let mut e = Encuentro::nuevo(
            &mundo(5, 4),
            &[actor(0), actor(1), actor(2), actor(3)],
            actor(4),
            &[],
            &mut Dado(0),
        )
        .unwrap();
        e.fichas[3].golpes = 2;
        e.reconsiderar(true);
        e.coordinar_planes();
        let herido = e.fichas[3].id;
        let auxiliares: Vec<_> = e
            .direcciones
            .iter()
            .filter(|(_, d)| d.rol == RolEncuentro::Auxiliador(herido))
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(auxiliares.len(), 1);
        let anterior = auxiliares[0];
        e.fichas
            .iter_mut()
            .find(|f| f.id == anterior)
            .unwrap()
            .golpes = 2;
        e.coordinar_planes();
        assert!(e.direccion(anterior).is_none());
        assert_eq!(
            e.direcciones
                .values()
                .filter(|d| d.rol == RolEncuentro::Auxiliador(herido))
                .count(),
            1
        );
    }

    #[test]
    fn planes_locales_cancelado_no_se_reanuda_y_sin_participantes_bloquea() {
        let mut e =
            Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        e.planes[0].estado = EstadoPlan::Cancelado;
        e.reconsiderar(true);
        e.reconsiderar(false);
        assert_eq!(
            e.planes.iter().find(|p| p.id == IdPlan(0)).unwrap().estado,
            EstadoPlan::Cancelado
        );
        e.coordinar_planes();
        assert!(e.direccion(e.fichas[0].id).is_none());
        e.planes[0].estado = EstadoPlan::Activo;
        e.fichas[0].golpes = 2;
        e.coordinar_planes();
        assert_eq!(e.planes[0].bloqueo, Some(MotivoBloqueo::SinParticipantes));
    }

    #[test]
    fn planes_locales_prioridad_no_cambia_orden_fisico() {
        let mut e =
            Encuentro::nuevo(&mundo(2, 1), &[actor(0)], actor(1), &[], &mut Dado(0)).unwrap();
        let orden = e.prioridad.clone();
        e.fichas[0].posicion = e.entrada;
        e.reconsiderar(true);
        assert_eq!(e.planes[0].prioridad, 20);
        assert_eq!(e.prioridad, orden);
        assert_eq!(e.planes[1].estado, EstadoPlan::Suspendido);
        e.planes[1].estado = EstadoPlan::Activo;
        assert_eq!(e.elegir(0), Intento::Salir);
        e.planes[1].prioridad = 21;
        assert!(matches!(e.elegir(0), Intento::Mover(_)));
    }
    #[test]
    fn puerta_local_cierre_compite_con_cruce_sin_aplastar() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.fichas[1].posicion = Posicion { x: 12, y: 8, z: 1 };
        e.espacios_descubiertos.insert(IdEspacio(1));
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Abierta, 0)
            .unwrap();
        e.recordar_terreno();
        e.solicitar_puerta(e.fichas[1].id, IdElementoLocal(0), EstadoPuerta::Cerrada)
            .unwrap();
        e.avanzar(&mut Dado(0)).unwrap();
        assert_eq!(e.fichas[0].posicion.x, 11);
        assert!(matches!(
            e.elementos[0].clase,
            ClaseElemento::PuertaManual {
                estado: EstadoPuerta::Abierta,
                ..
            }
        ));
        assert!(!e.sucesos.iter().any(|s| matches!(
            s,
            Suceso::PuertaCambiada {
                estado: EstadoPuerta::Cerrada,
                ..
            }
        )));
    }

    #[test]
    fn puerta_local_abrir_y_cerrar_no_se_encadenan_en_un_pulso() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.fichas[1].posicion = Posicion { x: 12, y: 8, z: 1 };
        e.solicitar_puerta(e.fichas[0].id, IdElementoLocal(0), EstadoPuerta::Abierta)
            .unwrap();
        e.solicitar_puerta(e.fichas[1].id, IdElementoLocal(0), EstadoPuerta::Cerrada)
            .unwrap();
        e.avanzar(&mut Dado(0)).unwrap();
        assert!(matches!(
            e.elementos[0].clase,
            ClaseElemento::PuertaManual {
                estado: EstadoPuerta::Abierta,
                ..
            }
        ));
        assert_eq!(
            e.sucesos
                .iter()
                .filter(|s| matches!(s, Suceso::PuertaCambiada { .. }))
                .count(),
            1
        );
        e.solicitar_puerta(e.fichas[0].id, IdElementoLocal(0), EstadoPuerta::Cerrada)
            .unwrap();
        e.solicitar_puerta(e.fichas[1].id, IdElementoLocal(0), EstadoPuerta::Cerrada)
            .unwrap();
        e.avanzar(&mut Dado(0)).unwrap();
        assert_eq!(
            e.sucesos
                .iter()
                .filter(|s| matches!(
                    s,
                    Suceso::PuertaCambiada {
                        estado: EstadoPuerta::Cerrada,
                        ..
                    }
                ))
                .count(),
            1
        );
    }

    #[test]
    fn puerta_local_rechaza_cierre_ocupado_y_persiste_abierta() {
        let mut e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        e.fichas[0].posicion = Posicion { x: 10, y: 8, z: 1 };
        e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Abierta, 0)
            .unwrap();
        e.fichas[1].posicion = Posicion { x: 11, y: 8, z: 1 };
        assert!(
            e.cambiar_puerta(IdElementoLocal(0), EstadoPuerta::Cerrada, 0)
                .is_err()
        );
        assert!(matches!(
            e.elementos[0].clase,
            ClaseElemento::PuertaManual {
                estado: EstadoPuerta::Abierta,
                ..
            }
        ));
    }
    #[test]
    fn generacion_local_palacio_separa_los_tres_salones() {
        let e = Encuentro::nuevo_con_escenario(
            &mundo(2, 1),
            &[actor(0)],
            actor(1),
            &[],
            &mut Dado(0),
            Escenario::Palacio,
        )
        .unwrap();
        assert_eq!(
            e.espacio_en(Posicion { x: 2, y: 8, z: 1 }),
            Some(IdEspacio(0))
        );
        assert_eq!(
            e.espacio_en(Posicion { x: 17, y: 8, z: 1 }),
            Some(IdEspacio(1))
        );
        assert_eq!(
            e.espacio_en(Posicion { x: 27, y: 8, z: 1 }),
            Some(IdEspacio(2))
        );
        assert!(
            e.contexto
                .conexiones
                .iter()
                .any(|c| c.desde == IdEspacio(1) && c.hasta == IdEspacio(2))
        );
    }

    #[test]
    fn generacion_local_todos_los_perfiles_respetan_limites() {
        for escenario in [
            Escenario::Recinto,
            Escenario::Campamento,
            Escenario::Palacio,
            Escenario::Ciudad,
            Escenario::Bosque,
            Escenario::Montana,
            Escenario::Playa,
            Escenario::DosAlturas,
        ] {
            let e = Encuentro::nuevo_con_escenario(
                &mundo(2, 1),
                &[actor(0)],
                actor(1),
                &[],
                &mut Dado(0),
                escenario,
            )
            .unwrap();
            assert!(e.ancho <= MAX_ANCHO && e.alto <= MAX_PROFUNDIDAD && e.altura <= MAX_ALTURA);
            assert_eq!(e.celdas.len(), usize::from(e.ancho * e.alto * e.altura));
            assert!(e.fichas.iter().all(|f| e.suelo(f.posicion)));
        }
    }
}
