# Plan: Sobrevivir y acampar durante la ruta

| Campo | Valor |
|---|---|
| ID | PLAN-2026-017 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

Una expedición puede buscar y tratar agua, obtener alimento, montar campamento y
descansar; sus recursos y estados forman una cadena causal capaz de permitir la
continuación o interrumpir el viaje.

## Referencias

- [Fichas de supervivencia](../../tecnica/acciones/particulares/02-supervivencia-campamento.md).
- [Ejecución por particulares](../../tecnica/acciones/02-viajar-expediciones.md#8-ejecución-mediante-acciones-particulares).
- [Tiempo y supervivencia](../../tecnica/acciones/02-viajar-expediciones.md#9-tiempo-consumo-y-supervivencia).
- [Estados progresivos](../../tecnica/componentes/11-operaciones-mundo.md#7-estados-progresivos-de-viaje).
- [Apoyo de contenido](../../contenido/trayectos-viaje.md#apoyo-inicial-para-probar-las-reglas).
- Dependencia: [PLAN-2026-016](../completados/2026-10-17-16-preparar-consumir-provisiones.md).

## Alcance

### Incluido

- las siete acciones particulares de supervivencia y campamento;
- oportunidades limitadas por Lugar y Expedición;
- calidad transitoria de agua y campamento;
- progresión de fatiga, agotamiento, sed, hambre y heridas;
- hechos naturales mínimos del Valle, Bosque y Marisma;
- viñetas y decisiones solo cuando sean significativas.

### No incluido

- medicina, enfermedades detalladas, inventario, regeneración o acción general `Acampar` desde el mapa.

## Precondiciones

- PLAN-2026-016 completado.

## Pasos

1. [x] Implementar `BuscarAgua` y `TratarAgua` con cantidades y calidades comprobadas.
2. [x] Implementar `Cazar`, `Pescar` y `Recolectar` con límites de intento.
3. [x] Implementar `MontarCampamento` y `Descansar` con calidad transitoria y operaciones persistentes.
4. [x] Construir el controlador causal que prioriza carencia, refugio y descanso sin descontar dos veces el día.
5. [x] Añadir al contenido los hechos aprobados de fuentes, fauna, peces y vegetación.
6. [x] Integrar resultados y resúmenes en la máquina de Viajar y en la fase NPC.
7. [x] Añadir pruebas de cada ficha, cadenas completas, límites y no filtración.
8. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] Los recursos solo aparecen cuando existen hechos de contenido que los justifican.
- [x] Obtener un recurso puede cubrir el consumo de esa misma jornada.
- [x] No se puede repetir indefinidamente una acción de cero días o una oportunidad agotada.
- [x] Descansar recupera conforme al campamento y no cura `herido`.
- [x] Una mala tirada rutinaria no mata directamente y toda progresión grave es visible.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [x] Se cumplieron los controles aplicables; la revisión manual se consolidó expresamente en PLAN-2026-020.

## Riesgos y decisiones operativas

- Las crisis obligatorias se procesarán aunque excedan el presupuesto narrativo ordinario.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: siete acciones particulares y un controlador causal de supervivencia, campamento y recuperación integrados en Viajar.
- Verificaciones ejecutadas: contenido oficial, formato, check, Clippy estricto, pruebas unitarias y de integración y build release.
- Desviaciones respecto del plan: las intervenciones rutinarias se eligen automáticamente por causa; solo las crisis que requieren una decisión interrumpen el flujo. La revisión manual se realizará en PLAN-2026-020.
- Trabajo posterior: [PLAN-2026-018](./2026-10-17-18-viajar-camellos-caravanas.md).
