use std::f64::consts;

/// Calculate initial spread index (ISI)
///
/// * `ffmc` - Fine fuel moisture code
/// * `ws` - Wind speed (km/h)
///
/// Returns ISI
///
/// ```
/// # use cffdrs::fwi::initial_spread_index;
/// let isi = initial_spread_index(0.6, 0.0);
/// assert_eq!(isi, 2.9864200773278824e-9);
///
/// assert_eq!(initial_spread_index(9.6, 0.0), 1.0478636061736632e-6);
/// assert_eq!(initial_spread_index(26.4, 24.3), 0.0040652916624135445);
/// assert_eq!(initial_spread_index(58.8, 24.3), 1.2853148812949597);
/// assert_eq!(initial_spread_index(7.2, 48.6), 3.049654948160232e-6);
/// assert_eq!(initial_spread_index(39.3, 72.9), 1.1899890130951611);
/// ```
pub fn initial_spread_index(ffmc: f64, ws: f64) -> f64 {
    // Moisture content
    let fm = 147.27723 * (101. - ffmc) / (59.5 + ffmc);

    // Wind effect
    let fw = consts::E.powf(0.05039 * ws);

    // Fine fuel moisture
    let ff = 91.9 * consts::E.powf(-0.1386 * fm) * (1. + (fm.powf(5.31) / 49300000.));

    // Spread index
    0.208 * fw * ff
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        ffmc: f64,
        ws: f64,
        isi: f64,
    }

    #[test]
    fn test_initial_spread_index() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/isi.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let isi = initial_spread_index(record.ffmc, record.ws);

            assert_eq!(precision_f64(isi, 4), record.isi);
        }

        Ok(())
    }
}
