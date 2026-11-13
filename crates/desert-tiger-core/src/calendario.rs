use std::error::Error;
use std::fmt::{self, Display, Formatter};

const ANIO_MINIMO: u16 = 1;
const ANIO_MAXIMO: u16 = 9999;

pub const PARTES_POR_HORA: u128 = 100_000_000;
pub const PARTES_POR_DIA: u128 = 24 * PARTES_POR_HORA;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FraccionDia(u32);

impl FraccionDia {
    pub const CERO: Self = Self(0);
    pub fn nueva(partes: u32) -> Result<Self, ErrorCalendario> {
        if u128::from(partes) >= PARTES_POR_DIA {
            return Err(ErrorCalendario::Desbordamiento);
        }
        Ok(Self(partes))
    }
    pub fn partes(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TiempoAccion {
    pub duracion: Duracion,
    pub fraccion: FraccionDia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanteAccion {
    pub fecha: Fecha,
    pub fraccion: FraccionDia,
}

pub fn proyectar_tiempo(
    inicio: InstanteAccion,
    tiempo: TiempoAccion,
) -> Result<InstanteAccion, ErrorCalendario> {
    let partes = u64::from(inicio.fraccion.partes()) + u64::from(tiempo.fraccion.partes());
    let dias = u32::try_from(u128::from(partes) / PARTES_POR_DIA)
        .map_err(|_| ErrorCalendario::Desbordamiento)?;
    let fecha = inicio
        .fecha
        .sumar(tiempo.duracion)?
        .sumar(Duracion::desde_dias(dias))?;
    let resto = u32::try_from(u128::from(partes) % PARTES_POR_DIA)
        .map_err(|_| ErrorCalendario::Desbordamiento)?;
    Ok(InstanteAccion {
        fecha,
        fraccion: FraccionDia::nueva(resto)?,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre,
}

impl Mes {
    pub fn numero(self) -> u8 {
        match self {
            Self::Enero => 1,
            Self::Febrero => 2,
            Self::Marzo => 3,
            Self::Abril => 4,
            Self::Mayo => 5,
            Self::Junio => 6,
            Self::Julio => 7,
            Self::Agosto => 8,
            Self::Septiembre => 9,
            Self::Octubre => 10,
            Self::Noviembre => 11,
            Self::Diciembre => 12,
        }
    }

    pub fn desde_numero(numero: u8) -> Result<Self, ErrorCalendario> {
        match numero {
            1 => Ok(Self::Enero),
            2 => Ok(Self::Febrero),
            3 => Ok(Self::Marzo),
            4 => Ok(Self::Abril),
            5 => Ok(Self::Mayo),
            6 => Ok(Self::Junio),
            7 => Ok(Self::Julio),
            8 => Ok(Self::Agosto),
            9 => Ok(Self::Septiembre),
            10 => Ok(Self::Octubre),
            11 => Ok(Self::Noviembre),
            12 => Ok(Self::Diciembre),
            mes => Err(ErrorCalendario::MesFueraDeRango { mes }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fecha {
    anio: u16,
    mes: Mes,
    dia: u8,
}

impl Fecha {
    pub fn nueva(anio: u16, mes: Mes, dia: u8) -> Result<Self, ErrorCalendario> {
        if !(ANIO_MINIMO..=ANIO_MAXIMO).contains(&anio) {
            return Err(ErrorCalendario::AnioFueraDeRango { anio });
        }

        let maximo = dias_del_mes(anio, mes);
        if !(1..=maximo).contains(&dia) {
            return Err(ErrorCalendario::DiaFueraDeRango {
                anio,
                mes,
                dia,
                maximo,
            });
        }

        Ok(Self { anio, mes, dia })
    }

    pub fn anio(self) -> u16 {
        self.anio
    }

    pub fn mes(self) -> Mes {
        self.mes
    }

    pub fn dia(self) -> u8 {
        self.dia
    }

    pub fn sumar(self, duracion: Duracion) -> Result<Self, ErrorCalendario> {
        let mes_absoluto = u64::from(self.anio - 1)
            .checked_mul(12)
            .and_then(|valor| valor.checked_add(u64::from(self.mes.numero() - 1)))
            .and_then(|valor| valor.checked_add(u64::from(duracion.meses)))
            .ok_or(ErrorCalendario::Desbordamiento)?;
        let ultimo_mes = u64::from(ANIO_MAXIMO) * 12 - 1;
        if mes_absoluto > ultimo_mes {
            return Err(ErrorCalendario::Desbordamiento);
        }

        let anio_tras_meses =
            u16::try_from(mes_absoluto / 12 + 1).map_err(|_| ErrorCalendario::Desbordamiento)?;
        let numero_mes =
            u8::try_from(mes_absoluto % 12 + 1).map_err(|_| ErrorCalendario::Desbordamiento)?;
        let mes_tras_meses = Mes::desde_numero(numero_mes)?;
        let dia_ajustado = self.dia.min(dias_del_mes(anio_tras_meses, mes_tras_meses));
        let fecha_tras_meses = Self::nueva(anio_tras_meses, mes_tras_meses, dia_ajustado)?;

        if duracion.dias == 0 {
            return Ok(fecha_tras_meses);
        }

        let ordinal = fecha_tras_meses.ordinal();
        let destino = ordinal
            .checked_add(u64::from(duracion.dias))
            .ok_or(ErrorCalendario::Desbordamiento)?;
        if destino > Self::ultima_fecha().ordinal() {
            return Err(ErrorCalendario::Desbordamiento);
        }

        Ok(Self::desde_ordinal(destino))
    }

    /// Días completos desde el inicio del calendario (1 de enero del año 1).
    pub fn ordinal(self) -> u64 {
        let mut dias = dias_antes_del_anio(self.anio);
        for numero in 1..self.mes.numero() {
            let mes = match Mes::desde_numero(numero) {
                Ok(mes) => mes,
                Err(_) => unreachable!("el rango 1..mes siempre contiene meses válidos"),
            };
            dias += u64::from(dias_del_mes(self.anio, mes));
        }
        dias + u64::from(self.dia - 1)
    }

    fn desde_ordinal(ordinal: u64) -> Self {
        let mut inferior = ANIO_MINIMO;
        let mut superior = ANIO_MAXIMO;
        while inferior < superior {
            let medio = inferior + (superior - inferior).div_ceil(2);
            if dias_antes_del_anio(medio) <= ordinal {
                inferior = medio;
            } else {
                superior = medio - 1;
            }
        }

        let anio = inferior;
        let mut restante = ordinal - dias_antes_del_anio(anio);
        for numero in 1..=12 {
            let mes = match Mes::desde_numero(numero) {
                Ok(mes) => mes,
                Err(_) => unreachable!("el rango 1..=12 solo contiene meses válidos"),
            };
            let longitud = u64::from(dias_del_mes(anio, mes));
            if restante < longitud {
                let dia = u8::try_from(restante + 1)
                    .unwrap_or_else(|_| unreachable!("un día de mes siempre cabe en u8"));
                return Self { anio, mes, dia };
            }
            restante -= longitud;
        }

        unreachable!("el ordinal validado siempre resuelve a una fecha")
    }

    fn ultima_fecha() -> Self {
        Self {
            anio: ANIO_MAXIMO,
            mes: Mes::Diciembre,
            dia: 31,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duracion {
    meses: u32,
    dias: u32,
}

impl Duracion {
    pub const CERO: Self = Self { meses: 0, dias: 0 };

    pub const fn nueva(meses: u32, dias: u32) -> Self {
        Self { meses, dias }
    }

    pub const fn desde_dias(dias: u32) -> Self {
        Self { meses: 0, dias }
    }

    pub const fn desde_meses(meses: u32) -> Self {
        Self { meses, dias: 0 }
    }

    pub const fn meses(self) -> u32 {
        self.meses
    }

    pub const fn dias(self) -> u32 {
        self.dias
    }

    pub const fn es_cero(self) -> bool {
        self.meses == 0 && self.dias == 0
    }

    pub fn checked_add(self, otra: Self) -> Result<Self, ErrorCalendario> {
        let meses = self
            .meses
            .checked_add(otra.meses)
            .ok_or(ErrorCalendario::Desbordamiento)?;
        let dias = self
            .dias
            .checked_add(otra.dias)
            .ok_or(ErrorCalendario::Desbordamiento)?;
        Ok(Self { meses, dias })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCalendario {
    AnioFueraDeRango {
        anio: u16,
    },
    MesFueraDeRango {
        mes: u8,
    },
    DiaFueraDeRango {
        anio: u16,
        mes: Mes,
        dia: u8,
        maximo: u8,
    },
    Desbordamiento,
}

impl Display for ErrorCalendario {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::AnioFueraDeRango { anio } => write!(formatter, "año fuera de rango: {anio}"),
            Self::MesFueraDeRango { mes } => write!(formatter, "mes fuera de rango: {mes}"),
            Self::DiaFueraDeRango {
                anio,
                mes,
                dia,
                maximo,
            } => write!(
                formatter,
                "día fuera de rango: {dia} para {mes:?}/{anio}; máximo {maximo}"
            ),
            Self::Desbordamiento => formatter.write_str("desbordamiento de calendario"),
        }
    }
}

impl Error for ErrorCalendario {}

fn es_bisiesto(anio: u16) -> bool {
    anio.is_multiple_of(4) && (!anio.is_multiple_of(100) || anio.is_multiple_of(400))
}

fn dias_del_mes(anio: u16, mes: Mes) -> u8 {
    match mes {
        Mes::Enero
        | Mes::Marzo
        | Mes::Mayo
        | Mes::Julio
        | Mes::Agosto
        | Mes::Octubre
        | Mes::Diciembre => 31,
        Mes::Abril | Mes::Junio | Mes::Septiembre | Mes::Noviembre => 30,
        Mes::Febrero if es_bisiesto(anio) => 29,
        Mes::Febrero => 28,
    }
}

fn dias_antes_del_anio(anio: u16) -> u64 {
    let previos = u64::from(anio - 1);
    previos * 365 + previos / 4 - previos / 100 + previos / 400
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn convierte_los_doce_meses() {
        for numero in 1..=12 {
            let mes = Mes::desde_numero(numero).expect("mes válido");
            assert_eq!(mes.numero(), numero);
        }
        assert_eq!(
            Mes::desde_numero(0),
            Err(ErrorCalendario::MesFueraDeRango { mes: 0 })
        );
        assert_eq!(
            Mes::desde_numero(13),
            Err(ErrorCalendario::MesFueraDeRango { mes: 13 })
        );
    }

    #[test]
    fn valida_anios_dias_y_bisiestos() {
        assert!(matches!(
            Fecha::nueva(0, Mes::Enero, 1),
            Err(ErrorCalendario::AnioFueraDeRango { anio: 0 })
        ));
        assert!(matches!(
            Fecha::nueva(10_000, Mes::Enero, 1),
            Err(ErrorCalendario::AnioFueraDeRango { anio: 10_000 })
        ));
        assert!(Fecha::nueva(1900, Mes::Febrero, 28).is_ok());
        assert!(Fecha::nueva(1900, Mes::Febrero, 29).is_err());
        assert!(Fecha::nueva(2000, Mes::Febrero, 29).is_ok());
        assert!(Fecha::nueva(2024, Mes::Febrero, 29).is_ok());
        assert!(Fecha::nueva(2025, Mes::Abril, 30).is_ok());
        assert!(Fecha::nueva(2025, Mes::Abril, 0).is_err());
        assert!(Fecha::nueva(2025, Mes::Abril, 31).is_err());

        let maximos = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for (indice, maximo) in maximos.into_iter().enumerate() {
            let numero = u8::try_from(indice + 1).expect("índice de mes válido");
            let mes = Mes::desde_numero(numero).expect("mes válido");
            assert!(Fecha::nueva(2025, mes, maximo).is_ok());
            assert!(Fecha::nueva(2025, mes, maximo + 1).is_err());
        }
    }

    #[test]
    fn suma_meses_en_una_sola_operacion_y_luego_dias() {
        let enero = Fecha::nueva(2025, Mes::Enero, 31).expect("fecha válida");
        assert_eq!(
            enero.sumar(Duracion::desde_meses(1)),
            Fecha::nueva(2025, Mes::Febrero, 28)
        );
        assert_eq!(
            enero.sumar(Duracion::desde_meses(2)),
            Fecha::nueva(2025, Mes::Marzo, 31)
        );
        assert_eq!(
            enero.sumar(Duracion::nueva(1, 1)),
            Fecha::nueva(2025, Mes::Marzo, 1)
        );
        let bisiesto = Fecha::nueva(2024, Mes::Enero, 31).expect("fecha válida");
        assert_eq!(
            bisiesto.sumar(Duracion::nueva(1, 1)),
            Fecha::nueva(2024, Mes::Marzo, 1)
        );
        assert_eq!(
            enero.sumar(Duracion::nueva(1, 30)),
            Fecha::nueva(2025, Mes::Marzo, 30)
        );
        assert_eq!(
            Fecha::nueva(2025, Mes::Noviembre, 30)
                .expect("fecha válida")
                .sumar(Duracion::desde_meses(2)),
            Fecha::nueva(2026, Mes::Enero, 30)
        );
    }

    #[test]
    fn suma_dias_sin_recorrer_un_bucle_por_dia() {
        let inicio = Fecha::nueva(2025, Mes::Diciembre, 31).expect("fecha válida");
        assert_eq!(
            inicio.sumar(Duracion::desde_dias(1)),
            Fecha::nueva(2026, Mes::Enero, 1)
        );
        let gran_salto = Fecha::nueva(1, Mes::Enero, 1).expect("fecha válida");
        assert_eq!(
            gran_salto.sumar(Duracion::desde_dias(3_652_058)),
            Fecha::nueva(9999, Mes::Diciembre, 31)
        );
        let limite = Fecha::nueva(9999, Mes::Diciembre, 31).expect("fecha válida");
        assert_eq!(
            limite.sumar(Duracion::desde_dias(1)),
            Err(ErrorCalendario::Desbordamiento)
        );
    }

    #[test]
    fn conserva_componentes_y_detecta_desbordamientos_de_duracion() {
        let duracion = Duracion::nueva(3, 40);
        assert_eq!(duracion.meses(), 3);
        assert_eq!(duracion.dias(), 40);
        assert!(!duracion.es_cero());
        assert!(Duracion::CERO.es_cero());
        assert_eq!(
            Duracion::nueva(u32::MAX, 0).checked_add(Duracion::desde_meses(1)),
            Err(ErrorCalendario::Desbordamiento)
        );
        assert_eq!(
            Duracion::nueva(0, u32::MAX).checked_add(Duracion::desde_dias(1)),
            Err(ErrorCalendario::Desbordamiento)
        );
        assert_eq!(
            Duracion::desde_dias(2).checked_add(Duracion::desde_meses(1)),
            Ok(Duracion::nueva(1, 2))
        );
    }

    #[test]
    fn rechaza_desbordamiento_por_meses_y_cero_conserva_fecha() {
        let fecha = Fecha::nueva(9999, Mes::Diciembre, 1).expect("fecha válida");
        assert_eq!(
            fecha.sumar(Duracion::desde_meses(1)),
            Err(ErrorCalendario::Desbordamiento)
        );
        assert_eq!(fecha.sumar(Duracion::CERO), Ok(fecha));
    }
}
