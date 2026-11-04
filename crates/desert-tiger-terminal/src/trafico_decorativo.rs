//! Reloj, composición y niebla compartidos por el tráfico ilustrativo del mapa.
use desert_tiger_app::{ClaveTipoTile, CoordenadaMapa, VisibilidadTile, VistaMapa, VistaMundo};
use ratatui::{Frame, style::Modifier};

use crate::recursos::RecursosVisuales;

pub(crate) struct ConfiguracionTrafico {
    pub apariencia: &'static str,
    pub terreno: &'static str,
    pub max_grupos: usize,
    pub integrantes: usize,
    pub separacion: usize,
    pub pasos_por_celda: usize,
}

pub(crate) struct TraficoDecorativo {
    pub(crate) rutas: Vec<Vec<CoordenadaMapa>>,
    pub(crate) periodo: usize,
    apariencia: ClaveTipoTile,
    configuracion: ConfiguracionTrafico,
    pausa: usize,
}

impl TraficoDecorativo {
    pub(crate) fn nuevo(
        rutas: Vec<Vec<CoordenadaMapa>>,
        configuracion: ConfiguracionTrafico,
    ) -> Self {
        let cola = (configuracion.integrantes - 1) * configuracion.separacion;
        let pausa = (std::time::Duration::from_secs(12).as_millis()
            / crate::animacion::PASO_ANIMACION.as_millis()) as usize
            / configuracion.pasos_por_celda;
        let periodo = rutas
            .iter()
            .map(|r| r.len() + cola + pausa)
            .sum::<usize>()
            .max(1);
        Self {
            rutas,
            periodo,
            apariencia: ClaveTipoTile::nueva(configuracion.apariencia.into())
                .expect("clave constante"),
            configuracion,
            pausa,
        }
    }

    pub(crate) fn posiciones(&self, paso: usize) -> impl Iterator<Item = CoordenadaMapa> {
        let c = &self.configuracion;
        let cantidad = c.max_grupos.min(self.rutas.len());
        let cola = (c.integrantes - 1) * c.separacion;
        let mut posiciones = Vec::with_capacity(cantidad * c.integrantes);
        for grupo in 0..cantidad {
            let t = paso / c.pasos_por_celda + grupo * self.periodo / cantidad;
            let ciclo = t / self.periodo;
            let mut posicion = t % self.periodo;
            for (numero, ruta) in self.rutas.iter().enumerate() {
                if posicion < ruta.len() + cola {
                    for integrante in 0..c.integrantes {
                        if let Some(p) = posicion.checked_sub(integrante * c.separacion)
                            && p < ruta.len()
                        {
                            let indice = if (ciclo + grupo + numero).is_multiple_of(2) {
                                p
                            } else {
                                ruta.len() - 1 - p
                            };
                            posiciones.push(ruta[indice]);
                        }
                    }
                    break;
                }
                if posicion < ruta.len() + cola + self.pausa {
                    break;
                }
                posicion -= ruta.len() + cola + self.pausa;
            }
        }
        posiciones.into_iter()
    }

    pub(crate) fn dibujar(
        &self,
        frame: &mut Frame<'_>,
        mundo: &VistaMundo,
        recursos: &RecursosVisuales,
        paso: usize,
        proyectar: impl Fn(CoordenadaMapa) -> Option<(u16, u16)>,
    ) {
        if !recursos.contiene_tile(&self.apariencia) {
            return;
        }
        for c in self.posiciones(paso) {
            if mundo.mapa.visibilidad_en(c) != VisibilidadTile::Visible
                || !es_terreno(&mundo.mapa, c, self.configuracion.terreno)
            {
                continue;
            }
            if let Some((x, y)) = proyectar(c)
                && let Some(celda) = frame.buffer_mut().cell_mut((x, y))
            {
                let apariencia = recursos.tile(&self.apariencia, c.x, c.y, paso);
                let estilo = apariencia
                    .estilo
                    .remove_modifier(Modifier::all().difference(apariencia.estilo.add_modifier));
                celda.set_char(apariencia.glifo).set_style(estilo);
            }
        }
    }
}

pub(crate) fn es_terreno(mapa: &VistaMapa, c: CoordenadaMapa, tipo: &str) -> bool {
    c.x < mapa.ancho
        && c.y < mapa.alto
        && mapa
            .celdas
            .get(c.y * mapa.ancho + c.x)
            .is_some_and(|t| t.como_str() == tipo)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn la_fila_sigue_el_mismo_recorrido_con_separacion_y_desaparece_al_llegar() {
        let ruta: Vec<_> = (0..8).map(|x| CoordenadaMapa { x, y: x }).collect();
        let t = TraficoDecorativo::nuevo(
            vec![ruta.clone()],
            ConfiguracionTrafico {
                apariencia: "caravanaDecorativa",
                terreno: "camino",
                max_grupos: 1,
                integrantes: 3,
                separacion: 2,
                pasos_por_celda: 6,
            },
        );
        assert_eq!(t.posiciones(0).collect::<Vec<_>>(), [ruta[0]]);
        assert_eq!(t.posiciones(5).collect::<Vec<_>>(), [ruta[0]]);
        assert_eq!(t.posiciones(6).collect::<Vec<_>>(), [ruta[1]]);
        assert_eq!(t.posiciones(12).collect::<Vec<_>>(), [ruta[2], ruta[0]]);
        assert_eq!(
            t.posiciones(24).collect::<Vec<_>>(),
            [ruta[4], ruta[2], ruta[0]]
        );
        assert_eq!(t.posiciones(60).collect::<Vec<_>>(), [ruta[6]]);
        assert!(t.posiciones(72).next().is_none());
        assert_eq!(t.posiciones(t.periodo * 6).collect::<Vec<_>>(), [ruta[7]]);
    }
}
