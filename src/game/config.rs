//! Game Configuration System
//! 
//! Centralized configuration for game balance, typing mechanics,
//! and user preferences. All tunable values live here.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Master game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Core typing mechanics
    pub typing: TypingConfig,
    
    /// Combat balance
    pub combat: CombatConfig,
    
    /// Difficulty settings
    pub difficulty: DifficultyConfig,
    
    /// Display and UI preferences
    pub display: DisplayConfig,
    
    /// Audio settings (for future use)
    pub audio: AudioConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            typing: TypingConfig::default(),
            combat: CombatConfig::default(),
            difficulty: DifficultyConfig::default(),
            display: DisplayConfig::default(),
            audio: AudioConfig::default(),
        }
    }
}

/// Typing mechanics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingConfig {
    /// Base time per character in seconds
    pub base_time_per_char: f32,
    
    /// Minimum WPM for speed bonus
    pub speed_bonus_threshold_wpm: f32,
    
    /// WPM at which maximum speed bonus is achieved
    pub speed_bonus_max_wpm: f32,
    
    /// Maximum speed bonus multiplier
    pub speed_bonus_max_mult: f32,
    
    /// Perfect word bonus multiplier (no backspaces)
    pub perfect_bonus_mult: f32,
    
    /// Accuracy threshold for penalty (below this = penalty)
    pub accuracy_penalty_threshold: f32,
    
    /// Maximum accuracy penalty multiplier
    pub accuracy_penalty_max: f32,
    
    /// Whether to allow backspace during combat
    pub allow_backspace: bool,
    
    /// Maximum backspaces allowed per word (0 = unlimited)
    pub max_backspaces_per_word: u32,
    
    /// Penalty per backspace (damage reduction %)
    pub backspace_penalty: f32,
}

impl Default for TypingConfig {
    fn default() -> Self {
        Self {
            base_time_per_char: 0.3,
            speed_bonus_threshold_wpm: 60.0,
            speed_bonus_max_wpm: 120.0,
            speed_bonus_max_mult: 1.5,
            perfect_bonus_mult: 1.25,
            accuracy_penalty_threshold: 0.85,
            accuracy_penalty_max: 0.5,
            allow_backspace: true,
            max_backspaces_per_word: 0, // unlimited
            backspace_penalty: 0.05,
        }
    }
}

/// Combat balance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    /// Starting combo multiplier
    pub combo_base_mult: f32,
    
    /// Combo increment per word
    pub combo_increment: f32,
    
    /// Maximum combo multiplier
    pub combo_max_mult: f32,
    
    /// Combo breaks on failed word
    pub combo_breaks_on_fail: bool,
    
    /// Combo timeout in seconds (0 = no timeout)
    pub combo_timeout: f32,
    
    /// Streak bonuses
    pub streak_thresholds: Vec<u32>,
    
    /// Bonus damage per streak level
    pub streak_bonus_per_level: f32,
    
    /// Critical hit chance base (percentage)
    pub crit_chance_base: f32,
    
    /// Critical hit damage multiplier
    pub crit_damage_mult: f32,
    
    /// Flee success chance base (percentage)
    pub flee_chance_base: f32,
    
    /// HP regeneration per floor cleared
    pub hp_regen_per_floor: f32,
    
    /// MP regeneration per combat victory
    pub mp_regen_per_victory: f32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            combo_base_mult: 1.0,
            combo_increment: 0.1,
            combo_max_mult: 3.0,
            combo_breaks_on_fail: true,
            combo_timeout: 3.0,
            streak_thresholds: vec![3, 5, 10, 15, 20],
            streak_bonus_per_level: 0.1,
            crit_chance_base: 5.0,
            crit_damage_mult: 2.0,
            flee_chance_base: 40.0,
            hp_regen_per_floor: 0.0,
            mp_regen_per_victory: 0.1,
        }
    }
}

/// Difficulty presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyPreset {
    Story,    // Easy mode for story enjoyment
    Normal,   // Standard challenge
    Hard,     // For experienced typists
    Ironman,  // Permadeath, no saves
    Custom,   // User-defined settings
}

/// Difficulty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyConfig {
    /// Current difficulty preset
    pub preset: DifficultyPreset,
    
    /// Enemy HP multiplier
    pub enemy_hp_mult: f32,
    
    /// Enemy damage multiplier
    pub enemy_damage_mult: f32,
    
    /// Player damage multiplier
    pub player_damage_mult: f32,
    
    /// Gold drop multiplier
    pub gold_drop_mult: f32,
    
    /// XP gain multiplier
    pub xp_gain_mult: f32,
    
    /// Time limit multiplier (higher = more time)
    pub time_mult: f32,
    
    /// Word difficulty scaling
    /// 0.0 = only easy words, 1.0 = full difficulty scaling
    pub word_difficulty_scale: f32,
    
    /// Adaptive difficulty enabled
    pub adaptive_difficulty: bool,
    
    /// Floor scaling factor (how much harder each floor gets)
    pub floor_scaling: f32,
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        Self::from_preset(DifficultyPreset::Normal)
    }
}

impl DifficultyConfig {
    pub fn from_preset(preset: DifficultyPreset) -> Self {
        match preset {
            DifficultyPreset::Story => Self {
                preset,
                enemy_hp_mult: 0.7,
                enemy_damage_mult: 0.5,
                player_damage_mult: 1.3,
                gold_drop_mult: 1.5,
                xp_gain_mult: 1.5,
                time_mult: 1.5,
                word_difficulty_scale: 0.5,
                adaptive_difficulty: true,
                floor_scaling: 0.05,
            },
            DifficultyPreset::Normal => Self {
                preset,
                enemy_hp_mult: 1.0,
                enemy_damage_mult: 1.0,
                player_damage_mult: 1.0,
                gold_drop_mult: 1.0,
                xp_gain_mult: 1.0,
                time_mult: 1.0,
                word_difficulty_scale: 1.0,
                adaptive_difficulty: true,
                floor_scaling: 0.1,
            },
            DifficultyPreset::Hard => Self {
                preset,
                enemy_hp_mult: 1.5,
                enemy_damage_mult: 1.3,
                player_damage_mult: 0.9,
                gold_drop_mult: 0.8,
                xp_gain_mult: 0.8,
                time_mult: 0.8,
                word_difficulty_scale: 1.0,
                adaptive_difficulty: false,
                floor_scaling: 0.15,
            },
            DifficultyPreset::Ironman => Self {
                preset,
                enemy_hp_mult: 1.3,
                enemy_damage_mult: 1.2,
                player_damage_mult: 1.0,
                gold_drop_mult: 1.0,
                xp_gain_mult: 1.2,
                time_mult: 0.9,
                word_difficulty_scale: 1.0,
                adaptive_difficulty: false,
                floor_scaling: 0.12,
            },
            DifficultyPreset::Custom => Self {
                preset,
                enemy_hp_mult: 1.0,
                enemy_damage_mult: 1.0,
                player_damage_mult: 1.0,
                gold_drop_mult: 1.0,
                xp_gain_mult: 1.0,
                time_mult: 1.0,
                word_difficulty_scale: 1.0,
                adaptive_difficulty: true,
                floor_scaling: 0.1,
            },
        }
    }
}

/// Display configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Show damage numbers
    pub show_damage_numbers: bool,
    
    /// Show combo indicator
    pub show_combo: bool,
    
    /// Show WPM during combat
    pub show_wpm: bool,
    
    /// Show accuracy percentage
    pub show_accuracy: bool,
    
    /// Typing cursor style
    pub cursor_style: CursorStyle,
    
    /// Color scheme
    pub color_scheme: ColorScheme,
    
    /// Enable screen shake on damage
    pub screen_shake: bool,
    
    /// Message log length
    pub message_log_length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorScheme {
    Default,
    HighContrast,
    Monochrome,
    Retro,
    Custom,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_damage_numbers: true,
            show_combo: true,
            show_wpm: true,
            show_accuracy: true,
            cursor_style: CursorStyle::Block,
            color_scheme: ColorScheme::Default,
            screen_shake: true,
            message_log_length: 10,
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Master volume (0.0 - 1.0)
    pub master_volume: f32,
    
    /// Sound effects volume
    pub sfx_volume: f32,
    
    /// Music volume
    pub music_volume: f32,
    
    /// Enable typing sounds
    pub typing_sounds: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            typing_sounds: true,
        }
    }
}

// === Configuration File Management ===

/// Get the config directory path
pub fn get_config_dir() -> PathBuf {
    if let Ok(config_dir) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(config_dir).join("typingquest")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config/typingquest")
    } else {
        PathBuf::from("./config")
    }
}

/// Get the config file path
pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.ron")
}

/// Load configuration from file, or return default
pub fn load_config() -> GameConfig {
    let path = get_config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                match ron::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("Config parse error: {}", e),
                }
            }
            Err(e) => eprintln!("Config read error: {}", e),
        }
    }
    GameConfig::default()
}

/// Save configuration to file
pub fn save_config(config: &GameConfig) -> std::io::Result<()> {
    let dir = get_config_dir();
    fs::create_dir_all(&dir)?;
    
    let content = ron::ser::to_string_pretty(config, ron::ser::PrettyConfig::default())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    
    fs::write(get_config_path(), content)?;
    Ok(())
}

// === Keybinding Configuration ===

/// Keybinding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub menu_up: Vec<String>,
    pub menu_down: Vec<String>,
    pub confirm: Vec<String>,
    pub cancel: Vec<String>,
    pub inventory: Vec<String>,
    pub stats: Vec<String>,
    pub explore: Vec<String>,
    pub flee: Vec<String>,
    pub quick_save: Vec<String>,
    pub quick_load: Vec<String>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            menu_up: vec!["Up".to_string(), "k".to_string()],
            menu_down: vec!["Down".to_string(), "j".to_string()],
            confirm: vec!["Enter".to_string(), "Space".to_string()],
            cancel: vec!["Escape".to_string(), "q".to_string()],
            inventory: vec!["i".to_string()],
            stats: vec!["s".to_string(), "c".to_string()],
            explore: vec!["e".to_string(), "Enter".to_string()],
            flee: vec!["Escape".to_string()],
            quick_save: vec!["F5".to_string()],
            quick_load: vec!["F9".to_string()],
        }
    }
}
