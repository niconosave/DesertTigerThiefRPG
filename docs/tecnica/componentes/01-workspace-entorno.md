# Workspace, entorno y límites de código

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.2 |
| Fecha | 2026-10-12 |

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
protocolo de acciones, Lua y la composición de la interfaz. Tampoco se fijan aquí
versiones exactas de Rust o de bibliotecas: deberán comprobarse juntas al crear
el entorno y quedar registradas en los manifiestos y archivos de bloqueo.

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
├── content/
└── docs/
```

Las versiones compartidas de dependencias, los lints y los metadatos comunes se
declararán en la raíz cuando Cargo permita heredarlos. Cada crate declarará solo
las dependencias que realmente utiliza. `Cargo.lock` se versionará porque la
workspace produce una aplicación ejecutable y debe poder reconstruir el mismo
conjunto de dependencias.

`rust-toolchain.toml` fijará el toolchain utilizado por el proyecto. La versión
se elegirá durante la preparación del entorno después de comprobar los requisitos
de las dependencias adoptadas y su funcionamiento en Manjaro. Este documento no
presupone una versión que todavía no haya sido verificada.

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
- acceso a definiciones de presentación y estado actual del mapa de la Partida;
- políticas de NPC e integración de Lua;
- fachada consumida por cualquier frontend.

Esta lista expresa propiedad de responsabilidades, no obliga a crear un archivo
por punto. Carga y Lua permanecerán como módulos internos mientras no exista una
necesidad demostrada de convertirlos en crates.

### `desert-tiger-terminal`

Es el único paquete con un binario. Su `main` se limitará a componer dependencias,
iniciar la aplicación, ejecutar el bucle y restaurar la terminal. El resto del
frontend se organizará como código comprobable, con responsabilidades separadas
para:

- traducir eventos físicos a intenciones;
- mantener el estado efímero de la interfaz;
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
fijado, el código y el contenido empaquetado por el producto. Su salida inicial
es el binario de terminal y sus artefactos de prueba.

Las rutas de contenido, diagnósticos y opciones de ejecución pertenecen a la
composición de la aplicación, no al núcleo. No habrá estado global mutable para
compartir servicios entre crates. Las fuentes variables —azar, reloj real o
entrada— se entregarán a la capa que las consume de una forma sustituible en
pruebas.

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
  además de
  comprobaciones manuales en Manjaro.

La workspace deberá superar los controles establecidos por la
[metodología](../../metodologia-desarrollo.md): formato, compilación de todos los
targets, Clippy sin advertencias y todas las pruebas. También deberá comprobarse
una compilación de producción antes de una entrega.

Este diseño estará aceptado cuando Cargo reconozca los tres miembros, produzca un
único binario, ejecute las pruebas por workspace y el grafo de dependencias
respete `terminal -> app -> core`. Una prueba de `core` deberá poder ejecutarse
sin inicializar Ratatui, Crossterm, Lua ni leer contenido.

## 10. Preguntas pendientes

- ¿Qué versión exacta de Rust, edición y resolver resultan compatibles con las
  versiones finalmente elegidas de las dependencias?
- ¿Qué versiones exactas de Ratatui, Crossterm y `mlua` se bloquearán al crear la
  workspace?
- ¿Qué plataformas, además de Manjaro Linux, se incluirán inicialmente en los
  controles automatizados?
- ¿Será necesario definir perfiles de compilación adicionales al perfil de
  producción estándar?
