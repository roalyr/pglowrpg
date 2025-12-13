// [Revision: v0.1] [Path: libs/lib_oracle/src/tables.rs] [Date: 2025-12-13]
// Description: Oracle table data structures and loading

use serde::{Deserialize, Serialize};

/// Result of a Yes/No oracle roll
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YesNoResult {
    NoCritical,    // 1-10: No, and...
    No,            // 11-25: No
    NoButMild,     // 26-40: No, but...
    YesButMild,    // 41-55: Yes, but...
    Yes,           // 56-75: Yes
    YesCritical,   // 76-100: Yes, and...
}

impl YesNoResult {
    pub fn from_roll(roll: u8) -> Self {
        match roll {
            1..=10 => YesNoResult::NoCritical,
            11..=25 => YesNoResult::No,
            26..=40 => YesNoResult::NoButMild,
            41..=55 => YesNoResult::YesButMild,
            56..=75 => YesNoResult::Yes,
            76..=100 => YesNoResult::YesCritical,
            _ => YesNoResult::Yes, // fallback
        }
    }
    
    pub fn is_yes(&self) -> bool {
        matches!(self, YesNoResult::YesButMild | YesNoResult::Yes | YesNoResult::YesCritical)
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            YesNoResult::NoCritical => "No, and something worse happens...",
            YesNoResult::No => "No.",
            YesNoResult::NoButMild => "No, but there's a silver lining...",
            YesNoResult::YesButMild => "Yes, but with a complication...",
            YesNoResult::Yes => "Yes.",
            YesNoResult::YesCritical => "Yes, and something fortunate occurs!",
        }
    }
}

/// A single entry in an oracle table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleEntry {
    pub min: u8,
    pub max: u8,
    pub result: String,
}

impl OracleEntry {
    pub fn new(min: u8, max: u8, result: impl Into<String>) -> Self {
        Self {
            min,
            max,
            result: result.into(),
        }
    }
    
    pub fn matches(&self, roll: u8) -> bool {
        roll >= self.min && roll <= self.max
    }
}

/// An oracle table with weighted entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleTable {
    pub name: String,
    pub entries: Vec<OracleEntry>,
}

impl OracleTable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, min: u8, max: u8, result: impl Into<String>) {
        self.entries.push(OracleEntry::new(min, max, result));
    }
    
    pub fn lookup(&self, roll: u8) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.matches(roll))
            .map(|e| e.result.as_str())
    }
}

// ============================================================================
// DEFAULT ORACLE TABLES (Ironsworn-inspired)
// ============================================================================

/// Action oracle - What are they doing?
pub fn action_table() -> OracleTable {
    let mut table = OracleTable::new("Action");
    table.add_entry(1, 2, "Scheme");
    table.add_entry(3, 4, "Clash");
    table.add_entry(5, 6, "Weaken");
    table.add_entry(7, 8, "Initiate");
    table.add_entry(9, 10, "Create");
    table.add_entry(11, 12, "Swear");
    table.add_entry(13, 14, "Avenge");
    table.add_entry(15, 16, "Guard");
    table.add_entry(17, 18, "Defend");
    table.add_entry(19, 20, "Threaten");
    table.add_entry(21, 22, "Attack");
    table.add_entry(23, 24, "Capture");
    table.add_entry(25, 26, "Bargain");
    table.add_entry(27, 28, "Locate");
    table.add_entry(29, 30, "Protect");
    table.add_entry(31, 32, "Aid");
    table.add_entry(33, 34, "Oppose");
    table.add_entry(35, 36, "Seize");
    table.add_entry(37, 38, "Overwhelm");
    table.add_entry(39, 40, "Challenge");
    table.add_entry(41, 42, "Acquire");
    table.add_entry(43, 44, "Support");
    table.add_entry(45, 46, "Transform");
    table.add_entry(47, 48, "Persecute");
    table.add_entry(49, 50, "Destroy");
    table.add_entry(51, 52, "Distract");
    table.add_entry(53, 54, "Remove");
    table.add_entry(55, 56, "Eliminate");
    table.add_entry(57, 58, "Withdraw");
    table.add_entry(59, 60, "Abandon");
    table.add_entry(61, 62, "Investigate");
    table.add_entry(63, 64, "Reveal");
    table.add_entry(65, 66, "Reject");
    table.add_entry(67, 68, "Gather");
    table.add_entry(69, 70, "Summon");
    table.add_entry(71, 72, "Arrive");
    table.add_entry(73, 74, "Repay");
    table.add_entry(75, 76, "Communicate");
    table.add_entry(77, 78, "Refuse");
    table.add_entry(79, 80, "Explore");
    table.add_entry(81, 82, "Depart");
    table.add_entry(83, 84, "Escape");
    table.add_entry(85, 86, "Deceive");
    table.add_entry(87, 88, "Hide");
    table.add_entry(89, 90, "Hunt");
    table.add_entry(91, 92, "Restore");
    table.add_entry(93, 94, "Advance");
    table.add_entry(95, 96, "Change");
    table.add_entry(97, 98, "Search");
    table.add_entry(99, 100, "Command");
    table
}

/// Theme oracle - What is it about?
pub fn theme_table() -> OracleTable {
    let mut table = OracleTable::new("Theme");
    table.add_entry(1, 2, "Risk");
    table.add_entry(3, 4, "Destiny");
    table.add_entry(5, 6, "Honor");
    table.add_entry(7, 8, "Fear");
    table.add_entry(9, 10, "Corruption");
    table.add_entry(11, 12, "Time");
    table.add_entry(13, 14, "Battle");
    table.add_entry(15, 16, "Treasure");
    table.add_entry(17, 18, "Debt");
    table.add_entry(19, 20, "Trade");
    table.add_entry(21, 22, "Blood");
    table.add_entry(23, 24, "Bond");
    table.add_entry(25, 26, "Burden");
    table.add_entry(27, 28, "Home");
    table.add_entry(29, 30, "Creation");
    table.add_entry(31, 32, "Creature");
    table.add_entry(33, 34, "Disease");
    table.add_entry(35, 36, "Death");
    table.add_entry(37, 38, "Desolation");
    table.add_entry(39, 40, "Decay");
    table.add_entry(41, 42, "Dream");
    table.add_entry(43, 44, "Duty");
    table.add_entry(45, 46, "Nature");
    table.add_entry(47, 48, "Weapon");
    table.add_entry(49, 50, "Survival");
    table.add_entry(51, 52, "Hope");
    table.add_entry(53, 54, "Hate");
    table.add_entry(55, 56, "Health");
    table.add_entry(57, 58, "Innocence");
    table.add_entry(59, 60, "Knowledge");
    table.add_entry(61, 62, "Legacy");
    table.add_entry(63, 64, "Life");
    table.add_entry(65, 66, "Loss");
    table.add_entry(67, 68, "Love");
    table.add_entry(69, 70, "Mysticism");
    table.add_entry(71, 72, "Portent");
    table.add_entry(73, 74, "Power");
    table.add_entry(75, 76, "Price");
    table.add_entry(77, 78, "Secret");
    table.add_entry(79, 80, "Promise");
    table.add_entry(81, 82, "Safety");
    table.add_entry(83, 84, "Stranger");
    table.add_entry(85, 86, "Spirit");
    table.add_entry(87, 88, "Revenge");
    table.add_entry(89, 90, "Ruin");
    table.add_entry(91, 92, "Trust");
    table.add_entry(93, 94, "Truth");
    table.add_entry(95, 96, "Warning");
    table.add_entry(97, 98, "Wealth");
    table.add_entry(99, 100, "War");
    table
}

/// Complication oracle - What goes wrong?
pub fn complication_table() -> OracleTable {
    let mut table = OracleTable::new("Complication");
    table.add_entry(1, 5, "A trusted ally is revealed to have ulterior motives");
    table.add_entry(6, 10, "An enemy gains unexpected reinforcements");
    table.add_entry(11, 15, "The path forward is blocked or destroyed");
    table.add_entry(16, 20, "A vital resource is lost or corrupted");
    table.add_entry(21, 25, "Someone innocent is put in danger");
    table.add_entry(26, 30, "A secret is exposed at the worst time");
    table.add_entry(31, 35, "The environment turns hostile");
    table.add_entry(36, 40, "A deadline is moved up unexpectedly");
    table.add_entry(41, 45, "An old rival appears seeking revenge");
    table.add_entry(46, 50, "A helpful contact demands a dangerous favor");
    table.add_entry(51, 55, "Equipment fails at a critical moment");
    table.add_entry(56, 60, "The target has already moved on");
    table.add_entry(61, 65, "Witnesses complicate the situation");
    table.add_entry(66, 70, "A curse or illness takes hold");
    table.add_entry(71, 75, "The reward is not what it seemed");
    table.add_entry(76, 80, "An ally falls into enemy hands");
    table.add_entry(81, 85, "The truth is darker than expected");
    table.add_entry(86, 90, "A bargain comes due");
    table.add_entry(91, 95, "Supernatural forces take notice");
    table.add_entry(96, 100, "Everything changes. Roll twice more.");
    table
}

/// Opportunity oracle - What advantage appears?
pub fn opportunity_table() -> OracleTable {
    let mut table = OracleTable::new("Opportunity");
    table.add_entry(1, 10, "An ally arrives unexpectedly");
    table.add_entry(11, 20, "A weakness in your enemy is revealed");
    table.add_entry(21, 30, "You find a shortcut or hidden passage");
    table.add_entry(31, 40, "Useful supplies or equipment are discovered");
    table.add_entry(41, 50, "An enemy makes a costly mistake");
    table.add_entry(51, 60, "Someone offers unexpected aid");
    table.add_entry(61, 70, "The environment provides an advantage");
    table.add_entry(71, 80, "Critical information falls into your hands");
    table.add_entry(81, 90, "A dangerous situation resolves peacefully");
    table.add_entry(91, 100, "Fortune smiles. Take +2 momentum.");
    table
}

/// NPC Disposition oracle
pub fn npc_disposition_table() -> OracleTable {
    let mut table = OracleTable::new("NPC Disposition");
    table.add_entry(1, 10, "Hostile - They mean you harm");
    table.add_entry(11, 25, "Suspicious - They distrust you");
    table.add_entry(26, 40, "Wary - They are cautious around you");
    table.add_entry(41, 60, "Neutral - They have no strong feelings");
    table.add_entry(61, 75, "Curious - They want to know more");
    table.add_entry(76, 90, "Friendly - They are well-disposed toward you");
    table.add_entry(91, 100, "Helpful - They actively want to aid you");
    table
}

/// NPC Goal oracle
pub fn npc_goal_table() -> OracleTable {
    let mut table = OracleTable::new("NPC Goal");
    table.add_entry(1, 5, "Obtain an object");
    table.add_entry(6, 10, "Deliver a message");
    table.add_entry(11, 15, "Find someone");
    table.add_entry(16, 20, "Escape danger");
    table.add_entry(21, 25, "Protect something precious");
    table.add_entry(26, 30, "Avenge a wrong");
    table.add_entry(31, 35, "Complete a journey");
    table.add_entry(36, 40, "Fulfill an oath");
    table.add_entry(41, 45, "Gain power or status");
    table.add_entry(46, 50, "Learn a truth");
    table.add_entry(51, 55, "Return home");
    table.add_entry(56, 60, "Survive the day");
    table.add_entry(61, 65, "Pay a debt");
    table.add_entry(66, 70, "Cure an affliction");
    table.add_entry(71, 75, "Find peace");
    table.add_entry(76, 80, "Preserve the old ways");
    table.add_entry(81, 85, "Overthrow tyranny");
    table.add_entry(86, 90, "Create something lasting");
    table.add_entry(91, 95, "Make amends");
    table.add_entry(96, 100, "Unknown or inscrutable");
    table
}

/// Location Feature oracle
pub fn location_feature_table() -> OracleTable {
    let mut table = OracleTable::new("Location Feature");
    table.add_entry(1, 5, "A well-traveled crossroads");
    table.add_entry(6, 10, "An ancient standing stone");
    table.add_entry(11, 15, "A freshwater spring");
    table.add_entry(16, 20, "The ruins of an old structure");
    table.add_entry(21, 25, "A dense thicket");
    table.add_entry(26, 30, "A steep cliff or ravine");
    table.add_entry(31, 35, "A cave entrance");
    table.add_entry(36, 40, "A shallow stream crossing");
    table.add_entry(41, 45, "Signs of recent travel");
    table.add_entry(46, 50, "Abandoned camp or campfire");
    table.add_entry(51, 55, "Animal tracks or markings");
    table.add_entry(56, 60, "A natural shelter");
    table.add_entry(61, 65, "Markers or waypoints");
    table.add_entry(66, 70, "Dense fog or mist");
    table.add_entry(71, 75, "Unusual plants or fungi");
    table.add_entry(76, 80, "A scenic overlook");
    table.add_entry(81, 85, "Signs of conflict");
    table.add_entry(86, 90, "A boundary marker");
    table.add_entry(91, 95, "Strange sounds or echoes");
    table.add_entry(96, 100, "An ominous presence");
    table
}

/// Pay the Price oracle - What happens when you fail?
pub fn pay_price_table() -> OracleTable {
    let mut table = OracleTable::new("Pay the Price");
    table.add_entry(1, 2, "A trusted person or community is harmed");
    table.add_entry(3, 4, "A person or community opposes you");
    table.add_entry(5, 6, "You are separated from something important");
    table.add_entry(7, 8, "You are betrayed by someone you trusted");
    table.add_entry(9, 10, "A new danger appears");
    table.add_entry(11, 14, "You face a difficult choice");
    table.add_entry(15, 18, "You are delayed or denied");
    table.add_entry(19, 22, "You lose something of value");
    table.add_entry(23, 26, "It causes collateral damage");
    table.add_entry(27, 30, "You are put in a bad position");
    table.add_entry(31, 34, "An ally is in danger");
    table.add_entry(35, 38, "You waste resources");
    table.add_entry(39, 42, "You are forced to defend yourself");
    table.add_entry(43, 46, "Your actions are made public");
    table.add_entry(47, 50, "You owe a favor or debt");
    table.add_entry(51, 54, "You uncover an unwelcome truth");
    table.add_entry(55, 58, "A weakness is exposed");
    table.add_entry(59, 62, "Something precious is damaged");
    table.add_entry(63, 66, "You are drawn into conflict");
    table.add_entry(67, 70, "An enemy gains an advantage");
    table.add_entry(71, 74, "You attract unwanted attention");
    table.add_entry(75, 78, "The situation grows more dangerous");
    table.add_entry(79, 82, "You suffer harm (-1 Health)");
    table.add_entry(83, 86, "You suffer stress (-1 Spirit)");
    table.add_entry(87, 90, "Your supplies are depleted (-1 Supply)");
    table.add_entry(91, 94, "You lose momentum (-2 Momentum)");
    table.add_entry(95, 100, "Roll twice and combine");
    table
}
