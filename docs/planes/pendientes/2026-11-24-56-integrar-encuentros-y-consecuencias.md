# Plan: Integración de encuentros y consecuencias

| Campo | Valor |
|---|---|
| ID | PLAN-2026-056 |
| Estado | Completado técnicamente; integración de etapa pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-25 |

## Resultado buscado

Ejecutar Atacar, Rescatar y recuperar tesoros sobre el único modelo nuevo,
con eventos adaptados y consecuencias confirmadas sin duplicaciones.

Este plan es un bloque interno de una única etapa jugable. Su terminación técnica
no exige que el juego funcione durante cada paso ni constituye una entrega final.
La revisión integrada y el cierre conjunto corresponden a PLAN-2026-057.

## Referencias

- [Avance y confirmación atómica](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#9-avance-y-confirmación-atómica).
- [Conclusiones y consecuencias](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#10-conclusiones-y-consecuencias).
- [Eventos narrativos existentes](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#11-eventos-narrativos-existentes).
- [Orden de integración](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#16-orden-de-integración).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).
- [PLAN-2026-053](./2026-11-24-53-planes-y-comportamientos.md).
- [PLAN-2026-054](./2026-11-24-54-traducir-escenarios-y-puerta.md).
- [PLAN-2026-055](./2026-11-24-55-vista-y-corte-por-altura.md).

## Alcance

### Incluido

- Sustitución del adaptador de sesión y publicación de vistas nuevas.
- Confirmación coordinada de Mundo, encuentro, RNG y presentación.
- Adaptación tridimensional de los eventos existentes, incluidos los de Gorun.
- Conclusiones causales, heridos o abandonados, cautiverio y tesoro asignado al líder.

### No incluido

- Compatibilidad con el motor anterior y duplicación de resolutores.
- Asignación de planes desde Lua, nuevos eventos complejos y sucesión de personaje.

## Precondiciones

- PLAN-2026-053, PLAN-2026-054 y PLAN-2026-055 completados técnicamente.
- Contratos de operaciones y eventos existentes disponibles como referencia.

## Pasos

 Sustituir el estado activo y el flujo de sesión por el nuevo encuentro.
 Integrar tokens, frames y operaciones sobre candidatos antes de confirmar efectos.
 Traducir consultas, comandos locales y referencias de los eventos existentes.
 Aplicar conclusiones causales y resolver a todos los participantes y bienes.
 Adaptar selectores y consecuencias mundiales; registrar la integración para el bloque final.

## Verificación

- [x] Un fallo en efectos o preparación de la presentación no confirma cambios parciales.
- [x] Tokens obsoletos, consultas y frames de reproducción no repiten acciones o azar.
- [x] Extraer un objeto asigna posesión al líder y rescatar conserva libertad y ubicación.
- [x] Retirada incompleta, captura y derrota no borran ni teletransportan participantes.
- [x] Tiempo y efectos ya confirmados no vuelven a aplicarse al cerrar.
- [x] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

La integración puede descubrir supuestos del modelo viejo. Corregir los contratos
antes de introducir excepciones; no crear una vía de compatibilidad para eludir
ocupación, identidad o consecuencias.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a completados; archivar con el cierre de la etapa.

- Fecha de finalización técnica: 2026-11-25.
- Resultado: Atacar, Rescatar y recuperar objetos usan el modelo espacial nuevo.
  El módulo `encuentros/resultado.rs` valida causa, objetivo y contabilidad total
  de fichas y bienes antes de publicar la conclusión. La etiqueta `Desenlace`
  resume esa conclusión exclusivamente para los textos.
- La evacuación atascada publica una decisión con identidades, nombres y motivos.
  Enter continúa el auxilio; A confirma el abandono de las personas enumeradas.
  La respuesta no ejecuta un pulso ni consume azar. La autorización no se extiende
  a otros heridos posteriores.
- La captura se calcula individualmente por acceso, coste de ruta e ID estable.
  Los compañeros vivos que quedan en el lugar o cautivos se separan del grupo
  con operaciones del Mundo; el jugador conserva las limitaciones de campaña
  vigentes, sin sucesión automática. No se libera a un cautivo sin extraerlo vivo.
- Extracción de tesoro, rescate y consecuencias de cierre tienen confirmaciones
  independientes. El tesoro se asigna al líder. Cerrar solo reconoce el resultado
  y confirma diez minutos de preparación más seis segundos por pulso efectivo.
- Eventos locales y operaciones mundiales se preparan junto a la presentación
  antes de confirmar; Gorun conserva sus intervenciones tridimensionales.
- Verificaciones ejecutadas: core `encuentros::` (35 pruebas antes de agregar las
  dos específicas de cierre), core `consecuencias_locales` (3), app
  `consecuencias_locales` (3), app `rescate_sesion` (3), app `eventos_locales_3d`
  (2) y terminal sin audio `consecuencias_locales` (1). Detalles en `/tmp/056-*.log`.
  Formato y comprobación de diferencias correctos; Clippy de los crates afectados
  conserva únicamente el aviso preexistente `too_many_arguments` de
  `dibujar_retratos_dialogo`.
- No se ejecutó una batería del workspace. La revisión jugable integrada,
  comprobaciones adicionales justificadas y archivo conjunto pertenecen al 057.
- Trabajo posterior: [hoja de ruta](../base-espacial-y-planes-encuentros.md).


## Complemento de cierre (2026-11-26)

La conclusión reutiliza rutas por captor y conserva un certificado del estado físico. Su validación no repite búsquedas y rechaza conclusiones obsoletas. Véase el cierre de los cuatro pendientes en el contrato técnico 12 y la revisión integrada 057.
