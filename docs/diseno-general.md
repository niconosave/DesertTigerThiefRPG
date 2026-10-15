# Desert Tiger Thief RPG — Diseño general del juego

| Campo | Valor |
|---|---|
| Estado | Base inicial aprobada |
| Versión | 0.8 |
| Fecha | 2026-10-15 |

## 1. Propósito

Este documento define solamente las bases del juego: sus elementos principales,
las relaciones entre ellos y el flujo general de una partida. Cada elemento será
desarrollado posteriormente en documentación técnica propia.

La documentación del proyecto tendrá cuatro niveles:

1. Este diseño general describe qué elementos forman el juego.
2. Un diseño técnico general describirá la organización global del desarrollo.
3. Documentos técnicos específicos diseñarán cada parte del código y de los datos.
4. Los planes de implementación contendrán únicamente pasos de trabajo y
   referencias a esos documentos técnicos.

El desarrollo será incremental. Cada etapa deberá dejar una versión jugable para
poder evaluar el funcionamiento y la calidad antes de añadir nuevos elementos.

En este documento, **primer incremento** nombra exclusivamente el corte vertical
definido en la sección 9. **Versión inicial** nombra la etapa más amplia que se
alcanzará mediante varios incrementos y que todavía no incluirá todos los
sistemas futuros del juego.

## 2. Concepto básico

El juego será un RPG sandbox local, escrito en Rust y jugado en una terminal con
gráficos ASCII. La duración buscada para una partida completa es de cinco o seis
horas y la muerte del actor jugador será permanente.

La primera versión será más limitada: no tendrá guardado, victoria ni derrota.
Si el actor jugador adquiere la característica `muerto`, dejará de poder tomar
acciones.

El mundo y sus elementos se diseñarán en archivos de contenido editables. Al
iniciar una partida, el juego cargará esos datos y creará nuevas instancias de
los elementos. La creación inicial no modificará ni elegirá aleatoriamente sus
datos. Las diferencias entre partidas surgirán de las acciones, las decisiones,
los resultados azarosos y el comportamiento de los NPC.

## 3. Elementos fundamentales

### Partida

Es la ejecución de una instancia del mundo, creada al cargar los archivos de
contenido. Contiene el estado mutable producido durante el juego.

### Mundo

Existe un solo mundo por partida. Es el elemento raíz y contiene o referencia los
lugares, actores, mapa y demás elementos de esa partida.

También mantiene datos globales, entre ellos:

- la fecha actual, expresada con un calendario de doce meses;
- variables del mundo que pueden consultar las acciones y los scripts Lua.

Las variables globales son datos extensibles del contenido, no una enumeración
cerrada dentro del código del juego. Un diseñador podrá añadir una clave nueva y
consultarla desde Lua sin modificar Rust. Los futuros eventos narrativos podrán
cambiar estos valores mediante las reglas del Mundo; las políticas Lua de los
NPC seguirán limitadas a observarlos y solicitar acciones.

### Mapa

Es una matriz de tiles con propósito exclusivamente visual. En su archivo, cada
tile se escribe mediante un carácter ASCII compacto y el mismo Mapa contiene una
leyenda que indica a qué tipo visual corresponde cada carácter. Esos caracteres
son símbolos de edición, no el arte que deba dibujar el frontend.

El frontend recibe los tipos visuales y decide qué carácter, color, sprite o
modelo utiliza para representarlos. La terminal empleará caracteres ASCII, pero
otro frontend podrá mostrar los mismos tipos mediante gráficos diferentes.

El mapa no establece dónde está un actor, qué lugares están conectados ni qué
acciones son posibles.

Cada Partida posee su propia instancia del mapa. Sus tipos de tile podrán
modificarse durante la partida para representar visualmente un cambio que haya
ocurrido en el mundo; por ejemplo, los tiles de un bosque podrán convertirse en
árboles quemados después de un incendio. La transformación automática se
diseñará en un incremento posterior. Esa modificación seguirá siendo visual:
los tiles no pasarán a ser la fuente del estado ni de las reglas que causaron el
cambio.

### Lugar

Es un elemento interactuable que pertenece al mundo. Puede contener actores y
poseer características.

Un lugar incluye información destinada a su presentación:

- un tile central correspondiente a una coordenada del mapa;
- una lista ordenada de coordenadas donde el frontend escribe su nombre.

El frontend toma los caracteres del nombre en orden y dibuja cada uno sobre el
tile indicado por la coordenada de la misma posición. El rótulo es una
superposición visual: no reemplaza ni modifica los tiles almacenados en el mapa.
Cada carácter admitido para estos nombres debe ocupar exactamente una celda de
terminal, de modo que nunca desplace o rompa el tileado.

Estas coordenadas no tienen consecuencias de gameplay. Solo relacionan al lugar
con su representación visual.

### Actor

Todos los actores usan el mismo modelo conceptual. Su representación concreta
en Rust pertenece a la documentación técnica y no se duplica aquí.

Un actor:

- siempre está en un lugar y nunca está lógicamente en un tile;
- posee características que expresan inicialmente todo su estado de gameplay;
- si no es el actor jugador, puede formar parte de su grupo;
- puede tener arte ASCII definido en sus archivos de contenido;
- puede incluir código Lua cuando necesita decidir acciones como NPC.

El actor jugador se diferencia por recibir sus decisiones desde el frontend. Los
NPC ajenos al grupo reciben sus decisiones desde reglas y scripts. Mientras un
actor forme parte del grupo, su política Lua no se evalúa: su participación se
decide dentro de las acciones del jugador.

### Grupo

El grupo representa exclusivamente a los compañeros del actor jugador. El actor
jugador no es un integrante del grupo, aunque siempre permanece junto a él. La
primera versión comenzará con al menos un compañero; su identidad se elegirá al
diseñar el setting.

En la primera versión:

- todos sus integrantes están siempre en el mismo lugar que el actor jugador;
- el actor jugador y el grupo viajan juntos y no pueden separarse;
- la lógica de cada acción general decide si un compañero ayuda, actúa o sufre
  una consecuencia.

### Característica

Es una etiqueta expresada como una palabra o adjetivo y asociada a un actor o
lugar. Las acciones consultan estas etiquetas para decidir qué puede ocurrir.

Ejemplos iniciales:

- `fuerte`;
- `rapido`;
- `empinado`;
- `fortificado`;
- `inalcanzable`;
- `cargaTesoro`;
- `herido`;
- `muerto`.

Las características pueden representar cualidades, posesiones o estados. En un
principio no habrá otro sistema de estadísticas para los actores.

### Relación

Es un objeto sencillo que conecta dos elementos del mundo. Por ejemplo, una
relación `conectado` entre dos lugares indicará que están conectados
geográficamente.

El sistema de relaciones se desarrollará más adelante. La primera versión de la
acción `Viajar` todavía no dependerá de él.

### Acción general

Representa una actividad completa elegida por un actor, como viajar o atacar.
Su código organiza el contexto, las decisiones, las acciones particulares, las
viñetas y el tiempo consumido.

### Acción particular

Es una operación reutilizable con su propia lógica de simulación. Puede ser
ejecutada desde distintas partes del código y desde distintas acciones generales.
Calcula y aplica un resultado sobre los elementos de la partida.

### Viñeta

Es una pantalla breve que presenta una parte de una acción. Puede mostrar una
descripción, pedir una decisión o comunicar el resultado de una acción
particular. Una acción general puede producir varias viñetas sucesivas.

## 4. Relaciones entre mundo, mapa, lugares y actores

```text
Mundo
├── Mapa visual: matriz de tiles
├── Lugares
│   └── coordenadas usadas solamente para presentación
└── Actores
    └── cada actor está lógicamente en un lugar
```

Las reglas fundamentales son:

- un actor está en un lugar, no en el mapa;
- un tile nunca determina la ubicación lógica de un actor;
- el frontend puede dibujar actores sobre el mapa para ilustrar dónde hay
  actividad;
- el tile exacto elegido para esa ilustración es una decisión estética;
- un cambio del estado del mundo puede producir un cambio en su representación
  visual;
- modificar por sí solo un tile no cambia el estado reglado ni habilita o
  impide acciones;
- viajar modifica el lugar del actor jugador y de los integrantes del grupo, no
  sus coordenadas de tiles.

## 5. Archivos de contenido e instancias

Los archivos de contenido definirán al menos:

- el mundo;
- el mapa y sus tiles;
- los lugares;
- los actores;
- las características asociadas;
- el arte ASCII de los actores;
- las partes de Lua utilizadas por los NPC.

Al comenzar una partida, el programa carga esos archivos y crea nuevas instancias
con los mismos datos iniciales, incluida una instancia visual propia del mapa que
podrá cambiar durante esa Partida. Los archivos fuente no se modifican. En esta
etapa no habrá generación procedural ni variación del estado inicial.

El formato, la distribución entre archivos, las referencias, la validación y la
representación de animaciones y scripts se diseñarán en la documentación técnica.
El setting, sus personajes, conflictos y lugares se diseñarán después de definir
esa estructura.

## 6. Sistema de acciones

Las acciones generales previstas son:

- viajar de un lugar a otro;
- acampar o descansar para recuperarse;
- atacar a alguien o a un lugar;
- rescatar o recuperar una persona o un tesoro;
- afrontar una prueba o tarea;
- buscar algo o buscar una salida;
- rogar o pedir ayuda o asilo;
- reclutar a un NPC;
- engañar;
- convencer;
- identificar una falsedad o una identidad oculta.

Los primeros incrementos previstos se concentrarán en:

1. viajar;
2. acampar;
3. atacar;
4. rescatar;
5. reclutar.

Esta lista expresa el orden de interés, no el alcance de un único incremento.
El primer incremento jugable implementará solamente `Viajar`, como se define en
la sección 9.

### Ejecución de una acción

La lógica inicial de las acciones estará escrita directamente en el backend:

1. Un actor solicita una acción general.
2. La acción establece su contexto y puede pedir una decisión.
3. El frontend presenta la decisión cuando quien actúa es el jugador.
4. La acción general ejecuta una o más acciones particulares.
5. Cada acción particular calcula y aplica su resultado.
6. El backend entrega al frontend el texto y los resultados correspondientes.
7. El frontend decide cómo presentarlos mediante viñetas.
8. La acción general continúa hasta terminar.

Una acción general puede quedar a la espera de una elección del jugador y
continuar después. Si un NPC debe tomar una decisión intermedia, la primera
versión elegirá al azar. Esa selección podrá volverse más compleja posteriormente
sin cambiar la lógica compartida de las acciones.

La participación de los compañeros se decide dentro del código de la acción
general. La acción puede consultar sus características e incluirlos en cálculos,
viñetas y consecuencias.

### Viajar en la primera versión

Al elegir `Viajar`, el frontend entra en un modo para seleccionar un lugar. Un
lugar con la característica `inalcanzable` no puede ser elegido como destino.

La lógica inicial de viaje tendrá una lista pequeña de grupos de viñetas y
seleccionará uno al azar. Más adelante podrá usar lugares, actores, relaciones y
reglas RPG para realizar una simulación más compleja.

Cuando el viaje se completa, el actor jugador y todos los integrantes de su grupo
pasan al lugar de destino.

## 7. Tiempo y fase de NPC

Las acciones particulares añaden al total de la acción general el tiempo que
consumen. Las cantidades iniciales —por ejemplo, un día o un mes— estarán
escritas directamente en su código y podrán modificarse a medida que se ejecuta
la acción.

Al terminar una acción del jugador:

1. La fecha del mundo avanza por el tiempo total calculado.
2. El programa recorre los scripts Lua de los NPC que no pertenecen al grupo.
3. Los scripts consultan características, variables del mundo, la fecha y otros
   datos disponibles.
4. Cuando se cumple una condición, el script solicita una acción para su NPC.
5. Esa acción se ejecuta mediante la misma lógica común utilizada por el jugador.
6. Las decisiones internas del NPC se toman al azar en la primera versión.
7. Al terminar la fase de NPC, el frontend vuelve a la vista del mundo y resume
   lo sucedido mediante un texto breve en la barra contextual.
8. Las acciones vuelven a estar disponibles sin abrir ni cerrar otra pantalla.

Las acciones de los NPC se consideran ocurridas dentro de la franja temporal que
ya produjo la acción del jugador. Por lo tanto, sus duraciones no vuelven a
adelantar la fecha del mundo.

## 8. Bucle inicial de la partida

```text
Iniciar partida
      ↓
Cargar archivos y crear instancias
      ↓
Mostrar el mundo y las acciones disponibles
      ↓
El jugador elige y completa una acción
      ↓
Avanzar la fecha según la duración acumulada
      ↓
Evaluar scripts y ejecutar acciones de NPC
      ↓
Mostrar resúmenes de la fase de NPC
      ↓
Volver a las acciones del jugador
```

Si el actor jugador posee la característica `muerto`, no puede solicitar otra
acción. En la primera versión esto no activa todavía una derrota formal.

## 9. Primer incremento jugable

El primer incremento implementará únicamente la acción general `Viajar`, pero
recorrerá el ciclo completo de una ronda para comprobar la arquitectura y poder
evaluar el juego desde el comienzo:

```text
cargar contenido mínimo
→ mostrar el mundo y el mapa
→ elegir Viajar y un destino
→ recorrer sus viñetas y decisiones
→ cerrar la última viñeta
→ avanzar la fecha una sola vez
→ evaluar y ejecutar la fase de un NPC mínimo mediante Lua
→ mostrar su resumen
→ regresar a la vista del mundo
```

El contenido inicial tendrá solamente los elementos necesarios para recorrer y
verificar ese flujo. Las acciones `Acampar`, `Atacar`, `Rescatar` y `Reclutar`
se incorporarán mediante incrementos posteriores, cada uno después de completar
su propio diseño técnico.

El orden anterior es observable: la fecha no se presenta como avanzada ni se
muestra la fase NPC antes de que el jugador cierre la última viñeta de su acción.
La documentación técnica decidirá si algún cálculo interno puede prepararse con
anticipación sin alterar decisiones ni revelar resultados futuros.

## 10. Frontend de terminal

La pantalla principal utiliza las dimensiones de la ventana de la terminal y
presenta:

- una vista del mapa mediante caracteres ASCII;
- los nombres de los lugares escritos sobre el mapa;
- arte ASCII animado, desde la cintura hacia arriba, del actor jugador y de los
  integrantes de su grupo;
- los retratos apoyados inmediatamente sobre la barra contextual, desde el borde
  izquierdo, como una capa que se extiende hacia arriba sobre el mapa sin
  reservar una franja propia;
- el retrato del jugador seguido por los del grupo, ligeramente desplazados
  hacia la derecha y con cada retrato posterior dibujado por delante de parte
  del anterior;
- una barra contextual que explica el estado o la elección actual;
- debajo de ella, en el borde inferior, una barra de acciones cuyo contenido
  cambia para mostrar las acciones, opciones o controles disponibles en el
  estado actual;
- una indicación visible `[X]` antepuesta a cada comando con atajo, cuya letra
  utiliza un color diferenciado.

Cuando el jugador puede actuar y todavía no está realizando una selección, la
barra contextual muestra `Elige una acción en el mundo.`.

La barra de acciones es contextual al modo aunque conserve siempre el mismo
nombre y posición. En la vista del mundo enumera acciones generales; al elegir
un destino muestra los controles de selección; durante una viñeta permite
continuar; y ante una decisión presenta sus opciones. No mantiene disponibles
acciones del mundo que no puedan ejecutarse en el modo actual.

Cuando comienza una acción, la viñeta vigente se dibuja como una ventana de
caracteres superpuesta a la pantalla normal. El mapa y los retratos permanecen
como fondo, mientras la barra contextual y la barra de acciones conservan sus
posiciones y cambian de contenido. Toda viñeta incluye texto que explica lo
sucedido o la decisión pendiente y puede incluir además una ilustración ASCII. La
ventana utiliza un marco estilizado siempre que las dimensiones disponibles
permitan presentarlo con claridad. Se centra dentro del espacio situado sobre
las barras y adapta su tamaño al contenido y a la terminal, dejando visible parte
del fondo alrededor siempre que exista espacio suficiente.

El backend puede acompañar el texto con una clave semántica opcional que describa
la escena, pero no entrega arte concreto. El frontend decide si representa esa
clave y qué ilustración utiliza. El arte incorporado en los archivos de Actor es
la excepción: sus retratos sí se cargan desde el contenido del mundo.

Cuando existe una ilustración, ocupa la parte superior de la ventana y el texto
aparece debajo. Si no existe, el texto aprovecha todo el espacio interior. Las
opciones de una decisión no se mezclan con ese contenido: se presentan en la
barra de acciones. En el primer incremento, las ilustraciones de viñeta son
estáticas; el arte animado se reserva para los retratos del jugador y del grupo.

Una decisión utiliza la misma ventana superpuesta y la barra de acciones permite
elegir una opción. El fondo conserva la última vista del mundo mostrada antes de
iniciar la acción; no revela cambios todavía ocultos. Al terminar la acción y la
fase de NPC, se dibuja la vista actualizada del mundo. El resumen NPC solo cambia
la barra contextual y no abre una viñeta.

Al activar `Viajar`:

- el frontend entra en modo de selección de lugar;
- la vista se centra en el lugar enfocado;
- el nombre de ese lugar cambia de color;
- la barra contextual reemplaza su mensaje normal por una frase breve que
  explica la selección actual.

Durante las acciones del jugador, el frontend muestra las viñetas y recoge sus
decisiones. Durante la fase de NPC, omite esas viñetas y presenta resúmenes
breves. La barra contextual es la misma en todos estos casos: cambia su
contenido para explicar una selección o decisión, mostrar el resumen NPC o dar
la ayuda correspondiente al estado actual. El resumen no crea una pantalla ni
un modo adicional.

El frontend controla la presentación, los modos, el foco, el dibujo, la entrada
y el arte concreto de las viñetas. El backend controla las reglas y le entrega
decisiones, textos, resultados y claves semánticas opcionales, nunca rutas o arte
de viñeta. Los contratos exactos de modos, menús, teclas y animaciones pertenecen
a la documentación técnica y no se duplican aquí.

## 11. Límites actuales y extensiones no diseñadas

La documentación técnica del primer incremento ya define la arquitectura, los
archivos RON, la acción `Viajar`, la fase NPC y el frontend necesario para su
recorrido vertical. Permanecen fuera de ese alcance:

- las reglas de las acciones generales posteriores y sus nuevas acciones
  particulares;
- el funcionamiento detallado de relaciones y facciones;
- sistemas de estadísticas distintos de las características;
- guardado y reanudación;
- victoria, derrota y presentación del final de una partida;
- comportamiento avanzado de NPC y varias políticas independientes;
- creación o modificación de variables globales durante la Partida;
- transformación automática de consecuencias del mundo en regiones del mapa;
- el setting, su contenido narrativo y el acabado artístico definitivo.

Estos elementos se añadirán de forma incremental. No deben suponerse antes de ser
diseñados.
