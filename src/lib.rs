pub mod fbp_system;
pub mod fwi;
pub mod weather;

#[cfg(test)]
pub mod test_util {
    /// Round to significant digits (rather than digits after the decimal).
    ///
    /// Not implemented for `f32`, because such an implementation showed precision
    /// glitches (e.g. `precision_f32(12300.0, 2) == 11999.999`), so for `f32`
    /// floats, convert to `f64` for this function and back as needed.
    ///
    /// Examples:
    /// ```
    ///   precision_f64(1.2300, 2)                      // 1.2<f64>
    ///   precision_f64(1.2300_f64, 2)                  // 1.2<f64>
    ///   precision_f64(1.2300_f32 as f64, 2)           // 1.2<f64>
    ///   precision_f64(1.2300_f32 as f64, 2) as f32    // 1.2<f32>
    /// ```
    pub fn precision_f64(x: f64, decimals: u32) -> f64 {
        if x == 0. || decimals == 0 {
            0.
        } else {
            let decimals = decimals.max(x.abs().log10().ceil() as u32 + 2);
            let shift = decimals as i32 - x.abs().log10().ceil() as i32;
            let shift_factor = 10_f64.powi(shift);

            (x * shift_factor).round() / shift_factor
        }
    }
}
