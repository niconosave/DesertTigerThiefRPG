# Plan: Incorporar el atlas regional jugable

| Campo | Valor |
|---|---|
| ID | PLAN-2026-012 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-17 |

## Resultado buscado

El contenido base ofrece los veinte Lugares aprobados del atlas, una red conexa
terrestre y marítima y rótulos que acompañan visualmente la geografía. Todos son
destinos válidos de `Viajar` sin modificar las reglas actuales de la acción.

## Referencias

- [Atlas regional aprobado](../../contenido/atlas-regional.md).
- [Formato y validación de Lugares](../../tecnica/componentes/03-contenido.md#3-entradas-y-datos-cubiertos).
- [Autoría y validación de Relaciones](../../tecnica/componentes/10-relaciones.md#4-formato-ron-y-autoría).
- [Disponibilidad y ruta de Viajar](../../tecnica/acciones/01-viajar.md#3-disponibilidad-y-cálculo-de-ruta).
- [Ciclo incremental](../../metodologia-desarrollo.md#4-ciclo-incremental).

## Alcance

### Incluido

- los quince Lugares continentales y los cinco marítimos o insulares del atlas;
- la red aprobada, declarada una sola vez por conexión;
- nombres visibles y rótulos adaptados al mapa;
- migración de los nombres provisionales a Hamada Roja y Valle del Drâa;
- validaciones automáticas y una revisión manual conjunta al terminar el lote.

### No incluido

- nuevas reglas para interpretar terrenos, navegación, asentamientos o medios;
- nuevos Actores, acciones o ilustraciones;
- selección manual o dibujo de rutas.

## Pasos

1. [x] Aprobar el atlas como fuente creativa del lote.
2. [x] Crear los Lugares y enumerarlos en el Mundo.
3. [x] Declarar la red terrestre y marítima sin duplicados.
4. [x] Componer centros y rótulos sobre la matriz actual.
5. [x] Actualizar fixtures y documentación afectada.
6. [x] Ejecutar validaciones automáticas y build de producción.
7. [x] Dejar el lote completo disponible para una única revisión manual final.

## Verificación

- [x] El cargador acepta veinte Lugares y veintiocho Relaciones.
- [x] El grafo completo es conexo y conserva la ruta mínima del recorrido inicial.
- [x] Todos los rótulos caben en el mapa y corresponden a sus nombres.
- [x] Pasan los controles automáticos y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Las claves de los tres Lugares existentes se conservan para no romper Lua ni
  referencias; solo cambian sus nombres visibles.
- Las Características candidatas sin regla se mantienen en el atlas y no se
  copian todavía a RON.
- La densidad y forma de los rótulos se evaluará en conjunto, no con pausas por
  cada Lugar.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-17.
- Resultado: veinte Lugares, veintiocho conexiones y sus rótulos están cargados
  como un único atlas navegable.
- Verificaciones ejecutadas: formato, check, Clippy estricto, 51 pruebas
  automáticas, build de release, validación de dimensiones, referencias,
  rótulos, conectividad y conservación del recorrido inicial.
- Desviaciones respecto del plan: ninguna; la revisión manual conjunta confirmó
  que distintos destinos funcionan y que la composición general es utilizable.
- Trabajo posterior: diseñar situaciones de viaje para la naturaleza de cada región.
