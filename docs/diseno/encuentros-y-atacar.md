# Encuentros y Atacar — Diseño de las mecánicas

| Campo | Valor |
|---|---|
| Estado | Borrador para revisión; no implementado |
| Versión | 0.1 |
| Fecha | 2026-11-19 |
| Alcance | Base compartida de encuentros, acción Atacar y primer episodio jugable |

## 1. Propósito y autoridad

Atacar representa un episodio completo: preparar una incursión, acercarse,
penetrar en el escenario, afrontar oposición, alcanzar al objetivo y resolver
la salida. Puede contener varios enfrentamientos. Un intercambio de golpes es
una parte del episodio, no toda la acción general.

El jugador persigue un propósito con personas concretas. Sus características,
relaciones, preparación y decisiones influyen en lo que pueden hacer y en las
situaciones que atraviesan. El fracaso puede transformar la historia y permitir
que la partida continúe, conservando pérdidas y consecuencias reales.

Este documento define reglas y experiencia de juego. No define estructuras de
Rust, formatos RON, funciones Lua, protocolos de presentación ni tareas de
implementación. El documento técnico y el plan se escribirán después de revisar
estas mecánicas. Los ejemplos ilustran situaciones, no agregan contenido al mundo.

Amplía el [diseño general](../diseno-general.md) para estos encuentros; no cambia
las reglas vigentes de Viajar o Acampar. Las reglas confirmadas en la conversación
se distinguen de las propuestas y preguntas pendientes de la sección 16.

## 2. Qué juega y qué observa el jugador

El jugador elige Atacar y un objetivo válido. Durante el episodio observa un
escenario de tiles y fichas que actúan automáticamente bajo las reglas. No elige
cada casilla, golpe o maniobra individual. Las decisiones que se le presenten
conciernen al curso del episodio: mantener un intento, cambiar el enfoque cuando
existan alternativas o retirarse.

Cada viñeta de acciones conserva una descripción breve mientras se representan
sus acontecimientos. Por ejemplo: «El grupo se escabulle por el pasillo» o «Los
guardias cierran el paso y entrás en combate». Los desplazamientos y ataques
individuales se ven en el tablero sin producir una sucesión de párrafos.

Las características importan tanto para la resolución como para las posibilidades
que aparecen. Una misma situación puede abrir caminos distintos para grupos
distintos. Una cualidad no garantiza siempre una ventaja: persistir puede sostener
una defensa o retrasar una retirada necesaria. La interpretación particular de
esas consecuencias pertenece a las reglas de la viñeta y a los eventos escritos.

## 3. Acciones generales que usan encuentros

### Atacar: alcance confirmado

- El grupo intenta matar a un actor o monstruo concreto.
- Solo se ofrecen objetivos situados en el mismo Lugar del mundo que el grupo.
  Compartir Lugar no implica comenzar a distancia de combate.
- Un objetivo ya muerto no se ofrece como blanco de Atacar.
- La preparación y la aproximación al objetivo forman parte de la acción.
- Puede haber enemigos y obstáculos antes de llegar al objetivo.
- Eliminar al objetivo puede terminar el episodio o dejar una salida pendiente.
- Atacar un Lugar para destruirlo, conquistarlo o saquearlo queda aplazado.

La condición de compartir Lugar es necesaria. No se agrega en este documento una
regla que revele automáticamente objetivos desconocidos; cómo se conocen y se
ofrecen en el selector se revisará al completar las reglas de información.

### Rescatar: finalidad prevista, desarrollo posterior

Rescatar o recuperar tendrá como propósito alcanzar una persona o un bien y
sacarlo de una situación. Compartirá escenarios, fichas, movimiento, oposición y
viñetas con Atacar. Alcanzar a la persona o al tesoro no bastará si todavía falta
extraerlo del escenario.

Esta primera entrega de diseño se concentra en Atacar. Las condiciones de una
persona rescatable, su cooperación, el transporte de un tesoro y sus desenlaces
requieren una ampliación propia antes de implementar Rescatar. Otras acciones
podrán utilizar encuentros cuando sus reglas lo requieran.

## 4. El escenario existe durante el episodio

Al comenzar se genera un escenario local apropiado al Lugar y a la situación.
Permanece estable durante ese encuentro, salvo los cambios producidos por sus
acontecimientos. Terminado el episodio, se descarta. Otra acción en el mismo
Lugar puede generar otra disposición en esta etapa del desarrollo.

Las habitaciones, posiciones y objetos locales no pasan a convertirse en nuevos
Lugares del mapa mundial. La generación no exige dibujar previamente un archivo
de mapa para cada ciudad, casa o cueva. Los mapas particulares persistentes y
diseñados a mano son una posibilidad futura.

Antes de descartar el escenario se conservan los hechos que afectan a la partida:
muertes, heridas y otros estados reglados, prisioneros, separaciones, cambios de
ubicación y consecuencias narrativas. Descartar el escenario no revive personajes
ni anula la derrota. Las alteraciones que deban afectar a encuentros posteriores
necesitarán su propia consecuencia persistente; no se deducen de un mapa eliminado.

## 5. Áreas, terreno y elementos

Un escenario está formado por áreas tileadas conectadas por accesos. Un área
puede ser una habitación, patio, tramo de calle o espacio agreste. Su forma no
está obligada a ocupar un rectángulo completo de suelo transitable.

| Elemento inicial | Regla de juego |
|---|---|
| Suelo | Ofrece apoyo y permite ocupar la casilla si no hay otro impedimento. |
| Pared o límite cerrado | Impide atravesar esa parte del escenario. |
| Obstáculo | Ocupa espacio e interrumpe recorridos; el primer reglamento no incluye intentos generales para moverlo o destruirlo. |
| Vacío | Carece de apoyo; no se cruza caminando. No equivale a agua ni a una casilla todavía desconocida. |
| Acceso | Comunica áreas; requiere alcanzar su posición y que el paso esté disponible. |
| Salida del encuentro | Permite abandonar el episodio cuando se cumplen sus condiciones. |

Los obstáculos pueden representar después muebles, rocas, árboles o columnas.
Su apariencia no les concede automáticamente reglas de cobertura, fuego o
resistencia estructural. Una mesa solo protege de flechas cuando exista esa regla.
Un acontecimiento narrativo sí puede mover, destruir o incendiar elementos desde
el primer episodio; lo aplazado es la simulación general de esos intentos.

Un vacío obliga a buscar otro recorrido, rodear o usar un acceso estrecho de
suelo. El diseño admite saltar por encima cuando se incorporen las reglas de
salto, con límite de distancia y destino válido. No todos los vacíos serán
saltables. El primer episodio no depende de una tirada de salto: siempre tiene
un recorrido continuo que permite alcanzar su objetivo y su salida.

La generación debe proporcionar recorridos, accesos y posiciones utilizables;
la dificultad inicial nace de la oposición y la disposición del terreno, no de
un objetivo accidentalmente inaccesible. Los impedimentos introducidos durante
el episodio sí pueden obligar a replantear o abandonar el intento.

## 6. Fichas y participantes

Una ficha representa una presencia en el escenario. Tiene una posición local y
un estado relevante para el encuentro. Si representa a un Actor del mundo,
conserva su identidad y sus capacidades; no crea otra persona con la misma biografía.

Las posiciones de tablero pertenecen a las fichas. El Actor no necesita tener
una posición de combate cuando no participa en un encuentro. Los elementos
locales, animales o defensores anónimos se distinguirán de los Actores persistentes
cuando se concrete su autoría; incorporar una ficha no crea necesariamente un
nuevo personaje importante para la historia.

Una pieza activa no atraviesa otra pieza que bloquea el paso. La forma de ceder
paso entre aliados y la ocupación de cuerpos incapacitados se fijarán junto con
el reglamento inicial de movimiento. No se presupone que dos personajes puedan
compartir una casilla o intercambiar posiciones instantáneamente.

## 7. Movimiento y zona de hostilidad

### Recorrido real por casillas

El desplazamiento ordinario atraviesa una secuencia de casillas. En cada paso
importan el apoyo, los obstáculos, la ocupación y los enemigos capaces de reaccionar.
El movimiento que se representa conserva ese recorrido y sus interrupciones.

Fuera de una situación peligrosa se puede resolver un desplazamiento largo en
conjunto, siempre que no se omitan detecciones, obstáculos o reacciones que deban
ocurrir. Infiltrarse no elimina la necesidad de considerar a los observadores.
Cuando hay oposición inmediata, las piezas alternan desplazamientos y acciones
según un orden interno. El jugador no debe confirmar cada uno de esos pasos.

### Ataque de reacción: regla confirmada

Un enemigo capaz de combatir amenaza las casillas próximas desde las que puede
alcanzar a otra ficha. Intentar abandonar cualquier casilla amenazada habilita
un ataque de reacción del enemigo elegible, aunque la casilla de destino siga
dentro de la zona de amenaza de ese mismo enemigo. La resolución es:

1. Se reconoce el intento de movimiento antes de que abandone la casilla.
2. El enemigo elegible realiza el ataque según sus reglas.
3. Si falla, el paso puede continuar, sujeto a otros impedimentos o reacciones.
4. Si acierta, interrumpe ese movimiento y la ficha permanece en la casilla de
   origen. También se aplican las consecuencias del golpe que establezca su regla.

El acierto no devuelve a la ficha al comienzo de toda su ruta: conserva los
pasos ya realizados. Un ataque de reacción es una resolución, no un golpe
adicional garantizado. Entrar en una posición próxima al enemigo permite
alcanzarlo, pero puede dificultar luego alejarse. Ambos adversarios pueden
amenazarse mutuamente.

Entrar en una casilla amenazada no genera por sí mismo una reacción bajo esta
regla de salida. Un defensor no alcanza a través de una pared o un obstáculo que
impida el contacto. Un personaje muerto o incapacitado no puede reaccionar.

Esta regla permite proteger accesos y compañeros vulnerables mediante posiciones
concretas. No garantiza que una primera línea sea impenetrable: un enemigo puede
encontrar otro recorrido o conseguir atravesar el peligro.

### Detalles que se revisan antes del reglamento definitivo

Deben fijarse la vecindad y las diagonales, las reacciones disponibles
por enemigo y cuándo se recuperan, el orden entre varios defensores y las
condiciones para retirarse con cuidado. La propuesta para evitar cadenas
ilimitadas figura en la sección 16. Reintentar una salida detenida debe consumir
una nueva oportunidad de actuar; no permite infinitos reintentos instantáneos.

Las transiciones narrativas explícitas no simulan un desplazamiento ordinario
y no generan automáticamente una reacción por cada casilla omitida. El escritor
determina sus consecuencias y debe mantener su coherencia con el acontecimiento.

## 8. Preparación, capacidades y resolución del combate

El grupo se prepara desde sus capacidades y circunstancias presentes. Sus
integrantes pueden proponer un enfoque cuando existe una alternativa relevante.
La viñeta elige responsables e intentos adecuados; las acciones particulares
resuelven lo que esos intentos consiguen. No se presume un menú de equipamiento
ni un sistema de inventario que todavía no se diseñó.

El combate cercano puede incluir varios atacantes, defensas, interrupciones y
movimientos. El reglamento deberá distinguir fallar un golpe, herir, quedar fuera
de combate y morir. Ninguna narración puede tratar esos resultados como equivalentes.

La regla de ataque que resuelve un golpe ordinario se utiliza también para los
ataques de reacción, con las diferencias que se acuerden explícitamente. Tener
capacidades parecidas no obliga a combatir indefinidamente. Debe existir una
forma reglada de reconocer un bloqueo, cambiar de intento o retirarse.

Las características proporcionan capacidades o modificadores conforme a las
reglas concretas. No se añaden características específicas de cada escena para
convertir cualquier encuentro en una cerradura con una única llave.

La fórmula del golpe, sus modificadores, sus efectos y el ritmo de actuación
siguen pendientes. Esta edición no inventa puntos de vida, daño, valores de armas
ni probabilidades como si ya hubieran sido acordados. La sección 16 delimita qué
falta antes del documento técnico que gobierne una implementación jugable.

## 9. Las viñetas de acciones

Cada viñeta tiene una situación de entrada, un propósito, participantes y formas
de concluir. Coordina los intentos que desarrolla y puede introducir cambios
propios según sus reglas. Su conclusión devuelve una situación nueva: la
siguiente viñeta responde a ella, sin imponer una secuencia indiferente al resultado.

La preparación, la aproximación y la confrontación son funciones recurrentes del
episodio. No todas necesitan una pantalla separada ni un orden invariable.

### Prepararse

El grupo considera el objetivo y adopta el enfoque disponible. El jugador conoce
el propósito, los participantes y la oposición que el grupo sabe que existe.
Un evento particular puede añadir un juramento, disfraz o decisión histórica.
No se produce genéricamente la decisión de quitarse una armadura o recrear un
suceso único de una leyenda.

### Aproximarse y penetrar

Las fichas se desplazan hacia accesos y posiciones útiles. Si intentan hacerlo
sigilosamente, sus intentos se resuelven con las reglas de infiltración que se
definan. El grupo puede ser descubierto antes de llegar al objetivo. Sus
integrantes pueden quedar separados y el episodio debe reconocerlo.

### Afrontar oposición

El grupo encuentra defensores u otros impedimentos. La viñeta coordina el
combate y el movimiento permitidos. Superar una guardia no implica haber
eliminado al objetivo. Ser descubierto puede transformar la oposición posterior.

### Alcanzar y atacar al objetivo

El grupo intenta matar al blanco de la acción. El objetivo puede defenderse,
recibir ayuda, huir o vencer al grupo. La ubicación y la capacidad para alcanzarlo
importan. No se aplica una muerte automática por haber llegado a su habitación.

### Salir o retirarse

El grupo intenta llegar a una salida. Una orden de huir inicia ese esfuerzo;
no transporta automáticamente a todos fuera del escenario. La hostilidad y la
ocupación siguen siendo relevantes. Si alguien queda atrás, se conserva ese
hecho y se ofrece una decisión de conjunto cuando sea significativa.

El episodio concluye cuando se ha logrado la salida requerida o se confirma otro
desenlace. No se exige eliminar a cada defensor como condición universal.

## 10. Dirección de las fichas y decisiones del jugador

El comportamiento automático persigue el propósito vigente utilizando la
situación y las capacidades de los participantes. No necesita descubrir por su
cuenta todos los trucos que un escritor puede imaginar. Los acontecimientos
particulares aportan esas oportunidades y transformaciones.

El jugador recibe decisiones al cambiar materialmente la situación, no una lista
de órdenes por personaje en cada casilla. Una retirada pendiente, un compañero
separado o una alternativa propuesta por alguien pueden justificar una pausa.
No se exige que el jugador optimice manualmente el orden de todos los golpes.

Los rivales actúan bajo las mismas reglas de desplazamiento y combate que los
integrantes del grupo. Pueden tener otros propósitos y disposiciones. La reacción
de unos guardias a la muerte de su jefe depende del encuentro: no todos huyen
ni todos combaten hasta morir por una regla universal.

## 11. Acontecimientos y eventos particulares

Una viñeta genérica puede producir acontecimientos conforme a su lógica. Un
evento escrito puede añadir sucesos propios de un encuentro: abrir una compuerta,
incorporar dos tigres, incendiar una cortina, derribar una pared o narrar una
escapatoria que traslada al grupo a otra área.

Se admiten dos formas de transformación:

- **Intento reglado:** alguien trata de hacer algo y acepta su resolución,
  incluyendo fallo, coste e interrupción.
- **Acontecimiento narrativo:** establece hechos y consecuencias particulares
  sin reproducir todos los movimientos y chequeos que podrían explicarlos.

Ambas transforman el mismo escenario. La libertad narrativa conserva identidades,
posiciones válidas y consecuencias ya confirmadas. Después de una transformación,
los participantes actúan desde la situación nueva. Un plan de movimiento
incompatible con una pared recién caída o un traslado no se continúa como si
nada hubiera sucedido.

No hace falta describir por adelantado todos los objetos y cuartos. Un evento
puede usar algo existente, precisar un elemento que seguía sin definir o añadir
un elemento nuevo. Los comodines son una posibilidad de autoría, no objetos
obligatorios de todos los mapas. Una cortina que ya se identificó conserva su
identidad mientras no ocurra algo que la transforme.

Revelar guardias ocultos y añadir nuevos guardias son acontecimientos distintos.
La presentación los distingue. Lo que el jugador ya comprobó no se contradice
silenciosamente: los cambios inesperados se muestran o narran como tales.

Los sucesos se insertan entre viñetas y en momentos definidos dentro de ellas.
No se alteran retrospectivamente ataques ya resueltos. Los acontecimientos
relacionados se consideran juntos: abrir una compuerta y colocar sus tigres debe
dejar una situación completa desde la que continuar.

## 12. Qué se muestra al jugador

El escenario ocupa la vista del encuentro. La representación permite reconocer
a los participantes cuya identidad se conoce. Se distinguen suelo,
obstáculos, vacíos, accesos y participantes hostiles de forma consistente.

El texto de una viñeta permanece visible durante su desarrollo. Los movimientos,
contactos, ataques fallidos e interrupciones se representan gráficamente; no
reemplazan la descripción a cada instante. Un resumen breve al concluir señala
consecuencias importantes que la animación por sí sola podría dejar ambiguas:
objetivo muerto, compañero incapacitado, captura o cambio de propósito.

Las opciones generales se muestran cuando corresponde una decisión. La información
de detalle puede explicar resultados y reglas sin interrumpir cada paso.

La representación se percibe continua, pero la velocidad visual no altera las
reglas ni el tiempo de la partida. El jugador puede pausar la presentación o
acelerarla. Omitir su reproducción no omite decisiones ni consecuencias.

Los desplazamientos visibles conservan los pasos reales. Una escapatoria
establecida por un evento puede representarse mediante una transición entre
áreas y su texto; no se inventa un recorrido reglado que nunca se resolvió.

La posición de enemigos desconocidos no se muestra por anticipado. El sistema
inicial de información de áreas y detección se definirá sin presuponer que la
visibilidad del mapa mundial sirve como percepción de combate.

## 13. Desenlaces y continuidad de la partida

Se conservan por separado lo ocurrido con el objetivo y lo ocurrido con cada
participante. Matar al objetivo y volver todos con vida son hechos diferentes.

| Desenlace | Qué concluye y qué conserva |
|---|---|
| Objetivo eliminado, sin oposición pendiente | Termina el ataque; se confirma su muerte y los demás cambios. |
| Objetivo eliminado, salida peligrosa | Se mantiene el logro y comienza la salida del escenario. |
| Retirada antes de alcanzar al blanco | Termina al lograr salir; el objetivo sigue vivo. |
| Objetivo que escapa | Se registra su supervivencia y su salida; el grupo resuelve su propia situación. |
| Derrota con supervivientes | Se conservan heridas, separaciones y la forma concreta de concluir el encuentro. |
| Captura | Se identifica quién queda prisionero y de quién; no equivale a muerte. |
| Muerte de los participantes | Se conservan las muertes y se entrega ese resultado a la partida. |

El encuentro puede combinar resultados: objetivo eliminado y supervivientes
capturados, por ejemplo. La fuga del blanco no inventa todavía una persecución
por varios Lugares del mundo. Si sobrevive debe resolverse su ubicación de una
forma válida antes de descartar el tablero; su destino concreto forma parte de
las reglas pendientes de cierre.

Atacar no impone por sí mismo el fin de la partida al morir el personaje jugador.
La continuidad con otro personaje y los finales narrativos pertenecen a las
reglas generales de campaña que correspondan. La misma separación se aplica a
los cautiverios particulares: la captura es un hecho genérico sobre el que el
diseñador puede construir después una historia singular.

## 14. Primer episodio jugable y ampliaciones

### Alcance inicial acordado

- Atacar a un actor o monstruo del mismo Lugar.
- Pocas áreas sencillas conectadas, con posiciones locales y fichas individuales.
- Suelo, obstáculos y vacíos que obliguen a rodear; un recorrido utilizable.
- Preparación y aproximación, combate cercano y desenlace del intento.
- Movimiento automático por casillas y ataques de reacción que pueden detenerlo.
- Viñetas con texto general estable y representación gráfica de sus acontecimientos.
- Intervención de eventos particulares sobre el escenario.
- Escenario temporal y consecuencias conservadas en la partida.

Como episodio de prueba se propone una entrada, un acceso con obstáculos y una
sala del objetivo. La disposición ofrece un camino estrecho y un rodeo alrededor
de un vacío; un defensor puede proteger el acceso. Hay pocas fichas. Se verifica
si la colocación de los defensores importa, si la aproximación conduce al
combate de forma clara y si la salida produce un desenlace comprensible.

El caso concreto, sus participantes y sus reglas cuantitativas se revisarán antes
de convertirlo en el primer incremento. Es una situación de prueba propuesta,
no una nueva historia canónica de Zahrim o de otro personaje.

### Entrega por ciclos

El alcance inicial anterior se construye mediante varias entregas jugables. La
[hoja de ruta](../planes/encuentros-atacar-rescatar.md) organiza esos ciclos sin
cambiar las reglas. El primer ataque completo utiliza el pequeño repertorio
aprobado que necesite; las transformaciones narrativas locales y la variedad de
escenarios se incorporan después. Cada entrega conserva los desenlaces que sus
reglas puedan producir, incluidos los desfavorables.

Los valores y decisiones todavía pendientes de la sección 16 se cierran antes
de implementar la parte del reglamento que los utilice. Los siguientes ciclos
amplían los diseños vivos; no requieren copiar todo este documento.

### Familia de escenarios a incorporar gradualmente

| Familia | Referencias de entorno |
|---|---|
| Agreste | Campo, desierto, bosque y montaña. |
| Civilizada | Calle, casa y torre. |
| Ampliaciones posteriores | Mansión, cueva, fortín, muralla, alcantarilla, costa, puerto y otras combinaciones. |
| Naval, aplazada | Cubiertas de barcos, abordajes y ataques desde el mar. |

El Lugar mundial orienta la elección del escenario. El primer episodio no exige
que todas las ciudades, bosques y montañas dispongan ya de una variante propia.
Se incorporan familias y reglas después de probar el episodio completo.

Salto, trepar, nadar, arquería, lanzamiento de objetos, fuego con efectos propios,
fauna incidental y destrucción del terreno se añaden mediante reglas posteriores.
No se prometen sus posibilidades mecánicas por el mero hecho de dibujarlas.

## 15. Verticalidad futura

El alcance inicial usa áreas planas. El concepto de escenario admite después
varios niveles: pisos de un edificio, balcones, techos y otros desniveles.

- Un nivel superior puede ocupar la misma posición horizontal que otro inferior.
- Cambiar el nivel observado no mueve las fichas ni altera el nivel en que están.
- Los niveles se comunican mediante accesos explícitos, como escaleras.
- La presentación podrá ocultar techos y paredes que impidan observar el interior.
  Ocultar una pared para mirar no elimina su efecto físico.
- Saltar, caer, trepar y atacar entre alturas tendrán reglas propias.
- La generación y las viñetas podrán referirse a un área o nivel concreto;
  no se presume que todo acceso está en el mismo plano.

Estas condiciones fijan el alcance futuro que debe respetar el diseño técnico.
No exigen implementar ahora cámara por niveles, pisos superiores, tiro vertical,
simulación estructural de edificios ni una matriz tridimensional completa.

## 16. Propuestas y decisiones todavía abiertas

La base anterior recoge la intención acordada. Para completar el reglamento del
primer episodio quedan estas decisiones; no se trasladarán como incógnitas al
código:

1. **Ritmo, geometría y límite de reacciones.** Se propone para revisión movimiento
   ortogonal inicial, alcance cercano de una casilla, orden de actuación interno
   y una reacción por enemigo en cada ronda interna. La ronda no requiere órdenes
   del jugador. Un primer acierto cancela el paso y las reacciones que dependían
   de ese mismo intento. Las diagonales y sus esquinas se incorporan cuando se
   acuerde cómo afectan al movimiento y a la amenaza.
2. **Golpes y estados.** Definir la resolución de ataque y defensa, qué
   características intervienen y cómo se producen heridas, incapacitación y
   muerte. No se establece en este borrador una fórmula arbitraria de daño.
3. **Comportamiento básico e información.** Fijar cómo el grupo adopta un enfoque,
   cómo detectan los guardias y cómo se elige entre avanzar, auxiliar y retirarse.
   Se propone una aproximación inicial y decisiones de conjunto al cambiar la
   situación; no un planificador capaz de inventar cualquier estrategia.
4. **Cierre y tiempo.** Fijar las duraciones mínimas, la ubicación del blanco que
   escapa y las condiciones genéricas para la captura. La transición al tiempo
   mundial se define a partir de esas reglas, sin depender de la animación.

La revisión de estas cuestiones completa las mecánicas del primer incremento.
Después corresponde redactar su diseño técnico y, finalmente, su plan. El primer
desarrollo tendrá un episodio jugable; las siguientes iteraciones ampliarán
escenarios y reglas sobre lo que ya se haya podido probar.
