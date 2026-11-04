//! Percepción del mapa de la partida, independiente de las reglas y del renderizador.
use crate::contenido::{ClaveTipoTile, CoordenadaMapa, EstadoMapa};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibilidadTile {
    Desconocido,
    Recordado,
    Visible,
}

pub(crate) struct VisionMapa {
    visibles: Vec<bool>,
    explorados: Vec<bool>,
    tipos_recordados: Vec<ClaveTipoTile>,
}

impl VisionMapa {
    pub(crate) fn nueva(mapa: &EstadoMapa) -> Self {
        Self {
            visibles: vec![false; mapa.celdas().len()],
            explorados: vec![false; mapa.celdas().len()],
            tipos_recordados: mapa.celdas().to_vec(),
        }
    }
    pub(crate) fn observar(&mut self, mapa: &EstadoMapa, origen: CoordenadaMapa) {
        let parametros = mapa.parametros_vision();
        self.visibles = calcular_visibles(
            mapa.ancho(),
            mapa.alto(),
            mapa.alturas(),
            origen,
            parametros.radio,
            parametros.altura_observador,
        );
        for (i, &visible) in self.visibles.iter().enumerate() {
            if visible {
                self.explorados[i] = true;
                self.tipos_recordados[i] = mapa.celdas()[i].clone();
            }
        }
    }
    pub(crate) fn proyectar(
        &self,
        mapa: &EstadoMapa,
    ) -> (Vec<ClaveTipoTile>, Vec<VisibilidadTile>) {
        let estados: Vec<_> = self
            .visibles
            .iter()
            .zip(&self.explorados)
            .map(|(&visible, &explorado)| {
                if visible {
                    VisibilidadTile::Visible
                } else if explorado {
                    VisibilidadTile::Recordado
                } else {
                    VisibilidadTile::Desconocido
                }
            })
            .collect();
        let tipos = estados
            .iter()
            .enumerate()
            .map(|(i, estado)| {
                if *estado == VisibilidadTile::Recordado {
                    self.tipos_recordados[i].clone()
                } else {
                    mapa.celdas()[i].clone()
                }
            })
            .collect();
        (tipos, estados)
    }
}

pub(crate) fn calcular_visibles(
    ancho: usize,
    alto: usize,
    alturas: &[u16],
    origen: CoordenadaMapa,
    radio: u16,
    altura_observador: u16,
) -> Vec<bool> {
    let mut visibles = vec![false; ancho * alto];
    let radio = usize::from(radio);
    for y in origen.y.saturating_sub(radio)..=origen.y.saturating_add(radio).min(alto - 1) {
        for x in origen.x.saturating_sub(radio)..=origen.x.saturating_add(radio).min(ancho - 1) {
            let dx = x as i128 - origen.x as i128;
            let dy = y as i128 - origen.y as i128;
            if dx * dx + dy * dy <= (radio as i128).pow(2)
                && linea_visible(
                    ancho,
                    alturas,
                    origen,
                    CoordenadaMapa { x, y },
                    altura_observador,
                )
            {
                visibles[y * ancho + x] = true;
            }
        }
    }
    visibles
}

fn linea_visible(
    ancho: usize,
    alturas: &[u16],
    s: CoordenadaMapa,
    t: CoordenadaMapa,
    ojo: u16,
) -> bool {
    let (sx, sy, tx, ty) = (s.x as i128, s.y as i128, t.x as i128, t.y as i128);
    let (dx, dy) = (tx - sx, ty - sy);
    let d = dx * dx + dy * dy;
    if d == 0 {
        return true;
    }
    let zs = i128::from(alturas[s.y * ancho + s.x]) + i128::from(ojo);
    let zt = i128::from(alturas[t.y * ancho + t.x]);
    let bloquea = |x: i128, y: i128| {
        if (x == sx && y == sy) || (x == tx && y == ty) {
            return false;
        }
        let u = (x - sx) * dx + (y - sy) * dy;
        i128::from(alturas[y as usize * ancho + x as usize]) * d > zs * d + (zt - zs) * u
    };
    let (ax, ay) = (dx.abs(), dy.abs());
    let (paso_x, paso_y) = (dx.signum(), dy.signum());
    let (mut x, mut y, mut nx, mut ny) = (sx, sy, 1_i128, 1_i128);
    while x != tx || y != ty {
        let orden = if ax == 0 {
            std::cmp::Ordering::Greater
        } else if ay == 0 {
            std::cmp::Ordering::Less
        } else {
            (nx * ay).cmp(&(ny * ax))
        };
        match orden {
            std::cmp::Ordering::Less => {
                x += paso_x;
                nx += 2;
            }
            std::cmp::Ordering::Greater => {
                y += paso_y;
                ny += 2;
            }
            std::cmp::Ordering::Equal => {
                if bloquea(x + paso_x, y) || bloquea(x, y + paso_y) {
                    return false;
                }
                x += paso_x;
                y += paso_y;
                nx += 2;
                ny += 2;
            }
        }
        if bloquea(x, y) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn llanura_radio_bordes_y_origen() {
        let alturas = vec![0; 81];
        let origen = CoordenadaMapa { x: 4, y: 4 };
        let mascara = calcular_visibles(9, 9, &alturas, origen, 3, 1);
        for y in 0..9 {
            for x in 0..9 {
                assert_eq!(
                    mascara[y * 9 + x],
                    (x as i32 - 4).pow(2) + (y as i32 - 4).pow(2) <= 9
                );
            }
        }
        assert_eq!(
            calcular_visibles(1, 1, &[65535], CoordenadaMapa { x: 0, y: 0 }, 1, 65535),
            vec![true]
        );
    }
    #[test]
    fn relieve_oculta_y_un_pico_puede_sobresalir() {
        let s = CoordenadaMapa { x: 0, y: 0 };
        let bajos = calcular_visibles(5, 1, &[0, 0, 4, 0, 0], s, 5, 1);
        assert_eq!(bajos, [true, true, true, false, false]);
        let pico = calcular_visibles(5, 1, &[0, 0, 4, 0, 10], s, 5, 1);
        assert!(pico[4]);
        assert!(calcular_visibles(5, 1, &[10, 0, 4, 0, 0], s, 5, 1)[4]);
        // El rayo toca exactamente la altura intermedia: no bloquea.
        assert!(calcular_visibles(3, 1, &[0, 2, 3], s, 3, 1)[2]);
    }
    #[test]
    fn las_esquinas_no_permiten_ver_entre_obstaculos() {
        let alturas = [0, 4, 0, 4, 0, 0, 0, 0, 0];
        assert!(!calcular_visibles(3, 3, &alturas, CoordenadaMapa { x: 0, y: 0 }, 4, 1)[4]);
        assert!(
            calcular_visibles(3, 3, &[0; 9], CoordenadaMapa { x: 0, y: 0 }, 4, 1)
                .iter()
                .all(|v| *v)
        );
    }
    #[test]
    fn memoria_conserva_tipo_no_observado_y_se_actualiza_al_volver() {
        let c = crate::cargar_contenido(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (_, mut mapa) = c.instanciar_estado().unwrap();
        let origen = CoordenadaMapa { x: 0, y: 0 };
        let mut vision = VisionMapa::nueva(&mapa);
        assert!(
            vision
                .proyectar(&mapa)
                .1
                .iter()
                .all(|s| *s == VisibilidadTile::Desconocido)
        );
        vision.observar(&mapa, origen);
        let antes = mapa.celdas()[0].clone();
        vision.observar(
            &mapa,
            CoordenadaMapa {
                x: mapa.ancho() - 1,
                y: mapa.alto() - 1,
            },
        );
        mapa.cambiar_tipo(origen, ClaveTipoTile::nueva("arena".into()).unwrap())
            .unwrap();
        let (tipos, estados) = vision.proyectar(&mapa);
        assert_eq!(estados[0], VisibilidadTile::Recordado);
        assert_eq!(tipos[0], antes);
        vision.observar(&mapa, origen);
        assert_eq!(vision.proyectar(&mapa).0[0].como_str(), "arena");
    }

    #[test]
    fn observacion_inicial_del_mapa_oficial_tiene_alcance_limitado() {
        let c = crate::cargar_contenido(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mundo, mapa) = c.instanciar_estado().unwrap();
        let origen = c
            .presentacion()
            .lugar(mundo.ubicacion_grupo())
            .unwrap()
            .coordenada_central()
            .unwrap();
        let mut vision = VisionMapa::nueva(&mapa);
        let inicio = std::time::Instant::now();
        vision.observar(&mapa, origen);
        let tiempo = inicio.elapsed();
        let visibles = vision.visibles.iter().filter(|&&v| v).count();
        assert!(visibles > 1 && visibles < mapa.celdas().len());
        assert!(vision.visibles[origen.y * mapa.ancho() + origen.x]);
        println!(
            "Observación oficial desde ({}, {}): {} tiles visibles en {:?}",
            origen.x, origen.y, visibles, tiempo
        );
    }
}
