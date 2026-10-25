# Contenido externo, carga e instanciación

| Campo | Valor |
|---|---|
| Estado | Implementado; ampliación causal aprobada para implementar |
| Versión | 0.23 |
| Fecha | 2026-10-19 |

## Revisión implementada el 2026-10-24: Validación de contenido para Viajar

La carga admite cero o más políticas de NPC independientes. Conserva los formatos RON y Lua; valida tamaños incompatibles, existencia de costa y centros de conexiones navegables. Las claves nuevas están en la sección 12 del contrato 06.

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

## 1. Propósito y alcance

Este documento diseña la frontera entre los archivos editables del juego y una
Partida nueva. Desarrolla el modelo descrito en el
[diseño general](../../diseno-general.md) y respeta la separación establecida por
el [diseño técnico general](../diseno-tecnico-general.md).

Su alcance comprende:

- leer el contenido necesario para un Mundo inicial;
- representar los datos externos sin acoplarlos a los tipos del núcleo;
- validar identificadores, referencias y datos de presentación;
- resolver y validar las Relaciones declaradas junto a sus elementos;
- producir catálogos validados para reglas, presentación y políticas de NPC;
- crear estados mutables nuevos, iguales al contenido inicial e independientes
  entre sí.

Quedan fuera el diseño creativo del setting, las reglas de las acciones, el
formato de guardado, la combinación de varios paquetes o prioridades de mods, la
recarga en caliente y la generación procedural. El
[brief del escenario inicial](../../contenido/escenario-inicial.md) define la
intención creativa; este documento solo fija cómo se representa y valida. La
ejecución de scripts se define en
[`08-integracion-lua.md`](./08-integracion-lua.md); aquí solo se carga, asocia y
valida su presencia.

El contenido mínimo deberá incluir al menos un Mundo, un mapa y los Lugares
necesarios para que tanto el viaje oficial del jugador como el del NPC sean
válidos, incluidos el origen y el destino; además incluirá el actor jugador, al
menos un compañero inicial y otro NPC ajeno al Grupo con una política Lua.
Después del viaje del jugador, las condiciones declaradas de esa política
deberán cumplirse y producir una solicitud válida de `Viajar`, de modo que el
recorrido oficial ejercite la fase NPC completa.

Un Lugar ocupado por el Actor que viaja no cuenta como destino disponible para
ese Actor. El fixture deberá situar a cada viajero en un origen distinto del
destino solicitado; no usará un viaje al mismo Lugar para satisfacer el recorrido.

## 2. Responsabilidades y límites

La carga pertenece a `desert-tiger-app`. El filesystem y el formato elegido no
atravesarán su frontera hacia `desert-tiger-core`.

El proceso distinguirá tres representaciones:

1. **DTO externos:** reflejan el esquema de los archivos y permiten informar un
   error en términos comprensibles para quien edita contenido.
2. **Contenido validado:** tiene referencias resueltas y separa datos reglados,
   presentación y políticas Lua. Es inmutable mientras dure una partida.
3. **Estado de Partida:** contiene el estado reglado mutable y una instancia
   visual propia del mapa. Las reglas cambian el primero; la aplicación puede
   actualizar la segunda para reflejar resultados semánticos.

Los DTO no serán los structs del dominio. Reorganizar archivos no deberá obligar
a cambiar las reglas; el núcleo no abrirá archivos ni deserializará formatos.

La carga será **atómica**: podrá acumular varios diagnósticos durante la revisión,
pero solo publicará un contenido validado si el conjunto completo es válido. Una
falla no creará una Partida ni dejará catálogos parcialmente utilizables.

## 3. Entradas y datos cubiertos

El conjunto de contenido del primer incremento aportará, como mínimo:

- un Mundo, su fecha inicial obligatoria y tres listas tipadas de variables
  globales iniciales;
- listas explícitamente ordenadas de los Lugares y Actores que pertenecen a ese
  Mundo;
- la clave externa del único actor jugador, declarada por `actor_jugador` en
  `world.ron`;
- una lista no vacía `companeros_iniciales` con las claves de los Actores que
  forman el Grupo al comenzar la Partida;
- un único mapa visual con una matriz compacta de símbolos ASCII y una leyenda
  que relaciona cada símbolo con una clave de tipo visual;
- lugares, con nombre visible, características, coordenada central y una lista
  ordenada `coordenadas_rotulo`;
- actores, con nombre visible, características y lugar inicial;
- una lista obligatoria de Relaciones dentro de cada archivo de Lugar y Actor,
  vacía cuando el elemento no declara ninguna;
- una lista ordenada de fotogramas de celdas, incorporada en el mismo archivo
  RON del Actor, obligatoria para el jugador y para cualquier compañero inicial;
  puede incluir glifos Unicode de una columna y una paleta ANSI de presentación;
  otros actores podrán declarar explícitamente que no tienen retrato;
- un campo de política en cada Actor, con una variante sin política o un bloque
  Lua incorporado para el NPC que lo necesite.

El esquema, la normalización y las restricciones de esas declaraciones se
definen en [`10-relaciones.md`](./10-relaciones.md#4-formato-ron-y-autoría). En
este incremento el único tipo admitido es `Conectado` entre dos Lugares. No se
deducen Relaciones a partir de coordenadas, tiles o proximidad del mapa.

Cada elemento referenciable tendrá una clave externa estable distinta del nombre
visible. Su sintaxis será ASCII minúscula con palabras separadas por guion bajo:
deberá coincidir con `^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$` y no superar 64 bytes. La
carga no corrige mayúsculas, espacios, acentos, guiones ni separadores repetidos;
los rechaza para evitar que dos textos diferentes se normalicen silenciosamente
a la misma clave.

Las claves son únicas dentro de cada categoría, no globalmente. Por ello, un
Actor y un Lugar pueden compartir el mismo texto sin volverse ambiguos: cada
campo de referencia declara el tipo esperado y resuelve a un ID Rust distinto.
Dentro de una categoría nunca puede haber dos candidatos para la misma clave.
El nombre del archivo sin `.ron` debe coincidir exactamente con la clave declarada
en su interior.

Las Características no usan la sintaxis de esas claves. Se escriben en
`lowerCamelCase`: primera palabra en minúscula y cada palabra posterior iniciada
con mayúscula, como `cargaTesoro`. Se rechazan espacios, guiones y guiones bajos,
y la carga no cambia el uso de mayúsculas. Solo se admiten letras ASCII: se
escribe `rapido`, no `rápido`. Los acentos y demás caracteres Unicode se reservan
para el texto visible al jugador. Una Característica no supera 64 bytes.

Las claves de variables globales usan la misma gramática `lowerCamelCase` ASCII
que las Características y no superan 64 bytes. Forman un único espacio de
nombres: una clave no puede repetirse dentro de una lista ni aparecer en listas
de tipos diferentes. No existe un catálogo equivalente codificado en Rust.

Las claves de tipo visual de tile, denominadas `ClaveTipoTile`, tampoco son
identificadores de archivos. Usan `lowerCamelCase` ASCII, como
`bosqueQuemado`, con la misma gramática de palabras definida para las
Características y sin superar 64 bytes. Son datos exclusivamente visuales: una
regla del núcleo no puede consultarlas aunque su nombre parezca describir el
mundo.

Las rutas internas se resolverán desde la raíz explícita del conjunto de
contenido. Después de resolver enlaces simbólicos y normalizar la ruta, todo
destino deberá continuar dentro de esa raíz. Un Actor no referenciará archivos
auxiliares para su arte o su política Lua en el primer incremento.

La raíz llega ya resuelta como ruta absoluta o relativa desde el binario de
terminal conforme al
[contrato del workspace](./01-workspace-entorno.md#6-entradas-salidas-y-configuración).
El cargador la canonicaliza primero y comprueba que sea un directorio. Después
canonicaliza cada archivo antes de abrirlo y exige que su ruta comience por esa
raíz canónica; así una raíz que sea ella misma un enlace simbólico funciona, pero
un enlace interno que escape del conjunto se rechaza. Para el cargador no existe
una opción de línea de comandos ni una búsqueda de alternativas: usa exactamente
la raíz recibida y espera encontrar allí `world.ron`. Una raíz ausente, ilegible
o que no sea un directorio invalida el inicio completo de la Partida.

## 4. Flujo de carga e instanciación

Conceptualmente, el inicio seguirá este flujo:

```text
raíz de contenido
      ↓ lectura y deserialización
DTO externos completos
      ↓ validación y resolución de referencias
contenido validado e inmutable
      ↓ instanciación sin azar
Partida nueva
```

La validación incluirá, según corresponda:

- archivos legibles y sintaxis válida;
- campos obligatorios, valores y dimensiones admitidos por el esquema;
- identificadores no ambiguos y referencias existentes del tipo correcto;
- sintaxis de claves, unicidad por categoría y coincidencia entre clave y nombre
  de archivo;
- exactamente un Mundo ligado a exactamente un mapa;
- una `fecha_inicial` presente y válida conforme al calendario gregoriano
  aprobado;
- un bloque obligatorio `variables_globales` con sus tres listas presentes,
  claves válidas y ninguna repetición dentro de una lista o entre tipos;
- matriz de mapa no vacía y rectangular, con al menos una fila y una columna;
- leyenda no vacía y uno a uno: cada símbolo ASCII y cada clave visual aparecen
  una sola vez;
- todos los símbolos usados por la matriz están declarados en la leyenda;
- ubicación inicial válida para cada actor;
- nombre visible obligatorio y válido para cada Actor;
- coordenada central y coordenadas de rótulo distintas entre sí y dentro de los
  límites del mapa;
- correspondencia posicional uno a uno entre los caracteres presentables del
  nombre del Lugar y `coordenadas_rotulo`;
- un campo obligatorio `actor_jugador` cuya clave resuelva a exactamente un
  Actor incluido en la lista canónica del Mundo;
- una lista `companeros_iniciales` no vacía, sin claves repetidas ni la clave del
  jugador, cuyas referencias resuelvan a Actores de la lista canónica y cuyas
  ubicaciones iniciales coincidan con la del jugador;
- arte y bloques Lua incorporados, asociados al actor que los contiene y válidos
  para sus respectivos subsistemas;
- exactamente un Actor con `politica: Lua(...)` que no sea el jugador ni un
  compañero inicial; las políticas presentes en esos roles se validan, pero no
  cuentan para este límite;
- campos `relaciones` presentes en todos los Actores y Lugares, destinos
  tipados existentes, extremos compatibles y ausencia de conexiones reflexivas
  o duplicadas en cualquier orientación, conforme al
  [contrato de Relaciones](./10-relaciones.md#5-carga-normalización-y-validaciones);
- invariantes del grupo cuando el contenido defina integrantes iniciales.

Los diagnósticos señalarán al menos archivo, elemento o identificador y causa;
cuando el parser lo permita, también línea y columna. El orden de lectura de un
directorio no podrá cambiar el resultado ni decidir silenciosamente qué
definición prevalece.

La separación de coordenadas se comprueba dentro de cada Lugar para mantener la
correspondencia de su propio nombre. No se detectan ni rechazan coincidencias
entre las coordenadas de rótulo de Lugares diferentes. Su composición es una
decisión del diseñador del mapa, no una condición de validez del contenido.

El orden canónico de Lugares y Actores procede de las listas del Mundo, no del
filesystem ni de un mapa asociativo. La carga rechaza referencias repetidas. El
cargador no recorre los directorios: un archivo físico que no esté enumerado en
`world.ron` se ignora y no forma parte del conjunto validado. Ese orden se
conserva al construir el núcleo y al ofrecer destinos al frontend.

Después de validar, la aplicación conservará por separado:

- las definiciones regladas que necesita el núcleo;
- las Relaciones resueltas en el orden canónico requerido por el constructor
  de Mundo;
- un catálogo inmutable de presentación con la matriz inicial ya resuelta a
  tipos visuales, el conjunto declarado por la leyenda, nombres y coordenadas de
  Lugares, y nombres y retratos animados de Actores, indexado por los
  identificadores correspondientes;
- un catálogo que vincula cada política Lua con su actor.

Al instanciar, la aplicación creará tanto el Mundo reglado como un `EstadoMapa`
con una copia mutable de esa matriz y el conjunto inmutable de tipos
declarados. Los símbolos empleados para escribir el archivo ya no forman parte
de las celdas de ejecución. No se sortearán valores ni se omitirán elementos.
Dos Partidas creadas desde el mismo contenido validado comenzarán con estados y
mapas equivalentes, pero mutar una no afectará a la otra ni al catálogo fuente.
Las coordenadas, los símbolos y las claves de tipo visual no se copiarán al
estado que consultan las reglas.

### API pública de carga

La única entrada pública del componente es:

```rust
pub fn cargar_contenido(
    raiz: &Path,
) -> Result<ContenidoValidado, ErrorContenido>;

#[derive(Clone)]
pub struct ContenidoValidado {
    interno: Arc<ContenidoValidadoInterno>,
}
```

`ContenidoValidado` es opaco, solo puede obtenerse mediante
`cargar_contenido` y su clonación comparte definiciones inmutables mediante
`Arc`. No publica DTO, rutas canónicas, scripts, mapas asociativos ni
colecciones mutables.

Su representación interna es:

```rust
struct ContenidoValidadoInterno {
    plantilla_mundo: DatosInicialesMundo,
    presentacion: CatalogoPresentacion,
    claves: CatalogoClaves,
    politicas: Vec<PoliticaLuaValidada>,
}

impl ContenidoValidado {
    pub(crate) fn instanciar_estado(
        &self,
    ) -> Result<(Mundo, EstadoMapa), ErrorInstanciarContenido>;

    pub(crate) fn presentacion(&self) -> &CatalogoPresentacion;
    pub(crate) fn claves(&self) -> &CatalogoClaves;
    pub(crate) fn politicas(&self) -> &[PoliticaLuaValidada];
}

#[derive(Debug)]
pub(crate) enum ErrorInstanciarContenido {
    Mundo(ErrorConstruirMundo),
    MapaNoCorresponde {
        esperado: IdMapa,
        encontrado: IdMapa,
    },
}
```

Las políticas aparecen en orden canónico de Actor y la secuencia omite las
variantes `Ninguna`. `instanciar_estado` clona `plantilla_mundo`, construye
un Mundo nuevo, crea un EstadoMapa nuevo desde la plantilla visual y comprueba
que ambos usan el mismo `IdMapa`; no consume azar.

Antes de publicar el contenido, el cargador realiza una construcción de
comprobación con `Mundo::nuevo` y la descarta. Por ello, un
`ErrorInstanciarContenido` posterior indica una inconsistencia interna y se
convierte en `ErrorCrearPartida`, no en un nuevo error de autoría.

### Correspondencia de claves

La tabla bidireccional es privada:

```rust
pub(crate) struct CatalogoClaves {
    actores_por_clave: BTreeMap<String, IdActor>,
    claves_por_actor: BTreeMap<IdActor, String>,
    lugares_por_clave: BTreeMap<String, IdLugar>,
    claves_por_lugar: BTreeMap<IdLugar, String>,
}
```

No expone esos mapas. Implementa exactamente `ResolucionClavesLua`, definida
por la integración de políticas:

```rust
impl ResolucionClavesLua for CatalogoClaves {
    fn clave_actor(&self, actor: IdActor) -> Option<&str>;
    fn clave_lugar(&self, lugar: IdLugar) -> Option<&str>;
    fn lugar_por_clave(&self, clave: &str) -> Option<IdLugar>;
}
```

Ambas direcciones se construyen y validan juntas y no pueden cambiar después.

### Tipos públicos de presentación

`ClaveTipoTile`, las coordenadas y los retratos son valores de aplicación que
pueden aparecer en las vistas públicas sin exponer el catálogo:

```rust
pub const MAX_BYTES_CLAVE_TIPO_TILE: usize = 64;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClaveTipoTile(String);

impl ClaveTipoTile {
    pub fn nueva(
        valor: String,
    ) -> Result<Self, ErrorClaveTipoTile>;

    pub fn como_str(&self) -> &str;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorClaveTipoTile {
    SintaxisInvalida,
    DemasiadoLarga {
        longitud_bytes: usize,
        maximo_bytes: usize,
    },
}

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct CoordenadaMapa {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FotogramaRetrato {
    ancho: usize,
    filas: Vec<String>,
}

impl FotogramaRetrato {
    pub fn ancho(&self) -> usize;
    pub fn alto(&self) -> usize;

    pub fn filas(
        &self,
    ) -> impl ExactSizeIterator<Item = &str> + '_;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetratoActor {
    fotogramas: Vec<FotogramaRetrato>,
}

impl RetratoActor {
    pub fn fotogramas(&self) -> &[FotogramaRetrato];
}
```

`ClaveTipoTile::nueva` usa la gramática y límite definidos en la sección 3.
Es pública porque la terminal debe validar las claves de su catálogo de
apariencias. `ErrorClaveTipoTile` implementa `Display` y `Error`.

Los constructores de `FotogramaRetrato` y `RetratoActor` son privados: el
frontend solo recibe arte ya normalizado y validado. `alto()` coincide con la
cantidad de filas, cada fila contiene `ancho()` caracteres ASCII y
`fotogramas()` nunca está vacío.

### Catálogo de presentación y mapa de la Partida

El catálogo inmutable permanece dentro de `desert-tiger-app`:

```rust
pub(crate) struct CatalogoPresentacion {
    mapa: PlantillaMapa,
    lugares: Vec<PresentacionLugar>,
    actores: Vec<PresentacionActor>,
}

struct PlantillaMapa {
    id: IdMapa,
    ancho: usize,
    alto: usize,
    celdas_iniciales: Vec<ClaveTipoTile>,
    tipos_declarados: BTreeSet<ClaveTipoTile>,
}

pub(crate) struct PresentacionLugar {
    id: IdLugar,
    nombre: String,
    coordenada_central: CoordenadaMapa,
    coordenadas_rotulo: Vec<CoordenadaMapa>,
}

pub(crate) struct PresentacionActor {
    id: IdActor,
    nombre: String,
    retrato: Option<RetratoActor>,
}

impl CatalogoPresentacion {
    pub(crate) fn lugares(&self) -> &[PresentacionLugar];
    pub(crate) fn actores(&self) -> &[PresentacionActor];
    pub(crate) fn lugar(
        &self,
        id: IdLugar,
    ) -> Option<&PresentacionLugar>;
    pub(crate) fn actor(
        &self,
        id: IdActor,
    ) -> Option<&PresentacionActor>;

    fn instanciar_mapa(&self) -> EstadoMapa;
}

impl PresentacionLugar {
    pub(crate) fn id(&self) -> IdLugar;
    pub(crate) fn nombre(&self) -> &str;
    pub(crate) fn coordenada_central(&self) -> CoordenadaMapa;
    pub(crate) fn coordenadas_rotulo(
        &self,
    ) -> &[CoordenadaMapa];
}

impl PresentacionActor {
    pub(crate) fn id(&self) -> IdActor;
    pub(crate) fn nombre(&self) -> &str;
    pub(crate) fn retrato(&self) -> Option<&RetratoActor>;
}
```

Los vectores conservan el orden canónico. Las búsquedas por ID son lineales en
el primer incremento; no se mantiene un segundo índice sin una necesidad
medida. `PresentacionActor` conserva el nombre validado del mismo archivo RON que
define al Actor. Ese nombre permite que la aplicación resuelva los participantes
de una traza causal sin exponer la clave técnica ni pedirle al frontend que la
transforme.

Cada Partida posee este estado:

```rust
pub(crate) struct EstadoMapa {
    id: IdMapa,
    ancho: usize,
    alto: usize,
    celdas: Vec<ClaveTipoTile>,
    tipos_declarados: BTreeSet<ClaveTipoTile>,
}

impl EstadoMapa {
    pub(crate) fn id(&self) -> IdMapa;
    pub(crate) fn ancho(&self) -> usize;
    pub(crate) fn alto(&self) -> usize;
    pub(crate) fn celdas(&self) -> &[ClaveTipoTile];

    pub(crate) fn tipos_declarados(
        &self,
    ) -> impl ExactSizeIterator<Item = &ClaveTipoTile> + '_;

    pub(crate) fn tipo_en(
        &self,
        coordenada: CoordenadaMapa,
    ) -> Option<&ClaveTipoTile>;

    pub(crate) fn cambiar_tipo(
        &mut self,
        coordenada: CoordenadaMapa,
        nuevo_tipo: ClaveTipoTile,
    ) -> Result<(), ErrorCambiarTile>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ErrorCambiarTile {
    CoordenadaFueraDeLimites {
        coordenada: CoordenadaMapa,
        ancho: usize,
        alto: usize,
    },
    TipoNoDeclarado {
        tipo: ClaveTipoTile,
    },
}
```

Las celdas usan orden de filas y cumplen
`celdas.len() == ancho * alto`. `tipo_en` y `cambiar_tipo` comprueban
primero `x < ancho` e `y < alto` y calculan el índice mediante operaciones
comprobadas. `cambiar_tipo` valida después que la clave esté declarada y solo
entonces escribe; cualquier error deja la matriz intacta. No existe una operación
para cambiar dimensiones ni el conjunto de tipos.

El iterador de tipos usa el orden lexicográfico del `BTreeSet`. `VistaMapa`
copia las celdas y ese conjunto; una vista previa no observa mutaciones
posteriores.

## 5. Formato y organización

Todos los elementos de contenido usarán RON. Cada Actor y cada Lugar tendrá un
único archivo autocontenido; ningún elemento se dividirá entre un archivo de
datos, otro de arte y otro de script. La distribución inicial será:

```text
content/base/
├── world.ron
├── maps/
│   └── <clave-mapa>.ron
├── places/
│   └── <clave-lugar>.ron
└── actors/
    └── <clave-actor>.ron
```

Todos los archivos son texto UTF-8. Cada struct DTO, incluidos los anidados,
aplica `#[serde(deny_unknown_fields)]`. El `Deserialize` derivado rechaza campos
nombrados repetidos y campos no opcionales ausentes; las repeticiones semánticas
dentro de listas se validan después. El esquema no define alias ni valores
predeterminados. Los comentarios y comas finales admitidos normalmente por RON
sí pueden usarse.

No se admite ninguna extensión activada mediante `#![enable(...)]`; en
particular, `implicit_some` no puede convertir un valor desnudo en `Some`. La
carga crea `ron::de::Deserializer` con opciones predeterminadas, comprueba que
`extensions().is_empty()`, deserializa el DTO y llama a `end()` para rechazar
datos residuales. Los metaatributos `#![type = ...]` y `#![schema = ...]` que RON
acepta e ignora no activan extensiones, no cambian el valor deserializado y se
toleran por esa razón; no forman parte del esquema ni deben usarse en el
contenido oficial. El cargador no realiza un análisis léxico adicional para
rechazarlos.

Las cadenas raw son la forma recomendada para escribir matrices, fotogramas y
código Lua, porque conservan el contenido de manera legible. Después de
deserializar, Rust recibe un `String` y ya no puede distinguir si fue escrito con
sintaxis raw o escapada: ambas formas son válidas si producen exactamente el
mismo valor y este supera las validaciones del campo.

`world.ron` es el punto de entrada y enumera explícitamente las claves del Mapa,
Lugares y Actores que forman el conjunto. También declara `actor_jugador`
mediante una clave de Actor y `companeros_iniciales` mediante una lista de claves
de Actor. Las listas de Lugares y Actores establecen además su orden canónico. La
carga no descubre elementos recorriendo directorios. Cada clave resuelve a la
ruta de su categoría y el valor declarado dentro del archivo debe coincidir con
el nombre de archivo. Como la sintaxis no admite separadores de ruta, una clave
nunca puede escapar de su directorio.

El esquema completo y normativo de `world.ron` es:

```ron
(
    mapa: "mapa_inicial",

    lugares: [
        "zona_arida",
        "paso_escarpado",
        "valle_fertil",
    ],
    actores: [
        "principe_perdido",
        "bandido",
        "cazador",
    ],

    actor_jugador: "principe_perdido",
    companeros_iniciales: [
        "bandido",
    ],

    fecha_inicial: (
        anio: 2025,
        mes: 1,
        dia: 31,
    ),

    variables_globales: (
        booleanas: [],
        enteras: [],
        textos: [],
    ),
)
```

`world.ron` no declara una `clave`: es el único punto de entrada y ningún otro
elemento lo referencia. Todos los campos mostrados son obligatorios. `mapa`
referencia exactamente un archivo; `lugares`, `actores` y
`companeros_iniciales` son listas no vacías y sin repeticiones. Las dos primeras
establecen el orden canónico y la tercera conserva exactamente el orden de los
compañeros. Cada una de las tres listas de variables puede estar vacía.
`world.ron` no reúne las Relaciones: cada declaración permanece en el archivo
del Lugar o Actor que funciona como origen de autoría y la carga las organiza al
instanciar el Mundo.

Cuando se necesiten valores, cada entrada usa exactamente una de estas tres
formas; este ejemplo aislado no añade esas claves al contenido oficial:

```ron
variables_globales: (
    booleanas: [(clave: "eventoOcurrio", valor: false)],
    enteras: [(clave: "contador", valor: 0)],
    textos: [(clave: "estado", valor: "inicial")],
)
```

La fecha inicial usa la estructura numérica mostrada dentro de `world.ron`.

`anio`, `mes` y `dia` son enteros RON sin signo que deben deserializarse como
`u64`; el cargador no conserva su grafía ni exige que sea exclusivamente
decimal. En el contenido oficial se recomienda la forma decimal mostrada. Sus
DTO usan `u64` para que incluso un valor positivo muy superior al tipo final
pueda recibir un diagnóstico de contenido. El año debe pertenecer a `1..=9999`, el mes a
`1..=12` y el día debe existir en ese mes y año según el calendario gregoriano.
La carga valida los rangos antes de convertir a `u16`, `u8`, `Mes` y `Fecha`;
un exceso externo es `ErrorContenido`, no `ErrorCalendario`. No completa una
fecha ausente, no limita un día inválido y no usa la fecha del sistema como
valor predeterminado.

El bloque `variables_globales` y las listas `booleanas`, `enteras` y `textos`
son obligatorios; cada lista puede estar vacía. Se usan listas de entradas, no
mapas RON, para detectar cualquier duplicado antes de que pueda sobrescribir
silenciosamente otra declaración. Las booleanas aceptan solo `true` o `false`,
las enteras abarcan exactamente `i64` y los textos son valores `String`, incluido
el texto vacío.

Los DTO de enteros usan `i128`, validan el intervalo `i64::MIN..=i64::MAX` y
recién entonces convierten el valor. La validación recorre primero `booleanas`,
luego `enteras` y finalmente `textos`, respetando dentro de cada lista el orden
escrito. Un duplicado produce un diagnóstico determinista que identifica la
clave y ambas declaraciones, incluidas sus categorías.

El orden de autoría ayuda a navegar el archivo y producir diagnósticos, pero no
expresa prioridad ni altera el gameplay. Al construir el Mundo, las tres listas
se convierten al registro genérico descrito por el modelo de dominio. Añadir una
clave válida hace que esté disponible para las reglas y Lua en toda Partida
nueva sin modificar Rust. En este incremento toda clave tiene un valor inicial;
la creación o modificación durante una Partida mediante eventos se diseñará más
adelante.

Ser el jugador es un rol de esa configuración del Mundo, no un atributo
intrínseco de la definición de Actor. Por ello, los archivos bajo `actors/` no
declaran un indicador como `es_jugador`; al construir la Partida, la referencia
`actor_jugador` se resuelve a `IdActor` y se entrega al modelo reglado.
El Grupo se construye solamente con `companeros_iniciales`: el jugador permanece
fuera de esa colección, aunque el núcleo exige que todos comiencen juntos.

Durante la validación, la posición de cada clave en su lista canónica se convierte
de forma comprobada al newtype `u32` de su categoría. El catálogo conserva en
ambas direcciones la correspondencia entre clave externa e ID interno. Las
referencias de los DTO se resuelven mediante esa tabla antes de construir una
Partida. Los `LugarInicial` y `ActorInicial` entregados al núcleo llevan esos IDs;
`Mundo::nuevo` comprueba nuevamente que cada uno coincida con su posición
canónica.

El siguiente ejemplo muestra todos los campos normativos de un archivo de Lugar
mediante `zona_arida`. Sus coordenadas solo ilustran la forma del dato; las
coordenadas oficiales se fijarán en `content/base/` después de dibujar el mapa:

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

Todos sus campos son obligatorios. `nombre` no puede estar vacío.
`caracteristicas` y `relaciones` son listas obligatorias que pueden estar vacías;
las primeras no admiten duplicados y las segundas siguen el esquema y la
normalización de [`10-relaciones.md`](./10-relaciones.md#41-esquema-normativo).
Los DTO de `x` e `y` usan `u64`; la carga los convierte de manera
comprobada a `usize` y después valida cada coordenada contra las dimensiones del
Mapa. La coordenada central y todas las coordenadas del rótulo son mutuamente
distintas dentro del mismo Lugar. Lugares diferentes sí pueden superponer sus
rótulos, porque esa composición es responsabilidad del diseño del contenido.

El siguiente ejemplo muestra todos los campos normativos de un archivo de Actor
mediante `principe_perdido`. El fotograma solo ilustra el formato; los datos
oficiales concretos permanecen en `content/base/`:

```ron
(
    clave: "principe_perdido",
    nombre: "Príncipe Perdido",
    lugar_inicial: "zona_arida",
    caracteristicas: [],
    relaciones: [],
    retrato: Some((
        paleta: {
            '0': (frente: "amarillo", fondo: Some("gris")),
            '1': (frente: "negro", fondo: Some("gris")),
        },
        fotogramas: [
            (
                arte: r#"
 █ 
<o>
"#,
                estilos: Some(r#"
000
010
"#),
            ),
        ],
    )),
    politica: Ninguna,
)
```

Todos sus campos son obligatorios. `nombre` es presentación y no puede estar
vacío; admite texto Unicode, pero rechaza controles C0/C1 y secuencias de escape
conforme a la regla general de texto visible. No exige caracteres de una sola
columna porque no se escribe sobre una matriz de tiles. La carga no lo normaliza,
no lo usa para resolver referencias y no exige que sea único: la identidad estable
sigue siendo `clave`. `caracteristicas` y `relaciones` pueden estar vacías; las
primeras no admiten duplicados. Durante este incremento la lista de Relaciones de
todo Actor debe estar vacía porque `Conectado` solo admite Lugares.
`lugar_inicial` es una clave de Lugar incluida en la lista canónica de
`world.ron`.

`retrato` usa exactamente `None` o
`Some((paleta: {...}, fotogramas: [...] ))`. Aunque se
represente como una opción RON, el campo no puede omitirse. El DTO usa
`#[serde(deserialize_with = "deserializar_opcion_requerida")]` sobre
`Option<RetratoDto>`; esa función delega en `Option::<RetratoDto>::deserialize`
sin declarar `default`. Así el campo ausente es un error, mientras `None` sigue
siendo un valor explícito válido según el rol. El jugador y cada compañero
inicial requieren `Some`; los demás Actores admiten cualquiera de los dos
valores. `fotogramas` es una lista ordenada y no vacía, sin duración individual.
Un solo elemento representa un retrato estático.

`paleta` relaciona claves ASCII visibles de una celda con un estilo neutral. Las
claves no se repiten y no pueden ser espacios, controles, comillas ni barra
invertida. `frente` es obligatorio y `fondo` usa `None` o `Some(color)`. Los
colores admitidos son exactamente `negro`, `rojo`, `verde`, `amarillo`, `azul`,
`magenta`, `cian`, `gris`, `grisOscuro`, `grisMuyOscuro`, `rojoClaro`,
`verdeClaro`, `amarilloClaro`, `azulClaro`, `magentaClaro`, `cianClaro` y
`blanco`. Estos valores son presentación neutral: `desert-tiger-app` no
construye un `Style` ni depende de Ratatui.

Cada entrada de `fotogramas` contiene `arte` y `estilos`. La carga convierte
CRLF a LF, rechaza cualquier `CR` aislado y retira como máximo un LF inicial y
uno final de cada bloque. No recorta espacios ni ningún otro carácter. `arte`
debe contener al menos una fila y una columna y cada `char` debe ser imprimible y
ocupar exactamente una columna según `UnicodeWidthChar::width(c) == Some(1)`.
Esto admite los bloques `█`, `▐`, `▌`, `▄` y `▀`, pero rechaza controles,
marcas combinantes, emojis anchos y glifos de ancho ambiguo que la biblioteca no
clasifique como una columna. Todas las filas y todos los fotogramas del Actor
tienen idénticos ancho y alto medidos en columnas, no en bytes UTF-8. Los
espacios finales cuentan para el ancho.

`estilos: None` aplica a cada celda visible el estilo predeterminado del
frontend y permite retratos monocromos sencillos. `estilos: Some(bloque)` exige
una matriz rectangular con las mismas dimensiones que `arte`; cada carácter de
esa matriz debe ser una clave declarada en `paleta`. El estilo de una celda cuyo
glifo sea un espacio se valida pero no elimina su transparencia. Una paleta
vacía es válida cuando ningún fotograma la referencia; una clave declarada y no
usada también es válida para facilitar iteraciones artísticas. Actores distintos
pueden tener dimensiones y paletas distintas; su distribución pertenece al
frontend.

El formato `.dur` no se carga durante una Partida. El binario de autoría
`importar-retrato-durdraw` de `desert-tiger-terminal` acepta archivos Durdraw de
versión 8, comprimidos con gzip, con codificación UTF-8 y paleta de 16 colores.
Valida dimensiones, secuencia de fotogramas, glifos, `colorMap` y demoras nulas,
y escribe de forma determinista un fragmento RON `RetratoDto` por la salida
estándar. La frecuencia global del archivo es informativa y no se copia porque
la cadencia pertenece al frontend. El importador nunca modifica el Actor: el
diseñador revisa e incorpora el fragmento al único RON canónico del elemento.
`flate2` y `serde_json` se justifican exclusivamente por la lectura segura de
gzip y JSON en esa herramienta de autoría.

En el modo admitido, los índices frontales `0..=16` siguen la paleta de frente
de Durdraw; `0` y `1` representan negro. Los índices de fondo tienen una tabla diferente: `1..=7` se
convierten respectivamente en azul, verde, cian, rojo, magenta, amarillo y gris.
El fondo `0` es el predeterminado y `8` es el negro del modo normal; el
importador convierte deliberadamente ambos en `None`: para retratos
superpuestos, ese negro de autoría funciona como transparencia. Los fondos
`9..=16`, propios de variantes no cubiertas por este subconjunto, se rechazan.
Un negro opaco aún puede declararse expresamente en la paleta RON después de
importar.

Desde la raíz del repositorio, la conversión se ejecuta así:

```sh
cargo run -p desert-tiger-terminal --bin importar-retrato-durdraw -- assets/principe.dur
```

La salida se muestra en la terminal para poder revisarla y copiarla en el campo
`retrato` del Actor. Regenerarla no altera ningún archivo de contenido.

`politica` usa exactamente `Ninguna` o `Lua(String)` y tampoco puede omitirse.
Por ejemplo, un NPC sin solicitud activa puede contener:

```ron
politica: Lua(r#"return nil"#),
```

Nunca contiene una ruta a un `.lua`. El bloque presente se prepara y valida
durante la carga conforme a la
[integración Lua](./08-integracion-lua.md). Un Actor completo escrito como script
Lua tampoco es un formato admitido: Lua decide si solicita una acción, pero no
construye la definición inicial del Actor.

Un archivo de Mapa incorpora una leyenda junto con su matriz compacta. El
siguiente ejemplo mínimo define la forma normativa del esquema, pero no es la
matriz del fixture oficial ni debe combinarse con las coordenadas del ejemplo
de Lugar anterior:

```ron
(
    clave: "mapa_inicial",
    leyenda: {
        '.': "arena",
        'B': "bosque",
        'Q': "bosqueQuemado",
    },
    matriz: r#"
........
..BBB...
..BQB...
........
"#,
)
```

Los tres campos son obligatorios y la leyenda no puede estar vacía. `clave` debe
coincidir con el nombre del archivo y con la referencia `mapa` de `world.ron`.
Como existe un solo Mapa en este alcance, la carga le asigna
`IdMapa::desde_posicion(0)`.

Cada clave de `leyenda` es un símbolo de autoría que ocupa exactamente un byte
ASCII visible. Para mantener segura y sencilla tanto la cadena raw como la clave
RON, no puede ser un espacio, un control, comillas simples, comillas dobles ni
barra invertida. Cada valor es una `ClaveTipoTile`. El símbolo permite reconocer
y editar el dibujo con facilidad, pero no es el glifo que el frontend está
obligado a mostrar: `B` puede codificar `bosque` y la terminal representarlo con
otro carácter y color.

La matriz se escribe normalmente como una cadena raw multilínea, sin comas entre
celdas ni filas. Si los delimitadores ocupan líneas separadas, como en el ejemplo,
la carga retira como máximo un salto inicial y uno final; estos son opcionales y
no forman filas. Antes de ese paso convierte CRLF a LF y rechaza cualquier `CR`
aislado. Dentro del cuerpo restante, cada línea es una fila, no se permite
indentación, tabulaciones ni filas vacías y todas las filas tienen el mismo
número de bytes. Las dimensiones se infieren de la matriz y no se declaran en
otro campo.

La coordenada `(0, 0)` es la celda superior izquierda; `x` crece hacia la derecha
e `y` hacia abajo. Durante la validación se sustituye cada símbolo por la clave
visual correspondiente. Un símbolo usado sin entrada es un error. La leyenda
puede declarar un tipo que todavía no aparezca en la matriz inicial, lo que
permite preparar, por ejemplo, `bosqueQuemado` para un cambio posterior. Dentro
de un Mapa, cada símbolo y cada clave visual aparecen una sola vez; la lectura
de RON debe detectar duplicados en vez de permitir que uno sobrescriba a otro.
Por ello, la leyenda no se deserializa primero a un `HashMap`, `BTreeMap` ni
`ron::Value::Map`. El newtype privado `LeyendaDto` implementa `deserialize_map`,
consume `MapAccess::next_entry::<char, String>()` en orden de autoría y mantiene
conjuntos de símbolos y claves visuales ya vistos. Rechaza ambos tipos de
duplicado antes de construir el mapa validado. La relación uno a uno conserva
una única abreviatura de autoría para cada tipo.

El conjunto de contenido no contiene ilustraciones de viñetas ni resuelve claves
semánticas de escena. Esos recursos pertenecen exclusivamente a cada frontend;
los retratos incorporados en archivos de Actor son la única excepción artística
que este cargador entrega a la presentación. La matriz compacta no se considera
arte ASCII: es una codificación cómoda de tipos visuales.

El esquema RON deberá preservar la codificación del mapa y el arte de Actor sin
cambios accidentales, permitir comentarios o documentación suficiente para
autores y producir buenos diagnósticos. Todo texto de contenido que pueda llegar
a la terminal —nombres, arte y mensajes incluidos en diagnósticos— rechazará
controles C0/C1 y secuencias de escape, salvo los separadores que el esquema
permita expresamente. Los colores neutrales del retrato constituyen la única
excepción de contenido; cursor, secuencias ANSI y composición siguen
perteneciendo al frontend. Un
diagnóstico sobre texto inválido lo representará de forma escapada, sin copiar
el control directamente a la salida.

Para el nombre de un Lugar, “carácter” significa un único valor `char` de Rust.
Cada valor debe ser imprimible y satisfacer exactamente
`UnicodeWidthChar::width(c) == Some(1)`; no se usa `width_cjk`. El espacio ASCII
también está permitido y ocupa una coordenada. Letras precompuestas como `á` y `ñ` son
válidas. Se rechazan caracteres de ancho cero o doble, controles, emojis anchos y
secuencias descompuestas que contengan marcas combinantes. La carga no normaliza
el texto silenciosamente: el diagnóstico indicará el carácter y la posición que
incumplen la regla.

La cantidad de `coordenadas_rotulo` debe coincidir con
`nombre.chars().count()`. Esta restricción pertenece únicamente al nombre que se
dibuja sobre tiles; otros textos visibles podrán admitir composiciones Unicode
más amplias cuando su región de presentación las soporte.

## 6. Contratos e invariantes

- Una Partida solo nace de un conjunto completamente validado.
- El estado inicial no consume azar y reproduce los valores declarados.
- Toda clave global aparece una sola vez entre las tres listas y conserva su tipo
  y valor inicial al instanciar la Partida.
- Toda referencia se resuelve explícitamente; no se enlaza por nombre visible,
  posición en una lista ni orden de archivos.
- El nombre visible de un elemento no sustituye a su identificador estable.
- Cada Actor posee un nombre visible validado que el catálogo conserva junto a
  su ID; una presentación nunca necesita mostrar su clave externa como nombre.
- Cada actor empieza en un lugar existente y nunca en un tile.
- El mapa y sus coordenadas son datos de presentación y no habilitan acciones.
- Cada celda del `EstadoMapa` contiene exactamente una clave de tipo visual
  declarada; nunca contiene el símbolo compacto usado en el archivo ni un glifo
  elegido por la terminal.
- Cambiar un tile selecciona otro tipo visual declarado y no altera las
  dimensiones de la matriz en el primer incremento.
- El carácter situado en la posición `n` del nombre de un Lugar se dibuja sobre
  el tile de `coordenadas_rotulo[n]` sin alterar la matriz del mapa.
- Los cambios de la matriz de una Partida no modifican los archivos, el catálogo
  validado ni la matriz de otra Partida.
- Una política Lua queda ligada a su actor; el script no elige otra identidad.
- Pertenecer al Grupo no elimina una política Lua del catálogo, pero impide que
  la fase NPC la evalúe mientras dure esa pertenencia.
- El contenido inicial tiene exactamente una política ligada a un NPC
  independiente y, por ello, una única política evaluable en la fase inicial.
- Ningún archivo o catálogo de contenido aporta arte de viñetas; una clave
  semántica narrativa no se resuelve durante la carga.
- La validez de una `ClaveTipoTile` no depende de que un frontend concreto tenga
  una apariencia para ella. El cargador de contenido no abre ni consulta el
  catálogo privado de la terminal.
- Cada Actor y Lugar procede de un solo archivo RON; una definición no depende de
  fragmentos externos de arte o Lua.
- Cada Relación se declara una sola vez junto a su origen de autoría y se
  resuelve a los IDs del mismo Mundo; `world.ron` y el mapa no mantienen copias
  paralelas del vínculo.
- Cada celda de retrato contiene un glifo de una columna y, opcionalmente, un
  estilo neutral resuelto desde la paleta del mismo Actor; nunca contiene una
  secuencia ANSI ni un tipo de Ratatui.
- Los archivos fuente no se modifican durante la carga ni durante la Partida.
- No habrá valores por defecto silenciosos que alteren el significado del
  contenido. La ausencia semántica de retrato o política se declara mediante su
  variante explícita y no omitiendo el campo.

## 7. Errores, pruebas y aceptación

### API de diagnósticos

La carga devuelve un único error agregado que siempre contiene al menos un
diagnóstico:

```rust
#[derive(Debug)]
pub struct ErrorContenido {
    diagnosticos: Vec<DiagnosticoContenido>,
}

impl ErrorContenido {
    pub fn diagnosticos(&self) -> &[DiagnosticoContenido];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaseDiagnosticoContenido {
    AccesoFilesystem,
    RutaFueraDeRaiz,
    FormatoRonInvalido,
    ClaveInvalida,
    Duplicado,
    ReferenciaInvalida,
    ValorInvalido,
    PoliticaLuaInvalida,
    InvarianteDominio,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoElementoContenido {
    Mundo,
    Mapa,
    Lugar,
    Actor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrigenDiagnosticoContenido {
    Raiz {
        ruta: PathBuf,
    },
    Archivo {
        ruta_relativa: PathBuf,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementoDiagnosticoContenido {
    pub tipo: TipoElementoContenido,
    pub clave: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosicionDiagnosticoContenido {
    Fuente {
        linea: usize,
        columna: usize,
    },
    Matriz {
        fila: usize,
        columna: usize,
        simbolo: Option<char>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticoContenido {
    clase: ClaseDiagnosticoContenido,
    origen: OrigenDiagnosticoContenido,
    elemento: Option<ElementoDiagnosticoContenido>,
    campo: Option<String>,
    posicion: Option<PosicionDiagnosticoContenido>,
    detalle: String,
}

impl DiagnosticoContenido {
    pub fn clase(&self) -> ClaseDiagnosticoContenido;
    pub fn origen(&self) -> &OrigenDiagnosticoContenido;
    pub fn elemento(&self) -> Option<&ElementoDiagnosticoContenido>;
    pub fn campo(&self) -> Option<&str>;
    pub fn posicion(&self) -> Option<&PosicionDiagnosticoContenido>;
    pub fn detalle(&self) -> &str;
}
```

`ErrorContenido` y `DiagnosticoContenido` implementan `Display`; el primero
también implementa `std::error::Error`. Sus textos no son API estable: la
terminal usa los campos estructurados. `detalle` es una sola línea, tiene los
controles y escapes representados de forma visible y nunca contiene un archivo,
retrato o bloque Lua completo.

Una ruta de raíz puede no ser UTF-8 y por eso se conserva como `PathBuf`; quien
la presenta usa `to_string_lossy` y vuelve a aplicar la política de texto seguro.
Las rutas relativas de archivos proceden de directorios fijos y claves ASCII.
Las posiciones `Fuente` usan línea y columna comenzando en uno, como el parser.
Las posiciones `Matriz` usan fila y columna comenzando en cero, igual que las
coordenadas del mapa.

Las categorías se asignan así:

- una raíz o archivo ausente, ilegible o de tipo incorrecto usa
  `AccesoFilesystem`;
- un enlace cuyo destino canónico no comienza por la raíz canónica usa
  `RutaFueraDeRaiz`;
- RON inválido, campos desconocidos, repetidos u omitidos, extensiones activadas
  o datos residuales usan `FormatoRonInvalido`;
- una clave con sintaxis inválida o distinta del nombre de archivo usa
  `ClaveInvalida`;
- una repetición en listas, referencias, Características, Relaciones, variables,
  leyenda o coordenadas usa `Duplicado`;
- una referencia ausente o de otra categoría usa `ReferenciaInvalida`;
- fechas, rangos, dimensiones, Unicode, matrices, retratos, extremos de Relación
  incompatibles y la cantidad inicial de políticas independientes usan
  `ValorInvalido`;
- un error producido por `validar_politica` usa
  `PoliticaLuaInvalida`;
- un rechazo de la construcción final de comprobación del Mundo usa
  `InvarianteDominio`.

No se expone `std::io::Error`, `ron::Error` ni `mlua::Error`. Su categoría y
causa segura quedan incorporadas al diagnóstico.

Si no se puede canonicalizar la raíz o leer y deserializar `world.ron`, la
carga termina con un único diagnóstico porque todavía no existe un grafo
confiable que recorrer. Una vez obtenido `WorldDto`, procesa el Mapa, los
Lugares y los Actores en orden canónico y acumula defectos independientes. Omite
una validación dependiente cuando su requisito falló para no producir errores en
cascada.

El orden final siempre es: `world.ron`, Mapa, Lugares en su orden declarado,
Actores en su orden declarado, Relaciones en el orden definido por su documento
y validaciones cruzadas. El orden del filesystem nunca interviene.

### Fixture vertical oficial

El recorrido técnico mínimo dentro del contenido oficial es la fuente canónica
de la prueba de humo y materializa el
[brief del escenario](../../contenido/escenario-inicial.md). El contenido reúne
los veintiséis Lugares del [atlas regional](../../contenido/atlas-regional.md) y
conserva estas claves técnicas estables y precondiciones del recorrido:

- `principe_perdido` y `bandido` comienzan en `zona_arida`, y `bandido` es el
  único integrante inicial del Grupo;
- `cazador` comienza en `valle_fertil`, no pertenece al Grupo y contiene la
  política canónica de
  [`08-integracion-lua.md`](./08-integracion-lua.md#43-solicitud-de-acción);
- `zona_arida`, `paso_escarpado` y `valle_fertil` conservan los IDs canónicos
  `0`, `1` y `2`;
- la ruta física entre `zona_arida` y `valle_fertil` atraviesa
  `camino_de_los_mojones`, `paso_escarpado` y `descenso_del_draa`; las lecturas
  inversas de `Conectado` no se duplican en los archivos;
- `paso_escarpado` posee `empinado` y `bandido` posee `fuerte`; la viabilidad
  se deriva de los hechos físicos y no de una etiqueta genérica;
- la fecha inicial es `2025-01-31`.

El recorrido envía primero
`SolicitudAccion::Viajar { actor: principe_perdido, destino: valle_fertil }`.
La ruta automática atraviesa `paso_escarpado`; el jugador elige
`Ascender directamente` y `fuerte` en el Bandido evita una demora adicional.
Con la fuente alta del humo reproducible, Príncipe Perdido y Bandido llegan a
`valle_fertil` y la fecha queda exactamente en `2025-02-05`. La fase NPC evalúa
entonces a
`cazador`; la condición temporal ya es verdadera y produce
`SolicitudAccion::Viajar { actor: cazador, destino: zona_arida }`. Ese viaje
mueve solo al NPC por la misma red, pero su duración no vuelve a aplicarse a la
fecha y deja el Mundo todavía en `2025-02-05`.

Evaluada contra la instantánea inicial, la misma política devuelve `nil`. Una vez
que el Cazador se encuentra en `zona_arida`, también devuelve `nil` aunque la
fecha siga cumpliendo la condición. No usa una variable alternativa ni otro
cambio de estado para habilitarse. Los retratos, la matriz y las coordenadas
exactas se crearán siguiendo el brief y serán datos canónicos de `content/base/`.

### Ejemplos inválidos

Estos casos son normativos. El primer Actor omite `retrato`; declarar
`retrato: None` habría sido válido para un NPC independiente, pero omitir el
campo produce `FormatoRonInvalido`:

```ron
(
    clave: "cazador",
    nombre: "Cazador",
    lugar_inicial: "valle_fertil",
    caracteristicas: [],
    relaciones: [],
    politica: Ninguna,
)
```

Este Actor incluye el campo desconocido `titulo` y también produce
`FormatoRonInvalido`:

```ron
(
    clave: "cazador",
    nombre: "Cazador",
    titulo: "Guardián del valle",
    lugar_inicial: "valle_fertil",
    caracteristicas: [],
    relaciones: [],
    retrato: None,
    politica: Ninguna,
)
```

Dentro de un `world.ron` por lo demás válido, repetir una clave entre listas de
variables produce `Duplicado` y señala ambas declaraciones:

```ron
variables_globales: (
    booleanas: [
        (clave: "puertaAbierta", valor: false),
    ],
    enteras: [],
    textos: [
        (clave: "puertaAbierta", valor: "cerrada"),
    ],
),
```

Finalmente, este archivo es RON válido pero su referencia no resuelve y produce
`ReferenciaInvalida` sobre `lugar_inicial`:

```ron
(
    clave: "cazador",
    nombre: "Cazador",
    lugar_inicial: "lugar_que_no_existe",
    caracteristicas: [],
    relaciones: [],
    retrato: None,
    politica: Ninguna,
)
```

Los errores esperables se devolverán como datos, no mediante `panic`. Todos los
archivos y campos del esquema inicial son obligatorios; la ausencia explícita de
retrato o política se expresa con los valores `None` y `Ninguna`, y la ausencia
de Relaciones con `relaciones: []`, no omitiendo esos campos. Sintaxis inválida,
identificador duplicado, referencia rota, campo
desconocido, nombre visible inválido, matriz o leyenda inválida, coordenada fuera del mapa, variable
global inválida, Relación inválida, retrato inválido o bloque Lua inválido
impedirá crear la Partida completa. Un error de matriz indicará, cuando
corresponda, archivo, fila, columna y símbolo.

Las pruebas automáticas cubrirán:

- un conjunto mínimo válido y cada clase de error anterior;
- carga y resolución de Relaciones, junto con todos los casos de contenido
  enumerados en [`10-relaciones.md`](./10-relaciones.md#82-contenido);
- clonación barata de `ContenidoValidado` con catálogos compartidos y estados
  de Partida independientes;
- construcción de `ClaveTipoTile` válida y rechazo exacto de sintaxis o longitud
  inválidas;
- consultas por coordenada y cambio atómico de tile, incluidos coordenada fuera
  de límites y tipo no declarado;
- igualdad de las instantáneas iniciales creadas desde una misma plantilla de
  mapa;
- diagnósticos con el contexto del origen;
- agregado siempre no vacío, orden determinista de diagnósticos y omisión de
  errores dependientes en cascada;
- independencia respecto del orden del filesystem;
- rechazo de campos desconocidos, repetidos u obligatorios ausentes en cada tipo
  de archivo;
- exigencia de `nombre` en cada Actor, conservación exacta de un nombre Unicode
  visible y rechazo de texto vacío, controles C0/C1 o secuencias de escape;
- proyección conjunta de ID, nombre y retrato de Actor mediante el catálogo de
  presentación, sin usar el nombre para resolver referencias;
- rechazo de cualquier `#![enable(...)]`, incluido `implicit_some`, y de datos
  residuales después del único valor raíz;
- omisión deliberada de archivos físicos que no estén enumerados en `world.ron`;
- rechazo de rutas que, tras resolver enlaces, escapen de la raíz;
- apertura de la raíz exacta recibida, sin búsqueda de alternativas;
- rechazo de controles de terminal en campos visibles;
- rechazo atómico cuando coexistieran datos válidos e inválidos;
- resolución tipada de referencias;
- rechazo de `actor_jugador` ausente, desconocido o no incluido en la lista
  canónica de Actores;
- rechazo de `companeros_iniciales` vacío, repetido, desconocido, que contenga al
  jugador o que sitúe un compañero fuera del Lugar inicial del jugador;
- rechazo de `fecha_inicial` ausente, de componentes fuera de rango y de días
  inexistentes, incluidos los casos bisiestos; aceptación de una fecha límite
  válida;
- aceptación de las tres listas globales vacías y de un valor válido de cada
  tipo;
- rechazo individual del bloque de variables o de cualquiera de sus tres listas
  cuando falte;
- aceptación de `i64::MIN` y `i64::MAX`, y rechazo de los enteros inmediatamente
  exteriores;
- rechazo de una variable con tipo RON incorrecto, clave inválida o clave mayor
  que 64 bytes;
- rechazo de una Característica mayor que 64 bytes;
- rechazo de duplicados dentro de una lista y entre listas distintas, con un
  diagnóstico estable que señale ambas declaraciones;
- adición de una clave nueva solo en un fixture RON, seguida de su consulta desde
  una política Lua, sin ningún cambio en una declaración Rust;
- aceptación de una misma clave textual en categorías distintas de elementos,
  como Actor y Lugar, y rechazo de un duplicado dentro de la misma categoría;
- rechazo de claves con sintaxis inválida, de nombres de archivo discordantes y
  de una cantidad que no pueda representarse mediante los IDs internos;
- igualdad inicial y aislamiento entre dos Partidas;
- conservación exacta de la misma `fecha_inicial` al instanciar dos Partidas
  desde el mismo contenido;
- igualdad inicial y aislamiento de las variables globales de esas Partidas;
- aislamiento de las modificaciones de tiles entre Partidas y respecto del
  catálogo validado;
- resolución de una matriz compacta a la secuencia exacta de tipos visuales y
  ausencia de símbolos de autoría en el `EstadoMapa` resultante;
- rechazo de una matriz vacía, una fila vacía, filas de distinto ancho,
  tabulaciones, indentación y símbolos sin entrada en la leyenda;
- rechazo de símbolos que no sean ASCII visibles, de símbolos repetidos y de
  claves visuales repetidas o con sintaxis inválida;
- aceptación de una entrada de leyenda todavía ausente de la matriz y uso de ese
  tipo en una modificación posterior de una celda;
- aceptación de una `ClaveTipoTile` válida aunque no exista en el catálogo de
  apariencias de la terminal;
- interpretación equivalente de LF y CRLF y tratamiento explícito del único
  salto inicial y final admitido en la cadena raw;
- ausencia de consumo de azar durante la instanciación;
- límites de coordenadas y separación del mapa respecto del núcleo;
- orden y cardinalidad de la correspondencia entre el nombre de cada Lugar y sus
  coordenadas de rótulo;
- aceptación de dos Lugares cuyas coordenadas de rótulo coincidan entre sí;
- aceptación de ASCII, espacios y letras acentuadas de ancho uno, y rechazo de
  caracteres de ancho cero o doble y de secuencias con marcas combinantes;
- validación conjunta de todo el contenido oficial;
- aceptación de un retrato con un fotograma y rechazo de una lista vacía, un
  bloque no rectangular o dimensiones distintas entre fotogramas del mismo
  Actor;
- exigencia del campo `retrato`, aceptación explícita de `None` para un NPC y
  rechazo de `None` para el jugador o un compañero inicial;
- normalización CRLF de fotogramas, rechazo de `CR` aislado y tratamiento exacto
  del único LF inicial y final;
- exigencia del campo `politica` y aceptación de sus variantes `Ninguna` y
  `Lua(String)`;
- validación de una política Lua incorporada en un compañero, aunque la fase NPC
  no vaya a evaluarla mientras pertenezca al Grupo.
- rechazo de cero o más de una política `Lua` asociada a Actores que no sean el
  jugador ni compañeros iniciales, sin contar las políticas de esos roles.

La prueba de humo cargará este fixture desde archivos reales y recorrerá el flujo
completo. Los demás componentes lo enlazan y no mantienen copias normativas de
sus datos o condiciones.

El componente será aceptable cuando el contenido oficial produzca una Partida
nueva repetible, cualquier defecto conocido impida una carga parcial y una
prueba pueda modificar tanto su estado reglado como su mapa visual sin cambiar el
catálogo ni otra instancia.

## 8. Preguntas bloqueantes

- Ninguna.

## 9. Decisiones aplazadas

- historia detallada y presentación definitiva de los Actores;
- cambios posteriores en la composición del Grupo;
- tipos de Relación distintos de `Conectado`, Relaciones mutables y facciones;
- transformación visual de zonas y tiles;
- evolución entre versiones del esquema;
- combinación, prioridad y compatibilidad de paquetes de mods;
- recarga de contenido durante una Partida;
- duraciones individuales por fotograma o varias animaciones para un mismo Actor.

## 10. Referencias técnicas

- [RON y su integración con Serde](https://docs.rs/ron/latest/ron/)
- [Gramática de RON y cadenas raw multilínea](https://docs.rs/crate/ron/latest/source/docs/grammar.md)
