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
