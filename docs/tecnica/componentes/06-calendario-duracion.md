# Calendario y duración del mundo

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.3 |
| Fecha | 2026-10-14 |

## 1. Propósito y alcance

Este documento es la fuente canónica para la fecha del Mundo, las duraciones de
acciones y el único avance temporal que sigue a una acción del jugador.

El alcance inicial debe permitir que `Viajar` acumule una duración, que la fecha
avance una vez después de cerrar su última viñeta y que la fase NPC observe la
fecha resultante. No define reglas de `Viajar`, orden de NPC ni reloj visual.

## 2. Responsabilidades

`desert-tiger-core` define y valida los tipos de fecha y duración, y realiza su
suma. La única fecha vigente forma parte de `Mundo`.

`desert-tiger-app` solicita el avance una sola vez cuando la fachada alcanza el
punto establecido por el
[contrato de sesión](./04-sesion-aplicacion.md). El frontend no calcula fechas.

## 3. Modelo confirmado

- el calendario es gregoriano, con sus doce meses, longitudes normales y años
  bisiestos;
- una duración de gameplay se expresa inicialmente en días y meses, nunca en
  segundos reales;
- las acciones particulares pueden añadir cantidades codificadas en Rust a la
  duración total de una acción general;
- una duración nunca es negativa;
- la fecha no cambia mientras la acción del jugador está pendiente;
- la duración total se aplica una sola vez cuando la acción se completa y se
  alcanza el punto visible acordado;
- las duraciones informadas por acciones NPC no modifican la fecha;
- el reloj monotónico del frontend para animaciones es independiente.

Los tipos públicos de valor pertenecen al módulo `calendario` de
`desert-tiger-core`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fecha {
    anio: u16,
    mes: Mes,
    dia: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duracion {
    meses: u32,
    dias: u32,
}
```

`Mes`, `Fecha` y `Duracion` son valores copiables, comparables e inmutables desde
el exterior. Los campos privados impiden construir una `Fecha` sin validar. El
año válido está limitado a `1..=9999`; no existe año cero. `Duracion` admite cero
en ambos componentes porque toda acción comienza con un acumulador vacío.

Un año es bisiesto si es divisible por `4`, excepto cuando es divisible por
`100` sin serlo por `400`. Por tanto, febrero tiene 29 días en 2000 pero 28 en
1900. Los demás meses conservan las longitudes gregorianas habituales.

## 4. API requerida

La API mínima será:

```rust
impl Mes {
    pub fn numero(self) -> u8;
    pub fn desde_numero(numero: u8) -> Result<Self, ErrorCalendario>;
}

impl Fecha {
    pub fn nueva(anio: u16, mes: Mes, dia: u8)
        -> Result<Self, ErrorCalendario>;

    pub fn anio(self) -> u16;
    pub fn mes(self) -> Mes;
    pub fn dia(self) -> u8;

    pub fn sumar(self, duracion: Duracion)
        -> Result<Self, ErrorCalendario>;
}

impl Duracion {
    pub const CERO: Self;

    pub const fn nueva(meses: u32, dias: u32) -> Self;
    pub const fn desde_dias(dias: u32) -> Self;
    pub const fn desde_meses(meses: u32) -> Self;
    pub const fn meses(self) -> u32;
    pub const fn dias(self) -> u32;
    pub const fn es_cero(self) -> bool;

    pub fn checked_add(self, otra: Self)
        -> Result<Self, ErrorCalendario>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCalendario {
    AnioFueraDeRango {
        anio: u16,
    },
    MesFueraDeRango {
        mes: u8,
    },
    DiaFueraDeRango {
        anio: u16,
        mes: Mes,
        dia: u8,
        maximo: u8,
    },
    Desbordamiento,
}
```

`ErrorCalendario` implementa además `std::fmt::Display` y
`std::error::Error`. Su texto describe la categoría y los valores implicados sin
constituir una API estable; el código consumidor decide cómo contextualizarlo
para un diagnóstico.

`Mes::numero` devuelve `1` para enero y `12` para diciembre. La conversión
inversa rechaza cero y todo valor mayor que doce. `Fecha::nueva` rechaza el año
cero, años superiores a 9999 y días fuera del intervalo válido para ese mes y
año. Los getters no permiten modificar campos por separado.

`Duracion::checked_add` suma meses con meses y días con días mediante aritmética
comprobada. No convierte días en meses, porque la longitud de un mes depende de
la fecha donde se aplique. Un desbordamiento de cualquiera de los dos `u32`
devuelve `ErrorCalendario::Desbordamiento` y conserva ambos operandos.

### Suma de una duración a una fecha

`Fecha::sumar` aplica una duración en este orden normativo:

1. suma el componente `meses` al par año-mes;
2. si el día original no existe en el mes resultante, lo limita al último día
   válido de ese mes;
3. suma el componente `dias` avanzando por el calendario gregoriano desde la
   fecha ya ajustada.

Así, `31/01/2025 + 1 mes` produce `28/02/2025`, mientras que en 2024 produce
`29/02/2024`. Si además la duración contiene un día, ambos casos terminan el
`01/03` del año correspondiente. Una duración de cero devuelve la misma fecha.

El componente completo de meses se aplica en una sola operación; no se repite
una suma de un mes con un ajuste intermedio por cada unidad. Por ejemplo,
`31/01/2025 + 2 meses` produce `31/03/2025`, no `28/03/2025`. En consecuencia,
la aplicación acumula primero todos los aportes en una sola `Duracion` y aplica
esa duración una vez.

La implementación debe calcular con enteros intermedios suficientemente amplios
y detectar el resultado fuera de `1..=9999` antes de construirlo. No recorrerá
un bucle por cada día cuando la duración sea grande. Cualquier resultado que
superaría el 31 de diciembre de 9999 devuelve `Desbordamiento`.

### Confirmación en el Mundo

La operación semántica del Mundo será:

```rust
impl Mundo {
    pub fn fecha(&self) -> Fecha;

    pub fn avanzar_fecha(
        &mut self,
        duracion: Duracion,
    ) -> Result<Fecha, ErrorCalendario>;
}
```

`avanzar_fecha` calcula primero el resultado completo con `Fecha::sumar`. Solo
si tiene éxito reemplaza la fecha del Mundo y devuelve el nuevo valor. Un error
deja la fecha anterior intacta. En el incremento inicial, la aplicación llama a
esta operación una sola vez con la duración de `Viajar` completada por el
jugador, después de que se confirme su última viñeta. La fase NPC conserva las
duraciones calculadas pero nunca llama a esta operación.

## 5. Invariantes y errores

- La fecha tiene una única fuente de verdad en `Mundo`.
- Mes y día siempre forman una fecha válida conforme al calendario aprobado.
- El año siempre pertenece a `1..=9999`.
- Meses y días de una duración se conservan por separado y nunca son negativos.
- Duración y fecha no pueden desbordar sus tipos.
- Un rechazo o una acción todavía pendiente no adelantan la fecha.
- Una acción completada del jugador produce como máximo un avance.
- Una acción NPC nunca produce otro avance.

Un componente inválido o una suma imposible se devuelve como
`ErrorCalendario`. Un desbordamiento al acumular la duración dentro del motor se
convierte en `FalloMotor::DesbordamientoDuracion`. Un error al avanzar la fecha
después de completar la acción es fatal para la sesión: no se presenta como un
rechazo de gameplay ni inicia la fase NPC.

## 6. Pruebas y aceptación

Las pruebas cubrirán:

- conversión exacta entre los doce valores de `Mes` y `1..=12`;
- rechazo de los números de mes `0` y `13`;
- rechazo de años `0` y superiores a `9999`;
- construcción de cada último día válido y rechazo tanto del día cero como del
  día posterior al máximo;
- febrero de 1900 con 28 días y febrero de 2000 y 2024 con 29;
- suma de días en límites de mes y año, incluido
  `31/12/9999 + 1 día -> Desbordamiento`;
- suma de meses que conserva el día cuando existe y lo limita cuando no existe;
- cruce de año al sumar meses y desbordamiento por meses cerca del año 9999;
- `31/01/2025 + 2 meses = 31/03/2025`, sin ajuste mensual intermedio;
- orden meses-antes-de-días mediante casos donde invertirlo daría otro resultado;
- conservación separada de meses y días, sin normalización implícita;
- suma comprobada de duraciones y desbordamiento individual de cada componente;
- suma que intentaría superar el año 9999 sin modificar la fecha original;
- duración cero que conserva exactamente la fecha;
- duración acumulada por varias acciones particulares;
- avance atómico y único de la acción del jugador;
- ausencia de avance por una acción NPC.

Una integración sin terminal verificará:

```text
fecha inicial
→ Viajar acumula duración
→ última viñeta todavía no cambia la fecha
→ continuar confirma un único avance
→ fase NPC observa la fecha nueva
```

## 7. Preguntas bloqueantes

Ninguna en este componente. La fecha inicial se declara obligatoriamente en
`world.ron`; su forma externa pertenece al documento de contenido. Las duraciones
concretas de `Viajar` permanecen únicamente en su propio documento.

## 8. Decisiones aplazadas

Horas, minutos, estaciones, zonas horarias y sincronización con tiempo real no
forman parte del alcance inicial. También se aplazan calendarios alternativos y
la presentación localizada de nombres de meses. Una acción futura podrá terminar
con duración cero sin cambiar la fecha, pero su efecto sobre el disparo de la
fase NPC deberá decidirse al diseñar esa acción; `Viajar` nunca dura cero.
