# Eventos narrativos y coordinación de ejecuciones

| Campo | Valor |
|---|---|
| Estado | Implementado, versión 1 |
| Versión | 1.2 |
| Fecha | 2026-11-06 |
| Alcance | Primera versión acotada y contratos de extensión |

> Ampliación implementada el 2026-11-11: el [contrato 25](./25-conceptos-relaciones-personales.md)
> incorpora Conceptos, relaciones personales, índices, autoría RON, fichas y
> consultas Lua. Gobierna esas extensiones respecto de los esquemas previos.

> Evolución diseñada, no implementada: el [contrato 24](./24-agenda-eventos-mundo.md)
> integra eventos externos, eventos dentro del actor y `politica` heredada bajo
> un catálogo, un coordinador y un registro común. Define `this`, fechas y
> quehacer NPC. El cuerpo de este documento describe la base v1 implementada;
> las diferencias del modelo objetivo pertenecen al 24.

## 1. Propósito y autoridad

El diseñador escribe momentos particulares de la historia, sus condiciones y
sus consecuencias. El motor no deduce gratitud, lealtad ni intenciones mediante
puntuaciones. Rust proporciona operaciones coherentes y Lua permite combinarlas
según el episodio. Un rescate no implica por sí mismo ninguna reacción de la
persona liberada.

Este documento es la fuente canónica de la base v1 implementada del sistema narrativo. Desarrolla el
[diseño general](../../diseno-general.md), la
[arquitectura](../diseno-tecnico-general.md) y los contratos de
[sesión](./04-sesion-aplicacion.md), [acciones](./05-motor-acciones.md),
[Lua NPC](./08-integracion-lua.md) y [operaciones](./11-operaciones-mundo.md).
La versión 1 está implementada. Las ampliaciones de la sección 14 permanecen
aplazadas y no forman parte de la API disponible.

## 2. Alcance de la primera versión

| Capacidad | Versión 1 |
|---|---|
| Autoría | Un RON por evento, con Lua incorporado; carga al inicio. |
| Activación | Puntos registrados, filtros declarativos y condición Lua. |
| Presentación | Secuencia finita de viñetas, decisiones y efectos. |
| Inserción | Antes de un paso preparado o después de resultados confirmados, según el punto. |
| Sustitución | Solo el relato genérico de llegada de Viajar, sin sustituir sus cálculos. |
| Interrupción | Cierre reglado de Viajar en un punto seguro; conserva avances y tiempo. |
| Continuación | Retomar una ejecución suspendida si sus precondiciones siguen siendo válidas. |
| Encadenamiento | Un evento activo y cola de eventos solicitados o habilitados por cambios. |
| Repetición | Una ocurrencia por evento y partida. |
| Efectos | Características de Actor, conocimiento y variables globales tipadas. |
| Tiempo | Los eventos no añaden duración propia; observan el instante efectivo de la acción. |
| NPC | Evaluación después de la fase NPC; no se suspende su ejecución interna. |

La primera versión no implementa Atacar, Rescatar o Reclutar ni inventa sus
puntos de intervención. Estas acciones incorporarán los mismos contratos cuando
se diseñen. Tampoco añade inventarios, objetos, modificación de relaciones,
reclutamiento, traslados escritos desde Lua, eventos repetibles, acciones iniciadas
desde eventos, corutinas Lua, guardado o agenda de fechas entre puntos.

Estas exclusiones acotan la entrega, no el modelo: las secciones 13 y 14 fijan
cómo ampliar operaciones y sustituciones sin dar acceso mutable al mundo.
No existe un máximo narrativo de viñetas ni se omiten acontecimientos para
abreviar un viaje.

## 3. Responsabilidades y módulos

| Capa | Módulos | Responsabilidad |
|---|---|---|
| `core` | `acciones/intervenciones.rs` | Puntos, tokens, contratos de continuación e interrupción. No conoce eventos concretos ni Lua. |
| `core` | `modelo.rs` / operaciones | Validar y preparar lotes; incorporar escrituras globales a la misma transacción. |
| `app` | `eventos/contenido.rs` | Descubrir RON, validar esquema, grafos, referencias y compilar Lua. |
| `app` | `eventos/lua.rs` | Contextos de consulta y constructores de solicitudes autorizadas. |
| `app` | `eventos/coordinador.rs` | Índices, selección, ejecución, cola, marcadores y confirmación. |
| `app` | `eventos/coordinador.rs` (tipos públicos) | Vistas neutrales, identificación de respuestas y referencias artísticas. |
| `app` | `sesion.rs` y `sesion/eventos_partida.rs` | Poseer el coordinador y la ejecución raíz; compartir el único cierre temporal. |
| `terminal` | Adaptador de viñetas | Dibujar, paginar y recoger elecciones mediante la fachada pública. |

Las definiciones son inmutables y compartibles. Cada Partida posee sus propios
estados de evento, cola y ejecución suspendida. No hay archivos que se reescriban
al jugar ni estado persistente dentro de una máquina Lua. No se crean procesos
ni hilos de simulación para los eventos.

## 4. Puntos de intervención del motor

### 4.1. Cesión explícita

Una acción ofrece el control en límites semánticos. El coordinador no intercepta
líneas arbitrarias de Rust, ni interpreta la prosa para adivinar qué sucede.

La salida del motor incorpora un envoltorio:

```rust
pub enum SalidaMotor {
    Punto(PuntoIntervencion),
    Paso(Box<PasoAccion>), // escena, decisión o finalización ya existentes
}

pub struct PuntoIntervencion {
    pub id: IdIntervencion,
    pub contexto: ContextoIntervencion, // incluye clase

}

pub enum ResolucionPunto {
    EjecutarGenerico,
    OmitirRelato,
    Interrumpir,
}
```

`IdIntervencion` es opaco, identifica una única visita al punto dentro de una
instancia de ejecución y nunca se serializa en contenido. Los puntos de sesión
los emite la aplicación y conservan una continuación propia, sin fabricar una
acción ni un token del motor; las vistas poseen su identificador de presentación. El núcleo conserva el
punto pendiente y rechaza resolverlo dos veces o con un token anterior.
Las funciones `iniciar`, `avanzar` y `responder` de
`acciones::intervenciones` entregan `SalidaMotor`; la reanudación se hace
mediante `resolver_punto(mundo, ejecucion, id, resolucion, contexto, azar)`, con
los mismos tipos de contexto y fuente de azar del motor actual.

El envoltorio conserva la máquina de Viajar existente como ejecución interior.
Los consumidores sin eventos, incluidas las políticas NPC, mantienen la API
genérica de `acciones`. No se duplica la simulación ni sus reglas.

Antes de `Punto` no se calculan ni confirman operaciones de la unidad siguiente.
Los resultados de unidades anteriores ya confirmadas permanecen confirmados.
`EjecutarGenerico` ejecuta la unidad pendiente una sola vez; `OmitirRelato` solo
es válido en un punto que lo autoriza; `Interrumpir` produce el cierre reglado.
La integración de `viajar.inicio` debe separar la creación y validación pura de
la ejecución de sus primeras búsquedas de ruta. La búsqueda actual que utiliza
azar debe ocurrir después de resolver ese punto, nunca al preparar su contexto.
Si no hay eventos registrados, la ejecución conserva resultados, orden de
chequeos, consumo de azar y tiempo del juego actual.

Interceptar únicamente `Partida::publicar` es insuficiente: allí las consecuencias
de una escena ya han sido confirmadas. Los puntos previos deben existir en el
motor, antes de esas consecuencias.

### 4.2. Catálogo inicial

Las claves de autoría se traducen a un enum registrado. Una clave desconocida o
reservada para una ampliación se rechaza en carga.

| Clave RON | Momento exacto | Sustituir | Interrumpir |
|---|---|---|---|
| `partida.disponible` | Al crear la partida y después de completar el cierre anterior, antes de admitir otra acción. | No | No |
| `viajar.inicio` | Ejecución creada, antes de su primera transición, tirada o coste. | No | Sí |
| `viajar.entre_lugares` | Tras completar una conexión, sus incidentes, traslados y contabilización; antes de comenzar la unidad siguiente. | No | Sí |
| `viajar.relato_llegada` | Destino y consecuencias de llegada confirmados, antes de publicar su relato genérico. | Sí | No |
| `acampar.inicio` | Acampada válida preparada, antes de mostrar las fichas. | No | No |
| `accion.terminada` | Resultado raíz calculado, antes de confirmar calendario y fase NPC. | No | No |
| `fase_npc.terminada` | Tras completar una única fase NPC y antes de volver a disponible. | No | No |

Una conexión de Viajar sigue resolviendo sus dos mitades según las reglas
vigentes. No se añade una cesión entre esas mitades. El episodio montañoso puede
ceder al alcanzar un Lugar confirmado solo cuando termina su momento en curso;
no se interrumpe a mitad de un chequeo o de un lote. Cada llegada efectiva a un
nuevo Lugar genera una visita al confirmar el momento que expone ese traslado.
Si un momento montañoso agrupa varios centros, ofrece una única visita después
de ese momento, en su último Lugar confirmado; no fabrica puntos retrospectivos.

`partida.disponible` se ofrece una vez por entrada real en ese estado; no por
consultar `vista()`, mover un foco, abrir el menú de desarrollo o dibujar.
La entrada y salida de la ficha de un actor no son puntos de intervención.

### 4.3. Contexto y hechos

`ContextoIntervencion` conserva: clase y token del punto, identidad de la ejecución
raíz si existe, tipo de acción, Actor dirigente, objetivo tipado opcional, último
Lugar confirmado, origen/destino de la unidad cuando correspondan, participantes,
instante efectivo y una copia inmutable del resultado confirmado, si existe.
El contexto retenido es un valor poseído; no contiene un préstamo al Mundo que
sobreviva a otras viñetas o mutaciones.

La presentación y los efectos de un evento conservan ese contexto de activación.
Las consultas al Mundo leen siempre el último estado confirmado. El contexto
no finge que un objetivo cambió porque otro evento alteró alguna variable.
No contiene punteros mutables ni detalles privados de las fases de Viajar.

Valores de contexto obligatorios por punto:

| Punto | `actor` | `accion` / `objetivo` | `lugar` |
|---|---|---|---|
| `partida.disponible`, `fase_npc.terminada` | Jugador | `nil` / `nil`; no se arrastra la acción anterior. | Ubicación actual del grupo. |
| `viajar.inicio`, `viajar.entre_lugares`, `viajar.relato_llegada` | Dirigente | `viajar` / Lugar de destino adoptado. | Último Lugar confirmado. |
| `acampar.inicio` | Jugador | `acampar` / Lugar del campamento. | Lugar del campamento. |
| `accion.terminada` | Dirigente de la raíz | Tipo y objetivo de la acción que está cerrando. | Lugar final confirmado. |

El resultado confirmado está ausente en el inicio, disponible como cierre en
`accion.terminada` y, en los puntos de viaje posteriores, corresponde a la unidad
recién completada. Los puntos de sesión no conservan un resultado de la acción
anterior. Un evento encadenado conserva el contexto de origen de su solicitud.

Los futuros puntos de Rescatar deberán distinguir intento, prueba y resultado
confirmado, y entregar la persona objetivo y su resultado. No se reutiliza una
cadena de texto de Viajar para representar un rescate.

## 5. Esquema de contenido

### 5.1. Ubicación, claves y compatibilidad

Se descubren `events/**/*.ron` dentro de la raíz de contenido, opcionalmente
ausente. Sin esa carpeta, o vacía, no hay eventos. Se conserva la política de
rutas y contención del cargador de contenido. No se modifica `world.ron` con
una lista central de eventos.

La evolución del contrato 24 mantiene este descubrimiento: todas las
subcarpetas usan el esquema común. `events/calendario/` no es una carpeta
reservada ni un formato distinto. Se conserva la carga de los RON v1 y se
normalizan junto con las definiciones nuevas.

Cada archivo contiene exactamente un evento. `clave` debe coincidir con su
nombre de archivo sin extensión y ser globalmente única, aunque existan
subcarpetas. Las claves de eventos, pasos, opciones e ilustraciones siguen la
sintaxis de claves externas ya utilizada en el contenido. El orden físico de
archivos o pasos no determina la prioridad de ejecución.

Esquema normativo de `EventoDto`:

| Campo | Tipo / valor por defecto |
|---|---|
| `clave` | Cadena obligatoria. |
| `puntos` | Lista no vacía de claves registradas, sin repetidos. |
| `modo` | `Insertar` por defecto; alternativa `SustituirRelato`. |
| `prioridad` | `i32`, cero por defecto; mayor valor primero. |
| `repeticion` | `UnaVezPorPartida` por defecto; única variante v1. |
| `filtros` | Registro vacío por defecto: `actor`, `lugar`, `objetivo`, todos opcionales. |
| `condicion` | Bloque Lua obligatorio; devuelve exactamente un booleano. |
| `inicio` | Clave de paso existente. |
| `encadena` | Lista de claves de otros eventos que este puede solicitar; vacía por defecto. |
| `pasos` | Lista no vacía de pasos con claves únicas. |

`actor` y `lugar` de los filtros son claves externas; `objetivo` usa
`Actor("clave")` o `Lugar("clave")`. Deben existir en el contenido. Un filtro
cuyo dato no está disponible en ese punto no coincide. Varios filtros se unen
con AND; varios puntos con OR.

`SustituirRelato` solo puede declarar puntos que autoricen sustitución. En v1,
por tanto, solo `viajar.relato_llegada`. No se acepta `ReemplazarMovimiento` ni
un campo de duración: capacidades no soportadas producen error de carga.
Todos los registros rechazan campos desconocidos.

### 5.2. Grafo finito de pasos

Los pasos tienen tres variantes:

- `Vigneta`: `clave`, `presentacion` Lua, `ilustracion: Option<String>`,
  `expresion: Option<String>`, `opciones` y `siguiente: Option<String>`.
- `Efectos`: `clave`, lista obligatoria `salidas` de destinos posibles y bloque
  Lua `resolver`.
- `Fin`: `clave`, `control: Continuar | InterrumpirAccion`.

Las listas opcionales valen vacío y los campos `Option` valen `None` al omitirse.
En RON los valores presentes de `Option` usan `Some(...)`. Una `Vigneta` sin
opciones exige `siguiente`; con opciones debe omitirlo. Cada opción es un registro
`(clave, texto, siguiente)`, con clave única dentro del paso. En v1 todas las
opciones declaradas están disponibles; las opciones condicionales se aplazan.

`presentacion` devuelve exactamente una tabla `{titulo, texto, detalle?}`: título
y texto no vacíos; detalle opcional. No cambia datos ni consume azar.
`resolver` devuelve `{operaciones, siguiente, solicitar?}`: operaciones es una
lista de solicitudes opacas, siguiente pertenece a `salidas` y solicitar es una
lista de claves declaradas en `encadena`. Las listas vacías son válidas.
No se admiten claves adicionales ni múltiples valores de retorno.

El grafo incluye los destinos estáticos y todas las `salidas` declaradas. Todos
los pasos deben ser alcanzables desde `inicio`; todos los caminos terminan en
`Fin`, sin ciclos. No hay un límite arbitrario de longitud. Los eventos no se
invocan recursivamente como pasos de otro evento.

### 5.3. Ejemplo válido y sin personajes nuevos

`events/viajes/recuerdo_del_hogar.ron`:

```ron
(
    clave: "recuerdo_del_hogar",
    puntos: ["viajar.entre_lugares"],
    filtros: (actor: Some("principe_perdido")),
    condicion: r#"
        return mundo:en_grupo("sirviente")
            and mundo:actor_tiene("sirviente", "musico")
            and not mundo:actor_tiene("sirviente", "muerto")
            and not mundo:booleana("recuerdoDelHogar", false)
    "#,
    inicio: "recordar",
    pasos: [
        Vigneta(
            clave: "recordar",
            presentacion: r#"
                return {
                    titulo = "Una canción conocida",
                    texto = "Tu compañero entona una canción que conocés desde niño. Durante un momento, el camino te devuelve el recuerdo de tu hogar."
                }
            "#,
            siguiente: Some("registrar"),
        ),
        Efectos(
            clave: "registrar",
            salidas: ["fin"],
            resolver: r#"
                return {
                    operaciones = {op.establecer_booleana("recuerdoDelHogar", true)},
                    siguiente = "fin"
                }
            "#,
        ),
        Fin(clave: "fin", control: Continuar),
    ],
)
```

Es un ejemplo de formato, no contenido aprobado para la partida. No simula que
el Sirviente decidió cantar mediante un algoritmo de emociones.

## 6. Estado por partida y selección

Cada definición tiene `NoOcurrido`, `EnCurso` o `Terminado` en la Partida. Marcar
`EnCurso` reserva el evento; solo se pasa a `Terminado` al alcanzar `Fin`.
La reserva inicial se confirma junto con la preparación válida del primer paso;
una reserva transitoria no escribe marcadores por sí sola. Un fallo técnico no
permite volver a iniciar el mismo episodio desde el principio.

El catálogo indexa eventos por `ClasePunto`. Ante una visita:

1. Se consideran solo los eventos de ese punto que siguen `NoOcurrido`.
2. Se aplican filtros y condición Lua contra el estado confirmado.
3. Se elige por prioridad descendente y luego clave ascendente.
4. Se reserva y ejecuta un evento completo; luego se reconsideran candidatos.
5. Al no quedar candidatos, se resuelve el punto y continúa su ejecución.

Un candidato se revalida al seleccionarlo, no se conserva una decisión de
activación tomada antes de los cambios de otro evento. No se ejecutan condiciones
por frame, por redimensionar o por consultar vistas. Un evento activo no abre
otro dentro de sus pasos: las condiciones naturales se reconsideran al terminar.
Una causa que deba sobrevivir a cambios posteriores se registra como hecho
persistente o como solicitud explícita; no se detectan estados pasajeros entre
instrucciones Lua. Lua arbitrario no se analiza
para adivinar todas sus dependencias. Los filtros son una optimización explícita,
no una razón para omitir eventos que sí los satisfacen.

Solo un evento puede ganar la sustitución de un relato en cada visita. Tras
reservar al primero elegible, ese espacio queda ocupado; los demás sustitutos
no se consumen y pueden ser candidatos en otra visita. Los eventos `Insertar`
siguen el orden normal. Al terminar el ganador se marca el relato pendiente como
omitido; no se oculta una escena que ya se haya mostrado.

La cola de solicitudes explícitas se atiende después de `Fin` y antes de
reconsiderar los eventos normales del punto. Conserva orden de solicitud, contexto
de origen y clave solicitante. Solicitudes duplicadas se unifican por evento;
un evento `EnCurso` o `Terminado` no vuelve a encolarse. Para seleccionar un evento
encadenado se vuelven a comprobar filtros y condición, pero no se exige coincidencia
con `puntos`: la solicitud explícita es su otro origen de activación. Si no se
cumplen, la solicitud se retira y el evento conserva `NoOcurrido`.

Antes de confirmar una solicitud explícita se comprueba que el punto real de
origen permite el modo y todos los controles del evento solicitado. Un evento
encadenado no puede sustituir desde un punto que no autoriza sustitución ni
interrumpir desde uno que no autoriza interrupción. Esas incompatibilidades son
errores de contenido en ejecución, no permisos obtenidos por omitir `puntos`.
Un sustituto cuyo espacio ya está ocupado no se selecciona ni se consume.

El grafo estático `encadena` debe ser acíclico; se rechaza solicitarse a sí mismo.
Las activaciones automáticas también terminan: v1 solo consume cada evento una
vez. Ningún contador corta una historia válida tras cierto número de escenas.

## 7. Coordinador y continuación

El coordinador posee un evento activo como máximo, su paso, el punto pendiente,
el ocupante de sustitución, la cola y los marcadores. La raíz suspendida es un
valor de datos poseído, nunca un préstamo al Mundo ni una pila Lua:

```text
raíz: Viajar(ejecución) | Acampar(acampada) | CierrePendiente(resultado) | Ninguna
punto pendiente y token
contexto de activación
estado de evento / paso / respuesta pendiente
cola y marcadores por partida
```

Una `Vigneta` genera una vista y espera confirmación o elección. `Efectos` y
`Fin` son pasos internos: no generan pantallas vacías. Al resolver un punto se
invalida su token; no se vuelve a ejecutar la entrada de la fase suspendida.

`Continuar` retoma la evaluación del punto y después su continuación. Antes de
confirmar efectos, el adaptador de la raíz valida que el mundo provisional siga
permitiendo esa continuación. En Viajar con trabajo pendiente deben seguir siendo válidos el dirigente,
los participantes, su ubicación conjunta, el destino y el estado transitorio.
En el relato final y en `CierrePendiente` solo se exigen las invariantes de cierre
y proyección: no se exige que el grupo pueda iniciar otro movimiento. Con raíz
`Ninguna` se validan el Mundo y la proyección. No se rechaza automáticamente una
consecuencia final porque ya no sea posible seguir viajando.
En `Acampar` preparado se exige que aún pueda abrirse el campamento y que su
Grupo conserve las invariantes actuales. Los nuevos estados no letales,
conocimientos y variables pueden conservarse;
no se vuelve a sortear ninguna prueba confirmada ni se rehace la ruta adoptada.

Si un lote vuelve inviable continuar, debe existir una salida válida
`InterrumpirAccion` para esa rama. En v1 un paso de efectos que invalide la raíz
solo puede dirigir inmediatamente a un `Fin(InterrumpirAccion)`; se rechaza antes
de confirmar si el punto no permite ese cierre. No se espera hasta reanudar para
descubrir que la ejecución quedó incoherente.

La interrupción de Viajar añade un cierre `InterrumpidoPorEvento`, conserva
último Lugar confirmado, horas, consumo, cambios e incidentes ya sucedidos y
concluye mediante el mismo cierre temporal de una acción normal. No es un
rollback ni una nueva acción. `InterrumpirAccion` queda pendiente mientras se
atienden las solicitudes explícitas de esa visita; no se seleccionan más eventos
naturales de ese punto. Los siguientes `Continuar` no anulan el cierre pedido.
Para esas solicitudes ya se valida una raíz que va a cerrar, no la capacidad de
seguir moviéndose. Al vaciar la cola se cierra la acción y se ofrecen sus puntos
posteriores. No se interrumpe otra vez desde `accion.terminada`.

El coordinador no permite iniciar una segunda acción general desde Lua en v1.
Los eventos encadenados son una cola, no acciones simultáneas ni eventos anidados.

## 8. Lua de eventos

### 8.1. Entorno y consultas

Se utiliza la dependencia `mlua` ya fijada, con las restricciones y límites de
memoria e instrucciones del [contrato Lua](./08-integracion-lua.md). Se compilan
los bloques una vez al cargar; cada invocación tiene un entorno nuevo, sin estado
Lua entre llamadas. No se copian el Mundo ni todas sus variables por condición:
las consultas resuelven claves e índices del estado confirmado de solo lectura.

No hay filesystem, red, reloj real, `math.random` ni fuente de azar disponible.
Los scripts no reciben punteros Rust ni referencias mutables. La API de políticas
NPC conserva sus permisos actuales; no hereda los constructores de efectos.

Consultas iniciales (validación estricta de cantidad y tipos de argumentos):

| Función Lua | Resultado |
|---|---|
| `ctx:punto()` | Clave del punto de origen. |
| `ctx:accion()` | `"viajar"`, `"acampar"` o `nil`. |
| `ctx:actor()` | Clave del dirigente o `nil`. |
| `ctx:lugar()` | Último Lugar confirmado o `nil`. |
| `ctx:objetivo()` | Tabla de solo lectura `{tipo="Actor"/"Lugar", clave=...}` o `nil`. |
| `ctx:participantes()` | Lista inmutable de claves de Actor del contexto, incluido el dirigente. |
| `ctx:ahora()` | Tabla de solo lectura con `anio`, `mes`, `dia`, `fraccion_numerador`, `fraccion_denominador`. |
| `mundo:actor_tiene(actor, caracteristica)` | Booleano. |
| `mundo:lugar_tiene(lugar, caracteristica)` | Booleano. |
| `mundo:ubicacion_actor(actor)` | Clave de Lugar o `nil` si está fuera del escenario. |
| `mundo:en_grupo(actor)` | Compañero del Grupo; el jugador no pertenece a esa lista. |
| `mundo:hay_relacion(tipo, origen, destino, mercancia?)` | Booleano; extremos `{tipo,clave}`. |
| `mundo:booleana(clave, defecto)` | Booleano; defecto solo si la variable no existe. |
| `mundo:entera(clave, defecto)` | Entero `i64`, con la misma regla. |
| `mundo:texto(clave, defecto)` | Cadena, con la misma regla. |

Los extremos de `hay_relacion` se validan como en los RON de relaciones; `Compra`
se normaliza a la venta inversa y exige mercancía igual que `Vende`. No deriva
parentescos nuevos. Una clave de Actor/Lugar inexistente o un tipo incompatible
es un error, no un falso silencioso. Las variables usan las claves dinámicas ya
aprobadas; un tipo existente incorrecto es error aunque se haya pasado defecto.
Las referencias estructurales se validan al cargar; las construidas dentro de
Lua se validan al usarse, sin pretender analizar estáticamente su código.

### 8.2. Constructores de efectos

Solo `resolver` recibe el módulo `op`. Sus funciones devuelven solicitudes
opacas e inmutables; llamarlas no modifica el Mundo:

```lua
op.anadir_caracteristica(clave_actor, clave_caracteristica)
op.quitar_caracteristica(clave_actor, clave_caracteristica)
op.registrar_conocimiento(clave_actor, clave_lugar)
op.establecer_booleana(clave_global, valor_booleano)
op.establecer_entera(clave_global, valor_i64)
op.establecer_texto(clave_global, valor_utf8)
```

`operaciones` solo admite objetos producidos por estos constructores; no tablas
que imiten comandos Rust. Una lectura Lua posterior a construir una solicitud
sigue viendo el Mundo confirmado, no un efecto pendiente. Para consultar el
resultado se utiliza el paso siguiente, después de confirmar.

Las escrituras globales crean claves inexistentes sin cambiar Rust y conservan
el tipo de las existentes. Los valores y las características obedecen sus
contratos actuales; los textos se sanean antes de mostrarlos en terminal.
Los helpers futuros conservarán esta frontera: producir solicitudes o una
continuación tipada, nunca ejecutar recursivamente una acción con sus viñetas.

## 9. Confirmación atómica por paso

`Mundo::aplicar_lote` actual no cubre todas las garantías necesarias. La
implementación ampliará su preparación para incluir escrituras globales y
expondrá una preparación opaca antes de confirmar. Un lote vacío de un paso
narrativo es válido y no obliga a inventar una operación.

Una transición de evento se prepara en la aplicación con:

- operaciones y mundo provisional resultante;
- validación de continuación de la raíz sobre ese resultado;
- presentación siguiente del mismo evento, si corresponde, ya validada contra ese resultado;
- marcador del evento, próximo paso, solicitudes de cola y control del punto;
- vista e identificadores nuevos listos para publicar.

Si todo es válido, un único punto de confirmación instala esos datos. Entre
preparación y confirmación no se atiende otra intención ni se invoca Lua de
nuevo. La aplicación no conserva un segundo Mundo autoritativo: el provisional
es transitorio y no se publica. La confirmación preparada no ejecuta nuevas
validaciones que puedan fallar a mitad de la instalación.

Confirmar `Fin` solo instala marcadores, cola y resolución pendiente del punto.
No llama al motor genérico ni prepara su próxima viñeta dentro de esa
transacción. Después, en otra transición, se resuelve la continuación del motor.
Si esta falla, se informa un fallo del motor sobre el estado ya confirmado;
no se promete revertir efectos narrativos ni azar de una transición posterior.

Una `Vigneta` relata lo ya confirmado o una decisión todavía pendiente. Si el
texto dice que una persona recibió una característica, el paso `Efectos` debe
precederlo. Elegir una opción cambia la rama, y los efectos de esa elección se
confirman antes de su viñeta de resultado. Leer páginas o volver a dibujar no
aplica efectos.

La atomicidad es por paso, no por episodio entero: lo confirmado en pasos
anteriores permanece. Si falla una operación, una salida Lua o la preparación
de la siguiente vista, no queda una parte del paso actual aplicada.

No se promete rollback de RNG: la fuente actual no lo ofrece. V1 no consume
azar desde eventos. Un chequeo ya realizado por una acción se conserva como
resultado de esa ejecución; jamás se vuelve a tirar por una reanudación.

## 10. Tiempo, recursos y fase NPC

El contexto usa `InstanteAccion` efectivo: inicio más el tiempo acumulado de la
raíz. Durante Viajar, `Mundo.fecha` puede conservar aún la fecha inicial y no se
expone como si fuera la hora actual. La barra, el contexto Lua y la viñeta deben
coincidir en la fecha proyectada.

Los eventos v1 tienen duración cero. No consumen días por leer texto ni reinician
la contabilización de provisiones. La actividad genérica conserva sus tiempos y
consumos incluso si se sustituye su relato. Un evento intermedio no llama a
`concluir_accion`, no confirma el calendario y no ejecuta una fase NPC.

Secuencia de cierre: `accion.terminada` → confirmar una vez fecha y fracción →
una fase NPC → `fase_npc.terminada` → `partida.disponible`. Tras una interrupción
se recorre el mismo cierre, conservando tiempo ya transcurrido. Los eventos que
se muestren tras la fase NPC no la vuelven a ejecutar.

La fase NPC usa directamente el motor genérico anterior, sin el envoltorio
`acciones::intervenciones` y sin consultar el catálogo de eventos. Así conserva
las reglas y el orden de azar sin abrir ventanas ni guardar una fase NPC
suspendida. Los cambios resultantes pueden habilitar eventos en
`fase_npc.terminada`, cuyo contexto corresponde a la sesión del jugador.

Una condición de fecha se evalúa en los puntos disponibles. Por ahora no hay una
agenda que inserte una fecha intermedia durante un salto de varios días: no se
promete que un `dia == 3` llegue a observarse. Para condiciones persistentes el
autor puede comprobar que el instante sea posterior a un umbral. Los hitos
exactos y el tiempo propio de eventos pertenecen a la sección 14.

## 11. Vistas y recursos gráficos

La fachada añade `PresentandoEvento` y `EsperandoDecisionEvento` con una vista
común: identidad opaca de presentación, clave de evento y paso, título, relato,
detalle opcional, opciones con claves estables e `IdOpcionEvento` opaco, contexto visual y referencia
opcional de ilustración/expresión. No utiliza `OpcionViaje` para elecciones Lua.
El contexto visual conserva el último Lugar confirmado y el instante efectivo;
la ventana de evento se dibuja sobre el mundo y los retratos como otras viñetas.

Las intenciones son `ContinuarEvento { presentacion }` y
`ResponderEvento { presentacion, opcion }`. La aplicación valida la vista activa,
el token y la pertenencia de la opción al paso actual. Una respuesta anterior,
una confirmación duplicada o una opción inventada no consume un evento ni aplica
operaciones. Entrar en una nueva vista narrativa invalida la respuesta del paso
genérico que quedó suspendido; al retomarlo se emite una presentación nueva.

La paginación de texto y detalle es local al frontend. Solo confirmar la última
página transmite una intención. Los textos siguen la validación de presentación
actual, sin controles de terminal ni inferencias de reglas a partir de la prosa.

Las claves `ilustracion` se resuelven en un archivo opcional
`assets/terminal/events.ron`, esquema `(ilustraciones: {"clave": "arte"})`.
El arte conserva las reglas de las ilustraciones actuales: texto rectangular,
caracteres de una columna y transparencia/recorte según el renderizador de
viñetas, sin introducir color nuevo en esta versión. Las variantes de ese archivo
en `alternativos/ascii/` y `alternativos/fuente/` sustituyen entradas por clave,
con fallback normal. Una clave sin recurso muestra solo texto; un recurso
presente inválido se diagnostica. Los datos de evento no incluyen rutas ni tipos
de Ratatui. Una expresión ausente en un retrato conserva su cara base y los
parpadeos del frontend.

La interfaz debe poder inicializarse con una vista de evento: no puede exigir
que `Partida::nueva` siempre entregue `Disponible`. Puede consultar
`mundo_visible()` para construir su fondo inicial. No se fuerza el campamento ni
se oculta su coste de salida para poder mostrar un evento.

## 12. Errores y diagnóstico

La carga diagnostica archivo, clave y campo: duplicados, puntos o modos no
soportados, referencias estructurales inválidas, grafos incompletos/cíclicos,
campos desconocidos, texto no presentable y errores de compilación Lua.
Un `Fin(InterrumpirAccion)` alcanzable solo es válido si todos sus puntos
naturales permiten interrumpir. Las solicitudes explícitas también validan el
punto real antes de empezar; no eluden ese contrato.

Los fallos de ejecución distinguen condición, presentación, efectos,
conversión de respuesta, límite Lua, operación de dominio y continuación inválida.
Incluyen evento, paso, punto, archivo y línea Lua cuando exista. La Partida
conserva el estado confirmado y entra en su vista de error fatal; no ignora el
episodio ni reintenta silenciosamente una tirada o un callback con efectos.
El tratamiento de errores de política NPC permanece independiente.

Los rechazos de una intención vieja son recuperables y no modifican nada.
La ausencia de eventos o ilustraciones es una situación válida.

## 13. Contratos para nuevas acciones y helpers

Una acción nueva registra puntos con nombre, fase anterior/posterior a la
confirmación, contexto, resolución genérica pendiente y controles permitidos.
Debe probar que sin eventos conserva sus reglas y que resolver un token dos
veces no repite nada. Añadir un archivo Lua no crea un punto Rust inexistente.

Cada helper de dominio nuevo debe definir precondiciones, resultado semántico,
operaciones, tiempo, consumo de recursos y estado transitorio que afecta.
Crear/quitar relaciones tendrá que actualizar sus índices; reclutar deberá
preservar las invariantes del Grupo. Un helper para una persona concreta se
escribe en Lua usando esas operaciones, no como `rescatar_capitana` en el núcleo.

No se incorporan una jerarquía universal de emociones ni una interpretación de
motivos. Un evento puede escribir un desenlace completamente particular y usar
reglas generales únicamente donde el diseñador decida que corresponden.

## 14. Ampliaciones previstas y límites de compatibilidad

- **Sustitución mecánica:** exige un punto previo con un contrato tipado de la
  unidad sustituida: resultado, operaciones, duración, consumo, avances y
  continuación. El adaptador del núcleo valida ese resultado antes de saltar la
  unidad genérica. Un `Continuar` o la ocultación de su texto no lo reemplaza.
  No se admite sustituir el movimiento de Viajar mediante la v1.
- **Eventos con tiempo propio:** deben entregar duración al adaptador raíz y
  atravesar la misma contabilización de comida, agua y demás costes de la acción.
  No basta sumar días a `Mundo.fecha`. Una agenda futura calculará fechas de
  intervención sin visitar todos los días ni perder hitos por saltos temporales.
- **Acciones o chequeos solicitados por Lua:** producirán una solicitud tipada y
  una continuación identificada. El coordinador ejecutará la acción fuera del
  callback y registrará su resultado una vez. Requerirán decidir la garantía de
  RNG antes de afirmar que sus efectos pueden revertirse.
- **Cambios estructurales y replanificación:** un evento que mueva al grupo,
  cambie compañeros, reservas o ruta necesitará reconciliar la ejecución raíz o
  cerrarla explícitamente. No se permite continuar usando sus datos anteriores.
- **Eventos dentro de NPC:** exigirán conservar también posición de la fase,
  ejecución NPC, resúmenes parciales y su intervalo temporal. La v1 no expone esos
  puntos ni altera la regla de que las acciones NPC no avanzan la fecha mundial.
- **Repetición y enlaces de retorno:** precisarán una identidad de ocurrencia y
  una política concreta de consumo. No se añaden bucles Lua ni se cambia el
  significado de `UnaVezPorPartida`.
- **Persistencia:** se guardarán pasos y contextos explícitos, no la pila de Lua.
  Versionar definiciones será parte del contrato de guardado, todavía aplazado.

## 15. Verificación y aceptación

| Contrato | Prueba necesaria |
|---|---|
| Ausencia de eventos | Viajar/Acampar mantienen comportamiento, tiempos y secuencia RNG. |
| Condición y presentación | Consultarlas repetidamente no altera mundo, fecha, RNG ni marcadores. |
| Inserción previa | La unidad siguiente no se calcula mientras el evento está pendiente. |
| Sustitución | Se ve el relato particular una vez; el genérico no se publica y sus cálculos no se repiten. |
| Reanudación | Continúa la fase exacta y conserva resultados, provisiones y participantes válidos. |
| Interrupción | Conserva lugar y avances previos, cierra una vez calendario y NPC. |
| Atomicidad | Fallar la última operación o la siguiente presentación narrativa no aplica parte del paso; Fin no consume RNG del motor. |
| Respuestas repetidas | Enter/opción antiguos no aplican dos veces efectos ni consumen otro evento. |
| Encadenamiento | B observa cambios confirmados de A; valida permisos del punto real; A puede pedir cierre y aun mostrar B antes de cerrar. |
| Competencia | Prioridad/clave estables; se revalida tras cambios y solo un sustituto gana. |
| Grafos | Referencias, ciclos y modos no soportados fallan durante carga con origen legible. |
| Estado independiente | Dos Partidas no comparten variables, cola ni eventos consumidos. |
| Tiempo efectivo | Lua, barra y calendario final coinciden, incluyendo una fracción de día. |
| Escala | Fixture de 1.000 eventos de varios puntos verifica que solo se evalúa el índice pertinente. |
| Frontend | Evento inicial, decisiones, detalle, resize, fondo, variantes y retorno sin reiniciar acción. |

La aceptación jugable de la primera implementación requiere un evento entre
Lugares de Viajar que marque una variable y reanude; una sustitución del relato
de llegada; una decisión que interrumpa un viaje; y A que habilite o solicite B.
Serán fixtures de desarrollo, sin declarar canon narrativo sobre la capitana ni
otros personajes todavía no diseñados. No se crean planes en este documento.

## 16. Decisiones pendientes

No hay decisiones técnicas abiertas para el alcance v1. Las capacidades de la
sección 14 no se presentan como implementadas ni como sintaxis aceptada. El
contenido dramático definitivo y los puntos de acciones todavía inexistentes se
diseñarán en sus documentos específicos.

## Ampliación de la apertura del príncipe

La campaña en `content/base/events/apertura/` utiliza dos constructores nuevos
de Efectos: `op.incorporar_companero(actor)` y
`op.separar_companero(actor, lugar)`. La incorporación exige un Actor vivo,
distinto del jugador, en el mismo Lugar directo y fuera de un colectivo. Es
idempotente y se confirma con el resto del lote, conservando su atomicidad.
La composición no puede cambiar en un Viajar que después vaya a continuar.

`ctx:ahora().dia_absoluto` expone el ordinal del calendario, comenzando en cero
el 1 de enero del año 1. Sirve para plazos relativos entre fechas reales, sin
contar meses artificialmente ni incrementar contadores por cada consulta.
