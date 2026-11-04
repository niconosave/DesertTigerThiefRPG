# Viajar — Episodio de cruce de pasos montañosos

| Campo | Valor |
|---|---|
| Estado | Implementado; revisión de jugabilidad del usuario pendiente |
| Versión | 1.1 |
| Fecha | 2026-11-03 |

## 1. Intención y alcance

Atravesar varios Lugares consecutivos de paso montañoso se presenta como un
solo episodio con comienzo, obstáculos, desarrollo y alivio final. Su estructura
narrativa no se repite por cada conexión: contiene seis momentos principales,
o siete cuando corresponde el campamento central. La extensión del recorrido
altera su duración, contexto y seguimiento geográfico, sin multiplicar los
cuatro obstáculos.

Este documento define las reglas confirmadas y su integración técnica.
El episodio está implementado en el motor y se presenta con las viñetas
existentes. Desarrolla el
[contrato de Viajar](./06-implementacion-viajar.md), conserva su protocolo y
convive con la [planificación vigente](./08-planificacion-rutas.md).

La primera versión incluye momentos narrativos y chequeos automáticos. No
incluye decisiones propias del episodio ni eventos de una tabla aleatoria;
ambas formas de momento tienen puntos de extensión previstos. Las decisiones
de supervivencia ya existentes pueden interrumpir el relato cuando correspondan.
No incorpora bonificaciones para `montaraz`, nuevos medios o accesos por
`Bloquea`.

## 2. Qué recorrido forma un episodio

Un Lugar cumple la condición cuando posee **ambas** características: `paso` y
`montanoso`. No basta con su nombre, con ser una montaña o con una relación de
región. Se consulta únicamente la ruta adoptada, sea propuesta o improvisada.

Desde la posición actual, si el Lugar actual y el siguiente cumplen la
condición, se toma la secuencia consecutiva máxima que la cumple. Se requieren
al menos dos Lugares. Un Lugar que no la cumple o el final de la ruta cierra la
secuencia. No se explora el grafo buscando cómo extenderla.

```text
Valle → Paso A → Paso B → Paso C → Bosque
        └────── un episodio ──────┘
```

La conexión Valle–A y la conexión C–Bosque usan las tarjetas habituales. El
episodio abarca A–B y B–C. Una secuencia de dos Lugares abarca una conexión;
una de cinco, cuatro conexiones. Un único Lugar aislado conserva el tratamiento
ordinario. Si más adelante aparece otra secuencia separada por otro terreno,
se trata como un episodio nuevo.

Se reconoce la secuencia antes de resolver su primera conexión interna. Mientras
está activa se conserva su identidad y progreso: llegar al siguiente Lugar no
vuelve a disparar la introducción. Un viaje que ya empieza dentro del paso usa
el segmento restante de su ruta; no inventa una entrada anterior.

El vecino directo conserva la omisión de planificación. Si ambos vecinos
cumplen la condición, su desplazamiento sí utiliza este episodio.

Los tres RON de Zunir actualmente cumplen la condición. La entrada y salida
fueron modificadas por el diseñador; no hay que añadirles las características
de nuevo ni alterar sus conexiones.

## 3. Momentos del episodio

Cada momento tiene un párrafo principal de ficción en segunda persona. Puede
ser algo más extenso que los relatos ordinarios, sin superar un párrafo. Su
consulta completa puede paginarse; una página de texto no es otro chequeo.
Detalles conserva pruebas, consecuencias, cuentas y referencias del recorrido.

Los cuatro obstáculos aparecen **una vez cada uno por episodio**. Fallar añade
la consecuencia indicada; no se repite el chequeo hasta lograr éxito. Todos
permiten continuar por el recorrido previsto: los rodeos son parte de su
abstracción, no una nueva búsqueda de ruta ni Lugares inventados.

### PM-01 — El camino por delante

**Disparador:** el grupo está en el primer Lugar de la secuencia y va a comenzar
su primera conexión interna. No tiene chequeo ni añade tiempo.

El relato presenta la altura, la extensión que queda por atravesar y el esfuerzo
que se anticipa. Comunica la duración prevista del **segmento de paso**, calculada
con sus Lugares, medios y carga actuales; no es una duración garantizada ni la
previsión del viaje completo. No adelanta las futuras tiradas.

**Texto orientativo:** «Ante vos, el paso se interna entre paredes de roca y
crestas que se ocultan unas detrás de otras. Calculás que atravesarlo les llevará
{previsión_del_paso}: un camino largo, donde cada avance exigirá cuidado y las
provisiones tendrán que alcanzar hasta la otra vertiente.»

### PM-02 — Camino difícil

El grupo atraviesa un terreno que obliga a medir los apoyos y sostener el ritmo
a pesar del cansancio. Se realiza un chequeo automático según la sección 4.

| Resultado | Consecuencia |
|---|---|
| Éxito | Supera la dificultad sin demora adicional. |
| Fallo | Añade **8 horas** de traslado por las dificultades y el cansancio. |

El texto explica el esfuerzo, quién conduce el intento y por qué se avanza con
regularidad o se pierde tiempo. No presenta un fallo como abandono del viaje.

### PM-03 — Alrededor de la gran pared

Una montaña alta y empinada cierra la vista; el grupo debe bordearla por un paso
expuesto. Se realiza otro chequeo automático, independiente del anterior.

| Resultado | Consecuencia |
|---|---|
| Éxito | El grupo y su carga pasan sin pérdida. |
| Fallo con animales de carga | Cae **un animal de carga** y se pierde la comida que transportaba. |

La pérdida se calcula con las reservas existentes al resolver el chequeo,
después de consumir el traslado hasta la pared. Sea `A` la cantidad de animales
de carga antes de la caída: se resta un animal y, para cada tipo con `Q` unidades
de comida, se pierde `ceil(Q / A)`. Las unidades son indivisibles y se redondean
hacia arriba; con un solo animal se pierde toda la comida restante. El agua
queda intacta. No se restan cantidades superiores a las disponibles.

Si no hay animales de carga, el momento y su chequeo se conservan. Un fallo
describe un paso accidentado de personas y bultos, sin inventar caída de animal,
pérdida de comida o retraso adicional.

El animal pertenece al contador de carga, no a los contadores de
caballos o camellos usados como monturas. No se mata un Actor ni se concede
una lesión por este resultado.

El relato indica la pérdida y su consecuencia práctica; no dice que se perdió
toda la comida salvo que la regla final y las reservas reales lo justifiquen.
La menor capacidad puede requerir la revisión de carga existente.

### PM-04 — Las vueltas de la montaña

El grupo debe recorrer un itinerario largo y trabajoso, rodeando elevaciones que
impiden avanzar en línea recta. Se realiza un tercer chequeo automático.

| Resultado | Consecuencia |
|---|---|
| Éxito | Completa el tramo muy cansado, sin demora adicional. |
| Fallo | Añade **1 día (24 horas)** de traslado. |

Se hace sentir lo que cuesta cada desvío y la distancia entre la meta visible y
el camino realmente disponible. El cansancio se describe en la ficción; no se
añade automáticamente la característica `cansado` en esta versión.

### PM-05 — Abrirse paso donde no hay camino

El recorrido obliga a encontrar apoyos entre rocas y salvar discontinuidades.
Si hay animales, el texto incluye el trabajo de hacerlos pasar con cuerdas;
si no los hay, describe a las personas y sus bultos. Representa buscar una forma
de desplazarse por ese terreno, sin introducir una prueba distinta de trepar.

| Resultado | Consecuencia |
|---|---|
| Éxito | Logra abrirse paso y continuar. |
| Fallo | No encuentra cómo continuar por allí; retrocede y toma un rodeo, añadiendo **2 días (48 horas)** de traslado. |

La narración deja claro que falló ese intento de abrir camino y que se pudo
continuar por otra parte. No cambia la secuencia de Lugares de la ruta.

### PM-06 — La otra vertiente

**Disparador:** se alcanza el último Lugar de la secuencia y se completaron los
cuatro obstáculos. No tiene chequeo ni añade tiempo.

Desde una altura se reconoce el otro lado y se comprende que el cruce del paso
ha terminado. La prosa transmite alivio después de tantos días y esfuerzo.
Menciona el tiempo acumulado **de traslado dentro del episodio**, sin confundirlo
con toda la acción de Viajar. No afirma haber alcanzado el destino final si la
ruta continúa, ni haber salido de toda la cordillera.

### PM-C — Noche en el centro del paso

Se incorpora solo si `N`, cantidad de Lugares de la secuencia, es impar. Como
la secuencia tiene al menos dos, la primera posibilidad es `N = 3`.

El centro es el Lugar de posición `(N + 1) / 2`, contando desde 1: segundo de
tres, tercero de cinco. El momento ocurre **inmediatamente después de confirmar
la llegada a ese Lugar**. No se busca la mitad del tiempo ni la mitad de los
tiles. No aparece para secuencias pares.

El relato ofrece una pausa de calidez: el grupo se acomoda junto al fuego,
disfruta del olor y el sabor de la comida y, entre las nubes de la noche, descubre
una abertura al cielo estrellado. Si está solo se adapta a esa soledad; si no
queda comida, no se narra que esté disfrutando de un alimento inexistente.

El descanso forma parte de la duración de traslado ya calculada: no agrega
ocho horas, otro día ni un consumo separado de comida. El avance hasta el centro
sí contabiliza su tiempo y raciones habituales. No cura estados ni inicia la acción
general de Acampar, su interfaz de fichas, su día de duración o su música.
El cielo y la fogata se describen en la narración; durante este momento se oye
el mismo crepitar procedural que en Acampar. La aplicación proyecta
`VistaFragmento::fogata_encendida` desde el tipo de momento. El frontend activa
la capa sonora de fogata existente mientras se presenta la viñeta, incluidas
sus páginas y Detalles, y la desactiva al pasar al siguiente momento. Se
conservan los sonidos ambientales del Lugar.

## 4. Chequeos y alcance de sus efectos

Se realizan cuatro pruebas de `Andar`, una por obstáculo, con dificultad 10 y
selección del integrante más capaz en cada caso. Se utiliza `mas_apto` con los
bonos actuales de Andar, sorteando únicamente los empates como ya hace el motor.
Cada prueba tira un d20; alcanza el éxito si el resultado ajustado es al menos
10. No se exige un 20 natural ni hay grados adicionales de fallo.
El contexto del episodio elige responsable y dificultad; la acción particular
solo resuelve el dado con sus modificadores habituales. No se crean habilidades
separadas para cada momento.

Estos chequeos reemplazan, **solo en las conexiones internas del episodio**, las
tiradas e incidentes ordinarios de las tarjetas de desplazamiento. No se
resuelven primero esas tarjetas y luego cuatro peligros adicionales. El medio
real se conserva para calcular tiempo y redactar animales, monturas o personas.

En esta versión las consecuencias de los chequeos son exclusivamente las de
las tarjetas anteriores. El cansancio narrativo no agrega estados ni otros
penalizadores. Los nuevos chequeos tampoco conceden automáticamente `Conoce`:
la exploración visual sigue funcionando con los traslados confirmados. Este
alcance debe quedar visible en las pruebas de integración.

## 5. Distribución de momentos y avance real

### 5.1. Posiciones narrativas

Se distribuyen sobre el progreso de las conexiones internas, no por número de
pantallas ni por resultados de los dados. Sea `L = N − 1` su cantidad:

| Momento | Progreso por conexiones |
|---|---|
| PM-01 | `0` |
| PM-02 | `L / 5` |
| PM-03 | `2L / 5` |
| PM-C, si corresponde | `L / 2`, que es una llegada al centro cuando N es impar. |
| PM-04 | `3L / 5` |
| PM-05 | `4L / 5` |
| PM-06 | `L` |

Las fracciones indican avance **dentro** de una conexión y no se redondean a
Lugares enteros. Así una secuencia de dos Lugares también puede distribuir el
esfuerzo y su tiempo entre cuatro obstáculos, sin simular una llegada antes de
haber recorrido la conexión.

Para operar con enteros puede usarse una escala de diez unidades por conexión:
los objetivos anteriores son `0, 2L, 4L, 5L, 6L, 8L, 10L`; los centros se alcanzan
en los múltiplos de diez. La duración usa `HorasViaje` y las partes de día
existentes, sin introducir un calendario diferente.

Esta colocación garantiza que el campamento central quede entre PM-03 y PM-04.
Las diferencias de tamaño o dificultad alteran el tiempo hasta cada momento,
pero no cambian qué Lugar es el central.

### 5.2. Tiempo, consumo y posición

Se mantienen las duraciones de cada conexión con sus dos mitades, tamaños,
medios, estados vigentes y demora de carga. Un Lugar con `paso` y `montanoso`
usa una base de **7 días por Lugar normal completo (84 horas por mitad)** para
cualquier medio, también con monturas o animales de carga. Prevalece sobre la
base montañosa de nueve/diez días. No es una duración fija para todo el episodio:
una secuencia de tres Lugares normales aporta dos conexiones, con 14 días de
traslado base. Se añaden tamaño, estados, carga y retrasos de los obstáculos.

La misma regla se usa en la búsqueda rápida, las propuestas, la previsión de
provisiones y la ejecución, incluso en las mitades de entrada o salida resueltas
por tarjetas ordinarias. Así el planificador distingue el paso del terreno
montañoso que lo rodea sin anticipar las futuras tiradas.

La suma de los avances parciales debe coincidir exactamente con el tiempo de
sus conexiones completas. Al dividir una duración se calculan diferencias entre
prefijos acumulados de partes de día; el residuo queda en el último segmento.
No se redondea cada trozo por separado ni se cuenta otra vez una mitad al
reanudar. Cada centro atravesado genera su traslado confirmado, en orden.

Las demoras de 8, 24 y 48 horas se añaden una sola vez al resolver sus fallos y
consumen provisiones mediante el sistema existente. El máximo de estas tres
demoras es **80 horas**, sin incluir carga, actividades de supervivencia ni
otras causas ya existentes. Los éxitos no conceden descuentos.

Se mantienen dos cuentas semánticas:

- **Traslado del episodio:** base recorrida, demora de carga y los retrasos de
  sus obstáculos. Es la cifra que acompaña el relato para dar peso al esfuerzo.
- **Otras actividades durante el episodio:** búsquedas de provisiones u otras
  interrupciones de supervivencia. Avanzan el mundo y consumen,
  pero no se presentan como tiempo dedicado a desplazarse.

Planificación, preparación y los tramos anteriores o posteriores no pertenecen
a ninguna de esas cuentas. Su suma sí forma parte del total general de Viajar.
El contador se reinicia al empezar otro episodio.

La cámara, fecha, memoria de exploración y posición del grupo siguen los
traslados confirmados. Entre centros se conserva el último Lugar alcanzado;
no se inventa una posición sobre tiles. Una escena puede resumir varios centros
atravesados, pero sus cambios y consumo no pueden omitirse.

### 5.3. Pérdidas e interrupciones

La pérdida de animal y comida actualiza reservas y capacidad antes de
continuar. Si el momento coincide con una llegada se revisa allí; de lo contrario,
se revisa la carga en la siguiente frontera de Lugar usando las
reglas existentes, sin aplicar cambios retroactivos al tiempo ya recorrido.
Las duraciones pendientes se calculan desde el progreso real, no desde el
comienzo de una conexión ya recorrida en parte.

La supervivencia puede interrumpir el relato con su búsqueda, aviso o decisión
de carga. Se conserva el punto de reanudación del episodio y no se repiten sus
momentos ni tiradas. Si se termina el viaje antes de cruzar, no se emite PM-06
ni se narra un éxito completo.

Seis o siete es la cantidad de momentos **principales**, no un límite que permita
suprimir acciones necesarias de la simulación. Las interrupciones justificadas
pueden añadir presentaciones.

## 6. Integración técnica

### 6.1. Responsabilidades

| Parte | Responsabilidad |
|---|---|
| `core/acciones/viajar/pasos_montanosos.rs`, nuevo | Reconocer la secuencia, mantener su cursor, ordenar momentos, resolver chequeos y efectos específicos. |
| `viajar/estado.rs` | Guardar el estado opcional del episodio y su continuación. |
| `viajar/mod.rs` | Delegar las conexiones del episodio y volver al flujo común al terminar o interrumpirse. |
| `viajar/registros.rs` | Publicar hechos del momento, pruebas, consecuencias y progreso del episodio. |
| `rutas`, `carga`, `reservas` | Conservar cálculos comunes, sin otra tabla de velocidades o un consumo paralelo. |
| `app/narracion_viaje` y `sesion` | Redactar párrafos, detalles y progreso; proyectar la posición confirmada. |
| Frontend terminal | Presentar momentos con la ventana, Enter, Detalles, mapa y retratos existentes. |

No se modifica la lógica de las tarjetas ordinarias para imponerles una
estructura de capítulo. Sí se necesitan puntos de integración en el coordinador,
el protocolo y la narración. No es suficiente crear un archivo que nadie invoque.

### 6.2. Estado y registros

El estado privado del episodio necesita, al menos: índices inicial y final de
la ruta, objetivo de momento pendiente, progreso dentro de la conexión, tiempo
ya contabilizado, siguiente momento principal, campamento emitido y continuación
tras una interrupción común. No guarda préstamos a Mundo entre llamadas.

Se define `MomentoPaso::{Panorama, CaminoDificil, Cornisa, Rodeos, AbrirPaso,
Campamento, OtraVertiente}` y una variante de `ContenidoEscena::PasoMontanoso`
con hechos propios. No se fuerza el episodio dentro de `Desplazamiento`, que hoy
exige exactamente dos chequeos de una sola conexión.

El registro público incluye: momento, extremos de la secuencia, cantidad de
Lugares, Lugar confirmado, siguiente Lugar si el plan es visible, previsión
inicial del episodio, traslado acumulado, tiempo de otras actividades, chequeo
opcional y consecuencia aplicada. La pérdida incluye cantidades concretas de
animal y alimentos; no se reconstruye desde el texto.

Cada momento sale mediante `PasoAccion::Escena`. El protocolo ya permite
`Decision`; no se agregan opciones vacías para anticipar contenido futuro.
Confirmar o repetir un ID inválido conserva los rechazos y garantías vigentes.
Los NPC ejecutan el mismo episodio y reciben un resumen breve en su fase.

### 6.3. Un solo lugar para confirmar efectos

El avance parcial no puede aplicar tiempo y consumo dentro del episodio y
volver a aplicarlos al publicar su escena. El diseño de implementación debe
separar el cálculo de intervalos, la confirmación de sus efectos y la publicación
del momento, reutilizando la contabilidad común exactamente una vez.

Si entre dos momentos se cruzan varios centros, se conservan sus operaciones en
orden y se controla cada frontera necesaria de supervivencia. Si hace falta
presentar una interrupción antes del siguiente momento, ese avance debe quedar
confirmado y visible en su progreso antes de pedir la decisión. No se puede
evaluar el resto del episodio con reservas o ubicación anteriores.

Las consultas de tiempo restante y revisión de carga requieren reconocer el
cursor parcial cuando hay un episodio activo. Sin episodio siguen el camino
actual. Para mantener esa diferencia localizada, el módulo del episodio
usa estas operaciones internas:

```rust
fn reconocer(mundo: &Mundo, ruta: &[IdLugar], posicion: usize)
    -> Option<SegmentoPaso>;
// Métodos de EjecucionAccion:
fn iniciar_paso_montanoso(&mut self, mundo: &Mundo, segmento: SegmentoPaso)
    -> Resultado<()>;
fn producir_paso_montanoso(&mut self, mundo: &mut Mundo,
    azar: &mut dyn FuenteAzar) -> Resultado<PasoAccion>;
// Método de EstadoPaso:
fn tiempo_pendiente(&self, ejecucion: &EjecucionAccion, mundo: &Mundo)
    -> Resultado<HorasViaje>;
```

`SegmentoPaso` fija los índices extremos. `EstadoPaso` conserva cursor, objetivos,
siguiente momento, duración fijada de la conexión parcial, previsión inicial,
traslado acumulado, instante inicial y revisión de carga pendiente. Usa el estado
del viaje y el Mundo recibido, sin archivos, colores ni texto narrativo.
`Contabilidad` reúne duración y consumo de los intervalos; las operaciones de
traslado se guardan en orden hasta publicar el momento. `Interrupcion` es un
momento de control adicional, nunca otro obstáculo ni una repetición del anterior.

La preparación trabaja sobre copias del estado privado y las reservas. Ordena
los intervalos como ocurrieron: primero traslado y consumo hasta el obstáculo,
después su consecuencia y finalmente consumo de la demora adicional. Así la
comida que cae no desaparece antes de haber alimentado al grupo durante los
días anteriores al obstáculo.

Publicar el resultado aplica una sola vez su lote de operaciones al Mundo y
adopta el estado preparado. `contabilizar` comparte el consumo y avance temporal;
`publicar_contabilizado` crea el registro sin volver a cobrarlo. Se extrajo esta
contabilidad de `registro`
para poder usarla tanto en este recorrido como en las tarjetas ordinarias;
estas últimas siguen invocando `registro` y conservan sus efectos. No se mantiene un segundo
algoritmo de raciones dentro del capítulo.

Un fallo de validación no deja un cursor adelantado con el Mundo atrasado, ni
viceversa. Si una interrupción requiere respuesta, se confirma primero el avance
hasta ella y se guarda su continuación. La previsión pendiente descuenta los
intervalos ya confirmados, incluyendo una conexión parcialmente recorrida.

Índices incoherentes, cursores fuera de la secuencia, desbordamientos temporales
y lotes inválidos producen `ErrorMotor` con contexto mediante el tratamiento
existente. No se corrige silenciosamente una ruta ni se confirma parte de un
momento fallido. Los IDs de escena antiguos mantienen sus rechazos normales.
No se necesitan dependencias, crates o formatos de contenido nuevos.

## 7. Narración y presentación

El párrafo de cada obstáculo sitúa el problema, describe el esfuerzo y explica
su resultado. Nombra al responsable cuando ayude a comprender qué ocurrió.
Evita anunciar en la introducción qué tirada fallará o cuánta comida se perderá.

En PM-02 a PM-06 se incorpora una frase con el traslado acumulado, como
«Llevás {tiempo} abriéndote camino por el paso». Se adapta al principio si aún
no transcurrió tiempo significativo. Campamento puede mencionarlo como balance
previo, sin confundir descanso adicional con traslado.

Los fallos transmiten retraso, riesgo y desgaste; el campamento ofrece contraste
y la última escena alivio. No se inventan heridas, hambre o animales presentes
si los datos no lo permiten. Se usan los nombres y descripciones de Lugares
cuando correspondan al momento, sin repetir el párrafo de paisaje en cada página.

La primera versión reutiliza la presentación normal de viñetas. No requiere una
interfaz nueva, arte de campamento procedural o cambios de paleta. Una página,
Detalle, animación o resize nunca vuelve a tirar el dado.

## 8. Puntos para futuros momentos

Entre los momentos principales quedan puntos de inserción para otros momentos
narrativos, chequeos o decisiones. Más adelante podrán usar una tabla finita y
una tirada que elija un evento.

En esta versión esos puntos no insertan contenido ni consumen azar. No se
implementa un framework genérico, un intérprete de escenas o una tabla con
resultados ficticios. Al incorporar una tabla real se documentarán sus opciones,
probabilidades, condiciones y consecuencias, manteniendo el progreso del episodio.

## 9. Verificación

| Regla | Casos necesarios |
|---|---|
| Reconocimiento | 1, 2, 3, 4, 5 y más Lugares; ambos rasgos exigidos; secuencia separada por otro terreno; inicio o destino dentro del paso. |
| Ruta adoptada | Propuesta, improvisación y vecino directo; no extender por otras conexiones del grafo. |
| Estructura | Cuatro obstáculos una vez; seis momentos sin campamento, siete con él, sin repetir introducción entre conexiones. |
| Centro | Campamento tras llegar al segundo de tres y tercero de cinco, también en sentido inverso; tamaños distintos no cambian el centro. |
| Tiempo | Conservación exacta al dividir conexiones; éxito sin demora; fallos de 8/24/48 h y suma de 80 h; sin conteo doble. |
| Provisiones | Consumo en fronteras correctas; pérdida de animal/comida; cero animales; carga pendiente e interrupción por carencia. |
| Reanudación | No repetir chequeos o efectos al continuar; cancelación durante interrupción sin falsa salida exitosa. |
| Contexto | Sin animales, solo Actor, varios compañeros, reservas vacías; texto fiel al resultado y al tiempo del episodio. |
| Integración | Fecha, cámara, traslados intermedios y memoria; NPC con resumen; otras tarjetas mantienen su comportamiento. |

Además se ejecutan los controles de la
[metodología](../../metodologia-desarrollo.md#7-controles-automáticos) y una
prueba jugable del episodio completo para revisar ritmo y contraste dramático.

## 10. Decisiones confirmadas y límites

No quedan preguntas bloqueantes para este alcance:

- Cuatro pruebas automáticas de Andar, dificultad 10, con el integrante más capaz.
- Sustituyen los incidentes ordinarios de las conexiones internas del episodio.
- Caer un animal pierde su parte proporcional de comida; el agua se conserva.
- El descanso central ya está incluido en el tiempo del traslado.

El redondeo de unidades y la ausencia de animales se especifican en PM-03 para
que no queden decisiones escondidas en el código. Cambiar esos parámetros será
una revisión de este contrato, sin alterar las otras tarjetas de Viajar.

Las tablas de eventos azarosos, decisiones del grupo, efectos persistentes de
fatiga, bonos de montaraz y nuevas pruebas particulares quedan para iteraciones
posteriores. No se agregan a la implementación inicial como efectos implícitos.

## 11. Cierre de implementación

El [PLAN-2026-045](../../planes/completados/2026-11-03-45-cruzar-pasos-montanosos.md)
registra el cierre técnico del 2026-11-03. Pasaron 263 pruebas del workspace,
Clippy, formato, compilación con y sin audio y build de producción. Se comprobó
el episodio de Zunir en PTY con una copia temporal del contenido.
La revisión de jugabilidad y ritmo narrativo queda para la prueba del usuario.
