# Apertura de la campaña del príncipe

La partida normal empieza en Costa Ámbar con el príncipe y el sirviente.
`naufragio_y_bandido` se activa en la fecha inicial de `world.ron` (31/01/2025),
antes de la primera acción, y presenta a Bandido. Al confirmar el encuentro,
Bandido se incorpora de verdad al grupo y recomienda Zarabad.

Cuando una acción termina con el grupo en Zarabad, `bandido_venta_zarabad`
presenta una advertencia antes de la audiencia: Nadin expresa su sospecha sobre
Rashak mediante un diálogo, sin ofrecer pruebas de la venta. Seguir a Bandido lleva a la venta;
rechazar la audiencia permite continuar sin quedar incorporado a la compañía.
En ambos casos Bandido deja el grupo. La venta registra `EsclavoDe` hacia Zahrim para el príncipe y el
sirviente y los incorpora juntos a la compañía de Dahran, al servicio de Zahrim.

El servicio utiliza sucesos por punto `fase_npc.terminada`, con plazos relativos
a `inicioServicioDahran`: una primera salida al Mar del Viento después de un día
y un regreso a Zarabad con permiso para desembarcar después de catorce. Se
comprueban días del calendario, no cantidad de pulsaciones o entradas al menú.
`ctx:ahora().dia_absoluto` permite comparar fechas incluso al cambiar mes o año.

El permiso devuelve el movimiento independiente y conserva la relación `EsclavoDe`: no resuelve
la guerra, no concede la libertad ni define todavía cómo abandonar esa obligación.
Es el cierre provisional del primer episodio de servicio, para que la partida
pueda seguir mientras se diseñan los siguientes acontecimientos. No se simulan
combates, recompensas o victorias contra Nuris en este episodio.

Los eventos corresponden al príncipe como protagonista. Otras futuras aperturas
podrán establecer sus propios encuentros y conflictos.
