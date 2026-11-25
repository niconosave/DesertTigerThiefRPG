# Plan: Verificación y revisión jugable de la nueva base

| Campo | Valor |
|---|---|
| ID | PLAN-2026-057 |
| Estado | Verificación técnica completada; revisión jugable del usuario pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-25 |

## Resultado buscado

Cerrar la etapa con el juego integrado y una revisión jugable de los encuentros
traducidos, conservando evidencia acotada de las reglas y consecuencias afectadas.

Este plan cierra la única etapa jugable. A diferencia de los bloques internos,
requiere el programa integrado, recorridos completos y revisión del usuario.
Al satisfacerlos se registra el resultado y se archivan juntos los bloques 052 a 057.

## Referencias

- [Verificación acotada](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#15-verificación-acotada).
- [Orden de integración](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#16-orden-de-integración).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).
- [PLAN-2026-056](./2026-11-24-56-integrar-encuentros-y-consecuencias.md).

## Alcance

### Incluido

- Verificación seleccionada de espacio, planes, puerta, corte y fronteras de integración.
- Recorridos de Atacar, Rescatar y recuperar tesoros; eventos existentes de Gorun.
- Revisión manual final, registro de resultados y cierre conjunto de la etapa.

### No incluido

- Batería global por defecto, pruebas repetidas sin cambios o nuevos sistemas.
- Preparación, sigilo, combate dinámico, generación modular e integración nueva de planes Lua.

## Precondiciones

- PLAN-2026-056 completado técnicamente y base integrada para iniciar recorridos.
- Resultados técnicos de los bloques anteriores registrados sin dar por aceptada la etapa.

## Pasos

1. [x] Revisar los contratos afectados y seleccionar las comprobaciones todavía necesarias.
2. [x] Ejecutar pruebas acotadas; corregir fallos y repetir únicamente lo justificado.
3. [x] Completar recorridos automatizados de los tres propósitos y eventos existentes.
4. [x] Preparar [instrucciones breves para la revisión jugable](../revision-jugable-base-encuentros.md).
5. [ ] Registrar la respuesta del usuario.
6. [ ] Actualizar estados documentales y cerrar juntos los planes al satisfacer la aceptación.

## Verificación

- [x] Se satisfacen los criterios automáticos observables de las mecánicas de esta etapa.
- [x] Se registran comandos, resultados y limitaciones sin inventar aceptación humana.
- [x] No se solicita una batería más amplia sin explicar necesidad y acordar su ejecución.
- [ ] La revisión jugable cubre corte por altura, puerta, robo, rescate y evento de Gorun.
- [ ] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

La aprobación técnica de un bloque no reemplaza la prueba jugable. Si surge una
ampliación ajena al alcance, registrarla por separado; no declarar terminada la
etapa por agotamiento de tiempo o por el solo paso de las pruebas automáticas.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

- Finalización técnica: 2026-11-25. Cierre conjunto: pendiente de revisión del usuario.
- Resultado: base integrada compilable con las funciones predeterminadas; recorridos
  de los tres propósitos y controles de terminal comprobados.
- La integración de ataque verifica apertura real de puerta, pausa, omisión,
  tamaño mínimo y cierre al mapa. El rescate de terminal usa DosAlturas en una
  copia temporal y comprueba ascenso real, cautivo arriba, extracción e incorporación.
  El robo verifica la extracción y que deje de ofrecerse el bien ya adquirido.
- Se corrigió el diagnóstico local restante: `ErrorEncuentro` distingue geometría,
  coordenadas, referencias, estado incompatible, azar y agotamiento de generación;
  app convierte esos diagnósticos en su frontera, sin cambiar las reglas.
- Verificaciones nuevas: terminal `integracion_encuentros_3d` (1),
  `rescate_terminal` (1), `objetos_terminal` (1), `corte_altura` (4);
  app `eventos_locales_3d` (2); core `errores_locales` (1).
  Se reutiliza la evidencia de espacio, planes, puertas y consecuencias de 052–056;
  no se repite la batería del workspace.
- `cargo check -p desert-tiger-terminal` y compilación del binario con funciones
  predeterminadas correctos. Arranque y salida normal comprobados en PTY 100x30,
  con audio desactivado; no equivale a aceptación visual humana ni a prueba de audio.
- Formato, diferencias y Clippy registrados en `/tmp/057-*.log`.
  Se conserva el aviso preexistente de argumentos de `dibujar_retratos_dialogo`.
- [Guía jugable lista](../revision-jugable-base-encuentros.md).
  Los planes no se archivan hasta registrar la respuesta del usuario.

## Revisión de los hallazgos anteriores

La [revisión inicial del código](../../tecnica/acciones/revision-base-encuentros-2026-11-24.md)
se conserva como historial. Sus hallazgos de dirección efectiva por planes,
identidades, conocimiento por espacios, conclusiones individuales y atomicidad
fueron atendidos en los bloques 052–056. No se usa el paso de los filtros antiguos
como sustituto de esos cambios ni de la aceptación humana pendiente.

## Respuesta de la revisión jugable

El usuario probó Gorun, Kazir, Tamin, copa y dos alturas. Confirmó rescate y
transporte de caídos. Detectó un bloqueo durante el robo: dos compañeros quedan
en el umbral incluso después de vencer al enemigo, sin que otro aliado esté
ocupando ese paso. Pendiente de diagnóstico y corrección; no atribuirlo sin
reproducción al bloqueo táctico de un compañero en combate.
La altura y escalera resultaron poco legibles. Se implementó la primera
[vista oblicua](../../tecnica/componentes/31-vista-oblicua-encuentros.md), pendiente
de valoración estética. La etapa no se archiva por esta prueba parcial.
