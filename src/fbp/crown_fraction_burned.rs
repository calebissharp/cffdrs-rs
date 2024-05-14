use std::f64::consts;

/// Calculate crown fraction burned (CFB)
///
///
/// * `ros` - Rate of spread (m/min)
/// * `rso` - Critical surface fire spread rate
///
/// Return CFB in the range `0..=1`
pub fn crown_fraction_burned(ros: f64, rso: f64) -> f64 {
    if ros > rso {
        1. - consts::E.powf(-0.23 * (ros - rso))
    } else {
        0.
    }
}
