# Plan: Integrar el primer recorrido jugable

| Campo | Valor |
|---|---|
| ID | PLAN-2026-007 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

El binario inicia localmente desde los archivos editables, permite completar un
`Viajar` con sus viñetas y decisiones, avanza el mundo, ejecuta al NPC y devuelve
el control al jugador en una terminal restaurable y verificadamente jugable.

## Referencias

- [Primer incremento jugable](../../diseno-general.md#9-primer-incremento-jugable).
- [Verificación de arquitectura](../../tecnica/diseno-tecnico-general.md#6-verificación-de-la-arquitectura).
- [Entradas, salidas y configuración](../../tecnica/componentes/01-workspace-entorno.md#6-entradas-salidas-y-configuración).
- [Alcance de sesión](../../tecnica/componentes/04-sesion-aplicacion.md#2-alcance-inicial).
- [Fixture vertical oficial](../../tecnica/componentes/03-contenido.md#fixture-vertical-oficial).
- [Pruebas del frontend](../../tecnica/componentes/09-frontend-terminal.md#11-pruebas-y-criterios-de-aceptación).
- [Definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).
- Plan previo: [PLAN-2026-006](./2026-10-15-06-implementar-frontend-terminal.md).

## Alcance

### Incluido

- composición final del binario, configuración `--content` e inicialización;
- integración de contenido, Partida, recursos, eventos y renderizado;
- prueba de humo automatizada del recorrido completo y semilla reproducible;
- controles de calidad, compilación de producción y prueba manual en Manjaro;
- correcciones de integración y regresiones dentro del diseño aprobado;
- cierre documental del primer incremento.

### No incluido

- reemplazar el fixture por el setting definitivo;
- acciones distintas de `Viajar`, guardado, victoria, derrota o NPC avanzados;
- refinamiento artístico posterior a comprobar la versión funcional.

## Precondiciones

- El alcance y las verificaciones acotadas de
  [PLAN-2026-006](./2026-10-15-06-implementar-frontend-terminal.md) están
  resueltos y registrados.
- Los seis planes anteriores continúan pendientes y registran sus verificaciones
  y desviaciones para poder cerrarse junto con este incremento jugable.

## Pasos

1. [ ] Implementar la composición de `main`: analizar configuración, cargar
       recursos y contenido, crear la Partida y entrar al bucle solo si todo
       tuvo éxito.
2. [ ] Conectar el adaptador visual exclusivamente con `VistaSesion`,
       `IntencionFrontend` y los errores públicos de app.
3. [ ] Añadir una prueba de humo con `TestBackend`, la fachada pública y una
       semilla explícita documentada que produzca una rama con decisión; recorrer
       inicio, selección, fragmentos, decisión, finalización, fase NPC y regreso
       a `VistaSesion::Disponible`.
4. [ ] Ejecutar dos veces ese recorrido con la misma semilla e intenciones y
       comprobar igualdad de las vistas observables, incluida la nueva ubicación
       del jugador, el resumen NPC y la disponibilidad de otra acción; los
       estados internos exactos del fixture permanecen cubiertos por
       PLAN-2026-005.
5. [ ] Ejecutar todos los controles normativos, validación del contenido oficial,
       árbol de dependencias y build de release.
6. [ ] Ejecutar manualmente en Manjaro el inicio predeterminado y con
       `--content`, controles, acentos, animación, redimensionado y salida
       normal; revisar la paleta con un tema ANSI oscuro habitual y otro de
       contraste reducido.
7. [ ] Provocar de forma controlada errores de inicio y de sesión para comprobar
       diagnóstico, código de salida y restauración de la terminal.
8. [ ] Corregir los defectos encontrados sin ampliar el alcance y añadir una
       prueba de regresión por cada corrección razonablemente automatizable.
9. [ ] Jugar nuevamente el recorrido completo y confirmar que termina en un
       estado válido donde el jugador puede elegir otra acción.
10. [ ] Actualizar el estado de los documentos implementados, completar los
        resultados de los planes y mover cada plan terminado a `completados/`
        conforme al flujo documentado.

## Verificación

- [ ] `cargo fmt`, `check`, `clippy`, `test`, build de release y controles de
      dependencias pasan sobre la workspace completa.
- [ ] El contenido y los tres recursos visuales incluidos validan; todos los
      tipos visuales oficiales tienen una apariencia explícita y ninguno usa la
      apariencia de reserva `?`.
- [ ] La prueba de humo demuestra el recorrido definido por el
      [diseño general](../../diseno-general.md#9-primer-incremento-jugable).
- [ ] La prueba manual confirma legibilidad, controles, ritmo, restauración y
      continuidad jugable en la terminal local.
- [ ] Se cumplieron todos los puntos de la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- Los defectos visuales menores se registrarán para refinamiento posterior; solo
  bloquean este plan si impiden comprender o completar el recorrido.
- Si la integración revela una decisión ausente, se actualizará primero su
  documento técnico y después este plan, sin resolverla únicamente en código.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: Pendiente.
- Verificaciones ejecutadas: Pendiente.
- Desviaciones respecto del plan: Pendiente.
- Trabajo posterior: ampliar y refinar el setting jugable cuando el recorrido
  técnico resulte satisfactorio.
