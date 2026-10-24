# Modelo reglado de `Mundo`

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.13 |
| Fecha | 2026-10-18 |

## 1. Propósito y autoridad

Este documento es la fuente canónica para el tipo `Mundo`, sus elementos
reglados, referencias, operaciones e invariantes dentro de
`desert-tiger-core`.

La composición de `Partida` pertenece al
[contrato de sesión](./04-sesion-aplicacion.md); los archivos de entrada y la
conversión desde datos externos pertenecen al
[documento de contenido](./03-contenido.md).

El alcance incluye Mundo, Lugar, Actor, Grupo, Característica, fecha, variables
globales, la identidad opaca del mapa y la incorporación de Relaciones. El
modelo, los índices y las consultas específicos de estas últimas tienen su única
fuente normativa en [`10-relaciones.md`](./10-relaciones.md).

## 2. Ubicación del tipo y de la instancia

`desert-tiger-core` define el tipo `Mundo`, sus operaciones y sus invariantes.
`desert-tiger-app` posee, dentro de la Partida, una única instancia concreta de
ese tipo. No hay dos Mundos ni una copia almacenada dentro del núcleo.

```text
desert-tiger-core define: Mundo + operaciones

desert-tiger-app posee:   Partida
                         └── mundo: Mundo
```

La instancia es la única autoridad sobre el gameplay. Sus colecciones no se
exponen de forma mutable; cualquier modificación atraviesa una operación
semántica del núcleo.

Actores y Lugares se almacenan en secuencias privadas que preservan el orden
explícito recibido durante la construcción. Las consultas que los enumeran usan
ese orden y nunca el de una tabla hash. Como cada ID canónico corresponde a su
posición, resolver un Actor o Lugar no necesita un índice auxiliar. Esto no
impide los índices de adyacencia derivados que permiten consultar las Relaciones
de un elemento sin recorrer la colección completa.

## 3. Identificadores internos

Cada clase referenciable del alcance tendrá un identificador tipado. Como
mínimo existirán conceptos equivalentes a:

- `IdActor`;
- `IdLugar`;
- `IdMapa`;
- `IdRelacion`, conforme al
  [contrato de Relaciones](./10-relaciones.md#31-tipos-y-extremos).

Un identificador de una clase no podrá usarse como otro. Los identificadores:

- son únicos dentro de su clase y estables durante la Partida;
- no son nombres visibles ni coordenadas;
- todo `IdActor` e `IdLugar` almacenado dentro de un Mundo válido resuelve allí;
- `IdMapa` resuelve en la Partida, porque el mapa pertenece a la aplicación;
- permiten expresar referencias sin préstamos mutables entre elementos.

Cada tipo es un newtype opaco y copiable sobre `u32`, con campo interno privado.
Las listas canónicas del contenido asignan esos valores desde cero en su orden
declarado. La API exacta es:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdActor(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdLugar(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdMapa(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorIdentificador {
    PosicionFueraDeRango {
        posicion: usize,
    },
}

impl IdActor {
    pub fn desde_posicion(
        posicion: usize,
    ) -> Result<Self, ErrorIdentificador>;
}

impl IdLugar {
    pub fn desde_posicion(
        posicion: usize,
    ) -> Result<Self, ErrorIdentificador>;
}

impl IdMapa {
    pub fn desde_posicion(
        posicion: usize,
    ) -> Result<Self, ErrorIdentificador>;
}
```

`desde_posicion` acepta exactamente los valores representables en `u32` y no
trunca. `ErrorIdentificador` implementa `Display` y `Error`; su texto no es API
estable. La implementación usa `u32::try_from(posicion)` al construir y una
conversión comprobada a `usize`, seguida de `Vec::get`, al resolver; no usa
conversiones `as`. El entero privado puede exponerse solamente dentro del módulo
del modelo mediante un acceso privado o `pub(crate)` si la implementación lo
necesita, nunca como parte de la API pública.

El valor se utiliza internamente para direccionar una secuencia, pero no se
expone como entero ni autoriza acceso sin comprobarlo contra el Mundo vigente.
No se serializa como identidad estable del contenido y ningún llamador debe
suponer el mismo número después de reordenar archivos. La identidad estable para
autoría es la clave textual externa; `desert-tiger-app` conserva la tabla que la
resuelve al newtype tipado. Dos Partidas creadas desde el mismo
`ContenidoValidado` reciben la misma correspondencia.

El newtype identifica una posición dentro de la correspondencia de una Partida,
pero no codifica de qué Partida proviene. Mientras no exista persistencia y haya
una sola sesión activa, `desert-tiger-app` mantiene los IDs asociados a esa
sesión y no mezcla valores procedentes de otro `Mundo`.

## 4. Elementos reglados

### `Lugar`

Tiene identidad y un conjunto de Características sin duplicados. No almacena
tiles, coordenadas ni una lista propia de actores presentes.

El Lugar conceptual del diseño general sí incluye datos para representarlo. La
frontera técnica los divide: este tipo conserva solamente su estado reglado y el
[catálogo de presentación](./03-contenido.md) conserva, bajo el mismo `IdLugar`,
su nombre, coordenada central y lista ordenada de coordenadas del rótulo. Juntas
forman el elemento cargado, sin copiar datos visuales al núcleo.

Los actores presentes se derivan consultando la ubicación de cada Actor; de ese
modo no existe una segunda fuente de verdad.

### `Actor`

Todos los actores usan el mismo tipo. Cada uno tiene:

- identidad;
- exactamente un `IdLugar` como ubicación lógica;
- un conjunto de Características sin duplicados.

Ser jugador o NPC no crea subtipos. `Mundo` identifica exactamente un Actor como
jugador. El retrato y la política Lua se asocian externamente mediante `IdActor`
y no forman parte del estado reglado.

### `Relación`

Es un vínculo reglado almacenado una sola vez dentro del Mundo y consultable
desde índices derivados. En este incremento sus extremos pueden identificar
Actores o Lugares, aunque el único tipo admitido, `Conectado`, une exclusivamente
dos Lugares. Su representación, semántica, orden, consultas y validaciones se
definen en [`10-relaciones.md`](./10-relaciones.md#3-modelo-reglado) y no se
duplican dentro de `Lugar` o `Actor`.

### `Grupo`

Registra únicamente a los compañeros del actor jugador, sin duplicados. El actor
jugador no es integrante del Grupo. En el primer incremento, el Grupo contiene
al menos un compañero. Su orden canónico es exactamente el orden recibido en
`DatosInicialesMundo::companeros`; el constructor lo conserva y no lo ordena.

El Grupo no posee ubicación propia: todos sus integrantes deben referenciar el
mismo Lugar que el actor jugador. Trasladar al jugador con su Grupo cambia
atómicamente la ubicación del jugador y de todos los compañeros o no mueve a
ninguno.

### `Característica`

Es una etiqueta tipada asociada a un Actor o Lugar y consultada por presencia.
El núcleo permite comprobar su presencia sin exponer la colección mutable y
agregarla o retirarla únicamente mediante los lotes atómicos definidos en
[`11-operaciones-mundo.md`](./11-operaciones-mundo.md).

```rust
pub const MAX_BYTES_CARACTERISTICA: usize = 64;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Caracteristica(String);

impl Caracteristica {
    pub fn nueva(valor: String) -> Result<Self, ErrorCaracteristica>;
    pub fn como_str(&self) -> &str;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCaracteristica {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}
```

`ErrorCaracteristica` implementa `Display` y `Error`; su texto no es API
estable.

Su representación externa utiliza `lowerCamelCase`: comienza con una palabra en
minúscula y cada palabra posterior comienza con mayúscula, por ejemplo
`cargaTesoro`. No admite espacios, guiones ni guiones bajos y no se corrige o
normaliza silenciosamente. La gramática exacta es una palabra inicial `[a-z]+`
seguida de cero o más palabras `[A-Z][a-z]*`; no se admiten dígitos. Sus claves
utilizan exclusivamente letras ASCII y ocupan como máximo 64 bytes; por ejemplo,
se escribe `rapido` y no `rápido`. Los acentos y demás caracteres Unicode quedan
reservados para el texto visible al jugador.

Este modelo solo almacena y consulta las etiquetas; no define su significado.
El bloqueo común del actor jugador por `muerto` pertenece al
[motor de acciones](./05-motor-acciones.md). La viabilidad de un recorrido
pertenece a las reglas de [`Viajar`](../acciones/02-viajar-expediciones.md) y se
deduce de causas concretas, no de una etiqueta genérica del modelo.

### Fecha

`Mundo` contiene la única fecha vigente, cuyo tipo pertenece al
[calendario](./06-calendario-duracion.md).

La fecha se almacena como un único valor `Fecha`, no como campos independientes
del Mundo. Este expone exactamente la consulta `fecha(&self) -> Fecha` y la
operación atómica `avanzar_fecha(&mut self, Duracion) -> Result<Fecha,
ErrorCalendario>` aprobadas en el documento de calendario. Ninguna otra
operación del Mundo modifica el tiempo.

### Variables globales

Las variables globales forman un registro de datos extensible por contenido. Sus
claves no se enumeran en Rust ni originan un campo, variante o rama de código por
cada variable. Agregar una entrada válida al contenido permite consultarla en
una Partida nueva sin modificar ni recompilar código específico para esa clave.

El contrato de dominio es:

```rust
pub const MAX_BYTES_CLAVE_VARIABLE_GLOBAL: usize = 64;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaveVariableGlobal(String);

impl ClaveVariableGlobal {
    pub fn nueva(valor: String)
        -> Result<Self, ErrorClaveVariableGlobal>;

    pub fn como_str(&self) -> &str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoVariableGlobal {
    Booleana,
    Entera,
    Texto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValorVariableGlobal {
    Booleana(bool),
    Entera(i64),
    Texto(String),
}

impl ValorVariableGlobal {
    pub fn tipo(&self) -> TipoVariableGlobal;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariableGlobalInicial {
    pub clave: ClaveVariableGlobal,
    pub valor: ValorVariableGlobal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorClaveVariableGlobal {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConsultaVariableGlobal {
    Inexistente {
        clave: ClaveVariableGlobal,
    },
    TipoIncompatible {
        clave: ClaveVariableGlobal,
        esperado: TipoVariableGlobal,
        encontrado: TipoVariableGlobal,
    },
}
```

`ErrorClaveVariableGlobal` y `ErrorConsultaVariableGlobal` implementan
`std::fmt::Display` y `std::error::Error`; sus textos no forman una API estable.

Una clave usa `lowerCamelCase` ASCII, con la misma gramática de las
Características: `[a-z]+(?:[A-Z][a-z]*)*`. Distingue mayúsculas, no se normaliza
y ocupa como máximo 64 bytes. Los valores admitidos son exclusivamente booleano,
entero con signo de 64 bits y texto UTF-8; el texto vacío es válido.

Cada clave es única en todo el registro, incluso entre tipos. Por ejemplo, no
pueden coexistir una booleana y un texto llamados `puertaNorteAbierta`. La
separación en tres listas pertenece al formato de autoría; el dominio las reúne
como valores tipados bajo un solo espacio de claves. Su almacenamiento e índice
son privados y no exponen una colección mutable.

Las consultas requeridas son:

```rust
impl Mundo {
    pub fn variable_global(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<&ValorVariableGlobal, ErrorConsultaVariableGlobal>;

    pub fn variable_booleana(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<bool, ErrorConsultaVariableGlobal>;

    pub fn variable_entera(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<i64, ErrorConsultaVariableGlobal>;

    pub fn variable_texto(
        &self,
        clave: &ClaveVariableGlobal,
    ) -> Result<&str, ErrorConsultaVariableGlobal>;

    pub fn variables_globales(
        &self,
    ) -> impl ExactSizeIterator<
        Item = (&ClaveVariableGlobal, &ValorVariableGlobal),
    > + '_;
}
```

La enumeración usa orden lexicográfico de clave para que toda instantánea sea
determinista; ese orden no expresa prioridad de gameplay. Una consulta ausente
devuelve `Inexistente`, nunca `false`, cero ni texto vacío. Una consulta tipada
contra otro tipo devuelve `TipoIncompatible` con ambos tipos. Estas reglas hacen
visible un error de escritura en contenido o Lua.

El primer incremento construye y consulta el registro, pero no crea, modifica ni
elimina variables durante una Partida. Su representación genérica permite añadir
después operaciones semánticas validadas para eventos narrativos sin convertir
Lua en dueño mutable del Mundo.

## 5. Vínculo opaco con el mapa

`Mundo` conserva un único `IdMapa` para mantener la relación conceptual con el
mapa de la Partida. No puede resolverlo ni consultar tiles, dimensiones,
coordenadas o arte.

La carga valida la referencia externa y la
[sesión de aplicación](./04-sesion-aplicacion.md) comprueba que el identificador
corresponda a su EstadoMapa. Los datos visuales se definen en contenido y no se
copian al núcleo.

Ninguna regla puede usar un tile como fuente de ubicación, conectividad,
disponibilidad o estado. Toda condición duradera relevante para gameplay debe
estar representada en este modelo, aunque posteriormente tenga una proyección
visual.

## 6. Construcción, consultas y operaciones

El núcleo recibe valores ya deserializados y con referencias externas resueltas,
incluida una `Fecha` ya válida, pero vuelve a comprobar todas sus invariantes de
dominio. No recibe por separado año, mes y día. La construcción devuelve un
Mundo completo o un error; nunca publica uno parcial.

### Datos de construcción y almacenamiento

Los datos de entrada son valores consumibles distintos de los tipos vivos:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LugarInicial {
    pub id: IdLugar,
    pub caracteristicas: Vec<Caracteristica>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActorInicial {
    pub id: IdActor,
    pub lugar: IdLugar,
    pub caracteristicas: Vec<Caracteristica>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatosInicialesMundo {
    pub mapa: IdMapa,
    pub fecha: Fecha,
    pub lugares: Vec<LugarInicial>,
    pub actores: Vec<ActorInicial>,
    pub actor_jugador: IdActor,
    pub companeros: Vec<IdActor>,
    pub variables_globales: Vec<VariableGlobalInicial>,
    pub relaciones: Vec<RelacionInicial>,
}

pub struct Lugar {
    id: IdLugar,
    caracteristicas: BTreeSet<Caracteristica>,
}

pub struct Actor {
    id: IdActor,
    lugar: IdLugar,
    caracteristicas: BTreeSet<Caracteristica>,
}

pub struct Grupo {
    integrantes: Vec<IdActor>,
}

pub struct Mundo {
    mapa: IdMapa,
    fecha: Fecha,
    lugares: Vec<Lugar>,
    actores: Vec<Actor>,
    actor_jugador: IdActor,
    grupo: Grupo,
    variables_globales:
        BTreeMap<ClaveVariableGlobal, ValorVariableGlobal>,
    // Colección e índices privados definidos en 10-relaciones.md.
}
```

`BTreeMap` y `BTreeSet` son los tipos de `std::collections`. Los IDs de cada
`LugarInicial` y `ActorInicial` deben coincidir exactamente con
`desde_posicion` para su posición en la secuencia. Esto permite resolverlos por
índice en tiempo constante sin una segunda tabla ni una segunda fuente de orden.
Las colecciones vivas y todos sus campos permanecen privados.

```rust
impl Mundo {
    pub fn nuevo(
        datos: DatosInicialesMundo,
    ) -> Result<Self, ErrorConstruirMundo>;
}
```

`nuevo` consume los datos y valida todo antes de publicar el valor. No modifica
los argumentos por referencia, no produce un Mundo parcial y no consume azar.
Los conjuntos ordenados eliminan la dependencia del orden de una tabla hash,
pero un duplicado se detecta y rechaza antes de construirlos.

### Consultas de solo lectura

```rust
impl Lugar {
    pub fn id(&self) -> IdLugar;
    pub fn tiene_caracteristica(
        &self,
        caracteristica: &Caracteristica,
    ) -> bool;
    pub fn caracteristicas(
        &self,
    ) -> impl ExactSizeIterator<Item = &Caracteristica> + '_;
}

impl Actor {
    pub fn id(&self) -> IdActor;
    pub fn lugar(&self) -> IdLugar;
    pub fn tiene_caracteristica(
        &self,
        caracteristica: &Caracteristica,
    ) -> bool;
    pub fn caracteristicas(
        &self,
    ) -> impl ExactSizeIterator<Item = &Caracteristica> + '_;
}

impl Grupo {
    pub fn integrantes(&self) -> &[IdActor];
    pub fn contiene(&self, actor: IdActor) -> bool;
}

impl Mundo {
    pub fn id_mapa(&self) -> IdMapa;
    pub fn actor_jugador(&self) -> IdActor;
    pub fn grupo(&self) -> &Grupo;
    pub fn ubicacion_grupo(&self) -> IdLugar;

    pub fn actor(&self, id: IdActor) -> Option<&Actor>;
    pub fn lugar(&self, id: IdLugar) -> Option<&Lugar>;
    pub fn ubicacion_actor(&self, id: IdActor) -> Option<IdLugar>;

    pub fn actores(&self) -> &[Actor];
    pub fn lugares(&self) -> &[Lugar];

    pub fn actores_en_lugar(
        &self,
        lugar: IdLugar,
    ) -> Option<impl Iterator<Item = &Actor> + '_>;
}
```

Los slices e iteradores conservan el orden canónico y no permiten mutación.
`actores_en_lugar` devuelve `None` si el Lugar no existe; en caso contrario,
deriva el resultado recorriendo los Actores en orden canónico. No existe una
lista de presentes almacenada en el Lugar. `ubicacion_grupo` deriva siempre la
ubicación del jugador.

Las consultas de fecha y variables globales son las definidas en sus secciones
canónicas anteriores; no hay getters mutables para ninguna colección.

Las consultas `relaciones`, `relacion`, `relaciones_de` y
`lugares_conectados` forman parte de `Mundo`, pero su contrato y complejidad se
definen exclusivamente en
[`10-relaciones.md`](./10-relaciones.md#34-consultas-de-mundo).

La clausura que implemente el filtro de `actores_en_lugar` captura el `IdLugar`
copiable por valor (`move`), de manera que el iterador devuelto no tome prestada
una variable local.

### Operaciones semánticas

La mutación reglada del Mundo se realiza mediante lotes atómicos de
`OperacionMundo`. El vocabulario, la precedencia de errores y la estrategia de
confirmación tienen su fuente normativa única en
[`11-operaciones-mundo.md`](./11-operaciones-mundo.md). Ese contrato incluye el
traslado y la incorporación o eliminación idempotentes de Características de
Actor.

La fachada de compatibilidad para un traslado aislado permanece disponible:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrasladoConfirmado {
    pub actores: Vec<IdActor>,
    pub origen: IdLugar,
    pub destino: IdLugar,
}

impl Mundo {
    pub fn trasladar(
        &mut self,
        actor: IdActor,
        destino: IdLugar,
    ) -> Result<TrasladoConfirmado, ErrorTraslado>;
}
```

Si `actor` es el jugador, la operación mueve al jugador seguido de todos los
integrantes del Grupo en su orden canónico. Si es un Actor independiente, mueve
solo a ese Actor. Un integrante no puede trasladarse separadamente del jugador.
El resultado poseído describe el cambio ya realizado y puede convertirse uno a
uno en `ResultadoSemantico::ActoresTrasladados`; no es una orden para repetirlo.
`trasladar` construye internamente un lote de una sola operación, delega en
`Mundo::aplicar_lote` y proyecta su cambio o error al contrato histórico de
`TrasladoConfirmado` y `ErrorTraslado`. No existe una segunda ruta de mutación.

`trasladar` no interpreta `muerto` ni ninguna otra Característica: esas son
reglas del motor y de las acciones. La operación protege
únicamente existencia, ausencia de un traslado vacío y las invariantes de
ubicación y Grupo. El avance de fecha permanece en la operación separada
aprobada por el calendario.

## 7. Invariantes

Un Mundo válido cumple siempre que:

- existe exactamente un actor jugador y no pertenece al Grupo;
- todo Actor referencia exactamente un Lugar existente;
- el Grupo contiene al menos un compañero;
- todos los integrantes del Grupo están en el mismo Lugar que el actor jugador;
- un Actor aparece como máximo una vez en el Grupo;
- todos los identificadores son únicos dentro de su clase;
- existe exactamente un `IdMapa` opaco;
- toda variable global tiene una clave válida, un único valor y un único tipo;
- ninguna clave de variable global aparece más de una vez;
- toda Relación e índice satisface las invariantes de
  [`10-relaciones.md`](./10-relaciones.md#7-contratos-e-invariantes);
- tiles y coordenadas no intervienen en consultas ni mutaciones regladas;
- una operación aceptada conserva todas las invariantes;
- una operación rechazada no altera el estado observable.

La única ubicación persistida es la de cada Actor. Lugar y Grupo derivan sus
consultas de ella.

## 8. Errores y validaciones

Los errores propios de construcción y traslado son:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorConstruirMundo {
    SinLugares,
    SinActores,
    PosicionLugarFueraDeRango {
        posicion: usize,
    },
    PosicionActorFueraDeRango {
        posicion: usize,
    },
    IdLugarNoCanonico {
        posicion: usize,
        esperado: IdLugar,
        encontrado: IdLugar,
    },
    IdActorNoCanonico {
        posicion: usize,
        esperado: IdActor,
        encontrado: IdActor,
    },
    CaracteristicaLugarDuplicada {
        lugar: IdLugar,
        caracteristica: Caracteristica,
    },
    CaracteristicaActorDuplicada {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    LugarDeActorDesconocido {
        actor: IdActor,
        lugar: IdLugar,
    },
    ActorJugadorDesconocido {
        actor: IdActor,
    },
    GrupoVacio,
    JugadorIncluidoEnGrupo {
        actor: IdActor,
    },
    IntegranteGrupoDuplicado {
        actor: IdActor,
    },
    IntegranteGrupoDesconocido {
        actor: IdActor,
    },
    IntegranteGrupoSeparado {
        actor: IdActor,
        lugar_actor: IdLugar,
        lugar_jugador: IdLugar,
    },
    VariableGlobalDuplicada {
        clave: ClaveVariableGlobal,
    },
    // Variantes de construcción de Relaciones definidas en 10-relaciones.md.
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorTraslado {
    ActorDesconocido {
        actor: IdActor,
    },
    DestinoDesconocido {
        destino: IdLugar,
    },
    DestinoEsLugarActual {
        lugar: IdLugar,
    },
    IntegranteGrupoNoPuedeTrasladarseSolo {
        actor: IdActor,
    },
    EstadoInconsistente,
}
```

Ambos implementan `Display` y `Error`; sus textos no forman una API estable. Los
tipos `Fecha`, `Caracteristica`, `ClaveVariableGlobal` e `IdMapa` vuelven
irrepresentables una fecha inválida, una etiqueta inválida o la ausencia del
campo de mapa dentro de `DatosInicialesMundo`. Tener un único campo
`actor_jugador` vuelve irrepresentable declarar dos jugadores; el constructor
solo debe comprobar que ese ID exista.

`nuevo` devuelve el primer error conforme a este orden determinista:

1. cardinalidad mínima de Lugares y Actores;
2. ID canónico y Características repetidas de cada Lugar, en orden;
3. ID canónico, Lugar y Características repetidas de cada Actor, en orden;
4. existencia del jugador;
5. Grupo no vacío y luego cada integrante en el orden recibido: jugador
   incluido, duplicado, desconocido o separado;
6. claves de variables globales repetidas en el orden recibido;
7. Relaciones e índices, en el orden y con la precedencia definidos en
   [`10-relaciones.md`](./10-relaciones.md#5-carga-normalización-y-validaciones).

`trasladar` conserva la precedencia observable de su contrato histórico:
existencia del Actor; existencia del destino; existencia del Lugar de origen
almacenado por el Actor; destino distinto del origen; prohibición de mover a un
integrante solo; y consistencia interna del Grupo cuando se mueve al jugador.
Un origen inexistente o una ruptura del Grupo produce `EstadoInconsistente`. La
validación y atomicidad reales se delegan al lote de una operación descrito en
[`11-operaciones-mundo.md`](./11-operaciones-mundo.md); un error no produce
ninguna escritura.

El motor usa consultas de solo lectura para convertir condiciones previsibles en
los rechazos de `Viajar`. Cuando la acción ya fue aceptada y alcanzó su punto de
traslado, cualquier `ErrorTraslado` indica que cambió un supuesto ya validado o
se rompió una invariante y se convierte en `FalloMotor::InvarianteRota`; nunca se
degrada a un rechazo recuperable después de emitir narrativa o consumir azar.

La capa de contenido conserva la responsabilidad por sintaxis, archivos y
diagnósticos de origen; el núcleo conserva siempre la validación de sus
invariantes aunque el llamador ya haya validado los DTO.

Por privacidad de Rust, toda implementación que acceda a campos privados de
`Mundo` reside en el mismo módulo que el tipo o en uno descendiente. En
particular, el comportamiento de `avanzar_fecha` sigue siendo canónico en el
documento de calendario, pero su bloque `impl Mundo` no se ubica en un módulo
hermano que carezca de acceso a `fecha`.

## 9. Pruebas y aceptación

Las pruebas unitarias del núcleo cubrirán:

- construcción de un Mundo mínimo válido;
- rechazo individual y precedencia de cada variante de `ErrorConstruirMundo`;
- resolución de IDs tipados, límites de `u32` y rechazo sin truncamiento;
- rechazo de un ID no canónico para su posición;
- conservación del orden canónico de los slices y de `actores_en_lugar`;
- consulta de actores por Lugar sin una lista duplicada en el Lugar;
- construcción y consulta de Características sin exponer su colección mutable;
- rechazo de Características inválidas, mayores que 64 bytes o repetidas;
- construcción y consulta genérica de variables booleanas, enteras y de texto;
- consulta tipada correcta de cada clase de variable global;
- aceptación de una clave inventada por contenido sin conocimiento semántico en
  Rust;
- rechazo de claves de variable con sintaxis inválida, Unicode, dígitos,
  separadores o más de 64 bytes;
- rechazo de una clave repetida dentro de un tipo o entre tipos diferentes;
- consulta inexistente sin valor predeterminado y consulta de tipo incompatible
  con los tipos esperado y encontrado;
- enumeración determinista de variables sin acceso mutable;
- traslado de un Actor ajeno al Grupo con un resultado de un solo ID;
- traslado atómico del jugador y todos los integrantes del Grupo, con jugador
  primero y compañeros en su orden canónico;
- origen, destino y actores exactos en `TrasladoConfirmado` después del cambio;
- rechazo de Actor desconocido, destino desconocido, destino actual e intento de
  traslado independiente de un compañero;
- `EstadoInconsistente` ante una ruptura interna detectada en una prueba del
  módulo;
- ausencia de cualquier cambio después de cada traslado rechazado;
- aplicación atómica de lotes mixtos de traslado y Características, incluso si
  falla su última operación;
- idempotencia de añadir una Característica ya presente y quitar una ausente;
- inexistencia de un chequeo de reglas de acción como `muerto` dentro de `trasladar`;
- imposibilidad estructural de consultar mapa, Lua o frontend.

Las pruebas de colección, índices, extremos, consultas y errores de Relaciones
pertenecen a
[`10-relaciones.md`](./10-relaciones.md#81-núcleo); este documento exige su
integración con el mismo constructor atómico de Mundo.

Una prueba de integración del núcleo ejecutará `Viajar`, convertirá los cambios
confirmados del lote en resultados semánticos y comprobará que el Mundo ya
contiene las ubicaciones y Características resultantes.

## 10. Preguntas bloqueantes

Ninguna para el alcance inicial.

## 11. Decisiones aplazadas

No bloquean el primer incremento:

- tipos de Relación distintos de `Conectado` y mutación de Relaciones durante
  una Partida;
- creación o eliminación dinámica de actores y lugares;
- facciones y pertenencia;
- proyección automática de cambios semánticos sobre el mapa;
- estadísticas distintas de Características;
- operaciones generales para agregar y retirar Características, hasta que una
  acción aprobada las necesite;
- creación, modificación o eliminación de variables globales durante una
  Partida y los efectos de eventos narrativos que solicitarán esas operaciones.
