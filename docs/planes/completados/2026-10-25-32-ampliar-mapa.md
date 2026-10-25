# Plan: Ampliar el mapa al doble

| Campo | Valor |
|---|---|
| ID | PLAN-2026-032 |
| Estado | Completado |
| Creado | 2026-10-25 |
| Actualizado | 2026-10-25 |

## Resultado buscado y referencias

Duplicar ancho y alto del mapa y adaptar los centros y rótulos de sus Lugares,
según el [atlas regional](../../contenido/atlas-regional.md#1-propósito).
La solicitud confirmó expresamente el doble de tamaño. No cambia el esquema,
las Relaciones ni las reglas de cámara o viaje.

## Pasos y verificación

1. [x] Ampliar la matriz de 96 × 32 a 192 × 64: cada tile original ocupa 2 × 2.
2. [x] Duplicar ambos componentes de los 26 centros; trasladar los rótulos
       conservando sus recorridos y letras contiguas.
3. [x] Validar dimensiones, correspondencia de tiles, límites y centros.
4. [x] Actualizar las dimensiones esperadas en la prueba del contenido oficial.
5. [x] Ejecutar los controles y comprobar enfoque de cámara en terminal.
6. [x] Actualizar el atlas y registrar los resultados.

## Resultado de la implementación

- Finalizado: 2026-10-25.
- Los 26 centros se multiplicaron exactamente por dos; se conserva por tanto
  la dirección relativa usada provisionalmente por navegación.
- Los rótulos se trasladaron como conjuntos compactos. Espinazo Gris se ajustó
  una celda adicional a la derecha para evitar ocupar su coordenada central.
- Pasaron `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
  `cargo clippy --workspace --all-targets -- -D warnings`, las 70 pruebas de
  `cargo test --workspace --all-targets` y `cargo build --workspace --release`.
- Revisión manual: inicio y selección de destinos en PTY de 80 × 24; se comprobó
  el desplazamiento de cámara y la salida normal del programa.
- Se cumplieron los controles y la definición de terminado de la metodología.
- Sin tareas pendientes de este arreglo; la valoración estética final corresponde
  al usuario al jugar con el nuevo tamaño.
