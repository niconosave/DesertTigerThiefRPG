use desert_tiger_app::CoordenadaMapa;

pub(crate) fn recorrer_linea(
    origen: CoordenadaMapa,
    destino: CoordenadaMapa,
    mut dibujar: impl FnMut(CoordenadaMapa),
) {
    // Se rasteriza en coordenadas del mapa, antes de recortar por la cámara.
    // i128 contiene sin pérdida cualquier coordenada usize admitida.
    let (mut x, mut y) = (origen.x as i128, origen.y as i128);
    let (final_x, final_y) = (destino.x as i128, destino.y as i128);
    let dx = (final_x - x).abs();
    let dy = -(final_y - y).abs();
    let sx = if x < final_x { 1 } else { -1 };
    let sy = if y < final_y { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        dibujar(CoordenadaMapa {
            x: x as usize,
            y: y as usize,
        });
        if (x, y) == (final_x, final_y) {
            break;
        }
        let doble = 2 * error;
        if doble >= dy {
            error += dy;
            x += sx;
        }
        if doble <= dx {
            error += dx;
            y += sy;
        }
    }
}
