use super::*;
use std::num::NonZeroUsize;

struct Dado(u8);
impl FuenteAzar for Dado {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, crate::azar::ErrorFuenteAzar> {
        Ok(if n.get() == 20 {
            usize::from(self.0 - 1)
        } else {
            0
        })
    }
}
fn id(i: usize) -> IdLugar {
    IdLugar::desde_posicion(i).unwrap()
}
fn actor() -> IdActor {
    IdActor::desde_posicion(0).unwrap()
}
fn mundo(n: usize) -> Mundo {
    let terrenos = vec![&["paso", "montanoso"][..]; n];
    super::super::pruebas::escenario(&terrenos, &[&["viajero"], &[]])
}
fn siguiente(
    m: &mut Mundo,
    e: &mut EjecucionAccion,
    paso: PasoAccion,
    ctx: &ContextoAccion<'_>,
    dado: u8,
) -> PasoAccion {
    match paso {
        PasoAccion::Escena(r) => avanzar(m, e, r.id, ctx, &mut Dado(dado)).unwrap(),
        PasoAccion::Decision(d) => {
            let opcion = *d
                .opciones
                .iter()
                .find(|o| {
                    !matches!(
                        o,
                        OpcionViaje::Cancelar
                            | OpcionViaje::Improvisar
                            | OpcionViaje::AceptarDemora
                    )
                })
                .unwrap_or(&d.opciones[0]);
            responder(
                m,
                e,
                RespuestaDecision {
                    paso: d.escena.id,
                    opcion,
                },
                ctx,
                &mut Dado(dado),
            )
            .unwrap()
        }
        _ => panic!("fin inesperado"),
    }
}

struct Recorrido {
    registros: Vec<RegistroEscena>,
    fin: FinalizacionAccion,
    mundo: Mundo,
}
fn recorrer(
    n: usize,
    dado: u8,
    forzar_animales: bool,
    fracciones: bool,
    escasez: bool,
) -> Recorrido {
    let mut m = mundo(n);
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        mut paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(n - 1),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let mut registros = Vec::new();
    for _ in 0..256 {
        let registro = match &paso {
            PasoAccion::Escena(r) => r,
            PasoAccion::Decision(d) => &d.escena,
            PasoAccion::Finalizada(fin) => {
                return Recorrido {
                    registros,
                    fin: fin.clone(),
                    mundo: m,
                };
            }
        };
        if registro.tarjeta == ClaveTarjeta::Preparacion {
            if forzar_animales {
                ejecucion.animales = 3;
                ejecucion.reservas = Some(ReservasViaje {
                    comidas: [10000, 0, 0, 0],
                    botas: vec![3000; 20],
                });
            }
            if fracciones {
                ejecucion.demoras = vec![[HorasViaje(3), HorasViaje(4)]; n - 1];
            }
            if escasez {
                ejecucion.reservas = Some(ReservasViaje {
                    comidas: [1, 0, 0, 0],
                    botas: vec![500],
                });
            }
        }
        registros.push(registro.clone());
        paso = siguiente(
            &mut m,
            &mut ejecucion,
            paso,
            &ctx,
            if escasez && registros.last().unwrap().tarjeta == ClaveTarjeta::PasoMontanoso {
                1
            } else {
                dado
            },
        );
    }
    panic!("recorrido sin progreso")
}
fn momentos(r: &Recorrido) -> Vec<&RegistroPasoMontanoso> {
    r.registros
        .iter()
        .filter_map(|r| {
            if let ContenidoEscena::PasoMontanoso(p) = &r.contenido {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn dos_tres_cuatro_cinco_y_mas_lugares_cuentan_un_solo_episodio() {
    for n in [2, 3, 4, 5, 9] {
        let r = recorrer(n, 20, false, false, false);
        let momentos = momentos(&r);
        assert_eq!(r.fin.cierre, CierreViaje::Llegada);
        assert_eq!(momentos.len(), if n % 2 == 1 { 7 } else { 6 });
        assert_eq!(momentos[0].momento, MomentoPaso::Panorama);
        assert_eq!(momentos.last().unwrap().momento, MomentoPaso::OtraVertiente);
        assert_eq!(momentos.iter().filter(|p| p.chequeo.is_some()).count(), 4);
        assert!(momentos.iter().all(|p| {
            p.chequeo.as_ref().is_none_or(|c| {
                c.exito && c.accion == AccionParticular::Andar && c.dificultad == 10
            })
        }));
        if n % 2 == 1 {
            let campamento = momentos
                .iter()
                .find(|p| p.momento == MomentoPaso::Campamento)
                .unwrap();
            assert_eq!(campamento.lugar_confirmado, id(n / 2));
        }
        assert_eq!(
            momentos.last().unwrap().traslado_acumulado,
            HorasViaje::horas((n - 1) as u64 * 168).tiempo().unwrap()
        );
        assert_eq!(r.mundo.ubicacion_grupo(), id(n - 1));
        assert!(
            !r.registros
                .iter()
                .any(|r| matches!(r.contenido, ContenidoEscena::Desplazamiento(_)))
        );
        let visitados: Vec<_> = r
            .registros
            .iter()
            .flat_map(|r| &r.cambios)
            .filter_map(|c| {
                if let crate::CambioConfirmado::ActoresTrasladados { destino, .. } = c {
                    Some(*destino)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(visitados, (1..n).map(id).collect::<Vec<_>>());
    }
}

#[test]
fn fraccionar_no_duplica_ni_pierde_tiempo_ni_raciones() {
    let r = recorrer(5, 20, false, true, false);
    let inicio=r.registros.iter().find(|r|matches!(&r.contenido,ContenidoEscena::PasoMontanoso(p) if p.momento==MomentoPaso::Panorama)).unwrap();
    let fin=r.registros.iter().find(|r|matches!(&r.contenido,ContenidoEscena::PasoMontanoso(p) if p.momento==MomentoPaso::OtraVertiente)).unwrap();
    let p = if let ContenidoEscena::PasoMontanoso(p) = &fin.contenido {
        p
    } else {
        panic!()
    };
    let esperado = HorasViaje::horas(672).sumar(HorasViaje(28)).unwrap();
    assert_eq!(partes(p.traslado_acumulado), esperado);
    assert_eq!(p.previsto, p.traslado_acumulado);
    assert_eq!(p.otras_actividades, HorasViaje::CERO.tiempo().unwrap());
    let mut reservas = inicio.reservas.clone().unwrap();
    let (_, consumo) = reservas::consumir(
        &mut reservas,
        partes(inicio.horas_desde_racion),
        esperado,
        inicio.participantes.len(),
        &mut Dado(20),
    )
    .unwrap();
    assert_eq!(fin.reservas.as_ref(), Some(&reservas));
    let consumidas: u64 = r
        .registros
        .iter()
        .filter(|r| r.tarjeta == ClaveTarjeta::PasoMontanoso)
        .map(|r| r.consumo.comidas[0])
        .sum();
    assert_eq!(consumidas, consumo.comidas[0]);
    assert_eq!(partes(fin.total).0 - partes(inicio.total).0, esperado.0);
}

#[test]
fn fallos_suman_ochenta_horas_y_pierden_comida_proporcional_sin_agua() {
    let r = recorrer(3, 1, true, false, false);
    let m = momentos(&r);
    let fin = m.last().unwrap();
    assert_eq!(partes(fin.traslado_acumulado), HorasViaje::horas(336 + 80));
    let cornisa=r.registros.iter().position(|r|matches!(&r.contenido,ContenidoEscena::PasoMontanoso(p) if p.momento==MomentoPaso::Cornisa)).unwrap();
    let actual = &r.registros[cornisa];
    let anterior = &r.registros[cornisa - 1];
    let p = if let ContenidoEscena::PasoMontanoso(p) = &actual.contenido {
        p
    } else {
        panic!()
    };
    let cantidad_previa =
        anterior.reservas.as_ref().unwrap().comidas[0] - actual.consumo.comidas[0];
    assert_eq!(p.animales_perdidos, 1);
    assert_eq!(p.comida_perdida[0], cantidad_previa.div_ceil(3));
    assert_eq!(actual.animales_carga, 2);
    assert_eq!(
        actual.reservas.as_ref().unwrap().comidas[0],
        cantidad_previa - p.comida_perdida[0]
    );
    assert_eq!(
        actual.reservas.as_ref().unwrap().agua().unwrap(),
        anterior.reservas.as_ref().unwrap().agua().unwrap() - actual.consumo.agua_ml
    );
    assert_eq!(r.fin.cierre, CierreViaje::Llegada);
}

#[test]
fn agotamiento_interrumpe_y_no_narra_un_cruce_que_no_termino() {
    let r = recorrer(5, 20, false, false, true);
    assert_eq!(r.fin.cierre, CierreViaje::SinRecursos);
    assert!(
        r.registros
            .iter()
            .any(|r| r.tarjeta == ClaveTarjeta::Forrajeo)
    );
    assert!(
        !momentos(&r)
            .iter()
            .any(|p| p.momento == MomentoPaso::OtraVertiente)
    );
    assert_ne!(r.mundo.ubicacion_grupo(), id(4));
}

#[test]
fn reconocer_exige_ambos_rasgos_y_sigue_solo_el_segmento_de_ruta() {
    let m = super::super::pruebas::escenario(
        &[
            &["montanoso"],
            &["paso", "montanoso"],
            &["paso", "montanoso"],
            &["paso"],
            &["paso", "montanoso"],
        ],
        &[&[], &[]],
    );
    let ruta = [id(0), id(1), id(2), id(3), id(4)];
    assert_eq!(reconocer(&m, &ruta, 0), None);
    assert_eq!(
        reconocer(&m, &ruta, 1),
        Some(SegmentoPaso { inicio: 1, fin: 2 })
    );
    assert_eq!(reconocer(&m, &ruta, 2), None);
    assert_eq!(
        reconocer(&m, &[id(2), id(1)], 0),
        Some(SegmentoPaso { inicio: 0, fin: 1 })
    );
}

#[test]
fn fallo_de_azar_no_confirma_avances_parciales() {
    struct Fallido;
    impl FuenteAzar for Fallido {
        fn elegir_indice(
            &mut self,
            _: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            Err(crate::azar::ErrorFuenteAzar)
        }
    }
    let mut m = mundo(2);
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(1),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let paso = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    let PasoAccion::Escena(panorama) = paso else {
        panic!()
    };
    assert!(
        matches!(panorama.contenido,ContenidoEscena::PasoMontanoso(ref p) if p.momento==MomentoPaso::Panorama)
    );
    let antes = ejecucion.clone();
    assert!(avanzar(&mut m, &mut ejecucion, panorama.id, &ctx, &mut Fallido).is_err());
    assert_eq!(ejecucion.total, antes.total);
    assert_eq!(ejecucion.posicion, antes.posicion);
    assert_eq!(ejecucion.reservas, antes.reservas);
    assert_eq!(ejecucion.ordinal, antes.ordinal);
    assert_eq!(m.ubicacion_grupo(), id(0));
    let siguiente = avanzar(&mut m, &mut ejecucion, panorama.id, &ctx, &mut Dado(20)).unwrap();
    assert!(
        matches!(siguiente,PasoAccion::Escena(r) if matches!(&r.contenido,ContenidoEscena::PasoMontanoso(p) if p.momento==MomentoPaso::CaminoDificil))
    );
}

#[test]
fn sin_animales_se_hacen_las_cuatro_pruebas_sin_inventar_perdidas() {
    let r = recorrer(2, 1, false, false, false);
    let p = momentos(&r)
        .into_iter()
        .find(|p| p.momento == MomentoPaso::Cornisa)
        .unwrap();
    assert!(!p.chequeo.as_ref().unwrap().exito);
    assert_eq!(p.animales_perdidos, 0);
    assert_eq!(p.comida_perdida, [0; 4]);
    assert_eq!(p.demora, HorasViaje::CERO.tiempo().unwrap());
    assert_eq!(
        momentos(&r).iter().filter(|p| p.chequeo.is_some()).count(),
        4
    );
}

#[test]
fn buscar_comida_y_reanudar_conserva_momentos_y_cuenta_solo_el_traslado() {
    let mut m = mundo(3);
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        mut paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(2),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let mut lista = Vec::new();
    let mut busquedas = 0;
    for _ in 0..100 {
        let r = match &paso {
            PasoAccion::Escena(r) => r,
            PasoAccion::Decision(d) => &d.escena,
            PasoAccion::Finalizada(f) => {
                assert_eq!(f.cierre, CierreViaje::Llegada);
                break;
            }
        };
        if r.tarjeta == ClaveTarjeta::Preparacion {
            ejecucion.reservas = Some(ReservasViaje {
                comidas: [1, 0, 0, 0],
                botas: vec![500],
            });
        }
        if r.tarjeta == ClaveTarjeta::Forrajeo {
            busquedas += 1;
        }
        if let ContenidoEscena::PasoMontanoso(p) = &r.contenido {
            lista.push(p.clone());
        }
        paso = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    }
    assert_eq!(busquedas, 1);
    assert_eq!(lista.len(), 7);
    let ultimo = lista.last().unwrap();
    assert_eq!(ultimo.momento, MomentoPaso::OtraVertiente);
    assert_eq!(
        ultimo.otras_actividades,
        HorasViaje::horas(6).tiempo().unwrap()
    );
    assert_eq!(
        ultimo.traslado_acumulado,
        HorasViaje::horas(336).tiempo().unwrap()
    );
    assert_eq!(
        lista
            .iter()
            .filter(|p| p.momento == MomentoPaso::Campamento)
            .count(),
        1
    );
}

#[test]
fn tiempo_restante_descuenta_la_conexion_parcial() {
    let mut m = mundo(2);
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(1),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let panorama = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    siguiente(&mut m, &mut ejecucion, panorama, &ctx, 20);
    assert_eq!(ejecucion.posicion, 0);
    assert_eq!(
        ejecucion.restante(&m).unwrap(),
        HorasViaje(HorasViaje::horas(168).0 * 4 / 5)
    );
}

#[test]
fn npc_independiente_cruza_solo_sin_mover_al_jugador() {
    let mut m = mundo(3);
    let npc = IdActor::desde_posicion(1).unwrap();
    m.aplicar_lote(LoteOperaciones::uno(OperacionMundo::SepararCompanero {
        actor: npc,
        lugar: id(0),
    }))
    .unwrap();
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        mut paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: npc,
            destino: id(2),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let mut momentos = 0;
    for _ in 0..100 {
        let r = match &paso {
            PasoAccion::Escena(r) => r,
            PasoAccion::Decision(d) => &d.escena,
            PasoAccion::Finalizada(f) => {
                assert_eq!(f.cierre, CierreViaje::Llegada);
                break;
            }
        };
        if matches!(r.contenido, ContenidoEscena::PasoMontanoso(_)) {
            momentos += 1;
            assert_eq!(r.participantes, vec![npc]);
        }
        paso = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    }
    assert_eq!(momentos, 7);
    assert_eq!(m.ubicacion_actor(npc), Some(id(2)));
    assert_eq!(m.ubicacion_grupo(), id(0));
}

#[test]
fn error_despues_de_una_frontera_no_deja_traslados_publicados() {
    struct FalloEnChequeo;
    impl FuenteAzar for FalloEnChequeo {
        fn elegir_indice(
            &mut self,
            n: NonZeroUsize,
        ) -> Result<usize, crate::azar::ErrorFuenteAzar> {
            if n.get() == 20 {
                Err(crate::azar::ErrorFuenteAzar)
            } else {
                Ok(0)
            }
        }
    }
    let mut m = mundo(9);
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        mut paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(8),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    loop {
        if matches!(&paso,PasoAccion::Escena(r) if matches!(&r.contenido,ContenidoEscena::PasoMontanoso(p) if p.momento==MomentoPaso::Panorama))
        {
            break;
        }
        paso = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    }
    let PasoAccion::Escena(r) = paso else {
        panic!()
    };
    let antes = ejecucion.clone();
    assert!(avanzar(&mut m, &mut ejecucion, r.id, &ctx, &mut FalloEnChequeo).is_err());
    assert_eq!(ejecucion.posicion, antes.posicion);
    assert_eq!(ejecucion.total, antes.total);
    assert_eq!(ejecucion.reservas, antes.reservas);
    assert_eq!(m.ubicacion_grupo(), id(0));
}

#[test]
fn campamento_central_no_depende_del_sentido_ni_de_la_mitad_del_tiempo() {
    let mut m = super::super::pruebas::escenario(
        &[
            &["paso", "montanoso"],
            &["paso", "montanoso"],
            &["paso", "montanoso", "vasto"],
        ],
        &[&["viajero"], &[]],
    );
    m.aplicar_lote(LoteOperaciones::uno(OperacionMundo::Trasladar {
        actor: actor(),
        destino: id(2),
    }))
    .unwrap();
    let rumbos = CatalogoRumbos::default();
    let ctx = ContextoAccion {
        inicio: InstanteAccion {
            fecha: m.fecha(),
            fraccion: crate::FraccionDia::CERO,
        },
        rumbos: &rumbos,
    };
    let InicioAccion {
        mut ejecucion,
        mut paso,
    } = iniciar(
        &mut m,
        SolicitudAccion::Viajar {
            actor: actor(),
            destino: id(0),
        },
        &ctx,
        &mut Dado(20),
    )
    .unwrap();
    let mut campamento = None;
    let mut final_paso = None;
    for _ in 0..100 {
        let r = match &paso {
            PasoAccion::Escena(r) => r,
            PasoAccion::Decision(d) => &d.escena,
            PasoAccion::Finalizada(_) => break,
        };
        if let ContenidoEscena::PasoMontanoso(p) = &r.contenido {
            if p.momento == MomentoPaso::Campamento {
                assert!(campamento.is_none());
                campamento = Some(p.clone());
            }
            if p.momento == MomentoPaso::OtraVertiente {
                final_paso = Some(p.clone());
            }
        }
        paso = siguiente(&mut m, &mut ejecucion, paso, &ctx, 20);
    }
    let campamento = campamento.unwrap();
    let fin = final_paso.unwrap();
    assert_eq!(campamento.lugar_confirmado, id(1));
    assert_eq!(
        partes(campamento.traslado_acumulado),
        HorasViaje::horas(252)
    );
    assert_eq!(partes(fin.traslado_acumulado), HorasViaje::horas(420));
    assert_eq!(
        campamento.otras_actividades,
        HorasViaje::CERO.tiempo().unwrap()
    );
    assert_eq!(m.ubicacion_grupo(), id(0));
}
