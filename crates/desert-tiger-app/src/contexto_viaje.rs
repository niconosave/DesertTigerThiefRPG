use crate::contenido::CatalogoPresentacion;
use desert_tiger_core::acciones::CatalogoRumbos;
use desert_tiger_core::{IdLugar, Mundo};

pub(crate) fn construir(
    mundo: &Mundo,
    presentacion: &CatalogoPresentacion,
) -> Result<CatalogoRumbos, String> {
    let tiene = |id: IdLugar, rasgo: &str| {
        mundo
            .lugar(id)
            .is_some_and(|l| l.caracteristicas().any(|c| c.como_str() == rasgo))
    };
    let hay_agua = mundo.lugares().iter().any(|l| tiene(l.id(), "acuatico"));
    if hay_agua && !mundo.lugares().iter().any(|l| tiene(l.id(), "costero")) {
        return Err("el mundo navegable no tiene ningún Lugar costero".into());
    }
    let mut catalogo = CatalogoRumbos::default();
    for a in mundo.lugares() {
        if tiene(a.id(), "pequeno") && tiene(a.id(), "vasto") {
            return Err(format!(
                "Lugar {:?}: pequeno y vasto son incompatibles",
                a.id()
            ));
        }
        for &b in mundo.lugares_conectados(a.id()).unwrap_or(&[]) {
            if !tiene(a.id(), "acuatico") && !tiene(b, "acuatico") {
                continue;
            }
            let origen = presentacion
                .lugar(a.id())
                .ok_or("centro de origen ausente")?
                .coordenada_central();
            let destino = presentacion
                .lugar(b)
                .ok_or("centro de destino ausente")?
                .coordenada_central();
            let dx = destino.x as f64 - origen.x as f64;
            let dy = origen.y as f64 - destino.y as f64;
            if dx == 0.0 && dy == 0.0 {
                return Err(format!(
                    "centros coincidentes en conexión navegable {:?} -> {b:?}",
                    a.id()
                ));
            }
            let rumbo = octante(dx, dy);
            catalogo.0.insert((a.id(), b), rumbo);
        }
    }
    Ok(catalogo)
}

fn octante(dx: f64, dy: f64) -> u8 {
    let grados = dy.atan2(dx).to_degrees().rem_euclid(360.0);
    (((grados + 22.5) / 45.0).floor() as u8) % 8
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn ocho_octantes_y_direccion_inversa() {
        for (i, (x, y)) in [
            (1., 0.),
            (1., 1.),
            (0., 1.),
            (-1., 1.),
            (-1., 0.),
            (-1., -1.),
            (0., -1.),
            (1., -1.),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(octante(x, y), i as u8);
            assert_eq!(octante(-x, -y), (i as u8 + 4) % 8);
        }
        let angulo = 22.5_f64.to_radians();
        assert_eq!(octante(angulo.cos(), angulo.sin()), 1);
    }
}
