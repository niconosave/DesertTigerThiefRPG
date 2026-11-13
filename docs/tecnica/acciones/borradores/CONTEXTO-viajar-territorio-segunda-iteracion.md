# Contexto: segunda iteración de Viajar y los Lugares

| Campo | Valor |
|---|---|
| Fecha | 2026-10-27 |
| Estado | Traspaso de contexto; nuevo diseño todavía abierto |
| Proyecto local | `/home/niconosave/Development/DesertTigerThiefRPG` |
| Tarea siguiente | Diseñar junto con la persona usuaria una segunda iteración de Viajar y revisar la disposición y el papel de los Lugares. |

Este archivo permite continuar en otra conversación sin reconstruir el historial.
Separa **objetivos confirmados**, **funcionamiento actual** y **propuestas sin
aprobar**. No es el reglamento nuevo ni un plan de implementación. La solicitud
actual autoriza preparar este traspaso; todavía no se pidió desarrollar la
segunda iteración.

## 1. Objetivo de diseño confirmado

El juego es un sandbox RPG en Rust, inicialmente para terminal con Ratatui, con
partidas que apuntan a unas cinco o seis horas reales. Viajar ya funciona y la
persona usuaria está conforme con su velocidad y su legibilidad actuales.

Ahora quiere que **el territorio se vuelva significativo mediante las mecánicas**.
Un río, una montaña o un bosque deben influir en lo que el grupo puede hacer,
necesita y experimenta. El jugador debería aprender a reconocer esas diferencias
y aprovecharlas al preparar sus viajes. También interesan las oportunidades:
agua, alimento, refugio y referencias para orientarse, además de dificultades.

La experiencia buscada tiene dos momentos:

- **Antes de partir:** el jugador conoce el recorrido en la medida que permita
  el conocimiento del grupo, observa sus partes complicadas y evalúa requisitos,
  preparativos y posibilidades de conseguir lo que falta. Aquí se concentra la
  decisión importante de si emprender ese viaje y por dónde hacerlo.
- **Durante el viaje:** el grupo resuelve principalmente de forma automática
  las dificultades, con chequeos y consecuencias reales. El jugador presencia
  lo sucedido y toma alguna decisión ocasional. No se busca convertir cada
  desplazamiento en una sucesión de menús.

El objetivo incluye revisar las conexiones y crear Lugares basados en el mapa
para representar barreras, pasos, rodeos y accesos condicionados. Se busca
profundidad con pocas reglas reutilizables, evitando una proliferación de
excepciones. Aumentar encuentros aleatorios, por sí solo, no resuelve el objetivo.

## 2. Motivación e inspiración aportadas por la persona usuaria

- En su lectura de *El Señor de los Anillos*, destaca la continuidad geográfica:
  nombres y posición de montes, recorridos, referencias y dificultades. Los
  conocimientos de los personajes conectan naturaleza, historia y personas.
- Su ejemplo de *La guerra de las Galias* enfatiza el río como barrera y cómo
  la capacidad de construir puentes puede cambiar las posibilidades de atravesarlo.

Son inspiraciones de diseño, no solicitudes de reproducir acontecimientos
históricos ni de implementar construcción de puentes. Interesa que un mismo
territorio plantee posibilidades distintas a grupos con conocimientos y medios
distintos. La experiencia humana y su dramatismo se expresan mediante el contexto
y la narración; no se pidió simular el pensamiento de los personajes.

## 3. Acuerdos que deben respetarse al discutir el rediseño

- **Un programa con separación interna estricta:** `terminal → app → core`.
  El núcleo conserva las reglas; la aplicación coordina y redacta; el frontend
  dibuja y recoge elecciones. No son servicios ni procesos separados.
- **Mapa visual, ubicación lógica en Lugares:** los Actores no están en tiles.
  Las coordenadas ilustran los Lugares y permiten enfocar la cámara; no crean
  conexiones ni determinan por sí mismas el movimiento. No introducir
  pathfinding de tiles para resolver la disposición geográfica.
- **Relaciones simples:** se declaran en los RON de los elementos y al cargar
  se normalizan e indexan. `Conectado` es consultable en ambos sentidos. La
  persona usuaria no quiere añadir dificultad, duración o requisitos a ese enlace.
- **Los obstáculos pueden representarse mediante Lugares ordinarios:** para
  describir un paso entre A y B puede disponerse A—C—B y retirar A—B. Esto nunca
  significó crear un tipo especial de «Lugar intermedio».
- **Características descriptivas:** los hechos de un Lugar permiten que una
  acción deduzca sus consecuencias. Evitar etiquetas que solo almacenen un
  resultado abstracto, como «jornada larga», o una dificultad calculada del viaje.
- **Acciones particulares reutilizables:** la acción general o tarjeta determina
  contexto, responsable, dificultad, tiempo, consecuencias y relato. La particular
  resuelve el chequeo que recibe; no elige quién actúa ni conoce la escena.
  Actualmente caballo y camello comparten `Cabalgar`, con viñetas distintas.
- **Sin límite artificial de escenas o chequeos:** el ritmo se trabaja en la
  presentación y la organización de las situaciones, sin saltarse resoluciones
  que exijan las reglas. Una viñeta puede contener varias resoluciones relacionadas.
- **Autoría accesible:** un RON por Actor o Lugar; contenido comprensible y
  editable. Medios, conocimientos y preparativos se mencionan conceptualmente;
  eso no aprueba crear categorías nuevas ni un inventario con cientos de objetos.

El modelo de Lugares forma parte de la discusión nueva. Cualquier revisión de
estos acuerdos debe hacerse explícita, sin deducirla de un ejemplo.

## 4. Funcionamiento actual verificado

El motor vigente está implementado. Este resumen es descriptivo, no un conjunto
de reglas que la segunda iteración esté obligada a conservar.

| Aspecto | Estado actual |
|---|---|
| Rutas | Se calcula una rápida mediante tiempos de referencia y otra más larga. Esta última puede terminar pareciéndose a la rápida o ser idéntica si no se encuentra alternativa. |
| Planificación | `viajero`, `explorador` o conocer al menos la mitad de los Lugares de la ruta habilita proponer la rápida. El conocimiento local no ayuda todavía a resolver cada paso. |
| Medio | Se elige automáticamente por conexión: barco si alguno de los extremos es acuático; caballo, camello o pie según disponibilidad y reglas de terreno. |
| Disponibilidad de medios | Las monturas necesarias se conceden en la preparación; el barco aparece donde se necesita. No se exige actualmente conseguirlo en un puerto concreto. |
| Terreno | Montañoso, boscoso y pantanoso reducen la velocidad montada de la misma manera. No cambian por sí mismos las dificultades de los dados ni las tablas de incidentes. |
| Desierto y tamaño | `desierto` más `vasto` impide caballo. Ninguno de los 26 Lugares oficiales lleva hoy `vasto` ni `pequeno`, por lo que esa restricción y esos factores de tamaño no se activan aquí. |
| Desplazamientos | Cada conexión resuelve dos chequeos, correspondientes a sus dos extremos, con el responsable elegido para esa tarjeta. Puede conceder `Conoce`, producir incidentes o separar compañeros. |
| Abastecimiento | Buscar recursos usa dificultad 14 y seis horas en cualquier Lugar. Por éxito obtiene comida y/o agua sin comprobar recursos geográficos locales. |
| Tiempo y reservas | El viaje cuenta horas internamente y conserva fracciones al traducirlas al calendario. Comida, agua, carga y contadores de monturas son temporales del viaje; no existe un inventario persistente de estos recursos. |

Las capacidades con bonos actuales son `vagabundo`, `jinete`, `navegante`,
`administrador`, `expedicionario` y `herbalista`. El Bandido tiene además
`conoceDesiertos`, `camellero`, `guia` y `conCamellos`, pero esas cuatro etiquetas
no intervienen en las reglas vigentes. Sus Relaciones `Conoce` sí pueden habilitar
la planificación. Los rasgos `cazador`, `rastreador` y `superviviente` del Cazador
tampoco modifican hoy la búsqueda de recursos.

La tranquilidad observada tiene una causa concreta: montados y guiados por el
Bandido con `jinete +2`, los incidentes se activan en 21 de las 400 combinaciones
posibles de los dos dados: **5,25 % por conexión**, independientemente del terreno.
Este dato no implica que la persona usuaria haya pedido aumentar esa probabilidad.

## 5. Presentación ya implementada y aceptada

- Relato principal de una a tres oraciones en segunda persona, con situación,
  acontecimiento y consecuencias esenciales. El tono depende del contexto.
- `D` abre el desarrollo, tiradas, tiempos, consumo y reservas; funciona también
  en decisiones. Consultar o paginar Detalles no ejecuta reglas ni responde opciones.
- La cámara sigue el Lugar presentado y la interfaz muestra progreso temporal.
- El selector de destinos es una **lista lateral derecha**, ordenada por
  `nombre_abreviado`; `↑/↓` recorren, Enter confirma y Esc cancela. La cámara sigue
  el foco. La navegación espacial con cuatro flechas fue reemplazada.
- Los 26 RON de Lugares tienen `nombre_abreviado: "..."`. Omitirlo o dejarlo
  vacío utiliza el nombre completo. Mapa, fichas y relatos conservan el nombre
  completo; la lista ajusta su ancho a las abreviaturas.

La velocidad real y estas mejoras fueron probadas y aceptadas por la persona
usuaria. **Mostrar antes de partir un recorrido y sus requisitos de la manera
que pide ahora todavía requiere diseño**; no asumir que el selector actual ya
ofrece esa capacidad ni que se aprobó una interfaz concreta para ella.

## 6. Territorio y conexiones reales que motivan el trabajo

El mapa actual mide **192 × 64 tiles**. Presenta una región árida occidental,
una cordillera central casi vertical, un ramal montañoso al sudeste, un golfo
meridional, valle fértil nororiental, cursos de agua, bosque oriental y marisma
cerca de Nahal. Los datos actuales son la fuente para posiciones y conexiones.

Casos verificados en los RON:

1. **Paso central evitable por tierra:** Hamada Roja → Cabo Errante → Playa
   Dorada → Sierra Quebrada → Bosque de Tamrut → Senda de Tamrut → Valle del Drâa.
   El enlace Playa Dorada–Sierra Quebrada representa otra travesía sin un paso
   propio que describa cómo se realiza.
2. **Vado y calzada sí son obligatorios en el corredor terrestre:** Bosque de
   Tamrut → Vado de Tamrut → Marisma Verde → Calzada de Nahal → Nahal.
3. **Rodeo marítimo que evita ese corredor:** Bosque → Sierra → Playa Dorada →
   Golfo del Siroco → Mar de Levante → Estuario de Nahal → Nahal. La aparición
   automática de barcos reduce lo que exige prepararlo.

No resumir estos hechos como «no hay ningún cuello de botella». Hay algunos,
pero varias condiciones todavía no tienen mecánica y el acceso al mar permite
evitarlos fácilmente.

El contenido ya describe oportunidades sin uso en Viajar: el Valle del Drâa
tiene `aguaDulce`, `vegetacionComestible` y `faunaAbundante`; el Vado, agua dulce,
poca profundidad y corriente rápida; el bosque, sombra, agua y fauna; Nahal,
mercado, puerto y muelle. La Calzada tiene `pantanoso`, y hoy se ralentiza igual
que la marisma porque todavía no se interpreta el beneficio de su terraplén.

Al revisar la cartografía, no tomar la recta entre dos centros como el recorrido
real. Algunos centros y rótulos necesitan revisión editorial respecto de los
tiles actuales: por ejemplo Sierra Quebrada está centrada sobre agua. Esto no es
una invalidez del núcleo ni pide un validador que vigile el diseño artístico.

## 7. Propuestas conversadas, todavía SIN aprobar

El asistente propuso estas posibilidades para discutir; la persona usuaria solo
confirmó que quiere abordar un segundo diseño, no estas soluciones particulares:

- Conservar el corredor central por Paso Escarpado y explorar un collado más
  septentrional próximo al Espinazo Gris, alcanzado mediante un rodeo.
- Revisar Playa Dorada–Sierra Quebrada y decidir si representa una cornisa
  terrestre, otro paso o una travesía marítima.
- Separar sectores por margen cuando un Lugar demasiado extenso permita
  atravesar un río implícitamente; agregar otro cruce además del Vado.
- Los nombres **Puente del Drâa**, **Ribera de los Sauces**, **Camino aguas arriba**
  y cualquier collado nuevo son ejemplos provisionales. Ningún diagrama propuesto
  constituye una topología aprobada.
- Contrastar calzada firme con desplazamiento por humedales, y hacer depender
  la navegación de un acceso concreto a embarcaciones.
- Distinguir existencia física de paso, necesidades para intentarlo e
  incertidumbre resuelta por chequeos. Distinguir también observar una ribera,
  recorrerla y cruzar el cauce.
- Hacer que la planificación y la ejecución usen las mismas reglas, distinguiendo
  requisitos conocidos, cosas que se espera conseguir e incertidumbre.

**No están aprobados** dos caminos por cada barrera, un sistema de construcción
de puentes, habilidades nuevas concretas, costes, materiales, umbrales, reglas
de retirada, cambios específicos de conexiones ni el modo visual de anticipar rutas.

## 8. Forma de continuar y decisiones abiertas

La persona usuaria decide el diseño y no conoce Rust. Explicar implicaciones
técnicas en lenguaje común; no pedirle resolver decisiones internas del lenguaje.
Ayudar a redactar reglas precisas, como un reglamento de RPG de mesa, una vez
discutidas. Mantener separados hechos confirmados y pendientes; no rellenar
huecos copiando código o inventando sistemas.

Preguntas de diseño para abordar progresivamente, no todas a la vez:

1. ¿Qué regiones y márgenes representan los Lugares, y dónde hay pasos reales?
2. ¿Qué alternativas concretas merece la pena disponer en esta geografía?
3. ¿Qué procedimientos generales permiten atravesarlas y qué distingue sus problemas?
4. ¿Qué aportan el conocimiento de un entorno y el de un Lugar específico?
5. ¿Qué se puede preparar, dónde se consigue y qué sabe el jugador antes de salir?
6. ¿Qué consecuencias tienen éxito y fallo, cuándo puede seguir el grupo y cuándo debe replantear?
7. ¿Cómo se presenta y elige un recorrido sin alargar demasiado el viaje real?

Primero completar el diseño acordado; después traducirlo a documentación técnica,
planes con referencias y desarrollo jugable según la
[metodología](../../../metodologia-desarrollo.md). No crear todavía planes para
las propuestas de la sección 7 ni refactorizar la acción por este traspaso.

## 9. Fuentes para consultar sin releer todo el proyecto

**Al comenzar:** este archivo. Para entender las formas del territorio, mirar
[`assets/map.png`](../../../../assets/map.png), la matriz vigente en
[`mapa_inicial.ron`](../../../../content/base/maps/mapa_inicial.ron) y los
[`Lugares RON`](../../../../content/base/places/). Los tiles siguen siendo visuales.

**Cuando se necesite contrastar el motor actual:**

| Fuente | Para qué consultarla |
|---|---|
| [Reglamento 05](./05-viajar-redisenio.md) | Reglas vigentes, presentación y tarjetas. Está implementado aunque viva en `borradores`. |
| [Contrato 06](../06-implementacion-viajar.md) | Traducción a código y autoridad frente a diseños anteriores. |
| `crates/desert-tiger-core/src/acciones/viajar/rutas.rs` | Conexiones, tiempos, medios y cálculo de rutas. |
| `crates/desert-tiger-core/src/acciones/viajar/tarjetas/` | Planificación, preparación, desplazamientos y forrajeo. |
| `crates/desert-tiger-core/src/acciones/particulares/chequeos.rs` | Pruebas y modificadores reutilizables. |
| `crates/desert-tiger-app/src/narracion_viaje.rs` y `narracion_viaje/resumen.rs` | Desarrollo consultable y relato breve. |
| `crates/desert-tiger-terminal/src/interfaz.rs` | Selector lateral, cámara, viñetas y controles. |
| [Modelo de Mundo](../../componentes/02-modelo-mundo.md), [Relaciones](../../componentes/10-relaciones.md), [Contenido](../../componentes/03-contenido.md) | Contratos de Lugares, índices y RON. |
| [Atlas regional](../../../contenido/atlas-regional.md) y [trayectos](../../../contenido/trayectos-viaje.md) | Intención cartográfica y antecedentes del contenido. |

**Autoridad y trampas del historial:** los documentos de Viajar `01` a `04` y
`CONTEXTO-viajar-redisenio.md` son históricos. Los briefs también contienen
aspiraciones de versiones anteriores, como obtener barcos en puertos, que no
describen necesariamente el código actual. No usarlos para completar silenciosamente
reglas nuevas ni reabrir viejos fallos de tiempo y navegación ya superados.
El reglamento nuevo podrá reemplazar decisiones del 05 cuando la persona usuaria
lo acuerde; hasta entonces el 05 y el 06 describen la versión jugable.

Para jugar durante la futura implementación se recomienda
`cargo run --release --bin desert-tiger-terminal`. La música ya se sintetiza en
segundo plano después de mostrar el juego; acampar antes de que esté preparada
no bloquea la interfaz, y salir cancela el trabajo pendiente. Su cálculo sigue
siendo más lento en desarrollo. No se ejecutan tests al arrancar el juego.

## 10. Mensaje para pegar en la conversación nueva

```text
Quiero diseñar una segunda iteración de Viajar y revisar los Lugares de este juego.
El proyecto está en /home/niconosave/Development/DesertTigerThiefRPG.

Leé primero:
docs/tecnica/acciones/borradores/CONTEXTO-viajar-territorio-segunda-iteracion.md

Conservemos el ritmo breve y los viajes principalmente automáticos. Quiero que
la disposición del territorio, los pasos, los conocimientos y los preparativos
importen, y que las decisiones principales sucedan antes de partir.

Vamos a diseñarlo juntos. Las propuestas de lugares, conexiones y reglas del
traspaso todavía no están aprobadas. No implementes ni crees planes aún; ayudame
a construir un borrador de reglas precisas y mantenelo actualizado con nuestras
decisiones, dejando visibles los pendientes.

Consultá el mapa, los RON y las partes pertinentes del código cuando hagan falta.
Explicame las implicaciones en lenguaje común: puedo decidir diseño, pero no Rust.
Empecemos por la disposición del territorio y los problemas que queremos que
plantee atravesarlo, sin diseñar todos los sistemas de una vez.
```
