use crate::acciones::{Resultado, fallo, sortear};
use crate::azar::FuenteAzar;
use crate::{IdActor, Mundo};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccionParticular {
    Andar,
    Cabalgar,
    NavegarEnBarco,
    CalcularProvisiones,
    Forajear,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModificadorAplicado {
    pub caracteristica: &'static str,
    pub bono: i16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResultadoChequeo {
    pub responsable: IdActor,
    pub accion: AccionParticular,
    pub dificultad: i16,
    pub natural: u8,
    pub modificadores: Vec<ModificadorAplicado>,
    pub ajustado: i16,
    pub exito: bool,
}

pub(crate) fn tiene(mundo: &Mundo, actor: IdActor, clave: &str) -> bool {
    mundo
        .actor(actor)
        .is_some_and(|a| a.caracteristicas().any(|c| c.como_str() == clave))
}

pub fn evaluar_capacidad(
    mundo: &Mundo,
    actor: IdActor,
    accion: AccionParticular,
) -> Resultado<Vec<ModificadorAplicado>> {
    mundo
        .actor(actor)
        .ok_or_else(|| fallo("responsable inexistente"))?;
    let lista: &[(&str, i16)] = match accion {
        AccionParticular::Andar => &[("vagabundo", 2)],
        AccionParticular::Cabalgar => &[("jinete", 2)],
        AccionParticular::NavegarEnBarco => &[("navegante", 4)],
        AccionParticular::CalcularProvisiones => &[("administrador", 2), ("expedicionario", 2)],
        AccionParticular::Forajear => &[("expedicionario", 2), ("herbalista", 2)],
    };
    Ok(lista
        .iter()
        .filter(|(c, _)| tiene(mundo, actor, c))
        .map(|&(caracteristica, bono)| ModificadorAplicado {
            caracteristica,
            bono,
        })
        .collect())
}

pub fn resolver_chequeo(
    mundo: &Mundo,
    responsable: IdActor,
    accion: AccionParticular,
    dificultad: i16,
    azar: &mut dyn FuenteAzar,
) -> Resultado<ResultadoChequeo> {
    let modificadores = evaluar_capacidad(mundo, responsable, accion)?;
    let natural = u8::try_from(sortear(azar, 20)? + 1).map_err(|_| fallo("d20 inválido"))?;
    let ajustado = (i16::from(natural) + modificadores.iter().map(|m| m.bono).sum::<i16>()).min(20);
    Ok(ResultadoChequeo {
        responsable,
        accion,
        dificultad,
        natural,
        modificadores,
        ajustado,
        exito: ajustado >= dificultad,
    })
}

pub(crate) fn mas_apto(
    mundo: &Mundo,
    actores: &[IdActor],
    accion: AccionParticular,
    azar: &mut dyn FuenteAzar,
) -> Resultado<IdActor> {
    let mut candidatos = Vec::new();
    let mut maximo = i16::MIN;
    for &id in actores {
        let valor = evaluar_capacidad(mundo, id, accion)?
            .iter()
            .map(|m| m.bono)
            .sum();
        if valor > maximo {
            maximo = valor;
            candidatos.clear();
        }
        if valor == maximo {
            candidatos.push(id);
        }
    }
    candidatos
        .get(sortear(azar, candidatos.len())?)
        .copied()
        .ok_or_else(|| fallo("sin responsable"))
}
