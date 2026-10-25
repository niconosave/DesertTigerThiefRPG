# Acción general `Viajar`

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por `02-viajar-expediciones.md` |
| Versión | 0.15 |
| Fecha | 2026-10-18 |

## 1. Propósito y autoridad

Este documento conserva el contrato histórico de la primera versión de
`Viajar`. La fuente canónica vigente es
[`02-viajar-expediciones.md`](./02-viajar-expediciones.md); ninguna regla
provisional de este archivo, incluida `inalcanzable`, debe implementarse de
nuevo.

La acción utiliza el [modelo de Mundo](../componentes/02-modelo-mundo.md) y las
[Relaciones](../componentes/10-relaciones.md), se ejecuta mediante el
[motor común](../componentes/05-motor-acciones.md) y entrega sus salidas a la
[fachada de aplicación](../componentes/04-sesion-aplicacion.md). Las Relaciones
`conectado` constituyen su red geográfica reglada; el mapa, sus tiles y sus
coordenadas siguen siendo exclusivamente visuales.

Esta versión incorpora:

- destinos alcanzables mediante el grafo de Relaciones `conectado`;
- elección de un destino final, sin elección manual de cada tramo;
- una ruta automática con la menor cantidad de conexiones;
- un día base por conexión recorrida;
- selección azarosa de un único suceso entre un trayecto tranquilo y las
  complicaciones permitidas por la ruta;
- resolución de `Ascender directamente` mediante la acción particular
  `Escalar`;
- llegada con traslado atómico o interrupción por una escalada fallida.

Medio de transporte, preparación, conocimiento y equipo todavía no son sistemas
independientes del modelo. `Escalar` puede leer hechos cualitativos existentes,
como `equipoMontana`, pero esta versión no introduce Objetos, inventario,
provisiones, monturas, orientación ni recursos consumibles.

## 2. Conceptos propios de la acción

### Ruta de viaje

Una ruta es una secuencia ordenada de `IdLugar` que, para una solicitud válida:

- comienza en el Lugar actual del Actor;
- termina en el destino elegido;
- contiene al menos dos Lugares;
- no repite Lugares;
- conecta cada par consecutivo mediante `conectado`;
- no contiene un Lugar `inalcanzable`, salvo que ese Lugar sea el origen actual.

La cantidad de conexiones de la ruta es `lugares.len() - 1`. La ruta es contexto
transitorio de la ejecución: no es un elemento persistente, no se serializa y no
se proyecta sobre el mapa.

El origen forma parte de la ruta para expresar y comprobar sus extremos, pero no
se considera un Lugar atravesado. Para reglas contextuales, los Lugares
atravesados son, en orden, todos los elementos posteriores al origen, incluido
el destino.

### Viajeros

La secuencia de viajeros es:

- cuando actúa el jugador, el propio jugador seguido por los integrantes de su
  Grupo en orden canónico;
- cuando actúa un NPC independiente, únicamente ese Actor.

La secuencia se determina al aceptar la solicitud y se conserva durante la
ejecución. Otros Actores presentes en los Lugares de la ruta no son viajeros y
no participan en sus reglas.

### Lugares marítimos

Una ruta puede contener Lugares cuya Característica editorial sea `maritimo`.
El núcleo no crea ni busca una embarcación: cuando un Actor entra en uno de esos
Lugares, el juego presupone que viaja en un barco apropiado; cuando sale hacia
tierra, presupone el desembarco. Mientras permanece allí, su ubicación lógica es
el propio Lugar marítimo, no un tile de agua.

`maritimo` no modifica por ahora disponibilidad, ruta, duración, candidaturas o
resultado. Una Relación `conectado` válida basta para atravesar el tramo, igual
que entre dos Lugares terrestres. Esta convención permite contenido navegable
sin introducir Objetos, inventario ni un sistema de medios de transporte.

### Características interpretadas

Esta versión interpreta exactamente tres Características:

| Característica | Elemento | Efecto en `Viajar` |
|---|---|---|
| `inalcanzable` | Lugar | Impide usar el Lugar como destino o atravesarlo; el origen actual es la única excepción y permite salir de él. |
| `empinado` | Lugar | Aporta una candidatura contextual de Paso empinado si el Lugar es atravesado. |
| `fuerte` | Actor | Mejora un resultado de Paso empinado si la posee al menos un viajero. |

Ninguna otra Característica modifica disponibilidad, ruta, duración o resultado
en esta versión. En particular, la ruta no intenta evitar `empinado`: primero se
elige la ruta canónica más corta y después se construyen sus candidaturas.
También puede elegir una ruta marítima más corta; el contenido debe diseñar sus
conexiones teniendo presente que el jugador todavía no elige entre rutas
alternativas.

## 3. Disponibilidad y cálculo de ruta

El módulo `acciones` de `desert-tiger-core` conserva la consulta pública:

```rust
pub fn destinos_disponibles(
    mundo: &Mundo,
    actor: IdActor,
) -> Result<Vec<IdLugar>, RechazoAccion>;
```

La consulta resuelve el Lugar actual del Actor y recorre el grafo normalizado de
`conectado` mediante `Mundo::lugares_conectados`. Una sola declaración válida de
esa Relación habilita el recorrido en ambos sentidos. El orden de sus extremos y
el orden en que fueron declaradas las Relaciones no cambian el resultado.

Un destino está disponible si y solo si:

- es distinto del Lugar actual;
- no posee `inalcanzable`;
- existe desde el origen una ruta formada por `conectado` que no atraviesa
  Lugares `inalcanzable`.

El origen puede poseer `inalcanzable`: se utiliza como raíz de la búsqueda y se
permite salir de él. Ningún otro Lugar con esa Característica se encola ni puede
aparecer en una ruta.

El vector de destinos usa el orden canónico global de Lugares del Mundo, no el
orden de descubrimiento del grafo. Una red aislada o completamente bloqueada
produce `Ok(vec![])`; no es un error.

La consulta no usa coordenadas, distancia visual, tiles ni azar. Tampoco evita
la validación al confirmar una solicitud.

### Ruta canónica más corta

Al confirmar un destino, `Viajar` calcula la ruta mediante búsqueda en anchura
(BFS) sobre las conexiones transitables:

1. encola el origen y lo marca como descubierto;
2. toma cada Lugar de la cola;
3. examina los vecinos entregados por `Mundo::lugares_conectados`, ya ordenados
   según el orden canónico global de Lugares;
4. omite vecinos ya descubiertos y Lugares `inalcanzable`;
5. registra como predecesor al Lugar desde el cual descubre cada vecino;
6. al descubrir el destino, reconstruye la secuencia desde el origen.

Esto minimiza exclusivamente la cantidad de conexiones. Si varias rutas tienen
esa longitud mínima, el orden de exploración elige de forma determinista la
secuencia lexicográficamente menor según el orden canónico de Lugares. No se
usan Características, duración estimada, cantidad de candidaturas ni orden de
los archivos para romper el empate.

La implementación puede obtener todos los alcanzables con una sola BFS para la
consulta y detenerse al descubrir el destino para una solicitud concreta, pero
ambos caminos deben respetar exactamente las mismas reglas de transitabilidad y
orden.

## 4. Solicitud y validación

Una solicitud contiene el `IdActor` que viaja y el `IdLugar` del destino final.
No contiene ruta, Relaciones ni coordenadas. El jugador y los NPC usan la misma
variante `SolicitudAccion::Viajar`.

El núcleo vuelve a validar la solicitud y calcula la ruta contra el Mundo real
al comenzar. El orden de validación es normativo:

1. el Actor existe;
2. el destino existe;
3. si el Actor es el jugador, no posee `muerto`;
4. el destino es distinto del Lugar actual;
5. el destino no posee `inalcanzable`;
6. el Actor no es un integrante del Grupo intentando viajar por separado;
7. existe una ruta conectada y transitable hasta el destino.

La última condición añade el rechazo específico:

```rust
RechazoAccion::DestinoSinRutaConectada {
    origen: IdLugar,
    destino: IdLugar,
}
```

Las demás condiciones conservan sus variantes existentes. Se devuelve solo el
primer rechazo según el orden anterior. El tratamiento general de `muerto` para
un NPC independiente continúa fuera de alcance.

Todo rechazo sucede antes de crear una ejecución, consumir azar, acumular
duración, emitir narrativa o modificar el Mundo. La ruta aceptada, los viajeros
y el contexto del suceso seleccionado se guardan como estado privado de
`EjecucionAccion`; la aplicación y el frontend no pueden sustituirlos.

## 5. Candidaturas y selección del suceso

Una solicitud válida construye una lista no vacía y ordenada de candidaturas:

1. `Trayecto tranquilo`, siempre en el índice `0`;
2. una candidatura `Paso empinado` por cada Lugar atravesado con `empinado`, en
   el mismo orden en que esos Lugares aparecen en la ruta.

El origen nunca aporta una candidatura, aunque tenga `empinado`. El destino sí
puede aportarla. Dos Lugares `empinado` distintos producen dos candidaturas
distintas aunque ambas ejecuten las mismas reglas y textos.

Después de validar, la acción hace exactamente una llamada a
`FuenteAzar::elegir_indice`, cuyo límite exclusivo es la cantidad de
candidaturas. El índice devuelto selecciona el único suceso del viaje. Todas las
candidaturas tienen la misma probabilidad; no se realizan extracciones
separadas para decidir primero si hay una complicación y después cuál.

Consecuencias de esta regla:

- sin un Lugar `empinado`, existe una sola candidatura y la llamada usa límite
  `1`;
- con un Lugar `empinado`, `Trayecto tranquilo` y Paso empinado tienen
  probabilidad `1/2` cada uno;
- con dos Lugares `empinado`, cada una de las tres candidaturas tiene
  probabilidad `1/3`;
- nunca se ejecuta más de un Paso empinado en el mismo viaje.

Con el mismo Mundo, solicitud, decisiones y secuencia de azar, la ruta, el suceso
y el resultado son reproducibles. Reordenar declaraciones de Relaciones no
reordena candidaturas porque estas dependen de la ruta canónica.

## 6. Ejecución común

La secuencia conceptual es:

```text
validar Actor y destino
→ calcular y conservar la ruta canónica
→ determinar viajeros y duración base
→ construir candidaturas y seleccionar exactamente un suceso
  ├── Trayecto tranquilo: narrar la marcha
  └── Paso empinado: presentar y resolver una decisión
→ narrar la llegada
→ trasladar atómicamente a los viajeros del origen al destino
→ emitir el resultado semántico del traslado
→ finalizar con la duración total
```

La acción general invoca una vez por conexión la acción particular interna
`RecorrerConexion`, que añade exactamente un día mediante suma comprobada. No
mueve Actores, no modifica el Mundo, no emite narrativa y no consume azar.

La ruta no causa traslados intermedios. Mientras la ejecución espera una
decisión, todos los viajeros continúan lógicamente en el origen. La sesión no
permite otra acción simultánea ni una mutación externa del Mundo; al reanudar se
utiliza el contexto guardado al aceptar el viaje.

Una solicitud aceptada termina en llegada o en interrupción por una escalada
fallida. No existe una opción que cancele voluntariamente el viaje, cambie el
destino o divida al Grupo.

## 7. Suceso `Trayecto tranquilo`

El índice `0` selecciona una ejecución sin decisión ni complicación. Emite antes
de la llegada este fragmento exacto:

| Texto | `ClaveEscena` |
|---|---|
| `El viaje avanza por la ruta prevista sin contratiempos.` | `Some(ClaveEscena::ViajeMarcha)` |

No añade duración aparte del día base de cada conexión y no realiza otra
extracción de azar.

## 8. Suceso contextual `Paso empinado`

Un índice mayor que cero selecciona el Lugar `empinado` asociado a esa
candidatura. Su identidad se conserva en el contexto interno para mantener el
vínculo con el elemento que originó el suceso y permitir su verificación, pero
esta versión no la interpola ni aplica diferencias entre Lugares.

La presentación y la solicitud usan estos valores exactos:

| Salida | Texto | `ClaveEscena` |
|---|---|---|
| Fragmento inicial | `El camino atraviesa una pendiente empinada. El terreno vuelve difícil mantener el ritmo.` | `Some(ClaveEscena::ViajeObstaculo)` |
| Solicitud de decisión | `La pendiente exige elegir cómo continuar el viaje.` | `Some(ClaveEscena::ViajeObstaculo)` |

La decisión ofrece exactamente dos opciones en este orden:

1. `Ascender directamente`;
2. `Buscar un paso menos empinado`.

`Ascender directamente` invoca la ficha compartida de `Escalar`. El dirigente
es responsable; para el jugador, el primer compañero es ayudante. La acción
particular evalúa los dos Lugares de la etapa, los viajeros, la capacidad, el
equipo, la carga y los estados documentados en su
[ficha](./particulares/01-movimiento-terrestre.md#5-escalar). Un intento
incierto consume exactamente una extracción con límite `20`; una resolución
automática no consume azar.

Los resultados visibles son:

| Resultado | Texto | Consecuencia persistente |
|---|---|---|
| holgada | `Los viajeros superan la pendiente con seguridad y sin perder el ritmo.` | llegada |
| lograda | `Los viajeros completan el ascenso y continúan la marcha.` | llegada |
| costosa | `Los viajeros completan el ascenso, pero el esfuerzo deja fatigado al responsable.` | `fatigado` y llegada en el mismo lote |
| fallida | `El ascenso fracasa. Un viajero resulta herido y el viaje queda interrumpido.` | `herido`, sin traslado |
| paso menos empinado | `Los viajeros encuentran un paso menos empinado a costa de una demora.` | llegada con un día adicional |

Todos usan `Some(ClaveEscena::ViajeObstaculo)`. Buscar un paso menos empinado
continúa siendo determinista y no invoca `Escalar`.

Cuando actúa un NPC, la fase NPC primero elige una de las dos opciones con una
extracción de límite `2`. Si elige ascender, consume después el mismo d20 que el
jugador necesitaría en igual contexto. Una interrupción produce un resumen NPC
válido y no se convierte en fallo técnico.

## 9. Llegada y presentación

Todo viaje que alcanza su destino emite después de Trayecto tranquilo o del
resultado de Paso empinado una única viñeta de llegada:

| Texto | `ClaveEscena` |
|---|---|
| `El viaje finalmente alcanza su destino.` | `Some(ClaveEscena::ViajeLlegada)` |

Los literales de esta acción no interpolan nombres de Lugares o Actores y no
contienen saltos de línea iniciales o finales. El núcleo entrega los fragmentos
en el orden definido y el frontend decide cómo dibujar sus viñetas, pero no
puede omitir una decisión ni reordenar resultados.

`ClaveEscena` es solamente una indicación semántica de presentación. No contiene
arte, rutas de archivos, coordenadas ni datos de gameplay. Esta versión deja de
producir `ClaveEscena::ViajeRumboIncierto`; conservar temporalmente esa variante
para compatibilidad de código o recursos no la convierte en un suceso vigente
de `Viajar`.

El frontend continúa mostrando únicamente tiles y nombres de Lugares. La ruta
calculada no se dibuja y el jugador elige solo el destino final.

## 10. Duración

La duración total es:

```text
un día × cantidad de conexiones de la ruta
+ un día adicional si se busca un paso menos empinado
```

Por tanto, una ruta válida siempre aporta al menos un día. Ejemplos:

| Conexiones | Suceso y resultado | Duración total |
|---:|---|---:|
| 1 | Trayecto tranquilo | 1 día |
| 2 | Trayecto tranquilo | 2 días |
| 2 | ascenso holgado, logrado o costoso | 2 días |
| 2 | paso menos empinado | 3 días |
| 2, pendiente en la primera conexión | ascenso fallido | 1 día |

El día aportado por `Escalar` sustituye el día base de esa conexión; no se
suma dos veces. Ante un fallo se cuentan las etapas previas a la pendiente y el
día del intento, pero no las posteriores. Hasta PLAN-2026-014, `Viajar` solo
confirma el traslado final: si la escalada falla, el Grupo permanece
lógicamente en el Lugar donde comenzó la acción. La Expedición por etapas
reemplazará esta limitación transitoria.

Todos los aportes usan `Duracion::checked_add`. Un desbordamiento es un fallo
fatal del motor, no una duración truncada ni un rechazo de gameplay.

`Viajar` entrega el total al motor y nunca modifica directamente la fecha. La
sesión aplica una sola vez la duración final cuando el jugador termina de cerrar
la acción. La duración calculada para un NPC describe su acción dentro de la
franja ya transcurrida y no vuelve a adelantar la fecha.

## 11. Consecuencia reglada y atomicidad

Después de emitir la viñeta de llegada y antes de cerrar el tramo como
`Completada`, la acción incluye el traslado en un `LoteOperaciones`:

- para el jugador, mueve en una operación al jugador y a todos sus compañeros;
- para un NPC independiente, mueve únicamente a ese Actor;
- usa el origen y el destino final, nunca cada Lugar recorrido antes del
  destino.

El lote cambia todas sus consecuencias o ninguna. Un ascenso costoso incluye el
alta de `fatigado` antes del traslado; una operación idempotente no emite un
resultado duplicado. Una vez confirmado, el motor emite exactamente un
`ResultadoSemantico::ActoresTrasladados { actores, origen, destino }` con los
datos de `CambiosConfirmados`, además de cualquier cambio efectivo de
Característica. Esas emisiones informan consecuencias ya aplicadas; la
aplicación no debe ejecutarlas otra vez.

Un ascenso fallido confirma solamente `herido` y no incluye traslado. `Viajar`
no altera Relaciones, variables globales, Lugares, mapa ni Grupo. Las rutas y
sucesos tampoco crean Objetos implícitos.

Como todas las causas previsibles se validaron antes de iniciar, un
`ErrorTraslado` al finalizar representa una invariante rota y se convierte en un
fallo fatal. Nunca se degrada a una llegada parcial.

## 12. Errores y rechazos

Las causas esperables al iniciar `Viajar` son exhaustivamente:

- Actor desconocido;
- destino desconocido;
- jugador con `muerto`;
- destino igual al Lugar actual;
- destino con `inalcanzable`;
- viaje independiente de un integrante del Grupo;
- destino sin ruta conectada y transitable desde el origen.

La consulta `destinos_disponibles` puede devolver los rechazos que dependen solo
del Actor: Actor desconocido, jugador con `muerto` o viaje independiente de un
integrante del Grupo. No tener destinos produce un vector vacío.

Una decisión inexistente o una opción ajena a Paso empinado usa los rechazos
comunes del motor y conserva sin cambios la ejecución pendiente, el Mundo, la
duración y el azar. Un fallo o índice fuera de rango de la fuente durante la
selección del suceso o la decisión automática de un NPC es fatal según el
contrato común.

Las referencias de una Relación inexistentes, los tipos inválidos y las
inconsistencias de normalización se rechazan al cargar el contenido; no se
convierten en ramas de gameplay. Un grafo válido puede estar desconectado y eso
no constituye un error de contenido.

## 13. Contratos e invariantes

- La conectividad reglada procede exclusivamente de Relaciones `conectado`.
- Una Relación `conectado` se recorre en ambos sentidos.
- La disponibilidad y la confirmación usan las mismas reglas de
  transitabilidad.
- `inalcanzable` no aparece como destino ni en un Lugar atravesado; solo se
  tolera en el origen actual para permitir salir.
- La BFS minimiza conexiones y resuelve empates mediante orden canónico.
- La ruta no depende del mapa, sus tiles, coordenadas ni distancias visuales.
- Los Lugares marítimos usan el mismo grafo y presuponen una embarcación sin
  crear estado adicional.
- El jugador selecciona un destino final y no administra los tramos.
- Cada conexión aporta exactamente un día base.
- Trayecto tranquilo siempre es candidato y cada Lugar `empinado` atravesado
  aporta una candidatura en orden de ruta.
- Exactamente una candidatura se selecciona y como máximo se ejecuta una
  complicación.
- Jugador y NPC usan las mismas reglas y acciones particulares.
- Un viaje llegado produce exactamente un traslado atómico final; una escalada
  fallida produce una interrupción sin traslado.
- Ningún viajero ocupa lógicamente los Lugares atravesados antes del destino
  durante la ejecución.
- La acción no introduce sistemas de inventario, transporte, preparación,
  conocimiento o equipo.
- Todo azar que afecta al viaje usa la fuente inyectada y es reproducible.

## 14. Pruebas y criterios de aceptación

### Grafo, disponibilidad y ruta

Las pruebas automáticas cubrirán:

- una única declaración `conectado` recorrible en ambos sentidos;
- destinos alcanzables de forma directa y transitiva;
- exclusión del origen y de componentes desconectados;
- vector vacío válido para un Actor aislado;
- orden global canónico de `destinos_disponibles`, independiente del orden de
  descubrimiento y declaración de Relaciones;
- ruta con la menor cantidad de conexiones;
- empate entre dos rutas mínimas resuelto mediante vecinos en orden canónico;
- mismo resultado al invertir extremos o reordenar las declaraciones;
- destino `inalcanzable` excluido y rechazado;
- Lugar de la ruta con `inalcanzable` no atravesado, usando una alternativa válida
  cuando exista y dejando el destino sin ruta cuando no exista;
- salida permitida desde un origen actual `inalcanzable`;
- independencia completa de tiles, coordenadas y distancia visual;
- nueva validación y cálculo de ruta al confirmar el destino;
- rechazo `DestinoSinRutaConectada` con origen y destino exactos, sin azar,
  duración, narrativa ni mutaciones.

Un caso de empate mínimo utilizará, en orden canónico, `origen`, `paso_a`,
`paso_b`, `destino`, con rutas `origen → paso_a → destino` y
`origen → paso_b → destino`; deberá elegir la primera.

### Candidaturas, complicación, duración y azar

Las pruebas cubrirán:

- un día base por conexión para rutas de una y varias conexiones;
- lista con solo Trayecto tranquilo y extracción de límite `1` cuando la ruta
  carece de `empinado`;
- candidatura de Paso empinado por un Lugar atravesado y por el destino;
- ausencia de candidatura por `empinado` en el origen;
- con varios Lugares `empinado`, candidaturas en orden de ruta y selección
  individual de cada una;
- selección uniforme con una sola extracción de límite
  `1 + cantidad de Lugares atravesados con empinado`;
- índices estables: `0` para Trayecto tranquilo y `1..` para cada Paso empinado;
- elección de ruta sin preferencia por evitar `empinado`;
- aceptación de un Lugar `maritimo` conectado sin exigir inventario, barco u
  otro estado adicional;
- textos, opciones, claves de escena y orden exactos;
- evaluación imposible, automática e incierta de `Escalar`;
- cuatro bandas del d20, con un único factor por eje causal;
- ayuda de un compañero competente sin acumular varios bonos iguales;
- un día adicional al buscar un paso menos empinado, con independencia de las
  Características de los viajeros;
- exclusión de otros Actores presentes y de Características no mencionadas;
- alta de `fatigado` solo en banda costosa y de `herido` solo en fallida;
- ninguna extracción posterior para Trayecto tranquilo o para buscar un paso;
- para un NPC con Paso empinado, selección de suceso, extracción `2 → 0/1`
  para la opción y, si asciende de forma incierta, exactamente un d20;
- respuesta inválida sin cambios ni una extracción adicional;
- error o índice inválido de la fuente convertido en fallo fatal;
- desbordamiento de duración convertido en fallo fatal.

### Llegada y consecuencias

Las pruebas cubrirán:

- llegada desde Trayecto tranquilo, rodeo y cada banda exitosa de `Escalar`;
- una única viñeta de llegada después de cada resultado exitoso y ninguna tras
  una escalada fallida;
- ubicación sin cambios mientras la acción espera una decisión;
- exactamente un traslado al final, sin traslados por cada Lugar recorrido;
- traslado atómico del jugador seguido por todos sus compañeros;
- traslado del único NPC independiente;
- un único `ActoresTrasladados` con actores, origen y destino exactos al llegar;
- lote costoso con fatiga y traslado, y lote fallido con herida sin traslado;
- duración final correcta sin una segunda modificación de fecha por acciones
  NPC;
- ausencia de mutaciones persistentes aparte de ubicación, `fatigado` o
  `herido` conforme al resultado;
- fallo de traslado posterior a la aceptación convertido en invariante fatal;
- misma lógica reglada para jugador y NPC.

La prueba de integración mínima usará una red que permita viajar desde la zona
inicial hasta el valle a través de un Paso Escarpado con `empinado`. Forzará por
separado los índices `0` y `1` para verificar las dos candidaturas equiprobables,
la selección del destino final, la ruta automática, la ayuda de un compañero,
el d20, la llegada o interrupción, la atomicidad y el regreso de la sesión al
turno del jugador.

La prueba manual comprobará que el frontend ofrece solamente destinos
alcanzables, centra el destino seleccionado sin dibujar una ruta, presenta la
complicación como una secuencia coherente de viñetas y vuelve al mapa en el
destino final.

## 15. Preguntas bloqueantes

Ninguna en este componente.

## 16. Decisiones aplazadas

Quedan fuera de este alcance hasta que una mecánica aprobada las necesite:

- sucesos adicionales derivados de `arido` u otras Características;
- pesos diferentes para las candidaturas;
- elección manual, descubrimiento, ocultamiento o presentación gráfica de
  rutas alternativas;
- rutas ponderadas por dificultad, duración, terreno o clima, así como pesos,
  distancias u otros datos adicionales dentro de `Conectado`;
- interrupción, fracaso, cambio de destino o permanencia en un Lugar anterior
  al destino;
- más de una complicación contextual por viaje;
- heridas o consecuencias persistentes distintas del traslado final, incluidas
  modificaciones de Lugares, Relaciones o mapa;
- permisos de facción, fronteras, conocimientos de viaje y Relaciones que
  involucren Actores;
- objetos, inventario, provisiones, monturas, barcos explícitos, sistemas de
  medios de transporte, equipo o recursos consumibles;
- separación temporal, abandono o expulsión de integrantes del Grupo;
- tratamiento general de un NPC independiente con `muerto`;
- proyección visual de una ruta sobre el mapa;
- configuración de sucesos, textos o reglas de `Viajar` desde contenido o Lua.

Incorporar una de estas capacidades requiere primero definir su efecto de juego
y actualizar el documento canónico correspondiente. Ninguna se infiere de los
nombres de Características o Relaciones que un mod pudiera añadir.
