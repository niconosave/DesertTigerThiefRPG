# Plan: Agregar Lugares y recorrer expediciones por etapas

| Campo | Valor |
|---|---|
| ID | PLAN-2026-014 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

`Viajar` recorre y confirma etapas reales mediante una `Expedicion`, elige una
ruta ponderada y permite detenerse en los nuevos caminos, pasos y vados sin
depender de la cantidad bruta de conexiones.

## Referencias

- [Lugares y etapas](../../tecnica/acciones/02-viajar-expediciones.md#3-lugares-y-etapas-del-recorrido).
- [Estado Expedicion](../../tecnica/acciones/02-viajar-expediciones.md#4-estado-transitorio-expedicion).
- [Ruta automática](../../tecnica/acciones/02-viajar-expediciones.md#5-destinos-conocimiento-y-ruta-automática).
- [Movimiento terrestre](../../tecnica/acciones/particulares/01-movimiento-terrestre.md).
- [Brief de nuevos Lugares](../../contenido/trayectos-viaje.md#5-lote-inicial).
- Dependencia: [PLAN-2026-013](../completados/2026-10-17-13-establecer-resolucion-particular.md).

## Alcance

### Incluido

- estado reanudable de Expedición, fecha virtual y traslado por etapa;
- `Orientarse`, `Marchar` y `Descender`;
- búsqueda ponderada, pronósticos neutrales y recálculo de ruta;
- seis nuevos Lugares y sustitución de sus conexiones directas;
- múltiples decisiones NPC con límite defensivo;
- entre tres y seis momentos ordinarios mediante la presentación existente.

### No incluido

- `Conoce`, reservas, supervivencia, camellos o navegación reglada;
- rutas dibujadas o elegidas conexión por conexión.

## Precondiciones

- PLAN-2026-013 completado.
- El brief de nuevos Lugares y las fichas terrestres están aprobados.

## Pasos

1. [x] Crear los módulos `viajar`, planificación, expedición y situaciones, migrando el estado vigente sin duplicarlo.
2. [x] Implementar pronóstico de etapa, puntuación lexicográfica, ruta determinista y recálculo desde cada Lugar confirmado.
3. [x] Implementar `Orientarse`, `Marchar` y `Descender` y conectarlas al ciclo de etapas.
4. [x] Confirmar cada traslado mediante un lote y conservar el último Lugar ante interrupción.
5. [x] Crear los seis Lugares, sus rasgos físicos de movimiento y sus rótulos; retirar en el mismo cambio las seis conexiones sustituidas.
6. [x] Adaptar disponibilidad, fachada, viñetas y frase de pronóstico del modo destino.
7. [x] Permitir que la fase NPC responda decisiones sucesivas con un límite probado y un único resumen final.
8. [x] Añadir pruebas de grafo, rutas, acciones particulares, interrupción, presentación y contenido.
9. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] El Mundo oficial tiene los seis Lugares nuevos, no conserva los atajos directos que sustituyen y sigue conectado.
- [x] Se puede viajar hacia uno de esos Lugares, continuar desde él y quedar allí tras una interrupción.
- [x] La duración procede de las acciones particulares y no de `lugares.len() - 1` como regla directa.
- [x] Ruta, desempate, azar y decisiones NPC son reproducibles.
- [x] El recorrido automatizado Hamada Roja–Valle del Drâa muestra momentos acotados y vuelve a un estado jugable; su revisión visual se acumula para PLAN-2026-020.
- [x] Pasan validador, formato, check, Clippy estricto y pruebas; el build release se acumula para el cierre de la serie.
- [x] Se cumplieron los controles aplicables del lote; la revisión manual conjunta queda en PLAN-2026-020 por indicación del usuario.

## Riesgos y decisiones operativas

- Los nuevos Lugares y el nuevo cálculo de duración se incorporan juntos para no duplicar artificialmente días durante un estado terminado.
- Una crisis obligatoria no se elimina para cumplir el presupuesto de viñetas.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: expediciones terrestres por etapas reales, ruta ponderada, seis Lugares y traslados progresivos implementados.
- Verificaciones ejecutadas: contenido oficial, formato, check, Clippy estricto y 67 pruebas automáticas.
- Desviaciones respecto del plan: la revisión manual se consolidará en PLAN-2026-020 para no detener el desarrollo después de cada feature.
- Trabajo posterior: [PLAN-2026-015](./2026-10-17-15-incorporar-conocimiento-lugares.md).
