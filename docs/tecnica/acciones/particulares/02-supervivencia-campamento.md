# Acciones particulares de supervivencia y campamento

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.4 |
| Fecha | 2026-10-20 |

## 1. Propósito y alcance

Este documento define `BuscarAgua`, `TratarAgua`, `Cazar`, `Pescar`,
`Recolectar`, `MontarCampamento` y `Descansar`. Se rigen por el
[contrato común](./00-contrato-resolucion.md) y producen recursos o estados para
la acción general que las invoca; no crean un inventario persistente.

El alcance inicial usa Características concretas del Lugar y del Actor. No
simula especies, recipientes, presas individuales, nutrición, medicina, clima,
regeneración de recursos ni campamentos persistentes.

## 2. Tiempo, reservas e intentos

Agua y alimento se expresan en raciones-viajero. Una jornada-grupo equivale a
una ración por cada viajero. Las acciones informan recursos obtenidos y consumos
extraordinarios; `Viajar` descuenta una sola vez el consumo ordinario por cada
día acumulado.

- Los recursos obtenidos durante un día pueden cubrir ese mismo día.
- Buscar, cazar, pescar o recolectar deliberadamente ocupa un día.
- Tratar agua dentro de un campamento normalmente añade cero días.
- Montar el campamento y dormir tras una marcha normalmente añaden cero días.
- Un reposo de jornada completa añade un día.
- Una acción de cero días se resuelve una sola vez por su circunstancia.
- `Cazar`, `Pescar` y `Recolectar` admiten un intento de cada tipo por Lugar y
  Expedición; cambiar de responsable no reinicia la oportunidad.
- Las reservas se saturan en cero; la carencia produce consecuencias, nunca un
  entero negativo.

## 3. `BuscarAgua`

Localiza una fuente físicamente presente o sus indicios. Una fuente visible,
potable y conocida se aprovecha automáticamente sin día adicional. Si los
viajeros conocen que no puede existir agua, el intento es imposible y no se
ofrece. Si la ausencia real era desconocida, dedicar la jornada produce
determinísticamente ningún hallazgo sin revelar la causa antes de actuar. Una
fuente posible no evidente usa dificultad base `11`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Entorno | `arido` o `desertico` | dificultad `+3` |
| Estado | `seco` | dificultad `+4` |
| Terreno | `rocoso` | dificultad `+2` |
| Fuente | `pozo` o `manantial` oculto | dificultad `-3` |
| Conocimiento | `Conoce` el Lugar | tirada `+5` |
| Conocimiento | `conoceDesiertos` en desierto | tirada `+4` |
| Capacidad | `superviviente` | tirada `+3` |
| Capacidad | `rastreador` | tirada `+2` |

| Banda | Resultado |
|---|---|
| holgada | Hasta dos jornadas-grupo de agua, limitadas por la fuente. |
| lograda | Una jornada-grupo. |
| costosa | Media jornada-grupo, redondeada hacia abajo en raciones individuales. |
| fallida | Ningún agua. |

La calidad procede de la fuente. `aguaDulce` sin estado incompatible es
`potable`; `aguaContaminada` es `insegura`; `aguaSalada` y `aguaSalobre` son
`salada`. Una tirada no cambia esa calidad.

## 4. `TratarAgua`

Transforma agua insegura ya obtenida; no busca una fuente.

- El agua potable produce resultado automático sin cambios.
- El agua `salada`, incluida la procedente de `aguaSalobre`, es imposible con
  los medios ordinarios de esta versión.
- Un método suficiente y preparado correctamente potabiliza de forma
  automática.
- Solo la improvisación usa una prueba, con dificultad base `10`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Calidad | contaminación o turbiedad severa | dificultad `+4` |
| Capacidad | `superviviente` | tirada `+4` |
| Medio | recipiente y fuego adecuados | tirada `+4` |
| Ayuda | ayudante competente | tirada `+1` |

| Banda | Resultado |
|---|---|
| holgada o lograda | Toda la cantidad queda potable. |
| costosa | La mitad queda potable y la otra mitad se pierde. |
| fallida | La cantidad continúa insegura. |

No se permite encadenar tratamientos gratuitos sobre la misma cantidad durante
la misma circunstancia.

## 5. `Cazar`

Requiere fauna declarada. Si su ausencia es conocida, el intento es imposible y
no se ofrece; si era desconocida, concluye determinísticamente sin alimento
después del día dedicado.

| Hecho | Dificultad o ajuste |
|---|---:|
| `faunaAbundante` | dificultad base `8` |
| `fauna` | dificultad base `12` |
| `faunaEscasa` | dificultad base `16` |
| terreno que impide seguir o alcanzar | dificultad `+2` |
| responsable `cazador` | tirada `+5` |
| responsable `rastreador` | tirada `+3` |
| `Conoce` el Lugar | tirada `+2` |
| ayudante competente | tirada `+2` |

| Banda | Resultado |
|---|---|
| holgada | Alimento para dos jornadas-grupo. |
| lograda | Alimento para una jornada-grupo. |
| costosa | Alimento para media jornada-grupo. |
| fallida | Ningún alimento. |

Solo un Lugar con fauna concretamente peligrosa puede convertir una banda
fallida en una herida; la acción no inventa depredadores.

## 6. `Pescar`

Requiere agua accesible con peces y equipo suficiente o un método que pueda
improvisarse. Dura un día.

| Hecho | Dificultad o ajuste |
|---|---:|
| `pecesAbundantes` | dificultad base `9` |
| `peces` | dificultad base `13` |
| agua agitada o `corrienteRapida` | dificultad `+3` |
| responsable `pescador` | tirada `+5` |
| `Conoce` el Lugar | tirada `+2` |
| equipo pertinente | tirada `+3` |
| ayudante competente | tirada `+2` |

Las bandas producen las mismas cantidades que `Cazar`. Sin peces, agua
accesible o método posible, la evaluación indica imposibilidad antes de gastar
el día; elegir pescar donde los viajeros creen razonablemente que hay peces pero
en realidad no los hay produce el resultado determinista de un día sin captura
sin revelar de antemano la causa oculta.

## 7. `Recolectar`

Requiere `vegetacionComestible` u otro recurso comestible concreto. Sin un
recurso posible consume el día y no obtiene alimento cuando su ausencia era
desconocida; si era conocida, la acción no se ofrece.

| Hecho | Dificultad o ajuste |
|---|---:|
| recurso abundante | dificultad base `8` |
| recurso ordinario | dificultad base `12` |
| responsable `recolector` | tirada `+4` |
| responsable `superviviente` | tirada `+2` |
| `Conoce` el Lugar | tirada `+3` |

| Banda | Resultado |
|---|---|
| holgada | Una jornada-grupo de alimento. |
| lograda | Media jornada-grupo. |
| costosa | Una ración por cada dos viajeros, con mínimo de una. |
| fallida | Nada aprovechable. |

Solo una Característica concreta como vegetación tóxica puede causar enfermedad.
Una banda baja no inventa veneno.

## 8. `MontarCampamento`

Produce una calidad transitoria, nunca una Característica permanente del Lugar:

```text
Protegido | Adecuado | Precario | Expuesto
```

Un refugio presente, conocido y utilizable puede determinar automáticamente la
calidad que describe. En otro caso se usa dificultad base `10`.

| Eje | Hecho principal | Ajuste |
|---|---|---:|
| Exposición | abierto, viento o calor adverso | dificultad `+3` |
| Terreno | pantano o inundación | dificultad `+4` |
| Refugio | cueva, ruinas utilizables u otra protección | dificultad `-3` |
| Capacidad | `superviviente` | tirada `+4` |
| Equipo | `equipoCampamento` | tirada `+4` |
| Ayuda | ayudante competente | tirada `+2` |

| Banda | Calidad |
|---|---|
| holgada | `Protegido` |
| lograda | `Adecuado` |
| costosa | `Precario` |
| fallida | `Expuesto` |

Construir un refugio duradero no forma parte de esta acción y requeriría otro
día y otra regla.

## 9. `Descansar`

El descanso normal es principalmente determinista:

- campamento `Protegido` o `Adecuado`, agua y alimento: retira `fatigado` o
  convierte `agotado` en `fatigado`;
- campamento `Precario`: evita empeorar, pero no recupera;
- campamento `Expuesto` sin condición adversa: evita empeorar;
- campamento `Expuesto` con condición adversa: prueba de dificultad `10`.

En esa prueba, `resistente` aporta `+4`, protección pertinente `+2`, `sediento`
aporta `-3` y `hambriento`, `-2`. Solo se toma el estado más grave.

| Banda expuesta | Resultado |
|---|---|
| holgada | Retira `fatigado`. |
| lograda | No empeora. |
| costosa | Conserva o añade `fatigado`. |
| fallida | Añade `fatigado` o progresa de `fatigado` a `agotado`. |

`Descansar` no cura `herido`. Dormir al terminar la jornada no añade días; una
jornada completa de reposo añade uno y consume normalmente agua y alimento.

## 10. Resultados, consecuencias y pronósticos

Los resultados incluyen cantidad y calidad de recursos, duración, calidad de
campamento, operaciones persistentes solicitadas y la traza causal del contrato
común. La traza identifica los factores exactos que se aplicaron, la
visibilidad de fuentes o recursos, la cantidad obtenida, el consumo o carencia
que motivó la intervención y todo cambio de reserva o estado que esta causó.
Las cantidades se comprueban contra desbordamiento antes de incorporarse a la
acción general.

Cada ejecución de búsqueda, tratamiento, obtención de alimento, campamento o
descanso conserva una traza propia. Varias pueden formar una única escena de
supervivencia cuando comparten contexto dramático, pero ninguna pierde su
momento causal por ocurrir en el mismo Lugar o no añadir días. El consumo
ordinario se atribuye a la acción cuyo intervalo lo causó; solo una nueva acción
particular genera otra traza.

Las Características `sediento`, `hambriento`, `fatigado` y `agotado` se aplican
mediante [operaciones atómicas](../../componentes/11-operaciones-mundo.md). La
acción general decide cuándo una carencia repetida invoca esa progresión; estas
fichas no vuelven a descontar el consumo diario.

Los pronósticos reutilizan la derivación autoritativa, limitados a hechos
conocidos. No anuncian una fuente, fauna, peces, refugio o toxicidad ocultos. La
falta de conocimiento usa la incertidumbre neutral del contrato común.

## 11. Errores e invariantes

Se rechazan participantes ajenos a la acción, Lugar inexistente, cantidad de
agua inválida, calidad incompatible, intento repetido y contexto temporal
incoherente.

Invariantes:

- no se inventan recursos ni peligros ausentes del contenido;
- las reservas nunca son negativas;
- cada día ordinario se consume una sola vez;
- no hay intentos infinitos de coste cero;
- una prueba incierta consume exactamente un d20;
- la misma ficha se usa dentro y fuera de `Viajar`;
- una mala tirada aislada no mata directamente;
- el frontend no calcula cantidades ni recuperación.

## 12. Pruebas y criterios de aceptación

Se cubrirán resultados automáticos, imposibles y las cuatro bandas; fuentes
potables, inseguras y saladas; ausencia real y desconocida de recursos; límites
de un intento; cantidades para grupos pares e impares; campamentos de cada
calidad; descanso con cada estado; cero y un día adicionales; operaciones
atómicas y pronósticos sin filtraciones.

También se comprobará una traza por cada intervención realmente ejecutada, con
causa de la carencia, factores visibles, cantidades y estados exactos; una
secuencia `BuscarAgua`–`MontarCampamento`–`Descansar` conservará tres trazas en
ese orden, incluso cuando dos de ellas duren cero días.

Una integración debe completar un viaje que agote reservas, obtenga agua o
alimento durante la jornada, acampe, descanse y continúe, además de un caso que
quede interrumpido sin recursos. Jugador y NPC deben obtener resultados
idénticos ante el mismo contexto y azar.

## 13. Decisiones aplazadas

Se aplazan medicina, enfermedades detalladas, nutrición, recipientes,
inventario, especies, estaciones, regeneración, contaminación dinámica,
campamentos persistentes y reparto individual de raciones.
