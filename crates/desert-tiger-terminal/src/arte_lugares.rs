//! Paisajes anclados a la pantalla, independientes de las coordenadas del mapa.
use desert_tiger_app::ArteLugar;
use ratatui::{Frame, layout::Rect};

use crate::{dibujo_caracteres::dibujar_celdas, recursos::RecursosVisuales};

pub(crate) fn origen_arte(area: Rect, ancho: usize, alto: usize) -> Option<(i64, i64)> {
    let (ancho, alto) = (i64::try_from(ancho).ok()?, i64::try_from(alto).ok()?);
    Some((
        i64::from(area.x) + (i64::from(area.width) - ancho).div_euclid(2),
        i64::from(area.bottom()) - alto,
    ))
}

pub(crate) fn dibujar(
    frame: &mut Frame<'_>,
    area: Rect,
    arte: &ArteLugar,
    recursos: &RecursosVisuales,
) {
    if let Some((x, y)) = origen_arte(area, arte.ancho(), arte.alto()) {
        dibujar_celdas(frame, area, x, y, arte.ancho(), arte.celdas(), recursos);
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn centra_con_margenes_y_recorta_sin_mover_la_base() {
        let area = Rect::new(10, 4, 20, 12);
        assert_eq!(origen_arte(area, 10, 5), Some((15, 11)));
        assert_eq!(origen_arte(area, 11, 5), Some((14, 11)));
        assert_eq!(origen_arte(area, 25, 18), Some((7, -2)));
        assert_eq!(
            origen_arte(Rect::new(0, 0, 80, 22), 160, 10),
            Some((-40, 12))
        );
        assert_eq!(
            origen_arte(Rect::new(0, 0, 120, 30), 160, 10),
            Some((-20, 20))
        );
    }
}
