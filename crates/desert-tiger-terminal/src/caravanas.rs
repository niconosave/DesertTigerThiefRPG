//! Recorridos exclusivamente visuales sobre los caminos dibujados en el mapa.
use std::collections::{BTreeSet, VecDeque};

use desert_tiger_app::{CoordenadaMapa, VistaMapa, VistaMundo};

use crate::trafico_decorativo::{ConfiguracionTrafico, TraficoDecorativo, es_terreno};

pub(crate) fn crear(mundo: &VistaMundo) -> TraficoDecorativo {
    let mapa = &mundo.mapa;
    let mut pueblos = BTreeSet::new();
    for lugar in &mundo.lugares {
        if !["civilizado", "poblado"]
            .iter()
            .all(|tipo| lugar.caracteristicas.iter().any(|c| c == tipo))
        {
            continue;
        }
        let centro = lugar.coordenada_central;
        let cercano = mapa
            .celdas
            .iter()
            .enumerate()
            .filter(|(_, t)| t.como_str() == "camino")
            .map(|(i, _)| {
                (
                    centro
                        .x
                        .abs_diff(i % mapa.ancho)
                        .max(centro.y.abs_diff(i / mapa.ancho)),
                    i,
                )
            })
            .filter(|(d, _)| *d <= 2)
            .min();
        if let Some((_, i)) = cercano {
            pueblos.insert(i);
        }
    }
    let mut rutas = Vec::new();
    let mut pares = BTreeSet::new();
    for &origen in &pueblos {
        let (padres, distancias) = recorrer_caminos(mapa, origen);
        let mut destinos: Vec<_> = pueblos
            .iter()
            .copied()
            .filter(|&i| i != origen && distancias[i] != usize::MAX)
            .collect();
        destinos.sort_by_key(|&i| (distancias[i], i));
        for destino in destinos.into_iter().take(2) {
            if !pares.insert((origen.min(destino), origen.max(destino))) {
                continue;
            }
            let mut actual = destino;
            let mut ruta = vec![actual];
            while actual != origen {
                actual = padres[actual];
                ruta.push(actual);
            }
            ruta.reverse();
            rutas.push(
                ruta.into_iter()
                    .map(|i| CoordenadaMapa {
                        x: i % mapa.ancho,
                        y: i / mapa.ancho,
                    })
                    .collect(),
            );
        }
    }
    TraficoDecorativo::nuevo(
        rutas,
        ConfiguracionTrafico {
            apariencia: "caravanaDecorativa",
            terreno: "camino",
            max_grupos: 3,
            integrantes: 3,
            separacion: 2,
            pasos_por_celda: 6,
        },
    )
}

fn recorrer_caminos(mapa: &VistaMapa, origen: usize) -> (Vec<usize>, Vec<usize>) {
    let mut padres = vec![usize::MAX; mapa.celdas.len()];
    let mut distancias = vec![usize::MAX; mapa.celdas.len()];
    padres[origen] = origen;
    distancias[origen] = 0;
    let mut cola = VecDeque::from([origen]);
    while let Some(i) = cola.pop_front() {
        let (x, y) = (i % mapa.ancho, i / mapa.ancho);
        // Las diagonales son necesarias para seguir los caminos de Bresenham del mapa.
        for ny in y.saturating_sub(1)..=(y + 1).min(mapa.alto - 1) {
            for nx in x.saturating_sub(1)..=(x + 1).min(mapa.ancho - 1) {
                let vecino = ny * mapa.ancho + nx;
                if distancias[vecino] == usize::MAX
                    && es_terreno(mapa, CoordenadaMapa { x: nx, y: ny }, "camino")
                {
                    padres[vecino] = i;
                    distancias[vecino] = distancias[i] + 1;
                    cola.push_back(vecino);
                }
            }
        }
    }
    (padres, distancias)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{graficos::ModoGraficos, recursos::RecursosVisuales};
    use desert_tiger_app::{
        ClaveTipoTile, ConfiguracionPartida, Partida, VisibilidadTile, cargar_contenido,
    };
    use ratatui::{Terminal, backend::TestBackend};

    fn oficial() -> VistaMundo {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = cargar_contenido(&raiz.join("content/base")).unwrap();
        Partida::nueva(&c, ConfiguracionPartida { semilla: Some(0) })
            .unwrap()
            .mundo_visible()
    }

    fn muestra() -> VistaMundo {
        let mut m = oficial();
        m.mapa.ancho = 9;
        m.mapa.alto = 7;
        m.mapa.celdas = vec![ClaveTipoTile::nueva("arena".into()).unwrap(); 63];
        m.mapa.visibilidad = vec![VisibilidadTile::Visible; 63];
        for (x, y) in [
            (0, 3),
            (1, 3),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 4),
            (7, 3),
            (8, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (0, 0),
        ] {
            m.mapa.celdas[y * 9 + x] = ClaveTipoTile::nueva("camino".into()).unwrap();
        }
        m.lugares.truncate(4);
        for (l, (x, y)) in m.lugares.iter_mut().zip([(0, 3), (8, 3), (4, 6), (0, 0)]) {
            l.coordenada_central = CoordenadaMapa { x, y };
            l.caracteristicas = vec!["civilizado".into(), "poblado".into()];
        }
        m
    }

    #[test]
    fn sigue_diagonales_y_ramales_sin_saltar_a_un_camino_desconectado() {
        let m = muestra();
        let t = crear(&m);
        assert_eq!(t.rutas.len(), 3);
        assert_eq!(crear(&m).rutas, t.rutas);
        for r in &t.rutas {
            assert!(r.iter().all(|&c| es_terreno(&m.mapa, c, "camino")));
            assert!(!r.contains(&CoordenadaMapa { x: 0, y: 0 }));
            assert!(
                r.windows(2)
                    .all(|p| p[0].x.abs_diff(p[1].x).max(p[0].y.abs_diff(p[1].y)) == 1)
            );
        }
        assert!(
            t.rutas
                .iter()
                .any(|r| r.windows(2).any(|p| p[0].x != p[1].x && p[0].y != p[1].y))
        );
        for paso in 0..t.periodo * 12 {
            let posiciones: Vec<_> = t.posiciones(paso).collect();
            assert!(posiciones.len() <= 9);
            assert!(
                posiciones
                    .into_iter()
                    .all(|c| es_terreno(&m.mapa, c, "camino"))
            );
        }
        let mut sin_caminos = m.clone();
        sin_caminos
            .mapa
            .celdas
            .fill(ClaveTipoTile::nueva("arena".into()).unwrap());
        assert!(crear(&sin_caminos).rutas.is_empty());
        let mut sin_pueblos = m;
        for l in &mut sin_pueblos.lugares {
            l.caracteristicas = vec!["civilizado".into(), "camino".into()];
        }
        assert!(crear(&sin_pueblos).rutas.is_empty());
    }

    #[test]
    fn caravana_respeta_niebla_alternativas_y_traslado_de_camara() {
        let mut m = muestra();
        let original = m.clone();
        let t = crear(&m);
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal");
        for (modo, glifo) in [
            (ModoGraficos::Predeterminado, "🬌"),
            (ModoGraficos::Ascii, "h"),
            (ModoGraficos::Fuente, "🬌"),
        ] {
            let recursos = RecursosVisuales::cargar_modo(&raiz, modo).unwrap();
            for estado in [
                VisibilidadTile::Visible,
                VisibilidadTile::Recordado,
                VisibilidadTile::Desconocido,
            ] {
                m.mapa.visibilidad.fill(estado);
                let mut terminal = Terminal::new(TestBackend::new(12, 8)).unwrap();
                terminal
                    .draw(|f| {
                        t.dibujar(f, &m, &recursos, 24, |c| Some((c.x as u16 + 2, c.y as u16)))
                    })
                    .unwrap();
                let buffer = terminal.backend().buffer();
                if estado == VisibilidadTile::Visible {
                    assert!(buffer.content.iter().any(|c| c.symbol() == glifo));
                    for c in t.posiciones(24) {
                        assert_eq!(buffer[(c.x as u16 + 2, c.y as u16)].symbol(), glifo);
                    }
                } else {
                    assert!(buffer.content.iter().all(|c| c.symbol() == " "));
                }
            }
        }
        m.mapa.visibilidad.fill(VisibilidadTile::Visible);
        assert_eq!(m, original);
    }

    #[test]
    fn el_mapa_oficial_incluye_los_ramales_largos_y_los_puentes() {
        let m = oficial();
        let t = crear(&m);
        assert!(!t.rutas.is_empty());
        for r in &t.rutas {
            assert!(r.iter().all(|&c| es_terreno(&m.mapa, c, "camino")));
        }
        for nombre in ["Debaret", "Til Azur", "Tazar"] {
            let centro = m
                .lugares
                .iter()
                .find(|l| l.nombre == nombre)
                .unwrap()
                .coordenada_central;
            assert!(
                t.rutas
                    .iter()
                    .any(|r| r.first() == Some(&centro) || r.last() == Some(&centro)),
                "{nombre}"
            );
        }
        let puente = m
            .lugares
            .iter()
            .find(|l| l.caracteristicas.iter().any(|c| c == "puente"))
            .unwrap()
            .coordenada_central;
        assert!(t.rutas.iter().any(|r| r.contains(&puente)));
    }
}
