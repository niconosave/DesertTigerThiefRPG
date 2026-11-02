# Atlas regional de trabajo

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.19 |
| Fecha | 2026-10-29 |

## 1. Propósito

Este documento conserva la colección implementada de Lugares basada en las formas de
`assets/map.png` y en su traducción inicial de `96 × 32` tiles, ampliada al
doble de ancho y alto (`192 × 64`) el 2026-10-25. Su objetivo es dar una
geografía reconocible a los viajes y preparar contextos para futuras reglas sin
convertir todavía cada idea en una mecánica.

Los nombres fueron aprobados como primera versión del atlas. Las claves,
coordenadas exactas, Características y Relaciones tienen su única fuente
ejecutable en `content/base/`; este documento conserva la intención geográfica
y creativa sin duplicar esos datos.

La ampliación duplica cada tile en ambas direcciones y escala los centros de
los Lugares por el mismo factor. Cada rótulo se traslada hasta su nueva región
conservando el recorrido compacto de sus letras, sin insertar espacios entre
ellas. Se mantienen las formas geográficas y las Relaciones del atlas.

## 2. Lectura del mapa

La referencia y la matriz permiten distinguir con suficiente claridad:

- una gran región árida occidental, con costa, llanos, una elevación al norte y
  una península al sur;
- una cordillera central casi vertical y un segundo ramal montañoso hacia el
  sudeste;
- una playa interior y un golfo meridional entre ambas masas terrestres;
- un valle fértil nororiental atravesado por agua;
- un bosque oriental y tierras húmedas junto a la desembocadura del sudeste;
- una isla al sudoeste y mar abierto alrededor de la región.

Los tiles ayudan a reconocer estas formas, pero no crean Lugares ni Relaciones.
Toda conexión continuará declarándose de manera explícita.

## 3. Criterio para convertir geografía en Lugares

Un Lugar de este atlas debe ser una posición lógica en la que un Actor pueda
estar y desde la cual tenga sentido comenzar una acción. No se crea un Lugar por
cada detalle dibujado.

El modelo actual no expresa contención: un Actor no puede encontrarse a la vez
en una macrorregión y en un Lugar contenido por ella. Por eso el atlas adopta
estas decisiones:

- sustituir el nombre visible provisional `Zona Árida` por `Hamada Roja`, sin
  conservar ambos como Lugares superpuestos;
- sustituir `Valle Fértil` por `Valle del Drâa`;
- mantener `Paso Escarpado` como el tercer Lugar del recorrido inicial;
- representar pueblos, fortalezas y puertos como Lugares propios conectados con
  su entorno, no como una segunda ubicación simultánea.

Las Características enumeradas son vocabulario editorial en `lowerCamelCase`,
no una copia de los campos RON.
Solo adquieren efecto cuando una acción aprobada decide interpretarlas. Las
fichas técnicas vigentes de movimiento, supervivencia y navegación son la fuente
de ese significado; este atlas no mantiene una segunda lista de cuáles ya
poseen regla.

## 4. Colección terrestre aprobada

### Occidente árido

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Costa Ámbar | Costa arenosa del borde occidental. | `costero`, `arenoso`, `ventoso` | Exposición, restos arrastrados por el mar, embarque futuro. |
| Mesa del Sol | Elevación árida del extremo norte. | `arido`, `elevado`, `expuesto` | Ascenso, observación y señales a distancia. |
| Hamada Roja | Desierto rocoso del centro occidental; reemplaza visualmente a `Zona Árida`. | `arido`, `rocoso`, `expuesto` | Punto inicial, rastreo, calor y escasez de refugio. |
| Hamada de Zahrun | Franja de territorio árido y rocoso entre la cordillera y Mar del Viento; reemplaza al antiguo Camino de los Mojones. | `arido`, `rocoso`, `abierto` | Desplazamientos por el piedemonte y reconocimiento del terreno. |
| Cabo Errante | Península árida del sudoeste. | `costero`, `rocoso`, `ventoso` | Rodeos, campamentos expuestos y temporales. |
| Playa Dorada | Franja de arena en la costa del golfo interior. | `costero`, `arenoso`, `resguardado` | Descanso, búsqueda y acceso marítimo futuro. |

### Cordilleras y pasos

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Espinazo Gris | Cordillera principal, estrecha y alargada de norte a sur. | `montanoso`, `elevado`, `empinado` | Escalada, desprendimientos, visibilidad y clima. |
| Paso Escarpado | Cruce estrecho entre la Hamada y el oriente fértil. | `empinado`, `estrecho` | Complicación ya diseñada para `Viajar`. |
| Valle Gris | Valle con vegetación al pie de Alcázar Gris; reemplaza a Sierra Quebrada. | `fertil`, `resguardado` | Paso por el fondo del valle y refugio bajo las montañas. |

### Oriente fértil

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Valle del Drâa | Valle fértil del nordeste; reemplaza visualmente a `Valle Fértil`. | `fertil`, `fluvial`, `resguardado` | Descanso, ayuda, recursos y crecidas. |
| Bosque oscuro | Sector umbrío dentro de la región Bosque de Tamrut. | `boscoso`, `cerrado`, `humedo` | Orientación, caza, ocultamiento y encuentros. |
| Marisma Verde | Tierras bajas y húmedas del sudeste. | `pantanoso`, `humedo`, `fluvial` | Demoras, vadeo, enfermedades y rastros difíciles. |

### Lugares civilizados

En el mapa se usarían solamente sus nombres propios cortos. Su naturaleza queda
en este atlas y, cuando haga falta para una regla, en Características.

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Tazar | Pueblo del Valle del Drâa. | `civilizado`, `poblado`, `acogedor` | Descanso, reclutamiento, ayuda y rumores. |
| Alcázar Gris | Fortaleza en la elevación montañosa sobre Valle Gris. | `civilizado`, `fortificado`, `controlado` | Permisos, asilo, engaño y conflicto político. |
| Nahal | Puerto pequeño en el borde de la marisma. | `civilizado`, `portuario`, `costero` | Comercio y punto de partida para navegación futura. |

La colección contiene quince Lugares terrestres. Es una cantidad suficiente
para producir rutas y regiones distintas sin llenar por completo el mapa ni
hacer inmanejable el selector inicial de destinos.

## 5. Lugares marítimos e insulares

El mar también se divide en Lugares elegibles como destino. El contrato vigente
de [`Viajar`](../tecnica/acciones/02-viajar-expediciones.md#7-medios-por-etapa)
requiere acceso cualitativo a bote o barco con procedencia concreta. En el
contenido inicial, un muelle o puerto puede declarar un servicio local rutinario
ya disponible para iniciar la fase; la deliberación lo reconoce como aportación
del Lugar y no como posesión del Actor. Una costa sin ese acceso no presupone una
embarcación.

Una Relación `Conectado` entre tierra y agua permite intentar embarcar o
desembarcar cuando existe el medio;
entre dos Lugares marítimos representa navegar de una región acuática a otra.
todos continúan siendo Lugares ordinarios del mismo grafo.

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Mar del Viento | Entrada de mar al noroeste interior, reemplaza al antiguo Llano del Viento. | `maritimo`, `aguaSalada`, `viento` | Navegación entre Hamada Roja, Hamada de Zahrun, Costa Ámbar y Aguas de Ámbar. |
| Aguas de Ámbar | Mar abierto al oeste de la masa árida. | `maritimo`, `abierto`, `ventoso` | Temporales, pérdida de rumbo y llegada a la isla. |
| Mar Boreal | Aguas del borde norte del mapa. | `maritimo`, `abierto`, `profundo` | Corrientes, niebla y navegación costera. |
| Aguas de Tamrut | Mar que bordea el bosque y la marisma por el este. | `maritimo`, `abierto`, `humedo` | Tormentas, navegación junto a la costa y acceso a Nahal. |
| Golfo del Siroco | Cuenca marítima situada al sur entre ambas masas terrestres. | `maritimo`, `costero`, `ventoso` | Navegación protegida o peligrosa según el viento, pesca y acceso a Playa Dorada. |
| Isla del Exilio | Isla árida del sudoeste. | `insular`, `aislado`, `arido` | Exploración, rescate y refugios difíciles de alcanzar. |

No se añade además un `Mar Austral`: el Golfo del Siroco ocupa la región
meridional útil y otro rótulo saturaría esa parte del mapa. El río visible se
considera por ahora contexto cartográfico y fluvial del Valle del Drâa, el
Bosque oscuro y la Marisma Verde. Podrá convertirse en uno o varios Lugares
cuando una acción diferencie vados, confluencias o navegación fluvial.

## 6. Estructura creativa de la red

El esquema aprobado organizó la topología en varios circuitos sin guardar pesos,
medios de transporte ni una ruta elegida manualmente en las Relaciones:

```text
Circuito occidental:
Costa Ámbar — Llano del Viento — Mesa del Sol — Espinazo Gris
     │                                                │
     └──────────── Hamada Roja — Paso Escarpado ─────┘

Circuito meridional:
Hamada Roja — Cabo Errante — Playa Dorada — Sierra Quebrada
      │                                             │
      └────────────── Paso Escarpado ──────────────┘

Circuito oriental:
Paso Escarpado — Valle del Drâa — Bosque oscuro — Sierra Quebrada
                       │                 │
                     Tazar         Marisma Verde — Nahal

Alcázar Gris se conecta con Paso Escarpado y Sierra Quebrada.

Circuito marítimo:
Aguas de Ámbar — Mar Boreal — Aguas de Tamrut — Golfo del Siroco
       │                                             │
       └────────────── Isla del Exilio ─────────────┘

Accesos costeros:
Costa Ámbar — Aguas de Ámbar
Mesa del Sol — Mar Boreal
Nahal — Aguas de Tamrut
Playa Dorada — Golfo del Siroco
Cabo Errante — Golfo del Siroco
```

El corredor oriental implementado por el brief de
[trayectos de viaje](./trayectos-viaje.md) es:

```text
Hamada Roja → Hamada de Zahrun → Paso Escarpado
→ Descenso del Drâa → Valle del Drâa → Senda de Tamrut
→ Bosque oscuro → Vado de Tamrut → Nahal → Aguas de Tamrut
```

La Calzada de Nahal fue retirada; Marisma Verde ya no enlaza con Nahal por
ese corredor. El tramo marítimo continúa desde Nahal:

```text
Nahal → Estuario de Nahal → Marisma Verde → Bosque de Ilun → Valle de Azen
```

Las conexiones concretas de toda la red y la orientación de autoría de cada
declaración `Conectado` ya pertenecen únicamente a los RON. Las rutas se eligen
mediante las estimaciones particulares y la estrategia del plan adoptado; el
frontend no administra conexiones ni intenta reconstruirlas desde el mapa.

## 7. Rótulos adaptados a la geografía

El formato actual ya permite que un nombre siga la forma de un accidente. El
carácter situado en la posición `n` del nombre se dibuja en
`coordenadas_rotulo[n]`; las coordenadas no tienen que formar una línea recta.
No hace falta cambiar el esquema RON ni el frontend.

La convención editorial vigente es:

- mantener cada letra en su orientación normal: no se rota ningún glifo;
- usar el orden de `coordenadas_rotulo` para trazar el recorrido de lectura del
  nombre;
- permitir recorridos horizontales, verticales, diagonales o curvos según la
  forma geográfica;
- colocar normalmente los caracteres consecutivos en celdas vecinas y evitar
  saltos o cruces que vuelvan ambiguo el orden;
- suavizar los cambios de dirección y comprobar que una palabra vertical o
  curvada siga siendo legible;
- situar el centro del Lugar cerca del centro visual del rótulo, pero fuera de
  sus coordenadas;
- usar nombres propios cortos para los tres asentamientos;
- revisar manualmente superposiciones y recortes de cámara, sin agregar un
  validador que intente corregir decisiones artísticas.

Las coordenadas exactas pertenecerán únicamente a cada archivo de Lugar. Antes
de aprobar un lote se comprobará visualmente en la terminal que el rótulo sigue
la forma pretendida, continúa legible al enfocarlo y no queda oculto de manera
problemática por los retratos.

La composición marítima de trabajo reserva zonas distintas: `Aguas de Ámbar`
en el agua superior izquierda, `Mar Boreal` en la franja superior central,
`Aguas de Tamrut` mediante un recorrido principalmente vertical junto al borde
oriental y `Golfo del Siroco` mediante una curva suave sobre la cuenca del sur.
Así los nombres describen la forma del agua sin necesitar rotar sus letras.

Para nombrar lugares marítimos se priorizan accidentes geográficos que encajen
con su forma: bahía, ensenada, golfo, canal o estrecho. Cuando no corresponda
uno de ellos, se usa «Aguas de…» con el nombre de la costa o región terrestre
adyacente, como Ámbar o Tamrut. Se conserva el nombre propio Mar del Viento
para la entrada de agua casi rodeada de tierra, y Golfo del Siroco para la
amplia entrada meridional.

## 8. Historial de incorporación

El atlas se incorporó de manera incremental para acompañar el crecimiento de
`Viajar`:

1. PLAN-2026-010 y PLAN-2026-011 establecen el recorrido mínimo Hamada
   Roja–Paso Escarpado–Valle del Drâa;
2. PLAN-2026-012 incorpora juntos los demás Lugares, circuitos y rótulos como
   base geográfica observable;
3. los incrementos posteriores empezaron a añadir situaciones que interpretan
   su naturaleza a medida que se aprueban sus reglas.

Cada ampliación del lote geográfico debe terminar con contenido jugable. Puede
conservar hechos descriptivos todavía no interpretados, pero no puede atribuirles
un efecto reglado inexistente.

## 9. Decisiones creativas aprobadas

Se aprobaron para esta primera versión:

- el tono de los nombres de trabajo;
- los reemplazos visibles `Zona Árida` → `Hamada Roja` y `Valle Fértil` →
  `Valle del Drâa`;
- la selección de quince Lugares continentales, cuatro marítimos y uno insular;
- la posición narrativa de Tazar, Alcázar Gris y Nahal;
- la legibilidad del conjunto completo de rótulos sobre el mapa.

## 10. Regiones cartográficas

Se mantienen trece regiones como Lugares sin conexiones transitables, con
Relaciones Abarca hacia los Lugares existentes. Cubren las tierras áridas,
ambas cordilleras, la cuenca fértil, el bosque, las tierras bajas,
la isla y los cuatro sectores marítimos. El Descenso del Drâa pertenece tanto
al conjunto montañoso como a la cuenca: estas agrupaciones pueden solaparse.

Los nombres son provisionales y editables, escritos en mayúsculas. En las
regiones grandes se separan las coordenadas de las letras para extender el
rótulo sobre su territorio; en regiones pequeñas se conserva una composición
compacta. El espaciado se edita en los RON, sin insertar letras o espacios
adicionales en el nombre. Sus rótulos siguen recorridos
horizontales, diagonales o verticales acordes a cada región y se presentan con
menor intensidad que los Lugares transitables. Su autoría concreta —nombres,
coordenadas y pertenencias— reside únicamente en `content/base/places/`.
El [contrato de Regiones y Abarca](../tecnica/componentes/10-relaciones.md#12-regiones-y-abarca--implementadas-el-2026-10-28)
define su significado. Se retiró Costas de Sahil: Costa Ámbar queda sin región,
y Cabo Errante y Playa Dorada pertenecen a Península de Sahir. Las conexiones
transitables vigentes se declaran únicamente en los RON.

La región antes llamada Tierras de Tamrut se denomina ahora Bosque de Tamrut.
Bosque oscuro identifica el Lugar transitable dentro de esa región.


### Ampliación de Sahran y cabeceras fluviales

Península de Sahir agrupa Cabo Errante, Playa Dorada y la nueva Punta de Namar.
Tierras de Zahrun agrupa Hamada de Zahrun y Mesa del Sol. Los cuatro lugares
preexistentes dejan de estar abarcados directamente por Tierras de Sahran.
Lomas de Anir ocupa la tierra al noreste de Costa Ámbar y pertenece a Sahran.

Monte Tazir, Pico de Izir y Risco de Adrar pertenecen a la Cordillera de Azrum;
los tres se conectan con Espinazo Gris. Monte Tazir y Pico de Izir también se
conectan entre sí; Risco de Adrar no tiene conexión directa con ellos. Punta de Namar conecta con
Cabo Errante y Playa Dorada; Lomas de Anir conecta con Costa Ámbar.

Río Tazir nace en Monte Tazir y Río Izir en Pico de Izir; ambos son afluentes de
Río Tamrut, de mayor caudal, que atraviesa el bosque. Estos vínculos se declaran
con `OrigenDe` y `AfluenteDe`. La característica `caudaloso` describe el tercer
río; no añade reglas nuevas. Los tres ríos siguen sin conexiones de viaje.

Los límites geográficos se expresan con `Bloquea`: Espinazo Gris frente a Hamada
de Zahrun, Risco de Adrar frente a Hamada Roja, Alcázar Gris frente a Valle Gris
y Monte Tazir frente a Valle del Drâa. Estos vínculos no modifican el tránsito
actual; su contrato se define en
[Relaciones, sección 13](../tecnica/componentes/10-relaciones.md#13-nacimiento-afluentes-y-bloqueo--implementados-el-2026-10-29).


### Pasos de Azrum

El Paso de Zunir se representa mediante dos accesos, antes llamados Paso Zek
y Paso Nur, y un Lugar central. Ambos accesos muestran el mismo nombre y
superponen su rótulo extendido entre sus centros. Conservan sus enlaces externos:
el occidental conecta con Hamada de Zahrun, Mesa del Sol, Espinazo Gris y Pico
de Izir, y el oriental con Pico de Izir. Se añade entre ambos el Lugar central
con `paso`, `montanoso` y `oculto`, que reúne la descripción del paisaje y no
se ofrece como destino del jugador. Las conexiones anteriores siguen disponibles,
incluida la alternativa por el Pico de Izir; el recorrido por el centro es una
posibilidad, no un cruce obligatorio.
Monte Dar, antes Paso Dar, enlaza Hamada Roja y Hamada de Zahrun con Espinazo
Gris. Es una montaña empinada con `muroNatural`: una pared alta de roca sólida.
Paso Akr enlaza Espinazo Gris con Paso Escarpado y tiene `empinado`.
Los lugares que siguen siendo pasos tienen `paso` y `montanoso`; estas
características describen su geografía. Las reglas específicas de su travesía
se desarrollarán en la acción de Viajar.

Los accesos, el centro del paso, Monte Dar y Paso Akr pertenecen a la Cordillera
de Azrum. La conexión directa entre
Espinazo Gris y Paso Escarpado se sustituye por el recorrido a través de Paso
Akr. Se retira también la conexión directa entre Paso Escarpado y Alcázar Gris.


### Valles del Drâa y acceso oriental de Qadim

Valle de Azen y Valle de Tirsa amplían la Cuenca del Drâa, y Bosque de Ilun
amplía el Bosque de Tamrut. Valle de Tirsa queda sin conexiones de tránsito. Valle de Azen conecta con
Bosque de Ilun, y este con Marisma Verde. Paso Mir pertenece a Montes de Qadim, conecta Alcázar Gris
con Bosque oscuro y tiene la característica `riscoEstrecho`.

Alcázar Gris y Valle Gris conservan únicamente la relación `Bloquea`, sin
conexión transitable entre ambos. El acceso ordinario a Alcázar Gris desde
Bosque oscuro pasa por Paso Mir.


La red occidental incorpora enlaces de Mesa del Sol con Paso Zek y Mar del
Viento, de Paso Dar con Hamada de Zahrun, y de Lomas de Anir con Mar del Viento,
Aguas de Ámbar y Mar Boreal. En el este, Vado de Tamrut enlaza con Nahal en
lugar de Marisma Verde; el acceso a Aguas de Tamrut parte directamente de Nahal,
mientras que el Estuario enlaza Nahal con Marisma Verde.
