# Plan: Implementar la sesión y la fase NPC

| Campo | Valor |
|---|---|
| ID | PLAN-2026-005 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-20 |

## Resultado buscado

La fachada de `desert-tiger-app` permite crear una Partida, recorrer `Viajar`,
avanzar la fecha, ejecutar la política del NPC y volver a una `VistaMundo` con
su resumen, todo sin una terminal real.

## Referencias

- [Composición de Partida](../../tecnica/componentes/04-sesion-aplicacion.md#4-composición-de-partida).
- [Fases observables](../../tecnica/componentes/04-sesion-aplicacion.md#5-fases-observables).
- [Intenciones del frontend](../../tecnica/componentes/04-sesion-aplicacion.md#6-intenciones-del-frontend).
- [Vistas y salidas](../../tecnica/componentes/04-sesion-aplicacion.md#7-vistas-y-salidas).
- [Categorías de error](../../tecnica/componentes/04-sesion-aplicacion.md#8-categorías-de-error).
- [Secuencia de la fase NPC](../../tecnica/componentes/07-fase-npc.md#3-secuencia-normativa).
- [Registro interno y proyección breve](../../tecnica/componentes/07-fase-npc.md#4-registro-interno-y-proyección-breve).
- Plan previo: [PLAN-2026-004](./2026-10-15-04-implementar-contenido-lua.md).

### Corte histórico y propietario del trabajo nuevo

Este plan conserva el corte de la fachada y fase NPC original. Sus casillas
pendientes cubren solamente la matriz histórica de IDs, semillas, contadores,
calendario, disponibilidad y resultados NPC simples. La cronología exhaustiva
del proyecto humano, `ResumenNpc` ampliado, su duración y su nueva proyección
pertenecen exclusivamente a
[PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md). Los enlaces a
documentos vivos aportan contexto actual, pero no amplían retroactivamente este
plan ni duplican aquel trabajo.

## Alcance

### Incluido

- implementación de `Partida`, sus estados y su fachada pública;
- adaptador de azar de producción, semilla explícita y fuente inyectable;
- normalización de tramos, vistas e IDs de presentación;
- avance temporal único al cerrar la acción del jugador;
- fase NPC síncrona, decisión automática y resumen estructurado;
- clasificación recuperable o fatal y pruebas de integración sin terminal.

### No incluido

- Ratatui, teclado, dibujo, paginación o recursos visuales;
- otras acciones, guardado, concurrencia o varias políticas independientes;
- texto redactado por frontend para los resúmenes.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-004](./2026-10-15-04-implementar-contenido-lua.md) están resueltos y
  registrados; el plan permanece pendiente hasta integrar el incremento.

## Pasos

1. [x] Implementar el adaptador de `ChaCha12Rng`, la obtención de semilla y el
       constructor interno con fuente programada para pruebas.
2. [x] Implementar `Partida::nueva` con estados independientes, handle de
       contenido compartido y proyección inicial validada.
3. [x] Implementar las vistas poseídas, reexportaciones y los IDs visibles
       monotónicos sin exponer tipos de Lua o del frontend.
4. [x] Implementar `procesar` y la precedencia de intenciones para comenzar,
       presentar y reanudar la acción, conservando el estado ante rechazo.
5. [x] Normalizar emisiones y fronteras sin revelar Mundo actualizado antes de
       tiempo, y aplicar una sola vez la duración al liberar la finalización.
6. [x] Implementar la fase NPC, incluida elegibilidad, evaluación Lua, motor
       común, elección automática, descarte narrativo y control de consecuencia.
7. [x] Implementar resúmenes y mapeos exactos de creación, intención,
       incidencia recuperable y estado fatal.
8. [ ] Completar la matriz histórica de vistas, IDs antiguos, semillas,
       contador, jugador muerto, ausencia de destinos, calendario y resultados
       NPC simples, excluyendo la ampliación asignada a PLAN-2026-023.
9. [x] Recorrer mediante la fachada el fixture completo sin terminal y ejecutar
       los controles de app y workspace.

## Verificación

- [ ] La matriz histórica indicada en el paso 8 está completa; los criterios
      añadidos por el proyecto humano se verifican en PLAN-2026-023, no aquí.
- [ ] El recorrido del fixture termina con jugador y Grupo trasladados, fecha
      avanzada una vez, NPC trasladado y fecha sin segundo avance.
- [ ] Consultar vistas o rechazar IDs antiguos no consume azar ni cambia la
      instantánea vigente.
- [ ] La API pública de app no expone `mlua`, Ratatui, Crossterm ni estado mutable
      del Mundo.
- [ ] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- La sesión coordina mutaciones ya confirmadas; ninguna emisión semántica se
  volverá a ejecutar en app.
- Los fallos posteriores a una consecuencia no admiten rollback y deben quedar
  en el estado fatal documentado.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: fachada de Partida y fase NPC integradas; el recorrido oficial
  vuelve a `Disponible` con el resumen estructurado esperado.
- Verificaciones ejecutadas: recorrido con azar `[0, 0]`, rama con decisión,
  rechazo de ID antiguo, estabilidad de vistas, fecha aplicada una sola vez y
  controles completos de workspace.
- Desviaciones respecto del plan: falta completar la matriz exhaustiva de fallos
  sintéticos y desbordamientos de contadores antes de cerrar el plan.
- Trabajo posterior histórico: [PLAN-2026-006](./2026-10-15-06-implementar-frontend-terminal.md).
- Ampliación vigente de esta capa:
  [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md).
