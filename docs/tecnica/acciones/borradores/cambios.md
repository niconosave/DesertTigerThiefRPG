changes
diff --git a//home/niconosave/Development/DesertTigerThiefRPG/docs/tecnica/acciones/borradores/05-viajar-redisenio.md b//home/niconosave/Development/DesertTigerThiefRPG/docs/tecnica/acciones/borradores/05-viajar-redisenio.md
--- a//home/niconosave/Development/DesertTigerThiefRPG/docs/tecnica/acciones/borradores/05-viajar-redisenio.md
+++ b//home/niconosave/Development/DesertTigerThiefRPG/docs/tecnica/acciones/borradores/05-viajar-redisenio.md
@@ -2,95 +2,87 @@
 
-> Estado: borrador en elaboración conjunta.  
-> Las reglas marcadas como **PENDIENTE** todavía necesitan una decisión de
-> diseño y no deben completarse por analogía con versiones anteriores.
+> Estado: borrador en elaboración conjunta. Las decisiones abiertas figuran en
+> [Pendientes](#pendientes); no deben completarse por analogía con versiones anteriores.
 
-## 1. Propósito de estas reglas
+## Índice
 
-`Viajar` comienza cuando el jugador decide que su Grupo se trasladará desde el
-Lugar donde se encuentra hasta otro Lugar del mundo. Como los Lugares están
-unidos por conexiones, alcanzar un destino lejano puede exigir recorrer una
-ruta formada por varios Lugares y puede haber más de una ruta posible.
+1. [Alcance, responsabilidades y estado](#alcance)
+2. [Curso del viaje](#curso)
+3. [Rutas y medios de transporte](#rutas)
+4. [Tiempo, reservas y consumo](#tiempo)
+5. [Carga y preparación de configuraciones](#carga)
+6. [Chequeos y selección de responsables](#chequeos)
+7. [Información y narración](#presentacion)
+8. [Tarjetas de viñeta](#tarjetas)
+9. [Decisiones pendientes](#pendientes)
+10. [Material retirado para revisión](#material-retirado) — fuera del cuerpo principal.
 
-Estas reglas cumplen la función que tendría el director de juego en un RPG de
-mesa. Conducen el viaje, plantean sus escenas, determinan cuándo corresponde
-hacer un chequeo e interpretan sus resultados. Sus decisiones no se improvisan
-durante la partida: quedan establecidas en este reglamento para que situaciones
-semejantes se resuelvan de manera coherente.
+<a id="alcance"></a>
+## 1. Alcance, responsabilidades y estado
 
-El reglamento describe la mecánica del juego y no la forma de programarla. La
-cantidad de ventanas necesaria para mostrar un texto, su paginado y otros
-detalles de interfaz no forman parte de estas reglas.
+`Viajar` comienza cuando el jugador decide trasladar su Grupo del Lugar actual a
+otro Lugar. El recorrido puede cruzar varias conexiones y admitir distintas rutas.
+Este reglamento cumple la función del director de juego: establece escenas,
+chequeos y consecuencias para resolver situaciones semejantes de manera coherente.
 
-## 2. Cómo se compone un viaje
+| Componente | Responsabilidad |
+|---|---|
+| Acción general `Viajar` | Planifica y conduce el recorrido; mantiene tiempo, provisiones, carga y medios disponibles; invoca las tarjetas. |
+| Tarjeta de viñeta | Define una clase de escena: disparador, información, decisiones, chequeos, consecuencias y continuación. |
+| Acción particular | Resuelve un chequeo reutilizable con el Actor y la dificultad que recibe; conserva sus propios modificadores y devuelve los datos de la tirada. No decide contexto, relato ni consecuencias. |
 
-La acción general `Viajar` conduce el viaje completo. Mantiene el recorrido,
-el tiempo transcurrido, el alimento, el agua, las monturas disponibles y la
-disponibilidad del barco del Grupo, y llama a las tarjetas de viñeta cuando
-corresponde representar una escena.
+Una viñeta de **resolución** resuelve la situación y después la relata, sin pedir
+decisiones ni fragmentarla para mostrar información que el jugador no puede usar
+para elegir. Una viñeta de **decisión** informa primero, permite elegir y aplica
+después la opción y sus consecuencias. El paginado y las ventanas son cuestiones
+de interfaz: no forman parte del reglamento ni determinan la cantidad de viñetas.
 
-Una **tarjeta de viñeta** contiene las reglas de una clase de escena. Explica
-cuándo ocurre, qué situación conoce el jugador, si debe tomar una decisión, qué
-chequeo se realiza y cómo sus resultados afectan el viaje. La tarjeta también
-establece qué debe comprender el jugador al terminar la escena.
+Durante la acción se mantienen la ruta adoptada, el Lugar actual, la composición
+del Grupo, el tiempo total, las horas desde la última ración, las comidas
+concretas, el agua y sus botas, los animales de carga, los contadores separados de
+caballos y camellos y la disponibilidad del barco. Son cuentas temporales: las
+reservas no ingresan al inventario general y todo remanente se descarta al terminar.
+Solo persisten cambios que una regla establezca expresamente, como Características,
+Relaciones o la separación y ubicación de Actores.
 
-Una **acción particular** resuelve un chequeo reutilizable. Recibe al Actor que
-interviene y la dificultad que el contexto fijó para ese intento, tira un dado
-de veinte caras y aplica los modificadores que pertenecen a esa misma acción.
-Después devuelve su resultado. La acción particular no decide por qué se hizo
-el chequeo ni qué consecuencias tiene en la historia: la tarjeta de viñeta
-interpreta el resultado dentro del viaje.
-
-Usar una misma acción particular en distintas acciones generales conserva sus
-bonificadores y penalizadores. La dificultad, el responsable, el Lugar, las
-consecuencias y el relato pertenecen en cambio al contexto que la invoca. La
-sección 8 reúne las acciones particulares que ya usa `Viajar` y distingue sus
-reglas reutilizables de las reglas propias de sus viñetas.
-
-Un resultado final igual a la dificultad la alcanza y cuenta como éxito, salvo
-que una tarjeta establezca expresamente otra cosa.
-
-En forma resumida:
+El reglamento no simula precios, dinero, comercio ni capacidad de pago. El Grupo
+puede pagar lo que necesita; conseguirlo sigue sujeto a los procedimientos de
+cada tarjeta. Las monturas y los animales de carga no son intercambiables.
 
-```text
-el jugador elige un destino
-→ Viajar conduce el recorrido
-→ una tarjeta plantea y resuelve una escena
-→ una acción particular produce el resultado de un chequeo, si hace falta
-→ la tarjeta aplica las consecuencias
-→ Viajar continúa con el recorrido
-```
+<a id="curso"></a>
+## 2. Curso del viaje
 
-## 3. Planificación de la ruta
+| Momento | Resolución |
+|---|---|
+| Destino | El jugador declara el Lugar al que quiere llevar al Grupo. |
+| Planificación | Se determinan las rutas rápida y larga y se busca un proponente válido. Sin él, se usa [`VIA-RUT-01`](#via-rut-01); con él, [`VIA-RUT-02`](#via-rut-02). |
+| Preparación | [`VIA-APR-01`](#via-apr-01) calcula y reúne provisiones. Se evalúan las configuraciones de carga y, si corresponde, [`VIA-APR-05`](#via-apr-05); se obtienen automáticamente los animales fijados. Si `CalcularProvisiones` fracasó, sigue [`VIA-APR-03`](#via-apr-03). |
+| Desplazamiento | Antes de cada conexión se elige el medio con los contadores actuales. Se presenta exactamente una tarjeta: `VIA-DES-NAV`, `VIA-DES-CAB`, `VIA-DES-CAM` o `VIA-DES-AND`. |
+| Cierre de escena | Se aplican tiempo, consumo y consecuencias. Salvo terminación expresa, el Grupo completa la conexión y su destino pasa a ser el Lugar actual. |
+| Destino final | Tras aplicar las consecuencias del último desplazamiento, se resuelve [`VIA-LLE-01`](#via-lle-01) y termina la acción. |
+| Entre conexiones | Si alguna reserva quedó en cero, se resuelve [`VIA-APR-02`](#via-apr-02). Una pérdida de capacidad también exige la [comprobación de carga](#recalculo-carga), que puede activar esa búsqueda. Si continúa el viaje, se revisa la carga resultante y puede seguir [`VIA-APR-06`](#via-apr-06). |
+| Continuación | Se completan las tarjetas intercaladas antes del siguiente desplazamiento y se repite hasta llegar o hasta que una regla termine o cancele el viaje. |
 
-Después de que el jugador declara el destino, `Viajar` determina primero la
-**ruta rápida**: la ruta válida entre el origen y el destino que requiere la
-menor cantidad de días de tiempo previsto sin demora. Las rutas se comparan por
-la suma de sus días, no mediante la cantidad de Lugares o conexiones como un
-criterio independiente.
+Las conexiones se recorren en orden; las tarjetas de desplazamiento no tienen una
+frecuencia aleatoria. Las demás aparecen cuando se cumple su disparador.
+`VIA-APR-02` se presenta como máximo una vez en cada secuencia entre conexiones,
+incluso si coinciden agotamiento y pérdida de capacidad.
 
-Si una sola ruta posee la duración mínima, esa es la ruta rápida. Si varias
-rutas empatan con la misma duración mínima, `Viajar` elige una al azar entre
-ellas. Encontrar esas rutas dentro del grafo del mundo es una responsabilidad
-del diseño técnico; estas reglas solo establecen el criterio que el resultado
-debe cumplir.
+Toda terminación anterior a la llegada se cierra con el relato de la tarjeta
+que la produjo y omite `VIA-LLE-01`. No concede estados ni Relaciones adicionales
+por el mero hecho de interrumpir el viaje.
 
-Con las duraciones universales de la sección 4.1, dos rutas que usen el mismo
-medio, atraviesen únicamente Lugares de tamaño normal y no tengan otros
-modificadores quedan ordenadas exactamente por su cantidad de tramos: entre
-ellas, menos conexiones significan menos días. Esta equivalencia es una
-abstracción deliberada de las reglas.
+<a id="rutas"></a>
+## 3. Rutas y medios de transporte
 
-Las Características de tamaño de los Lugares pueden alterar esa comparación.
-Por ahora, `Pequeño` reduce el tiempo y `Vasto` lo aumenta según la sección 4.1.
-Así, una ruta con más Lugares pequeños puede resultar más rápida que otra con
-menos Lugares vastos.
+### 3.1. Construcción y elección de ruta
 
-A partir de la ruta rápida, `Viajar` construye una **ruta larga**. Esta segunda
-ruta se aparta por otro camino y alcanza el mismo destino, pero contiene más
-Lugares. Representa el camino que seguiría un Grupo sin conocimientos
-suficientes para planificar bien el viaje.
+La **ruta rápida** es una ruta válida de duración prevista sin demora mínima,
+calculada con las reglas de tiempo. Se comparan días, no cantidad de conexiones;
+si varias empatan, se elige una al azar. El algoritmo para encontrarlas pertenece
+al diseño técnico.
 
-Sea `C` la cantidad de Lugares de la ruta rápida, incluidos el origen y el
-destino, y sea `E` la cantidad de Lugares adicionales de la ruta larga. La ruta
-larga debe cumplir:
+La **ruta larga** alcanza el mismo destino por otro camino y contiene más Lugares.
+Representa el recorrido de un Grupo sin conocimientos suficientes para planificar.
+Si `C` cuenta los Lugares de la rápida, incluidos origen y destino, añade un entero
+`E` que cumple:
 
@@ -98,96 +90,73 @@
 C ÷ 4 < E < C
-cantidad de Lugares de la ruta larga = C + E
+mínimo de E = parte entera(C ÷ 4) + 1
+máximo de E = C − 1
+Lugares de la ruta larga = C + E
 ```
 
-Como `E` es un número entero, su mínimo es `parte entera(C ÷ 4) + 1` y su
-máximo es `C − 1`. Estas reglas no indican cómo encontrar el desvío: cualquier
-técnica puede construirlo siempre que forme una ruta válida entre los mismos
-extremos y respete ese intervalo. Su duración se calcula después mediante las
-mismas reglas que la de cualquier otra ruta.
+Cualquier técnica puede construirla si produce una ruta válida con esos extremos
+y ese intervalo. Su duración se calcula con las mismas reglas que la rápida.
 
-Una vez determinadas ambas rutas, `Viajar` busca dentro del Grupo un Actor
-capaz de proponer la ruta rápida. Un Actor cumple esa condición si posee la
-Característica `Viajero`, posee la Característica `Explorador` o conoce al menos
-la mitad de los Lugares incluidos en la ruta rápida. Basta con cumplir uno de
-estos criterios y la comprobación no requiere un chequeo.
+Puede proponer la ruta rápida cualquier Actor que cumpla al menos una condición:
 
-Para aplicar el criterio territorial, se cuentan todos los Lugares de la ruta,
-incluidos el origen y el destino. El Actor debe mantener una Relación `Conoce`
-con al menos la mitad de ellos; los Lugares conocidos no necesitan ser
-consecutivos. Cuando la mitad no es un número entero, la cantidad mínima se
-redondea hacia arriba.
+- Posee `Viajero`.
+- Posee `Explorador`.
+- Tiene una Relación `Conoce` con al menos `redondear hacia arriba(C ÷ 2)` Lugares
+  de la ruta rápida, contando ambos extremos. No necesitan ser consecutivos.
 
-Si ningún Actor cumple la condición, se presenta
-[`VIA-RUT-01`](#via-rut-01). Si uno o más Actores la cumplen, se elige a un
-proponente y se presenta [`VIA-RUT-02`](#via-rut-02). Cada tarjeta establece qué
-ruta se adopta, qué conoce el jugador y cómo continúa la acción.
+No hay chequeo ni grados de aptitud entre candidatos; si hay varios, se sortea
+el proponente. La tarjeta de ruta determina la elección y la información visible.
 
-Los tres criterios habilitan al proponente, pero no establecen grados de
-aptitud entre quienes los cumplen. Cuando hay varios Actores válidos, se elige
-uno al azar entre ellos.
+### 3.2. Medio de cada conexión
 
-Para comparar tanto la ruta rápida como la ruta larga, `Viajar` presume que el
-Grupo tendrá disponibles todos los medios de transporte necesarios: barcos,
-caballos y camellos. Después del aprovisionamiento, la ruta adoptada se recorre
-con esos mismos medios mientras no se pierdan durante el viaje.
+Para comparar rutas se presumen disponibles todos los medios necesarios.
+Después del aprovisionamiento se usan mientras sigan disponibles. La prioridad,
+tanto para prever duración como para elegir tarjeta, es:
 
-`Viajar` asigna un medio a cada conexión. Si el Lugar de origen o el de destino
-posee la Característica
-`Acuático`, toda la conexión usa barco y se resuelve mediante `VIA-DES-NAV`,
-incluida la mitad que pertenezca a un Lugar no acuático. Esta abstracción
-presupone que la transición entre tierra y agua forma parte de la misma escena.
+| Condición | Medio y tarjeta |
+|---|---|
+| Alguno de los dos Lugares es `Acuático` | Barco, `VIA-DES-NAV`, para toda la conexión, incluida su mitad no acuática. |
+| Conexión terrestre, al menos un caballo por Actor y terreno permitido | Caballo, `VIA-DES-CAB`. |
+| No puede usarse caballo y hay al menos un camello por Actor | Camello, `VIA-DES-CAM`. |
+| Ninguna especie alcanza para todos o puede usarse | A pie, `VIA-DES-AND`. |
 
-En las demás conexiones, el Grupo cabalga a caballo cuando dispone de un
-caballo por Actor y ese medio puede atravesar ambos Lugares. Si no puede hacerlo,
-cabalga en camello cuando dispone de un camello por Actor; si tampoco puede,
-anda a pie. Esta prioridad también resuelve un Grupo que dispone de ambas
-especies. La prohibición de atravesar un `Desierto` `Vasto` a caballo se aplica
-antes de elegir el medio, por lo que una conexión así pasa a camello o a pie.
+El caballo no puede atravesar un Lugar simultáneamente `Desierto` y `Vasto`;
+esa prohibición se comprueba antes de elegir el medio. Los demás terrenos que
+reducen la velocidad montada se detallan en [Duraciones](#duraciones).
 
-La misma asignación se usa para calcular el tiempo previsto y para elegir la
-tarjeta de desplazamiento. `VIA-APR-01` proporciona después todas las monturas
-y medios que exija la ruta adoptada.
+`VIA-APR-01` proporciona una montura por Actor de cada especie requerida por la
+ruta. Si requiere ambas, obtiene ambas cantidades. Una muerte resta inmediatamente
+una montura del contador de su especie y se conserva durante el viaje actual,
+sin afectar viajes futuros. Antes de cada nueva conexión terrestre se reevalúa
+la prioridad con los Actores y contadores actuales; una montura extra sirve como
+reserva mientras siga habiendo una por integrante.
 
-## 4. Tiempo, carga, alimento y agua durante el viaje
-
-El tiempo, la carga, el alimento y el agua pertenecen a la acción general, no a
-una viñeta en particular. `Viajar` mantiene estas cuentas durante todo el
-recorrido.
-
-`Viajar` no simula precios, dinero ni capacidad de pago. Siempre presume que el
-Grupo puede pagar los bienes que necesita para el viaje. Esto no garantiza que
-logre conseguirlos: su disponibilidad y los resultados de los intentos de
-aprovisionamiento siguen sujetos a las reglas y chequeos de sus tarjetas. El
-comercio y los mercados quedan fuera de estas reglas por ahora.
+El barco está disponible automáticamente en el punto exacto donde se necesita:
+no se busca, paga ni traslada mediante otra escena o chequeo. No es un objeto
+persistente. Su naufragio termina el viaje actual, pero no impide disponer de
+otro en viajes futuros. La transición entre tierra y agua forma parte de la
+misma escena de navegación.
 
-### 4.1. Tiempo previsto
-
-Antes de que el Grupo parta, `Viajar` calcula cuánto duraría la ruta sin demoras
-por carga, tiradas ni sucesos inesperados. Cada desplazamiento parte de una
-duración universal establecida por las reglas y la modifica mediante las
-circunstancias que ya se conocen durante la planificación. La suma forma el
-**tiempo previsto sin demora**.
+<a id="tiempo"></a>
+## 4. Tiempo, reservas y consumo
 
-Un **tramo** es el desplazamiento desde el centro del Lugar de origen hasta el
-centro del Lugar de destino a través de una conexión. Su duración base universal
-entre dos Lugares de tamaño normal depende del medio empleado:
+<a id="duraciones"></a>
+### 4.1. Duraciones previstas y reales
 
-| Medio | Duración base entre dos Lugares normales |
-|---|---:|
-| Andar | 7 días |
-| Cabalgar a caballo | 4 días |
-| Cabalgar en camello | 5 días |
-| Navegar en barco | 2 días |
+Un **tramo** recorre una conexión del centro del Lugar de origen al centro del
+Lugar de destino. Cada Lugar aporta su mitad:
 
-Cada uno de los dos Lugares aporta el tiempo necesario para recorrer su propia
-mitad. Para una duración base `B`, `Viajar` calcula:
+| Medio | Base entre dos Lugares normales | Base por mitad normal |
+|---|---:|---:|
+| Andar | 7 días | 3,5 días |
+| Caballo | 4 días | 2 días |
+| Camello | 5 días | 2,5 días |
+| Barco | 2 días | 1 día |
+| Animal de carga, en tierra | 5 días | 2,5 días |
 
 ```text
-duración del tramo = (B ÷ 2 × factor del origen)
-                   + (B ÷ 2 × factor del destino)
+duración de una mitad = base por mitad × factor de tamaño
+duración del tramo = duración de la mitad del origen + duración de la del destino
 ```
 
-El factor de tamaño de cada Lugar es:
-
-| Tamaño del Lugar | Factor |
+| Tamaño | Factor |
 |---|---:|
@@ -196,248 +165,240 @@
 | `Vasto` | 2 |
-
-Los dos medios montados comparten una restricción de velocidad. Si el Lugar es
-`Montañoso`, `Pantanoso` o `Boscoso`, esa mitad usa los 7 días universales de
-`Andar` en lugar de los 4 días de `Cabalgar` o los 5 días de
-`CabalgarEnCamello`. La sustitución se aplica antes del factor de tamaño.
 
-Además, una mitad que sea a la vez `Desierto` y `Vasto` no puede recorrerse a
-caballo. Los camellos sí pueden atravesarla. Esa mitad usa la referencia de 5
-días de `CabalgarEnCamello`, salvo que también sea `Montañoso`, `Pantanoso` o
-`Boscoso`; en ese caso usa la referencia de 7 días indicada en la regla
-anterior. Después aplica el factor `Vasto`.
+Un Lugar con `Pequeño` y `Vasto` a la vez es un error de diseño: no tiene factor
+definido. En una ruta, cada Lugar intermedio aporta una mitad de llegada y otra
+de salida; los extremos solo aportan una.
 
-Varias de las tres Características que reducen la marcha no acumulan sus
-efectos: basta con que una esté presente para sustituir la velocidad. Esta
-sustitución no cambia la acción particular ni convierte la escena en
-[`VIA-DES-AND`](#via-des-andar); solo cambia la duración de la mitad. Por
-ejemplo, cabalgar a caballo desde un Lugar normal `Boscoso` hacia otro Lugar
-normal sin esas Características toma 3,5 días por la primera mitad y 2 días por
-la segunda: 5,5 días en total antes de otras modificaciones.
-
-Por ejemplo, al andar entre dos Lugares normales, cada mitad toma 3,5 días y
-el tramo completo toma 7 días. Al andar desde un Lugar `Vasto` hasta uno
-`Pequeño`, las mitades toman 7 y 1,75 días respectivamente, para una duración
-de 8,75 días.
+Caballos y camellos usan la base de `Andar` en cada mitad `Montañoso`,
+`Pantanoso` o `Boscoso`, antes de aplicar tamaño. Basta una de esas Características:
+no acumulan efectos. El camello puede atravesar `Desierto` `Vasto` con su base
+propia, salvo que también tenga alguno de esos terrenos lentos. Sustituir la
+referencia temporal no cambia la tarjeta, su acción particular ni su tabla.
 
-En una ruta de varios tramos, cada Lugar intermedio aporta una mitad al llegar
-y otra al salir. El origen de la ruta solo aporta su mitad de salida y el
-destino final solo aporta su mitad de llegada.
+El Grupo acompaña al elemento limitante más lento. Un animal de carga limita el
+paso terrestre solo si su referencia es más lenta que la del Grupo; varios
+animales no vuelven a reducirlo y no limitan una conexión navegada. Ninguna otra
+Característica altera las bases salvo regla expresa de este documento.
 
-`Viajar` usa la duración ajustada por tamaño tanto al calcular el tiempo
-previsto como al resolver el desplazamiento real mediante su tarjeta de viñeta.
-Otras Características territoriales más complejas no alteran todavía el tiempo
-de desplazamiento.
+El **tiempo previsto sin demora** suma las duraciones con medios, tamaños y
+modificadores conocidos durante la planificación; excluye carga, tiradas e
+imprevistos futuros. Sirve para comparar rutas, presentar el plan y fijar la
+necesidad base de provisiones. La [preparación de carga](#configuracion-inicial)
+resuelve el efecto de los animales sobre esa referencia antes de partir.
 
-`Viajar` usa esta misma referencia para comparar rutas, explicar un plan y
-calcular la necesidad base de provisiones según la cantidad de integrantes. La
-necesidad base se fija después de calcular el tiempo previsto y no se recalcula
-retroactivamente cuando la carga o los acontecimientos alteran el viaje real.
-De esta manera, la planificación siempre parte de una referencia estable.
+La **duración real** usa la misma base y aplica velocidad limitante, estados,
+demora por carga asignada e incidentes de la tarjeta. Primero se aplica tamaño;
+después se suman o restan las modificaciones expresadas en horas. Se conservan
+todas las fracciones, sin redondear calendario ni acumulaciones.
 
-Un Lugar nunca puede poseer a la vez `Pequeño` y `Vasto`. Su coexistencia es un
-error de diseño del mundo; `Viajar` no define un factor para ese estado inválido.
+Solo pasa tiempo si una regla establece expresamente una duración; su omisión
+equivale a 0 horas. Todo tiempo avanza el calendario y, desde que existen
+reservas, el contador de consumo, incluso si después se cancela el viaje.
+El plan no corrige ni limita la duración real y las demoras de ejecución no
+reinician el aprovisionamiento inicial.
 
-### 4.2. Velocidad y carga del Grupo
+<a id="reservas"></a>
+### 4.2. Reservas y materialización
 
-Las duraciones de la sección 4.1 expresan el paso base universal del Grupo a
-pie, a caballo, en camello o en barco. No se anota una velocidad numérica
-individual en cada personaje. Las Características del Actor, la montura o el
-vehículo modifican la base cuando corresponde. Todo animal obtenido
-específicamente como animal de carga posee una duración base universal de 5 días
-por conexión terrestre entre dos Lugares normales. Esa base se divide entre las
-dos mitades y recibe los factores de tamaño de sus Lugares.
+Las unidades alimenticias miden internamente aporte y consumo; nunca se muestran
+al jugador. Se conservan las cantidades concretas para calcular peso y relato.
 
-En cada desplazamiento, el Grupo solo puede avanzar a la velocidad del elemento
-más lento que limite ese recorrido. Un animal de carga puede aliviar el peso de
-las provisiones, pero también fija el paso máximo del Grupo si es más lento que
-los demás. Por eso puede beneficiar un viaje cargado y perjudicar un recorrido
-corto que el Grupo podría realizar con mayor rapidez sin él.
+| Comida, por pieza completa | Aporte oculto | Peso |
+|---|---:|---:|
+| Carne seca | 1 unidad alimenticia | 10 g |
+| Pan | 2 unidades alimenticias | 12 g |
+| Lonja de queso | 0,5 unidades alimenticias | 15 g |
+| Pescado salado | 4 unidades alimenticias | 7 g |
 
-Las provisiones adicionales no forman una categoría especial: aumentan las
-reservas y también la carga. Cada Actor permite que el Grupo transporte 10 kg
-sin sufrir una demora por peso. Para un Grupo de `A` Actores, `Viajar` calcula
-primero el exceso que soportan sus integrantes:
+Para cantidades `C`, `P`, `Q` y `S`, respectivamente:
 
 ```text
-límite sin demora = 10 kg × A
-x = máximo(0, carga soportada por los Actores − límite sin demora)
+reserva alimenticia = C + 2P + 0,5Q + 4S
+peso de comida = (10C + 12P + 15Q + 7S) ÷ 1000 kg
 ```
 
-La carga que los animales transportan se descuenta antes de calcular `x`.
-Después se llama `T` a la duración prevista sin demora del recorrido completo,
-expresada en horas. `T` incluye los medios de transporte, los tamaños de los
-Lugares y los demás modificadores conocidos durante la planificación, pero no
-incluye la carga, tiradas futuras ni acontecimientos inesperados. La demora por
-carga es:
+Para **materializar alimento** se inicia un lote vacío y se sortea una pieza de
+cualquiera de los cuatro tipos en cada repetición, hasta alcanzar o superar el
+objetivo. Se conserva el excedente de la última pieza; no se fracciona comida.
+Es una operación lógica, sin nuevos intentos de los Actores ni tiempo adicional.
+
+El agua se lleva en botas de 3 litros. El cuero pesa 0,1 kg y el agua 1 kg/litro:
+una bota llena pesa 3,1 kg. Consumir agua reduce su peso, pero conserva el cuero;
+una bota puede quedar parcial o vacía. Solo una escena que permita conseguir agua
+puede permitir rellenarla.
+
+Toda concesión de agua se **materializa** añadiendo botas completas y llenas:
 
 ```text
-demora por carga = T × (x ÷ 10 kg)²
-                  = T × x² ÷ 100 kg²
+botas añadidas = redondear hacia arriba(litros objetivo ÷ 3 litros)
+agua incorporada = 3 litros × botas añadidas
 ```
 
-La demora conserva así un crecimiento cuadrático, pero también depende de la
-distancia. Un exceso de 1 kg añade el 1 % de `T`; 2 kg añaden el 4 %; 5 kg, el
-25 %; y 10 kg, el 100 %. Por ejemplo, si `T` es 168 horas, un exceso de 2 kg
-añade 6,72 horas. Las horas resultantes forman una demora total del viaje; cada
-24 horas forman un día.
+Los redondeos se aplican incluso si una regla impone un máximo de provisiones:
+la última pieza o bota se conserva entera aunque lo supere. Los excedentes
+cuentan como reserva y carga. `VIA-APR-02` fija su propia obtención de carne seca;
+el agua que obtiene usa la conversión anterior.
 
-La demora se reparte entre las conexiones y sus mitades en proporción a sus
-duraciones previstas sin carga. Si una parte del recorrido tiene una duración
-prevista `t`, recibe:
+<a id="consumo"></a>
+### 4.3. Consumo y agotamiento
+
+Cada Actor consume **2 unidades alimenticias y 0,5 litros de agua por cada
+período acumulado de 24 horas**. No hay fracciones de ración por períodos menores.
+Las tasas no reciben modificadores. Monturas y animales de carga no consumen
+reservas del Grupo; su alimentación queda abstraída.
+
+Al crear las reservas en `VIA-APR-01` comienza en cero el contador de horas desde
+la última ración. Esa tarjeta añade sus propias 24 horas y provoca el primer
+consumo. Toda escena posterior conserva y acumula las horas restantes:
 
 ```text
-demora de esa parte = t × (x ÷ 10 kg)²
+períodos completados = parte entera(horas acumuladas ÷ 24 horas)
+alimento requerido = 2 × A × períodos completados unidades alimenticias
+agua requerida = 0,5 × A × períodos completados litros
+horas restantes = horas acumuladas − 24 horas × períodos completados
 ```
 
-La suma de esas demoras parciales coincide con la demora total calculada con
-`T`. El exceso `x` fijado antes de partir se conserva para este reparto aunque
-el consumo reduzca después el peso. Cuando una pérdida de capacidad obliga a
-recalcular el resto del viaje, el nuevo exceso sustituye al anterior solamente
-en las conexiones y mitades que todavía no se recorrieron.
+`A` es la cantidad de Actores aplicable a la escena. Las fronteras de 24 horas
+se resuelven en orden. Para pagar cada ración alimenticia se sortea una pieza
+completa entre los tipos que aún quedan y se repite hasta cubrirla o agotar la
+reserva. Si la última pieza aporta de más, ese aporte sobrante se pierde y no
+paga una ración posterior. La elección representa gustos momentáneos, no una
+prioridad nutricional.
 
-Para calcular la carga se cuentan el peso de las comidas y el peso actual del
-agua y de sus botas según la sección 4.3. Los demás elementos básicos del
-aprovisionamiento permanecen resumidos y no añaden peso en estas reglas.
+Alimento y agua se descuentan por separado. Si un recurso no alcanza, se consume
+todo lo disponible y queda en cero; el otro se descuenta normalmente. Ninguna
+reserva puede ser negativa. Las horas sobrantes pasan a la siguiente escena;
+alcanzar exactamente 24 horas también provoca una ración.
 
-Cada animal de carga puede transportar hasta 100 kg de provisiones y reduce en
-esa medida la carga que soportan los integrantes. Pueden incorporarse varios
-animales para volver a colocar esa carga dentro del límite. Todos los animales
-de carga poseen la misma velocidad, por lo que añadir más no reduce nuevamente
-el paso máximo del Grupo: si uno de ellos ya era el elemento más lento, los
-demás avanzan a ese mismo ritmo. En una conexión terrestre, su referencia de 5
-días sustituye la del medio del Grupo solamente cuando resulte más lenta. No
-limita la velocidad de una conexión navegada.
+La **carencia** de cada recurso comienza en la primera frontera cuya ración
+completa no puede pagarse y dura hasta el final de la escena. Se registran por
+separado el tiempo sin alimento y sin agua. Pagar la última ración completa y
+quedar exactamente en cero no implica carencia: solo empezará si llega otra
+frontera impagable.
 
-#### Elección ante una sobrecarga moderada
+El hambre o la sed no interrumpen un desplazamiento: el Grupo completa toda la
+conexión con mayor esfuerzo narrado, sin tiempo, Características ni penalizaciones
+adicionales por la carencia. Si quedan conexiones y alguna reserva terminó en
+cero, se presenta `VIA-APR-02` antes de continuar; si se alcanzó el destino final,
+se presenta la llegada.
 
-Una vez fijadas las provisiones, `Viajar` determina `N`: la menor cantidad de
-animales de carga que dejaría el peso soportado por los Actores dentro de su
-límite conjunto y eliminaría por completo la demora cuadrática. Si `L` es la
-carga total de provisiones y `A` la cantidad de Actores, se calcula:
+Cuando una consecuencia aparta a un Actor durante una viñeta, no se recalculan
+sus chequeos, duración ni consumo. El Actor deja de contarse al cerrarla;
+las escenas posteriores, la capacidad de carga y las monturas usan entonces
+la nueva composición.
 
-```text
-N = redondear hacia arriba(
-      máximo(0, L − 10 kg × A) ÷ 100 kg
-    )
-```
+<a id="carga"></a>
+## 5. Carga y preparación de configuraciones
 
-Si `N` es mayor que cero, `Viajar` evalúa también una sola alternativa con
-`N − 1` animales. No considera configuraciones con dos o más animales menos.
-En esa alternativa, cada animal recibe hasta 100 kg y los Actores soportan el
-resto. Si se compara viajar sin animales con viajar con uno, cada configuración
-usa además la velocidad que le corresponde; por lo tanto, no llevar el animal
-puede evitar que su paso limite al Grupo.
+### 5.1. Capacidad y demora
 
-`Viajar` calcula entonces la demora cuadrática de la alternativa. Para saber
-cuántas raciones nuevas exige, no cuenta la demora desde cero: la suma al tiempo
-previsto y comprueba cuántas nuevas fronteras de 24 horas atraviesa el viaje.
-Primero establece las jornadas ya cubiertas por la necesidad base:
+Solo pesan las comidas, el agua actual y el cuero de sus botas. Los demás
+suministros básicos quedan resumidos y no añaden peso.
+
+Cada Actor aporta 10 kg de capacidad sin demora y cada animal de carga transporta
+hasta 100 kg de provisiones. Para `A` Actores, carga total `L` y `M` animales:
 
 ```text
-jornadas base = parte entera(T ÷ 24 horas)
+capacidad animal = 100 kg × M
+carga sobre los Actores = máximo(0, L − capacidad animal)
+límite de los Actores = 10 kg × A
+exceso x = máximo(0, carga sobre los Actores − límite de los Actores)
 ```
 
-Para una demora `D`, el suplemento debe cubrir:
+Si `T` es la duración prevista sin carga del recorrido, expresada en horas:
 
 ```text
-jornadas con demora = parte entera((T + D) ÷ 24 horas)
-jornadas suplementarias = jornadas con demora − jornadas base
+demora por carga D = T × (x ÷ 10 kg)²
+demora asignada a una parte de duración prevista t = t × (x ÷ 10 kg)²
 ```
 
-Así, si `T` es 30 horas, la última frontera ya cubierta es la hora 24 y faltan
-18 horas para alcanzar la siguiente, situada en la hora 48. Una demora de 17
-horas no exige otra ración; una demora de 18 horas sí la exige.
+Así se reparte la demora proporcionalmente entre conexiones y mitades, sin
+duplicarla. `T` incluye la velocidad de animales que limiten el paso, tamaños y
+demás factores conocidos; excluye la propia carga, tiradas e imprevistos.
+Un exceso de 1 kg añade 1 % de `T`; 5 kg añaden 25 %; 10 kg, 100 %.
+
+El exceso fijado al partir no disminuye por consumir reservas. Solo las
+[comprobaciones durante el viaje](#recalculo-carga) sustituyen la demora pendiente;
+nunca modifican tiempo ya transcurrido. No se pueden abandonar provisiones
+libremente: esa decisión existe únicamente en `VIA-APR-06`.
+
+<a id="configuracion-inicial"></a>
+### 5.2. Configuración inicial y alternativa con un animal menos
 
-Cada paquete diario por Actor contiene 2 piezas de carne seca, que aportan las
-2 unidades alimenticias de un día, y 0,5 litros de agua. Para `q` jornadas
-suplementarias y `A` Actores, el suplemento total es:
+Después de materializar las provisiones se calcula la cantidad mínima de
+animales que elimina toda demora por carga:
 
 ```text
-piezas de carne seca = 2 × A × q
-litros de agua necesarios = 0,5 × A × q
-botas llenas = redondear hacia arriba(A × q ÷ 6)
+N = redondear hacia arriba(máximo(0, L − 10 kg × A) ÷ 100 kg)
 ```
 
-El agua se incorpora en botas completas de 3 litros. El excedente producido
-por ese redondeo forma parte de las reservas y de la carga. La carne seca y el
-agua del suplemento están disponibles sin otro chequeo aunque no hubieran
-formado parte del aprovisionamiento ordinario. El suplemento no corrige un
-faltante respecto de la necesidad base ni se reduce por un excedente secreto
-de las provisiones ya obtenidas: solo cubre las nuevas jornadas de consumo
-producidas al sumar la demora de carga al tiempo previsto.
+Si `N > 0`, se evalúa además una sola alternativa con `N − 1`; nunca se
+consideran dos o más animales menos. Cada animal recibe hasta 100 kg y los
+Actores cargan el resto. Cada configuración usa su propia velocidad: pasar de
+un animal a ninguno puede eliminar su limitación del paso.
 
-El propio suplemento también pesa y puede aumentar la demora. Para resolver
-esa retroalimentación, `q` representa la cantidad de jornadas nuevas que ya
-cubre el suplemento tentativo. `Viajar` aplica este ciclo:
+La preparación debe cerrar también la relación entre velocidad y provisiones.
+Comienza calculando la necesidad base sin animales de carga, materializa las
+reservas y evalúa las configuraciones permitidas. Si los 5 días de un animal
+limitan una configuración, recalcula con esa duración las provisiones, su peso
+y la cantidad de animales. Repite hasta que duración, reservas y animales sean
+coherentes entre sí. Conserva el único chequeo de `CalcularProvisiones`; estas
+operaciones no son nuevos intentos ni añaden tiempo. La velocidad de la
+configuración aceptada queda fijada al partir y se aplica aunque resulte más lenta.
 
-1. Comienza con `q = 0`, sin suplemento, y calcula la demora de esa carga.
-2. Calcula las jornadas suplementarias que exige la demora actual:
+Para evaluar `N − 1`, la demora `D` exige un suplemento solo por las nuevas
+fronteras de consumo que añade a su tiempo previsto `T`:
 
-   ```text
-   jornadas exigidas = parte entera((T + demora actual) ÷ 24 horas)
-                     − parte entera(T ÷ 24 horas)
-   ```
+```text
+jornadas suplementarias q =
+  parte entera((T + D) ÷ 24 horas) − parte entera(T ÷ 24 horas)
 
-3. Si las jornadas exigidas son iguales a `q`, el cálculo queda cerrado.
-4. Si son mayores, reemplaza el suplemento tentativo por el suplemento total
-   correspondiente a esa nueva cantidad y vuelve a calcular la demora. El
-   redondeo de las botas se aplica sobre toda el agua suplementaria acumulada:
-   una bota ya incluida puede cubrir varias jornadas y no se añade otra solo
-   por comenzar una nueva repetición.
-5. Resta la demora anterior de la nueva. Esta diferencia cuenta únicamente las
-   horas creadas por el peso incorporado en esa repetición.
-6. Si la diferencia es mayor que 24 horas, descarta el suplemento tentativo y
-   la alternativa con `N − 1` animales: los `N` animales son obligatorios.
-7. Si la diferencia es menor o igual a 24 horas, incluida exactamente esa
-   cantidad, asigna a `q` la nueva cantidad de jornadas exigidas y vuelve al
-   paso 2.
+suplemento para A Actores y q jornadas:
+  carne seca = 2 × A × q piezas
+  agua objetivo = 0,5 × A × q litros
+  botas llenas = redondear hacia arriba(A × q ÷ 6)
+```
 
-Las repeticiones forman una sola operación de las reglas y no representan
-nuevos intentos del Actor. Si el ciclo se cierra sin superar el límite, se
-presenta [`VIA-APR-05`](#via-apr-05) con la demora final y el suplemento final.
-Estas incorporaciones son tentativas durante el cálculo: las provisiones solo
-se añaden realmente a las reservas si el jugador acepta esa alternativa.
+La carne seca y el agua suplementarias están disponibles sin otro chequeo.
+Su excedente por botas enteras cuenta como reserva y peso. El suplemento solo
+cubre las jornadas creadas por la demora: no corrige un faltante de la necesidad
+base ni se reduce por un excedente secreto del aprovisionamiento.
 
-El cálculo inicial de carga ocurre antes de partir y no se repite solamente
-porque el Grupo consuma reservas y su carga disminuya. Sí se vuelve a comprobar
-la capacidad después de una viñeta que quite del Grupo a un Actor o a un animal
-que estaba aportando capacidad de carga. Esta comprobación se realiza después
-de completar la viñeta y aplicar su consumo, pero antes del siguiente
-desplazamiento; nunca modifica retroactivamente la conexión ya resuelta.
+Como el suplemento pesa, se resuelve este ciclo:
 
-La comprobación vuelve a calcular el resto del viaje con el peso y la
-composición que existen en ese momento. Primero establece:
-
-```text
-A = cantidad actual de Actores del Grupo
-L = peso actual de todas las provisiones
-C = 100 kg × cantidad de animales de carga que permanecen
-carga soportada por los Actores = máximo(0, L − C)
-x = máximo(0, carga soportada por los Actores − 10 kg × A)
-```
+1. Comenzar con `q = 0`, sin suplemento, y calcular la demora de esa carga.
+2. Calcular las jornadas exigidas con la fórmula anterior. Si son `q`, cerrar.
+3. Si son mayores, sustituir el suplemento tentativo por el **total** para esa
+   cantidad de jornadas, redondeando las botas sobre toda el agua acumulada.
+4. Recalcular la demora y restarle la anterior: la diferencia mide solo las
+   horas creadas por el peso de esta repetición.
+5. Si la diferencia supera 24 horas, descartar suplemento y alternativa:
+   los `N` animales son obligatorios. Exactamente 24 horas todavía es admisible.
+6. En otro caso, actualizar `q` a las jornadas exigidas y volver al paso 2.
 
-Si `x` es cero, la pérdida no produjo una sobrecarga y esta previsión especial
-termina. No revela si el aprovisionamiento inicial alcanzará; solo rige el
-disparador ordinario de `VIA-APR-02` cuando una reserva queda en cero. Si `x` es
-mayor que cero, la comprobación continúa.
+Las incorporaciones son tentativas hasta que el jugador acepte. Si el ciclo
+cierra, se ofrece `VIA-APR-05` con suplemento y demora finales; si no, se
+obtienen los `N` animales sin decisión. Obtener cualquier cantidad fijada de
+animales de carga es automático, sin responsable, chequeo, fracaso ni tiempo
+extra.
 
-Después llama `T restante` a la suma, en horas, de las duraciones sin demora por
-carga de todas las conexiones que faltan desde el Lugar actual hasta el destino.
-Usa la ruta adoptada, la composición, los medios y los estados actuales, junto
-con todos los modificadores ya conocidos. No anticipa tiradas ni incidentes
-futuros. Con el exceso actual calcula:
+<a id="recalculo-carga"></a>
+### 5.3. Comprobación durante el viaje
 
-```text
-demora restante por carga = T restante × (x ÷ 10 kg)²
-duración restante con carga = T restante + demora restante por carga
-```
+Se comprueba la capacidad después de una viñeta que quite un Actor o animal de
+carga y después de que `VIA-APR-02` añada provisiones, aunque no haya habido
+una pérdida. Se hace tras cerrar la viñeta y aplicar su consumo, antes del
+siguiente desplazamiento. Perder una montura afecta al medio, no a la capacidad
+de carga.
 
-Esta es una nueva previsión desde el Lugar actual: reemplaza la parte todavía
-no transcurrida de la demora por carga calculada antes de partir. No se suma a
-esa demora anterior ni cambia el tiempo que ya pasó.
+Con la composición y el peso actuales, se recalcula `x` según 5.1. Si es cero,
+termina la comprobación: no se revela si el aprovisionamiento inicial bastará.
+Sigue vigente el disparador ordinario de búsqueda cuando una reserva llegue a cero.
 
-Para saber si las reservas cubren esa duración, `Viajar` toma también las horas
-`h` acumuladas desde la última ración y calcula:
+Si `x > 0`, se obtiene `T restante` sumando las duraciones sin carga de las
+conexiones pendientes con ruta, medios, estados y modificadores actuales,
+sin anticipar tiradas ni incidentes. Después:
 
 ```text
+D restante = T restante × (x ÷ 10 kg)²
+duración restante con carga = T restante + D restante
 períodos restantes = parte entera(
@@ -449,752 +410,541 @@
 
-Una reserva que iguala su cantidad necesaria alcanza; una cantidad menor no.
-El alimento y el agua se comparan por separado.
+`h` son las horas desde la última ración. Cada reserva se compara por separado;
+igualar lo necesario alcanza.
 
-Si esta previsión muestra que el alimento, el agua o ambos no alcanzarán,
-`VIA-APR-02` se resuelve primero. Si ya se había resuelto esa tarjeta después de
-la misma conexión porque una reserva quedó en cero, no se presenta una segunda
-vez: se usan las provisiones que dejó. Si el viaje continúa, `Viajar` repite los
-cálculos de peso, duración y necesidades con ese estado resultante y pasa a
-juzgar la carga. La carga se considera intransportable y se presenta
-[`VIA-APR-06`](#via-apr-06) solamente si todavía existe un exceso `x` y la nueva
-duración restante con carga exige más alimento o agua que las reservas
-disponibles. Si ambas reservas alcanzan, la demora recién calculada sustituye la
-demora pendiente anterior y el viaje continúa sin `VIA-APR-06`.
+Si falta alimento o agua, se resuelve primero `VIA-APR-02`, salvo que ya se
+haya resuelto tras la misma conexión. Si el viaje continúa, se repiten los
+cálculos con las reservas resultantes. Solo se considera la carga
+**intransportable** y se presenta `VIA-APR-06` si persisten ambas condiciones:
+`x > 0` y alguna reserva no cubre la nueva duración restante con carga.
 
-Esta previsión especial se realiza después de una pérdida de capacidad y se
-repite después de que `VIA-APR-02` añada provisiones. No se usa en otros
-momentos del curso ordinario para revelar por adelantado si la estimación
-oculta del aprovisionamiento inicial fue suficiente.
+Si las reservas alcanzan, se continúa con la nueva demora. Esta sustituye la
+demora de las partes pendientes, sin sumarse a la anterior ni alterar el pasado.
+La previsión especial no se usa fuera de estos disparadores para revelar si
+la estimación inicial oculta fue suficiente.
 
-Los animales de carga no consumen las reservas del Grupo. Las reglas presuponen
-que los herbívoros encuentran alimento durante el recorrido y que los camellos
-dependen de sus propias reservas de agua. Su alimentación no se simula ni
-aumenta la necesidad base de provisiones.
+<a id="chequeos"></a>
+## 6. Chequeos y selección de responsables
 
-Reducir la cantidad de provisiones también puede devolver la carga al intervalo
-sin demora. El jugador solo puede abandonarlas cuando
-[`VIA-APR-06`](#via-apr-06) presenta expresamente esa decisión.
+Una acción particular recibe Actor y dificultad, tira 1d20 y suma sus
+modificadores. La cara es el **resultado natural**; el total es el
+**resultado ajustado**, limitado a 20 si lo supera. Igualar la dificultad es
+éxito salvo excepción expresa. Devuelve la cara, cada modificador, el total
+ajustado y si alcanzó la dificultad.
 
-El tiempo previsto `T` ya incorpora la duración de 5 días de un animal de carga
-cuando este limita al Grupo. Por lo tanto, la necesidad base de provisiones
-incluye ese tiempo desde el principio; no necesita un suplemento separado.
+La lista de modificadores de cada acción es exhaustiva: ninguna Característica,
+Relación o circunstancia ajena aporta bonos por analogía. Cada Característica
+aplicable suma su bono una sola vez y varias se acumulan salvo excepción.
+Una lista todavía incompleta no permite determinar definitivamente la aptitud.
 
-La preparación resuelve esta relación como una sola operación. Primero calcula
-la necesidad base sin animales de carga, materializa las provisiones y compara
-las configuraciones permitidas por **Elección ante una sobrecarga moderada**.
-Cada configuración se evalúa con su propia velocidad: si contiene al menos un
-animal de carga y sus 5 días limitan al Grupo, vuelve a calcular con esa
-duración las provisiones, su peso y la cantidad de animales necesaria. Repite
-la evaluación hasta que la duración, las provisiones y los animales de la
-configuración coincidan entre sí. Estas repeticiones no son nuevos chequeos ni
-hacen pasar tiempo. La configuración aceptada por el jugador queda fijada al
-partir y su velocidad se aplica durante el viaje aunque resulte más lenta.
+Por defecto, la tarjeta elige al **Actor más apto** entre todos los integrantes
+actuales, salvo exclusión expresa. Compara únicamente la suma de modificadores
+que esa acción aplicaría y sortea entre quienes empaten en el máximo, incluso
+si todos tienen cero. Se elige por acción particular: un mismo responsable
+conserva los dos intentos de una conexión; acciones diferentes pueden tener
+responsables diferentes.
 
-Ninguna Característica modifica las duraciones base salvo cuando una regla de
-este documento lo establece expresamente.
+| Acción | Capacidad evaluada | Modificadores propios | Uso en `Viajar` |
+|---|---|---|---|
+| `Andar` | Orientarse, leer terreno, elegir paso y abrir camino a pie. | `Vagabundo` +2. | `VIA-DES-AND`, dificultad 5. |
+| `Cabalgar` | Guiar y dominar un caballo. | `Jinete` +2. | `VIA-DES-CAB`, dificultad 6. |
+| `CabalgarEnCamello` | Guiar y dominar un camello. | `Jinete` +2. | `VIA-DES-CAM`, dificultad 6. |
+| `NavegarEnBarco` | Interpretar viento y clima, gobernar velas y conservar rumbo útil. | `Navegante` +4. | `VIA-DES-NAV`, dificultad 14. |
+| `CalcularProvisiones` | Calcular y reunir alimento, agua y suministros. | `Administrador` +2 y `Expedicionario` +2; ambas, +4. | `VIA-APR-01`, dificultad 10. |
+| `Forajear` | Buscar alimento y avituallamiento en entorno salvaje. | `Expedicionario` +2 y `Herbalista` +2; ambas, +4. | `VIA-APR-02`, dificultad 14; actúan todos. |
 
-#### Monturas disponibles durante el viaje
+Las dificultades pertenecen a estas tarjetas, no universalmente a las acciones.
+No hay otros modificadores de dado confirmados. `Viajero`, `Explorador` y
+`Conoce` habilitan la planificación; estados, tamaño, terreno y viento modifican
+las reglas o duraciones de la viñeta, sin modificar estas tiradas.
 
-Al comenzar `Viajar`, la acción general registra dos contadores temporales: uno
-para los caballos de monta disponibles y otro para los camellos de monta
-disponibles. Estos contadores pertenecen al viaje completo, no a una viñeta
-particular. Una montura que muere reduce en uno el contador de su especie y esa
-reducción se conserva durante el resto de la acción general.
+La acción particular no elige responsable ni dificultad, no conoce Lugares ni
+ruta, no avanza tiempo, consume reservas, concede Relaciones o escoge incidentes.
 
-Antes de cada nuevo desplazamiento, `Viajar` compara la cantidad de Actores que
-integran actualmente el Grupo con el contador del medio correspondiente:
+Dos sorteos de 1d20 quedan fuera de las acciones particulares:
 
-```text
-puede Cabalgar si caballos disponibles >= Actores del Grupo
-puede CabalgarEnCamello si camellos disponibles >= Actores del Grupo
-```
+- **Incidente:** selecciona una consecuencia después de determinar que ocurrió.
+  Lee la cara natural, sin dificultad, responsable ni modificadores.
+- **Recuperación de montura:** azar simple tras su crisis; 8 o más se recupera,
+  menos de 8 muere. Tampoco tiene responsable ni modificadores.
 
-Antes de cada conexión terrestre, `Viajar` vuelve a aplicar la prioridad de la
-sección 3 con los contadores actuales: usa caballos si alcanzan para todos y el
-terreno lo permite; de lo contrario usa camellos si alcanzan para todos; si
-tampoco alcanzan, usa `Andar`. Una montura adicional funciona como reserva:
-perder una montura no obliga a cambiar de medio mientras el contador de esa
-especie siga siendo igual o mayor que la cantidad de Actores.
+<a id="presentacion"></a>
+## 7. Información y narración
 
-Los contadores reciben sus cantidades iniciales de las monturas obtenidas en
-`VIA-APR-01`. Son una cuenta interna de `Viajar` y se descartan cuando termina
-la acción general. La muerte de una montura solo reduce el contador durante el
-viaje actual: no modifica ningún estado persistente ni afecta viajes futuros.
-`VIA-APR-01` proporciona una montura de la especie correspondiente por cada
-Actor para todos los medios que exija la ruta adoptada. Si unas conexiones
-requieren caballos y otras requieren camellos, proporciona una cantidad
-suficiente de ambas especies. Un animal de carga nunca cuenta como montura:
-sus cantidades y funciones se registran por separado y no son intercambiables.
+Cada viñeta explica qué ocurrió y por qué mediante la ficción y los datos útiles.
+Identifica circunstancias relevantes, responsable, intento, causas de facilidad
+o dificultad, tiempo, consumo y consecuencias. Evita valoraciones vagas cuando
+puede explicar la causa concreta.
 
-#### Disponibilidad del barco durante el viaje
+| Aspecto | Información que debe comunicar |
+|---|---|
+| Plan frente a ejecución | Distinguir tiempo previsto sin demora de tiempo real y explicar cómo velocidad, tiradas, carga e imprevistos alteraron la referencia. |
+| Tamaño | Identificar Lugar, `Pequeño` o `Vasto`, duración de su mitad y duración final del tramo. |
+| Carga | Peso soportado por Actores, límite conjunto, exceso en kg y horas añadidas; hacer visible el efecto de un animal limitante. |
+| Provisiones | Cantidades concretas de carne seca, panes, queso, pescado, botas y litros. Nunca unidades alimenticias. |
+| Chequeos | Cada tarjeta decide si revela dificultad, cara, bonos y total; siempre relaciona causas y resultado. Navegación exige el detalle indicado en su tarjeta. |
+| Incertidumbre inicial | Sin proponente no se revela la ruta ni su previsión. `VIA-APR-01` muestra responsable, capacidades y cantidades obtenidas, pero no la necesidad base ni si alcanzarán. |
 
-`Viajar` presume que siempre hay un barco disponible en el punto exacto donde
-una conexión acuática lo exige. Obtenerlo, pagarlo o trasladarlo no requiere un
-chequeo ni otra viñeta. Esta disponibilidad se establece durante
-`VIA-APR-01`, pero no representa un objeto persistente del Grupo.
+Los desplazamientos se narran en segunda persona, desde el jugador, como un
+relato integrado y cronológico de las mitades del origen y destino, descubrimientos
+e incidentes. Estos se calculan al comparar ambos chequeos, pero se narran en
+el Lugar donde suceden. No se separa una descripción genérica de un informe numérico.
 
-Si el barco naufraga, el viaje actual termina inmediatamente y no se presenta
-`VIA-LLE-01`. El naufragio no impide que un viaje futuro vuelva a disponer de
-otro barco mediante la misma abstracción.
+El cierre reúne todo el consumo de la viñeta: enumera cada comida realmente
+consumida y el agua bebida, aclara que se consumieron durante el recorrido y
+muestra reservas finales. No interrumpe la narración en cada frontera de 24 horas.
+Si no se completó ninguna, informa horas acumuladas y ausencia de nueva ración.
+Si hubo carencia, distingue hambre y sed con sus duraciones; si una reserva
+solo llegó a cero al pagar la última ración, dice que se agotó al llegar sin
+describir carencia.
 
-### 4.3. Reservas temporales del viaje
+Los estados limitantes y sus demoras deben ser visibles, incluida la suma de
+ambos si un Actor ya posee `Esguince` y `Cansado`. Si un incidente no concede
+otro estado porque el Actor ya posee uno, explica que no empeoró.
 
-Al aprovisionarse, `Viajar` crea reservas temporales de alimento y agua. Estas
-reservas no forman parte del inventario general: solo existen mientras la acción
-general mantiene el viaje. Cuando `Viajar` termina, cualquier remanente deja de
-contabilizarse y no se transfiere a otro sistema del juego.
+La prosa puede variar, combinar u omitir ejemplos si conserva esa información.
+Una fogata u otro adorno no es un acontecimiento obligatorio, no determina cuándo
+se descuentan las raciones ni añade tiempo o estados.
 
-Las **unidades alimenticias** son una medida interna y nunca se nombran ante el
-jugador. `Viajar` conserva la cantidad concreta de cada comida para poder
-calcular su aporte y su peso, pero las viñetas hablan de piezas, panes, lonjas y
-pescados consumidos o restantes.
+<a id="tarjetas"></a>
+## 8. Tarjetas de viñeta
 
-| Comida visible | Aporte oculto | Peso |
-|---|---:|---:|
-| Una pieza de carne seca | 1 unidad alimenticia | 10 g |
-| Un pan | 2 unidades alimenticias | 12 g |
-| Una lonja de queso | 0,5 unidades alimenticias | 15 g |
-| Una pieza de pescado salado | 4 unidades alimenticias | 7 g |
+Las tarjetas aplican las reglas compartidas anteriores y especifican aquí
+disparadores, resultados, decisiones y requisitos propios.
 
-Si las cantidades de esas comidas son respectivamente `C`, `P`, `Q` y `S`, sus
-totales ocultos son:
+### 8.1. Planificación
 
-```text
-reserva alimenticia = C + 2P + 0,5Q + 4S
-peso de la comida = (10C + 12P + 15Q + 7S) ÷ 1000 kg
-```
+<a id="via-rut-01"></a>
+#### VIA-RUT-01 — Partir sin un plan
 
-Los cuatro tipos de comida pueden aparecer en cualquier aprovisionamiento. Para
-convertir una cantidad objetivo de unidades alimenticias en objetos concretos,
-`Viajar` comienza el lote en cero. En cada repetición escoge al azar uno de los
-cuatro tipos, añade una unidad completa de esa comida y vuelve a calcular el
-aporte acumulado. Repite el procedimiento hasta que el lote alcance o supere el
-objetivo. El eventual excedente permanece en las reservas. Esta repetición es
-solo una operación lógica: no representa intentos sucesivos de los Actores ni
-hace pasar tiempo.
+**Resolución; 24 horas, antes de crear reservas.** Ocurre si no hay proponente
+válido. Adopta la ruta larga y la mantiene oculta. Explica que el Grupo conoce
+la dirección general, pero carece de un recorrido fiable y se orientará sobre
+la marcha. No muestra duración, provisiones ni dificultades y no permite cancelar.
+Sigue `VIA-APR-01`; el día transcurrido no consume reservas todavía inexistentes.
 
-Cuando otra regla impone un máximo —como el límite de provisiones adicionales
-de `VIA-APR-01`—, la conversión nunca parte una comida ni deja incompleta una
-bota para respetarlo. La comida se añade mediante el procedimiento aleatorio
-anterior hasta alcanzar o superar el objetivo permitido. El agua siempre se
-redondea hacia arriba a botas completas. El excedente causado por la última
-comida o bota permanece en las reservas aunque haga superar ese máximo: estas
-reglas prefieren un excedente indivisible antes que dejar al Grupo por debajo
-de la cantidad que le corresponde obtener.
+<a id="via-rut-02"></a>
+#### VIA-RUT-02 — Propuesta de ruta
 
-El agua se transporta en botas de cuero. Cada bota puede contener 3 litros y su
-cuero pesa 0,1 kg. El agua pesa 1 kg por litro, por lo que una bota llena pesa
-3,1 kg. Al consumir agua, el peso de su contenido disminuye con los litros
-restantes y los 0,1 kg de la bota se conservan.
+**Decisión; 24 horas, antes de crear reservas.** Identifica al proponente y el
+conocimiento que lo habilita; describe principales Lugares, medios, dificultades
+conocidas y tiempo previsto sin demora, sin garantizar duración real.
 
-Durante el aprovisionamiento inicial y al formar el suplemento por sobrecarga,
-la cantidad de agua se redondea hacia arriba a botas completas. Todas se
-incorporan llenas con 3 litros y no se adquieren botas parcialmente llenas. Una
-bota puede quedar parcial o vacía a medida que el Grupo bebe. El recipiente
-vacío no aporta agua y solo puede rellenarse si una escena posterior permite
-conseguirla.
+| Opción | Consecuencia |
+|---|---|
+| Aceptar el plan | Adopta la ruta rápida y sigue `VIA-APR-01`. |
+| Rechazarlo y partir por otro camino | Adopta la ruta larga y sigue `VIA-APR-01`. |
+| Cancelar | Termina `Viajar` antes de partir. |
 
-Siempre que una regla conceda una cantidad objetivo de agua, `Viajar` añade
-todas las botas llenas necesarias para alcanzar al menos esos litros:
+No hay chequeo: `Viajar` calcula la ruta y sus estimaciones. El día pasa
+cualquiera sea la elección, sin consumo de reservas. No revela estimaciones
+de alimento o agua, necesidad base ni el cálculo posterior de aprovisionamiento.
 
-```text
-botas añadidas = redondear hacia arriba(litros objetivo ÷ 3 litros)
-agua incorporada = 3 litros × botas añadidas
-```
+<a id="via-desplazamiento"></a>
+### 8.2. Desplazamientos: reglas comunes
+
+Cada tarjeta recorre una conexión completa mediante dos chequeos de su acción:
+primero la mitad del origen y después la del destino, con el mismo responsable.
+Recibe esos Lugares, Grupo y Características, Actor del jugador, responsable,
+medios disponibles, reservas, contador de consumo y demoras aplicables.
+Cada medio tiene riesgos, resultados y relato propios.
 
-El excedente causado por el redondeo queda disponible y aumenta el peso.
+| Tarjeta | Acción | Dificultad | Obtención de `Conoce`, por mitad | Cuándo hay incidente |
+|---|---|---:|---|---|
+| `VIA-DES-AND` | `Andar` | 5 | Resultado ajustado ≥17. | Ambos ajustados <8 y alguno <5. |
+| `VIA-DES-CAB` | `Cabalgar` | 6 | Resultado ajustado ≥17. | Ambos ajustados <8 y alguno <6. |
+| `VIA-DES-CAM` | `CabalgarEnCamello` | 6 | Resultado ajustado ≥17. | Ambos ajustados <8 y alguno <6. |
+| `VIA-DES-NAV` | `NavegarEnBarco` | 14 | Cara natural ≥18; los bonos no ayudan. | Ambos ajustados <14. |
 
-Cada Actor consume 2 unidades alimenticias y 0,5 litros de agua cada vez que el
-viaje completa un período acumulado de 24 horas. El consumo no es proporcional:
-una cantidad menor de 24 horas no consume una fracción de la ración diaria.
+El umbral de conocimiento se evalúa independientemente por dado y concede a
+**todos** los integrantes una Relación `Conoce` con el Lugar de esa mitad.
+Puede concederse sobre ambos Lugares; se evalúa aunque no haya incidente.
 
-Cuando `VIA-APR-01` forma las reservas, `Viajar` inicia en cero un contador de
-horas desde la última ración. Esa misma tarjeta añade después sus 24 horas de
-preparación. A partir de allí, cada escena que hace pasar tiempo añade su
-duración al contador. Después aplica:
+Si se cumple la condición de incidente, se hace **una sola** tirada de incidente
+de 1d20 y se consulta la tabla del medio. Todas las caras eligen una consecuencia;
+nunca se consulta dos veces para una conexión. Los intervalos avanzan de menor a
+mayor gravedad. Su ubicación depende de los dos resultados ajustados:
 
 ```text
-períodos completados = parte entera(horas acumuladas ÷ 24 horas)
-alimento requerido = A × 2 × períodos completados unidades alimenticias
-agua requerida = A × 0,5 × períodos completados litros
-horas restantes = horas acumuladas − 24 horas × períodos completados
+primer resultado < segundo → mitad del origen
+segundo resultado < primero → mitad del destino
+empate → mitad del destino
 ```
 
-El alimento y el agua se descuentan por separado. Cuando una reserva alcanza,
-se descuenta toda la cantidad requerida. Cuando no alcanza, se consume todo lo
-disponible y esa reserva queda en cero; la otra reserva se descuenta con
-normalidad. Las reservas nunca adquieren valores negativos.
+Tipo y ubicación se resuelven por separado. Si se necesita un Actor afectado
+sin otro criterio expreso, se sortea entre todos los elegibles después de aplicar
+exclusiones; si solo queda uno, se elige ese. Esto no elige al responsable ni
+al proponente.
 
-Para consumir cada ración de alimento, `Viajar` elige al azar entre los tipos de
-comida que todavía quedan y consume una pieza completa del tipo escogido. Resta
-su aporte oculto y repite hasta cubrir la ración o agotar la reserva. Este orden
-no expresa una prioridad nutricional: representa los gustos y caprichos
-momentáneos de los Actores. Si la última pieza aporta más de lo que faltaba, se
-consume completa y el excedente de su aporte no se conserva para la ración
-siguiente.
+Se calculan las duraciones y se aplican los hechos en orden de ubicación,
+con calendario y consumo comunes. Todo tiempo añadido debe corresponder a una
+situación narrada. Un estado adquirido no modifica retroactivamente lo recorrido.
+Una tarjeta solo se convierte en decisión cuando un resultado lo indica.
 
-Las horas restantes se conservan para la escena siguiente. El contador no se
-reinicia entre viñetas, tramos ni demoras: por ejemplo, una escena de 20 horas y
-otra de 6 completan juntas un período, consumen una ración diaria por Actor y
-dejan 2 horas acumuladas. Una escena que lleva el contador exactamente a 24
-horas también provoca el consumo. Llegar exactamente a cero después de pagar
-esa ración cuenta como agotar la reserva, pero no significa que el Grupo haya
-pasado tiempo sin ese recurso.
+<a id="via-des-andar"></a>
+#### 8.2.1. VIA-DES-AND — Andar
 
-Cuando una escena completa varios períodos y una reserva no cubre todos, las
-fronteras de 24 horas se resuelven en orden. Para cada recurso, el tiempo de
-carencia comienza en la primera frontera cuya ración completa no puede pagarse
-y se mide desde allí hasta el final de la escena. El alimento y el agua pueden
-iniciar su carencia en fronteras diferentes y la viñeta conserva ambas
-duraciones por separado. Si una reserva paga su última ración completa y queda
-exactamente en cero, la carencia solo comienza si se alcanza después otra
-frontera que ya no puede pagar.
+**Resolución; decisión ante separación con varios Actores.** Representa
+orientación, lectura del terreno y apertura de camino, además del esfuerzo físico.
+Usa la duración a pie y la comparación de la tabla común: un resultado ≥8
+descarta el incidente aunque el otro falle; dos éxitos también lo descartan.
 
-Ninguna Característica modifica por ahora estas tasas. Los animales de montura
-y de carga no consumen alimento ni agua de las reservas del Grupo.
+| 1d20 de incidente | Consecuencia |
+|---:|---|
+| 1–7 | **Agotamiento momentáneo.** Un Actor se agota; esperarlo añade 2 horas. No obtiene `Cansado`. |
+| 8–13 | **Esguince.** Un Actor obtiene `Esguince` si no tiene ni `Esguince` ni `Cansado`; en otro caso conserva su estado. |
+| 14–17 | **Pérdida del rumbo.** El Grupo toma un camino más difícil. Cada Actor sin `Esguince` ni `Cansado` obtiene `Cansado`; los demás conservan su estado. |
+| 18–20, varios Actores | **Separación de un Actor.** Un integrante distinto del Actor del jugador queda en el Lugar del incidente. Se decide buscarlo o continuar. |
+| 18–20, un solo Actor | **Enfermedad del Actor solitario.** Se detiene 24 horas y se recupera de una enfermedad desconocida. Obtiene `Cansado` solo si no tenía ninguno de los dos estados. |
 
-`VIA-APR-02` aplica sus propias reglas para determinar cuánta comida y agua se
-obtiene durante el recorrido. El agua que conceda se materializa mediante botas
-llenas con la regla anterior.
+**Paso a pie y estados.** `Esguince` añade 4 horas por mitad y `Cansado`, 6;
+equivalen a 8 y 12 horas por conexión normal, respectivamente. Se aplican después
+del tamaño, sin multiplicar esas horas por su factor. El Grupo acompaña al
+integrante más lento: las demoras no se multiplican por cantidad de afectados.
+Los incidentes no acumulan un estado con el otro. Si un Actor ya posee ambos
+por causa externa, sus demoras sí se suman: 10 horas por mitad, 20 por conexión
+normal. `Viajar` no elimina esos estados.
 
-### 4.4. Tiempo y consumo reales
+Un estado adquirido en la mitad del origen afecta la del destino y las
+conexiones posteriores; si se adquiere en destino, rige desde la siguiente
+conexión.
 
-Una regla o tarjeta solo hace pasar tiempo cuando lo establece expresamente y
-señala su duración. Si no menciona un transcurso de tiempo, su resolución dura
-0 horas a efectos del calendario y del consumo; esa omisión no crea una regla
-pendiente. Una conversación breve podría no consumir una cantidad relevante de
-tiempo, mientras que atravesar un valle podría tomar varios días. `Viajar` suma
-cada duración expresa al tiempo total.
+**Separación.** Se sortea al perdido entre los demás integrantes; el Actor del
+jugador nunca puede desaparecer. La desaparición se aplica antes de continuar:
 
-Siempre que pasa tiempo, `Viajar` lo añade tanto al calendario como al contador
-acumulado de la sección 4.3. Por cada período de 24 horas que se complete,
-resta una ración diaria de alimento y agua para todos los Actores. En un
-desplazamiento, la duración real parte de la misma referencia utilizada por el
-plan y después incorpora las consecuencias que la tarjeta asigna al resultado
-del chequeo, la velocidad que limita al Grupo, la parte de la demora total por
-carga que le corresponda y cualquier suceso inesperado. La demora por carga ya
-quedó fijada antes de partir y no vuelve a calcularse en cada tramo, salvo la
-previsión especial de la sección 4.2 después de una pérdida de capacidad.
+- **Buscarlo:** cancela `Viajar` en el Lugar del incidente. La búsqueda y una
+  posible reincorporación quedan fuera de esta acción.
+- **Continuar:** el separado deja de intervenir en el Grupo y el Actor del
+  jugador obtiene la Relación dirigida `Abandono` hacia él. Si ya existe,
+  se conserva sin duplicarla ni modificarla.
 
-Todo tiempo que una tarjeta haga pasar mientras `Viajar` ya mantiene reservas
-avanza tanto el calendario como el contador de consumo. No existe una excepción
-para el tiempo perdido preparando un viaje que luego se cancela. Una tarjeta de
-duración cero no completa por sí misma ninguna frontera de consumo.
+Los dos intentos mantienen al responsable originalmente elegido. La composición
+para la escena se trata según la regla común de pérdida de Actores.
 
-Una duración real mayor puede hacer que las provisiones se agoten antes de lo
-previsto, pero no reinicia el cálculo del aprovisionamiento inicial.
+<a id="via-des-monturas"></a>
+#### 8.2.2. Reglas de los desplazamientos montados
 
-La falta de alimento, de agua o de ambos nunca interrumpe una tarjeta de
-desplazamiento. Aunque las cantidades requeridas para sus jornadas superen las
-reservas disponibles, el Grupo completa toda la conexión y alcanza el Lugar de
-destino de esa tarjeta. Consume todo lo que quede de cada recurso insuficiente,
-deja su reserva en cero y descuenta normalmente el recurso que sí alcance. La
-viñeta identifica si el Grupo avanzó con hambre, con sed o con ambas durante la
-parte restante. La carencia no añade por sí misma tiempo, Características ni
-otras penalizaciones.
+Caballo y camello usan sus duraciones y restricciones territoriales, la
+dificultad 6 y la comparación común: un resultado ≥8 o dos éxitos descartan
+el incidente.
 
-Si todavía quedan conexiones por recorrer, `VIA-APR-02` se presenta en el
-Lugar alcanzado y debe resolverse antes del siguiente desplazamiento. Si la
-conexión termina en el destino final, se resuelve la llegada y no se busca
-alimento ni agua para poder continuar.
+| 1d20 de incidente | Consecuencia |
+|---:|---|
+| 1–6 | **Montura obstinada.** Se desvía; corregirla y recuperar el rumbo añade 4 horas. |
+| 7–11 | **Montura detenida.** Se niega a continuar; reanudar la marcha añade 8 horas. |
+| 12–15 | **Montura asustada.** Corre en otra dirección, cae su jinete y el Grupo se reagrupa y regresa al camino: añade 12 horas. |
+| 16–18 | **Crisis de la montura.** Espera de 48 horas, seguida de la tirada simple de recuperación: ≥8 se repone; <8 muere. |
+| 19–20 | **Muerte inmediata.** Muere sin recuperación. |
 
-Cuando una consecuencia quita a un Actor del Grupo durante una viñeta, no se
-recalculan los chequeos, la duración ni el consumo de la viñeta que ya se está
-resolviendo. Al cerrarla, el Actor deja de formar parte del Grupo y todas las
-escenas posteriores usan la nueva cantidad de integrantes. Antes del siguiente
-desplazamiento, `Viajar` comprueba además la capacidad de carga y las monturas
-disponibles con esa nueva composición.
+Cuando se requiere identificar una montura, se sortea entre las de la especie
+usada. Si también se requiere un Actor, ambos sorteos son independientes;
+el relato puede presentar la montura como la que cabalgaba ese Actor.
 
-Las tarjetas no crean reglas propias para estas cuentas. Pueden mostrar la
-duración, el tamaño del Grupo, la carga, el efecto de un animal, las comidas y
-el agua consumidas y las reservas concretas restantes, pero todos esos valores
-proceden de la misma simulación general.
+Al morir una montura, se reduce inmediatamente su contador. Si todavía hay una
+por Actor, una reserva ocupa su lugar y se conserva medio y velocidad.
+Si ya no alcanzan:
 
-Los desplazamientos expresan su duración en días. El cálculo por tamaño conserva
-las fracciones al determinar y sumar las duraciones, sin redondearlas. El tiempo
-del mundo y el calendario avanzan exactamente la duración resultante; las
-fracciones sobrantes se conservan y se acumulan con las escenas posteriores.
+- **Incidente en origen:** conserva el segundo chequeo montado, sin repetir
+  tirada ni incidente. Solo la duración de la mitad del destino pasa a la
+  referencia de `Andar` con su tamaño; no aplica `Esguince`, `Cansado` ni otros
+  modificadores personales de andar, porque la acción particular sigue siendo
+  montada.
+- **Incidente en destino:** esa mitad conserva su duración montada.
+- **Conexiones posteriores:** se vuelve a elegir medio por la prioridad
+  general con ambos contadores actualizados; puede pasar a la otra especie
+  antes de recurrir a pie.
 
-Cuando una conexión que no es la final termina con la reserva alimenticia, la
-reserva de agua o ambas en cero, `Viajar` incorpora la tarjeta
-[`VIA-APR-02`](#via-apr-02) antes de permitir otro desplazamiento. Si esa
-tarjeta no deja siquiera alimento y agua para una ración completa de un día,
-el viaje se cancela automáticamente. Si deja esa ración pero no consigue los
-recursos necesarios para todo el recorrido restante, la continuación es
-[`VIA-APR-03`](#via-apr-03).
+El relato informa recuento y efecto: reserva disponible, mitad recorrida a pie
+si corresponde, conservación del segundo chequeo y nueva elección antes de
+la conexión siguiente.
 
-## 5. Información que recibe el jugador
+<a id="via-des-cabalgar"></a>
+#### VIA-DES-CAB — Cabalgar
 
-Cada viñeta debe permitir que el jugador entienda qué sucedió y por qué. Una
-afirmación vaga como «el viaje será arduo» no basta cuando las reglas pueden
-explicar qué lo vuelve arduo y qué efecto tiene.
+**Resolución.** Usa caballo, con base de 4 días por conexión normal y crisis
+por ataque de una fiera que deja herida a la montura. Aplica las reglas montadas.
 
-El texto conserva la ficción de una aventura, pero incluye la información
-concreta que sirve para tomar decisiones: las circunstancias relevantes, quién
-actuó, qué intentó, por qué el intento era más fácil o más difícil, cuánto
-tiempo pasó, qué recursos se consumieron y qué consecuencias se produjeron.
-Así, la experiencia de un viaje ayuda al jugador a estimar los riesgos y las
-exigencias materiales de recorridos futuros.
+Sin restricciones ni incidentes, transmite rapidez, amplitud y grandes vistas,
+sin presuponer un paisaje. En terreno lento mantiene al Grupo montado y explica
+la causa: pendientes que obligan a medir apoyos en `Montañoso`, suelo incierto
+en `Pantanoso` u obstáculos y paso estrecho en `Boscoso`. Informa duración
+de la mitad afectada y cuánto habría tomado sin la restricción.
 
-Esta claridad no vuelve omnisciente al jugador. Antes de partir, solo recibe
-una previsión detallada si un Actor del Grupo puede formular un plan de viaje
-según la sección 3. Sin ese Actor, la falta de información forma parte de la
-ficción y de las reglas.
+<a id="via-des-camello"></a>
+#### VIA-DES-CAM — Cabalgar en camello
 
-Cuando el tamaño del Grupo, la cantidad transportada, un animal de carga o una
-Característica alteren un resultado, la viñeta correspondiente debe hacer
-visible ese efecto. Los demás elementos básicos del aprovisionamiento pueden
-permanecer resumidos, pero el alimento y el agua se presentan mediante sus
-formas concretas: piezas de carne seca, panes, lonjas de queso, pescados salados,
-botas y litros. Las unidades alimenticias nunca se muestran al jugador.
+**Resolución.** Usa camello, con base de 5 días por conexión normal y crisis
+por picadura de un animal venenoso que nadie alcanza a ver. Aplica las reglas
+montadas.
 
-Cuando el peso produzca una demora, la explicación debe indicar el peso que
-soportan los Actores, el límite conjunto del Grupo, los kilogramos de exceso y
-las horas añadidas por la regla cuadrática.
+Transmite un avance más rápido que a pie, sostenido y con el balance de la montura,
+sin presuponer un desierto. Para terreno lento usa las mismas causas del caballo,
+adaptadas al camello y mostrando duración afectada y duración sin restricción.
+En `Desierto` `Vasto` hace explícita su ventaja: puede cruzarlo donde el caballo
+no puede, con la duración correspondiente.
 
-Las viñetas distinguen una previsión de un resultado real. Al planificar, el
-texto muestra el tiempo previsto sin demora y los factores conocidos. Después
-de un desplazamiento, muestra cómo la tirada, la velocidad del Grupo, el exceso
-de carga y los imprevistos modificaron esa referencia para producir el tiempo
-real.
+<a id="via-des-navegar"></a>
+#### 8.2.3. VIA-DES-NAV — Navegar en barco
 
-Cuando `Pequeño` o `Vasto` modifique la duración de un tramo, la explicación
-debe identificar cada Lugar pertinente, su Característica, la contribución de
-su mitad y la duración final del tramo.
+**Resolución.** Usa barco en toda conexión con algún extremo `Acuático`.
+Aplica la base de 2 días, ambos tamaños, los dos chequeos y los umbrales de la
+tabla común. Un éxito ajustado ≥14 basta para una buena navegación y descarta
+incidentes. Solo dos resultados <14 hacen consultar su tabla.
 
-La estimación de provisiones de [`VIA-APR-01`](#via-apr-01) constituye una
-excepción deliberada. La viñeta muestra quién hizo el cálculo y qué capacidades
-lo ayudaron, pero no revela si la cantidad estimada alcanza ni cuál era la
-cantidad base oculta. De otro modo, el jugador conocería antes de partir una
-incertidumbre que debe descubrir durante el viaje.
+La dificultad abarca viento, clima, velas, olas y rumbo, no solo conservar
+dirección. En una buena navegación el tono es romántico, placentero, firme y
+amplio; ante dificultades es serio y sombrío, mostrando al Grupo abriéndose
+paso sin desesperanza. No exige un paisaje costero concreto.
 
-Cada tarjeta decide cuánto detalle numérico revela de los chequeos que invoca:
-puede mostrar u ocultar la dificultad, la cara natural, los modificadores y el
-resultado ajustado según las necesidades de esa escena. La acción particular
-solo devuelve esos datos; no decide cuáles ve el jugador. Cualquiera sea el
-formato elegido por la tarjeta, su explicación debe relacionar claramente las
-causas con el resultado.
+**Viento estacional.** Se fija al comenzar la viñeta y no cambia aunque esta
+cruce de mes:
 
-## 6. Clases de viñeta
+| Mes inicial | Dirección hacia la que sopla |
+|---|---|
+| Enero, febrero, marzo | Este |
+| Junio, julio | Oeste |
+| Septiembre, octubre | Norte |
+| Abril, mayo, agosto, noviembre, diciembre | Azar uniforme entre norte, noreste, este, sureste, sur, suroeste, oeste y noroeste. |
 
-Una **viñeta de resolución** no pide una decisión. Resuelve primero la situación
-y luego cuenta qué hizo el Grupo, qué chequeo se realizó, qué resultado obtuvo
-y cómo cambió el viaje. No se divide en pasos artificiales solo para mostrar
-información que el jugador no puede usar para elegir.
+Barco y viento usan esos ocho puntos. La comparación cubre todos los casos:
 
-Una **viñeta de decisión** presenta una situación y la información necesaria
-para que el jugador elija qué hacer. Después aplica la opción elegida, realiza
-los chequeos que correspondan y muestra sus consecuencias.
+| Relación | Criterio | Tiempo sobre la conexión completa |
+|---|---|---:|
+| A favor | Rumbo a un máximo de 45° de la dirección del viento. | −24 horas |
+| Perpendicular | Diferencia de 90°. | 0 |
+| En contra | Rumbo a un máximo de 45° de la dirección opuesta al viento. | +48 horas |
 
-Que una escena necesite más de una ventana para mostrar su texto es una cuestión
-de interfaz y no la convierte en varias viñetas.
+El viento no integra la previsión inicial. Se aplica una vez sobre el total
+posterior al tamaño, junto con las demás sumas y restas de horas; su orden
+aditivo no altera el resultado. Se admite duración real de 0 horas: dos Lugares
+`Pequeño` dan 24 horas y el viento favorable puede eliminarlas.
 
-## 7. Curso general de `Viajar`
+**Incidentes.** La capacidad de navegación afecta a la frecuencia de consulta,
+no al incidente sorteado.
 
-Por ahora, el curso confirmado del viaje es el siguiente:
+| 1d20 de incidente | Consecuencia |
+|---:|---|
+| 1–6 | **Olas y desvíos:** +12 horas. |
+| 7–12 | **Mal tiempo y lluvia:** +24 horas. |
+| 13–16 | **Falta de viento:** inmóvil en el mar, +48 horas. |
+| 17–18 | **Tormenta fuerte:** naufragio según el Lugar del incidente. |
+| 19–20, varios Actores | **Actor por la borda:** un Actor distinto del del jugador sale del Grupo y queda en un Lugar `Costero` aleatorio. |
+| 19–20, un solo Actor | **Tormenta fuerte**, en lugar de caída. |
 
-1. El jugador declara el Lugar al que quiere llevar al Grupo.
-2. `Viajar` determina la ruta rápida y construye a partir de ella la ruta larga,
-   según la sección 3.
-3. Comprueba si algún integrante puede proponer la ruta rápida.
-4. Si no existe un proponente, resuelve [`VIA-RUT-01`](#via-rut-01). Si existe,
-   elige uno al azar entre los candidatos válidos y resuelve
-   [`VIA-RUT-02`](#via-rut-02); el jugador acepta la ruta rápida, adopta la ruta
-   larga o cancela el viaje.
-5. Si el viaje sigue adelante, se resuelve la tarjeta de aprovisionamiento
-   inicial [`VIA-APR-01`](#via-apr-01).
-6. `Viajar` determina `N`. Si `N` es mayor que cero, evalúa la alternativa con
-   `N − 1` animales. Cuando esa alternativa es admisible, resuelve
-   [`VIA-APR-05`](#via-apr-05); de lo contrario, mantiene como obligatoria la
-   cantidad `N`.
-7. `Viajar` incorpora automáticamente la cantidad total de animales de carga
-   fijada por la regla general o por `VIA-APR-05`. No realiza un chequeo y no
-   puede fracasar al conseguirlos. Si la cantidad es cero, no incorpora ninguno.
-8. Si el viaje no fue cancelado y `CalcularProvisiones` no alcanzó su
-   dificultad, se resuelve [`VIA-APR-03`](#via-apr-03); el jugador continúa con
-   el 90 % conseguido o cancela el viaje.
-9. Para cada conexión de la ruta adoptada, `Viajar` elige el medio mediante la
-   prioridad de la sección 3 y presenta exactamente una tarjeta de
-   desplazamiento: `VIA-DES-NAV`, `VIA-DES-CAB`, `VIA-DES-CAM` o
-   `VIA-DES-AND`. La tarjeta recorre la conexión completa antes de que la acción
-   general pase a la siguiente.
-10. Después de cada escena que haga pasar tiempo, `Viajar` actualiza el tiempo
-   total y las reservas de alimento y agua según la sección 4.
-11. Una tarjeta de desplazamiento completa toda su conexión aunque el alimento,
-   el agua o ambos se agoten durante ella. Al cerrarla, el Lugar de destino de
-   esa conexión pasa a ser el Lugar actual del Grupo, salvo que una consecuencia
-   de la propia tarjeta establezca otra cosa.
-12. Si ese Lugar es el destino final, se resuelve
-   [`VIA-LLE-01`](#via-lle-01) y la acción general termina.
-13. Si quedan conexiones y alguna reserva quedó en cero, `Viajar` resuelve
-   primero [`VIA-APR-02`](#via-apr-02). Si el viaje continúa, comprueba la carga
-   con las provisiones resultantes aunque no haya ocurrido una pérdida previa de
-   capacidad. Una pérdida de capacidad también exige la previsión especial de
-   la sección 4.2 y puede activar primero `VIA-APR-02`; esa tarjeta se presenta
-   como máximo una vez dentro de la misma secuencia. Después, `Viajar` resuelve
-   [`VIA-APR-06`](#via-apr-06) si la comprobación aplicable determina que la
-   carga resultante es intransportable. Todas las tarjetas que correspondan se
-   completan antes del siguiente desplazamiento.
-14. El viaje continúa hasta que el Grupo alcanza su destino o una regla o
-   tarjeta ordena terminarlo o cancelarlo.
+El Actor por la borda se sortea entre los elegibles; su costa, entre todos los
+Lugares `Costero` del mundo. Navegar en un mundo sin ninguno es un error de
+diseño para el que no se define recuperación. La caída no añade tiempo.
 
-Si una consecuencia termina el viaje antes de llegar, la viñeta que la produce
-cierra ese episodio mediante su propio relato. La interrupción no concede por
-sí misma una Característica, una Relación ni otro estado especial: solo
-persisten los cambios que una regla haya establecido expresamente. En ese caso
-no se presenta `VIA-LLE-01`.
+En **Tormenta fuerte**, si el Lugar del incidente es `Costero`, el barco choca
+contra esa costa y el Grupo queda allí. Si no lo es, se sortea independientemente
+un Lugar `Costero` para cada Actor; todos salvo el del jugador salen del Grupo.
+El del jugador queda como único integrante y su costa pasa a ser el Lugar actual.
+En ambos casos termina `Viajar` sin llegada.
 
-Las conexiones se recorren en el orden de la ruta. Cada una muestra una tarjeta
-de desplazamiento; no existe una frecuencia aleatoria. Otras tarjetas pueden
-intercalarse entre dos conexiones cuando se cumpla su disparador.
+La tormenta no añade tiempo, muertes, pérdida de provisiones ni daños implícitos.
+Se conserva el tiempo producido por las demás reglas del desplazamiento.
 
-## 8. Acciones particulares utilizadas por `Viajar`
+**Información propia.** Al cierre informa ambos resultados y si se aplicó el +4
+de `Navegante`; si fallaron ambos, también la tirada de incidente y su
+consecuencia. Desglosa base, tamaños, viento y demoras, además del cierre común
+de tiempo y consumo.
 
-Las acciones particulares de esta sección son reglas reutilizables. Cuando una
-acción general invoca una de ellas, el contexto elige al responsable y fija la
-dificultad; la acción particular conserva sus propios modificadores sin
-importar qué acción general la haya invocado.
+### 8.3. Aprovisionamiento y carga
 
-Cada una se resuelve del mismo modo:
+<a id="via-apr-01"></a>
+#### VIA-APR-01 — Aprovisionarse antes de partir
 
-1. El responsable tira 1d20. La cara mostrada es el **resultado natural**.
-2. Se aplican los bonificadores y penalizadores confirmados para esa acción y
-   ese Actor. La suma es el **resultado ajustado**.
-3. Si el resultado ajustado es mayor que 20, se interpreta como 20.
-4. La acción devuelve el resultado natural, cada modificador aplicado, el
-   resultado ajustado y si este alcanzó la dificultad recibida.
+**Resolución; 24 horas.** Antes de recorrer la ruta, el Actor más apto realiza
+una sola acción `CalcularProvisiones`, dificultad 10. No ofrece decisiones;
+la eventual elección de carga pertenece a `VIA-APR-05`.
 
-La acción particular no elige al responsable, no fija su propia dificultad, no
-conoce los Lugares ni el resto de la ruta y no aplica consecuencias narrativas.
-Tampoco hace avanzar el tiempo, consume reservas, concede Relaciones ni elige
-incidentes. Todas esas decisiones pertenecen a la acción general y a la tarjeta
-que interpreta el chequeo.
+Para `A` Actores y `T` horas previstas de recorrido se calcula en secreto:
 
-Las acciones pueden representar técnicas específicas. La capacidad compartida
-entre técnicas se expresa mediante Características: una misma Característica
-puede conceder su bono a varias acciones particulares. Cada Característica
-aplicable aporta su bono una sola vez; si un Actor posee varias Características
-aplicables a la misma acción, sus bonos se suman salvo que la propia acción
-establezca una excepción.
+```text
+períodos previstos = parte entera((24 horas + T) ÷ 24 horas)
+necesidad base de alimento F = 2 × A × períodos previstos unidades alimenticias
+necesidad base de agua W = 0,5 × A × períodos previstos litros
+```
 
-La regla de cada acción particular enumera de forma exhaustiva todas las
-fuentes que modifican su chequeo. Una Característica, circunstancia o relación
-que no figure en esa regla no aporta por analogía ningún bonificador ni
-penalizador. Mientras la lista de una acción todavía contenga una fuente sin
-definir, tampoco puede determinarse de forma definitiva quién es el Actor más
-apto para realizarla.
+Las 24 horas iniciales cubren esta tarjeta. La referencia usa ruta y medios
+adoptados y la configuración de carga estabilizada; no garantiza el consumo real.
 
-Cuando una tarjeta necesita elegir al responsable de una acción particular,
-compara a todos los Actores elegibles que integran el Grupo. Salvo que la
-tarjeta establezca una exclusión expresa, todos los integrantes actuales son
-elegibles. Para cada uno suma únicamente los modificadores que esa misma acción
-aplicaría a su chequeo. El Actor con el total más alto es el más apto y realiza
-la acción. Si varios Actores comparten el total más alto, incluido el caso en
-que ninguno posee un modificador aplicable, se elige al azar solamente entre
-ellos.
+| Resultado ajustado | Multiplicador m sobre F y W |
+|---|---:|
+| <10, fracaso | 0,9 |
+| 10–15, éxito | 1 |
+| ≥16, éxito superior | 1,1 |
 
-La elección se hace una vez por cada acción particular que invoque la tarjeta.
-Cuando una tarjeta realiza dos chequeos de la misma acción para las dos mitades
-de una conexión, el Actor elegido conserva la responsabilidad en ambos. Si una
-misma tarjeta invoca acciones particulares diferentes, elige por separado al
-Actor más apto para cada una y, por lo tanto, puede mostrar a varios integrantes
-colaborando en la escena. Una regla expresa de la tarjeta puede establecer una
-excepción.
+El mismo resultado determina ambos recursos: alimento objetivo `F × m` y agua
+objetivo `W × m`. No hay tiradas separadas ni crece el beneficio por superar 16.
+Se materializan con las reglas comunes y se conservan excedentes indivisibles.
 
-Dentro de `Viajar` se usan estas acciones:
+La preparación resuelve la configuración inicial y su posible `VIA-APR-05`;
+después obtiene automáticamente los animales de carga fijados. También establece
+monturas por especie y disponibilidad del barco según 3.2. Estos medios no
+requieren otro responsable ni chequeo y no añaden tiempo.
 
-| Acción particular | Capacidad que pone a prueba | Uso de `Viajar` | Modificadores y reglas propias |
-|---|---|---:|---|
-| `Andar` | Orientarse, leer el terreno, elegir el paso y abrir camino a pie. | Dificultad 5 en `VIA-DES-AND`. | `Vagabundo` añade +2. |
-| `Cabalgar` | Guiar y dominar un caballo durante el desplazamiento. | Dificultad 6 en `VIA-DES-CAB`. | `Jinete` añade +2. |
-| `CabalgarEnCamello` | Guiar y dominar un camello durante el desplazamiento. | Dificultad 6 en `VIA-DES-CAM`. | `Jinete` añade +2. |
-| `NavegarEnBarco` | Interpretar el viento y el clima, gobernar las velas y conservar un rumbo útil. | Dificultad 14 en `VIA-DES-NAV`. | La Característica `Navegante` del responsable añade +4. |
-| `CalcularProvisiones` | Calcular y reunir el alimento, el agua y los suministros para un viaje. | Dificultad 10 en `VIA-APR-01`; el resultado fija la cantidad final obtenida. | `Administrador` añade +2 y `Expedicionario` añade +2. Un Actor que posea ambas recibe +4. |
-| `Forajear` | Buscar alimento y avituallamiento en un entorno salvaje. | Dificultad 14 en `VIA-APR-02`; cada Actor realiza su propio chequeo. | `Expedicionario` añade +2 y `Herbalista` añade +2. Un Actor que posea ambas recibe +4. |
+La escena identifica al responsable, las Características que lo ayudaron y
+las cantidades concretas finales de cada comida y agua. Narra la obtención de
+animales una vez fijada su cantidad. Con un plan puede relacionar provisiones
+y previsión; sin él conserva la incertidumbre. No revela necesidad base ni si
+las reservas alcanzarán.
 
-Las dificultades de la tabla son las que establecen las tarjetas de `Viajar`;
-no son dificultades universales de esas acciones en cualquier otro contexto.
-No hay todavía otros modificadores de dado confirmados. En particular,
-`Viajero`, `Explorador` y las Relaciones `Conoce` sirven para planificar la
-ruta, mientras que `Esguince`, `Cansado`, el tamaño de los Lugares, el terreno
-y el viento modifican reglas de la viñeta o su duración, no el resultado de
-estas tiradas.
+Una vez formadas las reservas, las 24 horas de esta tarjeta provocan el consumo
+de una ración por Actor. Si el chequeo fue <10, sigue `VIA-APR-03`; en los demás
+casos comienza el recorrido.
 
-El chequeo de recuperación posterior a **Crisis de la montura** no es una
-acción particular. Es una tirada simple de azar de 1d20, sin responsable,
-significado técnico adicional ni modificadores. La propia tarjeta interpreta su
-cara natural mediante el umbral de 8.
+<a id="via-apr-05"></a>
+#### VIA-APR-05 — Decidir ante una sobrecarga moderada
 
-Una **tirada de incidente** tampoco es una acción particular: nadie intenta
-realizarla, no tiene responsable ni dificultad y no recibe modificadores. Solo
-elige una consecuencia después de que una tarjeta ya determinó que ocurrió un
-incidente.
+**Decisión; 0 horas.** Solo aparece si la alternativa inicial con `N − 1`
+animales es admisible. Usa los valores finales del ciclo, incluido el peso del
+suplemento. Si no se cruzó una nueva frontera de consumo, el suplemento es nulo.
 
-## 9. Tarjetas de viñeta confirmadas
+Identifica a quienes participaron del aprovisionamiento y presenta `N`,
+`N − 1` —o ningún animal—, peso sobre Actores, límite, exceso, horas añadidas
+y suplemento concreto de carne seca, botas llenas y litros. Con plan puede
+mostrar tiempo total; sin plan solo revela las horas añadidas, no la base ni
+el total ocultos.
 
-| Clave | Tarjeta |
+| Opción | Consecuencia |
 |---|---|
-| `VIA-RUT-01` | Partir sin un plan |
-| `VIA-RUT-02` | Propuesta de ruta |
-| `VIA-DES-AND` | Andar |
-| `VIA-DES-CAB` | Cabalgar |
-| `VIA-DES-CAM` | Cabalgar en camello |
-| `VIA-DES-NAV` | Navegar en barco |
-| `VIA-APR-01` | Aprovisionarse antes de partir |
-| `VIA-APR-05` | Decidir ante una sobrecarga moderada |
-| `VIA-APR-03` | Reconsiderar un aprovisionamiento incompleto |
-| `VIA-APR-02` | Conseguir alimento y agua durante el viaje |
-| `VIA-APR-06` | Afrontar una sobrecarga imprevista |
-| `VIA-LLE-01` | Llegar al destino |
+| Aceptar la demora | Incorpora el suplemento sin chequeo, fija una sola vez la demora final y obtiene `N − 1` animales. |
+| Evitar la demora por carga | No incorpora suplemento, fija demora cuadrática cero y obtiene `N` animales. |
 
-Las cuatro tarjetas de desplazamiento son distintas porque representan medios y
-experiencias de viaje diferentes. Forman una misma familia porque cumplen la
-misma función general: resolver y narrar el desplazamiento del Grupo a lo largo
-de una parte de la ruta.
+La cantidad elegida se obtiene automáticamente y no puede fracasar.
 
-<a id="via-rut-01"></a>
-### 9.1. Partir sin un plan
+<a id="via-apr-03"></a>
+#### VIA-APR-03 — Reconsiderar un aprovisionamiento incompleto
 
-> **VIA-RUT-01 — Partir sin un plan**  
-> Tipo: viñeta de resolución
+**Decisión; 0 horas.** Ocurre tras el fracaso de `VIA-APR-01` —objetivo del
+90 %— o cuando `VIA-APR-02` deja alimento y agua para al menos un día pero no
+para todo el recorrido restante.
 
-Esta tarjeta se presenta cuando ningún integrante del Grupo puede proponer la
-ruta rápida según la sección 3. La tarjeta adopta la ruta larga como recorrido
-del viaje, pero no la revela al jugador. Las reglas conocen el recorrido aunque
-los personajes no lo conozcan de antemano.
+Antes de partir advierte que se obtuvo menos de lo previsto y no se sabe si
+alcanzará, sin revelar necesidad base ni duración de reservas. Durante el viaje
+identifica el recurso insuficiente y el Lugar actual.
 
-La viñeta introduce el viaje y explica que el Grupo conoce la dirección
-general del destino, pero no cuenta con alguien capaz de trazar un recorrido
-fiable. Sus integrantes se disponen a partir y decidirán por dónde avanzar
-sobre la marcha.
+| Opción | Consecuencia |
+|---|---|
+| Viajar de todas formas | Conserva lo obtenido y comienza el recorrido o continúa desde el Lugar actual hacia la siguiente conexión. |
+| Cancelar | Termina `Viajar` en el Lugar actual. |
 
-La tarjeta no ofrece una previsión de duración, alimento, agua ni dificultades
-y tampoco permite cancelar el viaje. Al terminar, `Viajar`
-continúa con el aprovisionamiento inicial.
+El suplemento de sobrecarga solo cubre su demora y no corrige por sí mismo
+el aprovisionamiento incompleto. Si cancela antes de partir, el Grupo devuelve
+o vende provisiones y animales; durante el recorrido, se descartan las cuentas
+temporales al terminar. No hay chequeo, nueva viñeta ni efecto económico.
+Se conserva todo el tiempo y consumo anteriores.
 
-Preparar la salida hace avanzar el calendario exactamente 1 día. Como las
-reservas temporales todavía no fueron creadas, ese día no se descuenta de ellas.
+<a id="via-apr-02"></a>
+#### VIA-APR-02 — Conseguir alimento y agua durante el viaje
 
-<a id="via-rut-02"></a>
-### 9.2. Propuesta de ruta
+**Resolución; 6 horas.** Aparece antes de otra conexión, en el Lugar actual,
+si alguna reserva terminó en cero o si la previsión por pérdida de capacidad
+detecta que no alcanzará para el resto. No aparece después de llegar al destino.
+Puede buscar solo alimento, solo agua o ambos; no ofrece una decisión.
 
-> **VIA-RUT-02 — Propuesta de ruta**  
-> Tipo: viñeta de decisión
+Los **recursos necesarios** cubren todo el recorrido restante previsto, con
+ruta, medios, estados y demora de carga conocidos, sin anticipar tiradas ni
+incidentes. Incluyen las 6 horas de búsqueda y las `h` horas desde la última ración:
 
-Esta tarjeta se presenta cuando al menos un integrante del Grupo puede proponer
-la ruta rápida según la sección 3.
-
-La viñeta identifica al proponente y explica qué conocimiento le permitió
-formular el plan. Después describe los principales Lugares del recorrido, los
-medios de transporte necesarios, las dificultades conocidas y la duración
-prevista.
-
-La duración presentada es el tiempo previsto sin demora de la sección 4.1. No
-incluye tiradas futuras ni acontecimientos inesperados y no garantiza cuánto
-durará realmente el viaje.
-
-El jugador elige entre:
-
-- **Aceptar el plan:** la ruta rápida se convierte en el recorrido del viaje y
-  `Viajar` continúa con el aprovisionamiento inicial.
-- **Rechazar el plan y partir por otro camino:** la ruta larga se convierte en
-  el recorrido del viaje y `Viajar` continúa con el aprovisionamiento inicial.
-- **Cancelar el viaje:** la acción general termina antes de que el Grupo parta.
-
-La propuesta no requiere un chequeo y hace avanzar el calendario exactamente 1
-día, cualquiera sea la elección posterior. Como las reservas temporales todavía
-no fueron creadas, ese día no se descuenta de ellas.
-
-La lógica de `Viajar` calcula la ruta y las estimaciones del plan; no las
-resuelve una acción particular. La tarjeta no muestra una estimación de
-alimento o agua ni revela la necesidad base, el cálculo oculto posterior o su
-resultado. La ruta rápida es la de menor duración prevista según la sección 3;
-si varias empatan, se usa la que esa sección escoge al azar.
-
-<a id="via-desplazamiento"></a>
-### 9.3. Familia de desplazamiento
-
-> **VIA-DES-AND — Andar**  
-> **VIA-DES-CAB — Cabalgar**  
-> **VIA-DES-CAM — Cabalgar en camello**  
-> **VIA-DES-NAV — Navegar en barco**
-
-Cada una de estas tarjetas se usa cuando el Grupo recorre una parte de la ruta
-mediante el modo que la tarjeta representa. Una vez establecidos la ruta y el
-medio, la tarjeta resuelve el desplazamiento y le cuenta al jugador lo ocurrido.
-Son tarjetas distintas porque cada medio posee sus propios riesgos, resultados
-y relato; no son cuatro apariencias de una misma resolución. Una tarjeta puede
-convertirse en una viñeta de decisión cuando uno de sus resultados lo indique.
-
-Las cuatro tarjetas calculan cada conexión mediante sus dos mitades: primero la
-que pertenece al Lugar de origen y después la que pertenece al Lugar de
-destino. Cada mitad usa el tamaño, el terreno, el medio y el estado del Grupo
-que le correspondan. Esta regla común no determina cuántos chequeos realizan
-las otras tarjetas ni cuáles son sus resultados; cada una los establece por
-separado.
-
-Cada tarjeta parte de la duración universal de su medio indicada en la sección
-4.1. Si un resultado añade tiempo, debe asociarlo con una situación concreta y
-narrarla como parte del viaje. No basta con informar que el Grupo avanzó más
-despacio.
-
-La viñeta debe identificar la parte del recorrido, el medio empleado, los
-integrantes que viajaron y las condiciones que influyeron. Si corresponde un
-chequeo, explica quién intervino, qué dificultad encontró, por qué surgió esa
-dificultad y qué resultado produjo. Al finalizar, informa cuánto duró el
-desplazamiento, qué comidas y cuánta agua consumió el Grupo y si alcanzó el
-próximo punto del recorrido o sufrió alguna complicación.
-
-Si el alimento, el agua o ambos no alcanzaron para todas las jornadas, la
-tarjeta no se interrumpe. Narra que el Grupo agotó cada recurso insuficiente y
-sostuvo con mayor esfuerzo el resto de la conexión, muestra por separado las
-reservas finales y alcanza igualmente el Lugar de destino de la tarjeta. Si un
-recurso llegó exactamente a cero al pagar la última ración, informa que se
-agotó al llegar pero no describe hambre ni sed. Cuando ese Lugar no es el
-destino final del viaje, la continuación es `VIA-APR-02` antes de otro
-desplazamiento.
-
-Las reglas de las cuatro tarjetas se establecen a continuación. Las
-restricciones territoriales de los desplazamientos montados se aplican según la
-sección 4.1, pero no hacen que esas tarjetas usen los resultados de `Andar`.
-
-#### Tiradas de incidente
-
-Cada tarjeta de desplazamiento establece una comparación entre sus dos
-chequeos para decidir si ocurre un incidente. Esa comparación solo decide si se
-consulta la tabla; no selecciona el tipo de incidente.
+```text
+períodos necesarios = parte entera(
+  (h + 6 horas + duración restante conocida) ÷ 24 horas
+)
+alimento necesario = 2 × A × períodos necesarios unidades alimenticias
+agua necesaria = 0,5 × A × períodos necesarios litros
+```
 
-Cuando la comparación lo ordena, se realiza una única tirada de incidente de
-1d20 y se consulta la tabla de la tarjeta. Se lee siempre la cara natural: la
-tirada no tiene dificultad, responsable, bonificadores ni penalizadores. Todas
-sus caras producen exactamente un incidente y nunca se consulta más de una vez
-la tabla durante una misma conexión. Dentro de cada tabla, los intervalos
-avanzan desde las complicaciones menos graves hacia las más graves.
+Cada Actor actual realiza una vez `Forajear`, dificultad 14. Por cada éxito
+se obtiene de forma independiente, para cada recurso buscado:
 
-El tipo y la ubicación del incidente se resuelven por separado. La tirada de
-incidente elige el tipo. Para ubicarlo, se comparan los dos resultados ajustados
-de la acción particular: si el primero es menor, ocurre en la mitad del Lugar
-de origen; si el segundo es menor, ocurre en la mitad del Lugar de destino. Si
-empatan, ocurre en la mitad del destino. Esta regla se aplica a las cuatro
-tarjetas de desplazamiento.
+- Alimento: un entero aleatorio entre 2 y 10 piezas de carne seca, inclusive.
+- Agua: una cantidad aleatoria entre 2 y 10 litros, inclusive.
 
-Cuando la consecuencia de una viñeta de desplazamiento debe escoger un Actor
-afectado y no establece otro criterio, lo escoge al azar entre todos los
-Actores elegibles. Primero se aplican las exclusiones expresas de la tarjeta;
-si después queda un solo Actor, ese Actor es el elegido. Esta regla no elige al
-responsable de un chequeo ni al proponente de una ruta.
+Los fracasos no aportan recursos. Se suman los éxitos; el agua total se
+convierte después a botas llenas. La comida de esta búsqueda es siempre carne
+seca, en lugar de la mezcla aleatoria de otros aprovisionamientos.
 
-<a id="via-des-andar"></a>
-#### 9.3.1. Andar
+Explica necesidad, oportunidades del entorno, intento de cada Actor y cantidades
+logradas. Tras incorporar lo obtenido y aplicar las 6 horas con consumo normal:
 
-> **VIA-DES-AND — Andar**  
-> Tipo: viñeta de resolución; se convierte en viñeta de decisión ante el
-> incidente **Separación de un Actor** cuando el Grupo tiene más de un Actor.
+1. Si alguna reserva no cubre **una ración completa de un día para todos**
+   —2 unidades alimenticias y 0,5 litros por Actor—, cancela automáticamente
+   en el Lugar actual, sin ofrecer `VIA-APR-03`.
+2. Si ambas cubren un día pero no los recursos necesarios para todo el resto,
+   presenta `VIA-APR-03`.
+3. Si continúa, directamente o por esa decisión, comprueba siempre la carga
+   con las nuevas reservas según 5.3. Si corresponde, presenta `VIA-APR-06`;
+   en otro caso inicia la conexión siguiente desde el mismo Lugar.
 
-Esta tarjeta recorre a pie una conexión completa entre un Lugar de origen y un
-Lugar de destino. Representa la orientación, la elección del paso, la lectura
-del terreno y la capacidad de abrir camino; no comprueba solamente si los
-Actores pueden poner un pie delante del otro.
+<a id="via-apr-06"></a>
+#### VIA-APR-06 — Afrontar una sobrecarga imprevista
 
-La tarjeta recibe ambos Lugares, el Grupo y sus Características, la identidad
-del Actor del jugador, el responsable elegido para guiarlo, las reservas, el
-contador de horas desde la última ración y las demoras generales que `Viajar`
-deba aplicar. Resuelve dos intentos de la acción particular `Andar`: uno
-asociado con la mitad del Lugar de origen y otro con la mitad del Lugar de
-destino. Cada intento tira un dado de veinte caras contra dificultad 5, pero
-los incidentes no se resuelven por separado: primero se conocen ambos resultados
-y después se comparan.
+**Decisión; 0 horas.** Solo aparece cuando la comprobación de 5.3 determina
+carga intransportable: exceso actual y reservas insuficientes para el tiempo
+restante con esa carga. Puede resultar de perder un Actor o animal de carga,
+o de las provisiones de `VIA-APR-02`; la muerte de una montura no la activa.
 
-Cada resultado ajustado de 17 o más se evalúa de manera independiente. Todos
-los Actores que integran el Grupo obtienen una Relación `Conoce` con el Lugar
-asociado a ese dado. Si ambos resultados son 17 o más, todos obtienen la
-Relación con ambos Lugares.
+La búsqueda que corresponda se resuelve primero, una sola vez en la secuencia.
+Se recalcula con su resultado antes de presentar la decisión; nunca se modifica
+la conexión ya terminada.
 
-Después se determina si ocurre un incidente:
+Explica la causa de la pérdida de capacidad o la nueva carga, cuántos Actores
+y animales quedan, peso transportado, exceso, tiempo restante, demora y
+reservas insuficientes. Calcula y muestra de antemano un **lote mínimo** de
+comidas y botas enteras cuya retirada elimine todo `x`, dejando el peso dentro
+de los 100 kg por animal más 10 kg por Actor.
 
-1. Si al menos uno de los dos resultados ajustados es 8 o más, no ocurre
-   ningún incidente, aunque el otro no alcance la dificultad.
-2. Si ambos son menores que 8, pero los dos alcanzan la dificultad 5, tampoco
-   ocurre un incidente.
-3. Si ambos son menores que 8 y al menos uno es menor que 5, se realiza una
-   única tirada de incidente y se consulta la tabla siguiente.
+Intenta repartir por igual el peso descartado entre alimento y agua, respetando
+objetos indivisibles. Si una reserva no puede aportar su mitad, completa con
+la otra. Muestra cada objeto del lote; el jugador no redefine cantidades.
 
-| Tirada de incidente | Incidente |
-|---:|---|
-| 1–7 | **Agotamiento momentáneo.** Un Actor se agota. El Grupo espera a que se recupere y el desplazamiento añade 2 horas. No obtiene `Cansado`. |
-| 8–13 | **Esguince.** Un Actor se esguinza. Si no posee `Esguince` ni `Cansado`, obtiene `Esguince`. Si ya posee cualquiera de las dos Características, no obtiene otra ni empeora su estado. Mientras `Esguince` limite el paso, una conexión normal recorrida a pie añade 8 horas: 4 por cada mitad normal. |
-| 14–17 | **Pérdida del rumbo.** El Grupo recorre un camino más difícil. Cada integrante que no posea `Esguince` ni `Cansado` obtiene `Cansado`; quien ya posea cualquiera de las dos Características conserva su estado sin recibir otra. Mientras `Cansado` limite el paso, una conexión normal recorrida a pie añade 12 horas: 6 por cada mitad normal. |
-| 18–20, con varios Actores | **Separación de un Actor.** Un Actor distinto del Actor del jugador desaparece del Grupo y queda en el Lugar donde ocurra el incidente. El jugador decide si detiene el viaje para buscarlo o continúa sin él. |
-| 18–20, con un solo Actor | **Enfermedad del Actor solitario.** El Actor sufre una enfermedad desconocida, permanece detenido 24 horas y se recupera. Si no posee `Esguince` ni `Cansado`, obtiene `Cansado`; si ya posee cualquiera de las dos, conserva su estado sin recibir otra. Las 24 horas avanzan el calendario y activan el consumo normal. |
+| Opción | Consecuencia |
+|---|---|
+| Abandonar provisiones | Quita definitivamente el lote y verifica que desapareció la demora por exceso. Se permite aunque las reservas resultantes puedan ser insuficientes después. |
+| Cancelar el viaje | Termina `Viajar` en el Lugar actual. |
 
-Después de fijar el incidente y la duración real de cada mitad, `Viajar` ordena
-los hechos según el Lugar donde ocurrieron, avanza el calendario, acumula sus
-horas y aplica el consumo de la sección 4.3.
+No obtiene animales nuevos ni otra ayuda. Si el descarte deja una reserva en
+cero, no repite `VIA-APR-02` en esta secuencia: puede completar la conexión
+siguiente sin ese recurso según la regla común. Al terminarla, buscará si
+quedan conexiones; si alcanzó el destino final, resolverá la llegada.
 
-Los modificadores de `Esguince` y `Cansado` alteran el paso a pie del Actor; la
-regla general hace que todo el Grupo acompañe al integrante más lento. No se
-multiplican por la cantidad de Actores afectados ni se duplican. Los incidentes
-de esta tarjeta tampoco pueden acumular una Característica con la otra: cuando
-un incidente fuera a conceder cualquiera de ellas a un Actor que ya posee una
-de las dos, se ignora la nueva concesión. Si un Actor ya posee ambas
-Características por una causa externa, ambas rigen su paso y sus demoras se
-suman: en una conexión normal recorrida a pie añaden 20 horas en total, 10 por
-cada mitad normal. Ninguna regla de `Viajar` elimina `Esguince` ni `Cansado`:
-una vez obtenida, la Característica permanece hasta que otra regla futura la
-quite.
+<a id="via-lle-01"></a>
+### 8.4. VIA-LLE-01 — Llegar al destino
 
-Para combinar estas demoras con el tamaño, primero se calcula la duración de la
-mitad mediante su base y el factor `Pequeño`, normal o `Vasto`. Después se suman
-los modificadores que añaden horas y se restan los que las reduzcan. De este
-modo, las 4 horas de `Esguince` y las 6 horas de `Cansado` por mitad no se
-multiplican por el factor de tamaño. Una Característica comienza a regir en el
-momento en que se asigna y no modifica retroactivamente el tiempo ya
-transcurrido. Si el incidente ocurrió en la mitad del Lugar de origen, afecta
-la mitad del destino y los desplazamientos posteriores. Si ocurrió en la mitad
-del Lugar de destino, comienza a modificar la conexión siguiente.
+**Resolución; 0 horas.** Se presenta inmediatamente después del tiempo, consumo
+y consecuencias del último desplazamiento. No realiza chequeos ni pide decisiones.
 
-Cuando ocurre **Separación de un Actor**, el Actor del jugador no es elegible
-para desaparecer. El Actor perdido se escoge al azar entre los demás
-integrantes del Grupo, según la regla común. Queda en el Lugar donde se ubique
-el incidente mediante la comparación de ambos chequeos.
+Cierra con una narración breve que nombra el destino. Una ruta de 2–4 Lugares,
+contando ambos extremos, recibe un cierre ligero; una de 5 o más enfatiza
+distancia, cansancio y satisfacción de llegar. Informa tiempo total y provisiones
+concretas restantes, sin recapitular Lugares, chequeos, consumos, incidentes ni
+decisiones del recorrido.
 
-La desaparición se aplica antes de continuar. Si el jugador elige seguir, el
-Actor separado deja de intervenir en el Grupo. En ese mismo momento, el Actor
-del jugador obtiene una Relación dirigida de subtipo `Abandono` cuyo destino es
-el Actor perdido. La Relación registra la decisión para que acontecimientos o
-reglas posteriores puedan consultarla:
+Identifica reservas agotadas y distingue agotamiento al llegar de carencia previa.
+No busca más provisiones ni concede Características, Relaciones u otros estados.
+Después de mostrar el remanente, termina `Viajar` y descarta sus cuentas temporales.
 
-```text
-Actor del jugador --Abandono--> Actor perdido
-```
+<a id="pendientes"></a>
+## 9. Decisiones pendientes
 
-Si el jugador decide buscar al Actor separado, `Viajar` se cancela en el Lugar
-del incidente y no presenta `VIA-LLE-01`. La búsqueda y una posible
-reincorporación del Actor quedan fuera de esta acción general. Si decide
-continuar, se aplican la separación y la Relación `Abandono` descritas arriba.
-Si esa misma Relación dirigida ya existía, se conserva sin duplicarla ni
-modificarla. Los dos intentos de `Andar` conservan al responsable elegido
-mediante la regla de la sección 8.
+Falta decidir qué ocurre cuando no existe una ruta válida, existe una sola o
+no puede construirse una ruta larga dentro del intervalo exigido, y si una ruta
+puede repetir un Lugar. No se completan estos casos por analogía. Cada decisión
+confirmada debe incorporarse a su regla y retirarse de esta lista.
 
-##### Presentación narrativa
+<a id="material-retirado"></a>
+## 10. Material retirado para revisión — excluido del cuerpo principal
 
-`VIA-DES-AND` no presenta una descripción genérica seguida por un resumen
-numérico. Convierte la resolución completa en un solo relato en segunda persona
-desde la perspectiva del jugador. Sigue el orden de los hechos: mitad del
-origen, mitad del destino, descubrimientos, un posible incidente y un cierre
-que resume el consumo total de la viñeta. El incidente se narra solo después de
-comparar ambos resultados, pero se ubica dentro del relato en el Lugar donde
-haya ocurrido.
+Este apartado conserva formulaciones que se retiraron para abreviar el reglamento
+y que pueden recuperarse como banco de ejemplos. No agrega reglas: los requisitos
+de información, tono y consecuencias están en el cuerpo principal. Los ejemplos
+son opcionales; sus detalles de ambientación no añaden tiempo ni efectos.
 
-La narración debe volver visibles las causas y cantidades que permiten entender
-las reglas: quién guía, cuánto aporta cada mitad y por qué, qué estado o
-resultado provoca una demora, cuántas horas añade, cuándo se completa una
-jornada, cuántas jornadas de consumo abarca el tramo, qué se consumió en total
-y cuál es el estado final. Los pasajes siguientes son ejemplos de tono y
-formulación, no textos fijos, piezas que deban mostrarse por separado ni reglas
-adicionales. Pueden reescribirse, combinarse u omitirse si el relato resultante
-comunica la misma información. Las expresiones entre llaves se sustituyen por
-los datos reales.
+Las repeticiones de reglas se consolidaron y no se archivan otra vez. Aquí se
+conserva el contenido con posible valor propio: prosa concreta y algunas
+explicaciones didácticas. Este apartado no cuenta para medir el tamaño del
+reglamento reescrito.
 
-Si el jugador guía la marcha:
+### A. Ejemplos de marcha a pie, estados y cierre
 
@@ -1204,4 +954,2 @@
 
-Si otro Actor guía la marcha:
-
 > Dejás atrás el corazón de {origen} y seguís a {responsable} rumbo a
@@ -1209,4 +957,2 @@
 > examina las huellas y elige por dónde debe avanzar el Grupo.
-
-Al entrar en el Lugar de destino:
 
@@ -1215,4 +961,2 @@
 > el paso y deja atrás, poco a poco, los lindes de {origen}.
-
-El tamaño de cada Lugar incorpora uno de estos fragmentos cuando corresponde:
 
@@ -1224,5 +968,2 @@
   lo que aportaría un Lugar normal».
-
-Cuando un resultado de 17 o más concede conocimiento del terreno, puede
-explicarse así:
 
@@ -1232,6 +973,2 @@
   integrantes obtienen la Relación `Conoce` con {lugar}».
-
-Si al menos uno de los resultados es 8 o más y la comparación descarta los
-incidentes, puede narrarse cómo la buena parte del intento compensó la más
-débil:
 
@@ -1242,5 +979,2 @@
 
-Si ocurre un incidente, se usa una sola formulación correspondiente a la
-tirada de incidente:
-
 - **Agotamiento momentáneo (1–7):** «El esfuerzo termina por alcanzar a
@@ -1267,5 +1001,2 @@
   ponerte en marcha, pero la debilidad permanece: {cambio_de_Cansado}».
-
-Si el jugador decide continuar sin el Actor perdido, la tarjeta narra y muestra
-la Relación resultante:
 
@@ -1281,5 +1012,2 @@
 
-Cuando una Característica ya reduce el paso antes de un intento, la explicación
-la hace visible:
-
 - **`Esguince`:** «{actor_limitante} no logra sostener el paso de los demás. El
@@ -1290,19 +1018,3 @@
   {demora_mitad}».
-
-Si un Actor posee ambas Características por una causa externa, el texto incluye
-las dos explicaciones y muestra también la demora total que producen juntas.
-
-La simulación descuenta las raciones en cada frontera de 24 horas según la
-sección 4.3, pero la narración no interrumpe el recorrido con un informe por
-cada comida. Al cierre, la tarjeta reúne todo lo consumido desde que comenzó la
-viñeta hasta que terminó. Enumera las cantidades reales de cada tipo que se
-haya consumido —piezas de carne seca, panes, lonjas de queso y piezas de
-pescado salado— y la cantidad de agua bebida. Nunca nombra unidades
-alimenticias.
 
-El cierre debe dejar claro que esas comidas y bebidas se consumieron a lo largo
-del recorrido, no todas durante la escena final. La fogata puede vestir el
-remate con la ficción del mundo, pero no es un acontecimiento obligatorio ni el
-momento mecánico en que se descuentan las reservas. Por ejemplo:
-
 > Cuando el tramo queda atrás, pasaron {duración_real_total}. La mitad de
@@ -1319,5 +1031,2 @@
 
-Si durante la viñeta no se completa una nueva frontera de 24 horas, el cierre
-lo explica sin descontar otra ración:
-
 > Desde la última ración se acumularon {horas_acumuladas} horas. Durante este
@@ -1325,7 +1034,2 @@
 
-Si un recurso se agota antes de cubrir todas las jornadas, el cierre sustituye
-el consumo inexistente por una explicación del esfuerzo. Incluye solamente las
-frases que correspondan y conserva tiempos separados porque el alimento y el
-agua pueden acabarse en momentos distintos:
-
 > {Si faltó alimento: Durante {tiempo_sin_alimento}, avanzás con el Grupo bajo
@@ -1336,108 +1040,4 @@
 > {alimento_agua_o_ambos}.
-
-Los detalles puramente narrativos de estos ejemplos no modifican el estado ni
-agregan horas. Solo una consecuencia mecánica indicada por la tarjeta altera
-la duración. La cantidad concreta de comidas que aparece en estos textos
-depende de los sorteos de aprovisionamiento y consumo y de la regla de la
-sección 4.3 para pagar una ración con piezas concretas.
 
-<a id="via-des-monturas"></a>
-#### Reglas comunes a los desplazamientos montados
-
-`VIA-DES-CAB` y `VIA-DES-CAM` resuelven una conexión completa mediante dos
-intentos, igual que `VIA-DES-AND`: el primero se asocia con la mitad del Lugar
-de origen y el segundo con la mitad del Lugar de destino. `VIA-DES-CAB` usa la
-acción particular `Cabalgar`; `VIA-DES-CAM` usa `CabalgarEnCamello`. Primero se
-conocen ambos resultados y después la tarjeta determina si ocurrió un único
-incidente.
-
-Ambas tarjetas reciben los Lugares de origen y destino, el Grupo y sus
-Características, el responsable elegido para ambos intentos mediante la sección
-8, las monturas, las reservas, el contador de horas desde la última ración y
-las demoras generales que deban aplicarse.
-
-En ambas tarjetas, cada intento tira un dado de veinte caras contra dificultad
-6. Un 6 alcanza la dificultad y cuenta como éxito.
-
-Cada resultado ajustado de 17 o más se evalúa de manera independiente. Todos
-los Actores que integran el Grupo obtienen una Relación `Conoce` con el Lugar
-asociado a ese dado. Si ambos resultados son 17 o más, todos obtienen la
-Relación con ambos Lugares.
-
-Cada tarjeta aplica la siguiente comparación:
-
-1. Si al menos uno de los dos resultados ajustados es 8 o más, no ocurre un
-   incidente, aunque el otro resultado no alcance la dificultad.
-2. Si ambos son menores que 8, pero los dos alcanzan la dificultad 6, tampoco
-   ocurre un incidente.
-3. Si ambos son menores que 8 y al menos uno es menor que 6, se realiza una
-   única tirada de incidente y se consulta la tabla siguiente.
-
-Los resultados desfavorables comunes a ambas tarjetas son:
-
-| Tirada de incidente | Incidente |
-|---:|---|
-| 1–6 | **Montura obstinada.** Un Actor tiene dificultades para guiarla porque se obstina en avanzar en otra dirección. Corregirla y recuperar el rumbo añade 4 horas. |
-| 7–11 | **Montura detenida.** Una montura se niega a continuar. Conseguir que reanude la marcha añade 8 horas. |
-| 12–15 | **Montura asustada.** Una montura corre en otra dirección. Su jinete cae, el Grupo tarda en reagruparse y después debe regresar al camino. El incidente añade 12 horas. |
-| 16–18 | **Crisis de la montura.** La montura sufre la crisis propia de la tarjeta. El Grupo espera 48 horas y después realiza un chequeo de recuperación contra dificultad 8. |
-| 19–20 | **Muerte inmediata.** Una montura muere sin chequeo de recuperación. `Viajar` reduce en uno el contador de esa especie y comprueba si todavía alcanza para todo el Grupo. |
-
-Las demoras de 4, 8, 12 o 48 horas se suman a la duración real, avanzan el
-calendario y pueden completar nuevas jornadas de consumo.
-
-En **Crisis de la montura**, un resultado de 8 o más en el chequeo de
-recuperación indica que la montura se repone y permite continuar. Con un
-resultado menor que 8, la montura muere y se aplican el mismo recuento y las
-mismas consecuencias que en **Muerte inmediata**. Es una tirada simple de azar
-de 1d20: no representa la acción de ningún Actor, no tiene responsable y no
-recibe Características ni otros modificadores.
-
-Después de ubicar el incidente, la tarjeta resuelve las dos mitades en orden. Si
-una montura muere, `Viajar` reduce inmediatamente en uno el contador de su
-especie. Si todavía queda al menos una montura apropiada por Actor, el Grupo
-conserva el medio y la velocidad previstos. Si quedan menos monturas que
-Actores, se aplican estas reglas:
-
-- si la muerte ocurre en la mitad del origen, el segundo intento conserva la
-  tirada y la acción particular `Cabalgar` o `CabalgarEnCamello` que ya se
-  resolvieron; no se vuelve a tirar ni se busca otro incidente. Solo la duración
-  de la mitad del destino se vuelve a calcular con la referencia universal de
-  `Andar` y su factor de tamaño. Como la acción particular continúa siendo
-  montada, `Esguince`, `Cansado` y otros modificadores personales de `Andar` no
-  alteran esta duración;
-- si la muerte ocurre en la mitad del destino, esa mitad conserva la duración
-  montada que ya le correspondía;
-- después de terminar la viñeta, `Viajar` consulta ambos contadores reducidos y
-  vuelve a aplicar la prioridad de medios de la sección 3. Por eso los
-  desplazamientos siguientes pueden cambiar a la otra especie de montura si
-  alcanza para todos; solo se resuelven mediante `VIA-DES-AND` cuando ninguna
-  de las dos alcanza o puede atravesar la conexión.
-
-Cuando un incidente necesita identificar a un Actor afectado, se lo escoge al
-azar mediante la regla común. Cuando necesita identificar una montura afectada,
-se escoge al azar entre las monturas de la especie usada en la viñeta. Si el
-incidente menciona además a un Actor, ambas selecciones se realizan por
-separado; la narración puede presentar esa montura como la que cabalgaba el
-Actor elegido.
-
-Usar solamente la referencia temporal de `Andar`, sin sus modificadores
-personales, es una simplificación deliberada: evita cambiar de acción particular
-dentro de una viñeta que ya resolvió sus dos intentos montados.
-
-Después de resolver los intentos y cualquier incidente, la tarjeta calcula la
-duración real de ambas mitades, avanza el calendario y aplica el consumo de la
-sección 4.3. Las monturas no consumen alimento ni agua de las reservas del
-Grupo.
-
-La narración se presenta en segunda persona y sigue el mismo criterio de
-claridad que `VIA-DES-AND`: identifica al responsable, el medio, la duración de
-cada mitad, los tamaños y condiciones que la modificaron y cualquier
-consecuencia del chequeo. El cierre reúne en una sola recapitulación todas las
-comidas concretas y el agua que consumieron los Actores durante la viñeta; no
-interrumpe el relato en cada frontera de 24 horas y nunca menciona unidades
-alimenticias.
-
-Si un resultado de 17 o más concede conocimiento, puede incorporarse este
-fragmento adaptado al medio:
+### B. Ejemplos comunes de monturas
 
@@ -1446,6 +1046,2 @@
 > todos los integrantes del Grupo obtienen la Relación `Conoce` con el Lugar.
-
-Los incidentes compartidos que no requieren un relato propio de cada especie
-pueden narrarse con estas formulaciones, adaptando `{montura}` a un caballo o
-un camello:
 
@@ -1464,33 +1060,4 @@
   integrantes del Grupo. {consecuencia_del_recuento_de_monturas}».
-
-Si todavía quedan monturas de esa especie para todos, la consecuencia explica
-que una montura de reserva ocupa el lugar de la perdida y que el Grupo conserva
-el medio y la velocidad previstos. Si ya no alcanzan y la muerte ocurrió en la
-mitad del origen, explica que uno de los integrantes recorre a pie la mitad del
-destino, cuánto toma esa mitad con la velocidad de `Andar` y que el segundo
-intento montado conserva su resultado sin repetirse. Si la muerte ocurrió en la
-mitad del destino, explica que la pérdida no altera el tiempo ya recorrido. En
-ambos casos de insuficiencia, también anuncia que antes de la conexión siguiente
-`Viajar` volverá a elegir entre caballos, camellos y marcha a pie mediante la
-prioridad general.
 
-<a id="via-des-cabalgar"></a>
-#### 9.3.2. Cabalgar
-
-> **VIA-DES-CAB — Cabalgar**  
-> Tipo: viñeta de resolución
-
-La tarjeta aplica la dificultad 6 y la tabla establecidas en las reglas comunes
-de los desplazamientos montados.
-
-La tarjeta usa la duración universal de 4 días entre dos Lugares normales: 2
-días por cada mitad normal antes de aplicar su tamaño. En una mitad `Montañoso`,
-`Pantanoso` o `Boscoso`, sustituye esa referencia por la mitad de los 7 días de
-`Andar` y después aplica el factor de tamaño. Si una mitad es a la vez
-`Desierto` y `Vasto`, esta tarjeta no puede recorrerla. Estas restricciones no
-cambian la acción particular ni hacen que se use la tabla de `VIA-DES-AND`.
-
-Cuando ninguna restricción territorial reemplaza la velocidad y no ocurre un
-incidente, el éxito debe transmitir la rapidez y amplitud de una cabalgata sin
-suponer un paisaje concreto. Por ejemplo:
+### C. Ejemplos de cabalgata y crisis del caballo
 
@@ -1501,4 +1068,2 @@
 > ganado se vuelve visible a cada momento.
-
-Al entrar en el destino, el relato continúa con la segunda mitad:
 
@@ -1508,19 +1073,2 @@
 
-Cuando el terreno obliga a usar la velocidad de `Andar`, el relato reemplaza
-la sensación de rapidez por la causa concreta de la demora y muestra que el
-Grupo continúa montado:
-
-- **`Montañoso`:** las pendientes obligan a las monturas a medir cada apoyo.
-- **`Pantanoso`:** el suelo incierto obliga a buscar apoyos firmes y sostener un
-  paso prudente.
-- **`Boscoso`:** el paso estrecho entre obstáculos impide aprovechar la
-  velocidad de los caballos.
-
-En cualquiera de esos casos, la viñeta informa cuánto toma la mitad afectada y
-cuánto habría tomado sin la restricción.
-
-En **Crisis de la montura**, un animal salvaje ataca al caballo. El ataque no
-lo mata de inmediato, pero lo deja herido y activa la espera y el chequeo de
-recuperación de la regla común. Puede narrarse así:
-
 > Una fiera se abalanza sobre {caballo}. Logran ahuyentarla, pero la montura
@@ -1529,4 +1077,2 @@
 
-Si el chequeo alcanza dificultad 8:
-
 > Al cabo de los 2 días, {caballo} vuelve a sostenerse y responde a la guía. El
@@ -1534,4 +1080,2 @@
 > y el Grupo puede continuar a caballo.
-
-Si el chequeo no alcanza dificultad 8:
 
@@ -1542,26 +1086,4 @@
 
-<a id="via-des-camello"></a>
-#### 9.3.3. Cabalgar en camello
-
-> **VIA-DES-CAM — Cabalgar en camello**  
-> Tipo: viñeta de resolución
-
-La tarjeta usa la duración universal de 5 días entre dos Lugares normales: 2,5
-días por cada mitad normal antes de aplicar su tamaño. En una mitad `Montañoso`,
-`Pantanoso` o `Boscoso`, sustituye esa referencia por la mitad de los 7 días de
-`Andar` y después aplica el factor de tamaño. A diferencia de
-`VIA-DES-CAB`, esta tarjeta sí puede atravesar una mitad que sea a la vez
-`Desierto` y `Vasto`, según las reglas de la sección 4.1.
-
-La tarjeta aplica la dificultad 6 y la tabla establecidas en las reglas comunes
-de los desplazamientos montados.
-
-Cuando `Montañoso`, `Pantanoso` o `Boscoso` obliga a usar la velocidad de
-`Andar`, la tarjeta emplea las mismas causas narrativas de `VIA-DES-CAB`,
-adaptadas a los camellos, e informa cuánto toma la mitad afectada y cuánto habría
-tomado sin la restricción.
+### D. Ejemplos de camello y picadura
 
-Si una mitad es `Desierto` y `Vasto`, la viñeta hace visible la ventaja propia
-del medio:
-
 > La extensión de {lugar} impediría atravesarlo a caballo. Los camellos pueden
@@ -1569,6 +1091,2 @@
 > {duración_mitad}.
-
-Cuando no ocurre un incidente, el éxito combina el avance más rápido que a pie
-con el ritmo sostenido y el balance propio de la montura, sin presuponer que el
-Grupo atraviesa un desierto ni ninguna otra clase de Lugar. Por ejemplo:
 
@@ -1579,4 +1097,2 @@
 > a espaldas del Grupo.
-
-Al internarse en el destino:
 
@@ -1587,6 +1103,2 @@
 
-En **Crisis de la montura**, un animal venenoso que el Grupo no alcanza a ver
-pica al camello. La picadura activa la espera y el chequeo de recuperación de
-la regla común. Puede narrarse así:
-
 > Una sacudida detiene a {camello}. Encontrás en su cuerpo la marca de una
@@ -1594,4 +1106,2 @@
 > con el Grupo durante 2 días completos para saber si podrá recuperarse.
-
-Si el chequeo alcanza dificultad 8:
 
@@ -1600,4 +1110,2 @@
 > dificultad 8, y el Grupo puede continuar en camello.
-
-Si el chequeo no alcanza dificultad 8:
 
@@ -1607,145 +1115,4 @@
 > integrantes del Grupo. {consecuencia_del_recuento_de_monturas}.
-
-<a id="via-des-navegar"></a>
-#### 9.3.4. Navegar en barco
-
-> **VIA-DES-NAV — Navegar en barco**  
-> Tipo: viñeta de resolución
-
-Esta tarjeta se presenta cuando cualquiera de los dos Lugares de la conexión es
-`Acuático`, según la sección 3. El barco está disponible automáticamente para
-esa conexión. La tarjeta usa la duración universal de 2 días entre dos Lugares
-normales y calcula las dos mitades según el tamaño de sus Lugares.
-
-La tarjeta realiza dos chequeos de la acción particular `NavegarEnBarco`. El
-primero se asocia con la mitad del Lugar de origen y el segundo con la mitad del
-Lugar de destino. En cada chequeo, el responsable tira un dado de veinte caras
-contra dificultad 14. Un 14 alcanza la dificultad y cuenta como éxito.
-
-`NavegarEnBarco` resuelve cada chequeo y aplica sus modificadores reutilizables
-según la sección 8, incluido el +4 de `Navegante`. La dificultad 14 y la
-comparación conjunta de ambos chequeos usan el resultado ajustado. La tarjeta
-elige al Actor más apto según la regla común de la sección 8 y ese responsable
-realiza ambos chequeos.
-
-Cada chequeo cuya cara natural sea 18 o más se evalúa de manera independiente.
-Todos los Actores que integran el Grupo obtienen una Relación `Conoce` con el
-Lugar asociado a ese dado. Los modificadores no ayudan a alcanzar este umbral.
-Si ambas caras naturales alcanzan 18, todos obtienen la Relación con ambos
-Lugares.
-
-Después de resolver ambos chequeos, la tarjeta determina si ocurrió un solo
-incidente:
-
-1. Si al menos uno de los dos resultados es 14 o más, la conexión completa se
-   considera una buena navegación. No se consulta la tabla de incidentes, aunque
-   el otro resultado sea menor que 14.
-2. Si ambos resultados son menores que 14, la navegación falla y se realiza una
-   tercera tirada de un dado de veinte caras para escoger un único incidente.
-   Esta **tirada de incidente** no es otro chequeo de `NavegarEnBarco`, no tiene
-   dificultad y no recibe el +4 de `Navegante` ni ningún otro modificador de los
-   Actores.
-3. El resultado natural de la tirada de incidente se consulta en la tabla de
-   incidentes una sola vez. Nunca ocurren dos incidentes en una misma conexión.
 
-El umbral de `Conoce` se evalúa aunque la regla conjunta haya descartado un
-incidente.
-
-La dificultad representa mucho más que sostener un rumbo. Incluye interpretar
-el viento y el clima, gobernar las velas, responder al movimiento de las olas y
-conservar una dirección útil. Por eso un navegante capaz puede convertir el
-trayecto en una experiencia placentera, mientras que confiarlo a alguien sin
-experiencia es arriesgado aunque la mayoría de los fallos solo añada tiempo.
-
-##### Viento estacional
-
-Al comenzar la viñeta, antes de calcular la duración real, la tarjeta consulta
-el mes vigente y fija la dirección hacia la que sopla el viento:
-
-| Mes al comenzar la viñeta | Dirección del viento |
-|---|---|
-| Enero, febrero o marzo | Este |
-| Junio o julio | Oeste |
-| Septiembre u octubre | Norte |
-| Abril, mayo, agosto, noviembre o diciembre | Una dirección al azar para esta viñeta. |
-
-Cuando corresponde una dirección al azar, se sortea con la misma probabilidad
-entre los ocho puntos de la rosa de los vientos: norte, noreste, este, sureste,
-sur, suroeste, oeste y noroeste. El resultado se conserva durante toda esa
-viñeta, aunque el calendario cambie de mes antes de que termine.
-
-La tarjeta compara la dirección del barco con la dirección hacia la que sopla
-el viento:
-
-| Relación con el viento | Modificación de la duración |
-|---|---:|
-| A favor | Resta 24 horas. |
-| Perpendicular | No modifica la duración. |
-| En contra | Añade 48 horas. |
-
-Las direcciones del barco y del viento se expresan mediante los mismos ocho
-puntos de la rosa. Ir **a favor** significa que entre ambas hay como máximo 45
-grados. Ir **en contra** significa que el rumbo del barco está como máximo a 45
-grados de la dirección exactamente opuesta al viento. Una diferencia de 90
-grados es **perpendicular**. Con direcciones separadas en intervalos de 45
-grados, estas tres categorías cubren todos los casos.
-
-El modificador corresponde a la conexión completa y debe hacerse visible en la
-narración. El viento no forma parte del tiempo previsto durante la
-planificación: solo se consulta y modifica la duración cuando se ejecuta esta
-viñeta. Los factores de tamaño determinan primero la duración de las dos
-mitades de la conexión; después, el viento y los demás modificadores expresados
-en horas se suman o se restan sobre ese total. Como todos esos modificadores
-son aditivos, el orden entre sus sumas y restas no altera el resultado. La
-reducción se aplica completa aunque la duración real quede por debajo del
-tiempo previsto. Con los tamaños y duraciones actuales, una conexión entre dos
-Lugares `Pequeño` dura 24 horas antes del viento y puede quedar en 0 horas al
-recibir viento favorable. Ese resultado es válido.
-
-El tiempo previsto no limita, corrige ni reemplaza esta duración real. Una vez
-que sirvió para estimar las provisiones iniciales, el plan no tiene
-preponderancia sobre la ejecución del viaje: el calendario y el consumo usan
-exclusivamente el tiempo que produzcan las viñetas al resolverse.
-
-##### Resultados desfavorables
-
-La tabla de incidentes solo se consulta cuando ambos resultados ajustados son
-menores que 14. Su resultado es independiente de los chequeos que causaron el
-fallo: una capacidad para navegar reduce la frecuencia con la que se consulta
-la tabla, pero nunca cambia qué incidente escoge una vez realizada la tirada.
-
-| Tirada de incidente | Incidente | Efecto |
-|---:|---|---|
-| 1–6 | **Olas y desvíos.** | Añade 12 horas. |
-| 7–12 | **Mal tiempo y lluvia.** | Añade 24 horas. |
-| 13–16 | **Falta de viento.** | El barco queda inmóvil en el mar y añade 48 horas. |
-| 17–18 | **Tormenta fuerte.** | El barco naufraga y el viaje termina según el Lugar donde ocurra el incidente. |
-| 19–20, con varios Actores | **Actor por la borda.** | Un Actor distinto del Actor del jugador sale del Grupo y se lo coloca en un Lugar `Costero` elegido al azar. |
-| 19–20, con un solo Actor | **Tormenta fuerte en lugar de una caída.** | No puede caer nadie por la borda; se resuelve **Tormenta fuerte** en su lugar. |
-
-El Actor del jugador nunca es elegible para caer por la borda. El Actor
-afectado se escoge al azar entre los demás integrantes según la regla común.
-Su destino se sortea entre todos los Lugares que el diseñador haya marcado con
-la Característica `Costero`. Un mundo que permita navegar pero no contenga
-ningún Lugar `Costero` es un error de diseño y estas reglas no intentan
-recuperarlo. La caída por la borda no añade tiempo al desplazamiento.
-
-En **Tormenta fuerte**, si el Lugar del incidente es `Costero`, el barco choca
-contra esa costa, el Grupo queda en ese Lugar y `Viajar` termina sin presentar
-`VIA-LLE-01`. Si no es `Costero`, se sortea de manera independiente un Lugar
-`Costero` para cada Actor y cada uno queda en el suyo. Todos los integrantes
-salvo el Actor del jugador salen del Grupo; el Actor del jugador permanece como
-su único integrante y el Lugar que recibió pasa a ser el Lugar actual del
-Grupo. Después `Viajar` termina sin presentar `VIA-LLE-01`.
-
-La tormenta no añade tiempo por sí misma. Antes de terminar el viaje se conserva
-únicamente el tiempo que ya corresponda al desplazamiento resuelto por las
-demás reglas de la tarjeta. El naufragio no añade por analogía muertes, pérdida
-de provisiones ni otros daños que estas reglas no hayan establecido.
-
-##### Presentación
-
-Cuando la navegación tiene éxito, la prosa adopta un tono romántico y transmite
-velocidad, firmeza y amplitud sin exigir un paisaje costero concreto. Por
-ejemplo:
+### E. Ejemplos de navegación, viento e incidentes
 
@@ -1759,4 +1126,2 @@
 
-La relación con el viento añade uno de estos fragmentos:
-
 - **A favor:** «El viento empuja hacia el rumbo del Grupo, hincha las velas y
@@ -1768,6 +1133,2 @@
   resistencia añade 2 días».
-
-Cuando surge una dificultad, el texto adopta un tono serio y sombrío, pero
-presenta al Grupo abriéndose paso en vez de describir la situación con
-desesperanza:
 
@@ -1787,419 +1148,7 @@
   incidente termina, ya no forma parte del Grupo y queda en {lugar_costero}».
-
-Al cerrar la viñeta, la tarjeta informa ambos resultados y si `Navegante` aportó
-el +4. Si ambos fallaron, también informa la tirada de incidente y la
-consecuencia escogida. Después desglosa la duración producida por la base, el
-tamaño de los Lugares, el viento y cualquier demora, avanza el calendario y
-resume las comidas concretas y el agua consumidas durante todo el
-desplazamiento, sin mencionar unidades alimenticias.
 
-<a id="via-apr-01"></a>
-### 9.4. Aprovisionarse antes de partir
-
-> **VIA-APR-01 — Aprovisionarse antes de partir**  
-> Tipo: viñeta de resolución
-
-Esta tarjeta ocurre antes de iniciar el recorrido y no ofrece decisiones. Elige
-al responsable de `CalcularProvisiones` mediante la regla del Actor más apto de
-la sección 8. Ese único chequeo determina la cantidad final de alimento y agua
-que obtiene el Grupo. Los animales de carga no tienen un responsable ni una
-acción particular; se incorporan después de resolver cualquier decisión sobre
-la sobrecarga.
-
-Las provisiones comprenden las reservas temporales de alimento y agua de la
-sección 4.3, además de otros elementos básicos que permanecen resumidos. Las
-comidas y las botas de agua no ingresan al inventario general, pero `Viajar`
-conserva sus cantidades concretas mientras dura la acción.
-
-La escena también establece las cantidades iniciales de animales de carga,
-caballos y camellos que `Viajar` conservará en sus contadores temporales. La
-tarjeta proporciona una montura por Actor de cada especie que la ruta adoptada
-requiera y todos los animales de carga que determine la configuración aceptada.
-Las monturas y los animales de carga no son intercambiables. El barco no se
-busca: la tarjeta simplemente deja establecida su disponibilidad automática
-para toda conexión acuática que lo requiera.
-
-Antes del chequeo, `Viajar` calcula en secreto la **necesidad base** a partir
-del Grupo, la jornada de aprovisionamiento, la ruta adoptada y sus duraciones de
-referencia. Para `A` Actores y `T` horas previstas de recorrido, primero calcula:
-
-```text
-períodos previstos = parte entera((24 horas + T) ÷ 24 horas)
-necesidad alimenticia = A × 2 × períodos previstos unidades alimenticias
-necesidad de agua = A × 0,5 × períodos previstos litros
-```
-
-El primer período cubre el día que consume esta misma tarjeta; los restantes
-cubren el recorrido previsto. Estas cifras no garantizan lo que se consumirá:
-los desplazamientos reales pueden tomar más o menos tiempo y completar una
-cantidad distinta de períodos de 24 horas.
-
-La ruta y sus medios ya fueron seleccionados antes de aplicar esta fórmula.
-Por lo tanto, la necesidad base surge mecánicamente de la cantidad prevista de
-días, multiplicada por la cantidad de Actores y por el consumo diario de cada
-Actor. La misma operación se realiza por separado para el alimento y el agua.
-
-Esta tarjeta invoca una sola acción particular, `CalcularProvisiones`. Después,
-cuando las reglas ya fijaron la cantidad total de animales de carga, el Grupo
-consigue automáticamente esa cantidad.
-
-#### Calcular y reunir las provisiones
-
-La tarjeta invoca `CalcularProvisiones` con dificultad 10. Su resultado ajustado
-se aplica a la necesidad base de alimento y a la necesidad base de agua por
-separado, y produce directamente las cantidades finales que consigue el Grupo:
-
-| Resultado ajustado | Efecto |
-|---|---|
-| Menor que 10 | El chequeo fracasa. Consigue el 90 % de la necesidad base de alimento y el 90 % de la necesidad base de agua. |
-| De 10 a 15 | El chequeo tiene éxito. Consigue el 100 % de ambas necesidades base. |
-| 16 o más | El chequeo obtiene un éxito superior. Consigue el 110 % de ambas necesidades base. |
-
-La misma franja se aplica a ambos recursos; no se realiza una tirada distinta
-para cada uno. Si `F` es la necesidad alimenticia en unidades y `W` es la
-necesidad de agua en litros, el multiplicador `m` es 0,9, 1 o 1,1 según la
-tabla, y las cantidades finales son:
-
-```text
-unidades alimenticias finales = F × m
-litros de agua finales = W × m
-```
-
-Primero se resuelve el chequeo y se calculan esos dos números. Después, para
-materializar el alimento, `Viajar` inicia un lote vacío y selecciona al azar una
-comida completa por vez entre los cuatro tipos de la sección 4.3. Continúa hasta
-que la suma alcance o supere las unidades alimenticias finales. Para
-materializar el agua, divide los litros finales por 3 y redondea hacia arriba;
-esa es la cantidad de botas llenas que incorpora. Los excedentes indivisibles
-de la última comida y de la última bota permanecen en las reservas.
-
-Un resultado de 16 recibe siempre exactamente el mismo 10 % adicional que un
-resultado mayor. Antes de aplicar la tabla, cualquier resultado ajustado mayor
-que 20 se interpreta como 20, de acuerdo con la regla general de las acciones
-particulares.
-
-Una vez fijada la cantidad obtenida mediante estas reglas, `Viajar` la compara
-con el límite de carga de la sección 4.2. Así determina `N`, evalúa la
-alternativa con `N − 1` animales y decide si corresponde presentar
-[`VIA-APR-05`](#via-apr-05).
-
-#### Incorporar animales de carga
-
-La cantidad queda fijada por la regla general o por la elección realizada en
-[`VIA-APR-05`](#via-apr-05). El Grupo consigue automáticamente la cantidad
-completa. No realiza una acción particular, no tira un dado, no elige un
-responsable y no puede fracasar. Obtenerlos tampoco añade tiempo aparte de las
-24 horas que ya dura `VIA-APR-01`.
-
-Los animales transportan provisiones según la sección 4.2: reducen la carga
-soportada por los integrantes, pero su velocidad puede limitar el paso del
-Grupo. No consumen sus reservas.
-
-#### Presentación y continuación
-
-La tarjeta presenta el único intento de aprovisionamiento en una sola escena.
-Identifica al responsable, informa la cantidad final de cada comida y de agua
-conseguida y señala las Características que favorecieron el chequeo. Las
-unidades alimenticias permanecen ocultas. El procedimiento posterior para
-reunir los animales se narra después de que quede fijada la cantidad necesaria,
-sin presentar un chequeo ni un responsable.
-
-Si el Grupo recibió una propuesta de ruta, el texto puede relacionar las
-provisiones con sus estimaciones. Si partió sin un plan, el jugador afronta el
-aprovisionamiento sin conocer la duración ni el alimento que realmente
-necesitará ni el agua que consumirá. Esa incertidumbre es una dificultad
-deliberada del viaje.
-
-Después de fijar las provisiones, `Viajar` resuelve la carga e incorpora
-automáticamente los animales necesarios según las secciones 4.2 y 7. Si
-`CalcularProvisiones` obtuvo menos de 10, la siguiente tarjeta es
-[`VIA-APR-03`](#via-apr-03). En cualquier otro caso comienza el recorrido.
-
-Resolver `VIA-APR-01` hace avanzar siempre 24 horas. Al final de la escena,
-después de formar las reservas, ese período completa una jornada de consumo y
-se descuenta una ración de alimento y agua por Actor. La velocidad base de los
-animales de carga es la establecida en la sección 4.2.
-
-<a id="via-apr-05"></a>
-### 9.5. Decidir ante una sobrecarga moderada
-
-> **VIA-APR-05 — Decidir ante una sobrecarga moderada**  
-> Tipo: viñeta de decisión
-
-Esta tarjeta ocurre cuando `Viajar` determina que el Grupo podría partir con un
-animal menos que la cantidad `N` necesaria para eliminar toda demora por exceso
-de carga, siempre que acepte una sobrecarga moderada.
-
-La demora mostrada incluye el peso del suplemento cuando corresponde. Esta
-tarjeta solo se presenta si la alternativa con `N − 1` animales resulta
-admisible según la sección 4.2; de lo contrario, los `N` animales son
-obligatorios.
-
-La demora y el suplemento mostrados son los valores finales con los que se
-cerró el ciclo de sobrecarga. Si al sumar la demora al tiempo previsto no se
-atraviesa una nueva frontera de 24 horas, el suplemento es nulo.
-
-La viñeta identifica a los Actores que participaron del aprovisionamiento y
-explica las dos formas de distribuir la carga que el Grupo está considerando.
-Debe informar:
-
-- los `N` animales que eliminarían la demora por carga;
-- los `N − 1` animales de la alternativa, o que esa alternativa no requiere
-  animales cuando el resultado es cero;
-- cuánto peso soportarían los Actores, cuál es su límite conjunto y cuántos
-  kilogramos lo superarían;
-- cuántas horas añadiría la sobrecarga;
-- y, si corresponde, cuánta carne seca y agua se incorporaría para sostener la
-  demora.
-
-Si el Grupo cuenta con un plan de ruta, la tarjeta puede sumar la demora a la
-previsión conocida. Si partió sin un plan, informa las horas que añadiría la
-carga, pero no revela la duración base ni la duración total oculta del viaje.
+### F. Ejemplos de llegada
 
-El jugador elige entre:
-
-- **Aceptar la demora:** `Viajar` incorpora sin chequeo el suplemento cuando
-  corresponda, fija una sola vez la demora calculada y establece `N − 1` como
-  la cantidad total de animales que se conseguirá.
-- **Evitar la demora por carga:** el suplemento no se incorpora, la demora
-  cuadrática es cero y se establece `N` como la cantidad total de animales que
-  se conseguirá.
-
-Después de la elección, el Grupo consigue automáticamente la cantidad total
-elegida. La viñeta no presenta una dificultad ni un riesgo de fracaso.
-
-La tarjeta nunca muestra unidades alimenticias. Cuando existe un suplemento,
-lo presenta mediante piezas enteras de carne seca, botas llenas y sus litros
-totales de agua.
-
-<a id="via-apr-03"></a>
-### 9.6. Reconsiderar un aprovisionamiento incompleto
-
-> **VIA-APR-03 — Reconsiderar un aprovisionamiento incompleto**  
-> Tipo: viñeta de decisión
-
-Esta tarjeta ocurre cuando el Grupo no consigue los recursos necesarios en una
-de estas dos situaciones:
-
-- durante `VIA-APR-01`, el responsable de `CalcularProvisiones` no alcanza su
-  dificultad y el Grupo reúne el 90 % de la necesidad base;
-- durante `VIA-APR-02`, los intentos del Grupo reúnen alimento y agua para al
-  menos un día, pero no lo necesario para todo el recorrido restante.
-
-Antes de partir, el texto explica que el Grupo obtuvo menos de lo previsto y
-no está seguro de contar con lo necesario para el viaje. No revela la necesidad
-base ni confirma cuánto durarán las reservas. Durante el recorrido, identifica
-qué recurso no se consiguió y recuerda que el Grupo se encuentra en el Lugar
-actual sin una solución suficiente para continuar con seguridad.
-
-Cuando la tarjeta procede de `VIA-APR-01`, la advertencia responde al fracaso
-del chequeo y a haber obtenido el 90 % calculado, sin revelar al jugador la
-necesidad base oculta. Cuando procede de `VIA-APR-02`, responde a la comparación
-con los recursos necesarios para el resto del recorrido.
-
-El jugador elige entre:
-
-- **Viajar de todas formas:** el Grupo conserva cuanto haya conseguido. Si aún
-  no partió, comienza el recorrido; si ya estaba viajando, permanece en el
-  Lugar actual y continúa hacia la conexión siguiente. El suplemento de una
-  posible sobrecarga solo cubre las nuevas jornadas de consumo causadas por la
-  demora de esa carga y no corrige por sí mismo un aprovisionamiento
-  incompleto.
-- **Cancelar el viaje:** `Viajar` termina en el Lugar actual sin presentar
-  `VIA-LLE-01`.
-
-Si el jugador cancela antes de partir, el Grupo devuelve o vende las
-provisiones y los animales que hubiera obtenido. Si cancela durante el
-recorrido, las reservas temporales simplemente se descartan al terminar
-`Viajar`. Ninguna de las dos resoluciones requiere otra viñeta ni un chequeo y,
-como estas reglas no simulan precios ni dinero, no produce una consecuencia
-económica.
-
-Esta tarjeta no hace pasar tiempo adicional. Conserva todo el tiempo y consumo
-que ya hayan producido `VIA-APR-01`, `VIA-APR-02` u otras escenas anteriores.
-
-<a id="via-apr-02"></a>
-### 9.7. Conseguir alimento y agua durante el viaje
-
-> **VIA-APR-02 — Conseguir alimento y agua durante el viaje**  
-> Tipo: viñeta de resolución
-
-Esta tarjeta ocurre en cualquiera de estas situaciones, siempre que el Grupo
-todavía no haya alcanzado el destino final:
-
-- después de completar una conexión, la reserva alimenticia, la reserva de agua
-  o ambas quedan en cero;
-- después de una pérdida de capacidad de carga, la previsión especial de la
-  sección 4.2 muestra que el alimento, el agua o ambos no alcanzarán para
-  completar el resto del viaje con la composición y el peso actuales.
-
-En el primer caso, el recurso puede haberse agotado durante el desplazamiento o
-exactamente al terminarlo. El Grupo ya alcanzó el Lugar de destino de esa
-conexión y no puede iniciar la siguiente antes de resolver esta tarjeta. Si un
-recurso faltó durante el tramo, la viñeta anterior ya narró el esfuerzo con el
-que el Grupo consiguió completarlo; si llegó exactamente a cero al pagar la
-última ración, solo informa que se agotó al llegar.
-
-La tarjeta representa cómo el Grupo se las ingenia para conseguir los recursos
-que faltan en el Lugar actual y bajo sus circunstancias actuales. Una misma
-presentación debe poder tratar solo alimento, solo agua o ambos a la vez. No
-ofrece una decisión al jugador.
-
-Para esta tarjeta, los **recursos necesarios** son los que cubrirían todo el
-recorrido restante previsto desde el Lugar actual. Con `h` horas acumuladas
-desde la última ración, `A` Actores y la duración restante conocida en ese
-momento, calcula:
-
-```text
-períodos necesarios = parte entera(
-  (h + duración restante conocida) ÷ 24 horas
-)
-alimento necesario = 2 × A × períodos necesarios unidades alimenticias
-agua necesaria = 0,5 × A × períodos necesarios litros
-```
-
-La duración restante usa la ruta, los medios, los estados y la demora por carga
-que ya sean conocidos, pero no anticipa chequeos ni incidentes futuros. Las 6
-horas que dura esta búsqueda se incluyen en la suma antes de calcular los
-períodos necesarios.
-
-La búsqueda dura 6 horas. Ese tiempo avanza el calendario, se añade al contador
-de horas desde la última ración y puede completar uno o más períodos normales
-de consumo.
-
-Todos los Actores que integran el Grupo realizan una vez la acción particular
-`Forajear`, cada uno con dificultad 14. No se elige un único responsable. Por
-cada Actor cuyo resultado ajustado alcance 14, la tarjeta obtiene de forma
-independiente:
-
-- si hace falta alimento, una cantidad entera elegida al azar entre 2 y 10
-  piezas de carne seca, ambos extremos incluidos;
-- si hace falta agua, una cantidad elegida al azar entre 2 y 10 litros, ambos
-  extremos incluidos.
-
-Un fracaso no añade alimento ni agua. Los resultados de todos los éxitos se
-suman. La comida obtenida en esta búsqueda es carne seca; cuando otra regla
-requiera formar una combinación de alimentos para alcanzar una cantidad de
-unidades alimenticias, se aplica en cambio la selección aleatoria entre todos
-los tipos de comida de la sección 4.3. Los litros de agua encontrados por los
-éxitos se suman primero y después se incorporan mediante la cantidad de botas
-llenas necesaria para alcanzar o superar ese total.
-
-La tarjeta debe explicar la necesidad, las oportunidades que ofrece el entorno,
-quién intervino, qué intentó cada Actor y cuánto alimento y agua consiguió el
-Grupo. Después de incorporar lo obtenido y resolver las 6 horas transcurridas,
-`Viajar` realiza estas comprobaciones en orden:
-
-1. Calcula una ración completa para un día como 2 unidades alimenticias y
-   0,5 litros de agua por cada Actor actual. Si la reserva de alimento o la de
-   agua no alcanza esa cantidad, el viaje se cancela automáticamente en el
-   Lugar actual, sin ofrecer la decisión de `VIA-APR-03` y sin presentar
-   `VIA-LLE-01`.
-2. Si ambas reservas cubren al menos un día pero no cubren los recursos
-   necesarios para todo el recorrido restante, la continuación es
-   `VIA-APR-03`.
-3. Si el viaje continúa, tanto directamente como por decisión del jugador en
-   `VIA-APR-03`, vuelve a comprobar el peso y la capacidad de carga con las
-   provisiones que dejó esta búsqueda. Esta comprobación se realiza aunque no
-   se haya perdido antes un Actor ni un animal de carga. Si corresponde, aplica
-   las reglas de sobrecarga de la sección 4.2 y presenta `VIA-APR-06`; de lo
-   contrario, continúa desde el mismo Lugar con la conexión siguiente.
-
-<a id="via-apr-06"></a>
-### 9.8. Afrontar una sobrecarga imprevista
-
-> **VIA-APR-06 — Afrontar una sobrecarga imprevista**  
-> Tipo: viñeta de decisión
-
-Esta tarjeta se presenta cuando una comprobación durante el viaje determina
-que la carga actual es intransportable. Esa comprobación puede deberse a que
-una consecuencia quitó del Grupo a un Actor o a un animal que aportaba
-capacidad de carga, o a que `VIA-APR-02` añadió nuevas provisiones. Se resuelve
-antes del siguiente desplazamiento y no modifica retroactivamente la conexión
-ya completada.
-
-Si la comprobación nació de una pérdida de capacidad y exige `VIA-APR-02`, esa
-tarjeta se resuelve primero. Si nació de las provisiones obtenidas en
-`VIA-APR-02`, no se repite la búsqueda. Si el viaje continúa, `Viajar` vuelve a
-calcular el peso, la duración restante y las necesidades con las provisiones
-resultantes. `VIA-APR-06` solo se presenta si todavía hay un exceso de carga y
-el alimento, el agua o ambos no cubren la nueva duración restante.
-
-La pérdida de una montura no activa esta tarjeta: las monturas y los animales
-de carga no son intercambiables. Su muerte se resuelve exclusivamente mediante
-las reglas de los desplazamientos montados. La pérdida de un animal de carga sí
-puede activarla porque reduce la capacidad disponible.
-
-La viñeta explica qué pérdida redujo la capacidad, cuántos Actores y
-animales capaces de transportar carga quedan y qué peso de provisiones ya no
-puede trasladarse con normalidad. También muestra el tiempo calculado para el
-resto del viaje, la demora causada por el exceso actual y qué reserva o reservas
-resultan insuficientes. De esta manera, el jugador conoce la necesidad material
-antes de decidir si abandona alguna provisión.
-
-Antes de presentar la decisión, `Viajar` calcula el lote mínimo de provisiones
-que debe abandonarse para que desaparezca todo el exceso `x`: el peso restante
-debe quedar cubierto por la capacidad de los animales de carga que permanecen
-más 10 kg por cada Actor actual. Como las comidas y las botas son indivisibles,
-el lote se redondea hacia arriba hasta que quitarlo cumpla ese límite.
-
-Al formar el lote, `Viajar` intenta repartir por igual el peso descartado entre
-alimento y agua. Selecciona objetos concretos de ambas reservas y procura que el
-peso retirado de cada una sea el mismo; las cantidades indivisibles pueden
-impedir una igualdad exacta. Si una reserva no contiene peso suficiente para su
-mitad, completa con la otra todo lo necesario para eliminar el exceso. La
-viñeta muestra de antemano cada comida y cada bota que integra el lote.
-
-El jugador elige entre:
-
-- **Abandonar provisiones:** acepta el lote ya calculado y lo quita
-  definitivamente de las reservas temporales. No elige cantidades distintas.
-  `Viajar` verifica que el peso restante ya no produzca demora por sobrecarga.
-  La elección no se invalida porque las provisiones restantes puedan resultar
-  insuficientes más adelante: si una reserva llega a cero tras otro
-  desplazamiento, se aplicará normalmente `VIA-APR-02`.
-- **Cancelar el viaje:** `Viajar` termina en el Lugar actual y no presenta
-  `VIA-LLE-01`.
-
-Organizar y abandonar ese lote no hace pasar tiempo. Esta tarjeta no concede
-nuevos animales de carga ni otra ayuda.
-
-Si el abandono deja una reserva en cero, `VIA-APR-02` no se repite en esta
-misma secuencia: ya se resolvió antes de `VIA-APR-06`. El Grupo puede iniciar la
-conexión siguiente sin ese recurso, completa el desplazamiento según la regla
-general y vuelve a presentar `VIA-APR-02` al terminarlo si todavía quedan
-conexiones. Si esa conexión alcanza el destino final, se presenta la llegada.
-
-<a id="via-lle-01"></a>
-### 9.9. Llegar al destino
-
-> **VIA-LLE-01 — Llegar al destino**  
-> Tipo: viñeta de resolución
-
-Esta tarjeta se presenta inmediatamente después de que el Grupo completa el
-último desplazamiento y alcanza el Lugar de destino. Se resuelve después de
-aplicar el tiempo, el consumo y las consecuencias de ese desplazamiento. No
-realiza chequeos, no ofrece decisiones y no hace pasar tiempo adicional.
-
-La tarjeta cierra el viaje con una narración breve dentro de la ficción. Nombra
-el Lugar alcanzado y adapta el peso emocional del relato a la extensión de la
-ruta adoptada. Una ruta de 2, 3 o 4 Lugares, incluidos el origen y el destino,
-es un viaje corto y recibe un cierre ligero. Una ruta de 5 o más Lugares es un
-viaje largo y da mayor peso al cansancio, la distancia recorrida y la
-satisfacción de haber llegado.
-
-El cierre informa de manera concisa cuánto tiempo pasó y qué provisiones
-concretas sobraron. No recapitula cada Lugar, chequeo, consumo, incidente ni
-decisión del recorrido. Llegar no concede por sí mismo una Característica, una
-Relación ni otro estado especial. Si no queda alimento, agua o ambos, el texto
-identifica cada reserva agotada; si un recurso alcanzó exactamente hasta la
-última ración, dice que se terminó al llegar y no describe hambre ni sed. Como
-el Grupo ya alcanzó el destino, no presenta `VIA-APR-02`.
-
-Por ejemplo, para un viaje corto:
-
 > Al cabo de {tiempo_total}, alcanzás con el Grupo el corazón de {destino}. El
 > trayecto queda atrás. {estado_breve_de_las_provisiones}.
-
-Para un viaje largo:
 
@@ -2209,15 +1158,22 @@
 
-Después de presentar la tarjeta, `Viajar` termina y descarta sus cuentas
-temporales según la sección 4. El remanente se informa antes de descartarlo.
-
-## 10. Decisiones pendientes para continuar el diseño
-
-Después de incorporar las decisiones confirmadas, todavía hace falta decidir:
-
-- qué sucede cuando no existe una ruta válida, existe una sola o no puede
-  construirse una ruta larga dentro del intervalo exigido; y si una ruta puede
-  repetir un Lugar.
+### G. Explicaciones didácticas y justificaciones retiradas
 
-Estas decisiones deben incorporarse a la regla o tarjeta donde se usan. Esta
-sección solo las reúne mientras sigan pendientes y debe acortarse a medida que
-se resuelvan.
+- Con un mismo medio, todos los Lugares normales y sin otros modificadores,
+  ordenar rutas por cantidad de conexiones equivale a ordenarlas por días.
+  Los tamaños pueden invertir esa comparación: más Lugares pequeños pueden
+  tomar menos tiempo que menos Lugares vastos.
+- Ejemplos de duración: andar entre dos Lugares normales toma 3,5 + 3,5 = 7
+  días; de uno `Vasto` a uno `Pequeño`, 7 + 1,75 = 8,75 días. Cabalgar de un
+  Lugar normal `Boscoso` a uno normal sin terreno lento toma 3,5 + 2 = 5,5 días.
+- Ejemplo de carga: con `T = 168 horas` y 2 kg de exceso, la demora es
+  `168 × (2 ÷ 10)² = 6,72 horas`.
+- Ejemplo de suplemento: si `T = 30 horas`, 17 horas de demora no cruzan una
+  nueva frontera de consumo; 18 horas llegan a la hora 48 y exigen otra ración.
+- Ejemplo de acumulación: escenas de 20 y 6 horas completan una jornada y dejan
+  2 horas para la siguiente. Exactamente 24 horas también consumen.
+- La alimentación de los animales estaba justificada suponiendo que los
+  herbívoros encuentran comida y los camellos dependen de sus reservas de agua.
+  Se conserva la regla de que no consumen provisiones, sin desarrollar esa ficción.
+- Usar solo la referencia temporal de `Andar` tras la muerte de una montura
+  evita cambiar de acción particular dentro de una viñeta cuyos dos chequeos
+  montados ya fueron resueltos.
