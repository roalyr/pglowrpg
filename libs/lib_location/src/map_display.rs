// [Revision: v0.1] [Path: libs/lib_location/src/map_display.rs] [Date: 2025-12-13]
// Description: ASCII map display for regions and locations

use crate::location_type::{Location, LocationType};
use crate::region::{Region, RegionType};
use lib_unit_systems::coords::Coord;

/// Configuration for map display.
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub scale: u32,  // World units per map cell
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            width: 40,
            height: 15,
            scale: 10,
        }
    }
}

/// Generate an ASCII map of a region.
pub fn render_region_map(
    region: &Region,
    player_position: Option<&Coord>,
    config: &MapConfig,
) -> Vec<String> {
    let mut lines = Vec::new();
    
    // Header
    lines.push(format!("┌{}┐", "─".repeat(config.width)));
    lines.push(format!("│{:^width$}│", region.name, width = config.width));
    lines.push(format!("├{}┤", "─".repeat(config.width)));
    
    // Calculate bounds from locations
    let (min_x, _max_x, min_y, _max_y) = calculate_bounds(&region.locations, config);
    
    // Generate map grid
    for row in 0..config.height {
        let mut line = String::from("│");
        for col in 0..config.width {
            let world_x = min_x + (col as u32 * config.scale);
            let world_y = min_y + (row as u32 * config.scale);
            
            // Check for player position
            if let Some(pos) = player_position {
                if is_in_cell(pos, world_x, world_y, config.scale) {
                    line.push('@');
                    continue;
                }
            }
            
            // Check for locations
            let mut found_location = false;
            for location in &region.locations {
                if is_in_cell(&location.position, world_x, world_y, config.scale) {
                    if location.discovered {
                        line.push(location_char(&location.location_type));
                    } else {
                        line.push('?');
                    }
                    found_location = true;
                    break;
                }
            }
            
            if !found_location {
                // Background terrain
                line.push(terrain_char(&region.region_type));
            }
        }
        line.push('│');
        lines.push(line);
    }
    
    // Footer
    lines.push(format!("└{}┘", "─".repeat(config.width)));
    
    // Legend
    lines.push(String::new());
    lines.push(format!("  @ You   {} Terrain", terrain_char(&region.region_type)));
    lines.push("  ■ Settlement  ▲ Ruins  ◆ Landmark  ✶ Sacred  ⚠ Danger  ● Resource".to_string());
    
    lines
}

/// Get character for location type.
fn location_char(loc_type: &LocationType) -> char {
    match loc_type {
        LocationType::Settlement => '■',
        LocationType::Ruin => '▲',
        LocationType::Landmark => '◆',
        LocationType::Sacred => '✶',
        LocationType::Danger => '⚠',
        LocationType::Resource => '●',
    }
}

/// Get character for terrain type.
fn terrain_char(region_type: &RegionType) -> char {
    match region_type {
        RegionType::Grassland => '.',
        RegionType::Forest => '♣',
        RegionType::Mountain => '^',
        RegionType::Desert => '~',
        RegionType::Tundra => '*',
        RegionType::Swamp => '%',
        RegionType::Coast => '≈',
        RegionType::Island => '○',
    }
}

/// Check if a position is within a map cell.
fn is_in_cell(pos: &Coord, cell_x: u32, cell_y: u32, scale: u32) -> bool {
    pos.x >= cell_x && pos.x < cell_x + scale &&
    pos.y >= cell_y && pos.y < cell_y + scale
}

/// Calculate map bounds from locations.
fn calculate_bounds(locations: &[Location], config: &MapConfig) -> (u32, u32, u32, u32) {
    if locations.is_empty() {
        return (0, config.width as u32 * config.scale, 0, config.height as u32 * config.scale);
    }
    
    let min_x = locations.iter().map(|l| l.position.x).min().unwrap_or(0);
    let max_x = locations.iter().map(|l| l.position.x).max().unwrap_or(256);
    let min_y = locations.iter().map(|l| l.position.y).min().unwrap_or(0);
    let max_y = locations.iter().map(|l| l.position.y).max().unwrap_or(256);
    
    // Add padding
    let padding = config.scale * 2;
    (
        min_x.saturating_sub(padding),
        max_x + padding,
        min_y.saturating_sub(padding),
        max_y + padding,
    )
}

/// Generate a simple location list display.
pub fn render_location_list(region: &Region, current_location_idx: Option<usize>) -> Vec<String> {
    let mut lines = Vec::new();
    
    lines.push(format!("Locations in {}:", region.name));
    lines.push(String::new());
    
    for (i, location) in region.locations.iter().enumerate() {
        let marker = if Some(i) == current_location_idx { ">" } else { " " };
        let status = if location.explored { "✓" } else if location.discovered { "○" } else { "?" };
        
        if location.discovered {
            lines.push(format!("{} {} {} ({:?})", 
                marker, status, location.name, location.location_type));
        } else {
            lines.push(format!("{} {} Unknown location", marker, status));
        }
    }
    
    lines
}

/// Render a mini-map for the turn header (compact version).
pub fn render_mini_map(region: &Region, player_position: Option<&Coord>) -> String {
    let config = MapConfig {
        width: 9,
        height: 5,
        scale: 20,
    };
    
    let mut result = String::new();
    let (min_x, _, min_y, _) = calculate_bounds(&region.locations, &config);
    
    for row in 0..config.height {
        for col in 0..config.width {
            let world_x = min_x + (col as u32 * config.scale);
            let world_y = min_y + (row as u32 * config.scale);
            
            if let Some(pos) = player_position {
                if is_in_cell(pos, world_x, world_y, config.scale) {
                    result.push('@');
                    continue;
                }
            }
            
            let mut found = false;
            for location in &region.locations {
                if location.discovered && is_in_cell(&location.position, world_x, world_y, config.scale) {
                    result.push(location_char(&location.location_type));
                    found = true;
                    break;
                }
            }
            
            if !found {
                result.push(terrain_char(&region.region_type));
            }
        }
        if row < config.height - 1 {
            result.push('\n');
        }
    }
    
    result
}
