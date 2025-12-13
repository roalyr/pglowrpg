// [Revision: v0.1] [Path: libs/lib_npc/src/lib.rs] [Date: 2025-12-13]
// Description: NPC system - characters, factions, relationships

pub mod npc;
pub mod faction;
pub mod relationship;
pub mod generator;

pub use npc::{Npc, NpcDisposition, NpcRole};
pub use faction::{Faction, FactionType};
pub use relationship::{Relationship, RelationshipLevel};
pub use generator::NpcGenerator;
