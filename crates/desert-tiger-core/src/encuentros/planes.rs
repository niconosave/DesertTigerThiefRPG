//! Dirección genérica: autoridad, roles y cometidos independientes de las reglas.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RolEncuentro {
    Actuante,
    Auxiliador(IdFicha),
    Evacuado,
    Oponente,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cometido {
    Alcanzar,
    AfrontarOposicion,
    ActuarSobreObjetivo,
    Auxiliar,
    Evacuar,
    Esperar,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotivoBloqueo {
    SinParticipantes,
    SinRuta,
    SinProgreso,
    PresupuestoAgotado,
    ParticipanteNoDisponible,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DireccionFicha {
    pub id: IdComportamiento,
    pub plan: IdPlan,
    pub rol: RolEncuentro,
    pub cometido: Cometido,
    pub bloqueo: Option<MotivoBloqueo>,
}
impl Encuentro {
    pub fn direccion(&self, ficha: IdFicha) -> Option<&DireccionFicha> {
        self.direcciones.get(&ficha)
    }

    pub(super) fn consumir_transicion(&mut self) -> Result<(), ErrorEncuentro> {
        if self.transiciones_direccion >= self.limite_direccion {
            return Err(ErrorEncuentro::PresupuestoDireccionAgotado);
        }
        self.transiciones_direccion += 1;
        Ok(())
    }
    pub(super) fn coordinar_planes(&mut self) -> Result<(), ErrorEncuentro> {
        let anteriores = std::mem::take(&mut self.direcciones);
        self.rutas_decision.clear();
        let mut orden: Vec<_> = self
            .planes
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                !matches!(
                    p.estado,
                    EstadoPlan::Suspendido | EstadoPlan::Cumplido | EstadoPlan::Cancelado
                )
            })
            .map(|(i, p)| (std::cmp::Reverse(p.prioridad), p.id, i))
            .collect();
        orden.sort();
        let mut reservados = BTreeSet::new();
        for (_, _, i) in orden {
            self.consumir_transicion()?;
            let plan = &self.planes[i];
            let grupo = plan.tipo != TipoPlan::Oposicion;
            let retirada = grupo
                && (plan.proposito == PropositoPlan::Retirarse
                    || self.objetivo_muerto()
                    || self
                        .botin
                        .as_ref()
                        .is_some_and(|b| b.portador.is_some() || b.fuera)
                    || (self.rescate
                        && self
                            .indice_objetivo()
                            .is_some_and(|j| !self.fichas[j].cautivo)));
            let mut disponibles: Vec<_> = self
                .fichas
                .iter()
                .filter(|f| {
                    f.puede_actuar()
                        && plan.participantes.contains(&f.id)
                        && !self.direcciones.contains_key(&f.id)
                })
                .map(|f| f.id)
                .collect();
            disponibles.sort();
            self.planes[i].estado = if disponibles.is_empty() {
                EstadoPlan::Bloqueado
            } else {
                EstadoPlan::Activo
            };
            self.planes[i].bloqueo =
                disponibles
                    .is_empty()
                    .then_some(if self.planes[i].participantes.is_empty() {
                        MotivoBloqueo::SinParticipantes
                    } else {
                        MotivoBloqueo::ParticipanteNoDisponible
                    });
            for id in disponibles {
                self.consumir_transicion()?;
                let f = self.ficha(id).unwrap();
                let indice = self.fichas.iter().position(|a| a.id == id).unwrap();
                let rutas = self.mapa_rutas(indice);
                let aux = if retirada
                    && f.aliado
                    && !self.fichas.iter().any(|h| h.transportado_por == Some(id))
                {
                    self.fichas
                        .iter()
                        .filter(|h| {
                            h.aliado
                                && !h.cautivo
                                && !h.salio
                                && h.golpes == 2
                                && h.transportado_por.is_none()
                                && !reservados.contains(&h.id)
                                && !self.abandonados_confirmados.contains(&h.id)
                        })
                        .filter_map(|h| {
                            rutas
                                .iter()
                                .filter(|(p, _)| self.contacto(**p, h.posicion))
                                .map(|(_, (coste, _))| *coste)
                                .min()
                                .map(|coste| (coste, h.id))
                        })
                        .min()
                        .map(|(_, id)| id)
                } else {
                    None
                };
                let (rol, cometido) = if let Some(h) = aux {
                    reservados.insert(h);
                    (RolEncuentro::Auxiliador(h), Cometido::Auxiliar)
                } else if retirada {
                    (RolEncuentro::Evacuado, Cometido::Evacuar)
                } else if f.aliado {
                    (RolEncuentro::Actuante, Cometido::Alcanzar)
                } else {
                    (RolEncuentro::Oponente, Cometido::AfrontarOposicion)
                };
                let comportamiento = if let Some(anterior) = anteriores
                    .get(&id)
                    .filter(|d| d.plan == self.planes[i].id && d.rol == rol)
                {
                    anterior.id
                } else {
                    let siguiente = self.proximo_comportamiento.checked_add(1).ok_or(
                        ErrorEncuentro::IdentidadesAgotadas {
                            dominio: "comportamiento",
                        },
                    )?;
                    let nuevo = IdComportamiento(self.proximo_comportamiento);
                    self.proximo_comportamiento = siguiente;
                    nuevo
                };
                self.rutas_decision.insert(id, rutas);
                self.direcciones.insert(
                    id,
                    DireccionFicha {
                        id: comportamiento,
                        plan: self.planes[i].id,
                        rol,
                        cometido,
                        bloqueo: None,
                    },
                );
            }
        }
        Ok(())
    }
    pub(super) fn actualizar_bloqueos_direccion(&mut self) {
        for plan in self
            .planes
            .iter_mut()
            .filter(|p| p.estado == EstadoPlan::Activo)
        {
            let ds: Vec<_> = self
                .direcciones
                .values()
                .filter(|d| d.plan == plan.id)
                .collect();
            if !ds.is_empty() && ds.iter().all(|d| d.bloqueo.is_some()) {
                plan.estado = EstadoPlan::Bloqueado;
                plan.bloqueo = ds[0].bloqueo;
            }
        }
    }
    pub(super) fn grupo_sin_direccion_practicable(&self) -> bool {
        let ds: Vec<_> = self
            .fichas
            .iter()
            .filter(|f| f.aliado && f.puede_actuar())
            .map(|f| self.direcciones.get(&f.id))
            .collect();
        !ds.is_empty() && ds.iter().all(|d| d.is_some_and(|d| d.bloqueo.is_some()))
    }
}
