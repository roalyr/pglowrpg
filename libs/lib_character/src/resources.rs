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

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Health tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_health_creation() {
        let health = Health::new(5);
        assert_eq!(health.current(), 5);
        assert_eq!(health.max(), 5);
    }

    #[test]
    fn test_health_default() {
        let health = Health::default();
        assert_eq!(health.current(), 5);
        assert_eq!(health.max(), 5);
    }

    #[test]
    fn test_health_negative_max_becomes_zero() {
        let health = Health::new(-3);
        assert_eq!(health.max(), 0);
    }

    #[test]
    fn test_health_modify_positive() {
        let mut health = Health::new(5);
        health.modify(-3);
        assert_eq!(health.current(), 2);
        health.modify(1);
        assert_eq!(health.current(), 3);
    }

    #[test]
    fn test_health_modify_clamps_at_zero() {
        let mut health = Health::new(5);
        health.modify(-10);
        assert_eq!(health.current(), 0);
    }

    #[test]
    fn test_health_modify_clamps_at_max() {
        let mut health = Health::new(5);
        health.modify(-2);
        health.modify(10);
        assert_eq!(health.current(), 5);
    }

    #[test]
    fn test_health_set_clamps_values() {
        let mut health = Health::new(5);
        health.set(100);
        assert_eq!(health.current(), 5);
        health.set(-100);
        assert_eq!(health.current(), 0);
    }

    #[test]
    fn test_health_is_depleted() {
        let mut health = Health::new(5);
        assert!(!health.is_depleted());
        health.set(0);
        assert!(health.is_depleted());
    }

    // -------------------------------------------------------------------------
    // Spirit tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_spirit_creation() {
        let spirit = Spirit::new(5);
        assert_eq!(spirit.current(), 5);
        assert_eq!(spirit.max(), 5);
    }

    #[test]
    fn test_spirit_modify_clamps() {
        let mut spirit = Spirit::new(5);
        spirit.modify(-10);
        assert_eq!(spirit.current(), 0);
        spirit.modify(100);
        assert_eq!(spirit.current(), 5);
    }

    #[test]
    fn test_spirit_is_depleted() {
        let mut spirit = Spirit::new(3);
        spirit.modify(-3);
        assert!(spirit.is_depleted());
    }

    // -------------------------------------------------------------------------
    // Supply tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_supply_creation() {
        let supply = Supply::new(5);
        assert_eq!(supply.current(), 5);
        assert_eq!(supply.max(), 5);
    }

    #[test]
    fn test_supply_modify_clamps() {
        let mut supply = Supply::new(5);
        supply.modify(-6);
        assert_eq!(supply.current(), 0);
        supply.modify(10);
        assert_eq!(supply.current(), 5);
    }

    #[test]
    fn test_supply_is_depleted() {
        let mut supply = Supply::new(3);
        assert!(!supply.is_depleted());
        supply.set(0);
        assert!(supply.is_depleted());
    }

    // -------------------------------------------------------------------------
    // Momentum tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_momentum_creation() {
        let momentum = Momentum::new(2, 10);
        assert_eq!(momentum.current(), 2);
        assert_eq!(momentum.reset_value(), 2);
        assert_eq!(momentum.max(), 10);
    }

    #[test]
    fn test_momentum_default() {
        let momentum = Momentum::default();
        assert_eq!(momentum.current(), 2);
        assert_eq!(momentum.reset_value(), 2);
        assert_eq!(momentum.max(), 10);
    }

    #[test]
    fn test_momentum_modify_positive() {
        let mut momentum = Momentum::default();
        momentum.modify(3);
        assert_eq!(momentum.current(), 5);
    }

    #[test]
    fn test_momentum_modify_negative() {
        let mut momentum = Momentum::default();
        momentum.modify(-4);
        assert_eq!(momentum.current(), -2);
    }

    #[test]
    fn test_momentum_clamps_at_max() {
        let mut momentum = Momentum::new(2, 10);
        momentum.modify(20);
        assert_eq!(momentum.current(), 10);
    }

    #[test]
    fn test_momentum_clamps_at_minimum() {
        let mut momentum = Momentum::default();
        momentum.modify(-100);
        assert_eq!(momentum.current(), -6);
    }

    #[test]
    fn test_momentum_reset() {
        let mut momentum = Momentum::new(2, 10);
        momentum.modify(5);
        assert_eq!(momentum.current(), 7);
        momentum.reset();
        assert_eq!(momentum.current(), 2);
    }

    #[test]
    fn test_momentum_suffer() {
        let mut momentum = Momentum::default();
        momentum.modify(4);
        assert_eq!(momentum.current(), 6);
        momentum.suffer(3);
        assert_eq!(momentum.current(), 3);
    }

    #[test]
    fn test_momentum_is_negative() {
        let mut momentum = Momentum::default();
        assert!(!momentum.is_negative());
        momentum.set(-1);
        assert!(momentum.is_negative());
    }

    #[test]
    fn test_momentum_set_clamps() {
        let mut momentum = Momentum::new(2, 10);
        momentum.set(100);
        assert_eq!(momentum.current(), 10);
        momentum.set(-100);
        assert_eq!(momentum.current(), -6);
    }

    // -------------------------------------------------------------------------
    // Resources tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_resources_creation() {
        let resources = Resources::new();
        assert_eq!(resources.health.current(), 5);
        assert_eq!(resources.spirit.current(), 5);
        assert_eq!(resources.supply.current(), 5);
        assert_eq!(resources.momentum.current(), 2);
    }

    #[test]
    fn test_resources_is_critical() {
        let mut resources = Resources::new();
        assert!(!resources.is_critical());

        resources.health.set(0);
        assert!(resources.is_critical());

        resources.health.set(5);
        resources.spirit.set(0);
        assert!(resources.is_critical());
    }

    #[test]
    fn test_resources_default() {
        let resources = Resources::default();
        assert_eq!(resources.health.current(), 5);
        assert_eq!(resources.spirit.current(), 5);
        assert_eq!(resources.supply.current(), 5);
        assert_eq!(resources.momentum.current(), 2);
    }
}
