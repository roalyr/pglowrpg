// [Revision: v0.1] [Path: libs/lib_location/src/region.rs] [Date: 2025-12-13]
// Description: Region system for organizing locations

use serde::{Deserialize, Serialize};

use crate::location_type::Location;

/// Type of region based on biome/terrain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegionType {
    /// Grasslands, plains.
    Grassland,
    /// Dense forest.
    Forest,
    /// Mountains and highlands.
    Mountain,
    /// Desert or badlands.
    Desert,
    /// Tundra and frozen lands.
    Tundra,
    /// Swamps and wetlands.
    Swamp,
    /// Coastal areas.
    Coast,
    /// Islands.
    Island,
}

impl RegionType {
    /// Get travel difficulty modifier for this region type.
    pub fn travel_difficulty(&self) -> i8 {
        match self {
            RegionType::Grassland => 0,
            RegionType::Coast => 0,
            RegionType::Forest => -1,
            RegionType::Desert => -1,
            RegionType::Swamp => -2,
            RegionType::Mountain => -2,
            RegionType::Tundra => -2,
            RegionType::Island => 0,
        }
    }
}

/// A region in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub description: String,
    pub region_type: RegionType,
    /// Locations within this region.
    pub locations: Vec<Location>,
    /// Whether the player has visited this region.
    pub visited: bool,
}

impl Region {
    pub fn new(name: String, description: String, region_type: RegionType) -> Self {
        Self {
            name,
            description,
            region_type,
            locations: Vec::new(),
            visited: false,
        }
    }

    /// Add a location to this region.
    pub fn add_location(&mut self, location: Location) {
        self.locations.push(location);
    }

    /// Get all discovered locations in this region.
    pub fn discovered_locations(&self) -> Vec<&Location> {
        self.locations.iter().filter(|l| l.discovered).collect()
    }

    /// Get all undiscovered locations in this region.
    pub fn undiscovered_locations(&self) -> Vec<&Location> {
        self.locations.iter().filter(|l| !l.discovered).collect()
    }

    /// Mark region as visited.
    pub fn visit(&mut self) {
        self.visited = true;
    }

    /// Get travel difficulty for this region.
    pub fn travel_difficulty(&self) -> i8 {
        self.region_type.travel_difficulty()
    }
}
