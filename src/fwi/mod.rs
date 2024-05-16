//! Fire Weather Index (FWI) System

mod hffmc;
mod isi;

pub use hffmc::*;
pub use isi::*;

/// Default value for FFMC
pub const DEFAULT_FFMC: f64 = 85.;
