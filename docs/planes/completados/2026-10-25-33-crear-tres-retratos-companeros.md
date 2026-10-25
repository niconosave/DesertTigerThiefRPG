# Plan: Crear tres retratos y mostrarlos en el grupo

| Campo | Valor |
|---|---|
| ID | PLAN-2026-033 |
| Estado | Completado |
| Creado | 2026-10-25 |
| Actualizado | 2026-10-25 |

## Resultado y referencias

Tres Actores nuevos solicitados por el usuario, dibujados siguiendo el estilo
del príncipe y añadidos al Grupo inicial para evaluarlos jugando. La intención
visual está en el [escenario inicial](../../contenido/escenario-inicial.md#4-actores);
el formato sigue el [contrato de contenido](../../tecnica/componentes/03-contenido.md).
No incluye habilidades nuevas, reglas, biografías ni modificaciones al frontend.

## Pasos y verificaciones

1. [x] Crear el Sirviente: ropa del príncipe, sin turbante y nariz más pequeña.
2. [x] Crear el Mercader: silueta ancha, ropa colorida y bigote.
3. [x] Crear el Niño ladrón: figura más baja con fez y chaleco, inspirada en Aladino.
4. [x] Añadir a cada RON una paleta, transparencia y dos fotogramas de parpadeo.
5. [x] Registrar los tres Actores en el mundo y el Grupo, en Hamada Roja.
6. [x] Actualizar la prueba de carga oficial para seis Actores y cuatro compañeros.
7. [x] Comprobar el conjunto en terminal, ejecutar controles y actualizar el brief.

## Resultado de la implementación

- Finalizado: 2026-10-25. Cada Actor reúne sus datos y retrato en un solo RON.
- Los retratos nuevos miden 16, 20 y 12 columnas, respectivamente, y 14 filas;
  el niño deja más filas vacías arriba. Con los retratos existentes y su
  solapamiento del 25 %, el conjunto ocupa 79 columnas.
- Los actores nuevos tienen Características y Relaciones vacías y política
  Ninguna. Son compañeros reales: el motor existente los incluye en el viaje.
- Pasaron los cinco controles generales: formato, check, Clippy sin advertencias,
  las 70 pruebas del workspace y compilación de producción. Se revalidó el
  contenido tras limpiar las entradas de paleta no utilizadas.
- Se inspeccionó una representación de los retratos y el programa en PTY de
  80 × 24: los cinco retratos aparecen, sus ojos parpadean y la salida es normal.
- Se cumplieron los controles y la definición de terminado de la metodología.
- El ajuste artístico posterior depende de la valoración del usuario al jugar.
