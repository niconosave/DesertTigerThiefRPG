# Plan: Implementar el frontend de terminal

| Campo | Valor |
|---|---|
| ID | PLAN-2026-006 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

`desert-tiger-terminal` puede transformar las vistas de aplicación en la
interfaz ASCII aprobada, traducir teclado a intenciones y probar su composición
completa mediante un backend de Ratatui sin depender de una terminal real.

## Referencias

- [Responsabilidades de `desert-tiger-terminal`](../../tecnica/componentes/01-workspace-entorno.md#desert-tiger-terminal).
- [Entradas, salidas y configuración](../../tecnica/componentes/01-workspace-entorno.md#6-entradas-salidas-y-configuración).
- [Pruebas y criterios de workspace](../../tecnica/componentes/01-workspace-entorno.md#9-pruebas-y-criterios-de-aceptación).
- [Intenciones del frontend](../../tecnica/componentes/04-sesion-aplicacion.md#6-intenciones-del-frontend).
- [Vistas y salidas](../../tecnica/componentes/04-sesion-aplicacion.md#7-vistas-y-salidas).
- [Categorías de error de sesión](../../tecnica/componentes/04-sesion-aplicacion.md#8-categorías-de-error).
- [Responsabilidades internas](../../tecnica/componentes/09-frontend-terminal.md#4-responsabilidades-internas).
- [Contrato con la aplicación](../../tecnica/componentes/09-frontend-terminal.md#5-contrato-con-la-aplicación).
- [Modos visuales](../../tecnica/componentes/09-frontend-terminal.md#6-modos-visuales-iniciales).
- [Composición de pantalla](../../tecnica/componentes/09-frontend-terminal.md#7-composición-de-la-pantalla).
- [Bucle y restauración](../../tecnica/componentes/09-frontend-terminal.md#8-bucle-animación-y-restauración).
- [Entrada y accesibilidad](../../tecnica/componentes/09-frontend-terminal.md#9-entrada-y-accesibilidad-básica).
- [Errores y validaciones](../../tecnica/componentes/09-frontend-terminal.md#10-errores-y-validaciones).
- Plan previo: [PLAN-2026-005](./2026-10-15-05-implementar-sesion-fase-npc.md).

## Alcance

### Incluido

- carga y validación de los tres recursos visuales de terminal;
- estado visual, modos, foco, cámara, paginación y animación;
- renderizado de mapa, rótulos, retratos, barras y ventanas superpuestas;
- asignación y traducción de teclas, selección, decisiones y salida;
- presentación segura de errores y resúmenes NPC;
- pruebas de estado, entrada y búfer con `TestBackend`.

### No incluido

- reglas de gameplay o reinterpretación de Características;
- setting y arte definitivo, ratón, audio o frontend gráfico;
- prueba manual integrada con el dispositivo real, reservada para el plan final.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-005](./2026-10-15-05-implementar-sesion-fase-npc.md) están resueltos
  y registrados; el plan permanece pendiente hasta integrar el incremento.

## Pasos

1. [x] Implementar DTO, carga y validación de `tiles.ron`, `ui.ron` y
       `scenes.ron` antes de activar el modo raw.
2. [x] Crear recursos visuales mínimos que cubran los tipos y escenas del fixture
       sin fijar el acabado artístico definitivo.
3. [x] Implementar el estado de interfaz y las transiciones entre vista del
       mundo, selección, presentación, decisión, salida y terminal insuficiente.
4. [x] Implementar layout, cámara y capas de mapa, rótulos y retratos con recorte
       seguro, transparencia y solapamiento aprobados.
5. [ ] Implementar ventanas, ajuste y paginación de texto, arte opcional y las
       dos barras con prioridades, estilos y estados deshabilitados.
6. [x] Implementar el adaptador de teclado y atajos por modo, incluida la
       confirmación local de salida sin enviar intenciones extra.
7. [x] Implementar reloj monotónico de animación, redimensionado y guardián de
       restauración de Crossterm para salidas y fallos.
8. [x] Implementar la presentación segura de errores iniciales, rechazos,
       incidencias NPC y vistas fatales.
9. [ ] Añadir pruebas con eventos sintéticos, reloj falso y `TestBackend` para
       los tamaños, modos, recursos y composiciones exigidos.
10. [x] Ejecutar los controles del crate y la workspace y registrar resultados o
        correcciones documentales.

## Verificación

- [ ] Se cumplen los
      [criterios automáticos del frontend](../../tecnica/componentes/09-frontend-terminal.md#11-pruebas-y-criterios-de-aceptación)
      que no requieren una terminal física.
- [ ] `desert-tiger-terminal` depende públicamente solo de la fachada de app y no
      ejecuta reglas ni inspecciona Características.
- [ ] Los recursos inválidos impiden entrar en modo raw y todo camino posterior
      restaura cursor, pantalla y modo de entrada.
- [ ] En `80 × 24` se conserva la composición mínima sin escribir fuera del
      búfer; los tamaños menores muestran únicamente el aviso aprobado.
- [ ] Todos los tipos visuales del contenido oficial tienen una apariencia
      explícita y ninguno necesita usar la apariencia de reserva `?`.
- [ ] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- El aspecto exacto de los recursos mínimos es provisional; su función es hacer
  comprobable el contrato visual sin convertir el plan en diseño artístico.
- La restauración de terminal debe probarse también en el plan final sobre
  Manjaro porque un backend en memoria no reproduce todos los fallos físicos.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: frontend Ratatui funcional con mapa, rótulos, retratos animados,
  selección, viñetas, decisiones, barras, confirmación y aviso de tamaño.
- Verificaciones ejecutadas: `TestBackend` en 80 × 24 y 79 × 24, recursos
  oficiales, recorrido reproducible y prueba PTY de inicio/salida/restauración.
- Desviaciones respecto del plan: la primera versión ajusta texto con el widget
  de Ratatui pero aún no implementa la paginación local normativa ni toda la
  matriz de composición; por ello el plan permanece abierto.
- Trabajo posterior: [PLAN-2026-007](./2026-10-15-07-integrar-recorrido-jugable.md).
