# Plan: comparar propuestas de caminos antes de viajar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-044 |
| Estado | Completado; revisión de jugabilidad del usuario pendiente |
| Creado | 2026-11-03 |
| Actualizado | 2026-11-03 |

## Resultado buscado

Viajar directamente a vecinos y comparar caminos propuestos por el Grupo con
previsualización sobre el mapa antes de confirmar.

## Referencias

- [Reglas y búsquedas](../../tecnica/acciones/08-planificacion-rutas.md#2-reglas-del-juego).
- [Contrato del motor y aplicación](../../tecnica/acciones/08-planificacion-rutas.md#4-motor-y-aplicación).
- [Interfaz y dibujo](../../tecnica/acciones/08-planificacion-rutas.md#5-selección-y-previsualización-terminal).
- [Verificación](../../tecnica/acciones/08-planificacion-rutas.md#7-verificación-y-aceptación).

## Alcance

### Incluido

Ruta directa, coste compartido, propuestas, elección dinámica, lista y dibujo.

### No incluido

Nuevos accesos por Bloquea o bonificaciones de desplazamiento para montaraz.

## Precondiciones

Contrato 08 aprobado; conservar datos y arte editados por el usuario.

## Pasos

1. [x] Implementar ruta directa, perfiles, límites, agrupación y protocolo del motor.
2. [x] Proyectar propuestas y detalles desde la aplicación.
3. [x] Incorporar lista, foco, cámara y puntos respetando paleta y niebla.
4. [x] Verificar reglas, integración, interfaz y compilación de producción.
5. [x] Actualizar contratos y registrar el resultado.

## Verificación

- [x] Pruebas de la sección 7 del contrato 08.
- [x] Formato, check, Clippy y pruebas del workspace.
- [x] Contenido oficial válido y compilación de producción.
- [x] Humo de la interfaz y controles del selector.
- [x] Se cumplieron los controles y la definición de terminado de la metodología.

## Riesgos y decisiones operativas

Las pruebas anteriores que exigían deliberación en destinos vecinos se adaptan
al nuevo contrato; las reglas conservadas mantienen sus verificaciones.

## Resultado de la implementación

- Fecha de finalización: 2026-11-03.
- Resultado: vecinos directos sin deliberación; propuestas por conocimientos,
  montaña y caballo, con agrupación y límite de duración; elección y recorrido
  previsualizado en mapa, detalles y cámara.
- Verificaciones: 248 pruebas del workspace y todos sus targets; formato, check,
  Clippy sin advertencias, check sin features predeterminadas y build release.
  Las pruebas de aplicación validaron la carga del contenido oficial.
- Humo en PTY: abrir selector, elegir destino no adyacente, cambiar el foco,
  consultar detalles, mover cámara, redimensionar, confirmar y salir.
- Regresiones: rutas óptimas requieren habilitación; vecinos y nodos ocultos;
  límites y duplicados; confirmación sin regeneración; colores y niebla sin
  descubrir terreno.
- Desviaciones: ninguna regla nueva fuera del contrato. Se actualizaron pruebas
  históricas que exigían planificación al viajar a un vecino.
- Revisión posterior: estética y sensación de las propuestas por el usuario.
  El desarrollo y las verificaciones del alcance están completos.
