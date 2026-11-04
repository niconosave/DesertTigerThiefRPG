# Plan: atravesar pasos montañosos como un episodio

| Campo | Valor |
|---|---|
| ID | PLAN-2026-045 |
| Estado | Completado; revisión de jugabilidad del usuario pendiente |
| Creado | 2026-11-03 |
| Actualizado | 2026-11-03 |

## Resultado buscado

Recorrer una secuencia de pasos en un único episodio narrativo con obstáculos,
consumo y avance reales, y campamento en el centro de las secuencias impares.

## Referencias

- [Reglas y momentos](../../tecnica/acciones/09-cruce-pasos-montanosos.md#3-momentos-del-episodio).
- [Distribución y contabilidad](../../tecnica/acciones/09-cruce-pasos-montanosos.md#5-distribución-de-momentos-y-avance-real).
- [Integración](../../tecnica/acciones/09-cruce-pasos-montanosos.md#6-integración-técnica).
- [Verificación](../../tecnica/acciones/09-cruce-pasos-montanosos.md#9-verificación-prevista).

## Alcance

### Incluido

Episodio consecutivo, cuatro chequeos, pérdidas, contabilidad parcial,
interrupciones de supervivencia y presentación con las viñetas existentes.

### No incluido

Tablas de eventos, decisiones nuevas, bonificaciones de montaraz o accesos Bloquea.

## Precondiciones

Contrato 09 aprobado; conservar los cambios del diseñador en entrada y salida de Zunir.

## Pasos

1. [x] Incorporar estado, registros y contabilidad compartida del episodio.
2. [x] Integrar recorrido, chequeos, pérdidas e interrupciones con el motor.
3. [x] Redactar momentos, detalles y proyección de progreso.
4. [x] Verificar secuencias, tiempos, consumo, fallos y reanudación.
5. [x] Ejecutar controles generales, humo y build de producción; cerrar documentación.

## Verificación

- [x] Casos de la sección 9 del contrato 09.
- [x] Formato, check, Clippy y tests del workspace.
- [x] Contenido oficial válido, humo de interfaz y build release.
- [x] Controles y definición de terminado de la metodología.

## Riesgos y decisiones operativas

La contabilidad parcial comparte consumo con las tarjetas actuales. Las pruebas
comparan el total y las reservas entre duraciones completas y fragmentadas,
además de comprobar que un error no confirma parcialmente un momento.

## Resultado de la implementación

- Fecha de finalización: 2026-11-03.
- Resultado: episodio montañoso único por secuencia, con seis/siete momentos,
  cuatro chequeos, pérdidas proporcionales y avance parcial con consumo común.
- Verificaciones: 263 tests del workspace; formato, check, Clippy sin advertencias,
  check sin features predeterminadas y compilación release. Las pruebas de carga
  validaron el contenido oficial.
- Casos del episodio: longitudes 2/3/4/5/9, regreso en sentido inverso, tamaños
  distintos, campamento central, fracciones de tiempo, raciones, pérdidas sin agua,
  NPC solo, hambre, reanudación, errores antes y después de fronteras.
- Humo PTY con fixture temporal de Zunir: introducción, campamento central y
  desenlace visibles. Los cambios del diseñador en los RON originales se conservaron.
- Ajuste técnico: se reutilizó el clon de EjecucionAccion que ya usan avanzar y
  responder; Contabilidad prepara los intervalos y el lote se publica una vez.
  Las firmas concretas y responsabilidades están actualizadas en el contrato 09.
- Trabajo posterior: revisión del ritmo narrativo por el usuario. Las tablas y
  decisiones futuras siguen fuera de este incremento.
