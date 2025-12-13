// [Revision: v0.1] [Path: libs/lib_oracle/src/oracle.rs] [Date: 2025-12-13]
// Description: Oracle interface for asking questions and rolling on tables

use crate::tables::*;
use lib_pseudo_rng::Rng;

/// The Oracle provides narrative prompts and answers through random tables.
pub struct Oracle {
    action_table: OracleTable,
    theme_table: OracleTable,
    complication_table: OracleTable,
    opportunity_table: OracleTable,
    npc_disposition_table: OracleTable,
    npc_goal_table: OracleTable,
    location_feature_table: OracleTable,
    pay_price_table: OracleTable,
}

impl Oracle {
    /// Create a new Oracle with default tables.
    pub fn new() -> Self {
        Self {
            action_table: action_table(),
            theme_table: theme_table(),
            complication_table: complication_table(),
            opportunity_table: opportunity_table(),
            npc_disposition_table: npc_disposition_table(),
            npc_goal_table: npc_goal_table(),
            location_feature_table: location_feature_table(),
            pay_price_table: pay_price_table(),
        }
    }
    
    /// Ask a Yes/No question with variable likelihood.
    /// likelihood: 50 = equal odds, lower = less likely yes, higher = more likely yes
    pub fn ask_yes_no(&self, rng: &mut Rng, likelihood: u8) -> YesNoResult {
        let roll = rng.d100();
        // Adjust roll based on likelihood (50 = neutral)
        let adjusted = if likelihood >= 50 {
            // More likely yes: reduce roll needed
            let bonus = (likelihood - 50) as i16;
            (roll as i16 - bonus).max(1) as u8
        } else {
            // Less likely yes: increase roll needed
            let penalty = (50 - likelihood) as i16;
            (roll as i16 + penalty).min(100) as u8
        };
        YesNoResult::from_roll(adjusted)
    }
    
    /// Ask the Oracle a simple 50/50 Yes/No question.
    pub fn ask(&self, rng: &mut Rng) -> YesNoResult {
        self.ask_yes_no(rng, 50)
    }
    
    /// Ask if something likely happens (75% base yes).
    pub fn ask_likely(&self, rng: &mut Rng) -> YesNoResult {
        self.ask_yes_no(rng, 75)
    }
    
    /// Ask if something unlikely happens (25% base yes).
    pub fn ask_unlikely(&self, rng: &mut Rng) -> YesNoResult {
        self.ask_yes_no(rng, 25)
    }
    
    /// Roll on the Action table.
    pub fn action(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.action_table.lookup(roll).unwrap_or("Take action")
    }
    
    /// Roll on the Theme table.
    pub fn theme(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.theme_table.lookup(roll).unwrap_or("Fate")
    }
    
    /// Generate an Action + Theme prompt.
    pub fn action_theme(&self, rng: &mut Rng) -> (String, String) {
        (self.action(rng).to_string(), self.theme(rng).to_string())
    }
    
    /// Roll on the Complication table.
    pub fn complication(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.complication_table.lookup(roll).unwrap_or("An unexpected obstacle appears")
    }
    
    /// Roll on the Opportunity table.
    pub fn opportunity(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.opportunity_table.lookup(roll).unwrap_or("Fortune favors you")
    }
    
    /// Roll on the NPC Disposition table.
    pub fn npc_disposition(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.npc_disposition_table.lookup(roll).unwrap_or("Neutral")
    }
    
    /// Roll on the NPC Goal table.
    pub fn npc_goal(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.npc_goal_table.lookup(roll).unwrap_or("Unknown motivation")
    }
    
    /// Roll on the Location Feature table.
    pub fn location_feature(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.location_feature_table.lookup(roll).unwrap_or("Nothing of note")
    }
    
    /// Roll on the Pay the Price table.
    pub fn pay_price(&self, rng: &mut Rng) -> &str {
        let roll = rng.d100();
        self.pay_price_table.lookup(roll).unwrap_or("You face a setback")
    }
    
    /// Generate a random encounter prompt.
    pub fn encounter_prompt(&self, rng: &mut Rng) -> String {
        let (action, theme) = self.action_theme(rng);
        format!("{} + {}", action, theme)
    }
    
    /// Generate NPC details.
    pub fn npc_details(&self, rng: &mut Rng) -> (String, String) {
        (
            self.npc_disposition(rng).to_string(),
            self.npc_goal(rng).to_string(),
        )
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yes_no_oracle() {
        let oracle = Oracle::new();
        let mut rng = Rng::new(12345);
        
        // Just verify it doesn't panic
        for _ in 0..100 {
            let _ = oracle.ask(&mut rng);
        }
    }
    
    #[test]
    fn test_table_lookups() {
        let oracle = Oracle::new();
        let mut rng = Rng::new(12345);
        
        // Verify all tables return something
        assert!(!oracle.action(&mut rng).is_empty());
        assert!(!oracle.theme(&mut rng).is_empty());
        assert!(!oracle.complication(&mut rng).is_empty());
        assert!(!oracle.opportunity(&mut rng).is_empty());
    }
}
