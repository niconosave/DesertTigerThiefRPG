# Crepitar procedural de la fogata

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.2 |
| Fecha | 2026-10-26 |

> El generador y el recurso están implementados, las pruebas pasan y el usuario
> aceptó la revisión auditiva. El [plan 041](../../planes/completados/2026-10-26-41-incorporar-audio-fogata.md)
> registra su cierre. La mezcla con el entorno pertenece al [contrato 19](./19-paisajes-sonoros.md).

## 1. Propósito y límites

El campamento presenta un fuego cercano y moderado: un fondo continuo suave,
pequeños chasquidos irregulares y crujidos menos frecuentes. El efecto se genera
con FunDSP y parámetros editables; no contiene una grabación en bucle.

Este contrato define el timbre, los datos y el generador. Habilitación, salida,
reloj, transiciones de pantalla, errores de dispositivo y compilación opcional
pertenecen al [contrato 17](./17-audio-procedural.md). No duplica esas decisiones.

No representa el combustible, tamaño o estado reglado de una fogata. No utiliza
las coordenadas de los personajes ni cambia por una chispa visual concreta.
Las intensidades y frecuencias son parámetros de presentación, sin consecuencias
de juego. No se agrega inventario, consumo de leña ni una acción particular.

## 2. Recurso editable

El archivo `assets/audio/fogata.ron` pertenece al frontend y contiene una única
definición, cargada al iniciar. Ejemplo válido y punto de partida artístico:

```ron
(
    volumen: 0.25,
    nivel_brasa: 0.18,
    chasquidos_por_segundo: 8.0,
    crujidos_por_segundo: 0.6,
    entrada_ms: 200,
    salida_ms: 250,
)
```

| Campo | Regla |
|---|---|
| `volumen` | `f32` finito, entre 0 y 1; ganancia total del efecto. |
| `nivel_brasa` | `f32` finito, entre 0 y 1; nivel del fondo continuo antes de la mezcla. |
| `chasquidos_por_segundo` | `f32` finito, entre 0 y 40; frecuencia media de transitorios pequeños. Cero desactiva esa familia. |
| `crujidos_por_segundo` | `f32` finito, entre 0 y 5; frecuencia media de transitorios mayores. Cero desactiva esa familia. |
| `entrada_ms`, `salida_ms` | `u32`, entre 5 y 2000 inclusive; rampas de activación y desactivación. |

Todos los campos son obligatorios. Se rechazan ausentes, duplicados y desconocidos.
`NaN`, infinitos, `volumen: 1.1`, tasas negativas y `entrada_ms: 0` son inválidos.
El diagnóstico identifica ruta y campo. El volumen cero es válido y produce
silencio exacto. La variación de estos valores no exige recompilar ni actualizar
una prueba que copie los números del recurso oficial.

`ConfiguracionFogataAudio` es un valor validado, con campos privados y acceso
de lectura para los módulos de audio. El DTO de `configuracion.rs` es privado.
Las pruebas de rangos usan fixtures propios; otra prueba carga el recurso
oficial sin imponer su configuración artística exacta.

## 3. Generación del sonido

El generador mezcla tres componentes mono. Todos los ruidos y filtros son nodos
FunDSP construidos antes de comenzar la reproducción. Los valores siguientes
definen el primer timbre; los parámetros de la sección 2 controlan su densidad
y nivel sin cambiar la topología.

### Fondo de brasas

Ruido rosa filtrado con paso alto de 80 Hz y paso bajo de 1500 Hz. Se multiplica
por `nivel_brasa` y por una modulación suave entre 0.7 y 1, interpolada mediante
`s*s*(3-2*s)` entre valores aleatorios cada 500 ms. Evita un siseo completamente
uniforme y deja espacio a los chasquidos.

### Chasquidos

Ruido blanco filtrado entre 1200 y 6000 Hz. Cada evento tiene ataque lineal de
1 ms, caída exponencial hasta cero entre 15 y 60 ms y amplitud aleatoria entre
0.2 y 0.6. La envolvente llega exactamente a cero al terminar; se retira su voz.
Hay seis voces preasignadas de esta familia.

### Crujidos

Ruido blanco filtrado entre 250 y 2800 Hz. Ataque de 2 ms, caída exponencial
entre 40 y 120 ms y amplitud entre 0.5 y 0.9. Hay dos voces preasignadas.
La envolvente también termina en cero exacto. Los crujidos son golpes breves
de madera, sin una explosión grave dominante.

Las caídas usan `exp(-6*u) * (1-u)`, para `u` entre cero y uno después del ataque.
La duración sorteada corresponde a la caída y el ataque se suma a ella. Las
frecuencias superiores se limitan a `0.45 * frecuencia_muestreo`; así el timbre
se adapta a las frecuencias de dispositivo admitidas por el contrato 17.

### Distribución temporal y mezcla

Cada familia tiene su propio próximo evento. La espera entre eventos se calcula
como `max(1, ceil(-ln(u) / tasa * frecuencia_muestreo))` cuadros, con `u` uniforme
estrictamente entre cero y uno. No se utiliza un intervalo periódico fijo.
Con tasa cero no se calcula el logaritmo ni se agenda un evento de esa familia.

Las esperas se guardan como contadores descendentes `u64`; una conversión fuera
de rango satura a `u64::MAX`. El índice total de muestras también usa suma
saturada y no gobierna las agendas. Se evitan sumas de fechas absolutas que
puedan desbordar al editar una tasa positiva extremadamente pequeña.

Se usa un generador SplitMix64 local, con semilla explícita de constructor y
secuencias separadas para agenda, amplitudes y ruido. Los nodos FunDSP reciben
semillas propias derivadas de ella. La semilla inicial del programa es una
constante cosmética; no se consulta el RNG ni la semilla de la Partida.

Cuando aparece un evento se utiliza una voz libre de su familia. Si todas están
ocupadas, se omite ese transitorio y se agenda el siguiente normalmente. Nunca
se reserva otra voz ni se corta una que todavía está sonando. Las ocho fuentes
de ruido mantienen su estado sin reconstruir o resetear nodos en el callback.

La muestra de mezcla es:

```text
mezcla = fondo_modulado + suma_de_transitorios
salida = ganancia_transicion * volumen * 0.8 * tanh(mezcla)
```

La ganancia de transición pertenece al motor del contrato 17. El resultado es
finito y acotado a `[-0.8, 0.8]`; no debe recortarse digitalmente por sumar voces.
El estéreo inicial duplica este resultado según el adaptador, sin simulación
de ubicación ni dependencia de la posición del fuego ASCII.

## 4. API y estado

`audio/fogata.rs` contiene:

```rust
impl GeneradorFogata {
    fn nuevo(config: ConfiguracionFogataAudio, frecuencia: u32, semilla: u64)
        -> Result<Self, String>;
    fn siguiente_muestra(&mut self) -> f32;
}
```

El constructor valida la frecuencia aceptada por el contrato 17 y prepara
ruidos, filtros, contadores y ocho voces. La memoria y el coste por muestra
son acotados; la duración de un campamento no aumenta ninguna colección.
Las ramas de voz inactiva producen cero y no asignan memoria. Un generador
puede renderizarse offline sin CPAL para pruebas y escucha de sus muestras.

`siguiente_muestra` avanza exactamente un cuadro mono. El estado contiene
índice de muestra, temporizadores de eventos, modulación, generadores de ruido
y voces. No recibe vistas del juego, tamaño de terminal ni tiempo de dibujo.
El motor suspende estas llamadas al completar la transición a silencio y las
reanuda desde el mismo estado al volver al campamento.

## 5. Verificación y aceptación

- Carga del ejemplo, recurso oficial y validación de todos los límites de la
  sección 2; ausencia o deshabilitación según el contrato 17.
- Render de varias decenas de segundos a 44100 y 48000 Hz: muestras finitas,
  amplitud dentro del rango, sonido no nulo con una fixture audible y silencio
  exacto con volumen cero y con todas las fuentes desactivadas.
- Igual semilla/configuración produce el mismo resultado en una misma
  plataforma y versión. Semillas diferentes cambian la señal. No se requiere
  identidad binaria entre arquitecturas de punto flotante diferentes.
- Dividir el mismo render en bloques de 1, 64, 256 o 1024 cuadros conserva la
  secuencia; los eventos no dependen del tamaño del callback.
- Tasas cero, máximas y pool ocupado no fallan ni crean más de ocho voces.
  Las envolventes concluyen en cero y los contadores no desbordan en pruebas
  prolongadas con valores controlados.
- La escucha manual confirma crepitación irregular, fondo moderado, transitorios
  distinguibles y ausencia de un ciclo corto evidente, cortes al navegar o
  golpes de activación ajenos al timbre. Se ajusta el RON antes de ampliar el
  motor o agregar más sonidos.

## 6. Decisiones cerradas y extensiones

El primer efecto se escucha al acampar, incluidas las fichas externas y la
confirmación de salida. La conducta exacta de contexto y reproducción se define
solo en el [contrato 17](./17-audio-procedural.md#4-fachada-y-ambiente-deseado).
El acabado artístico se evalúa sobre una versión jugable. No hay preguntas
bloqueantes para este alcance; nuevos ambientes y música se diseñarán después
de escuchar y ajustar esta primera fogata.

## 7. Verificación de la implementación — 2026-10-26

El recurso oficial carga y valida. Las pruebas offline comprueban señal finita
y acotada a 8000, 44100, 48000 y 192000 Hz; reproducibilidad, semillas distintas,
silencio exacto, tasas extremas, voces ocupadas y saturación de contadores.
Otro render compara veinte segundos de señal repartida en bloques de distintos
tamaños. El motor verifica transiciones, inversión, cierre y suspensión del
generador al quedar en silencio.

La [verificación de integración](./17-audio-procedural.md#10-verificación-de-la-implementación--2026-10-26)
comprueba la reproducción a través de CPAL. El recurso conserva los valores
iniciales del diseño. El usuario escuchó el crepitar jugando y aceptó el
resultado antes de solicitar nuevos ambientes.
