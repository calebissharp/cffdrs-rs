use super::FbpFuelType;

/// Crown fuel consumption (CFC)
///
/// * `cfl` - Crown fuel load (km/m^2) - See [crown_fuel_load()][crate::fbp_system::crown_fuel_load()]
/// * `cfb` - Crown fraction burned (0-1) - See [crown_fraction_burned()][crate::fbp_system::crown_fraction_burned()]
/// * `pc` - Percent conifer (%)
/// * `pdf` - Percent dead balsam fir (%)
///
/// Returns CFC (kg/m^2)
///
pub fn crown_fuel_consumption(
    fuel_type: FbpFuelType,
    cfl: f64,
    cfb: f64,
    pc: f64,
    pdf: f64,
) -> f64 {
    let cfc = cfl * cfb;

    match fuel_type {
        FbpFuelType::M1 | FbpFuelType::M2 => pc / 100. * cfc,
        FbpFuelType::M3 | FbpFuelType::M4 => pdf / 100. * cfc,
        _ => cfc,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        fuel_type: FbpFuelType,
        cfl: f64,
        cfb: f64,
        pc: f64,
        pdf: f64,
        cfc: f64,
    }

    #[test]
    fn test_crown_fuel_consumption() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/crown_fuel_consumption.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let cfc = crown_fuel_consumption(
                record.fuel_type,
                record.cfl,
                record.cfb,
                record.pc,
                record.pdf,
            );

            assert_eq!(precision_f64(cfc, 4), record.cfc);
        }

        Ok(())
    }
}
