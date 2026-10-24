# Diseños técnicos de acciones particulares

Esta carpeta contiene la fuente normativa de cada operación de simulación con
reglas propias que pueda ser invocada desde más de una acción general. El
[motor común](../../componentes/05-motor-acciones.md) define cómo se ejecutan y
las especificaciones de acciones generales definen cuándo y con qué contexto se
invocan.

Cada documento fija, sin repetir el motor:

- entradas regladas y precondiciones;
- cálculo y uso de azar, si corresponde;
- consecuencias atómicas sobre `Mundo`;
- traza causal y resultados semánticos que produce;
- duración que aporta a la acción general;
- rechazos, invariantes y pruebas.

| Documento | Fuente normativa de |
|---|---|
| [`00-contrato-resolucion.md`](./00-contrato-resolucion.md) | Ciclo evaluar–resolver, dificultad contextual, d20, ayuda, pronóstico y resultados. |
| [`01-movimiento-terrestre.md`](./01-movimiento-terrestre.md) | `Orientarse`, `Marchar`, `Escalar`, `Descender`, `Cabalgar` y `GuiarCaravana`. |
| [`02-supervivencia-campamento.md`](./02-supervivencia-campamento.md) | Agua, sustento, campamento y descanso. |
| [`03-navegacion.md`](./03-navegacion.md) | Remo, navegación, timón, fondeo, atraque y achique. |

El catálogo conceptual de verbos permanece dentro de la sección de referencia
del contrato común. Solo una acción con reglas aprobadas obtiene una ficha
normativa; enumerar una acción futura no la incorpora al juego.
