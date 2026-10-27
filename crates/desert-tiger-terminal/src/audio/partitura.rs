//! Prueba musical offline: genera un WAV sin abrir el dispositivo de audio.
use fundsp::prelude32::{AttoHash, AudioUnit, highpass_hz, lowpass_hz, pluck, zero};
use serde::Deserialize;
use std::{error::Error, fs, io::Write, path::Path};

pub(super) const FRECUENCIA: u32 = 48000;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Melodia {
    titulo: String,
    tempo: f32,
    volumen: f32,
    amortiguacion: f32,
    // Inicio (negras), nombre/octava, duración (negras), intensidad.
    notas: Vec<(f32, String, f32, f32)>,
}

fn altura(nota: &str) -> Result<u8, String> {
    let b = nota.as_bytes();
    let base = match b.first() {
        Some(b'C') => 0,
        Some(b'D') => 2,
        Some(b'E') => 4,
        Some(b'F') => 5,
        Some(b'G') => 7,
        Some(b'A') => 9,
        Some(b'B') => 11,
        _ => return Err(format!("nota inválida: {nota}")),
    };
    let (alteracion, octava) = match b {
        [_, n @ b'0'..=b'8'] => (0, n - b'0'),
        [_, b'#', n @ b'0'..=b'8'] => (1, n - b'0'),
        [_, b'b', n @ b'0'..=b'8'] => (-1, n - b'0'),
        _ => return Err(format!("nota inválida: {nota}")),
    };
    let midi = (i16::from(octava) + 1) * 12 + base + alteracion;
    if !(24..=96).contains(&midi) {
        return Err(format!("nota fuera de C1–C7: {nota}"));
    }
    Ok(midi as u8)
}

impl Melodia {
    fn validar(&self) -> Result<(), String> {
        if self.titulo.trim().is_empty() || self.titulo.chars().any(char::is_control) {
            return Err("titulo: se espera texto visible".into());
        }
        for (campo, valor, min, max) in [
            ("tempo", self.tempo, 40.0, 200.0),
            ("volumen", self.volumen, 0.0, 1.0),
            ("amortiguacion", self.amortiguacion, 0.0, 1.0),
        ] {
            if !valor.is_finite() || !(min..=max).contains(&valor) {
                return Err(format!("{campo}: fuera de {min}–{max}"));
            }
        }
        if self.notas.is_empty() || self.notas.len() > 512 {
            return Err("notas: se esperan 1–512 eventos".into());
        }
        for (i, (inicio, nota, duracion, intensidad)) in self.notas.iter().enumerate() {
            for (campo, valor, min, max) in [
                ("inicio", *inicio, 0.0, 512.0),
                ("duracion", *duracion, 0.05, 16.0),
                ("intensidad", *intensidad, 0.0, 1.0),
            ] {
                if !valor.is_finite() || !(min..=max).contains(&valor) {
                    return Err(format!("notas[{i}].{campo}: fuera de {min}–{max}"));
                }
            }
            altura(nota).map_err(|e| format!("notas[{i}]: {e}"))?;
        }
        Ok(())
    }

    fn renderizar(&self) -> Result<Vec<f32>, String> {
        self.validar()?;
        let pulso = 60.0 / f64::from(self.tempo);
        let final_pulsos = self
            .notas
            .iter()
            .map(|(inicio, _, duracion, _)| inicio + duracion)
            .fold(0.0_f32, f32::max);
        let mut salida = vec![
            0.0;
            ((f64::from(final_pulsos) * pulso + 0.5) * f64::from(FRECUENCIA)).ceil()
                as usize
        ];
        for (i, (inicio, nota, duracion, intensidad)) in self.notas.iter().enumerate() {
            let midi = altura(nota)?;
            let frecuencia = 440.0 * 2.0_f32.powf((f32::from(midi) - 69.0) / 12.0);
            let mut cuerda = zero() >> pluck(frecuencia, 0.45, self.amortiguacion);
            cuerda.set_sample_rate(f64::from(FRECUENCIA));
            cuerda.ping(false, AttoHash::new(0x68696c6f ^ i as u64));
            cuerda.allocate();
            let inicio = (f64::from(*inicio) * pulso * f64::from(FRECUENCIA)).round() as usize;
            let duracion = (f64::from(*duracion) * pulso * f64::from(FRECUENCIA)).round() as usize;
            for (j, celda) in salida.iter_mut().skip(inicio).take(duracion).enumerate() {
                let ataque = (j as f32 / (FRECUENCIA as f32 * 0.003)).min(1.0);
                let retirada = ((duracion - j) as f32 / (FRECUENCIA as f32 * 0.09)).min(1.0);
                *celda += cuerda.get_mono() * intensidad * ataque * retirada;
            }
        }
        let q = std::f32::consts::FRAC_1_SQRT_2;
        let mut filtro: Box<dyn AudioUnit> =
            Box::new(highpass_hz(65.0, q) >> lowpass_hz(6500.0, q));
        filtro.set_sample_rate(f64::from(FRECUENCIA));
        filtro.allocate();
        for muestra in &mut salida {
            let mut filtrada = [0.0];
            filtro.tick(&[*muestra], &mut filtrada);
            *muestra = 0.8 * (filtrada[0] * self.volumen / 0.8).tanh();
            if !muestra.is_finite() {
                return Err("muestra musical no finita".into());
            }
        }
        Ok(salida)
    }
}

fn escribir_wav(ruta: &Path, muestras: &[f32]) -> Result<(), Box<dyn Error>> {
    if let Some(padre) = ruta.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(padre)?;
    }
    let datos = u32::try_from(muestras.len() * 2)?;
    let mut archivo = std::io::BufWriter::new(fs::File::create(ruta)?);
    archivo.write_all(b"RIFF")?;
    archivo.write_all(&(36 + datos).to_le_bytes())?;
    archivo.write_all(b"WAVEfmt ")?;
    archivo.write_all(&16_u32.to_le_bytes())?;
    archivo.write_all(&1_u16.to_le_bytes())?;
    archivo.write_all(&1_u16.to_le_bytes())?;
    archivo.write_all(&FRECUENCIA.to_le_bytes())?;
    archivo.write_all(&(FRECUENCIA * 2).to_le_bytes())?;
    archivo.write_all(&2_u16.to_le_bytes())?;
    archivo.write_all(&16_u16.to_le_bytes())?;
    archivo.write_all(b"data")?;
    archivo.write_all(&datos.to_le_bytes())?;
    for muestra in muestras {
        archivo.write_all(&((muestra * 32767.0).round() as i16).to_le_bytes())?;
    }
    archivo.flush()?;
    Ok(())
}

pub(super) fn sintetizar_archivo(ruta: &Path) -> Result<std::sync::Arc<[f32]>, String> {
    let resultado = || -> Result<_, String> {
        let texto = fs::read_to_string(ruta).map_err(|e| e.to_string())?;
        let melodia: Melodia = ron::from_str(&texto).map_err(|e| e.to_string())?;
        Ok(std::sync::Arc::from(melodia.renderizar()?))
    };
    resultado().map_err(|e| format!("{}: {e}", ruta.display()))
}

pub(super) fn previsualizar(entrada: &Path, salida: &Path) -> Result<f64, String> {
    let muestras = sintetizar_archivo(entrada)?;
    escribir_wav(salida, &muestras).map_err(|e| format!("{}: {e}", salida.display()))?;
    Ok(muestras.len() as f64 / f64::from(FRECUENCIA))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn partitura_oficial_carga_y_valida() {
        let ruta = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/audio/musica/hilo_del_camino.ron");
        let melodia: Melodia = ron::from_str(&fs::read_to_string(ruta).unwrap()).unwrap();
        melodia.validar().unwrap();
    }
    #[test]
    fn alturas_y_datos_invalidos() {
        assert_eq!(altura("A4"), Ok(69));
        assert_eq!(altura("C#4"), altura("Db4"));
        for nota in ["", "Q4", "C", "C9", "A♯4"] {
            assert!(altura(nota).is_err());
        }
        let mut melodia = Melodia {
            titulo: "Prueba".into(),
            tempo: 92.0,
            volumen: 0.8,
            amortiguacion: 0.15,
            notas: vec![(0.0, "A3".into(), 1.0, 0.5)],
        };
        assert!(melodia.validar().is_ok());
        melodia.notas[0].0 = f32::NAN;
        assert!(melodia.validar().is_err());
    }
    #[test]
    fn render_musical_reproducible_acotado_y_silenciable() {
        let mut melodia = Melodia {
            titulo: "Prueba".into(),
            tempo: 92.0,
            volumen: 0.8,
            amortiguacion: 0.15,
            notas: vec![(0.0, "A3".into(), 1.0, 0.5), (0.5, "E4".into(), 1.0, 0.6)],
        };
        let a = melodia.renderizar().unwrap();
        assert_eq!(a, melodia.renderizar().unwrap());
        assert!(a.iter().all(|x| x.is_finite() && x.abs() <= 0.8));
        assert!(a.iter().any(|x| x.abs() > 0.01));
        melodia.volumen = 0.0;
        assert!(melodia.renderizar().unwrap().iter().all(|x| *x == 0.0));
    }
}
