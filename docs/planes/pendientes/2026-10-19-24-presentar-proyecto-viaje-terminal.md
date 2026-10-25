# Plan: Presentar el proyecto y seguimiento de Viajar en terminal

| Campo | Valor |
|---|---|
| ID | PLAN-2026-024 |
| Estado | Sustituido; cierre manual anterior incompleto |
| Creado | 2026-10-19 |
| Actualizado | 2026-10-24 |

> Sustituido por la secuencia PLAN-2026-025 a PLAN-2026-031 del
> [refactor implementado](../completados/README.md#refactor-del-reglamento-de-viajar).
> Conserva las verificaciones efectivamente realizadas y el cierre manual que
> quedó pendiente; el desarrollo nuevo no completa retrospectivamente este plan.

## Resultado buscado

Durante un viaje, la terminal presenta en orden todas las viñetas del proyecto
humano y de sus situaciones dramáticas mediante la ventana superpuesta ya
existente. El jugador puede comprender qué quiere hacer el Grupo, qué plan
adopta, de dónde salen sus medios, qué problema intenta resolver, qué ocurrió y
por qué decide continuar o replantear. La cámara, el marcador, el marco y la
barra contextual permiten seguir dónde está el Grupo y cuánto tiempo
transcurrió; `[D]` permite inspeccionar el cálculo visible sin alterar la
partida.

No se crea una pantalla especial para planificar. Propósito, deliberación,
compromiso, episodios e interpretación usan los mismos modos de presentación y
decisión que el resto de la acción.

## Referencias

- [Proyecto humano de Viajar](../../tecnica/acciones/04-proyecto-humano-viajar.md).
- [Texto principal, detalle y paginación](../../tecnica/acciones/03-cronologia-causal-viaje.md#8-texto-principal-detalle-d-y-paginación).
- [Progreso geográfico y temporal](../../tecnica/acciones/03-cronologia-causal-viaje.md#7-progreso-geográfico-y-temporal).
- [Contrato terminal con la aplicación](../../tecnica/componentes/09-frontend-terminal.md#5-contrato-con-la-aplicación).
- [Composición terminal](../../tecnica/componentes/09-frontend-terminal.md#7-composición-de-la-pantalla).
- [Criterios del frontend](../../tecnica/componentes/09-frontend-terminal.md#11-pruebas-y-criterios-de-aceptación).
- Dependencia: [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md).

## Alcance

### Incluido

- presentación ilimitada y ordenada de todos los fragmentos recibidos;
- roles visuales para propósito, deliberación, compromiso, planteamiento,
  desarrollo, desenlace, replanteamiento y conclusión;
- uso de la misma ventana superpuesta para registros de proyecto, escenas y
  decisiones, sin añadir otro modo o pantalla;
- títulos y texto que nombren al proponente, al aportante y la procedencia de un
  medio cuando la aplicación los entregue;
- Lugar, fecha proyectada y duración acumulada en marco y barra contextual;
- cámara, foco y marcador `@` guiados únicamente por el progreso recibido;
- detalle mecánico local con `[D]`, respeto de información oculta y paginación
  que no confirme prematuramente;
- asociación opcional entre situación semántica y arte local, sin usar la
  identidad de una acción particular; los registros de proyecto sin situación
  se presentan correctamente como texto solo;
- decisiones cuyo texto y lista completa de alternativas se paginan localmente
  sin limitar las propuestas del proyecto;
- resúmenes NPC de forma acotada ya redactados por la aplicación en la barra
  contextual, sin enumerar allí una cronología de longitud arbitraria;
- pruebas de búfer con `TestBackend` y recorridos manuales terrestre y marítimo.

### No incluido

- rutas dibujadas, movimiento interpolado entre tiles o una pantalla táctica de
  planificación;
- nuevas reglas, textos, agrupaciones, pensamientos, tonos o cálculos creados
  por el frontend;
- arte ASCII definitivo para cada transición o situación;
- viñetas interactivas de la fase NPC;
- historial persistente.

## Precondiciones

- PLAN-2026-023 completado.
- La aplicación entrega roles, transición de proyecto opcional, situación
  opcional, progreso, texto y detalle conforme al contrato aprobado.

## Pasos

1. [x] Adaptar el estado visual para conservar rol, transición de proyecto,
       situación opcional, progreso, página y visibilidad local del detalle.
2. [x] Presentar propósito, deliberación, compromiso, subobjetivo,
       interpretación y replanteamiento con el modo de presentación existente y
       sin fabricar un detalle mecánico ni reclasificar el
       `RolPresentacion` recibido.
3. [x] Presentar escenas, decisiones y conclusiones con títulos, prosa, Lugar,
       fecha y duración recibidos, sin inspeccionar el Mundo ni reconstruir
       reglas.
4. [x] Mover cámara, resaltado y marcador según el Lugar confirmado de cada
       elemento cronológico, sin dibujar rutas ni adelantar el siguiente Lugar.
5. [x] Mantener la situación semántica separada de la elección local de arte;
       situación o ilustración ausentes sin perder la viñeta.
6. [x] Implementar `[D] Detalles`: contexto común durante la narración y, en la
       etapa de opciones, bloques de detalle para la página visible, con
       paginación local propia y sin mezclar datos de otras páginas.
7. [x] Dividir opciones en bloques lógicos de hasta 24, asignar sus atajos
       estables antes de subdividir por ancho y validar etiquetas completas de
       hasta 40 columnas, manteniendo ocultos los campos que la aplicación no
       exponga.
8. [x] Adaptar la barra contextual para transiciones de proyecto y para dibujar
       el `VistaResumenNpc` breve sin volver a resumir su cronología interna.
9. [x] Ampliar `TestBackend` con un proyecto completo, más de seis escenas,
       acciones repetidas, una escena con varias acciones, una acción en varias
       viñetas, salto multidiario, traslado, fallo, redimensionado, detalle común
       y detalles diferentes por opción.
10. [ ] Ejecutar controles generales y realizar recorridos manuales por tierra y
       mar antes del cierre integral de PLAN-2026-020.

## Verificación

- [x] No hay contador, presupuesto ni ruta visual que omita fragmentos o cambie
      su orden, proyecto, escena o momentos causales.
- [x] Propósito, deliberación, compromiso y las transiciones vigentes se ven en la misma
      interfaz de viñetas; no aparece una pantalla adicional.
- [x] El frontend conserva el `RolPresentacion` recibido y no lo deduce desde el
      título, el texto o la presencia de una situación.
- [x] Una transición sin acción particular carece de tirada y detalle mecánico,
      pero sigue siendo legible y presentable.
- [x] Una propuesta de camellos nombra al Actor aportante y una propuesta
      náutica nombra el Lugar y servicio recibidos; la terminal no inventa su
      origen.
- [x] El seguimiento se actualiza solo con el progreso vigente y nunca adelanta
      desenlace, fecha, estado o Lugar.
- [x] La terminal representa una escena con varias acciones y una acción en
      varias viñetas sin exigir una relación uno a uno.
- [x] La explicación principal conserva intención, causa y consecuencia con `[D]` cerrado; el panel
      abierto coincide con el desglose recibido.
- [x] En las opciones, `[D]` muestra exclusivamente el bloque visible, reinicia
      su paginación al cambiar de página y `Enter` nunca confirma mientras el
      panel está abierto.
- [x] Textos largos y listas numerosas de opciones se leen completos en
      `80 × 24` mediante paginación y bloques de hasta 24, sin perder controles,
      cambiar el atajo de una opción por redimensionado ni limitar las propuestas
      semánticas; cada etiqueta validada ocupa como máximo 40 columnas.
- [x] `Esc` no cancela ni abandona una acción iniciada; el abandono solo ocurre
      mediante la opción semántica `Abandonar` en un replanteamiento seguro.
- [x] El mapa sigue siendo visual: cámara y `@` cambian por `IdLugar`, pero no
      producen coordenadas de dominio ni rutas entre tiles.
- [x] `TestBackend` recorre el lote completo sin una terminal real.
- [ ] La prueba manual terrestre y marítima permite reconstruir intención,
      plan, procedencias, acciones, causas, consecuencias, interpretación,
      posición y tiempo sin abrir `[D]`.
- [x] Pasan formato, check, Clippy estricto, pruebas y build release.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Una viñeta extensa puede ocupar varias páginas, pero avanzar página nunca
  equivale a confirmar el fragmento.
- La ventana puede ocultar temporalmente parte del mapa; marco y cámara
  conservan el contexto suficiente entre fragmentos.
- Un problema de redacción o hechos ausentes se corrige en aplicación o núcleo;
  el frontend no lo compensa inventando explicaciones.
- Un registro de proyecto puede no tener ilustración. El texto tiene prioridad
  y su ausencia no constituye un error.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: pendiente del recorrido manual final.
- Resultado: la terminal presenta la cronología sin presupuesto de viñetas,
  sigue Lugar, siguiente Lugar, fecha y duración, mueve cámara y marcador con el
  progreso, pagina texto y opciones, y ofrece `[D]` sin mutar la partida.
- Verificaciones ejecutadas: `TestBackend` en `80 × 24`, detalle largo sin
  confirmación por `Enter`, contenido paginado sin descarte, atajos estables,
  formato, check, Clippy estricto, pruebas del workspace y build release.
- Desviaciones respecto del plan: cada página visual muestra una sola opción
  completa; es la subdivisión conservadora de los bloques de 24 para garantizar
  legibilidad mínima. El detalle mecánico llega como líneas visibles ya
  preparadas por la aplicación. Falta exclusivamente la revisión humana de los
  recorridos terrestre y marítimo.
- Trabajo posterior: cerrar [PLAN-2026-020](./2026-10-17-20-equilibrar-viajar-expediciones.md).
