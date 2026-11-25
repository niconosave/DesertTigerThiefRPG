# Objetos y ubicaciones compartidas

Estado: Implementado; pendiente revisión de jugabilidad. Fecha: 2026-11-20.

## Alcance

Un objeto es un elemento persistente con identidad, nombre, descripción,
características y relaciones. Representa piezas particulares de la historia,
como un tesoro con nombre propio. No representa un inventario de armas comunes,
munición o dinero. Equipamiento y efectos de equipo quedan para otro desarrollo.

## Ubicación y cautiverio

| Relación | Origen | Destino | Significado |
|---|---|---|---|
| GuardadoEn | Actor u Objeto | Lugar o Colectivo | Ubicación física. |
| LlevadoPor | Actor u Objeto | Actor | Acompaña físicamente al portador. |
| PrisioneroDe | Actor | Actor | Cautiverio independiente de la ubicación. |

Cada elemento tiene como máximo una relación espacial. Todo objeto debe tener
una ubicación resoluble; las cadenas de portadores no pueden formar ciclos.
Un preso guardado en una torre permanece allí aunque su captor viaje. Un objeto
llevado por una persona sigue su ubicación efectiva, incluso dentro de un colectivo.
Estas relaciones no declaran propiedad legal.

Los actores antiguos conservan compatibilidad con `ubicacion_inicial`. Si existe
una relación espacial, esta tiene prioridad. Puede omitirse ese campo al declarar
la ubicación mediante relaciones; la carga deriva la posición inicial. Tamin y
Farid usan GuardadoEn, manteniendo separado su vínculo de cautiverio.

`Mundo` mantiene objetos con `IdObjeto`, extremos tipados y un índice espacial.
Las consultas `ubicacion_elemento`, `ubicacion_actor` y `ubicacion_objeto`
resuelven la misma cadena. Las fichas presentan objetos y permiten navegar sus
relaciones, incluidos el portador y los personajes que los desean.

## Autoría

`world.ron` incluye una lista opcional `objetos` de claves. Cada clave corresponde
a un archivo `objects/<clave>.ron`, por ejemplo:

```ron
(
    clave: "tesoro_ejemplo",
    nombre: "Tesoro de ejemplo",
    descripcion: "Una pieza reconocible por su historia.",
    caracteristicas: ["tesoro"],
    relaciones: [
        (tipo: GuardadoEn, destino: Lugar("lomas_de_anir")),
    ],
)
```

Para llevarlo se usa `LlevadoPor` con `Actor("clave")`. Las relaciones personales
Desea, Debe, Necesita y EsLlamadoA admiten destinos Objeto además de Concepto.
Las referencias y características se validan al cargar el contenido.

En los efectos Lua, `op.situar_elemento("Objeto", "clave_objeto", "Actor",
"clave_actor")` crea una operación para el lote; no modifica el mundo por sí sola.
Para dejarlo en tierra, el destino es `"Lugar"`. La consulta
`mundo:ubicacion_objeto("clave_objeto")` devuelve la clave del lugar efectivo.
Los filtros de eventos y ctx.objetivo admiten objetivos de tipo Objeto.

## Operaciones e invariantes

SituarElemento reemplaza la relación espacial conservando su ID cuando existe.
El lote se valida sobre un candidato antes de confirmar cambios e índices; una
referencia inválida, ubicación ambigua o ciclo rechaza el lote sin efectos parciales.
Trasladar, colectivos y rescate mantienen coherentes las ubicaciones compartidas.
La reubicación del grupo jugador conserva sus operaciones específicas; SituarElemento
no sirve para saltarse sus invariantes moviendo a un integrante por separado.

## Recuperación en encuentros

`encounters.ron` admite `recuperaciones`: objeto, custodio, defensores y escenario.
La oposición se declara explícitamente; no se vuelve hostil a cualquier portador
por el hecho de llevar un objeto. El grupo libre puede elegir Robar con R cuando
el tesoro configurado está en su lugar actual y no lo lleva ya consigo.

El motor reutiliza movimiento, combate y salida del rescate. El botín es una
pieza local independiente, nunca un actor ficticio. Se recoge por contacto y se
representa con $. El portador real debe alcanzar la salida. Si cae antes, lo deja
en su casilla y otro integrante puede recogerlo. Una retirada sin extracción
conserva el objeto en su ubicación anterior. Al extraerlo se confirma LlevadoPor
hacia el líder del grupo (actor jugador), aunque otro integrante lo haya sacado, sin duplicar la instancia. Las consecuencias normales del
encuentro permanecen vigentes.

La representación y las posiciones del tablero son temporales. La ubicación
persistente cambia al confirmar la extracción. Las operaciones narrativas se
validan contra ese estado para evitar cambios contradictorios durante el episodio.

## Caso jugable y verificación

La Copa de las Dos Mareas está en el palacio de Miraz en Nuris.
Neshra la desea. Llegar con el grupo libre, pulsar R y elegir la copa permite
combatir, recogerla y escapar. Miraz es el único defensor configurado.
Rescatar a Tamin en Lomas de Anir sigue siendo una operación independiente con R.

Pruebas acotadas: ubicaciones comunes, portadores y colectivos, separación del
captor, ciclos y atomicidad; recogida y caída del portador; extracción y retirada
en la aplicación; selector y representación mediante TestBackend. La aceptación
de ritmo y dificultad corresponde a la prueba manual.

El selector R reúne personas y objetos: se anuncia como Rescatar, Robar o
Recuperar según haya personas, objetos o ambos, respectivamente. Elegir una
persona inicia el rescate; elegir un objeto inicia su recuperación.
