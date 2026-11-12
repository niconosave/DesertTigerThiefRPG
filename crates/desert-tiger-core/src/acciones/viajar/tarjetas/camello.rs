use super::*;
pub(crate) fn resolver(
    c: &ContextoMovimiento<'_>,
    azar: &mut dyn FuenteAzar,
) -> Resultado<Movimiento> {
    let mut m = comenzar(c, Medio::Camello, AccionParticular::Cabalgar, 6, azar)?;
    let [a, b] = m.dato.chequeos.as_slice() else {
        return Err(fallo("se esperaban dos chequeos de desplazamiento"));
    };
    if a.ajustado < 8 && b.ajustado < 8 && (a.ajustado < 6 || b.ajustado < 6) {
        let dado = (sortear(azar, 20)? + 1) as u8;
        aplicar_montura(c, &mut m, dado, azar)?;
    } else {
        terminar_mitades(c, &mut m, [0, 0], false)?;
    }
    Ok(m)
}
