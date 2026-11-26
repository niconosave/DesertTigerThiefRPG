# Vista oblicua de encuentros

| Campo | Valor |
|---|---|
| Estado | Primera implementación integrada; revisión estética pendiente |
| Fecha | 2026-11-25 |
| Base | [Contrato 12](../acciones/12-base-espacial-y-planes-encuentros.md) |

## Alcance

Sustituir la proyección cenital de la terminal por una vista oblicua de caracteres.
Conservar simulación, posiciones XYZ, corte manual/automático, pausas y consecuencias.
La revisión jugable del 057 detectó falta de legibilidad de alturas y escalera;
este documento no declara aceptada esa revisión.

Se incluyen colores de altura, paredes sólidas con detalles, personajes con torso
y piernas y marcas de escalera. La silueta invertida ayuda a localizar personajes
ya observables tapados por terreno conocido. No se recortan paredes alrededor de
personajes en esta primera versión. El bloqueo del robo es una corrección de
comportamiento independiente: este renderizado no lo resuelve.

## Proyección y cámara

Coordenadas de dibujo, antes de centrar y recortar:

    columna = 2 * x
    fila = y - z

Calcular en enteros con signo. El factor horizontal mantiene dos columnas de
terminal por casilla; subir una unidad Z desplaza una fila hacia arriba.
Es una aproximación oblicua con caracteres, no una promesa de ángulo físico exacto:
las proporciones dependen de la fuente. No se rota el plano XY.

Una columna física deja de producir un único carácter. Se generan muestras para
sus superficies, caras y elementos visibles hasta el corte. Un muro de dos bloques
de altura puede ocupar dos filas. La cámara centra la posición proyectada del
personaje seguido; conservar centro guardado cuando no haya foco. Al cambiar el
corte manual, conservar el anclaje de cámara para que el escenario no salte.

El recorte usa la zona del tablero entre cabecera y texto, no todo el frame.
Proyectar primero y comprobar límites después: objetos altos próximos a los bordes
no deben escribir sobre menús, descripciones ni el roster. Redimensionar, cambiar
corte o dibujar no ejecuta reglas ni modifica la vista de sesión.

## Superficies, caras y profundidad

Reemplazar la reducción de columnas `proyectar_celda` por un compositor puro de
muestras. Cada muestra incluye posición de pantalla, origen XYZ, clase visual,
glifo, colores, opacidad e identidad estable cuando corresponda.

Recorrer de fondo a frente (Y creciente) y, para un mismo Y, Z creciente. Cuando
varias muestras disputen una celda de pantalla, resolver por la clave
`(y, z, prioridad_visual, identidad_estable)`: mayor profundidad cercana gana;
la identidad solo rompe empates, no depende del orden de los vectores. En un mismo
origen, entidades e interacción se dibujan sobre su soporte. Reservar la segunda
columna para continuidad de superficies o señales de impacto, con la misma prueba
de profundidad; nunca superponer un impacto a otra entidad cercana.

Distinguir superficie superior y caras expuestas de sólidos usando vecinos del
volumen público. No dibujar caras interiores entre bloques sólidos conocidos.
Una superficie creada por el corte representa una sección, no un techo nuevo.
Prototipar bordes con caracteres simples (`_`, `|`, esquinas) y muros con espacios
que tienen fondo coloreado, cubriendo las dos columnas de la casilla. Árboles,
rocas y obstáculos conservan diferencias de material: no convertir todo en paredes.

La prueba decisiva del orden no es solo una columna vertical: incluir una pared
alta al fondo, un personaje cercano y dos personajes alineados en Y. La pared
trasera no puede sobrescribir al cercano por haberse dibujado su Z al final.

## Paleta y símbolos

- Superficies transitables: alternar tonos apagados de cian y azul por paridad del
  Z de su soporte. No usar variantes `Light*` ni destellos. La alternancia señala
  diferencias, no identifica de manera única la altura; bordes y desplazamiento
  completan la lectura. Mantener el número de corte visible.
- Caras de pared: fondo sólido oscuro, distinto de su coronamiento. Evitar bandas
  brillantes; concentrar el código de altura en superficies y sus bordes.
- Escaleras: `△` en el recorrido real, por encima del soporte. En modo ASCII usar
  `>` para no confundirlas con las piernas. No inferir escaleras por cualquier
  desnivel ni marcar todas las conexiones interiores.
- Personaje capaz de pie: `^` para piernas en la posición de sus pies y su letra
  identificadora para el torso una unidad Z más arriba. Mantener colores de bando,
  herida e identidad; no alternar sus colores con la altura del terreno.
- Caídos y muertos: conservar `x` y `%` a ras de su posición, sin torso erguido.
  Transportados y salidos conservan su tratamiento actual. El botín sigue siendo `$`.

Una parte cortada del cuerpo no se dibuja. Si solo se ven las piernas, incluir su
identificación en el roster; no dibujar el torso fuera del corte para forzarlo.
Comprobar el ancho de los glifos con la infraestructura existente y el modo ASCII.

## Corte, ocultación y siluetas

Excluir muestras por encima del corte antes de componer. El terreno que queda
más abajo sigue visible solo donde no lo tape una superficie cercana. La proyección
oblicua calcula la ocultación por rayos discretos de pantalla, no reutiliza la
prueba cenital de una única columna XY.

Desconocido es opaco: no equivale a aire. No generar bordes que delaten material
ni marcas de escalera dentro del volumen desconocido. Tampoco consultar la
geometría privada del núcleo para completar caras.

Tras componer terreno y entidades, un personaje observable oculto por terreno
conocido puede tener una marca con su letra negra sobre fondo de su color. La
marca corresponde a su posición proyectada bajo el corte, no atraviesa lo que
está por encima del corte ni revela actores omitidos por app. No tapa otro actor
visible ni se repite en cada bloque de la pared. Si solo queda oculto un segmento,
conservar los segmentos visibles sin duplicar la letra. Aplicar la misma regla a
impactos; no mostrar golpes a través de desconocido.

## Datos y responsabilidades

Core conserva la autoridad de geometría y las conexiones de escalera existentes.
App agrega a `VistaEncuentro` una lista mínima de marcas de escalera con ID de
conexión y posiciones XYZ conocidas. Solo exportar tramos descubiertos mediante
el mismo conocimiento usado para publicar el terreno, incluyendo las vistas de
encuentros suspendidos por relatos. Frontend no deduce una escalera por proximidad
a enemigos ni recibe nuevos participantes ocultos.

La matriz, fichas y objeto actuales son suficientes para las demás muestras.
El renderizador no busca rutas ni modifica la memoria de descubrimiento. Separar
proyección/composición de dibujo de widgets para probar la geometría visual sin
arrancar una partida. La profundidad cuesta un recorrido acotado del volumen más
las muestras de entidades; no crear un motor 3D ni dependencias gráficas nuevas.

## Implementación y comprobación acotada

1. Construir el compositor con una escena pequeña: pared, plataforma, escalera y
   dos personajes. Verificar recorte, profundidad y superposición torso/piernas.
2. Incorporar las marcas filtradas de app y conectar el compositor al encuentro
   real, incluyendo relatos suspendidos y seguimiento del corte.
3. Ajustar paleta/bordes y probar DosAlturas, puerta del palacio y brecha de Gorun.
   No mantener dos motores de proyección en producción; la escena pequeña es una
   fixture de prueba, no un modo de juego nuevo.

Pruebas seleccionadas: piso superior que tapa, hueco que deja ver, desconocido
opaco, pared trasera frente a actor cercano, corte parcial del cuerpo, silueta
sin revelación, escalera filtrada, recorte y ventanas pequeñas. Traducir los tests
cenitales afectados en lugar de conservar expectativas de dibujo obsoletas.
Reutilizar pruebas de consecuencias; repetirlas solo si cambia esa frontera.
Guardar salida en `/tmp`, sin batería global por defecto.

Revisión estética: captura o vista de la escena pequeña y encuentro real con el
usuario. La prueba automática confirma composición y seguridad; no sustituye la
valoración de legibilidad, colores ni estética.

## Registro de implementación

Implementada la proyección `2*x, y-z`, composición por profundidad, paredes con
fondo y coronamiento, colores alternos de soporte, cuerpo de dos caracteres,
marcas de escalera filtradas y silueta sobre terreno conocido. El modo ASCII usa
`>` para la escalera. El corte manual conserva el anclaje vertical del dibujo.
La vista de app publica actualmente posiciones XYZ de tramos de escalera; la
identidad de conexión permanece en core y no se necesita para dibujar las marcas.

Se retiró la reducción cenital usada en producción y sus dos tests específicos;
las comprobaciones nuevas ejercitan el renderizado oblicuo real. Pasaron
`vista_oblicua` (1), `corte_altura` (2) e `integracion_encuentros_3d` (1).
Clippy conserva solo el aviso preexistente de `dibujar_retratos_dialogo`.
Logs en `/tmp/oblicua-*.log`; formato y diferencias comprobados.
La selección estética y la legibilidad se revisarán jugando; no se afirma
aceptación humana. El bloqueo del robo permanece como corrección separada.

## Ajuste de sólidos tras revisión visual

Suelo, muro y obstáculo usan la misma regla visual de sólido, sin introducir
materiales ni modificar la geometría. Al cortar a su altura o tener otro bloque
encima se dibujan llenos, con gris oscuro/gris alternos por Z. Solo una celda
superior conocida de aire o vacío demuestra una superficie expuesta. Una
superficie interior expuesta se dibuja con puntos; su borde/coronamiento utiliza
`+`, sustituyendo `_`. Desconocido no descubre coronamientos. No se generan
techos: únicamente se representan los bloques existentes del escenario.

Se verifican sección en Z0, suelo expuesto, bloque apilado, paridad de grises,
coronamiento y desconocido superior mediante `solidos_corte`. Se conserva la
comprobación `vista_oblicua` y la de corte/impactos de terminal.

## Sombra ficticia sobre el piso

Implementada una huella hacia X menor e Y mayor. Cada tramo de altura extiende
las tres posiciones del ejemplo aprobado: izquierda, diagonal abajo-izquierda
y abajo, desplazadas diagonalmente por cada tramo adicional. La altura se mide
respecto del suelo receptor. Se consideran los bloques sólidos conocidos de toda la altura, independientemente del corte visual; no se
consulta geometría privada o desconocida.

La sombra cambia únicamente el fondo del suelo expuesto a negro RGB `(0,0,0)`,
no al color negro configurable de la terminal. El piso sin
sombra recibe un fondo muy oscuro `(18,24,28)` para que la diferencia sea visible;
se conservan puntos y colores de altura. No sombrea paredes, personajes, objetos
ni superficies desconocidas. Es un recurso visual sin luz física ni consecuencias
mecánicas. Pasaron `sombra_diagonal` (huella de tres niveles, límites, corte,
receptor y desconocido) y `vista_oblicua`; logs en `/tmp/sombra-*.log`.

La longitud se conserva al pasar entre cortes: el corte oculta el dibujo de la
columna, pero no recorta la altura conocida usada por la sombra. La prueba de
renderizado `sombra_fondo_negro_y_longitud_independiente_del_corte` comprueba el
extremo de una sombra de tres bloques con cortes 1, 2 y 4, y contrasta los fondos
RGB de piso sombreado e iluminado. Se ejecutaron las dos pruebas `sombra_`.

## Franjas oscuras y sección iluminada

Las caras sólidas alternan gris oscuro y el `grisMuyOscuro` ya existente de los
retratos (RGB 32,32,32). El bloque que coincide exactamente con el corte Z utiliza
gris claro (`Gray`), independientemente de la paridad. Si el corte está por encima
de la estructura, no se inventa una capa adicional iluminada. Se conservan `+`,
las superficies de piso y las sombras. La prueba `solidos_corte` comprueba sección
iluminada, franja oscura y coronamiento con la paleta nueva.

## Arquitectura del escenario de dos alturas

La receta DosAlturas incorpora cuatro pilares en las esquinas de la plataforma,
entre Z1 y Z2, bajo el soporte de Z3. El salón superior tiene paredes en Z4 y Z5,
con una abertura de tres casillas frente a la escalera. No se añade techo. Estos
bloques son geometría real del encuentro, no decoración superpuesta; conservan
libres la escalera y las posiciones iniciales de Zahrim y su defensor.
La descripción de ataque identifica ahora el palacio elevado, no un campamento.

## Sombra de bloques suspendidos

Cada bloque sólido conocido proyecta independientemente las tres casillas de su
tramo de altura, aunque tenga aire debajo. Una plataforma elevada proyecta la
unión de esas huellas; los huecos no añaden tramos ficticios ni es necesario un
pilar continuo. Se conserva el receptor de suelo, negro RGB y altura independiente
del corte. Pasaron las tres pruebas `sombra_`, incluida la regresión de un bloque
en Z3 sin apoyos intermedios y su desaparición al retirarlo o volverlo desconocido.

## Textura ajedrezada del piso

Las superficies expuestas de suelo reemplazan el punto por `[]`, uno por cada
columna de terminal, solo donde `(x+y)%2 == 0`. Las casillas impares mantienen
ambos caracteres vacíos. El patrón usa coordenadas del escenario y no de cámara,
por lo que permanece anclado al desplazarse la vista. Conserva sombras y colores;
secciones sólidas, coronamientos, escaleras y personajes no cambian.

La textura ajedrezada usa gris azulado RGB `(65,80,95)` en alturas pares y gris
verdoso RGB `(65,85,80)` en impares, sustituyendo el cian/azul saturado del piso.
Los personajes conservan sus colores para destacarse sobre esa textura.

## Baldosas de fondo: presentación vigente del piso

Se retiran los corchetes de las iteraciones anteriores. Cada casilla de piso
expuesto ocupa dos espacios con el mismo fondo; la paridad `(x+y)%2` elige la
baldosa y la paridad de Z elige su pareja de colores. No se añaden símbolos.

| Altura | Baldosa par RGB | Baldosa impar RGB |
|---|---|---|
| Z par | 18,26,36 | 25,35,46 |
| Z impar | 24,34,25 | 33,44,31 |

Ambas parejas son oscuras y poco saturadas; la primera es azulada y la segunda
verdosa. La sombra prevalece como negro explícito sobre ambas baldosas.
Torso, piernas y señales conservan el fondo de la baldosa. Pasaron
`vista_oblicua` y las tres pruebas `sombra_` tras actualizar las expectativas de
fondo. Esta sección sustituye las decisiones anteriores sobre textura con puntos
o corchetes; las secciones anteriores registran las iteraciones visuales.

## Negro de sombra elegido tras revisar las baldosas

Con las baldosas de fondo ya aceptadas, el usuario eligió reemplazar el negro
absoluto de las sombras por `Color::Black`, el negro de la paleta de la terminal.
Esta elección sustituye el RGB `(0,0,0)` de las iteraciones anteriores. Longitud,
dirección, geometría conocida y patrones de piso se conservan.

## Fondo predeterminado y transparencia de la terminal

La intención final del usuario es recuperar el fondo semitransparente de su
terminal, no el color ANSI negro. Sombras y espacios vacíos usan `Color::Reset`:
restablecer fondo predeterminado, como el área exterior del tablero. No se cambia
el negro de las letras invertidas ni las baldosas coloreadas. `Color::Black` es
la entrada configurable de paleta; RGB `(0,0,0)` es pintura negra explícita.
La transparencia la decide la terminal. Los modos explícitos de fondo negro
`-B` o esquema fijo pueden sustituir Reset por un fondo opaco según su contrato.

## Alineación de superficies y pies

El origen de un bloque sólido está en Z, pero su cara superior se proyecta en
Z+1. Las superficies expuestas de suelo se dibujan a esa altura, coincidiendo
con los pies apoyados sobre ellas. Los coronamientos expuestos también se elevan
una fila; las caras verticales permanecen en Z para conservar el volumen de la
columna. Si el corte atraviesa el sólido, se mantiene su sección en Z: no se
inventa una cara superior fuera del corte. Cada muestra se recorta después de
proyectarla, sin escribir fuera del tablero.

Las posiciones de actores, puertas y escalera no cambian. Una columna detrás
puede coincidir en pantalla con los pies de un personaje situado delante; el
suelo de su casilla se dibuja antes de los pies y proporciona su fondo, incluida
la sombra, sin heredar el gris de esa columna. La regresión oblicua cubre este
caso y el corte parcial del cuerpo. Las pruebas de sombra ajustan solamente la
fila de pantalla del piso, no su huella ni su altura física.

## Bordes de los escenarios exteriores

Campamento, Ciudad, Bosque, Montana y Playa ya no generan el muro rectangular
perimetral. Los límites del volumen siguen impidiendo salir del tablero; no se
crean salidas de encuentro adicionales. Se mantienen obstáculos y edificios
interiores. El barranco del campamento se prolonga hasta los bordes para conservar
sus pasos de cruce sin añadir un rodeo accidental por el perímetro.
Recinto (guarida cerrada), Palacio y DosAlturas (palacio elevado) conservan sus
paredes exteriores. Las pruebas de generación comprueban perímetro abierto,
límites válidos y barranco; se verifica además el recorrido del campamento.

## Fin del tablero sin borde artificial

Las superficies expuestas de piso que llegan al límite XY del volumen se dibujan
con las mismas baldosas que las interiores. La ausencia de una casilla fuera del
tablero no genera coronamiento `+` ni cara gris. Los vacíos reales dentro del
escenario siguen diferenciándose de esos límites artificiales. No cambian paredes,
columnas ni secciones sólidas al cortar Z. Se verifican esquinas y laterales con
`piso_perimetral` y se conserva la regresión `vista_oblicua`.

## Corrección de bordes internos y encuadre

Tras revisar el campamento, se elimina también el coronamiento del suelo junto
a huecos internos: toda superficie expuesta de Suelo usa baldosas, sin cara gris
adicional. El vacío sigue siendo vacío; no se rellena el barranco con geometría.
Paredes y columnas conservan sus coronamientos. Las secciones sólidas en Z0 no
se confunden con una superficie expuesta y conservan su representación.

El origen vertical de cámara se compensa una fila para incorporar la cara superior
del suelo base y llenar la zona visible del tablero sin la franja vacía que dejó
la eliminación de las caras perimetrales. La misma compensación se aplica a
terreno, escaleras, fichas, objetos y siluetas: no cambia su alineación relativa.
La regresión de perímetro comprueba ahora el buffer real en las cuatro esquinas
y junto al hueco, además de la clasificación de superficies. Pasaron
`piso_perimetral`, `vista_oblicua` y `sombra_fondo`.

## Referencia común para sección y superficie

La sección cortada de un bloque en Z también termina en su cara superior Z+1.
Se sustituye la regla anterior que dibujaba la sección en el origen Z: era la
causa del salto del suelo y los huecos al pasar de corte 0 a corte 1. El color
sigue distinguiendo sección gris de superficie expuesta con baldosas, sin cambiar
el contorno proyectado.

Las caras verticales de suelo expuestas hacia el frente junto a un vacío real
conocido vuelven a dibujarse en gris, en Z. No se generan por ausencia de celdas
fuera del tablero ni por terreno desconocido, y no añaden signos de coronamiento
al piso. Se mantienen los coronamientos de paredes y columnas. La modificación
solo cambia la representación, no geometría, rutas ni posiciones de fichas.

Pasaron cinco pruebas seleccionadas: contorno estable entre sección/superficie,
perímetro sin borde artificial, cuerpo oblicuo y dos pruebas de corte. La prueba
nueva compara el contorno del buffer completo del tablero entre Z0 y Z1 y verifica
una cara vertical junto al hueco. Logs en `/tmp/seccion-*.log`.

## Regla vigente de sección y volumen por corte

Tras la aclaración del usuario, se sustituye la regla anterior de mantener fijo
el contorno entre sección y superficie. Un bloque en el nivel del corte Z se
muestra como sección plana en su propio Z. Cuando el corte lo supera, su cara
lateral se dibuja en Z y su cara superior expuesta en Z+1. La base permanece fija;
la sección asciende según el nivel elegido. Los bloques por encima del corte no
se dibujan. Un soporte de suelo cubierto por otro bloque no añade una cara gris
interior si no tiene frente expuesto.

Una columna con bloques Z1–Z4 tiene sección sola en corte 1; una, dos y tres líneas
de pared bajo la sección en cortes 2, 3 y 4; cuatro líneas de pared y superficie
superior en corte 5. Los cortes posteriores conservan esa misma forma. No se
agregan techos inexistentes. La regla se aplica también al suelo Z0: al pasar a
corte 1 se descubre su volumen y su superficie superior, desplazada una fila.

Se reemplazó la prueba del contorno invariable por un recorrido del renderizador
con esa columna de cuatro bloques, cortes 1–7 y comprobación del suelo en cortes
0 y 1. Pasaron `seccion_plana`, `vista_oblicua`, `corte_altura` (dos pruebas) y
`piso_perimetral`. La geometría, movimientos, puertas y cámaras no cambian.

## Encuadre completo de la proyección y cara inferior exterior

Se sustituye el límite vertical basado únicamente en `v.alto` por el intervalo
proyectado de la geometría conocida y las fichas publicadas. Se reserva margen
superior para caras elevadas y torsos, y una fila para la cara inferior del suelo.
Los límites se calculan sin depender del corte actual, evitando desplazamientos
de cámara al inspeccionar alturas. En ventanas amplias entra toda la proyección;
en ventanas pequeñas se recorta siguiendo la posición proyectada del personaje
principal, con coordenadas verticales de cámara que pueden ser negativas.

La cara vertical del suelo en el extremo frontal del tablero vuelve a mostrarse,
sin coronamiento `+` ni pared añadida. Es el espesor del bloque existente. Su fila
está dentro del área del tablero; no invade la descripción ni los controles.
La misma cámara se aplica a terreno, escaleras, fichas, impactos, objetos y siluetas.

Se comprueba el renderizado de una pared alta en el extremo superior, el torso
y piernas de un personaje en esa fila y la cara inferior completa en una ventana
alta. También se comprueba seguimiento en una ventana corta. Pasaron las pruebas
`encuadre_muestra`, `sombra_fondo`, `seccion_plana`, `integracion_encuentros_3d`,
`vista_oblicua` y `corte_altura` (dos), reutilizando el resto de evidencia previa.
Los logs de esta corrección están en `/tmp/encuadre-*.log`.

Los coronamientos expuestos de los sólidos se dibujan con `++`, uno por cada
columna del tile. Las puertas conservan su símbolo individual.

## Esquinas del coronamiento de muros

Corregida la clasificación de esquinas del volumen: la ausencia de vecinos fuera
del tablero no cuenta como un bloque sólido para determinar la superficie
interior de un muro. En las cuatro esquinas el coronamiento conserva `++` y su
fondo sólido, sin puntos con fondo Reset. El piso expuesto conserva su regla
propia de baldosas, anterior a esa comprobación. Pasaron `esquinas_de_muros` y
`piso_perimetral`.

Gorun sigue usando Recinto por configuración de encuentros. Su relato describe
un refugio entre rocas con separación de piedra, no establece explícitamente
una cueva: la forma rectangular cerrada es la simplificación de la receta actual,
no una decisión narrativa adicional.
