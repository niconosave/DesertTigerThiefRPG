# Motor de acciones y protocolo semántico

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.12 |
| Fecha | 2026-10-15 |

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
- la ejecución de acciones particulares reutilizables;
- decisiones, fragmentos narrativos, resultados y duración acumulada;
- validación común para acciones del jugador y de los NPC;
- acceso controlado al azar.

El primer incremento necesita ejecutar `Viajar` tanto para el jugador como para
el NPC mínimo. Sus reglas concretas pertenecen a
[`acciones/01-viajar.md`](../acciones/01-viajar.md).

Quedan fuera el contenido y cálculo de cada acción, la representación exacta del
tiempo, la selección visual, Lua y el dibujo de viñetas. El alcance inicial no
admite cancelar una acción ya comenzada, deshacer ni ejecución simultánea; el
frontend sí puede abandonar una selección antes de enviar la solicitud.

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
- texto o información narrativa neutral;
- una lista no vacía de opciones, cada una con identificador y texto neutral.

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
    pub texto: String,
    pub escena: Option<ClaveEscena>,
    pub opciones: Vec<OpcionDecision>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpcionDecision {
    pub id: IdOpcionDecision,
    pub texto: String,
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

Los constructores internos garantizan que el texto no esté vacío, que las
opciones tengan IDs distintos y que la lista contenga al menos dos opciones.

`ClaveEscena` es un enum semántico cerrado del núcleo. Sus primeras variantes y
la forma del fragmento son:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaveEscena {
    ViajeMarcha,
    ViajeObstaculo,
    ViajeRumboIncierto,
    ViajeLlegada,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentoNarrativo {
    pub texto: String,
    pub escena: Option<ClaveEscena>,
}
```

Las cuatro variantes están aprobadas por las tres ramas de
[`acciones/01-viajar.md`](../acciones/01-viajar.md). Cada variante futura se
añadirá solamente después de aprobar su significado y uso en el documento de la
acción que la produce.

La clave no contiene una ruta, bytes de arte, dimensiones, estilos ni un
identificador del catálogo de contenido. El núcleo tampoco conoce su nombre
externo `lowerCamelCase`, su representación RON ni la ruta de un recurso. Otro
frontend puede asociar la misma variante con una imagen 2D, una escena 3D, arte
ASCII o ninguna representación.

Cada llamada aceptada avanza hasta la próxima frontera y devuelve un tramo:

```text
TramoAccion
├── emisiones ordenadas
│   ├── Narrativa(FragmentoNarrativo)
│   └── ResultadoSemantico(ResultadoSemantico)
└── cierre
    ├── EsperandoDecision(SolicitudDecision)
    └── Completada(FinalizacionAccion)
```

Las emisiones pueden incluir:

- **fragmentos narrativos**, con el texto y una clave de escena opcional que el
  frontend puede convertir en una viñeta;
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
    Narrativa(FragmentoNarrativo),
    ResultadoSemantico(ResultadoSemantico),
}

pub enum CierreTramo {
    EsperandoDecision(SolicitudDecision),
    Completada(FinalizacionAccion),
}
```

El constructor interno de `FragmentoNarrativo` rechaza un texto vacío; su valor
se convierte uno a uno en una `VistaFragmento` de la aplicación sin alterar el
texto ni la clave. Los demás tipos públicos de salida iniciales son:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResultadoSemantico {
    ActoresTrasladados {
        actores: Vec<IdActor>,
        origen: IdLugar,
        destino: IdLugar,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FinalizacionAccion {
    pub duracion: Duracion,
}
```

`ResultadoSemantico` describe una consecuencia que el núcleo ya confirmó; nunca
es una orden para que la aplicación la aplique otra vez. En
`ActoresTrasladados`, `actores` es una secuencia no vacía y sin duplicados. Para
un viaje del jugador contiene primero al jugador y después a los integrantes del
Grupo en su orden canónico; para un NPC independiente contiene solamente a ese
Actor. Todos los actores enumerados estaban en `origen` y ya se encuentran en
`destino` cuando el motor devuelve la emisión.

El motor obtiene esos tres campos directamente del `TrasladoConfirmado` devuelto
por `Mundo::trasladar`; no vuelve a consultar ubicaciones para reconstruirlos.

Cada `Viajar` aceptado produce exactamente un `ActoresTrasladados`: el motor
confirma primero el traslado mediante la operación semántica de `Mundo`, añade el
resultado después del último fragmento narrativo de la rama y cierra el tramo
con `Completada`. La aplicación puede usar el resultado para coordinar otros
subsistemas o formar un resumen, pero no vuelve a ejecutar el traslado.

`FinalizacionAccion` contiene únicamente la duración total. El cierre ya expresa
que la acción terminó y las emisiones ya describen sus consecuencias, por lo que
repetir el destino, los actores o un indicador de éxito crearía una segunda
fuente de verdad. La representación de `Duracion` pertenece al
[calendario](./06-calendario-duracion.md).

Los fragmentos y decisiones no contienen arte, rutas, estilos, coordenadas ni
tipos de Ratatui. En el primer incremento, el código Rust de cada acción produce
sus propios textos y claves semánticas; Lua y el frontend no redactan ni
sustituyen esa lógica narrativa. La aplicación conserva el orden conforme al
contrato de sesión.

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
fragmentos.

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
contexto reglado, calcula y aplica un resultado, y puede aportar fragmentos y
tiempo a la acción general. No conoce la presentación ni permite evitar la
validación de quien la invoca.

Las acciones particulares iniciales son funciones o tipos internos estáticos
invocados por `Viajar`. No necesitan IDs públicos ni un catálogo dinámico en
este incremento.

La duración comienza exactamente en `Duracion::CERO`. Cada aporte de una acción
particular se incorpora mediante `Duracion::checked_add`; un
`ErrorCalendario::Desbordamiento` se convierte en
`FalloMotor::DesbordamientoDuracion`. Las acciones particulares pueden modificar
el total durante la ejecución; el motor lo comunica al exterior únicamente como
duración final de la acción completa. El motor nunca llama a
`Mundo::avanzar_fecha`. El avance de la fecha y el tratamiento especial de la
duración de NPC se definen en
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
en `0..limite_exclusivo`. `Viajar` hará una extracción uniforme para seleccionar
su grupo de fragmentos. Cada decisión automática de un NPC hará otra extracción
uniforme sobre las opciones en el orden estable entregado por la acción. No hay
pesos configurables en el primer incremento.

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
- Cada fragmento conserva el orden en que fue emitido por la lógica.
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
    DestinoInalcanzable {
        destino: IdLugar,
    },
    ViajeIndependienteDeIntegranteGrupo {
        actor: IdActor,
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

`RechazoAccion` enumera condiciones esperables del dominio y puede mostrarse o
traducirse sin invalidar la Partida. `FalloMotor` cubre una invariante rota, un
fallo de `FuenteAzar` y cualquier error inesperado posterior al punto de
confirmación; obliga a llevar la sesión a `Fatal`.

Las seis primeras variantes de `RechazoAccion` corresponden a las validaciones
aprobadas para iniciar `Viajar`. `JugadorMuerto` se usa solamente cuando el Actor
es el jugador; el efecto general de `muerto` sobre otros NPC continúa aplazado.
Las dos últimas protegen la reanudación directa del motor frente a una decisión
antigua o una opción que no pertenece a la espera vigente. La fachada puede
interceptarlas y convertirlas en sus errores homónimos, pero el núcleo conserva
la validación. Mientras existe una acción pendiente, la fachada evita iniciar
otra antes de llamar al motor.

`FuenteAzar` representa un error devuelto por la capacidad inyectada;
`IndiceAzarFueraDeRango` representa que esa capacidad devolvió `Ok` con un valor
que no pertenece a `0..limite_exclusivo`. `DesbordamientoDuracion` indica que una
suma de incrementos reglados no puede representarse. `InvarianteRota` agrupa los
estados internos imposibles y un fallo inesperado de una operación de Mundo
después de haber superado sus validaciones. La aplicación no necesita tratar
estas causas de forma distinta: todas invalidan la continuidad de la sesión.

Al comenzar `Viajar`, el motor valida en orden: Actor existente, destino
existente, jugador no `muerto`, destino distinto del Lugar actual, destino sin
`inalcanzable` y Actor no integrante del Grupo intentando viajar solo. Cada
condición se convierte en la variante homónima de `RechazoAccion`. Al final de
una acción ya aceptada, cualquier `ErrorTraslado` se convierte en
`ErrorMotor::Fatal(FalloMotor::InvarianteRota)`, porque todas sus condiciones
previsibles ya fueron validadas y la sesión no permite otra mutación intermedia.

Una solicitud rechazada no crea una acción, consume azar ni tiempo. Una respuesta
inválida conserva el punto de espera. Los diagnósticos identifican la causa sin
provocar `panic` en caminos normales. Un fallo interno fatal se propaga a la
aplicación, que detiene la sesión y permite al frontend restaurar la terminal.

## 9. Pruebas y criterios de aceptación

Las pruebas unitarias usarán un Mundo mínimo, una acción de prueba y azar
controlado para comprobar:

- inicio, espera, respuesta y finalización de una acción con varias transiciones;
- distinción estructural entre emisiones, decisión, finalización y rechazo;
- suma de duraciones aportadas por más de una acción particular;
- selección reproducible de una rama azarosa;
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
- conservación exacta de `ClaveEscena::ViajeMarcha` y
  `ClaveEscena::ViajeLlegada` en sus fragmentos al atravesar el tramo y la
  fachada de aplicación;
- conservación exacta de `ClaveEscena::ViajeObstaculo` tanto en el fragmento
  inicial como en `SolicitudDecision`, su cierre de tramo y la fachada de
  aplicación;
- conservación exacta de `ClaveEscena::ViajeRumboIncierto` tanto en el fragmento
  inicial como en `SolicitudDecision`, su cierre de tramo y la fachada de
  aplicación;
- igualdad de reglas, azar, duración y consecuencias con una `ClaveEscena`
  presente o ausente;
- ausencia de rutas, arte, nombres RON y tipos de terminal en
  `FragmentoNarrativo` y `ClaveEscena`;
- emisión de exactamente un `ActoresTrasladados` por viaje aceptado, después de
  confirmar el traslado, con origen, destino y actores exactos en orden canónico;
- secuencia no vacía y sin duplicados para `actores`, con jugador y Grupo o el
  único NPC según corresponda;
- imposibilidad de usar un resultado semántico como orden para aplicar por
  segunda vez la consecuencia;
- finalización que contiene solamente la duración total aprobada de dos, tres o
  cuatro días, sin duplicar destino, actores ni éxito;
- cobertura individual de todas las variantes de `RechazoAccion`, conservando
  Mundo, ejecución, duración y azar en cada rechazo.

Una prueba de integración sin frontend completará una acción mediante la fachada
de aplicación y verificará sus fragmentos ordenados, consecuencias y duración.
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
- representación genérica de resultados desfavorables no requerida por las
  ramas aprobadas de `Viajar`;
- efecto general de `muerto` sobre NPC distintos del actor jugador.

