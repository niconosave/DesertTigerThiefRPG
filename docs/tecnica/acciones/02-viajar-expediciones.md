# Acción general `Viajar`: expediciones

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por el reglamento y contrato 06 |
| Versión | 1.7 |
| Fecha | 2026-10-20 |

> Antecedente histórico. Sus reglas y APIs fueron retiradas del motor activo.
> La fuente vigente es el [contrato de Viajar](./06-implementacion-viajar.md),
> en particular su sección 8 para las cinco acciones particulares actuales.

## 1. Propósito y autoridad

Este documento define el contrato físico de una expedición de `Viajar`: etapas
reales, preparación, acciones particulares reutilizables, consecuencias
progresivas y planificación basada en lo que saben los viajeros. No decide por
qué las personas viajan, qué propuesta adoptan ni qué significa un resultado
para su propósito. Esa lógica superior pertenece al
[`proyecto humano de Viajar`](./04-proyecto-humano-viajar.md).

Reemplaza al [contrato histórico](./01-viajar.md). Las reglas de aquel documento
no constituyen una ruta de compatibilidad activa.

Son fuentes complementarias:

- el [contrato de acciones particulares](./particulares/00-contrato-resolucion.md);
- las fichas de [movimiento terrestre](./particulares/01-movimiento-terrestre.md),
  [supervivencia](./particulares/02-supervivencia-campamento.md) y
  [navegación](./particulares/03-navegacion.md);
- la [cronología causal y presentación dramática](./03-cronologia-causal-viaje.md)
  de las situaciones del viaje y sus momentos causales;
- el [proyecto humano de `Viajar`](./04-proyecto-humano-viajar.md), que gobierna
  propósito, propuestas, plan, subobjetivos, procedencia de medios y replanteo;
- las [operaciones atómicas del Mundo](../componentes/11-operaciones-mundo.md);
- el [conocimiento de Lugares](../componentes/12-conocimiento-lugares.md);
- el brief de [nuevos Lugares del recorrido](../../contenido/trayectos-viaje.md).

## 2. Resultado jugable y límites

Después de que el proyecto humano adopta un plan, prepara una expedición y pide
una ruta conforme a la estrategia prometida por ese plan. La expedición recorre
Lugares conectados, resuelve las intervenciones que su naturaleza exige y puede
llegar, ser abandonada o quedar interrumpida en el último Lugar alcanzado.

Esta versión incluye:

- caminos, pasos, senderos, vados, calzadas y aguas como Lugares reales;
- ruta automática ponderada por estimaciones conocidas;
- ritmo, preparación, agua y alimento transitorios;
- conocimiento específico y ambiental;
- marcha, orientación, relieve, supervivencia, campamento y descanso;
- camellos, botes y barcos como acceso cualitativo suficiente para el Grupo;
- consecuencias persistentes mediante Características de Actor;
- ejecución equivalente para jugador y NPC.

Quedan fuera Objetos e inventario, cantidades por recipiente, animales o naves
individuales, clima simulado, estaciones, rutas dibujadas, elección manual de
cada conexión, separación del Grupo, comercio, reparación detallada, guardado y
Lua capaz de modificar directamente una expedición. Estas exclusiones no
permiten usar un medio sin la procedencia confirmada que exige el proyecto.

## 3. Lugares y etapas del recorrido

### 3.1. Representar un tramo mediante un Lugar

Un camino, paso, vado, puente, desfiladero, estuario o región marítima se modela
como Lugar cuando puede ser destino o parada, contener Actores, servir de
escenario o conservar la posición del Grupo si el viaje se interrumpe.

No existe una categoría, subtipo, marca ni colección especial para estos
Lugares. Todos usan exactamente el mismo modelo y participan del mismo catálogo
que un valle, un bosque o una ciudad. La distinción procede exclusivamente de
su identidad y de sus Características de contenido.

Si el diseñador crea un Lugar ordinario `C` para representar lo que existe entre
los Lugares `A` y `B`, el grafo contiene:

```text
A --Conectado-- C --Conectado-- B
```

La conexión directa `A -- B` se elimina. Conservarla permitiría saltar el
Lugar nuevo y sus condiciones cuando el diseño exige atravesarlo. `C` puede
seleccionarse como destino, aparece en las mismas consultas, tiene nombre y
representación visual y puede contener Actores como cualquier otro Lugar.

El modelo no almacena que `C` "está entre" `A` y `B`, ni enlaza entre sí las
dos Relaciones. Ese hecho resulta únicamente del contenido: dos declaraciones
`Conectado` ordinarias y la ausencia de la antigua conexión directa.

No todo detalle geográfico merece otro Lugar. Se crea únicamente cuando aporta
una etapa con identidad jugable. Su cantidad no se usa como una distancia
precargada, aunque cada etapa real puede requerir su propia intervención y
duración.

### 3.2. Hechos, no conclusiones

Los Lugares describen causas físicas mediante Características como `camino`,
`sendero`, `empinado`, `rocoso`, `pantanoso`, `aguaDulce`, `aguasSomeras`,
`corrienteRapida`, `derrumbado` o `inundado`.

No se escriben `dificil`, `facil`, `jornadaLarga`, `viajePeligroso`,
`transitable` ni otra conclusión propia de una acción. La misma pendiente puede
ser rutinaria para un escalador equipado, costosa para una caravana e imposible
para otro medio. Cada acción particular deduce su dificultad en ese contexto.

`Conectado` continúa significando solo adyacencia. No contiene pesos, duración,
medio, riesgo ni dificultad.

## 4. Estado transitorio `Expedicion`

`Expedicion` es el libro físico transitorio que el `ProyectoViaje` posee después
de adoptar un plan. No es un elemento persistente ni un archivo RON y no duplica
propósito, propuesta, promesa o interpretación. Contiene conceptualmente:

```text
Expedicion
├── actor dirigente y viajeros ordenados
├── destino final
├── etapas previstas con origen, destino, medio y fuente
├── último Lugar confirmado
├── ritmo y preparación
├── agua y alimento restantes
├── duración acumulada
├── estado de la embarcación activa, cuando corresponda
├── intentos físicos ya usados
└── estado físico pendiente de reanudar
```

Los viajeros son el jugador seguido de sus compañeros en orden canónico, o un
único NPC independiente. La lista se fija al aceptar la acción y nunca se
divide. Los estados que solo importan durante el recorrido —reservas, calidad
del campamento, desorientación o estado de una embarcación— viven aquí. Una
herida, fatiga consolidada o traslado confirmado se aplica al Mundo mediante un
lote validado.

El proyecto establece el subobjetivo que justifica cada intervención y proyecta
el tramo vigente del plan como etapas físicas. Cada etapa contiene exactamente
el medio que utilizará y su fuente confirmada cuando no sea peatonal. La
expedición no inventa un medio, no decide el plan y no interpreta sus resultados.

La fecha del Mundo no se muta mientras la expedición está abierta. La duración
acumulada se devuelve al cierre y la aplicación avanza la fecha una sola vez
para la acción del jugador; los NPC actúan dentro de esa franja y sus viajes no
vuelven a adelantarla.

## 5. Destinos, conocimiento y ruta automática

### 5.1. Destinos

El Lugar actual se omite siempre. Un Lugar puede enfocarse cuando pertenece al
mismo componente de `Conectado` y el proyecto puede formular al menos una
propuesta que no sea conocidamente imposible con los viajeros y las fuentes de
medios confirmadas. Una ruta incierta no se oculta: puede seleccionarse y
terminar interrumpida al descubrir una condición real. En el alcance vigente no
se considera una posibilidad que dependa de procurar un medio ausente, porque
todavía no existe un episodio reglado que pueda obtenerlo.

La Característica provisional `inalcanzable` deja de ser interpretada por esta
versión y se retira del contenido oficial. Una imposibilidad debe surgir de una
causa concreta y de la acción que intenta superarla.

El mapa, los tiles, la distancia visual y las coordenadas no participan.

### 5.2. Pronóstico de una etapa

La planificación deriva para cada transición una estimación pura a partir del
movimiento que exigiría y de los hechos conocidos por los viajeros. Devuelve:

- viabilidad `posible`, `imposible` o `incierta`;
- días mínimos y máximos previstos;
- exposición conocida en puntos ordinales no negativos;
- incertidumbre en puntos no negativos;
- viabilidad según el acceso conocido al medio.

La resolución detallada continúa perteneciendo a la ficha particular. La
estimación no tira dados ni muta el Mundo. Una etapa que ninguno de los viajeros
conoce suma un punto neutral de incertidumbre sin identificar el hecho oculto.

Las propuestas anteriores al compromiso se comparan bajo un pronóstico
provisional común: `RitmoViaje::Normal` y `PreparacionViaje::Ligera`, sin
consumir ni aplicar todavía `bienAprovisionado`. Sí se consideran los estados
persistentes, conocimientos y fuentes que los viajeros ya poseen. El resultado
declara esos supuestos y sirve para comparar configuraciones de medios y fuentes;
no convierte la ruta provisional en la ruta definitiva.

### 5.3. Elección y reevaluación

La búsqueda ponderada determinista calcula rutas candidatas. El plan directo
aprobado por el dirigente aplica su promesa y, para cada camino, suma los
pronósticos de sus etapas y minimiza lexicográficamente:

```text
(
    suma de días máximos previstos,
    suma de exposición conocida + incertidumbre,
    cantidad de etapas,
    secuencia canónica de IdLugar,
)
```

Una alternativa `imposible` no participa. `incierta` sí participa con su coste
conocido y su incertidumbre. La secuencia de IDs solo desempata. El orden
completo es la estrategia humana explícita `plan directo`, no una preferencia
universal de `Viajar`; estrategias futuras podrán ordenar los mismos pronósticos
de otra manera sin cambiar las acciones particulares.

Una vez adoptados estrategia, medios y fuentes, el planificador construye sin
mutar estado la tabla completa `ritmo × preparación disponible`. Cada celda
busca su propia mejor ruta usando la carga cualitativa de la preparación, el
ritmo correspondiente y solo los medios y fuentes comprometidos. Un ritmo es
materialmente distinto si en al menos una preparación cambia la ruta, duración,
exposición, incertidumbre, consumo previsto o factores conocidos de una acción
particular. El dirigente elige primero entre esos ritmos; con el ritmo fijado,
cada opción de preparación expone su propia ruta y calcula sus raciones desde el
máximo de días de esa ruta. Elegirla adopta conjuntamente preparación, reservas
y ruta definitiva. Si todos los ritmos son equivalentes, se usa el canónico sin
abrir una decisión artificial.

`cargaVoluminosa` depende de la categoría `Cargada`, no del número de raciones;
por eso la reserva resultante no obliga a iterar el cálculo. La contribución de
`bienAprovisionado` se proyecta en cada celda sin consumir la Característica y se
confirma una sola vez al comenzar la expedición elegida.

Al confirmar cada Lugar o producirse un cambio material del plan, la cola
restante se calcula otra vez desde el último Lugar confirmado con la preparación
y el ritmo elegidos, las reservas restantes, el estado y conocimiento actuales
y solamente los medios y fuentes comprometidos. El proyecto interpreta la causa
y decide si conserva el plan, formula otro o termina; el cálculo no toma esa
decisión por sí solo. Una fuente nueva no se incorpora en silencio y nunca se
revierte un traslado ya confirmado.

Si la submáquina física deja de encontrar una continuación para el plan
comprometido, devuelve esa causa y el último Lugar confirmado al proyecto, que
abre el replanteamiento correspondiente. La expedición solo recibe la conclusión
`Interrupcion` cuando la máquina superior determina que no existe ninguna
propuesta global de continuación; la capa física no puede cerrar el viaje antes
de considerar otro plan. Un desvío solo puede terminar en otro Lugar realmente
conectado; nunca en un tile ni en una posición inventada.

## 6. Preparación, reservas y ritmo

### 6.1. Unidad y abastecimiento

Agua y alimento son enteros transitorios medidos en
**raciones para un viajero durante un día**. La vista los resume como
jornadas-grupo. No hay reparto individual ni reservas negativas.

Un origen permite preparar agua cuando posee una fuente potable concreta
(`aguaDulce`, `pozo` o `manantial` sin un estado incompatible) o es un Lugar
civilizado capaz de abastecer (`poblado`, `posada` o `mercado`). Permite preparar
alimento si posee `poblado`, `posada` o `mercado`. La presencia silvestre de
fauna o vegetación habilita las acciones correspondientes, no entrega raciones
automáticas.

Cada preparación rutinaria conserva y expone la fuente concreta que la habilitó.
La abstracción solo es válida cuando no existe oposición, incertidumbre o coste
material distinguido por las reglas. Si el contexto introduce alguno, el
proyecto debe procurar el recurso mediante un episodio y acciones particulares;
la expedición no puede atribuir después una compra, ayuda o hallazgo que no se
resolvió.

Las opciones solo se presentan cuando producen resultados distintos:

| Preparación | Jornadas-grupo iniciales por recurso disponible | Efecto |
|---|---:|---|
| ligera | `1` | Sin factor de carga. |
| suficiente | `max(2, díasMáximosConocidos + 1)` | Sin factor de carga. |
| cargada | `max(4, díasMáximosConocidos + 3)` | Estado transitorio `cargaVoluminosa`. |

Cada jornada-grupo se multiplica por la cantidad de viajeros al crear la
reserva interna. Un recurso no disponible comienza en cero aunque el otro pueda
prepararse.

`díasMáximosConocidos` es el máximo de días de la ruta candidata calculada para
esa preparación bajo el ritmo ya elegido; no es el valor de otra alternativa ni
el del pronóstico provisional de la propuesta. Como el número de raciones no
cambia la categoría de carga, este cálculo se realiza una sola vez por celda de
la tabla de planificación.

Si el dirigente posee `bienAprovisionado`, la Característica se consume al
partir y añade dos jornadas-grupo de agua y dos de alimento. Al cerrar el viaje,
si quedan al menos dos jornadas-grupo completas de ambos recursos, se añade
`bienAprovisionado` al dirigente. El resto ordinario se descarta: representa lo
consumido, compartido o perecido durante el intervalo abstraído antes de otra
acción general.

### 6.2. Ritmo

El ritmo es una elección transitoria:

- `prudente`: aplica el factor previsto por `Marchar`, puede demandar más días y
  reduce desgaste;
- `normal`: no añade factor;
- `forzado`: intenta sostener el avance con penalización y consumo extraordinario
  de agua.

No se pide elegir ritmo si todas las preparaciones disponibles vuelven
equivalentes sus efectos conocidos según la tabla pura de planificación. Una
acción particular sigue siendo la autoridad sobre sus valores exactos.

## 7. Medios por etapa

El proyecto propone medios con procedencia confirmada y el dirigente adopta una
configuración concreta cuando las alternativas son materialmente distintas. El
plan fija el medio y su fuente para cada etapa prevista. La expedición valida y
ejecuta esa configuración: no vuelve a elegir otro medio. Si una incompatibilidad
se descubre durante el recorrido, devuelve la causa al proyecto para
replanteamiento; una contradicción sin descubrimiento o consecuencia previa es
una invariante rota. La fase acuática conserva su embarcación hasta desembarcar:

| Medio | Disponibilidad inicial | Acción dominante |
|---|---|---|
| a pie | Todos los viajeros; solo terreno físicamente recorrible | `Marchar`, `Escalar` o `Descender` |
| camellos | Algún viajero posee `conCamellos`; representa monturas suficientes para todos | `Cabalgar` o `GuiarCaravana` |
| bote | Algún viajero posee `conBote`, o la expedición comienza la fase acuática en un `muelle`; apropiado para río, costa protegida o aguas someras | `Remar` y acciones náuticas necesarias |
| barco | Algún viajero posee `conBarco`, o la expedición comienza la fase acuática en un `puerto`; apropiado para costa y mar abierto | `Navegar` y acciones náuticas necesarias |

Las Características expresan acceso suficiente, no objetos individuales. El
Actor que posee `conCamellos`, `conBote` o `conBarco` queda registrado como
aportante. En el contenido inicial, `muelle` y `puerto` declaran respectivamente
un servicio local rutinario de bote o barco: el Lugar es la fuente visible, sin
afirmar compra, alquiler, propiedad ni coste no reglamentado. El medio acompaña
al Grupo durante esa fase y deja de estar disponible al desembarcar, salvo que
un viajero conserve su Característica.

Una costa, una conexión tierra–agua o la mera presencia previa en un Lugar
acuático nunca crean un medio. Si existe una única alternativa viable se adopta
sin una elección vacía, pero el proyecto conserva y muestra su procedencia. Si
varias cambian materialmente pronóstico o consecuencias, decide el dirigente;
un NPC elige al azar entre las propuestas válidas.

`Marchar` y `Cabalgar`, o `Cabalgar` y `GuiarCaravana`, nunca representan dos
veces el mismo avance. Los medios aportan sus necesidades y estados al contexto
transitorio; una ficha particular decide si el Lugar concreto los favorece,
dificulta o impide.

## 8. Ejecución mediante acciones particulares

El proyecto humano coordina; esta expedición ejecuta la parte física sin resolver
por sí misma las capacidades de los Actores:

```text
recibir propósito, plan, estrategia, medios previstos y fuentes confirmadas
→ preparar reservas y ritmo conforme a la configuración adoptada
→ por cada etapa:
     recibir el subobjetivo que justifica la intervención
     orientar si el recorrido lo exige
     resolver una acción dominante de movimiento
     confirmar traslado solamente al completar la etapa
     consumir una vez los días transcurridos
     atender agua, alimento, refugio, descanso o crisis
     devolver resultado y estado confirmado al proyecto
     recalcular candidatos desde el Lugar alcanzado
→ dejar que el proyecto mantenga, replantee, abandone o cierre
```

Una intervención usa la misma ficha aunque después sea invocada por `Acampar`,
`Rescatar` o `Atacar`. El proyecto de `Viajar` fija subobjetivo, responsable,
ayudante y sentido; la expedición aporta su contexto físico y la particular
valida, deduce dificultad, consume su azar y devuelve duración y consecuencias.

`Viajar` también reúne en una misma situación dramática las intervenciones que
comparten Lugar, problema y propósito inmediato. Esa agrupación no cambia ni
resume su ejecución: cada invocación conserva un momento causal identificable y
ordenado. La misma ficha particular puede recibir otro sentido dramático cuando
otra acción general la invoque en un contexto diferente.

Una tarea rutinaria o sin desenlace interesante se resuelve de forma
determinista. Un intento incierto usa el procedimiento documentado de la ficha,
normalmente `1d20`. No se tira una tabla genérica adicional de “evento de
viaje” después de resolver la causa real.

## 9. Tiempo, consumo y supervivencia

- Cada acción particular devuelve días completos adicionales.
- Por cada día acumulado, `Viajar` descuenta exactamente una ración de agua y
  una de alimento por viajero.
- Una ficha solo informa consumos extraordinarios, como agua por marcha forzada.
- Un recurso obtenido durante el día puede cubrir el consumo de ese mismo día.
- Acampar y dormir tras la marcha normalmente añaden cero días; dedicar una
  jornada completa a buscar, cazar, pescar, recolectar o reposar añade uno.
- Una oportunidad de cero días se resuelve una sola vez por circunstancia; no
  permite intentos infinitos.

Cuando falta agua o alimento, el controlador intenta una sola vez por Lugar las
intervenciones físicamente habilitadas: agua, caza, pesca y recolección. Prioriza
la fuente más productiva disponible y después solicita la progresión
correspondiente de estados. Cada intervención que realmente ejecuta permanece
visible como un momento causal; varias pueden formar una sola escena cuando
pertenecen a la misma situación de supervivencia. El hallazgo y la contabilidad
se atribuyen a la acción que los causó. Una prueba rutinaria aislada no añade
`muerto`. La progresión persistente y la atomicidad pertenecen a
[Operaciones del Mundo](../componentes/11-operaciones-mundo.md).

La expedición completa siempre dura al menos un día. Todas las sumas usan
`Duracion` comprobada y un desbordamiento falla sin aplicar el cierre.

## 10. Cronología, decisiones y presentación

La [cronología causal](./03-cronologia-causal-viaje.md) es la única fuente del
orden y la exhaustividad de la presentación. El
[proyecto humano](./04-proyecto-humano-viajar.md) aporta propósito, propuestas,
compromiso, subobjetivos, interpretaciones y replanteamientos sin convertirlos
en falsas acciones particulares. `Viajar` compone situaciones dramáticas a
partir de esos registros, el Lugar, el medio, los viajeros, sus decisiones y las
consecuencias ya confirmadas. Sus textos pueden expresar
aventura, libertad, esfuerzo, alivio, descubrimiento, tensión u otra emoción
propia del contexto; el peligro no es el tono obligatorio.

Una situación puede contener varias acciones particulares y una acción puede
necesitar varias viñetas. Cada invocación ejecutada queda, sin embargo,
representada por un momento causal identificable en el orden correcto. No
existe un presupuesto ni un límite de escenas o viñetas, y ninguna acción se
oculta por ser automática, rutinaria o repetida. Agrupar no sirve para resumir
ni saltear pasos de la simulación.

La contabilidad derivada se atribuye al momento que la causó. Una acción de
varios días sigue siendo un único momento y muestra su intervalo completo sin
inventar acciones diarias. Una consulta de elegibilidad que no ejecuta la
acción no crea un momento. El progreso visible usa el último Lugar confirmado,
la duración acumulada y una fecha proyectada sin adelantar todavía la fecha del
Mundo.

La ficción y las reglas describen la misma realidad. Un hecho narrado que
modifica dificultad, duración, reservas, estado o desenlace debe existir en el
contexto de simulación y en la traza de la acción particular. Puede añadirse
ambientación sin efecto mecánico, pero nunca atribuirle causalidad ni usarla
para encubrir una causa real.

El frontend conserva la vista normal y dibuja las viñetas en su ventana
superpuesta. La barra contextual explica la situación y la barra de acciones
presenta opciones y el detalle mecánico `[D]` cuando corresponda. En modo
destino puede mostrarse una frase breve con duración, reservas y riesgo
conocidos, siempre obtenida del pronóstico del núcleo.

Las acciones particulares devuelven resultados semánticos y neutrales. La
acción general define situaciones y contexto, y un compositor narrativo fuera
del frontend terminal genera inicialmente los textos hardcodeados en Rust. El
frontend elige marco, color e ilustración, pero no inventa éxitos, causas,
costes ni información oculta.

### 10.1. Claves de situación del alcance vigente

`Viajar` identifica el contexto concreto de cada situación mediante este
catálogo cerrado inicial:

```rust
pub enum ClaveSituacionViaje {
    AvanceTerrestre,
    PasoAbrupto,
    RumboIncierto,
    SupervivenciaEnRuta,
    CampamentoEnRuta,
    TravesiaAcuatica,
    ManiobraNautica,
    CrisisNautica,
}
```

Estas variantes describen qué circunstancia une los momentos, no una emoción,
una acción particular ni una plantilla de cantidad fija. Por ejemplo,
`PasoAbrupto` puede reunir `Orientarse`, `Escalar` y `Descansar`, mientras
`TravesiaAcuatica` puede reunir navegación y maniobras ordinarias. Los hechos,
el medio, el Lugar, los participantes, las decisiones y los desenlaces concretos
siguen acompañando la referencia y permiten que dos apariciones de la misma
clave tengan textos y carácter dramático diferentes.

La apertura, las decisiones generales de preparación y la conclusión de la
expedición se identifican por sus registros o roles propios; no se fabrican
situaciones vacías solo para asignarles una clave. Añadir otra situación exige
actualizar este catálogo y sus proyecciones de forma exhaustiva, pero nunca
añadir un tono genérico como `Peligro` o `Aventura`.

## 11. Traslado, conclusión y operaciones

El Grupo permanece en el último Lugar confirmado mientras se resuelve una
etapa. Al completarla, un lote atómico traslada juntos a todos los viajeros y
aplica las consecuencias que deban confirmarse en ese momento. Una etapa fallida
no deja integrantes a ambos lados.

La implementación vigente posee:

- `Llegada`: el último Lugar es el destino final;
- `Interrupcion`: no existe continuación aceptada o posible y el Grupo queda en
  el último Lugar confirmado.

El refactor del proyecto añade:

- `AbandonoVoluntario`: durante un replanteamiento seguro, el dirigente decide
  dejar de perseguir el propósito y el Grupo conserva el último Lugar
  confirmado, el tiempo y todas las consecuencias ya causadas. La definición
  exacta de esa frontera pertenece al contrato del proyecto humano.

Regresos y desvíos futuros usarán estas mismas conexiones y traslados; no poseen
todavía una conclusión pública distinta. Si una regla futura añade `muerto`, la
restricción de acciones del jugador pertenece al estado del Actor, no a otro
cierre de expedición.

La duración acumulada y la conclusión se emiten una sola vez. Los cambios
persistentes se comunican únicamente después de que `Mundo::aplicar_lote` los
confirme.

## 12. Máquina reanudable y API

La solicitud pública identifica Actor y destino final. El
[`ProyectoViaje`](./04-proyecto-humano-viajar.md#9-estado-reanudable-y-responsabilidades-técnicas)
es la máquina superior. Conserva deliberación, compromiso, subobjetivo,
interpretación y replanteo; posee una submáquina física con los estados que
correspondan:

```text
EstadoExpedicion
├── Preparacion
├── Ritmo
├── CrisisNautica
└── PasoEmpinado
```

Los medios previstos y sus fuentes ya forman parte del plan adoptado por la
máquina superior; no reaparecen como una decisión física independiente. No todos los estados
aparecen en todo viaje. Cada decisión incluye un ID opaco,
opciones estables y el contexto visible necesario; reanudar con una decisión
vieja, de otro Actor o fuera de sus opciones produce rechazo sin consumir azar.

Los tipos internos implementados son equivalentes a:

```rust
enum PreparacionViaje {
    Ligera,
    Suficiente,
    Cargada,
}

enum RitmoViaje {
    Prudente,
    Normal,
    Forzado,
}

enum MedioViaje {
    APie,
    Camellos,
    Bote,
    Barco,
}

struct EtapaPrevista {
    origen: IdLugar,
    destino: IdLugar,
    medio: MedioViaje,
    fuente: Option<AccesoMedio>,
}

struct ReservasExpedicion {
    agua: u32,
    alimento: u32,
}

struct Expedicion {
    dirigente: IdActor,
    viajeros: Vec<IdActor>,
    destino_final: IdLugar,
    etapas_previstas: Vec<EtapaPrevista>,
    posicion: usize,
    preparacion: PreparacionViaje,
    ritmo: RitmoViaje,
    reservas: ReservasExpedicion,
    duracion: Duracion,
    intentos_supervivencia: BTreeSet<(IdLugar, u8)>,
    estado_embarcacion: Option<EstadoEmbarcacion>,
}
```

Los enteros se construyen y suman mediante conversiones comprobadas. `BTreeSet`
expresa el requisito de orden y unicidad;
puede sustituirse por otra colección determinista si conserva el mismo contrato.
`EtapaPrevista::fuente` es `None` solamente al caminar; camellos, bote y barco
exigen `Some`. Etapas acuáticas consecutivas pueden conservar el mismo medio,
fuente y `estado_embarcacion`. Desembarcar cierra esa fase; otra fase acuática
posterior posee sus propias etapas y puede usar un servicio distinto. Al
recalcular la ruta solo se sustituye la cola todavía no iniciada desde el último
Lugar confirmado: nunca cambia una etapa ejecutada ni su procedencia histórica.

La consulta de destinos se acompaña de un pronóstico puro por destino. La API
del núcleo devuelve destino, rango de duración, exposición conocida y presencia
de incertidumbre. La fachada transforma esos campos en la vista; no vuelve a
calcular rutas ni factores. Las jornadas-grupo concretas se presentan en la
decisión de preparación, cuando ya existe una expedición aceptada. Consultar la
vista no crea una `Expedicion` ni consume azar.

La organización objetivo dentro de `desert-tiger-core` es:

```text
acciones/viajar/
├── mod.rs
├── proyecto.rs
├── planificacion.rs
├── expedicion.rs
└── situaciones.rs
```

Las reglas de cada verbo viven en `acciones/particulares/`. La aplicación y
Ratatui nunca acceden a `Expedicion` ni recalculan rutas.

## 13. Viajes de NPC

Lua solicita la misma acción general con Actor y destino; no elige resultados ni
modifica reservas. La fase NPC responde al azar a cada decisión pendiente entre
sus opciones válidas y continúa hasta terminar la acción.

Un viaje NPC usa el mismo proyecto, propuestas, estrategia, rutas, conocimiento,
fuentes de medios, acciones particulares, consumos y operaciones. Solo cambia
quién responde las decisiones. Conserva los mismos registros y trazas causales,
aunque la fase NPC conserva internamente una entrada ordenada por momento y la
aplicación deriva después un resumen material breve. La terminal dibuja esa
proyección sin decidir qué agrupar, conforme al
[contrato cronológico](./03-cronologia-causal-viaje.md#9-viajes-de-npc). Un
límite defensivo de transiciones convierte un bucle en fallo técnico; no lo
disfraza como resultado narrativo. Su duración se informa, pero no vuelve a
avanzar la fecha del Mundo.

## 14. Errores e invariantes

Rechazos de gameplay incluyen Actor o destino inválido, Lugar actual, ausencia
de conexión, ausencia de toda propuesta conocidamente viable en el preflight y
una identidad de decisión u opción inválida. Un medio sin fuente se excluye de
las propuestas y puede causar `SinPropuestaViajeViable`; no posee un rechazo
tardío propio. Después de comenzar, su pérdida causada abre replanteamiento y la
ausencia de continuación produce `Interrupcion`, nunca un rechazo retroactivo.
Fallos técnicos incluyen incoherencia del grafo, pronóstico inválido, fuente de
azar fallida, desaparición de una fuente sin consecuencia causal, desbordamiento,
operación atómica rechazada tras aceptar la acción o límite de transiciones
excedido.

Invariantes:

- mapa, tiles y coordenadas no intervienen en ninguna regla;
- `Conectado` expresa solo adyacencia;
- ninguna dificultad o duración precalculada se almacena en un Lugar;
- cuando un Lugar nuevo sustituye una conexión `A--B`, no se conserva además
  ese atajo directo;
- todos los viajeros permanecen juntos;
- cada particular conserva sus reglas en cualquier acción general;
- toda acción particular de la expedición responde a un subobjetivo del proyecto;
- pronosticar no muta ni consume azar ni revela hechos desconocidos;
- el plan directo, y no el calculador por sí solo, adopta el orden lexicográfico
  de tiempo, exposición e incertidumbre, etapas e IDs;
- ningún medio no peatonal se ofrece ni se usa sin una fuente confirmada y
  observable;
- el medio de cada fase y su fuente proceden de la configuración del plan
  adoptado; la submáquina física no vuelve a pedir una elección de medio
  independiente;
- cada etapa prevista conserva su propia fuente y una segunda fase acuática puede
  proceder de otro servicio sin sobrescribir la historia de la primera;
- igual Mundo, decisiones y secuencia de azar producen el mismo resultado;
- una reserva nunca es negativa;
- cada día consume suministros una sola vez;
- un fallo rutinario aislado no mata directamente;
- solo se traslada a un Lugar realmente conectado y después de completar la
  etapa;
- jugador y NPC atraviesan el mismo código de reglas.

## 15. Pruebas y criterios de aceptación

Se verificarán como mínimo:

- sustitución `A--B` por `A--C--B` sin ruta de bypass;
- selección de `C` como destino y conservación allí tras una interrupción;
- ruta ponderada, desempates canónicos y reevaluación desde cada Lugar;
- propuestas comparadas con el mismo pronóstico provisional `Normal` y
  `Ligera`, sin consumo anticipado de `bienAprovisionado`;
- tabla pura completa `ritmo × preparación`, eliminación de ritmos equivalentes
  y adopción conjunta de la preparación, sus reservas y su propia mejor ruta;
- raciones `Suficiente` y `Cargada` calculadas desde los días máximos de su
  propia ruta candidata, sin iteración causada por el entero resultante;
- pronóstico diferente con `Conoce` sin filtración de un hecho oculto;
- eliminación de `inalcanzable` del contenido y de las reglas nuevas;
- preparación ligera, suficiente, cargada y consumo de `bienAprovisionado`;
- consumo diario exacto para uno y varios viajeros;
- obtención de recursos durante la misma jornada y crisis por carencia;
- marcha, relieve, supervivencia, camellos, bote y barco mediante sus acciones
  particulares;
- traslado atómico del Grupo y terminaciones de llegada, abandono voluntario e
  interrupción;
- camellos atribuidos al Actor que aporta `conCamellos` y servicios de bote o
  barco atribuidos al muelle o puerto correspondiente;
- recorrido que embarca desde un servicio, desembarca y vuelve a embarcar desde
  otro, con la fuente correcta en cada etapa y sin reutilizar implícitamente la
  primera embarcación;
- ausencia de cualquier opción de medio que no tenga Actor o servicio local como
  fuente confirmada;
- una escena con varias acciones particulares representadas como momentos
  distinguibles y ordenados, y una acción desplegada en varias viñetas;
- correspondencia entre invocaciones ejecutadas y momentos causales, sin límite
  ni supresión, y ausencia de momentos para consultas no ejecutadas;
- variantes dramáticas de una misma acción particular según su contexto, con
  causas regladas fieles y ambientación sin efectos inventados;
- contabilidad causal, progreso por Lugar y fecha proyectada conforme al
  contrato cronológico;
- viaje NPC con múltiples decisiones, límite defensivo y fecha sin doble avance;
- reproducibilidad con fuente de azar controlada;
- recorrido manual representativo por tierra y otro por agua;
- validación de todo el contenido oficial y controles generales del proyecto.

## 16. Decisiones aplazadas

Se aplazan rutas elegidas tramo por tramo, estrategias distintas del plan
directo, rutas dibujadas, Objetos, economía, procura de medios ausentes,
capacidad de recipientes, animales y embarcaciones individuales, velamen
detallado, clima y estaciones simulados, campamentos persistentes, recursos que
se regeneran, conocimiento mutable y guardado de una expedición a mitad de
acción.

No quedan preguntas bloqueantes para este alcance.
