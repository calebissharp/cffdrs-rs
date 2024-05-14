mod crown_fraction_burned;
mod rate_of_spread;
mod slope_adjustment;

pub use crown_fraction_burned::*;
pub use rate_of_spread::*;
pub use slope_adjustment::*;

use std::f64::consts;

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

// Surface fuel consumption (km/m^2)
pub fn sfc(fuel_type: FbpFuelType, ffmc: f64, bui: f64) -> f64 {
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

/// Get crown base height (m)
/// * `sd` - stand density (stems/ha)
/// * `sh` - stand height (m)
pub fn cbh(fuel_type: FbpFuelType, sd: f64, sh: f64) -> f64 {
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
pub const fn cfl(fuel_type: FbpFuelType) -> f64 {
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
        consts::E.powf(50. * q.ln() * (1. / bui - 1. / bui_avg))
        // .min(max)
    } else {
        1.
    }
}

/// Calculate foliar moisture content (FMC)
///
/// * `lat` - Latitude (degrees)
/// * `long` - Longitude (degrees)
/// * `day_of_year` - Julian day of year
/// * `elev` - Elevation at the given lat/long (optional)
/// * `date_of_minimum_fmc` - Date of the year with minimum FMC, as a Julian day (optional)
///
/// # Examples
///
/// ```
/// # use cffdrs::fbp::{foliar_moisture_content};
///
/// let lat = -48.7;
/// let long = 107.1;
/// let day_of_year = 81;
/// let elev = Some(6561.);
/// let date_of_minimum_fmc = None;
///
/// let fmc = foliar_moisture_content(lat, long, day_of_year, elev, date_of_minimum_fmc);
/// assert_eq!(fmc, 120.);
///
/// assert_eq!(foliar_moisture_content(-80.1, 180., 81, None, Some(81)), 85.);
/// assert_eq!(foliar_moisture_content(-80.1, 180., 0, None, Some(81)), 120.);
/// assert_eq!(foliar_moisture_content(-14.6, 34.2, 243, Some(6561.), Some(324)), 120.);
/// assert_eq!(foliar_moisture_content(31.5, 180., 0, None, None), 114.4572);
///
/// ```
pub fn foliar_moisture_content(
    lat: f64,
    long: f64,
    day_of_year: i32,
    elev: Option<f64>,
    date_of_minimum_fmc: Option<i32>,
) -> f64 {
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
    let nd = (day_of_year - d0).abs() as f64;

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
/// * `wsv` - slope-adjusted wind speed (km/h) (see [slope_adjustment()] to calculate this value)
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
