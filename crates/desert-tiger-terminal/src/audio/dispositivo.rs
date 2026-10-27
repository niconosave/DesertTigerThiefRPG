use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    BufferSize, Device, FromSample, SampleFormat, SizedSample, Stream, StreamConfig,
    SupportedStreamConfigRange,
};

use super::motor::{CERRAR, Comunicacion, FALLO_MUESTRA, FALLO_STREAM, MotorAudio};
use super::{AvisoAudio, configuracion::ConfiguracionAudio};

pub(super) struct SalidaAudio {
    _stream: Stream,
    comunicacion: Comunicacion,
}

// La elección trabaja sobre datos, sin abrir el dispositivo: también es comprobable en CI.
fn elegir(
    rangos: &[SupportedStreamConfigRange],
    predeterminada: Option<u32>,
) -> Option<(StreamConfig, SampleFormat)> {
    for formato in [
        SampleFormat::F32,
        SampleFormat::I16,
        SampleFormat::I32,
        SampleFormat::U16,
    ] {
        for canales in [2, 1] {
            let candidatos: Vec<_> = rangos
                .iter()
                .filter(|r| r.sample_format() == formato && r.channels() == canales)
                .filter_map(|r| {
                    let minimo = r.min_sample_rate().max(8000);
                    let maximo = r.max_sample_rate().min(192000);
                    (minimo <= maximo).then_some((minimo, maximo))
                })
                .collect();
            for frecuencia in [predeterminada, Some(48000), Some(44100)]
                .into_iter()
                .flatten()
            {
                if candidatos
                    .iter()
                    .any(|&(min, max)| (min..=max).contains(&frecuencia))
                {
                    return Some((
                        StreamConfig {
                            channels: canales,
                            sample_rate: frecuencia,
                            buffer_size: BufferSize::Default,
                        },
                        formato,
                    ));
                }
            }
            if let Some(frecuencia) = candidatos
                .iter()
                .map(|&(min, max)| 48000_u32.clamp(min, max))
                .min_by_key(|f| (f.abs_diff(48000), *f))
            {
                return Some((
                    StreamConfig {
                        channels: canales,
                        sample_rate: frecuencia,
                        buffer_size: BufferSize::Default,
                    },
                    formato,
                ));
            }
        }
    }
    None
}

fn muestra_segura(muestra: f32, comunicacion: &Comunicacion, fallo: &mut bool) -> f32 {
    if !muestra.is_finite() {
        comunicacion.informar_fallo(FALLO_MUESTRA);
        *fallo = true;
        0.0
    } else {
        muestra
    }
}

fn escribir<T: SizedSample + FromSample<f32>>(
    datos: &mut [T],
    canales: usize,
    comunicacion: &Comunicacion,
    motor: &mut MotorAudio,
) {
    let orden = comunicacion.orden.load(Ordering::Acquire);
    motor.ordenar(orden);
    let mut fallo = comunicacion.fallo.load(Ordering::Acquire) != 0;
    for cuadro in datos.chunks_mut(canales) {
        let muestra = if fallo { 0.0 } else { motor.siguiente() };
        let muestra = muestra_segura(muestra, comunicacion, &mut fallo);
        cuadro.fill(T::from_sample(muestra));
    }
    // Solo se confirma la orden final observada, nunca un silencio anterior.
    if orden == CERRAR && (motor.en_silencio() || fallo) {
        comunicacion.silencio.store(1, Ordering::Release);
    }
}

fn construir<T: SizedSample + FromSample<f32>>(
    dispositivo: &Device,
    config: StreamConfig,
    comunicacion: &Comunicacion,
    mut motor: MotorAudio,
) -> Result<Stream, AvisoAudio> {
    let canales = config.channels as usize;
    let salida = comunicacion.clone();
    let errores = comunicacion.clone();
    dispositivo
        .build_output_stream::<T, _, _>(
            config,
            move |datos, _| escribir(datos, canales, &salida, &mut motor),
            move |_| errores.informar_fallo(FALLO_STREAM),
            Some(Duration::from_millis(500)),
        )
        .map_err(|_| AvisoAudio::Creacion)
}

impl SalidaAudio {
    pub(super) fn abrir(config: ConfiguracionAudio) -> Result<Self, AvisoAudio> {
        let dispositivo = cpal::default_host()
            .default_output_device()
            .ok_or(AvisoAudio::SinDispositivo)?;
        let predeterminada = dispositivo
            .default_output_config()
            .ok()
            .map(|c| c.sample_rate());
        let rangos: Vec<_> = dispositivo
            .supported_output_configs()
            .map_err(|_| AvisoAudio::Configuracion)?
            .collect();
        let (salida, formato) = elegir(&rangos, predeterminada).ok_or(AvisoAudio::Configuracion)?;
        let motor = MotorAudio::con_configuracion(config, salida.sample_rate)
            .map_err(|_| AvisoAudio::Configuracion)?;
        let comunicacion = Comunicacion::default();
        let stream = match formato {
            SampleFormat::F32 => construir::<f32>(&dispositivo, salida, &comunicacion, motor),
            SampleFormat::I16 => construir::<i16>(&dispositivo, salida, &comunicacion, motor),
            SampleFormat::I32 => construir::<i32>(&dispositivo, salida, &comunicacion, motor),
            SampleFormat::U16 => construir::<u16>(&dispositivo, salida, &comunicacion, motor),
            _ => return Err(AvisoAudio::Configuracion),
        }?;
        stream.play().map_err(|_| AvisoAudio::Arranque)?;
        Ok(Self {
            _stream: stream,
            comunicacion,
        })
    }

    pub(super) fn ordenar(&self, orden: u8) {
        self.comunicacion.orden.store(orden, Ordering::Release);
    }

    pub(super) fn fallo(&self) -> Option<AvisoAudio> {
        match self.comunicacion.fallo.load(Ordering::Acquire) {
            0 => None,
            FALLO_MUESTRA => Some(AvisoAudio::MuestraNoFinita),
            _ => Some(AvisoAudio::Flujo),
        }
    }

    pub(super) fn cerrar(&self) {
        self.comunicacion.silencio.store(0, Ordering::Release);
        self.ordenar(CERRAR);
        let inicio = Instant::now();
        while self.comunicacion.silencio.load(Ordering::Acquire) == 0
            && inicio.elapsed() < Duration::from_millis(150)
        {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::super::{configuracion::fixture, motor::FOGATA};
    use super::*;
    use cpal::SupportedBufferSize;

    fn rango(
        formato: SampleFormat,
        canales: u16,
        minimo: u32,
        maximo: u32,
    ) -> SupportedStreamConfigRange {
        SupportedStreamConfigRange::new(
            canales,
            minimo,
            maximo,
            SupportedBufferSize::Unknown,
            formato,
        )
    }

    #[test]
    fn orden_de_configuraciones_y_frecuencias() {
        let rangos = [
            rango(SampleFormat::I16, 2, 44100, 48000),
            rango(SampleFormat::F32, 1, 44100, 44100),
            rango(SampleFormat::F32, 2, 8000, 96000),
        ];
        let (c, f) = elegir(&rangos, Some(88200)).unwrap();
        assert_eq!(
            (c.channels, c.sample_rate, f),
            (2, 88200, SampleFormat::F32)
        );
        assert_eq!(elegir(&rangos, None).unwrap().0.sample_rate, 48000);
        assert_eq!(elegir(&rangos[..2], None).unwrap().0.channels, 1);
        assert_eq!(
            elegir(&[rango(SampleFormat::F32, 2, 50000, 96000)], None)
                .unwrap()
                .0
                .sample_rate,
            50000
        );
        assert!(elegir(&[rango(SampleFormat::F64, 6, 48000, 48000)], None).is_none());
        assert!(elegir(&[rango(SampleFormat::F32, 2, 200000, 300000)], None).is_none());
        assert!(elegir(&[], None).is_none());
    }

    #[test]
    fn buffers_duplican_mono_convierten_y_silencian_fallos_sin_dispositivo() {
        let crear = || MotorAudio::nuevo(fixture(1.0, 1.0, [8.0, 0.6]), 48000).unwrap();
        let comunicacion = Comunicacion::default();
        let mut motor = crear();
        comunicacion.orden.store(FOGATA, Ordering::Release);
        let mut muestras = [f32::NAN; 257];
        escribir(&mut muestras, 2, &comunicacion, &mut motor);
        assert!(muestras.iter().all(|m| m.is_finite()));
        assert!(muestras.as_chunks::<2>().0.iter().all(|p| p[0] == p[1]));
        comunicacion.informar_fallo(FALLO_STREAM);
        comunicacion.informar_fallo(FALLO_MUESTRA);
        assert_eq!(comunicacion.fallo.load(Ordering::Acquire), FALLO_STREAM);
        let mut enteras = [0_u16; 257];
        escribir(&mut enteras, 2, &comunicacion, &mut motor);
        assert!(enteras.iter().all(|m| *m == 32768));
        comunicacion.orden.store(CERRAR, Ordering::Release);
        escribir(&mut muestras, 2, &comunicacion, &mut motor);
        assert_eq!(comunicacion.silencio.load(Ordering::Acquire), 1);
        assert!(muestras.iter().all(|m| *m == 0.0));
    }

    #[test]
    fn callback_es_independiente_del_tamano_de_buffer() {
        let render = |bloque: usize| {
            let mut motor = MotorAudio::nuevo(fixture(1.0, 1.0, [8.0, 0.6]), 44100).unwrap();
            let c = Comunicacion::default();
            let mut salida = vec![0.0; 44100];
            c.orden.store(FOGATA, Ordering::Release);
            for parte in salida.chunks_mut(bloque) {
                escribir(parte, 1, &c, &mut motor);
            }
            salida
        };
        assert_eq!(render(1), render(1024));
    }

    #[test]
    fn muestras_no_finitas_se_reemplazan_y_desactivan_el_audio() {
        for invalida in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let c = Comunicacion::default();
            let mut fallo = false;
            assert_eq!(muestra_segura(invalida, &c, &mut fallo), 0.0);
            assert!(fallo);
            assert_eq!(c.fallo.load(Ordering::Acquire), FALLO_MUESTRA);
            let mut motor = MotorAudio::nuevo(fixture(1.0, 1.0, [40.0, 5.0]), 48000).unwrap();
            c.orden.store(FOGATA, Ordering::Release);
            let mut datos = [1.0; 256];
            escribir(&mut datos, 2, &c, &mut motor);
            assert!(datos.iter().all(|m| *m == 0.0));
        }
    }
}
