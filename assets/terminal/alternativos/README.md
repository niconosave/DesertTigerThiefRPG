# Gráficos alternativos

Desde la raíz del proyecto, elegí un modo al iniciar:

```sh
cargo run --release --bin desert-tiger-terminal -- --graphics custom-font
cargo run --release --bin desert-tiger-terminal -- --graphics ascii
```

`custom-font` lee `fuente/`; `ascii` lee `ascii/`. Sin esta opción, o con
`--graphics default`, se usan los gráficos habituales. Se puede combinar con
`--color-scheme fixed`, `-B`, `--no-audio` y `--content RUTA`.

El juego no instala ni selecciona fuentes. Para ver tus caracteres privados,
configurá tu terminal con la fuente que los contiene. Los archivos se leen al
iniciar: reiniciá el juego después de editarlos.

## Retratos

Cada retrato va en `actors/<clave-del-actor>.ron` dentro del modo elegido.
Por ejemplo:

```text
fuente/actors/principe_perdido.ron
fuente/actors/sirviente.ron
ascii/actors/principe_perdido.ron
```

Usá la `clave` del RON del actor, aunque su nombre visible sea distinto.
El archivo alternativo contiene solamente el retrato: `paleta`, `base` y,
si los necesitás, `rostro` y `parpadeo`. Para adaptar un retrato existente,
copiá el contenido interior de `retrato: Some(( ... ))` y envolvelo en
`( ... )`, sin `retrato:` ni el `Some` exterior. No copies biografía,
características ni relaciones.

[retrato.ron.example](./retrato.ron.example) es un ejemplo mínimo válido.
Copialo a la carpeta elegida y renombralo con la clave real del actor y
extensión `.ron`. Reemplazá sus dibujos y paleta por los tuyos.

El retrato alternativo sustituye la base y todas las capas del actor. Puede
tener otras dimensiones; sus coordenadas de rostro y parpadeo se miden desde
su propia esquina superior izquierda. No se mezclan ojos ni expresiones del
retrato original. Se conservan transparencia, colores, iluminación y animación.

## Tiles

Los `tiles.ron` de ambas carpetas comienzan vacíos. Agregá únicamente los tipos
que quieras reemplazar. Por ejemplo:

```ron
(
    apariencias: {
        "bosque": (
            patron: r#"
^Y^
Y^Y
"#,
            estilo: (frente: "verdeOscuro", fondo: None, modificadores: []),
        ),
    },
)
```

El resto de los terrenos conserva su apariencia habitual. Podés usar `glifo`,
`patron` o `fotogramas` y `duracion_fotograma_ms` como en el catálogo original.
En `fuente/tiles.ron` podés pegar tus caracteres privados en esos mismos campos.
`reserva` es opcional y tiene el formato del catálogo original.

## Otros dibujos

También se admiten estos archivos dentro de cada modo:

| Archivo | Contenido que reemplaza |
|---|---|
| `scenes.ron` | Solo las entradas presentes en `ilustraciones`; las demás se conservan. |
| `campamento.ron` | El recurso completo de la fogata y sus ajustes, con el formato original. |
| `ui.ron` | Los estilos completos de la interfaz, con el formato original. |

## Respaldo y validación

Si no existe un archivo de actor, un catálogo opcional o una entrada de tile o
escena, se usa su versión habitual. No se buscan archivos en el otro modo.
Si un archivo existe pero tiene errores, se informa su ruta y el problema para
que puedas corregirlo.

Los dibujos conservan las reglas de tamaño y ancho de una columna. En `ascii/`
solo se admiten caracteres imprimibles entre espacio y `~` en los dibujos,
incluidas todas las capas y fotogramas. Los comentarios pueden llevar tildes.

**Esta carpeta prepara un catálogo ASCII alternativo, no una conversión
automática de toda la pantalla.** Hasta que agreguemos sus dibujos, el respaldo
puede mostrar Unicode. Los marcos, símbolos de lugares, notas musicales y
efectos generados por código también conservan sus caracteres actuales.

Contrato técnico: [frontend, gráficos alternativos](../../../docs/tecnica/componentes/09-frontend-terminal.md#15-gráficos-alternativos).
