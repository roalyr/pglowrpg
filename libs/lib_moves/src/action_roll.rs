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
