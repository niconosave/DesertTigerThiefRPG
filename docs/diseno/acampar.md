# Acampar — Diseño de la acción general

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.3 |
| Fecha | 2026-10-26 |

## 1. Propósito y alcance

Acampar permite detenerse a conocer al actor jugador y a sus compañeros:
consultar sus fichas, comprender sus características y explorar las relaciones
que los conectan con otros elementos del mundo. La navegación funciona como los
enlaces de una enciclopedia: desde una ficha se puede abrir otra y continuar
explorando sus referencias.

Esta primera versión comprende fichas de Actores y Lugares. Otros elementos,
como objetos, se incorporarán cuando existan en el juego. El documento define
el comportamiento para el jugador; el siguiente documento técnico fijará
estructuras, archivos, contratos entre capas y controles de calidad. No es un
plan de implementación.

## 2. Entrada y presentación

El jugador elige la acción general **Acampar** desde la vista del mundo. La
pantalla cambia a la vista de campamento:

- el mapa deja de dibujarse y su región pasa a mostrar una ficha navegable;
- los retratos del jugador y sus compañeros permanecen en las posiciones y el
  orden visual que ya tienen, con el jugador a la derecha;
- una fogata dibujada con caracteres ASCII y Unicode aparece justo a la derecha
  del conjunto de retratos;
- la barra contextual explica qué se está consultando o seleccionando;
- la barra de acciones comunica los controles disponibles.

La ficha se dispone de forma que los retratos y la fogata no impidan leer la
información seleccionada. Sus dimensiones, distribución y arte concreto se
resolverán en el diseño técnico y en los recursos visuales del frontend. No se
requiere animar la fogata en este alcance.

## 3. Información de las fichas

### Actor

La ficha muestra el nombre, una biografía breve, las características actuales y
las relaciones del Actor. Incluye tanto al jugador como a sus compañeros y a
los Actores externos que se alcancen mediante referencias.

La biografía se escribe dentro del mismo archivo RON del Actor. Describe al
personaje; no sustituye ni determina sus características o relaciones. La ficha
muestra el estado actual de la partida, incluidos cambios adquiridos al jugar.

### Lugar

Al seguir una referencia a un Lugar se abre su ficha con nombre,
características y relaciones. No se exige una biografía de Lugar para esta
versión. Desde allí también se pueden seguir referencias hacia otros elementos.

### Aclaraciones de características

Habrá un catálogo editable que asocia cada característica con una pequeña
explicación de su significado. Las fichas usan ese catálogo para explicar la
característica enfocada, tanto en Actores como en Lugares.

Las explicaciones no crean reglas: describen las que ya tiene el juego. Tampoco
se añaden estadísticas numéricas, habilidades ni inventarios para completar las
fichas. El formato y la ubicación del catálogo corresponden al documento técnico.

### Relaciones y resumen de referencias

Cada entrada permite reconocer el tipo de relación y el elemento al que conduce.
Se muestran las relaciones incidentes desde ambos extremos, incluidas las
entrantes. Las relaciones dirigidas deben conservar su sentido: consultar una
relación desde uno de sus extremos no cambia quién conoce o se relaciona con quién.
Las simétricas, como Conectado, se pueden explorar desde ambos extremos.
Por ejemplo, desde un Actor aparece «Conoce [Lugar]» y desde ese Lugar,
«Conocido por [Actor]». Ambos permiten abrir el otro elemento. Esto permite
alcanzar Actores externos con las relaciones que ya existen.

Cuando una relación está enfocada, un apartado muestra un resumen breve del
elemento referido. Para un Actor permite reconocerlo por su nombre y biografía;
para un Lugar, por su nombre y características. El resumen no reemplaza la ficha
completa ni obliga a abrirla.

La consulta expone las relaciones del elemento que se está viendo. No se añade
un sistema de secretos o restricciones de conocimiento en este incremento.

## 4. Los dos focos de navegación

### Selección horizontal: integrantes y salida

Las flechas izquierda y derecha recorren a los Actores del grupo, incluyendo
al actor jugador como opción consultable, y una opción **Abandonar campamento**.
Incluir al jugador en este selector no modifica el modelo del Grupo.

Al entrar se abre la ficha del jugador. El recorrido horizontal es circular:
compañeros en su orden visual, jugador y Abandonar campamento. Al seleccionar
un Actor se muestra su ficha. El foco identifica claramente cuál se está
consultando sin desplazar los retratos. La salida también debe resultar
visible y distinguible de un Actor.

### Selección vertical: información de la ficha

Las flechas arriba y abajo recorren primero las características y después las
relaciones de la ficha abierta; se detienen en los extremos. El nombre y la
biografía sirven de contexto de lectura; no son enlaces. El foco vertical
distingue la entrada seleccionada:

- una característica muestra su aclaración del catálogo;
- una relación muestra el resumen de su referencia;
- Enter sobre una relación abre la ficha del otro elemento;
- Enter sobre una característica no abre otra ficha.

La navegación debe permitir leer todas las entradas y el texto completo de
biografías, aclaraciones y resúmenes, aunque no quepan juntos en la terminal.
El desplazamiento y paginado se concretan en el documento técnico.

### Seguir referencias

Abrir un elemento por una relación cambia la ficha consultada. No traslada al
grupo, no recluta al Actor referido y no lo incorpora al selector horizontal.
Tampoco reemplaza los retratos del campamento por el retrato del elemento abierto.

Desde la nueva ficha se puede seguir otra relación, y repetir el proceso sin un
límite de saltos impuesto por el diseño. Una referencia de vuelta a un elemento
ya consultado vuelve a mostrar su ficha; los ciclos no bloquean la exploración.

Aunque se esté viendo una ficha externa al grupo, izquierda o derecha vuelve a
seleccionar la opción anterior o siguiente desde la última selección horizontal;
el elemento externo no se inserta en el selector. El foco vertical pasa
a corresponder a la ficha que se acaba de abrir, sin conservar una selección
que pertenezca a otro elemento.

Cada ficha nueva enfoca su primera entrada disponible. Una ficha vacía muestra
su información de contexto y permite cambiar de Actor o salir. Si falta la
aclaración de una característica, conserva su nombre e indica que no hay una
descripción disponible. No se incorpora un historial de navegación.

## 5. Salida y paso del tiempo

Seleccionar Abandonar campamento muestra «Abandonar el campamento hará pasar
un día». Enter confirma la salida; pasar por esa opción con una flecha no la
ejecuta. Explorar fichas y relaciones no hace avanzar el tiempo. Abandonar el
campamento completa la acción general y hace transcurrir **un día**, independientemente de
cuántas fichas se hayan abierto o cuánto tiempo real haya durado la consulta.
El grupo permanece en el mismo Lugar.

Después se aplica el ciclo general de la partida: se actualiza la fecha, actúan
los NPC cuando corresponde y se vuelve a la vista del mundo con el estado
resultante. No hay un coste por ficha ni un día adicional por relación seguida.

Acampar no recupera heridas, elimina estados ni concede otras ventajas en esta
primera versión. Las futuras reglas de descanso requerirán su propio diseño.
Tampoco se añaden provisiones, chequeos de supervivencia, encuentros ni decisiones
de seguridad del campamento como parte de este alcance.

## 6. Criterios de aceptación

- Consultar al jugador y sus compañeros, y seguir referencias entre fichas de
  Actores y Lugares, incluidos elementos externos y recorridos cíclicos.
- Leer toda la información sin que los retratos o la fogata la oculten.
- Mantener los focos distinguibles y poder salir también desde una ficha vacía.
- Consumir un solo día al confirmar la salida y retomar el ciclo habitual.

Su traducción a código está en el
[diseño técnico de Acampar](../tecnica/acciones/07-acampar.md), siguiendo la
[metodología del proyecto](../metodologia-desarrollo.md). No quedan decisiones
de juego pendientes dentro de este alcance.
