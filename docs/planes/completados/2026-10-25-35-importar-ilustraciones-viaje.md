# Plan: Importar las cuatro ilustraciones de viaje

| Campo | Valor |
|---|---|
| ID | PLAN-2026-035 |
| Estado | Completado |
| Creado | 2026-10-25 |
| Actualizado | 2026-10-25 |

## Alcance

Reemplazar los cuatro dibujos de `assets/terminal/scenes.ron` por los archivos
aportados por el usuario. El [contrato de frontend](../../tecnica/componentes/09-frontend-terminal.md)
admite ahora glifos Unicode de una columna en ilustraciones, manteniendo un solo
estilo de color y dibujos estáticos. No cambia ventanas, paginación ni gameplay.

## Pasos y resultado

1. [x] Asociar `travel.ascii`, `obstaculo.ascii`, `choice.ascii` y `finish.ascii`
       con marcha, obstáculo, rumbo incierto y llegada, respectivamente.
2. [x] Conservar los dibujos, completar filas con espacios y quitar líneas vacías
       sobrantes al final; mantener la línea inicial del dibujo de marcha.
3. [x] Validar anchura por caracteres de una columna, en lugar de bytes ASCII;
       cubrir bloques, anchura doble, combinantes y controles con una prueba.
4. [x] Ejecutar controles y comprobar una viñeta en terminal.

Finalizado el 2026-10-25. Los dibujos ocupan cuatro filas y entre 20 y 24 columnas.
Los originales permanecen intactos; el recurso que carga el juego es `scenes.ron`.
Pasaron formato, check, Clippy sin advertencias, las 71 pruebas del workspace y
el build de producción. La comprobación manual en PTY de 80 × 24 mostró la
ilustración de decisión alineada dentro de la ventana; la salida fue normal.
Se cumplieron los controles y la definición de terminado de la metodología.
