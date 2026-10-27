# Prueba de cuerda pulsada: Hilo del camino

Melodía nueva de ocho compases, con arpegios de acompañamiento y una segunda
frase más alta antes de volver al reposo. El MIDI `Whatever_FF8_Breezy.mid`
aportado como referencia sugiere una guitarra tranquila y ligera; esta pieza
usa otras notas, ritmo y progresión. No importa ni reproduce ese MIDI.

La versión grave está aceptada y se interpreta ocasionalmente al acampar si
hay un integrante vivo con `musico`. El sirviente inicial tiene esa característica.
Mientras toca, unas notas salen de su retrato. El volumen y los intervalos
se configuran en [`campamento.ron`](./campamento.ron), y se cargan al reiniciar.
La primera pieza espera 60–120 segundos; entre piezas hay pausas de 20–180 segundos.
`--no-audio` deshabilita tanto la música como sus notas animadas.

Solo interpreta un integrante a la vez, aunque haya varios músicos en el
grupo. Todos sus integrantes pueden incluir la expresión `music`, que se
utiliza mientras escuchan una pieza y después vuelve a la cara anterior.
El príncipe, el bandido y el sirviente ya la incluyen. Un Actor sin esa variante
conserva su expresión vigente. Las notas siguen saliendo solo del músico. Tocar
en conjunto como una banda queda para un diseño futuro.

## Repertorio del campamento

`campamento.ron` contiene `partituras`, una lista de archivos que se preparan al
iniciar. El primer archivo suena primero; luego se elige otra pieza, evitando
repeticiones consecutivas. Con las dos piezas actuales se alternan incluso
entre campamentos. Se mantiene un músico a la vez. También se admite el campo
antiguo `partitura` para un único archivo, sin combinarlo con `partituras`.

- `recuerdo_del_hogar.ron`: nueva pieza de unos 61 segundos, 24 compases de
  tres tiempos. Usa como inspiración el carácter hogareño de `FF5_home.mid`,
  con notas y acompañamiento distintos y la misma guitarra grave.
- `hilo_del_camino_castellana.ron`: versión grave ya aceptada, unos 25 segundos.

Para escuchar la nueva pieza por separado:

```sh
cargo run --release -p desert-tiger-terminal --example previsualizar_melodia -- assets/audio/musica/recuerdo_del_hogar.ron target/previsualizaciones/recuerdo_del_hogar.wav
```

Los niveles y las pausas actuales se conservan. El repertorio admite hasta
16 archivos distintos y 10 minutos de audio preparado en total; esos límites
controlan la memoria reservada antes de reproducir.

## Escuchar y editar

Desde la raíz del proyecto:

```sh
cargo run --release -p desert-tiger-terminal --example previsualizar_melodia
```

Genera `target/previsualizaciones/hilo_del_camino_castellana.wav`, mono PCM de 16 bits a
48000 Hz. Puede abrirse con un reproductor de audio habitual. No consulta ni
abre dispositivos, y no requiere MIDI, SoundFonts o bibliotecas adicionales.
Es una previsualización generada; el archivo editable fuente es
[`hilo_del_camino_castellana.ron`](./hilo_del_camino_castellana.ron).

También admite otras rutas, en el orden entrada RON y salida WAV:

```sh
cargo run --release -p desert-tiger-terminal --example previsualizar_melodia -- assets/audio/musica/hilo_del_camino.ron target/previsualizaciones/prueba.wav
```

### Versión más grave y calmada

[`hilo_del_camino_castellana.ron`](./hilo_del_camino_castellana.ron) conserva la
melodía una cuarta más grave, con los bajos acomodados para no bajar de E2.
El tempo pasa de 92 a 80 negras por minuto y la amortiguación de 0.15 a 0.28,
para probar una cuerda menos brillante. La primera versión se conserva para
comparar. El usuario aceptó esta segunda versión como pieza del campamento.

```sh
cargo run --release -p desert-tiger-terminal --example previsualizar_melodia -- assets/audio/musica/hilo_del_camino_castellana.ron target/previsualizaciones/hilo_del_camino_castellana.wav
```

## Formato experimental

Es el formato inicial compartido por el campamento y la previsualización;
otros repertorios podrán ampliarlo cuando se diseñen.

- `titulo`: texto visible no vacío, sin caracteres de control.
- `tempo`: negras por minuto, finito entre 40 y 200.
- `volumen`: ganancia global entre 0 y 1.
- `amortiguacion`: entre 0 y 1; valores mayores apagan más los agudos.
- `notas`: entre 1 y 512 eventos `(inicio, nota, duración, intensidad)`.
  Inicio y duración se expresan en negras: los eventos pueden superponerse.
  Los huecos entre eventos son silencios. Inicio admite 0–512; duración,
  0.05–16; intensidad, 0–1. Todos los números deben ser finitos.
- Las notas usan letras internacionales: `C`=Do, `D`=Re, `E`=Mi, `F`=Fa,
  `G`=Sol, `A`=La, `B`=Si, más octava; por ejemplo `A3`, `C#4`, `Bb3`.
  Se admite el rango MIDI 24–96 (C1–C7).

Se rechazan campos ausentes, desconocidos y duplicados. El sintetizador usa el
generador `pluck` de FunDSP con semillas estables, ataque y retirada suaves,
filtrado y limitación de amplitud. Renderiza las notas offline, donde puede
preparar cada cuerda antes de sumarla. El juego comparte este sintetizador con
la previsualización: prepara la pieza una vez y el callback mezcla su buffer
inmutable, sin crear instrumentos durante la reproducción. Los disparadores,
la coordinación de notas y los límites de configuración se definen en el
[contrato 20](../../../docs/tecnica/componentes/20-musica-campamento.md).
