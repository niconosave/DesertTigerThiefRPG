# Desert Tiger Thief RPG — Documento de diseño general

| Campo | Valor |
|---|---|
| Estado | Borrador base para discusión |
| Versión | 0.1 |
| Fecha | 2026-10-12 |
| Alcance | Diseño de producto y contratos conceptuales |

## 1. Propósito del documento

Este documento define la experiencia que se quiere construir, el vocabulario
común del proyecto y las restricciones que deberán respetar la documentación
técnica y los planes de implementación posteriores. No prescribe todavía crates,
módulos de Rust, traits, formatos de archivo ni APIs concretas.

Las afirmaciones importantes usan identificadores `DG-*` para que otros
documentos puedan enlazarlas sin depender solamente del título de una sección.

- **Decidido:** forma parte de la visión actual y solo debería cambiar mediante
  una revisión explícita de este documento.
- **Provisional:** permite avanzar, pero debe validarse jugando o durante el
  diseño técnico.
- **Abierto:** requiere una decisión futura y no debe asumirse en una
  implementación sin dejar constancia.

## 2. Visión

### 2.1 Resumen

Un RPG sandbox local, para una sola persona y presentado en una terminal con
gráficos ASCII. Cada campaña crea un mundo autocontenido a partir de contenido
editable y una semilla. El jugador controla a un actor dentro de una simulación
de lugares, actores, recursos, relaciones y sucesos que continúa reaccionando a
sus decisiones. La muerte es permanente y cada campaña busca producir una
historia distinta, comprensible y con final.

### 2.2 Promesa al jugador

> Entrar en un mundo desconocido, comprender sus tensiones, elegir qué perseguir,
> improvisar frente a consecuencias duraderas y cerrar una historia propia,
> incluso cuando termina en fracaso.

### 2.3 Forma del producto

- **DG-VIS-001 — Decidido:** la duración objetivo de una campaña completa es de
  cinco a seis horas. No es obligatorio jugarla de una sola vez; se podrá
  suspender y continuar.
- **DG-VIS-002 — Decidido:** el juego es un **sandbox acotado**. Da libertad sobre
  métodos, alianzas, rutas y objetivos secundarios, pero una presión del mundo
  conduce a un desenlace en un horizonte finito.
- **DG-VIS-003 — Decidido:** las campañas son independientes. Al crear una nueva,
  se genera un mundo nuevo en vez de continuar indefinidamente el anterior.
- **DG-VIS-004 — Provisional:** una campaña puede acabar por logro, retirada,
  fracaso irreversible, vencimiento de la presión global o muerte. La fantasía
  concreta del personaje y el significado exacto de “ganar” siguen abiertos.
- **DG-VIS-005 — Decidido:** “campaña” o “partida” designa el recorrido completo
  hasta un desenlace; “sesión” designa cada período real de juego entre apertura
  y cierre de la aplicación.
- **DG-VIS-006 — Decidido:** el frontend inicial se ejecuta en una terminal y usa
  gráficos ASCII.
- **DG-VIS-007 — Decidido:** la implementación inicial se desarrolla en Rust. La
  elección no altera el requisito de mantener el núcleo separado del frontend.

## 3. Pilares de diseño

### DG-PIL-001 — Libertad dentro de límites legibles

El jugador debe poder elegir qué problemas abordar y cómo hacerlo. El mundo
impone escasez, tiempo y consecuencias suficientes para que no todas las opciones
sean compatibles en una misma campaña.

### DG-PIL-002 — Variación con coherencia

Las campañas deben variar en actores, relaciones, conflictos, recursos,
oportunidades y sucesos; no solamente en nombres o en el dibujo del mapa. La
generación debe obedecer restricciones que produzcan mundos jugables y
comprensibles.

### DG-PIL-003 — Consecuencias sistémicas y explicables

Las acciones cambian el estado de actores, lugares y mundo. El jugador debe poder
reconstruir las causas principales de un resultado mediante mensajes, señales y
un registro, sin conocer necesariamente toda la información oculta.

### DG-PIL-004 — Riesgo justo y permadeath significativo

La muerte termina la campaña. Las amenazas graves deben poder anticiparse,
investigarse o evitarse; una derrota puede sorprender, pero no debería depender
solo de información imposible de obtener o de azar opaco.

### DG-PIL-005 — Actores con capacidad de iniciativa

El personaje del jugador y los demás actores participan del mismo modelo
conceptual de acciones. Los actores no jugadores tienen estado y motivaciones
suficientes para alterar el mundo sin esperar siempre al jugador.

### DG-PIL-006 — Contenido modificable

Los elementos de contenido se describen en archivos humanos editables. Una
campaña nueva carga, valida y combina esas definiciones para crear sus instancias
de juego. Modificar contenido no debe requerir recompilar el programa.

### DG-PIL-007 — Núcleo independiente de la presentación

Las reglas pueden ejecutarse sin una terminal. Teclas, caracteres, color, ANSI,
tamaño de ventana y coordenadas de pantalla pertenecen al frontend, no a la
lógica del juego.

### DG-PIL-008 — La terminal es una interfaz completa

El frontend inicial no es una consola de depuración: debe ofrecer información,
controles y respuesta visual suficientes para jugar una campaña completa con
claridad.

## 4. Estructura y ritmo de una campaña

La duración se controla con tiempo del mundo y evolución de conflictos, no con
un cronómetro de tiempo real.

| Fase | Duración orientativa | Propósito |
|---|---:|---|
| Descubrimiento | 30–60 min | Reconocer el entorno, sus actores, peligros y oportunidades. |
| Desarrollo | 3–4 h | Conseguir recursos, crear relaciones y perseguir una ambición. |
| Crisis | 60–90 min | La presión global obliga a comprometerse y aceptar pérdidas. |
| Desenlace | 10–30 min | Resolver, escapar, retirarse, fracasar o morir; producir un epílogo. |

Estas cifras son objetivos de ritmo, no límites rígidos por fase.

- **DG-RIT-001 — Decidido:** desde el comienzo existen varias líneas de acción
  viables; no hay una única secuencia obligatoria de misiones.
- **DG-RIT-002 — Provisional:** una crisis o reloj global se vuelve más urgente a
  medida que avanzan las acciones y garantiza que el mundo no quede estático.
- **DG-RIT-003 — Decidido:** el final debe resumir la causa del desenlace y los
  cambios más importantes provocados por el jugador y otros actores.
- **DG-RIT-004 — Provisional:** la retirada voluntaria puede conservar una historia
  de supervivencia, pero no equivale necesariamente a cumplir la ambición de la
  campaña.

## 5. Bucle de juego

### 5.1 Bucle principal

1. Observar el lugar actual, los actores presentes y la información conocida.
2. Elegir una intención: viajar, investigar, conversar, negociar, tomar, usar,
   descansar, ayudar, robar, luchar u otra acción disponible.
3. Verificar requisitos y comunicar costes o riesgos que el actor pueda conocer.
4. Resolver la acción y registrar sus efectos.
5. Avanzar el tiempo del mundo cuando corresponda.
6. Resolver acciones de otros actores y sistemas del mundo.
7. Presentar consecuencias, nueva información y opciones actualizadas.
8. Repetir hasta alcanzar un desenlace.

- **DG-BUC-001 — Decidido:** toda mutación relevante de la simulación ocurre a
  través de acciones, efectos o sucesos identificables, no desde el frontend.
- **DG-BUC-002 — Decidido:** una acción se expresa como intención semántica; el
  frontend solicita, la lógica valida y la lógica devuelve un resultado.
- **DG-BUC-003 — Provisional:** el juego usa turnos discretos y espera una orden
  del jugador. La unidad exacta de tiempo y el orden de actuación se definirán en
  la documentación técnica.

### 5.2 Flujo de una campaña

```text
Elegir contenido y mods
          ↓
Cargar y validar definiciones
          ↓
Generar y validar el mundo desde una semilla
          ↓
Jugar: intención → acción → consecuencias → avance del mundo
          ↓
Desenlace o muerte
          ↓
Epílogo y registro histórico
```

## 6. Modelo conceptual

### 6.1 Definiciones e instancias

El proyecto distingue dos clases de datos:

- Una **definición de contenido** describe una plantilla editable: por ejemplo,
  un arquetipo de actor, objeto o lugar.
- Una **instancia de partida** posee identidad y estado propios: por ejemplo, un
  actor concreto herido, con inventario y relaciones generadas.

- **DG-MOD-001 — Decidido:** las reglas nunca deben confundir una definición con
  una instancia mutable.
- **DG-MOD-002 — Decidido:** los identificadores de instancia son únicos dentro de
  una campaña y las referencias entre elementos no dependen de nombres visibles.

### 6.2 Elementos del mundo

| Concepto | Responsabilidad conceptual | No es responsable de |
|---|---|---|
| **Mundo** | Raíz de la campaña. Mantiene la semilla, reloj, estado global y referencias o registros de todos sus elementos runtime. | Leer teclado, dibujar o decidir el layout de la terminal. |
| **Lugar** | Contexto lógico en el que hay actores, objetos, oportunidades y sucesos. Sus rutas permiten viajar a otros lugares. | Ser una región de tiles o una coordenada de pantalla. |
| **Actor** | Entidad capaz de intentar acciones. Posee estado, capacidades, conocimiento, relaciones, objetivos y una situación en el mundo. | Mutar el mundo saltándose la resolución de acciones. |
| **Acción** | Intención validable que puede consumir tiempo o recursos y producir efectos. | Mostrar por sí misma menús, teclas o animaciones. |
| **Objeto** | Entidad manipulable o recurso con identidad y estado cuando la partida lo necesita. | Definir código arbitrario desde un mod. |
| **Suceso** | Hecho ocurrido o programado que comunica consecuencias y puede provocar reacciones. | Sustituir silenciosamente el estado autoritativo del mundo. |
| **Relación/ruta** | Vínculo lógico entre actores o lugares, con los datos necesarios para las reglas. | Depender de que dos glifos sean adyacentes. |
| **Mapa visual** | Matriz de tiles o tokens dedicada a representar el mundo. | Decidir ubicación lógica, colisión, distancia, pertenencia o acciones posibles. |

Se podrán añadir categorías —por ejemplo facciones, rumores, misiones o efectos—
cuando el diseño técnico demuestre que tienen identidad y ciclo de vida propios.

### 6.3 Relaciones e invariantes

- **DG-MUN-001 — Decidido:** el mundo ofrece acceso autoritativo a todos los
  elementos runtime que pertenecen a la campaña.
- **DG-MUN-002 — Decidido:** cada dato tiene una sola fuente de verdad. No se
  mantiene, por ejemplo, la ubicación de un actor en dos colecciones mutables que
  puedan contradecirse.
- **DG-MUN-003 — Provisional:** un actor está en exactamente un lugar o en un
  estado explícito de tránsito, retiro o muerte.
- **DG-MUN-004 — Decidido:** el viaje ocurre entre lugares mediante relaciones y
  reglas lógicas, nunca recorriendo tiles por colisión.
- **DG-MUN-005 — Decidido:** el personaje controlado por el jugador es un actor y
  no una excepción completamente separada del modelo del mundo.
- **DG-MUN-006 — Decidido:** la simulación debe poder avanzar y resolver acciones
  sin construir ninguna vista de terminal.

### 6.4 Contrato del mapa visual

El mapa y los lugares son conceptos deliberadamente distintos.

- **DG-MAP-001 — Decidido:** un tile no contiene un identificador de lugar y un
  lugar no contiene coordenadas de tiles como parte de sus reglas.
- **DG-MAP-002 — Decidido:** la lógica no consulta caracteres, colores o tiles
  para validar movimiento, alcance, presencia, interacción o combate.
- **DG-MAP-003 — Decidido:** cambiar, regenerar u omitir el mapa visual no cambia
  por sí mismo el estado lógico de la campaña.
- **DG-MAP-004 — Provisional:** el mundo puede referenciar una representación
  visual neutral generada para la campaña, pero el frontend decide cómo
  convertir sus tokens a ASCII, color y composición en pantalla.
- **DG-MAP-005 — Provisional:** cualquier indicador de actores o lugares que se
  superponga al mapa es una proyección transitoria de presentación, no una nueva
  fuente de verdad.

Esta separación implica una obligación de interfaz: si el mapa parece permitir
movimiento celda por celda, el jugador esperará reglas espaciales que no existen.
La presentación debe comunicar con claridad que la navegación real sucede entre
lugares o contextos.

## 7. Acciones, sucesos y simulación

### 7.1 Resolución de acciones

Toda acción sigue conceptualmente este recorrido:

```text
intención → validación → cálculo → efectos → sucesos → avance temporal
```

- **DG-SIM-001 — Decidido:** una acción inválida no altera parcialmente el mundo.
- **DG-SIM-002 — Decidido:** la aleatoriedad usada al resolver una acción proviene
  del estado controlado de la campaña, no de una fuente global invisible.
- **DG-SIM-003 — Decidido:** el resultado contiene información suficiente para
  que cualquier frontend pueda describir qué ocurrió sin reimplementar reglas.
- **DG-SIM-004 — Provisional:** los actores no jugadores reutilizan el mismo
  catálogo de acciones cuando las capacidades coinciden; sus controladores eligen
  intenciones, pero no obtienen permisos especiales implícitos.
- **DG-SIM-005 — Decidido:** los cambios importantes producen una traza causal
  apta para el registro de juego, depuración y epílogo.

### 7.2 Actividad del mundo

El mundo debe dar la impresión de tener iniciativa propia, pero no es requisito
simular cada actor con el mismo detalle en todo momento.

- Los actores persiguen objetivos o necesidades simples y reaccionan a sucesos.
- Los conflictos y oportunidades pueden evolucionar sin intervención del
  jugador.
- El paso del tiempo puede activar viajes, recuperación, escasez, rumores,
  cambios de relación o etapas de la crisis.
- La frecuencia y nivel de detalle de la simulación fuera del lugar actual quedan
  abiertos para el diseño técnico, siempre que los resultados sean coherentes y
  reproducibles.

## 8. Contenido editable y generación

### 8.1 Contrato de contenido

- **DG-CON-001 — Decidido:** el contenido del juego base y el contenido de mods
  pasan por la misma carga y validación conceptual.
- **DG-CON-002 — Decidido:** las referencias usan identificadores estables y los
  errores indican archivo, elemento y causa antes de iniciar una campaña.
- **DG-CON-003 — Decidido:** una campaña conserva la versión o huella del juego y
  del conjunto de contenido con el que fue creada.
- **DG-CON-004 — Provisional:** el primer alcance de modding acepta datos y
  parámetros de comportamientos soportados, pero no scripts ni código arbitrario.
- **DG-CON-005 — Provisional:** no hay recarga en caliente durante una campaña.
  Los cambios de archivos afectan la siguiente campaña, no el estado ya creado.
- **DG-CON-006 — Abierto:** el formato, los esquemas, el orden de capas, la
  resolución de conflictos y la política de compatibilidad se decidirán en la
  documentación técnica.

“Fácil de modificar” exige además ejemplos, mensajes de error accionables y una
forma de validar contenido sin tener que jugar una campaña completa.

### 8.2 Generación de una campaña

Pipeline conceptual:

1. Descubrir los paquetes de contenido habilitados.
2. Cargar y validar sintaxis, identificadores, referencias y restricciones.
3. Elegir o aceptar una semilla.
4. Seleccionar una premisa y plantillas compatibles.
5. Crear lugares y sus relaciones lógicas de viaje.
6. Crear actores, objetos, recursos y relaciones.
7. Crear conflictos, oportunidades y una presión de final de campaña.
8. Establecer al personaje del jugador y su contexto inicial.
9. Crear la representación visual.
10. Validar invariantes de jugabilidad; reparar o regenerar si fallan.
11. Guardar la huella de contenido y comenzar la campaña.

- **DG-GEN-001 — Decidido:** la misma semilla solo promete el mismo mundo cuando
  también coinciden la versión del juego y la huella del contenido.
- **DG-GEN-002 — Decidido:** el generador combina azar con restricciones; una
  selección puramente aleatoria no se considera generación suficiente.
- **DG-GEN-003 — Decidido:** un fallo de validación nunca entrega silenciosamente
  al jugador un mundo imposible; debe reparar, reintentar o abortar con un error.
- **DG-GEN-004 — Decidido:** la semilla y la información necesaria para reproducir
  una campaña forman parte de su registro.

### 8.3 Invariantes mínimos de una campaña válida

- El actor del jugador existe y tiene una situación inicial válida.
- Todas las referencias requeridas resuelven a elementos existentes y compatibles.
- Existe más de una acción inicial razonable.
- Los lugares necesarios para el recorrido lógico son alcanzables mediante rutas.
- La presión global tiene señales observables y al menos una respuesta viable.
- Ningún actor vivo ocupa estados incompatibles a la vez.
- Existe alguna vía hacia un desenlace; la campaña no depende de continuar para
  siempre.
- La representación visual tiene dimensiones y tokens válidos, pero un problema
  visual no redefine las reglas del mundo.

## 9. Permadeath, persistencia y epílogo

- **DG-PER-001 — Decidido:** se guarda para suspender y continuar, no para volver
  voluntariamente a un estado anterior y deshacer consecuencias.
- **DG-PER-002 — Decidido:** al morir, la campaña queda terminada y ya no puede
  reanudarse como partida activa.
- **DG-PER-003 — Decidido:** debe existir una estrategia de recuperación frente a
  cierres inesperados que no convierta el guardado en rebobinado normal.
- **DG-PER-004 — Decidido:** una campaña terminada conserva al menos semilla,
  duración, causa del desenlace y un resumen de hechos relevantes.
- **DG-PER-005 — Provisional:** cualquier metaprogresión futura desbloquea variedad,
  opciones o conocimiento, no aumentos permanentes fuertes de poder.
- **DG-PER-006 — Decidido:** al ser un juego local y modificable, el objetivo es
  preservar la intención del permadeath en el flujo normal; no se promete impedir
  toda manipulación deliberada de archivos por parte del usuario.

## 10. Frontend de terminal y portabilidad

### 10.1 Responsabilidades del frontend

El frontend de terminal:

- transforma entrada de teclado en comandos semánticos;
- consulta una vista pública del estado permitido al jugador;
- representa mapa, contexto del lugar, actores, estado personal y registro;
- presenta opciones, costes conocidos, errores y resultados;
- gestiona composición, foco, redimensionado, color y glifos;
- nunca decide por su cuenta si una acción es válida o cuál es su resultado.

### 10.2 Requisitos de interacción

- **DG-UI-001 — Decidido:** ninguna información imprescindible depende solamente
  del color.
- **DG-UI-002 — Decidido:** el jugador puede identificar el lugar actual, los
  actores accesibles, su estado relevante, la presión global conocida y las
  acciones disponibles.
- **DG-UI-003 — Decidido:** una orden rechazada explica el motivo sin cambiar el
  estado de la campaña.
- **DG-UI-004 — Provisional:** la interfaz se adapta a un tamaño mínimo de terminal
  todavía por definir y degrada paneles secundarios antes de ocultar información
  imprescindible.
- **DG-UI-005 — Decidido:** las reglas no reciben glifos, códigos ANSI, eventos de
  teclado ni coordenadas de pantalla.

### 10.3 Prueba de portabilidad

- **DG-ARQ-001 — Decidido:** el núcleo completo puede ejecutarse de forma headless
  mediante pruebas o un controlador mínimo que envía comandos y lee resultados.
- **DG-ARQ-002 — Decidido:** sustituir la terminal por otro frontend no requiere
  reescribir generación, acciones, IA, persistencia ni reglas del mundo.
- **DG-ARQ-003 — Decidido:** el núcleo no depende de rutas de archivos, reloj del
  sistema o aleatoriedad ambiental sin que esas dependencias entren mediante un
  límite explícito.

La documentación técnica determinará cómo materializar estos límites en Rust.

## 11. Alcance inicial

El primer hito jugable debe ser una sección vertical completa y corta. Intentar
producir inmediatamente seis horas de contenido haría demasiado lenta la
validación del bucle y del permadeath.

| Área | Sección vertical / MVP | Objetivo de producto |
|---|---|---|
| Duración por campaña | 30–90 minutos para validar el ciclo | 5–6 horas |
| Mundo | Una región y una premisa | Varias combinaciones dentro de una campaña coherente |
| Lugares | 4–8 lugares lógicos | Cantidad ajustada por ritmo; referencia inicial de 12–20 |
| Actores | Jugador y 8–15 actores simples | Densidad ajustada; referencia inicial de 15–30 |
| Grupos | 2 intereses enfrentados | Al menos 3 fuerzas con relaciones variables |
| Acciones | Observar, viajar, interactuar, tomar/usar, descansar y resolver un conflicto | Verbos sociales, sigilo, comercio y conflicto suficientemente expresivos |
| Arco | Una presión global y varios desenlaces | Campaña completa con desarrollo y crisis |
| Contenido | Archivos externos para lugares, actores y objetos | Cobertura de todos los elementos configurables acordados |
| Presentación | Frontend ASCII jugable | Experiencia de terminal completa y legible |
| Persistencia | Suspender, reanudar y cerrar por muerte | Historial y epílogo completos |

Las cantidades son referencias provisionales; importan más la densidad de
decisiones y el ritmo que alcanzar un número fijo.

### 11.1 Fuera del MVP

- Multijugador o servicios en línea.
- Tiempo real.
- Movimiento o combate táctico gobernado por tiles.
- Mundo infinito o una campaña persistente sin final.
- Scripting o ejecución de código aportado por mods.
- Editor visual de contenido.
- Economía, política o planificación de IA de gran profundidad.
- Múltiples frontends de producción; basta demostrar la independencia del núcleo.
- Compatibilidad indefinida con todos los guardados o mods futuros.
- Metaprogresión de poder entre campañas.

## 12. Criterios de éxito del diseño

- **DG-ACE-001:** cambiar un archivo válido de contenido cambia una campaña nueva
  sin recompilar el juego.
- **DG-ACE-002:** una referencia de contenido inválida produce un diagnóstico
  claro antes de generar el mundo.
- **DG-ACE-003:** con versión, huella de contenido, semilla y secuencia de comandos
  iguales, la simulación produce el mismo resultado salvo excepciones documentadas.
- **DG-ACE-004:** ninguna regla decide en función de un carácter ASCII o tile.
- **DG-ACE-005:** el núcleo puede completar campañas simuladas sin frontend.
- **DG-ACE-006:** una campaña puede suspenderse sin ofrecer rebobinado como parte
  del flujo normal.
- **DG-ACE-007:** el mundo cambia sin intervención directa del jugador.
- **DG-ACE-008:** toda campaña válida puede llegar a uno o más desenlaces.
- **DG-ACE-009:** los peligros capaces de terminar una campaña tienen señales o
  contramedidas descubribles.
- **DG-ACE-010:** el epílogo identifica las cadenas causales más importantes en
  vez de mostrar solo una etiqueta de victoria o derrota.

## 13. Riesgos de diseño

| Riesgo | Consecuencia | Respuesta de diseño |
|---|---|---|
| Sandbox sin cierre | Campañas mucho más largas o sin propósito | Presión global, horizonte finito y varios desenlaces. |
| Muerte arbitraria tras varias horas | Frustración y pérdida de confianza | Señales, vías de escape, registro causal y azar controlado. |
| Mapa visual engañoso | El jugador espera movimiento espacial inexistente | Separar visualmente navegación lógica de decoración por tiles. |
| Azar sin estructura | Mundos absurdos, inertes o imposibles | Generación por fases, restricciones, validación y reparación. |
| Demasiados actores simulados | Rendimiento pobre y comportamiento difícil de explicar | Niveles de detalle o frecuencias distintas, preservando coherencia. |
| Contenido y runtime mezclados | Mods frágiles y guardados inconsistentes | Definiciones separadas de instancias y huella por campaña. |
| Reglas filtradas al frontend | Portar el juego obliga a reescribirlo | Comandos semánticos, resultados y ejecución headless. |
| Alcance de modding excesivo | Retraso, inseguridad y APIs inestables | Datos validados primero; scripting fuera del MVP. |
| Variación superficial | Las campañas se sienten iguales | Variar relaciones, objetivos, presiones y recursos, no solo nombres. |

## 14. Decisiones abiertas prioritarias

Estas preguntas no bloquean el documento base. Las decisiones provisionales
permiten avanzar, pero deberán resolverse antes del plan que dependa de ellas.

| ID | Pregunta |
|---|---|
| DG-ABI-001 | ¿Cuál es la ambientación y la fantasía concreta del personaje? |
| DG-ABI-002 | ¿Qué ambiciones equivalen a ganar y cómo se presenta la retirada? |
| DG-ABI-003 | ¿Qué forma toma la presión global que limita una campaña? |
| DG-ABI-004 | ¿El jugador controla siempre un solo actor? |
| DG-ABI-005 | ¿Qué muestra exactamente el mapa: mundo, región, lugar actual o una composición de escalas? |
| DG-ABI-006 | ¿Cuál es el modelo de combate y cuánto depende de posición lógica? |
| DG-ABI-007 | ¿Qué información del mundo se conoce al comenzar y cómo se descubre el resto? |
| DG-ABI-008 | ¿Qué profundidad de objetivos, memoria y planificación necesitan los actores? |
| DG-ABI-009 | ¿Qué simulación continúa fuera del lugar actual? |
| DG-ABI-010 | ¿Cuál es la unidad temporal y el orden exacto de resolución? |
| DG-ABI-011 | ¿Qué compatibilidad se promete a contenido y guardados entre versiones? |
| DG-ABI-012 | ¿Cómo se ordenan mods y cómo se resuelven conflictos? |
| DG-ABI-013 | ¿Existe metaprogresión de variedad o conocimiento? |
| DG-ABI-014 | ¿Qué tamaño mínimo de terminal y modos de entrada se soportan? |

## 15. Glosario

- **Campaña / partida:** mundo autocontenido desde su generación hasta un
  desenlace; puede abarcar varias sesiones reales.
- **Sesión:** intervalo real durante el que la aplicación permanece abierta.
- **Contenido:** definiciones editables que describen posibilidades del juego.
- **Definición:** plantilla inmutable leída del contenido.
- **Instancia:** elemento concreto y mutable creado para una campaña.
- **Mundo:** raíz autoritativa del estado de una campaña.
- **Lugar:** contexto lógico de presencia e interacción, independiente de tiles.
- **Actor:** entidad capaz de proponer acciones.
- **Acción:** intención validada y resuelta por las reglas.
- **Efecto:** cambio de estado producido al resolver una acción o suceso.
- **Suceso:** hecho registrado o programado capaz de informar y provocar reacciones.
- **Mapa visual:** matriz no autoritativa destinada a representación.
- **Tile:** celda de esa matriz visual; no equivale a un lugar.
- **Frontend:** adaptador de entrada y presentación; inicialmente, la terminal.
- **Núcleo:** lógica de generación, reglas y simulación independiente del frontend.
- **Semilla:** dato inicial de la aleatoriedad reproducible.
- **Huella de contenido:** identificación del conjunto y versión de definiciones
  usados para crear una campaña.
- **Permadeath:** cierre irreversible de la campaña activa cuando muere el actor
  del jugador.

## 16. Uso de este documento

La documentación técnica deberá citar los requisitos `DG-*` relevantes y explicar
cómo los satisface. Si una decisión técnica contradice este documento, primero se
actualiza o discute el diseño general; no se oculta la diferencia dentro de un
plan de implementación.

Los planes pueden dividir el trabajo, pero no crear requisitos de producto nuevos
sin incorporarlos aquí. Las preguntas `DG-ABI-*` resueltas deben convertirse en
decisiones dentro de la sección correspondiente, conservando un breve registro
del cambio.
