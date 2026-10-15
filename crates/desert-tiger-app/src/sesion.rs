use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;

use desert_tiger_core::acciones::{
    self, CierreTramo, ClaveEscena, EjecucionAccion, EmisionAccion, ErrorMotor, FinalizacionAccion,
    FragmentoNarrativo, RechazoAccion, RespuestaDecision, SolicitudAccion, SolicitudDecision,
    TramoAccion,
};
use desert_tiger_core::azar::{ErrorFuenteAzar, FuenteAzar};
use desert_tiger_core::{Caracteristica, IdActor, IdLugar, Mundo};
use rand::rngs::SysRng;
use rand::{RngExt, SeedableRng, TryRng};
use rand_chacha::ChaCha12Rng;

use crate::contenido::{
    ClaveTipoTile, ContenidoValidado, CoordenadaMapa, EstadoMapa, RetratoActor,
};
use crate::fase_npc::{FalloFaseNpc, ResumenNpc, ejecutar_fase_npc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfiguracionPartida {
    pub semilla: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoErrorCrearPartida {
    ContenidoInconsistente,
    EntropiaNoDisponible,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCrearPartida {
    pub codigo: CodigoErrorCrearPartida,
    pub detalle: String,
}

impl Display for ErrorCrearPartida {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "no se pudo crear la partida: {}", self.detalle)
    }
}

impl Error for ErrorCrearPartida {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdFragmento(u64);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdDecisionVista(u64);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdOpcionVista(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntencionFrontend {
    ComenzarViaje {
        destino: IdLugar,
    },
    ResponderDecision {
        decision: IdDecisionVista,
        opcion: IdOpcionVista,
    },
    ContinuarPresentacion {
        fragmento: IdFragmento,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorIntencion {
    FaseIncompatible,
    FragmentoNoVigente,
    DecisionNoVigente,
    OpcionNoVigente,
    AccionRechazada(RechazoAccion),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VistaSesion {
    Disponible(VistaMundo),
    PresentandoAccion(VistaFragmento),
    EsperandoDecision(VistaDecision),
    Fatal(VistaFatal),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaMundo {
    pub mapa: VistaMapa,
    pub lugares: Vec<VistaLugar>,
    pub lugar_jugador: IdLugar,
    pub jugador: VistaActor,
    pub companeros: Vec<VistaActor>,
    pub acciones: Vec<AccionDisponible>,
    pub causa_indisponibilidad: Option<CausaIndisponibilidad>,
    pub resumen_npc_reciente: Option<ResumenNpc>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaMapa {
    pub ancho: usize,
    pub alto: usize,
    pub celdas: Vec<ClaveTipoTile>,
    pub tipos_declarados: Vec<ClaveTipoTile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaLugar {
    pub id: IdLugar,
    pub nombre: String,
    pub coordenada_central: CoordenadaMapa,
    pub coordenadas_rotulo: Vec<CoordenadaMapa>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaActor {
    pub id: IdActor,
    pub retrato: RetratoActor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccionDisponible {
    Viajar { destinos: Vec<IdLugar> },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausaIndisponibilidad {
    JugadorMuerto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFragmento {
    pub id: IdFragmento,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaDecision {
    pub id: IdDecisionVista,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub opciones: Vec<VistaOpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaOpcionDecision {
    pub id: IdOpcionVista,
    pub texto: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoFatalSesion {
    Motor,
    Calendario,
    IntegracionNpc,
    AzarNpc,
    CoordinacionNpc,
    EstadoInconsistente,
    ContadorAgotado,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFatal {
    pub codigo: CodigoFatalSesion,
    pub mensaje: String,
}

pub struct Partida {
    mundo: Mundo,
    mapa: EstadoMapa,
    contenido: ContenidoValidado,
    estado: EstadoSesion,
    azar: Box<dyn FuenteAzar>,
    siguiente_fragmento: u64,
    siguiente_decision: u64,
}

enum EstadoSesion {
    Disponible { vista: VistaMundo },
    Presentando(PresentacionPendiente),
    EsperandoDecision(DecisionPendiente),
    Fatal { vista: VistaFatal },
}

struct PresentacionPendiente {
    vista_actual: VistaFragmento,
    fragmentos_restantes: VecDeque<FragmentoNarrativo>,
    cierre: CierrePendiente,
}

enum CierrePendiente {
    Decision {
        ejecucion: EjecucionAccion,
        decision: SolicitudDecision,
    },
    Completada(FinalizacionAccion),
}

struct DecisionPendiente {
    ejecucion: EjecucionAccion,
    decision_interna: acciones::IdDecision,
    vista: VistaDecision,
    opciones: Vec<(IdOpcionVista, acciones::IdOpcionDecision)>,
}

struct AzarChaCha {
    rng: ChaCha12Rng,
}

impl FuenteAzar for AzarChaCha {
    fn elegir_indice(&mut self, limite: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
        Ok(self.rng.random_range(0..limite.get()))
    }
}

impl Partida {
    pub fn nueva(
        contenido: &ContenidoValidado,
        configuracion: ConfiguracionPartida,
    ) -> Result<Self, ErrorCrearPartida> {
        let semilla = match configuracion.semilla {
            Some(semilla) => semilla,
            None => SysRng.try_next_u64().map_err(|error| ErrorCrearPartida {
                codigo: CodigoErrorCrearPartida::EntropiaNoDisponible,
                detalle: format!("no se pudo obtener entropía: {error}"),
            })?,
        };
        Self::con_azar(
            contenido,
            Box::new(AzarChaCha {
                rng: ChaCha12Rng::seed_from_u64(semilla),
            }),
        )
    }

    fn con_azar(
        contenido: &ContenidoValidado,
        azar: Box<dyn FuenteAzar>,
    ) -> Result<Self, ErrorCrearPartida> {
        let (mundo, mapa) = contenido
            .instanciar_estado()
            .map_err(|error| ErrorCrearPartida {
                codigo: CodigoErrorCrearPartida::ContenidoInconsistente,
                detalle: format!("el contenido validado no pudo instanciarse: {error}"),
            })?;
        let mut partida = Self {
            mundo,
            mapa,
            contenido: contenido.clone(),
            estado: EstadoSesion::Fatal {
                vista: VistaFatal {
                    codigo: CodigoFatalSesion::EstadoInconsistente,
                    mensaje: "inicialización incompleta".to_owned(),
                },
            },
            azar,
            siguiente_fragmento: 0,
            siguiente_decision: 0,
        };
        let vista = partida
            .proyectar_mundo(None)
            .map_err(|mensaje| ErrorCrearPartida {
                codigo: CodigoErrorCrearPartida::ContenidoInconsistente,
                detalle: mensaje,
            })?;
        partida.estado = EstadoSesion::Disponible { vista };
        Ok(partida)
    }

    pub fn vista(&self) -> VistaSesion {
        match &self.estado {
            EstadoSesion::Disponible { vista } => VistaSesion::Disponible(vista.clone()),
            EstadoSesion::Presentando(pendiente) => {
                VistaSesion::PresentandoAccion(pendiente.vista_actual.clone())
            }
            EstadoSesion::EsperandoDecision(pendiente) => {
                VistaSesion::EsperandoDecision(pendiente.vista.clone())
            }
            EstadoSesion::Fatal { vista } => VistaSesion::Fatal(vista.clone()),
        }
    }

    pub fn procesar(
        &mut self,
        intencion: IntencionFrontend,
    ) -> Result<VistaSesion, ErrorIntencion> {
        match intencion {
            IntencionFrontend::ComenzarViaje { destino } => self.comenzar_viaje(destino)?,
            IntencionFrontend::ContinuarPresentacion { fragmento } => self.continuar(fragmento)?,
            IntencionFrontend::ResponderDecision { decision, opcion } => {
                self.responder_decision(decision, opcion)?
            }
        }
        Ok(self.vista())
    }

    fn comenzar_viaje(&mut self, destino: IdLugar) -> Result<(), ErrorIntencion> {
        if !matches!(self.estado, EstadoSesion::Disponible { .. }) {
            return Err(ErrorIntencion::FaseIncompatible);
        }
        let solicitud = SolicitudAccion::Viajar {
            actor: self.mundo.actor_jugador(),
            destino,
        };
        let inicio = match acciones::iniciar(&mut self.mundo, solicitud, self.azar.as_mut()) {
            Ok(inicio) => inicio,
            Err(ErrorMotor::Rechazo(rechazo)) => {
                return Err(ErrorIntencion::AccionRechazada(rechazo));
            }
            Err(ErrorMotor::Fatal(_)) => {
                self.fatal(
                    CodigoFatalSesion::Motor,
                    "el motor no pudo iniciar la acción",
                );
                return Ok(());
            }
        };
        self.establecer_tramo(Some(inicio.ejecucion), inicio.tramo);
        Ok(())
    }

    fn continuar(&mut self, fragmento: IdFragmento) -> Result<(), ErrorIntencion> {
        let EstadoSesion::Presentando(actual) = &self.estado else {
            return Err(ErrorIntencion::FaseIncompatible);
        };
        if actual.vista_actual.id != fragmento {
            return Err(ErrorIntencion::FragmentoNoVigente);
        }
        let estado = std::mem::replace(&mut self.estado, estado_transitorio());
        let EstadoSesion::Presentando(mut pendiente) = estado else {
            unreachable!()
        };
        if let Some(siguiente) = pendiente.fragmentos_restantes.pop_front() {
            match self.vista_fragmento(siguiente) {
                Ok(vista) => {
                    pendiente.vista_actual = vista;
                    self.estado = EstadoSesion::Presentando(pendiente);
                }
                Err(()) => self.fatal(
                    CodigoFatalSesion::ContadorAgotado,
                    "se agotó el contador de fragmentos",
                ),
            }
        } else {
            self.liberar_cierre(pendiente.cierre);
        }
        Ok(())
    }

    fn responder_decision(
        &mut self,
        decision: IdDecisionVista,
        opcion: IdOpcionVista,
    ) -> Result<(), ErrorIntencion> {
        let EstadoSesion::EsperandoDecision(actual) = &self.estado else {
            return Err(ErrorIntencion::FaseIncompatible);
        };
        if actual.vista.id != decision {
            return Err(ErrorIntencion::DecisionNoVigente);
        }
        let opcion_motor = actual
            .opciones
            .iter()
            .find(|(visible, _)| *visible == opcion)
            .map(|(_, interna)| *interna)
            .ok_or(ErrorIntencion::OpcionNoVigente)?;
        let estado = std::mem::replace(&mut self.estado, estado_transitorio());
        let EstadoSesion::EsperandoDecision(mut pendiente) = estado else {
            unreachable!()
        };
        let resultado = acciones::responder(
            &mut self.mundo,
            &mut pendiente.ejecucion,
            RespuestaDecision {
                decision: pendiente.decision_interna,
                opcion: opcion_motor,
            },
            self.azar.as_mut(),
        );
        match resultado {
            Ok(tramo) => self.establecer_tramo(Some(pendiente.ejecucion), tramo),
            Err(ErrorMotor::Rechazo(RechazoAccion::DecisionNoVigente { .. })) => {
                self.estado = EstadoSesion::EsperandoDecision(pendiente);
                return Err(ErrorIntencion::DecisionNoVigente);
            }
            Err(ErrorMotor::Rechazo(RechazoAccion::OpcionNoVigente { .. })) => {
                self.estado = EstadoSesion::EsperandoDecision(pendiente);
                return Err(ErrorIntencion::OpcionNoVigente);
            }
            Err(ErrorMotor::Rechazo(rechazo)) => {
                self.estado = EstadoSesion::EsperandoDecision(pendiente);
                return Err(ErrorIntencion::AccionRechazada(rechazo));
            }
            Err(ErrorMotor::Fatal(_)) => self.fatal(
                CodigoFatalSesion::Motor,
                "el motor no pudo resolver la decisión",
            ),
        }
        Ok(())
    }

    fn establecer_tramo(&mut self, ejecucion: Option<EjecucionAccion>, tramo: TramoAccion) {
        let fragmentos: VecDeque<_> = tramo
            .emisiones
            .into_iter()
            .filter_map(|emision| match emision {
                EmisionAccion::Narrativa(fragmento) => Some(fragmento),
                EmisionAccion::ResultadoSemantico(_) => None,
            })
            .collect();
        let cierre = match tramo.cierre {
            CierreTramo::EsperandoDecision(decision) => match ejecucion {
                Some(ejecucion) => CierrePendiente::Decision {
                    ejecucion,
                    decision,
                },
                None => {
                    self.fatal(
                        CodigoFatalSesion::EstadoInconsistente,
                        "falta la ejecución pendiente",
                    );
                    return;
                }
            },
            CierreTramo::Completada(finalizacion) => CierrePendiente::Completada(finalizacion),
        };
        let mut fragmentos = fragmentos;
        if let Some(primero) = fragmentos.pop_front() {
            match self.vista_fragmento(primero) {
                Ok(vista_actual) => {
                    self.estado = EstadoSesion::Presentando(PresentacionPendiente {
                        vista_actual,
                        fragmentos_restantes: fragmentos,
                        cierre,
                    })
                }
                Err(()) => self.fatal(
                    CodigoFatalSesion::ContadorAgotado,
                    "se agotó el contador de fragmentos",
                ),
            }
        } else {
            self.liberar_cierre(cierre);
        }
    }

    fn liberar_cierre(&mut self, cierre: CierrePendiente) {
        match cierre {
            CierrePendiente::Decision {
                ejecucion,
                decision,
            } => match self.vista_decision(&decision) {
                Ok((vista, opciones)) => {
                    self.estado = EstadoSesion::EsperandoDecision(DecisionPendiente {
                        ejecucion,
                        decision_interna: decision.id,
                        vista,
                        opciones,
                    })
                }
                Err(()) => self.fatal(
                    CodigoFatalSesion::ContadorAgotado,
                    "no se pudo representar la decisión",
                ),
            },
            CierrePendiente::Completada(finalizacion) => self.finalizar_accion(finalizacion),
        }
    }

    fn finalizar_accion(&mut self, finalizacion: FinalizacionAccion) {
        if self.mundo.avanzar_fecha(finalizacion.duracion).is_err() {
            self.fatal(CodigoFatalSesion::Calendario, "la fecha no pudo avanzar");
            return;
        }
        let resultado = ejecutar_fase_npc(
            &mut self.mundo,
            self.contenido.politicas(),
            self.contenido.claves(),
            self.azar.as_mut(),
        );
        match resultado {
            Ok(resumen) => match self.proyectar_mundo(resumen) {
                Ok(vista) => self.estado = EstadoSesion::Disponible { vista },
                Err(mensaje) => self.fatal(CodigoFatalSesion::EstadoInconsistente, &mensaje),
            },
            Err(fallo) => {
                let (codigo, mensaje) = fatal_npc(fallo);
                self.fatal(codigo, mensaje);
            }
        }
    }

    fn vista_fragmento(&mut self, fragmento: FragmentoNarrativo) -> Result<VistaFragmento, ()> {
        let id = IdFragmento(self.siguiente_fragmento);
        self.siguiente_fragmento = self.siguiente_fragmento.checked_add(1).ok_or(())?;
        Ok(VistaFragmento {
            id,
            texto: fragmento.texto,
            escena: fragmento.escena,
        })
    }

    fn vista_decision(
        &mut self,
        decision: &SolicitudDecision,
    ) -> Result<
        (
            VistaDecision,
            Vec<(IdOpcionVista, acciones::IdOpcionDecision)>,
        ),
        (),
    > {
        let id = IdDecisionVista(self.siguiente_decision);
        self.siguiente_decision = self.siguiente_decision.checked_add(1).ok_or(())?;
        let mut opciones_vista = Vec::with_capacity(decision.opciones.len());
        let mut correspondencia = Vec::with_capacity(decision.opciones.len());
        for (posicion, opcion) in decision.opciones.iter().enumerate() {
            let visible = IdOpcionVista(u32::try_from(posicion).map_err(|_| ())?);
            opciones_vista.push(VistaOpcionDecision {
                id: visible,
                texto: opcion.texto.clone(),
            });
            correspondencia.push((visible, opcion.id));
        }
        Ok((
            VistaDecision {
                id,
                texto: decision.texto.clone(),
                escena: decision.escena,
                opciones: opciones_vista,
            },
            correspondencia,
        ))
    }

    fn proyectar_mundo(&self, resumen: Option<ResumenNpc>) -> Result<VistaMundo, String> {
        let presentacion = self.contenido.presentacion();
        let lugares = presentacion
            .lugares()
            .iter()
            .map(|lugar| VistaLugar {
                id: lugar.id(),
                nombre: lugar.nombre().to_owned(),
                coordenada_central: lugar.coordenada_central(),
                coordenadas_rotulo: lugar.coordenadas_rotulo().to_vec(),
            })
            .collect();
        let jugador_id = self.mundo.actor_jugador();
        let jugador = proyectar_actor(presentacion, jugador_id)?;
        let companeros = self
            .mundo
            .grupo()
            .integrantes()
            .iter()
            .copied()
            .map(|id| proyectar_actor(presentacion, id))
            .collect::<Result<Vec<_>, _>>()?;
        let muerto =
            Caracteristica::nueva("muerto".to_owned()).map_err(|error| error.to_string())?;
        let actor = self
            .mundo
            .actor(jugador_id)
            .ok_or_else(|| "el jugador no existe".to_owned())?;
        let (acciones, causa_indisponibilidad) = if actor.tiene_caracteristica(&muerto) {
            (Vec::new(), Some(CausaIndisponibilidad::JugadorMuerto))
        } else {
            let destinos = acciones::destinos_disponibles(&self.mundo, jugador_id)
                .map_err(|error| error.to_string())?;
            (vec![AccionDisponible::Viajar { destinos }], None)
        };
        Ok(VistaMundo {
            mapa: VistaMapa {
                ancho: self.mapa.ancho(),
                alto: self.mapa.alto(),
                celdas: self.mapa.celdas().to_vec(),
                tipos_declarados: self.mapa.tipos_declarados().cloned().collect(),
            },
            lugares,
            lugar_jugador: actor.lugar(),
            jugador,
            companeros,
            acciones,
            causa_indisponibilidad,
            resumen_npc_reciente: resumen,
        })
    }

    fn fatal(&mut self, codigo: CodigoFatalSesion, mensaje: &str) {
        self.estado = EstadoSesion::Fatal {
            vista: VistaFatal {
                codigo,
                mensaje: crate::contenido::sanear(mensaje),
            },
        };
    }
}

fn proyectar_actor(
    presentacion: &crate::contenido::CatalogoPresentacion,
    id: IdActor,
) -> Result<VistaActor, String> {
    let actor = presentacion
        .actor(id)
        .ok_or_else(|| "falta presentación de actor".to_owned())?;
    let retrato = actor
        .retrato()
        .cloned()
        .ok_or_else(|| "falta retrato obligatorio".to_owned())?;
    Ok(VistaActor { id, retrato })
}

fn fatal_npc(fallo: FalloFaseNpc) -> (CodigoFatalSesion, &'static str) {
    match fallo {
        FalloFaseNpc::Integracion(error) => {
            let _ = error.actor;
            (
                CodigoFatalSesion::IntegracionNpc,
                "falló la integración de la política NPC",
            )
        }
        FalloFaseNpc::FuenteAzar(error) => {
            let _ = error;
            (CodigoFatalSesion::AzarNpc, "falló el azar de la fase NPC")
        }
        FalloFaseNpc::IndiceAzarFueraDeRango {
            indice,
            limite_exclusivo,
        } => {
            let _ = (indice, limite_exclusivo);
            (CodigoFatalSesion::AzarNpc, "falló el azar de la fase NPC")
        }
        FalloFaseNpc::Motor(fallo) => {
            let _ = fallo;
            (
                CodigoFatalSesion::Motor,
                "falló el motor durante la fase NPC",
            )
        }
        FalloFaseNpc::RespuestaAutomaticaRechazada(rechazo) => {
            let _ = rechazo;
            (
                CodigoFatalSesion::CoordinacionNpc,
                "falló la coordinación de la fase NPC",
            )
        }
        FalloFaseNpc::InvarianteRota => (
            CodigoFatalSesion::CoordinacionNpc,
            "falló la coordinación de la fase NPC",
        ),
    }
}

fn estado_transitorio() -> EstadoSesion {
    EstadoSesion::Fatal {
        vista: VistaFatal {
            codigo: CodigoFatalSesion::EstadoInconsistente,
            mensaje: "transición incompleta".to_owned(),
        },
    }
}

#[cfg(test)]
mod pruebas {
    use std::collections::VecDeque;

    use super::*;
    use crate::{EntradaResumenNpc, cargar_contenido};

    struct AzarProgramado {
        valores: VecDeque<usize>,
        llamadas: usize,
    }

    impl AzarProgramado {
        fn nuevo(valores: impl IntoIterator<Item = usize>) -> Self {
            Self {
                valores: valores.into_iter().collect(),
                llamadas: 0,
            }
        }
    }

    impl FuenteAzar for AzarProgramado {
        fn elegir_indice(
            &mut self,
            _limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            self.llamadas += 1;
            self.valores.pop_front().ok_or(ErrorFuenteAzar)
        }
    }

    fn contenido() -> ContenidoValidado {
        let raiz = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base");
        cargar_contenido(&raiz).expect("fixture válido")
    }

    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID válido")
    }

    #[test]
    fn recorre_el_fixture_completo_y_avanza_solo_por_el_jugador() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(&contenido, Box::new(AzarProgramado::nuevo([0, 0])))
            .expect("partida válida");
        let vista_inicial = partida.vista();
        let VistaSesion::Disponible(inicial) = vista_inicial else {
            panic!("se esperaba el mundo inicial")
        };
        assert_eq!(inicial.lugar_jugador, id_lugar(0));
        assert_eq!(
            inicial.acciones,
            vec![AccionDisponible::Viajar {
                destinos: vec![id_lugar(1)]
            }]
        );

        let primera = partida
            .procesar(IntencionFrontend::ComenzarViaje {
                destino: id_lugar(1),
            })
            .expect("viaje aceptado");
        let VistaSesion::PresentandoAccion(primera) = primera else {
            panic!("se esperaba primera viñeta")
        };
        let segunda = partida
            .procesar(IntencionFrontend::ContinuarPresentacion {
                fragmento: primera.id,
            })
            .expect("continuación aceptada");
        let VistaSesion::PresentandoAccion(segunda) = segunda else {
            panic!("se esperaba viñeta de llegada")
        };
        assert_eq!(partida.mundo.fecha().dia(), 31);
        let finalizada = partida
            .procesar(IntencionFrontend::ContinuarPresentacion {
                fragmento: segunda.id,
            })
            .expect("cierre aceptado");
        let VistaSesion::Disponible(finalizada) = finalizada else {
            panic!("se esperaba volver al mundo")
        };

        assert_eq!(finalizada.lugar_jugador, id_lugar(1));
        assert_eq!(partida.mundo.fecha().mes().numero(), 2);
        assert_eq!(partida.mundo.fecha().dia(), 2);
        assert_eq!(
            partida
                .mundo
                .ubicacion_actor(IdActor::desde_posicion(2).expect("ID válido")),
            Some(id_lugar(0))
        );
        let resumen = finalizada
            .resumen_npc_reciente
            .expect("el viaje NPC produce resumen");
        assert_eq!(
            resumen.entradas(),
            &[EntradaResumenNpc::ViajeCompletado {
                actor: IdActor::desde_posicion(2).expect("ID válido"),
                origen: id_lugar(1),
                destino: id_lugar(0),
            }]
        );
    }

    #[test]
    fn decision_visible_se_corresponde_con_la_opcion_interna() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(&contenido, Box::new(AzarProgramado::nuevo([1, 0])))
            .expect("partida válida");
        let vista = partida
            .procesar(IntencionFrontend::ComenzarViaje {
                destino: id_lugar(1),
            })
            .expect("viaje aceptado");
        let VistaSesion::PresentandoAccion(fragmento) = vista else {
            panic!("se esperaba obstáculo")
        };
        let vista = partida
            .procesar(IntencionFrontend::ContinuarPresentacion {
                fragmento: fragmento.id,
            })
            .expect("continuación aceptada");
        let VistaSesion::EsperandoDecision(decision) = vista else {
            panic!("se esperaba decisión")
        };
        assert_eq!(decision.opciones.len(), 2);
        assert_eq!(
            partida.procesar(IntencionFrontend::ResponderDecision {
                decision: IdDecisionVista(decision.id.0 + 1),
                opcion: decision.opciones[0].id,
            }),
            Err(ErrorIntencion::DecisionNoVigente)
        );
        let vista = partida
            .procesar(IntencionFrontend::ResponderDecision {
                decision: decision.id,
                opcion: decision.opciones[0].id,
            })
            .expect("opción vigente");
        assert!(matches!(vista, VistaSesion::PresentandoAccion(_)));
    }

    #[test]
    fn vista_es_una_instantanea_estable() {
        let contenido = contenido();
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) })
            .expect("partida válida");
        assert_eq!(partida.vista(), partida.vista());
    }
}
