# `Viajar` como proyecto humano

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por el reglamento y contrato 06 |
| Versión | 1.2 |
| Fecha | 2026-10-20 |

> Antecedente histórico. Sus reglas y APIs fueron retiradas del motor activo.
> La fuente vigente es el [contrato de Viajar](./06-implementacion-viajar.md),
> en particular su sección 8 para las cinco acciones particulares actuales.

## 1. Propósito y autoridad

Este documento es la fuente canónica de la lógica que ordena la acción general
`Viajar`. Define cómo unas personas convierten el deseo de alcanzar un destino
en un proyecto: comprenden lo que saben, consideran maneras posibles de actuar,
adoptan un plan, procuran o emplean medios reales, realizan intentos, reciben la
respuesta del Mundo e interpretan si deben mantener, revisar o abandonar lo que
se propusieron.

No sustituye:

- la [expedición física](./02-viajar-expediciones.md), que resuelve rutas,
  reservas, etapas, movimiento, supervivencia y medios ya disponibles;
- el [contrato de acciones particulares](./particulares/00-contrato-resolucion.md),
  que resuelve cada intento concreto;
- la [cronología causal](./03-cronologia-causal-viaje.md), que establece cómo el
  proyecto, sus intentos y sus consecuencias se vuelven observables;
- las [operaciones del Mundo](../componentes/11-operaciones-mundo.md), que son la
  única vía para confirmar cambios persistentes.

Las reglas físicas vigentes permanecen autoritativas. Este contrato autoriza un
refactor integral de la orquestación anterior de `Viajar` cuando sea necesario
para subordinarlas al proyecto humano; no autoriza a duplicarlas, omitirlas ni
alterar sus resultados durante ese refactor.

## 2. Alcance y límites

El primer proyecto implementable posee un propósito mínimo y explícito:

> El dirigente quiere que los viajeros alcancen el Lugar elegido como destino.

Elegir el destino basta para expresar ese propósito en este alcance. La forma
admite motivos futuros —huir, rescatar, explorar, perseguir, llegar antes de una
fecha— sin inventarlos todavía ni obligar al jugador a escoger uno.

Esta versión incluye:

- deliberación visible antes de comprometerse con un plan;
- propuestas derivadas de hechos que los viajeros conocen realmente;
- decisión del dirigente, con aportes posibles de sus compañeros;
- un plan directo inicial con una promesa de selección de ruta explícita;
- procedencia visible de preparación y medios ya disponibles;
- subobjetivos que organizan las acciones particulares;
- interpretación del resultado respecto del plan;
- replanteamiento cuando cambia materialmente su viabilidad;
- abandono voluntario en el último Lugar confirmado;
- la misma máquina para el jugador y los NPC.

Quedan fuera conflictos entre integrantes, voluntades independientes del Grupo,
emociones como estado reglado, simulación del pensamiento, elección manual de
cada conexión, economía, inventario, animales o naves individuales y una acción
concreta para obtener un medio ausente.

## 3. Principio rector

`Viajar` no transporta una colección abstracta de Actores. Organiza el esfuerzo
de personas que actúan por una razón, desde lo que conocen y mediante recursos
que existen en su situación.

Su ciclo rector es:

```text
querer alcanzar un destino
→ comprender la situación conocida
→ formular propuestas realizables
→ adoptar un plan
→ asegurar sus precondiciones
→ perseguir un subobjetivo mediante acciones particulares
→ recibir la respuesta del Mundo
→ interpretar qué significa para el plan
→ mantenerlo, replantearlo o abandonarlo
→ llegar o terminar sin llegar
```

El código de la acción general compone este razonamiento mediante una gramática
autoral finita. No simula una mente completa ni determina qué emoción «posee» un
Actor. Entrega en cambio hechos humanos legibles —quién propuso algo, por qué
parecía razonable, quién se esforzó, qué se consiguió y qué cambió— a partir de
los cuales el jugador puede experimentar esperanza, penuria, alivio, aventura,
asombro u otra interpretación apropiada.

La relación normativa es:

```text
el proyecto humano propone una forma de actuar
→ las acciones particulares resuelven sus intentos
→ el Mundo confirma las consecuencias
→ el proyecto interpreta el nuevo estado y decide cómo continuar
```

Un plan nunca obliga al Mundo a producir la historia esperada. La simulación es
la autoridad sobre lo que efectivamente ocurrió.

## 4. Vocabulario del proyecto

### 4.1. Propósito

Es aquello que los viajeros intentan conseguir mediante la acción general. En
esta versión es `AlcanzarDestino(IdLugar)`. El destino indica adónde quieren
llegar; futuros motivos podrán explicar por qué sin cambiar las reglas físicas
de los Lugares.

### 4.2. Propuesta

Es una manera que un viajero puede concebir para cumplir el propósito según los
hechos que conoce. Indica:

- quién la propone o qué conocimiento del Grupo la sustenta;
- la estrategia general ofrecida;
- los medios y las fuentes o requisitos de preparación que supone;
- sus beneficios, riesgos e incertidumbres conocidos;
- las precondiciones que ya están satisfechas;
- las precondiciones que exigirían procurar algo antes de partir.

Una propuesta no es una ruta confirmada, una acción particular ni un texto. Es
información semántica que la aplicación puede redactar sin recalcular reglas.
Tampoco elige todavía la categoría física `Ligera`, `Suficiente` o `Cargada` ni
el ritmo: solo declara qué preparación parece disponible o necesaria para que
la propuesta sea honesta.
Una consulta interna puede estudiar muchas posibilidades, pero solo las
propuestas válidas que los personajes realmente consideran forman parte de la
deliberación observable.

### 4.3. Plan

Es la propuesta adoptada por el dirigente. Desde su adopción conserva la
estrategia y la configuración prevista de medios y fuentes para sus distintas
fases: no se pregunta de nuevo cómo viajar después de haber elegido una
propuesta que ya prometía marchar, usar camellos o embarcar en puntos concretos.
La preparación y el ritmo refinan después ese compromiso y quedan incorporados
al plan cuando se deciden, junto con la ruta prevista y la sucesión vigente de
subobjetivos. Es una hipótesis humana revisable, no una secuencia de resultados
garantizados.

El **plan directo** es la única estrategia de ruta de este alcance. Promete que,
entre las alternativas que los viajeros consideran posibles, la planificación
automática preferirá en este orden:

1. la menor suma de días máximos previstos;
2. la menor suma de exposición conocida e incertidumbre;
3. la menor cantidad de etapas;
4. la secuencia canónica de IDs, solo como desempate técnico.

Este orden pertenece a esa estrategia concreta. No es una voluntad universal de
`Viajar`. Planes futuros podrán prometer seguridad, discreción, abastecimiento o
terreno conocido antes que rapidez sin modificar las fichas particulares.

La propuesta muestra un pronóstico provisional normalizado con `RitmoViaje::Normal`
y `PreparacionViaje::Ligera`, sin consumir todavía `bienAprovisionado` ni otra
ventaja transitoria. Sí incluye los estados persistentes y conocimientos que los
viajeros ya poseen. Esos supuestos se muestran como tales: permiten comparar
honestamente medios y fuentes antes de decidir, pero no prometen que la ruta
provisional sea la definitiva.

Después de adoptar estrategia, medios y fuentes, el refinamiento sigue un orden
único y sin iteración ambigua. Primero, el planificador calcula de forma pura la
tabla completa de combinaciones entre ritmos y preparaciones disponibles. Cada
celda usa la carga cualitativa de esa preparación y solamente los medios y
fuentes comprometidos para hallar su mejor ruta. Esa consulta previa no consume
`bienAprovisionado`, reservas ni azar. Un ritmo se considera materialmente
distinto si cambia, en al menos una preparación disponible, la ruta, el rango de
duración, la exposición, la incertidumbre, el consumo previsto o los factores
conocidos de una acción particular. Luego:

1. el dirigente elige entre los ritmos materialmente distintos; si todos son
   equivalentes, el plan adopta el canónico sin abrir una decisión vacía;
2. cada alternativa de preparación muestra el pronóstico de su propia mejor
   ruta bajo ese ritmo y calcula sus raciones desde los días máximos de esa
   misma ruta;
3. al elegirla, el plan adopta juntos preparación, reservas y ruta definitiva.

La cantidad de raciones no retroalimenta el cálculo: `cargaVoluminosa` depende
de la categoría `Cargada`, que el pronóstico ya conoce, no del entero resultante.
`bienAprovisionado` puede sumarse de forma hipotética a las reservas de cada
celda, pero solo se consume después de adoptar la combinación y comenzar la
expedición. Por eso no existe un ciclo de convergencia ni una mutación durante la
consulta. Después de cada Lugar confirmado, el plan directo recalcula solo la
cola pendiente con el ritmo, preparación, reservas restantes, estado,
conocimiento, medios y fuentes vigentes, conservando la misma promesa
lexicográfica. Perder un medio, descubrir
una imposibilidad o necesitar una fuente no comprometida abre replanteamiento;
el cálculo nunca la incorpora en silencio.

### 4.4. Subobjetivo

Es el resultado inmediato que da sentido a la siguiente intervención. Ejemplos
del alcance vigente son preparar la partida, alcanzar el próximo Lugar, superar
un paso, encontrar agua, recuperar fuerzas o estabilizar una embarcación.

Una acción particular se invoca porque ayuda a cumplir un subobjetivo, no solo
porque una Característica permite ejecutarla. El proyecto fija quién intenta
qué y para qué; la ficha particular conserva la autoridad sobre cómo se evalúa
y qué ocurre.

### 4.5. Episodio de proyecto

Es una unidad autoral que desarrolla uno o más subobjetivos relacionados. Puede
contener deliberación, decisiones, acciones particulares, consecuencias e
interpretación. La situación dramática es su expresión narrativa observable;
ninguna de las dos reemplaza la ejecución mecánica.

El episodio no tiene una cantidad fija de acciones o viñetas. Tampoco es una
tabla azarosa de acontecimientos desconectados: solo puede surgir de un
propósito, un plan y hechos reales del contexto.

## 5. Participantes y autoridad para decidir

El actor que solicita `Viajar` es el dirigente del proyecto. Para el jugador,
los viajeros son el actor jugador seguido por sus compañeros en orden canónico;
para un NPC independiente, solo ese NPC.

En este alcance:

- el dirigente adopta el plan y responde las decisiones generales;
- un compañero puede proponer una alternativa, aportar conocimiento, ofrecer un
  medio al que ya tiene acceso, ser responsable de un intento o ayudar;
- la propuesta identifica ese aporte para que no parezca una capacidad anónima
  del Grupo;
- todos aceptan la decisión del dirigente y permanecen juntos;
- no se simulan discusiones, negativas, lealtades ni conflictos internos.

La ausencia de conflicto no vuelve impersonales a los compañeros. Si el Bandido
aporta conocimiento del desierto o camellos, el proyecto conserva su identidad
y la narración lo reconoce.

Un NPC usa exactamente las mismas propuestas y reglas. Cuando necesita elegir,
la política inicial de la fase NPC selecciona al azar entre las opciones válidas
en orden estable.

## 6. Medios y preparación con procedencia

### 6.1. Regla general

Una opción material solo puede proponerse si su acceso está confirmado o si
existe un episodio reglado capaz de procurarla. El nombre de un medio nunca lo
hace aparecer y una conexión geográfica nunca lo concede por sí sola.

El proyecto conserva una fuente semántica del acceso, por ejemplo:

```text
AccesoMedio
├── AportadoPorActor { actor, caracteristica }
└── ServicioLocal { lugar, servicio }
```

Caminar no requiere una fuente externa, aunque continúa sujeto a las
capacidades físicas de los viajeros y del terreno.

En el alcance vigente:

- `conCamellos` significa que ese Actor ya posee o controla acceso suficiente a
  monturas para los viajeros; la propuesta lo nombra como aportante;
- `conBote` y `conBarco` tienen el mismo significado cualitativo para sus medios;
- un `muelle` representa un servicio local rutinario ya disponible para iniciar
  una fase en bote;
- un `puerto` representa un servicio local rutinario ya disponible para iniciar
  una fase en barco.

El servicio local identifica el Lugar y el tipo de servicio. No afirma compra,
alquiler, propiedad, deuda ni coste cuando esas reglas no existen. El medio
acompaña a los viajeros solo durante la fase definida por la expedición física.

Si una acción nueva comienza mientras el Grupo ya ocupa un Lugar `acuatico`, la
propuesta registra `EmbarcacionActual { lugar }`: expresa que los viajeros
continúan a bordo del medio con el que alcanzaron esa posición. Esta regla hace
posible elegir un mar o estuario como destino —decisión aprobada para el atlas—
sin convertir el agua en un servicio, una propiedad o una embarcación creada de
la nada. Al desembarcar, esa procedencia deja de ser aplicable. No equivale a
`muelle`, `puerto`, `conBote` o `conBarco`.

Si ninguna fuente válida está presente, el medio no se ofrece. La interfaz no
presenta una elección para después inventar cómo se obtuvo.

### 6.2. Abastecimiento rutinario

Agua o alimento pueden prepararse de forma abstraída cuando una fuente real,
conocida y ordinariamente accesible ya existe en el Lugar, según el contrato de
expedición. La deliberación y la presentación nombran esa fuente: pozo,
manantial, mercado, posada u otra aprobada.

Esta abstracción significa que obtener la provisión no plantea, en ese contexto,
una incertidumbre o coste material que el juego distinga. Si hay oposición,
escasez, peligro, negociación, pago reglado o posibilidad significativa de
fracaso, deja de ser preparación automática y requiere una acción particular o
un episodio diseñado. La prosa no puede fingir que se pagó, robó o convenció a
alguien si esas acciones no ocurrieron.

### 6.3. Procura futura de un medio ausente

La gramática admite planes con una precondición todavía no satisfecha:

```text
detectar una oportunidad real
→ proponer procurar el medio
→ elegir un método reglado
→ ejecutar sus acciones particulares
→ confirmar o negar el acceso
→ continuar o replantear el viaje
```

No se implementa todavía un episodio concreto de esta clase. Requiere diseñar
primero las acciones particulares sociales, económicas o de captura que le den
consecuencias reales. Hasta entonces, una posibilidad que depende de un medio
ausente no es una propuesta válida y no participa en la selección de ruta.

Esta extensión no exige necesariamente Objetos individuales: una consecuencia
validada podrá conceder acceso cualitativo transitorio o persistente cuando su
diseño lo establezca.

## 7. Gramática inicial del proyecto

### 7.1. Inicio y comprensión

Confirmar un destino solicita iniciar `Viajar`, pero todavía no significa que el
motor haya aceptado la acción. `iniciar` realiza primero un **preflight puro**:
valida dirigente, viajeros, origen y destino, reúne solo su conocimiento y
construye las propuestas materialmente válidas con rutas, dificultades
previsibles, fuentes rutinarias y medios ya asegurados.

Si el preflight no obtiene ninguna propuesta válida, `iniciar` devuelve un
rechazo: no crea `ProyectoViaje` ni `EjecucionAccion`, no emite `Proposito`, no
consume azar o tiempo y no muta el Mundo. Solo cuando existe al menos una
propuesta, el motor acepta la acción, fija dirigente, viajeros, origen y
propósito, crea el proyecto y emite `Proposito` como su primera entrada
cronológica. Desde ese punto el proyecto **ya comenzó**; perder después toda
continuación válida conduce a `Interrupcion`, nunca a un rechazo retroactivo.

Estas consultas no son acontecimientos ni consumen azar. Sus resultados sirven
para construir las propuestas que los viajeros pueden formular honestamente.

### 7.2. Deliberación

La deliberación presenta las propuestas materialmente distintas. Cada una
explica, dentro de lo conocido:

- qué pretenden hacer;
- quién aporta conocimiento o medios relevantes;
- qué ventaja promete;
- qué coste, exposición o incertidumbre conocen;
- qué fuentes y requisitos cualitativos de preparación supone. Esto no es la
  categoría posterior `Ligera`, `Suficiente` o `Cargada`.

No se multiplican opciones cosméticas. Si solo existe un plan válido, la
deliberación puede ser breve y el dirigente lo adopta sin una elección vacía;
aun así se conserva por qué ese plan es realizable. Si varias propuestas cambian
materialmente el recorrido, el medio, la exposición o las consecuencias, el
jugador elige y un NPC aplica su política automática.

La cantidad de propuestas no se limita para acomodarla a una terminal concreta.
Tampoco se enumeran todos los recorridos posibles: para cada configuración
semánticamente distinta de estrategia, medios y fuentes conocidas, el
planificador devuelve su mejor ruta según la promesa del plan directo. Como los
Actores, servicios y configuraciones son finitos, el conjunto de propuestas
también lo es aun cuando el grafo contenga ciclos. Se elimina toda duplicación
material y se ordena de forma reproducible por el pronóstico lexicográfico del
plan directo, seguido por la identidad semántica canónica de la propuesta como
último desempate. El frontend pagina ese orden si no cabe en una sola barra; un
NPC elige sobre la lista completa, no sobre una página visual.

### 7.3. Adopción y preparación

Adoptar un plan confirma su estrategia, sus medios previstos y sus fuentes, pero
no resuelve todavía ninguna acción particular. Después se realizan solamente
las decisiones de preparación y ritmo que realmente cambien el resultado. Una
alternativa con otra configuración de medios exige otra propuesta o un
replanteamiento; nunca una segunda selección de medio desconectada del
compromiso adoptado.

Una preparación rutinaria se registra con su procedencia. Una preparación que
requiera superar una incertidumbre abre un episodio y solo queda cumplida si
las acciones particulares correspondientes lo consiguen.

### 7.4. Ejecución por subobjetivos

El proyecto entrega a la expedición física el plan adoptado y establece el
subobjetivo vigente. Por cada etapa:

1. determina qué impide o permite avanzar hacia el siguiente Lugar;
2. elige responsable y ayudante según el plan y los viajeros;
3. invoca la acción particular apropiada;
4. incorpora su duración, cambios transitorios y consecuencias confirmadas;
5. interpreta qué significa el resultado para el subobjetivo y el plan;
6. establece el siguiente subobjetivo o abre un replanteamiento.

Necesidades urgentes como sed, agotamiento o una embarcación en crisis pueden
reemplazar temporalmente el subobjetivo de avance. Atenderlas forma parte del
mismo proyecto porque preserva o recupera la capacidad de continuar.

### 7.5. Interpretación y replanteamiento

Una interpretación compara el estado confirmado con la promesa y los supuestos
del plan. No vuelve a resolver una acción ni añade consecuencias.

Hay cambio material cuando, al menos:

- una etapa o medio resulta imposible o deja de estar disponible;
- se descubre un hecho que cambia la viabilidad o el pronóstico restante;
- una consecuencia altera de forma relevante viajeros, reservas o medio;
- la cola recalculada al llegar a un Lugar difiere materialmente de la que el
  plan esperaba desde ese mismo punto;
- aparece una crisis que exige abandonar temporalmente el avance.

Ante un cambio material, el proyecto recalcula desde el último Lugar confirmado
y formula solo las propuestas ahora válidas. Puede conservar el mismo plan,
adoptar otro o terminar. No borra intentos, duración, consumos ni traslados ya
confirmados.

No se abre un replanteamiento si el cambio no altera materialmente la viabilidad,
el intercambio conocido ni las opciones del plan; la razón del ajuste menor sí
puede permanecer observable como interpretación. Cuando un cambio material sí
abre un replanteamiento, su cardinalidad se resuelve exactamente así:

| Continuaciones válidas | Resultado |
|---:|---|
| `0` | `Interrupcion` automática, sin decisión ni falsa posibilidad de abandono voluntario. |
| `1` | Decisión entre esa continuación y `Abandonar`. |
| `2` o más | Decisión entre todas las continuaciones, en orden canónico, y `Abandonar` al final. |

Por tanto, «queda una continuación equivalente» evita una decisión solamente
cuando nunca se abrió un replanteamiento material. Una vez abierto, continuar o
abandonar son alternativas humanas distintas aunque exista un solo plan viable.

Una **frontera segura de replanteamiento** existe únicamente después de aplicar
y presentar por completo las consecuencias del último intento, con el Grupo
unido en un Lugar confirmado y sin acción particular ni traslado pendientes. Si
el proyecto ya comenzó y todavía existe alguna continuación después de abrir el
replanteamiento, la decisión contiene todas las propuestas válidas y la
alternativa `Abandonar` como última opción. No se ofrece abandono cuando hay
cero continuaciones, durante una resolución, entre el inicio y el cierre de una
etapa ni después de llegar al destino. Cancelar la selección del destino antes
de iniciar `Viajar` tampoco es abandono: no crea un proyecto ni una conclusión.

### 7.6. Conclusiones

El proyecto concluye como:

- `Llegada`: los viajeros alcanzaron el destino y se liquidaron todas las
  consecuencias del último intervalo;
- `AbandonoVoluntario`: el dirigente decidió dejar de perseguir el propósito en
  una frontera segura de replanteamiento y los viajeros permanecen en el último
  Lugar confirmado;
- `Interrupcion`: ninguna continuación válida puede aceptarse o ejecutarse y
  los viajeros permanecen en el último Lugar confirmado.

Abandonar no retrocede automáticamente, no borra consecuencias y no inventa un
traslado. Regresar a un Lugar anterior exige un nuevo recorrido o un plan futuro
que lo ejecute mediante conexiones reales.

## 8. Variación y autoría de episodios

La variedad procede de combinar:

- el propósito y el destino;
- los integrantes y quién aporta cada posibilidad;
- su conocimiento;
- los Lugares y sus hechos físicos;
- fuentes reales de medios y provisiones;
- las elecciones del dirigente;
- resultados de acciones particulares;
- consecuencias y descubrimientos acumulados.

Los episodios hardcodeados definen formas plausibles de deliberar y actuar, no
historias completas con final prefijado. Una misma ficha particular puede
participar en episodios diferentes; un episodio puede requerir varias fichas.

El azar puede resolver incertidumbres regladas y seleccionar variantes
equivalentes de expresión. No elige una complicación genérica sin causa ni
reemplaza la selección de propuestas basada en el Mundo.

Para evitar que todos los viajes repitan una secuencia ceremonial, la gramática
solo emite una transición cuando existe contenido semántico que comunicar. No
omite por ello ninguna acción particular ejecutada ni ningún cambio material
del plan.

## 9. Estado reanudable y responsabilidades técnicas

`ProyectoViaje` es estado privado de la acción general, no elemento del Mundo ni
contenido RON. Su forma conceptual es:

```text
ProyectoViaje
├── dirigente y viajeros
├── origen inicial y último Lugar confirmado
├── propósito y destino final
├── conocimiento usado para deliberar
├── propuestas vigentes
├── plan adoptado y su estrategia
├── fuentes confirmadas de medios y preparación
├── subobjetivo vigente
├── Expedicion física, cuando ya comenzó
├── causa pendiente de interpretación o replanteamiento
├── registros de proyecto emitidos
└── decisión, escena o conclusión pendiente
```

La organización objetivo del núcleo incorpora una frontera explícita para esta
responsabilidad:

```text
acciones/viajar/
├── mod.rs
├── proyecto.rs
├── planificacion.rs
├── expedicion.rs
└── situaciones.rs
```

- `proyecto.rs` posee la máquina superior, propuestas, plan, subobjetivos,
  replanteamiento y cierre;
- `planificacion.rs` realiza consultas puras y aplica la promesa de la estrategia
  adoptada;
- `expedicion.rs` conserva la física transitoria;
- `situaciones.rs` asocia episodios con contexto semántico, sin redactar texto
  de frontend.

El núcleo emite registros neutrales de proyecto además de los momentos causales
de acciones particulares. La aplicación los transforma en prosa hardcodeada y
el frontend los presenta en las mismas ventanas y barras existentes. Una
deliberación, adopción o revisión no recibe una traza particular falsa. Cuando
un episodio ejecuta una acción particular, esa invocación conserva exactamente
su propio momento causal según el contrato cronológico.

## 10. Errores e invariantes

Son rechazos de gameplay un dirigente o destino inválido, la ausencia de
conexión, la ausencia de toda propuesta válida durante el preflight anterior a
crear el proyecto y una respuesta ajena a la deliberación vigente. Un medio sin
fuente no produce propuesta y, si no queda ninguna, causa
`SinPropuestaViajeViable`. Si un proyecto que ya comenzó alcanza un
replanteamiento sin ninguna continuación válida, concluye como `Interrupcion`: no se convierte
retroactivamente en una solicitud rechazada. Son fallos técnicos un plan incoherente con sus propuestas,
una fuente que desaparece sin consecuencia previa, una transición de proyecto
fuera de orden, una expedición física que contradice el plan adoptado o una
acción ejecutada sin subobjetivo que le dé sentido.

Invariantes:

- todo viaje abierto posee propósito, dirigente y último Lugar confirmado;
- toda propuesta procede únicamente de hechos conocidos y posibilidades
  materialmente válidas;
- el dirigente adopta el plan; un compañero puede proponer o aportar, pero no se
  simula conflicto en este alcance;
- la estrategia del plan gobierna la comparación de rutas;
- el plan directo promete explícitamente tiempo, después exposición e
  incertidumbre, después etapas y finalmente desempate canónico;
- ningún medio ausente se ofrece ni aparece después por prosa;
- todo acceso no peatonal conserva una fuente visible;
- una fuente rutinaria abstraída no afirma un coste o incertidumbre inexistente;
- cada acción particular ejecutada responde a un subobjetivo y conserva una
  única traza y un único momento causal;
- interpretar o replantear no vuelve a tirar, recalcular ni aplicar la acción ya
  resuelta;
- un replanteamiento comienza desde el estado confirmado y no revierte historia;
- abandono e interrupción conservan unidos a los viajeros en el último Lugar;
- la cantidad de episodios, acciones o viñetas no posee un límite narrativo.

## 11. Pruebas y criterios de aceptación

Se verificarán como mínimo:

- creación de un propósito `AlcanzarDestino` desde la solicitud vigente;
- preflight sin propuestas que rechaza sin crear proyecto, emitir propósito,
  consumir azar o tiempo ni mutar el Mundo, y pérdida posterior de toda
  continuación que concluye como interrupción;
- una única propuesta válida adoptada sin decisión artificial y dos propuestas
  materialmente diferentes que sí requieren elección;
- propuesta atribuida al compañero cuyo conocimiento cambia el pronóstico;
- propuesta de camellos que nombra al Actor con `conCamellos` como fuente;
- bote desde `muelle` y barco desde `puerto` con Lugar y servicio visibles, sin
  afirmar propiedad, compra o coste;
- viaje con dos fases acuáticas separadas que conserva una fuente distinta por
  etapa y no hace reaparecer la primera embarcación después de desembarcar;
- ausencia completa de una opción de medio sin Actor o servicio que la sustente;
- abastecimiento rutinario que nombra su fuente y rechazo de una abstracción
  cuando el caso declara oposición o incertidumbre;
- plan directo que conserva exactamente su promesa lexicográfica y desempate;
- comparación reproducible de propuestas directas y prueba de que su desempate
  final no depende del orden de recorrido de mapas o colecciones;
- acción particular invocada por un subobjetivo y resultado interpretado sin
  ejecutar por segunda vez la regla;
- continuidad del plan cuando nada material cambió;
- replanteamiento después de perder un medio, descubrir una imposibilidad o
  sufrir una consecuencia relevante;
- propuesta nueva construida desde el último Lugar y conocimiento confirmados;
- abandono voluntario durante un replanteamiento seguro antes de otra etapa,
  conservando Lugar, duración, estados y reservas ya causados, y ausencia de esa
  opción mientras una acción particular sigue pendiente;
- cardinalidad de replanteamiento exacta: cero continuaciones interrumpe sin
  decisión, una ofrece `[continuar, abandonar]` y varias ofrecen todas en orden
  canónico seguidas por `abandonar`;
- interrupción sin alternativas válidas y llegada después de liquidar la última
  etapa;
- paridad entre jugador y NPC, con selección automática reproducible;
- registros de deliberación, adopción, subobjetivo y replanteamiento, seguidos
  por una conclusión cronológica propia, sin trazas particulares inventadas;
- toda particular real conservada exactamente una vez dentro de esos episodios;
- recorrido manual en que se pueda explicar qué querían los viajeros, por qué
  eligieron el plan y el medio, quién los aportó, qué esfuerzo realizaron y por
  qué continuaron, cambiaron o abandonaron.

## 12. Decisiones aplazadas

Se aplazan:

- motivos distintos de alcanzar el destino;
- conflictos, negociación interna, obediencia, moral y emociones regladas;
- estrategias de ruta distintas del plan directo;
- el primer episodio concreto para procurar camellos, bote o barco ausentes;
- economía, precio, alquiler, deuda, robo y propiedad individual;
- captura, doma y cuidado persistente de animales;
- embarcaciones persistentes y tripulaciones independientes;
- planes propuestos por contenido editable o Lua;
- regreso automático, desvíos elegidos como nuevo propósito y encadenamiento de
  varias acciones generales.

Estas extensiones utilizarán el mismo ciclo de proyecto. No quedan preguntas
bloqueantes para el primer refactor descrito en este documento.
