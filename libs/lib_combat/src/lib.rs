// [Revision: v0.1] [Path: libs/lib_combat/src/lib.rs] [Date: 2025-12-13]
// Description: Combat system - enemies, encounters, combat state

pub mod enemy;
pub mod encounter;
pub mod combat_state;

pub use enemy::{Enemy, EnemyRank, EnemyType, EnemyBehavior};
pub use encounter::{Encounter, EncounterType};
pub use combat_state::{CombatState, CombatPhase, CombatAction};
