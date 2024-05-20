use super::FbpFuelType;

/// Crown fuel load (CFL) (kg/m^2)
pub const fn crown_fuel_load(fuel_type: FbpFuelType) -> f64 {
    match fuel_type {
        FbpFuelType::C1 => 0.75,
        FbpFuelType::C2 => 0.8,
        FbpFuelType::C3 => 1.15,
        FbpFuelType::C4 => 1.2,
        FbpFuelType::C5 => 1.2,
        FbpFuelType::C6 => 1.8,
        FbpFuelType::C7 => 0.5,
        FbpFuelType::M1 => 0.8,
        FbpFuelType::M2 => 0.8,
        FbpFuelType::M3 => 0.8,
        FbpFuelType::M4 => 0.8,
        FbpFuelType::D1 => 0.,
        FbpFuelType::S1 => 0.,
        FbpFuelType::S2 => 0.,
        FbpFuelType::S3 => 0.,
        FbpFuelType::O1a => 0.,
        FbpFuelType::O1b => 0.,

        FbpFuelType::NonFuel => 0.,
    }
}
