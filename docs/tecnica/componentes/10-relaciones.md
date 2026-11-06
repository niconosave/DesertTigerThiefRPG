# Relaciones entre elementos del Mundo

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.6 |
| Fecha | 2026-11-04 |

> Ampliación diseñada, no implementada: el [contrato 24](./24-agenda-eventos-mundo.md)
> incorpora el registro histórico de rescate y sus índices como operaciones
> del modelo común de eventos. No crea categorías distintas de relaciones para
> rutinas de actor y episodios externos.

> Ampliación implementada el 2026-10-26. Filiación, MatadoPor e índices familiares: [contrato 14, secciones 4–5](./14-parentesco-actores-contexto.md#5-almacenamiento-índices-y-consultas-familiares).

> Ampliación implementada el 2026-10-29. `OrigenDe`, `AfluenteDe` y `Bloquea`:
> [sección 13](#13-nacimiento-afluentes-y-bloqueo--implementados-el-2026-10-29).

> Ampliación implementada el 2026-11-04: comercio dirigido con mercancía y
> lectura inversa, según la [sección 14](#14-comercio-dirigido-con-mercancía).

## Revisión implementada el 2026-10-24: Conoce adquirido durante el viaje

Se conserva Conectado y el grafo indexado. RegistrarConocimiento incorpora una Relación Conoce idempotente junto a todos sus índices. No se añade Abandono ni atributos a Conectado. La sección 10 de 06 y las operaciones del Mundo fijan la confirmación atómica.

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

## 1. Propósito y alcance

Este documento es la fuente canónica para representar, cargar, almacenar y
consultar Relaciones entre elementos del Mundo. Desarrolla el concepto de
Relación del [diseño general](../../diseno-general.md) sin trasladar al sistema
de relaciones las reglas particulares de una acción.

Este documento describe el alcance ya implementado con `Conectado`. La
ampliación aprobada `Conoce` se define en
[`12-conocimiento-lugares.md`](./12-conocimiento-lugares.md) y se incorporará a
este componente durante su plan de implementación.

El incremento comprende:

- declaraciones de Relación incorporadas en el mismo archivo RON del elemento
  que actúa como origen de autoría;
- resolución de sus referencias durante la carga;
- una representación reglada genérica cuyos extremos pueden identificar un
  Actor o un Lugar;
- almacenamiento único de cada Relación e índices para consultarla desde
  cualquiera de sus extremos sin recorrer todas las Relaciones;
- el primer tipo concreto `Conectado`, exclusivamente entre dos Lugares y con
  significado simétrico;
- una consulta especializada de vecinos geográficos para que las reglas de
  `Viajar` puedan recorrer el grafo.

`Conectado` se declara una sola vez. Que pueda consultarse desde ambos Lugares
no crea una segunda Relación y no obliga a repetirla en el archivo del otro
Lugar.

Quedan fuera de este incremento:

- otros tipos de Relación;
- Relaciones válidas que involucren Actores;
- dirección o lecturas inversas con significados diferentes;
- características, estados, metadatos, nombres o IDs externos propios de una
  Relación;
- crear, eliminar o modificar Relaciones durante una Partida;
- declarar Relaciones mediante Lua o exponerlas a su API;
- deducir conectividad a partir del mapa, coordenadas o proximidad visual;
- dibujar rutas o conexiones en un frontend;
- objetos, inventario, medios de transporte, equipo, preparativos o
  conocimiento de un trayecto.

Estas exclusiones no impiden usar las características ya existentes de los
Actores y Lugares desde una acción. Solo evitan convertir ideas conceptuales
futuras en categorías nuevas antes de que una regla aprobada las necesite.

## 2. Responsabilidades y límites

`desert-tiger-app` es responsable de leer las declaraciones RON, conservar su
origen para los diagnósticos, resolver claves externas a IDs tipados y entregar
al núcleo datos completos. Los DTO y las cadenas de las claves no atraviesan esa
frontera.

`desert-tiger-core` define los tipos reglados, vuelve a validar todos los
extremos e invariantes, almacena cada Relación una sola vez y construye los
índices de consulta dentro de `Mundo`. Esos índices son estado derivado del
conjunto validado: no constituyen Relaciones adicionales ni otra fuente de
verdad.

Las acciones consultan el grafo mediante la API de `Mundo`. El componente de
Relaciones no elige destinos, no calcula caminos completos, no interpreta
Características, no traslada Actores y no consume azar. Las reglas concretas de
selección de destino, ruta y consecuencias pertenecen a
[`Viajar`](../acciones/02-viajar-expediciones.md).

`desert-tiger-terminal` no lee archivos de Relaciones ni ejecuta consultas para
decidir gameplay. Presenta las acciones y destinos que recibe de la fachada de
aplicación. El Mapa continúa siendo visual y no interviene en la construcción
del grafo.

Centralizar las Relaciones resueltas dentro de `Mundo` es una organización de
ejecución, no una exigencia de autoría. Quien diseña contenido encuentra cada
declaración junto a uno de sus elementos; al comenzar la Partida, el programa la
convierte en estructuras adecuadas para consultar muchas Relaciones con bajo
costo.

## 3. Modelo reglado

### 3.1. Tipos y extremos

La API pública pertenece al módulo del modelo de `desert-tiger-core`. Los tipos
iniciales son equivalentes a:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdRelacion(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExtremoRelacion {
    Actor(IdActor),
    Lugar(IdLugar),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TipoRelacion {
    Sirviente,
    Conoce,
    Padre,
    Madre,
    MatadoPor,
    Conectado,
    Abarca,
    OrigenDe,
    AfluenteDe,
    Bloquea,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelacionInicial {
    pub mercancia: Option<String>,
    pub tipo: TipoRelacion,
    pub origen: ExtremoRelacion,
    pub destino: ExtremoRelacion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relacion {
    mercancia: Option<String>,
    id: IdRelacion,
    tipo: TipoRelacion,
    origen: ExtremoRelacion,
    destino: ExtremoRelacion,
}
```

`RelacionInicial` es un dato consumible de construcción. Sus nombres `origen` y
`destino` preservan la forma en que la aplicación resolvió una declaración
incorporada en un elemento. No implican dirección para `Conectado`.

`Relacion` conserva ese origen y destino para representar fielmente el hecho
declarado. La simetría de `Conectado` pertenece a su semántica y a sus índices:
no reescribe ni invierte el objeto. El par ordenado por `IdLugar` se calcula
solamente como clave temporal para detectar duplicados y nunca se almacena como
una segunda Relación.

`IdRelacion` es opaco, copiable y asignado desde cero conforme a la posición de
la declaración resuelta. Su entero privado permite resolver una Relación en
tiempo constante, pero no se serializa ni es una identidad estable de contenido.
Reordenar elementos o sus listas puede cambiarlo en una Partida nueva.

La API de solo lectura es:

```rust
impl Relacion {
    pub fn mercancia(&self) -> Option<&str>;
    pub fn id(&self) -> IdRelacion;
    pub fn tipo(&self) -> TipoRelacion;
    pub fn origen(&self) -> ExtremoRelacion;
    pub fn destino(&self) -> ExtremoRelacion;
    pub fn extremos(&self) -> (ExtremoRelacion, ExtremoRelacion);
    pub fn contiene(&self, extremo: ExtremoRelacion) -> bool;
    pub fn otro_extremo(
        &self,
        extremo: ExtremoRelacion,
    ) -> Option<ExtremoRelacion>;
}
```

`extremos` devuelve `(origen, destino)` en el orden conservado. `otro_extremo`
devuelve `None` cuando el elemento recibido no participa en esa Relación. Como
las relaciones reflexivas están prohibidas, una respuesta existente siempre es
inequívoca. Poder obtener el otro extremo no cambia por sí solo el significado
del tipo; para `Conectado` ambos sentidos están expresamente aprobados.

### 3.2. Semántica inicial de `Conectado`

Un `Conectado` representa una conexión geográfica directa entre dos Lugares.
Cumple exactamente estas reglas:

- ambos extremos son `ExtremoRelacion::Lugar`;
- los dos IDs existen en el mismo Mundo;
- los Lugares son distintos;
- el par no aparece en otra Relación `Conectado`, sin importar desde cuál de
  los dos archivos fue declarado;
- A conectado con B significa también B conectado con A;
- no contiene distancia, dificultad, medio admitido ni otra información;
- no garantiza por sí solo que los viajeros posean el medio necesario ni
  determina las consecuencias de recorrerlo.

`Conectado` puede unir dos Lugares terrestres, dos acuáticos o uno de cada tipo.
La Relación no inspecciona Características ni almacena un medio: `Viajar`
comprueba por separado el acceso a bote o barco, y una entrada al agua no
presupone una embarcación.

Una cadena de varias Relaciones puede formar un camino, pero la existencia y
elección de ese camino son deducciones de la acción consumidora. El sistema de
Relaciones solo informa vecinos directos.

### 3.3. Construcción, almacenamiento e índices

`DatosInicialesMundo` incorpora una lista obligatoria:

```rust
pub struct DatosInicialesMundo {
    // Campos definidos por el modelo de Mundo.
    pub relaciones: Vec<RelacionInicial>,
}
```

La secuencia puede estar vacía para un Mundo que todavía no tenga Relaciones.
La aplicación la produce recorriendo primero los Lugares en el orden canónico de
`world.ron` y luego los Actores en su propio orden canónico; dentro de cada
elemento conserva el orden escrito en `relaciones`. En el alcance actual toda
lista de Actor debe estar vacía, pero participa del contrato de formato para que
las declaraciones continúen junto a los elementos cuando se apruebe un tipo
compatible.

`Mundo` incorpora conceptualmente estos campos privados:

```rust
pub struct Mundo {
    // Campos definidos por el modelo de Mundo.
    relaciones: Vec<Relacion>,
    relaciones_por_lugar: Vec<Vec<IdRelacion>>,
    relaciones_por_actor: Vec<Vec<IdRelacion>>,
    conectados_por_lugar: Vec<Vec<IdLugar>>,
}
```

`relaciones` es la única colección de objetos Relación. Los otros campos son
índices derivados que almacenan exclusivamente IDs copiados:

- `relaciones_por_lugar` y `relaciones_por_actor` permiten hallar las
  Relaciones incidentes a un elemento mediante su ID;
- `conectados_por_lugar` permite recorrer directamente los vecinos requeridos
  por el grafo geográfico sin filtrar la colección global.

Los índices exteriores tienen exactamente una entrada por Lugar o Actor y usan
la posición comprobada de su ID. No hay punteros, referencias Rust almacenadas
entre objetos ni búsquedas lineales sobre todas las Relaciones durante una
consulta normal.

Cada lista de Relaciones incidentes está ordenada por `IdRelacion`. Cada lista
de `conectados_por_lugar` está ordenada por `IdLugar`, que coincide con el orden
canónico de Lugares. Por ello, el resultado no depende del extremo desde el que
se declaró una conexión, del orden de lectura del filesystem ni del orden de una
tabla hash.

La construcción consume la lista, valida todos los datos antes de publicar el
Mundo y deriva los tres índices. No existe una ruta de construcción que acepte
índices externos ya preparados. Crear dos Mundos desde el mismo contenido
produce Relaciones e índices equivalentes, pero poseídos independientemente.

El almacenamiento ocupa `O(A + L + R)`, donde `A` es la cantidad de Actores,
`L` la cantidad de Lugares y `R` la cantidad de Relaciones. Un `Conectado`
aparece una vez en `relaciones` y aporta una entrada liviana a cada extremo de
los índices; nunca se duplica el objeto completo.

### 3.4. Consultas de `Mundo`

La API pública es:

```rust
impl Mundo {
    pub fn relaciones(&self) -> &[Relacion];

    pub fn relacion(
        &self,
        id: IdRelacion,
    ) -> Option<&Relacion>;

    pub fn relaciones_de(
        &self,
        extremo: ExtremoRelacion,
    ) -> Option<&[IdRelacion]>;

    pub fn lugares_conectados(
        &self,
        lugar: IdLugar,
    ) -> Option<&[IdLugar]>;
}
```

`relacion` y las dos consultas indexadas resuelven en `O(1)`, sin contar el
recorrido posterior del slice. `relaciones_de` devuelve `None` si el Actor o
Lugar no existe; si existe pero no tiene Relaciones devuelve `Some(&[])`. Sus
IDs resuelven siempre mediante `relacion` dentro del mismo Mundo.

`lugares_conectados` aplica la misma distinción: `None` significa Lugar
inexistente y `Some(&[])`, Lugar existente sin vecinos. El slice contiene cada
vecino directo exactamente una vez y en orden ascendente de `IdLugar`. La
consulta desde cualquiera de los dos extremos de un mismo `Conectado` incluye
al otro.

Ninguna consulta expone una colección mutable, asigna un `Vec`, consume azar,
consulta Características o cambia el Mundo. Un algoritmo que recorra solo las
listas devueltas cuesta en proporción al grado de los Lugares visitados, no al
número total de Relaciones del juego.

## 4. Formato RON y autoría

### 4.1. Esquema normativo

Cada archivo de Lugar y Actor incorpora el campo obligatorio `relaciones`. Es
una lista que puede estar vacía. No se crea un archivo por Relación ni una lista
global en `world.ron`.

Los DTO privados de `desert-tiger-app` son conceptualmente:

```rust
#[derive(Deserialize)]
enum TipoRelacionDto {
    Conectado,
}

#[derive(Deserialize)]
enum DestinoRelacionDto {
    Lugar(String),
    Actor(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeclaracionRelacionDto {
    tipo: TipoRelacionDto,
    destino: DestinoRelacionDto,
}
```

El origen no se repite como campo: es el Actor o Lugar cuyo archivo contiene la
lista. `destino` exige la categoría de la clave para que una clave textual igual
en Actores y Lugares nunca sea ambigua. Los nombres de variantes mostrados son
exactos y distinguen mayúsculas.

El campo es obligatorio incluso cuando está vacío. Omitirlo, usar un tipo no
reconocido o agregar campos a una declaración viola el esquema, de acuerdo con
las reglas generales de [contenido externo](./03-contenido.md).

### 4.2. Ejemplo válido

La conexión oficial entre `zona_arida` y `paso_escarpado` se declara en el
archivo `places/zona_arida.ron`:

```ron
(
    clave: "zona_arida",
    nombre: "Zona Árida",
    caracteristicas: [],
    relaciones: [
        (
            tipo: Conectado,
            destino: Lugar("paso_escarpado"),
        ),
    ],
    coordenada_central: (
        x: 14,
        y: 16,
    ),
    coordenadas_rotulo: [
        (x: 10, y: 15),
        (x: 11, y: 15),
        (x: 12, y: 15),
        (x: 13, y: 15),
        (x: 14, y: 15),
        (x: 15, y: 15),
        (x: 16, y: 15),
        (x: 17, y: 15),
        (x: 18, y: 15),
        (x: 19, y: 15),
    ],
)
```

Entonces `places/paso_escarpado.ron` no repite la lectura inversa; su lista
contiene únicamente la segunda conexión del recorrido oficial:

```ron
relaciones: [
    (
        tipo: Conectado,
        destino: Lugar("valle_fertil"),
    ),
],
```

Las consultas observan ambos vínculos desde cualquiera de sus extremos aunque
cada uno se encuentre declarado una sola vez:

```text
lugares_conectados(zona_arida)     = [paso_escarpado]
lugares_conectados(paso_escarpado) = [zona_arida, valle_fertil]
lugares_conectados(valle_fertil)   = [paso_escarpado]
```

Cada archivo de Actor incorpora de igual manera `relaciones: []` durante este
incremento. El resto de sus campos conserva el esquema definido por el documento
de contenido.

### 4.3. Ejemplos inválidos

Declarar también esto en `paso_escarpado.ron` duplica el primer vínculo y
produce un diagnóstico `Duplicado`:

```ron
relaciones: [
    (
        tipo: Conectado,
        destino: Lugar("zona_arida"),
    ),
],
```

Dentro de `zona_arida.ron`, una conexión reflexiva es un `ValorInvalido`:

```ron
relaciones: [
    (
        tipo: Conectado,
        destino: Lugar("zona_arida"),
    ),
],
```

`Conectado` no admite Actores. Dentro de un archivo de Lugar, la siguiente
declaración es RON bien formado, pero produce `ValorInvalido` por la combinación
de extremos:

```ron
relaciones: [
    (
        tipo: Conectado,
        destino: Actor("cazador"),
    ),
],
```

Una clave bien formada que no resuelve a un elemento enumerado en `world.ron`
produce `ReferenciaInvalida`, no crea un elemento implícito y no se ignora.

## 5. Carga, normalización y validaciones

La carga conserva cada declaración junto con su archivo, elemento y posición en
la lista hasta terminar los diagnósticos. Solo después de validar y resolver sus
claves crea `RelacionInicial`.

La identidad canónica del único tipo inicial es:

```text
(Conectado, min(IdLugar A, IdLugar B), max(IdLugar A, IdLugar B))
```

Esa identidad se usa para detectar tanto repeticiones idénticas como
declaraciones inversas. No se expone como un nuevo ID de contenido y no sustituye
la declaración original en los diagnósticos.

La validación de contenido recorre las declaraciones en el orden definido en la
sección 3.3. Para cada una comprueba, en orden:

1. que la variante y sus campos hayan deserializado conforme al esquema;
2. que la clave de destino tenga la sintaxis externa ya aprobada;
3. que el origen pertenezca al conjunto canónico cargado;
4. que el destino exista en la categoría indicada y pertenezca al mismo Mundo;
5. que el tipo admita las categorías de ambos extremos;
6. que los extremos sean distintos;
7. que la identidad canónica no haya aparecido antes.

Una declaración con referencia o combinación inválida no participa en la
búsqueda de duplicados dependiente, para evitar diagnósticos en cascada. Cuando
hay un duplicado, el diagnóstico de la segunda declaración identifica también el
archivo, elemento y posición de la primera.

Se reutilizan las clases de diagnóstico existentes:

| Problema | Clase |
|---|---|
| Campo `relaciones` ausente, tipo desconocido, forma RON incorrecta o campo adicional | `FormatoRonInvalido` |
| Clave de destino con sintaxis inválida | `ClaveInvalida` |
| Destino inexistente o no enumerado en el Mundo | `ReferenciaInvalida` |
| Categorías incompatibles o extremos iguales | `ValorInvalido` |
| Misma Relación repetida en cualquier orientación | `Duplicado` |
| Rechazo inesperado al comprobar el Mundo después de validar | `InvarianteDominio` |

El diagnóstico usa como `TipoElementoContenido` el tipo del elemento que
contiene la declaración y señala un campo como `relaciones[0].destino`; no se
agrega una categoría de archivo Relación porque tal archivo no existe.

El núcleo repite las invariantes sin confiar en el cargador. Una vez superadas
las validaciones anteriores del modelo de Mundo, procesa `relaciones` por
posición y puede producir estas variantes adicionales de
`ErrorConstruirMundo`:

```rust
pub enum ErrorConstruirMundo {
    // Variantes definidas por el modelo de Mundo.
    PosicionRelacionFueraDeRango {
        posicion: usize,
    },
    ExtremoRelacionDesconocido {
        posicion: usize,
        extremo: ExtremoRelacion,
    },
    ExtremosRelacionIncompatibles {
        posicion: usize,
        tipo: TipoRelacion,
        origen: ExtremoRelacion,
        destino: ExtremoRelacion,
    },
    RelacionReflexiva {
        posicion: usize,
        tipo: TipoRelacion,
        extremo: ExtremoRelacion,
    },
    RelacionDuplicada {
        posicion: usize,
        primera: IdRelacion,
    },
}
```

Para cada posición, `Mundo::nuevo` comprueba: conversión representable a
`IdRelacion`; existencia del origen; existencia del destino; compatibilidad de
categorías; extremos distintos; y ausencia de la identidad canónica. Devuelve
el primer error según ese orden y no publica un Mundo ni índices parciales.

No se usa `as` para convertir la posición. La construcción utiliza una conversión
comprobada a `u32`, igual que los demás IDs internos. Todas las variantes nuevas
implementan los contratos de `Display` y `Error` del error contenedor; el texto
no es API estable.

## 6. Integración con `Viajar`

`Viajar` es el primer consumidor de `lugares_conectados`. El contrato que este
componente garantiza es solamente un grafo no dirigido, validado y con vecinos
en orden canónico.

La acción conserva autoridad sobre:

- qué Lugares se ofrecen como destinos;
- cómo encuentra un camino entre origen y destino;
- cómo desempata entre varios caminos;
- cuándo examina las Características de los Lugares y viajeros;
- cuántas complicaciones presenta, qué decisiones solicita y cuánto tiempo
  transcurre;
- qué rechazo produce cuando no existe un camino admitido.

El recorrido de `Viajar` nunca se deduce de tiles ni coordenadas. Jugador y NPC
consultan el mismo grafo y atraviesan el mismo motor de acciones. Una política
Lua puede solicitar `Viajar` como antes, pero en este incremento no recibe una
API adicional para inspeccionar Relaciones.

La fachada y el frontend no reciben el grafo completo para recalcular reglas.
Continúan recibiendo destinos ya determinados por el núcleo. El frontend no
dibuja la ruta y el Mapa no se modifica al consultar o recorrer conexiones.

## 7. Contratos e invariantes

Un Mundo válido cumple siempre que:

- todo extremo de toda Relación resuelve a un Actor o Lugar existente en ese
  Mundo;
- cada `IdRelacion` coincide con la posición de su Relación y resuelve allí;
- cada Relación aparece una sola vez en la colección canónica;
- cada Relación aparece exactamente una vez en el índice de cada uno de sus
  extremos y en ningún otro;
- todo ID de los índices resuelve en la misma instancia de Mundo;
- cada lista indexada tiene orden determinista;
- `Conectado` une exactamente dos Lugares diferentes y se observa en ambos
  sentidos;
- los vecinos de cada Lugar son únicos y siguen el orden canónico de Lugares;
- no existe un `Conectado` Actor–Actor ni Actor–Lugar;
- mapa, coordenadas y arte no participan en la validez o consulta del grafo;
- consultar una Relación nunca modifica el Mundo;
- una Partida no comparte colecciones mutables de Relaciones con otra.

La representación externa y la interna tienen una sola fuente de verdad en
momentos distintos: antes de iniciar una Partida son los archivos validados;
durante ella es la colección de Relaciones del `Mundo`. Los índices siempre se
derivan de esta última y no pueden mutarse por separado.

## 8. Pruebas y criterios de aceptación

### 8.1. Núcleo

Las pruebas unitarias de `desert-tiger-core` cubrirán:

- construcción sin Relaciones y consultas vacías de elementos válidos;
- construcción de un `Conectado` y almacenamiento de una sola Relación;
- consulta del mismo `IdRelacion` desde ambos extremos;
- resolución por `relacion`, `extremos` y `otro_extremo`;
- vecinos simétricos mediante `lugares_conectados`;
- orden de vecinos por `IdLugar` aunque las declaraciones se encuentren en otro
  orden o en cualquiera de los extremos;
- distinción entre ID inexistente y elemento existente sin Relaciones;
- rechazo de cada extremo desconocido;
- rechazo de Actor–Actor y Actor–Lugar para `Conectado`;
- rechazo de una Relación reflexiva;
- rechazo de un duplicado idéntico y de su declaración inversa;
- rechazo de una cantidad cuya posición no sea representable en `u32`, mediante
  la unidad comprobable del constructor sin reservar una colección imposible;
- precedencia determinista de los errores y ausencia de un Mundo parcial;
- coherencia interna entre la colección canónica y todos los índices;
- independencia completa respecto de Mapa, coordenadas, frontend, Lua y azar.

### 8.2. Contenido

Las pruebas de `desert-tiger-app` cubrirán:

- carga de una declaración válida desde un archivo de Lugar;
- resolución tipada de `Lugar("clave")`;
- exigencia del campo `relaciones` en archivos de Lugar y Actor, admitiendo una
  lista vacía;
- rechazo de tipo desconocido, forma incorrecta, campo adicional y datos
  residuales;
- diagnóstico de clave inválida o referencia inexistente sobre el elemento y
  posición correctos;
- rechazo de `Conectado` declarado desde un Actor o dirigido a un Actor;
- rechazo de conexión reflexiva;
- rechazo de duplicado dentro de una lista, entre dos archivos y en orientación
  inversa, identificando ambas declaraciones;
- orden determinista de diagnósticos sin depender del filesystem;
- rechazo atómico del conjunto completo ante cualquier declaración inválida;
- dos Partidas instanciadas desde el mismo `ContenidoValidado` con grafos
  equivalentes y colecciones poseídas independientemente.

### 8.3. Integración y aceptación observable

Las pruebas de integración cubrirán:

- un fixture con al menos dos Lugares conectados mediante una sola declaración;
- disponibilidad de la conexión para viajar en ambos sentidos;
- una cadena de Lugares cuya consulta permita a `Viajar` alcanzar un destino no
  conectado directamente;
- Lugares desconectados que permanezcan fuera de los destinos alcanzables;
- desempate reproducible gracias al orden canónico de vecinos;
- la misma red y las mismas reglas para el jugador, su Grupo y un NPC
  independiente;
- ninguna consulta o viaje altere el conjunto de Relaciones ni el Mapa.

El incremento queda aceptado cuando una persona pueda declarar una conexión una
sola vez junto a un Lugar, cargarla con diagnósticos claros, consultarla desde
ambos Lugares y completar mediante la interfaz el recorrido de `Viajar` que la
utiliza. Además deberán pasar los controles generales de la
[metodología](../../metodologia-desarrollo.md).

## 9. Preguntas bloqueantes

Ninguna para este alcance.

## 10. Decisiones aplazadas

Se diseñarán solamente cuando una mecánica aprobada las necesite:

- tipos de Relación adicionales a los ya implementados en este contrato y sus
  ampliaciones enlazadas;
- Relaciones que involucren categorías futuras distintas de Actor y Lugar;
- características, estado mutable, metadatos o identidad externa de una
  Relación;
- operaciones para crear o retirar Relaciones durante una Partida;
- consultas de Relaciones desde Lua;
- representación visual de rutas;
- persistencia y migración de `IdRelacion` en partidas guardadas.

Agregar alguno de estos alcances exigirá actualizar este documento antes de
ampliar enums, DTO, índices o API. No se reserva ahora una estructura genérica de
propiedades ni un inventario para anticiparlos.

## 11. Relación Sirviente — implementada el 2026-10-26

`Sirviente` es dirigida Actor→Actor: el origen sirve al destino. Se declara en
el RON de quien sirve, por ejemplo:

```ron
(tipo: Sirviente, destino: Actor("principe_perdido")),
```

La ficha del origen muestra «Sirviente de: {destino}» y la del destino,
«Servido por: {origen}». Se indexa en ambos extremos como los demás vínculos
registrados entre Actores, sin otra declaración inversa. Rechaza referencias
inexistentes, extremos de Lugar, duplicados y autorreferencias. No impone una
cardinalidad adicional ni modifica parentesco, pertenencia al Grupo o conducta.

El contenido oficial declara este vínculo desde Bandido y Sirviente hacia el
Príncipe Perdido. Las pruebas de fichas comprueban las dos lecturas y que no
se deduzca hermandad a partir de esta relación.

## 12. Regiones y Abarca — implementadas el 2026-10-28

Una región es un Lugar con la Característica `region`. `Lugar::es_region()`
consulta esa marca; no se introduce un subtipo ni un archivo de formato distinto.
La Relación dirigida `Abarca` vincula una región de origen con un Lugar de destino:

```ron
// Dentro de un Lugar con caracteristicas: ["region"].
relaciones: [
    (tipo: Abarca, destino: Lugar("zona_arida")),
],
```

Se declara una vez en el RON de la región. Se almacena como una única Relación y
se indexa en `relaciones_por_lugar` para ambos extremos; nunca se incorpora a
`conectados_por_lugar`. Las fichas muestran «Abarca: {destino}» desde la región
y «Dentro de: {origen}» desde el Lugar incluido. Los enlaces pueden explorarse
en el campamento mediante el mecanismo habitual.

El núcleo y la carga rechazan extremos inexistentes o distintos de Lugar,
autorreferencias, declaraciones duplicadas y un origen sin `region`.
`Conectado` rechaza una región en cualquiera de sus extremos, aunque esa región
no declare ninguna Relación Abarca. El diagnóstico identifica el Lugar afectado.

Abarca expresa inclusión declarada, sin calcular polígonos a partir de los tiles,
heredar Características, deducir otras Relaciones ni trasladar Actores.
Un Lugar puede pertenecer a más de una región. No se calcula cierre transitivo;
las consultas de fichas muestran exclusivamente los vínculos declarados.
Los destinos y ambas búsquedas de ruta conservan el grafo exclusivo de Conectado,
por lo que las regiones no ofrecen atajos ni pasan a integrar los viajes de NPC.

Las pruebas del núcleo verifican índices en ambos extremos, destinos de jugador
y NPC, conexiones prohibidas en ambos sentidos y validación de origen/duplicados.
Las de contenido verifican el atlas completo y los diagnósticos; las de fichas,
las dos lecturas navegables. La composición visual se especifica en el
[frontend de terminal](./09-frontend-terminal.md).

## 13. Nacimiento, afluentes y bloqueo — implementados el 2026-10-29

Se incorporan tres tipos dirigidos entre Lugares. El archivo del origen declara
el vínculo una sola vez; el texto inverso de la ficha no es otra relación.

| Tipo RON y del núcleo | Sentido de autoría | Ficha del origen | Ficha del destino |
|---|---|---|---|
| `OrigenDe` | Montaña → río que nace en ella | Origen de: {destino} | Nace de: {origen} |
| `AfluenteDe` | Río afluente → río que recibe sus aguas | Afluente de: {destino} | Recibe al afluente: {origen} |
| `Bloquea` | Montaña → territorio con el que limita | Bloquea: {destino} | Bloqueado por: {origen} |

Ejemplos de autoría en el RON de una montaña y en el de un río, respectivamente:

```ron
// Monte Tazir
relaciones: [
    (tipo: OrigenDe, destino: Lugar("rio_tazir")),
    (tipo: Bloquea, destino: Lugar("valle_fertil")),
],
```

```ron
// Río Tazir
relaciones: [
    (tipo: AfluenteDe, destino: Lugar("rio_tamrut")),
],
```

Cada declaración produce una única `Relacion`, indexada en ambos extremos por
`relaciones_por_lugar`. Conserva su orientación al detectar duplicados; no se
normaliza como una conexión simétrica. Las fichas permiten seguir el vínculo en
ambos sentidos mediante las referencias habituales del campamento.

La carga y el núcleo exigen extremos existentes de tipo Lugar, rechazan
autorreferencias y duplicados del mismo tipo, origen y destino. Un Lugar con
`region` también puede ser extremo de estos vínculos. No se exige una marca
física concreta, ni se añaden restricciones de cardinalidad, ciclos fluviales o
coordenadas: la correspondencia entre montañas y ríos es responsabilidad del
contenido.

Ninguno de estos tipos se incorpora a `conectados_por_lugar`. No habilitan
viajes, no alteran costes ni eliminan conexiones existentes. `Bloquea` registra
el límite geográfico; sus consecuencias futuras sobre el desplazamiento aún no
están implementadas. Tampoco hay herencia a través de `Abarca`, propagación de
caudal, cambios de audio ni nuevas líneas en el visor de conexiones.

La verificación cubre la carga RON, la orientación e índices de las relaciones,
las lecturas navegables en ambas fichas, las declaraciones inválidas y que los
destinos de viaje continúen dependiendo exclusivamente de `Conectado`.

## 14. Comercio dirigido con mercancía

`Vende` y `Compra` son las dos lecturas de un intercambio habitual entre dos
Lugares. El autor puede declarar cualquiera de ellas desde el archivo de una
ciudad:

```ron
// En la ciudad vendedora:
(tipo: Vende, destino: Lugar("esh_kadur"), mercancia: "grano"),
// Alternativa equivalente, en la ciudad compradora (no declarar ambas):
(tipo: Compra, destino: Lugar("tazar"), mercancia: "grano"),
```

La carga convierte `Compra` en `Vende` intercambiando origen y destino.
El núcleo almacena una sola Relación cuyo origen es el vendedor y cuyo destino
es el comprador. Ambos extremos se indexan como en las demás relaciones dirigidas;
no se añade un vecino al grafo `Conectado`. El comercio no habilita rutas.

`mercancia` es texto obligatorio, no vacío ni compuesto solo de espacios, de una
línea y sin caracteres de control. Puede contener varias palabras, como
`"pescado seco"`. La carga recorta espacios en sus extremos y conserva mayúsculas,
acentos y espacios interiores; debe usarse la misma escritura para identificar
una misma mercancía. El núcleo comprueba que el texto ya esté normalizado.
No representa un objeto de inventario ni requiere un catálogo de productos.

Este campo es específico del comercio: los demás tipos lo rechazan. La clave
de duplicados incorpora mercancía, dirección y ambos extremos. Dos ciudades
pueden comerciar varias mercancías y hacerlo en ambos sentidos. Declarar la misma
venta y su compra inversa constituye un duplicado; las mercancías distintas no.
Los autoenlaces y los extremos de Actor se rechazan.

Las fichas muestran `Vende grano a: Esh Kadur` y `Compra grano a: Tazar`;
Enter abre la ficha del otro Lugar, también si está fuera del mapa. Las fichas y
los resúmenes de Lugar utilizan su descripción cuando está presente.

Esta incorporación registra el contexto comercial del mundo. No transfiere
existencias ni dinero, no calcula cantidades, precios, transporte o producción y
no ejecuta compras durante el viaje. Las relaciones no comerciales conservan
sus contratos sin mercancía ni otros atributos.
