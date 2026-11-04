//! Tráfico ilustrativo: no crea actores ni utiliza el reloj o el azar del juego.
use std::collections::{BTreeSet, VecDeque};

use crate::trafico_decorativo::{ConfiguracionTrafico, TraficoDecorativo};
use desert_tiger_app::{CoordenadaMapa, VistaMapa, VistaMundo};

const MAX_BARCOS: usize = 5;
const PASOS_POR_CELDA: usize = 4; // Un movimiento por segundo del reloj visual.

pub(crate) fn crear(mundo: &VistaMundo) -> TraficoDecorativo {
    let mapa = &mundo.mapa;
    let mut puertos = BTreeSet::new();
    for lugar in &mundo.lugares {
        if lugar.caracteristicas.iter().any(|c| c == "puerto") {
            // El símbolo de la ciudad puede estar tierra adentro: buscar su embarcadero visual.
            let centro = lugar.coordenada_central;
            let cercano = mapa
                .celdas
                .iter()
                .enumerate()
                .filter(|(_, t)| t.como_str() == "agua")
                .map(|(i, _)| {
                    (
                        centro.x.abs_diff(i % mapa.ancho) + centro.y.abs_diff(i / mapa.ancho),
                        i,
                    )
                })
                .filter(|(distancia, _)| *distancia <= 10)
                .min();
            if let Some((_, i)) = cercano {
                puertos.insert(i);
            }
        }
    }
    let mut rutas = Vec::new();
    let mut pares = BTreeSet::new();
    for &origen in &puertos {
        let (padres, distancias) = recorrer_mar(mapa, origen);
        let mut destinos: Vec<_> = puertos
            .iter()
            .copied()
            .filter(|&i| i != origen && distancias[i] != usize::MAX)
            .collect();
        destinos.sort_by_key(|&i| (distancias[i], i));
        // Pocos enlaces locales por puerto, en lugar de una red de todos con todos.
        destinos.truncate(2);
        let borde = (0..mapa.celdas.len())
            .filter(|&i| {
                let (x, y) = (i % mapa.ancho, i / mapa.ancho);
                distancias[i] != usize::MAX
                    && (x == 0 || y == 0 || x + 1 == mapa.ancho || y + 1 == mapa.alto)
            })
            .min_by_key(|&i| (distancias[i], i));
        if let Some(borde) = borde {
            destinos.push(borde);
        }
        for destino in destinos {
            let par = (origen.min(destino), origen.max(destino));
            if origen == destino || !pares.insert(par) {
                continue;
            }
            let mut camino = vec![destino];
            let mut actual = destino;
            while actual != origen {
                actual = padres[actual];
                camino.push(actual);
            }
            camino.reverse();
            rutas.push(
                camino
                    .into_iter()
                    .map(|i| CoordenadaMapa {
                        x: i % mapa.ancho,
                        y: i / mapa.ancho,
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }
    TraficoDecorativo::nuevo(
        rutas,
        ConfiguracionTrafico {
            apariencia: "barcoDecorativo",
            terreno: "agua",
            max_grupos: MAX_BARCOS,
            integrantes: 1,
            separacion: 0,
            pasos_por_celda: PASOS_POR_CELDA,
        },
    )
}

fn es_mar(mapa: &VistaMapa, c: CoordenadaMapa) -> bool {
    c.x < mapa.ancho
        && c.y < mapa.alto
        && mapa
            .celdas
            .get(c.y * mapa.ancho + c.x)
            .is_some_and(|t| t.como_str() == "agua")
}

fn recorrer_mar(mapa: &VistaMapa, origen: usize) -> (Vec<usize>, Vec<usize>) {
    let mut padres = vec![usize::MAX; mapa.celdas.len()];
    let mut distancias = vec![usize::MAX; mapa.celdas.len()];
    let mut cola = VecDeque::from([origen]);
    padres[origen] = origen;
    distancias[origen] = 0;
    while let Some(i) = cola.pop_front() {
        let (x, y) = (i % mapa.ancho, i / mapa.ancho);
        for (x, y) in [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            (x + 1 < mapa.ancho).then_some((x + 1, y)),
            (y + 1 < mapa.alto).then_some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
        {
            let vecino = y * mapa.ancho + x;
            if distancias[vecino] == usize::MAX && es_mar(mapa, CoordenadaMapa { x, y }) {
                padres[vecino] = i;
                distancias[vecino] = distancias[i] + 1;
                cola.push_back(vecino);
            }
        }
    }
    (padres, distancias)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::graficos::ModoGraficos;
    use crate::recursos::RecursosVisuales;
    use desert_tiger_app::{ClaveTipoTile, VisibilidadTile};
    use desert_tiger_app::{ConfiguracionPartida, Partida, cargar_contenido};
    use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

    fn mundo() -> VistaMundo {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        let mut mundo = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(0) })
            .unwrap()
            .mundo_visible();
        mundo.mapa.ancho = 7;
        mundo.mapa.alto = 5;
        mundo.mapa.celdas = vec![ClaveTipoTile::nueva("agua".into()).unwrap(); 35];
        mundo.mapa.visibilidad = vec![VisibilidadTile::Visible; 35];
        mundo.mapa.celdas[17] = ClaveTipoTile::nueva("arena".into()).unwrap();
        mundo.mapa.celdas[10] = ClaveTipoTile::nueva("aguaRio".into()).unwrap();
        mundo.lugares.truncate(2);
        for (i, puerto) in mundo.lugares.iter_mut().enumerate() {
            puerto.coordenada_central = CoordenadaMapa { x: 1 + i * 4, y: 2 };
            puerto.caracteristicas = vec!["puerto".into()];
        }
        mundo
    }

    fn recursos(modo: ModoGraficos) -> RecursosVisuales {
        RecursosVisuales::cargar_modo(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal"),
            modo,
        )
        .unwrap()
    }

    fn cuadro(
        trafico: &TraficoDecorativo,
        mundo: &VistaMundo,
        recursos: &RecursosVisuales,
        paso: usize,
        desplazamiento: u16,
    ) -> Buffer {
        let mut terminal = Terminal::new(TestBackend::new(10, 6)).unwrap();
        terminal
            .draw(|frame| {
                trafico.dibujar(frame, mundo, recursos, paso, |c| {
                    Some((c.x as u16 + desplazamiento, c.y as u16))
                })
            })
            .unwrap();
        terminal.backend().buffer().clone()
    }

    #[test]
    fn recorridos_solo_por_mar_unen_puertos_y_alcanzan_bordes() {
        let m = mundo();
        let t = crear(&m);
        assert!(!t.rutas.is_empty());
        assert_eq!(t.rutas, crear(&m).rutas);
        let puertos: Vec<_> = m.lugares.iter().map(|l| l.coordenada_central).collect();
        assert!(
            t.rutas.iter().any(
                |r| puertos.contains(r.first().unwrap()) && puertos.contains(r.last().unwrap())
            )
        );
        assert!(t.rutas.iter().any(|r| {
            r.iter()
                .any(|c| c.x == 0 || c.y == 0 || c.x == 6 || c.y == 4)
        }));
        for ruta in &t.rutas {
            assert!(ruta.iter().all(|&c| es_mar(&m.mapa, c)));
            assert!(
                ruta.windows(2)
                    .all(|p| p[0].x.abs_diff(p[1].x) + p[0].y.abs_diff(p[1].y) == 1)
            );
        }
        for paso in 0..t.periodo * 12 {
            let posiciones: Vec<_> = t.posiciones(paso).collect();
            assert!(posiciones.len() <= MAX_BARCOS);
            assert!(posiciones.iter().all(|&c| es_mar(&m.mapa, c)));
        }
        assert_eq!(
            t.posiciones(0).collect::<Vec<_>>(),
            t.posiciones(3).collect::<Vec<_>>()
        );
        assert!((4..t.periodo * 4).any(|p| t.posiciones(p).collect::<Vec<_>>() != t.posiciones(0).collect::<Vec<_>>()));
    }

    #[test]
    fn niebla_oculta_barcos_y_el_dibujo_respeta_camara_y_modos_graficos() {
        let mut m = mundo();
        let original = m.clone();
        let t = crear(&m);
        let r = recursos(ModoGraficos::Predeterminado);
        let visible = cuadro(&t, &m, &r, 0, 0);
        assert!(visible.content.iter().any(|c| c.symbol() == "◣"));
        assert_eq!(m, original);
        for estado in [VisibilidadTile::Desconocido, VisibilidadTile::Recordado] {
            m.mapa.visibilidad.fill(estado);
            assert!(
                cuadro(&t, &m, &r, 0, 0)
                    .content
                    .iter()
                    .all(|c| c.symbol() == " ")
            );
        }
        m.mapa.visibilidad.fill(VisibilidadTile::Visible);
        assert_eq!(cuadro(&t, &m, &r, 0, 0), visible);
        let movido = cuadro(&t, &m, &r, 0, 2);
        for c in t.posiciones(0) {
            assert_eq!(movido[(c.x as u16 + 2, c.y as u16)].symbol(), "◣");
        }
        let ascii = cuadro(&t, &m, &recursos(ModoGraficos::Ascii), 0, 0);
        assert!(ascii.content.iter().any(|c| c.symbol() == "^"));
        assert!(ascii.content.iter().all(|c| c.symbol().is_ascii()));
        assert!(
            cuadro(&t, &m, &recursos(ModoGraficos::Fuente), 0, 0)
                .content
                .iter()
                .any(|c| c.symbol() == "◣")
        );
    }

    #[test]
    fn sin_mar_o_sin_puertos_no_hay_trafico_y_el_mapa_oficial_es_navegable() {
        let mut m = mundo();
        m.mapa
            .celdas
            .fill(ClaveTipoTile::nueva("arena".into()).unwrap());
        assert!(crear(&m).rutas.is_empty());
        m = mundo();
        m.lugares.clear();
        assert!(crear(&m).posiciones(0).next().is_none());
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        let p = Partida::nueva(&c, ConfiguracionPartida { semilla: Some(0) }).unwrap();
        let antes = p.vista();
        let mapa = p.mundo_visible();
        let t = crear(&mapa);
        assert!(!t.rutas.is_empty());
        assert!(t.rutas.iter().flatten().all(|&c| es_mar(&mapa.mapa, c)));
        assert_eq!(p.vista(), antes);
    }
}
