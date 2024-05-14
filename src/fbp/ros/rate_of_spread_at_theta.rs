/// Calculate rate of spread (ROS) at the perimeter of an elliptically shaped fire
/// at angle theta
///
/// `ros` - Fire rate of spread (see [crate::fbp::ros::rate_of_spread()])
/// `fros` - Flank fire rate of spread (see [crate::fbp::ros::flank_rate_of_spread()])
/// `bros` - Back fire rate of spread
/// `theta` - Angle in degrees, 0 = N, 90 = E, etc
///
/// Returns rate of spread at angle theta (m/min)
///
/// # Examples
/// ```
/// # use cffdrs::fbp::{rate_of_spread_at_theta};
/// let ros = 34.02;
/// let fros = 393.66;
/// let bros = 590.49;
/// let theta = 33.66;
///
/// let ros_east = rate_of_spread_at_theta(ros, fros, bros, theta);
/// assert_eq!(ros_east, 5.92207724376657);
///
/// assert_eq!(rate_of_spread_at_theta(464.13, 196.83, 0.0, 230.49), 1145.7899421765774);
/// assert_eq!(rate_of_spread_at_theta(349.92, 590.49, 393.66, -163.17), 354.1664343281862);
/// assert_eq!(rate_of_spread_at_theta(58.32, 0.0, 590.49, -360.), -168.618286063626);
/// ```
pub fn rate_of_spread_at_theta(ros: f64, fros: f64, bros: f64, theta: f64) -> f64 {
    let c1 = theta.cos();
    let s1 = theta.sin();
    let c1 = if c1 == 0. { (theta + 0.001).cos() } else { c1 };

    // Eq. 94 (https://cfs.nrcan.gc.ca/pubwarehouse/pdfs/31414.pdf)

    ((ros - bros) / (2. * c1) + ((ros + bros) / (2. * c1)))
        * ((fros * c1 * (fros.powi(2) * c1.powi(2) + (ros * bros) * s1.powi(2)).sqrt()
            - (((ros.powi(2) - bros.powi(2)) / 4.) * s1.powi(2)))
            / (fros.powi(2) * c1.powi(2) + ((ros + bros) / 2.0).powi(2) * s1.powi(2)))
}
