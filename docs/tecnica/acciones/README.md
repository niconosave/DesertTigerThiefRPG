# Diseños técnicos de acciones

Esta carpeta contiene la fuente normativa de las reglas concretas de cada acción
general. El [motor común](../componentes/05-motor-acciones.md) define cómo se
ejecutan acciones, decisiones y resultados; estos documentos definen qué hace
cada acción.

| Documento | Alcance |
|---|---|
| [`01-viajar.md`](./01-viajar.md) | Selección de destino, viñetas, traslado y duración de `Viajar`. |
| [`particulares/`](./particulares/README.md) | Fuente normativa de operaciones de simulación reutilizables. |

Una acción nueva requiere su diseño aprobado antes de incorporarse a un plan de
implementación. Los documentos no repiten los contratos del motor, el Mundo o la
fachada: los enlazan y solamente definen sus reglas particulares.

Una acción particular que solo sea una operación interna trivial puede quedar
descrita dentro de su acción general mientras tenga un único consumidor. En el
momento en que posea reglas propias o se reutilice, requiere un documento en
`particulares/`; ese documento pasa a ser su única fuente normativa.
