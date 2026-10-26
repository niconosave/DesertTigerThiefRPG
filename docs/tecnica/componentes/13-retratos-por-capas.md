# Retratos por capas

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.4 |
| Fecha | 2026-10-26 |

> Ampliación implementada el 2026-10-26: el
> [contrato 15](./15-fogata-procedural.md#3-tiempo-visual) conserva los pasos de
> 250 ms de este documento, derivados de un reloj visual más fino. Su
> [iluminación](./15-fogata-procedural.md#6-iluminación-y-composición) se aplica
> después de componer los retratos del campamento, sin modificar sus archivos.

## Alcance y responsabilidades

El retrato permanece en el único RON del Actor. La aplicación carga y valida
gráficos neutrales; la terminal compone sus capas y calcula el parpadeo.
No cambia reglas, estados emocionales, tiempo del Mundo ni azar de la Partida.

## Formato

`retrato: Some((...))` contiene `paleta`, `base` y dos campos opcionales:
`rostro` y `parpadeo`. Omitir cualquiera de estos últimos equivale a `None`.
El esquema anterior con `fotogramas` se reemplaza por este formato.

```ron
retrato: Some((
    paleta: {
        'p': (frente: "blanco", fondo: Some("negro")),
    },
    base: (arte: "(o)", estilos: Some("ppp")),
    rostro: Some((
        origen: (x: 0, y: 0),
        expresiones: {
            "enojado": (arte: "\\ /", estilos: Some("ppp")),
        },
    )),
    parpadeo: Some((
        origen: (x: 1, y: 0),
        intervalo_ms: 4500,
        duracion_ms: 250,
        ojo_cerrado: (arte: "-", estilos: Some("p")),
    )),
)),
```

Cada gráfico admite cadenas raw multilínea. `arte` es rectangular y no vacío;
sus caracteres deben ocupar una columna según `unicode-width`, sin controles.
Se normaliza CRLF a LF, se rechaza CR aislado y se retira como máximo un LF
inicial y uno final. Los espacios finales cuentan; no se recortan.

`estilos: None` usa la reserva monocroma del frontend. `Some(bloque)` exige las
mismas dimensiones que `arte` y todas sus claves deben existir en la paleta
compartida. Las claves de paleta son ASCII visibles salvo espacio, comillas y
barra invertida; no se repiten. Cada estilo exige `frente` y `fondo` opcional.
Los colores son `negro`, `rojo`, `verde`, `amarillo`, `azul`, `magenta`, `cian`,
`gris`, `grisOscuro`, `grisMuyOscuro`, `rojoClaro`, `verdeClaro`, `amarilloClaro`,
`azulClaro`, `magentaClaro`, `cianClaro` y `blanco`.

Ambos orígenes son coordenadas relativas a la esquina superior izquierda de la
base, desde cero. Cada capa debe caber completamente en ella. Las expresiones
pueden tener dimensiones diferentes entre sí; sus claves siguen lowerCamelCase
y el límite de 64 bytes. Se rechazan duplicados. Un catálogo vacío es válido.
El parche del ojo puede tener cualquier tamaño rectangular que quepa en la base;
no existe una restricción de 2×1. El diseñador decide qué parte cubre.

Los errores de carga identifican archivo, Actor y gráfico involucrado.

## Composición y API

`GraficoRetrato` publica `ancho()`, `alto()` y `celdas()` en orden de filas.
Cada `CeldaRetrato` tiene `glifo` y `estilo` opcional; `EstiloCeldaRetrato`
contiene frente y fondo opcional de `ColorRetrato`.
`RetratoActor` publica `base()`, `rostro()`, `parpadeo()` y
`capas(expresion: Option<&str>, parpadeando: bool)`, que devuelve tres entradas
opcionales `(PosicionRetrato, &GraficoRetrato)` en este orden:

1. Base completa.
2. Expresión seleccionada, si existe.
3. Parche de ojo cerrado, durante el parpadeo.

`RostroRetrato` publica `origen()` y `expresion(clave)`;
`ParpadeoRetrato` publica `origen()`, `ojo_cerrado()`, `intervalo_ms()` y
`duracion_ms()`. `PosicionRetrato` contiene `x` e `y` públicos.
Los constructores son privados: la carga garantiza sus invariantes.

El renderizador omite por completo los espacios de cualquier capa, incluidos
sus estilos. En una celda visible, un fondo `None` conserva el fondo existente;
`Some("negro")` lo reemplaza por negro puro. Un estilo ausente usa `destacado`.
El frente `"negro"` usa el negro ANSI nativo (`Color::Black`), como los demás
colores nominales; el modo fijo lo resuelve mediante su paleta RGB. Para que un
ojo o bloque parcial herede el lienzo se utiliza `fondo: None`, también en el
parche de parpadeo. Un fondo negro explícito sigue siendo una decisión del arte.
Se terminan todas las capas de un Actor antes de dibujar al siguiente. El orden
del Grupo, jugador a la derecha, alineación, recorte y solapamiento del 25 %
siguen el contrato 09; sus dimensiones siempre proceden de la base.

`VistaActor.expresion: Option<String>` selecciona una expresión para esa vista.
`None` o una clave inexistente conserva la cara base. Los retratos conservan su
imagen de ojos abiertos como base y su ojo cerrado como parche.

### Señal desde las viñetas

La dirección dramática pertenece al código narrativo de la acción general.
`TextoEscena.expresion: Option<&'static str>` comunica su elección a la sesión.
Al publicar una viñeta, la sesión llama a
`VistaMundo::mostrar_expresion_grupo(Option<&str>)`, que asigna la clave al
jugador y a los compañeros presentes en esa viñeta. Este método público permite
que otros códigos de presentación emitan la misma señal; también pueden indicar
una expresión individual mediante `VistaActor.expresion`.

La terminal consulta esas claves al dibujar los retratos. No analiza el texto,
no decide emociones ni exige que todos los Actores tengan los mismos dibujos.
La señal se renueva en cada viñeta, incluidas las decisiones; una escena sin
expresión vuelve a la base. Al terminar o cancelar la acción, la vista normal
se proyecta con `None`. Cambiar de página, abrir el detalle o confirmar la salida
conserva la expresión de la viñeta vigente. No existe una variable global
mutable ni un estado emocional persistente en el Mundo.

En Viajar, `narracion_viaje::expresion_escena` fija estas señales iniciales:

| Contexto | Expresión |
|---|---|
| Llegada | `happy` |
| Desplazamiento con incidente o algún chequeo fallido | `scared` |
| Desplazamiento sin incidentes ni fallos | `happy` |
| Preparación | `happy` si el chequeo tiene éxito; `scared` si falla |
| Forrajeo | `happy` si algún chequeo tiene éxito; `scared` en otro caso |
| Provisiones incompletas | `scared` |
| Plan, carga inicial, sobrecarga | Ninguna |

Son decisiones de presentación editables junto al relato, sin efecto en las
reglas de los chequeos. `angry` está disponible cuando un contexto lo solicite;
no se asigna automáticamente a un acontecimiento de Viajar en esta versión.

## Temporización

Sin `parpadeo` el retrato permanece estático. Sus valores predeterminados son
intervalo de 4500 ms y duración de 250 ms. Ambos deben ser múltiplos positivos
de 250 ms y la duración debe ser menor que el intervalo.

El frontend obtiene un desfase del hash del ID del Actor, sin consumir azar del
juego. En pasos de 250 ms: `periodo = intervalo_ms / 250`,
`cerrado = duracion_ms / 250`, `desfase = hash(id) % periodo`; el ojo se dibuja
cuando `(paso + desfase) % periodo >= periodo - cerrado`. Intervalos editables
por Actor y desfases individuales evitan un parpadeo global sincronizado;
pueden coincidir ocasionalmente. El hash es cosmético, sin contrato de
persistencia entre versiones. La pausa de confirmación congela estos pasos.

## Autoría y verificación

El importador Durdraw exporta una sola base: `--fotograma N` selecciona un
fotograma desde uno (por defecto el primero). Deja las capas en `None`; no
deduce expresiones ni ubicación de ojos. El diseñador agrega esas capas en el
RON; la herramienta no sobrescribe el Actor. El contrato 03 detalla la CLI.

Las pruebas de carga verifican límites, duplicados, tiempos y gráficos estáticos
o con un ojo de tamaño distinto de 2×1. `TestBackend` comprueba las tres capas,
transparencia de espacios, herencia de fondo, selección desconocida, posición
estable y parpadeos independientes. El importador prueba la selección explícita.
El contenido oficial se valida en conjunto. No hay decisiones pendientes.
Las pruebas de sesión recorren un viaje y verifican que la señal alcanza al
grupo y se limpia al terminar; las del narrador comprueban señales de éxito,
fallo, carencia, llegada y retorno a una escena neutral.
