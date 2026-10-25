# Plan: Incorporar navegación a las expediciones

| Campo | Valor |
|---|---|
| ID | PLAN-2026-019 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

El jugador y los NPC pueden recorrer el estuario, costas y mares mediante bote
o barco, responder a problemas de la embarcación y atracar sin que una sola
tirada rutinaria produzca un naufragio fatal.

## Referencias

- [Acciones de navegación](../../tecnica/acciones/particulares/03-navegacion.md).
- [Medios por etapa](../../tecnica/acciones/02-viajar-expediciones.md#7-medios-por-etapa).
- [Conclusiones y operaciones](../../tecnica/acciones/02-viajar-expediciones.md#11-traslado-conclusión-y-operaciones).
- [Estuario y Nahal](../../contenido/trayectos-viaje.md#5-lote-inicial).
- Dependencia: [PLAN-2026-018](../completados/2026-10-17-18-viajar-camellos-caravanas.md).

## Alcance

### Incluido

- bote y barco como medios transitorios o acceso cualitativo;
- las seis acciones particulares marítimas;
- estado progresivo de la embarcación y crisis con respuesta;
- inicio de fase acuática en muelle o puerto y fin al desembarcar;
- corredor Nahal–Estuario–Mar de Levante y hechos mínimos de navegación.

### No incluido

- naves con identidad, velamen detallado, mareas calculadas, combate naval o natación.

## Precondiciones

- PLAN-2026-018 completado.

## Pasos

1. [x] Incorporar medio y estado transitorio de embarcación a Expedición.
2. [x] Implementar `Remar`, `Navegar`, `GobernarTimon`, `Fondear`, `Atracar` y `Achicar`.
3. [x] Integrar disponibilidad desde Características de viajeros, `muelle` y `puerto`, y cerrarla al desembarcar.
4. [x] Integrar pronósticos, ruta, maniobras habilitadas y respuestas a crisis.
5. [x] Completar las Características aprobadas de Nahal, Estuario y el corredor marítimo de prueba.
6. [x] Adaptar viñetas y resúmenes NPC sin dibujar rutas ni crear otra pantalla.
7. [x] Añadir pruebas de bote, barco, deriva, estados, atraque y no fatalidad inmediata.
8. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] Entrar al agua desde una costa sin medio, muelle o puerto no presupone un barco.
- [x] Cada etapa usa Remar o Navegar como movimiento dominante, no ambos.
- [x] Una deriva conserva el último Lugar confirmado; nunca crea una coordenada.
- [x] `NaufragioInminente` ofrece una respuesta antes de una consecuencia definitiva.
- [x] Atracar solo traslada al Grupo cuando la transición se completa.
- [x] Pasan validador, formato, check, Clippy estricto, pruebas y build release.
- [x] Se cumplieron los controles aplicables; la revisión manual se consolidó expresamente en PLAN-2026-020.

## Riesgos y decisiones operativas

- El primer modelo de barco a vela queda absorbido por Navegar; no se añadirá una tirada rutinaria de timón o velamen.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: navegación en bote y barco, seis maniobras, estado progresivo y respuesta explícita a crisis integrados en Viajar.
- Verificaciones ejecutadas: corredor marítimo oficial, formato, check, Clippy estricto, pruebas unitarias y de integración y build release.
- Desviaciones respecto del plan: `acuatico` distingue posiciones sobre el agua, `canalNavegable` permite el paso de barcos por el estuario somero y comenzar en un Lugar acuático conserva acceso implícito al medio con el que se llegó para no dejar varado al Grupo. La revisión manual se realizará en PLAN-2026-020.
- Trabajo posterior: [PLAN-2026-020](../pendientes/2026-10-17-20-equilibrar-viajar-expediciones.md).
