// [Revision: v0.1] [Path: libs/lib_moves/src/move_outcome.rs] [Date: 2025-12-13]
// Description: Move outcomes and types

use serde::{Deserialize, Serialize};

/// Type of move being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveType {
    /// Standard action move.
    Action,
    /// Progress move (completing a vow, journey, etc.).
    Progress,
    /// Suffer move (taking harm).
    Suffer,
    /// Recover move (healing, resting).
    Recover,
}

/// Outcome of a move with narrative effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOutcome {
    pub move_type: MoveType,
    pub move_name: String,
    pub description: String,
    /// Momentum gain/loss.
    pub momentum_delta: i8,
    /// Health gain/loss.
    pub health_delta: i8,
    /// Spirit gain/loss.
    pub spirit_delta: i8,
    /// Supply gain/loss.
    pub supply_delta: i8,
}

impl MoveOutcome {
    /// Create a new move outcome.
    pub fn new(move_type: MoveType, move_name: String, description: String) -> Self {
        Self {
            move_type,
            move_name,
            description,
            momentum_delta: 0,
            health_delta: 0,
            spirit_delta: 0,
            supply_delta: 0,
        }
    }

    /// Set momentum change.
    pub fn with_momentum(mut self, delta: i8) -> Self {
        self.momentum_delta = delta;
        self
    }

    /// Set health change.
    pub fn with_health(mut self, delta: i8) -> Self {
        self.health_delta = delta;
        self
    }

    /// Set spirit change.
    pub fn with_spirit(mut self, delta: i8) -> Self {
        self.spirit_delta = delta;
        self
    }

    /// Set supply change.
    pub fn with_supply(mut self, delta: i8) -> Self {
        self.supply_delta = delta;
        self
    }

    /// Apply this outcome to a character's resources.
    pub fn apply_to_character(&self, character: &mut lib_character::character::Character) {
        character.resources.momentum.modify(self.momentum_delta);
        character.resources.health.modify(self.health_delta);
        character.resources.spirit.modify(self.spirit_delta);
        character.resources.supply.modify(self.supply_delta);
    }

    /// Check if this outcome has any resource changes.
    pub fn has_changes(&self) -> bool {
        self.momentum_delta != 0
            || self.health_delta != 0
            || self.spirit_delta != 0
            || self.supply_delta != 0
    }
}
