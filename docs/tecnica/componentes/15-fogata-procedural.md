# Fogata procedural del campamento

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.2 |
| Fecha | 2026-10-26 |

> Ampliación implementada: el [contrato 16](./16-paleta-terminal.md) gobierna
> la resolución de colores nativos, los desconocidos sin tintado y el modo fijo.

## 1. Propósito y alcance

La fogata del campamento tiene una llama animada, chispas que ascienden hacia la
derecha y luz cálida que modifica los colores de los retratos cercanos. La
variación se genera en el frontend y utiliza colores RGB continuos entre los
extremos de un degradado.

Desarrolla la presentación de [Acampar](../../diseno/acampar.md#2-entrada-y-presentación).
Para esta ampliación prevalece sobre el recurso estático y el aplazamiento de
animación del [contrato de Acampar](../acciones/07-acampar.md#8-composición-visual-y-lectura-completa),
y sobre la frecuencia del bucle del
[contrato 09](./09-frontend-terminal.md#8-bucle-animación-y-restauración).
La implementación y sus verificaciones están cerradas en
[PLAN-2026-039](../../planes/completados/2026-10-26-39-animar-iluminar-fogata.md).

Comprende el recurso visual, generación del efecto, composición, reloj y pruebas.
Quedan aplazados sombras, oclusión de luz, simulación física del fuego, otros
emisores, efectos en el mapa, controles gráficos y detección de capacidades de
terminal. Las reglas de Acampar, sus fichas y su coste de un día se conservan.

## 2. Responsabilidades y módulos

Todo el desarrollo pertenece a `crates/desert-tiger-terminal`. Se mantienen un
proceso, el bucle síncrono y las dependencias `terminal → app → core`.

| Módulo | Responsabilidad |
|---|---|
| `animacion.rs`, nuevo y privado | Tiempo visual y próximos vencimientos. |
| `fogata.rs`, nuevo y privado | Generar llama y chispas, convertir colores para la luz y componer el efecto. |
| `recursos.rs` | Cargar y validar la configuración opcional. |
| `campamento.rs` | Calcular región y origen; llamar al efecto en el orden establecido. |
| `interfaz.rs` | Conservar el reloj visual y transmitir sus dos resoluciones. |
| `lib.rs` | Medir tiempo monotónico y combinar sus vencimientos con la entrada. |

La API de `app` y `core`, los archivos de Actores, Lua y el azar de la Partida
no cambian. No se agregan dependencias, hilos, tareas asíncronas ni otro
renderizador. Ratatui sigue recibiendo el cuadro completo.

## 3. Tiempo visual

`RelojVisual` es un tipo privado del crate. Acumula `std::time::Duration` de
tiempo activo y conserva la fracción pendiente. Expone:

```rust
fn avanzar(&mut self, transcurrido: Duration);
fn instante_ms(&self) -> u64;
fn paso_250_ms(&self) -> usize;
fn hasta_siguiente(&self, intervalo: Duration) -> Duration;
```

- Comienza en cero. `avanzar` usa suma saturada, sin consultar relojes externos.
- `instante_ms` devuelve el tiempo acumulado redondeado hacia abajo a múltiplos
  de 50 ms; cualquier conversión que exceda su tipo satura.
- `paso_250_ms` deriva del mismo acumulador mediante división entera por 250 ms.
  Sustituye el contador independiente `fotograma` de `Interfaz`.
- `hasta_siguiente` devuelve la distancia al próximo múltiplo estricto del
  intervalo. Solo se utiliza con 50 o 250 ms.

La espera del bucle es de hasta el próximo múltiplo de 50 ms cuando se muestra
un campamento con efecto habilitado; en los demás modos conserva 250 ms. La
entrada y el resize interrumpen esa espera. Hay un solo reloj y un solo cuadro
por actualización: una demora permite muestrear directamente el instante
actual, sin reproducir cuadros atrasados ni iterar pasos de simulación.

`Instant` queda en el bucle de terminal. Después de cada espera, haya vencido o
haya llegado un evento, se contabiliza el intervalo transcurrido bajo el estado
de pausa anterior. Esto se hace antes de procesar la entrada; se actualiza el
punto de medición incluso si está pausado. Mientras está abierta la confirmación
global de salida no se llama a `avanzar`. Al cancelarla se conservan el instante
y su fracción pendiente y se descarta la duración pausada. Durante esa pausa se
usa una espera fija de 250 ms interrumpible por teclado y resize, sin calcular
vencimientos desde el reloj congelado ni producir redibujados continuos.

El reloj comienza con la interfaz, continúa al entrar o salir del campamento y
no se reinicia al consultar otra ficha ni redimensionar. Los efectos se
reconstruyen a partir de ese tiempo. Esperar en el campamento nunca adelanta la
fecha del Mundo.

### Compatibilidad con las animaciones actuales

Tiles y retratos reciben exclusivamente `paso_250_ms()`. Conservan sus fórmulas,
desfases por Actor y restricciones de duración en múltiplos de 250 ms. La
constante de esa unidad debe distinguirse del vencimiento fino de 50 ms: cambiar
la frecuencia del bucle no cambia el significado de un paso antiguo.

Las pruebas que asignan `fotograma` directamente pasan a controlar el reloj.
El contrato de [parpadeo](./13-retratos-por-capas.md#temporización) y los RON
existentes mantienen su comportamiento.

## 4. Recurso y compatibilidad

`assets/terminal/campamento.ron` conserva `fogata.arte` y `fogata.estilo` y agrega
`fogata.efecto: Option<ConfiguracionFogata>`. Omitir `efecto` equivale a `None`:
se dibuja exactamente la fogata estática anterior, sin iluminación ni chispas.
`Some` activa la representación procedural completa y sustituye ese dibujo.

El recurso oficial agrega este campo dentro de `fogata`:

```ron
efecto: Some((
    luz: (r: 255, g: 180, b: 72),
    intensidad_luz: 0.45,
    radio_luz: 28,
)),
```

| Campo de `ConfiguracionFogata` | Tipo y validación |
|---|---|
| `luz` | Bloque con `r`, `g`, `b` obligatorios, enteros entre 0 y 255. El DTO lee `u16` para diagnosticar cada canal fuera de rango y convierte a `[u8; 3]` validado. |
| `intensidad_luz` | `f32` finito, entre 0 y 1 inclusive. Cero desactiva el halo y conserva llama y chispas. |
| `radio_luz` | `u16`, entre 1 y 128 inclusive, en columnas horizontales. |

Los campos de la configuración son obligatorios si existe `Some`. Son inválidos
`radio_luz: 0`, `intensidad_luz: 1.1` y `luz: (r: 256, g: 180, b: 72)`.
Los errores identifican `campamento.ron`, `fogata.efecto` y el campo afectado.
La carga termina antes de abrir la terminal interactiva.

El arte y estilo anteriores siguen validándose, incluso con el efecto activo,
para conservar una alternativa estática utilizable. La normalización, paleta
nominal y validación de ancho actuales no se amplían. Los valores RGB nuevos
pertenecen solo a esta configuración visual.

## 5. Generación de la imagen

La imagen se calcula como función del instante, coordenadas locales y una
semilla cosmética constante del módulo. Dibujar dos veces el mismo instante
produce la misma imagen. No se consulta el reloj, el sistema de archivos ni
ningún generador de azar durante el dibujo.

El área lógica mide **12 columnas por 10 filas**, con origen superior izquierdo
`(0, 0)`. La base queda en la última fila. Sus dimensiones no cambian durante la
animación; el estilo artístico puede afinarse después sin cambiar el contrato
de reloj o composición.

### 5.1. Llama y brasas

Se usa un campo pequeño de intensidad muestreado directamente. Una función de
ruido suave `N(clave, t, periodo)` interpola entre valores en `[0, 1]` obtenidos
de un hash entero explícito y fijo de clave, semilla y número de intervalo.
La interpolación usa `s*s*(3-2*s)`; no depende de `DefaultHasher` ni de RNG.
Los números de intervalo y ciclo se calculan con enteros; solo la fracción o
edad local se convierte a punto flotante, para conservar precisión tras una
sesión larga.

Punto de partida para el campo, con `t` en milisegundos:

```text
para y = 1..7:
    h = (8 - y) / 7                       # altura sobre las brasas
    centro = 5 + h * (0.8 + 1.2 * (2*N(0,t,400)-1))
    ancho = (3.2*(1-h) + 0.45) * (0.85 + 0.3*N(y,t,200))
    perfil = max(0, 1 - abs(x-centro)/ancho)
    intensidad = clamp(perfil * (1-0.45*h) * (0.8+0.2*N(20+y,t,150)), 0, 1)
```

Las divisiones del campo son reales. La intensidad determina glifo y color:
menos de 0.12 es transparente; de 0.12 a 0.35 usa `░`; de 0.35 a 0.60 `▒`;
de 0.60 a 0.82 `▓`; el resto usa `█`. El frente interpola por canales entre
rojo oscuro `(120,24,0)`, naranja `(255,96,0)`, amarillo `(255,200,64)` y crema
`(255,244,192)`, en intensidades 0, 0.35, 0.70 y 1. El fondo hereda el lienzo,
según la [opción de fondo del contrato 16](./16-paleta-terminal.md#8-fondo-predeterminado-y-opción--b).

La fila 8 contiene brasas entre las columnas 3 y 7, con oscilación suave de
rojo a naranja. La fila 9 contiene leños entre las columnas 2 y 8, con `▄` y
tonos marrones. El resto de las celdas del efecto es transparente. La llama
oscila ligeramente hacia los lados y su sesgo superior es hacia la derecha.

### 5.2. Chispas

Se usan seis emisores calculados por ciclos de 1800 ms y desfases de 300 ms.
El hash de emisor y ciclo fija el punto de salida, duración entre 900 y 1400 ms
y pequeñas variaciones de velocidad. Cada chispa parte cerca de `(5, 7)`;
asciende entre 3 y 4 filas por segundo y deriva entre 1 y 2 columnas por segundo
hacia la derecha. Se dibuja en la celda más cercana mientras vive y está dentro
del área lógica; no se almacena una lista que crezca con el tiempo.

Su glifo pasa de `·` a `.` al envejecer y su frente pierde brillo de amarillo a
naranja oscuro. El fondo de la celda existente se conserva. Si coinciden varias
chispas se dibuja la más luminosa; se componen después de leños, brasas y llama.
Los caracteres utilizados deben ocupar una columna según `unicode-width`.

## 6. Iluminación y composición

### 6.1. Posición y orden

El alto del efecto activo es 10; con `None` sigue siendo el del arte estático.
Ese alto participa en el máximo que reserva la región de arte del
[contrato de Acampar, sección 8.1](../acciones/07-acampar.md#81-regiones-y-retratos).
Se conservan las seis filas mínimas para ficha, las barras y el mínimo 80×24.

El origen X sigue siendo el extremo completo de los retratos más una columna.
El origen Y es el borde inferior de la región del mundo menos el alto lógico.
Se usan coordenadas con signo antes del recorte; no se recoloca ni escala el
efecto cuando falta espacio. La fuente luminosa está en la coordenada local
`(5, 7)`, incluso cuando queda fuera de pantalla.

Orden en cada cuadro:

1. Limpiar el fondo y dibujar todas las capas de los retratos.
2. Aplicar la luz sobre las celdas ya dibujadas dentro de la región de arte.
3. Dibujar leños, brasas, llama y chispas.
4. Dibujar ficha, barras y, cuando corresponde, la confirmación global.

El efecto nunca escribe fuera de la intersección entre región de arte y buffer.
Su transparencia conserva la celda anterior. No se acumula luz entre cuadros:
cada uno comienza desde los colores originales de los retratos.

### 6.2. Cálculo de la luz

Para una celda a distancia `(dx, dy)` de la fuente:

```text
d = sqrt(dx*dx + (2*dy)*(2*dy))
caida = max(0, 1 - d/radio_luz)^2
oscilacion = 0.85 + 0.15*N(0,t,400)
a = intensidad_luz * caida * oscilacion
```

La corrección vertical aproxima las proporciones de una celda de terminal.
Con `a == 0` se conserva el estilo exacto. Con luz, frente y fondo se calculan
por separado; para cada canal de color original `C` y de luz `L`:

```text
C_nuevo = round(C + a * (255-C) * L/255)
```

El resultado se limita a `[0,255]` y se escribe como `Color::Rgb`. Un color negro
exacto se conserva negro: esto mantiene los fondos opacos de los ojos y evita
convertir todos los espacios negros en un rectángulo luminoso. Sí se iluminan
fondos coloreados. Se conservan símbolos y modificadores de cada celda.

La conversión a RGB usa `PaletaColores`, definida en el
[contrato 16](./16-paleta-terminal.md#2-argumentos-y-resolución). En modo nativo,
solo se tiñen colores consultados y RGB explícitos; cada canal desconocido se
conserva intacto. En modo fijo se usa la paleta compartida del frontend. La
fogata no contiene una tabla alternativa ni convierte por aproximación un color
nativo desconocido.

## 7. API interna y límites de coste

`recursos.rs` convierte el DTO opcional a `ConfiguracionFogata`, con campos
privados y acceso de lectura dentro del crate. La carga garantiza sus rangos.
`RecursosVisuales` expone la configuración y un método `alto_fogata()` que elige
entre altura estática y procedural.

`fogata.rs` expone dentro del crate la constante de dimensiones y:

```rust
fn dibujar(
    buffer: &mut ratatui::buffer::Buffer,
    recorte: ratatui::layout::Rect,
    origen: (i64, i64),
    instante_ms: u64,
    configuracion: &ConfiguracionFogata,
    paleta: &PaletaColores,
);
```

La llamada aplica luz y luego compone la imagen, según la sección 6. El camino
estático conserva el dibujo existente en `campamento.rs`. Muestreo, conversión
y mezcla se separan en funciones internas para probar sus contratos.

El trabajo por cuadro queda acotado por la región visible dentro del radio de
luz, 120 celdas de fuego y seis chispas. No depende del número de elementos del
Mundo ni del tiempo transcurrido. Se recorta antes de acceder al buffer; los
orígenes fuera de pantalla y las regiones vacías son válidos y no producen
errores ni `panic`. El efecto no necesita reservar memoria por cuadro.

## 8. Compatibilidad visual y errores

La versión animada envía RGB mediante el backend actual. Su apariencia completa
requiere una terminal con color de 24 bits y una fuente que dibuje los glifos
usados. Se conservan el ancho de una celda y el recurso estático seleccionable
mediante `efecto: None`. No se implementa detección automática ni un menú nuevo;
el [plan de diagnóstico pendiente](../../planes/pendientes/2026-10-15-09-diagnosticar-compatibilidad-unicode.md)
mantiene su alcance independiente.

Los errores de recursos siguen el diagnóstico previo a la entrada interactiva.
El efecto no introduce errores de sesión ni altera su tratamiento. Los errores
de E/S de terminal conservan la restauración mediante el guardián existente.

## 9. Verificación y aceptación

| Contrato | Verificación requerida |
|---|---|
| Tiempo y pausa | Reloj controlado: límites de 50/250 ms, fracciones, pausas, reanudación y demora larga sin ráfaga. |
| Duraciones anteriores | Mismos frames de tiles y estados de parpadeo en instantes equivalentes; RON previos aceptados. |
| Configuración | Carga oficial, campo ausente, `None`, valores límite y errores con contexto. |
| Generación | Mismo instante/semilla produce las mismas celdas; distintos instantes cambian la imagen; muestras válidas, RGB y glifos de una columna. |
| Luz | Caída con distancia, rango de canales, símbolos intactos, negro conservado y zonas sin luz idénticas. |
| Composición | TestBackend con retratos en 80×24 y tamaño mayor; ficha y barras iguales; llama/chispas a la derecha y recorte superior/derecho seguro. |
| Repetición y resize | Dos dibujos del mismo instante son idénticos; resize ida/vuelta conserva la muestra y no acumula luz. |
| Aislamiento | Esperar y redibujar no cambia fecha, fase, grupo ni resultados de una acción posterior con la misma semilla y entradas. |

La prueba de aislamiento compara dos sesiones equivalentes con diferentes
cantidades de actualizaciones visuales, incluido el cierre normal del campamento
y una acción posterior, para detectar consumo accidental de azar o tiempo del
juego. La prueba de ficha/barras compara los estilos y símbolos fuera de la
región de arte con el efecto activado y desactivado a igual estado de selección,
con retratos de al menos diez filas, como los oficiales de catorce. Con retratos
más bajos se comprueba por separado la reserva de altura de la sección 6.1:
activar la fogata puede aumentar la región de arte, conservando la lectura
completa de la ficha.

La revisión manual se hace al terminar el incremento en Manjaro: llama legible,
oscilación suave, chispas visibles, luz cálida sin perder contraste de caras,
navegación fluida, pausa, resize y salida normal. Se prueba también `None`.
Las pruebas automáticas fijan los contratos, sin exigir capturas exactas de cada
variación artística. Se aplican todos los controles de la metodología y build
de producción antes de cerrar el plan.

## 10. Decisiones pendientes

No hay preguntas bloqueantes para este alcance. El ajuste artístico final se
realiza en la revisión del incremento; cambiar las reglas de composición,
configuración o tiempo requiere actualizar este contrato.

## 11. Verificación de la implementación — 2026-10-26

Se implementaron `animacion.rs` y `fogata.rs` y su integración en los recursos,
la interfaz y el campamento. El recurso oficial activa la animación; los
anteriores y `efecto: None` conservan la alternativa estática.

Pasan los cuatro controles de la metodología y
`cargo build --workspace --release`: 115 pruebas, distribuidas en 31 de app,
46 de core, 34 de terminal y 4 del importador. Las ocho pruebas nuevas cubren
reloj, validación, muestreo, luz, recorte, composición y aislamiento del azar.
Las regresiones de tiles, retratos, fichas y viajes también pasan. Las pruebas
de carga validan contenido, Lua y recursos oficiales.

Se ejercitó el ejecutable release en PTY de 80×24 con RGB habilitado, tanto con
el recurso oficial como con `None` en una copia temporal de los recursos:
entrada al campamento, navegación horizontal/vertical, PageDown, Q cancelado,
resize a 120×40 y retorno, cierre del campamento y salida del programa. La
confirmación congeló la imagen y ambos procesos salieron con código 0 y
restauraron cursor y pantalla principal. La captura animada cambió 119 celdas
en la muestra de espera; el camino estático permaneció estable en esa muestra.
Se inspeccionaron capturas de ambos caminos con Noto Sans Mono para revisar
llama, chispas, colores, contraste y separación de ficha/barras. La valoración
artística en la terminal habitual del usuario queda abierta a refinamientos.
