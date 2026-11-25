# Plan: Modelo espacial tridimensional y contexto mínimo

| Campo | Valor |
|---|---|
| ID | PLAN-2026-052 |
| Estado | Completado técnicamente; integración de etapa pendiente |
| Creado | 2026-11-24 |
| Actualizado | 2026-11-24 |

## Resultado buscado

Disponer de un único modelo espacial tridimensional con identidad local estable,
ocupación y consultas suficientes para expresar los encuentros de la nueva etapa.

Este plan es un bloque interno de una única etapa jugable. Su terminación técnica
no exige que el juego funcione durante cada paso ni constituye una entrega final.
La revisión integrada y el cierre conjunto corresponden a PLAN-2026-057.

## Referencias

- [Identidades y coordenadas](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#3-identidades-y-coordenadas).
- [Volumen y elementos](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#4-volumen-y-elementos).
- [Consultas y contexto](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#5-consultas-y-contexto).
- [Errores y límites](../../tecnica/acciones/12-base-espacial-y-planes-encuentros.md#14-errores-y-límites).
- [Hoja de ruta de la etapa](../base-espacial-y-planes-encuentros.md).

## Alcance

### Incluido

- Grilla acotada hasta 64 × 64 × 16 y volumen inicial de combatiente 1 × 1 × 2.
- Identidades locales estables para fichas, elementos, espacios y conexiones.
- Elementos definidos y probables; ocupación, apoyo, acceso y contexto mínimo.
- Invalidación del contexto y rechazo de geometría o referencias incoherentes.

### No incluido

- Generación modular compleja, mapas persistentes y simulación estructural.
- Sigilo, sonido, fuego, combate dinámico y nuevas órdenes narrativas de planes.

## Precondiciones

- Contrato técnico aplicable listo para implementar, sin decisiones abiertas de este bloque.
- No depende de otro bloque de esta etapa.

## Pasos

1. [x] Sustituir las estructuras espaciales planas por tipos e identidades locales.
2. [x] Implementar volumen, elementos, ocupación y consultas espaciales compartidas.
3. [x] Introducir contexto de espacios, conexiones y salidas y consulta de conocimiento.
4. [x] Comprobar límites, referencias, posiciones superpuestas, apoyo y transporte.
5. [x] Registrar resultado técnico y dependencias pendientes de integración.

## Verificación

- [x] Casos acotados de dos alturas, ocupación 1 × 1 × 2 y separación de superficies.
- [x] Rechazo de dimensiones fuera de límite, referencias inválidas y volumen ocupado.
- [x] Cambiar el contexto físico invalida la información derivada afectada.
- [ ] Se cumplieron los controles y la definición de terminado de
      [la metodología](../../metodologia-desarrollo.md), con cierre de etapa en el bloque 057.

## Riesgos y decisiones operativas

El campo de altura anterior no ofrecía soporte tridimensional real. Sustituir las
consultas y el almacenamiento conforme al contrato; no conservar un segundo modelo plano.

Esta sección no crea reglas ni contratos. Si aparece una decisión nueva, se
actualiza primero el diseño canónico y se registra aquí su efecto sobre el trabajo.

## Resultado de la implementación

> Completar antes de mover a completados; archivar con el cierre de la etapa.

- Fecha de finalización técnica: 2026-11-24.
- Avance: objetivo, transportistas, portador y referencias de ficha de los sucesos usan `IdFicha`. La validación de la escalera comparte las reglas de transición con la navegación.
- Avance: la grilla interna es privada. La app consulta vóxeles con coordenadas. La ocupación informa el impedimento tipado e identifica fichas y puertas. Se validan los límites de espacios y el vínculo entre la puerta y su conexión.
- Avance: los sólidos introducidos por Lua conservan su material. Los obstáculos bloquean el perfil completo y la edición de terreno preserva el volumen de los elementos locales.
- Apariencias y materiales: consultas semánticas sin glifos del frontend. Las presencias probables tienen envolvente comprobable y coherencia entre clase y definición.
- ContextoNavegacion entrega terreno recordado, elementos conocidos, conexiones disponibles e IDs observables; no publica el objetivo oculto. Su consumo por los planes corresponde al bloque 053.
- Organización: espacio.rs separa geometría/ocupación/rutas; elementos.rs y conocimiento.rs contienen las consultas de sus dominios.
- Verificación final: 27 pruebas del módulo encuentros aprobadas; cargo check de desert-tiger-app aprobado; formato y diff revisados. Logs /tmp/052-cierre.log y /tmp/052-integracion-check.log.
- Integración de etapa pendiente: bloques 053 a 057. Este resultado no declara terminada ni aceptada jugando la etapa completa.
- Resultado: volumen 31 × 17 × 8, posiciones XYZ, IDs locales, contexto y conexión vertical explícita.
- Verificaciones ejecutadas: `espacio_local`, `contexto_local` y módulo acotado de encuentros.
- Desviaciones respecto del plan: la fachada existente conserva el nombre `Posicion` como alias público de `PosicionLocal`; no existe motor plano alternativo.
- Trabajo posterior: [hoja de ruta](../base-espacial-y-planes-encuentros.md).
