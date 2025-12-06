use std::f64::consts;

use crate::fbp_system::{buildup_effect, crown_fraction_burned, FbpFuelType};

pub struct RateOfSpreadParams {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl RateOfSpreadParams {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

fn default_rsi_calc(a: f64, b: f64, c: f64, isi: f64) -> f64 {
    a * (1. - consts::E.powf(-b * isi)).powf(c)
}

/// Fuel type-specific coefficients for rate of spread
pub fn rate_of_spread_params(fuel_type: FbpFuelType) -> RateOfSpreadParams {
    match fuel_type {
        FbpFuelType::C1 => RateOfSpreadParams::new(90., 0.0649, 4.5),
        FbpFuelType::C2 => RateOfSpreadParams::new(110., 0.0282, 1.5),
        FbpFuelType::C3 => RateOfSpreadParams::new(110., 0.0444, 3.0),
        FbpFuelType::C4 => RateOfSpreadParams::new(110., 0.0293, 1.5),
        FbpFuelType::C5 => RateOfSpreadParams::new(30., 0.0697, 4.0),
        FbpFuelType::C6 => RateOfSpreadParams::new(30., 0.0800, 3.0),
        FbpFuelType::C7 => RateOfSpreadParams::new(45., 0.0305, 2.0),
        FbpFuelType::M3 => RateOfSpreadParams::new(120., 0.0572, 1.4),
        FbpFuelType::M4 => RateOfSpreadParams::new(100., 0.0404, 1.48),
        FbpFuelType::D1 => RateOfSpreadParams::new(30., 0.0232, 1.6),
        FbpFuelType::S1 => RateOfSpreadParams::new(75., 0.0297, 1.3),
        FbpFuelType::S2 => RateOfSpreadParams::new(40., 0.0438, 1.7),
        FbpFuelType::S3 => RateOfSpreadParams::new(55., 0.0829, 3.2),
        FbpFuelType::O1a => RateOfSpreadParams::new(190., 0.0310, 1.4),
        FbpFuelType::O1b => RateOfSpreadParams::new(250., 0.0350, 1.7),
        _ => RateOfSpreadParams::new(0., 0., 0.),
    }
}

/// Rate of spread index
///
/// * `isi` - Initial spread index
/// * `pc` - Percent conifer (%)
/// * `pdf` - Percent dead balsam fir (%)
fn rsi(fuel_type: FbpFuelType, isi: f64, pc: f64, pdf: f64) -> f64 {
    match fuel_type {
        FbpFuelType::C1
        | FbpFuelType::C2
        | FbpFuelType::C3
        | FbpFuelType::C4
        | FbpFuelType::C5
        | FbpFuelType::C7
        | FbpFuelType::D1
        | FbpFuelType::S1
        | FbpFuelType::S2
        | FbpFuelType::S3
        | FbpFuelType::O1a
        | FbpFuelType::O1b => {
            let params = rate_of_spread_params(fuel_type);
            default_rsi_calc(params.a, params.b, params.c, isi)
        }
        FbpFuelType::M1 => {
            (pc / 100.) * (rsi(FbpFuelType::C2, isi, pc, pdf))
                + ((100. - pc) / 100.) * (rsi(FbpFuelType::D1, isi, pc, pdf))
        }
        FbpFuelType::M2 => {
            (pc / 100.) * (rsi(FbpFuelType::C2, isi, pc, pdf))
                + 0.2 * ((100. - pc) / 100.) * (rsi(FbpFuelType::D1, isi, pc, pdf))
        }
        FbpFuelType::M3 => {
            let params_100 = rate_of_spread_params(fuel_type);
            let rsi_100 = default_rsi_calc(params_100.a, params_100.b, params_100.c, isi);

            (pdf / 100.) * rsi_100 + (1. - (pdf / 100.)) * rsi(FbpFuelType::D1, isi, pc, pdf)
        }
        FbpFuelType::M4 => {
            let params_100 = rate_of_spread_params(fuel_type);
            let rsi_100 = default_rsi_calc(params_100.a, params_100.b, params_100.c, isi);

            (pdf / 100.) * rsi_100 + 0.2 * (1. - pdf / 100.) * rsi(FbpFuelType::D1, isi, pc, pdf)
        }
        FbpFuelType::C6 => 30. * (1. - consts::E.powf(-0.08 * isi)).powf(3.0),

        FbpFuelType::NonFuel => 0.,
    }
}

/// Critical surface intensity
fn csi(fmc: f64, cbh: f64) -> f64 {
    0.001 * cbh.powf(1.5) * (460. + 25.9 * fmc).powf(1.5)
}

/// Surface fire rate of spread
pub fn rso(csi: f64, sfc: f64) -> f64 {
    csi / (300. * sfc)
}

pub struct ExtendedRateOfSpread {
    /// Rate of spread (m/min)
    pub ros: f64,
    /// Crown fraction burned
    pub cfb: f64,
    /// Critical surface intensity
    pub csi: f64,
    /// Surface fire rate of spread
    pub rso: f64,
}

/// Rate of spread calculation, returning ROS, as well as CFB, CSI and RSO
///
/// See [rate_of_spread()] for a more information
#[allow(clippy::too_many_arguments)]
pub fn rate_of_spread_extended(
    fuel_type: FbpFuelType,
    isi: f64,
    bui: f64,
    fmc: f64,
    sfc: f64,
    pc: f64,
    pdf: f64,
    cc: f64,
    cbh: f64,
) -> ExtendedRateOfSpread {
    let rsi = rsi(fuel_type, isi, pc, pdf);

    let cf = if cc < 58.8 {
        0.005 * (consts::E.powf(0.061 * cc) - 1.)
    } else {
        0.176 + 0.02 * (cc - 58.8)
    };

    let rsi = match fuel_type {
        FbpFuelType::O1a | FbpFuelType::O1b => rsi * cf,
        _ => rsi,
    };

    let csi = csi(fmc, cbh);

    let rso = rso(csi, sfc);

    let rsi = match fuel_type {
        FbpFuelType::C6 => 30. * (1. - consts::E.powf(-0.08 * isi)).powf(3.0),
        _ => rsi,
    };

    // Crown fire spread rate (m/min)
    let rsc = match fuel_type {
        FbpFuelType::C6 => {
            // Average foliar moisture effect
            let fme_avg = 0.778;
            let fme = ((1.5 - 0.00275 * fmc).powf(4.0) / (460. + (25.9 * fmc))) * 1000.;
            Some(60. * (1. - consts::E.powf(-0.0497 * isi)) * (fme / fme_avg))
        }
        _ => None,
    };

    let rss = rsi * buildup_effect(fuel_type, bui);

    let cfb = match fuel_type {
        FbpFuelType::C6 => {
            if rsc.unwrap() > rss && !rso.is_nan() && rss > rso {
                crown_fraction_burned(rss, rso)
            } else {
                0.
            }
        }
        _ => crown_fraction_burned(rss, rso),
    };

    let ros = match fuel_type {
        FbpFuelType::C6 => {
            if rsc.unwrap() > rss {
                rss + (cfb) * (rsc.unwrap() - rss)
            } else {
                rss
            }
        }
        _ => rss,
    };

    let ros = if ros <= 0. { 0.000001 } else { ros };

    ExtendedRateOfSpread { ros, cfb, csi, rso }
}

/// Calculate fire rate of spread
///
/// * `isi` - Initial spread index (See [crate::fwi_system::initial_spread_index()] to calculate this value)
/// * `bui` - Buildup index
/// * `fmc` - Foliar moisture content
/// * `sfc` - Surface fuel consumption
/// * `pc` - Percent confier
/// * `pdf` - Percent dead balsam fir
/// * `cc` - Degree of curing (usually a constant)
/// * `cbh` - Crown base height
///
/// Returns fire rate of spread (m/min)
///
/// # Examples
///
/// ```
/// # use cffdrs::fbp_system::{FbpFuelType, ros::rate_of_spread};
/// let isi = 120.6;
/// let bui = 437.4;
/// let fmc = 0.0;
/// let sfc = 0.0;
/// let pc = 0.0;
/// let pdf = 0.0;
/// let cc = 0.0;
/// let cbh = 0.0;
/// let ros = rate_of_spread(FbpFuelType::C3, isi, bui, fmc, sfc, pc, pdf, cc, cbh);
/// assert_eq!(ros, 132.34133371748217);
///
/// assert_eq!(
///     rate_of_spread(FbpFuelType::C6, 277.2, 656.1, 218.7, 6561., 81., 81., 54., 72.9),
///     35.30930607800089
/// );
/// assert_eq!(
///     rate_of_spread(FbpFuelType::O1a, 6.3, 218.7, 437.4, 19683., 54., 81., 54., 72.9),
///     2.1900055814792427
/// );
/// assert_eq!(
///     rate_of_spread(FbpFuelType::S2, 151.2, 437.4, 0., 6561., 27., 0., 0., 0.),
///     48.52361319847542
/// );
/// ```
#[allow(clippy::too_many_arguments)]
pub fn rate_of_spread(
    fuel_type: FbpFuelType,
    isi: f64,
    bui: f64,
    fmc: f64,
    sfc: f64,
    pc: f64,
    pdf: f64,
    cc: f64,
    cbh: f64,
) -> f64 {
    rate_of_spread_extended(fuel_type, isi, bui, fmc, sfc, pc, pdf, cc, cbh).ros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        fuel_type: FbpFuelType,
        isi: f64,
        bui: f64,
        fmc: f64,
        sfc: f64,
        pc: f64,
        pdf: f64,
        cc: f64,
        cbh: f64,
        ros: f64,
    }

    #[test]
    fn test_rate_of_spread() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/rate_of_spread.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let ros = rate_of_spread(
                record.fuel_type,
                record.isi,
                record.bui,
                record.fmc,
                record.sfc,
                record.pc,
                record.pdf,
                record.cc,
                record.cbh,
            );

            assert_eq!(precision_f64(ros, 4), record.ros);
        }

        Ok(())
    }
}
