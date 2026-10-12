# Modelo de Partida y Mundo

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.3 |
| Fecha | 2026-10-12 |

## 1. Propósito

Este documento define la propiedad de los datos, las referencias y las
invariantes mínimas de la Partida y del Mundo descritos en el
[diseño general](../../diseno-general.md), dentro de los límites del
[diseño técnico general](../diseno-tecnico-general.md). Incluye actores, lugares,
grupo, características, relaciones y el vínculo obligatorio con el mapa visual.

Quedan fuera los formatos de contenido, reglas de acciones, calendario, Lua,
conducta de NPC, composición visual, guardado y generación procedural.

## 2. Propiedad del estado

Aquí se distinguen dos clases de propiedad que no implican dos Mundos:

- **propiedad del código:** `desert-tiger-core` define el tipo `Mundo`, sus
  operaciones y sus invariantes;
- **propiedad en ejecución:** la `Partida`, administrada por
  `desert-tiger-app`, almacena una única instancia concreta de ese tipo.

Por lo tanto, `Mundo` no está copiado ni dividido entre ambos crates. La
aplicación posee el valor y solo puede consultarlo o modificarlo mediante la API
definida por el núcleo.

Conceptualmente, sin fijar todavía los nombres Rust definitivos:

```text
desert-tiger-core define: Mundo + sus operaciones

desert-tiger-app posee:   Partida
                         ├── mundo: Mundo      (la única instancia)
                         └── mapa: EstadoMapa  (presentación)
```

Además de esa única instancia de `Mundo`, la Partida posee el estado actual del
mapa y tiene acceso de solo lectura a las definiciones de presentación del
contenido validado. Otros estados de sesión se detallarán en sus documentos
correspondientes.

La instancia de `Mundo` es la autoridad sobre el gameplay. Posee o referencia
mediante identificadores:

- la fecha, variables globales, lugares y actores;
- el actor jugador;
- el grupo del jugador;
- características y relaciones;
- la identidad opaca del único mapa visual asociado.

Sus datos mutables no se exponen directamente. Cada operación semántica del
núcleo preserva las invariantes o falla sin cambios parciales. La atomicidad de
una transición completa que combine varias operaciones pertenece al motor de
acciones.

## 3. Identificadores y referencias

Cada clase de elemento tendrá un identificador tipado: un `IdActor` no podrá
usarse como `IdLugar`, `IdRelacion` o `IdMapa`. Su representación concreta se
decidirá junto con el esquema de contenido.

Los identificadores cumplen estos contratos:

- son únicos por clase y estables durante la Partida;
- no son índices ni coordenadas;
- toda referencia resuelve a un elemento existente;
- permiten señalar elementos sin referencias mutables.

La carga convierte referencias externas en identificadores tipados. El núcleo
comprueba igualmente sus invariantes al construir el Mundo. La unicidad por
clase descrita aquí corresponde a los IDs internos; el espacio y la sintaxis de
los identificadores escritos en contenido se definen en el documento de carga.

## 4. Elementos reglados

### Lugar

Un Lugar tiene identidad y características. No almacena tiles ni coordenadas.
Tampoco mantiene otra lista mutable de sus actores: los presentes se obtienen
consultando la ubicación de cada Actor, única fuente de verdad.

### Actor

Todos los actores comparten el mismo tipo de dominio. Cada Actor tiene:

- identidad;
- exactamente un `IdLugar` como ubicación lógica;
- un conjunto de características.

Ser jugador o NPC no crea subtipos. El Mundo identifica a un único actor como
jugador; la aplicación determina el origen de sus decisiones.

El arte ASCII y la política Lua se asocian al `IdActor` fuera del núcleo. No son
parte de su estado reglado.

### Grupo

El Grupo registra al jugador y a los reclutados, sin duplicados. No posee una
ubicación propia: esta se deriva de sus integrantes.

El jugador pertenece al Grupo. Todos sus integrantes están en el mismo Lugar y no
pueden separarse. Mover al Grupo cambia atómicamente el `IdLugar` de todos o no
mueve a ninguno.

Reclutar y determinar la participación de compañeros corresponde al motor de
acciones. Todo cambio de integrantes debe mantener estas invariantes.

### Característica

Una Característica es una etiqueta asociada a un Actor o Lugar. Se almacena sin
duplicados y se consulta por presencia.

Las reglas pueden consultarlas y modificarlas. El significado de etiquetas como
`muerto` o `inalcanzable` se aplica en las acciones correspondientes; sintaxis y
normalización se definirán con el esquema de contenido.

### Relación

Una Relación identificada conecta dos elementos existentes y posee un tipo. Este
modelo no presupone que sea dirigida, simétrica o exclusiva.

Viajar no depende todavía de `conectado`. La disponibilidad, semántica y
operaciones de las relaciones se diseñarán cuando una regla vaya a utilizarlas.

## 5. Mapa y datos de presentación

Toda Partida requiere un mapa visual. `Mundo` conserva su `IdMapa` opaco como
referencia conceptual al mapa de la Partida, pero no puede consultar tiles ni
coordenadas. No será una referencia Rust prestada al objeto: la sesión posee
ambos estados y `desert-tiger-app` resuelve el identificador estable.

La aplicación distingue:

- un catálogo inmutable perteneciente al contenido validado, con las
  definiciones visuales cargadas;
- un estado de mapa propio de la Partida, cuya matriz de tiles sí puede cambiar.

La Partida solo necesita acceso de lectura al catálogo. Este diseño no exige
copiarlo para cada sesión ni decide todavía cómo se compartirá en Rust.

Las definiciones de presentación contienen:

- la matriz inicial del mapa;
- el nombre visible de cada Lugar, distinto de su identificador;
- el tile central y las coordenadas de rótulo de cada Lugar;
- el arte ASCII de los actores que lo tengan definido;
- cualquier otro dato exclusivamente visual que se defina después.

Al crear la Partida, `desert-tiger-app` instancia la matriz actual a partir de
esa definición. Puede modificarla para representar un resultado semántico del
Mundo —por ejemplo, mostrar un bosque quemado— sin modificar el catálogo ni el
mapa de otra Partida. La forma exacta de traducir cada cambio semántico a una
zona y unos tiles se diseñará cuando se implemente esa capacidad.

El flujo es unidireccional: el estado reglado puede provocar una actualización
visual, pero cambiar el dibujo no cambia ubicaciones lógicas, características,
disponibilidad de acciones ni resultados. El núcleo no recibe la matriz actual.
Toda condición duradera que deba consultar una regla queda confirmada en el
Mundo; la matriz solo la representa y nunca se usa para reconstruirla.

## 6. Entradas, consultas y salidas

La construcción recibe datos deserializados y validados por la capa de contenido.
El núcleo produce un Mundo válido o un error contextualizado, nunca uno parcial.

El Mundo ofrece consultas de solo lectura para:

- obtener actores, lugares y relaciones por identificador;
- conocer la ubicación de un actor;
- enumerar actores presentes en un lugar;
- consultar integrantes y ubicación común del Grupo;
- consultar características, fecha y variables globales.

La aplicación recibe valores o vistas inmutables. La mutación usa operaciones
semánticas, no acceso libre a colecciones. El motor de acciones definirá quién
puede invocarlas y en qué orden.

## 7. Contratos e invariantes

Un Mundo válido cumple siempre que:

- existe exactamente un actor jugador y pertenece al Grupo;
- todo Actor referencia exactamente un Lugar existente;
- todos los integrantes del Grupo están en el mismo Lugar;
- un Actor aparece como máximo una vez en el Grupo;
- toda Relación referencia elementos existentes;
- todos los identificadores son únicos dentro de su clase;
- existe un `IdMapa` y la Partida posee las definiciones asociadas y una
  instancia actual de ese mapa;
- tiles y coordenadas nunca intervienen en una consulta o mutación reglada;
- una mutación aceptada conserva todas las invariantes;
- una mutación rechazada no altera el estado observable.

No se duplica la ubicación en Actor, Lugar y Grupo. La única ubicación persistida
es la de cada Actor; las demás vistas se derivan de ella.

## 8. Errores y validaciones

La creación de la Partida debe rechazar, con el identificador y contexto
pertinentes:

- identificadores duplicados o referencias inexistentes;
- ausencia o multiplicidad del actor jugador;
- un Actor sin Lugar válido;
- un Grupo sin el jugador, con duplicados o con integrantes separados;
- Relaciones con extremos inexistentes;
- ausencia del mapa obligatorio o divergencia entre su identidad, las
  definiciones y la instancia visual;
- datos de presentación que referencien actores o lugares inexistentes, cuando
  esa referencia sea requerida por el contenido.

En ejecución se rechazan identificadores desconocidos, cambios que rompan el
Grupo y transiciones inválidas. Los errores distinguirán datos iniciales
inválidos de solicitudes de juego inválidas, sin fallos silenciosos ni `panic`
ante entradas esperables.

## 9. Pruebas y criterios de aceptación

Las pruebas unitarias del núcleo cubrirán al menos:

- construcción de un Mundo mínimo válido;
- rechazo individual de cada invariante rota;
- consulta de actores por Lugar sin una lista duplicada;
- alta, consulta y retirada de características sin duplicados;
- traslado atómico de todos los integrantes del Grupo;
- rechazo de un traslado que dejaría integrantes separados;
- resolución de referencias y relaciones por identificadores tipados;
- ausencia de cambios después de una operación rechazada.

Las pruebas de aplicación comprobarán el enlace con mapa, coordenadas y arte;
que la aplicación pueda modificar los tiles de una Partida sin cambiar el Mundo
reglado; y que no afecte al mapa de otra Partida ni a las definiciones fuente. La
traducción automática de un resultado semántico a una región visual se probará
cuando esa capacidad sea diseñada. Estas pruebas no dependerán del frontend,
Lua, el orden de archivos ni el azar.

El componente estará aceptado cuando una Partida mínima pueda crearse, consultarse
por APIs inmutables y mutarse sin romper invariantes. `desert-tiger-core` tampoco
podrá acceder a tiles o coordenadas por sus tipos o dependencias.

## 10. Preguntas pendientes

- ¿Qué tipos de valores podrán almacenar las variables globales del Mundo?
- ¿Qué clases de elementos podrán ocupar los extremos de una Relación y cómo se
  expresarán sus tipos?
- ¿Qué semántica tendrán dirección, simetría y multiplicidad cuando las
  relaciones comiencen a intervenir en reglas?
- ¿Qué operaciones de ciclo de vida necesitarán actores, lugares y relaciones
  una vez iniciada la Partida?
- ¿Qué identidad y cambio del dominio comunicará el núcleo, y cómo los traducirá
  `desert-tiger-app` a una región y una transformación definidas exclusivamente
  en los datos de presentación?
