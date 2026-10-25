# Plan: Preparar y consumir provisiones de viaje

| Campo | Valor |
|---|---|
| ID | PLAN-2026-016 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

El jugador elige una preparación relevante, la Expedición consume agua y
alimento una sola vez por jornada y `bienAprovisionado` conserva únicamente una
ventaja excepcional para el viaje siguiente.

## Referencias

- [Preparación y reservas](../../tecnica/acciones/02-viajar-expediciones.md#6-preparación-reservas-y-ritmo).
- [Tiempo y consumo](../../tecnica/acciones/02-viajar-expediciones.md#9-tiempo-consumo-y-supervivencia).
- [Convenciones de supervivencia](../../tecnica/acciones/particulares/02-supervivencia-campamento.md#2-tiempo-reservas-e-intentos).
- [Operaciones de Características](../../tecnica/componentes/11-operaciones-mundo.md#4-semántica-de-las-operaciones).
- Dependencia: [PLAN-2026-015](../completados/2026-10-17-15-incorporar-conocimiento-lugares.md).

## Alcance

### Incluido

- reservas transitorias en raciones-viajero;
- preparación ligera, suficiente y cargada según el origen;
- ritmo prudente, normal y forzado cuando difieran;
- consumo ordinario y extraordinario;
- consumo y concesión atómicos de `bienAprovisionado`;
- presentación resumida de jornadas-grupo.

### No incluido

- buscar recursos, hambre y sed persistentes, inventario, comercio o recipientes.

## Precondiciones

- PLAN-2026-015 completado.

## Pasos

1. [x] Incorporar reservas, preparación, carga y ritmo al estado de Expedición.
2. [x] Derivar las opciones válidas desde las Características concretas del origen y del pronóstico.
3. [x] Implementar las fórmulas aprobadas de jornadas-grupo y conversiones comprobadas a raciones-viajero.
4. [x] Descontar consumo ordinario una vez por día y consumos extraordinarios desde resultados particulares.
5. [x] Integrar `bienAprovisionado` mediante un lote de salida y otro de cierre cuando corresponda.
6. [x] Añadir decisiones y vistas sin crear una pantalla nueva ni mostrar opciones equivalentes.
7. [x] Incorporar al corredor los hechos de abastecimiento aprobados que correspondan a este incremento.
8. [x] Añadir pruebas de límites, grupos, duración, no negatividad y jugador/NPC.
9. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] Cada día consume exactamente una vez y ningún contador queda negativo.
- [x] La cantidad visible corresponde al tamaño del Grupo.
- [x] `bienAprovisionado` se consume y concede únicamente bajo las condiciones aprobadas.
- [x] Una opción irrelevante no aparece y el NPC solo elige entre opciones válidas.
- [x] Las carencias se entregan al controlador de supervivencia sin inventar recursos.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [x] Se cumplieron los controles aplicables; la revisión manual se consolidó expresamente en PLAN-2026-020.

## Riesgos y decisiones operativas

- El contador pertenece solo a la Expedición; no se añadirá estado numérico persistente a Actor o Mundo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: preparación, ritmo, reservas y consumo por jornadas-grupo integrados en la Expedición.
- Verificaciones ejecutadas: contenido oficial, formato, check, Clippy estricto, pruebas del workspace y build release.
- Desviaciones respecto del plan: la preparación se omite cuando el origen no permite formar opciones distintas; las etiquetas muestran jornadas-grupo y la revisión manual comparativa se realizará en PLAN-2026-020.
- Trabajo posterior: [PLAN-2026-017](./2026-10-17-17-sobrevivir-acampar-ruta.md).
