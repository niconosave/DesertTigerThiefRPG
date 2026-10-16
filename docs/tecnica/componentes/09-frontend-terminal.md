# Frontend de terminal con Ratatui

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.29 |
| Fecha | 2026-10-15 |

## 1. Propósito

Este documento es la fuente canónica del frontend inicial que presentará el
juego dentro de una terminal de Linux. Desarrolla la sección de frontend del
[diseño general](../../diseno-general.md) y respeta los límites del
[diseño técnico general](../diseno-tecnico-general.md).

Su responsabilidad es transformar vistas y resultados de la aplicación en una
experiencia visual, y transformar la entrada del usuario en intenciones
semánticas.
No contiene ni deduce reglas del juego.

## 2. Alcance

Incluye:

- ciclo de vida de la terminal;
- recepción de teclado y redimensionado;
- estado y transiciones visuales;
- composición del mapa, rótulos, retratos y barras;
- presentación sucesiva de fragmentos narrativos;
- animación basada en tiempo real;
- traducción entre entrada física e intenciones de `desert-tiger-app`.

El primer incremento cubrirá la vista del mundo, selección de destino de
`Viajar`, presentación de sus viñetas y decisiones, continuación hasta la fase
NPC y regreso a la vista del mundo con un resumen breve en la barra contextual.
La entrada será solo por teclado.

No incluye reglas de acciones, validación de objetivos, decisiones de NPC,
avance de fecha, carga de contenido ni una abstracción universal para engines
gráficos.

## 3. Bibliotecas y modelo de ejecución

Se usarán Ratatui para componer y dibujar la interfaz y Crossterm para eventos,
modo raw, pantalla alternativa, cursor y tamaño de terminal. Ambas dependencias
existirán exclusivamente en `desert-tiger-terminal`.

El frontend, la aplicación y el núcleo se ejecutarán en un solo proceso. Los
intenciones y resultados serán llamadas y valores Rust; no habrá red, REST ni
comunicación entre procesos. `desert-tiger-terminal` dependerá únicamente de la
interfaz pública de `desert-tiger-app`.

Las versiones se fijarán juntas en la workspace para evitar dos versiones
incompatibles de Crossterm. La versión exacta se registrará en `Cargo.toml` y
`Cargo.lock` al preparar el entorno.

El mismo paquete contiene el binario no interactivo de autoría
`importar-retrato-durdraw`. Usa `flate2` y `serde_json` para leer un `.dur`, pero
esas dependencias no atraviesan la fachada ni intervienen en el ejecutable del
juego. La herramienta imprime un fragmento RON y no abre ni modifica contenido.

## 4. Responsabilidades internas

El crate separará conceptualmente estas partes, sin convertirlas en nuevos
servicios:

- **ciclo de terminal:** entrar y salir de modo raw y pantalla alternativa;
- **bucle de eventos:** combinar entrada, redimensionado y vencimientos de
  animación;
- **adaptador de entrada:** convertir teclas en intenciones según el modo visual;
- **estado de interfaz:** conservar modo, foco, cámara y fotograma de animación;
- **catálogo visual:** cargar la paleta semántica, asociar claves de tipo de tile
  con glifos y estilos, y claves semánticas de escena con arte ASCII propio del
  frontend;
- **composición:** distribuir las regiones de la pantalla;
- **renderizado:** dibujar cada región a partir de la vista de aplicación.

Los tipos `Frame`, `Rect`, `Style`, `KeyCode` y demás tipos de Ratatui o
Crossterm no atravesarán el límite hacia `desert-tiger-app`.

## 5. Contrato con la aplicación

La forma normativa de las intenciones, fases, vistas y errores se define en
[`04-sesion-aplicacion.md`](./04-sesion-aplicacion.md). Este documento define
únicamente cómo la terminal las traduce y representa.

El frontend recibirá valores poseídos que describan:

- una vista de solo lectura del estado vigente del Mundo y una `VistaMapa` con
  dimensiones, claves por celda y todos los tipos declarados para esa Partida;
- acciones y objetivos ofrecidos al jugador;
- fragmentos narrativos y resultados;
- decisiones pendientes con opciones identificadas;
- resúmenes de las acciones de NPC;
- rechazos o errores recuperables que deban comunicarse.

La aplicación entrega un fragmento narrativo semántico por vez, con texto y una
`Option<ClaveEscena>`. Las decisiones pueden incluir la misma clase de clave. La
fachada nunca entrega arte ni rutas. El frontend convierte cada elemento en una
viñeta visual y puede paginarlo si no cabe, sin solicitar el siguiente fragmento
hasta haber mostrado el actual completo. De ese modo, el backend fija texto,
orden, claves semánticas y límites de decisiones, mientras la terminal conserva
el control del marco, tamaño, paginación, color, arte y composición de cada
pantalla.

Los resúmenes NPC llegan como entradas estructuradas en orden estable. La
terminal las convierte en un texto breve para la barra contextual. Puede elegir
su redacción y recorte, pero no inventar consecuencias ni deducir nuevas reglas.

El frontend no elige qué actor actúa. Mover el foco entre objetivos es una
operación visual local; al confirmar, envía comenzar una acción con el ID del
objetivo o responder una opción con su ID. Nunca envía una coordenada visual
para expresar el destino de un viaje.

La aplicación ofrece posibilidades para construir la interfaz y vuelve a
validar toda solicitud. El frontend no interpretará por sí mismo características
como `inalcanzable` o `muerto` para autorizar una acción.

`desert-tiger-terminal` tampoco decide cómo un resultado del mundo transforma
los tiles vigentes durante la Partida. `desert-tiger-app` posee ese estado del
mapa y el frontend se limita a dibujar la vista que recibe.

La terminal resuelve cada `ClaveTipoTile` mediante su catálogo visual privado.
La apariencia resultante contiene como mínimo un glifo ASCII de una celda y un
estilo de Ratatui. La terminal no recibe ni reconstruye la leyenda del archivo de
Mapa y nunca usa su símbolo de autoría como glifo implícito. Un frontend 2D o 3D
podrá asociar la misma clave con un recurso completamente distinto. La lista de
tipos declarados permite comprobar o preparar esas asociaciones aunque un tipo
todavía no aparezca en la matriz vigente.

### Catálogo de apariencias de tile

La fuente del catálogo inicial es `assets/terminal/tiles.ron`, resuelta respecto
del directorio de trabajo. Es un recurso editable de
`desert-tiger-terminal`, no un archivo del Mundo: cambiar `--content` no lo
reemplaza y `desert-tiger-app` nunca lo abre. La terminal lo lee y valida antes
de activar el modo raw. El archivo se conserva separado del catálogo de
ilustraciones de viñeta.

El RON contiene exactamente dos partes conceptuales:

- `reserva`: una apariencia obligatoria cuyo glifo es `?`; su estilo se declara
  directamente en `tiles.ron`, no referencia `ui.ron` y se decidirá con el
  setting;
- `apariencias`: un mapa de `ClaveTipoTile` a una apariencia compuesta por un
  glifo y un estilo terminal.

La forma del archivo será esta; los valores concretos solo ilustran el esquema y
no fijan todavía la paleta artística del juego:

```ron
(
    reserva: (
        glifo: '?',
        estilo: (
            frente: "blanco",
            fondo: Some("negro"),
            modificadores: ["invertido"],
        ),
    ),
    apariencias: {
        "arena": (
            glifo: '.',
            estilo: (
                frente: "amarillo",
                fondo: None,
                modificadores: [],
            ),
        ),
        "bosque": (
            glifo: 'T',
            estilo: (
                frente: "verde",
                fondo: None,
                modificadores: ["negrita"],
            ),
        ),
    },
)
```

Las claves siguen la gramática `lowerCamelCase` definida por contenido. Cada
glifo es un único `char` ASCII imprimible o un espacio y ocupa una celda. El
estilo se declara directamente dentro de cada apariencia y no referencia otro
nombre: `frente` es obligatorio, `fondo` es `None` o `Some(color)` y
`modificadores` es una lista sin duplicados cuyo orden no cambia el resultado.

Los únicos colores admitidos son los dieciséis colores ANSI nombrados. La
conversión normativa es:

| Valor RON | `ratatui::style::Color` |
|---|---|
| `negro` | `Black` |
| `rojo` | `Red` |
| `verde` | `Green` |
| `amarillo` | `Yellow` |
| `azul` | `Blue` |
| `magenta` | `Magenta` |
| `cian` | `Cyan` |
| `gris` | `Gray` |
| `grisOscuro` | `DarkGray` |
| `rojoClaro` | `LightRed` |
| `verdeClaro` | `LightGreen` |
| `amarilloClaro` | `LightYellow` |
| `azulClaro` | `LightBlue` |
| `magentaClaro` | `LightMagenta` |
| `cianClaro` | `LightCyan` |
| `blanco` | `White` |

No se aceptan alias, traducciones alternativas ni diferencias de mayúsculas. En
particular, `gris` representa el color ANSI base `Gray` y `blanco` el brillante
`White`; `negrita` nunca se interpreta como una solicitud de color claro.

Los únicos modificadores admitidos son `negrita`, `tenue` e `invertido`, que se
convierten respectivamente en `BOLD`, `DIM` y `REVERSED`. `negrita` y `tenue` no
pueden aparecer juntas porque expresan intensidades contradictorias;
`invertido` puede combinarse con cualquiera de las dos. Cuando `fondo` es
`None`, el estilo no sustituye el fondo de la capa existente e `invertido` usa
ese fondo heredado. No se admiten colores RGB o indexados, `Reset`, color de
subrayado, parpadeo, texto oculto, cursiva, subrayado ni tachado en el primer
incremento.

Estos datos nunca contienen secuencias ANSI o de control escritas libremente. Un
DTO privado los valida y después construye `ratatui::style::Style`; el RON no
deserializa directamente tipos de Ratatui. El foco, la selección u otras capas
de interfaz pueden aplicar después su propio estilo y no se codifican como parte
del tile base.

La carga rechaza claves duplicadas, claves con sintaxis inválida, una reserva
ausente o cuyo glifo no sea `?`, glifos inválidos y estilos no reconocidos. Una
entrada del catálogo que el contenido vigente no utilice es válida y varias
claves pueden compartir la misma apariencia. La deserialización debe detectar
una clave repetida antes de construir el mapa interno, sin permitir que la
última sobrescriba silenciosamente a la anterior. En cambio, que una
`ClaveTipoTile` declarada por un contenido modificado no tenga entrada no
invalida el catálogo ni la Partida: cada celda de ese tipo se dibuja con la
apariencia `reserva`, sin `panic` y sin convertirlo en un error de gameplay.

El contenido oficial sigue una regla más estricta de calidad: una prueba de
integración exige que todos sus tipos declarados, incluidos los que aún no estén
presentes en la matriz, tengan una entrada explícita. Por tanto, la reserva
protege la compatibilidad con modificaciones, pero no oculta una omisión del
arte oficial.

### Paleta semántica de interfaz

La interfaz que rodea al mapa usa una paleta separada en
`assets/terminal/ui.ron`. Es otro recurso privado y editable de la terminal,
resuelto respecto del directorio de trabajo y validado antes de activar el modo
raw. No contiene apariencias de tiles ni ilustraciones y `--content` no lo
reemplaza.

El archivo declara una estructura fija con todos estos roles obligatorios y los
valores aprobados:

```ron
(
    pantalla: (
        frente: "gris",
        fondo: Some("negro"),
        modificadores: [],
    ),
    texto_normal: (
        frente: "gris",
        fondo: None,
        modificadores: [],
    ),
    destacado: (
        frente: "blanco",
        fondo: None,
        modificadores: ["negrita"],
    ),
    secundario: (
        frente: "grisOscuro",
        fondo: None,
        modificadores: [],
    ),
    marco: (
        frente: "cian",
        fondo: None,
        modificadores: [],
    ),
    titulo: (
        frente: "cianClaro",
        fondo: None,
        modificadores: ["negrita"],
    ),
    foco: (
        frente: "negro",
        fondo: Some("cianClaro"),
        modificadores: ["negrita"],
    ),
    atajo: (
        frente: "amarilloClaro",
        fondo: None,
        modificadores: ["negrita"],
    ),
    error: (
        frente: "blanco",
        fondo: Some("rojo"),
        modificadores: ["negrita"],
    ),
    deshabilitado: (
        frente: "gris",
        fondo: None,
        modificadores: ["tenue"],
    ),
    barra_contextual: (
        frente: "blanco",
        fondo: Some("grisOscuro"),
        modificadores: [],
    ),
    barra_acciones: (
        frente: "blanco",
        fondo: Some("azul"),
        modificadores: [],
    ),
)
```

Cada campo usa el mismo DTO de estilo y las mismas conversiones cerradas que el
catálogo de tiles. Ningún rol es opcional. Se rechazan campos ausentes,
duplicados o desconocidos para que un error tipográfico no produzca un valor por
defecto silencioso. `pantalla` rellena todo el búfer al comenzar cada fotograma;
en los demás roles, `fondo: None` conserva el fondo de la capa sobre la que se
aplican.

Los roles describen función y no widgets concretos: `destacado` señala
información activa importante, se aplica inicialmente a los caracteres visibles
de los retratos y no sustituye a `foco`; `marco` y `titulo` pertenecen tanto a
viñetas como a otros paneles; `barra_contextual` y `barra_acciones` rellenan por
completo sus respectivas franjas. Los colores de los tipos de tile no forman
parte de esta paleta y se elegirán al diseñar el setting.

### Catálogo de ilustraciones de viñeta

Todas las ilustraciones ASCII propias de las viñetas se agrupan en
`assets/terminal/scenes.ron`. Es un único recurso editable y privado de
`desert-tiger-terminal`, resuelto respecto del directorio de trabajo y cargado
antes de activar el modo raw. No pertenece a `content/`, no cambia con
`--content` y no se crean archivos de arte separados por escena.

Su forma es una lista para que el cargador pueda diagnosticar duplicados antes
de construir el índice interno:

```ron
(
    ilustraciones: [
        (
            // Ejemplo real; el arte todavía es solamente esquemático.
            escena: "viajeMarcha",
            arte: r#"
+----+
| ** |
+----+
"#,
        ),
    ],
)
```

`escena` usa el nombre externo `lowerCamelCase` asignado a una variante cerrada
de `ClaveEscena`. Las conversiones aprobadas inicialmente son:

| Variante Rust recibida | Nombre exacto en `scenes.ron` |
|---|---|
| `ClaveEscena::ViajeMarcha` | `viajeMarcha` |
| `ClaveEscena::ViajeObstaculo` | `viajeObstaculo` |
| `ClaveEscena::ViajeRumboIncierto` | `viajeRumboIncierto` |
| `ClaveEscena::ViajeLlegada` | `viajeLlegada` |

Este documento es la autoridad de los nombres externos y de la conversión. El
documento de cada acción conserva la autoridad sobre el significado de la
variante y sobre qué fragmentos la usan. El adaptador del frontend convierte de
forma exhaustiva entre el enum y estos nombres exactos, sensibles a mayúsculas;
no admite alias, variantes en `snake_case` ni otras grafías. Cada nueva variante
del enum exige agregar aquí una conversión antes de que el frontend pueda
compilar. Esa conversión define todo el vocabulario reconocido, pero
`scenes.ron` puede contener cualquier subconjunto de él, incluido ninguno: la
ausencia de una entrada significa que esa escena se presenta solamente con
texto. Una clave desconocida o repetida dentro del archivo es un error de
inicialización. El orden de las entradas no afecta el resultado.

`arte` es un bloque estático, rectangular, no vacío, de caracteres ASCII
imprimibles y espacios. Los saltos de línea separan filas; no se permiten
tabulaciones, controles ni secuencias de escape. Todas las filas tienen el mismo
ancho y los espacios finales cuentan. Para permitir la escritura multilínea
cómoda, el cargador elimina como máximo un salto de línea inmediatamente después
de abrir la cadena raw y otro inmediatamente antes de cerrarla, normalizando
previamente CRLF a LF; no recorta ningún otro carácter.

El catálogo no contiene texto narrativo, rutas, reglas, condiciones, tiempos,
fotogramas ni estilos. Una `ClaveEscena` sin entrada es válida y produce una
viñeta de solo texto; en cambio, omitir o invalidar el archivo completo es un
error de empaquetado. Esta diferencia permite decidir expresamente qué escenas
no tienen arte sin ocultar la pérdida accidental de todo el recurso.

## 6. Modos visuales iniciales

Los modos son estado de interfaz y no sustituyen la fase mantenida por la
aplicación:

| Modo | Responsabilidad |
|---|---|
| Vista del mundo | Mostrar el mundo y permitir elegir una acción disponible. |
| Selección de destino | Enfocar lugares ofrecidos para `Viajar` y confirmar uno. |
| Presentación de acción | Superponer a la última vista del mundo una ventana para cada fragmento narrativo y mostrarlas en sucesión. |
| Decisión | Mantener la ventana superpuesta, presentar el contexto de una decisión pendiente y enviar la opción elegida. |
| Confirmación de salida | Suspender el modo visual vigente y confirmar una salida solicitada por el jugador sin enviar una intención a la aplicación. |
| Terminal insuficiente | Explicar que la ventana debe ampliarse sin enviar intenciones de juego. |

Una transición visual nunca creará ni completará por sí sola una acción. Fuera
de las instantáneas conservadas expresamente para presentar una acción o una
confirmación, el frontend reconstruye el estado semántico desde la vista y fase
de la aplicación; nunca lo infiere de los píxeles que había dibujado.

## 7. Composición de la pantalla

La vista principal ocupará las dimensiones disponibles y respetará este orden
conceptual:

1. área del mapa ASCII, que ocupa toda la región situada sobre las barras;
2. nombres de lugares y resaltados dibujados sobre el mapa;
3. capa de retratos animados del jugador y su grupo, superpuesta a la parte
   inferior izquierda del mapa sin reservar una franja propia;
4. barra contextual con el mensaje correspondiente al estado actual;
5. barra de acciones, en el borde inferior, con los comandos disponibles y una
   indicación visible `[X]` para cada atajo alfabético activo.

Estos son los nombres canónicos en toda la documentación: **barra contextual**
para la que cambia su mensaje y **barra de acciones** para la franja inferior que
permite actuar sobre el modo vigente. `Acción` en este nombre de interfaz abarca
acciones generales, opciones y controles visuales; no significa que todo elemento
de la barra sea una `AcciónGeneral` del dominio. No se usará `barra inferior` como
nombre de un componente, porque podría referirse a cualquiera de las dos.

Ambas barras conservan su posición y cambian de contenido según el modo:

| Modo | Barra contextual | Barra de acciones |
|---|---|---|
| Vista del mundo | Explica la disponibilidad vigente o muestra el resumen NPC reciente. | Representa las acciones ofrecidas por la aplicación y añade el comando global `Salir`; `Viajar` puede estar activo, deshabilitado por una lista de destinos vacía o ausente cuando el jugador no puede actuar. |
| Selección de destino | Explica que se está eligiendo adónde viajar. | Permite recorrer destinos, confirmar el enfocado o cancelar la selección, y ofrece `Salir` al final. |
| Presentación de acción | Indica cómo continuar después de leer el fragmento actual. | Ofrece `Continuar` cuando el fragmento está listo para confirmarse y `Salir` al final. |
| Decisión | Explica brevemente qué se está decidiendo. | Enumera los controles u opciones vigentes y añade `Salir` al final. |
| Confirmación de salida | Explica que se debe confirmar la salida o volver al modo suspendido. | Ofrece `Sí`, `No` y `Cancelar`; no conserva los comandos del modo suspendido. |
| Terminal insuficiente | No se dibuja; un aviso seguro sustituye la composición completa e incluye el comando local `Salir`. | No se dibuja y no se ofrecen acciones de juego. |

La barra de acciones nunca mezcla comandos de dos modos ni muestra acciones del
mundo mientras la aplicación espera una presentación o una decisión. Los rótulos
y teclas exactos se fijarán en la sección de entrada. `Salir` es un comando
global del frontend y no se considera una mezcla con el modo vigente.

El tamaño mínimo admitido es de 80 columnas por 24 filas de celdas de terminal.
En cualquier tamaño admitido, la barra de acciones ocupa la última fila y la
barra contextual la fila inmediatamente superior. La región del mundo ocupa
todo el ancho y las `alto - 2` filas restantes; por tanto, en el mínimo mide
`80 × 22`. Aumentar o reducir la ventana dentro del intervalo admitido cambia la
cantidad de mapa visible, no sus datos ni la escala de sus caracteres.

Si el ancho es menor que 80 o el alto es menor que 24, el frontend entra en
`Terminal insuficiente`: limpia el búfer y muestra únicamente un aviso que
incluye el mínimo requerido y el tamaño actual. No dibuja el mapa, retratos,
viñeta ni las barras normales, y ninguna tecla puede producir una intención de
juego. El modo semántico, el foco, la página vigente y la instantánea visual se
conservan internamente. Al recuperar ambas dimensiones mínimas, se recalcula la
composición y se retoma el mismo estado sin reiniciar ni avanzar la Partida. El
aviso incluye `[Q] Salir`; es una operación local y por tanto no contradice la
prohibición de enviar intenciones de juego.

Cada barra rellena todas las celdas de su franja con su rol de paleta, incluso
después del último carácter visible. Los fragmentos internos, como el carácter
alfabético entre corchetes de un atajo, aplican su rol sobre ese fondo sin
reemplazarlo. El borde entre ambas barras y su posición fija siguen
distinguiéndolas aunque el emulador altere los colores ANSI.

La composición de acción se construye por capas:

```text
capa de fondo: última vista del mundo
┌────────────────────────────────┐
│ mapa y rótulos                 │
│      ┌────────────────┐        │
│      │ ventana de     │        │
│      │ viñeta o       │        │
│      │ decisión        │        │
│      └────────────────┘        │
│ retrato▐retrato▐retrato        │
├────────────────────────────────┤
│ barra contextual               │
├────────────────────────────────┤
│ barra de acciones              │
└────────────────────────────────┘
```

Al aceptar una acción, el frontend conserva la última `VistaMundo` recibida como
una instantánea de presentación inmutable. Durante los modos de presentación y
decisión vuelve a dibujar esa instantánea como fondo y coloca la ventana encima.
No consulta ni reconstruye un Mundo vivo y no actualiza ese fondo aunque el
núcleo ya haya calculado consecuencias. Cuando la aplicación vuelve a entregar
`VistaMundo` después de la fase NPC, la instantánea anterior se descarta.

La inmutabilidad anterior se refiere a los datos recibidos de la aplicación. El
frontend puede seleccionar otros fotogramas de los retratos contenidos en esa
misma instantánea, por lo que la animación continúa detrás de una viñeta sin
revelar consecuencias nuevas del Mundo.

La ventana se centra dentro de la región del mundo situada por encima de las dos
barras y se limita estrictamente a ella. Sea `(ancho_region, alto_region)` el
tamaño de esa región. Su ancho exterior se calcula en celdas con:

```text
ancho_limite = min(72, ancho_region - 4)
ancho_deseado = max(40, ancho_titulo + 4,
                    linea_explicita_mas_ancha + 4,
                    ancho_arte + 4 si existe)
ancho_ventana = min(ancho_deseado, ancho_limite)
ancho_contenido = ancho_ventana - 4
```

Los cuatro caracteres descontados corresponden a las dos celdas de borde y a una
celda de relleno interior en cada lado. Las medidas de texto usan ancho visible de
terminal, no cantidad de bytes. Un texto más ancho se ajustará al
`ancho_contenido`; un arte que lo exceda se omite conforme a la prioridad del
texto.

La altura exterior necesaria suma dos filas de borde, una fila de relleno arriba
y abajo, las filas de contenido de la página y, cuando existe arte visible, sus
filas más una fila vacía de separación antes del texto. La ventana usa:

```text
alto_ventana = clamp(alto_necesario, 7, alto_region - 2)
```

Por tanto, siempre deja al menos una fila de fondo arriba y otra abajo, nunca
invade las barras y una viñeta breve conserva un mínimo visual de siete filas. Su
origen es `((ancho_region - ancho_ventana) / 2,
(alto_region - alto_ventana) / 2)`, con división entera hacia abajo; cualquier
celda impar sobrante queda a la derecha o abajo. Dentro del marco hay una celda
de relleno en los cuatro lados.

Ratatui renderiza primero el fondo y después aplica `Clear` sobre el rectángulo
calculado. A continuación rellena todas sus celdas con `pantalla`, dibuja el
`Block` con `marco` y escribe en el borde superior un título con el rol `titulo`:
`Suceso` en el modo de presentación y `Decisión` en el modo de decisión. Estos
títulos son texto fijo del frontend y no añaden información narrativa ni
modifican el estado de aplicación. El borde podrá usar un conjunto de caracteres
personalizado; sus glifos concretos se decidirán con el resto de la identidad
artística.

La confirmación de salida reutiliza esa composición superpuesta, sin ilustración
ni paginación. Su título es `Salir` y su texto fijo es
`La partida actual no se guardará. ¿Quieres salir del juego?`. Conserva como
fondo el modo visual suspendido y usa las mismas reglas de centrado, marco,
relleno y ancho mínimo; su altura es el mínimo de siete filas. Si se abrió desde
`Terminal insuficiente`, no intenta reconstruir las barras o el mundo: limpia el
búfer y muestra una versión compacta del título, la advertencia y los controles,
recortada de forma segura a las celdas disponibles.

El estado suspendido incluye modo, foco, cámara, página, instantánea, resumen
NPC, rechazo recuperable y fotograma de retrato. La animación queda congelada y
conserva el tiempo que faltaba para su siguiente paso; al cancelar continúa desde
ese punto sin contar como atrasado el tiempo pasado en la confirmación. Un
redimensionado recalcula solamente la composición exterior: no descarta la
confirmación ni el estado que contiene.

Toda viñeta contiene obligatoriamente el texto narrativo entregado por la
aplicación. Si recibe una `ClaveEscena` con entrada en `scenes.ron`, el frontend
puede añadir su ilustración ASCII estática. Una clave ausente o deliberadamente
no representada produce una viñeta de solo texto y nunca es un error de gameplay.
El texto tiene prioridad: si el tamaño disponible no permite mostrar ambos con
claridad, la ilustración se oculta antes de recortar o volver ilegible el texto.
Un texto que no quepa se pagina localmente conforme al contrato de fragmentos de
la sección 5.

El ajuste usa el ancho visible calculado mediante `unicode-width`, no bytes. Un
salto de línea explícito comienza una línea nueva y una línea explícitamente
vacía se conserva. Dentro de cada línea lógica se ajusta por palabras: los
espacios que quedarían al comienzo o final de una línea creada por el ajuste se
omiten, mientras los espacios interiores permanecen. Si una sola palabra supera
`ancho_contenido`, se divide por límites de `char` sin producir una línea más
ancha; los valores de ancho cero permanecen unidos al carácter precedente cuando
existe. El frontend no reescribe, resume ni añade texto narrativo.

Después del ajuste, las líneas se distribuyen en páginas. Cuando existe más de
una, todas reservan su última fila de contenido para `Página n/N`, alineado a la
derecha con el rol `secundario`, y la ventana usa `alto_region - 2` durante todo
el fragmento. Una viñeta de una sola página conserva la altura adaptada definida
antes y no muestra indicador. El ancho y la posición tampoco cambian entre
páginas.

La ilustración aparece únicamente en la primera página. Solo se conserva si cabe
en `ancho_contenido` y, después de reservar su altura, una fila separadora y el
indicador cuando corresponda, quedan al menos tres filas narrativas. Si no se
cumple cualquiera de esas condiciones, se omite para el fragmento completo y se
recalcula la paginación como texto solo. El cálculo puede realizarse en dos
pasadas: primero prueba la ilustración y después produce el resultado definitivo.

El arte concreto no procede de `desert-tiger-core`, `desert-tiger-app` ni
`content/`. El catálogo del frontend no puede ejecutar reglas ni cambiar el
significado de la clave; otro frontend es libre de usar otro medio visual o de
ignorarla. Los retratos de Actor recibidos desde contenido son la única excepción
de arte ASCII externo al crate de terminal.

Las ilustraciones de viñeta son estáticas en el primer incremento. No tienen
reloj, fotogramas ni transiciones propias. Esta restricción no afecta la
animación independiente de los retratos del jugador y del Grupo dibujados en el
fondo.

Cuando la ilustración está presente, la distribución interior es vertical: arte
en la región superior y texto debajo. Sin ilustración, el texto utiliza toda la
región interior del marco. El cálculo se realiza después de reservar borde y
relleno, y no inserta las opciones de una decisión dentro de estas regiones.

La ilustración usa `destacado` y su texto narrativo usa `texto_normal`. Cada
espacio del bloque de arte es transparente: conserva el fondo `pantalla` de la
ventana y no escribe glifo ni estilo. Los demás caracteres sustituyen el glifo y
aplican el frente blanco y `negrita` de `destacado`, manteniendo el fondo negro.
Cuando existe más de una página, su indicador usa `secundario`. Estos estilos
pertenecen a la terminal y nunca se incorporan a `scenes.ron` ni a las vistas de
aplicación.

En `Presentación de acción`, `Enter` avanza solamente la página local mientras
quede otra. En la última página solicita a la aplicación confirmar el fragmento;
un único evento de tecla nunca puede cambiar de página y confirmar además el
fragmento. La barra de acciones muestra `[Enter] Continuar  [Q] Salir` en ambos
casos. Las reglas de lectura y opciones para una decisión paginada se especifican
por separado.

Una decisión emplea la misma ventana para su texto y contexto; sus opciones se
escriben en la barra de acciones. Su texto, ilustración e indicador siguen las
mismas reglas de ajuste y paginación que un fragmento. Mientras no se haya
mostrado la última página, la barra contiene únicamente el control del modo
`[Enter] Continuar` y el comando global `[Q] Salir`; las teclas de opción están
inactivas y `Enter` solo avanza una página local. En la última página desaparece
`Continuar` y se muestran simultáneamente todas las opciones seguidas de
`[Q] Salir`; `Enter` por sí solo no elige ninguna.

Las opciones conservan el orden entregado por la aplicación. Para cada una, el
frontend busca de izquierda a derecha la primera letra ASCII de su texto que no
haya sido asignada antes ni sea `Q`, sin distinguir mayúsculas. Si no existe, usa
la primera letra libre de `A` a `Z`, también excluyendo `Q`. La presentación falla
si no puede producir una letra única. Cada opción se escribe como `[X] Texto`; la
`X` usa `atajo`, se deja un espacio al comienzo y al final de la barra y dos
espacios entre opciones y antes de `[Q] Salir`. No se envuelve, abrevia ni recorta
una opción.

En el primer incremento, toda decisión oficial debe caber completa en la barra
de 80 columnas con ese formato, incluido `[Q] Salir`. Esta es una condición de
calidad comprobada por una prueba de integración del frontend, no una restricción
de `SolicitudDecision` ni del motor. Un desbordamiento inesperado se convierte en
un error tipado de presentación y nunca en escritura fuera del búfer. Los menús
de opciones paginados quedan fuera del alcance inicial.

Después de elegir una opción, el fondo permanece sin cambios mientras la
aplicación produce la continuación. Tras la última viñeta, el fondo sigue así
hasta que la aplicación haya avanzado la fecha y terminado la fase NPC. Entonces
el frontend dibuja la nueva `VistaMundo`. El resumen NPC ocupa solamente la barra
contextual y nunca abre una ventana superpuesta.

El mapa se compondrá por capas. Para cada celda, la clave de tipo visual vigente
se resuelve a un glifo y estilo del catálogo de terminal; esos tiles se dibujan
primero y después se añaden rótulos, foco e indicadores ilustrativos. Para cada
Lugar, el carácter situado en la posición `n` de su nombre se dibuja sobre la
coordenada `n` de su lista de rótulo. Dibujar esta capa no modifica los tipos de
la Partida. El frontend recibe nombres ya validados carácter por carácter con
ancho de una columna y no intenta normalizarlos, redistribuirlos ni permitir que
un carácter ocupe una coordenada adicional. La posición exacta de un actor sobre
el mapa es estética y nunca se devuelve a la aplicación.

Los rótulos se recorren en el orden canónico recibido en `VistaMundo`. Solo se
dibujan sus caracteres cuyas coordenadas caen dentro de la cámara; los restantes
se omiten sin mover, envolver ni abreviar el nombre. No se buscan colisiones entre
rótulos ni se intenta resolverlas. Si dos caracteres ocupan la misma celda, el
Lugar dibujado después reemplaza visualmente al anterior. Esto no invalida el
contenido: la legibilidad de la composición pertenece al diseñador. Durante la
selección, el rótulo enfocado participa en el mismo orden y puede quedar parcial
o totalmente cubierto; la barra contextual siempre repite su nombre completo.

Cada fotograma escribe el glifo y el estilo base completos de todas las celdas
visibles después de que `pantalla` haya rellenado el búfer. Por ello,
`fondo: None` no impone un fondo propio y conserva el fondo negro de `pantalla`;
nunca hereda el de otro tipo de tile o del fotograma previo. Un fondo explícito
del tile sí lo reemplaza. Las capas posteriores pueden aplicar un estilo
adicional sin modificar el catálogo.

La cámara es estado efímero del frontend y su objetivo siempre es la coordenada
central de un Lugar. En `Vista del mundo` usa el Lugar del jugador; en
`Selección de destino`, el Lugar enfocado. Cancelar la selección devuelve el
objetivo al Lugar del jugador. No existe desplazamiento libre de cámara en el
primer incremento. Durante una presentación o decisión se conserva la cámara de
la instantánea usada como fondo y una viñeta no la desplaza.

Para cada eje donde `mapa >= vista`, el origen visible se calcula y limita así:

```text
origen_deseado = objetivo - (vista / 2)
origen = clamp(origen_deseado, 0, mapa - vista)
```

Las divisiones enteras redondean hacia abajo. El cálculo de `origen_deseado` se
realiza en un entero capaz de representar valores negativos y solo se convierte
al tipo de coordenada después de aplicar `clamp`; un objetivo próximo al borde
izquierdo o superior no puede causar un desbordamiento aritmético. El objetivo
queda tan centrado como permiten los bordes y nunca se solicitan celdas
exteriores al mapa. Un redimensionado admitido repite el cálculo con el objetivo
vigente.

Si el mapa es menor que la vista en un eje, su origen de mapa permanece en cero
y se desplaza al centro de la región mediante
`margen_inicial = (vista - mapa) / 2`. Cuando sobra una celda impar, queda en el
margen derecho o inferior. Las celdas de margen conservan el rol `pantalla` y no
se inventan tiles. Este cálculo se aplica por separado a ambos ejes, por lo que
un mapa puede estar centrado horizontalmente y limitado por cámara
verticalmente, o al revés.

Los retratos representan la parte superior de los actores y no reciben una
región de distribución exclusiva. Sus bordes inferiores se alinean con la fila
inmediatamente superior a la barra contextual y el primero comienza en el borde
izquierdo; por ello se extienden hacia arriba y cubren visualmente una parte del
mapa, pero nunca invaden ninguna de las dos barras.

El frontend dibuja primero al jugador y después a los compañeros en el orden
recibido de `VistaMundo`. Cada retrato posterior comienza más a la derecha que el
anterior, antes de que termine su ancho, y se dibuja después: en las celdas de
intersección queda visualmente por delante.

Para cada pareja consecutiva, el solapamiento horizontal equivale al 25 % del
ancho del retrato más estrecho, medido en columnas de caracteres. El cálculo usa
división entera hacia abajo y un mínimo de una columna:

```text
solapamiento = max(1, min(ancho_anterior, ancho_siguiente) / 4)
x_siguiente = x_anterior + ancho_anterior - solapamiento
```

Como todos los fotogramas de un mismo Actor tienen las mismas dimensiones, la
posición no cambia durante su animación.

Los espacios de un fotograma de retrato son transparentes. Conservan su posición
y cuentan para calcular el ancho, la alineación y el solapamiento, pero el
renderizador omite esas celdas por completo: no reemplaza el glifo ni el estilo
que ya aportaron el mapa, un rótulo u otro retrato. Cualquier carácter ASCII
distinto del espacio, o cualquier glifo Unicode validado de una columna, sí
reemplaza el símbolo de la celda. Si `CeldaRetrato::estilo` es `None`, se aplica
el rol `destacado` como reserva monocroma. Si contiene un estilo, la terminal
convierte `ColorRetrato` a la variante equivalente de
`ratatui::style::Color`, aplica el frente y aplica el fondo solamente cuando sea
`Some`; no añade `negrita`, no interpreta secuencias ANSI y no consulta los
recursos de tiles. Un fondo ausente conserva el fondo que ya aportó el mapa. El
orden de capas mantiene la transparencia y permite que los bloques parciales de
Durdraw combinen frente y fondo dentro de una celda.

Los retratos del contenido oficial inicial deberán caber completos en la región
situada sobre las barras cuando la terminal tenga el tamaño mínimo admitido. Esta
es una comprobación de integración entre contenido y frontend, no una restricción
general del formato de Actor. Si contenido modificado aporta retratos demasiado
altos o un conjunto demasiado ancho, el renderizador conserva las posiciones y
el solapamiento calculados y recorta únicamente las celdas que quedan fuera del
borde superior o derecho de la región. No escala, redistribuye, incrementa el
solapamiento ni escribe sobre las barras; un retrato que quede completamente
fuera puede no producir celdas visibles sin invalidar la Partida.

El fotograma vigente pertenece al frontend y la lista ordenada de fotogramas
procede del catálogo de presentación.

En selección de viaje, la cámara enfoca el lugar activo y su nombre usa el rol
`foco`. La barra contextual comienza con `> Destino: ` y repite el nombre del
Lugar, además de explicar que se está eligiendo adónde viajar. En listas o menús
futuros, la fila enfocada comenzará también con `>`. La lista de candidatos
procede de la aplicación. Los lugares no disponibles siguen dibujados como parte
del mundo, pero no reciben foco ni pueden confirmarse. El foco inicial y el orden
de navegación usan la secuencia entregada por la aplicación; las flechas y el
comportamiento en los extremos siguen el contrato de entrada de la sección 9.

Al terminar la fase NPC se vuelve a `Vista del mundo`, con la disponibilidad
declarada por la nueva `VistaMundo`. El resumen se muestra como texto breve en
la misma barra contextual de esa vista; no constituye una pantalla o modo
independiente y no necesita una tecla para cerrarse.

En `Vista del mundo`, `Viajar { destinos }` se muestra activo solamente cuando
`destinos` no está vacío. Con una lista vacía permanece visible como
`Viajar (no disponible)`, sin `[V]`, foco ni posibilidad de entrar en selección.
Si `acciones` está vacía y `causa_indisponibilidad` es
`Some(JugadorMuerto)`, no se muestra `Viajar`: la barra de acciones contiene
únicamente `[Q] Salir` y la vista no abre una pantalla de muerte. El frontend
decide entre estas presentaciones solo mediante los campos de `VistaMundo`; no
inspecciona Características.

La barra contextual aplica esta prioridad:

1. explicación de la confirmación de salida activa;
2. rechazo recuperable todavía vigente;
3. explicación de la selección, decisión o presentación activa;
4. causa `JugadorMuerto`:
   `El personaje jugador está muerto y no puede actuar.`;
5. resumen NPC reciente, si existe;
6. `Viajar` sin destinos: `No hay destinos disponibles para viajar.`;
7. mensaje normal de la vista del mundo: `Elige una acción en el mundo.`.

Entrar en selección de destino reemplaza visualmente el resumen por la
explicación de la selección. Si se cancela antes de enviar una acción, el resumen
puede volver a mostrarse porque sigue presente en `VistaMundo`. Cuando la
aplicación acepta el siguiente viaje, descarta ese resumen.

El contrato de sesión es la fuente normativa del ciclo de vida del resumen; este
documento solo define la prioridad con que la barra representa el estado que
recibe.

## 8. Bucle, animación y restauración

El primer incremento usará un bucle síncrono y monohilo:

1. espera un evento de Crossterm o el próximo vencimiento visual;
2. traduce y procesa la entrada;
3. consulta o actualiza la aplicación cuando corresponde;
4. actualiza el estado de interfaz;
5. solicita a Ratatui un nuevo renderizado cuando cambió algo visible.

La cadencia inicial de los retratos será global y fija: un paso cada 250 ms. El
RON no contiene duraciones. El bucle calcula el tiempo de espera hasta el próximo
paso y permite que la entrada o un redimensionado lo interrumpan. Ratatui no
reproduce la animación: solo dibuja el fotograma que el estado del frontend haya
seleccionado.

La temporización utiliza un reloj monotónico del frontend y nunca adelanta la
fecha del Mundo. Si un vencimiento se atiende tarde, se calcula cuántos intervalos
completos transcurrieron y se avanza esa cantidad módulo el número de fotogramas;
no se reproducen en ráfaga los estados atrasados. Una lista de un solo fotograma
permanece estática y no requiere vencimientos propios. No se incorporará un
runtime asíncrono mientras este bucle resulte suficiente.

La inicialización y el cierre serán simétricos. Un guardián de ciclo de vida
restaurará cursor, modo raw y pantalla principal tanto ante una salida normal
como ante un error propagado. Si la terminal cambia de tamaño, se recalculará la
composición antes de dibujar otra vez.

## 9. Entrada y accesibilidad básica

El esquema inicial prioriza teclas habituales y visibles, sin exigir que el
jugador conozca convenciones particulares de editores o roguelikes:

| Modo | Tecla | Resultado |
|---|---|---|
| Vista del mundo | atajo `[X]` antepuesto a una acción activa | Activa esa acción; `[V] Viajar` abre la selección solamente si su lista de destinos no está vacía. |
| Selección de destino | `←` / `→` | Mueve el foco al destino anterior o siguiente según el orden recibido. |
| Selección de destino | `Enter` | Confirma el destino enfocado y solicita `Viajar`. |
| Selección de destino | `Esc` | Cancela solamente la selección visual y vuelve a la vista del mundo. |
| Presentación de acción | `Enter` | Avanza la página local o, al terminar el fragmento, solicita continuar la presentación. |
| Decisión | atajo `[X]` antepuesto a una opción | Envía la opción correspondiente a la aplicación. |
| Todo modo salvo confirmación de salida | `Q` | Abre la confirmación local de salida sin enviar una intención de juego. |
| Confirmación de salida | `S` | Confirma la salida sin guardar y comienza el cierre normal del frontend. |
| Confirmación de salida | `N` o `Esc` | Cancela la salida y restaura el modo visual suspendido. |

Las teclas alfabéticas se aceptan sin distinguir mayúsculas de minúsculas. Cada
acción u opción visible recibe un atajo no ambiguo, mostrado como `[X]` antes de
su rótulo. La asignación es estado del frontend y no forma parte de la regla ni
del identificador semántico enviado a la aplicación. `Q` queda reservada para
`[Q] Salir`: se considera ocupada antes de asignar cualquier atajo dinámico y no
puede representar una acción general ni una opción de decisión.

La forma activa canónica es `[V] Viajar`: los corchetes hacen visible la tecla y
la letra `V` recibe el rol `atajo`; las opciones de una decisión siguen el mismo
patrón. En el primer incremento, una variante `Viajar` con lista de destinos
vacía produce exactamente el control `Viajar (no disponible)`. Usa el rol
`deshabilitado`, no recibe foco, no anuncia un atajo activo y ninguna entrada
puede convertirlo en una intención. Si la acción está ausente, el frontend no
inventa ese control, ni siquiera deshabilitado.

La barra de acciones escribe también los nombres de las teclas especiales:
flechas, `Enter` y `Esc` no quedan como controles ocultos. `[Q] Salir` ocupa la
última posición de toda barra normal y se separa del control anterior mediante
dos espacios. En `Terminal insuficiente` aparece dentro del aviso que sustituye
las barras. Si se intenta avanzar más allá de un extremo de la lista de destinos,
el foco permanece en el mismo lugar; la navegación no da la vuelta. `Esc` deja de
cancelar una selección una vez que la aplicación acepta el destino, porque la
cancelación de una acción ya iniciada queda fuera del primer incremento; esto no
impide abrir y cancelar la confirmación de salida durante su presentación.

Un adaptador dependiente del modo convierte cada tecla física en una operación
visual local o una intención semántica. Al pulsar `Q`, guarda el modo visual, el
foco, la cámara, la página, la instantánea y cualquier mensaje recuperable
vigentes, y abre `Confirmación de salida`. Esta transición no llama a
`Partida::procesar`, no consume azar y no avanza la fecha. Mientras la
confirmación está abierta, ninguna tecla alcanza el modo suspendido: un solo
`Esc` vuelve a él, pero no cancela además una selección subyacente.

La barra de confirmación muestra `[S] Sí  [N] No  [Esc] Cancelar`. `N` y `Esc`
restauran exactamente el estado visual guardado. `S` descarta la Partida sin
guardar, ejecuta el cierre normal que restaura la terminal y termina con código
de salida cero. Debido al bucle síncrono, el frontend no puede interrumpir una
llamada a la aplicación que ya se está ejecutando; procesa `Q` cuando recupera el
control y puede hacerlo antes de comenzar una acción o durante sus modos de
presentación y decisión. Un error fatal no solicita confirmación y conserva el
recorrido de cierre definido en la sección 10.

Dentro de la confirmación, `Q`, `Enter`, flechas y cualquier otra tecla distinta
de `S`, `N` o `Esc` se ignoran. Por tanto, no se pueden anidar confirmaciones y un
evento nunca se reutiliza después de cerrar el modo. `S` y `N` solo tienen este
significado especial dentro de la confirmación y siguen disponibles para atajos
dinámicos en los demás modos.

Cada glifo elegido por el catálogo de terminal para un tile será un carácter
ASCII de exactamente una celda; puede ser un espacio. Esta restricción se aplica
a la apariencia terminal, no a los símbolos compactos del archivo de Mapa, que
ya fueron resueltos antes de construir `VistaMapa`. Los textos podrán contener
acentos; el renderizado deberá medir su ancho visible en celdas y recortarlos sin
romper la composición. El color reforzará el estado, pero una selección también
deberá reconocerse por el texto o la forma, no solo por el color.

## 10. Errores y validaciones

La aplicación es la fuente normativa de la categoría de cada problema. El
frontend no inspecciona Lua, no reinterpreta un `ErrorIntencion` y no decide si
un fallo de política NPC permite continuar: recibe un error de inicio, un
rechazo recuperable de intención, una incidencia recuperable dentro de
`ResumenNpc` o una `VistaSesion::Fatal` según los contratos de aplicación y
presenta esa categoría.

### Errores de inicio

La ausencia, ilegibilidad o invalidez de `assets/terminal/scenes.ron`,
`assets/terminal/tiles.ron` o `assets/terminal/ui.ron` es un error de
inicialización. Los errores al cargar contenido, construir la Partida o preparar
la terminal siguen el mismo recorrido: se escriben como texto plano en la salida
de error estándar antes de activar el modo raw o la pantalla alternativa y el
proceso termina con un código distinto de cero. No se abre una interfaz parcial.

Cada diagnóstico comienza con `Error:` y conserva el contexto seguro que esté
disponible, como la categoría, el archivo, el identificador de Actor y la línea o
columna. El formato no presupone que todos esos datos existan; la estructura
definitiva de los diagnósticos pertenece a los componentes que los producen.

### Rechazos recuperables

Un `Err(ErrorIntencion)` se muestra en la barra contextual, con el prefijo
`Error:` y el rol `error`. La terminal conserva el modo, el foco, la cámara, la
página local y la vista de aplicación vigentes; no aplica una mutación local para
simular que la solicitud tuvo éxito. El mensaje sustituye temporalmente el texto
contextual normal y permanece hasta que la aplicación acepta otra intención o el
usuario cancela o cambia el modo visual local. Abrir `Confirmación de salida` es
la excepción: el mensaje forma parte del estado suspendido y reaparece intacto si
se cancela la salida.

El resumen será breve y orientado a la acción que el jugador puede tomar. Si no
cabe en una fila, se recorta por ancho visible y termina en `...`, sin separar un
carácter de ancho múltiple ni perder el prefijo `Error:`. Los detalles técnicos
no se vuelcan dentro de la interfaz de juego.

### Incidencias recuperables de NPC

`EntradaResumenNpc::ViajeCompletado` usa el rol normal de la barra contextual.
`SolicitudViajeRechazada` y `ErrorPolitica` son incidencias recuperables, pero se
presentan con el prefijo `Error:` y el rol `error`, igual que un rechazo de
intención. Siguen perteneciendo al resumen: no restauran un modo anterior, no
abren una viñeta y no impiden elegir la siguiente acción.

La terminal usa las claves o nombres de Lugar ya incluidos en `VistaMundo` y el
código estructurado de la incidencia. No muestra el bloque Lua, una traza,
`Debug`, IDs numéricos ni el campo técnico completo `detalle` dentro de la barra.
Ese detalle permanece disponible en la vista para un futuro mecanismo de reporte,
pero la interfaz inicial muestra una causa breve y segura.

El ciclo de vida es el del resumen normal: la incidencia permanece mientras no
se acepte otra acción del jugador, puede quedar cubierta temporalmente por un
modo de mayor prioridad y no necesita confirmación para cerrarse. Un fallo fatal
de infraestructura nunca llega por esta vía.

### Fallos fatales

Al recibir `VistaSesion::Fatal`, la terminal deja de aceptar intenciones y sale
del bucle. Un fallo propio durante la lectura de eventos, la composición o el
dibujo sigue el mismo recorrido. Primero se realiza el cierre de mejor esfuerzo:
mostrar el cursor, abandonar la pantalla alternativa y desactivar el modo raw;
solo después se escribe el diagnóstico seguro como texto plano en la salida de
error estándar y se termina con un código distinto de cero. El guardián de ciclo
de vida aplica la misma restauración aunque el error se propague desde una capa
interna.

### Texto seguro para terminal

Antes de mostrar un campo procedente de contenido o de un componente interno, el
presentador lo convierte en una sola línea segura. Escapa como notación visible
todo `char` para el que `is_control()` sea verdadero —incluidos saltos de línea,
tabulaciones y `ESC`— y los controles bidireccionales Unicode `U+202A..U+202E` y
`U+2066..U+2069`. Usa `\n`, `\r` y `\t` para esos tres controles y
`\u{CODIGO}` para los restantes. Solo el formateador añade saltos de línea entre
campos. De este modo, un nombre o mensaje modificado no puede inyectar ANSI,
borrar la pantalla ni falsear visualmente el orden del diagnóstico.

Los diagnósticos para el jugador no incluyen el bloque Lua completo, el
contenido íntegro de un archivo RON, trazas internas, volcados de Mundo o del
generador aleatorio ni representaciones `Debug` sin filtrar. La causa resumida y
el contexto estructurado disponible deben bastar para copiar el mensaje en un
reporte; los detalles de desarrollo permanecen en la cadena de error interna.

- Todo error visible comienza con `Error:`; dentro de la interfaz usa además el
  rol `error`, por lo que el color nunca es la única señal de fallo.
- La composición recortará contenido fuera de su región y no escribirá fuera del
  búfer.
- Una terminal menor que `80 × 24` mostrará el aviso seguro de pantalla completa
  y no una interfaz parcialmente interactiva.

## 11. Pruebas y criterios de aceptación

Pruebas automáticas:

- eventos sintéticos se traducen a la intención correcta para cada modo;
- los atajos `[X]` funcionan con mayúsculas y minúsculas y nunca identifican dos
  comandos visibles a la vez; `Q` nunca se asigna dinámicamente y `S` o `N`
  siguen disponibles fuera de la confirmación;
- `←` y `→` recorren destinos sin envolver los extremos, `Enter` confirma y
  `Esc` cancela la selección solamente antes de iniciar la acción;
- `Enter` pagina localmente un fragmento y solo envía su confirmación después de
  mostrar la última página;
- una decisión paginada mantiene inactivas sus opciones hasta la última página,
  donde desaparece `Continuar` y cada letra de opción envía solamente su opción;
  `[Q] Salir` permanece disponible en ambas etapas;
- los atajos de opción se asignan en orden mediante la primera letra ASCII libre
  distinta de `Q` y luego mediante la primera de `A` a `Z` disponible que tampoco
  sea `Q`;
- las decisiones oficiales de `Obstáculo` y `Rumbo incierto` producen,
  respectivamente, los pares de atajos naturales `[A]`/`[R]` y `[C]`/`[R]`, sin
  alterar los textos ni los identificadores recibidos;
- todas las opciones de cada decisión oficial caben con separadores y márgenes en
  80 columnas junto con `[Q] Salir`; un caso sintético que excede el ancho
  devuelve un error tipado sin escribir fuera del búfer;
- una vista de jugador vivo con destinos muestra `[V] Viajar` y `[Q] Salir`, y
  `V` abre la selección sin alterar la lista recibida;
- una vista de jugador vivo con `Viajar { destinos: [] }` muestra exactamente
  `Viajar (no disponible)` y `[Q] Salir`, sin `[V]`; `V`, flechas y `Enter` no
  crean foco, no cambian de modo y no llaman a la aplicación;
- una vista con `acciones: []` y `JugadorMuerto` no muestra `Viajar`, conserva
  solamente `[Q] Salir` y usa el mensaje contextual de muerte sin abrir otro
  modo;
- con `JugadorMuerto` y un resumen NPC simultáneo, el mensaje de muerte tiene
  prioridad y la terminal sigue sin crear una pantalla adicional;
- la barra inicial no muestra acciones generales de incrementos futuros;
- cada modo muestra en la barra de acciones solamente sus comandos u opciones
  vigentes y el comando global `Salir`, sin conservar controles del modo anterior;
- la selección de destino sustituye `Viajar` por navegación, confirmación y
  cancelación; la presentación ofrece `Continuar`; una decisión muestra sus
  opciones; y las cuatro barras terminan con `[Q] Salir`;
- `q` y `Q` abren una sola confirmación desde la vista del mundo, selección,
  presentación, decisión y el aviso de terminal insuficiente, sin llamar a la
  aplicación ni alterar Mundo, fecha o azar;
- la confirmación muestra el título, la advertencia sin guardado y exactamente
  `[S] Sí  [N] No  [Esc] Cancelar`; cualquier otra tecla se ignora y `Q` no anida
  otra confirmación;
- `N` y `Esc` restauran exactamente modo, foco, cámara, página, instantánea,
  resumen NPC, rechazo recuperable, fotograma y tiempo restante de animación;
  desde selección, ese mismo `Esc` no cancela además el viaje;
- `S` descarta la Partida, restaura cursor, pantalla principal y modo de entrada,
  y termina con código cero sin confirmar fragmentos, elegir opciones, avanzar
  tiempo ni ejecutar la fase NPC;
- una viñeta, una decisión y la confirmación de salida se superponen al fondo
  correspondiente sin cubrir las barras ni revelar una actualización prematura;
- la ventana queda centrada, permanece dentro de la región superior y deja un
  margen de fondo visible cuando el tamaño admitido lo permite;
- contenidos breves, líneas largas y arte de varios anchos producen una ventana
  entre 40 y 72 columnas con una celda de relleno, altura entre 7 y
  `alto_region - 2` y el centrado exacto definido;
- un arte que excede `ancho_contenido` se omite y, cuando se muestra, una fila
  vacía lo separa del texto;
- toda viñeta muestra su texto dentro de un marco y una ilustración opcional se
  omite antes de perjudicar la legibilidad;
- la ventana rellena primero todo su rectángulo con `pantalla`, usa `marco` para
  el borde y muestra `Suceso`, `Decisión` o `Salir` con `titulo` según el modo;
- el arte usa `destacado`, sus espacios conservan el fondo de la ventana, el
  texto usa `texto_normal` y un indicador de página usa `secundario`;
- saltos explícitos, líneas vacías, ajuste por palabras y palabras mayores que el
  ancho producen las líneas esperadas según su ancho visible;
- una viñeta paginada conserva geometría, reserva una fila para `Página n/N` y
  solo confirma el fragmento al pulsar `Enter` desde la última página;
- la ilustración aparece solo en la primera página cuando cabe y deja al menos
  tres filas narrativas; en caso contrario se omite y las páginas se recalculan;
- una clave conocida resuelve arte local del frontend, mientras una clave
  ausente o sin representación produce una viñeta válida de solo texto;
- `ClaveEscena::ViajeMarcha`, `ClaveEscena::ViajeObstaculo`,
  `ClaveEscena::ViajeRumboIncierto` y `ClaveEscena::ViajeLlegada` se convierten
  exactamente en `viajeMarcha`, `viajeObstaculo`, `viajeRumboIncierto` y
  `viajeLlegada`, respectivamente, y cada una puede resolver una ilustración
  distinta sin alterar el texto recibido;
- una decisión con `ClaveEscena::ViajeObstaculo` resuelve `viajeObstaculo` sin
  alterar su texto, opciones ni comportamiento, y continúa siendo una decisión
  válida de solo texto cuando el catálogo no contiene esa ilustración;
- una decisión con `ClaveEscena::ViajeRumboIncierto` resuelve
  `viajeRumboIncierto` con las mismas garantías de texto, opciones,
  comportamiento y reserva de solo texto;
- `scenes.ron` admite una lista vacía y rechaza claves desconocidas, duplicadas,
  alias, diferencias de mayúsculas, grafías `snake_case`, bloques vacíos, no
  rectangulares, no ASCII o con controles;
- el arte multilínea conserva espacios y aplica exactamente la normalización de
  saltos inicial, final y CRLF definida por el catálogo;
- ninguna vista de aplicación ni archivo bajo `content/` aporta bytes o rutas de
  arte de viñeta;
- cuando hay ilustración, aparece encima del texto; cuando no la hay, el texto
  ocupa toda la región interior disponible;
- las ilustraciones de viñeta permanecen estáticas aunque los retratos del fondo
  continúen usando sus fotogramas de animación;
- el regreso desde la acción dibuja la vista actualizada del mundo y mantiene el
  resumen NPC limitado a la barra contextual;
- las transiciones visuales no ejecutan reglas propias;
- una vista con tiles modificados se representa sin que el frontend infiera o
  vuelva a ejecutar el cambio semántico que la produjo;
- una `ClaveTipoTile` conocida se representa con el glifo y estilo exactos del
  catálogo, aunque ese glifo sea distinto del símbolo usado en el archivo RON;
- una clave válida sin entrada usa exactamente la apariencia de reserva `?`, sin
  impedir cargar contenido modificado ni provocar `panic`;
- todos los tipos visuales del contenido oficial tienen una apariencia terminal
  y cada glifo configurado ocupa exactamente una celda;
- el catálogo admite entradas adicionales, pero rechaza duplicados, claves,
  glifos o estilos inválidos y una reserva ausente o distinta de `?`;
- los dieciséis nombres de color se convierten a la variante exacta de Ratatui,
  `None` no sustituye el fondo de la capa existente y se rechazan `Reset`, RGB,
  índices, alias y nombres desconocidos;
- los modificadores se convierten a `BOLD`, `DIM` y `REVERSED` sin depender del
  orden; se rechazan duplicados y la combinación de `negrita` con `tenue`;
- dos fotogramas consecutivos con fondos distintos no dejan estilos residuales
  en una celda y una capa de foco no modifica la apariencia base almacenada;
- cada uno de los doce roles de `ui.ron` se convierte a los colores y
  modificadores exactos declarados en la paleta aprobada, y cualquier campo
  ausente, duplicado o desconocido se rechaza;
- el fondo `pantalla` cubre el búfer completo, ambas barras cubren sus franjas y
  los estilos `None` conservan el fondo de la capa correspondiente;
- foco, atajos, errores y elementos deshabilitados conservan respectivamente
  `>`, corchetes, `Error:` y `(no disponible)` al inspeccionar el búfer sin color;
- un control deshabilitado no puede recibir foco ni producir una intención;
- las rutas de `assets/terminal/scenes.ron`, `assets/terminal/tiles.ron` y
  `assets/terminal/ui.ron` se resuelven respecto del directorio de trabajo y
  ninguna cambia al usar `--content`;
- la ausencia o invalidez de cualquiera de esos tres recursos impide iniciar el
  modo raw;
- un error de contenido, construcción o recursos se escribe en texto plano con
  `Error:`, deja intacto el estado normal de la terminal y termina con código
  distinto de cero;
- un `ErrorIntencion` sustituye temporalmente la barra contextual sin cambiar el
  modo, el foco, la cámara, la página ni la vista vigente; persiste ante entradas
  irrelevantes y desaparece cuando la aplicación acepta otra intención o el
  usuario cancela o cambia el modo visual local, salvo que solo abra y cancele la
  confirmación de salida;
- un `ViajeCompletado` de NPC usa el rol normal de la barra, mientras
  `SolicitudViajeRechazada` y `ErrorPolitica` conservan el prefijo `Error:` y el
  rol `error` sin abrir otro modo ni impedir la siguiente acción;
- una incidencia NPC permanece con el ciclo de vida del resumen, queda cubierta
  por modos de mayor prioridad y reaparece al cancelarlos;
- el resumen visible de una política fallida conserva su clave de Actor y código
  útil para reportarla, pero no muestra el bloque, la traza, IDs numéricos ni el
  detalle técnico completo;
- un rechazo demasiado largo conserva `Error:`, se recorta por ancho visible y
  termina en `...` sin desbordar la barra;
- una `VistaSesion::Fatal` y los fallos sintéticos de eventos o dibujo dejan de
  aceptar intenciones, restauran cursor, pantalla principal y modo de entrada
  antes de escribir en `stderr`, terminan con código distinto de cero y nunca
  abren la confirmación de salida;
- campos de diagnóstico con saltos, tabulaciones, `ESC` o controles
  bidireccionales se muestran escapados, nunca emiten esos caracteres de control
  y no alteran las líneas generadas por el formateador;
- ningún diagnóstico presentado contiene el bloque Lua o RON completo, un
  volcado de Mundo o azar, una traza interna ni una representación `Debug` sin
  filtrar;
- los caracteres de cada nombre aparecen, en orden, sobre las coordenadas de su
  rótulo sin modificar las celdas del mapa conservadas por la aplicación;
- ningún carácter de un rótulo desplaza las celdas posteriores ni ocupa más de
  una coordenada;
- dos rótulos coincidentes se aceptan y el carácter del Lugar posterior en el
  orden canónico queda visible, sin reubicar ni rechazar ninguno;
- los caracteres exteriores a la cámara se omiten y el destino enfocado conserva
  su nombre completo en la barra contextual;
- la cámara se centra en el Lugar del jugador y cambia al destino enfocado sin
  modificar ninguna coordenada del mapa;
- objetivos centrales y próximos a cada uno de los cuatro bordes producen el
  origen limitado por la fórmula, sin solicitar celdas exteriores;
- un mapa menor que la región se centra en cada eje y coloca cualquier celda
  impar sobrante en el margen derecho o inferior con el rol `pantalla`;
- cancelar una selección restaura el objetivo del jugador, una viñeta conserva
  la cámara de su fondo y un redimensionado recalcula el origen vigente;
- los componentes dibujan dentro de su región en varios tamaños;
- en `80 × 24`, el mundo ocupa exactamente `80 × 22`, la barra contextual la
  penúltima fila y la barra de acciones la última;
- `79 × 24` y `80 × 23` muestran solamente el aviso con tamaño requerido y
  actual; `[Q] Salir` abre su confirmación compacta, pero ninguna entrada produce
  intenciones de juego;
- reducir la terminal durante una selección, presentación o decisión y volver a
  un tamaño admitido conserva el modo, el foco o página y la instantánea visual;
- reducir la terminal durante la confirmación conserva tanto esa confirmación
  como su estado suspendido; al recuperar el tamaño reaparece la ventana y al
  cancelarla vuelve el modo original;
- el mapa conserva toda la altura disponible sobre las barras y los retratos se
  dibujan como una capa, sin reducirla;
- el retrato del jugador comienza en el borde izquierdo, todos los retratos se
  apoyan en la fila inmediatamente superior a la barra contextual y ninguno
  invade las barras;
- cada compañero se desplaza hacia la derecha y, en las celdas donde se cruza
  con el retrato anterior, el retrato dibujado después queda por delante;
- retratos consecutivos de anchos iguales y diferentes aplican la fórmula del
  25 %, incluido su redondeo hacia abajo y el mínimo de una columna;
- cambiar de fotograma no desplaza ningún retrato;
- un espacio de retrato conserva exactamente el glifo y el estilo previamente
  dibujados en la celda; una celda visible sin estilo usa `destacado`, y una
  celda coloreada reemplaza glifo, frente y fondo explícito sin perder un fondo
  heredado cuando el suyo es `None`;
- los dos fotogramas oficiales del Príncipe conservan dimensiones y colores,
  alternan el ojo sin mover el retrato y dejan ver el mapa a través del padding;
- el conjunto de retratos del contenido oficial cabe completo en el tamaño
  mínimo admitido;
- retratos modificados que exceden la región se recortan en sus bordes superior
  o derecho sin `panic`, sin alterar posiciones y sin escribir sobre las barras;
- la barra contextual aparece encima de la barra de acciones y muestra el
  mensaje normal cuando el jugador puede actuar;
- el resumen NPC aparece en la barra contextual sobre la vista del mundo y la
  barra de acciones refleja la disponibilidad de esa misma `VistaMundo`, sin una
  pantalla de confirmación;
- con un jugador capaz de actuar, abrir la selección oculta temporalmente el
  resumen NPC, cancelarla vuelve a mostrarlo y aceptar el viaje permite que la
  aplicación lo descarte;
- un reloj falso comprueba el primer fotograma, los límites exactos de 250 ms,
  el ciclo y un vencimiento tardío que salta fotogramas sin renderizarlos en
  ráfaga;
- el reloj falso comprueba que la confirmación congela el fotograma y el tiempo
  restante, y que al cancelarla no cuenta su duración como vencimientos
  atrasados;
- el reloj visual no cambia la fecha del Mundo ni invoca una acción;
- `TestBackend` recorre pantallas completas sin una terminal real.

Pruebas manuales en Manjaro comprobarán color, teclas, fluidez, acentos,
redimensionado y restauración después de salir o provocar un error controlado.
La paleta se revisará al menos con un tema ANSI oscuro habitual y otro de
contraste reducido; ninguna información indispensable podrá depender de que
ambos muestren el mismo tono.

No quedan preguntas bloqueantes internas en este componente. La implementación
se aceptará cuando el frontend pueda completar el recorrido de humo usando solo
la fachada de aplicación y `cargo tree` confirme que Ratatui y Crossterm no
aparecen en los otros crates.

## 12. Preguntas bloqueantes

Ninguna en este componente.

## 13. Decisiones aplazadas

- entrada mediante ratón;
- ilustración de actores ajenos al Grupo sobre el mapa;
- duraciones diferentes por fotograma y animaciones distintas por estado;
- presentación de grupos numerosos después de incorporar `Reclutar`;
- menús de opciones paginados para decisiones que no quepan en una sola barra;
- refinamientos de geometría, bordes y acabado artístico posteriores a la
  validación manual de la primera versión jugable;
- frontends gráficos 2D o 3D;
- apariencias concretas de los tipos de tile oficiales, que se decidirán con el
  setting;
- carga de temas alternativos o sustitución de catálogos y paletas visuales
  durante la ejecución.

## 14. Referencias

- [Backends de Ratatui](https://ratatui.rs/concepts/backends/)
- [Renderizado de Ratatui](https://ratatui.rs/concepts/rendering/under-the-hood/)
- [Ejemplo síncrono de Ratatui con espera hasta el próximo tick](https://ratatui.rs/examples/apps/demo/)
- [Manejo de eventos en Ratatui](https://ratatui.rs/concepts/event-handling/)
- [`Clear` de Ratatui para superposiciones](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html)
- [`Block` de Ratatui para bordes y estilos](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html)
- [`Color` de Ratatui](https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html)
- [`Modifier` de Ratatui](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html)
- [`TestBackend` de Ratatui](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html)
- [Eventos de Crossterm](https://docs.rs/crossterm/latest/crossterm/event/)
