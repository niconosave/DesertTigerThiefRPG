# Fase de NPC

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.7 |
| Fecha | 2026-10-15 |

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
6. consumir internamente las emisiones del tramo, descartando sus fragmentos
   narrativos y conservando sus resultados semánticos;
7. si el cierre espera una decisión, elegir una opción y llamar una vez a
   acciones::responder;
8. consumir el segundo tramo de la misma forma y exigir que termine en
   Completada;
9. no aplicar la Duracion recibida a la fecha;
10. comprobar la consecuencia confirmada y construir el resumen;
11. volver a Disponible.

Las viñetas de la acción NPC no se publican ni se convierten en vistas. Descartar
su narrativa no autoriza otra implementación de la acción: el NPC atraviesa las
mismas funciones y reglas que el jugador.

### Decisión automática

El Viajar inicial produce cero o una decisión. Si aparece, la fase exige que la
lista de opciones sea no vacía, llama una vez a FuenteAzar::elegir_indice con su
longitud como límite exclusivo y selecciona el IdOpcionDecision situado en ese
índice. Construye una RespuestaDecision con el IdDecision vigente y reanuda la
misma EjecucionAccion.

Un fallo de la fuente, un índice exterior, un rechazo de esa respuesta construida
por la propia aplicación o una segunda decisión después de responder indican una
inconsistencia fatal. No se elige por texto, posición visual ni clave de teclado.

La selección automática consume una extracción distinta de las que la propia
acción pueda realizar. Su orden observable está fijado en la especificación de
[Viajar](../acciones/01-viajar.md).

### Consecuencia exitosa

Durante todos los tramos de la acción se debe recibir exactamente un:

~~~rust
ResultadoSemantico::ActoresTrasladados {
    actores: vec![actor_vinculado],
    origen,
    destino,
}
~~~

El resultado ya describe un traslado confirmado. La fase no llama a
Mundo::trasladar ni reconstruye la consecuencia comparando instantáneas. Una
lista vacía, más de un Actor, un Actor distinto, cero resultados o más de un
ActoresTrasladados hacen fatal la sesión.

La Duracion final demuestra que el recorrido terminó, pero se descarta después
de comprobar el cierre y nunca se aplica a la fecha. La fecha anterior y posterior
a toda la fase NPC debe ser idéntica.

## 4. Resumen público

El frontend recibe hechos poseídos y neutrales, nunca objetos mlua, fragmentos
descartados ni texto ya redactado:

~~~rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumenNpc {
    entradas: Vec<EntradaResumenNpc>,
}

impl ResumenNpc {
    pub fn entradas(&self) -> &[EntradaResumenNpc];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntradaResumenNpc {
    ViajeCompletado {
        actor: IdActor,
        origen: IdLugar,
        destino: IdLugar,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtapaFalloPoliticaNpc {
    Preparacion,
    Ejecucion,
    ConversionRespuesta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodigoFalloPoliticaNpc {
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
pub struct DiagnosticoPoliticaNpc {
    pub actor: IdActor,
    pub clave_actor: String,
    pub archivo_relativo: String,
    pub etapa: EtapaFalloPoliticaNpc,
    pub codigo: CodigoFalloPoliticaNpc,
    pub linea: Option<u32>,
    pub detalle: String,
}
~~~

ResumenNpc solo se construye dentro de la aplicación con una lista no vacía. En
el primer incremento siempre contiene exactamente una entrada. Si Lua no
solicita nada, VistaMundo.resumen_npc_reciente es None, no un resumen vacío.

DiagnosticoPoliticaNpc es una proyección segura y estable del error interno de
integración. Su ruta procede del patrón ASCII validado
actors/<clave-actor>.ron; no expone PathBuf, mlua::Error, código fuente, trazas
ni tipos del runtime. detalle ya tiene escapados los controles conforme al
contrato de integración. La terminal decide cuánto mostrar.

Una entrada exitosa se construye exclusivamente desde el ActoresTrasladados
confirmado. Una solicitud rechazada conserva la causa devuelta por
acciones::iniciar. En ninguno de los tres casos se añade duración, fecha,
estilos, texto narrativo, nombre visible de Actor ni un booleano redundante de
éxito.

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
- una segunda decisión o una consecuencia semántica que incumpla este contrato;
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
- Una acción NPC completada produce exactamente un traslado del Actor vinculado.
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
- orden exacto de las extracciones de azar de cada rama;
- tratamiento fatal de fuente fallida, índice exterior, respuesta rechazada y
  segunda decisión;
- descarte de narrativa sin publicarla;
- recepción de exactamente un traslado con el NPC, origen y destino correctos;
- tratamiento fatal de un resultado ausente, duplicado o con Actor discordante;
- ausencia de una segunda aplicación del traslado;
- cálculo de duración por el motor sin un segundo avance temporal;
- igualdad de Mundo::fecha() antes y después de la fase completa;
- resumen no vacío con una única entrada del tipo correspondiente;
- ausencia de tipos mlua y de texto redactado en el resumen público.

El componente será aceptable cuando las tres salidas recuperables —sin solicitud,
viaje rechazado y error de política— y el viaje exitoso vuelvan de forma
determinista al jugador, mientras toda inconsistencia interna detenga la sesión.

## 8. Preguntas bloqueantes

- Ninguna.

## 9. Decisiones aplazadas

- varias políticas NPC independientes y su orden;
- si varias políticas observan una instantánea común o consecuencias anteriores;
- límites totales de una fase con varios NPC;
- planificación avanzada, memoria interna y prioridades dinámicas;
- reintentos, deshabilitación persistente o contadores de fallos de una política;
- acciones NPC distintas de Viajar.
