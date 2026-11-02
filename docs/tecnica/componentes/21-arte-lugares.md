# Arte de lugares sobre el mapa

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.0 |
| Fecha | 2026-11-02 |
| Implementación | Carga, contexto narrativo y composición sobre el mapa disponibles |

## 1. Propósito y alcance

Presentar una ilustración del paisaje sobre el mapa, detrás de los retratos del
grupo y de las ventanas. Su función es dar identidad visual al lugar y contexto
a la escena del viaje. Un lugar puede tener arte sin descripción, descripción
sin arte, ambos o ninguno.

El formato permite ilustraciones amplias. Alrededor de diez renglones es una
referencia artística para el primer contenido, sin convertirse en altura fija,
límite de validación ni requisito para mostrar un dibujo. Su ancho también es
libre. El frontend conserva las proporciones de las celdas y recorta lo que no
entra en la ventana.

Este documento diseña el código de presentación. No cambia la simulación, las
rutas, la selección de destinos ni el significado de las características.
Tampoco añade animación de paisajes, desplazamiento del dibujo con la cámara,
iluminación de la fogata sobre el paisaje ni controles nuevos para el jugador.

## 2. Datos existentes y responsabilidades

El arte se escribe dentro del RON de cada Lugar. Su formato, paleta, validación
y compatibilidad tienen una única fuente en
[Contenido, descripción e ilustración](./03-contenido.md#descripción-e-ilustración-de-lugar--2026-11-02).
No se agregan campos de posición, anclaje o tamaño de pantalla al RON.

La aplicación ya entrega `VistaLugar.arte: Option<ArteLugar>`. `ArteLugar`
expone `ancho()`, `alto()` y `celdas()`, con `CeldaRetrato` y sus estilos. La
terminal consulta esa vista; no abre archivos ni deserializa RON al dibujar.
`None` significa que no hay ilustración para ese lugar.

La aplicación decide qué Lugar corresponde al contexto narrativo de la escena.
La terminal decide dónde y cómo dibujar su arte. El core no recibe dibujos,
colores ni geometría de pantalla. Un frontend futuro puede ignorar el arte de
caracteres o representarlo de otra forma.

En esta primera versión se utiliza el arte base recibido en `VistaLugar`, en
todos los modos `--graphics`. Los catálogos alternativos de lugares quedan
aplazados; no se amplía ahora el cargador de alternativas de actores. Como en
los demás recursos base usados de respaldo, `--graphics ascii` no convierte
automáticamente este arte a ASCII.

## 3. Qué paisaje se muestra

El contexto de la ilustración es independiente del objetivo de la cámara y de
la posición confirmada del grupo. La cámara y el marcador `@` mantienen sus
reglas actuales.

| Estado de la interfaz | Lugar que aporta el arte |
|---|---|
| Mapa disponible | Lugar actual del grupo, `VistaMundo.lugar_jugador`. |
| Selección de destino | El mismo lugar actual; mover el foco no previsualiza el destino. |
| Viñeta de desplazamiento | Origen del desplazamiento que relata esa viñeta. |
| Viñeta final de llegada | Lugar alcanzado. |
| Otras viñetas y decisiones de viaje | Lugar de la escena, indicado por el registro del motor. |
| Campamento | No se dibuja esta capa: el fondo es la ficha, no el mapa. |
| Error fatal o terminal menor al mínimo | No se dibuja esta capa. |

Las ventanas de detalle, el menú de desarrollo y la confirmación de salida no
cambian por sí mismos el contexto narrativo. Se dibujan sobre la composición
que corresponda al estado subyacente.

Un lugar `oculto`, como el centro del Paso de Zunir, puede aportar arte. La
terminal no filtra esta capa por nombre vacío, visibilidad del rótulo, relación
Conoce o posibilidad de seleccionarlo como destino.

Si el Lugar del contexto no tiene arte, se muestra el mapa sin esta capa. No
se conserva el arte anterior ni se busca otro entre vecinos o regiones. Cambiar
de lugar, de viñeta o de modo se refleja en el siguiente dibujo, sin memoria de
visitas, fundidos o escenas adicionales.

### Contrato entre aplicación y terminal

Se añade a la vista pública existente:

```rust
pub struct VistaProgresoViaje {
    // Campos existentes de posición, destino, fecha y tiempo.
    pub lugar_paisaje: IdLugar,
}
```

`narracion_viaje::lugar_paisaje(&RegistroEscena) -> IdLugar` es una función pura:
devuelve `Desplazamiento.origen` para ese contenido, y `registro.lugar` para
los demás contenidos. `sesion::progreso` publica ese ID tanto en
`VistaFragmento` como en `VistaDecision`, mediante su campo `progreso`.

`Narrador::redactar` reutiliza la misma función cuando añade la descripción en
Desplazamiento o Llegada. Mantiene los momentos actuales de inserción del texto;
no añade descripciones a otras tarjetas por introducir esta función. La elección
del ID nunca depende de que exista arte o descripción.

Durante una escena, la terminal resuelve `progreso.lugar_paisaje` contra
`ultima_vista_mundo.lugares`. No interpreta el texto ni usa la clave de
ilustración de viñeta para elegir un Lugar. Fuera de las escenas utiliza el ID
actual del grupo. Un ID ausente del catálogo no produce un dibujo; la aplicación
debe garantizar en sus pruebas que los IDs publicados existen.

## 4. Posición y recorte

La ilustración usa coordenadas de pantalla. Se centra horizontalmente y apoya
su borde inferior en el borde inferior del rectángulo del mapa, inmediatamente
encima de la barra contextual. Los retratos quedan por delante de ella.

Si el rectángulo del mapa es `(ax, ay, aw, ah)` y el arte mide `(w, h)`, el
origen sin recortar es:

```text
x = ax + (aw - w).div_euclid(2)
y = ay + ah - h
```

La resta se calcula con enteros con signo antes del recorte. Una diferencia
impar deja una celda más en uno de los lados, de forma determinista. Un dibujo
más ancho que el mapa se recorta a ambos lados; uno más alto pierde sus filas
superiores y conserva la base. No se reenvuelven líneas ni se escalan glifos.
Los márgenes y filas vacías del dibujo cuentan para su tamaño y anclaje.

El recorte es exactamente el rectángulo real del mapa, no el de la terminal
completa. En selección de destino se utiliza el mapa reducido por la lista
lateral; la ilustración nunca invade la lista ni las dos barras inferiores.
Mover la cámara no desplaza el arte. Redimensionar recalcula su anclaje y recorte
sin reiniciar el viaje ni modificar el contenido.

No se reserva una franja para la ilustración: el mapa sigue ocupando el mismo
espacio que antes. Un lugar sin arte mantiene la distribución habitual.

## 5. Orden de composición

El dibujo conserva esta prioridad, de fondo a frente:

```text
tiles, símbolos y rótulos del mapa
                ↓
         arte del Lugar
                ↓
       marcador del grupo @
                ↓
     retratos de compañeros y jugador
                ↓
  barras, lista lateral y ventanas según su región
                ↓
     menú de desarrollo y confirmación de salida
```

Las ventanas de viaje siguen dibujándose después de la composición del mundo.
Pueden cubrir el paisaje o los retratos; no se desplazan ni cambian de tamaño
para dejar visible un dibujo. La ilustración nunca pasa a ser contenido interior
de esas ventanas. Las ilustraciones de `scenes.ron` conservan su función actual.

Los rótulos del mapa pueden quedar cubiertos por el arte. El marcador `@` queda
encima para conservar la referencia de la posición real del grupo, aunque el
paisaje corresponda al origen de una escena ya resuelta.

Para utilizar las herramientas de desarrollo se suspende esta capa mientras
esté activo el inspector de coordenadas o la visualización de conexiones. Esto
permite inspeccionar los tiles y las líneas sin confundirlos con el paisaje.
Desactivar ambas herramientas restaura el arte del contexto actual. No cambia
su selección ni requiere un ajuste adicional en el menú.

## 6. Transparencia y colores

Se utiliza la misma composición por celda que en los retratos:

- Un espacio no escribe glifo, color ni fondo, aunque tenga un estilo asignado.
- Un carácter visible reemplaza el glifo y aplica su frente.
- Un fondo `None` conserva el fondo de la celda inferior; `Some` lo sustituye.
- Sin estilo explícito se utiliza `recursos.ui.destacado`, la reserva monocroma
  vigente para estos gráficos.
- La celda no hereda negrita, atenuación u otros modificadores del terreno.
  Los estilos propios de los retratos tampoco se heredan del paisaje.

No se usa `Clear` ni se pinta un rectángulo de fondo antes de la ilustración.
Para cubrir una celda completa el artista debe utilizar un glifo visible y los
colores apropiados. Se conservan las diferencias actuales entre negro nativo,
fondo negro explícito, `-B` y el esquema fijo; la fuente de ese comportamiento
es el [contrato de paleta](./16-paleta-terminal.md).

La conversión global de paleta permanece al final de `Interfaz::dibujar`. No
se añade un segundo tratamiento RGB. El arte es estático y no necesita reloj
ni consume azar de la partida.

## 7. Organización del código

| Módulo | Responsabilidad |
|---|---|
| `app/src/narracion_viaje.rs` | Elegir el Lugar del contexto y compartirlo con la descripción. |
| `app/src/sesion.rs` | Publicar `lugar_paisaje` dentro del progreso de ambas clases de vista. |
| `terminal/src/arte_lugares.rs` | Calcular anclaje, recorte y dibujo del `ArteLugar` seleccionado. |
| `terminal/src/dibujo_caracteres.rs` | Compartir el pintado de celdas entre retratos y lugares. |
| `terminal/src/interfaz.rs` | Resolver la vista vigente y componer el paisaje en el orden establecido. |

API interna de la terminal:

```rust
// Interfaz: resuelve el estado y las exclusiones de la sección 3 y de desarrollo.
fn arte_lugar_visible(&self) -> Option<&ArteLugar>;

// arte_lugares: geometría pura y renderizado, sin consultar Partida.
fn origen_arte(area: Rect, ancho: usize, alto: usize) -> Option<(i64, i64)>;
fn dibujar(frame: &mut Frame<'_>, area: Rect, arte: &ArteLugar,
           recursos: &RecursosVisuales);

// dibujo_caracteres: extraído del pintado actual de los retratos.
fn dibujar_celdas(frame: &mut Frame<'_>, recorte: Rect, x: i64, y: i64,
                 ancho: usize, celdas: &[CeldaRetrato],
                 recursos: &RecursosVisuales);
```

`dibujar_grafico_retrato` pasa a delegar en la función compartida, conservando
orígenes, capas, parpadeo y solapamiento actuales. La extracción mueve la
conversión de estilos de celda al mismo módulo compartido y mantiene una sola
implementación de la limpieza de modificadores.

`Interfaz::dibujar_mundo` inserta la ilustración entre el recorrido de rótulos y
el marcador `@`. `dibujar_contenido` conserva la prioridad de lista, barras y
modales. El campamento sigue usando su composición propia.

Solo se recorren las filas y columnas de arte que intersectan el recorte. Las
conversiones a coordenadas sin signo se realizan después de comprobar límites.
Si una dimensión no puede representarse en los cálculos con signo, se omite ese
dibujo sin escribir fuera del buffer. No se introducen dependencias, cachés de
imágenes ni lecturas de disco durante el renderizado.

## 8. Validación y criterios de aceptación

Las pruebas usan dibujos pequeños sintéticos y `TestBackend`; no necesitan
instalar la fuente especial ni depender de su apariencia en una máquina concreta.

| Prueba | Resultado exigido |
|---|---|
| Movimiento cuyo lugar confirmado ya es el destino | Paisaje del origen, coherente con el relato; cámara y `@` conservan el destino confirmado. |
| Llegada y otras tarjetas | ID de paisaje correcto y válido en fragmentos y decisiones. |
| Lugar con arte y descripción vacía | Dibujo visible sin añadir texto. |
| Lugar sin arte después de uno ilustrado | Desaparece el dibujo anterior y se recupera el mapa. |
| Centro oculto sin nombre | Su arte se puede utilizar durante el recorrido. |
| Centrado, dimensiones pares e impares y rectángulo desplazado | Coordenadas reproducibles, sin desbordamientos. |
| Arte más ancho o alto que la vista | Recorte lateral y superior, base conservada y ninguna escritura en las barras. |
| Selección de destinos | Contexto actual, centrado en el mapa reducido y lista lateral intacta. |
| Transparencia, fondo explícito y modificadores del tile | Composición idéntica a la de retratos, sin franjas de estilo heredado. |
| Actor y ventana sobre el paisaje | Sus glifos opacos conservan prioridad; sus transparencias funcionan como antes. |
| Paginar, abrir detalle y redimensionar | No cambia el ID de paisaje, el mundo, el tiempo ni las decisiones. |
| Campamento, error fatal y tamaño insuficiente | Se cumplen las exclusiones de la sección 3. |
| Herramientas de desarrollo | El paisaje se suspende y se restaura según su activación. |

Una comprobación manual final utiliza un paisaje ancho en un Lugar de prueba,
lo observa desde el mapa y durante un viaje, y cambia el tamaño de la terminal.
Se revisa que la ventana siga siendo legible y los retratos queden por delante.
El contenido artístico definitivo y sus retoques se trabajan después sobre el
comportamiento ya jugable.
