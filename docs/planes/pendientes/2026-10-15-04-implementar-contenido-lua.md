# Plan: Implementar contenido RON e integración Lua

| Campo | Valor |
|---|---|
| ID | PLAN-2026-004 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

`desert-tiger-app` carga y valida atómicamente un conjunto RON editable, crea
estados independientes de Mundo y mapa, y valida o evalúa la política Lua del
fixture sin permitirle mutar el juego.

## Referencias

- [Brief del escenario inicial](../../contenido/escenario-inicial.md).
- [Flujo de carga e instanciación](../../tecnica/componentes/03-contenido.md#4-flujo-de-carga-e-instanciación).
- [Formato y organización RON](../../tecnica/componentes/03-contenido.md#5-formato-y-organización).
- [Errores, pruebas y aceptación de contenido](../../tecnica/componentes/03-contenido.md#7-errores-pruebas-y-aceptación).
- [Fixture vertical oficial](../../tecnica/componentes/03-contenido.md#fixture-vertical-oficial).
- [Contrato Rust de Lua](../../tecnica/componentes/08-integracion-lua.md#3-contrato-rust-interno).
- [API disponible dentro de Lua](../../tecnica/componentes/08-integracion-lua.md#4-api-disponible-dentro-de-lua).
- [Entorno aislado](../../tecnica/componentes/08-integracion-lua.md#5-entorno-aislado-y-determinista).
- Plan previo: [PLAN-2026-003](./2026-10-15-03-implementar-motor-viajar.md).

## Alcance

### Incluido

- materialización del brief creativo aprobado en los archivos de contenido;
- DTO privados, lectura segura, deserialización y validación acumulada;
- catálogos de claves y presentación, tipos visuales y `EstadoMapa`;
- `ContenidoValidado` compartido e instanciación independiente;
- compilación y evaluación aislada de la política Lua con límites;
- contenido mínimo editable del fixture en `content/base`;
- pruebas válidas, inválidas y de aislamiento.

### No incluido

- ampliación narrativa más allá del brief o setting definitivo;
- recursos artísticos propios de la terminal;
- ejecución de la fase NPC o fachada de sesión;
- mutación de variables globales desde Lua o archivos `.lua` separados.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-003](./2026-10-15-03-implementar-motor-viajar.md) están resueltos y
  registrados; el plan permanece pendiente hasta integrar el incremento.
- El [brief del escenario](../../contenido/escenario-inicial.md) está aprobado y
  el fixture de los documentos técnicos refleja sus Lugares, Actores y roles.

## Pasos

1. [x] Implementar los tipos públicos de presentación y los catálogos internos
       con sus órdenes, consultas y mutaciones atómicas aprobadas.
2. [x] Implementar los DTO RON estrictos y la resolución segura y determinista
       de `world.ron`, Mapa, Lugares y Actores enumerados.
3. [x] Convertir y validar referencias, IDs, fecha, globales, Características,
       matriz, rótulos y retratos antes de construir el contenido opaco.
4. [ ] Implementar diagnósticos agregados y estables, evitando cascadas y la
       exposición de errores físicos o bloques completos.
5. [x] Implementar validación y evaluación Lua con una máquina nueva, entorno de
       solo lectura, API mínima, presupuestos y conversión declarativa a solicitud.
6. [x] Integrar políticas validadas y catálogos dentro de `ContenidoValidado`, y
       comprobar la creación independiente de Mundo y EstadoMapa.
7. [x] Materializar el brief y el fixture técnico en `content/base`: un mapa,
       dos Lugares y los Actores aprobados, autocontenidos, con los retratos
       mínimos y la única política Lua independiente.
8. [ ] Añadir los ejemplos inválidos y pruebas de rutas, RON, referencias,
       Unicode, duplicados, Lua, diagnósticos y dos instancias aisladas.
9. [x] Ejecutar la validación sobre `content/base`, los controles de app y de la
       workspace, y registrar resultados o desviaciones.

## Verificación

- [ ] El fixture cumple exactamente el
      [recorrido de contenido](../../tecnica/componentes/03-contenido.md#fixture-vertical-oficial)
      y la intención del brief sin fijar todavía un setting definitivo.
- [ ] Se cumplen las pruebas de
      [contenido](../../tecnica/componentes/03-contenido.md#7-errores-pruebas-y-aceptación)
      y de [Lua](../../tecnica/componentes/08-integracion-lua.md#9-pruebas-y-aceptación).
- [ ] Dos instanciaciones comparten solo definiciones inmutables y no comparten
      Mundo, fecha, globales o celdas mutables.
- [ ] Lua no accede a filesystem, terminal, azar o mutación del Mundo y todo
      fallo conserva el estado.
- [ ] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Los archivos mínimos se escribirán para probar el esquema, no para decidir por
  accidente el setting final; cualquier decisión narrativa nueva se consultará
  antes de incorporarla como contenido canónico.
- Las rutas y límites Lua son fronteras de seguridad accidental: sus pruebas no
  se sustituirán por una comprobación manual.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: cargador RON atómico, contenido base, mapa instanciable y sandbox
  Lua funcionales; el plan sigue abierto por la agregación de diagnósticos y la
  matriz completa de fixtures inválidos.
- Verificaciones ejecutadas: fixture oficial 96 × 32, instancias aisladas,
  política enero/febrero, consultas tipadas, entorno sin bibliotecas, escritura
  rechazada, límite de instrucciones, respuesta exacta, `fmt`, `check`, Clippy y
  pruebas de workspace.
- Desviaciones respecto del plan: el cargador actual devuelve el primer
  diagnóstico estable en vez de acumular varios; debe completarse antes de
  cerrar este plan.
- Trabajo posterior: [PLAN-2026-005](./2026-10-15-05-implementar-sesion-fase-npc.md).
