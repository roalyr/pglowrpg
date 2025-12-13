// [Revision: v0.1] [Path: libs/lib_soul/src/lib.rs] [Date: 2025-12-13]
// Description: Soul system - reincarnation, memory persistence, legacy effects

pub mod soul;
pub mod soul_mark;
pub mod chronicle;
pub mod fate_thread;

pub use soul::{Soul, SoulState, DeathCause};
pub use soul_mark::{SoulMark, MarkType, MarkStrength};
pub use chronicle::{Chronicle, ChronicleEntry, EntryType};
pub use fate_thread::{FateThread, ThreadStrength};
