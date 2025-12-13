// [Revision: v0.1] [Path: libs/lib_combat/src/enemy.rs] [Date: 2025-12-13]
// Description: Enemy stat blocks and definitions

use serde::{Deserialize, Serialize};

/// Rank/difficulty of an enemy (matches Ironsworn progress track).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyRank {
    /// Troublesome: 3 progress per harm, 12 boxes = 4 hits
    Troublesome,
    /// Dangerous: 2 progress per harm, 12 boxes = 6 hits
    Dangerous,
    /// Formidable: 1 progress per harm, 12 boxes = 12 hits
    Formidable,
    /// Extreme: 2 ticks per harm, 12 boxes = 24 hits
    Extreme,
    /// Epic: 1 tick per harm, 12 boxes = 48 hits
    Epic,
}

impl EnemyRank {
    /// Progress marked per successful hit.
    pub fn progress_per_harm(&self) -> u8 {
        match self {
            EnemyRank::Troublesome => 12, // 3 boxes = 12 ticks
            EnemyRank::Dangerous => 8,    // 2 boxes = 8 ticks
            EnemyRank::Formidable => 4,   // 1 box = 4 ticks
            EnemyRank::Extreme => 2,      // half box = 2 ticks
            EnemyRank::Epic => 1,         // quarter box = 1 tick
        }
    }
    
    /// Typical harm dealt by this rank enemy.
    pub fn harm_dealt(&self) -> i8 {
        match self {
            EnemyRank::Troublesome => 1,
            EnemyRank::Dangerous => 2,
            EnemyRank::Formidable => 2,
            EnemyRank::Extreme => 3,
            EnemyRank::Epic => 4,
        }
    }
    
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            EnemyRank::Troublesome => "Troublesome",
            EnemyRank::Dangerous => "Dangerous",
            EnemyRank::Formidable => "Formidable",
            EnemyRank::Extreme => "Extreme",
            EnemyRank::Epic => "Epic",
        }
    }
}

/// Type of enemy creature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyType {
    /// Human or humanoid foe.
    Humanoid,
    /// Beast or animal.
    Beast,
    /// Supernatural creature.
    Horror,
    /// Undead entity.
    Undead,
    /// Giant or large creature.
    Giant,
    /// Swarm of smaller creatures.
    Swarm,
}

impl EnemyType {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            EnemyType::Humanoid => "Humanoid",
            EnemyType::Beast => "Beast",
            EnemyType::Horror => "Horror",
            EnemyType::Undead => "Undead",
            EnemyType::Giant => "Giant",
            EnemyType::Swarm => "Swarm",
        }
    }
}

/// Enemy combat behavior pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyBehavior {
    /// Aggressive: Always attacks when able.
    Aggressive,
    /// Defensive: Focuses on protecting itself.
    Defensive,
    /// Cunning: Uses tactics and environment.
    Cunning,
    /// Berserk: Reckless attacks, ignores defense.
    Berserk,
    /// Pack: Coordinates with allies.
    Pack,
    /// Ambush: Strikes from hiding.
    Ambush,
}

impl EnemyBehavior {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            EnemyBehavior::Aggressive => "Aggressive",
            EnemyBehavior::Defensive => "Defensive",
            EnemyBehavior::Cunning => "Cunning",
            EnemyBehavior::Berserk => "Berserk",
            EnemyBehavior::Pack => "Pack",
            EnemyBehavior::Ambush => "Ambush",
        }
    }
    
    /// Modifier to enemy actions.
    pub fn attack_modifier(&self) -> i8 {
        match self {
            EnemyBehavior::Aggressive => 1,
            EnemyBehavior::Defensive => -1,
            EnemyBehavior::Cunning => 0,
            EnemyBehavior::Berserk => 2,
            EnemyBehavior::Pack => 1,
            EnemyBehavior::Ambush => 2,
        }
    }
}

/// An enemy combatant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    /// Display name.
    pub name: String,
    /// Enemy rank (determines progress track).
    pub rank: EnemyRank,
    /// Type of creature.
    pub enemy_type: EnemyType,
    /// Combat behavior.
    pub behavior: EnemyBehavior,
    /// Progress track (0-40 ticks, 40 = defeated).
    pub progress: u8,
    /// Special abilities or traits.
    pub traits: Vec<String>,
    /// Whether enemy has initiative.
    pub has_initiative: bool,
    /// Whether enemy is defeated.
    pub defeated: bool,
}

impl Enemy {
    /// Create a new enemy.
    pub fn new(name: impl Into<String>, rank: EnemyRank, enemy_type: EnemyType) -> Self {
        Self {
            name: name.into(),
            rank,
            enemy_type,
            behavior: EnemyBehavior::Aggressive,
            progress: 0,
            traits: Vec::new(),
            has_initiative: false,
            defeated: false,
        }
    }
    
    /// Set behavior.
    pub fn with_behavior(mut self, behavior: EnemyBehavior) -> Self {
        self.behavior = behavior;
        self
    }
    
    /// Add a trait.
    pub fn with_trait(mut self, trait_name: impl Into<String>) -> Self {
        self.traits.push(trait_name.into());
        self
    }
    
    /// Mark progress against this enemy.
    pub fn mark_progress(&mut self, hits: u8) {
        let progress_gain = self.rank.progress_per_harm() * hits;
        self.progress = (self.progress + progress_gain).min(40);
        if self.progress >= 40 {
            self.defeated = true;
        }
    }
    
    /// Get current progress as boxes (0-10).
    pub fn progress_boxes(&self) -> u8 {
        self.progress / 4
    }
    
    /// Get remaining ticks in current box (0-3).
    pub fn progress_ticks(&self) -> u8 {
        self.progress % 4
    }
    
    /// Check if ready to end fight (progress >= threshold).
    pub fn can_end_fight(&self) -> bool {
        self.progress >= 32 // 8+ boxes filled
    }
    
    /// Harm dealt by this enemy.
    pub fn harm(&self) -> i8 {
        self.rank.harm_dealt()
    }
    
    /// Get description string.
    pub fn description(&self) -> String {
        format!("{} {} ({})", 
            self.rank.name(), 
            self.name, 
            self.enemy_type.name())
    }
    
    /// Get progress bar display.
    pub fn progress_bar(&self) -> String {
        let filled = self.progress_boxes() as usize;
        let empty = 10 - filled;
        format!("[{}{}]", "■".repeat(filled), "□".repeat(empty))
    }
}

// Predefined enemy templates

/// Create a wolf enemy.
pub fn wolf() -> Enemy {
    Enemy::new("Wolf", EnemyRank::Troublesome, EnemyType::Beast)
        .with_behavior(EnemyBehavior::Pack)
        .with_trait("Pack Hunter")
}

/// Create a bandit enemy.
pub fn bandit() -> Enemy {
    Enemy::new("Bandit", EnemyRank::Troublesome, EnemyType::Humanoid)
        .with_behavior(EnemyBehavior::Cunning)
        .with_trait("Desperate")
}

/// Create a raider enemy.
pub fn raider() -> Enemy {
    Enemy::new("Raider", EnemyRank::Dangerous, EnemyType::Humanoid)
        .with_behavior(EnemyBehavior::Aggressive)
        .with_trait("Battle-hardened")
}

/// Create a bear enemy.
pub fn bear() -> Enemy {
    Enemy::new("Bear", EnemyRank::Dangerous, EnemyType::Beast)
        .with_behavior(EnemyBehavior::Defensive)
        .with_trait("Territorial")
}

/// Create a wight enemy.
pub fn wight() -> Enemy {
    Enemy::new("Wight", EnemyRank::Formidable, EnemyType::Undead)
        .with_behavior(EnemyBehavior::Aggressive)
        .with_trait("Undying")
        .with_trait("Cold Touch")
}

/// Create a horror from the deep.
pub fn deep_horror() -> Enemy {
    Enemy::new("Deep Horror", EnemyRank::Extreme, EnemyType::Horror)
        .with_behavior(EnemyBehavior::Ambush)
        .with_trait("Tentacles")
        .with_trait("Mind-rending")
}

/// Create a giant.
pub fn giant() -> Enemy {
    Enemy::new("Giant", EnemyRank::Extreme, EnemyType::Giant)
        .with_behavior(EnemyBehavior::Berserk)
        .with_trait("Massive")
        .with_trait("Crushing Blows")
}

/// Create a dragon.
pub fn dragon() -> Enemy {
    Enemy::new("Dragon", EnemyRank::Epic, EnemyType::Horror)
        .with_behavior(EnemyBehavior::Cunning)
        .with_trait("Flight")
        .with_trait("Fire Breath")
        .with_trait("Armored Scales")
}
