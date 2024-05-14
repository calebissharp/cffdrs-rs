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
/// # use cffdrs::fbp::{flank_rate_of_spread};
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
