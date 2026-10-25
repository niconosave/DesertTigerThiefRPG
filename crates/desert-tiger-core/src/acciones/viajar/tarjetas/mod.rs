pub(crate) mod andar;
pub(crate) mod cabalgar;
pub(crate) mod camello;
mod forrajear;
mod llegada;
pub(crate) mod navegar;
mod preparacion;
mod rutas;
mod sobrecarga;
use super::rutas as geografia;
use super::{
    registros::*,
    rutas::{Medio, lugar_tiene},
    tiempo::HorasViaje,
};
use crate::acciones::particulares::chequeos::{mas_apto, tiene};
use crate::acciones::{AccionParticular, Resultado, fallo, resolver_chequeo, sortear};
use crate::azar::FuenteAzar;
use crate::{IdActor, IdLugar, Mundo, OperacionMundo};

#[derive(Clone, Debug)]
pub(crate) struct Movimiento {
    pub dato: Desplazamiento,
    pub duracion: HorasViaje,
    pub operaciones: Vec<OperacionMundo>,
    pub viajeros: Vec<IdActor>,
    pub lugar: IdLugar,
    pub cierre: Option<CierreViaje>,
    pub caballos: u64,
    pub camellos: u64,
}
pub(crate) struct ContextoMovimiento<'a> {
    pub mundo: &'a Mundo,
    pub viajeros: &'a [IdActor],
    pub origen: IdLugar,
    pub destino: IdLugar,
    pub animales: u64,
    pub caballos: u64,
    pub camellos: u64,
    pub carga: [HorasViaje; 2],
    pub mes: u8,
    pub rumbo: Option<u8>,
}

pub(crate) fn comenzar(
    c: &ContextoMovimiento<'_>,
    medio: Medio,
    accion: AccionParticular,
    dificultad: i16,
    azar: &mut dyn FuenteAzar,
) -> Resultado<Movimiento> {
    let responsable = mas_apto(c.mundo, c.viajeros, accion, azar)?;
    let a = resolver_chequeo(c.mundo, responsable, accion, dificultad, azar)?;
    let b = resolver_chequeo(c.mundo, responsable, accion, dificultad, azar)?;
    let mut operaciones = Vec::new();
    for (l, prueba) in [(c.origen, &a), (c.destino, &b)] {
        if if medio == Medio::Barco {
            prueba.natural >= 18
        } else {
            prueba.ajustado >= 17
        } {
            for &actor in c.viajeros {
                operaciones.push(OperacionMundo::RegistrarConocimiento { actor, lugar: l });
            }
        }
    }
    Ok(Movimiento {
        dato: Desplazamiento {
            medio,
            origen: c.origen,
            destino: c.destino,
            chequeos: [a, b],
            mitades: Vec::new(),
            incidente: None,
            viento: None,
        },
        duracion: HorasViaje::CERO,
        operaciones,
        viajeros: c.viajeros.to_vec(),
        lugar: c.destino,
        cierre: None,
        caballos: c.caballos,
        camellos: c.camellos,
    })
}
pub(crate) fn mitad_incidente(m: &Movimiento) -> usize {
    usize::from(m.dato.chequeos[0].ajustado >= m.dato.chequeos[1].ajustado)
}
pub(crate) fn afectado(
    c: &ContextoMovimiento<'_>,
    excluir_dirigente: bool,
    azar: &mut dyn FuenteAzar,
) -> Resultado<IdActor> {
    let lista = if excluir_dirigente {
        &c.viajeros[1..]
    } else {
        c.viajeros
    };
    Ok(lista[sortear(azar, lista.len())?])
}
pub(crate) fn conceder_estado(
    c: &ContextoMovimiento<'_>,
    m: &mut Movimiento,
    actor: IdActor,
    estado: &str,
) -> Resultado<()> {
    if !tiene(c.mundo, actor, "esguince") && !tiene(c.mundo, actor, "cansado") {
        let caracteristica = crate::Caracteristica::nueva(estado.to_owned())
            .map_err(|_| fallo("Característica interna inválida"))?;
        m.operaciones
            .push(OperacionMundo::AnadirCaracteristicaActor {
                actor,
                caracteristica,
            });
    }
    Ok(())
}
pub(crate) fn terminar_mitades(
    c: &ContextoMovimiento<'_>,
    m: &mut Movimiento,
    estados: [u64; 2],
    a_pie_destino: bool,
) -> Resultado<()> {
    for (i, &lugar) in [c.origen, c.destino].iter().enumerate() {
        let medio = if i == 1 && a_pie_destino {
            Medio::Pie
        } else {
            m.dato.medio
        };
        let estado = if a_pie_destino { 0 } else { estados[i] };
        let horas = geografia::mitad(c.mundo, lugar, medio, c.animales, estado)?;
        let duracion = HorasViaje::horas(horas).sumar(c.carga[i])?;
        m.duracion = m.duracion.sumar(duracion)?;
        let base = match medio {
            Medio::Pie => 84,
            Medio::Caballo => 48,
            Medio::Camello => 60,
            Medio::Barco => 24,
        };
        let lento = medio != Medio::Pie
            && medio != Medio::Barco
            && ["montanoso", "pantanoso", "boscoso"]
                .iter()
                .any(|s| lugar_tiene(c.mundo, lugar, s));
        m.dato.mitades.push(MitadRecorrida {
            lugar,
            medio,
            horas_base: base,
            pequena: lugar_tiene(c.mundo, lugar, "pequeno"),
            vasta: lugar_tiene(c.mundo, lugar, "vasto"),
            terreno_lento: lento,
            animal_limitante: c.animales > 0 && medio == Medio::Caballo && !lento,
            estado_horas: if medio == Medio::Pie { estado } else { 0 },
            duracion: duracion.tiempo()?,
            demora_carga: c.carga[i].tiempo()?,
        });
    }
    Ok(())
}
pub(crate) fn aplicar_montura(
    c: &ContextoMovimiento<'_>,
    m: &mut Movimiento,
    dado: u8,
    azar: &mut dyn FuenteAzar,
) -> Resultado<()> {
    let i = mitad_incidente(m);
    let lugar = [c.origen, c.destino][i];
    let tipo = match dado {
        1..=6 => Incidente::Obstinada,
        7..=11 => Incidente::Detenida,
        12..=15 => Incidente::Asustada,
        16..=18 => Incidente::CrisisMontura,
        _ => Incidente::MuerteMontura,
    };
    let demora = match tipo {
        Incidente::Obstinada => 4,
        Incidente::Detenida => 8,
        Incidente::Asustada => 12,
        Incidente::CrisisMontura => 48,
        _ => 0,
    };
    let cantidad = if m.dato.medio == Medio::Caballo {
        m.caballos
    } else {
        m.camellos
    };
    let _montura = sortear(
        azar,
        usize::try_from(cantidad).map_err(|_| fallo("cantidad de monturas fuera de rango"))?,
    )?;
    let actor = afectado(c, false, azar)?;
    let recuperacion = if tipo == Incidente::CrisisMontura {
        Some((sortear(azar, 20)? + 1) as u8)
    } else {
        None
    };
    let muere = tipo == Incidente::MuerteMontura || recuperacion.is_some_and(|r| r < 8);
    if muere {
        if m.dato.medio == Medio::Caballo {
            m.caballos -= 1;
        } else {
            m.camellos -= 1;
        }
    }
    let restantes = if m.dato.medio == Medio::Caballo {
        m.caballos
    } else {
        m.camellos
    };
    m.dato.incidente = Some(IncidenteResuelto {
        tipo,
        dado,
        lugar,
        afectado: Some(actor),
        recuperacion,
        monturas_restantes: Some(restantes),
        demora: HorasViaje::horas(demora).tiempo()?,
    });
    terminar_mitades(
        c,
        m,
        [0, 0],
        muere && i == 0 && restantes < c.viajeros.len() as u64,
    )?;
    m.duracion = m.duracion.sumar(HorasViaje::horas(demora))?;
    Ok(())
}
