// [Revision: v0.1] [Path: libs/lib_moves/src/moves/quest_moves.rs] [Date: 2025-12-13]
// Description: Quest and vow moves

use crate::action_roll::ProgressRoll;
use crate::move_outcome::{MoveOutcome, MoveType};
use lib_character::character::Character;
use lib_character::vows::VowRank;
use lib_pseudo_rng::Rng;

/// Swear an Iron Vow: When you swear upon iron to complete a quest.
pub fn swear_iron_vow(
    character: &mut Character,
    description: String,
    rank: VowRank,
) -> MoveOutcome {
    let vow = lib_character::vows::Vow::new(description.clone(), rank);
    character.add_vow(vow);
    
    MoveOutcome::new(
        MoveType::Action,
        "Swear an Iron Vow".to_string(),
        format!("You swear to: {}", description),
    )
}

/// Reach Your Destination: When you complete a journey.
pub fn reach_destination(
    character: &mut Character,
    progress_score: u8,
    rng: &mut Rng,
) -> (ProgressRoll, MoveOutcome) {
    let roll = ProgressRoll::roll(rng, progress_score);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Progress,
            "Reach Your Destination".to_string(),
            "You reach your destination and the situation is favorable. Take +1 momentum.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Progress,
            "Reach Your Destination".to_string(),
            "You arrive but face a cost or complication.".to_string(),
        )
    } else {
        MoveOutcome::new(
            MoveType::Progress,
            "Reach Your Destination".to_string(),
            "You are waylaid by a crisis or arrive to find a dire situation.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Fulfill Your Vow: When your quest is complete.
pub fn fulfill_vow(
    character: &mut Character,
    vow_index: usize,
    rng: &mut Rng,
) -> Option<(ProgressRoll, MoveOutcome)> {
    let vow = character.vow_mut(vow_index)?;
    let progress_score = vow.progress.progress_score();
    
    let roll = ProgressRoll::roll(rng, progress_score);
    
    let outcome = if roll.is_strong_hit() {
        vow.complete();
        MoveOutcome::new(
            MoveType::Progress,
            "Fulfill Your Vow".to_string(),
            "Your quest is complete. Mark experience and take +2 momentum.".to_string(),
        )
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        vow.complete();
        MoveOutcome::new(
            MoveType::Progress,
            "Fulfill Your Vow".to_string(),
            "Your quest is complete, but there is an unforeseen complication. Mark experience.".to_string(),
        )
    } else {
        MoveOutcome::new(
            MoveType::Progress,
            "Fulfill Your Vow".to_string(),
            "Your quest is undone. Choose: forsake your vow or recommit and clear all progress.".to_string(),
        )
        .with_momentum(-1)
    };
    
    outcome.apply_to_character(character);
    Some((roll, outcome))
}

/// Forsake Your Vow: When you give up on a quest.
pub fn forsake_vow(character: &mut Character, vow_index: usize) -> Option<MoveOutcome> {
    let vow = character.vow_mut(vow_index)?;
    vow.forsake();
    
    let outcome = MoveOutcome::new(
        MoveType::Action,
        "Forsake Your Vow".to_string(),
        "You abandon your quest. Clear the vow and suffer -1 Spirit.".to_string(),
    )
    .with_spirit(-1);
    
    outcome.apply_to_character(character);
    Some(outcome)
}
