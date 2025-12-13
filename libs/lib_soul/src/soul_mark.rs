// [Revision: v0.1] [Path: libs/lib_soul/src/soul_mark.rs] [Date: 2025-12-13]
// Description: Soul marks - echoes from past lives

use serde::{Deserialize, Serialize};

/// Type of soul mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkType {
    /// Affinity for a person, place, or thing.
    Affinity,
    /// Fear or aversion.
    Fear,
    /// Unfinished business or purpose.
    Purpose,
    /// Connection to a specific location.
    Location,
    /// Bond with another soul.
    SoulBond,
    /// Skill or knowledge echo.
    Echo,
    /// Wound or trauma.
    Wound,
}

impl MarkType {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            MarkType::Affinity => "Affinity",
            MarkType::Fear => "Fear",
            MarkType::Purpose => "Purpose",
            MarkType::Location => "Location",
            MarkType::SoulBond => "Soul Bond",
            MarkType::Echo => "Echo",
            MarkType::Wound => "Wound",
        }
    }
    
    /// Description of how this mark manifests.
    pub fn manifestation(&self) -> &'static str {
        match self {
            MarkType::Affinity => "unexplained warmth or familiarity",
            MarkType::Fear => "inexplicable dread or avoidance",
            MarkType::Purpose => "a calling or drive without clear origin",
            MarkType::Location => "déjà vu and vivid dreams of a place",
            MarkType::SoulBond => "instant connection with a stranger",
            MarkType::Echo => "instinctive knowledge or skill",
            MarkType::Wound => "phantom pain or emotional triggers",
        }
    }
}

/// Strength of a soul mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MarkStrength {
    /// Faint whisper, easily overlooked.
    Faint,
    /// Clear impression.
    Clear,
    /// Strong and unmistakable.
    Strong,
    /// Overwhelming, defines the character.
    Overwhelming,
}

impl MarkStrength {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            MarkStrength::Faint => "Faint",
            MarkStrength::Clear => "Clear",
            MarkStrength::Strong => "Strong",
            MarkStrength::Overwhelming => "Overwhelming",
        }
    }
    
    /// How many incarnations until this fades.
    pub fn duration(&self) -> u32 {
        match self {
            MarkStrength::Faint => 1,
            MarkStrength::Clear => 2,
            MarkStrength::Strong => 4,
            MarkStrength::Overwhelming => 8,
        }
    }
    
    /// Weaken to next lower strength.
    pub fn weaken(&self) -> Option<MarkStrength> {
        match self {
            MarkStrength::Overwhelming => Some(MarkStrength::Strong),
            MarkStrength::Strong => Some(MarkStrength::Clear),
            MarkStrength::Clear => Some(MarkStrength::Faint),
            MarkStrength::Faint => None,
        }
    }
}

/// A soul mark - an echo from a past life.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulMark {
    /// Type of mark.
    pub mark_type: MarkType,
    /// Strength of the mark.
    pub strength: MarkStrength,
    /// Subject of the mark (name, place, concept).
    pub subject: String,
    /// Description or context.
    pub description: String,
    /// Which incarnation created this mark (0 = current).
    pub source_incarnation: u32,
    /// Remaining duration before fading.
    pub duration_remaining: u32,
    /// Whether this mark has been triggered/revealed.
    pub triggered: bool,
}

impl SoulMark {
    /// Create a new soul mark.
    pub fn new(mark_type: MarkType, strength: MarkStrength, subject: impl Into<String>) -> Self {
        let strength_copy = strength;
        Self {
            mark_type,
            strength,
            subject: subject.into(),
            description: String::new(),
            source_incarnation: 0,
            duration_remaining: strength_copy.duration(),
            triggered: false,
        }
    }
    
    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    /// Set source incarnation.
    pub fn from_incarnation(mut self, incarnation: u32) -> Self {
        self.source_incarnation = incarnation;
        self
    }
    
    /// Mark as triggered.
    pub fn trigger(&mut self) {
        self.triggered = true;
    }
    
    /// Fade the mark by one cycle.
    pub fn fade(&mut self) {
        self.duration_remaining = self.duration_remaining.saturating_sub(1);
        
        if self.duration_remaining == 0 {
            if let Some(weaker) = self.strength.weaken() {
                self.strength = weaker;
                self.duration_remaining = weaker.duration();
            }
        }
    }
    
    /// Check if mark has completely faded.
    pub fn is_faded(&self) -> bool {
        self.strength == MarkStrength::Faint && self.duration_remaining == 0
    }
    
    /// Get display string for this mark.
    pub fn display(&self) -> String {
        format!("{} {} toward {} ({})",
            self.strength.name(),
            self.mark_type.name(),
            self.subject,
            if self.triggered { "awakened" } else { "dormant" }
        )
    }
    
    /// Get narrative hint for this mark.
    pub fn narrative_hint(&self) -> String {
        format!("You feel {}{}",
            self.mark_type.manifestation(),
            if !self.subject.is_empty() {
                format!(" regarding {}", self.subject)
            } else {
                String::new()
            }
        )
    }
}

// === Soul Mark Factories ===

/// Create an affinity mark for a person.
pub fn affinity_person(name: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let name_str = name.into();
    SoulMark::new(MarkType::Affinity, strength, name_str.clone())
        .with_description(format!("An inexplicable warmth when thinking of {}", name_str))
}

/// Create an affinity mark for a place.
pub fn affinity_place(place: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let place_str = place.into();
    SoulMark::new(MarkType::Location, strength, place_str.clone())
        .with_description(format!("Dreams of {} feel like memories", place_str))
}

/// Create a fear mark.
pub fn fear_of(subject: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let subject_str = subject.into();
    SoulMark::new(MarkType::Fear, strength, subject_str.clone())
        .with_description(format!("Unreasonable dread of {}", subject_str))
}

/// Create a purpose mark from an unfulfilled vow.
pub fn unfinished_vow(vow: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let vow_str = vow.into();
    SoulMark::new(MarkType::Purpose, strength, vow_str.clone())
        .with_description(format!("A compulsion to {}", vow_str))
}

/// Create a soul bond mark.
pub fn soul_bond(other_name: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let name_str = other_name.into();
    SoulMark::new(MarkType::SoulBond, strength, name_str.clone())
        .with_description(format!("Fated to encounter {} again", name_str))
}

/// Create a trauma wound mark.
pub fn trauma(cause: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let cause_str = cause.into();
    SoulMark::new(MarkType::Wound, strength, cause_str.clone())
        .with_description(format!("Echoes of {}", cause_str))
}

/// Create a skill echo mark.
pub fn skill_echo(skill: impl Into<String>, strength: MarkStrength) -> SoulMark {
    let skill_str = skill.into();
    SoulMark::new(MarkType::Echo, strength, skill_str.clone())
        .with_description(format!("Instinctive familiarity with {}", skill_str))
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // MarkType tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_mark_type_name() {
        assert_eq!(MarkType::Affinity.name(), "Affinity");
        assert_eq!(MarkType::Fear.name(), "Fear");
        assert_eq!(MarkType::SoulBond.name(), "Soul Bond");
    }

    #[test]
    fn test_mark_type_manifestation() {
        assert!(!MarkType::Fear.manifestation().is_empty());
        assert!(MarkType::Echo.manifestation().contains("knowledge"));
    }

    // -------------------------------------------------------------------------
    // MarkStrength tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_mark_strength_ordering() {
        assert!(MarkStrength::Faint < MarkStrength::Clear);
        assert!(MarkStrength::Clear < MarkStrength::Strong);
        assert!(MarkStrength::Strong < MarkStrength::Overwhelming);
    }

    #[test]
    fn test_mark_strength_duration() {
        assert_eq!(MarkStrength::Faint.duration(), 1);
        assert_eq!(MarkStrength::Clear.duration(), 2);
        assert_eq!(MarkStrength::Strong.duration(), 4);
        assert_eq!(MarkStrength::Overwhelming.duration(), 8);
    }

    #[test]
    fn test_mark_strength_weaken() {
        assert_eq!(MarkStrength::Overwhelming.weaken(), Some(MarkStrength::Strong));
        assert_eq!(MarkStrength::Strong.weaken(), Some(MarkStrength::Clear));
        assert_eq!(MarkStrength::Clear.weaken(), Some(MarkStrength::Faint));
        assert_eq!(MarkStrength::Faint.weaken(), None);
    }

    // -------------------------------------------------------------------------
    // SoulMark tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_soul_mark_creation() {
        let mark = SoulMark::new(MarkType::Fear, MarkStrength::Strong, "fire");
        assert_eq!(mark.mark_type, MarkType::Fear);
        assert_eq!(mark.strength, MarkStrength::Strong);
        assert_eq!(mark.subject, "fire");
        assert_eq!(mark.duration_remaining, 4);
        assert!(!mark.triggered);
    }

    #[test]
    fn test_soul_mark_with_description() {
        let mark = SoulMark::new(MarkType::Purpose, MarkStrength::Clear, "revenge")
            .with_description("Avenge the fallen");
        assert_eq!(mark.description, "Avenge the fallen");
    }

    #[test]
    fn test_soul_mark_from_incarnation() {
        let mark = SoulMark::new(MarkType::Echo, MarkStrength::Faint, "swordsmanship")
            .from_incarnation(3);
        assert_eq!(mark.source_incarnation, 3);
    }

    #[test]
    fn test_soul_mark_trigger() {
        let mut mark = SoulMark::new(MarkType::Affinity, MarkStrength::Clear, "home");
        assert!(!mark.triggered);
        mark.trigger();
        assert!(mark.triggered);
    }

    #[test]
    fn test_soul_mark_fade() {
        let mut mark = SoulMark::new(MarkType::Location, MarkStrength::Clear, "castle");
        assert_eq!(mark.strength, MarkStrength::Clear);
        assert_eq!(mark.duration_remaining, 2); // Clear duration = 2

        mark.fade(); // 2 -> 1
        assert_eq!(mark.duration_remaining, 1);
        assert_eq!(mark.strength, MarkStrength::Clear);

        mark.fade(); // 1 -> 0, weakens to Faint, duration becomes 1
        assert_eq!(mark.strength, MarkStrength::Faint);
        assert_eq!(mark.duration_remaining, 1); // Faint duration = 1
    }

    #[test]
    fn test_soul_mark_is_faded() {
        let mut mark = SoulMark::new(MarkType::Wound, MarkStrength::Faint, "betrayal");
        assert!(!mark.is_faded());

        mark.fade();
        // duration_remaining is now 0, but strength is still Faint
        assert!(mark.is_faded());
    }

    #[test]
    fn test_soul_mark_display() {
        let mark = SoulMark::new(MarkType::Fear, MarkStrength::Strong, "darkness");
        let display = mark.display();
        assert!(display.contains("Strong"));
        assert!(display.contains("Fear"));
        assert!(display.contains("darkness"));
        assert!(display.contains("dormant"));
    }

    #[test]
    fn test_soul_mark_narrative_hint() {
        let mark = SoulMark::new(MarkType::Fear, MarkStrength::Clear, "water");
        let hint = mark.narrative_hint();
        assert!(hint.contains("dread"));
        assert!(hint.contains("water"));
    }

    // -------------------------------------------------------------------------
    // Factory function tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_affinity_person() {
        let mark = affinity_person("Kira", MarkStrength::Strong);
        assert_eq!(mark.mark_type, MarkType::Affinity);
        assert!(mark.description.contains("Kira"));
    }

    #[test]
    fn test_affinity_place() {
        let mark = affinity_place("Shadowmoor", MarkStrength::Clear);
        assert_eq!(mark.mark_type, MarkType::Location);
        assert!(mark.description.contains("Shadowmoor"));
    }

    #[test]
    fn test_fear_of() {
        let mark = fear_of("heights", MarkStrength::Overwhelming);
        assert_eq!(mark.mark_type, MarkType::Fear);
        assert!(mark.description.contains("heights"));
    }

    #[test]
    fn test_unfinished_vow() {
        let mark = unfinished_vow("protect the innocent", MarkStrength::Strong);
        assert_eq!(mark.mark_type, MarkType::Purpose);
        assert!(mark.description.contains("protect"));
    }

    #[test]
    fn test_soul_bond() {
        let mark = soul_bond("The Dark One", MarkStrength::Overwhelming);
        assert_eq!(mark.mark_type, MarkType::SoulBond);
        assert!(mark.description.contains("encounter"));
    }

    #[test]
    fn test_trauma() {
        let mark = trauma("burning", MarkStrength::Clear);
        assert_eq!(mark.mark_type, MarkType::Wound);
        assert!(mark.description.contains("burning"));
    }

    #[test]
    fn test_skill_echo() {
        let mark = skill_echo("navigation", MarkStrength::Faint);
        assert_eq!(mark.mark_type, MarkType::Echo);
        assert!(mark.description.contains("navigation"));
    }
}
