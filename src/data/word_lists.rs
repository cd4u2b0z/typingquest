//! Word list data structures and loaders

use serde::{Deserialize, Serialize};

/// Word lists organized by difficulty and theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDatabase {
    pub easy: Vec<String>,      // 3-4 letters
    pub medium: Vec<String>,    // 5-7 letters
    pub hard: Vec<String>,      // 8-10 letters
    pub expert: Vec<String>,    // 11+ letters
    pub themed: ThemeWords,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeWords {
    pub magic: Vec<String>,
    pub combat: Vec<String>,
    pub nature: Vec<String>,
    pub technology: Vec<String>,
    pub corruption: Vec<String>,
    pub ancient: Vec<String>,
}

impl Default for WordDatabase {
    fn default() -> Self {
        Self::embedded()
    }
}

impl WordDatabase {
    /// Get words by difficulty level (1-10)
    pub fn get_by_difficulty(&self, level: u32) -> Vec<&String> {
        match level {
            1..=2 => self.easy.iter().collect(),
            3..=4 => self.easy.iter().chain(self.medium.iter()).collect(),
            5..=6 => self.medium.iter().collect(),
            7..=8 => self.medium.iter().chain(self.hard.iter()).collect(),
            9..=10 => self.hard.iter().chain(self.expert.iter()).collect(),
            _ => self.medium.iter().collect(),
        }
    }
    
    /// Get themed words for special encounters
    pub fn get_themed(&self, theme: &str) -> Vec<&String> {
        match theme {
            "magic" => self.themed.magic.iter().collect(),
            "combat" => self.themed.combat.iter().collect(),
            "nature" => self.themed.nature.iter().collect(),
            "technology" => self.themed.technology.iter().collect(),
            "corruption" => self.themed.corruption.iter().collect(),
            "ancient" => self.themed.ancient.iter().collect(),
            _ => self.medium.iter().collect(),
        }
    }
    
    /// Embedded default database
    pub fn embedded() -> Self {
        Self {
            easy: vec![
                // Common 3-4 letter words
                "the", "and", "for", "are", "but", "not", "you", "all",
                "can", "had", "her", "was", "one", "our", "out", "day",
                "get", "has", "him", "his", "how", "its", "may", "new",
                "now", "old", "see", "two", "way", "who", "boy", "did",
                "key", "run", "hit", "type", "word", "fast", "slow",
                "book", "read", "mind", "soul", "fire", "wind", "dark",
                "glow", "echo", "void", "rune", "mage", "sage", "lore",
            ].into_iter().map(String::from).collect(),
            
            medium: vec![
                // 5-7 letter words
                "typing", "wisdom", "battle", "scroll", "ancient",
                "script", "symbol", "cipher", "riddle", "mystic",
                "shadow", "forest", "temple", "shrine", "sacred",
                "memory", "keeper", "warden", "scribe", "master",
                "novice", "warrior", "silence", "whisper", "thunder",
                "crystal", "archive", "codex", "grimoire", "essence",
                "energy", "spirit", "phantom", "specter", "wraith",
                "corrupt", "tainted", "blessed", "cursed", "enchant",
                "written", "spoken", "thought", "dreams", "visions",
                "reality", "fiction", "legends", "stories", "history",
                "future", "present", "eternal", "mortal", "divine",
            ].into_iter().map(String::from).collect(),
            
            hard: vec![
                // 8-10 letter words
                "corruption", "unwriting", "keystroke", "precision",
                "transcend", "sanctuary", "illuminate", "obliterate",
                "manuscript", "literature", "philosophy", "meditation",
                "dedication", "persevere", "resilience", "determined",
                "typography", "calligraphy", "vocabulary", "linguistics",
                "alphabetic", "encryption", "decryption", "mysterious",
                "arcane", "forbidden", "knowledge", "revelation",
                "apocalypse", "cataclysm", "prophecy", "chronicle",
                "testament", "scripture", "anthology", "compendium",
            ].into_iter().map(String::from).collect(),
            
            expert: vec![
                // 11+ letter words
                "transcendence", "consciousness", "determination",
                "extraordinary", "philosophical", "understanding",
                "enlightenment", "manifestation", "transformation",
                "disintegration", "reconstitution", "interpretation",
                "communication", "authentication", "configuration",
                "comprehension", "representation", "implementation",
                "documentation", "pronunciation", "disambiguation",
            ].into_iter().map(String::from).collect(),
            
            themed: ThemeWords {
                magic: vec![
                    "spell", "enchant", "conjure", "summon", "invoke",
                    "arcane", "mystic", "eldritch", "sorcery", "wizardry",
                    "incantation", "transmutation", "evocation", "divination",
                    "rune", "glyph", "sigil", "ward", "hex", "curse", "blessing",
                ].into_iter().map(String::from).collect(),
                
                combat: vec![
                    "strike", "parry", "dodge", "block", "thrust",
                    "slash", "pierce", "shatter", "crush", "cleave",
                    "assault", "defend", "counter", "riposte", "feint",
                    "victory", "defeat", "battle", "skirmish", "duel",
                ].into_iter().map(String::from).collect(),
                
                nature: vec![
                    "forest", "river", "mountain", "valley", "meadow",
                    "blossom", "willow", "ancient", "verdant", "serene",
                    "wilderness", "sanctuary", "grove", "glade", "thicket",
                    "stream", "cascade", "canopy", "undergrowth", "roots",
                ].into_iter().map(String::from).collect(),
                
                technology: vec![
                    "keyboard", "terminal", "circuit", "binary", "digital",
                    "process", "execute", "compile", "runtime", "protocol",
                    "algorithm", "interface", "mechanism", "automation",
                    "efficiency", "optimize", "overclock", "bandwidth",
                ].into_iter().map(String::from).collect(),
                
                corruption: vec![
                    "taint", "decay", "rot", "wither", "corrupt",
                    "unwrite", "erase", "dissolve", "fragment", "scatter",
                    "distortion", "aberration", "mutation", "entropy",
                    "void", "null", "empty", "hollow", "broken", "shattered",
                ].into_iter().map(String::from).collect(),
                
                ancient: vec![
                    "primordial", "forgotten", "eternal", "timeless",
                    "ancestral", "prehistoric", "mythical", "legendary",
                    "chronicle", "testament", "scripture", "prophecy",
                    "relic", "artifact", "remnant", "vestige", "monument",
                ].into_iter().map(String::from).collect(),
            },
        }
    }
}
