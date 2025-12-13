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

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_enum_equality() {
        assert_eq!(Stat::Edge, Stat::Edge);
        assert_ne!(Stat::Edge, Stat::Heart);
    }

    #[test]
    fn test_stats_new_defaults_to_one() {
        let stats = Stats::new();
        assert_eq!(stats.edge, 1);
        assert_eq!(stats.heart, 1);
        assert_eq!(stats.iron, 1);
        assert_eq!(stats.shadow, 1);
        assert_eq!(stats.wits, 1);
    }

    #[test]
    fn test_stats_with_values() {
        let stats = Stats::with_values(3, 2, 2, 1, 3);
        assert_eq!(stats.edge, 3);
        assert_eq!(stats.heart, 2);
        assert_eq!(stats.iron, 2);
        assert_eq!(stats.shadow, 1);
        assert_eq!(stats.wits, 3);
    }

    #[test]
    fn test_stats_with_values_clamps_high() {
        let stats = Stats::with_values(10, 10, 10, 10, 10);
        assert_eq!(stats.edge, 3);
        assert_eq!(stats.heart, 3);
        assert_eq!(stats.iron, 3);
        assert_eq!(stats.shadow, 3);
        assert_eq!(stats.wits, 3);
    }

    #[test]
    fn test_stats_with_values_clamps_low() {
        let stats = Stats::with_values(0, 0, 0, 0, 0);
        assert_eq!(stats.edge, 1);
        assert_eq!(stats.heart, 1);
        assert_eq!(stats.iron, 1);
        assert_eq!(stats.shadow, 1);
        assert_eq!(stats.wits, 1);
    }

    #[test]
    fn test_stats_get() {
        let stats = Stats::with_values(3, 2, 1, 2, 3);
        assert_eq!(stats.get(Stat::Edge), 3);
        assert_eq!(stats.get(Stat::Heart), 2);
        assert_eq!(stats.get(Stat::Iron), 1);
        assert_eq!(stats.get(Stat::Shadow), 2);
        assert_eq!(stats.get(Stat::Wits), 3);
    }

    #[test]
    fn test_stats_set() {
        let mut stats = Stats::new();
        stats.set(Stat::Edge, 3);
        assert_eq!(stats.edge, 3);
        stats.set(Stat::Heart, 2);
        assert_eq!(stats.heart, 2);
    }

    #[test]
    fn test_stats_set_clamps() {
        let mut stats = Stats::new();
        stats.set(Stat::Edge, 10);
        assert_eq!(stats.edge, 3);
        stats.set(Stat::Edge, 0);
        assert_eq!(stats.edge, 1);
    }

    #[test]
    fn test_stats_validate_total_standard() {
        // Standard Ironsworn: 3+2+2+1+3 = 11
        let stats = Stats::with_values(3, 2, 2, 1, 3);
        assert!(stats.validate_total(11));
    }

    #[test]
    fn test_stats_validate_total_fails() {
        let stats = Stats::new(); // All 1s = 5
        assert!(!stats.validate_total(11));
        assert!(stats.validate_total(5));
    }

    #[test]
    fn test_stats_default() {
        let stats = Stats::default();
        assert_eq!(stats.edge, 1);
        assert_eq!(stats.heart, 1);
        assert_eq!(stats.iron, 1);
        assert_eq!(stats.shadow, 1);
        assert_eq!(stats.wits, 1);
    }
}
