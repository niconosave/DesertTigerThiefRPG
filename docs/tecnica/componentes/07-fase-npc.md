# Fase de NPC

| Campo | Valor |
|---|---|
| Estado | Implementado parcialmente; proyecto y ampliación causal aprobados para implementar |
| Versión | 0.14 |
| Fecha | 2026-10-20 |

## 1. Propósito y alcance

Este documento define la fase síncrona que ocurre después de aplicar a la fecha
la duración de una acción completada por el jugador. Fija qué política se
evalúa, cómo se ejecuta su solicitud mediante el motor común, cómo se resuelven
decisiones automáticas y qué resultado recibe el frontend.

El primer incremento admite exactamente una política evaluable perteneciente a
un NPC independiente, la evalúa una vez y procesa como máximo una solicitud. Las
políticas asociadas al jugador o a integrantes actuales del Grupo se conservan
validadas, pero no son evaluables. El contenido que contenga cero o más de una
política independiente no satisface el alcance inicial y se rechaza al cargar.

La [ampliación aprobada de `Viajar`](../acciones/02-viajar-expediciones.md#13-viajes-de-npc)
puede responder una secuencia finita de decisiones dentro de esa única
solicitud. Cada transición usa la misma ejecución reanudable y cada acción
particular conserva su traza, aunque el frontend no abra viñetas para los NPC.
El NPC recorre además el mismo
[proyecto humano](../acciones/04-proyecto-humano-viajar.md): formula las mismas
propuestas válidas, adopta un plan y conserva subobjetivos e interpretaciones;
solo cambia que esta fase responde al azar por ahora.

Quedan fuera varias políticas independientes, prioridades, acciones simultáneas,
memoria de IA y redacción visual. La API interna de la política pertenece a
[08-integracion-lua.md](./08-integracion-lua.md) y las reglas de la acción al
[motor](./05-motor-acciones.md).

## 2. Responsabilidades

desert-tiger-app filtra la política, crea su contexto, interpreta su respuesta,
coordina el motor y construye un resumen estructurado. desert-tiger-core valida
y ejecuta la solicitud. Lua no muta el Mundo y el frontend no evalúa scripts,
elige resultados ni vuelve a aplicar consecuencias.

La pertenencia al Grupo se consulta en el Mundo vigente. No se copia a una marca
del catálogo. Antes de crear una máquina Lua, la aplicación comprueba que el
Actor vinculado:

- existe;
- no es el actor jugador;
- no pertenece al Grupo.

Una política omitida por cualquiera de las dos últimas condiciones no consume
memoria Lua, azar ni tiempo y no genera una entrada de resumen.

La coordinación se concentra en una función interna sin acceso al mapa ni al
frontend:

~~~rust
pub(crate) fn ejecutar_fase_npc(
    mundo: &mut Mundo,
    politicas: &[PoliticaLuaValidada],
    claves: &CatalogoClaves,
    azar: &mut dyn FuenteAzar,
) -> Result<Option<ResumenNpc>, FalloFaseNpc>;
~~~

El slice usa el orden canónico de Actor provisto por ContenidoValidado. La
función no conserva referencias dentro de su resultado.

## 3. Secuencia normativa

La fase empieza únicamente después de que Mundo::avanzar_fecha terminó con éxito
para la acción del jugador. La política observa el valor vigente de
Mundo::fecha(). Esta fase nunca invoca avanzar_fecha.

La secuencia exacta es:

1. localizar la única política independiente evaluable;
2. comprobar su elegibilidad contra el Mundo vigente;
3. evaluarla una vez mediante evaluar_politica;
4. si devuelve Ninguna, terminar sin resumen;
5. si devuelve una solicitud, llamar a acciones::iniciar con el Mundo y la
   única FuenteAzar de la Partida;
6. consumir internamente las emisiones del tramo en orden, añadir cada
   `RegistroCronologico` a la `CronologiaNpc` transitoria y corroborar sus
   resultados semánticos;
7. si el cierre espera una decisión, elegir una opción, llamar a
   acciones::responder y volver al paso 6 con la misma ejecución;
8. repetir hasta obtener `Completada`, sin publicar viñetas interactivas;
9. no aplicar la `Duracion` recibida a la fecha;
10. comprobar la consecuencia confirmada y, desde la cronología cerrada,
    construir el `ResumenNpc` estructurado y su proyección breve;
11. volver a `Disponible`.

Las situaciones dramáticas y sus viñetas no se publican para un NPC ni se
convierten en ventanas interactivas. La fase consume y conserva hasta el cierre
la misma cronología neutral que produciría el jugador, incluida toda transición
de proyecto, situación, momento y conclusión. La posterior presentación
compacta no autoriza otra simulación ni omite pasos de ejecución: el NPC
atraviesa las mismas funciones, acciones particulares, hechos contextuales
reglados y consecuencias que el jugador. Solo cambia la forma de comunicarlos.

El registro conserva también qué propósito perseguía, qué plan adoptó y todo
replanteamiento que cambió materialmente el curso. La vista breve puede agrupar
acciones rutinarias, pero debe nombrar un cambio de curso o consecuencia
persistente; tampoco puede inventar una motivación distinta.

### Decisión automática

`Viajar` puede producir cero o varias decisiones sucesivas. La llamada que llega
a cada frontera consume primero las extracciones que pertenecen a la propia
acción. Si el tramo espera una decisión, la fase exige que la lista de opciones
sea no vacía, llama una vez a `FuenteAzar::elegir_indice` con su longitud como
límite exclusivo y selecciona el `IdOpcionDecision` situado en ese índice.
Construye una `RespuestaDecision` con el `IdDecision` vigente y reanuda la misma
`EjecucionAccion`; el procedimiento se repite ante otra espera.

Un fallo de la fuente, un índice exterior o un rechazo de una respuesta
construida por la propia aplicación indican una inconsistencia fatal. También
es fatal exceder el resguardo técnico de transiciones sin alcanzar otra
frontera válida. Ese resguardo detecta un posible bucle: no limita escenas ni
convierte una ejecución válida en éxito parcial. No se elige por texto, posición
visual ni clave de teclado.

La selección automática consume una extracción distinta y posterior a las que
realiza la propia acción. Si no aparece una decisión, la fase no realiza esa
extracción. El orden observable completo y las opciones vigentes están fijados
en la especificación de [Viajar](../acciones/02-viajar-expediciones.md); este componente no
duplica la composición de sus candidaturas ni las reglas de sus resultados.
`Abandonar` solo participa cuando el proyecto abrió una frontera segura de
replanteamiento; en ese caso es una opción semántica ordinaria de la lista
completa y puede ser elegida por la misma política aleatoria.

### Consecuencia confirmada

Cada etapa completada puede contener exactamente un:

~~~rust
ResultadoSemantico::ActoresTrasladados {
    actores: vec![actor_vinculado],
    origen: origen_de_la_etapa,
    destino: destino_de_la_etapa,
}
~~~

Cada resultado ya describe un traslado confirmado. La fase no llama a
`Mundo::trasladar` ni reconstruye la consecuencia comparando instantáneas. En un
viaje de varias etapas acepta varios traslados en orden: el origen del primero es
el Lugar inicial y cada origen siguiente coincide con el destino anterior. Todos
contienen únicamente al Actor vinculado. Los resultados de alta o baja de
Característica se aceptan cuando pertenecen al mismo NPC. Un Actor distinto, una
ruptura de continuidad o una ubicación final incoherente hacen fatal la sesión.

Una llegada exige al menos un traslado y que el último termine en el destino
solicitado. Una interrupción puede ocurrir antes del primer traslado o después de
varias etapas; el NPC permanece entonces en el último Lugar confirmado y la fase
produce `ViajeInterrumpido`. Esto es un desenlace reglado, no una solicitud
rechazada ni un fallo de coordinación.

La `Duracion` final demuestra que el recorrido terminó y se contrasta mediante
suma comprobada con las duraciones de sus momentos causales. Se conserva como
`duracion_simulada` del resumen estructurado y se vuelve visible como información
del viaje, pero nunca se aplica a la fecha. Una discrepancia de duración es una
invariante rota. La fecha anterior y posterior a toda la fase NPC debe ser
idéntica.

## 4. Registro interno y proyección breve

La coordinación de la fase mantiene primero una cronología poseída, neutral y
exhaustiva, nunca objetos mlua ni fragmentos interactivos:

~~~rust
pub(crate) struct CronologiaNpc {
    registros: Vec<RegistroCronologico>,
}
~~~

Cada emisión cronológica del motor se añade una sola vez y en el orden recibido.
`CronologiaNpc` existe hasta validar el cierre; permite probar paridad completa
de propósito, propuestas, compromiso, subobjetivos, situaciones, momentos,
interpretaciones, replanteamientos y conclusión. No se publica al frontend y se
descarta después de derivar el resumen estructurado:

~~~rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResumenNpc {
    entradas: Vec<EntradaResumenNpc>,
    duracion_simulada: Option<Duracion>,
}

impl ResumenNpc {
    pub(crate) fn entradas(&self) -> &[EntradaResumenNpc];
    pub(crate) fn duracion_simulada(&self) -> Option<Duracion>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EntradaResumenNpc {
    PropositoViaje {
        actor: IdActor,
        destino: IdLugar,
    },
    PlanViajeAdoptado {
        actor: IdActor,
        plan: ResumenPlanViajeNpc,
    },
    ViajeReplanteado {
        actor: IdActor,
        lugar: IdLugar,
        causa: CausaReplanteamientoNpc,
    },
    AccionParticular {
        actor: IdActor,
        resumen: ResumenAccionParticular,
    },
    ViajeCompletado {
        actor: IdActor,
        origen: IdLugar,
        destino: IdLugar,
    },
    ViajeAbandonado {
        actor: IdActor,
        lugar: IdLugar,
    },
    ViajeInterrumpido {
        actor: IdActor,
        lugar: IdLugar,
    },
    SolicitudViajeRechazada {
        actor: IdActor,
        destino: IdLugar,
        causa: RechazoAccion,
    },
    ErrorPolitica {
        diagnostico: DiagnosticoPoliticaNpc,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResumenPlanViajeNpc {
    pub estrategia: EstrategiaViajeNpc,
    pub medios: Vec<ResumenMedioViajeNpc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EstrategiaViajeNpc {
    Directo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResumenMedioViajeNpc {
    pub medio: TipoMedioViajeNpc,
    pub fuente: Option<FuenteMedioViajeNpc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TipoMedioViajeNpc {
    APie,
    Camellos,
    Bote,
    Barco,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FuenteMedioViajeNpc {
    AportadoPorActor {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    ServicioLocal {
        lugar: IdLugar,
        servicio: Caracteristica,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CausaReplanteamientoNpc {
    MedioNoDisponible,
    EtapaImposible,
    HechoDescubierto,
    ConsecuenciaMaterial,
    Crisis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResumenAccionParticular {
    pub accion: TipoAccionParticular,
    pub lugar: IdLugar,
    pub desenlace: DesenlaceParticularVisible,
    pub duracion: Duracion,
    pub consecuencias: Vec<ConsecuenciaVisibleNpc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EtapaFalloPoliticaNpc {
    Preparacion,
    Ejecucion,
    ConversionRespuesta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CodigoFalloPoliticaNpc {
    LimiteInstrucciones,
    LimiteMemoria,
    ArgumentoInvalido,
    ClaveVariableGlobalInvalida,
    VariableGlobalInexistente,
    TipoVariableGlobalIncorrecto,
    CaracteristicaInvalida,
    ClaveLugarInvalida,
    LugarInexistente,
    RespuestaInvalida,
    Ejecucion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiagnosticoPoliticaNpc {
    pub actor: IdActor,
    pub clave_actor: String,
    pub archivo_relativo: String,
    pub etapa: EtapaFalloPoliticaNpc,
    pub codigo: CodigoFalloPoliticaNpc,
    pub linea: Option<u32>,
    pub detalle: String,
}
~~~

`ResumenAccionParticular`, `DesenlaceParticularVisible` y
`ConsecuenciaVisibleNpc` son proyecciones neutrales cerradas de la traza del
motor. Conservan verbo, Lugar confirmado, desenlace, duración y consecuencias
principales; no conservan situación dramática, prosa, widgets ni un cálculo
nuevo. Cada `MomentoCausal` del motor produce exactamente una entrada
`AccionParticular`, incluso si es automático, repetido o dura cero días.

`PropositoViaje`, `PlanViajeAdoptado` y `ViajeReplanteado` son la proyección
compacta de registros de proyecto, no acciones particulares. `medios` conserva
el orden estable del plan; caminar usa `fuente: None`, mientras camellos, bote y
barco exigen `Some` con el Actor aportante o el servicio local que los hace
posibles. El plan adoptado después de un replanteamiento produce otra entrada
`PlanViajeAdoptado`; así la causa y la nueva decisión permanecen ordenadas sin
campos opcionales ambiguos.

`ResumenNpc` solo se construye dentro de la aplicación con una lista no vacía.
Un viaje que comenzó contiene primero `PropositoViaje`, luego cada plan y
replanteamiento material en su punto causal, todas sus entradas
`AccionParticular` y, al final, exactamente una de `ViajeCompletado`,
`ViajeAbandonado` o `ViajeInterrumpido`; posee además
`duracion_simulada: Some(...)`, igual a la suma comprobada de sus acciones y a
la duración final del motor. Una solicitud rechazada o una política fallida
produce una única entrada de su tipo y `duracion_simulada: None`, porque no
inició el proyecto ni ejecutó acciones particulares. Si Lua no solicita nada,
`VistaMundo.resumen_npc_reciente` es `None`, no un resumen vacío.

Antes de construir `VistaMundo`, la fachada transforma este resumen estructurado
en el [`VistaResumenNpc`](./04-sesion-aplicacion.md#7-vistas-y-salidas) breve que
recibe el frontend. La proyección:

- para un viaje iniciado, comienza por Actor, conclusión, propósito
  `AlcanzarDestino`, último Lugar y duración simulada total, sin presentarla como
  avance adicional de la fecha del Mundo;
- para una solicitud rechazada, comienza por Actor, destino solicitado y causa,
  sin inventar conclusión, último Lugar ni duración;
- para un error de política anterior a una solicitud, comienza por clave de
  Actor y código estable, sin exigir destino, Lugar ni duración inexistentes;
- indica las cantidades totales de acciones particulares y replanteamientos;
- si hubo replanteamientos, nombra solamente la causa del último y conserva la
  cantidad de los anteriores;
- si hubo consecuencias persistentes, nombra solamente la última en orden
  causal e indica cuántas adicionales quedaron confirmadas;
- agrega el desenlace dominante de las acciones rutinarias sin enumerarlas;
- nunca intenta expandir una lista de longitud proporcional al viaje.

`CronologiaNpc` y `ResumenNpc` siguen siendo la autoridad para pruebas y
auditoría. La frase visible es deliberadamente acotada y no pretende sustituir
un historial consultable, que queda aplazado.
La proyección breve no vuelve a ejecutar reglas, no cambia el orden causal y no
convierte una acción omitida del texto en una acción omitida de la simulación.

DiagnosticoPoliticaNpc es una proyección segura y estable del error interno de
integración. Su ruta procede del patrón ASCII validado
actors/<clave-actor>.ron; no expone PathBuf, mlua::Error, código fuente, trazas
ni tipos del runtime. detalle ya tiene escapados los controles conforme al
contrato de integración. La terminal decide cuánto mostrar.

`ViajeCompletado` se construye desde la cadena confirmada de traslados y conserva
su origen inicial y destino final. `ViajeAbandonado` y `ViajeInterrumpido`
conservan el último Lugar confirmado, exista o no alguna etapa previa; el primero
representa una decisión reglada y el segundo la ausencia de continuación válida.
Una solicitud rechazada conserva la causa devuelta por
acciones::iniciar, incluido `DestinoSinRutaConectada` cuando el grafo válido no
conecta origen y destino, o `SinPropuestaViajeViable` cuando existe conexión pero
ninguna propuesta válida. Las entradas de conclusión no duplican duración,
fecha, estilos, texto narrativo, nombre visible de Actor ni un booleano
redundante de éxito: la duración total pertenece a `ResumenNpc`, los demás datos
a las entradas particulares previas o al catálogo de presentación.

## 5. Política de errores

### Solicitud rechazada al iniciar

ErrorMotor::Rechazo al iniciar la solicitud NPC es una incidencia recuperable de
política o contenido en tiempo de partida. El motor garantiza que no consumió
azar, duración ni cambios. La fase:

- no reintenta;
- no crea ni presenta viñetas;
- añade SolicitudViajeRechazada;
- vuelve a Disponible.

No se devuelve como ErrorIntencion: la intención del jugador anterior ya se
completó y la incidencia pertenece al resumen de la fase NPC.

### Error de política durante la Partida

Las clases de integración que tienen una variante equivalente en
CodigoFalloPoliticaNpc son atribuibles al script y recuperables. La fase las
proyecta a ErrorPolitica, no llama al motor y vuelve a Disponible. La política
no se reintenta dentro de la misma fase ni queda deshabilitada para fases
futuras.

ClaseErrorLua::Infraestructura, una sintaxis inválida que reaparezca después de
la validación inicial o una etapa imposible son fallos internos fatales. Los
errores de sintaxis normales se detectan durante la carga y no permiten crear la
Partida.

### Fallos fatales

También vuelven fatal la sesión:

- todo ErrorMotor::Fatal, ocurra antes o después de una consecuencia;
- un fallo o índice inválido de la fuente durante la elección automática;
- cualquier ErrorMotor::Rechazo al responder la decisión vigente construida por
  la aplicación;
- una frontera, traza o consecuencia semántica que incumpla este contrato;
- la ausencia o multiplicidad inesperada de políticas elegibles en contenido ya
  validado.

No se promete rollback si una llamada aceptada ya confirmó una consecuencia. La
fase se detiene y entrega la causa al mecanismo fatal de sesión.

El canal interno distingue estas causas sin exponerlas como entrada del resumen:

~~~rust
pub(crate) enum FalloFaseNpc {
    Integracion(ErrorPoliticaLua),
    FuenteAzar(ErrorFuenteAzar),
    IndiceAzarFueraDeRango {
        indice: usize,
        limite_exclusivo: NonZeroUsize,
    },
    Motor(FalloMotor),
    RespuestaAutomaticaRechazada(RechazoAccion),
    InvarianteRota,
}
~~~

`FuenteAzar` representa un `Err` devuelto por la capacidad;
`IndiceAzarFueraDeRango` representa un `Ok(indice)` exterior al intervalo
solicitado. La fase distingue ambos casos antes de acceder a la lista de
opciones y nunca indexa con un valor sin validar.

## 6. Invariantes

- La fase evalúa exactamente una vez la única política independiente inicial.
- Cada política produce como máximo una solicitud.
- Jugador e integrantes del Grupo nunca se evalúan.
- Una ausencia de solicitud no produce resumen.
- Un error recuperable produce exactamente una entrada y ningún cambio.
- Toda solicitud atraviesa el motor común.
- Una decisión se resuelve únicamente con la fuente de la Partida.
- La fase no expone viñetas NPC.
- Cada acción particular ejecutada aporta una entrada de resumen en el mismo
  orden; no hay límite, muestreo ni fusión de entradas.
- Todo viaje iniciado conserva primero su propósito y cada plan o
  replanteamiento material en el orden en que ocurrió, sin atribuirles una
  acción particular ficticia.
- Camellos, bote y barco conservan una fuente en el resumen; caminar no recibe
  una procedencia artificial.
- La presentación compacta no sustituye la ejecución de una acción particular,
  el contexto reglado de una situación ni una consecuencia del motor.
- Cada etapa confirmada produce un traslado del Actor vinculado; una llegada
  termina en el destino solicitado y un abandono o interrupción conserva la
  última ubicación alcanzada.
- La fase nunca adelanta la fecha.
- Un fallo fatal no se degrada a resumen recuperable.

## 7. Pruebas y aceptación

Las pruebas sin terminal real cubrirán:

- observación de la fecha ya avanzada;
- condición falsa que no produce solicitud ni resumen;
- recorrido del
  [fixture vertical](./03-contenido.md#fixture-vertical-oficial), con viaje NPC,
  resumen exitoso y fecha intacta;
- omisión del jugador y de un compañero con política sin crear entorno Lua,
  consumir azar ni producir resumen;
- rechazo de cero o varias políticas independientes durante la carga;
- una sola evaluación y como máximo una solicitud;
- nueva validación de la solicitud por el motor;
- rechazo inicial recuperable sin mutación, azar ni tiempo;
- proyección de cada código recuperable de política;
- regreso a Disponible y posibilidad de evaluar de nuevo en una fase futura;
- conversión de infraestructura o sintaxis reaparecida en fallo fatal;
- selección automática reproducible por índice, con el ID de decisión vigente;
- orden exacto de las extracciones definido por la acción: las de
  `acciones::iniciar` y, solo si el cierre espera, la selección automática de
  una opción;
- ejecución NPC de un recorrido sin decisión, sin extracción automática de
  opción;
- ejecución NPC de un recorrido con decisión, con la extracción de opción
  posterior a las de la acción y reanudación mediante el ID vigente;
- rechazos recuperables `DestinoSinRutaConectada` y
  `SinPropuestaViajeViable` sin mutación ni resumen de éxito;
- tratamiento fatal de fuente fallida, índice exterior, respuesta rechazada y
  ciclo de transiciones inválido;
- cronología NPC exhaustiva con toda transición del proyecto y una entrada
  estructurada por cada momento causal, sin abrir presentación interactiva;
- mismo propósito, propuestas y plan que recibiría el jugador ante igual Mundo
  y azar, con selección automática únicamente en las fronteras de decisión;
- resumen que conserva propósito, compromiso y replanteamientos materiales sin
  fabricar momentos particulares para esas transiciones;
- recepción de una cadena de uno o varios traslados con el NPC, continuidad de
  etapas y destino final correctos al llegar;
- ascenso fallido en la primera etapa con `herido` y sin traslado, y fallo
  posterior que conserva todos los traslados previos antes del resumen de
  interrupción;
- abandono automático elegido en una frontera segura de replanteamiento, con
  resumen distinto de una interrupción y sin revertir tiempo ni traslados, y
  ausencia de esa opción durante una acción particular pendiente;
- tratamiento fatal de una etapa duplicada, una ruptura de continuidad o un
  resultado con Actor discordante;
- ausencia de una segunda aplicación del traslado;
- suma comprobada de duración coincidente con el cierre, presencia del total en
  el resumen visible y ausencia de un segundo avance temporal;
- igualdad de Mundo::fecha() antes y después de la fase completa;
- viaje con más de seis acciones particulares que conserva cronología y entradas
  internas en orden y añade una única conclusión, mientras su vista breve nombra
  propósito/destino y duración, cuenta acciones y replanteamientos, conserva
  solo los últimos datos materiales y no crece proporcionalmente al recorrido;
- una situación NPC con varias acciones particulares que conserva una entrada
  por cada traza, aunque omita por completo la puesta en escena dramática;
- solicitud rechazada o política fallida con una única entrada y ninguna entrada
  particular ficticia, sin conclusión, Lugar, duración o propósito inventados
  cuando sus datos no existen;
- ausencia de tipos mlua en el registro y de inferencias regladas nuevas en la
  vista breve redactada.

El componente será aceptable cuando las salidas recuperables —sin solicitud,
viaje rechazado, viaje interrumpido y error de política— y el viaje exitoso
vuelvan de forma determinista al jugador, mientras toda inconsistencia interna
detenga la sesión.

## 8. Preguntas bloqueantes

- Ninguna.

## 9. Decisiones aplazadas

- varias políticas NPC independientes y su orden;
- si varias políticas observan una instantánea común o consecuencias anteriores;
- límites totales de una fase con varios NPC;
- planificación avanzada, memoria interna y prioridades dinámicas;
- reintentos, deshabilitación persistente o contadores de fallos de una política;
- acciones NPC distintas de Viajar.
