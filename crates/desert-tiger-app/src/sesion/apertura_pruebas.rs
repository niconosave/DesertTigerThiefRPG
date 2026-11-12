use super::*;
use desert_tiger_core::{ClaveVariableGlobal, LoteOperaciones, OperacionMundo};
fn nueva() -> Partida {
    let c = crate::cargar_contenido(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
    )
    .unwrap();
    Partida::nueva(&c, ConfiguracionPartida { semilla: Some(8) }).unwrap()
}
fn relatos(p: &mut Partida, opcion: usize) {
    for _ in 0..100 {
        let i = match p.vista() {
            VistaSesion::PresentandoEvento(v) => {
                IntencionFrontend::ContinuarEvento { presentacion: v.id }
            }
            VistaSesion::EsperandoDecisionEvento(v) => IntencionFrontend::ResponderEvento {
                presentacion: v.id,
                opcion: v.opciones[opcion].id,
            },
            VistaSesion::ProcesandoCierre { token } => IntencionFrontend::ContinuarCierre { token },
            VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
            _ => return,
        };
        p.procesar(i).unwrap();
    }
    panic!("relato sin terminar")
}
fn llegada(p: &mut Partida, opcion: usize) {
    let ciudad = p.contenido.claves().lugar_por_clave("zarabad").unwrap();
    p.mundo.trasladar(p.mundo.actor_jugador(), ciudad).unwrap();
    p.abrir_eventos(
        p.contexto_sesion(ClasePunto::Disponible),
        eventos_partida::Continuacion::Disponible,
    )
    .unwrap();
    relatos(p, opcion);
}
fn esperar_dia(p: &mut Partida) {
    if matches!(p.vista(), VistaSesion::Disponible(_)) {
        p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
        relatos(p, 0);
        let VistaSesion::Acampando(c) = p.vista() else {
            panic!()
        };
        p.procesar(IntencionFrontend::EsperarOrdenes { campamento: c.id })
            .unwrap();
    }
    let VistaSesion::EsperandoOrdenes(e) = p.vista() else {
        panic!()
    };
    p.procesar(IntencionFrontend::AvanzarEspera {
        espera: e.id,
        intervalo: e.intervalo,
    })
    .unwrap();
    relatos(p, 0);
}
#[test]
fn apertura_pruebas_naufragio_introduce_y_recluta_bandido_una_sola_vez() {
    let mut p = nueva();
    let bandido = p.contenido.claves().actor_por_clave("bandido").unwrap();
    assert!(!p.mundo.grupo().contiene(bandido));
    assert_eq!(p.mundo.grupo().integrantes().len(), 1);
    assert!(
        matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento=="naufragio_y_bandido")
    );
    let fecha = p.mundo.fecha();
    relatos(&mut p, 0);
    assert_eq!(p.mundo.fecha(), fecha);
    assert!(p.mundo.grupo().contiene(bandido));
    let ciudad = p.contenido.claves().lugar_por_clave("zarabad").unwrap();
    assert!(
        acciones::destinos_seleccionables_jugador(&p.mundo)
            .unwrap()
            .contains(&ciudad)
    );
    let grupo = p.mundo.grupo().integrantes().to_vec();
    p.abrir_eventos(
        p.contexto_sesion(ClasePunto::Disponible),
        eventos_partida::Continuacion::Disponible,
    )
    .unwrap();
    relatos(&mut p, 0);
    assert_eq!(p.mundo.grupo().integrantes(), grupo);
}
#[test]
fn apertura_pruebas_venta_servicio_y_permiso_de_tierra_sin_declarar_libertad() {
    let mut p = nueva();
    relatos(&mut p, 0);
    llegada(&mut p, 0);
    assert!(p.mundo_visible.colectivo.is_some());
    assert_eq!(p.mundo_visible.acciones, vec![AccionDisponible::Acampar]);
    assert_eq!(p.mundo.grupo().integrantes().len(), 1);
    for k in ["principe_perdido", "sirviente"] {
        assert!(es_esclavo(
            &p,
            p.contenido.claves().actor_por_clave(k).unwrap()
        ));
    }
    for _ in 0..14 {
        esperar_dia(&mut p);
    }
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    assert!(p.mundo_visible.colectivo.is_none());
    assert_eq!(
        p.mundo.ubicacion_grupo(),
        p.contenido.claves().lugar_por_clave("zarabad").unwrap()
    );
    assert!(
        p.mundo
            .variable_booleana(&ClaveVariableGlobal::nueva("permisoTierraZarabad".into()).unwrap())
            .unwrap()
    );
    assert!(es_esclavo(&p, p.mundo.actor_jugador()));
}
#[test]
fn apertura_pruebas_advertencia_permite_evitar_venta() {
    let mut p = nueva();
    relatos(&mut p, 0);
    llegada(&mut p, 1);
    assert!(p.mundo_visible.colectivo.is_none());
    assert!(!es_esclavo(&p, p.mundo.actor_jugador()));
    assert!(
        !p.mundo
            .grupo()
            .contiene(p.contenido.claves().actor_por_clave("bandido").unwrap())
    );
    assert!(
        p.mundo_visible
            .acciones
            .iter()
            .any(|a| matches!(a, AccionDisponible::Viajar { .. }))
    );
}
#[test]
fn apertura_pruebas_duracion_del_servicio_se_cuenta_desde_una_llegada_tardia() {
    let mut p = nueva();
    relatos(&mut p, 0);
    p.mundo.avanzar_fecha(Duracion::desde_dias(400)).unwrap();
    llegada(&mut p, 0);
    let fecha = p.mundo.fecha();
    for _ in 0..14 {
        esperar_dia(&mut p);
    }
    assert_eq!(
        p.mundo.fecha(),
        fecha.sumar(Duracion::desde_dias(14)).unwrap()
    );
    assert!(p.mundo_visible.colectivo.is_none());
}
#[test]
fn apertura_pruebas_reclutamiento_revierte_el_lote_y_no_duplica_companeros() {
    let mut p = nueva();
    let bandido = p.contenido.claves().actor_por_clave("bandido").unwrap();
    let antes = p.mundo.clone();
    assert!(
        p.mundo
            .aplicar_lote(
                LoteOperaciones::nuevo(vec![
                    OperacionMundo::IncorporarCompanero { actor: bandido },
                    OperacionMundo::IncorporarCompanero {
                        actor: desert_tiger_core::IdActor::desde_posicion(999).unwrap()
                    }
                ])
                .unwrap()
            )
            .is_err()
    );
    assert_eq!(p.mundo, antes);
    for _ in 0..2 {
        p.mundo
            .aplicar_lote(LoteOperaciones::uno(OperacionMundo::IncorporarCompanero {
                actor: bandido,
            }))
            .unwrap();
    }
    assert_eq!(
        p.mundo
            .grupo()
            .integrantes()
            .iter()
            .filter(|&&a| a == bandido)
            .count(),
        1
    );
}

#[test]
fn orientacion_inicial_nombres_conocimientos_y_enlace_maritimo() {
    let mut p = nueva();
    relatos(&mut p, 0);
    let claves = p.contenido.claves();
    for (clave, nombre, lugar) in [
        ("bandido", "Rashak", "zarabad"),
        ("sirviente", "Nadin", "ur_eshan"),
        ("principe_perdido", "Ashur", "ur_eshan"),
    ] {
        let actor = claves.actor_por_clave(clave).unwrap();
        assert_eq!(
            p.contenido.presentacion().actor(actor).unwrap().nombre(),
            nombre
        );
        assert_eq!(
            p.mundo.lugares_conocidos(actor).unwrap(),
            &[claves.lugar_por_clave(lugar).unwrap()]
        );
    }
    let ciudad = claves.lugar_por_clave("zarabad").unwrap();
    let mar = claves.lugar_por_clave("mar_del_viento").unwrap();
    assert!(p.mundo.lugares_conectados(ciudad).unwrap().contains(&mar));
    assert!(p.mundo.lugares_conectados(mar).unwrap().contains(&ciudad));
    assert!(
        p.mundo
            .lugares_conectados(mar)
            .unwrap()
            .contains(&claves.lugar_por_clave("shahraz").unwrap())
    );
}

fn es_esclavo(p: &Partida, actor: desert_tiger_core::IdActor) -> bool {
    p.mundo.relaciones().iter().any(|r| {
        r.tipo() == desert_tiger_core::TipoRelacion::EsclavoDe
            && r.origen() == desert_tiger_core::ExtremoRelacion::Actor(actor)
            && r.destino()
                == desert_tiger_core::ExtremoRelacion::Actor(
                    p.contenido.claves().actor_por_clave("zahrim").unwrap(),
                )
    })
}

#[test]
fn dialogos_encadenan_momentos_y_rechazan_confirmaciones_vencidas() {
    let mut p = nueva();
    relatos(&mut p, 0);
    let ciudad = p.contenido.claves().lugar_por_clave("zarabad").unwrap();
    p.mundo.trasladar(p.mundo.actor_jugador(), ciudad).unwrap();
    p.abrir_eventos(
        p.contexto_sesion(ClasePunto::Disponible),
        eventos_partida::Continuacion::Disponible,
    )
    .unwrap();
    let VistaSesion::PresentandoEvento(primero) = p.vista() else {
        panic!("se esperaba narración")
    };
    assert_eq!(primero.total_momentos, 3);
    assert!(primero.hablante.is_none() && primero.opciones.is_empty());
    p.procesar(IntencionFrontend::ContinuarEvento {
        presentacion: primero.id,
    })
    .unwrap();
    let VistaSesion::PresentandoEvento(dialogo) = p.vista() else {
        panic!("se esperaba diálogo")
    };
    assert_eq!(
        dialogo.hablante,
        p.contenido.claves().actor_por_clave("sirviente")
    );
    assert_eq!(dialogo.momento, 1);
    assert!(dialogo.opciones.is_empty());
    assert!(
        p.procesar(IntencionFrontend::ContinuarEvento {
            presentacion: primero.id
        })
        .is_err()
    );
    let VistaSesion::PresentandoEvento(vigente) = p.vista() else {
        panic!()
    };
    assert_eq!(vigente.id, dialogo.id);
    p.procesar(IntencionFrontend::ContinuarEvento {
        presentacion: dialogo.id,
    })
    .unwrap();
    let VistaSesion::EsperandoDecisionEvento(final_) = p.vista() else {
        panic!("se esperaba decisión final")
    };
    assert!(final_.hablante.is_none());
    assert_eq!(final_.momento, 2);
    assert_eq!(final_.opciones.len(), 2);
    p.procesar(IntencionFrontend::ResponderEvento {
        presentacion: final_.id,
        opcion: final_.opciones[1].id,
    })
    .unwrap();
    relatos(&mut p, 0);
    assert!(!es_esclavo(&p, p.mundo.actor_jugador()));
}
