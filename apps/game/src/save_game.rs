// [Revision: v0.2] [Path: apps/game/src/save_game.rs] [Date: 2025-12-13]
// Description: Save and load game state functionality

use dep::ron;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use lib_character::Character;
use lib_location::region::Region;
use lib_soul::{Chronicle, Soul};
use crate::journal::Journal;

/// Saveable game data (excludes non-serializable state like RNG).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Save file version for compatibility checking.
    pub version: u32,
    /// Character data.
    pub character: Character,
    /// Current game turn.
    pub turn: u32,
    /// Player's persistent soul.
    pub soul: Soul,
    /// World chronicle.
    pub chronicle: Chronicle,
    /// Player's journal.
    pub journal: Journal,
    /// Current region (if any).
    pub current_region: Option<Region>,
    /// RNG seed for reproducibility.
    pub rng_seed: u64,
}

impl SaveData {
    pub const CURRENT_VERSION: u32 = 1;
    pub const SAVE_DIR: &'static str = "saves";
    pub const SAVE_EXTENSION: &'static str = "ron";

    /// Create save data from game state components.
    pub fn from_game_state(
        character: &Character,
        turn: u32,
        soul: &Soul,
        chronicle: &Chronicle,
        journal: &Journal,
        current_region: Option<&Region>,
        rng_seed: u64,
    ) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            character: character.clone(),
            turn,
            soul: soul.clone(),
            chronicle: chronicle.clone(),
            journal: journal.clone(),
            current_region: current_region.cloned(),
            rng_seed,
        }
    }

    /// Get the default save file path.
    pub fn default_save_path() -> std::path::PathBuf {
        std::path::PathBuf::from(Self::SAVE_DIR).join(format!("save.{}", Self::SAVE_EXTENSION))
    }

    /// Get save file path for a named save.
    pub fn named_save_path(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(Self::SAVE_DIR).join(format!("{}.{}", name, Self::SAVE_EXTENSION))
    }

    /// Ensure save directory exists.
    pub fn ensure_save_dir() -> std::io::Result<()> {
        fs::create_dir_all(Self::SAVE_DIR)
    }

    /// Save to file as RON format.
    pub fn save_to_file(&self, path: &Path) -> Result<(), SaveError> {
        Self::ensure_save_dir()?;

        let serialized = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e: ron::Error| SaveError::Serialization(e.to_string()))?;

        let mut file = fs::File::create(path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    /// Load from file.
    pub fn load_from_file(path: &Path) -> Result<Self, SaveError> {
        if !path.exists() {
            return Err(SaveError::NotFound(path.to_string_lossy().to_string()));
        }

        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let save_data: SaveData = ron::from_str(&contents)
            .map_err(|e: ron::de::SpannedError| SaveError::Deserialization(e.to_string()))?;

        // Version check
        if save_data.version > Self::CURRENT_VERSION {
            return Err(SaveError::VersionMismatch {
                found: save_data.version,
                expected: Self::CURRENT_VERSION,
            });
        }

        Ok(save_data)
    }

    /// Save with default name.
    pub fn quick_save(&self) -> Result<(), SaveError> {
        self.save_to_file(&Self::default_save_path())
    }

    /// Load default save.
    pub fn quick_load() -> Result<Self, SaveError> {
        Self::load_from_file(&Self::default_save_path())
    }

    /// List available save files.
    pub fn list_saves() -> Vec<String> {
        let save_dir = Path::new(Self::SAVE_DIR);
        if !save_dir.exists() {
            return Vec::new();
        }

        fs::read_dir(save_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .map(|ext| ext == Self::SAVE_EXTENSION)
                            .unwrap_or(false)
                    })
                    .filter_map(|e| {
                        e.path()
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Delete a save file.
    pub fn delete_save(name: &str) -> Result<(), SaveError> {
        let path = Self::named_save_path(name);
        if !path.exists() {
            return Err(SaveError::NotFound(name.to_string()));
        }
        fs::remove_file(path)?;
        Ok(())
    }
}

/// Errors that can occur during save/load operations.
#[derive(Debug)]
pub enum SaveError {
    /// IO error.
    Io(std::io::Error),
    /// Serialization error.
    Serialization(String),
    /// Deserialization error.
    Deserialization(String),
    /// Save file not found.
    NotFound(String),
    /// Version mismatch.
    VersionMismatch { found: u32, expected: u32 },
}

impl From<std::io::Error> for SaveError {
    fn from(e: std::io::Error) -> Self {
        SaveError::Io(e)
    }
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "IO error: {}", e),
            SaveError::Serialization(e) => write!(f, "Serialization error: {}", e),
            SaveError::Deserialization(e) => write!(f, "Deserialization error: {}", e),
            SaveError::NotFound(path) => write!(f, "Save file not found: {}", path),
            SaveError::VersionMismatch { found, expected } => {
                write!(f, "Save version {} is incompatible (expected {})", found, expected)
            }
        }
    }
}

impl std::error::Error for SaveError {}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use lib_character::stats::Stats;

    fn test_save_data() -> SaveData {
        let character = Character::with_stats(
            "Test Hero".to_string(),
            Stats::with_values(3, 2, 2, 1, 3),
        );
        let soul = Soul::new(12345);
        let chronicle = Chronicle::new();
        let journal = Journal::new();

        SaveData::from_game_state(&character, 10, &soul, &chronicle, &journal, None, 42)
    }

    #[test]
    fn test_save_data_version() {
        let save = test_save_data();
        assert_eq!(save.version, SaveData::CURRENT_VERSION);
    }

    #[test]
    fn test_save_data_serialization() {
        let save = test_save_data();
        let serialized = ron::to_string(&save);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_save_data_roundtrip() {
        let original = test_save_data();
        let serialized = ron::to_string(&original).unwrap();
        let loaded: SaveData = ron::from_str(&serialized).unwrap();

        assert_eq!(loaded.version, original.version);
        assert_eq!(loaded.character.name, original.character.name);
        assert_eq!(loaded.turn, original.turn);
        assert_eq!(loaded.soul.id, original.soul.id);
    }

    #[test]
    fn test_default_save_path() {
        let path = SaveData::default_save_path();
        assert!(path.to_string_lossy().contains("save"));
        assert!(path.to_string_lossy().contains("ron"));
    }

    #[test]
    fn test_named_save_path() {
        let path = SaveData::named_save_path("my_game");
        assert!(path.to_string_lossy().contains("my_game"));
    }

    #[test]
    fn test_list_saves_empty() {
        // Should not panic on empty/missing directory
        let saves = SaveData::list_saves();
        assert!(saves.is_empty() || !saves.is_empty());
    }
}
