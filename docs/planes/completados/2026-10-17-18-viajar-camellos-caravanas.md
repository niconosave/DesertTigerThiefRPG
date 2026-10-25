# Plan: Viajar con camellos y caravanas

| Campo | Valor |
|---|---|
| ID | PLAN-2026-018 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

El Grupo puede elegir camellos en etapas compatibles y obtiene ventajas y
costes distintos de la marcha, sin administrar animales individuales.

## Referencias

- [Medios por etapa](../../tecnica/acciones/02-viajar-expediciones.md#7-medios-por-etapa).
- [Cabalgar](../../tecnica/acciones/particulares/01-movimiento-terrestre.md#7-cabalgar).
- [Guiar caravana](../../tecnica/acciones/particulares/01-movimiento-terrestre.md#8-guiarcaravana).
- [Bandido y corredor](../../contenido/trayectos-viaje.md#apoyo-inicial-para-probar-las-reglas).
- Dependencia: [PLAN-2026-017](../completados/2026-10-17-17-sobrevivir-acampar-ruta.md).

## Alcance

### Incluido

- estado transitorio del medio terrestre;
- `Cabalgar` y `GuiarCaravana`;
- elección contextual entre pie y camellos;
- alimento abstracto del medio, pastizal y fatiga;
- capacidades y acceso del Bandido aprobados.

### No incluido

- camellos como Objetos, cantidad de animales, compra, robo, cría o combate montado.

## Precondiciones

- PLAN-2026-017 completado.

## Pasos

1. [x] Incorporar disponibilidad y estado transitorio de camellos a Expedición.
2. [x] Implementar y probar `Cabalgar` y `GuiarCaravana`.
3. [x] Seleccionar una única acción dominante por etapa y ofrecer medio solo cuando cambie el resultado.
4. [x] Integrar consumo, pastizal, fatiga y alternativa a pie en terreno incompatible.
5. [x] Materializar las capacidades aprobadas del Bandido y los hechos de terreno necesarios.
6. [x] Adaptar pronóstico, ruta, decisiones, viñetas y resolución NPC.
7. [x] Añadir pruebas de terreno favorable, incompatible, carga y exclusión mutua.
8. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] `conCamellos` representa medios suficientes para todo el Grupo sin crear entidades individuales.
- [x] La etapa ejecuta Marchar, Cabalgar o GuiarCaravana, nunca más de una para el mismo avance.
- [x] Un terreno incompatible hace continuar esa etapa a pie.
- [x] La ruta y el pronóstico cambian mediante estimaciones particulares, no una bonificación hardcodeada en Viajar.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [x] Se cumplieron los controles aplicables; la revisión manual se consolidó expresamente en PLAN-2026-020.

## Riesgos y decisiones operativas

- El estado del medio no se persistirá al cerrar la Expedición; solo las Características aprobadas conservan acceso cualitativo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: marcha, cabalgata y caravana son alternativas dominantes por etapa, con consumo y capacidades del Grupo.
- Verificaciones ejecutadas: contenido oficial, formato, check, Clippy estricto, pruebas unitarias y del workspace y build release.
- Desviaciones respecto del plan: en terreno incompatible el Grupo conserva los camellos como acceso cualitativo pero resuelve la etapa a pie; no se añadió estado persistente de animales. La revisión manual se realizará en PLAN-2026-020.
- Trabajo posterior: [PLAN-2026-019](./2026-10-17-19-incorporar-navegacion.md).
