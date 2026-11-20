# Plan: Combatir en el campamento de Kazir

| Campo | Valor |
|---|---|
| ID | PLAN-2026-049 |
| Estado | En desarrollo |
| Creado | 2026-11-19 |
| Actualizado | 2026-11-19 |

## Resultado buscado

El ataque a Kazir ocurre en terreno exterior con varios accesos, defensores
visibles y obstáculos que cambian las rutas, usando el mismo motor de combate.

## Referencias

- [Mecánicas, sección 19](../../diseno/encuentros-y-atacar.md#19-ciclo-3-campamento-en-las-lomas).
- [Contrato técnico de encuentros](../../tecnica/acciones/10-encuentros-y-atacar.md).
- [Hoja de ruta](../encuentros-atacar-rescatar.md).

## Alcance

Una familia adicional seleccionable en encounters.ron, con generación acotada y
reproducible, visibilidad abierta, dos pasos del barranco, rocas y texto adaptado.
No incluye rescate, nuevas reglas de combate, salto, sigilo ni IA táctica avanzada.

## Pasos y aceptación

- [x] Añadir selección explícita de escenario con valor por defecto compatible.
- [x] Generar un campamento conectado con varios accesos e inicios válidos.
- [x] Adaptar presentación y frontera de contacto al espacio abierto.
- [x] Comprobar movimiento, final válido y retirada bajo las reglas existentes.
- [x] Mantener funcionando el refugio y los acontecimientos de Gorun.
- [x] Actualizar el contrato y registrar verificaciones acotadas.
- [ ] Revisar jugando con el usuario.

## Verificación

Pruebas de generación, conexiones, movimiento y cierre del escenario nuevo;
integración del perfil de Kazir y conservación del cautivo; regresión del caso de
Gorun. Compilación/Clippy y comprobación visual con TestBackend. Sin batería global.


## Resultado

Escenario Campamento explícito para Kazir; Recinto por defecto para objetivos
anteriores. Dos pasos de barranco reproducibles, obstáculos, posiciones válidas y
visibilidad abierta. El encuentro y la retirada usan el mismo reglamento y cierre.
Tamin permanece cautivo y fuera de las fichas hostiles.

La comprobación de generación cubre dos variantes y acceso desde la entrada a todos
los participantes y pasos. La prueba de sesión recorre un ataque completo real,
comprueba el movimiento de ambos bandos y conserva el cautivo al cerrar. Se revisa
el caso de Gorun para no romper las escenas del ciclo anterior. La proyección nueva
usa el contrato gráfico existente; no exige una nueva implementación de render.

## Recorrido manual

Con el grupo libre, viajar a Lomas de Anir y pulsar T para elegir a Kazir. Observar
el espacio abierto, las rocas y los dos pasos sobre el barranco. Enter inicia;
Espacio pausa, D muestra detalle, R pide retirada. La banda tiene cuatro miembros.
El resultado no está garantizado y no libera a Tamin por sí solo. Para comparar,
Gorun en Costa Ámbar conserva los dos cuartos y su brecha narrativa.


### Verificaciones ejecutadas

- Core `ciclo3`: 1 caso con dos variantes del generador y retirada.
- App `ciclo3`: 1 episodio completo, proyección abierta y cautivo conservado.
- App `ciclo2`: 2 regresiones de Gorun y transacción narrativa.
- Terminal sin audio `encuentro_terminal`: 2 casos de dibujo y reproducción.
- Clippy de terminal con audio: compila; solo advertencia preexistente de argumentos
  en `dibujar_retratos_dialogo`.
- Formato y `git diff --check` sin errores.

Todos los casos seleccionados pasaron. Logs en `/tmp/ciclo3-*.log`; no se ejecutó
la batería completa del workspace.
