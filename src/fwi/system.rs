//! Helper functions for calculating FWI values from weather

use chrono::Datelike;

use crate::weather::Weather;

use super::{
    buildup_index, drought_code, duff_moisture_code, fire_weather_index,
    hourly_fine_fuel_moisture_code, initial_spread_index,
};

/// Hourly values for the FWI system
#[derive(Debug, Clone)]
pub struct HourlyFwiValues {
    pub isi: f64,
    pub dc: f64,
    pub dmc: f64,
    pub ffmc: f64,
    pub bui: f64,
    pub fwi: f64,
}

impl HourlyFwiValues {
    const DEFAULT: HourlyFwiValues = HourlyFwiValues {
        ffmc: 50.,
        dmc: 50.,
        dc: 50.,
        isi: 0.,
        bui: 0.,
        fwi: 0.,
    };
}

/// Calculate all values for the FWI system from hourly weather
///
/// * `weather` - Hourly weather
/// * `previous` - FWI values from the previous hour. Providing this argument will improve the
/// accuracy of calculations
///
/// # Examples
///
/// ```
/// use cffdrs::fwi::calculate_hourly;
/// use cffdrs::weather::Weather;
/// use chrono::prelude::*;
///
/// let weather = Weather {
///     time: Utc.with_ymd_and_hms(2023, 7, 8, 9, 10, 11).unwrap(),
///     location: geo::Point::new(-120.34, 50.69),
///     temp: 28.,
///     rh: 12.,
///     ws: 16.,
///     wd: 95.,
///     precip: 0.,
/// };
///
/// let fwi_values = calculate_hourly(&weather, None);
///
/// assert_eq!(fwi_values.ffmc, 57.90482214144054);
/// assert_eq!(fwi_values.dmc, 56.014192448);
/// assert_eq!(fwi_values.dc, 58.744);
/// assert_eq!(fwi_values.isi, 0.7933315614248974);
/// assert_eq!(fwi_values.bui, 55.44718193037011);
/// assert_eq!(fwi_values.fwi, 2.2766000331952063);
/// ```
pub fn calculate_hourly(weather: &Weather, previous: Option<&HourlyFwiValues>) -> HourlyFwiValues {
    let previous = previous.unwrap_or(&HourlyFwiValues::DEFAULT);

    let ffmc = hourly_fine_fuel_moisture_code(
        weather.temp,
        weather.rh,
        weather.ws,
        weather.precip,
        previous.ffmc,
        Some(1.),
    );
    let dc = drought_code(
        previous.dc,
        weather.temp,
        weather.precip,
        weather.location.y(),
        weather.time.month() as usize,
        Some(true),
    );
    let dmc = duff_moisture_code(
        previous.dmc,
        weather.temp,
        weather.rh,
        weather.precip,
        weather.location.y(),
        weather.time.month() as usize,
        Some(true),
    );
    let bui = buildup_index(dmc, dc);
    let isi = initial_spread_index(ffmc, weather.ws);
    let fwi = fire_weather_index(isi, bui);

    HourlyFwiValues {
        ffmc,
        dmc,
        dc,
        isi,
        bui,
        fwi,
    }
}

// TODO: Add calculate_daily (and maybe calculate_diurnally?)
