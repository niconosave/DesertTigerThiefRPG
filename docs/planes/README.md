# Planes de implementación

Los planes convierten documentación técnica ya discutida en pasos de desarrollo
pequeños, ordenados y verificables.

El sistema de eventos narrativos está implementado:
[PLAN-2026-046](./completados/2026-11-04-46-implementar-eventos-narrativos.md),
basado en el [contrato 23](../tecnica/componentes/23-eventos-narrativos.md).
El plan registra las verificaciones y la demo jugable aislada.

El audio procedural de la fogata está completado y aceptado por el usuario en
[PLAN-2026-041](./completados/2026-10-26-41-incorporar-audio-fogata.md), según los
[contratos 17](../tecnica/componentes/17-audio-procedural.md) y
[18](../tecnica/componentes/18-crepitar-fogata.md).

La selección de colores nativos o fijos está
[completada en PLAN-2026-040](./completados/2026-10-26-40-resolver-paleta-terminal.md),
según el [contrato 16](../tecnica/componentes/16-paleta-terminal.md).

La fogata procedural e iluminación del campamento está
[completada en PLAN-2026-039](./completados/2026-10-26-39-animar-iluminar-fogata.md),
según el [contrato técnico 15](../tecnica/componentes/15-fogata-procedural.md).

El desarrollo de parentesco y Actores de contexto está
[completado en PLAN-2026-038](./completados/2026-10-26-38-incorporar-parentesco-actores-contexto.md),
según el [contrato técnico 14](../tecnica/componentes/14-parentesco-actores-contexto.md).

La secuencia de Acampar, PLAN-2026-036 y PLAN-2026-037, está
[completada](./completados/README.md#acción-general-acampar). Su comportamiento
implementado se define en el [contrato técnico](../tecnica/acciones/07-acampar.md).

La secuencia de refactor de `Viajar`, PLAN-2026-025 a PLAN-2026-031, está
[completada](./completados/README.md#refactor-del-reglamento-de-viajar).
El índice de [pendientes](./pendientes/README.md) conserva otros alcances y
antecedentes sustituidos sin atribuirles verificaciones del refactor nuevo.

Los paisajes del entorno están completados y aceptados por el usuario en el
[plan 042](./completados/2026-10-26-42-incorporar-paisajes-sonoros.md), según el
[contrato 19](../tecnica/componentes/19-paisajes-sonoros.md).

## Flujo

La música ocasional y las notas animadas del campamento están completadas y
aceptadas por el usuario en
[PLAN-2026-043](./completados/2026-10-26-43-integrar-musicos-campamento.md), según
el [contrato 20](../tecnica/componentes/20-musica-campamento.md).

1. Copiar [`PLANTILLA.md`](./PLANTILLA.md) a
   `pendientes/AAAA-MM-DD-NN-nombre.md`, usando el siguiente ID y ordinal
   disponibles.
2. Enlazar las secciones aplicables de la documentación técnica.
3. Definir alcance, exclusiones, pasos y verificaciones antes de implementar.
4. Mantener el plan actualizado si la ejecución se desvía de forma relevante.
5. Cumplir la [definición de terminado](../metodologia-desarrollo.md#8-definición-de-terminado).
6. Completar resultado, verificaciones y desviaciones al terminar.
7. Mover el mismo archivo a `completados/`; no crear un resumen distinto que
   pierda la historia del plan.

Un plan permanece en `pendientes/` mientras le falte implementación o alguna
verificación obligatoria. Si se abandona, debe indicar el motivo; no se presenta
como completado.
