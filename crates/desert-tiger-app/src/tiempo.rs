use desert_tiger_core::{
    Duracion, FraccionDia, InstanteAccion, Mundo, TiempoAccion, proyectar_tiempo,
};

pub(crate) fn confirmar(
    mundo: &mut Mundo,
    resto: &mut FraccionDia,
    tiempo: TiempoAccion,
) -> Result<(), String> {
    let inicio = InstanteAccion {
        fecha: mundo.fecha(),
        fraccion: *resto,
    };
    let fin = proyectar_tiempo(inicio, tiempo).map_err(|e| e.to_string())?;
    // Ambos resultados se validan antes de confirmar cualquiera de los dos.
    let acarreo = (u64::from(resto.partes()) + u64::from(tiempo.fraccion.partes())) / 2_400_000_000;
    let duracion = tiempo
        .duracion
        .checked_add(Duracion::desde_dias(acarreo as u32))
        .map_err(|e| e.to_string())?;
    mundo.avanzar_fecha(duracion).map_err(|e| e.to_string())?;
    *resto = fin.fraccion;
    Ok(())
}
pub(crate) fn formatear(t: TiempoAccion) -> String {
    let mut texto = format!("{} días", t.duracion.dias());
    if t.duracion.meses() > 0 {
        texto = format!("{} meses y {texto}", t.duracion.meses());
    }
    let partes = u64::from(t.fraccion.partes());
    if partes > 0 {
        let horas = partes / 100_000_000;
        let decimal = partes % 100_000_000;
        if decimal == 0 {
            texto.push_str(&format!(" y {horas} horas"));
        } else {
            let dec = format!("{decimal:08}");
            texto.push_str(&format!(" y {horas},{} horas", dec.trim_end_matches('0')));
        }
    }
    texto
}
