# Desert Tiger Thief RPG — Diseño general del juego

| Campo | Valor |
|---|---|
| Estado | Base inicial aprobada |
| Versión | 0.19 |
| Fecha | 2026-10-26 |

## 1. Propósito

Este documento define solamente las bases del juego: sus elementos principales,
las relaciones entre ellos y el flujo general de una partida. Cada elemento será
desarrollado posteriormente en documentación técnica propia.

La documentación del proyecto tendrá cinco niveles:

1. Este diseño general describe qué elementos forman el juego.
2. Briefs de contenido describirán la intención creativa de regiones,
   personajes y otros conjuntos concretos sin duplicar sus archivos de datos.
3. Un diseño técnico general describirá la organización global del desarrollo.
4. Documentos técnicos específicos diseñarán cada parte del código y de los datos.
5. Los planes de implementación contendrán únicamente pasos de trabajo y
   referencias a los diseños aplicables.

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

Un Lugar puede representar una región terrestre, un asentamiento, una isla, un
camino, un paso, otro accidente geográfico o una región marítima. Cuando un Actor
se encuentra en un Lugar marítimo se entiende que está navegando; las reglas de
la acción determinan qué medio le permitió llegar. El medio puede expresarse de
forma cualitativa sin convertir todavía cada embarcación en un Objeto.

Esas posibilidades no son subtipos: todos usan el mismo elemento Lugar. Su
naturaleza concreta procede de su contenido y de sus Características.

Un camino, paso, vado o accidente semejante se convierte en Lugar cuando puede
ser destino o parada, contener Actores, servir como escenario o conservar la
posición del Grupo si una acción se interrumpe. Si el diseñador lo agrega para
representar lo que existe entre dos Lugares, retira la conexión directa y crea
dos conexiones ordinarias: la ruta debe entrar realmente en el tercer Lugar.
No se guarda en las Relaciones ningún dato que lo identifique como tal. Sus
Características describen hechos físicos como `empinado`, `rocoso` o `inundado`;
no conclusiones como `dificil` o `jornadaLarga`.

Las coordenadas del rótulo pueden formar una línea horizontal, vertical,
diagonal o curva. Cada letra conserva su orientación normal; lo que cambia es la
disposición de las celdas que forman el recorrido del nombre.

### Actor

Todos los actores usan el mismo modelo conceptual. Su representación concreta
en Rust pertenece a la documentación técnica y no se duplica aquí.

Un actor:

- si está en el escenario jugable, está en un lugar y nunca lógicamente en un tile;
- puede existir como contexto fuera del escenario, conservando su ficha y sus relaciones;
- posee características que expresan inicialmente todo su estado de gameplay;
- si no es el actor jugador, puede formar parte de su grupo;
- puede tener arte ASCII definido en sus archivos de contenido;
- puede incluir código Lua cuando necesita decidir acciones como NPC.

El actor jugador se diferencia por recibir sus decisiones desde el frontend. Los
NPC ajenos al grupo reciben sus decisiones desde reglas y scripts. Mientras un
actor forme parte del grupo, su política Lua no se evalúa: su participación se
decide dentro de las acciones del jugador.

Los Actores de contexto fuera del escenario no actúan localmente ni forman
parte del Grupo; se pueden consultar siguiendo relaciones desde Acampar. Esta
ampliación está implementada según el
[contrato de parentesco y contexto](./tecnica/componentes/14-parentesco-actores-contexto.md).

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
- `cargaTesoro`;
- `herido`;
- `muerto`.

Las características pueden representar cualidades, posesiones o estados. En un
principio no habrá otro sistema de estadísticas para los actores.

### Relación

Es un objeto sencillo que conecta dos elementos del mundo. Conserva un origen,
un tipo y un destino; no se convierte por ello en otro Actor, Lugar u objeto con
identidad narrativa propia. Actores y Lugares podrán declarar Relaciones dentro
de sus respectivos archivos de contenido para que cada elemento y sus vínculos
resulten fáciles de encontrar y modificar.

Una relación `conectado` entre dos Lugares indica que existe una conexión
geográfica entre ellos; las reglas todavía deben determinar si un Actor puede
recorrerla en sus circunstancias actuales. Para el primer uso del sistema,
`conectado` será simétrica: una única declaración permite considerarla en ambos
sentidos. Las conexiones forman un grafo independiente del Mapa visual. Ni la
cercanía entre tiles ni la posición de los rótulos crea una conexión implícita.

Una conexión entre tierra y mar puede representar embarcar o desembarcar, y una
conexión entre dos Lugares marítimos permite plantear navegación. La Relación no
almacena el barco ni garantiza que los viajeros dispongan del medio necesario:
las acciones particulares aplicables deducen esa posibilidad.

Las Relaciones no producen efectos por sí solas. Cada acción o regla decide qué
tipos interpreta y cómo los combina con los elementos y sus Características. El
primer tipo con significado reglado es `conectado`, utilizado por `Viajar`.

Una Relación dirigida `Conoce` entre un Actor y un Lugar expresa experiencia
específica de ese Actor con el Lugar. Las acciones particulares pueden usarla
para orientar, estimar o reconocer hechos sin ocultar necesariamente el nombre
y las conexiones del atlas. Para el Grupo, el conocimiento disponible reúne el
del actor jugador y sus compañeros, sin convertirlo en una cualidad universal
del Grupo.

Las relaciones Padre y Madre describen la filiación biológica; en el alcance
actual se asume también su vínculo social. Pueden estar documentados uno, ambos
o ninguno de los progenitores. La hermandad se deduce de progenitores
compartidos al consultar, sin guardar otra relación en el contenido.

MatadoPor registra un hecho histórico entre Actores, independiente de la
filiación. Estas relaciones permiten explorar el contexto del príncipe desde
las fichas; no introducen reglas de reino o sucesión. Su exilio se expresa en
la biografía. La ampliación se detalla en el contrato de parentesco citado.

Medios de transporte, preparativos, conocimientos y equipo son inicialmente
ideas con las que diseñar situaciones y decisiones, no categorías obligatorias
del juego. Un hecho cualitativo puede expresarse mediante una Característica; un
vínculo entre elementos, mediante una Relación; y una elección que solo importa
durante una acción puede permanecer en el estado temporal de esa acción. Solo se
creará una nueva categoría de elemento cuando algo necesite identidad propia,
persistencia y capacidad de ser relacionado o elegido como objetivo. Esta regla
evita convertir las partidas breves en administración de un inventario extenso.

### Acción general

Representa un proyecto humano completo elegido o emprendido por uno o más
Actores, como viajar o atacar. No se limita a encadenar operaciones: conserva
quién quiere conseguir algo, por qué lo intenta, qué cree saber, qué plan adopta
y cómo interpreta las consecuencias para continuar, cambiar de plan o abandonar.

El código de una acción general contiene un repertorio autoral y finito de
maneras plausibles de concebir y llevar adelante ese proyecto. No simula una
mente humana completa. Organiza propósito, deliberación, decisiones, medios,
subobjetivos, acciones particulares, situaciones dramáticas, viñetas y tiempo
consumido.

La lógica rectora es:

```text
querer algo
→ comprender la situación desde el conocimiento de los Actores
→ imaginar y elegir un plan
→ procurar y emplear medios reales
→ actuar
→ recibir la respuesta del Mundo
→ interpretar lo ocurrido
→ continuar, reformular el plan o abandonar
```

El plan humano propone qué se intentará; las acciones particulares resuelven los
esfuerzos concretos; el Mundo determina qué ocurre; la acción general comprende
qué cambia para el proyecto y decide cuál es su próxima situación. El frontend
presenta esta estructura, pero no la inventa ni la reconstruye.

### Acción particular

Es una operación reutilizable con su propia lógica de simulación. Puede ser
ejecutada desde distintas partes del código y desde distintas acciones generales.
Calcula un resultado y solicita las consecuencias regladas sobre los elementos
de la partida; el backend las valida antes de aplicarlas.

Cada tipo funciona como una entrada de reglas de un juego de rol de mesa: explica
cuándo corresponde resolverlo, qué hechos del Actor y del contexto importan,
cuándo es imposible o rutinario y, si existe incertidumbre, cómo se determina
su dificultad y sus resultados. La dificultad se deduce para el intento
concreto; no es una cualidad universal del Actor ni una conclusión precargada en
el Lugar.

### Situación o escena dramática

Es una unidad narrativa contextual organizada por una acción general. Explica
qué está ocurriendo, por qué importa en ese momento y qué carácter emocional
tiene la experiencia. La emoción surge de la situación y de su escritura —puede
ser peligro, aventura, libertad, alivio, asombro o cualquier otra apropiada— y no
requiere una categoría universal de `tono` almacenada en el Mundo.

Una escena también puede mostrar a los participantes comprendiendo su situación,
proponiendo cursos de acción, adoptando un plan, procurando un medio o
replanteándose lo que harán. Esas transiciones humanas pueden ser observables sin
inventar una acción particular: deliberar o comprometerse con un plan no finge
que ya se haya ejecutado ninguno de sus verbos. Cuando sí se realiza un esfuerzo
reglado para concretarlo, cada acción particular conserva su momento causal.

Una escena no corresponde a un tipo de acción particular. La misma acción
`Derribar` puede participar en el asalto a una fortaleza o en la huida de un
edificio incendiado; conserva sus reglas, pero la escena, los participantes, el
objetivo y el texto son diferentes. De igual forma, una escena puede contener
varias acciones particulares relacionadas por la misma situación.

En una acción presentada al jugador, cada ejecución particular debe quedar
reconocible dentro de exactamente un momento causal de alguna escena y conservar
su orden, resultado, duración y consecuencias. Una escena puede dedicarle varias
viñetas. Esta libertad narrativa no autoriza a omitir una intervención ejecutada
ni a sustituir varias resoluciones por un resultado que la simulación no produjo.
La fase NPC conserva esos momentos internamente, pero los comunica después
mediante el resumen breve establecido para ella.

La acción general solo puede atribuir efecto mecánico a un hecho narrativo si
ese hecho también forma parte del contexto reglado entregado a las acciones
particulares. La prosa puede aportar detalles sensoriales sin efecto, pero nunca
contradecir el estado o inventar retrospectivamente la causa de un resultado.

### Viñeta

Es una pantalla breve que presenta una parte de una escena dramática. Puede
establecer su contexto, desarrollar un momento, pedir una decisión, explicar
una acción particular o comunicar las consecuencias de varias intervenciones
relacionadas. Una escena y una acción general pueden producir tantas viñetas
sucesivas como exija lo realmente simulado; la viñeta no es la unidad de las
reglas.

## 4. Relaciones entre mundo, mapa, lugares y actores

```text
Mundo
├── Mapa visual: matriz de tiles
├── Lugares
│   └── coordenadas usadas solamente para presentación
├── Actores
│   └── cada actor está lógicamente en un lugar
└── Relaciones
    └── conectan elementos sin derivarse del Mapa
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
- la conexión entre Lugares procede de Relaciones explícitas y nunca de la
  distancia o continuidad aparente de sus tiles;
- viajar modifica el lugar del actor jugador y de los integrantes del grupo, no
  sus coordenadas de tiles.

## 5. Archivos de contenido e instancias

Los archivos de contenido definirán al menos:

- el mundo;
- el mapa y sus tiles;
- los lugares;
- los actores;
- las características asociadas;
- las relaciones declaradas junto a los elementos que funcionan como origen;
- el arte ASCII de los actores;
- las partes de Lua utilizadas por los NPC.

Al comenzar una partida, el programa carga esos archivos y crea nuevas instancias
con los mismos datos iniciales, incluida una instancia visual propia del mapa que
podrá cambiar durante esa Partida. Los archivos fuente no se modifican. En esta
etapa no habrá generación procedural ni variación del estado inicial.

El formato, la distribución entre archivos, las referencias, la validación y la
representación de animaciones y scripts se diseñarán en la documentación técnica.
Los briefs de contenido definirán la intención del setting, sus personajes,
conflictos y lugares; los archivos editables serán la fuente de sus datos
concretos.

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
2. La acción establece quién impulsa el proyecto, su propósito y el conocimiento
   con el que los participantes interpretan la situación.
3. La acción presenta cursos de acción que esas personas pueden concebir,
   incluidas sus premisas, medios y consecuencias conocidas.
4. El actor dirigente adopta un plan y la acción establece su siguiente
   subobjetivo. En el alcance inicial los compañeros pueden aportar conocimiento,
   medios y capacidades, pero no se simulan desacuerdos internos.
5. Si el plan requiere algo todavía no disponible, los participantes deben
   procurarlo mediante acciones reales antes de poder usarlo.
6. El backend entrega las viñetas de presentación o decisión que correspondan.
7. El frontend las presenta y devuelve la elección cuando quien actúa es el
   jugador.
8. La acción general ejecuta una o más acciones particulares dentro de esa
   situación.
9. Cada acción particular calcula su resultado y solicita las consecuencias
   regladas que correspondan.
10. El backend valida y aplica esas consecuencias, las enlaza con momentos
    causales de la escena y redacta las siguientes viñetas contextuales.
11. La acción general interpreta qué significan para el plan: puede conservarlo,
    cambiar su próximo subobjetivo, volver a deliberar o terminar.
12. La acción general continúa con la misma escena o abre otra hasta llegar,
    abandonar o quedar imposibilitada.

El primer incremento no incluye todavía acciones capaces de procurar un medio
ausente. Por eso, aunque esta gramática reserva su lugar, una propuesta que
dependa de conseguir algo inexistente no se ofrece hasta que ese episodio tenga
reglas propias.

Una acción general puede quedar a la espera de una elección del jugador y
continuar después. Si un NPC debe tomar una decisión intermedia, la primera
versión elegirá al azar. Esa selección podrá volverse más compleja posteriormente
sin cambiar la lógica compartida de las acciones.

La acción general decide qué compañeros son responsables o ayudantes y los
incluye en el contexto, las viñetas y la interpretación del resultado. Cuando
una intervención requiere calcular su capacidad o sus consecuencias, ese
cálculo pertenece a la acción particular correspondiente.

Las acciones particulares constituyen reglas reutilizables, no pasos escritos de
nuevo dentro de cada acción general. Una misma acción particular conserva su
lógica, requisitos y forma de evaluar al Actor sin importar si fue invocada por
`Viajar`, `Acampar`, `Rescatar` u otra acción. Por ejemplo, la capacidad de un
Actor para `Cazar` u `Orientarse` debe acompañarlo en todos esos contextos.

La acción general decide por qué se realiza el intento, quién participa, cuándo
se invoca y cómo su resultado afecta al objetivo general. La acción particular
decide cómo se resuelve la intervención concreta, qué duración aporta y qué
consecuencias regladas produce. Las propiedades del Actor, el blanco y el entorno
se vuelven a evaluar en cada contexto; los archivos de contenido no guardan
conclusiones como `viajeFacil` o `buenLugarParaCazar`.

### Viajar

Al elegir `Viajar`, el frontend entra en un modo para seleccionar el destino
final. El Lugar actual se ignora. Los destinos y recorridos proceden de
Relaciones `conectado`; tiles y coordenadas nunca participan.

`Viajar` representa el proyecto mediante el cual unas personas, movidas por una
razón y guiadas por lo que conocen, conciben una manera de alcanzar un destino,
procuran y emplean medios reales y ponen su esfuerzo en abrirse camino frente a
las dificultades del Mundo. La geografía y las reglas responden a sus intentos;
el Grupo interpreta lo ocurrido y conserva, modifica o abandona su plan.

El destino indica adónde quieren llegar, pero no constituye por sí solo el plan.
El plan reúne la razón disponible, la ruta imaginada, los medios, la preparación,
las aportaciones de los viajeros, los subobjetivos y las expectativas conocidas.
Es una narración prospectiva de lo que creen que harán, no una garantía de que
el Mundo permita realizarlo.

El jugador elige el destino final, no cada conexión. La acción selecciona una
ruta automáticamente a partir de lo que conocen los viajeros y del curso de
acción que hayan adoptado, y vuelve a evaluarla cuando cambia materialmente el
proyecto o alcanza una etapa. El cálculo de rutas propone posibilidades y
pronósticos; no reemplaza la voluntad de quienes viajan. Una ruta desconocida
puede ser incierta: el juego no revela por adelantado hechos que los Actores
todavía no conocen.

Antes de partir, el Grupo delibera cuando existen cursos de acción materialmente
distintos. Las opciones se formulan como planes humanos —por ejemplo, partir a
pie o usar los camellos que aporta un compañero para cruzar una región árida— y
no como bonificaciones aisladas. Una versión futura podrá proponer conseguir un
medio ausente cuando existan acciones que resuelvan realmente esa procura.
Inicialmente el actor dirigente decide y los demás lo siguen; los conflictos
internos quedan aplazados.

La expedición posee durante la acción agua, alimento, ritmo, preparación, medio,
duración y estados temporales. Esas reservas no forman un inventario persistente
ni obligan a administrar objetos individuales. Una abundancia excepcional puede
representarse con `bienAprovisionado` en el Actor dirigente y consumirse en un
viaje posterior.

La ruta atraviesa Lugares reales. Un camino, paso, vado, estuario o mar puede
ser destino y también el último Lugar alcanzado si la expedición se interrumpe.
La acción confirma el traslado del Grupo unido al completar cada etapa; mientras
se resuelve una intervención, todos permanecen en el último Lugar confirmado.

`Viajar` coordina acciones particulares como `Orientarse`, `Marchar`, `Escalar`,
`BuscarAgua`, `MontarCampamento`, `Cabalgar` o `Navegar`. Por ejemplo, un Lugar
`empinado` no es difícil por definición: `Escalar` o `GuiarCaravana` interpretan
esa pendiente de manera diferente según Actor, Grupo, carga, medio, estado y
conocimiento.

Los medios se representan inicialmente de forma cualitativa, como acceso a
camellos, un bote o un barco suficientes para el Grupo. Toda opción debe poseer
una procedencia explicable: un viajero ya dispone del medio, una persona o
servicio real lo proporciona, o el Grupo intenta conseguirlo mediante un
episodio preparatorio. Elegir un medio nunca lo hace aparecer. Si obtenerlo
requiere esfuerzo, riesgo o un coste distinguido por las reglas, esa obtención
forma parte del viaje y puede necesitar varias acciones particulares, fracasar y
obligar a reformular el plan.

No se modela todavía cada animal, pieza de equipo o embarcación como Objeto. La
abstracción se detiene en la relevancia causal: un medio que cambia las opciones,
costes o consecuencias debe existir o ser procurado; sus componentes incidentales
pueden permanecer incluidos en las reglas que lo utilizan.

`Viajar` organiza el recorrido en situaciones dramáticas surgidas de su contexto
real. No todas representan peligro: una navegación favorable puede transmitir
la libertad y la velocidad de avanzar sobre el mar, un descanso seguro puede
producir alivio y un paisaje desconocido puede comunicar aventura o asombro. La
explicación de los factores y resultados forma parte de esa experiencia, en vez
de aparecer como un informe separado de la ficción.

La narración acompaña tres partes inseparables: el proyecto que los viajeros
imaginan, la historia efectiva producida por sus intentos y la interpretación de
lo que esos resultados cambian. No hace falta almacenar esperanza, sufrimiento o
asombro como estadísticas para mostrar el esfuerzo acumulado, la contribución de
un compañero, la cercanía del destino, un paisaje abierto, una pérdida o el
alivio de haber dejado atrás una dificultad. Esos hechos permiten que el jugador
experimente el carácter humano del viaje sin fingir una simulación completa del
pensamiento.

No existe un límite de escenas o viñetas en el viaje presentado al jugador y no
se omiten pasos de su simulación para acelerar la presentación. Una escena puede
abarcar varias intervenciones relacionadas, pero cada acción particular que el
motor ejecutó permanece identificable en sus momentos causales. Una única acción
multidiaria puede narrar todo su intervalo sin inventar resoluciones diarias; si
el motor ejecutó varias acciones, la narración del jugador no puede convertirlas
en una sola. El resumen NPC continúa siendo deliberadamente compacto y no abre
estas escenas.

En el primer alcance la acción termina al llegar, al abandonar voluntariamente
el propósito o al quedar interrumpida en el último Lugar alcanzado. Regresar o
convertir un desvío en otro destino requerirá un plan posterior y conexiones
reales; nunca sucede como teletransporte de cierre. El Grupo nunca se separa.
Los problemas progresan mediante causas comprensibles como sed, fatiga o heridas;
una prueba rutinaria aislada no mata sin una cadena previa o una opción
claramente letal.

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

Los incrementos siguientes amplían este recorrido sin añadir otra acción
general: incorporan nuevos Lugares —como caminos, pasos y vados—, resolución
contextual, conocimiento, preparación, supervivencia y distintos medios. Cada
uno conserva el mismo bucle completo en estado jugable antes de profundizar la
etapa siguiente.

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

El backend puede acompañar el texto con claves semánticas opcionales que
describan la situación o el momento de la escena, pero no entrega arte concreto.
El frontend decide si las representa y qué ilustración utiliza. El arte
incorporado en los archivos de Actor es la excepción: sus retratos sí se cargan
desde el contenido del mundo.

Cuando existe una ilustración, ocupa la parte superior de la ventana y el texto
aparece debajo. Si no existe, el texto aprovecha todo el espacio interior. Las
opciones de una decisión no se mezclan con ese contenido: se presentan en la
barra de acciones. En el primer incremento, las ilustraciones de viñeta son
estáticas; el arte animado se reserva para los retratos del jugador y del grupo.

Una decisión utiliza la misma ventana superpuesta y la barra de acciones permite
elegir una opción. El mapa y los retratos proceden de la última vista confirmada,
pero cada viñeta puede aportar una instantánea histórica de progreso para mover
la cámara, el marcador y la fecha visible hasta su propio punto causal. Esa
instantánea no revela cambios posteriores. Al terminar la acción y la fase de
NPC, se dibuja la vista actualizada del mundo. El resumen NPC solo cambia la
barra contextual y no abre una viñeta.

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
- tipos de Relación con significado reglado distintos de `conectado`, así como
  facciones y relaciones que puedan cambiar durante una Partida;
- inventarios extensos, economía detallada y sistemas independientes de
  transporte, preparativos o conocimiento; esto no permite que un medio relevante
  aparezca sin una procedencia causal dentro de la abstracción vigente;
- sistemas de estadísticas distintos de las características;
- guardado y reanudación;
- victoria, derrota y presentación del final de una partida;
- comportamiento avanzado de NPC y varias políticas independientes;
- conflictos internos, opiniones persistentes y estados emocionales simulados
  de los integrantes del Grupo;
- creación o modificación de variables globales durante la Partida;
- transformación automática de consecuencias del mundo en regiones del mapa;
- la ampliación del setting más allá del
  [escenario inicial](./contenido/escenario-inicial.md), sus conflictos
  narrativos y el acabado artístico definitivo.

Estos elementos se añadirán de forma incremental. No deben suponerse antes de ser
diseñados.
