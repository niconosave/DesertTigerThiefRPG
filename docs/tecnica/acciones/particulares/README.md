# Acciones particulares

La fuente vigente es la [sección 8 del contrato de Viajar](../06-implementacion-viajar.md#8-contrato-de-acciones-particulares-y-azar)
y la [sección 6 del reglamento](../borradores/05-viajar-redisenio.md#chequeos).
El catálogo implementado contiene `Andar`, `Cabalgar`, `NavegarEnBarco`,
`CalcularProvisiones` y `Forajear`. Caballo y camello comparten `Cabalgar`.

El código común recibe Actor, acción y dificultad, aplica modificadores y
resuelve un d20. El contexto que llama elige al responsable y decide duración,
incidentes, consecuencias y presentación. Las tarjetas no son habilidades.

Los documentos `00` a `03` se conservan como antecedentes obsoletos. Sus verbos,
bandas, pronósticos y trazas no forman parte del motor actual. No se reutilizan
para completar mecánicas que el reglamento nuevo no especificó.
