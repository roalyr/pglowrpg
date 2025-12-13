// [Revision: v0.1] [Path: libs/lib_character/src/stats.rs] [Date: 2025-12-13]
// Description: Character stats (Edge, Heart, Iron, Shadow, Wits)

use serde::{Deserialize, Serialize};

/// The five core stats in Ironsworn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stat {
    /// Quickness, agility, and prowess in combat.
    Edge,
    /// Courage, willpower, empathy, and social skills.
    Heart,
    /// Physical strength, endurance, and resistance.
    Iron,
    /// Sneaking, deception, and cunning.
    Shadow,
    /// Expertise, observation, and mental acuity.
    Wits,
}

/// Character stats with values 1-3 (standard Ironsworn range).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stats {
    pub edge: u8,
    pub heart: u8,
    pub iron: u8,
    pub shadow: u8,
    pub wits: u8,
}

impl Stats {
    /// Create new stats with default values (all 1).
    pub fn new() -> Self {
        Self {
            edge: 1,
            heart: 1,
            iron: 1,
            shadow: 1,
            wits: 1,
        }
    }

    /// Create stats with custom values. Values are clamped to 1-3.
    pub fn with_values(edge: u8, heart: u8, iron: u8, shadow: u8, wits: u8) -> Self {
        Self {
            edge: edge.clamp(1, 3),
            heart: heart.clamp(1, 3),
            iron: iron.clamp(1, 3),
            shadow: shadow.clamp(1, 3),
            wits: wits.clamp(1, 3),
        }
    }

    /// Get the value of a specific stat.
    pub fn get(&self, stat: Stat) -> u8 {
        match stat {
            Stat::Edge => self.edge,
            Stat::Heart => self.heart,
            Stat::Iron => self.iron,
            Stat::Shadow => self.shadow,
            Stat::Wits => self.wits,
        }
    }

    /// Set the value of a specific stat (clamped to 1-3).
    pub fn set(&mut self, stat: Stat, value: u8) {
        let clamped = value.clamp(1, 3);
        match stat {
            Stat::Edge => self.edge = clamped,
            Stat::Heart => self.heart = clamped,
            Stat::Iron => self.iron = clamped,
            Stat::Shadow => self.shadow = clamped,
            Stat::Wits => self.wits = clamped,
        }
    }

    /// Validate that total stat points equal expected value (default 11).
    pub fn validate_total(&self, expected: u8) -> bool {
        self.edge + self.heart + self.iron + self.shadow + self.wits == expected
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
