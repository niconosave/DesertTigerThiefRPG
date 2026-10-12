# Tiempo del mundo y fase de NPC

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.1 |
| Fecha | 2026-10-12 |

## 1. Propósito

Este documento diseña la coordinación entre una acción terminada por el jugador,
el avance de la fecha y las acciones posteriores de los NPC. Desarrolla el ciclo
del [diseño general](../../diseno-general.md) bajo los límites del
[diseño técnico general](../diseno-tecnico-general.md).

Su objetivo es que el tiempo avance una sola vez por acción del jugador y que los
NPC actúen dentro de esa franja ya transcurrida, usando el mismo
[motor de acciones](./04-motor-acciones.md).

## 2. Alcance y exclusiones

Este componente comprende:

- la fecha del Mundo y una duración expresable en días y meses;
- el avance provocado por una acción completa del jugador;
- la fase que consulta políticas Lua y ejecuta solicitudes de NPC;
- la resolución aleatoria de decisiones intermedias de esos NPC;
- los resúmenes neutrales entregados al frontend;
- el regreso a la disponibilidad del jugador.

No define duraciones, días de cada mes, fecha inicial, scripts ni API Lua. También
deja pendientes el orden, la cantidad de acciones y la visibilidad entre NPC.

El tiempo del Mundo es tiempo de gameplay. Es independiente del reloj real usado
por el frontend para entrada y animaciones.

## 3. Responsabilidades y límites

`desert-tiger-core` representa y valida fecha y duraciones, realiza su suma y
ejecuta acciones. El Mundo contiene la única fecha vigente.

`desert-tiger-app` coordina las fases: solicita un único avance, ofrece contexto
de solo lectura a Lua, valida sus respuestas y recopila resúmenes. Mientras
procesa NPC no acepta una acción nueva del jugador.

Lua solo decide si solicita actuar desde el contexto permitido. No muta el Mundo
ni ejecuta acciones: su respuesta se transforma en la misma solicitud semántica
que usaría el jugador.

El frontend presenta los resúmenes; no recorre scripts ni calcula fechas.

## 4. Datos, entradas y salidas

La **fecha del Mundo** representa año, uno de doce meses y un día válido. Los
nombres, numeración y longitudes de los meses no están definidos.

Una **duración de mundo** acumula meses y días no negativos, nunca segundos
reales. La política de calendario definirá su suma y normalización.

La entrada inicial es la finalización válida de una acción del jugador. Durante
la fase NPC también intervienen:

- una vista de solo lectura del Mundo que Rust expone a cada política;
- cero o más solicitudes semánticas devueltas por las políticas Lua;
- una fuente de azar inyectada para resolver decisiones intermedias.

Las salidas son la fecha actualizada, resúmenes ordenados y el fin de fase. Cada
resumen informa qué sucedió sin color, posición ni widgets.

## 5. Secuencia de una ronda

La coordinación ocurre después de que el motor completa, no meramente inicia, la
acción del jugador:

1. La aplicación recibe su duración total acumulada.
2. El núcleo calcula y establece la nueva fecha una sola vez.
3. La aplicación entra en fase NPC; la fecha consultable ya es la nueva.
4. Se evalúan las políticas Lua conforme al orden que se defina.
5. Cada solicitud producida se valida y, si se acepta, se ejecuta mediante el
   motor común.
6. Cuando una acción de NPC pide una decisión, el coordinador selecciona al azar
   una de las opciones válidas y continúa hasta que la acción termine.
7. Su duración puede conservarse como resultado, pero no se suma a la fecha.
8. La aplicación crea los resúmenes en el mismo orden en que se consideren
   ejecutadas las acciones y termina la fase.
9. El jugador puede volver a actuar si las reglas lo permiten; con `muerto`, no.

Los fragmentos de NPC no entran al flujo de viñetas del jugador. La aplicación
entrega un resumen que el frontend presenta en la barra inferior.

## 6. Decisiones de NPC y Lua

Las políticas Lua aportan intenciones, no otra implementación de las acciones.
Rust prepara un contexto permitido con la fecha nueva y las consultas que defina
el documento de Lua. El script devuelve ninguna intención o una solicitud.

El motor valida actor, acción, objetivos y condiciones aunque la solicitud venga
de Lua. Las decisiones usan las opciones emitidas por el motor; la selección
inicial es azarosa y consume la fuente inyectada por la sesión.

La fase debe ser finita. Antes de aprobar el documento se definirá cómo impedir
que scripts o acciones encadenadas retengan indefinidamente el control, sin
suponer todavía una cantidad concreta.

## 7. Contratos e invariantes

- El Mundo usa un calendario de exactamente doce meses por año.
- La fecha tiene una única fuente de verdad en el estado reglado.
- Solo una acción completada del jugador inicia el avance y la fase NPC.
- Toda su duración acumulada se aplica mediante un único avance de fecha.
- La fase NPC comienza después del avance y consulta la fecha resultante.
- Ninguna duración de una acción NPC modifica la fecha del Mundo.
- Un NPC y el jugador usan el mismo motor y las mismas validaciones.
- Lua solo consulta el contexto permitido y solicita; nunca muta el Mundo.
- Las decisiones intermedias de NPC se resuelven al azar entre opciones vigentes.
- El azar es inyectable y reproducible en pruebas.
- El jugador no puede iniciar otra acción hasta que finaliza la fase NPC.
- Los resúmenes no contienen instrucciones propias de Ratatui ni reglas nuevas.

## 8. Errores y validaciones

Fecha y duración deben ser válidas; esta última no puede ser negativa ni
desbordarse. Una suma inválida no inicia la fase NPC ni cambia parcialmente la
fecha. Si el fallo se detecta después de que la acción del jugador ya confirmó
consecuencias, la sesión pasa a estado fatal y no vuelve a aceptar acciones; no
se presenta como un rechazo normal ni se continúa entre fases.

Las solicitudes Lua atraviesan las validaciones normales. Referencias inválidas
producen un error estructurado, nunca una mutación directa. La política ante un
error Lua se define exclusivamente en
[`06-integracion-lua.md`](./06-integracion-lua.md); esta fase aplicará el resultado
de esa política sin inventar otro tratamiento.

Un fallo interno fatal del motor detiene la sesión según su contrato y nunca se
resume como una acción terminada.

## 9. Pruebas y criterios de aceptación

Las pruebas unitarias e integradas cubrirán:

- validación de meses fuera del rango de doce y de días inválidos cuando se
  defina el calendario;
- suma reproducible de duraciones en días, meses y combinaciones de ambos;
- un solo cambio de fecha después de una acción del jugador con varias acciones
  particulares;
- observación de la fecha nueva por una política Lua de prueba;
- ejecución de una solicitud NPC por el mismo motor usado por el jugador;
- resolución reproducible de una o varias decisiones intermedias;
- ausencia de un segundo avance de fecha aunque la acción NPC informe duración;
- rechazo de una solicitud Lua inválida sin mutación no autorizada;
- orden estable de los resúmenes para una política de recorrido fijada;
- bloqueo de nuevas solicitudes del jugador durante la fase y retorno posterior.

Una integración sin terminal completará
`acción del jugador → avance único → NPC → resúmenes → jugador`. La fecha,
consecuencias y resúmenes deberán comprobarse con azar controlado, sin depender
de la velocidad de la terminal.

## 10. Preguntas pendientes

- ¿Cuántos días tiene cada mes, cómo se nombran y cuál es la fecha inicial?
- ¿Qué cantidades de días o meses consume cada acción o acción particular?
- ¿Se permiten duraciones de cero y cómo se comportaría entonces la fase NPC?
- ¿En qué orden se evalúan los NPC?
- ¿Cada política observa las consecuencias de NPC anteriores o una misma
  instantánea tomada al comenzar la fase?
- ¿Cuántas solicitudes puede producir cada NPC y cuál es el máximo total por
  fase?
- ¿Qué NPC y acciones merecen un resumen visible y cómo se agrupan?
- ¿Necesitan los scripts conocer también la fecha anterior o el intervalo
  transcurrido, además de la fecha nueva?

