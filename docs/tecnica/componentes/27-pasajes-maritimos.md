# Pasajes marítimos derivados del comercio

Estado: implementado. Fecha: 2026-11-12.

## Regla y contenido

Los vínculos Compra/Vende existentes identifican ciudades que comercian. Compra
se normaliza como la Vende inversa; no hay una relación OfrecePasaje.

Dos lugares locales con `puerto`, que no sean regiones, permiten contratar pasaje
si comercian y existe un recorrido entre ellos con lugares intermedios
`acuatico`. Se asume que algún carguero acepta pasajeros en ambos sentidos,
aunque solo se declare comercio en una dirección. Varias mercancías no crean
servicios duplicados. El comercio externo sin recorrido local no genera pasajes.

El pago es narrativo provisional, sin precio numérico ni saldo. Durante los tramos contratados el capitán proporciona comida y agua: no se
consumen reservas del grupo ni se acumulan horas pendientes de ración. Los
tramos terrestres y la preparación mantienen su consumo habitual. Contratar
pasaje no entrega un barco.

## Búsqueda y ejecución

Al construir Mundo, para cada par comercial portuario se recorre el grafo
Conectado con una búsqueda en anchura. Solo se admiten lugares acuáticos entre
los puertos; se elige el recorrido con menos conexiones, con desempate estable.
No se buscan caminos entre tiles ni se atraviesa tierra. Sin recorrido marítimo,
el comercio sigue existiendo pero no habilita transporte marítimo.

Se indexan los destinos, itinerarios y pares consecutivos de cada servicio para
consultarlos durante el viaje. El planificador combina conexiones terrestres y
servicios completos. El destino debe ser conocido o inmediatamente accesible,
como en las otras formas de Viajar. No se habilita navegación libre.

La previsión suma todos los tramos del itinerario marítimo. La visualización
expande esos mismos lugares. Antes de preparar provisiones se expande también
la ruta de ejecución: se recorren sus conexiones una por una mediante escenas
de pasaje. Por ejemplo: Shahraz → Mar del Viento → Zarabad. No se salta el mar
ni se desembarca en un lugar acuático. El pasajero no elige desvíos en el mar.

Cada mitad de conexión contratada usa una base de 24 horas, ajustada por tamaño
y por el sistema de carga. Los rasgos terrestres del puerto no ralentizan el
barco. Los pasajeros no hacen chequeos de navegación; el capitán anónimo y su
tripulación lo conducen. El relato describe el tramo actual a bordo, sin afirmar
que se contrata un barco nuevo o se desembarca en cada lugar de mar. No se inicia búsqueda de provisiones entre tramos contratados aunque las
reservas estén vacías. La previsión de alimento y agua excluye esos tramos; el
tiempo del viaje, el conocimiento y los eventos siguen sus procesos habituales.

Los índices son derivados del estado de relaciones y conexiones al construir
Mundo. La API actual no permite añadir o quitar comercio durante una partida;
cuando se incorpore esa mutación deberá reconstruir estos índices. No mantener
una lista independiente de pasajes que pueda quedar desactualizada.

## Contenido inicial y comprobación

Zarabad comercia con Shahraz, y Shahraz con Nuris; Nuris vende pescado seco a
Shahraz, no a Zarabad. Nuris está conectada a Aguas de Ámbar. Los recorridos usan
Mar del Viento y, para Nuris, Aguas de Ámbar. No hay pasaje directo entre las
ciudades enfrentadas; se puede contratar otro servicio desde Shahraz.

Pruebas acotadas: ida y vuelta, ausencia de navegación libre, puertos válidos,
ausencia de ruta o comercio, expansión de lugares marítimos, duración prevista
y ejecutada, ausencia de dados del pasajero y viaje completo con el contenido base.
