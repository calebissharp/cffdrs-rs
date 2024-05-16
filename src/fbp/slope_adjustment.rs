use crate::{
    fbp::ros::{rate_of_spread, rate_of_spread_params},
    fbp::FbpFuelType,
    fwi::initial_spread_index,
};

use std::f64::consts;

/// Slope adjusted wind speed or slope direction of spread
///
/// * `ffmc` - Fine fuel moisture code
/// * `ws` - Windspeed (km/h)
/// * `waz` - Wind azimuth (radians)
/// * `gs` - Ground slope (%)
/// * `saz` - Slope azimuth (radians)
/// * `fmc` - Foliar moisture content
/// * `sfc` - Surface fuel consumption (km/m^2)
/// * `pc` - Percent conifer (%)
/// * `pdf` - Percent dead balsam fir (%)
/// * `cc` - Curing constant
/// * `cbh` - Crown base height (m)
///
/// Return (raz, wsv)
/// * `raz` - Rate of spread azimuth (radian)
/// * `wsv` - Wind slope speed (km/h)
///
/// # Examples
///
/// ```
/// # use cffdrs::fbp::{slope_adjustment, FbpFuelType};
/// let fuel_type = FbpFuelType::S2;
/// let ffmc = 7.2;
/// let ws = 145.8;
/// let waz = -3.142;
/// let gs = 81.;
/// let saz = 4.492;
/// let fmc = 437.4;
/// let sfc = 6561.;
/// let pc = 0.;
/// let pdf = 0.;
/// let cc = 0.;
/// let cbh = 0.;
///
/// let (raz, wsv) = slope_adjustment(fuel_type, ffmc, ws, waz, gs, saz, fmc, sfc, pc, pdf, cc, cbh);
/// assert_eq!(wsv, 153.91722882100433);
/// assert_eq!(raz, 3.3124493937190334);
/// ```
pub fn slope_adjustment(
    fuel_type: FbpFuelType,
    ffmc: f64,
    ws: f64,
    waz: f64,
    gs: f64,
    saz: f64,
    fmc: f64,
    sfc: f64,
    pc: f64,
    pdf: f64,
    cc: f64,
    cbh: f64,
) -> (f64, f64) {
    // Spread factor
    let sf = if gs >= 70. {
        10.
    } else {
        consts::E.powf(3.533 * (gs / 100.).powf(1.2))
    };
    // ISI with 0 wind on level grounds
    let isz = initial_spread_index(ffmc, 0.);

    // Surface spread rate with 0 wind on level ground
    let rsz = rate_of_spread(fuel_type, isz, -1., fmc, sfc, pc, pdf, cc, cbh);

    // Surface spread rate with 0 wind upslope
    let rsf = rsz * sf;

    // Surface spread rate for C2 and D1 (used for M1/M2 ISF calculation)
    let rsf_c2 = rate_of_spread(FbpFuelType::C2, isz, -1., fmc, sfc, pc, pdf, cc, cbh) * sf;
    let rsf_d1 = rate_of_spread(FbpFuelType::D1, isz, -1., fmc, sfc, pc, pdf, cc, cbh) * sf;
    // Surface spread rate for M3 with 100% PDF
    let rsf_m3 = rate_of_spread(FbpFuelType::M3, isz, -1., fmc, sfc, pc, 100., cc, cbh) * sf;
    // Surface spread rate for M4 with 100% PDF
    let rsf_m4 = rate_of_spread(FbpFuelType::M4, isz, -1., fmc, sfc, pc, 100., cc, cbh) * sf;

    let cf = if cc < 58.8 {
        0.005 * (consts::E.powf(0.061 * cc) - 1.)
    } else {
        0.176 + 0.02 * (cc - 58.8)
    };

    let isf = isf(fuel_type, rsf, cf, pc, pdf, rsf_c2, rsf_d1, rsf_m3, rsf_m4);

    let m = 147.27723 * (101. - ffmc) / (59.5 + ffmc);
    let ff = 91.9 * consts::E.powf(-0.1386 * m) * (1. + m.powf(5.31) / 4.93e7);

    let wse1 = 1. / 0.05039 * (isf / (0.208 * ff)).ln();
    let wse2 = if isf < 0.999 * 2.496 * ff {
        28. - (1. / 0.0818 * (1. - isf / (2.496 * ff)).ln())
    } else {
        112.45
    };

    let wse = if wse1 <= 40. { wse1 } else { wse2 };

    let wsx = ws * waz.sin() + wse * saz.sin();
    let wsy = ws * waz.cos() + wse * saz.cos();

    let wsv = (wsx * wsx + wsy * wsy).sqrt();
    let raz = (wsy / wsv).acos();
    let raz = if wsx < 0. { 2. * consts::PI - raz } else { raz };

    (raz, wsv)
}

/// Used just in [slope_adjustment()] function
fn isf(
    fuel_type: FbpFuelType,
    rsf: f64,
    cf: f64,
    pc: f64,
    pdf: f64,
    rsf_c2: f64,
    rsf_d1: f64,
    rsf_m3: f64,
    rsf_m4: f64,
) -> f64 {
    let params = rate_of_spread_params(fuel_type);
    let a = params.a;
    let b = params.b;
    let c = params.c;

    match fuel_type {
        FbpFuelType::O1a | FbpFuelType::O1b => {
            let ln_inner = 1. - (rsf / (cf * a)).powf(1. / c);

            if ln_inner >= 0.01 {
                (ln_inner).ln() / (-b)
            } else {
                (0.01f64).ln() / -b
            }
        }
        FbpFuelType::M1 | FbpFuelType::M2 => {
            (pc / 100.)
                * isf(
                    FbpFuelType::C2,
                    rsf_c2,
                    cf,
                    pc,
                    pdf,
                    rsf_c2,
                    rsf_d1,
                    rsf_m3,
                    rsf_m4,
                )
                + (1. - pc / 100.)
                    * isf(
                        FbpFuelType::D1,
                        rsf_d1,
                        cf,
                        pc,
                        pdf,
                        rsf_c2,
                        rsf_d1,
                        rsf_m3,
                        rsf_m4,
                    )
        }
        FbpFuelType::M3 => {
            let ln_inner = 1. - (rsf_m3 / a).powf(1. / c);

            // ISF as if PC was 100%
            let isf_full = if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f64).ln() / -b
            };

            (pdf / 100.) * isf_full
                + (1. - pdf / 100.)
                    * isf(
                        FbpFuelType::D1,
                        rsf_d1,
                        cf,
                        pc,
                        pdf,
                        rsf_c2,
                        rsf_d1,
                        rsf_m3,
                        rsf_m4,
                    )
        }
        FbpFuelType::M4 => {
            let ln_inner = 1. - (rsf_m4 / a).powf(1. / c);

            // ISF as if PC was 100%
            let isf_full = if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f64).ln() / -b
            };

            (pdf / 100.) * isf_full
                + (1. - pdf / 100.)
                    * isf(
                        FbpFuelType::D1,
                        rsf_d1,
                        cf,
                        pc,
                        pdf,
                        rsf_c2,
                        rsf_d1,
                        rsf_m3,
                        rsf_m4,
                    )
        }
        _ => {
            let ln_inner = 1. - (rsf / a).powf(1. / c);

            if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f64).ln() / -b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        fuel_type: FbpFuelType,
        ffmc: f64,
        ws: f64,
        waz: f64,
        gs: f64,
        saz: f64,
        fmc: f64,
        sfc: f64,
        pc: f64,
        pdf: f64,
        cc: f64,
        cbh: f64,
        wsv: f64,
        raz: f64,
    }

    #[test]
    fn test_slope_adjustment() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/slope_adjustment.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let (raz, wsv) = slope_adjustment(
                record.fuel_type,
                record.ffmc,
                record.ws,
                record.waz,
                record.gs,
                record.saz,
                record.fmc,
                record.sfc,
                record.pc,
                record.pdf,
                record.cc,
                record.cbh,
            );

            // TODO: our values should be more precise but there's a lot of floating point error 🤷
            assert!(approx_eq!(
                f64,
                precision_f64(raz, 4),
                record.raz,
                ulps = 2,
                epsilon = 0.1
            ));
            assert!(approx_eq!(
                f64,
                precision_f64(wsv, 4),
                record.wsv,
                ulps = 2,
                epsilon = 0.1
            ));
        }

        Ok(())
    }
}
