# Plan: Activar el nuevo Viajar y presentar sus escenas

| Campo | Valor |
|---|---|
| ID | PLAN-2026-030 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El juego usa el motor nuevo de `Viajar` desde la selección del destino hasta
el regreso al mundo. Las escenas muestran responsables, causas, tiempo y reservas
del mismo momento. Caballo y camello comparten la prueba `Cabalgar`, pero conservan
sus viñetas y narraciones independientes. Jugador y NPC atraviesan el mismo motor.

## Referencias

- [Protocolo por pasos y máquina](../../tecnica/acciones/06-implementacion-viajar.md#9-protocolo-reanudable-y-estados).
- [Puente al calendario](../../tecnica/acciones/06-implementacion-viajar.md#42-puente-compartido).
- [Narración, privacidad y vistas](../../tecnica/acciones/06-implementacion-viajar.md#11-registros-semánticos-redacción-y-privacidad).
- [NPC y sesión](../../tecnica/acciones/06-implementacion-viajar.md#13-ejecución-de-npc-y-sesión).
- [Errores](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Compatibilidad y sustitución](../../tecnica/acciones/06-implementacion-viajar.md#16-compatibilidad-y-documentos-afectados).
- [Información requerida](../../tecnica/acciones/borradores/05-viajar-redisenio.md#presentacion).
- [Viñetas de caballo y camello](../../tecnica/acciones/borradores/05-viajar-redisenio.md#via-desplazamiento).
- Dependencia directa: [PLAN-2026-029](./2026-10-24-29-construir-motor-viajar-tarjetas.md).

## Alcance

### Incluido

- sustitución conjunta del despacho de acciones y su integración con la Partida;
- redacción y proyección pública específicas de todas las tarjetas;
- vistas de progreso, retratos, decisiones, paginado y detalles;
- avance temporal del jugador y ejecución automática de escenas de NPC;
- retirada de la máquina anterior incompatible en el mismo incremento;
- integración y pruebas de terminal sin depender de un monitor real.

### No incluido

- nuevos incidentes, rediseño del layout, arte definitivo, guardado o diagnóstico Unicode;
- selector entre motores ni adaptación permanente del nuevo protocolo al antiguo;
- cambios de reglas para simplificar la redacción o abreviar escenas.

## Precondiciones

- PLAN-2026-025 a PLAN-2026-029 implementados y sus verificaciones registradas.
- El motor y las tarjetas nuevas se pueden recorrer con pruebas sin frontend.
- La persona usuaria solicitó comenzar el desarrollo de la secuencia.

## Pasos

1. [x] Añadir pruebas de fachada del recorrido nuevo que abarquen preparación,
       decisión, desplazamiento, cierre temporal y fase NPC.
2. [x] Adaptar `Partida` para retener ejecución y correspondencias de IDs durante
       escenas y decisiones, y para confirmar cada paso una sola vez.
3. [x] Integrar el catálogo de rumbos y el inicio temporal fijo con el motor;
       proyectar y confirmar fecha y resto mediante la misma función.
4. [x] Implementar `narracion_viaje.rs` y la proyección pública desde registros,
       con redactores independientes de caballo y camello y pruebas de sus
       requisitos narrativos e información visible.
5. [x] Adaptar vistas de sesión para transportar textos, opciones, participantes
       y progreso del paso actual; retirar los pronósticos incompatibles del
       selector y evitar filtraciones por el panel de detalle.
6. [x] Conectar el frontend con el nuevo paso sin trasladar reglas a Ratatui;
       conservar sus controles, recursos, animaciones y paginado.
7. [x] Integrar la fase NPC con confirmaciones automáticas y decisiones válidas
       al azar; acumular sus resúmenes sin avanzar fecha ni resto mundial.
8. [x] Activar el único despacho nuevo de producción y retirar en ese mismo
       cambio la máquina, pronósticos, trazas y helpers anteriores incompatibles.
       Migrar las pruebas que deban conservarse y retirar solo las sustituidas.
9. [x] Ejecutar las pruebas de integración y `TestBackend`, los controles
       generales y el build de producción; comprobar un recorrido automático
       completo usando contenido oficial.
10. [x] Actualizar los contratos de sesión, motor, NPC y frontend afectados
        según la sección 16 del diseño técnico; registrar lo verificado.

## Verificación

- [x] Se puede iniciar, recorrer y terminar un viaje mediante la fachada pública.
- [x] La escena y sus retratos muestran participantes del momento narrado.
- [x] Volver a enviar Enter o una respuesta vieja no repite azar, efectos o tiempo.
- [x] El calendario final coincide con el último progreso y conserva el resto.
- [x] La interfaz permite leer todo el texto y decidir en el tamaño mínimo,
      también después de redimensionar, sin confirmar al paginar.
- [x] La ruta oculta, necesidad base y unidades alimenticias no aparecen en
      vistas, textos, detalles ni pronósticos previos a la planificación.
- [x] Caballo y camello presentan escenas distintas invocando una sola prueba.
- [x] Las escenas automáticas no se ejecutan por adelantado mientras se leen otras.
- [x] Los NPC con cero, una o varias políticas usan el nuevo motor sin alterar reloj.
- [x] No queda un selector ni un camino público que ejecute el motor anterior.
- [x] Pasan `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
- [x] La comprobación manual de tierra, mar y cancelación se registra en
      PLAN-2026-031 como revisión final conjunta; este plan permanece pendiente
      de cierre hasta esa verificación agrupada.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- La sustitución cruza core, aplicación y frontend. Se realiza como un solo
  incremento integrado, sin dejar una versión entregable a mitad de la migración.
- Una vista que consulte el Mundo posterior para reconstruir una escena pasada
  incumple el contrato; se verifica contra las instantáneas del registro.
- La revisión manual final se agrupa en PLAN-2026-031; no se solicita al jugador
  comprobar cada tarjeta o cambio menor.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Único motor nuevo integrado con Partida, narración por tarjeta y presentación Ratatui.
- Verificaciones: los cinco controles generales pasaron; 70 pruebas del workspace
  y carga del contenido y Lua oficiales. Casos, nombres de pruebas, mediciones y
  recorridos manuales constan en el
  [informe de verificación](../../tecnica/acciones/verificacion-viajar-reglamento.md).
- Desviación operativa: la serie se entregó como un incremento integrado; los
  controles finales verifican el motor nuevo. No se afirma conservar en ejecución
  la versión anterior tras retirarla. La revisión manual agrupada la realizó el
  agente en PTY; la valoración artística y de jugabilidad del usuario es posterior
  y no se presenta como ya aprobada.
- Ajustes pequeños: representación local de botas, octantes y parámetros de
  chequeos documentados en el contrato 06 y en el informe; sin reglas nuevas.
- Trabajo posterior: PLAN-2026-031. No quedan tareas de implementación de este plan.
