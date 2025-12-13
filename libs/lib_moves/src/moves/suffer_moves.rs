// [Revision: v0.1] [Path: libs/lib_moves/src/moves/suffer_moves.rs] [Date: 2025-12-13]
// Description: Suffer moves and recovery

use crate::action_roll::ActionRoll;
use crate::move_outcome::{MoveOutcome, MoveType};
use lib_character::character::Character;
use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;

/// Face Death: When you are brought to the brink of death.
pub fn face_death(
    character: &mut Character,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Heart, &character.stats, 0);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Death".to_string(),
            "You are cast back into the mortal world. Set health to 1 and take +2 momentum.".to_string(),
        )
        .with_health(1)
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Death".to_string(),
            "You survive, but death leaves its mark. Set health to 1 and suffer a debility.".to_string(),
        )
        .with_health(1)
    } else {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Death".to_string(),
            "You die. Your story ends here.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Face Desolation: When you are brought to the brink of sanity.
pub fn face_desolation(
    character: &mut Character,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Heart, &character.stats, 0);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Desolation".to_string(),
            "You resist and press on. Set spirit to 1 and take +2 momentum.".to_string(),
        )
        .with_spirit(1)
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Desolation".to_string(),
            "You survive, but are traumatized. Set spirit to 1 and suffer a debility.".to_string(),
        )
        .with_spirit(1)
    } else {
        MoveOutcome::new(
            MoveType::Suffer,
            "Face Desolation".to_string(),
            "You are broken. You give up your quest and forsake your vows.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Heal: When you treat an injury or ailment.
pub fn heal(
    character: &mut Character,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Wits, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Heal".to_string(),
            "Treatment is successful. Take or give +2 health.".to_string(),
        )
        .with_health(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Heal".to_string(),
            "Treatment is helpful but slow. Take or give +1 health. Suffer -1 supply.".to_string(),
        )
        .with_health(1)
        .with_supply(-1)
    } else {
        MoveOutcome::new(
            MoveType::Recover,
            "Heal".to_string(),
            "Treatment fails. Pay the Price.".to_string(),
        )
        .with_supply(-1)
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Resupply: When you seek provisions and gear.
pub fn resupply(
    character: &mut Character,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Wits, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Resupply".to_string(),
            "You acquire supplies. Take +2 supply.".to_string(),
        )
        .with_supply(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Resupply".to_string(),
            "You acquire some supplies. Take +1 supply.".to_string(),
        )
        .with_supply(1)
    } else {
        MoveOutcome::new(
            MoveType::Recover,
            "Resupply".to_string(),
            "You find nothing useful. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}
