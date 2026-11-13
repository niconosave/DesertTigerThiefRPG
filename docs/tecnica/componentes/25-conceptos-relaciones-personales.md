# Conceptos y relaciones personales

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.0 |
| Fecha | 2026-11-11 |

## 1. Alcance

Materializa los [Conceptos y relaciones personales](../../contenido/borradores/conceptos-relaciones-y-finales.md)
y el [reparto de los dos puertos](../../contenido/borradores/personajes-zarabad-nuris.md).
Amplía el modelo, la carga RON, las fichas de Acampar y las consultas de eventos
del contrato 23. No implementa finales ni acciones de captura, reclutamiento
o rescate. Los estados narrativos no ejecutan por sí solos esas mecánicas.

## 2. Contenido editable

`world.ron` admite una lista opcional `conceptos`. Como con Actores y Lugares,
sus claves identifican los archivos que forman parte de la partida:

```ron
conceptos: ["recuperar_la_libertad"],
```

Cada archivo `concepts/<clave>.ron` contiene exclusivamente:

```ron
(
    clave: "recuperar_la_libertad",
    nombre: "Recuperar la libertad",
    descripcion: "Dejar de estar retenido y poder decidir el propio rumbo.",
)
```

No declarar la lista o declararla vacía conserva los contenidos sin Conceptos.
No se descubren archivos al margen del manifiesto. Se validan claves, duplicados,
correspondencia de clave y archivo, texto presentable y rutas dentro de la raíz,
con los diagnósticos y controles existentes.

Las relaciones se escriben en el Actor o Lugar de origen con la sintaxis habitual:

```ron
relaciones: [
    (tipo: Necesita, destino: Concepto("recuperar_la_libertad")),
    (tipo: PrisioneroDe, destino: Actor("dahran")),
],
```

El uso en un Lugar de un tipo que exige Actor se rechaza. Las nuevas relaciones
no admiten el campo `mercancia`, reservado a Vende y Compra.

## 3. Tipos y validación

| Tipos | Extremos admitidos | Sentido |
|---|---|---|
| Desea, Debe, Necesita, EsLlamadoA | Actor → Concepto | Dirigido. |
| Gobierna, ComerciaCon | Actor → Lugar | Dirigido; no habilita tránsito. |
| PrisioneroDe, SalvadoPor | Actor → Actor | Dirigido, con lectura inversa en fichas. |
| EnemigoDe, PactoCon, TratosCon | Actor ↔ Actor | Simétrico; se declara una vez. |

Se conservan los tipos previos. `TipoRelacion::admite_extremos` centraliza las
restricciones usadas por dominio, cargador y consultas Lua. `es_simetrica`
identifica las relaciones cuyo par inverso es el mismo hecho, incluido Conectado.
Se rechazan referencias desconocidas, relaciones reflexivas y duplicados, también
si una relación simétrica se declara desde el otro extremo.

Un PactoCon no introduce reglas de secreto. Su carácter secreto pertenece a la
biografía o al episodio; la ficha muestra el vínculo registrado.

## 4. Núcleo e índices

`IdConcepto` es distinto de IdActor e IdLugar. `ExtremoRelacion` incorpora la
variante `Concepto(IdConcepto)`. El núcleo solo registra identidades e índices:
el nombre y la descripción pertenecen al catálogo de presentación.

`Mundo::nuevo_con_conceptos(datos, cantidad)` registra los IDs canónicos de los
Conceptos declarados y valida los extremos. `Mundo::nuevo(datos)` conserva su
API y equivale a construir con cero Conceptos. La aplicación pasa el tamaño
del catálogo al validar e instanciar cada partida.

`relaciones_de` conserva índices separados por Actor, Lugar y Concepto. La
consulta obtiene solo los vínculos del elemento, sin recorrer todo el grafo.
Los nuevos tipos no modifican los índices de conexión ni los conocimientos de
lugares: únicamente Conectado y Conoce conservan esas funciones particulares.

Los Conceptos no tienen posición, características, progreso ni comportamiento.
Compartir uno no comparte estados de cumplimiento entre Actores. No se añaden
operaciones para completar objetivos o elegir finales automáticamente.

## 5. Fichas y consultas Lua

Las fichas muestran etiquetas directas e inversas de los nuevos tipos. Enter
permite abrir una referencia conceptual y consultar su descripción y relaciones
entrantes. La terminal identifica el elemento como Concepto, sin ubicación ni
destino geográfico. Se reutilizan los controles de navegación existentes.

Las condiciones y presentaciones de eventos admiten consultas como:

```lua
return mundo:hay_relacion(
    "Necesita",
    {tipo = "Actor", clave = "farid"},
    {tipo = "Concepto", clave = "recuperar_la_libertad"}
)
```

Los tipos simétricos se consultan desde cualquiera de sus extremos. Se validan
claves y tipos antes de consultar. `ctx:objetivo()` puede representar la nueva
variante si un futuro punto proporciona un Concepto. No se crea por ello una
acción que lo ejecute.

## 6. Contenido incluido y comprobaciones

Se incorporan Zahrim, Dahran, Sahmun, Rima, Miraz, Zulema, Bahir, Farid, Samir
y Neshra, además de diecisiete Conceptos. Dahran es el capitán masculino elegido
para este reparto. Se adaptan las relaciones de Bandido y se
declaran el deseo del príncipe y el deber del sirviente. Los nuevos rasgos tienen
descripciones en el glosario existente.

Los NPCs nuevos no tienen políticas ni retratos. Sus ubicaciones y relaciones
son iniciales; el inicio de la partida continúa en Hamada Roja. La venta del
príncipe y los encargos navales requieren eventos posteriores.

Las pruebas cubren índices conceptuales, extremos inválidos, duplicados
simétricos, manifiestos sin Conceptos, navegación desde el grupo hasta un
Concepto compartido, filiación de Farid y Samir y consultas Lua de los nuevos
tipos. Se conserva la batería de viajes, calendario, campamento y presentación.
