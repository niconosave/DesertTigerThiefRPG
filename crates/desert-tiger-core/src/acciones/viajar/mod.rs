pub(crate) mod carga;
#[cfg(test)]
mod pruebas;
pub(crate) mod registros;
pub(crate) mod reservas;
pub(crate) mod rutas;
mod tarjetas;
pub(crate) mod tiempo;

use super::particulares::chequeos::tiene;
use super::{
    AccionParticular, CatalogoRumbos, ContextoAccion, ErrorMotor, RechazoAccion, Resultado,
    ResultadoChequeo, SolicitudAccion, fallo, resolver_chequeo, sortear,
};
use crate::azar::FuenteAzar;
use crate::{
    IdActor, IdLugar, InstanteAccion, LoteOperaciones, Mundo, OperacionMundo, proyectar_tiempo,
};
use carga::{Configuracion, LoteDescarte};
use registros::*;
use reservas::{RegistroConsumo, ReservasViaje, multiplicar, sumar};
use rutas::Medio;
use tiempo::HorasViaje;

mod estado;
use estado::{EfectosEscena, Fase, Preparacion};
pub use estado::{EjecucionAccion, InicioAccion};
pub fn iniciar(
    mundo: &mut Mundo,
    solicitud: SolicitudAccion,
    contexto: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<InicioAccion> {
    let SolicitudAccion::Viajar { actor, destino } = solicitud;
    let destinos = super::destinos_disponibles(mundo, actor).map_err(ErrorMotor::Rechazo)?;
    let lugar = mundo
        .ubicacion_actor(actor)
        .ok_or(ErrorMotor::Rechazo(RechazoAccion::ActorDesconocido))?;
    if mundo.lugar(destino).is_none()
        || lugar == destino
        || (actor == mundo.actor_jugador() && super::lugar_oculto(mundo, destino))
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::DestinoInvalido));
    }
    if !destinos.contains(&destino) {
        return Err(ErrorMotor::Rechazo(RechazoAccion::SinRuta));
    }
    let mut viajeros = vec![actor];
    if actor == mundo.actor_jugador() {
        viajeros.extend(mundo.grupo().integrantes());
        viajeros[1..].sort_unstable();
    }
    for l in mundo.lugares() {
        if rutas::lugar_tiene(mundo, l.id(), "pequeno")
            && rutas::lugar_tiene(mundo, l.id(), "vasto")
        {
            return Err(fallo("tamaño contradictorio"));
        }
        for &vecino in mundo.lugares_conectados(l.id()).unwrap_or(&[]) {
            if rutas::lugar_tiene(mundo, l.id(), "acuatico")
                || rutas::lugar_tiene(mundo, vecino, "acuatico")
            {
                if !contexto
                    .rumbos
                    .0
                    .get(&(l.id(), vecino))
                    .is_some_and(|&r| r < 8)
                {
                    return Err(fallo("rumbo navegable ausente o inválido"));
                }
                if !mundo
                    .lugares()
                    .iter()
                    .any(|c| rutas::lugar_tiene(mundo, c.id(), "costero"))
                {
                    return Err(fallo("mundo navegable sin costas"));
                }
            }
        }
    }
    let rapida = rutas::rapida(mundo, lugar, destino, azar)?;
    let larga = rutas::larga(mundo, &rapida, azar)?;
    let mut ejecucion = EjecucionAccion {
        dirigente: actor,
        viajeros,
        destino,
        lugar,
        ruta: rapida.clone(),
        rapida,
        larga,
        posicion: 0,
        inicio: contexto.inicio,
        rumbos: contexto.rumbos.clone(),
        total: HorasViaje::CERO,
        h: HorasViaje::CERO,
        carencia_comida: None,
        carencia_agua: None,
        reservas: None,
        caballos: 0,
        camellos: 0,
        animales: 0,
        demoras: Vec::new(),
        visible: false,
        fase: Fase::Plan,
        ordinal: 0,
        espera: None,
    };
    let paso = ejecucion.producir(mundo, azar)?;
    Ok(InicioAccion { ejecucion, paso })
}
fn validar(
    e: &EjecucionAccion,
    paso: IdPaso,
    c: &ContextoAccion<'_>,
    decision: bool,
) -> Resultado<()> {
    if e.inicio != c.inicio || &e.rumbos != c.rumbos {
        return Err(fallo("contexto de ejecución modificado"));
    }
    if !e
        .espera
        .as_ref()
        .is_some_and(|(id, op)| *id == paso && op.is_empty() != decision)
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente));
    }
    Ok(())
}
pub fn avanzar(
    mundo: &mut Mundo,
    e: &mut EjecucionAccion,
    paso: IdPaso,
    c: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<PasoAccion> {
    validar(e, paso, c, false)?;
    let mut siguiente = e.clone();
    siguiente.espera = None;
    let salida = siguiente.producir(mundo, azar)?;
    *e = siguiente;
    Ok(salida)
}
pub fn responder(
    mundo: &mut Mundo,
    e: &mut EjecucionAccion,
    r: RespuestaDecision,
    c: &ContextoAccion<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<PasoAccion> {
    validar(e, r.paso, c, true)?;
    if !e
        .espera
        .as_ref()
        .is_some_and(|(_, op)| op.contains(&r.opcion))
    {
        return Err(ErrorMotor::Rechazo(RechazoAccion::OpcionNoVigente));
    }
    let mut s = e.clone();
    s.espera = None;
    match s.fase.clone() {
        Fase::ElegirRuta => {
            s.total = s.total.sumar(HorasViaje::horas(24))?;
            if r.opcion == OpcionViaje::Cancelar {
                s.fase = Fase::Terminar(CierreViaje::Cancelado);
            } else {
                s.visible = r.opcion == OpcionViaje::AceptarPlan;
                s.ruta = if s.visible {
                    s.rapida.clone()
                } else {
                    s.larga.clone()
                };
                s.fase = Fase::Preparar;
            }
        }
        Fase::ElegirCarga(p) => {
            let config = if r.opcion == OpcionViaje::AceptarDemora {
                p.alternativa
            } else {
                p.base
            };
            s.fase = Fase::ConfirmarPreparacion(p.chequeo, config);
        }
        Fase::ElegirAviso { inicial } => {
            s.fase = if r.opcion == OpcionViaje::Cancelar {
                Fase::Terminar(CierreViaje::Cancelado)
            } else if inicial {
                Fase::Mover
            } else {
                Fase::Entre {
                    revisar: true,
                    busco: true,
                }
            };
        }
        Fase::ElegirSeparacion(mut m) => {
            let incidente = m
                .dato
                .incidente
                .as_ref()
                .ok_or_else(|| fallo("separación sin incidente"))?;
            let actor = incidente
                .afectado
                .ok_or_else(|| fallo("separación sin Actor"))?;
            m.operaciones.push(OperacionMundo::SepararCompanero {
                actor,
                lugar: incidente.lugar,
            });
            m.viajeros.retain(|&x| x != actor);
            if r.opcion == OpcionViaje::Buscar {
                m.lugar = incidente.lugar;
                m.cierre = Some(CierreViaje::InterrumpidoPorSeparacion);
            }
            s.fase = Fase::CerrarMovimiento(m);
        }
        Fase::Descartar(lote) => {
            if r.opcion == OpcionViaje::Cancelar {
                s.fase = Fase::Terminar(CierreViaje::Cancelado);
            } else {
                lote.quitar(
                    s.reservas
                        .as_mut()
                        .ok_or_else(|| fallo("descarte sin reservas"))?,
                )?;
                s.recalcular_carga(mundo)?;
                s.fase = Fase::Mover;
            }
        }
        _ => return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente)),
    }
    let salida = s.producir(mundo, azar)?;
    *e = s;
    Ok(salida)
}

impl EjecucionAccion {
    fn registro(
        &mut self,
        mundo: &mut Mundo,
        tarjeta: ClaveTarjeta,
        contenido: ContenidoEscena,
        duracion: HorasViaje,
        efectos: EfectosEscena,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<RegistroEscena> {
        let EfectosEscena {
            participantes,
            operaciones,
        } = efectos;
        let total = self.total.sumar(duracion)?;
        let instante = proyectar_tiempo(self.inicio, total.tiempo()?)
            .map_err(|_| fallo("calendario desbordado"))?;
        let mut consumo = RegistroConsumo::default();
        if let Some(reservas) = &mut self.reservas {
            let (h, c) = reservas::consumir(reservas, self.h, duracion, participantes.len(), azar)?;
            self.h = h;
            consumo = c;
        }
        let partes = |t: crate::TiempoAccion| -> u128 {
            u128::from(t.duracion.dias()) * crate::calendario::PARTES_POR_DIA
                + u128::from(t.fraccion.partes())
        };
        if let Some(t) = consumo.sin_alimento {
            self.carencia_comida = Some(
                self.carencia_comida
                    .unwrap_or(HorasViaje::CERO)
                    .sumar(HorasViaje(partes(t)))?,
            );
        }
        if let Some(t) = consumo.sin_agua {
            self.carencia_agua = Some(
                self.carencia_agua
                    .unwrap_or(HorasViaje::CERO)
                    .sumar(HorasViaje(partes(t)))?,
            );
        }
        if tarjeta == ClaveTarjeta::Llegada {
            // Some(0) distingue la ración que faltó justo al llegar del saldo
            // que llegó a cero después de pagar completa la última ración.
            consumo.sin_alimento = self.carencia_comida.map(HorasViaje::tiempo).transpose()?;
            consumo.sin_agua = self.carencia_agua.map(HorasViaje::tiempo).transpose()?;
        }
        let id = IdPaso(self.ordinal);
        let estados_iniciales = participantes
            .iter()
            .filter_map(|&id| {
                mundo.actor(id).map(|a| {
                    (
                        id,
                        a.caracteristicas()
                            .filter(|c| matches!(c.como_str(), "esguince" | "cansado"))
                            .map(|c| c.como_str().to_owned())
                            .collect::<Vec<_>>(),
                    )
                })
            })
            .filter(|(_, estados)| !estados.is_empty())
            .collect();
        self.ordinal = self
            .ordinal
            .checked_add(1)
            .ok_or_else(|| fallo("ordinal de paso agotado"))?;
        let cambios = if operaciones.is_empty() {
            Vec::new()
        } else {
            mundo
                .aplicar_lote(
                    LoteOperaciones::nuevo(operaciones).map_err(|_| fallo("lote inválido"))?,
                )
                .map_err(|_| fallo("operación de Mundo inválida"))?
                .en_vec()
        };
        self.total = total;
        self.espera = Some((id, Vec::new()));
        Ok(RegistroEscena {
            id,
            tarjeta,
            contenido,
            participantes,
            composicion_resultante: self.viajeros.clone(),
            estados_iniciales,
            caballos: self.caballos,
            camellos: self.camellos,
            animales_carga: self.animales,
            lugar: self.lugar,
            destino_final: self.destino,
            siguiente: if self.visible && !matches!(self.fase, Fase::Terminar(_)) {
                self.ruta.get(self.posicion + 1).copied()
            } else {
                None
            },
            plan_visible: self.visible,
            duracion: duracion.tiempo()?,
            total: total.tiempo()?,
            instante,
            consumo,
            horas_desde_racion: self.h.tiempo()?,
            cierre: if let Fase::Terminar(cierre) = self.fase {
                Some(cierre)
            } else {
                None
            },
            reservas: self.reservas.clone(),
            cambios,
        })
    }
    fn escena(
        &mut self,
        mundo: &mut Mundo,
        tarjeta: ClaveTarjeta,
        contenido: ContenidoEscena,
        duracion: HorasViaje,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        Ok(PasoAccion::Escena(self.registro(
            mundo,
            tarjeta,
            contenido,
            duracion,
            EfectosEscena {
                participantes: self.viajeros.clone(),
                operaciones: Vec::new(),
            },
            azar,
        )?))
    }
    fn decision(
        &mut self,
        mundo: &mut Mundo,
        tarjeta: ClaveTarjeta,
        contenido: ContenidoEscena,
        opciones: Vec<OpcionViaje>,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        let escena = self.registro(
            mundo,
            tarjeta,
            contenido,
            HorasViaje::CERO,
            EfectosEscena {
                participantes: self.viajeros.clone(),
                operaciones: Vec::new(),
            },
            azar,
        )?;
        self.espera = Some((escena.id, opciones.clone()));
        Ok(PasoAccion::Decision(SolicitudDecision { escena, opciones }))
    }
    fn tiempos_restantes(&self, mundo: &Mundo) -> Resultado<Vec<[u64; 2]>> {
        rutas::tiempos(
            mundo,
            &self.ruta[self.posicion..],
            &self.viajeros,
            self.caballos,
            self.camellos,
            self.animales,
        )
    }
    fn restante(&self, mundo: &Mundo) -> Resultado<HorasViaje> {
        rutas::total(&self.tiempos_restantes(mundo)?)?.sumar(carga::suma_plan(
            self.demoras.get(self.posicion..).unwrap_or(&[]),
        )?)
    }
    fn necesarias(&self, mundo: &Mundo, horas_busqueda: u64) -> Resultado<u64> {
        u64::try_from(
            self.h
                .sumar(HorasViaje::horas(horas_busqueda))?
                .sumar(self.restante(mundo)?)?
                .jornadas(),
        )
        .map_err(|_| fallo("necesidad fuera de rango"))
    }
    fn recalcular_carga(&mut self, mundo: &Mundo) -> Resultado<()> {
        let reservas = self
            .reservas
            .as_ref()
            .ok_or_else(|| fallo("carga sin reservas"))?;
        let x = carga::exceso(reservas, self.viajeros.len(), self.animales)?;
        let nuevas = carga::plan(&self.tiempos_restantes(mundo)?, x)?;
        self.demoras.truncate(self.posicion);
        self.demoras.extend(nuevas);
        Ok(())
    }
    fn vista_config(&self, c: &Configuracion) -> Resultado<ConfiguracionVista> {
        let demora = carga::suma_plan(&c.demoras)?;
        Ok(ConfiguracionVista {
            reservas: c.reservas.clone(),
            animales: c.animales,
            exceso_gramos: c.exceso,
            peso_actores: c
                .reservas
                .peso()?
                .saturating_sub(multiplicar(c.animales, 100_000)?),
            limite_actores: multiplicar(self.viajeros.len() as u64, 10_000)?,
            tiempo: HorasViaje::horas(c.referencia).sumar(demora)?.tiempo()?,
            demora: demora.tiempo()?,
            suplemento: c.suplemento.clone(),
        })
    }
    fn producir(&mut self, mundo: &mut Mundo, azar: &mut dyn FuenteAzar) -> Resultado<PasoAccion> {
        loop {
            match self.fase.clone() {
                Fase::Plan => {
                    return self.proponer_ruta(mundo, azar);
                }
                Fase::Preparar => {
                    return self.preparar_viaje(mundo, azar);
                }
                Fase::ConfirmarPreparacion(chequeo, config) => {
                    return self.confirmar_preparacion(mundo, azar, chequeo, config);
                }
                Fase::Aviso { inicial } => {
                    return self.advertir_provisiones(mundo, azar, inicial);
                }
                Fase::Mover => {
                    let destino = *self
                        .ruta
                        .get(self.posicion + 1)
                        .ok_or_else(|| fallo("conexión pendiente ausente"))?;
                    let instante = proyectar_tiempo(self.inicio, self.total.tiempo()?)
                        .map_err(|_| fallo("fecha proyectada inválida"))?;
                    let carga = *self
                        .demoras
                        .get(self.posicion)
                        .ok_or_else(|| fallo("plan de carga ausente"))?;
                    let contexto = tarjetas::ContextoMovimiento {
                        mundo,
                        viajeros: &self.viajeros,
                        origen: self.lugar,
                        destino,
                        animales: self.animales,
                        caballos: self.caballos,
                        camellos: self.camellos,
                        carga,
                        mes: instante.fecha.mes().numero(),
                        rumbo: self.rumbos.0.get(&(self.lugar, destino)).copied(),
                    };
                    let medio = rutas::medio(
                        mundo,
                        self.lugar,
                        destino,
                        self.viajeros.len(),
                        self.caballos,
                        self.camellos,
                    );
                    let movimiento = match medio {
                        Medio::Pie => tarjetas::andar::resolver(&contexto, azar)?,
                        Medio::Caballo => tarjetas::cabalgar::resolver(&contexto, azar)?,
                        Medio::Camello => tarjetas::camello::resolver(&contexto, azar)?,
                        Medio::Barco => tarjetas::navegar::resolver(&contexto, azar)?,
                    };
                    if movimiento
                        .dato
                        .incidente
                        .as_ref()
                        .is_some_and(|i| i.tipo == Incidente::Separacion)
                    {
                        let contenido = ContenidoEscena::Desplazamiento(movimiento.dato.clone());
                        self.fase = Fase::ElegirSeparacion(Box::new(movimiento));
                        return self.decision(
                            mundo,
                            ClaveTarjeta::Andar,
                            contenido,
                            vec![OpcionViaje::Buscar, OpcionViaje::Continuar],
                            azar,
                        );
                    }
                    self.fase = Fase::CerrarMovimiento(Box::new(movimiento));
                }
                Fase::CerrarMovimiento(m) => {
                    let participantes = self.viajeros.clone();
                    let perdida = m.viajeros.len() < participantes.len();
                    self.viajeros = m.viajeros;
                    self.lugar = m.lugar;
                    self.caballos = m.caballos;
                    self.camellos = m.camellos;
                    let mut operaciones = m.operaciones;
                    if mundo.ubicacion_actor(self.dirigente) != Some(self.lugar) {
                        operaciones.push(OperacionMundo::Trasladar {
                            actor: self.dirigente,
                            destino: self.lugar,
                        });
                    }
                    self.posicion += 1;
                    self.fase = if let Some(cierre) = m.cierre {
                        Fase::Terminar(cierre)
                    } else {
                        Fase::Entre {
                            revisar: perdida,
                            busco: false,
                        }
                    };
                    let tarjeta = match m.dato.medio {
                        Medio::Pie => ClaveTarjeta::Andar,
                        Medio::Caballo => ClaveTarjeta::Caballo,
                        Medio::Camello => ClaveTarjeta::Camello,
                        Medio::Barco => ClaveTarjeta::Navegar,
                    };
                    return Ok(PasoAccion::Escena(self.registro(
                        mundo,
                        tarjeta,
                        ContenidoEscena::Desplazamiento(m.dato),
                        m.duracion,
                        EfectosEscena {
                            participantes,
                            operaciones,
                        },
                        azar,
                    )?));
                }
                Fase::Entre { revisar, busco } => {
                    if self.lugar == self.destino {
                        self.fase = Fase::Llegar;
                        continue;
                    }
                    if revisar {
                        self.recalcular_carga(mundo)?;
                    }
                    let reservas = self
                        .reservas
                        .as_ref()
                        .ok_or_else(|| fallo("viaje sin reservas"))?;
                    let x = carga::exceso(reservas, self.viajeros.len(), self.animales)?;
                    let insuficiente = revisar
                        && x > 0
                        && !reservas
                            .alcanza(self.viajeros.len() as u64, self.necesarias(mundo, 0)?)?;
                    if !busco && (reservas.agotada()? || insuficiente) {
                        self.fase = Fase::Forrajear;
                        continue;
                    }
                    if insuficiente {
                        return self.ofrecer_descarte(mundo, x, azar);
                    }
                    self.fase = Fase::Mover;
                }
                Fase::Forrajear => {
                    return self.forrajear(mundo, azar);
                }
                Fase::Llegar => {
                    return self.llegar(mundo, azar);
                }
                Fase::Terminar(cierre) => {
                    self.fase = Fase::Finalizada;
                    self.espera = None;
                    return Ok(PasoAccion::Finalizada(FinalizacionAccion {
                        tiempo: self.total.tiempo()?,
                        cierre,
                        lugar: self.lugar,
                    }));
                }
                _ => return Err(ErrorMotor::Rechazo(RechazoAccion::PasoNoVigente)),
            }
        }
    }
}
