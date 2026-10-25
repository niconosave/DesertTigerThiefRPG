# Plan: Preparar rutas y rumbo provisional de Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-026 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El refactor puede obtener rutas y duraciones previstas desde las Relaciones,
y dispone de un catálogo de rumbos dirigido construido por la aplicación.
Sus resultados se prueban sin ejecutar todavía las tarjetas nuevas en producción.

## Referencias

- [Rutas, medios y rumbo provisional](../../tecnica/acciones/06-implementacion-viajar.md#6-rutas-medios-y-rumbo-provisional).
- [Contenido y validaciones](../../tecnica/acciones/06-implementacion-viajar.md#12-contenido-y-validaciones).
- [Errores](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Pruebas y aceptación](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Reglamento de rutas](../../tecnica/acciones/borradores/05-viajar-redisenio.md#rutas).
- [Duraciones regladas](../../tecnica/acciones/borradores/05-viajar-redisenio.md#duraciones).
- [Dependencia: PLAN-2026-025](./2026-10-24-25-preparar-tiempo-chequeos-reservas.md).
- [Metodología y controles](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- evaluación pura de duración por mitad y selección de medio;
- algoritmo de ruta rápida y construcción de la alternativa;
- catálogo de rumbos y validaciones correspondientes en aplicación;
- diagnósticos del contenido y pruebas de grafos y geometría.

### No incluido

- publicar pronósticos nuevos en el selector o activar el motor del refactor;
- dibujar rutas ni deducir conexiones desde tiles;
- preparación de carga, incidentes, viento estacional o narración de tarjetas;
- rediseñar la distribución del atlas o crear una geografía adicional.

## Precondiciones

- PLAN-2026-025 implementado y verificado.
- Contrato técnico y reglamento aprobados, incluidos el retorno a la ruta
  rápida y la abstracción provisional del rumbo desde centros.
- Relaciones e identificadores de la base anterior continúan disponibles.

## Pasos

1. [x] Implementar y probar el evaluador compartido de medios y duración por
       mitad, conservando el desglose que necesitarán las escenas.
2. [x] Implementar la ruta rápida y su reconstrucción con desempate reproducible
       conforme al contrato; mantener puras las consultas de destinos.
3. [x] Implementar la alternativa mediante desvíos, sus exclusiones, terminación
       y retorno permitido a la ruta rápida.
4. [x] Probar grafos con rutas únicas, empates, destinos desconectados, distintos
       tamaños y alternativas que no alcanzan el intervalo deseado.
5. [x] Añadir en la aplicación el adaptador de centros a rumbos y construir el
       catálogo inmutable que recibirá el motor nuevo.
6. [x] Implementar los diagnósticos de tamaño contradictorio, navegación sin
       costa y conexiones navegables sin rumbo, según la sección 12.
7. [x] Probar octantes, inversión de dirección, eje vertical, fronteras y
       coordenadas coincidentes sin introducir datos visuales en el núcleo.
8. [x] Comprobar el contenido oficial con las validaciones nuevas y registrar
       cualquier incompatibilidad real antes de modificar datos creativos.
9. [x] Ejecutar controles y prueba de humo de la versión actual; actualizar el
       registro de implementación sin declarar activadas las tarjetas futuras.

## Verificación

- [x] Pasan los casos de rutas, rumbo y medios aplicables de la sección 15.
- [x] Consultar destinos no sortea rutas, gasta tiempo ni modifica Mundo.
- [x] Una ruta adoptable conserva conexiones reales y extremos correctos.
- [x] El catálogo no expone coordenadas o tiles al core y admite el mismo uso
      para jugador y NPC.
- [x] La carga de todos los archivos oficiales y sus referencias sigue válida.
- [x] La versión integrada continúa jugable; véase la desviación de integración al final.
- [x] Se ejecutaron satisfactoriamente:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- El adaptador es la excepción provisional definida por el contrato; no
  habilita consultas de mapa desde las demás reglas del núcleo.
- Los algoritmos se incorporan como preparación del refactor y no añaden una
  selección de motor en la interfaz. La sustitución se realiza en PLAN-2026-030.
- No se pide una revisión manual por cada variante de grafo o dirección;
  se cubren con pruebas reproducibles y se conserva el recorrido actual.
- Una regla ausente se resuelve en su fuente técnica antes de seguir.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Rutas rápida y alternativa, medios por mitad y catálogo provisional de rumbos implementados.
- Verificaciones: los cinco controles generales pasaron; 70 pruebas del workspace
  y carga del contenido y Lua oficiales. Casos, nombres de pruebas, mediciones y
  recorridos manuales constan en el
  [informe de verificación](../../tecnica/acciones/verificacion-viajar-reglamento.md).
- Desviación operativa: la serie se entregó como un incremento integrado; los
  controles finales verifican el motor nuevo. No se afirma conservar en ejecución
  la versión anterior tras retirarla. La revisión manual agrupada la realizó el
  agente en PTY; la valoración artística y de jugabilidad del usuario es posterior
  y no se presenta como ya aprobada.
- Ajustes pequeños: representación local de botas, octantes y parámetros de
  chequeos documentados en el contrato 06 y en el informe; sin reglas nuevas.
- Trabajo posterior: PLAN-2026-027. No quedan tareas de implementación de este plan.
