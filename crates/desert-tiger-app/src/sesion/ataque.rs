use super::*;
use crate::encuentro::{FichaEncuentro, ObjetivoAtaque, TerrenoEncuentro, VistaEncuentro};
use desert_tiger_core::encuentros::{Celda, Desenlace, Encuentro, Escenario, Suceso};
use desert_tiger_core::{Caracteristica, ExtremoRelacion, OperacionMundo, TipoRelacion};
#[derive(Clone)]
pub(super) struct AtaqueActivo {
    id: u64,
    captor_rescate: Option<IdActor>,
    rescate_confirmado: bool,
    pub(super) motor: Encuentro,
    azar: ChaCha12Rng,
    pendiente: Option<VistaEncuentro>,
    secuencia: u64,
    fronteras: std::collections::BTreeSet<&'static str>,
}
pub(super) fn cautivo(m: &Mundo, a: IdActor) -> bool {
    m.relaciones()
        .iter()
        .any(|r| r.tipo() == TipoRelacion::PrisioneroDe && r.origen() == ExtremoRelacion::Actor(a))
}
pub(super) fn objetivos(m: &Mundo, c: &ContenidoValidado) -> Vec<ObjetivoAtaque> {
    if m.colectivo_de(m.actor_jugador()).is_some()
        || cautivo(m, m.actor_jugador())
        || m.actor(m.actor_jugador()).is_none_or(|a| {
            a.caracteristicas()
                .any(|c| matches!(c.como_str(), "muerto" | "incapacitado"))
        })
    {
        return vec![];
    }
    c.encuentros()
        .into_iter()
        .flat_map(|c| &c.objetivos)
        .filter_map(|o| {
            let actor = c.claves().actor_por_clave(&o.actor)?;
            if actor == m.actor_jugador()
                || m.grupo().contiene(actor)
                || m.ubicacion_actor(actor) != Some(m.ubicacion_grupo())
                || m.actor(actor)?
                    .caracteristicas()
                    .any(|v| v.como_str() == "muerto")
            {
                return None;
            }
            Some(ObjetivoAtaque {
                actor,
                nombre: c.presentacion().actor(actor)?.nombre().to_owned(),
            })
        })
        .collect()
}
pub(super) fn objetivos_rescate(m: &Mundo, c: &ContenidoValidado) -> Vec<ObjetivoAtaque> {
    if m.colectivo_de(m.actor_jugador()).is_some()
        || cautivo(m, m.actor_jugador())
        || m.actor(m.actor_jugador()).is_none_or(|a| {
            a.caracteristicas()
                .any(|c| matches!(c.como_str(), "muerto" | "incapacitado"))
        })
    {
        return vec![];
    }
    c.encuentros()
        .into_iter()
        .flat_map(|c| &c.rescates)
        .filter_map(|r| {
            let actor = c.claves().actor_por_clave(&r.actor)?;
            let captor = c.claves().actor_por_clave(&r.captor)?;
            let relaciones = m
                .relaciones()
                .iter()
                .filter(|r| {
                    r.tipo() == TipoRelacion::PrisioneroDe
                        && r.origen() == ExtremoRelacion::Actor(actor)
                })
                .collect::<Vec<_>>();
            if m.grupo().contiene(actor)
                || actor == m.actor_jugador()
                || m.colectivo_de(actor).is_some()
                || m.grupo().contiene(captor)
                || captor == m.actor_jugador()
                || m.ubicacion_actor(actor) != Some(m.ubicacion_grupo())
                || m.actor(actor)?
                    .caracteristicas()
                    .any(|c| c.como_str() == "muerto")
                || relaciones.len() != 1
                || relaciones[0].destino() != ExtremoRelacion::Actor(captor)
            {
                return None;
            }
            Some(ObjetivoAtaque {
                actor,
                nombre: c.presentacion().actor(actor)?.nombre().into(),
            })
        })
        .collect()
}
fn tiempo_encuentro(pulso: u64) -> Result<desert_tiger_core::TiempoAccion, String> {
    let segundos = 600u128 + u128::from(pulso) * 6;
    let partes = segundos * 2_400_000_000u128 / 86_400;
    Ok(desert_tiger_core::TiempoAccion {
        duracion: Duracion::desde_dias(
            u32::try_from(partes / 2_400_000_000).map_err(|_| "duración fuera de rango")?,
        ),
        fraccion: FraccionDia::nueva((partes % 2_400_000_000) as u32)
            .map_err(|_| "fracción fuera de rango")?,
    })
}
fn vista(a: &AtaqueActivo, c: &ContenidoValidado) -> VistaEncuentro {
    let m = &a.motor;
    let nombre = |actor| {
        c.presentacion()
            .actor(actor)
            .map(|a| a.nombre().to_owned())
            .unwrap_or_else(|| "Combatiente".into())
    };
    let finalizado = m.desenlace.is_some();
    let texto = if m.rescate {
        let quien = nombre(m.fichas[m.objetivo].actor);
        if let Some(fin) = m.desenlace {
            match fin {
                Desenlace::Exito=>format!("Logran sacar a {quien} y el grupo alcanza la salida. El rescate ha terminado."),
                Desenlace::Retirada=>format!("El grupo se retira sin conseguir sacar a {quien}."),
                Desenlace::Captura {captor}=>format!("El grupo queda fuera de combate. {} toma prisioneros a quienes no lograron salir.",nombre(captor)),
                Desenlace::Derrota=>"El rescate termina con el grupo fuera de combate. Las pérdidas y heridas permanecen.".into(),
            }
        } else if m.retirando {
            format!(
                "El grupo intenta retirarse del lugar. {quien} necesita alcanzar la salida para quedar a salvo."
            )
        } else if m.fichas[m.objetivo].muerto() {
            format!("{quien} ha muerto. El grupo intenta salir con sus supervivientes.")
        } else if !m.fichas[m.objetivo].cautivo {
            format!("{quien} ya está desatado. El grupo intenta llevarlo hasta la salida.")
        } else {
            format!(
                "El grupo busca alcanzar a {quien}, soltar sus ataduras y sacarlo del lugar. Los guardias intentarán impedirlo."
            )
        }
    } else if let Some(fin) = m.desenlace {
        match fin {
        Desenlace::Exito=>"El objetivo ha muerto. El grupo logra salir del lugar.".into(),
        Desenlace::Retirada=>"El grupo alcanza la salida y abandona el ataque. El objetivo sigue vivo.".into(),
        Desenlace::Captura{captor}=>format!("El grupo queda fuera de combate. {} toma prisioneros a los supervivientes.",nombre(captor)),
        Desenlace::Derrota=>"El enfrentamiento termina con el grupo fuera de combate. Sus pérdidas y heridas permanecen.".into(),
    }
    } else if m.retirando {
        "El grupo intenta retirarse y sacar a sus compañeros del lugar.".into()
    } else if m.fichas[m.objetivo].muerto() {
        "El objetivo ha caído. El grupo busca la salida.".into()
    } else if m.escenario != Escenario::Recinto {
        let (inicio, curso) = match m.escenario {
            Escenario::Palacio => (
                "Te preparás para entrar en un palacio de amplios salones, columnas y accesos vigilados.",
                "El grupo avanza entre columnas y salones para alcanzar a su objetivo.",
            ),
            Escenario::Ciudad => (
                "Te preparás para atacar entre las calles y callejones de la ciudad.",
                "El grupo se abre paso entre las manzanas y sus defensores.",
            ),
            Escenario::Bosque => (
                "Te preparás para atacar en un bosque. Los troncos obligan a buscar pasos entre los árboles.",
                "El grupo maniobra entre los troncos para alcanzar a su objetivo.",
            ),
            Escenario::Montana => (
                "Te preparás para atacar entre rocas y desniveles. Los vacíos dejan un paso estrecho por la ladera.",
                "El grupo avanza por los pasos de la ladera, entre las rocas y sus adversarios.",
            ),
            Escenario::Playa => (
                "Te preparás para atacar en la playa, entre rocas dispersas y la orilla. El agua limita el terreno transitable.",
                "El grupo avanza por la arena y busca paso entre las rocas de la costa.",
            ),
            _ => (
                "Te preparás para atacar un campamento entre las lomas. Más allá de las rocas, los defensores vigilan los dos pasos de un barranco seco.",
                "El grupo busca abrirse paso entre las rocas y los defensores para alcanzar a su objetivo.",
            ),
        };
        if m.pulso == 0 {
            inicio.into()
        } else {
            curso.into()
        }
    } else if m.descubierta {
        "El grupo entra en combate e intenta abrirse paso hasta su objetivo.".into()
    } else if m.pulso == 0 {
        "Te preparás para atacar. El objetivo espera al otro lado del recinto.".into()
    } else {
        "El grupo se aproxima al objetivo a través de los cuartos.".into()
    };
    let celdas = m
        .celdas
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if !m.descubierta && i % usize::from(m.ancho) > 15 {
                TerrenoEncuentro::Desconocido
            } else {
                match t {
                    Celda::Suelo => TerrenoEncuentro::Suelo,
                    Celda::Muro => TerrenoEncuentro::Muro,
                    Celda::Vacio => TerrenoEncuentro::Vacio,
                }
            }
        })
        .collect();
    let fichas = m
        .fichas
        .iter()
        .enumerate()
        .filter(|(_, f)| (f.aliado && !f.cautivo) || m.descubierta)
        .map(|(id, f)| FichaEncuentro {
            cautivo: f.cautivo,
            rescatado: m.rescate && id == m.objetivo,
            id,
            impacto: m
                .sucesos
                .iter()
                .filter_map(|s| match s {
                    Suceso::Golpe {
                        blanco, acierto, ..
                    } if *blanco == id => Some(*acierto),
                    _ => None,
                })
                .max(),
            nombre: nombre(f.actor),
            x: f.posicion.x,
            y: f.posicion.y,
            aliado: f.aliado,
            golpes: f.golpes,
            salio: f.salio,
            transportado: f.transportado_por.is_some(),
        })
        .collect();
    let detalle = m
        .sucesos
        .iter()
        .filter_map(|s| match s {
            Suceso::Golpe {
                atacante,
                blanco,
                acierto,
                reaccion,
            } => {
                let at = &m.fichas[*atacante];
                let bl = &m.fichas[*blanco];
                (m.descubierta || at.aliado || bl.aliado).then(|| {
                    format!(
                        "{} {} a {}{}.",
                        nombre(at.actor),
                        if *acierto { "alcanza" } else { "no alcanza" },
                        nombre(bl.actor),
                        if *reaccion {
                            " al intentar alejarse"
                        } else {
                            ""
                        }
                    )
                })
            }
            Suceso::Liberado { ficha, por } => Some(format!(
                "{} suelta las ataduras de {}. El grupo intenta llevarlo hasta la salida.",
                nombre(m.fichas[*por].actor),
                nombre(m.fichas[*ficha].actor)
            )),
            Suceso::Recogido { ficha, por } => Some(format!(
                "{} carga a {} para sacarlo.",
                nombre(m.fichas[*por].actor),
                nombre(m.fichas[*ficha].actor)
            )),
            _ => None,
        })
        .collect();
    VistaEncuentro {
        rescate: m.rescate,
        escenario: m.escenario,
        id: a.id,
        paso: a.secuencia,
        ancho: m.ancho,
        alto: m.alto,
        celdas,
        fichas,
        texto,
        detalle,
        ronda: m.pulso,
        finalizado,
        puede_retirar: a.pendiente.is_none(),
        pausa_decision: m.bloqueado,
        retirando: m.retirando,
    }
}
impl Partida {
    /// Vista local durante un relato suspendido, sin adelantar reglas ni azar.
    pub fn encuentro_suspendido(&self) -> Option<VistaEncuentro> {
        if !matches!(
            self.continuacion_evento,
            Some(eventos_partida::Continuacion::Encuentro)
        ) {
            return None;
        }
        let mut activo = self.ataque.as_ref()?.clone();
        if let Some((motor, _)) = &self.eventos.encuentro {
            activo.motor = motor.clone();
        }
        Some(vista(&activo, &self.contenido))
    }
    pub(super) fn restaurar_encuentro(&mut self) -> Result<(), String> {
        let (motor, _) = self
            .eventos
            .encuentro
            .take()
            .ok_or("tablero narrativo ausente")?;
        let activo = self.ataque.as_mut().ok_or("ataque ausente")?;
        activo.motor = motor;
        activo.secuencia = activo.secuencia.checked_add(1).ok_or("secuencia agotada")?;
        self.vista = VistaSesion::Encuentro(vista(activo, &self.contenido));
        Ok(())
    }
    fn publicar_frontera_encuentro(&mut self) -> Result<bool, String> {
        let a = self.ataque.as_mut().ok_or("ataque ausente")?;
        if a.pendiente.is_some() {
            return Ok(false);
        }
        let objetivo = &a.motor.fichas[a.motor.objetivo];
        let frontera = if !a.fronteras.contains("preparacion") {
            Some(("preparacion", ClasePunto::EncuentroAntesVineta))
        } else if (match a.motor.escenario {
            Escenario::Recinto | Escenario::Palacio => a.motor.descubierta,
            _ => a.motor.contacto_hostil(),
        }) && !a.fronteras.contains("contacto")
        {
            Some(("contacto", ClasePunto::EncuentroMomento))
        } else if objetivo.golpes == 1 && !a.fronteras.contains("herida") {
            Some(("herida", ClasePunto::EncuentroMomento))
        } else if a.motor.desenlace.is_some() && !a.fronteras.contains("conclusion") {
            Some(("conclusion", ClasePunto::EncuentroAntesCierre))
        } else {
            None
        };
        let Some((momento, clase)) = frontera else {
            return Ok(false);
        };
        let id_objetivo = objetivo.actor;
        let pulso = a.motor.pulso;
        let rescate = a.motor.rescate;
        a.fronteras.insert(momento);
        self.eventos.encuentro = Some((a.motor.clone(), momento.into()));
        let mut ctx = self.contexto_sesion(clase);
        ctx.accion = Some(if rescate { "rescatar" } else { "atacar" });
        ctx.instante = desert_tiger_core::proyectar_tiempo(self.inicio, tiempo_encuentro(pulso)?)
            .map_err(|e| e.to_string())?;
        ctx.objetivo = Some(crate::ExtremoRelacion::Actor(id_objetivo));
        self.abrir_eventos(ctx, eventos_partida::Continuacion::Encuentro)?;
        Ok(true)
    }
    pub(super) fn iniciar_ataque_local(&mut self, objetivo: IdActor) -> Result<(), String> {
        if !objetivos(&self.mundo, &self.contenido)
            .iter()
            .any(|o| o.actor == objetivo)
        {
            self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
            return Ok(());
        }
        self.crear_ataque_local(objetivo, false)
            .map_err(|e| format!("{e:?}"))?;
        self.publicar_frontera_encuentro()?;
        Ok(())
    }
    pub(super) fn iniciar_rescate_local(&mut self, objetivo: IdActor) -> Result<(), String> {
        if !objetivos_rescate(&self.mundo, &self.contenido)
            .iter()
            .any(|o| o.actor == objetivo)
        {
            self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
            return Ok(());
        }
        self.crear_ataque_local(objetivo, true)
            .map_err(|e| format!("{e:?}"))?;
        self.publicar_frontera_encuentro()?;
        Ok(())
    }
    fn crear_ataque_local(
        &mut self,
        objetivo: IdActor,
        rescate: bool,
    ) -> Result<(), ErrorIntencion> {
        let grupo = std::iter::once(self.mundo.actor_jugador())
            .chain(self.mundo.grupo().integrantes().iter().copied())
            .filter(|&id| {
                self.mundo
                    .actor(id)
                    .is_some_and(|a| !a.caracteristicas().any(|c| c.como_str() == "muerto"))
            })
            .collect::<Vec<_>>();
        let config = self
            .contenido
            .encuentros()
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        let (escenario, claves_defensa, captor_rescate) = if rescate {
            let r = config
                .rescates
                .iter()
                .find(|r| self.contenido.claves().actor_por_clave(&r.actor) == Some(objetivo))
                .ok_or(ErrorIntencion::FaseIncompatible)?;
            (
                r.escenario,
                std::iter::once(&r.captor)
                    .chain(&r.defensores)
                    .collect::<Vec<_>>(),
                self.contenido.claves().actor_por_clave(&r.captor),
            )
        } else {
            let r = config
                .objetivos
                .iter()
                .find(|r| self.contenido.claves().actor_por_clave(&r.actor) == Some(objetivo))
                .ok_or(ErrorIntencion::FaseIncompatible)?;
            (r.escenario, r.defensores.iter().collect::<Vec<_>>(), None)
        };
        let defensores = claves_defensa
            .into_iter()
            .filter_map(|k| self.contenido.claves().actor_por_clave(k))
            .filter(|&a| {
                a != objetivo
                    && !grupo.contains(&a)
                    && self.mundo.ubicacion_actor(a) == Some(self.mundo.ubicacion_grupo())
                    && self
                        .mundo
                        .actor(a)
                        .is_some_and(|a| !a.caracteristicas().any(|c| c.como_str() == "muerto"))
            })
            .collect::<Vec<_>>();
        let seed = self
            .azar
            .elegir_indice(NonZeroUsize::new(u32::MAX as usize).unwrap())
            .map_err(|_| ErrorIntencion::FaseIncompatible)? as u64;
        let mut azar = ChaCha12Rng::seed_from_u64(seed);
        let constructor = if rescate {
            Encuentro::nuevo_rescate
        } else {
            Encuentro::nuevo_con_escenario
        };
        let motor = constructor(
            &self.mundo,
            &grupo,
            objetivo,
            &defensores,
            &mut AzarChaCha(azar.clone()),
            escenario.motor(&self.mundo),
        )
        .map_err(|_| ErrorIntencion::AccionRechazada(RechazoAccion::DestinoInvalido))?;
        // Flujo separado de resolución, independiente del número de sorteos del generador.
        azar = ChaCha12Rng::seed_from_u64(seed ^ 0x617461717565);
        self.ordinal = self
            .ordinal
            .checked_add(1)
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        let activo = AtaqueActivo {
            captor_rescate,
            rescate_confirmado: false,
            id: self.ordinal,
            motor,
            azar,
            pendiente: None,
            secuencia: 0,
            fronteras: Default::default(),
        };
        self.inicio = InstanteAccion {
            fecha: self.mundo.fecha(),
            fraccion: self.resto,
        };
        self.vista = VistaSesion::Encuentro(vista(&activo, &self.contenido));
        self.ataque = Some(activo);
        Ok(())
    }
    pub(super) fn procesar_ataque(
        &mut self,
        i: IntencionFrontend,
    ) -> Result<VistaSesion, ErrorIntencion> {
        if let IntencionFrontend::ComenzarAtaque { objetivo }
        | IntencionFrontend::ComenzarRescate { objetivo } = i
        {
            let rescate = matches!(i, IntencionFrontend::ComenzarRescate { .. });
            if !matches!(self.vista, VistaSesion::Disponible(_)) {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            if !(if rescate {
                objetivos_rescate(&self.mundo, &self.contenido)
            } else {
                objetivos(&self.mundo, &self.contenido)
            })
            .iter()
            .any(|o| o.actor == objetivo)
            {
                return Err(ErrorIntencion::AccionRechazada(
                    RechazoAccion::DestinoInvalido,
                ));
            }
            let mut ctx = self.contexto_sesion(if rescate {
                ClasePunto::RescatarInicio
            } else {
                ClasePunto::AtacarInicio
            });
            ctx.accion = Some(if rescate { "rescatar" } else { "atacar" });
            ctx.objetivo = Some(crate::ExtremoRelacion::Actor(objetivo));
            if let Err(e) = self.abrir_eventos(
                ctx,
                if rescate {
                    eventos_partida::Continuacion::IniciarRescate(objetivo)
                } else {
                    eventos_partida::Continuacion::IniciarAtaque(objetivo)
                },
            ) {
                self.fatal(CodigoFatalSesion::EventoNarrativo, e);
            }
            return Ok(self.vista());
        }
        let (id, paso) = match i {
            IntencionFrontend::AvanzarEncuentro { id, paso }
            | IntencionFrontend::RetirarEncuentro { id, paso }
            | IntencionFrontend::ReanudarEncuentro { id, paso }
            | IntencionFrontend::CerrarEncuentro { id, paso } => (id, paso),
            _ => return Err(ErrorIntencion::FaseIncompatible),
        };
        if !matches!(&self.vista,VistaSesion::Encuentro(v) if v.id==id&&v.paso==paso) {
            return Err(ErrorIntencion::FragmentoNoVigente);
        }
        if matches!(
            i,
            IntencionFrontend::AvanzarEncuentro { .. } | IntencionFrontend::CerrarEncuentro { .. }
        ) {
            match self.publicar_frontera_encuentro() {
                Ok(true) => return Ok(self.vista()),
                Ok(false) => {}
                Err(e) => {
                    self.fatal(CodigoFatalSesion::EventoNarrativo, e);
                    return Ok(self.vista());
                }
            }
        }
        let a = self
            .ataque
            .as_ref()
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        if matches!(i, IntencionFrontend::CerrarEncuentro { .. }) {
            if a.motor.desenlace.is_none() || a.pendiente.is_some() {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            let tiempo =
                tiempo_encuentro(a.motor.pulso).map_err(|_| ErrorIntencion::FaseIncompatible)?;
            if let Err(e) = self.concluir_accion(tiempo) {
                self.fatal(e.codigo(), e.detalle());
            }
            return Ok(self.vista());
        }
        let mut nuevo = a.clone();
        nuevo.secuencia = nuevo
            .secuencia
            .checked_add(1)
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        if matches!(
            i,
            IntencionFrontend::RetirarEncuentro { .. }
                | IntencionFrontend::ReanudarEncuentro { .. }
        ) {
            // Un frame pendiente se muestra antes de aceptar otra decisión.
            if nuevo.pendiente.is_some() {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            nuevo
                .motor
                .reconsiderar(matches!(i, IntencionFrontend::RetirarEncuentro { .. }));
            self.vista = VistaSesion::Encuentro(vista(&nuevo, &self.contenido));
            self.ataque = Some(nuevo);
            return Ok(self.vista());
        }
        if let Some(mut pendiente) = nuevo.pendiente.take() {
            pendiente.paso = nuevo.secuencia;
            self.vista = VistaSesion::Encuentro(pendiente);
            self.ataque = Some(nuevo);
            if let Err(e) = self.publicar_frontera_encuentro() {
                self.fatal(CodigoFatalSesion::EventoNarrativo, e);
            }
            return Ok(self.vista());
        }
        if nuevo.motor.desenlace.is_some() || nuevo.motor.bloqueado {
            return Err(ErrorIntencion::FaseIncompatible);
        }
        let posiciones = nuevo
            .motor
            .fichas
            .iter()
            .map(|f| f.posicion)
            .collect::<Vec<_>>();
        let mut azar = AzarChaCha(nuevo.azar.clone());
        if let Err(e) = nuevo.motor.avanzar(&mut azar) {
            self.fatal(CodigoFatalSesion::Motor, e);
            return Ok(self.vista());
        }
        nuevo.azar = azar.0;
        let mut operaciones = vec![];
        if nuevo.motor.rescatado_fuera() && !nuevo.rescate_confirmado {
            let actor = nuevo.motor.fichas[nuevo.motor.objetivo].actor;
            operaciones.push(OperacionMundo::ConfirmarRescate {
                actor,
                captor: nuevo
                    .captor_rescate
                    .ok_or(ErrorIntencion::FaseIncompatible)?,
                rescatador: self.mundo.actor_jugador(),
            });
            operaciones.push(OperacionMundo::IncorporarCompanero { actor });
            nuevo.rescate_confirmado = true;
        }
        for f in &nuevo.motor.fichas {
            let actual = self
                .mundo
                .actor(f.actor)
                .ok_or(ErrorIntencion::FaseIncompatible)?;
            for (tag, activa) in [
                ("herido", f.golpes >= 1),
                ("incapacitado", f.golpes == 2),
                ("muerto", f.golpes >= 3),
            ] {
                let tiene = actual.caracteristicas().any(|c| c.como_str() == tag);
                if activa && !tiene {
                    operaciones.push(OperacionMundo::AnadirCaracteristicaActor {
                        actor: f.actor,
                        caracteristica: Caracteristica::nueva(tag.into())
                            .map_err(|_| ErrorIntencion::FaseIncompatible)?,
                    });
                }
                if !activa && tiene {
                    operaciones.push(OperacionMundo::QuitarCaracteristicaActor {
                        actor: f.actor,
                        caracteristica: Caracteristica::nueva(tag.into())
                            .map_err(|_| ErrorIntencion::FaseIncompatible)?,
                    });
                }
            }
        }
        if let Some(Desenlace::Captura { captor }) = nuevo.motor.desenlace {
            for f in nuevo.motor.fichas.iter().filter(|f| {
                f.aliado
                    && !f.salio
                    && !f.muerto()
                    && (!nuevo.motor.rescate
                        || f.actor != nuevo.motor.fichas[nuevo.motor.objetivo].actor)
            }) {
                operaciones.push(OperacionMundo::RegistrarCautiverio {
                    actor: f.actor,
                    captor,
                });
            }
        }
        let preparado = match self.mundo.preparar_operaciones(operaciones) {
            Ok(p) => p,
            Err(e) => {
                self.fatal(CodigoFatalSesion::Motor, e.to_string());
                return Ok(self.vista());
            }
        };
        let mut movimiento = vista(&nuevo, &self.contenido);
        let mut impacto = movimiento.clone();
        if !movimiento.detalle.is_empty() {
            // La reacción/ataque se ve en el origen; después se muestra el movimiento colectivo.
            for (f, _) in nuevo
                .motor
                .fichas
                .iter()
                .enumerate()
                .filter(|(_, f)| (f.aliado && !f.cautivo) || nuevo.motor.descubierta)
            {
                if let Some(vf) = impacto.fichas.iter_mut().find(|vf| vf.id == f) {
                    vf.x = posiciones[f].x;
                    vf.y = posiciones[f].y;
                }
            }
            impacto.puede_retirar = false;
            impacto.finalizado = false;
            impacto.pausa_decision = false;
            for ficha in &mut movimiento.fichas {
                ficha.impacto = None;
            }
            nuevo.pendiente = Some(movimiento);
        }
        preparado.confirmar(&mut self.mundo);
        self.vista = VistaSesion::Encuentro(impacto);
        self.ataque = Some(nuevo);
        if let Err(e) = self.publicar_frontera_encuentro() {
            self.fatal(CodigoFatalSesion::EventoNarrativo, e);
        }
        Ok(self.vista())
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_core::{LoteOperaciones, UbicacionActor};

    fn partida() -> (Partida, IdActor) {
        let c = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let objetivo = c.claves().actor_por_clave("miraz").unwrap();
        let mut p = Partida::nueva(&c, crate::ConfiguracionPartida { semilla: Some(42) }).unwrap();
        for _ in 0..100 {
            let intencion = match p.vista() {
                VistaSesion::Disponible(_) => break,
                VistaSesion::PresentandoEvento(v) => {
                    IntencionFrontend::ContinuarEvento { presentacion: v.id }
                }
                VistaSesion::EsperandoDecisionEvento(v) => IntencionFrontend::ResponderEvento {
                    presentacion: v.id,
                    opcion: v.opciones[0].id,
                },
                VistaSesion::ProcesandoCierre { token } => {
                    IntencionFrontend::ContinuarCierre { token }
                }
                otra => panic!("prólogo inesperado: {otra:?}"),
            };
            p.procesar(intencion).unwrap();
        }
        let lugar = match p.mundo.actor(objetivo).unwrap().ubicacion() {
            UbicacionActor::EnLugar(l) => l,
            _ => panic!("objetivo fuera del mapa"),
        };
        p.mundo
            .aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
                actor: p.mundo.actor_jugador(),
                destino: lugar,
            }))
            .unwrap();
        p.mundo_visible =
            proyectar_mundo(&p.mundo, &p.mapa, &p.contenido, &p.vision, None).unwrap();
        p.vista = VistaSesion::Disponible(p.mundo_visible.clone());
        (p, objetivo)
    }
    fn avanzar(p: &mut Partida) {
        let VistaSesion::Encuentro(v) = p.vista() else {
            panic!("no hay encuentro")
        };
        p.procesar(IntencionFrontend::AvanzarEncuentro {
            id: v.id,
            paso: v.paso,
        })
        .unwrap();
    }
    fn cierre(p: &mut Partida) {
        let VistaSesion::Encuentro(v) = p.vista() else {
            panic!()
        };
        p.procesar(IntencionFrontend::CerrarEncuentro {
            id: v.id,
            paso: v.paso,
        })
        .unwrap();
        for _ in 0..100 {
            match p.vista() {
                VistaSesion::Disponible(_) => return,
                VistaSesion::PresentandoEvento(v) => {
                    p.procesar(IntencionFrontend::ContinuarEvento { presentacion: v.id })
                        .unwrap();
                }
                VistaSesion::ProcesandoCierre { token } => {
                    p.procesar(IntencionFrontend::ContinuarCierre { token })
                        .unwrap();
                }
                otra => panic!("cierre inesperado: {otra:?}"),
            }
        }
        panic!("cierre sin terminar");
    }
    fn preparar_ogro(p: &mut Partida) -> IdActor {
        let destino = p.contenido.claves().lugar_por_clave("costa_ambar").unwrap();
        p.mundo
            .aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
                actor: p.mundo.actor_jugador(),
                destino,
            }))
            .unwrap();
        p.mundo_visible =
            proyectar_mundo(&p.mundo, &p.mapa, &p.contenido, &p.vision, None).unwrap();
        p.vista = VistaSesion::Disponible(p.mundo_visible.clone());
        p.contenido.claves().actor_por_clave("ogro_gorun").unwrap()
    }
    fn preparar_rescate(p: &mut Partida, sin_guardias: bool) -> IdActor {
        let destino = p
            .contenido
            .claves()
            .lugar_por_clave("lomas_de_anir")
            .unwrap();
        let mut ops = vec![OperacionMundo::Trasladar {
            actor: p.mundo.actor_jugador(),
            destino,
        }];
        if sin_guardias {
            for clave in [
                "jefe_bandido_kazir",
                "bandido_nazur",
                "bandido_hazek",
                "bandido_sufan",
            ] {
                ops.push(OperacionMundo::AnadirCaracteristicaActor {
                    actor: p.contenido.claves().actor_por_clave(clave).unwrap(),
                    caracteristica: Caracteristica::nueva("muerto".into()).unwrap(),
                });
            }
        }
        p.mundo
            .aplicar_lote(LoteOperaciones::nuevo(ops).unwrap())
            .unwrap();
        p.mundo_visible =
            proyectar_mundo(&p.mundo, &p.mapa, &p.contenido, &p.vision, None).unwrap();
        p.vista = VistaSesion::Disponible(p.mundo_visible.clone());
        p.contenido.claves().actor_por_clave("tamin").unwrap()
    }
    #[test]
    fn rescate_sesion_con_guardias_tiene_cierre_valido() {
        let (mut p, _) = partida();
        let objetivo = preparar_rescate(&mut p, false);
        p.procesar(IntencionFrontend::ComenzarRescate { objetivo })
            .unwrap();
        for _ in 0..1000 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            if let VistaSesion::Encuentro(v) = p.vista() {
                if v.pausa_decision {
                    p.procesar(IntencionFrontend::RetirarEncuentro {
                        id: v.id,
                        paso: v.paso,
                    })
                    .unwrap();
                }
            }
            avanzar(&mut p);
        }
        assert!(matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado));
        let fuera = p.ataque.as_ref().unwrap().motor.rescatado_fuera();
        assert_eq!(p.mundo.grupo().contiene(objetivo), fuera);
        assert_eq!(cautivo(&p.mundo, objetivo), !fuera);
        cierre(&mut p);
    }
    #[test]
    fn rescate_sesion_tamin_sin_guardias_confirma_y_no_se_repite() {
        let (mut p, _) = partida();
        let objetivo = preparar_rescate(&mut p, true);
        assert!(
            objetivos_rescate(&p.mundo, &p.contenido)
                .iter()
                .any(|o| o.actor == objetivo)
        );
        let antes = p.resto;
        p.procesar(IntencionFrontend::ComenzarRescate { objetivo })
            .unwrap();
        assert!(
            matches!(p.vista(),VistaSesion::Encuentro(v) if v.rescate && v.fichas.iter().any(|f|f.cautivo && f.rescatado))
        );
        for _ in 0..400 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            avanzar(&mut p);
        }
        assert_eq!(
            p.ataque.as_ref().unwrap().motor.desenlace,
            Some(Desenlace::Exito)
        );
        assert!(!cautivo(&p.mundo, objetivo));
        assert!(p.mundo.grupo().contiene(objetivo));
        assert!(
            p.mundo
                .relaciones()
                .iter()
                .any(|r| r.tipo() == TipoRelacion::SalvadoPor
                    && r.origen() == ExtremoRelacion::Actor(objetivo)
                    && r.destino() == ExtremoRelacion::Actor(p.mundo.actor_jugador()))
        );
        assert!(objetivos_rescate(&p.mundo, &p.contenido).is_empty());
        assert_eq!(p.resto, antes);
        cierre(&mut p);
        assert_ne!(p.resto, antes);
        assert!(
            p.procesar(IntencionFrontend::ComenzarRescate { objetivo })
                .is_err()
        );
    }
    #[test]
    fn rescate_sesion_retirada_guarda_cautiverio_y_guarnicion() {
        let (mut p, _) = partida();
        let objetivo = preparar_rescate(&mut p, false);
        p.procesar(IntencionFrontend::ComenzarRescate { objetivo })
            .unwrap();
        assert_eq!(
            p.ataque
                .as_ref()
                .unwrap()
                .motor
                .fichas
                .iter()
                .filter(|f| !f.aliado)
                .count(),
            4
        );
        let VistaSesion::Encuentro(v) = p.vista() else {
            panic!()
        };
        p.procesar(IntencionFrontend::RetirarEncuentro {
            id: v.id,
            paso: v.paso,
        })
        .unwrap();
        for _ in 0..150 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            avanzar(&mut p);
        }
        assert_eq!(
            p.ataque.as_ref().unwrap().motor.desenlace,
            Some(Desenlace::Retirada)
        );
        cierre(&mut p);
        assert!(cautivo(&p.mundo, objetivo));
        assert!(!p.mundo.grupo().contiene(objetivo));
    }
    #[test]
    fn escenarios_seleccionan_terreno_y_palacios_explicitos() {
        let (mut p, _) = partida();
        use crate::encuentro::EscenarioConfigurado;
        for (lugar, esperado) in [
            ("zarabad", Escenario::Ciudad),
            ("bosque_oscuro", Escenario::Bosque),
            ("monte_tazir", Escenario::Montana),
            ("costa_ambar", Escenario::Playa),
        ] {
            let destino = p.contenido.claves().lugar_por_clave(lugar).unwrap();
            p.mundo
                .aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
                    actor: p.mundo.actor_jugador(),
                    destino,
                }))
                .unwrap();
            assert_eq!(EscenarioConfigurado::Automatico.motor(&p.mundo), esperado);
        }
        for clave in ["zahrim", "miraz"] {
            let perfil = p
                .contenido
                .encuentros()
                .unwrap()
                .objetivos
                .iter()
                .find(|o| o.actor == clave)
                .unwrap();
            assert_eq!(perfil.escenario.motor(&p.mundo), Escenario::Palacio);
        }
    }
    #[test]
    fn ciclo3_bandidos_campamento_abierto_termina_y_no_libera_cautivo() {
        let (mut p, _) = partida();
        let destino = p
            .contenido
            .claves()
            .lugar_por_clave("lomas_de_anir")
            .unwrap();
        p.mundo
            .aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
                actor: p.mundo.actor_jugador(),
                destino,
            }))
            .unwrap();
        p.mundo_visible =
            proyectar_mundo(&p.mundo, &p.mapa, &p.contenido, &p.vision, None).unwrap();
        p.vista = VistaSesion::Disponible(p.mundo_visible.clone());
        let jefe = p
            .contenido
            .claves()
            .actor_por_clave("jefe_bandido_kazir")
            .unwrap();
        let preso = p.contenido.claves().actor_por_clave("tamin").unwrap();
        p.procesar(IntencionFrontend::ComenzarAtaque { objetivo: jefe })
            .unwrap();
        let VistaSesion::Encuentro(inicial) = p.vista() else {
            panic!()
        };
        assert!(!inicial.celdas.contains(&TerrenoEncuentro::Desconocido));
        assert!(inicial.texto.contains("campamento"));
        let motor = &p.ataque.as_ref().unwrap().motor;
        assert_eq!(motor.escenario, Escenario::Campamento);
        assert_eq!(motor.fichas.iter().filter(|f| !f.aliado).count(), 4);
        assert!(motor.fichas.iter().all(|f| f.actor != preso));
        let antes = motor.fichas.clone();
        avanzar(&mut p);
        let despues = &p.ataque.as_ref().unwrap().motor.fichas;
        for aliado in [true, false] {
            assert!(
                antes
                    .iter()
                    .zip(despues)
                    .any(|(a, b)| a.aliado == aliado && a.posicion != b.posicion)
            );
        }
        for _ in 0..800 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            assert!(
                !matches!(p.vista(),VistaSesion::Encuentro(v) if v.pausa_decision),
                "campamento bloqueado"
            );
            avanzar(&mut p);
        }
        assert!(matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado));
        cierre(&mut p);
        assert!(cautivo(&p.mundo, preso));
        assert!(
            !p.mundo
                .actor(preso)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "muerto")
        );
        let ogro = p
            .contenido
            .encuentros()
            .unwrap()
            .objetivos
            .iter()
            .find(|o| o.actor == "ogro_gorun")
            .unwrap();
        assert_eq!(ogro.escenario.motor(&p.mundo), Escenario::Recinto);
    }
    #[test]
    fn ciclo2_efectos_mixtos_atomicos_y_reubicacion() {
        fn copiar(a: &std::path::Path, b: &std::path::Path) {
            std::fs::create_dir_all(b).unwrap();
            for e in std::fs::read_dir(a).unwrap() {
                let e = e.unwrap();
                let destino = b.join(e.file_name());
                if e.path().is_dir() {
                    copiar(&e.path(), &destino);
                } else {
                    std::fs::copy(e.path(), destino).unwrap();
                }
            }
        }
        for (valido, x, y) in [(false, 0, 0), (true, 20, 8)] {
            let (mut p, _) = partida();
            let ogro = preparar_ogro(&mut p);
            let raiz =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base");
            let tmp = std::env::temp_dir()
                .join(format!("dt-ciclo2-mixto-{}-{valido}", std::process::id()));
            copiar(&raiz, &tmp);
            let fuente = format!(
                r##"(
                clave:"prueba_mixta",prioridad:10000,puntos:["encuentro.antes_vineta"],
                filtros:(objetivo:Some(Actor("ogro_gorun"))),condicion:"return true",inicio:"cambiar",
                pasos:[Efectos(clave:"cambiar",salidas:["fin"],resolver:r#"return {{
                    operaciones={{op.anadir_caracteristica('samir','herido')}},
                    encuentro={{op_encuentro.terreno(15,7,'suelo'),op_encuentro.reubicar('ogro_gorun',{x},{y})}},
                    siguiente='fin'}}"#),Fin(clave:"fin",control:Continuar)]
            )"##
            );
            std::fs::write(tmp.join("events/prueba_mixta.ron"), fuente).unwrap();
            p.contenido = crate::cargar_contenido(&tmp).unwrap();
            std::fs::remove_dir_all(tmp).unwrap();
            p.procesar(IntencionFrontend::ComenzarAtaque { objetivo: ogro })
                .unwrap();
            let samir = p.contenido.claves().actor_por_clave("samir").unwrap();
            let herido = p
                .mundo
                .actor(samir)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "herido");
            assert_eq!(herido, valido);
            let local = &p.eventos.encuentro.as_ref().unwrap().0;
            assert_eq!(
                local.terreno(desert_tiger_core::encuentros::Posicion {
                    area: 0,
                    nivel: 0,
                    x: 15,
                    y: 7
                }),
                Some(if valido { Celda::Suelo } else { Celda::Muro })
            );
            let ficha = local.fichas.iter().find(|f| f.actor == ogro).unwrap();
            assert_eq!(ficha.posicion.x, if valido { 20 } else { 27 });
            if valido {
                assert!(matches!(p.vista(), VistaSesion::PresentandoEvento(_)));
            } else {
                assert!(matches!(p.vista(), VistaSesion::Fatal(_)));
            }
        }
    }
    #[test]
    fn ciclo2_ogro_transformacion_continua_hasta_cierre_sin_repetir() {
        let (mut p, _) = partida();
        let ogro = preparar_ogro(&mut p);
        let antes = p.resto;
        p.procesar(IntencionFrontend::ComenzarAtaque { objetivo: ogro })
            .unwrap();
        let mut escenas = std::collections::BTreeSet::new();
        let mut vio_brecha = false;
        for _ in 0..1000 {
            match p.vista() {
                VistaSesion::PresentandoEvento(v) => {
                    let local = p
                        .encuentro_suspendido()
                        .expect("tablero conservado durante el relato");
                    escenas.insert(v.evento.clone());
                    if v.evento == "gorun_contacto" && v.paso == "brecha" {
                        assert_eq!(local.celdas[7 * 31 + 15], TerrenoEncuentro::Suelo);
                        assert_eq!(local.celdas[9 * 31 + 15], TerrenoEncuentro::Suelo);
                        vio_brecha = true;
                    }
                    let motor = p.ataque.as_ref().unwrap().motor.clone();
                    let primero = IntencionFrontend::ContinuarEvento { presentacion: v.id };
                    p.procesar(primero).unwrap();
                    let estado = p.vista();
                    assert!(p.procesar(primero).is_err());
                    assert_eq!(p.vista(), estado);
                    assert_eq!(p.ataque.as_ref().unwrap().motor.pulso, motor.pulso);
                }
                VistaSesion::Encuentro(v) if v.finalizado => break,
                VistaSesion::Encuentro(v) => {
                    assert!(!v.pausa_decision, "encuentro bloqueado");
                    avanzar(&mut p);
                }
                otra => panic!("estado inesperado: {otra:?}"),
            }
            assert_eq!(p.resto, antes, "leer o avanzar no confirma tiempo mundial");
        }
        assert!(escenas.contains("gorun_preparacion"));
        assert!(escenas.contains("gorun_contacto"));
        assert!(escenas.contains("gorun_conclusion"));
        assert!(vio_brecha);
        assert!(matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado));
        let VistaSesion::Encuentro(fin) = p.vista() else {
            panic!()
        };
        let motor = p.ataque.as_ref().unwrap().motor.clone();
        assert!(
            p.procesar(IntencionFrontend::AvanzarEncuentro {
                id: fin.id,
                paso: fin.paso
            })
            .is_err()
        );
        assert_eq!(p.ataque.as_ref().unwrap().motor, motor);
        cierre(&mut p);
        assert!(p.ataque.is_none());
        assert!(p.eventos.encuentro.is_none());
        assert_ne!(p.resto, antes);
    }
    #[test]
    fn objetivos_de_prueba_ogro_banda_y_cautivo() {
        let (mut p, _) = partida();
        for (lugar, clave, defensores) in [
            ("costa_ambar", "ogro_gorun", 0),
            ("lomas_de_anir", "jefe_bandido_kazir", 3),
        ] {
            let destino = p.contenido.claves().lugar_por_clave(lugar).unwrap();
            p.mundo
                .aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
                    actor: p.mundo.actor_jugador(),
                    destino,
                }))
                .unwrap();
            let objetivo = p.contenido.claves().actor_por_clave(clave).unwrap();
            assert!(
                objetivos(&p.mundo, &p.contenido)
                    .iter()
                    .any(|o| o.actor == objetivo)
            );
            let perfil = p
                .contenido
                .encuentros()
                .unwrap()
                .objetivos
                .iter()
                .find(|o| o.actor == clave)
                .unwrap();
            assert_eq!(perfil.defensores.len(), defensores);
        }
        let preso = p.contenido.claves().actor_por_clave("tamin").unwrap();
        assert!(cautivo(&p.mundo, preso));
        assert!(
            !objetivos(&p.mundo, &p.contenido)
                .iter()
                .any(|o| o.actor == preso)
        );
    }
    #[test]
    fn ataque_sesion_retirada_tokens_y_tiempo_una_vez() {
        let (mut p, objetivo) = partida();
        let antes = p.resto;
        let v = p
            .procesar(IntencionFrontend::ComenzarAtaque { objetivo })
            .unwrap();
        let VistaSesion::Encuentro(v) = v else {
            panic!("{v:?}")
        };
        assert!(v.fichas.iter().all(|f| f.aliado));
        assert!(v.celdas.contains(&TerrenoEncuentro::Desconocido));
        let viejo = IntencionFrontend::AvanzarEncuentro {
            id: v.id,
            paso: v.paso,
        };
        p.procesar(viejo).unwrap();
        let estado = p.ataque.as_ref().unwrap().motor.clone();
        assert!(p.procesar(viejo).is_err());
        assert_eq!(p.ataque.as_ref().unwrap().motor, estado);
        let VistaSesion::Encuentro(v) = p.vista() else {
            panic!()
        };
        p.procesar(IntencionFrontend::RetirarEncuentro {
            id: v.id,
            paso: v.paso,
        })
        .unwrap();
        for _ in 0..200 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            avanzar(&mut p);
        }
        assert_eq!(
            p.ataque.as_ref().unwrap().motor.desenlace,
            Some(Desenlace::Retirada)
        );
        let VistaSesion::Encuentro(v) = p.vista() else {
            panic!()
        };
        let duplicado = IntencionFrontend::CerrarEncuentro {
            id: v.id,
            paso: v.paso,
        };
        cierre(&mut p);
        assert_ne!(antes, p.resto);
        let tiempo = p.resto;
        assert!(p.ataque.is_none());
        assert!(p.procesar(duplicado).is_err());
        assert_eq!(tiempo, p.resto);
    }
    #[test]
    fn ataque_sesion_episodio_completo_sin_guion_de_resultado() {
        let (mut p, objetivo) = partida();
        p.procesar(IntencionFrontend::ComenzarAtaque { objetivo })
            .unwrap();
        for _ in 0..800 {
            if matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
                break;
            }
            assert!(
                !matches!(p.vista(),VistaSesion::Encuentro(v) if v.pausa_decision),
                "episodio bloqueado"
            );
            avanzar(&mut p);
        }
        assert!(matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado));
        assert_eq!(
            p.ataque.as_ref().unwrap().motor.desenlace,
            Some(Desenlace::Exito)
        );
        cierre(&mut p);
        assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    }
    #[test]
    fn ataque_sesion_captura_persistente_y_rechazo_viaje() {
        let (mut p, objetivo) = partida();
        p.procesar(IntencionFrontend::ComenzarAtaque { objetivo })
            .unwrap();
        // Preparamos la frontera de derrota para verificar la integración, no la probabilidad del combate.
        for f in &mut p.ataque.as_mut().unwrap().motor.fichas {
            if f.aliado {
                f.golpes = 2;
            }
        }
        avanzar(&mut p);
        assert!(cautivo(&p.mundo, p.mundo.actor_jugador()));
        if !matches!(p.vista(),VistaSesion::Encuentro(v) if v.finalizado) {
            avanzar(&mut p);
        }
        cierre(&mut p);
        assert!(objetivos(&p.mundo, &p.contenido).is_empty());
        assert!(
            p.procesar(IntencionFrontend::ComenzarViaje {
                destino: p.mundo.ubicacion_grupo()
            })
            .is_err()
        );
    }
    #[test]
    fn ataque_sesion_descanso_recupera_sin_revivir_ni_liberar() {
        let (mut p, captor) = partida();
        let actor = p.mundo.actor_jugador();
        let companero = p.mundo.grupo().integrantes()[0];
        p.mundo
            .aplicar_lote(
                LoteOperaciones::nuevo(vec![
                    OperacionMundo::AnadirCaracteristicaActor {
                        actor,
                        caracteristica: Caracteristica::nueva("herido".into()).unwrap(),
                    },
                    OperacionMundo::AnadirCaracteristicaActor {
                        actor,
                        caracteristica: Caracteristica::nueva("incapacitado".into()).unwrap(),
                    },
                    OperacionMundo::AnadirCaracteristicaActor {
                        actor: companero,
                        caracteristica: Caracteristica::nueva("muerto".into()).unwrap(),
                    },
                    OperacionMundo::RegistrarCautiverio { actor, captor },
                ])
                .unwrap(),
            )
            .unwrap();
        p.vista = VistaSesion::Disponible(
            proyectar_mundo(&p.mundo, &p.mapa, &p.contenido, &p.vision, None).unwrap(),
        );
        assert!(
            p.procesar(IntencionFrontend::ComenzarViaje {
                destino: p.mundo.ubicacion_grupo()
            })
            .is_err()
        );
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!()
        };
        p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
            .unwrap();
        for _ in 0..100 {
            match p.vista() {
                VistaSesion::Disponible(_) => break,
                VistaSesion::ProcesandoCierre { token } => {
                    p.procesar(IntencionFrontend::ContinuarCierre { token })
                        .unwrap();
                }
                otra => panic!("descanso inesperado: {otra:?}"),
            }
        }
        let caracteristicas = p
            .mundo
            .actor(actor)
            .unwrap()
            .caracteristicas()
            .map(|c| c.como_str())
            .collect::<Vec<_>>();
        assert!(caracteristicas.contains(&"herido"));
        assert!(!caracteristicas.contains(&"incapacitado"));
        assert!(cautivo(&p.mundo, actor));
        assert!(
            p.mundo
                .actor(companero)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "muerto")
        );
    }
    #[test]
    fn ataque_sesion_exito_persiste_y_oculta_objetivo_muerto() {
        let (mut p, objetivo) = partida();
        p.procesar(IntencionFrontend::ComenzarAtaque { objetivo })
            .unwrap();
        let motor = &mut p.ataque.as_mut().unwrap().motor;
        motor.fichas[motor.objetivo].golpes = 3;
        for f in motor.fichas.iter_mut().filter(|f| f.aliado) {
            f.posicion = motor.entrada;
        }
        avanzar(&mut p);
        assert_eq!(
            p.ataque.as_ref().unwrap().motor.desenlace,
            Some(Desenlace::Exito)
        );
        cierre(&mut p);
        assert!(
            p.mundo
                .actor(objetivo)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "muerto")
        );
        assert!(
            !objetivos(&p.mundo, &p.contenido)
                .iter()
                .any(|o| o.actor == objetivo)
        );
    }
}
