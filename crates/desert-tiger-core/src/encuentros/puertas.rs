//! Interacción manual y resolución de puertas en un pulso simultáneo.
use super::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RechazoPuerta {
    NoDisponible,
    FueraDeAlcance,
    ElementoInvalido,
    VanoOcupado,
    PasoEnCurso,
    Bloqueada,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultadoPuerta {
    Cambiada,
    Concurrente { ejecutor: IdFicha },
    SinCambio,
    Rechazada(RechazoPuerta),
}

impl Encuentro {
    pub fn solicitar_puerta(
        &mut self,
        ficha: IdFicha,
        puerta: IdElementoLocal,
        estado: EstadoPuerta,
    ) -> Result<(), ErrorEncuentro> {
        let actor = self.ficha(ficha).ok_or(ErrorEncuentro::ReferenciaLocal {
            ficha,
            elemento: puerta,
        })?;
        let elemento = self.elementos.iter().find(|e| e.id == puerta).ok_or(
            ErrorEncuentro::ReferenciaLocal {
                ficha,
                elemento: puerta,
            },
        )?;
        if !actor.puede_actuar()
            || !matches!(elemento.clase, ClaseElemento::PuertaManual { .. })
            || !self.contacto(actor.posicion, elemento.posicion)
        {
            return Err("interacción fuera de alcance".into());
        }
        let plan = self
            .planes
            .iter()
            .filter(|p| p.estado == EstadoPlan::Activo && p.participantes.contains(&ficha))
            .min_by_key(|p| (std::cmp::Reverse(p.prioridad), p.id))
            .ok_or("sin dirección efectiva")?
            .id;
        self.interacciones_puerta
            .insert(ficha, (plan, puerta, estado));
        Ok(())
    }
    pub(super) fn cerrar_puertas_del_pulso(&mut self, antes: &[Ficha], intentos: &[Intento]) {
        for i in self.prioridad.clone() {
            let Intento::Cerrar(id) = intentos[i] else {
                continue;
            };
            let Some(puerta) = self.elementos.iter().find(|e| e.id == id) else {
                continue;
            };
            let p = puerta.posicion;
            let cruzada = self.sucesos.iter().any(|s| {
                matches!(s, Suceso::Paso {desde,hasta,..}
                if [desde,hasta].iter().any(|q| q.x == p.x && q.y == p.y && q.z.abs_diff(p.z) < 2))
            });
            let abierta_al_inicio = self.puertas_inicio.get(&id) == Some(&EstadoPuerta::Abierta);
            if abierta_al_inicio && !cruzada && antes[i].puede_actuar() {
                let _ = self.cambiar_puerta(id, EstadoPuerta::Cerrada, i);
            }
        }
        self.registrar_interacciones_puerta(antes, intentos);
        for (f, intento) in antes.iter().zip(intentos) {
            if matches!(intento, Intento::Abrir(_) | Intento::Cerrar(_)) {
                self.interacciones_puerta.remove(&f.id);
            }
        }
    }
    fn registrar_interacciones_puerta(&mut self, antes: &[Ficha], intentos: &[Intento]) {
        for (i, intento) in intentos.iter().enumerate() {
            let (elemento, solicitado) = match intento {
                Intento::Abrir(id) => (*id, EstadoPuerta::Abierta),
                Intento::Cerrar(id) => (*id, EstadoPuerta::Cerrada),
                _ => continue,
            };
            let ficha = antes[i].id;
            let puerta = self.elementos.iter().find(|e| e.id == elemento);
            let inicial = self.puertas_inicio.get(&elemento);
            let resultado = if !antes[i].puede_actuar() || !self.fichas[i].puede_actuar() {
                ResultadoPuerta::Rechazada(RechazoPuerta::NoDisponible)
            } else if inicial.is_none() || puerta.is_none() {
                ResultadoPuerta::Rechazada(RechazoPuerta::ElementoInvalido)
            } else if inicial == Some(&solicitado) {
                ResultadoPuerta::SinCambio
            } else if !self.contacto(antes[i].posicion, puerta.unwrap().posicion) {
                ResultadoPuerta::Rechazada(RechazoPuerta::FueraDeAlcance)
            } else if let Some(ejecutor) = self.sucesos.iter().find_map(|s| match s {
                Suceso::PuertaCambiada {
                    elemento: id,
                    por,
                    estado,
                } if *id == elemento && *estado == solicitado => Some(*por),
                _ => None,
            }) {
                if ejecutor == ficha {
                    ResultadoPuerta::Cambiada
                } else {
                    ResultadoPuerta::Concurrente { ejecutor }
                }
            } else {
                let p = puerta.unwrap().posicion;
                let cruzada=self.sucesos.iter().any(|s|matches!(s,Suceso::Paso{desde,hasta,..} if [desde,hasta].iter().any(|q|q.x==p.x && q.y==p.y && q.z.abs_diff(p.z)<2)));
                let ocupada = self.fichas.iter().any(|f| {
                    f.puede_actuar()
                        && f.posicion.x == p.x
                        && f.posicion.y == p.y
                        && f.posicion.z.abs_diff(p.z) < 2
                });
                ResultadoPuerta::Rechazada(if cruzada {
                    RechazoPuerta::PasoEnCurso
                } else if ocupada {
                    RechazoPuerta::VanoOcupado
                } else {
                    RechazoPuerta::Bloqueada
                })
            };
            self.sucesos.push(Suceso::InteraccionPuerta {
                ficha,
                elemento,
                solicitado,
                resultado,
            });
        }
    }
}
