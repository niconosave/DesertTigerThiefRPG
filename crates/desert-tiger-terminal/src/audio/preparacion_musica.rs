use super::partitura::Melodia;
use std::{
    path::PathBuf,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub(super) struct Repertorio {
    pub piezas: Vec<Arc<[f32]>>,
    pub audible: bool,
}

pub(super) type RepertorioCompartido = Arc<OnceLock<Result<Repertorio, String>>>;

impl Repertorio {
    fn nuevo(piezas: Vec<Arc<[f32]>>) -> Self {
        let audible = piezas.iter().any(|p| p.iter().any(|&m| m != 0.0));
        Self { piezas, audible }
    }

    #[cfg(test)]
    pub(super) fn listo(piezas: Vec<Arc<[f32]>>) -> RepertorioCompartido {
        Arc::new(OnceLock::from(Ok(Self::nuevo(piezas))))
    }
}

pub(super) struct TrabajoMusica {
    cancelado: Arc<AtomicBool>,
    repertorio: RepertorioCompartido,
    hilo: Option<JoinHandle<()>>,
}

impl TrabajoMusica {
    pub(super) fn iniciar(
        pendientes: Vec<(PathBuf, Melodia)>,
        repertorio: RepertorioCompartido,
    ) -> Result<Self, String> {
        Self::con_tarea(repertorio, move |cancelado| {
            let mut piezas = Vec::with_capacity(pendientes.len());
            for (ruta, melodia) in pendientes {
                let Some(muestras) = melodia
                    .renderizar_cancelable(cancelado)
                    .map_err(|e| format!("{}: {e}", ruta.display()))?
                else {
                    return Ok(None);
                };
                piezas.push(Arc::from(muestras));
            }
            if cancelado.load(Ordering::Relaxed) {
                return Ok(None);
            }
            Ok(Some(Repertorio::nuevo(piezas)))
        })
    }

    fn con_tarea(
        repertorio: RepertorioCompartido,
        tarea: impl FnOnce(&AtomicBool) -> Result<Option<Repertorio>, String> + Send + 'static,
    ) -> Result<Self, String> {
        let cancelado = Arc::new(AtomicBool::new(false));
        let cancelacion = cancelado.clone();
        let salida = repertorio.clone();
        let hilo = thread::Builder::new()
            .name("preparar-musica".into())
            .spawn(move || {
                let resultado = match tarea(&cancelacion) {
                    Ok(Some(piezas)) => Ok(piezas),
                    Ok(None) => return,
                    Err(e) => Err(e),
                };
                if !cancelacion.load(Ordering::Relaxed) {
                    let _ = salida.set(resultado);
                }
            })
            .map_err(|e| format!("no se pudo iniciar la preparación musical: {e}"))?;
        Ok(Self {
            cancelado,
            repertorio,
            hilo: Some(hilo),
        })
    }

    // Solo recoge un hilo que ya terminó; el bucle de interfaz nunca lo espera.
    pub(super) fn revisar(&mut self) -> Option<Result<(), String>> {
        if !self.hilo.as_ref()?.is_finished() {
            return None;
        }
        let hilo = self.hilo.take()?;
        if hilo.join().is_err() {
            let _ = self
                .repertorio
                .set(Err("falló el hilo de preparación musical".into()));
        }
        self.repertorio
            .get()
            .map(|r| r.as_ref().map(|_| ()).map_err(Clone::clone))
    }

    #[cfg(test)]
    pub(super) fn esperar(&mut self) {
        if let Some(hilo) = self.hilo.take() {
            hilo.join().unwrap();
        }
    }
}

impl Drop for TrabajoMusica {
    fn drop(&mut self) {
        self.cancelado.store(true, Ordering::Relaxed);
        if let Some(hilo) = self.hilo.take() {
            let _ = hilo.join();
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::super::{configuracion::ConfiguracionMusica, reproductor::ReproductorMusica};
    use super::*;
    use std::{sync::mpsc, time::Duration};

    #[test]
    fn preparar_no_bloquea_y_el_reproductor_espera_la_publicacion_completa() {
        let repertorio = RepertorioCompartido::default();
        let (comenzo, entrada) = mpsc::channel();
        let (continuar, permiso) = mpsc::channel();
        let mut trabajo = TrabajoMusica::con_tarea(repertorio.clone(), move |_| {
            comenzo.send(()).unwrap();
            permiso.recv().unwrap();
            Ok(Some(Repertorio::nuevo(vec![Arc::from(vec![0.2; 4800])])))
        })
        .unwrap();
        entrada.recv_timeout(Duration::from_secs(3)).unwrap();
        assert!(repertorio.get().is_none());
        assert!(trabajo.revisar().is_none());
        let mut config = ConfiguracionMusica::fixture();
        config.piezas = repertorio.clone();
        let mut reproductor = ReproductorMusica::nuevo(config, 48000);
        reproductor.ordenar(7);
        for _ in 0..1000 {
            assert_eq!(reproductor.siguiente(), 0.0);
        }
        assert_eq!(reproductor.estado(), 0);
        continuar.send(()).unwrap();
        trabajo.esperar();
        assert_eq!(repertorio.get().unwrap().as_ref().unwrap().piezas.len(), 1);
        // La solicitud que llegó temprano no se pierde ni publica música ficticia.
        reproductor.ordenar(7);
        for _ in 0..2000 {
            reproductor.siguiente();
        }
        assert!((reproductor.siguiente() - 0.1).abs() < f32::EPSILON);
        assert_eq!(reproductor.estado() >> 32, 7);
    }

    #[test]
    fn cerrar_cancela_y_recoge_el_trabajo_sin_publicar_resultados_parciales() {
        let repertorio = RepertorioCompartido::default();
        let (comenzo, entrada) = mpsc::channel();
        let (termino, salida) = mpsc::channel();
        let trabajo = TrabajoMusica::con_tarea(repertorio.clone(), move |cancelado| {
            comenzo.send(()).unwrap();
            while !cancelado.load(Ordering::Relaxed) {
                thread::yield_now();
            }
            termino.send(()).unwrap();
            Ok(None)
        })
        .unwrap();
        entrada.recv_timeout(Duration::from_secs(3)).unwrap();
        drop(trabajo);
        assert_eq!(salida.try_recv(), Ok(()));
        assert!(repertorio.get().is_none());
    }

    #[test]
    fn fallos_de_sintesis_o_hilo_se_notifican_una_sola_vez() {
        for panico in [false, true] {
            let repertorio = RepertorioCompartido::default();
            let mut trabajo = TrabajoMusica::con_tarea(repertorio.clone(), move |_| {
                if panico {
                    panic!("fallo simulado del sintetizador");
                }
                Err("partitura.ron: muestra no finita".into())
            })
            .unwrap();
            let resultado = loop {
                if let Some(r) = trabajo.revisar() {
                    break r;
                }
                thread::yield_now();
            };
            assert!(resultado.is_err());
            assert!(repertorio.get().unwrap().is_err());
            assert!(trabajo.revisar().is_none());
        }
    }
}
