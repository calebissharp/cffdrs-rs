//! Fire Weather Index (FWI) System
//!
//! This module contains some helper functions for generating all FWI values from weather data.
//!
//! # Examples
//!
//! ```
//! use cffdrs::fwi::system::calculate_hourly;
//! use cffdrs::weather::Weather;
//! use chrono::prelude::*;
//!
//! let weather = Weather {
//!     time: Utc.with_ymd_and_hms(2023, 7, 8, 9, 10, 11).unwrap(),
//!     location: geo::Point::new(-120.34, 50.69),
//!     temp: 28.,
//!     rh: 12.,
//!     ws: 16.,
//!     wd: 95.,
//!     precip: 0.,
//! };
//!
//! let fwi_values = calculate_hourly(&weather, None);
//!
//! assert_eq!(fwi_values.ffmc, 57.90482214144054);
//! assert_eq!(fwi_values.dmc, 56.014192448);
//! assert_eq!(fwi_values.dc, 58.744);
//! assert_eq!(fwi_values.isi, 0.7933315614248974);
//! assert_eq!(fwi_values.bui, 55.44718193037011);
//! assert_eq!(fwi_values.fwi, 2.2766000331952063);
//! ```

mod bui;
mod dc;
mod dmc;
mod fwi;
mod hffmc;
mod isi;
mod system;

pub use bui::*;
pub use dc::*;
pub use dmc::*;
pub use fwi::*;
pub use hffmc::*;
pub use isi::*;
pub use system::calculate_hourly;

/// Default value for FFMC
pub const DEFAULT_FFMC: f64 = 85.;
