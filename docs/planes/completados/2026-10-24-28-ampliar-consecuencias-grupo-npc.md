# Plan: Preparar consecuencias del viaje y varios NPC independientes

| Campo | Valor |
|---|---|
| ID | PLAN-2026-028 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El Mundo permite separar compañeros y adquirir conocimiento de Lugares mediante
operaciones atómicas verificadas. La fase NPC acepta cero, uno o varios Actores
independientes sin alterar la fecha mundial. El juego existente continúa
funcionando mientras se prepara la sustitución del motor.

## Referencias

- [Estado del viaje y composición de participantes](../../tecnica/acciones/06-implementacion-viajar.md#3-estado-y-tipos-propios).
- [Consecuencias persistentes y atomicidad](../../tecnica/acciones/06-implementacion-viajar.md#10-consecuencias-persistentes-y-atomicidad).
- [Ejecución de NPC y sesión](../../tecnica/acciones/06-implementacion-viajar.md#13-ejecución-de-npc-y-sesión).
- [Errores y límites](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Pruebas y aceptación](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Compatibilidad y documentos afectados](../../tecnica/acciones/06-implementacion-viajar.md#16-compatibilidad-y-documentos-afectados).
- Dependencia: [PLAN-2026-027](./2026-10-24-27-preparar-carga-provisiones.md).

## Alcance

### Incluido

- operaciones de separación y registro de la Relación existente `Conoce`;
- validación y confirmación conjunta de Actores, Grupo, Relaciones e índices;
- resultados semánticos necesarios para reconocer las operaciones confirmadas;
- recorrido estable de las políticas NPC y acumulación de sus resultados;
- pruebas de invariantes, atomicidad y fase NPC sobre el protocolo actualmente
  conectado a la aplicación.

### No incluido

- el tipo de Relación `Abandono` u otros tipos nuevos;
- políticas basadas en Relaciones, cambios de la API Lua o IA nueva;
- economía, inventario, monturas persistentes o búsqueda de compañeros;
- activación de las tarjetas del nuevo viaje en la sesión o su narración.

## Precondiciones

- PLAN-2026-027 tiene su alcance acotado verificado.
- El contrato técnico del refactor está aprobado para implementar.
- La fase de NPC y las operaciones existentes siguen pasando sus pruebas antes
  de comenzar los cambios.

## Pasos

1. [x] Añadir a `OperacionMundo` las operaciones definidas en la sección 10 del
       contrato y sus resultados confirmados, conservando los IDs existentes.
2. [x] Ampliar el estado candidato de `aplicar_lote` para incluir Grupo,
       Relaciones e índices afectados; publicar el conjunto únicamente tras
       validar todas las operaciones y cálculos del lote.
3. [x] Implementar la separación validada de un compañero y su ubicación de
       cierre, manteniendo las invariantes de pertenencia y traslado del Grupo.
4. [x] Implementar la adquisición idempotente de `Conoce` y actualizar todos sus
       índices en la misma confirmación, sin un almacén paralelo de conocimiento.
5. [x] Cubrir con pruebas la combinación de conocimiento, estados, separación y
       traslado, incluida una operación posterior inválida que impida confirmar
       cualquier parte del lote.
6. [x] Sustituir en `fase_npc` la suposición de un único elegible por el recorrido
       y la revalidación establecidos en la sección 13; conservar todavía el
       protocolo del motor conectado a la sesión.
7. [x] Acumular resúmenes de todos los NPC y comprobar el tratamiento de políticas
       ausentes, Actores excluidos, rechazos y errores Lua recuperables.
8. [x] Verificar el fixture oficial y un fixture de compañero separado que quede
       disponible para la fase NPC, sin adelantar su separación al contenido base.
9. [x] Actualizar los documentos de Mundo, operaciones, conocimiento y fase NPC
       afectados por el alcance ya implementado, enlazando el contrato objetivo.
10. [x] Ejecutar controles generales y registrar resultados, desviaciones y la
        integración posterior con el nuevo protocolo en PLAN-2026-030.

## Verificación

- [x] La separación del jugador o de un Actor no perteneciente al Grupo se
      rechaza sin publicar cambios parciales.
- [x] Registrar dos veces el mismo conocimiento conserva una sola Relación y
      todas las consultas por índices concuerdan con el almacén de Relaciones.
- [x] Un lote fallido conserva Actores, Grupo, Relaciones e índices anteriores.
- [x] Los traslados de compañeros separados y de integrantes restantes respetan
      los casos de separación y costas diferentes de la sección 10.
- [x] La fase NPC funciona con cero, uno y varios elegibles; revalida exclusiones
      antes de cada evaluación y conserva los resúmenes anteriores ante un error
      recuperable posterior.
- [x] Las acciones NPC no avanzan la fecha; los componentes temporales preparados
      tampoco se confirman desde esa fase.
- [x] La carga del contenido oficial y el recorrido de humo existentes siguen
      pasando con el motor nuevo como única ruta de producción; véase la desviación registrada.
- [x] `cargo fmt --all -- --check`
- [x] `cargo check --workspace --all-targets`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace --all-targets`
- [x] `cargo build --workspace --release`
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- La nueva atomicidad afecta índices además de Actores; los casos de fallo
  deben comparar el estado completo, conforme a la sección 10 del contrato.
- Este plan prepara operaciones reutilizables sin conectar todavía los eventos
  nuevos. La sustitución del protocolo de la fase NPC se realiza con la sesión
  en PLAN-2026-030, evitando una integración parcial no jugable.
- No hace falta interrumpir el lote para una prueba manual de cada operación;
  la aceptación visible se agrupa en PLAN-2026-031.

Esta sección no crea reglas. Una decisión nueva descubierta durante la ejecución
debe registrarse primero en su documento canónico.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Separación de compañeros y Conoce atómicos; fase de cero o más NPC sin avance mundial implementada.
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
- Trabajo posterior: PLAN-2026-029. No quedan tareas de implementación de este plan.
