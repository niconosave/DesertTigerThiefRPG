# Plan: Refactorizar Viajar como proyecto humano

| Campo | Valor |
|---|---|
| ID | PLAN-2026-022 |
| Estado | Sustituido; implementación anterior conservada |
| Creado | 2026-10-19 |
| Actualizado | 2026-10-24 |

> Sustituido por la secuencia PLAN-2026-025 a PLAN-2026-031 del
> [refactor implementado](../completados/README.md#refactor-del-reglamento-de-viajar).
> Sus resultados y casillas conservan el historial; no se ejecutan nuevas tareas
> de este plan ni se lo declara completado por implementar el diseño nuevo.

## Resultado buscado

`Viajar` deja de ser un encadenamiento logístico de etapas y pasa a ejecutar un
proyecto humano reanudable. En el primer corte, el dirigente quiere alcanzar el
destino, los integrantes del Grupo pueden proponer maneras conocidas y posibles
de hacerlo, el dirigente compromete un plan y personas concretas ponen su
esfuerzo en sus subobjetivos. El Mundo responde mediante las acciones
particulares vigentes y la acción general interpreta cada resultado para
continuar, reformular o interrumpir el proyecto.

El refactor puede reemplazar por completo la orquestación interna actual de
`Viajar`. Conserva las acciones particulares, el Mundo, el grafo de Lugares y el
protocolo reanudable, pero no mantiene una ruta interna de compatibilidad con la
máquina sustituida. Ningún medio material se presupone: toda alternativa para
usarlo identifica una procedencia existente en el estado conocido.

## Referencias

- [Proyecto humano de Viajar](../../tecnica/acciones/04-proyecto-humano-viajar.md).
- [Expediciones y reglas físicas de Viajar](../../tecnica/acciones/02-viajar-expediciones.md).
- [Contrato de resolución particular](../../tecnica/acciones/particulares/00-contrato-resolucion.md).
- [Motor de acciones reanudable](../../tecnica/componentes/05-motor-acciones.md).
- [Sesión y fachada de aplicación](../../tecnica/componentes/04-sesion-aplicacion.md).
- Dependencia: [PLAN-2026-021](../completados/2026-10-19-21-conservar-trazas-causales.md).

### Contratos históricos que este plan migra

Los planes completados continúan siendo el registro fiel de la implementación
anterior. Este plan no los corrige retroactivamente; documenta qué parte de sus
resultados conserva o reemplaza.

| Plan completado | Tratamiento en el refactor |
|---|---|
| [PLAN-2026-014](../completados/2026-10-17-14-recorrer-expediciones-por-lugares.md) | Conserva etapas por Lugares reales, conexiones y traslado confirmado; reemplaza su orquestación superior. |
| [PLAN-2026-015](../completados/2026-10-17-15-incorporar-conocimiento-lugares.md) | Conserva el conocimiento limitado del Grupo y su no filtración; lo usa para formular propuestas. |
| [PLAN-2026-016](../completados/2026-10-17-16-preparar-consumir-provisiones.md) | Conserva reservas, consumo, preparación y ritmo; los subordina al plan elegido. |
| [PLAN-2026-017](../completados/2026-10-17-17-sobrevivir-acampar-ruta.md) | Conserva supervivencia, campamento, progresión y límites de intento como respuestas a necesidades del proyecto. |
| [PLAN-2026-018](../completados/2026-10-17-18-viajar-camellos-caravanas.md) | Conserva `Cabalgar` y `GuiarCaravana`; reemplaza la disponibilidad abstracta por una procedencia explícita. |
| [PLAN-2026-019](../completados/2026-10-17-19-incorporar-navegacion.md) | Conserva las acciones y crisis de navegación; elimina todo acceso implícito a una embarcación. |

## Alcance

### Incluido

- propósito inicial `AlcanzarDestino`, dirigente, participantes, propuestas,
  plan comprometido, subobjetivo vigente e interpretación del último resultado;
- deliberación finita y sin duplicados cosméticos en la que los compañeros
  pueden proponer alternativas justificadas por su conocimiento o capacidad y
  el dirigente toma la decisión, sin un máximo impuesto por el frontend;
- para el jugador, decisiones reanudables; para NPC, selección automática entre
  las mismas alternativas válidas;
- procedencia explícita de cada medio no intrínseco del Grupo durante todo el
  proyecto;
- primer vertical de procedencias oficiales: los camellos aportados mediante el
  Bandido con `conCamellos`, y el servicio local de bote o barco ofrecido por un
  `muelle` o `puerto` compatible;
- caminar como capacidad humana ordinaria, sin convertirlo en un Objeto o una
  procedencia artificial;
- exclusión de alternativas cuyo medio no existe, no es conocido o no está
  disponible desde una procedencia válida;
- estrategia, configuración de medios y procedencias comprometidas por la
  propuesta adoptada, con ruta, preparación y ritmo como partes revisables del
  plan y no como finalidad superior de la acción;
- elección de responsable y ayudantes, y asociación de cada acción particular
  ejecutada con el subobjetivo humano que intenta cumplir;
- interpretación de resultados, actualización del proyecto y reevaluación desde
  el último estado confirmado, sin reescribir lo sucedido;
- sustitución de la orquestación, estados, helpers y pruebas antiguas que hayan
  quedado incompatibles o duplicados;
- pruebas de migración que distingan contratos conservados de comportamientos
  expresamente reemplazados.

### No incluido

- conseguir un medio ausente mediante compra, préstamo, persuasión, captura,
  robo, intercambio u otra aventura de procura;
- dinero, economía, inventario detallado, animales o embarcaciones como entidades
  persistentes;
- conflictos internos, votación del Grupo, obediencia, lealtad o simulación del
  pensamiento de los Actores;
- estados numéricos de emoción, moral, esperanza o sufrimiento;
- nuevos propósitos de viaje, nuevas acciones particulares o cambios en sus
  reglas y balance;
- redacción definitiva de escenas, composición de la cronología o presentación
  de Ratatui.

## Precondiciones

- PLAN-2026-021 completado.
- El proyecto humano, las expediciones y el protocolo reanudable están aprobados
  en los documentos citados.
- El contenido oficial conserva al Bandido con `conCamellos` y los Lugares de
  prueba con `muelle` o `puerto` necesarios para comprobar las procedencias.

## Pasos

1. [x] Inventariar la máquina vigente y clasificar cada estado, transición y
       prueba como conservado, reemplazado o retirado según la tabla histórica.
2. [x] Introducir el estado reanudable del proyecto con propósito, dirigente,
       participantes, propuestas, plan comprometido, subobjetivo y estado de
       interpretación, sin dependencias de aplicación o terminal.
3. [x] Separar el preflight puro del inicio aceptado: si no existe ninguna
       propuesta válida, rechazar sin crear ejecución o proyecto, sin emitir
       propósito, sin consumir azar o tiempo y sin mutar el Mundo; después de
       aceptar, tratar la ausencia de continuación como interrupción.
4. [x] Construir propuestas únicamente desde conocimiento, capacidades,
       geografía y procedencias conocidas; conservar quién propuso cada una y
       por qué es posible sin exponer hechos ocultos, y entregar su referencia y
       carga semántica completa al protocolo de decisión.
5. [x] Ordenar de forma canónica todas las propuestas materiales distintas,
       hacer que el dirigente elija entre la lista completa y que los NPC
       seleccionen automáticamente entre exactamente esas mismas opciones.
6. [x] Comparar propuestas con el pronóstico provisional común `Normal` y
       `Ligera`; tras comprometer estrategia, medios y fuentes, construir de
       forma pura la tabla `ritmo × preparación`, elegir primero el ritmo y
       después preparación, reservas y ruta como un compromiso conjunto.
7. [x] Registrar en el plan la procedencia de camellos, bote o barco y retirar
       todo acceso implícito. En el preflight, filtrar cada propuesta cuya fuente
       falte y rechazar la solicitud solo si no queda ninguna; durante un viaje
       abierto, una pérdida causada abre replanteamiento o interrupción y una
       desaparición sin consecuencia previa es fallo fatal de invariante.
8. [x] Convertir el plan elegido en subobjetivos revisables y hacer que cada
       invocación particular reciba responsable, ayudantes, blanco, entorno y
       sentido procedentes del subobjetivo actual.
9. [x] Aplicar trazas, duración y consecuencias una sola vez; interpretar su
       efecto sobre el proyecto y continuar, reformular, abandonar durante una
       frontera segura de replanteamiento o interrumpir desde el último estado
       confirmado.
10. [x] Integrar el nuevo protocolo con la fachada y la fase NPC sin duplicar
       reglas ni avanzar dos veces la fecha.
11. [x] Retirar la máquina, accesos implícitos y helpers sustituidos; no conservar
       dos orquestadores activos ni una rama silenciosa de compatibilidad.
12. [x] Añadir pruebas unitarias, de integración y de migración, ejecutar los
        controles generales y sincronizar los estados documentales efectivos.

## Verificación

- [x] Todo viaje posee un propósito, un dirigente, un plan comprometido y un
      subobjetivo vigente antes de ejecutar una acción particular.
- [x] Una solicitud sin propuestas se rechaza antes de crear proyecto, emitir
      propósito, consumir azar o tiempo o mutar el Mundo; si las continuaciones
      desaparecen después de aceptarlo, el proyecto concluye como interrupción.
- [x] Una acción particular puede reutilizarse bajo proyectos distintos sin
      contener el propósito ni la prosa del viaje.
- [x] El Bandido puede proponer camellos por `conCamellos`; sin ese Actor o una
      procedencia equivalente, la opción no aparece ni se inventa.
- [x] Un `muelle` o `puerto` compatible puede ofrecer un servicio acuático y la
      procedencia queda registrada; comenzar en el agua conserva explícitamente
      `EmbarcacionActual`, mientras conocer una ruta no crea un medio.
- [x] El alcance vigente no ofrece procurar un medio ausente ni simula que fue
      comprado, capturado, pedido o robado fuera de una acción real.
- [x] Los compañeros proponen cuando aportan una alternativa válida, pero el
      dirigente toma la decisión; no se simulan acuerdos o conflictos internos.
- [x] Cada traza ejecutada puede enlazarse con el subobjetivo, responsable y
      plan que le dieron sentido, sin contaminar la traza neutral.
- [ ] Un fracaso o descubrimiento puede reformular el plan desde el último Lugar
      confirmado sin revertir tiempo, consumo, operaciones ni conocimiento ya
      aplicados.
- [ ] Un replanteamiento con cero continuaciones interrumpe sin decisión; con
      una ofrece continuar o abandonar; con varias ofrece todas en orden
      canónico y `Abandonar` al final.
- [x] Grupo, grafo, conocimiento, supervivencia, acciones particulares y
      protocolo reanudable conservan sus invariantes aprobadas.
- [x] No quedan dos máquinas activas ni una selección abstracta de medio que
      evite comprobar su procedencia.
- [x] `DestinoSinRutaConectada` distingue la falta de conexión topológica de
      `SinPropuestaViajeViable`; ninguna aparece después de crear el proyecto y
      durante una decisión solo se rechazan IDs de decisión u opción no vigentes.
- [x] Adoptar una propuesta fija sus medios previstos y procedencias;
      preparación y ritmo refinan el plan sin abrir una segunda selección
      redundante de medio.
- [ ] Las propuestas se comparan bajo `Normal` y `Ligera`; después de adoptar
      medios y fuentes se calcula sin mutaciones toda la tabla
      `ritmo × preparación`, se elige ritmo primero y cada preparación muestra y
      adopta su propia mejor ruta y reservas, sin consumir anticipadamente
      `bienAprovisionado`.
- [x] La opción semántica de cada propuesta transporta referencia estable,
      proponente, estrategia, medios, fuentes, pronóstico, requisitos y fuentes
      de preparación, beneficios, costes, incertidumbre y precondiciones
      visibles; la aplicación no reconstruye esos datos consultando otra vez al
      núcleo.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- El reemplazo puede ser amplio; se favorecerá una máquina coherente frente a
  conservar estructuras internas cuya única razón sea la implementación previa.
- La procedencia no equivale todavía a propiedad persistente. En este vertical,
  `conCamellos`, `muelle` y `puerto` justifican medios existentes; su adquisición
  cuando faltan pertenece a un incremento posterior.
- La deliberación es autoral y acotada: ordena posibilidades verificables, no
  intenta simular una mente ni generar planes generales de forma automática.
- Si el diseño aprobado cambia un contrato histórico, la prueba correspondiente
  debe afirmar el comportamiento nuevo y registrar expresamente la migración; no
  se altera el plan completado que documenta el estado anterior.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización técnica: 2026-10-20; cierre documental agrupado en PLAN-2026-020.
- Resultado: `Viajar` posee un `ProyectoViaje` reanudable con participantes,
  propuestas materiales, plan comprometido, subobjetivo, fuentes explícitas y
  la misma política de decisiones para jugador y NPC. La selección redundante
  de embarcación fue retirada.
- Verificaciones ejecutadas: preflight sin mutación, distinción entre conexión
  y propuesta viable, propuestas a pie/camello/bote/barco, procedencias
  semánticas, matriz completa del atlas, formato, check, Clippy estricto,
  pruebas del workspace y build release.
- Desviaciones respecto del plan: el Mundo vigente no puede eliminar conexiones
  ni medios durante una acción; por eso no se mantiene una máquina muerta de
  adquisición o pérdida de medios. Los fracasos actuales interpretan y
  concluyen desde el último Lugar confirmado. La decisión cardinal de
  replanteamiento se implementará junto con la primera regla capaz de invalidar
  materialmente un plan abierto. Estar ya en un Lugar acuático usa la
  procedencia explícita `EmbarcacionActual`, conforme a la decisión de permitir
  mares como destinos.
- Trabajo posterior: [PLAN-2026-023](./2026-10-19-23-emitir-cronologia-humana-viaje.md).
