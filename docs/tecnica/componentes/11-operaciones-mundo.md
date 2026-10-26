# Operaciones validadas del Mundo

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.2 |
| Fecha | 2026-10-26 |

> Ampliación implementada el 2026-10-26. Ubicación local y rechazo de traslados externos: [contrato 14, secciones 3 y 7](./14-parentesco-actores-contexto.md#3-ubicación-del-actor).

## Revisión implementada el 2026-10-24: Separación y conocimiento atómicos

OperacionMundo y CambioConfirmado incluyen SepararCompanero y RegistrarConocimiento. aplicar_lote prepara Actores, Grupo, Relaciones e índices, y publica todo junto. La tarjeta omite Trasladar cuando no cambia la ubicación; la primitiva conserva su validación anterior.

La fuente vigente para estos cambios es el
[contrato implementado de Viajar](../acciones/06-implementacion-viajar.md).
Los apartados del cuerpo original incompatibles con esta revisión se conservan
como historial, no como especificación alternativa. Los demás contratos del
componente mantienen su alcance. Las verificaciones están en el
[informe del refactor](../acciones/verificacion-viajar-reglamento.md).

## 1. Propósito y alcance

Este documento define el único mecanismo por el que una acción aplica un lote de
consecuencias persistentes al `Mundo`. Amplía la operación de traslado existente
para que las acciones particulares puedan añadir o retirar Características sin
obtener acceso mutable directo a Actores o Lugares.

El alcance incluye:

- traslado del actor dirigente y, cuando corresponda, de su Grupo inseparable;
- alta y baja de Características de Actor;
- validación completa antes de modificar;
- aplicación atómica de un lote;
- resultados semánticos de cambios ya confirmados.

No incluye inventarios, Objetos, modificación de Relaciones, altas o bajas de
Actores y Lugares, cambios de miembros del Grupo ni edición del Mapa. Esas
capacidades requerirán operaciones nuevas y documentación propia.

## 2. Responsabilidades

Las acciones particulares y generales pueden construir solicitudes de operación,
pero no mutan directamente los campos privados del modelo. `Mundo` valida los
IDs, tipos, precondiciones e invariantes. El motor aplica un lote solamente
después de que todas sus operaciones sean válidas en la secuencia solicitada.

La aplicación y el frontend reciben resultados semánticos de operaciones ya
confirmadas. Nunca reciben órdenes para volver a mutar el Mundo.

## 3. Vocabulario y API objetivo

El núcleo incorporará el equivalente a:

```rust
pub enum OperacionMundo {
    Trasladar {
        actor: IdActor,
        destino: IdLugar,
    },
    AnadirCaracteristicaActor {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
    QuitarCaracteristicaActor {
        actor: IdActor,
        caracteristica: Caracteristica,
    },
}

pub struct LoteOperaciones {
    operaciones: Vec<OperacionMundo>,
}

impl Mundo {
    pub fn aplicar_lote(
        &mut self,
        lote: LoteOperaciones,
    ) -> Result<CambiosConfirmados, ErrorOperacionMundo>;
}
```

Los nombres exactos pueden ajustarse al implementar si conservan estas
responsabilidades y no amplían la API pública. `LoteOperaciones` no admite un
lote vacío. Su orden es significativo para validar transiciones, pero el
resultado final debe preservar todas las invariantes del Mundo.

La API implementada añade `LoteOperaciones::nuevo`, que rechaza un vector vacío,
`LoteOperaciones::uno`, `CambiosConfirmados::como_slice` y
`CambiosConfirmados::en_vec`. `Mundo::trasladar` permanece como fachada
compatible y delega en un lote de una operación; por tanto no conserva otra
fórmula de traslado.

## 4. Semántica de las operaciones

### 4.1. `Trasladar`

Conserva el contrato actual de `Mundo::trasladar`:

- valida Actor y Lugar;
- rechaza trasladar al Lugar actual;
- si el Actor es el jugador, traslada al jugador y a todos sus compañeros;
- rechaza trasladar por separado a un compañero;
- un NPC que no pertenece al Grupo se traslada solo.

Cada avance confirmado entre Lugares durante `Viajar` solicita una operación de
traslado. De este modo un camino, paso o vado creado como Lugar es una ubicación
real y puede convertirse en el último Lugar alcanzado si la Expedición se
interrumpe. No existe una variante especial de Lugar para esos casos.

### 4.2. `AnadirCaracteristicaActor`

Valida que el Actor exista y que la Característica tenga la sintaxis ya aprobada.
Si el Actor todavía no posee la Característica, la añade y registra un cambio
efectivo. Si ya la posee, la operación es idempotente: no duplica datos y se
registra como satisfecha sin otro cambio observable.

Ejemplos de uso en esta ampliación son `bienAprovisionado`, `fatigado`,
`agotado`, `sediento`, `hambriento`, `herido` y `muerto`.

### 4.3. `QuitarCaracteristicaActor`

Valida que el Actor exista y la sintaxis de la Característica. Si está presente,
la retira. Si está ausente, la operación es idempotente y no inventa un error de
gameplay.

Quitar `muerto` no forma parte de ninguna regla aprobada. El hecho de que la
operación genérica pueda representarlo no autoriza a una acción a solicitarlo.

## 5. Validación y atomicidad

`aplicar_lote` ejecuta conceptualmente dos fases:

1. valida en un estado virtual todas las operaciones en su orden, incluidas las
   interacciones entre altas y bajas del mismo lote;
2. si no existe ningún error, confirma el estado final y construye sus resultados
   semánticos.

Si cualquier operación falla, ninguna modificación del lote alcanza el Mundo y
no se emite ningún resultado. La implementación puede usar un registro de deltas,
copias acotadas o una estrategia equivalente; no expondrá estados intermedios ni
requerirá clonar recursos visuales o contenido inmutable.

La implementación inicial clona solamente el vector reglado de Actores, valida y
aplica allí la secuencia completa y sustituye el vector del Mundo únicamente al
terminar con éxito. No clona el Mapa, los Lugares, las Relaciones ni los
catálogos de presentación.

Las solicitudes contradictorias sobre la misma Característica se evalúan en
orden. Añadir y después quitar produce ausencia; quitar y después añadir produce
presencia. Una acción debe evitar lotes confusos, pero el motor conserva una
semántica determinista.

Un lote puede trasladar al Grupo y modificar Características de sus integrantes.
Todas las identidades se validan antes de confirmar. La atomicidad se limita al
lote: no promete rollback de lotes confirmados en momentos anteriores de una
acción general.

## 6. Resultados semánticos

`CambiosConfirmados` conserva orden y contiene solamente resultados de cambios
efectivos:

```text
CambioConfirmado
├── ActoresTrasladados { actores, origen, destino }
├── CaracteristicaActorAnadida { actor, caracteristica }
└── CaracteristicaActorQuitada { actor, caracteristica }
```

Una operación idempotente satisfecha no añade un resultado duplicado. El motor
puede utilizar estos cambios para producir fragmentos narrativos; la aplicación
los usa para reconstruir su vista, no para aplicarlos otra vez.

## 7. Estados progresivos de viaje

Las reglas de `Viajar` usarán operaciones sencillas para expresar estados
persistentes. La progresión normativa es:

```text
sin desgaste → fatigado → agotado → herido → muerto
```

`sediento` y `hambriento` son causas paralelas, no escalones sustitutos. Una
crisis repetida hace avanzar un escalón de desgaste; obtener agua o alimento
puede retirar su causa. `Descansar` puede retirar o reducir desgaste conforme a
su propia ficha, pero no cura `herido`. Las acciones particulares solicitan los
cambios exactos; la operación genérica no deduce la progresión.

La cadena anterior gobierna desgaste y carencias acumuladas. Un accidente
físico concreto —por ejemplo una caída reglada por `Escalar`— puede añadir
`herido` directamente a un Actor sano cuando su ficha lo establece. Aun en ese
caso, un único fallo rutinario no añade `muerto`.

Un fallo rutinario no añade `muerto` directamente. La muerte durante `Viajar`
solo puede ocurrir al agravar un Actor ya `herido` mediante otra crisis o al
aceptar una opción expresamente presentada como letal. Un desastre marítimo debe
haber sido precedido por contexto y decisiones comprensibles; no es una única
extracción sorpresiva.

## 8. Errores

Las categorías mínimas son:

- lote vacío al construir `LoteOperaciones`;
- Actor desconocido;
- Lugar desconocido;
- traslado al Lugar actual;
- compañero trasladado por separado;
- estado del Grupo inconsistente;
- invariante interna rota al traducir un resultado que contradiga una operación
  ya validada.

No existe un error de sintaxis durante `aplicar_lote`: una operación recibe una
`Caracteristica` ya construida y, por lo tanto, validada por su propio tipo.

Un error esperado de precondición se devuelve al motor para que este lo clasifique
como rechazo o fallo según el punto de confirmación. Una invariante rota después
de haber comenzado una acción aceptada es fatal para la sesión; nunca se presenta
como mala suerte narrativa.

## 9. Invariantes

- Actor y compañeros permanecen siempre en el mismo Lugar.
- Ningún conjunto contiene dos veces la misma Característica.
- Un lote fallido deja el Mundo byte a byte equivalente en sus datos reglados.
- Los resultados semánticos describen exclusivamente cambios ya confirmados.
- Aplicación y frontend no vuelven a aplicar los cambios.
- Lua no obtiene acceso directo a `aplicar_lote`; solo solicita una acción
  validada.
- El Mapa visual y sus tiles no participan en la validación.

## 10. Pruebas y criterios de aceptación

Se verificarán:

- traslado de jugador y Grupo y traslado individual de NPC;
- rechazo de compañero separado y destino actual;
- alta, baja e idempotencia de Características;
- orden determinista de dos cambios sobre la misma Característica;
- fallo de la última operación sin confirmar ninguna anterior;
- lote que traslada y modifica varios Actores de forma atómica;
- resultados semánticos exactos y ausencia de resultados para no-op;
- progresiones solicitadas por pruebas de acciones, incluida la imposibilidad de
  muerte rutinaria directa;
- integración donde una Expedición confirma varios lotes por Lugar y termina en
  el último Lugar real alcanzado.

## 11. Decisiones aplazadas

Se aplazan operaciones sobre Lugares, Relaciones, variables globales, Objetos,
Grupo y Mapa; procedencia persistente de una Característica; deshacer operaciones;
transacciones entre acciones generales y un registro público de auditoría.
