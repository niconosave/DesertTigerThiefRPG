use super::super::*;

impl EjecucionAccion {
    pub(crate) fn ofrecer_descarte(
        &mut self,
        mundo: &mut Mundo,
        exceso: u64,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        let reservas = self
            .reservas
            .as_ref()
            .ok_or_else(|| fallo("descarte sin reservas"))?;
        let lote = carga::descarte(reservas, exceso)?;
        let tiempos = self.tiempos_restantes(mundo)?;
        let config = Configuracion {
            reservas: reservas.clone(),
            animales: self.animales,
            exceso,
            referencia: rutas::sumar_horas(&tiempos)?,
            suplemento: ReservasViaje::default(),
            demoras: carga::plan(&tiempos, exceso)?,
        };
        let contenido = ContenidoEscena::Sobrecarga {
            configuracion: self.vista_config(&config)?,
            lote: lote.clone(),
        };
        self.fase = Fase::Descartar(lote);
        self.decision(
            mundo,
            ClaveTarjeta::Sobrecarga,
            contenido,
            vec![OpcionViaje::Descartar, OpcionViaje::Cancelar],
            azar,
        )
    }
}
