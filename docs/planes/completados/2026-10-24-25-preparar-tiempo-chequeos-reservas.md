# Plan: Preparar tiempo exacto, chequeos y reservas de Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-025 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El núcleo dispone de los cálculos básicos del refactor, comprobables sin
frontend: tiempo exacto, chequeos particulares y reservas concretas con consumo.
La versión actual continúa jugable mientras se prepara la sustitución del motor.

## Referencias

- [Organización y estado del refactor](../../tecnica/acciones/06-implementacion-viajar.md#2-organización-del-código-y-dependencias).
- [Tiempo exacto y puente al calendario](../../tecnica/acciones/06-implementacion-viajar.md#4-tiempo-exacto-y-traducción-al-calendario).
- [Reservas, materialización y consumo](../../tecnica/acciones/06-implementacion-viajar.md#5-reservas-materialización-y-consumo).
- [Contrato de acciones particulares y azar](../../tecnica/acciones/06-implementacion-viajar.md#8-contrato-de-acciones-particulares-y-azar).
- [Errores](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Pruebas y aceptación](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Reglas de chequeos](../../tecnica/acciones/borradores/05-viajar-redisenio.md#chequeos).
- [Metodología y controles](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- tipos y operaciones exactas de tiempo, su conversión y proyección;
- chequeos, evaluación de aptitud y registro de modificadores;
- comidas, botas, materialización, peso y consumo de escena;
- pruebas reproducibles de límites y errores de estos módulos.

### No incluido

- activar el nuevo motor en la sesión o cambiar la interfaz;
- rutas, configuraciones de carga, tarjetas o narración;
- nuevas Características, reglas de consumo o tipos de Relación;
- una versión alternativa seleccionable del juego.

## Precondiciones

- Reglamento y contrato técnico aprobados para el alcance del refactor.
- Base anterior conservada en Git y controles actuales disponibles.
- La unificación de `Cabalgar` está incorporada a las fuentes; las tarjetas de
  caballo y camello siguen independientes.

## Pasos

1. [x] Añadir pruebas de las operaciones temporales y sus conversiones según
       los casos de la sección 15 del contrato.
2. [x] Implementar `HorasViaje`, sus operaciones comprobadas y el puente
       `FraccionDia`–`TiempoAccion`–`InstanteAccion` sin sustituir aún la
       finalización pública de las acciones existentes.
3. [x] Implementar los chequeos particulares, la consulta pura de capacidad,
       los modificadores y la validación de la fuente de azar.
4. [x] Verificar que ambas futuras tarjetas de montura puedan invocar la misma
       acción `Cabalgar`, sin introducir aptitudes independientes por especie.
5. [x] Implementar cantidades de comida, identificación y volumen de botas,
       peso y materialización según el contrato.
6. [x] Implementar la única función de consumo y sus registros usando la
       composición congelada de la escena.
7. [x] Cubrir fronteras de consumo, desperdicios, saldos agotados, carencias,
       fracciones, entradas inválidas y desbordamientos con azar controlado.
8. [x] Ejecutar los controles y las pruebas existentes del recorrido jugable.
9. [x] Registrar qué contratos quedaron implementados, sus verificaciones y el
       trabajo que permanece reservado para la activación del nuevo motor.

## Verificación

- [x] Tiempo, consumo y chequeos pasan los casos aplicables de la sección 15.
- [x] Consultar capacidad o proyectar fechas no consume azar ni modifica Mundo.
- [x] La comida y el agua se registran sin negativos ni pérdida de fracciones
      temporales; los fallos comprobados no se convierten en valores válidos.
- [x] La ejecución pública integrada y su prueba de humo funcionan; véase la desviación de integración al final.
- [x] Se ejecutaron satisfactoriamente:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

- [x] La carga del contenido oficial sigue cubierta por sus pruebas existentes.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Estos módulos preparan el reemplazo; su existencia no cambia todavía las
  mecánicas disponibles al jugar. La activación corresponde a PLAN-2026-030.
- No se requiere una revisión visual nueva en este plan porque no modifica
  vistas; se conserva la comprobación automática del recorrido anterior.
- Si aparece una decisión nueva, se resuelve primero en el contrato técnico y
  después se actualiza este plan; no se completa el reglamento desde código.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Tiempo exacto, puente de fracción de día, chequeos particulares y reservas con consumo implementados.
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
- Trabajo posterior: PLAN-2026-026. No quedan tareas de implementación de este plan.
