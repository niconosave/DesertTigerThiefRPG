//! Cometidos genéricos; las reglas del pulso permanecen en el resolutor.
use super::*;
impl Encuentro {
    pub(super) fn elegir(&self, i: usize) -> Intento {
        let f = &self.fichas[i];
        if !f.puede_actuar() {
            return Intento::Esperar;
        }
        let rutas = std::cell::OnceCell::new();
        let siguiente = |meta, adyacente| {
            rutas
                .get_or_init(|| {
                    self.rutas_decision
                        .get(&f.id)
                        .cloned()
                        .unwrap_or_else(|| self.mapa_rutas(i))
                })
                .iter()
                .filter(|&(p, _)| {
                    *p != f.posicion
                        && !self
                            .fichas
                            .iter()
                            .any(|otra| otra.puede_actuar() && otra.posicion == *p)
                        && if adyacente {
                            self.contacto(*p, meta)
                        } else {
                            *p == meta
                        }
                })
                .min_by_key(|&(p, (coste, _))| (*coste, *p))
                .map(|(_, &(_, primero))| primero)
        };
        let objetivo = self.indice_objetivo().map(|j| &self.fichas[j]);
        let proposito = self
            .planes
            .iter()
            .filter(|p| p.estado == EstadoPlan::Activo && p.participantes.contains(&f.id))
            .min_by_key(|p| (std::cmp::Reverse(p.prioridad), p.id))
            .map(|p| p.proposito);
        if proposito.is_none() {
            return Intento::Esperar;
        }
        if let Some((plan, puerta, estado)) = self.interacciones_puerta.get(&f.id)
            && self.direccion(f.id).is_some_and(|d| d.plan == *plan)
        {
            return match estado {
                EstadoPuerta::Abierta => Intento::Abrir(*puerta),
                EstadoPuerta::Cerrada => Intento::Cerrar(*puerta),
            };
        }
        let retirando = f.aliado
            && (proposito == Some(PropositoPlan::Retirarse)
                || if self.rescate {
                    !objetivo.unwrap().cautivo || objetivo.unwrap().muerto()
                } else {
                    self.objetivo_muerto()
                        || self
                            .botin
                            .as_ref()
                            .is_some_and(|b| b.portador.is_some() || b.fuera)
                });
        if retirando {
            return self.comportamiento_evacuar(i, &siguiente);
        }
        if let Some(intento) = self.comportamiento_explorar(i, &siguiente) {
            return intento;
        }
        if matches!(
            proposito,
            Some(PropositoPlan::Rescatar | PropositoPlan::Recuperar)
        ) && f.aliado
        {
            return self.comportamiento_recuperar(i, &siguiente);
        }
        self.comportamiento_combatir(i, &siguiente)
    }
    fn comportamiento_evacuar(
        &self,
        i: usize,
        siguiente: &impl Fn(Posicion, bool) -> Option<Posicion>,
    ) -> Intento {
        let f = &self.fichas[i];

        if !self.fichas.iter().any(|p| p.transportado_por == Some(f.id))
            && let Some((j, herido)) = self
                .fichas
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    p.aliado
                        && !p.cautivo
                        && p.golpes == 2
                        && !p.salio
                        && p.transportado_por.is_none()
                        && self.direccion(f.id).is_none_or(
                            |d| matches!(d.rol, RolEncuentro::Auxiliador(id) if id == p.id),
                        )
                })
                .min_by_key(|(_, p)| distancia(f.posicion, p.posicion))
        {
            if self.contacto(f.posicion, herido.posicion) {
                return Intento::Recoger(j);
            }
            if let Some(p) = siguiente(herido.posicion, true) {
                return Intento::Mover(p);
            }
        }
        if f.posicion == self.entrada {
            return Intento::Salir;
        }
        if let Some(p) = siguiente(self.entrada, false) {
            return Intento::Mover(p);
        }
        if let Some(j) = self.fichas.iter().enumerate().find_map(|(j, e)| {
            (!e.aliado && e.puede_actuar() && self.contacto(f.posicion, e.posicion)).then_some(j)
        }) {
            return Intento::Atacar(j);
        }
        Intento::Esperar
    }
    fn comportamiento_explorar(
        &self,
        i: usize,
        siguiente: &impl Fn(Posicion, bool) -> Option<Posicion>,
    ) -> Option<Intento> {
        let f = &self.fichas[i];
        let objetivo_conocido = self
            .botin
            .as_ref()
            .is_some_and(|b| self.terreno_descubierto(b.posicion))
            || self
                .indice_objetivo()
                .is_some_and(|j| self.ficha_observable(&self.fichas[j]));
        if objetivo_conocido {
            return None;
        }
        if f.aliado
            && let Some(paso) = self
                .contexto
                .conexiones
                .iter()
                .filter(|c| !c.salida)
                .filter_map(|c| {
                    // La frontera puede estar más allá de una sala ya conocida.
                    // Las rutas siguen limitadas a la memoria y al primer umbral.
                    let destino = match (
                        self.espacios_descubiertos.contains(&c.desde),
                        self.espacios_descubiertos.contains(&c.hasta),
                    ) {
                        (true, false) => c.hasta,
                        (false, true) => c.desde,
                        _ => return None,
                    };
                    let umbral = *c.recorrido.last()?;
                    self.vecinos(umbral)
                        .into_iter()
                        .filter(|&p| self.espacio_en(p) == Some(destino))
                        .find_map(|p| siguiente(p, false))
                })
                .next()
        {
            return Some(Intento::Mover(paso));
        }

        None
    }
    fn comportamiento_recuperar(
        &self,
        i: usize,
        siguiente: &impl Fn(Posicion, bool) -> Option<Posicion>,
    ) -> Intento {
        let f = &self.fichas[i];
        let objetivo = self.indice_objetivo().map(|j| &self.fichas[j]);

        let posicion = self
            .botin
            .as_ref()
            .map(|b| b.posicion)
            .unwrap_or_else(|| objetivo.unwrap().posicion);
        if self.contacto(f.posicion, posicion) {
            return if self.botin.is_some() {
                Intento::TomarObjeto
            } else {
                Intento::Liberar
            };
        }
        if let Some(j) = self.fichas.iter().enumerate().find_map(|(j, e)| {
            (!e.aliado && e.puede_actuar() && self.contacto(f.posicion, e.posicion)).then_some(j)
        }) {
            return Intento::Atacar(j);
        }
        siguiente(posicion, true)
            .map(Intento::Mover)
            .unwrap_or(Intento::Esperar)
    }
    fn comportamiento_combatir(
        &self,
        i: usize,
        siguiente: &impl Fn(Posicion, bool) -> Option<Posicion>,
    ) -> Intento {
        let f = &self.fichas[i];

        let enemigos = self.fichas.iter().enumerate().filter(|(_, e)| {
            e.aliado != f.aliado
                && self.participante_observable_por(f, e)
                && !e.salio
                && e.transportado_por.is_none()
                && (e.puede_actuar()
                    || (!self.rescate
                        && self.botin.is_none()
                        && f.aliado
                        && e.actor == self.fichas[self.indice_objetivo().unwrap()].actor
                        && !e.muerto()))
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
        siguiente(self.fichas[j].posicion, true)
            .map(Intento::Mover)
            .unwrap_or(Intento::Esperar)
    }
}
