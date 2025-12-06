use std::f64::consts::PI;

use crate::{fwi::HourlyFwiValues, weather::Weather};

use super::{
    crown_base_height, crown_fuel_consumption, crown_fuel_load, fire_intensity,
    foliar_moisture_content, length_to_breadth,
    ros::{back_rate_of_spread, flank_rate_of_spread, rate_of_spread_extended},
    slope_adjustment, surface_fuel_consumption, total_fuel_consumption, FbpFuelType,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct FbpValues {
    /// Crown fraction burned (0-1)
    pub cfb: f64,
    /// Direction of spread (radians)
    pub raz: f64,
    /// Head rate of spread (m/min)
    pub ros: f64,
    /// Flank rate of spread (m/min)
    pub fros: f64,
    /// Back rate of spread  (m/min)
    pub bros: f64,
    /// Foliar moisture content
    pub fmc: f64,
    /// Surface fuel consumption (kg/m^2)
    pub sfc: f64,
    /// Crown fuel consumption (kg/m^2)
    pub cfc: f64,
    /// Total fuel consumption (kg/m^2)
    pub tfc: f64,
    /// Crown fuel load
    pub cfl: f64,
    /// Crown base height (m)
    pub cbh: f64,
    /// Effective wind speed (km/h)
    pub wsv: f64,
    /// Effective wind direction (radians)
    pub wsz: f64,
    /// Head fire intensity (kW/m)
    pub fi: f64,
}

pub struct FbpOptions {
    pub elevation: Option<f64>,
    pub date_of_minimum_fmc: Option<i32>,
    /// Slope (%)
    pub slope: f64,
    /// Slope azimuth (degrees)
    pub aspect: f64,
}

impl Default for FbpOptions {
    fn default() -> Self {
        Self {
            elevation: None,
            date_of_minimum_fmc: None,
            slope: 0.,
            aspect: 0.,
        }
    }
}

/// Calculate all FBP values from weather and FWI values
pub fn calculate_fbp(
    fuel_type: FbpFuelType,
    fwi: &HourlyFwiValues,
    weather: &Weather,
    options: FbpOptions,
) -> FbpValues {
    let pc = 50.;
    let cc = 80.;
    let pdf = 35.;
    let sd = 0.;
    let sh = 0.;

    let julian_date = julian::Date::from(weather.time);
    let lat = weather.location.y();
    let long = weather.location.x();

    let cbh = crown_base_height(fuel_type, sd, sh);

    let fmc = foliar_moisture_content(
        lat,
        long,
        julian_date.ordinal() as i32,
        options.elevation,
        options.date_of_minimum_fmc,
    );

    let sfc = surface_fuel_consumption(fuel_type, fwi.ffmc, fwi.bui);

    let (wsz, wsv) = slope_adjustment(
        fuel_type,
        fwi.ffmc,
        weather.ws,
        weather.wd.to_radians(),
        options.slope,
        options.aspect.to_radians(),
        fmc,
        sfc,
        pc,
        pdf,
        cc,
        cbh,
    );

    let ros = rate_of_spread_extended(fuel_type, fwi.isi, fwi.bui, fmc, sfc, pc, pdf, cc, cbh);
    let bros = back_rate_of_spread(
        fuel_type, fwi.ffmc, fwi.bui, wsv, fmc, sfc, pc, pdf, cc, cbh,
    );
    let lb = length_to_breadth(fuel_type, wsv);
    let fros = flank_rate_of_spread(ros.ros, bros, lb);

    let cfl = crown_fuel_load(fuel_type);
    let cfc = crown_fuel_consumption(fuel_type, cfl, ros.cfb, pc, pdf);
    let tfc = total_fuel_consumption(sfc, cfc);

    let fi = fire_intensity(tfc, ros.ros);

    let raz = wsz + PI;

    FbpValues {
        cfb: ros.cfb,
        ros: ros.ros,
        fros,
        bros,
        raz,
        fmc,
        sfc,
        cfc,
        tfc,
        cfl,
        cbh,
        wsv,
        wsz,
        fi,
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use chrono::prelude::*;

    use super::*;
    use crate::fwi::calculate_hourly;

    #[test]
    fn test_calculate_fbp() -> Result<(), Box<dyn std::error::Error>> {
        let fuel_type = FbpFuelType::C2;
        let weather = Weather {
            time: Utc.with_ymd_and_hms(2023, 7, 8, 9, 10, 11).unwrap(),
            location: geo::Point::new(-122., 37.),
            temp: 35.,
            rh: 5.,
            ws: 35.,
            wd: 45.,
            precip: 0.,
        };

        let fwi_values = calculate_hourly(
            &weather,
            Some(&HourlyFwiValues {
                isi: 3.,
                dc: 60.,
                dmc: 60.,
                ffmc: 90.,
                bui: 55.,
                fwi: 10.,
            }),
        );

        let fbp = calculate_fbp(
            fuel_type,
            &fwi_values,
            &weather,
            FbpOptions {
                aspect: 90.,
                slope: 10.,
                ..Default::default()
            },
        );

        // These values are from the BC Gov FBP calculator (slightly modified for floating point errors): https://psu.nrs.gov.bc.ca/fbp-go
        assert_ulps_eq!(fbp.cfb, 0.9999982568944272);
        assert_ulps_eq!(fbp.ros, 59.02036259012382);
        assert_ulps_eq!(fbp.fi, 61913.09917582581);
        assert_ulps_eq!(fbp.fros, 6.28955758552331);
        assert_ulps_eq!(fbp.bros, 0.5030990617854207);
        assert_ulps_eq!(fbp.wsv, 37.37715072152419);
        assert_ulps_eq!(fbp.wsz, 0.8471315044464778);
        assert_ulps_eq!(fbp.cfc, 0.7999986055155418);
        assert_ulps_eq!(fbp.tfc, 3.4967083934399077);
        assert_ulps_eq!(fbp.raz, 3.9887241580362707);

        Ok(())
    }
}
