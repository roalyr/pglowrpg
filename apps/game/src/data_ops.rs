// [Revision: v0.1] [Path: apps/game/src/data_ops.rs] [Date: 2025-12-13]
// Description: World data loading operations

use dep::bincode;
use lib_constants::app as ca;
use lib_game_data_codec::LayerPack;
use lib_game_options::OPTIONS;  // Required by prompt_input! macro
use lib_io_ops::decompress_to_memory;
use lib_text_ops::prompt_input;
use std::path::Path;

/// Prompts user to select a saved world and loads the LayerPack.
/// Returns None if no world is selected or loading fails.
pub fn get_layerpack() -> Option<LayerPack> {
    // List available saved worlds
    let save_dir = lib_io_ops::dir_dir_contents(ca::PATH_SAVE);
    
    if save_dir.is_empty() {
        println!("No saved worlds found in '{}'", ca::PATH_SAVE);
        return None;
    }

    // Prompt for world selection
    let input_save = prompt_input!(
        "word"; &save_dir;
        {
            println!("Available worlds:");
            for (i, name) in save_dir.iter().enumerate() {
                println!("  [{}] {}", i + 1, name);
            }
        }
    );

    if input_save.is_empty() {
        println!("No world selected.");
        return None;
    }

    // Build path and load data
    let path = Path::new(ca::PATH_SAVE)
        .to_path_buf()
        .join(&input_save)
        .join(ca::PATH_SAVE_DATA)
        .join(ca::NAME_DATA_WORLD)
        .with_extension(ca::EXTENSION_SAVE_DATA);

    let data_read = decompress_to_memory(&path);
    
    match bincode::deserialize(&data_read[..]) {
        Ok(lp) => {
            println!("Loaded world: {}", input_save);
            Some(lp)
        }
        Err(e) => {
            println!(
                "ERROR: Unable to read world data: {}\nPath: {}",
                e,
                path.display()
            );
            None
        }
    }
}
