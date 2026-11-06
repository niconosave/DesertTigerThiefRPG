use super::*;
use crate::eventos::coordinador::Salida;
use desert_tiger_core::acciones::intervenciones::{
    self as motor, ClasePunto, ContextoIntervencion, IdIntervencion, ResolucionPunto, SalidaMotor,
};

pub(super) enum Continuacion {
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
    pub(super) fn conducir_eventos(&mut self) -> Result<(), String> {
        let raiz = &self.ejecucion;
        let activa = matches!(self.continuacion_evento, Some(Continuacion::Motor(_)));
        let camp = matches!(self.continuacion_evento,Some(Continuacion::Restaurar(ref v)) if matches!(**v,VistaSesion::Acampando(_)));
        let salida = self.eventos.conducir(
            &mut self.mundo,
            self.contenido.claves(),
            self.contenido.eventos(),
            |m, interrumpir| {
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
        let resumen = self.mundo_visible.resumen_npc_reciente.clone();
        self.mundo_visible = proyectar_mundo(
            &self.mundo,
            &self.mapa,
            &self.contenido,
            &self.vision,
            resumen,
        )?;
        match salida {
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
                        self.vista = *v;
                        Ok(())
                    }
                    Continuacion::Cerrar(t) => {
                        self.concluir_accion_directa(t)
                            .map_err(FalloCierreAccion::detalle)?;
                        self.abrir_eventos(
                            self.contexto_sesion(ClasePunto::FaseNpcTerminada),
                            Continuacion::DespuesNpc,
                        )
                    }
                    Continuacion::DespuesNpc => self.abrir_eventos(
                        self.contexto_sesion(ClasePunto::Disponible),
                        Continuacion::Disponible,
                    ),
                    Continuacion::Disponible => {
                        self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
                        Ok(())
                    }
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
