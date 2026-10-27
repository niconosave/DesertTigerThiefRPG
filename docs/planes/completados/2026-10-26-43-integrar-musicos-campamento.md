# Plan: músicos y notas animadas en el campamento

| Campo | Valor |
|---|---|
| ID | PLAN-2026-043 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

El sirviente y otros integrantes con `musico` interpretan ocasionalmente la
versión grave aceptada; unas notas salen de su retrato mientras suena.

## Referencias y alcance

Implementa el [contrato 20](../../tecnica/componentes/20-musica-campamento.md).
No agrega mecánicas de instrumentos, acciones del RPG ni música durante Viajar.
Conserva el audio ambiental y los datos artísticos existentes.

## Precondiciones

- Pieza grave aceptada y audio ambiental funcionando.
- Conservar archivos y cambios ajenos presentes en el workspace.

## Pasos

1. [x] Compartir el sintetizador con la previsualización, agregar configuración
       y preparar el buffer, según la [sección 2](../../tecnica/componentes/20-musica-campamento.md#2-datos-y-preparación).
2. [x] Implementar reproducción y estado confirmado según la
       [sección 3](../../tecnica/componentes/20-musica-campamento.md#3-agenda-y-coordinación).
3. [x] Integrar agenda, elegibilidad del grupo y sincronización del bucle;
       incorporar `musico` al sirviente y al glosario según las secciones 1–3.
4. [x] Compartir geometría y dibujar notas según la
       [sección 4](../../tecnica/componentes/20-musica-campamento.md#4-notas-sobre-los-retratos).
5. [x] Ejecutar las pruebas de la [sección 5](../../tecnica/componentes/20-musica-campamento.md#5-verificación),
       fmt, check, clippy, test y release de workspace con y sin audio; validar
       contenido y recursos y recorrer el ejecutable con captura de sonido.
6. [x] Actualizar documentos e índices, registrar resultados y revisión manual;
       archivar al cumplir la [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Verificación

- [x] Pasan las pruebas, controles, recursos y ambas compilaciones.
- [x] Se comprueba el recorrido del campamento con música y notas coordinadas.
- [x] El usuario revisó el resultado jugando, lo aceptó y se cumple la definición de terminado.

## Riesgos y decisiones operativas

La síntesis se realiza antes del stream, con memoria acotada por la partitura
validada. Una decisión nueva se registra en el contrato; el plan no la define.
Una valoración manual pendiente se registra sin atribuirla a pruebas numéricas.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26; desarrollo, verificación funcional y
  aceptación del usuario completados.
- Resultado: sirviente `musico`, pieza grave sintetizada al inicio, agenda
  ocasional, mezcla ambiental y notas sobre su retrato desde el avance real.
- Verificaciones: pasan fmt, check, clippy, 169 pruebas con audio y 145 sin
  audio; release de ambas variantes y validación del contenido oficial. Seis
  recorridos PTY verifican música/notas, salida normal, audio deshabilitado,
  salida incompatible, compilación sin audio y cierre fatal; todos restauran
  la terminal. El contrato 20 registra el detalle.
- Desviaciones: ninguna de alcance. La previsualización y el juego comparten
  síntesis sin nuevas dependencias; no se agregaron mecánicas al backend.
- Revisión manual: el usuario confirmó que la música y las notas funcionaron
  perfectamente antes de solicitar una expresión para el intérprete.
- Trabajo posterior: ese retoque se registra en el contrato 20, sin cambiar el
  alcance ni atribuirlo a las verificaciones históricas de este plan.
