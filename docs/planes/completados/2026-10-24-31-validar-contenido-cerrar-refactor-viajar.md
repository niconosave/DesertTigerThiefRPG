# Plan: Validar contenido y cerrar el refactor de Viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-031 |
| Estado | Completado |
| Creado | 2026-10-24 |
| Actualizado | 2026-10-24 |

## Resultado buscado

El nuevo viaje funciona sobre el atlas oficial y los casos de sus tarjetas están
cubiertos con pruebas reproducibles. Una revisión manual técnica agrupada comprueba su
presentación y continuidad, y la documentación identifica inequívocamente el
reglamento implementado y el historial sustituido.

## Referencias

- [Pruebas y aceptación del refactor](../../tecnica/acciones/06-implementacion-viajar.md#15-pruebas-y-aceptación).
- [Contenido y claves](../../tecnica/acciones/06-implementacion-viajar.md#12-contenido-y-validaciones).
- [Errores y límites](../../tecnica/acciones/06-implementacion-viajar.md#14-errores-y-límites).
- [Documentos y compatibilidad](../../tecnica/acciones/06-implementacion-viajar.md#16-compatibilidad-y-documentos-afectados).
- [Información y narración](../../tecnica/acciones/borradores/05-viajar-redisenio.md#presentacion).
- [Escenario inicial](../../contenido/escenario-inicial.md).
- [Atlas regional](../../contenido/atlas-regional.md).
- [Trayectos de contenido](../../contenido/trayectos-viaje.md).
- [Controles de calidad](../../metodologia-desarrollo.md#7-controles-automáticos).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).
- Dependencia directa: [PLAN-2026-030](./2026-10-24-30-integrar-viajar-narracion-terminal.md).

## Alcance

### Incluido

- adecuación del contenido oficial al significado confirmado de las claves;
- matriz de recorridos y regresiones, comprobaciones de recursos y cierre manual;
- sincronización documental y archivo de los planes realmente terminados.

### No incluido

- inventar historias, Lugares, capacidades de personajes o reglas para forzar
  casos que puedan verificarse con fixtures de prueba;
- nuevas acciones generales, arte avanzado, Unicode, economía o guardado;
- completar retrospectivamente pruebas que no se hicieron del diseño anterior.

## Precondiciones

- PLAN-2026-025 a PLAN-2026-030 implementados, con resultados automáticos registrados.
- Existe un único camino público de `Viajar` y puede jugarse desde la terminal.
- Las pruebas usan azar controlado y cuentan con fixtures específicos por regla.

## Pasos

1. [x] Auditar los RON y los briefs de contenido contra claves, tamaños, costas,
       rumbos y capacidades del contrato. Ajustar solo equivalencias sustentadas
       en el contenido ya aprobado; registrar cualquier decisión creativa nueva
       antes de modificarla.
2. [x] Mantener los casos especiales de carga, camello, separación y naufragio
       en fixtures de prueba cuando el atlas no los ofrezca; no distorsionar
       la geografía oficial para hacer pasar las pruebas.
3. [x] Ampliar la matriz de recorridos oficiales por destinos y semillas para
       comprobar terminación válida, coherencia de Grupo, conexiones, reloj,
       reservas y resúmenes, admitiendo los cierres narrativos reglados.
4. [x] Cubrir las regresiones de tiempo mostrado frente a fecha final y de
       navegación hasta mares, con semillas que ejerciten llegada y otras que
       ejerciten interrupción; distinguir fallos técnicos de consecuencias válidas.
5. [x] Ejecutar los casos límite del contrato y medir preparación, búsqueda de
       rutas y descarte con cargas y mapas representativos; corregir defectos
       respetando sus algoritmos y registrar cualquier necesidad de revisión técnica.
6. [x] Ejecutar controles generales, carga y validación de todos los datos y Lua
       oficiales, pruebas de extremo a extremo y build de producción.
7. [x] Preparar y efectuar una sola revisión manual agrupada de viaje terrestre,
       marítimo e interrupción. Registrar legibilidad, tono de cada medio,
       controles, cámara, retratos, consumo, calendario y retorno a acciones.
8. [x] Corregir los defectos encontrados y repetir solo las verificaciones
       afectadas; completar también la revisión manual agrupada de PLAN-2026-030.
9. [x] Sincronizar estados y contratos técnicos de los componentes migrados,
       conservar referencias históricas y señalar qué documentos quedaron
       sustituidos. No atribuir al código comportamientos aún no verificados.
10. [x] Completar los resultados de PLAN-2026-025 a PLAN-2026-031 y mover a
        `completados/` solo los que cumplan su definición de terminado; actualizar
        índices y enlaces en el mismo cambio.

## Verificación

- [x] El contenido oficial carga sin diagnósticos ni referencias inconsistentes.
- [x] La matriz de aceptación de la sección 15 tiene pruebas y resultados registrados.
- [x] Puede alcanzarse un mar con una semilla reproducible sin exigir muelle,
      puerto o capacidad material que el reglamento ya no requiere.
- [x] Toda interrupción de gameplay termina en estado válido y conserva el
      tiempo y las consecuencias regladas; no se trata como error del motor.
- [x] Fecha final, progreso visible y consumo provienen del mismo tiempo exacto.
- [x] La revisión manual confirma escenas propias de caballo y camello, cuando
      corresponda usando contenido de prueba, con la misma acción Cabalgar.
- [x] Los algoritmos no omiten tramos ni escenas para cumplir un presupuesto.
- [x] Pasan `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
- [x] Los planes anteriores sustituidos conservan su historial y no se marcan
      completados por haber implementado un diseño distinto.
- [x] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Una tabla de incidentes permite terminar sin llegar: la matriz distingue ese
  resultado de una imposibilidad técnica de alcanzar un destino.
- Si falta una decisión creativa sobre contenido, se registra y se consulta;
  los fixtures mecánicos permiten avanzar sin inventar biografías o poderes.
- Los ajustes artísticos no bloqueantes se anotan para otro incremento sin
  ampliar este refactor ni omitir información que el reglamento exige.

## Resultado de la implementación

- Fecha de finalización: 2026-10-24.
- Resultado: Contenido oficial validado, regresiones verificadas, revisión funcional agrupada ejecutada y documentación sincronizada.
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
- Trabajo posterior: otro incremento tras la evaluación de jugabilidad del usuario. No quedan tareas de implementación de este plan.
