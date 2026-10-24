# Acciones particulares de movimiento terrestre

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.5 |
| Fecha | 2026-10-20 |

## 1. Propósito y alcance

Este documento es la fuente normativa de `Orientarse`, `Marchar`, `Escalar`,
`Descender`, `Cabalgar` y `GuiarCaravana`. Las seis acciones siguen el
[contrato común](./00-contrato-resolucion.md): evalúan un intento concreto,
deducen su dificultad y competencia, resuelven lo automático o una prueba
`1d20`, y devuelven duración, consecuencias transitorias y operaciones
solicitadas.

No incluye navegación, búsqueda de recursos, campamento, clima dinámico,
monturas individuales, inventario ni división del Grupo.

## 2. Convenciones de esta familia

Las solicitudes incluyen responsable, viajeros ordenados, ayudante opcional,
Lugar de origen, Lugar siguiente, sentido del movimiento, ritmo, carga y medio
pertinente. Los dos Lugares deben estar unidos por `Conectado`. Las
Características examinadas pertenecen al Actor, los viajeros o ambos Lugares;
los tiles nunca son entrada. No existe un tipo especial de Lugar para los
tramos de un recorrido: caminos, pasos, vados, regiones y asentamientos se
evalúan mediante el mismo contrato. Cada conexión representa una etapa real,
y la acción dominante realmente ejecutada en cada etapa conserva su propia
traza aunque sea rutinaria o repita la anterior. La acción general decide si
esa traza comparte una escena contextual con otras intervenciones.

Para un Lugar `empinado` situado entre una región alta y otra baja, el sentido
se deduce sin añadir dirección a `Conectado`:

- ir desde un extremo `elevado` hacia el Lugar `empinado`, o desde este hacia un
  extremo no `elevado`, invoca `Descender`;
- ir desde un extremo no `elevado` hacia el Lugar `empinado`, o desde este hacia
  un extremo `elevado`, invoca `Escalar`;
- dos extremos que no permiten esa deducción no inventan un sentido: la etapa
  usa `Marchar` o resulta incierta hasta que una regla concreta aporte el hecho.

Así, detenerse en mitad de una ladera sigue siendo una ubicación válida: volver
hacia la región baja desciende y dirigirse hacia la alta asciende. El nombre del
Lugar y sus coordenadas no participan en la deducción.

Los ajustes físicos modifican la dificultad; capacidad, estado y ayuda modifican
la tirada. Se aplica un único factor por eje causal conforme al contrato común.
Una misma etapa usa una sola acción dominante de avance. `Orientarse` puede
precederla, pero `Marchar`, `Cabalgar` y `GuiarCaravana` no se apilan para mover
dos veces al Grupo.

Un resultado que “avanza” solicita el traslado atómico de todos los viajeros al
Lugar siguiente. Un fallo que “no avanza” deja a todos en el origen. Cada
resultado identifica su causa mecánica sin contener texto de frontend.

## 3. `Orientarse`

Determina o recupera el rumbo; nunca traslada por sí sola.

Es automática si existe un camino continuo, conservado y reconocible sin un
estado que lo oculte. Es imposible únicamente cuando no existe ninguna
referencia utilizable ni método para distinguir una continuación; esa conclusión
no consume tiempo. En los demás casos la dificultad base es `10`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Entorno | mar abierto o desierto sin referencias | dificultad `+4` |
| Entorno | bosque denso o pantano | dificultad `+3` |
| Visibilidad | adversa | dificultad `+2` |
| Referencia | río, costa o accidente reconocible | dificultad `-3` |
| Referencia | camino claramente conservado | dificultad `-4` |
| Capacidad | `guia` en tierra o `navegante` en mar | tirada `+5` |
| Conocimiento | `Conoce` ese Lugar | tirada `+4` |
| Conocimiento | ambiental pertinente, como `conoceDesiertos` | tirada `+3` |
| Capacidad | `rastreador` o `superviviente` | tirada `+2` |
| Ayuda | ayudante competente | tirada `+2` |
| Estado | `agotado` | tirada `-4` |
| Estado | `herido` o `sediento` | tirada `-2` |

| Banda | Resultado |
|---|---|
| holgada | Rumbo confirmado y un hecho próximo perceptible reconocido. |
| lograda | Rumbo confirmado. |
| costosa | Rumbo confirmado después de un día de demora. |
| fallida | Un día perdido y estado transitorio `desorientado`; no avanza. |

La acción general decide después si reintenta, regresa o acepta un desvío hacia
otro Lugar realmente conectado.

## 4. `Marchar`

Representa atravesar a pie una etapa. Es imposible en un medio enteramente
acuático o ante un obstáculo que exija expresamente otra acción. Es automática
en camino ordinario, estado estable y Grupo sin impedimentos. Su dificultad
base es `9`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Terreno | inundación o pantano profundo | dificultad `+5` |
| Terreno | pendiente fuerte o arena suelta | dificultad `+4` |
| Terreno | roca o vegetación cerrada | dificultad `+3` |
| Cruce | `vado` con `corrienteRapida` | dificultad `+3` |
| Cruce | `vado` con `aguasSomeras` | dificultad `-2` |
| Carga | `cargaVoluminosa` | dificultad `+2` |
| Exposición | calor o exposición física | dificultad `+2` |
| Vía | `sendero` | dificultad `-3` |
| Vía | `camino` o `calzada` conservados | dificultad `-5` |
| Capacidad | `resistente` | tirada `+4` |
| Capacidad | `superviviente` | tirada `+2` |
| Capacidad | `fuerte` cuando domina la carga | tirada `+2` |
| Conocimiento | ambiental pertinente | tirada `+2` |
| Equipo | protección o equipo pertinente | tirada `+2` |
| Estado | `agotado` | tirada `-4` |
| Estado | `sediento` | tirada `-3` |
| Estado | `fatigado` o `herido` | tirada `-2` |
| Ritmo | prudente | tirada `+2` |
| Ritmo | forzado | tirada `-3` |

La combinación `calzada` y `terraplen` evita que `pantanoso` sea el terreno
predominante mientras la obra no posea `derrumbado` o `inundado`. Un cauce sin
`vado` o sin `aguasSomeras` es imposible para `Marchar` y requiere otro medio o
una futura acción `Nadar`.

La preparación `cargada` aporta `cargaVoluminosa`; no crea una segunda
penalización si otra carga ya fue elegida como principal. `fuerte` puede ayudar
frente a esa carga como eje de capacidad diferente. En terreno exigente el
ritmo prudente puede fijar duración base de dos días. El ritmo forzado solicita
una ración extraordinaria de agua por viajero.

| Banda | Resultado |
|---|---|
| holgada | Avanza en la duración prevista sin desgaste. |
| lograda | Avanza en la duración prevista. |
| costosa | Avanza; añade `fatigado` o consumo extraordinario según la causa dominante. |
| fallida | No avanza, transcurre un día y añade `fatigado`; solo hiere si el Lugar declara un peligro físico que lo justifique. |

## 5. `Escalar`

Supera hacia arriba una formación que no puede recorrerse marchando. Requiere
una superficie físicamente escalable; una pared imposible sin medios produce
`Imposible`. Dificultad base `12`.

La implementación considera superficie escalable cuando alguno de los
dos Lugares de la etapa posee `empinado`, `pared` o `acantilado`. La ausencia de
esos hechos produce `Imposible`. Un responsable con `escalador` y
`equipoMontana` obtiene resolución
automática holgada cuando no existen formación o estado físico adversos, carga
voluminosa ni estados personales negativos. Los demás intentos usan `1d20`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Formación | pared o acantilado | dificultad `+3` |
| Estado físico | roca húmeda, suelta o viento | dificultad `+2` |
| Carga | `cargaVoluminosa` | dificultad `+2` |
| Capacidad | `escalador` | tirada `+5` |
| Capacidad | `fuerte` | tirada `+2` |
| Equipo | `equipoMontana` | tirada `+4` |
| Ayuda | ayudante competente | tirada `+2` |
| Estado | `agotado` | tirada `-4` |
| Estado | `herido` | tirada `-3` |
| Estado | `fatigado` | tirada `-2` |

| Banda | Resultado |
|---|---|
| holgada | Todo el Grupo supera el obstáculo en un día. |
| lograda | Lo supera en un día usando normalmente sus medios. |
| costosa | Lo supera en un día y pierde reservas o recibe `fatigado`, según la causa. |
| fallida | Permanece en el origen, transcurre un día y un participante expuesto recibe `herido`. |

La elección del participante expuesto es determinista: responsable, después
ayudante y después viajeros por `IdActor`, eligiendo el primero cuya situación
explique la exposición. Un fallo ordinario no añade `muerto`.

En la integración inicial, el dirigente es responsable y el primer compañero es
ayudante. Un ayudante con `escalador`, `fuerte` o `equipoMontana` aporta el bono
de ayuda. Una banda costosa solicita `fatigado` para el responsable; una fallida
solicita `herido` para el participante expuesto y no solicita avance. Estas
consecuencias no mutan el Mundo hasta que la acción general confirma su lote.

`Orientarse`, `Marchar`, `Descender`, `Cabalgar` y `GuiarCaravana` comparten ya
esta resolución y son invocadas por la expedición según el terreno y el medio.

## 6. `Descender`

Usa los mismos requisitos y factores que `Escalar`, con dificultad base `10`.
Su diferencia está en el desenlace:

| Banda | Resultado |
|---|---|
| holgada | Descenso controlado; avanza en un día. |
| lograda | Descenso completado en un día. |
| costosa | Avanza con fatiga o pérdida de reservas. |
| fallida | Descenso incontrolado: todo el Grupo termina unido al otro lado y un participante expuesto recibe `herido`. |

El avance fallido es deliberado: una caída controlada de manera insuficiente
puede impedir deshacer el trayecto. La operación de traslado y la herida se
aplican en el mismo lote.

## 7. `Cabalgar`

Es la acción dominante cuando la expedición usa monturas para avanzar. Requiere
`conCamellos` u otro acceso aprobado suficiente para todos los viajeros. En este
alcance solo se reglamentan camellos; no existen fichas individuales.

Es automática sobre camino abierto, condiciones estables y medio descansado.
Dificultad base `9`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Terreno | pantano o bosque cerrado | dificultad `+5` |
| Terreno | roca o pendiente | dificultad `+3` |
| Exposición | calor | dificultad `+2` |
| Capacidad | `jinete` | tirada `+4` |
| Capacidad | `camellero` con camellos | tirada `+4` |
| Medio | montura adecuada al ambiente | tirada `+4` |
| Conocimiento | `Conoce` el Lugar | tirada `+2` |
| Estado medio | monturas fatigadas | tirada `-3` |

| Banda | Resultado |
|---|---|
| holgada | Avanza un Lugar en un día y conserva el medio descansado. |
| lograda | Avanza un Lugar en un día. |
| costosa | Avanza; el medio o el responsable queda fatigado. |
| fallida | No avanza, transcurre un día y solo hay lesión si el terreno la explica. |

El medio consume una jornada-grupo adicional abstracta de reservas por día. En
un Lugar `pastizal` no consume alimento transportado. No se descuenta por animal.

## 8. `GuiarCaravana`

Sustituye a `Cabalgar` cuando el problema central es coordinar animales de carga,
carros o una preparación cargada. Dificultad base `11`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Vía | `camino` o `calzada` | dificultad `-4` |
| Terreno | paso `estrecho` | dificultad `+3` |
| Terreno | `empinado` | dificultad `+3` |
| Carga | `cargaVoluminosa` | dificultad `+2` |
| Capacidad | `camellero` o `arriero` | tirada `+5` |
| Capacidad | `guia` | tirada `+3` |
| Ayuda | ayudante competente | tirada `+2` |

| Banda | Resultado |
|---|---|
| holgada | Avanza en un día sin pérdida. |
| lograda | Avanza normalmente. |
| costosa | Avanza y pierde reservas o fatiga el medio. |
| fallida | No avanza, consume un día y exige descargar, descansar o regresar antes de repetir. |

La exigencia posterior se devuelve como opciones habilitadas; la acción
particular no elige por el jugador.

## 9. Resultados y estimaciones

Cada resultado identifica acción, responsable, banda o determinación, avance,
duración, consumos extraordinarios, cambios transitorios, operaciones
solicitadas y la traza causal del contrato común. Esa traza conserva los
factores exactos de las tablas anteriores que se aplicaron, su procedencia y
visibilidad, el detalle de la tirada cuando exista, el sentido del movimiento y
si el traslado quedó solicitado o fue imposible. No contiene widgets ni texto
de Ratatui.

`Orientarse` y la acción dominante de avance son invocaciones diferentes y
producen trazas diferentes. Dos etapas consecutivas que ejecuten `Marchar`
también producen dos trazas. Pueden participar de una misma situación dramática,
pero sus momentos causales conservan por separado verbo, orden, duración y
resultado.

Las seis acciones comparten su derivación entre evaluación y pronóstico. El
pronóstico:

- usa solo hechos proyectados como conocidos;
- devuelve la forma normalizada del contrato común;
- considera un único intento, no reintentos ilimitados;
- marca incertidumbre neutral cuando falta conocimiento específico;
- no confirma un peligro oculto por la amplitud de un rango;
- no consume azar, reservas ni duración.

## 10. Errores e invariantes

Se rechazan IDs desconocidos, Lugares no conectados, responsable o ayudante que
no sea viajero, medio ausente, sentido incompatible, Grupo dividido y elección
de acción físicamente imposible.

Invariantes:

- una etapa tiene una sola acción dominante de avance;
- el Grupo se traslada completo o no se traslada;
- dificultad y tiempo se deducen y no se leen de `dificil` o `jornadaLarga`;
- ninguna prueba aislada rutinaria mata;
- un resultado incierto consume exactamente un d20;
- una resolución automática no consume azar;
- mismo contexto y azar producen el mismo resultado.

## 11. Pruebas y criterios de aceptación

Se cubrirán para cada acción los casos imposible, automático y las cuatro
bandas; cada factor en aislamiento; precedencia de ejes equivalentes; estados y
ayuda; duración y consumos; solicitud exacta de operaciones; pronóstico puro y
conocimiento oculto.

Además se probarán:

- camino ordinario automático y pendiente contextual;
- `Escalar` fallido sin traslado y `Descender` fallido con lote atómico;
- ritmo prudente y forzado;
- exclusión mutua entre `Marchar`, `Cabalgar` y `GuiarCaravana`;
- una traza por cada invocación, incluidos `Orientarse` seguido de movimiento y
  dos movimientos rutinarios consecutivos, con factores y consecuencias
  exactos;
- camellos en terreno favorable e incompatible;
- paridad al invocar una misma acción desde dos acciones generales y desde un
  NPC.

## 12. Decisiones aplazadas

Se aplazan correr, saltar, nadar, conducir carros con identidad, animales
individuales, desgaste persistente de equipo, clima dinámico y separación de
viajeros.
