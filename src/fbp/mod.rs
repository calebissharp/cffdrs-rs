//! Contains calculations for the Fire Behaviour Prediction (FBP) system

mod buildup_effect;
mod crown_base_height;
mod crown_fraction_burned;
mod crown_fuel_consumption;
mod crown_fuel_load;
mod fire_intensity;
mod foliar_moisture_content;
mod length_to_breadth;
mod slope_adjustment;
mod surface_fuel_consumption;
mod total_fuel_consumption;

pub mod ros;

pub use buildup_effect::*;
pub use crown_base_height::*;
pub use crown_fraction_burned::*;
pub use crown_fuel_consumption::*;
pub use crown_fuel_load::*;
pub use fire_intensity::*;
pub use foliar_moisture_content::*;
pub use length_to_breadth::*;
pub use slope_adjustment::*;
pub use surface_fuel_consumption::*;
pub use total_fuel_consumption::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum FbpFuelType {
    /// Spruce-Lichen Woodland
    C1,
    /// Boreal Spruce
    C2,
    /// Mature Jack or Lodgepole Pine
    C3,
    /// Immature Jack or Lodgepole Pine
    C4,
    /// Red and White Pine
    C5,
    /// Conifer Plantation
    C6,
    /// Ponderosa Pine-Douglas-fir
    C7,
    /// Boreal Mixedwood
    M1,
    /// Boreal Mixedwood
    M2,
    /// Dead Balsam Fir Mixedwood
    M3,
    /// Dead Balsam Fir Mixedwood
    M4,
    /// Leafless Aspen
    D1,
    /// Jack or Lodgepole Pine Slash
    S1,
    /// White Spruce-Balsam Slash
    S2,
    /// Coastal Cedar-Hemlock-Douglas fir Slash
    S3,
    /// Grass
    O1a,
    /// Grass
    O1b,
    /// Non-fuel
    NonFuel,
}
