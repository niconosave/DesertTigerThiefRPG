# Paleta nativa y colores fijos

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.3 |
| Fecha | 2026-10-26 |

## 1. Alcance

El frontend conserva por defecto los colores nominales de la terminal. Consulta
su paleta para calcular la iluminación de la fogata y ofrece una paleta fija
opcional. Este contrato sustituye la tabla asumida para iluminar colores ANSI
del [contrato 15](./15-fogata-procedural.md#62-cálculo-de-la-luz).
No cambia reglas, archivos de Actores ni el reloj de animación.

| Modo | Dibujo normal | Base de la iluminación |
|---|---|---|
| `terminal`, predeterminado | Colores ANSI nativos, como antes. | RGB consultado para cada color. Si falta, ese color se conserva sin tintado. |
| `fixed` | Todos los colores nominales se convierten a RGB propios. | La misma paleta fija; no se consulta la terminal. |

Los RGB explícitos, incluido el negro puro, mantienen sus valores en ambos modos
y pueden recibir iluminación según las reglas del contrato 15. La llama y las
chispas siempre mantienen su generación RGB. No se sustituye automáticamente
el modo terminal por el fijo al fallar la consulta, ni se inventa una equivalencia
RGB para un color nativo desconocido. Las respuestas parciales se usan por color.

## 2. Argumentos y resolución

`--color-scheme terminal|fixed` selecciona el modo. Se admite como máximo una
vez, combinado con el argumento existente `--content RUTA` en cualquier orden.
Falta de valor, valor desconocido, duplicado u opción desconocida producen un
error antes de activar raw mode. El valor predeterminado es `terminal`.

```sh
cargo run --release --bin desert-tiger-terminal
cargo run --release --bin desert-tiger-terminal -- --color-scheme fixed
```

`colores.rs` contiene `EsquemaColores`, `PaletaColores` y la única tabla RGB fija
de dieciséis entradas, ordenados como ANSI 0–15:

| Índice | Color | RGB | Índice | Color | RGB |
|---|---|---|---|---|---|
| 0 | Negro | 0,0,0 | 8 | Gris oscuro | 128,128,128 |
| 1 | Rojo | 128,0,0 | 9 | Rojo claro | 255,0,0 |
| 2 | Verde | 0,128,0 | 10 | Verde claro | 0,255,0 |
| 3 | Amarillo | 128,128,0 | 11 | Amarillo claro | 255,255,0 |
| 4 | Azul | 0,0,128 | 12 | Azul claro | 0,0,255 |
| 5 | Magenta | 128,0,128 | 13 | Magenta claro | 255,0,255 |
| 6 | Cian | 0,128,128 | 14 | Cian claro | 0,255,255 |
| 7 | Gris | 192,192,192 | 15 | Blanco | 255,255,255 |

`PaletaColores::default()` representa modo terminal con
todos los colores consultables desconocidos. Su API privada del crate permite:

- registrar RGB por índice 0–15;
- resolver un color para iluminar: RGB explícito, entrada conocida o `None`;
- aplicar la paleta fija al buffer final o el forzado independiente de fondo
  negro de la sección 8; sin esas opciones conserva el buffer nativo.

`Reset` se conserva durante el tintado. En el buffer fijo se resuelve a gris
192 para frente y negro puro para fondo. Los índices 0–15 usan la tabla fija;
los índices 16–255 usan el cubo y grises de la paleta xterm de 256 colores.
No se alteran glifos ni modificadores. La conversión final cubre mapa, retratos,
fichas, barras, viñetas, errores y modal de salida, incluidos tamaños pequeños.
Los RGB producidos por el efecto no se convierten nuevamente.

## 3. Consulta y entrada

`consulta_paleta.rs` encapsula consulta, filtrado de respuestas y una cola de
eventos del jugador. Se realiza después de entrar en raw mode y antes de crear
la terminal Ratatui. Se usan `crossterm::event::poll/read` en el mismo hilo que
el resto del programa; no se abre otro lector ni se agregan dependencias.

Se consultan los dieciséis índices con `ESC ] 4 ; índice ; ? ST`, sin modificar
la paleta de la terminal. El plazo total es de 300 ms, con terminación anticipada
si llegan las dieciséis respuestas. `TERM=dumb` omite la consulta. Fallo o
ausencia de respuesta deja entradas desconocidas y permite iniciar normalmente.

El adaptador reconstruye las respuestas OSC a partir de los eventos de teclado
de Crossterm 0.29: `Alt+]` inicia OSC, `Ctrl+G` representa BEL y `Alt+\\`
representa ST. También admite ESC separado del carácter siguiente. Se caracteriza
este comportamiento mediante una prueba del ejecutable en PTY; las plataformas
que no entreguen esas respuestas conservan el camino de paleta desconocida.

Se acepta `OSC 4;índice;rgb:R/G/B` con terminador BEL o ST, incluso fragmentado
y fuera de orden. Cada canal tiene entre uno y cuatro dígitos hexadecimales y
se escala con redondeo de su rango a 0–255. Solo se aceptan índices 0–15.
Las respuestas inválidas no actualizan la paleta. El parser guarda como máximo
256 bytes y no admite crecimiento sin límite.

Los eventos ajenos se conservan en orden para entregarlos al juego después del
inicio. Un candidato ESC/OSC que no coincide con el prefijo de consulta se
restituye como sus eventos originales. Se permite hasta 100 ms entre fragmentos;
un prefijo incompleto no reconocido se restituye al vencer ese plazo; una
respuesta ya identificada como OSC 4 se descarta si está truncada o excede el
límite. Una respuesta completa OSC 4 siempre se consume, válida o inválida.

El mismo filtro permanece delante de la entrada durante la partida para consumir
respuestas tardías. Después de la consulta inicial estas se descartan sin
actualizar la paleta, manteniendo estable la base de iluminación. Su vencimiento
se combina con la espera de animación; un ESC del jugador se entrega al vencer
el breve plazo o al comprobarse que no comienza una respuesta.

## 4. Integración

`RecursosVisuales` conserva `PaletaColores`. Los recursos cargados comienzan en
modo terminal sin consultas; el inicio interactivo asigna el resultado real o
la paleta fija. `fogata::dibujar` recibe esa paleta y solo mezcla canales cuyo
RGB puede resolver. Una celda puede tener frente conocido y fondo desconocido,
o al revés; se tratan por separado. El negro RGB sigue siendo negro.

`Interfaz::dibujar` aplica la conversión fija al terminar la composición completa,
incluidos los caminos de retorno temprano por tamaño insuficiente. La espera de
entrada filtra las respuestas antes de enviarlas a `manejar_tecla` y conserva
las reglas del reloj y la pausa de la fogata.

La tabla fija permanece en código del frontend. Las consultas no leen archivos
de configuración personales ni cambian el tema del emulador. Cambiar el tema con
el juego abierto requiere reiniciar para renovar los RGB consultados.

## 5. Verificación y aceptación

- Argumentos: valores predeterminados, ambos modos, combinación con contenido,
  duplicados, valores ausentes y errores.
- Colores: ANSI intacto en modo terminal, desconocidos sin tintado, registros
  parciales, RGB explícito, negro y modo fijo coherente con y sin iluminación.
- Parser: canales de 1–4 dígitos, escala, índices, formatos inválidos, BEL/ST,
  fragmentación, exceso de tamaño, expiración y restitución de teclas.
- Integración: comparar buffer normal y acampado con paleta ausente, parcial y
  completa; el modo fijo convierte también la interfaz y las pantallas pequeñas.
- PTY: emular respuestas de una paleta personalizada y comparar RGB iluminado;
  simular ausencia, respuesta parcial, tardía, tecla durante consulta y modo fijo.
  Comprobar que la consulta no activa acciones y que la salida restaura terminal.
- Pasan los controles de la metodología, la carga oficial y el build release.

## 6. Fuente del protocolo

[XTerm Control Sequences, OSC 4](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html)
define índices, consultas y terminadores. Crossterm se mantiene fijado en
0.29.0; el adaptador se revisa si cambia su interpretación de entrada.

No hay decisiones bloqueantes en este alcance. Detección RGB, menú de opciones,
actualización de tema en vivo y otros protocolos de consulta quedan aplazados.

## 7. Verificación de la implementación — 2026-10-26

Se implementaron los dos modos, consulta y filtrado en `colores.rs` y
`consulta_paleta.rs`, sin dependencias nuevas ni cambios de backend. La CLI,
el recurso visual compartido, la fogata y el buffer final usan este contrato.

Pasan fmt, check, clippy con advertencias como errores, las 123 pruebas de
workspace (31 app, 46 core, 42 terminal, 4 importador) y build release. Las ocho
pruebas adicionales cubren argumentos, modos, parser, fragmentación, expiración,
restitución de entrada, canales desconocidos e integración de pantalla completa.
La carga oficial de contenido, scripts y recursos también pasa.

Se ejercitó el release en PTY 80×24 con seis escenarios: paleta personalizada
completa, parcial, sin respuesta, respuesta tardía, tecla durante la consulta y
modo fijo. La paleta completa se respondió en orden inverso y fragmentada. Se
comprobó que la iluminación deriva de un magenta personalizado 187,136,170,
que los colores ausentes conservan su aspecto y que los informes tardíos no
abren Acampar. La tecla enviada durante el inicio se conservó y el modo fijo
no emitió consultas. Los seis casos permitieron pausar, cerrar el campamento y
salir con código 0 y restauración de la pantalla. Se inspeccionaron capturas
de los modos nativo consultado y fijo con Noto Sans Mono.

## 8. Fondo predeterminado y opción `-B`

El modo terminal vuelve a usar por defecto el fondo del emulador mediante
`Color::Reset`, que es distinto del índice ANSI negro. El recurso oficial de
pantalla, barra contextual y tiles que antes imponían un fondo negro utiliza
`fondo: None` para heredar ese lienzo. La llama también conserva su fondo.

`-B`, con B mayúscula y sin valor, fuerza únicamente el fondo general a RGB
0,0,0. Se combina con `--content` y `--color-scheme` en cualquier orden y se
rechaza si está duplicado. No selecciona el modo fijo ni cambia el frente,
glifos, modificadores, luz o fondos coloreados de tiles y controles.

`PaletaColores` conserva esta opción independientemente del esquema. Su método
`fondo_pantalla` resuelve el lienzo antes de componer y `aplicar` convierte los
fondos `Reset` restantes, por ejemplo los de `Clear`, en negro RGB con `-B`.
Sin la opción, `Reset` permanece nativo en modo terminal; el modo fijo conserva
su fondo negro de la paleta propia. Los fondos explícitos del arte de Actores,
como los ojos negros opacos, mantienen su significado.

Se verifican argumentos y combinaciones, preservación del frente y fondos
coloreados, mapa, campamento, modal y pantalla pequeña. Los retratos oficiales
del príncipe y del bandido usan fondo heredado en sus ojos y parpadeos, como el
sirviente. Los trazos negros de los Actores usan el color ANSI nativo; el modo
fijo los convierte con su paleta. La regresión del ojo comprueba la herencia
del lienzo con los ojos abiertos y cerrados.

Verificación del ajuste: pasan los controles generales, build release y 126
pruebas (tres nuevas para argumentos, resolución del fondo y composición).
Se probó el ejecutable en PTY con fondo predeterminado gris 28,28,28: modo
nativo conservó ese fondo, `-B` produjo RGB 0,0,0 y el modo fijo conservó su
paleta. Los tres recorridos permitieron acampar, pausar, cerrar y salir con
código 0 y restauración de terminal. Se revisó la captura del fondo nativo.
