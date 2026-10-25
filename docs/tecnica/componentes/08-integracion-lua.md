# Integración de Lua para decisiones de NPC

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.8 |
| Fecha | 2026-10-15 |

## 1. Propósito y alcance

Este documento define cómo `desert-tiger-app` ejecuta el bloque Lua incorporado
en un Actor para decidir si solicita una acción. Lua permite modificar las
condiciones de actuación mediante contenido, pero no implementa reglas, no muta
el Mundo, no ejecuta acciones y no decide su presentación.

Una política puede estar presente en cualquier Actor y se valida al cargar el
contenido. La [fase NPC](./07-fase-npc.md) decide cuáles evalúa: nunca evalúa la
del jugador ni la de un integrante actual del Grupo.

El primer incremento ofrece solamente las consultas necesarias sobre la fecha,
las variables globales y el propio Actor, y permite solicitar `Viajar`. Las
consultas sobre otros Actores, Lugares o sistemas se añadirán solo cuando una
regla aprobada las necesite.

## 2. Propiedad y dependencias

La integración es un módulo interno de `desert-tiger-app`. Usa `mlua 0.12.2`
con Lua 5.4 incorporado mediante las características `lua54` y `vendored`, como
fija el [workspace](./01-workspace-entorno.md#dependencias-directas-aprobadas).
Ni `desert-tiger-core` ni `desert-tiger-terminal` dependen de `mlua`.

El bloque permanece como una cadena dentro del único RON del Actor. No se
admiten archivos `.lua`, importaciones ni una definición completa del Actor en
Lua. El núcleo recibe un `SolicitudAccion` normal y no distingue si se originó en
el jugador, Lua o una prueba.

La correspondencia bidireccional entre claves externas e IDs internos pertenece
al catálogo de contenido. La integración usa solamente esta vista estrecha:

```rust
pub(crate) trait ResolucionClavesLua {
    fn clave_actor(&self, actor: IdActor) -> Option<&str>;
    fn clave_lugar(&self, lugar: IdLugar) -> Option<&str>;
    fn lugar_por_clave(&self, clave: &str) -> Option<IdLugar>;
}
```

El catálogo validado implementa el trait sin consultar el filesystem. Un `None`
para un ID que procede de la misma Partida representa una inconsistencia interna,
no una ausencia normal de contenido.

## 3. Contrato Rust interno

El módulo conserva una política validada con su asociación, origen y texto:

```rust
pub(crate) struct PoliticaLuaValidada {
    actor: IdActor,
    clave_actor: String,
    archivo_relativo: PathBuf,
    codigo: String,
}

impl PoliticaLuaValidada {
    pub(crate) fn actor(&self) -> IdActor;
}

pub(crate) enum ResultadoPoliticaLua {
    Ninguna,
    Solicitud(SolicitudAccion),
}

pub(crate) fn validar_politica(
    actor: IdActor,
    clave_actor: String,
    archivo_relativo: PathBuf,
    codigo: String,
) -> Result<PoliticaLuaValidada, ErrorPoliticaLua>;

pub(crate) fn evaluar_politica<R: ResolucionClavesLua>(
    politica: &PoliticaLuaValidada,
    mundo: &Mundo,
    claves: &R,
) -> Result<ResultadoPoliticaLua, ErrorPoliticaLua>;
```

Los campos de `PoliticaLuaValidada` son privados y el valor solo se obtiene de
`validar_politica`. La ruta es relativa a la raíz de contenido ya validada. La
función de validación compila el bloque para comprobar su sintaxis, pero no lo
ejecuta. La evaluación vuelve a compilarlo dentro de una máquina nueva; no se
conservan funciones, bytecode, tablas, `upvalues` ni objetos Lua entre llamadas.

Antes de crear el entorno, `evaluar_politica` comprueba que el Actor asociado
exista en el Mundo y que sus IDs puedan resolverse en el catálogo. Copia a datos
propios la fecha, las variables globales, la clave y Lugar actual del Actor y sus
Características. Esa instantánea permanece estable durante toda la llamada y no
mantiene un acceso mutable al Mundo.

La elegibilidad, el orden y la cantidad de evaluaciones no forman parte de esta
API; pertenecen a la fase NPC.

## 4. API disponible dentro de Lua

El bloque ve exactamente tres valores globales de solo lectura: `mundo`, `actor`
y `acciones`. No se instala `_G` ni ningún otro nombre global.

### 4.1 Mundo

La fecha es la ya vigente al comenzar la fase NPC. Sus componentes son enteros y
el mes usa `1` para enero y `12` para diciembre:

```lua
mundo.fecha.anio
mundo.fecha.mes
mundo.fecha.dia
```

Las variables globales se consultan con una función distinta por tipo:

```lua
mundo.booleano("incendioBosqueOcurrio")
mundo.entero("vecesVisitadoBosque")
mundo.texto("estadoPuertaNorte")
```

Cada función exige exactamente una cadena Lua con UTF-8 válido. Rust construye
una `ClaveVariableGlobal` y consulta la instantánea con la API tipada del Mundo.
No existe una lista de claves codificada en Rust: cualquier clave válida añadida
a `world.ron` queda disponible al crear una Partida nueva.

Una clave ausente o una consulta con el tipo incorrecto falla; nunca produce
silenciosamente `false`, cero, texto vacío ni `nil`.

### 4.2 Actor vinculado

Lua puede leer la identidad externa y ubicación actual del propio Actor, y
consultar una Característica:

```lua
actor.clave
actor.lugar
actor.tiene_caracteristica("herido")
```

`actor.clave` y `actor.lugar` son cadenas estables del contenido, no IDs
numéricos ni nombres visibles. `tiene_caracteristica` exige exactamente una
cadena UTF-8 válida, la convierte mediante `Caracteristica::nueva` y devuelve un
booleano. Una Característica bien formada pero ausente devuelve `false`; una
clave mal formada produce un error.

No se acepta un Actor como argumento. Por ello, un script no puede inspeccionar
ni suplantar a otro mediante esta API inicial.

### 4.3 Solicitud de acción

La única factoría inicial es:

```lua
acciones.viajar("zona_arida")
```

Recibe exactamente una clave externa de Lugar como cadena UTF-8 y devuelve un
objeto opaco e inmutable de tipo privado `SolicitudLua`. Crear el objeto no
consulta reglas, no consume azar y no ejecuta la acción. Lua no puede indicar el
Actor que actuará.

```rust
enum SolicitudLua {
    Viajar {
        destino: String,
    },
}

impl mlua::UserData for SolicitudLua {}
```

El userdata no registra métodos, campos ni metamétodos. Las funciones expuestas
reciben `MultiValue` y comprueban su cardinalidad antes de convertir el único
argumento; no dependen de la regla usual de Lua que ignora argumentos
adicionales. Las cadenas se reciben como `mlua::String` y se aceptan solamente
si `to_str()` confirma UTF-8.

El bloque completo devuelve:

- ningún valor o un único `nil`, para no solicitar una acción;
- un único objeto producido por `acciones.viajar`, para solicitarla.

La llamada del chunk obtiene `MultiValue` para conservar la cantidad exacta.
Múltiples valores o cualquier otro tipo producen `RespuestaInvalida`. Al
convertir el objeto, Rust valida la sintaxis de la clave, la resuelve mediante
`ResolucionClavesLua` y construye:

```rust
SolicitudAccion::Viajar {
    actor: politica.actor,
    destino,
}
```

Una clave mal formada produce `ClaveLugarInvalida`; una clave bien formada que
no resuelva produce `LugarInexistente`. La solicitud
resultante todavía atraviesa todas las validaciones del motor; haberla construido
no implica que sea aceptable en el estado vigente.

El bloque oficial del fixture vertical será:

```lua
if actor.lugar == "valle_fertil"
    and mundo.fecha.anio == 2025
    and mundo.fecha.mes >= 2
then
    return acciones.viajar("zona_arida")
end

return nil
```

La política pertenece a `cazador`, que comienza en `valle_fertil`. La fecha
inicial del fixture estará en enero y el viaje de `principe_perdido` junto con
`bandido` la hará avanzar a febrero. Por ello, la política no solicita nada
antes de esa acción y después solicita el viaje del Cazador a `zona_arida`. La
condición sobre `actor.lugar` hace que una evaluación posterior vuelva a
producir `nil` en vez de solicitar un viaje al Lugar donde ya se encuentra.

## 5. Entorno aislado y determinista

Cada validación y evaluación crea una máquina con
`Lua::new_with(StdLib::NONE, LuaOptions::new())`. No se habilita ninguna
biblioteca estándar: quedan ausentes filesystem, red, `os`, `io`, `package`,
`debug`, corrutinas, carga dinámica, reloj, entorno, procesos, azar y funciones
base como `load`, `dofile`, `rawset`, `getmetatable` o `setmetatable`.

La sintaxis del lenguaje sigue disponible: literales, variables locales,
operadores, tablas locales, condiciones, bucles y `return`. Eso basta para
combinar las consultas aprobadas sin abrir capacidades externas.

Cada tabla expuesta, incluida la del entorno global, es un proxy vacío. Sus
valores viven en una tabla de respaldo que el script no recibe; `__index` lee
esa tabla, `__newindex` siempre falla y `__metatable` protege el metatable. Este
patrón se aplica también a `mundo.fecha`. No basta con poner `__newindex` sobre
una tabla que ya contenga campos, porque Lua permitiría reemplazarlos sin llamar
al metamétodo.

El chunk se carga con el proxy global como entorno propio. Una variable `local`
puede llamarse `mundo` sin consecuencias fuera de la llamada y toda escritura
que atraviese uno de los proxies falla. Lua permite que el propio chunk reasigne
su upvalue léxico `_ENV`; hacerlo solo reemplaza su entorno local para el resto
de esa evaluación, pierde acceso a las capacidades aprobadas y no modifica el
proxy ni estado del host. Las funciones expuestas devuelven solo valores
primitivos o el objeto opaco de solicitud.

Lua se ejecuta dentro del proceso y estas restricciones no prometen aislamiento
adversarial equivalente a otro proceso. Sí constituyen el contrato soportado
para contenido y evitan acceso accidental a capacidades no aprobadas.

## 6. Presupuestos de ejecución

Los valores iniciales son constantes del módulo:

```rust
pub(crate) const LIMITE_MEMORIA_LUA_BYTES: usize = 2 * 1024 * 1024;
pub(crate) const INTERVALO_HOOK_LUA: u32 = 1_000;
pub(crate) const LIMITE_INSTRUCCIONES_LUA: u32 = 100_000;
```

Inmediatamente después de crear la máquina, y antes de crear el entorno o cargar
el bloque, se instala el límite total de memoria con `Lua::set_memory_limit`.
Durante una evaluación se instala antes de ejecutar el chunk un hook cada 1.000
instrucciones de la VM. El contador aumenta en 1.000 y aborta cuando alcanza
100.000; por tanto, una política no sigue ejecutándose después del centésimo
aviso. La compilación no ejecuta instrucciones, pero sí queda sujeta al límite de
memoria.

No se usa un plazo de reloj: el resultado no depende de la velocidad del equipo
ni de la terminal. Alcanzar cualquiera de los dos presupuestos cancela la llamada
sin solicitud ni cambio de Mundo. Los valores solo cambiarán mediante una nueva
decisión documentada y pruebas de rendimiento.

Si crear las tablas y funciones aprobadas excediera el presupuesto antes de
cargar el bloque, se clasificaría como `Infraestructura`; las pruebas garantizan
que el entorno base cabe. Un agotamiento al compilar o ejecutar el código se
clasifica como `LimiteMemoria`.

## 7. Errores y diagnósticos

La frontera no deja escapar `mlua::Error`, no usa `panic` para fallas de
contenido y devuelve esta estructura:

```rust
pub(crate) enum EtapaLua {
    Validacion,
    Preparacion,
    Ejecucion,
    ConversionRespuesta,
}

pub(crate) enum ClaseErrorLua {
    Sintaxis,
    LimiteInstrucciones,
    LimiteMemoria,
    ArgumentoInvalido,
    ClaveVariableGlobalInvalida,
    VariableGlobalInexistente,
    TipoVariableGlobalIncorrecto,
    CaracteristicaInvalida,
    ClaveLugarInvalida,
    LugarInexistente,
    RespuestaInvalida,
    Ejecucion,
    Infraestructura,
}

pub(crate) struct ErrorPoliticaLua {
    pub actor: IdActor,
    pub clave_actor: String,
    pub archivo_relativo: PathBuf,
    pub etapa: EtapaLua,
    pub clase: ClaseErrorLua,
    pub linea: Option<u32>,
    pub detalle: String,
}
```

Estos tipos implementan `Debug`, `Display` y `std::error::Error` donde
corresponda; sus campos, no el texto de `Display`, forman el contrato interno.
El chunk recibe como nombre virtual la ruta relativa seguida de `:politica`.
`linea` se completa solo cuando `mlua` ofrece esa posición sin analizar texto de
error no estable; el nombre virtual y la causa segura permanecen en `detalle`.

Todo control C0/C1 o escape que aparezca en un mensaje de Lua se representa de
forma escapada al construir `detalle`. La aplicación podrá proyectar el
diagnóstico a una vista, pero nunca enviará texto de Lua sin sanear a la terminal.

Las consultas globales traducen exactamente:

- `ErrorConsultaVariableGlobal::Inexistente` a
  `VariableGlobalInexistente`;
- `ErrorConsultaVariableGlobal::TipoIncompatible` a
  `TipoVariableGlobalIncorrecto`, conservando clave, tipo esperado y encontrado
  en el detalle.

Un error de sintaxis o memoria durante `validar_politica` invalida toda la carga
atómica. Un error que solo aparece al ejecutar no produce una solicitud, no
consume azar y no modifica ni adelanta el Mundo. La fase NPC es la única fuente
canónica para decidir si ese error deja continuar o vuelve fatal la sesión.

## 8. Invariantes

- Una política está ligada a exactamente un `IdActor` validado.
- Cada evaluación observa una instantánea y una máquina Lua nuevas.
- Lua nunca recibe una referencia mutable al Mundo ni al mapa.
- Una evaluación produce como máximo una solicitud.
- La identidad del Actor de la solicitud procede de la asociación Rust.
- Crear o devolver una solicitud no ejecuta sus reglas.
- Lua no crea, cambia ni elimina variables globales.
- Todo azar de gameplay permanece en la fuente de la Partida.
- Un fallo no deja estado Lua persistente ni cambios parciales.

## 9. Pruebas y aceptación

Las pruebas cubrirán:

- compilación válida sin ejecución durante la carga;
- error de sintaxis con Actor, archivo, etapa y clase correctos;
- condición falsa que devuelve `nil` y no solicita una acción;
- ejecución del bloque del fixture en enero desde `valle_fertil`, con resultado
  `nil`;
- ejecución del bloque en febrero desde `valle_fertil` y conversión al `Viajar`
  esperado hacia `zona_arida`;
- nueva ejecución en febrero desde `zona_arida`, con resultado `nil`;
- aceptación equivalente de ningún valor y de un solo `nil`;
- rechazo de varios valores, booleanos, cadenas, tablas y userdata ajeno;
- imposibilidad de indicar o reemplazar el Actor vinculado;
- resolución de destino y diagnósticos distintos ante clave inválida o Lugar
  inexistente;
- nueva validación de la solicitud por el motor, incluido un rechazo sin
  mutación;
- lectura de la fecha ya avanzada y numeración de meses `1..=12`;
- consulta correcta de una global booleana, entera y de texto;
- clave global añadida solo al fixture RON y leída sin declarar nada en Rust;
- diagnósticos distintos para clave global ausente, tipo incorrecto, argumento
  no textual, UTF-8 inválido y sintaxis de clave inválida;
- rechazo de cero o más de un argumento en cada función expuesta, sin ignorar
  valores adicionales;
- lectura de clave y Lugar del Actor y presencia o ausencia de una
  Característica;
- rechazo de una Característica mal formada;
- ausencia de bibliotecas, funciones base y capacidades del sistema;
- rechazo de escrituras en el entorno y en cada proxy anidado;
- reasignación local de `_ENV` sin mutación del host ni persistencia;
- imposibilidad de recuperar o reemplazar metatables;
- aislamiento de variables locales, tablas y objetos entre dos evaluaciones;
- interrupción determinista de un bucle por instrucciones;
- rechazo de una asignación que supere memoria;
- clasificación separada de un fallo del entorno base;
- saneamiento de controles en diagnósticos;
- igualdad de resultado para el mismo script y la misma instantánea;
- ausencia de mutación, avance temporal o consumo de azar ante cualquier fallo.

El componente queda aceptado cuando el fixture puede solicitar un `Viajar`
común sin que Lua pueda ejecutarlo, mutar estado, suplantar al Actor, persistir
datos propios ni acceder a capacidades externas.

## 10. Decisiones aplazadas

- consultas sobre otros Actores o Lugares;
- otras acciones generales;
- azar accesible directamente desde Lua;
- varias políticas por Actor y scripts compartidos;
- archivos `.lua` independientes;
- persistencia de estado interno de Lua;
- creación o modificación de variables globales durante una Partida mediante
  eventos narrativos u operaciones semánticas del núcleo;
- aislamiento adversarial en otro proceso.

## 11. Referencias técnicas

- [`mlua::Lua`](https://docs.rs/mlua/0.12.2/mlua/struct.Lua.html)
- [`mlua::StdLib`](https://docs.rs/mlua/0.12.2/mlua/struct.StdLib.html)
- [hooks de `mlua`](https://docs.rs/mlua/0.12.2/mlua/struct.HookTriggers.html)
