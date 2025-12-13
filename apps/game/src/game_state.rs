// [Revision: v0.3] [Path: apps/game/src/game_state.rs] [Date: 2025-12-13]
// Description: Game state management

use lib_character::character::Character;
use lib_combat::combat_state::CombatState;
use lib_location::region::Region;
use lib_pseudo_rng::Rng;
use lib_soul::{Chronicle, Soul, DeathCause};
use lib_soul::soul_mark::{SoulMark, MarkStrength, unfinished_vow, trauma};
use lib_travel::journey::Journey;

use crate::journal::Journal;
use crate::save_game::SaveData;

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
    /// Character has died, showing death screen.
    CharacterDeath,
    /// Reincarnation screen.
    Reincarnation,
    /// Game over (soul dissipated - true permadeath).
    GameOver,
}

/// Main game state container.
pub struct GameState {
    pub character: Character,
    pub rng: Rng,
    pub rng_seed: u64,
    pub current_region: Option<Region>,
    pub active_journey: Option<Journey>,
    pub combat_state: Option<CombatState>,
    pub phase: GamePhase,
    pub turn: u32,
    /// The player's persistent soul.
    pub soul: Soul,
    /// World chronicle recording history.
    pub chronicle: Chronicle,
    /// Player's personal journal.
    pub journal: Journal,
    /// Cause of death for current character (if dead).
    pub death_cause: Option<DeathCause>,
}

impl GameState {
    pub fn new(character_name: String, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let soul_id = Soul::generate_id(&mut rng);
        let character = Character::new(character_name.clone());
        let mut chronicle = Chronicle::new();
        let mut journal = Journal::new();
        
        // Record the character's birth
        chronicle.record_birth(&character.name, soul_id, None);
        journal.log_story(0, "Journey Begins", &format!("{} sets out into the world", character_name));
        
        Self {
            character,
            rng,
            rng_seed: seed,
            current_region: None,
            active_journey: None,
            combat_state: None,
            phase: GamePhase::CharacterCreation,
            turn: 0,
            soul: Soul::new(soul_id),
            chronicle,
            journal,
            death_cause: None,
        }
    }
    
    /// Create game state from save data.
    pub fn from_save_data(save: SaveData) -> Self {
        Self {
            character: save.character,
            rng: Rng::new(save.rng_seed),
            rng_seed: save.rng_seed,
            current_region: save.current_region,
            active_journey: None,
            combat_state: None,
            phase: GamePhase::Exploration,
            turn: save.turn,
            soul: save.soul,
            chronicle: save.chronicle,
            journal: save.journal,
            death_cause: None,
        }
    }
    
    /// Create save data from current state.
    pub fn to_save_data(&self) -> SaveData {
        SaveData::from_game_state(
            &self.character,
            self.turn,
            &self.soul,
            &self.chronicle,
            &self.journal,
            self.current_region.as_ref(),
            self.rng_seed,
        )
    }
    
    /// Create a new game with an existing soul (reincarnation).
    pub fn new_with_soul(character_name: String, seed: u64, mut soul: Soul, mut chronicle: Chronicle) -> Self {
        let rng = Rng::new(seed);
        let character = Character::new(character_name.clone());
        let mut journal = Journal::new();
        
        // Record the old name for the chronicle
        let old_name = soul.past_names.last().cloned().unwrap_or_default();
        
        // Reincarnate the soul
        soul.reincarnate();
        
        // Record in chronicle
        chronicle.record_reincarnation(&old_name, &character_name, soul.id);
        chronicle.record_birth(&character_name, soul.id, None);
        
        // Log the reincarnation in journal
        journal.log_story(0, "Reborn", &format!("{} awakens with memories of a past life", character_name));
        
        Self {
            character,
            rng,
            rng_seed: seed,
            current_region: None,
            active_journey: None,
            combat_state: None,
            phase: GamePhase::CharacterCreation,
            turn: 0,
            soul,
            chronicle,
            journal,
            death_cause: None,
        }
    }

    pub fn advance_turn(&mut self) {
        self.turn += 1;
        self.chronicle.set_turn(self.turn);
    }

    pub fn is_character_defeated(&self) -> bool {
        self.character.resources.health.is_depleted()
            || self.character.resources.spirit.is_depleted()
    }
    
    /// Handle character death.
    pub fn handle_death(&mut self, cause: DeathCause) {
        // Record in chronicle
        self.chronicle.record_death(
            &self.character.name,
            self.soul.id,
            &cause,
            None, // TODO: get current location
        );
        
        // Create soul marks based on death
        self.create_death_marks(&cause);
        
        // Process soul death
        self.soul.on_death(self.character.name.clone(), &cause, &mut self.rng);
        
        // Store death cause
        self.death_cause = Some(cause);
        
        // Check for soul dissipation (true permadeath)
        if self.soul.is_dissipated() {
            self.phase = GamePhase::GameOver;
        } else {
            self.phase = GamePhase::CharacterDeath;
        }
    }
    
    /// Create soul marks based on how the character died.
    fn create_death_marks(&mut self, cause: &DeathCause) {
        match cause {
            DeathCause::Combat { enemy_name } => {
                // Fear of the enemy type
                self.soul.add_mark(trauma(
                    enemy_name.clone(), 
                    MarkStrength::Clear
                ));
            }
            DeathCause::Sacrifice { for_whom } => {
                // Soul bond to the person saved
                self.soul.add_mark(
                    SoulMark::new(
                        lib_soul::soul_mark::MarkType::SoulBond,
                        MarkStrength::Strong,
                        for_whom.clone(),
                    ).with_description(format!("Gave life for {}", for_whom))
                );
            }
            DeathCause::Betrayal { betrayer } => {
                // Strong mark about betrayer
                self.soul.add_mark(
                    SoulMark::new(
                        lib_soul::soul_mark::MarkType::Fear,
                        MarkStrength::Strong,
                        betrayer.clone(),
                    ).with_description(format!("Betrayed by {}", betrayer))
                );
            }
            DeathCause::VowForsaken { vow_names } => {
                // Purpose marks for unfulfilled vows
                for vow in vow_names {
                    self.soul.add_mark(unfinished_vow(
                        vow.clone(),
                        MarkStrength::Clear,
                    ));
                }
            }
            _ => {}
        }
        
        // Add marks for any active vows
        for vow in &self.character.vows {
            if !vow.completed {
                self.soul.add_mark(unfinished_vow(
                    &vow.description,
                    MarkStrength::Faint,
                ));
            }
        }
    }

    pub fn check_game_over(&mut self) {
        if self.is_character_defeated() {
            // Default to combat death
            self.handle_death(DeathCause::Combat { 
                enemy_name: "unknown foe".to_string() 
            });
        }
    }
    
    /// Check if player can reincarnate.
    pub fn can_reincarnate(&self) -> bool {
        !self.soul.is_dissipated()
    }
    
    /// Get soul summary for display.
    pub fn soul_summary(&self) -> String {
        self.soul.summary()
    }
}
