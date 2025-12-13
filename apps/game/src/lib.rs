// [Revision: v0.7] [Path: apps/game/src/lib.rs] [Date: 2025-12-13]
// Description: Interactive Ironsworn gameplay

pub mod data_ops;
pub mod game_loop;
pub mod game_state;
pub mod journal;
pub mod save_game;
pub mod struct_ops;

use game_loop::run_game_loop;
use game_state::GameState;
use save_game::SaveData;

/// Entry point for the game module.
pub fn start() {
    // Check for existing save
    let saves = SaveData::list_saves();
    if !saves.is_empty() {
        println!("\n=== Existing Saves Found ===");
        for (i, save) in saves.iter().enumerate() {
            println!("  {}. {}", i + 1, save);
        }
        println!("  N. New Game");
        println!("\nLoad a save? (enter number or N): ");
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim();
            if let Ok(idx) = input.parse::<usize>() {
                if idx > 0 && idx <= saves.len() {
                    let save_name = &saves[idx - 1];
                    match SaveData::load_from_file(&SaveData::named_save_path(save_name)) {
                        Ok(save_data) => {
                            println!("Loaded save: {}", save_name);
                            let state = GameState::from_save_data(save_data);
                            run_game_loop(state);
                            return;
                        }
                        Err(e) => {
                            println!("Failed to load save: {}", e);
                        }
                    }
                }
            }
        }
    }

    // Start new game
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
