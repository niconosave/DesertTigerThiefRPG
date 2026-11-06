# Motor de acciones y protocolo semántico

| Campo | Valor |
|---|---|
| Estado | Implementado parcialmente; proyecto y ampliación causal aprobados para implementar |
| Versión | 0.20 |
| Fecha | 2026-10-26 |

> Ampliación implementada el 2026-10-26: Acampar usa una API core
> propia, definida en [su sección 3](../acciones/07-acampar.md#3-regla-del-núcleo).
> No se introduce en los registros y opciones específicos de Viajar.

> La ampliación implementada de [eventos narrativos, sección 4](./23-eventos-narrativos.md#4-puntos-de-intervención-del-motor)
> añade cesiones explícitas antes/después de unidades y un cierre reglado por
> evento. Conserva el estado reanudable y no autoriza deshacer resultados ya
> confirmados. Su sustitución inicial solo afecta al relato de llegada.

## Revisión implementada el 2026-10-24: Motor reanudable vigente

Las firmas activas son iniciar, avanzar y responder del contrato 06, sección 9. PasoAccion contiene Escena, Decision o Finalizada. Las particulares solo hacen el chequeo de la sección 8; las tarjetas resuelven sus consecuencias. Los estados, pronósticos y trazas del cuerpo histórico no son APIs activas.

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

## 1. Propósito

Este documento es la fuente canónica del motor de `desert-tiger-core` que ejecuta
acciones generales y particulares. Define su protocolo interno con la aplicación
sin repetir la fachada pública del frontend ni las reglas particulares de una
acción.

Desarrolla la separación establecida por el
[diseño técnico general](../diseno-tecnico-general.md): el núcleo es la autoridad
sobre las reglas, la aplicación posee la sesión activa y el frontend solo envía
intenciones y presenta resultados.

## 2. Alcance y exclusiones

Este componente comprende:

- las solicitudes semánticas para iniciar o continuar acciones;
- el estado reanudable de una acción general;
- los proyectos de acción y sus registros intencionales neutrales;
- la ejecución de acciones particulares reutilizables;
- decisiones, registros causales ordenados, resultados y duración acumulada;
- validación común para acciones del jugador y de los NPC;
- acceso controlado al azar.

El alcance vigente ejecuta `Viajar` tanto para el jugador como para el NPC
mínimo. Sus reglas físicas pertenecen a
[`acciones/02-viajar-expediciones.md`](../acciones/02-viajar-expediciones.md) y
su lógica rectora pertenece a
[`acciones/04-proyecto-humano-viajar.md`](../acciones/04-proyecto-humano-viajar.md).

El motor incorpora el
[contrato de acciones particulares](../acciones/particulares/00-contrato-resolucion.md),
las fichas terrestres, de supervivencia y de navegación, y las
[operaciones atómicas](./11-operaciones-mundo.md).

Quedan fuera el contenido y cálculo de cada acción concreta, la redacción y el
formato visible del tiempo, la selección visual, Lua y el dibujo de viñetas. El
alcance inicial no admite cancelar técnicamente una ejecución ya comenzada,
deshacer ni ejecución simultánea; el frontend sí puede abandonar una selección
antes de enviar la solicitud. Esto no impide la conclusión reglada
`AbandonoVoluntario` de `Viajar`: elegirla es una transición de gameplay que
conserva tiempo y consecuencias, no un rollback ni el descarte de la ejecución.

## 3. Responsabilidades y límites

`desert-tiger-core` define las reglas y el estado opaco de cada acción. Valida
las solicitudes contra el Mundo real, ejecuta transiciones, aplica consecuencias
y produce salidas neutrales. Las acciones particulares también viven en el
núcleo y solo pueden cambiar el estado mediante sus operaciones regladas.

El núcleo define el tipo opaco `EjecucionAccion`; la única instancia concreta
pendiente pertenece a la Partida, según el
[contrato de sesión](./04-sesion-aplicacion.md). Es un valor poseído formado por
datos e IDs y nunca mantiene referencias prestadas al Mundo o a la fuente de
azar entre llamadas.

Dentro de una acción general, el núcleo conserva además su proyecto: propósito,
propuestas realmente consideradas, compromiso, subobjetivo, interpretación y
replanteamiento. La aplicación no deduce esas transiciones desde texto o trazas;
las recibe como registros semánticos neutrales. Una transición del proyecto no
finge que se ejecutó una acción particular.

La aplicación construye una solicitud interna vinculada al actor jugador o a un
NPC con política que la fase NPC haya considerado elegible. Ambas solicitudes
recorren la misma entrada y las mismas validaciones del motor. La API pública que
origina esas solicitudes se define únicamente en el documento de sesión.

## 4. Datos, entradas y salidas

La API pública reside en el módulo `acciones` de `desert-tiger-core`. Los campos
de `EjecucionAccion` son privados; la aplicación solo puede iniciarla o
reanudarla mediante las funciones de este módulo.

El núcleo ofrece consultas para calcular acciones y objetivos disponibles. La
aplicación los proyecta en su vista, pero la consulta no reemplaza la validación
al confirmar una solicitud.

En el primer incremento la solicitud es un enum cerrado con una única variante:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolicitudAccion {
    Viajar {
        actor: IdActor,
        destino: IdLugar,
    },
}
```

Se añadirá una variante al incorporar una acción general nueva; no existe un ID
de acción ni un catálogo dinámico en este incremento.

No contiene teclas, colores, posiciones de widgets ni coordenadas de tiles. El
jugador y un NPC usan esta misma forma de solicitud.

Una **respuesta a decisión** contiene el identificador de la decisión pendiente
y el identificador de una de sus opciones. Cada solicitud de decisión producida
por el motor contiene:

- un identificador válido solamente para esa espera;
- contexto y hechos conocidos suficientes para explicar por qué se decide;
- la instantánea de progreso correspondiente a ese punto causal;
- una lista no vacía de opciones, cada una con identificador, significado
  semántico y efectos conocidos por los viajeros.

Los identificadores del motor son locales a una `EjecucionAccion` y no se
publican directamente a un frontend:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdDecision(u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdOpcionDecision(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RespuestaDecision {
    pub decision: IdDecision,
    pub opcion: IdOpcionDecision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolicitudDecision {
    pub id: IdDecision,
    pub contexto: ContextoDecision,
    pub situacion: Option<ReferenciaSituacion>,
    pub progreso: ProgresoAccion,
    pub opciones: Vec<OpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpcionDecision {
    pub id: IdOpcionDecision,
    pub contenido: OpcionSemantica,
    pub efectos_conocidos: Vec<HechoCausalVisible>,
}
```

Los campos internos y los constructores de `IdDecision` e
`IdOpcionDecision` son privados: otros crates solo pueden conservar y reenviar
valores emitidos por el motor. En cambio, los campos mostrados como `pub` en
`RespuestaDecision`, `SolicitudDecision` y `OpcionDecision` permanecen públicos;
la aplicación debe leer la solicitud y construir una respuesta usando esos IDs
opacos. Dentro de cada ejecución, las decisiones reciben un ordinal convertido
mediante `u32::try_from` y cada opción recibe su posición, también comprobada.
Reutilizar el mismo entero privado en otra ejecución no vuelve vigente un ID
antiguo: estos identificadores solo protegen la espera recibida junto con esa
`EjecucionAccion`.
`desert-tiger-app` asigna además IDs monotónicos de vista y conserva la
correspondencia mientras una decisión se expone; esa capa es la que impide
colisiones entre acciones ante un frontend.

`ContextoDecision` y `OpcionSemantica` son enums cerrados por acción general; sus
variantes concretas pertenecen a la documentación de esa acción. Expresan hechos
y alternativas, no frases localizadas ni instrucciones de interfaz. La
aplicación resuelve nombres visibles y redacta la decisión, pero no deduce
efectos adicionales. Los constructores internos garantizan que las opciones
tengan IDs distintos, que la lista contenga al menos dos y que todo hecho
publicado sea visible para los participantes en ese punto.

Las variantes pueden expresar propuestas de plan y medio, compromiso,
preparación, ritmo, replanteamiento o abandono. Una opción de propuesta de viaje
conserva una referencia tipada estable a la propuesta vigente y su proyección
visible completa. Esa proyección contiene, como mínimo, proponente o aportante,
estrategia, configuración prevista de medios y sus fuentes, recorrido
provisional conocido, requisitos y fuentes de preparación conocidos,
beneficios, costes, exposición e incertidumbres conocidos y precondiciones
satisfechas o pendientes admitidas por el alcance. Los requisitos cualitativos
no son todavía la categoría `Ligera`, `Suficiente` o `Cargada`, que se decide
después de adoptar propuesta y ritmo. La aplicación puede resolver nombres y
redactar esos campos, pero no reconstruir la propuesta mediante otra consulta.

Ni la propuesta ni `efectos_conocidos` publican el resultado futuro, hechos
ocultos o una posibilidad como si ya fuera un hecho consumado. Una respuesta
selecciona el ID de la opción emitida; no reenvía una propuesta reconstruida por
el llamador.

Una acción general puede abrir una **situación dramática** y conservarla a través
de varias transiciones o decisiones. La referencia estable que acompaña esos
registros tiene esta forma conceptual:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdSituacion(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenciaSituacion {
    pub id: IdSituacion,
    pub clave: ClaveSituacionViaje,
}
```

`ClaveSituacionViaje` es un enum cerrado de contextos concretos definidos por
[`Viajar`](../acciones/02-viajar-expediciones.md). Identifica, por ejemplo, una
travesía marítima favorecida por el tiempo o un ascenso por un paso abrupto; no
clasifica la emoción mediante valores genéricos como `Peligro`, `Aventura` o
`Suspenso`. El carácter emotivo surge del guion concreto que la aplicación
asocia con la situación y con los hechos ocurridos.

La clave no contiene una ruta, bytes de arte, dimensiones, estilos ni un
identificador del catálogo de contenido. El núcleo tampoco conoce su nombre
externo `lowerCamelCase`, su representación RON ni la ruta de un recurso. Otro
frontend puede asociar la misma variante con una imagen 2D, una escena 3D, arte
ASCII o ninguna representación.

El identificador distingue dos apariciones de la misma clase de situación dentro
de un viaje. Se genera de manera determinista dentro de la ejecución general,
sin usar `FuenteAzar`, y solo es válido junto con esa `EjecucionAccion`.

La acción general conserva además un `ContextoSituacionReglado` interno con los
hechos concretos que pueden afectar reglas. Ese contexto se entrega a cada acción
particular pertinente antes de resolverla. Su proyección pública contiene solo
los hechos que los participantes pueden observar; un hecho oculto puede influir
conforme a las reglas, pero no se revela por accidente en el registro visible.
La acción particular recibe esos hechos mecánicos, no `ClaveSituacionViaje`, y
por ello la misma regla reutilizable sirve bajo contextos dramáticos distintos.

### Cronología causal

La fuente canónica del comportamiento observable de `Viajar` es
[`03-cronologia-causal-viaje.md`](../acciones/03-cronologia-causal-viaje.md).
Este componente garantiza su frontera neutral: no entrega una frase genérica
por etapa, sino hechos causalmente ordenados que la aplicación puede narrar sin
reconstruir reglas.

El progreso que sale del núcleo tiene la forma equivalente a:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgresoAccion {
    pub lugar_confirmado: IdLugar,
    pub siguiente_lugar: Option<IdLugar>,
    pub destino_final: IdLugar,
    pub duracion_transcurrida: Duracion,
}
```

`duracion_transcurrida` comienza en cero, solo crece mediante suma comprobada y
representa el punto exacto de la simulación al que pertenece el registro. El
núcleo no incluye una segunda fecha ni llama a `Mundo::avanzar_fecha`; la
aplicación completa la instantánea con la fecha proyectada mediante el calendario
común y la fecha capturada al iniciar la acción.

Las emisiones causales usan conceptualmente esta suma tipada:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdMomentoCausal(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistroProyecto {
    Proposito { hechos: Vec<HechoCausalVisible> },
    Propuesta { hechos: Vec<HechoCausalVisible> },
    Compromiso { hechos: Vec<HechoCausalVisible> },
    Subobjetivo { hechos: Vec<HechoCausalVisible> },
    Interpretacion { hechos: Vec<HechoCausalVisible> },
    Replanteamiento { hechos: Vec<HechoCausalVisible> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistroCronologico {
    Proyecto {
        registro: RegistroProyecto,
        progreso: ProgresoAccion,
    },
    SituacionIniciada {
        situacion: ReferenciaSituacion,
        hechos_contextuales: Vec<HechoCausalVisible>,
        progreso: ProgresoAccion,
    },
    MomentoCausal {
        situacion: ReferenciaSituacion,
        momento: IdMomentoCausal,
        traza: TrazaAccionParticular,
        hechos_contextuales: Vec<HechoCausalVisible>,
        progreso: ProgresoAccion,
    },
    SituacionConcluida {
        situacion: ReferenciaSituacion,
        hechos: Vec<HechoCausalVisible>,
        progreso: ProgresoAccion,
    },
    ConclusionViaje {
        conclusion: ConclusionViaje,
        hechos: Vec<HechoCausalVisible>,
        progreso: ProgresoAccion,
    },
}
```

Las variantes muestran la categoría conceptual; sus formas concretas de
`Viajar` añaden IDs semánticos, proponente o aportante, estrategia, fuente de
medio, subobjetivo, causa de revisión y alternativas conforme al
[contrato del proyecto](../acciones/04-proyecto-humano-viajar.md). No contienen
prosa. `RegistroProyecto::Proposito` es la única apertura semántica de la acción;
no se emite además otra variante cronológica de apertura. `ConclusionViaje` es
una variante cronológica separada que la cierra. Un `RegistroProyecto` puede
abrir o hacer avanzar una situación, pero no
posee `IdMomentoCausal`, tirada, duración u operaciones si ninguna acción
particular se ejecutó.

`TrazaAccionParticular`, `HechoCausalVisible` y `ConclusionViaje` son tipos
neutrales y cerrados del núcleo. Las fichas particulares son la autoridad sobre
las variantes concretas de una traza. Una traza conserva verbo, participantes,
factores realmente aplicados, resoluciones aplicadas, dificultad y tirada cuando
sean revelables, banda, duración y efectos transitorios o persistentes. Los
factores incluyen su fuente, eje causal y ajuste firmado; el núcleo filtra la
visibilidad antes de publicarlos. Un total que delatara un hecho oculto se
representa como incertidumbre, no como una cifra de la que la aplicación pueda
inferirlo.

Normalmente existe una sola resolución aplicada. Si una acción grupal ejecuta
una prueba real por participante, la traza conserva todas en orden y señala su
sujeto; esto no divide la invocación ni crea varios momentos causales.

`Viajar`, como acción general, decide cuándo comienza y termina cada situación y
qué invocaciones particulares pertenecen a ella. Una situación contiene uno o
más momentos causales y puede abarcar varias acciones particulares diferentes o
repetidas. Cada invocación que realmente llega a ejecutarse aporta exactamente
un `MomentoCausal` que posee su traza completa. Ese momento puede dar lugar a una
o varias viñetas y una viñeta puede relatar varios momentos contiguos únicamente
si todos continúan identificables y ordenados. Por tanto, la cobertura es uno a
uno entre invocación ejecutada y momento causal, no entre acción particular y
escena ni entre acción particular y viñeta.

La apertura y el desenlace de una situación pertenecen a la acción general. No
se fabrican una introducción y una consecuencia narrativas independientes para
cada verbo particular. Dos ejecuciones iguales producen dos `MomentoCausal`
distintos y ordenados, aunque el guion las integre en la misma situación.

Consultar elegibilidad, pronosticar, comparar candidatos, escoger responsable,
recalcular una ruta o validar antes de aceptar una solicitud no ejecuta una
acción particular y no produce un momento. La duración tampoco multiplica
ejecuciones: una única acción de varios días conserva un momento con su duración
exacta completa, aunque el guion pueda necesitar varias viñetas para presentarla.

Cuando el proyecto determina que los personajes realmente consideran una
propuesta, adoptan un plan, persiguen un subobjetivo o reinterpretan un
resultado, emite un `RegistroProyecto`. Esto vuelve observable la acción humana
sin convertir las consultas que la prepararon en acontecimientos ni inventar una traza
particular.

Duración, consumo ordinario o extraordinario, reservas obtenidas o perdidas,
fatiga, heridas, estado del medio y traslado se incorporan a la traza y a la
acción que los causó. No se ocultan ni generan por obligación un fragmento
administrativo separado. Si una carencia causa la ejecución de otra acción
particular, esa nueva ejecución recibe su propio momento causal.

Todo hecho que el guion vaya a expresar y que cambie dificultad, modificadores,
candidaturas, consumo, duración o consecuencias debe existir primero como dato
del contexto reglado de la situación y debe entregarse a las acciones
particulares afectadas. La aplicación puede redactar que el viento impulsa una
embarcación porque el núcleo entregó ese hecho y lo aplicó a `Navegar`; no puede
inventarlo después como explicación de un cálculo que nunca lo consideró. Los
detalles puramente literarios pueden variar entre viñetas siempre que no afirmen
una causa o un efecto mecánico inexistente.

No existe presupuesto ni máximo de situaciones, momentos o viñetas. La acción
general no toma muestras, no suprime pasos rutinarios y no sustituye varias
ejecuciones por un montaje que altere lo que el jugador puede aprender de las
reglas.

Cada llamada aceptada avanza hasta la próxima frontera y devuelve un tramo:

```text
TramoAccion
├── emisiones ordenadas
│   ├── Cronologia(RegistroCronologico)
│   └── ResultadoSemantico(ResultadoSemantico)
└── cierre
    ├── EsperandoDecision(SolicitudDecision)
    └── Completada(FinalizacionAccion)
```

Las emisiones pueden incluir:

- **registros cronológicos**, con situaciones, hechos, momentos causales, trazas
  y progreso que la aplicación convierte en un guion de viñetas;
- **resultados semánticos estructurados**, que identifican qué cambió en el
  dominio sin incluir instrucciones, coordenadas ni tiles de presentación.

La solicitud de decisión y la finalización son cierres mutuamente excluyentes
del tramo, no emisiones narrativas. Un rechazo es un error recuperable de la
llamada y nunca forma parte del tramo ni finge que la acción ocurrió. Un
resultado desfavorable de una acción ya iniciada continúa siendo una
finalización válida, no un rechazo.

La estructura pública del tramo será:

```rust
pub struct TramoAccion {
    pub emisiones: Vec<EmisionAccion>,
    pub cierre: CierreTramo,
}

pub enum EmisionAccion {
    Cronologia(RegistroCronologico),
    ResultadoSemantico(ResultadoSemantico),
}

pub enum CierreTramo {
    EsperandoDecision(SolicitudDecision),
    Completada(FinalizacionAccion),
}
```

Los constructores internos rechazan una cronología incompleta, una situación
cerrada o referenciada fuera de orden, un `IdMomentoCausal` repetido, una
invocación sin momento, factores duplicados por eje o un progreso incompatible
con los cambios confirmados. Los demás tipos públicos de salida iniciales son:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResultadoSemantico {
    ActoresTrasladados {
        actores: Vec<IdActor>,
        origen: IdLugar,
        destino: IdLugar,
    },
    CaracteristicaActorAnadida {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    CaracteristicaActorQuitada {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FinalizacionAccion {
    pub duracion: Duracion,
}
```

`ResultadoSemantico` describe una consecuencia que el núcleo ya confirmó; nunca
es una orden para que la aplicación la aplique otra vez. Las variantes de
Característica solo se emiten para altas o bajas efectivas; una operación
idempotente no produce otra emisión. En
`ActoresTrasladados`, `actores` es una secuencia no vacía y sin duplicados. Para
un viaje del jugador contiene primero al jugador y después a los integrantes del
Grupo en su orden canónico; para un NPC independiente contiene solamente a ese
Actor. Todos los actores enumerados estaban en `origen` y ya se encuentran en
`destino` cuando el motor devuelve la emisión.

El motor obtiene esos campos directamente de `CambiosConfirmados` después de
que `Mundo::aplicar_lote` confirma la secuencia; no vuelve a consultar el Mundo
para reconstruirlos.

Cada etapa que traslada al Grupo puede producir un `ActoresTrasladados`; un viaje
con varias etapas puede contener varios en orden causal. Un ascenso costoso puede
emitir además un alta de `fatigado`; ambos cambios pertenecen al lote confirmado
por esa acción particular y aparecen en su consecuencia. Un ascenso fallido emite
el alta de `herido`, no traslada y aun así cierra normalmente la acción. La
aplicación usa los resultados para coordinar subsistemas y corroborar la traza,
pero no vuelve a ejecutar los cambios.

`FinalizacionAccion` contiene únicamente la duración total. El cierre ya expresa
que la acción terminó y las emisiones ya describen sus consecuencias, por lo que
repetir el destino, los actores o un indicador de éxito crearía una segunda
fuente de verdad. La representación de `Duracion` pertenece al
[calendario](./06-calendario-duracion.md).

Los registros y decisiones no contienen prosa, arte, estilos, coordenadas de
tiles ni tipos de Ratatui. El núcleo y la acción general entregan los contextos,
hechos, factores, trazas y resultados autoritativos; la aplicación resuelve los
nombres visibles y redacta el texto dramático mediante guiones hardcodeados de
cada situación concreta, sin reinterpretar las reglas. La terminal decide
composición, marco, detalle y animación, pero no redacta ni deduce causas. La
distribución de responsabilidades y la revelación ordenada pertenecen al
[contrato de sesión](./04-sesion-aplicacion.md).

## 5. Ejecución y estado reanudable

El módulo expone conceptualmente estas firmas:

```rust
pub struct EjecucionAccion {
    estado: EstadoEjecucion,
}

pub struct InicioAccion {
    pub ejecucion: EjecucionAccion,
    pub tramo: TramoAccion,
}

pub fn iniciar(
    mundo: &mut Mundo,
    solicitud: SolicitudAccion,
    azar: &mut dyn FuenteAzar,
) -> Result<InicioAccion, ErrorMotor>;

pub fn responder(
    mundo: &mut Mundo,
    ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision,
    azar: &mut dyn FuenteAzar,
) -> Result<TramoAccion, ErrorMotor>;
```

`EjecucionAccion` es un tipo público para que `desert-tiger-app` pueda poseerlo
y devolverlo al motor, pero es opaco: `EstadoEjecucion`, el campo y todos sus
constructores permanecen privados al módulo de acciones. La aplicación no puede
inspeccionar, clonar, fabricar ni serializar una ejecución.

`InicioAccion` siempre devuelve la ejecución junto con su primer tramo. Si el
cierre ya es `Completada`, la aplicación descarta inmediatamente esa ejecución;
si queda esperando, la conserva. `responder` la recibe por préstamo mutable: un
rechazo recuperable garantiza que tanto la ejecución como el Mundo y el azar
quedan sin cambios. Al completar, la aplicación descarta la ejecución.

Al comenzar una acción general, el núcleo vuelve a validar actor, acción,
objetivos y condiciones actuales. Si son válidos, crea su estado de ejecución y
avanza por sus transiciones. Cada transición puede consultar el Mundo, usar el
azar inyectado, invocar acciones particulares, aplicar consecuencias y emitir
registros cronológicos.

En una acción gobernada por proyecto, las transiciones siguen fases de dominio:
propósito, deliberación, compromiso, preparación o procura, ejecución por
subobjetivo, interpretación, replanteamiento y conclusión. No todas suspenden la
llamada ni producen una decisión; sí deben conservar su orden semántico. La
máquina anterior puede reemplazarse por completo durante el refactor mientras
mantenga esta API reanudable y sus garantías atómicas.

El avance se detiene cuando la acción:

1. necesita una decisión externa, en cuyo caso conserva su estado; o
2. termina y entrega la duración acumulada.

Una respuesta válida reanuda exactamente esa acción. Una respuesta no puede
seleccionar una opción de otra decisión ni reutilizar una decisión ya resuelta.
Mientras existe una acción pendiente no puede iniciarse otra en la misma sesión.

La unidad atómica es cada llamada aceptada de inicio o reanudación, no la acción
completa. Toda validación que pueda causar un rechazo recuperable ocurre antes
de consumir azar o confirmar cambios. Un rechazo deja intactos Mundo, ejecución,
duración y azar.

Una llamada aceptada puede consumir azar y confirmar consecuencias antes de una
decisión posterior. Esas extracciones y consecuencias no se revierten: el
alcance inicial no permite cancelar una acción comenzada ni promete rollback de
toda la acción. No existe un estado parcialmente observable durante la llamada
síncrona. Si ocurre un fallo inesperado después del punto de confirmación, se
devuelve un fallo interno fatal y la sesión deja de continuar; nunca se lo
degrada a rechazo recuperable.

Una **acción particular** es una operación de simulación reutilizable. Recibe su
contexto reglado, calcula un resultado y solicita operaciones persistentes; la
acción general las confirma mediante un lote. Toda invocación ejecutada aporta su
traza causal y su tiempo, aunque resulte automática, imposible o desfavorable;
no conoce la presentación ni permite evitar la validación de quien la invoca.

Las acciones particulares iniciales son funciones o tipos internos estáticos
invocados por `Viajar`. No necesitan IDs públicos ni un catálogo dinámico en
este incremento.

La duración comienza exactamente en `Duracion::CERO`. Cada aporte de una acción
particular se incorpora mediante `Duracion::checked_add`; un
`ErrorCalendario::Desbordamiento` se convierte en
`FalloMotor::DesbordamientoDuracion`. Las acciones particulares pueden modificar
el total durante la ejecución; el motor comunica tanto la duración aportada por
cada consecuencia como el acumulado de su `ProgresoAccion`, y conserva además la
duración final de la acción completa. El motor nunca llama a
`Mundo::avanzar_fecha`. La proyección pura, el avance único de la fecha y el
tratamiento especial de la duración de NPC se definen en
[`06-calendario-duracion.md`](./06-calendario-duracion.md) y
[`07-fase-npc.md`](./07-fase-npc.md).

## 6. Azar compartido y reproducible

El trait `FuenteAzar` pertenece al módulo `azar` de `desert-tiger-core`. La
Partida posee una implementación y presta `&mut dyn FuenteAzar` al motor. Su
operación mínima recibe un límite exclusivo distinto de cero y devuelve un
índice dentro de ese rango o un error explícito:

```rust
pub trait FuenteAzar {
    fn elegir_indice(
        &mut self,
        limite_exclusivo: NonZeroUsize,
    ) -> Result<usize, ErrorFuenteAzar>;
}
```

`ErrorFuenteAzar` es un error público opaco y sin datos de gameplay:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ErrorFuenteAzar;
```

Permite que una implementación indique que no pudo producir un índice. El motor
no interpreta una causa específica ni expone detalles de la implementación; una
fuente programada de pruebas puede usarlo, por ejemplo, cuando se agota su
secuencia prefijada.

Un índice fuera de rango o un fallo de la fuente es fatal, porque no expresa un
rechazo válido de la acción. El motor no crea generadores ocultos ni consulta
fuentes globales. Acciones generales y particulares consumen la misma capacidad
sin conocer su implementación concreta.

La implementación de producción elegirá cada índice con distribución uniforme
en `0..limite_exclusivo`. Cada acción concreta define y documenta sus propias
selecciones; cada decisión automática de un NPC hace una extracción separada y
uniforme sobre las opciones en el orden estable entregado por la acción. La
composición, el orden y la cantidad de candidaturas de la acción vigente
pertenecen al [documento de `Viajar`](../acciones/02-viajar-expediciones.md); el motor común
solo aporta y controla la fuente.

En pruebas se inyectará una fuente prefijada o programada. Con igual estado,
entradas y respuestas de azar, una transición produce el mismo resultado.

Solamente una llamada aceptada al motor o la resolución automática de una
decisión NPC puede consumir azar. Consultar disponibilidad o construir una vista
no lo consume. Las extracciones de una llamada aceptada
no se revierten. Los identificadores de decisiones y opciones se generan
mediante contadores o datos deterministas, nunca con esta fuente, para no alterar
la secuencia de gameplay. Los IDs de fragmentos expuestos pertenecen a la sesión.

Cada acción concreta documentará los puntos y el orden de sus extracciones. Una
selección lógica realiza una llamada a `elegir_indice`; la implementación de la
fuente puede consumir internamente los bits que necesite sin alterar ese orden
observable. Las pruebas utilizarán una fuente programada que también detecte
extracciones faltantes o adicionales.

## 7. Contratos e invariantes

- Solo el núcleo valida y aplica consecuencias de gameplay.
- Jugador y NPC atraviesan la misma entrada, validaciones y lógica de acción.
- El actor jugador con la característica `muerto` no puede iniciar una acción.
- Una decisión pertenece a una única acción pendiente y solo acepta sus opciones
  vigentes.
- Una respuesta inválida no modifica el Mundo ni descarta la acción pendiente.
- Cada registro conserva el orden causal en que fue emitido por la lógica.
- Un proyecto posee propósito antes de formular propuestas y compromiso antes
  de ejecutar un subobjetivo de su plan.
- Propósito, propuesta, compromiso, subobjetivo, interpretación y replanteamiento
  se emiten como registros neutrales, nunca como momentos causales ficticios.
- Toda acción particular ejecutada dentro de un proyecto responde al subobjetivo
  vigente y conserva responsable e intención.
- Interpretar o replantear no vuelve a ejecutar, tirar ni aplicar la resolución
  que produjo el cambio.
- Ningún medio se propone o utiliza sin la fuente que exige la acción general.
- La acción general es la autoridad sobre las situaciones dramáticas; una
  situación puede abarcar varias acciones particulares y varias viñetas.
- Cada invocación particular ejecutada posee exactamente un momento causal con
  una traza completa; ningún momento se pierde aunque se comparta una situación
  o una viñeta con otros.
- Las consultas puras y los candidatos no ejecutados no producen un momento
  causal.
- Una acción particular multidiaria sigue siendo una sola invocación y declara
  su duración exacta completa; su presentación puede ocupar varias viñetas.
- No existe límite, presupuesto, muestreo ni supresión de situaciones, momentos
  o viñetas.
- Todo hecho narrativo con efecto mecánico forma parte del contexto reglado antes
  de resolver la acción particular afectada.
- Ningún enum genérico de tono decide la emoción; esta surge de la situación
  concreta y de su guion en la aplicación.
- Ningún dato administrativo observable se oculta: permanece acoplado a la
  resolución o consecuencia que lo causó.
- Cada tramo termina exactamente en una decisión o en una finalización.
- Un rechazo no aparece dentro de la secuencia de emisiones.
- El tiempo acumulado nunca es negativo ni altera directamente la fecha.
- Ninguna regla consulta Ratatui, teclas, modos visuales o coordenadas del mapa.
- Los tiles no son una entrada ni una fuente de verdad para una acción, aunque
  su resultado pueda causar después una actualización visual.
- Todo azar que afecte reglas entra por la fuente inyectada.

## 8. Errores y validaciones

El canal de error distingue obligatoriamente:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorMotor {
    Rechazo(RechazoAccion),
    Fatal(FalloMotor),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RechazoAccion {
    ActorDesconocido {
        actor: IdActor,
    },
    DestinoDesconocido {
        destino: IdLugar,
    },
    JugadorMuerto {
        actor: IdActor,
    },
    DestinoEsLugarActual {
        lugar: IdLugar,
    },
    ViajeIndependienteDeIntegranteGrupo {
        actor: IdActor,
    },
    DestinoSinRutaConectada {
        origen: IdLugar,
        destino: IdLugar,
    },
    SinPropuestaViajeViable {
        origen: IdLugar,
        destino: IdLugar,
    },
    DecisionNoVigente {
        decision: IdDecision,
    },
    OpcionNoVigente {
        decision: IdDecision,
        opcion: IdOpcionDecision,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FalloMotor {
    FuenteAzar(ErrorFuenteAzar),
    IndiceAzarFueraDeRango {
        indice: usize,
        limite_exclusivo: NonZeroUsize,
    },
    DesbordamientoDuracion,
    InvarianteRota,
}
```

`DestinoSinRutaConectada` significa que el grafo no conecta origen y destino.
`SinPropuestaViajeViable` significa que sí pertenecen al mismo componente, pero
el preflight no puede formular ninguna propuesta válida con conocimiento,
capacidades, medios y fuentes vigentes. Ninguna de las dos variantes se emite
después de crear el proyecto. Durante una ejecución solo una identidad de
decisión u opción ajena a la espera vigente es un rechazo; la falta reglada de
continuaciones concluye como `Interrupcion` y una fuente planificada que
desaparece sin una consecuencia causal previa es `FalloMotor::InvarianteRota`.

`RechazoAccion` enumera condiciones esperables del dominio y puede mostrarse o
traducirse sin invalidar la Partida. `FalloMotor` cubre una invariante rota, un
fallo de `FuenteAzar` y cualquier error inesperado posterior al punto de
confirmación; obliga a llevar la sesión a `Fatal`.

Las siete primeras variantes de `RechazoAccion` corresponden a las validaciones
aprobadas para iniciar `Viajar`. `JugadorMuerto` se usa solamente cuando el Actor
es el jugador; el efecto general de `muerto` sobre otros NPC continúa aplazado.
`DestinoSinRutaConectada` contiene el origen lógico vigente y el destino
solicitado; expresa un grafo válido sin conexión entre ambos, no un error al
cargar Relaciones. `SinPropuestaViajeViable` distingue el caso conectado que no
ofrece una manera válida de viajar con los hechos conocidos. Las dos últimas
variantes protegen la reanudación directa del motor frente a una decisión
antigua o una opción que no pertenece a la espera vigente. La fachada puede
interceptarlas y convertirlas en sus errores
homónimos, pero el núcleo conserva la validación. Mientras existe una acción
pendiente, la fachada evita iniciar otra antes de llamar al motor.

`FuenteAzar` representa un error devuelto por la capacidad inyectada;
`IndiceAzarFueraDeRango` representa que esa capacidad devolvió `Ok` con un valor
que no pertenece a `0..limite_exclusivo`. `DesbordamientoDuracion` indica que una
suma de incrementos reglados no puede representarse. `InvarianteRota` agrupa los
estados internos imposibles y un fallo inesperado de una operación de Mundo
después de haber superado sus validaciones. La aplicación no necesita tratar
estas causas de forma distinta: todas invalidan la continuidad de la sesión.

Al solicitar comenzar `Viajar`, el motor aplica un preflight puro en el orden
aprobado por su documento: validaciones comunes de Actor y Grupo, validaciones
de origen y destino y construcción de propuestas conocidas. La ausencia de al
menos una propuesta válida devuelve `SinPropuestaViajeViable`, mientras la
ausencia de conexión topológica devuelve `DestinoSinRutaConectada`, antes de
crear `EjecucionAccion`, `ProyectoViaje` o
emitir `Proposito`. Consultar disponibilidad o rechazar una solicitud no muta el
Mundo ni consume azar o tiempo. A partir de la aceptación y de la emisión de
`Proposito`, perder toda continuación concluye como `Interrupcion`; no vuelve a
la frontera de rechazo. Al final de una acción ya aceptada, cualquier
`ErrorTraslado` se convierte en
`ErrorMotor::Fatal(FalloMotor::InvarianteRota)`, porque todas sus condiciones
previsibles ya fueron validadas y la sesión no permite otra mutación intermedia.

Una solicitud rechazada no crea una acción ni consume azar o tiempo. Una
respuesta inválida conserva el punto de espera. Los diagnósticos identifican la
causa sin provocar `panic` en caminos normales. Un fallo interno fatal se propaga a la
aplicación, que detiene la sesión y permite al frontend restaurar la terminal.

## 9. Pruebas y criterios de aceptación

Las pruebas unitarias usarán un Mundo mínimo, una acción de prueba y azar
controlado para comprobar:

- inicio, espera, respuesta y finalización de una acción con varias transiciones;
- distinción estructural entre emisiones, decisión, finalización y rechazo;
- orden de propósito, propuestas, compromiso, subobjetivo, interpretación y
  replanteamiento, seguido por su conclusión cronológica separada, sin analizar
  prosa;
- deliberación y compromiso visibles mediante registros de proyecto sin crear
  momentos causales ni consumir azar por sus consultas puras;
- preflight sin propuestas que rechaza antes de crear ejecución o proyecto y
  antes de emitir propósito, frente a ausencia de continuación posterior que
  produce una conclusión `Interrupcion`;
- opción de propuesta que conserva su referencia estable, proponente,
  estrategia, medios y fuentes, pronóstico, requisitos y fuentes cualitativas de
  preparación, intercambios, incertidumbre y precondiciones visibles sin una
  consulta reconstructiva de la aplicación;
- acción particular enlazada al subobjetivo, responsable e intención vigentes;
- interpretación que no duplica tirada, duración, operación ni momento causal;
- replanteamiento material con cero continuaciones que finaliza como
  `Interrupcion`, con una que emite `[continuar, abandonar]` y con varias que
  conserva todas en orden canónico seguidas por `abandonar`;
- filtrado de una propuesta de medio sin fuente durante el preflight y rechazo
  `SinPropuestaViajeViable` cuando no queda ninguna; pérdida causada después de
  aceptar que replantea o interrumpe, y desaparición sin causa que resulta fatal;
- suma de duraciones aportadas por más de una acción particular;
- correspondencia entre `N` invocaciones particulares ejecutadas y `N` momentos
  causales con trazas completas, también cuando `N` supera seis y cuando dos
  ejecuciones consecutivas son iguales;
- una situación con varias acciones particulares y una acción particular
  presentada mediante varias viñetas, sin duplicar su momento causal;
- varias situaciones que reutilizan el mismo tipo de acción particular bajo
  contextos dramáticos distintos, sin trasladar ese contexto a la regla
  reutilizable;
- ausencia de registros para consultas de elegibilidad, pronósticos, candidatos
  descartados y validaciones previas;
- una acción multidiaria representada por un solo momento con su duración
  exacta, aunque el guion lo proyecte en varias viñetas;
- contabilidad, estados, reservas y traslado dentro de la traza causal
  correspondiente, sin omisión ni fragmento administrativo obligatorio;
- un hecho contextual que modifica una regla presente antes de la resolución,
  aplicado por la acción particular y conservado en su traza visible;
- rechazo de una cronología con una invocación sin momento, un momento duplicado
  o una referencia a una situación cerrada;
- resolución reproducible de cada incertidumbre reglada con una fuente
  programada y ausencia de una tabla o «suceso de viaje» genérico adicional;
- rechazo de una opción o identificador de decisión incorrectos sin mutación;
- rechazo esperado sin consumo de azar ni tiempo;
- conservación de las extracciones y consecuencias de una transición aceptada;
- ausencia de consumo de azar al consultar disponibilidad o rechazar una
  solicitud;
- conversión de error o índice inválido de la fuente en fallo fatal;
- distinción entre `FuenteAzar` e `IndiceAzarFueraDeRango`;
- conversión de un desbordamiento de duración en fallo fatal;
- rechazo de una acción nueva del actor jugador con `muerto`;
- igualdad del recorrido de validación para solicitudes de jugador y NPC;
- conservación sin alteraciones de las referencias de situación y registros
  causales emitidos por la acción concreta al atravesar el tramo;
- emisión de la situación de rumbo incierto únicamente ante una interrupción
  efectiva por orientación fallida;
- igualdad de reglas, azar, duración y consecuencias al cambiar solamente la
  redacción de una misma situación;
- ausencia de arte, nombres visibles del contenido y tipos de terminal en
  `RegistroCronologico` y `ClaveSituacionViaje`;
- una emisión `ActoresTrasladados` por cada etapa realmente confirmada y ausencia
  de esa variante ante una escalada fallida;
- emisión de cambios efectivos de Característica después de confirmar el lote;
- secuencia no vacía y sin duplicados para `actores`, con jugador y Grupo o el
  único NPC según corresponda;
- imposibilidad de usar un resultado semántico como orden para aplicar por
  segunda vez la consecuencia;
- finalización que contiene solamente la duración total calculada por la acción,
  sin duplicar destino, actores ni éxito;
- cobertura individual de todas las variantes de `RechazoAccion`, conservando
  Mundo, ejecución, duración y azar en cada rechazo.

Una prueba de integración sin frontend completará una acción mediante la fachada
de aplicación y verificará sus situaciones, momentos y viñetas ordenadas,
factores visibles, consecuencias, progreso y duración.
El componente será aceptable cuando ninguna dependencia o tipo de terminal sea
necesario para realizar ese recorrido y el frontend no deba recalcular ninguna
regla para presentarlo.

## 10. Preguntas bloqueantes

Ninguna en este componente.

## 11. Decisiones aplazadas

- cancelación de una acción ya iniciada;
- ejecución simultánea y deshacer;
- proyección de resultados semánticos sobre regiones o tiles;
- acciones generales distintas de `Viajar`;
- taxonomía compartida de conclusiones desfavorables para futuras acciones
  generales; `Viajar` conserva por ahora sus conclusiones tipadas propias;
- efecto general de `muerto` sobre NPC distintos del actor jugador.

