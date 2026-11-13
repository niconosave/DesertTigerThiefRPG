# Borrador: personajes de Zarabad y Nuris

| Campo | Valor |
|---|---|
| Estado | Actores, Conceptos y relaciones implementados en RON; eventos pendientes |
| Versión | 0.2 |
| Fecha | 2026-11-11 |

## 1. Criterio y alcance

El usuario encarga un reparto pequeño para la guerra de los dos puertos y la
llegada del príncipe. Se usan relaciones sociales y relaciones
[Desea, Debe y Necesita hacia Conceptos](./conceptos-relaciones-y-finales.md).
Son la base para escribir después condiciones, decisiones y consecuencias de
eventos particulares. No se intenta deducir una conducta completa del grafo.

Cada persona tiene una presentación breve y unas pocas relaciones importantes.
La nota **Vida propia** reserva espacio a su sentido fuera de la historia del
príncipe, sin exigirle una campaña exclusiva, una familia completa ni el mismo
interés dramático que a las demás. Estas diferencias también se conservarían
al jugar desde Nuris o al controlar a otro Actor.

Se proponen cuatro personajes ligados a Zarabad y cuatro a Nuris. Farid pertenece
al entorno de Nuris, pero comienza cautivo en Zarabad. Bandido, Samir y Neshra
ya propuestos conectan el reparto con otros lugares; no se agregan aquí otros
cortesanos ni familiares. Ruzan queda como reserva del borrador regional.

La guerra comercial, el destino en Zarabad y la venta del príncipe son la base
conversada. El usuario autorizó materializar este reparto, cambiando el personaje
de Dahran a un capitán masculino. Sus datos se cargan desde `content/base/actors/`
y los Conceptos desde `content/base/concepts/`. No se fija que la captura de Farid
haya iniciado la guerra: el detonante sigue abierto.

Las relaciones sociales y conceptuales de estas fichas ya son tipos disponibles
en el código. Un mismo hecho puede leerse desde ambos extremos sin declararlo
dos veces. La filiación y su lectura inversa, por ejemplo, permiten
mostrar al padre y a sus hijos.

## 2. Zarabad

### Zahrim

**Rey de Zarabad.** Orgulloso y calculador, entiende la prosperidad de sus muelles
como una demostración de su grandeza. Compra al príncipe porque necesita
combatientes: puede recompensar sus servicios y concederle autonomía sin pensar
que por ello haya dejado de pertenecer a su casa.

- **Gobierna →** Zarabad.
- **Enemigo de →** Miraz.
- **Desea →** Controlar la salida comercial del Mar del Viento.
- **Necesita →** Reforzar las tripulaciones de Zarabad.

**Vida propia:** recibió una ciudad pequeña y quiere que sea recordada como un
gran puerto. Su interés por la guerra y sus barcos existe antes del príncipe.

### Dahran

**Capitán al servicio de Zahrim.** Severo y poco impresionado por los títulos
de un extranjero, juzga a los recién llegados por lo que hacen a bordo. Puede
llegar a respetar al príncipe y seguir exigiéndole obediencia. Custodia a Farid,
capturado al interceptar un barco de Nuris.

- **Sirviente de →** Zahrim.
- **Fue salvado por →** Samir.
- **Debe →** Proteger a su tripulación.

La relación **Prisionero de** declarada desde Farid también la muestra como su
custodia. Su deber hacia la tripulación puede entrar en conflicto con órdenes
arriesgadas del rey o con la petición del hombre que la salvó.

**Vida propia:** un naufragio anterior le enseñó el valor de quienes la acompañan
en el mar. Ser capitán es su oficio y su posición en la ciudad.

### Sahmun

**Consejero de Zahrim.** Practica artes de adivinación aprendidas entre
navegantes. Su ambición es conocer lo que llega por mar antes de que aparezcan
las velas en el horizonte. Ofrece información en secreto a Miraz para conservar
recursos y contactos a ambos lados de la guerra; su utilidad no lo vuelve leal.

- **Sirviente de →** Zahrim.
- **Tiene un pacto secreto con →** Miraz.
- **Desea →** Conocer de antemano las llegadas por mar.

**Vida propia:** el palacio le proporciona medios para continuar sus estudios.
Todavía no se lo vincula al dragón ni a los magos del desierto; su práctica y
su ambición bastan para este conflicto local.

### Rima

**Sirvienta esclavizada de la casa de Zahrim.** Trabaja con los abastos del
palacio y conoce sus rutinas. Puede reconocer en el recién llegado otra persona
sometida y ofrecerle ayuda limitada si ve una oportunidad de mejorar también
su situación. No se presupone romance ni obediencia al príncipe.

- **Sirviente de →** Zahrim.
- **Necesita →** Recuperar la libertad.
- **Desea →** Abrir una casa de comidas en Shahraz.

**Vida propia:** lleva años organizando comidas ajenas y quiere sostenerse con
su propio trabajo fuera de la casa del rey. No necesita un linaje oculto para
tener una historia particular.

## 3. Nuris

### Miraz

**Gobernante de Nuris.** Ve en las exigencias de Zarabad una amenaza para su
puerto y una ofensa a su autoridad. Ofrece una alternativa a los comerciantes
del interior, pero también busca que esa actividad enriquezca su propia ciudad.
Contrata guerreros y protege salidas comerciales para disputar ese tráfico.

- **Gobierna →** Nuris.
- **Enemigo de →** Zahrim.
- **Desea →** Comerciar sin someterse a Zarabad.
- **Necesita →** Mantener abierto el tráfico de Nuris.

**Vida propia:** su posición depende de que el pequeño puerto siga proporcionando
sustento y oportunidades a sus vecinos. No se define toda su relación con ellos
por ser el rival del rey que capturó al príncipe.

### Zulema

**Encargada de contratar las escoltas de Nuris.** Organiza acuerdos entre el
gobernante, los mercaderes y quienes defienden sus barcos. Es rigurosa con los
compromisos: puede ofrecer al príncipe trabajo voluntario y una entrada al
bando de Nuris, con obligaciones concretas y sin prometerle ayuda para su trono.

- **Sirviente de →** Miraz.
- **Debe →** Cumplir los acuerdos de escolta.
- **Necesita →** Reunir guerreros para los convoyes de Nuris.

**Vida propia:** empezó llevando cuentas de comerciantes y vive de que sus
acuerdos sean fiables. Tiene un oficio que puede prosperar o arruinarse durante
la guerra.

### Bahir

**Carpintero de ribera de Nuris.** Repara barcas y barcos en un pequeño
astillero. La guerra le trae trabajo, pero también ha interrumpido entregas de
madera. Es padre de Farid y Samir; su familia ya está afectada por el cautiverio
del primero y por los riesgos que asume el segundo para ayudarlo.

- **Desea →** Conservar su astillero.
- **Necesita →** Restablecer el suministro de madera de Nuris.

La filiación declarada desde sus dos hijos permite mostrar su relación con
ellos. Sus intereses familiares y los de su oficio pueden coincidir o competir;
no se añade aquí otro encargo obligatorio de rescate.

**Vida propia:** quiere seguir construyendo y reparando barcos. Su trabajo y su
familia le dan un lugar en Nuris sin necesitar una intriga cortesana propia.

### Farid

**Marinero de Nuris, cautivo en Zarabad.** Trabajaba en los barcos de Miraz
cuando Dahran lo apresó. Necesita salir de esa situación para recuperar el
control de su vida; no se decide de antemano a quién servirá después ni cómo
responderá ante una ayuda particular.

- **Padre →** Bahir.
- **Sirviente de →** Miraz.
- **Prisionero de →** Dahran.
- **Necesita →** Recuperar la libertad.

**Vida propia:** procede de una familia ligada al mar y aprendió entre las
barcas de su padre. Liberarlo abre su vida a distintas decisiones; no lo convierte
automáticamente en compañero del príncipe.

## 4. Personas que conectan las ciudades con el territorio

Estas personas ya formaban parte de la propuesta regional. Aquí se precisan
solo los vínculos necesarios para conectar el nuevo reparto.

| Persona | Relaciones relevantes | Sentido propio mínimo |
|---|---|---|
| Bandido | Tiene tratos con Zahrim; conoce por negocios a Neshra; **Desea → Hacer fortuna**. | Busca oportunidades, sin fidelidad fija a una ciudad. Llevar al príncipe ante Zahrim forma parte de su negocio. |
| Samir, pescador de la costa | **Padre → Bahir**; salvó a Dahran; **Desea → Liberar a Farid**. | Tiene su propio sustento en la costa. La cautividad de su hermano lo obliga a pedir algo que su relación con el capitán vuelve posible, pero no seguro. |
| Neshra, comerciante de Shahraz | Mantiene tratos comerciales con ambos puertos; conoce al Bandido; **Desea → Conservar sus intercambios con los dos puertos**. | Quiere continuar su actividad sin quedar sometida por completo a uno de los rivales. |

Samir y Farid comparten padre; su fraternidad no requiere otra persona nueva
en este reparto. Las historias del naufragio de Dahran y de la captura de Farid
son dos sucesos distintos. No se los confunde con el naufragio inicial del
príncipe.

## 5. Conceptos usados en estas relaciones

Los Conceptos solo nombran y explican los objetos de relación. Cada clave de
esta tabla tiene su archivo en `content/base/concepts/`. Compartir un Concepto,
como Recuperar la libertad, no comparte automáticamente su resultado entre
Actores.

| Clave propuesta | Nombre | Descripción |
|---|---|---|
| `controlar_salida_mar_del_viento` | Controlar la salida comercial del Mar del Viento | Hacer que el comercio entre el interior y el exterior dependa de las condiciones impuestas por Zarabad. |
| `reforzar_tripulaciones_zarabad` | Reforzar las tripulaciones de Zarabad | Incorporar combatientes para sostener los barcos de Zahrim durante la guerra. |
| `proteger_tripulacion` | Proteger a su tripulación | Preservar a las personas que navegan bajo su mando durante las tareas y peligros del mar. |
| `conocer_llegadas_por_mar` | Conocer de antemano las llegadas por mar | Obtener mediante artes ocultas conocimiento de los barcos y peligros que se aproximan. No presupone que el intento tenga éxito. |
| `recuperar_la_libertad` | Recuperar la libertad | Dejar de estar retenido o sometido a una custodia que impide decidir el propio rumbo. |
| `abrir_casa_comidas_shahraz` | Abrir una casa de comidas en Shahraz | Establecer allí un pequeño negocio propio donde Rima pueda vivir de su trabajo. |
| `comerciar_sin_someterse_zarabad` | Comerciar sin someterse a Zarabad | Mantener una salida para Nuris y sus comerciantes que no dependa de la autoridad de Zahrim. |
| `mantener_trafico_nuris` | Mantener abierto el tráfico de Nuris | Sostener las entradas y salidas de barcos amenazadas por los ataques y la presión rival. |
| `cumplir_acuerdos_escolta` | Cumplir los acuerdos de escolta | Proporcionar la protección comprometida a quienes contrataron los servicios organizados por Zulema. |
| `reunir_guerreros_convoyes_nuris` | Reunir guerreros para los convoyes de Nuris | Conseguir personas que defiendan los barcos comerciales durante sus travesías. |
| `conservar_astillero` | Conservar su astillero | Mantener en funcionamiento el lugar del que Bahir obtiene su sustento y donde ejerce su oficio. |
| `restablecer_madera_nuris` | Restablecer el suministro de madera de Nuris | Recuperar las entregas necesarias para construir y reparar barcos en sus astilleros. |
| `hacer_fortuna` | Hacer fortuna | Obtener medios y ventajas mediante las oportunidades que encuentre el personaje. |
| `liberar_a_farid` | Liberar a Farid | Conseguir que Farid deje de estar cautivo de Dahran. |
| `conservar_intercambios_dos_puertos` | Conservar sus intercambios con los dos puertos | Continuar comerciando con Zarabad y Nuris pese a la guerra y las presiones de sus autoridades. |

No se fuerza una relación **Es llamado a** en este núcleo. Todavía no se ha
elegido para estas personas una llamada, destino o compulsión que la justifique.
La práctica de artes ocultas de Sahmun no supone por sí sola que tenga una.

## 6. Dinámicas que quedan disponibles para escribir eventos

- **Entrada por Zarabad:** Bandido lleva al príncipe a Zahrim; el rey lo destina
  al servicio de Dahran; Rima puede encontrar un motivo propio para ayudarlo.
- **Entrada por Nuris:** Zulema puede ofrecer un trabajo y compromisos con los
  barcos de Miraz. El jugador encuentra a las mismas personas del otro bando
  desde otra posición.
- **Cautiverio y deuda:** Farid, Samir y Dahran permiten escribir una petición
  o un intento de rescate donde pesan el servicio, la familia y una ayuda anterior.
- **Necesidades del puerto:** el suministro de Bahir y los intercambios de Neshra
  permiten viajes y disputas por mercancías que no requieren al príncipe como causa.
- **Información y deslealtad:** el pacto de Sahmun y Miraz puede interferir con
  los encargos de Zarabad. Sus consecuencias y las formas de descubrirlo se
  escribirán de manera particular.

Estas son posibilidades de autoría. No fijan una secuencia de escenas, un final
del conflicto ni una reacción automática a un favor. Los eventos deberán consultar
quién sigue vivo, dónde está, qué relaciones conserva y qué ocurrió antes.

## 7. Alcance implementado y pendientes

Se incorporaron ocho personas ligadas a las ciudades y dos contactos regionales
nuevos, Samir y Neshra. Bandido conserva su retrato y su lugar en el grupo, con
sus relaciones adaptadas al nuevo papel. Los NPCs nuevos tienen biografía, rasgos
y ubicación, sin retratos ni políticas automáticas. El inicio sigue en Hamada
Roja; el naufragio, la compra del príncipe, las órdenes y las restricciones de
cautividad son acontecimientos y mecánicas pendientes.

El [contrato técnico de Conceptos y relaciones](../../tecnica/componentes/25-conceptos-relaciones-personales.md)
describe el formato y su implementación.

«Pacto secreto» describe el hecho dentro de la ficción. Las fichas actuales no
ocultan relaciones por conocimiento; cómo se presenta o se descubre ese pacto
queda como detalle pendiente, sin añadir aquí un sistema de secretos.
