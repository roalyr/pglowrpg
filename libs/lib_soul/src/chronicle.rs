// [Revision: v0.1] [Path: libs/lib_soul/src/chronicle.rs] [Date: 2025-12-13]
// Description: Chronicle - recording history across incarnations

use crate::soul::DeathCause;
use serde::{Deserialize, Serialize};

/// Type of chronicle entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    /// A character's birth/creation.
    Birth,
    /// A character's death.
    Death,
    /// A vow sworn.
    VowSworn,
    /// A vow fulfilled.
    VowFulfilled,
    /// A vow forsaken.
    VowForsaken,
    /// A bond forged.
    BondForged,
    /// A bond broken.
    BondBroken,
    /// A significant battle.
    Battle,
    /// A location discovered.
    Discovery,
    /// A milestone achieved.
    Milestone,
    /// World event (not character specific).
    WorldEvent,
    /// Reincarnation occurred.
    Reincarnation,
}

impl EntryType {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            EntryType::Birth => "Birth",
            EntryType::Death => "Death",
            EntryType::VowSworn => "Vow Sworn",
            EntryType::VowFulfilled => "Vow Fulfilled",
            EntryType::VowForsaken => "Vow Forsaken",
            EntryType::BondForged => "Bond Forged",
            EntryType::BondBroken => "Bond Broken",
            EntryType::Battle => "Battle",
            EntryType::Discovery => "Discovery",
            EntryType::Milestone => "Milestone",
            EntryType::WorldEvent => "World Event",
            EntryType::Reincarnation => "Reincarnation",
        }
    }
}

/// A single entry in the chronicle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEntry {
    /// Type of entry.
    pub entry_type: EntryType,
    /// World turn when this occurred.
    pub turn: u32,
    /// Character name involved (if any).
    pub character_name: Option<String>,
    /// Soul ID involved (if any).
    pub soul_id: Option<u64>,
    /// Location where it occurred.
    pub location: Option<String>,
    /// Title/summary of the entry.
    pub title: String,
    /// Detailed description.
    pub description: String,
    /// Related entity names.
    pub related: Vec<String>,
}

impl ChronicleEntry {
    /// Create a new chronicle entry.
    pub fn new(entry_type: EntryType, turn: u32, title: impl Into<String>) -> Self {
        Self {
            entry_type,
            turn,
            character_name: None,
            soul_id: None,
            location: None,
            title: title.into(),
            description: String::new(),
            related: Vec::new(),
        }
    }
    
    /// Set character name.
    pub fn by_character(mut self, name: impl Into<String>) -> Self {
        self.character_name = Some(name.into());
        self
    }
    
    /// Set soul ID.
    pub fn with_soul(mut self, soul_id: u64) -> Self {
        self.soul_id = Some(soul_id);
        self
    }
    
    /// Set location.
    pub fn at_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
    
    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    /// Add related entity.
    pub fn with_related(mut self, entity: impl Into<String>) -> Self {
        self.related.push(entity.into());
        self
    }
    
    /// Get narrative text for this entry.
    pub fn narrative(&self) -> String {
        let mut text = format!("[Turn {}] ", self.turn);
        
        if let Some(name) = &self.character_name {
            text.push_str(&format!("{}: ", name));
        }
        
        text.push_str(&self.title);
        
        if let Some(loc) = &self.location {
            text.push_str(&format!(" at {}", loc));
        }
        
        text
    }
}

/// The world chronicle recording all significant events.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chronicle {
    /// All entries in chronological order.
    entries: Vec<ChronicleEntry>,
    /// Current world turn.
    current_turn: u32,
}

impl Chronicle {
    /// Create a new empty chronicle.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_turn: 0,
        }
    }
    
    /// Set current turn.
    pub fn set_turn(&mut self, turn: u32) {
        self.current_turn = turn;
    }
    
    /// Advance turn.
    pub fn advance_turn(&mut self) {
        self.current_turn += 1;
    }
    
    /// Get current turn.
    pub fn turn(&self) -> u32 {
        self.current_turn
    }
    
    /// Add an entry.
    pub fn add(&mut self, entry: ChronicleEntry) {
        self.entries.push(entry);
    }
    
    /// Record a character birth.
    pub fn record_birth(&mut self, name: &str, soul_id: u64, location: Option<&str>) {
        let mut entry = ChronicleEntry::new(
            EntryType::Birth,
            self.current_turn,
            format!("{} enters the world", name),
        )
        .by_character(name)
        .with_soul(soul_id);
        
        if let Some(loc) = location {
            entry = entry.at_location(loc);
        }
        
        self.add(entry);
    }
    
    /// Record a character death.
    pub fn record_death(&mut self, name: &str, soul_id: u64, cause: &DeathCause, location: Option<&str>) {
        let mut entry = ChronicleEntry::new(
            EntryType::Death,
            self.current_turn,
            format!("{} meets their end", name),
        )
        .by_character(name)
        .with_soul(soul_id)
        .with_description(cause.description());
        
        if let Some(loc) = location {
            entry = entry.at_location(loc);
        }
        
        self.add(entry);
    }
    
    /// Record a vow sworn.
    pub fn record_vow_sworn(&mut self, name: &str, vow: &str) {
        self.add(ChronicleEntry::new(
            EntryType::VowSworn,
            self.current_turn,
            format!("{} swears: {}", name, vow),
        ).by_character(name));
    }
    
    /// Record a vow fulfilled.
    pub fn record_vow_fulfilled(&mut self, name: &str, vow: &str) {
        self.add(ChronicleEntry::new(
            EntryType::VowFulfilled,
            self.current_turn,
            format!("{} fulfills their vow: {}", name, vow),
        ).by_character(name));
    }
    
    /// Record a bond forged.
    pub fn record_bond(&mut self, name: &str, other: &str) {
        self.add(ChronicleEntry::new(
            EntryType::BondForged,
            self.current_turn,
            format!("{} forges a bond with {}", name, other),
        )
        .by_character(name)
        .with_related(other));
    }
    
    /// Record a battle.
    pub fn record_battle(&mut self, name: &str, enemy: &str, location: Option<&str>, victory: bool) {
        let outcome = if victory { "emerges victorious" } else { "is defeated" };
        let mut entry = ChronicleEntry::new(
            EntryType::Battle,
            self.current_turn,
            format!("{} {} against {}", name, outcome, enemy),
        )
        .by_character(name)
        .with_related(enemy);
        
        if let Some(loc) = location {
            entry = entry.at_location(loc);
        }
        
        self.add(entry);
    }
    
    /// Record a discovery.
    pub fn record_discovery(&mut self, name: &str, discovery: &str) {
        self.add(ChronicleEntry::new(
            EntryType::Discovery,
            self.current_turn,
            format!("{} discovers {}", name, discovery),
        ).by_character(name));
    }
    
    /// Record a reincarnation.
    pub fn record_reincarnation(&mut self, old_name: &str, new_name: &str, soul_id: u64) {
        self.add(ChronicleEntry::new(
            EntryType::Reincarnation,
            self.current_turn,
            format!("The soul of {} is reborn as {}", old_name, new_name),
        )
        .with_soul(soul_id)
        .with_related(old_name)
        .with_related(new_name));
    }
    
    /// Record a world event.
    pub fn record_world_event(&mut self, title: &str, description: &str) {
        self.add(ChronicleEntry::new(
            EntryType::WorldEvent,
            self.current_turn,
            title,
        ).with_description(description));
    }
    
    /// Get all entries.
    pub fn all_entries(&self) -> &[ChronicleEntry] {
        &self.entries
    }
    
    /// Get entries for a specific character.
    pub fn entries_for_character(&self, name: &str) -> Vec<&ChronicleEntry> {
        self.entries.iter()
            .filter(|e| e.character_name.as_deref() == Some(name))
            .collect()
    }
    
    /// Get entries for a specific soul.
    pub fn entries_for_soul(&self, soul_id: u64) -> Vec<&ChronicleEntry> {
        self.entries.iter()
            .filter(|e| e.soul_id == Some(soul_id))
            .collect()
    }
    
    /// Get entries of a specific type.
    pub fn entries_of_type(&self, entry_type: EntryType) -> Vec<&ChronicleEntry> {
        self.entries.iter()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }
    
    /// Get recent entries (last N).
    pub fn recent_entries(&self, count: usize) -> Vec<&ChronicleEntry> {
        self.entries.iter().rev().take(count).collect()
    }
    
    /// Get entries mentioning an entity.
    pub fn entries_mentioning(&self, entity: &str) -> Vec<&ChronicleEntry> {
        self.entries.iter()
            .filter(|e| {
                e.character_name.as_deref() == Some(entity) ||
                e.related.iter().any(|r| r == entity) ||
                e.title.contains(entity) ||
                e.description.contains(entity)
            })
            .collect()
    }
    
    /// Generate narrative summary of a soul's history.
    pub fn soul_narrative(&self, soul_id: u64) -> String {
        let entries = self.entries_for_soul(soul_id);
        
        if entries.is_empty() {
            return "This soul's history is unknown.".to_string();
        }
        
        let mut narrative = String::from("The Chronicle speaks:\n\n");
        
        for entry in entries {
            narrative.push_str(&entry.narrative());
            narrative.push('\n');
        }
        
        narrative
    }
    
    /// Get total entry count.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
