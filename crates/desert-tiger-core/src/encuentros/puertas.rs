//! Interacción manual y resolución de puertas en un pulso simultáneo.
use super::*;
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
        for (f, intento) in antes.iter().zip(intentos) {
            if matches!(intento, Intento::Abrir(_) | Intento::Cerrar(_)) {
                self.interacciones_puerta.remove(&f.id);
            }
        }
    }
}
