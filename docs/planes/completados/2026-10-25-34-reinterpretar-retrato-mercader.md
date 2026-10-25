# Plan: Reinterpretar el Mercader desde la referencia visual

| Campo | Valor |
|---|---|
| ID | PLAN-2026-034 |
| Estado | Completado |
| Creado | 2026-10-25 |
| Actualizado | 2026-10-25 |

## Alcance y referencia

Sustituir solo el retrato del Mercader por una traducción de la imagen aportada
por el usuario, conservando el estilo del ojo y el parpadeo. La intención visual
se registra en el [escenario inicial](../../contenido/escenario-inicial.md#4-actores).
Se conserva el formato del [contenido](../../tecnica/componentes/03-contenido.md),
su pertenencia al Grupo, las dimensiones del retrato y las reglas del juego.

## Pasos y resultado

1. [x] Dibujar perfil, pluma, turbante rojo y dorado, mejillas, bigote y vestuario.
2. [x] Codificar los detalles con medios bloques y paleta de frente y fondo.
3. [x] Conservar el ojo de dos celdas sobre negro en los dos fotogramas.
4. [x] Validar contenido, representación en terminal y controles generales.
5. [x] Actualizar el brief y archivar el resultado.

Finalizado el 2026-10-25. El retrato conserva 20 × 14 celdas y se editó en el
propio `mercader.ron`. No requiere nuevas dependencias ni cambios de renderizado.
Se inspeccionó una vista de trabajo y se comprobó el retrato y su parpadeo dentro
del grupo en una PTY de 80 × 24, con salida normal.

Pasaron formato, check, Clippy sin advertencias, las 70 pruebas del workspace y
la compilación de producción. Se cumplieron los controles y la definición de
terminado de la metodología. La valoración del parecido artístico queda para
la prueba del usuario; no se presenta como ya aprobada.
