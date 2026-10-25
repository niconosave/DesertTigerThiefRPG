use super::*;
pub(crate) fn resolver(
    c: &ContextoMovimiento<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<Movimiento> {
    let mut m = comenzar(c, Medio::Pie, AccionParticular::Andar, 5, azar)?;
    let previo = geografia::estado_pie(c.mundo, c.viajeros);
    let mut estados = [previo; 2];
    let mut demora = 0;
    let [a, b] = &m.dato.chequeos;
    if a.ajustado < 8 && b.ajustado < 8 && (a.ajustado < 5 || b.ajustado < 5) {
        let dado = (sortear(azar, 20)? + 1) as u8;
        let i = mitad_incidente(&m);
        let lugar = [c.origen, c.destino][i];
        let tipo = match dado {
            1..=7 => Incidente::Agotamiento,
            8..=13 => Incidente::Esguince,
            14..=17 => Incidente::RumboPerdido,
            _ if c.viajeros.len() > 1 => Incidente::Separacion,
            _ => Incidente::Enfermedad,
        };
        let actor = if tipo == Incidente::RumboPerdido {
            None
        } else {
            Some(afectado(c, tipo == Incidente::Separacion, azar)?)
        };
        match tipo {
            Incidente::Agotamiento => demora = 2,
            Incidente::Esguince => {
                if let Some(id) = actor {
                    conceder_estado(c, &mut m, id, "esguince")?;
                }
            }
            Incidente::RumboPerdido => {
                for &id in c.viajeros {
                    conceder_estado(c, &mut m, id, "cansado")?;
                }
            }
            Incidente::Enfermedad => {
                demora = 24;
                if let Some(id) = actor {
                    conceder_estado(c, &mut m, id, "cansado")?;
                }
            }
            _ => {}
        }
        if i == 0 {
            for op in &m.operaciones {
                if let OperacionMundo::AnadirCaracteristicaActor { caracteristica, .. } = op {
                    estados[1] = estados[1].max(if caracteristica.como_str() == "esguince" {
                        4
                    } else {
                        6
                    });
                }
            }
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
    }
    terminar_mitades(c, &mut m, estados, false)?;
    m.duracion = m.duracion.sumar(HorasViaje::horas(demora))?;
    Ok(m)
}
