use super::*;

pub(super) fn titulo_momento(momento: MomentoPaso) -> &'static str {
    match momento {
        MomentoPaso::Panorama => "El paso por delante",
        MomentoPaso::CaminoDificil => "Cada apoyo cuenta",
        MomentoPaso::Cornisa => "Alrededor de la gran pared",
        MomentoPaso::Campamento => "Una noche entre las montañas",
        MomentoPaso::Rodeos => "Las vueltas de la montaña",
        MomentoPaso::AbrirPaso => "Donde termina el camino",
        MomentoPaso::OtraVertiente => "La otra vertiente",
        MomentoPaso::Interrupcion => "Una pausa necesaria en el paso",
    }
}

impl Narrador<'_> {
    pub(super) fn relato_paso(&self, r: &RegistroEscena, p: &RegistroPasoMontanoso) -> String {
        let exito = p.chequeo.as_ref().is_none_or(|c| c.exito);
        let responsable = p
            .chequeo
            .as_ref()
            .map(|c| self.actor(c.responsable))
            .unwrap_or_default();
        let compania = if r.participantes.len() > 1 {
            "con tus compañeros"
        } else {
            "con tus bultos"
        };
        let animales = r.animales_carga > 0 || r.caballos > 0 || r.camellos > 0;
        let nombre_origen = self.lugar(p.origen);
        let nombre_destino = self.lugar(p.destino);
        let extremo = if nombre_origen == nombre_destino {
            "la otra vertiente del paso".to_owned()
        } else {
            nombre_destino
        };
        let extension = match p.cantidad_lugares {
            2 => "La estrechez del corredor apenas deja adivinar su otra salida.",
            3 | 4 => {
                "Una cresta oculta a la siguiente y obliga a imaginar el camino que queda detrás."
            }
            _ => {
                "Las crestas se suceden tan lejos que el recorrido parece perderse entre las alturas."
            }
        };
        let relato = match p.momento {
            MomentoPaso::Panorama => format!(
                "Desde {nombre_origen} contemplás el paso que se interna entre paredes de roca. {extension} Calculás {} de marcha para alcanzar {extremo}, sin contar los contratiempos: la altura y los apoyos inciertos exigirán cuidado, y las provisiones tendrán que alcanzar hasta el otro lado.",
                formatear(p.previsto)
            ),
            MomentoPaso::CaminoDificil => format!(
                "Avanzás {compania} por un terreno de piedras sueltas y apoyos estrechos, donde sostener el ritmo exige toda tu atención. {responsable} busca los pasos más firmes y conduce el esfuerzo. {}",
                if exito {
                    "Con paciencia lográs superar la pendiente sin perder tiempo adicional; el camino cede, aunque el esfuerzo ya se siente en las piernas."
                } else {
                    "La dificultad y el cansancio deshacen una y otra vez el ritmo de la marcha: tardás ocho horas más en superar el tramo, pero finalmente conseguís pasar."
                }
            ),
            MomentoPaso::Cornisa => {
                let resultado = if exito {
                    "Bordeás la pared con cuidado y conseguís pasar sin perder la carga.".into()
                } else if p.animales_perdidos > 0 && p.comida_perdida == [0; 4] {
                    "Un animal de carga pierde pie y se precipita. Ya no quedaba comida sobre su carga, pero la pérdida te deja con menos capacidad para llevar los bultos; el agua queda a salvo.".into()
                } else if p.animales_perdidos > 0 {
                    format!(
                        "Un animal de carga pierde pie y se precipita con la comida que transportaba. Perdés {}: el agua queda a salvo, pero ahora tendrás que afrontar el resto del paso con menos alimento y capacidad para llevar los bultos.",
                        if r.reservas.as_ref().is_some_and(|v| v.comidas == [0; 4]) {
                            "toda la comida que quedaba"
                        } else {
                            "parte de las provisiones"
                        }
                    )
                } else {
                    "La marcha se vuelve accidentada y los bultos obligan a recuperar el equilibrio, pero no llevás animales de carga que puedan caer: conseguís pasar sin pérdida de comida ni demora adicional.".into()
                };
                format!(
                    "Una montaña alta y empinada se alza ante vos como una gran pared. Bajo la guía de {responsable}, buscás cómo bordearla por una cornisa que deja ver el vacío. {resultado}"
                )
            }
            MomentoPaso::Campamento => {
                let comida = if r
                    .reservas
                    .as_ref()
                    .is_some_and(|v| v.comidas.iter().any(|&n| n > 0))
                {
                    "El olor de la comida caliente se mezcla con el humo; el sabor de cada bocado y el calor de las brasas te devuelven por un momento una calma sencilla."
                } else {
                    "El olor del humo y el calor de las brasas te ofrecen un poco de consuelo, aunque no queda comida para acompañar el descanso."
                };
                format!(
                    "Al llegar a {}, en el centro de este recorrido, te acomodás junto al fuego {compania}. {comida} La noche está nublada, pero entre dos nubes se abre un claro de estrellas: por un rato, las montañas dejan de ser solo esfuerzo y podés relajarte antes de seguir.",
                    self.lugar(p.lugar_confirmado)
                )
            }
            MomentoPaso::Rodeos => format!(
                "La salida parece cercana desde las alturas, pero las montañas te obligan a dar vuelta tras vuelta para encontrar por dónde continuar. {responsable} mantiene el rumbo mientras la distancia se resiste a disminuir. {}",
                if exito {
                    "Lográs completar el recorrido sin demora adicional, muy cansado después de tantos desvíos."
                } else {
                    "Un rodeo se prolonga más de lo previsto y perdés un día entero antes de volver a ganar camino hacia la salida."
                }
            ),
            MomentoPaso::AbrirPaso => format!(
                "El camino se deshace entre rocas y tenés que ingeniártelas para avanzar, saltando entre apoyos y buscando una continuidad que no se ve a primera vista. {responsable} organiza el intento{}; {}",
                if animales {
                    ", usando cuerdas para ayudar a pasar a los animales y sus cargas"
                } else {
                    " y asegura el paso de los bultos"
                },
                if exito {
                    "con cuidado conseguís abrirte paso y continuar por el terreno roto."
                } else {
                    "no encontrás cómo atravesar esa parte. Tenés que retroceder y buscar otra vuelta entre las montañas: el rodeo añade dos días al viaje, pero te permite continuar."
                }
            ),
            MomentoPaso::OtraVertiente => format!(
                "Desde una altura distinguís por fin el otro lado y reconocés que el cruce hasta {} ha terminado. Después de tanta piedra, desvíos y esfuerzo, la amplitud que se abre ante vos trae un alivio profundo: podés soltar la tensión de cada apoyo y mirar lo que viene con otro ánimo.",
                extremo
            ),
            MomentoPaso::Interrupcion => format!(
                "Al alcanzar {}, necesitás detener la marcha para revisar las provisiones y el peso que todavía podés llevar. La salida sigue por delante; antes de volver a internarte en el paso tendrás que resolver lo que falta para continuar.",
                self.lugar(p.lugar_confirmado)
            ),
        };
        if p.momento == MomentoPaso::Panorama {
            relato
        } else {
            format!(
                "{relato} Llevás {} dedicados a trasladarte por este paso.",
                formatear(p.traslado_acumulado)
            )
        }
    }

    pub(super) fn detalle_paso(&self, r: &RegistroEscena, p: &RegistroPasoMontanoso) -> String {
        let mut texto = self.relato_paso(r, p);
        texto.push_str(&format!("\n\nEpisodio: {} → {}; {} lugares. Previsión inicial de traslado: {}. Traslado acumulado: {}. Otras actividades: {}.",self.lugar(p.origen),self.lugar(p.destino),p.cantidad_lugares,formatear(p.previsto),formatear(p.traslado_acumulado),formatear(p.otras_actividades)));
        if let Some(c) = &p.chequeo {
            texto.push_str(&format!(
                "\n{}\nDemora adicional: {}.",
                self.chequeo(c),
                formatear(p.demora)
            ));
        }
        if p.animales_perdidos > 0 {
            texto.push_str(&format!("\nSe perdió {} animal de carga. Comida perdida: {} carnes secas, {} panes, {} quesos y {} pescados. El agua no se pierde.",p.animales_perdidos,p.comida_perdida[0],p.comida_perdida[1],p.comida_perdida[2],p.comida_perdida[3]));
        }
        if p.momento == MomentoPaso::Campamento {
            texto.push_str("\nEl descanso está incluido en la duración de la marcha: no añade tiempo, otra ración ni recuperación de estados.");
        }
        texto
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    #[test]
    fn campamento_y_perdidas_reflejan_reservas_reales_y_tiempo_del_episodio() {
        let (contenido, mundo, mut r) = super::super::pruebas::escenario();
        let narrador = Narrador(&contenido);
        let tiempo = |dias| desert_tiger_core::TiempoAccion {
            duracion: desert_tiger_core::Duracion::desde_dias(dias),
            fraccion: desert_tiger_core::FraccionDia::CERO,
        };
        let mut p = RegistroPasoMontanoso {
            momento: MomentoPaso::Campamento,
            origen: r.lugar,
            destino: r.destino_final,
            cantidad_lugares: 3,
            lugar_confirmado: r.lugar,
            previsto: tiempo(20),
            traslado_acumulado: tiempo(10),
            otras_actividades: tiempo(0),
            chequeo: None,
            demora: tiempo(0),
            animales_perdidos: 0,
            comida_perdida: [0; 4],
        };
        r.tarjeta = ClaveTarjeta::PasoMontanoso;
        r.reservas = Some(ReservasViaje::default());
        r.contenido = ContenidoEscena::PasoMontanoso(p.clone());
        let texto = narrador.redactar(&r);
        assert!(texto.texto.contains("no queda comida"));
        assert!(texto.texto.contains("estrellas") && texto.texto.contains("10 días"));
        assert!(!texto.texto.contains('\n'));
        assert_eq!(texto.expresion, Some("happy"));
        assert!(texto.detalle.join(" ").contains("no añade tiempo"));
        r.reservas.as_mut().unwrap().comidas[0] = 10;
        let con_comida = narrador.redactar(&r);
        assert!(con_comida.texto.contains("sabor de cada bocado"));
        p.momento = MomentoPaso::Cornisa;
        p.animales_perdidos = 1;
        p.chequeo = Some(
            resolver_chequeo(
                &mundo,
                mundo.actor_jugador(),
                AccionParticular::Andar,
                10,
                &mut super::super::pruebas::Cero,
            )
            .unwrap(),
        );
        r.reservas = Some(ReservasViaje::default());
        r.contenido = ContenidoEscena::PasoMontanoso(p.clone());
        let sin_comida = narrador.redactar(&r);
        assert!(sin_comida.texto.contains("Ya no quedaba comida"));
        assert!(!sin_comida.texto.contains("Perdés toda"));
        p.comida_perdida = [20, 0, 0, 0];
        r.contenido = ContenidoEscena::PasoMontanoso(p);
        let perdida = narrador.redactar(&r);
        assert!(perdida.texto.contains("toda la comida que quedaba"));
        assert_eq!(perdida.expresion, Some("scared"));
    }
}
