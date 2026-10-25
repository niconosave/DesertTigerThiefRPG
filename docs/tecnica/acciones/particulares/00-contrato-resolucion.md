# Contrato de resolución de acciones particulares

| Campo | Valor |
|---|---|
| Estado | Obsoleto; sustituido por el reglamento y contrato 06 |
| Versión | 1.5 |
| Fecha | 2026-10-20 |

> Antecedente histórico. Sus reglas y APIs fueron retiradas del motor activo.
> La fuente vigente es el [contrato de Viajar](../06-implementacion-viajar.md),
> en particular su sección 8 para las cinco acciones particulares actuales.

## 1. Propósito y alcance

Este documento es la fuente canónica del contrato compartido por las acciones
particulares. Una acción particular representa un tipo concreto de intento,
como `Marchar`, `Orientarse`, `Escalar`, `Cazar` o `Navegar`, y conserva sus
reglas cuando es invocada desde distintas acciones generales.

La inspiración en las acciones de un juego de rol de mesa significa que cada
tipo de intento posee una entrada de reglamento propia: determina qué hechos
importan, cómo se deduce la dificultad, cuándo existe incertidumbre y qué puede
ocurrir. No significa que todos los verbos deban usar la misma fórmula ni que el
Mundo almacene una dificultad precalculada.

Este contrato define:

- la separación entre acción general, acción particular y operación del Mundo;
- el ciclo evaluar, decidir, resolver e interpretar;
- el uso opcional y reproducible de `1d20`;
- dificultad y competencia contextuales;
- pronósticos puros para planificación;
- trazas causales estructuradas de cada ejecución;
- responsabilidades técnicas, invariantes y verificaciones.

Las reglas concretas de movimiento, supervivencia y navegación viven en los
documentos hermanos de esta carpeta. El contrato no convierte acciones nuevas en
contenido RON o Lua: el catálogo mecánico permanece cerrado en Rust.
La forma en que una acción general les da propósito se define, para el primer
caso concreto, en [`Viajar` como proyecto humano](../04-proyecto-humano-viajar.md).

## 2. Separación de responsabilidades

```text
acción general
  fija propósito, plan, subobjetivo, contexto, participantes, orden y conclusión
    → acción particular
      evalúa y resuelve una intervención concreta
        → operaciones validadas
          modifican el Mundo de forma atómica
```

La acción general:

- elige el responsable y los ayudantes;
- obtiene las decisiones del jugador o del NPC antes de resolver;
- decide cuándo invocar cada acción particular;
- declara qué subobjetivo humano intenta cumplir esa invocación;
- conserva su estado transitorio, como una Expedición;
- interpreta el resultado respecto de su objetivo;
- emite registros neutrales de proyecto y organiza situaciones y momentos
  causales; la aplicación compone después las viñetas;
- acumula duración;
- decide si continúa, cambia de curso o termina.

La acción particular:

- valida un intento acotado;
- consulta solamente los hechos declarados por su regla;
- deduce dificultad y competencia para ese contexto;
- decide si el intento es imposible, determinista o incierto;
- consume azar únicamente cuando corresponde;
- devuelve un resultado concreto, duración, traza causal y operaciones
  solicitadas;
- no conoce el objetivo completo de la acción general;
- no abre pantallas, no espera entrada y no modifica directamente el frontend.

Una operación valida y aplica un cambio concreto del Mundo. No elige intención,
dificultad, azar, texto ni duración. Los cambios que solo pertenecen al estado
transitorio de la acción general —por ejemplo las raciones de una Expedición— no
son operaciones del Mundo.

## 3. Ciclo normativo

Toda invocación sigue este orden:

1. La acción general fija el subobjetivo vigente y construye una solicitud con
   responsable, ayudantes, blanco o entorno y enfoque ya decidido.
2. La acción particular valida identidades, pertenencia al contexto, requisitos
   y coherencia de la solicitud.
3. La acción particular deriva una evaluación autoritativa usando el Mundo y el
   estado transitorio permitido.
4. La evaluación resulta `Imposible`, `Determinista` o `Incierta`.
5. `Imposible` produce una conclusión válida con su causa, sin consumir azar,
   tiempo ni consecuencias; la acción general decide si no ofrece el intento o
   si comunica esa imposibilidad.
6. `Determinista` produce el resultado sin consumir azar.
7. `Incierta` consume exactamente el azar declarado y selecciona un resultado
   mediante sus bandas.
8. La acción general incorpora cambios transitorios y solicita al ejecutor las
   operaciones persistentes.
9. El ejecutor valida el lote completo y solo entonces lo aplica.
10. La acción general interpreta el resultado y decide su siguiente momento.

Una decisión necesaria debe resolverse antes del paso 3. Una acción particular
es atómica y nunca devuelve una continuación pendiente al frontend.

## 4. Forma conceptual y organización en Rust

Las acciones particulares no se implementarán inicialmente mediante objetos de
trait dinámicos ni un registro de funciones modificable. Cada una tendrá un
módulo Rust y tipos concretos. Esto permite que `ResultadoCazar` y
`ResultadoOrientarse` expresen datos diferentes sin campos opcionales
artificiales.

La organización objetivo de `desert-tiger-core` es:

```text
acciones/
├── mod.rs
├── particulares/
│   ├── mod.rs
│   ├── resolucion.rs
│   ├── movimiento.rs
│   ├── supervivencia.rs
│   └── navegacion.rs
└── viajar/
    ├── mod.rs
    ├── proyecto.rs
    ├── planificacion.rs
    ├── expedicion.rs
    └── situaciones.rs
```

El incremento inicial implementa `acciones/particulares/resolucion.rs` y
`acciones/particulares/movimiento.rs`. El archivo `acciones.rs` conserva por
ahora la acción general `Viajar` y su fachada pública; PLAN-2026-014 realizará
la separación del submódulo `viajar/` cuando introduzca la Expedición por etapas.

Cada acción concreta expone internamente el equivalente a:

```rust
fn evaluar(
    mundo: &Mundo,
    solicitud: &Solicitud,
) -> Result<Evaluacion, RechazoParticular>;

fn resolver(
    evaluacion: Evaluacion,
    azar: &mut dyn FuenteAzar,
) -> Result<Resultado, FalloMotor>;
```

`evaluar` no consume azar ni muta. `Err(RechazoParticular)` queda reservado para
una solicitud inválida —IDs, participantes o contexto incoherentes—;
`Evaluacion::Imposible` es una evaluación válida. `resolver` recibe una
evaluación ya validada y no vuelve a consultar datos que pudieran producir otra
regla. Los tipos exactos son privados al núcleo hasta que un consumidor externo
real justifique una API pública.

Conceptualmente, una solicitud incluye:

```text
SolicitudParticular
├── tipo estable
├── actor responsable
├── ayudantes ordenados y sin duplicados
├── blanco, medio o entorno
├── enfoque ya elegido
└── contexto transitorio expresamente permitido
```

Cada resultado concreto incluye, cuando corresponde:

```text
ResultadoParticular
├── resultado propio del verbo
├── duración
├── costes o aportes para el estado transitorio
├── operaciones del Mundo solicitadas
└── traza causal neutral
```

### 4.1. Traza causal obligatoria

Toda invocación aceptada que alcanza una evaluación autoritativa, incluida una
conclusión imposible y una resolución automática, devuelve una traza neutral.
La traza es parte del resultado reglado: no es prosa terminal ni un registro
reconstruido después por la acción general.
Su forma pública común y su transporte pertenecen al
[motor de acciones](../../componentes/05-motor-acciones.md); esta especificación
fija su contenido semántico:

```text
TrazaAccionParticular
├── tipo estable de acción
├── responsable y ayudante pertinente
├── contexto de origen, objetivo o entorno
├── factores realmente aplicados, en orden canónico
│   ├── clave estable
│   ├── eje causal
│   ├── procedencia
│   ├── ajuste firmado
│   ├── explicación pública
│   └── visibilidad para los viajeros
├── resoluciones aplicadas, normalmente una
│   ├── sujeto cuando una regla grupal resuelve por participante
│   ├── modo: imposible | determinista | prueba
│   └── dificultad, cara, modificador y margen cuando corresponda
├── banda y desenlace propio del verbo
├── duración
├── cambios transitorios y contabilidad causados
└── operaciones persistentes solicitadas y, tras el lote, confirmadas
```

La evaluación conserva suficientes datos previos para que la acción general
pueda establecer el intento dentro de su situación dramática antes de revelar el
desenlace. Puede hacerlo en una o varias viñetas y compartir la escena con otras
intervenciones. `resolver`, o la propia conclusión imposible cuando no
corresponde llamarlo, completa la misma traza; no vuelve a consultar el Mundo ni
recalcula factores.
La mayoría de las acciones contiene exactamente una resolución aplicada. Una
regla grupal que realmente realiza una prueba separada por participante —como el
descanso expuesto— las conserva dentro de la misma traza, en el orden de los
viajeros y con el sujeto de cada una. Esto no crea varias invocaciones ni varias
trazas: la acción particular grupal sigue siendo una sola intervención.
Un factor no aplicado no aparece. Un hecho desconocido puede intervenir en la
evaluación autoritativa, pero su explicación y su ajuste permanecen ocultos
hasta que las reglas lo vuelvan perceptible. El total visible no permitirá
deducir por resta los factores todavía ocultos.

La traza distingue los datos del cálculo detallado de los hechos necesarios
para una explicación causal normal. Ocultar el panel de detalle no autoriza a
reemplazar la explicación por una frase genérica. La aplicación puede redactar
los hechos recibidos y resolver nombres visibles; no puede decidir de nuevo qué
causó la dificultad, la banda, la duración o una consecuencia.

Cada invocación resuelta conserva su propia traza aunque sea igual a la
anterior. No existe una regla para muestrear, fusionar o descartar trazas por
cantidad. Cuando existe un proyecto, la acción general enlaza cada traza con el
subobjetivo que le dio sentido y con un momento causal identificable de una
escena, exactamente una vez. No existe correspondencia uno a
uno entre ese momento, la escena y la viñeta: una escena puede reunir varios
momentos y un momento puede ocupar varias viñetas. El contrato completo se define en la
[cronología dramática de `Viajar`](../03-cronologia-causal-viaje.md).

## 5. Dificultad y competencia contextuales

La dificultad no es una Característica de Lugar y no se serializa como
`dificil`, `facil`, `jornadaLarga`, `rutaSegura` o equivalente. Cada acción la
deduce de hechos intrínsecos del intento.

Por ejemplo, `Escalar` puede examinar `empinado`, `rocoso`, humedad, equipo,
estado físico y ayuda. `Cazar` puede examinar fauna, fecha, conocimiento,
tiempo dedicado y estado del cazador. Que ambos terminen usando un dado no
convierte sus factores ni resultados en la misma regla.

La competencia tampoco es una estadística universal en esta versión. Se deriva
de:

- Características del Actor, como `escalador`, `cazador`, `navegante` o
  `fuerte`;
- Relaciones de conocimiento con elementos concretos;
- estados como `herido`, `fatigado`, `sediento` o `hambriento`;
- medios y equipo disponibles;
- ayuda de otros Actores;
- propiedades intrínsecas del blanco y el entorno.

Cada acción particular es la única autoridad sobre el significado de esos
hechos. Si `cazador` favorece `Cazar`, la regla se aplica igual dentro de
`Viajar`, `Acampar`, `Rescatar` o un evento. La acción general no puede añadir
otro modificador de caza por su cuenta.

Los factores poseen una clave estable, una explicación y un ajuste firmado. Cada
ficha documenta sus valores exactos. Para evitar acumulaciones artificiales se
toma como máximo un hecho por eje causal: el terreno predominante, el estado
personal más grave, la capacidad principal pertinente, el medio o equipo
principal y como máximo un ayudante. Una misma causa no se cuenta dos veces bajo
nombres distintos.

## 6. Resolución incierta con `1d20`

El dado común mantiene resultados comprensibles en pasos del cinco por ciento.
Las acciones de esta ampliación usan por defecto un margen, pero cada ficha sigue
siendo responsable de derivar su dificultad y dar significado a los resultados:

```text
cara = FuenteAzar.elegir_indice(20) + 1
margen = cara + sumaModificadores - dificultadDerivada
```

La dificultad es un valor transitorio construido por la acción particular. No
se guarda en RON ni se copia al Lugar. La convención inicial de bandas es:

| Margen | Banda de resolución |
|---:|---|
| `5` o más | holgada |
| `0..=4` | lograda |
| `-4..=-1` | costosa |
| `-5` o menos | fallida |

Estas bandas ofrecen un vocabulario regular; no imponen una consecuencia
universal. `Cazar` y `Orientarse` interpretan una banda costosa de maneras
distintas. Una ficha puede definir otros límites cuando explique la necesidad.

Las reglas comunes son:

- se valida todo el intento antes de tirar;
- cada prueba consume exactamente una extracción con límite `20`;
- `1` y `20` no son críticos automáticos;
- las consecuencias de cada banda pertenecen a la acción concreta;
- los límites cubren todo margen posible sin huecos ni solapamientos;
- si, después de evaluar el contexto, solo un resultado es posible, la
  evaluación se vuelve determinista y no tira;
- una acción rutinaria o sin consecuencia interesante tampoco tira;
- una acción puede definir justificadamente otro procedimiento, pero debe
  documentar su consumo de azar con la misma precisión.

La fuente existente `FuenteAzar` sigue siendo la única fuente. No se incorpora
un generador global ni una dependencia aleatoria al núcleo.

## 7. Ayudantes y Actor responsable

La acción general elige un responsable válido antes de invocar la particular.
Según el contexto puede seleccionarlo automáticamente, permitir que el jugador
lo elija o imponerlo por la ficción.

Una regla puede admitir ayudantes. Para esta versión:

- el responsable nunca aparece también como ayudante;
- los ayudantes están ordenados por `IdActor` y no se repiten;
- solo cuenta el mejor aporte de ayuda expresamente admitido por la acción;
- reunir muchos compañeros no acumula indefinidamente el mismo beneficio;
- un ayudante puede quedar expuesto a una consecuencia únicamente si la ficha
  de la acción lo declara;
- la acción general no calcula el bono de ayuda: solo entrega participantes.

## 8. Pronóstico y conocimiento

Una acción que participe en planificación ofrece una estimación pura construida
por la misma derivación interna que su evaluación autoritativa.

La parte comparable entre acciones usa esta forma normalizada:

```text
PronosticoParticular
├── viabilidad: posible | imposible | incierta
├── diasMinimos
├── diasMaximos
├── exposicionConocida
└── incertidumbre
```

`diasMinimos` y `diasMaximos` abarcan una única resolución de la intervención,
no una cantidad ilimitada de reintentos. `exposicionConocida` se calcula, para
una prueba d20 con las bandas comunes, como una unidad por cada cara conocida
que produciría resultado costoso y dos por cada cara que produciría resultado
fallido. Una ficha puede asignar una severidad mayor a su banda fallida cuando
documente una progresión concreta, pero debe usarla también en ejecución y
pruebas.

`incertidumbre` no cuenta secretos reales. Vale `0` cuando todos los ejes que el
pronóstico necesita son conocidos y `20` cuando la falta de conocimiento del
Lugar obliga a usar el supuesto neutral de la ficha. Por ello no revela si
existe uno, cinco o ningún peligro oculto. Las acciones deterministas sin coste
usan exposición e incertidumbre `0`.

La estimación:

- no modifica el Mundo ni el estado de una acción;
- no consume azar;
- usa únicamente hechos conocidos por cualquiera de los viajeros autorizados;
- puede indicar imposibilidad conocida, duración mínima y máxima, necesidades,
  factores visibles y riesgo conocido;
- marca incertidumbre sin confirmar si existe un factor oculto concreto;
- conserva un orden canónico y no depende del orden de mapas o archivos.

La evaluación autoritativa usa todos los hechos reales. Un factor desconocido
puede influir en el desenlace sin aparecer en el pronóstico. La mera presencia o
ausencia de una tirada, banda o texto no debe filtrar el número de factores
ocultos.

Cuando una planificación necesita un coste comparable, la acción general lo
deriva de los campos normalizados de las estimaciones; no vuelve a implementar
las reglas particulares.

## 9. Duración, consecuencias y narración

La duración aportada usa `Duracion` y se acumula mediante suma comprobada. Una
acción particular puede durar cero días si ocurre dentro de una jornada ya
contabilizada, pero la acción general `Viajar` completa nunca dura cero.

Los resultados distinguen:

- cambios transitorios del contexto llamador, como agua, alimento, fatiga de la
  Expedición o un desvío propuesto;
- operaciones persistentes, como trasladar Actores o añadir `herido`;
- hechos narrativos, como haber encontrado huellas o haber progresado con
  demora.

La traza causal no contiene prosa ni texto de Ratatui. El núcleo produce hechos
semánticos suficientes para que la acción general los sitúe en su contexto y la
aplicación redacte una escena dramática que explique factores conocidos,
desenlace y consecuencias observables. El frontend decide composición, marco y
color, pero no inventa contexto o causas ni descarta una ejecución.

## 10. Errores e invariantes

Una evaluación imposible representa una limitación física válida. Un rechazo
particular representa una solicitud inválida y tampoco consume azar, duración
ni operaciones. Un fallo de la fuente de azar, desbordamiento o invariante rota
es fatal para la acción y conserva la política del motor general.

Invariantes:

- jugador y NPC usan las mismas funciones particulares;
- una acción general nunca duplica factores, bandas o consecuencias de una
  acción particular;
- cada invocación resuelta devuelve exactamente una traza causal y conserva su
  orden respecto de las demás;
- ninguna cantidad de invocaciones habilita a fusionar u omitir trazas;
- cada traza se asigna después a exactamente un momento causal;
- toda traza ejecutada dentro de un proyecto queda asociada con el subobjetivo
  vigente sin incorporar ese propósito a la regla particular;
- una traza no define por sí sola una escena, un tono ni una cantidad de
  viñetas;
- consultar o estimar nunca muta ni consume azar;
- igual Mundo, solicitud, decisiones y secuencia de azar producen igual
  resultado;
- cada extracción tiene motivo, orden y límite documentados;
- una operación persistente solo se comunica después de confirmarse;
- una acción particular no conoce Ratatui, teclas, tiles o coordenadas visuales;
- los mods pueden aportar hechos mediante datos, pero no reemplazar reglas Rust
  ni ejecutar operaciones desde Lua sin validación.

## 11. Pruebas y criterios de aceptación

El contrato requiere pruebas de:

- evaluación imposible y determinista sin extracción;
- evaluación incierta con exactamente una extracción de límite `20`, convertida
  a una cara `1..=20`;
- caras `1` y `20` sin semántica especial no declarada;
- bandas exhaustivas, ordenadas y sin solapamientos;
- factores acumulados una sola vez por eje causal;
- misma regla particular invocada desde dos contextos generales de prueba;
- responsable y ayudantes válidos, ordenados y sin duplicados;
- pronóstico sin mutación, azar ni filtración de hechos desconocidos;
- diferencia reproducible entre Actor competente y Actor sin competencia;
- duración comprobada y operaciones solicitadas pero no aplicadas directamente;
- traza completa para modos imposible, determinista e incierto, con factores en
  orden canónico, detalle de la prueba y consecuencias exactas;
- filtrado de causas desconocidas sin permitir inferir su ajuste por los
  totales visibles;
- dos invocaciones consecutivas iguales que conservan dos trazas distintas y
  ordenadas;
- una misma acción particular ejecutada bajo dos subobjetivos distintos conserva
  la misma regla y deja que el proyecto registre el sentido de cada invocación;
- paridad de resolución entre jugador y NPC.

## 12. Catálogo de referencia

El catálogo orienta capacidades jugables. No obliga a implementar cada verbo en
esta ampliación ni a crear una función diferente cuando varios compartan una
estructura interna.

| Familia | Acciones particulares de referencia |
|---|---|
| Percepción y pesquisa | observar, escuchar, olfatear, palpar, examinar, vigilar, rastrear, recoger indicios |
| Lengua y conocimiento | consultar una fuente, leer, escribir, traducir, cifrar, descifrar, cartografiar, interpretar un presagio |
| Movimiento | marchar, correr, gatear, saltar, escalar, descender, mantener el equilibrio, nadar |
| Monturas y transporte terrestre | cabalgar, ensillar, embridar, domar, calmar un animal, arrear, conducir un carro, guiar una caravana |
| Marinería | remar, navegar, gobernar el timón, izar una vela, arriar una vela, fondear, atracar, achicar agua |
| Accesos y trampas | usar una llave, ganzuar una cerradura, forzar una puerta, forzar un recipiente, levantar una tranca, accionar un mecanismo, preparar una trampa, desactivar una trampa |
| Sigilo y subterfugio | esconderse, camuflarse, moverse en silencio, acechar, espiar, escuchar a escondidas, disfrazarse, suplantar una identidad |
| Objetos y entorno | recoger, cargar, arrastrar, empujar, tirar de, lanzar, atar, cortar |
| Ataque y control físico | apuntar, golpear, cortar con un arma, apuñalar, disparar un arco, arrojar un arma, forcejear, desarmar |
| Defensa y retirada | bloquear, parar un golpe, esquivar, cubrirse, interponerse, inmovilizar, retirarse, rendirse |
| Supervivencia y campamento | orientarse, buscar agua, tratar agua, montar campamento, descansar, hacer guardia, recolectar, cazar, pescar, encender fuego, construir un refugio |
| Medicina y cuidado | diagnosticar, detener una hemorragia, vendar, suturar, entablillar, preparar un remedio, administrar un antídoto, cuidar a un enfermo |
| Conversación e influencia | preguntar, interrogar, explicar, argumentar, negociar, ofrecer, suplicar, amenazar |
| Engaño, reputación y compromiso | mentir, distraer, adular, sobornar, acusar, prometer, jurar, desafiar |
| Arte y representación | cantar, tocar un instrumento, bailar, recitar, contar una historia, actuar, pintar, esculpir |
| Oficio e intercambio | cocinar, reparar, forjar, coser, tejer, extraer mineral, construir, trocar |
| Mando y gobierno | dar una orden, arengar, coordinar un ataque, organizar una defensa, convocar un consejo, conceder audiencia, juzgar, conceder perdón |
| Rito y mundo sobrenatural | rezar, presentar una ofrenda, sacrificar, bendecir, maldecir, purificar, consultar un oráculo, invocar un espíritu |

## 13. Decisiones aplazadas

Quedan fuera de esta versión las estadísticas numéricas persistentes de
habilidad, niveles de personaje, ventaja o desventaja universal, tiradas
enfrentadas universales, críticos globales, edición de reglas particulares desde
RON o Lua y una API pública para plugins mecánicos.

Estas extensiones no son necesarias para implementar las acciones particulares
de `Viajar` y no alteran el contrato aprobado.
