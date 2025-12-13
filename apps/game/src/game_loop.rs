// [Revision: v0.3] [Path: apps/game/src/game_loop.rs] [Date: 2025-12-13]
// Description: Interactive gameplay loop

use crate::game_state::{GamePhase, GameState};
use crate::journal::JournalCategory;
use crate::save_game::SaveData;
use lib_character::stats::{Stat, Stats};
use lib_character::vows::VowRank;
use lib_combat::combat_state::{CombatAction, CombatPhase, CombatState};
use lib_combat::encounter::generate_encounter;
use lib_combat::enemy::EnemyRank;
use lib_location::location_type::{Location, LocationRank, LocationType};
use lib_location::map_display::{MapConfig, render_region_map, render_location_list};
use lib_location::region::{Region, RegionType};
use lib_moves::moves::{adventure_moves, combat_moves, quest_moves};
use lib_oracle::Oracle;
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
            display_map(state);
        }
        "3" => {
            journal_menu(state);
        }
        "4" => {
            state.phase = GamePhase::QuestManagement;
        }
        "5" => {
            start_journey_menu(state);
        }
        "6" => {
            state.phase = GamePhase::Camp;
        }
        "7" => {
            explore_location(state);
        }
        "8" => {
            gather_information_action(state);
        }
        "9" => {
            oracle_menu(state);
        }
        "10" => {
            save_game_menu(state);
        }
        "11" => {
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
    
    // Initialize combat state if not already present
    if state.combat_state.is_none() {
        let encounter = generate_encounter(&mut state.rng, EnemyRank::Dangerous);
        state.combat_state = Some(CombatState::new(encounter));
    }
    
    // Get mutable reference to combat state
    let combat = state.combat_state.as_mut().unwrap();
    
    // Transition to player turn if entering
    if combat.phase == CombatPhase::Entering {
        if combat.player_has_initiative {
            combat.player_turn();
        } else {
            combat.enemy_turn();
        }
    }
    
    // Display enemy status
    display_combat_status(combat);
    UI.print_newline();
    
    // Display available actions and get player choice
    let actions = combat.available_actions();
    display_combat_menu(&actions);
    
    let choice = input::prompt_option();
    
    // Parse and execute action
    if let Ok(idx) = choice.parse::<usize>() {
        if idx > 0 && idx <= actions.len() {
            let action = actions[idx - 1];
            execute_combat_action(state, action);
        } else {
            GS.print_invalid_choice();
            return;
        }
    } else {
        GS.print_invalid_choice();
        return;
    }
    
    // Check combat end conditions
    check_combat_end(state);
}

/// Display current combat status with enemy info and progress bars.
fn display_combat_status(combat: &CombatState) {
    println!("  {}", combat.status());
    UI.print_separator_thin("");
    
    // Display all enemies
    for (i, enemy) in combat.encounter.active_enemies().iter().enumerate() {
        let target_marker = if i == combat.target_index { "▶ " } else { "  " };
        println!("{}{}  {}  {}", 
            target_marker,
            enemy.description(),
            enemy.progress_bar(),
            if enemy.can_end_fight() { "[Vulnerable]" } else { "" }
        );
        
        // Display enemy traits
        if !enemy.traits.is_empty() {
            println!("    Traits: {}", enemy.traits.join(", "));
        }
    }
    
    // Show defeated enemies count
    let defeated = combat.encounter.enemies.len() - combat.encounter.active_enemies().len();
    if defeated > 0 {
        println!("  ({} defeated)", defeated);
    }
}

/// Display combat menu based on available actions.
fn display_combat_menu(actions: &[CombatAction]) {
    println!("  What do you do?");
    for (i, action) in actions.iter().enumerate() {
        println!("  [{}] {} - {}", i + 1, action.name(), action.description());
    }
}

/// Execute a combat action.
fn execute_combat_action(state: &mut GameState, action: CombatAction) {
    // Get the target harm before executing action
    let target_harm = state.combat_state.as_ref()
        .and_then(|c| c.current_target())
        .map(|t| t.harm())
        .unwrap_or(2);
    
    match action {
        CombatAction::Strike => {
            let (roll, outcome) = combat_moves::strike(
                &mut state.character, 
                Stat::Edge, 
                0, 
                &mut state.rng
            );
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            
            if roll.is_strong_hit() {
                // Strong hit: mark 2 progress
                let combat = state.combat_state.as_mut().unwrap();
                combat.harm_target(2);
                combat.log_event("You strike with decisive force!");
                combat.player_turn(); // Retain initiative
            } else if roll.is_weak_hit() {
                // Weak hit: mark 1 progress, enemy counter
                let combat = state.combat_state.as_mut().unwrap();
                combat.harm_target(1);
                combat.log_event("You land a blow but expose yourself!");
                combat.enemy_turn();
                
                // Enemy counter-attack
                enemy_counter_attack(state, target_harm);
            } else {
                // Miss: enemy attacks
                let combat = state.combat_state.as_mut().unwrap();
                combat.log_event("Your strike misses!");
                combat.enemy_turn();
                
                enemy_attacks(state, target_harm);
            }
        }
        
        CombatAction::Clash => {
            let (roll, outcome) = combat_moves::clash(
                &mut state.character, 
                0, 
                &mut state.rng
            );
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            
            if roll.is_strong_hit() {
                // Strong hit: mark progress, take initiative
                let combat = state.combat_state.as_mut().unwrap();
                combat.harm_target(1);
                combat.log_event("You clash and gain the upper hand!");
                combat.player_turn();
            } else if roll.is_weak_hit() {
                // Weak hit: trade harm
                let combat = state.combat_state.as_mut().unwrap();
                combat.harm_target(1);
                combat.log_event("You trade blows with your foe!");
                
                enemy_counter_attack(state, target_harm);
            } else {
                // Miss: Pay the Price
                let combat = state.combat_state.as_mut().unwrap();
                combat.log_event("Your defense falters!");
                combat.enemy_turn();
                
                enemy_attacks(state, target_harm);
            }
        }
        
        CombatAction::FaceDanger => {
            let (roll, outcome) = adventure_moves::face_danger(
                &mut state.character, 
                Stat::Shadow, 
                0, 
                &mut state.rng
            );
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            
            if roll.is_strong_hit() {
                println!("  You successfully disengage from combat!");
                state.combat_state = None;
                state.phase = GamePhase::Exploration;
                return;
            } else if roll.is_weak_hit() {
                println!("  You escape but take harm in the process!");
                let (roll2, outcome2) = combat_moves::endure_harm(
                    &mut state.character, 
                    1, 
                    &mut state.rng
                );
                display_roll_result(&roll2);
                println!("  {}", outcome2.description);
                state.combat_state = None;
                state.phase = GamePhase::Exploration;
                return;
            } else {
                let combat = state.combat_state.as_mut().unwrap();
                combat.log_event("You fail to escape!");
                combat.enemy_turn();
                
                enemy_attacks(state, target_harm);
            }
        }
        
        CombatAction::EndFight => {
            // End the Fight move - roll progress dice vs challenge dice
            let combat = state.combat_state.as_ref().unwrap();
            let progress = combat.current_target()
                .map(|t| t.progress_boxes())
                .unwrap_or(0);
            
            let roll = lib_moves::ProgressRoll::roll(&mut state.rng, progress);
            
            // Display the progress roll result
            println!("  End the Fight: Progress {} vs [{}, {}]",
                roll.progress_score, roll.challenge_die_1, roll.challenge_die_2);
            
            if roll.is_strong_hit() {
                println!("  Strong Hit! You decisively defeat your foe!");
                let combat = state.combat_state.as_mut().unwrap();
                if let Some(target) = combat.current_target_mut() {
                    target.defeated = true;
                }
                combat.log_event("Enemy defeated with a decisive blow!");
                
                // Check if all enemies defeated
                if combat.encounter.all_defeated() {
                    println!("  Victory! All enemies defeated.");
                    state.combat_state = None;
                    state.phase = GamePhase::Exploration;
                    return;
                }
            } else if roll.is_weak_hit() {
                println!("  Weak Hit! You defeat the enemy, but at a cost.");
                let combat = state.combat_state.as_mut().unwrap();
                if let Some(target) = combat.current_target_mut() {
                    target.defeated = true;
                }
                
                // Take stress or make a story concession
                let (roll2, _outcome2) = combat_moves::endure_harm(
                    &mut state.character, 
                    1, 
                    &mut state.rng
                );
                display_roll_result(&roll2);
                
                let combat = state.combat_state.as_mut().unwrap();
                if combat.encounter.all_defeated() {
                    state.combat_state = None;
                    state.phase = GamePhase::Exploration;
                    return;
                }
            } else {
                println!("  Miss! Your attempt to finish the fight fails!");
                let combat = state.combat_state.as_mut().unwrap();
                combat.log_event("Failed to end the fight!");
                combat.enemy_turn();
                
                enemy_attacks(state, target_harm);
            }
        }
        
        CombatAction::SecureAdvantage => {
            let (roll, outcome) = adventure_moves::secure_advantage(
                &mut state.character, 
                Stat::Wits, 
                0, 
                &mut state.rng
            );
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            
            if roll.is_strong_hit() {
                state.character.resources.momentum.modify(2);
                let combat = state.combat_state.as_mut().unwrap();
                combat.player_turn();
                combat.log_event("You gain a significant advantage!");
            } else if roll.is_weak_hit() {
                state.character.resources.momentum.modify(1);
                let combat = state.combat_state.as_mut().unwrap();
                combat.player_turn();
                combat.log_event("You gain a small advantage.");
            } else {
                let combat = state.combat_state.as_mut().unwrap();
                combat.enemy_turn();
                combat.log_event("Your attempt backfires!");
            }
        }
        
        CombatAction::Flee => {
            // Same as Face Danger but with different narrative
            let (roll, outcome) = adventure_moves::face_danger(
                &mut state.character, 
                Stat::Edge, 
                0, 
                &mut state.rng
            );
            display_roll_result(&roll);
            println!("  {}", outcome.description);
            UI.print_separator_thin("");
            
            if roll.is_hit() {
                println!("  You flee from combat!");
                state.combat_state = None;
                state.phase = GamePhase::Exploration;
                return;
            } else {
                let combat = state.combat_state.as_mut().unwrap();
                combat.log_event("You fail to flee!");
                combat.enemy_turn();
                
                enemy_attacks(state, target_harm);
            }
        }
        
        _ => {
            println!("  Action not yet implemented.");
        }
    }
    
    GS.print_press_continue();
    input::prompt_option();
}

/// Handle enemy counter-attack (during weak hits).
fn enemy_counter_attack(state: &mut GameState, harm: i8) {
    println!("  The enemy strikes back!");
    let (roll, outcome) = combat_moves::endure_harm(
        &mut state.character, 
        harm, 
        &mut state.rng
    );
    display_roll_result(&roll);
    println!("  {}", outcome.description);
}

/// Handle full enemy attack (during misses or enemy turn).
fn enemy_attacks(state: &mut GameState, harm: i8) {
    println!("  The enemy attacks!");
    let (roll, outcome) = combat_moves::endure_harm(
        &mut state.character, 
        harm + 1, // Full attack deals more harm
        &mut state.rng
    );
    display_roll_result(&roll);
    println!("  {}", outcome.description);
}

/// Check if combat should end.
fn check_combat_end(state: &mut GameState) {
    // Check character defeat
    if state.is_character_defeated() {
        state.phase = GamePhase::GameOver;
        state.combat_state = None;
        return;
    }
    
    // Check victory (all enemies defeated)
    if let Some(combat) = &state.combat_state {
        if combat.encounter.all_defeated() {
            println!("\n  Victory! All enemies defeated.");
            state.combat_state = None;
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

    let outcome = quest_moves::swear_iron_vow(&mut state.character, description.clone(), rank);
    println!("  {}", outcome.description);
    
    // Log the vow in journal
    state.journal.log_quest(
        state.turn,
        "Iron Vow Sworn",
        &format!("Swore an iron vow: {}", description)
    );
    
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
    
    // Log the journey start
    state.journal.log_travel(
        state.turn,
        "Journey Begun",
        &format!("Set out for {}", destination.name)
    );

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
    
    let location_name = state.current_region.as_ref()
        .and_then(|r| r.locations.first())
        .map(|l| l.name.as_str())
        .unwrap_or("Unknown");
    
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
            // Log combat encounter
            state.journal.log_combat(
                state.turn, 
                "Hostile Encounter",
                &format!("Discovered a hostile creature while exploring {}", location_name),
                "Unknown Creature"
            );
            UI.print_separator_thin("");
            GS.print_press_continue();
            input::prompt_option();
            state.phase = GamePhase::Combat;
            return;
        } else {
            // Log successful exploration
            state.journal.log_travel(
                state.turn,
                "Exploration",
                &format!("Made a discovery while exploring {}", location_name)
            );
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

fn oracle_menu(state: &mut GameState) {
    let oracle = Oracle::new();
    
    loop {
        UI.print_newline();
        UI.print_banner_block(GS.str_banner_oracle());
        UI.print_newline();
        
        GS.print_oracle_menu();
        
        let choice = input::prompt_option();
        
        match choice.as_str() {
            "1" => {
                // Yes/No question with likelihood
                UI.print_newline();
                GS.print_oracle_likelihood();
                let likelihood_choice = input::prompt_option();
                
                let likelihood = match likelihood_choice.as_str() {
                    "1" => 90,  // Almost Certain
                    "2" => 75,  // Likely
                    "3" => 50,  // Fifty-Fifty
                    "4" => 25,  // Unlikely
                    "5" => 10,  // Small Chance
                    _ => 50,    // Default
                };
                
                let result = oracle.ask_yes_no(&mut state.rng, likelihood);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {} {}", GS.str_oracle_answer(), result.description());
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "2" => {
                // Action + Theme
                let (action, theme) = oracle.action_theme(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {} {}", GS.str_oracle_action(), action);
                println!("  {} {}", GS.str_oracle_theme(), theme);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "3" => {
                // Complication
                let result = oracle.complication(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {}", result);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "4" => {
                // Opportunity
                let result = oracle.opportunity(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {}", result);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "5" => {
                // NPC Details
                let (disposition, goal) = oracle.npc_details(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {} {}", GS.str_oracle_npc_disposition(), disposition);
                println!("  {} {}", GS.str_oracle_npc_goal(), goal);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "6" => {
                // Location Feature
                let result = oracle.location_feature(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {} {}", GS.str_oracle_feature(), result);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "7" => {
                // Pay the Price
                let result = oracle.pay_price(&mut state.rng);
                
                UI.print_separator_thin("");
                println!("  {}", GS.str_oracle_rolling());
                println!();
                println!("  {} {}", GS.str_oracle_price(), result);
                UI.print_separator_thin("");
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "8" | "" => {
                // Return
                break;
            }
            _ => {
                GS.print_invalid_choice();
            }
        }
    }
}

fn journal_menu(state: &mut GameState) {
    loop {
        UI.print_newline();
        UI.print_separator_thin("");
        GS.print_journal_banner();
        UI.print_separator_thin("");
        
        if state.journal.is_empty() {
            GS.print_journal_empty();
        } else {
            println!("  {} entries total", state.journal.len());
        }
        
        GS.print_journal_menu();
        
        let choice = input::prompt_option();
        
        match choice.as_str() {
            "1" => {
                // View recent entries
                UI.print_newline();
                GS.print_journal_recent();
                UI.print_separator_thin("");
                
                let recent = state.journal.recent(10);
                if recent.is_empty() {
                    GS.print_journal_empty();
                } else {
                    for entry in recent {
                        println!("{}", entry);
                        UI.print_newline();
                    }
                }
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "2" => {
                // View all entries
                UI.print_newline();
                GS.print_journal_all();
                UI.print_separator_thin("");
                
                if state.journal.is_empty() {
                    GS.print_journal_empty();
                } else {
                    for entry in state.journal.all_entries() {
                        println!("{}", entry);
                        UI.print_newline();
                    }
                }
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "3" => {
                // View by category
                UI.print_newline();
                GS.print_journal_category_prompt();
                
                let cat_choice = input::prompt_option();
                let category = match cat_choice.as_str() {
                    "1" => Some(JournalCategory::Story),
                    "2" => Some(JournalCategory::Combat),
                    "3" => Some(JournalCategory::Travel),
                    "4" => Some(JournalCategory::Social),
                    "5" => Some(JournalCategory::Quest),
                    "6" => Some(JournalCategory::Note),
                    "7" => Some(JournalCategory::System),
                    _ => None,
                };
                
                if let Some(cat) = category {
                    UI.print_separator_thin("");
                    let filtered = state.journal.entries_by_category(cat);
                    if filtered.is_empty() {
                        GS.print_journal_no_results();
                    } else {
                        for entry in filtered {
                            println!("{}", entry);
                            UI.print_newline();
                        }
                    }
                    
                    GS.print_press_continue();
                    input::prompt_option();
                }
            }
            "4" => {
                // Add personal note
                UI.print_newline();
                GS.print_journal_note_prompt();
                
                let note = input::prompt_option();
                if !note.trim().is_empty() {
                    state.journal.add_note(state.turn, note.trim());
                    UI.print_separator_thin("");
                    println!("  Note added.");
                }
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "5" => {
                // Search
                UI.print_newline();
                GS.print_journal_search_prompt();
                
                let query = input::prompt_option();
                if !query.trim().is_empty() {
                    UI.print_separator_thin("");
                    GS.print_journal_search_results();
                    UI.print_newline();
                    
                    let results = state.journal.search(&query);
                    if results.is_empty() {
                        GS.print_journal_no_results();
                    } else {
                        for entry in results {
                            println!("{}", entry);
                            UI.print_newline();
                        }
                    }
                }
                
                GS.print_press_continue();
                input::prompt_option();
            }
            "6" | "" => {
                // Return to exploration
                return;
            }
            _ => {
                GS.print_invalid_choice();
            }
        }
    }
}

fn save_game_menu(state: &GameState) {
    UI.print_newline();
    UI.print_separator_thin("");
    println!("  Enter save name (empty for quick save):");
    UI.print_separator_thin("");
    
    let input_str = input::prompt_option();
    let save_name = if input_str.trim().is_empty() {
        "save".to_string()
    } else {
        input_str.trim().to_string()
    };
    
    let save_data = state.to_save_data();
    let path = SaveData::named_save_path(&save_name);
    
    match save_data.save_to_file(&path) {
        Ok(()) => {
            UI.print_separator_thin("");
            println!("  Game saved: {}", save_name);
            UI.print_separator_thin("");
        }
        Err(e) => {
            UI.print_separator_thin("");
            println!("  Failed to save: {}", e);
            UI.print_separator_thin("");
        }
    }
    
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

fn display_map(state: &GameState) {
    UI.print_newline();
    UI.print_banner_block(GS.str_banner_map());
    UI.print_newline();
    
    if let Some(region) = &state.current_region {
        // Get player position (use first location as default)
        let player_pos = region.locations.first().map(|l| &l.position);
        
        // Render the map
        let config = MapConfig::default();
        let map_lines = render_region_map(region, player_pos, &config);
        
        for line in map_lines {
            println!("{}", line);
        }
        
        UI.print_newline();
        
        // Show location list
        let location_list = render_location_list(region, Some(0));
        for line in location_list {
            println!("  {}", line);
        }
    } else {
        println!("  No region data available.");
    }
    
    UI.print_newline();
    GS.print_press_continue();
    input::prompt_option();
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
