# Parentesco y Actores fuera del escenario

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.1 |
| Fecha | 2026-10-26 |

## 1. Alcance y fuente de verdad

Amplía el modelo de Actor y Relación del
[diseño general](../../diseno-general.md) y las fichas de
[Acampar](../acciones/07-acampar.md). Documenta el incremento implementado: Actores de contexto sin ubicación jugable, Padre/Madre,
MatadoPor y hermanos deducidos al consultar.

Las relaciones familiares representan filiación biológica; en este alcance se
asume también el vínculo social. No se modelan adopción, matrimonio, afecto,
obligaciones, reinos, legitimidad, sucesión ni eventos de nacimiento. «Rey» y
«príncipe» son nombres y contexto narrativo, sin reglas de títulos. El exilio
del príncipe aparece solo en su biografía, sin característica nueva.

No se agregan primos, tíos u otros parentescos calculados, un lenguaje genérico
de consultas, secretos, guardado ni recarga de archivos. Una familia puede estar
incompleta en los datos: no se exige declarar ascendientes indefinidamente.

Este contrato es la fuente de las ampliaciones indicadas en la sección 11. Los
contratos existentes conservan los demás comportamientos. La implementación
mantiene `terminal → app → core`, un proceso y las dependencias actuales.

## 2. Responsabilidades y archivos

| Archivo del workspace | Cambio |
|---|---|
| `desert-tiger-core/src/modelo.rs` | Ubicación, invariantes, tipos de relación, índices y consulta de hermanos. |
| `desert-tiger-core/src/lib.rs` | Reexportar ubicación y resultados de parentesco. |
| `desert-tiger-core/src/acciones.rs`, `acciones/acampar.rs` | Rechazar la actuación local de un Actor de contexto. |
| `desert-tiger-core/src/acciones/viajar/` | Adaptar lecturas de ubicación sin alterar reglas del viaje. |
| `desert-tiger-app/src/contenido.rs` | Nuevo esquema de ubicación, relaciones y diagnósticos de autoría. |
| `desert-tiger-app/src/lua.rs`, `fase_npc.rs` | Omitir Actores fuera del escenario antes de ejecutar su política. |
| `desert-tiger-app/src/fichas.rs`, `lib.rs` | Ubicación visible, vínculos registrados y deducciones diferenciados. |
| `desert-tiger-terminal/src/campamento.rs` | Mostrar ubicación y explicación de una deducción; conservar controles. |
| `content/base/actors/`, `world.ron` | Migración y contenido de integración de la sección 9. |

Los prefijos de archivos Rust son relativos a `crates/`. No se crea otro modelo
de personaje: local o externo sigue siendo el mismo Actor, con características,
relaciones, biografía, retrato opcional y política.

## 3. Ubicación del Actor

### 3.1. Representación y consultas

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UbicacionActor {
    EnLugar(IdLugar),
    FueraDelEscenario,
}
// ActorInicial.lugar y Actor.lugar se reemplazan por ubicacion.
impl Actor {
    pub fn ubicacion(&self) -> UbicacionActor;
    pub fn lugar(&self) -> Option<IdLugar>;
}
```

`FueraDelEscenario` significa que el Actor está registrado, pero no ocupa ningún
Lugar del escenario jugable. Puede estar vivo o muerto y puede ser el extremo
de una relación. No implica crear un continente o Lugar ficticio ni significa
que el Actor no exista.

`Actor::lugar()` devuelve `Some(id)` para EnLugar y `None` para FueraDelEscenario.
`Mundo::ubicacion_actor(id) -> Option<IdLugar>` conserva su firma usando
`and_then`: devuelve None para Actor inexistente o externo; `Mundo::actor(id)`
permite distinguir ambos casos. `actores_en_lugar` excluye a los externos.
`ubicacion_grupo() -> IdLugar` conserva su firma e invariante.

### 3.2. Invariantes y operaciones

- Jugador y compañeros iniciales deben estar EnLugar, en el mismo Lugar válido.
  La exigencia inicial de Grupo no vacío se conserva.
- Un Actor externo no puede incorporarse al Grupo mediante las operaciones
  actuales. Su ficha no requiere retrato y nunca lo añade al selector horizontal.
- Viajar y Acampar rechazan al Actor externo antes de ejecutar acciones o
  consumir azar; la muerte y los demás rechazos actuales conservan su precedencia.
- Un traslado ordinario desde FueraDelEscenario se rechaza: no introduce al
  Actor implícitamente en el escenario. No se incorpora una operación de entrada
  o salida del escenario en este incremento.
- Trasladar un Actor local y separar un compañero conservan su conducta, pero
  escriben EnLugar(destino). Los lotes continúan validándose antes de confirmar.
- Cambiar características y registrar Conoce siguen admitiendo cualquier Actor
  existente: no requieren ubicación jugable ni convierten al Actor en local.

Los accesores no sustituyen una ubicación ausente por el Lugar del jugador ni
por el primer Lugar. La lectura del núcleo que exige ubicación local resuelve
la variante explícitamente; no se distribuyen `unwrap` nuevos por los consumidores.

### 3.3. NPC y Lua

La fase NPC salta a los Actores FueraDelEscenario antes de evaluar Lua, incluidos
los vivos. La carga puede validar un bloque Lua presente en ellos, pero no lo
ejecuta. Los Actores nuevos del ejemplo tienen `politica: Ninguna`.

Una llamada directa a `evaluar_politica` para un Actor externo existente devuelve
`Ok(ResultadoPoliticaLua::Ninguna)` sin crear una VM ni ejecutar su código. Los
errores por Actor inexistente mantienen su comportamiento. Para Actores locales
la API Lua conserva `actor.lugar` como cadena de clave de Lugar; no pasa a nil.
No se amplían solicitudes Lua ni reglas de Viajar, Acampar o fase temporal.

## 4. Autoría RON y compatibilidad

### 4.1. Ubicación inicial

El campo obligatorio `lugar_inicial: String` se reemplaza por
`ubicacion_inicial: UbicacionInicialDto`. Su esquema es:

```ron
ubicacion_inicial: EnLugar("zona_arida"),
// Alternativa para un Actor de contexto:
ubicacion_inicial: FueraDelEscenario,
```

El DTO usa `EnLugar(String)` y `FueraDelEscenario`. La carga resuelve la clave
de EnLugar y produce `UbicacionActor`; una clave desconocida es error. Ambos
campos simultáneos, el antiguo `lugar_inicial`, None y una cadena sin variante
son inválidos. Se conserva `deny_unknown_fields`, sin extensiones RON.

Se migran juntos cargador, seis RON actuales y fixtures. Los Actores existentes
mantienen exactamente el mismo Lugar, arte, características, políticas y
relaciones. Mods con el esquema anterior deben renombrar el campo y envolver su
valor en EnLugar. No hay migración de partidas guardadas porque no hay guardado.

### 4.2. Relaciones nuevas

Se agregan `Padre`, `Madre` y `MatadoPor` a `TipoRelacionDto` y
`TipoRelacion`. Las tres se declaran en el archivo del Actor de origen:

```ron
relaciones: [
    (tipo: Padre, destino: Actor("rey")),
    (tipo: Madre, destino: Actor("madre_del_principe")),
],
```

Padre/Madre apuntan **del hijo al progenitor**, no al revés. MatadoPor apunta
**de la víctima al autor**. Sus extremos deben ser Actores existentes, locales
o externos. No se escribe una segunda declaración inversa; las fichas usan la
misma relación desde cualquiera de los extremos.

Ejemplo completo de Actor externo (los demás Actores referidos se declaran en
sus propios archivos y en la lista canónica de `world.ron`):

```ron
(
    clave: "rey",
    nombre: "Rey",
    biografia: "Padre del príncipe, procedente de otro continente.",
    ubicacion_inicial: FueraDelEscenario,
    caracteristicas: ["muerto"],
    relaciones: [
        (tipo: Padre, destino: Actor("abuelo_paterno")),
        (tipo: Madre, destino: Actor("abuela_paterna")),
        (tipo: MatadoPor, destino: Actor("hermano_del_rey")),
    ],
    retrato: None,
    politica: Ninguna,
)
```

Las referencias siempre resuelven a un RON cargado; FueraDelEscenario no permite
referencias inexistentes. Un progenitor no documentado se omite, sin crear un
Actor «desconocido» compartido que haría aparecer falsos hermanos.

### 4.3. Validación de parentesco

Se comprueba tanto en carga como al construir Mundo:

- como máximo un Padre y una Madre por hijo;
- si ambos están declarados, apuntan a personas distintas;
- no hay relaciones reflexivas ni duplicados de tipo/origen/destino;
- el subgrafo combinado Padre/Madre no tiene ciclos de ascendencia;
- Conectado y Conoce conservan sus extremos, dirección y normalización actuales.

No se exige completar ambos progenitores. No se deducen sexo, edad, fechas,
matrimonio o derechos a partir de ellos. La comprobación de ciclos se limita a
ascendencia; no prohíbe los ciclos de las conexiones geográficas ni de otras
relaciones.

MatadoPor es un hecho histórico explícito. No ejecuta combate ni añade
automáticamente `muerto`; el ejemplo declara ambos datos. No se limita a un
único autor, para no impedir varios participantes en el mismo hecho. No se
añaden reglas de resurrección ni acciones de asesinato en este incremento.

Ejemplos inválidos: dos Padres diferentes para un Actor; Padre y Madre apuntando
al mismo Actor; Padre apuntando a un Lugar; A tiene Padre B y B tiene Madre A;
MatadoPor reflexiva o con referencia inexistente. Compartir progenitores entre
personas distintas sí es válido y es la base de la consulta de hermanos.

## 5. Almacenamiento, índices y consultas familiares

### 5.1. Relaciones registradas

Las aristas nuevas mantienen el objeto Relacion existente, con su IdRelacion
real, origen, tipo y destino. Se incorporan a los índices incidentes de ambos
Actores. No se añaden a `lugares_conocidos_por_actor` ni a los vecinos de viaje;
esos índices conservan exclusivamente sus tipos actuales.

La normalización trata los tres tipos nuevos como dirigidos: no ordena sus
extremos. Solo Conectado conserva la normalización simétrica. `world.ron` sigue
enumerando elementos, sin centralizar sus relaciones.

### 5.2. Índices familiares y API core

```rust
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Progenitores {
    pub padre: Option<IdActor>,
    pub madre: Option<IdActor>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HermanoDerivado {
    pub actor: IdActor,
    pub progenitores_compartidos: Vec<IdActor>,
}
impl Mundo {
    pub fn progenitores(&self, actor: IdActor) -> Option<Progenitores>;
    pub fn hijos_de(&self, actor: IdActor) -> Option<&[IdActor]>;
    pub fn hermanos_de(&self, actor: IdActor) -> Option<Vec<HermanoDerivado>>;
}
```

Actor inexistente produce None. Actor existente sin datos devuelve progenitores
vacíos, hijos vacíos o `Some(vec![])`, según la consulta. No se inventan
parentescos a partir de información ausente.

Mundo mantiene `progenitores_por_actor: Vec<Progenitores>` y
`hijos_por_actor: Vec<Vec<IdActor>>`, construidos en orden canónico desde
Padre/Madre y con hijos ordenados por ID. Son índices derivados, no otra fuente
editable de parentesco. Ocupan espacio lineal en Actores y aristas parentales.

Para validar ascendencia se usa DFS iterativo con colores no visitado/en curso/
terminado, recorriendo hasta dos progenitores por Actor. Una arista hacia un
Actor en curso detecta el ciclo y permite devolver su recorrido de IDs. No se
usa recursión dependiente de la profundidad de la genealogía ni un límite
arbitrario de generaciones. Construcción y validación son O(V + E parental).

### 5.3. Hermandad deducida

Para consultar hermanos de X:

1. Obtener sus progenitores registrados.
2. Recorrer los hijos de cada uno de esos progenitores por el índice inverso.
3. Excluir X y agrupar por IdActor, conservando los progenitores compartidos.
4. Devolver un resultado por hermano, por ID ascendente, con los progenitores
   compartidos también ordenados y sin duplicados.

Compartir uno o ambos progenitores registrados basta para mostrar hermandad.
Compartir uno no demuestra que el otro sea diferente: no se etiqueta «medio
hermano» basándose en datos ausentes. Los dos progenitores compartidos producen
una sola entrada, no dos. No se deduce que los progenitores sean pareja.

La consulta no recorre todos los Actores o todas las relaciones del Mundo:
trabaja sobre las familias de hasta dos progenitores y ordena sus candidatos.
No se materializan todas las parejas de hermanos. Es pura: no modifica Mundo,
relaciones, fecha, RNG ni conocimiento del jugador.

Solo las relaciones de ascendencia y MatadoPor de contenido se incorporan en
este incremento; no se añaden operaciones de alta/baja de esas aristas en una
partida. RegistrarConocimiento continúa actualizando sus índices existentes y
conserva intactos los familiares. Una futura mutación parental deberá diseñar
su propia validación y actualización atómica.

## 6. Fichas: ubicación, inversas y deducciones

### 6.1. Contrato público

`app` reexporta `UbicacionActor` y los tipos core de parentesco necesarios.
`VistaFicha` añade `ubicacion: Option<VistaUbicacionActor>`: Some para Actor y
None para Lugar. La proyección neutral es:

```rust
pub enum VistaUbicacionActor {
    EnLugar { lugar: IdLugar, nombre: String },
    FueraDelEscenario,
}
pub enum VinculoFicha {
    Registrado {
        id: IdRelacion,
        tipo: TipoRelacion,
        sentido: SentidoRelacionFicha,
    },
    Hermandad { progenitores_compartidos: Vec<IdActor> },
}
pub struct VistaRelacionFicha {
    pub vinculo: VinculoFicha,
    pub etiqueta: String,
    pub referencia: ResumenElemento,
    pub explicacion: Option<String>,
}
```

Estos valores derivan Clone, Debug, Eq y PartialEq. `vinculo` sustituye los
campos sueltos id/tipo/sentido actuales: no hay IdRelacion ficticios, cero como
reserva, variante Hermano en TipoRelacion ni aristas derivadas escritas en RON.
Los vínculos registrados transportan sus mismos datos dentro de Registrado.

### 6.2. Proyección y lectura

| Vínculo | Desde su origen | Desde su destino |
|---|---|---|
| Padre | `Padre: {nombre}` | `Padre de: {nombre del hijo}` |
| Madre | `Madre: {nombre}` | `Madre de: {nombre del hijo}` |
| MatadoPor | `Matado por: {nombre}` | `Mató a: {nombre de la víctima}` |

Son Saliente/Entrante respectivamente. Las etiquetas de Conoce y Conectado
permanecen iguales. En una ficha de Actor se emiten características, relaciones
registradas por IdRelacion y al final hermanos derivados por IdActor. Una
deducción lleva etiqueta `Hermano/a de: {nombre}` y explicación
`Progenitor en común: {nombre}` o `Progenitores en común: {nombres}`.
No se presupone el género del otro Actor ni se agrega un atributo de sexo.

La explicación procede de los IDs devueltos por el núcleo y sus nombres de
presentación. Compartir dos progenitores no duplica entradas. Tampoco se
fusionan vínculos distintos por apuntar al mismo Actor: en el rey deben aparecer
tanto MatadoPor como Hermandad hacia su hermano.

El detalle de campamento incluye la explicación, cuando exista, y la ubicación
de la ficha actual: nombre del Lugar o «Fuera de la región». Es información
paginable de contexto, no una tercera clase de foco o enlace. El resumen de una
referencia a Actor externo incluye esa indicación junto a su biografía breve;
el texto completo sigue disponible al abrir su ficha.

Enter sigue usando `referencia.elemento`, sin depender de un IdRelacion. Las
entradas registradas y las deducciones se recorren igual, con los controles,
páginas, ancla horizontal, retratos y fogata de Acampar. Abrir al rey no añade su
retrato al grupo ni cambia su ubicación. Los recorridos rey↔hermano pueden
repetirse sin memoria acumulativa ni mutación de las aristas.

## 7. Errores y confirmación de estado

| Frontera | Nuevos errores o condiciones |
|---|---|
| Construcción de Mundo | `JugadorFueraDelEscenario { actor }`, `IntegranteGrupoFueraDelEscenario { actor }`. |
| Construcción familiar | `ProgenitorMultiplicado { actor, tipo }`, `ProgenitoresNoDistintos { actor, progenitor }`, `CicloAscendencia { actores }`. |
| Disponibilidad/inicio de Viajar | `RechazoAccion::ActorFueraDelEscenario`. |
| Inicio de Acampar | `ErrorAcampada::ActorFueraDelEscenario`. |
| Traslado/lote | `ErrorOperacionMundo::ActorFueraDelEscenario { actor }` y equivalente en ErrorTraslado; no confirmar ninguna operación del lote fallido. |
| Carga | Referencias, ubicación, esquema viejo, duplicados, cardinalidad o ciclos inválidos: ErrorContenido contextualizado. |

Las nuevas variantes participan en los Display y conversiones exhaustivas
existentes. Se conservan los errores actuales de Actor/Lugar inexistente,
extremos incompatibles, reflexividad y duplicados. No se usa un error Fatal
para una solicitud externa rechazada de manera esperable.

El núcleo valida sus invariantes también para consumidores que no usan RON.
La aplicación traduce los errores familiares a archivo `actors/{clave}.ron`,
campo/tipo y Actor afectado mediante el catálogo de claves. En un ciclo incluye
el recorrido de claves y sitúa el diagnóstico en el primer Actor del recorrido.
No se publica un Mundo ni una ficha parcial después de un error de validación.

## 8. Compatibilidad de acciones y presentación

La adaptación de ubicación incluye todas las lecturas de Actor.lugar y los
constructores de pruebas. No se limita al cargador: cubre traslados y separación,
consulta de destinos, inicio de Viajar, Lua, fase NPC y proyecciones de sesión.
La fecha y los resultados mecánicos de los Actores locales deben conservarse.

Los matches sobre TipoRelacion y VistaRelacionFicha se actualizan explícitamente;
una reserva silenciosa no sustituye la semántica de los tipos nuevos. El cambio
de forma de la ficha se adapta junto a sus pruebas y al detalle de terminal.
Los IdRelacion guardados y los índices de Conoce siguen válidos cuando una
operación añade conocimiento durante Viajar.

## 9. Contenido de integración

Se conservan los seis Actores actuales y su orden en `world.ron`; se añaden al
final cinco Actores de contexto, con nombres funcionales y biografías mínimas:

| Clave | Nombre | Hechos que debe expresar el contenido |
|---|---|---|
| `rey` | Rey | Padre del príncipe; muerto; MatadoPor hacia hermano_del_rey. |
| `madre_del_principe` | Madre del príncipe | Madre del príncipe. |
| `hermano_del_rey` | Hermano del rey | Comparte Padre y Madre con rey. |
| `abuelo_paterno` | Abuelo paterno | Padre de rey y hermano_del_rey, mediante sus referencias entrantes. |
| `abuela_paterna` | Abuela paterna | Madre de ambos, mediante sus referencias entrantes. |

Los cinco usan FueraDelEscenario, retrato None y política Ninguna. Solo el rey
requiere `muerto`; no se inventan edades, nombres propios, continentes jugables,
coronaciones o derechos sucesorios. La filiación no documentada de los abuelos y
de la madre se omite.

En el RON del príncipe se declaran Padre→rey y Madre→madre_del_principe; su
biografía indica que vive exiliado. En los RON de rey y hermano_del_rey se
declaran Padre→abuelo_paterno y Madre→abuela_paterna. No se escribe Hermano.
El Grupo, su posición y sus retratos iniciales no cambian. Las matrices y el
arte existentes se conservan; no hay recursos de retrato nuevos obligatorios.

## 10. Verificación y aceptación

| Caso | Evidencia requerida |
|---|---|
| Ubicación | Actor externo vivo y muerto válidos; jugador o compañero externo rechazados; EnLugar desconocido rechazado. |
| Actuación local | Viajar, Acampar y traslado rechazan a un externo vivo sin RNG, tiempo ni cambios parciales; traslado local conserva su comportamiento. |
| NPC/Lua | Externo vivo con código que fallaría si se ejecuta no actúa; la llamada directa devuelve Ninguna; NPC local conserva actor.lugar y su fase habitual. |
| Esquema | Migración completa de RON/fixtures; rechazo del campo antiguo, valores ambiguos y referencias Actor inexistentes o de otro tipo. |
| Ascendencia | Cero, uno y dos progenitores; cardinalidad, igualdad, autorreferencia y ciclos mezclando Padre/Madre; genealogía profunda sin recursión. |
| Hermandad | Uno/dos progenitores compartidos, ambos desconocidos, autoexclusión, orden estable y ausencia de duplicados. |
| Pureza | Consultar hermanos o fichas no crea aristas ni modifica RNG, fecha, conocimiento o estado. |
| Índices | Relaciones familiares incidentes en ambos extremos; RegistrarConocimiento no altera los familiares ni su consulta. |
| Ficha | Etiquetas e inversas correctas; Rey con MatadoPor y Hermandad separados hacia el mismo Actor, con explicación verificable. |
| Navegación | Príncipe→Rey→hermano, vuelta por deducción, consulta de progenitores, ficha externa sin retrato y retorno horizontal al Grupo. |
| Regresión | Campamento sigue cerrando un día y fase NPC una vez; Viajar local mantiene sus resultados y calendario; todos los recursos oficiales cargan. |

El humo desde archivos reales comprueba los once Actores, el exilio solo en
biografía y la familia de la sección 9. TestBackend recorre las referencias y
el detalle paginado sin modificar los retratos del grupo. La revisión manual
final confirma que parentesco, autor del asesinato y ubicación externa se
comprenden desde las fichas. Se ejecutan los controles de la
[metodología](../../metodologia-desarrollo.md#7-controles-automáticos), carga de
contenido y Lua oficiales, y build de producción.

## 11. Contratos afectados y límites

| Documento | Ampliación vigente para este desarrollo |
|---|---|
| [02-modelo-mundo](./02-modelo-mundo.md) | Ubicación y construcción: sección 3; relaciones e índices: sección 5. |
| [03-contenido](./03-contenido.md) | Esquema, validación y migración: sección 4. |
| [10-relaciones](./10-relaciones.md) | Tres tipos Actor→Actor, su normalización e índices: secciones 4–5. |
| [11-operaciones-mundo](./11-operaciones-mundo.md) | Ubicación local y rechazo de traslado externo: secciones 3 y 7. |
| [08-integracion-lua](./08-integracion-lua.md), [07-fase-npc](./07-fase-npc.md) | Exclusión de Actores externos: sección 3.3. |
| [Acampar](../acciones/07-acampar.md) | Forma de vínculos de ficha y ubicación visible: sección 6; navegación y coste sin cambios. |

No quedan preguntas bloqueantes para este alcance. El resultado es consultable
y sirve de base a futuras acciones que interpreten parentescos; no incorpora
consecuencias automáticas por ser familiar, rey, asesino o exiliado.


## 12. Verificación de la implementación — 2026-10-26

Pasaron `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace --all-targets` y `cargo build --workspace --release`.
Son 107 pruebas: 46 core, 31 app, 26 terminal y 4 importador. La base anterior
pasaba 96; se añadieron pruebas de ubicación externa, familias y su integración.

La validación incluye una ascendencia de 20.000 Actores sin recursión, rechazo
por ciclos/cardinalidad, omisión de Lua para un externo vivo, conservación de
índices al registrar Conoce, migración y diagnósticos de RON, consultas puras y
las dos referencias diferentes al mismo hermano. TestBackend recorre las fichas
reales y permite leer la explicación paginada en 80×24.

Se verificó el ejecutable release en PTY 80×24: Acampar → Padre del príncipe →
Rey; consulta de MatadoPor y Hermandad, con los progenitores compartidos; salto
al hermano y vuelta al rey; retorno horizontal al Sirviente; consulta de la
madre desde el príncipe; abandono del campamento y apertura del selector de
Viajar. Se conservaron retratos y fogata. La salida restauró la terminal y
terminó con código cero. Los seis retratos, sus expresiones, paletas y políticas
anteriores se compararon con Git y permanecen idénticos.

Los once Actores oficiales cargan: los cinco nuevos son de contexto y no se
incorporan al Grupo. El exilio está únicamente en la biografía; no se implementó
un sistema de reinos o sucesión. No se registraron desviaciones funcionales.
