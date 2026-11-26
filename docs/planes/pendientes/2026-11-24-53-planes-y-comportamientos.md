# Plan: Planes genéricos y comportamientos de encuentro

| Campo | Valor |
|---|---|
| ID | PLAN-2026-053 |
| Estado | Completado técnicamente; integración de etapa pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-25 |

## Resultado buscado

Dirigir participantes mediante planes con prioridad y roles, reutilizando las
acciones actuales y conservando la resolución simultánea del encuentro.

Este plan es un bloque interno de una única etapa jugable. Su terminación técnica
no exige que el juego funcione durante cada paso ni constituye una entrega final.
La revisión integrada y el cierre conjunto corresponden a PLAN-2026-057.

## Referencias

- [Planes, roles y prioridades](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#7-planes-roles-y-prioridades).
- [Comportamientos y resolución](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#8-comportamientos-y-resolución).
- [Avance y confirmación atómica](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#9-avance-y-confirmación-atómica).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).
- [PLAN-2026-052](./2026-11-24-52-modelo-espacial-tridimensional.md).

## Alcance

### Incluido

- Planes genéricos de Atacar, Rescatar y recuperar objetos.
- Roles por disponibilidad y capacidad; suspensión, bloqueo y sustitución válida.
- Comportamientos actuales de acercarse, combatir, auxiliar, recoger, liberar y salir.
- Prioridad del plan separada del desempate físico de movimientos y reacciones.

### No incluido

- Planificador universal, preparación con equipo nuevo y combate dinámico.
- Creación, composición o sustitución de planes desde Lua.

## Precondiciones

- PLAN-2026-052 completado técnicamente sobre el nuevo modelo.
- Contrato de dirección y resolución aplicable listo para implementar.

## Pasos

1. [x] Extraer la dirección a planes.rs y los cometidos a comportamientos.rs.
2. [x] Coordinar planes por prioridad, asignar roles únicos y conservar identidades de comportamiento.
3. [x] Sustituir auxiliares incapacitados y preservar propósitos cancelados al reanudar.
4. [x] Reutilizar mapas de rutas por ficha con conocimiento filtrado y conservar resolución simultánea.
5. [x] Comprobar el comportamiento y registrar el resultado técnico.

## Verificación

- [x] Un participante recibe una sola dirección efectiva por oportunidad de actuar.
- [x] Una prioridad superior suspende dirección sin conceder ataques o velocidad extra.
- [x] Pérdida y recuperación de un rol preservan hechos confirmados y revisan rutas.
- [x] Los ataques simultáneos y las reacciones mantienen su reglamento vigente.
- [ ] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

La extracción de dirección no debe convertir la prioridad de un plan en iniciativa
física ni reiniciar la historia al recuperar participantes. No añadir reglas nuevas
para resolver un caso no contemplado: actualizar primero su diseño canónico.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a completados; archivar con el cierre de la etapa.

- Fecha de finalización técnica: 2026-11-24.
- Avance: la dirección selecciona el plan activo por prioridad. Cada ficha reutiliza un mapa de rutas ponderadas durante su decisión.
- Roles: actuante, auxiliador único por herido, evacuado y oponente; sustitución por incapacidad y asignación según ruta viable.
- Comportamientos: evacuación, exploración, recuperación y combate separados del resolutor. Dirección con ID estable mientras conserva plan y rol.
- Navegación: un mapa reutilizable por participante; rutas del grupo restringidas a memoria y primer paso de conexiones conocidas; rivales no eligen blancos secretos.
- Estado: cancelación, suspensión y reanudación sin reinicio de posiciones; bloqueo tipado y resolución del estado final del plan.
- Integración de etapa todavía pendiente de los bloques 054 a 057.
- Resultado: planes tipados por propósito, participantes, prioridad y estado; retirada suspende el plan anterior sin alterar el orden físico.
- Verificaciones ejecutadas: 31 pruebas del módulo encuentros; tres de rescate_sesion y una de consecuencias_locales, todas aprobadas. Logs /tmp/053-entrega-core.log, /tmp/053-entrega-rescate.log y /tmp/053-entrega-botin.log. Formato y Clippy del core comprobados.
- Desviaciones respecto del plan: ninguna ampliación de mecánicas; la nueva autoría de planes Lua sigue fuera de alcance.
- Trabajo posterior: [hoja de ruta](../base-espacial-y-planes-encuentros.md).

## Complemento de implementación tras revisión de los pendientes

La primera implementación dejó dirección efectiva pero no toda la memoria y
notificación descrita. Se completan ahora esos puntos; el registro anterior no
constituye prueba de que ya existieran.

- `TipoPlan` distingue incursión de ataque, rescate, robo, oposición y retirada.
  `MemoriaPlan`, asociada por ID, conserva propósito inicial, fase, estado y hechos.
- `ComportamientoActivo` guarda identidad, plan, participantes, rol/cometido,
  destino, referencias de fichas, estado, pulsos, falta de progreso y resultados
  confirmados. La dirección actual apunta a esa instancia; el historial conserva
  las que terminan o se interrumpen. El repertorio inicial asigna instancias por
  ficha y coordina la colaboración por roles del plan.
- Una exploración mantiene su destino mientras siga pendiente y alcanzable;
  al necesitar otro compara coste de ruta e ID de conexión, sin usar orden del
  vector ni información de habitaciones desconocidas.
- Suspender conserva identidad, destino y resultados. Reanudar cancela la retirada;
  una retirada posterior crea otro plan con otro ID, sin revivir un cancelado.
- Incapacidad interrumpe el comportamiento conservando sus resultados. Cumplimiento,
  interrupción y cancelación se distinguen; la oposición tiene su propio resultado,
  independiente del éxito de la misión del jugador.
- `SucesoDireccion` registra plan, pulso, participantes y cambio de estado, fase
  o comportamiento, incluyendo motivo de bloqueo. Consultar no genera sucesos.
- Progreso y hechos se registran por pulso único. Fallar el candidato no confirma
  memoria ni historial. Un comportamiento sin progreso puede quedar bloqueado;
  los resultados ya confirmados no se borran por el bloqueo.

Verificación seleccionada: `planes_locales` (10), `robo_palacio` (1),
`rescate_sesion` (3) y `consecuencias_locales` (3). Clippy del core sin avisos;
formato y diferencias comprobados. Logs `/tmp/053-*.log`. Se reutilizaron las
regresiones existentes; no se ejecutó la batería global.

La interfaz visual mantiene las acciones básicas existentes. Este complemento
implementa su dirección persistente, no un nuevo repertorio de maniobras ni
asignación de planes mediante Lua. El cierre conjunto sigue en el 057.


## Complemento de cierre (2026-11-26)

Se completaron motivos de bloqueo y presupuesto de dirección; la espera conserva el objetivo del comportamiento y la congestión temporal no congela el encuentro. Véase el cierre de los cuatro pendientes en el contrato técnico 12 y la revisión integrada 057.
