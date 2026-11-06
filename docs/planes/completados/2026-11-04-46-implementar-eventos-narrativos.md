# Plan: integrar eventos narrativos durante las acciones

| Campo | Valor |
|---|---|
| ID | PLAN-2026-046 |
| Estado | Completado |
| Creado | 2026-11-04 |
| Actualizado | 2026-11-05 |

## Resultado buscado

Jugar eventos definidos en RON/Lua que aparezcan en puntos preparados de Viajar,
Acampar y la sesión; tomar decisiones, confirmar sus efectos y continuar o
interrumpir el viaje sin repetir consecuencias. Comprobar también la sustitución
del relato de llegada y el encadenamiento de eventos.

El contrato 23 define qué construir. Este plan ordena su implementación y
verificación; no establece mecánicas ni episodios canónicos nuevos.

## Referencias

- [Alcance inicial y módulos](../../tecnica/componentes/23-eventos-narrativos.md#2-alcance-de-la-primera-versión), secciones 2–3.
- [Puntos y contexto](../../tecnica/componentes/23-eventos-narrativos.md#4-puntos-de-intervención-del-motor).
- [Autoría RON y grafos](../../tecnica/componentes/23-eventos-narrativos.md#5-esquema-de-contenido).
- [Selección y continuación](../../tecnica/componentes/23-eventos-narrativos.md#6-estado-por-partida-y-selección), secciones 6–7.
- [API Lua](../../tecnica/componentes/23-eventos-narrativos.md#8-lua-de-eventos) y [confirmación atómica](../../tecnica/componentes/23-eventos-narrativos.md#9-confirmación-atómica-por-paso).
- [Tiempo y fase NPC](../../tecnica/componentes/23-eventos-narrativos.md#10-tiempo-recursos-y-fase-npc).
- [Vistas y recursos](../../tecnica/componentes/23-eventos-narrativos.md#11-vistas-y-recursos-gráficos).
- [Errores](../../tecnica/componentes/23-eventos-narrativos.md#12-errores-y-diagnóstico), [extensiones aplazadas](../../tecnica/componentes/23-eventos-narrativos.md#14-ampliaciones-previstas-y-límites-de-compatibilidad) y [aceptación](../../tecnica/componentes/23-eventos-narrativos.md#15-verificación-y-aceptación).
- [Metodología](../../metodologia-desarrollo.md), especialmente controles y definición de terminado.

## Alcance

### Incluido

La versión 1 completa del contrato 23: catálogo opcional, condiciones y pasos
Lua, puntos registrados, coordinador por partida, efectos autorizados,
continuaciones, interrupción reglada, sustitución del relato de llegada, cola,
vistas e ilustraciones alternativas. Fixtures jugables para verificarla.

### No incluido

Las extensiones aplazadas en la sección 14 del contrato: tiempo propio,
repetición, sustitución de cálculos de movimiento, nuevas operaciones de
Relaciones/Grupo/inventario, ejecución interna de eventos en NPC, acciones o
chequeos solicitados por Lua y guardado. Tampoco se implementan Atacar,
Rescatar o Reclutar ni se fija el drama definitivo del príncipe.

## Precondiciones

- Contrato 23 v1.0 aprobado para implementar, sin decisiones técnicas abiertas.
- Conservar los datos y ajustes visuales del diseñador; trabajar sobre el código
  actual, sin recuperar diseños históricos sustituidos de Viajar.
- Usar las dependencias ya documentadas; si aparece una necesidad no cubierta,
  resolverla en el documento técnico antes de ampliar el plan.

## Pasos

1. [x] Registrar la línea base y preparar los fixtures de desarrollo.
   Ejecutar las pruebas existentes relevantes de Viajar, Acampar y fase NPC;
   identificar las comprobaciones de tiempo, provisiones y RNG que deberán
   conservarse. Preparar datos aislados para los casos de aceptación de la
   [sección 15](../../tecnica/componentes/23-eventos-narrativos.md#15-verificación-y-aceptación),
   con un recorrido reproducible mediante la opción `--content` existente.
   Los fixtures no dependen de archivos personales de arte ni modifican el canon.

2. [x] Ampliar la preparación y confirmación de operaciones del núcleo.
   Implementar las bases transaccionales y escrituras globales tipadas de las
   [secciones 8.2 y 9](../../tecnica/componentes/23-eventos-narrativos.md#82-constructores-de-efectos).
   Verificar creación dinámica de claves, incompatibilidad de tipos, lotes
   narrativos vacíos y fallo de la última operación sin efectos parciales.

3. [x] Implementar la carga y validación del catálogo de eventos.
   Crear el esquema RON, descubrimiento opcional, referencias, grafos finitos,
   catálogo de puntos e índice de candidatos según las
   [secciones 5–6](../../tecnica/componentes/23-eventos-narrativos.md#5-esquema-de-contenido).
   Cubrir carpeta ausente/vacía, duplicados, pasos inalcanzables, ciclos,
   controles o modos no soportados y referencias de encadenamiento.

4. [x] Implementar la API Lua de eventos con permisos separados de las políticas.
   Añadir contexto, consultas, presentación pura, constructores opacos y
   conversión estricta de resultados conforme a la
   [sección 8](../../tecnica/componentes/23-eventos-narrativos.md#8-lua-de-eventos).
   Probar límites, errores con origen, independencia entre llamadas y ausencia
   de mutación, RNG o IO en condiciones y presentación. Conservar la API NPC.

5. [x] Incorporar los puntos y las continuaciones sin activar eventos todavía.
   Adaptar motor, sesión y consumidores a `SalidaMotor`, tokens y resolución de
   puntos; separar la búsqueda con azar de `viajar.inicio`. Integrar todos los
   puntos de la [sección 4](../../tecnica/componentes/23-eventos-narrativos.md#4-puntos-de-intervención-del-motor),
   incluido el episodio montañoso, y la resolución genérica dentro de la fase
   NPC de la [sección 10](../../tecnica/componentes/23-eventos-narrativos.md#10-tiempo-recursos-y-fase-npc).
   Comprobar que el catálogo vacío conserva el juego y la secuencia RNG, y que
   resolver un token anterior o duplicado no ejecuta una transición otra vez.

6. [x] Integrar el coordinador narrativo en Partida.
   Implementar selección, reserva, pasos, marcadores, sustitución y cola conforme
   a las [secciones 6–7](../../tecnica/componentes/23-eventos-narrativos.md#6-estado-por-partida-y-selección).
   Conectar la confirmación conjunta de efectos, estado del evento y siguiente
   vista de la [sección 9](../../tecnica/componentes/23-eventos-narrativos.md#9-confirmación-atómica-por-paso).
   Verificar prioridad y reevaluación, validación del punto real en encadenados,
   un solo sustituto, dos partidas independientes y fallos atómicos. Confirmar
   `Fin` antes de ejecutar, por separado, la continuación genérica.

7. [x] Completar reanudación, interrupción y cierre temporal.
   Verificar los adaptadores por raíz/fase y el cierre `InterrumpidoPorEvento`
   según las [secciones 7 y 10](../../tecnica/componentes/23-eventos-narrativos.md#7-coordinador-y-continuación).
   Cubrir cambios incompatibles antes de confirmar, interrupción pendiente con
   eventos encadenados, tiempo efectivo con fracción, conservación de consumo
   y una sola confirmación de calendario y fase NPC. Ningún cierre de evento
   debe cerrar por sí mismo otra acción ni volver a ejecutar NPC.

8. [x] Añadir las vistas e interacciones de eventos al frontend.
   Implementar la [sección 11](../../tecnica/componentes/23-eventos-narrativos.md#11-vistas-y-recursos-gráficos):
   evento al iniciar la partida, decisiones propias, detalle, páginas,
   ilustraciones y variantes ASCII/fuente. Verificar identificadores y opciones
   vencidos, redimensionado y retorno a la acción suspendida. Dibujar o avanzar
   páginas no debe consultar condiciones ni confirmar efectos.

9. [x] Completar la aceptación integrada y revisar los recorridos jugables.
   Ejecutar la matriz de la [sección 15](../../tecnica/componentes/23-eventos-narrativos.md#15-verificación-y-aceptación).
   Probar inserción con variable y reanudación, sustitución de llegada, decisión
   de interrupción y cadena A→B, además de un evento inicial y Acampar. Cubrir
   el fixture de 1.000 eventos para comprobar la selección por índice. Revisar
   los casos representativos en terminal y registrar el comando de demostración.

10. [x] Ejecutar controles generales y cerrar la documentación del incremento.
    Validar el contenido oficial y los fixtures, ejecutar los comandos de
    verificación y compilar release. Actualizar los contratos para describir el
    resultado real, registrar verificaciones y desviaciones y mover este mismo
    plan a `completados/` solo al cumplir la definición de terminado.

Los pasos son secuenciales por sus dependencias. Durante los cambios de firmas se
adaptan sus consumidores en el mismo bloque para conservar una compilación
utilizable. El juego con catálogo vacío debe seguir siendo jugable durante el
trabajo. Las verificaciones automáticas se hacen con cada bloque; la revisión
manual se concentra al final del lote, sin pedir confirmación después de cada
viñeta o detalle.

## Verificación

- [x] Matriz del contrato 23, sección 15, cubierta por pruebas de comportamiento.
- [x] Regresiones de Viajar, Acampar, fase NPC y secuencia RNG sin eventos.
- [x] Contenido oficial y fixtures RON/Lua válidos, con diagnósticos de los casos inválidos.
- [x] Recorridos representativos revisados en terminal, incluida la reanudación.
- [x] `cargo fmt --all -- --check`.
- [x] `cargo check --workspace --all-targets`.
- [x] `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] `cargo test --workspace --all-targets`.
- [x] `cargo build --release -p desert-tiger-terminal`.
- [x] Se cumplieron los controles y la [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- La cesión antes de las búsquedas con azar se verifica antes de habilitar
  condiciones de eventos: no basta insertar texto en `publicar`.
- La preparación transaccional precede al coordinador para no implementar
  efectos parciales como solución transitoria.
- Los consumidores NPC se adaptan junto al protocolo del motor; su ejecución
  interna sigue sin mostrar eventos ni avanzar la fecha mundial.
- Los ejemplos jugables usan contenido de desarrollo aislado. La implementación
  no decidirá lealtades o historias de personajes todavía no diseñados.

Esta sección no crea reglas ni decisiones técnicas. Si aparece un problema no
resuelto por el contrato, se actualiza primero su documento canónico y después
se registra aquí la repercusión en el trabajo.

## Resultado de la implementación

- Fecha de finalización: 2026-11-05.
- Resultado: catálogo opcional RON/Lua, siete puntos registrados, decisiones,
  efectos atómicos, eventos de una ocurrencia, encadenamiento, interrupción y
  sustitución del relato de llegada integrados en la sesión y la terminal.
- El contenido base mantiene su historia. La
  [demo aislada](../../../content/demos/eventos/README.md) contiene cinco eventos
  y explica cómo preparar una copia mediante `tools/preparar-demo-eventos.py`.

### Verificaciones ejecutadas

- `cargo fmt --all -- --check`: correcto.
- `cargo check --workspace --all-targets`: correcto.
- `cargo clippy --workspace --all-targets -- -D warnings`: correcto.
- `cargo test --workspace --all-targets`: correcto; después se ejecutaron las
  pruebas adicionales de cierre indicadas a continuación, también correctas.
- `cargo test -p desert-tiger-app eventos_pruebas`: doce pruebas correctas,
  incluida comparación de todos los pasos, cambios y consumo de azar entre el
  motor genérico y el adaptador sin eventos para todos los destinos visibles,
  con una fuente reproducible favorable.
- Prueba del núcleo sobre preparación vacía, globales dinámicas, incompatibilidad
  de tipos y fallo tardío sin efectos parciales: correcta.
- Pruebas de terminal para evento inicial, paginación, detalle, decisiones y
  redimensionado, y variantes parciales de ilustraciones: correctas.
- Fixture de 1.000 eventos: correcto; no evalúa condiciones de puntos ajenos.
- `cargo build --release -p desert-tiger-terminal`: correcto.
- Dos recorridos en pseudoterminal con la demo y el ejecutable release:
  continuar hasta el regreso al mundo y detenerse en el evento intermedio.
  Ambos mostraron sus ventanas y terminaron con salida normal del programa.
- Se verificaron también los puntos de Acampar y cierre, encadenamiento con
  interrupción pendiente, prioridades, revalidación, errores de Lua y lotes,
  respuestas vencidas e independencia de dos partidas.

### Ajustes respecto de la organización prevista

- `acciones::intervenciones` envuelve la máquina existente. Los consumidores sin
  eventos, incluidos los NPC, conservan la API genérica. La comparación de
  recorridos verifica que no cambia su simulación ni su consumo de azar.
- Las vistas públicas están junto al coordinador; la integración de sesión está
  en `sesion/eventos_partida.rs`. No se creó un módulo vacío de vistas.
- Un momento montañoso que agrupa varios centros ofrece un único punto seguro
  al finalizar ese momento, situado en el último lugar confirmado. No se
  inventan intervenciones retrospectivas en centros ya recorridos. El contrato
  23 refleja esta granularidad.
- Se corrigieron dos pruebas previas de tráfico decorativo que suponían glifos
  fijos. Ahora consultan los recursos del diseñador, sin modificar su arte.

Trabajo posterior: las extensiones excluidas en la sección 14 del contrato 23.
La revisión artística del contenido definitivo sigue siendo trabajo de diseño;
esta demo no decide nuevos acontecimientos canónicos del mundo.
