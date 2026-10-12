# Frontend de terminal con Ratatui

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.2 |
| Fecha | 2026-10-12 |

## 1. Propósito

Este documento diseña el frontend inicial que presentará el juego dentro de una
terminal de Linux. Desarrolla la sección de frontend del
[diseño general](../../diseno-general.md) y respeta los límites del
[diseño técnico general](../diseno-tecnico-general.md).

Su responsabilidad es transformar vistas y resultados de la aplicación en una
experiencia visual, y transformar la entrada del usuario en comandos semánticos.
No contiene ni deduce reglas del juego.

## 2. Alcance

Incluye:

- ciclo de vida de la terminal;
- recepción de teclado y redimensionado;
- estado y transiciones visuales;
- composición del mapa, rótulos, retratos y barras;
- presentación sucesiva de fragmentos narrativos;
- animación basada en tiempo real;
- traducción entre entrada física y comandos de `desert-tiger-app`.

No incluye reglas de acciones, validación de objetivos, decisiones de NPC,
avance de fecha, carga de contenido ni una abstracción universal para engines
gráficos.

## 3. Bibliotecas y modelo de ejecución

Se usarán Ratatui para componer y dibujar la interfaz y Crossterm para eventos,
modo raw, pantalla alternativa, cursor y tamaño de terminal. Ambas dependencias
existirán exclusivamente en `desert-tiger-terminal`.

El frontend, la aplicación y el núcleo se ejecutarán en un solo proceso. Los
comandos y resultados serán llamadas y valores Rust; no habrá red, REST ni
comunicación entre procesos. `desert-tiger-terminal` dependerá únicamente de la
interfaz pública de `desert-tiger-app`.

Las versiones se fijarán juntas en la workspace para evitar dos versiones
incompatibles de Crossterm. La versión exacta se registrará en `Cargo.toml` y
`Cargo.lock` al preparar el entorno.

## 4. Responsabilidades internas

El crate separará conceptualmente estas partes, sin convertirlas en nuevos
servicios:

- **ciclo de terminal:** entrar y salir de modo raw y pantalla alternativa;
- **bucle de eventos:** combinar entrada, redimensionado y ticks de animación;
- **adaptador de entrada:** convertir teclas en intenciones según el modo visual;
- **estado de interfaz:** conservar modo, foco, cámara y cuadro de animación;
- **composición:** distribuir las regiones de la pantalla;
- **renderizado:** dibujar cada región a partir de la vista de aplicación.

Los tipos `Frame`, `Rect`, `Style`, `KeyCode` y demás tipos de Ratatui o
Crossterm no atravesarán el límite hacia `desert-tiger-app`.

## 5. Contrato con la aplicación

El frontend recibirá valores poseídos que describan:

- una vista de solo lectura del estado vigente del Mundo y del mapa de esa
  Partida;
- acciones y objetivos ofrecidos al jugador;
- fragmentos narrativos y resultados;
- decisiones pendientes con opciones identificadas;
- resúmenes de las acciones de NPC;
- rechazos o errores recuperables que deban comunicarse.

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

## 6. Modos visuales iniciales

Los modos son estado de interfaz y no sustituyen la fase mantenida por la
aplicación:

| Modo | Responsabilidad |
|---|---|
| Vista del mundo | Mostrar el mundo y permitir elegir una acción disponible. |
| Selección de destino | Enfocar lugares ofrecidos para `Viajar` y confirmar uno. |
| Presentación de acción | Mostrar en sucesión los fragmentos narrativos recibidos. |
| Decisión | Presentar las opciones de una decisión pendiente y enviar la elegida. |
| Resumen de NPC | Comunicar brevemente los resultados de la fase de NPC. |
| Terminal insuficiente | Explicar que la ventana debe ampliarse sin ejecutar comandos de juego. |

Una transición visual nunca creará ni completará por sí sola una acción. Si el
frontend reconstruye su pantalla, consultará nuevamente la vista y fase de la
aplicación en lugar de inferirlas de lo que había dibujado.

## 7. Composición de la pantalla

La vista principal ocupará las dimensiones disponibles y respetará este orden
conceptual:

1. área del mapa ASCII;
2. nombres de lugares y resaltados dibujados sobre el mapa;
3. área inferior de retratos animados del jugador y su grupo;
4. barra de acciones con una letra de color diferente por acción;
5. barra contextual cuando el modo necesite una instrucción o resumen.

El mapa se compondrá por capas. Los tiles actuales de la Partida se dibujan
primero; después se añaden rótulos, foco e indicadores ilustrativos. La posición
exacta de un actor sobre el mapa es estética y nunca se devuelve a la aplicación.

Los retratos representan la parte superior de los actores y pueden superponerse
de manera controlada. El orden, recorte y cuadro de animación pertenecen al
frontend; los frames proceden del catálogo de presentación.

En selección de viaje, la cámara enfoca el lugar activo, su nombre cambia de
color y la barra contextual explica que se está eligiendo un destino. La lista
de candidatos procede de la aplicación; otra señal visual podrá acompañar al
color para mantener legible la selección.

## 8. Bucle, animación y restauración

La primera versión usará un bucle síncrono y monohilo:

1. espera un evento o el siguiente tick visual;
2. traduce y procesa la entrada;
3. consulta o actualiza la aplicación cuando corresponde;
4. actualiza el estado de interfaz;
5. renderiza el cuadro.

El tick utiliza un reloj monotónico del frontend. Puede cambiar un frame o una
transición visual, pero nunca adelanta la fecha del mundo. No se incorporará un
runtime asíncrono mientras este bucle resulte suficiente.

La inicialización y el cierre serán simétricos. Un guardián de ciclo de vida
restaurará cursor, modo raw y pantalla principal tanto ante una salida normal
como ante un error propagado. Si la terminal cambia de tamaño, se recalculará la
composición antes de dibujar otra vez.

## 9. Entrada y accesibilidad básica

La barra mostrará el atajo de cada acción coloreando una de sus letras. Un
adaptador dependiente del modo convertirá la tecla física en una intención. Las
teclas concretas para mover foco, confirmar, abandonar una selección visual antes
de enviar la acción, continuar y salir quedan pendientes de diseño visual. Esto
no define todavía la cancelación de una acción ya iniciada.

Los tiles del mapa se limitarán a caracteres ASCII. Los textos podrán contener
acentos; el renderizado deberá medir su ancho visible en celdas y recortarlos sin
romper la composición. El color reforzará el estado, pero una selección también
deberá reconocerse por el texto o la forma, no solo por el color.

## 10. Errores y validaciones

- Un fallo al iniciar la terminal impedirá entrar al bucle y conservará la
  terminal original.
- Un fallo durante entrada o dibujo provocará una salida controlada después de
  restaurar la terminal.
- Un comando rechazado por la aplicación se presentará sin alterar localmente
  el estado del mundo.
- La composición recortará contenido fuera de su región y no escribirá fuera del
  búfer.
- Una terminal menor que el mínimo definido mostrará el modo seguro y no una
  interfaz parcialmente interactiva.

## 11. Pruebas y criterios de aceptación

Pruebas automáticas:

- eventos sintéticos se traducen al comando correcto para cada modo;
- las transiciones visuales no ejecutan reglas propias;
- una vista con tiles modificados se representa sin que el frontend infiera o
  vuelva a ejecutar el cambio semántico que la produjo;
- los componentes dibujan dentro de su región en varios tamaños;
- el foco de viaje, las barras y la superposición de retratos producen las
  celdas esperadas;
- un reloj falso controla los cuadros de animación;
- `TestBackend` recorre pantallas completas sin una terminal real.

Pruebas manuales en Manjaro comprobarán color, teclas, fluidez, acentos,
redimensionado y restauración después de salir o provocar un error controlado.

El documento podrá aprobarse para implementar cuando estén definidos el mapa de
teclas, el tamaño mínimo y el formato de las animaciones. La implementación se
aceptará cuando el frontend pueda completar el recorrido de humo usando solo la
fachada de aplicación y `cargo tree` confirme que Ratatui y Crossterm no aparecen
en los otros crates.

## 12. Preguntas pendientes

- ¿Qué teclas moverán el foco, confirmarán, cancelarán y continuarán?
- ¿Cuál será el tamaño mínimo de terminal y qué regiones se reducen primero?
- ¿Qué paleta y estilos distinguirán foco, atajos, errores y texto normal?
- ¿Cómo se representarán los frames y a qué velocidad se reproducirán?
- ¿Los actores presentes en los lugares se ilustrarán también sobre el mapa?
- ¿Se admitirá ratón o solamente teclado en la primera versión?

## 13. Referencias

- [Backends de Ratatui](https://ratatui.rs/concepts/backends/)
- [Renderizado de Ratatui](https://ratatui.rs/concepts/rendering/under-the-hood/)
- [`TestBackend` de Ratatui](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html)
- [Eventos de Crossterm](https://docs.rs/crossterm/latest/crossterm/event/)
