# Cronología causal y presentación dramática de `Viajar`

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por el reglamento y contrato 06 |
| Versión | 1.4 |
| Fecha | 2026-10-20 |

> Antecedente histórico. Sus reglas y APIs fueron retiradas del motor activo.
> La fuente vigente es el [contrato de Viajar](./06-implementacion-viajar.md),
> en particular su sección 8 para las cinco acciones particulares actuales.

## 1. Propósito y autoridad

Este documento es la fuente canónica de la cronología observable de `Viajar`:
define cómo la acción general vuelve comprensible un proyecto humano antes,
durante y después de sus intentos, sin ocultar pasos, y cómo acompaña el avance
del Grupo por el mundo y el calendario.

El [proyecto humano](./04-proyecto-humano-viajar.md) decide propósito,
propuestas, plan, subobjetivos, interpretación y replanteamiento. Las reglas que
deciden qué acciones particulares se ejecutan, sus factores y sus consecuencias
continúan perteneciendo a
[`02-viajar-expediciones.md`](./02-viajar-expediciones.md) y a las
[fichas particulares](./particulares/README.md). Este contrato no cambia esos
cálculos. Exige que el jugador pueda observar qué sucedió, por qué sucedió y
qué significó para la travesía, sin que el frontend vuelva a implementar las
reglas.

El alcance incluye viajes del jugador, la traza equivalente de viajes NPC, el
detalle mecánico opcional de la terminal y el progreso temporal y geográfico de
una expedición. No diseña arte definitivo, animaciones de desplazamiento, un
registro persistente entre partidas ni textos editables desde RON.

## 2. Vocabulario y relación entre unidades

### 2.1. Acción general

`Viajar` es la **acción general**. Coordina la expedición, decide qué acciones
particulares corresponde intentar y es responsable del proyecto y del contexto
dramático en que el jugador las experimenta. Conoce el propósito, las propuestas
que los viajeros consideran, el plan adoptado, su subobjetivo, la etapa, el
Lugar, los participantes, las fuentes de medios, las reservas, las decisiones y
las consecuencias anteriores necesarias para dar sentido a lo que ocurre.

La cronología articula tres dimensiones de una sola historia:

- **prospectiva:** qué quieren hacer los viajeros y por qué creen que su plan
  puede funcionar;
- **efectiva:** qué intentan realmente y cómo responde el Mundo;
- **retrospectiva:** qué cambia ese resultado para el plan y por qué continúan,
  deliberan otra vez o terminan.

### 2.2. Registro de proyecto

Un **registro de proyecto** vuelve observable una transición intencional real:
propósito, propuesta, compromiso, subobjetivo, interpretación o
replanteamiento. Puede sostener una o varias viñetas, pero no declara por sí
mismo una acción particular ni recibe un momento causal ficticio. El registro
de propósito es la única apertura semántica del viaje; no existe además una
segunda emisión de «apertura» con los mismos hechos.

La **conclusión de viaje** es otro registro cronológico: cierra el proyecto como
llegada, abandono o interrupción después de liquidar sus consecuencias. No es
una variante de `RegistroProyecto`, aunque pueda sostener sus propias viñetas.

Las consultas internas que calculan posibilidades no son registros. Una
propuesta pasa a ser observable solamente cuando `Viajar` determina que los
personajes realmente la consideran; el compromiso existe cuando el dirigente la
adopta.

### 2.3. Situación o escena dramática

Una **situación dramática**, también llamada **escena**, es una unidad
contextual de la acción general. Presenta una circunstancia concreta de la
travesía y sostiene una intención o evolución emocional comprensible: aventura,
libertad, esfuerzo, descubrimiento, incertidumbre, alivio, camaradería, temor u
otra que surja del caso. No existe una enumeración rígida de tonos que el código
deba seleccionar.

Una escena puede presentar a los viajeros comprendiendo su situación,
deliberando, comprometiéndose con un plan, procurando una precondición,
esforzándose por cumplir un subobjetivo o replanteándose el curso. La narración
no empieza necesariamente con el primer desplazamiento ni termina al calcular
la última tirada.

Una escena puede contener varias acciones particulares cuando forman parte de
la misma situación. Por ejemplo, cruzar un paso puede requerir `Orientarse`,
`Escalar` y `Descansar` dentro de una sola escena sobre el esfuerzo de superar
la montaña. La escena termina cuando cambia materialmente el Lugar, el problema,
el objetivo inmediato o el contexto que les da unidad.

La escena no es una acción particular ni una envoltura creada una vez por cada
invocación. Es composición propia de `Viajar`. Usar la misma acción particular
en otro contexto puede producir una escena completamente distinta.

### 2.4. Viñeta

Una **viñeta** es una unidad de presentación dentro de una escena: una ventana
con texto y, opcionalmente, ilustración, decisión o resultado. Una escena usa
tantas viñetas como necesite para plantear la situación, hacer avanzar sus
acciones y mostrar sus consecuencias. Una acción particular puede ocupar una o
varias viñetas. Una viñeta también puede enlazar momentos relacionados, siempre
que cada uno permanezca inequívocamente identificable y ordenado.

No hay una cantidad fija de introducciones o resoluciones, ni una equivalencia
entre viñeta y cálculo mecánico.

### 2.5. Momento causal

Un **momento causal** es la representación observable de una invocación de una
acción particular que realmente se ejecutó. Permite reconocer:

- qué intentaron hacer los viajeros;
- quién actuó cuando sea pertinente;
- qué causas conocidas influyeron;
- qué desenlace produjo la resolución;
- qué consecuencias observables dejó.

Cada invocación ejecutada corresponde a un único momento causal. El momento
puede desplegarse a lo largo de varias viñetas, pero pertenece a una sola escena
y no puede quedar oculto dentro de ambientación genérica. Una escena con varias
acciones contiene varios momentos causales en el mismo orden de ejecución.

### 2.6. Acción particular

Una **acción particular** es una operación mecánica reutilizable como
`Marchar`, `Orientarse`, `Escalar`, `Cazar`, `MontarCampamento`, `Descansar`,
`Remar` o `Navegar`. Valida el intento, consulta hechos, resuelve reglas y
produce una traza neutral. No decide si el contexto es un ascenso heroico, una
huida angustiante o una jornada alegre: ese sentido pertenece a la acción
general que la invoca.

La relación canónica es:

```text
acción general Viajar
└── situación o escena dramática
    ├── viñetas de planteamiento, transición, decisión o desenlace
    ├── registros de proyecto que correspondan
    ├── momento causal de una acción particular
    ├── momento causal de otra acción particular, si corresponde
    └── viñetas de consecuencia o cierre
```

## 3. Exhaustividad sin equivalencia entre acciones y escenas

Durante una acción del jugador, toda acción particular realmente ejecutada debe
quedar cubierta por un momento causal visible. No puede omitirse, muestrearse ni
suprimirse por ser rutinaria, repetida, automática o poco adversa. Dos
invocaciones iguales consecutivas son dos momentos causales, aunque compartan
una escena. Una acción NPC conserva la misma cronología exhaustiva pero la
proyecta después al resumen breve definido en la sección 9; no abre esas
viñetas mientras el jugador espera.

No existe un máximo de escenas, viñetas ni momentos causales por viaje. Agrupar
varias acciones en una situación sirve para expresar su contexto común, no para
resumir la simulación ni reducir la cantidad de pasos que el jugador observa.
Una agrupación es inválida si impide distinguir una acción ejecutada, altera su
orden o esconde sus causas o consecuencias.

Tampoco existe una forma obligatoria de dos fragmentos por acción. Según la
situación, un momento puede necesitar una viñeta que anticipe el desafío, otra
para una decisión, otra para el desenlace y otra para sus repercusiones; una
acción automática sencilla puede quedar clara dentro de una viñeta compartida.
En ambos casos debe poder reconstruirse la cadena causal completa.

La duración no determina la cantidad de escenas o viñetas. Una acción
particular de cero días y otra de varios días producen un momento cada una. La
acción multidiaria explica el intervalo completo, pero no inventa acciones
diarias, noches ni campamentos que el motor no haya ejecutado.

### 3.1. Consultas que no son momentos causales

Las operaciones puras que estudian posibilidades no son ejecuciones:

- consultar destinos o elegibilidad;
- pronosticar una etapa;
- comparar rutas, medios o responsables;
- inspeccionar si existe agua, fauna, refugio u otra condición antes de decidir
  si se invocará una acción;
- recalcular la ruta;
- validar una solicitud antes de aceptarla.

Esas consultas no producen momentos ni textos que finjan que algo ocurrió. Si
el controlador invoca una acción y esta concluye como automática, fallida o
imposible, sí hubo una ejecución y debe aparecer como momento causal.

Deliberar no convierte las consultas previas en acontecimientos, pero la
deliberación de los personajes sí es una transición real del proyecto. Las
propuestas que efectivamente consideran, el compromiso del dirigente y un
replanteamiento se presentan mediante registros de proyecto y viñetas sin
atribuirles una traza particular. Si procurar una precondición ejecuta
`Convencer`, `Buscar`, `Capturar` u otra acción futura, cada invocación sí produce
su momento causal normal.

Preparación, ritmo, medio y otras decisiones de `Viajar` pueden tener contexto
visible sin convertirse por ello en acciones particulares. Nunca se narran
como ya ejecutados todos los verbos posibles antes de que la elección determine
cuáles se intentarán realmente.

## 4. Construcción dramática fiel a la simulación

### 4.1. Contexto de la escena

`Viajar` construye una escena a partir de hechos disponibles en su contexto:

- propósito y razón disponible para emprender el viaje;
- propuestas, proponente o aportante y beneficios o costes conocidos;
- plan adoptado, promesa vigente y procedencia de sus medios;
- subobjetivo que explica por qué se intenta cada intervención;
- origen, etapa, siguiente Lugar y destino final;
- naturaleza conocida de los Lugares atravesados;
- medio, ritmo, preparación y reservas;
- viajeros presentes, capacidades y estados observables;
- elecciones del jugador;
- acciones ya ejecutadas y consecuencias que siguen vigentes;
- interpretación vigente de cómo esos resultados afectan el plan.

La apertura sitúa al jugador antes de revelar el desenlace. Las viñetas
siguientes hacen avanzar la circunstancia, incorporan decisiones cuando las
haya y muestran cómo las acciones cambian la travesía. El cierre expresa la
consecuencia dramática y material de lo ocurrido y qué cambia para el proyecto.
Puede enlazar con la continuación o con una nueva deliberación sin anticipar su
resolución.

El texto debe comunicar una experiencia, no solamente recitar modificadores.
Una navegación favorable puede expresar amplitud, velocidad y libertad bajo un
día luminoso; un campamento seguro puede transmitir alivio y camaradería; un
desvío puede generar desconcierto. El peligro y el suspenso son posibilidades,
no el tono predeterminado de todo viaje.

### 4.2. Coherencia entre ficción y reglas

La prosa y la simulación describen una sola realidad. Si una viñeta afirma que
el viento favorable acelera la nave, ese viento debe existir como hecho del
contexto de simulación, haber sido consultado por `Navegar` y figurar en su
traza. No se puede inventar después un hecho causal para adornar un resultado.

Sí pueden existir detalles atmosféricos sin efecto mecánico —luz sobre el agua,
un olor, un gesto o el sonido de las velas— siempre que el texto no les atribuya
un cambio en dificultad, duración, reservas, estados o desenlace. Esa
ambientación nunca contradice hechos conocidos ni revela información oculta.

De forma recíproca, toda causa mecánica observable que cambie materialmente el
resultado debe llegar a la narración. La escena puede expresarla con lenguaje
propio del contexto, pero no sustituirla por una explicación falsa o genérica.

### 4.3. Autoría de los textos

En este alcance, los textos y variantes dramáticas permanecen hardcodeados en
Rust. `Viajar` define la escena, su contexto semántico y sus momentos; un
compositor narrativo fuera del frontend terminal transforma esos datos en texto
visible. Las acciones particulares aportan trazas mecánicas neutrales y Ratatui
solo presenta el material recibido.

Así, cambiar de terminal a un frontend 2D o 3D no obliga a reimplementar reglas,
y reutilizar `Escalar` desde `Atacar` o `Rescatar` no arrastra una narración de
viaje fuera de contexto. Mover los textos a contenido editable queda aplazado,
pero no cambia esta separación de responsabilidades.

## 5. Cronología observable y revelación progresiva

La secuencia del jugador conserva este orden general:

```text
apertura del propósito
→ deliberación de propuestas conocidas
→ compromiso con un plan y sus fuentes
→ preparación que corresponda
→ por cada situación dramática, en orden:
     planteamiento de la situación
     declaración o actualización del subobjetivo
     por cada acción particular ejecutada dentro de ella:
         anticipación o intención, cuando haga falta
         decisión que configure el intento, si corresponde
         resolución mecánica
         presentación del desenlace y sus consecuencias
     interpretación de qué cambia para el plan
     continuación o replanteamiento, cuando corresponda
→ conclusión de llegada, abandono o interrupción
```

La estructura describe orden causal, no una cantidad obligatoria de viñetas.
Una sola viñeta puede cumplir más de una función compatible, y una función puede
necesitar varias viñetas. Lo invariable es que no se revele una consecuencia
antes de que el jugador vea el contexto necesario y responda cualquier decisión
pendiente.

Los registros de proyecto respetan el mismo orden y pueden requerir viñetas aun
cuando entre ellos no se ejecute una acción particular. La cronología nunca
presenta una propuesta como si ya hubiera sido realizada ni una consecuencia
del plan imaginado como si el Mundo la hubiera confirmado.

El núcleo puede calcular una resolución dentro de la misma llamada síncrona
cuando no hay entrada pendiente, pero la aplicación conserva la cola ordenada y
no publica cambios posteriores ni progreso futuro antes del momento que los
causó. Tras una decisión, se presenta el momento resuelto antes de mostrar otro
que dependa de él.

Una decisión rechazada no añade escena, viñeta ni momento y conserva intacta la
situación vigente. Las operaciones confirmadas del Mundo y los registros
narrativos respetan el mismo orden.

## 6. Explicación y traza causal

### 6.1. Contenido visible de un momento

Antes de una resolución incierta, el momento puede presentar:

- qué está intentando el Grupo y por qué;
- quién es responsable y qué ayuda conocida resulta pertinente;
- los hechos físicos, capacidades, estados, conocimiento, preparación, ritmo o
  medio conocidos que cambian materialmente la situación;
- el riesgo o intercambio comprensible antes de elegir.

Después de resolver, el momento enlaza expresamente:

```text
acción intentada
+ factores determinantes
+ resolución automática o incierta
→ desenlace
→ costes, beneficios, estados, traslado y tiempo
```

Debe permitir comprender por qué sucedió el resultado. Identifica el verbo, el
responsable cuando importe, al menos la causa favorable o adversa dominante, la
banda obtenida y todo efecto observable. Frases genéricas como «algo salió mal»
o «el terreno pasó factura» no bastan cuando el núcleo conoce la causa concreta.

Un hecho antes desconocido solo se nombra cuando el resultado lo vuelve
perceptible. Que haya influido internamente no autoriza a revelarlo. La misma
política de conocimiento gobierna el texto principal y el detalle mecánico.

### 6.2. Traza neutral de la acción particular

Cada ficha particular conserva al resolver una traza neutral suficiente para
construir el momento sin recalcular reglas. Conceptualmente incluye:

```text
TrazaAccionParticular
├── identificador único de invocación
├── verbo estable
├── responsable y ayudante pertinente
├── origen, destino o Lugar de la intervención
├── factores aplicados en orden canónico
│   ├── clave estable
│   ├── explicación pública
│   ├── eje causal
│   ├── ajuste firmado
│   └── visibilidad para los viajeros
├── modo: imposible | determinista | incierto
├── dificultad, dado, modificador y margen cuando sean revelables
├── banda y desenlace propio del verbo
├── duración aportada
├── cambios transitorios y de reservas
└── operaciones persistentes confirmadas
```

La acción particular es la única autoridad sobre factores, ajustes y banda.
`Viajar` decide en qué situación ocurre, le aporta contexto real y ordena su
momento, pero no reconstruye la dificultad. El compositor narrativo expresa los
datos ya filtrados y Ratatui solo los dibuja.

Un factor que no se aplicó no aparece como causa. La traza conserva la regla de
un máximo por eje causal definida en el contrato común y permite comprobar que
un mismo hecho no se contó dos veces.

### 6.3. Registros neutrales del proyecto

Cada transición intencional conserva datos suficientes para redactarla sin
analizar texto ni repetir consultas. Según su clase incluye propósito, propuesta
y proponente, promesa y fuentes conocidas, plan adoptado, subobjetivo,
interpretación de un cambio o alternativas de replanteamiento. La causa y la
clase del cierre pertenecen al registro cronológico `ConclusionViaje` separado.

Estos registros no poseen tirada, banda, duración ni operaciones salvo que una
acción particular real los haya causado; en ese caso enlazan el momento causal
ya existente en vez de duplicarlo. La interpretación explica la relación entre
estado y plan, pero no añade retrospectivamente una consecuencia.

### 6.4. Contabilidad acoplada

Duración, consumo extraordinario, raciones ordinarias causadas por ese
intervalo, recursos obtenidos, cambios de reservas, fatiga, heridas, estado de
la embarcación y traslado se atribuyen al momento que los causó. No se convierten
en sucesos azarosos ni en escenas contables independientes.

Si una carencia provoca otra acción particular —por ejemplo `Cazar` o
`BuscarAgua`—, esa nueva ejecución recibe otro momento causal. Ambos pueden
pertenecer a la misma escena si comparten la situación: el primero deja visible
la necesidad y el segundo explica el intento, lo obtenido y el saldo producido.

Las operaciones internas sin verbo propio —aplicar un lote, descontar un valor,
confirmar el traslado o recalcular la ruta— se muestran únicamente como efectos
del momento causal. Si no cambian nada observable, no añaden texto de relleno.

## 7. Progreso geográfico y temporal

Toda viñeta y decisión de la expedición lleva una instantánea neutral de
progreso correspondiente a su punto cronológico:

```text
ProgresoViaje
├── lugar_confirmado
├── siguiente_lugar, si existe
├── destino_final
├── duracion_transcurrida
└── fecha_proyectada
```

Antes de una acción, `lugar_confirmado`, duración y fecha representan el estado
previo. `siguiente_lugar` expresa hacia dónde se intenta avanzar, no que el
Grupo ya esté allí. Después de sus efectos se publican el último Lugar realmente
confirmado y la duración acumulada. Un fracaso que no traslada conserva el
origen; un avance confirmado publica el destino de la etapa.

`fecha_proyectada` se calcula de forma comprobada como:

```text
fecha al iniciar Viajar + duracion_transcurrida
```

Es información de la expedición, no una mutación anticipada del Mundo. No activa
NPC, scripts ni reglas fechadas mientras `Viajar` continúa abierto. Al cerrar la
acción del jugador debe coincidir con la fecha que la aplicación confirmará una
sola vez en el Mundo. Una acción multidiaria hace avanzar directamente esta
proyección por su duración completa, sin inventar pasos intermedios.

La terminal usa `lugar_confirmado` para centrar la cámara, resaltar el rótulo y
dibujar un marcador ilustrativo del Grupo en la coordenada central del Lugar.
También muestra dentro del marco el nombre del Lugar, la fecha proyectada y el
tiempo total transcurrido; cuando existe, puede añadir el siguiente Lugar. Esas
coordenadas siguen siendo presentación: nunca regresan al núcleo ni determinan
una regla.

## 8. Texto principal, detalle `[D]` y paginación

La lectura normal prioriza ficción, emoción y causalidad, no una tabla de
cálculo. Cuando existe una traza, la barra de acciones ofrece `[D] Detalles`; al
activarlo cambia a `[D] Ocultar detalles` y expande la misma ventana con los
momentos disponibles hasta ese punto:

- acción particular y participante responsable;
- dificultad conocida y factores aplicados, con sus ajustes;
- resolución determinista, imposible o tirada de `1d20`;
- cara, modificador, margen y banda cuando esos valores sean revelables;
- duración aportada;
- cambios de reservas, estados, medio y ubicación causados por el momento.

Antes del desenlace, el detalle solo muestra el pronóstico y los factores que el
Grupo conoce. Después muestra la resolución efectiva hasta el límite permitido
por ese conocimiento. Un valor que revelaría un hecho oculto se sustituye por
una indicación de incertidumbre; no se puede inferir su cantidad ni ajuste por
totales parciales.

En una escena con varios momentos, el detalle los presenta en el mismo orden y
permite identificar a cuál pertenece cada cálculo. Mostrar u ocultar detalles
es estado local del frontend: no confirma la viñeta, no responde una decisión,
no consume azar, no modifica la aplicación ni crea otra escena. `D` queda
reservado mientras se presenta un viaje y no puede ser atajo de otra opción
simultánea.

No existe límite narrativo de viñetas. La paginación solo divide la superficie
de una viñeta demasiado extensa para la terminal; no elimina texto, fusiona
momentos ni altera la simulación. El frontend puede elegir marco, color,
ilustración y disposición, pero no redacta causas nuevas ni calcula reglas.

## 9. Viajes de NPC

Jugador y NPC generan los mismos registros de proyecto, trazas y progreso para
las mismas acciones, decisiones y azar. La fase NPC no abre una sucesión de
ventanas mientras el jugador espera, pero tampoco usa otra lógica de viaje.

La fase NPC conserva hasta el cierre una `CronologiaNpc` con todos los
`RegistroCronologico` en orden. Después deriva un `ResumenNpc` con una entrada
por cada momento causal, además del propósito, el plan, los replanteamientos
materiales y la conclusión. La cronología exhaustiva permite auditar que
atravesó el mismo proyecto y simulación que el jugador, pero no se vuelca
completa en una sola fila de la terminal.

Para un viaje iniciado, la aplicación deriva después una frase breve y material:
comienza por Actor, conclusión, propósito `AlcanzarDestino`, último Lugar y
duración simulada total; cuenta acciones y replanteamientos; nombra solo la causa
del último replanteamiento y la última consecuencia persistente, con cantidades
para las anteriores; y puede agregar un desenlace rutinario dominante. Su
longitud semántica es acotada y no crece con cada momento. Esa duración se valida
contra los momentos, pero no avanza la fecha del Mundo. Una solicitud rechazada
comunica Actor, destino solicitado y causa; una política fallida antes de
solicitar no inventa propósito, Lugar o duración. La terminal dibuja el texto recibido y puede
recortarlo por el ancho visible, pero no decide qué hechos son importantes ni
concatena por su cuenta la cronología. Las consultas y candidatos descartados
se omiten igual que para el jugador.

Las decisiones automáticas del NPC continúan resolviéndose con la política de la
fase NPC. No se convierten en elecciones retrospectivas del jugador y no
adelantan por segunda vez la fecha del Mundo.

## 10. Responsabilidades entre capas

- Las acciones particulares producen resolución y traza causal autoritativas,
  sin apropiarse del contexto dramático de quien las invoca.
- `Viajar` conserva el proyecto humano, crea situaciones dramáticas, asigna cada
  ejecución a un momento causal, interpreta su relación con el plan y conserva
  cronología y progreso.
- El motor emite registros de proyecto, escenas, momentos, decisiones y
  consecuencias estructuradas, sin prosa visible ni datos de Ratatui.
- El compositor narrativo fuera del frontend terminal produce los textos
  hardcodeados a partir de ese material, sin recalcular reglas.
- La aplicación impide revelar emisiones futuras, proyecta la fecha mediante el
  calendario común y entrega instantáneas poseídas.
- El frontend controla paginación, detalle expandido, cámara, marcador, marco y
  entrada; no ejecuta ni interpreta reglas.

Las claves y textos pueden evolucionar sin convertir orden, exhaustividad,
contexto dramático o progreso en convenciones exclusivas de la terminal.

## 11. Errores e invariantes

Son invariantes de este contrato:

- toda invocación particular ejecutada está cubierta por exactamente un momento
  causal; el jugador lo recibe de forma visible y la fase NPC lo conserva en su
  registro interno antes de resumirlo;
- una escena puede contener varios momentos y un momento puede ocupar varias
  viñetas;
- no existe presupuesto, máximo ni supresión narrativa por cantidad;
- agrupar nunca oculta, fusiona causalmente ni reordena acciones ejecutadas;
- el orden visible de los momentos coincide con el orden causal de ejecución;
- una acción multidiaria sigue siendo un momento y no inventa acciones menores;
- una consulta pura o un candidato no ejecutado no crea un momento;
- propósito, propuesta, compromiso, subobjetivo, interpretación y
  replanteamiento, además de la conclusión cronológica separada, pueden ser
  visibles sin crear un momento causal artificial;
- una propuesta nunca se presenta como hecho consumado y una interpretación no
  añade consecuencias a la acción ya resuelta;
- todo medio nombrado por el plan conserva y muestra su procedencia conocida;
- hechos narrativos con efecto reglado existen en el contexto de simulación;
- la contabilidad aparece con su causa y nunca como suceso azaroso separado;
- ninguna capa posterior recalcula factores, dificultad, banda o consecuencias;
- una vista no revela resultados, Lugares, fechas ni estados posteriores a su
  punto cronológico;
- fecha proyectada y fecha del Mundo son valores distintos mientras la acción
  está abierta;
- mapa, cámara y marcador no participan en gameplay;
- el detalle `[D]` no altera acción, tiempo, azar ni orden;
- información oculta continúa oculta tanto en prosa como en cifras;
- jugador y NPC conservan un momento por la misma ejecución particular.

La ausencia o duplicación de un momento; una inversión causal; una escena que
atribuye efectos mecánicos a ficción inexistente; un progreso que no coincide
con la operación confirmada; una fecha proyectada inválida; o una traza
incoherente son fallos internos. No se degradan a una narración genérica.

## 12. Pruebas y criterios de aceptación

Se verificarán como mínimo:

- apertura que presenta el propósito antes de deliberar;
- propuestas que identifican proponente, promesa y procedencia de medios sin
  afirmar todavía que el plan se ejecutó;
- compromiso del dirigente seguido del primer subobjetivo y no de un resultado
  anticipado;
- interpretación que enlaza un resultado real con continuidad o replanteamiento
  sin ejecutar dos veces la acción particular;
- replanteamiento visible después de perder un medio o descubrir una
  imposibilidad, y abandono voluntario desde el último Lugar confirmado;
- correspondencia estructural entre `N` invocaciones y `N` momentos causales,
  incluso cuando `N` supera seis, sin exigir `N` escenas ni una cantidad fija de
  viñetas;
- una escena de paso montañoso con `Orientarse`, `Escalar` y `Descansar` como
  tres momentos distinguibles y ordenados;
- una acción particular desplegada en varias viñetas sin duplicar su ejecución;
- dos invocaciones iguales consecutivas conservadas como dos momentos, aunque
  compartan escena;
- acciones automáticas, inciertas, imposibles, de cero días y multidiarias;
- decisión anterior a consumir su azar y desenlace posterior al contexto que
  permite comprenderlo;
- consultas de destino, pronóstico, elegibilidad y candidatos descartados sin
  momentos ni consumo de azar;
- variantes dramáticas distintas para una misma acción particular en contextos
  distintos, sin cambiar su resolución mecánica;
- texto aventurero o de alivio en situaciones favorables, sin convertir todo
  viaje en peligro o suspenso;
- rechazo de una causa narrativa con efecto mecánico que no figure en el
  contexto y la traza;
- factores por eje causal, causa dominante, banda y consecuencias procedentes de
  la misma resolución;
- consumo ordinario y extraordinario, hallazgos, estados y traslados atribuidos
  al momento causal correspondiente, sin escena contable adicional;
- Lugar conservado tras fallo y actualizado después de un traslado confirmado;
- duración y fecha proyectadas en cada punto, incluido un salto multidiario, y
  coincidencia con la fecha final aplicada una sola vez;
- ausencia de filtraciones antes y después de resolver un hecho desconocido;
- `[D]` diferencia los cálculos de varios momentos, reserva su tecla, pagina sin
  perder contenido y puede alternarse sin llamar a la aplicación;
- cambio de cámara, rótulo y marcador a partir de `IdLugar`, sin consultar tiles
  como regla;
- viaje NPC con cronología interna que conserva un momento ordenado por cada
  acción ejecutada y proyección breve que comunica propósito, desenlace,
  ubicación, duración simulada total y consecuencias materiales sin viñetas
  interactivas ni avance adicional de la fecha;
- recorrido manual terrestre y marítimo que permita reconstruir la historia,
  las causas, el Lugar y el tiempo del Grupo.

## 13. Decisiones aplazadas

Se aplazan un diario persistente consultable después del viaje, repetición de
escenas, animación interpolada entre Lugares, voces, localización de textos y
plantillas narrativas modificables desde contenido.

No quedan preguntas bloqueantes para este alcance.
