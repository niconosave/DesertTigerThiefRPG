//! Dirección y contabilidad del cierre; la geometría no decide los objetivos.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausaFin {
    SalidaResuelta,
    RetiradaResuelta,
    Captura,
    Derrota,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultadoFicha {
    Salio,
    Transportado(IdFicha),
    Muerto,
    Capturado(IdFicha),
    /// Permanece en el lugar mundial donde se inició el encuentro.
    Permanece(crate::IdLugar),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultadoBien {
    Extraido(IdFicha),
    Permanece(ExtremoRelacion),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultadoEncuentro {
    pub causa: CausaFin,
    pub objetivo_cumplido: bool,
    pub fichas: BTreeMap<IdFicha, ResultadoFicha>,
    pub bienes: BTreeMap<IdObjeto, ResultadoBien>,
}
impl Encuentro {
    pub fn pendientes_evacuacion(&self) -> Vec<IdFicha> {
        self.fichas
            .iter()
            .filter(|f| {
                f.aliado
                    && !f.cautivo
                    && !f.muerto()
                    && !f.salio
                    && f.golpes == 2
                    && f.transportado_por.is_none()
                    && !self.abandonados_confirmados.contains(&f.id)
            })
            .map(|f| f.id)
            .collect()
    }
    /// Respuesta explícita: no ejecuta un pulso, no tira dados ni mueve fichas.
    pub fn responder_evacuacion(&mut self, abandonar: bool) -> Result<(), ErrorEncuentro> {
        if self.decision_evacuacion.is_empty() || self.resultado.is_some() {
            return Err(ErrorEncuentro::EstadoIncompatible {
                operacion: "responder a la evacuación",
            });
        }
        if abandonar {
            self.abandonados_confirmados
                .extend(self.decision_evacuacion.iter().copied());
        }
        self.decision_evacuacion.clear();
        self.reconsiderar(true);
        Ok(())
    }
    fn capturas_posibles(&self) -> BTreeMap<IdFicha, IdFicha> {
        let rutas: Vec<_> = self
            .fichas
            .iter()
            .enumerate()
            .filter(|(_, f)| !f.aliado && f.puede_actuar())
            .map(|(i, f)| (f.id, self.mapa_rutas(i)))
            .collect();
        let mut capturas = BTreeMap::new();
        for f in self
            .fichas
            .iter()
            .filter(|f| f.aliado && !f.salio && !f.muerto() && !f.cautivo)
        {
            if let Some((_, captor)) = rutas
                .iter()
                .filter_map(|(id, rutas)| {
                    rutas
                        .iter()
                        .filter(|(p, _)| self.contacto(**p, f.posicion))
                        .map(|(_, &(coste, _))| coste)
                        .min()
                        .map(|coste| (coste, *id))
                })
                .min()
            {
                capturas.insert(f.id, captor);
            }
        }
        capturas
    }
    pub(super) fn evaluar_conclusion(&mut self) -> Result<(), ErrorEncuentro> {
        let libres = self.fichas.iter().any(|f| f.aliado && f.puede_actuar());
        if libres {
            return Ok(());
        }
        let hay_fuera = self
            .fichas
            .iter()
            .any(|f| f.aliado && f.salio && !f.muerto());
        if hay_fuera {
            let pendientes = self.pendientes_evacuacion();
            if !pendientes.is_empty() {
                self.decision_evacuacion = pendientes;
                return Ok(());
            }
        }
        self.capturas = self.capturas_posibles();
        let salieron = self
            .fichas
            .iter()
            .any(|f| f.aliado && f.salio && !f.muerto());
        let logro = if self.rescate || self.botin.is_some() {
            self.extraccion_fuera()
        } else {
            self.objetivo_muerto()
        };
        let causa = if salieron {
            if logro {
                CausaFin::SalidaResuelta
            } else {
                CausaFin::RetiradaResuelta
            }
        } else if !self.capturas.is_empty() {
            CausaFin::Captura
        } else {
            CausaFin::Derrota
        };
        let fichas = self
            .fichas
            .iter()
            .map(|f| {
                let r = if f.muerto() {
                    ResultadoFicha::Muerto
                } else if f.salio {
                    self.evacuados
                        .get(&f.id)
                        .copied()
                        .map(ResultadoFicha::Transportado)
                        .unwrap_or(ResultadoFicha::Salio)
                } else if let Some(captor) = self.capturas.get(&f.id) {
                    ResultadoFicha::Capturado(*captor)
                } else {
                    ResultadoFicha::Permanece(self.lugar)
                };
                (f.id, r)
            })
            .collect();
        let mut bienes = BTreeMap::new();
        if let Some(b) = &self.botin {
            bienes.insert(
                b.objeto,
                if b.fuera {
                    ResultadoBien::Extraido(b.portador.ok_or("tesoro extraído sin portador")?)
                } else {
                    if b.portador.is_some() {
                        return Err("tesoro todavía en transporte local".into());
                    }
                    ResultadoBien::Permanece(b.ubicacion_inicial)
                },
            );
        }
        let resultado = ResultadoEncuentro {
            causa,
            objetivo_cumplido: logro,
            fichas,
            bienes,
        };
        self.validar_conclusion(&resultado)?;
        // Desenlace es únicamente la etiqueta resumida de presentación.
        self.desenlace = Some(match causa {
            CausaFin::SalidaResuelta => Desenlace::Exito,
            CausaFin::RetiradaResuelta => Desenlace::Retirada,
            CausaFin::Captura => Desenlace::Captura {
                captor: self
                    .ficha(*self.capturas.values().next().ok_or("captura vacía")?)
                    .ok_or("captor ausente")?
                    .actor,
            },
            CausaFin::Derrota => Desenlace::Derrota,
        });
        self.resultado = Some(resultado);
        Ok(())
    }
    pub fn validar_conclusion(&self, r: &ResultadoEncuentro) -> Result<(), ErrorEncuentro> {
        if r.fichas.len() != self.fichas.len()
            || self.fichas.iter().any(|f| !r.fichas.contains_key(&f.id))
        {
            return Err("conclusión sin contabilidad total de fichas".into());
        }
        if self.fichas.iter().any(|f| f.aliado && f.puede_actuar()) {
            return Err("aún hay participantes activos dentro".into());
        }
        if self.capturas != self.capturas_posibles() {
            return Err("captores sin acceso vigente".into());
        }
        for f in &self.fichas {
            let valido = match r.fichas[&f.id] {
                ResultadoFicha::Muerto => f.muerto(),
                ResultadoFicha::Salio => {
                    f.salio && !f.muerto() && !self.evacuados.contains_key(&f.id)
                }
                ResultadoFicha::Transportado(por) => {
                    f.salio
                        && !f.muerto()
                        && self.evacuados.get(&f.id) == Some(&por)
                        && self.ficha(por).is_some_and(|p| p.salio)
                }
                ResultadoFicha::Capturado(por) => {
                    !f.muerto()
                        && !f.salio
                        && self.capturas.get(&f.id) == Some(&por)
                        && self
                            .ficha(por)
                            .is_some_and(|p| !p.aliado && p.puede_actuar())
                }
                ResultadoFicha::Permanece(lugar) => {
                    lugar == self.lugar
                        && !f.muerto()
                        && !f.salio
                        && !self.capturas.contains_key(&f.id)
                }
            };
            if !valido {
                return Err("resultado de ficha incompatible con su estado".into());
            }
        }
        match &self.botin {
            Some(b) if r.bienes.len() == 1 => match r.bienes.get(&b.objeto) {
                Some(ResultadoBien::Extraido(por))
                    if b.fuera
                        && b.portador == Some(*por)
                        && self.ficha(*por).is_some_and(|f| f.salio) => {}
                Some(ResultadoBien::Permanece(donde))
                    if !b.fuera && b.portador.is_none() && *donde == b.ubicacion_inicial => {}
                _ => return Err("resultado de bien incompatible".into()),
            },
            None if r.bienes.is_empty() => {}
            _ => return Err("conclusión sin contabilidad total de bienes".into()),
        }
        let salieron = self
            .fichas
            .iter()
            .any(|f| f.aliado && f.salio && !f.muerto());
        let logro = if self.rescate || self.botin.is_some() {
            self.extraccion_fuera()
        } else {
            self.objetivo_muerto()
        };
        let valida = match r.causa {
            CausaFin::SalidaResuelta => salieron && logro,
            CausaFin::RetiradaResuelta => salieron && !logro,
            CausaFin::Captura => !salieron && !self.capturas.is_empty(),
            CausaFin::Derrota => !salieron && self.capturas.is_empty(),
        };
        if !valida || r.objetivo_cumplido != logro {
            return Err("causa de conclusión incompatible".into());
        }
        if salieron
            && self.fichas.iter().any(|f| {
                f.aliado
                    && !f.cautivo
                    && !f.muerto()
                    && !f.salio
                    && !self.abandonados_confirmados.contains(&f.id)
            })
        {
            return Err("evacuación incompleta sin confirmación".into());
        }
        Ok(())
    }
}
