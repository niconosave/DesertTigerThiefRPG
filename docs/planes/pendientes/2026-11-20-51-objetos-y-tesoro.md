# Plan: Objetos y recuperación de un tesoro

ID: PLAN-2026-051. Fecha: 2026-11-20.
Estado: Implementado; pendiente revisión de jugabilidad.

## Diseño aplicable

[Objetos y ubicaciones compartidas](../../tecnica/componentes/30-objetos-y-ubicaciones-compartidas.md)
y [mecánicas de encuentros, sección 23](../../diseno/encuentros-y-atacar.md#23-objetos-particulares-y-recuperación-de-tesoros).

## Incremento entregado

- [x] Identidad, RON, relaciones y fichas para objetos.
- [x] Ubicación común GuardadoEn/LlevadoPor, independiente de PrisioneroDe.
- [x] Validación de referencias, ubicación única, ciclos y efectos atómicos.
- [x] Consultas y operación de ubicación desde Lua.
- [x] Recuperación local con portador, caída, recogida y salida.
- [x] Copa de las Dos Mareas en el palacio de Miraz; deseo de Neshra.
- [x] Selector B y pruebas acotadas de motor, aplicación y terminal.
- [ ] Revisión manual por el usuario.

## Prueba manual

Llegar libre a Nuris, pulsar B y elegir la copa. Observar combate,
recogida y retirada hacia la salida. Consultar después la ficha del líder del grupo:
Lleva consigo debe enlazar al objeto. Viajar y comprobar que lo acompaña.
Retirarse antes de recogerla debe dejarla allí. R sigue ofreciendo a Tamin.

No incluye equipamiento, comercio de objetos, inventario genérico ni IA nueva.
