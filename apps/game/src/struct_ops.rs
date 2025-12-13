// [Revision: v0.1] [Path: apps/game/src/struct_ops.rs] [Date: 2025-12-13]
// Description: Game data structures - simplified for barebones version

use lib_game_data_codec::LayerPack;

/// Core game data container.
/// Currently holds only the world data (LayerPack).
/// Will be expanded with character, vows, connections per GDD.
pub struct GameData {
    pub lp: LayerPack,
    pub player_x: u32,
    pub player_y: u32,
}

impl GameData {
    /// Create new GameData from a loaded LayerPack.
    /// Player starts at the center of the map.
    pub fn new(lp: LayerPack) -> Self {
        let center = lp.wi.map_size / 2;
        GameData {
            lp,
            player_x: center,
            player_y: center,
        }
    }
}
