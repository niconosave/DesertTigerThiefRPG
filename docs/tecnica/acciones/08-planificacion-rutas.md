# Viajar — Propuestas de caminos y previsualización

| Campo | Valor |
|---|---|
| Estado | Implementado; revisión de jugabilidad del usuario pendiente |
| Versión | 1.1 |
| Fecha | 2026-11-03 |

## 1. Propósito y alcance

Antes de un viaje largo, los integrantes del Grupo proponen caminos según su
experiencia. El jugador compara recorridos y tiempos, elige uno, improvisa o
cancela. Un desplazamiento a un vecino directo comienza sin esta deliberación.

Este documento define las reglas del incremento y su traducción técnica.
Sustituye la planificación de las secciones 3.1 y 8.1 del
[reglamento](./borradores/05-viajar-redisenio.md) y los apartados incompatibles
de las secciones 3, 6.1–6.2, 9 y 11 del
[contrato de Viajar](./06-implementacion-viajar.md). Amplía además la
[vista terminal](../componentes/09-frontend-terminal.md). Los apartados
sustituidos remiten a este contrato.

Se conservan destinos permitidos, preparación, medios, consumo, desplazamientos,
incidentes y cierre. El alcance inicial comprende propuestas por lugares
conocidos, montaña y caballo. No habilita desplazamientos por `Bloquea` ni cambia
los tiempos montañosos.

## 2. Reglas del juego

### 2.1. Viaje a un vecino directo

Después de validar el destino, si pertenece a
`Mundo::lugares_conectados(origen)`, la ruta será exactamente `[origen, destino]`.
No se busca la rápida, no se construye la larga, no se generan propuestas ni se
presenta su decisión. Tampoco se cobra el día de planificación. Se pasa a la
preparación habitual, que conserva su duración, chequeos y decisiones.

La excepción representa una sola arista `Conectado`. Dos Lugares visibles unidos
mediante un Lugar `oculto` siguen siendo dos conexiones y mantienen la
planificación. `Abarca`, `Bloquea` y las relaciones fluviales no son conexiones.

### 2.2. Referencia temporal y límite

Para destinos no adyacentes se fija un contexto de planificación: viajeros
actuales, suficientes caballos y camellos para todos y barco disponible según
las reglas vigentes, sin animales de carga ni sobrecarga. Se aplican terreno,
tamaño y estados actuales mediante el evaluador ordinario de duración. No se
anticipan incidentes ni tiradas.

`T(ruta)` suma las horas de traslado de sus conexiones con ese mismo contexto.
No incluye planificación ni aprovisionamiento. Se presenta como **tiempo de
traslado previsto, sin carga ni imprevistos**; el tiempo real puede variar.

| Referencia interna | Función |
|---|---|
| Rápida | Menor `T` entre origen y destino; fija `T_min`. |
| Improvisada | Constructor actual de ruta larga, partiendo de la rápida; fija `T_max` y el recorrido utilizado al improvisar. |

La larga se genera una sola vez. Si `C` cuenta los Lugares de la rápida, intenta
una cantidad entre `C + floor(C/4) + 1` y `2C − 1`, sin repetir Lugares; si no
lo consigue, conserva la mejor ampliación encontrada o la rápida. No representa el camino más largo posible del
grafo; conserva la posibilidad actual de coincidir con la rápida si no encuentra
un desvío. En ese caso `T_max = T_min`.

Toda propuesta cumple **`T_min ≤ T(propuesta) ≤ T_max`**. Se admite igualdad:
se descarta lo que supere el techo, sin inventar vueltas ni ampliar ese techo
para producir más opciones. Las rutas idénticas se agrupan.

### 2.3. Propuestas de los actores

Se examinan todos los viajeros, incluido el dirigente. Un Actor puede proponer
varios caminos; varios Actores pueden respaldar el mismo.

| Perfil | Habilitación y preferencia |
|---|---|
| Rápida | `viajero`, `explorador`, o `Conoce` al menos `ceil(N/2)` de los `N` Lugares del recorrido, incluidos los extremos. Conserva el requisito actual. |
| Conocidos | Favorece Lugares que ese Actor `Conoce`; el candidato debe contener al menos uno intermedio conocido. |
| Montaña | Requiere `montaraz`; favorece Lugares `montanoso`. El candidato debe contener al menos uno intermedio montañoso. |
| Caballo | Requiere `jinete`; favorece mitades donde la conexión elige `Medio::Caballo` y el Lugar no es `montanoso`, `boscoso` ni `pantanoso`. El candidato debe contener al menos un Lugar intermedio favorable. |

Intermedio significa distinto de origen y destino. Conocer solo los extremos no
basta para el perfil Conocidos, aunque puede alcanzar el umbral de la rápida.
Para Caballo se calcula el medio de la conexión completa: un extremo acuático
o desierto vasto puede impedirlo aunque el otro parezca favorable.

Una propuesta con tiempo `T_min`, aunque proceda de un perfil especializado,
requiere que su proponente cumpla también la habilitación de Rápida para ese
recorrido. Así los perfiles nuevos no eluden el requisito confirmado para ofrecer
el camino óptimo.

Las preferencias orientan la búsqueda sin restringir los nodos transitables:
pueden atravesarse Lugares desconocidos o de otros terrenos. No se concede
`Conoce` al proponer ni al elegir. `montaraz` permite proponer; todavía no altera
velocidad, chequeos ni accesos.

Si no se encuentra una propuesta relevante dentro del límite, no se ofrece una
por ese motivo. No se garantiza una alternativa distinta por Actor o rasgo.

### 2.4. Decisión y tiempo

Para destinos no adyacentes siempre hay una decisión, incluso sin propuestas:

1. Una opción por cada recorrido propuesto, con sus proponentes y motivos.
2. **Improvisar el camino:** adopta la ruta improvisada fijada al inicio.
3. **Cancelar el viaje:** termina sin partir.

Improvisar conserva el recorrido oculto: no ofrece previsión ni puntos en el
mapa. Puede coincidir con un recorrido propuesto; representa no adoptar el plan.
Sin propuestas se ofrecen improvisar y cancelar, sin partida automática.

Se conservan las **24 horas de planificación** para estos viajes, cualquiera sea
la elección, incluida cancelar. Se confirman una sola vez al responder, antes de
tener reservas y sin consumo. Consultar, mover el foco, desplazar la cámara o
abrir Detalles no consume tiempo ni azar.

## 3. Algoritmo de propuestas

### 3.1. Coste común y búsquedas

`rutas.rs` tiene un evaluador puro de conexión con las horas ordinarias de sus
dos mitades. Lo comparten la rápida, el cálculo de `T`, los límites y la previsión,
reutilizando `medio` y `mitad`. No habrá tablas de duración paralelas.

`rapida()` y `tiempos()` reciben el mismo contexto explícito, evitando
comparar costes distintos.

Cada perfil habilitado, salvo Rápida, realiza dos búsquedas Dijkstra:

| Intensidad | Peso de mitad favorecida | Peso de otra mitad |
|---|---|---|
| Moderada | `3 × horas` | `4 × horas` |
| Marcada | `1 × horas` | `4 × horas` |

Una mitad es favorecida según la sección 2.3. Todos los pesos son enteros
positivos. Los empates se resuelven por menor tiempo ordinario y después por
menor `IdLugar` del predecesor inmediato; los vecinos se recorren por ID y la
cola desempata por ID del nodo. Estas búsquedas no consumen azar.

**El descuento solo orienta la búsqueda:** nunca reduce las horas mostradas o
reales, las provisiones ni el tiempo usado para admitir una propuesta. Los dos
niveles son parámetros de esta primera versión, no características RON nuevas.

### 3.2. Validación, agrupación y orden

Cada candidato empieza en el origen solicitado, termina en el destino solicitado,
usa solo conexiones válidas, no repite Lugares y satisface habilitación,
relevancia y límite temporal. Se descartan candidatos irrelevantes o demasiado
lentos. Un recorrido inconexo o con extremos incorrectos es un fallo interno del
generador; no se adopta silenciosamente.

Se agrupan por la secuencia completa y ordenada de `IdLugar`, conservando todos
los respaldos `(Actor, perfil)` sin duplicarlos. Dos rutas de igual tiempo pero
distinto recorrido siguen siendo opciones distintas. Se ordenan por `T` y luego
por secuencia de IDs; respaldos por Actor y perfil. Los IDs de propuesta se
asignan después y permanecen estables durante la decisión.

La rápida conserva su desempate mediante `FuenteAzar` y la larga sus sorteos
actuales. Ambas se fijan antes de las propuestas y nunca se regeneran al
consultar o confirmar. Para Rápida se conservan todos los proponentes habilitados,
sin sortear cuál habla.

### 3.3. Complejidad y límites

No se enumeran todos los caminos simples. Cada perfil genera como máximo dos
candidatos; pueden existir alternativas válidas que estas búsquedas no encuentren.
Se reutilizan búsquedas de montaña y caballo entre Actores; para Conocidos se
reutilizan cuando coincide su conjunto ordenado de `Conoce`.

Con `P` perfiles distintos, las búsquedas nuevas cuestan aproximadamente
`2P × O((V + E) log V)`, además del constructor largo existente. Se usan los
índices de conexiones y conocimiento. No hay búsquedas durante el renderizado
ni un límite arbitrario de opciones que oculte propuestas ya generadas.

## 4. Motor y aplicación

### 4.1. Datos y opciones

En `core::acciones::viajar::registros`, reexportados como el resto del protocolo:

```rust
pub struct IdPropuestaRuta(u32); // Copy, Eq; único dentro de IdPaso
pub enum PerfilRuta { Rapida, Conocidos, Montana, Caballo }
pub struct RespaldoRuta {
    pub actor: IdActor,
    pub perfil: PerfilRuta,
    pub caracteristicas_habilitantes: Vec<Caracteristica>,
    pub lugares_relevantes: Vec<IdLugar>,
}
pub struct PropuestaRuta {
    pub id: IdPropuestaRuta,
    pub ruta: Vec<IdLugar>,
    pub previsto: TiempoAccion,
    pub respaldos: Vec<RespaldoRuta>,
}
```

En perfiles especializados, `lugares_relevantes` contiene los intermedios
favorecidos en orden de recorrido y sin duplicados. En Rápida contiene los
Lugares conocidos de todo el recorrido, aunque además se habilite por rasgo.
`caracteristicas_habilitantes` conserva los rasgos aplicables: `viajero` y/o
`explorador` para Rápida, `montaraz` para Montaña, `jinete` para Caballo y ninguno
para Conocidos. Así no se pierde el motivo cuando cumple varios requisitos.
La aplicación redacta los motivos desde estos hechos; el núcleo no usa nombres,
coordenadas, colores o texto narrativo.

`ContenidoEscena::Plan` pasa a contener `propuestas: Vec<PropuestaRuta>`,
sustituyendo los campos del proponente único. `OpcionViaje` sustituye
`AceptarPlan`/`OtraRuta` por `ElegirRuta(IdPropuestaRuta)`/`Improvisar`; conserva
`Cancelar` y las opciones de otras decisiones. El frontend devuelve una opción
publicada del paso vigente, nunca una ruta arbitraria.

### 4.2. Estado y transiciones

`EjecucionAccion` almacena propuestas y ruta improvisada inmutables durante
`ElegirRuta`, además de la ruta adoptada. Rápida y límite son referencias de
generación, no la antigua elección binaria.

| Caso | Transición |
|---|---|
| Vecino | Validación → ruta directa → `Preparar`. |
| Otro destino | Validación → generación → `ElegirRuta`. |
| Elegir propuesta | Sumar planificación → copiar su ruta → `Preparar`. |
| Improvisar | Sumar planificación → copiar improvisada → `Preparar`. |
| Cancelar | Sumar planificación → `Terminar(Cancelado)`. |

Propuesta aceptada y ruta directa fijan `plan_visible = true`; improvisar lo deja
en `false`. Esto no concede `Conoce`. Los cierres por incidentes conservan sus
reglas; aceptar un plan garantiza el itinerario adoptado, no llegar sin problemas.

`VIA-RUT-02` identifica la decisión con cero o más propuestas. `VIA-RUT-01`
deja de emitirse al desaparecer la partida automática sin plan. Jugador y NPC
usan este protocolo; los NPC conservan su elección aleatoria provisional entre
las opciones publicadas.

### 4.3. Vista pública y narración

Se conservan `VistaSesion::EsperandoDecision` y
`IntencionFrontend::ResponderDecision`. Se añaden:

- `PresentacionDecision::{Vineta, PlanificacionRutas}` en `VistaDecision`;
- `ruta: Option<VistaRutaPropuesta>` en `VistaOpcionDecision`;
- `VistaRutaPropuesta` con lugares ordenados, tiempo previsto formateado,
  nombres de proponentes sin duplicados y respaldos redactados.

Solo las opciones de propuesta tienen `ruta: Some(...)`. La aplicación relaciona
sus IDs opacos de opción con las opciones del motor y proyecta los Lugares
necesarios para resolver sus centros, incluidos los `oculto`.

La deliberación se redacta como un párrafo en segunda persona compuesto según
la situación: detención en el Lugar actual, objetivo del viaje, conversación con
los compañeros si los hay y propuestas con Actor, motivos y una referencia del
recorrido. Si viaja solo no se inventan interlocutores; sin propuestas se explica
que no se logra idear un itinerario fiable y que partir exigiría improvisar.
El cierre sitúa el día de deliberación. No se adelanta una ruta ya elegida.

Esta viñeta puede superar las tres oraciones para conservar las intervenciones;
se pagina sin recortar propuestas. Las demás viñetas conservan su jerarquía
narrativa vigente. Los datos de tiempo, recorrido y respaldos completos siguen
en Detalles. Un Lugar sin nombre usa la denominación narrativa de reserva.

## 5. Selección y previsualización terminal

### 5.1. Composición y controles

La presentación tiene dos momentos locales, sin cambiar la decisión del motor:

1. **Deliberación:** ventana de viñeta sobre el mapa completo con el párrafo
   narrativo y la ilustración de escena habitual. El mapa se centra en el Lugar
   actual; no se dibujan todavía rutas ni números. Enter recorre sus páginas y,
   al terminar, abre la elección sin enviar una respuesta a la aplicación.
2. **Elección:** desaparece la ventana y se usa todo el ancho del mapa. La barra
   contextual muestra posición de la opción, nombre, previsión y proponentes;
   la barra de acciones permite pasar por las alternativas y confirmar. No hay
   panel lateral. La ruta enfocada se previsualiza con puntos y números.

Se conservan retratos y ambas barras. El arte de paisaje y las conexiones de
modo de desarrollo se suspenden durante ambos momentos. Las otras decisiones
no cambian de formato.

| Entrada | Deliberación | Elección |
|---|---|---|
| `Enter` | Página siguiente; al terminar, abrir elección. | Confirmar la opción enfocada. |
| `← / →` | Página anterior/siguiente. | Considerar opción anterior/siguiente. |
| `↑ / ↓` | Sin efecto sobre el foco. | También cambia de opción. |
| `D` | Consultar Detalles. | Detalles de la opción enfocada. |
| `Esc` | No confirma ni cancela el viaje. | Enfocar cancelar; Enter confirma. |
| `Shift + flechas` | Sin desplazamiento de cámara. | Desplazar cámara un tile. |
| `Home` | Sin efecto. | Recuperar encuadre automático. |

En Detalles, `↑/↓` o `PageUp/PageDown` recorren texto; `D/Esc` vuelve conservando
momento, foco, página narrativa y cámara. Enter no confirma ninguna ruta.
`Q` mantiene la confirmación de salida. Por debajo de 80 × 24 se mantiene el
aviso existente sin confirmar ni perder la decisión.

`EstadoPlanificacion::eligiendo` comienza en `false` y se activa al asentir en
la última página. Se reinicia junto con el foco ante otro `IdDecisionVista`.
El foco inicial es la primera propuesta o improvisar si no hay ninguna.
Leer, asentir, cambiar foco y consultar Detalles no envía intenciones ni consume
tiempo o azar; solo Enter durante la elección responde con el `IdOpcionVista`.
El día de planificación sigue confirmándose una única vez en el motor.

### 5.2. Encuadre y puntos

La propuesta enfocada une con puntos ASCII `.` los centros de cada par de
Lugares consecutivos. Se rasteriza en coordenadas globales con Bresenham y luego
se recorta al viewport, extrayendo el trazado de desarrollo a una utilidad común.
Se dibuja únicamente esta ruta, incluidos los nodos ocultos que atraviese.

Al cambiar de opción o redimensionar se centra el rectángulo de sus centros, a
la escala actual y con los límites habituales de cámara. Si no cabe completa,
se recorta y puede inspeccionarse desplazando la cámara; no hay zoom ni
compresión de tiles. En improvisar/cancelar se quitan los puntos y se centra el
Lugar actual.

Los centros se numeran desde 1 siguiendo el recorrido, incluido el origen. La
primera cifra ocupa el centro y las siguientes las celdas de su derecha, con
el color del camino como fondo y texto blanco o negro según contraste. Los
números respetan el recorte del mapa y se dibujan también en zonas desconocidas
sin descubrir terreno.

Orden de dibujo: terreno → puntos → nombres y símbolos → `@` → números → retratos.
No se escribe en paneles o barras. El trazado representa un recorrido del grafo,
no un corredor físico transitable definido por tiles, ni todas sus conexiones.

### 5.3. Color y niebla

Cada punto considera el estilo del **terreno**, antes de rótulos y retratos,
resuelto con `RecursosVisuales::tile` en esa coordenada y fotograma. Conserva su
fondo y limpia modificadores heredados.

- Tile visible y color resoluble: complemento RGB del frente,
  `(255 − r, 255 − g, 255 − b)`.
- `Recordado` o `Desconocido`: amarillo, conservando el fondo recordado o general.
  No se consulta el terreno oculto para colorear.
- Gris neutro visible (`r = g = b`): amarillo para evitar un complemento gris.
- RGB nativo no disponible: amarillo, sin asumir otra paleta de terminal.

Se reutiliza `PaletaColores` y el
[contrato de paleta](../componentes/16-paleta-terminal.md). Amarillo es `Yellow`,
sujeto al modo nativo o fijo; `-B` conserva su efecto de fondo. La animación del
terreno puede cambiar el complemento, pero no la geometría de la ruta.

Los puntos sobre zonas desconocidas son información del plan. No descubren
terreno, no modifican memoria, no conceden `Conoce` ni levantan la niebla de
nombres y símbolos. Se conserva el
[contrato de visibilidad](../componentes/22-alturas-visibilidad-memoria.md).

## 6. Responsabilidades y errores

| Módulo | Responsabilidad |
|---|---|
| `core/acciones/viajar/rutas.rs` | Coste común, rápida, larga y búsquedas ponderadas. |
| `core/acciones/viajar/propuestas.rs` (nuevo) | Habilitación, perfiles, límite, agrupación e IDs. |
| `core/acciones/viajar/{mod,estado,registros}.rs` y `tarjetas/rutas.rs` | Ruta directa, protocolo y adopción. |
| `app/{sesion,narracion_viaje}.rs` | Proyección, opciones y redacción. |
| `terminal/planificacion_rutas.rs` (nuevo) | Foco, lista, encuadre y puntos. |
| `terminal/{interfaz,desarrollo}.rs` y utilidad de líneas | Integración y trazado compartido. |

No hay dependencias nuevas ni cambio de esquema RON. Las características y
`Conoce` usan la carga existente. Se incorpora `montaraz` al catálogo descriptivo
sin asignarlo automáticamente a un Actor ni añadir otros modificadores.

Se conservan `DestinoInvalido`, `SinRuta`, `PasoNoVigente` y `OpcionNoVigente`.
Un ID antiguo o no publicado no confirma nada ni suma tiempo. Desbordamientos
de horas, pesos o IDs e inconsistencias internas usan `ErrorMotor` con contexto
y la presentación de errores existente, sin `panic`.

Sin propuestas y candidatos duplicados son resultados válidos. Sin conectividad
no se inicia planificación. No hay migración de guardados porque no existen
sesiones persistentes; se actualizan los consumidores del protocolo anterior.

## 7. Verificación y aceptación

| Contrato | Verificación requerida |
|---|---|
| Vecino | Un tramo sin escena ni día de planificación; preparación conservada. Incluir una vuelta indirecta que antes podía elegirse. |
| Oculto | Dos conexiones mediante un oculto no son viaje directo; sigue sin ofrecerse como destino. |
| Actores | Habilitación de rápida y tres perfiles; varios respaldos y varias propuestas; óptimo especializado sin habilitación descartado. |
| Tiempos | Coste común, tamaños, medios y montaña de 9/10 días; los descuentos no reducen la previsión publicada. |
| Rodeos | Montaña remota que supera el techo descartada; igualdad admitida; rápida igual a improvisada. |
| Diversidad | Duplicados agrupados; distintos recorridos de igual tiempo conservados; orden reproducible. |
| Opciones | Cero, una y muchas propuestas; improvisar/cancelar; IDs inválidos; confirmación del itinerario exacto sin regeneración. |
| Pureza | Planificación cobrada una vez; foco, Detalles, cámara y resize no modifican mundo ni azar. |
| Dibujo | Viñeta antes de elegir, sin ruta; al asentir, mapa completo y opción en barras. Línea enfocada, sin puntos al improvisar/cancelar; recorte y cámara. |
| Paleta y niebla | Complementos, grises, nativo incompleto y fijo; amarillo sobre memoria/vacío sin explorar ni conceder Conoce. |
| Integración | Viaje directo, planificado e improvisado hasta cierre; fase NPC con nuevas opciones. |

Se ejecutan los controles de la
[metodología](../../metodologia-desarrollo.md#7-controles-automáticos), validación
de contenido y compilación de producción. La prueba manual compara recorridos
montañosos, favorables al caballo y conocidos; incluye destinos vecinos, niebla
y distintos tamaños de ventana.

El incremento termina cuando se pueden comparar y confirmar los caminos
publicados viendo su trazo, el motor adopta exactamente el elegido y los viajes
vecinos dejan de desviarse por planificación. Las secciones sustituidas de los contratos anteriores remiten aquí, sin
mantener reglas incompatibles duplicadas.

## 8. Decisiones aplazadas

No quedan preguntas bloqueantes. Quedan fuera otras especialidades, acceso por
`Bloquea`, adquisición de monturas, peligros o recursos como criterios de
búsqueda, replanteamiento durante el viaje y búsqueda más exhaustiva de rutas.

## 9. Cierre de implementación

Implementado y verificado el 2026-11-03 mediante el
[PLAN-2026-044](../../planes/completados/2026-11-03-44-planificar-caminos.md).
Pasaron 250 pruebas, controles de formato y compilación, Clippy, build de
producción y humo en PTY. La revisión de jugabilidad queda para la próxima
prueba del usuario; los parámetros de preferencias pueden ajustarse en futuras
iteraciones.

La revisión 1.1 recupera la viñeta y la elección por barras, manteniendo las
búsquedas y opciones del motor. Sustituye el panel lateral inicial sin crear
una nueva fase de simulación ni otro coste de deliberación.
