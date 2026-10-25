# Plan: Construir el nuevo motor de Viajar y sus tarjetas

| Campo | Valor |
|---|---|
| ID | PLAN-2026-029 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El nuevo `Viajar` puede recorrerse directamente mediante su protocolo semántico,
con todas las tarjetas, decisiones y consecuencias verificadas. Su integración
con `Partida` se realiza en el plan siguiente: durante esta preparación, la
sesión conserva su única ruta de producción actual.

## Referencias

- [Organización del código](../../tecnica/acciones/06-implementacion-viajar.md#2-organización-del-código-y-dependencias).
- [Estado y tipos propios](../../tecnica/acciones/06-implementacion-viajar.md#3-estado-y-tipos-propios).
- [Acciones particulares y azar](../../tecnica/acciones/06-implementacion-viajar.md#8-contrato-de-acciones-particulares-y-azar).
- [Protocolo reanudable y estados](../../tecnica/acciones/06-implementacion-viajar.md#9-protocolo-reanudable-y-estados).
- [Separación dentro de un desplazamiento](../../tecnica/acciones/06-implementacion-viajar.md#91-separación-dentro-de-un-desplazamiento).
- [Orden entre conexiones](../../tecnica/acciones/06-implementacion-viajar.md#92-orden-entre-conexiones).
- [Consecuencias persistentes](../../tecnica/acciones/06-implementacion-viajar.md#10-consecuencias-persistentes-y-atomicidad).
- [Registros semánticos](../../tecnica/acciones/06-implementacion-viajar.md#11-registros-semánticos-redacción-y-privacidad).
- [Pruebas y aceptación](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Curso del viaje](../../tecnica/acciones/borradores/05-viajar-redisenio.md#curso)
  y [tarjetas del reglamento](../../tecnica/acciones/borradores/05-viajar-redisenio.md#tarjetas).
- Dependencia: [PLAN-2026-028](./2026-10-24-28-ampliar-consecuencias-grupo-npc.md).

## Alcance

### Incluido

- estado reanudable y protocolo por paso del nuevo viaje;
- tarjetas de ruta, preparación, desplazamiento, necesidades, carga y llegada;
- integración de las funciones probadas en PLAN-2026-025 a PLAN-2026-028;
- registros semánticos completos, decisiones vigentes y cierres reglados;
- pruebas directas del motor para jugador y NPC, sin frontend.

### No incluido

- sustitución del motor en `Partida`, redacción pública o cambios de Ratatui;
- una segunda versión seleccionable, opción de compatibilidad o dependencia nueva;
- unificar las tarjetas de caballo y camello; solo comparten la acción `Cabalgar`;
- comportamientos, Características o Relaciones ajenos al reglamento aprobado.

## Precondiciones

- PLAN-2026-028 verificado y funciones de tiempo, chequeos, rutas, reservas y
  carga de los planes anteriores disponibles para integración interna.
- Contrato técnico y reglamento aprobados, incluida la acción particular única
  `Cabalgar` y las tarjetas independientes `VIA-DES-CAB` y `VIA-DES-CAM`.
- La preparación temporal del motor respeta la sección 16 del contrato: no se
  ofrece un selector de versiones ni se deja compatibilidad permanente.

## Pasos

1. [x] Crear los tipos privados de ejecución, esperas, participantes y registros
       del nuevo viaje según las secciones 2, 3, 9 y 11 del contrato.
2. [x] Implementar inicio, avance, respuesta, IDs y finalización contra las
       validaciones de paso vigente; probar rechazos antes de añadir tarjetas.
3. [x] Conectar las tarjetas de planificación y preparación con los cálculos de
       rutas, provisiones y carga, incluida la preparación tentativa y su
       confirmación única tras una decisión.
4. [x] Implementar por separado las cuatro tarjetas de desplazamiento y sus
       tablas del reglamento, invocando la misma acción `Cabalgar` desde caballo
       y camello sin compartir sus resoluciones narrativas o incidentes.
5. [x] Integrar tiempos por mitad, selección de responsable, consumo y lotes de
       consecuencias con registros que preserven orden y participantes iniciales.
6. [x] Implementar las esperas de separación, la conservación de resultados y el
       cierre definido por la sección 9.1, sin repetir chequeos ni consumo.
7. [x] Integrar la secuencia entre conexiones: búsqueda, advertencia, comprobación
       de carga y descarte, respetando sus continuaciones y la guardia de búsqueda.
8. [x] Implementar llegada y terminaciones anticipadas con su resultado semántico
       y `TiempoAccion`, sin aplicar todavía el calendario mundial desde el motor.
9. [x] Recorrer todas las tarjetas con azar controlado; ampliar casos por tablas,
       límites e interacciones de la sección 15, no solo ejemplos favorables.
10. [x] Verificar ejecuciones extensas y decisiones automáticas de un NPC mediante
        el mismo protocolo interno; conservar la sesión productiva existente.
11. [x] Actualizar los apartados de motor y acciones particulares afectados por
        la implementación interna, indicando qué integración queda pendiente.
12. [x] Ejecutar controles generales y registrar el estado que recibirá
        PLAN-2026-030 para la sustitución integral en la aplicación.

## Verificación

- [x] Cada llamada produce el paso semántico establecido; IDs inválidos o
      repetidos no vuelven a gastar azar, tiempo, comida ni operaciones.
- [x] Caballo y camello comparten chequeo y aptitud `Cabalgar`, pero conservan
      identidades de tarjeta, incidentes y tiempos verificables por separado.
- [x] La matriz de desplazamientos cubre las filas del reglamento, incidentes en
      ambas mitades, empates, estados previos, pérdida de montura y navegación.
- [x] La cancelación por separación conserva ambos resultados y el tiempo y
      consumo de toda la conexión, con la ubicación de cierre correcta.
- [x] Preparación, suplemento, búsqueda y descarte confirman cada cambio una
      sola vez; la llegada prevalece sobre otra búsqueda y no hay límite de escenas.
- [x] Los registros preservan participantes de entrada, composición resultante,
      consecuencias confirmadas y los datos de visibilidad sin analizar prosa.
- [x] Ningún recorrido directo del motor confirma fecha ni resto mundial; las
      finalizaciones conservan su tiempo exacto y causa reglada.
- [x] El juego existente sigue pasando su recorrido de humo hasta el relevo
      de PLAN-2026-030; no se publica una opción para elegir entre motores.
- [x] `cargo fmt --all -- --check`
- [x] `cargo check --workspace --all-targets`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace --all-targets`
- [x] `cargo build --workspace --release`
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- La preparación de un motor nuevo sin activarlo es temporal y pertenece a la
  migración del contrato. PLAN-2026-030 sustituye la ruta productiva y retira el
  motor anterior; no se consolida una arquitectura con dos versiones estables.
- Las dos tarjetas montadas pueden evolucionar independientemente. Se prueba
  por separado cada una para evitar que compartir el chequeo fusione sus escenas.
- Si una prueba revela una regla faltante, se corrige primero su fuente técnica
  o el reglamento; no se completa el diseño desde el propio plan.
- Las verificaciones manuales de percepción y dramatismo se agrupan en
  PLAN-2026-031; este plan verifica la ejecución semántica sin ventanas.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Motor reanudable y tarjetas del reglamento implementados, con separación, incidentes y cierres válidos.
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
- Trabajo posterior: PLAN-2026-030. No quedan tareas de implementación de este plan.
