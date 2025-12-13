// [Revision: v0.1] [Path: apps/game/src/game_loop.rs] [Date: 2025-12-13]
// Description: Interactive gameplay loop

use crate::game_state::{GamePhase, GameState};
use lib_character::stats::{Stat, Stats};
use lib_character::vows::VowRank;
use lib_location::location_type::{Location, LocationRank, LocationType};
use lib_location::region::{Region, RegionType};
use lib_moves::moves::{adventure_moves, combat_moves, quest_moves};
use lib_text_ops::{input, INTERFACE_STRINGS as UI, GAME_STRINGS as GS};
use lib_travel::journey::Journey;
use lib_unit_systems::coords::Coord;

pub fn run_game_loop(mut state: GameState) {
    UI.print_newline();
    UI.print_banner_dash(GS.str_banner_ironsworn());
    UI.print_newline();

    // Character creation
    if state.phase == GamePhase::CharacterCreation {
        create_character(&mut state);
        state.phase = GamePhase::Exploration;
    }

    // Set up initial region
    setup_initial_world(&mut state);

    // Main game loop
    loop {
        state.check_game_over();

        match state.phase {
            GamePhase::Exploration => exploration_loop(&mut state),
            GamePhase::Combat => combat_loop(&mut state),
            GamePhase::Camp => camp_loop(&mut state),
            GamePhase::QuestManagement => quest_loop(&mut state),
            GamePhase::GameOver => {
                game_over(&state);
                break;
            }
            _ => break,
        }
    }
}

fn create_character(state: &mut GameState) {
    UI.print_banner_block(GS.str_banner_char_creation());
    UI.print_newline();

    GS.print_char_name(&state.character.name);
    GS.print_char_stats_prompt();
    GS.print_char_stats_default();
    UI.print_newline();

    GS.print_char_use_default();
    let choice = input::prompt_option();

    if choice.to_lowercase() != "y" {
        GS.print_char_using_default();
    }

    state.character.stats = Stats::with_values(2, 2, 3, 1, 3);
    display_character_stats(&state.character);
}

fn setup_initial_world(state: &mut GameState) {
    let mut region = Region::new(
        "Ironwood Vale".to_string(),
        "A dense forest region with ancient ruins".to_string(),
        RegionType::Forest,
    );

    region.add_location(Location::new(
        "Thornhaven".to_string(),
        "Your home village".to_string(),
        LocationType::Settlement,
        LocationRank::Troublesome,
        Coord { x: 128, y: 128 },
    ));

    region.add_location(Location::new(
        "Forgotten Temple".to_string(),
        "Ancient ruins in the deep forest".to_string(),
        LocationType::Ruin,
        LocationRank::Dangerous,
        Coord { x: 180, y: 145 },
    ));

    region.add_location(Location::new(
        "Ironwood Grove".to_string(),
        "A sacred grove of ancient trees".to_string(),
        LocationType::Sacred,
        LocationRank::Troublesome,
        Coord { x: 100, y: 150 },
    ));

    state.current_region = Some(region);
}

fn exploration_loop(state: &mut GameState) {
    UI.print_newline();
    UI.print_separator_thick("");
    
    // Context header: Turn | Location | Region
    let location_name = state.current_region.as_ref()
        .and_then(|r| r.locations.first())
        .map(|l| l.name.as_str())
        .unwrap_or("Unknown");
    let region_name = state.current_region.as_ref()
        .map(|r| r.name.as_str())
        .unwrap_or("Unknown Region");
    
    println!("  Turn {} | {} | {}", state.turn, location_name, region_name);
    UI.print_separator_thin("");
    display_character_summary(&state.character);
    display_active_vows(&state.character);
    UI.print_separator_thin("");

    GS.print_exploration_menu();

    let choice = input::prompt_option();

    match choice.as_str() {
        "1" => {
            display_character_full(&state.character);
        }
        "2" => {
            state.phase = GamePhase::QuestManagement;
        }
        "3" => {
            start_journey_menu(state);
        }
        "4" => {
            state.phase = GamePhase::Camp;
        }
        "5" => {
            explore_location(state);
        }
        "6" => {
            gather_information_action(state);
        }
        "7" => {
            state.phase = GamePhase::GameOver;
        }
        _ => {
            GS.print_invalid_choice();
        }
    }

    state.advance_turn();
}

fn combat_loop(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_combat());
    GS.print_combat_encounter();
    UI.print_newline();

    GS.print_combat_menu();

    let choice = input::prompt_option();

    match choice.as_str() {
        "1" => {
            let (roll, outcome) = combat_moves::strike(&mut state.character, Stat::Edge, 0, &mut state.rng);
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            GS.print_press_continue();
            input::prompt_option();

            if roll.is_miss() {
                GS.print_combat_enemy_strikes();
                let (roll, outcome) = combat_moves::endure_harm(&mut state.character, 2, &mut state.rng);
                display_roll_result(&roll);
                println!("  {}", outcome.description);
                UI.print_separator_thin("");
                GS.print_press_continue();
                input::prompt_option();
            }
        }
        "2" => {
            let (roll, outcome) = combat_moves::clash(&mut state.character, 0, &mut state.rng);
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            GS.print_press_continue();
            input::prompt_option();
        }
        "3" => {
            let (roll, outcome) = adventure_moves::face_danger(&mut state.character, Stat::Shadow, 0, &mut state.rng);
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");

            if roll.is_hit() {
                GS.print_combat_escape_success();
            }
            GS.print_press_continue();
            input::prompt_option();
            
            if roll.is_hit() {
                state.phase = GamePhase::Exploration;
            }
        }
        _ => {
            GS.print_invalid_choice();
            return;
        }
    }

    // Check if combat continues
    if state.phase == GamePhase::Combat {
        GS.print_combat_continue();
        let choice = input::prompt_option();
        if choice.to_lowercase() != "y" {
            state.phase = GamePhase::Exploration;
        }
    }
}

fn camp_loop(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_camp());
    GS.print_camp_setup();
    UI.print_newline();

    let (roll, outcome) = adventure_moves::make_camp(&mut state.character, 0, &mut state.rng);
    display_roll_result(&roll);
    println!("{}", outcome.description);
    UI.print_newline();

    display_character_summary(&state.character);
    UI.print_newline();

    GS.print_press_continue();
    input::prompt_option();

    state.phase = GamePhase::Exploration;
}

fn quest_loop(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_vows());
    UI.print_newline();

    GS.print_vows_active();
    let active_vows = state.character.active_vows();
    if active_vows.is_empty() {
        GS.print_vows_none();
    } else {
        for vow in active_vows.iter() {
            println!("  {} {} ({:?})",
                     make_progress_bar(vow.progress.progress(), 40),
                     vow.description, vow.rank);
        }
    }
    UI.print_newline();

    GS.print_vows_menu();

    let choice = input::prompt_option();

    match choice.as_str() {
        "1" => {
            swear_vow_menu(state);
        }
        "2" => {
            GS.print_vows_details_stub();
        }
        "3" => {
            state.phase = GamePhase::Exploration;
        }
        _ => {
            GS.print_invalid_choice();
        }
    }
}

fn swear_vow_menu(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_swear_vow());
    GS.print_swear_vow_menu();

    let choice = input::prompt_option();

    let (description, rank) = match choice.as_str() {
        "1" => ("Explore the Forgotten Temple and uncover its secrets".to_string(), VowRank::Dangerous),
        "2" => ("Protect Thornhaven from the bandit threat".to_string(), VowRank::Formidable),
        "3" => ("Find the lost Ironwood Artifact".to_string(), VowRank::Extreme),
        _ => {
            GS.print_invalid_choice();
            return;
        }
    };

    let outcome = quest_moves::swear_iron_vow(&mut state.character, description, rank);
    println!("  {}", outcome.description);
    
    UI.print_separator_thin("");
    GS.print_press_continue();
    input::prompt_option();

    state.phase = GamePhase::Exploration;
}

fn start_journey_menu(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_journey());

    let region = match &state.current_region {
        Some(r) => r,
        None => {
            GS.print_journey_no_region();
            return;
        }
    };

    GS.print_journey_destinations();
    for (i, location) in region.locations.iter().enumerate() {
        println!("  [{}] {} ({:?})", i + 1, location.name, location.location_type);
    }
    UI.print_newline();

    GS.print_journey_choose();
    let choice = input::prompt_option();

    let index: usize = match choice.parse::<usize>() {
        Ok(n) if n > 0 && n <= region.locations.len() => n - 1,
        _ => {
            GS.print_invalid_choice();
            return;
        }
    };

    let destination = &region.locations[index];
    let journey = Journey::new(
        "Current Location".to_string(),
        destination.name.clone(),
        destination.rank,
    );

    UI.print_separator_thin("");
    GS.print_journey_started(&destination.name);
    GS.print_journey_rank(format!("{:?}", journey.rank));
    UI.print_separator_thin("");

    state.active_journey = Some(journey);

    // Immediately undertake first leg
    undertake_journey_action(state);
}

fn undertake_journey_action(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_undertake_journey());

    let difficulty = state.current_region.as_ref()
        .map(|r| r.travel_difficulty())
        .unwrap_or(0);

    let (roll, outcome) = adventure_moves::undertake_journey(
        &mut state.character,
        Stat::Wits,
        difficulty,
        &mut state.rng,
    );

    display_roll_result(&roll);
    println!("  {}", outcome.description);

    if let Some(journey) = &mut state.active_journey {
        if roll.is_hit() {
            journey.mark_progress();
            println!("  Journey progress: {} {}/40", 
                     make_progress_bar(journey.progress.progress(), 40),
                     journey.progress.progress());
        }
        journey.advance_day();
    }

    UI.print_separator_thin("");
    GS.print_press_continue();
    input::prompt_option();
}

fn explore_location(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_explore());
    
    let (roll, outcome) = adventure_moves::secure_advantage(
        &mut state.character,
        Stat::Wits,
        0,
        &mut state.rng,
    );

    display_roll_result(&roll);
    println!("  {}", outcome.description);

    if roll.is_strong_hit() {
        println!();
        GS.print_explore_discover();
        // Could trigger discovery, combat, etc.
        let random_roll = state.rng.d10();
        if random_roll <= 3 {
            GS.print_explore_creature();
            UI.print_separator_thin("");
            GS.print_press_continue();
            input::prompt_option();
            state.phase = GamePhase::Combat;
            return;
        }
    }
    
    UI.print_separator_thin("");
    GS.print_press_continue();
    input::prompt_option();
}

fn gather_information_action(state: &mut GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_gather_info());
    
    let (roll, outcome) = adventure_moves::gather_information(
        &mut state.character,
        Stat::Wits,
        0,
        &mut state.rng,
    );

    display_roll_result(&roll);
    println!("  {}", outcome.description);
    
    UI.print_separator_thin("");
    GS.print_press_continue();
    input::prompt_option();
}

fn game_over(state: &GameState) {
    UI.print_newline();
    UI.print_separator_thick("");
    UI.print_banner_block(GS.str_banner_game_over());
    UI.print_separator_thick("");

    if state.character.resources.health.is_depleted() {
        GS.print_over_fallen();
    } else if state.character.resources.spirit.is_depleted() {
        GS.print_over_broken();
    } else {
        GS.print_over_journey_ends();
    }

    UI.print_newline();
    GS.print_over_statistics();
    GS.print_over_turns(state.turn);
    GS.print_over_vows_completed(state.character.vows.iter().filter(|v| v.completed).count());
    GS.print_over_vows_active(state.character.active_vow_count());
    UI.print_newline();
}

// Helper functions

fn display_character_stats(character: &lib_character::character::Character) {
    GS.print_stats_header();
    println!("  Edge: {}, Heart: {}, Iron: {}, Shadow: {}, Wits: {}",
             character.stats.edge, character.stats.heart, character.stats.iron,
             character.stats.shadow, character.stats.wits);
    UI.print_newline();
}

fn display_character_summary(character: &lib_character::character::Character) {
    let health = character.resources.health.current();
    let spirit = character.resources.spirit.current();
    let supply = character.resources.supply.current();
    let momentum = character.resources.momentum.current();
    
    println!("  Health: {}  Spirit: {}  Supply: {}",
             make_bar(health, 5), make_bar(spirit, 5), make_bar(supply, 5));
    println!("  Momentum: {:+}", momentum);
}

// Helper: Create visual bar like ●●●○○
fn make_bar(current: i8, max: i8) -> String {
    let filled = current.max(0) as usize;
    let empty = (max - current).max(0) as usize;
    format!("{}{}", "●".repeat(filled), "○".repeat(empty))
}

// Helper: Create progress bar like [■■■■□□□□□□]
fn make_progress_bar(current: u8, max: u8) -> String {
    let filled = ((current as f32 / max as f32) * 10.0).round() as usize;
    let empty = 10 - filled;
    format!("[{}{}]", "■".repeat(filled), "□".repeat(empty))
}

// Helper: Display active vows with progress bars in compact form
fn display_active_vows(character: &lib_character::character::Character) {
    let active_vows = character.active_vows();
    if !active_vows.is_empty() {
        println!("  VOWS:");
        for vow in active_vows.iter().take(3) {
            println!("  {} {} ({:?})",
                     make_progress_bar(vow.progress.progress(), 40),
                     truncate_string(&vow.description, 35), vow.rank);
        }
    }
}

// Helper: Truncate string with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len-3])
    } else {
        s.to_string()
    }
}

fn display_character_full(character: &lib_character::character::Character) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_char_sheet());
    println!("{}Name: {}", UI.str_bul1(), character.name);
    UI.print_separator_thin("");
    display_character_stats(character);
    display_character_summary(character);
    UI.print_newline();
    println!("{}Vows: {}", UI.str_bul1(), character.vows.len());
    println!("{}Bonds: {}", UI.str_bul1(), character.bond_count());
    UI.print_newline();
    GS.print_press_continue();
    input::prompt_option();
}

fn display_roll_result(roll: &lib_moves::action_roll::ActionRoll) {
    UI.print_separator_thin("");
    println!("  Rolling...");
    println!();
    println!("  ACTION:    [{}] + {} + {} = {}",
             roll.action_die, roll.stat_value, roll.modifier, roll.action_total);
    println!("  CHALLENGE: [{}]  [{}]", roll.challenge_die_1, roll.challenge_die_2);
    println!();
    
    let result = if roll.is_match() {
        "★★ MATCH! ★★"
    } else if roll.is_strong_hit() {
        "★ STRONG HIT ★"
    } else if roll.is_weak_hit() {
        "◆ WEAK HIT ◆"
    } else {
        "✗ MISS ✗"
    };
    
    println!("  {}", result);
    println!();
}
