# Plan: escuchar el crepitar de la fogata al acampar

| Campo | Valor |
|---|---|
| ID | PLAN-2026-041 |
| Estado | Completado |
| Creado | 2026-10-26 |
| Actualizado | 2026-10-26 |

## Resultado buscado

Al acampar se escucha una fogata procedural que continúa mientras se exploran
las fichas y se desvanece al abandonar el campamento. El juego sigue siendo
jugable sin dispositivo de audio, con el sonido deshabilitado o compilado sin
las dependencias sonoras.

## Referencias

- [Organización y dependencias](../../tecnica/componentes/17-audio-procedural.md#2-organización-y-dependencias).
- [Opciones y preparación](../../tecnica/componentes/17-audio-procedural.md#3-opciones-y-preparación).
- [Fachada e integración con el campamento](../../tecnica/componentes/17-audio-procedural.md#4-fachada-y-ambiente-deseado).
- [Dispositivo y estados](../../tecnica/componentes/17-audio-procedural.md#5-dispositivo-y-estados).
- [Transiciones y callback](../../tecnica/componentes/17-audio-procedural.md#6-tiempo-transiciones-y-callback).
- [Errores y diagnóstico](../../tecnica/componentes/17-audio-procedural.md#7-errores-y-diagnóstico).
- [Recurso editable](../../tecnica/componentes/18-crepitar-fogata.md#2-recurso-editable).
- [Síntesis del crepitar](../../tecnica/componentes/18-crepitar-fogata.md#3-generación-del-sonido).
- [API y estado del generador](../../tecnica/componentes/18-crepitar-fogata.md#4-api-y-estado).
- [Verificación de integración](../../tecnica/componentes/17-audio-procedural.md#8-verificación-y-aceptación)
  y [del sonido](../../tecnica/componentes/18-crepitar-fogata.md#5-verificación-y-aceptación).

## Alcance

### Incluido

- Dependencias opcionales, configuración RON y opción de inicio sin audio.
- Generador FunDSP, salida CPAL, transiciones y cierre seguro.
- Integración con Acampar, diagnóstico recuperable, pruebas y revisión auditiva.

### No incluido

- Música, otros efectos, selector de dispositivo o menú de audio.
- Cambios de reglas, tiempo de juego, retratos o animación visual de la fogata.

## Precondiciones

- Contratos 17 y 18 aprobados para implementar.
- Campamento y fogata visual funcionando en el estado actual del proyecto.
- Conservar los cambios ajenos presentes en el árbol de trabajo.

## Pasos

1. [x] Comprobar el entorno de compilación e incorporar las dependencias y la
       feature opcional conforme al [contrato 17, sección 2](../../tecnica/componentes/17-audio-procedural.md#2-organización-y-dependencias).
       Actualizar `Cargo.lock` y verificar que `app` y `core` conservan sus
       dependencias actuales.
2. [x] Crear la configuración validada y `assets/audio/fogata.ron` según el
       [contrato 18, sección 2](../../tecnica/componentes/18-crepitar-fogata.md#2-recurso-editable).
       Cubrir campos inválidos, diagnóstico contextual y carga del recurso
       oficial sin fijar sus valores artísticos en las pruebas.
3. [x] Incorporar `--no-audio` y la preparación previa al modo interactivo según
       el [contrato 17, sección 3](../../tecnica/componentes/17-audio-procedural.md#3-opciones-y-preparación).
       Probar combinaciones con argumentos existentes y omisión de recursos
       sonoros y dispositivo cuando el audio está deshabilitado.
4. [x] Implementar y probar offline el generador FunDSP conforme al
       [contrato 18, secciones 3–5](../../tecnica/componentes/18-crepitar-fogata.md#3-generación-del-sonido),
       incluyendo las familias de sonido, voces acotadas, azar propio y límites
       de muestras, tasas y contadores.
5. [x] Implementar el motor de transiciones y su comunicación atómica conforme
       al [contrato 17, sección 6](../../tecnica/componentes/17-audio-procedural.md#6-tiempo-transiciones-y-callback).
       Verificar rampas, silencio exacto, reentrada, independencia de buffers y
       cierre con confirmación y espera acotada.
6. [x] Implementar el adaptador CPAL y la fachada silenciosa según el
       [contrato 17, secciones 4–7](../../tecnica/componentes/17-audio-procedural.md#4-fachada-y-ambiente-deseado).
       Probar selección de salida y fallos mediante dobles; revisar que el
       callback no reserva memoria, bloquea ni accede a la sesión.
7. [x] Integrar el control con el bucle y los avisos con la barra contextual
       según el [contrato 17, secciones 4 y 7](../../tecnica/componentes/17-audio-procedural.md#4-fachada-y-ambiente-deseado).
       Cubrir navegación, confirmación de salida, cierre rechazado y aceptado,
       tamaño insuficiente, fallos y restauración de terminal.
8. [x] Completar las verificaciones automáticas de ambos contratos, incluida
       la equivalencia del estado de juego con y sin audio; ejecutar los
       controles de calidad y validar contenido y recursos oficiales.
9. [x] Realizar la revisión manual del incremento completo conforme a los
       [criterios de integración](../../tecnica/componentes/17-audio-procedural.md#8-verificación-y-aceptación)
       y [escucha](../../tecnica/componentes/18-crepitar-fogata.md#5-verificación-y-aceptación).
       Ajustar el recurso artístico y registrar cualquier limitación de prueba.
10. [x] Actualizar los contratos, las instrucciones de ejecución y los índices
        para describir el resultado real. Completar las verificaciones y
        desviaciones de este archivo y moverlo a `../completados/` cuando se
        cumpla la definición de terminado.

## Verificación

- [x] Pasan los casos automáticos de aceptación de los contratos 17 y 18; las pruebas
      automáticas no abren un dispositivo real.
- [x] Pasan los controles generales, con features predeterminadas y sin audio:

  ```sh
  cargo fmt --all -- --check
  cargo check --workspace --all-targets
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace --all-targets
  cargo check --workspace --all-targets --no-default-features
  cargo clippy --workspace --all-targets --no-default-features -- -D warnings
  cargo test --workspace --all-targets --no-default-features
  cargo build --release --bin desert-tiger-terminal
  cargo build --release --bin desert-tiger-terminal --no-default-features
  ```

- [x] El contenido y los recursos oficiales cargan y validan; el nuevo recurso
      sonoro se comprueba también en sus pruebas de carga.
- [x] El usuario escuchó la fogata jugando y aceptó el resultado. Los casos
      de navegación y presentación se comprobaron en las pruebas registradas.
- [x] Se comprueba el inicio con `--no-audio` y con la compilación sin audio.
      La revisión con sonido se realiza sobre una compilación que lo incluya.
- [x] La salida no disponible conserva el juego utilizable y muestra el aviso
      previsto; se registra si se pudo comprobar con un dispositivo real además
      del adaptador simulado.
- [x] Se verifica el cierre normal y por error, con restauración de terminal.
- [x] Se cumple la [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- La compilación con audio puede requerir dependencias de sistema; se comprueba
  el entorno antes de integrar la salida y se registra cualquier instalación
  necesaria según el contrato 17.
- La corrección de las muestras no prueba que el fuego suene convincente. Se
  agrupan las verificaciones automáticas y se realiza la escucha al finalizar
  la integración, sin detener el desarrollo por cada variante pequeña.
- Si el entorno no permite escuchar el efecto, se registra la verificación
  pendiente y el plan permanece aquí hasta completarla.
- Cualquier decisión técnica nueva se resuelve en el contrato correspondiente;
  este plan solo actualiza sus referencias y el orden operativo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-26; desarrollo y pruebas funcionales completados
  y revisión auditiva aceptada por el usuario.
- Resultado: audio opcional integrado con FunDSP y CPAL, recurso RON editable,
  crepitar durante Acampar, transiciones y diagnóstico recuperable.
- Verificaciones ejecutadas: pasan fmt, check, clippy sin advertencias, pruebas
  de workspace y compilaciones release con y sin audio. Pasan 146 pruebas con
  audio y 135 sin audio; contenido, Lua y recursos oficiales incluidos. El
  árbol sin features excluye FunDSP, CPAL y ALSA.
- Recorridos del ejecutable: cuatro escenarios PTY verificaron captura de CPAL,
  audio deshabilitado, salida incompatible y compilación sin audio. Todos
  finalizaron con código cero y terminal restaurada. La captura ALSA file/null
  contiene 2427496 muestras, de las que 1761752 no son cero: todas son finitas,
  acotadas y duplicadas correctamente en estéreo. Un quinto escenario provocó
  un fatal de calendario al cerrar Acampar y verificó código uno y restauración
  de terminal, con salida de audio activa. No se usó un dispositivo físico.
- Desviaciones respecto del plan: ninguna de alcance. La revisión auditiva quedó
  inicialmente pendiente y el usuario la completó al confirmar que el efecto
  funcionó muy bien, antes de solicitar los nuevos paisajes sonoros.
- Trabajo posterior: los nuevos ambientes pertenecen al
  [plan 042](./2026-10-26-42-incorporar-paisajes-sonoros.md).
