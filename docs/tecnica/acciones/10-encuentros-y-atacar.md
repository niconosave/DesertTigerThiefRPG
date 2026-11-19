# Encuentros y Atacar — Diseño técnico

| Campo | Valor |
|---|---|
| Estado | Borrador técnico para revisión; no implementado; reglamento inicial pendiente |
| Versión | 0.1 |
| Fecha | 2026-11-19 |
| Fuente de mecánicas | [Encuentros y Atacar](../../diseno/encuentros-y-atacar.md) |
| Alcance | Tablero temporal, ejecución de Atacar, viñetas, intervenciones y vista terminal |

## 1. Autoridad y alcance

Este contrato traduce las mecánicas a responsabilidades, datos y transiciones de
código. No sustituye el reglamento ni es un plan de implementación. Los nombres
de tipos y formatos siguientes son API objetivo; no describen capacidades ya
publicadas por la aplicación.

La acción Atacar intenta matar a un Actor o monstruo situado en el mismo Lugar
del mundo que el grupo. Rescatar reutilizará el subsistema cuando tenga su propio
contrato. Atacar Lugares, barcos, combate a distancia, salto y niveles superiores
no se implementan en este alcance. Los comandos narrativos sí pueden transformar
el terreno y reubicar participantes; no requieren implementar la simulación de
cada maniobra que explicaría esos cambios.

La hostilidad confirmada se conserva literalmente: abandonar **cualquier casilla
amenazada** habilita reacción del enemigo elegible, aunque la casilla de destino
siga junto al mismo enemigo. Si el ataque acierta se cancela ese paso, conservando
su origen y los pasos anteriores de la ruta. Entrar en alcance no activa por sí
solo esta regla de salida.

Referencias normativas:

- [Arquitectura general](../diseno-tecnico-general.md): tres crates, un proceso.
- [Motor de acciones](../componentes/05-motor-acciones.md) y
  [Viajar implementado](./06-implementacion-viajar.md): ejecución reanudable.
- [Operaciones del Mundo](../componentes/11-operaciones-mundo.md): mutación validada.
- [Eventos](../componentes/23-eventos-narrativos.md),
  [agenda](../componentes/24-agenda-eventos-mundo.md) y
  [diálogos](../componentes/28-dialogos-eventos-narrativos.md).

Los pendientes de la sección 23 impiden dar por cerrado el reglamento jugable;
no impiden definir estos contratos técnicos. No se suplirán con fórmulas ocultas
en el código ni con valores elegidos por el generador.

## 2. Situación del código y ampliación necesaria

Hoy `acciones.rs` reexporta una ejecución y unos registros pertenecientes a
Viajar. `SolicitudAccion` solo contiene Viajar. Sus decisiones, progreso y cierres
no son contenedores generales. El adaptador de intervenciones conserva también
suposiciones de viaje. Acampar tiene un flujo propio.

La aplicación ya posee coordinador de eventos, efectos Lua, presentación por
momentos, agenda y una única instancia de Mundo. La terminal dibuja tiles,
retratos, diálogos y animaciones. No existe todavía tablero reglado de encuentro,
ocupación local, generación de habitaciones ni movimiento de fichas.

La integración agregará una envoltura por acción, conservando el contrato de
Viajar dentro de su variante:

```rust
// Esquema de responsabilidades; tipos omitidos se detallan más adelante.
enum SolicitudAccion {
    Viajar { actor: IdActor, destino: IdLugar },
    Atacar { actor: IdActor, objetivo: IdActor },
}
enum EjecucionAccion {
    Viajar(viajar::EjecucionViaje),
    Atacar(atacar::EjecucionAtaque),
}
enum PasoAccion {
    Viaje(viajar::PasoViaje),
    Encuentro(encuentros::SalidaEncuentro),
}
```

El refactor debe alcanzar las entradas de la sesión, las decisiones, el cierre,
el adaptador de intervenciones y el despacho de solicitudes NPC. No se llenan
registros de ataque con reservas, rutas mundiales o destinos ficticios. Acampar
continúa con su API actual; compartir la envoltura no exige migrarlo ahora.

## 3. Límites entre crates

| Parte | Responsabilidad |
|---|---|
| Core: encuentros | Estado reglado local, generación, ocupación, accesos, consultas espaciales, comandos y registros neutrales. |
| Core: Atacar | Objetivo, fases, elección de viñetas y desenlace de la acción. |
| Core: particulares | Evaluar y resolver intentos concretos con reglas y azar explícitos. |
| App | Autoría RON, vínculo con el Mundo, coordinador Lua, transacciones, percepción pública y vistas narradas. |
| Terminal | Entrada, cámara local, arte, reproducción, pausa, velocidad y paneles. |

Las casillas mecánicas son datos del núcleo. Los glifos, colores, fuentes y
fotogramas son presentación. Esto amplía el dominio espacial del core sin llevar
allí el `Mapa` mundial visual, Ratatui, archivos ni Lua.

No se agrega un crate, servicio, proceso de red, motor de física ni sistema ECS
para este incremento. Las llamadas siguen `terminal → app → core`.

## 4. Propiedad, duración e identidades

`Partida` posee una ejecución activa, que posee su `EstadoEncuentro`. La ejecución
conserva su tablero durante todas las viñetas. Al cerrar se conservan consecuencias
persistentes y se libera el estado local. El arte y las plantillas son recursos
compartidos; los mapas generados no se escriben en archivos de Lugares.

Se introducen identidades tipadas:

- `IdEncuentro`: único durante la Partida, asignado al aceptar la acción.
- `IdArea`, `IdNivel`, `IdFicha`, `IdElemento`, `IdAcceso`: locales al encuentro.
- `IdVineta`, `IdTramo`, `IdDecisionEncuentro`: identifican unidades de ejecución.
- `RevisionEncuentro`: versión interna del estado confirmado.
- `TokenContinuacionEncuentro`: autoriza una única continuación vigente.

Toda referencia externa local incluye o queda ligada a `IdEncuentro`. Los IDs
locales no se reciclan mientras exista ese encuentro. Una ficha retirada deja de
ser resoluble; una orden posterior con su referencia falla sin actuar sobre otra.

No se agrega X/Y de combate al Actor. Tampoco se usan `IdLugar` para habitaciones
ni `CoordenadaMapa` para posiciones locales.

## 5. Modelo espacial y preparación para niveles

```rust
struct PosicionLocal {
    area: IdArea,
    nivel: IdNivel,
    x: u16,
    y: u16,
}
struct Plano {
    ancho: u16,
    alto: u16,
    celdas: Vec<TipoCelda>,
}
enum TipoCelda { Suelo, Muro, Vacio }
struct Area {
    clave: ClaveArea,
    planos: BTreeMap<IdNivel, Plano>,
}
struct Acceso {
    id: IdAcceso,
    extremos: [PosicionLocal; 2],
    estado: EstadoAcceso,
}
```

Cada matriz es rectangular en memoria; `Vacio` y `Muro` permiten superficies
transitables irregulares. La consulta de paso combina celda, acceso, elemento,
ocupación y regla vigente. No se almacenan varias versiones independientes de
«transitable» que puedan contradecirse.

En el incremento inicial cada área contiene únicamente el nivel base. Referenciar
otro nivel falla. Área y nivel siguen presentes en posiciones y accesos para no
identificar ocupaciones por X/Y solamente.

La futura cámara podrá mostrar niveles superpuestos, techos y paredes ocultables.
Su recorte no modifica la geometría mecánica. Los accesos ya admiten extremos en
áreas y niveles diferentes; no se habilita movimiento vertical implícito. Las
reglas de caída, tiro vertical o resistencia de pisos pertenecen a ampliaciones.
No se construye ahora una matriz tridimensional del escenario completo.

## 6. Fichas, elementos y vínculo con el Mundo

```rust
enum OrigenFicha {
    Actor(IdActor),
    Local(ClavePerfilFicha),
}
struct Ficha {
    id: IdFicha,
    origen: OrigenFicha,
    posicion: Option<PosicionLocal>,
    bando: IdBando,
    estado_local: EstadoFicha,
}
struct ElementoLocal {
    id: IdElemento,
    clave: Option<ClaveElemento>,
    clase: ClaseElemento,
    posicion: PosicionLocal,
    propiedades: PropiedadesElemento,
}
```

`posicion: None` representa una ficha que ya salió o se retiró del tablero; no
se usa para esconder una ficha existente a los observadores. La ocupación se
indexa por posición completa. La regla inicial de cuerpos y aliados determinará
qué fichas bloquean; no se presume intercambio de posiciones ni apilamiento.

Un Actor solo tiene una ficha activa en el encuentro. Sus características y
relaciones se consultan desde Mundo; no se clonan biografías ni se mantiene otro
booleano mutable independiente para su muerte. Los estados locales representan
lo que solo existe durante el episodio. Un resultado persistente sobre un Actor
se aplica mediante operaciones mundiales en la misma confirmación que ajusta su
presencia local. En fichas anónimas, sus estados físicos pertenecen al encuentro.

Los perfiles locales aportan capacidades regladas de defensores o animales
anónimos, sin fabricar Actores persistentes. El blanco inicial de Atacar sigue
siendo un Actor real. Un participante que deba persistir como personaje tras el
encuentro necesita identidad mundial; la promoción automática de anónimos a
Actores nuevos queda fuera del incremento.

Los bandos expresan oposición en este episodio, no nuevas relaciones diplomáticas
permanentes. La hostilidad se consulta desde una sola política del encuentro.

Los elementos aportan propiedades semánticas: ocupación, bloqueo, soporte o estado
narrativo. Una cortina identificada conserva su `IdElemento` al incendiarse. Los
elementos no definidos pueden especializarse mediante comandos; no son un tipo
obligatorio que el generador deba llenar en todas las habitaciones.

## 7. Invariantes del estado local

La validación inicial y la preparación de cambios comprueban:

1. Referencias, posiciones, niveles y tamaños válidos, con multiplicación de
   dimensiones comprobada antes de reservar memoria.
2. Unicidad de claves semánticas e identidades locales; ausencia de fichas
   duplicadas del mismo Actor.
3. Ocupaciones y apoyos legales según clase y reglamento. Los elementos adheridos
   a una pared no se validan como si fueran un combatiente parado en ella.
4. Accesos consistentes con sus extremos y política de paso.
5. Coherencia de participantes, identidad mundial y estado local derivado.
6. Validez del objetivo, fase y referencias conservadas por el director.
7. Correspondencia entre índices de ocupación/elementos y los datos fuente.

Una intervención narrativa puede omitir la prueba de recorrido o de salto, pero
no instalar referencias inexistentes ni ocupaciones incompatibles. Eliminar el
suelo bajo una ficha exige declarar o resolver su recolocación/condición en el
mismo cambio. Esta validación no impone simular el derrumbe completo.

## 8. Generación y autoría del escenario

La app carga un catálogo opcional `content/base/encounters.ron`. Su ausencia
mantiene compatibles contenidos anteriores; no crea un escenario universal oculto.
El catálogo incorpora perfiles de generación y de fichas locales. No contiene
una matriz dibujada para cada Lugar.

Ejemplo propuesto de un perfil, sujeto al reglamento que se cierre:

```ron
(
    version: 1,
    perfiles: [(
        clave: "edificacion_basica",
        requiere: ["civilizado"],
        prioridad: 10,
        generador: HabitacionesConectadas,
        reglamento: "ataque_inicial_v1",
        areas: [
            (clave: "entrada", rol: Entrada, ancho: 24, alto: 12),
            (clave: "acceso", rol: Paso, ancho: 24, alto: 12),
            (clave: "sala", rol: Objetivo, ancho: 24, alto: 12),
        ],
        conexiones: [("entrada", "acceso"), ("acceso", "sala")],
        obstaculos: 4,
        vacios: 1,
    )],
)
```

El registro de generadores está cerrado en Rust. Los datos eligen un generador
conocido y sus parámetros; no ejecutan algoritmos RON. Los roles se vinculan con
la entrada del grupo, el objetivo y la salida reales al iniciar. El nombre de
reglamento del ejemplo todavía no designa una versión implementada ni aprobada.

La elección de perfil filtra las características del Lugar, comprueba requisitos
y utiliza prioridad explícita; desempata por clave estable. No infiere un perfil
naval por encontrar mar si no existe esa familia implementada. Sin perfil
compatible se informa que no hay escenario disponible, sin consumir tiempo.

El generador recibe datos validados y una fuente de azar. Construye primero áreas,
accesos y recorridos utilizables; coloca después obstáculos y vacíos preservando
la conectividad física requerida, y finalmente participantes y elementos. La
ocupación enemiga puede dificultar una ruta físicamente conectada.

Cada intento utiliza una semilla registrada. Los intentos son limitados y no se
sortean nuevos mapas hasta que alguno facilite la victoria. Tras el límite, un
fallo diagnosticado conserva la partida. La generación exitosa se realiza una
sola vez por encuentro; dibujar, cambiar cámara o continuar no la repite.

Las claves de área, roles y anclas hacen posible referirse a posiciones sin
conocer sus coordenadas. Si falta una cortina, un evento puede especializar un
elemento compatible o crearla explícitamente. Si necesita otra habitación, un
comando puede incorporar una nueva área generada y sus accesos. No se exige
predeclarar todos los objetos posibles.

El primer perfil será pequeño y de cuartos sencillos. Familias agrestes,
calles, casas y torres se incorporarán gradualmente con pruebas jugables; el
catálogo admite varias familias sin exigir implementarlas todas ahora.

## 9. Ejecución, viñetas y segmentos

`atacar::EjecucionAtaque` conserva dirigente, objetivo, Lugar de origen, participantes,
instante inicial, duración acumulada, director y estado local.

Fases propias: `Preparar`, `Aproximar`, `Penetrar`, `AfrontarOposicion`,
`AlcanzarObjetivo`, `Salir`, `Cerrar` y `Finalizada`. Son estados de propósito,
no una lista invariable de escenas: los resultados pueden omitir, repetir o
cambiar fases. No codifican la historia particular de un NPC.

Una viñeta posee un `IdVineta`, una clase y un propósito. Produce uno o más tramos
acotados. Un tramo contiene intentos, cambios confirmados, registro causal y
un punto de continuación. El título y la descripción de la viñeta permanecen
estables entre tramos. Un resumen final presenta los cambios significativos.

```rust
enum SalidaEncuentro {
    Tramo(RegistroTramo),
    Decision(DecisionEncuentro),
    Punto(PuntoIntervencion),
    Pendiente(TokenContinuacionEncuentro),
    Finalizada(ResultadoAtaque),
}
```

Una llamada no calcula todo el ataque por adelantado. Se detiene ante decisiones,
intervenciones, cambios de viñeta, cierre o presupuesto de trabajo. Un punto
situado a mitad del combate debe alcanzarse antes de resolver consecuencias que
el evento podría modificar. El prefijo confirmado se conserva.

Antes de consumir azar, continuar valida encuentro, token, revisión, fase y
participantes. La consulta de vista nunca ejecuta reglas. Las continuaciones
obsoletas o duplicadas se rechazan sin volver a resolver el tramo.

## 10. Reglas, dirección y acciones particulares

El director de Atacar consulta hechos y selecciona la próxima viñeta y los
intentos. La viñeta coordina responsables, blancos y recorridos. Las reglas
resuelven cada intento y producen consecuencias; no escriben prosa ni dibujan.

Las reglas de combate permanecen código Rust. `ReglamentoEncuentro` identifica
una versión cerrada que agrupa geometría, orden de actuación, golpe, estados,
percepción y duración. No es un lenguaje de fórmulas editables ni una colección
de callbacks Lua para sustituir cada golpe. Sus funciones concretas se completan
cuando se acuerden las mecánicas pendientes.

Las reglas espaciales compartidas incluyen consultas de soporte, ocupación,
vecindad, acceso, contacto y amenaza. Los tipos de intento iniciales necesitarán
una entrada mecánica propia antes de implementarse: desplazarse localmente,
combatir cerca e infiltrarse según la versión aprobada. No se reutiliza `Andar`
con su duración de viaje para mover una ficha una casilla.

El director no es una IA que descubre todas las estrategias de Conan. Utiliza
prioridades finitas por propósito y reconoce `SinRuta`, `Bloqueado`, `Descubierto`,
`Separado`, `ObjetivoFueraDeAlcance` y resultados de combate. Los eventos pueden
introducir oportunidades que el director genérico no inventa.

Para alcanzar un adversario, las metas son posiciones de contacto legales según
el reglamento; no se intenta ocupar la casilla bloqueada del propio blanco.
La búsqueda de recorridos consulta el mismo predicado de paso que la ejecución.
Puede considerar riesgo cuando la regla lo defina. Una ruta encontrada es una
propuesta, no una autorización para atravesar enemigos o ignorar reacciones.
Fuera de peligro puede comprimirse trabajo interno solo cuando se demuestre
equivalente al recorrido paso a paso: sin saltar detecciones, ocupaciones ni
reacciones. La vista conserva la ruta real por casillas.
Las rutas guardan la revisión en la que se calcularon; una mutación relevante
las invalida y obliga a reconsiderarlas antes del próximo paso.

## 11. Secuencia del desplazamiento amenazado

Para un paso ordinario `origen → destino`:

1. Validar posición actual, vecindad/acceso, apoyo y ocupación del destino.
2. Consultar enemigos elegibles que amenazan la casilla de **origen**.
3. Resolver sus reacciones en el orden que establezca el reglamento.
4. Tras cada resolución, volver a comprobar que mover la ficha sigue siendo
   posible. Si hay acierto, conservar origen, aplicar el golpe y terminar el
   intento sin publicar un cambio de posición a destino.
5. Si no se interrumpe, confirmar exactamente ese paso y sus consecuencias.
6. Continuar la ruta solo desde el estado resultante.

Cruzar un acceso conserva esta comprobación de salida. El registro público
distingue `CruzarAcceso` de un paso dentro del mismo plano; cambiar de área o
nivel no se representa atravesando visualmente el espacio entre dos matrices.

No se consulta únicamente si el destino queda fuera del alcance del rival.
Rodear al enemigo dentro de su zona sigue exponiendo cada salida elegible.
Tampoco se cancela retroactivamente la ruta completa tras un acierto posterior.

Las reacciones disponibles, las diagonales, el orden entre defensores y la
recuperación de oportunidades son políticas del reglamento pendiente. No se
ocultan valores por defecto en el algoritmo de ruta. La versión inicial deberá
impedir reintentos y cadenas de reacciones ilimitados sin progreso temporal.

Los intercambios simultáneos se expresan como grupos de cambios compatibles.
No se asume que dos fichas puedan ocupar el mismo destino, cruzarse entre sí o
intercambiarse por el mero hecho de animarlas a la vez. El orden causal confirma
ocupación y reacciones; la animación respeta ese orden.

## 12. Comandos narrativos y fronteras de libertad

Se introduce `ComandoEncuentro`, separado de `OperacionMundo`. Vocabulario objetivo:

| Comando | Efecto |
|---|---|
| IncorporarFicha | Coloca un Actor existente o un perfil local en una posición válida. |
| RevelarFicha | Cambia la información disponible sobre una ficha que ya existe. |
| EspecializarElemento | Define un elemento poco determinado conservando su identidad. |
| IncorporarElemento | Añade explícitamente un elemento de escenario. |
| CambiarTerreno | Aplica un parche de celdas y declara ajustes requeridos por sus ocupantes. |
| CambiarAcceso | Abre, cierra o sustituye la conexión local indicada. |
| CambiarEstadoElemento | Por ejemplo, establece que una cortina está incendiada. |
| IncorporarArea | Genera un área compatible y enlaza accesos declarados. |
| ReubicarNarrativamente | Recoloca fichas o grupo sin probar una ruta ordinaria; declara duración. |
| RetirarFicha | Retira una presencia local según la situación y las consecuencias persistentes. |

Un comando no infiere calor, daño de fuego ni derrumbes por dibujar llamas. Esos
resultados requieren comandos expresos o reglas futuras. La retirada de una ficha
vinculada no elimina al Actor del Mundo.

`ReubicarNarrativamente` no dispara reacciones por las casillas omitidas ni finge
una tirada de salto. Su duración se suma una vez al acontecimiento, no una vez por
ficha trasladada; cero es válido si el cambio no añade tiempo. Trasladar entre
áreas del mismo escenario no usa `OperacionMundo::Trasladar`, cuyo destino es un
Lugar mundial.

Los comandos pueden crear hechos inesperados; validan coherencia estructural y
referencias, no que el acontecimiento pueda deducirse de una cadena de maniobras.
Los intentos reglados siguen consultando sus precondiciones físicas completas.

## 13. Confirmación conjunta y azar

Todos los cambios se preparan antes de publicarse. El estado candidato comprende
Mundo, encuentro, continuación del director, duración acumulada, traza causal,
coordinador narrativo candidato y su proyección para el observador. El cursor del
evento, cola, marcadores, estado de finalización y próxima vista se confirman
juntos con sus efectos. Si una parte falla, no se instala otra mitad.

La app amplía el patrón de `Mundo::preparar_operaciones` y `MundoPreparado`:

1. Copiar el estado local candidato, el generador de azar del encuentro y, si
   interviene un evento, su coordinador narrativo.
2. Preparar un Mundo candidato sin instalarlo.
3. Procesar los cambios mundiales y locales en su orden sobre esos candidatos.
4. Validar invariantes cruzadas y preparar los resultados públicos.
5. Confirmar Mundo, ejecución, coordinador, azar y registro en una sección síncrona sin
   callbacks ni operaciones que todavía puedan fallar.

La API de preparación conjunta pertenece al núcleo; la app conserva la posesión
de Partida y coordina la confirmación. El núcleo devuelve solo un candidato y
registros semánticos de solo lectura. La app prepara sus vistas a partir de ellos
antes de confirmar; el núcleo no recibe tipos de presentación ni redacta texto.
El coordinador y las vistas candidatas permanecen en app, que conserva el patrón
de clonar el coordinador por paso. La preparación conjunta retiene préstamos
exclusivos a los destinos durante la validación y confirmación, o una exclusión
equivalente garantizada por la fachada.

Se permite ejecutar Lua síncrono de consulta y presentación sobre los candidatos
de solo lectura antes de confirmar. Se prohíben reentrada mutable, esperas y
conservar un candidato entre llamadas de entrada o reproducción. Toda la
preparación y confirmación ocurre en la misma llamada; ningún evento intermedio
puede quedar borrado por instalar una copia antigua.

El azar del encuentro se conserva en app mediante una fuente reproducible que
pueda prepararse sobre una copia y se entrega como `FuenteAzar` al núcleo.
Generación y resolución utilizan flujos derivados y registrados; el reloj visual
nunca los consume. No se reintenta una transacción fallida con nuevos dados para
obtener un resultado aceptable.

Los límites de trabajo pueden confirmar un prefijo coherente y devolver un token
nuevo. Un fallo técnico posterior conserva ese prefijo; no lo vuelve a ejecutar.
La revisión interna aumenta al confirmar un cambio mecánico, informativo o
narrativo relevante. No aumenta por un repintado de la terminal.

## 14. Integración de los eventos narrativos

Se amplía `ContextoIntervencion` con un contexto local opcional: encuentro,
revisión, viñeta, momento, participantes y propósito. Los eventos de Viajar siguen
recibiendo su contexto anterior sin fabricar un tablero.

Puntos iniciales a registrar y validar:

- `atacar.inicio`: solicitud válida, antes de comprometer la preparación.
- `encuentro.antes_vineta`: se conoce el propósito; sus intentos todavía no se resolvieron.
- `encuentro.momento`: frontera causal habilitada por esa viñeta, por ejemplo al entrar en un área.
- `encuentro.despues_vineta`: resultados confirmados y resumen preparado.
- `encuentro.antes_cierre`: estado final local conservado para preparar consecuencias.
- `accion.terminada`: punto común ya existente.

No se evalúan todos los eventos después de cada casilla. Se reutiliza el catálogo
indexado por punto y se filtra por el contexto de momento. Una viñeta publica solo
sus fronteras explícitas. El coordinador sigue admitiendo un evento activo.

En `atacar.inicio` todavía puede no existir tablero generado. Ese punto admite
operaciones mundiales y control del inicio, pero las consultas y comandos locales
requieren un estado de encuentro disponible. Los primeros cambios sobre fichas
pueden insertarse en `encuentro.antes_vineta`, después de generar el escenario.

En efectos Lua, `operaciones` admite una secuencia ordenada de operaciones
mundiales y comandos de encuentro, mediante descriptores tipados. Los programas
actuales conservan su significado. Las consultas locales son de solo lectura y
los comandos se ofrecen únicamente en efectos de un encuentro activo.

Ejemplo ilustrativo de la ampliación:

```lua
return {
    operaciones = {
        op.encuentro.abrir_acceso("compuerta"),
        op.encuentro.incorporar_ficha({perfil="tigre", area="sala", ancla="jaula_1"}),
        op.encuentro.incorporar_ficha({perfil="tigre", area="sala", ancla="jaula_2"})
    },
    siguiente = "reaccion"
}
```

`op.encuentro` es un espacio de constructores, no acceso mutable al tablero.
Referencias y claves se resuelven contra el encuentro de la intervención. Fuera
de ese contexto se rechazan; una clave ausente no selecciona otro elemento al
azar. Crear un objeto nuevo es un comando explícito distinto de buscarlo.

Los comandos de Mundo y encuentro se aplican en el orden común del lote, no
primero todos los de un dominio y después los del otro. Así una operación puede
observar el resultado candidato de la anterior, conservando atomicidad conjunta.

`Continuar` tras mutaciones reconsidera el propósito y descarta planes pendientes
incompatibles. No adopta la prohibición específica de Viajar contra cambios de
participantes. Si un Actor vinculado cambia de ubicación mundial, muere o sale del
grupo, la reconciliación valida su presencia y la continuación antes de publicar.

`InterrumpirAccion` solicita el cierre reglado de Atacar, con sus consecuencias;
no libera el tablero antes de conservarlas. `OmitirRelato` no se convierte en
permiso para omitir combate. Sustituir un relato y establecer un nuevo estado
narrativo son operaciones distintas. Los diálogos siguen usando el contrato 28.

Una intervención que cambie el Lugar mundial de los participantes deberá resolver
su desvinculación o cierre local en la misma preparación; la persistencia al
cerrar no puede devolverlos automáticamente al Lugar anterior.

## 15. Resultado, persistencia y liberación

`ResultadoAtaque` conserva dos dimensiones: estado del objetivo y situación de
los participantes. Incluye actores muertos, supervivientes, salidos, capturados
y separados, junto con el cierre y la duración acumulada. No se reduce a un
booleano de victoria.

La muerte y la herida persistentes utilizan operaciones de características de
Actor conforme al reglamento aprobado. No se copian ciegamente todos los estados
locales como nuevas etiquetas del Mundo.

La captura necesitará una operación mundial específica, por ejemplo
`RegistrarCautiverio { prisionero, captor }`, que registre `PrisioneroDe` y valide
identidades, ubicación y composición. No utiliza `RegistrarEsclavitud` ni convierte
la captura en esclavitud por inferencia. La selección del captor y los efectos
sobre la libertad de movimiento pertenecen al reglamento de captura pendiente.

Las consecuencias confirmadas durante el combate no se repiten al cerrar. El
cierre aplica solo lo pendiente y valida ubicación de todos los supervivientes,
composición del grupo, cautiverios y destino del blanco fugado. Ninguna referencia
local imprescindible puede quedar colgando de un escenario ya liberado.

Los anónimos locales no se exportan automáticamente como Actores. Si una
consecuencia requiere conservar a una persona concreta y no hay representación
persistente válida, el cierre debe resolverlo antes de finalizar; no inventa ni
pierde cautivos silenciosamente.

Atacar entrega su resultado a Partida. No agrega un sistema de sucesión del
personaje jugador ni un final de campaña propio. Los finales narrativos observan
los hechos conservados bajo los contratos existentes o su futura ampliación.
Tras el último resumen, la app conserva solo lo necesario para el cierre temporal
y libera tablero, índices, comandos y rutas; la terminal libera su reproducción.

## 16. Un solo tiempo de partida

Cada intento resuelto y cada elipsis que añada duración aportan `TiempoAccion`
al acumulador del encuentro. Los registros llevan el instante proyectado desde
el inicio. La exactitud de las duraciones depende del reglamento pendiente;
no se presupone una ronda de cierta cantidad de segundos.

Una acción bloqueada no se reintenta infinitamente sin una nueva oportunidad
reglada de actuar. Una consulta, repintado, página de texto o animación añade
cero tiempo. Los cambios de fichas de un mismo acontecimiento no suman varias
veces su duración común.

Se reutiliza el cierre de sesión: acontecimientos de fin de acción, confirmación
del tiempo mundial una sola vez, agenda correspondiente al intervalo y fase NPC,
con la continuación que fije Partida. No se crea una segunda agenda independiente
por casilla ni se ejecuta la fase NPC durante cada fotograma. La observación de
los sucesos fechados conserva la regla vigente de usar la situación al finalizar
la acción.

## 17. Proyección pública, memoria y datos ocultos

La vista local es una proyección del conocimiento reglado, no el tablero completo
con un booleano que la terminal decide tapar. No se reutiliza directamente la
`VistaMapa` mundial, que conserva datos de terreno aun en celdas desconocidas.

La regla de percepción pertenece al núcleo. La app proyecta solo lo que permite
esa regla, sin calcular por su cuenta si un guardia detectó a alguien. Mientras
el reglamento no defina percepción y conocimiento de objetivos, no se habilita
una selección que enumere por defecto todos los Actores del Lugar.

```rust
enum VistaCeldaEncuentro {
    Desconocida,
    Recordada(DatosUltimaObservacion),
    Visible(DatosObservados),
}
struct PasoVisible {
    ficha: IdFichaVisible,
    desde: PosicionLocal,
    hasta: PosicionLocal,
}
```

Los IDs visibles no revelan el `IdActor` de un enemigo no identificado. La memoria
conserva la última observación, no el estado actual oculto de una puerta o una
ficha. Las áreas todavía desconocidas no revelan su matriz, dimensiones secretas
o habitantes. El cambio de área por un acceso entrega la vista que corresponda.

Cada tramo se proyecta en su orden causal antes de avanzar al siguiente estado
público. No se filtra toda una secuencia utilizando únicamente la visibilidad
final. Descubrir un guardia y crear un guardia son hechos diferentes; si un hecho
no fue observable no se inventa una señal visual que revele su procedencia.

Las revisiones y secuencias públicas son propias de la vista. No publican saltos
del contador interno que permitan deducir cuántos movimientos ocultos hubo.

## 18. Protocolo de reproducción

La app expone tipos de vista propios, por ejemplo:

```rust
struct SnapshotEncuentro {
    encuentro: IdEncuentro,
    revision: RevisionVista,
    ultima_secuencia: u64,
    areas: Vec<VistaArea>,
    fichas: Vec<VistaFichaLocal>,
    elementos: Vec<VistaElementoLocal>,
}
struct FrameEncuentro {
    secuencia: u64,
    desde: RevisionVista,
    hasta: RevisionVista,
    cambios: Vec<CambioVisible>,
    senales: Vec<SenalVisible>,
}
struct LoteEncuentro {
    id: IdLoteEncuentro,
    vineta: VistaVinetaEncuentro,
    revision_inicial: RevisionVista,
    secuencia_inicial: u64,
    frames: Vec<FrameEncuentro>,
    limite: LimiteLote,
}
enum LimiteLote {
    Continuar(TokenContinuacionEncuentro),
    Intervencion(IdIntervencion),
    Resumen(VistaResumenEncuentro),
    Decision(VistaDecisionEncuentro),
    Fin(VistaResultadoAtaque),
}
```

El snapshot se entrega al abrir la vista o al recuperar una reproducción, no
como una copia completa adjunta a cada frame o lote. Los lotes ordinarios parten
de la revisión y secuencia que ya conserva la terminal.

La revisión de vista es independiente de la revisión interna; los cambios
ocultos no publican saltos de contador ni revelan cuántos intentos se resolvieron.
Cada frame exige la secuencia pública siguiente exacta, incluso si su revisión
no cambia. Un snapshot establece tanto revisión como última secuencia. Se
rechazan duplicados, omisiones y lotes de otro encuentro.

Cambios iniciales: paso real de ficha, cruce de acceso, actualización de ficha, incorporación,
revelación, ocultación, retirada, terreno, acceso y traslado narrativo. Señales:
intento de ataque, fallo, acierto e interrupción. Todos son datos tipados; la
terminal no interpreta prosa para decidir qué sucedió.

Los cambios y señales de un frame son simultáneos a efectos de presentación;
si requieren orden causal, ocupan frames distintos. En particular, el ataque de
reacción y el paso que solo ocurre si falla nunca comparten frame.

Cada paso de una ruta visible es una casilla realmente atravesada. La reacción
se representa antes del movimiento y un acierto no lleva un paso posterior a un
destino que nunca se alcanzó. Una reubicación narrativa usa una transición
explícita, no una ruta sintética que aparenta haber activado amenazas.

Un frame contiene como máximo un paso por ficha, aunque varias fichas puedan
avanzar en el mismo frame si así se resolvió. Nunca comprime varios pasos de una
ficha hasta hacerla saltar visualmente sobre casillas intermedias.

Un frame aplica un grupo atómico de cambios compatibles. Sus cambios de ocupación
se aplican como grupo, no se validan como movimientos independientes en un orden
que invente colisiones. Frames sucesivos conservan el orden causal resuelto. Una señal sin modificación
de datos puede conservar la revisión pública y avanzar solo su secuencia.

El límite del lote es continuación, decisión, intervención, fin de viñeta o fin
de encuentro. Al agotarlo se confirma su consumo mediante un token vigente;
la app entonces puede producir el siguiente segmento. No se anticipan combates
posteriores a una decisión del jugador o a un evento pendiente.

El snapshot inicial y los frames de reproducción pueden ir por detrás del estado
interno ya confirmado del tramo actual. La vista en reproducción es la fuente del dibujo y de las consultas locales;
no se refresca desde el estado interno final para adelantar heridas o muertes.
Ninguna acción del jugador puede actuar sobre ese estado futuro mientras el lote
no haya terminado de mostrarse. Saltar
la reproducción aplica sus cambios en orden, sin ejecutar otra vez el motor.

La app conserva el snapshot de inicio y los frames del único lote pendiente
hasta que se confirme su lectura. Si se detecta una revisión o secuencia
incompatible a mitad de lote, se pausa la reproducción y se puede solicitar
`ReiniciarReproduccion` para ese lote vigente: devuelve el mismo inicio y los
mismos frames, sin ejecutar reglas, Lua ni azar. Se reinicia el cursor visual;
repetir una señal gráfica no repite su consecuencia mecánica.

La recuperación no intenta seguir aplicando deltas sobre un estado incompatible
ni entrega automáticamente el snapshot final que adelantaría revelaciones. El
jugador puede optar por omitir la reproducción recuperada; entonces sus cambios
se aplican en orden. La confirmación de lectura valida encuentro, lote y última
secuencia antes de habilitar la continuación. Solo hay un lote pendiente: no se
piden futuros lotes ni se ejecuta el evento del límite antes de confirmar el
actual. Una petición de un lote ya vencido se rechaza.

Al confirmar el límite puede publicarse el snapshot siguiente de la vista, con
su secuencia correspondiente. Se retiene como máximo el material del lote
vigente y la vista actual, no toda la historia gráfica del encuentro.

## 19. Fachada de Partida y controles

La app agrega `AccionDisponible::Atacar` y
`IntencionFrontend::ComenzarAtaque { objetivo }`, revalidando disponibilidad,
ubicación efectiva de actor y objetivo y su estado vivo al comenzar. Respeta las
restricciones generales existentes del personaje y su colectivo. Cancelar el
selector previo no inicia una acción ni consume tiempo.

`VistaSesion::Encuentro(VistaEncuentro)` representa la acción local, con estado
de reproducción, resumen o decisión. Se agregan intenciones semánticas para
confirmar lectura de lote, reiniciar su reproducción y responder decisión, con
encuentro e ID vigente. La confirmación de lectura incluye la última secuencia
y consume el token de continuación asociado al límite; su repetición se rechaza. No se obliga
al frontend a conocer `EstadoEncuentro` ni las operaciones internas.

Cuando un evento o diálogo ocupa la vista principal, la app conserva una vista
local de fondo consultable mientras el encuentro esté activo. El contrato de
diálogos usa los Actores identificados; no convierte cualquier ficha anónima en
un Actor. La vista de fondo se mantiene sincronizada en fronteras confirmadas.

La retirada durante una acción es una decisión del director en un punto permitido.
No es cancelar la ventana y liberar el tablero. Un gesto de retirada durante
una animación queda pendiente hasta el próximo punto habilitado, con indicación
visible. No se promete revertir hechos que ya se resolvieron en el lote.

## 20. Terminal: vista y reloj visual

Se incorpora un módulo de encuentro con cámara local y
`EstadoReproduccionEncuentro { snapshot, lote, cursor, pausa, velocidad }`.
Reutiliza primitivas de dibujo, paleta, fuentes alternativas y controles de
entrada. No reutiliza el dibujo completo del mundo con sus rutas comerciales,
rótulos, regiones, tráfico y niebla de relieve.

El tablero es una ventana local; el texto general de la viñeta ocupa una banda
estable y los resúmenes/opciones se presentan en pausas relevantes. Los detalles
no sustituyen el texto a cada paso. Los recursos de terreno y fichas se cargan
en frontend mediante un recurso `assets/terminal/encuentros.ron` y sus variantes
en las carpetas alternativas existentes. Se usan claves de apariencia y
fallback por recurso ausente, incluyendo una opción ASCII; ningún glifo tiene efecto
mecánico. Una apariencia de llama no concede propagación de incendio.

El reloj visual solo decide cuándo aplicar el siguiente frame público. Pausa,
velocidad, detalle, confirmación de salida y cambio de tamaño conservan cursor y
estado. Redimensionar recalcula cámara y recorte sin regenerar el escenario.
Si no cabe el área mínima de lectura se pausa la reproducción y no se piden más
lotes; se indica que debe ampliarse la ventana.

Omitir es un estado persistente de reproducción. En cada iteración del bucle
terminal consume como máximo un lote y solicita como máximo una continuación,
devolviendo control a entrada y dibujo antes de seguir. Se detiene ante resumen,
decisión o intervención; nunca atraviesa automáticamente una elección. No encadena
un bucle ilimitado de llamadas síncronas aunque cada llamada esté acotada. Un traslado entre áreas puede mostrarse con un cambio de cámara;
la visibilidad de otra área sigue filtrada por la app.

La futura selección de nivel será un control de cámara sobre las áreas/niveles
visibles; no modificará la posición de fichas ni retirará paredes mecánicas.

## 21. Trabajo acotado y rendimiento

La primera versión es síncrona y local. La app entrega snapshots y lotes de
datos Rust; no hay serialización por red ni envíos de matrices por fotograma.
El núcleo no espera milisegundos de animación.

Límites técnicos iniciales propuestos, revisables con el prototipo:

| Recurso | Cota inicial |
|---|---|
| Áreas generadas, incluidas adiciones de eventos | 8 |
| Dimensión de un plano | 64 × 40 casillas |
| Casillas totales | 16.384 |
| Fichas simultáneas | 64 |
| Elementos locales | 256 |
| Intentos de generación | 16 |
| Transiciones internas por llamada | 128 |
| Frames públicos por lote | 32 |

Las cotas limitan recursos y se validan antes de reservar memoria. No definen
alcance de armas, rapidez de personajes ni probabilidades de victoria. Si se
agota trabajo por llamada se devuelve continuación; no se declara una derrota.
Si un evento excede capacidad estructural, falla su preparación antes de mutar
la partida y se diagnostica el contenido.

Un presupuesto acota también búsqueda y planificación, conservando su trabajo
pendiente entre continuaciones. No se permite un bucle de regeneración o ruta
que bloquee la entrada indefinidamente. El director reconoce falta de progreso
y aplica la salida reglada que se acuerde; un timeout técnico no decide el final
de la historia. Se miden generación, resolución y dibujo por separado antes de
ampliar los escenarios. No se prometen tiempos de máquina sin medición.

## 22. Archivos y responsabilidades de implementación

| Archivo o módulo objetivo | Responsabilidad |
|---|---|
| `core/src/encuentros/{estado,espacio,generacion,comandos,registros}.rs` | Dominio local y confirmación de cambios. |
| `core/src/acciones/atacar/{mod,director,vinetas}.rs` | Ejecución del propósito Atacar. |
| `core/src/acciones/particulares/` | Reglas de intentos locales conforme al reglamento aprobado. |
| `core/src/acciones.rs` e `intervenciones.rs` | Envoltura por acción y puntos comunes/locales. |
| `core/src/modelo.rs` | Consecuencias mundiales necesarias y validaciones cruzadas. |
| `app/src/encuentros/{contenido,proyeccion,narracion}.rs` | Catálogo RON y vistas observables. |
| `app/src/sesion.rs` y `sesion/eventos_partida.rs` | Propiedad, intenciones, continuación y cierre de Partida. |
| `app/src/eventos/{contenido,lua,coordinador}.rs` | Nuevos puntos y efectos mixtos. |
| `app/src/fase_npc.rs` y `lua.rs` | Despacho de la nueva solicitud cuando exista su política; mismo motor sin reproducción gráfica. |
| `terminal/src/encuentro/` | Dibujo y reproducción local. |
| `terminal/src/interfaz.rs` y recursos visuales | Entrada y selección de vista; apariencias alternativas. |

Los prefijos `core`, `app` y `terminal` representan las crates actuales de la
workspace. Los módulos pequeños se crean cuando tengan responsabilidad real;
esta tabla no exige archivos vacíos ni abstracciones sin consumidor. Rescatar
podrá incorporar su director usando `encuentros`, sin copiar el tablero.

## 23. Reglas pendientes y condiciones para el plan jugable

El diseño de mecánicas todavía deja pendientes:

| Decisión de gameplay | Contrato técnico que depende de ella |
|---|---|
| Vecindad, diagonales y ocupación aliada/cuerpos | Predicado de paso y construcción de recorridos. |
| Orden, oportunidades y recuperación de reacciones | Agenda local y elegibilidad del ataque de reacción. |
| Golpe, defensa, heridas e incapacitación | Resolución particular y consecuencias persistentes. |
| Enfoques, detección y conocimiento de objetivos | Director, percepción y selector de blancos. |
| Duraciones, captura y destino del objetivo fugado | Acumulador temporal y cierre válido. |

Estas decisiones no impiden fijar IDs, propiedad, comandos, lotes, generación
acotada o integración de eventos. Sí deben cerrarse en las mecánicas antes de
habilitar el primer combate jugable. Este contrato no convierte las propuestas
de ronda interna o movimiento ortogonal en reglas aprobadas por omisión.

La confirmación de la reacción al abandonar cualquier casilla queda cerrada;
ninguna elección posterior de geometría la cambia por «solo abandonar alcance».

El contenido requiere una versión de reglamento reconocida y completa. Mientras
no exista, la app no habilita Atacar en la partida normal ni carga un perfil de
combate con valores inventados. Un desarrollo parcial podrá demostrar tablero y
reproducción con datos de prueba explícitos, sin presentarlo como el reglamento.

La [hoja de ruta de entregas](../../planes/encuentros-atacar-rescatar.md) ordena
incrementos verticales. El [primer plan](../../planes/pendientes/2026-11-19-47-atacar-episodio-minimo.md)
selecciona los contratos necesarios para un ataque mínimo completo y conserva
como precondiciones las mecánicas que todavía están abiertas. Terminar ese ciclo
no declara implementado todo el repertorio de comandos o escenarios de este
contrato. Cada capacidad conserva su estado real.

## 24. Verificación y revisión de cierre

La verificación será acotada a cada ampliación. Casos que deben cubrir los
contratos correspondientes al implementarse:

- Generación reproducible y conectada; fallo acotado; el mismo encuentro no cambia
  de mapa al repintar. Coordenadas de otro nivel no colisionan con el nivel base.
- Referencias locales vencidas, posiciones inválidas y Actor duplicado rechazados.
- Salida amenazada hacia otra casilla del mismo alcance activa reacción; entrada
  sola no. Fallo permite continuar; acierto cancela solo el paso pendiente.
- Cambios de terreno o reubicación invalidan rutas; lo confirmado no se repite.
- Operaciones mixtas fallidas conservan Mundo, tablero, tiempo, continuación y azar.
- Evento que abre una compuerta y coloca tigres; evento que cambia de habitación
  sin simular salto. Misma secuencia de diálogos y decisiones vigentes.
- Vistas sin terreno, identidades o posiciones ocultas; memoria sin actualización
  remota; revelación en su momento causal, no según el final del lote.
- Pausa, resize, omisión y resincronización sin repetir consecuencias ni alterar
  tiempo. Texto general estable durante los pasos de la viñeta.
- Objetivo muerto con salida pendiente; fuga; retirada; captura y muerte de
  participantes con persistencia y cierre idempotente.
- Viajar y sus eventos conservan comportamiento al ampliar la envoltura.

El episodio manual de referencia será el pequeño escenario acordado en las
mecánicas, con oposición en un paso estrecho y pocas fichas. Se revisarán lectura,
claridad de la hostilidad, continuidad narrativa y duración real percibida antes
de sumar familias, ataques a distancia o alturas. Este documento no crea todavía
un plan de tareas ni declara implementada ninguna de estas capacidades.
