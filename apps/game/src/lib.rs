// [Revision: v0.4] [Path: apps/game/src/lib.rs] [Date: 2025-12-13]
// Description: Game module with Phase 3 - Full move catalog demo

pub mod data_ops;
pub mod struct_ops;

use lib_character::character::Character;
use lib_character::stats::Stats;
use lib_character::vows::VowRank;
use lib_location::location_type::{Location, LocationRank, LocationType};
use lib_location::region::{Region, RegionType};
use lib_moves::action_roll::ActionRoll;
use lib_moves::moves::{adventure_moves, combat_moves, fate_moves, quest_moves};
use lib_pseudo_rng::Rng;
use lib_text_ops::INTERFACE_STRINGS as UI;
use lib_travel::encounter::generate_encounter;
use lib_travel::journey::Journey;
use lib_unit_systems::coords::Coord;

/// Entry point for the game module.
/// Demonstrates Phase 1-3: Complete Ironsworn mechanics.
pub fn start() {
    UI.print_newline();
    UI.print_banner_dash("Game Module - Complete Ironsworn System");
    UI.print_newline();

    let lp = match data_ops::get_layerpack() {
        Some(lp) => lp,
        None => {
            println!("No world loaded. Generate a world first.");
            return;
        }
    };

    println!("World loaded:");
    println!("  Map size: {}x{}", lp.wi.map_size, lp.wi.map_size);
    println!("  Seed: {}", lp.wi.seed);
    UI.print_newline();

    // === CHARACTER CREATION ===
    println!("=== CHARACTER CREATION ===");
    UI.print_newline();

    let stats = Stats::with_values(2, 2, 3, 1, 3);
    let mut character = Character::with_stats("Wanderer".to_string(), stats);
    
    println!("Character: {}", character.name);
    println!("  Stats - Edge: {}, Heart: {}, Iron: {}, Shadow: {}, Wits: {}",
             character.stats.edge, character.stats.heart, character.stats.iron,
             character.stats.shadow, character.stats.wits);
    println!("  Resources - Health: {}/{}, Spirit: {}/{}, Supply: {}/{}, Momentum: {}",
             character.resources.health.current(), character.resources.health.max(),
             character.resources.spirit.current(), character.resources.spirit.max(),
             character.resources.supply.current(), character.resources.supply.max(),
             character.resources.momentum.current());
    UI.print_newline();

    let mut rng = Rng::new(lp.wi.seed as u64);

    // === SWEAR IRON VOW ===
    println!("=== SWEAR IRON VOW ===");
    let vow_outcome = quest_moves::swear_iron_vow(
        &mut character,
        "Explore the Forgotten Temple and uncover its secrets".to_string(),
        VowRank::Dangerous,
    );
    println!("{}", vow_outcome.description);
    UI.print_newline();

    // === SET UP WORLD ===
    let mut region = Region::new(
        "Ironwood Vale".to_string(),
        "A dense forest with ancient ruins".to_string(),
        RegionType::Forest,
    );

    region.add_location(Location::new(
        "Thornhaven".to_string(),
        "A fortified village".to_string(),
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

    let mut journey = Journey::new(
        "Thornhaven".to_string(),
        "Forgotten Temple".to_string(),
        LocationRank::Dangerous,
    );

    println!("Region: {}", region.name);
    println!("Journey: {} -> {} (Rank: {:?})", journey.origin, journey.destination, journey.rank);
    UI.print_newline();

    // === DAY 1: UNDERTAKE A JOURNEY ===
    println!("=== DAY 1: UNDERTAKE A JOURNEY ===");
    journey.advance_day();
    
    let (roll, outcome) = adventure_moves::undertake_journey(
        &mut character,
        lib_character::stats::Stat::Wits,
        region.travel_difficulty(),
        &mut rng,
    );
    
    print_roll_result(&roll);
    println!("{}", outcome.description);
    
    if roll.is_hit() {
        journey.mark_progress();
        println!("Journey progress: {}/40", journey.progress.progress());
    }
    UI.print_newline();

    // === ENCOUNTER ===
    let encounter = generate_encounter(&mut rng, region.travel_difficulty());
    println!("Encounter: {:?}", encounter.encounter_type);
    println!("  {}", encounter.description);
    
    if let Some(stat) = encounter.stat {
        UI.print_newline();
        println!("=== FACE DANGER ===");
        let (roll, outcome) = adventure_moves::face_danger(
            &mut character,
            stat,
            encounter.difficulty,
            &mut rng,
        );
        
        print_roll_result(&roll);
        println!("{}", outcome.description);
    }
    UI.print_newline();

    // === DAY 2: COMBAT ENCOUNTER ===
    println!("=== DAY 2: COMBAT ENCOUNTER ===");
    journey.advance_day();
    
    println!("A corrupted guardian emerges from the ruins ahead!");
    UI.print_newline();

    println!("ENTER THE FRAY:");
    let (roll, outcome) = combat_moves::enter_fray(
        &mut character,
        lib_character::stats::Stat::Iron,
        0,
        &mut rng,
    );
    print_roll_result(&roll);
    println!("{}", outcome.description);
    UI.print_newline();

    if roll.is_hit() {
        println!("STRIKE:");
        let (roll, outcome) = combat_moves::strike(
            &mut character,
            lib_character::stats::Stat::Edge,
            0,
            &mut rng,
        );
        print_roll_result(&roll);
        println!("{}", outcome.description);
        UI.print_newline();

        if !roll.is_strong_hit() {
            println!("ENDURE HARM (2 harm):");
            let (roll, outcome) = combat_moves::endure_harm(&mut character, 2, &mut rng);
            print_roll_result(&roll);
            println!("{}", outcome.description);
            println!("Health is now: {}/{}", character.resources.health.current(), character.resources.health.max());
            UI.print_newline();
        }
    }

    // === MAKE CAMP ===
    println!("=== MAKE CAMP (After Combat) ===");
    let (roll, outcome) = adventure_moves::make_camp(&mut character, 0, &mut rng);
    print_roll_result(&roll);
    println!("{}", outcome.description);
    println!("Resources after rest:");
    println!("  Health: {}/{}, Spirit: {}/{}, Supply: {}/{}, Momentum: {}",
             character.resources.health.current(), character.resources.health.max(),
             character.resources.spirit.current(), character.resources.spirit.max(),
             character.resources.supply.current(), character.resources.supply.max(),
             character.resources.momentum.current());
    UI.print_newline();

    // === FINAL JOURNEY LEG ===
    println!("=== DAY 3: FINAL APPROACH ===");
    journey.advance_day();
    
    let (roll, outcome) = adventure_moves::undertake_journey(
        &mut character,
        lib_character::stats::Stat::Wits,
        region.travel_difficulty(),
        &mut rng,
    );
    
    print_roll_result(&roll);
    println!("{}", outcome.description);
    
    if roll.is_hit() {
        journey.mark_progress();
        journey.mark_progress(); // Extra progress
        println!("Journey progress: {}/40", journey.progress.progress());
    }
    UI.print_newline();

    // === REACH DESTINATION ===
    println!("=== REACH YOUR DESTINATION ===");
    let progress_score = journey.progress_score();
    let (roll, outcome) = quest_moves::reach_destination(&mut character, progress_score, &mut rng);
    print_progress_roll(&roll);
    println!("{}", outcome.description);
    journey.complete();
    UI.print_newline();

    // === ASK THE ORACLE ===
    println!("=== ASK THE ORACLE ===");
    println!("Question: Is the temple entrance guarded?");
    let answer = fate_moves::ask_oracle(&mut rng, 0); // 50/50
    println!("Answer: {:?}", answer);
    UI.print_newline();

    // === SUMMARY ===
    println!("--- DEMO COMPLETE ---");
    UI.print_newline();
    println!("Phase 3 Implemented:");
    println!("  ✓ Full move catalog (20+ moves)");
    println!("  ✓ Adventure moves (Face Danger, Undertake Journey, Make Camp, etc.)");
    println!("  ✓ Combat moves (Enter the Fray, Strike, Clash, Endure Harm)");
    println!("  ✓ Quest moves (Swear Vow, Reach Destination)");
    println!("  ✓ Fate moves (Ask Oracle, Pay the Price)");
    UI.print_newline();
    println!("Final Character State:");
    println!("  Health: {}/{}, Spirit: {}/{}, Supply: {}/{}, Momentum: {}",
             character.resources.health.current(), character.resources.health.max(),
             character.resources.spirit.current(), character.resources.spirit.max(),
             character.resources.supply.current(), character.resources.supply.max(),
             character.resources.momentum.current());
    println!("  Active Vows: {}", character.active_vow_count());
    println!("  Days Traveled: {}", journey.days_traveled);
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

/// Print the result of a progress roll.
fn print_progress_roll(roll: &lib_moves::action_roll::ProgressRoll) {
    println!("  Progress Score: {}", roll.progress_score);
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
