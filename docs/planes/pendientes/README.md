# Planes pendientes

Aquí viven los planes aprobados o en preparación cuya implementación todavía no
terminó. Deben crearse desde [`../PLANTILLA.md`](../PLANTILLA.md) y conservar
referencias precisas a la documentación técnica que define el trabajo.

## Paleta nativa y colores fijos terminados

[PLAN-2026-040](../completados/2026-10-26-40-resolver-paleta-terminal.md) está
implementado y archivado. Sus escenarios y verificaciones están en el
[contrato 16](../../tecnica/componentes/16-paleta-terminal.md#7-verificación-de-la-implementación--2026-10-26).
No quedan tareas de este incremento.

## Fogata procedural terminada

[PLAN-2026-039](../completados/2026-10-26-39-animar-iluminar-fogata.md) está
implementado, verificado y archivado. El
[contrato 15](../../tecnica/componentes/15-fogata-procedural.md#11-verificación-de-la-implementación--2026-10-26)
registra su cierre. No quedan tareas de este incremento.

## Parentesco y Actores de contexto terminado

PLAN-2026-038 está implementado, verificado y archivado en
[completados](../completados/README.md#parentesco-y-actores-de-contexto).
El [contrato 14](../../tecnica/componentes/14-parentesco-actores-contexto.md#12-verificación-de-la-implementación--2026-10-26)
registra las pruebas y el recorrido manual. No quedan tareas pendientes de este
alcance; los planes históricos conservan su estado anterior.

## Acampar terminado

PLAN-2026-036 y PLAN-2026-037 están implementados, verificados y archivados en
[completados](../completados/README.md#acción-general-acampar).
El [contrato de Acampar](../../tecnica/acciones/07-acampar.md#12-verificación-de-la-implementación--2026-10-26)
registra las pruebas y el recorrido manual del incremento. No quedan tareas de
esta secuencia; los alcances anteriores conservan sus propios estados.

## Refactor del reglamento de Viajar terminado

PLAN-2026-025 a PLAN-2026-031 están implementados, verificados y archivados en
[completados](../completados/README.md#refactor-del-reglamento-de-viajar).
El [informe de verificación](../../tecnica/acciones/verificacion-viajar-reglamento.md)
distingue el cierre funcional realizado de la valoración de jugabilidad que
hará el usuario al probarlo. No quedan tareas de desarrollo de esta secuencia.

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

## Antecedentes sustituidos: Viajar por expediciones y proyecto humano

Los planes 016 a 019 y 021 están archivados en `../completados/`. La versión
posterior quedó conservada en los commits locales `2ad4e54` y `b4cc2e4` antes
de abrir `refactor/viajar-reglamento`. La prueba manual detectó diferencias de
tiempo y dificultades de navegación, y se decidió reemplazar ese diseño.

| Plan | Estado operativo | Tratamiento |
|---|---|---|
| [PLAN-2026-022](./2026-10-19-22-refactorizar-viajar-proyecto-humano.md) | Sustituido; implementación anterior conservada | Historial, sin nuevas tareas de corrección. |
| [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md) | Sustituido; implementación anterior conservada | Historial, sin nuevas tareas de corrección. |
| [PLAN-2026-024](./2026-10-19-24-presentar-proyecto-viaje-terminal.md) | Sustituido; cierre manual anterior incompleto | No se declara completado por el refactor nuevo. |
| [PLAN-2026-020](./2026-10-17-20-equilibrar-viajar-expediciones.md) | Sustituido; cierre anterior incompleto | No bloquea la secuencia 025–031. |

Estos archivos permanecen aquí para conservar su implementación y verificaciones
reales. No se trasladan a `completados/` ni se completan sus casillas por haber
desarrollado el reglamento nuevo. Las tareas actuales pertenecen exclusivamente
a PLAN-2026-025 a PLAN-2026-031.
