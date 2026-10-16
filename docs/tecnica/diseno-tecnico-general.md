# Desert Tiger Thief RPG — Diseño técnico general

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 0.8 |
| Fecha | 2026-10-15 |

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
| `desert-tiger-core` | Define `Mundo` y las reglas. Es la única autoridad que valida y modifica el estado reglado. No conoce terminal, tiles, filesystem ni Lua. |
| `desert-tiger-app` | Define y posee la `Partida`, coordina los componentes y ofrece una fachada independiente del frontend. Contiene la única instancia concreta de `Mundo` y el estado actual del mapa. |
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

## 3. Vocabulario técnico

- **Mundo:** único valor que contiene el estado sujeto a reglas. Su tipo y sus
  operaciones se definen en `desert-tiger-core`.
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

Definir el vocabulario aquí no traslada los contratos detallados de sus
documentos canónicos.

El backend puede producir una clave semántica opcional para una escena
narrativa. La fachada la reenvía sin convertirla en una ruta o en arte. Cada
frontend decide si la ignora o la asocia con un recurso propio.

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
- Frontend y aplicación se comunican mediante intenciones, vistas y resultados
  con significado del juego, nunca mediante widgets o teclas.
- Jugador y NPC atraviesan el mismo motor y las mismas validaciones de acciones.
- Lua solo consulta un contexto autorizado y devuelve una solicitud declarativa;
  no modifica el Mundo ni evita la validación del núcleo.
- Los archivos editables se validan antes de crear una Partida. Las definiciones
  validadas permanecen inmutables y dos Partidas evolucionan de forma
  independiente.
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
| Reglas, narrativa, consecuencias y duración de `Viajar` | [`acciones/01-viajar.md`](./acciones/01-viajar.md) |
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
- una prueba sin frontend pueda recorrer una acción, el avance temporal y la
  fase NPC mediante la fachada de la aplicación;
- jugador y NPC utilicen el mismo motor;
- dos Partidas creadas desde el mismo contenido puedan evolucionar de manera
  independiente sin modificar sus definiciones fuente.

Los casos concretos, errores y criterios de aceptación adicionales se definen en
cada documento técnico específico.
