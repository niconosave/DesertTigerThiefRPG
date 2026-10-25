# Plan: Incorporar retratos coloreados desde Durdraw

| Campo | Valor |
|---|---|
| ID | PLAN-2026-008 |
| Estado | En verificación manual |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

El retrato del Príncipe Perdido diseñado en Durdraw aparece en el juego con sus
dos fotogramas, glifos de bloque, colores y transparencia, mientras el RON del
Actor continúa siendo la única fuente cargada durante una Partida.

## Referencias

- [Esquema de Actor y retrato](../../tecnica/componentes/03-contenido.md#5-formato-y-organización).
- [Vista neutral de Actor](../../tecnica/componentes/04-sesion-aplicacion.md#7-vistas-y-salidas).
- [Composición de retratos](../../tecnica/componentes/09-frontend-terminal.md#7-composición-de-la-pantalla).
- [Bucle y animación](../../tecnica/componentes/09-frontend-terminal.md#8-bucle-animación-y-restauración).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Alcance

### Incluido

- glifos Unicode de exactamente una columna en retratos;
- paleta neutral ANSI de dieciséis colores y estilo opcional por celda;
- matrices compactas y paralelas de arte y claves de estilo dentro del RON;
- importador determinista para `.dur` versión 8, gzip, UTF-8 y 16 colores;
- conversión diferenciada de colores frontales y fondos, usando el fondo negro
  predeterminado de Durdraw como transparencia de retrato;
- conversión de `assets/principe.dur` y representación de sus dos fotogramas;
- compatibilidad monocroma para el retrato existente del Bandido;
- pruebas de carga, validación, conversión y renderizado.

### No incluido

- cargar `.dur` durante una Partida o referenciarlo desde un Actor;
- duración individual de fotogramas o adopción de los 12 FPS del editor;
- Unicode de ancho cero o doble, RGB, 256 colores o modificadores ANSI;
- editor integrado, recorte automático del lienzo o color para tiles y viñetas.

## Precondiciones

- El archivo `assets/principe.dur` contiene los dos fotogramas aprobados de
  `24 × 14` y utiliza el modo de 16 colores.
- Los contratos técnicos 03, 04 y 09 describen el nuevo formato antes de
  modificar código o contenido.

## Pasos

1. [x] Ampliar los DTO, valores neutrales y validaciones de retrato en
       `desert-tiger-app`.
2. [x] Adaptar el renderizador de Ratatui para glifos y estilos por celda sin
       alterar transparencia, capas, solapamiento ni cadencia.
3. [x] Implementar y probar `importar-retrato-durdraw` dentro del paquete de
       terminal.
4. [x] Convertir el retrato del Príncipe al RON y migrar el retrato monocromo del
       Bandido al esquema explícito.
5. [x] Cubrir matrices inválidas, glifos de ancho no admitido, paletas faltantes,
       conversión reproducible y render de ambos fotogramas.
6. [x] Corregir la tabla de fondos Durdraw, regenerar las paletas oficiales y
       cubrir la transparencia negra y los fondos coloreados con regresiones.
7. [ ] Ejecutar controles automáticos, build de producción y prueba manual del
       parpadeo sobre el mapa.
8. [ ] Sincronizar documentación, registrar resultados y mover el plan cuando
       la prueba manual confirme el acabado.

## Verificación

- [x] El importador convierte dos veces `assets/principe.dur` en bytes RON
      idénticos y el fragmento producido valida al incorporarse al Actor.
- [x] El contenido oficial carga dos fotogramas de `24 × 14`; el ojo cambia y el
      resto de dimensiones permanece estable.
- [x] Ratatui representa los colores de primer plano y fondo esperados, omite
      espacios y conserva el mapa debajo de las celdas transparentes.
- [x] `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace --all-targets` y el build de release pasan.
- [ ] La prueba manual confirma legibilidad, color, parpadeo y solapamiento en
      Manjaro.
- [ ] Se cumplieron los controles y la definición de terminado de
      `docs/metodologia-desarrollo.md`.

## Riesgos y decisiones operativas

- La apariencia exacta de los dieciséis colores depende de la paleta de la
  terminal. Se conserva la semántica ANSI de Durdraw y se revisa manualmente en
  el mismo entorno usado para jugar.
- El padding transparente de `24 × 14` se conserva deliberadamente en este
  incremento. Cualquier recorte será una decisión artística posterior.
- Si Durdraw produce una variante fuera del subconjunto documentado, el
  importador la rechazará con contexto en vez de degradarla silenciosamente.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente de prueba manual.
- Resultado: la carga, la animación y la transparencia de espacios funcionan.
  La tabla de fondos se corrigió, el negro predeterminado de Durdraw ahora se
  importa como transparencia y los fondos coloreados conservan su color real;
  falta confirmar el resultado visual.
- Verificaciones ejecutadas: formato, compilación de todos los targets, Clippy
  sin advertencias, 47 pruebas automáticas y build de release.
- Desviaciones respecto del plan: Durdraw usa tablas diferentes para frente y
  fondo en modo de 16 colores; el contrato inicial del importador no reflejó esa
  diferencia.
- Trabajo posterior: repetir la verificación visual antes de mover el plan a
  `completados/`.
