# Reglamento de la acción general `Viajar`

> Estado: borrador en elaboración conjunta. Las decisiones abiertas figuran en
> [Pendientes](#pendientes); no deben completarse por analogía con versiones anteriores.

## Índice

1. [Alcance, responsabilidades y estado](#alcance)
2. [Curso del viaje](#curso)
3. [Rutas y medios de transporte](#rutas)
4. [Tiempo, reservas y consumo](#tiempo)
5. [Carga y preparación de configuraciones](#carga)
6. [Chequeos y selección de responsables](#chequeos)
7. [Información y narración](#presentacion)
8. [Tarjetas de viñeta](#tarjetas)
9. [Decisiones pendientes](#pendientes)
10. [Material retirado para revisión](#material-retirado) — fuera del cuerpo principal.

<a id="alcance"></a>
## 1. Alcance, responsabilidades y estado

`Viajar` comienza cuando el jugador decide trasladar su Grupo del Lugar actual a
otro Lugar. El recorrido puede cruzar varias conexiones y admitir distintas rutas.
Este reglamento cumple la función del director de juego: establece escenas,
chequeos y consecuencias para resolver situaciones semejantes de manera coherente.

| Componente | Responsabilidad |
|---|---|
| Acción general `Viajar` | Planifica y conduce el recorrido; mantiene tiempo, provisiones, carga y medios disponibles; invoca las tarjetas. |
| Tarjeta de viñeta | Define una clase de escena: disparador, información, decisiones, chequeos, consecuencias y continuación. |
| Acción particular | Resuelve un chequeo reutilizable con el Actor y la dificultad que recibe; conserva sus propios modificadores y devuelve los datos de la tirada. No decide contexto, relato ni consecuencias. |

Una viñeta de **resolución** resuelve la situación y después la relata, sin pedir
decisiones ni fragmentarla para mostrar información que el jugador no puede usar
para elegir. Una viñeta de **decisión** informa primero, permite elegir y aplica
después la opción y sus consecuencias. El paginado y las ventanas son cuestiones
de interfaz: no forman parte del reglamento ni determinan la cantidad de viñetas.

Durante la acción se mantienen la ruta adoptada, el Lugar actual, la composición
del Grupo, el tiempo total, las horas desde la última ración, las comidas
concretas, el agua y sus botas, los animales de carga, los contadores separados de
caballos y camellos y la disponibilidad del barco. Las cuentas del viaje son
temporales: las reservas no ingresan al inventario general y todo remanente se
descarta al terminar.
Solo persisten cambios que una regla establezca expresamente, como Características,
Relaciones o la separación y ubicación de Actores.

El reglamento no simula precios, dinero, comercio ni capacidad de pago. El Grupo
puede pagar lo que necesita; conseguirlo sigue sujeto a los procedimientos de
cada tarjeta. Las monturas y los animales de carga no son intercambiables.

<a id="curso"></a>
## 2. Curso del viaje

| Momento | Resolución |
|---|---|
| Destino | El jugador declara el Lugar al que quiere llevar al Grupo. |
| Planificación | Se determinan las rutas rápida y larga y se busca un proponente válido. Sin él, se usa [`VIA-RUT-01`](#via-rut-01); con él, [`VIA-RUT-02`](#via-rut-02). |
| Preparación | [`VIA-APR-01`](#via-apr-01) calcula y reúne provisiones. Se evalúan las configuraciones de carga y, si corresponde, [`VIA-APR-05`](#via-apr-05); se obtienen automáticamente los animales fijados. Si `CalcularProvisiones` fracasó, sigue [`VIA-APR-03`](#via-apr-03). |
| Desplazamiento | Antes de cada conexión se elige el medio con los contadores actuales. Se presenta exactamente una tarjeta: [`VIA-DES-NAV`](#via-des-navegar), [`VIA-DES-CAB`](#via-des-cabalgar), [`VIA-DES-CAM`](#via-des-camello) o [`VIA-DES-AND`](#via-des-andar). |
| Cierre de escena | Se aplican tiempo, consumo y consecuencias de cada escena. En un desplazamiento, salvo terminación expresa, el Grupo completa la conexión y el Lugar de destino de esa conexión pasa a ser el Lugar actual. |
| Destino final | Tras aplicar las consecuencias del último desplazamiento, se resuelve [`VIA-LLE-01`](#via-lle-01) y termina la acción. |
| Entre conexiones | Si alguna reserva quedó en cero, se resuelve [`VIA-APR-02`](#via-apr-02). Una pérdida de capacidad también exige la [comprobación de carga](#recalculo-carga), que puede activar esa búsqueda. Si continúa el viaje, se revisa la carga resultante y puede seguir [`VIA-APR-06`](#via-apr-06). |
| Continuación | Se completan las tarjetas intercaladas antes del siguiente desplazamiento y se repite hasta llegar o hasta que una regla termine o cancele el viaje. |

Las conexiones se recorren en orden; las tarjetas de desplazamiento no tienen una
frecuencia aleatoria. Las demás aparecen cuando se cumple su disparador.
`VIA-APR-02` se presenta como máximo una vez en cada secuencia entre conexiones,
incluso si coinciden agotamiento y pérdida de capacidad.

Toda terminación anterior a la llegada se cierra con el relato de la tarjeta
que la produjo y omite `VIA-LLE-01`. No concede estados ni Relaciones adicionales
por el mero hecho de interrumpir el viaje.

<a id="rutas"></a>
## 3. Rutas y medios de transporte

### 3.1. Construcción y elección de ruta

La **ruta rápida** es la ruta válida con menor tiempo previsto sin demora,
calculado con las reglas de tiempo. Se comparan días, no cantidad de conexiones;
si varias empatan, se elige una al azar. El algoritmo para encontrarlas pertenece
al diseño técnico.

La **ruta larga** alcanza el mismo destino por otro camino y contiene más Lugares.
Representa el recorrido de un Grupo sin conocimientos suficientes para planificar.
Si `C` cuenta los Lugares de la rápida, incluidos origen y destino, añade un entero
`E` que cumple:

```text
C ÷ 4 < E < C
mínimo de E = parte entera(C ÷ 4) + 1
máximo de E = C − 1
Lugares de la ruta larga = C + E
```

Cualquier técnica puede construirla si produce una ruta válida con esos extremos
y ese intervalo. Su duración se calcula con las mismas reglas que la rápida.

Puede proponer la ruta rápida cualquier Actor que cumpla al menos una condición:

- Posee `Viajero`.
- Posee `Explorador`.
- Tiene una Relación `Conoce` con al menos `redondear hacia arriba(C ÷ 2)` Lugares
  de la ruta rápida, contando ambos extremos. No necesitan ser consecutivos.

No hay chequeo ni grados de aptitud entre candidatos; si hay varios, se sortea
el proponente. La tarjeta de ruta determina la elección y la información visible.

### 3.2. Medio de cada conexión

Para comparar rutas se presumen disponibles barcos, caballos y camellos.
Después del aprovisionamiento se usan mientras sigan disponibles. La prioridad,
tanto para prever duración como para elegir tarjeta, es:

| Condición | Medio y tarjeta |
|---|---|
| Alguno de los dos Lugares es `Acuático` | Barco, `VIA-DES-NAV`, para toda la conexión, incluida su mitad no acuática. |
| Conexión terrestre, al menos un caballo por Actor y el caballo puede atravesar ambos Lugares | Caballo, `VIA-DES-CAB`. |
| No puede usarse caballo y hay al menos un camello por Actor | Camello, `VIA-DES-CAM`. |
| Ninguna especie alcanza para todos o puede usarse | A pie, `VIA-DES-AND`. |

El caballo no puede atravesar un Lugar simultáneamente `Desierto` y `Vasto`;
esa prohibición se comprueba antes de elegir el medio. Los demás terrenos que
reducen la velocidad montada se detallan en [Duraciones](#duraciones).

`VIA-APR-01` proporciona una montura por Actor de cada especie requerida por la
ruta. Si requiere ambas, obtiene ambas cantidades. Una muerte resta inmediatamente
una montura del contador de su especie y se conserva durante el viaje actual,
sin afectar viajes futuros. Antes de cada nueva conexión terrestre se reevalúa
la prioridad con los Actores y contadores actuales; una montura extra sirve como
reserva mientras siga habiendo una por integrante.

El barco está disponible automáticamente en el punto exacto donde se necesita:
no se busca, paga ni traslada mediante otra escena o chequeo. No es un objeto
persistente. Su naufragio termina el viaje actual, pero no impide disponer de
otro en viajes futuros. La transición entre tierra y agua forma parte de la
misma escena de navegación.

<a id="tiempo"></a>
## 4. Tiempo, reservas y consumo

<a id="duraciones"></a>
### 4.1. Duraciones previstas y reales

Un **tramo** recorre una conexión del centro del Lugar de origen al centro del
Lugar de destino. Cada Lugar aporta su mitad:

| Medio | Base entre dos Lugares normales | Base por mitad normal |
|---|---:|---:|
| Andar | 7 días | 3,5 días |
| Caballo | 4 días | 2 días |
| Camello | 5 días | 2,5 días |
| Barco | 2 días | 1 día |
| Animal de carga, en tierra | 5 días | 2,5 días |

```text
duración de una mitad = base por mitad × factor de tamaño
duración del tramo = duración de la mitad del origen + duración de la del destino
```

| Tamaño | Factor |
|---|---:|
| `Pequeño` | 1/2 |
| Normal, sin Característica de tamaño | 1 |
| `Vasto` | 2 |

Un Lugar con `Pequeño` y `Vasto` a la vez es un error de diseño: no tiene factor
definido. En una ruta, cada Lugar intermedio aporta una mitad de llegada y otra
de salida; los extremos solo aportan una.

Caballos y camellos usan la base de `Andar` en cada mitad `Montañoso`,
`Pantanoso` o `Boscoso`, antes de aplicar tamaño. Basta una de esas Características:
no acumulan efectos. El camello puede atravesar `Desierto` `Vasto` con su base
propia, salvo que también tenga alguno de esos terrenos lentos. Sustituir la
referencia temporal no cambia la tarjeta, su acción particular ni su tabla.

No se anota una velocidad numérica individual en cada Actor: el paso parte de
las bases universales y solo cambia mediante reglas expresas.
El Grupo acompaña al elemento limitante más lento. Un animal de carga limita el
paso terrestre solo si su referencia es más lenta que la del Grupo; varios
animales no vuelven a reducirlo y no limitan una conexión navegada. Ninguna otra
Característica altera las bases salvo regla expresa de este documento.

El **tiempo previsto sin demora** suma las duraciones con medios, tamaños y
modificadores conocidos durante la planificación; excluye carga, tiradas e
imprevistos futuros. Sirve para comparar rutas, presentar el plan y fijar la
necesidad base de provisiones. La [preparación de carga](#configuracion-inicial)
resuelve el efecto de los animales sobre esa referencia antes de partir.

La **duración real** usa la misma base y aplica velocidad limitante, estados,
demora por carga asignada e incidentes de la tarjeta. Primero se aplica tamaño;
después se suman o restan las modificaciones expresadas en horas. Se conservan
todas las fracciones, sin redondear calendario ni acumulaciones. Los
desplazamientos expresan su duración en días.

Solo pasa tiempo si una regla establece expresamente una duración; su omisión
equivale a 0 horas. Todo tiempo avanza el calendario y, desde que existen
reservas, el contador de consumo, incluso si después se cancela el viaje.
El plan no corrige ni limita la duración real y las demoras de ejecución no
reinician el aprovisionamiento inicial.

<a id="reservas"></a>
### 4.2. Reservas y materialización

Las unidades alimenticias miden internamente aporte y consumo; nunca se muestran
al jugador. Se conservan las cantidades concretas para calcular peso y relato.

| Comida, por pieza completa | Aporte oculto | Peso |
|---|---:|---:|
| Carne seca | 1 unidad alimenticia | 10 g |
| Pan | 2 unidades alimenticias | 12 g |
| Lonja de queso | 0,5 unidades alimenticias | 15 g |
| Pescado salado | 4 unidades alimenticias | 7 g |

Para cantidades `C`, `P`, `Q` y `S`, respectivamente:

```text
reserva alimenticia = C + 2P + 0,5Q + 4S
peso de comida = (10C + 12P + 15Q + 7S) ÷ 1000 kg
```

Para **materializar alimento** se inicia un lote vacío y se sortea una pieza de
cualquiera de los cuatro tipos en cada repetición, hasta alcanzar o superar el
objetivo. Se conserva el excedente de la última pieza; no se fracciona comida.
Es una operación lógica, sin nuevos intentos de los Actores ni tiempo adicional.

El agua se lleva en botas de 3 litros. El cuero pesa 0,1 kg y el agua 1 kg/litro:
una bota llena pesa 3,1 kg. Consumir agua reduce su peso, pero conserva el cuero;
una bota puede quedar parcial o vacía. Solo una escena que permita conseguir agua
puede permitir rellenarla.

Toda concesión de agua se **materializa** añadiendo botas completas y llenas:

```text
botas añadidas = redondear hacia arriba(litros objetivo ÷ 3 litros)
agua incorporada = 3 litros × botas añadidas
```

Los redondeos se aplican incluso si una regla impone un máximo de provisiones:
la última pieza o bota se conserva entera aunque lo supere. Los excedentes
cuentan como reserva y carga. `VIA-APR-02` fija su propia obtención de carne seca;
el agua que obtiene usa la conversión anterior.

<a id="consumo"></a>
### 4.3. Consumo y agotamiento

Cada Actor consume **2 unidades alimenticias y 0,5 litros de agua por cada
período acumulado de 24 horas**. No hay fracciones de ración por períodos menores.
Las tasas no reciben modificadores. Monturas y animales de carga no consumen
reservas del Grupo; su alimentación queda abstraída.

Al crear las reservas en `VIA-APR-01` comienza en cero el contador de horas desde
la última ración. Esa tarjeta añade sus propias 24 horas y provoca el primer
consumo. Toda escena posterior conserva y acumula las horas restantes:

```text
períodos completados = parte entera(horas acumuladas ÷ 24 horas)
alimento requerido = 2 × A × períodos completados unidades alimenticias
agua requerida = 0,5 × A × períodos completados litros
horas restantes = horas acumuladas − 24 horas × períodos completados
```

`A` es la cantidad de Actores aplicable a la escena. Las fronteras de 24 horas
se resuelven en orden. Para pagar cada ración alimenticia se sortea una pieza
completa entre los tipos que aún quedan y se repite hasta cubrirla o agotar la
reserva. Si la última pieza aporta de más, ese aporte sobrante se pierde y no
paga una ración posterior. La elección representa gustos momentáneos, no una
prioridad nutricional.

Alimento y agua se descuentan por separado. Si un recurso no alcanza, se consume
todo lo disponible y queda en cero; el otro se descuenta normalmente. Ninguna
reserva puede ser negativa. Las horas sobrantes pasan a la siguiente escena;
alcanzar exactamente 24 horas también provoca una ración.

La **carencia** de cada recurso comienza en la primera frontera cuya ración
completa no puede pagarse y dura hasta el final de la escena. Se registran por
separado el tiempo sin alimento y sin agua. Pagar la última ración completa y
quedar exactamente en cero no implica carencia: solo empezará si llega otra
frontera impagable.

El hambre o la sed no interrumpen un desplazamiento: el Grupo completa toda la
conexión con mayor esfuerzo narrado, sin tiempo, Características ni penalizaciones
adicionales por la carencia. Si quedan conexiones y alguna reserva terminó en
cero, se presenta `VIA-APR-02` antes de continuar; si se alcanzó el destino final,
se presenta la llegada.

Cuando una consecuencia aparta a un Actor durante una viñeta, no se recalculan
sus chequeos, duración ni consumo. El Actor deja de contarse al cerrarla;
las escenas posteriores, la capacidad de carga y las monturas usan entonces
la nueva composición.

<a id="carga"></a>
## 5. Carga y preparación de configuraciones

### 5.1. Capacidad y demora

Solo pesan las comidas, el agua actual y el cuero de sus botas. Los demás
suministros básicos quedan resumidos y no añaden peso.
Toda provisión adicional aumenta tanto las reservas como la carga; no forma
una categoría exenta de peso.

Cada Actor aporta 10 kg de capacidad sin demora y cada animal de carga transporta
hasta 100 kg de provisiones. Para `A` Actores, carga total `L` y `M` animales:

```text
capacidad animal = 100 kg × M
carga sobre los Actores = máximo(0, L − capacidad animal)
límite de los Actores = 10 kg × A
exceso x = máximo(0, carga sobre los Actores − límite de los Actores)
```

Si `T` es la duración prevista sin demora por carga del recorrido, en horas:

```text
demora por carga D = T × (x ÷ 10 kg)²
demora asignada a una parte de duración prevista t = t × (x ÷ 10 kg)²
```

Así se reparte la demora proporcionalmente entre conexiones y mitades, sin
duplicarla. `T` incluye la velocidad de animales que limiten el paso, tamaños y
demás factores conocidos; excluye la propia carga, tiradas e imprevistos.
Un exceso de 1 kg añade 1 % de `T`; 5 kg añaden 25 %; 10 kg, 100 %.

El exceso fijado al partir no disminuye por consumir reservas. Solo las
[comprobaciones durante el viaje](#recalculo-carga) sustituyen la demora pendiente;
nunca modifican tiempo ya transcurrido. No se pueden abandonar provisiones
libremente: esa decisión existe únicamente en `VIA-APR-06`.

<a id="configuracion-inicial"></a>
### 5.2. Configuración inicial y alternativa con un animal menos

Después de materializar las provisiones se calcula la cantidad mínima de
animales que elimina toda demora por carga:

```text
N = redondear hacia arriba(máximo(0, L − 10 kg × A) ÷ 100 kg)
```

Si `N > 0`, se evalúa además una sola alternativa con `N − 1`; nunca se
consideran dos o más animales menos. Cada animal recibe hasta 100 kg y los
Actores cargan el resto. Cada configuración usa su propia velocidad: pasar de
un animal a ninguno puede eliminar su limitación del paso.

La preparación debe cerrar también la relación entre velocidad y provisiones,
sin volver a elegir la ruta adoptada. `T` corresponde a la configuración
evaluada, inicialmente provisional y luego estabilizada.
Comienza calculando la necesidad base sin animales de carga, materializa las
reservas y evalúa las configuraciones permitidas. Si los 5 días de un animal
limitan una configuración, recalcula con esa duración las provisiones, su peso
y la cantidad de animales. Repite hasta que duración, reservas y animales sean
coherentes entre sí. Conserva el único chequeo de `CalcularProvisiones`; estas
operaciones no son nuevos intentos ni añaden tiempo. La velocidad de la
configuración aceptada queda fijada al partir y se aplica aunque resulte más lenta.

Para evaluar `N − 1`, la demora `D` exige un suplemento solo por las nuevas
fronteras de consumo que añade a su tiempo previsto `T`:

```text
jornadas suplementarias q =
  parte entera((T + D) ÷ 24 horas) − parte entera(T ÷ 24 horas)

suplemento para A Actores y q jornadas:
  carne seca = 2 × A × q piezas
  agua objetivo = 0,5 × A × q litros
  botas llenas = redondear hacia arriba(A × q ÷ 6)
```

La carne seca y el agua suplementarias están disponibles sin otro chequeo.
Su excedente por botas enteras cuenta como reserva y peso. El suplemento solo
cubre las jornadas creadas por la demora: no corrige un faltante de la necesidad
base ni se reduce por un excedente secreto del aprovisionamiento.

Como el suplemento pesa, se resuelve este ciclo:

1. Comenzar con `q = 0`, sin suplemento, y calcular la demora de esa carga.
2. Calcular las jornadas exigidas con la fórmula anterior. Si son `q`, cerrar.
3. Si son mayores, sustituir el suplemento tentativo por el **total** para esa
   cantidad de jornadas, redondeando las botas sobre toda el agua acumulada.
4. Recalcular la demora y restarle la anterior: la diferencia mide solo las
   horas creadas por el peso de esta repetición.
5. Si la diferencia supera 24 horas, descartar suplemento y alternativa:
   los `N` animales son obligatorios. Exactamente 24 horas todavía es admisible.
6. En otro caso, actualizar `q` a las jornadas exigidas y volver al paso 2.

Las incorporaciones son tentativas hasta que el jugador acepte. Si el ciclo
cierra, se ofrece `VIA-APR-05` con suplemento y demora finales; si no, se
obtienen los `N` animales sin decisión. Obtener cualquier cantidad fijada de
animales de carga es automático, sin responsable, chequeo, fracaso ni tiempo
extra.

<a id="recalculo-carga"></a>
### 5.3. Comprobación durante el viaje

Se comprueba la capacidad después de una viñeta que quite un Actor o animal de
carga y después de que `VIA-APR-02` añada provisiones, aunque no haya habido
una pérdida. Se hace tras cerrar la viñeta y aplicar su consumo, antes del
siguiente desplazamiento. Perder una montura afecta al medio, no a la capacidad
de carga.

Con la composición y el peso actuales, se recalcula `x` según 5.1. Si es cero,
termina la comprobación: no se revela si el aprovisionamiento inicial bastará.
Sigue vigente el disparador ordinario de búsqueda cuando una reserva llegue a cero.

Si `x > 0`, se obtiene `T restante` sumando las duraciones sin demora por carga de las
conexiones pendientes con ruta, medios, estados y modificadores actuales,
sin anticipar tiradas ni incidentes. Después:

```text
D restante = T restante × (x ÷ 10 kg)²
duración restante con carga = T restante + D restante
períodos restantes = parte entera(
  (h + duración restante con carga) ÷ 24 horas
)
alimento necesario = 2 × A × períodos restantes unidades alimenticias
agua necesaria = 0,5 × A × períodos restantes litros
```

`h` son las horas desde la última ración. Cada reserva se compara por separado;
igualar lo necesario alcanza.

Si falta alimento o agua, se resuelve primero `VIA-APR-02`, salvo que ya se
haya resuelto tras la misma conexión. Si el viaje continúa, se repiten los
cálculos con las reservas resultantes. Solo se considera la carga
**intransportable** y se presenta `VIA-APR-06` si persisten ambas condiciones:
`x > 0` y alguna reserva no cubre la nueva duración restante con carga.

Si las reservas alcanzan, se continúa con la nueva demora. Esta sustituye la
demora de las partes pendientes, sin sumarse a la anterior ni alterar el pasado.
La previsión especial no se usa fuera de estos disparadores para revelar si
la estimación inicial oculta fue suficiente.

<a id="chequeos"></a>
## 6. Chequeos y selección de responsables

Una acción particular recibe Actor y dificultad, tira 1d20 y suma sus
modificadores. La cara es el **resultado natural**; el total es el
**resultado ajustado**, limitado a 20 si lo supera. Igualar la dificultad es
éxito salvo excepción expresa. Devuelve la cara, cada modificador, el total
ajustado y si alcanzó la dificultad.

La lista de modificadores de cada acción es exhaustiva: ninguna Característica,
Relación o circunstancia ajena aporta bonos por analogía. Cada Característica
aplicable suma su bono una sola vez y varias se acumulan salvo excepción.
Una lista todavía incompleta no permite determinar definitivamente la aptitud.

Por defecto, la tarjeta elige al **Actor más apto** entre todos los integrantes
actuales, salvo exclusión expresa. Compara únicamente la suma de modificadores
que esa acción aplicaría y sortea entre quienes empaten en el máximo, incluso
si todos tienen cero. Se elige por acción particular: un mismo responsable
conserva sus chequeos salvo que la tarjeta indique otra distribución;
acciones diferentes pueden tener responsables diferentes.

Cuando una regla necesita un Actor afectado sin otro criterio, se sortea entre
los elegibles después de aplicar exclusiones; si queda uno, se lo elige.
Este sorteo no reemplaza la selección del responsable ni la del proponente.

| Acción | Capacidad evaluada | Modificadores propios | Uso en `Viajar` |
|---|---|---|---|
| `Andar` | Orientarse, leer terreno, elegir paso y abrir camino a pie. | `Vagabundo` +2. | `VIA-DES-AND`, dificultad 5. |
| `Cabalgar` | Guiar y dominar un caballo. | `Jinete` +2. | `VIA-DES-CAB`, dificultad 6. |
| `CabalgarEnCamello` | Guiar y dominar un camello. | `Jinete` +2. | `VIA-DES-CAM`, dificultad 6. |
| `NavegarEnBarco` | Interpretar viento y clima, gobernar velas y conservar rumbo útil. | `Navegante` +4. | `VIA-DES-NAV`, dificultad 14. |
| `CalcularProvisiones` | Calcular y reunir alimento, agua y suministros. | `Administrador` +2 y `Expedicionario` +2; ambas, +4. | `VIA-APR-01`, dificultad 10. |
| `Forajear` | Buscar alimento y avituallamiento en entorno salvaje. | `Expedicionario` +2 y `Herbalista` +2; ambas, +4. | `VIA-APR-02`, dificultad 14; actúan todos. |

Las dificultades pertenecen a estas tarjetas, no universalmente a las acciones.
No hay otros modificadores de dado confirmados. `Viajero`, `Explorador` y
`Conoce` habilitan la planificación; estados, tamaño, terreno y viento modifican
las reglas o duraciones de la viñeta, sin modificar estas tiradas.

La acción particular no elige responsable ni dificultad ni conoce Lugares o ruta.
Tiempo, consumo, Relaciones e incidentes pertenecen a la tarjeta y a `Viajar`.

Dos sorteos de 1d20 quedan fuera de las acciones particulares:

- **Incidente:** selecciona una consecuencia después de determinar que ocurrió.
  Lee la cara natural, sin dificultad, responsable ni modificadores.
- **Recuperación de montura:** azar simple tras su crisis; 8 o más se recupera,
  menos de 8 muere. Tampoco tiene responsable ni modificadores.

<a id="presentacion"></a>
## 7. Información y narración

Estos criterios orientan la redacción de las tarjetas. Cada tarjeta establece
su propio orden, tono y forma de presentar la información; puede adaptarlos
sin modificar las demás. Las cantidades visibles proceden siempre de las
cuentas del viaje y las unidades alimenticias permanecen ocultas.

Cada viñeta explica qué ocurrió y por qué mediante la ficción y los datos útiles.
Identifica circunstancias relevantes, responsable, intento, causas de facilidad
o dificultad, tiempo, consumo y consecuencias. Evita valoraciones vagas cuando
puede explicar la causa concreta.

Hace visibles la composición del Grupo y cualquier efecto de su cantidad de
integrantes, las Características y la carga cuando alteren el resultado.

| Aspecto | Información que debe comunicar |
|---|---|
| Plan frente a ejecución | Distinguir tiempo previsto sin demora de tiempo real y explicar cómo velocidad, tiradas, carga e imprevistos alteraron la referencia. |
| Tamaño | Identificar Lugar, `Pequeño` o `Vasto`, duración de su mitad y duración final del tramo. |
| Carga | Peso soportado por Actores, límite conjunto, exceso en kg y horas añadidas; hacer visible el efecto de un animal limitante. |
| Provisiones | Cantidades concretas de carne seca, panes, queso, pescado, botas y litros. Nunca unidades alimenticias. |
| Chequeos | Cada tarjeta decide si revela dificultad, cara, bonos y total; siempre relaciona causas y resultado. Navegación exige el detalle indicado en su tarjeta. |
| Incertidumbre inicial | Sin proponente no se revela la ruta ni su previsión. `VIA-APR-01` muestra responsable, capacidades y cantidades obtenidas, pero no la necesidad base ni si alcanzarán. |

El criterio actual para los desplazamientos es narrarlos en segunda persona,
desde el jugador, como un relato integrado y cronológico del recorrido,
descubrimientos e incidentes. Estos se narran en el Lugar donde suceden.
No se separa una descripción genérica de un informe numérico.
Identifican a quienes viajaron y su cierre declara qué Lugar alcanzó el Grupo
o dónde y por qué terminó el viaje.

El cierre reúne todo el consumo de la viñeta: enumera cada comida realmente
consumida y el agua bebida, aclara que se consumieron durante el recorrido y
muestra reservas finales. No interrumpe la narración en cada frontera de 24 horas.
Indica cuántos períodos de consumo completó la viñeta; si no se completó ninguno,
informa horas acumuladas y ausencia de nueva ración.
Si hubo carencia, distingue hambre y sed con sus duraciones; si una reserva
solo llegó a cero al pagar la última ración, dice que se agotó al llegar sin
describir carencia.

Los estados limitantes y sus demoras deben ser visibles, incluida la suma de
ambos si un Actor ya posee `Esguince` y `Cansado`. Si un incidente no concede
otro estado porque el Actor ya posee uno, explica que no empeoró.

La prosa puede variar, combinar u omitir ejemplos si conserva esa información.
Una fogata u otro adorno no es un acontecimiento obligatorio, no determina cuándo
se descuentan las raciones ni añade tiempo o estados.

<a id="tarjetas"></a>
## 8. Tarjetas de viñeta

Las tarjetas aplican las reglas compartidas anteriores y especifican aquí
disparadores, resultados, decisiones y requisitos propios.

### 8.1. Planificación

<a id="via-rut-01"></a>
#### VIA-RUT-01 — Partir sin un plan

**Resolución; 24 horas, antes de crear reservas.** Ocurre si no hay proponente
válido. Adopta la ruta larga y la mantiene oculta. Explica que el Grupo conoce
la dirección general, pero carece de un recorrido fiable y se orientará sobre
la marcha. No muestra duración, provisiones ni dificultades y no permite cancelar.
Sigue `VIA-APR-01`; el día transcurrido no consume reservas todavía inexistentes.

<a id="via-rut-02"></a>
#### VIA-RUT-02 — Propuesta de ruta

**Decisión; 24 horas, antes de crear reservas.** Identifica al proponente y el
conocimiento que lo habilita; describe principales Lugares, medios, dificultades
conocidas y tiempo previsto sin demora, sin garantizar duración real.

| Opción | Consecuencia |
|---|---|
| Aceptar el plan | Adopta la ruta rápida y sigue `VIA-APR-01`. |
| Rechazarlo y partir por otro camino | Adopta la ruta larga y sigue `VIA-APR-01`. |
| Cancelar | Termina `Viajar` antes de partir. |

No hay chequeo: `Viajar` calcula la ruta y sus estimaciones. El día pasa
cualquiera sea la elección, sin consumo de reservas. No revela estimaciones
de alimento o agua, necesidad base ni el cálculo posterior de aprovisionamiento.

<a id="via-desplazamiento"></a>
### 8.2. Desplazamientos

Son cuatro tarjetas independientes. Cada una define su secuencia, chequeos,
decisiones, consecuencias y presentación. Las coincidencias actuales no obligan
a conservar una misma resolución al desarrollar cualquiera de ellas.

`Viajar` entrega a la tarjeta elegida los dos Lugares, el Grupo y sus
Características, el Actor del jugador, medios, reservas, contador de consumo y
demoras aplicables. La tarjeta usa las cuentas de tiempo, consumo y carga de
las secciones 4 y 5, y establece aquí cómo transcurre su escena. La sección 7
aporta criterios de narración que cada tarjeta adapta por separado.

<a id="via-des-andar"></a>
#### 8.2.1. VIA-DES-AND — Andar

**Resolución; decisión ante separación con varios Actores.** Representa
orientación, lectura del terreno y apertura de camino, además del esfuerzo físico.

**Resolución del recorrido.** El Actor más apto realiza dos chequeos de `Andar`,
dificultad 5: origen y destino, con el mismo responsable. Usa la base de 7 días
y calcula las dos mitades con sus tamaños y estados.
Cada resultado ajustado ≥17 concede a todos los integrantes `Conoce` con el
Lugar de esa mitad, antes de resolver el incidente; pueden conocer ambos.

Solo hay incidente si ambos ajustados son <8 y alguno es <5. En ese caso tira
una vez 1d20 en la tabla siguiente; lo sitúa en la mitad del resultado ajustado
menor, o en destino si empatan. Un resultado ≥8 o dos éxitos evitan el incidente.

| 1d20 de incidente | Consecuencia |
|---:|---|
| 1–7 | **Agotamiento momentáneo.** Un Actor se agota; esperarlo añade 2 horas. No obtiene `Cansado`. |
| 8–13 | **Esguince.** Un Actor obtiene `Esguince` si no tiene ni `Esguince` ni `Cansado`; en otro caso conserva su estado. |
| 14–17 | **Pérdida del rumbo.** El Grupo toma un camino más difícil. Cada Actor sin `Esguince` ni `Cansado` obtiene `Cansado`; los demás conservan su estado. |
| 18–20, varios Actores | **Separación de un Actor.** Un integrante distinto del Actor del jugador queda en el Lugar del incidente. Se decide buscarlo o continuar. |
| 18–20, un solo Actor | **Enfermedad del Actor solitario.** Se detiene 24 horas y se recupera de una enfermedad desconocida. Obtiene `Cansado` solo si no tenía ninguno de los dos estados. |

**Paso a pie y estados.** Mientras limite el paso a pie, `Esguince` añade
4 horas por mitad y `Cansado`, 6;
equivalen a 8 y 12 horas por conexión normal, respectivamente. Se aplican después
del tamaño, sin multiplicar esas horas por su factor. El Grupo acompaña al
integrante más lento: las demoras no se multiplican por cantidad de afectados.
Los incidentes no acumulan un estado con el otro. Si un Actor ya posee ambos
por causa externa, sus demoras sí se suman: 10 horas por mitad, 20 por conexión
normal. `Viajar` no elimina esos estados.

Un estado adquirido en la mitad del origen afecta la del destino y las
conexiones posteriores; si se adquiere en destino, rige desde la siguiente
conexión.

**Separación.** Se sortea al perdido entre los demás integrantes; el Actor del
jugador nunca puede desaparecer. La desaparición se aplica antes de continuar:

- **Buscarlo:** cancela `Viajar` en el Lugar del incidente. La búsqueda y una
  posible reincorporación quedan fuera de esta acción.
- **Continuar:** el separado deja de intervenir en el Grupo y el Actor del
  jugador obtiene la Relación dirigida `Abandono` hacia él. Si ya existe,
  se conserva sin duplicarla ni modificarla.

Los dos intentos mantienen al responsable originalmente elegido. La composición
para la escena se trata según la regla común de pérdida de Actores.

**Presentación y cierre.** Relata la marcha en segunda persona siguiendo las
mitades y situando descubrimientos e incidente donde ocurrieron, con los datos
de la sección 7. Aplica las demoras y estados en ese orden, sin efectos
retroactivos. Completa la conexión salvo que la separación cancele el viaje;
`Viajar` aplica tiempo y consumo y continúa según la sección 2.

<a id="via-des-cabalgar"></a>
#### 8.2.2. VIA-DES-CAB — Cabalgar

**Resolución del recorrido.** El Actor más apto realiza dos chequeos de
`Cabalgar`, dificultad 6: origen y destino, con el mismo responsable. Usa la
base de 4 días, las mitades por tamaño y las restricciones de 4.1.
Cada ajustado ≥17 concede a todos los integrantes `Conoce` con el Lugar de
esa mitad, antes del incidente; pueden conocer ambos.

Solo hay incidente si ambos ajustados son <8 y alguno es <6. Tira una vez
1d20 en su tabla y lo sitúa en la mitad del ajustado menor; empate, destino.
Un resultado ≥8 o dos éxitos evitan el incidente.

| 1d20 de incidente | Consecuencia |
|---:|---|
| 1–6 | **Caballo obstinado.** Un Actor no consigue mantenerlo en el rumbo; corregir los rodeos añade 4 horas. |
| 7–11 | **Caballo detenido.** Se niega a continuar; reanudar la marcha añade 8 horas. |
| 12–15 | **Caballo asustado.** Corre en otra dirección y cae su jinete; reagruparse y regresar al camino añade 12 horas. |
| 16–18 | **Crisis del caballo.** Una fiera lo ataca y queda herido. Espera 48 horas y tira 1d20 sin modificadores: ≥8 se recupera; <8 muere. |
| 19–20 | **Muerte inmediata.** Muere sin recuperación. |

Se sortea el caballo entre los disponibles; si hace falta un Actor, se sortea
independientemente y puede narrarse que cabalgaba ese caballo.
Una muerte reduce inmediatamente el contador. Si aún alcanza un caballo por
Actor, una reserva lo sustituye sin cambiar medio ni velocidad. Si no alcanza:

- En origen, conserva el segundo chequeo de `Cabalgar` y el incidente único;
  solo la duración del destino usa la base de `Andar` con su tamaño, sin
  `Esguince`, `Cansado` ni otros modificadores personales de andar.
- En destino, conserva la duración montada de esa mitad.

Antes de la siguiente conexión, `Viajar` reelige medio según 3.2: puede usar
camellos si hay suficientes antes de recurrir a pie.

**Presentación y cierre.** Transmite rapidez, amplitud y grandes vistas,
sin presuponer un paisaje. En terreno lento mantiene al Grupo montado y explica
la causa: pendientes que obligan a medir apoyos en `Montañoso`, suelo incierto
en `Pantanoso` u obstáculos y paso estrecho en `Boscoso`. Informa duración
de la mitad afectada y cuánto habría tomado sin la restricción.

Si muere un caballo, informa cuántos quedan y si hay reserva. Si faltó desde
origen, narra que un integrante hizo a pie la mitad del destino, conservando el
segundo chequeo montado; si faltó en destino, aclara que el tiempo ya recorrido
no cambia. Relata las mitades e incidente en orden y completa la conexión;
`Viajar` aplica tiempo, consumo y continuación según las secciones 4 y 2.

<a id="via-des-camello"></a>
#### 8.2.3. VIA-DES-CAM — Cabalgar en camello

**Resolución del recorrido.** El Actor más apto realiza dos chequeos de
`CabalgarEnCamello`, dificultad 6: origen y destino, con el mismo responsable.
Usa la base de 5 días, las mitades por tamaño y las restricciones de 4.1;
puede atravesar `Desierto` `Vasto`.
Cada ajustado ≥17 concede a todos los integrantes `Conoce` con el Lugar de
esa mitad, antes del incidente; pueden conocer ambos.

Solo hay incidente si ambos ajustados son <8 y alguno es <6. Tira una vez
1d20 en su tabla y lo sitúa en la mitad del ajustado menor; empate, destino.
Un resultado ≥8 o dos éxitos evitan el incidente.

| 1d20 de incidente | Consecuencia |
|---:|---|
| 1–6 | **Camello obstinado.** Un Actor no consigue mantenerlo en el rumbo; corregir los rodeos añade 4 horas. |
| 7–11 | **Camello detenido.** Se niega a continuar; reanudar la marcha añade 8 horas. |
| 12–15 | **Camello asustado.** Corre en otra dirección y cae su jinete; reagruparse y regresar al camino añade 12 horas. |
| 16–18 | **Crisis del camello.** Lo pica un animal venenoso que nadie alcanza a ver. Espera 48 horas y tira 1d20 sin modificadores: ≥8 se recupera; <8 muere. |
| 19–20 | **Muerte inmediata.** Muere sin recuperación. |

Se sortea el camello entre los disponibles; si hace falta un Actor, se sortea
independientemente y puede narrarse que cabalgaba ese camello.
Una muerte reduce inmediatamente el contador. Si aún alcanza un camello por
Actor, una reserva lo sustituye sin cambiar medio ni velocidad. Si no alcanza:

- En origen, conserva el segundo chequeo de `CabalgarEnCamello` y el incidente
  único; solo la duración del destino usa la base de `Andar` con su tamaño,
  sin `Esguince`, `Cansado` ni otros modificadores personales de andar.
- En destino, conserva la duración montada de esa mitad.

Antes de la siguiente conexión, `Viajar` reelige medio según 3.2: puede usar
caballos si son suficientes y el terreno lo permite antes de recurrir a pie.

**Presentación y cierre.** Transmite avance sostenido, más rápido que a pie y
con el balance de la montura, sin presuponer un desierto. En terreno lento
explica los apoyos difíciles en `Montañoso`, el suelo incierto en `Pantanoso`
o los obstáculos en `Boscoso`, mostrando duración afectada y sin restricción.
En `Desierto` `Vasto` hace explícita la posibilidad de cruzarlo y su duración.

Si muere un camello, informa cuántos quedan y si hay reserva. Si faltó desde
origen, narra que un integrante hizo a pie la mitad del destino, conservando el
segundo chequeo montado; si faltó en destino, aclara que el tiempo ya recorrido
no cambia. Relata las mitades e incidente en orden y completa la conexión;
`Viajar` aplica tiempo, consumo y continuación según las secciones 4 y 2.

<a id="via-des-navegar"></a>
#### 8.2.4. VIA-DES-NAV — Navegar en barco

**Resolución.** Usa barco en toda conexión con algún extremo `Acuático`.
El Actor más apto realiza dos chequeos de `NavegarEnBarco`, dificultad 14:
origen y destino, con el mismo responsable. Aplica la base de 2 días y ambos
tamaños. Cada cara natural ≥18 concede a todos los integrantes `Conoce` con
el Lugar de esa mitad, antes del incidente; los bonos no ayudan y pueden
conocer ambos.

Un éxito ajustado ≥14 basta para una buena navegación y descarta incidentes.
Solo dos resultados <14 hacen tirar una vez 1d20 en su tabla; se sitúa en la
mitad del ajustado menor, o en destino si empatan.

La dificultad abarca viento, clima, velas, olas y rumbo, no solo conservar
dirección. En una buena navegación el tono es romántico, placentero, firme y
amplio; ante dificultades es serio y sombrío, mostrando al Grupo abriéndose
paso sin desesperanza. No exige un paisaje costero concreto.

**Viento estacional.** Se fija al comenzar la viñeta y no cambia aunque esta
cruce de mes:

| Mes inicial | Dirección hacia la que sopla |
|---|---|
| Enero, febrero, marzo | Este |
| Junio, julio | Oeste |
| Septiembre, octubre | Norte |
| Abril, mayo, agosto, noviembre, diciembre | Azar uniforme entre norte, noreste, este, sureste, sur, suroeste, oeste y noroeste. |

Barco y viento usan esos ocho puntos. La comparación cubre todos los casos:

| Relación | Criterio | Tiempo sobre la conexión completa |
|---|---|---:|
| A favor | Rumbo a un máximo de 45° de la dirección del viento. | −24 horas |
| Perpendicular | Diferencia de 90°. | 0 |
| En contra | Rumbo a un máximo de 45° de la dirección opuesta al viento. | +48 horas |

El viento no integra la previsión inicial. Se aplica una vez sobre el total
posterior al tamaño, junto con las demás sumas y restas de horas; su orden
aditivo no altera el resultado. Se admite duración real de 0 horas: dos Lugares
`Pequeño` dan 24 horas y el viento favorable puede eliminarlas.

**Incidentes.** La capacidad de navegación afecta a la frecuencia de consulta,
no al incidente sorteado.

| 1d20 de incidente | Consecuencia |
|---:|---|
| 1–6 | **Olas y desvíos:** +12 horas. |
| 7–12 | **Mal tiempo y lluvia:** +24 horas. |
| 13–16 | **Falta de viento:** inmóvil en el mar, +48 horas. |
| 17–18 | **Tormenta fuerte:** naufragio según el Lugar del incidente. |
| 19–20, varios Actores | **Actor por la borda:** un Actor distinto del del jugador sale del Grupo y queda en un Lugar `Costero` aleatorio. |
| 19–20, un solo Actor | **Tormenta fuerte**, en lugar de caída. |

El Actor por la borda se sortea entre los elegibles; su costa, entre todos los
Lugares `Costero` del mundo. Navegar en un mundo sin ninguno es un error de
diseño para el que no se define recuperación. La caída no añade tiempo.

En **Tormenta fuerte**, si el Lugar del incidente es `Costero`, el barco choca
contra esa costa y el Grupo queda allí. Si no lo es, se sortea independientemente
un Lugar `Costero` para cada Actor; todos salvo el del jugador salen del Grupo.
El del jugador queda como único integrante y su costa pasa a ser el Lugar actual.
En ambos casos termina `Viajar` sin llegada.

La tormenta no añade tiempo, muertes, pérdida de provisiones ni daños implícitos.
Se conserva el tiempo producido por las demás reglas del desplazamiento.

**Información propia.** Al cierre informa ambos resultados y si se aplicó el +4
de `Navegante`; si fallaron ambos, también la tirada de incidente y su
consecuencia. Desglosa base, tamaños, viento y demoras, además del cierre común
de tiempo y consumo.

Relata las mitades e incidente en orden. Completa la conexión salvo naufragio;
`Viajar` aplica tiempo y consumo y continúa según las secciones 4 y 2.

### 8.3. Aprovisionamiento y carga

<a id="via-apr-01"></a>
#### VIA-APR-01 — Aprovisionarse antes de partir

**Resolución; 24 horas.** Antes de recorrer la ruta, el Actor más apto realiza
una sola acción `CalcularProvisiones`, dificultad 10. No ofrece decisiones;
la eventual elección de carga pertenece a `VIA-APR-05`.

Para `A` Actores y `T` horas previstas de recorrido se calcula en secreto:

```text
períodos previstos = parte entera((24 horas + T) ÷ 24 horas)
necesidad base de alimento F = 2 × A × períodos previstos unidades alimenticias
necesidad base de agua W = 0,5 × A × períodos previstos litros
```

Las 24 horas iniciales cubren esta tarjeta. `T` usa los medios de la
configuración evaluada; la necesidad se fija al estabilizar la preparación
de 5.2. No garantiza el consumo real.

| Resultado ajustado | Multiplicador m sobre F y W |
|---|---:|
| <10, fracaso | 0,9 |
| 10–15, éxito | 1 |
| ≥16, éxito superior | 1,1 |

El mismo resultado determina ambos recursos: alimento objetivo `F × m` y agua
objetivo `W × m`. No hay tiradas separadas ni crece el beneficio por superar 16.
Se materializan con las reglas comunes y se conservan excedentes indivisibles.

La preparación resuelve la configuración inicial y su posible `VIA-APR-05`;
después obtiene automáticamente los animales de carga fijados. También establece
monturas por especie y disponibilidad del barco según 3.2. Estos medios no
requieren otro responsable ni chequeo y no añaden tiempo.

La escena identifica al responsable, las Características que lo ayudaron y
las cantidades concretas finales de cada comida y agua. Narra la obtención de
animales una vez fijada su cantidad. Con un plan puede relacionar provisiones
y previsión; sin él conserva la incertidumbre. No revela necesidad base ni si
las reservas alcanzarán.

Una vez formadas las reservas, las 24 horas de esta tarjeta provocan el consumo
de una ración por Actor. Si el chequeo fue <10, sigue `VIA-APR-03`; en los demás
casos comienza el recorrido.

<a id="via-apr-05"></a>
#### VIA-APR-05 — Decidir ante una sobrecarga moderada

**Decisión; 0 horas.** Solo aparece si la alternativa inicial con `N − 1`
animales es admisible. Usa los valores finales del ciclo, incluido el peso del
suplemento. Si no se cruzó una nueva frontera de consumo, el suplemento es nulo.

Identifica a quienes participaron del aprovisionamiento y presenta `N`,
`N − 1` —o ningún animal—, peso sobre Actores, límite, exceso, horas añadidas
y suplemento concreto de carne seca, botas llenas y litros. Con plan puede
mostrar tiempo total; sin plan solo revela las horas añadidas, no la base ni
el total ocultos.

| Opción | Consecuencia |
|---|---|
| Aceptar la demora | Incorpora el suplemento sin chequeo, fija una sola vez la demora final y obtiene `N − 1` animales. |
| Evitar la demora por carga | No incorpora suplemento, fija demora cuadrática cero y obtiene `N` animales. |

La cantidad elegida se obtiene automáticamente y no puede fracasar.

<a id="via-apr-03"></a>
#### VIA-APR-03 — Reconsiderar un aprovisionamiento incompleto

**Decisión; 0 horas.** Ocurre tras el fracaso de `VIA-APR-01` —objetivo del
90 %— o cuando `VIA-APR-02` deja alimento y agua para al menos un día pero no
para todo el recorrido restante.

Antes de partir advierte que se obtuvo menos de lo previsto y no se sabe si
alcanzará, sin revelar necesidad base ni duración de reservas. Durante el viaje
identifica el recurso insuficiente y el Lugar actual.

| Opción | Consecuencia |
|---|---|
| Viajar de todas formas | Conserva lo obtenido y comienza el recorrido o continúa desde el Lugar actual hacia la siguiente conexión. |
| Cancelar | Termina `Viajar` en el Lugar actual. |

El suplemento de sobrecarga solo cubre su demora y no corrige por sí mismo
el aprovisionamiento incompleto. Si cancela antes de partir, el Grupo devuelve
o vende provisiones y animales; durante el recorrido, se descartan las cuentas
temporales al terminar. No hay chequeo, nueva viñeta ni efecto económico.
Se conserva todo el tiempo y consumo anteriores.

<a id="via-apr-02"></a>
#### VIA-APR-02 — Conseguir alimento y agua durante el viaje

**Resolución; 6 horas.** Aparece antes de otra conexión, en el Lugar actual,
si alguna reserva terminó en cero o si la previsión por pérdida de capacidad
detecta que no alcanzará para el resto. No aparece después de llegar al destino.
Puede buscar solo alimento, solo agua o ambos; no ofrece una decisión.

Los **recursos necesarios** cubren todo el recorrido restante previsto, con
ruta, medios, estados y demora de carga conocidos, sin anticipar tiradas ni
incidentes. Incluyen las 6 horas de búsqueda y las `h` horas desde la última ración:

```text
períodos necesarios = parte entera(
  (h + 6 horas + duración restante conocida) ÷ 24 horas
)
alimento necesario = 2 × A × períodos necesarios unidades alimenticias
agua necesaria = 0,5 × A × períodos necesarios litros
```

Cada Actor actual realiza una vez `Forajear`, dificultad 14. Por cada éxito
se obtiene de forma independiente, para cada recurso buscado:

- Alimento: un entero aleatorio entre 2 y 10 piezas de carne seca, inclusive.
- Agua: una cantidad aleatoria entre 2 y 10 litros, inclusive.

Los fracasos no aportan recursos. Se suman los éxitos; el agua total se
convierte después a botas llenas. La comida de esta búsqueda es siempre carne
seca, en lugar de la mezcla aleatoria de otros aprovisionamientos.

Explica necesidad, oportunidades del entorno, intento de cada Actor y cantidades
logradas. Tras incorporar lo obtenido y aplicar las 6 horas con consumo normal:

1. Si alguna reserva no cubre **una ración completa de un día para todos**
   —2 unidades alimenticias y 0,5 litros por Actor—, cancela automáticamente
   en el Lugar actual, sin ofrecer `VIA-APR-03`.
2. Si ambas cubren un día pero no los recursos necesarios para todo el resto,
   presenta `VIA-APR-03`.
3. Si continúa, directamente o por esa decisión, comprueba siempre la carga
   con las nuevas reservas según 5.3. Si corresponde, presenta `VIA-APR-06`;
   en otro caso inicia la conexión siguiente desde el mismo Lugar.

<a id="via-apr-06"></a>
#### VIA-APR-06 — Afrontar una sobrecarga imprevista

**Decisión; 0 horas.** Solo aparece cuando la comprobación de 5.3 determina
carga intransportable: exceso actual y reservas insuficientes para el tiempo
restante con esa carga. Puede resultar de perder un Actor o animal de carga,
o de las provisiones de `VIA-APR-02`; la muerte de una montura no la activa.

La búsqueda que corresponda se resuelve primero, una sola vez en la secuencia.
Se recalcula con su resultado antes de presentar la decisión; nunca se modifica
la conexión ya terminada.

Explica la causa de la pérdida de capacidad o la nueva carga, cuántos Actores
y animales quedan, peso transportado, exceso, tiempo restante, demora y
reservas insuficientes. Calcula y muestra de antemano un **lote mínimo** de
comidas y botas enteras cuya retirada elimine todo `x`, dejando el peso dentro
de los 100 kg por animal más 10 kg por Actor.

Intenta repartir por igual el peso descartado entre alimento y agua, respetando
objetos indivisibles. Si una reserva no puede aportar su mitad, completa con
la otra. Muestra cada objeto del lote; el jugador no redefine cantidades.

| Opción | Consecuencia |
|---|---|
| Abandonar provisiones | Quita definitivamente el lote y verifica que desapareció la demora por exceso. Se permite aunque las reservas resultantes puedan ser insuficientes después. |
| Cancelar el viaje | Termina `Viajar` en el Lugar actual. |

No obtiene animales nuevos ni otra ayuda. Si el descarte deja una reserva en
cero, no repite `VIA-APR-02` en esta secuencia: puede completar la conexión
siguiente sin ese recurso según la regla común. Al terminarla, buscará si
quedan conexiones; si alcanzó el destino final, resolverá la llegada.

<a id="via-lle-01"></a>
### 8.4. VIA-LLE-01 — Llegar al destino

**Resolución; 0 horas.** Se presenta inmediatamente después del tiempo, consumo
y consecuencias del último desplazamiento. No realiza chequeos ni pide decisiones.

Cierra con una narración breve que nombra el destino. Una ruta de 2–4 Lugares,
contando ambos extremos, recibe un cierre ligero; una de 5 o más enfatiza
distancia, cansancio y satisfacción de llegar. Informa tiempo total y provisiones
concretas restantes, sin recapitular Lugares, chequeos, consumos, incidentes ni
decisiones del recorrido.

Identifica reservas agotadas y distingue agotamiento al llegar de carencia previa.
No busca más provisiones ni concede Características, Relaciones u otros estados.
Después de mostrar el remanente, termina `Viajar` y descarta sus cuentas temporales.

<a id="pendientes"></a>
## 9. Decisiones pendientes

Falta decidir qué ocurre cuando no existe una ruta válida, existe una sola o
no puede construirse una ruta larga dentro del intervalo exigido, y si una ruta
puede repetir un Lugar. No se completan estos casos por analogía. Cada decisión
confirmada debe incorporarse a su regla y retirarse de esta lista.

<a id="material-retirado"></a>
## 10. Material retirado para revisión — excluido del cuerpo principal

Este apartado conserva formulaciones que se retiraron para abreviar el reglamento
y que pueden recuperarse como banco de ejemplos. No agrega reglas: los requisitos
de información, tono y consecuencias están en el cuerpo principal. Los ejemplos
son opcionales; sus detalles de ambientación no añaden tiempo ni efectos.

Las repeticiones de reglas se consolidaron y no se archivan otra vez. Aquí se
conserva el contenido con posible valor propio: prosa concreta y algunas
explicaciones didácticas. Este apartado no cuenta para medir el tamaño del
reglamento reescrito.

### A. Ejemplos de marcha a pie, estados y cierre

> Dejás atrás el corazón de {origen} y te abrís camino junto al Grupo hacia
> {destino}. Recorrer esta mitad de {origen} exige {duración_origen}. Buscás
> referencias, examinás las huellas y elegís por dónde avanzar.

> Dejás atrás el corazón de {origen} y seguís a {responsable} rumbo a
> {destino}. Durante {duración_origen}, {responsable} busca referencias,
> examina las huellas y elige por dónde debe avanzar el Grupo.

> Cuando por fin distinguís {destino}, todavía queda internarse hasta su
> corazón. Esa mitad del recorrido exige {duración_destino}. El Grupo sostiene
> el paso y deja atrás, poco a poco, los lindes de {origen}.

- **`Pequeño`:** «{lugar} es Pequeño y sus confines quedan pronto a la espalda.
  Por su tamaño, recorrer su mitad toma {duración_mitad}, la mitad de lo que
  aportaría un Lugar normal».
- **`Vasto`:** «{lugar} se prolonga mucho más de lo que su primera silueta
  prometía. Por ser Vasto, recorrer su mitad toma {duración_mitad}, el doble de
  lo que aportaría un Lugar normal».

- «{responsable} no se limita a seguir adelante: aprende el
  terreno. Reconoce pendientes, huellas y referencias que el Grupo podrá
  volver a identificar. Al terminar esta parte del recorrido, todos sus
  integrantes obtienen la Relación `Conoce` con {lugar}».

- «Por un momento, las referencias de {lugar_del_resultado_menor} parecen
  contradecirse. {responsable} vuelve a examinar el terreno, corrige el rumbo y
  evita que la duda se convierta en una demora. El Grupo atraviesa la conexión
  sin una complicación adicional».

- **Agotamiento momentáneo (1–7):** «El esfuerzo termina por alcanzar a
  {actor_afectado}. Su respiración se vuelve corta y sus piernas dejan de
  responder al ritmo del Grupo. Nadie queda atrás: hacen un alto hasta que
  puede volver a ponerse en pie. La pausa añade 2 horas y el desplazamiento
  termina tomando {duración_real_total}».
- **Esguince (8–13):** «Un mal apoyo basta. {actor_afectado} trastabilla y el
  dolor le impide cargar el peso con normalidad. Le aseguran el pie como pueden
  y continúan a su paso. {cambio_de_Esguince}. La lesión añade
  {demora_aplicada} a este desplazamiento».
- **Pérdida del rumbo (14–17):** «Las referencias dejan de coincidir. La senda
  que parecía directa se deshace entre desvíos y, cuando vuelven a reconocer el
  rumbo, ya recorrieron mucho más de lo necesario. {cambios_de_Cansado}. El
  cansancio añade {demora_aplicada} a este desplazamiento».
- **Separación de un Actor (18–20, con varios Actores):** «Al reagruparse,
  contás una presencia menos. Llaman a {actor_perdido}, pero no llega respuesta;
  su último rastro se pierde en {lugar}. {actor_perdido} deja el Grupo y
  permanece allí. Debés decidir si detenés el viaje para buscarlo o si
  continuás hacia {destino_final} sin él».
- **Enfermedad del Actor solitario (18–20, con un solo Actor):** «Un dolor
  repentino te obliga a detenerte. Pasás un día entero abatido por una
  enfermedad que no alcanzás a reconocer. Cuando el dolor cede, podés volver a
  ponerte en marcha, pero la debilidad permanece: {cambio_de_Cansado}».

> Elegís no volver sobre tus pasos. Mientras el Grupo retoma el camino,
> {actor_perdido} queda atrás en {lugar}. Esta decisión establece una Relación
> `Abandono` desde {actor_jugador} hacia {actor_perdido}.

Cuando un resultado intenta entregar `Esguince` o `Cansado` a un Actor que ya
posee cualquiera de las dos Características, el texto debe decir que su estado
no empeora ni cambia. Por ejemplo: «El pie vuelve a resentirse, pero
{actor_afectado} ya carga con {estados_existentes}; conserva su estado y no
recibe otra Característica».

- **`Esguince`:** «{actor_limitante} no logra sostener el paso de los demás. El
  esguince le obliga a medir cada apoyo y todo el Grupo acompasa la marcha. En
  esta mitad, la lesión añade {demora_mitad}».
- **`Cansado`:** «El cansancio pesa sobre {actores_cansados}. Las pausas son más
  frecuentes y el paso se acorta. En esta mitad, `Cansado` añade
  {demora_mitad}».

> Cuando el tramo queda atrás, pasaron {duración_real_total}. La mitad de
> {origen} aportó {duración_origen} y la mitad de {destino},
> {duración_destino}; {resumen_de_demoras_y_causas}. Jornada tras jornada,
> durante el recorrido, los {cantidad_actores} integrantes consumieron en total
> {piezas_carne_seca} piezas de carne seca, {panes} panes,
> {lonjas_queso} lonjas de queso y {piezas_pescado_salado} piezas de pescado
> salado, y bebieron {litros_agua} litros de agua. En las reservas quedan
> {comidas_restantes} y {agua_restante} litros. Ahora que el camino inmediato
> queda atrás, el Grupo hace un alto. En el breve círculo de luz de una fogata,
> las brasas arrojan chispas contra la oscuridad mientras unos descansan y otros
> toman su turno de vigilancia. {estado_final}.

> Desde la última ración se acumularon {horas_acumuladas} horas. Durante este
> tramo el Grupo no completó otra jornada de consumo y conserva sus reservas.

> {Si faltó alimento: Durante {tiempo_sin_alimento}, avanzás con el Grupo bajo
> el peso del hambre, concentrado en alcanzar {destino}.} {Si faltó agua:
> Durante {tiempo_sin_agua}, la sed vuelve más áspero cada esfuerzo y obliga a
> medir cada paso.} Cuando por fin llegan, {resumen_de_recursos_agotados}. Antes
> de emprender otro desplazamiento tendrán que conseguir
> {alimento_agua_o_ambos}.

### B. Ejemplos comunes de monturas

> Desde la montura, {responsable} observa cómo se enlazan los pasos, las
> distancias y las referencias de {lugar}. Al terminar esta parte del recorrido,
> todos los integrantes del Grupo obtienen la Relación `Conoce` con el Lugar.

- **Montura obstinada (1–6):** «{actor_afectado} intenta mantener a {montura}
  en el rumbo, pero el animal se obstina en avanzar hacia otro lado. Entre
  rodeos y correcciones, el Grupo pierde 4 horas antes de recuperar el camino».
- **Montura detenida (7–11):** «{montura} se detiene y se niega a dar otro
  paso. Ni las riendas ni la insistencia consiguen moverla durante horas.
  Cuando por fin reanuda la marcha, el Grupo ha perdido 8 horas».
- **Montura asustada (12–15):** «Algo sobresalta a {montura}. El animal se
  lanza en otra dirección y {actor_afectado} cae mientras intenta dominarlo.
  Cuando el Grupo logra reagruparse por completo y regresar al camino, han
  pasado 12 horas».
- **Muerte inmediata (19–20):** «{montura} muere durante el trayecto. Quedan
  {monturas_restantes} {especie_de_montura} para los {cantidad_actores}
  integrantes del Grupo. {consecuencia_del_recuento_de_monturas}».

### C. Ejemplos de cabalgata y crisis del caballo

> Dejás atrás el corazón de {origen} al ritmo de los cascos. Durante
> {duración_origen}, {responsable} conduce al Grupo hacia {destino}; referencias
> que parecían lejanas se acercan, pasan a tu lado y pronto quedan atrás. Desde
> lo alto de la montura, el viaje se despliega en grandes vistas y el camino
> ganado se vuelve visible a cada momento.

> Cuando alcanzás los confines de {destino}, las monturas sostienen el impulso.
> Todavía quedan {duración_destino} hasta internarse en su corazón. El Grupo
> mantiene el ritmo mientras una nueva extensión del viaje se abre ante vos.

> Una fiera se abalanza sobre {caballo}. Logran ahuyentarla, pero la montura
> queda herida y no puede continuar. Te detenés con el Grupo durante 2 días
> completos para saber si podrá recuperarse.

> Al cabo de los 2 días, {caballo} vuelve a sostenerse y responde a la guía. El
> chequeo de recuperación obtuvo {resultado}, suficiente frente a dificultad 8,
> y el Grupo puede continuar a caballo.

> Al cabo de los 2 días, las heridas vencen a {caballo}. El chequeo de
> recuperación obtuvo {resultado}, por debajo de dificultad 8, y la montura
> muere. Quedan {caballos_restantes} caballos para los {cantidad_actores}
> integrantes del Grupo. {consecuencia_del_recuento_de_monturas}.

### D. Ejemplos de camello y picadura

> La extensión de {lugar} impediría atravesarlo a caballo. Los camellos pueden
> sostener la marcha a través de este `Desierto` `Vasto`; recorrer su mitad toma
> {duración_mitad}.

> Dejás atrás el corazón de {origen} sobre el paso largo y regular de los
> camellos. Durante {duración_origen}, {responsable} mantiene el rumbo hacia
> {destino}. Desde la altura de la montura contemplás cómo las referencias
> distantes se acercan poco a poco y cómo el trayecto ya recorrido se extiende
> a espaldas del Grupo.

> Los camellos cruzan los confines de {destino} sin quebrar su cadencia. Durante
> {duración_destino}, el Grupo continúa hacia su corazón; el balance de las
> monturas acompaña un avance constante mientras el nuevo Lugar ocupa cada vez
> más de tu vista.

> Una sacudida detiene a {camello}. Encontrás en su cuerpo la marca de una
> picadura, pero nadie llegó a ver al animal venenoso que la causó. Te detenés
> con el Grupo durante 2 días completos para saber si podrá recuperarse.

> Al cabo de los 2 días, el efecto del veneno cede. {camello} vuelve a sostener
> el paso; el chequeo de recuperación obtuvo {resultado}, suficiente frente a
> dificultad 8, y el Grupo puede continuar en camello.

> Al cabo de los 2 días, el veneno termina por vencer a {camello}. El chequeo de
> recuperación obtuvo {resultado}, por debajo de dificultad 8, y la montura
> muere. Quedan {camellos_restantes} camellos para los {cantidad_actores}
> integrantes del Grupo. {consecuencia_del_recuento_de_monturas}.

### E. Ejemplos de navegación, viento e incidentes

> El casco de {barco} corta las olas con un ritmo firme. Bajo la guía de
> {responsable}, las velas sostienen el avance hacia {destino}; sobre vos, las
> nubes cruzan ligeras y dejan pasar haces de sol que recorren el agua. Durante
> {duración}, el barco gana distancia con una facilidad que vuelve placentero el
> viaje. Los chequeos obtuvieron {resultado_origen} y {resultado_destino}; al
> menos uno alcanzó la dificultad 14 y la conexión se resolvió como una buena
> navegación.

- **A favor:** «El viento empuja hacia el rumbo del Grupo, hincha las velas y
  permite ganar un día respecto del tiempo normal».
- **Perpendicular:** «El viento cruza el rumbo. {responsable} ajusta las velas
  para aprovecharlo sin ganar ni perder tiempo».
- **En contra:** «El viento se opone al rumbo. El Grupo alterna bordadas,
  esperas y avances trabajosos hasta encontrar impulsos útiles; superar esa
  resistencia añade 2 días».

- **Olas y desvíos:** «Las olas apartan una y otra vez la proa del rumbo.
  {responsable} recupera cada desvío, pero las correcciones añaden 12 horas».
- **Mal tiempo y lluvia:** «La luz se apaga bajo las nubes y la lluvia golpea cubierta y
  velas. El Grupo sostiene las maniobras bajo el mal tiempo; cuando el cielo
  finalmente concede paso, se perdió 1 día».
- **Falta de viento:** «Las velas caen flojas. Durante 2 días, el barco permanece sobre
  un mar sin impulso mientras el Grupo vigila el horizonte y espera la menor
  señal de viento».
- **Tormenta fuerte en un Lugar `Costero`:** «La tormenta empuja a {barco} hacia tierra. Aun
  trabajando juntos contra el viento y las olas, el casco golpea la costa de
  {lugar}. El Grupo queda varado; esta travesía no puede continuar y el viaje
  termina en la costa».
- **Actor por la borda:** «Una sacudida arroja a {actor_afectado} por la borda. Cuando el
  incidente termina, ya no forma parte del Grupo y queda en {lugar_costero}».

### F. Ejemplos de llegada

> Al cabo de {tiempo_total}, alcanzás con el Grupo el corazón de {destino}. El
> trayecto queda atrás. {estado_breve_de_las_provisiones}.

> Después de {tiempo_total}, {destino} finalmente se abre ante vos. El
> cansancio y la distancia recorrida dan otro peso a la llegada. El Grupo entra
> en el Lugar. {estado_breve_de_las_provisiones}.

### G. Explicaciones didácticas y justificaciones retiradas

- Con un mismo medio, todos los Lugares normales y sin otros modificadores,
  ordenar rutas por cantidad de conexiones equivale a ordenarlas por días.
  Los tamaños pueden invertir esa comparación: más Lugares pequeños pueden
  tomar menos tiempo que menos Lugares vastos.
- Ejemplos de duración: andar entre dos Lugares normales toma 3,5 + 3,5 = 7
  días; de uno `Vasto` a uno `Pequeño`, 7 + 1,75 = 8,75 días. Cabalgar de un
  Lugar normal `Boscoso` a uno normal sin terreno lento toma 3,5 + 2 = 5,5 días.
- Ejemplo de carga: con `T = 168 horas` y 2 kg de exceso, la demora es
  `168 × (2 ÷ 10)² = 6,72 horas`.
- Ejemplo de suplemento: si `T = 30 horas`, 17 horas de demora no cruzan una
  nueva frontera de consumo; 18 horas llegan a la hora 48 y exigen otra ración.
- Ejemplo de acumulación: escenas de 20 y 6 horas completan una jornada y dejan
  2 horas para la siguiente. Exactamente 24 horas también consumen.
- La alimentación de los animales estaba justificada suponiendo que los
  herbívoros encuentran comida y los camellos dependen de sus reservas de agua.
  Se conserva la regla de que no consumen provisiones, sin desarrollar esa ficción.
- Usar solo la referencia temporal de `Andar` tras la muerte de una montura
  evita cambiar de acción particular dentro de una viñeta cuyos dos chequeos
  montados ya fueron resueltos.
