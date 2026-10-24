# Traspaso de contexto: rediseño de `Viajar`

| Campo | Valor |
|---|---|
| Estado | Trabajo de diseño en curso |
| Fecha del traspaso | 2026-10-21 |
| Documento activo | [`05-viajar-redisenio.md`](./05-viajar-redisenio.md) |

## 1. Objetivo activo

Diseñar desde sus fundamentos las mecánicas de la acción general `Viajar` antes
de volver a implementarla.

El resultado debe ser un reglamento suficientemente preciso para convertir sus
reglas en código sin tener que inventar comportamiento durante el desarrollo.
Todavía no corresponde escribir código ni planes de implementación.

## 2. Fuentes y autoridad

Para continuar el trabajo, leer primero y en este orden:

1. este archivo de traspaso;
2. [`05-viajar-redisenio.md`](./05-viajar-redisenio.md).

El borrador nuevo es la fuente vigente para este rediseño. El código actual y
los documentos técnicos `01` a `04` describen versiones anteriores. Pueden
consultarse como antecedentes cuando resulte útil, pero no poseen autoridad
para completar reglas pendientes ni limitar el nuevo diseño.

No es necesario leer toda la documentación o todo el código al comenzar la
nueva conversación. Deben abrirse otras fuentes solamente cuando una cuestión
concreta lo requiera.

## 3. Forma de trabajo acordada

- La persona usuaria aporta las decisiones y objetivos de diseño del juego.
- El asistente los organiza, detecta contradicciones o ambigüedades y ayuda a
  convertirlos en reglas completas.
- Las preguntas dirigidas a la persona usuaria deben ser cuestiones de diseño,
  no decisiones internas de Rust.
- Cuando una decisión de diseño tenga consecuencias técnicas relevantes, el
  asistente debe explicarlas en lenguaje común antes de pedir una elección.
- Los detalles técnicos menores pueden resolverse posteriormente sin interrumpir
  el diseño.
- No deben inventarse reglas para rellenar silenciosamente un vacío. Los vacíos
  se registran como **PENDIENTE** hasta que sean discutidos.
- Después de cada aclaración, se actualiza el borrador para conservar la decisión
  fuera del historial de la conversación.

## 4. Estilo obligatorio del documento

El documento debe escribirse como el reglamento de un juego de rol de mesa:
reglas concretas, orden de resolución, entradas, chequeos, resultados y cambios
de estado.

No debe limitarse a describir qué «podría» hacer el sistema ni enumerar
posibilidades futuras. Cuando todavía no exista una regla, se marca como
**PENDIENTE**.

El reglamento se organiza en dos niveles:

1. **Lógica de la acción general:** compone el viaje completo, conserva su
   estado transitorio, ordena las viñetas y mantiene la simulación.
2. **Tarjetas de viñeta:** cada tarjeta tiene una clave estable y especifica su
   disparador, entradas, acción particular, responsable, resolución, resultado
   mecánico, presentación y transición.

La lógica general debe referirse a las tarjetas por sus claves en vez de repetir
sus reglas.

## 5. Decisiones confirmadas

### 5.1. Desplazamiento

- Una ruta es una sucesión de Lugares relacionados directamente.
- Cada conexión recorrida constituye un tramo y exige su propio intento.
- No existe un límite artificial de escenas, tramos o chequeos por viaje.
- El medio determina la acción particular:
  - a pie: `Andar`;
  - a caballo: `Cabalgar`;
  - en camello: `CabalgarEnCamello`;
  - por mar: `NavegarEnBarco`.
- Para la tarjeta de desplazamiento de `Viajar`, todo el Grupo se mueve junto y
  `Viajar` elige como líder al integrante con mejor capacidad aplicable.
- Esta selección pertenece al contexto de esa tarjeta. No pertenece a la acción
  particular ni es una regla universal para otros contextos.
- La acción particular recibe al responsable ya seleccionado, resuelve su
  chequeo y devuelve resultado y duración.
- Cada intento de desplazamiento se presenta mediante una viñeta automática que
  el jugador cierra con `Enter`.

### 5.2. Contexto y acciones particulares

- La acción general conoce el propósito y la situación humana completa.
- La acción general decide qué acción particular invoca, quién es responsable y
  cómo interpreta el resultado.
- Una acción particular resuelve un intento concreto, pero no conoce la historia
  que lo motiva. La misma acción puede reutilizarse en contextos narrativos
  diferentes.

### 5.3. Tiempo y alimento

- El aprovisionamiento funciona durante todo el viaje, no solo al comienzo.
- Toda acción particular que hace pasar tiempo provoca consumo de alimento para
  todos los integrantes.
- Duración mostrada, consumo y avance final de la fecha deben proceder de una
  única duración acumulada.
- Primero se resuelve un intento y se actualizan estado, tiempo y alimento;
  después se construye la viñeta con esos mismos valores.
- El aprovisionamiento inicial tiene su propia tarjeta.
- Si las reservas resultan insuficientes durante el viaje, se ejecuta una
  tarjeta para buscar alimento.
- Toda la búsqueda de alimento y sus chequeos se presentan en una sola viñeta.

### 5.4. Medios de transporte

No se diseñará todavía una tarjeta para conseguir caballos, camellos o barcos.
Normalmente su obtención dependerá de Lugares, recursos y situaciones que aún no
están diseñados. Este tema queda expresamente fuera del borrador actual.

## 6. Tarjetas existentes

El borrador contiene actualmente:

| Clave | Tarjeta |
|---|---|
| `VIA-DES-01` | Desplazamiento entre dos Lugares conectados. |
| `VIA-APR-01` | Aprovisionamiento inicial. |
| `VIA-APR-02` | Búsqueda de alimento durante el viaje. |

Las reglas sin resolver están enumeradas en la sección **Reglas todavía
pendientes** del borrador. Esa lista debe mantenerse actualizada en vez de
duplicarse aquí.

## 7. Observaciones sobre la implementación anterior

Durante la última prueba manual se observaron dos problemas:

- la interfaz indicó tres días transcurridos, pero la fecha final avanzó solo un
  día;
- después de varios intentos no fue posible alcanzar el mar.

No corresponde investigar ni reparar todavía esos defectos. Motivaron el
rediseño, pero la prioridad actual es completar primero el reglamento.

## 8. Límites de esta etapa

Mientras el reglamento no haya sido aprobado:

- no modificar el código;
- no crear planes de implementación;
- no sustituir los documentos técnicos anteriores;
- no declarar definitivo el borrador;
- no completar puntos pendientes a partir del comportamiento actual;
- no ampliar el alcance hacia otras acciones generales.

Cuando el reglamento esté completo, se revisará su coherencia y recién entonces
se decidirá qué documentación reemplaza, qué código se conserva y qué debe
refactorizarse.

## 9. Mensaje recomendado para iniciar la conversación nueva

```text
Quiero continuar el rediseño de la acción general Viajar de este proyecto.

Antes de responder, leé:

- docs/tecnica/acciones/borradores/CONTEXTO-viajar-redisenio.md
- docs/tecnica/acciones/borradores/05-viajar-redisenio.md

El borrador nuevo es la fuente vigente para esta tarea. El código implementado
y los documentos anteriores describen el diseño viejo y no deben usarse para
completar reglas faltantes.

Todavía estamos diseñando: no modifiques código, planes ni documentos técnicos
anteriores. Ayudame a convertir mis instrucciones en reglas mecánicas precisas,
redactadas como un reglamento de RPG de mesa. Las escenas deben especificarse
mediante tarjetas de viñeta. No rellenes puntos PENDIENTES sin consultarme.

Explicame en lenguaje común cualquier consecuencia técnica relevante, porque
puedo decidir cuestiones de diseño pero no cuestiones específicas de Rust.

Confirmá brevemente que entendiste el estado actual y esperá mi próxima
instrucción de diseño.
```
