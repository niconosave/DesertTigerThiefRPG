use desert_tiger_app::{
    EntradaFicha, ExtremoRelacion, IdActor, IdCampamento, Partida, VistaCampamento,
};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub(crate) struct AgendaMusica {
    pub inicial: (u32, u32),
    pub pausa: (u32, u32),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ProgresoMusica {
    pub evento: u32,
    pub transcurrido: Duration,
    pub finalizada: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct NotasMusicales {
    pub actor: IdActor,
    pub transcurrido: Duration,
}

pub(crate) fn candidatos(partida: &Partida, vista: &VistaCampamento) -> Vec<IdActor> {
    vista
        .integrantes
        .iter()
        .filter_map(|integrante| {
            let ficha = partida
                .consultar_ficha(vista.id, ExtremoRelacion::Actor(integrante.actor))
                .ok()?;
            es_musico(&ficha.entradas).then_some(integrante.actor)
        })
        .collect()
}

fn es_musico(entradas: &[EntradaFicha]) -> bool {
    let tiene = |clave: &str| {
        entradas
            .iter()
            .any(|e| matches!(e, EntradaFicha::Caracteristica { clave: c, .. } if c == clave))
    };
    tiene("musico") && !tiene("muerto")
}

pub(crate) struct MusicaCampamento {
    campamento: Option<IdCampamento>,
    candidatos: Vec<IdActor>,
    proxima: Duration,
    evento: Option<(u32, IdActor)>,
    notas: Option<NotasMusicales>,
    contador: u32,
    azar: u64,
}

impl Default for MusicaCampamento {
    fn default() -> Self {
        Self {
            campamento: None,
            candidatos: Vec::new(),
            proxima: Duration::ZERO,
            evento: None,
            notas: None,
            contador: 0,
            azar: 0x6d757369636f,
        }
    }
}

impl MusicaCampamento {
    pub(crate) fn campamento(&self) -> Option<IdCampamento> {
        self.campamento
    }
    fn sortear(&mut self, cantidad: u64) -> u64 {
        self.azar = self.azar.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.azar;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31)) % cantidad
    }
    fn intervalo(&mut self, rango: (u32, u32)) -> Duration {
        Duration::from_secs(u64::from(rango.0) + self.sortear(u64::from(rango.1 - rango.0) + 1))
    }
    pub(crate) fn abrir(
        &mut self,
        id: IdCampamento,
        candidatos: Vec<IdActor>,
        ahora: Duration,
        ajustes: AgendaMusica,
    ) {
        self.cancelar();
        self.campamento = Some(id);
        self.candidatos = candidatos;
        self.proxima = ahora.saturating_add(self.intervalo(ajustes.inicial));
    }
    pub(crate) fn cancelar(&mut self) {
        self.campamento = None;
        self.candidatos.clear();
        self.evento = None;
        self.notas = None;
    }
    pub(crate) fn actualizar(
        &mut self,
        ahora: Duration,
        progreso: Option<ProgresoMusica>,
        modal: bool,
        ajustes: AgendaMusica,
    ) {
        self.notas = None;
        if let Some((id, actor)) = self.evento {
            if let Some(progreso) = progreso.filter(|p| p.evento == id) {
                if progreso.finalizada {
                    self.evento = None;
                    self.proxima = ahora.saturating_add(self.intervalo(ajustes.pausa));
                } else {
                    self.notas = Some(NotasMusicales {
                        actor,
                        transcurrido: progreso.transcurrido,
                    });
                }
            }
        } else if self.campamento.is_some()
            && !self.candidatos.is_empty()
            && !modal
            && ahora >= self.proxima
        {
            let Some(id) = self.contador.checked_add(1) else {
                return;
            };
            self.contador = id;
            let indice = self.sortear(self.candidatos.len() as u64) as usize;
            if let Some(&actor) = self.candidatos.get(indice) {
                self.evento = Some((id, actor));
            }
        }
    }
    pub(crate) fn solicitud(&self) -> u32 {
        self.evento.map(|e| e.0).unwrap_or(0)
    }
    pub(crate) fn notas(&self) -> Option<NotasMusicales> {
        self.notas
    }
}

pub(crate) fn dibujar_notas(
    frame: &mut ratatui::Frame<'_>,
    area: ratatui::layout::Rect,
    recorte: ratatui::layout::Rect,
    mundo: &desert_tiger_app::VistaMundo,
    notas: NotasMusicales,
    estilo: ratatui::style::Style,
) {
    use unicode_width::UnicodeWidthChar;
    let Some((actor, x, y)) =
        crate::interfaz::disposicion_retratos(mundo, area).find(|(a, _, _)| a.id == notas.actor)
    else {
        return;
    };
    let origen_x =
        x.saturating_add(i64::try_from(actor.retrato.base().ancho() * 2 / 3).unwrap_or(i64::MAX));
    for (i, glifo) in ['♪', '♫', '♩'].into_iter().enumerate() {
        if glifo.width() != Some(1) {
            continue;
        }
        let Some(edad) = notas.transcurrido.as_millis().checked_sub(i as u128 * 650) else {
            continue;
        };
        let edad = edad % 2100;
        if edad >= 1750 {
            continue;
        }
        let py = y.saturating_add(3).saturating_sub((edad / 450) as i64);
        let px = origen_x.saturating_add((edad / 650) as i64);
        // Buscamos un hueco cercano; nunca borramos el dibujo de una cara.
        for desplazamiento in [0, 1, -1, 2, -2] {
            let px = px.saturating_add(desplazamiento);
            if px < i64::from(recorte.x)
                || px >= i64::from(recorte.right())
                || py < i64::from(recorte.y)
                || py >= i64::from(recorte.bottom())
            {
                continue;
            }
            if let (Ok(px), Ok(py)) = (u16::try_from(px), u16::try_from(py))
                && let Some(celda) = frame.buffer_mut().cell_mut((px, py))
                && celda.symbol() == " "
            {
                let fondo = celda.bg;
                celda.set_char(glifo).set_style(estilo.bg(fondo));
                break;
            }
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_app::{
        ConfiguracionPartida, IntencionFrontend, VistaSesion, cargar_contenido,
    };

    fn escenario() -> (Partida, VistaCampamento) {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut p = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(7) }).unwrap();
        let VistaSesion::Acampando(v) = p.procesar(IntencionFrontend::ComenzarCampamento).unwrap()
        else {
            panic!("campamento esperado")
        };
        (p, v)
    }

    #[test]
    fn solo_musicos_vivos_y_el_sirviente_oficial_es_elegible() {
        let (p, v) = escenario();
        let c = candidatos(&p, &v);
        assert_eq!(c.len(), 1);
        let mut ficha = p
            .consultar_ficha(v.id, ExtremoRelacion::Actor(c[0]))
            .unwrap();
        assert_eq!(ficha.nombre, "Sirviente");
        assert!(ficha.entradas.iter().any(|e| matches!(e, EntradaFicha::Caracteristica { clave, descripcion: Some(_), .. } if clave == "musico")));
        ficha.entradas.push(EntradaFicha::Caracteristica {
            clave: "muerto".into(),
            nombre: "Muerto".into(),
            descripcion: None,
        });
        assert!(!es_musico(&ficha.entradas));
        assert!(!es_musico(&[]));
    }

    #[test]
    fn agenda_confirma_audio_respeta_pausas_modal_y_reapertura() {
        let (p, v) = escenario();
        let ajustes = AgendaMusica {
            inicial: (1, 1),
            pausa: (2, 2),
        };
        let mut agenda = MusicaCampamento::default();
        agenda.abrir(v.id, candidatos(&p, &v), Duration::ZERO, ajustes);
        agenda.actualizar(Duration::ZERO, None, false, ajustes);
        assert_eq!(agenda.solicitud(), 0);
        agenda.actualizar(Duration::from_secs(1), None, true, ajustes);
        assert_eq!(agenda.solicitud(), 0);
        agenda.actualizar(Duration::from_secs(2), None, false, ajustes);
        let id = agenda.solicitud();
        assert!(id > 0 && agenda.notas().is_none());
        agenda.actualizar(
            Duration::from_secs(3),
            Some(ProgresoMusica {
                evento: id,
                transcurrido: Duration::from_secs(1),
                finalizada: false,
            }),
            true,
            ajustes,
        );
        assert!(agenda.notas().is_some());
        assert_eq!(agenda.solicitud(), id);
        agenda.actualizar(
            Duration::from_secs(10),
            Some(ProgresoMusica {
                evento: id,
                transcurrido: Duration::ZERO,
                finalizada: true,
            }),
            false,
            ajustes,
        );
        assert!(agenda.notas().is_none());
        assert_eq!(agenda.solicitud(), 0);
        agenda.actualizar(Duration::from_secs(11), None, false, ajustes);
        assert_eq!(agenda.solicitud(), 0);
        agenda.actualizar(Duration::from_secs(12), None, false, ajustes);
        assert!(agenda.solicitud() > id);
        agenda.cancelar();
        assert_eq!(agenda.solicitud(), 0);
        agenda.abrir(v.id, candidatos(&p, &v), Duration::from_secs(20), ajustes);
        agenda.actualizar(
            Duration::from_secs(21),
            Some(ProgresoMusica {
                evento: id,
                transcurrido: Duration::from_secs(1),
                finalizada: false,
            }),
            false,
            ajustes,
        );
        assert!(agenda.notas().is_none() && agenda.solicitud() > id);
    }

    #[test]
    fn sin_candidatos_no_hay_piezas_y_se_alternan_varios_musicos() {
        let (_, v) = escenario();
        let ajustes = AgendaMusica {
            inicial: (1, 1),
            pausa: (1, 1),
        };
        let mut a = MusicaCampamento::default();
        a.abrir(v.id, vec![], Duration::ZERO, ajustes);
        a.actualizar(Duration::from_secs(100), None, false, ajustes);
        assert_eq!(a.solicitud(), 0);
        let actores: Vec<_> = v.integrantes.iter().take(2).map(|i| i.actor).collect();
        a.abrir(v.id, actores.clone(), Duration::ZERO, ajustes);
        let mut elegidos = std::collections::BTreeSet::new();
        for i in 1..=20 {
            let ahora = Duration::from_secs(i);
            a.actualizar(ahora, None, false, ajustes);
            let (id, actor) = a.evento.unwrap();
            assert!(actores.contains(&actor));
            elegidos.insert(actor);
            a.actualizar(
                ahora,
                Some(ProgresoMusica {
                    evento: id,
                    transcurrido: Duration::ZERO,
                    finalizada: true,
                }),
                false,
                ajustes,
            );
        }
        assert_eq!(elegidos.len(), 2);
    }

    #[test]
    fn varios_musicos_comparten_un_solo_turno_hasta_terminar_la_pieza() {
        let (_, v) = escenario();
        let ajustes = AgendaMusica {
            inicial: (1, 1),
            pausa: (1, 1),
        };
        let actores: Vec<_> = v.integrantes.iter().map(|i| i.actor).collect();
        assert!(actores.len() >= 2);
        let mut agenda = MusicaCampamento::default();
        agenda.abrir(v.id, actores.clone(), Duration::ZERO, ajustes);
        agenda.actualizar(Duration::from_secs(1), None, false, ajustes);
        let (id, actor) = agenda.evento.unwrap();
        // Ni una espera de confirmación ni la reproducción admiten otro músico.
        agenda.actualizar(Duration::from_secs(100), None, false, ajustes);
        assert_eq!(agenda.evento, Some((id, actor)));
        for segundo in 101..120 {
            agenda.actualizar(
                Duration::from_secs(segundo),
                Some(ProgresoMusica {
                    evento: id,
                    transcurrido: Duration::from_secs(segundo - 100),
                    finalizada: false,
                }),
                false,
                ajustes,
            );
            assert_eq!(agenda.solicitud(), id);
            assert_eq!(agenda.notas().unwrap().actor, actor);
        }
        agenda.actualizar(
            Duration::from_secs(120),
            Some(ProgresoMusica {
                evento: id,
                transcurrido: Duration::ZERO,
                finalizada: true,
            }),
            false,
            ajustes,
        );
        assert_eq!(agenda.solicitud(), 0);
        agenda.actualizar(Duration::from_secs(121), None, false, ajustes);
        assert!(agenda.solicitud() > id);
        assert!(actores.contains(&agenda.evento.unwrap().1));
        assert!(agenda.notas().is_none());
    }

    #[test]
    fn esperas_largas_iniciales_y_pausas_amplias_respetan_sus_limites() {
        let (p, v) = escenario();
        let ajustes = AgendaMusica {
            inicial: (60, 120),
            pausa: (20, 180),
        };
        let mut agenda = MusicaCampamento::default();
        for vuelta in 0..16 {
            let ahora = Duration::from_secs(vuelta * 600);
            agenda.abrir(v.id, candidatos(&p, &v), ahora, ajustes);
            let espera = agenda.proxima - ahora;
            assert!((Duration::from_secs(60)..=Duration::from_secs(120)).contains(&espera));
            let inicio = agenda.proxima;
            agenda.actualizar(inicio - Duration::from_nanos(1), None, false, ajustes);
            assert_eq!(agenda.solicitud(), 0);
            agenda.actualizar(inicio, None, false, ajustes);
            let id = agenda.solicitud();
            assert!(id > 0);
            let fin = inicio + Duration::from_secs(61);
            agenda.actualizar(
                fin,
                Some(ProgresoMusica {
                    evento: id,
                    transcurrido: Duration::ZERO,
                    finalizada: true,
                }),
                false,
                ajustes,
            );
            let pausa = agenda.proxima - fin;
            assert!((Duration::from_secs(20)..=Duration::from_secs(180)).contains(&pausa));
            let siguiente = agenda.proxima;
            agenda.actualizar(siguiente - Duration::from_nanos(1), None, false, ajustes);
            assert_eq!(agenda.solicitud(), 0);
            agenda.actualizar(siguiente, None, false, ajustes);
            assert!(agenda.solicitud() > id);
        }
    }
}
