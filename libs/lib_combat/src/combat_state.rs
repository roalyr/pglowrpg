// [Revision: v0.1] [Path: libs/lib_combat/src/combat_state.rs] [Date: 2025-12-13]
// Description: Combat state machine

use crate::enemy::Enemy;
use crate::encounter::Encounter;
use serde::{Deserialize, Serialize};

/// Phase of combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatPhase {
    /// Combat is starting, enter the fray.
    Entering,
    /// Player's turn to act.
    PlayerTurn,
    /// Enemy's turn to act.
    EnemyTurn,
    /// Resolving an action.
    Resolving,
    /// Combat is ending (victory or defeat).
    Ending,
    /// Combat has concluded.
    Finished,
}

/// Actions available in combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatAction {
    /// Attack with Strike move.
    Strike,
    /// Defensive Clash move.
    Clash,
    /// Use Face Danger to escape or maneuver.
    FaceDanger,
    /// Attempt to end the fight.
    EndFight,
    /// Secure an advantage.
    SecureAdvantage,
    /// Compel enemy to surrender.
    Compel,
    /// Aid an ally.
    Aid,
    /// Flee combat.
    Flee,
}

impl CombatAction {
    /// Display name for this action.
    pub fn name(&self) -> &'static str {
        match self {
            CombatAction::Strike => "Strike",
            CombatAction::Clash => "Clash",
            CombatAction::FaceDanger => "Face Danger",
            CombatAction::EndFight => "End the Fight",
            CombatAction::SecureAdvantage => "Secure an Advantage",
            CombatAction::Compel => "Compel",
            CombatAction::Aid => "Aid",
            CombatAction::Flee => "Flee",
        }
    }
    
    /// Description of this action.
    pub fn description(&self) -> &'static str {
        match self {
            CombatAction::Strike => "Attack your foe with intent to harm",
            CombatAction::Clash => "Fight defensively against an attacking foe",
            CombatAction::FaceDanger => "Maneuver or overcome an obstacle",
            CombatAction::EndFight => "Attempt to decisively end the fight",
            CombatAction::SecureAdvantage => "Gain position or setup for next action",
            CombatAction::Compel => "Force enemy to yield or negotiate",
            CombatAction::Aid => "Help an ally in their action",
            CombatAction::Flee => "Attempt to escape the combat",
        }
    }
}

/// Full combat state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    /// The current encounter.
    pub encounter: Encounter,
    /// Current phase of combat.
    pub phase: CombatPhase,
    /// Whether player has initiative.
    pub player_has_initiative: bool,
    /// Index of currently targeted enemy.
    pub target_index: usize,
    /// Combat log for this encounter.
    pub log: Vec<String>,
}

impl CombatState {
    /// Create a new combat state from an encounter.
    pub fn new(encounter: Encounter) -> Self {
        let player_init = encounter.player_initiative;
        Self {
            encounter,
            phase: CombatPhase::Entering,
            player_has_initiative: player_init,
            target_index: 0,
            log: Vec::new(),
        }
    }
    
    /// Get current target enemy.
    pub fn current_target(&self) -> Option<&Enemy> {
        let active = self.encounter.active_enemies();
        active.get(self.target_index).copied()
    }
    
    /// Get mutable current target.
    pub fn current_target_mut(&mut self) -> Option<&mut Enemy> {
        let idx = self.target_index;
        self.encounter.active_enemies_mut().into_iter().nth(idx)
    }
    
    /// Switch to next target.
    pub fn next_target(&mut self) {
        let count = self.encounter.active_enemies().len();
        if count > 0 {
            self.target_index = (self.target_index + 1) % count;
        }
    }
    
    /// Add entry to combat log.
    pub fn log_event(&mut self, event: impl Into<String>) {
        self.log.push(event.into());
    }
    
    /// Get available actions based on current state.
    pub fn available_actions(&self) -> Vec<CombatAction> {
        match self.phase {
            CombatPhase::PlayerTurn => {
                let mut actions = vec![
                    CombatAction::Strike,
                    CombatAction::Clash,
                    CombatAction::FaceDanger,
                    CombatAction::SecureAdvantage,
                ];
                
                // End Fight available if enough progress on target
                if let Some(target) = self.current_target() {
                    if target.can_end_fight() {
                        actions.push(CombatAction::EndFight);
                    }
                }
                
                actions.push(CombatAction::Flee);
                actions
            }
            CombatPhase::EnemyTurn => {
                // During enemy turn, player can only Clash or Face Danger
                vec![CombatAction::Clash, CombatAction::FaceDanger]
            }
            _ => vec![],
        }
    }
    
    /// Transition to player turn.
    pub fn player_turn(&mut self) {
        self.phase = CombatPhase::PlayerTurn;
        self.player_has_initiative = true;
    }
    
    /// Transition to enemy turn.
    pub fn enemy_turn(&mut self) {
        self.phase = CombatPhase::EnemyTurn;
        self.player_has_initiative = false;
    }
    
    /// Begin ending phase.
    pub fn begin_ending(&mut self) {
        self.phase = CombatPhase::Ending;
    }
    
    /// Finish combat.
    pub fn finish(&mut self) {
        self.phase = CombatPhase::Finished;
        self.encounter.end();
    }
    
    /// Check if combat is over.
    pub fn is_over(&self) -> bool {
        self.phase == CombatPhase::Finished || self.encounter.all_defeated()
    }
    
    /// Apply harm to current target.
    pub fn harm_target(&mut self, hits: u8) {
        let mut target_defeated = false;
        let mut target_name = String::new();
        
        if let Some(target) = self.current_target_mut() {
            target.mark_progress(hits);
            
            if target.defeated {
                target_defeated = true;
                target_name = target.name.clone();
            }
        }
        
        // Handle defeat after releasing borrow
        if target_defeated {
            self.log_event(format!("{} is defeated!", target_name));
            if !self.encounter.all_defeated() {
                self.target_index = 0;
            }
        }
    }
    
    /// Get status summary string.
    pub fn status(&self) -> String {
        let phase_str = match self.phase {
            CombatPhase::Entering => "Entering Combat",
            CombatPhase::PlayerTurn => "Your Turn",
            CombatPhase::EnemyTurn => "Enemy Turn",
            CombatPhase::Resolving => "Resolving...",
            CombatPhase::Ending => "Ending Combat",
            CombatPhase::Finished => "Combat Finished",
        };
        
        format!("{} | Round {} | {} enemies remaining",
            phase_str,
            self.encounter.round,
            self.encounter.active_enemies().len())
    }
}
