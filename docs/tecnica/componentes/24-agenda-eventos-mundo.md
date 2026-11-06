# Modelo común de eventos: autoría, actividad NPC y agenda

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar; código pendiente |
| Versión | 2.0 |
| Fecha | 2026-11-06 |
| Alcance | Integrar los mecanismos existentes bajo un único EventoNarrativo |

## 1. Propósito y autoridad

Existe un concepto de contenido: **EventoNarrativo**. Puede definir un episodio
único, un quehacer habitual de un actor o un acontecimiento fechado del mundo.
La ubicación del archivo organiza su autoría; no determina quién participa,
cuántas veces ocurre, cuándo se activa o si se muestra al jugador.

El [contrato 23](./23-eventos-narrativos.md) describe la versión implementada y
las bases que se reutilizan. Este documento gobierna su evolución y sustituye
la propuesta 1.0 de una agenda con coordinador y estados separados. No modifica
por sí mismo el código ni aprueba contenido del [borrador de historia](../../contenido/borradores/drama-y-mitos.md).

Se conserva la decisión del usuario: los sucesos fechados se resuelven al cerrar
la acción, usando el lugar donde termina el jugador. No se reconstruye la
posición ni el estado histórico del mundo durante fechas intermedias.

## 2. Qué se reutiliza y qué se integra

| Base actual | Destino en este diseño |
|---|---|
| Eventos externos y grafos finitos del contrato 23 | Se conservan y se normalizan al catálogo común. |
| Condiciones, Lua puro, efectos, vistas y respuestas identificadas | Se reutilizan, sin un motor narrativo alternativo. |
| `politica` del RON del actor | Se adapta a un evento local interno; conserva el comportamiento legado. |
| Ejecución de Viajar para NPC | Sigue usando el motor de acciones existente, solicitada por un paso del evento. |
| Fechas y observación remota de la propuesta 24 v1.0 | Se conservan como activación y presentación del mismo modelo. |
| Catálogos, marcadores y coordinadores separados para agenda/políticas | Se reemplazan por catálogo, registro de ocurrencias y coordinador únicos. |
| Formato independiente `fecha/resolver/relato` de la propuesta 24 v1.0 | Se sustituye por el esquema común de activación y pasos. Nunca estuvo implementado. |

El incremento incluye autoría incrustada, `this`, adaptación de políticas,
activaciones por puntos/fecha/fase NPC, repetición controlada por cierre,
solicitud de Viajar para NPC y operaciones directas acotadas.

Quedan fuera nuevas acciones generales, calendario intercalado dentro de Viajar,
recurrencias de calendario, decisiones del jugador a distancia, rumores,
inventario, reclutamiento, cambios de conectividad/filiación, guardado, sucesión
del jugador y finales de época. Tampoco se crean nuevos tipos de relaciones
narrativas salvo el registro de rescate de la sección 11.

## 3. Un catálogo y un ejecutor

El contenido se convierte al cargar en definiciones normalizadas. Cada partida
posee un coordinador narrativo y un registro común de ocurrencias. Los índices
por punto, fecha y fase NPC seleccionan candidatos del mismo catálogo: no
interpretan Lua, no ejecutan pasos ni guardan otro marcador de consumo.

| Módulo | Responsabilidad |
|---|---|
| `app/eventos/contenido.rs` | DTO, normalización, identidades, referencias e índices. |
| `app/eventos/coordinador.rs` | Selección, ocurrencia, grafo, efectos, vistas y continuación. |
| `app/eventos/agenda.rs` | Índice temporal y cursor del intervalo; entrega IDs al coordinador. |
| `app/eventos/lua.rs` | API común, propietario y contexto, constructores autorizados. |
| `app/fase_npc.rs` | Adaptador para ejecutar solicitudes NPC mediante el motor común y producir sus resultados. |
| `app/lua.rs` | Adaptador de los bloques `politica` heredados y sus diagnósticos. |
| `app/sesion/eventos_partida.rs` | Cierre por fases, activaciones y fachada pública. |
| `core/modelo.rs` | Operaciones e índices atómicos; desconoce autoría y Lua. |
| `terminal` | Presentación y entradas existentes, sin evaluar sucesos. |

La agenda no conserva un `CoordinadorAgenda` ni un estado `Ocurrido` paralelo.
La fase NPC deja de recorrer políticas por fuera del catálogo. El adaptador de
compatibilidad conserva el evaluador antiguo, no un segundo ciclo de ejecución.

## 4. Propietario, `this` e identidad

### 4.1. Referencia contextual al actor

En todo bloque Lua de un evento incrustado, `this` es la **clave externa del
actor cuyo RON contiene la definición**. Es una cadena, compatible con las
consultas y operaciones que reciben claves:

```lua
return mundo:actor_tiene(this, "musico")
```

Está disponible en condición, presentación, efectos y solicitud de acción. En
un evento externo `this == nil`: sus referencias a actores usan claves explícitas.
No se proporciona un objeto mutable, una clase ni un puntero Rust.

| Referencia | Significado |
|---|---|
| `this` | Propietario de la definición, o nil si es externa. |
| `ctx:actor()` | Dirigente del punto que originó la activación, cuando existe. |
| `mundo:actor_jugador()` | Clave del personaje controlado por el jugador. |
| Actor de una solicitud | Persona que el guion elige para ejecutar esa acción. |

Las cuatro identidades pueden diferir. `this` no cambia por traslado, muerte,
pertenencia al grupo o cambio futuro de protagonista. Un evento encadenado
conserva el contexto de activación, pero resuelve **su propio propietario**;
no hereda el `this` del solicitante.

Rust conserva el propietario en la definición. El entorno Lua protege también
la vinculación global `this`: asignarle otro valor produce error. Inyectarla
como un campo normal de una tabla y añadir `__newindex` no basta, porque una
escritura sobre un campo existente eludiría esa protección. Se debe usar un
entorno proxy/backing protegido o una garantía equivalente, sin exponer rawset
ni el backing. `local this = ...` puede sombrear el nombre dentro del script,
pero nunca cambia propietario, ocurrencias ni permisos en Rust.

### 4.2. Ámbitos de claves

La identidad interna distingue `Global(clave)` y
`DelActor { actor, clave_local }`. Dos actores pueden declarar `viaje_habitual`
sin compartir estado. Un evento externo puede usar esa misma clave sin colisionar.
No se identifica una ocurrencia únicamente por el nombre local del paso o evento.

Las claves de archivos, actores y eventos siguen la validación existente.
El orden total usado en puntos y fechas coloca Global antes de DelActor;
ordena los primeros por clave global y los segundos por IdActor y clave local.
La fase NPC usa el orden particular de la sección 6.3 para conservar el legado.
Para referencias `encadena`, una cadena sin prefijo sigue identificando un
evento global, por compatibilidad. También se admiten `global:clave` y
`actor:clave_actor:clave_local`. Los componentes se validan por separado; no
se busca primero en un ámbito y luego en otro. Esos prefijos son sintaxis de
referencia, no parte de las claves de autoría.

## 5. Autoría y normalización de formatos

Los archivos externos siguen en `events/**/*.ron`, un evento por archivo con
clave igual al nombre base. **No se reserva `events/calendario/`** ni se cambia
el esquema según la subcarpeta. Su nombre solo sirve para organizar contenido.
La carpeta puede faltar o estar vacía. No se agrega una lista a `world.ron`.

El RON del actor admite `eventos: [...]`, vacío al omitirse. Cada entrada usa
el mismo esquema que un archivo externo y una clave local única en ese actor.
El propietario se deriva del contenedor; no puede declararse o falsificarse
mediante un campo `propietario`. Los eventos locales pueden referenciar otros
actores y no se limitan a acciones de su dueño.

Se reutilizan `clave`, `modo`, `prioridad`, `filtros`, `condicion`, `inicio`,
`encadena` y `pasos` del contrato 23. `condicion` sigue siendo Lua obligatorio;
para una cita incondicional se escribe `return true`.

| Campo ampliado | Contrato |
|---|---|
| `activacion` | `PorPuntos(puntos: [...])`, `EnFecha(fecha: (anio, mes, dia))` o `EnFaseNpc`. |
| `puntos` legado | Se normaliza a `PorPuntos`; no se admite junto con `activacion`. |
| `repeticion` | `UnaVezPorPartida` por defecto; se añade `UnaVezPorCierre`. |
| `observacion` | `DelPunto`, `Local(lugar: "clave")` o `Silenciosa`. |

Debe existir exactamente una activación, nueva o legada. `PorPuntos` exige una
lista no vacía sin repetidos. `EnFecha` conserva el calendario existente de doce
meses. `UnaVezPorCierre` se habilita inicialmente solo con `EnFaseNpc`; no genera
recurrencias para una cita fija ni reabre eventos durante un mismo viaje.

`observacion` omitida equivale a `DelPunto` para PorPuntos y a `Silenciosa` para
las otras activaciones. Estos valores dependen de la activación, nunca de que
el archivo pertenezca a un actor. Los campos desconocidos se rechazan.

Los eventos RON v1 siguen funcionando sin edición: activación PorPuntos,
UnaVezPorPartida, observación DelPunto y propietario ausente. Su prioridad,
condiciones, grafo, respuestas y consumo de azar conservan el contrato vigente.

## 6. Activaciones y repetición

### 6.1. Puntos de acciones y sesión

Se conservan puntos, límites seguros, tokens, prioridades y continuaciones del
contrato 23. Un evento local sobre un compañero puede activarse en Viajar; su
propietario no se confunde con el dirigente que ofreció el punto.

La ubicación del evento dentro de un actor no implica omitirlo si ese actor
está en el grupo, muerto o fuera del escenario. Un episodio puede tratar de
esas circunstancias. Sus condiciones y las operaciones solicitadas determinan
su validez. La exclusión automática del comportamiento NPC antiguo solo se
conserva en su adaptador de compatibilidad, sección 9.

### 6.2. Fechas atravesadas

La cita sucede al comienzo del día declarado, con fracción cero. Al cerrar una
acción se consideran las fechas dentro de `(inicio, final]`, comparando el
instante completo. Orden: fecha ascendente, prioridad descendente e identidad
normalizada ascendente. No se recorren los días uno a uno.

Se resuelve contra el mundo resultante de la acción y los eventos anteriores
del cierre. `Mundo.fecha` permanece en el final real; `fecha_programada` conserva
la fecha nominal. No se reconstruyen estado, posición o tiradas intermedias.
La presencia usa el lugar final del jugador, conforme a la elección del usuario.

Al iniciar la partida, una entrada inclusiva considera las citas de la fecha
inicial una sola vez, antes de partida.disponible y sin fase NPC. Se implementa
como `iniciar_partida(instante, lugar)`, sin restar un día al calendario mínimo.
Una fecha anterior a la inicial se rechaza; sus hechos pertenecen al mundo inicial.

Se evalúa la condición justo antes de la cita. Si es falsa, se marca
`OmitidoPorCondicion` definitivamente. Si falla técnicamente, no se consume.
Una cita omitida no se reconsidera aunque otra posterior la hubiera habilitado.
La ausencia del jugador no convierte la condición en falsa.

El cursor temporal solo recorre IDs candidatos. Un evento ya atendido mediante
el registro común no vuelve a ejecutarse al reencontrarlo. La búsqueda cuesta
`O(log N + K)`, más el trabajo de las K citas del intervalo.

### 6.3. Fase NPC y quehacer habitual

`EnFaseNpc` se considera una vez por cierre de una acción del jugador, después
de las citas de fecha. Puede pertenecer a un actor o ser externo. Cada candidato
se revalida después de los efectos de los anteriores. Orden: prioridad
descendente, propietario por IdActor (externos después de actores) y clave local;
entre externos se ordena por clave global. Las políticas adaptadas usan prioridad
cero y así conservan el orden relativo actual de sus actores.

Un evento UnaVezPorPartida con condición falsa sigue disponible para otra fase;
uno que llega a Fin queda consumido. Uno UnaVezPorCierre puede participar en
el cierre siguiente. La misma activación no vuelve a considerarlo aunque su
condición siga siendo verdadera o se vuelva verdadera durante esa fase.

Para conservar el comportamiento legado se ofrece la fase incluso cuando la
acción se cancela con duración cero. No se abre durante la entrada inicial ni
por leer, dibujar, cambiar un foco o ejecutar la acción de un NPC.

### 6.4. Registro de ocurrencias y cadenas

Cada resultado raíz recibe un `IdCierre` una sola vez. Leer, reanudar o recibir
una finalización duplicada no asigna otro ID ni reinicia fecha/fase NPC. Una visita
a punto conserva su token propio. La ocurrencia une IdEvento e identidad de
activación. El registro único conserva el consumo de los eventos únicos y la
última fase considerada de **todos** los EnFaseNpc, únicos o repetibles.

Una condición falsa de EnFaseNpc o la exclusión del legado marca considerado
en ese IdCierre, sin consumir la unicidad global. Una confirmación válida retiene
además el paso y su ocurrencia. Un error técnico ordinario no confirma esa marca;
el adaptador legado trata sus fallos recuperables según la sección 9.
No basta devolver un repetible a NoOcurrido al terminarlo. Marcas, selección y
pasos se coordinan atómicamente, evitando ciclos de reconsideración en la fase.
Una vez por cierre significa una evaluación/ocurrencia por ese cierre, no una
acción extra por cada día atravesado.

Las cadenas v1 se conservan para eventos únicos PorPuntos, incluidos eventos
locales. Sus referencias usan los ámbitos de la sección 4.2. Esta etapa rechaza
encadenar explícitamente citas, repetibles o el adaptador de politica, y exige
`encadena: []` en EnFecha/EnFaseNpc. Los hechos y globales pueden habilitar los
candidatos posteriores de una fase sin invocación recursiva. Las extensiones de
encadenamiento entre otras activaciones quedan aplazadas.

## 7. Grafo común y presentación

Se reutilizan Vigneta, Efectos y Fin y sus validaciones del contrato 23: claves,
salidas declaradas, alcance completo, ausencia de ciclos, fin de todos los caminos
y atomicidad por paso. No existe un segundo formato resolver/relato para agenda.

`DelPunto` conserva la interacción de los puntos del jugador. Solo es válido
para PorPuntos y bajo las restricciones de sustitución/interrupción actuales.

`Local(lugar)` y `Silenciosa` permiten resolución sin respuesta humana. Sus
grafos no admiten opciones en Vigneta, SustituirRelato ni InterrumpirAccion;
todos sus Fin son Continuar. Local muestra viñetas informativas si coincide el
Lugar declarado con el del jugador al activar; a distancia las salta siguiendo
su `siguiente`. Silenciosa las salta siempre. Saltar presentación no salta los
pasos de efectos ni consume RNG.

La presencia no se infiere de cámara, tiles, conocimiento o cercanía visual.
Fuera de escena no se ejecuta presentacion Lua ni se genera aviso, expresión,
resumen, exploración o conocimiento automático. El autor puede solicitar un
cambio de conocimiento expresamente; los efectos no se omiten por no observarse.
Un guion que consulta presencia puede escribir diferencias particulares.

Las vistas reutilizan PresentandoEvento y los identificadores compartidos.
`VistaEvento.fecha` sigue mostrando la fecha efectiva; se añade
`fecha_suceso: Option<Fecha>`, Some en EnFecha y None en las otras activaciones.
Cuando difieren, la observación identifica «Suceso del …» sin retroceder la barra,
el mapa o el reloj. El texto del autor debe admitir esa observación al cierre.

Ante Efectos se prepara el Mundo provisional y la siguiente vista que realmente
se mostrará antes de confirmar; se conserva la garantía por paso del contrato 23.
Si falla una presentación posterior, permanecen los pasos anteriores, pero no
se instala medio paso actual. Páginas, detalle, resize y entradas vencidas no
repiten condiciones, efectos ni acciones.

## 8. Solicitar una acción general desde un evento

Se añade el paso `AccionNpc(clave, solicitar, siguiente)`, inicialmente solo en
grafos EnFaseNpc automáticos. `solicitar` es Lua puro: devuelve exactamente nil
o una solicitud opaca de `acciones.viajar(actor, destino)`. Ambos argumentos son
claves, por lo que un evento local puede escribir `acciones.viajar(this, destino)`.
Lua no ejecuta la acción al construir la solicitud.

El coordinador entrega la solicitud al motor actual. Exige un NPC independiente;
no actúa por el jugador o un compañero. Un rechazo reglado produce resultado
Rechazada sin cambios; una ejecución válida usa las reglas, RNG y decisiones
automáticas actuales. No hay implementaciones alternativas de Viajar. Sus
viñetas internas no abren otros eventos ni se muestran una a una en esta etapa.
La duración pertenece al intervalo ya consumido y no adelanta el calendario.

Al terminar se conserva un resultado y se avanza a siguiente una sola vez.
`ctx:resultado_accion()` consulta el último resultado de esa ocurrencia, o nil
si aún no existe. Está disponible en todos los bloques, comienza en nil en cada
ocurrencia y no se hereda de una cadena. Devuelve una tabla inmutable:

| `estado` (cadena Lua) | Campos |
|---|---|
| `"SinSolicitud"` | actor, destino, lugar_final y cierre son nil. |
| `"Rechazada"` | actor/destino solicitados; lugar_final actual si existe; cierre nil. |
| `"Completada"` | actor/destino solicitados, lugar_final confirmado y cierre `"Llegada"`. |
| `"Interrumpida"` | actor/destino solicitados, lugar_final confirmado y otro cierre válido del motor. |

Los cierres se expresan con las cadenas estables Llegada, Cancelado, SinRecursos,
InterrumpidoPorSeparacion, Naufragio o InterrumpidoPorEvento. Este último queda
reservado si el adaptador llegara a recibirlo; no se provocan eventos internos
en la acción NPC en esta etapa. No se usa la prosa para resolver consecuencias.
El contexto de origen y `this` permanecen estables.

Con nil se registra SinSolicitud y se continúa sin invocar al motor. Las salidas
regladas no se convierten en fallos de Lua. Un fallo fatal del motor detiene la
sesión; no se revierten efectos o RNG ya confirmados de la acción. Antes de
empezar se valida la solicitud; después de realizarla se retiene su resultado,
sin volver a tirarla al leer el relato o reanudar el evento.

Un paso Efectos sigue solicitando operaciones directas sin chequeos. AccionNpc
solicita una acción que puede fallar. Son dos capacidades del mismo evento,
expresadas por pasos distintos; ninguna concede a Lua acceso mutable al Mundo.

## 9. Adaptación del campo `politica` existente

`politica: Ninguna`, o el campo omitido en el formato ampliado del actor, no
produce una definición. `politica: Lua(codigo)` se normaliza a un evento interno
DelActor con clave reservada `__politica_legacy`, EnFaseNpc y UnaVezPorCierre.
El nombre reservado no se acepta en eventos escritos por el diseñador ni como
destino de encadena. Puede coexistir con otras claves explícitas del actor.

El adaptador conserva las consultas y sintaxis antiguas, incluido
`acciones.viajar(destino)` con propietario implícito, y su retorno nil/solicitud.
No obliga a reescribir los RON existentes. También vincula this al propietario,
sin sustituir el antiguo acceso `actor.lugar`. Cada evaluación conserva los
límites y diagnósticos del contrato 08.

Su grafo interno consiste en un AccionNpc seguido de Fin. El programa de
solicitud usa la variante interna AdaptadorPolitica, que invoca el evaluador
legado y convierte su resultado al común. Esa variante no se admite en RON;
no requiere un nuevo ejecutor de eventos ni una segunda marca de ocurrencia.

Antes de evaluarlo se mantienen las exclusiones actuales: jugador, compañeros,
muertos y actores fuera del escenario. Esta elegibilidad es propia del legado;
no se aplica a todos los eventos locales.

La definición adaptada usa el mismo registro y coordinador. Su solicitud se
entrega al adaptador de acciones de la sección 8, manteniendo RNG, validaciones
y resumen NPC actuales. Su presentación interna de compatibilidad conserva ese
resumen, incluso donde el comportamiento antiguo ya lo mostraba a distancia;
no se convierte en la observación por defecto de los eventos nuevos.
Los errores recuperables de política conservan su diagnóstico y continuación de
fase, marcando la activación como atendida para no reintentarlos en ese cierre;
un error de infraestructura o del motor sigue siendo fatal.

No se permite inscribir simultáneamente esa política en el catálogo y en el
recorrido antiguo de ejecutar_fase_npc. Al migrar un actor a eventos explícitos,
el diseñador elimina su bloque politica o lo deja en Ninguna. No se reescriben
archivos durante carga ni se copian rutinas al contenido externo automáticamente.

## 10. Contexto, permisos y cierre temporal

El contexto normalizado conserva propietario, origen de activación, identidad
de ocurrencia y continuación, separados del dirigente y participantes de un
punto. `ctx:punto()` usa la clave existente para PorPuntos,
`mundo.fecha_programada` para EnFecha y `fase_npc.evaluacion` para EnFaseNpc.
Estas dos últimas no son puntos admitidos dentro de PorPuntos.

EnFecha/EnFaseNpc usan `ctx:actor()`, `ctx:accion()` y `ctx:objetivo()` igual a nil;
participantes vacío. `ctx:ahora()` es el final efectivo. `ctx:fecha_programada()`
es la fecha de cita o nil; `ctx:lugar()` es la sede Local, o nil. Se añade
`ctx:jugador_presente()` y `mundo:actor_jugador()` como consultas puras.
Las consultas y constructores v1 se conservan; op solo existe dentro de Efectos,
acciones solo dentro de AccionNpc. No hay IO ni RNG disponible desde Lua.

En PorPuntos, `ctx:lugar()` conserva el lugar del punto, incluso cuando se use
observación Local con otra sede. Nunca se reemplaza por la ubicación del dueño.
`ctx:jugador_presente()` vale true en DelPunto, la igualdad de ubicación fijada
al activar en Local y false en Silenciosa, donde no hay sede que observar.

Los permisos dependen del punto y la continuación segura, no del archivo ni de
this. Las operaciones nuevas de traslado NPC se habilitan inicialmente en los
cierres EnFecha/EnFaseNpc, sin raíz de jugador suspendida. AccionNpc se limita a
EnFaseNpc. Los puntos activos conservan sus validaciones de continuidad v1.

```text
Resultado raíz y eventos accion.terminada
    → ConfirmarCalendario (una vez; retirar la raíz terminada)
    → EnFecha del intervalo, mediante el coordinador común
    → EnFaseNpc, incluidas políticas normalizadas, mediante el mismo coordinador
    → Eventos fase_npc.terminada
    → Eventos partida.disponible
    → Disponible
```

Leer una viñeta retiene la continuación exacta del cierre y no abre otra acción
del jugador. El intervalo guarda inicio/final con fracciones y su IdCierre.
Una interrupción de Viajar usa el tiempo efectivo, no el plan descartado. Las
acciones NPC no abren otro cierre ni agenda. Al no existir nuevos eventos,
la adaptación conserva las secuencias y resultados del juego actual.

## 11. Operaciones directas acotadas

Se conservan globales tipadas, rasgos de actor y conocimiento del contrato 23.
No se reescriben archivos ni se omiten invariantes estructurales de Rust.

`op.trasladar_npc(actor, lugar)` solicita `TrasladarNpcNarrativo`. Exige NPC
independiente con ubicación local, destino local válido distinto del origen;
rechaza jugador, compañeros y externos. No busca rutas ni añade costes o RNG.
No se expone la operación genérica que también puede mover al grupo.

`op.registrar_rescate(rescatado, rescatador)` solicita RegistrarRescate e incorpora
RescatadoPor, dirigida Actor→Actor. Extremos existentes y distintos, sin mercancía;
vínculo repetido idempotente. Desde el rescatado: «Rescatado por»; desde el otro:
«Rescató a». No añade servicio, lealtad, conocimiento, grupo, traslado ni quita
prisionero automáticamente: el autor reúne las consecuencias deseadas en un lote.

El almacén y ambos índices por actor se actualizan juntos, con IDs estables y
detección indexada de duplicados. Se actualizan carga, ficha de campamento y
`mundo:hay_relacion`; admite RescatadoPor y la consulta inversa RescatoA. La
inversa consulta el mismo vínculo con extremos intercambiados, sin otra copia.
El RON inicial acepta solo la forma canónica RescatadoPor. No se abren operaciones
arbitrarias para modificar filiación, conectividad o cualquier otro tipo.

Para Efectos, operaciones, índices, marcador/posición, continuación, asignación
de IDs y siguiente vista se preparan antes de instalarse juntos. Un fallo no
aplica parte del lote; los pasos anteriores permanecen.

AccionNpc conserva una garantía distinta: solicitud validada → ejecución única
→ resultado y avance retenidos → preparación de la siguiente presentación.
Si falla esa presentación, la sesión queda fatal con el resultado conservado;
no vuelve a ejecutar la acción ni revierte su RNG o lotes ya confirmados.

## 12. Ejemplos de autoría nueva

Estos fragmentos ilustran formato, no agregan sucesos canónicos.

Evento incrustado dentro del RON de un actor:

```ron
eventos: [(
    clave: "salida_habitual",
    activacion: EnFaseNpc,
    repeticion: UnaVezPorCierre,
    observacion: Silenciosa,
    condicion: r#"
        return mundo:ubicacion_actor(this) == "valle_fertil"
            and not mundo:actor_tiene(this, "muerto")
            and not mundo:en_grupo(this)
            and this ~= mundo:actor_jugador()
            and ctx:ahora().mes >= 2
    "#,
    inicio: "viajar",
    pasos: [
        AccionNpc(
            clave: "viajar",
            solicitar: r#"return acciones.viajar(this, "zona_arida")"#,
            siguiente: "fin",
        ),
        Fin(clave: "fin", control: Continuar),
    ],
)],
```

Evento externo fechado, con los actores referenciados explícitamente:

```ron
(
    clave: "liberacion_del_cazador",
    activacion: EnFecha(fecha: (anio: 2025, mes: 2, dia: 10)),
    observacion: Local(lugar: "nahal"),
    condicion: r#"
        return mundo:actor_tiene("cazador", "prisionero")
            and mundo:ubicacion_actor("cazador") == "nahal"
    "#,
    inicio: "liberar",
    pasos: [
        Efectos(
            clave: "liberar", salidas: ["relato"],
            resolver: r#"
                return {operaciones = {
                    op.trasladar_npc("cazador", "valle_fertil"),
                    op.quitar_caracteristica("cazador", "prisionero"),
                    op.registrar_rescate("cazador", "bandido")
                }, siguiente = "relato"}
            "#,
        ),
        Vigneta(
            clave: "relato",
            presentacion: r#"
                return {titulo = "Una partida apresurada",
                    texto = "Ves al cazador abandonar su encierro y perderse entre las calles."}
            "#,
            siguiente: Some("fin"),
        ),
        Fin(clave: "fin", control: Continuar),
    ],
)
```

El segundo ejemplo requiere un fixture con el Cazador independiente y prisionero
en Nahal. Una relación de rescate registra un hecho de autoría; no afirma que se
haya simulado una acción de Rescatar. Una escena de camaradería única puede usar
el mismo grafo PorPuntos, sin dueño, con condiciones sobre ambos compañeros;
su relación concreta requerirá el tipo y operación que se diseñen después.

## 13. Validaciones, errores y aceptación

Se conserva el rechazo de campos desconocidos, tipos/retornos Lua incorrectos,
grafos inválidos, referencias inexistentes y controles no autorizados. Se añade
la validación de ámbitos, propietario, activación única, repetición compatible,
modo de observación y pasos habilitados por contexto. Las referencias creadas
dentro de Lua se validan al ejecutarse. El diagnóstico identifica archivo,
propietario si existe, clave local/global, paso, activación y fecha nominal/final.

| Contrato | Prueba necesaria |
|---|---|
| Reutilización v1 | Mismas escenas, operaciones, fecha y RNG con el contenido externo actual. |
| Política adaptada | Cazador evaluado una vez por cierre; exclusiones, RNG, resumen y errores compatibles; sin doble inscripción. |
| Autoría | Mismo formato y capacidades según activación en archivo externo y actor. |
| Propietario | Dos actores con clave local igual conservan this y estados distintos; externo tiene nil. |
| Identidades | Propietario, dirigente, jugador y ejecutante distintos en una prueba. |
| Entorno | Reasignación global this rechazada; sombreado local no cambia permisos ni propietario. |
| Cadenas | Local→externo→otro local mantiene contexto y el this propio de cada definición. |
| Unicidad | Un evento único multi-actor no se duplica por consultar a cada participante. |
| Repetición | Condiciones verdaderas/falsas y exclusiones no reevalúan EnFaseNpc en el mismo cierre; sí en otro. |
| Cierre | Respuesta/finalización duplicada en cada fase no crea otro IdCierre ni repite calendario o NPC. |
| Elegibilidad | Evento nuevo de compañero/muerto puede activarse por sus condiciones; legado conserva sus exclusiones. |
| Fecha | Varias citas en salto temporal, inicialización inclusiva, límites y fracciones; omisión definitiva si falsa. |
| Observación | Resolución remota sin avisos automáticos; misma lógica al observarse salvo ramas explícitas del autor. |
| Acción | Viajar usa el motor actual, resultado retenido y rechazo reglado; leer no vuelve a ejecutar. |
| Efectos atómicos | Fallo en la última operación/vista no instala parte del paso Efectos ni consume ocurrencia/IDs. |
| Acción confirmada | Fallo en su texto posterior conserva resultado/avance y no repite el motor ni RNG. |
| Escala | 1.000 definiciones distribuidas entre activaciones, sin evaluar scripts de otros índices. |
| Independencia | Dos partidas comparten definiciones y no sus ocurrencias, cursores ni continuaciones. |

La aceptación usa fixtures aislados para legado, evento local repetible, escena
única compartida y cita local/remota. Se conservan los controles de la metodología.
No se introducen acontecimientos nuevos en el contenido base para probar motores.

## 14. Límites y estado del diseño

No hay preguntas bloqueantes para este alcance. No existe una categoría nueva
de comportamiento en el mundo; politica pasa a ser sintaxis heredada de autoría.
Un solo sistema narrativo admite varias activaciones y varios pasos autorizados.
Su integración requiere refactor de carga, registro y coordinación, no únicamente
renombrar campos.

La repetición en otros puntos, las recurrencias, los encadenamientos entre más
activaciones, acciones distintas de Viajar y la cronología intercalada requieren
ampliaciones explícitas. Sucesores, protagonistas alternativos y cierre de época
tienen diseños propios pendientes; ningún contexto se identifica por defecto
con el Príncipe.
