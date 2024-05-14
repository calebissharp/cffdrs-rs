use std::f64::consts;

use super::FbpFuelType;

/// Calculate length-to-breadth ratio (LB)
///
/// * `wsv` - slope-adjusted wind speed (km/h) (see [crate::fbp::slope_adjustment()] to calculate this value)
pub fn length_to_breadth(fuel_type: FbpFuelType, wsv: f64) -> f64 {
    match fuel_type {
        FbpFuelType::O1a | FbpFuelType::O1b => {
            if wsv >= 1.0 {
                1.1 * wsv.powf(0.464)
            } else {
                1.
            }
        }
        _ => 1. + 8.729 * (1. - consts::E.powf(-0.030 * wsv)).powf(2.155),
    }
}
