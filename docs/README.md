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

## Exploración del mapa

El grupo ve hasta 32 tiles, con obstáculos según el relieve. Lo que nunca vio
queda vacío; el terreno observado anteriormente queda en gris y sin animación
cuando sale de su vista. La memoria se conserva durante la partida. Elegir un
destino o mover la cámara no descubre terreno.

Las alturas se editan en el bloque `alturas` de
[`mapa_inicial.ron`](../content/base/maps/mapa_inicial.ron), con caracteres y su
propia leyenda. Tiene las mismas 192 × 64 posiciones que la matriz de terreno.
En el bloque `vision` podés ajustar `radio` y `altura_observador`; los cambios
se cargan al iniciar una nueva partida. El [contrato de visibilidad](./tecnica/componentes/22-alturas-visibilidad-memoria.md)
describe el formato y las reglas del recuerdo.

## Descripciones y arte de lugares

Los archivos `content/base/places/*.ron` admiten una descripción opcional:

```ron
descripcion: "Ante vos se alza una pared de roca rojiza. Su borde corta el cielo muy por encima del camino.",
```

Escribila en segunda persona, con una o dos oraciones sobre lo que ve el jugador.
Se presenta antes del relato del desplazamiento para situarte en el paisaje
del lugar de partida, y en la viñeta final para describir el destino alcanzado.
También se conserva en el detalle. No se lleva registro de lugares ya descritos.
Omitir el campo conserva el relato actual. Los cambios se cargan al iniciar
una nueva partida.

También se admite `arte: Some((paleta: {...}, base: (...)))`, con la misma
estructura de dibujo y estilos usada por la base de los retratos. Se dibuja
centrado y apoyado encima de las barras, sobre el mapa y detrás de los actores
y las ventanas. Los espacios dejan ver el mapa. El [formato completo y un ejemplo](./tecnica/componentes/03-contenido.md#descripción-e-ilustración-de-lugar--2026-11-02)
explican cómo escribirlo. Ambos campos son independientes y opcionales.
Durante un desplazamiento muestra el paisaje del lugar de partida que relata
la escena; al llegar, el del destino. El inspector de coordenadas y las
conexiones de desarrollo ocultan esta capa mientras están activos.

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

## Gráficos alternativos

Para cargar tus dibujos con caracteres de una fuente propia:

```sh
cargo run --release --bin desert-tiger-terminal -- --graphics custom-font
```

`--graphics ascii` selecciona la carpeta de dibujos ASCII y `--graphics default`
usa los habituales. Omitir la opción equivale a `default`. Cada gráfico sin
alternativa conserva su versión habitual; las carpetas están preparadas para
que puedas ir agregando archivos. El juego supone que configuraste la fuente
en tu terminal.

La [guía de gráficos alternativos](../assets/terminal/alternativos/README.md)
explica dónde poner retratos, tiles y escenas, e incluye un ejemplo de RON.
El modo ASCII valida los dibujos alternativos, pero no convierte el resto de
la interfaz ni el respaldo Unicode.

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

## Comparar caminos al viajar

Los vecinos directos omiten la deliberación. En viajes más largos aparece una
viñeta que narra la deliberación. Al terminar de leerla con Enter, las opciones
se recorren con `←/→` en las barras inferiores, sobre el mapa completo. `Enter`
adopta la opción, `D` consulta detalles y `Esc` enfoca cancelar. Los puntos muestran el
recorrido considerado; improvisar mantiene el camino oculto. `Shift + flechas`
desplaza la cámara y `Home` recupera el encuadre.

El [contrato de planificación](./tecnica/acciones/08-planificacion-rutas.md)
define los perfiles, tiempos y límites.
