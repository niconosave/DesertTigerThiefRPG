# Brief de contenido: nuevos Lugares del recorrido

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.3 |
| Fecha | 2026-10-20 |

## 1. Propósito y autoridad

Este documento define la intención creativa del primer lote de Lugares que
representan caminos, pasos, descensos, vados y transiciones costeras del
[`atlas regional`](./atlas-regional.md). Estos Lugares proporcionan hechos
geográficos que `Viajar` y sus acciones particulares pueden interpretar sin
guardar dificultad, duración o coste dentro de una Relación `Conectado`.

El brief fija los nombres, la función geográfica, los extremos que sustituye
cada Lugar y las propiedades físicas que el contenido debe expresar. Las claves
externas, coordenadas centrales, coordenadas de rótulo y demás datos
serializados ya están materializados y tienen su única fuente ejecutable en
`content/base/`. Este brief conserva su intención y sus invariantes editoriales;
no duplica los valores concretos de los RON.

## 2. Un único modelo de Lugar

Un camino o un paso incluido en este lote es un Lugar con el mismo significado
reglado que cualquier valle, bosque o población:

- un Actor puede encontrarse allí;
- puede elegirse como destino final de `Viajar`;
- puede ser origen de una acción posterior;
- puede contener Actores y participar en Relaciones;
- sus Características describen lo que existe físicamente allí;
- su posición visual en el Mapa no determina conectividad ni reglas.

No existe `LugarIntermedio`, `LugarDeTrayecto` ni ninguna otra clase, variante,
marca o lista especial. Los seis elementos de este brief se cargan en el mismo
catálogo, usan el mismo tipo Rust y se consultan por las mismas operaciones que
todos los demás Lugares. Tampoco son estados internos de una conexión ni eventos
descartables.

Si dos Lugares `A` y `B` estaban conectados directamente y el diseñador crea un
Lugar ordinario `C` para representar lo que existe entre ambos, la topología
pasa de:

```text
A ── B
```

a:

```text
A ── C ── B
```

La Relación directa `A ── B` se elimina. Mantenerla produciría un atajo que
permitiría ignorar el Lugar nuevo y sus condiciones. Cada una de las dos
conexiones nuevas se declara una sola vez; la simetría de `Conectado` permite
recorrerlas en ambos sentidos sin duplicarlas.

Ninguna Relación recibe un indicador de dificultad, una función de "tramo", una
referencia a la otra conexión ni otro dato adicional. El programa tampoco
registra que `C` reemplazó a `A ── B`: solo observa tres Lugares ordinarios y
las conexiones que efectivamente existen al iniciar la Partida.

Para que ambas conexiones sean visibles al editar el elemento que les da
sentido, el RON de `C` declara, en el orden de la tabla del lote, primero
`Conectado` hacia `A` y después hacia `B`. Los archivos de los extremos no
repiten esas lecturas inversas. Esta orientación de autoría no cambia la
simetría reglada.

Que exista otro recorrido más largo por la red regional no constituye un
bypass: los circuitos alternativos del atlas continúan siendo parte válida del
Mundo. Lo que este brief prohíbe es conservar la conexión directa sustituida o
crear otra conexión cuya única finalidad sea omitir el trayecto.

## 3. Criterio para representar un tramo como Lugar

El diseñador crea otro Lugar para un tramo cuando se cumplen estas condiciones:

1. corresponde a una formación, obra o transición geográfica reconocible;
2. un Actor podría detenerse allí y tendría sentido comenzar otra acción desde
   ese punto;
3. posee hechos propios que no pertenecen por completo a ninguno de sus dos
   extremos;
4. esos hechos pueden producir decisiones o resultados diferentes al ser
   interpretados por acciones particulares;
5. representarlo como Lugar hace que el contenido sea más claro que añadir
   datos especializados a `Conectado`.

No se crea un Lugar solamente para sumar una jornada, alargar artificialmente
una ruta, imitar la distancia dibujada en tiles o proporcionar una viñeta más.
Tampoco se divide todo camino en segmentos rutinarios. Cuando un tramo no tiene
identidad ni consecuencias propias, la conexión directa es suficiente.

## 4. Hechos físicos y dificultad deducida

Las Características de estos Lugares describen terreno, agua, vegetación,
exposición o construcciones. Son apropiadas expresiones como `empinado`,
`estrecho`, `rocoso`, `pantanoso`, `aguaDulce`, `sendero` o `calzada` porque
afirman qué encuentra un Actor.

No se incorporan conclusiones de una acción como `dificil`, `lento`,
`rutaSegura`, `jornadaLarga`, `facilDeAtravesar` o `requiereBarco`. Una acción
particular determina la dificultad del intento combinando los hechos del Lugar
con el sentido del recorrido, el estado y conocimiento del Actor, sus
compañeros, sus medios, la fecha y cualquier otra condición pertinente. Por
eso, el mismo descenso puede resultar distinto para un montañista descansado y
para un viajero herido, aunque ambos atraviesen exactamente el mismo Lugar.

Los nombres de Característica de la sección siguiente son el vocabulario de
autoría aprobado para el lote. Los RON determinan cuáles están serializados y
los documentos técnicos de las acciones determinan cuáles tienen significado
reglado. Una Característica todavía no interpretada puede permanecer como hecho
descriptivo sin cambiar la identidad creativa del Lugar.

## 5. Lote inicial

Las seis incorporaciones sustituyen conexiones de la red implementada. En la
tabla, `A — C — B` indica las dos conexiones que deben existir después de
retirar la conexión directa `A — B`.

| Nuevo Lugar (`C`) | Sustitución de la conexión | Hechos físicos previstos | Papel jugable |
|---|---|---|---|
| Camino de los Mojones | Hamada Roja — **Camino de los Mojones** — Paso Escarpado | `camino`, `arido`, `rocoso`, `abierto` | Aproximación occidental marcada por mojones de piedra. Permite que `Marchar` y `Orientarse` interpreten exposición, suelo y referencias visibles antes de entrar en la cordillera. |
| Descenso del Drâa | Paso Escarpado — **Descenso del Drâa** — Valle del Drâa | `montanoso`, `empinado`, `estrecho`, `rocoso` | Bajada oriental desde la cordillera hacia el valle. Da contexto a `Descender`, mantener el equilibrio o buscar una forma menos expuesta de bajar; el sentido inverso transforma narrativamente el descenso en ascenso sin cambiar el Lugar. |
| Senda de Tamrut | Valle del Drâa — **Senda de Tamrut** — Bosque de Tamrut | `sendero`, `boscoso`, `fluvial`, `sombreado` | Transición gradual del valle abierto al bosque. Permite interpretar visibilidad, orientación, marcha bajo vegetación y oportunidades propias del ambiente forestal. |
| Vado de Tamrut | Bosque de Tamrut — **Vado de Tamrut** — Marisma Verde | `vado`, `fluvial`, `aguaDulce`, `aguasSomeras`, `corrienteRapida` | Cruce reconocible de un cauce entre bosque y tierras bajas. Proporciona contexto para vadear, mantener el equilibrio, buscar agua o evaluar una crecida sin declarar de antemano si cruzar será fácil. |
| Calzada de Nahal | Marisma Verde — **Calzada de Nahal** — Nahal | `calzada`, `terraplen`, `pantanoso` | Obra elevada y discontinua que comunica el puerto con la marisma. Permite que el estado del terreno y del terraplén afecte la marcha sin convertir la construcción en una bonificación fija. |
| Estuario de Nahal | Nahal — **Estuario de Nahal** — Mar de Levante | `maritimo`, `fluvial`, `costero`, `aguaSalobre`, `aguasSomeras` | Transición entre el puerto, el río y el mar. Da un Lugar real para embarcar, desembarcar, atracar u observar corrientes; ninguna de esas operaciones se presupone por la mera conexión. |

Las propiedades previstas son deliberadamente descriptivas. Por ejemplo,
`camino` constata una obra o huella transitable, pero no garantiza una marcha
rápida; `vado` identifica un punto habitual de cruce, pero no garantiza que la
fecha o el caudal permitan usarlo; `terraplen` constata una obra elevada, pero
una regla puede encontrarla dañada o inadecuada para cierto medio.

### Apoyo inicial para probar las reglas

El mismo lote completa los hechos necesarios del corredor, sin intentar
caracterizar todavía todo el atlas:

- Hamada Roja conserva `arido` y añade `desertico`, `rocoso` y `abierto`;
- Paso Escarpado añade `montanoso`, `rocoso` y `elevado` a `empinado`;
- Valle del Drâa conserva `fertil` y añade `fluvial`, `aguaDulce`,
  `vegetacionComestible` y `faunaAbundante`;
- Bosque de Tamrut incorpora `boscoso`, `sombreado`, `aguaDulce` y `fauna`;
- Marisma Verde incorpora `pantanoso`, `fluvial`, `aguasSomeras` y `peces`;
- Nahal incorpora `poblado`, `mercado`, `puerto` y `muelle`. En este primer
  contenido, ambas infraestructuras incluyen un servicio local rutinario y ya
  disponible: el puerto puede proporcionar pasaje en barco y el muelle, un bote.
  La deliberación de `Viajar` debe reconocer y narrar esa procedencia; entrar en
  otra costa o Lugar acuático no hace aparecer una embarcación. El servicio no
  entrega al Actor un medio persistente ni finge una compra, porque la economía
  todavía no está reglamentada.

El Bandido acompaña al Príncipe como conocedor del corredor occidental: aporta
al Grupo monturas ya aseguradas y suficientes mediante `conCamellos`, además de
`guia`, `camellero`, `conoceDesiertos` y Relaciones `Conoce`
hacia Hamada Roja, Camino de los Mojones, Paso Escarpado y Descenso del Drâa. El
Cazador añade `cazador`, `rastreador` y `superviviente`, y conoce Valle del
Drâa, Senda de Tamrut, Bosque de Tamrut y Vado de Tamrut. El Príncipe comienza
sin conocimiento regional específico; así el Grupo se beneficia realmente de
su compañero y la deliberación puede atribuirle la propuesta y procedencia de
los camellos. Un NPC demuestra la misma mecánica sin pertenecer al Grupo.

## 6. Topología resultante del corredor oriental

El lote produce el siguiente corredor continuo:

```text
Hamada Roja
  └─ Camino de los Mojones
       └─ Paso Escarpado
            └─ Descenso del Drâa
                 └─ Valle del Drâa
                      └─ Senda de Tamrut
                           └─ Bosque de Tamrut
                                └─ Vado de Tamrut
                                     └─ Marisma Verde
                                          └─ Calzada de Nahal
                                               └─ Nahal
                                                    └─ Estuario de Nahal
                                                         └─ Mar de Levante
```

Las demás conexiones de cada región permanecen como están salvo que un diseño
posterior apruebe otro Lugar. En particular, este lote no altera por sí solo los
circuitos occidental, meridional ni marítimo descritos en el atlas.

El recorrido inicial del Príncipe y el Bandido pasa a ser:

```text
Hamada Roja
→ Camino de los Mojones
→ Paso Escarpado
→ Descenso del Drâa
→ Valle del Drâa
```

Que todos los Lugares formen parte de la ruta lógica no obliga a crear una
escena o viñeta independiente por cada conexión. Una situación dramática puede
abarcar varias etapas relacionadas; si el motor ejecuta una acción particular
en cada una, sus momentos causales siguen apareciendo en orden y no se sustituyen
por un avance que la simulación no produjo.

## 7. Colocación visual y autoría

En el lote implementado, cada centro y rótulo se ubica sobre la formación que
representa y, cuando resulta legible, entre los centros visuales de sus extremos.
Esta colocación comunica la geografía al jugador, pero no prueba ni crea las
Relaciones.

Las claves y coordenadas exactas no se duplican en este brief: pertenecen a los
RON y se revisan observando conjuntamente la matriz vigente, los demás rótulos y
el encuadre de la terminal. El contenido materializado cumple estos criterios:

- escoger claves `lower_snake_case` únicas y nombres de archivo coherentes;
- crear un rótulo legible por cada nombre sin exigir que sea horizontal;
- conservar cada glifo en una celda y en su orientación normal;
- evitar que el centro coincida con una coordenada de su propio rótulo;
- comprobar visualmente que el nuevo conjunto no vuelve ilegible la región;
- enumerar los seis Lugares en el Mundo conforme al contrato de contenido.

La orientación de sus Relaciones tampoco queda abierta a una implementación
posterior: ambas se escriben en el archivo del nuevo Lugar conforme a la
sección 2.

La revisión visual es responsabilidad editorial. No se añade código para
corregir solapamientos de nombres ni para inferir la posición correcta a partir
de la topología.

## 8. Validación y aceptación creativa

El lote está correctamente materializado cuando:

- existen los seis Lugares con sus nombres aprobados y funciones geográficas;
- cada Lugar puede ser origen o destino y puede alojar Actores como cualquier
  otro Lugar;
- se eliminaron las seis conexiones directas sustituidas;
- para cada fila del lote existen exactamente los dos vínculos indicados y no
  un vínculo adicional que omita el Lugar nuevo;
- cada `Conectado` se declara una sola vez y continúa siendo consultable en
  ambos sentidos;
- el Mundo oficial contiene veintiséis Lugares y treinta y cuatro Relaciones
  `Conectado`, siempre que ninguna otra incorporación cambie el atlas en el
  mismo incremento;
- el grafo completo continúa conectado;
- la ruta directa del recorrido inicial contiene, en orden, Hamada Roja,
  Camino de los Mojones, Paso Escarpado, Descenso del Drâa y Valle del Drâa;
- las Características serializadas describen hechos del Lugar y ninguna guarda
  duración, dificultad o valoración de una acción;
- el validador de contenido acepta referencias, nombres, centros y rótulos;
- una inspección manual del mapa confirma que cada Lugar acompaña la geografía
  pretendida y que los rótulos son utilizables;
- viajar a uno de los seis Lugares nuevos y continuar desde él devuelve siempre
  el juego a un estado válido.

Un cambio futuro puede incorporar primero un subconjunto como parte de un
incremento jugable, pero no debe declarar terminado un estado que conserve a la
vez la conexión directa y el recorrido nuevo que la sustituye.
