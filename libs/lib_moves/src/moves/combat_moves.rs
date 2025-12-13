// [Revision: v0.1] [Path: libs/lib_moves/src/moves/combat_moves.rs] [Date: 2025-12-13]
// Description: Combat moves

use crate::action_roll::ActionRoll;
use crate::move_outcome::{MoveOutcome, MoveType};
use lib_character::character::Character;
use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;

/// Enter the Fray: When you enter combat.
pub fn enter_fray(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Enter the Fray".to_string(),
            "You take initiative. Take +2 momentum.".to_string(),
        )
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Enter the Fray".to_string(),
            "You are in a bad position or lose advantage. Choose one: +2 momentum or +1 momentum and initiative.".to_string(),
        )
        .with_momentum(1)
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Enter the Fray".to_string(),
            "Combat begins with you in a bad spot. Pay the Price.".to_string(),
        )
        .with_momentum(-1)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Strike: When you attack in melee or at range.
pub fn strike(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Strike".to_string(),
            "You inflict harm. Mark progress twice.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Strike".to_string(),
            "You inflict harm and mark progress, but you expose yourself to danger.".to_string(),
        )
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Strike".to_string(),
            "Your attack fails. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Clash: When you fight in close quarters and your foe attacks.
pub fn clash(
    character: &mut Character,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Iron, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Clash".to_string(),
            "You inflict harm. Mark progress twice. You retain initiative.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Clash".to_string(),
            "You inflict harm and mark progress, but you suffer harm in return.".to_string(),
        )
        .with_health(-1)
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Clash".to_string(),
            "You are defeated or outmatched. Pay the Price.".to_string(),
        )
        .with_health(-2)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Endure Harm: When you face physical damage.
pub fn endure_harm(
    character: &mut Character,
    harm: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    // Apply harm first
    character.resources.health.modify(-harm);
    
    let roll = ActionRoll::roll_with_stat(rng, Stat::Iron, &character.stats, 0);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Harm".to_string(),
            format!("You suffer {} harm but shake it off. Choose one: +1 momentum or +1 health.", harm),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Harm".to_string(),
            format!("You suffer {} harm and press on.", harm),
        )
    } else {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Harm".to_string(),
            format!("You suffer {} harm and also suffer -1 momentum. If health is 0, you are incapacitated.", harm),
        )
        .with_momentum(-1)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Endure Stress: When you face mental shock or despair.
pub fn endure_stress(
    character: &mut Character,
    stress: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    // Apply stress first
    character.resources.spirit.modify(-stress);
    
    let roll = ActionRoll::roll_with_stat(rng, Stat::Heart, &character.stats, 0);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Stress".to_string(),
            format!("You suffer {} stress but steel yourself. Choose one: +1 momentum or +1 spirit.", stress),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Stress".to_string(),
            format!("You suffer {} stress and press on.", stress),
        )
    } else {
        MoveOutcome::new(
            MoveType::Suffer,
            "Endure Stress".to_string(),
            format!("You suffer {} stress and also suffer -1 momentum. If spirit is 0, you are traumatized.", stress),
        )
        .with_momentum(-1)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}
