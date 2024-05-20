//! Rate of spread (ROS) related calculations

mod back_rate_of_spread;
mod flank_rate_of_spread;
mod rate_of_spread;
mod rate_of_spread_at_theta;

pub use back_rate_of_spread::*;
pub use flank_rate_of_spread::*;
pub use rate_of_spread::*;
pub use rate_of_spread_at_theta::*;
