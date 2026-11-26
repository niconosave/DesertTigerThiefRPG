# Base espacial y planes de encuentros

| Campo | Valor |
|---|---|
| Estado | Base implementada técnicamente (052–056); revisión integrada 057 pendiente |
| Versión | 1.0 |
| Fecha | 2026-11-24 |
| Mecánicas | [Espacio, planes y sistemas](../../diseno/encuentros-espacio-planes-y-sistemas.md), versión 0.3 |
| Ejecución | [Etapa y bloques de trabajo](../../planes/base-espacial-y-planes-encuentros.md) |

## 1. Autoridad y alcance

Este contrato sustituye el modelo local del contrato
[10](./10-encuentros-y-atacar.md) para la próxima etapa. Conserva el reglamento
actual de pulsos simultáneos, golpes, reacciones, lesiones y tiempo. El espacio,
la dirección de fichas, la vista y los cierres se reemplazan por lo aquí definido.
El contrato [11](./11-rescatar-personas.md) y el componente
[30](../componentes/30-objetos-y-ubicaciones-compartidas.md) siguen gobernando los
efectos de libertad y posesión salvo las adaptaciones expresamente descritas aquí.

Se desarrolla un único modelo nuevo. Se traducen los siete perfiles y los eventos
actuales sin mantener el motor anterior, un adaptador al tablero plano o dos
proyecciones de reglas. Las distribuciones pueden cambiar; se conservan intención
y consecuencias. Los pasos internos pueden dejar temporalmente sin funcionar los
encuentros. La aceptación jugable corresponde al cierre de la etapa integrada.

Incluye volumen y corte visual, contexto mínimo, identidades estables, puerta
manual, planes genéricos y cierre causal. No incluye generación modular adaptable,
mapas persistentes, combate dinámico, sigilo individual, tamaños grandes, física
estructural, calor, fuego, salto, escalada, equipo ni creación de planes desde Lua.
El componente [29](../componentes/29-escenarios-volumetricos-modulares.md)
permanece como dirección posterior de generación; no amplía este alcance.

Los nombres de tipos y entradas siguientes son el contrato a implementar, no API
que ya exista. El contenido actual no se presenta como compatible sin traducción.

## 2. Responsabilidades y módulos

Se conserva el workspace y no se añade un crate para el encuentro.

| Ubicación | Responsabilidad |
|---|---|
| core::encuentros | Fachada de creación, avance, decisiones y resultados. |
| core::encuentros::espacio | Volumen, apoyo, ocupación, contacto, conexiones y navegación. |
| core::encuentros::elementos | Elementos definidos/probables, puerta e índices de ocupación. |
| core::encuentros::contexto | Espacios, accesos, salida y referencias semánticas. |
| core::encuentros::generacion | Recetas sencillas para los perfiles existentes y fixture de dos alturas. |
| core::encuentros::planes | Planes genéricos, roles, prioridades, progreso y bloqueo. |
| core::encuentros::comportamientos | Estado de cometidos y propuesta de intenciones particulares. |
| core::encuentros::reglas | Resolución simultánea, puerta, golpes, reacción y auxilio. |
| core::encuentros::resultado | Solicitudes causales de conclusión y resultados por entidad. |
| app::sesion::ataque | Ejecución activa, candidatos, RNG, efectos del Mundo y cierre temporal. |
| app::encuentro | Configuración y proyección pública filtrada del encuentro. |
| app::eventos | Adaptación de las intervenciones locales existentes. |
| terminal::encuentro | Proyección por corte y dibujo con caracteres. |
| terminal::interfaz | Corte, cámara, reproducción y entrada del jugador. |

El módulo actual encuentros.rs se descompone en un directorio. Mantener una
fachada de importación no implica mantener las estructuras ni reglas anteriores.
Viajar conserva su ejecución. Partida mantiene exclusión entre viaje, encuentro,
espera y presentación narrativa, sin generalizar todo el motor de acciones.

Mundo conserva identidad, relaciones y estados persistentes. No recibe coordenadas
locales de combate. El escenario no conoce controles, terminal ni reglas de campaña.

La fachada core sustituida expone crear(EntradaEncuentro, azar_generacion),
avanzar(azar_resolucion), solicitar_retirada, reanudar y responder_decision.
EntradaEncuentro contiene perfil, propósito/objetivo tipados, lugar de origen y
participantes con vínculo mundial y estado vigente. avanzar devuelve
SalidaEncuentro::Pulso, ::Decision, ::Bloqueado o ::Conclusion; el núcleo no
retorna tokens de pantalla. La app llama a estas operaciones solo sobre candidatos
y añade los tokens públicos. No se expone finalizar sin causa.

## 3. Identidades y coordenadas

PosicionLocal contiene x, y, z enteros no negativos. z es altura física discreta,
no número nominal de piso. Se retiran area/nivel del modelo anterior; un espacio
semántico es IdEspacio, no otro sistema de coordenadas. Un sótano ocupa alturas
inferiores dentro del volumen; no necesita coordenadas negativas.

Se usan IDs locales tipados distintos: IdFicha, IdElementoLocal, IdEspacio,
IdConexion, IdPista, IdPlan e IdComportamiento. No son índices públicos de vectores.
Son estables, no se reutilizan durante la ejecución y solo tienen sentido junto
con el ID del encuentro. Los vínculos con IdActor e IdObjeto son explícitos.
Un tesoro no se representa mediante un actor ficticio.

Las referencias de transportista, objetivo, ejecutor de un suceso y roles usan
estos IDs. Eliminar una presencia no cambia los IDs de las restantes. La primera
etapa no necesita reciclaje de IDs ni un almacén genérico de entidades.

FichaLocal conserva vínculo IdActor, posición, estado de golpes y capacidad, bando,
transportista opcional y situación de salida. BotinLocal conserva vínculo
IdObjeto, ubicación mundial inicial, posición, transportista y extracción.
Las fichas capaces bloquean su volumen; los cuerpos muertos/incapacitados no
bloquean, como en el reglamento actual. Un transportado no ocupa otro volumen y
el botín en el suelo no impide caminar. Su representación no los convierte en
otra ficha activa al compartir columna.

Dimensiones iniciales: ancho/profundidad 31 por 17 para los perfiles actuales y
altura de ocho vóxeles. Máximos: 64 por 64 por 16, 32 fichas, 512 elementos,
128 espacios, 256 conexiones, 512 pistas, 64 planes y 128 comportamientos activos.
Las multiplicaciones y conversiones se comprueban antes de reservar memoria.
Una receta que exceda límites se rechaza; no se recorta silenciosamente.

El pie del perfil básico está en z y ocupa las celdas z y z+1 en su columna.
Requiere apoyo en z-1; no hay posición caminable con z=0. No se fija equivalencia
en metros. Todas las fichas adaptadas usan huella 1 por 1 y altura 2.
La primera capa de suelo habitual es z=0 y los pies están en z=1.
Un piso elevado puede estar en z=3 y soportar pies en z=4; su espesor y la altura
libre dejan físicamente espacio para transitar debajo.

## 4. Volumen y elementos

VolumenLocal almacena una grilla densa acotada. El índice privado es
((z * profundidad) + y) * ancho + x. Ningún plan, comportamiento o frontend calcula
ese índice para consultar reglas. Acceden por las operaciones del apartado 5.

VoxelLocal distingue Aire y Solido. El sólido lleva MaterialLocal y AparienciaLocal;
estas propiedades no incorporan por sí mismas resistencia, combustión o calor.
La materia está en el volumen; las fichas y elementos móviles tienen ocupación
superpuesta. Los índices espaciales derivados no son una segunda fuente de estado.

ElementoLocal contiene ID, clase, volumen o posición de presencia, estado,
apariencia, propiedades y definición Definido o Probable. Puede tener un vínculo
opcional a Objeto del Mundo. Las clases iniciales son PuertaManual, Obstaculo y
PresenciaProbable; paredes y pisos inmóviles pueden ser terreno sólido sin crear
un objeto mundial por cada vóxel.

PuertaManual referencia una conexión, un vano vertical de altura dos y su estado
Abierta/Cerrada. Cerrada ocupa ese vano y obstruye paso, contacto y observación.
Abierta deja el vano libre; no se simula el barrido lateral de una hoja.
La apertura no modifica el suelo que proporciona apoyo.
No hay cerradura, daño de puerta ni interacción térmica inicial.

Obstaculo declara ocupación y apariencia; todavía no se empuja o destruye mediante
un intento genérico. PresenciaProbable declara identidad, envolvente y propiedades
observables que una concreción debe respetar. No es terreno intercambiable sin
límites: no puede cambiar retrospectivamente un elemento ya identificado.
La primera etapa admite su representación pero no añade una API narrativa nueva
para convertirla en cortina incendiada.

Se reconstruyen los índices afectados al confirmar cambios de elementos o terreno.
Abrir una puerta no copia su estado a otra matriz de transitabilidad permanente.
Un elemento vinculado al Mundo conserva esa identidad cuando pasa a llevarlo una
ficha. Al extraerse se confirma LlevadoPor hacia el líder, según el contrato 30.

## 5. Consultas y contexto

Las entradas espaciales son funciones del modelo y devuelven resultados tipados,
no prosa ni un booleano ambiguo. El contrato mínimo es:

| Consulta | Entrada y resultado |
|---|---|
| voxel | Posición; dato del volumen o FueraDeLimites. |
| puede_ocupar | Perfil, posición y exclusión de su propia ficha; apoyo, altura y ocupación válidos o impedimento. |
| vecinos_caminables | Perfil y origen; pasos ordinarios legales con su destino. |
| contacto | Dos fichas/posiciones; alcance cercano y ausencia de obstrucción. |
| puede_interactuar | Ficha y elemento; contacto válido con la cara del elemento. |
| conexiones_desde | Espacio y conocimiento disponible; conexiones y estados consultables. |
| opciones_de_paso | Origen, perfil y política; movimiento, puerta manual operable o transición explícita. |
| buscar_ruta | Ficha, conjunto de destinos y política; recorrido, SinRuta o límite agotado. |
| validar_geometria | Escenario candidato; errores de apoyo, ocupación, referencias o dimensiones. |

Los ocho vecinos ordinarios conservan z. Se comprueba el volumen completo en
destino y los laterales de una diagonal; no se cortan esquinas cerradas.
El contacto cercano exige pies en la misma altura y laterales libres.
La interacción puede alcanzar el vano cerrado sin exigir que el actor ocupe ese
vano; solo se excluye el elemento destinatario de la prueba de contacto.
No se alcanza otra planta por proximidad X/Y.

ConexionLocal identifica espacios extremos, puntos de entrada/salida, recorrido
transitable y elemento de control opcional. Una escalera inicial es una conexión
con secuencia explícita de apoyos, cuyos pasos pueden cambiar z. No concede
movimiento vertical libre. Cada paso consume un pulso como el movimiento actual.
La primera escalera no requiere simular pendiente continua ni atacar entre sus
alturas. Los destinos y volúmenes de cada paso se validan al generar y al ejecutar.

Cada paso vertical avanza una casilla cardinal en planta y como máximo un vóxel
en altura; no admite saltos entre extremos lejanos ni subida en una columna sin
conexión. Al subir se comprueba el volumen intermedio elevado sobre el origen;
al bajar, el volumen intermedio sobre el destino. Así la cabeza no atraviesa un
techo aunque los dos apoyos extremos sean válidos. Los pasos diagonales de una
escalera se dejan fuera de esta etapa.

ContextoLocal mantiene pistas Espacio, Conexion y Salida. Referencias nombradas
apuntan a posiciones, elementos o conexiones mediante un enum tipado. Para Gorun
se conservan entrada, acceso y brecha_norte/sur; una brecha indica la base z=1
del vano que puede despejarse. Los nombres no son coordenadas reconstruidas en Lua.

Los datos de contexto se derivan de geometría y elementos. Un cambio incrementa
revision_geometria, actualiza conexiones afectadas e invalida rutas anteriores.
También cambia revision_conocimiento cuando se descubre otro espacio.
Una ruta guarda ambas revisiones. La ocupación dinámica se valida en cada paso.

La navegación usa Dijkstra determinista sobre pasos conocidos con costes positivos:
mover un paso cuesta una oportunidad; abrir una puerta y cruzar cuesta dos.
Solo existen esos métodos y las transiciones explícitas. No busca escaladas,
saltos, destrucciones ni soluciones hipotéticas. Empates por posición y conexión
tienen orden fijo. Una ruta es una propuesta; no evita reacciones ni ocupación.

El cálculo produce MapaRutas por ficha y revisión evaluada: distancias y
predecesores hacia todos los candidatos conocidos. La asignación de roles y la
reconstrucción del próximo paso consultan ese mismo mapa; no ejecutan una búsqueda
por cada pareja de participante y cometido. buscar_ruta es una consulta de ese
resultado. La búsqueda se limita como indica el apartado 14.

Si no hay ruta por ocupación temporal, se consideran otros destinos válidos de
contacto o se espera/replantea. No se confunde ese impedimento con incapacidad para
cumplir la misión. El límite agotado se informa por separado de SinRuta.

El conocimiento inicial conserva la distinción actual entre perfiles abiertos y
recintos. Abiertos publican sus espacios iniciales; Recinto y Palacio publican
el espacio de entrada, conexiones ya perceptibles y el objetivo localizable.
Se descubre otro espacio al entrar por su conexión abierta. La regla x>15 se retira.
Si el objetivo no tiene posición conocida, el plan elige una conexión conocida
hacia un espacio no explorado: menor coste accesible y luego ID de conexión.
La conexión no tiene que ser adyacente al cuarto actual: se permiten rutas a
través de salas conocidas para alcanzar una frontera conocida. Así, descubrir
un salón con un integrante no deja sin destino a quienes todavía están detrás.
Alcanza su umbral, abre la puerta si procede y cruza el primer paso autorizado
por esa conexión; al confirmarlo descubre el espacio. No inspecciona su contenido
para elegir. Si no hay objetivos ni fronteras alcanzables se bloquea con motivo.
La memoria de fronteras visitadas evita alternar indefinidamente entre dos salas.
Una puerta cerrada no revela el interior por tener una referencia contextual.
Esto es revelación por espacios, no percepción individual ni simulación de sonido.

ConocimientoLocal distingue los dos bandos actuales, Grupo y Oposicion. La
oposición conoce la geometría de sus espacios defendidos, no la posición secreta
de cada visitante. Ambos calculan espacios observables con la regla del apartado
12. Los planes solo reciben su ContextoDecision filtrado; no consultan la posición
real de un blanco oculto para elegir métodos o roles. La ruta usa geometría y
estados conocidos; el resolver comprueba la realidad al intentar cada paso.
Descubrir un impedimento actualiza el conocimiento relevante antes de replantear.

## 6. Generación y traducción de escenarios

La acción general entrega PerfilEscenario, propósito tipado, participantes y
situación mundial. El generador produce geometría, elementos, contexto, posiciones
iniciales y referencias. Su RNG es distinto del de resolución.

Se traducen Recinto, Campamento, Palacio, Ciudad, Bosque, Montana y Playa.
Pueden conservar su planta o cambiarla para satisfacer volumen y acceso.
Piso habitual en z=0, paredes y obstáculos con volumen suficiente para bloquear
altura dos, fichas y botín apoyados en z=1. Los vacíos retiran apoyo; no son
vóxeles desconocidos. No se añaden techos a todos los perfiles por obligación.

Palacio incorpora una puerta manual en un acceso a un salón del objetivo.
Las rutas habituales pueden utilizarla sin un guion de resultado; cualquier
incursión apropiada puede abrirla. La puerta sirve como prueba genérica y no como
un rompecabezas con solución exclusiva.

Una receta pequeña DosAlturas, usada en verificación y pruebas de integración,
contiene piso inferior, piso superior, hueco, techo y una conexión ascendente.
Debe demostrar superposición y tránsito; no añade un Lugar al mapa ni un generador
complejo. Es accesible mediante una configuración de encuentro de prueba opcional.

Las recetas reservan la salida, los puntos de acción y al menos una ruta viable
con los métodos disponibles, considerando puertas operables. Se valida que cada
ficha tenga apoyo y altura libre y que el objetivo no ocupe una pared.
El número de intentos acotados de generación es ocho; si no hay una receta válida
se devuelve ErrorGeneracion. No se publica un escenario incompleto.

Persistencia solo durante el encuentro. Un encuentro nuevo consulta Mundo para
no regenerar muertos, cautivos ya extraídos ni bienes que ya lleva el grupo.
No se obliga a conservar el mismo plano en visitas posteriores.

Configuración RON conserva claves, listas objetivos/rescates/recuperaciones y
escenarios actuales. Se admite DosAlturas en escenarios de prueba. No se agregan
matrices3D a los archivos de actores, ni un archivo persistente por ciudad.
Las geometrías se construyen por recetas en esta etapa.

Las tres listas son opcionales y omitidas valen vacío. Cada entrada requiere
actor en objetivos; actor y captor en rescates; objeto y custodio en recuperaciones.
defensores es opcional y vale vacío. escenario es opcional, con Automatico por
defecto; admite Recinto, Campamento, Palacio, Ciudad, Bosque, Montana, Playa y
DosAlturas. Automatico conserva la elección por características del Lugar.
No se aceptan campos desconocidos. Se validan claves existentes del tipo indicado,
objetivos únicos por lista, defensores únicos y hasta ocho por entrada.
Captor/custodio no se repite como defensor. Presencia y capacidad se revalidan al abrir.

Ejemplo válido para una configuración de prueba aislada:

```ron
(
    objetivos: [
        (actor: "miraz", escenario: DosAlturas, defensores: []),
    ],
)
```

Ejemplos inválidos: usar objeto en lugar de actor dentro de objetivos; declarar
escenario: Palacio3D, que no existe; repetir al custodio en defensores; o indicar
una clave inexistente. Se rechaza el contenido antes de iniciar partida.

## 7. Planes, roles y prioridades

PlanActivo contiene ID, tipo de plan, prioridad, propósito, participantes
elegibles, roles, fase, estado y memoria de resultados. EstadoPlan es
EnEjecucion, Suspendido, Bloqueado, Cumplido o Cancelado.
Su consulta es pura. Sus decisiones solo se calculan al avanzar el encuentro.

Se implementan tipos finitos en Rust, sin intérprete de programas ni compositor
Lua: IncursionAtacar, IncursionRescatar, IncursionRobar, Oposicion y Retirada.
El propósito usa ObjetivoEncuentro::Actor o ::Objeto, separado del tipo de escenario.
El propósito inicial y su resultado se conservan aunque cambie el vigente.

El plan genérico recorre preparación existente, aproximación, oposición,
acción sobre el objetivo y salida según condiciones, no por un contador rígido.
Preparación conserva diez minutos sin inventar equipo nuevo. Derrotar al objetivo
no cierra desde el tablero: lleva al plan a resolver su salida o pedir conclusión.

Los roles iniciales son Actuante, Auxiliador, Evacuado y Oponente.
Se eligen entre fichas válidas por capacidad actual, disponibilidad, ruta viable,
distancia de ruta y finalmente ID estable. No se deduce aptitud por el nombre.
Para auxiliares se considera transporte existente antes de asignar otro.
La asignación evita duplicar recogidas o reservar a dos transportistas para el
mismo incapacitado.

La prioridad de autoridad es PrioridadPlan; se distingue de OrdenResolucion,
sorteado para conflictos de movimiento y reacciones. Valores iniciales:
misión y oposición 10, retirada voluntaria 20. La comparación es descendente;
empates por orden de creación del plan. Solo un plan efectivo dirige cada ficha.
No se puede ganar iniciativa o más acciones aumentando PrioridadPlan.

Los roles imprescindibles se recalculan cuando cambia la disponibilidad. Si no
hay sustituto compatible se bloquea la parte dependiente. Las labores
independientes pueden continuar. Un nuevo rol no confirma acciones que no ejecutó
y un efecto continuo no sobrevive al abandono de quien lo sostenía.

Retirada suspende la misión, conserva sus resultados y asigna evacuación y auxilio.
Reanudar cancela esa retirada y revalida la misión si no fue cancelada o imposible.
No se resucita un propósito abandonado mediante una prioridad menor.
La primera etapa no agrega decisiones de moral o fatiga para huir automáticamente.

Los cambios internos se notifican mediante sucesos de plan con ID, causa y
participantes. La futura integración narrativa podrá observarlos; no se añade aún
un filtro o comando Lua de plan. Un plan de espera debe conservar una condición
concreta de continuación. Ausencia de órdenes válidas se diagnostica, no dispara
un ataque genérico universal.

## 8. Comportamientos y resolución

ComportamientoActivo guarda ID, plan propietario, cometido, participantes,
destinos/referencias, progreso y resultados. El repertorio inicial:
Alcanzar, AfrontarOposicion, ActuarSobreObjetivo, Auxiliar, Evacuar y Esperar.
Abrir una puerta operable es parte del recorrido de Alcanzar/Evacuar.
No se añaden Barricar, Sigilo o Escalar como implementaciones vacías.

Cada comportamiento propone IntentoFicha a partir de la instantánea y su rol.
IntentoFicha admite Esperar, Mover, Atacar, Recoger, Liberar, TomarObjeto,
AbrirPuerta, CerrarPuerta y Salir. Usa IDs estables; no altera Mundo ni geometría
al elegir. La interfaz técnica principal es:

- Encuentro::proponer_direccion(): roles y comportamientos candidatos.
- ComportamientoActivo::proponer(contexto): intenciones y estado de progreso.
- resolver_pulso(snapshot, intenciones, azar): cambios y sucesos confirmables.
- Encuentro::reaccionar(sucesos): progreso de planes y solicitudes causales.

Son responsabilidades separadas, no llamadas públicas que el frontend encadena.
El avance de Encuentro las ordena. Los algoritmos existentes de rodear a un aliado,
elegir posiciones de contacto, recoger heridos y volver a la salida se traducen.

El resolver conserva el reglamento de golpes/reacciones y usa una única
instantánea previa para todos. No se elimina una intención ya elegida porque el
adversario actuó primero en el orden de iteración. La reacción que acierta puede
interrumpir movimiento según su regla vigente. Una ficha propone a lo sumo una
acción por pulso.

Abrir/Cerrar requiere capacidad, contacto válido con la puerta y plan efectivo.
Abrir se confirma al final del pulso: nadie cruza una puerta todavía cerrada en
su snapshot inicial. Para cerrar, el vano debe quedar libre y no haber sido
atravesado por un movimiento confirmado en ese pulso. Si está ocupado, falla sin
expulsar ni dañar. La elegibilidad se evalúa contra el estado inicial: pedir un
estado ya vigente produce SinCambio, no permite encadenar otra transición. Por
puerta se confirma como máximo un cambio en un pulso. Intentos iguales eligen un
ejecutor por OrdenResolucion y registran a los demás como concurrentes; todos
consumen su oportunidad. Abrir y cerrar no pueden aplicarse sucesivamente en ese
pulso. Un rechazo consume la oportunidad, no vuelve a tirar ni reintenta
infinitamente. El suceso identifica al ejecutor y al elemento reales.

La entrada core solicitar_puerta(ficha, elemento, estado) prepara una interacción
para el siguiente pulso y la liga al plan efectivo del participante. No actúa desde
el frontend ni desde Lua en esta etapa. Si otro plan obtiene autoridad, la petición
no se ejecuta hasta que corresponda. Cada intento ejecutado consume su solicitud;
una petición sin cambio no cierra una puerta que acaba de abrirse en ese mismo pulso.
El efecto de cierre se valida tras resolver los movimientos, incluyendo cruces
confirmados por el vano, sin modificar esos recorridos retrospectivamente.

El registro de cada interacción conserva ID de suceso, actor ejecutor, objetivo,
posición3D y resultado tipado. Liberado y ObjetoRecogido identifican al ejecutor
real. Una lesión y un impedimento transitorio no son el mismo estado.

## 9. Avance y confirmación atómica

La app conserva EncuentroActivo: ID de ejecución, secuencia de continuación,
motor, RNG de resolución, frame pendiente, fronteras narrativas y efectos aplicados.
Se mantienen las intenciones actuales con id/paso, añadiendo las decisiones
explícitas de evacuación incompleta. Los controles visuales no son intenciones
de simulación.

Orden de un avance:
1. Validar fase, ID y secuencia antes de consumir azar.
2. Si hay frame pendiente, publicarlo sin ejecutar otro pulso.
3. Atender frontera narrativa pendiente; no calcular el futuro que el evento podría cambiar.
4. Clonar candidatos de encuentro y RNG; decidir y resolver como máximo un pulso.
5. Traducir sus sucesos a operaciones de Mundo y preparar ese lote sin confirmarlo.
6. Revalidar elementos, contexto, roles, vínculos y todas las consecuencias.
7. Preparar también las vistas de impacto/movimiento y cualquier presentación necesaria.
8. Confirmar juntos Mundo, encuentro, RNG y vista; incrementar secuencia una sola vez.

La comprobación posterior valida las relaciones espaciales, características de
actores y los efectos de extracción. El registro de efectos aplicados usa
identidad de suceso y clase de consecuencia, sustituyendo el booleano único
rescate_confirmado. No hay doble extracción ni repetición de lesiones al cerrar.

Si una operación, proyección o preparación narrativa falla, se descarta todo el
candidato. El fallo no debe dejar una geometría avanzada con Mundo viejo ni RNG
consumido. Se diagnostica con la categoría del apartado 14.
La atomicidad es en memoria: no incorpora guardado de partidas.

Se preserva el orden visual impacto y luego movimiento. Mientras ese segundo
frame esté pendiente no se aceptan decisiones que modifiquen el encuentro ni se
ejecutan nuevas fronteras. El segundo frame no consume tiempo, RNG o efectos.
Cada frame tiene un token nuevo de continuación aunque comparta revisión física.
La revisión de geometría, la de conocimiento y la secuencia pública son distintas.

## 10. Conclusiones y consecuencias

Se sustituye Desenlace único por una solicitud causal validada y ResultadoEncuentro.
En la implementación el director compone el resultado candidato y
validar_conclusion verifica esa solicitud antes de confirmarla. Desenlace se
conserva únicamente como etiqueta de presentación derivada; no autoriza cierres
ni determina por sí sola los efectos mundiales.
CausaFin admite SalidaResuelta, RetiradaResuelta, Captura y Derrota.
AbandonarProposito cambia dirección y resultado del propósito; no cierra
automáticamente. Solo el coordinador/director del encuentro solicita estas causas.
El espacio no conoce metas ni escribe finalizado.

La solicitud causal incluye propósito inicial/vigente, resultados de objetivos, de cada
ficha y de cada bien. ResultadoFicha admite Salio, Transportado, Muerto,
Capturado(captor) y Permanece(lugar). ResultadoBien distingue Extraido(portador),
Permanece(ubicacion) y EnTransporteAunNoExtraido. Esta última impide el cierre.
No se omiten fichas caídas, separadas o ya salidas al elaborar el resultado.

validar_conclusion comprueba causa y contabilidad total. No exige eliminar a todo
oponente ni confunde recoger con extraer. Si la salida requerida se ha resuelto,
los logros y las pérdidas se conservan por separado. Una aprobación narrativa
futura no podrá saltarse la validación de referencias y estados.

La incapacidad de todos los aliados capaces dentro del escenario puede producir
captura solo si hay un captor vivo, libre y capaz con acceso practicable a cada
cautivo. Se evalúa por persona: se elige el captor con menor coste de ruta y luego
ID estable. Un captor no aparece por proximidad horizontal atravesando pisos o
muros. Sin captor elegible para alguien, se conserva su permanencia, aunque otros
sí queden capturados. Los que ya salieron no son capturados por el estado de los
que permanecen.

Si se intenta concluir dejando a alguien no evacuado, se publica
DecisionEvacuacion con IDs y motivos, token id/paso y opciones ContinuarAuxilio o
ConfirmarAbandono. No se avanza el tiempo mientras se lee. Continuar revalida
roles/rutas; Confirmar resuelve a cada afectado como captura válida o permanencia.
La ventana identifica expresamente quién se abandona y no marca rescate ficticio.

App utiliza SepararCompanero para compañeros vivos que permanecen o quedan
capturados y no marchan con el grupo; no aplica esa operación al actor jugador.
La separación se incluye en el mismo lote que el cautiverio, cuando corresponda. La incapacidad/muerte del
jugador conserva las limitaciones actuales de campaña, sin elegir otro personaje,
revivirlo o transferir control. El resultado de quienes salieron se conserva
independientemente. Un muerto no se captura ni se incorpora al grupo.

Los efectos sobre Mundo emplean sus operaciones vigentes:
lesiones por características; RegistrarCautiverio; ConfirmarRescate más
IncorporarCompanero al extraer vivo al cautivo; SituarElemento para el tesoro,
LlevadoPor el líder de la partida. El transportista local puede ser otro.
Un rescatado no extraído mantiene su cautiverio mundial hasta que se resuelva su
resultado; perder una oportunidad no lo libera a distancia.

La conclusión local consolida efectos que falten. CerrarEncuentro es el
reconocimiento del resultado y la única confirmación temporal de la acción:
diez minutos de preparación más seis segundos por pulso efectivo. Repetir un token,
publicar un frame o inspeccionar un corte no aplica tiempo. Después se utiliza el
circuito de agenda, NPCs y eventos ya existente.

## 11. Eventos narrativos existentes

Se conservan puntos de preparación, contacto, herida y conclusión y su registro
de fronteras ya consumidas. Se adaptan los cuatro eventos de Gorun y las pruebas
que utilizan cambios locales. No se mantiene una firma de efectos2D alternativa.

encuentro:posicion y encuentro:referencia devuelven coordenadas x/y/z; una
referencia contextual se resuelve por su tipo y posición de interacción.
Las entradas de efectos propuestas sustituyen las anteriores:

| Entrada Lua | Operación preparada |
|---|---|
| op_encuentro.voxel(x,y,z,tipo) | Aire o sólido con tipo admitido: aire, piedra, madera o metal. |
| op_encuentro.despejar(x,y,z,alto) | Convierte en aire una columna acotada desde z; no borra el apoyo inferior. |
| op_encuentro.reubicar(clave_actor,x,y,z) | Reubica una ficha existente validando volumen y ocupación. |

No son firmas existentes. Todas exigen contexto de encuentro suspendido,
argumentos enteros en rango y validación del lote completo. No alteran directamente
Mundo ni añaden fichas o planes. Una orden que toca el volumen de un elemento
definido no puede borrar su identidad por editar terreno; sus efectos físicos
siguen perteneciendo a ese elemento.

Gorun despeja las dos brechas con altura2 desde sus bases. No queda un vóxel de
muro en el torso por traducir solo la planta, ni se elimina el suelo debajo.
El lote no puede crear intersecciones, quitar apoyo a una ficha sin resolverla
o invalidar la salida dejando el episodio sin una conclusión posible.
No se exige que todo cambio mantenga una ruta al propósito: un bloqueo puede ser
un resultado legítimo con decisión o conclusión.

El coordinador prepara Mundo, encuentro, contexto y presentación candidatos.
La preparación incluye cola, marcas de eventos consumidos, fronteras y estado
narrativo afectado: un fallo tampoco consume el evento dejando su efecto sin
confirmar. No exige mantener una VM Lua ejecutándose entre pasos.
Solo confirma cuando todos son válidos; se corrige el borde actual donde el
estado local puede cambiar antes de fallar una presentación. Leer y terminar
diálogos no regenera ni resuelve pulsos. La vista suspendida utiliza el mismo
modelo filtrado y corte que el encuentro normal.

## 12. Vista filtrada del encuentro

Se sustituye la matriz plana de VistaEncuentro por VistaVolumen y entidades3D.
No se entregan referencias de contexto privadas ni datos de participantes ocultos.

VistaEncuentro contiene id, paso, revision_estado, dimensiones, propósito y fase,
texto/detalle, volumen conocido, muestras de elementos, fichas y bienes visibles,
estado de pausa/bloqueo/decisión, resultado opcional y acciones disponibles.
Cada ficha/bien visible tiene ID, base3D, altura visual, apariencia y estados
presentables. Nombres, heridas e impactos ocultos no se envían.

La altura publicada de una ficha es su intervalo geométrico [base.z, base.z+1],
independiente de que se dibuje con un solo carácter. Descubrir un espacio publica
también su aire interior y huecos conocidos, no solo los apoyos; de otro modo una
capa desconocida taparía a sus propios participantes. Se publican sus límites
conocidos sin revelar el contenido de otros espacios por compartir la columna.

Cada vóxel público es Aire, Superficie(apariencia, opaca) o Desconocido.
Desconocido no especifica si realmente hay aire, sólido, entidad o mecanismo.
Es una barrera visual conservadora al proyectar; no concede visibilidad a capas
inferiores. El volumen de una entidad no conocida no se publica como su silueta.
Los elementos conocidos publican muestras y opacidad, sin capacidad de consultar
su interior secreto mediante el frontend.

El núcleo mantiene espacios descubiertos y espacios actualmente observables;
la app filtra y convierte ese conocimiento a la vista pública, sin resolver otra
versión de percepción.
Los observables son los ocupados por aliados dentro del escenario y sus espacios
vecinos ya descubiertos mediante conexiones abiertas; no se propaga visibilidad
recursivamente. Un primer cruce confirmado descubre su espacio de llegada.
Una puerta cerrada obstruye observación a través del vano, sin borrar la memoria
del terreno descubierto. Las entidades se publican solo en espacios observables;
no se actualiza una ficha oculta por haber sido vista anteriormente. Es una regla
por espacios, no visión geométrica individual. El terreno recordado puede seguir
representándose conforme a su última observación; no se actualiza con cambios
secretos del volumen real. Los elementos recordados tampoco revelan cambios de
estado fuera de observación. Esta memoria local se descarta con el encuentro.
La presencia conocida del objetivo en algún punto del escenario no revela su
posición exacta si el espacio permanece desconocido.
La vista está libre de RNG, decisiones de IA y mutación del Mundo.

La vista no depende del corte elegido. El frontend puede cambiarlo sin pedir
otro avance ni recibir información adicional. En el primer alcance, la copia
completa del volumen acotado es suficiente; no se introduce un protocolo de deltas
3D ni transmisión entre procesos.

## 13. Terminal y corte visual

La terminal sigue utilizando caracteres en una vista superior. No hay render
vectorial, perspectiva ni motor3D. Se mantiene el ancho visual de dos celdas de
terminal por casilla horizontal usado por el dibujo actual.

EstadoVistaEncuentro conserva ID de encuentro, altura_corte, seguimiento y cámara
XY. La altura inicial es base z+1 del aliado de foco, para incluir su altura2.
El foco es el jugador visible dentro del escenario; si no está disponible, el
aliado visible no salido ni transportado de menor ID. Sin candidato se conserva
la cámara y el último corte válido, sin seguir posiciones de fichas ocultas.
En modo seguimiento cambia con ese foco; elegir otra altura lo desactiva.
El estado sobrevive a frames y eventos suspendidos; otro encuentro lo reinicia.
El corte se limita al rango [0, alto_z-1] sin modificar estado físico.

Controles: [ baja un vóxel; ] sube un vóxel; C vuelve al seguimiento del foco.
Actúan también al mostrar el encuentro suspendido detrás de una ventana narrativa,
sin resolverla ni ejecutar acciones. PageUp/PageDown conservan paginación.
Enter/Espacio, R, +/-, S y D mantienen reproducción, retirada, velocidad y detalle.
Si una entrada narrativa necesita texto libre, tiene prioridad sobre estos atajos.
Cabecera muestra altura máxima y seguimiento/manual; el detalle identifica altura
de origen y destino de una conexión vertical conocida.

proyectar_corte(vista,corte) es una función pura de terminal::encuentro:
para cada columna X/Y considera muestras a z<=corte desde arriba. Aire continúa;
una superficie opaca o Desconocido tapa lo inferior. Una entidad ocupa su volumen
de presentación y puede mostrarse simbólicamente si parte queda incluida y libre.
No se dibujan todas las fichas después del terreno sin comprobar oclusión.

En la misma altura compatible, la precedencia visual es ficha, bien, elemento y
terreno; empates residuales por ID estable. No se admite intersección física de
dos fichas activas para resolverla con un orden de dibujo.
CeldaProyectada conserva símbolo/apariencia, origen y z para detalles y pruebas.
La opacidad se respeta al continuar hacia abajo; una marca decorativa transparente
no convierte una capa inferior desconocida en visible.
Seleccionar una ficha o bien visible fija el símbolo final de esa columna; no se
continúa hasta sobrescribirlo con un piso inferior. Una decoración transparente
solo se usa si no hay una muestra opaca o dibujable prioritaria.

Impactos y señales se superponen únicamente a su ficha visible en la proyección.
No se muestran golpes fantasma a través de pisos. Una ficha cortada no se mutila
ni desactiva: es una proyección de su estado intacto.

La cámara puede recortar X/Y. Cambiar corte, redimensionar o inspeccionar detalle
no avanza reglas. Menos de 20 por 8 celdas de terminal detiene el avance hasta
recuperar tamaño válido. La omisión de animación consume las mismas continuaciones
y fronteras, sin saltarse decisiones ni eventos.

## 14. Errores y límites

Se sustituyen errores locales String por ErrorEncuentro con contexto e IDs.
La fachada app los convierte a sus códigos vigentes sin exponer datos ocultos.

| Categoría | Tratamiento |
|---|---|
| PosicionFueraDeLimites / VolumenInvalido / PerfilNoCabe | Rechazar construcción o candidato completo. |
| ReferenciaInvalida / IdAjeno / RolIncompatible | Rechazar asignación o efecto, sin reasignación silenciosa. |
| ConexionInvalida / ElementoInconsistente | Rechazar receta/cambio; no publicar contexto obsoleto. |
| SinRuta / ParticipanteNoDisponible | Resultado de comportamiento; reconsiderar o bloquear con motivo. |
| PresupuestoAgotado | Diferente de imposible; pausar y ofrecer reintento/retirada. |
| TokenObsoleto / FaseIncompatible | Rechazar antes de mutación y azar. |
| ConsecuenciaInvalida / Desincronizacion | Descartar candidatos y emitir diagnóstico de motor/evento. |

Un avance resuelve como máximo un pulso, 32 intenciones y 128 transiciones internas
de dirección. Una búsqueda expande como máximo el número de posiciones del volumen,
sin reabrir indefinidamente nodos con igual coste. Hay como máximo una búsqueda por
ficha y revisión evaluada; se reutiliza su MapaRutas para roles y movimiento.
Un avance evalúa a lo sumo dos revisiones: snapshot inicial y candidato final
para consecuencias. No hay una búsqueda adicional por cada posible cautivo.
Las recetas tienen ocho intentos. Los límites producen resultados explícitos,
nunca regeneración o replanificación infinita en una llamada.

La proyección es lineal en el volumen público y sus muestras. No llama al buscador
de rutas, no descubre habitaciones y no calcula reglas físicas. No se optimiza
prematuramente mediante grillas dispersas o simulación paralela.

## 15. Verificación acotada

No se ejecuta la batería global por cada bloque. Cada comprobación debe cubrir
una regla, frontera o regresión relevante; se reutilizan las existentes que sigan
expresando mecánicas válidas y se traducen las ligadas al modelo retirado.

| Grupo propuesto de pruebas | Contrato comprobado |
|---|---|
| espacio_local | Apoyo/altura2, dos superficies superpuestas, diagonales, transición vertical y ausencia de alcance a través de un piso. |
| contexto_local | Puerta/conexión/salida, invalidación tras cambio y referencia ajena rechazada. |
| planes_locales | Prioridad distinta de iniciativa, roles perdidos, sustitución válida, suspensión sin reinicio y bloqueo finito. |
| puerta_local | Abrir antes de cruzar, cierre ocupado rechazado, orden simultáneo y camino recalculado. |
| generacion_local | Perfiles traducidos, rutas disponibles y receta de dos alturas dentro de límites. |
| consecuencias_locales | Extracción única a líder, rescate vivo, captura parcial, abandono explícito y tiempo único. |
| eventos_locales_3d | Brecha de altura2, ocupación y Mundo/local/presentación atómicos, fronteras y RNG sin repetición. |
| corte_altura | Piso tapa sótano, hueco deja ver, actor parcial intacto, desconocido opaco e impacto oculto. |
| integracion_encuentros_3d | Atacar, R Rescatar/Robar/Recuperar, dos alturas, puerta y eventos suspendidos en terminal. |

Estos filtros son nombres a crear durante implementación, no pruebas ya ejecutadas.
Los planes especifican comandos seleccionados, salida breve y logs fuera del repo.
Clippy/formato se aplican al código cambiado cuando exista, sin afirmar éxito ahora.

Revisión manual al integrar: atacar a Gorun y ver su brecha, afrontar la banda,
rescatar a Tamin, recuperar la copa del palacio de Miraz y recorrer DosAlturas
cambiando corte. Verificar una retirada y una conclusión con bajas sin duplicar
tiempo ni bienes. No se exige obtener un resultado favorable en todo combate.

## 16. Orden de integración

Los bloques 52 a 57 son partes internas de una etapa, no seis promesas de
jugabilidad intermedia. Primero espacio/contexto, después dirección y recetas;
la vista puede prepararse en paralelo tras fijar el espacio. La app y los eventos
se integran con todo lo anterior y al final se verifica el recorrido completo.

El alcance termina con encuentros traducidos sobre un modelo único. No se mantiene
un modo viejo mientras se desarrolla el nuevo ni se obliga a conservar tests de
estructuras reemplazadas. Cada bloque registra sus verificaciones internas y su
dependencia de integración; el cierre jugable y archivo se hacen al completar la
etapa. La [hoja de ruta](../../planes/base-espacial-y-planes-encuentros.md)
enlaza los planes concretos.

Las nuevas órdenes de planes Lua tendrán otro diseño después de validar la base
genérica. Los sistemas de preparación, sigilo, combate dinámico y generación
modular seguirán ampliándose con reglamentos y entregas propios.

## Complemento implementado de dirección persistente (053)

El contrato de planes y comportamientos se concreta en `encuentros/planes.rs`,
`comportamientos.rs` y `seguimiento.rs`. `PlanLocal` guarda configuración y tipo;
`MemoriaPlan` guarda fase, propósito inicial y hechos por ID. El avance actualiza
las memorias exclusivamente sobre el candidato del encuentro.

La secuencia implementada es coordinación de roles, elección de intenciones,
preparación/reanudación de `ComportamientoActivo`, resolución simultánea y registro
de progreso, resultados y sucesos de dirección. El avance inicial dispone de fase
Preparacion; los pulsos pasan por Aproximacion, Oposicion, Objetivo y Salida según
las condiciones reales, y Terminado al concluir/cancelar el plan. La preparación
mantiene el coste existente y no añade equipo ni decisiones nuevas.

Cada instancia contiene ID, propietario, participantes, rol/cometido, destino y
referencias de fichas, estado y hechos confirmados. La exploración reutiliza su
destino hasta descubrirlo o perder su ruta; la selección de una nueva frontera
compara costes e identidad estable. Se mantienen las restricciones de conocimiento.
Las consultas `comportamiento`, `memoria_plan` y `sucesos_direccion` son puras.

Los estados del comportamiento son Activo, Suspendido, Bloqueado, Cumplido,
Interrumpido y Cancelado. La indisponibilidad no convierte una tarea pendiente en
un logro. Los pulsos y sus resultados se registran una sola vez por instancia;
responder una decisión, reproducir un frame o consultar no añade progreso.
`SucesoDireccion` publica cambios de estado del plan con motivo, de fase y de
estado de comportamiento, identificando plan, participantes, pulso e instancia.
Todavía no se agregan filtros ni comandos de autoría Lua para estos sucesos.

Reanudar cancela el plan de retirada y revalida el de misión, conservando memoria.
Una segunda retirada crea un plan nuevo. El resultado del plan Oposicion se calcula
para su bando, no copiando el resultado de la misión. La memoria suspendida no se
reescribe con fases o resultados de otro plan.


## Cierre de los cuatro pendientes de la base (2026-11-26)

### Bloqueos y continuidad

La dirección distingue `SinRuta`, `ParticipanteNoDisponible` y
`SinParticipantes`; el motivo se conserva en planes, comportamientos y sucesos,
y la sesión explica la pausa al jugador. Una espera conserva su cometido,
destino y referencias para poder reanudarla. La congestión transitoria no pausa
al grupo mientras otra ficha avanza, sale o cambia su estado físico. Se mantiene
el límite de ocho pulsos sin progreso como protección adicional.

### Presupuesto de dirección

Cada pulso dispone de 128 transiciones: considerar un plan, asignar un rol o
preparar una propuesta de comportamiento consume una unidad. La observación y
el registro del resultado no vuelven a planificar. Estas operaciones se realizan
antes de resolver acciones y consumir azar. Si se agota el presupuesto se descarta
el candidato completo y se presenta un bloqueo `PresupuestoAgotado`, reintentable
mediante los controles existentes. No se confirma movimiento, pulso ni consumo
de azar de ese candidato. Agotar identidades es un error distinto, no una pausa
reintentable por presupuesto.

### Interacciones con puertas

Cada intención elegida de abrir o cerrar produce `InteraccionPuerta` por ficha:
`Cambiada`, `Concurrente` con el ejecutor real, `SinCambio` o `Rechazada` con su
causa. `PuertaCambiada` sigue describiendo el único cambio físico. El resultado
individual entra en la memoria del comportamiento y del plan; rechazos y
`SinCambio` no cuentan como progreso. Una solicitud externa inválida se rechaza
antes de ejecutar el pulso y no se convierte en una acción consumida.

### Validación de las conclusiones

Al concluir se calculan rutas por captor y se reutilizan para comprobar a sus
posibles cautivos. El resultado conserva un certificado del estado físico exacto
(celdas, elementos, fichas, contexto y capturas). Las validaciones posteriores
comparan ese estado sin repetir búsquedas ni mutar cachés desde una consulta.
Un cambio que invalide el certificado se rechaza atómicamente; no se aplica una
conclusión obsoleta. Esto evita repetir el cálculo en presentación y cierre sin
renunciar a comprobar que el acceso y las personas siguen siendo los mismos.
