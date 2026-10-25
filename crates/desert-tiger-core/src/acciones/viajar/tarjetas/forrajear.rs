use super::super::*;

impl EjecucionAccion {
    pub(crate) fn forrajear(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        let n = self.necesarias(mundo, 6)?;
        let a = self.viajeros.len() as u64;
        let r = self
            .reservas
            .as_ref()
            .ok_or_else(|| fallo("búsqueda sin reservas"))?;
        let busca_comida =
            r.alimento()? < multiplicar(multiplicar(4, a)?, n)? || r.alimento()? == 0;
        let busca_agua = r.agua()? < multiplicar(multiplicar(500, a)?, n)? || r.agua()? == 0;
        let mut chequeos = Vec::new();
        let mut obtenido = ReservasViaje::default();
        let mut agua = 0;
        for &id in &self.viajeros {
            let c = resolver_chequeo(mundo, id, AccionParticular::Forajear, 14, azar)?;
            if c.exito {
                if busca_comida {
                    obtenido.comidas[0] =
                        sumar(obtenido.comidas[0], (sortear(azar, 9)? + 2) as u64)?;
                }
                if busca_agua {
                    agua = sumar(agua, multiplicar((sortear(azar, 9)? + 2) as u64, 1000)?)?;
                }
            }
            chequeos.push(c);
        }
        obtenido.agregar_agua(agua)?;
        self.reservas
            .as_mut()
            .ok_or_else(|| fallo("búsqueda sin reserva"))?
            .incorporar(&obtenido)?;
        let mut registro = self.registro(
            mundo,
            ClaveTarjeta::Forrajeo,
            ContenidoEscena::Forrajeo {
                chequeos,
                obtenido,
                busca_comida,
                busca_agua,
            },
            HorasViaje::horas(6),
            EfectosEscena {
                participantes: self.viajeros.clone(),
                operaciones: Vec::new(),
            },
            azar,
        )?;
        let r = self
            .reservas
            .as_ref()
            .ok_or_else(|| fallo("reservas perdidas"))?;
        self.fase = if !r.alcanza(a, 1)? {
            Fase::Terminar(CierreViaje::SinRecursos)
        } else if !r.alcanza(a, self.necesarias(mundo, 0)?)? {
            Fase::Aviso { inicial: false }
        } else {
            Fase::Entre {
                revisar: true,
                busco: true,
            }
        };
        if let Fase::Terminar(cierre) = self.fase {
            registro.cierre = Some(cierre);
        }
        Ok(PasoAccion::Escena(registro))
    }
}
