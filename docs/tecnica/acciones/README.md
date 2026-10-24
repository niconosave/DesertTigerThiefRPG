# Diseños técnicos de acciones

Esta carpeta contiene la fuente normativa de las reglas concretas de cada acción
general. El [motor común](../componentes/05-motor-acciones.md) define cómo se
ejecutan acciones, decisiones y resultados; estos documentos definen qué hace
cada acción.

| Documento | Alcance |
|---|---|
| [`01-viajar.md`](./01-viajar.md) | Contrato histórico obsoleto de la primera versión. |
| [`02-viajar-expediciones.md`](./02-viajar-expediciones.md) | Contrato de la expedición física: etapas, preparación, supervivencia y medios. |
| [`03-cronologia-causal-viaje.md`](./03-cronologia-causal-viaje.md) | Contrato aprobado para componer situaciones dramáticas de viaje, preservar cada momento causal y presentar el progreso. |
| [`04-proyecto-humano-viajar.md`](./04-proyecto-humano-viajar.md) | Fuente canónica de propósito, deliberación, plan, subobjetivos, procedencia de medios, interpretación y replanteamiento. |
| [`particulares/`](./particulares/README.md) | Fuente normativa de operaciones de simulación reutilizables. |

Una acción nueva requiere su diseño aprobado antes de incorporarse a un plan de
implementación. Los documentos no repiten los contratos del motor, el Mundo o la
fachada: los enlazan y solamente definen sus reglas particulares.

Una acción particular que solo sea una operación interna trivial puede quedar
descrita dentro de su acción general mientras tenga un único consumidor. En el
momento en que posea reglas propias o se reutilice, requiere un documento en
`particulares/`; ese documento pasa a ser su única fuente normativa.
