// [Revision: v0.1] [Path: libs/lib_npc/src/faction.rs] [Date: 2025-12-13]
// Description: Faction system for NPC groups and organizations

use serde::{Deserialize, Serialize};

/// Type of faction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionType {
    /// Local village or settlement.
    Settlement,
    /// Religious or spiritual organization.
    Religious,
    /// Military or martial group.
    Military,
    /// Merchant guild or trade organization.
    Merchant,
    /// Criminal organization.
    Criminal,
    /// Noble house or ruling family.
    Noble,
    /// Scholarly or academic group.
    Academic,
    /// Tribal or clan-based group.
    Tribal,
    /// Monster or hostile faction.
    Monster,
}

/// A faction or organization in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// Unique identifier.
    pub id: u32,
    /// Display name.
    pub name: String,
    /// Description of the faction.
    pub description: String,
    /// Type of faction.
    pub faction_type: FactionType,
    /// Player reputation with this faction (-100 to +100).
    pub reputation: i16,
    /// Whether faction is known to player.
    pub discovered: bool,
}

impl Faction {
    /// Create a new faction.
    pub fn new(id: u32, name: String, faction_type: FactionType) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            faction_type,
            reputation: 0,
            discovered: false,
        }
    }
    
    /// Set faction description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    
    /// Modify reputation.
    pub fn modify_reputation(&mut self, amount: i16) {
        self.reputation = (self.reputation + amount).clamp(-100, 100);
    }
    
    /// Get reputation level as descriptive string.
    pub fn reputation_level(&self) -> &'static str {
        match self.reputation {
            -100..=-61 => "Hated",
            -60..=-21 => "Hostile",
            -20..=-1 => "Unfriendly",
            0..=19 => "Neutral",
            20..=59 => "Friendly",
            60..=99 => "Honored",
            100 => "Exalted",
            _ => "Unknown",
        }
    }
    
    /// Check if faction is hostile.
    pub fn is_hostile(&self) -> bool {
        self.reputation < -20
    }
    
    /// Check if faction is friendly.
    pub fn is_friendly(&self) -> bool {
        self.reputation >= 20
    }
    
    /// Discover this faction.
    pub fn discover(&mut self) {
        self.discovered = true;
    }
}

/// Collection of factions in the world.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactionRegistry {
    factions: Vec<Faction>,
    next_id: u32,
}

impl FactionRegistry {
    /// Create a new faction registry.
    pub fn new() -> Self {
        Self {
            factions: Vec::new(),
            next_id: 1,
        }
    }
    
    /// Add a new faction.
    pub fn add(&mut self, name: String, faction_type: FactionType) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.factions.push(Faction::new(id, name, faction_type));
        id
    }
    
    /// Get faction by ID.
    pub fn get(&self, id: u32) -> Option<&Faction> {
        self.factions.iter().find(|f| f.id == id)
    }
    
    /// Get mutable faction by ID.
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Faction> {
        self.factions.iter_mut().find(|f| f.id == id)
    }
    
    /// Get all discovered factions.
    pub fn discovered(&self) -> Vec<&Faction> {
        self.factions.iter().filter(|f| f.discovered).collect()
    }
    
    /// Get all factions.
    pub fn all(&self) -> &[Faction] {
        &self.factions
    }
}
