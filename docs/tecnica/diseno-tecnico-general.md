# Desert Tiger Thief RPG — Diseño técnico general

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.2 |
| Fecha | 2026-10-12 |

## 1. Propósito

Este documento traduce el [diseño general](../diseno-general.md) a una estructura
técnica global. Define las fronteras del código, la dirección de sus dependencias
y los contratos que compartirán los sistemas.

No define structs, esquemas de archivos, reglas RPG ni pantallas concretas. Esos
detalles pertenecerán a los documentos de
[`componentes/`](./componentes/README.md), siguiendo la
[metodología de desarrollo](../metodologia-desarrollo.md).

## 2. Criterios de la arquitectura

La estructura debe garantizar que:

- las reglas puedan ejecutarse sin terminal ni interfaz gráfica;
- los tiles y las coordenadas visuales nunca determinen el gameplay;
- jugador y NPC utilicen el mismo motor de acciones;
- el contenido editable produzca instancias nuevas al comenzar una partida;
- Lua solicite acciones, pero no modifique el mundo directamente;
- una acción pueda esperar una decisión y continuar después;
- azar, entrada y tiempo real puedan controlarse en pruebas;
- otro frontend pueda reutilizar la aplicación y las reglas.

La primera arquitectura será pequeña. No necesita ECS, event sourcing, base de
datos, asincronía ni multihilo. Su adopción futura requerirá una necesidad
comprobada y un cambio documentado.

## 3. Workspace y dirección de dependencias

El proyecto será una workspace virtual de Cargo con tres crates:

```text
desert-tiger-terminal  ─────►  desert-tiger-app  ─────►  desert-tiger-core
Ratatui y Crossterm            sesión y adaptadores          reglas
```

Esta será la única dirección permitida para las dependencias internas.

### Modelo de ejecución

La primera versión será **un único programa local ejecutado en un solo proceso**.
`desert-tiger-terminal` será el ejecutable; `desert-tiger-app` y
`desert-tiger-core` serán bibliotecas Rust compiladas y enlazadas dentro de ese
mismo ejecutable.

Las flechas anteriores representan dependencias de código y llamadas directas a
funciones dentro del proceso. No representan programas independientes ni habrá
servidor, REST, HTTP, sockets o comunicación entre procesos. En este documento,
“API” significa solamente la interfaz de tipos y funciones Rust que una parte
del código ofrece a otra.

### `desert-tiger-core`

Biblioteca Rust que contiene:

- el estado del mundo sujeto a las reglas;
- sus invariantes y consultas;
- las acciones generales y particulares;
- el estado reanudable de una acción pendiente;
- el cálculo de resultados, fragmentos narrativos neutrales y duración.

Es la única autoridad que valida y modifica el estado reglado, incluida la
fecha. No depende de filesystem, formatos de contenido, Lua, Ratatui, Crossterm,
teclas, colores ni reloj del sistema. El azar se recibe mediante una entrada que
pueda sustituirse en pruebas.

### `desert-tiger-app`

Biblioteca que ofrece una fachada independiente del frontend:

- carga y valida contenido;
- crea y posee la sesión activa;
- coordina las fases del jugador, avance temporal y NPC;
- integra Lua como fuente de decisiones;
- conserva los datos de presentación;
- expone comandos, salidas narrativas y vistas de solo lectura.

La aplicación coordina operaciones, pero toda mutación del mundo pasa por el
núcleo. Su API pública usa valores poseídos y tipos propios o reexportados por
`desert-tiger-app`, de modo que un frontend nunca necesite depender directamente
de `desert-tiger-core`.

La carga de contenido y Lua comenzarán como módulos internos de este crate. Solo
se extraerán a crates separados si aparece una razón concreta.

### `desert-tiger-terminal`

Ejecutable que:

- compone e inicia la aplicación;
- administra la terminal con Crossterm;
- dibuja con Ratatui;
- mantiene modos, foco, cámara y animación;
- convierte eventos y selecciones en comandos semánticos;
- decide cómo presentar fragmentos narrativos y resúmenes.

No calcula reglas ni modifica el mundo. Su punto de entrada será pequeño y la
lógica de interfaz quedará en módulos comprobables sin una terminal real.

Un frontend Rust alternativo podrá sustituir este crate y enlazar las mismas
bibliotecas en su propio ejecutable. Un engine que use otro lenguaje necesitará
un adaptador o FFI para incorporar esas bibliotecas en su proceso, pero no deberá
reimplementar las reglas ni ejecutar el frontend de terminal simultáneamente.

## 4. Estructura del repositorio

```text
/
├── Cargo.toml                 workspace virtual
├── Cargo.lock                 dependencias reproducibles
├── rust-toolchain.toml        toolchain fijado
├── crates/
│   ├── desert-tiger-core/
│   ├── desert-tiger-app/
│   └── desert-tiger-terminal/
├── content/                   archivos editables del juego
└── docs/                      diseño, técnica y planes
```

La workspace centralizará versiones de dependencias y lints. Se conservará
`Cargo.lock` porque el producto final es una aplicación. La versión exacta de
Rust y la estructura interna de cada crate se definirán al diseñar el entorno de
desarrollo.

## 5. Sesión, estado y presentación

La sesión de juego representará técnicamente la **Partida** del diseño general.
Poseerá la proyección reglada del Mundo, el contenido de presentación, las
políticas de NPC, la acción pendiente y la fuente de azar. Su nombre y sus tipos
concretos se definirán en los documentos específicos.

El estado reglado usará identificadores estables y tipados para referenciar sus
elementos. Todos los actores compartirán el mismo tipo; un identificador marcará
al actor jugador. La ubicación de cada actor tendrá una sola fuente de verdad:
su lugar. El grupo conservará referencias a sus integrantes y el núcleo
garantizará que permanezcan siempre en el mismo lugar y se muevan juntos.

La Partida contendrá exactamente un Mundo y este estará ligado obligatoriamente
a un único mapa visual. El núcleo solo conservará una referencia opaca a ese
mapa: sus tiles, las coordenadas de rótulos y el arte ASCII estarán en un
catálogo de presentación administrado por `desert-tiger-app`. Las políticas Lua
también estarán asociadas por identificador con sus actores. El frontend
recibirá una vista que combina estos datos sin exponer el estado mutable.

Así se respeta que mapa, lugares y actores pertenecen al mismo mundo, pero se
impide que viajar, atacar u otra regla consulte accidentalmente una coordenada
visual.

## 6. Carga de contenido

Los archivos se deserializarán a tipos de entrada separados de los tipos del
núcleo. Luego se validarán campos, identificadores, referencias y scripts antes
de convertirlos en contenido apto para crear una partida.

La carga será atómica: cualquier error impedirá crear una sesión parcial y
producirá un diagnóstico con contexto. Los archivos fuente permanecerán
inmutables durante la partida. Dos sesiones creadas desde el mismo contenido
comenzarán iguales y tendrán estados mutables independientes; la creación
inicial no introducirá azar.

El formato, esquema, distribución de `content/` y representación concreta de
referencias se decidirán en el documento técnico de contenido.

## 7. Protocolo y acciones

La frontera entre frontend y aplicación será una interfaz interna de Rust con
significado propio del juego. Mediante llamadas directas, el frontend enviará
comandos como comenzar una acción o responder una decisión, identificando
acciones, actores, objetivos y opciones; nunca enviará teclas ni coordenadas de
tiles al núcleo.

La aplicación devolverá:

- vistas de solo lectura derivadas del estado real;
- fragmentos narrativos y resultados neutrales;
- solicitudes de decisión con opciones identificadas;
- rechazos y resúmenes de la fase de NPC.

El frontend decide si un fragmento se muestra como viñeta, barra u otra forma.
El texto inicial puede estar escrito en el backend, pero no incluirá color,
posición ni instrucciones de dibujo.

La aplicación ofrecerá acciones y objetivos disponibles para ayudar a construir
la interfaz, pero el núcleo volverá a validar cada comando. Si el actor jugador
posee la característica `muerto`, rechazará el inicio de nuevas acciones sin
convertirlo todavía en una derrota formal.

Una acción general será una máquina de estados reanudable. El núcleo definirá
su estado y la sesión de aplicación lo poseerá mientras espera una respuesta.
Cada transición podrá emitir resultados, solicitar una decisión, completarse
con una duración o rechazar una entrada. Una respuesta inválida conservará la
acción pendiente y no modificará el mundo.

Las acciones particulares serán operaciones reutilizables llamadas por la
acción general. Jugador y NPC producirán el mismo tipo de solicitud y pasarán
por la misma validación. Solo cambia quién resuelve cada decisión intermedia: el
frontend para el jugador y el azar controlado para un NPC en la primera versión.

Los modos visuales pertenecen al frontend. La aplicación controla la fase de la
partida y el núcleo valida y continúa la acción pendiente.

## 8. Tiempo, NPC y Lua

Al terminar una acción del jugador:

1. el núcleo informa su duración y aplica el avance de fecha solicitado por la
   aplicación;
2. la aplicación consulta las políticas Lua de los NPC;
3. cada solicitud aceptada recorre el motor común, sin volver a avanzar la fecha;
4. la aplicación entrega resúmenes al frontend y devuelve el control al jugador.

Se usará `mlua` con un runtime de Lua `vendored`, para no depender de la versión
instalada en Manjaro. La versión exacta de Lua se fijará en su documento técnico
porque formará parte de la compatibilidad de los scripts.

Cada política funcionará conceptualmente como una transformación de un contexto
de solo lectura a ninguna acción o una solicitud declarativa. Lua no recibirá el
mundo mutable; Rust convertirá y validará su respuesta antes de ejecutarla. Todo
estado persistente deberá vivir en el Mundo y el azar será proporcionado por
Rust, de modo que las pruebas puedan reproducirlo.

El entorno permitirá únicamente capacidades necesarias. El documento específico
definirá su API, bibliotecas disponibles, límites de ejecución y política ante
errores. También definirá el orden de NPC, qué cambios observa cada uno y cuántas
acciones puede solicitar durante una fase.

## 9. Frontend de terminal

Se adopta **Ratatui con Crossterm**. Ratatui se ajusta al mapa de celdas, las
capas de nombres y selección, los paneles, los retratos y las barras, y permite
probar el resultado en un búfer. Crossterm proporciona entrada, redimensionado,
modo raw y pantalla alternativa. No hay una razón específica de Manjaro para
descartar esta combinación.

Usar solo Crossterm obligaría a reconstruir búferes, composición y widgets;
Cursive está más orientado a árboles de vistas y formularios; el modelo de
componentes de iocraft no aporta por ahora una ventaja que compense su mayor
complejidad para este juego.

Ratatui y Crossterm existirán exclusivamente en `desert-tiger-terminal`. Su
bucle inicial será síncrono y monohilo, separando:

1. traducción de eventos;
2. actualización del estado visual;
3. renderizado.

El reloj usado para animar no avanza el tiempo del mundo. El frontend deberá
responder a cambios de tamaño y restaurar la terminal al salir o fallar. Sus
versiones compatibles y el diseño exacto de widgets, modos y composición se
fijarán en el documento específico.

## 10. Verificación de la arquitectura

Además de las pruebas definidas por la metodología:

- el núcleo tendrá pruebas de reglas sin filesystem, Lua ni frontend;
- la aplicación tendrá pruebas sin terminal para carga y para el ciclo completo
  jugador–tiempo–NPC;
- el frontend probará entrada, modos y renderizado con eventos sintéticos,
  búferes y `TestBackend`;
- las pruebas manuales verificarán aspecto, animación, controles,
  redimensionado y restauración en la terminal de Manjaro.

Este diseño se considerará respetado cuando:

- el grafo de dependencias siga únicamente `terminal → app → core`;
- ninguna regla pueda acceder a tipos de terminal ni a datos del mapa visual;
- una prueba sin frontend pueda completar una acción, avanzar la fecha, procesar
  NPC y devolver el control;
- jugador y NPC atraviesen el mismo motor de acciones;
- contenido inválido no produzca una partida parcial.

## 11. Documentos técnicos derivados

Se crearán solo cuando el siguiente incremento los necesite:

1. workspace, toolchain y módulos;
2. modelo del mundo, identificadores e invariantes;
3. formato, carga e instanciación de contenido;
4. protocolo y motor de acciones;
5. tiempo, calendario y fase de NPC;
6. integración de Lua;
7. frontend Ratatui.

## 12. Decisiones aplazadas

Quedan para sus documentos específicos:

- structs, APIs, formatos y esquemas concretos;
- reglas, textos y duraciones de las acciones;
- calendario exacto y orden de NPC;
- API, versión y recuperación de errores de Lua;
- teclas, colores, medidas y modos detallados;
- guardado y compatibilidad entre versiones de contenido;
- cualquier adopción futura de más crates, ECS, asincronía o multihilo.

## 13. Referencias técnicas

- [Workspaces — Cargo Book](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo.toml y Cargo.lock — Cargo Book](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Backends de Ratatui](https://ratatui.rs/concepts/backends/)
- [Comparación de backends de Ratatui](https://ratatui.rs/concepts/backends/comparison/)
- [`TestBackend` de Ratatui](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html)
- [`mlua`: versiones de Lua y compilación vendored](https://github.com/mlua-rs/mlua)
- [Cursive](https://docs.rs/cursive/latest/cursive/)
- [iocraft](https://docs.rs/iocraft/latest/iocraft/)
