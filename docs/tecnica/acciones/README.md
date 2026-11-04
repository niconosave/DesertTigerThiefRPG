# Diseños técnicos de acciones

Esta carpeta contiene la fuente normativa de las reglas concretas de cada acción
general. El [motor común](../componentes/05-motor-acciones.md) define cómo se
ejecutan acciones, decisiones y resultados; estos documentos definen qué hace
cada acción.

| Documento | Alcance |
|---|---|
| [`01-viajar.md`](./01-viajar.md) | Contrato histórico obsoleto de la primera versión. |
| [`02-viajar-expediciones.md`](./02-viajar-expediciones.md) | Antecedente obsoleto de expedición física: etapas, preparación, supervivencia y medios. |
| [`03-cronologia-causal-viaje.md`](./03-cronologia-causal-viaje.md) | Antecedente obsoleto para componer situaciones dramáticas de viaje, preservar cada momento causal y presentar el progreso. |
| [`04-proyecto-humano-viajar.md`](./04-proyecto-humano-viajar.md) | Antecedente obsoleto de propósito, deliberación, plan, subobjetivos, procedencia de medios, interpretación y replanteamiento. |
| [`06-implementacion-viajar.md`](./06-implementacion-viajar.md) | Contrato implementado del refactor: módulos, cálculos, ejecución por escenas e integración. |
| [`07-acampar.md`](./07-acampar.md) | Contrato implementado: acampada, fichas, navegación, contenido y vista terminal. |
| [`08-planificacion-rutas.md`](./08-planificacion-rutas.md) | Implementado: viaje directo sin planificación, propuestas según los Actores y previsualización del recorrido en el mapa. |
| [`09-cruce-pasos-montanosos.md`](./09-cruce-pasos-montanosos.md) | Implementado: un episodio para pasos consecutivos, cuatro obstáculos, tiempo de travesía y campamento central en secuencias impares. |
| [`borradores/05-viajar-redisenio.md`](./borradores/05-viajar-redisenio.md) | Reglamento aprobado de `Viajar`, conservado en su ruta para mantener referencias. |
| [`borradores/CONTEXTO-viajar-redisenio.md`](./borradores/CONTEXTO-viajar-redisenio.md) | Traspaso histórico de la etapa anterior al motor actual. |
| [`borradores/CONTEXTO-viajar-territorio-segunda-iteracion.md`](./borradores/CONTEXTO-viajar-territorio-segunda-iteracion.md) | Traspaso activo para diseñar una segunda iteración de Viajar y Lugares, centrada en territorio y planificación. |
| [`borradores/INVESTIGACION-viajes-territorio-relatos-miticos.md`](./borradores/INVESTIGACION-viajes-territorio-relatos-miticos.md) | Borrador independiente sobre territorio, paisaje y viajes largos en cuatro relatos míticos; no define reglas del juego. |
| [`particulares/`](./particulares/README.md) | Índice vigente de las cinco pruebas y archivos históricos sustituidos. |

Para el motor implementado, las mecánicas pertenecen al reglamento de
`borradores/05-viajar-redisenio.md` y su traducción a código al contrato `06`.
La planificación vigente se define en el contrato `08`, que sustituye únicamente
las secciones que enumera y conserva el resto del reglamento y del contrato `06`.
Los documentos `01` a `04` describen alcances anteriores: no deben utilizarse
para completar reglas del refactor. La sección 16 del contrato `06` delimita los
cambios sobre componentes compartidos. Ambos documentos describen el motor actual; el informe de
[verificación](./verificacion-viajar-reglamento.md) registra el cierre técnico. El nombre histórico de la carpeta `borradores` no cambia su estado.

Una acción nueva requiere su diseño aprobado antes de incorporarse a un plan de
implementación. Los documentos no repiten los contratos del motor, el Mundo o la
fachada: los enlazan y solamente definen sus reglas particulares.

Una acción particular que solo sea una operación interna trivial puede quedar
descrita dentro de su acción general mientras tenga un único consumidor. En el
momento en que posea reglas propias o se reutilice, requiere un documento en
`particulares/`; ese documento pasa a ser su única fuente normativa.
