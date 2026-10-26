# Documentos técnicos específicos

Esta carpeta contiene un documento por parte concreta del código o de los datos.
Todos desarrollan el
[diseño técnico general](../diseno-tecnico-general.md) y siguen la
[metodología de desarrollo](../../metodologia-desarrollo.md).

| Documento | Alcance |
|---|---|
| [`01-workspace-entorno.md`](./01-workspace-entorno.md) | Workspace, crates, dependencias y entorno Rust. |
| [`02-modelo-mundo.md`](./02-modelo-mundo.md) | Mundo reglado, elementos, referencias e invariantes. |
| [`03-contenido.md`](./03-contenido.md) | Archivos editables, validación e instanciación. |
| [`04-sesion-aplicacion.md`](./04-sesion-aplicacion.md) | Partida, fases y fachada pública de la aplicación. |
| [`05-motor-acciones.md`](./05-motor-acciones.md) | Protocolo y ejecución de acciones generales y particulares. |
| [`06-calendario-duracion.md`](./06-calendario-duracion.md) | Fecha, duraciones y avance temporal. |
| [`07-fase-npc.md`](./07-fase-npc.md) | Orden, ejecución y resúmenes de NPC. |
| [`08-integracion-lua.md`](./08-integracion-lua.md) | Contrato y límites de las decisiones escritas en Lua. |
| [`09-frontend-terminal.md`](./09-frontend-terminal.md) | Entrada, modos, animación y presentación con Ratatui. |
| [`10-relaciones.md`](./10-relaciones.md) | Relaciones entre elementos, índices de consulta y grafo `Conectado`. |
| [`11-operaciones-mundo.md`](./11-operaciones-mundo.md) | Consecuencias persistentes y lotes de aplicación atómica. |
| [`12-conocimiento-lugares.md`](./12-conocimiento-lugares.md) | Relación dirigida `Conoce`, índices y proyección de hechos conocidos. |
| [`13-retratos-por-capas.md`](./13-retratos-por-capas.md) | Base, expresiones, ojo superpuesto y parpadeo individual. |

Las reglas concretas de cada acción viven en
[`../acciones/`](../acciones/README.md); los componentes anteriores solo definen
el mecanismo compartido.

La numeración facilita la lectura; no constituye un plan de implementación.
Cada documento permanece en estado `Borrador` mientras tenga decisiones
pendientes que afecten su alcance actual. Las extensiones expresamente aplazadas
no bloquean su aprobación. Antes de crear un plan para un alcance, su
documentación aplicable deberá estar aprobada para implementar.
