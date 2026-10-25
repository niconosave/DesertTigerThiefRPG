use super::*;
pub(crate) fn resolver(
    c: &ContextoMovimiento<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<Movimiento> {
    let rumbo = c.rumbo.ok_or_else(|| fallo("rumbo navegable ausente"))?;
    let viento = match c.mes {
        1..=3 => 0,
        6..=7 => 4,
        9..=10 => 2,
        _ => sortear(azar, 8)? as u8,
    };
    let distancia = rumbo.abs_diff(viento).min(8 - rumbo.abs_diff(viento));
    let ajuste = if distancia <= 1 {
        -24
    } else if distancia == 2 {
        0
    } else {
        48
    };
    let mut m = comenzar(c, Medio::Barco, AccionParticular::NavegarEnBarco, 14, azar)?;
    m.dato.viento = Some((rumbo, viento, ajuste));
    terminar_mitades(c, &mut m, [0, 0], false)?;
    if ajuste < 0 {
        m.duracion.0 = m
            .duracion
            .0
            .checked_sub(HorasViaje::horas(24).0)
            .ok_or_else(|| fallo("viento produce tiempo negativo"))?;
    } else {
        m.duracion = m.duracion.sumar(HorasViaje::horas(ajuste as u64))?;
    }
    if m.dato.chequeos.iter().all(|r| !r.exito) {
        let dado = (sortear(azar, 20)? + 1) as u8;
        let i = mitad_incidente(&m);
        let lugar = [c.origen, c.destino][i];
        let tipo = match dado {
            1..=6 => Incidente::Olas,
            7..=12 => Incidente::Lluvia,
            13..=16 => Incidente::SinViento,
            17..=18 => Incidente::Naufragio,
            _ if c.viajeros.len() > 1 => Incidente::PorLaBorda,
            _ => Incidente::Naufragio,
        };
        let demora = match tipo {
            Incidente::Olas => 12,
            Incidente::Lluvia => 24,
            Incidente::SinViento => 48,
            _ => 0,
        };
        let costas: Vec<_> = c
            .mundo
            .lugares()
            .iter()
            .filter(|l| lugar_tiene(c.mundo, l.id(), "costero"))
            .map(|l| l.id())
            .collect();
        let mut actor = None;
        if tipo == Incidente::PorLaBorda {
            let id = afectado(c, true, azar)?;
            actor = Some(id);
            let costa = costas[sortear(azar, costas.len())?];
            m.operaciones.push(OperacionMundo::SepararCompanero {
                actor: id,
                lugar: costa,
            });
            m.viajeros.retain(|&x| x != id);
        } else if tipo == Incidente::Naufragio {
            if lugar_tiene(c.mundo, lugar, "costero") {
                m.lugar = lugar;
            } else {
                for (j, &id) in c.viajeros.iter().enumerate() {
                    let costa = costas[sortear(azar, costas.len())?];
                    if j == 0 {
                        m.lugar = costa;
                    } else {
                        m.operaciones.push(OperacionMundo::SepararCompanero {
                            actor: id,
                            lugar: costa,
                        });
                    }
                }
                m.viajeros.truncate(1);
            }
            m.cierre = Some(CierreViaje::Naufragio);
        }
        m.dato.incidente = Some(IncidenteResuelto {
            tipo,
            dado,
            lugar,
            afectado: actor,
            recuperacion: None,
            monturas_restantes: None,
            demora: HorasViaje::horas(demora).tiempo()?,
        });
        m.duracion = m.duracion.sumar(HorasViaje::horas(demora))?;
    }
    Ok(m)
}
