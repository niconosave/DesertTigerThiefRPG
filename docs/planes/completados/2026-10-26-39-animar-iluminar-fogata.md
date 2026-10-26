# Plan: animar la fogata e iluminar los retratos del campamento

| Campo | Valor |
|---|---|
| ID | PLAN-2026-039 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

El campamento presenta una fogata con llama procedural, chispas y luz cálida
sobre los retratos cercanos. La navegación sigue funcionando y las animaciones
existentes conservan su velocidad.

## Referencias

- [Contrato 15: alcance y módulos](../../tecnica/componentes/15-fogata-procedural.md#1-propósito-y-alcance).
- [Tiempo visual y compatibilidad](../../tecnica/componentes/15-fogata-procedural.md#3-tiempo-visual).
- [Recurso editable](../../tecnica/componentes/15-fogata-procedural.md#4-recurso-y-compatibilidad).
- [Generación de llama y chispas](../../tecnica/componentes/15-fogata-procedural.md#5-generación-de-la-imagen).
- [Iluminación y composición](../../tecnica/componentes/15-fogata-procedural.md#6-iluminación-y-composición).
- [API, coste y errores](../../tecnica/componentes/15-fogata-procedural.md#7-api-interna-y-límites-de-coste).
- [Verificación y aceptación](../../tecnica/componentes/15-fogata-procedural.md#9-verificación-y-aceptación).
- [Acampar implementado](../../tecnica/acciones/07-acampar.md#12-verificación-de-la-implementación--2026-10-26).
- [Metodología y controles](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- Reloj visual, recurso opcional, efecto procedural y composición del campamento.
- Regresión de tiles, parpadeos, controles, fichas y aislamiento de la Partida.
- Revisión visual agrupada y actualización de los contratos afectados.

### No incluido

- Cambios de reglas, contenido de Actores, API de aplicación o Lua.
- Otros efectos, fuentes de luz, menú gráfico o diagnóstico de terminal.

## Precondiciones

- El contrato 15 está aprobado para implementar.
- El campamento actual funciona y su recurso estático carga correctamente.
- Se conservan los cambios ajenos presentes en el árbol de trabajo.

## Pasos

1. [x] Crear el reloj de la sección 3 y sus pruebas de fracciones, vencimientos,
       pausa y demoras. Separar la unidad de animación existente del intervalo
       fino y derivar ambos valores del mismo reloj.
2. [x] Integrarlo en el bucle y en Interfaz; adaptar los consumidores y pruebas
       del contador anterior. Comprobar la regresión temporal de tiles y ojos.
3. [x] Extender la carga del recurso según la sección 4. Cubrir compatibilidad
       estática, rangos y diagnósticos; incorporar `alto_fogata()`.
4. [x] Implementar en el módulo de fogata el muestreo reproducible de llama,
       brasas, leños y chispas de la sección 5, con coste acotado.
5. [x] Implementar conversión local de color, caída y mezcla de la sección 6;
       probar preservación de símbolos, negro, estilos fuera del halo y rangos.
6. [x] Integrar efecto y recorte en campamento según las secciones 6–7, manteniendo
       el camino estático. Activar la configuración en el recurso oficial y
       usar la espera fina únicamente cuando corresponda.
7. [x] Añadir las pruebas de composición, redibujado, pausa, resize y aislamiento
       de la sección 9; comprobar navegación y salida sin efectos de juego extra.
8. [x] Ejecutar controles generales, carga de todo el contenido y recursos
       oficiales y compilación de producción. Resolver los fallos encontrados.
9. [x] Realizar una revisión manual conjunta del campamento animado y estático,
       navegación, contraste, fluidez, resize, pausa y restauración de terminal.
       Ajustar el arte dentro del contrato y repetir las pruebas afectadas.
10. [x] Actualizar el contrato 15, los enlaces de precedencia de 09, 13 y Acampar,
        y sus índices para distinguir lo implementado de lo aplazado. Completar
        el resultado de este plan y moverlo a completados al cumplir el cierre.

## Verificación

- [x] Se cumplen los contratos y casos de la sección 9 del documento técnico.
- [x] La llama y chispas se animan; la luz varía y decrece con la distancia.
- [x] La ficha y las barras conservan su contenido, estilo y controles.
- [x] Los RON anteriores siguen siendo válidos y `None` conserva la fogata estática.
- [x] Tiles y parpadeos conservan sus duraciones; la pausa no acumula tiempo.
- [x] Los redibujados no alteran el estado ni el azar del juego.
- [x] Pasan `cargo fmt --all -- --check`,
      `cargo check --workspace --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings` y
      `cargo test --workspace --all-targets`.
- [x] Se valida todo el contenido y recursos oficiales mediante sus pruebas
      de carga y pasa `cargo build --workspace --release`.
- [x] Se registra la revisión manual del incremento completo.
- [x] Se cumplen los controles y la definición de terminado de la
      [metodología](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- La migración del reloj afecta consumidores existentes: sus pruebas se
  ejecutan antes de valorar el acabado del fuego.
- El aspecto depende de la fuente y del soporte RGB de la terminal; la revisión
  manual usa los dos caminos del recurso definidos en el contrato 15.
- El plan es un único incremento: no requiere detenerse para probar manualmente
  cada componente. Si surge una decisión técnica nueva se actualiza primero su
  fuente canónica, sin definirla dentro de estos pasos.

## Resultado de la implementación

- Fecha de finalización: 2026-10-26.
- Resultado: fogata procedural RGB con chispas y luz sobre retratos, recurso
  opcional compatible y reloj visual fino con duraciones anteriores conservadas.
- Verificaciones ejecutadas: pasan los cuatro controles de la metodología y
  `cargo build --workspace --release`; 115 pruebas, incluidas ocho nuevas de
  reloj, efecto, recurso e integración. Se validan contenido, Lua y recursos
  oficiales. El [informe técnico](../../tecnica/componentes/15-fogata-procedural.md#11-verificación-de-la-implementación--2026-10-26)
  registra casos y revisión del ejecutable release en PTY, con ambas variantes,
  navegación, pausa, resize, cierre y restauración (código 0). Se inspeccionaron
  capturas RGB con Noto Sans Mono.
- Desviaciones respecto del plan: ninguna de alcance. El DTO de canales RGB lee
  `u16` y convierte a `u8` con contexto para identificar valores como 256; los
  rangos aceptados son los definidos y el contrato refleja ese detalle.
- Trabajo posterior: valoración artística por el usuario; diagnóstico automático
  de terminal y demás extensiones permanecen aplazados según el contrato.
