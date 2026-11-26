# Revisión de la implementación de la base de encuentros

Fecha: 2026-11-24. Resultado: implementación parcial, no apta para cerrar la etapa.
Revisión por lectura de código y contraste con el contrato 12; no aceptación jugable.

## Hallazgos prioritarios

1. **Los planes no gobiernan las fichas.** En core/encuentros.rs, elegir() sigue
   decidiendo por retirando, rescate, botin y objetivo. No consulta planes, sus
   participantes, prioridad ni estado. PlanLocal es metadato: no existe la
   coordinación por roles/comportamientos exigida por el bloque 053. Cambiar un
   plan no cambia efectivamente la dirección. reconsiderar(false) tampoco restaura
   los estados del plan suspendido; la retirada funciona por el booleano antiguo.

2. **Los efectos locales de eventos siguen pudiendo confirmarse parcialmente.**
   app/eventos/coordinador.rs aplica local.aplicar_comandos antes de preparar_vista,
   que puede fallar. En ese fallo ya se cambió el encuentro mientras Mundo no se
   confirmó. Falta la preparación coordinada de local, Mundo y presentación del
   contrato 12, sección 11.

3. **La conclusión sigue siendo la antigua.** core/encuentros.rs:1320-1349 calcula
   Desenlace directamente y elige el primer enemigo capaz como captor. No comprueba
   acceso por cautivo, no representa resultados individuales ni implementa la
   decisión de abandono. Un enemigo aislado en otro piso puede provocar captura
   sin una ruta practicable. Faltan las conclusiones causales del bloque 056.

4. **Conocimiento y contexto todavía no controlan la revelación.** El resolver
   activa descubierta mediante x>15 y app/sesion/ataque.rs:270 filtra por esa misma
   frontera. No se usa descubrimiento por espacios ni observación por conexiones
   abiertas. Al cruzarla se publica terreno/entidades que deberían seguir ocultos
   detrás de puertas o en otros espacios. Faltan memoria y filtrado del contrato.

5. **Las identidades estables están declaradas, pero no integradas.** Ficha no tiene
   IdFicha; objetivo, transportistas y ejecutores siguen siendo índices usize.
   Declarar wrappers no satisface el contrato de identidad y referencias estables.

## Verificación realizada

Se ejecutó únicamente cargo test -q -p desert-tiger-core planes_locales: una prueba
pasó. Esa prueba comprueba prioridad almacenada y estados tras reconsiderar(true),
pero no demuestra que la prioridad dirija acciones ni que funcionen roles,
sustitución y reanudación. No se ejecutó una batería global.

## Continuación necesaria

Completar los contratos pendientes de 052 a 056 y agregar comprobaciones de
comportamiento real para estos hallazgos. No sustituir esos contratos por notas de
limitación ni marcar el bloque 057 como satisfecho porque pasen los filtros actuales.
Conservar las partes 3D ya implementadas y las modificaciones del usuario.

## Avance después de completar los bloques 052 y 053

La geometría está separada y dispone de contexto filtrado. Los planes ahora
coordinan roles y comportamientos efectivos, incluidos auxiliadores exclusivos y
sustitución por incapacidad. Se comprobaron prioridad, cancelación, reanudación,
identidad de comportamiento y navegación sin revelar el objetivo oculto.
El hallazgo original de planes meramente descriptivos queda superado por ese
trabajo; la revisión integrada de cierres y frontend corresponde a los bloques
posteriores y no se declara finalizada la etapa.

## Cierre técnico de los bloques 054 y 055

Las recetas verifican recorridos y puertas simultáneas. La representación usa
memoria de elementos, profundidad de entidades, controles durante eventos y
seguimiento por encuentro. Los impactos respetan origen XYZ y ocultación.
Se verificaron las regresiones seleccionadas sin declarar terminados los cierres
causales del 056 ni la aceptación integrada del 057.

## Cierre técnico del bloque 056 (2026-11-25)

El cierre ya no depende exclusivamente de una etiqueta global. El director
construye y valida un resultado causal con contabilidad de todas las fichas y el
botín. La etiqueta `Desenlace` es un resumen de presentación de ese resultado.
Los captores se comprueban por persona y por recorrido practicable; se separan
los compañeros vivos que permanecen o quedan capturados. La evacuación incompleta
ofrece continuar el auxilio o confirmar quién queda atrás. Las autorizaciones
son por identidad y no alcanzan a heridos posteriores.

La sesión confirma extracción, rescate y consecuencias de conclusión una sola
vez. El cierre reconoce el resultado y aplica el tiempo único. Las respuestas y
frames mantienen tokens propios y no ejecutan pulsos. El coordinador de eventos
trabaja sobre copias de Mundo y encuentro hasta preparar la presentación; un
fallo no confirma sus efectos parciales. Las pruebas acotadas cubrieron abandono,
captura parcial, posesión al líder, rescate, eventos mixtos y diálogo de decisión.
El hallazgo original de conclusiones y el de confirmación parcial quedan
atendidos. La revisión integrada y la aceptación manual siguen en el 057.

## Verificación integrada 057 (2026-11-25)

Se completaron los recorridos y controles de terminal que faltaban: puerta,
pausa, omisión, tamaño mínimo, rescate con escalera real y robo con eliminación
de la oportunidad ya resuelta. Se conservó la comprobación de eventos de Gorun
y se completaron los errores tipados locales. El binario con funciones
predeterminadas compiló y arrancó/cerró correctamente en una PTY; esa prueba
se hizo con audio desactivado y no sustituye escuchar los ambientes.

Pasaron diez pruebas seleccionadas, formato y Clippy (un aviso preexistente en
la firma de dibujar_retratos_dialogo). No se ejecutó la batería global.
La [guía de revisión del usuario](../../planes/revision-jugable-base-encuentros.md)
registra el único paso de aceptación pendiente. No se presenta la automatización
como una revisión humana ni se archivan los planes antes de esa respuesta.

## Pendientes confirmados al contrastar el contrato después del complemento 053

La implementación de memoria y sucesos del 053 no cierra por sí sola todo el
contrato 12. En la revisión posterior, sin ejecutar una batería nueva, se
identifican estos puntos concretos:

- **Bloqueos con causa (053 / contrato 14):** `SinRuta` y
  `ParticipanteNoDisponible` están declarados, pero la ejecución no los emplea
  para distinguir esos casos. Predominan ausencia de participantes y ocho pulsos
  sin progreso. Falta completar la clasificación y su notificación.
- **Resultados de interacción con puerta (054 / contrato 8):** se conserva el
  único cambio por pulso y el ejecutor real, pero no se registran como resultados
  explícitos todos los concurrentes, rechazos y casos SinCambio que pide el
  contrato. La memoria de comportamiento no sustituye esa contabilidad.
- **Presupuesto de dirección (053 / contrato 14):** no hay un contador explícito
  que aplique las 128 transiciones internas y devuelva un bloqueo reintentable
  por agotamiento. El agotamiento de IDs actualmente contemplado es otra cosa.
- **Reutilización en conclusiones (056 / contrato 14):** evaluar y validar el
  cierre vuelven a ejecutar `capturas_posibles`; la validación de sesión/cierre
  puede repetir esas búsquedas. Falta respetar la reutilización por ficha/revisión
  sin eliminar la revalidación de acceso de cada cautivo.

Son requisitos de la base ya documentada, no nuevas maniobras, planes Lua ni
inteligencia artificial universal. Deben tener pruebas acotadas propias antes de
marcar cerrado el conjunto. Esta lista registra hallazgos comprobados; no afirma
haber agotado una revisión línea por línea de todos los contratos.


## Resolución de los cuatro pendientes (2026-11-26)

Quedan implementados los cuatro hallazgos registrados en la revisión del contrato:

- Bloqueos con causa específica, visibles en dirección, memoria y presentación.
- Resultados de puerta por participante, incluidos concurrentes, rechazos y
  `SinCambio`; los intentos sin efecto no simulan progreso.
- Presupuesto explícito de 128 transiciones con descarte del candidato y pausa
  reintentable antes de consumir azar o confirmar acciones.
- Conclusiones con rutas reutilizadas y certificado físico: validar repetidamente
  no ejecuta nuevas búsquedas; modificar ese estado invalida el resultado.

Las cuatro regresiones `base_pendiente` cubren estos contratos. También pasaron
las regresiones seleccionadas de puertas, planes, consecuencias, robo, rescate
y eventos locales. La corrección conserva el avance durante congestión temporal
de una salida: que una ficha espere no bloquea al grupo si otras progresan.
No se ejecutó la batería global. Esta resolución se limita a los cuatro hallazgos;
no afirma una auditoría exhaustiva de cada cláusula ni incluye preparación de
equipo, sigilo, nuevas maniobras o autoría de planes desde Lua.
