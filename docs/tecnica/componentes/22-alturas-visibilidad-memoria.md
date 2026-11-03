# Alturas del mapa, visibilidad y memoria de exploración

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.1 |
| Fecha | 2026-11-03 |
| Implementación | Relieve, visión, memoria y representación en terminal |

## 1. Comportamiento buscado

El mapa posee dos matrices de idénticas dimensiones: una de tipos de terreno y
otra de alturas. Ambas se editan mediante caracteres y una leyenda dentro del
mismo RON del mapa. La segunda describe el relieve que permite u obstaculiza
la vista; no contiene números en cada casillero.

El grupo ve hasta una distancia limitada y el relieve puede ocultar parte del
terreno dentro de ese alcance. La partida conserva qué tiles se observaron:

| Estado del tile | Presentación |
|---|---|
| Visible ahora | Aspecto y animación normales. |
| Visto antes, fuera de la visión actual | Último tipo de terreno observado, en gris y sin animación. |
| Nunca visto y fuera de la visión actual | Espacio vacío con el fondo general de la terminal. |

Observar, recordar y conocer un Lugar son conceptos independientes. Esta
función no añade relaciones `Conoce`, no modifica las rutas y no restringe
los destinos del jugador o de los NPC. Un destino conocido puede figurar en
la lista aunque su zona del mapa todavía no esté explorada.

La visión y la memoria son del grupo del jugador y duran toda la partida.
No se reinician al viajar, acampar o separar compañeros. El guardado de ese
estado y la percepción individual de NPC quedan fuera del alcance actual.

## 2. Autoría del relieve

La [carga de contenido](./03-contenido.md) amplía el RON de Mapa con un bloque
obligatorio `alturas`. `leyenda` y `matriz` actuales conservan sus funciones.
El ejemplo completo siguiente tiene dos matrices de ocho columnas y cuatro filas:

```ron
(
    clave: "ejemplo",
    leyenda: {
        'A': "agua",
        '-': "sueloArido",
        '^': "montana",
    },
    matriz: r#"
AAAA----
AAA--^^-
AA--^^^^
A---^^^^
"#,
    alturas: (
        leyenda: {
            '~': 0,
            '.': 1,
            ':': 2,
            'o': 4,
            '^': 6,
            'A': 9,
        },
        matriz: r#"
~~~~....
~~~..oo.
~~..o^^o
~...o^Ao
"#,
    ),
    vision: (radio: 32, altura_observador: 1),
)
```

Las dos leyendas son independientes: un mismo carácter puede tener significados
distintos en cada una. Los valores de la leyenda de alturas son enteros `u16`
no negativos, en unidades abstractas compartidas con `altura_observador`.
No se interpretan como metros. Se permiten varios símbolos para la misma altura,
pero una clave de carácter repetida se rechaza.

Los símbolos de altura deben ser ASCII imprimible sin espacios ni dígitos: `.`, `:`, `o`,
`^`, etc. El diseñador elige su paleta de signos. Evitar el espacio permite
distinguir alturas reales de huecos o espacios que introdujo el editor.
El motor no deduce alturas de los caracteres del mapa de terreno ni de las
características de los Lugares. Un bosque puede estar en un valle o sobre una
montaña sin cambiar la regla del relieve.

La carga elimina únicamente un salto inicial y uno final de cada bloque,
normaliza CRLF y conserva el resto de sus caracteres. Rechaza filas vacías,
matrices no rectangulares, símbolos sin declarar y cualquier diferencia de ancho
o alto respecto de la matriz de terreno. No rellena, escala ni recorta matrices.
Ambas comparten coordenadas de base cero e índice `y * ancho + x`.

`vision` es opcional. Los valores iniciales son `radio: 32` tiles y
`altura_observador: 1`, también usados cuando se omite ese bloque. Si el bloque
está presente, ambos campos son obligatorios y de tipo `u16` positivo. El radio representa distancia euclídea en coordenadas de
mapa, sin corregirla por la proporción de la fuente de la terminal. Son parámetros
editables de esta presentación, no estadísticas del actor.

Los errores incluyen archivo de mapa, campo y coordenada cuando corresponda.
La implementación debe incorporar una matriz explícita de 192 × 64 al mapa
oficial actual y actualizar las pruebas de contenido. No se mantiene una
inferencia automática de alturas por bioma como sustituto de esa autoría.

## 3. Propietario del estado y observador

`desert-tiger-app` posee el relieve, la visión actual y la memoria dentro de la
Partida. La terminal recibe el resultado y lo dibuja; el core sigue trabajando
con Lugares y relaciones, sin consultar tiles o alturas. Así otro frontend
puede conservar la misma exploración del mapa.

El observador está en la coordenada central del Lugar realmente ocupado por el
actor jugador, obtenida del Mundo y del catálogo de presentación. Su altura
visual es la altura de ese tile más `altura_observador`.

No se usa el foco de selección, el objetivo de cámara, la posición de un retrato
ni `lugar_paisaje` de una viñeta. Este último puede representar el origen de un
tramo que el grupo ya terminó de recorrer. El mapa, la cámara y el arte narrativo
conservan sus responsabilidades distintas.

La primera observación se calcula al crear la partida. Después se actualiza
cuando un traslado confirmado cambia el Lugar del jugador, incluidos los
Lugares intermedios u ocultos de un viaje y el lugar real de una interrupción.
Las decisiones pendientes que todavía no trasladaron al grupo no lo exploran.

Los viajes actuales no definen un recorrido tile a tile. Por eso se observan
los centros confirmados sucesivos, sin descubrir una línea artificial entre
ellos. Mover la cámara, abrir detalles, paginar, consultar `vista()` o cambiar
el tamaño de la ventana son operaciones de lectura: no descubren tiles.

## 4. Cálculo de visibilidad

Se incorpora una función pura de cálculo, determinista y sin azar. Solo el
relieve obstaculiza esta primera versión; los tipos `bosque`, `agua`, etc. no
añaden otra obstrucción. No hay clima, visión nocturna ni capacidades especiales.

### Alcance

Se visitan candidatos dentro del rectángulo del radio, limitado por los bordes
del mapa. Un destino pertenece al alcance cuando:

```text
dx² + dy² <= radio²
```

El tile del observador siempre es visible. No se consulta lo que hay más allá
del borde del mapa y no se considera visible ningún candidato fuera del radio,
aunque tenga una altura muy grande.

### Relieve sobre la línea de vista

Para cada destino dentro del alcance se traza una línea desde el centro del
tile del observador hasta el centro del tile de destino. Se enumeran todas las
celdas tocadas por esa línea mediante recorrido de cuadrícula con cobertura
completa (*supercover*). En un cruce exacto de esquina se incluyen las dos
celdas laterales y la diagonal; así no se generan ventanas entre dos obstáculos
que se tocan por una esquina. Se excluyen origen y destino de los obstáculos.

El recorrido compara el próximo cruce de borde vertical y horizontal mediante
fracciones, sin flotantes. Los cruces iniciales de cada eje están a media celda;
cada cruce posterior añade una celda. Un eje sin desplazamiento no tiene cruces.
Al empatar se incluyen ambas celdas laterales antes de avanzar en ambos ejes.

Se compara la altura de cada celda intermedia con la recta ascendente o
descendente entre el ojo del observador y la superficie del destino. Para
evitar resultados dependientes del redondeo se usan productos enteros:

```text
s = centro del tile del observador
t = centro del tile destino
q = centro de una celda intermedia tocada por el rayo
z_s = altura(s) + altura_observador
z_t = altura(t)
D = (t.x - s.x)² + (t.y - s.y)²
u = (q.x - s.x) * (t.x - s.x) + (q.y - s.y) * (t.y - s.y)

la celda q bloquea si:
altura(q) * D > z_s * D + (z_t - z_s) * u
```

Las diferencias y productos se calculan con enteros con signo `i128`. La altura
se compara en la proyección del centro de la celda sobre el rayo; es una regla
discreta del mapa, no una simulación de superficies tridimensionales. La igualdad
permite ver: el rayo que roza exactamente una altura no queda bloqueado.

El destino es visible si ninguna celda intermedia lo bloquea. La propia montaña
que obstaculiza la vista puede verse; un pico más alto detrás de una cresta
también puede verse si su línea supera los obstáculos. Por tanto no basta con
marcar cualquier montaña como un muro que oculta todo lo que tiene detrás.

El coste depende del área del radio y de las longitudes de esos rayos. Se corta
cada rayo al primer obstáculo y se recalcula cuando cambia la posición del
observador, también al regresar a una coordenada ya visitada.
No se ejecuta este algoritmo por frame, ni al explorar destinos con la cámara.
El mapa y el radio iniciales permiten medir esta primera implementación antes
de introducir algoritmos o estructuras más complejos.

## 5. Memoria de exploración

El estado conceptual es:

```rust
struct VisionMapa {
    visibles: Vec<bool>,
    explorados: Vec<bool>,
    tipos_recordados: Vec<ClaveTipoTile>,
}
```

Las tres listas tienen exactamente `ancho * alto` entradas. `explorados` es el
array de booleanos de «alguna vez visto»: empieza en `false` y solo cambia a
`true`. `visibles` describe exclusivamente la observación actual y se sustituye
en cada actualización. Todo tile visible debe estar explorado.

En cada observación se copia el tipo de terreno de los tiles visibles a
`tipos_recordados`. Sus entradas no exploradas se inicializan al crear el mapa,
pero nunca se dibujan. Esto conserva el último tipo visto: si en el futuro cambia
un bosque fuera de la vista, el recuerdo no revela ese cambio automáticamente.
No se guardan colores, glifos de Ratatui ni fotogramas dentro de la Partida.

Sin observación nueva, la memoria no se modifica. Al volver a observar un tile
se actualiza su tipo recordado y recupera la presentación normal. No se utiliza
una variable global de Lua para cada casillero ni una relación por tile.

En esta versión «sin animación» significa dibujar el **fotograma cero** del
patrón correspondiente al último tipo observado, usando sus coordenadas globales
habituales. Al salir de visión puede cambiar una vez de glifo por ese paso al
fotograma cero; después permanece fijo. No se captura el fotograma exacto que
estaba viendo un frontend concreto. Esta decisión mantiene la memoria portable.

## 6. API y organización del código

`contenido.rs` deserializa y valida `AlturasMapaDto` y `VisionMapaDto`, incorporando
alturas y parámetros a la plantilla y al `EstadoMapa` de la aplicación. Los datos
validados pueden almacenarse como `Vec<u16>`: el requisito de usar caracteres
corresponde al archivo editable, no a la representación interna.

Se añade `app/src/vision_mapa.rs` con estas responsabilidades y API interna:

```rust
fn calcular_visibles(
    ancho: usize, alto: usize, alturas: &[u16],
    origen: CoordenadaMapa, radio: u16, altura_observador: u16,
) -> Vec<bool>;

impl VisionMapa {
    fn nueva(mapa: &EstadoMapa) -> Self;
    fn observar(&mut self, mapa: &EstadoMapa, origen: CoordenadaMapa);
}
```

La sesión crea y posee `VisionMapa`. La operación `observar` actualiza ambas
máscaras y los tipos recordados. Sus precondiciones —dimensiones coincidentes,
origen válido y parámetros positivos— están garantizadas por la carga y la
sesión. Las consultas públicas no llaman a `observar`.

La proyección pública amplía el tipo existente:

```rust
pub enum VisibilidadTile { Desconocido, Recordado, Visible }

pub struct VistaMapa {
    // Dimensiones y catálogo de tipos existentes.
    pub celdas: Vec<ClaveTipoTile>,
    pub visibilidad: Vec<VisibilidadTile>,
}
```

Las listas comparten el índice existente. `celdas` ofrece el tipo actual en un
tile visible y el último tipo observado en uno recordado. En uno desconocido
su valor no se interpreta para dibujar. Se conservan `tipos_declarados` y la
validación de recursos visuales. El catálogo completo no pretende ser una
barrera contra inspeccionar los datos o modificar el juego abierto.

`Partida` integra la observación antes de publicar el mapa de cada traslado
confirmado. Usa los cambios `ActoresTrasladados` que incluyen al jugador. Si un
único resultado contiene varios traslados efectivos, observa sus destinos en
orden, conservando la exploración de todos ellos y dejando como visión actual
la última posición. No consulta repetidamente la posición final para sustituir
esos destinos intermedios.
Las solicitudes de NPC no observan desde sus posiciones.

En este incremento el relieve y los parámetros de visión permanecen inmutables
durante la partida. Si más adelante se añaden operaciones para modificarlos,
deberán invalidar la visión y observar de nuevo aunque el grupo no se mueva.

## 7. Presentación en terminal y otras capas

`dibujar_mundo` consulta la visibilidad antes de resolver la apariencia de una
celda. Para `Visible` utiliza el paso de animación actual y el estilo del tile.
Para `Recordado` utiliza el tipo recordado, paso cero y el estilo de memoria.
Para `Desconocido` conserva la celda vacía con el fondo general de pantalla.
El muestreo del patrón mantiene `x` e `y` globales; mover la cámara no desplaza
las texturas ni cambia la exploración.

Se añade el rol `mapa_recordado` a `ui.ron`, inicialmente con frente
`grisOscuro`, sin modificadores. Su fondo es siempre el fondo general de pantalla,
sin conservar fondos coloreados del terreno. Así el agua recordada tampoco
mantiene un fondo azul. La paleta terminal, la fija y `-B` se aplican mediante
el mecanismo existente. El rol es obligatorio en los recursos oficiales y no
modifica los colores de los tiles que se ven actualmente.

Los símbolos de lugares y cada carácter de sus rótulos respetan la visibilidad
del tile que ocupan: normal si es visible, gris si es recordado y omitido si es
desconocido. Esto también se aplica a los rótulos de regiones. No se descarta
un rótulo entero por su centro; pueden quedar fragmentos legibles según los
tiles observados. El diseño de sus coordenadas sigue siendo tarea del autor.

Durante la selección de destino, el rótulo del Lugar en foco se muestra con su
resaltado completo, incluso sobre tiles desconocidos o recordados. Se dibuja al
final de los rótulos para conservar el foco cuando otro Lugar comparte nombre y
coordenadas. Se mantiene el recorte de cámara. Esta excepción afecta solo a ese
nombre: no revela terreno, símbolos, otros lugares ni modifica la memoria.
Al cambiar de foco, el nombre anterior vuelve a respetar la máscara normal.

El marcador `@` conserva prioridad; su posición siempre pertenece a la visión
actual. Los retratos, las ventanas y el arte de paisaje del
[contrato 21](./21-arte-lugares.md) mantienen su composición y colores: son
ilustraciones narrativas superpuestas, no tiles por descubrir.

La lista de destinos y sus nombres conservan las reglas de conocimiento y
selección. Centrar la cámara en uno lejano no lo explora: puede verse una zona
recordada o vacía alrededor. El inspector sigue informando coordenadas aunque
el terreno esté oculto. Las líneas y centros de conexiones de desarrollo
respetan la misma máscara: en tiles recordados quedan grises y sin ciclo de
color; en tiles desconocidos no se dibujan.

No se añade en esta versión un modo de revelar todo el mapa. El cálculo tampoco
se desactiva al abrir el menú de desarrollo. Las pruebas del mapa usan máscaras
explícitas para comprobar sus distintas capas.

## 8. Pruebas y criterios de aceptación

| Área | Verificaciones |
|---|---|
| Carga | Matrices idénticas en tamaño, leyendas independientes, símbolos repetidos o ausentes, caracteres inválidos y coordenada del diagnóstico. |
| Alcance | Disco euclídeo, radio inclusivo, origen visible, esquinas y bordes del mapa. |
| Relieve | Llanura, pared cercana, terreno detrás oculto, pico alto visible por encima, observador elevado, igualdad de alturas y cruces diagonales. |
| Determinismo | Misma entrada produce la misma máscara; sin fecha de sistema, RNG o dependencia del tamaño de terminal. |
| Memoria | Arranque sin exploración previa, unión de visitas, vuelta a una zona y conservación al acampar o cambiar compañeros. |
| Recuerdo | Gris, fondo neutro y fotograma fijo; un cambio de terreno no observado conserva el último tipo visto. |
| Viajar | Observación de cada centro confirmado, lugar real de interrupción, ninguna observación por decisiones pendientes, planes o movimientos de NPC. |
| Interfaz | Cámara, redimensionado, detalle y lista de destinos no descubren nada; las letras y símbolos no revelan tiles desconocidos. |
| Integración | Descripciones, arte de lugares, retratos, controles y rutas conservan sus contratos. |

El cálculo se prueba con mapas pequeños dibujados en texto y máscaras esperadas,
sin depender de Ratatui. El renderizado se verifica con `TestBackend`, comparando
dos instantes distintos para asegurar que los recuerdos no se animan. Se mide
el coste de observaciones del mapa oficial con radio 32; no se fija un tiempo
absoluto dependiente de una computadora particular como requisito del formato.

La prueba jugable inicial recorre una llanura, se acerca a una cordillera,
alcanza un punto alto y regresa. Debe distinguir terreno desconocido, visible
y recordado, comprobar el bloqueo de la montaña y reconocer los tiles grises
sin que el movimiento de cámara explore el resto de la isla.

## 9. Valores iniciales y ampliaciones aplazadas

Los valores iniciales son radio 32 y altura del observador 1, fotograma cero para
los recuerdos y gris oscuro como frente de memoria. Se pueden ajustar sin
introducir nuevas reglas de viaje. La matriz oficial explícita tiene 192 × 64
casilleros; representa agua, llanuras, faldas y crestas de las cordilleras, con
cumbres destacadas. El juego no genera esa matriz a partir del terreno.

No se diseñan todavía reducción del alcance por clima, percepción por actor,
transmisión de mapas entre NPC, memoria individual, persistencia ni desplazamiento
continuo por tiles. Esas ampliaciones no son necesarias para este comportamiento.
