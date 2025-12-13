// [Revision: v0.2] [Path: apps/game/src/lib.rs] [Date: 2025-12-13]
// Description: Game module with basic Ironsworn mechanics demonstration

pub mod data_ops;
pub mod struct_ops;

use lib_character::character::Character;
use lib_character::stats::{Stat, Stats};
use lib_character::vows::{Vow, VowRank};
use lib_moves::action_roll::ActionRoll;
use lib_pseudo_rng::Rng;
use lib_text_ops::INTERFACE_STRINGS as UI;

/// Entry point for the game module.
/// Demonstrates character system and action roll mechanics.
pub fn start() {
    UI.print_newline();
    UI.print_banner_dash("Game Module - Ironsworn Mechanics Demo");
    UI.print_newline();

    // Load world data
    let lp = match data_ops::get_layerpack() {
        Some(lp) => lp,
        None => {
            println!("No world loaded. Generate a world first.");
            return;
        }
    };

    // Display world info
    println!("World loaded:");
    println!("  Map size: {}x{}", lp.wi.map_size, lp.wi.map_size);
    println!("  Seed: {}", lp.wi.seed);
    UI.print_newline();

    // Create a character
    println!("Creating character...");
    let stats = Stats::with_values(2, 2, 3, 1, 3); // Edge 2, Heart 2, Iron 3, Shadow 1, Wits 3
    let mut character = Character::with_stats("Wanderer".to_string(), stats);
    
    println!("Character: {}", character.name);
    println!("  Edge: {}, Heart: {}, Iron: {}, Shadow: {}, Wits: {}",
             character.stats.edge,
             character.stats.heart,
             character.stats.iron,
             character.stats.shadow,
             character.stats.wits);
    println!("  Health: {}/{}", character.resources.health.current(), character.resources.health.max());
    println!("  Spirit: {}/{}", character.resources.spirit.current(), character.resources.spirit.max());
    println!("  Supply: {}/{}", character.resources.supply.current(), character.resources.supply.max());
    println!("  Momentum: {}", character.resources.momentum.current());
    UI.print_newline();

    // Add a vow
    let vow = Vow::new("Explore the ancient ruins".to_string(), VowRank::Dangerous);
    character.add_vow(vow);
    println!("Vow sworn: {}", character.vows[0].description);
    println!("  Rank: {:?}", character.vows[0].rank);
    UI.print_newline();

    // Demonstrate action rolls
    let mut rng = Rng::new(lp.wi.seed as u64);
    
    println!("--- Action Roll Examples ---");
    UI.print_newline();

    // Roll 1: Face Danger (Wits)
    println!("1. Face Danger (Wits + 3):");
    let roll = ActionRoll::roll_with_stat(&mut rng, Stat::Wits, &character.stats, 0);
    print_roll_result(&roll);
    UI.print_newline();

    // Roll 2: Clash (Iron)
    println!("2. Clash (Iron + 3):");
    let roll = ActionRoll::roll_with_stat(&mut rng, Stat::Iron, &character.stats, 0);
    print_roll_result(&roll);
    UI.print_newline();

    // Roll 3: Compel (Heart)
    println!("3. Compel (Heart + 2):");
    let roll = ActionRoll::roll_with_stat(&mut rng, Stat::Heart, &character.stats, 0);
    print_roll_result(&roll);
    UI.print_newline();

    println!("--- Demo Complete ---");
    println!("Phase 1.1 & 1.2 implemented: Character system and action rolls.");
    println!("Next: Full move catalog and narrative integration.");
    UI.print_newline();
}

/// Print the result of an action roll.
fn print_roll_result(roll: &ActionRoll) {
    println!("  Action: {} + {} + {} = {}",
             roll.action_die,
             roll.stat_value,
             roll.modifier,
             roll.action_total);
    println!("  Challenge: {} vs {}", roll.challenge_die_1, roll.challenge_die_2);
    
    let result_text = if roll.is_match() {
        "MATCH (special outcome!)"
    } else if roll.is_strong_hit() {
        "STRONG HIT"
    } else if roll.is_weak_hit() {
        "WEAK HIT"
    } else {
        "MISS"
    };
    
    println!("  Result: {}", result_text);
}
