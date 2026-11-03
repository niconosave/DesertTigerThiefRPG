use super::super::*;
impl EjecucionAccion {
    pub(crate) fn proponer_ruta(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        self.fase = Fase::ElegirRuta;
        let mut opciones: Vec<_> = self
            .propuestas
            .iter()
            .map(|p| OpcionViaje::ElegirRuta(p.id))
            .collect();
        opciones.extend([OpcionViaje::Improvisar, OpcionViaje::Cancelar]);
        self.decision(
            mundo,
            ClaveTarjeta::PropuestaRuta,
            ContenidoEscena::Plan {
                propuestas: self.propuestas.clone(),
            },
            opciones,
            azar,
        )
    }
}
