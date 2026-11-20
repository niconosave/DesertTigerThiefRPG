# Entregas de Encuentros, Atacar y Rescatar

Estado: ciclos 1 y 2 implementados; el ciclo 2 espera revisión de jugabilidad. Fecha: 2026-11-19.

Esta hoja de ruta organiza el trabajo. No define mecánicas ni sustituye el plan
de cada incremento. Su referencia es el [diseño de mecánicas](../diseno/encuentros-y-atacar.md)
y el [contrato técnico](../tecnica/acciones/10-encuentros-y-atacar.md).

## Cómo se documenta cada ciclo

Se mantienen dos diseños canónicos vivos: mecánicas y contrato técnico. Cada
cambio acordado actualiza allí su regla o contrato antes de implementarse. El
estado debe distinguir lo implementado, lo diseñado y lo aplazado por capacidad;
terminar el primer ciclo no convierte todo el subsistema en implementado.

Cada ciclo tiene un plan creado desde [la plantilla](./PLANTILLA.md), con un
resultado observable, alcance acotado, referencias, precondiciones y aceptación.
Al completarlo se registra la evidencia y se mueve el mismo archivo a
`completados/`. Los ajustes pequeños detectados jugando no necesitan un documento
de diseño nuevo ni un plan separado por cada detalle.

Un documento específico adicional corresponde cuando una capacidad adquiere
reglas o un contrato propio reutilizable: por ejemplo, arquería, percepción o
verticalidad. Su contenido pasa a ser la fuente de ese detalle; el diseño común
lo enlaza. No se copia todo el diseño para cada ciclo.

El plan siguiente se detalla después de jugar el resultado anterior. Esta hoja
anticipa capacidades para conservar una dirección, sin congelar todas las tareas
futuras ni aprobar sus reglas pendientes.

## Qué cuenta como una entrega funcional

Al terminar un ciclo se puede iniciar el juego, llegar a la función nueva, usarla
y terminar en un estado válido. El jugador recibe instrucciones concretas para
probarla. También se mantienen accesibles las funciones anteriores.

Una demostración de generación o reproducción puede servir como comprobación
intermedia, pero no se presenta como Atacar jugable. La primera entrega de Atacar
requiere un objetivo, oposición reglada, desenlace, tiempo y consecuencias.
Las tareas de modelo, reglas, aplicación y dibujo se agrupan dentro de una misma
entrega observable; no constituyen entregas independientes por capa.

Las revisiones manuales se hacen al cerrar cada entrega. No se interrumpe cada
modificación para pedir una prueba del jugador. Los hallazgos se convierten en
ajustes del ciclo actual o en cambios de diseño del siguiente según su alcance.

## Secuencia propuesta

| Ciclo | Capacidad que se prueba jugando | Límite de la entrega |
|---|---|---|
| 1 | Atacar a un objetivo en un escenario pequeño generado, atravesar oposición y concluir el intento. | Una familia de cuartos, pocas fichas, movimiento cercano, reacción, combate y salidas básicas. |
| 2 | Un acontecimiento escrito transforma ese encuentro y las fichas continúan desde la nueva situación. | Un caso narrativo completo y su pequeño repertorio de comandos; se amplía después. |
| 3 | Nuevas disposiciones y familias ofrecen situaciones distintas al mismo Atacar. | Incorporar una familia o capacidad de aproximación por entrega, con sus reglas acordadas. |
| 4 | Rescatar a una persona y extraerla usando el encuentro común. | Primer reglamento de rescate de personas; no incluye automáticamente tesoros ni inventarios. |
| Posteriores | Nuevas maniobras, alturas, escenarios navales y otras acciones. | Se deciden a partir del juego y tienen su diseño específico cuando corresponde. |

El ciclo 3 puede repetirse en varias entregas pequeñas. Rescatar no depende de
haber implementado todas las familias ni todas las habilidades. Su posición se
puede adelantar cuando la base compartida esté probada.

### Ciclo 1: episodio mínimo de Atacar

[PLAN-2026-047](./pendientes/2026-11-19-47-atacar-episodio-minimo.md) detalla el
trabajo inicial implementado. El primer episodio usa dos cuartos dentro de un
área y objetivos públicamente localizables declarados por el contenido. El grupo se aproxima, afronta oposición,
intenta matar al objetivo y termina con éxito, retirada o derrota. El movimiento
y los ataques se observan en el tablero con un texto general por viñeta.

La derrota debe cerrar de forma válida si el reglamento permite heridas,
incapacitación o muerte; no puede quedar sin continuación por haber planificado
solamente el caso favorable. La captura puede llegar después, siempre que el
reglamento del primer episodio no produzca una captura sin persistencia implementada.

Se reutiliza la presentación de eventos existente para los puntos iniciales o
finales incorporados. La transformación narrativa arbitraria del tablero y los
comandos Lua locales extensos corresponden al ciclo 2. Las interfaces comunes
necesarias se preparan sin implementar repertorios vacíos de capacidades futuras.

**Precondición actual:** completar el reglamento mínimo de las
[mecánicas, sección 16](../diseno/encuentros-y-atacar.md#16-propuestas-y-decisiones-todavía-abiertas).
El plan no elige daño, diagonales, detección ni duraciones durante la programación.
El contrato técnico ya separa las decisiones que esto requiere.

### Ciclo 2: acontecimiento escrito dentro del encuentro

Se elige un caso que modifique realmente el tablero: por ejemplo, cambiar un
acceso e incorporar oposición, o reubicar al grupo mediante una transición
narrada. El caso permite comprobar presentación, confirmación conjunta y
replanteamiento de recorridos. La aceptación exige continuar y terminar el ataque
usando el estado nuevo; mostrar una ventana sin efectos no completa esta entrega.

El [PLAN-2026-048](./pendientes/2026-11-19-48-eventos-encuentro-ogro.md)
implementa el caso de Gorun: una brecha real, escenas propias y continuación del
ataque. Selecciona cambio de terreno y reubicación como primeros comandos. Los demás comandos del contrato permanecen diseñados y pendientes.

### Ampliaciones siguientes

Cada familia, maniobra o nueva acción se incorpora con un recorrido jugable que
la necesite. Un bosque no se considera una ampliación mecánica solo por cambiar
el color de los tiles; debe quedar claro qué variación se está probando, sea de
geometría, información, oposición o reglas del terreno.

Los niveles futuros ya están previstos en identidades y posiciones. La capacidad
de subir, cambiar el nivel de cámara o atacar entre alturas tendrá una entrega
propia; no se exige completar la verticalidad para jugar los ciclos anteriores.

## Cierre de cada ciclo y verificación

Se verifican las reglas y fronteras modificadas, con casos reproducibles y
salidas breves. Se conserva la preferencia explícita del usuario de mantener las
pruebas acotadas por desarrollo. Si un cambio requiere una verificación más
amplia, se explica antes y se acuerda una ejecución que evite logs extensos y
repeticiones innecesarias.

Al terminar se registra qué puede probarse, cómo iniciarlo, qué comprobaciones
pasaron y qué queda pendiente. Después de jugar se revisa el alcance de la
siguiente entrega; las decisiones nuevas se anotan en su diseño canónico.
