# Plan: Establecer la resolución de acciones particulares

| Campo | Valor |
|---|---|
| ID | PLAN-2026-013 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

El núcleo dispone del contrato reusable de acciones particulares, operaciones
atómicas del Mundo y una primera integración vertical de `Escalar`, sin perder
el recorrido jugable vigente de `Viajar`.

## Referencias

- [Contrato de resolución](../../tecnica/acciones/particulares/00-contrato-resolucion.md#3-ciclo-normativo).
- [Ficha de Escalar](../../tecnica/acciones/particulares/01-movimiento-terrestre.md#5-escalar).
- [Operaciones validadas](../../tecnica/componentes/11-operaciones-mundo.md#3-vocabulario-y-api-objetivo).
- [Motor implementado](../../tecnica/componentes/05-motor-acciones.md#5-ejecución-y-estado-reanudable).

## Alcance

### Incluido

- módulos y tipos comunes de evaluación, resolución, pronóstico y d20;
- lotes atómicos de traslado y Características de Actor;
- `Escalar` como primera ficha ejecutable;
- sustitución de la regla privada de pendiente por la acción particular;
- pruebas reproducibles y adaptación de resultados semánticos.

### No incluido

- Lugares nuevos, expediciones por etapas, provisiones o navegación;
- registro dinámico o acciones definidas en RON o Lua.

## Precondiciones

- PLAN-2026-012 completado.
- Documentos técnicos citados aprobados.

## Pasos

1. [x] Separar `acciones` en los módulos previstos sin cambiar la fachada pública vigente.
2. [x] Implementar las evaluaciones imposible, determinista e incierta, las bandas comunes y el uso controlado de `FuenteAzar`.
3. [x] Implementar `OperacionMundo`, validación en estado virtual, aplicación atómica y cambios confirmados.
4. [x] Implementar solicitud, evaluación, pronóstico y resultado concretos de `Escalar`.
5. [x] Reemplazar la resolución ad hoc de la pendiente de `Viajar` por esa ficha y traducir su salida al protocolo vigente.
6. [x] Añadir pruebas unitarias, de atomicidad y de integración desde jugador y NPC.
7. [x] Actualizar los documentos implementados si los nombres finales difieren sin cambiar contratos.
8. [x] Ejecutar controles, build de producción y una única prueba manual del recorrido vigente.

## Verificación

- [x] Una evaluación imposible o automática no consume azar y una incierta consume exactamente un d20.
- [x] Un lote cuyo último cambio falla no modifica ninguna parte del Mundo.
- [x] La pendiente usa `Escalar`; no queda una segunda fórmula activa.
- [x] El juego inicia, viaja, completa la fase NPC y vuelve a la vista del mundo.
- [x] Pasan el validador de contenido, `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
- [x] Se cumplieron los controles y la definición de terminado de `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Se preservará el orden observable del azar del recorrido vigente salvo donde la ficha aprobada exija el d20; las pruebas fijarán cualquier cambio intencional.
- La reorganización no introducirá traits dinámicos ni API pública prematura.

## Resultado de la implementación

- Fecha de finalización: 2026-10-18.
- Resultado: infraestructura común de resolución y d20, lotes atómicos del Mundo y `Escalar` integrados en la pendiente de `Viajar`. Un fallo puede herir e interrumpir el viaje; un éxito costoso puede fatigar y trasladar en el mismo lote.
- Verificaciones ejecutadas: 64 pruebas automáticas, carga del contenido oficial mediante sus pruebas, `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, `cargo build --workspace --release` y `git diff --check`, todas satisfactorias el 2026-10-18.
- Desviaciones respecto del plan: la fachada general continúa en `acciones.rs`; solo la infraestructura particular se separó en módulos, para no adelantar la migración completa prevista en PLAN-2026-014. La revisión manual confirmó en terminal el evento de ascenso empinado y su decisión.
- Trabajo posterior: [PLAN-2026-014](./2026-10-17-14-recorrer-expediciones-por-lugares.md).
