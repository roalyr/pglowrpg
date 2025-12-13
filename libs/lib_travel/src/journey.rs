// [Revision: v0.1] [Path: libs/lib_travel/src/journey.rs] [Date: 2025-12-13]
// Description: Journey system with progress tracking

use lib_character::vows::ProgressTrack;
use lib_location::location_type::LocationRank;
use serde::{Deserialize, Serialize};

/// Status of a journey.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JourneyStatus {
    /// Journey is ongoing.
    Active,
    /// Journey completed successfully.
    Completed,
    /// Journey abandoned or failed.
    Failed,
}

/// A journey between locations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journey {
    pub origin: String,
    pub destination: String,
    pub rank: LocationRank,
    pub progress: ProgressTrack,
    pub status: JourneyStatus,
    /// Days elapsed on journey.
    pub days_traveled: u32,
}

impl Journey {
    /// Start a new journey.
    pub fn new(origin: String, destination: String, rank: LocationRank) -> Self {
        Self {
            origin,
            destination,
            rank,
            progress: ProgressTrack::new(),
            status: JourneyStatus::Active,
            days_traveled: 0,
        }
    }

    /// Mark progress on the journey.
    pub fn mark_progress(&mut self) {
        let marks = match self.rank {
            LocationRank::Troublesome => 12,
            LocationRank::Dangerous => 8,
            LocationRank::Formidable => 4,
            LocationRank::Extreme => 2,
            LocationRank::Epic => 1,
        };
        self.progress.mark_progress(marks);
    }

    /// Advance a day of travel.
    pub fn advance_day(&mut self) {
        self.days_traveled += 1;
    }

    /// Complete the journey.
    pub fn complete(&mut self) {
        self.status = JourneyStatus::Completed;
    }

    /// Fail the journey.
    pub fn fail(&mut self) {
        self.status = JourneyStatus::Failed;
    }

    /// Check if journey is complete.
    pub fn is_complete(&self) -> bool {
        self.progress.is_complete()
    }

    /// Get progress score for progress roll.
    pub fn progress_score(&self) -> u8 {
        self.progress.progress_score()
    }
}

/// Calculate journey rank based on distance and terrain difficulty.
pub fn calculate_journey_rank(distance: f32, terrain_difficulty: i8) -> LocationRank {
    let adjusted_distance = distance - (terrain_difficulty as f32 * 10.0);
    
    if adjusted_distance < 50.0 {
        LocationRank::Troublesome
    } else if adjusted_distance < 100.0 {
        LocationRank::Dangerous
    } else if adjusted_distance < 200.0 {
        LocationRank::Formidable
    } else if adjusted_distance < 400.0 {
        LocationRank::Extreme
    } else {
        LocationRank::Epic
    }
}

/// Calculate estimated journey time in days.
pub fn estimate_journey_days(distance: f32, terrain_difficulty: i8) -> u32 {
    let base_days = (distance / 20.0).ceil() as u32; // ~20 units per day
    let difficulty_penalty = terrain_difficulty.abs() as u32;
    base_days + difficulty_penalty
}
