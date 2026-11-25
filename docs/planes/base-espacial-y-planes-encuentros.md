# Etapa de base espacial y planes de encuentros

| Campo | Valor |
|---|---|
| Estado | Implementación y verificación técnica completas; revisión jugable pendiente |
| Fecha | 2026-11-24 |
| Alcance | Sustitución directa del modelo de encuentros y traducción del contenido actual |

La etapa implementa las [mecánicas de espacio, planes y sistemas](../diseno/encuentros-espacio-planes-y-sistemas.md)
mediante el [contrato técnico de la nueva base](../tecnica/acciones/12-base-espacial-y-planes-encuentros.md).
Es un único resultado jugable dividido en bloques internos dependientes.

## Bloques y dependencias

| Bloque | Resultado técnico | Depende de |
|---|---|---|
| [PLAN-2026-052](./pendientes/2026-11-24-52-modelo-espacial-tridimensional.md) | modelo espacial tridimensional y contexto mínimo. | Ninguno |
| [PLAN-2026-053](./pendientes/2026-11-24-53-planes-y-comportamientos.md) | planes genéricos y comportamientos de encuentro. | 052 |
| [PLAN-2026-054](./pendientes/2026-11-24-54-traducir-escenarios-y-puerta.md) | escenarios traducidos y primera puerta interactiva. | 052 y 053 |
| [PLAN-2026-055](./pendientes/2026-11-24-55-vista-y-corte-por-altura.md) | vista segura y corte visual por altura. | 052 |
| [PLAN-2026-056](./pendientes/2026-11-24-56-integrar-encuentros-y-consecuencias.md) | integración de encuentros y consecuencias. | 053, 054 y 055 |
| [PLAN-2026-057](./pendientes/2026-11-24-57-verificar-base-de-encuentros.md) | verificación y revisión jugable de la nueva base. | 056 |

El bloque 055 puede avanzar después del 052, independientemente del 053. El resto
sigue las dependencias de la tabla. La numeración no obliga a paralelizar ni a
ejecutar pruebas de partes que todavía no están integradas.

## Transición y cierre

Solo existirá el modelo nuevo. Escenarios, consultas, contenido narrativo y
representación se traducen a él; no se construyen adaptadores temporales ni un
segundo motor para conservar los formatos o las distribuciones anteriores.

El usuario autorizó que los encuentros queden temporalmente sin funcionar durante
el trabajo interno. Se registran las comprobaciones acotadas de cada bloque, pero
su resultado no se presenta como una entrega jugable independiente. La prueba
integrada se realiza al completar el 056 y se revisa jugando en el 057.

Los planes permanecen en pendientes mientras la etapa no cumpla sus criterios de
cierre. Se completa el resultado real de cada archivo y se archivan conjuntamente
cuando concluyan integración, comprobaciones técnicas y revisión jugable. Si un
bloque se difiere o cambia de alcance, se registra la decisión; no se lo marca
completo por haber terminado los demás.

## Límite de esta etapa

Se conserva el repertorio actual de acciones, una puerta manual nueva, los
eventos existentes adaptados, posesión del tesoro por el líder y consecuencias
causales. Crear o componer planes desde Lua, sigilo detallado, combate dinámico,
equipo nuevo, generación modular compleja y mapas locales persistentes se diseñan
y planifican posteriormente.

La [verificación acotada](../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#15-verificación-acotada)
selecciona controles por frontera modificada. Una batería más amplia requiere
explicar por qué es necesaria y acordar una ejecución que evite salida y
repeticiones innecesarias.


## Comprobaciones previstas por bloque

Los filtros siguientes se crearán con la implementación. Se ejecutan cuando el
crate afectado compile; una comprobación bloqueada por la integración se registra
como pendiente. No se mantiene código anterior para hacerla pasar artificialmente.
Cada filtro se utiliza cuando aporta evidencia nueva, sin repetir toda la tabla
al terminar cada bloque.

| Bloque | Comandos seleccionados previstos |
|---|---|
| 052 | cargo test -q -p desert-tiger-core espacio_local; filtro contexto_local por separado. |
| 053 | cargo test -q -p desert-tiger-core planes_locales. |
| 054 | cargo test -q -p desert-tiger-core puerta_local; filtro generacion_local por separado. |
| 055 | cargo test -q -p desert-tiger-terminal --no-default-features corte_altura. |
| 056 | cargo test -q -p desert-tiger-app consecuencias_locales; filtro eventos_locales_3d por separado. |
| 057 | cargo test -q -p desert-tiger-terminal --no-default-features integracion_encuentros_3d y regresiones concretas que aún falten. |

La salida detallada se guarda fuera del repositorio y se informa el resultado
breve. Formato y Clippy se aplican a los crates modificados al completar código
compilable. Esta planificación no afirma que se hayan ejecutado pruebas nuevas.

## Estado al terminar el bloque 056 (2026-11-25)

Los bloques 052 a 056 están completados técnicamente. El cierre distingue los
resultados de cada participante y tesoro, ofrece una decisión de evacuación y
confirma consecuencias sin repetirlas al cerrar. Se conserva un único motor de
encuentros. La etapa sigue abierta hasta la revisión integrada y jugable del
[057](./pendientes/2026-11-24-57-verificar-base-de-encuentros.md); los planes no se
archivan antes de ese cierre.

## Verificación técnica del 057 (2026-11-25)

Se completaron los recorridos integrados y la compilación del programa. La
[guía de revisión](./revision-jugable-base-encuentros.md) reúne los lugares,
controles y consecuencias a observar. No quedan tareas de implementación
identificadas en esta revisión; falta registrar la prueba del usuario para
cerrar y archivar conjuntamente los bloques. Las ampliaciones de sistemas de
combate y generación modular continúan fuera de esta etapa.

## Verificación integrada 057 (2026-11-25)

Se comprobaron los recorridos de la terminal y sus controles sobre el programa
integrado. El rescate cubre un ascenso y una evacuación reales entre alturas;
el ataque cubre puerta y conclusión; el robo conserva el bien extraído. Se
completó el diagnóstico tipado del núcleo y se comprobó el arranque y salida del
binario normal en una terminal PTY, sin audio durante esa comprobación.

La [guía de revisión jugable](./revision-jugable-base-encuentros.md) deja preparada
la parte que corresponde al usuario. Esa respuesta es el único requisito de
aceptación pendiente para archivar juntos los seis planes; no quedan tareas de
implementación identificadas dentro del alcance de la etapa.
