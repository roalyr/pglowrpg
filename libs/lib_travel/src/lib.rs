// [Revision: v0.1] [Path: libs/lib_travel/src/lib.rs] [Date: 2025-12-13]
// Description: Travel system - journeys, exploration, encounters

pub mod journey;
pub mod encounter;

pub use journey::{Journey, JourneyStatus};
pub use encounter::{Encounter, EncounterType};
