use std::f64::consts::E;

use super::FbpFuelType;

/// Calculate head spread distance of a fire at `time`
///
/// * `ros` - Rate of spread (m/min)
/// * `time` - Elapsed time (min)
/// * `cfb` - Crow fraction burned
pub fn distance_at_time(fuel_type: FbpFuelType, ros: f64, time: f64, cfb: f64) -> f64 {
    let alpha = match fuel_type {
        FbpFuelType::C1
        | FbpFuelType::O1a
        | FbpFuelType::O1b
        | FbpFuelType::S1
        | FbpFuelType::S2
        | FbpFuelType::S3
        | FbpFuelType::D1 => 0.115,
        _ => 0.115 - 18.8 * cfb.powf(2.5) * E.powf(-8. * cfb),
    };

    ros * (time + E.powf(-alpha * time) / alpha - 1. / alpha)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        fuel_type: FbpFuelType,
        ros: f64,
        time: f64,
        cfb: f64,
        distance: f64,
    }

    #[test]
    fn test_distance_at_time() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/distance_at_time.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let distance = distance_at_time(record.fuel_type, record.ros, record.time, record.cfb);

            assert_eq!(precision_f64(distance, 4), record.distance);
        }

        Ok(())
    }
}
