use super::*;
use desert_tiger_core::{Caracteristica, ExtremoRelacion, LoteOperaciones, OperacionMundo};

fn fixture(incorporado: bool) -> Fixture {
    let f = Fixture::nuevo();
    let world = f.0.join("world.ron");
    let texto = std::fs::read_to_string(&world).unwrap().replacen(
        '(',
        "(\ncolectivos:[\"compania_prueba\"],",
        1,
    );
    std::fs::write(world, texto).unwrap();
    std::fs::create_dir(f.0.join("collectives")).unwrap();
    std::fs::write(f.0.join("collectives/compania_prueba.ron"),r#"(clave:"compania_prueba",nombre:"Compañía de prueba",descripcion:"Tripulación de prueba.",ubicacion_inicial:EnLugar("zona_arida"),lider:Some("dahran"))"#).unwrap();
    for clave in ["dahran", "principe_perdido", "bandido", "sirviente"] {
        if clave != "dahran" && !incorporado {
            continue;
        }
        let ruta = f.0.join("actors").join(format!("{clave}.ron"));
        let texto = std::fs::read_to_string(&ruta).unwrap();
        let desde = texto.find("ubicacion_inicial:").unwrap();
        let hasta = texto[desde..].find('\n').unwrap() + desde;
        let texto = format!(
            "{}ubicacion_inicial: EnColectivo(\"compania_prueba\"),{}",
            &texto[..desde],
            &texto[hasta..]
        );
        std::fs::write(ruta, texto).unwrap();
    }
    f
}
fn cita(clave: &str, dia: u8, condicion: &str, efectos: &str, relato: bool) -> String {
    let siguiente = if relato { "relato" } else { "fin" };
    let ver = if relato {
        r##"Vigneta(clave:"relato",presentacion:r#"return {titulo="Orden de la compañía",texto="La compañía sigue su camino."}"#,siguiente:Some("fin")),"##
    } else {
        ""
    };
    format!(
        r###"(clave:"{clave}",activacion:EnFecha(fecha:(anio:2025,mes:2,dia:{dia})),observacion:EnColectivo(colectivo:"compania_prueba"),condicion:r#"{condicion}"#,inicio:"efecto",pasos:[Efectos(clave:"efecto",salidas:["{siguiente}"],resolver:r#"{efectos}"#),{ver}Fin(clave:"fin",control:Continuar)])"###
    )
}
fn estabilizar(p: &mut Partida) -> Vec<String> {
    let mut vistos = vec![];
    for _ in 0..2000 {
        match p.vista() {
            VistaSesion::PresentandoEvento(v) => {
                vistos.push(v.evento);
                p.procesar(IntencionFrontend::ContinuarEvento { presentacion: v.id })
                    .unwrap();
            }
            VistaSesion::ProcesandoCierre { token } => {
                p.procesar(IntencionFrontend::ContinuarCierre { token })
                    .unwrap();
            }
            VistaSesion::Fatal(v) => panic!("{}", v.mensaje),
            _ => return vistos,
        }
    }
    panic!("procesamiento sin terminar")
}
fn abrir_espera(p: &mut Partida) -> VistaEsperaOrdenes {
    if !matches!(p.vista(), VistaSesion::Acampando(_)) {
        p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
        estabilizar(p);
    }
    let VistaSesion::Acampando(v) = p.vista() else {
        panic!()
    };
    p.procesar(IntencionFrontend::EsperarOrdenes { campamento: v.id })
        .unwrap();
    let VistaSesion::EsperandoOrdenes(e) = p.vista() else {
        panic!()
    };
    e
}
fn avanzar(p: &mut Partida) -> Vec<String> {
    let VistaSesion::EsperandoOrdenes(e) = p.vista() else {
        panic!()
    };
    p.procesar(IntencionFrontend::AvanzarEspera {
        espera: e.id,
        intervalo: e.intervalo,
    })
    .unwrap();
    estabilizar(p)
}
#[test]
fn espera_cancelable_sin_citas_lider_o_coste_doble_y_tokens_vencidos() {
    let f = fixture(true);
    let mut p = f.partida();
    let inicio = p.mundo.fecha();
    let id = p.mundo_visible().colectivo.unwrap().id;
    assert_eq!(p.mundo_visible().acciones, vec![AccionDisponible::Acampar]);
    let viaje = p.procesar(IntencionFrontend::ComenzarViaje {
        destino: p
            .contenido
            .claves()
            .lugar_por_clave("valle_fertil")
            .unwrap(),
    });
    assert!(matches!(
        viaje,
        Err(ErrorIntencion::AccionRechazada(
            RechazoAccion::ActorIncorporado
        ))
    ));
    let e = abrir_espera(&mut p);
    p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
        .unwrap();
    assert_eq!(p.mundo.fecha(), inicio);
    assert!(matches!(p.vista(),VistaSesion::Acampando(v) if !v.descanso_cubierto));
    let e = abrir_espera(&mut p);
    p.mundo
        .aplicar_lote(LoteOperaciones::uno(
            OperacionMundo::EstablecerLiderColectivo {
                colectivo: id,
                lider: None,
            },
        ))
        .unwrap();
    avanzar(&mut p);
    avanzar(&mut p);
    assert_eq!(
        p.mundo.fecha(),
        inicio.sumar(Duracion::desde_dias(2)).unwrap()
    );
    assert!(
        p.procesar(IntencionFrontend::AvanzarEspera {
            espera: e.id,
            intervalo: 0
        })
        .is_err()
    );
    p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
        .unwrap();
    let VistaSesion::Acampando(camp) = p.vista() else {
        panic!()
    };
    assert!(camp.descanso_cubierto);
    assert!(
        p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
            .is_err()
    );
    let fecha = p.mundo.fecha();
    p.procesar(IntencionFrontend::AbandonarCampamento {
        campamento: camp.id,
    })
    .unwrap();
    estabilizar(&mut p);
    assert_eq!(p.mundo.fecha(), fecha);
    assert_eq!(p.mundo.colectivo_de(p.mundo.actor_jugador()), Some(id));
}
#[test]
fn agenda_mueve_detiene_espera_y_libera_sin_perder_su_relato() {
    let f = fixture(true);
    f.escribir("zarpar",&cita("zarpar",1,"return true",r#"return {operaciones={op.trasladar_colectivo("compania_prueba","valle_fertil")},siguiente="relato"}"#,true));
    f.escribir("liberar",&cita("liberar",2,"return true",r#"return {operaciones={op.salir_grupo_de_colectivo("compania_prueba")},siguiente="relato"}"#,true));
    let mut p = f.partida();
    let principio = p.mundo.fecha();
    abrir_espera(&mut p);
    assert_eq!(avanzar(&mut p), vec!["zarpar"]);
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    let destino = p
        .contenido
        .claves()
        .lugar_por_clave("valle_fertil")
        .unwrap();
    assert_eq!(p.mundo.ubicacion_grupo(), destino);
    for id in std::iter::once(p.mundo.actor_jugador())
        .chain(p.mundo.grupo().integrantes().iter().copied())
    {
        assert_eq!(p.mundo.ubicacion_actor(id), Some(destino));
    }
    abrir_espera(&mut p);
    assert_eq!(avanzar(&mut p), vec!["liberar"]);
    assert!(p.mundo_visible().colectivo.is_none());
    assert!(
        p.mundo_visible()
            .acciones
            .iter()
            .any(|a| matches!(a, AccionDisponible::Viajar { .. }))
    );
    assert_eq!(
        p.mundo.fecha(),
        principio.sumar(Duracion::desde_dias(2)).unwrap()
    );
}
#[test]
fn eventos_remotos_aplican_efectos_sin_ejecutar_presentacion() {
    let f = fixture(false);
    let evento=cita("remoto",1,"return true",r#"return {operaciones={op.trasladar_colectivo("compania_prueba","valle_fertil")},siguiente="relato"}"#,true).replace("return {titulo=\"Orden de la compañía\",texto=\"La compañía sigue su camino.\"}","while true do end");
    f.escribir("remoto", &evento);
    let mut p = f.partida();
    p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
    let VistaSesion::Acampando(c) = p.vista() else {
        panic!()
    };
    p.procesar(IntencionFrontend::AbandonarCampamento { campamento: c.id })
        .unwrap();
    assert!(estabilizar(&mut p).is_empty());
    let colectivo = p
        .contenido
        .claves()
        .colectivo_por_clave("compania_prueba")
        .unwrap();
    assert_eq!(
        p.mundo.colectivo(colectivo).unwrap().lugar(),
        p.contenido
            .claves()
            .lugar_por_clave("valle_fertil")
            .unwrap()
    );
    assert_eq!(
        p.mundo.ubicacion_grupo(),
        p.contenido.claves().lugar_por_clave("zona_arida").unwrap()
    );
}
#[test]
fn actividad_expresa_despierta_pero_traslado_idempotente_y_evento_ajeno_no() {
    let f = fixture(true);
    f.escribir("quietos",&cita("quietos",1,"return true",r#"return {operaciones={op.trasladar_colectivo("compania_prueba","zona_arida")},siguiente="fin"}"#,false));
    f.escribir(
        "orden",
        &cita(
            "orden",
            2,
            "return true",
            r#"return {operaciones={},acciones_colectivas={"compania_prueba"},siguiente="fin"}"#,
            false,
        ),
    );
    let mut p = f.partida();
    abrir_espera(&mut p);
    avanzar(&mut p);
    assert!(matches!(p.vista(), VistaSesion::EsperandoOrdenes(_)));
    avanzar(&mut p);
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
}
#[test]
fn cancelacion_durante_cierre_cooperativo_no_inicia_otro_dia() {
    let f = fixture(true);
    for i in 0..150 {
        let k = format!("cita_{i}");
        f.escribir(
            &k,
            &cita(
                &k,
                1,
                "return false",
                r#"return {operaciones={},siguiente="fin"}"#,
                false,
            ),
        );
    }
    let mut p = f.partida();
    let e = abrir_espera(&mut p);
    let fecha = p.mundo.fecha();
    p.procesar(IntencionFrontend::AvanzarEspera {
        espera: e.id,
        intervalo: 0,
    })
    .unwrap();
    let VistaSesion::ProcesandoCierre { token } = p.vista() else {
        panic!("debe ceder CPU")
    };
    p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
        .unwrap();
    p.procesar(IntencionFrontend::ContinuarCierre { token })
        .unwrap();
    assert!(
        p.procesar(IntencionFrontend::ContinuarCierre { token })
            .is_err()
    );
    estabilizar(&mut p);
    assert!(matches!(p.vista(), VistaSesion::Acampando(_)));
    assert_eq!(
        p.mundo.fecha(),
        fecha.sumar(Duracion::desde_dias(1)).unwrap()
    );
}
#[test]
fn fichas_derivan_compania_lider_e_integrantes_y_consultas_lua() {
    let f = fixture(true);
    f.escribir("consulta",&simple("consulta","partida.disponible",r#"return mundo:colectivo_de("principe_perdido")=="compania_prueba" and mundo:lider_colectivo("compania_prueba")=="dahran" and mundo:ubicacion_colectivo("compania_prueba")=="zona_arida" and #mundo:miembros_colectivo("compania_prueba")==4"#));
    let mut p = f.partida();
    assert_eq!(estabilizar(&mut p), vec!["consulta"]);
    p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
    let VistaSesion::Acampando(c) = p.vista() else {
        panic!()
    };
    let colectivo = p.mundo.colectivo_de(p.mundo.actor_jugador()).unwrap();
    let ficha = p
        .consultar_ficha(c.id, ExtremoRelacion::Colectivo(colectivo))
        .unwrap();
    assert!(ficha.entradas.iter().any(
        |e| matches!(e,crate::EntradaFicha::Relacion(r) if r.etiqueta.starts_with("Liderado por"))
    ));
    assert_eq!(ficha.entradas.iter().filter(|e|matches!(e,crate::EntradaFicha::Relacion(r) if r.etiqueta.starts_with("Integrante:"))).count(),4);
}
#[test]
fn cita_inicial_se_atiende_una_vez_y_condicion_falsa_no_reintenta() {
    let f = fixture(true);
    f.escribir("inicial",&cita("inicial",1,"return true",r#"return {operaciones={op.establecer_entera("veces",mundo:entera("veces",0)+1)},siguiente="fin"}"#,false).replace("mes:2,dia:1","mes:1,dia:31"));
    f.escribir(
        "omitido",
        &cita(
            "omitido",
            1,
            "return false",
            r#"return {operaciones={op.establecer_booleana("nunca",true)},siguiente="fin"}"#,
            false,
        ),
    );
    let mut p = f.partida();
    estabilizar(&mut p);
    abrir_espera(&mut p);
    avanzar(&mut p);
    avanzar(&mut p);
    assert_eq!(
        p.mundo
            .variable_entera(
                &desert_tiger_core::ClaveVariableGlobal::nueva("veces".into()).unwrap()
            )
            .unwrap(),
        1
    );
    assert!(!variable(&p, "nunca"));
}
#[test]
fn muerte_del_lider_no_impide_salida_fechada() {
    let f = fixture(true);
    f.escribir("liberar",&cita("liberar",2,"return mundo:colectivo_de(\"principe_perdido\")==\"compania_prueba\"",r#"return {operaciones={op.salir_grupo_de_colectivo("compania_prueba")},siguiente="fin"}"#,false));
    let mut p = f.partida();
    let lider = p.contenido.claves().actor_por_clave("dahran").unwrap();
    p.mundo
        .aplicar_lote(LoteOperaciones::uno(
            OperacionMundo::AnadirCaracteristicaActor {
                actor: lider,
                caracteristica: Caracteristica::nueva("muerto".into()).unwrap(),
            },
        ))
        .unwrap();
    abrir_espera(&mut p);
    avanzar(&mut p);
    avanzar(&mut p);
    assert!(p.mundo.colectivo_de(p.mundo.actor_jugador()).is_none());
}

#[test]
fn demo_completa_permite_cancelar_y_termina_servicio() {
    let f = fixture(false);
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/demos/colectivos/events");
    // La demo utiliza su clave oficial; el fixture conserva la misma compañía
    // de prueba para evitar que existan dos fuentes de pertenencia.
    for e in std::fs::read_dir(raiz).unwrap() {
        let e = e.unwrap();
        let t = std::fs::read_to_string(e.path())
            .unwrap()
            .replace("compania_dahran", "compania_prueba");
        std::fs::write(f.0.join("events").join(e.file_name()), t).unwrap();
    }
    let mut p = f.partida();
    seguir(&mut p, 0);
    estabilizar(&mut p);
    assert!(p.mundo_visible().colectivo.is_some());
    abrir_espera(&mut p);
    assert_eq!(avanzar(&mut p), vec!["demo_zarpar"]);
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    let e = abrir_espera(&mut p);
    p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
        .unwrap();
    abrir_espera(&mut p);
    assert_eq!(avanzar(&mut p), vec!["demo_jornada_a_bordo"]);
    assert!(matches!(p.vista(), VistaSesion::EsperandoOrdenes(_)));
    assert_eq!(avanzar(&mut p), vec!["demo_licencia"]);
    assert!(p.mundo_visible().colectivo.is_none());
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
}
#[test]
fn cancelar_ante_una_accion_confirmada_retorna_al_campamento_actualizado() {
    let f = fixture(true);
    f.escribir("zarpar",&cita("zarpar",1,"return true",r#"return {operaciones={op.trasladar_colectivo("compania_prueba","valle_fertil")},siguiente="relato"}"#,true));
    let mut p = f.partida();
    let e = abrir_espera(&mut p);
    p.procesar(IntencionFrontend::AvanzarEspera {
        espera: e.id,
        intervalo: 0,
    })
    .unwrap();
    let VistaSesion::PresentandoEvento(v) = p.vista() else {
        panic!()
    };
    let fecha = p.mundo.fecha();
    for _ in 0..10 {
        assert_eq!(p.vista(), VistaSesion::PresentandoEvento(v.clone()));
        assert_eq!(p.mundo.fecha(), fecha);
    }
    p.procesar(IntencionFrontend::CancelarEspera { espera: e.id })
        .unwrap();
    estabilizar(&mut p);
    let VistaSesion::Acampando(c) = p.vista() else {
        panic!()
    };
    assert_eq!(
        c.lugar,
        p.contenido
            .claves()
            .lugar_por_clave("valle_fertil")
            .unwrap()
    );
    assert!(c.descanso_cubierto);
    assert_eq!(p.mundo.fecha(), fecha);
}
#[test]
fn fechas_prioridades_condiciones_y_contexto_no_dependen_del_protagonista() {
    let f = fixture(false);
    for (k, dia, prioridad, condicion, marca) in [
        ("b", 1, 0, "return true", "b"),
        ("a", 1, 10, "return true", "a"),
        (
            "omitida",
            1,
            20,
            "return mundo:texto(\"registro\",\"\")==\"a\"",
            "x",
        ),
        (
            "c",
            2,
            0,
            "return ctx:actor()==nil and ctx:accion()==nil and ctx:objetivo()==nil and #ctx:participantes()==0 and ctx:punto()==\"mundo.fecha_programada\" and ctx:fecha_programada().dia==2 and ctx:ahora().dia==3",
            "c",
        ),
    ] {
        let efectos = format!(
            r#"return {{operaciones={{op.establecer_texto("registro",mundo:texto("registro","").."{marca}")}},siguiente="fin"}}"#
        );
        let evento = cita(k, dia, condicion, &efectos, false).replacen(
            "inicio:",
            &format!("prioridad:{prioridad},inicio:"),
            1,
        );
        f.escribir(k, &evento);
    }
    let mut p = f.partida();
    p.concluir_accion(desert_tiger_core::TiempoAccion {
        duracion: Duracion::desde_dias(3),
        fraccion: FraccionDia::CERO,
    })
    .unwrap();
    estabilizar(&mut p);
    assert_eq!(
        p.mundo
            .variable_texto(
                &desert_tiger_core::ClaveVariableGlobal::nueva("registro".into()).unwrap()
            )
            .unwrap(),
        "abc"
    );
}
#[test]
fn ingreso_tardio_consulta_agenda_y_carga_rechaza_activaciones_invalidas() {
    let f = fixture(false);
    f.escribir(
        "licencia",
        &cita(
            "licencia",
            1,
            "return true",
            r#"return {operaciones={},siguiente="fin"}"#,
            false,
        ),
    );
    f.escribir("ingresar",r###"(clave:"ingresar",puntos:["partida.disponible"],condicion:r#"return mundo:booleana("llego",false) and mundo:hay_citas_futuras_del_colectivo("compania_prueba")"#,inicio:"ver",pasos:[Efectos(clave:"ver",salidas:["fin"],resolver:r#"return {operaciones={op.ingresar_grupo_en_colectivo("compania_prueba")},siguiente="fin"}"#),Fin(clave:"fin",control:Continuar)])"###);
    let mut p = f.partida();
    p.concluir_accion(desert_tiger_core::TiempoAccion {
        duracion: Duracion::desde_dias(2),
        fraccion: FraccionDia::CERO,
    })
    .unwrap();
    estabilizar(&mut p);
    p.mundo
        .aplicar_lote(LoteOperaciones::uno(OperacionMundo::EstablecerGlobal {
            clave: desert_tiger_core::ClaveVariableGlobal::nueva("llego".into()).unwrap(),
            valor: desert_tiger_core::ValorVariableGlobal::Booleana(true),
        }))
        .unwrap();
    p.abrir_eventos(
        p.contexto_sesion(ClasePunto::Disponible),
        eventos_partida::Continuacion::Disponible,
    )
    .unwrap();
    estabilizar(&mut p);
    assert!(p.mundo_visible().colectivo.is_none());
    let original = cita(
        "invalida",
        1,
        "return true",
        r#"return {operaciones={},siguiente="fin"}"#,
        false,
    );
    for t in [
        original.replace("anio:2025,mes:2,dia:1", "anio:2025,mes:1,dia:1"),
        original.replace("activacion:", "puntos:[],activacion:"),
        original.replace("compania_prueba", "ausente"),
        original.replace("dia:1", "dia:40"),
    ] {
        f.escribir("invalida", &t);
        assert!(crate::cargar_contenido(&f.0).is_err());
    }
}

#[test]
fn traslados_sin_relato_recuerdan_cada_posicion_confirmada_del_grupo() {
    let f = fixture(true);
    for (clave, dia, destino) in [("ida", 1, "valle_fertil"), ("vuelta", 2, "zona_arida")] {
        let efecto = format!(
            r#"return {{operaciones={{op.trasladar_colectivo("compania_prueba","{destino}")}},siguiente="fin"}}"#
        );
        f.escribir(clave, &cita(clave, dia, "return true", &efecto, false));
    }
    let mut p = f.partida();
    let inicial = p.vision.proyectar(&p.mapa);
    let mut esperada = VisionMapa::nueva(&p.mapa);
    esperada.observar(
        &p.mapa,
        p.contenido
            .presentacion()
            .lugar(p.mundo.ubicacion_grupo())
            .unwrap()
            .coordenada_central()
            .unwrap(),
    );
    for destino in ["valle_fertil", "zona_arida"] {
        let l = p.contenido.claves().lugar_por_clave(destino).unwrap();
        esperada.observar(
            &p.mapa,
            p.contenido
                .presentacion()
                .lugar(l)
                .unwrap()
                .coordenada_central()
                .unwrap(),
        );
    }
    assert_ne!(esperada.proyectar(&p.mapa), inicial);
    p.concluir_accion(desert_tiger_core::TiempoAccion {
        duracion: Duracion::desde_dias(2),
        fraccion: FraccionDia::CERO,
    })
    .unwrap();
    estabilizar(&mut p);
    assert_eq!(p.vision.proyectar(&p.mapa), esperada.proyectar(&p.mapa));
}
