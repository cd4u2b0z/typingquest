//! Run Modifiers System - Hades-style variations that make each run unique
//!
//! Each run applies modifiers that change game rules, spawn rates, and challenges.
//! Some modifiers are chosen, others are seeded from NarrativeSeed.
//!
//! Inspired by: Hades (Pact of Punishment), Slay the Spire (Ascension), Balatro (Jokers)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::narrative::Faction;

/// All modifiers active for this run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunModifiers {
    /// Active modifiers and their levels
    pub active: Vec<ActiveModifier>,
    /// Total "heat" or difficulty level
    pub total_heat: u32,
    /// Rewards multiplier from heat
    pub reward_multiplier: f32,
    /// Run-specific mutations
    pub mutations: Vec<RunMutation>,
    /// Faction-specific run conditions
    pub faction_conditions: HashMap<Faction, FactionRunCondition>,
    /// Special run type
    pub run_type: RunType,
}

impl Default for RunModifiers {
    fn default() -> Self {
        Self {
            active: Vec::new(),
            total_heat: 0,
            reward_multiplier: 1.0,
            mutations: Vec::new(),
            faction_conditions: HashMap::new(),
            run_type: RunType::Standard,
        }
    }
}

impl RunModifiers {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Calculate total heat from active modifiers
    pub fn calculate_heat(&mut self) {
        self.total_heat = self.active.iter().map(|m| m.modifier.heat_cost() * m.level).sum();
        self.reward_multiplier = 1.0 + (self.total_heat as f32 * 0.05);
    }
    
    /// Add a modifier at a given level
    pub fn add_modifier(&mut self, modifier: Modifier, level: u32) {
        if let Some(existing) = self.active.iter_mut().find(|m| std::mem::discriminant(&m.modifier) == std::mem::discriminant(&modifier)) {
            existing.level = level;
        } else {
            self.active.push(ActiveModifier { modifier, level });
        }
        self.calculate_heat();
    }
    
    /// Check if a modifier is active
    pub fn has_modifier(&self, modifier: &Modifier) -> bool {
        self.active.iter().any(|m| std::mem::discriminant(&m.modifier) == std::mem::discriminant(modifier))
    }
    
    /// Get modifier level (0 if not active)
    pub fn modifier_level(&self, modifier: &Modifier) -> u32 {
        self.active.iter()
            .find(|m| std::mem::discriminant(&m.modifier) == std::mem::discriminant(modifier))
            .map(|m| m.level)
            .unwrap_or(0)
    }
    
    /// Get all typing-related modifiers
    pub fn typing_modifiers(&self) -> Vec<&ActiveModifier> {
        self.active.iter().filter(|m| m.modifier.affects_typing()).collect()
    }
    
    /// Get all combat-related modifiers
    pub fn combat_modifiers(&self) -> Vec<&ActiveModifier> {
        self.active.iter().filter(|m| m.modifier.affects_combat()).collect()
    }
    
    /// Apply run type presets
    pub fn set_run_type(&mut self, run_type: RunType) {
        self.run_type = run_type.clone();
        
        match run_type {
            RunType::Standard => {
                // No preset modifiers
            }
            RunType::SpeedRun => {
                self.add_modifier(Modifier::TimeLimit { minutes: 30 }, 1);
                self.add_modifier(Modifier::NoHealing, 1);
            }
            RunType::Pacifist => {
                self.add_modifier(Modifier::PacifistChallenge, 1);
            }
            RunType::NoDamage => {
                self.add_modifier(Modifier::GlassCannon, 1);
            }
            RunType::Ironman => {
                self.add_modifier(Modifier::Permadeath, 1);
                self.add_modifier(Modifier::NoSaving, 1);
            }
            RunType::PureTypist => {
                self.add_modifier(Modifier::NoSkills, 1);
                self.add_modifier(Modifier::NoItems, 1);
            }
            RunType::FactionWar => {
                self.add_modifier(Modifier::AllFactionsHostile, 1);
                self.add_modifier(Modifier::EnhancedFactionRewards, 1);
            }
            RunType::Corruption => {
                self.add_modifier(Modifier::AcceleratedCorruption, 1);
            }
            RunType::DrBaklavaRun => {
                // Easter egg run - everything pastry themed
                self.add_modifier(Modifier::SecretModifier { name: "Phyllo Dimension".to_string() }, 1);
            }
        }
        
        self.calculate_heat();
    }
}

/// A modifier with its active level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveModifier {
    pub modifier: Modifier,
    pub level: u32,
}

impl ActiveModifier {
    pub fn description(&self) -> String {
        self.modifier.description_at_level(self.level)
    }
}

/// All possible run modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Modifier {
    // === Typing Modifiers ===
    /// Words must be typed faster
    SpeedPressure { min_wpm: f32 },
    /// Higher accuracy required
    AccuracyDemand { min_accuracy: f32 },
    /// Mistakes deal damage
    MistakeDamage { damage_per_error: i32 },
    /// Words are longer
    LongerWords { min_length: usize },
    /// Less time per word
    TimeCrunch { time_reduction_percent: f32 },
    /// Words have letters swapped
    DyslexicText { swap_frequency: f32 },
    /// Some letters are invisible
    InvisibleLetters { invisible_percent: f32 },
    /// Words shift while typing
    ShiftingText { shift_frequency: f32 },
    /// Must type at a specific rhythm
    Metronome { target_cpm: f32 },
    /// No backspace allowed
    NoBackspace,
    
    // === Combat Modifiers ===
    /// Enemies have more health
    ToughEnemies { health_multiplier: f32 },
    /// Enemies deal more damage
    DangerousEnemies { damage_multiplier: f32 },
    /// More enemies per encounter
    Swarming { extra_enemies: u32 },
    /// Enemies have armor
    ArmoredEnemies { armor_amount: i32 },
    /// Enemies regenerate
    RegeneratingEnemies { regen_per_turn: i32 },
    /// Boss fights enhanced
    NightmareBosses { enhancement_level: u32 },
    /// Elite enemies appear more often
    EliteSpawn { chance_increase: f32 },
    /// No fleeing from combat
    NoRetreat,
    
    // === Resource Modifiers ===
    /// Less gold from all sources
    GoldDrain { reduction_percent: f32 },
    /// Items cost more
    Inflation { price_multiplier: f32 },
    /// Healing less effective
    WeakHealing { reduction_percent: f32 },
    /// No healing at all
    NoHealing,
    /// Start with less health
    Frail { health_reduction: i32 },
    /// Items break after use
    FragileItems { break_chance: f32 },
    
    // === World Modifiers ===
    /// Corruption spreads faster
    AcceleratedCorruption,
    /// Time passes faster
    AcceleratedTime { multiplier: f32 },
    /// Fewer safe zones
    ReducedHavens,
    /// Random encounters more frequent
    DangerousRoads { encounter_multiplier: f32 },
    /// Weather effects more severe
    HarshClimate,
    /// Traps more common and deadly
    TrapWorld { trap_damage_multiplier: f32 },
    
    // === Faction Modifiers ===
    /// One faction starts hostile
    FactionEnemy { faction: Faction },
    /// All factions start neutral
    Distrusted,
    /// All factions hostile
    AllFactionsHostile,
    /// Faction reputation changes doubled
    VolatileReputation,
    /// Enhanced rewards for faction quests
    EnhancedFactionRewards,
    
    // === Challenge Modifiers ===
    /// One hit kills you
    GlassCannon,
    /// Death is permanent
    Permadeath,
    /// Cannot save
    NoSaving,
    /// Time limit for entire run
    TimeLimit { minutes: u32 },
    /// Must complete run without killing
    PacifistChallenge,
    /// No skills allowed
    NoSkills,
    /// No items allowed
    NoItems,
    
    // === Secret/Easter Egg ===
    SecretModifier { name: String },
}

impl Modifier {
    /// Heat cost (difficulty contribution)
    pub fn heat_cost(&self) -> u32 {
        match self {
            Self::SpeedPressure { .. } => 2,
            Self::AccuracyDemand { .. } => 2,
            Self::MistakeDamage { .. } => 3,
            Self::LongerWords { .. } => 1,
            Self::TimeCrunch { .. } => 3,
            Self::DyslexicText { .. } => 4,
            Self::InvisibleLetters { .. } => 4,
            Self::ShiftingText { .. } => 5,
            Self::Metronome { .. } => 3,
            Self::NoBackspace => 5,
            
            Self::ToughEnemies { .. } => 2,
            Self::DangerousEnemies { .. } => 3,
            Self::Swarming { .. } => 2,
            Self::ArmoredEnemies { .. } => 2,
            Self::RegeneratingEnemies { .. } => 3,
            Self::NightmareBosses { .. } => 4,
            Self::EliteSpawn { .. } => 2,
            Self::NoRetreat => 2,
            
            Self::GoldDrain { .. } => 1,
            Self::Inflation { .. } => 1,
            Self::WeakHealing { .. } => 2,
            Self::NoHealing => 4,
            Self::Frail { .. } => 3,
            Self::FragileItems { .. } => 2,
            
            Self::AcceleratedCorruption => 3,
            Self::AcceleratedTime { .. } => 2,
            Self::ReducedHavens => 3,
            Self::DangerousRoads { .. } => 2,
            Self::HarshClimate => 1,
            Self::TrapWorld { .. } => 2,
            
            Self::FactionEnemy { .. } => 2,
            Self::Distrusted => 2,
            Self::AllFactionsHostile => 5,
            Self::VolatileReputation => 2,
            Self::EnhancedFactionRewards => 0, // Bonus, not penalty
            
            Self::GlassCannon => 10,
            Self::Permadeath => 8,
            Self::NoSaving => 3,
            Self::TimeLimit { .. } => 5,
            Self::PacifistChallenge => 6,
            Self::NoSkills => 4,
            Self::NoItems => 4,
            
            Self::SecretModifier { .. } => 0,
        }
    }
    
    /// Whether this modifier affects typing mechanics
    pub fn affects_typing(&self) -> bool {
        matches!(self,
            Self::SpeedPressure { .. } |
            Self::AccuracyDemand { .. } |
            Self::MistakeDamage { .. } |
            Self::LongerWords { .. } |
            Self::TimeCrunch { .. } |
            Self::DyslexicText { .. } |
            Self::InvisibleLetters { .. } |
            Self::ShiftingText { .. } |
            Self::Metronome { .. } |
            Self::NoBackspace
        )
    }
    
    /// Whether this modifier affects combat
    pub fn affects_combat(&self) -> bool {
        matches!(self,
            Self::ToughEnemies { .. } |
            Self::DangerousEnemies { .. } |
            Self::Swarming { .. } |
            Self::ArmoredEnemies { .. } |
            Self::RegeneratingEnemies { .. } |
            Self::NightmareBosses { .. } |
            Self::EliteSpawn { .. } |
            Self::NoRetreat |
            Self::MistakeDamage { .. }
        )
    }
    
    pub fn name(&self) -> &str {
        match self {
            Self::SpeedPressure { .. } => "Speed Pressure",
            Self::AccuracyDemand { .. } => "Accuracy Demand",
            Self::MistakeDamage { .. } => "Punishing Errors",
            Self::LongerWords { .. } => "Verbose",
            Self::TimeCrunch { .. } => "Time Crunch",
            Self::DyslexicText { .. } => "Scrambled",
            Self::InvisibleLetters { .. } => "Invisible Ink",
            Self::ShiftingText { .. } => "Shifting Sands",
            Self::Metronome { .. } => "Metronome",
            Self::NoBackspace => "No Second Chances",
            
            Self::ToughEnemies { .. } => "Tough Enemies",
            Self::DangerousEnemies { .. } => "Deadly Foes",
            Self::Swarming { .. } => "Swarming",
            Self::ArmoredEnemies { .. } => "Armored",
            Self::RegeneratingEnemies { .. } => "Regenerating",
            Self::NightmareBosses { .. } => "Nightmare",
            Self::EliteSpawn { .. } => "Elite Spawn",
            Self::NoRetreat => "No Retreat",
            
            Self::GoldDrain { .. } => "Gold Drain",
            Self::Inflation { .. } => "Inflation",
            Self::WeakHealing { .. } => "Weak Healing",
            Self::NoHealing => "No Healing",
            Self::Frail { .. } => "Frail",
            Self::FragileItems { .. } => "Fragile Items",
            
            Self::AcceleratedCorruption => "Accelerated Corruption",
            Self::AcceleratedTime { .. } => "Time Flies",
            Self::ReducedHavens => "Reduced Havens",
            Self::DangerousRoads { .. } => "Dangerous Roads",
            Self::HarshClimate => "Harsh Climate",
            Self::TrapWorld { .. } => "Trap World",
            
            Self::FactionEnemy { .. } => "Faction Enemy",
            Self::Distrusted => "Distrusted",
            Self::AllFactionsHostile => "Hostile World",
            Self::VolatileReputation => "Volatile Reputation",
            Self::EnhancedFactionRewards => "Faction Rewards+",
            
            Self::GlassCannon => "Glass Cannon",
            Self::Permadeath => "Permadeath",
            Self::NoSaving => "No Saving",
            Self::TimeLimit { .. } => "Time Limit",
            Self::PacifistChallenge => "Pacifist",
            Self::NoSkills => "No Skills",
            Self::NoItems => "No Items",
            
            Self::SecretModifier { name } => name.as_str(),
        }
    }
    
    pub fn description_at_level(&self, level: u32) -> String {
        match self {
            Self::SpeedPressure { min_wpm } => {
                format!("Must maintain at least {:.0} WPM", min_wpm * level as f32)
            }
            Self::AccuracyDemand { min_accuracy } => {
                format!("Must maintain {:.0}% accuracy", (min_accuracy + level as f32 * 0.05) * 100.0)
            }
            Self::MistakeDamage { damage_per_error } => {
                format!("Each typo deals {} damage", damage_per_error * level as i32)
            }
            Self::ToughEnemies { health_multiplier } => {
                format!("Enemies have {:.0}% more health", (health_multiplier + level as f32 * 0.25 - 1.0) * 100.0)
            }
            Self::DangerousEnemies { damage_multiplier } => {
                format!("Enemies deal {:.0}% more damage", (damage_multiplier + level as f32 * 0.25 - 1.0) * 100.0)
            }
            Self::TimeLimit { minutes } => {
                format!("Complete run in {} minutes", minutes / level)
            }
            Self::GlassCannon => "One hit kills you".to_string(),
            Self::Permadeath => "Death is permanent".to_string(),
            Self::NoBackspace => "Cannot correct mistakes".to_string(),
            Self::AcceleratedCorruption => {
                format!("Corruption spreads {}x faster", level + 1)
            }
            _ => format!("{} (Level {})", self.name(), level),
        }
    }
}

/// Mutations are smaller, stackable modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMutation {
    pub name: String,
    pub description: String,
    pub effect: MutationEffect,
    pub stacks: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationEffect {
    /// Stat modifier
    StatBonus { stat: String, amount: i32 },
    /// Typing modifier
    TypingBonus { wpm_bonus: f32, accuracy_bonus: f32 },
    /// Combat modifier
    CombatBonus { damage: i32, defense: i32 },
    /// Resource modifier
    ResourceBonus { gold_percent: f32, xp_percent: f32 },
    /// Special ability
    SpecialAbility { ability_id: String },
}

/// Faction-specific run conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRunCondition {
    pub faction: Faction,
    /// Special condition for this faction this run
    pub condition: FactionCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionCondition {
    /// Faction is in crisis
    InCrisis { crisis_type: String },
    /// Faction is ascendant
    Ascendant { bonus_rewards: f32 },
    /// Faction is at war
    AtWar { enemy_faction: Faction },
    /// Faction has special mission
    SpecialMission { mission_id: String },
    /// Faction is suspicious of outsiders
    Paranoid { trust_penalty: i32 },
    /// Faction is recruiting heavily
    Recruiting { bonus_reputation: i32 },
}

/// Type of run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunType {
    /// Normal run
    Standard,
    /// Timed run
    SpeedRun,
    /// No killing run
    Pacifist,
    /// No damage run
    NoDamage,
    /// Permadeath with no saves
    Ironman,
    /// Pure typing, no items/skills
    PureTypist,
    /// All factions hostile
    FactionWar,
    /// Start with high corruption
    Corruption,
    /// Easter egg run
    DrBaklavaRun,
}

impl RunType {
    pub fn name(&self) -> &str {
        match self {
            Self::Standard => "Standard",
            Self::SpeedRun => "Speed Run",
            Self::Pacifist => "Pacifist",
            Self::NoDamage => "No Damage",
            Self::Ironman => "Ironman",
            Self::PureTypist => "Pure Typist",
            Self::FactionWar => "Faction War",
            Self::Corruption => "Corruption",
            Self::DrBaklavaRun => "???",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            Self::Standard => "A normal run through the world of TypingQuest.",
            Self::SpeedRun => "Complete the run within the time limit.",
            Self::Pacifist => "Complete the run without killing anyone.",
            Self::NoDamage => "Complete the run without taking any damage.",
            Self::Ironman => "Permadeath, no saving. One life.",
            Self::PureTypist => "No skills, no items. Just typing.",
            Self::FactionWar => "All factions are hostile. Trust no one.",
            Self::Corruption => "The world starts heavily corrupted.",
            Self::DrBaklavaRun => "Something smells... delicious?",
        }
    }
}

/// Generate run modifiers from narrative seed
pub fn generate_from_seed(seed_value: u64) -> RunModifiers {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut modifiers = RunModifiers::new();
    
    // Use seed to determine world conditions
    let mut hasher = DefaultHasher::new();
    seed_value.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Determine number of mutations (1-3)
    let mutation_count = (hash % 3) + 1;
    
    for i in 0..mutation_count {
        let mutation_hash = (hash.wrapping_add(i)) % 10;
        let mutation = match mutation_hash {
            0 => RunMutation {
                name: "Quick Fingers".to_string(),
                description: "Start with +5 WPM".to_string(),
                effect: MutationEffect::TypingBonus { wpm_bonus: 5.0, accuracy_bonus: 0.0 },
                stacks: 1,
            },
            1 => RunMutation {
                name: "Precise".to_string(),
                description: "Start with +5% accuracy".to_string(),
                effect: MutationEffect::TypingBonus { wpm_bonus: 0.0, accuracy_bonus: 0.05 },
                stacks: 1,
            },
            2 => RunMutation {
                name: "Wealthy".to_string(),
                description: "Start with +25% gold".to_string(),
                effect: MutationEffect::ResourceBonus { gold_percent: 0.25, xp_percent: 0.0 },
                stacks: 1,
            },
            3 => RunMutation {
                name: "Studious".to_string(),
                description: "Gain +15% XP".to_string(),
                effect: MutationEffect::ResourceBonus { gold_percent: 0.0, xp_percent: 0.15 },
                stacks: 1,
            },
            4 => RunMutation {
                name: "Battle Hardened".to_string(),
                description: "+5 damage in combat".to_string(),
                effect: MutationEffect::CombatBonus { damage: 5, defense: 0 },
                stacks: 1,
            },
            5 => RunMutation {
                name: "Thick Skinned".to_string(),
                description: "+3 defense".to_string(),
                effect: MutationEffect::CombatBonus { damage: 0, defense: 3 },
                stacks: 1,
            },
            6 => RunMutation {
                name: "Hardy".to_string(),
                description: "+10 max HP".to_string(),
                effect: MutationEffect::StatBonus { stat: "max_hp".to_string(), amount: 10 },
                stacks: 1,
            },
            7 => RunMutation {
                name: "Focused".to_string(),
                description: "+5 max MP".to_string(),
                effect: MutationEffect::StatBonus { stat: "max_mp".to_string(), amount: 5 },
                stacks: 1,
            },
            8 => RunMutation {
                name: "Lucky".to_string(),
                description: "+3 luck".to_string(),
                effect: MutationEffect::StatBonus { stat: "luck".to_string(), amount: 3 },
                stacks: 1,
            },
            _ => RunMutation {
                name: "Baklava Blessing".to_string(),
                description: "A faint smell of honey and pastry follows you.".to_string(),
                effect: MutationEffect::SpecialAbility { ability_id: "baklava_blessing".to_string() },
                stacks: 1,
            },
        };
        
        modifiers.mutations.push(mutation);
    }
    
    modifiers
}

/// Preset modifier combinations for quick selection
pub fn get_preset_modifiers() -> Vec<(&'static str, Vec<(Modifier, u32)>)> {
    vec![
        ("Easy", vec![]),
        ("Normal", vec![
            (Modifier::ToughEnemies { health_multiplier: 1.25 }, 1),
        ]),
        ("Hard", vec![
            (Modifier::ToughEnemies { health_multiplier: 1.5 }, 1),
            (Modifier::DangerousEnemies { damage_multiplier: 1.25 }, 1),
            (Modifier::AccuracyDemand { min_accuracy: 0.85 }, 1),
        ]),
        ("Nightmare", vec![
            (Modifier::ToughEnemies { health_multiplier: 2.0 }, 2),
            (Modifier::DangerousEnemies { damage_multiplier: 1.5 }, 2),
            (Modifier::AccuracyDemand { min_accuracy: 0.90 }, 1),
            (Modifier::MistakeDamage { damage_per_error: 2 }, 1),
            (Modifier::AcceleratedCorruption, 1),
        ]),
        ("Hell", vec![
            (Modifier::ToughEnemies { health_multiplier: 3.0 }, 3),
            (Modifier::DangerousEnemies { damage_multiplier: 2.0 }, 3),
            (Modifier::AccuracyDemand { min_accuracy: 0.95 }, 2),
            (Modifier::MistakeDamage { damage_per_error: 5 }, 2),
            (Modifier::NoBackspace, 1),
            (Modifier::AcceleratedCorruption, 2),
            (Modifier::Permadeath, 1),
        ]),
    ]
}
