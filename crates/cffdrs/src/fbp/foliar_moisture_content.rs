use std::f64::consts;

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
