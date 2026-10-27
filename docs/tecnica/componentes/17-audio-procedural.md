# Audio procedural del frontend

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.3 |
| Fecha | 2026-10-27 |

> Implementación y escucha de la fogata aceptadas por el usuario; el
> [plan 041](../../planes/completados/2026-10-26-41-incorporar-audio-fogata.md)
> registra su cierre. El [contrato 19](./19-paisajes-sonoros.md) amplía la
> selección de ambientes, las órdenes, la carga de recursos y la mezcla de las
> secciones 2–6. Las demás reglas de dispositivo, errores y compilación siguen
> vigentes. Las descripciones de un único efecto corresponden al incremento 041.
> El [contrato 20](./20-musica-campamento.md) añade la preparación de música,
> las solicitudes y su estado confirmado; amplía la carga y la sincronización
> de las secciones 3–6 sin cambiar el tratamiento de fallos de dispositivo.

## 1. Propósito y alcance

El frontend genera efectos de sonido con FunDSP y los reproduce mediante CPAL.
El primer efecto es el [crepitar de la fogata](./18-crepitar-fogata.md), audible
durante Acampar. Sus parámetros se cargan desde un recurso editable del frontend.

El audio está activado por defecto en la compilación habitual. Se puede jugar
sin sonido y compilar sin las bibliotecas de audio. Ausencia de dispositivo o
fallo de reproducción desactiva el sonido y permite continuar la partida.

Este incremento no incorpora música, MIDI, SoundFonts, grabaciones, selector de
dispositivo, menú de volumen, audio espacial ni un sistema general de eventos
sonoros. FunDSP queda disponible para una posterior prueba musical; esa prueba
y una posible integración de otro sintetizador tendrán su propio diseño.

Desarrolla la presentación opcional del
[diseño general](../../diseno-general.md#10-frontend-de-terminal), respetando
`terminal → app → core`. No modifica las reglas, la fecha ni el azar del juego.

## 2. Organización y dependencias

Todo el desarrollo pertenece a `crates/desert-tiger-terminal/src/audio/`:

| Módulo | Responsabilidad |
|---|---|
| `mod.rs` | Fachada, preparación, habilitación, estado y cierre. |
| `configuracion.rs` | Carga y validación del recurso definido en el contrato 18. |
| `motor.rs` | Estado de reproducción y transiciones de ganancia por muestra. |
| `dispositivo.rs` | Adaptador CPAL, formato de salida y comunicación con su callback. |
| `fogata.rs` | Generador FunDSP del contrato 18, sin dependencia de CPAL. |

La configuración y la fachada silenciosa existen también sin la feature
`audio`; los módulos de síntesis y dispositivo se compilan solo con ella.
`app` y `core` no reciben nuevas dependencias, mensajes ni tipos. `Interfaz`
solo comunica el ambiente sonoro deseado, sin conocer FunDSP, CPAL o muestras.
El generador tampoco conoce Ratatui, teclado, Partida ni coordenadas visuales.

Se mantienen un programa y un proceso. Simulación, entrada y dibujo conservan
su hilo actual. CPAL ejecuta el procesamiento sonoro en su callback de audio,
independiente del reloj visual. No se crean servicios, ejecutables auxiliares
ni un hilo adicional de planificación del juego.

Las [dependencias del componente 01](./01-workspace-entorno.md#dependencias-directas-aprobadas)
incluyen estas versiones exactas y opcionales, incorporadas a los manifiestos
y a `Cargo.lock`:

```toml
# [workspace.dependencies]
fundsp = { version = "=0.23.0", default-features = false, features = ["std"] }
cpal = { version = "=0.18.2", default-features = false }

# Cargo.toml de desert-tiger-terminal
[features]
default = ["audio"]
audio = ["dep:fundsp", "dep:cpal"]

[dependencies]
fundsp = { workspace = true, optional = true }
cpal = { workspace = true, optional = true }
```

FunDSP aporta síntesis y filtrado; se excluyen sus features de archivos y FFT.
CPAL aporta únicamente salida al dispositivo predeterminado; no se habilitan
ASIO, JACK, PipeWire, PulseAudio ni prioridad de tiempo real opcionales.
En Linux se usa su backend ALSA y se requieren sus archivos de desarrollo al
compilar con audio. El 2026-10-26, `pkg-config` encontró ALSA 1.2.16.1 en la
máquina de desarrollo. La implementación compiló con ese entorno sin instalar
paquetes adicionales de sistema.

## 3. Opciones y preparación

`--no-audio`, sin valor y como máximo una vez, deshabilita el audio. Se combina
en cualquier orden con `--content`, `--color-scheme` y `-B`. Un duplicado o valor
inesperado es un error de argumentos previo a raw mode.

```sh
# Compilación habitual, con audio disponible.
cargo run --release --bin desert-tiger-terminal

# Mismo ejecutable, reproducción deshabilitada.
cargo run --release --bin desert-tiger-terminal -- --no-audio

# Compilación sin FunDSP, CPAL ni sus dependencias de sistema.
cargo run --release --no-default-features --bin desert-tiger-terminal
```

`--no-audio` es válido
también en una compilación sin la feature. En cualquiera de esos dos casos se
omite la lectura de recursos sonoros y no se consulta ni abre un dispositivo.

Con audio habilitado se valida `assets/audio/fogata.ron` antes de activar raw
mode. `--content` no cambia esa ruta. Archivo ausente o inválido produce un
error de recursos con contexto y aborta ese inicio, como los recursos visuales;
`--no-audio` permite jugar sin utilizarlo. No hay recarga de audio en caliente.

## 4. Fachada y ambiente deseado

La API es privada del crate; los tipos de bibliotecas quedan encapsulados:

```rust
enum AmbienteSonoro { Silencio, Fogata }

fn preparar(habilitado: bool, raiz: &Path)
    -> Result<PreparacionAudio, String>;

impl ControlAudio {
    fn iniciar(preparacion: PreparacionAudio) -> Self;
    fn actualizar(&mut self, ambiente: AmbienteSonoro);
    fn tomar_aviso(&mut self) -> Option<AvisoAudio>;
    fn cerrar(&mut self);
}
```

`PreparacionAudio` contiene configuración validada o la decisión de deshabilitar.
`ControlAudio` posee el stream o una variante silenciosa. Un fallo de dispositivo
se conserva como aviso recuperable; no sale como error de sesión. `actualizar`
es idempotente: repetir un ambiente no reinicia el generador, no abre streams y
no duplica efectos. `tomar_aviso` entrega cada aviso una sola vez.

`Interfaz::ambiente_sonoro()` devuelve `Fogata` mientras la vista sea
`VistaSesion::Acampando` y el programa continúe; devuelve `Silencio` en los demás
casos, incluida salida confirmada o sesión fatal. Consultar otros Actores,
navegar fichas, paginar o redimensionar no cambia ese ambiente. Un cierre de
Acampar rechazado conserva `Fogata` porque la sesión sigue acampando.

El sonido no depende de `fogata.efecto`: la fogata estática también crepita.
La confirmación Q conserva el ambiente del campamento. El audio continúa aunque
el reloj visual esté congelado; no sincroniza chasquidos con chispas concretas.

El bucle crea `ControlAudio` una sola vez, después de inicializar la interfaz y
dibujar el primer cuadro. La preparación musical en segundo plano se rige por
el [contrato 20](./20-musica-campamento.md#2-datos-y-preparación).
Antes de esperar entrada y después de procesarla,
transmite el ambiente actual y recoge avisos. `dibujar` nunca inicia, cambia ni
detiene audio. Un cálculo de juego o un vencimiento visual tardío no interrumpe
el callback de sonido.

## 5. Dispositivo y estados

Estados observables del control: deshabilitado, activo o no disponible. El
control activo empieza en silencio y mantiene un único stream durante la sesión.
Un fallo lo lleva a no disponible; no reintenta automáticamente ni solicita
permisos o dispositivos de entrada.

Se utiliza el host y dispositivo de salida predeterminados de CPAL. Se elige
una configuración anunciada por el dispositivo con este orden:

1. Formato `f32`, luego `i16`, `i32` y `u16`.
2. Para cada formato, estéreo antes que mono.
3. Para cada combinación, frecuencia predeterminada del dispositivo, luego
   48000 Hz, luego 44100 Hz, cuando pertenezcan al rango anunciado.
4. Si ninguna coincide, la frecuencia del rango más cercana a 48000 Hz,
   limitada al intervalo admitido de 8000–192000 Hz.

Los empates se resuelven ordenando por frecuencia. Se solicita buffer
predeterminado y un timeout de creación de 500 ms a CPAL. Se rechazan formatos
o cantidades de canales diferentes en este alcance; se informa configuración
no compatible y el juego continúa sin sonido.

El procesador produce mono `f32`: en estéreo se duplica en ambos canales. El
adaptador convierte a la representación del dispositivo usando la conversión
de muestras de CPAL y escribe todos los elementos del buffer. No reproduce un
archivo ni utiliza un sintetizador instalado por el usuario.

## 6. Tiempo, transiciones y callback

El procesador cuenta cuadros de audio a la frecuencia real del stream. No usa
`Instant`, el contador de Ratatui, días del Mundo ni pasos de la Partida.
En una misma plataforma y versión, la misma configuración, semilla y secuencia
de comandos aplicada en los mismos cuadros produce la misma señal,
independientemente del tamaño de los buffers.

Entrar y salir de `Fogata` aplica las rampas de ganancia del contrato 18, desde
la ganancia actual hasta uno o cero respectivamente. La interpolación es lineal
durante `ceil(duracion_ms * frecuencia_muestreo / 1000)` cuadros; al completar
la rampa se fija exactamente su destino. Una inversión durante la rampa
continúa suavemente desde ese punto usando la duración de la nueva transición.
Al llegar a silencio exacto se escriben ceros y se suspende el avance
del generador hasta reactivarlo, conservando su estado. No se repiten grabaciones
ni se reinicia una secuencia en cada renderizado.

El callback posee el procesador y sus nodos; se construyen, ajustan a la
frecuencia, inicializan y preasignan antes de `Stream::play`. En el callback:

- no se reserva memoria, crea nodos, lee archivos, imprime ni consulta la sesión;
- no se adquieren mutex, esperan canales ni realizan llamadas al dispositivo;
- se consulta una orden atómica al principio del buffer y se procesa una
  cantidad de trabajo acotada por cuadro;
- una muestra no finita se sustituye por cero y se comunica un fallo para
  desactivar la reproducción.

La comunicación usa `Arc<AtomicU8>` para la orden (silencio, fogata, cerrar), el
primer código de fallo y la confirmación de silencio. Publicación `Release`,
lectura `Acquire`; un fallo se registra una sola vez. Los detalles de diagnóstico
se formatean en el hilo de interfaz. No hay un canal que pueda acumular comandos.

`cerrar` es idempotente. Solicita una rampa final de 20 ms y espera su confirmación
como máximo 150 ms antes de liberar el stream. Si el control no tiene stream no
espera. `Drop` ejecuta el mismo cierre para cubrir errores del bucle y conserva
la restauración de terminal existente. La espera acotada solo se permite al
terminar el programa; las transiciones entre pantallas no bloquean la interfaz.

## 7. Errores y diagnóstico

| Categoría | Tratamiento |
|---|---|
| Argumentos o recurso inválido | Error contextual antes de raw mode; no inicia una sesión interactiva parcialmente configurada. |
| Sin dispositivo, consulta de configuración, formato incompatible, creación o arranque fallido | Control no disponible; aviso único y partida jugable sin sonido. |
| Error del stream o muestra no finita | El callback comunica un código; el hilo principal libera el stream y continúa en silencio. |
| Audio deshabilitado explícitamente o al compilar | Estado normal, sin avisos ni consultas al dispositivo. |

`AvisoAudio` identifica la etapa y contiene texto breve. No se escribe en stdout
ni stderr mientras la terminal está en modo interactivo. La interfaz conserva
el aviso hasta poder mostrarlo en la barra contextual del mundo o campamento:
`Audio no disponible. El juego continúa sin sonido.`. Usa el rol `secundario`;
errores de juego, decisiones y confirmación de salida tienen prioridad. El aviso
mostrado se retira con la próxima tecla; no añade una pantalla de aceptación.
Los avisos pendientes no se pierden por teclas recibidas mientras están ocultos.

No se reintenta al volver a acampar. Reconectar o elegir otro dispositivo exige
reiniciar el juego en este alcance. Un fallo sonoro no provoca `FatalSesion` ni
impide el cierre normal de Acampar.

## 8. Verificación y aceptación

| Contrato | Verificación |
|---|---|
| Configuración y CLI | Recurso oficial y casos del contrato 18; `--no-audio` combinado con opciones actuales y duplicados. |
| Contexto | Dobles de audio: apertura, navegación, Q cancelado, cierre rechazado/aceptado, tamaño insuficiente y fatal; una sola transición por cambio real. |
| Continuidad | Muestreo offline con comandos por cuadro: rampas, silencio exacto, reentrada durante salida e independencia del tamaño de buffer. |
| Dispositivo | Adaptador simulado para ausencia, formatos, frecuencia, fallo al abrir/arrancar y desconexión; sin depender de parlantes en CI. |
| Callback | Revisión de preasignación y ausencia de bloqueos/E/S; voces limitadas; fallo no finito comunica estado sin `panic`. |
| Aislamiento | Dos sesiones iguales, con/sin actualizaciones sonoras, cierran Acampar y realizan el mismo viaje; vistas y resultados coinciden. |
| Compilación | Controles generales con features predeterminadas y con `--no-default-features`; build release en ambas variantes. |

Las pruebas automáticas nunca abren un dispositivo real. En la revisión manual
se escucha el contrato 18 mientras se navega, redimensiona, abre/cancela Q, sale
y vuelve a entrar al campamento. Se prueba `--no-audio`, cierre del programa y
una salida no disponible. Se confirma restauración de terminal en todos los
casos. La calidad del timbre se evalúa escuchando; RMS o muestras finitas no la
sustituyen.

## 9. Fuentes y decisiones aplazadas

- [FunDSP 0.23.0](https://docs.rs/crate/fundsp/0.23.0): generadores, filtros y
  features mínimas; su [AudioUnit](https://github.com/SamiPerttu/fundsp/blob/master/src/audiounit.rs)
  permite fijar frecuencia y preasignar los recursos del grafo.
- [CPAL 0.18.2](https://docs.rs/cpal/0.18.2/cpal/): dispositivo, configuraciones,
  callback, conversión y errores de stream.

No quedan preguntas bloqueantes para este incremento. La mezcla de varios
ambientes, música con FunDSP, MIDI, otro sintetizador, notificaciones de cambio
de dispositivo y controles de audio en pantalla quedan aplazados.

## 10. Verificación de la implementación — 2026-10-26

El módulo `audio/` implementa los cinco módulos de la sección 2. Los errores de
recurso son textos privados con ruta y campo; los fallos de reproducción se
conservan como `AvisoAudio`. Las dependencias y la CLI están integradas; `app`,
`core`, las reglas y los recursos visuales no cambiaron.

- Pasan fmt, check, clippy sin advertencias y pruebas de workspace, tanto con
  features predeterminadas como con `--no-default-features`.
- Pasan 146 pruebas con audio y 135 sin audio. Las compilaciones release de
  ambas variantes terminan correctamente. El árbol sin features no incorpora
  FunDSP, CPAL ni ALSA.
- Un recorrido del ejecutable en PTY utilizó el backend ALSA de archivo/null
  para capturar la salida real de CPAL. Se comprobaron muestras no nulas,
  finitas y acotadas, duplicación estéreo, navegación, Q cancelado, resize,
  cierre y reapertura de Acampar, y restauración de terminal.
- Otros recorridos comprobaron `--no-audio`, una salida incompatible y una
  compilación sin audio. Todos finalizaron con código cero y terminal
  restaurada; solo el escenario incompatible presentó el aviso.
- Un quinto recorrido provocó un fatal de calendario al cerrar Acampar con
  audio activo: terminó con código uno y restauró los atributos de terminal.

La captura verifica la integración sin usar los parlantes. El usuario completó
la escucha jugando y aceptó el resultado antes de solicitar los paisajes del
contrato 19; esa valoración no se atribuye a las pruebas numéricas.
