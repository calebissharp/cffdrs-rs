use std::f64::consts;

use super::FbpFuelType;

/// Calculate the buildup effect on fire spread rate
///
/// * `bui` - Buildup index value
///
/// Returns the buildup effect
///
/// ```
/// # use cffdrs::fbp::{buildup_effect, FbpFuelType};
/// # {
/// let bui = 13.5;
/// let buildup_effect = buildup_effect(FbpFuelType::C3, bui);
/// assert_eq!(buildup_effect, 0.43453121924260246);
/// # }
///
/// assert_eq!(buildup_effect(FbpFuelType::O1a, 10.8), 1.);
/// assert_eq!(buildup_effect(FbpFuelType::S3, 13.5), 0.5479968092625566);
/// ```
pub fn buildup_effect(fuel_type: FbpFuelType, bui: f64) -> f64 {
    let bui_avg: f64 = match fuel_type {
        FbpFuelType::C1 => 72.,
        FbpFuelType::C2 => 64.,
        FbpFuelType::C3 => 62.,
        FbpFuelType::C4 => 66.,
        FbpFuelType::C5 => 56.,
        FbpFuelType::C6 => 62.,
        FbpFuelType::C7 => 106.,
        FbpFuelType::D1 => 32.,
        FbpFuelType::M1 => 50.,
        FbpFuelType::M2 => 50.,
        FbpFuelType::M3 => 50.,
        FbpFuelType::M4 => 50.,
        FbpFuelType::S1 => 38.,
        FbpFuelType::S2 => 63.,
        FbpFuelType::S3 => 31.,
        FbpFuelType::O1a => 01.,
        FbpFuelType::O1b => 01.,
        FbpFuelType::NonFuel => 0.,
    };
    let q: f64 = match fuel_type {
        FbpFuelType::C1 => 0.9,
        FbpFuelType::C2 => 0.7,
        FbpFuelType::C3 => 0.75,
        FbpFuelType::C4 => 0.8,
        FbpFuelType::C5 => 0.8,
        FbpFuelType::C6 => 0.8,
        FbpFuelType::C7 => 0.85,
        FbpFuelType::D1 => 0.9,
        FbpFuelType::M1 => 0.8,
        FbpFuelType::M2 => 0.8,
        FbpFuelType::M3 => 0.8,
        FbpFuelType::M4 => 0.8,
        FbpFuelType::S1 => 0.75,
        FbpFuelType::S2 => 0.75,
        FbpFuelType::S3 => 0.75,
        FbpFuelType::O1a => 1.0,
        FbpFuelType::O1b => 1.0,
        FbpFuelType::NonFuel => 0.,
    };
    let max: f64 = match fuel_type {
        FbpFuelType::C1 => 1.076,
        FbpFuelType::C2 => 1.321,
        FbpFuelType::C3 => 1.261,
        FbpFuelType::C4 => 1.184,
        FbpFuelType::C5 => 1.220,
        FbpFuelType::C6 => 1.197,
        FbpFuelType::C7 => 1.134,
        FbpFuelType::D1 => 1.179,
        FbpFuelType::M1 => 1.25,
        FbpFuelType::M2 => 1.25,
        FbpFuelType::M3 => 1.25,
        FbpFuelType::M4 => 1.25,
        FbpFuelType::S1 => 1.46,
        FbpFuelType::S2 => 1.256,
        FbpFuelType::S3 => 1.59,
        FbpFuelType::O1a => 1.0,
        FbpFuelType::O1b => 1.0,
        FbpFuelType::NonFuel => 0.,
    };

    if bui > 0. && bui_avg > 0. {
        consts::E
            .powf(50. * q.ln() * (1. / bui - 1. / bui_avg))
            .min(max)
    } else {
        1.
    }
}
