// [Revision: v0.2] [Path: libs/lib_moves/src/lib.rs] [Date: 2025-12-13]
// Description: Move system with action rolls, outcomes, and full move catalog

pub mod action_roll;
pub mod move_outcome;
pub mod moves;

pub use action_roll::{ActionRoll, ActionRollResult, ProgressRoll};
pub use move_outcome::{MoveOutcome, MoveType};
pub use moves::{
    adventure_moves, combat_moves, fate_moves, quest_moves, relationship_moves, suffer_moves,
};
