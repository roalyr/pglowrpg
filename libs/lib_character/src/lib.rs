// [Revision: v0.1] [Path: libs/lib_character/src/lib.rs] [Date: 2025-12-13]
// Description: Character system - stats, resources, vows, bonds

pub mod stats;
pub mod resources;
pub mod vows;
pub mod character;

pub use character::Character;
pub use stats::{Stats, Stat};
pub use resources::{Resources, Momentum};
pub use vows::{Vow, VowRank, Bond};
