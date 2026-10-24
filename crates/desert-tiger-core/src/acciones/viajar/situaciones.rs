use crate::{Caracteristica, IdLugar, Mundo};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MovimientoEtapa {
    Marchar,
    Escalar,
    Descender,
}

pub(crate) fn movimiento_etapa(
    mundo: &Mundo,
    origen: IdLugar,
    destino: IdLugar,
) -> Option<MovimientoEtapa> {
    let origen = mundo.lugar(origen)?;
    let destino = mundo.lugar(destino)?;
    let empinado_origen = tiene(origen, "empinado");
    let empinado_destino = tiene(destino, "empinado");
    let elevado_origen = tiene(origen, "elevado");
    let elevado_destino = tiene(destino, "elevado");

    if (empinado_destino && !elevado_origen) || (empinado_origen && elevado_destino) {
        Some(MovimientoEtapa::Escalar)
    } else if (empinado_origen && !elevado_destino) || (empinado_destino && elevado_origen) {
        Some(MovimientoEtapa::Descender)
    } else {
        Some(MovimientoEtapa::Marchar)
    }
}

pub(crate) fn exposicion_conocida(mundo: &Mundo, origen: IdLugar, destino: IdLugar) -> Option<u32> {
    let movimiento = movimiento_etapa(mundo, origen, destino)?;
    let lugar_origen = mundo.lugar(origen)?;
    let lugar_destino = mundo.lugar(destino)?;
    let riesgo = match movimiento {
        MovimientoEtapa::Escalar => 3,
        MovimientoEtapa::Descender => 2,
        MovimientoEtapa::Marchar => {
            let rasgos = ["pantanoso", "rocoso", "corrienteRapida", "estrecho"];
            u32::from(
                rasgos
                    .iter()
                    .any(|rasgo| tiene(lugar_origen, rasgo) || tiene(lugar_destino, rasgo)),
            )
        }
    };
    Some(riesgo)
}

fn tiene(lugar: &crate::Lugar, valor: &str) -> bool {
    lugar.tiene_caracteristica(&caracteristica(valor))
}

fn caracteristica(valor: &str) -> Caracteristica {
    Caracteristica::nueva(valor.to_owned())
        .unwrap_or_else(|_| unreachable!("Característica literal válida"))
}
