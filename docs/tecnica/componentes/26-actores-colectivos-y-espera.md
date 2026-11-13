# Actores colectivos, incorporación y espera de órdenes

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.0 |
| Fecha | 2026-11-12 |
| Alcance | Colectivos físicos, pertenencia, modo reclutado y espera continua cancelable |

## 1. Propósito y límite del incremento

El usuario propone un elemento **Actor colectivo**: una compañía, ejército,
caravana, multitud o bandada con identidad propia que puede contener Actores
concretos y tener un líder opcional. Representar una compañía no exige crear
un Actor por cada soldado. El Grupo del jugador puede quedar incorporado y
desplazarse con ella, conservando sus compañeros y retratos.

Este diseño comprende:

- Contenido RON, identidad, ubicación, miembros y líder opcional del colectivo.
- Entrada y salida de Actores y del Grupo mediante operaciones validadas.
- Movimiento narrativo del colectivo, con sus miembros, mediante eventos fechados.
- Restricción de acciones individuales mientras el Grupo está incorporado.
- Marco visual del mapa y opción contextual Esperar órdenes en Acampar.
- Espera sobre el mapa con avance automático del tiempo, cancelación y parada
  ante una acción del colectivo; cada intervalo tiene procesamiento acotado.

Se dejan para después las acciones generales de colectivos, IA colectiva,
reclutamiento voluntario desde menús, deserción, prisioneros con reglas propias,
colectivos anidados, mando jugable de ejércitos y colectivos institucionales sin
una ubicación física única. No se añaden inventarios, tropas numéricas ni barcos
como elementos. La compañía puede representar personas anónimas no instanciadas.

La incorporación física se distingue de servir a alguien o pertenecer a una
institución. Sirviente o PrisioneroDe no ingresan automáticamente a nadie en un
colectivo. Salir de una compañía no borra esas otras relaciones ni concede por
sí solo la libertad de un personaje esclavizado.

## 2. Base actual y dependencia de la agenda

El [contrato 25](./25-conceptos-relaciones-personales.md) implementa Actores,
Lugares y Conceptos en el grafo. El [23](./23-eventos-narrativos.md) implementa
eventos por puntos. El [24](./24-agenda-eventos-mundo.md) define la agenda común; este incremento
implementa su primer subconjunto de citas EnFecha sobre el coordinador existente.

Este incremento implementa un primer corte de esa agenda: activaciones PorPuntos
y EnFecha, orden temporal, registro común de ocurrencias y observación local o
silenciosa. No crea una agenda exclusiva de compañías. La integración de eventos
incrustados, `this`, EnFaseNpc, repetibles y AccionNpc puede desarrollarse después;
las políticas NPC legadas siguen ejecutándose una vez por cierre hasta su
migración, sin ejecutarse también como eventos normalizados.

Se conserva la decisión del usuario y del contrato 24: las citas atravesadas
se resuelven al cerrar una acción, contra el estado resultante. No se reconstruye
la posición histórica del Grupo durante fechas intermedias. Los traslados de
compañías son consecuencias históricas escritas, no ejecuciones encubiertas de
Viajar ni una segunda fuente de duración.

## 3. Modelo e invariantes de ubicación

### 3.1. Tipos y fuente de verdad

El núcleo incorpora `IdColectivo`, distinto de IdActor, IdLugar e IdConcepto,
y `ActorColectivo`, con ID, Lugar actual, características y líder opcional.
Su nombre y descripción pertenecen al catálogo de presentación.
`DatosInicialesMundo` incorpora los colectivos iniciales; sus constructores
actuales conservan el tratamiento de Conceptos del contrato 25.

`UbicacionActor` añade `EnColectivo(IdColectivo)`. Esta variante es la única
fuente de pertenencia física. No se guarda además un Lugar duplicado en cada
miembro ni una segunda lista mutable de miembros dentro del RON del colectivo.
`Mundo` deriva y mantiene un índice de Actores por colectivo.

Un colectivo tiene un Lugar directo, local y no regional. En esta versión no
está dentro de otro colectivo ni fuera del escenario. Puede existir sin miembros
concretos: representa también a personas o animales no individualizados y no
desaparece automáticamente cuando el último Actor sale.

### 3.2. Ubicación efectiva

`Mundo::ubicacion_actor(actor)` obtiene el Lugar efectivo:

- EnLugar devuelve ese Lugar.
- EnColectivo devuelve el Lugar del colectivo.
- FueraDelEscenario devuelve None.

`ubicacion_grupo`, presencia, visión, cámara, sonido ambiental, fichas, Lua y
acciones utilizan esa consulta. `Actor::lugar_directo()` sustituye al antiguo `Actor::lugar()` y distingue
explícitamente ese acceso directo de la consulta efectiva. Los consumidores de
presencia, incluido Acampar, utilizan Mundo para no confundir a un incorporado
con un Actor ausente.

Mover el colectivo cambia un solo Lugar y con ello la posición efectiva de
todos sus miembros. Las consultas de pertenencia y ubicación no recorren el
grafo; usan IDs e índices. La preparación atómica puede reconstruir índices
cuando cambia la composición, siguiendo el contrato de operaciones existente.

### 3.3. Grupo y liderazgo

Jugador y compañeros conservan la misma ubicación directa: el mismo EnLugar
o el mismo EnColectivo. Estar en colectivos distintos situados en un mismo Lugar
no satisface esa invariante. El Grupo sigue representando solo a los compañeros;
no se agregan a él el capitán ni los demás integrantes de la compañía.

Si existe líder, debe ser un Actor vivo incorporado al mismo colectivo. Por eso
permanece junto a sus miembros sin sincronizar otra copia de su posición. No se
elige sucesor automáticamente. La muerte o salida del líder deja el colectivo
sin líder en el mismo lote que confirma ese cambio; no lo disuelve ni libera a
todos sus miembros. Una resurrección no restablece el mando automáticamente.

La ausencia de líder es válida y no impide que un evento mueva al colectivo o
libere miembros. La espera nunca depende de que exista una política Lua activa
del líder. Ser líder tampoco incorpora acciones de mando no diseñadas todavía;
este corte controla el movimiento colectivo exclusivamente por eventos.

## 4. Autoría y fichas

`world.ron` admite `colectivos: [...]`, vacío al omitirse. Cada clave carga
`collectives/<clave>.ron`. Se resuelven primero todas las claves de Actores,
Lugares, Conceptos y colectivos, y después sus referencias.

```ron
(
    clave: "compania_dahran",
    nombre: "Compañía de Dahran",
    descripcion: "La gente y las embarcaciones que siguen al capitán Dahran.",
    ubicacion_inicial: EnLugar("zarabad"),
    caracteristicas: [],
    lider: Some("dahran"),
)
```

`caracteristicas` y `lider` son opcionales, con lista vacía y None como valores
por defecto. No hay `miembros`, `politica`, `eventos` ni ubicación anidada en este
formato. Los Actores se incorporan declarando su propia ubicación:

```ron
ubicacion_inicial: EnColectivo("compania_dahran"),
```

Ese sería el caso de Dahran en el futuro contenido de la compañía. El príncipe
puede comenzar fuera y entrar por un evento posterior. El contenido base conserva
su inicio libre. La [demo aislada](../../../content/demos/colectivos/README.md)
ejemplifica incorporación, traslado y salida.

Se añade `ExtremoRelacion::Colectivo(IdColectivo)` para identificar fichas y
referencias. La pertenencia y el liderazgo se proyectan como vínculos derivados,
con variantes propias de `VinculoFicha`, siguiendo el precedente de Hermandad:
desde un Actor, «Incorporado a»; desde el colectivo, «Integrantes» y «Liderado por».
No se inventan IdRelacion persistidos ni se permite declarar esas mismas
pertenencias una segunda vez en `relaciones`.

La ficha colectiva muestra nombre, descripción, características, Lugar efectivo,
líder e integrantes concretos. Los anónimos no se enumeran. Enter permite seguir
referencias; no incorpora, libera ni cambia líderes. Los tipos registrados del
contrato 25 conservan sus extremos actuales: admitir un extremo colectivo en una
nueva relación necesitará definir ese tipo expresamente.

## 5. Operaciones atómicas

Las operaciones se preparan sobre estado virtual, validan el lote y confirman
ubicaciones, liderazgo e índices juntos. Una referencia inválida en la última
operación no deja confirmado medio traslado o parte del Grupo.

| Operación del núcleo | Regla |
|---|---|
| IngresarActorEnColectivo { actor, colectivo } | Actor independiente y colectivo existentes, en el mismo Lugar efectivo. Rechaza al jugador y a sus compañeros; usan la operación del Grupo. |
| IngresarGrupoEnColectivo { colectivo } | Incorpora al jugador y todos sus compañeros juntos; todos deben estar libres y en el Lugar del colectivo, o ya incorporados a ese mismo colectivo. |
| SalirActorDeColectivo { actor, colectivo } | Exige la pertenencia esperada y un Actor independiente. Queda EnLugar del colectivo en ese instante. |
| SalirGrupoDeColectivo { colectivo } | Exige el colectivo esperado y saca a jugador y compañeros juntos, al Lugar actual del colectivo. |
| TrasladarColectivo { colectivo, destino } | Cambia su Lugar y registra el traslado efectivo de sus miembros. Destino actual es una operación satisfecha sin otro cambio. No calcula ruta ni duración. |
| EstablecerLiderColectivo { colectivo, lider } | None quita el mando; Some exige un Actor vivo incorporado al colectivo. |

Ingresar en el mismo colectivo ya presente es idempotente; pertenecer a otro
exige salir primero. Salir conserva la precondición del colectivo esperado para
que un evento antiguo no extraiga al Grupo de una compañía distinta.

`Trasladar(actor)` y Viajar rechazan Actores contenidos, incluido el líder.
`SepararCompanero` rechaza la separación mientras esté incorporado; en este corte
se sale primero con el Grupo y luego se aplica la separación explícita. No se
produce una salida implícita al ejecutar operaciones antiguas.

Los cambios de pertenencia que afecten al jugador se permiten en sesión disponible
o en un cierre sin acción Viajar suspendida. Un evento de llegada puede realizar
la incorporación desde `accion.terminada` o `partida.disponible`. No la confirma
en medio de un motor pendiente que después intentaría continuar un viaje individual.
La futura interrupción y posterior incorporación dentro de Viajar queda fuera.
Una raíz ya terminada que se conserva para preparar el contexto no equivale
a un Viajar pendiente de continuación.

## 6. Modo reclutado y autoridad de las reglas

El modo visible se deriva de que el jugador está físicamente EnColectivo; no
se guarda un booleano global paralelo. En este corte, estar incorporado limita
las acciones disponibles a Acampar. Las herramientas de desarrollador siguen
siendo accesibles como herramientas del frontend, no como acciones del mundo.

La sesión rechaza también ComenzarViaje recibido con una intención antigua o
desde otro consumidor. El núcleo y las solicitudes NPC rechazan Viajar individual
para cualquier miembro: ocultar el menú no sustituye la regla. Las políticas
legadas de Actores incorporados se omiten en esta etapa para no solicitar un
traslado individual incompatible; sus restantes eventos pueden seguir existiendo.

Salir devuelve las acciones normales si el Actor sigue vivo y cumple sus
precondiciones. No convierte a otros miembros en compañeros del jugador.
Abandonar campamento solo cierra Acampar: no es abandonar el colectivo.

## 7. Esperar órdenes sobre el mapa

### 7.1. Experiencia y condiciones de parada

En Acampar hay una sección horizontal **Acciones del campamento**. Dentro de
ella, arriba y abajo seleccionan Abandonar campamento o, si el Grupo está
incorporado, **Esperar órdenes**. La lista contextual permite agregar futuras
actividades sin crear una sección horizontal por acción. Al confirmarla se muestra el
mapa y comienza a avanzar automáticamente el tiempo de juego. No se pide
confirmación por cada jornada ni se vuelve al campamento al terminar cada una.

La espera continúa hasta que:

- El colectivo esperado realiza una acción confirmada: se muestran sus relatos
  y se vuelve al mapa, con el avance automático detenido.
- El jugador pulsa **Esc**: se detiene el avance y se vuelve a Acampar.
- El Grupo deja de estar en ese colectivo, muere el jugador o aparece un fallo
  técnico: se abandona la espera y se presenta el estado o diagnóstico correspondiente.

Cancelar no abandona la compañía ni deshace tiempo o efectos confirmados.
La barra indica «Esperando órdenes · Esc: volver al campamento» y sigue mostrando
la fecha. No tener líder, citas pendientes o condiciones verdaderas no desactiva
la cancelación ni detiene por sí solo el avance solicitado por el jugador.
No se genera una viñeta repetitiva por cada día sin novedades.

### 7.2. Estado, ritmo e intenciones

La fachada añade `EsperarOrdenes { campamento: IdCampamento }`, que valida fase,
ID, vida y pertenencia antes de abrir `EsperandoOrdenes`. Este estado guarda un
`IdEspera`, el colectivo esperado, el ordinal del próximo intervalo, la petición
de cancelación y el contexto de retorno al campamento. Entrar no adelanta el reloj.

El avance usa intervalos de **una jornada de juego**, conservando la fracción
del día. Cada intervalo completa un cierre normal con un IdCierre propio.
La cadencia inicial del frontend es una jornada cada 500 ms reales,
ajustable en su código. No es una regla del calendario ni depende de la tasa de redibujo.

El frontend usa un reloj monótono y solicita `AvanzarEspera { espera, intervalo }`
solo cuando vence el plazo y no hay un cierre, viñeta o menú pendiente. La sesión
acepta cada par ID/ordinal una sola vez. No admite avanzar dos intervalos en
paralelo ni intenciones de una espera anterior. `CancelarEspera { espera }`
invalida los avances futuros de esa espera.

Leer vistas, redibujar, animar el mapa o consultar fichas no pasa tiempo. El bucle
de entrada atiende teclas antes de iniciar otro intervalo: Esc ya recibido tiene
prioridad sobre el siguiente avance. Los plazos vencidos mientras se leía una
viñeta o se procesaba un cierre no se acumulan. Al reanudar se programa un nuevo
plazo completo, sin ejecutar una ráfaga para recuperar segundos reales.

### 7.3. Un cierre por intervalo, sin duplicados

Antes de abrir un intervalo se valida que la espera siga vigente y se proyecta
la fecha final comprobando desbordamientos. El cierre guarda origen explícito
`esperarOrdenes` y colectivo esperado; `ctx:objetivo()` puede identificarlo.
El origen no se deduce por descarte de Viajar.

```text
EsperandoOrdenes + AvanzarEspera válido
  → preparar cierre de una jornada con IdCierre nuevo
  → eventos accion.terminada
  → confirmar calendario una vez
  → citas EnFecha atravesadas
  → una fase NPC legada
  → eventos fase_npc.terminada y partida.disponible
  → revisar señales de parada
      otra salida: estado correspondiente
      cancelación: campamento actualizado
      acción del colectivo: mapa detenido
      ninguna: mapa en espera, próximo intervalo habilitado
```

Una semana de espera completada produce siete cierres y siete fases NPC, nunca
una fase por frame ni una sola por toda la semana. El coordinador común conserva
las ocurrencias atendidas. La ausencia de novedades no reejecuta una fecha ni
reabre el mismo cierre.

La acción del colectivo o una cancelación impiden iniciar otra jornada, pero no
descartan sucesos pendientes del cierre ya iniciado. Ese cierre termina en orden,
incluidas las demás citas del intervalo y su fase NPC. Un jugador que muere
durante él no recupera acciones vivas al terminar.
Si coinciden una acción y una cancelación, se respeta el retorno al campamento
solicitado. La muerte, salida del colectivo o fallo técnico tienen prioridad y
se presentan según su estado, sin restaurar una espera ya incompatible.

### 7.4. Qué significa que el colectivo actuó

La parada necesita una señal de actividad confirmada del colectivo esperado.
No basta que cambie cualquier elemento del mundo, que se muestre una viñeta
o que un evento declare observación EnColectivo.

Un `TrasladarColectivo` que cambia efectivamente su Lugar produce esa señal en
el resultado del lote. Repetir su mismo destino no la produce. Para órdenes u
otras acciones narradas sin traslado, la salida Lua de un paso Efectos admite
`acciones_colectivas: ["clave", ...]`, opcional y vacía por defecto. Son hechos
de actividad del mundo; no implementan todavía acciones generales colectivas.

Se validan las claves, los permisos, el lote y su continuación antes de confirmar
operaciones y señales juntos. Una condición falsa, un lote rechazado o un paso
que no se ejecutó no emite actividad. La señal queda asociada a la ocurrencia y
al paso confirmado, sin crear un booleano global persistente que dispare otra espera.

La sesión acumula las señales del cierre y las compara con el colectivo guardado
al iniciar la espera. Un evento ajeno puede pausar el reloj mientras se lee y
después reanudarlo; no da por recibidas órdenes de la compañía. La salida del
Grupo o muerte del jugador tienen su propia causa de parada, aunque ningún
colectivo haya actuado.

### 7.5. Cancelación y procesamiento cooperativo

Cada intervalo y programa Lua tiene límites, aunque el modo de espera pueda
continuar mientras el jugador lo desee. Se conservan los grafos y cadenas acíclicos
del contrato 23, el registro único de ocurrencias y el límite de instrucciones
Lua. Las citas no se encadenan recursivamente ni se reevalúan en el mismo cierre.

El coordinador cede control tras un máximo de 64 transiciones automáticas por
llamada. Si queda trabajo, publica ProcesandoCierre con un token. La intención
`ContinuarCierre { token }` solicita el siguiente bloque en un pulso posterior.
El token identifica cierre y revisión del cursor y se acepta una sola vez.
Continuar no confirma de nuevo calendario ni inicia otra fase NPC. Este presupuesto
distribuye CPU sin omitir sucesos ni limitar la cantidad de relatos.

Si Esc llega entre intervalos, vuelve al campamento inmediatamente. Si llega
durante un cierre, se registra la cancelación y se completa ese cierre mediante
bloques acotados, sin iniciar otro. Se siguen atendiendo teclado, resize y salida
del programa entre bloques. No hay un `while` que monopolice el hilo hasta hallar
una orden.

Las viñetas pausan el avance y conservan sus confirmaciones normales. Esc puede
solicitar el fin de la espera automática, pero no omite relatos o decisiones
pendientes del sistema narrativo. La petición se conserva hasta el retorno seguro.
Un fallo de Lua produce diagnóstico acotado; no se reintenta esperando que cambie.
Al alcanzar el límite representable del calendario también se detiene el avance
y se informa el problema; no se desborda la fecha ni se repite el último día.

### 7.6. Retorno al campamento y coste temporal

La espera conserva la acampada como contexto suspendido, sin cobrar antes otra
jornada por abandonarla. Sus intenciones de menú quedan inhabilitadas fuera de
Acampando. Al cancelar se reconstruyen ficha, integrantes, ubicación y opciones
desde el mundo actual, con un ID de campamento nuevo. Se conserva el foco anterior
solo si su referencia continúa siendo válida. Este retorno no vuelve a disparar
AcamparInicio como si fuera otra entrada voluntaria.

El núcleo amplía el ciclo de Acampada para distinguir descanso pendiente
y descanso ya cubierto por una jornada de espera confirmada. La sesión marca
ese crédito solo tras confirmar el primer intervalo; el crédito se conserva
al cancelar o reanudar la misma acampada. Su finalización consume el valor opaco
una sola vez y cobra cero jornadas adicionales si el descanso ya estaba cubierto.
No se ignora a posteriori un coste que el núcleo haya devuelto como obligatorio.

Cancelar antes del primer intervalo no pasa tiempo y conserva el día pendiente
del cierre normal de Acampar. Si ya pasaron jornadas, regresar al campamento o
cerrarlo después no añade un día artificial ni abre otro cierre o fase NPC.
Si una acción del colectivo termina
la espera, se retira también el contexto suspendido sin otro coste. Un nuevo
campamento abierto luego desde el mapa sí tiene su coste normal.

### 7.7. Continuidad de la historia

La cancelación garantiza que el jugador puede interrumpir el avance automático;
no demuestra que un guion arbitrario vaya a producir alguna vez una orden o una
salida del servicio. Ese segundo compromiso pertenece al contenido. No se libera
al Grupo ni se inventa una acción porque falte una cita.

La primera secuencia de servicio deberá incluir una salida fechada y finita,
cuya condición dependa solo de que el Grupo siga incorporado a esa compañía.
No dependerá de que el capitán viva, una victoria anterior ocurra o una variable
opcional esté activa. Otros sucesos pueden cambiar o abreviar ese servicio.
El escenario debe probarse hasta esa fecha, también con órdenes intermedias
omitidas y con el líder muerto.

Las condiciones de incorporación deben comprobar que el episodio de servicio
todavía es pertinente si se llega tarde. Una consulta pura
`mundo:hay_citas_futuras_del_colectivo(clave)` puede apoyar esas condiciones usando
el índice de citas con observación EnColectivo; solo informa de citas pendientes,
no promete que su condición vaya a cumplirse.

La validación detecta claves o fechas inexistentes y grafos sin terminación;
no afirma probar el significado de cualquier Lua. Para el contenido oficial,
la prueba del recorrido hasta la salida es un criterio de aceptación obligatorio.

## 8. Agenda y eventos que trasladan colectivos

Las citas siguen `(inicio, final]`, con entrada inclusiva única al iniciar la
partida. Se ordenan por fecha, prioridad e identidad del contrato 24. Una condición
falsa se registra OmitidoPorCondicion y no se reintenta para completar una espera.
Los eventos pueden modificar colectivos aunque el jugador esté en otro lugar.

Se añade `observacion: EnColectivo(colectivo: "clave")` al modelo común. Al activar
la ocurrencia se comprueba si el jugador está incorporado. Esa decisión de mostrar
el relato se conserva durante la ocurrencia, de modo que una salida no borre su
propia viñeta. Fuera del colectivo se omite la presentación y se confirman los
efectos, como en la observación automática del contrato 24.

EnFecha conserva grafos informativos sin opciones ni interrupción de acciones.
No se le concede una decisión remota al jugador. La incorporación inicial puede
usar las decisiones de un evento PorPuntos; las acciones futuras incorporarán
los puntos que necesiten.

Los cambios se realizan en Efectos mediante constructores de operaciones:

| Consulta u operación Lua nueva | Resultado o permiso |
|---|---|
| mundo:colectivo_de(actor) | Clave del colectivo físico o nil. |
| mundo:ubicacion_colectivo(colectivo) | Clave del Lugar actual. |
| mundo:lider_colectivo(colectivo) | Clave del líder o nil. |
| mundo:miembros_colectivo(colectivo) | Lista inmutable de claves de los Actores concretos. |
| mundo:hay_citas_futuras_del_colectivo(colectivo) | Hay citas pendientes con observación EnColectivo; no garantiza que sus condiciones se cumplan. |
| op.ingresar_actor_en_colectivo(actor, colectivo) | Ingreso de NPC independiente. |
| op.ingresar_grupo_en_colectivo(colectivo) | Ingreso conjunto del jugador y compañeros. |
| op.salir_actor_de_colectivo(actor, colectivo) | Salida de NPC independiente. |
| op.salir_grupo_de_colectivo(colectivo) | Salida conjunta del colectivo esperado. |
| op.trasladar_colectivo(colectivo, lugar) | Traslado narrativo, sin duración adicional. |
| op.establecer_lider_colectivo(colectivo, actor_o_nil) | Cambio de mando validado. |

Solo Efectos recibe `op`. La sesión habilita las operaciones que afecten al Grupo
en los puntos estables de la sección 5 y en el cierre de fecha. No se amplía
`op.trasladar_npc` para eludir sus prohibiciones sobre el jugador.
Para EnFecha, `ctx:fecha_programada()` devuelve la fecha nominal, `ctx:ahora()`
el cierre efectivo y `ctx:actor()`, `ctx:accion()` y `ctx:objetivo()` devuelven nil;
participantes queda vacío. `ctx:jugador_presente()` conserva la observación fijada
al activar. `mundo:actor_jugador()` devuelve la clave del protagonista actual.

La salida Efectos admite además `acciones_colectivas`, según la sección 7.4,
como ampliación del contrato 23. Sus claves y señales se confirman junto al lote;
el formato anterior que omite el campo conserva su significado.

Después de cada lote confirmado se recalculan la posición efectiva, la vista y
la observación del mapa. Las futuras citas usan ese mundo actualizado. El contexto
de origen del evento y la fecha programada se conservan por separado. La cámara
y `VistaEvento.lugar` no permanecen fijadas al puerto anterior después de zarpar.
No se revelan tiles de trayectos que no se hayan representado.

Una continuación que restaure Acampar reconstruye su vista desde el mundo actual,
conservando el ID solo mientras esa acampada siga viva y no sea el retorno de
Esperar órdenes, que renueva el ID según la sección 7.6. No reinstala una copia
antigua de la pertenencia, el líder o las opciones. Una acampada retirada no se
reabre implícitamente para prolongar la espera.

Ejemplo de efecto, dentro del esquema Efectos del contrato 23:

```lua
return {
    operaciones = {
        op.trasladar_colectivo("compania_dahran", "mar_del_viento")
    },
    siguiente = "relato"
}
```

La fecha pertenece a `activacion: EnFecha(...)`; el efecto no espera, no ejecuta
un bucle y no adelanta de nuevo el reloj. El relato posterior describe el suceso
que se acaba de confirmar.

## 9. Interfaz del modo reclutado

### 9.1. Información neutral

`VistaMundo` incorpora un resumen opcional de pertenencia: ID y nombre del
colectivo, Lugar efectivo y líder opcional con su nombre. `VistaCampamento`
incluye el mismo contexto y si ofrece Esperar órdenes. La disponibilidad procede
de la aplicación; el frontend no la deduce leyendo un booleano o una biografía.

La vista de EsperandoOrdenes comparte el mapa de VistaMundo y añade el IdEspera,
el próximo ordinal habilitado y si hay un cierre o cancelación en curso. No habilita
el menú normal de acciones. La terminal dibuja la fecha y el aviso de cancelación,
emite los pulsos temporales y mantiene el reloj pausado durante viñetas o menús de
desarrollador. Cerrar esos menús renueva el plazo y no acumula jornadas pendientes.

Los retratos siguen siendo únicamente los del jugador y sus compañeros. Se
conservan su orden y su anclaje sobre las dos barras inferiores.

### 9.2. Marco y composición

Mientras se dibuja el mapa y el Grupo está incorporado, su región exterior
recibe un marco opaco negro de tres celdas de grosor. Las dos barras inferiores
quedan fuera. El negro respeta la política cromática vigente del frontend. Los fondos
transparentes de los retratos dejan ver el marco allí donde se superponen.
El marco utiliza espacios con estilo explícito, sin heredar colores o modificadores
de los tiles que estuvieran debajo.

La banda superior muestra **«Dentro de: {nombre del colectivo}»**, centrado y
recortado al ancho disponible. No concatena el nombre del líder para inventar un
nombre; un colectivo sin líder conserva su identidad y su rótulo.

El mapa se proyecta en el rectángulo interior. Capas de fondo, arte de Lugares,
nombres, tráfico y trazos de desarrollo respetan ese recorte. Los retratos se
dibujan después sobre la región exterior original, por encima del marco. Las
viñetas y diálogos conservan su prioridad superior. Así el marco queda entre
el mapa y los personajes sin desplazarlos.

Cámara, rutas e inspector de coordenadas comparten esa nueva geometría interior.
El borde no corresponde a tiles y el ratón sobre él no devuelve coordenadas.
Redimensionar no modifica ubicaciones ni memoria de exploración. El modo de
desarrollador que revela el mapa conserva el marco y no altera las restricciones.

Se mantiene el mínimo terminal actual. Si una región de dibujo extraordinariamente
pequeña no admite el marco completo, se reduce su grosor hasta dejar área interior
o se usa la presentación compacta existente, con aritmética saturada. No se
introducen desbordamientos por restar seis celdas.

En Acampar sigue mostrándose la ficha, no el mapa. Su cabecera o barra contextual
conserva el nombre del colectivo; Esperar órdenes y Abandonar campamento
comparten la lista vertical de Acciones del campamento. No se agregan opciones para incorporarse, salir o formar compañías.

## 10. Responsabilidades y errores

| Capa | Responsabilidad |
|---|---|
| core/modelo.rs y operaciones | IDs, ubicación efectiva, miembros, líder, invariantes e índices atómicos. |
| core/acciones | Rechazo de acciones individuales incompatibles, intervalos de espera y crédito temporal de Acampada. |
| app/contenido.rs y fichas.rs | RON, claves, diagnósticos, referencias derivadas y vistas neutrales. |
| app/eventos | Agenda común, ocurrencias, permisos, observación EnColectivo, operaciones Lua y señales confirmadas de actividad. |
| app/sesion | IdEspera, IdCierre por intervalo, cancelación, calendario único, procesamiento reanudable y retorno al control. |
| terminal | Marco, geometría, mapa de espera, cadencia monótona, Esc y continuación del procesamiento. |

La carga rechaza colectivo desconocido, líder ausente de su propio colectivo,
ubicación inválida, grupos divididos, duplicados y referencias inválidas. Los errores
de dominio distinguen pertenencia distinta, ubicación no coincidente, Actor del
Grupo usado como independiente y traslado individual de un incorporado.

Una condición falsa, ausencia de líder o falta de citas no es un error técnico.
Un fallo de Lua, operación inválida o presentación inválida usa el diagnóstico
del sistema narrativo, conserva lo ya confirmado y no reejecuta automáticamente
el cierre. No se oculta reintentando la espera. Si corresponde una sesión Fatal,
se informa el problema de contenido o infraestructura en vez de dejar una pantalla
aparentemente esperando para siempre.

## 11. Verificación y criterios de aceptación

1. Cargar contenidos anteriores sin colectivos conserva sus ubicaciones,
   fichas, acciones y resultados. No se introducen archivos obligatorios nuevos.
2. Ingresar y sacar al Grupo afecta a todos sus integrantes, conserva los
   compañeros y actualiza los índices. Un último efecto inválido revierte el lote.
3. Trasladar la compañía mueve la posición efectiva del líder y de sus miembros
   sin copiar Lugares. No mueve a otros Actores que estaban en el mismo puerto.
4. Se rechazan Viajar y Trasladar individuales para incorporados, desde interfaz,
   intenciones antiguas, núcleo y políticas NPC. Acampar sigue siendo válido.
5. Cada intervalo confirma exactamente una jornada y una fase NPC. Dos Enter,
   un pulso duplicado o un token antiguo no agregan tiempo ni repiten efectos.
6. Sin citas, con todas las condiciones falsas o sin líder, el mapa sigue visible
   y el tiempo avanza hasta cancelar. Esc retorna a Acampar; no se crea un líder.
7. Citas con fecha atravesada se resuelven en orden y una vez; una omitida no se
   reevalúa. Una cadena inválida o un Lua infinito produce diagnóstico acotado.
8. Un catálogo con miles de pasos cede procesamiento sin omitir ninguno; resize,
   cancelación, salida y vistas repetidas no reinician el cursor ni ejecutan efectos.
9. Se observa el movimiento y la salida del propio colectivo aunque cambie la
   pertenencia durante el episodio. Jugador externo no ve relatos privados.
10. Movimiento, salida y muerte del líder actualizan cámara, visión, ambiente,
    cabecera y menú desde el estado confirmado. No se restaura una vista antigua.
11. TestBackend comprueba marco, rótulo, orden de capas, conservación de retratos,
    barras, tamaños y ausencia de coordenadas en el borde; después se revisa en juego.
12. El fixture de servicio tiene salida finita. Se recorre desde la incorporación
    hasta quedar libre de la compañía, con y sin órdenes intermedias válidas,
    con el líder muerto y con llegada tardía que impide una incorporación inválida.
13. Cancelar antes del primer pulso no pasa tiempo; cancelar durante un cierre
    termina solo ese intervalo. El campamento vuelve actualizado, con ID nuevo,
    sin repetir AcamparInicio ni cobrar otra jornada ya cubierta por la espera.
14. Una acción confirmada del colectivo esperado detiene el avance tras su cierre.
    Un traslado idempotente, condición falsa, evento ajeno o lote rechazado no
    emiten esa señal. Una cancelación coincidente conserva su retorno a Acampar.
15. Reloj simulado verifica la cadencia sin sleeps: redibujos no adelantan tiempo,
    una viñeta o menú pausa los pulsos y su cierre no dispara jornadas acumuladas.
    El límite del calendario detiene la espera con diagnóstico.

Los controles de formato, compilación, clippy, pruebas, carga de contenido y
compilación optimizada siguen la metodología vigente. La demo se prepara con
`python3 tools/preparar-demo-colectivos.py /tmp/desert-tiger-colectivos-demo`;
su README explica el recorrido y el comando completo del binario terminal.
