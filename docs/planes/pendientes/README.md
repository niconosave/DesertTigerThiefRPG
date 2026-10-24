# Planes pendientes

Aquí viven los planes aprobados o en preparación cuya implementación todavía no
terminó. Deben crearse desde [`../PLANTILLA.md`](../PLANTILLA.md) y conservar
referencias precisas a la documentación técnica que define el trabajo.

## Pendientes de cierre del primer incremento jugable

Los siete planes forman juntos un único incremento jugable. Su implementación
es lineal: cada plan comienza cuando el alcance y las verificaciones acotadas del
anterior están resueltos y registrados. La capacidad principal ya está integrada,
pero los archivos permanecen aquí mientras falten verificaciones o cierres
documentales expresamente registrados en ellos. Ese trabajo pendiente no se
redefine ni se duplica en los planes del incremento siguiente.

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

### Refinamiento visual pendiente de cierre

| Plan | Estado operativo | Resultado principal |
|---|---|---|
| [PLAN-2026-008](./2026-10-15-08-importar-retrato-durdraw.md) | En verificación manual | Retratos coloreados importados desde Durdraw. |

Este plan se cierra conforme a sus propios criterios y no es una dependencia de
las Relaciones o de las reglas nuevas de `Viajar`.

## Trabajo aplazado sin diseño técnico aprobado

| Plan | Estado operativo | Motivo |
|---|---|---|
| [PLAN-2026-009](./2026-10-15-09-diagnosticar-compatibilidad-unicode.md) | No ejecutable | Requiere diseñar y aprobar primero el contrato del diagnóstico de terminal. |

PLAN-2026-009 no pertenece a la secuencia activa ni bloquea los planes nuevos.
Solo podrá implementarse después de resolver las precondiciones que registra y
sin convertir el propio plan en fuente de decisiones técnicas.

## Secuencia activa: Viajar por expediciones

Esta serie profundiza únicamente `Viajar`. Los planes 016 a 019 ya están
archivados en `../completados/`. La primera revisión manual confirmó que las
reglas terrestres y marítimas funcionan, pero reveló que la implementación
carece de un proyecto humano observable que ordene la simulación. Los planes 021
a 024 conservan primero las trazas mecánicas, refactorizan `Viajar` alrededor de
propósito, deliberación, plan y subobjetivos, producen después su cronología
humana y causal y finalmente la presentan sin reinterpretarla. La implementación
automática de 022 a 024 está terminada; 024 y el cierre integral de
PLAN-2026-020 esperan ahora un único recorrido manual terrestre y marítimo. Los cierres documentales antiguos
que permanecen pendientes no duplican ni bloquean esta secuencia.

| Orden | Plan | Dependencia directa | Resultado principal |
|---:|---|---|---|
| 1 | [PLAN-2026-022](./2026-10-19-22-refactorizar-viajar-proyecto-humano.md) | Implementado; cierre agrupado | Orquestación reanudable como proyecto humano con medios de procedencia visible. |
| 2 | [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md) | Implementado; cierre agrupado | Cronología humana, causal y dramática sin omisiones. |
| 3 | [PLAN-2026-024](./2026-10-19-24-presentar-proyecto-viaje-terminal.md) | Pendiente de prueba manual | Proyecto, escenas y seguimiento legibles en terminal. |
| 4 | [PLAN-2026-020](./2026-10-17-20-equilibrar-viajar-expediciones.md) | Pendiente de la misma prueba manual | Revisión manual final y cierre del atlas consolidado. |

La prueba manual se realiza al final de cada lote, no después de cada detalle.
Las variantes pequeñas y el azar se cubren con pruebas reproducibles. No existe
un límite de escenas o viñetas en la acción del jugador: la prueba controla que
toda acción particular ejecutada quede reconocible en su orden causal. Una
escena puede reunir varias acciones y una acción puede desarrollarse en varias
viñetas; ningún plan supone una correspondencia uno a uno. Para NPC, la misma
cronología se conserva internamente y solo su proyección breve ocupa la barra
contextual.
