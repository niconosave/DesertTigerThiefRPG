# Plan: conservar la paleta nativa y ofrecer colores fijos

| Campo | Valor |
|---|---|
| ID | PLAN-2026-040 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

Los retratos conservan los tonos de la terminal al acampar y la iluminación usa
solo colores cuya equivalencia RGB se conoce. Una opción permite jugar con una
paleta fija coherente en todo el frontend.

## Referencias

- [Contrato 16: modos](../../tecnica/componentes/16-paleta-terminal.md#1-alcance).
- [Argumentos y resolución](../../tecnica/componentes/16-paleta-terminal.md#2-argumentos-y-resolución).
- [Consulta y entrada](../../tecnica/componentes/16-paleta-terminal.md#3-consulta-y-entrada).
- [Integración](../../tecnica/componentes/16-paleta-terminal.md#4-integración).
- [Verificación](../../tecnica/componentes/16-paleta-terminal.md#5-verificación-y-aceptación).

## Alcance

### Incluido

- Resolver colores, consultar la paleta, filtrar respuestas e incorporar la CLI.
- Integración con fogata, render completo, pruebas y documentación.

### No incluido

- Cambios de reglas, actores o arte, selector gráfico y actualización en vivo.

## Precondiciones

- Contrato 16 resuelto y fogata del plan 039 funcionando.
- Conservar los cambios ajenos presentes en el árbol de trabajo.

## Pasos

1. [x] Implementar paleta, modos y argumentos conforme a las secciones 1–2.
2. [x] Implementar parser, consulta y cola de entrada de la sección 3.
3. [x] Integrar resolución de iluminación y render fijo según la sección 4.
4. [x] Cubrir los casos automáticos de la sección 5 y regresiones del frontend.
5. [x] Ejecutar controles, validación oficial y compilación release.
6. [x] Probar el ejecutable en PTY con respuestas completas, parciales, tardías,
       inexistentes y modo fijo; revisar la apariencia y los controles.
7. [x] Actualizar contratos e índices, registrar resultados y archivar el plan.

## Verificación

- [x] Se cumplen los escenarios y los casos del contrato 16, sección 5.
- [x] Pasan fmt, check, clippy, pruebas de workspace y build release.
- [x] Contenido y recursos oficiales cargan correctamente.
- [x] Se realiza la revisión del ejecutable y su restauración de terminal.
- [x] Se cumple la [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- El filtro debe preservar las teclas del jugador y consumir respuestas tardías.
  Se verifica con fragmentación y una terminal emulada antes del cierre.
- Una decisión técnica nueva se resuelve en el contrato, sin introducirla aquí.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26.
- Resultado: modo nativo conservado, consulta opcional de paleta por color,
  iluminación sin equivalencias inventadas y modo fijo para todo el frontend.
- Verificaciones ejecutadas: pasan fmt, check, clippy, 123 pruebas de workspace
  y build release, con validación de contenido, Lua y recursos oficiales. Ocho
  pruebas nuevas cubren el incremento. Se ejercitaron seis escenarios del
  ejecutable en PTY con paleta personalizada, respuestas fragmentadas,
  parciales, ausentes y tardías, tecla durante el inicio y modo fijo; todos
  salieron con código 0 y restauración de terminal. Se revisaron capturas RGB.
  El [informe técnico](../../tecnica/componentes/16-paleta-terminal.md#7-verificación-de-la-implementación--2026-10-26)
  conserva el detalle.
- Desviaciones respecto del plan: ninguna de alcance. Sin dependencias nuevas.
- Trabajo posterior: diagnóstico RGB y cambios de tema en vivo permanecen
  aplazados según el contrato; no quedan bloqueos de este incremento.
