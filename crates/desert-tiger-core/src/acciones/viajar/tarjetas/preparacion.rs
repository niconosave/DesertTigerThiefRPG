use super::super::*;

impl EjecucionAccion {
    pub(crate) fn preparar_viaje(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        let actor = crate::acciones::particulares::chequeos::mas_apto(
            mundo,
            &self.viajeros,
            AccionParticular::CalcularProvisiones,
            azar,
        )?;
        let chequeo = resolver_chequeo(
            mundo,
            actor,
            AccionParticular::CalcularProvisiones,
            10,
            azar,
        )?;
        let mult = if chequeo.ajustado < 10 {
            9
        } else if chequeo.ajustado < 16 {
            10
        } else {
            11
        };
        let (base, alternativa) = carga::preparar(mundo, &self.ruta, &self.viajeros, mult, azar)?;
        if let Some(alternativa) = alternativa {
            let contenido = ContenidoEscena::CargaInicial {
                base: self.vista_config(&base)?,
                alternativa: self.vista_config(&alternativa)?,
            };
            self.fase = Fase::ElegirCarga(Preparacion {
                chequeo,
                base,
                alternativa,
            });
            return self.decision(
                mundo,
                ClaveTarjeta::CargaInicial,
                contenido,
                vec![OpcionViaje::AceptarDemora, OpcionViaje::EvitarDemora],
                azar,
            );
        }
        self.fase = Fase::ConfirmarPreparacion(chequeo, base);
        self.producir(mundo, azar)
    }
}

impl EjecucionAccion {
    pub(crate) fn confirmar_preparacion(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
        chequeo: ResultadoChequeo,
        config: Configuracion,
    ) -> Resultado<PasoAccion> {
        let vista = self.vista_config(&config)?;
        self.animales = config.animales;
        self.demoras = config.demoras;
        self.reservas = Some(config.reservas);
        self.h = HorasViaje::CERO;
        let a = self.viajeros.len() as u64;
        for p in self.ruta.windows(2) {
            match rutas::medio(mundo, p[0], p[1], self.viajeros.len(), a, a) {
                Medio::Caballo => self.caballos = a,
                Medio::Camello => self.camellos = a,
                _ => {}
            }
        }
        self.fase = if chequeo.exito {
            Fase::Mover
        } else {
            Fase::Aviso { inicial: true }
        };
        self.escena(
            mundo,
            ClaveTarjeta::Preparacion,
            ContenidoEscena::Preparacion {
                chequeo,
                configuracion: vista,
            },
            HorasViaje::horas(24),
            azar,
        )
    }
}

impl EjecucionAccion {
    pub(crate) fn advertir_provisiones(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
        inicial: bool,
    ) -> Resultado<PasoAccion> {
        let necesarias = self.necesarias(mundo, 0)?;
        let r = self
            .reservas
            .as_ref()
            .ok_or_else(|| fallo("aviso sin reservas"))?;
        let falta_comida =
            r.alimento()? < multiplicar(multiplicar(4, self.viajeros.len() as u64)?, necesarias)?;
        let falta_agua =
            r.agua()? < multiplicar(multiplicar(500, self.viajeros.len() as u64)?, necesarias)?;
        self.fase = Fase::ElegirAviso { inicial };
        self.decision(
            mundo,
            ClaveTarjeta::Incompleto,
            ContenidoEscena::Incompleto {
                inicial,
                falta_comida,
                falta_agua,
            },
            vec![OpcionViaje::Continuar, OpcionViaje::Cancelar],
            azar,
        )
    }
}
