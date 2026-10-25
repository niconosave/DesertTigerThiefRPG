use super::super::*;

impl EjecucionAccion {
    pub(crate) fn proponer_ruta(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        let umbral = self.rapida.len().div_ceil(2);
        let candidatos: Vec<_> = self
            .viajeros
            .iter()
            .copied()
            .filter(|&id| {
                tiene(mundo, id, "viajero")
                    || tiene(mundo, id, "explorador")
                    || self
                        .rapida
                        .iter()
                        .filter(|&&l| mundo.actor_conoce_lugar(id, l).unwrap_or(false))
                        .count()
                        >= umbral
            })
            .collect();
        if candidatos.is_empty() {
            self.ruta = self.larga.clone();
            self.fase = Fase::Preparar;
            return self.escena(
                mundo,
                ClaveTarjeta::SinPlan,
                ContenidoEscena::Plan {
                    proponente: None,
                    habilitacion: Vec::new(),
                    ruta: None,
                    previsto: None,
                },
                HorasViaje::horas(24),
                azar,
            );
        }
        let proponente = candidatos[sortear(azar, candidatos.len())?];
        let mut habilitacion = Vec::new();
        if tiene(mundo, proponente, "viajero") {
            habilitacion.push("Viajero");
        }
        if tiene(mundo, proponente, "explorador") {
            habilitacion.push("Explorador");
        }
        if self
            .rapida
            .iter()
            .filter(|&&l| mundo.actor_conoce_lugar(proponente, l).unwrap_or(false))
            .count()
            >= umbral
        {
            habilitacion.push("Conoce al menos la mitad de los Lugares de la ruta");
        }
        let a = self.viajeros.len() as u64;
        let previsto = rutas::total(&rutas::tiempos(
            mundo,
            &self.rapida,
            &self.viajeros,
            a,
            a,
            0,
        )?)?
        .tiempo()?;
        self.fase = Fase::ElegirRuta;
        self.decision(
            mundo,
            ClaveTarjeta::PropuestaRuta,
            ContenidoEscena::Plan {
                proponente: Some(proponente),
                habilitacion,
                ruta: Some(self.rapida.clone()),
                previsto: Some(previsto),
            },
            vec![
                OpcionViaje::AceptarPlan,
                OpcionViaje::OtraRuta,
                OpcionViaje::Cancelar,
            ],
            azar,
        )
    }
}
