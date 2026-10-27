# Planes completados

Aquí se archivan los planes cuya implementación y verificación terminaron. Cada
archivo debe conservar sus pasos originales y añadir el resultado real, la fecha,
las verificaciones y cualquier desviación encontrada.

Mover un plan aquí no sustituye actualizar la documentación técnica cuando la
implementación cambia una decisión o un contrato.

## Incremento jugable de Relaciones y viaje contextual

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-17 | [PLAN-2026-010](./2026-10-16-10-incorporar-relaciones-y-rutas.md) | Relaciones cargadas e indexadas, destinos alcanzables y rutas de varios tramos. |
| 2026-10-17 | [PLAN-2026-011](./2026-10-16-11-contextualizar-viajar.md) | Viajes tranquilos y contextuales con decisiones, duración y llegada final. |
| 2026-10-17 | [PLAN-2026-012](./2026-10-17-12-incorporar-atlas-regional.md) | Veinte Lugares y una red terrestre y marítima navegable. |
| 2026-10-18 | [PLAN-2026-013](./2026-10-17-13-establecer-resolucion-particular.md) | Resolución particular, `Escalar` y operaciones atómicas del Mundo. |
| 2026-10-18 | [PLAN-2026-014](./2026-10-17-14-recorrer-expediciones-por-lugares.md) | Expediciones por etapas y corredor de seis Lugares. |
| 2026-10-18 | [PLAN-2026-015](./2026-10-17-15-incorporar-conocimiento-lugares.md) | `Conoce`, unión del conocimiento y pronósticos limitados. |
| 2026-10-18 | [PLAN-2026-016](./2026-10-17-16-preparar-consumir-provisiones.md) | Preparación, ritmo, reservas y consumo por jornadas-grupo. |
| 2026-10-18 | [PLAN-2026-017](./2026-10-17-17-sobrevivir-acampar-ruta.md) | Supervivencia, recursos, campamento y recuperación durante la ruta. |
| 2026-10-18 | [PLAN-2026-018](./2026-10-17-18-viajar-camellos-caravanas.md) | Marcha, camellos y caravanas como medios alternativos por etapa. |
| 2026-10-18 | [PLAN-2026-019](./2026-10-17-19-incorporar-navegacion.md) | Bote, barco, maniobras y crisis náuticas progresivas. |

## Proyecto humano y cronología causal de Viajar

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-20 | [PLAN-2026-021](./2026-10-19-21-conservar-trazas-causales.md) | Trazas regladas completas para todas las acciones particulares vigentes. |

## Refactor del reglamento de Viajar

Cierre técnico: 2026-10-24. La serie se integró como un incremento jugable;
los controles automáticos, recorridos manuales y ajustes están en el
[informe de verificación](../../tecnica/acciones/verificacion-viajar-reglamento.md).
El 2026-10-25 el usuario confirmó su prueba de viajes, navegación, interrupciones
y separación de un compañero, con resultado general satisfactorio. Los detalles
se corregirán en incrementos posteriores.

| Plan | Resultado |
|---|---|
| [PLAN-2026-025](./2026-10-24-25-preparar-tiempo-chequeos-reservas.md) | Preparar tiempo exacto, chequeos y reservas de Viajar. |
| [PLAN-2026-026](./2026-10-24-26-preparar-rutas-rumbos.md) | Preparar rutas y rumbo provisional de Viajar. |
| [PLAN-2026-027](./2026-10-24-27-preparar-carga-provisiones.md) | Preparar configuraciones de carga y provisiones de Viajar. |
| [PLAN-2026-028](./2026-10-24-28-ampliar-consecuencias-grupo-npc.md) | Preparar consecuencias del viaje y varios NPC independientes. |
| [PLAN-2026-029](./2026-10-24-29-construir-motor-viajar-tarjetas.md) | Construir el nuevo motor de Viajar y sus tarjetas. |
| [PLAN-2026-030](./2026-10-24-30-integrar-viajar-narracion-terminal.md) | Activar el nuevo Viajar y presentar sus escenas. |
| [PLAN-2026-031](./2026-10-24-31-validar-contenido-cerrar-refactor-viajar.md) | Validar contenido y cerrar el refactor de Viajar. |

## Ajustes del mapa

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-25 | [PLAN-2026-032](./2026-10-25-32-ampliar-mapa.md) | Mapa al doble de ancho y alto; centros y rótulos de los 26 Lugares adaptados. |

## Pruebas de retratos de compañeros

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-25 | [PLAN-2026-033](./2026-10-25-33-crear-tres-retratos-companeros.md) | Sirviente, Mercader y Niño ladrón con retratos animados e incorporados al Grupo inicial. |
| 2026-10-25 | [PLAN-2026-034](./2026-10-25-34-reinterpretar-retrato-mercader.md) | Retrato del Mercader reinterpretado desde la referencia visual, conservando ojos y parpadeo. |

## Ilustraciones de viñetas

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-25 | [PLAN-2026-035](./2026-10-25-35-importar-ilustraciones-viaje.md) | Cuatro dibujos del usuario incorporados a `scenes.ron`, con bloques Unicode de una columna. |


## Acción general Acampar

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-26 | [PLAN-2026-036](./2026-10-26-36-preparar-contenido-regla-acampar.md) | Regla core, nombres, biografías y catálogo editable de características. |
| 2026-10-26 | [PLAN-2026-037](./2026-10-26-37-integrar-acampar-fichas-terminal.md) | Fichas, navegación por relaciones, pantalla de campamento y cierre de un día con fase NPC. |

Verificación conjunta: 96 pruebas, cinco controles generales satisfactorios y
recorrido manual del ejecutable release en PTY 80×24. El detalle está en la
[sección 12 del contrato](../../tecnica/acciones/07-acampar.md#12-verificación-de-la-implementación--2026-10-26).


## Parentesco y Actores de contexto

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-26 | [PLAN-2026-038](./2026-10-26-38-incorporar-parentesco-actores-contexto.md) | Actores externos, filiación, MatadoPor, hermanos deducidos y familia del príncipe consultable desde Acampar. |

Verificación: 107 pruebas, controles generales y build release satisfactorios;
recorrido manual en PTY 80×24. La [sección 12 del contrato](../../tecnica/componentes/14-parentesco-actores-contexto.md#12-verificación-de-la-implementación--2026-10-26)
registra los resultados.

## Fogata procedural del campamento

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-26 | [PLAN-2026-039](./2026-10-26-39-animar-iluminar-fogata.md) | Llama RGB, chispas, iluminación de retratos y reloj fino con compatibilidad temporal. |

El [contrato 15](../../tecnica/componentes/15-fogata-procedural.md#11-verificación-de-la-implementación--2026-10-26)
registra controles, pruebas y revisión del ejecutable.

## Paleta nativa y colores fijos

| Fecha | Plan | Resultado |
|---|---|---|
| 2026-10-26 | [PLAN-2026-040](./2026-10-26-40-resolver-paleta-terminal.md) | Consulta de paleta, nativos intactos, iluminación de colores conocidos y CLI de colores fijos. |

El [contrato 16](../../tecnica/componentes/16-paleta-terminal.md#7-verificación-de-la-implementación--2026-10-26)
registra pruebas, escenarios de terminal y revisión del ejecutable.

## Audio de la fogata

[PLAN-2026-041](./2026-10-26-41-incorporar-audio-fogata.md) está completado:
FunDSP, salida CPAL opcional y crepitar del campamento, aceptado por el usuario.

## Paisajes sonoros del entorno

[PLAN-2026-042](./2026-10-26-42-incorporar-paisajes-sonoros.md) está completado:
viento, aves, río y mar según los Lugares y sus conexiones, durante el mapa,
Viajar y Acampar. El usuario aceptó el resultado y confirmó combinaciones de
ambientes como bosque y río. El plan conserva las verificaciones históricas
de 154 pruebas con audio, 139 sin audio y nueve recorridos PTY.
