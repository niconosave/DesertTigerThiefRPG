use super::AmbienteSonoro;
use desert_tiger_app::{IdLugar, VistaMundo};

fn buscar(mundo: &VistaMundo, id: IdLugar) -> Option<&desert_tiger_app::VistaLugar> {
    // Las vistas oficiales conservan el orden de IDs; el respaldo admite vistas
    // construidas/reordenadas por otras herramientas sin alterar la selección.
    id.indice()
        .and_then(|i| mundo.lugares.get(i))
        .filter(|l| l.id == id)
        .or_else(|| mundo.lugares.iter().find(|l| l.id == id))
}

pub(super) fn seleccionar(mundo: &VistaMundo, id: IdLugar, fogata: bool) -> AmbienteSonoro {
    let Some(lugar) = buscar(mundo, id) else {
        return AmbienteSonoro::SILENCIO;
    };
    let tiene = |clave: &str| lugar.caracteristicas.iter().any(|c| c == clave);
    // Solo vecinos del lugar actual, sin recorrer toda la red de conexiones.
    let rio = tiene("fluvial")
        || lugar.conectados.iter().any(|id| {
            buscar(mundo, *id).is_some_and(|l| l.caracteristicas.iter().any(|c| c == "fluvial"))
        });
    AmbienteSonoro {
        exterior: true,
        fogata,
        rio,
        mar: tiene("costero") || tiene("maritimo"),
        bosque: tiene("boscoso"),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_app::{ConfiguracionPartida, Partida, cargar_contenido};
    use std::path::Path;

    fn mundo() -> VistaMundo {
        let contenido =
            cargar_contenido(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"))
                .unwrap();
        Partida::nueva(&contenido, ConfiguracionPartida { semilla: Some(7) })
            .unwrap()
            .mundo_visible()
    }

    #[test]
    fn lugares_oficiales_seleccionan_bosque_rio_inverso_costa_y_mar() {
        let m = mundo();
        let lugar = |nombre: &str| m.lugares.iter().find(|l| l.nombre == nombre).unwrap();
        let bosque = lugar("Bosque oscuro");
        let vado = lugar("Vado de Tamrut");
        assert!(bosque.conectados.contains(&vado.id));
        let ambiente = seleccionar(&m, bosque.id, true);
        assert!(
            ambiente.exterior
                && ambiente.bosque
                && ambiente.rio
                && ambiente.fogata
                && !ambiente.mar
        );
        for nombre in ["Costa Ámbar", "Mar de Poniente"] {
            assert!(seleccionar(&m, lugar(nombre).id, false).mar);
        }
        assert!(seleccionar(&m, vado.id, false).rio);
        assert_eq!(
            seleccionar(&m, IdLugar::desde_posicion(9999).unwrap(), false),
            AmbienteSonoro::SILENCIO
        );
    }

    #[test]
    fn agua_dulce_no_implica_rio_ni_se_propaga_desde_segundo_grado() {
        let mut m = mundo();
        let ids: Vec<_> = m.lugares.iter().take(3).map(|l| l.id).collect();
        for l in &mut m.lugares {
            l.caracteristicas.clear();
            l.conectados.clear();
        }
        m.lugares[0].caracteristicas.push("aguaDulce".into());
        m.lugares[0].conectados.push(ids[1]);
        m.lugares[1].conectados.push(ids[2]);
        m.lugares[2].caracteristicas.push("fluvial".into());
        assert!(!seleccionar(&m, ids[0], false).rio);
        m.lugares[1].caracteristicas.push("fluvial".into());
        assert!(seleccionar(&m, ids[0], false).rio);
        m.lugares.reverse();
        assert!(seleccionar(&m, ids[0], false).rio);
    }
}
