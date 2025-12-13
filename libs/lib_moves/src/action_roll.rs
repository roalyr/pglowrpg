// [Revision: v0.1] [Path: libs/lib_moves/src/action_roll.rs] [Date: 2025-12-13]
// Description: 2d10 + stat vs 2d10 action roll mechanics

use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;
use serde::{Deserialize, Serialize};

/// Result of an action roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionRollResult {
    /// Strong hit: action die beats both challenge dice.
    StrongHit,
    /// Weak hit: action die beats one challenge die.
    WeakHit,
    /// Miss: action die beats neither challenge die.
    Miss,
    /// Match: both challenge dice show the same value (special outcome).
    Match,
}

/// Detailed action roll with all dice values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ActionRoll {
    /// The action die (1d10).
    pub action_die: u8,
    /// The stat modifier added to action die.
    pub stat_value: u8,
    /// First challenge die (1d10).
    pub challenge_die_1: u8,
    /// Second challenge die (1d10).
    pub challenge_die_2: u8,
    /// Additional modifier (e.g., from assets or conditions).
    pub modifier: i8,
    /// Total action value.
    pub action_total: i8,
    /// The result of the roll.
    pub result: ActionRollResult,
}

impl ActionRoll {
    /// Perform an action roll: 1d10 + stat + modifier vs 2d10.
    pub fn roll(rng: &mut Rng, stat_value: u8, modifier: i8) -> Self {
        let action_die = rng.d10();
        let challenge_die_1 = rng.d10();
        let challenge_die_2 = rng.d10();

        let action_total = action_die as i8 + stat_value as i8 + modifier;

        // Check for match (special outcome).
        let is_match = challenge_die_1 == challenge_die_2;

        // Determine result.
        let beats_first = action_total > challenge_die_1 as i8;
        let beats_second = action_total > challenge_die_2 as i8;

        let result = if is_match {
            ActionRollResult::Match
        } else if beats_first && beats_second {
            ActionRollResult::StrongHit
        } else if beats_first || beats_second {
            ActionRollResult::WeakHit
        } else {
            ActionRollResult::Miss
        };

        Self {
            action_die,
            stat_value,
            challenge_die_1,
            challenge_die_2,
            modifier,
            action_total,
            result,
        }
    }

    /// Roll with a specific stat from a character.
    pub fn roll_with_stat(
        rng: &mut Rng,
        stat: Stat,
        stats: &lib_character::stats::Stats,
        modifier: i8,
    ) -> Self {
        let stat_value = stats.get(stat);
        Self::roll(rng, stat_value, modifier)
    }

    /// Check if this is a strong hit.
    pub fn is_strong_hit(&self) -> bool {
        self.result == ActionRollResult::StrongHit
    }

    /// Check if this is a weak hit.
    pub fn is_weak_hit(&self) -> bool {
        self.result == ActionRollResult::WeakHit
    }

    /// Check if this is a miss.
    pub fn is_miss(&self) -> bool {
        self.result == ActionRollResult::Miss
    }

    /// Check if this is a match.
    pub fn is_match(&self) -> bool {
        self.result == ActionRollResult::Match
    }

    /// Check if the roll is a hit (strong or weak).
    pub fn is_hit(&self) -> bool {
        matches!(
            self.result,
            ActionRollResult::StrongHit | ActionRollResult::WeakHit
        )
    }
}

/// Progress roll: progress score vs 2d10.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProgressRoll {
    /// Progress score (0-10).
    pub progress_score: u8,
    /// First challenge die.
    pub challenge_die_1: u8,
    /// Second challenge die.
    pub challenge_die_2: u8,
    /// The result of the roll.
    pub result: ActionRollResult,
}

impl ProgressRoll {
    /// Perform a progress roll: progress score vs 2d10.
    pub fn roll(rng: &mut Rng, progress_score: u8) -> Self {
        let challenge_die_1 = rng.d10();
        let challenge_die_2 = rng.d10();

        let is_match = challenge_die_1 == challenge_die_2;

        let beats_first = progress_score > challenge_die_1;
        let beats_second = progress_score > challenge_die_2;

        let result = if is_match {
            ActionRollResult::Match
        } else if beats_first && beats_second {
            ActionRollResult::StrongHit
        } else if beats_first || beats_second {
            ActionRollResult::WeakHit
        } else {
            ActionRollResult::Miss
        };

        Self {
            progress_score,
            challenge_die_1,
            challenge_die_2,
            result,
        }
    }

    /// Check if this is a strong hit.
    pub fn is_strong_hit(&self) -> bool {
        self.result == ActionRollResult::StrongHit
    }

    /// Check if this is a weak hit.
    pub fn is_weak_hit(&self) -> bool {
        self.result == ActionRollResult::WeakHit
    }

    /// Check if this is a miss.
    pub fn is_miss(&self) -> bool {
        self.result == ActionRollResult::Miss
    }

    /// Check if this is a match.
    pub fn is_match(&self) -> bool {
        self.result == ActionRollResult::Match
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ActionRollResult tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_action_roll_result_equality() {
        assert_eq!(ActionRollResult::StrongHit, ActionRollResult::StrongHit);
        assert_ne!(ActionRollResult::StrongHit, ActionRollResult::WeakHit);
        assert_ne!(ActionRollResult::Miss, ActionRollResult::Match);
    }

    // -------------------------------------------------------------------------
    // ActionRoll creation and mechanics tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_action_roll_with_known_dice() {
        // Create a deterministic roll by setting values directly
        let roll = ActionRoll {
            action_die: 6,
            stat_value: 3,
            challenge_die_1: 4,
            challenge_die_2: 5,
            modifier: 0,
            action_total: 9,
            result: ActionRollResult::StrongHit,
        };

        // Action total (9) beats both challenge dice (4 and 5)
        assert!(roll.is_strong_hit());
        assert!(roll.is_hit());
        assert!(!roll.is_miss());
        assert!(!roll.is_match());
    }

    #[test]
    fn test_action_roll_weak_hit() {
        let roll = ActionRoll {
            action_die: 3,
            stat_value: 2,
            challenge_die_1: 4,
            challenge_die_2: 8,
            modifier: 0,
            action_total: 5,
            result: ActionRollResult::WeakHit,
        };

        // Action total (5) beats first (4) but not second (8)
        assert!(roll.is_weak_hit());
        assert!(roll.is_hit());
        assert!(!roll.is_strong_hit());
        assert!(!roll.is_miss());
    }

    #[test]
    fn test_action_roll_miss() {
        let roll = ActionRoll {
            action_die: 2,
            stat_value: 1,
            challenge_die_1: 7,
            challenge_die_2: 8,
            modifier: 0,
            action_total: 3,
            result: ActionRollResult::Miss,
        };

        // Action total (3) beats neither challenge die
        assert!(roll.is_miss());
        assert!(!roll.is_hit());
        assert!(!roll.is_strong_hit());
        assert!(!roll.is_weak_hit());
    }

    #[test]
    fn test_action_roll_match_detection() {
        let roll = ActionRoll {
            action_die: 5,
            stat_value: 2,
            challenge_die_1: 6,
            challenge_die_2: 6,
            modifier: 0,
            action_total: 7,
            result: ActionRollResult::Match,
        };

        // Both challenge dice are the same = match
        assert!(roll.is_match());
    }

    #[test]
    fn test_action_roll_with_positive_modifier() {
        // Test that modifiers are correctly applied
        let mut rng = Rng::new(12345);
        let roll = ActionRoll::roll(&mut rng, 2, 3);

        // action_total should include the +3 modifier
        assert_eq!(
            roll.action_total,
            roll.action_die as i8 + roll.stat_value as i8 + roll.modifier
        );
        assert_eq!(roll.modifier, 3);
    }

    #[test]
    fn test_action_roll_with_negative_modifier() {
        let mut rng = Rng::new(54321);
        let roll = ActionRoll::roll(&mut rng, 3, -2);

        assert_eq!(roll.modifier, -2);
        assert_eq!(
            roll.action_total,
            roll.action_die as i8 + roll.stat_value as i8 - 2
        );
    }

    #[test]
    fn test_action_roll_determinism_with_same_seed() {
        let mut rng1 = Rng::new(99999);
        let mut rng2 = Rng::new(99999);

        let roll1 = ActionRoll::roll(&mut rng1, 2, 0);
        let roll2 = ActionRoll::roll(&mut rng2, 2, 0);

        assert_eq!(roll1.action_die, roll2.action_die);
        assert_eq!(roll1.challenge_die_1, roll2.challenge_die_1);
        assert_eq!(roll1.challenge_die_2, roll2.challenge_die_2);
    }

    #[test]
    fn test_action_roll_stat_value_affects_total() {
        let mut rng = Rng::new(11111);
        let roll_low = ActionRoll::roll(&mut rng, 1, 0);

        let mut rng = Rng::new(11111);
        let roll_high = ActionRoll::roll(&mut rng, 3, 0);

        // Same dice rolls, different stat values
        assert_eq!(roll_low.action_die, roll_high.action_die);
        assert_eq!(roll_high.action_total, roll_low.action_total + 2);
    }

    // -------------------------------------------------------------------------
    // ProgressRoll tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_progress_roll_strong_hit() {
        let roll = ProgressRoll {
            progress_score: 9,
            challenge_die_1: 5,
            challenge_die_2: 7,
            result: ActionRollResult::StrongHit,
        };

        // Progress (9) beats both dice (5 and 7)
        assert!(roll.is_strong_hit());
        assert!(!roll.is_weak_hit());
        assert!(!roll.is_miss());
    }

    #[test]
    fn test_progress_roll_weak_hit() {
        let roll = ProgressRoll {
            progress_score: 6,
            challenge_die_1: 4,
            challenge_die_2: 9,
            result: ActionRollResult::WeakHit,
        };

        // Progress (6) beats first (4) but not second (9)
        assert!(roll.is_weak_hit());
    }

    #[test]
    fn test_progress_roll_miss() {
        let roll = ProgressRoll {
            progress_score: 3,
            challenge_die_1: 5,
            challenge_die_2: 8,
            result: ActionRollResult::Miss,
        };

        // Progress (3) beats neither
        assert!(roll.is_miss());
    }

    #[test]
    fn test_progress_roll_match() {
        let roll = ProgressRoll {
            progress_score: 5,
            challenge_die_1: 7,
            challenge_die_2: 7,
            result: ActionRollResult::Match,
        };

        assert!(roll.is_match());
    }

    #[test]
    fn test_progress_roll_with_rng() {
        let mut rng = Rng::new(42);
        let roll = ProgressRoll::roll(&mut rng, 8);

        assert_eq!(roll.progress_score, 8);
        assert!(roll.challenge_die_1 >= 1 && roll.challenge_die_1 <= 10);
        assert!(roll.challenge_die_2 >= 1 && roll.challenge_die_2 <= 10);
    }

    #[test]
    fn test_progress_roll_max_progress_likely_hits() {
        // With max progress of 10, we should mostly get strong hits
        let mut hits = 0;
        let mut rng = Rng::new(777);

        for _ in 0..100 {
            let roll = ProgressRoll::roll(&mut rng, 10);
            if roll.is_strong_hit() {
                hits += 1;
            }
        }

        // With progress 10, strong hits should be common (beats any die < 10)
        assert!(hits > 50, "Expected >50 strong hits with progress 10, got {}", hits);
    }

    #[test]
    fn test_progress_roll_zero_progress_likely_misses() {
        // With progress 0, we should mostly miss
        let mut misses = 0;
        let mut rng = Rng::new(888);

        for _ in 0..100 {
            let roll = ProgressRoll::roll(&mut rng, 0);
            if roll.is_miss() {
                misses += 1;
            }
        }

        // Progress 0 can't beat any die (1-10), so most should be misses or matches
        assert!(misses > 70, "Expected >70 misses with progress 0, got {}", misses);
    }
}
