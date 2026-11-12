use super::*;
struct Favorable;
impl FuenteAzar for Favorable {
    fn elegir_indice(&mut self, n: NonZeroUsize) -> Result<usize, ErrorFuenteAzar> {
        Ok(n.get() - 1)
    }
}
#[test]
fn pasajes_integran_contenido_planificacion_y_relato_sin_barco_propio() {
    let c = crate::cargar_contenido(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
    )
    .unwrap();
    let mut p = Partida::con_azar(&c, Box::new(Favorable)).unwrap();
    for _ in 0..20 {
        if let VistaSesion::PresentandoEvento(v) = p.vista() {
            p.procesar(IntencionFrontend::ContinuarEvento { presentacion: v.id })
                .unwrap();
        } else {
            break;
        }
    }
    let origen = c.claves().lugar_por_clave("shahraz").unwrap();
    let destino = c.claves().lugar_por_clave("zarabad").unwrap();
    p.mundo.trasladar(p.mundo.actor_jugador(), origen).unwrap();
    assert!(p.mundo.ofrece_pasaje(origen, destino));
    assert!(
        !p.mundo
            .ofrece_pasaje(destino, c.claves().lugar_por_clave("nuris").unwrap())
    );
    p.procesar(IntencionFrontend::ComenzarViaje { destino })
        .unwrap();
    let mut vio_pasaje = false;
    let mut vio_mar = false;
    assert_eq!(
        p.mundo.recorrido_pasaje(origen, destino).unwrap(),
        &[
            origen,
            c.claves().lugar_por_clave("mar_del_viento").unwrap(),
            destino
        ]
    );
    for _ in 0..100 {
        let i = match p.vista() {
            VistaSesion::PresentandoAccion(f) => {
                if f.texto.contains("pasajes") {
                    vio_mar |= f.texto.contains("Mar del Viento");
                    vio_pasaje = true;
                    assert!(f.texto.contains("capitán"));
                    assert!(!f.detalle.join(" ").contains("interpreta el viento"));
                }
                IntencionFrontend::ContinuarPresentacion { fragmento: f.id }
            }
            VistaSesion::EsperandoDecision(d) => IntencionFrontend::ResponderDecision {
                decision: d.id,
                opcion: d.opciones[0].id,
            },
            VistaSesion::PresentandoEvento(v) => {
                IntencionFrontend::ContinuarEvento { presentacion: v.id }
            }
            VistaSesion::EsperandoDecisionEvento(v) => IntencionFrontend::ResponderEvento {
                presentacion: v.id,
                opcion: v.opciones[1].id,
            },
            VistaSesion::ProcesandoCierre { token } => IntencionFrontend::ContinuarCierre { token },
            VistaSesion::Disponible(_) => break,
            VistaSesion::Fatal(f) => panic!("{}", f.mensaje),
            _ => panic!("fase imprevista"),
        };
        p.procesar(i).unwrap();
    }
    assert!(vio_pasaje);
    assert!(vio_mar);
    assert_eq!(p.mundo.ubicacion_grupo(), destino);
}
