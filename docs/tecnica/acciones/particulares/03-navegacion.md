# Acciones particulares de navegación

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por el reglamento y contrato 06 |
| Versión | 1.4 |
| Fecha | 2026-10-20 |

> Antecedente histórico. Sus reglas y APIs fueron retiradas del motor activo.
> La fuente vigente es el [contrato de Viajar](../06-implementacion-viajar.md),
> en particular su sección 8 para las cinco acciones particulares actuales.

## 1. Propósito y alcance

Este documento define `Remar`, `Navegar`, `GobernarTimon`, `Fondear`, `Atracar`
y `Achicar`. Todas siguen el
[contrato de resolución](./00-contrato-resolucion.md) y pueden ser invocadas por
`Viajar` u otra acción general sin cambiar sus reglas.

El alcance abstrae un bote o barco mediante `conBote` o `conBarco`, suficientes
para los viajeros y su operación básica. No crea embarcaciones, tripulantes,
velas, carga ni daños como elementos persistentes.

## 2. Contexto y estado de la embarcación

La solicitud incluye responsable, viajeros, ayudante, medio, Lugar acuático de
origen, siguiente Lugar conectado, maniobra y estado transitorio del medio. Un
bote es compatible con `fluvial`, `costero` o `aguasSomeras`, salvo que otra
condición física lo impida. Un barco es compatible con `costero` o `maritimo` y
con aguas profundas; no atraviesa por defecto un cauce somero. `corrienteRapida`
cuenta como corriente adversa cuando se navega contra ella. Las fichas validan
la combinación de ambos Lugares y no deducen agua desde los tiles.

`acuatico` identifica un Lugar cuya posición de gameplay está sobre el agua.
`fluvial`, `costero` y `aguasSomeras` describen su entorno, pero por sí solos no
obligan a navegar. `canalNavegable` autoriza el paso de un barco por un Lugar
somero, como el Estuario de Nahal. Estas claves describen hechos físicos; no
guardan dificultad ni una valoración precalculada de `Viajar`.

Comenzar una expedición en un Lugar `acuatico` implica que el Grupo continúa a
bordo del medio con el que llegó: admite barco y, fuera de `marAbierto`, también
bote. Esta continuidad no crea un Objeto persistente ni permite entrar al agua
desde tierra sin acceso; evita que elegir un mar como destino deje al Grupo
varado al terminar la acción anterior.

El estado transitorio progresa así:

```text
Estable → AguaEmbarcada → Anegada → NaufragioInminente
```

Una acción solo cambia los grados que su ficha declara. Llegar a
`NaufragioInminente` obliga a presentar una respuesta posible —por ejemplo
`Achicar`, fondear, regresar o pedir ayuda— antes de una consecuencia definitiva.
Una única prueba rutinaria no mata ni destruye el medio de inmediato.

`Remar` y `Navegar` son acciones dominantes de avance y no se combinan para
mover dos veces la misma etapa. Las otras cuatro son intervenciones habilitadas
por una situación concreta.

## 3. `Remar`

Es la acción dominante para un bote o embarcación movida a remo. Requiere
`conBote` o acceso equivalente, agua compatible y tripulación suficiente. Es
automática en agua calma, recorrido corto reconocido y estado `Estable`.
Dificultad base `9`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Agua | corriente adversa o agua agitada | dificultad `+4` |
| Tiempo | viento adverso | dificultad `+2` |
| Capacidad | `remero` | tirada `+5` |
| Capacidad | `fuerte` | tirada `+3` |
| Grupo | tripulación suficiente | tirada `+3` |
| Estado | `agotado` | tirada `-4` |
| Estado | `fatigado` o `herido` | tirada `-2` |

| Banda | Resultado |
|---|---|
| holgada | Avanza en un día sin desgaste. |
| lograda | Avanza en un día. |
| costosa | Avanza en un día y añade fatiga al responsable. |
| fallida | No avanza, transcurre un día y, si la causa permite entrada de agua, el medio empeora un grado. |

## 4. `Navegar`

Determina y mantiene el recorrido de un barco. En esta versión representa
también su conducción rutinaria a vela; no exige una segunda prueba de timón en
condiciones normales.

Es automática con costa reconocible, condición estable y recorrido conocido.
La dificultad base es `10` en costa o aguas protegidas y `15` en mar abierto.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Visibilidad | tormenta o visibilidad muy mala | dificultad `+4` |
| Agua | corriente desconocida | dificultad `+2` |
| Capacidad | `navegante` | tirada `+5` |
| Conocimiento | `Conoce` el Lugar marítimo | tirada `+4` |
| Medio | carta adecuada | tirada `+2` |
| Estado | `agotado` | tirada `-4` |
| Estado | `fatigado` o `herido` | tirada `-2` |

| Banda | Resultado |
|---|---|
| holgada | Avanza en un día, conserva el rumbo y anticipa una condición próxima perceptible. |
| lograda | Avanza en un día con rumbo correcto. |
| costosa | Avanza tras corregir una deriva; añade un día. |
| fallida | Un día de deriva sin avance confirmado. |

Una deriva no crea un Lugar arbitrario. La acción general conserva el último
Lugar confirmado o propone únicamente un desvío que exista en `Conectado`.

## 5. `GobernarTimon`

Solo se invoca para una maniobra peligrosa; la navegación tranquila no realiza
esta prueba además de `Navegar`. Requiere control físico suficiente del medio.
Dificultad base `12`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Maniobra | rocas, paso estrecho o corriente cruzada | dificultad `+3` |
| Tiempo | tormenta | dificultad `+4` |
| Capacidad | `timonel` | tirada `+5` |
| Capacidad | `navegante` | tirada `+3` |
| Conocimiento | `Conoce` la costa | tirada `+2` |

| Banda | Resultado |
|---|---|
| holgada | Maniobra limpia. |
| lograda | Curso mantenido. |
| costosa | Completa la maniobra con desviación o el medio empeora a `AguaEmbarcada`. |
| fallida | No completa la maniobra y el medio empeora un grado conforme a la causa. |

Una banda fallida en `NaufragioInminente` no resuelve por sí sola un naufragio;
devuelve la crisis a la acción general.

## 6. `Fondear`

Busca mantener la embarcación resguardada. Es automática en un fondeadero
conocido y estable. Dificultad base `10`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Abrigo | bahía protegida | dificultad `-3` |
| Exposición | costa abierta | dificultad `+3` |
| Fondo | rocoso o con corriente | dificultad `+3` |
| Capacidad | `marino` | tirada `+4` |
| Capacidad | `navegante` | tirada `+3` |

| Banda | Resultado |
|---|---|
| holgada | Resguardo firme. |
| lograda | Resguardo normal. |
| costosa | El ancla garrea y exige corregir o aceptar exposición. |
| fallida | No puede quedar resguardado en ese intento. |

Normalmente añade cero días. Recuperar una situación fallida mediante otro
intento después de cambiar la circunstancia puede añadir uno.

## 7. `Atracar`

Completa la transición desde un Lugar acuático hacia un muelle, playa o acceso
terrestre concreto. Es automática en muelle accesible y agua calma. Dificultad
base `10`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Acceso | rompiente, corriente o espacio estrecho | dificultad `+3` |
| Capacidad | `timonel` | tirada `+4` |
| Conocimiento | `Conoce` el puerto o costa | tirada `+3` |
| Ayuda | ayudante competente en tierra | tirada `+2` |

| Banda | Resultado |
|---|---|
| holgada o lograda | Transición completada al Lugar terrestre. |
| costosa | Atraca con demora de un día o el medio empeora un grado, según la causa. |
| fallida | Permanece en el Lugar acuático; transcurre un día o empeora el medio, según la causa. |

El traslado del Grupo solo se solicita cuando el resultado completa la
transición.

## 8. `Achicar`

Retira agua embarcada. Solo está habilitada en un estado posterior a `Estable`.
Es automática con entrada menor, recipientes y personas suficientes. En otro
caso usa dificultad base `10`, más `3` por cada grado posterior a
`AguaEmbarcada`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Capacidad | `fuerte` | tirada `+3` |
| Equipo | recipientes suficientes | tirada `+4` |
| Ayuda | ayudante competente | tirada `+2` |
| Estado | responsable `agotado` | tirada `-4` |

| Banda | Resultado |
|---|---|
| holgada | El medio vuelve a `Estable`. |
| lograda | Reduce un grado. |
| costosa | Impide que empeore, sin reducirlo. |
| fallida | Empeora un grado. |

Normalmente añade cero días porque ocurre durante la crisis. Desde `Anegada` o
`NaufragioInminente`, la acción general puede dedicar una jornada completa y
entonces añade un día.

## 9. Resultados y pronósticos

Los resultados incluyen avance o transición, duración, cambio del estado del
medio, fatiga u operaciones persistentes y la traza causal del contrato común.
La traza conserva maniobra, medio, estado anterior y posterior, factores
aplicados, detalle de resolución, avance confirmado y toda demora o crisis
causada. El estado del medio pertenece a la Expedición; solo una regla futura
que cree una embarcación persistente podrá conservarlo después.

`Navegar` seguido de `GobernarTimon`, `Achicar`, `Fondear` o `Atracar` representa
invocaciones diferentes y produce una traza para cada una. Pueden desarrollarse
dentro de una misma escena náutica, incluso mediante varias viñetas, pero cada
maniobra automática o de cero días conserva un momento causal propio y no se
absorbe dentro del resultado de la navegación dominante.

Los pronósticos usan la derivación de la misma ficha y la forma normalizada del
contrato común. Sin `Conoce`, una costa o corriente no pública usa incertidumbre
neutral; no confirma tormenta, rocas, fondeadero ni estado oculto. La
planificación compara `Remar` y `Navegar` como alternativas dominantes, no suma
ambas.

## 10. Errores e invariantes

Se rechazan medio ausente o incompatible, Lugares no conectados, maniobra sin
causa habilitante, estado del medio inválido, participantes ajenos, transición
tierra–agua incoherente y decisión repetida.

Invariantes:

- ninguna entrada al mar presupone gratis una embarcación;
- una etapa tiene una sola acción dominante de avance;
- `GobernarTimon` no duplica la navegación rutinaria;
- el Grupo permanece unido y solo ocupa Lugares;
- una deriva no inventa coordenadas ni conexiones;
- una única prueba rutinaria no causa muerte o naufragio definitivo;
- toda crisis grave ofrece al menos una respuesta antes de la consecuencia
  final;
- la estimación no muta, no tira ni filtra hechos desconocidos.

## 11. Pruebas y criterios de aceptación

Se cubrirán los casos automáticos, imposibles y las cuatro bandas de cada acción;
bote y barco compatibles e incompatibles; costa y mar abierto; todos los grados
del medio; duración; traslado atómico al atracar; deriva sin Lugar inventado;
pronóstico con y sin conocimiento; y orden reproducible del azar.

Las integraciones mínimas serán un cruce en bote, una ruta de mar abierto, una
maniobra costosa seguida de `Achicar`, un atraque fallido que conserva la
ubicación y una crisis que llega a `NaufragioInminente` sin muerte automática.
Jugador y NPC usarán las mismas reglas.

Cada integración comprobará además el orden y la integridad de una traza por
invocación, incluidos cambios del medio, avance, demora y acciones automáticas
o de cero días, sin fusionar dos maniobras sucesivas.

## 12. Decisiones aplazadas

Se aplazan embarcaciones con identidad, tripulación individual, carga, casco y
velas persistentes, mareas calculadas, clima dinámico, combate naval,
reparaciones, naufragios detallados y natación.
