use super::*;
use crate::{
    ActorInicial, Caracteristica, DatosInicialesMundo, ExtremoRelacion, Fecha, FraccionDia, IdMapa,
    LugarInicial, Mes, RelacionInicial, TipoRelacion,
};
use std::num::NonZeroUsize;

struct Azar(u64);
struct Dados(std::collections::VecDeque<usize>);
impl Dados {
    fn nuevos(dados: &[usize]) -> Self {
        Self(dados.iter().copied().collect())
    }
}
impl FuenteAzar for Dados {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, crate::azar::ErrorFuenteAzar> {
        let valor = self.0.pop_front().unwrap_or(0);
        assert!(valor < n.get(), "dado {valor} fuera de {}", n.get());
        Ok(valor)
    }
}

fn escenario(terrenos: &[&[&str]], actores: &[&[&str]]) -> Mundo {
    let mut iniciales: Vec<_> = actores
        .iter()
        .enumerate()
        .map(|(i, c)| ActorInicial {
            id: actor(i),
            ubicacion: crate::UbicacionActor::EnLugar(id(0)),
            caracteristicas: rasgos(c),
        })
        .collect();
    if iniciales.len() == 1 {
        iniciales.push(ActorInicial {
            id: actor(1),
            ubicacion: crate::UbicacionActor::EnLugar(id(0)),
            caracteristicas: vec![],
        });
    }
    let companeros = (1..iniciales.len()).map(actor).collect();
    let mut mundo = Mundo::nuevo(DatosInicialesMundo {
        mapa: IdMapa::desde_posicion(0).unwrap(),
        fecha: Fecha::nueva(2026, Mes::Enero, 1).unwrap(),
        lugares: terrenos
            .iter()
            .enumerate()
            .map(|(i, c)| LugarInicial {
                id: id(i),
                caracteristicas: rasgos(c),
            })
            .collect(),
        actores: iniciales,
        actor_jugador: actor(0),
        companeros,
        variables_globales: vec![],
        relaciones: (1..terrenos.len())
            .map(|i| RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id(i - 1)),
                destino: ExtremoRelacion::Lugar(id(i)),
            })
            .collect(),
    })
    .unwrap();
    if actores.len() == 1 {
        mundo
            .aplicar_lote(LoteOperaciones::uno(OperacionMundo::SepararCompanero {
                actor: actor(1),
                lugar: id(0),
            }))
            .unwrap();
    }
    mundo
}
impl FuenteAzar for Azar {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, crate::azar::ErrorFuenteAzar> {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        Ok((self.0 >> 32) as usize % n.get())
    }
}
fn id(i: usize) -> IdLugar {
    IdLugar::desde_posicion(i).unwrap()
}
fn actor(i: usize) -> IdActor {
    IdActor::desde_posicion(i).unwrap()
}
fn rasgos(lista: &[&str]) -> Vec<Caracteristica> {
    lista
        .iter()
        .map(|s| Caracteristica::nueva((*s).into()).unwrap())
        .collect()
}
fn mundo() -> Mundo {
    Mundo::nuevo(DatosInicialesMundo {
        mapa: IdMapa::desde_posicion(0).unwrap(),
        fecha: Fecha::nueva(1, Mes::Enero, 1).unwrap(),
        lugares: vec![
            LugarInicial {
                id: id(0),
                caracteristicas: rasgos(&["costero"]),
            },
            LugarInicial {
                id: id(1),
                caracteristicas: rasgos(&["desierto", "vasto"]),
            },
            LugarInicial {
                id: id(2),
                caracteristicas: rasgos(&["acuatico"]),
            },
        ],
        actores: vec![
            ActorInicial {
                id: actor(0),
                ubicacion: crate::UbicacionActor::EnLugar(id(0)),
                caracteristicas: rasgos(&["viajero", "jinete", "navegante", "expedicionario"]),
            },
            ActorInicial {
                id: actor(1),
                ubicacion: crate::UbicacionActor::EnLugar(id(0)),
                caracteristicas: vec![],
            },
        ],
        actor_jugador: actor(0),
        companeros: vec![actor(1)],
        variables_globales: vec![],
        relaciones: vec![
            RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id(0)),
                destino: ExtremoRelacion::Lugar(id(1)),
            },
            RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id(1)),
                destino: ExtremoRelacion::Lugar(id(2)),
            },
        ],
    })
    .unwrap()
}
#[test]
fn tiempo_exacto_y_resto() {
    let demora = HorasViaje::demora(168, 2000).unwrap();
    assert_eq!(demora.0, 672_000_000);
    let inicio = InstanteAccion {
        fecha: Fecha::nueva(2026, crate::Mes::Enero, 31).unwrap(),
        fraccion: FraccionDia::CERO,
    };
    let p = proyectar_tiempo(inicio, HorasViaje::horas(50).tiempo().unwrap()).unwrap();
    assert_eq!(p.fecha.dia(), 2);
    assert_eq!(p.fecha.mes(), crate::Mes::Febrero);
    assert_eq!(
        proyectar_tiempo(p, HorasViaje::horas(22).tiempo().unwrap())
            .unwrap()
            .fecha
            .dia(),
        3
    );
}
#[test]
fn consumo_acumulado_y_saldo_cero() {
    let mut r = ReservasViaje {
        comidas: [2, 0, 0, 0],
        botas: vec![500],
    };
    let mut a = Azar(2);
    let (h, c) =
        reservas::consumir(&mut r, HorasViaje::CERO, HorasViaje::horas(20), 1, &mut a).unwrap();
    assert_eq!(c.jornadas, 0);
    let (h, c) = reservas::consumir(&mut r, h, HorasViaje::horas(6), 1, &mut a).unwrap();
    assert_eq!(h, HorasViaje::horas(2));
    assert_eq!(c.jornadas, 1);
    assert!(c.sin_alimento.is_none());
    assert!(c.sin_agua.is_none());
    assert_eq!(r.peso().unwrap(), 100);
    let (_, c) = reservas::consumir(&mut r, h, HorasViaje::horas(24), 1, &mut a).unwrap();
    assert_eq!(c.sin_agua, Some(HorasViaje::horas(2).tiempo().unwrap()));
}
#[test]
fn recorrer_todos_los_medios_sin_desfase() {
    for semilla in 0..64 {
        let mut m = mundo();
        let mut azar = Azar(semilla);
        let r = CatalogoRumbos(std::collections::BTreeMap::from([
            ((id(1), id(2)), 0),
            ((id(2), id(1)), 4),
        ]));
        let c = ContextoAccion {
            inicio: InstanteAccion {
                fecha: m.fecha(),
                fraccion: FraccionDia::CERO,
            },
            rumbos: &r,
        };
        let inicio = iniciar(
            &mut m,
            SolicitudAccion::Viajar {
                actor: actor(0),
                destino: id(2),
            },
            &c,
            &mut azar,
        )
        .unwrap();
        let mut e = inicio.ejecucion;
        let mut paso = inicio.paso;
        let mut ultima = HorasViaje::CERO.tiempo().unwrap();
        let mut n = 0;
        loop {
            n += 1;
            assert!(n < 100);
            paso = match paso {
                PasoAccion::Escena(registro) => {
                    ultima = registro.total;
                    avanzar(&mut m, &mut e, registro.id, &c, &mut azar).unwrap()
                }
                PasoAccion::Decision(d) => {
                    let opcion = d.opciones[0];
                    responder(
                        &mut m,
                        &mut e,
                        RespuestaDecision {
                            paso: d.escena.id,
                            opcion,
                        },
                        &c,
                        &mut azar,
                    )
                    .unwrap()
                }
                PasoAccion::Finalizada(fin) => {
                    assert_eq!(ultima, fin.tiempo);
                    assert_eq!(m.fecha(), c.inicio.fecha);
                    break;
                }
            };
        }
    }
}

#[test]
fn cada_fila_de_incidente_a_pie_y_sus_estados() {
    for (dado, tipo) in [
        (1, Incidente::Agotamiento),
        (7, Incidente::Agotamiento),
        (8, Incidente::Esguince),
        (13, Incidente::Esguince),
        (14, Incidente::RumboPerdido),
        (17, Incidente::RumboPerdido),
        (18, Incidente::Separacion),
        (20, Incidente::Separacion),
    ] {
        let m = escenario(&[&[], &[]], &[&[], &[]]);
        let viajeros = [actor(0), actor(1)];
        let c = tarjetas::ContextoMovimiento {
            mundo: &m,
            viajeros: &viajeros,
            origen: id(0),
            destino: id(1),
            animales: 0,
            caballos: 0,
            camellos: 0,
            carga: [HorasViaje::CERO; 2],
            mes: 1,
            rumbo: None,
        };
        let mut azar = Dados::nuevos(&[0, 0, 1, dado - 1, 0]);
        let res = tarjetas::andar::resolver(&c, &mut azar).unwrap();
        assert_eq!(res.dato.incidente.as_ref().unwrap().tipo, tipo);
        if tipo == Incidente::Esguince {
            assert_eq!(res.duracion, HorasViaje::horas(172));
        }
        if tipo == Incidente::RumboPerdido {
            assert_eq!(res.duracion, HorasViaje::horas(174));
        }
    }
    let m = escenario(&[&[], &[]], &[&[]]);
    let viajeros = [actor(0)];
    let c = tarjetas::ContextoMovimiento {
        mundo: &m,
        viajeros: &viajeros,
        origen: id(0),
        destino: id(1),
        animales: 0,
        caballos: 0,
        camellos: 0,
        carga: [HorasViaje::CERO; 2],
        mes: 1,
        rumbo: None,
    };
    let res = tarjetas::andar::resolver(&c, &mut Dados::nuevos(&[0, 1, 19])).unwrap();
    assert_eq!(res.dato.incidente.unwrap().tipo, Incidente::Enfermedad);
    assert_eq!(res.duracion, HorasViaje::horas(198));
}

#[test]
fn caballo_y_camello_comparten_prueba_pero_no_duracion_ni_incidentes() {
    for (dado, tipo, demora) in [
        (1, Incidente::Obstinada, 4),
        (6, Incidente::Obstinada, 4),
        (7, Incidente::Detenida, 8),
        (11, Incidente::Detenida, 8),
        (12, Incidente::Asustada, 12),
        (15, Incidente::Asustada, 12),
        (16, Incidente::CrisisMontura, 48),
        (18, Incidente::CrisisMontura, 48),
        (19, Incidente::MuerteMontura, 0),
        (20, Incidente::MuerteMontura, 0),
    ] {
        let m = escenario(&[&[], &[]], &[&["jinete"]]);
        let viajeros = [actor(0)];
        let c = tarjetas::ContextoMovimiento {
            mundo: &m,
            viajeros: &viajeros,
            origen: id(0),
            destino: id(1),
            animales: 0,
            caballos: 1,
            camellos: 1,
            carga: [HorasViaje::CERO; 2],
            mes: 1,
            rumbo: None,
        };
        let a = tarjetas::cabalgar::resolver(&c, &mut Dados::nuevos(&[0, 1, dado - 1, 7])).unwrap();
        let b = tarjetas::camello::resolver(&c, &mut Dados::nuevos(&[0, 1, dado - 1, 7])).unwrap();
        assert_eq!(a.dato.chequeos, b.dato.chequeos);
        assert_eq!(a.dato.chequeos[0].accion, AccionParticular::Cabalgar);
        assert_eq!(a.dato.incidente.as_ref().unwrap().tipo, tipo);
        assert_eq!(b.dato.incidente.as_ref().unwrap().tipo, tipo);
        let muerte = dado >= 19;
        assert_eq!(
            a.duracion,
            HorasViaje::horas(if muerte { 48 + 84 } else { 96 } + demora)
        );
        assert_eq!(
            b.duracion,
            HorasViaje::horas(if muerte { 60 + 84 } else { 120 } + demora)
        );
        assert_eq!(a.caballos, if muerte { 0 } else { 1 });
        assert_eq!(b.camellos, if muerte { 0 } else { 1 });
    }
}

#[test]
fn conocimiento_natural_en_barco_y_viento_pueden_producir_cero_horas() {
    let m = escenario(
        &[&["pequeno", "costero"], &["pequeno", "acuatico"]],
        &[&["navegante"]],
    );
    let viajeros = [actor(0)];
    let c = tarjetas::ContextoMovimiento {
        mundo: &m,
        viajeros: &viajeros,
        origen: id(0),
        destino: id(1),
        animales: 0,
        caballos: 0,
        camellos: 0,
        carga: [HorasViaje::CERO; 2],
        mes: 1,
        rumbo: Some(0),
    };
    let r = tarjetas::navegar::resolver(&c, &mut Dados::nuevos(&[13, 17])).unwrap();
    assert_eq!(r.duracion, HorasViaje::CERO);
    assert!(r.dato.incidente.is_none());
    let conocidos: Vec<_> = r
        .operaciones
        .iter()
        .filter_map(|op| {
            if let OperacionMundo::RegistrarConocimiento { lugar, .. } = op {
                Some(*lugar)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(conocidos, vec![id(1)]);
    let mut contra = tarjetas::ContextoMovimiento {
        rumbo: Some(4),
        ..c
    };
    let r = tarjetas::navegar::resolver(&contra, &mut Dados::nuevos(&[19, 19])).unwrap();
    assert_eq!(r.duracion, HorasViaje::horas(72));
    contra.rumbo = Some(2);
    assert_eq!(
        tarjetas::navegar::resolver(&contra, &mut Dados::nuevos(&[19, 19]))
            .unwrap()
            .duracion,
        HorasViaje::horas(24)
    );
}

#[test]
fn todas_las_filas_de_navegacion_y_costas_de_naufragio() {
    for (dado, tipo, horas) in [
        (1, Incidente::Olas, 12),
        (6, Incidente::Olas, 12),
        (7, Incidente::Lluvia, 24),
        (12, Incidente::Lluvia, 24),
        (13, Incidente::SinViento, 48),
        (16, Incidente::SinViento, 48),
        (17, Incidente::Naufragio, 0),
        (18, Incidente::Naufragio, 0),
        (19, Incidente::PorLaBorda, 0),
        (20, Incidente::PorLaBorda, 0),
    ] {
        let m = escenario(&[&["costero"], &["acuatico"]], &[&["navegante"], &[]]);
        let viajeros = [actor(0), actor(1)];
        let c = tarjetas::ContextoMovimiento {
            mundo: &m,
            viajeros: &viajeros,
            origen: id(0),
            destino: id(1),
            animales: 0,
            caballos: 0,
            camellos: 0,
            carga: [HorasViaje::CERO; 2],
            mes: 1,
            rumbo: Some(2),
        };
        let r = tarjetas::navegar::resolver(&c, &mut Dados::nuevos(&[0, 1, dado - 1])).unwrap();
        assert_eq!(r.dato.incidente.as_ref().unwrap().tipo, tipo);
        assert_eq!(r.duracion, HorasViaje::horas(48 + horas));
        if tipo == Incidente::Naufragio {
            assert_eq!(r.cierre, Some(CierreViaje::Naufragio));
            assert_eq!(r.lugar, id(0));
        }
        if tipo == Incidente::PorLaBorda {
            assert_eq!(r.viajeros, vec![actor(0)]);
        }
    }
}

#[test]
fn separar_y_conocer_se_confirman_juntos_y_un_fallo_no_deja_indicios() {
    let mut m = mundo();
    let fallido = LoteOperaciones::nuevo(vec![
        OperacionMundo::RegistrarConocimiento {
            actor: actor(0),
            lugar: id(1),
        },
        OperacionMundo::SepararCompanero {
            actor: actor(1),
            lugar: id(0),
        },
        OperacionMundo::Trasladar {
            actor: actor(0),
            destino: id(99),
        },
    ])
    .unwrap();
    assert!(m.aplicar_lote(fallido).is_err());
    assert!(m.grupo().contiene(actor(1)));
    assert!(!m.actor_conoce_lugar(actor(0), id(1)).unwrap());
    let ops = vec![
        OperacionMundo::RegistrarConocimiento {
            actor: actor(0),
            lugar: id(1),
        },
        OperacionMundo::SepararCompanero {
            actor: actor(1),
            lugar: id(0),
        },
        OperacionMundo::Trasladar {
            actor: actor(0),
            destino: id(1),
        },
    ];
    m.aplicar_lote(LoteOperaciones::nuevo(ops).unwrap())
        .unwrap();
    assert!(!m.grupo().contiene(actor(1)));
    assert_eq!(m.ubicacion_actor(actor(1)), Some(id(0)));
    assert!(m.actor_conoce_lugar(actor(0), id(1)).unwrap());
    assert!(
        m.aplicar_lote(LoteOperaciones::uno(
            OperacionMundo::RegistrarConocimiento {
                actor: actor(0),
                lugar: id(1)
            }
        ))
        .unwrap()
        .como_slice()
        .is_empty()
    );
}

#[test]
fn lote_compartido_no_se_resortea_y_descarte_es_minimo_global() {
    let mut azar = Azar(9);
    let mut lote = reservas::LoteTentativo::default();
    let a = lote.materializar(120, 10_000, &mut azar).unwrap();
    let _ = lote.materializar(500, 30_000, &mut azar).unwrap();
    let estado = azar.0;
    assert_eq!(a, lote.materializar(120, 10_000, &mut azar).unwrap());
    assert_eq!(azar.0, estado);
    let r = ReservasViaje {
        comidas: [3, 2, 2, 2],
        botas: vec![0, 500],
    };
    for exceso in [1, 7, 13, 29, 100, 140, 600] {
        let lote = carga::descarte(&r, exceso).unwrap();
        let mut despues = r.clone();
        lote.quitar(&mut despues).unwrap();
        let retirado = r.peso().unwrap() - despues.peso().unwrap();
        let mut mejor = u64::MAX;
        for c in 0..=3 {
            for p in 0..=2 {
                for q in 0..=2 {
                    for s in 0..=2 {
                        for botas in 0..4 {
                            let peso = c * 10
                                + p * 12
                                + q * 15
                                + s * 7
                                + if botas & 1 > 0 { 100 } else { 0 }
                                + if botas & 2 > 0 { 600 } else { 0 };
                            if peso >= exceso {
                                mejor = mejor.min(peso);
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(retirado, mejor, "exceso {exceso}");
    }
}

#[test]
fn cancelacion_por_separacion_conserva_toda_la_conexion() {
    let mut m = escenario(&[&[], &[]], &[&[], &[]]);
    let mut azar = Azar(2);
    let r = CatalogoRumbos::default();
    let c = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &r,
    };
    let inicio = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(1),
        },
        &c,
        &mut azar,
    )
    .unwrap();
    let mut e = inicio.ejecucion;
    e.fase = Fase::Mover;
    e.reservas = Some(ReservasViaje {
        comidas: [100, 0, 0, 0],
        botas: vec![3000; 10],
    });
    e.demoras = vec![[HorasViaje::CERO; 2]];
    e.caballos = 0;
    e.camellos = 0;
    e.espera = None;
    let paso = e
        .producir(&mut m, &mut Dados::nuevos(&[0, 0, 1, 19]))
        .unwrap();
    let PasoAccion::Decision(d) = paso else {
        panic!("esperaba separación")
    };
    assert_eq!(e.total, HorasViaje::horas(24));
    let paso = responder(
        &mut m,
        &mut e,
        RespuestaDecision {
            paso: d.escena.id,
            opcion: OpcionViaje::Buscar,
        },
        &c,
        &mut azar,
    )
    .unwrap();
    let PasoAccion::Escena(r) = paso else {
        panic!()
    };
    assert_eq!(r.duracion, HorasViaje::horas(168).tiempo().unwrap());
    assert_eq!(r.consumo.jornadas, 7);
    assert_eq!(r.lugar, id(0));
    assert_eq!(r.participantes.len(), 2);
    assert_eq!(r.composicion_resultante.len(), 1);
    assert_eq!(m.ubicacion_actor(actor(1)), Some(id(0)));
    assert!(
        responder(
            &mut m,
            &mut e,
            RespuestaDecision {
                paso: d.escena.id,
                opcion: OpcionViaje::Buscar
            },
            &c,
            &mut azar
        )
        .is_err()
    );
}

#[test]
fn preparacion_estable_y_demora_exacta() {
    let m = escenario(&[&[], &[], &[], &[], &[], &[]], &[&[], &[]]);
    let ruta: Vec<_> = (0..6).map(id).collect();
    let (base, alternativa) =
        carga::preparar(&m, &ruta, &[actor(0), actor(1)], 10, &mut Azar(3)).unwrap();
    assert!(base.animales > 0);
    assert_eq!(carga::exceso(&base.reservas, 2, base.animales).unwrap(), 0);
    assert_eq!(
        base.referencia,
        rutas::sumar_horas(
            &rutas::tiempos(&m, &ruta, &[actor(0), actor(1)], 2, 2, base.animales).unwrap()
        )
        .unwrap()
    );
    if let Some(alt) = alternativa {
        assert_eq!(alt.animales, base.animales - 1);
        assert_eq!(
            alt.demoras,
            carga::plan(
                &rutas::tiempos(&m, &ruta, &[actor(0), actor(1)], 2, 2, alt.animales).unwrap(),
                alt.exceso
            )
            .unwrap()
        );
    }
    let plan = carga::plan(&[[84, 84]], 2000).unwrap();
    assert_eq!(
        carga::suma_plan(&plan).unwrap(),
        HorasViaje::demora(168, 2000).unwrap()
    );
    assert_eq!(
        HorasViaje::demora(24, 10000).unwrap(),
        HorasViaje::horas(24)
    );
    assert!(HorasViaje::demora(24, 10001).unwrap() > HorasViaje::horas(24));
}

#[test]
fn estados_no_se_acumulan_y_las_dos_condiciones_previas_si_suman() {
    for (rasgos, h) in [(&["cansado"][..], 180), (&["esguince", "cansado"][..], 188)] {
        let m = escenario(&[&[], &[]], &[rasgos, &[]]);
        let viajeros = [actor(0), actor(1)];
        let c = tarjetas::ContextoMovimiento {
            mundo: &m,
            viajeros: &viajeros,
            origen: id(0),
            destino: id(1),
            animales: 0,
            caballos: 0,
            camellos: 0,
            carga: [HorasViaje::CERO; 2],
            mes: 1,
            rumbo: None,
        };
        let r = tarjetas::andar::resolver(&c, &mut Dados::nuevos(&[0, 0, 1, 7, 0])).unwrap();
        assert_eq!(r.duracion, HorasViaje::horas(h));
        assert!(r.operaciones.is_empty());
    }
}

#[test]
fn descarte_de_botas_identicas_usa_ids_menores() {
    let r = ReservasViaje {
        comidas: [0; 4],
        botas: vec![0, 0, 0],
    };
    let lote = carga::descarte(&r, 200).unwrap();
    assert_eq!(lote.botas, vec![0, 1]);
}

#[test]
fn montanas_retrasan_todos_los_medios_y_penalizan_la_presencia_de_animales() {
    let m = escenario(
        &[
            &["montanoso"],
            &["montanoso"],
            &["montanoso", "pequeno"],
            &["montanoso", "vasto", "boscoso"],
        ],
        &[&[], &[]],
    );
    for medio in [Medio::Pie, Medio::Caballo, Medio::Camello, Medio::Barco] {
        for (animales, monturas, base) in [
            (0, false, 108),
            (1, false, 120),
            (0, true, 120),
            (2, true, 120),
        ] {
            assert_eq!(
                rutas::mitad(&m, id(0), medio, animales, monturas, 0).unwrap(),
                base
            );
            assert_eq!(
                rutas::mitad(&m, id(2), medio, animales, monturas, 0).unwrap(),
                base / 2
            );
            assert_eq!(
                rutas::mitad(&m, id(3), medio, animales, monturas, 0).unwrap(),
                base * 2
            );
        }
    }
    let viajeros = [actor(0), actor(1)];
    for (caballos, camellos, animales, horas) in [
        (0, 0, 0, 216),
        (2, 0, 0, 240),
        (0, 2, 0, 240),
        (0, 0, 1, 240),
        (1, 0, 0, 240),
    ] {
        let previstas =
            rutas::tiempos(&m, &[id(0), id(1)], &viajeros, caballos, camellos, animales).unwrap();
        assert_eq!(rutas::sumar_horas(&previstas).unwrap(), horas);
        let c = tarjetas::ContextoMovimiento {
            mundo: &m,
            viajeros: &viajeros,
            origen: id(0),
            destino: id(1),
            animales,
            caballos,
            camellos,
            carga: [HorasViaje::CERO; 2],
            mes: 1,
            rumbo: None,
        };
        let medio = rutas::medio(&m, id(0), id(1), viajeros.len(), caballos, camellos);
        let accion = if medio == Medio::Pie {
            AccionParticular::Andar
        } else {
            AccionParticular::Cabalgar
        };
        let mut movimiento = tarjetas::comenzar(&c, medio, accion, 5, &mut Azar(19)).unwrap();
        tarjetas::terminar_mitades(&c, &mut movimiento, [0, 0], false).unwrap();
        assert_eq!(movimiento.duracion, HorasViaje::horas(horas));
        assert!(
            movimiento
                .dato
                .mitades
                .iter()
                .all(|m| m.terreno_lento && m.horas_base == horas / 2)
        );
    }
}

#[test]
fn rutas_ponderan_tamano_y_destinos_son_consultas_puras() {
    let terrenos: [&[&str]; 5] = [
        &[],
        &["vasto", "montanoso"],
        &["pequeno"],
        &[],
        &["pequeno"],
    ];
    let m = Mundo::nuevo(DatosInicialesMundo {
        mapa: IdMapa::desde_posicion(0).unwrap(),
        fecha: Fecha::nueva(2026, Mes::Enero, 1).unwrap(),
        lugares: terrenos
            .iter()
            .enumerate()
            .map(|(i, c)| LugarInicial {
                id: id(i),
                caracteristicas: rasgos(c),
            })
            .collect(),
        actores: vec![
            ActorInicial {
                id: actor(0),
                ubicacion: crate::UbicacionActor::EnLugar(id(0)),
                caracteristicas: vec![],
            },
            ActorInicial {
                id: actor(1),
                ubicacion: crate::UbicacionActor::EnLugar(id(0)),
                caracteristicas: vec![],
            },
        ],
        actor_jugador: actor(0),
        companeros: vec![actor(1)],
        variables_globales: vec![],
        relaciones: [(0, 1), (1, 3), (0, 2), (2, 4), (4, 3)]
            .iter()
            .map(|&(a, b)| RelacionInicial {
                tipo: TipoRelacion::Conectado,
                origen: ExtremoRelacion::Lugar(id(a)),
                destino: ExtremoRelacion::Lugar(id(b)),
            })
            .collect(),
    })
    .unwrap();
    let rapida = rutas::rapida(&m, id(0), id(3), &[actor(0), actor(1)], &mut Azar(0)).unwrap();
    assert_eq!(rapida, vec![id(0), id(2), id(4), id(3)]);
    let larga = rutas::larga(&m, &rapida, &mut Azar(0)).unwrap();
    assert_eq!(larga.first(), Some(&id(0)));
    assert_eq!(larga.last(), Some(&id(3)));
    assert_eq!(
        larga
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        larga.len()
    );
    let destinos = crate::acciones::destinos_disponibles(&m, actor(0)).unwrap();
    assert_eq!(destinos.len(), 4);
    assert!(!destinos.contains(&id(0)));
}

#[test]
fn agotamiento_en_destino_no_busca_mas_y_una_busqueda_no_se_repite() {
    let mut m = escenario(&[&[], &[], &[]], &[&[], &[]]);
    let mut azar = Azar(0);
    let r = CatalogoRumbos::default();
    let c = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &r,
    };
    let mut e = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(2),
        },
        &c,
        &mut azar,
    )
    .unwrap()
    .ejecucion;
    e.reservas = Some(ReservasViaje::default());
    e.lugar = id(2);
    e.fase = Fase::Entre {
        revisar: true,
        busco: false,
    };
    e.espera = None;
    let PasoAccion::Escena(escena) = e.producir(&mut m, &mut azar).unwrap() else {
        panic!()
    };
    assert_eq!(escena.tarjeta, ClaveTarjeta::Llegada);
    e.lugar = id(0);
    e.posicion = 0;
    e.ruta = vec![id(0), id(1), id(2)];
    e.demoras = vec![[HorasViaje::CERO; 2]; 2];
    e.fase = Fase::Forrajear;
    e.espera = None;
    let paso = e.producir(&mut m, &mut Dados::nuevos(&[0, 0])).unwrap();
    let PasoAccion::Escena(escena) = paso else {
        panic!()
    };
    assert_eq!(escena.tarjeta, ClaveTarjeta::Forrajeo);
    assert_eq!(escena.cierre, Some(CierreViaje::SinRecursos));
    assert!(matches!(
        e.producir(&mut m, &mut azar).unwrap(),
        PasoAccion::Finalizada(_)
    ));
}

#[test]
fn chequeos_usan_bonos_exhaustivos_tope_y_validan_azar() {
    let m = escenario(
        &[&[], &[]],
        &[&["administrador", "expedicionario", "fuerte", "jinete"]],
    );
    let c = resolver_chequeo(
        &m,
        actor(0),
        AccionParticular::CalcularProvisiones,
        16,
        &mut Dados::nuevos(&[19]),
    )
    .unwrap();
    assert_eq!(c.natural, 20);
    assert_eq!(c.modificadores.len(), 2);
    assert_eq!(c.ajustado, 20);
    assert!(c.exito);
    let c = resolver_chequeo(
        &m,
        actor(0),
        AccionParticular::Cabalgar,
        15,
        &mut Dados::nuevos(&[0]),
    )
    .unwrap();
    assert_eq!(c.ajustado, 3);
    assert!(!c.exito);
    struct Invalido;
    impl FuenteAzar for Invalido {
        fn elegir_indice(
            &mut self,
            limite: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            Ok(limite.get())
        }
    }
    assert!(resolver_chequeo(&m, actor(0), AccionParticular::Andar, 12, &mut Invalido).is_err());
    assert!(HorasViaje(u128::MAX).sumar(HorasViaje::horas(1)).is_err());
    assert!(HorasViaje::demora(u64::MAX, u64::MAX).is_err());
    assert!(
        ReservasViaje {
            comidas: [u64::MAX; 4],
            botas: vec![]
        }
        .peso()
        .is_err()
    );
}

#[test]
fn entradas_viejas_o_azar_fallido_no_confirman_un_paso() {
    let mut m = escenario(&[&[], &[], &[]], &[&["viajero"], &[]]);
    let rumbos = CatalogoRumbos::default();
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(2),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    let PasoAccion::Decision(d) = paso else {
        panic!()
    };
    struct NoDebeSortear;
    impl FuenteAzar for NoDebeSortear {
        fn elegir_indice(
            &mut self,
            _: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            panic!("entrada inválida consumió azar")
        }
    }
    assert!(
        responder(
            &mut m,
            &mut ejecucion,
            RespuestaDecision {
                paso: d.escena.id,
                opcion: OpcionViaje::Descartar,
            },
            &contexto,
            &mut NoDebeSortear
        )
        .is_err()
    );
    assert!(
        avanzar(
            &mut m,
            &mut ejecucion,
            d.escena.id,
            &contexto,
            &mut NoDebeSortear
        )
        .is_err()
    );
    struct Fallido;
    impl FuenteAzar for Fallido {
        fn elegir_indice(
            &mut self,
            _: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            Err(crate::azar::ErrorFuenteAzar)
        }
    }
    let respuesta = RespuestaDecision {
        paso: d.escena.id,
        opcion: OpcionViaje::ElegirRuta(IdPropuestaRuta(0)),
    };
    assert!(responder(&mut m, &mut ejecucion, respuesta, &contexto, &mut Fallido).is_err());
    assert_eq!(ejecucion.total, HorasViaje::CERO);
    assert_eq!(m.ubicacion_actor(actor(0)), Some(id(0)));
    assert_eq!(m.fecha(), contexto.inicio.fecha);
    assert!(responder(&mut m, &mut ejecucion, respuesta, &contexto, &mut Azar(4)).is_ok());
}

#[test]
fn descarte_representativo_y_consumo_no_reducen_demora_asignada() {
    let reservas = ReservasViaje {
        comidas: [200, 100, 50, 30],
        botas: vec![3000; 12],
    };
    let inicio = std::time::Instant::now();
    let lote = carga::descarte(&reservas, 7000).unwrap();
    let mut reducidas = reservas.clone();
    lote.quitar(&mut reducidas).unwrap();
    assert!(reservas.peso().unwrap() - reducidas.peso().unwrap() >= 7000);
    eprintln!("descarte representativo: {:?}", inicio.elapsed());

    let mut m = escenario(&[&[], &[]], &[&[], &[]]);
    let rumbos = CatalogoRumbos::default();
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let mut e = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(1),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap()
    .ejecucion;
    e.reservas = Some(reservas);
    e.demoras = carga::plan(&[[84, 84]], 2000).unwrap();
    let plan = e.demoras.clone();
    let peso_antes = e.reservas.as_ref().unwrap().peso().unwrap();
    e.fase = Fase::Mover;
    e.espera = None;
    let PasoAccion::Escena(_) = e
        .producir(&mut m, &mut Dados::nuevos(&[0, 19, 19]))
        .unwrap()
    else {
        panic!()
    };
    assert!(e.reservas.as_ref().unwrap().peso().unwrap() < peso_antes);
    assert_eq!(e.demoras, plan);
}

#[test]
fn llegada_conserva_carencia_que_comienza_en_la_ultima_frontera() {
    let mut m = escenario(&[&[], &[]], &[&[], &[]]);
    let rumbos = CatalogoRumbos::default();
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let mut e = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(1),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap()
    .ejecucion;
    e.reservas = Some(ReservasViaje::default());
    e.h = HorasViaje::CERO;
    e.registro(
        &mut m,
        ClaveTarjeta::Andar,
        ContenidoEscena::Incompleto {
            inicial: false,
            falta_comida: true,
            falta_agua: true,
        },
        HorasViaje::horas(24),
        EfectosEscena {
            participantes: vec![actor(0), actor(1)],
            operaciones: vec![],
        },
        &mut Azar(0),
    )
    .unwrap();
    let PasoAccion::Escena(r) = e.llegar(&mut m, &mut Azar(0)).unwrap() else {
        panic!()
    };
    let cero = Some(HorasViaje::CERO.tiempo().unwrap());
    assert_eq!(r.consumo.sin_alimento, cero);
    assert_eq!(r.consumo.sin_agua, cero);
}

#[test]
fn destinos_seleccionables_combinan_vecinos_y_conocimiento_del_grupo_actual() {
    use crate::acciones::{destinos_disponibles, destinos_seleccionables_jugador};
    let mut m = escenario(&[&[], &[], &[], &[], &[], &[]], &[&[], &[], &[]]);
    m.aplicar_lote(
        LoteOperaciones::nuevo(vec![
            OperacionMundo::RegistrarConocimiento {
                actor: actor(0),
                lugar: id(0),
            },
            OperacionMundo::RegistrarConocimiento {
                actor: actor(0),
                lugar: id(5),
            },
            OperacionMundo::RegistrarConocimiento {
                actor: actor(1),
                lugar: id(3),
            },
            OperacionMundo::RegistrarConocimiento {
                actor: actor(1),
                lugar: id(5),
            },
            OperacionMundo::RegistrarConocimiento {
                actor: actor(2),
                lugar: id(4),
            },
            OperacionMundo::SepararCompanero {
                actor: actor(2),
                lugar: id(0),
            },
        ])
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        destinos_seleccionables_jugador(&m).unwrap(),
        vec![id(1), id(3), id(5)]
    );
    // El camino al destino conocido atraviesa 2 y 4, que el grupo desconoce.
    assert_eq!(
        rutas::rapida(&m, id(0), id(5), &[actor(0), actor(1)], &mut Azar(0)).unwrap(),
        vec![id(0), id(1), id(2), id(3), id(4), id(5)]
    );
    assert_eq!(
        destinos_disponibles(&m, actor(2)).unwrap(),
        vec![id(1), id(2), id(3), id(4), id(5)]
    );
    m.aplicar_lote(LoteOperaciones::uno(OperacionMundo::SepararCompanero {
        actor: actor(1),
        lugar: id(0),
    }))
    .unwrap();
    assert_eq!(
        destinos_seleccionables_jugador(&m).unwrap(),
        vec![id(1), id(5)]
    );
    m.aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
        actor: actor(0),
        destino: id(1),
    }))
    .unwrap();
    assert_eq!(
        destinos_seleccionables_jugador(&m).unwrap(),
        vec![id(0), id(2), id(5)]
    );
    m.aplicar_lote(LoteOperaciones::uno(
        OperacionMundo::RegistrarConocimiento {
            actor: actor(0),
            lugar: id(3),
        },
    ))
    .unwrap();
    assert_eq!(
        destinos_seleccionables_jugador(&m).unwrap(),
        vec![id(0), id(2), id(3), id(5)]
    );
    let solo = escenario(&[&[]], &[&[]]);
    assert!(destinos_seleccionables_jugador(&solo).unwrap().is_empty());
}

#[test]
fn lugares_ocultos_permiten_cruzar_hasta_la_siguiente_salida_visible() {
    use crate::acciones::{destinos_disponibles, destinos_seleccionables_jugador};
    let mut m = escenario(&[&[], &["oculto"], &["oculto"], &[], &[]], &[&[]]);
    m.aplicar_lote(LoteOperaciones::uno(
        OperacionMundo::RegistrarConocimiento {
            actor: actor(0),
            lugar: id(1),
        },
    ))
    .unwrap();
    assert_eq!(destinos_seleccionables_jugador(&m).unwrap(), vec![id(3)]);
    assert!(destinos_disponibles(&m, actor(0)).unwrap().contains(&id(1)));
    assert_eq!(
        rutas::rapida(&m, id(0), id(3), &[actor(0), actor(1)], &mut Azar(0)).unwrap(),
        vec![id(0), id(1), id(2), id(3)]
    );
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &CatalogoRumbos::default(),
    };
    assert!(matches!(
        iniciar(
            &mut m,
            SolicitudAccion::Viajar {
                actor: actor(0),
                destino: id(1)
            },
            &contexto,
            &mut Azar(0)
        ),
        Err(ErrorMotor::Rechazo(RechazoAccion::DestinoInvalido))
    ));
    assert!(
        iniciar(
            &mut m,
            SolicitudAccion::Viajar {
                actor: actor(0),
                destino: id(3)
            },
            &contexto,
            &mut Azar(0)
        )
        .is_ok()
    );
}

fn escenario_caminos(montana_vasta: bool) -> Mundo {
    let base = escenario(
        &[
            &[],
            &[],
            &["boscoso"],
            if montana_vasta {
                &["montanoso", "vasto"]
            } else {
                &["montanoso"]
            },
            &[],
            &[],
        ],
        &[&["viajero", "jinete", "montaraz"], &["explorador"]],
    );
    let mut relaciones: Vec<_> = [(0, 1), (1, 5), (0, 2), (2, 4), (4, 5), (0, 3), (3, 5)]
        .into_iter()
        .map(|(a, b)| RelacionInicial {
            tipo: TipoRelacion::Conectado,
            origen: ExtremoRelacion::Lugar(id(a)),
            destino: ExtremoRelacion::Lugar(id(b)),
        })
        .collect();
    relaciones.extend([2, 4].into_iter().map(|l| RelacionInicial {
        tipo: TipoRelacion::Conoce,
        origen: ExtremoRelacion::Actor(actor(1)),
        destino: ExtremoRelacion::Lugar(id(l)),
    }));
    Mundo::nuevo(DatosInicialesMundo {
        mapa: IdMapa::desde_posicion(0).unwrap(),
        fecha: base.fecha(),
        lugares: base
            .lugares()
            .iter()
            .map(|l| LugarInicial {
                id: l.id(),
                caracteristicas: l.caracteristicas().cloned().collect(),
            })
            .collect(),
        actores: base
            .actores()
            .iter()
            .map(|a| ActorInicial {
                id: a.id(),
                ubicacion: a.ubicacion(),
                caracteristicas: a.caracteristicas().cloned().collect(),
            })
            .collect(),
        actor_jugador: actor(0),
        companeros: vec![actor(1)],
        variables_globales: vec![],
        relaciones,
    })
    .unwrap()
}

#[test]
fn propuestas_agrupan_respaldos_y_descartan_montanas_con_rodeo_excesivo() {
    for vasta in [false, true] {
        let m = escenario_caminos(vasta);
        let viajeros = [actor(0), actor(1)];
        let rapida = rutas::rapida(&m, id(0), id(5), &viajeros, &mut Azar(0)).unwrap();
        assert_eq!(rapida, vec![id(0), id(1), id(5)]);
        let larga = vec![id(0), id(2), id(4), id(5)];
        let propuestas = propuestas::generar(&m, &viajeros, &rapida, &larga).unwrap();
        assert_eq!(
            propuestas,
            propuestas::generar(&m, &viajeros, &rapida, &larga).unwrap()
        );
        assert_eq!(propuestas[0].ruta, rapida);
        assert_eq!(
            propuestas[0]
                .respaldos
                .iter()
                .filter(|r| r.perfil == PerfilRuta::Rapida)
                .count(),
            2
        );
        assert!(
            propuestas[0]
                .respaldos
                .iter()
                .any(|r| r.perfil == PerfilRuta::Caballo)
        );
        assert!(propuestas.iter().any(|p| {
            p.ruta == larga
                && p.respaldos
                    .iter()
                    .any(|r| r.perfil == PerfilRuta::Conocidos)
        }));
        assert_eq!(
            propuestas
                .iter()
                .any(|p| p.ruta == vec![id(0), id(3), id(5)]),
            !vasta
        );
        let techo = propuestas::tiempo(&m, &viajeros, &larga).unwrap();
        for p in &propuestas {
            assert!(propuestas::tiempo(&m, &viajeros, &p.ruta).unwrap() <= techo);
            assert_eq!(
                p.previsto,
                HorasViaje::horas(propuestas::tiempo(&m, &viajeros, &p.ruta).unwrap())
                    .tiempo()
                    .unwrap()
            );
        }
        assert_eq!(
            propuestas
                .iter()
                .map(|p| &p.ruta)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            propuestas.len()
        );
        let iguales = propuestas::generar(&m, &viajeros, &rapida, &rapida).unwrap();
        assert_eq!(iguales.len(), 1);
        assert_eq!(iguales[0].ruta, rapida);
    }
}

#[test]
fn vecino_omite_deliberacion_y_otro_destino_ofrece_improvisar_o_cancelar() {
    let mut m = escenario_caminos(false);
    let rumbos = CatalogoRumbos::default();
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let directo = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(1),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    assert_eq!(directo.ejecucion.ruta, vec![id(0), id(1)]);
    assert!(directo.ejecucion.propuestas.is_empty() && directo.ejecucion.larga.is_empty());
    assert!(directo.ejecucion.visible);
    match directo.paso {
        PasoAccion::Escena(r) => assert!(!matches!(r.contenido, ContenidoEscena::Plan { .. })),
        PasoAccion::Decision(d) => {
            assert!(!matches!(d.escena.contenido, ContenidoEscena::Plan { .. }))
        }
        _ => panic!(),
    }

    let mut m = escenario(&[&[], &[], &[]], &[&[], &[]]);
    let inicio = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(2),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    let PasoAccion::Decision(d) = inicio.paso else {
        panic!()
    };
    assert_eq!(
        d.opciones,
        vec![OpcionViaje::Improvisar, OpcionViaje::Cancelar]
    );
    assert!(
        matches!(d.escena.contenido,ContenidoEscena::Plan{ref propuestas} if propuestas.is_empty())
    );
    assert_eq!(inicio.ejecucion.total, HorasViaje::CERO);
    let mut e = inicio.ejecucion;
    let salida = responder(
        &mut m,
        &mut e,
        RespuestaDecision {
            paso: d.escena.id,
            opcion: OpcionViaje::Cancelar,
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    assert!(matches!(
        salida,
        PasoAccion::Escena(_) | PasoAccion::Finalizada(_)
    ));
    assert_eq!(e.total, HorasViaje::horas(24));
    assert_eq!(m.ubicacion_grupo(), id(0));
}

#[test]
fn elegir_copia_la_propuesta_y_un_oculto_intermedio_no_es_viaje_directo() {
    let mut m = escenario_caminos(false);
    let rumbos = CatalogoRumbos::default();
    let contexto = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(5),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    let PasoAccion::Decision(d) = paso else {
        panic!()
    };
    for p in ejecucion.propuestas.clone() {
        let mut e = ejecucion.clone();
        let mut copia = escenario_caminos(false);
        responder(
            &mut copia,
            &mut e,
            RespuestaDecision {
                paso: d.escena.id,
                opcion: OpcionViaje::ElegirRuta(p.id),
            },
            &contexto,
            &mut Azar(0),
        )
        .unwrap();
        assert_eq!(e.ruta, p.ruta);
        assert!(e.visible);
    }
    assert!(
        responder(
            &mut m,
            &mut ejecucion,
            RespuestaDecision {
                paso: d.escena.id,
                opcion: OpcionViaje::ElegirRuta(IdPropuestaRuta(u32::MAX))
            },
            &contexto,
            &mut Azar(0)
        )
        .is_err()
    );
    assert_eq!(ejecucion.total, HorasViaje::CERO);
    let mut m = escenario(&[&[], &["oculto"], &[]], &[&["viajero"], &[]]);
    let inicio = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(0),
            destino: id(2),
        },
        &contexto,
        &mut Azar(0),
    )
    .unwrap();
    assert!(
        matches!(inicio.paso,PasoAccion::Decision(d) if matches!(d.escena.contenido,ContenidoEscena::Plan{..}))
    );
}

#[test]
fn especialidad_no_elude_la_habilitacion_de_la_ruta_optima() {
    for rasgos in [&["jinete"][..], &["montaraz"][..]] {
        let m = escenario(&[&[], &["montanoso"], &[]], &[rasgos, &[]]);
        let ruta = [id(0), id(1), id(2)];
        assert!(
            propuestas::generar(&m, &[actor(0), actor(1)], &ruta, &ruta)
                .unwrap()
                .is_empty()
        );
    }
}
