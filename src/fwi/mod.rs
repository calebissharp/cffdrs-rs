//! Fire Weather Index (FWI) System

mod bui;
mod dc;
mod hffmc;
mod isi;

pub use bui::*;
pub use dc::*;
pub use hffmc::*;
pub use isi::*;

/// Default value for FFMC
pub const DEFAULT_FFMC: f64 = 85.;
