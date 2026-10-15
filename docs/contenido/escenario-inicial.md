# Brief de contenido: escenario inicial

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.2 |
| Fecha | 2026-10-15 |

## 1. Propósito y autoridad

Este documento define la intención creativa mínima del contenido usado por el
primer incremento jugable. Complementa el
[diseño general](../diseno-general.md) y se materializará conforme al
[contrato técnico de contenido](../tecnica/componentes/03-contenido.md).

No contiene la matriz definitiva, coordenadas, claves externas,
Características, retratos ni código Lua. Cuando existan, esos datos concretos
tendrán una única fuente de verdad en `content/base/`; este brief conservará
solamente su propósito narrativo y visual.

## 2. Objetivo creativo

El escenario inicial será una región ficticia pequeña donde el terreno cambie
claramente entre dos ambientes:

- una zona árida y montañosa, hostil y expuesta;
- un valle fértil que transmita protección, agua y posibilidad de descanso.

El contraste debe comprenderse al mirar el mapa ASCII, incluso antes de mejorar
su acabado artístico. El escenario solo necesita sostener el recorrido de
`Viajar`, la presencia del Grupo y una acción NPC. No intenta fijar todavía el
setting definitivo, su historia amplia ni sus conflictos políticos.

El valle puede presentarse narrativamente como un buen lugar para descansar,
pero la acción `Acampar` o `Descansar` todavía no forma parte del incremento.

## 3. Lugares

| Nombre inicial | Función inicial | Presencias al comenzar |
|---|---|---|
| Zona Árida | Punto de inicio del jugador; entorno difícil desde el que se desea partir. | Príncipe Perdido y Bandido. |
| Valle Fértil | Primer destino; refugio visual y lugar apropiado para descansar en el futuro. | Cazador. |

Estos nombres descriptivos son los nombres visibles iniciales. Un refinamiento
posterior podrá sustituirlos mediante otro plan de contenido. La proximidad
dibujada entre ambos Lugares no crea una conexión reglada ni modifica la
duración de `Viajar`.

## 4. Actores

| Actor de trabajo | Papel inicial | Lugar inicial |
|---|---|---|
| Príncipe Perdido | Actor jugador. | Zona Árida. |
| Bandido | Único compañero inicial del príncipe e integrante del Grupo. | Zona Árida. |
| Cazador | NPC independiente y amistoso que contiene la única política Lua evaluable. | Valle Fértil. |

El Grupo inicial contiene solamente al Bandido. El Príncipe Perdido, como actor
jugador, no pertenece a esa colección, y el Cazador permanece independiente.
Esto ejercita el traslado conjunto y el solapamiento de retratos ya incluidos en
el alcance; no agrega una mecánica nueva.

La relación concreta que llevó al Bandido a viajar con el príncipe permanece
aplazada: para este incremento basta con que sea su compañero. `amistoso`
describe el papel narrativo del Cazador; no crea una Característica ni una regla
de facción todavía.

## 5. Método para crear el mapa ASCII

La referencia aprobada es el valle del Drâa y su entorno montañoso al sur del
Atlas, en Marruecos. Se usará como guía la imagen satelital y la descripción
geográfica publicadas por el
[CNES para la región de Zagora](https://cnes.fr/geoimage/maroc-vallee-draa-un-espace-profondes-mutations-entre-monde-desert-saharien).
El juego no copiará su raster, sus poblaciones ni sus rutas: reinterpretará
manualmente el contraste entre relieve mineral, cauce y franja cultivada.

Para el primer mapa no hace falta construir un editor ni importar datos GIS. Se
usará este procedimiento:

1. Recortar mentalmente de la referencia una composición donde la montaña árida
   limite con un valle irrigado.
2. Reinterpretar sus masas a mano sin copiar nombres, caminos, construcciones ni
   coordenadas reales.
3. Trazar primero una cuadrícula gruesa de `24 × 8` macroceldas en un editor
   monoespaciado.
4. Clasificar esas macroceldas con una paleta pequeña: suelo árido, ladera
   rocosa, montaña, vegetación fértil y cauce.
5. Expandir cada macrocélula a un bloque de `4 × 4` y suavizar manualmente los
   bordes. Esto produce un primer boceto de `96 × 32` tiles, mayor que la región
   visible mínima y suficiente para probar el movimiento de cámara.
6. Usar provisionalmente un símbolo ASCII por tipo —por ejemplo `.`, `:`, `^`,
   `,` y `~`—. La leyenda RON posterior será la autoridad sobre su significado y
   el frontend decidirá sus glifos y colores finales.
7. Situar los centros y reservar espacio para los rótulos después de estabilizar
   las masas del terreno. Las letras de los nombres se colocarán carácter por
   carácter sobre tiles existentes.
8. Copiar el boceto aprobado a la cadena raw del Mapa RON y validarlo con el
   mismo cargador que utilizará el juego.

El mapa continúa siendo presentación. Sus montañas, cauces y zonas fértiles no
habilitan, impiden ni conectan Lugares por sí mismos.

## 6. Recorrido de contenido inicial

El Príncipe Perdido y el Bandido comienzan en la Zona Árida y viajan juntos al
Valle Fértil. Al cerrarse el viaje y avanzar la fecha, la política del Cazador
solicita que viaje desde el valle hacia la Zona Árida. Narrativamente realiza
una salida de exploración; técnicamente permite comprobar la fase NPC mediante
la misma acción `Viajar`. La política comprueba además que todavía se encuentre
en el valle, por lo que no vuelve a solicitar el mismo viaje una vez trasladado.

La fecha, las claves externas, la condición Lua y la secuencia de azar exactas
pertenecen al fixture del documento técnico, no se duplican aquí.

## 7. Criterios de aceptación creativa

- El mapa permite distinguir a simple vista la Zona Árida del Valle Fértil.
- Los centros de ambos Lugares están suficientemente separados para que el
  cambio de foco desplace la cámara en el mapa de `96 × 32`.
- El Príncipe Perdido y el Bandido tienen retratos provisionales distinguibles y
  compatibles con el solapamiento del frontend.
- La única conducta inicial del Cazador es una salida de exploración y no
  contradice su papel narrativo amistoso.
- Los datos concretos respetan el fixture técnico sincronizado con este brief.
