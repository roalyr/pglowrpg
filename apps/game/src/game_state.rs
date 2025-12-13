// [Revision: v0.1] [Path: apps/game/src/game_state.rs] [Date: 2025-12-13]
// Description: Game state management

use lib_character::character::Character;
use lib_location::region::Region;
use lib_pseudo_rng::Rng;
use lib_travel::journey::Journey;

/// Current phase of gameplay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    /// Character creation.
    CharacterCreation,
    /// Exploring the world, managing journeys.
    Exploration,
    /// Active combat encounter.
    Combat,
    /// Resting and recovery.
    Camp,
    /// Quest/vow management.
    QuestManagement,
    /// Game over.
    GameOver,
}

/// Main game state container.
pub struct GameState {
    pub character: Character,
    pub rng: Rng,
    pub current_region: Option<Region>,
    pub active_journey: Option<Journey>,
    pub phase: GamePhase,
    pub turn: u32,
}

impl GameState {
    pub fn new(character_name: String, seed: u64) -> Self {
        let character = Character::new(character_name);
        Self {
            character,
            rng: Rng::new(seed),
            current_region: None,
            active_journey: None,
            phase: GamePhase::CharacterCreation,
            turn: 0,
        }
    }

    pub fn advance_turn(&mut self) {
        self.turn += 1;
    }

    pub fn is_character_defeated(&self) -> bool {
        self.character.resources.health.is_depleted()
            || self.character.resources.spirit.is_depleted()
    }

    pub fn check_game_over(&mut self) {
        if self.is_character_defeated() {
            self.phase = GamePhase::GameOver;
        }
    }
}
