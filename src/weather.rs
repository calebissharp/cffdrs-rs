//! Weather types and functions

use chrono::{DateTime, Utc};
use geo::Point;

/// A weather record representing the weather for a single point-in-time, at a location
#[derive(Debug, Clone)]
pub struct Weather {
    /// UTC time when the weather was sampled
    pub time: DateTime<Utc>,
    /// Location the weather was sampled at
    pub location: Point<f64>,
    /// Temperature (Celcius)
    pub temp: f64,
    /// Relative humidity (%)
    pub rh: f64,
    /// Wind speed (at 10m height, km/h)
    pub ws: f64,
    /// Wind direction (degrees, 0 = wind from north, 90 = from east, etc.)
    pub wd: f64,
    /// Precipitation (mm)
    pub precip: f64,
}
