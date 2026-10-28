# Atlas regional de trabajo

| Campo | Valor |
|---|---|
| Estado | Implementado |
| Versión | 0.9 |
| Fecha | 2026-10-28 |

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
| Llano del Viento | Llanura abierta del noroeste interior. | `arido`, `abierto`, `ventoso` | Orientación, persecuciones y falta de refugio. |
| Mesa del Sol | Elevación árida del extremo norte. | `arido`, `elevado`, `expuesto` | Ascenso, observación y señales a distancia. |
| Hamada Roja | Desierto rocoso del centro occidental; reemplaza visualmente a `Zona Árida`. | `arido`, `rocoso`, `expuesto` | Punto inicial, rastreo, calor y escasez de refugio. |
| Cabo Errante | Península árida del sudoeste. | `costero`, `rocoso`, `ventoso` | Rodeos, campamentos expuestos y temporales. |
| Playa Dorada | Franja de arena en la costa del golfo interior. | `costero`, `arenoso`, `resguardado` | Descanso, búsqueda y acceso marítimo futuro. |

### Cordilleras y pasos

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Espinazo Gris | Cordillera principal, estrecha y alargada de norte a sur. | `montanoso`, `elevado`, `empinado` | Escalada, desprendimientos, visibilidad y clima. |
| Paso Escarpado | Cruce estrecho entre la Hamada y el oriente fértil. | `empinado`, `estrecho` | Complicación ya diseñada para `Viajar`. |
| Sierra Quebrada | Ramal montañoso diagonal que limita con el bosque. | `montanoso`, `quebrado`, `boscoso` | Elección entre roca y vegetación, emboscadas y extravío. |

### Oriente fértil

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Valle del Drâa | Valle fértil del nordeste; reemplaza visualmente a `Valle Fértil`. | `fertil`, `fluvial`, `resguardado` | Descanso, ayuda, recursos y crecidas. |
| Bosque de Tamrut | Gran bosque oriental atravesado por cursos de agua. | `boscoso`, `cerrado`, `humedo` | Orientación, caza, ocultamiento y encuentros. |
| Marisma Verde | Tierras bajas y húmedas del sudeste. | `pantanoso`, `humedo`, `fluvial` | Demoras, vadeo, enfermedades y rastros difíciles. |

### Lugares civilizados

En el mapa se usarían solamente sus nombres propios cortos. Su naturaleza queda
en este atlas y, cuando haga falta para una regla, en Características.

| Nombre | Naturaleza y posición visual | Vocabulario editorial | Posibilidades futuras |
|---|---|---|---|
| Tazar | Pueblo del Valle del Drâa. | `civilizado`, `poblado`, `acogedor` | Descanso, reclutamiento, ayuda y rumores. |
| Alcázar Gris | Fortaleza que controla el paso central. | `civilizado`, `fortificado`, `controlado` | Permisos, asilo, engaño y conflicto político. |
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
| Mar de Poniente | Mar abierto al oeste de la masa árida. | `maritimo`, `abierto`, `ventoso` | Temporales, pérdida de rumbo y llegada a la isla. |
| Mar Boreal | Aguas del borde norte del mapa. | `maritimo`, `abierto`, `profundo` | Corrientes, niebla y navegación costera. |
| Mar de Levante | Mar que bordea el bosque y la marisma por el este. | `maritimo`, `abierto`, `humedo` | Tormentas, navegación junto a la costa y acceso a Nahal. |
| Golfo del Siroco | Cuenca marítima situada al sur entre ambas masas terrestres. | `maritimo`, `costero`, `ventoso` | Navegación protegida o peligrosa según el viento, pesca y acceso a Playa Dorada. |
| Isla del Exilio | Isla árida del sudoeste. | `insular`, `aislado`, `arido` | Exploración, rescate y refugios difíciles de alcanzar. |

No se añade además un `Mar Austral`: el Golfo del Siroco ocupa la región
meridional útil y otro rótulo saturaría esa parte del mapa. El río visible se
considera por ahora contexto cartográfico y fluvial del Valle del Drâa, el
Bosque de Tamrut y la Marisma Verde. Podrá convertirse en uno o varios Lugares
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
Paso Escarpado — Valle del Drâa — Bosque de Tamrut — Sierra Quebrada
                       │                 │
                     Tazar         Marisma Verde — Nahal

Alcázar Gris se conecta con Paso Escarpado y Sierra Quebrada.

Circuito marítimo:
Mar de Poniente — Mar Boreal — Mar de Levante — Golfo del Siroco
       │                                             │
       └────────────── Isla del Exilio ─────────────┘

Accesos costeros:
Costa Ámbar — Mar de Poniente
Mesa del Sol — Mar Boreal
Nahal — Mar de Levante
Playa Dorada — Golfo del Siroco
Cabo Errante — Golfo del Siroco
```

El corredor oriental implementado por el brief de
[trayectos de viaje](./trayectos-viaje.md) es:

```text
Hamada Roja → Camino de los Mojones → Paso Escarpado
→ Descenso del Drâa → Valle del Drâa → Senda de Tamrut
→ Bosque de Tamrut → Vado de Tamrut → Marisma Verde
→ Calzada de Nahal → Nahal → Estuario de Nahal → Mar de Levante
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

La composición marítima de trabajo reserva zonas distintas: `Mar de Poniente`
en el agua superior izquierda, `Mar Boreal` en la franja superior central,
`Mar de Levante` mediante un recorrido principalmente vertical junto al borde
oriental y `Golfo del Siroco` mediante una curva suave sobre la cuenca del sur.
Así los nombres describen la forma del agua sin necesitar rotar sus letras.

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

Se incorporan doce regiones como Lugares sin conexiones transitables, con
Relaciones Abarca hacia los Lugares existentes. Cubren las tierras áridas,
las costas, ambas cordilleras, la cuenca fértil, el bosque, las tierras bajas,
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
define su significado. Esta ampliación conserva la red de viajes existente.
