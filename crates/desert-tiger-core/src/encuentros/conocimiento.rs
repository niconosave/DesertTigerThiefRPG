//! Consultas de navegación limitadas al conocimiento disponible.
use super::*;

#[derive(Clone, Debug)]
pub struct ContextoNavegacion {
    pub revision_geometria: u64,
    pub revision_conocimiento: u64,
    terreno: BTreeMap<Posicion, Celda>,
    elementos: BTreeMap<IdElementoLocal, ElementoLocal>,
    conexiones: Vec<ConexionLocal>,
    participantes: Vec<IdFicha>,
}
impl ContextoNavegacion {
    pub fn voxel(&self, p: Posicion) -> Option<Celda> {
        self.terreno.get(&p).copied()
    }
    pub fn elemento(&self, id: IdElementoLocal) -> Option<&ElementoLocal> {
        self.elementos.get(&id)
    }
    pub fn conexiones(&self) -> &[ConexionLocal] {
        &self.conexiones
    }
    pub fn participantes(&self) -> &[IdFicha] {
        &self.participantes
    }
}
impl Encuentro {
    pub fn elementos_recordados(&self) -> impl Iterator<Item = &ElementoLocal> {
        self.memoria_elementos.values()
    }
    pub fn punto_observable(&self, p: Posicion) -> bool {
        self.espacio_en(p)
            .is_some_and(|id| self.espacio_observable(id))
    }

    pub(super) fn participante_observable_por(&self, observador: &Ficha, blanco: &Ficha) -> bool {
        if blanco.salio {
            return false;
        }
        if observador.aliado == blanco.aliado {
            return true;
        }
        if observador.aliado {
            return self.ficha_observable(blanco);
        }
        let Some(destino) = self.espacio_en(blanco.posicion) else {
            return false;
        };
        self.fichas
            .iter()
            .filter(|a| !a.aliado && !a.salio && !a.muerto())
            .filter_map(|a| self.espacio_en(a.posicion))
            .any(|origen| {
                origen == destino
                    || self.conexiones_desde(origen).any(|c| {
                        !c.salida
                            && self.conexion_abierta(c)
                            && (c.desde == destino || c.hasta == destino)
                    })
            })
    }

    pub fn contexto_navegacion(
        &self,
        participante: IdFicha,
    ) -> Result<ContextoNavegacion, ErrorGeometria> {
        let ficha = self
            .ficha(participante)
            .ok_or(ErrorGeometria::ReferenciaInvalida)?;
        let propio = ficha.aliado;
        let terreno = if propio {
            self.memoria_terreno.clone()
        } else {
            self.voxeles().collect()
        };
        let elementos = if propio {
            self.memoria_elementos.clone()
        } else {
            self.elementos.iter().cloned().map(|e| (e.id, e)).collect()
        };
        let conexiones = self
            .contexto
            .conexiones
            .iter()
            .filter(|c| {
                !propio
                    || self.espacios_descubiertos.contains(&c.desde)
                    || self.espacios_descubiertos.contains(&c.hasta)
            })
            .cloned()
            .collect();
        let participantes = self
            .fichas
            .iter()
            .filter(|f| {
                !f.salio
                    && (f.aliado == ficha.aliado
                        || if propio {
                            self.ficha_observable(f)
                        } else {
                            let destino = self.espacio_en(f.posicion);
                            self.fichas
                                .iter()
                                .filter(|a| !a.aliado && !a.salio && !a.muerto())
                                .filter_map(|a| self.espacio_en(a.posicion))
                                .any(|origen| {
                                    Some(origen) == destino
                                        || self.conexiones_desde(origen).any(|c| {
                                            !c.salida
                                                && self.conexion_abierta(c)
                                                && (Some(c.desde) == destino
                                                    || Some(c.hasta) == destino)
                                        })
                                })
                        })
            })
            .map(|f| f.id)
            .collect();
        Ok(ContextoNavegacion {
            revision_geometria: self.contexto.revision_geometria,
            revision_conocimiento: self.contexto.revision_conocimiento,
            terreno,
            elementos,
            conexiones,
            participantes,
        })
    }
}
