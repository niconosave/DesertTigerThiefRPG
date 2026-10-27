# Plan: acompañar al grupo con paisajes sonoros

| Campo | Valor |
|---|---|
| ID | PLAN-2026-042 |
| Estado | Implementación funcional terminada; en verificación auditiva |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

Escuchar el entorno del grupo en el mapa, durante Viajar y al acampar, con
viento, aves, río, mar y bosque según los lugares y sus conexiones.

## Referencias y alcance

El [contrato 19](../../tecnica/componentes/19-paisajes-sonoros.md) define todo el
incremento y su aceptación. Se conserva el dispositivo y los controles del
[contrato 17](../../tecnica/componentes/17-audio-procedural.md). No incluye música,
audio espacial ni nuevas mecánicas de juego.

## Precondiciones

- Audio de la fogata funcionando y aceptado por el usuario.
- Preservar arte y cambios ajenos al incremento.

## Pasos

1. [x] Proyectar características y vecinos de Lugares y probarlos según la
       [sección 2](../../tecnica/componentes/19-paisajes-sonoros.md#2-selección-del-paisaje).
2. [x] Incorporar configuración y selección frontend según las
       [secciones 2–3](../../tecnica/componentes/19-paisajes-sonoros.md#3-datos-y-organización).
3. [x] Implementar generadores, mezcla y transiciones según la
       [sección 4](../../tecnica/componentes/19-paisajes-sonoros.md#4-síntesis-y-transiciones).
4. [x] Integrar durante Viajar, mapa y Acampar; ejecutar las pruebas de la
       [sección 5](../../tecnica/componentes/19-paisajes-sonoros.md#5-verificación).
5. [x] Ejecutar fmt, check, clippy y test de workspace, y compilación release,
       con features predeterminadas y con `--no-default-features`; comprobar
       recursos oficiales y recorrido del ejecutable con captura de salida.
6. [ ] Actualizar documentación e índices, registrar resultados y revisión
       auditiva; archivar al cumplir la
       [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

La valoración artística requiere escuchar el resultado jugando. Si queda
pendiente, se registra y se conserva este plan aquí. Una decisión nueva se
documenta primero en el contrato, sin introducirla únicamente en este plan.

## Verificación

- [x] Configuración oficial válida y campos incorrectos rechazados con contexto.
- [x] Selección por datos neutrales y seguimiento del lugar confirmado en Viajar.
- [x] Síntesis, mezcla, transiciones y regresiones automáticas de ambos modos
      de compilación; controles de calidad y release sin advertencias.
- [x] Captura CPAL en entorno inicial, bosque/río, costa, mar y vado; controles,
      cierre normal y fatal, recuperación sin audio y restauración de terminal.
- [ ] Revisión auditiva y aceptación del usuario jugando, según el contrato 19.
- [ ] Definición de terminado y archivo en `completados/`.

## Resultado de la implementación

- Fecha: desarrollo y verificación funcional terminados el 2026-10-26.
- Resultado: ambientes procedurales configurables, mezclados por capas durante
  el mapa, Viajar y Acampar; proyección neutral de datos y conexiones de Lugares.
- Verificaciones: fmt, check, clippy sin advertencias, test y release con y sin
  audio. Pasan 154 pruebas con audio y 139 sin audio. Nueve recorridos PTY
  comprobaron cinco entornos, audio deshabilitado, salida incompatible,
  compilación sin audio y cierre fatal; todos restauraron la terminal.
- Desviaciones: ninguna de alcance; no hay dependencias nuevas. La fogata
  aceptada se cerró y archivó en el plan 041. Este plan conserva su revisión
  auditiva pendiente; documentación e instrucciones están actualizadas.
- Trabajo posterior: escuchar los ambientes, ajustar sus niveles si hace falta
  y registrar la aceptación antes de archivar el plan.
