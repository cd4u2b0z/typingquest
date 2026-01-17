//! Save/Load System - Persistent game state serialization
//! 
//! Handles saving and loading game progress using serde + RON format.
//! The system is designed to be forward-compatible with future versions.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io;

use super::player::Player;
use super::dungeon::Dungeon;

/// Version of the save format for migration support
const SAVE_VERSION: u32 = 1;

/// Complete save file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFile {
    /// Save format version for future compatibility
    pub version: u32,
    /// Unix timestamp of when the save was created
    pub timestamp: u64,
    /// Total playtime in seconds
    pub playtime_seconds: u64,
    /// The actual game data
    pub data: SaveData,
}

/// Core save data - everything needed to restore game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    // Player state
    pub player: PlayerSave,
    
    // Dungeon progress
    pub dungeon: DungeonSave,
    
    // Global stats
    pub stats: GameStats,
    
    // Unlocks and achievements
    pub unlocks: UnlockState,
}

/// Serializable player state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSave {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub experience: u64,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub gold: u64,
    pub inventory: Vec<ItemSave>,
    pub equipped: EquipmentSave,
    pub skills_unlocked: Vec<String>,
}

/// Serializable item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSave {
    pub id: String,
    pub name: String,
    pub quantity: u32,
}

/// Serializable equipment slots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EquipmentSave {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub accessory: Option<String>,
}

/// Serializable dungeon state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonSave {
    pub current_floor: i32,
    pub rooms_cleared: i32,
    pub seed: Option<u64>,
}

/// Persistent game statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameStats {
    pub runs_completed: i32,
    pub runs_won: i32,
    pub total_enemies_defeated: i32,
    pub total_bosses_defeated: i32,
    pub total_words_typed: i32,
    pub total_perfect_words: i32,
    pub highest_combo: i32,
    pub highest_wpm: f32,
    pub total_playtime_seconds: u64,
    pub total_gold_earned: u64,
    pub total_damage_dealt: i64,
    pub total_damage_taken: i64,
}

/// Unlocks and progression
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnlockState {
    pub classes_unlocked: Vec<String>,
    pub achievements: Vec<String>,
    pub lore_discovered: Vec<String>,
    pub highest_floor_reached: i32,
}

/// Errors that can occur during save/load
#[derive(Debug)]
pub enum SaveError {
    IoError(io::Error),
    SerializeError(String),
    DeserializeError(String),
    VersionMismatch { expected: u32, found: u32 },
    CorruptedSave,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::IoError(e) => write!(f, "IO error: {}", e),
            SaveError::SerializeError(e) => write!(f, "Serialization error: {}", e),
            SaveError::DeserializeError(e) => write!(f, "Deserialization error: {}", e),
            SaveError::VersionMismatch { expected, found } => {
                write!(f, "Save version mismatch: expected {}, found {}", expected, found)
            }
            SaveError::CorruptedSave => write!(f, "Save file is corrupted"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<io::Error> for SaveError {
    fn from(err: io::Error) -> Self {
        SaveError::IoError(err)
    }
}

/// Get the save directory path
pub fn get_save_dir() -> PathBuf {
    // Try XDG data directory first, then fallback
    if let Ok(data_dir) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(data_dir).join("typingquest")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local/share/typingquest")
    } else {
        PathBuf::from("./saves")
    }
}

/// Get the path to a specific save slot
pub fn get_save_path(slot: u32) -> PathBuf {
    get_save_dir().join(format!("save_{}.ron", slot))
}

/// Save the game to a slot
pub fn save_game(data: &SaveData, slot: u32) -> Result<(), SaveError> {
    let save_dir = get_save_dir();
    fs::create_dir_all(&save_dir)?;
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    
    let save_file = SaveFile {
        version: SAVE_VERSION,
        timestamp,
        playtime_seconds: data.stats.total_playtime_seconds,
        data: data.clone(),
    };
    
    let content = ron::ser::to_string_pretty(&save_file, ron::ser::PrettyConfig::default())
        .map_err(|e| SaveError::SerializeError(e.to_string()))?;
    
    let path = get_save_path(slot);
    fs::write(&path, content)?;
    
    Ok(())
}

/// Load a game from a slot
pub fn load_game(slot: u32) -> Result<SaveData, SaveError> {
    let path = get_save_path(slot);
    let content = fs::read_to_string(&path)?;
    
    let save_file: SaveFile = ron::from_str(&content)
        .map_err(|e| SaveError::DeserializeError(e.to_string()))?;
    
    // Version check - in the future, add migration logic here
    if save_file.version > SAVE_VERSION {
        return Err(SaveError::VersionMismatch {
            expected: SAVE_VERSION,
            found: save_file.version,
        });
    }
    
    Ok(save_file.data)
}

/// Check if a save exists in a slot
pub fn save_exists(slot: u32) -> bool {
    get_save_path(slot).exists()
}

/// Delete a save in a slot
pub fn delete_save(slot: u32) -> Result<(), SaveError> {
    let path = get_save_path(slot);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

/// Get metadata about available saves
pub fn get_save_info(slot: u32) -> Option<SaveInfo> {
    let path = get_save_path(slot);
    if !path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&path).ok()?;
    let save_file: SaveFile = ron::from_str(&content).ok()?;
    
    Some(SaveInfo {
        slot,
        player_name: save_file.data.player.name.clone(),
        player_class: save_file.data.player.class.clone(),
        level: save_file.data.player.level,
        floor: save_file.data.dungeon.current_floor,
        playtime_seconds: save_file.playtime_seconds,
        timestamp: save_file.timestamp,
    })
}

/// Summary info about a save for display
#[derive(Debug, Clone)]
pub struct SaveInfo {
    pub slot: u32,
    pub player_name: String,
    pub player_class: String,
    pub level: u32,
    pub floor: i32,
    pub playtime_seconds: u64,
    pub timestamp: u64,
}

impl SaveInfo {
    pub fn format_playtime(&self) -> String {
        let hours = self.playtime_seconds / 3600;
        let minutes = (self.playtime_seconds % 3600) / 60;
        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

// === Conversion implementations ===

impl From<&Player> for PlayerSave {
    fn from(player: &Player) -> Self {
        Self {
            name: player.name.clone(),
            class: format!("{:?}", player.class),
            level: player.level,
            experience: player.experience,
            hp: player.hp,
            max_hp: player.max_hp,
            mp: player.mp,
            max_mp: player.max_mp,
            gold: player.gold,
            inventory: player.inventory.iter().map(|item| ItemSave {
                id: item.name.to_lowercase().replace(' ', "_"),
                name: item.name.clone(),
                quantity: 1,
            }).collect(),
            equipped: EquipmentSave::default(),
            skills_unlocked: Vec::new(),
        }
    }
}

impl From<&Dungeon> for DungeonSave {
    fn from(dungeon: &Dungeon) -> Self {
        Self {
            current_floor: dungeon.current_floor,
            rooms_cleared: dungeon.rooms_cleared,
            seed: None,
        }
    }
}
