# Plan: Preparar configuraciones de carga y provisiones de Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-027 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El núcleo calcula las configuraciones de preparación, los suplementos, la demora
pendiente y los lotes de descarte que usarán las tarjetas del refactor. Todos
los resultados pueden verificarse con cantidades y azar controlados, sin activar
todavía el viaje nuevo en la sesión.

## Referencias

- [Reservas y materialización](../../tecnica/acciones/06-implementacion-viajar.md#5-reservas-materialización-y-consumo).
- [Duraciones y medios por mitad](../../tecnica/acciones/06-implementacion-viajar.md#6-rutas-medios-y-rumbo-provisional).
- [Preparación y estabilización de carga](../../tecnica/acciones/06-implementacion-viajar.md#7-preparación-y-estabilización-de-carga).
- [Errores](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Pruebas y aceptación](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Configuración inicial del reglamento](../../tecnica/acciones/borradores/05-viajar-redisenio.md#configuracion-inicial).
- [Comprobaciones durante el viaje](../../tecnica/acciones/borradores/05-viajar-redisenio.md#recalculo-carga).
- [Dependencia: PLAN-2026-026](./2026-10-24-26-preparar-rutas-rumbos.md).
- [Metodología y controles](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- lotes tentativos compartidos y configuraciones de preparación;
- estabilización de velocidad, provisiones y animales;
- evaluación del suplemento y del plan de demora;
- comprobación de capacidad y algoritmo de descarte mínimo;
- pruebas de exactitud, terminación y ausencia de cambios tentativos en Mundo.

### No incluido

- ejecutar o redactar las tarjetas APR-01, APR-05 o APR-06 en la sesión;
- activar un motor alternativo o modificar la interfaz;
- economía, nuevos recursos, monturas persistentes o reglas adicionales de peso;
- retirar provisiones al Grupo fuera de las decisiones ya diseñadas.

## Precondiciones

- PLAN-2026-025 y PLAN-2026-026 implementados y verificados.
- Reglamento y contrato técnico de carga aprobados.
- Disponibles las cantidades exactas, materialización y previsiones puras que
  establecen los planes anteriores.

## Pasos

1. [x] Implementar el lote tentativo y la selección por prefijo compartido para
       evaluar configuraciones sin resorteos ni incorporaciones definitivas.
2. [x] Implementar la estabilización principal y la evaluación de la alternativa
       permitida, utilizando las previsiones de velocidad del plan anterior.
3. [x] Implementar el ciclo de suplemento, su sustitución tentativa y sus
       resultados de aceptación o rechazo conforme a la sección 7.
4. [x] Construir el plan de demora por mitad pendiente y las consultas puras de
       capacidad y recursos que consumirán las tarjetas.
5. [x] Implementar el algoritmo de lote mínimo con sus objetivos y desempates
       ya fijados; devolver los objetos concretos sin retirarlos todavía.
6. [x] Contrastar la selección de descarte con enumeración exhaustiva de casos
       pequeños y medir su uso con cantidades del contenido oficial.
7. [x] Probar casos sin animales, alternativas admisibles y rechazadas,
       igualdad en fronteras, consumo sin reducción automática del plan,
       sustitución de demora y desbordamientos.
8. [x] Verificar que consultar configuraciones o descartar una alternativa no
       cambie el Mundo, la ruta adoptada ni las reservas confirmadas.
9. [x] Ejecutar controles y regresiones existentes; registrar los resultados y
       la integración de tarjetas que queda para los planes posteriores.

## Verificación

- [x] Pasan los casos de preparación y carga de la sección 15 del contrato.
- [x] Las alternativas preservan el chequeo y lote compartidos indicados por
      el diseño; no consumen de nuevo recursos definitivos al reevaluarse.
- [x] El descarte satisface la capacidad y los objetivos exactos documentados.
- [x] Las comprobaciones no alteran tiempos ya transcurridos.
- [x] La prueba de humo y la versión jugable integrada funcionan; véase la desviación de integración al final.
- [x] Se ejecutaron satisfactoriamente:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

- [x] Las pruebas de carga del contenido oficial siguen pasando.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Los cálculos pueden revelar defectos de una interpretación; se corrige primero
  su fuente técnica, sin añadir reglas dentro del plan o del código.
- La medición del descarte verifica su viabilidad con el atlas; no habilita una
  aproximación silenciosa a los mínimos exigidos.
- Estas funciones siguen siendo preparación del reemplazo hasta su integración
  y activación. No requieren detenerse para probar manualmente cada variante.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Preparación estable T0/T1, alternativa N−1, suplementos, demora pendiente y descarte mínimo implementados.
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
- Trabajo posterior: PLAN-2026-028. No quedan tareas de implementación de este plan.
