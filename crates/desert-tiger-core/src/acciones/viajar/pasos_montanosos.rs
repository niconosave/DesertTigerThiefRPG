//! Recorrido reanudable de un segmento de pasos. La contabilidad usa el motor común.
use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SegmentoPaso {
    pub inicio: usize,
    pub fin: usize,
}

#[derive(Clone, Debug)]
pub(super) struct EstadoPaso {
    segmento: SegmentoPaso,
    cursor: u128, // diez unidades por conexión; no es tiempo ni una coordenada de tile
    siguiente: usize,
    objetivos: Vec<(MomentoPaso, u128)>,
    pub conexion: Option<HorasViaje>,
    previsto: HorasViaje,
    traslado: HorasViaje,
    inicio_total: HorasViaje,
    revisar_carga: bool,
}

pub(super) fn reconocer(mundo: &Mundo, ruta: &[IdLugar], posicion: usize) -> Option<SegmentoPaso> {
    let cantidad = ruta
        .get(posicion..)?
        .iter()
        .take_while(|&&l| {
            rutas::lugar_tiene(mundo, l, "paso") && rutas::lugar_tiene(mundo, l, "montanoso")
        })
        .count();
    (cantidad >= 2).then_some(SegmentoPaso {
        inicio: posicion,
        fin: posicion + cantidad.saturating_sub(1),
    })
}

fn objetivos(cantidad: usize) -> Vec<(MomentoPaso, u128)> {
    let l = (cantidad - 1) as u128;
    let mut momentos = vec![
        (MomentoPaso::Panorama, 0),
        (MomentoPaso::CaminoDificil, 2 * l),
        (MomentoPaso::Cornisa, 4 * l),
    ];
    if cantidad % 2 == 1 {
        momentos.push((MomentoPaso::Campamento, 5 * l));
    }
    momentos.extend([
        (MomentoPaso::Rodeos, 6 * l),
        (MomentoPaso::AbrirPaso, 8 * l),
        (MomentoPaso::OtraVertiente, 10 * l),
    ]);
    momentos
}

fn prefijo(duracion: HorasViaje, unidades: u128) -> Resultado<HorasViaje> {
    duracion
        .0
        .checked_mul(unidades)
        .map(|n| HorasViaje(n / 10))
        .ok_or_else(|| fallo("prefijo de paso desbordado"))
}

fn partes(t: crate::TiempoAccion) -> HorasViaje {
    HorasViaje(
        u128::from(t.duracion.dias()) * crate::calendario::PARTES_POR_DIA
            + u128::from(t.fraccion.partes()),
    )
}

#[derive(Default)]
struct Contabilidad {
    duracion: HorasViaje,
    consumo: RegistroConsumo,
}
impl Contabilidad {
    fn incorporar(&mut self, duracion: HorasViaje, consumo: RegistroConsumo) -> Resultado<()> {
        self.duracion = self.duracion.sumar(duracion)?;
        self.consumo.jornadas = sumar(self.consumo.jornadas, consumo.jornadas)?;
        self.consumo.agua_ml = sumar(self.consumo.agua_ml, consumo.agua_ml)?;
        for (a, b) in self.consumo.comidas.iter_mut().zip(consumo.comidas) {
            *a = sumar(*a, b)?;
        }
        // Una carencia iniciada en un intervalo sigue hasta el cierre del momento.
        self.consumo.sin_alimento = match self.consumo.sin_alimento {
            Some(t) => Some(partes(t).sumar(duracion)?.tiempo()?),
            None => consumo.sin_alimento,
        };
        self.consumo.sin_agua = match self.consumo.sin_agua {
            Some(t) => Some(partes(t).sumar(duracion)?.tiempo()?),
            None => consumo.sin_agua,
        };
        Ok(())
    }
    fn avanzar(
        &mut self,
        e: &mut EjecucionAccion,
        duracion: HorasViaje,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<()> {
        let consumo = e.contabilizar(duracion, e.viajeros.len(), azar)?;
        self.incorporar(duracion, consumo)
    }
}

impl EstadoPaso {
    pub(super) fn tiempo_pendiente(
        &self,
        e: &EjecucionAccion,
        mundo: &Mundo,
    ) -> Resultado<HorasViaje> {
        let duracion = self
            .conexion
            .ok_or_else(|| fallo("conexión parcial ausente"))?;
        let recorrido = prefijo(duracion, self.cursor % 10)?;
        let actual = HorasViaje(
            duracion
                .0
                .checked_sub(recorrido.0)
                .ok_or_else(|| fallo("cursor parcial inválido"))?,
        );
        let tiempos = rutas::tiempos(
            mundo,
            &e.ruta[e.posicion + 1..],
            &e.viajeros,
            e.caballos,
            e.camellos,
            e.animales,
        )?;
        actual
            .sumar(rutas::total(&tiempos)?)?
            .sumar(carga::suma_plan(
                e.demoras.get(e.posicion + 1..).unwrap_or(&[]),
            )?)
    }
}

impl EjecucionAccion {
    pub(super) fn iniciar_paso_montanoso(
        &mut self,
        mundo: &Mundo,
        segmento: SegmentoPaso,
    ) -> Resultado<()> {
        if segmento.inicio != self.posicion
            || segmento.fin <= segmento.inicio
            || segmento.fin >= self.ruta.len()
        {
            return Err(fallo("segmento de paso inválido"));
        }
        let tiempos = rutas::tiempos(
            mundo,
            &self.ruta[segmento.inicio..=segmento.fin],
            &self.viajeros,
            self.caballos,
            self.camellos,
            self.animales,
        )?;
        let previsto = rutas::total(&tiempos)?.sumar(carga::suma_plan(
            self.demoras
                .get(segmento.inicio..segmento.fin)
                .ok_or_else(|| fallo("carga del paso ausente"))?,
        )?)?;
        self.paso_montanoso = Some(EstadoPaso {
            segmento,
            cursor: 0,
            siguiente: 0,
            objetivos: objetivos(segmento.fin - segmento.inicio + 1),
            conexion: None,
            previsto,
            traslado: HorasViaje::CERO,
            inicio_total: self.total,
            revisar_carga: false,
        });
        Ok(())
    }

    pub(super) fn producir_paso_montanoso(
        &mut self,
        mundo: &mut Mundo,
        azar: &mut dyn FuenteAzar,
    ) -> Resultado<PasoAccion> {
        // avanzar/responder trabajan sobre un clon: se publica un único lote al final.
        let mut paso = self
            .paso_montanoso
            .take()
            .ok_or_else(|| fallo("episodio de paso ausente"))?;
        let conexiones = paso
            .segmento
            .fin
            .checked_sub(paso.segmento.inicio)
            .ok_or_else(|| fallo("segmento de paso invertido"))?;
        if conexiones == 0
            || paso.segmento.fin >= self.ruta.len()
            || paso.cursor > conexiones as u128 * 10
            || self.posicion != paso.segmento.inicio + (paso.cursor / 10) as usize
            || self.ruta.get(self.posicion) != Some(&self.lugar)
            || (paso.cursor % 10 != 0) != paso.conexion.is_some()
        {
            return Err(fallo("cursor de paso inconsistente"));
        }
        let (mut momento, objetivo) = *paso
            .objetivos
            .get(paso.siguiente)
            .ok_or_else(|| fallo("momento de paso fuera de rango"))?;
        let mut contabilidad = Contabilidad::default();
        let mut operaciones = Vec::new();
        let mut fase = Fase::Mover;
        while paso.cursor < objetivo {
            if self.posicion >= paso.segmento.fin {
                return Err(fallo("cursor del paso supera su ruta"));
            }
            let duracion = if let Some(d) = paso.conexion {
                d
            } else {
                let tiempos = rutas::tiempos(
                    mundo,
                    &self.ruta[self.posicion..=self.posicion + 1],
                    &self.viajeros,
                    self.caballos,
                    self.camellos,
                    self.animales,
                )?;
                let base = rutas::total(&tiempos)?;
                let carga = carga::suma_plan(
                    self.demoras
                        .get(self.posicion..=self.posicion)
                        .ok_or_else(|| fallo("carga parcial ausente"))?,
                )?;
                let d = base.sumar(carga)?;
                paso.conexion = Some(d);
                d
            };
            let frontera = (paso.cursor / 10 + 1) * 10;
            let hasta = objetivo.min(frontera);
            let inicio_local = paso.cursor % 10;
            let fin_local = if hasta == frontera { 10 } else { hasta % 10 };
            let delta = HorasViaje(
                prefijo(duracion, fin_local)?
                    .0
                    .checked_sub(prefijo(duracion, inicio_local)?.0)
                    .ok_or_else(|| fallo("avance negativo de paso"))?,
            );
            contabilidad.avanzar(self, delta, azar)?;
            paso.traslado = paso.traslado.sumar(delta)?;
            paso.cursor = hasta;
            if hasta == frontera {
                self.posicion += 1;
                self.lugar = *self
                    .ruta
                    .get(self.posicion)
                    .ok_or_else(|| fallo("destino del paso ausente"))?;
                operaciones.push(OperacionMundo::Trasladar {
                    actor: self.dirigente,
                    destino: self.lugar,
                });
                paso.conexion = None;
                let revisar = paso.revisar_carga;
                if revisar {
                    self.recalcular_carga(mundo)?;
                    paso.revisar_carga = false;
                }
                let reservas = self
                    .reservas
                    .as_ref()
                    .ok_or_else(|| fallo("paso sin reservas"))?;
                let exceso = carga::exceso(reservas, self.viajeros.len(), self.animales)?;
                let insuficiente = revisar
                    && exceso > 0
                    && !reservas.alcanza(self.viajeros.len() as u64, self.necesarias(mundo, 0)?)?;
                if self.lugar != self.destino && (reservas.agotada()? || insuficiente) {
                    fase = Fase::Entre {
                        revisar,
                        busco: false,
                    };
                    if hasta < objetivo {
                        momento = MomentoPaso::Interrupcion;
                    }
                    break;
                }
            }
        }
        let mut chequeo = None;
        let mut demora = HorasViaje::CERO;
        let mut comida_perdida = [0; 4];
        let mut animales_perdidos = 0;
        if momento != MomentoPaso::Interrupcion {
            if matches!(
                momento,
                MomentoPaso::CaminoDificil
                    | MomentoPaso::Cornisa
                    | MomentoPaso::Rodeos
                    | MomentoPaso::AbrirPaso
            ) {
                let responsable = crate::acciones::particulares::chequeos::mas_apto(
                    mundo,
                    &self.viajeros,
                    AccionParticular::Andar,
                    azar,
                )?;
                let c = resolver_chequeo(mundo, responsable, AccionParticular::Andar, 10, azar)?;
                if !c.exito {
                    demora = HorasViaje::horas(match momento {
                        MomentoPaso::CaminoDificil => 8,
                        MomentoPaso::Rodeos => 24,
                        MomentoPaso::AbrirPaso => 48,
                        _ => 0,
                    });
                    if momento == MomentoPaso::Cornisa && self.animales > 0 {
                        let cantidad = self.animales;
                        self.animales -= 1;
                        animales_perdidos = 1;
                        let reservas = self
                            .reservas
                            .as_mut()
                            .ok_or_else(|| fallo("caída sin reservas"))?;
                        for (n, perdida) in reservas.comidas.iter_mut().zip(&mut comida_perdida) {
                            *perdida = n.div_ceil(cantidad);
                            *n -= *perdida;
                        }
                        paso.revisar_carga = true;
                    }
                }
                chequeo = Some(c);
                if demora != HorasViaje::CERO {
                    contabilidad.avanzar(self, demora, azar)?;
                    paso.traslado = paso.traslado.sumar(demora)?;
                }
            }
            paso.siguiente += 1;
        }
        if paso.revisar_carga && paso.cursor % 10 == 0 {
            fase = Fase::Entre {
                revisar: true,
                busco: false,
            };
            paso.revisar_carga = false;
        }
        let otras = HorasViaje(
            self.total
                .0
                .checked_sub(paso.inicio_total.sumar(paso.traslado)?.0)
                .ok_or_else(|| fallo("contabilidad del episodio inconsistente"))?,
        );
        let dato = RegistroPasoMontanoso {
            momento,
            origen: self.ruta[paso.segmento.inicio],
            destino: self.ruta[paso.segmento.fin],
            cantidad_lugares: paso.segmento.fin - paso.segmento.inicio + 1,
            lugar_confirmado: self.lugar,
            previsto: paso.previsto.tiempo()?,
            traslado_acumulado: paso.traslado.tiempo()?,
            otras_actividades: otras.tiempo()?,
            chequeo,
            demora: demora.tiempo()?,
            animales_perdidos,
            comida_perdida,
        };
        if momento != MomentoPaso::OtraVertiente {
            self.paso_montanoso = Some(paso);
        }
        self.fase = fase;
        self.publicar_contabilizado(
            mundo,
            ClaveTarjeta::PasoMontanoso,
            ContenidoEscena::PasoMontanoso(dato),
            contabilidad.duracion,
            contabilidad.consumo,
            EfectosEscena {
                participantes: self.viajeros.clone(),
                operaciones,
            },
        )
        .map(PasoAccion::Escena)
    }
}

#[cfg(test)]
mod pruebas;
