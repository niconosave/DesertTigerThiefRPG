# Plan: Preparar el contenido y la regla de Acampar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-036 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

El núcleo dispone de la regla de acampada y el contenido aporta nombres,
biografías y explicaciones de características validados. El juego existente
continúa funcionando; la activación de Acampar corresponde a PLAN-2026-037.

## Referencias

- [Módulos y responsabilidades](../../tecnica/acciones/07-acampar.md#2-módulos-y-responsabilidades).
- [Regla del núcleo](../../tecnica/acciones/07-acampar.md#3-regla-del-núcleo).
- [Nombre y biografía de Actor](../../tecnica/acciones/07-acampar.md#41-nombre-y-biografía-del-actor).
- [Catálogo de características](../../tecnica/acciones/07-acampar.md#42-catálogo-de-características).
- [Consultas eficientes de presentación](../../tecnica/acciones/07-acampar.md#43-consultas-eficientes).
- [Errores e invariantes](../../tecnica/acciones/07-acampar.md#9-errores-e-invariantes).
- [Verificación y aceptación](../../tecnica/acciones/07-acampar.md#10-verificación-y-aceptación).
- [Contratos afectados](../../tecnica/acciones/07-acampar.md#11-contratos-afectados-y-decisiones-cerradas).
- [Controles generales](../../metodologia-desarrollo.md#7-controles-automáticos).
- Continuación: [PLAN-2026-037](./2026-10-26-37-integrar-acampar-fichas-terminal.md).

## Alcance

### Incluido

- API core de inicio y finalización de acampada y sus errores.
- Acceso comprobado a presentación por ID.
- Nuevos metadatos de Actor, su proyección y su uso por el narrador.
- Migración de los seis Actores oficiales y catálogo editable de características.
- Validación, pruebas de contenido y regresión del juego existente.

### No incluido

- Habilitar Acampar en la sesión o en la terminal.
- Consulta de fichas, focos, fogata o nueva pantalla.
- Modificar retratos, mapa, relaciones, reglas de Viajar o comportamiento Lua.

## Precondiciones

- El contrato técnico de Acampar está aprobado para implementar.
- La base actual permite cargar el contenido y recorrer Viajar.
- Se identifican y conservan las ediciones del usuario en RON y recursos.

## Pasos

1. [x] Comprobar el estado inicial del workspace y la carga del contenido;
       registrar los fallos previos que afecten al alcance sin atribuirlos a
       Acampar ni modificar cambios ajenos.
2. [x] Implementar y reexportar la API core de acampada de la sección 3; añadir
       pruebas de inicio válido, rechazos, coste y ausencia de mutación.
3. [x] Ampliar la visibilidad de los índices tipados y sustituir las búsquedas
       lineales del catálogo según la sección 4.3, con comprobaciones de ID
       válido e inexistente.
4. [x] Incorporar nombre y biografía al DTO y a la presentación de Actor,
       aplicando la validación y compatibilidad de la sección 4.1. Añadir nombre
       a VistaActor y actualizar el narrador y los consumidores afectados.
5. [x] Migrar juntos los seis RON oficiales y los fixtures que cargan Actores;
       conservar arte, expresiones, paletas, políticas, características y
       relaciones. Las biografías se limitan al contenido ya establecido.
6. [x] Implementar la lectura, validación y almacenamiento del catálogo de la
       sección 4.2 y crear su archivo oficial con explicaciones acordes a las
       reglas actuales.
7. [x] Probar nombres y textos inválidos, biografía ausente y CRLF, catálogo
       ausente/vacío, duplicados, claves inválidas y diagnósticos de acceso.
       Verificar que una característica sin explicación sigue siendo válida.
8. [x] Ejecutar la carga de todo el contenido y Lua oficiales, las regresiones
       de Viajar, los controles generales y la compilación de producción.
9. [x] Actualizar los apartados implementados de los componentes afectados y
       registrar resultados parciales. Mantener el contrato completo de
       Acampar pendiente de implementación hasta cerrar PLAN-2026-037.

## Verificación

- [x] La API core satisface los casos de la sección 3 sin usar RNG ni alterar
      estado del Mundo o fecha.
- [x] Los seis Actores cargan con nombre y biografía; se conservan sus gráficos
      y datos reglados.
- [x] El catálogo respeta los casos de compatibilidad y error de la sección
      4.2; no restringe las características dinámicas del Mundo.
- [x] Los accesos de presentación por ID son comprobados y no recorren todos
      los Actores o Lugares.
- [x] Viajar y su narración siguen funcionando con los nombres visibles.
- [x] Pasan los cuatro controles de la metodología y
      `cargo build --workspace --release`; la carga oficial queda verificada.
- [x] La revisión visible de los nombres se registra en la prueba manual
      agrupada de PLAN-2026-037 antes de archivar este plan.
- [x] Se cumplen los controles y la definición de terminado de la
      [metodología](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Exigir nombre requiere migrar contenido y cargador en el mismo lote. No se
  entrega una versión que rechace los Actores oficiales.
- Los metadatos nuevos no justifican redibujar personajes ni ampliar su historia.
- Los dos planes forman un incremento; las verificaciones automáticas de este
  alcance permiten seguir con el siguiente sin esperar pruebas manuales
  individuales. El archivo permanece pendiente mientras falte su cierre.
- Si aparece una decisión técnica o de juego, se resuelve en el contrato
  canónico antes de ajustar los pasos; este plan no completa el diseño.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26.
- Resultado: Regla core de acampada, acceso comprobado por ID, nombres y biografías de los seis Actores y catálogo editable de características implementados. Los dibujos y datos reglados previos se conservaron.
- Verificaciones ejecutadas: pasan `cargo fmt --all -- --check`,
  `cargo check --workspace --all-targets`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
  Verificación final conjunta: 96 pruebas (40 core, 27 app, 25 terminal y 4
  importador), incluida la carga oficial de contenido y Lua y la regresión de
  Viajar. Los casos y el recorrido están registrados en la
  [sección 12 del contrato](../../tecnica/acciones/07-acampar.md#12-verificación-de-la-implementación--2026-10-26).
- Revisión manual: Revisión de nombres, biografías y explicaciones realizada junto al recorrido manual de PLAN-2026-037 en PTY 80×24.
- Desviaciones respecto del plan: ninguna funcional. Los dos planes se
  verificaron y cerraron como el incremento conjunto previsto, sin entregas
  intermedias ni pruebas manuales por cada detalle.
- Trabajo posterior: La integración se completó en [PLAN-2026-037](./2026-10-26-37-integrar-acampar-fichas-terminal.md); no quedan tareas de este alcance.
