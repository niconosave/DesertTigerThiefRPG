# Frontend de terminal con Ratatui

| Campo | Valor |
|---|---|
| Estado | Implementado parcialmente; proyecto humano y ampliación causal aprobados para implementar |
| Versión | 0.44 |
| Fecha | 2026-10-25 |

## Revisión implementada el 2026-10-24: Presentación por pasos

El layout, arte, controles, transparencia y animación se conservan. La terminal recibe la proyección pública del paso actual; mundo_visible conserva sus participantes. Paginar, consultar detalle y redimensionar no ejecuta reglas. Se retiraron pronósticos previos al plan; el progreso usa el mismo tiempo exacto de la fecha final.

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

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
- presentación del propósito, deliberación, compromiso, subobjetivos e
  interpretaciones de un proyecto humano;
- presentación sucesiva y paginada de la cronología causal;
- seguimiento ilustrativo del Lugar y la fecha proyectada durante `Viajar`;
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
Aplica por separado las tablas Durdraw de frente y fondo; en este flujo de
retratos, el fondo negro predeterminado se convierte en transparencia y los
otros fondos admitidos permanecen explícitos.

## 4. Responsabilidades internas

El crate separará conceptualmente estas partes, sin convertirlas en nuevos
servicios:

- **ciclo de terminal:** entrar y salir de modo raw y pantalla alternativa;
- **bucle de eventos:** combinar entrada, redimensionado y vencimientos de
  animación;
- **adaptador de entrada:** convertir teclas en intenciones según el modo visual;
- **estado de interfaz:** conservar modo, foco, cámara y fotograma de animación;
- **catálogo visual:** cargar la paleta semántica, asociar claves de tipo de tile
  con glifos y estilos, y claves locales de arte con ilustraciones ASCII propias
  del frontend;
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

La aplicación entrega un fragmento narrativo semántico por vez, con texto, rol
cronológico, transición de proyecto opcional, pertenencia opcional a una escena
contextual, progreso y detalle causal disponible. Las decisiones incluyen el
progreso y pueden incluir la misma referencia de situación. La fachada nunca
entrega arte, coordenadas ni rutas. El frontend convierte cada elemento en una
viñeta visual y lo pagina si no cabe, sin solicitar el siguiente fragmento hasta
haber mostrado el actual completo. De ese modo, el backend fija texto,
transiciones humanas, agrupación en escenas, orden, causalidad, claves
semánticas y límites de decisiones, mientras la terminal conserva el control
del marco, tamaño, paginación, color, arte y composición de cada pantalla.

El [`proyecto humano de Viajar`](../acciones/04-proyecto-humano-viajar.md) es
también normativo. La terminal presenta sus registros de propósito, propuesta,
compromiso, subobjetivo, interpretación y replanteamiento mediante las mismas
ventanas, barras y controles ya definidos. No añade una pantalla o modo visual
exclusivo, no inventa pensamientos de los personajes y no convierte una
deliberación sin acción particular en una tirada o resultado mecánico.

La [cronología causal de `Viajar`](../acciones/03-cronologia-causal-viaje.md) es
normativa: no existe un máximo, presupuesto ni muestreo visual de escenas. La
terminal presenta en orden todos los fragmentos recibidos y respeta las escenas
y momentos causales que compuso la acción general. Una escena representa una
situación dramática contextual, no una clase ni una ejecución de acción
particular: puede contener momentos de varias acciones particulares, y una misma
ejecución puede desarrollarse mediante varias viñetas. El frontend no crea,
separa, fusiona ni reagrupa esas unidades a partir de su identidad mecánica. La
paginación puede dividir una viñeta demasiado extensa, pero no cambia su escena
ni confirma el fragmento antes de mostrar todas sus páginas.

La exhaustividad se recibe ya resuelta por la aplicación: cada traza ejecutada
pertenece a exactamente un momento causal reconocible y todos esos momentos
llegan ordenados, aunque compartan escena o tipo de acción particular. La
terminal no omite fragmentos rutinarios, automáticos o repetidos y tampoco exige
la correspondencia artificial de una escena por traza. El carácter emotivo
surge del guion contextual recibido; el frontend no asigna una categoría o
enumeración de tono ni reescribe la prosa para producirla.

El resumen NPC llega como `VistaResumenNpc`, ya redactado por la aplicación a
partir de `ResumenNpc`. La terminal solo aplica el rol
normal o de incidencia recibido y ajusta el texto a la barra contextual; no
elige qué acciones agrupar, no calcula la duración simulada, no deduce
consecuencias ni intenta enumerar cada ejecución en una sola fila. La fase NPC
no abre viñetas interactivas.

El frontend no elige qué actor actúa. Mover el foco entre objetivos es una
operación visual local; al confirmar, envía comenzar una acción con el ID del
objetivo o responder una opción con su ID. Nunca envía una coordenada visual
para expresar el destino de un viaje.

La aplicación ofrece posibilidades para construir la interfaz y vuelve a
validar toda solicitud. En particular, `Viajar { destinos }` ya contiene
exclusivamente los Lugares que el núcleo determinó alcanzables mediante
Relaciones `conectado` y las demás reglas vigentes de transitabilidad. El
frontend conserva esa lista y su orden: no inspecciona Relaciones, no busca
caminos, no agrega o elimina destinos y no interpreta por sí mismo
Características del Mundo como `muerto`, `acuatico` o `muelle` para autorizar
una acción.

La ruta calculada para un viaje es contexto privado de la acción y no atraviesa
la vista de aplicación. La terminal permite elegir solamente el destino final;
no dibuja conexiones ni trayectos sobre el mapa y no intenta reconstruirlos a
partir de tiles, coordenadas o de la lista de destinos.

`desert-tiger-terminal` tampoco decide cómo un resultado del mundo transforma
los tiles vigentes durante la Partida. `desert-tiger-app` posee ese estado del
mapa y el frontend se limita a dibujar la vista que recibe.

La terminal resuelve cada `ClaveTipoTile` mediante su catálogo visual privado.
La apariencia resultante contiene como mínimo un glifo de una celda y un estilo
de Ratatui. La terminal no recibe ni reconstruye la leyenda del archivo de Mapa
y nunca usa su símbolo de autoría como glifo implícito. Un frontend 2D o 3D podrá
asociar la misma clave con un recurso completamente distinto. La lista de tipos
declarados permite comprobar o preparar esas asociaciones aunque un tipo todavía
no aparezca en la matriz vigente.

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
glifo es un único `char` que no es de control y cuyo ancho terminal, calculado
con `unicode-width`, es exactamente una celda; esto admite tanto ASCII como
Unicode compatible y excluye caracteres combinantes, anchos y secuencias emoji.
El estilo se declara directamente dentro de cada apariencia y no referencia
otro nombre: `frente` es obligatorio, `fondo` es `None` o `Some(color)` y
`modificadores` es una lista sin duplicados cuyo orden no cambia el resultado.

Los únicos nombres admitidos son los diecisiete colores de la paleta inicial. La
conversión normativa es:

| Valor RON | `ratatui::style::Color` |
|---|---|
| `negro` | `Rgb(0, 0, 0)` |
| `rojo` | `Red` |
| `verde` | `Green` |
| `amarillo` | `Yellow` |
| `azul` | `Blue` |
| `azulOscuro` | `Rgb(0, 24, 64)` |
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
particular, `negro` es negro RGB puro y `azulOscuro` es el azul marino fijo
`#001840`; ninguno depende de los equivalentes configurables del tema de la
terminal. `gris` representa el color ANSI base `Gray` y `blanco` el brillante
`White`. `negrita` nunca se interpreta como una solicitud de color claro.

Los únicos modificadores admitidos son `negrita`, `tenue` e `invertido`, que se
convierten respectivamente en `BOLD`, `DIM` y `REVERSED`. `negrita` y `tenue` no
pueden aparecer juntas porque expresan intensidades contradictorias;
`invertido` puede combinarse con cualquiera de las dos. Cuando `fondo` es
`None`, el estilo no sustituye el fondo de la capa existente e `invertido` usa
ese fondo heredado. El RON no admite valores RGB o indexados arbitrarios,
`Reset`, color de subrayado, parpadeo, texto oculto, cursiva, subrayado ni
tachado en el primer incremento. Los RGB fijos asociados a `negro` y
`azulOscuro` son decisiones del frontend y no permiten declarar valores RGB
arbitrarios en el esquema editable.

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
        fondo: Some("negro"),
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

`escena` usa el nombre externo `lowerCamelCase` asignado a una variante del enum
local `ClaveArteEscena`. Este tipo pertenece solamente a
`desert-tiger-terminal`; no atraviesa la fachada y no expresa reglas,
agrupamiento narrativo ni emoción:

| Variante local | Nombre exacto en `scenes.ron` | Uso visual vigente |
|---|---|---|
| `ClaveArteEscena::ViajeMarcha` | `viajeMarcha` | Apertura, avance terrestre o travesía acuática. |
| `ClaveArteEscena::ViajeObstaculo` | `viajeObstaculo` | Paso abrupto o crisis náutica. |
| `ClaveArteEscena::ViajeLlegada` | `viajeLlegada` | Conclusión que alcanza el destino final. |
| `ClaveArteEscena::ViajeRumboIncierto` | `viajeRumboIncierto` | Situación de rumbo incierto. |

Este documento es la autoridad de los nombres externos y de la conversión. El
adaptador convierte de forma exhaustiva entre `ClaveArteEscena` y estos nombres
exactos, sensibles a mayúsculas; no admite alias, variantes en `snake_case` ni
otras grafías. Esa conversión define todo el vocabulario reconocido, pero
`scenes.ron` puede contener cualquier subconjunto de él, incluido ninguno: la
ausencia de una entrada significa que esa escena se presenta solamente con
texto. Una clave desconocida o repetida dentro del archivo es un error de
inicialización. El orden de las entradas no afecta el resultado.

La terminal elige opcionalmente una clave de arte mediante un `match` exhaustivo
sobre datos semánticos recibidos, sin analizar la prosa:

| Vista semántica | Clave local resultante |
|---|---|
| `RolPresentacion::Apertura` | `Some(ViajeMarcha)` |
| situación `AvanceTerrestre` o `TravesiaAcuatica` | `Some(ViajeMarcha)` |
| situación `PasoAbrupto` o `CrisisNautica` | `Some(ViajeObstaculo)` |
| situación `RumboIncierto` | `Some(ViajeRumboIncierto)` |
| `RolPresentacion::Conclusion` con Lugar confirmado igual al destino final | `Some(ViajeLlegada)` |
| otra situación o conclusión | `None` |

Esta tabla es una elección artística del frontend y no modifica la escena. Una
nueva variante de `ClaveSituacionViaje` obliga a decidir explícitamente su
asociación local, que puede ser `None`. La igualdad entre Lugar confirmado y
destino solo selecciona una ilustración: la conclusión ya fue resuelta por la
aplicación. `scenes.ron` puede omitir cualquier entrada y, en ese caso, la
viñeta se presenta solo con texto.

`arte` es un bloque estático, rectangular y no vacío, de glifos Unicode de una
columna, incluidos ASCII y los bloques usados en los dibujos. Se valida cada
carácter mediante `UnicodeWidthChar::width() == Some(1)` y se rechazan controles.
Los saltos de línea separan filas; no se permiten
tabulaciones, controles ni secuencias de escape. Todas las filas tienen el mismo
ancho y los espacios finales cuentan. Para permitir la escritura multilínea
cómoda, el cargador elimina como máximo un salto de línea inmediatamente después
de abrir la cadena raw y otro inmediatamente antes de cerrarla, normalizando
previamente CRLF a LF; no recorta ningún otro carácter.

El catálogo no contiene texto narrativo, rutas, reglas, condiciones, tiempos,
fotogramas ni estilos. Una `ClaveArteEscena` sin entrada es válida y produce una
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
| Presentación de acción | Superponer a la última vista del mundo cada viñeta del proyecto humano o de sus escenas narrativas y mostrarlas en sucesión. |
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
| Presentación de acción | Identifica la transición del proyecto o la escena contextual, el Lugar confirmado, la fecha proyectada, el tiempo transcurrido y el sentido de la viñeta actual. | Ofrece `Continuar`, el control local de detalles cuando exista y `Salir` al final. |
| Decisión | Explica brevemente qué se está decidiendo y conserva el progreso previo a resolver. | Enumera los controles u opciones vigentes, el control local de detalles cuando exista y `Salir` al final. |
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
juego. El modo semántico, el foco, las páginas vigentes de texto y opciones y la
instantánea visual se conservan internamente. Al recuperar ambas dimensiones
mínimas, se recalcula la
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
fuente inmutable de mapa, rótulos y retratos. Durante los modos de presentación
y decisión vuelve a dibujar esos datos y coloca la ventana encima. No consulta
ni reconstruye un Mundo vivo y no proyecta consecuencias futuras aunque el
núcleo ya las haya calculado. En cambio, el fragmento vigente aporta su propia
instantánea cronológica de progreso: con ella la terminal cambia cámara,
resaltado, marcador y textos de fecha sin afirmar que el estado final del Mundo
ya fue aplicado. Cuando la aplicación vuelve a entregar `VistaMundo` después de
la fase NPC, la instantánea anterior se descarta.

La inmutabilidad anterior se refiere a los datos de Mundo recibidos de la
aplicación. El frontend puede seleccionar otros fotogramas de los retratos y
usar el `ProgresoViaje` vigente, por lo que la animación y el seguimiento
continúan detrás de una viñeta sin revelar consecuencias posteriores.

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
calculado. A continuación rellena todas sus celdas con `pantalla` y dibuja el
`Block` con `marco`. El título superior combina el nombre o situación dramática
y el rol recibidos, por ejemplo `El plan del grupo · Compromiso`, `Viento
abierto · Presentación`, `El paso bajo el sol · Desenlace` o `Decisión`. No
deriva ese título del nombre de una acción particular. El título inferior
muestra el nombre del Lugar confirmado, la fecha proyectada y el tiempo
acumulado; puede añadir el siguiente Lugar cuando la vista lo aporta. Ambos usan
datos ya redactados por la aplicación y el rol `titulo`; no interpretan IDs,
reglas ni tono. El borde podrá usar un conjunto de caracteres personalizado;
sus glifos concretos se decidirán con el resto de la identidad artística.

La confirmación de salida reutiliza esa composición superpuesta, sin ilustración
ni paginación. Su título es `Salir` y su texto fijo es
`La partida actual no se guardará. ¿Quieres salir del juego?`. Conserva como
fondo el modo visual suspendido y usa las mismas reglas de centrado, marco,
relleno y ancho mínimo; su altura es el mínimo de siete filas. Si se abrió desde
`Terminal insuficiente`, no intenta reconstruir las barras o el mundo: limpia el
búfer y muestra una versión compacta del título, la advertencia y los controles,
recortada de forma segura a las celdas disponibles.

El estado suspendido incluye modo, foco, cámara, páginas de texto y opciones,
instantánea, resumen NPC, rechazo recuperable y fotograma de retrato. La
animación queda congelada y
conserva el tiempo que faltaba para su siguiente paso; al cancelar continúa desde
ese punto sin contar como atrasado el tiempo pasado en la confirmación. Un
redimensionado recalcula solamente la composición exterior: no descarta la
confirmación ni el estado que contiene.

Toda viñeta contiene obligatoriamente el texto narrativo entregado por la
aplicación. Si su referencia de situación se asocia con una `ClaveArteEscena`
que tiene entrada en `scenes.ron`, el frontend puede añadir su ilustración ASCII
estática. Un registro de proyecto sin situación, una asociación `None` o una
clave deliberadamente no representada produce una viñeta de solo texto y nunca
es un error de gameplay.
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

En `Presentación de acción`, con el detalle cerrado, `Enter` avanza solamente la
página narrativa local mientras quede otra. En la última página solicita a la
aplicación confirmar el fragmento; un único evento de tecla nunca puede cambiar
de página y confirmar además el fragmento. Cuando el fragmento contiene un
desglose, la barra añade `[D] Detalles` o `[D] Ocultar detalles`. Narración y
detalle conservan paginadores separados: al abrirlo, `D` calcula su primera
página sin cambiar la página narrativa; mientras permanece abierto, `Enter` solo
recorre sus páginas y en la última no confirma ni avanza. `D` lo cierra y
restaura exactamente la página narrativa subyacente. La barra termina con
`[Q] Salir`. Las reglas de lectura y opciones para una decisión paginada se
especifican por separado.

La vista normal nunca depende de abrir el detalle para explicar el resultado:
el texto recibido sitúa el momento dentro de la escena, comunica su carácter
dramático y nombra las causas conocidas y consecuencias observables pertinentes.
El panel expandido presenta los factores, dificultad, dado, modificador, margen,
banda, duración y cambios que la aplicación marcó como visibles. La terminal no
completa totales, no consulta Características y no revela campos reservados.
Alternar el panel no llama a la aplicación, no consume azar, no cambia la fecha
y no crea otra viñeta, escena ni momento causal.

Una decisión emplea la misma ventana para su texto y contexto; sus opciones se
escriben en la barra de acciones. Su texto, ilustración e indicador siguen las
mismas reglas de ajuste y paginación que un fragmento. Mientras no se haya
mostrado la última página del texto, la barra contiene `[Enter] Continuar`, el
control `[D]` solamente cuando existe detalle común y `[Q] Salir`; las teclas de
opción están inactivas y `Enter` solo avanza una página local. En la última
página desaparece `Continuar` y se habilita la primera página local de opciones.
`Enter` por sí solo no elige ninguna.

En el primer vertical implementado, cada página visual de opciones contiene una
sola opción completa. Es una subdivisión válida de los bloques lógicos de hasta
24: prioriza que la etiqueta, los controles y su detalle sean legibles en
`80 × 24`, y deja una optimización posterior agrupar varias opciones cuando el
ancho disponible lo permita. La ventana conserva el texto común; al abrir `[D]`
añade únicamente el desglose de la opción visible.

Las opciones conservan el orden entregado por la aplicación. El frontend las
parte primero en bloques lógicos contiguos de hasta 24, la cantidad de letras
ASCII disponibles de `A` a `Z` después de reservar `Q` y `D`. Dentro de cada
bloque, para cada opción busca de izquierda a derecha la primera letra ASCII de
su texto que no haya sido asignada en ese bloque ni sea `Q` o `D`, sin distinguir
mayúsculas. Si no existe, usa la primera letra libre de `A` a `Z` con las mismas
reservas. Cada opción se escribe como `[X] Texto`; la `X` usa `atajo`, se deja un
espacio al comienzo y al final de la barra y dos espacios entre opciones y antes
de `[Q] Salir`. No se envuelve, abrevia ni recorta una opción.

Cada bloque lógico se subdivide después en páginas visuales contiguas que caben
en la barra; nunca se combinan opciones de dos bloques aunque sobre espacio. Se
reserva lugar para `[←]`, `[→]`, `[D]` cuando la decisión posea algún detalle y
`[Q] Salir`. Las flechas cambian solo la página local de opciones. Los atajos se
asignan al bloque antes de medir ancho: por eso una opción conserva su tecla al
abrir `[D]`, recorrer páginas o redimensionar la terminal, y una letra solo puede
reutilizarse en otro bloque lógico. La barra contextual conserva la explicación
de qué se está eligiendo y le añade `Opciones i–j de n`. No hay un máximo
reglado de propuestas: una lista extensa crea más bloques y páginas sin alterar
lo que recibe el motor o la política NPC.

La aplicación garantiza etiquetas no vacías de hasta 40 columnas visibles. El
frontend vuelve a validar que cada etiqueta individual quepa completa en una
página de 80 columnas junto con los controles indispensables. Una violación
produce un error tipado de presentación; nunca se envuelve, abrevia, recorta ni
escribe fuera del búfer. Tener muchas opciones, en cambio, solo produce más
bloques y páginas y no es un error.

`D` queda reservada antes de asignar atajos dinámicos, igual que `Q`. En las
páginas narrativas, `[D] Detalles` aparece únicamente si
`VistaDecision::detalle` es `Some` y muestra sus líneas de contexto común. En la
etapa de opciones aparece en todas sus páginas si existe ese detalle común o al
menos un `VistaOpcionDecision::detalle` en la decisión. El panel expandido
muestra primero el contexto común, si existe, y luego un bloque por cada opción
de la página visible, en el mismo orden, encabezado por `[X] etiqueta`; un bloque
sin detalle propio dice `Sin desglose adicional.`. Nunca muestra en esa página
el detalle de opciones pertenecientes a otra página.

El contenido expandido posee su propia paginación vertical. Mientras está
abierto, `Enter` solo avanza esa paginación y se detiene en su última página;
nunca avanza la narración ni elige una opción. `D` lo cierra. En la etapa de
opciones, las flechas cambian la página visual y reinician el detalle en su
primera página, manteniéndolo abierto; los atajos de las opciones visibles
siguen activos porque el texto narrativo ya terminó. Antes de esa etapa no hay
atajos de opción activos. Abrir, cerrar o paginar el detalle no llama a la
aplicación ni altera la partición, los IDs o los atajos.

Todo detalle anterior a resolver muestra únicamente contexto, costes, ventajas,
procedencias, precondiciones e incertidumbre conocidos; nunca una tirada o un
desenlace futuro.

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

La composición no contiene una capa de rutas o conexiones. Durante la selección
solo cambia el foco del rótulo y la cámara; los tiles situados entre el origen y
el destino se dibujan igual que en la vista normal. Durante una expedición se
dibuja un único marcador ilustrativo `@` sobre la coordenada central del Lugar
confirmado por el progreso vigente y se resalta ese rótulo. El marcador no
indica un tile de gameplay ni se devuelve a la aplicación.

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
`Selección de destino`, el Lugar enfocado; durante una presentación o decisión
usa `lugar_confirmado` del progreso de ese elemento. Cancelar la selección
devuelve el objetivo al Lugar del jugador. No existe desplazamiento libre de
cámara en el primer incremento. Cambiar de fragmento recalcula la cámara solo
cuando cambia el Lugar confirmado; una introducción que señala el siguiente
Lugar no adelanta todavía el objetivo.

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

El frontend dibuja primero a los compañeros en el orden recibido de `VistaMundo`
y al final al jugador, que queda a la derecha del grupo. Cada retrato posterior comienza más a la derecha que el
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
recursos de tiles. En esa conversión, `Negro` produce `Rgb(0, 0, 0)` y
`GrisMuyOscuro` produce `Rgb(32, 32, 32)`; los demás colores conservan su
variante ANSI equivalente. Un fondo ausente conserva el fondo que ya aportó el
mapa. El orden de capas mantiene la transparencia y permite que los bloques
parciales de Durdraw combinen frente y fondo dentro de una celda.

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
Lugar, además de explicar que se está eligiendo adónde viajar. Cuando la
aplicación aporta un pronóstico, añade su rango de días y describe por separado
la exposición conocida y la incertidumbre. Solo puede decir `sin riesgos
conocidos` cuando `exposicionConocida == 0`; esto no elimina ni oculta una
incertidumbre positiva. En listas o menús
futuros, la fila enfocada comenzará también con `>`. La lista de candidatos
procede de la aplicación ya filtrada por conectividad y transitabilidad. Los
lugares no disponibles siguen dibujados como parte del mundo, pero no reciben
foco ni pueden confirmarse. El frontend no distingue si fueron excluidos por
falta de ruta o por otra regla. El foco inicial y el orden de navegación usan la
secuencia entregada por la aplicación; las flechas y el comportamiento en los
extremos siguen el contrato de entrada de la sección 9.

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
| Presentación de acción, detalle cerrado | `Enter` | Avanza la página narrativa local o, en su final, solicita continuar la presentación. |
| Decisión, detalle cerrado y antes de sus opciones | `Enter` | Avanza solamente la página narrativa local; nunca selecciona una opción. |
| Detalle abierto de fragmento o decisión | `Enter` | Avanza solo la página local del detalle y se detiene en la última; nunca confirma ni responde. |
| Presentación de acción o decisión con desglose | `D` | Muestra u oculta localmente los detalles conocidos del fragmento, contexto o página de opciones sin confirmar nada. |
| Decisión en su última página de texto | `←` / `→` | Recorre páginas locales de opciones sin envolver los extremos ni responder. |
| Decisión | atajo `[X]` antepuesto a una opción | Envía la opción correspondiente a la aplicación. |
| Presentación de acción o decisión | `Esc` | No produce efecto: no cancela ni abandona un proyecto ya iniciado. |
| Todo modo salvo confirmación de salida | `Q` | Abre la confirmación local de salida sin enviar una intención de juego. |
| Confirmación de salida | `S` | Confirma la salida sin guardar y comienza el cierre normal del frontend. |
| Confirmación de salida | `N` o `Esc` | Cancela la salida y restaura el modo visual suspendido. |

Las teclas alfabéticas se aceptan sin distinguir mayúsculas de minúsculas. Cada
acción u opción visible recibe un atajo no ambiguo, mostrado como `[X]` antes de
su rótulo. La asignación es estado del frontend y no forma parte de la regla ni
del identificador semántico enviado a la aplicación. `Q` queda reservada para
`[Q] Salir` y `D` para el detalle contextual: se consideran ocupadas antes de
asignar cualquier atajo dinámico y no pueden representar una acción general ni
una opción de decisión. Si una escena no ofrece desglose, pulsar `D` no produce
efecto y la barra no anuncia el control.

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

`Esc` nunca representa `Abandonar`. Esa conclusión solo puede elegirse mediante
la opción semántica visible `Abandonar` que el núcleo emite en una frontera
segura de replanteamiento; `Q` abandona el programa completo, no el proyecto de
viaje.

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

Cada glifo elegido por el catálogo de terminal para un tile será un único
`char` no de control de exactamente una celda según `unicode-width`; puede ser
un espacio. Esta restricción se aplica a la apariencia terminal, no a los
símbolos compactos del archivo de Mapa, que ya fueron resueltos antes de
construir `VistaMapa`. Los textos podrán contener acentos; el renderizado deberá
medir su ancho visible en celdas y recortarlos sin romper la composición. El
color reforzará el estado, pero una selección también deberá reconocerse por el
texto o la forma, no solo por el color.

## 10. Errores y validaciones

La aplicación es la fuente normativa de la categoría de cada problema. El
frontend no inspecciona Lua, no reinterpreta un `ErrorIntencion` y no decide si
un fallo de política NPC permite continuar: recibe un error de inicio, un
rechazo recuperable de intención, un `VistaResumenNpc` normal o de incidencia,
o una `VistaSesion::Fatal` según los contratos de aplicación y
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
`Error:` y el rol `error`. La terminal conserva el modo, el foco, la cámara, las
páginas locales de texto y opciones y la vista de aplicación vigentes; no aplica
una mutación local para simular que la solicitud tuvo éxito. El mensaje
sustituye temporalmente el texto contextual normal y permanece hasta que la
aplicación acepta otra intención o el usuario cancela o cambia el modo visual
local. Abrir `Confirmación de salida` es
la excepción: el mensaje forma parte del estado suspendido y reaparece intacto si
se cancela la salida.

El resumen será breve y orientado a la acción que el jugador puede tomar. Si no
cabe en una fila, se recorta por ancho visible y termina en `...`, sin separar un
carácter de ancho múltiple ni perder el prefijo `Error:`. Los detalles técnicos
no se vuelcan dentro de la interfaz de juego.

### Incidencias recuperables de NPC

`ClaseVistaResumenNpc::Normal` usa el rol normal de la barra contextual.
`ClaseVistaResumenNpc::Incidencia` es recuperable, pero se presenta con el
prefijo `Error:` y el rol `error`, igual que un rechazo de intención. No restaura
un modo anterior, no abre una viñeta y no impide elegir la siguiente acción.

La terminal usa exclusivamente el texto seguro recibido. No muestra el bloque
Lua, una traza, `Debug`, IDs numéricos ni un detalle técnico completo dentro de
la barra; la aplicación incluye en la frase la clave de Actor y el código estable
necesarios para reportar una incidencia.

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

Antes de mostrar texto procedente de contenido o de un componente interno, el
presentador aplica una de dos políticas explícitas:

- los campos atómicos —nombres, títulos, etiquetas, resumen NPC, rechazos y
  diagnósticos— se convierten en una sola línea segura;
- los cuerpos narrativos —`VistaFragmento::texto`, `VistaDecision::texto` y los
  detalles— pueden conservar `LF` intencionales y líneas vacías para su ajuste y
  paginación, pero ningún otro control.

En ambos casos escapa como notación visible todo `char` para el que
`is_control()` sea verdadero, salvo el `LF` permitido en un cuerpo narrativo, y
los controles bidireccionales Unicode `U+202A..U+202E` y `U+2066..U+2069`. Usa
`\n`, `\r` y `\t` para esos tres controles cuando deban escaparse y
`\u{CODIGO}` para los restantes. El formateador es el único que interpreta los
`LF` narrativos o añade separaciones entre campos. De este modo, un nombre o
mensaje modificado no puede inyectar ANSI, borrar la pantalla ni falsear
visualmente el orden del diagnóstico, mientras una viñeta conserva su
composición autoral.

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
  comandos visibles a la vez; `Q` y `D` nunca se asignan dinámicamente y `S` o
  `N` siguen disponibles fuera de la confirmación;
- `←` y `→` recorren destinos sin envolver los extremos, `Enter` confirma y
  `Esc` cancela la selección solamente antes de iniciar la acción;
- `Enter` pagina localmente un fragmento y solo envía su confirmación después de
  mostrar la última página;
- una secuencia sintética con más de seis escenas y viñetas conserva y presenta
  todas en orden, incluida una escena con varias acciones particulares, una
  ejecución desarrollada en varias viñetas y dos ejecuciones rutinarias
  consecutivas iguales;
- una secuencia de proyecto presenta en orden propósito, propuesta, compromiso,
  subobjetivo, interpretación y replanteamiento mediante el mismo modo de
  presentación, sin fabricar una escena, momento causal o detalle mecánico para
  las transiciones que no ejecutaron una acción particular;
- una propuesta de camellos nombra al Actor que aporta `conCamellos`, y una
  propuesta náutica nombra el Lugar y su servicio `puerto` o `muelle`; la
  terminal no reemplaza esas fuentes por una explicación genérica ni ofrece un
  medio que la vista no contiene;
- `D` expande y oculta el desglose conocido sin confirmar, responder, consumir
  azar ni llamar a la aplicación; `Enter` pagina solo ese desglose mientras está
  abierto y se detiene en su última página;
- una decisión paginada mantiene inactivas sus opciones hasta la última página
  de texto, donde desaparece `Continuar`; `[D]` permanece cuando corresponde y
  `[Q] Salir` está disponible en todas las etapas;
- una decisión con detalle común y detalles distintos por opción muestra el
  común durante su texto y, al llegar a las opciones, únicamente los bloques de
  la página visible; cambiar de página reinicia el detalle sin omitir, duplicar
  ni anticipar información;
- los atajos de opción se asignan de forma determinista dentro de cada bloque
  mediante la primera letra ASCII libre distinta de `Q` y `D`, y luego mediante
  la primera de `A` a `Z` disponible; cada bloque contiene como máximo 24
  opciones, otra letra solo se reutiliza en otro bloque y una opción conserva su
  tecla ante paginación, detalle y redimensionado;
- la deliberación de plan, preparación, ritmo, Paso empinado y crisis náutica
  usan el mismo adaptador genérico y conservan textos, orden e identificadores
  recibidos; el caso Paso empinado produce los atajos naturales `[A]` y `[B]`;
- una decisión con más de 24 opciones y opciones suficientes para ocupar varias
  barras conserva atajos y orden al recorrerlas con flechas, mientras una
  política NPC sigue viendo la lista completa; una etiqueta vacía, de más de 40
  columnas o que excede el ancho útil devuelve un error tipado sin escribir
  fuera del búfer;
- una vista de jugador vivo con destinos muestra `[V] Viajar` y `[Q] Salir`, y
  `V` abre la selección sin alterar, completar ni volver a filtrar la lista
  recibida;
- la selección solo puede enfocar y confirmar los IDs presentes en la lista ya
  filtrada; un Lugar visible pero omitido no se incorpora aunque sus tiles sean
  contiguos al origen;
- al enfocar un destino, el búfer no incorpora una línea, glifos ni estilos de
  ruta entre Lugares: solo cambian la cámara, el rótulo enfocado y las barras;
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
- las viñetas de una escena, una decisión y la confirmación de salida se
  superponen al fondo correspondiente sin cubrir las barras ni revelar una
  actualización prematura;
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
  el borde, muestra título de situación y rol en la parte superior y Lugar,
  fecha proyectada y tiempo transcurrido en la inferior; `Salir` conserva su
  título propio;
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
- las variantes locales de `ClaveArteEscena` se convierten exactamente en
  `viajeMarcha`, `viajeObstaculo`, `viajeLlegada` y
  `viajeRumboIncierto`, y cada una puede resolver una ilustración distinta sin
  alterar el texto recibido;
- las situaciones `AvanceTerrestre`, `PasoAbrupto`, `TravesiaAcuatica`,
  `CrisisNautica` y `RumboIncierto`, además de apertura y llegada, producen la
  asociación local definida por la tabla; las demás producen `None`;
- una decisión dentro de `PasoAbrupto` resuelve `viajeObstaculo` sin alterar su
  texto, opciones ni comportamiento, y continúa siendo válida como solo texto
  cuando el catálogo no contiene esa ilustración;
- la asociación visual no cambia el `IdSituacion`, no crea otra escena y no
  clasifica su emoción;
- `scenes.ron` admite una lista vacía y rechaza claves desconocidas, duplicadas,
  alias, diferencias de mayúsculas, grafías `snake_case`, bloques vacíos, no
  rectangulares, con glifos de ancho distinto de una columna o con controles;
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
- los diecisiete nombres de color se convierten a la variante exacta de Ratatui,
  incluidos `negro` como `Rgb(0, 0, 0)` y `azulOscuro` como
  `Rgb(0, 24, 64)`; `None` no sustituye el fondo de la capa existente y se
  rechazan `Reset`, valores RGB editables, índices, alias y nombres desconocidos;
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
- un resumen NPC normal usa el rol normal de la barra, mientras una incidencia
  conserva el prefijo `Error:` y el rol `error` sin abrir otro modo ni impedir la
  siguiente acción;
- un resumen NPC derivado de más de seis acciones conserva una forma acotada:
  presenta Actor, conclusión, propósito/destino, ubicación y duración simulada
  primero; cuenta acciones y replanteamientos; y nombra solo el último replanteamiento y la
  última consecuencia persistente con cantidades para las anteriores, sin
  convertir la cronología en viñetas, avanzar la fecha ni volver a ejecutar
  reglas;
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
- nombres, títulos, etiquetas y resúmenes con `LF` se escapan como campos
  atómicos, mientras un cuerpo narrativo conserva sus `LF` autorales y escapa
  `CR`, tabulaciones, `ESC` y controles bidireccionales;
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
- durante `Viajar`, cada fragmento centra la cámara en su Lugar confirmado,
  resalta su rótulo y dibuja `@` en su coordenada central; indicar un siguiente
  Lugar no adelanta el marcador antes de la consecuencia que confirma el avance;
- la fecha y duración visibles proceden del progreso recibido, avanzan en un
  salto completo ante una acción multidiaria y coinciden con la vista final sin
  que el frontend mute el calendario;
- un destino con exposición conocida positiva nunca se describe como `sin
  riesgos conocidos`; incertidumbre y exposición se presentan por separado;
- objetivos centrales y próximos a cada uno de los cuatro bordes producen el
  origen limitado por la fórmula, sin solicitar celdas exteriores;
- un mapa menor que la región se centra en cada eje y coloca cualquier celda
  impar sobrante en el margen derecho o inferior con el rol `pantalla`;
- cancelar una selección restaura el objetivo del jugador, una viñeta usa la
  cámara de su propio progreso y un redimensionado recalcula el origen vigente;
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
- el primer compañero comienza en el borde izquierdo y el jugador queda a la
  derecha de los compañeros; si está solo, comienza en el borde izquierdo. Todos los retratos se
  apoyan en la fila inmediatamente superior a la barra contextual y ninguno
  invade las barras;
- cada retrato sucesivo se desplaza hacia la derecha y, en las celdas donde se cruza
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
Un recorrido terrestre largo y otro marítimo deberán mostrar por qué viaja el
Grupo, qué alternativas materialmente posibles consideró, qué plan adoptó, de
dónde obtiene sus medios, qué intenta resolver en cada episodio y cómo interpreta
los cambios. También deberán ofrecer escenas con carácter dramático acorde a sus
situaciones y permitir reconstruir, sin abrir el detalle, qué acciones
particulares intervinieron, qué causas conocidas determinaron sus desenlaces,
dónde quedó el Grupo y cuánto tiempo había transcurrido. Esto debe seguir siendo
cierto cuando una escena abarque varias acciones o una acción se desarrolle en
varias viñetas; el panel `[D]` deberá permitir auditar después el cálculo visible.
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

- pantalla de diagnóstico que permita al jugador inspeccionar el repertorio
  Unicode admitido y reconocer glifos ausentes o con un ancho incompatible; su
  acceso, fuente del repertorio, interacción y recomendaciones se diseñarán
  antes de implementarla;
- entrada mediante ratón;
- ilustración de actores ajenos al Grupo sobre el mapa;
- duraciones diferentes por fotograma y animaciones distintas por estado;
- presentación de grupos numerosos después de incorporar `Reclutar`;
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
