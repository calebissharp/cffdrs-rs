use std::f64::consts::E;

/// Calculate fire weather index (FWI)
///
/// * `isi` - Initial spread index (see [initial_spread_index](crate::fwi_system::initial_spread_index))
/// * `bui` - Buildup index (see [buildup_index](crate::fwi_system::buildup_index))
pub fn fire_weather_index(isi: f64, bui: f64) -> f64 {
    let bb = if bui > 80. {
        0.1 * isi * (1000. / (25. + 108.64 / E.powf(0.023 * bui)))
    } else {
        0.1 * isi * (0.626 * bui.powf(0.809) + 2.)
    };

    if bb <= 1. {
        bb
    } else {
        E.powf(2.72 * (0.434 * bb.ln()).powf(0.647))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        isi: f64,
        bui: f64,
        fwi: f64,
    }

    #[test]
    fn test_fire_weather_index() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/fwi.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let fwi = fire_weather_index(record.isi, record.bui);

            assert_eq!(precision_f64(fwi, 4), record.fwi);
        }

        Ok(())
    }
}
