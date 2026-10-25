use super::super::*;

impl EjecucionAccion {
    pub(crate) fn llegar(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        self.fase = Fase::Terminar(CierreViaje::Llegada);
        self.escena(
            mundo,
            ClaveTarjeta::Llegada,
            ContenidoEscena::Llegada {
                lugares: self.ruta.len(),
            },
            HorasViaje::CERO,
            azar,
        )
    }
}
