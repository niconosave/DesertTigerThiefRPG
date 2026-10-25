# Plan: Convertir Viajar en una acción contextual

| Campo | Valor |
|---|---|
| ID | PLAN-2026-011 |
| Estado | Completado |
| Creado | 2026-10-16 |
| Actualizado | 2026-10-17 |

## Resultado buscado

`Viajar` utiliza la ruta resuelta para seleccionar como máximo una complicación,
combina las Características de los Lugares atravesados y los viajeros, calcula
una duración coherente y garantiza la llegada. El jugador puede recorrer desde
la Zona Árida hasta el Valle Fértil, decidir cómo afrontar el Paso Escarpado con
su compañero y volver a un turno jugable; los NPC emplean el mismo motor.

## Referencias

- [Integración de Relaciones con Viajar](../../tecnica/componentes/10-relaciones.md#6-integración-con-viajar).
- [Conceptos propios de Viajar](../../tecnica/acciones/01-viajar.md#2-conceptos-propios-de-la-acción).
- [Candidaturas y selección del suceso](../../tecnica/acciones/01-viajar.md#5-candidaturas-y-selección-del-suceso).
- [Ejecución común](../../tecnica/acciones/01-viajar.md#6-ejecución-común).
- [Suceso Trayecto tranquilo](../../tecnica/acciones/01-viajar.md#7-suceso-trayecto-tranquilo).
- [Suceso contextual Paso empinado](../../tecnica/acciones/01-viajar.md#8-suceso-contextual-paso-empinado).
- [Llegada y presentación](../../tecnica/acciones/01-viajar.md#9-llegada-y-presentación).
- [Duración de Viajar](../../tecnica/acciones/01-viajar.md#10-duración).
- [Consecuencia reglada y atomicidad](../../tecnica/acciones/01-viajar.md#11-consecuencia-reglada-y-atomicidad).
- [Contratos e invariantes de Viajar](../../tecnica/acciones/01-viajar.md#13-contratos-e-invariantes).
- [Pruebas y criterios de aceptación de Viajar](../../tecnica/acciones/01-viajar.md#14-pruebas-y-criterios-de-aceptación).
- [Ejecución y estado reanudable del motor](../../tecnica/componentes/05-motor-acciones.md#5-ejecución-y-estado-reanudable).
- [Azar compartido y reproducible](../../tecnica/componentes/05-motor-acciones.md#6-azar-compartido-y-reproducible).
- [Fases observables de la sesión](../../tecnica/componentes/04-sesion-aplicacion.md#5-fases-observables).
- [Recorrido de contenido inicial](../../contenido/escenario-inicial.md#6-recorrido-de-contenido-inicial).
- [Ciclo incremental](../../metodologia-desarrollo.md#4-ciclo-incremental).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).
- Plan previo: [PLAN-2026-010](./2026-10-16-10-incorporar-relaciones-y-rutas.md).

## Alcance

### Incluido

- ejecución de los sucesos, decisiones, duración, llegada y consecuencias
  definidos en las secciones 5 a 13 de `01-viajar.md`;
- adaptación de la máquina reanudable y del azar compartido para cumplir esos
  contratos;
- integración del mismo recorrido con la sesión y la fase NPC;
- adaptación de vistas y recursos de presentación solo en la medida exigida por
  las emisiones semánticas aprobadas;
- actualización del contenido oficial, pruebas automáticas y recorrido manual
  completo.

### No incluido

- las decisiones aplazadas de
  [`Viajar`](../../tecnica/acciones/01-viajar.md#16-decisiones-aplazadas)
  y de
  [Relaciones](../../tecnica/componentes/10-relaciones.md#10-decisiones-aplazadas);
- otras acciones generales.

## Precondiciones

- PLAN-2026-010 está terminado: el contenido carga Relaciones válidas, la ruta
  canónica se resuelve y el recorrido base continúa siendo jugable.
- Los contratos enlazados están aprobados y los literales, opciones, consumos de
  azar y duraciones no presentan preguntas bloqueantes.
- El contenido oficial contiene el Paso Escarpado en la ruta entre los Lugares
  extremos y al Bandido como compañero `fuerte` del jugador.

## Pasos

1. [x] Adaptar el estado privado de `EjecucionAccion` para conservar la ruta,
       los viajeros y el contexto seleccionable sin exponerlos a aplicación o
       frontend.
2. [x] Implementar la construcción ordenada de candidaturas y la selección de
       exactamente un suceso mediante la fuente de azar inyectada.
3. [x] Implementar la duración base por conexión y el suceso Trayecto tranquilo
       con sus emisiones aprobadas.
4. [x] Implementar Paso empinado, su decisión, la consulta de `fuerte`, sus tres
       resultados y sus aportes comprobados a la duración.
5. [x] Integrar la llegada única, el traslado atómico final y la clasificación
       de rechazos o fallos sin introducir movimientos intermedios.
6. [x] Adaptar la fachada, la fase NPC y los recursos visuales necesarios para
       presentar las nuevas emisiones y resolver al azar una decisión NPC con el
       mismo motor.
7. [x] Marcar el Paso Escarpado oficial como `empinado` y comprobar que el
       Bandido `fuerte` participa como compañero sin crear otra categoría de
       datos.
8. [x] Añadir las pruebas unitarias y de integración de candidaturas, azar,
       decisiones, duraciones, presentación, traslado, tiempo y regresiones del
       recorrido existente.
9. [x] Ejecutar el validador de contenido, los controles de la workspace, la
       prueba de humo extremo a extremo y una compilación de producción.
10. [x] Jugar un recorrido completo que presente Paso empinado y confirmar la
        continuidad hasta el destino; comprobar automáticamente ambas opciones.
11. [x] Sincronizar documentación y recursos con el resultado, registrar toda
        desviación y mover el plan cuando se cumpla la definición de terminado.

## Verificación

- [x] Se cumplen todos los casos de
      [pruebas de Viajar](../../tecnica/acciones/01-viajar.md#14-pruebas-y-criterios-de-aceptación)
      aplicables al núcleo, la sesión, NPC y presentación.
- [x] La prueba reproducible y los casos de integración satisfacen las
      invariantes de
      [Viajar](../../tecnica/acciones/01-viajar.md#13-contratos-e-invariantes)
      sin excepciones propias del frontend o de los NPC.
- [x] Las pruebas de sesión confirman la aplicación de la duración y la fase NPC
      conforme a los contratos enlazados, sin una regla duplicada en app.
- [x] El frontend centra el destino, no dibuja ni solicita la ruta, presenta la
      secuencia contextual y vuelve al mapa en un estado jugable.
- [x] Pasan `cargo fmt --all -- --check`,
      `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace --all-targets` y el build de release.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Cambiar la máquina de estados puede alterar el consumo de azar y romper
  semillas de prueba. Se comprobarán límites y orden de cada extracción en vez
  de depender de resultados casuales.
- Las claves de escena existentes pueden conservar recursos que esta versión ya
  no emite. No se eliminarán por conveniencia durante el cambio si eso dificulta
  compatibilidad o navegación; su limpieza requiere una decisión separada.
- El frontend solo presenta el protocolo neutral. Cualquier necesidad de
  inspeccionar Características o reconstruir la ruta desde terminal indica una
  violación de capas y debe resolverse en el documento técnico, no en el plan.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-17.
- Resultado: `Viajar` calcula duración por conexión, selecciona un único suceso,
  resuelve Paso empinado con el Grupo y garantiza una llegada atómica.
- Verificaciones ejecutadas: formato, check, Clippy sin advertencias, 51 pruebas
  automáticas, build de release y recorrido manual del evento contextual, la
  decisión de ascender y la llegada al destino.
- Desviaciones respecto del plan: la revisión manual usó una opción
  representativa; las pruebas reproducibles cubren ambas opciones para evitar
  detener el desarrollo por cada detalle menor.
- Trabajo posterior: ampliar `Viajar` o incorporar otra acción solamente después
  de evaluar este recorrido jugable.
