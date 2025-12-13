// [Revision: v0.1] [Path: libs/lib_npc/src/relationship.rs] [Date: 2025-12-13]
// Description: Relationship tracking between player and NPCs

use serde::{Deserialize, Serialize};

/// Level of relationship with an NPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationshipLevel {
    /// Sworn enemies.
    Enemy,
    /// Actively dislikes player.
    Rival,
    /// Has had negative interactions.
    Unfriendly,
    /// No significant relationship.
    Stranger,
    /// Has met and had positive interaction.
    Acquaintance,
    /// Trusted ally.
    Friend,
    /// Deep bond, will risk life.
    Bond,
}

impl RelationshipLevel {
    /// Get bond level as numeric value.
    pub fn value(&self) -> i8 {
        match self {
            RelationshipLevel::Enemy => -3,
            RelationshipLevel::Rival => -2,
            RelationshipLevel::Unfriendly => -1,
            RelationshipLevel::Stranger => 0,
            RelationshipLevel::Acquaintance => 1,
            RelationshipLevel::Friend => 2,
            RelationshipLevel::Bond => 3,
        }
    }
    
    /// Get descriptive name.
    pub fn name(&self) -> &'static str {
        match self {
            RelationshipLevel::Enemy => "Enemy",
            RelationshipLevel::Rival => "Rival",
            RelationshipLevel::Unfriendly => "Unfriendly",
            RelationshipLevel::Stranger => "Stranger",
            RelationshipLevel::Acquaintance => "Acquaintance",
            RelationshipLevel::Friend => "Friend",
            RelationshipLevel::Bond => "Bond",
        }
    }
    
    /// Create from numeric value.
    pub fn from_value(value: i8) -> Self {
        match value {
            v if v <= -3 => RelationshipLevel::Enemy,
            -2 => RelationshipLevel::Rival,
            -1 => RelationshipLevel::Unfriendly,
            0 => RelationshipLevel::Stranger,
            1 => RelationshipLevel::Acquaintance,
            2 => RelationshipLevel::Friend,
            _ => RelationshipLevel::Bond,
        }
    }
}

/// A relationship between player and an NPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// NPC ID.
    pub npc_id: u32,
    /// Current relationship level.
    pub level: RelationshipLevel,
    /// Progress toward next level (0-10).
    pub progress: u8,
    /// Notes about the relationship.
    pub notes: Vec<String>,
}

impl Relationship {
    /// Create a new relationship.
    pub fn new(npc_id: u32) -> Self {
        Self {
            npc_id,
            level: RelationshipLevel::Stranger,
            progress: 0,
            notes: Vec::new(),
        }
    }
    
    /// Improve relationship.
    pub fn improve(&mut self, amount: u8) {
        self.progress += amount;
        while self.progress >= 10 && self.level < RelationshipLevel::Bond {
            self.progress -= 10;
            self.level = RelationshipLevel::from_value(self.level.value() + 1);
        }
        if self.level == RelationshipLevel::Bond {
            self.progress = self.progress.min(10);
        }
    }
    
    /// Worsen relationship.
    pub fn worsen(&mut self, amount: u8) {
        if amount as i16 > self.progress as i16 {
            let overflow = amount - self.progress;
            self.progress = 0;
            if self.level > RelationshipLevel::Enemy {
                self.level = RelationshipLevel::from_value(self.level.value() - 1);
                self.progress = 10u8.saturating_sub(overflow);
            }
        } else {
            self.progress -= amount;
        }
    }
    
    /// Add a note to the relationship.
    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }
    
    /// Check if this is a bond (for Ironsworn mechanics).
    pub fn is_bond(&self) -> bool {
        self.level == RelationshipLevel::Bond
    }
}

/// Collection of player relationships.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationshipTracker {
    relationships: Vec<Relationship>,
}

impl RelationshipTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
        }
    }
    
    /// Get or create relationship with NPC.
    pub fn get_or_create(&mut self, npc_id: u32) -> &mut Relationship {
        if !self.relationships.iter().any(|r| r.npc_id == npc_id) {
            self.relationships.push(Relationship::new(npc_id));
        }
        // Safety: We just ensured the relationship exists above
        self.relationships.iter_mut().find(|r| r.npc_id == npc_id)
            .expect("relationship was just added")
    }
    
    /// Get relationship with NPC.
    pub fn get(&self, npc_id: u32) -> Option<&Relationship> {
        self.relationships.iter().find(|r| r.npc_id == npc_id)
    }
    
    /// Count total bonds.
    pub fn bond_count(&self) -> usize {
        self.relationships.iter().filter(|r| r.is_bond()).count()
    }
    
    /// Get all relationships.
    pub fn all(&self) -> &[Relationship] {
        &self.relationships
    }
    
    /// Get all bonds.
    pub fn bonds(&self) -> Vec<&Relationship> {
        self.relationships.iter().filter(|r| r.is_bond()).collect()
    }
}
