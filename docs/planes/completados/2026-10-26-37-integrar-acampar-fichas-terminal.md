# Plan: Integrar Acampar, sus fichas y la pantalla de campamento

| Campo | Valor |
|---|---|
| ID | PLAN-2026-037 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

El jugador puede acampar, consultar las fichas del grupo, explorar referencias
entre Actores y Lugares y abandonar para continuar jugando. La acción se
entrega integrada desde el núcleo hasta la terminal, con lectura completa y
cierre temporal verificados.

## Referencias

- [Propósito y alcance](../../diseno/acampar.md#1-propósito-y-alcance).
- [API y proyección de fichas](../../tecnica/acciones/07-acampar.md#5-proyección-de-fichas).
- [Sesión, disponibilidad y cierre](../../tecnica/acciones/07-acampar.md#6-sesión-disponibilidad-y-cierre).
- [Finalización única](../../tecnica/acciones/07-acampar.md#63-finalización-única).
- [Estado local y controles](../../tecnica/acciones/07-acampar.md#7-estado-local-y-controles-de-terminal).
- [Regiones y retratos](../../tecnica/acciones/07-acampar.md#81-regiones-y-retratos).
- [Ficha compacta y lectura](../../tecnica/acciones/07-acampar.md#82-ficha-compacta).
- [Recurso de fogata](../../tecnica/acciones/07-acampar.md#83-recurso-de-fogata).
- [Errores e invariantes](../../tecnica/acciones/07-acampar.md#9-errores-e-invariantes).
- [Verificación y aceptación](../../tecnica/acciones/07-acampar.md#10-verificación-y-aceptación).
- [Contratos afectados](../../tecnica/acciones/07-acampar.md#11-contratos-afectados-y-decisiones-cerradas).
- [Controles generales](../../metodologia-desarrollo.md#7-controles-automáticos).
- Dependencia directa: [PLAN-2026-036](./2026-10-26-36-preparar-contenido-regla-acampar.md).

## Alcance

### Incluido

- Proyección de fichas y resúmenes desde el estado actual de la partida.
- Integración de fase, intenciones, identificadores y cierre con la aplicación.
- Pantalla completa, focos, navegación, paginado y recurso de fogata.
- Finalización compartida con Viajar, pruebas de regresión y revisión final.
- Actualización de documentación y cierre de ambos planes.

### No incluido

- Nuevas reglas de descanso, recuperación, supervivencia o encuentros.
- Nuevos elementos o tipos de relación, historial de fichas o cambios de Lua.
- Guardado, animación de fogata, rediseño del mapa o de los retratos.

## Precondiciones

- El alcance y las verificaciones automáticas de PLAN-2026-036 están resueltos;
  su cierre manual puede permanecer agrupado con el de este plan.
- El contrato técnico de Acampar mantiene resueltas las decisiones aplicables.
- Viajar funciona antes de extraer su finalización común.

## Pasos

1. [x] Implementar los tipos de ficha y el proyector de la sección 5. Resolver
       características y relaciones mediante los índices existentes, con sus
       sentidos y resúmenes inmediatos, sin expansión recursiva.
2. [x] Añadir pruebas de fichas actuales, referencias entrantes/salientes,
       recorridos cíclicos, Actor sin retrato, listas vacías y reservas de
       información ausente.
3. [x] Incorporar la fase de campamento, su identificador y la API de consulta
       a Partida según la sección 6; verificar fase e ID antes de consultar.
       Reexportar la fachada neutral necesaria.
4. [x] Extraer la finalización común de Viajar conforme a la sección 6.3 y
       verificar que su recorrido conserva fecha, resto y fase NPC.
5. [x] Conectar apertura, disponibilidad y cierre de Acampar a la regla core;
       implementar rechazo de intenciones incompatibles, antiguas o repetidas
       y el tratamiento de fallos del cierre.
6. [x] Actualizar exhaustivamente consumidores y pruebas de los enums
       ampliados. Mantener los pasos 3–12 en el mismo lote de integración,
       sin entregar un atajo a una pantalla incompleta ni ramas silenciosas.
7. [x] Extraer el cálculo y dibujo compartido de retratos conforme a la
       sección 8.1; comprobar que la vista normal conserva composición,
       transparencia, expresiones y parpadeos.
8. [x] Crear y cargar el recurso de fogata de la sección 8.3, con sus
       validaciones y diagnóstico previo al modo interactivo.
9. [x] Implementar el estado local y los dos focos de la sección 7, incluidas
       consultas atómicas, ancla horizontal, referencias, salida y errores.
10. [x] Implementar ficha, lista, detalle paginado, barras y distribución de
        la sección 8. Verificar lectura completa y recorte seguro de arte.
11. [x] Integrar entrada y renderizado de Acampando en Interfaz sin interferir
        con las viñetas de Viajar, la confirmación global ni el resize.
12. [x] Habilitar el recorrido completo desde la acción del mundo hasta su
        regreso; retirar cualquier adaptación temporal usada durante el lote.
13. [x] Ejecutar pruebas de sesión con RNG controlado: pureza de consultas,
        cierre único, ID atrasado, resto previo, fase NPC y fallo de calendario.
14. [x] Cubrir con TestBackend navegación y retorno desde referencias, textos
        largos, ficha vacía, errores de consulta, Q cancelado, resize y tamaños
        de arte que exceden la región, según las secciones 7–10.
15. [x] Ejecutar el humo completo de la sección 10 con archivos reales y una
        fixture controlada para las variantes que no ejerza el contenido
        oficial; comprobar que se puede volver a Viajar después de acampar.
16. [x] Ejecutar controles generales, validación de todo el contenido oficial y
        build de producción. Repetir pruebas solo ante cambios o fallos que lo
        justifiquen.
17. [x] Realizar una revisión manual agrupada: fichas, biografías, referencias,
        focos, fogata y salida. Registrar también la revisión visible de
        PLAN-2026-036, sin solicitar pruebas por cada detalle.
18. [x] Actualizar el contrato de Acampar y los componentes de su sección 11
        para describir lo implementado; completar resultados, verificaciones
        y desviaciones de ambos planes. Moverlos a completados únicamente al
        cumplir sus criterios y actualizar los índices y enlaces.

## Verificación

- [x] La aplicación publica la nueva fase y permite consultar fichas sin
      depender de terminal, arte ni entrada física.
- [x] Las fichas muestran el estado actual y permiten seguir ambos tipos de
      relación existentes sin cambiar su dirección ni exigir retrato externo.
- [x] Abrir, navegar o consultar no consume RNG ni modifica el estado del Mundo.
- [x] El cierre suma el tiempo previsto, conserva el resto anterior y ejecuta
      NPC una sola vez; errores e intenciones repetidas no duplican efectos.
- [x] El frontend cumple la matriz de entrada y los casos de foco de la
      sección 7; los errores no dejan selecciones parcialmente actualizadas.
- [x] En 80×24 y tamaños mayores se lee toda la ficha mediante sus controles;
      el arte no oculta información ni invade las barras.
- [x] TestBackend cubre textos extensos, referencias cíclicas, ausencia de
      entradas, recortes y redimensionado sin acciones de juego adicionales.
- [x] El humo completo recorre campamento, referencia y salida; después el
      jugador puede volver a usar Viajar.
- [x] Pasan los cuatro controles de la metodología y
      `cargo build --workspace --release`, incluida la regresión de Viajar.
- [x] Se valida todo el contenido y Lua oficiales y se registra la revisión
      manual conjunta de la experiencia visible.
- [x] Ambos planes, contratos e índices reflejan resultados reales; no se
      declaran implementadas capacidades aplazadas.
- [x] Se cumplen los controles y la definición de terminado de la
      [metodología](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Fase y pantalla se integran como un solo corte vertical; ampliar los enums
  exige revisar todos los consumidores, no solo la nueva vista.
- Extraer la finalización compartida puede afectar Viajar. Su regresión se
  comprueba junto al cambio y antes del cierre del incremento.
- La prueba de lectura y recorte usa los casos exigidos por el contrato, sin
  cambiar el diseño visual para acomodar una prueba simplificada.
- La revisión manual se concentra al final. Un defecto bloqueante mantiene el
  plan pendiente; los refinamientos posteriores se registran sin ampliar el
  alcance ni inventar reglas durante la ejecución.
- Toda decisión nueva se resuelve en la fuente técnica correspondiente antes
  de modificar los pasos del plan.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26.
- Resultado: Acampar integrado desde la API core hasta la terminal: fichas actuales y referencias incidentes, dos focos, fogata, paginado y cierre único con fase NPC. Viajar conserva su recorrido y finalización.
- Verificaciones ejecutadas: pasan `cargo fmt --all -- --check`,
  `cargo check --workspace --all-targets`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
  Verificación final conjunta: 96 pruebas (40 core, 27 app, 25 terminal y 4
  importador), incluida la carga oficial de contenido y Lua y la regresión de
  Viajar. Los casos y el recorrido están registrados en la
  [sección 12 del contrato](../../tecnica/acciones/07-acampar.md#12-verificación-de-la-implementación--2026-10-26).
- Revisión manual: Ejecutable release revisado en PTY 80×24: Príncipe → Bandido → Hamada Roja → Bandido → Descenso del Drâa → Valle del Drâa → Cazador Amigable; PageDown, Q cancelado, retorno horizontal al Sirviente, Esc + Enter, selector de Viajar y salida con restauración de terminal (código 0).
- Desviaciones respecto del plan: ninguna funcional. Los dos planes se
  verificaron y cerraron como el incremento conjunto previsto, sin entregas
  intermedias ni pruebas manuales por cada detalle.
- Trabajo posterior: Ningún bloqueo conocido. Los refinamientos de arte o mecánicas aplazadas requieren su propio alcance.
