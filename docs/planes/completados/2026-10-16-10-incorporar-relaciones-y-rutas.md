# Plan: Incorporar Relaciones y rutas alcanzables

| Campo | Valor |
|---|---|
| ID | PLAN-2026-010 |
| Estado | Completado |
| Creado | 2026-10-16 |
| Actualizado | 2026-10-17 |

## Resultado buscado

El contenido oficial puede declarar una sola Relación `Conectado` junto a un
Lugar, cada Partida construye un grafo simétrico validado e indexado y `Viajar`
ofrece solamente destinos alcanzables mediante una ruta canónica mínima. El
recorrido vigente continúa pudiendo completarse desde la terminal y devuelve el
control al jugador en un estado válido.

## Referencias

- [Construcción, consultas y operaciones del Mundo](../../tecnica/componentes/02-modelo-mundo.md#6-construcción-consultas-y-operaciones).
- [Construcción, almacenamiento e índices de Relaciones](../../tecnica/componentes/10-relaciones.md#33-construcción-almacenamiento-e-índices).
- [Consultas de Relaciones desde el Mundo](../../tecnica/componentes/10-relaciones.md#34-consultas-de-mundo).
- [Formato RON y autoría de Relaciones](../../tecnica/componentes/10-relaciones.md#4-formato-ron-y-autoría).
- [Carga, normalización y validaciones de Relaciones](../../tecnica/componentes/10-relaciones.md#5-carga-normalización-y-validaciones).
- [Pruebas y criterios de aceptación de Relaciones](../../tecnica/componentes/10-relaciones.md#8-pruebas-y-criterios-de-aceptación).
- [Flujo de carga e instanciación de contenido](../../tecnica/componentes/03-contenido.md#4-flujo-de-carga-e-instanciación).
- [Formato y organización del contenido](../../tecnica/componentes/03-contenido.md#5-formato-y-organización).
- [Errores, pruebas y aceptación de contenido](../../tecnica/componentes/03-contenido.md#7-errores-pruebas-y-aceptación).
- [Disponibilidad y cálculo de ruta de Viajar](../../tecnica/acciones/01-viajar.md#3-disponibilidad-y-cálculo-de-ruta).
- [Solicitud y validación de Viajar](../../tecnica/acciones/01-viajar.md#4-solicitud-y-validación).
- [Lugares del escenario inicial](../../contenido/escenario-inicial.md#3-lugares).
- [Recorrido de contenido inicial](../../contenido/escenario-inicial.md#6-recorrido-de-contenido-inicial).
- [Ciclo incremental](../../metodologia-desarrollo.md#4-ciclo-incremental).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Alcance

### Incluido

- modelo, almacenamiento, índices y consultas de Relaciones definidos por
  `10-relaciones.md`;
- esquema, carga, resolución y diagnósticos de las declaraciones RON del mismo
  documento;
- disponibilidad, cálculo de ruta y validación definidos en las secciones
  enlazadas de `01-viajar.md`;
- integración de esos resultados con la fachada y la fase NPC;
- incorporación del Paso Escarpado, sus conexiones y su presentación mínima al
  contenido oficial;
- migración de los fixtures existentes y pruebas de núcleo, contenido,
  integración y humo;
- comprobación manual de que el binario sigue siendo jugable al terminar el
  plan.

### No incluido

- selección y ejecución de los sucesos, presentación y duración diseñados en
  las secciones 5 a 13 de `01-viajar.md`, que corresponden a PLAN-2026-011;
- todo alcance aplazado por
  [`10-relaciones.md`](../../tecnica/componentes/10-relaciones.md#10-decisiones-aplazadas)
  o por
  [`01-viajar.md`](../../tecnica/acciones/01-viajar.md#16-decisiones-aplazadas).

## Precondiciones

- Los documentos técnicos enlazados están aprobados para implementar y no
  contienen preguntas bloqueantes para este alcance.
- El binario base inicia desde `content/base`, permite terminar el recorrido de
  `Viajar` vigente y supera los controles automáticos antes de comenzar.
- El mapa dispone de coordenadas aptas para presentar el Paso Escarpado sin
  convertir esa presentación en conectividad reglada.

## Pasos

1. [x] Incorporar a `desert-tiger-core` los tipos de Relación, su construcción
       validada, el almacenamiento canónico y los índices definidos por el
       contrato técnico.
2. [x] Implementar las consultas de Relaciones y vecinos, cubriendo orden,
       simetría, inexistencia, duplicados e independencia entre instancias de
       Mundo.
3. [x] Ampliar los DTO y el cargador de contenido para exigir, resolver y
       validar `relaciones` en cada Lugar y Actor sin publicar conjuntos
       parciales.
4. [x] Implementar en `Viajar` la alcanzabilidad, la ruta canónica mínima y la
       validación al confirmar un destino, usando una misma regla de
       transitabilidad para jugador y NPC.
5. [x] Adaptar la fachada y la fase NPC para consumir los destinos ya
       determinados por el núcleo, sin exponer el grafo completo al frontend ni
       a Lua.
6. [x] Migrar los archivos y fixtures oficiales, añadir el Paso Escarpado y
       declarar una sola vez cada conexión necesaria para el recorrido entre la
       Zona Árida y el Valle Fértil.
7. [x] Añadir las pruebas unitarias, inválidas y de integración exigidas por los
       contratos de Relaciones, contenido y cálculo de ruta.
8. [x] Ejecutar el validador de contenido, los controles de la workspace y una
       compilación de producción.
9. [x] Jugar en la terminal un viaje al destino final a través del nuevo grafo,
       comprobar el regreso al turno del jugador y confirmar que no se dibuja ni
       se solicita elegir la ruta.
10. [x] Sincronizar el estado de la documentación, registrar resultados y mover
        el plan cuando se cumpla la definición de terminado.

## Verificación

- [x] Se cumplen las pruebas de
      [Relaciones](../../tecnica/componentes/10-relaciones.md#8-pruebas-y-criterios-de-aceptación)
      correspondientes al modelo, contenido e integración.
- [x] Se cumplen los casos de disponibilidad, ruta y rechazo de
      [la especificación de Viajar](../../tecnica/acciones/01-viajar.md#grafo-disponibilidad-y-ruta)
      incluidos en el alcance de este plan.
- [x] El contenido oficial declara cada conexión una sola vez, carga el Paso
      Escarpado y permite el viaje en ambos sentidos al jugador y al Cazador.
- [x] Los archivos válidos e inválidos satisfacen los criterios de
      [contenido de Relaciones](../../tecnica/componentes/10-relaciones.md#82-contenido).
- [x] La prueba de humo y la prueba manual terminan nuevamente en un turno
      jugable sin haber expuesto ni dibujado la ruta.
- [x] Pasan `cargo fmt --all -- --check`,
      `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace --all-targets` y el build de release.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Hacer obligatorio `relaciones` afecta a todos los archivos y fixtures de
  Actor y Lugar; la migración se realizará en el mismo cambio que el cargador
  para que no quede un estado intermedio presentado como válido.
- La disponibilidad y la confirmación podrían divergir si implementaran dos
  recorridos con reglas distintas. Las pruebas compartirán casos de
  transitabilidad y comprobarán ambos puntos de entrada.
- El estado entre ambos planes es una etapa operativa: este plan implementa solo
  las secciones enlazadas y conserva un recorrido jugable; PLAN-2026-011
  completa las secciones restantes sin crear reglas dentro de los planes.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-17.
- Resultado: Relaciones cargadas e indexadas, ruta BFS y contenido oficial de
  tres Lugares implementados; el recorrido vuelve a un turno válido.
- Verificaciones ejecutadas: formato, check, Clippy sin advertencias, 51 pruebas
  automáticas, build de release y recorridos manuales tranquilo y contextual
  hasta el Valle.
- Desviaciones respecto del plan: la implementación continuó inmediatamente con
  PLAN-2026-011 antes del cierre documental; no cambió ninguna regla aprobada.
- Trabajo posterior: [PLAN-2026-011](./2026-10-16-11-contextualizar-viajar.md).
