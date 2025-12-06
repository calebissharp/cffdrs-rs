use std::f64::consts::FRAC_PI_2;

/// Calculate rate of spread (ROS) at the perimeter of an elliptically shaped fire
/// at angle theta
///
/// `ros` - Fire rate of spread (see [crate::fbp::ros::rate_of_spread()])
/// `fros` - Flank fire rate of spread (see [crate::fbp::ros::flank_rate_of_spread()])
/// `bros` - Back fire rate of spread
/// `theta` - Angle in radians from fire direction of spread
///
/// Returns rate of spread at angle theta (m/min)
///
/// # Examples
/// ```
/// # use cffdrs::fbp::ros::{rate_of_spread_at_theta};
/// let ros = 34.02;
/// let fros = 393.66;
/// let bros = 590.49;
/// let theta = 90_f64.to_radians();
///
/// let ros_east = rate_of_spread_at_theta(ros, fros, bros, theta);
/// assert_eq!(ros_east, 178.72808229830116);
///
/// assert_eq!(rate_of_spread_at_theta(464.13, 196.83, 0.0, 230.49_f64.to_radians()), 0.0);
/// assert_eq!(rate_of_spread_at_theta(349.92, 590.49, 393.66, -163.17_f64.to_radians()), 403.5940759749405);
/// assert_eq!(rate_of_spread_at_theta(58.32, 10.0, 590.49, -360_f64.to_radians()), 58.32000000000005);
/// ```
pub fn rate_of_spread_at_theta(ros: f64, fros: f64, bros: f64, theta: f64) -> f64 {
    let theta = if theta == FRAC_PI_2 || theta == 3. * FRAC_PI_2 {
        theta + 0.0001
    } else {
        theta
    };
    let c1 = theta.cos();
    let s1 = theta.sin();

    // Eq. 94 (https://cfs.nrcan.gc.ca/pubwarehouse/pdfs/31414.pdf)

    (ros - bros) / (2. * c1)
        + ((ros + bros) / (2. * c1))
            * ((fros * c1 * (fros.powi(2) * c1.powi(2) + (ros * bros) * s1.powi(2)).sqrt()
                - (((ros.powi(2) - bros.powi(2)) / 4.) * s1.powi(2)))
                / (fros.powi(2) * c1.powi(2) + ((ros + bros) / 2.0).powi(2) * s1.powi(2)))
}

#[cfg(test)]
mod tests {
    use crate::test_util::precision_f64;

    use super::*;

    #[derive(Debug, serde::Deserialize)]
    struct TestRow {
        ros: f64,
        fros: f64,
        bros: f64,
        theta: f64,
        ros_theta: f64,
    }

    #[test]
    fn test_rate_of_spread_at_theta() -> Result<(), Box<dyn std::error::Error>> {
        let fixture = std::fs::File::open("./tests/fixtures/ros_at_theta.csv")?;
        let mut rdr = csv::Reader::from_reader(fixture);

        for result in rdr.deserialize() {
            let record: TestRow = result?;
            let ros = rate_of_spread_at_theta(
                record.ros,
                record.fros,
                record.bros,
                record.theta.to_radians(),
            );

            assert_eq!(precision_f64(ros, 4), record.ros_theta);
        }

        Ok(())
    }
}
