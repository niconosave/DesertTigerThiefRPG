//! Geometría, ocupación y navegación del escenario local.
use super::*;
#[cfg(test)]
thread_local! {pub(super) static BUSQUEDAS_PRUEBA: std::cell::Cell<usize> = const {std::cell::Cell::new(0)};}

impl Encuentro {
    pub fn terreno(&self, p: Posicion) -> Option<Celda> {
        if p.x >= self.ancho || p.y >= self.alto || p.z >= self.altura {
            None
        } else {
            self.celdas
                .get(usize::from(((p.z * self.alto + p.y) * self.ancho) + p.x))
                .copied()
        }
    }

    pub fn voxeles(&self) -> impl Iterator<Item = (Posicion, Celda)> + '_ {
        (0..self.altura).flat_map(move |z| {
            (0..self.alto).flat_map(move |y| {
                (0..self.ancho).filter_map(move |x| {
                    let p = Posicion { x, y, z };
                    self.terreno(p).map(|v| (p, v))
                })
            })
        })
    }

    pub fn validar_geometria(&self) -> Result<(), ErrorGeometria> {
        use ErrorGeometria::*;
        if !(1..=MAX_ANCHO).contains(&self.ancho)
            || !(1..=MAX_PROFUNDIDAD).contains(&self.alto)
            || !(1..=MAX_ALTURA).contains(&self.altura)
            || self.celdas.len()
                != usize::from(self.ancho) * usize::from(self.alto) * usize::from(self.altura)
            || self.fichas.len() > 32
            || self.elementos.len() > 512
            || self.contexto.espacios.len() > 128
            || self.contexto.conexiones.len() > 256
        {
            return Err(VolumenInvalido);
        }
        if !self.suelo(self.entrada) {
            return Err(PerfilNoCabe(self.entrada));
        }
        let mut ids = BTreeSet::new();
        let mut ocupados = BTreeSet::new();
        for f in &self.fichas {
            if !ids.insert(f.id)
                || f.transportado_por
                    .is_some_and(|id| id == f.id || self.ficha(id).is_none())
            {
                return Err(ReferenciaInvalida);
            }
            if f.salio || f.transportado_por.is_some() {
                continue;
            }
            if !self.suelo(f.posicion) {
                return Err(PerfilNoCabe(f.posicion));
            }
            if f.puede_actuar() {
                for z in f.posicion.z..=f.posicion.z + 1 {
                    let p = Posicion { z, ..f.posicion };
                    if self.puerta_cerrada_en(p).is_some()
                        || self.obstaculo_en(p).is_some()
                        || !ocupados.insert(p)
                    {
                        return Err(VolumenOcupado(f.id));
                    }
                }
            }
        }
        // El transporte es una relación dirigida: sus cadenas no pueden cerrarse.
        for ficha in &self.fichas {
            let mut vistos = BTreeSet::new();
            let mut actual = Some(ficha.id);
            while let Some(id) = actual {
                if !vistos.insert(id) {
                    return Err(ReferenciaInvalida);
                }
                actual = self.ficha(id).ok_or(ReferenciaInvalida)?.transportado_por;
            }
        }
        let espacios: BTreeSet<_> = self.contexto.espacios.iter().map(|e| e.id).collect();
        let conexiones: BTreeSet<_> = self.contexto.conexiones.iter().map(|c| c.id).collect();
        let elementos: BTreeSet<_> = self.elementos.iter().map(|e| e.id).collect();
        if espacios.len() != self.contexto.espacios.len()
            || conexiones.len() != self.contexto.conexiones.len()
            || elementos.len() != self.elementos.len()
            || self.contexto.espacios.iter().any(|e| {
                self.terreno(e.min).is_none()
                    || self.terreno(e.max).is_none()
                    || e.min.x > e.max.x
                    || e.min.y > e.max.y
                    || e.min.z > e.max.z
            })
            || self.elementos.iter().any(|e| {
                e.alto == 0
                    || e.envolvente().is_none()
                    || (matches!(e.clase, ClaseElemento::PresenciaProbable) == e.definido)
                    || self.terreno(e.posicion).is_none()
                    || e.posicion.z.saturating_add(e.alto) > self.altura
                    || match e.clase {
                        ClaseElemento::PuertaManual { conexion, .. } => {
                            e.alto != 2
                                || !self.suelo(e.posicion)
                                || !self.contexto.conexiones.iter().any(|c| {
                                    c.id == conexion
                                        && c.control == Some(e.id)
                                        && c.recorrido.contains(&e.posicion)
                                })
                        }
                        _ => false,
                    }
            })
            || self.objetivo.is_some_and(|id| !ids.contains(&id))
            || self.botin.as_ref().is_some_and(|b| {
                !b.fuera
                    && b.portador.is_none()
                    && (!self.suelo(b.posicion) || self.obstaculo_en_perfil(b.posicion).is_some())
            })
            || self
                .botin
                .as_ref()
                .and_then(|b| b.portador)
                .is_some_and(|id| !ids.contains(&id))
            || self
                .referencias
                .values()
                .any(|&p| self.terreno(p).is_none())
            || self.contexto.conexiones.iter().any(|c| {
                !espacios.contains(&c.desde)
                    || !espacios.contains(&c.hasta)
                    || c.control.is_some_and(|id| {
                        !self.elementos.iter().any(|e| {
                            e.id == id
                                && matches!(e.clase,
                                ClaseElemento::PuertaManual { conexion, .. } if conexion == c.id)
                        })
                    })
                    || c.recorrido.is_empty()
                    || c.recorrido.iter().any(|&p| !self.suelo(p))
                    || c.recorrido.windows(2).any(|p| {
                        if p[0].z == p[1].z {
                            !self.contacto(p[0], p[1])
                        } else {
                            !self.transicion_vertical(p[0], p[1])
                        }
                    })
            })
        {
            return Err(ReferenciaInvalida);
        }
        Ok(())
    }

    pub(super) fn suelo(&self, p: Posicion) -> bool {
        p.z > 0
            && matches!(
                self.terreno(Posicion { z: p.z - 1, ..p }),
                Some(Celda::Suelo | Celda::Muro | Celda::Solido(_))
            )
            && self.terreno(p) == Some(Celda::Aire)
            && self.terreno(Posicion { z: p.z + 1, ..p }) == Some(Celda::Aire)
    }

    pub(super) fn puerta_cerrada_en(&self, p: Posicion) -> Option<IdElementoLocal> {
        self.elementos.iter().find_map(|e| match e.clase {
            ClaseElemento::PuertaManual {
                estado: EstadoPuerta::Cerrada,
                ..
            } if e.posicion.x == p.x
                && e.posicion.y == p.y
                && p.z >= e.posicion.z
                && p.z < e.posicion.z + e.alto =>
            {
                Some(e.id)
            }
            _ => None,
        })
    }

    pub(super) fn obstaculo_en(&self, p: Posicion) -> Option<IdElementoLocal> {
        self.elementos
            .iter()
            .find(|e| {
                matches!(e.clase, ClaseElemento::Obstaculo)
                    && e.posicion.x == p.x
                    && e.posicion.y == p.y
                    && p.z >= e.posicion.z
                    && p.z - e.posicion.z < e.alto
            })
            .map(|e| e.id)
    }

    pub(super) fn obstaculo_en_perfil(&self, p: Posicion) -> Option<IdElementoLocal> {
        self.obstaculo_en(p).or_else(|| {
            self.obstaculo_en(Posicion {
                z: p.z.saturating_add(1),
                ..p
            })
        })
    }

    pub(super) fn contacto(&self, a: Posicion, b: Posicion) -> bool {
        if a.z != b.z || distancia(a, b) != 1 {
            return false;
        }
        if a.x != b.x && a.y != b.y {
            [Posicion { y: b.y, ..a }, Posicion { x: b.x, ..a }]
                .into_iter()
                .all(|p| {
                    self.suelo(p)
                        && self.obstaculo_en_perfil(p).is_none()
                        && self.puerta_cerrada_en(p).is_none()
                })
        } else {
            true
        }
    }

    pub fn contacto_hostil(&self) -> bool {
        self.fichas.iter().any(|a| {
            a.puede_actuar()
                && self.fichas.iter().any(|b| {
                    b.puede_actuar()
                        && a.aliado != b.aliado
                        && self.contacto(a.posicion, b.posicion)
                })
        })
    }

    pub(super) fn transicion_vertical(&self, a: Posicion, b: Posicion) -> bool {
        let inferior = if a.z < b.z { a } else { b };
        a.z.abs_diff(b.z) == 1
            && u32::from(a.x.abs_diff(b.x)) + u32::from(a.y.abs_diff(b.y)) == 1
            && self.suelo(a)
            && self.suelo(b)
            && self.terreno(Posicion {
                z: inferior.z + 2,
                ..inferior
            }) == Some(Celda::Aire)
    }

    pub(super) fn vecinos(&self, p: Posicion) -> Vec<Posicion> {
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
                    z: p.z,
                    x: x as u16,
                    y: y as u16,
                };
                if self.suelo(q) && self.obstaculo_en_perfil(q).is_none() && self.contacto(p, q) {
                    r.push(q);
                }
            }
        }
        for conexion in &self.contexto.conexiones {
            if let Some(k) = conexion.recorrido.iter().position(|q| *q == p) {
                for q in [
                    k.checked_sub(1),
                    (k + 1 < conexion.recorrido.len()).then_some(k + 1),
                ]
                .into_iter()
                .flatten()
                .map(|j| conexion.recorrido[j])
                {
                    if self.transicion_vertical(p, q)
                        && self.obstaculo_en_perfil(q).is_none()
                        && !r.contains(&q)
                    {
                        r.push(q);
                    }
                }
            }
        }
        r
    }

    pub fn vecinos_caminables(&self, p: Posicion) -> Vec<Posicion> {
        self.vecinos(p)
            .into_iter()
            .filter(|q| self.puerta_cerrada_en(*q).is_none())
            .collect()
    }

    pub fn puede_ocupar(
        &self,
        p: Posicion,
        excluir: Option<IdFicha>,
    ) -> Result<(), ImpedimentoOcupacion> {
        if self.terreno(p).is_none() {
            return Err(ImpedimentoOcupacion::FueraDeLimites);
        }
        if !self.suelo(p) {
            return Err(ImpedimentoOcupacion::SinApoyoOAltura);
        }
        if let Some(id) = self.obstaculo_en_perfil(p) {
            return Err(ImpedimentoOcupacion::Obstaculo(id));
        }
        if let Some(id) = self
            .puerta_cerrada_en(p)
            .or_else(|| self.puerta_cerrada_en(Posicion { z: p.z + 1, ..p }))
        {
            return Err(ImpedimentoOcupacion::PuertaCerrada(id));
        }
        if let Some(f) = self.fichas.iter().find(|f| {
            Some(f.id) != excluir
                && f.puede_actuar()
                && f.posicion.x == p.x
                && f.posicion.y == p.y
                && f.posicion.z.abs_diff(p.z) < 2
        }) {
            return Err(ImpedimentoOcupacion::Ficha(f.id));
        }
        Ok(())
    }

    pub fn cambiar_puerta(
        &mut self,
        id: IdElementoLocal,
        estado_nuevo: EstadoPuerta,
        por: usize,
    ) -> Result<(), ErrorEncuentro> {
        if por >= self.fichas.len() || !self.fichas[por].puede_actuar() {
            return Err("participante no disponible".into());
        }
        let pos = self
            .elementos
            .iter()
            .find(|e| e.id == id)
            .ok_or("elemento inexistente")?
            .posicion;
        if !self.contacto(self.fichas[por].posicion, pos) {
            return Err("contacto bloqueado".into());
        }
        let elemento = self
            .elementos
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or("elemento inexistente")?;
        if distancia(self.fichas[por].posicion, elemento.posicion) != 1
            || self.fichas[por].posicion.z != elemento.posicion.z
        {
            return Err("elemento fuera de alcance".into());
        }
        let ClaseElemento::PuertaManual { estado, .. } = &mut elemento.clase else {
            return Err("elemento no es puerta".into());
        };
        if *estado == estado_nuevo {
            return Err("sin cambio".into());
        }
        if estado_nuevo == EstadoPuerta::Cerrada
            && self.fichas.iter().any(|f| {
                f.puede_actuar()
                    && f.posicion.x == elemento.posicion.x
                    && f.posicion.y == elemento.posicion.y
                    && f.posicion.z.abs_diff(elemento.posicion.z) < 2
            })
        {
            return Err("vano ocupado".into());
        }
        *estado = estado_nuevo;
        self.contexto.revision_geometria += 1;
        self.sucesos.push(Suceso::PuertaCambiada {
            elemento: id,
            por: self.fichas[por].id,
            estado: estado_nuevo,
        });
        Ok(())
    }

    pub(super) fn mapa_rutas(&self, desde: usize) -> BTreeMap<Posicion, (u32, Posicion)> {
        #[cfg(test)]
        BUSQUEDAS_PRUEBA.with(|n| n.set(n.get() + 1));
        let Ok(conocido) = self.contexto_navegacion(self.fichas[desde].id) else {
            return BTreeMap::new();
        };
        let mut escena = self.clone();
        for (p, _) in self.voxeles() {
            let i = usize::from((p.z * self.alto + p.y) * self.ancho + p.x);
            escena.celdas[i] = conocido.voxel(p).unwrap_or(Celda::Vacio);
        }
        // Solo se autoriza el primer paso por un umbral conocido: no el cuarto entero.
        for c in conocido.conexiones().iter().filter(|c| !c.salida) {
            for umbral in &c.recorrido {
                for p in self.vecinos(*umbral) {
                    if self.terreno_descubierto(p) {
                        continue;
                    }
                    for z in p.z.saturating_sub(1)..=p.z.saturating_add(1) {
                        let q = Posicion { z, ..p };
                        if let Some(v) = self.terreno(q) {
                            let i = usize::from((q.z * self.alto + q.y) * self.ancho + q.x);
                            escena.celdas[i] = v;
                        }
                    }
                }
            }
        }
        escena.elementos = self
            .elementos
            .iter()
            .filter_map(|e| conocido.elemento(e.id).cloned())
            .collect();
        // Conservar índices privados pero eliminar influencia de presencias desconocidas.
        for f in &mut escena.fichas {
            if !conocido.participantes().contains(&f.id) {
                f.salio = true;
            }
        }
        escena.mapa_rutas_fisicas(desde)
    }

    pub(super) fn mapa_rutas_fisicas(&self, desde: usize) -> BTreeMap<Posicion, (u32, Posicion)> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let ficha = &self.fichas[desde];
        let origen = ficha.posicion;
        let mut rutas = BTreeMap::from([(origen, (0, origen))]);
        let mut cola = BinaryHeap::from([Reverse((0, origen))]);
        while let Some(Reverse((coste, a))) = cola.pop() {
            if rutas[&a].0 != coste {
                continue;
            }
            for b in self.vecinos(a) {
                if self.fichas.iter().any(|f| {
                    f.id != ficha.id
                        && f.puede_actuar()
                        && f.posicion == b
                        && (f.aliado != ficha.aliado
                            || self.fichas.iter().any(|enemigo| {
                                enemigo.puede_actuar()
                                    && enemigo.aliado != f.aliado
                                    && self.contacto(f.posicion, enemigo.posicion)
                            }))
                }) {
                    continue;
                }
                let nuevo = coste
                    + if self.puerta_cerrada_en(b).is_some() {
                        2
                    } else {
                        1
                    };
                if rutas.get(&b).is_some_and(|&(previo, _)| previo <= nuevo) {
                    continue;
                }
                let primero = if a == origen { b } else { rutas[&a].1 };
                rutas.insert(b, (nuevo, primero));
                cola.push(Reverse((nuevo, b)));
            }
        }
        rutas
    }
}
