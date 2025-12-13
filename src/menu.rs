// [Revision: v0.1] [Path: src/menu.rs] [Date: 2025-12-13]
// Description: Main menu - simplified for barebones version

use lib_game_options::OPTIONS;  // Required by prompt_input! macro
use lib_text_ops::prompt_input;
use lib_text_ops::INTERFACE_STRINGS as UI;
use lib_text_ops::MENU_STRINGS as MS;

pub fn start() {
    UI.print_newline();
    UI.print_separator_thick("");
    UI.print_newline();
    UI.print_banner_block(MS.game_title());
    UI.print_banner_empty(MS.game_name());
    UI.print_banner_empty(MS.developer());
    UI.print_newline();
    UI.print_separator_thick("");

    loop {
        let input = prompt_input!(
            "opt";
        {
            UI.print_newline();
            UI.print_banner_dash(MS.menu_title());
            UI.print_newline();
            // Simplified menu for barebones version
            println!("  [1] Generate World");
            println!("  [2] Load World (stub)");
            println!("  [3] Quit");
            UI.print_newline();
        });

        match input.as_str() {
            "1" => worldgen::start(),
            "2" => game::start(),
            "3" => return,
            _ => continue,
        }
    }
}
