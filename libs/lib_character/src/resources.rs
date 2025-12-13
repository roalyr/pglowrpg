// [Revision: v0.1] [Path: libs/lib_character/src/resources.rs] [Date: 2025-12-13]
// Description: Character resources (Health, Spirit, Supply, Momentum)

use serde::{Deserialize, Serialize};

/// Health represents physical condition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    current: i8,
    max: i8,
}

impl Health {
    pub fn new(max: i8) -> Self {
        Self {
            current: max,
            max: max.max(0),
        }
    }

    pub fn current(&self) -> i8 {
        self.current
    }

    pub fn max(&self) -> i8 {
        self.max
    }

    pub fn modify(&mut self, delta: i8) {
        self.current = (self.current + delta).clamp(0, self.max);
    }

    pub fn set(&mut self, value: i8) {
        self.current = value.clamp(0, self.max);
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(5)
    }
}

/// Spirit represents mental/emotional resilience.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spirit {
    current: i8,
    max: i8,
}

impl Spirit {
    pub fn new(max: i8) -> Self {
        Self {
            current: max,
            max: max.max(0),
        }
    }

    pub fn current(&self) -> i8 {
        self.current
    }

    pub fn max(&self) -> i8 {
        self.max
    }

    pub fn modify(&mut self, delta: i8) {
        self.current = (self.current + delta).clamp(0, self.max);
    }

    pub fn set(&mut self, value: i8) {
        self.current = value.clamp(0, self.max);
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0
    }
}

impl Default for Spirit {
    fn default() -> Self {
        Self::new(5)
    }
}

/// Supply represents equipment, food, and resources.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Supply {
    current: i8,
    max: i8,
}

impl Supply {
    pub fn new(max: i8) -> Self {
        Self {
            current: max,
            max: max.max(0),
        }
    }

    pub fn current(&self) -> i8 {
        self.current
    }

    pub fn max(&self) -> i8 {
        self.max
    }

    pub fn modify(&mut self, delta: i8) {
        self.current = (self.current + delta).clamp(0, self.max);
    }

    pub fn set(&mut self, value: i8) {
        self.current = value.clamp(0, self.max);
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0
    }
}

impl Default for Supply {
    fn default() -> Self {
        Self::new(5)
    }
}

/// Momentum represents narrative advantage/disadvantage.
/// Range: -6 to +10 (standard Ironsworn).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Momentum {
    current: i8,
    reset: i8,
    max: i8,
}

impl Momentum {
    /// Create new momentum tracker.
    /// - reset: Value momentum resets to (default 2)
    /// - max: Maximum momentum value (default 10)
    pub fn new(reset: i8, max: i8) -> Self {
        Self {
            current: reset,
            reset,
            max,
        }
    }

    pub fn current(&self) -> i8 {
        self.current
    }

    pub fn reset_value(&self) -> i8 {
        self.reset
    }

    pub fn max(&self) -> i8 {
        self.max
    }

    /// Modify momentum, clamped to -6..=max.
    pub fn modify(&mut self, delta: i8) {
        self.current = (self.current + delta).clamp(-6, self.max);
    }

    /// Set momentum to specific value, clamped to -6..=max.
    pub fn set(&mut self, value: i8) {
        self.current = value.clamp(-6, self.max);
    }

    /// Reset momentum to reset value.
    pub fn reset(&mut self) {
        self.current = self.reset;
    }

    /// Suffer momentum (used when burning momentum fails).
    pub fn suffer(&mut self, amount: i8) {
        self.modify(-amount);
    }

    /// Check if momentum is negative.
    pub fn is_negative(&self) -> bool {
        self.current < 0
    }
}

impl Default for Momentum {
    fn default() -> Self {
        Self::new(2, 10)
    }
}

/// Container for all character resources.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Resources {
    pub health: Health,
    pub spirit: Spirit,
    pub supply: Supply,
    pub momentum: Momentum,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            health: Health::default(),
            spirit: Spirit::default(),
            supply: Supply::default(),
            momentum: Momentum::default(),
        }
    }

    /// Check if character is in critical condition.
    pub fn is_critical(&self) -> bool {
        self.health.is_depleted() || self.spirit.is_depleted()
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}
