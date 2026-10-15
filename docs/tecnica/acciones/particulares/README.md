# Diseños técnicos de acciones particulares

Esta carpeta contiene la fuente normativa de cada operación de simulación con
reglas propias que pueda ser invocada desde más de una acción general. El
[motor común](../../componentes/05-motor-acciones.md) define cómo se ejecutan y
las especificaciones de acciones generales definen cuándo y con qué contexto se
invocan.

Cada documento deberá fijar, sin repetir el motor:

- entradas regladas y precondiciones;
- cálculo y uso de azar, si corresponde;
- consecuencias atómicas sobre `Mundo`;
- fragmentos narrativos o resultados semánticos que produce;
- duración que aporta a la acción general;
- rechazos, invariantes y pruebas.

El primer incremento define `RecorrerTramo`, `ResolverObstaculo` y
`ResolverRumbo`, pero por ahora las tres son operaciones privadas usadas
únicamente por `Viajar`; su fuente normativa permanece en
[`../01-viajar.md`](../01-viajar.md) para no duplicar reglas. Una operación se
extraerá a un documento de esta carpeta cuando una segunda acción general vaya a
reutilizarla o cuando adquiera un contrato público independiente. La extracción
deberá mover, y no copiar, su definición normativa.
