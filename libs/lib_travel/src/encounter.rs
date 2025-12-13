// [Revision: v0.1] [Path: libs/lib_travel/src/encounter.rs] [Date: 2025-12-13]
// Description: Random encounters during travel

use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;
use serde::{Deserialize, Serialize};

/// Type of encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncounterType {
    /// Hostile creature or enemy.
    Combat,
    /// Obstacle or hazard.
    Obstacle,
    /// NPC or friendly encounter.
    Social,
    /// Discovery of location or item.
    Discovery,
    /// Nothing happens.
    None,
}

/// An encounter during travel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub encounter_type: EncounterType,
    pub description: String,
    /// Stat to use for resolving encounter.
    pub stat: Option<Stat>,
    /// Difficulty modifier.
    pub difficulty: i8,
}

impl Encounter {
    pub fn new(
        encounter_type: EncounterType,
        description: String,
        stat: Option<Stat>,
        difficulty: i8,
    ) -> Self {
        Self {
            encounter_type,
            description,
            stat,
            difficulty,
        }
    }

    /// Create a combat encounter.
    pub fn combat(description: String, difficulty: i8) -> Self {
        Self::new(EncounterType::Combat, description, Some(Stat::Iron), difficulty)
    }

    /// Create an obstacle encounter.
    pub fn obstacle(description: String, difficulty: i8) -> Self {
        Self::new(EncounterType::Obstacle, description, Some(Stat::Wits), difficulty)
    }

    /// Create a social encounter.
    pub fn social(description: String, difficulty: i8) -> Self {
        Self::new(EncounterType::Social, description, Some(Stat::Heart), difficulty)
    }

    /// Create a discovery encounter.
    pub fn discovery(description: String) -> Self {
        Self::new(EncounterType::Discovery, description, None, 0)
    }

    /// Create an empty encounter (no event).
    pub fn none() -> Self {
        Self::new(
            EncounterType::None,
            "The journey continues without incident.".to_string(),
            None,
            0,
        )
    }
}

/// Generate a random encounter.
pub fn generate_encounter(rng: &mut Rng, region_danger: i8) -> Encounter {
    let roll = rng.d10();
    
    match roll {
        1..=2 => {
            // Combat
            let difficulty = region_danger;
            let descriptions = [
                "A hostile creature blocks your path.",
                "Bandits ambush you on the road.",
                "A territorial beast charges at you.",
            ];
            let idx = (rng.rand_range(0, 3)) as usize;
            Encounter::combat(descriptions[idx].to_string(), difficulty)
        }
        3..=4 => {
            // Obstacle
            let difficulty = region_danger / 2;
            let descriptions = [
                "Treacherous terrain slows your progress.",
                "A raging river blocks the path.",
                "Dense fog obscures your way.",
            ];
            let idx = (rng.rand_range(0, 3)) as usize;
            Encounter::obstacle(descriptions[idx].to_string(), difficulty)
        }
        5..=6 => {
            // Social
            let difficulty = 0;
            let descriptions = [
                "You encounter a fellow traveler.",
                "A merchant caravan passes by.",
                "Locals from a nearby settlement greet you.",
            ];
            let idx = (rng.rand_range(0, 3)) as usize;
            Encounter::social(descriptions[idx].to_string(), difficulty)
        }
        7..=8 => {
            // Discovery
            let descriptions = [
                "You discover ancient ruins off the path.",
                "A hidden cave catches your attention.",
                "Strange markings on a tree reveal a secret.",
            ];
            let idx = (rng.rand_range(0, 3)) as usize;
            Encounter::discovery(descriptions[idx].to_string())
        }
        _ => {
            // Nothing
            Encounter::none()
        }
    }
}
