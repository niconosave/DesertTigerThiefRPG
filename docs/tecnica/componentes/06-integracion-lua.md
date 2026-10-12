# Integración de Lua para decisiones de NPC

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.1 |
| Fecha | 2026-10-12 |

## 1. Propósito y alcance

Este documento diseña cómo `desert-tiger-app` ejecutará código Lua asociado a un
NPC para decidir si solicita una acción. Desarrolla la fase descrita en el
[diseño general](../../diseno-general.md) dentro de los límites del
[diseño técnico general](../diseno-tecnico-general.md).

La integración permitirá que una persona que diseña contenido modifique las
condiciones de actuación de los NPC sin cambiar Rust. Lua no implementará las
reglas de una acción, no modificará el Mundo y no decidirá cómo mostrar sus
resultados.

Quedan fuera el orden de evaluación de NPC, el número de acciones permitido por
fase, la lógica particular de las acciones, las decisiones visuales y un sistema
avanzado de inteligencia artificial. Esos asuntos pertenecen respectivamente al
sistema de tiempo/NPC, al motor de acciones y al frontend.

## 2. Ubicación y dependencia

La integración será un módulo interno de `desert-tiger-app` y utilizará `mlua`
con un runtime Lua compilado mediante su opción `vendored`. De ese modo, el juego
no dependerá de la instalación de Lua de Manjaro ni de otra máquina de destino.
La versión de `mlua` y la versión del lenguaje Lua deberán fijarse juntas antes
de aprobar este documento.

Ni `desert-tiger-core` ni `desert-tiger-terminal` dependerán de `mlua`. El núcleo
recibirá solicitudes Rust y no sabrá si proceden del jugador, de Lua o de una
prueba.

Los scripts se leerán y comprobarán durante la carga atómica de contenido. Podrán
prepararse para evitar leer el filesystem en cada fase; esa caché no será
observable por las reglas.

## 3. Contrato de evaluación

Cada política estará vinculada por los datos de contenido a un único actor NPC.
La aplicación invocará conceptualmente una operación con esta forma:

```text
evaluar(política, actor_vinculado, contexto_solo_lectura)
    → ninguna_solicitud | solicitud_de_acción | error
```

El contexto será una instantánea de solo lectura construida por Rust para esa
evaluación. Podrá contener únicamente datos aprobados, entre ellos la fecha,
variables del Mundo, características y referencias necesarias para consultar el
estado disponible. La API exacta decidirá qué consultas se exponen; no se pasará
el objeto mutable del Mundo ni referencias Rust que permitan alterarlo.

Una respuesta de Lua será declarativa: identificará la acción, sus objetivos y
los parámetros permitidos. La identidad del actor será implícita y provendrá de
la asociación del script; Lua no podrá actuar en nombre de otro actor.

La aplicación convertirá la respuesta a los tipos del motor común.
`desert-tiger-core` volverá a comprobar actor, fase, acción, objetivos y
condiciones antes de cualquier cambio. Solicitar algo no autoriza a ejecutarlo.

Cuando la acción aceptada requiera una decisión intermedia, esa decisión se
resolverá mediante el azar controlado de la aplicación en la primera versión.
No se reimplementará el recorrido de la acción dentro de Lua.

## 4. Ausencia de mutación y estado oculto

La ejecución del script terminará antes de que su solicitud pueda modificar el
Mundo. Por ello, un error de sintaxis, ejecución, conversión o validación no podrá
dejar cambios parciales.

Una política no conservará variables globales mutables entre evaluaciones. En la
primera versión, cada evaluación creará una máquina y un entorno Lua nuevos,
instalará sus límites y cargará allí el script. Se podrá conservar el texto o el
resultado de su validación inicial, pero no funciones, tablas, upvalues, registro
ni otros objetos Lua entre ejecuciones.

Los datos persistentes de un NPC deberán expresarse mediante elementos del Mundo,
como características o variables, y solo podrán cambiar después a través de una
acción validada por el núcleo.

Lua no utilizará su propio generador global ni una semilla tomada del reloj. Si
se expone azar a los scripts, cada extracción procederá de la fuente inyectada de
la Partida y podrá reproducirse en pruebas. La forma exacta de esa operación sigue
pendiente.

## 5. Capacidades y aislamiento

El entorno ofrecerá la superficie mínima necesaria para evaluar condiciones y
construir una solicitud. No expondrá acceso al filesystem, red, procesos, shell,
variables de entorno, reloj del sistema, terminal ni carga arbitraria de código.
Tampoco entregará funciones que muten el Mundo o invoquen directamente una
acción.

Las bibliotecas y funciones de Lua se habilitarán mediante una lista explícita.
La carga dinámica, importación no controlada y acceso al sistema permanecerán
ausentes. La lista debe aprobarse junto con la API para mods.

Cada evaluación estará limitada en instrucciones y memoria para que un bucle
infinito o una asignación excesiva no bloquee indefinidamente el juego. `mlua`
ofrece mecanismos para instalar hooks de ejecución y establecer límites de
memoria, pero los presupuestos concretos se medirán y decidirán después. Los
límites se instalarán antes de cargar o ejecutar código del script. Un límite no
debe depender de la velocidad de la terminal ni avanzar la fecha del Mundo.

Estas restricciones reducen errores y abuso accidental. Al ejecutarse dentro
del proceso del juego, no prometen aislamiento perfecto frente a contenido
deliberadamente hostil; un aislamiento adversarial requeriría otro proceso y
queda fuera de la primera versión.

## 6. Entradas, salidas y diagnósticos

La entrada incluye el script validado, el actor asociado y una vista autorizada
del estado. Si posteriormente se aprueba una operación de azar para Lua, también
se proporcionará explícitamente desde Rust. La salida será una de tres categorías:

- ninguna acción solicitada;
- una solicitud declarativa que aún requiere validación del núcleo;
- un error estructurado.

Los errores indicarán, cuando la información exista, archivo de script, actor,
fase y causa original. La frontera convertirá errores de Lua y de tipos en errores
Rust explícitos; no usará `panic` para fallas de contenido.

Un error de sintaxis o preparación detectado al inicio invalida el conjunto de
contenido completo, conforme a la carga atómica. Para un error que solo aparece
durante una Partida, queda pendiente elegir si se omite ese NPC, se detiene la
fase de NPC o se interrumpe la sesión. Cualquiera sea la política, deberá
producir un diagnóstico visible para desarrollo, no aplicar una solicitud
parcial ni repetir indefinidamente el script fallido.

Una solicitud mal formada o rechazada por el núcleo también conservará el Mundo
intacto. Aún debe decidirse si se tratará como error del contenido o como una
decisión legítima que simplemente no puede ejecutarse.

## 7. Pruebas y criterios de aceptación

Las pruebas usarán contextos mínimos. Si se habilita una operación de azar para
Lua, utilizarán además una fuente predecible. Cubrirán:

- una condición falsa que no produce solicitud y una verdadera que sí la
  produce;
- conversión correcta de una solicitud declarativa;
- imposibilidad de reemplazar la identidad del actor vinculado;
- nueva validación por el núcleo, incluido el rechazo sin mutación;
- acceso de solo lectura y ausencia de capacidades de sistema;
- aislamiento de variables globales entre evaluaciones;
- reproducción del mismo resultado con igual contexto y, cuando corresponda,
  igual secuencia de azar;
- errores de sintaxis detectados durante la carga atómica;
- errores de ejecución y tipos con diagnóstico contextual;
- interrupción por presupuesto de instrucciones y rechazo por límite de memoria;
- incapacidad de un script fallido para alterar el Mundo o avanzar su fecha.

El componente será aceptable cuando un NPC pueda consultar el contexto permitido
y solicitar una acción común, pero no ejecutarla por fuera del núcleo, mutar
estado, suplantar otro actor ni acceder a capacidades externas. Los scripts
oficiales deberán validar y cualquier prueba con azar deberá ser repetible.

## 8. Preguntas pendientes

- ¿Qué versión de Lua y de `mlua` formarán el contrato de compatibilidad?
- ¿Cuál será la API visible: nombres, tipos, consultas y forma exacta del retorno?
- ¿Qué bibliotecas y funciones estándar integrarán la lista permitida?
- ¿Cómo solicitará azar un script, si necesita hacerlo?
- ¿Qué presupuestos de instrucciones y memoria tendrá cada evaluación?
- ¿Qué política seguirá el juego ante un error ocurrido durante una Partida?
- ¿Cómo se tratará una solicitud bien formada pero rechazada por las reglas?
- ¿Habrá una política por actor, varias reglas por actor o scripts compartidos?
- ¿Cómo se informarán diagnósticos a una persona jugadora y a una autora de mods?

## 9. Referencias técnicas

- [`mlua` — repositorio y características oficiales](https://github.com/mlua-rs/mlua)
- [`mlua::Lua` — documentación de la API](https://docs.rs/mlua/latest/mlua/struct.Lua.html)
