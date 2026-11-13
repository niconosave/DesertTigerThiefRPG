# Desert Tiger Thief RPG — Diseño técnico general

| Campo | Valor |
|---|---|
| Estado | Implementado; audio en verificación auditiva; detalles históricos señalados en sus componentes |
| Versión | 0.23 |
| Fecha | 2026-11-06 |

> El [contrato implementado de Viajar](./acciones/06-implementacion-viajar.md)
> sustituye sus contratos anteriores. Las cabeceras de los componentes delimitan
> los apartados históricos. La revisión estética con el usuario sigue pendiente.

## 1. Propósito

Este documento define únicamente las decisiones técnicas que atraviesan todo el
proyecto: las capas del programa, su dirección de dependencia, la autoridad
sobre cada clase de estado y los contratos obligatorios entre sistemas.

Los tipos, APIs, formatos, comportamientos y pruebas concretos pertenecen a los
[documentos técnicos específicos](./componentes/README.md). La
[metodología de desarrollo](../metodologia-desarrollo.md) determina cómo se
aprueban, implementan y verifican.

## 2. Arquitectura del programa

El primer incremento y las versiones inmediatas serán un único programa local
ejecutado en un solo proceso.
La workspace tendrá tres crates y una sola dirección de dependencias:

```text
desert-tiger-terminal ──► desert-tiger-app ──► desert-tiger-core
```

Las flechas representan dependencias y llamadas Rust dentro del proceso. No
representan servicios separados y no habrá REST, HTTP, sockets ni comunicación
entre procesos.

| Crate | Responsabilidad transversal |
|---|---|
| `desert-tiger-core` | Define `Mundo`, las reglas y el proyecto semántico transitorio de cada acción general. Es la única autoridad que valida y modifica el estado reglado. No conoce terminal, prosa visible, tiles, filesystem ni Lua. |
| `desert-tiger-app` | Define y posee la `Partida`, coordina los componentes, redacta vistas narrativas desde registros semánticos neutrales y ofrece una fachada independiente del frontend. Contiene la única instancia concreta de `Mundo` y el estado actual del mapa. |
| `desert-tiger-terminal` | Es el ejecutable inicial. Traduce entrada física a intenciones semánticas, posee el arte concreto de viñetas y representa vistas y resultados. No ejecuta reglas. |

`desert-tiger-terminal` dependerá de la fachada pública de
`desert-tiger-app`, no directamente de `desert-tiger-core`. Esa fachada no
expondrá tipos de terminal ni obligará a conocer la organización interna del
núcleo.

Otro frontend Rust podrá sustituir al crate de terminal y reutilizar la misma
aplicación y las mismas reglas. Un engine escrito en otro lenguaje necesitará un
adaptador o FFI, pero no una segunda implementación del gameplay.

La organización física, el toolchain y los controles de dependencias pertenecen
a [Workspace, entorno y límites de código](./componentes/01-workspace-entorno.md).

La ampliación de [audio procedural](./componentes/17-audio-procedural.md)
pertenece exclusivamente al frontend. Conserva las tres crates y el mismo
proceso: el callback de salida procesa sonido independientemente del bucle de
entrada y dibujo, sin acceder a la Partida ni ejecutar reglas del juego.

La ampliación implementada de [eventos narrativos](./componentes/23-eventos-narrativos.md)
añade un coordinador en `desert-tiger-app`: conserva las ejecuciones suspendidas y
aplica la autoría de eventos mediante operaciones del núcleo. El núcleo ofrece
puntos semánticos de cesión y conserva la validación de las continuaciones; no
interpreta Lua ni el contenido de un evento concreto. El frontend recibe las
vistas resultantes con el mismo límite de dependencias.

La [evolución común de eventos](./componentes/24-agenda-eventos-mundo.md) está
diseñada y pendiente de implementación. Normaliza la autoría externa, la autoría
incrustada en Actores y las políticas heredadas en un catálogo, un coordinador
y un registro de ocurrencias por partida. Puntos, fechas y fase NPC seleccionan
candidatos del mismo modelo. La referencia Lua `this` identifica al propietario
del contenido, separada del personaje controlado. Las acciones solicitadas siguen
usando el motor del núcleo; no se agrega un motor narrativo paralelo.

## 3. Vocabulario técnico

- **Mundo:** único valor que contiene el estado sujeto a reglas. Su tipo y sus
  operaciones se definen en `desert-tiger-core`.
- **Relación:** vínculo reglado entre dos elementos identificados del Mundo. El
  contenido la declara junto a su origen; el núcleo conserva una sola instancia
  y permite recorrerla mediante índices, sin convertir el archivo `world.ron`
  en una lista central de vínculos.
- **Partida o sesión:** agregado poseído por `desert-tiger-app` que contiene el
  Mundo, el mapa de esa ejecución y los estados necesarios para coordinarla.
- **definiciones de presentación:** datos inmutables cargados desde contenido,
  como la matriz codificada, su leyenda de tipos visuales, nombres, coordenadas y
  retratos de celdas de Actor; estos últimos pueden incorporar glifos de una
  columna y colores ANSI neutrales, pero no tipos de Ratatui. No incluyen glifos
  concretos de tiles ni arte de viñetas.
- **estado del mapa:** copia mutable de la matriz de `ClaveTipoTile`
  perteneciente a una Partida; sus apariencias concretas pertenecen a cada
  frontend.
- **frontend:** adaptador que recoge entrada y presenta vistas; inicialmente,
  `desert-tiger-terminal`.
- **backend:** cuando aparezca en el diseño general, significa el conjunto
  formado por `desert-tiger-app` y `desert-tiger-core`. Los documentos técnicos
  preferirán nombrar el crate responsable.
- **situación o escena dramática:** unidad narrativa contextual organizada por
  una acción general. Puede contener varias acciones particulares y varias
  viñetas; no es otra regla mecánica ni corresponde a un verbo reutilizable.
- **proyecto de acción (vocabulario histórico de Viajar 04):** estructura semántica transitoria mediante la que una
  acción general conserva qué quieren conseguir sus participantes, qué
  posibilidades consideran, con cuál se comprometen, qué subobjetivo persiguen
  y cómo interpretan los resultados antes de continuar, replantear o concluir.
  No es un elemento persistente del Mundo ni una simulación general de la mente.
- **registro de proyecto (histórico):** hecho neutral y ordenado de propósito, propuesta,
  compromiso, subobjetivo, interpretación o replanteo emitido por el núcleo.
  Puede sustentar una viñeta, pero no afirma que se haya ejecutado una acción
  particular y por ello no crea un momento causal ficticio. La conclusión es un
  registro cronológico separado que cierra el proyecto; el propósito es su única
  apertura semántica.
- **momento causal (histórico):** vínculo ordenado entre una parte de la escena y una
  ejecución particular, con los hechos necesarios para reconocer qué se
  intentó, por qué se resolvió así y qué consecuencias produjo.
- **viñeta:** fragmento de presentación de una escena. Contiene texto y datos
  neutrales, nunca widgets; no tiene cardinalidad fija respecto de acciones
  particulares.

Definir el vocabulario aquí no traslada los contratos detallados de sus
documentos canónicos.

El núcleo puede producir registros de proyecto y claves semánticas para
situaciones y momentos. La aplicación los convierte en texto contextual y
conserva sus referencias semánticas en la vista. Cada frontend decide si las
ignora o las asocia con una clave visual y un recurso propios; la fachada nunca
entrega rutas, bytes de arte ni una clave perteneciente a un frontend concreto.

## 4. Contratos transversales

- El estado reglado tiene una sola fuente de verdad: la instancia de `Mundo`.
- La Partida contiene esa instancia; no existe otra copia dentro del núcleo.
- El mapa es estado de presentación poseído por la aplicación. `Mundo` solo
  conserva su identidad opaca y nunca consulta tiles o coordenadas.
- Los símbolos ASCII de la matriz RON son una codificación de autoría. La
  leyenda los resuelve a valores `ClaveTipoTile` y cada frontend representa esas
  claves con recursos propios.
- Un cambio reglado puede causar después una actualización visual. El mapa nunca
  causa por sí mismo un cambio de gameplay ni es el único registro de una
  condición que las reglas deban observar.
  El rediseño de `Viajar` introduce una excepción provisional explícita: la
  aplicación deriva el rumbo de una conexión desde los centros visuales de
  sus Lugares y entrega solo ese rumbo al núcleo. El contrato técnico `06`,
  sección 6.4, define el adaptador. No se usan tiles ni distancias visuales
  para deducir conexiones o duración base.
- Frontend y aplicación se comunican mediante intenciones, vistas y resultados
  con significado del juego, nunca mediante widgets o teclas.
- Jugador y NPC atraviesan el mismo motor y las mismas validaciones de acciones.
- Lua consulta contextos autorizados y devuelve solicitudes declarativas; no
  modifica directamente el Mundo ni evita la validación del núcleo. Las políticas
  NPC solo solicitan acciones; los eventos narrativos podrán solicitar lotes de
  efectos mediante su API separada, según el contrato 23.
- Los archivos editables se validan antes de crear una Partida. Las definiciones
  validadas permanecen inmutables y dos Partidas evolucionan de forma
  independiente.
- Las Relaciones se resuelven a IDs internos durante la carga y se consultan
  mediante el índice del Mundo. Ninguna regla descubre conexiones recorriendo
  tiles, coordenadas o todos los archivos en tiempo de ejecución.
- Las acciones generales coordinan objetivos y decisiones; las acciones
  particulares resuelven intentos concretos mediante reglas reutilizables. Una
  dificultad se deriva del contexto y no se serializa como conclusión del
  contenido.
- Toda acción general que represente una empresa humana se organiza mediante un
  proyecto de acción dentro del núcleo. Su estado y sus registros son
  semánticos: la aplicación redacta y el frontend presenta, sin que ninguna de
  esas capas reconstruya propósitos, planes o reglas a partir de prosa.
- `Viajar` conserva su estado transitorio en una ejecución reanudable de
  tarjetas. Cada desplazamiento confirma dos chequeos, consecuencias, consumo
  y tiempo; la aplicación redacta desde el registro semántico confirmado.
- Las acciones particulares reciben responsable y dificultad del contexto;
  calculan un d20 y modificadores. Las tarjetas poseen duración y consecuencias.
  Caballo y camello comparten `Cabalgar` y mantienen viñetas independientes.
- La disponibilidad provisional de monturas y barcos sigue el reglamento de
  `Viajar`. No exige puertos ni inventario persistente. Obtener medios mediante
  economía y mercados queda aplazado.
- Una escena puede contener varios chequeos y páginas; avanzar páginas no
  ejecuta nuevas reglas. No existe un presupuesto de escenas ni se omiten
  conexiones de la ruta adoptada.
- El viaje acumula horas exactas. La aplicación proyecta y confirma una sola
  fecha final mediante el puente de fracción de día, conservando el resto para
  la acción siguiente. Las acciones NPC no confirman avance mundial.
- Un hecho de escena que afecte una resolución debe estar incluido en el
  contexto reglado antes de resolverla. La aplicación redacta desde hechos
  recibidos y el frontend terminal no inventa causas ni contexto mecánico.
- Una secuencia de consecuencias persistentes se valida y aplica como lote
  atómico. El frontend solo observa cambios ya confirmados.
- El azar, el reloj real y otras entradas variables se proporcionan de manera
  controlable para permitir pruebas reproducibles.
- Una decisión se define normativamente en un solo documento. Los demás pueden
  resumir el contexto y deben enlazar su fuente canónica.

## 5. Fuentes canónicas

| Tema | Documento normativo |
|---|---|
| Workspace, toolchain, crates y dependencias | [`01-workspace-entorno.md`](./componentes/01-workspace-entorno.md) |
| `Mundo`, actores, lugares, grupo, características e IDs internos | [`02-modelo-mundo.md`](./componentes/02-modelo-mundo.md) |
| Archivos, DTO, validación, catálogos e instanciación | [`03-contenido.md`](./componentes/03-contenido.md) |
| `Partida`, fases, fachada, vistas y errores de sesión | [`04-sesion-aplicacion.md`](./componentes/04-sesion-aplicacion.md) |
| Solicitudes, decisiones, resultados, atomicidad y azar de acciones | [`05-motor-acciones.md`](./componentes/05-motor-acciones.md) |
| Fecha, calendario, duración y avance temporal | [`06-calendario-duracion.md`](./componentes/06-calendario-duracion.md) |
| Orden, límites, ejecución y resúmenes de NPC | [`07-fase-npc.md`](./componentes/07-fase-npc.md) |
| Runtime, API, capacidades y errores Lua | [`08-integracion-lua.md`](./componentes/08-integracion-lua.md) |
| Modos, entrada, layout, Ratatui y renderizado | [`09-frontend-terminal.md`](./componentes/09-frontend-terminal.md) |
| Audio del frontend, dispositivo, activación y errores | [`17-audio-procedural.md`](./componentes/17-audio-procedural.md) |
| Timbre y parámetros editables del crepitar de la fogata | [`18-crepitar-fogata.md`](./componentes/18-crepitar-fogata.md) |
| Paisajes sonoros y selección por características y conexiones de Lugares | [`19-paisajes-sonoros.md`](./componentes/19-paisajes-sonoros.md) |
| Música de los integrantes al acampar y notas animadas coordinadas | [`20-musica-campamento.md`](./componentes/20-musica-campamento.md) |
| Relaciones, extremos, índices, consultas y semántica de `conectado` | [`10-relaciones.md`](./componentes/10-relaciones.md) |
| Operaciones persistentes y lotes atómicos | [`11-operaciones-mundo.md`](./componentes/11-operaciones-mundo.md) |
| Relación `Conoce` y proyección de hechos conocidos | [`12-conocimiento-lugares.md`](./componentes/12-conocimiento-lugares.md) |
| Contrato histórico obsoleto de la primera versión de `Viajar` | [`acciones/01-viajar.md`](./acciones/01-viajar.md) |
| Antecedente obsoleto de `Viajar` por expediciones | [`acciones/02-viajar-expediciones.md`](./acciones/02-viajar-expediciones.md) |
| Antecedente de cronología causal de `Viajar` | [`acciones/03-cronologia-causal-viaje.md`](./acciones/03-cronologia-causal-viaje.md) |
| Antecedente de proyecto humano de `Viajar` | [`acciones/04-proyecto-humano-viajar.md`](./acciones/04-proyecto-humano-viajar.md) |
| Contrato implementado de `Viajar` por tarjetas | [`acciones/06-implementacion-viajar.md`](./acciones/06-implementacion-viajar.md) |
| Reglas reutilizables de cada acción particular | [`acciones/particulares/`](./acciones/particulares/README.md) |

Una decisión limitada a uno de estos sistemas se registra exclusivamente en su
documento. Este diseño general solo cambia cuando una decisión afecta a varias
capas o altera sus fronteras.

La proyección futura de resultados semánticos sobre regiones del mapa recibirá
su propio documento cuando forme parte de un incremento; no es necesaria para
el primero.

## 6. Verificación de la arquitectura

La arquitectura se considerará respetada cuando:

- el grafo interno mantenga únicamente `terminal → app → core`;
- el núcleo pueda compilarse y probarse sin filesystem, Lua ni terminal;
- ninguna regla reciba tiles, coordenadas, teclas o tipos gráficos;
- la alcanzabilidad y las rutas entre Lugares procedan exclusivamente de
  Relaciones resueltas e indexadas dentro del Mundo;
- una prueba sin frontend pueda recorrer una acción, el avance temporal y la
  fase NPC mediante la fachada de la aplicación;
- jugador y NPC utilicen el mismo motor;
- jugador y NPC recorran la misma estructura de proyecto de acción; en el
  alcance inicial solo cambia que la fase NPC elige al azar entre las opciones
  semánticas vigentes;
- una prueba pueda reconstruir, sin analizar prosa, el propósito, las propuestas,
  el compromiso, los subobjetivos, las interpretaciones y los replanteos, y
  distinguir la conclusión cronológica que cierra el viaje;
- una prueba pueda asociar todas las trazas particulares de una acción general
  con momentos causales sin exigir una escena por traza ni perder su orden;
- ningún registro de proyecto sin ejecución particular produzca un momento
  causal artificial;
- dos Partidas creadas desde el mismo contenido puedan evolucionar de manera
  independiente sin modificar sus definiciones fuente.

Los casos concretos, errores y criterios de aceptación adicionales se definen en
cada documento técnico específico.
