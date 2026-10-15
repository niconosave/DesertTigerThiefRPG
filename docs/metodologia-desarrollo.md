# Metodología de diseño técnico, desarrollo y calidad

| Campo | Valor |
|---|---|
| Estado | Vigente |
| Versión | 0.4 |
| Fecha | 2026-10-15 |

## 1. Propósito

Este documento define cómo se transforma una idea confirmada del juego en diseño
técnico, planes, código y verificaciones. No diseña mecánicas ni elige todavía la
arquitectura, las bibliotecas o los formatos de datos.

La metodología persigue tres objetivos:

- mantener una versión jugable al terminar cada incremento;
- evitar que las decisiones queden escondidas en planes o código;
- comprobar mediante pruebas automáticas y juego manual que lo implementado tiene
  la calidad esperada.

Una necesidad no aclarada se registra como pregunta. No se convierte en requisito
por inferencia.

## 2. Fuentes de verdad

| Nivel | Documento o artefacto | Responsabilidad |
|---|---|---|
| 1 | [`diseno-general.md`](./diseno-general.md) | Define qué juego se quiere construir y sus elementos confirmados. |
| 2 | `contenido/*.md` | Define la intención creativa de conjuntos concretos sin duplicar sus datos serializados. |
| 3 | `tecnica/diseno-tecnico-general.md` | Define la organización global, los límites y las reglas compartidas del desarrollo. |
| 4 | `tecnica/componentes/*.md` y `tecnica/acciones/**/*.md` | Diseñan en detalle un componente o las reglas de una acción. |
| 5 | `planes/pendientes/*.md` | Ordena pasos de implementación y verificación basados en diseños aprobados. |
| 6 | Código, datos y pruebas | Implementan el diseño aprobado. |
| Historial | `planes/completados/*.md` | Conserva qué se hizo, cómo se verificó y qué se desvió. |

Una capa inferior puede detallar a una superior, pero no contradecirla. Cada
decisión debe tener una sola fuente de verdad; los demás documentos la enlazan en
vez de copiarla.

- Un cambio conceptual del juego comienza en el diseño general.
- Una decisión narrativa o visual limitada a un conjunto de contenido comienza
  en su brief creativo.
- Una decisión que afecta a todo el proyecto comienza en el diseño técnico
  general.
- Una decisión limitada a una parte comienza en su documento técnico específico.
- Un cambio en el orden o ejecución del trabajo se registra en el plan.
- Los briefs expresan propósito, atmósfera y función, pero no copian matrices,
  coordenadas, claves, retratos ni scripts completos.
- Los archivos de contenido serán la fuente de sus datos concretos; la
  documentación técnica definirá su estructura y significado sin duplicarlos.

El código existente no convierte por sí solo una desviación en una decisión
válida. Si implementación y documentación difieren, se resuelve la diferencia y
se actualiza la fuente correspondiente.

## 3. Documentos técnicos

El diseño técnico general contendrá solamente decisiones transversales. Los
detalles de cada sistema vivirán en documentos separados para poder diseñarlos e
implementarlos de forma incremental.

Estados posibles de un documento técnico:

- **Borrador:** contiene preguntas o decisiones todavía no resueltas que afectan
  al alcance declarado de su versión actual.
- **Aprobado para implementar:** no contiene decisiones abiertas dentro de su
  alcance y tiene información suficiente para crear un plan sin completar el
  diseño durante la implementación.
- **Implementado:** describe fielmente código y datos ya verificados.
- **Obsoleto:** fue reemplazado y enlaza su sustituto.

Una capacidad futura que esté expresamente fuera del alcance no mantiene el
documento en `Borrador`. Se registra en una sección de decisiones aplazadas o
puntos de extensión, sin diseñarla prematuramente. La sección de preguntas
pendientes se reserva para decisiones que sí impiden aprobar el alcance actual.

Todo documento técnico debe indicar estado, versión y fecha. Su contenido mínimo
es:

1. propósito, alcance de la versión y aspectos excluidos o aplazados;
2. elementos del diseño general que desarrolla;
3. responsabilidades y límites;
4. datos, entradas, salidas y comportamiento observable;
5. contratos e invariantes que no pueden romperse;
6. errores y validaciones esperadas;
7. estrategia de pruebas y criterios de aceptación;
8. preguntas bloqueantes, si todavía existen.

Cuando el documento gobierne código, antes de aprobarse deberá fijar además, en
la medida aplicable:

- el crate y los módulos responsables;
- los tipos propios y la API pública entre capas;
- los estados y transiciones observables;
- las categorías de error y su tratamiento;
- la relación entre cada contrato importante y su verificación.

Cuando gobierne archivos de contenido, deberá incluir un esquema normativo,
ejemplos válidos e inválidos y las reglas de compatibilidad necesarias para su
alcance.

Cuando sea útil, puede incluir tipos, firmas, pseudocódigo, flujos o ejemplos de
datos. Debe ser suficientemente preciso para que el plan no tenga que decidir qué
construir, pero no debe contener una lista de pasos de implementación.

Alternativas descartadas, compatibilidad y migraciones se documentan solo cuando
realmente sean relevantes.

## 4. Ciclo incremental

Cada incremento implementa la capacidad observable más pequeña que aporte valor
y pueda comprobarse jugando.

1. Elegir una capacidad dentro del diseño ya confirmado.
2. Definir el resultado observable y lo que queda fuera.
3. Resolver con el usuario cualquier decisión de juego faltante.
4. Actualizar el diseño técnico general solo si aparece una decisión transversal.
5. Crear o actualizar los documentos técnicos específicos necesarios.
6. Aprobar esos documentos y crear un plan que enlace sus secciones.
7. Implementar el incremento y sus pruebas.
8. Ejecutar controles automáticos y la prueba manual pertinente.
9. Jugar el recorrido afectado y registrar los problemas encontrados.
10. Actualizar la documentación para que describa el resultado real.
11. Completar y mover el plan cuando el proyecto vuelva a un estado jugable.

“Siempre jugable” se aplica a cada incremento terminado, no a cada modificación
intermedia. Un incremento es jugable cuando el programa puede iniciarse, permite
alcanzar y usar la capacidad disponible y continúa en un estado válido sin fallos
críticos.

Si la implementación descubre un problema de diseño, se corrige primero o junto
con el documento correspondiente y después se ajusta el plan. No se oculta una
decisión nueva dentro de una desviación del plan.

## 5. Estándares de implementación

- Se implementa la solución más sencilla que satisfaga el diseño vigente. No se
  añaden extensiones futuras sin documentarlas.
- La presentación visual inicial prioriza funcionamiento, legibilidad y
  coherencia. Su acabado artístico se mejora en incrementos posteriores a partir
  de pruebas manuales sobre una versión jugable; esos refinamientos no bloquean
  una capacidad funcional ni relajan sus criterios de accesibilidad.
- La lógica del juego permanece independiente de terminal, teclas, color y
  composición visual.
- El frontend presenta información y recoge decisiones; no duplica ni evita las
  reglas del backend.
- Jugador y NPC utilizan el mismo sistema de ejecución de acciones.
- Lua consulta el contexto permitido y solicita acciones; no evita su validación
  ni modifica directamente el mundo.
- La posición visual sobre el mapa nunca se usa como estado de gameplay.
- El código Rust debe estar formateado con `rustfmt` y libre de advertencias de
  `clippy` bajo la configuración acordada por el proyecto.
- Los errores esperables de entrada, archivos, contenido o scripts se representan
  explícitamente y proporcionan contexto. No provocan un `panic` deliberado.
- `unwrap` y `expect` no se usan en caminos normales de producción, salvo para una
  invariante justificada por escrito. En pruebas pueden usarse cuando aclaren el
  caso.
- Las interfaces públicas se mantienen pequeñas y expresan conceptos del dominio.
- Toda dependencia nueva debe tener una necesidad concreta y quedar justificada
  en el documento técnico que la introduce.
- Los comentarios explican motivos, restricciones o decisiones no evidentes; no
  repiten el código.
- El azar, la fecha externa y otras entradas variables deben poder controlarse en
  pruebas para reproducir un fallo. Esto no obliga a que el juego completo sea
  determinista.
- Los datos y scripts inválidos deben producir diagnósticos claros, no fallos
  silenciosos ni partidas parcialmente creadas.
- Cuando se corrige un defecto, se añade una prueba de regresión siempre que sea
  razonable.

La organización de crates y módulos, las bibliotecas, el formato de datos, la API
de Lua y el protocolo entre backend y frontend se decidirán en los documentos
técnicos, no en estas normas.

## 6. Estrategia de pruebas

| Tipo | Qué comprueba |
|---|---|
| **Unitaria** | Reglas aisladas: características, acciones particulares, tiempo, fecha y estados. |
| **Integración** | Colaboración entre sistemas: carga de contenido, acciones generales, grupo, avance temporal y fase de NPC. |
| **Contenido y Lua** | Sintaxis, campos, referencias, coordenadas, mapa, retratos de Actor y scripts incluidos con el mundo. |
| **Frontend** | Traducción de entrada a intenciones, modos, foco, resolución de arte propio y representación de resultados sin ejecutar reglas. |
| **Humo de extremo a extremo** | Inicio desde archivos, una acción completa, avance del mundo, fase de NPC y regreso al jugador. |
| **Prueba manual** | Estética ASCII, animación, legibilidad, controles, ritmo y sensación de juego. |

Cada documento técnico específico determina los casos que su sistema necesita.
Las reglas repetibles se verifican automáticamente siempre que sea viable. La
experiencia visual y la jugabilidad requieren además revisión manual.

Las pruebas con azar deben ser reproducibles. Las pruebas no deben depender del
reloj real, del orden accidental de archivos o de la velocidad de la terminal.
No se fija un porcentaje de cobertura: se exige cubrir los comportamientos
importantes, sus límites y los defectos ya encontrados.

## 7. Controles automáticos

Cuando exista el proyecto Cargo, todo plan que modifique código debe ejecutar:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
```

Cuando exista un validador de contenido, también deberá ejecutarse sobre todos los
datos y scripts oficiales. Antes de una entrega se comprobará además una
compilación de producción. La automatización futura deberá ejecutar estas mismas
reglas, no crear una definición de calidad diferente.

Una comprobación solo puede marcarse como no aplicable si el plan explica el
motivo.

## 8. Definición de terminado

Un plan está terminado únicamente cuando:

- el comportamiento acordado está implementado y puede observarse;
- código, contenido y frontend respetan la documentación aplicable;
- existen y pasan las pruebas requeridas;
- pasan los controles generales de calidad;
- el contenido oficial afectado valida correctamente;
- se realizó una prueba manual si cambió la experiencia visible;
- el estado estable del juego continúa siendo jugable;
- no queda un defecto conocido que impida usar el incremento;
- los documentos técnicos describen el resultado real;
- el plan registra verificaciones, desviaciones y trabajo posterior.

Solo entonces el archivo del plan se mueve de `planes/pendientes/` a
`planes/completados/`. Los planes completados conservan el historial; una
corrección posterior se realiza mediante un nuevo plan.
