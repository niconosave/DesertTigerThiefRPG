use std::num::NonZeroUsize;

use crate::azar::{ErrorFuenteAzar, FuenteAzar};
use crate::{CambioConfirmado, Duracion, IdActor, IdLugar, OperacionMundo};

const CARAS_D20: usize = 20;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BandaResolucion {
    Holgada,
    Lograda,
    Costosa,
    Fallida,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PruebaD20 {
    pub dificultad: i16,
    pub modificador: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Evaluacion {
    Imposible,
    Determinista(BandaResolucion),
    Incierta(PruebaD20),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Resolucion {
    Imposible,
    Determinista {
        banda: BandaResolucion,
    },
    Incierta {
        cara: u8,
        margen: i16,
        banda: BandaResolucion,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccionParticular {
    Orientarse,
    Marchar,
    Escalar,
    Descender,
    Cabalgar,
    GuiarCaravana,
    BuscarAgua,
    TratarAgua,
    Cazar,
    Pescar,
    Recolectar,
    MontarCampamento,
    Descansar,
    Remar,
    Navegar,
    GobernarTimon,
    Fondear,
    Atracar,
    Achicar,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EjeCausal {
    Base,
    Entorno,
    Via,
    Carga,
    Capacidad,
    Conocimiento,
    Estado,
    Ritmo,
    Medio,
    Grupo,
    Recurso,
    Calidad,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcedenciaFactor {
    Actor(IdActor),
    Lugar(IdLugar),
    Grupo,
    Contexto,
    Regla,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibilidadFactor {
    Visible,
    Oculta,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactorAplicado {
    pub clave: &'static str,
    pub explicacion_publica: &'static str,
    pub eje: EjeCausal,
    pub procedencia: ProcedenciaFactor,
    pub ajuste: i16,
    pub visibilidad: VisibilidadFactor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatoResolucion<T> {
    Conocido(T),
    Oculto,
    NoCorresponde,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModoResolucion {
    Imposible,
    Determinista,
    PruebaD20 {
        dificultad: DatoResolucion<i16>,
        modificador: DatoResolucion<i16>,
        cara: DatoResolucion<u8>,
        margen: DatoResolucion<i16>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolucionAplicada {
    pub sujeto: Option<IdActor>,
    pub modo: ModoResolucion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextoTraza {
    Trayecto { origen: IdLugar, destino: IdLugar },
    Lugar { lugar: IdLugar },
    SinLugar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CambioTransitorio {
    Avance { confirmado: bool },
    Reservas { agua: i64, alimento: i64 },
    Calidad { clave: &'static str },
    EstadoMedio { anterior: u8, posterior: u8 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrazaAccionParticular {
    pub accion: AccionParticular,
    pub responsable: Option<IdActor>,
    pub ayudante: Option<IdActor>,
    pub contexto: ContextoTraza,
    pub factores: Vec<FactorAplicado>,
    pub resoluciones: Vec<ResolucionAplicada>,
    pub banda: Option<BandaResolucion>,
    pub desenlace: &'static str,
    pub duracion: Duracion,
    pub cambios_transitorios: Vec<CambioTransitorio>,
    pub operaciones_solicitadas: Vec<OperacionMundo>,
    pub operaciones_confirmadas: Option<Vec<CambioConfirmado>>,
}

impl TrazaAccionParticular {
    pub fn factores_visibles(&self) -> impl Iterator<Item = &FactorAplicado> {
        self.factores
            .iter()
            .filter(|factor| factor.visibilidad == VisibilidadFactor::Visible)
    }

    pub fn confirmar_operaciones(&mut self, cambios: &[CambioConfirmado]) {
        self.operaciones_confirmadas = Some(cambios.to_vec());
    }
}

pub(crate) fn modo_resuelto(
    evaluacion: Evaluacion,
    resolucion: Resolucion,
    contiene_factor_oculto: bool,
) -> ModoResolucion {
    match (evaluacion, resolucion) {
        (Evaluacion::Imposible, Resolucion::Imposible) => ModoResolucion::Imposible,
        (Evaluacion::Determinista(_), Resolucion::Determinista { .. }) => {
            ModoResolucion::Determinista
        }
        (Evaluacion::Incierta(prueba), Resolucion::Incierta { cara, margen, .. }) => {
            let visible = |valor| {
                if contiene_factor_oculto {
                    DatoResolucion::Oculto
                } else {
                    DatoResolucion::Conocido(valor)
                }
            };
            ModoResolucion::PruebaD20 {
                dificultad: visible(prueba.dificultad),
                modificador: visible(prueba.modificador),
                cara: DatoResolucion::Conocido(cara),
                margen: visible(margen),
            }
        }
        _ => unreachable!("la resolución debe corresponder a su evaluación"),
    }
}

pub(crate) fn factor(
    clave: &'static str,
    explicacion_publica: &'static str,
    eje: EjeCausal,
    procedencia: ProcedenciaFactor,
    ajuste: i16,
) -> FactorAplicado {
    FactorAplicado {
        clave,
        explicacion_publica,
        eje,
        procedencia,
        ajuste,
        visibilidad: VisibilidadFactor::Visible,
    }
}

impl Resolucion {
    pub fn banda(self) -> Option<BandaResolucion> {
        match self {
            Self::Imposible => None,
            Self::Determinista { banda } | Self::Incierta { banda, .. } => Some(banda),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ViabilidadPronostico {
    Posible,
    Imposible,
    Incierta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PronosticoParticular {
    pub viabilidad: ViabilidadPronostico,
    pub duracion_minima: Duracion,
    pub duracion_maxima: Duracion,
    pub exposicion_conocida: u32,
    pub incertidumbre: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ErrorResolucion {
    ContextoInvalido,
    FuenteAzar(ErrorFuenteAzar),
    IndiceFueraDeRango {
        indice: usize,
        limite_exclusivo: NonZeroUsize,
    },
    Desbordamiento,
}

pub(crate) fn resolver(
    evaluacion: Evaluacion,
    azar: &mut dyn FuenteAzar,
) -> Result<Resolucion, ErrorResolucion> {
    match evaluacion {
        Evaluacion::Imposible => Ok(Resolucion::Imposible),
        Evaluacion::Determinista(banda) => Ok(Resolucion::Determinista { banda }),
        Evaluacion::Incierta(prueba) => {
            let limite = NonZeroUsize::new(CARAS_D20).expect("un d20 tiene caras");
            let indice = azar
                .elegir_indice(limite)
                .map_err(ErrorResolucion::FuenteAzar)?;
            if indice >= limite.get() {
                return Err(ErrorResolucion::IndiceFueraDeRango {
                    indice,
                    limite_exclusivo: limite,
                });
            }
            let cara = u8::try_from(indice + 1).map_err(|_| ErrorResolucion::Desbordamiento)?;
            let margen = margen(prueba, i16::from(cara))?;
            Ok(Resolucion::Incierta {
                cara,
                margen,
                banda: banda_para_margen(margen),
            })
        }
    }
}

pub(crate) fn pronosticar(
    evaluacion: Evaluacion,
    duracion: Duracion,
    incertidumbre: u32,
) -> Result<PronosticoParticular, ErrorResolucion> {
    let (viabilidad, exposicion_conocida, duracion_minima, duracion_maxima) = match evaluacion {
        Evaluacion::Imposible => (
            ViabilidadPronostico::Imposible,
            0,
            Duracion::CERO,
            Duracion::CERO,
        ),
        Evaluacion::Determinista(_) => (ViabilidadPronostico::Posible, 0, duracion, duracion),
        Evaluacion::Incierta(prueba) => {
            let mut exposicion = 0_u32;
            for cara in 1_i16..=20 {
                exposicion = exposicion
                    .checked_add(match banda_para_margen(margen(prueba, cara)?) {
                        BandaResolucion::Costosa => 1,
                        BandaResolucion::Fallida => 2,
                        BandaResolucion::Holgada | BandaResolucion::Lograda => 0,
                    })
                    .ok_or(ErrorResolucion::Desbordamiento)?;
            }
            (
                ViabilidadPronostico::Incierta,
                exposicion,
                duracion,
                duracion,
            )
        }
    };
    Ok(PronosticoParticular {
        viabilidad,
        duracion_minima,
        duracion_maxima,
        exposicion_conocida,
        incertidumbre,
    })
}

fn margen(prueba: PruebaD20, cara: i16) -> Result<i16, ErrorResolucion> {
    cara.checked_add(prueba.modificador)
        .and_then(|total| total.checked_sub(prueba.dificultad))
        .ok_or(ErrorResolucion::Desbordamiento)
}

fn banda_para_margen(margen: i16) -> BandaResolucion {
    match margen {
        5.. => BandaResolucion::Holgada,
        0..=4 => BandaResolucion::Lograda,
        -4..=-1 => BandaResolucion::Costosa,
        ..=-5 => BandaResolucion::Fallida,
    }
}

#[cfg(test)]
mod pruebas {
    use std::collections::VecDeque;

    use super::*;

    #[derive(Default)]
    struct AzarContado {
        valores: VecDeque<Result<usize, ErrorFuenteAzar>>,
        limites: Vec<usize>,
    }

    impl AzarContado {
        fn con(valores: impl IntoIterator<Item = usize>) -> Self {
            Self {
                valores: valores.into_iter().map(Ok).collect(),
                limites: Vec::new(),
            }
        }
    }

    impl FuenteAzar for AzarContado {
        fn elegir_indice(
            &mut self,
            limite_exclusivo: NonZeroUsize,
        ) -> Result<usize, ErrorFuenteAzar> {
            self.limites.push(limite_exclusivo.get());
            self.valores.pop_front().unwrap_or(Err(ErrorFuenteAzar))
        }
    }

    #[test]
    fn imposible_y_determinista_no_consumen_azar() {
        let mut azar = AzarContado::default();
        assert_eq!(
            resolver(Evaluacion::Imposible, &mut azar),
            Ok(Resolucion::Imposible)
        );
        assert_eq!(
            resolver(
                Evaluacion::Determinista(BandaResolucion::Holgada),
                &mut azar
            ),
            Ok(Resolucion::Determinista {
                banda: BandaResolucion::Holgada
            })
        );
        assert!(azar.limites.is_empty());
    }

    #[test]
    fn incierta_consume_un_d20_y_cubre_las_cuatro_bandas() {
        for (indice, esperado) in [
            (14, BandaResolucion::Holgada),
            (9, BandaResolucion::Lograda),
            (5, BandaResolucion::Costosa),
            (0, BandaResolucion::Fallida),
        ] {
            let mut azar = AzarContado::con([indice]);
            let resultado = resolver(
                Evaluacion::Incierta(PruebaD20 {
                    dificultad: 12,
                    modificador: 2,
                }),
                &mut azar,
            )
            .expect("resolución válida");
            assert_eq!(resultado.banda(), Some(esperado));
            assert_eq!(azar.limites, vec![20]);
        }
    }

    #[test]
    fn pronosticar_es_puro_y_cuenta_exposicion_sin_tirar() {
        let evaluacion = Evaluacion::Incierta(PruebaD20 {
            dificultad: 12,
            modificador: 2,
        });
        let pronostico =
            pronosticar(evaluacion, Duracion::desde_dias(1), 0).expect("pronóstico válido");
        assert_eq!(pronostico.viabilidad, ViabilidadPronostico::Incierta);
        assert_eq!(pronostico.duracion_minima, Duracion::desde_dias(1));
        assert_eq!(pronostico.duracion_maxima, Duracion::desde_dias(1));
        assert_eq!(pronostico.exposicion_conocida, 14);
        assert_eq!(pronostico.incertidumbre, 0);
    }

    #[test]
    fn oculta_totales_si_un_factor_no_es_visible() {
        let evaluacion = Evaluacion::Incierta(PruebaD20 {
            dificultad: 15,
            modificador: 4,
        });
        let mut azar = AzarContado::con([9]);
        let resuelta = resolver(evaluacion, &mut azar).expect("resolución válida");
        assert_eq!(
            modo_resuelto(evaluacion, resuelta, true),
            ModoResolucion::PruebaD20 {
                dificultad: DatoResolucion::Oculto,
                modificador: DatoResolucion::Oculto,
                cara: DatoResolucion::Conocido(10),
                margen: DatoResolucion::Oculto,
            }
        );
    }
}
