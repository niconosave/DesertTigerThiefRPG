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

## Sonido de la fogata

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
