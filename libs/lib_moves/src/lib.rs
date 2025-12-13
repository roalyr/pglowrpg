// [Revision: v0.1] [Path: libs/lib_moves/src/lib.rs] [Date: 2025-12-13]
// Description: Move system with action rolls and outcomes

pub mod action_roll;
pub mod move_outcome;

pub use action_roll::{ActionRoll, ActionRollResult};
pub use move_outcome::{MoveOutcome, MoveType};
