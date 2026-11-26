//! Memoria de dirección. Solo registra decisiones y resultados de pulsos confirmados.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FasePlan {
    Preparacion,
    Aproximacion,
    Oposicion,
    Objetivo,
    Salida,
    Terminado,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstadoComportamiento {
    Activo,
    Suspendido,
    Bloqueado,
    Cumplido,
    Interrumpido,
    Cancelado,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComportamientoActivo {
    pub id: IdComportamiento,
    pub plan: IdPlan,
    pub participantes: Vec<IdFicha>,
    pub rol: RolEncuentro,
    pub cometido: Cometido,
    pub destino: Option<Posicion>,
    pub estado: EstadoComportamiento,
    pub motivo: Option<MotivoBloqueo>,
    pub pulsos: u64,
    pub ultimo_pulso: Option<u64>,
    pub pulsos_sin_progreso: u8,
    pub referencias: Vec<IdFicha>,
    pub resultados: Vec<Suceso>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoriaPlan {
    pub proposito_inicial: PropositoPlan,
    pub fase: FasePlan,
    pub estado: EstadoPlan,
    pub resultados: Vec<Suceso>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CambioDireccion {
    Plan {
        anterior: Option<EstadoPlan>,
        nuevo: EstadoPlan,
        motivo: Option<MotivoBloqueo>,
    },
    Fase {
        anterior: FasePlan,
        nueva: FasePlan,
    },
    Comportamiento {
        id: IdComportamiento,
        anterior: Option<EstadoComportamiento>,
        nuevo: EstadoComportamiento,
        motivo: Option<MotivoBloqueo>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SucesoDireccion {
    pub pulso: u64,
    pub plan: IdPlan,
    pub participantes: Vec<IdFicha>,
    pub cambio: CambioDireccion,
}
impl Encuentro {
    pub fn comportamiento(&self, id: IdComportamiento) -> Option<&ComportamientoActivo> {
        self.comportamientos.get(&id)
    }
    pub fn memoria_plan(&self, id: IdPlan) -> Option<&MemoriaPlan> {
        self.memoria_planes.get(&id)
    }
    pub fn sucesos_direccion(&self) -> &[SucesoDireccion] {
        &self.historial_direccion
    }

    fn cambiar_comportamiento(
        &mut self,
        id: IdComportamiento,
        nuevo: EstadoComportamiento,
        motivo: Option<MotivoBloqueo>,
    ) {
        let b = self.comportamientos.get_mut(&id).unwrap();
        if b.estado == nuevo && b.motivo == motivo {
            return;
        }
        let anterior = b.estado;
        b.estado = nuevo;
        b.motivo = motivo;
        self.historial_direccion.push(SucesoDireccion {
            pulso: self.pulso,
            plan: b.plan,
            participantes: b.participantes.clone(),
            cambio: CambioDireccion::Comportamiento {
                id,
                anterior: Some(anterior),
                nuevo,
                motivo,
            },
        });
    }
    pub(super) fn preparar_comportamientos(
        &mut self,
        intentos: &[Intento],
    ) -> Result<(), ErrorEncuentro> {
        for (i, intento) in intentos.iter().enumerate() {
            self.consumir_transicion()?;
            let ficha = self.fichas[i].id;
            let Some(d) = self.direcciones.get(&ficha).cloned() else {
                continue;
            };
            let destino = match intento {
                Intento::Mover(_) => match d.rol {
                    RolEncuentro::Evacuado => Some(self.entrada),
                    RolEncuentro::Auxiliador(id) => self.ficha(id).map(|f| f.posicion),
                    RolEncuentro::Oponente => self
                        .fichas
                        .iter()
                        .filter(|f| {
                            f.aliado != self.fichas[i].aliado
                                && f.puede_actuar()
                                && self.participante_observable_por(&self.fichas[i], f)
                        })
                        .min_by_key(|f| (distancia(self.fichas[i].posicion, f.posicion), f.id))
                        .map(|f| f.posicion),
                    RolEncuentro::Actuante => self
                        .botin
                        .as_ref()
                        .filter(|b| self.terreno_descubierto(b.posicion))
                        .map(|b| b.posicion)
                        .or_else(|| {
                            self.indice_objetivo()
                                .filter(|&j| self.ficha_observable(&self.fichas[j]))
                                .map(|j| self.fichas[j].posicion)
                        })
                        .or_else(|| self.frontera_exploracion(i)),
                },
                Intento::Atacar(j) | Intento::Recoger(j) => Some(self.fichas[*j].posicion),
                Intento::Salir => Some(self.entrada),
                _ => None,
            };
            let reutilizable = self
                .comportamientos
                .values()
                .find(|b| {
                    b.plan == d.plan
                        && b.participantes == [ficha]
                        && b.rol == d.rol
                        && b.cometido == d.cometido
                        && matches!(
                            b.estado,
                            EstadoComportamiento::Activo
                                | EstadoComportamiento::Suspendido
                                | EstadoComportamiento::Bloqueado
                        )
                })
                .map(|b| b.id);
            let id = if let Some(id) = reutilizable {
                id
            } else {
                let id = IdComportamiento(self.proximo_comportamiento);
                self.proximo_comportamiento = self.proximo_comportamiento.checked_add(1).ok_or(
                    ErrorEncuentro::IdentidadesAgotadas {
                        dominio: "comportamiento",
                    },
                )?;
                self.comportamientos.insert(
                    id,
                    ComportamientoActivo {
                        id,
                        plan: d.plan,
                        participantes: vec![ficha],
                        rol: d.rol,
                        cometido: d.cometido,
                        destino,
                        estado: EstadoComportamiento::Activo,
                        motivo: None,
                        pulsos: 0,
                        ultimo_pulso: None,
                        pulsos_sin_progreso: 0,
                        referencias: vec![],
                        resultados: vec![],
                    },
                );
                self.historial_direccion.push(SucesoDireccion {
                    pulso: self.pulso,
                    plan: d.plan,
                    participantes: vec![ficha],
                    cambio: CambioDireccion::Comportamiento {
                        id,
                        anterior: None,
                        nuevo: EstadoComportamiento::Activo,
                        motivo: None,
                    },
                });
                id
            };
            self.direcciones.get_mut(&ficha).unwrap().id = id;
            if !matches!(
                intento,
                Intento::Esperar | Intento::Abrir(_) | Intento::Cerrar(_)
            ) || destino.is_some()
            {
                self.comportamientos.get_mut(&id).unwrap().destino = destino;
            }
            if !matches!(
                intento,
                Intento::Esperar | Intento::Abrir(_) | Intento::Cerrar(_)
            ) {
                self.comportamientos.get_mut(&id).unwrap().referencias = match intento {
                    Intento::Atacar(j) | Intento::Recoger(j) => vec![self.fichas[*j].id],
                    Intento::Liberar => self.objetivo.into_iter().collect(),
                    _ => vec![],
                };
            }
            if let Some(motivo) = d.bloqueo {
                self.cambiar_comportamiento(id, EstadoComportamiento::Bloqueado, Some(motivo));
            } else {
                self.cambiar_comportamiento(id, EstadoComportamiento::Activo, None);
            }
        }
        Ok(())
    }
    pub(super) fn registrar_direccion(&mut self, pulso_resuelto: bool) {
        for p in &self.planes {
            let fase = if matches!(p.estado, EstadoPlan::Cumplido | EstadoPlan::Cancelado) {
                FasePlan::Terminado
            } else if p.estado == EstadoPlan::Suspendido {
                self.memoria_planes
                    .get(&p.id)
                    .map(|m| m.fase)
                    .unwrap_or(FasePlan::Preparacion)
            } else if p.proposito == PropositoPlan::Retirarse
                || (p
                    .participantes
                    .iter()
                    .any(|id| self.ficha(*id).is_some_and(|f| f.aliado))
                    && (self.objetivo_muerto()
                        || self.extraccion_fuera()
                        || self.botin.as_ref().is_some_and(|b| b.portador.is_some())
                        || (self.rescate
                            && self
                                .indice_objetivo()
                                .is_some_and(|i| !self.fichas[i].cautivo))))
            {
                FasePlan::Salida
            } else if !pulso_resuelto {
                self.memoria_planes
                    .get(&p.id)
                    .map(|m| m.fase)
                    .unwrap_or(FasePlan::Preparacion)
            } else if self
                .direcciones
                .values()
                .any(|d| d.plan == p.id && d.cometido == Cometido::ActuarSobreObjetivo)
            {
                FasePlan::Objetivo
            } else if self
                .direcciones
                .values()
                .any(|d| d.plan == p.id && d.cometido == Cometido::AfrontarOposicion)
            {
                FasePlan::Oposicion
            } else {
                FasePlan::Aproximacion
            };
            let existe = self.memoria_planes.contains_key(&p.id);
            let memoria = self
                .memoria_planes
                .entry(p.id)
                .or_insert_with(|| MemoriaPlan {
                    proposito_inicial: p.proposito,
                    fase: FasePlan::Preparacion,
                    estado: p.estado,
                    resultados: vec![],
                });
            if !existe || memoria.estado != p.estado {
                self.historial_direccion.push(SucesoDireccion {
                    pulso: self.pulso,
                    plan: p.id,
                    participantes: p.participantes.clone(),
                    cambio: CambioDireccion::Plan {
                        anterior: existe.then_some(memoria.estado),
                        nuevo: p.estado,
                        motivo: p.bloqueo,
                    },
                });
            }
            if memoria.fase != fase {
                self.historial_direccion.push(SucesoDireccion {
                    pulso: self.pulso,
                    plan: p.id,
                    participantes: p.participantes.clone(),
                    cambio: CambioDireccion::Fase {
                        anterior: memoria.fase,
                        nueva: fase,
                    },
                });
            }
            memoria.estado = p.estado;
            memoria.fase = fase;
        }
        let ids: Vec<_> = self.comportamientos.keys().copied().collect();
        for id in ids {
            let b = &self.comportamientos[&id];
            if matches!(
                b.estado,
                EstadoComportamiento::Cumplido
                    | EstadoComportamiento::Cancelado
                    | EstadoComportamiento::Interrumpido
            ) {
                continue;
            }
            let plan = self.planes.iter().find(|p| p.id == b.plan);
            let disponible = b
                .participantes
                .iter()
                .any(|id| self.ficha(*id).is_some_and(Ficha::puede_actuar));
            let efectivo = b
                .participantes
                .iter()
                .any(|id| self.direcciones.get(id).is_some_and(|d| d.id == b.id));
            let logrado=b.participantes.iter().all(|id|self.ficha(*id).is_some_and(|f|f.salio))
                || b.destino.is_some_and(|p| b.participantes.iter().any(|id|self.ficha(*id).is_some_and(|f|f.posicion==p)))
                || b.resultados.iter().any(|s|matches!(s,Suceso::Liberado{..}|Suceso::ObjetoRecogido{..}|Suceso::Recogido{..}))
                || b.resultados.iter().any(|s|matches!(s,Suceso::Golpe{blanco,acierto:true,..} if self.ficha(*blanco).is_some_and(|f|f.golpes>=2)));
            let mut nuevo = match plan.map(|p| p.estado) {
                Some(EstadoPlan::Cancelado) | None => EstadoComportamiento::Cancelado,
                Some(EstadoPlan::Cumplido) => {
                    if logrado {
                        EstadoComportamiento::Cumplido
                    } else {
                        EstadoComportamiento::Cancelado
                    }
                }
                Some(EstadoPlan::Suspendido) => EstadoComportamiento::Suspendido,
                _ if !disponible
                    && b.participantes
                        .iter()
                        .all(|id| self.ficha(*id).is_some_and(|f| f.salio)) =>
                {
                    EstadoComportamiento::Cumplido
                }
                _ if !disponible => EstadoComportamiento::Interrumpido,
                _ if !efectivo
                    && b.participantes
                        .iter()
                        .any(|id| self.direcciones.get(id).is_some_and(|d| d.plan != b.plan)) =>
                {
                    EstadoComportamiento::Suspendido
                }
                _ if !efectivo && pulso_resuelto => {
                    if logrado {
                        EstadoComportamiento::Cumplido
                    } else {
                        EstadoComportamiento::Interrumpido
                    }
                }
                Some(EstadoPlan::Bloqueado) => EstadoComportamiento::Bloqueado,
                _ => b.estado,
            };
            if pulso_resuelto && efectivo && b.ultimo_pulso != Some(self.pulso) {
                let participantes = b.participantes.clone();
                let plan = b.plan;
                let resultados: Vec<_> = self
                    .sucesos
                    .iter()
                    .filter(|s| match s {
                        Suceso::Paso { ficha, .. } | Suceso::InteraccionPuerta { ficha, .. } => {
                            participantes.contains(ficha)
                        }
                        Suceso::Golpe { atacante, .. } => participantes.contains(atacante),
                        Suceso::PuertaCambiada { por, .. }
                        | Suceso::ObjetoRecogido { por }
                        | Suceso::Liberado { por, .. }
                        | Suceso::Recogido { por, .. } => participantes.contains(por),
                    })
                    .cloned()
                    .collect();
                let b = self.comportamientos.get_mut(&id).unwrap();
                b.pulsos += 1;
                b.ultimo_pulso = Some(self.pulso);
                let progreso = resultados.iter().any(Suceso::es_progreso);
                b.pulsos_sin_progreso = if progreso {
                    0
                } else {
                    b.pulsos_sin_progreso.saturating_add(1)
                };
                if b.pulsos_sin_progreso >= 8 && nuevo == EstadoComportamiento::Activo {
                    nuevo = EstadoComportamiento::Bloqueado;
                }
                b.resultados.extend(resultados.iter().cloned());
                if let Some(m) = self.memoria_planes.get_mut(&plan) {
                    m.resultados.extend(resultados);
                }
            }
            let motivo = match nuevo {
                EstadoComportamiento::Interrumpido
                    if !self.comportamientos[&id]
                        .participantes
                        .iter()
                        .any(|id| self.ficha(*id).is_some_and(Ficha::puede_actuar)) =>
                {
                    Some(MotivoBloqueo::ParticipanteNoDisponible)
                }
                EstadoComportamiento::Bloqueado
                    if self.planes.iter().any(|p| {
                        p.id == self.comportamientos[&id].plan
                            && p.bloqueo == Some(MotivoBloqueo::PresupuestoAgotado)
                    }) =>
                {
                    Some(MotivoBloqueo::PresupuestoAgotado)
                }
                EstadoComportamiento::Bloqueado => self.comportamientos[&id]
                    .participantes
                    .iter()
                    .find_map(|id| self.direcciones.get(id).and_then(|d| d.bloqueo))
                    .or_else(|| {
                        self.planes
                            .iter()
                            .find(|p| p.id == self.comportamientos[&id].plan)
                            .and_then(|p| p.bloqueo)
                    })
                    .or(Some(MotivoBloqueo::SinProgreso)),
                _ => None,
            };
            self.cambiar_comportamiento(id, nuevo, motivo);
        }
    }
}
