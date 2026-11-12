# Acampar — Contrato técnico

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 1.4 |
| Fecha | 2026-10-26 |

> Ampliación implementada el 2026-11-11: el [contrato 25](../componentes/25-conceptos-relaciones-personales.md)
> extiende la navegación a Conceptos y las etiquetas de relaciones personales.

> Ampliación implementada el 2026-10-26: la ubicación visible y la
> representación de vínculos registrados/deducidos se definen en el
> [contrato 14, sección 6](../componentes/14-parentesco-actores-contexto.md#6-fichas-ubicación-inversas-y-deducciones).

> Ampliación implementada el 2026-10-26: el
> [contrato 15](../componentes/15-fogata-procedural.md) amplía las secciones 8.1
> y 8.3 con la fogata animada y su iluminación. Prevalece para ese incremento
> sobre la presentación estática original de este contrato.

## 1. Fuente, alcance y precedencia

Implementa el [diseño de Acampar](../../diseno/acampar.md). Ese documento define
la experiencia y las reglas; este fija su traducción a Rust, contenido e interfaz.
Describe el código implementado y sus contratos; no es un plan de trabajo.

Comprende apertura y cierre de la acción, consulta de fichas, metadatos de
Actores, catálogo de características y vista terminal con retratos y fogata.
No añade recuperación, encuentros, inventario, tipos de relación, secretos
ni nuevos comportamientos de Lua. Las reglas de Viajar
permanecen en el [contrato 06](./06-implementacion-viajar.md).

Este contrato amplía los componentes 02, 03, 04, 05 y 09 en los puntos enumerados
en la sección 11. Prevalece para esas ampliaciones;
los demás contratos conservan su alcance. Se mantienen un proceso y las
dependencias `terminal → app → core` del
[diseño técnico general](../diseno-tecnico-general.md).

## 2. Módulos y responsabilidades

| Archivo | Responsabilidad |
|---|---|
| `core/src/acciones/acampar.rs` | Validar quién acampa y producir el coste de cierre. |
| `core/src/acciones.rs` | Declarar y reexportar esa API junto a la de Viajar. |
| `app/src/contenido.rs` | Cargar nombres, biografías y explicaciones; catálogo de presentación. |
| `app/src/fichas.rs` | Proyectar una ficha y sus referencias inmediatas desde el estado actual. |
| `app/src/sesion.rs` | Fase Acampando, disponibilidad, intenciones y finalización común. |
| `app/src/lib.rs` | Reexportar las vistas, referencias y errores públicos necesarios. |
| `terminal/src/campamento.rs` | Focos, consulta de fichas y composición de la pantalla. |
| `terminal/src/interfaz.rs` | Integrar modo, entrada y dibujo compartido de retratos. |
| `terminal/src/recursos.rs` | Cargar el arte de fogata propio de la terminal. |

Los prefijos `core`, `app` y `terminal` representan los crates `desert-tiger-*`
existentes. No hay dependencias nuevas. Las fichas no contienen tipos Ratatui,
teclas, rutas de recursos, punteros mutables al Mundo ni una copia de su grafo.

## 3. Regla del núcleo

La API reexportada desde `acciones` es:

```rust
pub struct Acampada { /* actor privado; sin Clone ni Copy */ }
pub struct ResultadoAcampada { pub tiempo: TiempoAccion }
pub enum ErrorAcampada {
    ActorFueraDelEscenario,
    ActorDesconocido,
    ActorMuerto,
    IntegranteDelGrupo,
}
pub fn iniciar_acampada(mundo: &Mundo, actor: IdActor)
    -> Result<Acampada, ErrorAcampada>;
pub fn finalizar_acampada(acampada: Acampada) -> ResultadoAcampada;
```

`iniciar_acampada` comprueba existencia, ausencia de `muerto` y, para un Actor
distinto del jugador, que no sea un integrante del Grupo. Devuelve un valor
opaco sin modificar el Mundo, la fecha ni el azar. La aplicación abre solo el
campamento del jugador. El núcleo no abre menús: otro consumidor podría iniciar
y cerrar la misma acción sin interfaz, con las mismas reglas.

`finalizar_acampada` consume el valor y devuelve exactamente
`TiempoAccion { duracion: Duracion::desde_dias(1), fraccion: FraccionDia::CERO }`.
No mueve Actores, recupera estados, crea relaciones ni cambia la fecha por sí
mismo. Los nombres y biografías pertenecen a presentación, fuera del núcleo.
`Acampada` y `ResultadoAcampada` derivan `Debug`; `ErrorAcampada` deriva
`Clone, Debug, Eq, PartialEq` e implementa `Display` y `std::error::Error`.

Actualmente `EjecucionAccion`, `PasoAccion` y sus funciones reexportadas son
específicos de Viajar. No se introduce Acampar en `RegistroEscena`, `OpcionViaje`
ni `CierreViaje`, ni se generaliza ese motor en este incremento. Acampar usa el
módulo anterior y una fase propia de sesión. `SolicitudAccion` y la API Lua
continúan admitiendo sus solicitudes actuales; no se habilita Acampar para
políticas NPC en este alcance.

## 4. Contenido y compatibilidad

### 4.1. Nombre y biografía del Actor

`ActorDto` incorpora `nombre: String` obligatorio y `biografia: String` con
`#[serde(default)]`. `PresentacionActor` conserva ambos y publica sus getters
internos. Ejemplo de los campos nuevos dentro del RON habitual, sin alterar
retrato, relaciones o política:

```ron
nombre: "Príncipe Perdido",
biografia: "Un príncipe perdido que intenta abrirse camino junto a sus compañeros.",
```

`nombre` no puede estar vacío ni consistir solo en espacios; admite Unicode,
sin controles C0/C1, escapes ANSI ni saltos de línea. No exige ancho de una celda
ni unicidad: la identidad sigue siendo `clave`. `biografia` admite LF y normaliza
CRLF a LF; rechaza CR aislado y los demás controles C0/C1. No recorta espacios.
Ausente o vacía significa «Sin biografía disponible». La biografía no puede
introducir ni modificar características o relaciones.

Se migran los seis RON oficiales agregando nombre visible y biografía breve,
sin inventar historia adicional a lo ya establecido. No se crean archivos de
biografía por Actor. Mods anteriores deben añadir `nombre`; la ausencia de
biografía sigue siendo válida. La carga mantiene `deny_unknown_fields`.
`VistaActor` incorpora `nombre: String` para identificar el selector del grupo;
la ficha usa los mismos datos. `Narrador::actor` consulta el nombre visible del
catálogo para que un mismo Actor no tenga dos nombres de presentación.

### 4.2. Catálogo de características

Archivo opcional `caracteristicas.ron` en la raíz de contenido seleccionada por
`--content`; para el contenido oficial, `content/base/caracteristicas.ron`:

```ron
(
    entradas: [
        (
            clave: "fuerte",
            nombre: "Fuerte",
            descripcion: "Destaca por su fuerza física. Las acciones que consideran esta cualidad aplican sus propias reglas.",
        ),
    ],
)
```

Cada entrada tiene los tres campos obligatorios. `clave` se valida con
`Caracteristica::nueva`: lowerCamelCase y límite vigente de 64 bytes. Se
rechazan claves duplicadas antes de construir el catálogo; se deserializa una
lista para no perder duplicados por sobrescritura de un mapa. Nombres visibles
siguen la regla de `nombre` de Actor; descripciones siguen la regla de biografía,
pero no pueden estar vacías ni contener solo espacios. No hay campos de bonos,
efectos o código ejecutable.

Archivo ausente equivale a catálogo vacío; un archivo presente ilegible o
inválido es error de contenido. Se permiten entradas no utilizadas y
características de Actores/Lugares sin entrada, incluidas las creadas durante la
partida. La reserva de una clave desconocida es su propio texto y «Sin
descripción disponible». El catálogo no es una lista cerrada de características
permitidas ni se declara en `world.ron`.

Ejemplos inválidos de campos o entradas (fragmentos del RON correspondiente):

| Fragmento | Error |
|---|---|
| `nombre: ""` | Nombre visible vacío. |
| `biografia: "Texto\tcon tabulador"` | Control no permitido; solo LF es admisible en el texto multilínea. |
| `clave: "ConoceDesiertos"` | No respeta lowerCamelCase. |
| Dos entradas con `clave: "fuerte"` | Clave duplicada, incluso con explicaciones diferentes. |
| `descripcion: "   "` | Explicación vacía. |

La lectura del catálogo reutiliza las restricciones de rutas, raíz de contenido
y diagnósticos del [componente 03](../componentes/03-contenido.md). Solo la
inexistencia del archivo opcional activa la reserva; un error de permisos o una
ruta que escape de la raíz no se confunde con un catálogo ausente.

Se guarda como `BTreeMap<String, DescripcionCaracteristica>` en
`CatalogoPresentacion`; la descripción contiene nombre y texto. Los diagnósticos
incluyen archivo, campo y clave de la entrada usando `elemento: None` y el
detalle del diagnóstico existente; no se amplía `TipoElementoContenido` ni se
convierte el catálogo en un
elemento del Mundo. El catálogo oficial documenta las características usadas
por el contenido y por los resultados actuales de Viajar, según sus reglas
existentes, sin añadir ventajas nuevas.

### 4.3. Consultas eficientes

Los catálogos de Actores y Lugares mantienen el orden canónico de IDs que ya
garantiza la carga. `IdActor::indice()` e `IdLugar::indice()` se hacen públicos
con el retorno comprobado `Option<usize>` actual. El catálogo resuelve
`vec.get(id.indice()?).filter(|elemento| elemento.id == id)`; se retiran las
búsquedas lineales anteriores de `CatalogoPresentacion::actor/lugar`.

Las relaciones se consultan con `Mundo::relaciones_de(elemento)` y
`Mundo::relacion(id)`. No se recorre `Mundo::relaciones()` entero para una ficha,
ni se crea un segundo índice de vínculos en la aplicación.

## 5. Proyección de fichas

### 5.1. API pública neutral

`app` reexporta `ExtremoRelacion`, `IdRelacion` y `TipoRelacion` ya existentes,
además de los tipos siguientes de `fichas.rs` (campos públicos):

```rust
pub struct VistaFicha {
    pub elemento: ExtremoRelacion,
    pub nombre: String,
    pub biografia: Option<String>, // Actor; None para Lugar
    pub ubicacion: Option<VistaUbicacionActor>,
    pub entradas: Vec<EntradaFicha>,
}
pub enum EntradaFicha {
    Caracteristica {
        clave: String,
        nombre: String,
        descripcion: Option<String>,
    },
    Relacion(VistaRelacionFicha),
}
pub struct VistaRelacionFicha {
    pub vinculo: VinculoFicha, // tipos y deducciones en contrato 14, sección 6
    pub etiqueta: String,
    pub referencia: ResumenElemento,
    pub explicacion: Option<String>,
}
pub enum SentidoRelacionFicha { Saliente, Entrante, Simetrica }
pub struct ResumenElemento {
    pub elemento: ExtremoRelacion,
    pub nombre: String,
    pub texto: String,
}
pub enum ErrorConsultaFicha {
    FaseIncompatible,
    CampamentoNoVigente,
    ElementoInexistente,
    EstadoInconsistente(String),
}
impl Partida {
    pub fn consultar_ficha(&self, campamento: IdCampamento,
        elemento: ExtremoRelacion) -> Result<VistaFicha, ErrorConsultaFicha>;
}
```

Las vistas y errores derivan `Clone, Debug, Eq, PartialEq`; `SentidoRelacionFicha`
también `Copy`. Son valores de lectura. El helper interno
`fichas::proyectar(mundo, contenido, elemento)` construye una sola ficha. No usa
`proyectar_actor`, que exige retrato: un Actor externo con `retrato: None`
también es consultable.

### 5.2. Reglas de proyección

1. Resolver el Actor o Lugar solicitado. Leer sus características del Mundo
   actual y su nombre/biografía del catálogo, sin copiar el estado inicial.
2. Emitir primero características en orden lexicográfico de clave estable,
   usando las explicaciones del catálogo cuando existan.
3. Leer sus relaciones incidentes por el índice existente y emitirlas por
   `IdRelacion` ascendente, una entrada por relación, aunque varios vínculos
   conduzcan al mismo elemento.
4. Resolver `otro_extremo(elemento)`, construir su resumen y conservar el ID,
   tipo y sentido. No proyectar recursivamente la ficha de esa referencia.

| Tipo y extremo consultado | Sentido | Etiqueta |
|---|---|---|
| Conectado, cualquiera de los dos Lugares | Simetrica | `Conectado con {nombre}` |
| Conoce, Actor de origen | Saliente | `Conoce {nombre del Lugar}` |
| Conoce, Lugar de destino | Entrante | `Conocido por {nombre del Actor}` |

El resumen de Actor usa hasta los primeros 160 caracteres de su biografía y
añade `…` si hay más; una biografía vacía usa la reserva. El de Lugar enumera
hasta tres nombres de características en orden de clave y añade «y N más»;
sin características dice «Sin características». El nombre del elemento siempre
se conserva completo en los datos. Abrir la ficha permite leer su información
completa; el resumen es deliberadamente breve.

Consultar no modifica fecha, resto de día, RNG, ubicación, Grupo, conocimiento,
características ni relaciones. Abrir un Lugar no equivale a viajar ni concede
Conoce. El coste de proyección depende de las características y relaciones del
elemento, sus resúmenes inmediatos y las búsquedas del catálogo; no del número
total de relaciones del Mundo. No se guardan fichas de todo el grafo ni un
historial creciente, por lo que los recorridos cíclicos no acumulan estructura.

## 6. Sesión, disponibilidad y cierre

### 6.1. Tipos y estado

Ampliaciones de la fachada de `sesion.rs`:

```rust
pub struct IdCampamento(u64); // campo y constructor privados
pub struct IntegranteCampamento { pub actor: IdActor, pub nombre: String }
pub struct VistaCampamento {
    pub id: IdCampamento,
    pub lugar: IdLugar,
    pub integrantes: Vec<IntegranteCampamento>,
}
// Variantes nuevas de enums existentes:
IntencionFrontend::ComenzarCampamento
IntencionFrontend::AbandonarCampamento { campamento: IdCampamento }
VistaSesion::Acampando(VistaCampamento)
AccionDisponible::Acampar
ErrorIntencion::CampamentoNoVigente
ErrorIntencion::AcampadaRechazada(ErrorAcampada)
```

`IdCampamento` deriva `Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd`;
las dos vistas derivan `Clone, Debug, Eq, PartialEq`. El ID usa el contador
comprobado `Partida.ordinal`, sin reutilización durante una partida. Su
agotamiento produce `CodigoFatalSesion::ContadorAgotado` antes de abrir la fase.
`Partida` incorpora `acampada: Option<Acampada>` privada. El identificador de
campamento distingue un Enter atrasado de una sesión anterior.

`integrantes` contiene compañeros en su orden visual y después al jugador. No
es el Grupo reglado ni lo modifica. Si el Grupo quedó vacío durante la partida,
siempre queda el jugador. No se modifica la exigencia actual de compañeros en
la carga inicial. La opción de salida la agrega el frontend después de esos Actores.

### 6.2. Transiciones

| Estado e intención | Resultado |
|---|---|
| Disponible + ComenzarCampamento | Validar jugador con el núcleo, asignar ID y abrir Acampando. |
| Acampando + consultar_ficha con ID vigente | Devolver ficha; sin transición. |
| Acampando + AbandonarCampamento con ID vigente | Consumir Acampada y finalizar un día. |
| Abandonar con otro ID durante Acampando | CampamentoNoVigente; conservar estado. |
| Abrir/abandonar/consultar desde otra fase | FaseIncompatible; sin cambios. |

`AccionDisponible::Acampar` se ofrece junto a Viajar cuando el jugador no tiene
`muerto`, aunque Viajar no tenga destinos. Jugador muerto no puede abrir
campamento. Mientras Acampando está activo no se puede iniciar Viajar ni otra
acampada; abrir un error recuperable tampoco cancela la fase.
El orden de acciones conserva Viajar primero y añade Acampar después. Los
consumidores seleccionan por variante, sin asumir que todas las acciones son
Viajar ni convertir sus `match` en reservas silenciosas.

Al abrir, la vista del Mundo para los retratos se proyecta con expresiones
`None`, sin resumen NPC reciente ni acciones disponibles de la fase normal.
La vista del campamento y los retratos no cambian al seguir referencias. No se
ejecuta la fase NPC al entrar o consultar: el Mundo permanece estable.

### 6.3. Finalización única

Extraer de `Partida::publicar(PasoAccion::Finalizada)` un helper privado común
`concluir_accion(tiempo: TiempoAccion) -> Result<(), FalloCierreAccion>`, usado
por Viajar y Acampar. El enum privado `FalloCierreAccion` distingue
`Calendario(String)`, `Npc(String)` y `Proyeccion(String)`:

1. Retirar la ejecución activa, pasos, opciones y acampada para impedir reentrada.
2. Confirmar el tiempo con `app::tiempo::confirmar`.
3. Ejecutar una sola vez la fase NPC existente, con la nueva fecha y el resto
   fraccionario que resulta de la confirmación.
4. Reconstruir `mundo_visible`, conservar el resumen resultante y publicar
   `VistaSesion::Disponible` con expresiones neutrales.

No se reinicia `Partida.resto`: sumar un día conserva la fracción procedente de
un viaje anterior. Se conserva el [contrato temporal](../componentes/06-calendario-duracion.md)
y el [comportamiento NPC](../componentes/07-fase-npc.md). Los NPC no añaden tiempo
al día ya transcurrido. Un segundo Enter o una intención repetida no confirma
otro día. No se permite cancelar Acampar y volver al Mundo sin ese coste.

El helper distingue fallos de calendario, fase NPC y proyección, y los mapea a
`Calendario`, `IntegracionNpc` o `EstadoInconsistente` de `CodigoFatalSesion`.
Un error de calendario se detecta sin cambiar fecha ni resto; si la fase NPC
falla después de confirmar el día, se publica Fatal y no se reintenta ni se
promete una reversión de esa fase. Se preserva el tratamiento de salida del
frontend. Cerrar el programa mediante Q descarta la partida sin guardado; no
equivale a regresar gratuitamente al Mundo.

## 7. Estado local y controles de terminal

`terminal::campamento::EstadoCampamento` conserva ID vigente, índice horizontal,
`Option<VistaFicha>`, `Option<usize>` de entrada vertical y página del detalle.
El desplazamiento de lista se deriva del foco y la altura disponible; la página
y las dimensiones del detalle usan estado visual local al recomponer la vista. La ficha es la única consultada; los resúmenes ya
incluidos en ella sirven al mover el foco vertical. El estado se crea al entrar
en Acampando, se mantiene para ese ID y se descarta al salir de esa fase.

| Entrada | Operación |
|---|---|
| A en la vista Disponible | ComenzarCampamento si Acampar está disponible. |
| Izquierda/derecha | Selección horizontal circular; abrir ficha del Actor elegido o enfocar Salir. |
| Arriba/abajo | Entrada anterior/siguiente, acotada en los extremos. |
| Enter sobre relación | Consultar y abrir `referencia.elemento`, conservando el ancla horizontal. |
| Enter sobre característica o ficha vacía | Sin acción. |
| Enter sobre Salir | AbandonarCampamento con el ID vigente. |
| PageUp/PageDown | Página anterior/siguiente del detalle, acotada. |
| Esc | Enfocar Salir; no ejecutar el cierre. |
| Q | Confirmación global existente de salida del programa. |

El índice inicial es el del jugador; cada ficha recién abierta enfoca la
primera entrada, lista al inicio y página de detalle cero. Si está vacía, el
foco vertical es `None`. Arriba/abajo reinicia la página del detalle, pero no
el selector horizontal; el desplazamiento mantiene la entrada seleccionada
visible. Tras seguir una referencia, izquierda/derecha opera desde el último
índice horizontal aunque el elemento abierto sea del grupo. En Salir no hay
foco vertical ni navegación de detalle.

Cambiar de Actor o seguir una referencia actualiza ficha y focos solo después
de que la consulta haya terminado correctamente. Un error conserva también el
ancla horizontal, no solo la ficha. Si falla la primera consulta al entrar, se
mantiene Acampando con ficha `None`, diagnóstico visible y posibilidad de
enfocar Salir; el día se cobra únicamente al confirmar ese cierre.

El despacho de Acampando ocurre antes del manejo de flechas usado para paginar
viñetas. D no abre detalle de Viajar y V/A no comienzan otra acción en esta fase.
Q tiene prioridad global; cancelar su confirmación conserva ficha, focos,
páginas y parpadeo según el contrato existente. Redimensionar no consulta ni
confirma acciones; solo recompone el texto y acota la página al rango vigente.
También reajusta el desplazamiento de la lista para mantener visible la entrada
vertical seleccionada.

## 8. Composición visual y lectura completa

### 8.1. Regiones y retratos

Se conserva el mínimo 80×24. Las dos últimas filas siguen siendo barra contextual
y barra de acciones. En la región restante `R`, se reserva abajo el arte y arriba
la ficha. Sea `alto_arte` el máximo alto de las bases del grupo y de la fogata:

```text
alto_region_arte = min(alto_arte, R.height - 6)
alto_ficha = R.height - alto_region_arte
```

Se usan restas saturadas y solo se compone esta vista cuando se cumple el
tamaño mínimo. Los retratos se calculan con su alineación inferior y posiciones
horizontales habituales; el dibujo se recorta contra la región de arte, sin
escalar ni desplazar sus bases. Retratos modificados demasiado altos pueden
perder parte superior para garantizar al menos seis filas de ficha legible.
El origen vertical lógico se calcula con signo, o se omiten explícitamente filas
superiores del gráfico antes de convertir a coordenadas de terminal. No se
satura ese origen a cero: eso desplazaría el dibujo y perdería la alineación.
Con los tres retratos oficiales de 14 filas quedan ocho filas de ficha en 80×24.

Extraer de `dibujar_mundo` el cálculo y dibujo compartido de retratos. Acampar
llama a esa parte sin dibujar tiles, rótulos ni marcador de mapa. Se conserva
compañeros → jugador, solapamiento del 25 %, transparencia y composición base →
expresión → ojo del [contrato 13](../componentes/13-retratos-por-capas.md).
Consultar otro Actor no reemplaza estos retratos.

La fogata se apoya en la misma línea inferior. Su coordenada X es la posición
del último retrato más su ancho base más una columna de separación, usando el
cálculo completo, no el borde recortado. Si sale de la región se recorta; nunca
desplaza retratos, modifica el selector ni invade las barras.

### 8.2. Ficha compacta

La primera fila es encabezado con tipo y nombre; la última indica entrada
seleccionada/total y página de detalle/total. Entre ambas hay dos columnas:
lista de ancho `(R.width / 3).clamp(24, 36)`, una columna separadora y el resto
para detalle. En 80×24 quedan seis filas de cuerpo, lista de 26 columnas y
detalle de 53. No se añaden marcos que consuman esas filas.

Cada entrada usa una fila con prefijo `[C]` o `[R]` y marcador `>` para el foco.
Los rótulos largos se recortan por ancho de terminal, no por bytes; su texto
completo está en el detalle. La lista se desplaza por filas al mover el foco.

El detalle contiene primero la información de la entrada seleccionada:
característica y aclaración, o relación y resumen del otro elemento. Después,
si la ficha es de Actor, contiene `Biografía de {nombre}` y su texto completo;
se incluye también el nombre completo de la ficha en este contenido paginable.
Una ficha vacía presenta nombre y biografía o la indicación de que no hay
características ni relaciones. La biografía no es una tercera selección.

El contenido se ajusta al ancho y se pagina por las filas del cuerpo, preservando
todo el texto. Se usa ajuste que no descarte palabras más anchas que la columna
y la misma medición Unicode que el resto de la interfaz. PageUp/PageDown permite
leerlo completo, incluida la biografía aunque no quepa en la primera página.
La página cero siempre comienza por la aclaración o referencia enfocada.

La barra contextual identifica el campamento, el elemento consultado y la
opción horizontal activa. Al ver un elemento externo no se presenta como
integrante: por ejemplo, `Ficha: Valle Fértil | Selector: Príncipe Perdido`.
La barra de acciones muestra las teclas de la sección 7. En Salir la ficha se
sustituye por el aviso del coste de un día y Enter como confirmación, sin otra
ventana modal. En terminal insuficiente se conserva el aviso existente; no se
avanza el tiempo al redimensionar ni se pierde el estado local.

### 8.3. Recurso de fogata

`assets/terminal/campamento.ron` pertenece al frontend y contiene:

```ron
(
    fogata: (
        arte: r#"
  /\  
 /██\ 
  ██  
 /__\ 
"#,
        estilo: (frente: "amarillo", fondo: None, modificadores: []),
    ),
)
```

Se reutilizan `EstiloDto`, conversión de colores y normalización de bloques de
`recursos.rs`. El gráfico debe ser rectangular, no vacío, sin controles y de
una columna por carácter; sus espacios son transparentes. Este archivo es un
recurso requerido de la terminal: ausencia o contenido inválido se diagnostica
antes de entrar al modo interactivo. No se crea una ClaveEscena de Viajar para
resolverlo. Este ejemplo de 6×4 sigue siendo el camino estático. El recurso
oficial activa la representación procedural mediante el campo `efecto` del
[contrato 15](../componentes/15-fogata-procedural.md#4-recurso-y-compatibilidad),
que define su esquema, altura, animación e iluminación.

## 9. Errores e invariantes

| Caso | Tratamiento |
|---|---|
| Nombre ausente/inválido, biografía con controles, catálogo presente inválido | ErrorContenido con archivo, campo y Actor o clave de característica. |
| Actor sin retrato en una referencia | Ficha válida; no usar la proyección de retratos del grupo. |
| Característica sin entrada o biografía ausente | Reserva visible de la sección 4; no error. |
| Consulta fuera de fase, ID viejo o elemento inexistente | ErrorConsultaFicha; conservar ficha y focos anteriores, mostrar diagnóstico breve en barra contextual. |
| Intención de otra fase, cierre repetido o ID viejo | ErrorIntencion; sin otro día ni fase NPC. |
| Relación indexada ausente, extremo incompatible o presentación faltante | EstadoInconsistente con detalle saneado; no ocultar la relación ni publicar una ficha parcial. |
| Calendario o cierre NPC fallido | Fatal según sección 6.3; sin reintento automático. |
| Lista vacía o página fuera de rango tras resize | `None` o acotación; nunca indexación sin comprobar. |

La consulta construye el resultado completo antes de sustituir la ficha local.
El error de una referencia válida es un defecto de integración, no motivo para
alterar vínculos o añadir datos durante el renderizado. Los diagnósticos no
interpolan controles de los archivos en la terminal. Ningún camino normal usa
`unwrap`, `expect` o conversiones de coordenadas que puedan desbordar.

## 10. Verificación y aceptación

| Contrato | Verificación necesaria |
|---|---|
| Regla core | Inicio válido y rechazos por inexistencia, muerto e integrante; cierre devuelve un día sin azar ni mutación del Mundo. |
| Lectura actual | Cambios de característica/Conoce de Viajar aparecen al acampar, sin reconstruir desde los RON iniciales. |
| Relaciones | Conectado desde ambos extremos; Conoce saliente/entrante con etiquetas correctas y destino consultable. |
| Exploración | Actor → Lugar → Actor externo, ciclo repetido y ficha de Actor sin retrato; sin expansión recursiva ni historial. |
| Contenido | Nombre requerido, biografía omitida/vacía/CRLF, controles rechazados, catálogo ausente/vacío, duplicados y claves inválidas. |
| Reserva | Característica dinámica sin explicación, Actor sin biografía y elemento sin entradas. |
| Pureza | Abrir y consultar no cambia fecha, resto, RNG, estado, Grupo ni relaciones; fuente de azar de prueba falla si se usa durante consulta. |
| Tiempo | Cerrar suma un día, conserva fracción previa y ejecuta NPC una vez; doble cierre y ID de otro campamento no repiten efectos. |
| Fallo temporal | Desbordamiento de calendario conserva fecha/resto y lleva a Fatal, sin fase NPC. |
| Entrada | Inicio en jugador, horizontal circular, vertical acotada, ancla tras referencia, Enter según foco y Esc solo enfoca Salir. |
| Render | TestBackend 80×24 y tamaño mayor: ficha legible, lista y textos largos accesibles, retratos estables, fogata a su derecha, recorte seguro. |
| Confirmación/resize | Q cancelado conserva ambos focos y página; tamaño insuficiente no pierde la consulta ni avanza el día. |
| Regresión | Viajar conserva escenas, decisiones, fecha exacta y fase NPC tras extraer el cierre común. |

La prueba de humo carga archivos reales, abre campamento, consulta biografía y
característica, sigue Conoce hasta un Lugar y su referencia entrante a otro
Actor, regresa al selector y abandona; comprueba día, resumen NPC y posibilidad
de volver a Viajar. Se ejecutan los controles de la
[metodología](../../metodologia-desarrollo.md#7-controles-automáticos) y build de
producción durante la implementación. La revisión manual final cubre lectura,
arte y comodidad de los dos focos; no se detiene por cada ficha o variante.

## 11. Contratos afectados y decisiones cerradas

| Documento | Ampliación que rige este contrato |
|---|---|
| [02-modelo-mundo](../componentes/02-modelo-mundo.md) | Acceso público comprobado a índice de IdActor/IdLugar; sin cambios de características o relaciones. |
| [03-contenido](../componentes/03-contenido.md) | Nombre/biografía de Actor y catálogo opcional de explicaciones, sección 4. |
| [04-sesion-aplicacion](../componentes/04-sesion-aplicacion.md) | Fase, intenciones, consulta y finalización común, secciones 5–6. |
| [05-motor-acciones](../componentes/05-motor-acciones.md) | API core separada de acampada, sección 3; el protocolo reanudable sigue siendo el de Viajar. |
| [09-frontend-terminal](../componentes/09-frontend-terminal.md) | Entrada, ficha compacta, retratos compartidos y recurso de fogata, secciones 7–8. |

Los planes futuros deben referenciar estas secciones; no necesitan completar
decisiones de datos, foco, tiempo o render. No quedan preguntas bloqueantes.
Se aplazan las mecánicas de descanso, otros tipos de elementos, historial de
fichas y solicitudes Lua de Acampar.


## 12. Verificación de la implementación — 2026-10-26

Pasaron `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace --all-targets` y `cargo build --workspace --release`.
Las 96 pruebas se distribuyen en 40 de núcleo, 27 de aplicación, 25 de terminal
y 4 del importador. La carga conjunta valida los seis Actores, los Lugares,
las políticas Lua y el catálogo oficial.

Las pruebas de sesión verifican consultas sin azar, estado adquirido y relaciones
entrantes, cierre único con fracción previa, ejecución NPC, campamentos antiguos,
Grupo vacío y desbordamiento de calendario. Las de terminal cubren recorridos
cíclicos y externos, focos, textos largos, Unicode, resize, recorte de retratos,
lista vacía, confirmación global y regreso al selector de Viajar.

Se hizo un recorrido manual del ejecutable de producción en una PTY de 80×24:
Príncipe → Bandido → Hamada Roja → Bandido → Descenso del Drâa → Valle del Drâa
→ Cazador Amigable. Se comprobaron biografías, explicaciones, referencias,
fogata, PageDown, Q cancelado, retorno horizontal a Sirviente, Esc + Enter para
abandonar y apertura posterior del selector de Viajar. La salida del programa
restauró la terminal y terminó con código cero. El acabado artístico queda
abierto a refinamientos posteriores, sin cambiar los contratos de esta versión.
