# Escenarios volumétricos modulares y contexto táctico

| Campo | Valor |
|---|---|
| Estado | Diseño futuro; no implementado ni aprobado como plan de ejecución |
| Fecha | 2026-11-19 |
| Alcance | Espacio tridimensional, generación modular adaptable y datos para IA de encuentros |
| Base | [Encuentros y Atacar](../acciones/10-encuentros-y-atacar.md) |

## 1. Propósito

Los escenarios futuros tendrán volumen en sus datos, aunque se sigan dibujando con
caracteres en la terminal. Su autoría combinará instrucciones de alto nivel:
colocar una casa, unir dos puntos mediante un puente, conectar una escalera o
agregar un balcón. Los módulos producirán geometría, conexiones y contexto útil
para decidir movimientos y tácticas.

La generación calculará información reutilizable para todos los combatientes.
Cada actor podrá consultar opciones locales y rutas candidatas sin reconstruir
el análisis completo del escenario en cada pulso. Las reglas de las acciones
seguirán determinando si puede ejecutar una opción concreta.

Es una ampliación del subsistema de encuentros, compartida por Atacar y Rescatar.
Los escenarios siguen siendo temporales por encuentro; no exige mapas locales
persistentes para todos los Lugares del mundo. Los generadores actuales continúan
vigentes hasta desarrollar esta capacidad mediante entregas jugables acotadas.

## 2. Volumen y representación

Una posición futura distinguirá área y coordenadas locales x, y, z. La altura z
representará espacio físico: suelo, pisos, cubiertas, arcos, huecos y obstáculos
con altura. No será solamente un número para ordenar dibujos. Antes de implementar
se fijarán escala vertical y correspondencia con los niveles del modelo actual;
no se añadirán dos campos de altura con significados ambiguos.

Un puente podrá tener una superficie transitable arriba, aire debajo, columnas
de apoyo y un arco. Un personaje podrá pasar por debajo si dispone de espacio
libre suficiente, y otro caminar encima en la misma posición horizontal. El
modelo comprobará superficies de apoyo, volumen ocupado y altura libre según la
huella del personaje. Muros, barandas y techos tendrán efectos físicos separados
de que la cámara los muestre u oculte.

El almacenamiento podrá usar capas, regiones dispersas o estructuras compactas;
no se exige una matriz densa gigante de vóxeles. Deberá poder representar sólidos,
superficies, aberturas y volúmenes libres. Material, geometría, soporte y apariencia
son datos distintos: una celda de piedra no explica por sí sola si es suelo,
columna, escalera o parte de una bóveda.

El frontend podrá mostrar cortes por nivel, ocultar techos y proyectar varias
alturas. Esa visualización no cambia colisiones ni permite atravesar paredes.
Las figuras de dos caracteres de alto son otra decisión estética, independiente
del volumen físico y de los futuros cuerpos articulados.

## 3. Módulos adaptables

Un módulo será un generador parametrizado, con identidad y subelementos, no un
sello que copia siempre el mismo rectángulo de tiles. Declarará:

- Parámetros: extremos, dimensiones, orientación, material y variaciones admitidas.
- Volumen solicitado, espacios que deben quedar libres y apoyos necesarios.
- Conectores: puertas posibles, entrada de escalera, unión de balcón o extremo de puente.
- Políticas de encuentro con otros módulos: adosar, abrir, recortar, adaptar o rechazar.
- Geometría producida, elementos semánticos y anclajes para eventos narrativos.

Ejemplo conceptual de una receta futura, no API disponible:

```text
casa = crear_casa(zona, tamaño_variable)
escalera = conectar_escalera(patio, casa.conector_de_acceso)
balcón = añadir_balcón(casa, fachada, altura)
puente = crear_puente(origen, destino, anchura, altura_libre)
```

Crear un puente incluirá cubierta, arco y apoyos adecuados al tramo. La receta no
colocará manualmente cada ladrillo. Añadir una escalera a una fachada compatible
podrá abrir una puerta, construir su descanso y registrar el acceso completo.
Añadir un balcón podrá modificar la fachada, crear su puerta y reservar espacio
libre bajo él cuando corresponda.

Las variaciones serán acotadas: dimensiones, posición de accesos, orientación o
disposición de apoyos dentro de límites válidos. Una misma semilla y versión de
receta deberán reproducir el escenario; distintas semillas podrán dar variantes,
sin prometer que nunca se repita una disposición. Repintar o leer una escena no
vuelve a generar el lugar.

## 4. Resolver contactos e intersecciones

Que un módulo sepa adaptarse no significa que pueda borrar cualquier elemento.
Ante un puente que toca un edificio, la composición podrá recortar una porción
permitida de fachada, abrir un acceso o ajustar el extremo del puente. La decisión
dependerá de los conectores y las políticas de ambos módulos.

Proceso propuesto:

1. Consultar geometría y elementos existentes en el volumen afectado.
2. Proponer la incorporación y las modificaciones necesarias, con su procedencia.
3. Comprobar límites, apoyos, espacios libres, compatibilidad de materiales y accesos.
4. Validar que no se destruyan elementos protegidos, objetivos ni conexiones requeridas.
5. Confirmar juntos el módulo nuevo, los cambios anteriores y sus datos derivados.

No se confirmarán primero las columnas y después se descubrirá que no cabe la
cubierta. Si no hay composición válida, se probará otra alternativa dentro de un
presupuesto acotado, o se devolverá un rechazo comprensible. Se evitarán intentos
infinitos y modificaciones dependientes del orden accidental de iteración.

Los módulos conservarán identidad y relación con sus partes: una puerta abierta
por una escalera seguirá perteneciendo a una fachada concreta. Las reglas de
soporte iniciales podrán ser estructurales y sencillas —apoyos declarados y
volúmenes compatibles— sin requerir una simulación de ingeniería de edificios.

## 5. Información contextual del escenario

La geometría generada y sus módulos producirán datos con referencias estables a
posiciones y elementos. Los siguientes son ejemplos de información deseada:

| Información | Utilidad y condición |
|---|---|
| Puerta | Posición, orientación, destinos, apertura y bloqueo actual. |
| Escalera | Superficies que conecta, desnivel, anchura y espacio libre. |
| Punto de salto | Origen, aterrizaje, distancia, desnivel y volumen a atravesar. |
| Borde de abismo | Dirección, caída y superficies inferiores conocidas. |
| Cobertura y exposición | Obstáculos, alturas y direcciones que protegen o dejan expuesto. |
| Paso defensivo | Anchura, accesos y direcciones desde las que puede rodearse. |
| Material combustible | Qué elemento puede arder y a qué otros está conectado. |
| Propagación potencial | Regiones que podrían verse afectadas por un incendio, sin garantizar su resultado. |
| Rutas candidatas | Alternativas hacia objetivo, entrada, salida o puntos de reunión. |

Una pista «puede saltarse» describe una oportunidad geométrica; no concede una
habilidad. La acción comprobará capacidad del actor, tamaño, carga, espacio libre,
ocupación, amenaza y riesgos. Una puerta puede existir pero estar cerrada; una
escalera puede estar bloqueada por una ficha.

«Bueno contra muchos enemigos» tampoco será una bonificación universal: el mapa
podrá identificar un paso estrecho y sus flancos. La IA valorará esa posición según
armas, aliados, adversarios y propósito. La exposición a disparos depende también
de desde dónde disparan y qué alturas ocupan; no se reducirá a un booleano fijo.

Los datos precalculados podrán indicar que prender un elemento es una oportunidad
para dañar a alguien. La evaluación tendrá que considerar también a los aliados,
rutas de retirada y propagación posible. Registrar combustibilidad no implementa
por sí solo fuego, humo, daño o derrumbes: necesitarán sus propias reglas.

## 6. Navegación y decisiones de los actores

Se derivará un grafo de superficies y conexiones utilizables. Caminar, atravesar
una puerta, subir una escalera, saltar o trepar serán tipos de conexión distintos,
con requisitos y costes. Las diagonales deberán respetar volumen libre y esquinas;
las fichas de varios segmentos requerirán una comprobación de huella propia.

Los módulos aportarán conexiones explícitas, complementadas por análisis de la
geometría final. No bastará con confiar en que una receta dijo «hay una escalera»:
su salida deberá conducir a una superficie válida.

Se podrán precalcular regiones, cuellos de botella, accesos, distancias y algunas
rutas alternativas. No se almacenarán todos los caminos posibles entre cada par
de tiles ni un conjunto completo por actor. El análisis compartido dará candidatos;
la IA elegirá según su situación y las acciones verificarán el siguiente paso.

Por ejemplo, una IA que quiere retirarse podrá consultar dos salidas y sus rutas
base, descartar una ocupada por enemigos y preferir la que conserve apoyo aliado.
Otra podrá evitar un borde aunque el recorrido sea más corto. La información
facilita decisiones tácticas complejas; no obliga a implementarlas todas junto
con la primera versión volumétrica.

## 7. Datos estáticos, cambios y actualización

Al generar el escenario se calculará lo que dependa de su estructura: apoyos,
conectores, superficies, bordes, anchuras, materiales y navegación base. La
ocupación, las amenazas y la posición de amigos y enemigos serán dinámicas.
Cobertura, exposición, fuego y rutas podrán combinar información estructural con
consultas sobre el estado actual.

Cada dato derivado tendrá una revisión y dependencias. Abrir una puerta, destruir
una columna, encender un elemento o reubicar fichas invalidará lo que corresponda.
Un cambio podrá afectar solo una zona, varias regiones o conectividad global; una
reconstrucción parcial no se considerará suficiente sin comprobar esas dependencias.

La IA no usará una pista desactualizada como garantía. Se actualizará la parte
necesaria o se hará una validación directa acotada antes de actuar. Los resultados
estructurales reutilizables se compartirán entre personajes; las preferencias y
condiciones particulares se evaluarán por actor.

## 8. Integración con encuentros y eventos

El core será autoridad sobre geometría, composición, navegación y validación.
La app cargará recetas, perfiles y contenido de autor. La terminal proyectará
solamente el estado visible y reproducirá cambios confirmados.

Los eventos Lua podrán solicitar operaciones de alto nivel sobre módulos o usar
anclajes semánticos. Crear una escalera o abrir una fachada seguirá el mismo
proceso de composición que la generación; no escribirá directamente matrices
internas. Se conservará la posibilidad de una reubicación narrativa explícita,
con validación física del destino y una explicación de autor.

Un bloque de evento confirmará conjuntamente Mundo, escenario, conexiones y
contexto derivado, conservando tiempo y azar al fallar. Los caminos anteriores
se invalidarán cuando proceda. Las pistas tácticas internas no revelarán al
jugador habitantes o geometría que su vista todavía deba ocultar.

## 9. Desarrollo gradual y aceptación futura

Esta dirección no requiere desarrollar todo en una sola entrega. Posible orden:

1. Volumen mínimo: dos alturas transitables y una escalera, con reglas coherentes.
2. Composición: casa con conectores; escalera que abre una puerta y balcón adosado.
3. Puente adaptable con paso inferior, apoyos y contacto válido con una construcción.
4. Contexto compartido: superficies, bordes, saltos candidatos y rutas de retirada.
5. Actualización tras un cambio narrativo y primeros consumidores tácticos de la IA.
6. Ampliar exposición, combustibilidad y propagación junto con sus reglas propias.

Cada entrega tendrá un caso jugable y pruebas acotadas. La aceptación incluirá
reproducibilidad, escenarios transitables, ausencia de intersecciones inválidas,
composición rechazada sin efectos parciales y actualización de datos derivados.
También se comprobará que pasar debajo de un puente no colisione con su cubierta,
que una escalera desemboque en un acceso válido y que un actor no siga una ruta
invalidada por un cambio.

Quedan por decidir al preparar cada entrega: escala vertical, almacenamiento,
catálogo inicial de módulos, políticas concretas de recorte y soporte, presupuesto
de generación y alcance de las reglas de movimiento, fuego y percepción. No se
establecen todavía fórmulas de daño, resistencia estructural o puntuación táctica.
