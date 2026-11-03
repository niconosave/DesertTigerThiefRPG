# Contrato técnico del rediseño de `Viajar`

| Campo | Valor |
|---|---|
| Estado | Implementado; revisión estética conjunta pendiente |
| Versión | 1.5 |
| Fecha | 2026-11-03 |
| Implementación | Motor único activo; verificaciones en el informe enlazado al final |

> Ampliación implementada el 2026-11-03: el
> [contrato de propuestas de caminos](./08-planificacion-rutas.md) define la
> sustitución de la planificación binaria y su selección en el mapa. Su sección 1
> delimita los apartados que reemplaza; el resto de este contrato se conserva.

## 1. Alcance y autoridad

La fuente de las mecánicas es el
[reglamento de Viajar](./borradores/05-viajar-redisenio.md). Este contrato define
su implementación: módulos, datos, APIs, algoritmos, transiciones, errores y
verificaciones. Las tablas de incidentes, dificultades y textos obligatorios
permanecen en el reglamento; no se mantienen copias de ellas aquí.

Para implementar este refactor, este contrato sustituye los diseños de
[expediciones](./02-viajar-expediciones.md),
[cronología](./03-cronologia-causal-viaje.md) y
[proyecto humano](./04-proyecto-humano-viajar.md) en lo relativo a `Viajar`.
También sustituye los apartados incompatibles de los componentes existentes
únicamente en los puntos enumerados en la sección 16. Esos documentos conservan
su valor histórico; sus contratos incompatibles ya no describen el motor activo.

Se mantienen un único proceso y la dependencia `terminal → app → core`.
Se reemplaza la orquestación interna anterior de `Viajar`, sus pronósticos y sus
resoluciones particulares incompatibles. No se mantiene una segunda versión del
viaje seleccionable en producción ni se crean servicios, plugins o intérpretes
de tarjetas. Las tarjetas son módulos Rust, identificados por claves estables.

El alcance incluye adquisición dinámica de la Relación existente `Conoce` y
separación de compañeros. No añade tipos de Relación; `Abandono` queda aplazada.
No incluye economía, inventario persistente, guardado de partidas, entidades
individuales de monturas ni nuevas acciones generales.

## 2. Organización del código y dependencias

```text
desert-tiger-core/src/
  acciones.rs                         fachada del motor
  acciones/particulares/
    chequeos.rs                       acciones nuevas y modificadores exhaustivos
  acciones/viajar/
    mod.rs                            ejecución reanudable
    estado.rs                         estado privado y transiciones
    registros.rs                      resultados semánticos e información pública
    rutas.rs                          rápida y alternativa
    tiempo.rs                         horas exactas internas
    reservas.rs                       piezas, botas y consumo
    carga.rs                          configuraciones, suplementos y descarte
    tarjetas/
      rutas.rs                        VIA-RUT-01 y VIA-RUT-02
      andar.rs                        VIA-DES-AND
      cabalgar.rs                     VIA-DES-CAB
      camello.rs                      VIA-DES-CAM
      navegar.rs                      VIA-DES-NAV
      preparacion.rs                  VIA-APR-01, VIA-APR-03 y VIA-APR-05
      forrajear.rs                    VIA-APR-02
      sobrecarga.rs                   VIA-APR-06
      llegada.rs                      VIA-LLE-01
  calendario.rs                       tipos existentes y puente de fracción de día
  modelo.rs                           operaciones persistentes ampliadas
desert-tiger-app/src/
  contexto_viaje.rs                    rumbo provisional desde centros
  narracion_viaje.rs                   proyección pública y redacción
  tiempo.rs                           coordinación del avance del jugador
  sesion.rs                           ejecución, vistas y entradas semánticas
  fase_npc.rs                         recorrido de cero o más políticas
```

Los nombres de archivo fijan responsabilidades, no un orden de desarrollo.
Cada tarjeta posee su resolución. Comparte funciones puras de tamaño, paso,
carga, consumo o selección, sin una macro ni una única tabla que obligue a que
las cuatro formas de desplazamiento evolucionen juntas.

El núcleo no consulta filesystem, Lua, nombres visibles, arte ni Ratatui.
No se necesita una dependencia nueva: las cantidades y los cálculos exactos
usan enteros comprobados de Rust. El adaptador de rumbo usa `atan2` de la
biblioteca estándar, exclusivamente para dirección geográfica.

## 3. Estado y tipos propios

Los identificadores existentes `IdActor`, `IdLugar` e `IdRelacion` se conservan.
`EjecucionAccion` posee sus datos; no guarda préstamos a `Mundo`, al catálogo ni
a la fuente de azar entre llamadas.

```rust
// Esquema conceptual: los campos privados concretos están en estado.rs.
struct EstadoViaje {
    dirigente: IdActor,
    viajeros: Vec<IdActor>,
    destino: IdLugar,
    ruta: Vec<IdLugar>,
    posicion: usize,
    inicio: InstanteAccion,
    transcurrido: HorasViaje,
    reservas: Option<ReservasViaje>,
    horas_desde_racion: HorasViaje,
    caballos: u64,
    camellos: u64,
    animales_carga: u64,
    carga_pendiente: PlanCarga,
    plan_visible: bool,
    fase: FaseViaje,
    secuencia: u64,
}
```

`viajeros` incluye al dirigente una vez y después a sus acompañantes por ID.
Para el jugador se construye desde el Grupo real; para un NPC independiente
contiene solo al NPC. El Grupo persistente sigue excluyendo al Actor jugador,
según el modelo existente. Las reglas que protegen al jugador protegen al
dirigente en una ejecución NPC.

`posicion` identifica el Lugar actual dentro de la ruta, salvo terminación por
reubicación: esta conserva su Lugar de cierre explícito. No se deduce posición
desde un tile. Cada escena congela sus participantes iniciales en
`ParticipantesEscena`; sus chequeos y consumo no se recalculan si después aparta
a uno de ellos.

`ReservasViaje` solo existe desde la confirmación de `VIA-APR-01`. No se guarda
en globales, Características ni inventario del Mundo. La finalización devuelve
su resumen antes de destruirla.

## 4. Tiempo exacto y traducción al calendario

### 4.1. Representación interna

`HorasViaje(u128)` tiene campos privados y usa `100_000_000` partes por hora.
Un día equivale a `2_400_000_000` partes. Solo admite valores no negativos.

Esta escala es exacta para este reglamento: todas las mitades, tamaños y estados
producen horas enteras, y el exceso de carga se mide en gramos enteros. Por eso:

```text
demora en horas = T_horas × x_gramos² / 10_000²
demora en partes = T_horas × x_gramos²
```

No se usan flotantes ni redondeos para comparar rutas, jornadas, reservas o
configuraciones. Toda suma, producto, resta y conversión usa operaciones
comprobadas. Un desbordamiento es un error explícito, nunca saturación, demora
cero ni truncamiento. La escala debe revisarse si una regla futura introduce
fracciones que no pueda representar.

### 4.2. Puente compartido

`Fecha` y `Duracion` conservan sus contratos de meses y días enteros. Se añaden
los valores neutrales siguientes en el módulo `calendario`:

```rust
pub struct FraccionDia(u32) // siempre menor que 2_400_000_000
pub struct TiempoAccion {
    pub duracion: Duracion,
    pub fraccion: FraccionDia,
}
pub struct InstanteAccion {
    pub fecha: Fecha,
    pub fraccion: FraccionDia,
}
pub fn proyectar_tiempo(
    inicio: InstanteAccion,
    tiempo: TiempoAccion,
) -> Result<InstanteAccion, ErrorCalendario>;
```

Constructores comprobados garantizan la fracción canónica. El viaje convierte
su total exacto a días completos y fracción mediante cociente y resto;
`TiempoAccion` no publica el acumulador privado `HorasViaje`.

`Partida` conserva el resto de día mundial, inicialmente cero. La fecha entera
permanece en `Mundo`. `proyectar_tiempo` suma primero la duración según el
calendario existente y después el acarreo de las fracciones. Una acción sin
fracción conserva el resto anterior.

La aplicación obtiene la fecha mostrada de esa función pura y confirma el mismo
resultado una sola vez tras cerrar la última viñeta. Prevalida el acarreo antes
de modificar fecha o resto. Ejemplo: 50 horas desde resto cero avanzan dos días
y conservan dos horas; una acción posterior de 22 horas avanza otro día.
Los NPC proyectan su cronología, pero no confirman fecha ni resto mundial.

La aplicación entrega días y horas formateadas como texto; el frontend no
calcula fechas ni divide cantidades. Una duración se presenta como días
completos y, si existe, el resto en horas decimales exactas: «2 días y 6,72 horas».
Así se evitan decimales periódicos de día. El formato no retroalimenta cuentas.
El mes del viento procede del instante proyectado al comienzo de esa escena,
incluido todo el tiempo previo, no de la fecha mundial todavía sin confirmar.

## 5. Reservas, materialización y consumo

### 5.1. Cantidades

`Comidas` contiene cuatro contadores `u64`, en orden canónico: carne seca, pan,
queso y pescado salado. El aporte usa medias unidades enteras; los valores del
reglamento se multiplican por dos. El peso se mide en gramos.

`ReservasViaje::botas: Vec<u16>` conserva entre cero y 3.000 ml por bota y
siempre 100 g de cuero por entrada. Su índice identifica la bota dentro de las
reservas de esa decisión, también si varias son iguales. Se consume por orden
de índice, agotando una antes de abrir otra. El lote queda congelado mientras
la decisión espera; tras un descarte se compactan los índices para la próxima
escena. No son identidades persistentes del Mundo. Una bota vacía sigue pesando.

Los objetivos del 90/100/110 % se representan como cocientes enteros. Para
materializar comida se compara el aporte contra el objetivo por productos,
sin redondear el objetivo hacia abajo. Cada pieza se sortea uniformemente entre
los cuatro tipos, conserva su aporte completo y se añade hasta alcanzar el
objetivo. El agua se convierte una sola vez al total de botas llenas necesario.
No se rellenan automáticamente las botas anteriores al añadir agua de forrajeo.

### 5.2. Una sola función de consumo

`reservas::consumir(reservas, h_anterior, duracion, cantidad_participantes, azar)` devuelve
las reservas posteriores, el resto de horas y un `RegistroConsumo`. Su ejecución:

1. Recorre en orden las fronteras de 24 horas alcanzadas, incluida la del final.
2. En cada frontera, recorre los participantes congelados en orden canónico.
3. Cada Actor paga su ración alimenticia sorteando uniformemente entre los
   tipos no vacíos, quitando piezas completas hasta cubrirla o agotar comida.
   El excedente de su última pieza se pierde y no paga la del siguiente Actor.
4. Descuenta el agua requerida independientemente del resultado alimenticio.
5. Registra por recurso la primera frontera cuya ración no pudo completarse.
6. Conserva el resto inferior a 24 horas para la escena siguiente.

El registro agrega piezas y agua realmente consumidas, fronteras completadas,
duración de carencia de cada recurso y cantidades realmente gastadas. El
registro de escena conserva las reservas finales; no duplica una copia inicial. Una
ración incompleta en la frontera final tiene carencia de duración cero; una
última ración pagada que deja saldo cero no es carencia. Las fronteras sin
reservas no producen valores negativos ni penalizaciones añadidas.

La planificación anterior a `VIA-APR-01` registra tiempo sin consumir. La
preparación incorpora reservas antes de contabilizar sus propias 24 horas.
`VIA-APR-02` incorpora lo obtenido antes de consumir sus seis horas.

Las comprobaciones de suficiencia usan el aporte agregado indicado por el
reglamento; no simulan futuras comidas ni gastan azar. El desperdicio de piezas
puede hacer que el consumo real difiera de esa previsión.

## 6. Rutas, medios y rumbo provisional

### 6.1. Ruta rápida

El [contrato 08](./08-planificacion-rutas.md#3-algoritmo-de-propuestas) define
el evaluador temporal común, rápida, perfiles y decisiones. Solo se recorre
`Mundo::lugares_conectados`; la ruta directa evita las búsquedas. Se rechaza un
destino inválido o sin conectividad antes de gastar tiempo.

### 6.2. Alternativa larga

La ruta improvisada conserva el constructor de rutas simples, sin repetir
Lugares, con el intervalo y excepción definidos por el contrato 08.

Se parte de la rápida y se buscan desvíos que sustituyan una conexión por un
camino con Lugares nuevos. Para cada conexión, BFS excluye los demás Lugares de
la ruta y la arista sustituida en ambos sentidos. Solo admite caminos con al
menos un Lugar interior nuevo y usa vecinos barajados una sola vez con
`FuenteAzar`. Se descartan los
desvíos que excedan el máximo de Lugares. Se elige al azar entre las ampliaciones
admisibles y se repite hasta entrar en el intervalo o agotar ampliaciones.
Cada aceptación aumenta la cantidad de Lugares; el algoritmo termina sin
enumerar todos los caminos simples del grafo.

Si no alcanza el intervalo, conserva la mejor ampliación encontrada; si no hay
ninguna, conserva la rápida. No garantiza encontrar una alternativa dentro del
intervalo aunque una búsqueda exhaustiva pudiera hallarla. Esta limitación
computacional pertenece al algoritmo inicial y no elimina ninguna escena de la
ruta finalmente adoptada. Las referencias y las propuestas quedan fijadas una vez.

### 6.3. Medios y tiempos por mitad

`rutas::medio` es una función pura que aplica la prioridad del reglamento
a los contadores y participantes actuales. Se invoca antes de cada conexión;
no elige otro verbo durante una tarjeta que ya resolvió sus dos chequeos.
La muerte de montura solo cambia la referencia temporal del destino cuando
su tarjeta lo indica. No crea una prueba de `Andar` ni aplica estados de andar
en esa sustitución temporal.

`rutas::base_mitad` establece la referencia temporal por terreno. `montanoso`
prevalece sobre bosque y pantano: 108 horas por mitad normal (9 días por Lugar
completo), o 120 horas (10 días) si hay monturas o animales de carga. Afecta a
todos los medios, incluido caminar, sin convertir el medio ni la prueba. Basta
una montura, aunque no alcance para todo el Grupo. Todavía no interviene un
montaraz. La búsqueda rápida conserva su supuesto de medios presumidos, con
monturas; la previsión y ejecución usan los animales reales del viaje.

`rutas::mitad` aplica después el tamaño y los estados. La misma base se usa en
previsión, carga y ejecución, y se publica en el desglose de Detalles. La pérdida
de montura conserva las reglas temporales de su tarjeta: solo una pérdida en
origen que obliga a seguir a pie cambia la referencia de destino.

Las tarjetas conservan base, tamaño,
restricción de terreno, limitante animal y estado en `MitadRecorrida`. La ejecución añade las horas de carga
preasignadas, viento e incidente; no aplica el factor de carga a esperas,
tormentas ni viento. No se rehace la ruta por pérdidas o nuevos estados.

### 6.4. Adaptador del rumbo

`desert-tiger-app::contexto_viaje` construye un catálogo inmutable de rumbos por
conexión dirigida navegable usando los centros de los dos Lugares:

```text
dx = x_destino − x_origen
dy = y_origen − y_destino   // en terminal Y crece hacia abajo
angulo = atan2(dy, dx), normalizado a [0°, 360°)
octante = floor((angulo + 22,5°) / 45°) mod 8
```

El octante cero es este y los siguientes avanzan hacia noreste, norte, etc.
Los límites pertenecen al sector siguiente en ese orden. La resta usa enteros
con signo de tamaño suficiente. Centros coincidentes de una conexión navegable
producen error de contenido con ambos Lugares; no se inventa dirección ni viento.

El core recibe octantes `u8` e IDs en `CatalogoRumbos`, nunca coordenadas o tiles.
El inicio valida el intervalo `0..8`; no existe un enum `Rumbo8` adicional.
Esta es una excepción provisional explícita a la independencia entre estética
y gameplay: mover un centro puede cambiar el viento relativo. La alcanzabilidad
y el coste base de la ruta siguen dependiendo de Relaciones y reglas, no del
ángulo ni de la distancia visual. Jugador y NPC reciben el mismo catálogo.

## 7. Preparación y estabilización de carga

### 7.1. Evaluación compartida de las configuraciones

Se ejecuta una sola tirada de `CalcularProvisiones`. Se conserva un
`LoteTentativo` con una secuencia aleatoria común de piezas; cada configuración suma sus aportes.
Cada configuración toma el prefijo mínimo que alcanza su propio objetivo;
si necesita más, se amplía esa misma secuencia. No se resortea ni se incorpora
al Grupo ninguna alternativa descartada. El agua de cada configuración se
calcula como total de botas, no como una nueva concesión acumulativa.

La estabilización principal requiere como máximo dos referencias de velocidad:

1. Calcular `T0` sin animales de carga, su lote y `N0`.
2. Si `N0 = 0`, confirmar esa configuración como base.
3. Si `N0 > 0`, calcular `T1` con limitación animal y el lote correspondiente.
4. Recalcular `N1` desde ese peso. La cantidad de animales no vuelve a cambiar
   la velocidad mientras siga siendo positiva: la configuración ya es estable.

`T1 >= T0`; usando el mismo prefijo, el peso y `N1` no disminuyen. No se impone
un máximo arbitrario de repeticiones ni se altera la ruta para cerrar el cálculo.

`N_base` es `N0` o `N1`, según la rama estabilizada. Solo si `N_base > 0` se
evalúa la alternativa `M = N_base − 1`. Usa `T0` si queda sin animales y `T1`
si mantiene alguno, materializando el lote base de su propia referencia. El
número de animales de esta alternativa permanece fijo durante el suplemento.

### 7.2. Suplemento y decisión

Con ese lote base se ejecuta literalmente el ciclo de suplemento de la sección
5.2 del reglamento. Cada iteración sustituye el suplemento total tentativo para
`q`; no añade otra vez suplementos anteriores ni consume azar. Se calculan las
fronteras y la diferencia de demora usando enteros exactos.

Al estabilizarse se conserva una `ConfiguracionCarga` con lote, animales,
referencia temporal, exceso, suplemento y plan de demora por mitad. Un aumento
de demora estrictamente mayor que 24 horas descarta solo la alternativa; la
igualdad permite seguir. El peso creciente hace que el ciclo estabilice o
alcance ese rechazo; un desbordamiento numérico se comunica como fallo técnico.

Cuando existe elección, se publica primero `VIA-APR-05` con las configuraciones
tentativas. Después de responder se confirma una sola, se contabilizan las
24 horas de preparación y su consumo, y se publica la resolución de
`VIA-APR-01` con sus cantidades finales. Esta preparación interna no crea una
viñeta de apertura adicional ni duplica el chequeo o las 24 horas de APR-01.
Si no existe elección, se confirma directamente la configuración base.

Al cerrar APR-01, si la tirada inicial fracasó, se publica `VIA-APR-03`.
La elección de carga no suprime esa advertencia.

### 7.3. Carga pendiente

`PlanCarga` guarda horas de demora asignadas a cada mitad pendiente. El consumo
ordinario no reduce ese plan. Solo los disparadores de la sección 5.3 del
reglamento lo sustituyen, usando el peso y la composición posteriores al cierre.
Una nueva asignación reemplaza, nunca suma, la demora pendiente.

Al llegar a una comprobación, una reserva en cero o una insuficiencia por exceso
puede activar APR-02, pero la marca `busqueda_realizada` impide repetirla en la
misma secuencia entre conexiones. Después de esa búsqueda se recalculan capacidad,
necesidad y carga; si corresponde, se ofrece APR-06.
La muerte de una montura no se interpreta como pérdida de animal de carga.

### 7.4. Algoritmo del lote mínimo de descarte

Cada pieza alimenticia y cada bota con su contenido son objetos indivisibles.
Se incluyen botas vacías. La selección debe retirar al menos el exceso en gramos.
Se ordenan sus objetivos de esta manera:

1. Minimizar el peso retirado por encima del exceso.
2. Entre esos lotes, minimizar la diferencia absoluta entre peso de comida y
   peso de botas con agua; si una reserva no aporta su mitad, la otra completa.
3. Minimizar cantidad de objetos y desempatar por orden canónico de comida e
   ID de bota.

Se usa selección entera acotada, sin un greedy que confunda mínimo con quitar
el objeto más pesado. Se agrupan multiplicidades de comida y botas de igual
volumen y se descomponen en bloques binarios para programación dinámica dispersa.
Para cada peso alimenticio alcanzable se conserva la mejor composición; para
cada peso de agua alcanzable, el mejor conjunto de botas. Una composición de
igual peso nunca se descarta si mejora los desempates.

Solo se necesitan sumas hasta `exceso + peso_maximo_de_objeto − 1`: un lote de
peso global mínimo no puede exceder ese límite, pues se podría devolver alguno
de sus objetos y seguir cubriendo el exceso. Se combinan ambas tablas ordenadas
para hallar primero el mínimo peso suficiente y después el mejor reparto.
El algoritmo es seudopolinómico en gramos; se mide con el contenido oficial y
no se sustituye por una aproximación silenciosa si un mundo modificado es enorme.
Un fallo de recursos es técnico, nunca una cancelación narrativa inventada.

Se guarda el lote exacto en la decisión. Al aceptarla se revalida, se quita una
sola vez y se comprueba `x = 0`. La simulación no busca de nuevo provisiones en
esa secuencia aunque el descarte deje un recurso en cero.

## 8. Contrato de acciones particulares y azar

El catálogo nuevo contiene las cinco acciones de la sección 6 del reglamento.
Sus dificultades y tablas de incidentes pertenecen a las tarjetas.

`Cabalgar` es una sola variante de `AccionParticular`, usada por VIA-DES-CAB y
VIA-DES-CAM con el mismo evaluador de capacidad y resolución. Se conservan los
módulos `tarjetas/cabalgar.rs` y `tarjetas/camello.rs`, sus claves, registros y
redactores independientes. La especie pertenece al contexto de la escena y a
sus consecuencias, no crea otra habilidad ni otra acción particular.
Compartir prueba no implica una única tarjeta parametrizada ni impide añadir
reglas narrativas o incidentes propios a cada medio.

```rust
pub fn resolver_chequeo(
    mundo: &Mundo, responsable: IdActor, accion: AccionParticular,
    dificultad: i16, azar: &mut dyn FuenteAzar,
) -> Result<ResultadoChequeo, ErrorMotor>;
pub struct ResultadoChequeo {
    pub responsable: IdActor,
    pub accion: AccionParticular,
    pub dificultad: i16,
    pub natural: u8,
    pub modificadores: Vec<ModificadorAplicado>,
    pub ajustado: i16,
    pub exito: bool,
}
```

`evaluar_capacidad` consulta exclusivamente los modificadores exhaustivos de la
acción. La tarjeta usa esa función para elegir al más apto y sortea los empates;
`resolver_chequeo` recibe al responsable, obtiene un único d20 y limita el total
a 20. No elige contexto, responsable ni dificultad y no devuelve duración,
incidentes, texto o cambios del Mundo. Estos pertenecen a la tarjeta.

El orden canónico de azar es selección de responsable, chequeo origen, chequeo
destino, incidente si corresponde y luego sorteos exigidos por esa consecuencia.
En navegación, el viento aleatorio se fija antes de los chequeos. En forrajeo se
recorren Actores por ID y, tras cada éxito, se sortea alimento y después agua
solo si se está buscando ese recurso. La materialización y el consumo tienen
sus propias llamadas, registradas con su causa para reproducirlas.

La recuperación de montura y la selección de incidente son sorteos simples;
no se registran como acciones particulares ni reciben modificadores.
Cada índice devuelto por `FuenteAzar` se valida. Consultar una vista, paginar,
redimensionar o abrir detalles no consume azar.

## 9. Protocolo reanudable y estados

```rust
pub fn iniciar(
    mundo: &mut Mundo, solicitud: SolicitudAccion,
    contexto: &ContextoAccion, azar: &mut dyn FuenteAzar,
) -> Result<InicioAccion, ErrorMotor>;
pub fn avanzar(
    mundo: &mut Mundo, ejecucion: &mut EjecucionAccion,
    paso: IdPaso,
    contexto: &ContextoAccion, azar: &mut dyn FuenteAzar,
) -> Result<PasoAccion, ErrorMotor>;
pub fn responder(
    mundo: &mut Mundo, ejecucion: &mut EjecucionAccion,
    respuesta: RespuestaDecision, contexto: &ContextoAccion,
    azar: &mut dyn FuenteAzar,
) -> Result<PasoAccion, ErrorMotor>;

pub enum PasoAccion {
    Escena(RegistroEscena),
    Decision(SolicitudDecision),
    Finalizada(FinalizacionAccion),
}
pub struct ContextoAccion<'a> {
    pub inicio: InstanteAccion,
    pub rumbos: &'a CatalogoRumbos,
}
pub struct InicioAccion {
    pub ejecucion: EjecucionAccion,
    pub paso: PasoAccion,
}
pub struct FinalizacionAccion {
    pub tiempo: TiempoAccion,
    pub cierre: CierreViaje,
    pub lugar: IdLugar,
}
```

`InicioAccion` entrega ejecución opaca y primer paso. Cada paso entrega una
única unidad semántica visible. `Partida` retiene la ejecución tanto en escenas
automáticas como en decisiones; no ejecuta el resto del viaje por adelantado.
`RegistroEscena` y `SolicitudDecision` contienen un `IdPaso` opaco y monotónico
local a la ejecución. `RespuestaDecision { paso, opcion }` combina ese ID con
una variante de `OpcionViaje`; se valida contra las opciones del paso vigente.
La fachada traduce sus IDs opacos de opción y decisión a este par. El motor compara el paso recibido antes de gastar azar o modificar
estado. Solo puede llamarse `avanzar` para confirmar una escena vigente; una
decisión exige `responder` con IDs válidos. El primer paso procede de `iniciar`.

`CierreViaje` distingue `Llegada`, `Cancelado`, `SinRecursos`,
`InterrumpidoPorSeparacion` y `Naufragio`; el registro conserva la tarjeta que
lo causó. La finalización se entrega una sola vez. Reanudar una ejecución
finalizada produce rechazo, sin avanzar otra vez el calendario.

`ContextoAccion::inicio` es el instante fijo capturado al iniciar toda la acción,
no el de la escena actual. Las reanudaciones reutilizan ese instante y el
mismo catálogo inmutable; la ejecución valida su identidad antes de continuar.
Las proyecciones se calculan siempre desde ese inicio y el total acumulado.

La máquina contiene estas fases, con los datos propios de cada espera:

| Fase | Salida y continuación |
|---|---|
| Planificar | Solo no adyacentes: decisión RUT-02 con propuestas, improvisar y cancelar; contrato 08. |
| Preparar | Una tirada, configuración tentativa; decisión APR-05 si existe. |
| ConfirmarPreparacion | Escena APR-01; después advertencia APR-03 si corresponde. |
| IniciarConexion | Fija participantes, medio, responsable y dos chequeos. |
| ResolverConexion | Tarjeta propia del medio; decisión de separación si corresponde. |
| CerrarConexion | Confirma consecuencias, consumo y registro de desplazamiento. |
| EntreConexiones | Llegada primero; en otro caso reservas y comprobación de carga. |
| BuscarRecursos | APR-02 una vez; después terminación, APR-03 o carga. |
| ResolverSobrecarga | Decisión APR-06; continuar o terminar. |
| Llegar | Escena LLE-01 y, al confirmarla, finalización. |
| Terminar | Finalización sin escena de llegada ni consecuencias añadidas. |

Las fases internas sin salida visible avanzan dentro de una llamada hasta el
siguiente paso; no aparecen como viñetas vacías. La secuencia entre conexiones
se identifica con un ordinal de conexión y reinicia su marca de búsqueda solo
cuando termina otro desplazamiento. No hay límite de pasos ni presupuesto
narrativo. La ruta finita, las decisiones que avanzan y la guardia de APR-02
impiden un bucle sin progreso.

Las decisiones de ruta aplican sus 24 horas cualquiera sea la opción, incluida
cancelar, antes de entregar el siguiente paso o finalizar. Una tarjeta automática
resuelve, confirma resultados y tiempo, y después se presenta.

### 9.1. Separación dentro de un desplazamiento

Se conservan las dos tiradas y el responsable originales. Si aparece separación,
la escena pasa a una decisión con el relato de lo ya ocurrido y las opciones
de su tarjeta. Se conserva un `DesplazamientoPendiente` tentativo; todavía no se
confirman consumo, traslado ni separación finales de esa escena. La respuesta
continúa ese mismo intento, prepara su cierre y confirma una sola vez el lote;
no ejecuta otra tarjeta de desplazamiento ni repite chequeos. Produce el cierre
narrativo de la elección sin inventar otro incidente. La decisión y su cierre
son dos momentos de esa misma escena; paginación y ventanas no cuentan escenas.

La composición aplicable al consumo sigue siendo la congelada al iniciar.
La siguiente escena usa la composición posterior. Tiempo y consecuencias se
confirman una sola vez.

Elegir buscar al compañero conserva la duración calculada de **toda la
conexión**, incluidas ambas mitades y las demoras aplicables, aunque la separación
ocurra en origen. Se aplica el consumo correspondiente a esa duración completa
y a todos los participantes iniciales. El Grupo termina en el Lugar del
incidente, sin confirmar llegada al otro extremo ni añadir tiempo de búsqueda.
La ubicación de cierre no se usa para recortar retroactivamente la duración.

Se conservan ambos resultados originales y los conocimientos que sus umbrales
concedan, sin anularlos por cancelar ni volver a tirar. Con las reglas actuales
de Andar, la condición de separación impide que cualquiera de los dos chequeos
conceda Conoce; conservar resultados no concede conocimiento sin su umbral.
El cierre narrativo comunica el tiempo completo contabilizado y el Lugar real
de terminación sin afirmar que el Grupo alcanzó el destino de la conexión.

### 9.2. Orden entre conexiones

Después del desplazamiento: si terminó por una consecuencia, finalizar; si
alcanzó el destino, LLE-01; si aún queda ruta, comprobar reservas y pérdidas.
APR-02 no puede ejecutarse al llegar. Después de forrajear se consume su tiempo,
se comprueba la ración mínima del día y se compara la reserva con la necesidad
del recorrido que todavía falta. No se vuelve a incluir en esa segunda cuenta
las seis horas ya consumidas.

Una advertencia APR-03 conserva su continuación explícita: primera salida o
comprobación de carga posterior a forrajeo. Cancelar conserva tiempo y cambios
confirmados. Continuar no evita una comprobación de carga pendiente.

## 10. Consecuencias persistentes y atomicidad

Se amplía `OperacionMundo` con operaciones acotadas:

```rust
SepararCompanero { actor: IdActor, lugar: IdLugar }
RegistrarConocimiento { actor: IdActor, lugar: IdLugar }
```

La primera valida pertenencia actual y prohíbe apartar al Actor jugador; elimina
al compañero y fija su ubicación. La segunda crea `Conoce` usando el almacén y
los índices existentes; si ya está, es idempotente. No se mantiene otro mapa de
conocimientos ni se añade `Abandono`.

`aplicar_lote` prepara también Grupo, Relaciones y todos los índices afectados,
además de Actores. Valida el lote completo sobre un estado candidato y publica
todo junto. Un fallo no deja separación sin traslado o Relación sin índice.

El orden del lote preserva conocimiento adquirido por los participantes antes
del incidente, aplica estados en orden de mitad, separa a quienes corresponda
y traslada al dirigente con los compañeros restantes. Los náufragos separados
pueden tener costas diferentes. La tarjeta omite un traslado a la ubicación ya ocupada. La operación primitiva
conserva su rechazo de destino actual; solicitar `Viajar` hacia él también
sigue rechazado.

La duración, las reservas, el lote y el registro de escena se preparan antes de
publicar cambios. Solo tras validar los cálculos y el lote se sustituye el
estado transitorio. Un fallo de azar invalida ese paso sin confirmar cambios;
no se promete rebobinar una fuente de azar que falló.

El núcleo confirma como máximo una escena visible por paso. Su registro retiene
participantes de entrada y resultados confirmados; ninguna capa consulta el
Grupo ya modificado para reconstruir quién viajó en esa escena.

## 11. Registros semánticos, redacción y privacidad

`RegistroEscena` contiene ID monotónico, `ClaveTarjeta`, participantes iniciales
y composición resultante, Lugares,
chequeos, incidente, hechos cronológicos por mitad, operaciones confirmadas,
tiempo, consumo, reservas finales, motivo de cierre y visibilidad de datos.
`ClaveTarjeta` distingue todas las tarjetas del reglamento, incluidas las cuatro
formas de desplazamiento. El núcleo no produce prosa localizada.

Los registros específicos son enums con datos propios:
las variantes de `ContenidoEscena` (`Plan`, `Preparacion`, `CargaInicial`,
`Desplazamiento`, `Forrajeo`, `Incompleto`, `Sobrecarga` y `Llegada`).
Un `Desplazamiento` conserva las dos resoluciones, sus descubrimientos,
mitad del incidente, consecuencias y desglose temporal; un registro de carga
conserva configuraciones y lote ofrecido. No se infiere gameplay desde texto.

`desert-tiger-app::narracion_viaje` resuelve nombres y redacta por tarjeta.
La jerarquía narrativa se rige por la [sección 7 del reglamento](./borradores/05-viajar-redisenio.md#presentacion).
`narracion_viaje/resumen.rs` redacta el relato inicial a partir del mismo
`RegistroEscena`; el módulo principal conserva el desarrollo completo en
`TextoEscena::detalle`. No se acorta una cadena por cantidad de caracteres ni
se omiten acontecimientos del motor para cumplir la brevedad. La selección del
suceso principal usa hechos confirmados; ante una separación pendiente no
afirma la llegada antes de recibir la respuesta. No consume azar para redactar.

`VistaFragmento` y `VistaDecision` entregan `texto: String` para el relato y
`detalle: Vec<String>` para el desarrollo consultable. La sesión transfiere ambos
campos incluso al esperar una decisión; los detalles propios de opciones siguen
siendo independientes y opcionales. Los consumidores no reconstruyen el
desglose desde el texto narrativo ni interpretan prosa para ejecutar reglas.

La aplicación redacta solo los campos permitidos y no entrega registros crudos
al frontend. La vista pública contiene texto, opciones y progreso:

- sin plan visible no expone ruta futura, siguiente Lugar, duración prevista,
  necesidad base ni suficiencia secreta de reservas;
- durante un desplazamiento sí muestra los Lugares de esa escena ya vivida;
- APR-05 muestra solo demora añadida si la duración base es oculta;
- `[D]` recibe la misma proyección y no puede recuperar campos privados;
- las unidades alimenticias nunca salen a la vista; se muestran piezas y agua;
- los pronósticos antiguos se retiran del selector de destino para no anticipar
  la información de RUT-02 antes de decidir el viaje.

La cámara y el marcador siguen el Lugar del registro presentado. Los retratos
usan sus participantes congelados y se actualizan al confirmar el cierre. El
frontend recibe texto, opciones y progreso ya proyectados; conserva su arte,
animación, controles y paginado. La lectura principal conserva la ilustración;
Detalles la sustituye por el desglose paginado, sin repetir el resumen. La
ventana reserva alto suficiente para texto e ilustración dentro del área
disponible. Enter confirma un paso solo al cerrar su última página narrativa;
en Detalles solo recorre sus páginas y se detiene en la última. Las opciones
no responden mientras el desglose está abierto; cerrarlo restaura la página
desde donde se consultó. Abrir detalles o cambiar tamaño no reanuda la ejecución.

## 12. Contenido y validaciones

Se conserva un RON por Actor o Lugar. No se crean archivos separados de tarjetas
ni una DSL. Las Características del manual son etiquetas legibles; sus claves
RON siguen la convención existente, sensible a mayúsculas y sin tildes:

| Manual | Clave RON |
|---|---|
| Viajero / Explorador | `viajero` / `explorador` |
| Vagabundo / Jinete / Navegante | `vagabundo` / `jinete` / `navegante` |
| Administrador / Expedicionario / Herbalista | `administrador` / `expedicionario` / `herbalista` |
| Pequeño / Vasto | `pequeno` / `vasto` |
| Acuático / Costero / Desierto | `acuatico` / `costero` / `desierto` |
| Montañoso / Pantanoso / Boscoso | `montanoso` / `pantanoso` / `boscoso` |
| Esguince / Cansado | `esguince` / `cansado` |

No se reinterpretan automáticamente capacidades anteriores como `camellero`,
`conoceDesiertos` o `conCamellos`; sus reglas viejas dejan de participar en este
viaje. La adecuación del contenido oficial debe expresar explícitamente las
capacidades deseadas, sin conceder bonos nuevos por similitud de nombres.

El cargador rechaza `pequeno` y `vasto` simultáneos. Si existe navegación posible,
exige al menos un Lugar `costero` y un rumbo definido para cada conexión
navegable en ambos sentidos. Los errores incluyen archivo, clave de Lugar y
campo. Las Características desconocidas siguen siendo válidas para otras reglas.
El núcleo valida también contextos de prueba o creados sin el cargador.

El resto fraccionario comienza en cero y no requiere ampliar `world.ron` ni
la API Lua. Esta continúa leyendo la fecha mundial entera confirmada. Si en el
futuro necesita hora del día, requerirá su propio contrato.

## 13. Ejecución de NPC y sesión

La fase NPC obtiene al comenzar una lista de políticas ordenada por ID y la
recorre una vez. Cero políticas elegibles produce un resumen vacío válido.
Antes de cada evaluación revalida existencia, vida y exclusión del jugador y
sus compañeros. Un Actor sin política activa no actúa automáticamente.

Cada política mantiene su contrato Lua actual: consulta y solicita una acción.
La ejecución usa este mismo motor con un viajero y el mismo catálogo de rumbos.
La aplicación confirma automáticamente las escenas y sortea solo las decisiones
entre opciones válidas. No simula nuevas políticas durante la misma acción ni
limita la cantidad de escenas del viaje.

Cada NPC produce un resumen y conserva su resultado de cierre. Los resúmenes se
acumulan; un rechazo o error Lua recuperable no impide evaluar al siguiente.
Los errores estructurales del motor siguen siendo fatales. Ni duración ni resto
de un NPC avanzan el reloj mundial. La fase completa usa la fecha posterior al
jugador y no acumula fechas proyectadas de distintos NPC.

`Partida` confirma una sola finalización del jugador después de su última escena
y después ejecuta una sola fase NPC. IDs antiguos, doble Enter y respuestas a
otra decisión se rechazan sin repetir operaciones, consumo o calendario.

## 14. Errores y límites

| Categoría | Tratamiento |
|---|---|
| Actor o destino inexistente, actor muerto, destino actual o desconectado | Rechazo de solicitud antes de gastar tiempo o azar de escenas. |
| Paso o decisión no vigente | Rechazo recuperable; ejecución conservada. |
| Tamaño contradictorio, costa ausente o centros coincidentes navegables | Diagnóstico de contenido; no se crea una Partida parcial. |
| Catálogo incompleto, Grupo incoherente o operación inválida | Fallo de integración/motor; no se confirma el lote. |
| Fuente de azar fallida o índice fuera de rango | Fallo explícito del motor con contexto del sorteo. |
| Desbordamiento de tiempo, peso, cantidad, ordinal o calendario | Fallo técnico contextualizado; nunca una consecuencia narrativa. |
| Fracaso de chequeo, recursos insuficientes, naufragio o cancelación reglada | Resultado válido de gameplay, no error técnico. |

No se añaden `panic`, `unwrap` ni recuperación que invente barcos, comida,
costas o estados fuera de las reglas. Los recursos automáticos que sí concede
el reglamento se incorporan exactamente en sus tarjetas.

## 15. Pruebas y aceptación

Las pruebas verifican hechos semánticos y resultados, sin comparar todos los
textos completos. La redacción exige aserciones específicas sobre nombres,
causas, cantidades y datos ocultos. El azar se controla por secuencias
etiquetadas; repetir una semilla reproduce decisiones y consumos.

| Contrato | Casos mínimos |
|---|---|
| Tiempo | 20+6 horas; frontera exacta de 24; 168 horas y 2 kg de exceso dan 6,72 horas; cero por viento; cruce de mes y año; resto 2+22 horas entre acciones; proyección igual a confirmación; desbordamientos. |
| Consumo | Cada tipo de comida; desperdicio sin arrastre; agua con botas parciales/vacías; ración impagable en mitad/final; saldo cero sin carencia; composición congelada; preparación y búsqueda consumen una sola vez. |
| Rutas | Menor duración con más Lugares; empate reproducible; una ruta; destino desconectado; alternativa fuera del intervalo e idéntica; sin repetición de Lugares; consultar destinos sin azar. |
| Rumbo | Ocho octantes; vector inverso; eje Y; fronteras de sector; centros coincidentes; mismo catálogo para jugador y NPC. |
| Preparación | Una tirada; lotes compartidos por prefijo; T0/T1 estables; cero/uno/varios animales; suplemento nulo; incremento exactamente 24 y mayor; rechazar alternativa no altera lote base. |
| Carga | Plan que no disminuye al comer; pérdida de Actor frente a montura; sustitución solo pendiente; descarte mínimo por fuerza bruta en casos pequeños; equilibrio y desempates; botas vacías; cero de recurso sin segunda búsqueda. |
| Chequeos | Naturales 1 y 20; tope ajustado; suma exhaustiva de bonos; empate cero; responsable fijo en ambas mitades; recuperación sin bono; caballo y camello usan Cabalgar con idéntica aptitud sin fusionar tarjetas. |
| Desplazamientos | Cada fila de las cuatro tablas; condiciones exactas de incidente; origen/destino y empate; estados previos y adquiridos; natural frente a ajustado para Conoce; montura perdida en cada mitad; coste sin cambios retroactivos. |
| Persistencia | Conoce idempotente e índices coherentes; separación más traslado atómicos; costas distintas; ninguna Relación nueva; fallo no deja cambios parciales. |
| Secuencia | Preparación con/sin APR-05; APR-03 inicial y posterior; una APR-02 por intervalo; llegada antes de búsqueda; toda cancelación conserva tiempo; sin límite de escenas. |
| Cancelación por separación | Buscar en origen conserva tiempo y consumo completos de la conexión; Grupo y compañero quedan en el Lugar del incidente; se conservan ambos chequeos y sus conocimientos aplicables; sin llegada, segunda tirada ni tiempo añadido por la búsqueda. |
| Presentación | Sin ruta o necesidad oculta en texto, vistas o detalles; mismos tiempos y reservas que simulación; relatos de una a tres oraciones; incidentes, ubicación y consecuencias esenciales visibles; desglose conservado en escenas y decisiones; retratos del momento correcto; paginado sin ejecutar reglas ni responder opciones. |
| NPC | Cero, uno y varios; compañero recién separado; política ausente; error recuperable; decisiones automáticas; fecha y resto mundial intactos. |

Una prueba de integración recorre archivos oficiales, preparación, varias
conexiones, consumo, decisión, cierre, calendario y fase NPC. Las pruebas de
Ratatui usan `TestBackend` para texto extenso, redimensionado y decisiones.
La aceptación manual agrupa un viaje terrestre, uno marítimo y una interrupción;
no detiene el desarrollo para probar cada fila de incidente.

Antes de considerar implementado el contrato deben pasar los controles de la
[metodología](../../metodologia-desarrollo.md#7-controles-automáticos), la carga
de todo el contenido oficial y el build de producción. Cada regla importante
debe quedar vinculada con una prueba; los planes registrarán la ejecución de
esos controles, sin duplicar el diseño.

## 16. Compatibilidad y documentos afectados

| Base existente | Cambio de este contrato |
|---|---|
| Arquitectura general | Mantiene las capas; reemplaza el proyecto anterior por tarjetas semánticas y declara el rumbo visual provisional. |
| Modelo y operaciones del Mundo | Separar compañeros y registrar Conoce con atomicidad e índices. |
| Contenido | Valida tamaño, costas, rumbo y claves; mantiene formatos RON y Lua. |
| Motor y sesión | Ejecución retenida durante escenas automáticas y protocolo por paso. |
| Calendario | Conserva Fecha/Duracion; añade puente de fracción sin extender Lua. |
| Fase NPC | Cero o más políticas, misma máquina, sin avance mundial. |
| Terminal | Recibe nueva proyección y texto; conserva layout, recursos y controles. |
| Fichas particulares anteriores | Se retiran del camino de Viajar los verbos y bandas incompatibles; las cinco pruebas nuevas usan el contrato de la sección 8. |
| Planes anteriores | Conservan su historial; no autorizan nuevas tareas de este refactor. |

No hay compatibilidad de partidas guardadas que mantener. No se añaden aliases
silenciosos de Características ni una capa para ejecutar el diseño viejo.
Las cabeceras de los componentes señalan sus apartados sustituidos y enlazan
a este contrato. Los diseños viejos permanecen como antecedentes, no como
alternativas de ejecución.

La sustitución integrada retiró los módulos anteriores de movimiento,
supervivencia, navegación, expedición, planificación y proyecto. Los tipos
neutrales de claves visuales se conservan para reutilizar el arte y las vistas.
No existe selector de motores. La base anterior sigue recuperable en Git.

## 17. Cierre del diseño

Las aclaraciones confirmadas el 2026-10-24 están incorporadas en la sección 9.1:
cancelar por separación conserva el tiempo de toda la conexión y ambos resultados
originales, incluidos los conocimientos que correspondan a sus umbrales.

No quedan decisiones de juego abiertas registradas. La aprobación para preparar
planes incluye la unificación de Cabalgar, manteniendo dos tarjetas, confirmada
el 2026-10-24. El desarrollo fue solicitado y está integrado. El
[informe de implementación y verificación](./verificacion-viajar-reglamento.md)
registra pruebas, alcance manual, ajustes técnicos y la revisión estética
conjunta pendiente; no equivale a una aprobación de jugabilidad del usuario.


## Selección de destinos por conocimiento — 2026-10-29

`acciones::destinos_seleccionables_jugador(&Mundo)` ofrece la intersección entre
los destinos alcanzables y la unión de los vecinos `Conectado` del Lugar actual
con los Lugares `Conoce` del jugador y sus compañeros actuales. El jugador se
incluye explícitamente, aunque no figure como integrante de `Grupo`. Los actores
externos al grupo no aportan conocimiento. Se elimina el Lugar actual y se
mantiene orden canónico sin duplicados. Regiones y lugares aislados no se ofrecen
por el solo hecho de conocerlos.

La consulta se usa al proyectar las acciones disponibles de la sesión. Conserva
los rechazos de `destinos_disponibles`, que sigue siendo la consulta de
alcanzabilidad utilizada por el motor. No filtra el grafo ni cambia las rutas
rápida, larga o conocida: un camino a un destino ofrecido puede atravesar lugares
desconocidos. Tampoco restringe las solicitudes de los NPCs. Se verifica el
aporte del jugador y compañeros, vecinos desconocidos, separación de compañeros,
cambio de ubicación y rutas que atraviesan lugares no ofrecidos como destino.

### Lugares ocultos a la selección — 2026-11-02

Un Lugar con `oculto` no se ofrece al jugador como destino, aunque sea conocido
o vecino. La vecindad seleccionable atraviesa cadenas de Lugares ocultos hasta
los primeros Lugares visibles; no continúa a través de estos últimos. Así la
entrada de un paso ofrece su salida aunque entre ambas exista un Lugar oculto.
La consulta evita ciclos y conserva el orden canónico de resultados.

La búsqueda de rutas y la ejecución conservan los Lugares ocultos con todos sus
tiempos, características, descripciones y escenas. La solicitud explícita de
Viajar del jugador hacia un Lugar oculto se rechaza con `DestinoInvalido`.
La marca no restringe los destinos de los NPC ni su tránsito. `destinos_disponibles`
sigue siendo una consulta de alcanzabilidad sin este filtro de presentación.

Cuando el catálogo tiene un nombre vacío, el narrador usa la expresión genérica
«un tramo del camino» para evitar frases incompletas en resúmenes, detalles y
progreso; no muestra claves internas como nombres. El mapa no necesita una
regla adicional para un Lugar sin símbolo y con rótulo vacío.

## Descripción del paisaje durante el viaje — 2026-11-02

La descripción visual opcional de cada Lugar se escribe en su RON, según
[Contenido, descripción e ilustración](../componentes/03-contenido.md#descripción-e-ilustración-de-lugar--2026-11-02).
Es un texto breve de autor en segunda persona que presenta lo que ve el jugador.

El narrador antepone la descripción del origen al resumen de cada viñeta de
Desplazamiento: primero sitúa al jugador en el paisaje y luego relata la partida,
el recorrido y sus consecuencias. En la viñeta final de Llegada, antepone la
descripción del lugar alcanzado. El detalle expandido conserva el mismo contexto.
No se crean viñetas adicionales.

Esta composición no consulta visitas anteriores, relaciones Conoce ni cambios de
ubicación para decidir si un paisaje ya fue descrito. No mantiene contadores,
conjuntos o memoria por viaje. Si una escena vuelve a usar un lugar, puede volver
a mostrar su descripción. La referencia al comienzo del camino es una indicación
narrativa, sin una mecánica de descubrimiento o de prevención de repeticiones.

Sin descripción se conserva el relato existente. El contenido se carga al inicio
de la partida y el narrador consulta el catálogo validado. La ilustración se
compone sobre el mapa según [Arte de lugares](../componentes/21-arte-lugares.md),
usando el mismo contexto narrativo; no se inserta dentro de la ventana de viñeta.
Las pruebas recorren viajes de ida y vuelta y comprueban la presencia del paisaje
del origen, los lugares intermedios y el destino, junto con el detalle expandido.
