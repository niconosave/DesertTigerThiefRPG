# Plan: Conservar las trazas causales de las acciones particulares

| Campo | Valor |
|---|---|
| ID | PLAN-2026-021 |
| Estado | Completado |
| Creado | 2026-10-19 |
| Actualizado | 2026-10-20 |

## Resultado buscado

Cada acción particular realmente resuelta devuelve al motor una traza
estructurada, completa y ordenada con los factores que aplicó, su resolución,
duración y consecuencias. La traza es una unidad mecánica, no una escena ni un
texto narrativo. Ninguna cantidad de ejecuciones permite fusionar, muestrear o
descartar esas trazas antes de que la acción general las sitúe dentro de un
proyecto humano y les otorgue su contexto.

## Referencias

- [Traza causal obligatoria](../../tecnica/acciones/particulares/00-contrato-resolucion.md#41-traza-causal-obligatoria).
- [Explicación y traza causal](../../tecnica/acciones/03-cronologia-causal-viaje.md#6-explicación-y-traza-causal).
- [Traza neutral de una acción particular](../../tecnica/acciones/03-cronologia-causal-viaje.md#62-traza-neutral-de-la-acción-particular).
- [Proyecto humano de Viajar](../../tecnica/acciones/04-proyecto-humano-viajar.md).
- [Resultados de movimiento terrestre](../../tecnica/acciones/particulares/01-movimiento-terrestre.md#9-resultados-y-estimaciones).
- [Resultados de supervivencia](../../tecnica/acciones/particulares/02-supervivencia-campamento.md#10-resultados-consecuencias-y-pronósticos).
- [Resultados de navegación](../../tecnica/acciones/particulares/03-navegacion.md#9-resultados-y-pronósticos).
- Dependencia: [PLAN-2026-019](../completados/2026-10-17-19-incorporar-navegacion.md).

## Alcance

### Incluido

- tipos neutrales del núcleo para factores, modo y desglose de resolución;
- conservación de dificultad, modificadores, dado, margen, banda, duración,
  cambios transitorios y operaciones causadas;
- procedencia, orden y visibilidad de los factores aplicados;
- producción de una traza por ejecución en movimiento, supervivencia,
  campamento y navegación;
- identidad y orden suficientes para que cada traza se enlace después con un
  único momento causal, una escena pueda reunir varios momentos y ese momento
  pueda desarrollarse en varias viñetas sin perder su correspondencia;
- pruebas de exhaustividad, orden, reproducibilidad y ocultamiento.

### No incluido

- redacción de textos para el jugador;
- composición o agrupación de escenas y viñetas por la acción general;
- propósito, propuestas, plan, subobjetivos o interpretación humana del
  resultado;
- vistas de aplicación o widgets de Ratatui;
- nuevas reglas, acciones particulares, Características o valores de balance;
- registro persistente de viajes.

## Precondiciones

- PLAN-2026-013 a PLAN-2026-019 completados.
- Contrato cronológico y fichas particulares aprobados en las referencias.
- El contrato del proyecto humano aprobado mantiene las acciones particulares
  como resoluciones neutrales subordinadas a la acción general.

## Pasos

1. [x] Incorporar los tipos comunes de traza causal sin introducir dependencias
       de aplicación, contenido o terminal en el núcleo.
2. [x] Hacer que evaluación y resolución conserven los factores realmente
       aplicados y sus datos visibles sin volver a consultar el Mundo.
3. [x] Adaptar todas las acciones particulares vigentes para producir una traza
       tanto en resoluciones imposibles y deterministas como inciertas.
4. [x] Conservar en cada traza la duración, contabilidad transitoria y
       operaciones persistentes que la ejecución causó.
5. [x] Añadir pruebas unitarias por familia y casos consecutivos que excedan
       seis ejecuciones sin pérdida, fusión ni cambio de orden.
6. [x] Ejecutar los controles generales y sincronizar solamente la documentación
       que revele una desviación real respecto del contrato aprobado.

## Verificación

- [x] Cada invocación resuelta produce exactamente una traza y una consulta pura
      no produce ninguna.
- [x] Dos invocaciones iguales consecutivas conservan dos trazas distinguibles y
      ordenadas; una secuencia de más de seis conserva todas.
- [x] La estructura no presupone ni fuerza una correspondencia de una escena o
      una viñeta por traza.
- [x] Automático, imposible y `1d20` exponen los campos que les corresponden sin
      inventar valores ausentes.
- [x] Un factor desconocido influye en la resolución autoritativa sin filtrarse
      mediante el detalle visible ni por resta de totales.
- [x] Jugador y NPC producen la misma traza con igual contexto, decisiones y
      azar.
- [x] Pasan formato, check, Clippy estricto y todas las pruebas del workspace.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Retener todos los campos puede tentar a exponer tipos internos demasiado
  amplios. Se mantendrá una proyección neutral explícita en la frontera pública.
- Los tests no fijarán prosa: comprobarán hechos, orden y visibilidad.
- Este plan preserva material causal; PLAN-2026-022 decide cómo la acción
  general lo emplea dentro del proyecto humano sin alterar sus resultados, y
  PLAN-2026-023 decide cómo ese proyecto se convierte en cronología dramática.
- Cualquier necesidad de cambiar una regla o un número se resolverá primero en
  su ficha técnica y no dentro de este plan.

## Resultado de la implementación

- Fecha de finalización: 2026-10-20.
- Resultado: el núcleo expone trazas neutrales y todas las acciones particulares
  vigentes conservan factores, resoluciones aplicadas, banda, duración,
  contabilidad transitoria y operaciones solicitadas o confirmadas.
- Verificaciones ejecutadas: formato, `cargo check`, Clippy estricto, 78 pruebas
  automatizadas y compilación de release. No correspondió prueba manual porque
  este plan no modifica comportamiento jugable ni presentación.
- Desviaciones respecto del plan: una acción de grupo puede contener varias
  resoluciones aplicadas, ordenadas e identificadas por sujeto, dentro de su
  única traza. El contrato técnico registra expresamente este caso.
- Trabajo posterior: [PLAN-2026-022](../pendientes/2026-10-19-22-refactorizar-viajar-proyecto-humano.md).
