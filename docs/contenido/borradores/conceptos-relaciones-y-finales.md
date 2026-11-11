# Borrador: conceptos, relaciones y finales particulares

| Campo | Valor |
|---|---|
| Estado | Diseño conceptual en elaboración; no modifica código ni RON |
| Versión | 0.2 |
| Fecha | 2026-11-11 |

## 1. Base del diseño

El usuario propone expresar los deseos, deberes, necesidades y llamadas mediante
relaciones del grafo y añadir **Conceptos** como posibles destinos. Sustituye
la propuesta anterior de una ficha separada de «trayectoria del personaje».
Los vínculos ya son relaciones; no se duplican en otra clasificación.

Estas relaciones tienen una relevancia particular para comprender a un Actor,
orientar al jugador y escribir sus finales. Se aplican al mismo personaje tanto
cuando lo controla el jugador como cuando actúa como NPC. No se aprueba aquí
un nuevo motor de objetivos, una psicología automática ni el formato técnico.

## 2. Relaciones propuestas por el usuario

Las etiquetas se redactan como aparecerían en la ficha. Los ejemplos son
posibilidades de contenido, no relaciones incorporadas al juego.

| Relación | Lo que expresa | Ejemplo de destino conceptual |
|---|---|---|
| Desea | Algo que el personaje quiere conseguir, conservar o realizar. | Recuperar el trono del Drâa. |
| Debe | Una obligación que pesa sobre el personaje. | Proteger al príncipe. |
| Necesita | Algo requerido por una situación que lo ha puesto en dificultades, incluso por causas ajenas a él. | Recuperar su libertad para volver con los suyos. |
| Es llamado a | Un destino, llamada o impulso que lo lleva hacia algo, aunque preferiría no verse empujado en esa dirección. | Regresar al lugar de un antiguo pacto. |

El deber puede presuponer disposición a cumplirlo, según la explicación del
usuario. No obliga a declarar también un deseo idéntico ni borra los conflictos
con otros deseos. La necesidad tiene que resultar comprensible en su contexto:
para qué se necesita algo y qué está amenazado por esa carencia.

El usuario considera que una compulsión, como una adicción dentro de una
historia, podría expresarse también mediante una llamada. Eso no equipara sus
causas a las de una profecía o mandato sobrenatural. Los sucesos concretos
determinarán de qué fuerza se trata y cómo influye. Que sea resistible o
ineludible pertenece a las reglas y acontecimientos de ese caso, no a una
consecuencia universal de declarar la relación.

Las cuatro relaciones no son categorías excluyentes de personas. Un Actor puede
tener varias, incluso hacia un mismo Concepto o hacia cosas que entren en
conflicto. No se establece que todos deban tener las cuatro.

## 3. Concepto: un elemento que sirve de objeto de relación

**Definición del usuario:** un Concepto puede ser una idea simple o compleja,
incluso una acción expresada como idea. Su único propósito es poder ser objeto
de una relación. Permite mostrar y entender aquello a lo que está vinculado
un personaje.

Para escribirlo y reconocerlo se proponen únicamente una clave estable, un
nombre legible y una descripción. Por ejemplo:

| Clave de ejemplo | Nombre | Descripción de ejemplo |
|---|---|---|
| `recuperar_trono_del_draa` | Recuperar el trono del Drâa | Restablecer al príncipe en el trono que ocupaba su padre antes de la usurpación. |
| `proteger_al_principe` | Proteger al príncipe | Preservar su vida y su seguridad frente a quienes lo persiguen. |
| `recuperar_la_libertad` | Recuperar la libertad | Dejar de estar retenido o sometido a una custodia que impide decidir el propio rumbo. |

Son muestras para discutir, no un esquema RON ya aprobado. La clave distingue
ideas y el texto explica su sentido al diseñador y al jugador. Pueden existir
Conceptos generales o particulares; no es necesario una interpretación universal
de «proteger» que resuelva todos los contextos.

Un mismo Concepto puede ser objeto de relaciones de distintos Actores.
Que Farid recupere la libertad no resuelve la necesidad de otra persona que
apunte al mismo Concepto: los hechos se interpretan en el contexto de cada una.

Un Concepto no tiene posición, medios, inventario ni conducta. «Recuperar el
trono» no inicia una acción general, no contiene un plan y no sabe comprobar
por sí mismo si se ha cumplido. Tampoco tiene progreso o estados de misión.
Las operaciones, condiciones y consecuencias siguen en las reglas y eventos
escritos para el mundo.

En la ficha del Actor se propone mostrar, por ejemplo, **Desea: Recuperar el
trono del Drâa**. Seguir la referencia permite leer el nombre y la descripción
del Concepto mediante la navegación de fichas. No lo convierte en un destino
de Viajar ni en una tarea ejecutable desde esa ficha.

## 4. Aplicación a los personajes y sus finales

Ejemplos propuestos de relaciones:

- Príncipe —Desea→ Recuperar el trono del Drâa.
- Sirviente —Debe→ Proteger al príncipe.
- Farid —Necesita→ Recuperar la libertad.
- Rey tigre —Debe→ Encontrar la estrella perdida.

El mandato del rey de la luna ya forma parte del relato del rey tigre; añadir
una llamada distinta necesitaría establecer esa fuerza en la historia. No se
le atribuye automáticamente solo por tener un deber.

Estas relaciones ayudan a seleccionar Actores con algo concreto en juego,
junto con sus relaciones familiares, sociales e históricas. El criterio se
probará con pocos personajes; no obliga a completar toda familia o ciudad ni
a escribir una campaña exclusiva para cada uno. Farid y los nuevos personajes
de Sahran siguen siendo propuestas de contenido.

La primera aplicación a un reparto concreto se desarrolla en
[Personajes de Zarabad y Nuris](./personajes-zarabad-nuris.md), con pocas
relaciones por persona y una nota breve de vida propia. No se obliga a que cada
Actor tenga todos los tipos de relación ni una historia igualmente compleja.

Los finales se escriben para personas y acontecimientos particulares. Pueden
consultar estas relaciones, otros vínculos y los hechos de la partida para
decidir qué textos corresponden. No basta contar deseos satisfechos, sumar
puntos ni declarar resuelto un Concepto. La condición de un final sigue siendo
autoría del diseñador, con las operaciones que ofrezca el juego.

Todo final del príncipe debe considerar si recuperó el trono, aunque el jugador
persiga otros propósitos. No conseguirlo no implica una derrota automática ni
demuestra por sí solo que renunció, lo olvidó o murió arrepentido. Cada desenlace
puede combinar logros, pérdidas y asuntos irresueltos, con frecuencias y resultados
desiguales. La pérdida del sirviente puede llevar habitualmente a peores resultados,
como ya se conversó.

Las relaciones pueden cambiar con los sucesos. Quitar o sustituir una relación
actual no debe hacer olvidar un acontecimiento que sigue importando al final.
Habrá que conservar los hechos relevantes cuando se diseñe esa implementación;
no se carga al Concepto con un historial ni un algoritmo de evaluación.

Se conservan las posibilidades conversadas de retiro voluntario, epílogos y
cierres de época. Sus reglas y su disponibilidad aún no están elegidas. El
epílogo puede combinar hechos de la partida con un futuro particular escrito
según su situación al terminar.

## 5. Motivos de relatos para afinar las distinciones

Se han vuelto a consultar estas traducciones el 2026-11-11. Las correspondencias
son una lectura de diseño propia; los relatos no proponen esta clasificación
de cuatro relaciones ni una teoría universal de la mitología.

| Relación | Motivo que permite estudiarla |
|---|---|
| Desea | En el libro V de la [Odisea, traducción de Samuel Butler](https://www.gutenberg.org/files/1727/1727-h/1727-h.htm), Odiseo afirma ante Calipso que quiere regresar a su hogar aun con los peligros del viaje. |
| Debe | En [Ruru, Jātakamālā 26, traducción de J. S. Speyer](https://ancient-buddhist-texts.net/English-Texts/Garland-of-Birth-Stories/26-The-Story-of-the-Ruru-Deer.htm), el hombre rescatado promete guardar el secreto del ciervo. El compromiso permite examinar deber, deseo y traición. |
| Necesita | En [Anpu y Bata, traducción de W. M. Flinders Petrie](https://www.gutenberg.org/files/7413/7413-h/7413-h.htm), la restitución de la vida de Bata depende de recuperar lo que la traducción llama su alma y devolverlo mediante agua. Es una necesidad impuesta por su condición, no un simple deseo de posesión. |
| Es llamado a | En el libro IV de la [Eneida, traducción de John Dryden](https://www.gutenberg.org/files/228/228-h/228-h.htm), Eneas diferencia su elección personal de los mandatos y el destino que lo llevan hacia Italia. |

La terminología de alma y corazón en la traducción de Petrie se explica entre
las [referencias anteriores](./drama-y-mitos.md#7-referencias-y-cuestiones-para-seguir-explorando).
Se toma el motivo del relato, no sus generalizaciones históricas sobre pueblos.
Las relaciones propias podrán enriquecerse con otros relatos sin trasladar
automáticamente sus moralejas ni sus desenlaces.

## 6. Alcance respecto del juego actual

El grafo actual reconoce Actores y Lugares como extremos. Agregar Conceptos y
estos cuatro tipos de relación requiere extender el modelo, la carga de contenido,
la validación y la navegación de fichas. Se reutiliza el grafo; no se supone que
baste añadir una palabra nueva a los RON para que ya funcione.

Este borrador no implementa esas extensiones ni una selección de finales.
Tampoco modifica las propuestas de cautividad, reclutamiento y **Esperar órdenes**
en [Sahran](./sahran-apertura-y-personajes.md#servicio-naval-y-avance-del-tiempo).
Esas situaciones siguen representando hechos y posibilidades de acción que
pueden influir en deseos, deberes y necesidades, sin confundirse con ellos.
