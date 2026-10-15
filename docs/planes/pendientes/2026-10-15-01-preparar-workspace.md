# Plan: Preparar la workspace reproducible

| Campo | Valor |
|---|---|
| ID | PLAN-2026-001 |
| Estado | Pendiente |
| Creado | 2026-10-15 |
| Actualizado | 2026-10-15 |

## Resultado buscado

La workspace Rust existe con los tres crates aprobados, un único binario, el
toolchain y las dependencias fijadas, y supera los controles básicos sin haber
introducido todavía reglas de juego.

## Referencias

- [Arquitectura del programa](../../tecnica/diseno-tecnico-general.md#2-arquitectura-del-programa).
- [Estructura de la workspace](../../tecnica/componentes/01-workspace-entorno.md#2-estructura-de-la-workspace).
- [Dependencias directas aprobadas](../../tecnica/componentes/01-workspace-entorno.md#dependencias-directas-aprobadas).
- [Crates y módulos de responsabilidad](../../tecnica/componentes/01-workspace-entorno.md#4-crates-y-módulos-de-responsabilidad).
- [Dirección de dependencias](../../tecnica/componentes/01-workspace-entorno.md#5-dirección-de-dependencias).
- [Controles automáticos](../../metodologia-desarrollo.md#7-controles-automáticos).

## Alcance

### Incluido

- preparación del toolchain estable fijado por el diseño;
- manifiesto raíz, lockfile, lints y perfiles de la workspace;
- creación de `desert-tiger-core`, `desert-tiger-app` y
  `desert-tiger-terminal`;
- grafo mínimo `terminal -> app -> core` y un solo objetivo binario;
- estructura mínima compilable y controles de calidad iniciales.

### No incluido

- tipos de dominio, acciones, carga de contenido, Lua o interfaz Ratatui;
- instalación implícita de herramientas sin la autorización necesaria;
- dependencias distintas de las ya aprobadas.

## Precondiciones

- La documentación técnica enlazada está aprobada para implementar.
- Al ejecutar el plan debe existir acceso autorizado para instalar el toolchain
  si continúa ausente en la máquina.

## Pasos

1. [ ] Comprobar `rustup`, `rustc`, `cargo`, compilador C y `pkg-config`, y
       registrar cualquier diferencia respecto del entorno documentado.
2. [ ] Instalar mediante `rustup`, si corresponde y con autorización, el
       toolchain y los componentes fijados.
3. [ ] Crear `rust-toolchain.toml` y el `Cargo.toml` raíz con miembros,
       metadatos, resolver, lints y dependencias exactamente aprobados.
4. [ ] Crear los tres paquetes en `crates/`, dejando `core` y `app` como
       bibliotecas y `terminal` como el único binario.
5. [ ] Declarar solamente las dependencias permitidas para cada crate y
       comprobar que no exista un enlace directo de `terminal` a `core`.
6. [ ] Añadir el código mínimo necesario para que todos los targets compilen sin
       advertencias y generar `Cargo.lock` sin implementar comportamiento futuro.
7. [ ] Ejecutar los controles de workspace, compilación de producción y árbol de
       dependencias definidos por el documento técnico.
8. [ ] Registrar resultados y desviaciones en este plan y sincronizar la
       documentación si el entorno real obliga a cambiar una decisión aprobada.

## Verificación

- [ ] Cargo reconoce exactamente los tres miembros y produce un solo binario.
- [ ] `cargo tree -p desert-tiger-core` no contiene dependencias de app,
      terminal, Lua, RON, Serde, `rand`, `rand_chacha` ni interfaz.
- [ ] `cargo tree --duplicates` no contiene dos versiones de Crossterm.
- [ ] Pasan formato, check, Clippy, pruebas vacías o mínimas y build de release
      conforme a los comandos de
      [aceptación de workspace](../../tecnica/componentes/01-workspace-entorno.md#9-pruebas-y-criterios-de-aceptación).
- [ ] Se cumplieron los controles y la
      [definición de terminado](../../metodologia-desarrollo.md#8-definición-de-terminado).

## Riesgos y decisiones operativas

- La instalación inicial y la descarga de crates requieren red; se solicitará
  autorización al ejecutar esas operaciones y no se sustituirán versiones para
  evitarla.
- Si una versión fijada ya no pudiera obtenerse, se detendrá el plan y se
  actualizará primero el documento técnico correspondiente.

Esta sección no puede crear reglas de juego ni decisiones técnicas. Si durante
la ejecución aparece una, se actualiza y aprueba primero su documento canónico;
el plan solo registra el enlace y el efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: Pendiente.
- Resultado: Pendiente.
- Verificaciones ejecutadas: Pendiente.
- Desviaciones respecto del plan: Pendiente.
- Trabajo posterior: [PLAN-2026-002](./2026-10-15-02-implementar-modelo-mundo-calendario.md).
