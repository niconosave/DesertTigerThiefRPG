use crate::contenido::ContenidoValidado;
use crate::tiempo::formatear;
use desert_tiger_core::acciones::*;
use desert_tiger_core::{IdActor, IdLugar, TiempoAccion};

mod pasos_montanosos;
mod resumen;

pub(crate) fn lugar_paisaje(registro: &RegistroEscena) -> IdLugar {
    match &registro.contenido {
        ContenidoEscena::Desplazamiento(d) => d.origen,
        _ => registro.lugar,
    }
}

pub(crate) struct Narrador<'a>(pub &'a ContenidoValidado);
pub(crate) struct TextoEscena {
    pub titulo: String,
    pub texto: String,
    pub detalle: Vec<String>,
    pub expresion: Option<&'static str>,
}

// Dirección dramática de Viajar: se decide aquí, junto al relato, no en los
// chequeos reutilizables ni a partir del texto que termina leyendo el jugador.
fn expresion_escena(contenido: &ContenidoEscena) -> Option<&'static str> {
    match contenido {
        ContenidoEscena::Llegada { .. } => Some("happy"),
        ContenidoEscena::PasoMontanoso(p) => match p.momento {
            MomentoPaso::Campamento | MomentoPaso::OtraVertiente => Some("happy"),
            _ if p.chequeo.as_ref().is_some_and(|c| !c.exito) => Some("scared"),
            _ => None,
        },
        ContenidoEscena::Desplazamiento(d) => {
            if d.incidente.is_some() || d.chequeos.iter().any(|c| !c.exito) {
                Some("scared")
            } else {
                Some("happy")
            }
        }
        ContenidoEscena::Preparacion { chequeo, .. } => {
            Some(if chequeo.exito { "happy" } else { "scared" })
        }
        ContenidoEscena::Forrajeo { chequeos, .. } => Some(if chequeos.iter().any(|c| c.exito) {
            "happy"
        } else {
            "scared"
        }),
        ContenidoEscena::Incompleto { .. } => Some("scared"),
        ContenidoEscena::Plan { .. }
        | ContenidoEscena::CargaInicial { .. }
        | ContenidoEscena::Sobrecarga { .. } => None,
    }
}
fn litros(ml: u64) -> String {
    if ml.is_multiple_of(1000) {
        format!("{}", ml / 1000)
    } else {
        format!("{:.3}", ml as f64 / 1000.0)
    }
}
pub(crate) fn reservas(r: &ReservasViaje) -> String {
    format!(
        "{} piezas de carne seca, {} panes, {} lonjas de queso y {} piezas de pescado salado; {} botas, con {} litros de agua",
        r.comidas[0],
        r.comidas[1],
        r.comidas[2],
        r.comidas[3],
        r.botas.len(),
        litros(r.botas.iter().map(|&v| u64::from(v)).sum())
    )
}
impl Narrador<'_> {
    pub fn actor(&self, id: IdActor) -> String {
        self.0
            .presentacion()
            .actor(id)
            .map(|a| a.nombre().to_owned())
            .unwrap_or_else(|| "Viajero desconocido".into())
    }
    pub fn lugar(&self, id: IdLugar) -> String {
        self.0
            .presentacion()
            .lugar(id)
            .map(|l| {
                if l.nombre().is_empty() {
                    "un tramo del camino".to_owned()
                } else {
                    l.nombre().to_owned()
                }
            })
            .unwrap_or_else(|| "Lugar desconocido".into())
    }
    fn chequeo(&self, c: &ResultadoChequeo) -> String {
        let capacidades = c
            .modificadores
            .iter()
            .map(|m| format!("{} {:+}", m.caracteristica, m.bono))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{} intenta {:?}: dado {}, {}total {}, dificultad {}: {}.",
            self.actor(c.responsable),
            c.accion,
            c.natural,
            if capacidades.is_empty() {
                String::new()
            } else {
                format!("{capacidades}; ")
            },
            c.ajustado,
            c.dificultad,
            if c.exito { "éxito" } else { "fallo" }
        )
    }
    fn configuracion(&self, c: &ConfiguracionVista, visible: bool) -> String {
        format!(
            "{} animales de carga. Sobre los actores: {} g; límite conjunto: {} g; exceso: {} g. Demora añadida: {}.{}\nProvisiones: {}. Suplemento: {}.",
            c.animales,
            c.peso_actores,
            c.limite_actores,
            c.exceso_gramos,
            formatear(c.demora),
            if visible {
                format!(" Tiempo total previsto: {}.", formatear(c.tiempo))
            } else {
                String::new()
            },
            reservas(&c.reservas),
            reservas(&c.suplemento)
        )
    }
    fn deliberacion(&self, r: &RegistroEscena, propuestas: &[PropuestaRuta]) -> String {
        let companeros = r
            .participantes
            .iter()
            .skip(1)
            .map(|&a| self.actor(a))
            .collect::<Vec<_>>();
        let mut partes = vec![if companeros.is_empty() {
            format!(
                "Te detenés en {} a pensar cómo alcanzar {}. Repasás las distancias y buscás en tu memoria un camino que puedas seguir.",
                self.lugar(r.lugar),
                self.lugar(r.destino_final)
            )
        } else {
            format!(
                "Te detenés en {} y discutís con {} cómo alcanzar {}. Entre recuerdos del territorio y dudas sobre lo que les espera, sopesan por dónde abrirse camino.",
                self.lugar(r.lugar),
                companeros.join(" y "),
                self.lugar(r.destino_final)
            )
        }];
        if propuestas.is_empty() {
            partes.push(if companeros.is_empty() {
                "No conocés lo bastante la ruta para idear un itinerario fiable; si partís, tendrás que orientarte sobre la marcha.".into()
            } else {
                "Nadie logra idear un itinerario fiable: los recuerdos no bastan para enlazar el camino. Si deciden partir, tendrán que orientarse sobre la marcha.".into()
            });
        }
        for p in propuestas {
            let mut actores = std::collections::BTreeMap::<IdActor, Vec<PerfilRuta>>::new();
            for respaldo in &p.respaldos {
                let perfiles = actores.entry(respaldo.actor).or_default();
                if !perfiles.contains(&respaldo.perfil) {
                    perfiles.push(respaldo.perfil);
                }
            }
            for (actor, perfiles) in actores {
                let motivos = perfiles
                    .iter()
                    .map(|perfil| match perfil {
                        PerfilRuta::Rapida => "seguir el recorrido que parece más rápido",
                        PerfilRuta::Conocidos => "enlazar los parajes conocidos",
                        PerfilRuta::Montana => "cruzar las montañas",
                        PerfilRuta::Caballo => {
                            "aprovechar el terreno donde los caballos pueden avanzar mejor"
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" y ");
                let sujeto = if Some(&actor) == r.participantes.first() {
                    "Vos proponés".into()
                } else {
                    format!("{} propone", self.actor(actor))
                };
                let referencia = p
                    .ruta
                    .iter()
                    .skip(1)
                    .find(|&&l| l != r.destino_final)
                    .map(|&l| format!(", pasando por {}", self.lugar(l)))
                    .unwrap_or_default();
                partes.push(format!("{sujeto} {motivos}{referencia}."));
            }
        }
        partes.push("Te tomás el día para valorar las posibilidades antes de decidir si emprender el viaje.".into());
        partes.join(" ")
    }
    pub(crate) fn describir_propuesta(&self, p: &PropuestaRuta) -> (String, String, Vec<String>) {
        let nombre = |perfil| match perfil {
            PerfilRuta::Rapida => "Ruta rápida",
            PerfilRuta::Conocidos => "Por lugares conocidos",
            PerfilRuta::Montana => "Por las montañas",
            PerfilRuta::Caballo => "Favorable al caballo",
        };
        let respaldos: Vec<_> = p
            .respaldos
            .iter()
            .map(|r| {
                let lugares = r
                    .lugares_relevantes
                    .iter()
                    .map(|&l| self.lugar(l))
                    .collect::<Vec<_>>()
                    .join(", ");
                let rasgos = r
                    .caracteristicas_habilitantes
                    .iter()
                    .map(|c| c.como_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{}: {}{}{}",
                    self.actor(r.actor),
                    nombre(r.perfil),
                    if rasgos.is_empty() {
                        String::new()
                    } else {
                        format!(" ({rasgos})")
                    },
                    if lugares.is_empty() {
                        String::new()
                    } else {
                        format!("; reconoce {lugares}")
                    }
                )
            })
            .collect();
        let titulo = format!(
            "{} - {}",
            p.respaldos
                .first()
                .map_or("Camino propuesto", |r| nombre(r.perfil)),
            formatear(p.previsto)
        );
        let detalle = format!(
            "Tiempo de traslado previsto, sin carga ni imprevistos: {}.\nRecorrido: {}.\n{}",
            formatear(p.previsto),
            p.ruta
                .iter()
                .map(|&l| self.lugar(l))
                .collect::<Vec<_>>()
                .join(" → "),
            respaldos.join("\n")
        );
        (titulo, detalle, respaldos)
    }
    pub fn redactar(&self, r: &RegistroEscena) -> TextoEscena {
        let mut detalle = Vec::new();
        let mut texto=match &r.contenido {
            ContenidoEscena::Plan{propuestas}=>self.deliberacion(r,propuestas),
            ContenidoEscena::Preparacion{chequeo,configuracion}=>format!("Antes de partir de {}, {} calcula y reúne comida, agua y suministros.\n{}\nConsiguen {} animales de carga, {} caballos y {} camellos para el recorrido previsto.\n{}",self.lugar(r.lugar),self.actor(chequeo.responsable),self.chequeo(chequeo),configuracion.animales,r.caballos,r.camellos,reservas(&configuracion.reservas)),
            ContenidoEscena::CargaInicial{base,alternativa}=>format!("Durante la preparación, {} evalúan cómo llevar las provisiones. El peso obliga a decidir.\nEvitar la demora:\n{}\nLlevar un animal menos:\n{}",r.participantes.iter().map(|&id|self.actor(id)).collect::<Vec<_>>().join(", "),self.configuracion(base,r.plan_visible),self.configuracion(alternativa,r.plan_visible)),
            ContenidoEscena::Incompleto{inicial:true,..}=>"Conseguís menos provisiones de las que habían calculado. No sabés si alcanzarán. Podés partir con lo reunido o cancelar y devolver los suministros.".into(),
            ContenidoEscena::Incompleto{falta_comida,falta_agua,..}=>format!("En {} queda una ración diaria para todos, pero la previsión restante señala insuficiencia de {}{}. Debés decidir si seguir o detener el viaje.",self.lugar(r.lugar),if *falta_comida{"alimento "}else{""},if *falta_agua{"agua"}else{""}),
            ContenidoEscena::PasoMontanoso(p)=>self.detalle_paso(r,p),
            ContenidoEscena::Desplazamiento(d)=>{
                let lider=self.actor(d.chequeos[0].responsable);let viajeros=r.participantes.iter().map(|&id|self.actor(id)).collect::<Vec<_>>().join(", ");
                let inicio=match d.medio {
                    Medio::Pie=>format!("Dejás atrás {} junto a {viajeros}. {lider} lee el terreno, busca referencias y abre camino hacia {}.",self.lugar(d.origen),self.lugar(d.destino)),
                    Medio::Caballo=>format!("Al ritmo de los cascos, dejás atrás {} con {viajeros}. Bajo la guía de {lider}, las distancias ceden y el camino se abre en grandes vistas hacia {}.",self.lugar(d.origen),self.lugar(d.destino)),
                    Medio::Camello=>format!("El paso largo y el balance de los camellos acompañan a {viajeros}. {lider} sostiene un avance regular desde {} hacia {}; las referencias se acercan lentamente desde la altura de las monturas.",self.lugar(d.origen),self.lugar(d.destino)),
                    Medio::Barco=>format!("Con {viajeros}, dejás {} a bordo. {lider} interpreta el viento y gobierna las velas hacia {}. {}",self.lugar(d.origen),self.lugar(d.destino),if d.chequeos.iter().any(|c|c.exito){"El casco corta las olas con firmeza y el horizonte devuelve amplitud y libertad al viaje."}else{"El agua y el cielo exigen maniobras cuidadosas; el grupo se abre paso con seriedad, sin perder la esperanza."}),
                };
                let mut t=inicio;
                for (actor,estados) in &r.estados_iniciales {
                    t.push_str(&format!("\n{} comienza este tramo con {}.",self.actor(*actor),estados.join(" y ")));
                }
                for (i,m) in d.mitades.iter().enumerate(){
                    t.push_str(&format!("\nEn {}, la mitad {} requiere {}: base {} horas{},{}{}{}; carga añadida {}.",self.lugar(m.lugar),if i==0{"de salida"}else{"de llegada"},formatear(m.duracion),m.horas_base,if m.pequena{"; Pequeño divide esa base por dos"}else if m.vasta{"; Vasto duplica esa base"}else{"; tamaño normal"},if m.terreno_lento{" el terreno exige un avance lento y cuidadoso;"}else{""},if m.animal_limitante{" los animales ralentizan el avance;"}else{""},if m.estado_horas>0{format!(" los estados limitantes añaden {} horas;",m.estado_horas)}else{String::new()},formatear(m.demora_carga)));
                    t.push_str(&format!("\n{}",self.chequeo(&d.chequeos[i])));
                    if m.medio==Medio::Pie && d.medio!=Medio::Pie {
                        t.push_str("\nAl faltar una montura, un integrante continúa a pie y el grupo acompaña ese paso. Se conserva el chequeo de Cabalgar ya realizado.");
                    }
                    if let Some(inc)=&d.incidente && inc.lugar==m.lugar {t.push_str(&format!("\n{}",self.incidente(d.medio,inc)));}
                }
                if let Some((rumbo,viento,ajuste))=d.viento{t.push_str(&format!("\nRumbo {}, viento hacia {}: {} horas sobre la conexión. El viento se fijó al comenzar esta escena.",direccion(rumbo),direccion(viento),ajuste));}
                for c in &r.cambios {if let desert_tiger_core::CambioConfirmado::ConocimientoRegistrado{actor,lugar}=c{t.push_str(&format!("\n{} aprende a reconocer {} y obtiene Conoce con ese Lugar.",self.actor(*actor),self.lugar(*lugar)));}}
                if r.duracion.duracion.es_cero()&&r.duracion.fraccion==desert_tiger_core::FraccionDia::CERO && !d.mitades.is_empty(){t.push_str("\nEl viento permite cubrir la conexión sin tiempo adicional.");}
                t.push_str(&format!("\nAl cerrar este momento, el grupo queda en {}.",self.lugar(r.lugar)));t
            }
            ContenidoEscena::Forrajeo{chequeos,obtenido,busca_comida,busca_agua}=>{
                let mut t=format!("En {} necesitás conseguir {}{}. El grupo explora las oportunidades del entorno para sostener el camino.\n",self.lugar(r.lugar),if *busca_comida{"comida "}else{""},if *busca_agua{"agua"}else{""});
                for c in chequeos{t.push_str(&self.chequeo(c));t.push('\n');}
                t.push_str(&format!("En seis horas reunís {}.",reservas(obtenido)));t
            }
            ContenidoEscena::Sobrecarga{configuracion,lote}=>format!("La composición o las provisiones cambiaron en {}. La carga supera la capacidad y las reservas no cubren la demora restante.\n{}\nPara eliminar el exceso podés abandonar {} piezas de carne seca, {} panes, {} lonjas de queso, {} pescados y las botas {:?}, o cancelar.",self.lugar(r.lugar),self.configuracion(configuracion,r.plan_visible),lote.comidas[0],lote.comidas[1],lote.comidas[2],lote.comidas[3],lote.botas),
            ContenidoEscena::Llegada{lugares}=>format!("{} {} se abre ante vos. El grupo alcanza su destino después de {}.",if *lugares>=5{"Después de tanta distancia y esfuerzo, la llegada tiene otro peso:"}else{"Por fin,"},self.lugar(r.destino_final),formatear(r.total)),
        };
        // Las cantidades y el tiempo siempre proceden del registro ya confirmado.
        if r.duracion.duracion != desert_tiger_core::Duracion::CERO
            || r.duracion.fraccion != desert_tiger_core::FraccionDia::CERO
        {
            texto.push_str(&format!(
                "\n\nEste momento tomó {}. Tiempo total: {}.",
                formatear(r.duracion),
                formatear(r.total)
            ));
        }
        if r.tarjeta != ClaveTarjeta::Llegada {
            if r.consumo.jornadas > 0 {
                texto.push_str(&format!("\nDurante el recorrido se completaron {} períodos de consumo: {} piezas de carne seca, {} panes, {} lonjas de queso, {} pescados y {} litros de agua consumidos.",r.consumo.jornadas,r.consumo.comidas[0],r.consumo.comidas[1],r.consumo.comidas[2],r.consumo.comidas[3],litros(r.consumo.agua_ml)));
            } else if r.reservas.is_some() {
                texto.push_str(&format!("\nNo se completó otra jornada de consumo; desde la última ración se acumulan {}.",formatear(r.horas_desde_racion)));
            }
        }
        if let Some(t) = r.consumo.sin_alimento {
            if t.duracion.es_cero() && t.fraccion == desert_tiger_core::FraccionDia::CERO {
                texto.push_str(
                    "\nAl cerrar este momento faltó alimento para completar la última ración.",
                );
            } else {
                texto.push_str(&format!(
                    "\nEl grupo afrontó hambre durante {}.",
                    formatear(t)
                ));
            }
        }
        if let Some(t) = r.consumo.sin_agua {
            if t.duracion.es_cero() && t.fraccion == desert_tiger_core::FraccionDia::CERO {
                texto.push_str(
                    "\nAl cerrar este momento faltó agua para completar la última ración.",
                );
            } else {
                texto.push_str(&format!("\nEl grupo afrontó sed durante {}.", formatear(t)));
            }
        }
        if let Some(v) = &r.reservas {
            texto.push_str(&format!("\nReservas al cierre: {}.", reservas(v)));
            if v.comidas == [0; 4] && r.consumo.sin_alimento.is_none() {
                texto.push_str(
                    " El alimento se agotó al pagar la última ración, sin carencia previa.",
                );
            }
            if v.botas.iter().all(|&x| x == 0) && r.consumo.sin_agua.is_none() {
                texto.push_str(" El agua se agotó al pagar la última ración, sin carencia previa.");
            }
        }
        if let Some(cierre) = r.cierre {
            let razon = match cierre {
                CierreViaje::Llegada => None,
                CierreViaje::SinRecursos => Some(
                    "No reunís una ración diaria de ambos recursos para todos. El viaje termina aquí.",
                ),
                CierreViaje::InterrumpidoPorSeparacion => Some(
                    "Elegís detener el viaje para buscar al compañero. Se contabiliza el tiempo de toda la conexión, pero quedás en el Lugar del incidente.",
                ),
                CierreViaje::Naufragio => Some("El naufragio pone fin al viaje en esta costa."),
                CierreViaje::Cancelado => Some("El viaje termina por tu decisión."),
            };
            if let Some(razon) = razon {
                texto.push_str(&format!("\n{razon}"));
            }
        }
        // El desglose conserva el desarrollo completo, pero la lectura inicial
        // prioriza lo ocurrido y sus consecuencias. Ambos respetan la privacidad.
        let lugar_descrito = matches!(
            &r.contenido,
            ContenidoEscena::Desplazamiento(_) | ContenidoEscena::Llegada { .. }
        )
        .then(|| lugar_paisaje(r));
        let descripcion = lugar_descrito
            .and_then(|id| self.0.presentacion().lugar(id))
            .map(|l| l.descripcion())
            .filter(|d| !d.is_empty());
        let resumen = self.resumen(r);
        // El paisaje sitúa al jugador antes de narrar el desplazamiento.
        // Es composición del relato, sin memoria de lugares ya descritos.
        let resumen = if let Some(descripcion) = descripcion {
            texto = format!("{descripcion}\n\n{texto}");
            format!("{descripcion}\n\n{resumen}")
        } else {
            resumen
        };
        detalle.push(texto);
        TextoEscena {
            titulo: match &r.contenido {
                ContenidoEscena::PasoMontanoso(p) => pasos_montanosos::titulo_momento(p.momento),
                _ => titulo(r.tarjeta),
            }
            .into(),
            texto: resumen,
            detalle,
            expresion: expresion_escena(&r.contenido),
        }
    }
    fn incidente(&self, medio: Medio, i: &IncidenteResuelto) -> String {
        let actor = i
            .afectado
            .map(|id| self.actor(id))
            .unwrap_or_else(|| "el grupo".into());
        let relato=match i.tipo {
            Incidente::Agotamiento=>format!("{actor} se agota; esperás a que pueda retomar el paso, sin adquirir Cansado."),
            Incidente::Esguince=>format!("Un mal apoyo lastima a {actor}; recibe Esguince solo si no tenía ya Esguince o Cansado. Si ya estaba afectado, su estado no empeora."),
            Incidente::RumboPerdido=>"Las referencias dejan de coincidir. Los rodeos cansan a quienes no tenían Esguince ni Cansado; los demás conservan su estado.".into(),
            Incidente::Separacion=>format!("Al reagruparse, falta {actor}. Su rastro quedó en {}. Podés detener el viaje para buscarlo o continuar sin él; la búsqueda posterior queda fuera de este viaje.",self.lugar(i.lugar)),
            Incidente::Enfermedad=>"Una enfermedad desconocida te obliga a detenerte un día. Te recuperás, aunque queda Cansado si no tenías ya un estado limitante.".into(),
            Incidente::Obstinada=>format!("La montura de {actor} se obstina en alejarse del rumbo; corregir los rodeos demora la marcha."),
            Incidente::Detenida=>"Una montura se niega a seguir. El grupo espera y trabaja hasta reanudar la marcha.".into(),
            Incidente::Asustada=>format!("Una montura se asusta, corre en otra dirección y derriba a {actor}. Reagruparse exige tiempo."),
            Incidente::CrisisMontura=>format!("{} El grupo espera dos días. La recuperación obtiene {}: {}.",if medio==Medio::Camello{"Un animal que nadie alcanza a ver pica al camello."}else{"Una fiera ataca al caballo y lo deja herido."},i.recuperacion.unwrap_or(0),if i.recuperacion.is_some_and(|r|r>=8){"la montura se recupera"}else{"la montura muere"}),
            Incidente::MuerteMontura=>"La montura muere durante el trayecto.".into(),
            Incidente::Olas=>"Las olas apartan la proa; las correcciones cuestan tiempo.".into(),
            Incidente::Lluvia=>"La lluvia oscurece cubierta y velas; sostenés el esfuerzo hasta recuperar el avance.".into(),
            Incidente::SinViento=>"Las velas caen flojas. El barco permanece dos días esperando un impulso sobre el mar.".into(),
            Incidente::Naufragio=>"Una tormenta fuerte hace naufragar el barco. El viaje termina en la costa; no se alcanza el destino previsto.".into(),
            Incidente::PorLaBorda=>format!("Una sacudida arroja a {actor} por la borda. Sale del grupo y queda en una costa."),
        };
        format!("En {}: {relato} Incidente: {}. Demora añadida: {}.{}",self.lugar(i.lugar),i.dado,formatear(i.demora),i.monturas_restantes.map(|n|format!(" Quedan {n} monturas de esta especie; se comprueba si alcanza una por integrante.")).unwrap_or_default())
    }
}
fn direccion(i: u8) -> &'static str {
    [
        "este", "noreste", "norte", "noroeste", "oeste", "suroeste", "sur", "sureste",
    ]
    .get(usize::from(i))
    .copied()
    .unwrap_or("indefinido")
}

fn titulo(t: ClaveTarjeta) -> &'static str {
    match t {
        ClaveTarjeta::SinPlan => "Partir sin plan",
        ClaveTarjeta::PropuestaRuta => "Una propuesta de ruta",
        ClaveTarjeta::Preparacion => "Preparar el viaje",
        ClaveTarjeta::CargaInicial => "Cómo llevar las provisiones",
        ClaveTarjeta::Incompleto => "Provisiones incompletas",
        ClaveTarjeta::Andar => "Abrirse camino a pie",
        ClaveTarjeta::Caballo => "Al ritmo de los caballos",
        ClaveTarjeta::Camello => "Sobre el paso de los camellos",
        ClaveTarjeta::Navegar => "Una travesía en barco",
        ClaveTarjeta::PasoMontanoso => "Cruzar el paso montañoso",
        ClaveTarjeta::Forrajeo => "Buscar alimento y agua",
        ClaveTarjeta::Sobrecarga => "Una carga difícil de llevar",
        ClaveTarjeta::Llegada => "Llegada",
    }
}
pub(crate) fn opcion(o: OpcionViaje) -> &'static str {
    match o {
        OpcionViaje::ElegirRuta(_) => "Elegir este camino",
        OpcionViaje::Improvisar => "Improvisar el camino",
        OpcionViaje::Cancelar => "Cancelar el viaje",
        OpcionViaje::AceptarDemora => "Aceptar la demora",
        OpcionViaje::EvitarDemora => "Evitar la demora por carga",
        OpcionViaje::Continuar => "Continuar",
        OpcionViaje::Buscar => "Detenerse para buscarlo",
        OpcionViaje::Descartar => "Abandonar provisiones",
    }
}
pub(crate) fn tiempo_texto(t: TiempoAccion) -> String {
    formatear(t)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use desert_tiger_core::{FraccionDia, InstanteAccion};

    pub(super) struct Cero;
    impl desert_tiger_core::azar::FuenteAzar for Cero {
        fn elegir_indice(
            &mut self,
            _: std::num::NonZeroUsize,
        ) -> Result<usize, desert_tiger_core::azar::ErrorFuenteAzar> {
            Ok(0)
        }
    }

    pub(super) fn escenario() -> (ContenidoValidado, desert_tiger_core::Mundo, RegistroEscena) {
        let contenido = crate::cargar_contenido(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content/base"),
        )
        .unwrap();
        let (mut mundo, _) = contenido.instanciar_estado().unwrap();
        let actor = mundo.actor_jugador();
        let destino = destinos_disponibles(&mundo, actor).unwrap()[0];
        let rumbos = crate::contexto_viaje::construir(&mundo, contenido.presentacion()).unwrap();
        let contexto = ContextoAccion {
            inicio: InstanteAccion {
                fecha: mundo.fecha(),
                fraccion: FraccionDia::CERO,
            },
            rumbos: &rumbos,
        };
        let inicio = iniciar(
            &mut mundo,
            SolicitudAccion::Viajar { actor, destino },
            &contexto,
            &mut Cero,
        )
        .unwrap();
        let registro = match inicio.paso {
            PasoAccion::Escena(r) => r,
            PasoAccion::Decision(d) => d.escena,
            _ => panic!(),
        };
        (contenido, mundo, registro)
    }

    #[test]
    fn deliberacion_distingue_companeros_soledad_y_propuestas() {
        let (contenido, mut mundo, mut registro) = escenario();
        let narrador = Narrador(&contenido);
        let sin_plan = narrador.deliberacion(&registro, &[]);
        assert!(sin_plan.contains("discutís con"));
        assert!(sin_plan.contains("Nadie logra idear"));
        registro.participantes.truncate(1);
        let solo = narrador.deliberacion(&registro, &[]);
        assert!(!solo.contains("discutís con") && !solo.contains("Nadie"));
        assert!(solo.contains("No conocés lo bastante"));
        let actor = mundo.actor_jugador();
        mundo
            .aplicar_lote(desert_tiger_core::LoteOperaciones::uno(
                desert_tiger_core::OperacionMundo::AnadirCaracteristicaActor {
                    actor,
                    caracteristica: desert_tiger_core::Caracteristica::nueva("viajero".into())
                        .unwrap(),
                },
            ))
            .unwrap();
        let origen = mundo.ubicacion_grupo();
        let destino = destinos_disponibles(&mundo, actor)
            .unwrap()
            .into_iter()
            .find(|l| !mundo.lugares_conectados(origen).unwrap().contains(l))
            .unwrap();
        let rumbos = crate::contexto_viaje::construir(&mundo, contenido.presentacion()).unwrap();
        let contexto = ContextoAccion {
            inicio: InstanteAccion {
                fecha: mundo.fecha(),
                fraccion: FraccionDia::CERO,
            },
            rumbos: &rumbos,
        };
        let inicio = iniciar(
            &mut mundo,
            SolicitudAccion::Viajar { actor, destino },
            &contexto,
            &mut Cero,
        )
        .unwrap();
        let PasoAccion::Decision(d) = inicio.paso else {
            panic!()
        };
        let ContenidoEscena::Plan { ref propuestas } = d.escena.contenido else {
            panic!()
        };
        assert!(!propuestas.is_empty());
        let texto = narrador.deliberacion(&d.escena, propuestas);
        assert!(texto.contains("Vos proponés"));
        assert!(texto.contains("recorrido que parece más rápido"));
        assert!(!texto.contains("Nadie logra idear"));
        for respaldo in propuestas
            .iter()
            .flat_map(|p| &p.respaldos)
            .filter(|r| r.actor != actor)
        {
            assert!(texto.contains(&narrador.actor(respaldo.actor)));
        }
    }

    #[test]
    fn sobrecarga_oculta_prevision_y_monturas_conservan_relato_propio() {
        let (contenido, mundo, mut registro) = escenario();
        let actor = mundo.actor_jugador();
        let destino = destinos_disponibles(&mundo, actor).unwrap()[0];
        registro.plan_visible = false;
        registro.tarjeta = ClaveTarjeta::Sobrecarga;
        registro.contenido = ContenidoEscena::Sobrecarga {
            configuracion: ConfiguracionVista {
                reservas: ReservasViaje::default(),
                animales: 0,
                exceso_gramos: 1,
                peso_actores: 20_001,
                limite_actores: 20_000,
                tiempo: TiempoAccion {
                    duracion: desert_tiger_core::Duracion::desde_dias(123),
                    fraccion: FraccionDia::CERO,
                },
                demora: TiempoAccion {
                    duracion: desert_tiger_core::Duracion::CERO,
                    fraccion: FraccionDia::CERO,
                },
                suplemento: ReservasViaje::default(),
            },
            lote: Default::default(),
        };
        let narrador = Narrador(&contenido);
        let texto = narrador.redactar(&registro);
        assert!(!texto.texto.contains("123 días"));
        assert!(!texto.detalle.join(" ").contains("Tiempo total previsto"));
        registro.plan_visible = true;
        assert!(
            narrador
                .redactar(&registro)
                .detalle
                .join(" ")
                .contains("123 días")
        );
        let chequeo =
            resolver_chequeo(&mundo, actor, AccionParticular::Cabalgar, 15, &mut Cero).unwrap();
        let desplazamiento = Desplazamiento {
            medio: Medio::Caballo,
            origen: registro.lugar,
            destino,
            chequeos: [chequeo.clone(), chequeo],
            mitades: vec![],
            incidente: None,
            viento: None,
        };
        registro.tarjeta = ClaveTarjeta::Caballo;
        registro.contenido = ContenidoEscena::Desplazamiento(desplazamiento.clone());
        let caballo = narrador.redactar(&registro);
        registro.tarjeta = ClaveTarjeta::Camello;
        registro.contenido = ContenidoEscena::Desplazamiento(Desplazamiento {
            medio: Medio::Camello,
            ..desplazamiento
        });
        let camello = narrador.redactar(&registro);
        assert_ne!(caballo.titulo, camello.titulo);
        assert!(caballo.texto.contains("cascos"));
        assert!(camello.texto.contains("balance de los camellos"));
        assert_eq!(camello.expresion, Some("scared"));
        if let ContenidoEscena::Desplazamiento(d) = &mut registro.contenido {
            for chequeo in &mut d.chequeos {
                chequeo.exito = true;
            }
        }
        assert_eq!(narrador.redactar(&registro).expresion, Some("happy"));
        registro.contenido = ContenidoEscena::Incompleto {
            inicial: false,
            falta_comida: true,
            falta_agua: false,
        };
        assert_eq!(narrador.redactar(&registro).expresion, Some("scared"));
        registro.contenido = ContenidoEscena::Llegada { lugares: 2 };
        assert_eq!(narrador.redactar(&registro).expresion, Some("happy"));
        registro.contenido = ContenidoEscena::Plan { propuestas: vec![] };
        assert_eq!(narrador.redactar(&registro).expresion, None);
    }
}
