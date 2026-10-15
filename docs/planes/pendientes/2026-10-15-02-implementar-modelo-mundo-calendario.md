# Plan: Implementar el Mundo y el calendario

| Campo | Valor |
|---|---|
| ID | PLAN-2026-002 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

`desert-tiger-core` puede construir y consultar un Mundo válido, trasladar
Actores de forma atómica y calcular o aplicar duraciones gregorianas mediante
APIs probadas y sin filesystem.

## Referencias

- [Identificadores internos](../../tecnica/componentes/02-modelo-mundo.md#3-identificadores-internos).
- [Elementos reglados](../../tecnica/componentes/02-modelo-mundo.md#4-elementos-reglados).
- [Construcción, consultas y operaciones](../../tecnica/componentes/02-modelo-mundo.md#6-construcción-consultas-y-operaciones).
- [Errores y validaciones del Mundo](../../tecnica/componentes/02-modelo-mundo.md#8-errores-y-validaciones).
- [Modelo de calendario](../../tecnica/componentes/06-calendario-duracion.md#3-modelo-confirmado).
- [API de calendario](../../tecnica/componentes/06-calendario-duracion.md#4-api-requerida).
- Plan previo: [PLAN-2026-001](./2026-10-15-01-preparar-workspace.md).

## Alcance

### Incluido

- IDs opacos, Características y variables globales extensibles;
- tipos iniciales y vivos de Lugar, Actor, Grupo y Mundo;
- construcción validada, consultas y vínculo opaco con el mapa;
- traslado atómico del jugador con su Grupo o de un NPC independiente;
- `Mes`, `Fecha`, `Duracion` y avance atómico de la fecha;
- errores, invariantes y pruebas unitarias del núcleo.

### No incluido

- motor de acciones o reglas de `Viajar`;
- lectura de RON, Lua, estado visual del mapa o frontend;
- Relaciones, facciones o mutación de variables globales durante la Partida.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-001](./2026-10-15-01-preparar-workspace.md) están resueltos y
  registrados; el plan permanece pendiente hasta integrar el incremento.

## Pasos

1. [ ] Crear los módulos de modelo, calendario y azar necesarios dentro de
       `desert-tiger-core`, manteniendo privados los campos reglados.
2. [ ] Implementar IDs canónicos, Característica, claves y valores de variables
       globales junto con sus constructores y errores.
3. [ ] Implementar los datos iniciales, tipos vivos y `Mundo::nuevo` con el orden
       de validación documentado.
4. [ ] Implementar las consultas de Mundo, Actor, Lugar y Grupo sin exponer
       colecciones mutables ni duplicar ubicaciones.
5. [ ] Implementar `Mundo::trasladar` con validación completa antes de la primera
       escritura y con `TrasladoConfirmado` ordenado.
6. [ ] Implementar los tipos y operaciones de calendario, suma de duración y
       `Mundo::avanzar_fecha` con actualización atómica.
7. [ ] Añadir pruebas unitarias para constructores, precedencia de errores,
       consultas, orden, límites numéricos, traslados y calendario.
8. [ ] Ejecutar los controles del crate y de la workspace; registrar el resultado
       y actualizar los documentos solo si la implementación exige corregirlos.

## Verificación

- [ ] Se cumplen todos los casos de
      [pruebas del Mundo](../../tecnica/componentes/02-modelo-mundo.md#9-pruebas-y-aceptación).
- [ ] Se cumplen todos los casos de
      [pruebas del calendario](../../tecnica/componentes/06-calendario-duracion.md#6-pruebas-y-aceptación).
- [ ] Un rechazo de construcción, traslado o avance temporal conserva el estado
      anterior y no requiere `panic`, `unwrap` ni rollback.
- [ ] Las pruebas de `desert-tiger-core` se ejecutan sin filesystem, Lua ni
      terminal y su árbol continúa sin dependencias externas aprobadas para app.
- [ ] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- El orden de validaciones es observable en pruebas; se implementará desde el
  contrato en vez de depender del orden accidental de colecciones.
- La privacidad de campos de `Mundo` requiere ubicar sus implementaciones en el
  módulo o descendientes autorizados por Rust.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: Pendiente.
- Verificaciones ejecutadas: Pendiente.
- Desviaciones respecto del plan: Pendiente.
- Trabajo posterior: [PLAN-2026-003](./2026-10-15-03-implementar-motor-viajar.md).
