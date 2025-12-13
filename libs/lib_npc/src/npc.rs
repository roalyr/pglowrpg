// [Revision: v0.1] [Path: libs/lib_npc/src/npc.rs] [Date: 2025-12-13]
// Description: Core NPC data structures

use serde::{Deserialize, Serialize};

/// NPC disposition toward the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcDisposition {
    /// Actively hostile, will attack.
    Hostile,
    /// Unfriendly, distrustful.
    Unfriendly,
    /// Wary, cautious.
    Wary,
    /// No strong feelings either way.
    Neutral,
    /// Somewhat positive.
    Curious,
    /// Well-disposed toward player.
    Friendly,
    /// Strongly positive, will help.
    Helpful,
}

impl NpcDisposition {
    /// Convert from string (e.g., from oracle).
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            s if s.contains("hostile") => NpcDisposition::Hostile,
            s if s.contains("unfriendly") => NpcDisposition::Unfriendly,
            s if s.contains("wary") => NpcDisposition::Wary,
            s if s.contains("curious") => NpcDisposition::Curious,
            s if s.contains("friendly") => NpcDisposition::Friendly,
            s if s.contains("helpful") => NpcDisposition::Helpful,
            _ => NpcDisposition::Neutral,
        }
    }
    
    /// Get modifier for social interactions.
    pub fn interaction_modifier(&self) -> i8 {
        match self {
            NpcDisposition::Hostile => -3,
            NpcDisposition::Unfriendly => -2,
            NpcDisposition::Wary => -1,
            NpcDisposition::Neutral => 0,
            NpcDisposition::Curious => 1,
            NpcDisposition::Friendly => 2,
            NpcDisposition::Helpful => 3,
        }
    }
    
    /// Display string for this disposition.
    pub fn description(&self) -> &'static str {
        match self {
            NpcDisposition::Hostile => "Hostile",
            NpcDisposition::Unfriendly => "Unfriendly",
            NpcDisposition::Wary => "Wary",
            NpcDisposition::Neutral => "Neutral",
            NpcDisposition::Curious => "Curious",
            NpcDisposition::Friendly => "Friendly",
            NpcDisposition::Helpful => "Helpful",
        }
    }
}

/// Role or archetype of an NPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcRole {
    /// Common villager or citizen.
    Commoner,
    /// Merchant or trader.
    Merchant,
    /// Warrior or guard.
    Warrior,
    /// Priest, healer, or mystic.
    Mystic,
    /// Criminal or outlaw.
    Outlaw,
    /// Noble or leader.
    Noble,
    /// Traveler or wanderer.
    Traveler,
    /// Craftsman or artisan.
    Artisan,
    /// Scholar or sage.
    Scholar,
    /// Hunter or ranger.
    Hunter,
}

impl NpcRole {
    /// Display name for this role.
    pub fn name(&self) -> &'static str {
        match self {
            NpcRole::Commoner => "Commoner",
            NpcRole::Merchant => "Merchant",
            NpcRole::Warrior => "Warrior",
            NpcRole::Mystic => "Mystic",
            NpcRole::Outlaw => "Outlaw",
            NpcRole::Noble => "Noble",
            NpcRole::Traveler => "Traveler",
            NpcRole::Artisan => "Artisan",
            NpcRole::Scholar => "Scholar",
            NpcRole::Hunter => "Hunter",
        }
    }
}

/// A non-player character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    /// Unique identifier.
    pub id: u32,
    /// Display name.
    pub name: String,
    /// Character role/archetype.
    pub role: NpcRole,
    /// Current disposition toward player.
    pub disposition: NpcDisposition,
    /// Personal goal or motivation.
    pub goal: String,
    /// Optional faction membership.
    pub faction_id: Option<u32>,
    /// Whether player has met this NPC.
    pub met: bool,
    /// Whether NPC is alive.
    pub alive: bool,
    /// Location ID where NPC resides.
    pub location_id: Option<u32>,
}

impl Npc {
    /// Create a new NPC.
    pub fn new(id: u32, name: String, role: NpcRole) -> Self {
        Self {
            id,
            name,
            role,
            disposition: NpcDisposition::Neutral,
            goal: String::new(),
            faction_id: None,
            met: false,
            alive: true,
            location_id: None,
        }
    }
    
    /// Set NPC's goal.
    pub fn with_goal(mut self, goal: impl Into<String>) -> Self {
        self.goal = goal.into();
        self
    }
    
    /// Set NPC's disposition.
    pub fn with_disposition(mut self, disposition: NpcDisposition) -> Self {
        self.disposition = disposition;
        self
    }
    
    /// Set NPC's faction.
    pub fn with_faction(mut self, faction_id: u32) -> Self {
        self.faction_id = Some(faction_id);
        self
    }
    
    /// Set NPC's location.
    pub fn with_location(mut self, location_id: u32) -> Self {
        self.location_id = Some(location_id);
        self
    }
    
    /// Mark NPC as met.
    pub fn meet(&mut self) {
        self.met = true;
    }
    
    /// Shift disposition by amount (clamped to valid range).
    pub fn shift_disposition(&mut self, amount: i8) {
        let current = match self.disposition {
            NpcDisposition::Hostile => 0,
            NpcDisposition::Unfriendly => 1,
            NpcDisposition::Wary => 2,
            NpcDisposition::Neutral => 3,
            NpcDisposition::Curious => 4,
            NpcDisposition::Friendly => 5,
            NpcDisposition::Helpful => 6,
        };
        
        let new_value = (current as i8 + amount).clamp(0, 6) as u8;
        
        self.disposition = match new_value {
            0 => NpcDisposition::Hostile,
            1 => NpcDisposition::Unfriendly,
            2 => NpcDisposition::Wary,
            3 => NpcDisposition::Neutral,
            4 => NpcDisposition::Curious,
            5 => NpcDisposition::Friendly,
            _ => NpcDisposition::Helpful,
        };
    }
    
    /// Check if NPC is hostile.
    pub fn is_hostile(&self) -> bool {
        matches!(self.disposition, NpcDisposition::Hostile | NpcDisposition::Unfriendly)
    }
    
    /// Check if NPC is friendly.
    pub fn is_friendly(&self) -> bool {
        matches!(self.disposition, NpcDisposition::Friendly | NpcDisposition::Helpful)
    }
}
