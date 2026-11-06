use super::*;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
static NUMERO: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
fn copiar(a: &Path, b: &Path) {
    std::fs::create_dir_all(b).unwrap();
    for e in std::fs::read_dir(a).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let d = b.join(e.file_name());
        if p.is_dir() {
            copiar(&p, &d);
        } else {
            std::fs::copy(p, d).unwrap();
        }
    }
}
impl Fixture {
    fn nuevo() -> Self {
        let p = std::env::temp_dir().join(format!(
            "dt-eventos-{}-{}",
            std::process::id(),
            NUMERO.fetch_add(1, Ordering::Relaxed)
        ));
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        copiar(&raiz.join("content/base"), &p);
        std::fs::create_dir(p.join("events")).unwrap();
        Self(p)
    }
    fn escribir(&self, k: &str, s: &str) {
        std::fs::write(self.0.join("events").join(format!("{k}.ron")), s).unwrap();
    }
    fn contenido(&self) -> ContenidoValidado {
        crate::cargar_contenido(&self.0).unwrap()
    }
    fn partida(&self) -> Partida {
        Partida::con_azar(&self.contenido(), Box::new(Favorable)).unwrap()
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
struct Favorable;
impl FuenteAzar for Favorable {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
        Ok(n.get() - 1)
    }
}
fn simple(k: &str, punto: &str, condicion: &str) -> String {
    format!(
        r##"(clave:"{k}",puntos:["{punto}"],condicion:r#"{condicion}"#,inicio:"ver",pasos:[Vigneta(clave:"ver",presentacion:r#"return {{titulo="Evento",texto="Texto de prueba"}}"#,siguiente:Some("fin")),Fin(clave:"fin",control:Continuar)])"##
    )
}
fn seguir(p: &mut Partida, opcion: usize) -> VistaSesion {
    match p.vista() {
        VistaSesion::PresentandoEvento(v) => p
            .procesar(IntencionFrontend::ContinuarEvento { presentacion: v.id })
            .unwrap(),
        VistaSesion::EsperandoDecisionEvento(v) => p
            .procesar(IntencionFrontend::ResponderEvento {
                presentacion: v.id,
                opcion: v.opciones[opcion].id,
            })
            .unwrap(),
        VistaSesion::PresentandoAccion(v) => p
            .procesar(IntencionFrontend::ContinuarPresentacion { fragmento: v.id })
            .unwrap(),
        VistaSesion::EsperandoDecision(v) => p
            .procesar(IntencionFrontend::ResponderDecision {
                decision: v.id,
                opcion: v.opciones[0].id,
            })
            .unwrap(),
        VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
        v => v,
    }
}
fn terminar(p: &mut Partida, opcion: usize) -> Vec<String> {
    let mut vistos = vec![];
    for _ in 0..500 {
        match p.vista() {
            VistaSesion::Disponible(_) => return vistos,
            VistaSesion::PresentandoEvento(v) | VistaSesion::EsperandoDecisionEvento(v) => {
                vistos.push(v.evento)
            }
            _ => {}
        }
        seguir(p, opcion);
    }
    panic!("no termina")
}
fn variable(p: &Partida, k: &str) -> bool {
    p.mundo
        .variable_booleana(&desert_tiger_core::ClaveVariableGlobal::nueva(k.into()).unwrap())
        .unwrap_or(false)
}

#[test]
fn demo_completa_reanuda_encadena_y_sustituye_sin_cambiar_fecha_por_eventos() {
    let f = Fixture::nuevo();
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/demos/eventos/events");
    copiar(&raiz, &f.0.join("events"));
    let mut p = f.partida();
    assert!(matches!(p.vista(), VistaSesion::PresentandoEvento(_)));
    let viejo = match p.vista() {
        VistaSesion::PresentandoEvento(v) => v.id,
        _ => panic!(),
    };
    seguir(&mut p, 0);
    assert_eq!(
        p.procesar(IntencionFrontend::ContinuarEvento {
            presentacion: viejo
        }),
        Err(ErrorIntencion::FragmentoNoVigente)
    );
    let destino = p
        .contenido
        .claves()
        .lugar_por_clave("valle_fertil")
        .unwrap();
    p.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    let vistos = terminar(&mut p, 0);
    assert!(vistos.iter().any(|s| s == "demo_trayecto"));
    assert!(vistos.iter().any(|s| s == "demo_cadena_b"));
    assert!(vistos.iter().any(|s| s == "demo_llegada"));
    assert_eq!(p.mundo.ubicacion_grupo(), destino);
    assert!(variable(&p, "demoContinuar") && variable(&p, "demoCadenaA"));
    let c =
        crate::cargar_contenido(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"))
            .unwrap();
    let mut sin = Partida::con_azar(&c, Box::new(Favorable)).unwrap();
    sin.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    terminar(&mut sin, 0);
    assert_eq!(p.mundo.fecha(), sin.mundo.fecha());
    assert_eq!(p.resto, sin.resto);
    // Ocurrencias y marcadores independientes entre partidas.
    assert!(matches!(
        f.partida().vista(),
        VistaSesion::PresentandoEvento(_)
    ));
}

#[test]
fn interrupcion_conserva_lugar_tiempo_y_no_repite_respuesta() {
    let f = Fixture::nuevo();
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/demos/eventos/events");
    copiar(&raiz, &f.0.join("events"));
    let mut p = f.partida();
    seguir(&mut p, 0);
    let destino = p
        .contenido
        .claves()
        .lugar_por_clave("valle_fertil")
        .unwrap();
    p.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    for _ in 0..100 {
        if let VistaSesion::EsperandoDecisionEvento(v) = p.vista() {
            let lugar = p.mundo.ubicacion_grupo();
            let ahora = v.fecha;
            let vieja = v.opciones[1].id;
            let token = v.id;
            seguir(&mut p, 1);
            assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
            assert_eq!(p.mundo.ubicacion_grupo(), lugar);
            assert_ne!(lugar, destino);
            assert_eq!(p.mundo.fecha(), ahora);
            assert!(variable(&p, "demoInterrumpir"));
            let antes = p.mundo.clone();
            assert_eq!(
                p.procesar(IntencionFrontend::ResponderEvento {
                    presentacion: token,
                    opcion: vieja
                }),
                Err(ErrorIntencion::DecisionNoVigente)
            );
            assert_eq!(p.mundo, antes);
            return;
        }
        seguir(&mut p, 0);
    }
    panic!("sin decisión narrativa")
}

#[test]
fn una_presentacion_invalida_no_confirma_el_lote_anterior() {
    let f = Fixture::nuevo();
    f.escribir("fallo",r##"(clave:"fallo",puntos:["acampar.inicio"],condicion:"return true",inicio:"efecto",pasos:[Efectos(clave:"efecto",salidas:["ver"],resolver:r#"return {operaciones={op.establecer_booleana("noDebeExistir",true),op.anadir_caracteristica("sirviente","herido")},siguiente="ver"}"#),Vigneta(clave:"ver",presentacion:"return {titulo=5,texto='inválido'}",siguiente:Some("fin")),Fin(clave:"fin",control:Continuar)])"##);
    let mut p = f.partida();
    let antes = p.mundo.clone();
    p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
    assert!(matches!(p.vista(), VistaSesion::Fatal(_)));
    assert_eq!(p.mundo, antes);
}

#[test]
fn ultima_operacion_invalida_y_continuacion_muerta_no_dejan_cambios() {
    for cuerpo in [
        r#"op.establecer_booleana("noDebeExistir",true),op.establecer_entera("dato",5)"#,
        r#"op.establecer_booleana("noDebeExistir",true),op.anadir_caracteristica("principe_perdido","muerto")"#,
    ] {
        let f = Fixture::nuevo();
        f.escribir("fallo",&format!(r##"(clave:"fallo",puntos:["viajar.inicio"],condicion:"return true",inicio:"efecto",pasos:[Efectos(clave:"efecto",salidas:["fin"],resolver:r#"return {{operaciones={{{cuerpo}}},siguiente="fin"}}"#),Fin(clave:"fin",control:Continuar)])"##));
        let mut p = f.partida();
        p.mundo
            .aplicar_lote(desert_tiger_core::LoteOperaciones::uno(
                desert_tiger_core::OperacionMundo::EstablecerGlobal {
                    clave: desert_tiger_core::ClaveVariableGlobal::nueva("dato".into()).unwrap(),
                    valor: desert_tiger_core::ValorVariableGlobal::Booleana(false),
                },
            ))
            .unwrap();
        let antes = p.mundo.clone();
        let destino = p
            .contenido
            .claves()
            .lugar_por_clave("hamada_de_zahrun")
            .unwrap();
        p.procesar(IntencionFrontend::ComenzarViaje { destino })
            .unwrap();
        assert!(matches!(p.vista(), VistaSesion::Fatal(_)));
        assert_eq!(p.mundo, antes);
    }
}

#[test]
fn condiciones_presentacion_y_limites_son_solo_lectura() {
    for codigo in [
        "return 1",
        "while true do end",
        "return op.establecer_booleana('x',true)",
        "return mundo:booleana('x','incorrecto')",
        "return mundo:actor_tiene('inexistente','herido')",
    ] {
        let f = Fixture::nuevo();
        f.escribir("fallo", &simple("fallo", "partida.disponible", codigo));
        let p = f.partida();
        assert!(matches!(p.vista(), VistaSesion::Fatal(_)), "{codigo}");
        assert!(!variable(&p, "x"));
    }
    let f = Fixture::nuevo();
    f.escribir("ver", &simple("ver", "partida.disponible", "return true"));
    let mut p = f.partida();
    let vista = p.vista();
    let mundo = p.mundo.clone();
    for _ in 0..8 {
        assert_eq!(p.vista(), vista);
        assert_eq!(p.mundo, mundo);
    }
    terminar(&mut p, 0);
    p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
    let VistaSesion::Acampando(v) = p.vista() else {
        panic!()
    };
    p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
        .unwrap();
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
}

#[test]
fn carga_rechaza_ciclos_campos_referencias_y_controles_no_admitidos() {
    let f = Fixture::nuevo();
    let original = simple("ver", "partida.disponible", "return true");
    for texto in [
        original.replace("partida.disponible", "rescatar.inicio"),
        original.replace("clave:\"ver\",puntos", "clave:\"ver\",tiempo:2,puntos"),
        original.replace("control:Continuar", "control:InterrumpirAccion"),
        original.replace("Some(\"fin\")", "Some(\"ver\")"),
        original.replace("Some(\"fin\")", "Some(\"ausente\")"),
        original.replace("inicio:\"ver\"", "encadena:[\"ver\"],inicio:\"ver\""),
    ] {
        f.escribir("ver", &texto);
        assert!(crate::cargar_contenido(&f.0).is_err(), "{texto}");
    }
}

#[test]
fn mil_eventos_se_filtran_por_punto_sin_ejecutar_condiciones_ajenas() {
    let f = Fixture::nuevo();
    f.escribir(
        "inicio",
        &simple("inicio", "partida.disponible", "return true"),
    );
    for i in 0..999 {
        let k = format!("evento_{i}");
        f.escribir(
            &k,
            &simple(&k, "viajar.entre_lugares", "return funcionInexistente()"),
        );
    }
    let p = f.partida();
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento=="inicio"));
    assert_eq!(p.contenido.eventos().eventos.len(), 1000);
}

#[test]
fn prioridad_reevaluacion_y_contexto_inicial_incluyen_al_grupo() {
    let f = Fixture::nuevo();
    let condicion = "return #ctx:participantes() > 1";
    // Menor clave no gana a prioridad mayor. El segundo candidato se reevalúa
    // después de que el primero confirme sus efectos.
    let primero = r##"(clave:"z_primero",prioridad:10,puntos:["partida.disponible"],condicion:r#"return #ctx:participantes() > 1"#,inicio:"ver",pasos:[Vigneta(clave:"ver",presentacion:r#"return {titulo="Primero",texto="Primero"}"#,siguiente:Some("efecto")),Efectos(clave:"efecto",salidas:["fin"],resolver:r#"return {operaciones={op.establecer_booleana("habilitado",true)},siguiente="fin"}"#),Fin(clave:"fin",control:Continuar)])"##;
    f.escribir("z_primero", primero);
    f.escribir(
        "a_segundo",
        &simple(
            "a_segundo",
            "partida.disponible",
            "return mundo:booleana('habilitado',false)",
        ),
    );
    let mut p = f.partida();
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento == "z_primero"));
    seguir(&mut p, 0);
    assert!(variable(&p, "habilitado"));
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento == "a_segundo"));
    terminar(&mut p, 0);
    // El punto previo al viaje también incluye a los compañeros.
    f.escribir(
        "inicio_viaje",
        &simple("inicio_viaje", "viajar.inicio", condicion),
    );
    let mut p = f.partida();
    terminar(&mut p, 0);
    let destino = p
        .contenido
        .claves()
        .lugar_por_clave("hamada_de_zahrun")
        .unwrap();
    p.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento == "inicio_viaje"));
}

#[test]
fn permiso_del_encadenado_se_valida_en_el_punto_real_sin_efectos_parciales() {
    let f = Fixture::nuevo();
    f.escribir("a", r##"(clave:"a",puntos:["partida.disponible"],encadena:["b"],condicion:"return true",inicio:"efecto",pasos:[Efectos(clave:"efecto",salidas:["fin"],resolver:r#"return {operaciones={op.establecer_booleana("noConfirmar",true)},siguiente="fin",solicitar={"b"}}"#),Fin(clave:"fin",control:Continuar)])"##);
    f.escribir(
        "b",
        &simple("b", "viajar.inicio", "return true")
            .replace("control:Continuar", "control:InterrumpirAccion"),
    );
    let p = f.partida();
    assert!(matches!(p.vista(), VistaSesion::Fatal(_)));
    assert!(!variable(&p, "noConfirmar"));
}

#[test]
fn adaptador_sin_eventos_conserva_cada_paso_y_el_consumo_de_azar() {
    use desert_tiger_core::acciones::{self as base, intervenciones as nuevo};
    struct Contado(Vec<usize>);
    impl FuenteAzar for Contado {
        fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
            self.0.push(n.get());
            Ok(n.get() - 1)
        }
    }
    let f = Fixture::nuevo();
    let p = f.partida();
    for destino in base::destinos_disponibles(&p.mundo, p.mundo.actor_jugador()).unwrap() {
        if destino == p.mundo.ubicacion_grupo()
            || p.mundo
                .lugar(destino)
                .unwrap()
                .caracteristicas()
                .any(|c| c.como_str() == "oculto")
        {
            continue;
        }
        let solicitud = base::SolicitudAccion::Viajar {
            actor: p.mundo.actor_jugador(),
            destino,
        };
        let contexto = base::ContextoAccion {
            inicio: p.inicio,
            rumbos: &p.rumbos,
        };
        let mut mundo_a = p.mundo.clone();
        let mut mundo_b = p.mundo.clone();
        let mut azar_a = Contado(vec![]);
        let mut azar_b = Contado(vec![]);
        let inicio = base::iniciar(&mut mundo_a, solicitud, &contexto, &mut azar_a).unwrap();
        let mut ejecucion_a = inicio.ejecucion;
        let mut paso_a = inicio.paso;
        let (mut ejecucion_b, mut salida) = nuevo::iniciar(&mundo_b, solicitud, &contexto).unwrap();
        assert!(azar_b.0.is_empty());
        let nuevo::SalidaMotor::Punto(inicial) = &salida else {
            panic!()
        };
        let token_inicial = inicial.id;
        for _ in 0..500 {
            while let nuevo::SalidaMotor::Punto(punto) = salida {
                salida = nuevo::resolver_punto(
                    &mut mundo_b,
                    &mut ejecucion_b,
                    punto.id,
                    nuevo::ResolucionPunto::EjecutarGenerico,
                    &contexto,
                    &mut azar_b,
                )
                .unwrap();
                let mundo_antes = mundo_b.clone();
                let azar_antes = azar_b.0.clone();
                assert!(
                    nuevo::resolver_punto(
                        &mut mundo_b,
                        &mut ejecucion_b,
                        token_inicial,
                        nuevo::ResolucionPunto::EjecutarGenerico,
                        &contexto,
                        &mut azar_b
                    )
                    .is_err()
                );
                assert_eq!(mundo_b, mundo_antes);
                assert_eq!(azar_b.0, azar_antes);
            }
            let nuevo::SalidaMotor::Paso(paso_b) = salida else {
                panic!()
            };
            assert_eq!(paso_a, *paso_b);
            assert_eq!(mundo_a, mundo_b);
            assert_eq!(azar_a.0, azar_b.0);
            match paso_a {
                base::PasoAccion::Escena(r) => {
                    paso_a =
                        base::avanzar(&mut mundo_a, &mut ejecucion_a, r.id, &contexto, &mut azar_a)
                            .unwrap();
                    salida = nuevo::avanzar(
                        &mut mundo_b,
                        &mut ejecucion_b,
                        r.id,
                        &contexto,
                        &mut azar_b,
                    )
                    .unwrap();
                }
                base::PasoAccion::Decision(d) => {
                    let respuesta = base::RespuestaDecision {
                        paso: d.escena.id,
                        opcion: d.opciones[0],
                    };
                    paso_a = base::responder(
                        &mut mundo_a,
                        &mut ejecucion_a,
                        respuesta,
                        &contexto,
                        &mut azar_a,
                    )
                    .unwrap();
                    salida = nuevo::responder(
                        &mut mundo_b,
                        &mut ejecucion_b,
                        respuesta,
                        &contexto,
                        &mut azar_b,
                    )
                    .unwrap();
                }
                base::PasoAccion::Finalizada(_) => break,
            }
        }
        assert!(matches!(paso_a, base::PasoAccion::Finalizada(_)));
    }
}

#[test]
fn interrupcion_pendiente_muestra_la_cadena_antes_de_cerrar() {
    let f = Fixture::nuevo();
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/demos/eventos/events");
    copiar(&raiz, &f.0.join("events"));
    let a = std::fs::read_to_string(f.0.join("events/demo_cadena_a.ron")).unwrap();
    f.escribir(
        "demo_cadena_a",
        &a.replace("control:Continuar", "control:InterrumpirAccion"),
    );
    let mut p = f.partida();
    terminar(&mut p, 0);
    let origen = p.mundo.ubicacion_grupo();
    let destino = p
        .contenido
        .claves()
        .lugar_por_clave("valle_fertil")
        .unwrap();
    p.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    let vistos = terminar(&mut p, 0);
    assert!(vistos.iter().any(|k| k == "demo_cadena_b"));
    assert!(!vistos.iter().any(|k| k == "demo_llegada"));
    assert_ne!(p.mundo.ubicacion_grupo(), origen);
    assert_ne!(p.mundo.ubicacion_grupo(), destino);
    assert!(variable(&p, "demoCadenaA"));
}

#[test]
fn acampar_y_cierre_ofrecen_sus_puntos_sin_repetir_el_dia() {
    let f = Fixture::nuevo();
    for (k, punto) in [
        ("campamento", "acampar.inicio"),
        ("cierre", "accion.terminada"),
        ("npc", "fase_npc.terminada"),
    ] {
        f.escribir(k, &simple(k, punto, "return true"));
    }
    let mut p = f.partida();
    let inicial = p.mundo.fecha();
    p.procesar(IntencionFrontend::ComenzarCampamento).unwrap();
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento=="campamento"));
    seguir(&mut p, 0);
    let VistaSesion::Acampando(v) = p.vista() else {
        panic!()
    };
    assert_eq!(p.mundo.fecha(), inicial);
    p.procesar(IntencionFrontend::AbandonarCampamento { campamento: v.id })
        .unwrap();
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento=="cierre"));
    assert_eq!(p.mundo.fecha(), inicial);
    seguir(&mut p, 0);
    assert!(matches!(p.vista(),VistaSesion::PresentandoEvento(v) if v.evento=="npc"));
    let finalizada = p.mundo.fecha();
    assert_ne!(finalizada, inicial);
    seguir(&mut p, 0);
    assert!(matches!(p.vista(), VistaSesion::Disponible(_)));
    assert_eq!(p.mundo.fecha(), finalizada);
}
