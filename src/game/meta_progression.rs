//! Meta-Progression System - Hades-style persistent unlocks
//!
//! Progress that persists across deaths. Each run teaches you something
//! about the world and makes future runs slightly different.
//!
//! Design philosophy:
//! - Death is progress, not failure
//! - Story unlocks matter more than power unlocks
//! - Knowledge is the ultimate reward
//! - Variety over power creep

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Persistent meta-progression save
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgress {
    /// Total runs attempted
    pub runs_attempted: u32,
    /// Runs that reached the end
    pub runs_completed: u32,
    /// Currency earned across all runs (ink drops)
    pub total_ink: u64,
    /// Current spendable ink
    pub current_ink: u64,
    /// Unlocked permanent upgrades
    pub unlocks: UnlockTree,
    /// Discovered lore (persists across deaths)
    pub lore_codex: LoreCodex,
    /// NPC relationship progress
    pub npc_bonds: HashMap<String, BondLevel>,
    /// Endings witnessed
    pub endings_seen: HashSet<String>,
    /// Achievements
    pub achievements: HashSet<String>,
    /// Milestones
    pub milestones: Milestones,
    /// Run history (last N runs)
    pub run_history: Vec<RunSummary>,
    /// Current heat level (difficulty modifier)
    pub heat_level: u32,
    /// Highest heat completed
    pub max_heat_completed: u32,
}

/// Unlock tree - persistent upgrades
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnlockTree {
    // Starting bonuses
    pub starting_hp_bonus: i32,          // +5 HP per level
    pub starting_gold_bonus: u32,         // +10 gold per level
    pub starting_item_slots: u32,         // Additional starting items
    
    // Typing improvements
    pub time_bonus_percent: u32,          // +5% time per level
    pub typo_forgiveness: u32,            // Backspaces don't break combo
    pub word_preview: bool,               // See next word coming
    
    // Combat unlocks
    pub damage_bonus_percent: u32,        // +3% damage per level
    pub crit_chance_bonus: u32,           // +2% crit per level
    pub combo_persistence: u32,           // Combo doesn't fully reset
    
    // Exploration unlocks
    pub map_reveal: bool,                 // See full floor layout
    pub trap_detection: bool,             // Traps are visible
    pub secret_sight: bool,               // Hidden rooms glow
    
    // Narrative unlocks
    pub dialogue_memory: bool,            // NPCs remember past runs
    pub lore_hints: bool,                 // Get hints about lore locations
    pub faction_favor: HashMap<String, u32>, // Starting reputation per faction
    
    // Special unlocks
    pub classes_unlocked: HashSet<String>,
    pub starting_spells: HashSet<String>,
    pub starting_items: HashSet<String>,
    pub run_modifiers_unlocked: HashSet<String>,
}

/// Lore discovered that persists
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoreCodex {
    /// Fragments discovered
    pub fragments: HashSet<String>,
    /// Entries fully read
    pub entries_read: HashSet<String>,
    /// Connections player has made
    pub connections: Vec<(String, String)>,
    /// Theories player has noted
    pub player_theories: Vec<String>,
    /// Percentage complete
    pub completion_percent: f32,
}

/// Bond level with an NPC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BondLevel {
    Stranger,      // Never met
    Acquaintance,  // Met once
    Familiar,      // Had multiple conversations
    Friend,        // Completed their quest
    Bonded,        // Maximum relationship
}

impl Default for BondLevel {
    fn default() -> Self {
        BondLevel::Stranger
    }
}

/// Milestone tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Milestones {
    // Typing milestones
    pub highest_wpm: f32,
    pub highest_combo: i32,
    pub total_words_typed: u64,
    pub total_perfect_words: u64,
    pub longest_word_typed: String,
    pub fastest_word_wpm: f32,
    
    // Combat milestones
    pub enemies_defeated: u64,
    pub bosses_defeated: u64,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub perfect_fights: u32,  // No damage taken
    pub speedrun_fights: u32, // Under time limit
    
    // Exploration milestones
    pub floors_explored: u64,
    pub secrets_found: u64,
    pub chests_opened: u64,
    pub traps_triggered: u64,
    pub traps_avoided: u64,
    
    // Narrative milestones
    pub npcs_met: HashSet<String>,
    pub quests_completed: u64,
    pub factions_maxed: HashSet<String>,
    pub story_beats_seen: HashSet<String>,
}

/// Summary of a single run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    /// When the run happened (unix timestamp)
    pub timestamp: u64,
    /// Which class was used
    pub class: String,
    /// How deep did we get
    pub floors_reached: i32,
    /// Did we win?
    pub victory: bool,
    /// How did we die/win
    pub ending: String,
    /// Run duration in seconds
    pub duration_seconds: u64,
    /// Ink earned this run
    pub ink_earned: u64,
    /// Key stats
    pub stats: RunStats,
    /// What modifiers were active
    pub modifiers: Vec<String>,
    /// Heat level
    pub heat: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunStats {
    pub enemies_killed: u32,
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub words_typed: u32,
    pub perfect_words: u32,
    pub max_combo: i32,
    pub avg_wpm: f32,
    pub accuracy: f32,
    pub gold_earned: u64,
    pub items_found: u32,
}

impl Default for MetaProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaProgress {
    pub fn new() -> Self {
        let mut unlocks = UnlockTree::default();
        // Everyone starts with the base class
        unlocks.classes_unlocked.insert("scribe".to_string());
        
        Self {
            runs_attempted: 0,
            runs_completed: 0,
            total_ink: 0,
            current_ink: 0,
            unlocks,
            lore_codex: LoreCodex::default(),
            npc_bonds: HashMap::new(),
            endings_seen: HashSet::new(),
            achievements: HashSet::new(),
            milestones: Milestones::default(),
            run_history: Vec::new(),
            heat_level: 0,
            max_heat_completed: 0,
        }
    }

    // ========================================================================
    // RUN MANAGEMENT
    // ========================================================================

    pub fn start_run(&mut self) -> RunStartBonus {
        self.runs_attempted += 1;
        
        RunStartBonus {
            hp_bonus: self.unlocks.starting_hp_bonus,
            gold_bonus: self.unlocks.starting_gold_bonus as i32,
            time_bonus_percent: self.unlocks.time_bonus_percent as f32 / 100.0,
            damage_bonus_percent: self.unlocks.damage_bonus_percent as f32 / 100.0,
            starting_items: self.unlocks.starting_items.iter().cloned().collect(),
            faction_reputation: self.unlocks.faction_favor.clone(),
            dialogue_memory: self.unlocks.dialogue_memory,
        }
    }

    pub fn end_run(&mut self, summary: RunSummary) {
        // Award ink based on performance
        let ink = self.calculate_ink_reward(&summary);
        self.total_ink += ink;
        self.current_ink += ink;
        
        // Update milestones
        self.update_milestones(&summary);
        
        // Check achievements
        self.check_achievements(&summary);
        
        // Track victory
        if summary.victory {
            self.runs_completed += 1;
            if summary.heat > self.max_heat_completed {
                self.max_heat_completed = summary.heat;
            }
            if !summary.ending.is_empty() {
                self.endings_seen.insert(summary.ending.clone());
            }
        }
        
        // Add to history
        self.run_history.push(summary);
        if self.run_history.len() > 20 {
            self.run_history.remove(0);
        }
    }

    fn calculate_ink_reward(&self, summary: &RunSummary) -> u64 {
        let mut ink: u64 = 0;
        
        // Base ink per floor
        ink += summary.floors_reached as u64 * 10;
        
        // Bonus for enemies
        ink += summary.stats.enemies_killed as u64 * 2;
        
        // Typing performance bonus
        if summary.stats.accuracy > 0.95 {
            ink += 50;
        }
        if summary.stats.max_combo >= 20 {
            ink += summary.stats.max_combo as u64;
        }
        if summary.stats.avg_wpm >= 80.0 {
            ink += 30;
        }
        
        // Victory bonus
        if summary.victory {
            ink += 200;
            // Heat multiplier
            ink += summary.heat as u64 * 50;
        }
        
        // First time bonuses
        if self.runs_attempted == 1 {
            ink += 100; // First run bonus
        }
        if summary.victory && self.runs_completed == 0 {
            ink += 500; // First victory bonus
        }
        
        ink
    }

    fn update_milestones(&mut self, summary: &RunSummary) {
        let m = &mut self.milestones;
        let s = &summary.stats;
        
        if s.avg_wpm > m.highest_wpm {
            m.highest_wpm = s.avg_wpm;
        }
        if s.max_combo > m.highest_combo {
            m.highest_combo = s.max_combo;
        }
        
        m.total_words_typed += s.words_typed as u64;
        m.total_perfect_words += s.perfect_words as u64;
        m.enemies_defeated += s.enemies_killed as u64;
        m.damage_dealt += s.damage_dealt;
        m.damage_taken += s.damage_taken;
        m.floors_explored += summary.floors_reached as u64;
    }

    fn check_achievements(&mut self, summary: &RunSummary) {
        // Speed demon - 100 WPM average
        if summary.stats.avg_wpm >= 100.0 {
            self.achievements.insert("speed_demon".to_string());
        }
        
        // Perfect run - no damage taken
        if summary.stats.damage_taken == 0 && summary.floors_reached >= 3 {
            self.achievements.insert("untouchable".to_string());
        }
        
        // Combo master - 50+ combo
        if summary.stats.max_combo >= 50 {
            self.achievements.insert("combo_master".to_string());
        }
        
        // Perfectionist - 100% accuracy
        if summary.stats.accuracy >= 0.999 && summary.stats.words_typed >= 50 {
            self.achievements.insert("perfectionist".to_string());
        }
        
        // Survivor - complete a run
        if summary.victory {
            self.achievements.insert("survivor".to_string());
        }
        
        // True Ending - see the Third Grammar ending
        if summary.ending == "third_grammar" {
            self.achievements.insert("true_ending".to_string());
        }
        
        // Lore master - discover 75% of lore
        if self.lore_codex.completion_percent >= 75.0 {
            self.achievements.insert("lore_master".to_string());
        }
    }

    // ========================================================================
    // UNLOCKS
    // ========================================================================

    pub fn get_available_upgrades(&self) -> Vec<Upgrade> {
        let mut upgrades = Vec::new();
        
        // HP upgrades (5 levels)
        if self.unlocks.starting_hp_bonus < 25 {
            let level = (self.unlocks.starting_hp_bonus / 5) + 1;
            upgrades.push(Upgrade {
                id: "starting_hp".to_string(),
                name: format!("Vital Words {}", level),
                description: format!("+5 starting HP (currently +{})", self.unlocks.starting_hp_bonus),
                cost: 50 * level as u64,
                category: UpgradeCategory::Survival,
            });
        }
        
        // Gold upgrades (5 levels)
        if self.unlocks.starting_gold_bonus < 50 {
            let level = (self.unlocks.starting_gold_bonus / 10) + 1;
            upgrades.push(Upgrade {
                id: "starting_gold".to_string(),
                name: format!("Gilded Pages {}", level),
                description: format!("+10 starting gold (currently +{})", self.unlocks.starting_gold_bonus),
                cost: 40 * level as u64,
                category: UpgradeCategory::Economy,
            });
        }
        
        // Time bonus (5 levels)
        if self.unlocks.time_bonus_percent < 25 {
            let level = (self.unlocks.time_bonus_percent / 5) + 1;
            upgrades.push(Upgrade {
                id: "time_bonus".to_string(),
                name: format!("Extended Deadline {}", level),
                description: format!("+5% typing time (currently +{}%)", self.unlocks.time_bonus_percent),
                cost: 60 * level as u64,
                category: UpgradeCategory::Typing,
            });
        }
        
        // Damage bonus (5 levels)
        if self.unlocks.damage_bonus_percent < 15 {
            let level = (self.unlocks.damage_bonus_percent / 3) + 1;
            upgrades.push(Upgrade {
                id: "damage_bonus".to_string(),
                name: format!("Cutting Words {}", level),
                description: format!("+3% damage (currently +{}%)", self.unlocks.damage_bonus_percent),
                cost: 70 * level as u64,
                category: UpgradeCategory::Combat,
            });
        }
        
        // Word preview (one-time)
        if !self.unlocks.word_preview {
            upgrades.push(Upgrade {
                id: "word_preview".to_string(),
                name: "Foresight".to_string(),
                description: "See the next word before it appears".to_string(),
                cost: 200,
                category: UpgradeCategory::Typing,
            });
        }
        
        // Map reveal (one-time)
        if !self.unlocks.map_reveal {
            upgrades.push(Upgrade {
                id: "map_reveal".to_string(),
                name: "Cartographer's Eye".to_string(),
                description: "See the full floor layout".to_string(),
                cost: 150,
                category: UpgradeCategory::Exploration,
            });
        }
        
        // Dialogue memory (one-time)
        if !self.unlocks.dialogue_memory {
            upgrades.push(Upgrade {
                id: "dialogue_memory".to_string(),
                name: "Persistent Echo".to_string(),
                description: "NPCs remember you between runs".to_string(),
                cost: 300,
                category: UpgradeCategory::Narrative,
            });
        }
        
        // Typo forgiveness (3 levels)
        if self.unlocks.typo_forgiveness < 3 {
            let level = self.unlocks.typo_forgiveness + 1;
            upgrades.push(Upgrade {
                id: "typo_forgiveness".to_string(),
                name: format!("Eraser {}", level),
                description: format!("{} backspace(s) won't break combo", level),
                cost: 100 * level as u64,
                category: UpgradeCategory::Typing,
            });
        }
        
        // Class unlocks based on achievements
        if self.achievements.contains("survivor") && !self.unlocks.classes_unlocked.contains("mechanist") {
            upgrades.push(Upgrade {
                id: "class_mechanist".to_string(),
                name: "Mechanist Class".to_string(),
                description: "Unlock the Mechanist starting class".to_string(),
                cost: 250,
                category: UpgradeCategory::Class,
            });
        }
        
        if self.achievements.contains("combo_master") && !self.unlocks.classes_unlocked.contains("shadow") {
            upgrades.push(Upgrade {
                id: "class_shadow".to_string(),
                name: "Shadow Writer Class".to_string(),
                description: "Unlock the Shadow Writer starting class".to_string(),
                cost: 300,
                category: UpgradeCategory::Class,
            });
        }
        
        upgrades
    }

    pub fn purchase_upgrade(&mut self, upgrade_id: &str) -> Result<(), &'static str> {
        let upgrades = self.get_available_upgrades();
        let upgrade = upgrades.iter()
            .find(|u| u.id == upgrade_id)
            .ok_or("Upgrade not available")?;
        
        if self.current_ink < upgrade.cost {
            return Err("Not enough ink");
        }
        
        self.current_ink -= upgrade.cost;
        
        match upgrade_id {
            "starting_hp" => self.unlocks.starting_hp_bonus += 5,
            "starting_gold" => self.unlocks.starting_gold_bonus += 10,
            "time_bonus" => self.unlocks.time_bonus_percent += 5,
            "damage_bonus" => self.unlocks.damage_bonus_percent += 3,
            "word_preview" => self.unlocks.word_preview = true,
            "map_reveal" => self.unlocks.map_reveal = true,
            "dialogue_memory" => self.unlocks.dialogue_memory = true,
            "typo_forgiveness" => self.unlocks.typo_forgiveness += 1,
            "class_mechanist" => { self.unlocks.classes_unlocked.insert("mechanist".to_string()); }
            "class_shadow" => { self.unlocks.classes_unlocked.insert("shadow".to_string()); }
            _ => return Err("Unknown upgrade"),
        }
        
        Ok(())
    }

    // ========================================================================
    // LORE CODEX
    // ========================================================================

    pub fn discover_lore(&mut self, fragment_id: &str) {
        if self.lore_codex.fragments.insert(fragment_id.to_string()) {
            // New discovery
            self.update_lore_completion();
        }
    }

    pub fn mark_lore_read(&mut self, fragment_id: &str) {
        self.lore_codex.entries_read.insert(fragment_id.to_string());
    }

    pub fn add_lore_connection(&mut self, a: &str, b: &str) {
        self.lore_codex.connections.push((a.to_string(), b.to_string()));
    }

    fn update_lore_completion(&mut self) {
        // Total fragments is defined in lore_fragments.rs
        let total = 15; // Update this to match actual count
        let found = self.lore_codex.fragments.len();
        self.lore_codex.completion_percent = (found as f32 / total as f32) * 100.0;
    }

    // ========================================================================
    // NPC BONDS
    // ========================================================================

    pub fn get_bond(&self, npc: &str) -> BondLevel {
        *self.npc_bonds.get(npc).unwrap_or(&BondLevel::Stranger)
    }

    pub fn advance_bond(&mut self, npc: &str) {
        let current = self.get_bond(npc);
        let next = match current {
            BondLevel::Stranger => BondLevel::Acquaintance,
            BondLevel::Acquaintance => BondLevel::Familiar,
            BondLevel::Familiar => BondLevel::Friend,
            BondLevel::Friend => BondLevel::Bonded,
            BondLevel::Bonded => BondLevel::Bonded,
        };
        self.npc_bonds.insert(npc.to_string(), next);
    }
}

/// Bonuses applied at run start
#[derive(Debug, Clone)]
pub struct RunStartBonus {
    pub hp_bonus: i32,
    pub gold_bonus: i32,
    pub time_bonus_percent: f32,
    pub damage_bonus_percent: f32,
    pub starting_items: Vec<String>,
    pub faction_reputation: HashMap<String, u32>,
    pub dialogue_memory: bool,
}

/// An upgrade available for purchase
#[derive(Debug, Clone)]
pub struct Upgrade {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: u64,
    pub category: UpgradeCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeCategory {
    Survival,
    Economy,
    Typing,
    Combat,
    Exploration,
    Narrative,
    Class,
}

impl UpgradeCategory {
    pub fn name(&self) -> &'static str {
        match self {
            UpgradeCategory::Survival => "Survival",
            UpgradeCategory::Economy => "Economy",
            UpgradeCategory::Typing => "Typing",
            UpgradeCategory::Combat => "Combat",
            UpgradeCategory::Exploration => "Exploration",
            UpgradeCategory::Narrative => "Narrative",
            UpgradeCategory::Class => "Classes",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            UpgradeCategory::Survival => "󰣐",
            UpgradeCategory::Economy => "󰆼",
            UpgradeCategory::Typing => "󰌌",
            UpgradeCategory::Combat => "󰓥",
            UpgradeCategory::Exploration => "󰦑",
            UpgradeCategory::Narrative => "󰂺",
            UpgradeCategory::Class => "󱃵",
        }
    }
}
