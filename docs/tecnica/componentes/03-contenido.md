# Contenido externo, carga e instanciación

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.2 |
| Fecha | 2026-10-12 |

## 1. Propósito y alcance

Este documento diseña la frontera entre los archivos editables del juego y una
Partida nueva. Desarrolla el modelo descrito en el
[diseño general](../../diseno-general.md) y respeta la separación establecida por
el [diseño técnico general](../diseno-tecnico-general.md).

Su alcance comprende:

- leer el contenido necesario para un Mundo inicial;
- representar los datos externos sin acoplarlos a los tipos del núcleo;
- validar identificadores, referencias y datos de presentación;
- producir catálogos validados para reglas, presentación y políticas de NPC;
- crear estados mutables nuevos, iguales al contenido inicial e independientes
  entre sí.

Quedan fuera el setting concreto, las reglas de las acciones, el formato de
guardado, la combinación de varios paquetes o prioridades de mods, la recarga en
caliente y la generación procedural. La ejecución de scripts se define en
[`06-integracion-lua.md`](./06-integracion-lua.md); aquí solo se carga, asocia y
valida su presencia.

## 2. Responsabilidades y límites

La carga pertenece a `desert-tiger-app`. El filesystem y el formato elegido no
atravesarán su frontera hacia `desert-tiger-core`.

El proceso distinguirá tres representaciones:

1. **DTO externos:** reflejan el esquema de los archivos y permiten informar un
   error en términos comprensibles para quien edita contenido.
2. **Contenido validado:** tiene referencias resueltas y separa datos reglados,
   presentación y políticas Lua. Es inmutable mientras dure una partida.
3. **Estado de Partida:** contiene el estado reglado mutable y una instancia
   visual propia del mapa. Las reglas cambian el primero; la aplicación puede
   actualizar la segunda para reflejar resultados semánticos.

Los DTO no serán los structs del dominio. Reorganizar archivos no deberá obligar
a cambiar las reglas; el núcleo no abrirá archivos ni deserializará formatos.

La carga será **atómica**: podrá acumular varios diagnósticos durante la revisión,
pero solo publicará un contenido validado si el conjunto completo es válido. Una
falla no creará una Partida ni dejará catálogos parcialmente utilizables.

## 3. Entradas y datos cubiertos

El conjunto de contenido de la primera versión aportará, como mínimo:

- un Mundo y sus variables iniciales;
- un único mapa visual y su matriz de tiles;
- lugares, con nombre visible, características, coordenada central y coordenadas
  para el rótulo;
- actores, con características, lugar inicial y los datos necesarios para
  identificar al actor jugador;
- arte ASCII asociado a los actores que lo posean;
- scripts Lua asociados a los NPC que los utilicen.

La existencia de relaciones como `conectado` está prevista por el diseño general,
pero su esquema y carga se incorporarán cuando se diseñe ese sistema. No se
deducirán relaciones a partir de coordenadas del mapa.

Cada elemento referenciable tendrá un identificador externo estable. Ese
identificador será distinto del nombre visible y de la ruta del archivo. Al
validar, la aplicación lo resolverá a un identificador tipado: una referencia a
un lugar no podrá resolver por accidente a un actor con el mismo texto. El
alcance exacto de unicidad —global o por tipo— queda pendiente, pero nunca podrá
haber dos candidatos válidos para una referencia.

Las rutas internas, si el formato finalmente las utiliza, se resolverán desde la
raíz explícita del conjunto de contenido. Después de resolver enlaces simbólicos
y normalizar la ruta, todo destino deberá continuar dentro de esa raíz.

## 4. Flujo de carga e instanciación

Conceptualmente, el inicio seguirá este flujo:

```text
raíz de contenido
      ↓ lectura y deserialización
DTO externos completos
      ↓ validación y resolución de referencias
contenido validado e inmutable
      ↓ instanciación sin azar
Partida nueva
```

La validación incluirá, según corresponda:

- archivos legibles y sintaxis válida;
- campos obligatorios, valores y dimensiones admitidos por el esquema;
- identificadores no ambiguos y referencias existentes del tipo correcto;
- exactamente un Mundo ligado a exactamente un mapa;
- ubicación inicial válida para cada actor;
- coordenada central y coordenadas de rótulo dentro de los límites del mapa;
- actor jugador identificable de manera inequívoca;
- arte y scripts accesibles, asociados al actor correcto y válidos para sus
  respectivos subsistemas;
- invariantes del grupo cuando el contenido defina integrantes iniciales.

Los diagnósticos señalarán al menos archivo, elemento o identificador y causa;
cuando el parser lo permita, también línea y columna. El orden de lectura de un
directorio no podrá cambiar el resultado ni decidir silenciosamente qué
definición prevalece.

Después de validar, la aplicación conservará por separado:

- las definiciones regladas que necesita el núcleo;
- un catálogo inmutable de presentación con la definición inicial del mapa,
  nombres visibles, coordenadas y arte, indexado por los identificadores
  correspondientes;
- un catálogo que vincula cada política Lua con su actor.

Al instanciar, la aplicación creará tanto el Mundo reglado como una copia mutable
de la matriz inicial del mapa. No se sortearán valores ni se omitirán elementos.
Dos Partidas creadas desde el mismo contenido validado comenzarán con estados y
mapas equivalentes, pero mutar una no afectará a la otra ni al catálogo fuente.
Las coordenadas y los tiles no se copiarán al estado que consultan las reglas.

## 5. Formato y organización propuestos

La siguiente distribución es una **propuesta pendiente de aprobación**. Sirve
para evaluar comodidad de autoría y modding; todavía no es un contrato:

```text
content/base/
├── world.<formato>
├── map.<formato-o-texto>
├── places/
├── actors/
├── art/
└── scripts/
```

También queda pendiente el formato estructurado. Una opción es mantener los
metadatos en un formato declarativo y el arte ASCII y Lua en archivos de texto
independientes. Deberá probarse con ejemplos reales antes de aprobarse.

El formato elegido deberá preservar el mapa y el arte sin cambios accidentales,
permitir comentarios o documentación suficiente para autores y producir buenos
diagnósticos. Todo texto de contenido que pueda llegar a la terminal —nombres,
arte y mensajes incluidos en diagnósticos— rechazará controles C0/C1 y secuencias
de escape, salvo los separadores que el esquema permita expresamente. Color,
cursor y estilo pertenecen al frontend. Un diagnóstico sobre texto inválido lo
representará de forma escapada, sin copiar el control directamente a la salida.

## 6. Contratos e invariantes

- Una Partida solo nace de un conjunto completamente validado.
- El estado inicial no consume azar y reproduce los valores declarados.
- Toda referencia se resuelve explícitamente; no se enlaza por nombre visible,
  posición en una lista ni orden de archivos.
- El nombre visible de un elemento no sustituye a su identificador estable.
- Cada actor empieza en un lugar existente y nunca en un tile.
- El mapa y sus coordenadas son datos de presentación y no habilitan acciones.
- Los cambios de la matriz de una Partida no modifican los archivos, el catálogo
  validado ni la matriz de otra Partida.
- Una política Lua queda ligada a su actor; el script no elige otra identidad.
- Los archivos fuente no se modifican durante la carga ni durante la Partida.
- No habrá valores por defecto silenciosos que alteren el significado del
  contenido. Todo valor omitible deberá estar definido como tal por el esquema.

## 7. Errores, pruebas y aceptación

Los errores esperables se devolverán como datos, no mediante `panic`. Un archivo
ausente, sintaxis inválida, identificador duplicado, referencia rota, coordenada
fuera del mapa, arte inválido o script inválido impedirá crear la Partida si ese
dato es obligatorio. La obligatoriedad se resolverá en el esquema aprobado.

Las pruebas automáticas cubrirán:

- un conjunto mínimo válido y cada clase de error anterior;
- diagnósticos con el contexto del origen;
- independencia respecto del orden del filesystem;
- rechazo de rutas que, tras resolver enlaces, escapen de la raíz;
- rechazo de controles de terminal en campos visibles;
- rechazo atómico cuando coexistieran datos válidos e inválidos;
- resolución tipada de referencias;
- igualdad inicial y aislamiento entre dos Partidas;
- aislamiento de las modificaciones de tiles entre Partidas y respecto del
  catálogo validado;
- ausencia de consumo de azar durante la instanciación;
- límites de coordenadas y separación del mapa respecto del núcleo;
- validación conjunta de todo el contenido oficial.

El componente será aceptable cuando el contenido oficial produzca una Partida
nueva repetible, cualquier defecto conocido impida una carga parcial y una
prueba pueda modificar tanto su estado reglado como su mapa visual sin cambiar el
catálogo ni otra instancia.

## 8. Preguntas pendientes

- ¿Qué formato estructurado se usará y qué esquema será su contrato?
- ¿Cuál será el árbol definitivo de archivos y cómo se seleccionará su raíz?
- ¿Los identificadores serán únicos globalmente o dentro de cada tipo?
- ¿Qué sintaxis y normalización tendrán identificadores y características?
- ¿Dónde se declarará cuál actor es el jugador?
- ¿Puede el contenido definir compañeros iniciales y, de ser así, cómo?
- ¿Cómo corresponden las coordenadas de rótulo con las letras de un nombre?
- ¿Cómo se representarán cuadros y tiempos del arte ASCII animado?
- ¿Las transformaciones visuales de zonas y tiles se declararán en contenido o
  se asociarán en código a los resultados semánticos?
- ¿Qué campos, artes y scripts serán obligatorios u opcionales?
- ¿Cómo evolucionará el esquema y cuándo se admitirán paquetes de mods?
