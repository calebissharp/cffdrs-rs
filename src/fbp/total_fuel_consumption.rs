/// Total fuel consumption (TFC)
///
/// * `sfc` - Surface fuel consumption (kg/m^2)
/// * `cfc` - Crown fuel consumption (kg/m^2)
///
/// Returns TFC (kg/m^2)
pub fn total_fuel_consumption(sfc: f64, cfc: f64) -> f64 {
    sfc + cfc
}
