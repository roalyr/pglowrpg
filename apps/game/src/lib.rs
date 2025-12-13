// [Revision: v0.5] [Path: apps/game/src/lib.rs] [Date: 2025-12-13]
// Description: Interactive Ironsworn gameplay

pub mod data_ops;
pub mod game_loop;
pub mod game_state;
pub mod struct_ops;

use game_loop::run_game_loop;
use game_state::GameState;

/// Entry point for the game module.
pub fn start() {
    // Load world to get seed
    let lp = match data_ops::get_layerpack() {
        Some(lp) => lp,
        None => {
            println!("No world loaded. Generate a world first.");
            return;
        }
    };

    let seed = lp.wi.seed as u64;
    let state = GameState::new("Wanderer".to_string(), seed);
    
    run_game_loop(state);
}
