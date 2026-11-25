# Plan: Vista segura y corte visual por altura

| Campo | Valor |
|---|---|
| ID | PLAN-2026-055 |
| Estado | Completado técnicamente; integración de etapa pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-24 |

## Resultado buscado

Representar el volumen conocido con caracteres y un corte de altura, con
superposición correcta y sin revelar datos ocultos o modificar la simulación.

Este plan es un bloque interno de una única etapa jugable. Su terminación técnica
no exige que el juego funcione durante cada paso ni constituye una entrega final.
La revisión integrada y el cierre conjunto corresponden a PLAN-2026-057.

## Referencias

- [Vista filtrada del encuentro](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#12-vista-filtrada-del-encuentro).
- [Terminal y corte visual](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#13-terminal-y-corte-visual).
- [Errores y límites](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#14-errores-y-límites).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).
- [PLAN-2026-052](./2026-11-24-52-modelo-espacial-tridimensional.md).

## Alcance

### Incluido

- Vista tipada filtrada según las reglas de información del encuentro.
- Proyección pura en planta con oclusión de pisos y corte superior.
- Control de altura mediante [ y ], retorno de cámara mediante C e indicador visible.
- Conservación de pausa, reproducción, resize y dibujo de actores u objetos.

### No incluido

- Gráficos vectoriales, representación isométrica y renderizado 3D.
- Cambiar colisiones o percepción al cambiar la cámara; nuevas órdenes tácticas.

## Precondiciones

- PLAN-2026-052 completado técnicamente.
- Contrato de vista y controles listo para implementar.

## Pasos

1. [x] Proyectar terreno y elementos recordados y filtrar entidades por observación.
2. [x] Resolver profundidad entre terreno, puertas, fichas y botín sin superposiciones incorrectas.
3. [x] Integrar corte, seguimiento y cámara vinculados al ID del encuentro.
4. [x] Admitir controles detrás de eventos, preservar reproducción y corregir altura del frame de impacto.
5. [x] Verificar proyección, controles, memoria y regresiones narrativas con casos seleccionados.

## Verificación

- [x] Un piso tapa el sótano; un hueco muestra lo inferior conocido no obstruido.
- [x] Cambiar corte no revela fichas desconocidas ni cambia azar, estado o tiempo.
- [x] La posición conserva altura real aunque una representación resulte recortada.
- [x] TestBackend comprueba controles, oclusión y resize sin una batería global.
- [ ] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

Separar datos permitidos y proyección: ocultar con la cámara no autoriza enviar
identidades desconocidas. El proyector no resuelve alcance, movimiento o visibilidad
física por su cuenta.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a completados; archivar con el cierre de la etapa.

- Fecha de finalización técnica: 2026-11-24.
- Avance: la app proyecta terreno recordado por espacio y filtra fichas por presencia y conexiones abiertas. Una puerta cerrada oculta los cambios posteriores del espacio recordado.
- Memoria de elementos: la vista usa elementos_recordados; un cierre fuera de observación no revela el estado real. El botín se publica por observación actual, no por haber descubierto alguna vez su sala.
- Proyección: profundidad por intervalo del cuerpo y corte, prioridad de ficha frente al botín y desempate por ID; pisos, obstáculos, puertas cerradas y desconocido ocultan las fichas e impactos inferiores.
- Cámara: seguimiento del foco conocido, corte manual y centro XY conservados cuando falta foco. Otro ID de encuentro reinicia seguimiento; los controles [ ] y C funcionan durante la suspensión narrativa sin avanzar eventos.
- Reproducción: el frame de impacto conserva X/Y/Z de origen y busca la ficha por ID estable.
- Integración final pendiente de los bloques 056 y 057.
- Resultado: vista volumétrica filtrada, proyección superior pura y controles `[`, `]` y `C` con indicador de corte.
- Verificaciones ejecutadas: cuatro corte_altura de terminal (incluye TestBackend y evento suspendido), una integracion_encuentros_3d, una vista_local_memoria y dos eventos_locales_3d; todas aprobadas. Logs /tmp/055-entrega-terminal.log, /tmp/055-final-integracion.log, /tmp/055-memoria.log y /tmp/055-final-eventos.log. Formato y diff correctos; Clippy sin errores, con el aviso previo too_many_arguments de dibujar_retratos_dialogo.
- Desviaciones respecto del plan: ninguna ampliación de reglas físicas; la vista pública sigue siendo exclusivamente la del grupo jugador.
- Trabajo posterior: [hoja de ruta](../base-espacial-y-planes-encuentros.md).
