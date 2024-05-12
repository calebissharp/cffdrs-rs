use std::f32::consts;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum FbpFuelType {
    /// Spruce-Lichen Woodland
    C1,
    /// Boreal Spruce
    C2,
    /// Mature Jack or Lodgepole Pine
    C3,
    /// Immature Jack or Lodgepole Pine
    C4,
    /// Red and White Pine
    C5,
    /// Conifer Plantation
    C6,
    /// Ponderosa Pine-Douglas-fir
    C7,
    /// Boreal Mixedwood
    M1,
    /// Boreal Mixedwood
    M2,
    /// Dead Balsam Fir Mixedwood
    M3,
    /// Dead Balsam Fir Mixedwood
    M4,
    /// Leafless Aspen
    D1,
    /// Jack or Lodgepole Pine Slash
    S1,
    /// White Spruce-Balsam Slash
    S2,
    /// Coastal Cedar-Hemlock-Douglas fir Slash
    S3,
    /// Grass
    O1a,
    /// Grass
    O1b,
    /// Non-fuel
    NonFuel,
}

struct RateOfSpreadParams {
    a: f32,
    b: f32,
    c: f32,
}

impl RateOfSpreadParams {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
}

// Surface fuel consumption (km/m^2)
pub fn sfc(fuel_type: FbpFuelType, ffmc: f32, bui: f32) -> f32 {
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
            (pc / 100. * sfc(FbpFuelType::C2, ffmc, bui))
                + (ph / 100. * sfc(FbpFuelType::D1, ffmc, bui))
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

fn default_rsi_calc(a: f32, b: f32, c: f32, isi: f32) -> f32 {
    a * (1. - consts::E.powf(-b * isi)).powf(c)
}

fn rate_of_spread_params(fuel_type: FbpFuelType) -> RateOfSpreadParams {
    match fuel_type {
        FbpFuelType::C1 => RateOfSpreadParams::new(90., 0.0649, 4.5),
        FbpFuelType::C2 => RateOfSpreadParams::new(110., 0.0282, 1.5),
        FbpFuelType::C3 => RateOfSpreadParams::new(110., 0.0444, 3.0),
        FbpFuelType::C4 => RateOfSpreadParams::new(110., 0.0293, 1.5),
        FbpFuelType::C5 => RateOfSpreadParams::new(30., 0.0697, 4.0),
        FbpFuelType::C6 => RateOfSpreadParams::new(30., 0.0800, 3.0),
        FbpFuelType::C7 => RateOfSpreadParams::new(45., 0.0305, 2.0),
        FbpFuelType::D1 => RateOfSpreadParams::new(30., 0.0232, 1.6),
        FbpFuelType::S1 => RateOfSpreadParams::new(75., 0.0297, 1.3),
        FbpFuelType::S2 => RateOfSpreadParams::new(40., 0.0438, 1.7),
        FbpFuelType::S3 => RateOfSpreadParams::new(55., 0.0829, 3.2),
        FbpFuelType::O1a => RateOfSpreadParams::new(190., 0.0310, 1.4),
        FbpFuelType::O1b => RateOfSpreadParams::new(250., 0.0350, 1.7),
        _ => RateOfSpreadParams::new(0., 0., 0.),
    }
}

/// Get crown base height (m)
/// * `sd` - stand density (stems/ha)
/// * `sh` - stand height (m)
pub fn cbh(fuel_type: FbpFuelType, sd: f32, sh: f32) -> f32 {
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

/// Crown fuel load (kg/m^2)
pub const fn cfl(fuel_type: FbpFuelType) -> f32 {
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
        // Values not provided in paper
        FbpFuelType::D1 => 0.,
        FbpFuelType::S1 => 0.,
        FbpFuelType::S2 => 0.,
        FbpFuelType::S3 => 0.,
        FbpFuelType::O1a => 0.,
        FbpFuelType::O1b => 0.,

        FbpFuelType::NonFuel => 0.,
    }
}

/// Rate of spread index
///
/// * `isi` - Initial spread index
/// * `pc` - Percent conifer (%)
/// * `pdf` - Percent dead balsam fir (%)
pub fn rsi(fuel_type: FbpFuelType, isi: f32, pc: f32, pdf: f32) -> f32 {
    match fuel_type {
        FbpFuelType::C1
        | FbpFuelType::C2
        | FbpFuelType::C3
        | FbpFuelType::C4
        | FbpFuelType::C5
        | FbpFuelType::C6
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
            let a = 170. * consts::E.powf(-35. / pdf);
            let b = 0.082 * consts::E.powf(-36. / pdf);
            let c = 1.698 - 0.00303 * pdf;
            default_rsi_calc(a, b, c, isi)
        }
        FbpFuelType::M4 => {
            let a = 140. * consts::E.powf(-33.5 / pdf);
            let b = 0.0404;
            let c = 3.02 * consts::E.powf(-0.00714 * pdf);
            default_rsi_calc(a, b, c, isi)
        }

        FbpFuelType::NonFuel => 0.,
    }
}

/// Critical surface intensity
fn csi(fmc: f32, cbh: f32) -> f32 {
    0.001 * cbh.powf(1.5) * (460. + 25.9 * fmc).powf(1.5)
}

/// Surface fire rate of spread
fn rso(csi: f32, sfc: f32) -> f32 {
    csi / (300. * sfc)
}

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
/// assert_eq!(buildup_effect, 0.43453118);
/// # }
///
/// assert_eq!(buildup_effect(FbpFuelType::O1a, 10.8), 1.);
/// assert_eq!(buildup_effect(FbpFuelType::S3, 13.5), 0.54799676);
/// ```
pub fn buildup_effect(fuel_type: FbpFuelType, bui: f32) -> f32 {
    let bui_avg: f32 = match fuel_type {
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
    let q: f32 = match fuel_type {
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

    if bui > 0. && bui_avg > 0. {
        consts::E.powf(50. * q.ln() * (1. / bui - 1. / bui_avg))
    } else {
        1.
    }
}

fn cfb(ros: f32, rso: f32) -> f32 {
    if ros > rso {
        1. - consts::E.powf(-0.23 * (ros - rso))
    } else {
        0.
    }
}

pub struct ExtendedRateOfSpread {
    pub ros: f32,
    pub cfb: f32,
    pub csi: f32,
    pub rso: f32,
}

/// Rate of spread
///
/// See [rate_of_spread]
pub fn ros_extended(
    fuel_type: FbpFuelType,
    isi: f32,
    bui: f32,
    fmc: f32,
    sfc: f32,
    pc: f32,
    pdf: f32,
    cc: f32,
    cbh: f32,
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
            let fme = (1.5 - 0.00275 * fmc).powf(4.0) / (460. + (25.9 * fmc)) * 1000.;
            Some(60. * (1. - consts::E.powf(-0.0497 * isi)).powf(1.0) * (fme / fme_avg))
        }
        _ => None,
    };

    let rss = rsi * buildup_effect(fuel_type, bui);

    let cfb = match fuel_type {
        FbpFuelType::C6 => {
            if rsc.unwrap() > rss && rss > rso {
                cfb(rss, rso)
            } else {
                0.
            }
        }
        _ => cfb(rss, rso),
    };

    let ros = match fuel_type {
        FbpFuelType::C6 => {
            if rsc.unwrap() > rss {
                rss + cfb * (rsc.unwrap() - rss)
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
/// * `isi` - Initial spread index (See [initial_spread_index] to calculate this value)
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
/// # use cffdrs::fbp::{FbpFuelType, rate_of_spread};
/// let isi = 120.6;
/// let bui = 437.4;
/// let fmc = 0.0;
/// let sfc = 0.0;
/// let pc = 0.0;
/// let pdf = 0.0;
/// let cc = 0.0;
/// let cbh = 0.0;
/// let ros = rate_of_spread(FbpFuelType::C3, isi, bui, fmc, sfc, pc, pdf, cc, cbh);
/// assert_eq!(ros, 132.34134);
///
/// assert_eq!(
///     rate_of_spread(FbpFuelType::C6, 277.2, 656.1, 218.7, 6561., 81., 81., 54., 72.9),
///     35.309303
/// );
/// assert_eq!(
///     rate_of_spread(FbpFuelType::O1a, 6.3, 218.7, 437.4, 19683., 54., 81., 54., 72.9),
///     2.1900055
/// );
/// ```
pub fn rate_of_spread(
    fuel_type: FbpFuelType,
    isi: f32,
    bui: f32,
    fmc: f32,
    sfc: f32,
    pc: f32,
    pdf: f32,
    cc: f32,
    cbh: f32,
) -> f32 {
    ros_extended(fuel_type, isi, bui, fmc, sfc, pc, pdf, cc, cbh).ros
}

/// Calculate flank rate of spread
///
/// * `ros` - fire rate of spread (m/min)
/// * `bros` - back fire rate of spread (m/min)
/// * `lb` - length to breadth ratio
///
/// Returns flank fire spread rate (m/min)
pub fn fros(ros: f32, bros: f32, lb: f32) -> f32 {
    (ros + bros) / lb / 2.
}

/// Calculate back fire rate of spread
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
pub fn bros(
    fuel_type: FbpFuelType,
    ffmc: f32,
    bui: f32,
    wsv: f32,
    fmc: f32,
    sfc: f32,
    pc: f32,
    pdf: f32,
    cc: f32,
    cbh: f32,
) -> f32 {
    let m = 147.27723 * (101. - ffmc) / (59.5 + ffmc);
    let ff = 91.9 * consts::E.powf(-0.1386 * m) + (1. + (m.powf(5.31) / 4.93e7));
    // Back fire wind function
    let bfw = consts::E.powf(0.05039 * wsv);
    // ISI associated with the back fire spread rate
    let bisi = 0.208 * bfw * ff;

    rate_of_spread(fuel_type, bisi, bui, fmc, sfc, pc, pdf, cc, cbh)
}

/// Calculate ROS at angle theta
///
/// `ros` - Fire rate of spread
/// `fros` - Flank fire rate of spread
/// `bros` - Back fire rate of spread
/// `theta` - Angle in degrees, 0 = E, 90 = S, etc
///
/// Returns rate of spread at angle theta (m/min)
pub fn ros_at_theta(ros: f32, fros: f32, bros: f32, theta: f32) -> f32 {
    let c1 = theta.cos();
    let s1 = theta.sin();
    let c1 = if c1 == 0. { (theta + 0.001).cos() } else { c1 };

    // Eq. 94 (https://cfs.nrcan.gc.ca/pubwarehouse/pdfs/31414.pdf)

    ((ros - bros) / (2. * c1) + (ros + bros) / 2. * c1)
        * ((fros * c1 * (fros.powi(2) * c1.powi(2) + (ros * bros) * s1.powi(2)).sqrt()
            - (ros.powi(2) - bros.powi(2)) / 4. * s1.powi(2))
            / (fros.powi(2) * c1.powi(2) + ((ros + bros) / 2.0).powi(2) * s1.powi(2)))
}

/// Calculate foliar moisture content
pub fn fmc(
    lat: f32,
    long: f32,
    day_of_year: i32,
    elev: Option<f32>,
    date_of_minimum_fmc: Option<i32>,
) -> f32 {
    let latn = if elev.is_some() {
        43. + 33.7 * consts::E.powf(-0.0351 * (150. - long))
    } else {
        46. + 23.4 * consts::E.powf(-0.0360 * (150. - long))
    };

    let d0 = date_of_minimum_fmc.unwrap_or_else(|| {
        if let Some(elev) = elev {
            (142.1 * (lat / latn) + 0.0172 * elev).round() as i32
        } else {
            (151. * (lat / latn)).round() as i32
        }
    });

    // Number of days between day of year and date of min FMC
    let nd = (day_of_year - d0).abs() as f32;

    if nd < 30. {
        85. + 0.0189 * nd.powi(2)
    } else if nd >= 30. && nd < 50. {
        32.9 + 3.17 * nd - 0.0288 * nd.powi(2)
    } else {
        120.
    }
}

/// Calculate length-to-breadth ratio
///
/// * `wsv` - slope-adjusted wind speed (km/h) (see [slope_adjustment] to calculate this value)
pub fn length_to_breadth(fuel_type: FbpFuelType, wsv: f32) -> f32 {
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

/// Slope adjusted wind speed or slope direction of spread
///
/// * `ffmc` - Fine fuel moisture code
/// * `bui` - Buildup index
/// * `ws` - Windspeed (km/h)
/// * `waz` - Wind azimuth
/// * `gs` - Ground slope (%)
/// * `saz` - Slope azimuth
/// * `fmc` - Foliar moisture content
/// * `sfc` - Surface fuel consumption (km/m^2)
/// * `pc` - Percent conifer (%)
/// * `pdf` - Percent dead balsam fir (%)
/// * `cc` - constant
/// * `cbh` - Crown base height (m)
/// * `isi` - Initial spread index
///
/// Return (raz, wsv)
/// * `raz` - Rate of spread azimuth (degrees)
/// * `wsv` - Wind slope speed (km/h)
pub fn slope_adjustment(
    fuel_type: FbpFuelType,
    ffmc: f32,
    bui: f32,
    ws: f32,
    waz: f32,
    gs: f32,
    saz: f32,
    fmc: f32,
    sfc: f32,
    pc: f32,
    pdf: f32,
    cc: f32,
    cbh: f32,
    isi: f32,
) -> (f32, f32) {
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

    let rsf = rsz * sf;

    let cf = if cc < 58.8 {
        0.005 * (consts::E.powf(0.061 * cc) - 1.)
    } else {
        0.176 + 0.02 * (cc - 58.8)
    };

    let isf = isf(fuel_type, rsf, cf, pc, pdf);

    let m = 147.27723 * (101. - ffmc) / (59.5 + ffmc);
    let ff = 91.9 * consts::E.powf(-0.1386 * m) * (1. + (m.powf(5.31) / 49300000.));

    let wse1 = (isf / 0.208 * ff).ln() / 0.05039;
    let wse2 = if isf < 0.999 * 2.496 * ff {
        28. - (1. - (isf / 2.496 * ff)).ln() / 0.0818
    } else {
        112.45
    };

    let wse = if wse1 <= 40. { wse1 } else { wse2 };

    let wsx = (ws * waz.sin()) + (wse * saz.sin());

    let wsy = (ws * waz.cos()) + (wse * saz.cos());
    let wsv = (wsx * wsx + wsy * wsy).sqrt();
    let raz = (wsy / wsv).acos();
    let raz = if wsx < 0. { 360. - raz } else { raz };

    (raz, wsv)
}

/// Calculate initial spread index
///
/// * `ffmc` - Fine fuel moisture code
/// * `ws` - Wind speed (km/h)
///
/// Returns ISI
///
/// ```
/// # use cffdrs::fbp::initial_spread_index;
/// let isi = initial_spread_index(0.6, 0.0);
/// assert_eq!(isi, 2.9864071e-9);
///
/// assert_eq!(initial_spread_index(9.6, 0.0), 1.0478607e-6);
/// assert_eq!(initial_spread_index(26.4, 24.3), 0.0040652887);
/// assert_eq!(initial_spread_index(58.8, 24.3), 1.2853143);
/// assert_eq!(initial_spread_index(7.2, 48.6), 3.0496415e-6);
/// assert_eq!(initial_spread_index(39.3, 72.9), 1.1899886);
/// ````
pub fn initial_spread_index(ffmc: f32, ws: f32) -> f32 {
    // Moisture content
    let fm = 147.27723 * (101. - ffmc) / (59.5 + ffmc);

    // Wind effect
    let fw = consts::E.powf(0.05039 * ws);

    // Fine fuel moisture
    let ff = 91.9 * consts::E.powf(-0.1386 * fm) * (1. + (fm.powf(5.31) / 49300000.));

    // Spread index
    0.208 * fw * ff
}

fn isf(fuel_type: FbpFuelType, rsf: f32, cf: f32, pc: f32, pdf: f32) -> f32 {
    let params = rate_of_spread_params(fuel_type);
    let a = params.a;
    let b = params.b;
    let c = params.c;

    match fuel_type {
        FbpFuelType::O1a | FbpFuelType::O1b => {
            let ln_inner = 1. - (rsf / cf * a).powf(1. / c);

            if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f32).ln() / -b
            }
        }
        FbpFuelType::M1 | FbpFuelType::M2 => {
            (pc / 100.) * isf(FbpFuelType::C2, rsf, cf, pc, pdf)
                + (1. - pc / 100.) * isf(FbpFuelType::D1, rsf, cf, pc, pdf)
        }
        FbpFuelType::M3 | FbpFuelType::M4 => {
            let ln_inner = 1. - (rsf / a).powf(1. / c);

            // ISF as if PC was 100%
            let isf_full = if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f32).ln() / -b
            };

            (pdf / 100.) * isf_full + (1. - pdf / 100.) * isf(FbpFuelType::D1, rsf, cf, pc, pdf)
        }
        _ => {
            let ln_inner = 1. - (rsf / a).powf(1. / c);

            if ln_inner >= 0.01 {
                (ln_inner).ln() / -b
            } else {
                (0.01f32).ln() / -b
            }
        }
    }
}
