#![allow(dead_code, unused_imports, unused_variables)]
//! Data module - External content loading for data-driven design
//! 
//! This module handles loading game content from RON files,
//! separating data from logic for easy content updates.

pub mod sentences;
pub mod word_lists;
pub mod enemies;

use std::fs;
use std::path::Path;
use rand::seq::SliceRandom;

pub use sentences::SentenceDatabase;
pub use word_lists::WordDatabase;
pub use enemies::EnemyDatabase;

/// Error type for data loading operations
#[derive(Debug)]
pub enum DataError {
    IoError(std::io::Error),
    ParseError(String),
}

impl From<std::io::Error> for DataError {
    fn from(err: std::io::Error) -> Self {
        DataError::IoError(err)
    }
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::IoError(e) => write!(f, "IO error: {}", e),
            DataError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for DataError {}

/// Load a RON file and deserialize it
pub fn load_ron<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, DataError> {
    let content = fs::read_to_string(path)?;
    ron::from_str(&content).map_err(|e| DataError::ParseError(e.to_string()))
}

/// Get the data directory path
pub fn data_dir() -> std::path::PathBuf {
    let paths = [
        std::path::PathBuf::from("data"),
        std::path::PathBuf::from("assets/data"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("data")))
            .unwrap_or_default(),
    ];
    
    for path in paths {
        if path.exists() {
            return path;
        }
    }
    
    std::path::PathBuf::from("data")
}

/// Central game data repository - holds all loaded content
#[derive(Debug, Clone)]
pub struct GameData {
    pub sentences: SentenceDatabase,
    pub words: WordDatabase,
    pub enemies: EnemyDatabase,
}

impl Default for GameData {
    fn default() -> Self {
        Self::new()
    }
}

impl GameData {
    /// Create a new GameData with embedded defaults
    pub fn new() -> Self {
        Self {
            sentences: SentenceDatabase::default(),
            words: WordDatabase::default(),
            enemies: EnemyDatabase::default(),
        }
    }
    
    /// Try to load data from external RON files, falling back to embedded defaults
    pub fn load_or_default() -> Self {
        let data_path = data_dir();
        
        let sentences_path = data_path.join("sentences.ron");
        let words_path = data_path.join("words.ron");
        let enemies_path = data_path.join("enemies.ron");
        
        Self {
            sentences: load_ron(&sentences_path).unwrap_or_default(),
            words: load_ron(&words_path).unwrap_or_default(),
            enemies: load_ron(&enemies_path).unwrap_or_default(),
        }
    }
    
    /// Get a random word appropriate for the given difficulty (1-10)
    pub fn get_word(&self, difficulty: u32) -> String {
        let words = self.words.get_by_difficulty(difficulty);
        let mut rng = rand::thread_rng();
        words.choose(&mut rng)
            .map(|s| (*s).clone())
            .unwrap_or_else(|| "word".to_string())
    }
    
    /// Get a random sentence appropriate for the given difficulty (1-10)
    pub fn get_sentence(&self, difficulty: u32) -> String {
        let sentences = self.sentences.get_by_difficulty(difficulty, difficulty);
        let mut rng = rand::thread_rng();
        sentences.choose(&mut rng)
            .map(|e| e.text.clone())
            .unwrap_or_else(|| "Type this sentence.".to_string())
    }
    
    /// Get themed words (for specific enemy types)
    pub fn get_themed_words(&self, theme: &str) -> Vec<String> {
        self.words.get_themed(theme)
            .into_iter()
            .cloned()
            .collect()
    }
    
    /// Get faction-specific sentences
    pub fn get_faction_sentences(&self, faction: &str) -> Vec<String> {
        self.sentences.get_faction_sentences(faction)
            .into_iter()
            .map(|e| e.text.clone())
            .collect()
    }
    
    /// Get boss-specific sentences
    pub fn get_boss_sentences(&self, boss_id: &str) -> Vec<String> {
        self.sentences.get_boss_sentences(boss_id)
            .into_iter()
            .map(|e| e.text.clone())
            .collect()
    }
}
