// [Revision: v0.1] [Path: apps/game/src/journal.rs] [Date: 2025-12-13]
// Description: Player journal for tracking narrative events and notes

use serde::{Deserialize, Serialize};

/// Category of journal entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalCategory {
    /// Story events and narrative beats.
    Story,
    /// Combat encounters and battles.
    Combat,
    /// Travel and exploration.
    Travel,
    /// Character interactions and relationships.
    Social,
    /// Vows and quests.
    Quest,
    /// Player notes and thoughts.
    Note,
    /// System messages and game events.
    System,
}

impl JournalCategory {
    /// Display name for the category.
    pub fn name(&self) -> &'static str {
        match self {
            JournalCategory::Story => "Story",
            JournalCategory::Combat => "Combat",
            JournalCategory::Travel => "Travel",
            JournalCategory::Social => "Social",
            JournalCategory::Quest => "Quest",
            JournalCategory::Note => "Note",
            JournalCategory::System => "System",
        }
    }
    
    /// Icon/symbol for display.
    pub fn icon(&self) -> &'static str {
        match self {
            JournalCategory::Story => "📜",
            JournalCategory::Combat => "⚔️",
            JournalCategory::Travel => "🗺️",
            JournalCategory::Social => "👥",
            JournalCategory::Quest => "⭐",
            JournalCategory::Note => "📝",
            JournalCategory::System => "⚙️",
        }
    }
}

/// A single journal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Turn when this was recorded.
    pub turn: u32,
    /// Category of entry.
    pub category: JournalCategory,
    /// Title/headline.
    pub title: String,
    /// Full description.
    pub description: String,
    /// Location where it occurred.
    pub location: Option<String>,
    /// Related characters/NPCs.
    pub participants: Vec<String>,
    /// Whether this is a significant event.
    pub significant: bool,
}

impl JournalEntry {
    /// Create a new journal entry.
    pub fn new(turn: u32, category: JournalCategory, title: impl Into<String>) -> Self {
        Self {
            turn,
            category,
            title: title.into(),
            description: String::new(),
            location: None,
            participants: Vec::new(),
            significant: false,
        }
    }
    
    /// Add description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    /// Set location.
    pub fn at_location(mut self, loc: impl Into<String>) -> Self {
        self.location = Some(loc.into());
        self
    }
    
    /// Add a participant.
    pub fn with_participant(mut self, name: impl Into<String>) -> Self {
        self.participants.push(name.into());
        self
    }
    
    /// Mark as significant.
    pub fn mark_significant(mut self) -> Self {
        self.significant = true;
        self
    }
    
    /// Format for display.
    pub fn display(&self) -> String {
        let loc = self.location.as_deref().unwrap_or("Unknown");
        format!(
            "[Turn {}] {} {} at {}\n  {}",
            self.turn,
            self.category.icon(),
            self.title,
            loc,
            self.description
        )
    }
    
    /// Short format for list display.
    pub fn display_short(&self) -> String {
        format!(
            "{} [{}] {}",
            self.category.icon(),
            self.turn,
            self.title
        )
    }
}

impl std::fmt::Display for JournalEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// The player's journal.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Journal {
    entries: Vec<JournalEntry>,
    /// Maximum entries to keep (0 = unlimited).
    max_entries: usize,
}

impl Journal {
    /// Create a new empty journal.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 0,
        }
    }
    
    /// Create a journal with entry limit.
    pub fn with_limit(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    
    /// Add an entry to the journal.
    pub fn add(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
        
        // Trim old non-significant entries if over limit
        if self.max_entries > 0 && self.entries.len() > self.max_entries {
            // Keep significant entries and most recent
            let mut to_remove = Vec::new();
            let excess = self.entries.len() - self.max_entries;
            let mut removed = 0;
            
            for (i, entry) in self.entries.iter().enumerate() {
                if !entry.significant && removed < excess {
                    to_remove.push(i);
                    removed += 1;
                }
            }
            
            // Remove in reverse order to maintain indices
            for i in to_remove.into_iter().rev() {
                self.entries.remove(i);
            }
        }
    }
    
    /// Add a story entry.
    pub fn log_story(&mut self, turn: u32, title: &str, description: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::Story, title)
                .with_description(description)
        );
    }
    
    /// Add a combat entry.
    pub fn log_combat(&mut self, turn: u32, title: &str, description: &str, enemy: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::Combat, title)
                .with_description(description)
                .with_participant(enemy)
        );
    }
    
    /// Add a travel entry.
    pub fn log_travel(&mut self, turn: u32, from: &str, to: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::Travel, format!("Traveled to {}", to))
                .with_description(format!("Journeyed from {} to {}", from, to))
                .at_location(to)
        );
    }
    
    /// Add a quest entry.
    pub fn log_quest(&mut self, turn: u32, title: &str, description: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::Quest, title)
                .with_description(description)
                .mark_significant()
        );
    }
    
    /// Add a player note.
    pub fn add_note(&mut self, turn: u32, note: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::Note, "Note")
                .with_description(note)
        );
    }
    
    /// Add a system message.
    pub fn log_system(&mut self, turn: u32, message: &str) {
        self.add(
            JournalEntry::new(turn, JournalCategory::System, message)
        );
    }
    
    /// Get all entries.
    pub fn all_entries(&self) -> &[JournalEntry] {
        &self.entries
    }
    
    /// Get entries by category.
    pub fn entries_by_category(&self, category: JournalCategory) -> Vec<&JournalEntry> {
        self.entries.iter()
            .filter(|e| e.category == category)
            .collect()
    }
    
    /// Get significant entries.
    pub fn significant_entries(&self) -> Vec<&JournalEntry> {
        self.entries.iter()
            .filter(|e| e.significant)
            .collect()
    }
    
    /// Get recent entries (last N).
    pub fn recent(&self, count: usize) -> Vec<&JournalEntry> {
        self.entries.iter()
            .rev()
            .take(count)
            .collect()
    }
    
    /// Get entries from a specific turn range.
    pub fn entries_from_turns(&self, start: u32, end: u32) -> Vec<&JournalEntry> {
        self.entries.iter()
            .filter(|e| e.turn >= start && e.turn <= end)
            .collect()
    }
    
    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if journal is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    /// Search entries by keyword.
    pub fn search(&self, keyword: &str) -> Vec<&JournalEntry> {
        let keyword_lower = keyword.to_lowercase();
        self.entries.iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&keyword_lower) ||
                e.description.to_lowercase().contains(&keyword_lower)
            })
            .collect()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_category_name() {
        assert_eq!(JournalCategory::Story.name(), "Story");
        assert_eq!(JournalCategory::Combat.name(), "Combat");
    }

    #[test]
    fn test_journal_entry_creation() {
        let entry = JournalEntry::new(5, JournalCategory::Story, "A dark discovery");
        assert_eq!(entry.turn, 5);
        assert_eq!(entry.title, "A dark discovery");
        assert!(!entry.significant);
    }

    #[test]
    fn test_journal_entry_builder() {
        let entry = JournalEntry::new(10, JournalCategory::Combat, "Battle at the bridge")
            .with_description("Fierce combat ensued")
            .at_location("Stone Bridge")
            .with_participant("Wolf Pack")
            .mark_significant();

        assert!(entry.significant);
        assert_eq!(entry.location.as_deref(), Some("Stone Bridge"));
        assert!(entry.participants.contains(&"Wolf Pack".to_string()));
    }

    #[test]
    fn test_journal_add_and_retrieve() {
        let mut journal = Journal::new();
        journal.log_story(1, "Beginning", "The journey starts");
        journal.log_combat(2, "First blood", "Victory!", "Goblin");
        
        assert_eq!(journal.len(), 2);
        assert!(!journal.is_empty());
    }

    #[test]
    fn test_journal_filter_by_category() {
        let mut journal = Journal::new();
        journal.log_story(1, "Story 1", "...");
        journal.log_combat(2, "Combat 1", "...", "enemy");
        journal.log_story(3, "Story 2", "...");
        
        let stories = journal.entries_by_category(JournalCategory::Story);
        assert_eq!(stories.len(), 2);
        
        let combats = journal.entries_by_category(JournalCategory::Combat);
        assert_eq!(combats.len(), 1);
    }

    #[test]
    fn test_journal_significant_entries() {
        let mut journal = Journal::new();
        journal.log_story(1, "Minor event", "...");
        journal.log_quest(2, "Major quest", "Important!"); // Quests are significant
        
        let significant = journal.significant_entries();
        assert_eq!(significant.len(), 1);
        assert_eq!(significant[0].title, "Major quest");
    }

    #[test]
    fn test_journal_recent() {
        let mut journal = Journal::new();
        journal.log_story(1, "First", "...");
        journal.log_story(2, "Second", "...");
        journal.log_story(3, "Third", "...");
        
        let recent = journal.recent(2);
        assert_eq!(recent.len(), 2);
        // Recent returns in reverse order (newest first)
        assert_eq!(recent[0].title, "Third");
        assert_eq!(recent[1].title, "Second");
    }

    #[test]
    fn test_journal_search() {
        let mut journal = Journal::new();
        journal.log_story(1, "The dark forest", "Trees everywhere");
        journal.log_story(2, "The bright meadow", "Sunshine and flowers");
        
        let results = journal.search("dark");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "The dark forest");
    }

    #[test]
    fn test_journal_with_limit() {
        let mut journal = Journal::with_limit(3);
        journal.log_story(1, "Entry 1", "...");
        journal.log_story(2, "Entry 2", "...");
        journal.log_story(3, "Entry 3", "...");
        journal.log_story(4, "Entry 4", "...");
        
        // Should trim to 3 entries
        assert_eq!(journal.len(), 3);
    }

    #[test]
    fn test_journal_limit_keeps_significant() {
        let mut journal = Journal::with_limit(2);
        journal.log_quest(1, "Important!", "Keep me"); // Significant
        journal.log_story(2, "Regular 1", "...");
        journal.log_story(3, "Regular 2", "...");
        journal.log_story(4, "Regular 3", "...");
        
        // Should keep the significant entry
        let significant = journal.significant_entries();
        assert!(significant.iter().any(|e| e.title == "Important!"));
    }

    #[test]
    fn test_entry_display() {
        let entry = JournalEntry::new(5, JournalCategory::Combat, "Battle")
            .with_description("A fierce fight")
            .at_location("Castle Gate");
        
        let display = entry.display();
        assert!(display.contains("Turn 5"));
        assert!(display.contains("Battle"));
        assert!(display.contains("Castle Gate"));
    }
}
