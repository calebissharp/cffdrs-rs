//! Fire Weather Index (FWI) System
//!
//! This module contains some helper functions for generating all FWI values from weather data.

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
pub use system::*;

/// Default value for FFMC
pub const DEFAULT_FFMC: f64 = 85.;
