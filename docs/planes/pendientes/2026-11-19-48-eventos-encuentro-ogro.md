# Plan: Acontecimientos propios en el encuentro de Gorun

| Campo | Valor |
|---|---|
| ID | PLAN-2026-048 |
| Estado | Implementado y probado por el usuario; pendiente archivo |
| Creado | 2026-11-19 |
| Actualizado | 2026-11-19 |

## Resultado buscado

Atacar a Gorun incluye escenas de autor que abren una brecha real; las fichas
continúan con reglas desde el estado modificado y concluyen el episodio.

## Referencias

- [Mecánicas, sección 18](../../diseno/encuentros-y-atacar.md#18-ciclo-2-acontecimientos-escritos-dentro-del-encuentro).
- [Contrato técnico, ciclo 2](../../tecnica/acciones/10-encuentros-y-atacar.md#ciclo-2-implementado-intervenciones-locales).
- [Hoja de ruta](../encuentros-atacar-rescatar.md#ciclo-2-acontecimiento-escrito-dentro-del-encuentro).

## Alcance

Incluye fronteras narrativas del encuentro, suspensión/reanudación, consultas Lua,
cambio de terreno y reubicación individual validados, transacción Mundo/tablero y
cuatro eventos originales de Gorun. Excluye creación de fichas, fuego, áreas nuevas,
objetos, rescate y nuevos generadores. No cambia el reglamento de golpes.

## Pasos y aceptación

- [x] Exponer fronteras semánticas sin evaluar eventos en cada paso.
- [x] Reutilizar el coordinador y sus viñetas; conservar el encuentro suspendido.
- [x] Implementar comandos locales con validación, anclajes y consultas de autor.
- [x] Confirmar cambios mundiales y locales sin aplicar medios bloques.
- [x] Mantener el tablero visible detrás del texto y continuar al terminar la escena.
- [x] Escribir preparación, contacto con brecha, herida condicional y conclusión.
- [x] Comprobar episodio real, fallo mixto y reubicación, más dibujo/reanudación.
- [x] Revisar jugando con el usuario legibilidad, personalidad y ritmo. Confirmó
  textos, herida y apertura efectiva de la brecha, con continuación correcta.

## Verificación acotada

Casos nuevos: `ciclo2` en core, app y terminal (TestBackend). Regresiones del motor
`encuentros`, sesión `ataque_sesion` e interfaz `encuentro_terminal`. Compilación y
Clippy de terminal con dependencias y audio. Logs breves en `/tmp/ciclo2-*.log`.
Resultados: 6 casos de motor (incluido el nuevo), 2 de ciclo 2 en app, 5 de
regresión de sesión, 1 de ciclo 2 en terminal y 2 de regresión visual: todos pasan.
Clippy con audio compila; solo permanece la advertencia preexistente de demasiados
argumentos en `dibujar_retratos_dialogo`. Formato y diff sin errores de espacios.
No se solicita ni ejecuta la batería completa del workspace.

## Resultado y adaptaciones

Se implementan solamente cambiar terreno y reubicar. El primer protocolo admite
listas separadas (Mundo primero, comandos locales después) dentro de una misma
confirmación; no la secuencia arbitrariamente intercalada del contrato futuro.
La matriz sigue siendo el perfil de dos cuartos, narrado como refugio rocoso.
Los eventos ocurren una vez por partida; la herida puede saltarse si el ogro pasa
directamente a incapacitado. Ningún relato garantiza victoria ni concede objetos.

## Recorrido manual

Iniciar una partida nueva, terminar el prólogo y pulsar T en Costa Ámbar. Elegir
Gorun. Continuar la preparación con Enter y comenzar la reproducción. Al entrar
en el segundo cuarto aparece la escena de contacto; al continuar se abre el paso.
Confirmar que el grupo sigue combatiendo y que el cierre respeta el resultado.
También se puede pedir R para retirarse: las escenas vistas no se repiten al volver.
