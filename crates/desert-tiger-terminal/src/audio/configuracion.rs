use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DatosFogata {
    volumen: f32,
    nivel_brasa: f32,
    chasquidos_por_segundo: f32,
    crujidos_por_segundo: f32,
    entrada_ms: u32,
    salida_ms: u32,
}

#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(super) struct ConfiguracionFogataAudio(DatosValidados);

// Solo el cargador puede construir o modificar los datos validados.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
struct DatosValidados {
    volumen: f32,
    nivel_brasa: f32,
    tasas: [f32; 2],
    entrada_ms: u32,
    salida_ms: u32,
}

impl ConfiguracionFogataAudio {
    pub(super) fn cargar(ruta: &Path) -> Result<Self, String> {
        let texto =
            std::fs::read_to_string(ruta).map_err(|e| format!("{}: {e}", ruta.display()))?;
        Self::interpretar(&texto).map_err(|e| format!("{}: {e}", ruta.display()))
    }

    fn interpretar(texto: &str) -> Result<Self, String> {
        let datos: DatosFogata = ron::from_str(texto).map_err(|e| e.to_string())?;
        for (campo, valor, maximo) in [
            ("volumen", datos.volumen, 1.0),
            ("nivel_brasa", datos.nivel_brasa, 1.0),
            ("chasquidos_por_segundo", datos.chasquidos_por_segundo, 40.0),
            ("crujidos_por_segundo", datos.crujidos_por_segundo, 5.0),
        ] {
            if !valor.is_finite() || !(0.0..=maximo).contains(&valor) {
                return Err(format!(
                    "{campo}: se espera un número finito entre 0 y {maximo}"
                ));
            }
        }
        for (campo, valor) in [
            ("entrada_ms", datos.entrada_ms),
            ("salida_ms", datos.salida_ms),
        ] {
            if !(5..=2000).contains(&valor) {
                return Err(format!("{campo}: se espera un entero entre 5 y 2000"));
            }
        }
        Ok(Self(DatosValidados {
            volumen: datos.volumen,
            nivel_brasa: datos.nivel_brasa,
            tasas: [datos.chasquidos_por_segundo, datos.crujidos_por_segundo],
            entrada_ms: datos.entrada_ms,
            salida_ms: datos.salida_ms,
        }))
    }
}

#[cfg(feature = "audio")]
impl ConfiguracionFogataAudio {
    pub(super) fn volumen(&self) -> f32 {
        self.0.volumen
    }
    pub(super) fn nivel_brasa(&self) -> f32 {
        self.0.nivel_brasa
    }
    pub(super) fn tasas(&self) -> [f32; 2] {
        self.0.tasas
    }
    pub(super) fn entrada_ms(&self) -> u32 {
        self.0.entrada_ms
    }
    pub(super) fn salida_ms(&self) -> u32 {
        self.0.salida_ms
    }
}

#[cfg(all(test, feature = "audio"))]
pub(super) fn fixture(volumen: f32, brasa: f32, tasas: [f32; 2]) -> ConfiguracionFogataAudio {
    ConfiguracionFogataAudio::interpretar(&format!(
        "(volumen:{volumen:?},nivel_brasa:{brasa:?},chasquidos_por_segundo:{:?},crujidos_por_segundo:{:?},entrada_ms:200,salida_ms:250)", tasas[0], tasas[1]
    )).expect("fixture válida")
}

#[cfg(test)]
mod pruebas {
    use super::*;
    const VALIDA: &str = "(volumen:0.25,nivel_brasa:0.18,chasquidos_por_segundo:8.0,crujidos_por_segundo:0.6,entrada_ms:200,salida_ms:250)";

    #[test]
    fn valida_limites_y_campos_estrictos() {
        assert!(ConfiguracionFogataAudio::interpretar(VALIDA).is_ok());
        for (campo, anterior, valor) in [
            ("volumen", "0.25", "1.1"),
            ("volumen", "0.25", "NaN"),
            ("nivel_brasa", "0.18", "inf"),
            ("chasquidos_por_segundo", "8.0", "-1.0"),
            ("chasquidos_por_segundo", "8.0", "40.1"),
            ("crujidos_por_segundo", "0.6", "5.1"),
            ("entrada_ms", "200", "0"),
            ("salida_ms", "250", "2001"),
        ] {
            let invalida =
                VALIDA.replace(&format!("{campo}:{anterior}"), &format!("{campo}:{valor}"));
            let error = ConfiguracionFogataAudio::interpretar(&invalida).unwrap_err();
            assert!(error.contains(campo), "{error}");
        }
        for texto in [
            VALIDA.replace("volumen:0.25,", ""),
            VALIDA.replace("volumen:0.25,", "volumen:0.25,volumen:0.2,"),
            VALIDA.replace("volumen:0.25,", "otro:1,volumen:0.25,"),
        ] {
            assert!(ConfiguracionFogataAudio::interpretar(&texto).is_err());
        }
        assert!(
            ConfiguracionFogataAudio::interpretar(
                &VALIDA
                    .replace("entrada_ms:200", "entrada_ms:5")
                    .replace("salida_ms:250", "salida_ms:2000")
            )
            .is_ok()
        );
    }

    #[test]
    fn recurso_oficial_y_error_con_ruta() {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(ConfiguracionFogataAudio::cargar(&raiz.join("assets/audio/fogata.ron")).is_ok());
        assert!(
            ConfiguracionFogataAudio::cargar(Path::new("/ruta/ausente/fogata.ron"))
                .unwrap_err()
                .contains("/ruta/ausente/fogata.ron")
        );
    }
}
