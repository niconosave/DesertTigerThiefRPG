# Encuentros con espacio tridimensional y planes

| Campo | Valor |
|---|---|
| Estado | Base implementada y verificada técnicamente; revisión jugable pendiente; ampliaciones posteriores sin reglamento cerrado |
| Versión | 0.3 |
| Fecha | 2026-11-24 |
| Alcance | Base espacial, elementos, contexto, dirección genérica y cierre; ampliaciones posteriores |

## 1. Experiencia buscada

Atacar, Robar y Rescatar representan episodios completos con personas concretas.
El grupo persigue un propósito, utiliza sus capacidades y puede cambiar de
dirección. Las consecuencias quedan en la partida aunque fracase el objetivo inicial.

Los escenarios genéricos ofrecen posibilidades. Un paso estrecho permite
resistir, proteger una retirada o impedir el acceso a un tesoro; no obliga a
seguir siempre una secuencia. Los participantes, relaciones y acontecimientos
aportan significado histórico a esas posibilidades.

Las interacciones obedecen a simulaciones compartidas. Un comportamiento que
intenta derribar una puerta solicita acciones sujetas a sus reglas; no elimina
el obstáculo porque necesita pasar. Cada sistema tendrá reglas propias antes de
ofrecer sus posibilidades. Dibujar fuego o nombrar un material no simula sus efectos.

El jugador decide sobre el curso del episodio y observa su ejecución. No ordena
cada casilla o golpe. La narración cambia ante momentos significativos y deja
tiempo para leerla, mientras los movimientos y efectos muestran la ejecución.

Desarrolla el [diseño general](../diseno-general.md). El
[reglamento del primer episodio](./encuentros-y-atacar.md#17-reglamento-mínimo-implementado)
describe lo implementado. Esta nueva base no define código, archivos, interfaces
de scripting ni pasos de programación.

## 2. Categorías

| Categoría | Función |
|---|---|
| Acción general | Determina propósito inicial, participantes, oposición, disposición apropiada y plan inicial. |
| Encuentro | Conserva el episodio, sus cambios de propósito y los resultados. |
| Escenario | Dispone el espacio físico, los elementos y el contexto; no decide la victoria. |
| Plan | Evalúa la situación y coordina comportamientos para un propósito. Tiene prioridad. |
| Comportamiento | Atiende un cometido acotado; puede coordinar muchas acciones y participantes. |
| Acción particular | Resuelve un intento reglado, como desplazarse, atacar o abrir. |
| Sistema simulado | Mantiene propiedades y estados y resuelve las interacciones admitidas. |
| Suceso | Identifica un hecho confirmado, sus participantes y su resultado. |
| Evento narrativo | Introduce circunstancias particulares y sus consecuencias. La nueva dirección de planes desde eventos se añadirá al final de esta etapa. |

Un plan es una lógica de decisiones, no una secuencia invariable. Puede elegir
aproximarse, afrontar oposición, alcanzar el objetivo o retirarse. Un comportamiento
de barricar también puede ser complejo: buscar materiales, arrastrarlos y sostener
el acceso mientras otros trabajan. La diferencia es el cometido, no su longitud.

## 3. Primera etapa

La primera etapa traduce los encuentros existentes al nuevo modelo y conserva movimiento
simultáneo, contacto, reacción, golpes, heridas, auxilio y extracción. Todavía no
introduce el futuro combate dinámico.

Se preservan el propósito, el carácter y las consecuencias de cada encuentro,
sin exigir reproducir exactamente sus casillas o disposición anterior. Los
escenarios y acontecimientos se adaptan a la nueva base; no condicionan su diseño
a seguir funcionando sobre la representación anterior.

| Parte | Alcance |
|---|---|
| Espacio | Volumen discreto tridimensional, apoyo, ocupación y conexiones transitables; un caso pequeño con dos alturas conectadas. |
| Vista | Corte por altura con superposición de lo inferior y ocultación por superficies opacas. |
| Escenarios actuales | Se traducen al modelo nuevo conservando carácter y objetivos; su disposición puede cambiar y seguir siendo mayoritariamente plana y simple. |
| Contexto | Espacios, conexiones entre ellos y salidas al exterior del encuentro. |
| Elementos | Paredes, obstáculos y una puerta manual sencilla. Identidad estable y presencias probables. |
| Dirección | Traducción de la lógica actual a planes genéricos y comportamientos con roles y prioridades. |
| Interacción nueva | Abrir y cerrar una puerta, modificando realmente el acceso. |
| Cierre de una retirada incompleta | Resolver explícitamente a quienes no pudieron evacuarse, sin borrarlos ni transportarlos por defecto. |
| Narrativa | Adaptar los eventos existentes conservando su sentido y efectos; crear, componer o sustituir planes desde eventos queda para después. |

Generación modular adaptable, mapas fijos, incendios, derrumbes automáticos,
sigilo detallado y maniobras nuevas son ampliaciones. Tener volumen o materiales
no incorpora automáticamente escalada, tiro, calor, destrucción o caída.

## 4. Duración e identidad del escenario

Se genera al comenzar y permanece durante todo el encuentro. Volver a una
habitación, cambiar de plan, leer o cambiar la altura visible no vuelve a
generarlo. Una puerta abierta sigue abierta hasta que algo cambia su estado.

Un encuentro nuevo genera otro escenario, incluso en el mismo Lugar mundial.
No se conservan edificios o muebles entre encuentros. Los hechos del mundo sí
se respetan: un muerto no reaparece y un tesoro llevado por el grupo no vuelve
a generarse en su ubicación anterior.

Las habitaciones no se convierten en Lugares del mapa. Las posiciones locales
pertenecen a fichas y elementos. Descartar el escenario no elimina los Actores
ni los objetos particulares representados.

## 5. Volumen y movimiento

El terreno ocupa una grilla con ancho, profundidad y altura. Cada volumen puede
estar libre u ocupado; una superficie sólida puede ofrecer apoyo. Piso, pared y
techo conservan efectos físicos aunque el corte visual los oculte. Vacío no
equivale a terreno desconocido ni a agua.

El perfil físico inicial de los combatientes ocupa una casilla horizontal y dos
unidades verticales libres sobre el apoyo. Es la convención común para adaptar las
fichas actuales; los tamaños físicos distintos se incorporarán después. El carácter
dibujado no determina su cuerpo. No se fija equivalencia en metros ni se introducen
todavía huellas grandes o cuerpos articulados.

Puede haber superficies transitables superpuestas en planta, como arriba y debajo
de un puente. Sus apoyos, ocupantes y conexiones se distinguen. La cercanía
horizontal no garantiza poder alcanzar a alguien en otro piso.

Caminar exige conexión, apoyo, espacio libre y ocupación compatible. Se conservan
diagonales y prohibición de cortar esquinas bloqueadas. Cambiar de altura requiere
una conexión habilitada, como una escalera. El movimiento ordinario no sube muros
ni cruza vacíos. La primera conexión vertical se recorre mediante pasos sujetos
a las mismas oportunidades e interrupciones del movimiento restante.

La primera escalera conecta apoyos mediante pasos cardinales de una casilla
horizontal y hasta una unidad vertical. Requiere altura libre también durante
la transición, para no atravesar techos. No permite subir en cualquier columna
ni saltar de un piso a otro en una sola oportunidad.

El contacto cercano inicial se conserva entre posiciones vecinas de una misma
superficie sin obstáculos interpuestos. No concede golpes a través de pisos ni
alcance vertical adicional. Combatir entre alturas tendrá reglas propias.
Las intenciones siguen resolviéndose simultáneamente, sin turnos completos
individuales añadidos por esta ampliación.

La disposición inicial ofrece recorridos practicables con las capacidades de la
entrega; no exige una escalada inexistente. Cambios posteriores sí pueden cerrar
rutas y bloquear planes.

La geometría podrá modificarse, pero no habrá derrumbe automático al retirar una
pared. Ese resultado necesita reglas de apoyo y colapso posteriores. Los cambios
narrativos deben dejar una disposición coherente y resolver a sus ocupantes.

## 6. Corte visual y conocimiento

El jugador elige la altura máxima observada. Lo superior se omite; lo que queda
a esa altura o debajo puede dibujarse. Entre elementos proyectados sobre una
misma posición, el más alto visible tapa a los inferiores cuando es opaco.

Un piso oculta el sótano aunque este esté incluido en el corte. Un hueco permite
ver lo inferior no tapado por otra superficie. Elegir un corte más bajo elimina
el piso superior de la vista y permite inspeccionar el espacio inferior conocido.

El corte permite observar el nivel de los personajes ocultando lo superior, como
un techo. Las paredes y objetos incluidos siguen representados. La presentación
distingue la altura de una posición señalada cuando coincida con otras en planta.

Cambiar el corte no altera apoyo, colisiones, amenazas, rutas ni tiempo. Tampoco
revela información desconocida: seleccionar una altura no permite localizar
enemigos a través de paredes. Se conservan las reglas de información del encuentro
actual; percepción individual y sigilo detallados se diseñarán después.

La información inicial se organiza por espacios. Se conserva terreno descubierto,
pero los estados actuales de entidades y elementos solo se actualizan al observarlos.
Cada bando observa sus espacios ocupados y espacios vecinos ya descubiertos a
través de conexiones abiertas, sin propagación recursiva. Cruzar una conexión
conocida descubre el espacio de llegada. Esta aproximación por espacios no
pretende simular todavía visión y oído individuales.

Un volumen cortado visualmente conserva su estado completo. La presentación puede
resumir una ficha con un símbolo, pero el corte no la hiere, desactiva o desplaza.

## 7. Elementos definidos y probables

Terreno, elementos físicos e información contextual se distinguen. Una puerta
tiene identidad, posición, estado y propiedades, además de apariencia. Puede
relacionar espacios y modificar la posibilidad de cruzarlos.

Un mueble ordinario puede existir solo dentro del escenario. La copa mantiene la
identidad de un objeto mundial y se representa localmente sin duplicarlo. La
misma distinción se aplica a una ficha que representa un Actor.

Un elemento definido conserva su identidad y lo que ya se sabe de él. No cambia
de naturaleza al dejar de mirarlo. Sus propiedades solo habilitan interacciones
con reglas existentes: declarar hierro no incorpora una regla de fusión.

### Puerta inicial

La puerta es manual, abierta o cerrada. Un participante capaz, adyacente y con
contacto válido puede abrirla o cerrarla como acción particular, ocupando una
oportunidad de actuar. Abierta permite cruzar; cerrada bloquea. No se cierra sobre
volumen ocupado: se rechaza el intento sin expulsar o dañar al ocupante.

La puerta cerrada también impide contacto y observación a través de su abertura;
abierta despeja ese impedimento. Esto no borra recuerdos de lo ya descubierto ni
incorpora todavía una simulación completa de visión individual.

No hay cerradura, llave ni resistencia al daño en esta primera prueba. Abrirla
no modifica otras puertas ni depende de matar enemigos. Un plan que quiere
cruzarla considera su estado y ordena la interacción cuando corresponde.

### Elemento probable

Reserva una presencia poco determinada, con identidad, ubicación y límites
observables de ocupación, impedimentos y carácter general. Podrá concretarse
sin contradecir esos límites ni lo que ya se mostró al jugador.

Una presencia decorativa compatible puede resultar ser una cortina. Una mesa
identificada no se convierte retrospectivamente en cortina ni un obstáculo sólido
desaparece por conveniencia. La concreción conserva identidad; los cambios físicos
adicionales se declaran y resuelven.

La primera base admite esas presencias sin exigir acontecimientos nuevos que
las utilicen. Una cortina incendiada requiere las reglas de fuego o los efectos
narrativos explícitos correspondientes; no se incorpora ese sistema ahora.

## 8. Información contextual

Una pista es información tipada y referenciable sobre una posición, elemento
o región. Puede incluir coordenadas y relaciones entre varios espacios. No es
una entidad física invisible: no ocupa volumen ni bloquea el paso, y no se puede
recoger o dañar.

| Primer tipo | Información |
|---|---|
| Espacio | Identifica una habitación, patio o región transitable y su extensión. |
| Conexión | Relaciona espacios y señala recorrido, acceso físico y alturas implicadas. |
| Salida del encuentro | Distingue el acceso al exterior de una conexión interior. |

Los planes utilizan esas distinciones sin deducirlas de todas las casillas.
No se exige todavía clasificar baños, tesorerías o funciones de cada habitación.

Una pista no autoriza una acción: una conexión puede estar cerrada, ocupada o
amenazada. Se consultan sus condiciones reales. Los cambios de acceso y geometría
actualizan el contexto afectado antes de volver a utilizarlo.

Conocer el contexto general no implica que todos los participantes lo conozcan.
Los planes respetan la información disponible. Las pistas futuras de guardias,
peligro o exposición pueden cambiar durante el episodio.

Se añadirán tipos cuando sean necesarios: posición defendible, ocultamiento,
escalada, saltos, bordes peligrosos o rutas candidatas. Su utilización desde nuevos
programas narrativos se diseñará en la integración posterior.

## 9. Planes genéricos y comportamientos

El plan inicial depende del propósito y la situación. Atacar busca eliminar un
objetivo; Rescatar, liberar y extraer a alguien; Robar, tomar y extraer un objeto.
El método puede cambiar sin cambiar el propósito. Abandonar o cambiar el propósito
también es posible y no borra lo ocurrido.

La traducción inicial utiliza acercarse, combatir, alcanzar, liberar, recoger,
auxiliar, esperar y salir. No concede sigilo, engaño o escalada por mencionarlos
como etapas futuras.

Si no conoce la posición del objetivo, el plan puede explorar conexiones conocidas
hacia espacios no visitados, en un orden acotado según acceso y distancia. No
elige consultando en secreto dónde está el blanco. Sin rumbo practicable reconoce
el impedimento en vez de repetir indefinidamente el mismo recorrido.

### Participantes y roles

Un plan puede dirigir a una persona, un subgrupo o todo el grupo. Asigna cometidos
según capacidades y disponibilidad, sin exigir nombres concretos. Los sucesos
identifican quién actuó realmente, no solo quién tenía asignado el rol.

Cuando alguien cae, se marcha o pierde disponibilidad, el plan revisa sus roles.
Puede sustituirlo o ejecutar sucesivamente labores antes paralelas cuando sus
reglas lo permitan. No asigna un rol a alguien incapaz ni progresa fingiendo
participantes presentes. Si faltan medios, reconoce el impedimento o cambia de
dirección. El comportamiento confirma cumplimiento tras las acciones efectivas.

### Prioridades

La prioridad pertenece al plan. Cada participante recibe dirección de un único
plan efectivo por oportunidad de actuar. Gana la prioridad mayor; entre iguales
se conserva la precedencia de asignación. No concede velocidad ni éxito físico.

Perder un rol indispensable detiene la parte que lo requiere. Pueden continuar
labores independientes compatibles. Los efectos continuos solo se mantienen
mientras alguien los ejecuta. Suspender no borra resultados ya confirmados.

Al recuperar participantes se revisa el estado actual. No se reinicia el episodio,
no se usa una ruta inválida ni se reactiva un propósito cancelado. Los
comportamientos no tienen otro peso que compita con la prioridad del plan.

### Progreso y repliegue

Un plan puede estar en ejecución, suspendido, bloqueado, cumplido o cancelado.
La espera deliberada tiene una condición de continuación; no equivale a atasco.
Un impedimento sin resolución detiene el avance automático y se comunica al
jugador, conservando reintento y retirada, sin repetir acciones imposibles.

Se conserva la retirada voluntaria y el auxilio actuales. Sin integrantes capaces
ya no puede ejecutarse la misión y se resuelve derrota o captura. Una parte
indispensable sin asignación viable produce un impedimento. Nuevos umbrales de
cansancio, moral o riesgo para retirarse automáticamente se diseñarán con sus
sistemas; no se inventan al traducir el comportamiento existente.

## 10. Conclusiones causales

El escenario dispone el tablero y conserva cambios; no decide cerrarlo porque
murió un objetivo o se recogió la copa. La dirección solicita una conclusión
con causa y resultados comprobables. Un plan puede iniciar la salida o solicitar
un cierre permitido, pero su prioridad no evita las condiciones de conclusión.

| Conclusión | Condiciones y resultados |
|---|---|
| Salida con logro | Se confirma el propósito y se resuelve la salida necesaria, si la hay. |
| Huida o retirada | Quienes pueden salir alcanzan una salida válida y se resuelve la situación de los demás. El propósito puede estar cumplido, incompleto o abandonado. |
| Captura | Se identifican cautivos, captor y ubicación; la incapacidad de uno no captura automáticamente a todos. |
| Derrota | Se conservan muertes, incapacidades y supervivientes, sin inventar un captor. |
| Abandono del propósito | Cancela ese propósito; otro puede continuar dentro del mismo encuentro. No cierra por sí solo el tablero. |

No se exige eliminar a todos ni cumplir el objetivo universalmente. Sin oposición
pendiente la dirección puede concluir una vez resueltos los resultados, sin una
huida ficticia. Negociación y otros cierres necesitarán sus reglas posteriores.

Cada participante debe quedar resuelto: salió por sus medios, fue transportado,
murió, quedó capturado o permanece separado en el Lugar. Que todos los caminantes
hayan salido no borra a incapacitados ni los transporta automáticamente. Se
intenta auxiliarlos; si no se puede, se comunica el impedimento. Abandonar a
alguien requiere una decisión explícita. La captura necesita un captor válido.

Ante una evacuación imposible, el jugador puede mantener el intento de auxilio
o confirmar el abandono; esa decisión resuelve a los que se quedan sin declarar
un rescate ficticio. No se añade sucesión automática del personaje jugador: su
incapacidad o muerte se entrega a las reglas de continuidad de la partida.

Los separados vivos mantienen ubicación válida y dejan el grupo que se marcha
cuando corresponda. El reglamento inicial no evacua cadáveres por defecto.
No se descarta el escenario con participantes o bienes todavía sin resolver.

Recoger no equivale a extraer. El tesoro recuperado queda en posesión del líder,
aunque otro lo recogiera. El cautivo cumple las condiciones de liberación y salida;
libertad, ubicación y relación con el captor son hechos separados.

Las consecuencias confirmadas permanecen. El cierre consolida las restantes sin
repetirlas y registra tiempo una sola vez. Se conservan las duraciones actuales;
leer, pausar o cambiar altura visible no añade tiempo de partida.

## 11. Integración narrativa posterior

Se conservan los eventos y efectos existentes. La base nueva no les añade
órdenes implícitas ni convierte narración en simulación física. Sus cambios
respetan identidad, ocupación y cierre.

Asignar planes desde eventos queda para el final de esta etapa. Se prevé usar
planes predefinidos y componer comportamientos sin escribir un evento por paso.
El encuentro conserva progreso y resultados; un programa narrativo no necesita
seguir ejecutándose mientras una ficha camina. La autoría se decidirá después.

## 12. Preparación y equipo futuros

Se diseñarán disponibilidad y selección de armas, protección, cuerdas, garfios,
ganzúas y equipo ordinario, junto con distribución de funciones y elección del
método inicial. Los objetos únicos no obligan a individualizar cada utensilio.

Lo llevado se fija al prepararse. Cambiarlo requiere acciones concretas, como
encontrar, tomar o perder equipo. Antes de ofrecer ventajas se definirán alcance,
protección, movilidad, carga, consumo y herramientas. No se introduce ahora una
economía de equipo ni un inventario detallado.

## 13. Adentramiento y acceso futuros

Se quieren permitir aproximación abierta, carga, infiltración, disfraces,
distracciones, vigilancia, animales guardianes y obstáculos del terreno.
Escalar, saltar, balancearse y usar cuerdas requerirán reglas de alcance,
apoyo, dificultad, duración, fallo y recuperación.

El sigilo necesita visión y escucha por observador. Detectar al grupo no alerta
a todos: la comunicación tendrá medios, demoras e interrupciones. Un guardia
que no avisa no transmite información automáticamente. Los disfraces requieren
reconocimiento y disposición; no solo un cambio de apariencia.

## 14. Combate dinámico futuro

Se buscan momentos con consecuencias causales: presión, desequilibrio, caída,
oportunidades decisivas, forcejeos, cansancio, miedo, cooperación y recuperación.
No basta con cambiar la prosa de los golpes.

Cada estado necesitará condiciones, duración, efectos, recuperación y reglas de
coincidencia. Una ventaja transitoria se distingue de una herida. Las inversiones
del enfrentamiento obedecerán reglas, sin garantizar resultados por el relato.

Armas distintas, lucha espalda con espalda, distancia frente a monstruos,
empujones y uso de muebles requieren reglas concretas. Fuego, barricadas,
destrucción, calor y colapsos se incorporarán como sistemas reutilizables, no
como efectos automáticos de un material o una animación.

## 15. Dinámicas de grupo futuras

Se ampliarán protección, contención, separación y reunión, asistencia a heridos,
retirada cubierta, evacuación, rendición y escape con monturas. Los rivales podrán
custodiar, perseguir, avisar o dirigir sin perseguir todos al más cercano.

Relaciones y acontecimientos podrán motivar furia, lealtad o desobediencia sin
simular una personalidad universal. Cada reacción necesita un comportamiento
realizable; la motivación no concede acciones inexistentes ni garantiza éxito.

## 16. Criterios observables de la primera etapa

- Los encuentros traducidos se inician, desarrollan y concluyen sobre la nueva base, conservando sus propósitos y consecuencias.
- Dos alturas distinguen posiciones superpuestas y ofrecen un recorrido válido entre ellas.
- Cambiar el corte muestra lo inferior conocido con ocultación por pisos, sin mover fichas ni revelar información prohibida.
- Volver conserva elementos y estados durante el encuentro; uno nuevo respeta bienes extraídos y bajas del mundo.
- Una puerta se abre mediante una acción, habilita una conexión y se cierra cuando el paso está libre.
- El plan distingue conexiones interiores y salidas sin sustituir sus condiciones físicas.
- La pérdida de disponibilidad de un rol no duplica acciones, borra resultados ni reactiva propósitos cancelados.
- Un impedimento se comunica y permite decidir cómo continuar; no provoca repetición infinita.
- Una evacuación incompleta permite continuar el auxilio o confirmar el abandono, conservando la situación de quienes se quedan.
- Huida, captura y derrota resuelven heridos, cautivos y tesoros sin duplicar efectos o tiempo.

La correspondencia de estos criterios con estructuras, consultas, pruebas y
entregas pertenece al diseño técnico y al plan posteriores.

Esa traducción se define en el [contrato técnico 12](../tecnica/acciones/12-base-espacial-y-planes-encuentros.md)
y en la [etapa de trabajo 052 a 057](../planes/base-espacial-y-planes-encuentros.md).

