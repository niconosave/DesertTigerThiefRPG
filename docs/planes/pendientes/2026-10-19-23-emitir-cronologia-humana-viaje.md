# Plan: Emitir la cronología humana y causal de Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-023 |
| Estado | Implementado; cierre agrupado en PLAN-2026-020 |
| Creado | 2026-10-19 |
| Actualizado | 2026-10-20 |

## Resultado buscado

El proyecto humano reanudable de `Viajar` produce una cronología semántica
completa y ordenada. La cronología hace observables su propósito, deliberación,
compromiso, subobjetivos, acciones particulares, interpretaciones,
replanteamientos y una conclusión cronológica separada. La aplicación transforma
las situaciones dramáticas y registros que `Viajar` ya compuso en prosa y
`VistaFragmento` hardcodeados que explican qué intentan las personas, por qué
actúan así, qué respondió el Mundo y cómo entienden el nuevo estado, sin
inventar hechos ni ocultar pasos de la simulación. La aplicación puede dividir
o agrupar esa presentación, pero no decide qué momentos pertenecen a una
situación ni crea escenas nuevas.

La implementación distingue tres niveles: el núcleo registra hechos y
transiciones neutrales; la aplicación les da contexto y prosa; cada frontend
decide cómo presentarlos. No hay máximo de escenas, momentos o viñetas ni una
correspondencia obligatoria de uno a uno entre esas unidades.

## Referencias

- [Proyecto humano de Viajar](../../tecnica/acciones/04-proyecto-humano-viajar.md).
- [Vocabulario cronológico](../../tecnica/acciones/03-cronologia-causal-viaje.md#2-vocabulario-y-relación-entre-unidades).
- [Construcción dramática](../../tecnica/acciones/03-cronologia-causal-viaje.md#4-construcción-dramática-fiel-a-la-simulación).
- [Cronología observable](../../tecnica/acciones/03-cronologia-causal-viaje.md#5-cronología-observable-y-revelación-progresiva).
- [Registros del motor](../../tecnica/componentes/05-motor-acciones.md#cronología-causal).
- [Fases y vistas de aplicación](../../tecnica/componentes/04-sesion-aplicacion.md#5-fases-observables).
- [Fase NPC](../../tecnica/componentes/07-fase-npc.md).
- Dependencia: [PLAN-2026-022](./2026-10-19-22-refactorizar-viajar-proyecto-humano.md).

## Alcance

### Incluido

- registros neutrales y ordenados de propósito, propuesta, compromiso,
  subobjetivo, interpretación y replanteamiento;
- asociación de cada acción particular ejecutada con un único momento causal,
  el subobjetivo que le da sentido y el progreso exacto al que pertenece;
- apertura y cierre explícitos de situaciones dramáticas contextualizadas por
  `Viajar`, sin usar la clase de acción particular como escena;
- progreso geográfico y temporal asociado a cada elemento cronológico;
- narrador hardcodeado de aplicación para la dimensión prospectiva —qué quieren
  hacer—, efectiva —qué hacen y qué sucede— y retrospectiva —cómo interpretan
  el resultado—;
- texto principal suficiente para comprender causas conocidas y consecuencias
  observables, más detalle mecánico estructurado para inspección opcional;
- separación entre contexto común de una decisión y el detalle exclusivo de
  cada opción, con ausencia representada por `None` y etiquetas breves validadas;
- atribución visible de propuestas, conocimientos y procedencias materiales a
  Actores o Lugares concretos;
- decisiones del jugador y respuestas automáticas de NPC apoyadas en la misma
  secuencia semántica;
- `CronologiaNpc` interna que conserve todos los registros, `ResumenNpc`
  estructurado con cada ejecución y su proyección breve material de longitud
  semántica acotada, sin abrir viñetas interactivas;
- pruebas de exhaustividad, orden, revelación progresiva y correspondencia entre
  registros, momentos y fragmentos.

### No incluido

- composición, paginación, cámara, arte o controles de Ratatui;
- nuevas reglas físicas, acciones particulares, medios o procedencias;
- generación libre de texto, clasificación automática de emociones o
  simulación del pensamiento;
- historial persistente de viajes;
- conflictos internos o procura de medios ausentes.

## Precondiciones

- PLAN-2026-021 y PLAN-2026-022 completados.
- La máquina del proyecto expone transiciones semánticas y las acciones
  particulares entregan trazas completas sin prosa.
- Los contratos de cronología, sesión y fase NPC están aprobados.

## Pasos

1. [x] Incorporar al núcleo la suma tipada de registros cronológicos y asignar
       identidad estable a cada momento causal sin introducir prosa.
2. [x] Emitir registros de proyecto en cada transición humana material y
       conservar Actor, propuesta, plan, procedencia, subobjetivo y causa de
       replanteamiento que correspondan.
3. [x] Enlazar exactamente una vez cada traza particular ejecutada con su
       momento causal, situación y progreso, sin fusionar repeticiones.
4. [x] Usar el registro de propósito como única apertura y emitir cambios de
       situación, progreso y la conclusión separada en el orden real de la
       simulación; retirar cualquier límite o muestreo de escenas.
5. [x] Adaptar la aplicación para normalizar los registros antes de mutar la
       fecha final y convertir las situaciones ya emitidas en
       `VistaFragmento`, `VistaDecision`, prosa y detalles sin volver a ejecutar
       reglas ni reagrupar momentos en escenas nuevas. Asignar además la tabla
       normativa `VistaRegistroProyecto` → `RolPresentacion` y los roles exactos
       de las cuatro clases de decisión definidos por la fachada.
6. [x] Escribir el guion contextual inicial de `Viajar` para propósito,
       deliberación, compromiso, episodios, interpretación, replanteamiento,
       llegada, abandono e interrupción.
7. [x] Adaptar la fase NPC para conservar internamente la misma cronología,
       derivar un resumen estructurado con cada momento y producir una vista
       breve que cuente acciones y replanteamientos, nombre solo sus últimos
       cambios materiales, incluya la duración simulada total y conserve errores
       reportables sin avanzar la fecha.
8. [x] Añadir pruebas unitarias y de integración para secuencias largas,
       repetidas, multidiarias, terrestres, de camellos y marítimas.
9. [x] Ejecutar los controles generales y sincronizar estados documentales que
       hayan cambiado efectivamente.

## Verificación

- [x] Toda transición humana material produce un registro de proyecto y una
      consulta pura no produce ninguno.
- [x] Toda acción particular ejecutada aparece exactamente una vez como momento
      causal, aunque su escena tenga otras acciones o su presentación requiera
      varias viñetas.
- [x] No existe máximo, presupuesto, muestreo ni descarte de registros, escenas
      o fragmentos.
- [x] El texto permite reconstruir propósito, plan, fuente del medio,
      subobjetivo, causas conocidas, consecuencia e interpretación sin abrir el
      detalle mecánico.
- [x] El detalle coincide con la traza neutral y no revela hechos ocultos por
      inferencia aritmética o redacción.
- [x] Una decisión con varias propuestas conserva por separado el contexto común
      y el intercambio propio de cada opción, sin datos duplicados, valores
      opcionales vacíos ni etiquetas de más de 40 columnas visibles.
- [x] Una deliberación sin acción particular no recibe tirada, traza ni momento
      causal falsos.
- [x] Cada registro de proyecto y cada decisión reciben el
      `RolPresentacion` normativo sin analizar su prosa; la clasificación del
      registro prevalece cuando una vista también refiere una situación.
- [x] Lugar confirmado, siguiente Lugar, fecha proyectada y duración acumulada
      corresponden al punto exacto del registro y no adelantan consecuencias.
- [x] Jugador y NPC usan la misma cronología; solo difieren en quién responde y
      en que la fase NPC la resume sin ventanas interactivas.
- [x] Un viaje NPC arbitrariamente largo no alarga su frase por cada ejecución:
      la cronología y el resumen estructurado siguen completos, mientras la
      vista usa propósito/destino, duración total, conteos, último replanteamiento
      y última consecuencia persistente; la fecha del Mundo permanece idéntica.
- [x] Un rechazo inicial no inventa conclusión, Lugar final o duración, y un
      error de política anterior a solicitar no inventa tampoco destino o
      propósito.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- La prosa puede ocultar una carencia del modelo. Se corregirá el registro o la
  regla que falte; el narrador no inventará una explicación compensatoria.
- Una transición de proyecto puede sostener una viñeta sin acción particular,
  pero nunca se disfrazará de resolución mecánica.
- Los textos son hardcodeados deliberadamente: expresan el contexto específico
  de `Viajar`, mientras las acciones particulares siguen siendo reutilizables.
- El progreso se captura al emitir cada registro; reconstruirlo a partir del
  Mundo final produciría adelantos y está prohibido.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización técnica: 2026-10-20; cierre documental agrupado en PLAN-2026-020.
- Resultado: propósito, propuesta, compromiso, subobjetivos, interpretaciones y
  conclusión se emiten en orden; cada acción particular conserva una identidad
  causal única, su subobjetivo y su progreso. La fase NPC retiene la cronología
  completa y muestra solo un resumen acotado.
- Verificaciones ejecutadas: viajes con más de seis viñetas, IDs causales
  únicos, progreso no decreciente, trazas repetidas sin descarte, cronología NPC
  completa, formato, check, Clippy estricto, pruebas y release.
- Desviaciones respecto del plan: el primer vertical representa el detalle
  visible como líneas ya filtradas en la vista, en vez del árbol anidado del
  ejemplo conceptual. No hay todavía IDs de situación separados de
  `ClaveEscena`; la escena sigue siendo contexto de `Viajar`, nunca la identidad
  de una acción particular.
- Trabajo posterior: [PLAN-2026-024](./2026-10-19-24-presentar-proyecto-viaje-terminal.md).
