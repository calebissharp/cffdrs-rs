use std::f64::consts::E;

/// Hourly fine fuel moisture code (HFFMC)
///
/// * `temp` - Temperature (Celcius)
/// * `rh` - Relative humidity (%)
/// * `ws` - 10m height wind speed (km/h)
/// * `precip` - 1-hour rainfall (mm)
/// * `previous_hffmc` - Previous hour's HFFMC
/// * `timestep` - Timestep between calculations, in hours (default 1 hour, optional). Note that
/// using a different value than the default may result in inaccuracies
pub fn hourly_fine_fuel_moisture_code(
    temp: f64,
    rh: f64,
    ws: f64,
    precip: f64,
    previous_hffmc: f64,
    timestep: Option<f64>,
) -> f64 {
    let t0 = timestep.unwrap_or(1.);

    let mo = 147.27723 * (101. - previous_hffmc) / (59.5 + previous_hffmc);

    let rf = precip;

    let mr = if mo <= 150. {
        mo + 42.5 * rf * E.powf(-100. / (251. - mo)) * (1. - E.powf(-6.93 / rf))
    } else {
        mo + 42.5 * rf * E.powf(-100. / (251. - mo)) * (1. - E.powf(-6.93 / rf))
            + 0.0015 * ((mo - 150.).powi(2)) * (rf.powf(0.5))
    };

    let mr = mr.min(250.);
    let mo = if precip > 0.0 { mr } else { mo };

    let ed = 0.942 * rh.powf(0.679)
        + 11. * E.powf((rh - 100.) / 10.)
        + 0.18 * (21.1 - temp) * (1. - E.powf(-0.115 * rh));

    let ko =
        0.424 * (1. - (rh / 100.).powf(1.7)) + 0.0694 * ws.powf(0.5) * (1. - (rh / 100.).powi(8));

    let kd = ko * 0.0579 * E.powf(0.0365 * temp);
    let md = ed + (mo - ed) * 10f64.powf(-kd * t0);

    let ew = 0.618 * rh.powf(0.753)
        + 10. * E.powf((rh - 100.) / 10.)
        + 0.18 * (21.1 - temp) * (1. - E.powf(-0.115 * rh));

    let k1 = 0.424 * (1. - ((100. - rh) / 100.).powf(1.7))
        + 0.0694 * ws.powf(0.5) * (1. - ((100. - rh) / 100.).powi(8));

    let kw = k1 * 0.0579 * E.powf(0.0365 * temp);
    let mw = ew - (ew - mo) * 10f64.powf(-kw * t0);

    let m = if mo > ed { md } else { mw };
    let m = if ed >= mo && mo >= ew { mo } else { m };

    (59.5 * (250. - m) / (147.27723 + m)).max(0.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        temp: f64,
        rh: f64,
        ws: f64,
        precip: f64,
        previous_hffmc: f64,
        timestep: Option<f64>,
        hffmc: f64,
    }

    #[test]
    fn test_hourly_fine_fuel_moisture_code() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/hffmc.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for (i, result) in rdr.deserialize().enumerate() {
            let record: TestRow = result?;
            let hffmc = hourly_fine_fuel_moisture_code(
                record.temp,
                record.rh,
                record.ws,
                record.precip,
                record.previous_hffmc,
                record.timestep,
            );

            println!("i:{}", i);
            assert_eq!(precision_f64(hffmc, 4), record.hffmc);
        }

        Ok(())
    }
}
