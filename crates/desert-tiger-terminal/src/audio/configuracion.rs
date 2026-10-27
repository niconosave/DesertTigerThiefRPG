use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(super) struct ConfiguracionAudio {
    pub fogata: ConfiguracionFogataAudio,
    pub ambientes: ConfiguracionAmbientes,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DatosAmbientes {
    #[serde(default)]
    tipo_rio: TipoRio,
    volumen: f32,
    viento: f32,
    ave_lejana: f32,
    rio: f32,
    mar: f32,
    ave_bosque_a: f32,
    ave_bosque_b: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
pub(super) enum TipoRio {
    #[default]
    Corriente,
    Riachuelo,
}

#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(super) struct ConfiguracionAmbientes {
    tipo_rio: TipoRio,
    volumen: f32,
    niveles: [f32; 6],
}

impl ConfiguracionAmbientes {
    pub(super) fn cargar(ruta: &Path) -> Result<Self, String> {
        let texto =
            std::fs::read_to_string(ruta).map_err(|e| format!("{}: {e}", ruta.display()))?;
        Self::interpretar(&texto).map_err(|e| format!("{}: {e}", ruta.display()))
    }

    fn interpretar(texto: &str) -> Result<Self, String> {
        let d: DatosAmbientes = ron::from_str(texto).map_err(|e| e.to_string())?;
        for (campo, valor) in [
            ("volumen", d.volumen),
            ("viento", d.viento),
            ("ave_lejana", d.ave_lejana),
            ("rio", d.rio),
            ("mar", d.mar),
            ("ave_bosque_a", d.ave_bosque_a),
            ("ave_bosque_b", d.ave_bosque_b),
        ] {
            if !valor.is_finite() || !(0.0..=1.0).contains(&valor) {
                return Err(format!("{campo}: se espera un número finito entre 0 y 1"));
            }
        }
        Ok(Self {
            tipo_rio: d.tipo_rio,
            volumen: d.volumen,
            niveles: [
                d.viento,
                d.ave_lejana,
                d.rio,
                d.mar,
                d.ave_bosque_a,
                d.ave_bosque_b,
            ],
        })
    }

    #[cfg(feature = "audio")]
    pub(super) fn tipo_rio(&self) -> TipoRio {
        self.tipo_rio
    }
    #[cfg(feature = "audio")]
    pub(super) fn volumen(&self) -> f32 {
        self.volumen
    }
    #[cfg(feature = "audio")]
    pub(super) fn niveles(&self) -> [f32; 6] {
        self.niveles
    }
    #[cfg(all(feature = "audio", test))]
    pub(super) fn fixture() -> Self {
        Self::interpretar("(volumen:0.35,viento:0.1,ave_lejana:0.18,rio:0.3,mar:0.26,ave_bosque_a:0.2,ave_bosque_b:0.16)").unwrap()
    }

    #[cfg(all(feature = "audio", test))]
    pub(super) fn fixture_silenciosa() -> Self {
        let mut config = Self::fixture();
        config.volumen = 0.0;
        config
    }

    #[cfg(all(feature = "audio", test))]
    pub(super) fn con_tipo_rio(mut self, tipo: TipoRio) -> Self {
        self.tipo_rio = tipo;
        self
    }
}

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
    fn tipo_rio_es_opcional_conserva_corriente_y_rechaza_valores_desconocidos() {
        let texto = "(volumen:0.5,viento:0.5,ave_lejana:0.5,rio:0.5,mar:0.5,ave_bosque_a:0.5,ave_bosque_b:0.5)";
        assert_eq!(
            ConfiguracionAmbientes::interpretar(texto).unwrap().tipo_rio,
            TipoRio::Corriente
        );
        for tipo in ["Corriente", "Riachuelo"] {
            let texto = texto.replacen('(', &format!("(tipo_rio:{tipo},"), 1);
            assert!(ConfiguracionAmbientes::interpretar(&texto).is_ok());
        }
        let texto = texto.replacen('(', "(tipo_rio:Desconocido,", 1);
        assert!(
            ConfiguracionAmbientes::interpretar(&texto)
                .unwrap_err()
                .contains("Desconocido")
        );
    }

    #[test]
    fn ambientes_validan_todos_los_niveles_y_campos_estrictos() {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(ConfiguracionAmbientes::cargar(&raiz.join("assets/audio/ambientes.ron")).is_ok());
        let valida = "(volumen:0.5,viento:0.5,ave_lejana:0.5,rio:0.5,mar:0.5,ave_bosque_a:0.5,ave_bosque_b:0.5)";
        for campo in [
            "volumen",
            "viento",
            "ave_lejana",
            "rio",
            "mar",
            "ave_bosque_a",
            "ave_bosque_b",
        ] {
            for invalido in ["-0.1", "1.1", "NaN", "inf"] {
                let texto = valida.replace(&format!("{campo}:0.5"), &format!("{campo}:{invalido}"));
                assert!(
                    ConfiguracionAmbientes::interpretar(&texto)
                        .unwrap_err()
                        .contains(campo)
                );
            }
        }
        for texto in [
            valida.replace("viento:0.5,", ""),
            valida.replace("viento:0.5,", "viento:0.5,viento:0.2,"),
            valida.replace("viento:0.5,", "otro:0.5,"),
        ] {
            assert!(ConfiguracionAmbientes::interpretar(&texto).is_err());
        }
        assert!(ConfiguracionAmbientes::interpretar(&valida.replace("0.5", "0.0")).is_ok());
        assert!(ConfiguracionAmbientes::interpretar(&valida.replace("0.5", "1.0")).is_ok());
    }

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
