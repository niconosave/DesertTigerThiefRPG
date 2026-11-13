use super::*;

pub(super) struct EsperaOrdenes {
    pub id: IdEspera,
    pub colectivo: IdColectivo,
    pub intervalo: u64,
    pub cancelar: bool,
}
impl Partida {
    pub(super) fn procesar_espera(
        &mut self,
        i: IntencionFrontend,
    ) -> Result<VistaSesion, ErrorIntencion> {
        match i {
            IntencionFrontend::EsperarOrdenes { campamento } => {
                let VistaSesion::Acampando(camp) = &self.vista else {
                    return Err(ErrorIntencion::FaseIncompatible);
                };
                if camp.id != campamento {
                    return Err(ErrorIntencion::CampamentoNoVigente);
                }
                if !camp.puede_esperar_ordenes || self.acampada.is_none() {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                let colectivo = self
                    .mundo
                    .colectivo_de(self.mundo.actor_jugador())
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                if self.mundo_visible.causa_indisponibilidad.is_some() {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                self.ordinal = self
                    .ordinal
                    .checked_add(1)
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                self.espera = Some(EsperaOrdenes {
                    id: IdEspera(self.ordinal),
                    colectivo,
                    intervalo: 0,
                    cancelar: false,
                });
                self.eventos.actividades.clear();
                self.publicar_espera();
            }
            IntencionFrontend::AvanzarEspera { espera, intervalo } => {
                if !matches!(self.vista, VistaSesion::EsperandoOrdenes(_)) {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                let e = self
                    .espera
                    .as_mut()
                    .filter(|e| e.id == espera && e.intervalo == intervalo && !e.cancelar)
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                e.intervalo = e
                    .intervalo
                    .checked_add(1)
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                self.inicio = InstanteAccion {
                    fecha: self.mundo.fecha(),
                    fraccion: self.resto,
                };
                let tiempo = desert_tiger_core::TiempoAccion {
                    duracion: Duracion::desde_dias(1),
                    fraccion: FraccionDia::CERO,
                };
                if let Err(error) = self.concluir_accion(tiempo) {
                    self.fatal(error.codigo(), error.detalle());
                }
            }
            IntencionFrontend::CancelarEspera { espera } => {
                let e = self
                    .espera
                    .as_mut()
                    .filter(|e| e.id == espera)
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                e.cancelar = true;
                if matches!(self.vista, VistaSesion::EsperandoOrdenes(_)) {
                    self.terminar_cierre()
                        .map_err(|_| ErrorIntencion::FaseIncompatible)?;
                }
            }
            IntencionFrontend::ContinuarCierre { token } => {
                if !matches!(self.vista,VistaSesion::ProcesandoCierre{token:t} if t==token) {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                if let Err(error) = self.conducir_eventos() {
                    self.fatal(CodigoFatalSesion::EventoNarrativo, error);
                }
            }
            _ => return Err(ErrorIntencion::FaseIncompatible),
        }
        Ok(self.vista())
    }
    pub fn espera_activa(&self) -> Option<IdEspera> {
        self.espera.as_ref().map(|e| e.id)
    }
    fn publicar_espera(&mut self) {
        let e = self.espera.as_ref().expect("espera vigente");
        self.vista = VistaSesion::EsperandoOrdenes(VistaEsperaOrdenes {
            id: e.id,
            intervalo: e.intervalo,
        });
    }
    pub(super) fn reconstruir_campamento(&mut self, id: IdCampamento) -> Result<(), String> {
        if self.acampada.is_none() {
            return Err("campamento sin acampada".into());
        }
        let m = &self.mundo_visible;
        self.vista = VistaSesion::Acampando(VistaCampamento {
            id,
            descanso_cubierto: self
                .acampada
                .as_ref()
                .is_some_and(|a| a.descanso_cubierto()),
            lugar: m.lugar_jugador,
            colectivo: m.colectivo.clone(),
            puede_esperar_ordenes: m.colectivo.is_some() && m.causa_indisponibilidad.is_none(),
            integrantes: m
                .companeros
                .iter()
                .chain(std::iter::once(&m.jugador))
                .map(|a| IntegranteCampamento {
                    actor: a.id,
                    nombre: a.nombre.clone(),
                })
                .collect(),
        });
        Ok(())
    }
    pub(super) fn terminar_cierre(&mut self) -> Result<(), String> {
        self.cierre = None;
        if let Some(e) = self.espera.as_ref() {
            let compatible = self.mundo.colectivo_de(self.mundo.actor_jugador())
                == Some(e.colectivo)
                && self.mundo_visible.causa_indisponibilidad.is_none();
            if compatible && e.cancelar {
                self.espera = None;
                self.ordinal = self.ordinal.checked_add(1).ok_or("IDs agotados")?;
                self.reconstruir_campamento(IdCampamento(self.ordinal))?;
                return Ok(());
            }
            if compatible && !self.eventos.actividades.contains(&e.colectivo) {
                self.publicar_espera();
                return Ok(());
            }
            self.espera = None;
            self.acampada = None;
        }
        self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
        Ok(())
    }
}
