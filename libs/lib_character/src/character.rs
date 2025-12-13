// [Revision: v0.1] [Path: libs/lib_character/src/character.rs] [Date: 2025-12-13]
// Description: Main character structure combining stats, resources, vows, and bonds

use lib_economy::Inventory;
use serde::{Deserialize, Serialize};

use crate::resources::Resources;
use crate::stats::Stats;
use crate::vows::{Bond, Vow};

/// A complete character with stats, resources, vows, and bonds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub resources: Resources,
    pub vows: Vec<Vow>,
    pub bonds: Vec<Bond>,
    pub inventory: Inventory,
}

impl Character {
    /// Create a new character with default values.
    pub fn new(name: String) -> Self {
        Self {
            name,
            stats: Stats::default(),
            resources: Resources::default(),
            vows: Vec::new(),
            bonds: Vec::new(),
            inventory: Inventory::default(),
        }
    }

    /// Create a character with custom stats.
    pub fn with_stats(name: String, stats: Stats) -> Self {
        Self {
            name,
            stats,
            resources: Resources::default(),
            vows: Vec::new(),
            bonds: Vec::new(),
            inventory: Inventory::default(),
        }
    }

    /// Add a vow to the character.
    pub fn add_vow(&mut self, vow: Vow) {
        self.vows.push(vow);
    }

    /// Get a mutable reference to a vow by index.
    pub fn vow_mut(&mut self, index: usize) -> Option<&mut Vow> {
        self.vows.get_mut(index)
    }

    /// Remove a vow by index.
    pub fn remove_vow(&mut self, index: usize) -> Option<Vow> {
        if index < self.vows.len() {
            Some(self.vows.remove(index))
        } else {
            None
        }
    }

    /// Get all active (incomplete) vows.
    pub fn active_vows(&self) -> Vec<&Vow> {
        self.vows.iter().filter(|v| !v.completed).collect()
    }

    /// Add a bond to the character.
    pub fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond);
    }

    /// Get a mutable reference to a bond by index.
    pub fn bond_mut(&mut self, index: usize) -> Option<&mut Bond> {
        self.bonds.get_mut(index)
    }

    /// Remove a bond by index.
    pub fn remove_bond(&mut self, index: usize) -> Option<Bond> {
        if index < self.bonds.len() {
            Some(self.bonds.remove(index))
        } else {
            None
        }
    }

    /// Check if character is in critical condition.
    pub fn is_critical(&self) -> bool {
        self.resources.is_critical()
    }

    /// Get total number of bonds.
    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    /// Get total number of active vows.
    pub fn active_vow_count(&self) -> usize {
        self.active_vows().len()
    }
}
