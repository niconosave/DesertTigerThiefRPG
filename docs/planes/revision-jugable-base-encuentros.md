# Revisión jugable de la base de encuentros (057)

| Campo | Valor |
|---|---|
| Fecha | 2026-11-25 |
| Estado | Preparada; respuesta del usuario pendiente |
| Plan | [PLAN-2026-057](./pendientes/2026-11-24-57-verificar-base-de-encuentros.md) |

La implementación y las comprobaciones técnicas están listas para jugar.
Esta lista ayuda a revisar la presentación y el comportamiento percibido; no
requiere ganar todos los combates ni repetir la batería automática.

## Iniciar

Desde la raíz del proyecto, ejecutar como habitualmente:

```sh
cargo run -p desert-tiger-terminal --bin desert-tiger-terminal -- --no-audio
```

`--no-audio` es opcional. La compilación con audio se comprobó, pero esta revisión
se concentra en encuentros, no en escuchar música o ambientes.

## Recorridos

Se pueden hacer en distintas partidas. Una derrota legítima no es un fallo.

| Lugar | Acción | Qué observar |
|---|---|---|
| Costa de Ámbar | **T**, atacar a Gorun | Los relatos suspenden el tablero; su brecha ensancha el acceso y el encuentro continúa. |
| Lomas de Anir | **T**, atacar a Kazir | El campamento permite aproximarse, combatir y concluir, incluso si el grupo pierde. |
| Lomas de Anir | **R**, rescatar a Tamin | Liberarlo todavía exige sacarlo; si sale vivo, aparece en el grupo al volver al mapa. |
| Nuris | **R**, robar la Copa de las Dos Mareas | La puerta se abre antes de cruzarla. Extraer la copa la entrega al líder y retira esa oportunidad de robo. |
| Sarabad | **T**, atacar a Zahrim | Este perfil tiene dos alturas. Las fichas recorren la escalera; los oponentes también pueden bajar para combatir. |

No es necesario forzar una victoria para ver el escenario de dos alturas. La
posición inicial del enemigo está en la plataforma superior; no se garantiza que
los aliados suban si los defensores bajan primero. La prueba automática de
rescate usa una copia temporal con un cautivo arriba para comprobar además el
ascenso y la evacuación completos sin alterar el contenido base.

## Controles y consecuencias

- **[ / ]** bajan/suben el corte visual; **C** vuelve al seguimiento automático.
  El corte no mueve fichas ni descubre espacios. Un piso oculta lo que queda debajo.
- **Enter/Espacio** pausa; **D** muestra detalles; **S** omite animación sin saltar
  decisiones ni relatos. Una ventana demasiado pequeña detiene la reproducción.
- **R** solicita retirada. Si no se puede completar la evacuación, la pantalla
  identifica a los afectados: **Enter** continúa el auxilio y **A** confirma que
  esas personas quedan atrás. No se aplica ese permiso a heridos posteriores.
- **Enter**, al concluir, vuelve al flujo del mapa. Heridas, bajas, captura,
  compañeros separados y posesión del tesoro deben conservarse. Leer textos o
  cambiar el corte no hace pasar tiempo; cerrar confirma el tiempo una sola vez.

## Respuesta necesaria para cerrar

Indicar si los recorridos resultan comprensibles y si apareció algún bloqueo,
problema de dibujo o consecuencia incoherente. Si hubo un problema, alcanza con
el objetivo, el último texto o acción y lo que se observó. No hace falta aportar
una captura ni repetir todos los casos ante cada corrección.

La revisión del usuario aún no se ha registrado. Los planes 052–057 permanecen en
pendientes hasta esa aceptación; no se declara una prueba humana por el hecho de
que los recorridos automatizados hayan pasado.
