// [Revision: v0.1] [Path: libs/lib_moves/src/moves/adventure_moves.rs] [Date: 2025-12-13]
// Description: Adventure and exploration moves

use crate::action_roll::ActionRoll;
use crate::move_outcome::{MoveOutcome, MoveType};
use lib_character::character::Character;
use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;

/// Face Danger: When you attempt something risky or react to an imminent threat.
pub fn face_danger(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Face Danger".to_string(),
            "You overcome the danger successfully.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Face Danger".to_string(),
            "You succeed, but face a troublesome cost.".to_string(),
        )
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Face Danger".to_string(),
            "You fail, or your progress is undermined by a dramatic turn of events.".to_string(),
        )
        .with_momentum(-1)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Secure an Advantage: When you assess a situation, prepare for action, or try to gain leverage.
pub fn secure_advantage(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Secure an Advantage".to_string(),
            "You gain advantage. Take +2 momentum.".to_string(),
        )
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Secure an Advantage".to_string(),
            "You gain advantage but face a complication. Take +1 momentum.".to_string(),
        )
        .with_momentum(1)
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Secure an Advantage".to_string(),
            "You fail to gain advantage and face a cost.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Gather Information: When you search an area, ask questions, or investigate.
pub fn gather_information(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Gather Information".to_string(),
            "You discover something helpful and specific. Take +2 momentum.".to_string(),
        )
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Gather Information".to_string(),
            "You discover something useful, but it is incomplete or complicates your quest.".to_string(),
        )
        .with_momentum(1)
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Gather Information".to_string(),
            "Your investigation unearths a dire threat or reveals an unwelcome truth.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Make Camp: When you rest and recover.
pub fn make_camp(
    character: &mut Character,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Heart, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Make Camp".to_string(),
            "You rest and recover. Choose two: +1 Health, +1 Spirit, +1 Supply, +1 Momentum.".to_string(),
        )
        .with_health(1)
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Make Camp".to_string(),
            "You rest but something is amiss. Choose one: +1 Health, +1 Spirit, +1 Supply, +1 Momentum.".to_string(),
        )
        .with_health(1)
    } else {
        MoveOutcome::new(
            MoveType::Recover,
            "Make Camp".to_string(),
            "You take no comfort. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Undertake a Journey: When you travel across hazardous or unfamiliar lands.
pub fn undertake_journey(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Undertake a Journey".to_string(),
            "You reach a waypoint. Mark progress.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Undertake a Journey".to_string(),
            "You reach a waypoint and mark progress, but suffer -1 Supply or face a peril.".to_string(),
        )
        .with_supply(-1)
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Undertake a Journey".to_string(),
            "You are waylaid by a perilous event. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}
