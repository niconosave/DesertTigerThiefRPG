use super::*;
use desert_tiger_core::CambioConfirmado;

impl Narrador<'_> {
    // La brevedad es una decisión de redacción, no un recorte de acontecimientos:
    // el motor sigue entregando todas las escenas y el detalle conserva sus datos.
    pub(super) fn resumen(&self, r: &RegistroEscena) -> String {
        match &r.contenido {
            ContenidoEscena::Plan { propuestas } => self.deliberacion(r,propuestas),
            ContenidoEscena::Preparacion { chequeo, .. } => format!(
                "Pasás un día preparando la partida en {}, mientras {} organiza la comida, el agua y los animales. {}{}.",
                self.lugar(r.lugar),
                self.actor(chequeo.responsable),
                if chequeo.exito {
                    "Ya reuniste provisiones para emprender el camino"
                } else {
                    "Reunís menos provisiones de las calculadas y no sabés si alcanzarán"
                },
                aviso_reservas(r),
            ),
            ContenidoEscena::CargaInicial { base, alternativa } => format!(
                "En {}, el peso de las provisiones te obliga a elegir cómo partir. Podés llevar {} animales de carga para evitar la demora, o {} y cargar más sobre el grupo. Llevar menos animales añade {} al recorrido.",
                self.lugar(r.lugar),
                base.animales,
                alternativa.animales,
                formatear(alternativa.demora),
            ),
            ContenidoEscena::Incompleto { inicial: true, .. } =>
                "Conseguís menos provisiones de las previstas y no sabés si alcanzarán. Podés arriesgarte a partir con lo reunido o cancelar y devolver los suministros.".into(),
            ContenidoEscena::Incompleto { falta_comida, falta_agua, .. } => format!(
                "En {} aún tenés una ración diaria para todos, pero te falta {} para completar el camino. Debés decidir si seguís con lo reunido o detenés el viaje.",
                self.lugar(r.lugar),
                recursos(*falta_comida, *falta_agua),
            ),
            ContenidoEscena::PasoMontanoso(p)=>self.relato_paso(r,p),
            ContenidoEscena::Desplazamiento(d) => self.resumen_desplazamiento(r, d),
            ContenidoEscena::Forrajeo { obtenido, busca_comida, busca_agua, .. } => {
                let resultado = if obtenido.comidas.iter().any(|&n| n > 0)
                    || obtenido.botas.iter().any(|&n| n > 0)
                {
                    "Después de seis horas de búsqueda, conseguís nuevas provisiones"
                } else {
                    "Después de seis horas de búsqueda, volvés con las manos vacías"
                };
                let cierre = if r.cierre == Some(CierreViaje::SinRecursos) {
                    format!("Lo reunido no alcanza para una ración diaria de todos y tu viaje termina aquí{}", aviso_reservas(r))
                } else {
                    format!("Podés retomar el camino{}", aviso_reservas(r))
                };
                format!(
                    "En {} detenés la marcha para buscar {}. {resultado}. {cierre}.",
                    self.lugar(r.lugar), recursos(*busca_comida, *busca_agua),
                )
            }
            ContenidoEscena::Sobrecarga { .. } => format!(
                "En {}, la carga te frena y las reservas no alcanzan para sostener esa demora. Para seguir debés abandonar parte de las provisiones y aliviar el peso, o dar por terminado el viaje.",
                self.lugar(r.lugar),
            ),
            ContenidoEscena::Llegada { lugares } => format!(
                "{} {} se abre ante vos. Alcanzás tu destino después de {}{}.",
                if *lugares >= 5 { "Después de tanta distancia y esfuerzo," } else { "Por fin," },
                self.lugar(r.destino_final),
                formatear(r.total),
                aviso_reservas(r),
            ),
        }
    }

    fn resumen_desplazamiento(&self, r: &RegistroEscena, d: &Desplazamiento) -> String {
        let lider = self.actor(d.chequeos[0].responsable);
        let medio = match d.medio {
            Medio::Pie => "a pie",
            Medio::Caballo => "al ritmo de los cascos",
            Medio::Camello => "con el balance de los camellos",
            Medio::Barco => "a bordo",
        };
        let inicio = format!(
            "Partís de {} hacia {} {medio}, bajo la guía de {lider}.",
            self.lugar(d.origen),
            self.lugar(d.destino),
        );
        let suceso = if let Some(i) = &d.incidente {
            self.resumen_incidente(r, d.medio, i)
        } else if d.mitades.iter().any(|m| m.terreno_lento) {
            "El terreno te obliga a avanzar despacio y a elegir con cuidado cada apoyo.".into()
        } else if d.mitades.iter().any(|m| m.estado_horas > 0) {
            "Debés ajustar la marcha al paso de quienes están lastimados o cansados.".into()
        } else if d.mitades.iter().any(|m| {
            !m.demora_carga.duracion.es_cero()
                || m.demora_carga.fraccion != desert_tiger_core::FraccionDia::CERO
        }) {
            "El peso de las provisiones te obliga a avanzar más despacio.".into()
        } else if d.mitades.iter().any(|m| m.animal_limitante) {
            "Los animales de carga marcan el paso y tenés que acompañarlos.".into()
        } else if let Some((_, _, ajuste)) = d.viento {
            match ajuste.cmp(&0) {
                std::cmp::Ordering::Less => {
                    "El viento hincha las velas a tu favor y ganás camino sobre las olas."
                }
                std::cmp::Ordering::Greater => {
                    "El viento contrario te obliga a maniobrar y alarga la travesía."
                }
                std::cmp::Ordering::Equal => {
                    "Sostenés el rumbo mientras el barco se abre paso entre las olas."
                }
            }
            .into()
        } else {
            "Encontrás el paso entre las referencias del paisaje y seguís adelante con el grupo."
                .into()
        };
        // Antes de responder a la separación, el movimiento aún no se confirmó.
        let separacion_pendiente = d.incidente.as_ref().is_some_and(|i| {
            i.tipo == Incidente::Separacion
                && i.afectado
                    .is_some_and(|actor| r.composicion_resultante.contains(&actor))
        });
        let cierre = if separacion_pendiente {
            "Podés detenerte allí para buscarlo o seguir sin él.".into()
        } else {
            let desenlace = match r.cierre {
                Some(CierreViaje::Naufragio) => "tu viaje termina",
                Some(CierreViaje::InterrumpidoPorSeparacion) => {
                    "detenés el viaje para buscar a tu compañero"
                }
                _ => "llegás",
            };
            let preposicion = if r.cierre.is_some() { "en" } else { "a" };
            format!(
                "Tras {}, {desenlace} {preposicion} {}{}.",
                formatear(r.duracion),
                self.lugar(r.lugar),
                aviso_reservas(r),
            )
        };
        format!("{inicio} {suceso} {cierre}")
    }

    fn resumen_incidente(&self, r: &RegistroEscena, medio: Medio, i: &IncidenteResuelto) -> String {
        let actor = i
            .afectado
            .map(|id| self.actor(id))
            .unwrap_or_else(|| "un compañero".into());
        let lugar = self.lugar(i.lugar);
        match i.tipo {
            Incidente::Agotamiento => format!(
                "En {lugar}, {actor} se queda sin fuerzas y debés esperarlo antes de seguir."
            ),
            Incidente::Esguince => {
                let nuevo = r.cambios.iter().any(|c| {
                    matches!(c,
                        CambioConfirmado::CaracteristicaActorAnadida { caracteristica, .. }
                        if caracteristica.como_str() == "esguince"
                    )
                });
                if nuevo {
                    format!(
                        "En {lugar}, un mal apoyo le causa un esguince a {actor} y el grupo debe acompañar su paso."
                    )
                } else {
                    format!(
                        "En {lugar}, {actor} sufre un mal apoyo; su estado no empeora y podés seguir."
                    )
                }
            }
            Incidente::RumboPerdido => format!(
                "En {lugar} perdés las referencias y los rodeos ponen a prueba las fuerzas del grupo."
            ),
            Incidente::Separacion => format!("En {lugar} perdés el rastro de {actor}."),
            Incidente::Enfermedad => format!(
                "En {lugar}, una enfermedad te obliga a detenerte un día antes de poder continuar."
            ),
            Incidente::Obstinada => format!(
                "En {lugar}, la montura de {actor} se aparta del rumbo y tenés que perder tiempo corrigiendo el camino."
            ),
            Incidente::Detenida => format!(
                "En {lugar}, una montura se niega a seguir y debés esperar hasta que retome la marcha."
            ),
            Incidente::Asustada => format!(
                "En {lugar}, una montura se espanta y derriba a {actor}; reagruparte demora la marcha."
            ),
            Incidente::CrisisMontura => {
                let causa = if medio == Medio::Camello {
                    "una picadura hiere a un camello"
                } else {
                    "una fiera hiere a un caballo"
                };
                let resultado = if i.recuperacion.is_some_and(|d| d >= 8) {
                    "la montura se recupera"
                } else {
                    "la montura muere"
                };
                format!("En {lugar}, {causa}; esperás dos días y {resultado}.")
            }
            Incidente::MuerteMontura => format!(
                "En {lugar} perdés una montura y debés continuar con los animales que quedan."
            ),
            Incidente::Olas => format!(
                "En {lugar}, las olas apartan la proa y debés esforzarte por recuperar el rumbo."
            ),
            Incidente::Lluvia => format!(
                "En {lugar}, la lluvia envuelve las velas y te obliga a navegar con más cautela."
            ),
            Incidente::SinViento => {
                format!("En {lugar}, las velas caen flojas y pasás dos días esperando viento.")
            }
            Incidente::Naufragio => {
                let separados = if r.composicion_resultante.len() < r.participantes.len() {
                    " y perdés el contacto con tus compañeros"
                } else {
                    ""
                };
                format!("Una tormenta te hace naufragar en {lugar}{separados}.")
            }
            Incidente::PorLaBorda => {
                let costa = r.cambios.iter().find_map(|c| match c {
                    CambioConfirmado::CompaneroSeparado { actor, lugar }
                        if Some(*actor) == i.afectado =>
                    {
                        Some(self.lugar(*lugar))
                    }
                    _ => None,
                });
                format!(
                    "En {lugar}, una sacudida arroja a {actor} por la borda y queda fuera del grupo{}.",
                    costa.map(|c| format!(", en {c}")).unwrap_or_default()
                )
            }
        }
    }
}

fn recursos(comida: bool, agua: bool) -> &'static str {
    match (comida, agua) {
        (true, true) => "comida y agua",
        (true, false) => "comida",
        (false, true) => "agua",
        (false, false) => "provisiones",
    }
}

// Son consecuencias vividas o reservas agotadas, nunca una previsión secreta.
fn aviso_reservas(r: &RegistroEscena) -> String {
    let mut avisos = Vec::new();
    if r.consumo.sin_alimento.is_some() || r.consumo.sin_agua.is_some() {
        avisos.push(format!(
            "el grupo sufrió {}",
            match (
                r.consumo.sin_alimento.is_some(),
                r.consumo.sin_agua.is_some()
            ) {
                (true, true) => "hambre y sed",
                (true, false) => "hambre",
                _ => "sed",
            }
        ));
    }
    if let Some(reservas) = &r.reservas {
        let comida = reservas.comidas.iter().all(|&n| n == 0);
        let agua = reservas.botas.iter().all(|&n| n == 0);
        if comida || agua {
            avisos.push(format!("ya no te queda {}", recursos(comida, agua)));
        }
    }
    if avisos.is_empty() {
        String::new()
    } else {
        format!("; {}", avisos.join(" y "))
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_core::{Duracion, FraccionDia};

    fn tiempo(dias: u32) -> TiempoAccion {
        TiempoAccion {
            duracion: Duracion::desde_dias(dias),
            fraccion: FraccionDia::CERO,
        }
    }

    fn mitades(origen: IdLugar, destino: IdLugar, medio: Medio) -> Vec<MitadRecorrida> {
        [origen, destino]
            .map(|lugar| MitadRecorrida {
                lugar,
                medio,
                horas_base: 72,
                pequena: false,
                vasta: false,
                terreno_lento: false,
                animal_limitante: false,
                estado_horas: 0,
                duracion: tiempo(3),
                demora_carga: tiempo(0),
            })
            .to_vec()
    }

    fn comprobar_jerarquia(texto: &TextoEscena) {
        let oraciones = texto
            .texto
            .split('.')
            .filter(|s| !s.trim().is_empty())
            .count();
        assert!(
            (1..=3).contains(&oraciones),
            "{}: {}",
            texto.titulo,
            texto.texto
        );
        assert!(!texto.texto.contains('\n'));
        for termino in [
            "dado ",
            "dificultad ",
            "límite conjunto",
            "períodos de consumo",
            "Conoce con",
        ] {
            assert!(!texto.texto.contains(termino), "{}", texto.texto);
        }
        assert!(!texto.detalle.is_empty());
        assert_ne!(texto.texto, texto.detalle.join("\n"));
    }

    #[test]
    fn todas_las_tarjetas_separan_relato_breve_del_desglose() {
        let (contenido, mundo, mut registro) = super::super::pruebas::escenario();
        let narrador = Narrador(&contenido);
        let actor = mundo.actor_jugador();
        let chequeo = resolver_chequeo(
            &mundo,
            actor,
            AccionParticular::Cabalgar,
            15,
            &mut super::super::pruebas::Cero,
        )
        .unwrap();
        let configuracion = ConfiguracionVista {
            reservas: ReservasViaje {
                comidas: [3, 5, 7, 9],
                botas: vec![3000],
            },
            animales: 2,
            exceso_gramos: 100,
            peso_actores: 20_100,
            limite_actores: 20_000,
            tiempo: tiempo(12),
            demora: tiempo(1),
            suplemento: ReservasViaje::default(),
        };
        let escenas = [
            (
                ClaveTarjeta::SinPlan,
                ContenidoEscena::Plan { propuestas: vec![] },
            ),
            (
                ClaveTarjeta::PropuestaRuta,
                ContenidoEscena::Plan { propuestas: vec![] },
            ),
            (
                ClaveTarjeta::Preparacion,
                ContenidoEscena::Preparacion {
                    chequeo: chequeo.clone(),
                    configuracion: configuracion.clone(),
                },
            ),
            (
                ClaveTarjeta::CargaInicial,
                ContenidoEscena::CargaInicial {
                    base: configuracion.clone(),
                    alternativa: ConfiguracionVista {
                        animales: 1,
                        ..configuracion.clone()
                    },
                },
            ),
            (
                ClaveTarjeta::Incompleto,
                ContenidoEscena::Incompleto {
                    inicial: true,
                    falta_comida: true,
                    falta_agua: true,
                },
            ),
            (
                ClaveTarjeta::Incompleto,
                ContenidoEscena::Incompleto {
                    inicial: false,
                    falta_comida: false,
                    falta_agua: true,
                },
            ),
            (
                ClaveTarjeta::Forrajeo,
                ContenidoEscena::Forrajeo {
                    chequeos: vec![chequeo.clone()],
                    obtenido: configuracion.reservas.clone(),
                    busca_comida: true,
                    busca_agua: true,
                },
            ),
            (
                ClaveTarjeta::Sobrecarga,
                ContenidoEscena::Sobrecarga {
                    configuracion: configuracion.clone(),
                    lote: Default::default(),
                },
            ),
            (
                ClaveTarjeta::Llegada,
                ContenidoEscena::Llegada { lugares: 5 },
            ),
        ];
        for (tarjeta, escena) in escenas {
            registro.tarjeta = tarjeta;
            registro.contenido = escena;
            let texto = narrador.redactar(&registro);
            if matches!(registro.contenido, ContenidoEscena::Plan { .. }) {
                assert!(!texto.texto.is_empty() && !texto.detalle.is_empty());
            } else {
                comprobar_jerarquia(&texto);
            }
        }
        for (medio, tarjeta) in [
            (Medio::Pie, ClaveTarjeta::Andar),
            (Medio::Caballo, ClaveTarjeta::Caballo),
            (Medio::Camello, ClaveTarjeta::Camello),
            (Medio::Barco, ClaveTarjeta::Navegar),
        ] {
            registro.tarjeta = tarjeta;
            registro.lugar = registro.destino_final;
            registro.contenido = ContenidoEscena::Desplazamiento(Desplazamiento {
                medio,
                origen: mundo.ubicacion_actor(actor).unwrap(),
                destino: registro.destino_final,
                chequeos: [chequeo.clone(), chequeo.clone()],
                mitades: mitades(
                    mundo.ubicacion_actor(actor).unwrap(),
                    registro.destino_final,
                    medio,
                ),
                incidente: None,
                viento: None,
            });
            let texto = narrador.redactar(&registro);
            comprobar_jerarquia(&texto);
            assert!(texto.texto.contains(&narrador.lugar(registro.lugar)));
            // Fallar una tirada no implica que el motor haya impedido la llegada.
            assert!(texto.texto.contains("llegás a"));
            assert!(texto.detalle.join(" ").contains("dado "));
        }
        registro.tarjeta = ClaveTarjeta::Forrajeo;
        registro.contenido = ContenidoEscena::Forrajeo {
            chequeos: vec![chequeo],
            obtenido: ReservasViaje::default(),
            busca_comida: true,
            busca_agua: false,
        };
        registro.cierre = Some(CierreViaje::SinRecursos);
        let texto = narrador.redactar(&registro);
        comprobar_jerarquia(&texto);
        assert!(texto.texto.contains("manos vacías"));
        assert!(texto.texto.contains("viaje termina"));
    }

    #[test]
    fn incidentes_y_consecuencias_no_quedan_ocultos_en_detalles() {
        let (contenido, mundo, mut r) = super::super::pruebas::escenario();
        let n = Narrador(&contenido);
        let afectado = r.participantes[1];
        let chequeo = resolver_chequeo(
            &mundo,
            mundo.actor_jugador(),
            AccionParticular::Andar,
            5,
            &mut super::super::pruebas::Cero,
        )
        .unwrap();
        let lugar = r.lugar;
        r.tarjeta = ClaveTarjeta::Andar;
        r.lugar = r.destino_final;
        r.duracion = tiempo(7);
        for incidente in [
            Incidente::Agotamiento,
            Incidente::Esguince,
            Incidente::RumboPerdido,
            Incidente::Separacion,
            Incidente::Enfermedad,
            Incidente::Obstinada,
            Incidente::Detenida,
            Incidente::Asustada,
            Incidente::CrisisMontura,
            Incidente::MuerteMontura,
            Incidente::Olas,
            Incidente::Lluvia,
            Incidente::SinViento,
            Incidente::Naufragio,
            Incidente::PorLaBorda,
        ] {
            r.contenido = ContenidoEscena::Desplazamiento(Desplazamiento {
                medio: Medio::Pie,
                origen: lugar,
                destino: r.destino_final,
                chequeos: [chequeo.clone(), chequeo.clone()],
                mitades: mitades(lugar, r.destino_final, Medio::Pie),
                viento: None,
                incidente: Some(IncidenteResuelto {
                    tipo: incidente,
                    dado: 20,
                    lugar,
                    afectado: Some(afectado),
                    recuperacion: Some(9),
                    monturas_restantes: Some(1),
                    demora: tiempo(1),
                }),
            });
            let texto = n.redactar(&r);
            comprobar_jerarquia(&texto);
            assert!(texto.detalle.join(" ").contains("Incidente: 20"));
            if incidente == Incidente::Separacion {
                assert!(texto.texto.contains(&n.actor(afectado)));
                assert!(texto.texto.contains("buscarlo o seguir sin él"));
                assert!(!texto.texto.contains("llegás a"));
                r.composicion_resultante.retain(|&id| id != afectado);
                r.cierre = Some(CierreViaje::InterrumpidoPorSeparacion);
                r.lugar = lugar;
                let confirmado = n.redactar(&r);
                comprobar_jerarquia(&confirmado);
                assert!(confirmado.texto.contains("detenés el viaje"));
                r.composicion_resultante = r.participantes.clone();
                r.cierre = None;
                r.lugar = r.destino_final;
            }
        }
        r.cierre = Some(CierreViaje::Naufragio);
        r.composicion_resultante.truncate(1);
        r.reservas = Some(ReservasViaje::default());
        r.consumo.sin_agua = Some(tiempo(1));
        if let ContenidoEscena::Desplazamiento(d) = &mut r.contenido {
            d.incidente.as_mut().unwrap().tipo = Incidente::Naufragio;
        }
        let texto = n.redactar(&r);
        comprobar_jerarquia(&texto);
        for hecho in [
            "naufragar",
            "perdés el contacto",
            "viaje termina",
            "sed",
            "ya no te queda comida y agua",
        ] {
            assert!(texto.texto.contains(hecho), "{hecho}: {}", texto.texto);
        }
    }
}
