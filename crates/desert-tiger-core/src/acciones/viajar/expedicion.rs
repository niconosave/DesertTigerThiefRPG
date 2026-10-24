use std::collections::BTreeSet;

use super::proyecto::ProyectoViaje;
use crate::{Duracion, Fecha, IdActor, IdLugar};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PreparacionViaje {
    Ligera,
    Suficiente,
    Cargada,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RitmoViaje {
    Prudente,
    Normal,
    Forzado,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MedioTerrestre {
    APie,
    Camellos,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MedioAcuatico {
    Ninguno,
    Bote,
    Barco,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum EstadoEmbarcacion {
    Estable,
    AguaEmbarcada,
    Anegada,
    NaufragioInminente,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ReservasExpedicion {
    pub agua: u32,
    pub alimento: u32,
}

#[derive(Debug)]
pub(crate) struct Expedicion {
    pub dirigente: IdActor,
    pub viajeros: Vec<IdActor>,
    pub destino_final: IdLugar,
    pub fecha_inicio: Fecha,
    pub ruta: Vec<IdLugar>,
    pub posicion: usize,
    pub duracion: Duracion,
    pub preparacion: PreparacionViaje,
    pub ritmo: RitmoViaje,
    pub medio_terrestre: MedioTerrestre,
    pub medio_acuatico: MedioAcuatico,
    pub estado_embarcacion: EstadoEmbarcacion,
    pub reservas: ReservasExpedicion,
    pub proyecto: Option<Box<ProyectoViaje>>,
    siguiente_momento: u64,
    intentos_supervivencia: BTreeSet<(IdLugar, u8)>,
}

impl Expedicion {
    pub fn nueva(
        dirigente: IdActor,
        viajeros: Vec<IdActor>,
        destino_final: IdLugar,
        ruta: Vec<IdLugar>,
        fecha_inicio: Fecha,
    ) -> Option<Self> {
        if viajeros.first() != Some(&dirigente)
            || ruta.len() < 2
            || ruta.last() != Some(&destino_final)
        {
            return None;
        }
        Some(Self {
            dirigente,
            viajeros,
            destino_final,
            fecha_inicio,
            ruta,
            posicion: 0,
            duracion: Duracion::CERO,
            preparacion: PreparacionViaje::Ligera,
            ritmo: RitmoViaje::Normal,
            medio_terrestre: MedioTerrestre::APie,
            medio_acuatico: MedioAcuatico::Ninguno,
            estado_embarcacion: EstadoEmbarcacion::Estable,
            reservas: ReservasExpedicion::default(),
            proyecto: None,
            siguiente_momento: 0,
            intentos_supervivencia: BTreeSet::new(),
        })
    }

    pub fn preparar(
        &mut self,
        preparacion: PreparacionViaje,
        jornadas_agua: u32,
        jornadas_alimento: u32,
    ) -> Option<()> {
        let viajeros = u32::try_from(self.viajeros.len()).ok()?;
        self.preparacion = preparacion;
        self.reservas = ReservasExpedicion {
            agua: jornadas_agua.checked_mul(viajeros)?,
            alimento: jornadas_alimento.checked_mul(viajeros)?,
        };
        Some(())
    }

    pub fn fijar_ritmo(&mut self, ritmo: RitmoViaje) {
        self.ritmo = ritmo;
    }

    pub fn fijar_medio_terrestre(&mut self, medio: MedioTerrestre) {
        self.medio_terrestre = medio;
    }

    pub fn fijar_medio_acuatico(&mut self, medio: MedioAcuatico) {
        self.medio_acuatico = medio;
        self.estado_embarcacion = EstadoEmbarcacion::Estable;
    }

    pub fn fijar_estado_embarcacion(&mut self, estado: EstadoEmbarcacion) {
        self.estado_embarcacion = estado;
    }

    pub fn fijar_proyecto(&mut self, proyecto: ProyectoViaje) {
        self.proyecto = Some(Box::new(proyecto));
    }

    pub fn siguiente_momento(&mut self) -> Option<crate::acciones::IdMomentoCausal> {
        let id = crate::acciones::IdMomentoCausal(self.siguiente_momento);
        self.siguiente_momento = self.siguiente_momento.checked_add(1)?;
        Some(id)
    }

    pub fn progreso(&self) -> Option<crate::acciones::ProgresoViaje> {
        Some(crate::acciones::ProgresoViaje {
            lugar_confirmado: self.actual()?,
            siguiente_lugar: self.siguiente(),
            destino_final: self.destino_final,
            duracion_transcurrida: self.duracion,
            fecha_proyectada: self.fecha_inicio.sumar(self.duracion).ok()?,
        })
    }

    pub fn consumir(
        &mut self,
        dias: u32,
        agua_extra: u32,
        alimento_extra: u32,
    ) -> Option<(u32, u32)> {
        let viajeros = u32::try_from(self.viajeros.len()).ok()?;
        let ordinario = dias.checked_mul(viajeros)?;
        let agua_necesaria = ordinario.checked_add(agua_extra)?;
        let falta_agua = agua_necesaria.saturating_sub(self.reservas.agua);
        let alimento_necesario = ordinario.checked_add(alimento_extra)?;
        let falta_alimento = alimento_necesario.saturating_sub(self.reservas.alimento);
        self.reservas.agua = self.reservas.agua.saturating_sub(agua_necesaria);
        self.reservas.alimento = self.reservas.alimento.saturating_sub(alimento_necesario);
        Some((falta_agua, falta_alimento))
    }

    pub fn agregar_agua(&mut self, raciones: u32) -> Option<()> {
        self.reservas.agua = self.reservas.agua.checked_add(raciones)?;
        Some(())
    }

    pub fn agregar_alimento(&mut self, raciones: u32) -> Option<()> {
        self.reservas.alimento = self.reservas.alimento.checked_add(raciones)?;
        Some(())
    }

    pub fn jornadas_completas_restantes(&self) -> u32 {
        let viajeros = u32::try_from(self.viajeros.len()).unwrap_or(u32::MAX);
        if viajeros == 0 {
            return 0;
        }
        (self.reservas.agua / viajeros).min(self.reservas.alimento / viajeros)
    }

    pub fn registrar_intento_supervivencia(&mut self, lugar: IdLugar, clase: u8) -> bool {
        self.intentos_supervivencia.insert((lugar, clase))
    }

    pub fn actual(&self) -> Option<IdLugar> {
        self.ruta.get(self.posicion).copied()
    }

    pub fn siguiente(&self) -> Option<IdLugar> {
        self.ruta.get(self.posicion.checked_add(1)?).copied()
    }

    pub fn confirmar_avance(&mut self, incremento: Duracion) -> Option<()> {
        self.posicion = self.posicion.checked_add(1)?;
        self.duracion = self.duracion.checked_add(incremento).ok()?;
        if let Some(proyecto) = self.proyecto.as_mut() {
            proyecto.ultimo_lugar_confirmado = *self.ruta.get(self.posicion)?;
            proyecto.duracion = self.duracion;
            proyecto.subobjetivo = self
                .posicion
                .checked_add(1)
                .and_then(|posicion| self.ruta.get(posicion))
                .copied()
                .map(super::proyecto::SubobjetivoViaje::Alcanzar);
        }
        Some(())
    }

    pub fn acumular_sin_avance(&mut self, incremento: Duracion) -> Option<()> {
        self.duracion = self.duracion.checked_add(incremento).ok()?;
        if let Some(proyecto) = self.proyecto.as_mut() {
            proyecto.duracion = self.duracion;
        }
        Some(())
    }

    pub fn llego(&self) -> bool {
        self.actual() == Some(self.destino_final)
    }

    pub fn reemplazar_ruta_desde_actual(&mut self, ruta: Vec<IdLugar>) -> Option<()> {
        if ruta.len() < 2
            || ruta.first().copied() != self.actual()
            || ruta.last() != Some(&self.destino_final)
        {
            return None;
        }
        self.ruta = ruta;
        self.posicion = 0;
        Some(())
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::Mes;

    fn fecha() -> Fecha {
        Fecha::nueva(2026, Mes::Octubre, 20).expect("fecha válida")
    }

    fn actor(indice: usize) -> IdActor {
        IdActor::desde_posicion(indice).expect("ID válido")
    }

    fn lugar(indice: usize) -> IdLugar {
        IdLugar::desde_posicion(indice).expect("ID válido")
    }

    #[test]
    fn reservas_multiplican_el_grupo_y_nunca_quedan_negativas() {
        let mut expedicion = Expedicion::nueva(
            actor(0),
            vec![actor(0), actor(1), actor(2)],
            lugar(1),
            vec![lugar(0), lugar(1)],
            fecha(),
        )
        .expect("expedición válida");
        expedicion
            .preparar(PreparacionViaje::Suficiente, 2, 1)
            .expect("reservas válidas");
        assert_eq!(expedicion.reservas.agua, 6);
        assert_eq!(expedicion.reservas.alimento, 3);
        assert_eq!(expedicion.consumir(2, 1, 0), Some((1, 3)));
        assert_eq!(expedicion.reservas, ReservasExpedicion::default());
    }

    #[test]
    fn oportunidades_y_jornadas_restantes_son_transitorias() {
        let mut expedicion = Expedicion::nueva(
            actor(0),
            vec![actor(0), actor(1)],
            lugar(1),
            vec![lugar(0), lugar(1)],
            fecha(),
        )
        .expect("expedición válida");
        expedicion
            .preparar(PreparacionViaje::Cargada, 3, 2)
            .expect("reservas válidas");
        assert_eq!(expedicion.jornadas_completas_restantes(), 2);
        assert!(expedicion.registrar_intento_supervivencia(lugar(0), 1));
        assert!(!expedicion.registrar_intento_supervivencia(lugar(0), 1));
        assert!(expedicion.registrar_intento_supervivencia(lugar(0), 2));
    }
}
