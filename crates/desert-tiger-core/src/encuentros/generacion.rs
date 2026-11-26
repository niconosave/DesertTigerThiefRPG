//! Recetas acotadas y validación de la disposición inicial.
use super::*;
impl Encuentro {
    pub(super) fn construir(
        mundo: &Mundo,
        grupo: &[IdActor],
        objetivo: Option<IdActor>,
        defensores: &[IdActor],
        azar: &mut dyn FuenteAzar,
        escenario: Escenario,
    ) -> Result<Self, ErrorEncuentro> {
        let mut ultimo = ErrorEncuentro::from("sin receta válida");
        for _ in 0..8 {
            match Self::construir_receta(mundo, grupo, objetivo, defensores, azar, escenario) {
                Ok(e) => return Ok(e),
                Err(error) => ultimo = error,
            }
        }
        Err(ErrorEncuentro::GeneracionAgotada {
            intentos: 8,
            ultimo: Box::new(ultimo),
        })
    }
    fn construir_receta(
        mundo: &Mundo,
        grupo: &[IdActor],
        objetivo: Option<IdActor>,
        defensores: &[IdActor],
        azar: &mut dyn FuenteAzar,
        escenario: Escenario,
    ) -> Result<Self, ErrorEncuentro> {
        if grupo.is_empty()
            || grupo.len() + defensores.len() + usize::from(objetivo.is_some()) > 32
            || objetivo.is_some_and(|o| grupo.contains(&o))
        {
            return Err("participantes inválidos".into());
        }
        let ancho = 31;
        let alto = 17;
        let (norte, sur) = if escenario == Escenario::Campamento {
            (3 + sortear(azar, 2)? as u16, 11 + sortear(azar, 2)? as u16)
        } else {
            (3, 11)
        };
        let mut celdas = vec![Celda::Suelo; usize::from(ancho * alto)];
        for y in 0..alto {
            for x in 0..ancho {
                let cerrado = matches!(
                    escenario,
                    Escenario::Recinto | Escenario::Palacio | Escenario::DosAlturas
                );
                if (cerrado && (x == 0 || y == 0 || x == ancho - 1 || y == alto - 1))
                    || (escenario == Escenario::Recinto && x == 15 && y != 8)
                {
                    celdas[usize::from(y * ancho + x)] = Celda::Muro;
                }
            }
        }
        if escenario == Escenario::Recinto {
            for y in 3..6 {
                for x in 7..10 {
                    celdas[usize::from(y * ancho + x)] = Celda::Vacio;
                }
            }
            for _ in 0..12 {
                let x = 3 + sortear(azar, 25)? as u16;
                let y = 2 + sortear(azar, 13)? as u16;
                if (7..=10).contains(&y) || x == 15 {
                    continue;
                }
                celdas[usize::from(y * ancho + x)] = Celda::Muro;
            }
        } else if escenario == Escenario::Campamento {
            for y in 0..alto {
                if !(norte..=norte + 1).contains(&y) && !(sur..=sur + 1).contains(&y) {
                    celdas[usize::from(y * ancho + 20)] = Celda::Vacio;
                }
            }
            for (x0, y0) in [(11, 4), (13, 11), (25, 2), (26, 13)] {
                for y in y0..y0 + 2 {
                    for x in x0..x0 + 2 {
                        celdas[usize::from(y * ancho + x)] = Celda::Muro;
                    }
                }
            }
        } else {
            let mut rect = |x0: u16, y0: u16, x1: u16, y1: u16, t: Celda| {
                for y in y0..=y1 {
                    for x in x0..=x1 {
                        celdas[usize::from(y * ancho + x)] = t;
                    }
                }
            };
            match escenario {
                Escenario::Palacio => {
                    for x in [11, 21] {
                        rect(x, 1, x, 7, Celda::Muro);
                        rect(x, 9, x, 15, Celda::Muro);
                    }
                    for x in [16, 25] {
                        for y in [4, 12] {
                            rect(x, y, x + 1, y + 1, Celda::Muro);
                        }
                    }
                }
                Escenario::Ciudad => {
                    for (x0, y0, x1, y1) in [
                        (7, 2, 11, 5),
                        (7, 11, 11, 14),
                        (17, 2, 21, 5),
                        (18, 11, 22, 14),
                        (25, 2, 28, 5),
                    ] {
                        rect(x0, y0, x1, y1, Celda::Muro);
                    }
                }
                Escenario::Bosque => {
                    let desplazamiento = sortear(azar, 2)? as u16;
                    for x in [6, 9, 12, 15, 18, 21, 24] {
                        for y in [3, 6, 11, 14] {
                            rect(x + desplazamiento, y, x + desplazamiento, y, Celda::Muro);
                        }
                    }
                }
                Escenario::Montana => {
                    rect(10, 1, 13, 5, Celda::Muro);
                    rect(10, 11, 13, 15, Celda::Muro);
                    rect(17, 2, 20, 6, Celda::Vacio);
                    rect(17, 10, 20, 14, Celda::Vacio);
                }
                Escenario::Playa => {
                    rect(5, 13, 29, 15, Celda::Vacio);
                    rect(7, 11, 10, 12, Celda::Vacio);
                    rect(10, 3, 11, 4, Celda::Muro);
                    rect(20, 8, 21, 9, Celda::Muro);
                }
                Escenario::DosAlturas => {}
                _ => unreachable!("perfiles anteriores resueltos arriba"),
            }
        }
        let mut fichas = Vec::new();
        let mut vistos = BTreeSet::new();
        for (i, actor) in grupo
            .iter()
            .chain(objetivo.iter())
            .chain(defensores)
            .copied()
            .enumerate()
        {
            if !vistos.insert(actor) {
                return Err("Actor repetido en el encuentro".into());
            }
            let a = mundo.actor(actor).ok_or("Actor inexistente")?;
            if mundo.ubicacion_actor(actor) != Some(mundo.ubicacion_grupo()) {
                return Err("participante fuera del Lugar".into());
            }
            let tiene = |c: &str| a.caracteristicas().any(|v| v.como_str() == c);
            if tiene("muerto") {
                return Err("participante muerto".into());
            }
            let aliado = i < grupo.len();
            let posicion = if aliado {
                Posicion {
                    z: 1,
                    x: 2 + (i / 13) as u16,
                    y: 2 + (i % 13) as u16,
                }
            } else if i == grupo.len() && objetivo.is_some() {
                Posicion { z: 1, x: 27, y: 8 }
            } else if escenario == Escenario::Campamento {
                let j = i - grupo.len() - usize::from(objetivo.is_some());
                Posicion {
                    z: 1,
                    x: 23 + (j % 3) as u16,
                    y: 4 + 4 * (j / 3) as u16,
                }
            } else {
                Posicion {
                    z: 1,
                    x: 17 + (i - grupo.len() - usize::from(objetivo.is_some())) as u16,
                    y: 8,
                }
            };
            if posicion.x >= ancho - 1 {
                return Err("demasiados defensores".into());
            }
            celdas[usize::from(posicion.y * ancho + posicion.x)] = Celda::Suelo;
            fichas.push(Ficha {
                id: IdFicha(i as u16),
                cautivo: false,
                actor,
                posicion,
                aliado,
                golpes: if tiene("incapacitado") {
                    2
                } else if tiene("herido") {
                    1
                } else {
                    0
                },
                ataque: if tiene("fuerte") { 2 } else { 0 },
                defensa: if tiene("agil") { 2 } else { 0 },
                salio: false,
                transportado_por: None,
            });
        }
        let mut prioridad = (0..fichas.len()).collect::<Vec<_>>();
        for i in (1..prioridad.len()).rev() {
            let j = sortear(azar, i + 1)?;
            prioridad.swap(i, j);
        }
        // La receta se expresa primero como planta y se materializa en un volumen:
        // suelo en z=0, aire transitable encima y obstáculos de altura dos.
        let planta = celdas;
        let altura = 8;
        let mut celdas = vec![Celda::Aire; usize::from(ancho * alto * altura)];
        for y in 0..alto {
            for x in 0..ancho {
                let base = planta[usize::from(y * ancho + x)];
                match base {
                    Celda::Suelo => celdas[usize::from(y * ancho + x)] = Celda::Suelo,
                    Celda::Muro | Celda::Solido(_) => {
                        for z in 0..=2 {
                            celdas[usize::from(((z * alto + y) * ancho) + x)] = Celda::Muro;
                        }
                    }
                    Celda::Vacio | Celda::Aire => {}
                }
            }
        }
        let mut elementos = Vec::new();
        let mut contexto = ContextoLocal::default();
        let umbral = if escenario == Escenario::Palacio {
            11
        } else {
            15
        };
        contexto.espacios.push(EspacioLocal {
            id: IdEspacio(0),
            nombre: "entrada".into(),
            min: Posicion { x: 0, y: 0, z: 0 },
            max: Posicion {
                x: umbral,
                y: alto - 1,
                z: altura - 1,
            },
        });
        contexto.espacios.push(EspacioLocal {
            id: IdEspacio(1),
            nombre: "objetivo".into(),
            min: Posicion {
                x: umbral + 1,
                y: 0,
                z: 0,
            },
            max: Posicion {
                x: ancho - 1,
                y: alto - 1,
                z: altura - 1,
            },
        });
        if escenario == Escenario::Palacio {
            contexto.espacios[1].nombre = "salón intermedio".into();
            contexto.espacios[1].max.x = 21;
            contexto.espacios.push(EspacioLocal {
                id: IdEspacio(2),
                nombre: "salón del objetivo".into(),
                min: Posicion { x: 22, y: 0, z: 0 },
                max: Posicion {
                    x: ancho - 1,
                    y: alto - 1,
                    z: altura - 1,
                },
            });
            contexto.conexiones.push(ConexionLocal {
                id: IdConexion(2),
                desde: IdEspacio(1),
                hasta: IdEspacio(2),
                recorrido: vec![Posicion { x: 21, y: 8, z: 1 }],
                control: None,
                salida: false,
            });
        }
        let control = if escenario == Escenario::Palacio {
            let id = IdElementoLocal(0);
            elementos.push(ElementoLocal {
                id,
                clase: ClaseElemento::PuertaManual {
                    conexion: IdConexion(0),
                    estado: EstadoPuerta::Cerrada,
                },
                posicion: Posicion { x: 11, y: 8, z: 1 },
                alto: 2,
                definido: true,
            });
            Some(id)
        } else {
            None
        };
        contexto.conexiones.push(ConexionLocal {
            id: IdConexion(0),
            desde: IdEspacio(0),
            hasta: IdEspacio(1),
            recorrido: vec![Posicion {
                x: if escenario == Escenario::Palacio {
                    11
                } else {
                    15
                },
                y: 8,
                z: 1,
            }],
            control,
            salida: false,
        });
        contexto.conexiones.push(ConexionLocal {
            id: IdConexion(1),
            desde: IdEspacio(0),
            hasta: IdEspacio(0),
            recorrido: vec![Posicion { x: 2, y: 8, z: 1 }],
            control: None,
            salida: true,
        });
        if escenario == Escenario::DosAlturas {
            // Plataforma en z=3 y escalera cardinal explícita hasta pies z=4.
            for y in 5..=11 {
                for x in 18..=27 {
                    celdas[usize::from(((3 * alto + y) * ancho) + x)] = Celda::Suelo;
                }
            }
            // Cuatro pilares sostienen la plataforma sin cerrar la planta baja.
            for (x, y) in [(18, 5), (27, 5), (18, 11), (27, 11)] {
                for z in 1..=2 {
                    celdas[usize::from((z * alto + y) * ancho + x)] = Celda::Muro;
                }
            }
            // Cerramiento superior con un acceso ancho frente a la escalera.
            for z in 4..=5 {
                for y in 5..=11 {
                    for x in 18..=27 {
                        if (y == 5 || y == 11 || x == 27 || x == 18)
                            && !(x == 18 && (7..=9).contains(&y))
                        {
                            celdas[usize::from((z * alto + y) * ancho + x)] = Celda::Muro;
                        }
                    }
                }
            }
            for (x, y, z) in [(15, 8, 1), (16, 8, 2), (17, 8, 3), (18, 8, 4)] {
                let apoyo = z - 1;
                celdas[usize::from(((apoyo * alto + y) * ancho) + x)] = Celda::Suelo;
            }
            contexto.conexiones.push(ConexionLocal {
                id: IdConexion(2),
                desde: IdEspacio(0),
                hasta: IdEspacio(1),
                recorrido: vec![
                    Posicion { x: 15, y: 8, z: 1 },
                    Posicion { x: 16, y: 8, z: 2 },
                    Posicion { x: 17, y: 8, z: 3 },
                    Posicion { x: 18, y: 8, z: 4 },
                ],
                control: None,
                salida: false,
            });
            for (j, ficha) in fichas.iter_mut().filter(|f| !f.aliado).enumerate() {
                ficha.posicion = Posicion {
                    x: 24 + j as u16,
                    y: 7 + j as u16,
                    z: 4,
                };
            }
        }
        let proposito = if objetivo.is_some() {
            PropositoPlan::Atacar
        } else {
            PropositoPlan::Recuperar
        };
        let mut planes = vec![PlanLocal {
            id: IdPlan(0),
            proposito,
            prioridad: 10,
            participantes: fichas.iter().filter(|f| f.aliado).map(|f| f.id).collect(),
            estado: EstadoPlan::Activo,
            bloqueo: None,
        }];
        planes.push(PlanLocal {
            id: IdPlan(2),
            proposito: PropositoPlan::Atacar,
            prioridad: 10,
            participantes: fichas.iter().filter(|f| !f.aliado).map(|f| f.id).collect(),
            estado: EstadoPlan::Activo,
            bloqueo: None,
        });
        let mut encuentro = Self {
            lugar: mundo.ubicacion_grupo(),
            botin: None,
            rescate: false,
            escenario,
            referencias: if escenario == Escenario::Recinto {
                vec![
                    ("entrada", 2, 8),
                    ("acceso", 15, 8),
                    ("brecha_norte", 15, 7),
                    ("brecha_sur", 15, 9),
                ]
            } else if escenario == Escenario::Campamento {
                vec![
                    ("entrada", 2, 8),
                    ("paso_norte", 20, norte),
                    ("paso_sur", 20, sur),
                    ("puesto_jefe", 27, 8),
                ]
            } else {
                vec![("entrada", 2, 8), ("objetivo", 27, 8)]
            }
            .into_iter()
            .map(|(nombre, x, y)| (nombre.to_owned(), Posicion { z: 1, x, y }))
            .collect(),
            ancho,
            alto,
            altura,
            celdas,
            elementos,
            contexto,
            planes,
            interacciones_puerta: BTreeMap::new(),
            puertas_inicio: BTreeMap::new(),
            direcciones: BTreeMap::new(),
            rutas_decision: BTreeMap::new(),
            proximo_comportamiento: 0,
            objetivo: objetivo.map(|_| fichas[grupo.len()].id),
            fichas,
            entrada: Posicion { z: 1, x: 2, y: 8 },
            pulso: 0,
            retirando: false,
            descubierta: !matches!(escenario, Escenario::Recinto | Escenario::Palacio),
            espacios_descubiertos: if matches!(escenario, Escenario::Recinto | Escenario::Palacio) {
                BTreeSet::from([IdEspacio(0)])
            } else {
                BTreeSet::from([IdEspacio(0), IdEspacio(1)])
            },
            memoria_terreno: BTreeMap::new(),
            memoria_elementos: BTreeMap::new(),
            sucesos: vec![],
            desenlace: None,
            capturas: BTreeMap::new(),
            resultado: None,
            decision_evacuacion: Vec::new(),
            abandonados_confirmados: BTreeSet::new(),
            evacuados: BTreeMap::new(),
            prioridad,
            sin_progreso: 0,
            bloqueado: false,
        };
        encuentro
            .validar_geometria()
            .map_err(ErrorEncuentro::Geometria)?;
        let mut prueba = encuentro.clone();
        for f in &mut prueba.fichas {
            f.salio = true;
        }
        let rutas = prueba.mapa_rutas_fisicas(0);
        if encuentro
            .fichas
            .iter()
            .any(|f| !rutas.contains_key(&f.posicion))
        {
            return Err("receta sin recorrido entre participantes".into());
        }
        encuentro.recordar_terreno();
        Ok(encuentro)
    }
}
