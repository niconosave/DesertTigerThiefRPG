use crate::contenido::{
    ClaveTipoTile, ContenidoValidado, CoordenadaMapa, EstadoMapa, RetratoActor,
};
use crate::fase_npc::{ResumenNpc, ejecutar_fase_npc};
use desert_tiger_core::acciones::{
    self, ClaveEscena, ClaveTarjeta, ContextoAccion, EjecucionAccion, IdPaso, OpcionViaje,
    PasoAccion, RechazoAccion, RegistroEscena, RegistroProyecto, RespuestaDecision,
    RolPresentacion,
};
use desert_tiger_core::azar::{ErrorFuenteAzar, FuenteAzar};
use desert_tiger_core::{Duracion, Fecha, FraccionDia, IdActor, IdLugar, InstanteAccion, Mundo};
use rand::rngs::SysRng;
use rand::{RngExt, SeedableRng, TryRng};
use rand_chacha::ChaCha12Rng;
use std::{error::Error, fmt, num::NonZeroUsize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfiguracionPartida {
    pub semilla: Option<u64>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoErrorCrearPartida {
    ContenidoInconsistente,
    EntropiaNoDisponible,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorCrearPartida {
    pub codigo: CodigoErrorCrearPartida,
    pub detalle: String,
}
impl fmt::Display for ErrorCrearPartida {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detalle)
    }
}
impl Error for ErrorCrearPartida {}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdFragmento(u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdDecisionVista(u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdOpcionVista(u32);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntencionFrontend {
    ComenzarCampamento,
    AbandonarCampamento {
        campamento: IdCampamento,
    },
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
    CampamentoNoVigente,
    AcampadaRechazada(acciones::ErrorAcampada),
    FaseIncompatible,
    FragmentoNoVigente,
    DecisionNoVigente,
    OpcionNoVigente,
    AccionRechazada(RechazoAccion),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VistaSesion {
    Acampando(VistaCampamento),
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
impl VistaMundo {
    /// Señal visual para los retratos del grupo; no modifica actores del Mundo.
    /// Una expresión ausente del retrato conserva su cara base.
    pub fn mostrar_expresion_grupo(&mut self, expresion: Option<&str>) {
        for actor in std::iter::once(&mut self.jugador).chain(&mut self.companeros) {
            actor.expresion = expresion.map(str::to_owned);
        }
    }
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
    pub nombre_abreviado: String,
    pub coordenada_central: CoordenadaMapa,
    pub coordenadas_rotulo: Vec<CoordenadaMapa>,
    pub caracteristicas: Vec<String>,
    pub conectados: Vec<IdLugar>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaActor {
    pub nombre: String,
    pub id: IdActor,
    pub retrato: RetratoActor,
    pub expresion: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccionDisponible {
    Acampar,
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
    pub tiempo_transcurrido: String,
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
    pub detalle: Vec<String>,
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

struct AzarChaCha(ChaCha12Rng);
impl FuenteAzar for AzarChaCha {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
        Ok(self.0.random_range(0..n.get()))
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdCampamento(u64);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegranteCampamento {
    pub actor: IdActor,
    pub nombre: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaCampamento {
    pub id: IdCampamento,
    pub lugar: IdLugar,
    pub integrantes: Vec<IntegranteCampamento>,
}
#[derive(Debug)]
enum FalloCierreAccion {
    Calendario(String),
    Npc(String),
    Proyeccion(String),
}
impl FalloCierreAccion {
    fn codigo(&self) -> CodigoFatalSesion {
        match self {
            Self::Calendario(_) => CodigoFatalSesion::Calendario,
            Self::Npc(_) => CodigoFatalSesion::IntegracionNpc,
            Self::Proyeccion(_) => CodigoFatalSesion::EstadoInconsistente,
        }
    }
    fn detalle(self) -> String {
        match self {
            Self::Calendario(s) | Self::Npc(s) | Self::Proyeccion(s) => s,
        }
    }
}
pub struct Partida {
    acampada: Option<acciones::Acampada>,
    mundo: Mundo,
    mapa: EstadoMapa,
    contenido: ContenidoValidado,
    vista: VistaSesion,
    mundo_visible: VistaMundo,
    azar: Box<dyn FuenteAzar>,
    ejecucion: Option<EjecucionAccion>,
    paso: Option<IdPaso>,
    opciones: Vec<OpcionViaje>,
    rumbos: acciones::CatalogoRumbos,
    inicio: InstanteAccion,
    resto: FraccionDia,
    ordinal: u64,
}
fn error_crear(detalle: String) -> ErrorCrearPartida {
    ErrorCrearPartida {
        codigo: CodigoErrorCrearPartida::ContenidoInconsistente,
        detalle,
    }
}
impl Partida {
    pub fn nueva(
        contenido: &ContenidoValidado,
        config: ConfiguracionPartida,
    ) -> Result<Self, ErrorCrearPartida> {
        let semilla = match config.semilla {
            Some(s) => s,
            None => SysRng.try_next_u64().map_err(|e| ErrorCrearPartida {
                codigo: CodigoErrorCrearPartida::EntropiaNoDisponible,
                detalle: e.to_string(),
            })?,
        };
        Self::con_azar(
            contenido,
            Box::new(AzarChaCha(ChaCha12Rng::seed_from_u64(semilla))),
        )
    }
    fn con_azar(
        contenido: &ContenidoValidado,
        azar: Box<dyn FuenteAzar>,
    ) -> Result<Self, ErrorCrearPartida> {
        let (mundo, mapa) = contenido
            .instanciar_estado()
            .map_err(|e| error_crear(e.to_string()))?;
        let rumbos = crate::contexto_viaje::construir(&mundo, contenido.presentacion())
            .map_err(error_crear)?;
        let inicio = InstanteAccion {
            fecha: mundo.fecha(),
            fraccion: FraccionDia::CERO,
        };
        let base = proyectar_mundo(&mundo, &mapa, contenido, None).map_err(error_crear)?;
        Ok(Self {
            acampada: None,
            mundo,
            mapa,
            contenido: contenido.clone(),
            vista: VistaSesion::Disponible(base.clone()),
            mundo_visible: base,
            azar,
            ejecucion: None,
            paso: None,
            opciones: Vec::new(),
            rumbos,
            inicio,
            resto: FraccionDia::CERO,
            ordinal: 0,
        })
    }
    pub fn vista(&self) -> VistaSesion {
        self.vista.clone()
    }
    pub fn mundo_visible(&self) -> VistaMundo {
        self.mundo_visible.clone()
    }
    pub fn procesar(
        &mut self,
        intencion: IntencionFrontend,
    ) -> Result<VistaSesion, ErrorIntencion> {
        match intencion {
            IntencionFrontend::ComenzarCampamento => {
                if !matches!(self.vista, VistaSesion::Disponible(_)) {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                let acampada = acciones::iniciar_acampada(&self.mundo, self.mundo.actor_jugador())
                    .map_err(ErrorIntencion::AcampadaRechazada)?;
                let Some(ordinal) = self.ordinal.checked_add(1) else {
                    self.fatal(
                        CodigoFatalSesion::ContadorAgotado,
                        "contador de campamentos agotado".into(),
                    );
                    return Ok(self.vista());
                };
                let mut mundo =
                    match proyectar_mundo(&self.mundo, &self.mapa, &self.contenido, None) {
                        Ok(m) => m,
                        Err(e) => {
                            self.fatal(CodigoFatalSesion::EstadoInconsistente, e);
                            return Ok(self.vista());
                        }
                    };
                let integrantes = mundo
                    .companeros
                    .iter()
                    .chain(std::iter::once(&mundo.jugador))
                    .map(|a| IntegranteCampamento {
                        actor: a.id,
                        nombre: a.nombre.clone(),
                    })
                    .collect();
                mundo.acciones.clear();
                self.ordinal = ordinal;
                self.acampada = Some(acampada);
                self.vista = VistaSesion::Acampando(VistaCampamento {
                    id: IdCampamento(ordinal),
                    lugar: mundo.lugar_jugador,
                    integrantes,
                });
                self.mundo_visible = mundo;
                return Ok(self.vista());
            }
            IntencionFrontend::AbandonarCampamento { campamento } => {
                let VistaSesion::Acampando(v) = &self.vista else {
                    return Err(ErrorIntencion::FaseIncompatible);
                };
                if v.id != campamento {
                    return Err(ErrorIntencion::CampamentoNoVigente);
                }
                let Some(acampada) = self.acampada.take() else {
                    self.fatal(
                        CodigoFatalSesion::EstadoInconsistente,
                        "campamento sin acampada".into(),
                    );
                    return Ok(self.vista());
                };
                let resultado = acciones::finalizar_acampada(acampada);
                if let Err(e) = self.concluir_accion(resultado.tiempo) {
                    self.fatal(e.codigo(), e.detalle());
                }
                return Ok(self.vista());
            }
            IntencionFrontend::ComenzarViaje { .. }
            | IntencionFrontend::ContinuarPresentacion { .. }
            | IntencionFrontend::ResponderDecision { .. } => {}
        }
        let resultado = match intencion {
            IntencionFrontend::ComenzarCampamento
            | IntencionFrontend::AbandonarCampamento { .. } => {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            IntencionFrontend::ComenzarViaje { destino } => {
                if !matches!(self.vista, VistaSesion::Disponible(_)) {
                    return Err(ErrorIntencion::FaseIncompatible);
                }
                self.inicio = InstanteAccion {
                    fecha: self.mundo.fecha(),
                    fraccion: self.resto,
                };
                let contexto = ContextoAccion {
                    inicio: self.inicio,
                    rumbos: &self.rumbos,
                };
                match acciones::iniciar(
                    &mut self.mundo,
                    acciones::SolicitudAccion::Viajar {
                        actor: self.mundo_visible.jugador.id,
                        destino,
                    },
                    &contexto,
                    self.azar.as_mut(),
                ) {
                    Ok(inicio) => {
                        self.ejecucion = Some(inicio.ejecucion);
                        Ok(inicio.paso)
                    }
                    Err(e) => Err(e),
                }
            }
            IntencionFrontend::ContinuarPresentacion { fragmento } => {
                let VistaSesion::PresentandoAccion(v) = &self.vista else {
                    return Err(ErrorIntencion::FaseIncompatible);
                };
                if v.id != fragmento {
                    return Err(ErrorIntencion::FragmentoNoVigente);
                }
                let paso = self.paso.ok_or(ErrorIntencion::FaseIncompatible)?;
                let e = self
                    .ejecucion
                    .as_mut()
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                acciones::avanzar(
                    &mut self.mundo,
                    e,
                    paso,
                    &ContextoAccion {
                        inicio: self.inicio,
                        rumbos: &self.rumbos,
                    },
                    self.azar.as_mut(),
                )
            }
            IntencionFrontend::ResponderDecision { decision, opcion } => {
                let VistaSesion::EsperandoDecision(d) = &self.vista else {
                    return Err(ErrorIntencion::FaseIncompatible);
                };
                if d.id != decision {
                    return Err(ErrorIntencion::DecisionNoVigente);
                }
                let i = d
                    .opciones
                    .iter()
                    .position(|o| o.id == opcion)
                    .ok_or(ErrorIntencion::OpcionNoVigente)?;
                let seleccion = *self
                    .opciones
                    .get(i)
                    .ok_or(ErrorIntencion::OpcionNoVigente)?;
                let e = self
                    .ejecucion
                    .as_mut()
                    .ok_or(ErrorIntencion::FaseIncompatible)?;
                acciones::responder(
                    &mut self.mundo,
                    e,
                    RespuestaDecision {
                        paso: self.paso.ok_or(ErrorIntencion::FaseIncompatible)?,
                        opcion: seleccion,
                    },
                    &ContextoAccion {
                        inicio: self.inicio,
                        rumbos: &self.rumbos,
                    },
                    self.azar.as_mut(),
                )
            }
        };
        match resultado {
            Ok(paso) => {
                if let Err(e) = self.publicar(paso) {
                    self.fatal(CodigoFatalSesion::EstadoInconsistente, e);
                }
            }
            Err(acciones::ErrorMotor::Rechazo(r)) => {
                return Err(ErrorIntencion::AccionRechazada(r));
            }
            Err(e) => self.fatal(CodigoFatalSesion::Motor, e.to_string()),
        }
        Ok(self.vista())
    }
    pub fn consultar_ficha(
        &self,
        campamento: IdCampamento,
        elemento: crate::ExtremoRelacion,
    ) -> Result<crate::VistaFicha, crate::ErrorConsultaFicha> {
        let VistaSesion::Acampando(v) = &self.vista else {
            return Err(crate::ErrorConsultaFicha::FaseIncompatible);
        };
        if v.id != campamento {
            return Err(crate::ErrorConsultaFicha::CampamentoNoVigente);
        }
        crate::fichas::proyectar(&self.mundo, &self.contenido, elemento)
    }
    fn concluir_accion(
        &mut self,
        tiempo: desert_tiger_core::TiempoAccion,
    ) -> Result<(), FalloCierreAccion> {
        self.ejecucion = None;
        self.paso = None;
        self.opciones.clear();
        self.acampada = None;
        crate::tiempo::confirmar(&mut self.mundo, &mut self.resto, tiempo)
            .map_err(FalloCierreAccion::Calendario)?;
        let inicio = InstanteAccion {
            fecha: self.mundo.fecha(),
            fraccion: self.resto,
        };
        let resumen = ejecutar_fase_npc(
            &mut self.mundo,
            self.contenido.politicas(),
            self.contenido.claves(),
            &self.rumbos,
            inicio,
            self.azar.as_mut(),
        )
        .map_err(|e| FalloCierreAccion::Npc(e.0))?;
        self.mundo_visible = proyectar_mundo(&self.mundo, &self.mapa, &self.contenido, resumen)
            .map_err(FalloCierreAccion::Proyeccion)?;
        self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
        Ok(())
    }
    fn publicar(&mut self, paso: PasoAccion) -> Result<(), String> {
        if let PasoAccion::Finalizada(fin) = paso {
            if let Err(e) = self.concluir_accion(fin.tiempo) {
                self.fatal(e.codigo(), e.detalle());
            }
            return Ok(());
        }
        let (registro, opciones) = match paso {
            PasoAccion::Escena(r) => (r, None),
            PasoAccion::Decision(d) => (d.escena, Some(d.opciones)),
            PasoAccion::Finalizada(_) => return Err("finalización duplicada".into()),
        };
        self.ordinal = self
            .ordinal
            .checked_add(1)
            .ok_or("contador de vistas agotado")?;
        self.paso = Some(registro.id);
        let narrador = crate::narracion_viaje::Narrador(&self.contenido);
        let texto = narrador.redactar(&registro);
        let progreso = progreso(&registro, &narrador);
        self.mundo_visible = proyectar_mundo(&self.mundo, &self.mapa, &self.contenido, None)?;
        self.mundo_visible.lugar_jugador = registro.lugar;
        self.mundo_visible.companeros = registro
            .participantes
            .iter()
            .copied()
            .filter(|&id| id != self.mundo_visible.jugador.id)
            .map(|id| proyectar_actor(&self.contenido, id))
            .collect::<Result<_, _>>()?;
        self.mundo_visible.mostrar_expresion_grupo(texto.expresion);
        let escena = Some(match registro.tarjeta {
            ClaveTarjeta::Llegada => ClaveEscena::ViajeLlegada,
            ClaveTarjeta::Navegar => ClaveEscena::ViajeMarcha,
            ClaveTarjeta::Andar | ClaveTarjeta::Caballo | ClaveTarjeta::Camello => {
                ClaveEscena::ViajeMarcha
            }
            _ => ClaveEscena::ViajeRumboIncierto,
        });
        let rol = if registro.tarjeta == ClaveTarjeta::Llegada {
            RolPresentacion::Conclusion
        } else {
            RolPresentacion::DesenlaceSituacion
        };
        self.vista = if let Some(opciones) = opciones {
            self.opciones = opciones;
            let opciones = self
                .opciones
                .iter()
                .enumerate()
                .map(|(i, &o)| {
                    Ok(VistaOpcionDecision {
                        id: IdOpcionVista(u32::try_from(i).map_err(|_| "demasiadas opciones")?),
                        texto: crate::narracion_viaje::opcion(o).into(),
                        detalle: None,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            VistaSesion::EsperandoDecision(VistaDecision {
                id: IdDecisionVista(self.ordinal),
                titulo: texto.titulo,
                texto: texto.texto,
                escena,
                rol,
                proyecto: None,
                progreso,
                detalle: texto.detalle,
                opciones,
            })
        } else {
            self.opciones.clear();
            VistaSesion::PresentandoAccion(VistaFragmento {
                id: IdFragmento(self.ordinal),
                titulo: texto.titulo,
                texto: texto.texto,
                escena,
                rol,
                proyecto: None,
                progreso,
                detalle: texto.detalle,
            })
        };
        Ok(())
    }
    fn fatal(&mut self, codigo: CodigoFatalSesion, mensaje: String) {
        self.vista = VistaSesion::Fatal(VistaFatal {
            codigo,
            mensaje: crate::contenido::sanear(&mensaje),
        });
    }
}
fn progreso(r: &RegistroEscena, n: &crate::narracion_viaje::Narrador<'_>) -> VistaProgresoViaje {
    VistaProgresoViaje {
        lugar_confirmado: r.lugar,
        nombre_lugar_confirmado: n.lugar(r.lugar),
        siguiente_lugar: r.siguiente,
        nombre_siguiente_lugar: r.siguiente.map(|id| n.lugar(id)),
        destino_final: r.destino_final,
        nombre_destino_final: n.lugar(r.destino_final),
        duracion_transcurrida: r.total.duracion,
        tiempo_transcurrido: crate::narracion_viaje::tiempo_texto(r.total),
        fecha_proyectada: r.instante.fecha,
    }
}
fn proyectar_actor(contenido: &ContenidoValidado, id: IdActor) -> Result<VistaActor, String> {
    let actor = contenido
        .presentacion()
        .actor(id)
        .ok_or("Actor desconocido")?;
    let retrato = contenido
        .presentacion()
        .actor(id)
        .and_then(|a| a.retrato())
        .cloned()
        .ok_or("Actor sin retrato")?;
    Ok(VistaActor {
        nombre: actor.nombre().to_owned(),
        id,
        retrato,
        expresion: None,
    })
}
fn proyectar_mundo(
    m: &Mundo,
    mapa: &EstadoMapa,
    c: &ContenidoValidado,
    resumen: Option<ResumenNpc>,
) -> Result<VistaMundo, String> {
    let jugador = proyectar_actor(c, m.actor_jugador())?;
    let muerto = m
        .actor(m.actor_jugador())
        .ok_or("jugador ausente")?
        .caracteristicas()
        .any(|c| c.como_str() == "muerto");
    let acciones = if muerto {
        Vec::new()
    } else {
        vec![
            AccionDisponible::Viajar {
                destinos: acciones::destinos_disponibles(m, m.actor_jugador())
                    .map_err(|e| e.to_string())?,
            },
            AccionDisponible::Acampar,
        ]
    };
    Ok(VistaMundo {
        mapa: VistaMapa {
            ancho: mapa.ancho(),
            alto: mapa.alto(),
            celdas: mapa.celdas().to_vec(),
            tipos_declarados: mapa.tipos_declarados().cloned().collect(),
        },
        lugares: c
            .presentacion()
            .lugares()
            .iter()
            .map(|l| {
                Ok(VistaLugar {
                    id: l.id(),
                    nombre: l.nombre().into(),
                    nombre_abreviado: l.nombre_abreviado().into(),
                    coordenada_central: l.coordenada_central(),
                    coordenadas_rotulo: l.coordenadas_rotulo().to_vec(),
                    caracteristicas: m
                        .lugar(l.id())
                        .ok_or("Lugar sin estado")?
                        .caracteristicas()
                        .map(|c| c.como_str().to_owned())
                        .collect(),
                    conectados: m
                        .lugares_conectados(l.id())
                        .ok_or("Lugar sin conexiones")?
                        .to_vec(),
                })
            })
            .collect::<Result<_, String>>()?,
        lugar_jugador: m
            .ubicacion_actor(m.actor_jugador())
            .ok_or("jugador sin Lugar")?,
        jugador,
        companeros: m
            .grupo()
            .integrantes()
            .iter()
            .map(|&id| proyectar_actor(c, id))
            .collect::<Result<_, _>>()?,
        acciones,
        causa_indisponibilidad: if muerto {
            Some(CausaIndisponibilidad::JugadorMuerto)
        } else {
            None
        },
        resumen_npc_reciente: resumen,
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{EntradaFicha, ErrorConsultaFicha, ExtremoRelacion, SentidoRelacionFicha};
    use desert_tiger_core::{Caracteristica, LoteOperaciones, OperacionMundo, TiempoAccion};

    #[test]
    fn acampar_consulta_estado_actual_y_relaciones_sin_consumir_azar() {
        struct SinAzar;
        impl FuenteAzar for SinAzar {
            fn elegir_indice(&mut self, _: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
                panic!("una consulta no puede consumir azar");
            }
        }
        let c = contenido();
        let mut p = Partida::con_azar(&c, Box::new(SinAzar)).unwrap();
        let jugador = p.mundo.actor_jugador();
        let lugar = p.mundo.ubicacion_actor(jugador).unwrap();
        p.mundo
            .aplicar_lote(
                LoteOperaciones::nuevo(vec![
                    OperacionMundo::AnadirCaracteristicaActor {
                        actor: jugador,
                        caracteristica: Caracteristica::nueva("rasgoNuevo".into()).unwrap(),
                    },
                    OperacionMundo::RegistrarConocimiento {
                        actor: jugador,
                        lugar,
                    },
                ])
                .unwrap(),
            )
            .unwrap();
        let antes = format!("{:?}", p.mundo);
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        let ficha = p
            .consultar_ficha(v.id, ExtremoRelacion::Actor(jugador))
            .unwrap();
        assert_eq!(ficha.nombre, "Príncipe Perdido");
        assert!(ficha.entradas.iter().any(|e| matches!(e, EntradaFicha::Caracteristica { clave, descripcion: None, .. } if clave == "rasgoNuevo")));
        assert!(ficha.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if matches!(&r.vinculo, crate::VinculoFicha::Registrado { sentido: SentidoRelacionFicha::Saliente, .. }))));
        let cazador = IdActor::desde_posicion(2).unwrap();
        assert!(c.presentacion().actor(cazador).unwrap().retrato().is_none());
        for _ in 0..100 {
            let f = p
                .consultar_ficha(v.id, ExtremoRelacion::Actor(cazador))
                .unwrap();
            let referencia = f
                .entradas
                .iter()
                .find_map(|e| match e {
                    EntradaFicha::Relacion(r) => Some(&r.referencia),
                    _ => None,
                })
                .unwrap();
            let f = p.consultar_ficha(v.id, referencia.elemento).unwrap();
            assert!(f.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if matches!(&r.vinculo, crate::VinculoFicha::Registrado { sentido: SentidoRelacionFicha::Entrante, .. }) && r.referencia.elemento == ExtremoRelacion::Actor(cazador))));
            assert!(f.entradas.iter().any(|e| matches!(e, EntradaFicha::Relacion(r) if matches!(&r.vinculo, crate::VinculoFicha::Registrado { sentido: SentidoRelacionFicha::Simetrica, .. }))));
        }
        assert_eq!(
            p.consultar_ficha(
                v.id,
                ExtremoRelacion::Actor(IdActor::desde_posicion(999).unwrap())
            ),
            Err(ErrorConsultaFicha::ElementoInexistente)
        );
        assert_eq!(
            p.procesar(IntencionFrontend::ComenzarCampamento),
            Err(ErrorIntencion::FaseIncompatible)
        );
        assert_eq!(format!("{:?}", p.mundo), antes);
    }

    #[test]
    fn acampar_confirma_un_dia_con_resto_y_npc_una_sola_vez() {
        use std::{cell::Cell, rc::Rc};
        struct Contado(Rc<Cell<usize>>);
        impl FuenteAzar for Contado {
            fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
                self.0.set(self.0.get() + 1);
                Ok(match n.get() {
                    20 => 19,
                    9 => 8,
                    _ => 0,
                })
            }
        }
        let cuenta = Rc::new(Cell::new(0));
        let mut p = Partida::con_azar(&contenido(), Box::new(Contado(cuenta.clone()))).unwrap();
        p.resto = FraccionDia::nueva(300_000_000).unwrap();
        let fecha = p.mundo.fecha();
        let lugar = p.mundo.ubicacion_actor(p.mundo.actor_jugador());
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        assert_eq!(cuenta.get(), 0);
        assert!(matches!(
            p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
                .unwrap(),
            VistaSesion::Disponible(_)
        ));
        let fin = desert_tiger_core::proyectar_tiempo(
            InstanteAccion {
                fecha,
                fraccion: FraccionDia::CERO,
            },
            TiempoAccion {
                duracion: Duracion::desde_dias(1),
                fraccion: FraccionDia::CERO,
            },
        )
        .unwrap();
        assert_eq!(p.mundo.fecha(), fin.fecha);
        assert_eq!(p.resto.partes(), 300_000_000);
        assert_eq!(p.mundo.ubicacion_actor(p.mundo.actor_jugador()), lugar);
        assert!(
            cuenta.get() > 0,
            "la política NPC debe actuar al cambiar de mes"
        );
        let llamadas = cuenta.get();
        assert_eq!(
            p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id }),
            Err(ErrorIntencion::FaseIncompatible)
        );
        let VistaSesion::Acampando(nuevo) =
            p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        assert_ne!(nuevo.id, v.id);
        assert_eq!(
            p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id }),
            Err(ErrorIntencion::CampamentoNoVigente)
        );
        assert_eq!(
            p.consultar_ficha(v.id, ExtremoRelacion::Actor(p.mundo.actor_jugador())),
            Err(ErrorConsultaFicha::CampamentoNoVigente)
        );
        assert_eq!(cuenta.get(), llamadas);
        assert_eq!(p.mundo.fecha(), fin.fecha);
    }

    #[test]
    fn acampar_rechaza_muerto_y_calendario_no_desborda_parcialmente() {
        let mut p = Partida::con_azar(&contenido(), Box::new(AzarFavorable)).unwrap();
        let fecha = p.mundo.fecha();
        let limite = Fecha::nueva(9999, desert_tiger_core::Mes::Diciembre, 31).unwrap();
        let meses = (9999 - u32::from(fecha.anio())) * 12 + 11;
        p.mundo.avanzar_fecha(Duracion::desde_meses(meses)).unwrap();
        p.resto = FraccionDia::nueva(123).unwrap();
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        assert!(matches!(
            p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
                .unwrap(),
            VistaSesion::Fatal(VistaFatal {
                codigo: CodigoFatalSesion::Calendario,
                ..
            })
        ));
        assert_eq!(p.mundo.fecha(), limite);
        assert_eq!(p.resto.partes(), 123);
        let mut p = Partida::con_azar(&contenido(), Box::new(AzarFavorable)).unwrap();
        p.mundo
            .aplicar_lote(
                LoteOperaciones::nuevo(vec![OperacionMundo::AnadirCaracteristicaActor {
                    actor: p.mundo.actor_jugador(),
                    caracteristica: Caracteristica::nueva("muerto".into()).unwrap(),
                }])
                .unwrap(),
            )
            .unwrap();
        assert_eq!(
            p.procesar(IntencionFrontend::ComenzarCampamento),
            Err(ErrorIntencion::AcampadaRechazada(
                acciones::ErrorAcampada::ActorMuerto
            ))
        );
    }

    #[test]
    fn campamento_sin_companeros_admite_ficha_vacia_y_cierre() {
        let c = contenido();
        let mut p = Partida::con_azar(&c, Box::new(AzarFavorable)).unwrap();
        let lugar = p.mundo.ubicacion_actor(p.mundo.actor_jugador()).unwrap();
        let operaciones = p
            .mundo
            .grupo()
            .integrantes()
            .iter()
            .map(|&actor| OperacionMundo::SepararCompanero { actor, lugar })
            .collect();
        p.mundo
            .aplicar_lote(LoteOperaciones::nuevo(operaciones).unwrap())
            .unwrap();
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        assert_eq!(v.integrantes.len(), 1);
        assert_eq!(v.integrantes[0].actor, p.mundo.actor_jugador());
        assert!(
            p.consultar_ficha(
                v.id,
                ExtremoRelacion::Actor(IdActor::desde_posicion(4).unwrap())
            )
            .unwrap()
            .entradas
            .is_empty()
        );
        assert!(matches!(
            p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
                .unwrap(),
            VistaSesion::Disponible(_)
        ));
        assert!(
            p.mundo_visible()
                .acciones
                .iter()
                .any(|a| matches!(a, AccionDisponible::Acampar))
        );
    }
    struct AzarFavorable;
    impl FuenteAzar for AzarFavorable {
        fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
            // Pruebas máximas, materialización sin desperdicio y buena búsqueda.
            Ok(match n.get() {
                20 => 19,
                9 => 8,
                _ => 0,
            })
        }
    }
    fn contenido() -> ContenidoValidado {
        crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap()
    }
    #[test]
    fn expresiones_de_vineta_llegan_al_grupo_y_se_limpian_al_final() {
        let c = contenido();
        let mut p = Partida::con_azar(&c, Box::new(AzarFavorable)).unwrap();
        let mut vista = p.mundo_visible();
        let antes = p.mundo_visible();
        vista.mostrar_expresion_grupo(Some("angry"));
        assert!(
            std::iter::once(&vista.jugador)
                .chain(&vista.companeros)
                .all(|a| a.expresion.as_deref() == Some("angry"))
        );
        assert_eq!(p.mundo_visible(), antes);
        let AccionDisponible::Viajar { destinos } = &antes.acciones[0] else {
            panic!("se esperaba Viajar")
        };
        let destino = destinos[0];
        let mut v = p
            .procesar(IntencionFrontend::ComenzarViaje { destino })
            .unwrap();
        let mut vio_happy = false;
        for _ in 0..500 {
            let mundo = p.mundo_visible();
            let expresion = mundo.jugador.expresion.as_deref();
            assert!(
                mundo
                    .companeros
                    .iter()
                    .all(|a| a.expresion.as_deref() == expresion)
            );
            vio_happy |= expresion == Some("happy");
            v = match v {
                VistaSesion::PresentandoAccion(r) => p
                    .procesar(IntencionFrontend::ContinuarPresentacion { fragmento: r.id })
                    .unwrap(),
                VistaSesion::EsperandoDecision(d) => p
                    .procesar(IntencionFrontend::ResponderDecision {
                        decision: d.id,
                        opcion: d.opciones[0].id,
                    })
                    .unwrap(),
                VistaSesion::Disponible(_) => {
                    assert_eq!(expresion, None);
                    assert!(vio_happy);
                    return;
                }
                VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
            };
        }
        panic!("el viaje no terminó");
    }
    #[test]
    fn todos_los_destinos_terminan_y_no_hay_desfase() {
        let c = contenido();
        let inicial = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(1) }).unwrap();
        let VistaSesion::Disponible(v) = inicial.vista() else {
            panic!()
        };
        let AccionDisponible::Viajar { destinos } = &v.acciones[0] else {
            panic!("se esperaba Viajar")
        };
        let mut llegadas = 0;
        for &destino in destinos {
            let antes = llegadas;
            for semilla in 0..8 {
                let mut p = Partida::nueva(
                    &c,
                    ConfiguracionPartida {
                        semilla: Some(semilla),
                    },
                )
                .unwrap();
                let inicio = p.mundo.fecha();
                let mut vista = p
                    .procesar(IntencionFrontend::ComenzarViaje { destino })
                    .unwrap();
                let mut ultimo = None;
                for _ in 0..500 {
                    vista = match vista {
                        VistaSesion::PresentandoAccion(r) => {
                            ultimo = Some(r.progreso.fecha_proyectada);
                            let id = r.id;
                            let v = p
                                .procesar(IntencionFrontend::ContinuarPresentacion {
                                    fragmento: id,
                                })
                                .unwrap();
                            assert!(
                                p.procesar(IntencionFrontend::ContinuarPresentacion {
                                    fragmento: id
                                })
                                .is_err()
                            );
                            v
                        }
                        VistaSesion::EsperandoDecision(d) => p
                            .procesar(IntencionFrontend::ResponderDecision {
                                decision: d.id,
                                opcion: d.opciones[0].id,
                            })
                            .unwrap(),
                        VistaSesion::Disponible(ref v) => {
                            if v.lugar_jugador == destino {
                                llegadas += 1;
                            }
                            if let Some(fecha) = ultimo {
                                assert_eq!(p.mundo.fecha(), fecha);
                            }
                            assert!(p.mundo.fecha() >= inicio);
                            break;
                        }
                        VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                        VistaSesion::Fatal(f) => {
                            panic!("destino {destino:?}, semilla {semilla}: {}", f.mensaje)
                        }
                    };
                }
                assert!(matches!(vista, VistaSesion::Disponible(_)), "no termina");
            }
            // Se comprueba además la alcanzabilidad sin incidentes. Las semillas
            // ordinarias pueden terminar válidamente sin alcanzar el destino.
            let mut p = Partida::con_azar(&c, Box::new(AzarFavorable)).unwrap();
            let mut v = p
                .procesar(IntencionFrontend::ComenzarViaje { destino })
                .unwrap();
            for _ in 0..500 {
                v = match v {
                    VistaSesion::PresentandoAccion(r) => p
                        .procesar(IntencionFrontend::ContinuarPresentacion { fragmento: r.id })
                        .unwrap(),
                    VistaSesion::EsperandoDecision(d) => p
                        .procesar(IntencionFrontend::ResponderDecision {
                            decision: d.id,
                            opcion: d.opciones[0].id,
                        })
                        .unwrap(),
                    VistaSesion::Disponible(ref m) => {
                        assert_eq!(
                            m.lugar_jugador, destino,
                            "destino inaccesible incluso sin incidentes: {destino:?}"
                        );
                        llegadas += 1;
                        break;
                    }
                    VistaSesion::Acampando(_) => panic!("campamento inesperado durante Viajar"),
                    VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
                };
            }
            assert!(matches!(v, VistaSesion::Disponible(_)));
            assert!(llegadas > antes);
        }
        assert!(llegadas > 0);
    }
    #[test]
    fn las_vistas_no_revelan_prevision_antes_del_plan() {
        let c = contenido();
        let p = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(2) }).unwrap();
        assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    }
}
