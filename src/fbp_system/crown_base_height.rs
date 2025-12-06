use super::FbpFuelType;

/// Get crown base height (CBH) (m)
/// * `sd` - stand density (stems/ha)
/// * `sh` - stand height (m)
pub fn crown_base_height(fuel_type: FbpFuelType, sd: f64, sh: f64) -> f64 {
    match fuel_type {
        FbpFuelType::C6 => -11.2 + 1.06 * sh + 0.0017 * sd,
        FbpFuelType::C1 => 2.,
        FbpFuelType::C2 => 3.,
        FbpFuelType::C3 => 8.,
        FbpFuelType::C4 => 4.,
        FbpFuelType::C5 => 18.,
        FbpFuelType::C7 => 10.,
        FbpFuelType::D1 => 0.,
        FbpFuelType::M1 => 6.,
        FbpFuelType::M2 => 6.,
        FbpFuelType::M3 => 6.,
        FbpFuelType::M4 => 6.,
        FbpFuelType::S1 => 0.,
        FbpFuelType::S2 => 0.,
        FbpFuelType::S3 => 0.,
        FbpFuelType::O1a => 0.,
        FbpFuelType::O1b => 0.,
        FbpFuelType::NonFuel => 0.,
    }
}
