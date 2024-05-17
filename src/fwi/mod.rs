//! Fire Weather Index (FWI) System

mod bui;
mod dc;
mod dmc;
mod fwi;
mod hffmc;
mod isi;

pub use bui::*;
pub use dc::*;
pub use dmc::*;
pub use fwi::*;
pub use hffmc::*;
pub use isi::*;

/// Default value for FFMC
pub const DEFAULT_FFMC: f64 = 85.;
