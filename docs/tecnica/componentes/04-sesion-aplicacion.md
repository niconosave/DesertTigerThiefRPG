# Sesión y fachada de `desert-tiger-app`

| Campo | Valor |
|---|---|
| Estado | Implementado parcialmente; proyecto y ampliación causal aprobados para implementar |
| Versión | 0.26 |
| Fecha | 2026-10-28 |

> Ampliación implementada el 2026-10-26: fichas, fase Acampando e
> intenciones se definen en [Acampar](../acciones/07-acampar.md#5-proyección-de-fichas);
> la finalización compartida con Viajar, en su sección 6.3.

## Revisión implementada el 2026-10-24: Sesión y vistas del viaje activo

Partida retiene EjecucionAccion e IdPaso en escenas y decisiones. La vista transmite tiempo_transcurrido como texto, fecha proyectada y participantes congelados. No publica pronosticos_viaje. Al finalizar confirma fecha y resto una vez y ejecuta una fase NPC.

Desde el 2026-10-27 ambas vistas conservan también el relato breve y el detalle
común por separado; sus tipos vigentes y la proyección se definen en la
[sección 11 del contrato de Viajar](../acciones/06-implementacion-viajar.md#11-registros-semánticos-redacción-y-privacidad).

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

> Ampliación implementada el 2026-10-28: `VistaLugar.simbolo: Option<char>`
> conserva el símbolo opcional validado por el [contrato de contenido](./03-contenido.md).
> Es presentación; el estado reglado de Lugar no cambia.

## 1. Propósito y autoridad

Este documento es la fuente canónica para la sesión activa y la fachada que
`desert-tiger-app` ofrece a cualquier frontend. Define:

- la composición y propiedad de una `Partida`;
- las fases coordinadas por la aplicación;
- las intenciones que puede enviar un frontend;
- las vistas y salidas que recibe;
- las categorías de error observables en esa frontera.

No define las estructuras internas de `Mundo`, las reglas de las acciones, el
esquema de contenido, la API de Lua ni la composición visual de la terminal.
Estos contratos pertenecen a sus documentos específicos.

## 2. Alcance inicial

La primera fachada implementable cubrirá el recorrido confirmado en el
[diseño general](../../diseno-general.md#9-primer-incremento-jugable):

```text
crear Partida desde contenido validado
→ consultar la vista del mundo
→ comenzar Viajar y seleccionar destino
→ presentar propósito, deliberación y compromiso con un plan
→ presentar sus viñetas y resolver decisiones
→ cerrar la última viñeta
→ avanzar la fecha una vez
→ procesar la fase de un NPC mediante Lua
→ entregar su resumen
→ volver a admitir acciones del jugador
```

Quedan fuera de este incremento las demás acciones del jugador, guardado, la
gestión simultánea de varias Partidas por el ejecutable, ejecución asíncrona y
comunicación entre procesos.

## 3. Dependencias y límites

```text
desert-tiger-terminal
        ↓ intenciones / vistas / salidas
desert-tiger-app
        ↓ operaciones de dominio
desert-tiger-core
```

La fachada pública no expone tipos de Ratatui, Crossterm o `mlua`. El frontend
no recibe acceso mutable al Mundo, al mapa, a la acción pendiente, a Lua ni a la
fuente de azar; tampoco necesita depender directamente de
`desert-tiger-core`.

Toda mutación reglada atraviesa la API del núcleo. La aplicación coordina esos
cambios y posee el estado de presentación persistente durante la Partida.

## 4. Composición de `Partida`

El ejecutable inicial administra una sola Partida jugable a la vez. El tipo no
es un singleton, no usa estado global y permite que varias instancias
independientes coexistan en pruebas o en futuros adaptadores. Cada Partida
conceptualmente posee:

```text
Partida
├── Mundo                         única instancia del estado reglado
├── EstadoMapa                    matriz mutable de claves de tipo visual
├── ContenidoValidado             handle compartido a definiciones inmutables
├── EstadoSesion                  una única variante interna válida
└── FuenteAzar                    controlable en pruebas
```

`desert-tiger-core` define el tipo `Mundo` y sus operaciones; la Partida posee
su única instancia concreta. El contenido validado puede compartirse de forma
inmutable, pero el Mundo y el EstadoMapa son independientes para cada Partida.
`Partida::nueva` clona el handle barato de `ContenidoValidado`, respaldado por
`Arc`, y accede mediante él al catálogo de presentación, las claves y las
políticas. No conserva un préstamo con el tiempo de vida del argumento, no
clona cada política Lua y no duplica esos catálogos en campos paralelos.

El `IdMapa` del Mundo debe resolver al EstadoMapa asociado al construir la
Partida. `EstadoMapa` contiene las dimensiones constantes del mapa, una clave de
tipo visual por celda y el conjunto inmutable de tipos declarados por el contenido.
La leyenda del archivo ya fue resuelta durante la carga: ni sus símbolos ASCII
de autoría ni los glifos de un frontend forman parte de las celdas de ejecución.
Una actualización visual sustituye la clave de una celda por otra del conjunto;
no cambia las dimensiones en el primer incremento. El núcleo no accede a esa
matriz. La proyección automática de cambios semánticos sobre tiles queda
aplazada; el mapa nunca se usa como entrada de una regla.

`desert-tiger-core` define el tipo opaco `EjecucionAccion`; la Partida posee su
única instancia concreta mientras una acción esté suspendida. El motor no guarda
referencias al Mundo ni a la FuenteAzar: ambas capacidades se le prestan solo
durante una llamada.

Para impedir combinaciones imposibles entre fases, el estado almacena una sola
variante y la instantánea ya construida que corresponde a esa variante. Su forma
conceptual es:

```text
EstadoSesion
├── Disponible {
│     vista: VistaMundo
│   }
├── Presentando {
│     vista_actual: VistaFragmento,
│     fragmentos_restantes,
│     fecha_inicio_accion,
│     cierre_pendiente
│   }
├── EsperandoDecision {
│     ejecucion,
│     vista: VistaDecision,
│     fecha_inicio_accion,
│     correspondencia_opciones
│   }
└── Fatal { vista: VistaFatal }

CierrePendiente
├── Decision { ejecucion, decision }
└── Completada { finalizacion }
```

`Presentando` posee exactamente un fragmento visible; la cola restante puede
estar vacía. La ejecución solo existe en `EsperandoDecision` o dentro de un
cierre `Decision` oculto detrás de fragmentos narrativos todavía no confirmados.
Una finalización nunca conserva una ejecución. El resumen NPC reciente permanece
dentro de la `VistaMundo` disponible hasta que la aplicación acepta la siguiente
acción del jugador.

`fecha_inicio_accion` es una copia inmutable de `Mundo::fecha()` tomada al aceptar
la acción. Acompaña una sola vez al estado de sesión vigente durante todas sus
fases pendientes y solo sirve para proyectar la fecha de cada registro; no
constituye otra fecha vigente ni se modifica al presentar una viñeta.

Toda consulta, correspondencia o conversión que pueda impedir construir la
próxima vista se resuelve durante la transición hacia el estado, nunca dentro de
`vista()`. Esto incluye disponibilidad, proyección de catálogo y asignación de
IDs visibles. En la creación, una incoherencia se devuelve como
`CodigoErrorCrearPartida::ContenidoInconsistente`; en una transición posterior se
guarda `Fatal`. Solo después de completar esas operaciones se sustituye la
variante estable. `vista(&self)` se limita a clonar la instantánea almacenada:
no consulta el Mundo o los catálogos, no asigna IDs, no consume azar y no puede
fallar.

## 5. Fases observables

Las fases estables que un frontend puede observar son:

```text
Disponible(VistaMundo)
PresentandoAccion(VistaFragmento)
EsperandoDecision(VistaDecision)
Fatal(VistaFatal)
```

El avance de fecha y la fase NPC son operaciones internas síncronas, no pantallas
ni fases estables. Mientras se ejecutan no se procesa otra intención. Al
terminar, la fachada devuelve `Disponible` con el resumen NPC reciente dentro de
`VistaMundo` y las acciones nuevamente habilitadas. No existe una fase, pantalla
ni intención para cerrar el resumen. Un fallo interno que impida garantizar
consistencia lleva a `Fatal`, que no acepta nuevas intenciones de juego.

La normalización de `InicioAccion` o de cada tramo posterior sigue esta
secuencia:

```text
recibir tramo
→ conservar la ejecución solo si el cierre espera una decisión
→ recorrer sus emisiones en orden:
  registro de proyecto → resolver nombres y construir su presentación
  otro registro cronológico → resolver nombres, proyectar fecha y construir vista
  resultado semántico → procesarlo internamente y corroborar su consecuencia
→ guardar su cierre: decisión o finalización
→ exponer el primer fragmento
→ cada continuación consume solamente un fragmento confirmado
→ al vaciarse la cola, exponer la decisión
  o, si la frontera es finalización:
  aplicar `finalizacion.duracion` mediante `Mundo::avanzar_fecha`
  → ejecutar fase NPC → volver a Disponible con resumen
```

Si un tramo no contiene fragmentos, su frontera se procesa inmediatamente. Esto
es válido para una decisión general que todavía no ejecutó una acción particular,
pero no permite omitir una ejecución ya realizada. La aplicación conserva las
situaciones, los momentos causales y las trazas recibidas en su orden. Por cada
momento construye una o varias viñetas del guion correspondiente; también puede
integrar momentos contiguos en una misma viñeta cuando todos siguen explícitos y
ordenados. No impone un límite de situaciones o viñetas, no toma muestras y no
descarta ejecuciones automáticas, repetidas o rutinarias. La secuencia canónica
se define en
[`03-cronologia-causal-viaje.md`](../acciones/03-cronologia-causal-viaje.md).

Propósito, propuesta, compromiso, subobjetivo, interpretación y replanteamiento
se redactan desde los registros neutrales del núcleo. Pueden usar las mismas
fases `PresentandoAccion` y `EsperandoDecision`: no requieren otra pantalla ni
permiten que la aplicación invente un plan a partir de la ruta o de la prosa.

La aplicación contiene el narrador hardcodeado de `Viajar`. Elige la redacción
dramática concreta a partir de los registros de proyecto,
`ClaveSituacionViaje`, los hechos contextuales y las trazas ya filtradas por el
motor; puede expresar aventura, alivio, libertad,
esfuerzo o peligro sin convertir esos matices en un enum genérico de tonos.
Resuelve `IdActor` e `IdLugar` contra el catálogo de presentación para usar
nombres visibles, pero no vuelve a calcular dificultad, modificadores, banda,
duración, consumo, reservas ni consecuencias. Una referencia que no resuelva en
un contenido validado es `EstadoInconsistente`, no una licencia para mostrar la
clave técnica o inventar una frase genérica.

El narrador puede añadir detalles literarios que no alteren la simulación. Si una
afirmación explica o modifica una regla —por ejemplo, que un viento favorable
redujo la dificultad de navegar— el hecho debe haber llegado en el contexto
reglado y en la traza; la aplicación solo lo verbaliza. Nunca crea después una
causa mecánica para embellecer un resultado.

Para cada progreso recibido calcula de forma pura y comprobada:

```text
fecha_proyectada = fecha_inicio_accion + duracion_transcurrida
```

Construir todas las instantáneas de una cola no las vuelve observables: solo se
publica `vista_actual`. La aplicación nunca consulta el `Mundo` ya mutado para
completar una vista anterior, pues una misma llamada del motor puede haber
confirmado consecuencias posteriores.

El avance se realiza una sola vez y únicamente al liberar una finalización de
la acción del jugador. La aplicación entrega la `Duracion` completa al Mundo;
no suma por separado sus meses o días. Si `avanzar_fecha` devuelve
`ErrorCalendario`, la sesión pasa a `Fatal` y la fase NPC no comienza. Como la
operación del Mundo es atómica, la fecha anterior permanece intacta.
La fecha resultante debe coincidir con la última fecha proyectada de la acción;
una discrepancia indica `EstadoInconsistente` y no se corrige en el frontend.

Los resultados semánticos vigentes describen traslados y altas o bajas de
Características de Actor. La aplicación los recibe como descripción de mutaciones
que el núcleo ya confirmó y nunca vuelve a aplicarlas. No producen por sí solos
otra pantalla: todo dato administrativo observable queda asociado a la traza del
momento que lo causó y aparece en alguna de las viñetas de su situación. Durante
la fase NPC se conserva además para validar la llegada o interrupción y construir
su resumen estructurado.

Contratos confirmados:

- solo `Disponible` admite comenzar una acción del jugador;
- `EstadoSesion` hace imposible conservar más de una ejecución general;
- una decisión solo acepta una opción de la espera vigente;
- el frontend debe confirmar que terminó de presentar el último fragmento antes
  del avance de fecha;
- una acción completada del jugador causa un único avance de fecha;
- la fecha proyectada puede cambiar entre vistas, pero no activa reglas, Lua ni
  NPC y no sustituye a `Mundo::fecha()` mientras la acción siga abierta;
- la fase NPC se procesa después y sus acciones no vuelven a adelantarla;
- terminado el procesamiento NPC, el control vuelve al jugador;
- confirmar un fragmento nunca llama al motor ni consume azar por sí mismo; si
  libera la finalización, la fase NPC que se ejecuta a continuación sí puede
  consumirlo conforme a su contrato;
- el [motor](./05-motor-acciones.md) rechaza una acción nueva del actor jugador
  con `muerto`, sin que eso constituya todavía una derrota formal.

El núcleo puede haber calculado un resultado antes de que se presente, pero la
fachada no publicará una vista o resumen que revele consecuencias posteriores al
punto visible actual.

## 6. Intenciones del frontend

La fachada aceptará exactamente estas clases de intención semántica, no eventos
físicos:

- **comenzar viaje:** identifica el `IdLugar` de destino; la aplicación añade el
  actor jugador;
- **responder decisión:** identifica la decisión vigente y la opción elegida;
- **continuar presentación:** identifica el fragmento vigente, confirma que el
  frontend ya terminó de presentarlo y solicita la siguiente transición
  observable.

La API pública tendrá esta forma. `vista` y `procesar` son métodos inherentes de
`Partida`, no funciones de un servicio separado:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntencionFrontend {
    ComenzarCampamento,
    AbandonarCampamento { campamento: IdCampamento },
    ComenzarViaje { destino: IdLugar },
    ResponderDecision {
        decision: IdDecisionVista,
        opcion: IdOpcionVista,
    },
    ContinuarPresentacion { fragmento: IdFragmento },
}

impl Partida {
    pub fn vista(&self) -> VistaSesion;

    pub fn procesar(
        &mut self,
        intencion: IntencionFrontend,
    ) -> Result<VistaSesion, ErrorIntencion>;
}
```

`IdLugar` está definido por el núcleo y es reexportado por la fachada.
`IdFragmento`, `IdDecisionVista` e `IdOpcionVista` pertenecen a la
aplicación:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdFragmento(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdDecisionVista(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IdOpcionVista(u32);
```

Los campos internos y constructores de estos tres newtypes son privados. La
Partida asigna fragmentos y decisiones con dos contadores monotónicos
independientes, iniciados en cero y
actualizados con aritmética comprobada. Cada ID se asigna cuando el fragmento o
la decisión pasa a ser el siguiente estado observable, no mientras permanece
oculto en una cola. Cada opción visible recibe su posición convertida mediante
`u32::try_from`; el entero no se publica. Un desbordamiento lleva la sesión a
`Fatal` antes de exponer el valor.

Cuando una `SolicitudDecision` del núcleo pasa a ser el siguiente estado
observable, la aplicación asigna un `IdDecisionVista` nuevo y conserva
privadamente la correspondencia entre cada `IdOpcionVista` y su
`IdOpcionDecision` del motor. Si todavía quedan fragmentos anteriores, conserva
la solicitud interna sin publicar IDs de vista. La intención pública se valida
contra la correspondencia vigente y recién entonces se construye la
`RespuestaDecision` interna. Por tanto, un ID de una acción anterior no puede
coincidir con la decisión vigente aunque el motor reutilizara un identificador
interno. El crate de terminal usa todos los IDs a través de
`desert-tiger-app` y no depende directamente de `core`.

Una confirmación repetida, antigua o distinta del fragmento vigente se rechaza
sin cambiar la cola. `ContinuarPresentacion` solo drena la cola y procesa la
frontera ya preparada; nunca reanuda el motor ni consume azar al validar la
presentación. Si libera una finalización, la fase NPC síncrona posterior puede
consumir azar. Solamente `ResponderDecision` puede pedir al motor el siguiente
tramo de la acción del jugador.

El frontend nunca proporciona la identidad del actor jugador. La aplicación la
vincula internamente. Tampoco envía teclas, posiciones de widgets ni coordenadas
de tiles.

Entrar o salir del modo visual de selección de destino no modifica el resumen en
la aplicación. Una `ComenzarViaje` aceptada sí lo descarta; si el usuario cancela
la selección antes de confirmar un destino, la vista disponible conserva el
resumen.

Las solicitudes NPC no pertenecen a esta API. La aplicación filtra primero las
políticas conforme a la fase NPC, excluye al jugador y a los integrantes del
Grupo, y crea cada solicitud elegible con el actor asociado antes de enviarla al
mismo motor.

La creación de una sesión será independiente del frontend y también será un
método inherente de `Partida`:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConfiguracionPartida {
    pub semilla: Option<u64>,
}

impl Partida {
    pub fn nueva(
        contenido: &ContenidoValidado,
        configuracion: ConfiguracionPartida,
    ) -> Result<Self, ErrorCrearPartida>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoErrorCrearPartida {
    ContenidoInconsistente,
    EntropiaNoDisponible,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCrearPartida {
    pub codigo: CodigoErrorCrearPartida,
    pub detalle: String,
}
```

Con `semilla: None`, la aplicación obtiene una semilla nueva de la fuente de
entropía del sistema; un fallo impide crear la Partida. Una semilla explícita se
usa en pruebas y diagnósticos reproducibles. No se deriva del reloj, de Lua ni
del frontend. La Partida crea y posee desde entonces una sola implementación de
`FuenteAzar`; las pruebas internas de aplicación también podrán inyectar una
fuente programada mediante un constructor no expuesto a frontends.

`ErrorCrearPartida` implementa `Display` y `std::error::Error`; `detalle`
es una línea segura y su texto no constituye una API estable. Un
`ErrorInstanciarContenido` se proyecta a `ContenidoInconsistente`; el fallo al
obtener entropía usa `EntropiaNoDisponible`. Durante la creación también se
calcula la disponibilidad inicial: un rechazo distinto de `JugadorMuerto`
indica contenido incoherente y se proyecta a `ContenidoInconsistente`. En una
transición posterior, la misma imposibilidad lleva la Partida existente a
`Fatal`. Una imposibilidad al proyectar la primera `VistaMundo` recibe también
`ContenidoInconsistente`; en estados posteriores usa
`CodigoFatalSesion::EstadoInconsistente`. Ninguno de los errores de creación
publica una Partida parcial.

## 7. Vistas y salidas

La ampliación del [contrato 19](./19-paisajes-sonoros.md#2-selección-del-paisaje)
expone características actuales de cada Lugar y sus vecinos directos del índice
`Conectado`, incluidas las conexiones declaradas desde el otro extremo. Son
datos neutrales: la aplicación no selecciona sonidos ni introduce tipos de
audio. Los IDs y el orden de Lugares se conservan en la proyección.

La fachada devuelve un `VistaSesion` poseído y discriminado por variante, no una
estructura con campos opcionales para todas las fases:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VistaSesion {
    Acampando(VistaCampamento),
    Disponible(VistaMundo),
    PresentandoAccion(VistaFragmento),
    EsperandoDecision(VistaDecision),
    Fatal(VistaFatal),
}
```

Todos los valores son instantáneas poseídas:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaMundo {
    pub mapa: VistaMapa,
    pub lugares: Vec<VistaLugar>,
    pub fecha: Fecha,
    pub lugar_jugador: IdLugar,
    pub jugador: VistaActor,
    pub companeros: Vec<VistaActor>,
    pub acciones: Vec<AccionDisponible>,
    pub causa_indisponibilidad: Option<CausaIndisponibilidad>,
    pub resumen_npc_reciente: Option<VistaResumenNpc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaseVistaResumenNpc {
    Normal,
    Incidencia,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaResumenNpc {
    pub clase: ClaseVistaResumenNpc,
    pub texto: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaMapa {
    pub ancho: usize,
    pub alto: usize,
    pub celdas: Vec<ClaveTipoTile>,
    pub tipos_declarados: Vec<ClaveTipoTile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaLugar {
    pub id: IdLugar,
    pub nombre: String,
    pub nombre_abreviado: String,
    pub simbolo: Option<char>,
    pub coordenada_central: CoordenadaMapa,
    pub coordenadas_rotulo: Vec<CoordenadaMapa>,
    pub caracteristicas: Vec<String>,
    pub conectados: Vec<IdLugar>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaActor {
    pub id: IdActor,
    pub nombre: String,
    pub retrato: RetratoActor,
    pub expresion: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccionDisponible {
    Acampar,
    Viajar {
        destinos: Vec<IdLugar>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausaIndisponibilidad {
    JugadorMuerto,
}
```

`fecha` es la fecha vigente y ya confirmada del Mundo cuando el jugador puede
elegir una acción. `lugares` conserva el orden canónico y contiene todos los Lugares. Cada
`coordenadas_rotulo` mantiene su correspondencia posicional con los caracteres
de `nombre`. `jugador` está separado de `companeros`, y estos últimos
conservan el orden del Grupo. Solo esos Actores se proyectan porque son los
únicos retratos que usa el frontend inicial; la ilustración de NPC independientes
sigue aplazada. El contenido garantiza que todos tienen nombre visible y
retrato; la clave externa del Actor nunca se usa como reemplazo de su nombre.

`RetratoActor` contiene gráficos neutrales validados, sin dependencia de
Ratatui ni secuencias ANSI. Su contrato está en
[`13-retratos-por-capas.md`](./13-retratos-por-capas.md).
`expresion` selecciona una clave del rostro para esta vista. La proyección
normal entrega `None`; al publicar una viñeta se aplica la señal elegida por
su narrador mediante `VistaMundo::mostrar_expresion_grupo`. Su ciclo de vida y
reserva visual están definidos en el contrato 13. No representa un estado
emocional del Mundo.

La celda `(x, y)` de `VistaMapa` ocupa `y * ancho + x` y
`celdas.len() == ancho * alto`. `tipos_declarados` incluye incluso las claves
ausentes de la matriz y usa orden lexicográfico. La vista no contiene símbolos
RON, glifos, estilos ni acceso al `EstadoMapa`.

Si el jugador no tiene `muerto`, `acciones` contiene `Viajar` y después
`Acampar`. Los destinos de Viajar son el resultado, posiblemente vacío, de
`destinos_disponibles` en orden canónico. El Lugar actual, los Lugares
sin ruta `Conectado` viable con los medios actuales ya están excluidos por el
núcleo. Una lista vacía permite mostrar el control
deshabilitado, pero no confirmar un destino; Acampar sigue disponible. En ese caso
`causa_indisponibilidad` es `None`.

Si el jugador tiene `muerto`, `acciones` queda vacía y
`causa_indisponibilidad` es `Some(JugadorMuerto)`. Cualquier otro rechazo de
la consulta de disponibilidad contradice una Partida validada y lleva a
`Fatal`; el frontend no interpreta Características para producir estas
decisiones.

La `CronologiaNpc` exhaustiva, el `ResumenNpc` estructurado, su ciclo de fase y
sus errores siguen
definidos por la [fase NPC](./07-fase-npc.md#4-registro-interno-y-proyección-breve).
Antes de construir `VistaMundo`, la fachada la proyecta a un solo
`VistaResumenNpc`. Para un viaje iniciado, `texto` comienza por Actor, desenlace,
propósito/destino, último Lugar y duración simulada total, y luego comunica
cantidades y los últimos cambios materiales mediante agregación acotada; esa
duración es informativa y no modifica la fecha del Mundo. Un rechazo inicial
nombra Actor, destino solicitado y causa sin inventar conclusión, ubicación
final o duración. Una política fallida antes de solicitar usa la clave de Actor
y su código estable sin exigir datos de viaje inexistentes. `clase` permite al
frontend distinguir una incidencia sin analizar la frase. La fachada no expone
el error interno de Lua. Conforme al
[contrato de cronología](../acciones/03-cronologia-causal-viaje.md#9-viajes-de-npc),
la cronología conserva todos los registros del proyecto y el resumen interno
conserva una entrada ordenada por cada acción particular
realmente ejecutada, además del propósito, el plan y los replanteamientos
materialmente relevantes. La vista breve puede contar intentos rutinarios, pero
no puede ocultar que existieron consecuencias persistentes o cambios de curso:
nombra el último de cada clase y cuenta los anteriores. Esto no cambia
cuándo corre la fase NPC ni abre viñetas interactivas.

`desert-tiger-app` reexporta los tipos de dominio que aparecen en esta API
pública —incluidos `IdActor`, `IdLugar`, `IdSituacion`, `IdMomentoCausal`,
`ClaveSituacionViaje` y `RechazoAccion`—, además de sus propios tipos de contenido
y sesión. Un frontend importa toda esta frontera desde la fachada y no declara
una dependencia directa de `desert-tiger-core`.

Los otros estados usan:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RolPresentacion {
    Apertura,
    Deliberacion,
    Compromiso,
    PlanteamientoSituacion,
    DesarrolloSituacion,
    DesenlaceSituacion,
    Replanteamiento,
    Conclusion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VistaRegistroProyecto {
    Proposito,
    Propuesta,
    Compromiso,
    Subobjetivo,
    Interpretacion,
    Replanteamiento,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VistaReferenciaSituacion {
    pub id: IdSituacion,
    pub clave: ClaveSituacionViaje,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaMomentoCausal {
    pub id: IdMomentoCausal,
    pub detalle: VistaDetalleMecanico,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaReferenciaLugar {
    pub id: IdLugar,
    pub nombre: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaProgresoViaje {
    pub lugar_confirmado: VistaReferenciaLugar,
    pub siguiente_lugar: Option<VistaReferenciaLugar>,
    pub destino_final: VistaReferenciaLugar,
    pub duracion_transcurrida: Duracion,
    pub fecha_proyectada: Fecha,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFragmento {
    pub id: IdFragmento,
    pub rol: RolPresentacion,
    pub titulo: String,
    pub texto: String,
    pub proyecto: Option<VistaRegistroProyecto>,
    pub situacion: Option<VistaReferenciaSituacion>,
    pub momentos: Vec<VistaMomentoCausal>,
    pub progreso: VistaProgresoViaje,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaDecision {
    pub id: IdDecisionVista,
    pub rol: RolPresentacion,
    pub titulo: String,
    pub texto: String,
    pub detalle: Option<VistaDetalleDecision>,
    pub proyecto: Option<VistaRegistroProyecto>,
    pub situacion: Option<VistaReferenciaSituacion>,
    pub progreso: VistaProgresoViaje,
    pub opciones: Vec<VistaOpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaOpcionDecision {
    pub id: IdOpcionVista,
    pub etiqueta: String,
    pub detalle: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaDetalleDecision {
    pub lineas_contexto: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoFatalSesion {
    Motor,
    Calendario,
    IntegracionNpc,
    AzarNpc,
    CoordinacionNpc,
    EstadoInconsistente,
    ContadorAgotado,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VistaFatal {
    pub codigo: CodigoFatalSesion,
    pub mensaje: String,
}
```

`VistaDetalleMecanico` es la proyección estructurada de una traza del motor:
conserva verbo, responsable, factores visibles y sus ajustes, modo de resolución,
valores revelables, banda, duración y efectos. `VistaDetalleDecision` conserva
únicamente el contexto mecánico conocido y común a toda la decisión; cada
`VistaOpcionDecision::detalle` contiene solo beneficios, costes, procedencia,
precondiciones e incertidumbre propios de esa alternativa. Un mismo dato no se
duplica entre ambos niveles. Todos reemplazan IDs de Actor o Lugar por nombres y
referencias visibles sin alterar cifras, orden ni visibilidad; siguen los tipos
cerrados del [motor](./05-motor-acciones.md#cronología-causal) y no constituyen un
segundo reglamento de aplicación.

La clasificación de los registros intencionales es normativa y no depende del
texto que redacte el narrador:

| `VistaRegistroProyecto` | `RolPresentacion` obligatorio |
|---|---|
| `Proposito` | `Apertura` |
| `Propuesta` | `Deliberacion` |
| `Compromiso` | `Compromiso` |
| `Subobjetivo` | `PlanteamientoSituacion` |
| `Interpretacion` | `DesenlaceSituacion` |
| `Replanteamiento` | `Replanteamiento` |

Si `proyecto` es `Some`, esta tabla tiene precedencia aunque el fragmento
también refiera una situación. Con `proyecto: None`, la apertura de una
situación usa `PlanteamientoSituacion`, su desarrollo usa
`DesarrolloSituacion` y su cierre usa `DesenlaceSituacion`. La conclusión de
`Viajar`, que no es otro `VistaRegistroProyecto`, usa `Conclusion`.

Las decisiones siguen la frontera que resuelven: elegir una propuesta usa
`Deliberacion`; precisar ritmo, preparación o ruta del plan adoptado usa
`Compromiso`; decidir dentro de una situación o acción particular usa
`DesarrolloSituacion`; y elegir una continuación durante un replanteamiento usa
`Replanteamiento`. Esta correspondencia permite que cualquier frontend componga
la misma secuencia sin analizar `titulo` o `texto`.

`None` significa que ese nivel carece de desglose adicional. `Some` nunca puede
estar vacío: `lineas_contexto` contiene al menos una línea no vacía y un detalle
de opción contiene texto no vacío. Así, un frontend puede decidir si ofrece
`[D]` sin analizar cadenas ni conocer variantes internas.

`VistaMomentoCausal` identifica sin ambigüedad qué ejecución particular sustenta
la viñeta y conserva su detalle mecánico. Un mismo momento puede aparecer en
varias viñetas consecutivas de la misma situación cuando su presentación necesita
más espacio o desarrollo; una viñeta puede enumerar varios momentos contiguos si
el texto mantiene explícito su orden y sus resultados. El narrador no duplica ni
fabrica momentos: la unión de IDs distintos a lo largo de la situación coincide
exactamente con los `MomentoCausal` recibidos del núcleo.

Los registros de proyecto, `SituacionIniciada` y `SituacionConcluida`, cada
momento causal y la conclusión pueden producir una o varias
`VistaFragmento` según el guion hardcodeado. `proyecto` conserva qué transición
intencional sustenta el fragmento sin convertirla en un momento. No existe una
segunda apertura además de `VistaRegistroProyecto::Proposito`, que se presenta
con `RolPresentacion::Apertura`. No existe una
fórmula de dos fragmentos por acción particular. La aplicación redacta `titulo`,
`texto`, `etiqueta` y `detalle` a partir del proyecto, la situación, los hechos,
las trazas y el catálogo; puede unir o dividir la presentación sin omitir ningún
momento ni alterar su orden. Una acción
multidiaria conserva un solo momento con su duración completa, aunque ese momento
ocupe varias viñetas.

`VistaDecision` sustituye los IDs internos por los IDs de vista descritos en la
sección 6 y conserva el orden de opciones. `etiqueta` es una línea no vacía, sin
controles, de como máximo 40 columnas visibles según `unicode-width`; este
límite forma parte de la validación de la vista y garantiza que una opción
completa quepa en el ancho terminal mínimo. El `detalle` opcional explica el
intercambio conocido exclusivo de la opción sin anticipar el desenlace. La
aplicación reenvía `ClaveSituacionViaje` sin resolver arte y no pagina; la
paginación y el estado local que muestra u oculta el detalle pertenecen al
frontend. `RolPresentacion` describe la función estructural tanto de fragmentos
como de decisiones; no es un tono emocional ni obliga a que toda situación use
los tres roles intermedios.

`VistaProgresoViaje` sustituye cada `IdLugar` neutral del motor por una
`VistaReferenciaLugar` con el mismo ID y su nombre visible. La terminal usa el
ID para cámara, foco y marcador y usa el nombre ya resuelto para el marco; nunca
busca ese texto en una instantánea anterior de `VistaMundo`. De igual modo,
`VistaDecision::titulo` y `rol` llegan redactados o clasificados por la
aplicación, y `proyecto` conserva la transición intencional que sustenta la
decisión cuando existe.

La representación Rust vigente aplana cada `VistaReferenciaLugar` en los pares
`lugar_confirmado`/`nombre_lugar_confirmado`,
`siguiente_lugar`/`nombre_siguiente_lugar` y
`destino_final`/`nombre_destino_final`. Es la misma información y evita que el
frontend vuelva a consultar el Mundo. Del mismo modo, el primer vertical
proyecta cada `VistaMomentoCausal` como una línea de detalle ya filtrada dentro
de `VistaFragmento::detalle`; la terminal solo la pagina y nunca reconstruye una
tirada. Una migración futura puede conservar la estructura anidada del ejemplo
sin cambiar este contrato semántico.

`VistaFatal` se construye una sola vez al entrar en `EstadoSesion::Fatal` y se
clona sin cambios en consultas posteriores. `mensaje` es una sola línea segura,
sin código Lua, trazas, controles de terminal ni `Debug` sin filtrar. No incluye
el prefijo visual `Error:`; cada frontend lo añade conforme a su medio.

El mapeo fatal es exacto: un `ErrorMotor::Fatal` usa `Motor`; un
`ErrorCalendario`, `Calendario`; las variantes `Integracion`, `FuenteAzar`,
`IndiceAzarFueraDeRango` y `Motor` de `FalloFaseNpc` usan, respectivamente,
`IntegracionNpc`, `AzarNpc`, `AzarNpc` y `Motor`; tanto
`FalloFaseNpc::RespuestaAutomaticaRechazada` como
`FalloFaseNpc::InvarianteRota` usan `CoordinacionNpc`; una proyección imposible
usa `EstadoInconsistente`; tanto el desbordamiento de un contador de vista como
la imposibilidad de representar la posición de una opción mediante
`IdOpcionVista` se mapean a `ContadorAgotado`.

Mientras se presenta o decide una acción, la vista no expone el mapa ni una
proyección viva del Mundo. Así, una consecuencia que el núcleo ya hubiera
confirmado no puede revelar una nueva ubicación u otro resultado antes de que el
jugador termine de presentar los fragmentos anteriores. `VistaProgresoViaje` es
una instantánea histórica del punto visible, no acceso al estado vigente del
Mundo; el frontend la combina con su última base visual para mover cámara y
marcador sin observar emisiones futuras.

Una vista nunca expone colecciones mutables, el estado interno reanudable de una
acción, objetos Lua, la fuente de azar ni reglas que el frontend deba recalcular.

El motor puede producir registros cronológicos, solicitudes de decisión,
resultados semánticos y finalización. La aplicación conserva su orden y expone
solo la parte correspondiente a la fase visible. Un resultado semántico sirve
para coordinar subsistemas y corroborar la consecuencia asociada; no se presenta
como una pantalla contable por sí mismo. Color, posición, paginación, animación y
widget pertenecen al frontend.

## 8. Categorías de error

Después de crear la Partida, `procesar` utiliza `Err(ErrorIntencion)` solamente
para rechazos recuperables y conserva la vista vigente. Sus variantes mínimas
son fase incompatible, fragmento no vigente, decisión no vigente, opción no
vigente y acción rechazada por el núcleo.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorIntencion {
    FaseIncompatible,
    FragmentoNoVigente,
    DecisionNoVigente,
    OpcionNoVigente,
    AccionRechazada(RechazoAccion),
}
```

La precedencia se determina antes de llamar al motor:

1. si la clase de intención no corresponde a la variante actual de
   `EstadoSesion`, devuelve `FaseIncompatible`; toda intención recibida en
   `Fatal` sigue esta regla;
2. en `Presentando`, una `ContinuarPresentacion` compara después el ID con el
   fragmento visible y devuelve `FragmentoNoVigente` si difiere;
3. en `EsperandoDecision`, una `ResponderDecision` compara primero la decisión y
   luego busca la opción dentro de su correspondencia vigente, devolviendo
   `DecisionNoVigente` antes que `OpcionNoVigente`;
4. en `Disponible`, una `ComenzarViaje` se entrega al motor y un rechazo normal
   se devuelve como `AccionRechazada`.

Ninguno de estos rechazos reemplaza la instantánea almacenada ni modifica sus
IDs. Por ejemplo, un ID de fragmento antiguo enviado durante una decisión se
clasifica como `FaseIncompatible`, no como `FragmentoNoVigente`.

Un `ErrorMotor::Fatal` no se devuelve como `ErrorIntencion`: la aplicación guarda
`EstadoSesion::Fatal` y devuelve `Ok(VistaSesion::Fatal(...))`. Todas las llamadas
posteriores a `vista()` muestran esa misma fase y cualquier intención de juego
se rechaza. Los errores de carga o construcción ocurren antes y pertenecen al
resultado de crear la Partida.

Un `ErrorCalendario` producido al aplicar la finalización recibe el mismo
tratamiento fatal. No se devuelve como `ErrorIntencion`, no se intenta continuar
con la fase NPC y el diagnóstico conserva la causa para la vista fatal.

Cuando el motor devuelve `RechazoAccion::DecisionNoVigente` u
`OpcionNoVigente`, la fachada lo convierte respectivamente en
`ErrorIntencion::DecisionNoVigente` u `OpcionNoVigente`; no los envuelve otra vez
en `AccionRechazada`. Las demás variantes de `RechazoAccion` sí se conservan en
`AccionRechazada`. En los tres casos la espera y la vista vigentes permanecen
intactas.

- **Error de inicio:** lectura, deserialización, validación o construcción
  fallida. No se publica una Partida parcial.
- **Rechazo recuperable:** una intención no es válida para la fase o el estado
  actual. La sesión continúa utilizable y el componente que rechaza garantiza
  que no confirmó una mutación parcial.
- **Incidencia recuperable de NPC:** un error atribuible a una política o el
  rechazo inicial de su solicitud se registra dentro de `ResumenNpc` y se
  proyecta como `ClaseVistaResumenNpc::Incidencia`; no es un `ErrorIntencion`, no
  impide volver a `Disponible` y no se reintenta en esa fase.
- **Error fatal de sesión:** ya no puede garantizarse la consistencia. La
  Partida deja de aceptar intenciones de juego.

La fase NPC convierte en fatal todo fallo de infraestructura Lua, del motor o de
la fuente de azar y toda invariante rota. La fachada guarda entonces
`EstadoSesion::Fatal`; solo las incidencias que el documento de fase clasifica
expresamente como recuperables llegan al resumen.

Los errores físicos de entrada o dibujo pertenecen al frontend y no a esta
fachada.

## 9. Invariantes

- Una Partida contiene exactamente un Mundo y un EstadoMapa asociado.
- Cada celda de EstadoMapa contiene un tipo visual declarado y ninguna vista
  permite modificarla ni confunde ese tipo con un símbolo RON o un glifo.
- Todo cambio de gameplay atraviesa `desert-tiger-core`.
- El frontend nunca posee estado mutable del dominio ni del mapa de la Partida.
- Hay como máximo una `EjecucionAccion` y una frontera pendiente.
- Jugador y NPC atraviesan el mismo motor y las mismas validaciones.
- La aplicación conserva y redacta el orden de los registros de proyecto sin
  inferir propósito, plan o subobjetivo desde una ruta, una traza o una frase.
- Un registro de proyecto sin acción particular puede producir una viñeta, pero
  nunca un `VistaMomentoCausal` artificial.
- El tiempo avanza una vez por acción completada del jugador y nunca por una
  acción NPC.
- La fecha proyectada de una vista se deriva exclusivamente de la fecha capturada
  al iniciar la acción y de la duración acumulada entregada por el motor; no muta
  el Mundo.
- Cada ejecución particular recibida conserva exactamente un momento causal
  visible dentro de una situación; la aplicación puede distribuirlo entre
  viñetas, pero no limitarlo, duplicarlo ni omitirlo.
- Una situación puede contener varias ejecuciones y no existe una correspondencia
  obligatoria entre escena, acción particular y fragmento.
- Nombres y prosa proceden del catálogo, de la situación y de los hechos del
  motor; la aplicación no reinterpreta factores ni consecuencias.
- Un hecho narrativo con efecto mecánico solo se redacta si el núcleo lo entregó
  como parte del contexto reglado y de la traza correspondiente.
- No existe un enum de tono: la emoción surge del guion concreto de la situación.
- Ninguna intención nueva del jugador se acepta durante la fase NPC.
- Todo azar que afecte gameplay o decisiones NPC procede de la FuenteAzar de la
  Partida.
- Los identificadores de fragmentos se obtienen de un contador de la Partida, no
  de la FuenteAzar de gameplay.
- Consultar `vista()` y confirmar un fragmento presentado no consumen azar ni
  ejecutan gameplay; las fases regladas que una confirmación final habilite
  conservan su comportamiento propio.
- Un rechazo recuperable conserva una sesión válida; un error fatal impide
  continuarla.

## 10. Pruebas y aceptación

Las pruebas de aplicación, sin terminal real, cubrirán:

- creación de dos Partidas independientes desde el mismo contenido;
- conservación interna de un handle clonado de `ContenidoValidado`, sin
  préstamos al argumento de `nueva`, sin copiar políticas y sin compartir Mundo
  o EstadoMapa entre Partidas;
- funcionamiento de la Partida después de descartar el handle de
  `ContenidoValidado` que el llamador entregó a `nueva`;
- dos Partidas con el mismo contenido, la misma semilla y las mismas intenciones
  que producen el mismo recorrido observable;
- semilla explícita que no consulta la fuente de entropía del sistema;
- consulta de vistas sin acceso mutable al estado;
- llamadas repetidas a `vista()` que devuelven exactamente la misma instantánea
  e IDs sin consumir azar, consultar disponibilidad ni modificar contadores;
- conservación del orden canónico de los Lugares visibles en `VistaMundo`;
- proyección exclusiva del jugador y los integrantes del Grupo en
  `jugador` y `companeros`, con el orden del Grupo y sin retratos de NPC
  independientes;
- proyección del nombre visible de cada Actor junto con su ID y retrato, sin
  mostrar la clave externa como nombre;
- exposición de la fecha confirmada del Mundo en `VistaMundo`;
- proyección de las claves de tipo visual, y no de los símbolos RON, en una
  `VistaMapa` con dimensiones y orden de celdas correctos;
- exposición ordenada de todos los tipos declarados, incluido uno que todavía no
  aparezca en ninguna celda;
- conservación de una `VistaMapa` anterior después de modificar una celda,
  mientras una vista nueva refleja el cambio sin afectar otra Partida;
- inicio de `Viajar`, fragmento, decisión y continuación;
- proyección ordenada de propósito, propuestas, compromiso, subobjetivos,
  interpretaciones y replanteamientos sin analizar texto;
- correspondencia exacta entre cada `VistaRegistroProyecto` y su
  `RolPresentacion`, incluida su precedencia cuando la vista también refiere una
  situación;
- decisiones de propuesta, refinamiento del plan, situación particular y
  replanteamiento clasificadas, respectivamente, como `Deliberacion`,
  `Compromiso`, `DesarrolloSituacion` y `Replanteamiento`;
- deliberación y elección de plan en las mismas fases generales de presentación
  y decisión, sin una pantalla especial ni momentos causales inventados;
- proyección exacta de `N` ejecuciones particulares como `N` momentos causales,
  sin límite cuando `N` supera seis y sin perder dos ejecuciones iguales
  consecutivas;
- una situación que contiene varios momentos y un momento distribuido en varias
  viñetas consecutivas, conservando IDs, trazas y orden;
- una viñeta que presenta varios momentos contiguos sin volverlos indistinguibles
  ni cambiar su secuencia;
- dos situaciones dramáticas distintas que reutilizan la misma acción particular
  y producen textos contextuales diferentes sin cambiar su traza mecánica;
- ausencia de vistas causales para consultas de elegibilidad, pronósticos,
  candidatos descartados y otras operaciones que no ejecuten una acción;
- una acción particular multidiaria conservada como un momento con su duración
  completa, aunque se presente en más de una viñeta;
- resolución de nombres y redacción desde la traza sin cambiar factores, ajustes,
  tirada, banda, duración ni efectos;
- redacción de una causa mecánica únicamente cuando aparece en los hechos
  contextuales y en la traza, y libertad de detalles literarios sin efecto
  reglado;
- ausencia del Lugar actual entre los objetivos ofrecidos y rechazo si se intenta
  enviarlo manualmente;
- ausencia de Lugares desconectados o sin una ruta viable, sin que la aplicación
  inspeccione Relaciones por su cuenta;
- jugador vivo sin destinos que produce exactamente
  `Viajar { destinos: vec![] }` y `causa_indisponibilidad: None`;
- jugador con `muerto` que produce `acciones: vec![]` y
  `causa_indisponibilidad: Some(JugadorMuerto)`;
- rechazo de intenciones incompatibles con la fase vigente;
- precedencia exacta entre fase incompatible, fragmento, decisión y opción no
  vigentes, incluida cualquier intención recibida en `Fatal`;
- respuesta inválida sin perder la espera ni consumir azar;
- ausencia de avance temporal antes de confirmar el último fragmento;
- fecha proyectada pura y monótona en cada fragmento y decisión, incluido un
  salto multidiario, sin modificar `Mundo::fecha()`;
- un único avance seguido de la fase NPC;
- coincidencia entre la última fecha y ubicación proyectadas y el Mundo después
  de liberar la finalización;
- uso conjunto de los componentes de meses y días de la `Duracion`, en el orden
  definido por el calendario;
- error de calendario que conserva la fecha, impide iniciar la fase NPC y deja
  la sesión en `Fatal`;
- acción NPC mediante el mismo motor y sin segundo avance;
- omisión de las políticas Lua de integrantes del Grupo durante la fase NPC;
- ausencia de resumen cuando la política no solicita nada;
- resumen breve de llegada, abandono, interrupción, solicitud rechazada o
  política fallida y regreso a `Disponible`, derivado de un registro interno que
  conserva todas las ejecuciones particulares;
- viaje NPC con más de seis acciones cuya vista cuenta acciones y
  replanteamientos, incluye propósito/destino y duración simulada total, nombra
  solo el último replanteamiento y la última
  consecuencia persistente con cantidades para las anteriores y no crece
  proporcionalmente al viaje;
- rechazo NPC cuya vista no inventa conclusión, Lugar final o duración, y error
  de política anterior a solicitar que tampoco inventa destino o propósito;
- fallo de infraestructura de la fase NPC que lleva a `Fatal` y no se presenta
  como resumen recuperable;
- persistencia del resumen mientras no se acepte otra acción y descarte al
  aceptar la siguiente;
- rechazo de una confirmación de fragmento antigua o repetida sin cambiar la cola;
- asignación de IDs de fragmento distintos entre acciones y rechazo de un ID
  antiguo en una presentación posterior sin consumir el fragmento vigente;
- conservación exacta de cada `ReferenciaSituacion`, momento y progreso causal
  desde el motor hasta la vista, sin resolver arte ni exponer tiles como ruta;
- consumo interno de resultados semánticos sin aplicar nuevamente los cambios ni
  convertirlos por sí solos en una pantalla, y presencia de todo dato observable
  dentro del momento causal correspondiente;
- conversión de los rechazos de decisión y opción a las variantes homónimas de
  `ErrorIntencion`, conservando la espera vigente;
- asignación de IDs de decisión visibles distintos entre dos acciones y rechazo
  de los IDs de vista antiguos sin reanudar el motor ni perder la espera actual;
- correspondencia exacta entre cada `IdOpcionVista` vigente y la opción interna
  que representa;
- separación sin duplicados entre el detalle común de una decisión y el detalle
  exclusivo de cada opción, con `None` para ausencia y rechazo de valores
  `Some` vacíos;
- etiquetas de opción no vacías, sin controles y de hasta 40 columnas visibles,
  incluidas etiquetas Unicode medidas por ancho terminal y no por bytes;
- ausencia de mapa y Mundo vivo en las vistas de fragmento y decisión, junto con
  una instantánea de progreso que nunca revela el punto cronológico siguiente;
- rechazo de una nueva acción del jugador con `muerto`;
- mapeo de `ErrorInstanciarContenido`, disponibilidad inicial incoherente y
  fallo de entropía a los códigos exactos de `ErrorCrearPartida`, sin publicar
  una Partida parcial;
- mapeo de cada causa fatal a su `CodigoFatalSesion`, saneamiento del mensaje y
  estabilidad de la misma `VistaFatal` en consultas posteriores;
- desbordamiento simulado de cada contador de vista que lleva a
  `ContadorAgotado` antes de publicar un ID inválido;
- cantidad sintética de opciones cuya posición no cabe en `IdOpcionVista`, que
  lleva también a `ContadorAgotado` sin publicar una decisión parcial;
- diferenciación entre rechazo recuperable y error fatal;
- ausencia de tipos de terminal, Lua o widgets en la API pública.

## 11. Preguntas bloqueantes

Ninguna en este componente.

La política ante errores Lua y solicitudes NPC rechazadas está definida
solamente en [`07-fase-npc.md`](./07-fase-npc.md); esta fachada se limita a
aplicar ese efecto.

## 12. Decisiones aplazadas

- transformación automática de resultados semánticos en regiones y tiles;
- guardado y reanudación;
- varias sesiones activas simultáneamente;
- ejecución asíncrona o multihilo;
- una ABI para frontends escritos en otro lenguaje.
