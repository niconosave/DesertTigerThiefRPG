# Workspace, entorno y límites de código

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.9 |
| Fecha | 2026-10-14 |

## 1. Propósito

Este documento desarrolla la organización física definida en el
[diseño técnico general](../diseno-tecnico-general.md). Establece cómo se divide
el código Rust, qué puede depender de qué y qué controles comunes debe superar
la workspace.

Su alcance comprende:

- la workspace virtual de Cargo;
- los tres crates iniciales y sus módulos de responsabilidad;
- el único ejecutable y su modelo de ejecución local;
- la administración común del toolchain, dependencias y lints;
- las comprobaciones estructurales y de calidad del proyecto.

Quedan fuera el diseño interno de las reglas, los esquemas de contenido, el
protocolo de acciones, Lua y la composición de la interfaz. Este documento sí
fija el toolchain y las versiones directas que materializan esos diseños; el
comportamiento de cada biblioteca permanece en su documento específico.

## 2. Estructura de la workspace

La raíz será una workspace virtual: su `Cargo.toml` no define un paquete propio,
sino que agrupa estos miembros:

```text
/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── crates/
│   ├── desert-tiger-core/
│   ├── desert-tiger-app/
│   └── desert-tiger-terminal/
├── assets/
│   └── terminal/
│       ├── scenes.ron
│       ├── tiles.ron
│       └── ui.ron
├── content/
└── docs/
```

Las versiones compartidas de dependencias, los lints y los metadatos comunes se
declararán en la raíz cuando Cargo permita heredarlos. Cada crate declarará solo
las dependencias que realmente utiliza. `Cargo.lock` se versionará porque la
workspace produce una aplicación ejecutable y debe poder reconstruir el mismo
conjunto de dependencias.

`rust-toolchain.toml` fijará Rust `1.99.0` con el perfil `minimal` y los
componentes `rustfmt` y `clippy`:

```toml
[toolchain]
channel = "1.99.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

Los tres paquetes usarán edición Rust `2024`. Mediante `[workspace.package]`
declararán `rust-version = "1.99.0"` y la workspace usará `resolver = "3"`. No
se utilizará nightly ni una característica inestable del compilador.

En la máquina Manjaro de desarrollo, la comprobación del 14 de octubre de 2026
encontró GCC `16.1.1` y `pkg-config` `2.5.1`, pero no encontró `rustc`, `cargo` ni
`rustup`. El primer plan de implementación deberá instalar `rustup`, dejar que
`rust-toolchain.toml` instale el toolchain fijado y comprobar esas versiones
antes de crear la workspace. La instalación no forma parte del diseño ni se
realiza implícitamente al leer estos documentos.

### Dependencias directas aprobadas

Las versiones iniciales se fijan exactamente en `[workspace.dependencies]` y
cada crate hereda solamente las que usa:

| Dependencia | Versión | Propietario | Características aprobadas | Uso |
|---|---:|---|---|---|
| `ratatui` | `0.30.2` | `terminal` | sin características predeterminadas; `crossterm_0_29` | composición y búfer de la interfaz |
| `crossterm` | `0.29.0` | `terminal` | sin características predeterminadas; `events` | teclado, redimensionado y ciclo de terminal |
| `unicode-width` | `0.2.2` | `app`, `terminal` | predeterminadas | validación y medición del ancho visible de texto |
| `serde` | `1.0.229` | `app`, `terminal` | `derive` | DTO privados de contenido y recursos visuales |
| `ron` | `0.12.2` | `app`, `terminal` | predeterminadas; `integer128` | archivos editables RON y diagnóstico de enteros fuera de `i64` |
| `mlua` | `0.12.2` | `app` | sin características predeterminadas; `lua54`, `vendored` | políticas NPC en Lua 5.4 embebido |
| `rand` | `0.10.3` | `app` | predeterminadas | semilla nueva desde el sistema y traits de azar |
| `rand_chacha` | `0.10.0` | `app` | predeterminadas | `ChaCha12Rng` poseído por la Partida |

`mlua` usa `vendored` para compilar y enlazar su propia Lua 5.4 en vez de depender
de un paquete Lua instalado globalmente. Esa compilación requiere el compilador C
ya disponible. No se habilitan `async`, `send`, módulos nativos, LuaJIT, Luau ni
serialización automática de valores Lua.

La característica `integer128` de RON permite que los DTO externos reciban un
entero como `i128`, informen con contexto que excede `i64` y recién después lo
conviertan al tipo reglado. No introduce enteros de 128 bits en el Mundo.

`ChaCha12Rng` ofrece una secuencia determinista y portable a partir de una
semilla. `desert-tiger-app` lo adaptará a `FuenteAzar`; el núcleo no dependerá de
`rand`. La fuente programada de pruebas tampoco usará estas bibliotecas.

Las restricciones exactas del manifiesto usarán `=`, por ejemplo
`ratatui = "=0.30.2"`. `Cargo.lock` fijará además todas las dependencias
transitivas. Una actualización requiere modificar este documento y regenerar el
lockfile mediante un plan explícito; no se acepta un cambio accidental producido
por `cargo update`.

La raíz materializará como mínimo este contrato; cada paquete añadirá
`[lints] workspace = true` y heredará sus metadatos y dependencias:

```toml
[workspace]
members = [
    "crates/desert-tiger-core",
    "crates/desert-tiger-app",
    "crates/desert-tiger-terminal",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.99.0"

[workspace.dependencies]
ratatui = { version = "=0.30.2", default-features = false, features = ["crossterm_0_29"] }
crossterm = { version = "=0.29.0", default-features = false, features = ["events"] }
unicode-width = "=0.2.2"
serde = { version = "=1.0.229", features = ["derive"] }
ron = { version = "=0.12.2", features = ["integer128"] }
mlua = { version = "=0.12.2", default-features = false, features = ["lua54", "vendored"] }
rand = "=0.10.3"
rand_chacha = "=0.10.0"

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
dbg_macro = "deny"
todo = "deny"
unimplemented = "deny"
```

El código propio no usará `unsafe`. Esto no impide compilar dependencias que lo
usen internamente. `-D warnings` se aplica desde el comando de Clippy para que
los lints normales del compilador y de Clippy tengan el mismo tratamiento en los
tres crates, sin activar por adelantado grupos opinativos que obliguen a agregar
excepciones sin relación con defectos reales.

## 3. Modelo de ejecución

Habrá un único binario: `desert-tiger-terminal`. Cargo compilará
`desert-tiger-app` y `desert-tiger-core` como bibliotecas y las enlazará dentro de
ese mismo binario.

Al jugar habrá un solo programa y un solo proceso. Las capas se comunicarán con
llamadas normales a funciones y tipos Rust. No forman servicios, no se inician
por separado y no usan HTTP, REST, sockets ni serialización de red.

La división en crates existe para hacer comprobable la separación del código y
permitir que otro ejecutable sustituya al frontend de terminal sin reescribir las
reglas.

## 4. Crates y módulos de responsabilidad

### `desert-tiger-core`

Es una biblioteca sin conocimiento del sistema de archivos, Lua ni interfaces.
Contendrá módulos de primer nivel para:

- el modelo y las consultas del mundo;
- las acciones generales y particulares;
- la fecha y la duración del mundo;
- los resultados semánticos producidos por las reglas.

Los nombres internos definitivos podrán precisarse en los documentos de esos
sistemas. La API pública del crate expondrá conceptos de dominio y mantendrá
privadas sus colecciones mutables e invariantes.

### `desert-tiger-app`

Es una biblioteca que compone el núcleo en una partida utilizable. Contendrá
módulos separados para:

- sesión y coordinación de fases;
- carga e instanciación de contenido;
- carga y resolución de la matriz codificada y su leyenda, y acceso al estado
  resultante del mapa, nombres, coordenadas y retratos de Actor;
- políticas de NPC e integración de Lua;
- fachada consumida por cualquier frontend.

Esta lista expresa propiedad de responsabilidades, no obliga a crear un archivo
por punto. Carga y Lua permanecerán como módulos internos mientras no exista una
necesidad demostrada de convertirlos en crates.

### `desert-tiger-terminal`

Es el único paquete con un binario. Su `main` se limitará a componer dependencias,
resolver la configuración del proceso, iniciar la aplicación, ejecutar el bucle
y restaurar la terminal. El resto del frontend se organizará como código
comprobable, con responsabilidades separadas para:

- traducir eventos físicos a intenciones;
- mantener el estado efímero de la interfaz;
- cargar y aplicar su paleta semántica de interfaz;
- asociar claves semánticas de escena con recursos artísticos propios;
- asociar valores `ClaveTipoTile` con glifos y estilos propios;
- dibujar con Ratatui;
- administrar Crossterm y el ciclo de vida de la terminal.

La estructura exacta de widgets, modos y archivos se decidirá en el documento
del frontend. Ratatui y Crossterm no podrán aparecer en las interfaces públicas
de `desert-tiger-app` ni en `desert-tiger-core`.

## 5. Dirección de dependencias

La única dirección interna permitida es:

```text
desert-tiger-terminal -> desert-tiger-app -> desert-tiger-core
```

Por lo tanto:

- `core` no depende de ningún otro crate del proyecto;
- `app` depende de `core`, pero no de `terminal`;
- `terminal` depende de `app` y no necesita depender directamente de `core`;
- no se permiten ciclos ni dependencias inversas;
- tipos de teclado, terminal, color, geometría visual o widgets no atraviesan la
  fachada de la aplicación;
- las capacidades externas, como filesystem y Lua, entran mediante adaptadores
  propiedad de `app`.

Una futura interfaz 2D o 3D será otro ejecutable que dependa de `app`. Si su
engine usa otro lenguaje, su adaptador o FFI reemplazará esa capa de entrada sin
trasladar las reglas fuera de `core`.

## 6. Entradas, salidas y configuración

La workspace recibe como entradas de construcción sus manifiestos, el toolchain
fijado, el código, los recursos propios de cada frontend y el contenido
empaquetado por el producto. Su salida inicial es el binario de terminal y sus
artefactos de prueba.

Las rutas de contenido, diagnósticos y opciones de ejecución pertenecen a la
composición de la aplicación, no al núcleo. No habrá estado global mutable para
compartir servicios entre crates. Las fuentes variables —azar, reloj real o
entrada— se entregarán a la capa que las consume de una forma sustituible en
pruebas.

El binario aceptará una opción de inicio con esta forma:

```text
desert-tiger-terminal [--content <ruta>]
```

`--content` es opcional. Si está ausente, se utiliza `content/base`. Tanto esa
ruta predeterminada como una ruta relativa explícita se resuelven respecto del
directorio de trabajo vigente al iniciar el proceso; una ruta absoluta se usa
directamente. El binario resolverá esta configuración antes de activar el modo
interactivo de la terminal y entregará la raíz resultante al cargador de
`desert-tiger-app`.

La ausencia completa de la opción activa el valor predeterminado. En cambio,
escribir `--content` sin valor, repetir la opción o indicar una ruta inexistente
es un error de configuración y nunca provoca una caída silenciosa a
`content/base`.

El catálogo de ilustraciones de viñeta, el catálogo de apariencias de tiles y la
paleta de interfaz son recursos propios del frontend. Se cargan respectivamente
desde `assets/terminal/scenes.ron`, `assets/terminal/tiles.ron` y
`assets/terminal/ui.ron`, también respecto del directorio de trabajo. No están
dentro del conjunto de contenido y `--content` no cambia sus ubicaciones. En el
primer incremento no habrá una opción para sustituirlos por otro tema. La
ubicación apropiada para una distribución instalada se decidirá junto con su
empaquetado; estas rutas son las normativas para el desarrollo local.

## 7. Contratos e invariantes

- La workspace compila como una unidad y posee un solo grafo de dependencias
  bloqueado.
- Solo `desert-tiger-terminal` produce el ejecutable jugable inicial.
- Los tres crates se ejecutan en un mismo proceso.
- Toda regla de gameplay reside en `core`; `app` coordina y `terminal` presenta.
- Una capa solo conoce a la capa situada inmediatamente debajo mediante su API
  pública.
- Ninguna dependencia nueva se incorpora sin una necesidad registrada en el
  documento técnico del componente que la utiliza.
- La API pública de cada biblioteca será la mínima necesaria; los módulos serán
  privados por defecto.
- El código de producción no depende del orden accidental de archivos, del reloj
  real ni de una terminal disponible para poder probar sus reglas.

## 8. Errores y validaciones

Los errores de configuración, construcción o inicialización deben incluir el
componente y la causa relevante. Un fallo esperado al leer archivos o preparar
la terminal se propagará como error; no se resolverá mediante `panic`, `unwrap` o
`expect` en un camino normal.

La aplicación no iniciará una partida parcial si falla una dependencia de
inicio. El frontend será responsable de restaurar la terminal tanto ante una
salida normal como ante un error propagado. La política detallada de cada fallo
pertenece al documento del componente que lo origina.

## 9. Pruebas y criterios de aceptación

Cada crate tendrá sus pruebas junto a la capa que verifican:

- `core`: pruebas unitarias sin filesystem, Lua ni terminal;
- `app`: pruebas de integración sin una terminal real;
- `terminal`: pruebas de entrada, estado de interfaz y búfer de renderizado,
  resolución de la opción `--content`, carga de sus recursos visuales, además de
  comprobaciones manuales en Manjaro.

La workspace deberá superar los controles establecidos por la
[metodología](../../metodologia-desarrollo.md): formato, compilación de todos los
targets, Clippy sin advertencias y todas las pruebas. También deberá comprobarse
una compilación de producción antes de una entrega.

Los comandos normativos son:

```text
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
cargo tree --duplicates
```

`cargo tree --duplicates` no puede mostrar dos versiones de Crossterm. Una
revisión adicional de `cargo tree -p desert-tiger-core` comprobará que el núcleo
no incorpora Ratatui, Crossterm, Serde, RON, `mlua`, `rand` ni `rand_chacha`.

Este diseño estará aceptado cuando Cargo reconozca los tres miembros, produzca un
único binario, ejecute las pruebas por workspace y el grafo de dependencias
respete `terminal -> app -> core`. Una prueba de `core` deberá poder ejecutarse
sin inicializar Ratatui, Crossterm, Lua ni leer contenido.

## 10. Preguntas bloqueantes

Ninguna en este componente.

## 11. Decisiones aplazadas

- plataformas adicionales a Manjaro Linux en los controles automatizados;
- ubicación predeterminada del contenido en una futura distribución instalada;
- perfiles de compilación adicionales al perfil de producción estándar;
- selección de otro catálogo o tema visual mediante una opción de inicio.

## 12. Referencias

- [Rust 1.99.0](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)
- [Formato de manifiestos de Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Ratatui 0.30.2](https://crates.io/crates/ratatui/0.30.2)
- [Crossterm 0.29.0](https://crates.io/crates/crossterm/0.29.0)
- [Serde 1.0.229](https://crates.io/crates/serde/1.0.229)
- [RON 0.12.2](https://crates.io/crates/ron/0.12.2)
- [`mlua` 0.12.2](https://crates.io/crates/mlua/0.12.2)
- [`unicode-width` 0.2.2](https://crates.io/crates/unicode-width/0.2.2)
- [`rand` 0.10.3](https://docs.rs/rand/0.10.3/rand/)
- [`rand_chacha` 0.10.0](https://docs.rs/rand_chacha/0.10.0/rand_chacha/)
