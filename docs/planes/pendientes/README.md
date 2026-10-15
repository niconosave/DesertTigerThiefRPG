# Planes pendientes

Aquí viven los planes aprobados o en preparación cuya implementación todavía no
terminó. Deben crearse desde [`../PLANTILLA.md`](../PLANTILLA.md) y conservar
referencias precisas a la documentación técnica que define el trabajo.

## Primer incremento jugable

Los siete planes forman juntos un único incremento jugable. Su implementación
es lineal: cada plan comienza cuando el alcance y las verificaciones acotadas del
anterior están resueltos y registrados. Los planes permanecen en `pendientes/`
hasta que PLAN-2026-007 integre el recorrido, deje el repositorio en un estado
jugable y permita cerrar y archivar el conjunto. La dependencia transitiva no se
repite en la tabla.

| Orden | Plan | Dependencia directa | Resultado principal |
|---:|---|---|---|
| 1 | [PLAN-2026-001](./2026-10-15-01-preparar-workspace.md) | Ninguna | Workspace reproducible. |
| 2 | [PLAN-2026-002](./2026-10-15-02-implementar-modelo-mundo-calendario.md) | PLAN-2026-001 | Mundo y calendario probados. |
| 3 | [PLAN-2026-003](./2026-10-15-03-implementar-motor-viajar.md) | PLAN-2026-002 | Motor y `Viajar` ejecutables sin frontend. |
| 4 | [PLAN-2026-004](./2026-10-15-04-implementar-contenido-lua.md) | PLAN-2026-003 | Contenido editable, fixture y Lua validados. |
| 5 | [PLAN-2026-005](./2026-10-15-05-implementar-sesion-fase-npc.md) | PLAN-2026-004 | Recorrido completo mediante la fachada. |
| 6 | [PLAN-2026-006](./2026-10-15-06-implementar-frontend-terminal.md) | PLAN-2026-005 | Interfaz terminal comprobable. |
| 7 | [PLAN-2026-007](./2026-10-15-07-integrar-recorrido-jugable.md) | PLAN-2026-006 | Primera versión local jugable. |

Al cerrar PLAN-2026-007, los siete resultados se completan y los archivos se
mueven juntos a `../completados/`. En ese mismo cambio, esta tabla se retira o
actualiza para enlazar el historial sin dejar enlaces rotos.
