// [Revision: v0.1] [Path: libs/lib_location/src/lib.rs] [Date: 2025-12-13]
// Description: Location system - regions, settlements, POIs, discoveries

pub mod location_type;
pub mod region;
pub mod discovery;
pub mod map_display;

pub use location_type::{Location, LocationType, LocationRank};
pub use region::{Region, RegionType};
pub use discovery::{DiscoveryTracker, DiscoveryState};
pub use map_display::{MapConfig, render_region_map, render_location_list, render_mini_map};
