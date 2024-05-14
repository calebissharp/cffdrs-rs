use std::f64::consts;

use super::FbpFuelType;

/// Surface fuel consumption SFC (km/m^2)
///
/// * `ffmc` - Fine fuel moisture code
/// * `bui` - Buildup index
pub fn surface_fuel_consumption(fuel_type: FbpFuelType, ffmc: f64, bui: f64) -> f64 {
    match fuel_type {
        FbpFuelType::C1 => (1.5 * (1. - consts::E.powf(-0.230 * (ffmc - 81.)))).max(0.),
        FbpFuelType::C2 | FbpFuelType::M3 | FbpFuelType::M4 => {
            5.0 * (1. - consts::E.powf(-0.0115 * bui)).powf(1.)
        }
        FbpFuelType::C3 | FbpFuelType::C4 => 5.0 * (1. - consts::E.powf(-0.0164 * bui)).powf(2.24),
        FbpFuelType::C5 | FbpFuelType::C6 => 5.0 * (1. - consts::E.powf(-0.0149 * bui)).powf(2.48),
        FbpFuelType::C7 => {
            let ffc = (2. * (1. - consts::E.powf(-0.104 * (ffmc - 70.)))).max(0.);
            let wfc = 1.5 * (1. - consts::E.powf(-0.0201 * bui));
            ffc + wfc
        }
        FbpFuelType::D1 => 1.5 * (1. - consts::E.powf(-0.0183 * bui)),
        FbpFuelType::M1 | FbpFuelType::M2 => {
            let pc = 50.;
            let ph = 50.;
            (pc / 100. * surface_fuel_consumption(FbpFuelType::C2, ffmc, bui))
                + (ph / 100. * surface_fuel_consumption(FbpFuelType::D1, ffmc, bui))
        }
        FbpFuelType::O1b | FbpFuelType::O1a => {
            // Grass fuel load (standard = 0.3km/m^2)
            let gfl = 0.3;
            gfl
        }
        FbpFuelType::S1 => {
            let ffc = 4.0 * (1. - consts::E.powf(-0.025 * bui));
            let wfc = 4.0 * (1. - consts::E.powf(-0.034 * bui));
            ffc + wfc
        }
        FbpFuelType::S2 => {
            let ffc = 10.0 * (1. - consts::E.powf(-0.013 * bui));
            let wfc = 6.0 * (1. - consts::E.powf(-0.060 * bui));
            ffc + wfc
        }
        FbpFuelType::S3 => {
            let ffc = 12.0 * (1. - consts::E.powf(-0.0166 * bui));
            let wfc = 20.0 * (1. - consts::E.powf(-0.0210 * bui));
            ffc + wfc
        }

        FbpFuelType::NonFuel => 0.,
    }
}
