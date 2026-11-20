# Rescatar personas — Primera implementación

| Campo | Valor |
|---|---|
| Estado | Implementado; pendiente revisión de jugabilidad |
| Fecha | 2026-11-20 |
| Base | [Encuentros y Atacar](./10-encuentros-y-atacar.md) |

## 1. Alcance y reglamento

Rescatar intenta alcanzar a una persona cautiva, soltarla y extraerla del escenario.
Comparte generación, movimiento simultáneo, combate, reacciones, heridas, retirada,
transporte de incapacitados y tiempo con Atacar. No exige matar al captor ni a todos
los defensores. Tampoco se completa al matar al jefe o llegar junto al cautivo.

El primer caso es Tamin, prisionero de Kazir en Lomas de Anir. Debe estar vivo,
fuera del grupo del jugador, en el mismo Lugar y con una única relación PrisioneroDe
hacia el captor declarado. El jugador debe estar libre, vivo, capaz y fuera de un
colectivo. Se ofrecen rescates definidos por contenido, no toda persona del mundo.
La primera versión no rescata a miembros de colectivos ni interpreta EsclavoDe.

El cautivo espera atado y no combate ni amenaza. Se coloca como ficha local
vinculada a su IdActor, cerca del fondo del escenario. Los rescatadores avanzan
hacia él y combaten a enemigos adyacentes si les salen al paso. Soltarlo requiere
contacto cercano y consume la intención ordinaria de un integrante capaz durante
un pulso. Si este queda incapacitado en el intercambio, no completa la liberación.
Dos aliados que intentan soltarlo simultáneamente no duplican el resultado.

Después de soltarlo, todos intentan retirarse con él. El rescatado coopera y camina;
si está incapacitado, puede ser cargado mediante las reglas existentes. Hasta
alcanzar la salida, su libertad es provisional dentro del encuentro. Puede ser
herido, morir o quedar atrás si la operación fracasa.

## 2. Resultados persistentes

Al alcanzar la salida vivo, el rescatado queda libre y se incorpora como compañero
para acompañar al grupo. No implica una promesa narrativa de lealtad permanente.
Se conserva su estado físico. La app confirma una sola vez la libertad, la relación
SalvadoPor hacia el actor jugador y la incorporación, junto con el paso del motor.

La relación PrisioneroDe hacia el captor se convierte en LiberadoDe, conservando su
IdRelacion y los índices de sus extremos. Es la memoria del último cambio de esa
relación, no un historial ilimitado. Una recaptura por el mismo captor reactiva la
misma relación como PrisioneroDe; un rescate posterior no duplica SalvadoPor. El
registro de cambios conserva los sucesos particulares. Se quita la característica
legada prisionero cuando ya no quedan relaciones de cautiverio activas.

- Éxito: el cautivo y los supervivientes logran salir.
- Retirada: los supervivientes salen sin completar la extracción.
- Captura o derrota: se usan los desenlaces del motor común, con heridas y pérdidas.
  Solo se registra captura para integrantes que no hayan salido. El objetivo que
  no logró escapar conserva su cautiverio original.
- Si el cautivo escapa pero otros quedan vencidos, su libertad ya confirmada no se
  revoca al cerrar el episodio. El resultado del resto del grupo se conserva.

La muerte del cautivo obliga a abandonar el rescate y buscar la salida. No se lo
resucita ni incorpora al grupo. Esta entrega no implementa sucesión del jugador,
prisioneros que rechazan la ayuda, tesoros, inventarios ni políticas NPC de rescate.

## 3. Contenido

`content/base/encounters.ron` agrega una lista opcional `rescates`, vacía por defecto:

```ron
rescates: [
    (actor: "tamin", captor: "jefe_bandido_kazir", escenario: Campamento,
     defensores: ["bandido_nazur", "bandido_hazek", "bandido_sufan"]),
],
```

Se validan claves, actores repetidos, identidad cautivo/captor y defensores
incompatibles; máximo ocho defensores adicionales. Se revalidan las condiciones
al iniciar y después de la intervención inicial. Participan el captor y defensores
vivos presentes; los muertos o ausentes se omiten. Es posible rescatar después de
haber derrotado a la banda con Atacar, incluso si murió el captor.

El perfil explícito o automático reutiliza las mismas familias de generación.
El cautivo no se suma a los defensores ni aparece como objetivo de Atacar por
estar presente. En perfiles con información oculta se revela cuando corresponde.

## 4. Organización del código

- Core: `Encuentro::nuevo_rescate` reutiliza el constructor común y marca el
  propósito mediante `rescate`. La ficha objetivo pertenece al bando aliado pero
  tiene `cautivo=true`, por lo que inicialmente no actúa. `Intento::Liberar` genera
  `Suceso::Liberado`. La salida del objetivo determina el éxito de la extracción.
- Mundo: `OperacionMundo::ConfirmarRescate` valida referencias y cautiverio, cambia
  la relación, registra SalvadoPor sin duplicados y limpia el rasgo legado. La
  incorporación usa IncorporarCompanero en el mismo lote preparado.
- App: se reutiliza el adaptador `Partida.ataque` y `AtaqueActivo` de los encuentros,
  con captor declarado y confirmación de extracción. No se copia el motor ni se
  refactoriza la envoltura pública histórica de Viajar en este ciclo.
- Fachada: `AccionDisponible::Rescatar` y `ComenzarRescate`; avances, retirada y
  cierre conservan los mismos tokens de encuentro. La vista añade `rescate` y
  distingue ficha cautiva y rescatada. Los IDs y pasos antiguos siguen rechazados.

La duración conserva diez minutos de preparación más seis segundos por pulso,
confirmados una sola vez al cerrar. Lectura y reproducción visual no añaden tiempo.
Las heridas y la extracción se confirman con sus pasos, no se repiten al cerrar.

## 5. Eventos e interfaz

`rescatar.inicio` admite la intervención inicial y su interrupción. Los puntos
locales de encuentro y `accion.terminada` reciben acción `rescatar` y objetivo Actor
cautivo. La modificación del grupo por extracción ocurre en el paso confirmado;
los eventos locales siguen sin poder alterar arbitrariamente su pertenencia.
Los guiones existentes del ogro no se reutilizan como relatos de un rescate.

Desde el mapa, R abre el selector de rescate; T conserva el de ataque. Esc cancela
la selección. Dentro del encuentro, R sigue significando retirada. El encabezado
muestra RESCATE y la persona se representa con @; los estados de herida o caída
conservan sus señales comunes. Enter/Espacio pausa, D muestra detalles y Enter
cierra el resultado. Al salir, la persona liberada aparece como compañero.

La cooperación, la liberación de cadenas, la extracción de un esclavo y las
consecuencias particulares de rescatar a alguien importante podrán ampliarse con
reglas y eventos de autor. No se incorporan esos casos por asumir que todo rescate
es idéntico al de Tamin.
