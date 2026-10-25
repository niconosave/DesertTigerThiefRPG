# Plan: Incorporar conocimiento de Lugares

| Campo | Valor |
|---|---|
| ID | PLAN-2026-015 |
| Estado | Completado |
| Creado | 2026-10-17 |
| Actualizado | 2026-10-18 |

## Resultado buscado

Actores y Grupo usan Relaciones `Conoce` para orientar y estimar recorridos sin
revelar hechos que no conocen, conservando visibles el atlas y sus conexiones.

## Referencias

- [Semántica de Conoce](../../tecnica/componentes/12-conocimiento-lugares.md#2-semántica-de-conoce).
- [Modelo e índices](../../tecnica/componentes/12-conocimiento-lugares.md#4-modelo-e-índices-en-rust).
- [Pronóstico común](../../tecnica/acciones/particulares/00-contrato-resolucion.md#8-pronóstico-y-conocimiento).
- [Conocimiento y ruta](../../tecnica/acciones/02-viajar-expediciones.md#5-destinos-conocimiento-y-ruta-automática).
- [Apoyo de contenido](../../contenido/trayectos-viaje.md#apoyo-inicial-para-probar-las-reglas).
- Dependencia: [PLAN-2026-014](./2026-10-17-14-recorrer-expediciones-por-lugares.md).

## Alcance

### Incluido

- `Conoce` Actor→Lugar en modelo, DTO, validación e índices;
- consultas individuales y unión canónica del conocimiento del Grupo;
- proyección conocida para pronósticos y `Orientarse`;
- relaciones y capacidades aprobadas del Bandido y Cazador;
- diferencias visibles de certeza sin niebla de guerra.

### No incluido

- aprender, olvidar, rumores falsos, grados de dominio o API Lua de Relaciones.

## Precondiciones

- PLAN-2026-014 completado.
- Esquema y contenido creativo citados aprobados.

## Pasos

1. [x] Ampliar tipos reglados, construcción e índices con `Conoce` dirigido.
2. [x] Ampliar el esquema RON, resolución tipada y diagnósticos de Relaciones.
3. [x] Implementar consultas individuales y unión del conocimiento de viajeros.
4. [x] Hacer que pronósticos y `Orientarse` reciban exclusivamente su proyección autorizada.
5. [x] Añadir las Relaciones y Características aprobadas al Bandido y al Cazador.
6. [x] Exponer en la vista de destino incertidumbre y factores conocidos sin listar secretos.
7. [x] Añadir pruebas unitarias, de contenido, integración y no filtración.
8. [x] Ejecutar controles y acumular la revisión manual para el cierre conjunto de la serie.

## Verificación

- [x] Consultar `Conoce` no recorre todas las Relaciones y distingue ID inválido de respuesta negativa.
- [x] El Grupo aprovecha el conocimiento del Bandido; el Príncipe por sí solo no lo posee.
- [x] El Cazador usa las mismas reglas como NPC y su política sigue excluida mientras perteneciera al Grupo.
- [x] Un hecho oculto cambia la ejecución autoritativa sin aparecer en el pronóstico.
- [x] El mapa conserva nombres y destinos visibles.
- [x] Pasan validador, formato, check y Clippy estricto; pruebas y build release final se repiten en PLAN-2026-020.
- [x] Se cumplieron los controles aplicables; la revisión manual se consolidó expresamente en PLAN-2026-020.

## Riesgos y decisiones operativas

- Estimación y ejecución compartirán derivación; no se mantendrá una segunda tabla de factores.

## Resultado de la implementación

> Completar antes de mover a `completados/`.

- Fecha de finalización: 2026-10-18.
- Resultado: `Conoce` dirigido, consultas indexadas, conocimiento conjunto y pronósticos sin filtración implementados.
- Verificaciones ejecutadas: contenido oficial, formato, check, Clippy estricto y 67 pruebas automáticas.
- Desviaciones respecto del plan: la revisión manual comparativa se realizará dentro de la sesión final de PLAN-2026-020.
- Trabajo posterior: [PLAN-2026-016](./2026-10-17-16-preparar-consumir-provisiones.md).
