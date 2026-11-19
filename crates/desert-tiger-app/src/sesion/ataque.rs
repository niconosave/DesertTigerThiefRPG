use super::*;
use crate::encuentro::{FichaEncuentro, ObjetivoAtaque, TerrenoEncuentro, VistaEncuentro};
use desert_tiger_core::encuentros::{Celda, Desenlace, Encuentro, Suceso};
use desert_tiger_core::{Caracteristica, ExtremoRelacion, OperacionMundo, TipoRelacion};
#[derive(Clone)]
pub(super) struct AtaqueActivo {
    id: u64,
    pub(super) motor: Encuentro,
    azar: ChaCha12Rng,
    pendiente: Option<VistaEncuentro>,
    secuencia: u64,
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
fn vista(a: &AtaqueActivo, c: &ContenidoValidado) -> VistaEncuentro {
    let m = &a.motor;
    let nombre = |actor| {
        c.presentacion()
            .actor(actor)
            .map(|a| a.nombre().to_owned())
            .unwrap_or_else(|| "Combatiente".into())
    };
    let finalizado = m.desenlace.is_some();
    let texto = if let Some(fin) = m.desenlace {
        match fin {
        Desenlace::Exito=>"El objetivo ha muerto. El grupo logra salir del recinto.".into(),
        Desenlace::Retirada=>"El grupo alcanza la salida y abandona el ataque. El objetivo sigue vivo.".into(),
        Desenlace::Captura{captor}=>format!("El grupo queda fuera de combate. {} toma prisioneros a los supervivientes.",nombre(captor)),
        Desenlace::Derrota=>"El enfrentamiento termina con el grupo fuera de combate. Sus pérdidas y heridas permanecen.".into(),
    }
    } else if m.retirando {
        "El grupo intenta retirarse y sacar a sus compañeros del recinto.".into()
    } else if m.fichas[m.objetivo].muerto() {
        "El objetivo ha caído. El grupo busca la salida.".into()
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
        .filter(|(_, f)| f.aliado || m.descubierta)
        .map(|(id, f)| FichaEncuentro {
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
            Suceso::Recogido { ficha, por } => Some(format!(
                "{} carga a {} para sacarlo.",
                nombre(m.fichas[*por].actor),
                nombre(m.fichas[*ficha].actor)
            )),
            _ => None,
        })
        .collect();
    VistaEncuentro {
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
    pub(super) fn iniciar_ataque_local(&mut self, objetivo: IdActor) -> Result<(), String> {
        if !objetivos(&self.mundo, &self.contenido)
            .iter()
            .any(|o| o.actor == objetivo)
        {
            self.vista = VistaSesion::Disponible(self.mundo_visible.clone());
            return Ok(());
        }
        self.crear_ataque_local(objetivo)
            .map_err(|e| format!("{e:?}"))
    }
    fn crear_ataque_local(&mut self, objetivo: IdActor) -> Result<(), ErrorIntencion> {
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
        let perfil = config
            .objetivos
            .iter()
            .find(|o| self.contenido.claves().actor_por_clave(&o.actor) == Some(objetivo))
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        let defensores = perfil
            .defensores
            .iter()
            .filter_map(|k| self.contenido.claves().actor_por_clave(k))
            .filter(|&a| {
                a != objetivo
                    && !grupo.contains(&a)
                    && self.mundo.ubicacion_actor(a) == Some(self.mundo.ubicacion_grupo())
                    && self
                        .mundo
                        .actor(a)
                        .is_some_and(|a| !a.caracteristicas().any(|v| v.como_str() == "muerto"))
            })
            .collect::<Vec<_>>();
        let seed = self
            .azar
            .elegir_indice(NonZeroUsize::new(u32::MAX as usize).unwrap())
            .map_err(|_| ErrorIntencion::FaseIncompatible)? as u64;
        let mut azar = ChaCha12Rng::seed_from_u64(seed);
        let motor = Encuentro::nuevo(
            &self.mundo,
            &grupo,
            objetivo,
            &defensores,
            &mut AzarChaCha(azar.clone()),
        )
        .map_err(|_| ErrorIntencion::AccionRechazada(RechazoAccion::DestinoInvalido))?;
        // Flujo separado de resolución, independiente del número de sorteos del generador.
        azar = ChaCha12Rng::seed_from_u64(seed ^ 0x617461717565);
        self.ordinal = self
            .ordinal
            .checked_add(1)
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        let activo = AtaqueActivo {
            id: self.ordinal,
            motor,
            azar,
            pendiente: None,
            secuencia: 0,
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
        if let IntencionFrontend::ComenzarAtaque { objetivo } = i {
            if !matches!(self.vista, VistaSesion::Disponible(_)) {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            if !objetivos(&self.mundo, &self.contenido)
                .iter()
                .any(|o| o.actor == objetivo)
            {
                return Err(ErrorIntencion::AccionRechazada(
                    RechazoAccion::DestinoInvalido,
                ));
            }
            let mut ctx = self.contexto_sesion(ClasePunto::AtacarInicio);
            ctx.accion = Some("atacar");
            ctx.objetivo = Some(crate::ExtremoRelacion::Actor(objetivo));
            if let Err(e) =
                self.abrir_eventos(ctx, eventos_partida::Continuacion::IniciarAtaque(objetivo))
            {
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
        let a = self
            .ataque
            .as_ref()
            .ok_or(ErrorIntencion::FaseIncompatible)?;
        if matches!(i, IntencionFrontend::CerrarEncuentro { .. }) {
            if a.motor.desenlace.is_none() || a.pendiente.is_some() {
                return Err(ErrorIntencion::FaseIncompatible);
            }
            let segundos = 600u128 + u128::from(a.motor.pulso) * 6;
            let partes = segundos * 2_400_000_000u128 / 86_400;
            let tiempo = desert_tiger_core::TiempoAccion {
                duracion: Duracion::desde_dias((partes / 2_400_000_000) as u32),
                fraccion: FraccionDia::nueva((partes % 2_400_000_000) as u32)
                    .map_err(|_| ErrorIntencion::FaseIncompatible)?,
            };
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
            for f in nuevo
                .motor
                .fichas
                .iter()
                .filter(|f| f.aliado && !f.muerto())
            {
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
                .filter(|(_, f)| f.aliado || nuevo.motor.descubierta)
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
