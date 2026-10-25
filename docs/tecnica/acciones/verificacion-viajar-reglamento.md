# Implementación y verificación de Viajar

| Campo | Valor |
|---|---|
| Fecha | 2026-10-24 |
| Estado | Cierre técnico; valoración de jugabilidad por el usuario posterior |
| Fuentes | Reglamento 05 y contrato técnico 06 |
| Planes | PLAN-2026-025 a PLAN-2026-031 |

## Resultado

El único motor público de Viajar utiliza las tarjetas del reglamento. Core
resuelve reglas y consecuencias; app coordina, proyecta el calendario y redacta;
terminal presenta cada paso. No se añadieron dependencias ni servicios.

El recorrido incluye planificación, preparación, elección de carga cuando
corresponde, desplazamientos por todas las conexiones, consumo, forrajeo,
advertencias, descarte y cierres reglados. Caballo y camello comparten Cabalgar
y conservan módulos y relatos propios. El calendario conserva la fracción de
día entre acciones; los NPC utilizan el mismo motor sin avanzar el reloj mundial.

Los módulos antiguos incompatibles fueron retirados. La base anterior permanece
en Git; los documentos sustituidos están señalados como históricos. No se
modificaron los archivos personales de `.idea` ni se hicieron commits o pushes.

## Controles automáticos

Ejecutados satisfactoriamente sobre el conjunto integrado:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

La suite contiene 70 pruebas: 39 de core, 16 de app, 12 de terminal y 3 del
importador de retratos. La carga de todo `content/base`, sus referencias,
retratos y políticas Lua se comprueba desde las pruebas de contenido; no existe
otro ejecutable de validación que deba ejecutarse separadamente.

| Contrato | Evidencia reproducible |
|---|---|
| Horas exactas, desbordamientos y resto | `tiempo_exacto_y_resto`, `chequeos_usan_bonos_exhaustivos_tope_y_validan_azar` y pruebas de calendario. |
| Fronteras de consumo, saldo agotado y carencia | `consumo_acumulado_y_saldo_cero` y `llegada_conserva_carencia_que_comienza_en_la_ultima_frontera`. |
| Prefijos comunes y descarte mínimo | `lote_compartido_no_se_resortea_y_descarte_es_minimo_global`, comparación exhaustiva de lotes pequeños; `descarte_de_botas_identicas_usa_ids_menores`. |
| Preparación y carga pendiente | `preparacion_estable_y_demora_exacta`, `descarte_representativo_y_consumo_no_reducen_demora_asignada`. |
| Rutas y dirección | `rutas_ponderan_tamano_y_destinos_son_consultas_puras`, `ocho_octantes_y_direccion_inversa`. |
| Tablas de desplazamiento | `cada_fila_de_incidente_a_pie_y_sus_estados`, `caballo_y_camello_comparten_prueba_pero_no_duracion_ni_incidentes`, `todas_las_filas_de_navegacion_y_costas_de_naufragio`. |
| Estados y Conoce | `estados_no_se_acumulan_y_las_dos_condiciones_previas_si_suman`, `conocimiento_natural_en_barco_y_viento_pueden_producir_cero_horas`. |
| Atomicidad y entradas inválidas | `separar_y_conocer_se_confirman_juntos_y_un_fallo_no_deja_indicios`, `entradas_viejas_o_azar_fallido_no_confirman_un_paso` y pruebas de lotes del modelo. |
| Separación y duración completa | `cancelacion_por_separacion_conserva_toda_la_conexion`. |
| Prioridad de llegada y límite de búsqueda entre conexiones | `agotamiento_en_destino_no_busca_mas_y_una_busqueda_no_se_repite`. |
| Privacidad y narraciones independientes | `sobrecarga_oculta_prevision_y_monturas_conservan_relato_propio`. |
| Recorrido sobre el atlas y calendario final | `todos_los_destinos_terminan_y_no_hay_desfase`: 25 destinos por ocho semillas, más una fuente favorable controlada para demostrar llegada a cada destino. |
| NPC | `cero_y_varios_npc_incluido_un_companero_separado`, junto con pruebas de contrato y errores Lua. |
| Render completo, tamaño mínimo y lectura | `viaje_completo_renderiza_cada_paso_y_redimensiona_sin_avanzar`, pruebas de paginación, detalles, retratos y animación. |

Las semillas ordinarias pueden terminar sin llegar por consecuencias válidas;
no se confunden con una imposibilidad técnica de navegar. Las guardias de
iteraciones existen en los tests para detectar bloqueos; el motor no tiene un
límite de escenas ni omite conexiones.

Medición orientativa local en debug: la matriz del atlas tardó aproximadamente
1,7 segundos. Un descarte de 7 kg con 380 piezas alimenticias y doce botas se
resolvió en unos 81 ms. Son observaciones de esta máquina, no garantías para
cualquier mod ni límites que alteren la simulación.

## Revisión manual técnica agrupada

Se ejecutó el programa en una PTY de 80 × 24, con `TERM=xterm-256color`:

- Contenido oficial: Hamada Roja → Paso Escarpado, llegada tras 11 días y
  12 horas; se recorrieron las páginas de preparación y desplazamiento.
- Contenido oficial: Paso Escarpado → Mar de Poniente, llegada tras 19 días
  y 18 horas. Se comprobó forrajeo, advertencia, continuación, navegación,
  viento y retorno al mapa con la fecha confirmada.
- Desde el mar se inició otro plan y se canceló mediante su opción; se volvió
  al modo de acciones. La salida con Q y confirmación terminó con código cero.
- Copia temporal del contenido, sin alterar el atlas: se añadió únicamente
  Vasto a Hamada Roja para forzar camellos. Apareció «Sobre el paso de los
  camellos», con Cabalgar, responsable y explicaciones por mitad; el viaje
  llegó al Paso Escarpado tras 15 días. Se confirmó la advertencia inicial,
  el paginado, hambre y reservas de llegada, y la salida normal.

Se revisaron controles, composición de escenas, progreso, retratos y retorno
al mundo. El agente realizó esta comprobación funcional; no se atribuye al
usuario una revisión que todavía no hizo. El tono, ritmo y satisfacción de
juego quedan para su próxima prueba del conjunto. Según la metodología, el
refinamiento artístico posterior no bloquea este cierre funcional.

## Notas de implementación y ajustes pequeños

- `ReservasViaje::botas` usa índices locales a las reservas de la decisión.
  El lote queda congelado hasta responder y se normalizan desempates entre
  botas iguales; no se crearon entidades persistentes para ellas.
- Los rumbos son octantes `u8` validados, sin un enum redundante. Las
  particulares reciben parámetros directos, sin `SolicitudChequeo` adicional.
- La tarjeta omite traslados a la ubicación ya ocupada; la primitiva mantiene
  el rechazo anterior. Estos detalles están sincronizados en el contrato 06.
- El Bandido incorpora explícitamente Jinete para expresar la aptitud de
  monta del contenido aprobado. Hamada Roja usa Desierto como clave canónica.
  No se infieren aliases de las características antiguas ni se añaden Vasto,
  Pequeño, habilidades o biografías al atlas para forzar casos de prueba.
- La migración se entregó como un único incremento integrado. Las comprobaciones
  registradas corresponden al motor nuevo; no se afirma haber repetido las
  pruebas del motor anterior después de retirarlo.
- Se corrigieron defectos detectados durante el cierre: desempate de botas
  iguales en el descarte e información de duración filtrada en sobrecarga sin
  plan visible. También se conserva la falta de una ración en la frontera
  exacta de llegada, sin confundirla con una ración completa que agota el saldo.
  Los tres casos tienen pruebas de regresión.

## Ejecución local

Desde la raíz del repositorio:

```sh
cargo run --bin desert-tiger-terminal
```

Para mejorar las reglas o la narración después de probarlo se abre otro
incremento; no se reactivan los planes históricos sustituidos.
