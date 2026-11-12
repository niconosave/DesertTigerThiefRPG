# Diálogos de actores en eventos narrativos

Estado: implementado, versión 1. Fecha: 2026-11-12.

## Propósito

Un evento puede presentar una escena que alterna narración e intervenciones de
actores identificados. Lua determina quién habla, qué dice y en qué orden. El
jugador confirma cada momento. Las decisiones y efectos del evento conservan
su lugar en la secuencia.

Cuando alguien habla, su nombre y su parlamento aparecen en la zona inferior.
Su retrato conserva el color; los demás pasan a gris. Al terminar la
intervención se recuperan los estilos habituales. El diálogo no añade tiempo de
juego ni modifica por sí mismo relaciones, características o emociones.

Este documento amplía el [contrato de eventos](./23-eventos-narrativos.md) y
respeta la [agenda del mundo](./24-agenda-eventos-mundo.md). No sustituye el
coordinador ni crea otro motor de escenas.

## Autoría desde Lua

La función de presentación de una `Vigneta` puede devolver una lista ordenada
`momentos`, en lugar del campo tradicional `texto`. Un espacio de nombres `ui`
expone constructores de datos, disponibles solo en programas de presentación:

```ron
Vigneta(
    clave: "advertencia",
    presentacion: r#"
        return {
            titulo = "Ante las puertas de Zahrim",
            momentos = {
                ui.narrar("Rashak se adelanta hacia la entrada del palacio. Nadin te retiene un instante."),
                ui.decir("sirviente", "No confío en él, señor. Sería mejor irnos y dejarlo aquí."),
                ui.narrar("Nadin mira hacia las puertas. No te ofrece ninguna prueba; solo su sospecha.")
            }
        }
    "#,
    opciones: [
        (clave: "audiencia", texto: "Seguir a Rashak hasta el rey", siguiente: "venta"),
        (clave: "rechazar", texto: "Alejarte con Nadin", siguiente: "separarse"),
    ],
    siguiente: None,
),
```

Esta API está disponible. En otros episodios se pueden
encadenar tantos `ui.decir` de distintos actores como necesite la escena.
`ui.decir(clave_actor, texto)` usa la clave estable, no el nombre visible. Si el guion dispone de una variable `this` que contiene una clave de actor,
puede pasarla como cualquier otra clave. Esta ampliación no crea un `this`
implícito ni cambia los contextos de los eventos.

Los constructores no dibujan, no bloquean Lua y no envían comandos al frontend.
Devuelven descriptores inmutables que Rust valida. La presentación es declarativa:
para producir una intervención se incluye el resultado de `ui.decir` en la
lista; invocarlo sin incluirlo no tiene efecto. No hay corutinas ni un Lua
suspendido mientras el jugador lee.

Contrato de devolución:

- `titulo`: como en la presentación actual.
- Exactamente uno de `texto` o `momentos`. `texto` se normaliza como un único
  momento narrativo, conservando los eventos existentes.
- `momentos`: secuencia Lua densa, no vacía, sin huecos ni claves adicionales.
- `detalle`: opcional, común a la viñeta, como hasta ahora.
- Narración y diálogo usan las validaciones de texto del sistema actual;
  ningún constructor acepta texto vacío ni controles de terminal arbitrarios.
- El actor debe existir. No se exige pertenencia al grupo del jugador.

En esta primera versión no hay voces, síntesis de habla, animación letra a
letra, tiempos automáticos de lectura ni selección de respuestas dentro de un
parlamento. Las respuestas son las opciones normales de la viñeta. Se puede
intercalar un paso `Efectos` entre dos viñetas si debe ocurrir un cambio del mundo
entre intervenciones; no se ocultan operaciones dentro de `ui.decir`.

## Datos y responsabilidades

En `desert-tiger-app`, un tipo de presentación distingue:

```rust
// Esquema orientativo, no código implementado.
enum MomentoNarrativo {
    Narracion { texto: String },
    Dialogo { actor: IdActor, texto: String },
}
```

`eventos/lua.rs` incorpora los constructores y devuelve la lista completa,
resolviendo las claves de actores con el catálogo. `eventos/contenido.rs` sigue
siendo responsable de las viñetas, opciones, enlaces y compilación Lua. No hace
falta un nuevo tipo de paso RON: los momentos pertenecen a una `Vigneta`.

`eventos/coordinador.rs` materializa y conserva una presentación preparada con
la lista validada y un cursor de momento. La función Lua se evalúa una vez al
entrar en esa viñeta. Continuar, cambiar de página, abrir detalles o redibujar no
la reevalúa. Los datos ya preparados no cambian si el mundo cambia mientras se
está leyendo. La siguiente viñeta observa el estado que le corresponda.

`VistaEvento` expone solo el momento activo, su índice, el total de momentos y
un hablante opcional. Conserva los datos comunes actuales: título, detalle,
ilustración, contexto y fecha. El hablante incluye ID, nombre visible y acceso
al retrato preparado con el mismo sistema de recursos y gráficos alternativos
que los demás actores. La vista no contiene Lua ni rutas de archivos de arte
que el frontend deba interpretar por su cuenta.

El frontend interpreta esos datos. Ningún concepto de Ratatui, posición de
terminal o color entra en la API Lua ni en el núcleo del mundo. Otro frontend
podría representar el mismo diálogo de otra forma.

## Avance, opciones y efectos

Cada momento publicado recibe un `IdPresentacionEvento` nuevo. Se reutilizan
`ContinuarEvento` y `ResponderEvento`, conservando sus validaciones de vigencia.
Una confirmación duplicada o de un momento anterior no avanza la escena.

1. Los momentos anteriores al último se publican como `PresentandoEvento`,
   sin opciones habilitadas. Continuar avanza el cursor dentro de la viñeta.
2. El último, si tiene opciones, se publica como `EsperandoDecisionEvento`.
   Responder elige el enlace normal del evento.
3. El último, sin opciones, sigue `siguiente` cuando el jugador confirma.
4. Los pasos `Efectos` y `Fin` mantienen su semántica y garantías actuales.

La viñeta no cuenta como completada al confirmar cada intervención: solo al
abandonar su último momento. No se vuelve a evaluar la condición del evento
entre parlamentos ni se permite que otro evento se intercale en esa escena.
La acción general que estuviera suspendida continúa suspendida hasta que el
coordinador lo decida con las reglas existentes.

La presentación completa se valida antes de publicar el primer momento. Un
actor desconocido en una intervención posterior no deja una escena mostrada a
medias. Los errores identifican evento, paso e índice de momento. Esto no
revierte efectos de pasos anteriores que ya se hubieran confirmado.

Los eventos no observados de la agenda siguen resolviendo sus efectos sin
mostrar interfaz. Tener diálogos no los convierte en observados ni detiene el
mundo remoto para pedir confirmaciones. No se añade una dependencia de la
terminal a la ejecución silenciosa existente.

## Presentación en la terminal

### Narración y parlamentos

Un momento narrativo usa la ventana actual del evento. Un diálogo presenta el
nombre del hablante seguido de su texto en una banda inferior, por encima de
las dos barras existentes. Durante ese momento se oculta la ventana narrativa
central del evento: no se repite el parlamento en dos lugares.

La banda usa fondo normal y texto legible, sin grandes superficies de cyan.
El nombre del actor puede destacarse en cyan. El área de mapa continúa detrás;
la banda recorta el área disponible para retratos y arte de lugar, de forma que
no tape caras. Mantiene la jerarquía: mapa, arte del lugar, retratos y
presentación del evento.

Su altura depende del contenido y de la terminal: hasta seis líneas de texto
más nombre e indicación de avance, limitada a un tercio de la altura disponible.
En terminales pequeñas se prioriza al menos una línea de lectura; se admite
recorte del decorado. No se dibuja fuera del área válida.

El texto se ajusta al ancho real con el cálculo de anchura Unicode que usa el
frontend. Se pagina si no entra. Izquierda/derecha cambian página; Enter pasa
primero a la siguiente página y, en la última, confirma el momento. Un cambio
de tamaño recalcula páginas conservando la posición de lectura más cercana.
La paginación no crea momentos ni consume identificadores del coordinador.

En el último momento con opciones, Enter todavía pasa las páginas pendientes;
solo en la última página se puede confirmar una opción. `D` abre los detalles
comunes sin avanzar la escena; al cerrarlos vuelve a la posición de lectura.
Si los detalles están abiertos, Enter no debe confirmar inadvertidamente una
intervención que no se está mostrando.

### Hablante dentro o fuera del grupo

Si el hablante está entre los retratos del grupo, se destaca en su posición
habitual. Si no está, se añade un retrato temporal de interlocutor a la derecha
del grupo, únicamente durante su intervención. No se cambia su ubicación,
pertenencia al grupo o al colectivo. No se duplican retratos de un mismo actor.

Si no tiene retrato o falta espacio, el nombre y el texto siguen siendo
suficientes para identificarlo. No es un error narrativo ni se omite su frase.
No se reorganiza la pertenencia real del grupo para acomodar el dibujo.

### Color y expresión

El foco de hablante es un dato de la presentación vigente, no una variable
global del mundo. Solo durante un diálogo los demás retratos se desaturan.
La narración, el fin del evento y la salida de la escena eliminan ese foco.

La conversión se hace sobre las celdas dibujadas del retrato, capa por capa
durante la composición de base, expresión y parpadeo. No se vuelve gris el rectángulo del
retrato ni el mapa visible entre sus espacios transparentes. No se modifica
la paleta compartida, el retrato almacenado ni sus archivos RON.

Para colores RGB se usa luminancia, por ejemplo
`(2126*r + 7152*g + 722*b) / 10000`, y se escribe el mismo valor en los tres
canales. Los colores ANSI nativos se convierten a grises ANSI mediante una
pequeña tabla de intensidades, sin consultar RGB ni fijar la paleta de la
terminal. Los colores indexados se resuelven con el conversor existente cuando
sea posible; ante un color sin equivalencia se usa gris ANSI. Es una
aproximación visual, sin efecto mecánico.

Se aplica la política vigente de colores fijos, nativos y fondo negro forzado.
Los fondos Reset/transparente y los espacios transparentes se conservan; los
fondos coloreados explícitos de un retrato sí se convierten a gris, sin tocar
el fondo general. Se revisan los modificadores para que `REVERSED` o colores
heredados no reintroduzcan el color original. La iluminación posterior de una fogata puede dar un tono cálido a los oyentes
en gris. Esto es deliberado y no requiere neutralizar de nuevo la luz.

El hablante conserva su expresión y parpadeo normales. No se exige un rostro
nuevo de «hablar», ni se inventa uno si el archivo no lo contiene. Esta primera
versión no añade sincronización de labios ni altera la música del campamento.

## Archivos principales

| Área | Responsabilidad |
|---|---|
| `desert-tiger-app/src/eventos/lua.rs` | Constructores `ui`, validación y resolución de actores. |
| `desert-tiger-app/src/eventos/coordinador.rs` | Presentación preparada, cursor, vistas e IDs vigentes. |
| `desert-tiger-app/src/sesion/eventos_partida.rs` | Exponer el momento activo y transportar las confirmaciones. |
| `desert-tiger-terminal/src/interfaz.rs` | Banda, páginas, detalles, opciones y foco del hablante. |
| Renderizado de retratos y paleta de terminal | Interlocutor temporal y conversión de las celdas a gris. |
| `content/base/events/apertura/bandido_venta_zarabad.ron` | Primera escena con narración y sospecha de Nadin. |

Se podrá extraer el dibujo del diálogo a un módulo pequeño si
el tamaño de `interfaz.rs` lo aconseja. El núcleo no necesita nuevas
operaciones del mundo para mostrar una intervención.

## Primera aplicación y verificación acotada

La advertencia antes de la venta reemplaza la conversación explícita sobre
el precio por la sospecha de Nadin. La elección de seguir o rechazar a Rashak
y las consecuencias existentes se conservan. No se escribe una respuesta de
Ashur que imponga al jugador una emoción o intención no elegida.

Pruebas centradas en esta ampliación:

- Compatibilidad de una viñeta antigua y avance ordenado de narración, diálogo
  y respuesta de otro actor. Una evaluación Lua por entrada, no por avance.
- Opciones solo al final; IDs caducados rechazados; efectos ejecutados una vez.
- Actor inexistente y secuencia mal formada rechazados antes de mostrarse.
- Hablante del grupo, externo y sin retrato, sin modificar el mundo.
- Paginación, detalles y terminal pequeña sin confirmaciones involuntarias.
- Restauración del color y transparencia; paleta nativa sin consultas RGB y
  modo fijo, incluyendo composición con iluminación.
- Un evento no observado con presentación dialogada no abre interfaz.

La prueba manual principal será llegar a Zarabad, leer la sospecha de Nadin y
comprobar ambos desenlaces de la elección. No se requiere correr toda la suite
para cada ajuste de texto o de apariencia.
