# Plan: Escenarios traducidos y primera puerta interactiva

| Campo | Valor |
|---|---|
| ID | PLAN-2026-054 |
| Estado | Completado técnicamente; integración de etapa pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-24 |

## Resultado buscado

Producir los siete perfiles actuales sobre la nueva base, con una puerta
operable en Palacio y un caso pequeño de dos alturas conectadas.

Este plan es un bloque interno de una única etapa jugable. Su terminación técnica
no exige que el juego funcione durante cada paso ni constituye una entrega final.
La revisión integrada y el cierre conjunto corresponden a PLAN-2026-057.

## Referencias

- [Volumen y elementos](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#4-volumen-y-elementos).
- [Consultas y contexto](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#5-consultas-y-contexto).
- [Generación y traducción de escenarios](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#6-generación-y-traducción-de-escenarios).
- [Comportamientos y resolución](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#8-comportamientos-y-resolución).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).
- [PLAN-2026-052](./2026-11-24-52-modelo-espacial-tridimensional.md).
- [PLAN-2026-053](./2026-11-24-53-planes-y-comportamientos.md).

## Alcance

### Incluido

- Traducción de Recinto, Campamento, Palacio, Ciudad, Bosque, Montaña y Playa.
- Identificación de espacios, accesos y salidas; conservación de referencias narrativas útiles.
- Puerta manual de Palacio y comportamiento que permite cruzarla correctamente.
- Muestra técnica de dos alturas sin exigir un mapa persistente o generación modular.

### No incluido

- Conservar exactamente la distribución o las interfaces del generador viejo.
- Cerraduras, llaves, daño de puertas, incendios, salto y escalada.

## Precondiciones

- PLAN-2026-052 y PLAN-2026-053 completados técnicamente.
- Reglas de puerta y generación del contrato listas para implementar.

## Pasos

1. [x] Separar las ocho recetas volumétricas en generacion.rs y validar su disposición.
2. [x] Comprobar recorridos entre participantes y limitar a ocho los intentos de generación.
3. [x] Distinguir tres salones en Palacio y sus conexiones, con puerta manual operable.
4. [x] Resolver apertura/cierre en el pulso, sin cambios opuestos encadenados ni cierre sobre cruces.
5. [x] Verificar DosAlturas y preservar los anclajes y acontecimientos de Gorun.

## Verificación

- [x] Cada perfil ofrece apoyo, conexiones y salidas coherentes sin maniobras inexistentes.
- [x] La puerta cambia mediante una acción y rechaza cerrarse sobre volumen ocupado.
- [x] Abrir o cerrar actualiza acceso y contexto; volver conserva el estado local.
- [x] La muestra distingue superficies superpuestas y ofrece conexión practicable.
- [ ] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

La traducción preserva carácter, propósito y consecuencias, no casillas idénticas.
Gorun necesita referencias coherentes para sus eventos; no mantener firmas antiguas
mediante un adaptador provisional.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a completados; archivar con el cierre de la etapa.

- Fecha de finalización técnica: 2026-11-24.
- Recetas: validación de geometría y rutas antes de publicar; ocho intentos máximos. Palacio distingue entrada, salón intermedio y salón del objetivo.
- Puertas: solicitudes de interacción ligadas al plan efectivo. La apertura y el cierre se resuelven contra el estado inicial, con un único cambio por puerta; los cruces confirmados impiden cerrarla en ese pulso.
- Regresión corregida: contacto narrativo por proximidad hostil real, sin depender de la antigua bandera de revelación. Gorun conserva su brecha.
- Integración de etapa pendiente: bloques 055 a 057.
- Resultado: ocho recetas volumétricas, puerta manual en Palacio y plataforma conectada en DosAlturas.
- Verificaciones ejecutadas: 34 pruebas core de encuentros, una de consecuencias_locales (copa en Palacio), dos de eventos_locales_3d (Gorun), todas aprobadas. Clippy del core y formato correctos. Logs /tmp/054-core.log, /tmp/054-copa.log, /tmp/054-gorun-final.log y /tmp/054-clippy.log.
- Desviaciones respecto del plan: DosAlturas se ofrece en contenido mediante el ataque a Zahrim para permitir revisión manual.
- Trabajo posterior: [hoja de ruta](../base-espacial-y-planes-encuentros.md).


## Complemento de cierre (2026-11-26)

Se completó el resultado por participante de cada interacción con puerta, separado del cambio físico único. Rechazos y SinCambio no cuentan como progreso. Véase el cierre de los cuatro pendientes en el contrato técnico 12 y la revisión integrada 057.
