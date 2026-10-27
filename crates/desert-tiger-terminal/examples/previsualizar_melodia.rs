//! Previsualización offline mediante el mismo sintetizador que el campamento.
use std::{error::Error, path::Path};
fn main() -> Result<(), Box<dyn Error>> {
    let argumentos: Vec<_> = std::env::args().skip(1).collect();
    if argumentos.len() > 2 {
        return Err("uso: previsualizar_melodia [entrada.ron] [salida.wav]".into());
    }
    let entrada = argumentos
        .first()
        .map(String::as_str)
        .unwrap_or("assets/audio/musica/hilo_del_camino_castellana.ron");
    let salida = argumentos
        .get(1)
        .map(String::as_str)
        .unwrap_or("target/previsualizaciones/hilo_del_camino_castellana.wav");
    let duracion =
        desert_tiger_terminal::previsualizar_melodia(Path::new(entrada), Path::new(salida))?;
    println!("Vista previa — {duracion:.1} s: {salida}");
    Ok(())
}
