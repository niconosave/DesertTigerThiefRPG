# Plan: Rescatar y extraer a Tamin

| Campo | Valor |
|---|---|
| ID | PLAN-2026-050 |
| Estado | Implementado; pendiente revisión de jugabilidad |
| Creado | 2026-11-20 |
| Actualizado | 2026-11-20 |

## Resultado buscado

Seleccionar Rescatar, alcanzar al cautivo, soltarlo y sacarlo del encuentro bajo
las reglas de combate. Conservar libertad, lesiones, vínculos y tiempo en la partida.

## Referencias

- [Mecánicas, sección 22](../../diseno/encuentros-y-atacar.md#22-rescatar-a-una-persona-cautiva).
- [Contrato de Rescatar personas](../../tecnica/acciones/11-rescatar-personas.md).
- [Hoja de ruta](../encuentros-atacar-rescatar.md).

## Alcance

Tamin y los cuatro bandidos de Kazir como caso inicial. Cooperación sencilla,
retirada con el rescatado, carga de incapacitados, registro de libertad y compañero.
No incluye objetos, esclavitud, rescate dentro de colectivos, sucesión o IA nueva.

## Pasos y aceptación

- [x] Elegibilidad y contenido para rescates sin hacer hostil al cautivo.
- [x] Reutilizar escenario, combate y movimiento; liberar por contacto y extraer.
- [x] Confirmar libertad y relaciones sin duplicar IDs ni repetir efectos.
- [x] Conservar consecuencias de retirada, muerte o captura; aceptar guardias muertos.
- [x] Integrar selector R, señal @, relato general y puntos narrativos.
- [x] Probar rescate completo, incapacitado, retirada, guardias y recaptura.
- [x] Comprobar interfaz real con TestBackend y documentar el alcance.
- [ ] Revisar jugando con el usuario y archivar al cerrar.

## Verificación acotada

- Core `rescate`: extracción capaz/incapacitado, retirada y relaciones estables.
- App `rescate_sesion`: rescate después de derrotar guardias, retirada y batalla real.
- Terminal `rescate_terminal`: selector, cancelación, dibujo y extracción completa.
- Regresiones seleccionadas de Atacar y del evento de Gorun, sin batería global.
- Clippy con audio, formato y diff sobre archivos modificados.

Logs en `/tmp/rescate-*.log`. La revisión manual y el ritmo narrativo quedan para
el usuario; no se presentan las pruebas de terminal como aceptación humana.

## Recorrido manual

Con el grupo libre, llegar a Lomas de Anir, pulsar R y elegir a Tamin. Enter inicia
la operación; @ representa al cautivo. Observar la aproximación, la liberación y
el regreso hacia la salida. R dentro del encuentro pide retirada. Al concluir con
la extracción, Tamin aparece en el grupo y ya no se ofrece como rescate.
También se puede atacar primero a Kazir y después rescatar a Tamin. Una retirada
sin liberarlo conserva el cautiverio; derrotar a Kazir no lo elimina por sí solo.

## Adaptaciones

Se reutiliza el adaptador de encuentro existente en vez de duplicar Atacar. La
captura original cambia a LiberadoDe al extraer al actor; conserva el ID de relación
y añade SalvadoPor si no existía. Una recaptura del mismo captor reactiva ese vínculo.
La primera incorporación al grupo es automática; las consecuencias históricas
particulares de un rescate podrán escribirse después mediante eventos.


### Resultados registrados

Pasaron los 3 casos nuevos de core, 3 de sesión de rescate y 1 de interfaz de
rescate. También pasaron 5 regresiones de sesión de Atacar, 2 del caso narrativo
de Gorun y 2 de dibujo/reproducción. Clippy con audio compila con la advertencia
preexistente de argumentos en dibujar_retratos_dialogo. Formato y diff sin errores.
