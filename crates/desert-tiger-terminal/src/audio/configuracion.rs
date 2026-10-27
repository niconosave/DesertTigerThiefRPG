use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(super) struct ConfiguracionAudio {
    pub fogata: ConfiguracionFogataAudio,
    pub ambientes: ConfiguracionAmbientes,
    pub musica: ConfiguracionMusica,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DatosMusica {
    partitura: Option<String>,
    partituras: Option<Vec<String>>,
    volumen: f32,
    espera_inicial_segundos: (u32, u32),
    pausa_entre_piezas_segundos: (u32, u32),
}

impl DatosMusica {
    fn nombres(&self) -> Result<Vec<&str>, String> {
        match (&self.partitura, &self.partituras) {
            (Some(nombre), None) => Ok(vec![nombre.as_str()]),
            (None, Some(nombres)) if !nombres.is_empty() && nombres.len() <= 16 => {
                Ok(nombres.iter().map(String::as_str).collect())
            }
            _ => Err(
                "se espera partitura o partituras (1–16 archivos), sin combinar ambos campos"
                    .into(),
            ),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "audio"), allow(dead_code))]
pub(super) struct ConfiguracionMusica {
    pub agenda: crate::musica_campamento::AgendaMusica,
    pub volumen: f32,
    #[cfg(feature = "audio")]
    pub piezas: Vec<std::sync::Arc<[f32]>>,
}

impl ConfiguracionMusica {
    pub(super) fn cargar(raiz: &Path) -> Result<Self, String> {
        let ruta = raiz.join("campamento.ron");
        let cargar = || -> Result<Self, String> {
            let datos =
                Self::interpretar(&std::fs::read_to_string(&ruta).map_err(|e| e.to_string())?)?;
            #[cfg(feature = "audio")]
            let piezas = {
                let mut piezas = Vec::new();
                let mut total = 0_usize;
                for nombre in datos.nombres()? {
                    let muestras = super::partitura::sintetizar_archivo(&raiz.join(nombre))?;
                    total += muestras.len();
                    if total > super::partitura::FRECUENCIA as usize * 600 {
                        return Err(
                            "partituras: el repertorio supera 10 minutos de audio preparado".into(),
                        );
                    }
                    piezas.push(muestras);
                }
                piezas
            };
            Ok(Self {
                agenda: crate::musica_campamento::AgendaMusica {
                    inicial: datos.espera_inicial_segundos,
                    pausa: datos.pausa_entre_piezas_segundos,
                },
                volumen: datos.volumen,
                #[cfg(feature = "audio")]
                piezas,
            })
        };
        cargar().map_err(|e| format!("{}: {e}", ruta.display()))
    }

    fn interpretar(texto: &str) -> Result<DatosMusica, String> {
        let datos: DatosMusica = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(texto)
            .map_err(|e| e.to_string())?;
        let mut vistos = std::collections::BTreeSet::new();
        for nombre in datos.nombres()? {
            if nombre.is_empty()
                || nombre.contains('/')
                || nombre.contains('\\')
                || Path::new(nombre).extension().and_then(|s| s.to_str()) != Some("ron")
                || Path::new(nombre).file_name().and_then(|s| s.to_str()) != Some(nombre)
            {
                return Err(
                    "partitura: se espera un nombre de archivo .ron sin directorios".into(),
                );
            }
            if !vistos.insert(nombre) {
                return Err(format!("partituras: archivo repetido {nombre}"));
            }
        }
        if !datos.volumen.is_finite() || !(0.0..=1.0).contains(&datos.volumen) {
            return Err("volumen: se espera un número finito entre 0 y 1".into());
        }
        for (campo, (min, max)) in [
            ("espera_inicial_segundos", datos.espera_inicial_segundos),
            (
                "pausa_entre_piezas_segundos",
                datos.pausa_entre_piezas_segundos,
            ),
        ] {
            if min == 0 || min > max || max > 3600 {
                return Err(format!("{campo}: se espera 1 <= mínimo <= máximo <= 3600"));
            }
        }
        Ok(datos)
    }

    #[cfg(all(feature = "audio", test))]
    pub(super) fn fixture() -> Self {
        Self {
            agenda: crate::musica_campamento::AgendaMusica {
                inicial: (5, 10),
                pausa: (40, 75),
            },
            volumen: 0.5,
            piezas: vec![std::sync::Arc::from(vec![0.2; 4800])],
        }
    }
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
    fn musica_valida_rutas_intervalos_y_campos_estrictos() {
        let base = "(partitura:\"pieza.ron\",volumen:0.5,espera_inicial_segundos:(5,10),pausa_entre_piezas_segundos:(40,75))";
        assert!(ConfiguracionMusica::interpretar(base).is_ok());
        for ruta in [
            "../pieza.ron",
            "/pieza.ron",
            "sub/pieza.ron",
            "sub\\\\pieza.ron",
            "pieza.wav",
            "",
        ] {
            assert!(ConfiguracionMusica::interpretar(&base.replace("pieza.ron", ruta)).is_err());
        }
        for texto in [
            base.replace("0.5", "NaN"),
            base.replace("0.5", "1.1"),
            base.replace("(5,10)", "(10,5)"),
            base.replace("(5,10)", "(0,10)"),
            base.replace("(40,75)", "(40,3601)"),
            base.replace("volumen:0.5,", ""),
            base.replace("volumen:0.5,", "volumen:0.5,volumen:0.2,"),
            base.replace("volumen:0.5,", "extra:1,volumen:0.5,"),
        ] {
            assert!(ConfiguracionMusica::interpretar(&texto).is_err(), "{texto}");
        }
    }

    #[cfg(feature = "audio")]
    #[test]
    fn musica_oficial_se_sintetiza_antes_de_abrir_salida() {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/audio/musica");
        let musica = ConfiguracionMusica::cargar(&raiz).unwrap();
        assert!(
            musica
                .piezas
                .iter()
                .all(|p| !p.is_empty() && p.iter().all(|m| m.is_finite() && m.abs() <= 0.8))
        );
        assert!(
            musica
                .piezas
                .iter()
                .all(|p| p.iter().any(|m| m.abs() > 0.01))
        );
        assert!(musica.piezas.iter().all(|p| p.len() > 48000));
    }

    #[test]
    fn repertorio_admite_lista_y_rechaza_vacios_duplicados_y_dos_formatos() {
        let base = "(partitura:\"pieza.ron\",volumen:0.5,espera_inicial_segundos:(5,10),pausa_entre_piezas_segundos:(40,75))";
        let lista = base.replace(
            "partitura:\"pieza.ron\"",
            "partituras:[\"una.ron\",\"otra.ron\"]",
        );
        assert_eq!(
            ConfiguracionMusica::interpretar(&lista)
                .unwrap()
                .nombres()
                .unwrap(),
            ["una.ron", "otra.ron"]
        );
        for campo in [
            "partituras:[]",
            "partituras:[\"una.ron\",\"una.ron\"]",
            "partituras:[\"../una.ron\"]",
            "partitura:\"pieza.ron\",partituras:[\"otra.ron\"]",
        ] {
            assert!(
                ConfiguracionMusica::interpretar(&base.replace("partitura:\"pieza.ron\"", campo))
                    .is_err()
            );
        }
        assert!(
            ConfiguracionMusica::interpretar(&base.replace("partitura:\"pieza.ron\",", ""))
                .is_err()
        );
    }

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
