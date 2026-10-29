# Documentación del proyecto

Este directorio reúne la documentación de producto, arquitectura y ejecución del
proyecto. La fuente principal de intención es el [diseño general](./diseno-general.md).

## Estructura

| Ruta | Propósito |
|---|---|
| [`diseno-general.md`](./diseno-general.md) | Define qué juego se quiere construir, sus conceptos y sus restricciones. |
| [`diseno/acampar.md`](./diseno/acampar.md) | Diseño revisado de Acampar: fichas, características, relaciones y navegación. |
| [`metodologia-desarrollo.md`](./metodologia-desarrollo.md) | Define cómo se diseñan, implementan y verifican los incrementos. |
| [`contenido/`](./contenido/) | Contiene briefs creativos del mundo sin duplicar los datos RON concretos. |
| [`tecnica/`](./tecnica/) | Contiene la arquitectura general y los diseños técnicos específicos. |
| [`planes/pendientes/`](./planes/pendientes/) | Contiene planes de implementación todavía no terminados. |
| [`planes/completados/`](./planes/completados/) | Conserva los planes ya implementados y su resultado real. |
| [`planes/PLANTILLA.md`](./planes/PLANTILLA.md) | Punto de partida obligatorio para un nuevo plan. |

## Relación entre documentos

1. El diseño general enumera los elementos fundamentales del juego y sus
   relaciones conceptuales.
2. Los briefs de contenido particularizan la intención narrativa y visual de un
   conjunto de datos sin definir su serialización.
3. El diseño técnico general define la organización global del desarrollo y los
   documentos técnicos específicos diseñan cada parte del código y de los datos.
4. Cada plan pendiente convierte una parte de esa documentación técnica y del
   brief aplicable en pasos verificables y enlaza las secciones en las que se
   apoya. El plan no sustituye ni repite el diseño técnico o creativo.
5. Cuando la implementación y sus verificaciones terminan, el mismo archivo se
   completa con el resultado y se mueve a `planes/completados/`.

La [metodología de desarrollo](./metodologia-desarrollo.md) gobierna este flujo,
los estándares de implementación, las pruebas y la definición de terminado.

## Herramientas de desarrollo

Desde el mapa o la selección de destino, `D` abre el menú de desarrollo.
Usá `↑/↓` para elegir una opción y `Enter` o `Espacio` para alternarla.
`C` alterna directamente las conexiones e `I` el inspector de coordenadas.
`Esc` o `D` cierran el menú. Ambas opciones comienzan apagadas.

- **Mostrar conexiones de lugares:** une los centros con puntos que recorren el
  arcoíris cada 250 ms, marca los centros con `O` y excluye las relaciones Abarca.
- **Inspeccionar coordenadas:** al pasar el mouse sobre el mapa, resalta el tile
  y muestra `Mapa: X=…, Y=…` en la barra contextual. Son las coordenadas de los
  RON: comienzan en cero, X aumenta hacia la derecha e Y hacia abajo.
  Un clic izquierdo fija el punto; otro elige uno nuevo. El clic derecho libera
  la coordenada y vuelve al seguimiento del mouse.

El inspector respeta la cámara y excluye márgenes, barras, retratos, ventanas y
lista de destinos. La captura del mouse se activa únicamente con el inspector
encendido y el mapa disponible; se suspende al viajar, acampar o confirmar la
salida. Se restaura al regresar al mapa. Durante las viñetas, `D` sigue mostrando
detalles. Estos ajustes no se guardan entre ejecuciones.

## Símbolos de lugares

En el RON de un Lugar podés añadir, por ejemplo:

```ron
simbolo: "⌂",
```

Se dibuja en el centro del Lugar con el mismo color que su nombre. Debe ser un
único carácter visible de una columna. Omitir el campo o usar `simbolo: ""`
no agrega ningún símbolo. El `@` del grupo y las `O` de desarrollo conservan
prioridad. Se carga al iniciar la partida, junto con los demás datos del Lugar.

## Elegir los colores al jugar

El modo predeterminado conserva la paleta de la terminal. Si puede consultarla,
la fogata utiliza esos tonos como base de su luz; los colores que no puede
consultar conservan su apariencia nativa.

```sh
cargo run --release --bin desert-tiger-terminal
```

Para usar la paleta RGB fija del juego, incluida la iluminación:

```sh
cargo run --release --bin desert-tiger-terminal -- --color-scheme fixed
```

También se admite `--color-scheme terminal` explícito. Estas opciones pueden
combinarse con `--content RUTA`. El negro puro y los otros RGB explícitos se
conservan en ambos modos. El [contrato 16](./tecnica/componentes/16-paleta-terminal.md)
define el comportamiento y la compatibilidad.

El fondo general usa por defecto el de tu terminal. Para forzar únicamente
negro puro, conservando los colores nativos:

```sh
cargo run --release --bin desert-tiger-terminal -- -B
```

`-B` se puede combinar con las otras opciones. Los fondos coloreados del mapa
y la interfaz, y los fondos explícitos de los retratos, se conservan.

## Música del campamento

La melodía de prueba “Hilo del camino” y las instrucciones para generar su
previsualización están en [música](../assets/audio/musica/README.md).
Usa cuerdas pulsadas sintetizadas con FunDSP y notas editables en RON. Al
acampar, un integrante vivo con `musico` puede tocar la versión grave aceptada;
el sirviente inicial tiene esa característica. Aparecen notas animadas junto a
su retrato mientras toca.

Solo interpreta un músico a la vez. Mientras suena, todos los integrantes con
la expresión `music` la muestran, también quienes escuchan, y recuperan sus
caras anteriores al terminar. El grupo inicial incluye esa variante. Las
notas animadas salen únicamente del intérprete. La banda queda aplazada.

El repertorio incluye la versión grave de “Hilo del camino” y la nueva
“Recuerdo del hogar”, de alrededor de un minuto. Se alternan para evitar que
suene siempre la misma; la lista `partituras` en `campamento.ron` permite
añadir otras piezas. El primer archivo de la lista suena primero.

La primera pieza espera entre 60 y 120 segundos. Entre interpretaciones hay
pausas de 20–180 segundos. El volumen y esos intervalos se editan en
[`campamento.ron`](../assets/audio/musica/campamento.ron) y se cargan al reiniciar.
Cambiar de ficha no reinicia la música. `--no-audio` también deshabilita sus
notas animadas. El [contrato 20](./tecnica/componentes/20-musica-campamento.md)
define su integración con los retratos y el resto del audio.

Las canciones se sintetizan en segundo plano después de mostrar el juego.
Si acampás antes de que estén listas, la fogata y los ambientes suenan normalmente;
la espera inicial de música comienza cuando termina la preparación.

## Sonido de la fogata

El juego también genera viento suave y un ave lejana ocasional. En lugares
fluviales o conectados a ellos se escucha corriente; en costas y mares, oleaje;
en bosques, dos cantos de aves distintos. Los sonidos siguen el lugar confirmado
del grupo durante Viajar y continúan al acampar, junto con la fogata.

Los volúmenes se ajustan en
[`assets/audio/ambientes.ron`](../assets/audio/ambientes.ron) y se cargan al
reiniciar. `volumen` controla el conjunto ambiental; los demás campos controlan
cada sonido. El [contrato 19](./tecnica/componentes/19-paisajes-sonoros.md) define
las condiciones y el formato. `--no-audio` silencia todos los efectos.

En ese mismo archivo, `tipo_rio: Riachuelo` prueba agua entre piedras, con
pequeños chapoteos y borboteos. `tipo_rio: Corriente` recupera el río original.
Si se omite el campo se utiliza `Corriente`. Hay que reiniciar para cambiarlo.

El sonido está habilitado por defecto y se escucha al acampar. Para desactivarlo:

```sh
cargo run --release --bin desert-tiger-terminal -- --no-audio
```

Se combina con `--content`, `--color-scheme` y `-B`. Si no hay una salida de
audio compatible, el juego muestra un aviso y continúa en silencio.

El volumen y la densidad del crepitar se editan en
[`assets/audio/fogata.ron`](../assets/audio/fogata.ron); los cambios se cargan al
reiniciar. El [contrato del sonido](./tecnica/componentes/18-crepitar-fogata.md#2-recurso-editable)
explica los campos y sus límites.

También se puede compilar sin FunDSP, CPAL ni sus requisitos de sistema:

```sh
cargo run --release --no-default-features --bin desert-tiger-terminal
```

En Linux, compilar con audio requiere los archivos de desarrollo de ALSA y
`pkg-config`. El juego sintetiza el sonido; no requiere grabaciones ni un
sintetizador MIDI instalado.
