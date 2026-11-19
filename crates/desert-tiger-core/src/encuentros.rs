//! Encuentro local efímero. Cada pulso elige intenciones desde una sola instantánea.
use crate::azar::FuenteAzar;
use crate::{IdActor, Mundo};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Posicion {
    pub area: u16,
    pub nivel: i16,
    pub x: u16,
    pub y: u16,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Celda {
    Suelo,
    Muro,
    Vacio,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ficha {
    pub actor: IdActor,
    pub posicion: Posicion,
    pub aliado: bool,
    pub golpes: u8,
    pub ataque: i16,
    pub defensa: i16,
    pub salio: bool,
    pub transportado_por: Option<usize>,
}
impl Ficha {
    pub fn puede_actuar(&self) -> bool {
        self.golpes < 2 && !self.salio && self.transportado_por.is_none()
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
    Paso {
        ficha: usize,
        desde: Posicion,
        hasta: Posicion,
    },
    Golpe {
        atacante: usize,
        blanco: usize,
        acierto: bool,
        reaccion: bool,
    },
    Recogido {
        ficha: usize,
        por: usize,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Encuentro {
    pub ancho: u16,
    pub alto: u16,
    pub celdas: Vec<Celda>,
    pub fichas: Vec<Ficha>,
    pub objetivo: usize,
    pub entrada: Posicion,
    pub pulso: u64,
    pub retirando: bool,
    pub descubierta: bool,
    pub sucesos: Vec<Suceso>,
    pub desenlace: Option<Desenlace>,
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
    Salir,
}
fn sortear(azar: &mut dyn FuenteAzar, n: usize) -> Result<usize, String> {
    azar.elegir_indice(NonZeroUsize::new(n).ok_or("sorteo vacío")?)
        .map_err(|e| e.to_string())
}
fn distancia(a: Posicion, b: Posicion) -> u16 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}
impl Encuentro {
    pub fn nuevo(
        mundo: &Mundo,
        grupo: &[IdActor],
        objetivo: IdActor,
        defensores: &[IdActor],
        azar: &mut dyn FuenteAzar,
    ) -> Result<Self, String> {
        if grupo.is_empty() || grupo.len() + defensores.len() > 32 || grupo.contains(&objetivo) {
            return Err("participantes inválidos".into());
        }
        let ancho = 31;
        let alto = 17;
        let mut celdas = vec![Celda::Suelo; usize::from(ancho * alto)];
        for y in 0..alto {
            for x in 0..ancho {
                if x == 0 || y == 0 || x == ancho - 1 || y == alto - 1 || (x == 15 && y != 8) {
                    celdas[usize::from(y * ancho + x)] = Celda::Muro;
                }
            }
        }
        for y in 3..6 {
            for x in 7..10 {
                celdas[usize::from(y * ancho + x)] = Celda::Vacio;
            }
        }
        for _ in 0..12 {
            let x = 3 + sortear(azar, 25)? as u16;
            let y = 2 + sortear(azar, 13)? as u16;
            if (7..=10).contains(&y) || x == 15 {
                continue;
            }
            celdas[usize::from(y * ancho + x)] = Celda::Muro;
        }
        let mut fichas = Vec::new();
        let mut vistos = BTreeSet::new();
        for (i, actor) in grupo
            .iter()
            .chain(std::iter::once(&objetivo))
            .chain(defensores)
            .copied()
            .enumerate()
        {
            if !vistos.insert(actor) {
                return Err("Actor repetido en el encuentro".into());
            }
            let a = mundo.actor(actor).ok_or("Actor inexistente")?;
            if mundo.ubicacion_actor(actor) != Some(mundo.ubicacion_grupo()) {
                return Err("participante fuera del Lugar".into());
            }
            let tiene = |c: &str| a.caracteristicas().any(|v| v.como_str() == c);
            if tiene("muerto") {
                return Err("participante muerto".into());
            }
            let aliado = i < grupo.len();
            let posicion = if aliado {
                Posicion {
                    area: 0,
                    nivel: 0,
                    x: 2 + (i / 13) as u16,
                    y: 2 + (i % 13) as u16,
                }
            } else if i == grupo.len() {
                Posicion {
                    area: 0,
                    nivel: 0,
                    x: 27,
                    y: 8,
                }
            } else {
                Posicion {
                    area: 0,
                    nivel: 0,
                    x: 17 + (i - grupo.len() - 1) as u16,
                    y: 8,
                }
            };
            if posicion.x >= ancho - 1 {
                return Err("demasiados defensores".into());
            }
            celdas[usize::from(posicion.y * ancho + posicion.x)] = Celda::Suelo;
            fichas.push(Ficha {
                actor,
                posicion,
                aliado,
                golpes: if tiene("incapacitado") {
                    2
                } else if tiene("herido") {
                    1
                } else {
                    0
                },
                ataque: if tiene("fuerte") { 2 } else { 0 },
                defensa: if tiene("agil") { 2 } else { 0 },
                salio: false,
                transportado_por: None,
            });
        }
        let mut prioridad = (0..fichas.len()).collect::<Vec<_>>();
        for i in (1..prioridad.len()).rev() {
            let j = sortear(azar, i + 1)?;
            prioridad.swap(i, j);
        }
        Ok(Self {
            ancho,
            alto,
            celdas,
            fichas,
            objetivo: grupo.len(),
            entrada: Posicion {
                area: 0,
                nivel: 0,
                x: 2,
                y: 8,
            },
            pulso: 0,
            retirando: false,
            descubierta: false,
            sucesos: vec![],
            desenlace: None,
            prioridad,
            sin_progreso: 0,
            bloqueado: false,
        })
    }
    pub fn terreno(&self, p: Posicion) -> Option<Celda> {
        if p.area != 0 || p.nivel != 0 || p.x >= self.ancho || p.y >= self.alto {
            None
        } else {
            self.celdas
                .get(usize::from(p.y * self.ancho + p.x))
                .copied()
        }
    }
    fn suelo(&self, p: Posicion) -> bool {
        self.terreno(p) == Some(Celda::Suelo)
    }
    fn contacto(&self, a: Posicion, b: Posicion) -> bool {
        if a.area != b.area || a.nivel != b.nivel || distancia(a, b) != 1 {
            return false;
        }
        if a.x != b.x && a.y != b.y {
            self.suelo(Posicion {
                area: 0,
                nivel: 0,
                x: a.x,
                y: b.y,
            }) && self.suelo(Posicion {
                area: 0,
                nivel: 0,
                x: b.x,
                y: a.y,
            })
        } else {
            true
        }
    }
    fn vecinos(&self, p: Posicion) -> Vec<Posicion> {
        let mut r = vec![];
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = i32::from(p.x) + dx;
                let y = i32::from(p.y) + dy;
                if x < 0 || y < 0 || x >= i32::from(self.ancho) || y >= i32::from(self.alto) {
                    continue;
                }
                let q = Posicion {
                    area: 0,
                    nivel: 0,
                    x: x as u16,
                    y: y as u16,
                };
                if self.suelo(q) && self.contacto(p, q) {
                    r.push(q);
                }
            }
        }
        r
    }
    fn siguiente(&self, desde: usize, meta: Posicion, adyacente: bool) -> Option<Posicion> {
        let p = self.fichas[desde].posicion;
        let mut padres = BTreeMap::from([(p, p)]);
        let mut cola = VecDeque::from([p]);
        while let Some(a) = cola.pop_front() {
            if a != p
                && !self
                    .fichas
                    .iter()
                    .any(|f| f.puede_actuar() && f.posicion == a)
                && (if adyacente {
                    self.contacto(a, meta)
                } else {
                    a == meta
                })
            {
                let mut q = a;
                while padres[&q] != p {
                    q = padres[&q];
                }
                return Some(q);
            }
            for b in self.vecinos(a) {
                if padres.contains_key(&b)
                    || self.fichas.iter().any(|f| {
                        f.puede_actuar()
                            && f.posicion == b
                            && (f.aliado != self.fichas[desde].aliado
                                || self.fichas.iter().any(|enemigo| {
                                    enemigo.puede_actuar()
                                        && enemigo.aliado != f.aliado
                                        && self.contacto(f.posicion, enemigo.posicion)
                                }))
                    })
                {
                    continue;
                }
                padres.insert(b, a);
                cola.push_back(b);
            }
        }
        None
    }
    fn elegir(&self, i: usize) -> Intento {
        let f = &self.fichas[i];
        if !f.puede_actuar() {
            return Intento::Esperar;
        }
        let retirando = f.aliado && (self.retirando || self.fichas[self.objetivo].muerto());
        if retirando {
            if !self.fichas.iter().any(|p| p.transportado_por == Some(i))
                && let Some((j, herido)) = self
                    .fichas
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| {
                        p.aliado && p.golpes == 2 && !p.salio && p.transportado_por.is_none()
                    })
                    .min_by_key(|(_, p)| distancia(f.posicion, p.posicion))
            {
                if self.contacto(f.posicion, herido.posicion) {
                    return Intento::Recoger(j);
                }
                if let Some(p) = self.siguiente(i, herido.posicion, true) {
                    return Intento::Mover(p);
                }
            }
            if f.posicion == self.entrada {
                return Intento::Salir;
            }
            if let Some(p) = self.siguiente(i, self.entrada, false) {
                return Intento::Mover(p);
            }
            if let Some(j) = self.fichas.iter().enumerate().find_map(|(j, e)| {
                (!e.aliado && e.puede_actuar() && self.contacto(f.posicion, e.posicion))
                    .then_some(j)
            }) {
                return Intento::Atacar(j);
            }
            return Intento::Esperar;
        }
        let enemigos = self.fichas.iter().enumerate().filter(|(_, e)| {
            e.aliado != f.aliado
                && !e.salio
                && e.transportado_por.is_none()
                && (e.puede_actuar()
                    || (f.aliado && e.actor == self.fichas[self.objetivo].actor && !e.muerto()))
        });
        let blanco = enemigos
            .min_by_key(|(j, e)| (distancia(f.posicion, e.posicion), *j))
            .map(|(j, _)| j);
        let Some(j) = blanco else {
            return Intento::Esperar;
        };
        if self.contacto(f.posicion, self.fichas[j].posicion) {
            return Intento::Atacar(j);
        }
        if !f.aliado && !self.descubierta {
            return Intento::Esperar;
        }
        self.siguiente(i, self.fichas[j].posicion, true)
            .map(Intento::Mover)
            .unwrap_or(Intento::Esperar)
    }
    pub fn avanzar(&mut self, azar: &mut dyn FuenteAzar) -> Result<(), String> {
        if self.desenlace.is_some() {
            return Err("encuentro terminado".into());
        }
        let mut candidato = self.clone();
        candidato.resolver(azar)?;
        *self = candidato;
        Ok(())
    }
    fn resolver(&mut self, azar: &mut dyn FuenteAzar) -> Result<(), String> {
        self.pulso = self.pulso.checked_add(1).ok_or("contador agotado")?;
        self.sucesos.clear();
        self.descubierta |= self
            .fichas
            .iter()
            .any(|f| f.aliado && f.puede_actuar() && f.posicion.x > 15);
        let antes = self.fichas.clone();
        let intentos = (0..antes.len()).map(|i| self.elegir(i)).collect::<Vec<_>>();
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
                atacante: i,
                blanco: j,
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
            if let Intento::Recoger(j) = intentos[i]
                && self.fichas[j].golpes == 2
                && self.fichas[j].transportado_por.is_none()
            {
                self.fichas[j].transportado_por = Some(i);
                self.sucesos.push(Suceso::Recogido { ficha: j, por: i });
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
                    for f in &mut self.fichas {
                        if f.transportado_por == Some(i) {
                            f.salio = true;
                            f.transportado_por = None;
                        }
                    }
                    let quedan = self.fichas.iter().any(|f| {
                        f.aliado && f.golpes == 2 && !f.salio && f.transportado_por.is_none()
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
            self.sucesos.push(Suceso::Paso {
                ficha: i,
                desde,
                hasta: p,
            });
        }
        for i in 0..self.fichas.len() {
            if let Some(j) = self.fichas[i].transportado_por {
                if self.fichas[j].puede_actuar() || self.fichas[j].salio {
                    self.fichas[i].posicion = self.fichas[j].posicion;
                    self.fichas[i].salio = self.fichas[j].salio;
                } else {
                    self.fichas[i].transportado_por = None;
                }
            }
        }
        let libres = self.fichas.iter().any(|f| f.aliado && f.puede_actuar());
        let fuera = self
            .fichas
            .iter()
            .filter(|f| f.aliado && !f.muerto())
            .all(|f| f.salio);
        if fuera && self.fichas.iter().any(|f| f.aliado && !f.muerto()) {
            self.desenlace = Some(if self.fichas[self.objetivo].muerto() {
                Desenlace::Exito
            } else {
                Desenlace::Retirada
            });
        } else if !libres {
            self.desenlace = Some(
                self.fichas
                    .iter()
                    .find(|f| !f.aliado && f.puede_actuar())
                    .map(|f| Desenlace::Captura { captor: f.actor })
                    .unwrap_or(Desenlace::Derrota),
            );
        }
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
        Ok(())
    }
    pub fn reconsiderar(&mut self, retirada: bool) {
        self.retirando = retirada;
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
        let lugar = IdLugar::desde_posicion(0).unwrap();
        Mundo::nuevo(DatosInicialesMundo {
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
        })
        .unwrap()
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
            e.fichas[i].posicion = Posicion {
                area: 0,
                nivel: 0,
                x,
                y: 8,
            };
        }
        e.descubierta = true;
        // Todos los golpes fallan: comprobamos posicionamiento, sin que una baja abra espacio.
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
        e.fichas[0].posicion = Posicion {
            area: 0,
            nivel: 0,
            x: 20,
            y: 8,
        };
        e.fichas[1].posicion = Posicion {
            area: 0,
            nivel: 0,
            x: 21,
            y: 8,
        };
        e.fichas[0].golpes = 1;
        e.fichas[1].golpes = 1;
        e.descubierta = true;
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
            e.fichas[0].posicion = Posicion {
                area: 0,
                nivel: 0,
                x: 20,
                y: 8,
            };
            e.fichas[1].posicion = Posicion {
                area: 0,
                nivel: 0,
                x: 21,
                y: 8,
            };
            e.entrada = Posicion {
                area: 0,
                nivel: 0,
                x: 21,
                y: 7,
            };
            e.retirando = true;
            e.descubierta = true;
            assert_eq!(e.elegir(0), Intento::Mover(e.entrada));
            e.avanzar(&mut Dado(dado)).unwrap();
            assert_eq!(e.fichas[0].posicion == e.entrada, se_mueve);
            assert_eq!(
                e.sucesos
                    .iter()
                    .filter(|s| matches!(
                        s,
                        Suceso::Golpe {
                            atacante: 1,
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
                area: 0,
                nivel: 0,
                x: if i < 5 { 4 } else { 25 },
                y: 2 + ((i % 5) * 3) as u16,
            };
        }
        assert!(
            e.vecinos(Posicion {
                area: 0,
                nivel: 0,
                x: 4,
                y: 8
            })
            .contains(&Posicion {
                area: 0,
                nivel: 0,
                x: 5,
                y: 9
            })
        );
        assert!(!e.contacto(
            Posicion {
                area: 0,
                nivel: 0,
                x: 14,
                y: 7
            },
            Posicion {
                area: 0,
                nivel: 0,
                x: 15,
                y: 8
            }
        ));
        let antes = e.fichas.clone();
        e.avanzar(&mut Dado(0)).unwrap();
        assert!(
            e.sucesos
                .iter()
                .any(|s| matches!(s,Suceso::Paso{ficha,..} if *ficha<5))
        );
        assert!(
            e.sucesos
                .iter()
                .any(|s| matches!(s,Suceso::Paso{ficha,..} if *ficha>=5))
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
}
