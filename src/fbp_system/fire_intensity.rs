/// Fire intensity (FI)
///
/// * `fc` - Fuel consumption, either surface or total (kg/m^2) - See
///   [total_fuel_consumption()][crate::fbp_system::total_fuel_consumption()] and [surface_fuel_consumption()][crate::fbp_system::surface_fuel_consumption()]
/// * `ros` - Rate of spread (m/min)
///
/// Returns FI (kW/m)
///
pub fn fire_intensity(fc: f64, ros: f64) -> f64 {
    300. * fc * ros
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        fc: f64,
        ros: f64,
        fi: f64,
    }

    #[test]
    fn test_fire_intensity() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/fire_intensity.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let fi = fire_intensity(record.fc, record.ros);

            assert_eq!(precision_f64(fi, 4), record.fi);
        }

        Ok(())
    }
}
