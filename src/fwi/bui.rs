/// Calculate Buildup Index (BUI)
///
/// * `dmc` - Duff moisture code
/// * `dc` - Drought code
pub fn buildup_index(dmc: f64, dc: f64) -> f64 {
    let bui1 = if dmc == 0. && dc == 0. {
        0.
    } else {
        0.8 * dc * dmc / (dmc + 0.4 * dc)
    };

    let p = if dmc == 0. { 0. } else { (dmc - bui1) / dmc };
    let cc = 0.92 + (0.0114 * dmc).powf(1.7);

    let bui0 = (dmc - cc * p).max(0.);

    if bui1 < dmc {
        bui0
    } else {
        bui1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        dmc: f64,
        dc: f64,
        bui: f64,
    }

    #[test]
    fn test_buildup_index() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/bui.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let bui = buildup_index(record.dmc, record.dc);

            assert_eq!(precision_f64(bui, 4), record.bui);
        }

        Ok(())
    }
}
