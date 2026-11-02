//! Selección de recursos gráficos; no cambia contenido ni reglas de la partida.

use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use desert_tiger_app::{ContenidoValidado, IdActor, RetratoActor, cargar_retrato};

use crate::recursos::ErrorRecursos;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum ModoGraficos {
    #[default]
    Predeterminado,
    Fuente,
    Ascii,
}

impl ModoGraficos {
    pub(crate) fn carpeta(self, raiz: &Path) -> Option<PathBuf> {
        let nombre = match self {
            Self::Predeterminado => return None,
            Self::Fuente => "fuente",
            Self::Ascii => "ascii",
        };
        Some(raiz.join("alternativos").join(nombre))
    }

    pub(crate) fn validar_glifos(
        self,
        glifos: impl IntoIterator<Item = char>,
    ) -> Result<(), ErrorRecursos> {
        if self == Self::Ascii
            && let Some(glifo) = glifos.into_iter().find(|c| !matches!(c, ' '..='~'))
        {
            return Err(ErrorRecursos(format!(
                "el dibujo alternativo ASCII contiene un carácter no permitido: {}",
                glifo.escape_default()
            )));
        }
        Ok(())
    }
}

pub(crate) fn cargar_retratos(
    raiz: &Path,
    modo: ModoGraficos,
    contenido: &ContenidoValidado,
) -> Result<HashMap<IdActor, RetratoActor>, ErrorRecursos> {
    let mut retratos = HashMap::new();
    let Some(carpeta) = modo.carpeta(raiz) else {
        return Ok(retratos);
    };
    for (id, clave) in contenido.claves_actores() {
        let ruta = carpeta.join("actors").join(format!("{clave}.ron"));
        match fs::metadata(&ruta) {
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(ErrorRecursos(format!("{}: {error}", ruta.display()))),
            Ok(_) => {}
        }
        let retrato = cargar_retrato(&ruta).map_err(|error| {
            let detalles = error
                .diagnosticos()
                .iter()
                .map(|d| d.detalle())
                .collect::<Vec<_>>()
                .join("; ");
            ErrorRecursos(format!("{}: {detalles}", ruta.display()))
        })?;
        modo.validar_glifos(
            retrato
                .graficos()
                .flat_map(|grafico| grafico.celdas().iter().map(|celda| celda.glifo)),
        )
        .map_err(|error| ErrorRecursos(format!("{}: {error}", ruta.display())))?;
        retratos.insert(id, retrato);
    }
    Ok(retratos)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_app::cargar_contenido;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CarpetaTemporal(PathBuf);

    impl CarpetaTemporal {
        fn nueva() -> Self {
            static NUMERO: AtomicUsize = AtomicUsize::new(0);
            let ruta = std::env::temp_dir().join(format!(
                "desert-tiger-retratos-alternativos-{}-{}",
                std::process::id(),
                NUMERO.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&ruta).unwrap();
            Self(ruta)
        }

        fn escribir(&self, modo: ModoGraficos, clave: &str, texto: &str) -> PathBuf {
            let carpeta = modo.carpeta(&self.0).unwrap().join("actors");
            fs::create_dir_all(&carpeta).unwrap();
            let ruta = carpeta.join(format!("{clave}.ron"));
            fs::write(&ruta, texto).unwrap();
            ruta
        }
    }

    impl Drop for CarpetaTemporal {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn contenido() -> ContenidoValidado {
        cargar_contenido(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base")).unwrap()
    }

    #[test]
    fn carpetas_y_ejemplo_distribuidos_se_pueden_cargar() {
        let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/terminal");
        for modo in [ModoGraficos::Fuente, ModoGraficos::Ascii] {
            crate::recursos::RecursosVisuales::cargar_modo(&raiz, modo).unwrap();
        }
        let retrato = cargar_retrato(&raiz.join("alternativos/retrato.ron.example")).unwrap();
        ModoGraficos::Ascii
            .validar_glifos(
                retrato
                    .graficos()
                    .flat_map(|grafico| grafico.celdas().iter().map(|celda| celda.glifo)),
            )
            .unwrap();
    }

    fn retrato(base: &str, expresion: &str, ojo: &str) -> String {
        format!(
            r#"(
            paleta: {{'p': (frente: "amarillo", fondo: None)}},
            base: (arte: "{base}", estilos: Some("p")),
            rostro: Some((origen: (x: 0, y: 0), expresiones: {{
                "music": (arte: "{expresion}", estilos: Some("p")),
            }})),
            parpadeo: Some((origen: (x: 0, y: 0), ojo_cerrado:
                (arte: "{ojo}", estilos: Some("p")))),
        )"#
        )
    }

    #[test]
    fn retratos_alternativos_por_clave_con_respaldo_y_modos_aislados() {
        let carpeta = CarpetaTemporal::nueva();
        let contenido = contenido();
        for modo in [ModoGraficos::Fuente, ModoGraficos::Ascii] {
            assert!(
                cargar_retratos(&carpeta.0, modo, &contenido)
                    .unwrap()
                    .is_empty()
            );
        }
        let principe = contenido
            .claves_actores()
            .find(|(_, clave)| *clave == "principe_perdido")
            .unwrap()
            .0;
        carpeta.escribir(
            ModoGraficos::Fuente,
            "principe_perdido",
            &retrato("\u{e000}", "\u{e001}", "\u{e002}"),
        );
        carpeta.escribir(
            ModoGraficos::Ascii,
            "principe_perdido",
            &retrato("o", "^", "-"),
        );
        // Un archivo ajeno al contenido seleccionado no debe cargarse.
        carpeta.escribir(ModoGraficos::Fuente, "actor_inexistente", "RON invalido");
        for (modo, esperado) in [
            (ModoGraficos::Fuente, '\u{e000}'),
            (ModoGraficos::Ascii, 'o'),
        ] {
            let cargados = cargar_retratos(&carpeta.0, modo, &contenido).unwrap();
            assert_eq!(cargados.len(), 1);
            assert_eq!(cargados[&principe].base().celdas()[0].glifo, esperado);
            assert_eq!(cargados[&principe].graficos().count(), 3);
        }
        let ruta = carpeta.escribir(ModoGraficos::Fuente, "principe_perdido", "RON invalido");
        assert!(
            cargar_retratos(&carpeta.0, ModoGraficos::Predeterminado, &contenido)
                .unwrap()
                .is_empty()
        );
        assert!(cargar_retratos(&carpeta.0, ModoGraficos::Ascii, &contenido).is_ok());
        let error = cargar_retratos(&carpeta.0, ModoGraficos::Fuente, &contenido).unwrap_err();
        assert!(error.to_string().contains(ruta.to_str().unwrap()));
    }

    #[test]
    fn ascii_valida_base_expresiones_y_parpadeo_del_retrato() {
        let carpeta = CarpetaTemporal::nueva();
        let contenido = contenido();
        for (base, expresion, ojo) in [("█", "^", "-"), ("o", "♪", "-"), ("o", "^", "▀")] {
            let ruta = carpeta.escribir(
                ModoGraficos::Ascii,
                "sirviente",
                &retrato(base, expresion, ojo),
            );
            let error = cargar_retratos(&carpeta.0, ModoGraficos::Ascii, &contenido).unwrap_err();
            assert!(error.to_string().contains("ASCII"));
            assert!(error.to_string().contains(ruta.to_str().unwrap()));
        }
    }
}
