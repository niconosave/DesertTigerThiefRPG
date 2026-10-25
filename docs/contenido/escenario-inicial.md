# Brief de contenido: escenario inicial

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.1 |
| Fecha | 2026-10-25 |

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
claramente entre dos ambientes enlazados por un paso reconocible:

- una zona árida y montañosa, hostil y expuesta;
- un paso escarpado que haga perceptible el cruce de la cordillera;
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
| Hamada Roja | Punto de inicio del jugador; entorno difícil desde el que se desea partir. | Príncipe Perdido y sus cuatro compañeros. |
| Paso Escarpado | Paso geográfico entre ambos ambientes; introduce la primera complicación contextual de `Viajar`. | Ninguna. |
| Valle del Drâa | Primer destino; refugio visual y lugar apropiado para descansar en el futuro. | Cazador. |

Los dos nombres descriptivos provisionales fueron sustituidos por los nombres
aprobados en el atlas regional. La proximidad
dibujada entre los Lugares no crea una conexión reglada ni modifica la duración
de `Viajar`: el recorrido existe por sus Relaciones declaradas.

## 4. Actores

| Actor de trabajo | Papel inicial | Lugar inicial |
|---|---|---|
| Príncipe Perdido | Actor jugador. | Hamada Roja. |
| Bandido | Compañero inicial del príncipe e integrante del Grupo. | Hamada Roja. |
| Cazador | NPC independiente y amistoso que contiene la única política Lua evaluable. | Valle del Drâa. |
| Sirviente | Compañero del príncipe e integrante del Grupo. | Hamada Roja. |
| Mercader | Compañero del príncipe e integrante del Grupo. | Hamada Roja. |
| Niño ladrón | Compañero del príncipe e integrante del Grupo. | Hamada Roja. |

El Grupo inicial contiene al Bandido, al Sirviente, al Mercader y al Niño ladrón.
El Príncipe Perdido, como actor jugador, no pertenece a esa colección, y el
Cazador permanece independiente.

La relación concreta que llevó al Bandido a viajar con el príncipe permanece
aplazada: para este incremento basta con que sea su compañero. `amistoso`
describe el papel narrativo del Cazador; no crea una Característica ni una regla
de facción todavía.

## 5. Método para crear el mapa ASCII

La referencia visual aprobada para la composición es `assets/map.png`, creada
por la diseñadora a partir de la idea inicial del valle del Drâa. La imagen de
trabajo no se carga durante una partida: el dato ejecutable y modificable sigue
siendo la matriz declarada en `content/base/maps/mapa_inicial.ron`.

La traducción conserva las masas y accidentes reconocibles de la referencia:

- mar exterior y cursos de agua interiores;
- región árida occidental y sus costas arenosas;
- cordillera central y ramal sudoriental;
- llanura verde nororiental y bosque oriental;
- islas menores al oeste y al norte.

La imagen se reduce a una cuadrícula de `96 × 32` y después se corrigen a mano
los bordes o cauces que pierdan legibilidad. Cada símbolo de la matriz remite a
un tipo mediante la leyenda RON; los glifos, colores y estilos que finalmente se
ven en la terminal se definen en `assets/terminal/tiles.ron`. Los centros y
rótulos de los Lugares se sitúan al final, sin modificar la matriz subyacente.

El mapa continúa siendo presentación. Sus montañas, cauces y zonas fértiles no
habilitan, impiden ni conectan Lugares por sí mismos.

## 6. Recorrido de contenido inicial

El Príncipe Perdido y el Bandido comienzan en la Hamada Roja y viajan juntos al
Valle del Drâa atravesando el Paso Escarpado. El paso proporciona el contexto de
la primera decisión de viaje y el Bandido permite que una cualidad de un
compañero altere uno de sus resultados.

Al cerrarse el viaje y avanzar la fecha, la política del Cazador solicita que
viaje desde el valle hacia la Hamada Roja por el mismo recorrido. Narrativamente
realiza una salida de exploración; técnicamente permite comprobar la fase NPC
mediante la misma acción `Viajar`. La política comprueba además que todavía se
encuentre en el valle, por lo que no vuelve a solicitar el mismo viaje una vez
trasladado.

La fecha, las claves externas, la condición Lua y la secuencia de azar exactas
pertenecen al fixture del documento técnico, no se duplican aquí.

## 7. Criterios de aceptación creativa

- El mapa permite distinguir a simple vista la Hamada Roja del Valle del Drâa.
- El Paso Escarpado se sitúa visualmente entre ambos y su rótulo no impide
  reconocer la cordillera que representa.
- Los centros de los dos Lugares extremos están suficientemente separados para
  que el cambio de foco desplace la cámara en el mapa de `96 × 32`.
- El Príncipe Perdido y el Bandido tienen retratos provisionales distinguibles y
  compatibles con el solapamiento del frontend.
- La única conducta inicial del Cazador es una salida de exploración y no
  contradice su papel narrativo amistoso.
- Los datos concretos respetan el fixture técnico sincronizado con este brief.
