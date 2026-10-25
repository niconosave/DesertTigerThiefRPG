# Plan: Implementar el motor y la acción Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-003 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-20 |

## Resultado buscado

El núcleo puede enumerar destinos e iniciar, suspender y completar `Viajar` con
las tres ramas aprobadas, decisiones opacas, azar inyectado, narrativa ordenada,
duración y traslado semántico verificables sin frontend.

## Referencias

- [Datos, entradas y salidas del motor](../../tecnica/componentes/05-motor-acciones.md#4-datos-entradas-y-salidas).
- [Ejecución y estado reanudable](../../tecnica/componentes/05-motor-acciones.md#5-ejecución-y-estado-reanudable).
- [Azar compartido](../../tecnica/componentes/05-motor-acciones.md#6-azar-compartido-y-reproducible).
- [Errores del motor](../../tecnica/componentes/05-motor-acciones.md#8-errores-y-validaciones).
- [Disponibilidad y ruta del contrato histórico ejecutado](../../tecnica/acciones/01-viajar.md#3-disponibilidad-y-cálculo-de-ruta).
- [Ejecución común del contrato histórico ejecutado](../../tecnica/acciones/01-viajar.md#6-ejecución-común).
- [Pruebas del contrato histórico ejecutado](../../tecnica/acciones/01-viajar.md#14-pruebas-y-criterios-de-aceptación).
- [Contrato físico actual de Viajar](../../tecnica/acciones/02-viajar-expediciones.md).
- [Proyecto humano actual de Viajar](../../tecnica/acciones/04-proyecto-humano-viajar.md).
- Plan previo: [PLAN-2026-002](./2026-10-15-02-implementar-modelo-mundo-calendario.md).

Este plan conserva el alcance histórico de la primera versión ya implementada.
`01-viajar.md` es ahora un documento obsoleto conservado como evidencia y no una
fuente normativa vigente. Los contratos actuales son `02` y `04`; sus sucesores
completados y la secuencia activa PLAN-2026-021 a PLAN-2026-024 son responsables
de migrar el código. Las casillas completadas de este archivo no afirman que esa
especificación posterior ya esté implementada.

## Alcance

### Incluido

- protocolo de solicitudes, tramos, emisiones, cierres y resultados;
- `EjecucionAccion` pública y opaca, IDs de decisión y reanudación validada;
- trait de azar y fuente programada para pruebas;
- disponibilidad, validaciones y máquina de estados completa de `Viajar`;
- acciones particulares internas, textos, escenas y duraciones aprobadas;
- pruebas unitarias y de integración dentro de `desert-tiger-core`.

### No incluido

- avance de la fecha después de la presentación;
- elección automática de NPC, Lua, sesión de aplicación o vistas;
- nuevas acciones generales, datos configurables para las ramas o Relaciones.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-002](./2026-10-15-02-implementar-modelo-mundo-calendario.md) están
  resueltos y registrados; el plan permanece pendiente hasta integrar el
  incremento.

## Pasos

1. [x] Implementar los tipos públicos del protocolo, manteniendo opacos los IDs
       internos y el estado reanudable.
2. [x] Implementar `FuenteAzar`, sus errores y una fuente determinista exclusiva
       de pruebas sin introducir `rand` en el núcleo.
3. [x] Implementar `destinos_disponibles` y la validación inicial de `Viajar` en
       el orden normativo, sin consumo de azar ante rechazo.
4. [x] Implementar las acciones particulares internas y la acumulación
       comprobada de `Duracion`.
5. [x] Implementar las tres ramas y sus fronteras usando exactamente el orden,
       textos, opciones, claves de escena y consumos de azar documentados.
6. [x] Integrar el traslado confirmado y emitir una sola consecuencia semántica
       antes de la finalización, sin volver a aplicar el cambio.
7. [x] Implementar la clasificación de rechazo y fallo fatal, garantizando el
       estado intacto en toda respuesta recuperable inválida.
8. [x] Añadir pruebas para jugador, Grupo y NPC independiente, las tres ramas,
       todas las opciones, secuencias de azar y límites de duración.
9. [x] Ejecutar los controles del núcleo y la workspace, y registrar resultados
       o cualquier corrección documental necesaria.

## Verificación

- [x] Se cumplieron los criterios del motor y de `Viajar` vigentes al ejecutar
      este alcance histórico. La verificación de la especificación actual
      pertenece a los planes sucesores y, para el refactor vigente, a
      [PLAN-2026-022](./2026-10-19-22-refactorizar-viajar-proyecto-humano.md).
- [x] La misma secuencia de azar y respuestas produce las mismas emisiones,
      duración y consecuencia.
- [x] El motor nunca adelanta la fecha ni conoce Lua, RON, terminal o tiles.
- [x] `cargo tree -p desert-tiger-core` continúa sin `rand`, `rand_chacha` ni
      dependencias de presentación.
- [x] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Las ramas contienen literales y órdenes normativos; se usarán pruebas de
  secuencia completa para detectar cambios accidentales.
- Una consecuencia puede confirmarse antes de que termine la presentación; las
  pruebas del motor no deben atribuirle responsabilidades de la futura sesión.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente del cierre conjunto en PLAN-2026-007.
- Resultado: alcance acotado implementado; motor reanudable, azar inyectable y
  las tres ramas completas de `Viajar` funcionan sin frontend.
- Verificaciones ejecutadas: 25 pruebas unitarias de `core`, incluidos textos,
  escenas, rechazos, azar, decisiones, duración y traslados, más `fmt`, `check` y
  `clippy -D warnings`, todos satisfactorios.
- Desviaciones respecto del plan: ninguna. Las extracciones para elegir
  automáticamente una opción NPC y el recorrido mediante la fachada quedaron
  implementados por PLAN-2026-005, su componente propietario.
- Trabajo posterior: [PLAN-2026-004](./2026-10-15-04-implementar-contenido-lua.md).
