// [Revision: v0.1] [Path: libs/lib_soul/src/soul.rs] [Date: 2025-12-13]
// Description: Core soul data structure and reincarnation mechanics

use crate::soul_mark::SoulMark;
use crate::fate_thread::FateThread;
use lib_pseudo_rng::Rng;
use serde::{Deserialize, Serialize};

/// Current state of a soul.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoulState {
    /// Soul is embodied in a living character.
    Embodied,
    /// Soul is awaiting reincarnation.
    Awaiting,
    /// Soul has been reincarnated into a new body.
    Reincarnated,
    /// Soul has been permanently destroyed.
    Dissipated,
}

/// Cause of death affecting soul marks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeathCause {
    /// Died in combat against a foe.
    Combat { enemy_name: String },
    /// Sacrificed self for another.
    Sacrifice { for_whom: String },
    /// Betrayed by someone trusted.
    Betrayal { betrayer: String },
    /// Died fulfilling a vow.
    VowFulfilled { vow_name: String },
    /// Died with vows unfulfilled.
    VowForsaken { vow_names: Vec<String> },
    /// Died from illness or old age.
    Natural,
    /// Died alone and forgotten.
    Forgotten,
    /// Died to supernatural forces.
    Supernatural { source: String },
    /// Unknown or unspecified death.
    Unknown,
}

impl DeathCause {
    /// Get display string for this death cause.
    pub fn description(&self) -> String {
        match self {
            DeathCause::Combat { enemy_name } => 
                format!("Fell in battle against {}", enemy_name),
            DeathCause::Sacrifice { for_whom } => 
                format!("Sacrificed their life for {}", for_whom),
            DeathCause::Betrayal { betrayer } => 
                format!("Betrayed by {}", betrayer),
            DeathCause::VowFulfilled { vow_name } => 
                format!("Died having fulfilled the vow: {}", vow_name),
            DeathCause::VowForsaken { vow_names } => 
                format!("Died with forsaken vows: {}", vow_names.join(", ")),
            DeathCause::Natural => 
                "Passed peacefully".to_string(),
            DeathCause::Forgotten => 
                "Died alone and forgotten".to_string(),
            DeathCause::Supernatural { source } => 
                format!("Claimed by {}", source),
            DeathCause::Unknown => 
                "Death came unexpectedly".to_string(),
        }
    }
    
    /// Check if this death cause risks soul dissipation.
    pub fn risks_dissipation(&self) -> bool {
        matches!(self, DeathCause::Forgotten | DeathCause::Supernatural { .. })
    }
}

/// A soul that persists across incarnations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soul {
    /// Unique identifier for this soul.
    pub id: u64,
    /// Current state of the soul.
    pub state: SoulState,
    /// Soul strength (accumulated through significant deeds).
    /// Higher strength = faster reincarnation, more marks.
    pub strength: u32,
    /// Number of times this soul has incarnated.
    pub incarnation_count: u32,
    /// Names of previous incarnations.
    pub past_names: Vec<String>,
    /// Soul marks from significant events.
    pub marks: Vec<SoulMark>,
    /// Fate threads connecting to other souls.
    pub fate_threads: Vec<FateThread>,
    /// Turns remaining before reincarnation (when awaiting).
    pub reincarnation_timer: u32,
}

impl Soul {
    /// Create a new soul.
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: SoulState::Embodied,
            strength: 10, // Starting strength
            incarnation_count: 1,
            past_names: Vec::new(),
            marks: Vec::new(),
            fate_threads: Vec::new(),
            reincarnation_timer: 0,
        }
    }
    
    /// Generate a random soul ID.
    pub fn generate_id(rng: &mut Rng) -> u64 {
        let high = rng.rand_range(0, u32::MAX) as u64;
        let low = rng.rand_range(0, u32::MAX) as u64;
        (high << 32) | low
    }
    
    /// Add strength from a significant deed.
    pub fn gain_strength(&mut self, amount: u32) {
        self.strength = self.strength.saturating_add(amount);
    }
    
    /// Add a soul mark.
    pub fn add_mark(&mut self, mark: SoulMark) {
        // Limit marks based on strength
        let max_marks = (self.strength / 20).max(1) as usize;
        if self.marks.len() < max_marks {
            self.marks.push(mark);
        } else if let Some(weakest) = self.marks.iter()
            .enumerate()
            .min_by_key(|(_, m)| m.strength as u8)
            .map(|(i, _)| i) 
        {
            // Replace weakest mark if new one is stronger
            if self.marks[weakest].strength < mark.strength {
                self.marks[weakest] = mark;
            }
        }
    }
    
    /// Add a fate thread to another soul.
    pub fn add_fate_thread(&mut self, thread: FateThread) {
        // Check if we already have a thread to this soul
        if let Some(existing) = self.fate_threads.iter_mut()
            .find(|t| t.other_soul_id == thread.other_soul_id) 
        {
            existing.strengthen();
        } else {
            self.fate_threads.push(thread);
        }
    }
    
    /// Handle character death.
    pub fn on_death(&mut self, character_name: String, cause: &DeathCause, rng: &mut Rng) {
        // Record the past name
        self.past_names.push(character_name);
        
        // Check for dissipation
        if cause.risks_dissipation() && self.strength < 20 {
            // Weak souls risk dissipation
            let dissipation_chance = match cause {
                DeathCause::Forgotten => 50,
                DeathCause::Supernatural { .. } => 30,
                _ => 0,
            };
            
            if rng.rand_range(0, 100) < dissipation_chance {
                self.state = SoulState::Dissipated;
                return;
            }
        }
        
        // Soul enters waiting state
        self.state = SoulState::Awaiting;
        
        // Calculate reincarnation timer based on strength
        // Stronger souls reincarnate faster
        self.reincarnation_timer = (1000 / self.strength.max(1)).max(10);
        
        // Gain strength from meaningful deaths
        match cause {
            DeathCause::Sacrifice { .. } => self.gain_strength(15),
            DeathCause::VowFulfilled { .. } => self.gain_strength(10),
            DeathCause::Combat { .. } => self.gain_strength(5),
            DeathCause::Betrayal { .. } => self.gain_strength(8),
            DeathCause::VowForsaken { .. } => {
                // Forsaking vows weakens the soul
                self.strength = self.strength.saturating_sub(5);
            }
            DeathCause::Forgotten => {
                self.strength = self.strength.saturating_sub(10);
            }
            _ => {}
        }
    }
    
    /// Tick the reincarnation timer.
    /// Returns true if soul is ready to reincarnate.
    pub fn tick(&mut self) -> bool {
        if self.state == SoulState::Awaiting {
            self.reincarnation_timer = self.reincarnation_timer.saturating_sub(1);
            if self.reincarnation_timer == 0 {
                return true;
            }
        }
        false
    }
    
    /// Reincarnate the soul into a new body.
    pub fn reincarnate(&mut self) {
        self.state = SoulState::Reincarnated;
        self.incarnation_count += 1;
        
        // Weaken some marks over time
        for mark in &mut self.marks {
            mark.fade();
        }
        
        // Remove faded marks
        self.marks.retain(|m| !m.is_faded());
        
        // After reincarnation, soul becomes embodied again
        self.state = SoulState::Embodied;
    }
    
    /// Check if soul is dissipated (true permadeath).
    pub fn is_dissipated(&self) -> bool {
        self.state == SoulState::Dissipated
    }
    
    /// Check if soul is ready to reincarnate.
    pub fn can_reincarnate(&self) -> bool {
        self.state == SoulState::Awaiting && self.reincarnation_timer == 0
    }
    
    /// Get number of active marks.
    pub fn mark_count(&self) -> usize {
        self.marks.len()
    }
    
    /// Get marks that match a specific type.
    pub fn marks_of_type(&self, mark_type: crate::soul_mark::MarkType) -> Vec<&SoulMark> {
        self.marks.iter().filter(|m| m.mark_type == mark_type).collect()
    }
    
    /// Get soul summary string.
    pub fn summary(&self) -> String {
        format!(
            "Soul #{} | Strength: {} | Incarnations: {} | Marks: {} | Threads: {}",
            self.id % 10000, // Show last 4 digits
            self.strength,
            self.incarnation_count,
            self.marks.len(),
            self.fate_threads.len()
        )
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::soul_mark::{MarkStrength, MarkType, SoulMark};

    // -------------------------------------------------------------------------
    // SoulState tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_state_equality() {
        assert_eq!(SoulState::Embodied, SoulState::Embodied);
        assert_ne!(SoulState::Embodied, SoulState::Awaiting);
    }

    // -------------------------------------------------------------------------
    // DeathCause tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_death_cause_combat_description() {
        let cause = DeathCause::Combat { enemy_name: "Dragon".to_string() };
        assert!(cause.description().contains("Dragon"));
        assert!(!cause.risks_dissipation());
    }

    #[test]
    fn test_death_cause_sacrifice_description() {
        let cause = DeathCause::Sacrifice { for_whom: "the village".to_string() };
        assert!(cause.description().contains("village"));
    }

    #[test]
    fn test_death_cause_forgotten_risks_dissipation() {
        let cause = DeathCause::Forgotten;
        assert!(cause.risks_dissipation());
    }

    #[test]
    fn test_death_cause_supernatural_risks_dissipation() {
        let cause = DeathCause::Supernatural { source: "void".to_string() };
        assert!(cause.risks_dissipation());
    }

    #[test]
    fn test_death_cause_vow_forsaken() {
        let cause = DeathCause::VowForsaken { 
            vow_names: vec!["protect".to_string(), "avenge".to_string()] 
        };
        assert!(cause.description().contains("protect"));
        assert!(cause.description().contains("avenge"));
    }

    // -------------------------------------------------------------------------
    // Soul creation tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_new() {
        let soul = Soul::new(12345);
        assert_eq!(soul.id, 12345);
        assert_eq!(soul.state, SoulState::Embodied);
        assert_eq!(soul.strength, 10);
        assert_eq!(soul.incarnation_count, 1);
        assert!(soul.past_names.is_empty());
        assert!(soul.marks.is_empty());
        assert!(soul.fate_threads.is_empty());
    }

    #[test]
    fn test_soul_generate_id() {
        let mut rng = Rng::new(42);
        let id1 = Soul::generate_id(&mut rng);
        let id2 = Soul::generate_id(&mut rng);
        assert_ne!(id1, id2);
    }

    // -------------------------------------------------------------------------
    // Soul strength tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_gain_strength() {
        let mut soul = Soul::new(1);
        assert_eq!(soul.strength, 10);
        soul.gain_strength(5);
        assert_eq!(soul.strength, 15);
    }

    #[test]
    fn test_soul_gain_strength_saturating() {
        let mut soul = Soul::new(1);
        soul.strength = u32::MAX - 5;
        soul.gain_strength(100);
        assert_eq!(soul.strength, u32::MAX);
    }

    // -------------------------------------------------------------------------
    // Soul mark management tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_add_mark() {
        let mut soul = Soul::new(1);
        soul.strength = 30; // Allows 1 mark (30/20 = 1)

        let mark = SoulMark::new(MarkType::Fear, MarkStrength::Strong, "darkness");
        soul.add_mark(mark);

        assert_eq!(soul.marks.len(), 1);
        assert_eq!(soul.marks[0].subject, "darkness");
    }

    #[test]
    fn test_soul_add_mark_respects_limit() {
        let mut soul = Soul::new(1);
        soul.strength = 20; // Allows 1 mark

        let mark1 = SoulMark::new(MarkType::Fear, MarkStrength::Strong, "fire");
        let mark2 = SoulMark::new(MarkType::Affinity, MarkStrength::Strong, "water");

        soul.add_mark(mark1);
        soul.add_mark(mark2);

        // Should only have 1 mark (whichever stayed)
        assert_eq!(soul.marks.len(), 1);
    }

    #[test]
    fn test_soul_mark_count() {
        let mut soul = Soul::new(1);
        soul.strength = 100; // Allows 5 marks

        for i in 0..3 {
            soul.add_mark(SoulMark::new(
                MarkType::Echo,
                MarkStrength::Clear,
                format!("skill_{}", i),
            ));
        }

        assert_eq!(soul.mark_count(), 3);
    }

    // -------------------------------------------------------------------------
    // Death and reincarnation tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_on_death_combat() {
        let mut soul = Soul::new(1);
        let initial_strength = soul.strength;
        let mut rng = Rng::new(42);

        let cause = DeathCause::Combat { enemy_name: "Wolf".to_string() };
        soul.on_death("Hero".to_string(), &cause, &mut rng);

        assert_eq!(soul.state, SoulState::Awaiting);
        assert!(soul.past_names.contains(&"Hero".to_string()));
        assert!(soul.strength > initial_strength); // Combat adds 5 strength
    }

    #[test]
    fn test_soul_on_death_sacrifice() {
        let mut soul = Soul::new(1);
        soul.strength = 30;
        let mut rng = Rng::new(42);

        let cause = DeathCause::Sacrifice { for_whom: "the child".to_string() };
        soul.on_death("Martyr".to_string(), &cause, &mut rng);

        // Sacrifice adds 15 strength
        assert_eq!(soul.strength, 45);
    }

    #[test]
    fn test_soul_on_death_vow_forsaken_weakens() {
        let mut soul = Soul::new(1);
        soul.strength = 30;
        let mut rng = Rng::new(42);

        let cause = DeathCause::VowForsaken { vow_names: vec!["protect".to_string()] };
        soul.on_death("Oathbreaker".to_string(), &cause, &mut rng);

        // Forsaking vows weakens by 5
        assert_eq!(soul.strength, 25);
    }

    #[test]
    fn test_soul_reincarnation_timer() {
        let mut soul = Soul::new(1);
        soul.strength = 100; // Faster reincarnation
        let mut rng = Rng::new(42);

        soul.on_death("Old One".to_string(), &DeathCause::Natural, &mut rng);

        // Timer should be set: 1000 / 100 = 10
        assert_eq!(soul.reincarnation_timer, 10);
    }

    #[test]
    fn test_soul_tick() {
        let mut soul = Soul::new(1);
        soul.state = SoulState::Awaiting;
        soul.reincarnation_timer = 3;

        assert!(!soul.tick()); // Timer: 3 -> 2
        assert_eq!(soul.reincarnation_timer, 2);
        assert!(!soul.tick()); // Timer: 2 -> 1
        assert_eq!(soul.reincarnation_timer, 1);
        assert!(soul.tick()); // Timer: 1 -> 0, returns true (ready to reincarnate)
        assert_eq!(soul.reincarnation_timer, 0);
    }

    #[test]
    fn test_soul_tick_only_when_awaiting() {
        let mut soul = Soul::new(1);
        soul.state = SoulState::Embodied;
        soul.reincarnation_timer = 5;

        assert!(!soul.tick());
        assert_eq!(soul.reincarnation_timer, 5); // Unchanged
    }

    #[test]
    fn test_soul_can_reincarnate() {
        let mut soul = Soul::new(1);
        assert!(!soul.can_reincarnate());

        soul.state = SoulState::Awaiting;
        soul.reincarnation_timer = 1;
        assert!(!soul.can_reincarnate());

        soul.reincarnation_timer = 0;
        assert!(soul.can_reincarnate());
    }

    #[test]
    fn test_soul_reincarnate() {
        let mut soul = Soul::new(1);
        soul.state = SoulState::Awaiting;

        // Add a mark
        soul.strength = 50;
        soul.add_mark(SoulMark::new(MarkType::Fear, MarkStrength::Strong, "test"));

        soul.reincarnate();

        assert_eq!(soul.state, SoulState::Embodied);
        assert_eq!(soul.incarnation_count, 2);
    }

    #[test]
    fn test_soul_is_dissipated() {
        let mut soul = Soul::new(1);
        assert!(!soul.is_dissipated());

        soul.state = SoulState::Dissipated;
        assert!(soul.is_dissipated());
    }

    // -------------------------------------------------------------------------
    // Soul summary tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_summary() {
        let soul = Soul::new(12345);
        let summary = soul.summary();
        assert!(summary.contains("Strength: 10"));
        assert!(summary.contains("Incarnations: 1"));
    }
}
