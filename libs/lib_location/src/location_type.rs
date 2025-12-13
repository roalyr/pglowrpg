// [Revision: v0.1] [Path: libs/lib_location/src/location_type.rs] [Date: 2025-12-13]
// Description: Location types and definitions

use lib_unit_systems::coords::Coord;
use serde::{Deserialize, Serialize};

/// Type of location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    /// Settlement (village, town, city).
    Settlement,
    /// Ancient ruins or abandoned structure.
    Ruin,
    /// Natural landmark (mountain, lake, forest).
    Landmark,
    /// Dangerous location (lair, cursed ground).
    Danger,
    /// Sacred site (temple, shrine).
    Sacred,
    /// Resource location (mine, quarry, hunting ground).
    Resource,
}

/// Difficulty/importance rank of a location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationRank {
    Troublesome,
    Dangerous,
    Formidable,
    Extreme,
    Epic,
}

impl LocationRank {
    /// Get danger level as a number (1-5).
    pub fn danger_level(&self) -> u8 {
        match self {
            LocationRank::Troublesome => 1,
            LocationRank::Dangerous => 2,
            LocationRank::Formidable => 3,
            LocationRank::Extreme => 4,
            LocationRank::Epic => 5,
        }
    }
}

/// A location in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub location_type: LocationType,
    pub rank: LocationRank,
    /// Coordinates on the world map.
    pub position: Coord,
    /// Whether the player has discovered this location.
    pub discovered: bool,
    /// Whether the player has explored/delved this location.
    pub explored: bool,
}

impl Location {
    pub fn new(
        name: String,
        description: String,
        location_type: LocationType,
        rank: LocationRank,
        position: Coord,
    ) -> Self {
        Self {
            name,
            description,
            location_type,
            rank,
            position,
            discovered: false,
            explored: false,
        }
    }

    /// Discover this location.
    pub fn discover(&mut self) {
        self.discovered = true;
    }

    /// Mark this location as fully explored.
    pub fn explore(&mut self) {
        self.explored = true;
    }

    /// Check if location is dangerous.
    pub fn is_dangerous(&self) -> bool {
        matches!(self.location_type, LocationType::Danger)
            || self.rank.danger_level() >= 3
    }

    /// Calculate distance to another location.
    pub fn distance_to(&self, other: &Location) -> f32 {
        let dx = (self.position.x - other.position.x) as f32;
        let dy = (self.position.y - other.position.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}
