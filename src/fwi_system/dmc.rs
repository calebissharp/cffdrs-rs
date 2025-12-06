use std::f64::consts::E;

/// 46N: Canadian standard, latitude >= 30N   (Van Wagner 1987)
const ELL01: [f64; 12] = [6.5, 7.5, 9., 12.8, 13.9, 13.9, 12.4, 10.9, 9.4, 8., 7., 6.];
/// 20N: For 30 > latitude >= 10
const ELL02: [f64; 12] = [7.9, 8.4, 8.9, 9.5, 9.9, 10.2, 10.1, 9.7, 9.1, 8.6, 8.1, 7.8];
/// 20S: For -10 > latitude >= -30
const ELL03: [f64; 12] = [10.1, 9.6, 9.1, 8.5, 8.1, 7.8, 7.9, 8.3, 8.9, 9.4, 9.9, 10.2];
/// 40S: For -30 > latitude
const ELL04: [f64; 12] = [
    11.5, 10.5, 9.2, 7.9, 6.8, 6.2, 6.5, 7.4, 8.7, 10., 11.2, 11.8,
];

/// Calculate Duff Moisture Code (DMC)
///
/// * `prev_dmc` - Yesterday's DMC
/// * `temp` - Temperature (Celcius)
/// * `rh` - Relative humidity (%)
/// * `precip` - Precipitation (mm)
/// * `lat` - Latitutde (decimal degrees)
/// * `mon` - Month (1-12)
/// * `lat_adjustment` - Whether to apply latitude adjustment (optional, default = `true`)
pub fn duff_moisture_code(
    prev_dmc: f64,
    temp: f64,
    rh: f64,
    precip: f64,
    lat: f64,
    mon: usize,
    lat_adjustment: Option<bool>,
) -> f64 {
    let lat_adjustment = lat_adjustment.unwrap_or(true);

    let index = (mon).clamp(1, 12) - 1;

    let temp = temp.max(-1.1);

    let rk = 1.894 * (temp + 1.1) * (100. - rh) * ELL01[index] * 1e-4;

    let rk = if lat_adjustment {
        if lat <= 30. && lat > 10. {
            1.894 * (temp + 1.1) * (100. - rh) * ELL02[index] * 1e-4
        } else if lat <= -10. && lat > -30. {
            1.894 * (temp + 1.1) * (100. - rh) * ELL03[index] * 1e-4
        } else if (-90. ..=-30.).contains(&lat) {
            1.894 * (temp + 1.1) * (100. - rh) * ELL04[index] * 1e-4
        } else if lat <= 10. && lat > -10. {
            1.894 * (temp + 1.1) * (100. - rh) * 9. * 1e-4
        } else {
            rk
        }
    } else {
        rk
    };

    let pr = if precip <= 1.5 {
        prev_dmc
    } else {
        let ra = precip;

        let rw = 0.92 * ra - 1.27;
        let wmi = 20. + 280. / E.powf(0.023 * prev_dmc);
        let b = if prev_dmc <= 33. {
            100. / (0.5 + 0.3 * prev_dmc)
        } else if prev_dmc <= 65. {
            14. - 1.3 * prev_dmc.ln()
        } else {
            6.2 * prev_dmc.ln() - 17.2
        };

        let wmr = wmi + 1000. * rw / (48.77 + b * rw);

        43.43 * (5.6348 - (wmr - 20.).ln())
    };

    let pr = pr.max(0.);

    (pr + rk).max(0.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        prev_dmc: f64,
        temp: f64,
        rh: f64,
        precip: f64,
        lat: f64,
        mon: usize,
        lat_adjustment: Option<bool>,
        dmc: f64,
    }

    #[test]
    fn test_duff_moisture_code() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/dmc.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let dmc = duff_moisture_code(
                record.prev_dmc,
                record.temp,
                record.rh,
                record.precip,
                record.lat,
                record.mon,
                record.lat_adjustment,
            );

            assert_eq!(precision_f64(dmc, 4), record.dmc);
        }

        Ok(())
    }
}
