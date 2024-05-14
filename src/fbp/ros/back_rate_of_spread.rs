use std::f64::consts;

use crate::fbp::{ros::rate_of_spread, FbpFuelType};

/// Calculate back fire rate of spread (BROS)
///
/// * `ffmc` - Fine fuel moisture code
/// * `bui` - Buildup index
/// * `wsv` - Wind speed vector
/// * `fmc` - Foliar moisture content
/// * `sfc` - Surface fuel consumption
/// * `pc` - Percent confier
/// * `pdf` - Percent dead balsam fir
/// * `cc` - Degree of curing
/// * `cbh` - Crown base height
///
/// Returns back fire rate of spread (m/min)
pub fn back_rate_of_spread(
    fuel_type: FbpFuelType,
    ffmc: f64,
    bui: f64,
    wsv: f64,
    fmc: f64,
    sfc: f64,
    pc: f64,
    pdf: f64,
    cc: f64,
    cbh: f64,
) -> f64 {
    let m = 147.27723 * (101. - ffmc) / (59.5 + ffmc);
    let ff = 91.9 * consts::E.powf(-0.1386 * m) + (1. + (m.powf(5.31) / 4.93e7));
    // Back fire wind function
    let bfw = consts::E.powf(0.05039 * wsv);
    // ISI associated with the back fire spread rate
    let bisi = 0.208 * bfw * ff;

    rate_of_spread(fuel_type, bisi, bui, fmc, sfc, pc, pdf, cc, cbh)
}
