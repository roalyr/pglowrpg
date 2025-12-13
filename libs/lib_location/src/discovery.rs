// [Revision: v0.1] [Path: libs/lib_location/src/discovery.rs] [Date: 2025-12-13]
// Description: Discovery and exploration tracking

use serde::{Deserialize, Serialize};

/// State of a discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryState {
    /// Not yet encountered.
    Unknown,
    /// Discovered but not fully explored.
    Discovered,
    /// Fully explored.
    Explored,
}

/// Track discoveries across the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryTracker {
    /// Total locations discovered.
    pub discovered_count: usize,
    /// Total locations fully explored.
    pub explored_count: usize,
    /// Regions visited.
    pub regions_visited: Vec<String>,
}

impl DiscoveryTracker {
    pub fn new() -> Self {
        Self {
            discovered_count: 0,
            explored_count: 0,
            regions_visited: Vec::new(),
        }
    }

    /// Record a new discovery.
    pub fn discover(&mut self) {
        self.discovered_count += 1;
    }

    /// Record a location fully explored.
    pub fn explore(&mut self) {
        self.explored_count += 1;
    }

    /// Visit a region.
    pub fn visit_region(&mut self, region_name: String) {
        if !self.regions_visited.contains(&region_name) {
            self.regions_visited.push(region_name);
        }
    }

    /// Get exploration progress (0.0 - 1.0).
    pub fn exploration_progress(&self, total_locations: usize) -> f32 {
        if total_locations == 0 {
            return 0.0;
        }
        self.discovered_count as f32 / total_locations as f32
    }
}

impl Default for DiscoveryTracker {
    fn default() -> Self {
        Self::new()
    }
}
