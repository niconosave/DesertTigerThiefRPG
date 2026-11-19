# Plan: Jugar un primer episodio completo de Atacar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-047 |
| Estado | Pendiente |
| Creado | 2026-11-19 |
| Actualizado | 2026-11-19 |

## Resultado buscado

El jugador puede elegir Atacar a un objetivo válido, recorrer un pequeño escenario
generado con oposición, observar movimiento y combate bajo reglas y concluir el
intento. La acción conserva tiempo y consecuencias en la Partida y libera el
tablero. Viajar, Acampar y los eventos existentes siguen utilizables.

La entrega incluye todas las capas necesarias para ese episodio. Un tablero que
solo reproduce una secuencia preparada no cumple este resultado.

## Referencias

- Mecánicas y alcance: [diseño, secciones 3–14](../../diseno/encuentros-y-atacar.md#3-acciones-generales-que-usan-encuentros).
- Precondiciones de reglamento: [mecánicas, sección 16](../../diseno/encuentros-y-atacar.md#16-propuestas-y-decisiones-todavía-abiertas).
- Modelos, generación y ejecución: [contrato técnico, secciones 2–11](../../tecnica/acciones/10-encuentros-y-atacar.md#2-situación-del-código-y-ampliación-necesaria).
- Mutación y cierre: [contrato técnico, secciones 13–16](../../tecnica/acciones/10-encuentros-y-atacar.md#13-confirmación-conjunta-y-azar).
- Percepción y presentación: [contrato técnico, secciones 17–21](../../tecnica/acciones/10-encuentros-y-atacar.md#17-proyección-pública-memoria-y-datos-ocultos).
- Límites de implementación: [contrato técnico, sección 23](../../tecnica/acciones/10-encuentros-y-atacar.md#23-reglas-pendientes-y-condiciones-para-el-plan-jugable).
- Secuencia de entregas: [hoja de ruta](../encuentros-atacar-rescatar.md).
- Controles y cierre: [metodología](../../metodologia-desarrollo.md), aplicando
  la preferencia explícita del usuario de pruebas acotadas por desarrollo.

## Alcance

### Incluido

- Entrada real de Atacar, selector de blanco y validación de la solicitud.
- Un perfil pequeño generado por código, con pocas áreas, obstáculos y vacíos,
  sin archivos de mapas particulares por Lugar.
- Fichas locales vinculadas al grupo, al objetivo y a la oposición definida por
  el contenido del episodio, sin asumir que todo habitante del Lugar es enemigo.
- Preparación y aproximación sencillas, combate cercano, reacción al abandonar
  cualquier casilla amenazada y salida conforme al reglamento mínimo acordado.
- Movimiento resuelto por casillas, texto estable por viñeta y reproducción visual.
- Decisiones de conjunto que correspondan al episodio, incluida la retirada.
- Éxito, retirada y todos los cierres de derrota que pueda producir el reglamento
  activo, con persistencia e información final comprensible.
- Tiempo confirmado una sola vez e integración con el cierre existente de Partida.
- Puntos de inicio y final e integración compatible de los eventos existentes.
- Condiciones de prueba reproducibles y revisión manual del episodio completo.

### No incluido

- Rescatar, atacar Lugares, saqueo o inventario de tesoros.
- Varias familias de generadores, escenarios navales o mapas locales persistentes.
- Saltos, trepar, nadar, arquería, fuego simulado o destrucción reglada del terreno.
- Nuevos pisos, cámara por niveles o combate vertical; las posiciones conservan
  la distinción de área y nivel prevista en el contrato.
- Repertorio amplio de comandos Lua locales y acontecimientos que transforman el
  tablero en mitad de una viñeta: su primer caso completo pertenece al ciclo 2.
- Nuevas políticas NPC, finales de campaña o sucesión del personaje jugador.
- Cautiverios complejos o captura que todavía no tenga reglamento y persistencia.

## Precondiciones

El plan está preparado, pero estas precondiciones permanecen abiertas al crearlo.
No se convierten en decisiones implícitas al implementar:

- [ ] Completar en las mecánicas el reglamento mínimo utilizado: golpe, defensa,
      estados y consecuencias; ritmo de actuación; vecindad, ocupación y límite
      de reacciones. El gatillo de salida amenazada ya está confirmado.
- [ ] Fijar la información disponible, los objetivos ofrecidos y la aproximación
      del primer episodio. No introducir sigilo sin reglas de detección.
- [ ] Fijar decisiones de conjunto, duración y condiciones de salida, incluyendo
      ubicación de supervivientes y objetivo fugado si ese resultado es posible.
- [ ] Confirmar qué derrotas puede producir el reglamento inicial y darles cierre
      válido. Un desenlace no implementado no puede aparecer con resultados perdidos.
- [ ] Precisar en el contenido de prueba el perfil, participantes y recorridos
      para probar éxito, retirada y derrota sin imponerlos en la historia canónica.
- [ ] Actualizar el contrato técnico si el cierre de las mecánicas altera sus
      contratos, y marcar como aprobadas para implementar las secciones aplicables.

Las propuestas pendientes siguen en los documentos enlazados. Este plan no
contiene una fórmula provisional que permita saltar estas precondiciones.

## Pasos

1. [ ] Verificar que las precondiciones estén resueltas y que el caso de aceptación
       esté identificado. Registrar los enlaces al reglamento y contenido definitivos.
2. [ ] Ampliar la envoltura de acciones para Atacar conservando los tipos propios de
       Viajar; integrar disponibilidad, solicitud, continuación, decisiones y cierre.
       Cubrir una ejecución existente de Viajar en esa frontera.
3. [ ] Incorporar estado local, IDs, posiciones con área/nivel, fichas e índices de
       ocupación. Validar referencias y vínculo único con cada Actor del mundo.
4. [ ] Incorporar el perfil mínimo y su generador acotado: áreas conectadas,
       recorridos utilizables, obstáculos, vacíos y posiciones de participantes.
       La misma ejecución conserva su escenario entre avances y repintados.
5. [ ] Incorporar las reglas aprobadas necesarias para desplazarse y combatir,
       incluida la reacción previa a abandonar una casilla amenazada. Reutilizar
       una resolución de golpe para combate ordinario y reacción.
6. [ ] Conducir el episodio mediante sus viñetas y tramos acotados: aproximación,
       oposición, objetivo y conclusión; ofrecer las decisiones acordadas y evitar
       recorridos o reintentos infinitos. No resolver todo el episodio por adelantado.
7. [ ] Confirmar conjuntamente cambios del encuentro y del Mundo, registro,
       continuación y azar. Conservar los cambios persistentes ya confirmados y
       aplicar al cerrar solo los pendientes.
8. [ ] Exponer vistas filtradas y lotes con secuencias, conservando el orden de
       reacciones y desplazamientos. Preparar texto general y resumen por viñeta,
       sin un párrafo por casilla o golpe.
9. [ ] Incorporar la vista terminal y los recursos mínimos compatibles con los
       modos gráficos existentes: tablero, fichas, lectura, decisiones, pausa,
       velocidad, omisión y cambio de tamaño.
10. [ ] Integrar las fronteras de inicio/final con el coordinador existente y la
        confirmación temporal de Partida. Mantener utilizable el juego fuera del
        encuentro; no habilitar comandos locales que aún no estén implementados.
11. [ ] Ejecutar las verificaciones acotadas y dejar un recorrido manual concreto
        para elegir objetivo, completar el episodio, retirarse y observar una derrota.
        Registrar limitaciones y revisar la experiencia con el usuario al cerrar.
12. [ ] Actualizar el estado de las capacidades realmente implementadas en los
        diseños e índices. Registrar resultados y mover este plan a `completados/`
        cuando la entrega esté terminada; detallar el ciclo 2 a partir de lo probado.

## Verificación

### Comportamiento y aceptación

- [ ] Se llega a Atacar desde la interfaz y se rechaza un blanco inválido sin tiempo
      ni efectos. El objetivo muerto deja de ofrecerse conforme a las mecánicas.
- [ ] El perfil genera un escenario conectado con oposición y espacio irregular.
      Un resize o una página de texto no cambia la generación.
- [ ] El motor resuelve recorridos y ocupación reales; la terminal no inventa rutas.
- [ ] Abandonar una casilla amenazada hacia otra del mismo alcance habilita reacción.
      El fallo permite continuar; el acierto detiene el paso sin deshacer el prefijo.
- [ ] El episodio logra el objetivo y concluye de forma válida. Si el reglamento
      exige salida posterior, esa salida forma parte del recorrido de prueba.
- [ ] Una retirada se resuelve bajo reglas y confirma la situación de los integrantes.
- [ ] Los resultados desfavorables posibles tienen cierre válido, incluyendo estados
      físicos y muerte cuando las reglas los produzcan. No se impone el caso favorable.
- [ ] Los cambios de Mundo, estado local, duración y azar no quedan a medias ante
      un rechazo o error de preparación; repetir una continuación no repite efectos.
- [ ] La vista no anticipa posiciones ocultas. Pausar, acelerar, omitir, consultar
      detalles o redimensionar no cambia el desenlace ni duplica tiempo.
- [ ] Viajar y Acampar siguen accesibles y sus eventos terminan correctamente.
- [ ] El usuario recibe instrucciones que especifican contenido, ubicación/objetivo,
      controles y qué observar. La revisión manual abarca legibilidad y ritmo.

### Controles automáticos acotados

Los nombres de filtros se fijarán con los módulos y casos reales al implementar:

- [ ] Formato Rust de los archivos modificados y compilación de las crates afectadas,
      incluyendo los consumidores de la envoltura de acciones.
- [ ] Clippy sobre los targets afectados, con salida resumida y sin pruebas ejecutadas
      por asociación a un chequeo de compilación.
- [ ] Pruebas reproducibles de generación, ocupación, movimiento, reacción y combate
      del encuentro, sin usar el reloj de pared para esperar animaciones.
- [ ] Pruebas de Atacar en la aplicación: solicitud, secuencia, resultados, tiempo y
      una ejecución de Viajar que cubra el despacho compartido.
- [ ] Pruebas de terminal para reproducción, secuencias, omisión, resize y lectura.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`, con la adaptación de alcance de pruebas
      indicada en Riesgos y decisiones operativas.

No se añade automáticamente una ejecución completa de todos los tests del
workspace al cierre. Si la integración exige ampliar la selección, se avisa al
usuario antes de hacerlo y se acuerda una ejecución de bajo coste de salida.

## Riesgos y decisiones operativas

- **Reglamento abierto.** Las precondiciones señalan decisiones de mecánicas reales,
  no detalles técnicos que el implementador deba elegir. Se actualiza primero su
  fuente canónica; no se resuelven con constantes arbitrarias en este plan.
- **Primer ciclo demasiado amplio.** Su criterio es el episodio mínimo completo.
  Nuevos generadores, habilidades y transformaciones narrativas extensas pasan
  a ciclos posteriores; no se confunde una demostración intermedia con la entrega.
- **Despacho compartido.** La ampliación de la envoltura afecta a Viajar; se cubre
  esa frontera con una prueba seleccionada y el recorrido manual pertinente.
- **Oposición o blancos indebidos.** El contenido de prueba especifica participantes;
  no se toma como enemigo a todo Actor que comparta Lugar con el objetivo.
- **Alcances futuros.** Se registran como pendientes por capacidad; no se declaran
  implementados los contratos completos de encuentros o eventos locales.
- **Coste de verificaciones.** Se aplica la instrucción explícita del usuario de
  mantener pruebas acotadas, que prevalece sobre la lista completa de comandos
  de la sección 7 de la metodología. Se registran comandos y resultados reales
  sin logs extensos; una ampliación requiere avisar antes de ejecutarla.

Esta sección no crea reglas de juego ni decisiones técnicas. Si durante la
implementación aparece una, se actualiza primero su documento canónico; el plan
registra el enlace y su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: pendiente.
- Resultado: pendiente.
- Verificaciones ejecutadas: ninguna; este cambio solo prepara documentación.
- Desviaciones respecto del plan: pendientes de registrar al implementar.
- Trabajo posterior: ciclo 2 de la [hoja de ruta](../encuentros-atacar-rescatar.md),
  cuyo plan detallado se redactará tras jugar esta entrega.
