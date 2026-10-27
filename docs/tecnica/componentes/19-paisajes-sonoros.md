# Paisajes sonoros del entorno

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 1.0 |
| Fecha | 2026-10-26 |

## 1. Alcance y contexto

Amplía el [audio procedural](./17-audio-procedural.md) con viento suave, un ave
lejana ocasional, corriente de río, oleaje y dos cantos de bosque distintos.
Todo se sintetiza con FunDSP en el frontend, sin grabaciones ni nuevas
dependencias. Se conservan la compilación opcional, `--no-audio`, los errores
recuperables y el único stream CPAL. No cambia reglas ni tiempos del juego.

El paisaje acompaña al grupo en el mundo, en el campamento y durante todas las
viñetas y decisiones de Viajar. Q conserva el sonido. Una salida confirmada o
un estado fatal solicita silencio.

## 2. Selección del paisaje

`VistaLugar` añade `caracteristicas: Vec<String>` y `conectados: Vec<IdLugar>`.
La aplicación proyecta datos actuales del Mundo y su índice de conexiones;
no decide qué suena ni consulta coordenadas. Se amplía la vista neutral del
[contrato 04](./04-sesion-aplicacion.md); los demás consumidores pueden ignorar
estos campos. No se agregan características artificiales al contenido.

El frontend toma `progreso.lugar_confirmado` en viñetas y decisiones de viaje;
en las otras vistas toma `lugar_jugador`. Cambiar el foco, elegir un destino o
abrir una ficha de otro lugar no cambia el entorno sonoro. No se anticipa el
paisaje de `siguiente_lugar`.

| Capa | Condición |
|---|---|
| Viento y ave distante | Grupo en cualquier lugar válido. |
| Río | Lugar actual con `fluvial`, o vecino directo `Conectado` con `fluvial`. |
| Mar | Lugar actual con `costero` o `maritimo`. |
| Bosque, dos aves | Lugar actual con `boscoso`. |
| Fogata existente | Vista `Acampando`, además del paisaje. |

Las capas se pueden sumar: un bosque conectado a un río tiene ambas.
`aguaDulce` sola no significa río. No se busca por nombre, distancia visual,
tipos de tile ni conexiones de segundo grado. La consulta usa los vecinos
indexados, sin recorrer el grafo entero por cuadro.

## 3. Datos y organización

`assets/audio/ambientes.ron` se carga junto con `fogata.ron`, una sola vez antes
de raw mode, salvo audio deshabilitado o compilación sin audio. Campos
obligatorios, desconocidos o duplicados rechazados; todos son `f32` finitos
en `[0,1]`. Cero silencia la capa; los errores indican ruta y campo.

```ron
(
    volumen: 0.35,
    viento: 0.10,
    ave_lejana: 0.18,
    rio: 0.30,
    mar: 0.26,
    ave_bosque_a: 0.20,
    ave_bosque_b: 0.16,
)
```

`audio/paisaje.rs` selecciona capas desde vistas neutrales, sin FunDSP.
`audio/ambientes.rs` contiene los generadores y su azar cosmético local.
`configuracion.rs` valida ambos recursos. `motor.rs` mezcla y aplica rampas;
`dispositivo.rs` conserva su responsabilidad de salida y saneamiento.
`fuentes.rs` comparte filtros de ruido y azar cosmético con la fogata, sin
modificar su receta ni compartir estados entre generadores.

`AmbienteSonoro` pasa a ser un valor con banderas de exterior, río, mar, bosque
y fogata. Se publica en una sola orden atómica `u8`: fogata=1, exterior=4,
río=8, mar=16, bosque=32. Cero es silencio y 128 es cierre. No se asigna memoria
ni se cargan datos dentro del callback. Cambios repetidos son idempotentes.

## 4. Síntesis y transiciones

Los nodos de ruido y filtros FunDSP se construyen y preasignan antes de iniciar
el stream, con frecuencia real y semillas propias. No usan el RNG del juego.

- Viento: ruido rosa filtrado entre 90 y 950 Hz, modulación lenta y suave.
- Río: ruido blanco filtrado entre 180 y 4800 Hz; pequeñas variaciones continuas
  y una componente burbujeante suave. No produce golpes periódicos fuertes.
- Mar: ruido rosa y blanco filtrado entre 70 y 2400 Hz; envolventes lentas de
  crecida y retirada, con periodos superpuestos distintos para evitar un pulso
  mecánico perfectamente uniforme.
- Ave lejana: una voz sinusoidal de tono descendente, duración aproximada
  de 0.7 s, intervalos aleatorios de 12–28 s.
- Ave de bosque A: frases de tres notas agudas ascendentes, aproximadamente
  0.9 s por frase, separadas por 2.5–6 s.
- Ave de bosque B: dos silbidos más graves y ondulados, aproximadamente
  1.1 s por frase, separados por 4–9 s. Agenda y tono independientes de A.

Las voces de aves usan osciladores FunDSP modulados por muestra, envolventes
suaves y un máximo de tres voces preasignadas: ninguna agenda acumula eventos.
La primera espera se sortea al construir cada voz. Los filtros superiores se
limitan a `0.45 * frecuencia_muestreo`; las frecuencias admitidas siguen el
contrato 17. Todos los generadores tienen memoria acotada.

Cada capa ambiental entra o sale con una rampa lineal de 1500 ms desde su
ganancia actual. Cambiar de río a bosque no reinicia viento ni aves comunes.
Una capa completamente apagada suspende su generador y su agenda. La fogata
mantiene sus tiempos propios y su receta. Cerrar aplica 20 ms a todas las
capas y conserva la espera máxima del contrato 17.

Se suman las capas con sus volúmenes y se limita suavemente la salida mediante
`0.8 * tanh(suma / 0.8)`. El resultado mono se duplica en estéreo. No hay
posicionamiento de fuentes a izquierda/derecha ni simulación de propagación.

## 5. Verificación

- Proyección de características y vecinos, incluidas conexiones inversas.
- Selección por lugar confirmado, sin anticipar destino ni seguir foco de
  cámara/fichas; combinaciones río/bosque, costa y mar abierto.
- Configuración estricta y recursos oficiales; silencio por volúmenes cero.
- Señales finitas y acotadas, reproducibilidad, ausencia de reinicios al repetir
  órdenes, rampas e independencia del tamaño de buffer. Aves con frases
  distintas e intervalos acotados, sin asignaciones en el callback.
- Regresiones de fogata, fase NPC, viaje y compilación sin audio.
- Controles de calidad de workspace y release con y sin audio; recorrido PTY
  con captura CPAL y recuperación ante salida no disponible.
- Revisión auditiva jugando: volumen sutil, aves ocasionales fuera del bosque,
  dos cantos reconocibles dentro, corriente y oleaje distinguibles. Las pruebas
  numéricas no sustituyen esta valoración.

## 6. Verificación funcional — 2026-10-26

El código y los recursos están integrados. Pasan fmt, check, clippy sin
advertencias, pruebas y compilación release con y sin audio: 154 pruebas con
las features predeterminadas y 139 sin ellas. Las pruebas incluyen proyección
de características y vecinos inversos, selección y combinaciones, seguimiento
del lugar confirmado en viñetas y decisiones, configuración estricta, señales
por capa y transición sin reiniciar capas compartidas.

La revisión del callback confirma que nodos, voces y filtros se construyen
antes del stream; las órdenes y muestras no crean colecciones, bloquean ni
acceden a archivos o a la sesión. La configuración deshabilitada omite ambos
recursos y el dispositivo.

El ejecutable se comprobó en PTY mediante captura ALSA file/null de la salida
CPAL: entorno inicial, bosque junto a río, mar abierto, costa y vado fluvial.
Las muestras son finitas, acotadas y no nulas. Navegación, cancelación de Q,
resize, cierre y reapertura del campamento terminaron con restauración de
terminal. También se verificaron audio deshabilitado, salida incompatible,
compilación sin audio y cierre por fatal de calendario.

La escucha de los nuevos ambientes queda pendiente del usuario; ninguna prueba
numérica se presenta como valoración artística. La aceptación previa de la
fogata se conserva por separado en el plan 041.
