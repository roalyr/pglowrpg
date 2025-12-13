// [Revision: v0.1] [Path: libs/lib_character/src/vows.rs] [Date: 2025-12-13]
// Description: Vows, bonds, and progress tracking

use serde::{Deserialize, Serialize};

/// Vow difficulty rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VowRank {
    Troublesome,
    Dangerous,
    Formidable,
    Extreme,
    Epic,
}

impl VowRank {
    /// Get the number of progress marks per successful roll.
    pub fn progress_per_mark(&self) -> u8 {
        match self {
            VowRank::Troublesome => 12,
            VowRank::Dangerous => 8,
            VowRank::Formidable => 4,
            VowRank::Extreme => 2,
            VowRank::Epic => 1,
        }
    }
}

/// Progress track for a vow or journey.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressTrack {
    /// Total progress (0-40 in standard Ironsworn).
    progress: u8,
    /// Maximum progress value (usually 40).
    max: u8,
}

impl ProgressTrack {
    pub fn new() -> Self {
        Self {
            progress: 0,
            max: 40,
        }
    }

    pub fn progress(&self) -> u8 {
        self.progress
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    /// Add progress marks.
    pub fn mark_progress(&mut self, amount: u8) {
        self.progress = (self.progress + amount).min(self.max);
    }

    /// Check if track is complete.
    pub fn is_complete(&self) -> bool {
        self.progress >= self.max
    }

    /// Get progress as a value from 0-10 (for progress rolls).
    pub fn progress_score(&self) -> u8 {
        (self.progress / 4).min(10)
    }

    /// Reset progress to 0.
    pub fn reset(&mut self) {
        self.progress = 0;
    }
}

impl Default for ProgressTrack {
    fn default() -> Self {
        Self::new()
    }
}

/// A vow or quest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vow {
    pub description: String,
    pub rank: VowRank,
    pub progress: ProgressTrack,
    pub completed: bool,
}

impl Vow {
    pub fn new(description: String, rank: VowRank) -> Self {
        Self {
            description,
            rank,
            progress: ProgressTrack::new(),
            completed: false,
        }
    }

    /// Mark progress based on vow rank.
    pub fn mark_progress(&mut self) {
        let marks = self.rank.progress_per_mark();
        self.progress.mark_progress(marks);
    }

    /// Complete the vow.
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Forsake the vow.
    pub fn forsake(&mut self) {
        self.completed = false;
        self.progress.reset();
    }
}

/// A bond with an NPC, faction, or location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bond {
    pub name: String,
    pub description: String,
    pub progress: ProgressTrack,
}

impl Bond {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            progress: ProgressTrack::new(),
        }
    }

    /// Mark progress on the bond.
    pub fn mark_progress(&mut self, amount: u8) {
        self.progress.mark_progress(amount);
    }
}
