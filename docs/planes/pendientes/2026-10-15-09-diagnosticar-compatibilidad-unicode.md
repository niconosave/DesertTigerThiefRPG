# Plan: Diagnosticar compatibilidad Unicode de la terminal

| Campo | Valor |
|---|---|
| ID | PLAN-2026-009 |
| Estado | Pendiente de diseño técnico |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

El jugador puede abrir un diagnóstico de terminal que presenta el repertorio
Unicode admitido por el juego mediante una composición verificable y explica
cómo reconocer glifos ausentes, sustituidos o dibujados con un ancho
incompatible, sin crear ni modificar una Partida.

## Referencias

- [Formato y validación de contenido](../../tecnica/componentes/03-contenido.md#5-formato-y-organización).
- [Responsabilidades del frontend](../../tecnica/componentes/09-frontend-terminal.md#4-responsabilidades-internas).
- [Composición de la pantalla](../../tecnica/componentes/09-frontend-terminal.md#7-composición-de-la-pantalla).
- [Entrada y accesibilidad](../../tecnica/componentes/09-frontend-terminal.md#9-entrada-y-accesibilidad-básica).
- [Pruebas del frontend](../../tecnica/componentes/09-frontend-terminal.md#11-pruebas-y-criterios-de-aceptación).
- [Decisión aplazada](../../tecnica/componentes/09-frontend-terminal.md#13-decisiones-aplazadas).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Alcance

### Incluido

- diseñar y aprobar primero el contrato técnico del diagnóstico;
- implementar el acceso, estado visual, composición e instrucciones aprobados;
- mostrar los glifos admitidos junto con una referencia geométrica que ayude a
  detectar sustituciones o desplazamientos;
- mantener el diagnóstico aislado de las reglas y el estado de la Partida;
- pruebas automáticas de composición, navegación y restauración de terminal;
- prueba manual con configuraciones de fuente diferentes;
- instrucciones breves para interpretar el resultado.

### No incluido

- instalar, descargar o cambiar la fuente del jugador;
- afirmar que la presencia de un glifo puede detectarse automáticamente con
  certeza;
- admitir caracteres de ancho cero o doble;
- diseñar en este plan un modo ASCII alternativo;
- cambiar reglas del juego;
- diagnosticar caracteres arbitrarios ajenos al repertorio acordado.

## Precondiciones

- Diseñar y aprobar en `09-frontend-terminal.md` el acceso, contenido,
  navegación, salida y criterios de aceptación de la pantalla.
- Decidir allí si el repertorio es oficial y fijo o se obtiene del contenido
  cargado; en el segundo caso, actualizar también `03-contenido.md`.
- Mantener, salvo un nuevo diseño aprobado, el contrato vigente de aceptar solo
  glifos para los que `UnicodeWidthChar::width(c) == Some(1)`.

## Pasos

1. [ ] Resolver las decisiones de las precondiciones y aprobar el contrato
       técnico correspondiente.
2. [ ] Implementar la obtención determinista del repertorio que establezca ese
       contrato.
3. [ ] Implementar el modo o recorrido de diagnóstico sin añadir dependencias a
       `desert-tiger-app` ni `desert-tiger-core`.
4. [ ] Dibujar la muestra, las referencias geométricas y las instrucciones
       aprobadas.
5. [ ] Cubrir generación, entrada, tamaños admitidos, restauración y renderizado
       con pruebas automáticas.
6. [ ] Ejecutar los controles generales y comparar manualmente configuraciones
       de fuente diferentes.
7. [ ] Sincronizar la documentación, registrar el resultado y mover el plan.

## Verificación

- [ ] El diagnóstico muestra exactamente el repertorio definido por el contrato,
      sin controles ni caracteres de ancho no admitido.
- [ ] La referencia visual permite reconocer una celda ausente o desplazada.
- [ ] Entrar y salir no crea ni modifica una Partida.
- [ ] `TestBackend` verifica composición y navegación en los tamaños acordados.
- [ ] La terminal se restaura correctamente al salir y ante un error controlado.
- [ ] Una prueba manual confirma el resultado con una fuente compatible y, si
      está disponible, otra configuración que revele al menos una limitación.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- Ratatui puede escribir un glifo, pero no determinar con certeza si la fuente
  lo contiene o si intervino una fuente de reserva.
- `unicode-width` expresa el ancho esperado; no garantiza que toda combinación
  de terminal y fuente lo represente de igual manera.
- Mantener dos listas manuales de glifos produciría divergencias. El diseño debe
  establecer una única fuente del repertorio.
- Antes de implementar se debe decidir cómo se abre el diagnóstico, qué
  repertorio inspecciona, si solicita confirmación al jugador, qué referencia
  visual utiliza y qué recomienda ante una incompatibilidad. Estas decisiones
  pertenecen al documento técnico, no a este plan.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: Pendiente.
- Verificaciones ejecutadas: Pendiente.
- Desviaciones respecto del plan: Pendiente.
- Trabajo posterior: Pendiente.
