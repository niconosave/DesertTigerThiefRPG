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
}
impl Encuentro {
    pub fn direccion(&self, ficha: IdFicha) -> Option<&DireccionFicha> {
        self.direcciones.get(&ficha)
    }

    pub(super) fn coordinar_planes(&mut self) {
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
            let plan = &self.planes[i];
            let grupo = plan
                .participantes
                .iter()
                .any(|id| self.ficha(*id).is_some_and(|f| f.aliado));
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
            self.planes[i].bloqueo = disponibles
                .is_empty()
                .then_some(MotivoBloqueo::SinParticipantes);
            for id in disponibles {
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
                    let Some(siguiente) = self.proximo_comportamiento.checked_add(1) else {
                        self.planes[i].estado = EstadoPlan::Bloqueado;
                        self.planes[i].bloqueo = Some(MotivoBloqueo::PresupuestoAgotado);
                        continue;
                    };
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
                    },
                );
            }
        }
    }
}
