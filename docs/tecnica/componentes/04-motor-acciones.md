# Motor de acciones y protocolo semántico

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.2 |
| Fecha | 2026-10-12 |

## 1. Propósito

Este documento diseña el contrato del motor que ejecutará las acciones generales
y particulares del [diseño general](../../diseno-general.md). Detalla la frontera
interna con el frontend sin decidir las reglas RPG de cada acción.

Desarrolla la separación establecida por el
[diseño técnico general](../diseno-tecnico-general.md): el núcleo es la autoridad
sobre las reglas, la aplicación posee la sesión activa y el frontend solo envía
intenciones y presenta resultados.

## 2. Alcance y exclusiones

Este componente comprende:

- las solicitudes semánticas para iniciar o continuar acciones;
- el estado reanudable de una acción general;
- la ejecución de acciones particulares reutilizables;
- decisiones, fragmentos narrativos, resultados y duración acumulada;
- validación común para acciones del jugador y de los NPC;
- acceso controlado al azar.

Quedan fuera el contenido y cálculo de cada acción, la representación exacta del
tiempo, la selección visual, Lua y el dibujo de viñetas. Tampoco se definen
cancelación, deshacer ni ejecución simultánea.

## 3. Responsabilidades y límites

`desert-tiger-core` define las reglas y el estado opaco de cada acción. Valida
las solicitudes contra el Mundo real, ejecuta transiciones, aplica consecuencias
y produce salidas neutrales. Las acciones particulares también viven en el
núcleo y solo pueden cambiar el estado mediante sus operaciones regladas.

La sesión de `desert-tiger-app` posee como máximo una acción general activa. Si
el núcleo queda esperando una decisión, la sesión conserva su estado reanudable
hasta recibir una respuesta válida. El frontend no recibe ni reconstruye ese
estado interno. Ese estado es un valor poseído formado por datos e IDs; nunca
mantiene referencias prestadas al Mundo o a la fuente de azar entre llamadas.

La fachada pública de `desert-tiger-app` no acepta un actor elegido por el
frontend: vincula el comando al actor jugador de la sesión. El coordinador de NPC
forma una solicitud interna vinculada al actor dueño de la política Lua. Ambas
se convierten en la misma solicitud del núcleo y recorren idénticas reglas.

El frontend mantiene únicamente su estado de interfaz efímero.
`desert-tiger-app` posee el estado actual del mapa; el frontend no lo modifica,
no valida acciones, no altera resultados ni convierte teclas o tiles directamente
en mutaciones del Mundo.

## 4. Datos, entradas y salidas

Los nombres siguientes son conceptos del contrato, no firmas Rust definitivas.

La aplicación ofrece al frontend acciones y objetivos disponibles mediante IDs.
Esa información sirve para presentar opciones, pero no reemplaza la validación
al confirmar una solicitud.

Una **solicitud de acción** contiene al menos:

- el identificador del actor que actuará;
- el identificador de la acción general solicitada;
- los identificadores de objetivos o parámetros semánticos que esa acción
  requiera.

No contiene teclas, colores, posiciones de widgets ni coordenadas de tiles. El
jugador y un NPC usan esta misma forma de solicitud.

Una **respuesta a decisión** contiene el identificador de la decisión pendiente
y el identificador de una de sus opciones. Cada solicitud de decisión producida
por el motor contiene:

- un identificador válido solamente para esa espera;
- texto o información narrativa neutral;
- una lista no vacía de opciones, cada una con identificador y texto neutral.

Las salidas forman una secuencia ordenada y pueden incluir:

- **fragmentos narrativos**, con el texto y los resultados que el frontend puede
  convertir en viñetas;
- **resultados semánticos estructurados**, que identifican qué cambió en el
  dominio sin incluir instrucciones, coordenadas ni tiles de presentación;
- una **solicitud de decisión**, que deja la acción pendiente;
- una **finalización**, con el resultado y la duración total acumulada;
- un **rechazo**, con una causa comprensible y sin fingir que la acción ocurrió.

Los fragmentos no contienen estilos, coordenadas ni tipos de Ratatui. El
frontend decide cómo presentarlos y conserva su orden para la acción del jugador.
En la primera versión, el código Rust de cada acción produce sus propios textos;
Lua y el frontend no redactan ni sustituyen esa lógica narrativa.

Cuando se diseñe esta capacidad, la aplicación podrá usar un resultado semántico
para actualizar el mapa de la Partida; por ejemplo, para reflejar visualmente que
una zona se quemó. Esa proyección no forma parte del cálculo de la acción: el
núcleo nunca inspecciona la matriz visual ni emite una orden de dibujo. El
contrato concreto entre cada resultado y su transformación visual queda
pendiente.

## 5. Ejecución y estado reanudable

Al comenzar una acción general, el núcleo vuelve a validar actor, acción,
objetivos y condiciones actuales. Si son válidos, crea su estado de ejecución y
avanza por sus transiciones. Cada transición puede consultar el Mundo, usar el
azar inyectado, invocar acciones particulares, aplicar consecuencias y emitir
fragmentos.

El avance se detiene cuando la acción:

1. necesita una decisión externa, en cuyo caso conserva su estado; o
2. termina y entrega la duración acumulada.

Una respuesta válida reanuda exactamente esa acción. Una respuesta no puede
seleccionar una opción de otra decisión ni reutilizar una decisión ya resuelta.
Mientras existe una acción pendiente no puede iniciarse otra en la misma sesión.

Cada transición pública de inicio o reanudación valida y prepara su resultado
antes de confirmarlo. Un rechazo esperado deja intactos Mundo, acción pendiente
y azar. Si un fallo interno rompe esa garantía después de confirmar cambios, la
sesión se considera fatal y no continúa como si su estado fuera confiable.

Una **acción particular** es una operación de simulación reutilizable. Recibe su
contexto reglado, calcula y aplica un resultado, y puede aportar fragmentos y
tiempo a la acción general. No conoce la presentación ni permite evitar la
validación de quien la invoca.

La duración comienza en cero y solo acepta incrementos válidos expresados por el
tipo de tiempo del mundo. Las acciones particulares pueden modificar el total
durante la ejecución; el motor lo comunica al exterior únicamente como duración
final de la acción completa. El avance de la fecha y el tratamiento especial de
la duración de NPC se definen en
[`05-tiempo-fase-npc.md`](./05-tiempo-fase-npc.md).

## 6. Azar compartido y reproducible

El motor no crea generadores ocultos ni consulta fuentes globales de azar. Toda
decisión aleatoria recibe una fuente administrada por la sesión a través de una
interfaz sustituible. Acciones generales y particulares consumen esa misma
capacidad sin conocer su implementación.

En pruebas se inyectará una fuente prefijada o programada. Con igual estado,
entradas y respuestas de azar, una transición produce el mismo resultado.

## 7. Contratos e invariantes

- Solo el núcleo valida y aplica consecuencias de gameplay.
- Hay como máximo una acción general activa por sesión.
- Jugador y NPC atraviesan la misma entrada, validaciones y lógica de acción.
- El actor jugador con la característica `muerto` no puede iniciar una acción.
- Una decisión pertenece a una única acción pendiente y solo acepta sus opciones
  vigentes.
- Una respuesta inválida no modifica el Mundo ni descarta la acción pendiente.
- Cada fragmento conserva el orden en que fue emitido por la lógica.
- El tiempo acumulado nunca es negativo ni altera directamente la fecha.
- Ninguna regla consulta Ratatui, teclas, modos visuales o coordenadas del mapa.
- Los tiles no son una entrada ni una fuente de verdad para una acción, aunque
  su resultado pueda causar después una actualización visual.
- Todo azar que afecte reglas entra por la fuente inyectada.

## 8. Errores y validaciones

Son rechazos esperables: referencias inexistentes, actor no autorizado,
condición incumplida, objetivo que dejó de ser válido, otra acción pendiente,
decisión desconocida u opción ajena. Se distinguen de una invariante rota.

Una solicitud rechazada no crea una acción, consume azar ni tiempo. Una respuesta
inválida conserva el punto de espera. Los diagnósticos identifican la causa sin
provocar `panic` en caminos normales. Un fallo interno fatal se propaga a la
aplicación, que detiene la sesión y permite al frontend restaurar la terminal.

## 9. Pruebas y criterios de aceptación

Las pruebas unitarias usarán un Mundo mínimo, una acción de prueba y azar
controlado para comprobar:

- inicio, espera, respuesta y finalización de una acción con varias transiciones;
- suma de duraciones aportadas por más de una acción particular;
- selección reproducible de una rama azarosa;
- rechazo de una opción o identificador de decisión incorrectos sin mutación;
- rechazo esperado sin consumo de azar ni tiempo;
- imposibilidad de iniciar otra acción mientras una está pendiente;
- rechazo de una acción nueva del actor jugador con `muerto`;
- reutilización de una acción particular desde dos contextos generales;
- igualdad del recorrido de validación para solicitudes de jugador y NPC.

Una prueba de integración sin frontend completará una acción mediante la fachada
de aplicación y verificará sus fragmentos ordenados, consecuencias y duración.
El componente será aceptable cuando ninguna dependencia o tipo de terminal sea
necesario para realizar ese recorrido y el frontend no deba recalcular ninguna
regla para presentarlo.

## 10. Preguntas pendientes

- ¿Qué forma exacta tendrán las solicitudes, resultados y errores en Rust?
- ¿Qué catálogo identifica las acciones generales y particulares disponibles?
- ¿Una acción podrá cancelarse mientras espera una decisión y, de ser así, qué
  consecuencias y tiempo conservará?
- ¿Cómo se representará un resultado desfavorable de una acción que sí comenzó
  y, por lo tanto, puede haber consumido tiempo?
- ¿La característica `muerto` impedirá actuar también a todos los NPC o solo se
  aplicará inicialmente al actor jugador como está confirmado?
- ¿Qué forma mínima necesita un resultado semántico para permitir proyecciones
  visuales sin acoplar las acciones a tiles o coordenadas?

