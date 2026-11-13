# Demostración de colectivos y espera de órdenes

Escenario aislado; no cambia `content/base` ni establece historia oficial.

```sh
python3 tools/preparar-demo-colectivos.py /tmp/desert-tiger-colectivos-demo
cargo run -p desert-tiger-terminal --bin desert-tiger-terminal -- --content /tmp/desert-tiger-colectivos-demo
```

Usá una carpeta de destino nueva. Se puede conservar el resto de opciones del
juego, como audio, colores o gráficos alternativos.

1. Aceptá la incorporación inicial. Aparecerá el marco con «Compañía de Dahran»;
   el menú permitirá Acampar y herramientas de desarrollo.
2. Abrí Acampar con A. Con derecha/izquierda seleccioná **Acciones del campamento**.
   Con abajo seleccioná **Esperar órdenes** y confirmá con Enter.
3. Se muestra el mapa y avanza una jornada cada medio segundo. El 1 de febrero
   la compañía viaja al Mar del Viento. Confirmá el relato; el avance se detiene.
4. Volvé a Acampar y esperá nuevamente. El relato del mercante del 2 de febrero
   pausa el reloj mientras se lee, pero no detiene definitivamente la espera.
5. Podés pulsar Esc para regresar al campamento. El tiempo pasado se conserva y
   al salir de esa acampada no se agrega otra jornada ya cubierta por la espera.
6. El 3 de febrero termina el servicio en Costa Ámbar: desaparece el marco,
   conservás tus compañeros y vuelve a estar disponible Viajar.

La licencia depende solo de seguir en la compañía, no de que el capitán esté
vivo. Los eventos también mueven a la compañía si rechazás la incorporación;
no verás sus relatos privados. El ingreso comprueba que queden citas futuras.

Los archivos ejemplifican el [contrato 26](../../../docs/tecnica/componentes/26-actores-colectivos-y-espera.md).
