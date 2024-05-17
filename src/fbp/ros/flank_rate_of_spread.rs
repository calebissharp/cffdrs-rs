/// Calculate flank rate of spread (FROS)
///
/// * `ros` - fire rate of spread (m/min)
/// * `bros` - back fire rate of spread (m/min)
/// * `lb` - length to breadth ratio (See [crate::fbp::length_to_breadth()])
///
/// Returns flank fire spread rate (m/min)
///
/// # Examples
///
/// ```
/// # use cffdrs::fbp::ros::{flank_rate_of_spread};
/// let ros = 332.91;
/// let bros = 0.0;
/// let lb = -1.;
///
/// let fros = flank_rate_of_spread(ros, bros, lb);
/// assert_eq!(fros, -166.455);
///
/// assert_eq!(flank_rate_of_spread(393.66, 196.83, -1.), -295.245);
/// assert_eq!(flank_rate_of_spread(274.59, 393.66, 0.62), 538.9112903225806);
/// ```
pub fn flank_rate_of_spread(ros: f64, bros: f64, lb: f64) -> f64 {
    (ros + bros) / lb / 2.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::precision_f64;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        ros: f64,
        bros: f64,
        lb: f64,
        fros: f64,
    }

    #[test]
    fn test_flank_rate_of_spread() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/flank_rate_of_spread.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let fros = flank_rate_of_spread(record.ros, record.bros, record.lb);

            assert_eq!(precision_f64(fros, 4), record.fros);
        }

        Ok(())
    }
}
