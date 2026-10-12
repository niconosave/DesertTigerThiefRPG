# Documentos técnicos específicos

Esta carpeta contiene un documento por parte concreta del código o de los datos.
Todos desarrollan el
[diseño técnico general](../diseno-tecnico-general.md) y siguen la
[metodología de desarrollo](../../metodologia-desarrollo.md).

| Documento | Alcance |
|---|---|
| [`01-workspace-entorno.md`](./01-workspace-entorno.md) | Workspace, crates, dependencias y entorno Rust. |
| [`02-modelo-mundo.md`](./02-modelo-mundo.md) | Partida, Mundo, elementos, referencias e invariantes. |
| [`03-contenido.md`](./03-contenido.md) | Archivos editables, validación e instanciación. |
| [`04-motor-acciones.md`](./04-motor-acciones.md) | Protocolo y ejecución de acciones generales y particulares. |
| [`05-tiempo-fase-npc.md`](./05-tiempo-fase-npc.md) | Calendario, avance temporal y fase de NPC. |
| [`06-integracion-lua.md`](./06-integracion-lua.md) | Contrato y límites de las decisiones escritas en Lua. |
| [`07-frontend-terminal.md`](./07-frontend-terminal.md) | Entrada, modos, animación y presentación con Ratatui. |

La numeración facilita la lectura; no constituye un plan de implementación.
Cada documento permanece en estado `Borrador` mientras tenga decisiones
pendientes. Antes de crear un plan para un alcance, su documentación aplicable
deberá estar aprobada para implementar.
