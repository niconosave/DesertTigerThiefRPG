# Plan: Equilibrar y cerrar Viajar por expediciones

| Campo | Valor |
|---|---|
| ID | PLAN-2026-020 |
| Estado | Sustituido; cierre anterior incompleto |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-24 |

> Sustituido por la secuencia PLAN-2026-025 a PLAN-2026-031 del
> [refactor implementado](../completados/README.md#refactor-del-reglamento-de-viajar).
> Conserva el historial anterior; no es una dependencia ni una autorización de
> trabajo adicional y sus verificaciones pendientes no se declaran completadas.

## Resultado buscado

Todo el atlas utiliza de forma coherente la versión de expediciones de `Viajar`,
con ritmo apropiado para partidas breves, contenido validado, recorridos
terrestres y marítimos jugables y documentación sincronizada con el código.

## Referencias

- [Criterios de Viajar](../../tecnica/acciones/02-viajar-expediciones.md#15-pruebas-y-criterios-de-aceptación).
- [Cronología causal](../../tecnica/acciones/03-cronologia-causal-viaje.md#12-pruebas-y-criterios-de-aceptación).
- [Proyecto humano](../../tecnica/acciones/04-proyecto-humano-viajar.md#11-pruebas-y-criterios-de-aceptación).
- [Invariantes particulares](../../tecnica/acciones/particulares/00-contrato-resolucion.md#10-errores-e-invariantes).
- [Atlas regional](../../contenido/atlas-regional.md).
- [Trayectos](../../contenido/trayectos-viaje.md).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).
- Antecedente completado: [PLAN-2026-019](../completados/2026-10-17-19-incorporar-navegacion.md).
- Dependencia directa para el cierre: [PLAN-2026-024](./2026-10-19-24-presentar-proyecto-viaje-terminal.md).
- Secuencia de correcciones: [PLAN-2026-021](../completados/2026-10-19-21-conservar-trazas-causales.md),
  [PLAN-2026-022](./2026-10-19-22-refactorizar-viajar-proyecto-humano.md),
  [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md) y
  [PLAN-2026-024](./2026-10-19-24-presentar-proyecto-viaje-terminal.md).

## Alcance

### Incluido

- caracterización física y revisión de la red completa del atlas;
- ajuste de valores ya documentados a partir de pruebas, actualizando primero su fuente normativa;
- proyecto humano, procedencia de medios, cronología exhaustiva, escenas
  dramáticas contextualizadas por `Viajar`, textos causales, seguimiento del
  progreso y resúmenes NPC fieles a todas las ejecuciones;
- eliminación del comportamiento provisional y código duplicado de Viajar;
- sincronización final de estados documentales.

### No incluido

- nuevas acciones generales, Objetos, clima, economía, guardado o acabado artístico avanzado.

## Precondiciones

- PLAN-2026-013 a PLAN-2026-019 completados.
- PLAN-2026-021 a PLAN-2026-024 completados antes del cierre de este plan.

## Pasos

1. [x] Auditar los veintiséis Lugares y toda conexión contra reglas y briefs vigentes.
2. [x] Incorporar únicamente las Características físicas necesarias y retirar `inalcanzable` del contenido y de las reglas de expedición.
3. [x] Crear una matriz automatizada de recorridos cortos, largos, terrestres, mixtos y marítimos con azar controlado.
4. [x] Medir duración, decisiones, consumo y momentos; registrar cualquier ajuste de diseño en su documento antes de cambiar valores.
5. [x] Refinar resúmenes, diagnósticos y presentación simple sin añadir mecánicas.
6. [x] Eliminar BFS, sucesos o helpers provisionales que ya no sean una ruta de compatibilidad necesaria.
7. [x] Ejecutar todas las pruebas, validador, controles y build de producción.
8. [ ] Realizar una sesión manual conjunta que cubra tierra, supervivencia, camellos y mar.
9. [x] Implementar PLAN-2026-021 a PLAN-2026-024, repetir verificaciones y
       actualizar estados y enlaces documentales.

## Verificación

- [x] Ningún Lugar oficial almacena dificultad, duración o valoración de una acción.
- [x] No queda una ruta activa que presuma barco, use tiles o calcule duración solo por conexiones.
- [x] Toda acción particular ejecutada queda reconocible en momentos causales
      ordenados, sin límite de escenas o viñetas y sin ocultar ejecuciones
      rutinarias, automáticas o repetidas; no se exige una escena por acción.
- [x] Todo viaje permite reconocer propósito, alternativas válidas, plan,
      procedencia de sus medios, subobjetivos e interpretaciones, y ninguna
      embarcación o montura aparece sin una fuente material visible.
- [x] Jugador y NPC atraviesan las mismas fichas, operaciones y límites.
- [x] Todos los destinos del atlas terminan en un estado válido con las precondiciones documentadas.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [ ] La revisión manual confirma escenas con carácter dramático coherente,
      legibilidad causal aun cuando agrupen varias acciones, seguimiento
      geográfico y temporal, ritmo y continuidad de la partida.
- [ ] Se cumplieron los controles y la definición de terminado de `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Este plan puede ajustar números, pero no inventarlos: toda modificación reglada se aprueba primero en la ficha canónica.
- Los problemas artísticos no bloqueantes se registrarán para otro incremento en vez de ampliar este alcance.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: pendiente de revisión manual.
- Resultado: la implementación reglada y su verificación automática funcionan;
  la prueba manual confirmó viajes terrestres y marítimos, pero detectó que la
  presentación no permite observar suficientemente la simulación, vivir sus
  situaciones como escenas dramáticas contextualizadas ni seguir con claridad
  el Lugar y el tiempo durante el recorrido.
- Verificaciones ejecutadas: 26 Lugares auditados, matriz automatizada que exige alcanzar cada destino, continuidad tierra–agua–tierra, cronología larga y NPC, paginación de detalle, formato, check, Clippy estricto, 84 pruebas y build release.
- Desviaciones respecto del plan: las revisiones manuales de PLAN-2026-016 a PLAN-2026-019 se agruparon aquí por pedido del usuario.
- Trabajo posterior: repetir la sesión manual completa terrestre y marítima
  antes de cerrar este plan y PLAN-2026-024.
