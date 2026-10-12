# Documentación del proyecto

Este directorio reúne la documentación de producto, arquitectura y ejecución del
proyecto. La fuente principal de intención es el [diseño general](./diseno-general.md).

## Estructura

| Ruta | Propósito |
|---|---|
| [`diseno-general.md`](./diseno-general.md) | Define qué juego se quiere construir, sus conceptos y sus restricciones. |
| [`tecnica/`](./tecnica/) | Alojará las decisiones técnicas que explican cómo implementar el diseño. |
| [`planes/pendientes/`](./planes/pendientes/) | Contiene planes de implementación todavía no terminados. |
| [`planes/completados/`](./planes/completados/) | Conserva los planes ya implementados y su resultado real. |
| [`planes/PLANTILLA.md`](./planes/PLANTILLA.md) | Punto de partida obligatorio para un nuevo plan. |

## Relación entre documentos

1. El diseño general fija la intención y usa identificadores estables `DG-*`.
2. La documentación técnica traduce esa intención a contratos y decisiones de
   implementación, y debe indicar qué requisitos `DG-*` satisface.
3. Cada plan pendiente divide una parte de esa documentación técnica en pasos
   verificables y enlaza las secciones en las que se apoya.
4. Cuando la implementación y sus verificaciones terminan, el mismo archivo se
   completa con el resultado y se mueve a `planes/completados/`.

Los estándares y protocolos de desarrollo se definirán en una etapa posterior.
Hasta entonces, este índice describe solamente el flujo documental, no normas de
programación.
