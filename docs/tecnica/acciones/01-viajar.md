# Acción general `Viajar`

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.11 |
| Fecha | 2026-10-14 |

## 1. Propósito y alcance

Este documento es la fuente canónica para las reglas, ramas narrativas,
consecuencias y duración de `Viajar`. Es la única acción general disponible para
el jugador en el primer incremento jugable.

Utiliza el [modelo de Mundo](../componentes/02-modelo-mundo.md), se ejecuta en el
[motor común](../componentes/05-motor-acciones.md) y entrega sus salidas a la
[fachada de aplicación](../componentes/04-sesion-aplicacion.md).

El alcance inicial no utiliza Relaciones, distancias de tiles, rutas,
inventarios, facciones ni simulación geográfica. Sus sucesos serán sencillos,
genéricos y codificados en Rust; incrementos posteriores podrán reemplazarlos o
ampliarlos con reglas que utilicen más elementos del Mundo.

## 2. Solicitud y disponibilidad

Una solicitud identifica:

- el Actor que viaja, vinculado por la aplicación o la fase NPC;
- un `IdLugar` de destino.

Nunca contiene coordenadas. Para ofrecer destinos, el núcleo considera los
Lugares del Mundo y aplica exactamente las condiciones aprobadas aquí. El
frontend solo presenta esa lista y el núcleo vuelve a validarla al confirmar.

El módulo de `Viajar` expone una consulta de solo lectura equivalente a:

```rust
pub fn destinos_disponibles(
    mundo: &Mundo,
    actor: IdActor,
) -> Result<Vec<IdLugar>, RechazoAccion>;
```

El vector conserva el orden estable de Lugares definido por el contenido. Un
Lugar `inalcanzable` permanece visible en el mapa, pero no aparece en este vector
y por ello no puede recibir foco ni confirmarse como destino. La consulta no
consume azar y su resultado no evita la validación posterior.

Condiciones confirmadas:

- Actor y destino deben existir;
- el destino debe ser distinto del Lugar actual del Actor;
- un Lugar con la Característica `inalcanzable` no puede elegirse;
- la acción no consulta Relaciones en esta versión;
- el mapa y la distancia visual no afectan disponibilidad ni duración;
- si actúa el jugador, el traslado incluye al propio jugador y a todo su Grupo;
- un integrante del Grupo no puede iniciar un viaje independiente.

La validación de una solicitud concreta sigue este orden normativo: Actor
existente, destino existente, jugador no `muerto`, destino distinto del Lugar
actual, destino sin `inalcanzable` y Actor no integrante del Grupo intentando
viajar solo. Se devuelve solamente el primer rechazo. El estado `muerto` de un
NPC independiente sigue fuera del alcance inicial.

`destinos_disponibles` ignora siempre el Lugar actual. La validación de la
solicitud repite esta comprobación: aunque un frontend o un script Lua envíe
manualmente ese identificador, el motor rechaza la solicitud antes de seleccionar
una rama, consumir azar, emitir fragmentos, acumular duración o mutar el Mundo.

Antes de enumerar Lugares, `destinos_disponibles` comprueba que el Actor exista,
que un jugador no tenga `muerto` y que el Actor no sea un integrante del Grupo
intentando viajar solo. Después conserva únicamente los Lugares distintos del
actual y sin `inalcanzable`, en su orden canónico.

El actor usado por la fase NPC del contenido oficial estará vivo y será ajeno al
Grupo. La fase no evalúa políticas Lua de compañeros y el motor rechaza cualquier
solicitud que intentara trasladar a uno de ellos por separado.

## 3. Ejecución inicial

La acción general organiza una secuencia reanudable:

```text
validar Actor y destino
→ seleccionar al azar un grupo de viñetas de viaje
→ recorrer la máquina de estados de la rama elegida
  ├── emitir un fragmento narrativo
  ├── solicitar y recibir una decisión, solamente si la rama la contiene
  ├── ejecutar una acción particular
  └── acumular duración
→ trasladar al Actor independiente o al jugador junto con su Grupo
→ finalizar
```

Una rama puede intercalar esas operaciones en el orden que defina su diseño; el
diagrama no obliga a emitir toda la narrativa antes de simular. Las subsecciones
siguientes fijan ese orden para las tres ramas iniciales.

Una vez aceptada la solicitud, las tres ramas iniciales terminan llegando al
destino. Ninguna opción puede cancelar el viaje, dejar al Actor en el origen o
enviarlo a un Lugar diferente. Las ramas solo varían los fragmentos, las
decisiones recorridas y la duración acumulada.

La selección aleatoria utiliza exclusivamente la fuente de azar proporcionada
por el motor. Con igual Mundo, solicitud, decisiones y secuencia de azar, el
resultado debe repetirse.

El contenido mínimo tiene exactamente tres grupos, todos con la misma
probabilidad de ser elegidos:

1. **Trayecto tranquilo:** presenta un viaje sin una complicación relevante.
2. **Obstáculo:** presenta una dificultad incidental y una decisión entre dos
   formas de continuar.
3. **Rumbo incierto:** presenta una duda de orientación y una elección entre
   riesgo y demora segura, sin consultar una red de rutas real.

Estos nombres describen ramas internas y no son IDs de contenido. El obstáculo y
la incertidumbre de rumbo no se derivan de tiles, distancia, Relaciones, terreno,
clima ni del Lugar de origen o destino: son sucesos narrativos propios de la
implementación inicial de `Viajar`. La selección consume exactamente una llamada a
`FuenteAzar::elegir_indice` con límite exclusivo `3`, antes de emitir el primer
fragmento. El orden estable para interpretar los índices es el de la lista
anterior.

### Rama `Trayecto tranquilo`

El índice `0` selecciona una rama determinista que se completa durante la misma
llamada de inicio, porque no contiene una decisión. Su secuencia reglada es:

1. ejecutar la acción particular `RecorrerTramo`, que añade exactamente un día;
2. emitir una viñeta breve de marcha sin incidentes;
3. ejecutar nuevamente `RecorrerTramo`, añadiendo otro día;
4. emitir una viñeta breve de llegada;
5. trasladar una sola vez al Actor independiente o al jugador con su Grupo;
6. finalizar el viaje con una duración total de dos días.

`RecorrerTramo` no consulta ni modifica el Mundo, no consume azar y no emite por
sí misma una viñeta: solamente aporta un día a la duración acumulada. La lógica
de la rama decide cuándo invocarla y qué fragmento narrativo sigue a cada
invocación. Esta rama no solicita decisiones ni realiza otra extracción después
de la selección inicial del grupo.

Los dos fragmentos usan estas cadenas exactas, sin interpolar nombres ni otros
datos del Mundo:

| Orden | Texto | `ClaveEscena` |
|---:|---|---|
| 1 | `El viaje avanza sin contratiempos. La marcha mantiene un ritmo constante.` | `Some(ClaveEscena::ViajeMarcha)` |
| 2 | `Tras una marcha tranquila, el destino aparece al final del camino.` | `Some(ClaveEscena::ViajeLlegada)` |

Cada texto consta exactamente de los caracteres mostrados en la tabla, sin salto
de línea inicial o final. Este documento es la autoridad de esos literales y de
su asociación con una variante; la declaración del enum pertenece al motor y el
nombre usado por el catálogo visual pertenece al frontend.

### Rama `Obstáculo`

El índice `1` selecciona una rama con una decisión. Su primer tramo ejecuta esta
secuencia:

1. ejecutar `RecorrerTramo` y añadir un día;
2. emitir una viñeta que presenta un derrumbe incidental bloqueando el paso;
3. solicitar una decisión con las opciones, en este orden, `Abrirse paso` y
   `Rodear el obstáculo`.

La viñeta y la solicitud de decisión usan estas cadenas exactas:

| Salida | Texto | `ClaveEscena` |
|---|---|---|
| Viñeta inicial | `Un derrumbe bloquea el paso. Los restos impiden continuar por el trayecto más directo.` | `Some(ClaveEscena::ViajeObstaculo)` |
| Solicitud de decisión | `El derrumbe cierra el paso directo. ¿Cómo debe continuar el viaje?` | `Some(ClaveEscena::ViajeObstaculo)` |

La solicitud conserva exactamente dos opciones en este orden: `Abrirse paso` y
`Rodear el obstáculo`.

Al responder, la rama ejecuta la acción particular `ResolverObstaculo`:

- **Abrirse paso:** consulta si al menos uno de los viajeros posee la
  Característica `fuerte`. Si existe, añade un día; si no existe, añade tres.
- **Rodear el obstáculo:** no consulta Características y añade dos días.

Para esta regla, los viajeros son el jugador seguido de los integrantes del
Grupo en su orden canónico cuando actúa el jugador, o únicamente el Actor cuando
actúa un NPC independiente. La presencia de `fuerte` es suficiente: no importa
cuántos viajeros la posean ni cuál sea el primero, y la Característica no se
consume ni se modifica. No cuentan otros actores presentes, el origen, el
destino ni ninguna Característica diferente.

Después de resolver la opción, la rama emite una única viñeta que combina el
resultado con la llegada, traslada una sola vez a los viajeros y finaliza. Los
tres recorridos posibles son:

| Opción | Condición | Incremento posterior | Duración total |
|---|---|---:|---:|
| `Abrirse paso` | algún viajero tiene `fuerte` | 1 día | 2 días |
| `Abrirse paso` | ningún viajero tiene `fuerte` | 3 días | 4 días |
| `Rodear el obstáculo` | no aplica | 2 días | 3 días |

La viñeta final correspondiente usa uno de estos literales:

| Resultado | Texto | `ClaveEscena` |
|---|---|---|
| `Abrirse paso` con algún viajero `fuerte` | `El esfuerzo abre un paso entre los restos. El viaje continúa sin una gran demora hasta su destino.` | `Some(ClaveEscena::ViajeLlegada)` |
| `Abrirse paso` sin viajeros `fuerte` | `Abrir un paso entre los restos exige mucho más tiempo de lo previsto, pero el viaje finalmente alcanza su destino.` | `Some(ClaveEscena::ViajeLlegada)` |
| `Rodear el obstáculo` | `Un rodeo evita el derrumbe a costa de una marcha más larga. El viaje finalmente alcanza su destino.` | `Some(ClaveEscena::ViajeLlegada)` |

Todos los textos y opciones de esta rama constan exactamente de los caracteres
mostrados, sin interpolaciones ni saltos de línea iniciales o finales.

La rama no realiza una extracción para resolver el resultado. Cuando actúa el
jugador, su respuesta determina directamente la opción. Cuando actúa un NPC, la
fase correspondiente puede consumir la extracción común para elegir una opción,
pero `ResolverObstaculo` sigue siendo determinista una vez recibida esa respuesta.

### Rama `Rumbo incierto`

El índice `2` selecciona la segunda rama con una decisión. Su primer tramo sigue
esta secuencia:

1. ejecutar `RecorrerTramo` y añadir un día;
2. emitir una viñeta que presenta señales ambiguas y un rumbo difícil de
   reconocer;
3. solicitar una decisión con las opciones, en este orden,
   `Confiar en la intuición` y `Revisar el rumbo`.

La viñeta y la solicitud de decisión usan estas cadenas exactas:

| Salida | Texto | `ClaveEscena` |
|---|---|---|
| Viñeta inicial | `Las señales del camino se vuelven confusas. No resulta claro cuál es el rumbo correcto.` | `Some(ClaveEscena::ViajeRumboIncierto)` |
| Solicitud de decisión | `El rumbo es incierto. ¿Cómo debe continuar el viaje?` | `Some(ClaveEscena::ViajeRumboIncierto)` |

La solicitud conserva exactamente dos opciones en este orden: `Confiar en la
intuición` y `Revisar el rumbo`.

Ese orden también fija los índices usados por la elección automática de un NPC:
`0` corresponde a `Confiar en la intuición` y `1` a `Revisar el rumbo`.

Al responder, la acción particular `ResolverRumbo` aplica una de estas reglas:

- **Confiar en la intuición:** realiza una llamada a
  `FuenteAzar::elegir_indice` con límite exclusivo `2`. El índice `0` representa
  un avance directo y añade un día; el índice `1` representa una demora y añade
  tres días.
- **Revisar el rumbo:** no consume azar y añade dos días.

Después del resultado, la rama emite una única viñeta que combina lo sucedido
con la llegada, realiza un único traslado y finaliza:

| Opción | Resultado | Incremento posterior | Duración total |
|---|---|---:|---:|
| `Confiar en la intuición` | índice `0`: avance directo | 1 día | 2 días |
| `Confiar en la intuición` | índice `1`: demora | 3 días | 4 días |
| `Revisar el rumbo` | comprobación segura | 2 días | 3 días |

La viñeta final correspondiente usa uno de estos literales:

| Resultado | Texto | `ClaveEscena` |
|---|---|---|
| Intuición, índice `0` | `La intuición resulta acertada y permite retomar el rumbo sin una gran demora. El viaje finalmente alcanza su destino.` | `Some(ClaveEscena::ViajeLlegada)` |
| Intuición, índice `1` | `La intuición conduce por un desvío equivocado. Corregir el rumbo demora la marcha, pero el viaje finalmente alcanza su destino.` | `Some(ClaveEscena::ViajeLlegada)` |
| `Revisar el rumbo` | `Una comprobación cuidadosa permite confirmar el rumbo. La marcha continúa por un trayecto seguro hasta su destino.` | `Some(ClaveEscena::ViajeLlegada)` |

Todos los textos y opciones de esta rama constan exactamente de los caracteres
mostrados, sin interpolaciones ni saltos de línea iniciales o finales.

Esta rama no consulta tiles, coordenadas, Relaciones, Lugares intermedios ni
Características. Sus señales y posibles desvíos existen solo dentro del suceso
narrativo y nunca crean una ruta persistente. Después de la validación común de
Actor y destino, `ResolverRumbo` recibe únicamente la opción, la fuente de azar y
la duración acumulada; no lee otra parte del Mundo.

El orden de azar observable es estricto. Para el jugador es selección de rama y,
solo si elige `Confiar en la intuición`, resolución del resultado. Para un NPC es
selección de rama, elección automática de opción y, solo si esa opción es
`Confiar en la intuición`, resolución del resultado. `Revisar el rumbo` nunca
añade una extracción posterior a la elección de opción.

Los grupos de viñetas, sus textos y cualquier decisión intermedia estarán
codificados inicialmente en Rust. Lua no define la acción ni redacta sus
resultados.

## 4. Consecuencia reglada

La finalización de toda solicitud aceptada cambia la ubicación lógica:

- si quien actúa es el jugador, el propio jugador y todos los integrantes del
  Grupo pasan atómicamente al destino;
- cuando quien actúa es un NPC vivo y ajeno al Grupo, solamente ese Actor pasa
  al destino mediante la misma operación reglada de traslado;
- ninguna consecuencia escribe coordenadas ni mueve representaciones dibujadas.

En este incremento, toda acción aceptada alcanza esa finalización y aplica un
único traslado. Los sucesos de viaje no agregan ni retiran Características, no
crean ni eliminan elementos del Mundo y no producen otra mutación persistente.
Un resultado narrativamente desfavorable puede costar más tiempo, pero no hiere,
mata ni impide llegar al Actor o a su Grupo.

El cambio de ubicación se confirma mediante una operación de `Mundo`. Un fallo
de validación previo no mueve a nadie. En las tres ramas, el motor lo confirma
después de emitir el último fragmento de la rama y antes de finalizar. Una vez
confirmado, añade exactamente una emisión
`ResultadoSemantico::ActoresTrasladados` con el origen, el destino y la secuencia
completa de actores trasladados. Esa emisión describe el cambio ya realizado y
no solicita que la aplicación lo repita.

La operación devuelve un `TrasladoConfirmado` con esos mismos tres datos. La
acción lo convierte directamente en `ActoresTrasladados`. Como todas las causas
previsibles fueron comprobadas antes de comenzar, cualquier `ErrorTraslado` en
este punto es un fallo fatal de invariante y no un rechazo tardío de gameplay.

## 5. Narrativa, decisiones y presentación

`desert-tiger-core` produce fragmentos narrativos neutrales en un orden definido.
La terminal decide su composición visual, pero no puede omitir una decisión ni
reordenar consecuencias.

Cada fragmento y cada solicitud de decisión podrán asociar una `ClaveEscena`
semántica opcional. No es una ruta, un identificador de contenido ni arte ASCII:
describe el contexto narrativo para que cada frontend pueda representarlo o
ignorarlo. Su presencia no modifica reglas, azar ni consecuencias.

Las variantes Rust se declaran en el
[motor de acciones](../componentes/05-motor-acciones.md) y su conversión a nombres
del catálogo pertenece al
[frontend de terminal](../componentes/09-frontend-terminal.md). Aprobar una
variante no obliga a que `scenes.ron` contenga una ilustración: todo fragmento o
decisión asociado sigue siendo válido como viñeta de solo texto.

Para el primer incremento deberá existir al menos un conjunto mínimo de ramas
que permita comprobar:

- selección reproducible de una rama azarosa;
- presentación de varias viñetas en orden;
- al menos una decisión intermedia;
- una finalización con traslado y duración.

La cantidad, el propósito general y el límite de consecuencias de las ramas
quedan aprobados en las secciones 3 y 4. También quedan aprobadas la secuencia y
las salidas narrativas completas de las tres ramas.

## 6. Duración

Cada acción particular de la rama puede agregar días o meses codificados en Rust
a la duración total. `Viajar` entrega el total al motor; no modifica directamente
la fecha.

La [sesión](../componentes/04-sesion-aplicacion.md) aplica una sola vez la
duración final después de que el jugador cierre la última viñeta. En una acción
NPC, la duración se calcula pero no adelanta la fecha.

La rama `Trayecto tranquilo` acumula dos incrementos de un día y termina con una
duración de dos días. `Obstáculo` termina con dos, tres o cuatro días conforme a
la opción y condición descritas en la sección 3. `Rumbo incierto` termina también
con dos, tres o cuatro días conforme a su opción y, cuando corresponde, al índice
azaroso descrito allí.

## 7. Resultados y rechazos

La acción distingue:

- **rechazo previo:** la solicitud no puede comenzar y no consume azar, tiempo
  ni mutaciones;
- **resultado de una rama:** la acción comenzó y puede producir una narración o
  una duración desfavorable, pero en este incremento no modifica
  Características ni evita el traslado;
- **resultado semántico:** informa el traslado ya confirmado mediante
  `ActoresTrasladados`;
- **finalización:** informa únicamente la duración total.

Las causas de rechazo propias de `Viajar` son exhaustivamente: Actor desconocido,
destino desconocido, destino igual al Lugar actual, destino con la
Característica `inalcanzable` e intento de viaje independiente por un integrante
del Grupo. Las validaciones comunes —como un jugador con `muerto`, una fase
incompatible o una respuesta de decisión inválida— pertenecen al motor o a la
fachada y no agregan causas específicas a esta acción.

## 8. Pruebas y aceptación

Las pruebas cubrirán:

- lista de destinos y nueva validación al confirmar;
- rechazo de Actor o destino desconocidos sin consumo de azar ni cambios;
- exclusión y rechazo del Lugar actual sin consumo de azar ni cambios;
- exclusión de `inalcanzable`;
- independencia respecto de tiles, coordenadas y Relaciones;
- selección reproducible y uniforme de los tres grupos mediante exactamente una
  extracción inicial con límite `3`;
- correspondencia estable de los índices `0`, `1` y `2` con, respectivamente,
  `Trayecto tranquilo`, `Obstáculo` y `Rumbo incierto`;
- orden de fragmentos y decisiones;
- acumulación de duración;
- selección de `Trayecto tranquilo` con índice `0`, dos invocaciones de
  `RecorrerTramo`, dos fragmentos en el orden aprobado, duración final de dos
  días y ninguna extracción de azar adicional;
- ausencia de `SolicitudDecision` en `Trayecto tranquilo` y traslado único
  después del fragmento de llegada, inmediatamente antes de la finalización;
- textos exactos de marcha y llegada, en ese orden y sin interpolaciones, con
  `Some(ClaveEscena::ViajeMarcha)` y `Some(ClaveEscena::ViajeLlegada)`;
- igualdad de esos textos y claves cuando la misma rama es ejecutada por el
  jugador o por un NPC;
- selección de `Obstáculo` con índice `1`, un primer incremento de un día, una
  viñeta de derrumbe y una decisión cuyos textos exactos usan
  `Some(ClaveEscena::ViajeObstaculo)` y cuyas dos opciones conservan el orden y
  texto aprobados;
- resultado de `Abrirse paso` en dos días cuando `fuerte` pertenece al jugador o
  a cualquier compañero, y en cuatro días cuando no pertenece a ningún viajero;
- resultado de `Rodear el obstáculo` en tres días con independencia de todas las
  Características;
- consulta exclusiva del propio Actor para un NPC independiente, sin considerar
  a otros actores presentes en su Lugar;
- ninguna Característica distinta de `fuerte`, ni una Característica del origen,
  destino u otro actor presente, cambia el resultado;
- después de seleccionar la rama, ausencia de nuevas extracciones para el
  jugador y exactamente una para que un NPC elija su opción; en ambos casos,
  `ResolverObstaculo` no consume azar;
- resultado de dos días para un NPC con `fuerte` y de cuatro para uno sin ella
  cuando su elección automática sea `Abrirse paso`;
- una única viñeta de resultado y llegada para cada una de las tres condiciones,
  con el literal exacto aprobado y `Some(ClaveEscena::ViajeLlegada)`, seguida de
  un único traslado y una finalización con la duración exacta;
- ausencia de interpolaciones y saltos iniciales o finales en todos los textos y
  opciones de `Obstáculo`, con los mismos valores al ejecutarla para el jugador
  o para un NPC;
- selección de `Rumbo incierto` con índice `2`, un primer incremento de un día,
  una viñeta de señales ambiguas y una decisión cuyos textos exactos usan
  `Some(ClaveEscena::ViajeRumboIncierto)`, con `Confiar en la intuición` y
  `Revisar el rumbo` en ese orden;
- resultados de intuición con índices `0` y `1`, duraciones respectivas de dos y
  cuatro días, y una única extracción de resultado con límite `2`;
- resultado de `Revisar el rumbo` en tres días y sin extracción de resultado;
- usando la notación `límite exclusivo → índice devuelto`, secuencia de jugador
  con intuición: selección de rama `3 → 2` y resolución de rumbo `2 → 0/1`; con
  revisión: únicamente selección `3 → 2`;
- con la misma notación, secuencia de NPC con intuición: selección `3 → 2`,
  elección de opción `2 → 0` y resolución `2 → 0/1`; con revisión: selección
  `3 → 2` y elección `2 → 1`, sin una tercera extracción;
- una respuesta de decisión inválida no consume la extracción de resultado ni
  cambia la duración acumulada;
- independencia completa de `Rumbo incierto` respecto de tiles, coordenadas,
  Relaciones, Lugares intermedios y Características;
- una única viñeta de resultado y llegada para cada resultado de rumbo, con el
  literal exacto y `Some(ClaveEscena::ViajeLlegada)`, seguida de un único
  traslado y una finalización con la duración exacta;
- ausencia de interpolaciones y saltos iniciales o finales en todos los textos y
  opciones de `Rumbo incierto`, con los mismos valores al ejecutarla para el
  jugador o para un NPC;
- traslado atómico del jugador y de todos sus compañeros;
- un único `ActoresTrasladados` por recorrido, con origen y destino correctos y
  la secuencia no vacía y sin duplicados del jugador seguido por sus compañeros;
- rechazo sin mutación de un viaje independiente solicitado para un compañero;
- traslado correcto del NPC mínimo y un `ActoresTrasladados` que contiene
  solamente su identificador;
- llegada al destino desde cada rama y cada opción válida, con exactamente un
  traslado y sin una finalización alternativa;
- conservación exacta de todas las Características y ausencia de cualquier
  elemento persistente nuevo después de cada recorrido;
- rechazo sin consumo de azar ni cambios;
- misma lógica para solicitudes de jugador y NPC;
- ninguna rama crea un Actor, Lugar, objeto o Relación persistente;
- integración visible completa definida para el primer incremento.

## 9. Preguntas bloqueantes

Ninguna en este componente.

## 10. Decisiones aplazadas

- uso de Relaciones `conectado`;
- rutas y encuentros derivados de lugares o actores;
- distancias, terreno o clima;
- facciones, permisos de entrada y fronteras;
- proyección del trayecto sobre el mapa;
- tratamiento general de un Actor NPC con `muerto`;
- separación temporal, abandono o expulsión de integrantes del Grupo;
- viajes que puedan fallar, interrumpirse, cambiar el destino o modificar
  Características.
