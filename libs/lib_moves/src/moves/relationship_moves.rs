// [Revision: v0.1] [Path: libs/lib_moves/src/moves/relationship_moves.rs] [Date: 2025-12-13]
// Description: Social and relationship moves

use crate::action_roll::ActionRoll;
use crate::move_outcome::{MoveOutcome, MoveType};
use lib_character::character::Character;
use lib_character::stats::Stat;
use lib_pseudo_rng::Rng;

/// Compel: When you try to persuade, charm, or threaten.
pub fn compel(
    character: &mut Character,
    stat: Stat,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, stat, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Compel".to_string(),
            "They'll do what you want or share what they know. Take +1 momentum.".to_string(),
        )
        .with_momentum(1)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Action,
            "Compel".to_string(),
            "They'll do it, but you must offer something in return or make a promise.".to_string(),
        )
    } else {
        MoveOutcome::new(
            MoveType::Action,
            "Compel".to_string(),
            "They refuse or make a demand. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Sojourn: When you spend time in a community seeking assistance.
pub fn sojourn(
    character: &mut Character,
    modifier: i8,
    rng: &mut Rng,
) -> (ActionRoll, MoveOutcome) {
    let roll = ActionRoll::roll_with_stat(rng, Stat::Heart, &character.stats, modifier);
    
    let outcome = if roll.is_strong_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Sojourn".to_string(),
            "You find comfort and aid. Choose two: +2 Health, +2 Spirit, +2 Supply, +2 Momentum.".to_string(),
        )
        .with_health(2)
        .with_momentum(2)
    } else if roll.is_weak_hit() {
        MoveOutcome::new(
            MoveType::Recover,
            "Sojourn".to_string(),
            "You find aid but face a cost. Choose one: +2 Health, +2 Spirit, +2 Supply, +2 Momentum.".to_string(),
        )
        .with_health(2)
    } else {
        MoveOutcome::new(
            MoveType::Recover,
            "Sojourn".to_string(),
            "You find no help here. Pay the Price.".to_string(),
        )
    };
    
    outcome.apply_to_character(character);
    (roll, outcome)
}

/// Forge a Bond: When you make a strong connection with someone or a community.
pub fn forge_bond(
    character: &mut Character,
    name: String,
    description: String,
) -> MoveOutcome {
    let bond = lib_character::vows::Bond::new(name.clone(), description.clone());
    character.add_bond(bond);
    
    MoveOutcome::new(
        MoveType::Action,
        "Forge a Bond".to_string(),
        format!("You form a bond with: {}", name),
    )
}
