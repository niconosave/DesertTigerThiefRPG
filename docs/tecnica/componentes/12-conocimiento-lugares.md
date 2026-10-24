# Conocimiento de Lugares

| Campo | Valor |
|---|---|
| Estado | Aprobado para implementar |
| Versión | 1.0 |
| Fecha | 2026-10-17 |

## 1. Propósito y alcance

Este documento define el primer conocimiento concreto de un Actor sobre el
Mundo: la Relación dirigida `Conoce` desde un Actor hacia un Lugar. Su propósito
inicial es permitir que la planificación y las acciones particulares de viaje
usen lo que saben los viajeros sin revelar automáticamente todos los hechos
reales del recorrido.

El alcance incluye:

- autoría de `Conoce` dentro del mismo RON del Actor;
- resolución, validación, almacenamiento e índices;
- consultas individuales y conocimiento conjunto de los viajeros;
- proyección de hechos conocidos para estimar acciones;
- interacción con la capacidad general `conoceDesiertos`.

No incluye memoria adquirida durante una Partida, olvido, grados numéricos,
rumores falsos, ocultar los nombres o conexiones del mapa, ni consultas de
Relaciones desde Lua.

## 2. Semántica de `Conoce`

`Conoce(actor, lugar)` afirma que ese Actor posee experiencia específica y
fiable del Lugar suficiente para reconocer sus hechos estables pertinentes a
una acción. Es una Relación dirigida:

```text
Actor --Conoce--> Lugar
```

No existe la lectura inversa “el Lugar conoce al Actor”. La Relación:

- une exactamente un Actor de origen con un Lugar de destino;
- exige que ambos existan en el mismo Mundo;
- prohíbe el mismo par duplicado;
- no posee intensidad, fecha, texto ni propiedades propias;
- no modifica el Lugar ni garantiza el éxito de una acción;
- permanece inmutable durante este incremento.

Para una acción del jugador, el conocimiento disponible es la unión del actor
jugador y todos sus compañeros. Para un NPC independiente solo cuenta el
conocimiento de ese NPC. Saber un Lugar no separa al Grupo ni convierte al
conocedor en responsable automático: cada acción particular decide qué Actor
realiza el intento y qué aporte admite.

`conoceDesiertos` continúa siendo una Característica general del Actor. Puede
ayudar en cualquier Lugar cuyo entorno físico sea desértico; `Conoce` aporta
conocimiento específico del Lugar relacionado. Cada ficha particular determina
cuál de los dos hechos es pertinente y aplica como máximo un aporte principal
de conocimiento, conforme al
[contrato común](../acciones/particulares/00-contrato-resolucion.md#5-dificultad-y-competencia-contextuales).

## 3. Visibilidad y hechos conocidos

En esta versión, la existencia, el nombre visible y las conexiones geográficas
de todos los Lugares del atlas permanecen disponibles para seleccionar un
destino. `Conoce` no convierte el mapa en niebla de guerra.

Sin conocimiento específico, una estimación puede usar:

- el nombre y la existencia del Lugar;
- que está conectado con sus vecinos;
- hechos que la presentación ya haya hecho públicos;
- supuestos básicos de la acción, marcados como inciertos.

Con `Conoce`, la estimación puede usar las Características estables del Lugar
que su ficha considere observables mediante experiencia: terreno, fuentes,
refugios, recursos, costa o accidentes geográficos. Un estado oculto o
recientemente modificado no se vuelve conocido por conservar la Relación; cada
acción decide si el cambio sería perceptible o requiere observación nueva.

La evaluación autoritativa siempre consulta el estado real. La estimación pura
solo recibe una proyección de hechos conocidos y nunca indica cuántos factores
ocultos existen.

## 4. Modelo e índices en Rust

`TipoRelacion` añade `Conoce`. Se conservan `Relacion`, `IdRelacion` y los
índices incidentes definidos por el
[componente de Relaciones](./10-relaciones.md). El Mundo añade un índice
especializado equivalente a:

```rust
pub struct Mundo {
    // Campos existentes.
    lugares_conocidos_por_actor: Vec<Vec<IdLugar>>,
}

impl Mundo {
    pub fn lugares_conocidos(
        &self,
        actor: IdActor,
    ) -> Option<&[IdLugar]>;

    pub fn actor_conoce_lugar(
        &self,
        actor: IdActor,
        lugar: IdLugar,
    ) -> Result<bool, ErrorConsultaRelacion>;
}
```

Los nombres exactos pueden ajustarse al implementar sin cambiar el contrato.
La lista de cada Actor contiene IDs únicos y ordenados por `IdLugar`; por eso la
consulta no recorre la colección global de Relaciones ni depende del orden del
filesystem. La consulta conjunta de una acción une las listas de sus viajeros
en orden canónico y elimina duplicados.

La identidad usada para detectar duplicados es
`(Conoce, IdActor, IdLugar)`. No se normalizan los extremos porque la Relación es
dirigida.

## 5. Formato RON y autoría

`Conoce` se declara en `relaciones` dentro del archivo del Actor que posee el
conocimiento:

```ron
relaciones: [
    (
        tipo: Conoce,
        destino: Lugar("valle_fertil"),
    ),
],
```

La variante exacta se añade a `TipoRelacionDto`; el destino reutiliza
`Lugar(String)`. Son inválidos:

- `Conoce` declarado dentro de un Lugar;
- destino `Actor(...)`;
- Actor o Lugar desconocido;
- par Actor–Lugar repetido;
- campos adicionales o una variante con grafía diferente.

El cargador conserva el diagnóstico junto al archivo, Actor y posición de la
declaración. El núcleo repite las invariantes y no publica un Mundo parcial.

## 6. Uso por acciones y planificación

Una acción particular no consulta `Conoce` de manera improvisada. Su ficha
declara si el conocimiento específico modifica:

- los factores del intento autoritativo;
- los hechos disponibles para su estimación;
- ambos.

La planificación de `Viajar` solicita estimaciones a las acciones particulares
con la proyección conjunta de los viajeros. No añade por su cuenta un bono por
`Conoce` ni inspecciona todas las Características del Lugar. Esto asegura que
`Orientarse`, `BuscarAgua`, `Cazar` o `Navegar` interpreten el mismo conocimiento
también cuando otra acción general las invoque.

## 7. Errores e invariantes

Errores mínimos:

- origen o destino desconocido;
- combinación de extremos incompatible;
- declaración duplicada;
- ID no representable;
- incoherencia entre Relaciones e índice especializado.

Invariantes:

- cada `Conoce` aparece una sola vez en la colección canónica;
- el índice del Actor contiene el Lugar exactamente una vez;
- la dirección declarada nunca se invierte;
- una consulta inválida no se confunde con una respuesta negativa;
- estimar no muta, no consume azar y no revela hechos no proyectados;
- jugador y NPC usan las mismas consultas;
- tiles y coordenadas nunca producen conocimiento reglado.

## 8. Pruebas y criterios de aceptación

Se verificarán:

- carga válida de `Conoce` desde un Actor;
- rechazo desde un Lugar, hacia un Actor, por referencia inexistente y por
  duplicado;
- una única Relación e índice especializado coherente;
- consultas positivas, negativas e IDs inválidos;
- orden canónico y unión sin duplicados del conocimiento del Grupo;
- diferencia entre estimación sin conocimiento y con `Conoce`;
- ausencia de filtración de una Característica no conocida;
- aporte general de `conoceDesiertos` sin inventar una Relación;
- paridad entre una acción del jugador y una de NPC.

## 9. Decisiones aplazadas

Se aplazan adquirir o perder conocimiento durante la Partida, conocimientos
sobre Actores u otros elementos, rumores, información falsa, niveles de dominio,
niebla de guerra, cartografía creada por el jugador y acceso de Lua al grafo de
Relaciones.
