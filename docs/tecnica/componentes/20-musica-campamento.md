# Música ocasional del campamento

| Campo | Valor |
|---|---|
| Estado | Implementado; preparación diferida verificada |
| Versión | 1.4 |
| Fecha | 2026-10-27 |

> La música y las notas fueron aceptadas por el usuario. El [plan 043](../../planes/completados/2026-10-26-43-integrar-musicos-campamento.md)
> conserva su cierre. La expresión del intérprete es un retoque posterior.

## 1. Alcance

Un Actor del grupo con `musico`, sin `muerto`, puede interpretar ocasionalmente
las piezas del repertorio de guitarra mientras se acampa. Incluye al jugador si
tiene esa característica. El sirviente inicial recibe `musico` y su descripción
se incorpora al catálogo. No hay chequeos, gasto de tiempo, inventario de
instrumentos ni nuevas acciones Lua. Es presentación del campamento.

La versión grave aceptada conserva sus notas, tempo y timbre. Se mezcla con la
fogata y el paisaje del [contrato 19](./19-paisajes-sonoros.md), a volumen
ajustable. Solo toca un Actor a la vez. Explorar fichas, cambiar el foco o
redimensionar no reinicia la pieza. Q conserva la música actual; mientras se
confirma la salida no comienzan piezas nuevas. Al dejar Acampar o salir, la
música se desvanece. Sin audio disponible no se programa ni se dibujan notas.

La agenda es única para todo el campamento: tener varios músicos amplía la
selección de intérprete, no crea solicitudes simultáneas. Otro Actor solo puede
ser elegido tras finalizar la pieza y respetar la pausa. Una banda con partes
distintas y armonizadas queda expresamente aplazada.

## 2. Datos y preparación

`assets/audio/musica/campamento.ron` contiene:

```ron
(
    partituras: [
        "recuerdo_del_hogar.ron",
        "hilo_del_camino_castellana.ron",
    ],
    volumen: 0.50,
    espera_inicial_segundos: (60, 120),
    pausa_entre_piezas_segundos: (20, 180),
)
```

Se declara `partituras` con 1–16 nombres distintos de archivos RON dentro de la
carpeta musical, sin directorios, rutas absolutas ni componentes de ascenso.
Se conserva el formato anterior `partitura: "archivo.ron"` para una sola pieza;
se rechaza combinar ambos campos, omitir ambos o usar una lista vacía.
Estos campos opcionales usan opciones implícitas de RON: no requieren `Some`.
Los demás campos son obligatorios; se rechazan desconocidos y duplicados.
El volumen es `f32`
finito en `[0,1]`. Cada intervalo es un par `u32`, mínimo 1, máximo 3600 y
primer valor menor o igual al segundo. La pausa se cuenta desde el fin de la
pieza, no desde su comienzo.

La preparación de audio lee y valida este archivo y las partituras antes de raw mode;
deshabilitar el audio o compilar sin él omite todos estos recursos. Los errores
identifican archivo y contexto. La partitura usa el formato experimental
[documentado junto al recurso](../../../assets/audio/musica/README.md).

`audio/partitura.rs` comparte con la previsualización la lectura, validación y
síntesis FunDSP. Calcula el tamaño previsto sin sintetizar y rechaza repertorios
que superen diez minutos a 48000 Hz antes de abrir la terminal.

Tras dibujar el primer cuadro y abrir la salida de audio, un único hilo de
`audio/preparacion_musica.rs` sintetiza secuencialmente las partituras ya leídas.
No relee los archivos durante el juego ni usa el azar de la Partida. La síntesis
cede ejecución y comprueba cancelación entre bloques de muestras. El hilo de
interfaz nunca espera su resultado mientras se juega. Deshabilitar audio, no
poder abrir el dispositivo o configurar volumen musical cero evita este trabajo.

Genera una sola vez buffers mono a 48000 Hz con las piezas completas. No se
carga un WAV. Publica el repertorio completo e inmutable mediante un
`Arc<OnceLock<Result<Repertorio, String>>>`; `get` permite consultarlo sin esperar
ni bloquear. Los buffers se comparten mediante `Arc`
con el procesador, que adapta su velocidad a la frecuencia del dispositivo
mediante interpolación lineal. No crea, resetea ni reserva voces en el callback.
Se conservan las muestras y el timbre de la previsualización aprobada.

La fogata y los ambientes funcionan desde el comienzo. Hasta que el repertorio
esté listo y contenga música audible, la agenda musical permanece deshabilitada:
acampar inmediatamente sigue funcionando, sin intérprete, notas ni caras musicales
ficticias. Al quedar listo, un campamento abierto inicia su espera inicial desde
ese momento; uno posterior la inicia al abrirse. El bucle consulta la disponibilidad
actual, no una copia tomada al arrancar.

Un fallo de síntesis o del hilo deshabilita únicamente la música y genera un
aviso breve una vez; conserva los demás sonidos. No se reintenta automáticamente.
Al cerrar el programa o perder la salida de audio se cancela la preparación y
se recoge el hilo; no se espera la síntesis completa. La previsualización WAV
continúa siendo una operación síncrona explícita fuera del juego.

La primera solicitud interpreta el primer archivo de la lista. En cada nueva
solicitud, el reproductor elige entre las otras piezas mediante un azar
cosmético propio; nunca repite inmediatamente la anterior si hay al menos dos.
Con dos piezas se alternan. El último índice se conserva también al salir y
volver al campamento, mientras siga abierto el mismo stream. Una orden
repetida no cambia la pieza ni la reinicia. La selección consulta el vector
ya preparado y no reserva memoria en el callback. Se mantiene un solo
intérprete y una sola pieza activa; no introduce bandas ni capas instrumentales.

## 3. Agenda y coordinación

`musica_campamento.rs` pertenece al frontend y mantiene la agenda. Al abrir un
campamento consulta las fichas de sus integrantes y conserva los elegibles.
No agrega campos de audio a `core` ni a `app`. El conjunto se vuelve a consultar
al abrir el siguiente campamento; sus fichas no cambian el estado del mundo.

El reloj de la agenda es tiempo real sustituible en pruebas, independiente del
calendario y del reloj visual que Q congela. Intervalos y elección entre músicos
usan un azar cosmético local, con semilla controlable, ajeno a la Partida.
La primera interpretación comienza tras el intervalo inicial. Las siguientes
respetan la pausa configurada. Sin candidatos no se solicita reproducción.

Cada solicitud lleva un ID `u32` creciente y el Actor elegido; el ID no se
reutiliza dentro de una ejecución. Si se agota, no se generan más solicitudes.
Solo el ID se comunica al audio; el callback no recibe Actores ni la Partida.
La orden usa `AtomicU32`. El estado confirmado usa `AtomicU64`: ID en los
32 bits altos y posición en muestras de la partitura en los bajos; el bit 31
inferior indica finalización. Cero significa que no hay interpretación.
Publicación Release y lectura Acquire. El cierre y los fallos limpian el estado.

El motor reproduce cada ID una vez, publica su avance y conserva el fin
confirmado hasta recibir otra orden. Esto evita perder la finalización si la
interfaz se retrasa. Una solicitud cero cancela con rampa de 150 ms; entrar
aplica 40 ms y el cierre del programa, 20 ms. La salida final usa el limitador
compartido. Se siguen mezclando las capas ambientales durante la interpretación.

El bucle sincroniza antes de dibujar y después de procesar entrada: consulta
fallos/estado, actualiza agenda, envía la solicitud vigente y recoge avisos.
El dibujo nunca inicia música. No se dibuja un músico hasta que el callback
confirme ese ID reproduciéndose. Se descartan estados de IDs anteriores al
cerrar y volver a abrir un campamento. No hay reproducción durante Viajar.

## 4. Notas sobre los retratos

Mientras el audio confirme que suena una pieza, todos los integrantes usan
temporalmente la expresión `music` de su propio RON, también quienes escuchan
sin ser músicos. Esta clave reemplaza el nombre anterior `playing`. Se aplica
únicamente al dibujar, antes del parche habitual de parpadeo; no modifica
`VistaActor.expresion` ni estados del mundo. Si un retrato no tiene esa variante,
conserva su expresión vigente. Al terminar o cancelar, todos recuperan sus
expresiones previas. El sirviente conserva su cara musical y el príncipe y el
bandido iniciales incluyen variantes basadas en sus sonrisas existentes.

La geometría de retratos se comparte entre el dibujo y la localización del
músico. El campamento dibuja `♪`, `♫` y `♩` solo cerca de su cabeza, ascendiendo y
desplazándose suavemente dentro de la región de arte de retratos/fogata. Se
derivan de la posición musical confirmada, sin otro temporizador independiente.

Se pintan en esa misma composición, después de retratos y antes de la fogata,
solo sobre celdas vacías, conservando el fondo. No reemplazan caras, barras ni
texto de fichas; se recortan en terminales pequeñas. Si no hay hueco se omite
esa nota visual, sin afectar la música. La animación sigue al músico aunque se
abra la ficha de otro elemento. El ritmo de refresco mientras toca permite
movimiento suave incluso con la fogata estática. Los tres glifos son de una
columna según el repertorio de ancho utilizado por el frontend.

## 5. Verificación

- Configuración, rutas, partitura oficial y valores inválidos; compilación sin
  audio y recursos omitidos cuando corresponde.
- Preparación diferida: carga sin síntesis, presupuesto previo, publicación única,
  espera sin bloquear, fallo aislado y cancelación por bloques. Campamento antes
  y después de disponibilidad; ninguna nota ni solicitud antes del repertorio.
- Agenda controlada: sin músicos, uno/varios, muerto excluido, intervalos,
  selección, ID estable, Q, cancelación, reapertura y fin confirmado tardío.
- Motor offline: mezcla, remuestreo a frecuencias admitidas, inicio/fin una sola
  vez, repetición de orden, cancelación, cierre, señal finita y acotada.
- Dibujo: origen del músico, glifos de una columna, movimiento y recorte, sin
  modificar caras, ficha ni barras. Ninguna nota sin confirmación de audio.
- Pruebas de regresión de Acampar, Viajar, NPC y restauración de terminal;
  controles de calidad y release con y sin audio. Captura CPAL en PTY para
  comprobar la música y las notas coordinadas. La pieza ya fue aceptada; la
  integración visual y auditiva se revisa jugando al terminar el incremento.

## 6. Verificación funcional — 2026-10-26

La implementación comparte el sintetizador en `audio/partitura.rs` entre el
ejemplo de previsualización y la preparación del juego. `audio/reproductor.rs`
procesa el buffer y su estado; `audio/transicion.rs` comparte las rampas con el
motor ambiental. La agenda y las notas viven en `musica_campamento.rs`; la
geometría de retratos se consulta desde un único iterador del frontend.

Pasan fmt, check, clippy sin advertencias y pruebas de workspace con y sin
audio: 169 y 145 pruebas respectivamente. Las dos variantes compilan en
release. Las pruebas cubren recursos, elegibilidad, selección, pausas, Q,
confirmación, estados antiguos, remuestreo, repetición, cancelación y cierre.
El TestBackend verifica notas visibles junto al sirviente oficial, movimiento,
recorte y conservación de las caras, fondos, fichas y barras.

Un recorrido del ejecutable en PTY con captura CPAL mostró las primeras notas
a los 8.16 segundos de acampar. Hubo navegación, Q cancelado, resize y cierre
con restauración de terminal; se capturaron 8334668 muestras finitas y acotadas.
Otros cinco recorridos comprobaron audio normal, deshabilitado, salida
incompatible, compilación sin audio y cierre fatal de calendario. La pieza fue
aceptada previamente por el usuario; estos recorridos no sustituyen la revisión
estética de su integración con el campamento.

El usuario confirmó después que la integración funcionó perfectamente. El
retoque posterior agrega `music` y verifica su aplicación exclusiva al
intérprete, la restauración de la expresión anterior, el respaldo sin variante
y que varios músicos no generen interpretaciones superpuestas. No modifica
la melodía ni añade un sistema de banda.

La ampliación pasa fmt, check, clippy, 171 pruebas con audio y 147 sin audio,
y las dos compilaciones release. La prueba de dibujo verifica que solo cambia
el parche del intérprete, que los demás retratos conservan su expresión y que
al terminar se recupera la imagen anterior. Otra prueba mantiene varios
candidatos durante una misma interpretación y confirma que la solicitud y el
intérprete permanecen únicos hasta el fin confirmado.

El repertorio incorpora después `Recuerdo del hogar`, una pieza más larga
inspirada en el compás de tres tiempos de `assets/FF5_home.mid`, con otra
melodía y acompañamiento. Conserva el instrumento de la versión grave.
La compatibilidad del formato individual, listas inválidas y selección sin
repeticiones consecutivas se verifican mediante pruebas dedicadas. Las
verificaciones históricas de los incrementos anteriores se conservan arriba.

La ampliación de repertorio pasa fmt, check, clippy, 173 pruebas con audio y
148 sin audio, además de ambas compilaciones release. La nueva previsualización
dura 60.88 segundos, frente a 24.84 de la pieza anterior. Un recorrido del
ejecutable con captura CPAL confirmó el inicio, las notas del intérprete,
navegación y salida con restauración de terminal. La valoración musical de
la segunda composición queda pendiente de escucharla; la primera se conserva.

Para el uso normal del juego, las esperas se ampliaron a 60–120 segundos antes
de la primera pieza y 20–180 segundos después del fin de cada interpretación.
Una prueba de reloj virtual verifica esos rangos y que no haya solicitudes
antes de sus vencimientos. Los registros de 8 segundos anteriores corresponden
a la configuración de prueba que se utilizaba al implementar el sistema.
La expresión se renombró a `music` y ahora incluye a los oyentes: la prueba de
dibujo verifica las caras del grupo inicial, sus restauraciones y el respaldo
para actores sin la variante. Las notas siguen identificando solo al intérprete.

Esta revisión pasa fmt, check, clippy, 174 pruebas con audio y 149 sin audio,
además de ambas compilaciones release. El recorrido PTY confirma música,
notas, navegación y restauración de terminal; usa una copia temporal del
contenido de audio con espera de un segundo para acelerar la comprobación.
Los intervalos oficiales 60–120 y 20–180 se conservan intactos y se verifican
con reloj virtual, sin esperar varios minutos en las pruebas.

## 7. Preparación diferida — 2026-10-27

Pasan los controles de calidad, 188 pruebas con audio y 158 sin él, y ambas
compilaciones release. Las pruebas incluyen publicación del repertorio,
cancelación, fallo del hilo, presupuesto previo y campamento abierto antes de
tener música. La previsualización de la pieza castellana produce un WAV idéntico
byte por byte al generado antes del cambio.

En PTY con salida CPAL de prueba, el primer cuadro aparece a los 0,054 segundos
en desarrollo, mientras el sintetizador sigue activo. El campamento funciona
con fogata audible y el programa cierra en 0,312 segundos, incluyendo las pausas
del guion de prueba, sin completar las canciones. En release se confirman
reproducción y notas tras la preparación. También se verifican `--no-audio`,
el binario sin esa feature y la restauración de la terminal en los cuatro casos.
Los tiempos son mediciones locales de arranque del ejecutable, sin compilación.
