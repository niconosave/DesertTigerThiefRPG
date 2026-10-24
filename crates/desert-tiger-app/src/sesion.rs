use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;

use desert_tiger_core::acciones::{
    self, CierreTramo, ClaveEscena, EjecucionAccion, EmisionAccion, ErrorMotor, FinalizacionAccion,
    FragmentoNarrativo, RechazoAccion, RegistroProyecto, RespuestaDecision, RolPresentacion,
    SolicitudAccion, SolicitudDecision, TramoAccion, TrazaAccionParticular,
};
use desert_tiger_core::acciones::{
    FuenteMedio, MedioAcuaticoProyecto, MedioTerrestreProyecto, PropuestaViaje,
};
use desert_tiger_core::azar::{ErrorFuenteAzar, FuenteAzar};
use desert_tiger_core::{Caracteristica, Duracion, Fecha, IdActor, IdLugar, Mundo};
use rand::rngs::SysRng;
use rand::{RngExt, SeedableRng, TryRng};
use rand_chacha::ChaCha12Rng;
use unicode_width::UnicodeWidthStr;

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

fn nombre_desde_clave(clave: &str) -> String {
    let mut resultado = String::new();
    for (indice, palabra) in clave.split('_').enumerate() {
        if indice > 0 {
            resultado.push(' ');
        }
        let mut caracteres = palabra.chars();
        if let Some(inicial) = caracteres.next() {
            resultado.extend(inicial.to_uppercase());
            resultado.extend(caracteres);
        }
    }
    resultado
}

fn nombre_medio_acuatico(medio: MedioAcuaticoProyecto) -> &'static str {
    match medio {
        MedioAcuaticoProyecto::Ninguno => "medio acuático",
        MedioAcuaticoProyecto::Bote => "bote",
        MedioAcuaticoProyecto::Barco => "barco",
    }
}

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
    pub pronosticos_viaje: Vec<VistaPronosticoViaje>,
    pub causa_indisponibilidad: Option<CausaIndisponibilidad>,
    pub resumen_npc_reciente: Option<ResumenNpc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VistaPronosticoViaje {
    pub destino: IdLugar,
    pub dias_minimos: u32,
    pub dias_maximos: u32,
    pub exposicion_conocida: u32,
    pub hay_incertidumbre: bool,
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
    pub titulo: String,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub rol: RolPresentacion,
    pub proyecto: Option<RegistroProyecto>,
    pub progreso: VistaProgresoViaje,
    pub detalle: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaProgresoViaje {
    pub lugar_confirmado: IdLugar,
    pub nombre_lugar_confirmado: String,
    pub siguiente_lugar: Option<IdLugar>,
    pub nombre_siguiente_lugar: Option<String>,
    pub destino_final: IdLugar,
    pub nombre_destino_final: String,
    pub duracion_transcurrida: Duracion,
    pub fecha_proyectada: Fecha,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaDecision {
    pub id: IdDecisionVista,
    pub titulo: String,
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub rol: RolPresentacion,
    pub proyecto: Option<RegistroProyecto>,
    pub progreso: VistaProgresoViaje,
    pub opciones: Vec<VistaOpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaOpcionDecision {
    pub id: IdOpcionVista,
    pub texto: String,
    pub detalle: Option<String>,
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
        ejecucion: Box<EjecucionAccion>,
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
                    ejecucion: Box::new(ejecucion),
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
                        ejecucion: *ejecucion,
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
            titulo: fragmento.titulo,
            texto: fragmento.texto,
            escena: fragmento.escena,
            rol: fragmento.rol,
            proyecto: fragmento.proyecto,
            progreso: VistaProgresoViaje {
                lugar_confirmado: fragmento.progreso.lugar_confirmado,
                nombre_lugar_confirmado: self
                    .nombre_lugar(fragmento.progreso.lugar_confirmado)
                    .ok_or(())?,
                siguiente_lugar: fragmento.progreso.siguiente_lugar,
                nombre_siguiente_lugar: fragmento
                    .progreso
                    .siguiente_lugar
                    .map(|id| self.nombre_lugar(id).ok_or(()))
                    .transpose()?,
                destino_final: fragmento.progreso.destino_final,
                nombre_destino_final: self
                    .nombre_lugar(fragmento.progreso.destino_final)
                    .ok_or(())?,
                duracion_transcurrida: fragmento.progreso.duracion_transcurrida,
                fecha_proyectada: fragmento.progreso.fecha_proyectada,
            },
            detalle: fragmento
                .momentos
                .into_iter()
                .map(|momento| formatear_traza(momento.id, momento.subobjetivo, &momento.traza))
                .collect(),
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
            if opcion.texto.is_empty() || opcion.texto.width() > 40 {
                return Err(());
            }
            let visible = IdOpcionVista(u32::try_from(posicion).map_err(|_| ())?);
            opciones_vista.push(VistaOpcionDecision {
                id: visible,
                texto: opcion.texto.clone(),
                detalle: opcion
                    .propuesta_viaje
                    .as_ref()
                    .map(|propuesta| self.detalle_propuesta(propuesta))
                    .or_else(|| opcion.detalle.clone()),
            });
            correspondencia.push((visible, opcion.id));
        }
        Ok((
            VistaDecision {
                id,
                titulo: "Decisión del viaje".to_owned(),
                texto: decision.texto.clone(),
                escena: decision.escena,
                rol: decision.rol,
                proyecto: decision.proyecto,
                progreso: VistaProgresoViaje {
                    lugar_confirmado: decision.progreso.lugar_confirmado,
                    nombre_lugar_confirmado: self
                        .nombre_lugar(decision.progreso.lugar_confirmado)
                        .ok_or(())?,
                    siguiente_lugar: decision.progreso.siguiente_lugar,
                    nombre_siguiente_lugar: decision
                        .progreso
                        .siguiente_lugar
                        .map(|id| self.nombre_lugar(id).ok_or(()))
                        .transpose()?,
                    destino_final: decision.progreso.destino_final,
                    nombre_destino_final: self
                        .nombre_lugar(decision.progreso.destino_final)
                        .ok_or(())?,
                    duracion_transcurrida: decision.progreso.duracion_transcurrida,
                    fecha_proyectada: decision.progreso.fecha_proyectada,
                },
                opciones: opciones_vista,
            },
            correspondencia,
        ))
    }

    fn nombre_lugar(&self, id: IdLugar) -> Option<String> {
        self.contenido
            .presentacion()
            .lugar(id)
            .map(|lugar| lugar.nombre().to_owned())
    }

    fn detalle_propuesta(&self, propuesta: &PropuestaViaje) -> String {
        let nombre_actor = |id| {
            self.contenido
                .claves()
                .clave_actor(id)
                .map(nombre_desde_clave)
                .unwrap_or_else(|| "un viajero".to_owned())
        };
        let nombre_lugar = |id| {
            self.nombre_lugar(id)
                .or_else(|| {
                    self.contenido
                        .claves()
                        .clave_lugar(id)
                        .map(nombre_desde_clave)
                })
                .unwrap_or_else(|| "un lugar conocido".to_owned())
        };
        let terrestre = match propuesta.medio_terrestre {
            MedioTerrestreProyecto::APie => {
                "La marcha a pie depende del esfuerzo ordinario del grupo".to_owned()
            }
            MedioTerrestreProyecto::Camellos => match propuesta.fuente_terrestre {
                FuenteMedio::Actor(actor) => {
                    format!("{} aporta los camellos", nombre_actor(actor))
                }
                FuenteMedio::Lugar(lugar) => {
                    format!("{} ofrece los camellos", nombre_lugar(lugar))
                }
                FuenteMedio::EmbarcacionActual(lugar) => {
                    format!("el grupo conserva su medio desde {}", nombre_lugar(lugar))
                }
                FuenteMedio::CapacidadHumana => "El grupo dispone de camellos".to_owned(),
            },
        };
        let acuatico = match (propuesta.medio_acuatico, propuesta.fuente_acuatica) {
            (MedioAcuaticoProyecto::Ninguno, _) => None,
            (medio, Some(FuenteMedio::Actor(actor))) => Some(format!(
                "{} aporta el {}",
                nombre_actor(actor),
                nombre_medio_acuatico(medio)
            )),
            (medio, Some(FuenteMedio::Lugar(lugar))) => Some(format!(
                "{} ofrece servicio de {}",
                nombre_lugar(lugar),
                nombre_medio_acuatico(medio)
            )),
            (medio, Some(FuenteMedio::EmbarcacionActual(lugar))) => Some(format!(
                "El grupo continúa en el {} con el que alcanzó {}",
                nombre_medio_acuatico(medio),
                nombre_lugar(lugar)
            )),
            (medio, _) => Some(format!(
                "El grupo dispone de un {}",
                nombre_medio_acuatico(medio)
            )),
        };
        let pronostico = if propuesta.hay_incertidumbre {
            "quedan tramos inciertos"
        } else {
            "no hay incertidumbre geográfica conocida"
        };
        match acuatico {
            Some(acuatico) => format!(
                "{terrestre}. {acuatico}. {} días previstos; {pronostico}.",
                propuesta.dias_previstos
            ),
            None => format!(
                "{terrestre}. {} días previstos; {pronostico}.",
                propuesta.dias_previstos
            ),
        }
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
        let (acciones, pronosticos_viaje, causa_indisponibilidad) =
            if actor.tiene_caracteristica(&muerto) {
                (
                    Vec::new(),
                    Vec::new(),
                    Some(CausaIndisponibilidad::JugadorMuerto),
                )
            } else {
                let destinos = acciones::destinos_disponibles(&self.mundo, jugador_id)
                    .map_err(|error| error.to_string())?;
                let pronosticos_viaje = acciones::pronosticos_viaje(&self.mundo, jugador_id)
                    .map_err(|error| error.to_string())?
                    .into_iter()
                    .map(|pronostico| VistaPronosticoViaje {
                        destino: pronostico.destino,
                        dias_minimos: pronostico.dias_minimos,
                        dias_maximos: pronostico.dias_maximos,
                        exposicion_conocida: pronostico.exposicion_conocida,
                        hay_incertidumbre: pronostico.hay_incertidumbre,
                    })
                    .collect();
                (
                    vec![AccionDisponible::Viajar { destinos }],
                    pronosticos_viaje,
                    None,
                )
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
            pronosticos_viaje,
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

fn formatear_traza(
    id: acciones::IdMomentoCausal,
    subobjetivo: Option<acciones::SubobjetivoViaje>,
    traza: &TrazaAccionParticular,
) -> String {
    let factores = traza
        .factores_visibles()
        .map(|factor| format!("{} ({:+})", factor.explicacion_publica, factor.ajuste))
        .collect::<Vec<_>>()
        .join(", ");
    let factores = if factores.is_empty() {
        "sin factores públicos".to_owned()
    } else {
        factores
    };
    format!(
        "Momento {id:?}: {:?}\nSubobjetivo: {subobjetivo:?}\nFactores: {factores}\nResolución: {:?}; resultado: {}; duración: {} meses y {} días.",
        traza.accion,
        traza.resoluciones,
        traza.desenlace,
        traza.duracion.meses(),
        traza.duracion.dias(),
    )
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
            limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            self.llamadas += 1;
            self.valores
                .pop_front()
                .map(|valor| valor % limite_exclusivo.get())
                .ok_or(ErrorFuenteAzar)
        }
    }

    fn contenido() -> ContenidoValidado {
        let raiz = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base");
        cargar_contenido(&raiz).expect("fixture válido")
    }

    fn id_lugar(posicion: usize) -> IdLugar {
        IdLugar::desde_posicion(posicion).expect("ID válido")
    }

    fn completar_viaje(
        partida: &mut Partida,
        destino: IdLugar,
        opcion_preferida: usize,
    ) -> VistaMundo {
        let mut vista = partida
            .procesar(IntencionFrontend::ComenzarViaje { destino })
            .expect("viaje aceptado");
        for _ in 0..256 {
            vista = match vista {
                VistaSesion::PresentandoAccion(fragmento) => partida
                    .procesar(IntencionFrontend::ContinuarPresentacion {
                        fragmento: fragmento.id,
                    })
                    .expect("continuación aceptada"),
                VistaSesion::EsperandoDecision(decision) => partida
                    .procesar(IntencionFrontend::ResponderDecision {
                        decision: decision.id,
                        opcion: decision.opciones[opcion_preferida].id,
                    })
                    .expect("decisión aceptada"),
                VistaSesion::Disponible(mundo) => return mundo,
                VistaSesion::Fatal(fatal) => panic!("sesión fatal: {}", fatal.mensaje),
            };
        }
        panic!("el viaje excedió el límite de prueba")
    }

    #[test]
    fn recorre_el_fixture_completo_y_avanza_solo_por_el_jugador() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(
            &contenido,
            Box::new(AzarProgramado::nuevo(std::iter::repeat_n(19, 64))),
        )
        .expect("partida válida");
        let vista_inicial = partida.vista();
        let VistaSesion::Disponible(inicial) = vista_inicial else {
            panic!("se esperaba el mundo inicial")
        };
        assert_eq!(inicial.lugar_jugador, id_lugar(0));
        assert_eq!(
            inicial.acciones,
            vec![AccionDisponible::Viajar {
                destinos: (1..26).map(id_lugar).collect()
            }]
        );
        let finalizada = completar_viaje(&mut partida, id_lugar(2), 1);

        assert_eq!(finalizada.lugar_jugador, id_lugar(2));
        assert_eq!(partida.mundo.fecha().mes().numero(), 2);
        assert_eq!(partida.mundo.fecha().dia(), 5);
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
                origen: id_lugar(2),
                destino: id_lugar(0),
            }]
        );
        let cronologia = resumen
            .cronologias()
            .first()
            .expect("la fase NPC conserva su cronología completa");
        assert!(cronologia.registros.len() > 6);
        let momentos = cronologia
            .registros
            .iter()
            .flat_map(|registro| registro.momentos.iter().map(|momento| momento.id))
            .collect::<Vec<_>>();
        assert!(!momentos.is_empty());
        assert_eq!(
            momentos
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            momentos.len()
        );
        assert!(cronologia.decisiones_automaticas > 0);
    }

    #[test]
    fn fase_npc_resume_un_viaje_interrumpido_sin_convertirlo_en_fallo_tecnico() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(
            &contenido,
            Box::new(AzarProgramado::nuevo(
                std::iter::repeat_n(19, 3).chain(std::iter::repeat_n(0, 61)),
            )),
        )
        .expect("partida válida");
        let disponible = completar_viaje(&mut partida, id_lugar(2), 1);

        let actor_npc = IdActor::desde_posicion(2).expect("ID válido");
        assert_eq!(partida.mundo.ubicacion_actor(actor_npc), Some(id_lugar(2)));
        assert_eq!(
            disponible
                .resumen_npc_reciente
                .expect("el intento NPC produce resumen")
                .entradas(),
            &[EntradaResumenNpc::ViajeInterrumpido {
                actor: actor_npc,
                lugar: id_lugar(2),
            }]
        );
    }

    #[test]
    fn decision_visible_se_corresponde_con_la_opcion_interna() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(&contenido, Box::new(AzarProgramado::nuevo([19])))
            .expect("partida válida");
        let mut vista = partida
            .procesar(IntencionFrontend::ComenzarViaje {
                destino: id_lugar(1),
            })
            .expect("viaje aceptado");
        while let VistaSesion::PresentandoAccion(fragmento) = vista {
            vista = partida
                .procesar(IntencionFrontend::ContinuarPresentacion {
                    fragmento: fragmento.id,
                })
                .expect("presentación aceptada");
        }
        let VistaSesion::EsperandoDecision(decision) = vista else {
            panic!("se esperaba deliberación")
        };
        assert_eq!(decision.opciones.len(), 2);
        assert_eq!(
            partida.procesar(IntencionFrontend::ResponderDecision {
                decision: IdDecisionVista(decision.id.0 + 1),
                opcion: decision.opciones[0].id,
            }),
            Err(ErrorIntencion::DecisionNoVigente)
        );
        let mut vista = partida
            .procesar(IntencionFrontend::ResponderDecision {
                decision: decision.id,
                opcion: decision.opciones[0].id,
            })
            .expect("opción vigente");
        while let VistaSesion::PresentandoAccion(fragmento) = vista {
            vista = partida
                .procesar(IntencionFrontend::ContinuarPresentacion {
                    fragmento: fragmento.id,
                })
                .expect("presentación aceptada");
        }
        assert!(matches!(vista, VistaSesion::EsperandoDecision(_)));
    }

    #[test]
    fn vista_es_una_instantanea_estable() {
        let contenido = contenido();
        let partida = Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) })
            .expect("partida válida");
        assert_eq!(partida.vista(), partida.vista());
    }

    #[test]
    fn viaje_largo_presenta_toda_la_cronologia_sin_presupuesto_de_vinetas() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(
            &contenido,
            Box::new(AzarProgramado::nuevo(std::iter::repeat_n(19, 512))),
        )
        .expect("partida válida");
        let mut vista = partida
            .procesar(IntencionFrontend::ComenzarViaje {
                destino: id_lugar(22),
            })
            .expect("viaje aceptado");
        let mut fragmentos = 0;
        let mut roles = Vec::new();
        let mut detalles = 0;
        for _ in 0..512 {
            vista = match vista {
                VistaSesion::PresentandoAccion(fragmento) => {
                    fragmentos += 1;
                    roles.push(fragmento.rol);
                    detalles += fragmento.detalle.len();
                    partida
                        .procesar(IntencionFrontend::ContinuarPresentacion {
                            fragmento: fragmento.id,
                        })
                        .expect("fragmento vigente")
                }
                VistaSesion::EsperandoDecision(decision) => {
                    let opcion = decision.opciones.get(1).unwrap_or(&decision.opciones[0]);
                    partida
                        .procesar(IntencionFrontend::ResponderDecision {
                            decision: decision.id,
                            opcion: opcion.id,
                        })
                        .expect("decisión vigente")
                }
                VistaSesion::Disponible(mundo) => {
                    assert_eq!(mundo.lugar_jugador, id_lugar(22));
                    assert!(fragmentos > 6);
                    assert!(detalles > 1);
                    assert!(roles.contains(&RolPresentacion::Apertura));
                    assert!(roles.contains(&RolPresentacion::PlanteamientoSituacion));
                    assert!(roles.contains(&RolPresentacion::DesenlaceSituacion));
                    assert!(roles.contains(&RolPresentacion::Conclusion));
                    return;
                }
                VistaSesion::Fatal(fatal) => panic!("sesión fatal: {}", fatal.mensaje),
            };
        }
        panic!("la cronología no concluyó")
    }

    #[test]
    fn integra_supervivencia_en_el_valle_y_navegacion_desde_nahal() {
        let contenido = contenido();
        let mut partida = Partida::con_azar(
            &contenido,
            Box::new(AzarProgramado::nuevo(std::iter::repeat_n(19, 256))),
        )
        .expect("partida válida");
        partida
            .mundo
            .trasladar(partida.mundo.actor_jugador(), id_lugar(2))
            .expect("traslado de preparación válido");
        let valle = completar_viaje(&mut partida, id_lugar(22), 1);
        assert_eq!(valle.lugar_jugador, id_lugar(22));
        assert!(
            !partida
                .mundo
                .actor(partida.mundo.actor_jugador())
                .expect("jugador existente")
                .tiene_caracteristica(
                    &desert_tiger_core::Caracteristica::nueva("hambriento".to_owned())
                        .expect("rasgo válido")
                )
        );

        partida
            .mundo
            .trasladar(partida.mundo.actor_jugador(), id_lugar(14))
            .expect("traslado al puerto válido");
        let estuario = completar_viaje(&mut partida, id_lugar(25), 0);
        assert_eq!(estuario.lugar_jugador, id_lugar(25));
        let regreso_a_nahal = completar_viaje(&mut partida, id_lugar(14), 0);
        assert_eq!(regreso_a_nahal.lugar_jugador, id_lugar(14));
        let mar = completar_viaje(&mut partida, id_lugar(17), 1);
        assert_eq!(mar.lugar_jugador, id_lugar(17));
    }

    #[test]
    fn todos_los_destinos_del_atlas_terminan_en_un_estado_valido() {
        let contenido = contenido();
        for posicion in 1..26 {
            let mut partida = Partida::con_azar(
                &contenido,
                Box::new(AzarProgramado::nuevo(std::iter::repeat_n(19, 512))),
            )
            .expect("partida válida");
            let vista = completar_viaje(&mut partida, id_lugar(posicion), 1);
            assert_eq!(
                vista.lugar_jugador,
                id_lugar(posicion),
                "el recorrido de matriz no alcanzó el destino {posicion}"
            );
            assert_eq!(
                partida.mundo.ubicacion_actor(partida.mundo.actor_jugador()),
                Some(vista.lugar_jugador),
                "destino de matriz {posicion}"
            );
        }
    }
}
