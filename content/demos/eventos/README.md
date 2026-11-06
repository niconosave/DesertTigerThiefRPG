# Demostración de eventos narrativos

No modifica la historia ni los datos de `content/base`.

Desde la raíz del proyecto:

```sh
python3 tools/preparar-demo-eventos.py /tmp/desert-tiger-eventos-demo
cargo run -p desert-tiger-terminal --bin desert-tiger-terminal -- --content /tmp/desert-tiger-eventos-demo
```

La carpeta de destino debe ser nueva. Para repetir, elegí otra ruta.

1. Confirmá el evento inicial con Enter.
2. Elegí viajar a Hamada de Zahrun o a otro destino disponible.
3. Después de un traslado aparece una elección: continuar o detenerse. Usá
   arriba/abajo para elegir y Enter para confirmar.
4. Continuar confirma una variable, habilita la cadena A→B y sustituye el relato
   de llegada si se alcanza el destino. Detenerse conserva lugar y tiempo ya
   recorridos y devuelve el control al mundo.

Las ocurrencias se consumen una sola vez por partida. Reiniciá el juego para
comparar ambas decisiones. `D` abre el detalle cuando exista; izquierda/derecha
cambia de página. Leer y dibujar no ejecuta Lua ni confirma efectos.

Los cinco RON de `events/` son ejemplos de autoría. El contrato completo y las
limitaciones de esta versión están en
[23-eventos-narrativos.md](../../../docs/tecnica/componentes/23-eventos-narrativos.md).
