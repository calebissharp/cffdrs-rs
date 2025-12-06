use std::f64::consts::E;

/// Day length factor, north of 20 degrees N
const FL01: [f64; 12] = [
    -1.6, -1.6, -1.6, 0.9, 3.8, 5.8, 6.4, 5., 2.4, 0.4, -1.6, -1.6,
];
/// Day length factor, south of 20 degrees S
const FL02: [f64; 12] = [
    6.4, 5., 2.4, 0.4, -1.6, -1.6, -1.6, -1.6, -1.6, 0.9, 3.8, 5.8,
];

/// Calculate Drought Code (DC)
///
/// * `prev_dc` - The previous iteration's drought code
/// * `temp` - Temperature (Celcius)
/// * `precip` - Precipitation (mm)
/// * `lat` - Latitutde (decimal degrees)
/// * `mon` - Month (1-12)
/// * `lat_adjustment` - Whether to apply latitude adjustment (optional, default = `true`)
pub fn drought_code(
    prev_dc: f64,
    temp: f64,
    precip: f64,
    lat: f64,
    mon: usize,
    lat_adjustment: Option<bool>,
) -> f64 {
    let lat_adjustment = lat_adjustment.unwrap_or(true);

    let temp = temp.max(-2.8);

    let index = (mon).clamp(1, 12) - 1;

    let pe = (0.36 * (temp + 2.8) + FL01[index]) / 2.;

    let pe = if lat_adjustment {
        if lat <= -20. {
            (0.36 * (temp + 2.8) + FL02[index]) / 2.
        } else if lat > -20. && lat <= 20. {
            (0.36 * (temp + 2.8) + 1.4) / 2.
        } else {
            pe
        }
    } else {
        pe
    };

    let pe = pe.max(0.);

    let ra = precip;
    let rw = 0.83 * ra - 1.27;

    let smi = 800. * E.powf(-prev_dc / 400.);

    let dr0 = (prev_dc - 400. * (1. + 3.937 * rw / smi).ln()).max(0.);

    let dr = if precip <= 2.8 { prev_dc } else { dr0 };

    (dr + pe).max(0.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        prev_dc: f64,
        temp: f64,
        precip: f64,
        lat: f64,
        mon: usize,
        lat_adjustment: Option<bool>,
        dc: f64,
    }

    #[test]
    fn test_drought_code() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/dc.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let dc = drought_code(
                record.prev_dc,
                record.temp,
                record.precip,
                record.lat,
                record.mon,
                record.lat_adjustment,
            );

            assert_eq!(precision_f64(dc, 4), record.dc);
        }

        Ok(())
    }
}
