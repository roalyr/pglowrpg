// [Revision: v0.1] [Path: libs/lib_combat/src/encounter.rs] [Date: 2025-12-13]
// Description: Combat encounter generation and management

use crate::enemy::{Enemy, EnemyRank, EnemyType, EnemyBehavior};
use lib_pseudo_rng::Rng;
use serde::{Deserialize, Serialize};

/// Type of combat encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncounterType {
    /// Random wilderness encounter.
    Wilderness,
    /// Ambush by hostile forces.
    Ambush,
    /// Defending a position.
    Defense,
    /// Hunting a target.
    Hunt,
    /// Boss or significant enemy.
    Boss,
    /// Duel or single combat.
    Duel,
}

impl EncounterType {
    /// Whether player starts with initiative.
    pub fn player_initiative(&self) -> bool {
        match self {
            EncounterType::Wilderness => true,
            EncounterType::Ambush => false,
            EncounterType::Defense => true,
            EncounterType::Hunt => true,
            EncounterType::Boss => false,
            EncounterType::Duel => true,
        }
    }
}

/// A combat encounter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    /// Type of encounter.
    pub encounter_type: EncounterType,
    /// Enemies in this encounter.
    pub enemies: Vec<Enemy>,
    /// Current round number.
    pub round: u32,
    /// Whether encounter is active.
    pub active: bool,
    /// Whether player has initiative.
    pub player_initiative: bool,
    /// Encounter description/context.
    pub description: String,
}

impl Encounter {
    /// Create a new encounter.
    pub fn new(encounter_type: EncounterType) -> Self {
        Self {
            encounter_type,
            enemies: Vec::new(),
            round: 1,
            active: true,
            player_initiative: encounter_type.player_initiative(),
            description: String::new(),
        }
    }
    
    /// Add an enemy to the encounter.
    pub fn add_enemy(&mut self, mut enemy: Enemy) {
        enemy.has_initiative = !self.player_initiative;
        self.enemies.push(enemy);
    }
    
    /// Set encounter description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    
    /// Check if all enemies are defeated.
    pub fn all_defeated(&self) -> bool {
        self.enemies.iter().all(|e| e.defeated)
    }
    
    /// Get active (non-defeated) enemies.
    pub fn active_enemies(&self) -> Vec<&Enemy> {
        self.enemies.iter().filter(|e| !e.defeated).collect()
    }
    
    /// Get mutable active enemies.
    pub fn active_enemies_mut(&mut self) -> Vec<&mut Enemy> {
        self.enemies.iter_mut().filter(|e| !e.defeated).collect()
    }
    
    /// Get total harm from all active enemies.
    pub fn total_enemy_harm(&self) -> i8 {
        self.active_enemies().iter().map(|e| e.harm()).sum()
    }
    
    /// Advance to next round.
    pub fn next_round(&mut self) {
        self.round += 1;
    }
    
    /// End the encounter.
    pub fn end(&mut self) {
        self.active = false;
    }
    
    /// Get encounter summary.
    pub fn summary(&self) -> String {
        let enemy_count = self.active_enemies().len();
        let defeated_count = self.enemies.len() - enemy_count;
        format!("Round {} | Enemies: {} active, {} defeated",
            self.round, enemy_count, defeated_count)
    }
}

/// Generate a random encounter.
pub fn generate_encounter(rng: &mut Rng, difficulty: EnemyRank) -> Encounter {
    let encounter_type = match rng.rand_range(1, 101) {
        1..=50 => EncounterType::Wilderness,
        51..=70 => EncounterType::Ambush,
        71..=85 => EncounterType::Defense,
        86..=95 => EncounterType::Hunt,
        _ => EncounterType::Duel,
    };
    
    let mut encounter = Encounter::new(encounter_type);
    
    // Generate enemies based on difficulty
    match difficulty {
        EnemyRank::Troublesome => {
            // 1-3 troublesome enemies
            let count = rng.rand_range(1, 4);
            for _ in 0..count {
                encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Troublesome));
            }
        }
        EnemyRank::Dangerous => {
            // 1 dangerous or 2 troublesome
            if rng.rand_range(0, 100) < 60 {
                encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Dangerous));
            } else {
                encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Troublesome));
                encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Troublesome));
            }
        }
        EnemyRank::Formidable => {
            encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Formidable));
        }
        EnemyRank::Extreme => {
            encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Extreme));
        }
        EnemyRank::Epic => {
            encounter.add_enemy(generate_random_enemy(rng, EnemyRank::Epic));
        }
    }
    
    encounter
}

/// Generate a random enemy of given rank.
pub fn generate_random_enemy(rng: &mut Rng, rank: EnemyRank) -> Enemy {
    let enemy_type = match rng.rand_range(1, 101) {
        1..=40 => EnemyType::Humanoid,
        41..=70 => EnemyType::Beast,
        71..=85 => EnemyType::Undead,
        86..=95 => EnemyType::Horror,
        _ => EnemyType::Giant,
    };
    
    let name = generate_enemy_name(rng, enemy_type, rank);
    let behavior = generate_behavior(rng);
    
    Enemy::new(name, rank, enemy_type)
        .with_behavior(behavior)
}

fn generate_enemy_name(rng: &mut Rng, enemy_type: EnemyType, rank: EnemyRank) -> String {
    let base_names = match enemy_type {
        EnemyType::Humanoid => &["Bandit", "Raider", "Outlaw", "Brigand", "Cultist", "Warrior"][..],
        EnemyType::Beast => &["Wolf", "Bear", "Boar", "Serpent", "Spider", "Bat"][..],
        EnemyType::Horror => &["Shadow", "Wraith", "Specter", "Aberration", "Fiend"][..],
        EnemyType::Undead => &["Wight", "Ghoul", "Skeleton", "Zombie", "Revenant"][..],
        EnemyType::Giant => &["Ogre", "Troll", "Giant", "Cyclops"][..],
        EnemyType::Swarm => &["Rat Swarm", "Insect Swarm", "Bat Swarm"][..],
    };
    
    let idx = rng.rand_range(0, base_names.len() as u32) as usize;
    let base = base_names[idx];
    
    // Add prefix for higher ranks
    match rank {
        EnemyRank::Troublesome => base.to_string(),
        EnemyRank::Dangerous => format!("Veteran {}", base),
        EnemyRank::Formidable => format!("Champion {}", base),
        EnemyRank::Extreme => format!("Dire {}", base),
        EnemyRank::Epic => format!("Ancient {}", base),
    }
}

fn generate_behavior(rng: &mut Rng) -> EnemyBehavior {
    match rng.rand_range(1, 101) {
        1..=35 => EnemyBehavior::Aggressive,
        36..=55 => EnemyBehavior::Defensive,
        56..=75 => EnemyBehavior::Cunning,
        76..=85 => EnemyBehavior::Berserk,
        86..=95 => EnemyBehavior::Pack,
        _ => EnemyBehavior::Ambush,
    }
}
