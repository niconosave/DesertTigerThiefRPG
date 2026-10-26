# Plan: Incorporar parentesco y Actores de contexto

| Campo | Valor |
|---|---|
| ID | PLAN-2026-038 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

Desde Acampar se puede abrir la familia del príncipe, consultar al rey fuera
del escenario y descubrir tanto a su asesino como su parentesco de hermanos.
Los vínculos declarados y las deducciones se presentan con su significado y
explicación, mientras Viajar y el campamento conservan su funcionamiento.

## Referencias

- [Alcance y fuente de verdad](../../tecnica/componentes/14-parentesco-actores-contexto.md#1-alcance-y-fuente-de-verdad).
- [Archivos y responsabilidades](../../tecnica/componentes/14-parentesco-actores-contexto.md#2-responsabilidades-y-archivos).
- [Ubicación de Actor y operaciones](../../tecnica/componentes/14-parentesco-actores-contexto.md#3-ubicación-del-actor).
- [Exclusión de NPC y Lua](../../tecnica/componentes/14-parentesco-actores-contexto.md#33-npc-y-lua).
- [Esquema RON y compatibilidad](../../tecnica/componentes/14-parentesco-actores-contexto.md#4-autoría-ron-y-compatibilidad).
- [Índices y consultas familiares](../../tecnica/componentes/14-parentesco-actores-contexto.md#5-almacenamiento-índices-y-consultas-familiares).
- [Fichas e inversas](../../tecnica/componentes/14-parentesco-actores-contexto.md#6-fichas-ubicación-inversas-y-deducciones).
- [Errores](../../tecnica/componentes/14-parentesco-actores-contexto.md#7-errores-y-confirmación-de-estado).
- [Regresión de acciones y presentación](../../tecnica/componentes/14-parentesco-actores-contexto.md#8-compatibilidad-de-acciones-y-presentación).
- [Contenido de integración](../../tecnica/componentes/14-parentesco-actores-contexto.md#9-contenido-de-integración).
- [Verificación y aceptación](../../tecnica/componentes/14-parentesco-actores-contexto.md#10-verificación-y-aceptación).
- [Contratos afectados](../../tecnica/componentes/14-parentesco-actores-contexto.md#11-contratos-afectados-y-límites).
- [Controles generales](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- Ubicación explícita local o fuera del escenario y adaptación de sus consumidores.
- Relaciones Padre, Madre y MatadoPor, con validación e índices.
- Hermandad calculada y su explicación en la ficha.
- Migración del contenido y familia de integración del contrato.
- Pruebas de núcleo, aplicación, contenido, frontend y recorrido completo.

### No incluido

- Reinos, títulos reglados, sucesión o características nuevas de rey/exiliado.
- Otros parentescos derivados, adopción, matrimonio o consultas configurables.
- Alta o baja de parentesco en partida, entrada de Actores desde fuera del escenario.
- Acciones de asesinato, cambios de combate, guardado, retratos nuevos o cambios del mapa.

## Precondiciones

- Contrato 14 aprobado para implementar y conceptos incorporados al diseño general.
- Base actual de Acampar y Viajar funcional en la rama de trabajo del usuario.
- Contenido y ediciones artísticas existentes identificados para conservarlos.

## Pasos

1. [x] Comprobar el estado inicial, controles y carga del contenido; registrar
       fallos previos y conservar cambios ajenos al alcance.
2. [x] Implementar el tipo de ubicación, adaptar Actor/ActorInicial y consultas
       del Mundo, y cubrir las invariantes de jugador y Grupo de la sección 3.
3. [x] Migrar el DTO de ubicación, los seis RON existentes y los constructores
       de fixtures junto al cargador, conservando el contenido previo.
4. [x] Adaptar traslados, separación y sus errores al nuevo tipo; comprobar
       que los lotes fallidos conservan el estado y los traslados locales funcionan.
5. [x] Adaptar Viajar, Acampar, disponibilidad, Lua y fase NPC a la sección 3;
       verificar rechazo local y omisión de políticas externas sin consumir azar.
6. [x] Añadir los tres tipos registrados a DTO/core, normalización e índices
       incidentes, preservando los de Conectado y Conoce.
7. [x] Implementar cardinalidad, ascendencia acíclica y diagnósticos según las
       secciones 4 y 7, incluida una genealogía profunda sin recursión.
8. [x] Implementar y probar los índices y consultas familiares de la sección
       5; cubrir ausencia de datos, progenitores compartidos, orden y deduplicación.
9. [x] Incorporar ubicación y vínculos diferenciados a la proyección de fichas
       y sus reexportaciones. Actualizar exhaustivamente los consumidores de
       los tipos públicos modificados, sin IDs de reserva ni ramas silenciosas.
10. [x] Integrar ubicación, etiquetas inversas y explicación de hermandad en
        el detalle paginado de campamento. Conservar controles, focos y retratos.
11. [x] Crear los cinco Actores de contexto y sus datos de integración de la
        sección 9, agregar sus claves al final de world.ron y enlazar al
        príncipe. Mantener el exilio exclusivamente en la biografía.
12. [x] Añadir pruebas de que RegistrarConocimiento conserva los índices
        familiares y de que consultar fichas o hermanos no modifica el Mundo,
        RNG, fecha ni las relaciones registradas.
13. [x] Cubrir con TestBackend las dos entradas distintas hacia el hermano,
        sus explicaciones, navegación externa/cíclica y regreso al Grupo.
14. [x] Ejecutar el humo desde archivos reales y las regresiones de Viajar,
        cierre de Acampar, resto temporal y fase NPC; validar contenido y Lua
        oficiales y ejecutar los controles generales y build de producción.
15. [x] Realizar una prueba manual agrupada del recorrido de la sección 10;
        comprobar que la historia y la ubicación externa se entienden en las
        fichas, sin detenerse a solicitar pruebas de cada variante pequeña.
16. [x] Actualizar el contrato y componentes de la sección 11 con el resultado
        implementado; registrar verificaciones y desviaciones. Al cumplir la
        definición de terminado, mover este mismo archivo a completados y
        actualizar los índices y enlaces.

## Verificación

- [x] La matriz de aceptación de la sección 10 está cubierta por pruebas y
      por el recorrido manual que corresponda.
- [x] El contenido oficial carga los once Actores sin alterar sus IDs previos,
      mapa, Grupo ni dibujos; las referencias resuelven a elementos existentes.
- [x] Actores externos son consultables y no ocupan ningún Lugar ni ejecutan
      acciones locales, incluso si están vivos y tienen una política Lua.
- [x] La validación rechaza filiaciones estructuralmente inválidas, admite
      genealogías incompletas y no confunde ciclos de ascendencia con otros ciclos.
- [x] Los hermanos se consultan por índices sin materializar todas las parejas
      ni introducir relaciones guardadas adicionales.
- [x] Las fichas muestran ubicación, parentesco y asesinato en ambos sentidos;
      MatadoPor y Hermandad al mismo destino se conservan como entradas distintas.
- [x] Campamento y Viajar mantienen navegación, retratos, tiempo y fase NPC.
- [x] Pasan los cuatro controles de la metodología y
      `cargo build --workspace --release`; todo el contenido y Lua oficiales validan.
- [x] Se registra la prueba manual final y los contratos reflejan el estado real.
- [x] Se cumplen los controles y la definición de terminado de la
      [metodología](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Cambiar ubicación afecta constructores y consumidores de varias capas. El
  cargador y los RON se migran como un solo lote; no se entrega una base que
  rechace su propio contenido.
- El cambio de identidad de vínculos de ficha se integra con el frontend en
  este mismo incremento. Los parentescos calculados no se adaptan mediante IDs falsos.
- Las validaciones nuevas no deben invalidar conexiones geográficas ni cambiar
  resultados del viaje. Las regresiones acompañan a las modificaciones compartidas.
- Todo el plan produce un incremento jugable; la revisión manual se concentra
  al final y no se amplía el alcance para completar el árbol familiar sin límite.
- Si aparece una decisión nueva, se resuelve en el contrato canónico antes
  de modificar el plan. Este archivo solo ordena el trabajo aprobado.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26.
- Resultado: ubicación explícita de Actores, Padre/Madre y MatadoPor,
  validación de ascendencia e índices de consulta; hermanos deducidos con
  evidencia y fichas neutrales que distinguen vínculos registrados de derivados.
  Los once Actores oficiales incluyen los cinco de contexto. El exilio del
  príncipe permanece solo en la biografía; no se añadieron reglas de sucesión.
- Verificaciones ejecutadas: pasan `cargo fmt --all -- --check`,
  `cargo check --workspace --all-targets`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
  La base tenía 96 pruebas; ahora pasan 107 (46 core, 31 app, 26 terminal,
  4 importador), con carga de contenido y Lua oficiales y regresión de Viajar
  y Acampar. Incluye una genealogía de 20.000 Actores y políticas externas
  vivas que fallarían si se ejecutaran.
- Revisión manual: ejecutable release en PTY 80×24. Desde el príncipe se abrió
  al rey, se consultaron MatadoPor y Hermandad con su explicación, se recorrió
  rey ↔ hermano y se volvió al grupo con una flecha. Se abrió también a la
  madre; al abandonar se recuperó el mundo y se abrió el selector de Viajar.
  El programa restauró la terminal y salió con código cero. Arte, expresiones,
  paletas y políticas de los seis Actores anteriores se compararon con Git y
  permanecen idénticos.
- Desviaciones respecto del plan: ninguna funcional. Se actualizaron las
  expectativas de fixtures y focos que suponían seis Actores o una ficha vacía
  del príncipe; ahora reflejan la familia de integración aprobada.
- Trabajo posterior: ninguno bloqueante dentro del alcance. Otros parentescos,
  entrada de Actores al escenario, reinos y sucesión continúan aplazados.

El detalle y los casos están registrados en la
[sección 12 del contrato](../../tecnica/componentes/14-parentesco-actores-contexto.md#12-verificación-de-la-implementación--2026-10-26).
