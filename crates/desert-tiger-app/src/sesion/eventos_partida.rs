use super::*;
use crate::eventos::coordinador::Salida;
use desert_tiger_core::acciones::intervenciones::{
    self as motor, ClasePunto, ContextoIntervencion, IdIntervencion, ResolucionPunto, SalidaMotor,
};

pub(super) enum Continuacion {
    Inicial,
    AntesNpc,
    Motor(IdIntervencion),
    Restaurar(Box<VistaSesion>),
    Cerrar(desert_tiger_core::TiempoAccion),
    DespuesNpc,
    Disponible,
}
impl Partida {
    pub(super) fn contexto_sesion(&self, clase: ClasePunto) -> ContextoIntervencion {
        ContextoIntervencion {
            clase,
            actor: self.mundo.actor_jugador(),
            accion: None,
            objetivo: None,
            lugar: self.mundo.ubicacion_grupo(),
            participantes: std::iter::once(self.mundo.actor_jugador())
                .chain(self.mundo.grupo().integrantes().iter().copied())
                .collect(),
            instante: InstanteAccion {
                fecha: self.mundo.fecha(),
                fraccion: self.resto,
            },
        }
    }
    pub(super) fn abrir_eventos(
        &mut self,
        ctx: ContextoIntervencion,
        continuar: Continuacion,
    ) -> Result<(), String> {
        self.eventos.abrir(ctx)?;
        self.continuacion_evento = Some(continuar);
        self.conducir_eventos()
    }
    pub(super) fn abrir_fechas(
        &mut self,
        inicio: InstanteAccion,
        inclusiva: bool,
        continuar: Continuacion,
    ) -> Result<(), String> {
        self.eventos.abrir_fechas(
            self.contexto_sesion(ClasePunto::Disponible),
            inicio,
            inclusiva,
            self.contenido.eventos(),
        )?;
        self.continuacion_evento = Some(continuar);
        self.conducir_eventos()
    }
    pub(super) fn conducir_eventos(&mut self) -> Result<(), String> {
        let raiz = &self.ejecucion;
        let integrantes_antes = self.mundo.grupo().integrantes().to_vec();
        let ubicacion_directa = self
            .mundo
            .actor(self.mundo.actor_jugador())
            .ok_or("jugador ausente")?
            .ubicacion();
        let activa = matches!(self.continuacion_evento, Some(Continuacion::Motor(_)));
        let camp = matches!(self.continuacion_evento,Some(Continuacion::Restaurar(ref v)) if matches!(**v,VistaSesion::Acampando(_)));
        let salida = self.eventos.conducir(
            &mut self.mundo,
            self.contenido.claves(),
            self.contenido.eventos(),
            |m, interrumpir| {
                if activa && !interrumpir && m.grupo().integrantes() != integrantes_antes {
                    return Err("el evento cambia el grupo de un viaje pendiente".into());
                }
                if activa
                    && m.actor(m.actor_jugador())
                        .is_none_or(|a| a.ubicacion() != ubicacion_directa)
                {
                    return Err(
                        "no se cambia pertenencia del Grupo durante un Viajar suspendido".into(),
                    );
                }
                if activa
                    && !interrumpir
                    && let Some(e) = raiz
                {
                    e.validar_continuacion(m).map_err(|e| e.to_string())?;
                }
                if camp
                    && m.actor(m.actor_jugador())
                        .is_some_and(|a| a.caracteristicas().any(|c| c.como_str() == "muerto"))
                {
                    return Err("el evento invalida el campamento preparado".into());
                }
                Ok(())
            },
        )?;
        for lugar in std::mem::take(&mut self.eventos.posiciones_observadas) {
            let centro = self
                .contenido
                .presentacion()
                .lugar(lugar)
                .and_then(|p| p.coordenada_central())
                .ok_or("Lugar visitado sin coordenadas")?;
            self.vision.observar(&self.mapa, centro);
        }
        if self.mundo_visible.lugar_jugador != self.mundo.ubicacion_grupo() {
            let centro = self
                .contenido
                .presentacion()
                .lugar(self.mundo.ubicacion_grupo())
                .and_then(|p| p.coordenada_central())
                .ok_or("Lugar sin coordenadas")?;
            self.vision.observar(&self.mapa, centro);
        }
        let resumen = self.mundo_visible.resumen_npc_reciente.clone();
        self.mundo_visible = proyectar_mundo(
            &self.mundo,
            &self.mapa,
            &self.contenido,
            &self.vision,
            resumen,
        )?;
        match salida {
            Salida::Pendiente => {
                self.ordinal = self.ordinal.checked_add(1).ok_or("IDs agotados")?;
                self.vista = VistaSesion::ProcesandoCierre {
                    token: IdContinuacionCierre(self.ordinal),
                };
                Ok(())
            }
            Salida::Vista(v) => {
                self.mundo_visible
                    .mostrar_expresion_grupo(v.expresion.as_deref());
                self.vista = if v.opciones.is_empty() {
                    VistaSesion::PresentandoEvento(*v)
                } else {
                    VistaSesion::EsperandoDecisionEvento(*v)
                };
                Ok(())
            }
            Salida::Fin {
                omitir,
                interrumpir,
            } => {
                let continuar = self
                    .continuacion_evento
                    .take()
                    .ok_or("continuación narrativa ausente")?;
                match continuar {
                    Continuacion::Inicial => self.abrir_eventos(
                        self.contexto_sesion(ClasePunto::Disponible),
                        Continuacion::Disponible,
                    ),
                    Continuacion::AntesNpc => {
                        self.ejecutar_npc_cierre()
                            .map_err(FalloCierreAccion::detalle)?;
                        self.abrir_eventos(
                            self.contexto_sesion(ClasePunto::FaseNpcTerminada),
                            Continuacion::DespuesNpc,
                        )
                    }
                    Continuacion::Motor(id) => {
                        let e = self.ejecucion.as_mut().ok_or("viaje suspendido ausente")?;
                        let r = if interrumpir {
                            ResolucionPunto::Interrumpir
                        } else if omitir {
                            ResolucionPunto::OmitirRelato
                        } else {
                            ResolucionPunto::EjecutarGenerico
                        };
                        let salida = motor::resolver_punto(
                            &mut self.mundo,
                            e,
                            id,
                            r,
                            &ContextoAccion {
                                inicio: self.inicio,
                                rumbos: &self.rumbos,
                            },
                            self.azar.as_mut(),
                        )
                        .map_err(|e| e.to_string())?;
                        self.publicar(salida)
                    }
                    Continuacion::Restaurar(v) => {
                        if let VistaSesion::Acampando(camp) = *v {
                            self.reconstruir_campamento(camp.id)
                        } else {
                            self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
                            Ok(())
                        }
                    }
                    Continuacion::Cerrar(t) => {
                        self.concluir_accion_directa(t)
                            .map_err(FalloCierreAccion::detalle)?;
                        self.abrir_fechas(self.inicio, false, Continuacion::AntesNpc)
                    }
                    Continuacion::DespuesNpc => self.abrir_eventos(
                        self.contexto_sesion(ClasePunto::Disponible),
                        Continuacion::Disponible,
                    ),
                    Continuacion::Disponible => self.terminar_cierre(),
                }
            }
        }
    }
    pub(super) fn publicar(&mut self, salida: SalidaMotor) -> Result<(), String> {
        match salida {
            SalidaMotor::Punto(p) => self.abrir_eventos(p.contexto, Continuacion::Motor(p.id)),
            SalidaMotor::Paso(paso) => self.publicar_generico(*paso),
        }
    }
}
