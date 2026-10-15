# Plan: Implementar el motor y la acción Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-003 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

El núcleo puede enumerar destinos e iniciar, suspender y completar `Viajar` con
las tres ramas aprobadas, decisiones opacas, azar inyectado, narrativa ordenada,
duración y traslado semántico verificables sin frontend.

## Referencias

- [Datos, entradas y salidas del motor](../../tecnica/componentes/05-motor-acciones.md#4-datos-entradas-y-salidas).
- [Ejecución y estado reanudable](../../tecnica/componentes/05-motor-acciones.md#5-ejecución-y-estado-reanudable).
- [Azar compartido](../../tecnica/componentes/05-motor-acciones.md#6-azar-compartido-y-reproducible).
- [Errores del motor](../../tecnica/componentes/05-motor-acciones.md#8-errores-y-validaciones).
- [Solicitud y disponibilidad de Viajar](../../tecnica/acciones/01-viajar.md#2-solicitud-y-disponibilidad).
- [Ejecución de Viajar](../../tecnica/acciones/01-viajar.md#3-ejecución-inicial).
- [Pruebas de Viajar](../../tecnica/acciones/01-viajar.md#8-pruebas-y-aceptación).
- Plan previo: [PLAN-2026-002](./2026-10-15-02-implementar-modelo-mundo-calendario.md).

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

- [x] Se cumplen los criterios del
      [motor](../../tecnica/componentes/05-motor-acciones.md#9-pruebas-y-criterios-de-aceptación)
      y de [`Viajar`](../../tecnica/acciones/01-viajar.md#8-pruebas-y-aceptación).
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
