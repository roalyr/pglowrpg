// [Revision: v0.1] [Path: libs/lib_soul/src/fate_thread.rs] [Date: 2025-12-13]
// Description: Fate threads connecting souls across lifetimes

use serde::{Deserialize, Serialize};

/// Strength of a fate thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreadStrength {
    /// Faint connection, may encounter.
    Faint,
    /// Moderate connection, will likely encounter.
    Moderate,
    /// Strong connection, will definitely encounter.
    Strong,
    /// Bound together, always drawn to each other.
    Bound,
}

impl ThreadStrength {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            ThreadStrength::Faint => "Faint",
            ThreadStrength::Moderate => "Moderate",
            ThreadStrength::Strong => "Strong",
            ThreadStrength::Bound => "Bound",
        }
    }
    
    /// Chance of encountering this soul (0-100).
    pub fn encounter_chance(&self) -> u32 {
        match self {
            ThreadStrength::Faint => 10,
            ThreadStrength::Moderate => 30,
            ThreadStrength::Strong => 60,
            ThreadStrength::Bound => 95,
        }
    }
    
    /// Strengthen to next level.
    pub fn strengthen(&self) -> ThreadStrength {
        match self {
            ThreadStrength::Faint => ThreadStrength::Moderate,
            ThreadStrength::Moderate => ThreadStrength::Strong,
            ThreadStrength::Strong => ThreadStrength::Bound,
            ThreadStrength::Bound => ThreadStrength::Bound,
        }
    }
    
    /// Weaken by one level.
    pub fn weaken(&self) -> Option<ThreadStrength> {
        match self {
            ThreadStrength::Bound => Some(ThreadStrength::Strong),
            ThreadStrength::Strong => Some(ThreadStrength::Moderate),
            ThreadStrength::Moderate => Some(ThreadStrength::Faint),
            ThreadStrength::Faint => None,
        }
    }
}

/// Nature of the fate thread relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadNature {
    /// Positive connection (friendship, love, mentorship).
    Positive,
    /// Negative connection (rivalry, enmity, betrayal).
    Negative,
    /// Complex/ambiguous connection.
    Complex,
    /// Familial connection.
    Familial,
}

impl ThreadNature {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            ThreadNature::Positive => "Positive",
            ThreadNature::Negative => "Negative",
            ThreadNature::Complex => "Complex",
            ThreadNature::Familial => "Familial",
        }
    }
    
    /// Description of how souls interact.
    pub fn description(&self) -> &'static str {
        match self {
            ThreadNature::Positive => "drawn together in friendship",
            ThreadNature::Negative => "fated to conflict",
            ThreadNature::Complex => "intertwined in complicated ways",
            ThreadNature::Familial => "bound as family across lives",
        }
    }
}

/// A fate thread connecting two souls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FateThread {
    /// ID of the other soul.
    pub other_soul_id: u64,
    /// Name of the other soul's current/last incarnation.
    pub other_name: String,
    /// Strength of the thread.
    pub strength: ThreadStrength,
    /// Nature of the connection.
    pub nature: ThreadNature,
    /// How the thread was formed.
    pub origin: String,
    /// Number of lifetimes this thread has existed.
    pub age: u32,
}

impl FateThread {
    /// Create a new fate thread.
    pub fn new(other_soul_id: u64, other_name: impl Into<String>) -> Self {
        Self {
            other_soul_id,
            other_name: other_name.into(),
            strength: ThreadStrength::Faint,
            nature: ThreadNature::Positive,
            origin: String::new(),
            age: 0,
        }
    }
    
    /// Set thread strength.
    pub fn with_strength(mut self, strength: ThreadStrength) -> Self {
        self.strength = strength;
        self
    }
    
    /// Set thread nature.
    pub fn with_nature(mut self, nature: ThreadNature) -> Self {
        self.nature = nature;
        self
    }
    
    /// Set origin description.
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = origin.into();
        self
    }
    
    /// Strengthen the thread.
    pub fn strengthen(&mut self) {
        self.strength = self.strength.strengthen();
    }
    
    /// Age the thread by one incarnation.
    /// Returns false if thread has faded.
    pub fn age_thread(&mut self) -> bool {
        self.age += 1;
        
        // Very old threads may weaken
        if self.age > 5 && self.strength != ThreadStrength::Bound {
            if let Some(weaker) = self.strength.weaken() {
                self.strength = weaker;
            } else {
                return false; // Thread has faded
            }
        }
        
        true
    }
    
    /// Check encounter chance.
    pub fn encounter_chance(&self) -> u32 {
        self.strength.encounter_chance()
    }
    
    /// Get display string.
    pub fn display(&self) -> String {
        format!("{} thread to {} ({}) - {}",
            self.strength.name(),
            self.other_name,
            self.nature.name(),
            self.nature.description()
        )
    }
}

// === Fate Thread Factories ===

/// Create a thread from friendship.
pub fn from_friendship(soul_id: u64, name: impl Into<String>) -> FateThread {
    FateThread::new(soul_id, name)
        .with_nature(ThreadNature::Positive)
        .with_origin("friendship in a past life")
}

/// Create a thread from love.
pub fn from_love(soul_id: u64, name: impl Into<String>) -> FateThread {
    FateThread::new(soul_id, name)
        .with_strength(ThreadStrength::Strong)
        .with_nature(ThreadNature::Positive)
        .with_origin("love that transcends death")
}

/// Create a thread from betrayal.
pub fn from_betrayal(soul_id: u64, name: impl Into<String>) -> FateThread {
    FateThread::new(soul_id, name)
        .with_strength(ThreadStrength::Moderate)
        .with_nature(ThreadNature::Negative)
        .with_origin("betrayal that cannot be forgotten")
}

/// Create a thread from rivalry.
pub fn from_rivalry(soul_id: u64, name: impl Into<String>) -> FateThread {
    FateThread::new(soul_id, name)
        .with_nature(ThreadNature::Negative)
        .with_origin("eternal rivalry")
}

/// Create a thread from sacrifice.
pub fn from_sacrifice(soul_id: u64, name: impl Into<String>) -> FateThread {
    FateThread::new(soul_id, name)
        .with_strength(ThreadStrength::Bound)
        .with_nature(ThreadNature::Positive)
        .with_origin("life given for another")
}
