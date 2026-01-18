//! Faction Consequences System - Deep faction membership with real stakes
//!
//! This system gives factions TEETH. Joining one should fundamentally
//! change how the game plays. Betrayal should have lasting consequences.
//!
//! Inspired by: Morrowind's Great Houses, Fallout's faction wars, Disco Elysium's political stances

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::narrative::Faction;
use super::items::Item;
use super::spells::Spell;

/// Complete faction relationship state for a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionRelations {
    /// Standing with each faction (-100 to 100)
    pub standings: HashMap<Faction, i32>,
    /// Which faction(s) the player has formally joined
    pub memberships: Vec<FactionMembership>,
    /// Factions that are permanently hostile
    pub blood_enemies: Vec<Faction>,
    /// Active bounties from factions
    pub bounties: Vec<FactionBounty>,
    /// Reputation events (for NPC dialogue reference)
    pub reputation_history: Vec<ReputationEvent>,
}

impl Default for FactionRelations {
    fn default() -> Self {
        Self::new()
    }
}

impl FactionRelations {
    pub fn new() -> Self {
        let mut standings = HashMap::new();
        for faction in &[
            Faction::MagesGuild,
            Faction::TempleOfDawn,
            Faction::RangersOfTheWild,
            Faction::ShadowGuild,
            Faction::MerchantConsortium,
        ] {
            standings.insert(*faction, 0);
        }
        
        Self {
            standings,
            memberships: Vec::new(),
            blood_enemies: Vec::new(),
            bounties: Vec::new(),
            reputation_history: Vec::new(),
        }
    }
    
    /// Get current standing with a faction
    pub fn standing(&self, faction: &Faction) -> i32 {
        *self.standings.get(faction).unwrap_or(&0)
    }
    
    /// Get the status tier for a faction
    pub fn status(&self, faction: &Faction) -> FactionStatus {
        let standing = self.standing(faction);
        if self.blood_enemies.contains(faction) {
            return FactionStatus::BloodEnemy;
        }
        
        match standing {
            -100..=-75 => FactionStatus::Nemesis,
            -74..=-50 => FactionStatus::Hostile,
            -49..=-25 => FactionStatus::Unfriendly,
            -24..=24 => FactionStatus::Neutral,
            25..=49 => FactionStatus::Friendly,
            50..=74 => FactionStatus::Honored,
            75..=89 => FactionStatus::Revered,
            90..=100 => FactionStatus::Exalted,
            _ => FactionStatus::Neutral,
        }
    }
    
    /// Modify standing with a faction (and handle ripple effects)
    pub fn modify_standing(&mut self, faction: Faction, change: i32) {
        let current = self.standings.entry(faction).or_insert(0);
        let old_value = *current;
        *current = (*current + change).clamp(-100, 100);
        let new_value = *current;
        
        // Record the event
        self.reputation_history.push(ReputationEvent {
            faction,
            old_standing: old_value,
            new_standing: new_value,
            cause: if change > 0 { "positive action".to_string() } else { "negative action".to_string() },
        });
        
        // Apply inter-faction relations (helping one faction may hurt another)
        let relations = get_interfaction_relations();
        if let Some(faction_relations) = relations.get(&faction) {
            for (other_faction, modifier) in faction_relations {
                if *other_faction != faction {
                    let ripple = (change as f32 * modifier) as i32;
                    if ripple != 0 {
                        let other_current = self.standings.entry(*other_faction).or_insert(0);
                        *other_current = (*other_current + ripple).clamp(-100, 100);
                    }
                }
            }
        }
        
        // Check for blood enemy status
        if new_value <= -90 && !self.blood_enemies.contains(&faction) {
            self.blood_enemies.push(faction);
            self.bounties.push(FactionBounty {
                faction,
                amount: 1000,
                reason: "Extreme crimes against the faction".to_string(),
                hunters_sent: 0,
            });
        }
    }
    
    /// Join a faction formally
    pub fn join_faction(&mut self, faction: Faction) -> Result<JoinResult, &'static str> {
        // Check if already a member
        if self.memberships.iter().any(|m| m.faction == faction) {
            return Err("Already a member of this faction");
        }
        
        // Check standing requirement
        if self.standing(&faction) < 25 {
            return Err("Insufficient reputation to join this faction");
        }
        
        // Check for enemy factions
        let enemies = get_enemy_factions(&faction);
        for enemy in &enemies {
            if self.memberships.iter().any(|m| m.faction == *enemy) {
                return Err("Cannot join - you belong to an enemy faction");
            }
        }
        
        // Add membership
        self.memberships.push(FactionMembership {
            faction,
            rank: FactionRank::Initiate,
            join_date: 0, // Set by game state
            quests_completed: 0,
            betrayed: false,
        });
        
        // Joining improves standing further
        self.modify_standing(faction, 15);
        
        Ok(JoinResult {
            faction,
            initial_benefits: get_faction_benefits(&faction, FactionRank::Initiate),
            enemies_made: enemies,
        })
    }
    
    /// Betray a faction (dramatic consequences)
    pub fn betray_faction(&mut self, faction: Faction) -> BetrayalResult {
        // Find and mark membership as betrayed
        if let Some(membership) = self.memberships.iter_mut().find(|m| m.faction == faction) {
            membership.betrayed = true;
        }
        
        // Remove from memberships
        self.memberships.retain(|m| m.faction != faction);
        
        // Massive reputation hit
        self.modify_standing(faction, -80);
        
        // Become blood enemy
        if !self.blood_enemies.contains(&faction) {
            self.blood_enemies.push(faction);
        }
        
        // Add bounty
        self.bounties.push(FactionBounty {
            faction,
            amount: 5000,
            reason: "Betrayal of sacred trust".to_string(),
            hunters_sent: 0,
        });
        
        // Some factions react positively to betrayal of their enemies
        let enemies = get_enemy_factions(&faction);
        for enemy in &enemies {
            self.modify_standing(*enemy, 20);
        }
        
        BetrayalResult {
            faction,
            new_standing: self.standing(&faction),
            bounty_placed: 5000,
            factions_pleased: enemies,
        }
    }
    
    /// Get current rank in a faction
    pub fn rank_in(&self, faction: &Faction) -> Option<FactionRank> {
        self.memberships.iter()
            .find(|m| m.faction == *faction)
            .map(|m| m.rank)
    }
    
    /// Check if player is a member of faction
    pub fn is_member_of(&self, faction: &Faction) -> bool {
        self.memberships.iter().any(|m| m.faction == *faction)
    }
    
    /// Get all current benefits from faction memberships
    pub fn current_benefits(&self) -> Vec<FactionBenefit> {
        self.memberships.iter()
            .flat_map(|m| get_faction_benefits(&m.faction, m.rank))
            .collect()
    }
    
    /// Get all current penalties from negative standings
    pub fn current_penalties(&self) -> Vec<FactionPenalty> {
        self.standings.iter()
            .filter(|(_, &standing)| standing < -25)
            .flat_map(|(faction, &standing)| get_faction_penalties(faction, standing))
            .collect()
    }
    
    /// Check if bounty hunters should spawn
    pub fn should_spawn_hunters(&self) -> Option<&FactionBounty> {
        self.bounties.iter()
            .find(|b| b.hunters_sent < 5) // Max 5 hunter squads per bounty
    }
    
    /// Record that hunters were sent
    pub fn record_hunters_sent(&mut self, faction: &Faction) {
        if let Some(bounty) = self.bounties.iter_mut().find(|b| b.faction == *faction) {
            bounty.hunters_sent += 1;
        }
    }
    
    /// Modify standing with all factions slightly (for general deeds)
    pub fn modify_all_standings(&mut self, change: i32, _reason: &str) {
        let factions = vec![
            Faction::MagesGuild,
            Faction::TempleOfDawn,
            Faction::RangersOfTheWild,
            Faction::ShadowGuild,
            Faction::MerchantConsortium,
        ];
        for faction in factions {
            let current = self.standings.entry(faction).or_insert(0);
            *current = (*current + change).clamp(-100, 100);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionStatus {
    BloodEnemy, // Beyond hostile - permanent
    Nemesis,    // -100 to -75
    Hostile,    // -74 to -50
    Unfriendly, // -49 to -25
    Neutral,    // -24 to 24
    Friendly,   // 25 to 49
    Honored,    // 50 to 74
    Revered,    // 75 to 89
    Exalted,    // 90 to 100
}

impl FactionStatus {
    pub fn description(&self) -> &'static str {
        match self {
            Self::BloodEnemy => "Your name is a curse among them. They will never forgive.",
            Self::Nemesis => "You are their greatest enemy. Kill on sight.",
            Self::Hostile => "They attack you openly. No quarter given.",
            Self::Unfriendly => "They distrust you deeply. Many doors are closed.",
            Self::Neutral => "They neither know nor care about you.",
            Self::Friendly => "You've earned some respect. Basic services available.",
            Self::Honored => "You're well-regarded. Special privileges unlocked.",
            Self::Revered => "You're a hero to them. Inner circle access.",
            Self::Exalted => "You are legend. Their highest honors are yours.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionRank {
    Initiate,
    Member,
    Trusted,
    Veteran,
    Elite,
    InnerCircle,
    Champion,
}

impl FactionRank {
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::Initiate => Some(Self::Member),
            Self::Member => Some(Self::Trusted),
            Self::Trusted => Some(Self::Veteran),
            Self::Veteran => Some(Self::Elite),
            Self::Elite => Some(Self::InnerCircle),
            Self::InnerCircle => Some(Self::Champion),
            Self::Champion => None,
        }
    }
    
    pub fn title(&self, faction: &Faction) -> &'static str {
        match (faction, self) {
            (Faction::MagesGuild, Self::Initiate) => "Novice Scribe",
            (Faction::MagesGuild, Self::Member) => "Scribe",
            (Faction::MagesGuild, Self::Trusted) => "Senior Scribe",
            (Faction::MagesGuild, Self::Veteran) => "Master Scribe",
            (Faction::MagesGuild, Self::Elite) => "Arch-Scribe",
            (Faction::MagesGuild, Self::InnerCircle) => "Keeper of Words",
            (Faction::MagesGuild, Self::Champion) => "Voice of the Eternal Word",
            
            (Faction::TempleOfDawn, Self::Initiate) => "Input Device",
            (Faction::TempleOfDawn, Self::Member) => "Processor",
            (Faction::TempleOfDawn, Self::Trusted) => "Subroutine",
            (Faction::TempleOfDawn, Self::Veteran) => "Function",
            (Faction::TempleOfDawn, Self::Elite) => "Module",
            (Faction::TempleOfDawn, Self::InnerCircle) => "Core Process",
            (Faction::TempleOfDawn, Self::Champion) => "Prime Algorithm",
            
            (Faction::RangersOfTheWild, Self::Initiate) => "Seedling",
            (Faction::RangersOfTheWild, Self::Member) => "Sapling",
            (Faction::RangersOfTheWild, Self::Trusted) => "Branch",
            (Faction::RangersOfTheWild, Self::Veteran) => "Tree",
            (Faction::RangersOfTheWild, Self::Elite) => "Grove Keeper",
            (Faction::RangersOfTheWild, Self::InnerCircle) => "Forest Voice",
            (Faction::RangersOfTheWild, Self::Champion) => "Ancient Oak",
            
            (Faction::ShadowGuild, Self::Initiate) => "Whisper",
            (Faction::ShadowGuild, Self::Member) => "Shadow",
            (Faction::ShadowGuild, Self::Trusted) => "Cipher",
            (Faction::ShadowGuild, Self::Veteran) => "Ghost Writer",
            (Faction::ShadowGuild, Self::Elite) => "Phantom",
            (Faction::ShadowGuild, Self::InnerCircle) => "Specter",
            (Faction::ShadowGuild, Self::Champion) => "The Unseen Pen",
            
            (Faction::MerchantConsortium, Self::Initiate) => "Page",
            (Faction::MerchantConsortium, Self::Member) => "Cataloger",
            (Faction::MerchantConsortium, Self::Trusted) => "Librarian",
            (Faction::MerchantConsortium, Self::Veteran) => "Curator",
            (Faction::MerchantConsortium, Self::Elite) => "Archivist",
            (Faction::MerchantConsortium, Self::InnerCircle) => "Lorekeeper",
            (Faction::MerchantConsortium, Self::Champion) => "Grand Chronicler",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionMembership {
    pub faction: Faction,
    pub rank: FactionRank,
    pub join_date: i32, // In-game days
    pub quests_completed: u32,
    pub betrayed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionBounty {
    pub faction: Faction,
    pub amount: i32,
    pub reason: String,
    pub hunters_sent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub faction: Faction,
    pub old_standing: i32,
    pub new_standing: i32,
    pub cause: String,
}

#[derive(Debug, Clone)]
pub struct JoinResult {
    pub faction: Faction,
    pub initial_benefits: Vec<FactionBenefit>,
    pub enemies_made: Vec<Faction>,
}

#[derive(Debug, Clone)]
pub struct BetrayalResult {
    pub faction: Faction,
    pub new_standing: i32,
    pub bounty_placed: i32,
    pub factions_pleased: Vec<Faction>,
}

/// Benefits granted by faction membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionBenefit {
    /// Discount at faction shops
    ShopDiscount { percent: f32 },
    /// Access to faction training grounds
    TrainingAccess { skill_tree: String },
    /// Exclusive spell only faction teaches
    ExclusiveSpell { spell_name: String },
    /// Can enter faction territory safely
    SafePassage { regions: Vec<String> },
    /// Faction NPCs share information
    IntelAccess,
    /// Faction provides backup in fights
    CombatSupport { ally_strength: u32 },
    /// Unique item only faction provides
    UniqueItem { item_name: String },
    /// Passive stat bonuses
    StatBonus { stat: String, amount: i32 },
    /// Reduced typing time requirements
    TypingBonus { time_extension: f32 },
    /// Access to faction-specific quests
    QuestAccess,
    /// Healing/rest at faction locations
    RestAccess,
    /// Faction warns of dangers
    DangerWarnings,
}

impl FactionBenefit {
    pub fn description(&self) -> String {
        match self {
            Self::ShopDiscount { percent } => format!("{}% discount at faction shops", (percent * 100.0) as i32),
            Self::TrainingAccess { skill_tree } => format!("Access to {} training", skill_tree),
            Self::ExclusiveSpell { spell_name } => format!("Can learn: {}", spell_name),
            Self::SafePassage { regions } => format!("Safe passage through: {}", regions.join(", ")),
            Self::IntelAccess => "Faction NPCs share valuable information".to_string(),
            Self::CombatSupport { ally_strength } => format!("Faction allies (power {}) may join battles", ally_strength),
            Self::UniqueItem { item_name } => format!("Access to: {}", item_name),
            Self::StatBonus { stat, amount } => format!("+{} {}", amount, stat),
            Self::TypingBonus { time_extension } => format!("+{:.1}s on typing challenges", time_extension),
            Self::QuestAccess => "Access to faction questline".to_string(),
            Self::RestAccess => "Can rest at faction locations".to_string(),
            Self::DangerWarnings => "Faction warns of nearby threats".to_string(),
        }
    }
}

/// Penalties from negative faction standing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionPenalty {
    /// Attacked on sight in faction territory
    AttackOnSight { regions: Vec<String> },
    /// Prices increased dramatically
    PriceGouging { multiplier: f32 },
    /// Can't enter faction territory
    TerritoryBan { regions: Vec<String> },
    /// Faction sends hunters
    BountyHunters { frequency: f32, difficulty: u32 },
    /// NPCs refuse to talk
    SocialPariah,
    /// Faction sabotages you
    ActiveSabotage,
    /// Reduced typing time (pressure!)
    TypingPenalty { time_reduction: f32 },
    /// Faction spreads rumors
    ReputationDamage { to_faction: Faction, amount: i32 },
}

impl FactionPenalty {
    pub fn description(&self) -> String {
        match self {
            Self::AttackOnSight { regions } => format!("Attacked on sight in: {}", regions.join(", ")),
            Self::PriceGouging { multiplier } => format!("Prices increased {}x", multiplier),
            Self::TerritoryBan { regions } => format!("Banned from: {}", regions.join(", ")),
            Self::BountyHunters { difficulty, .. } => format!("Bounty hunters (difficulty {}) track you", difficulty),
            Self::SocialPariah => "Faction members refuse to speak with you".to_string(),
            Self::ActiveSabotage => "Faction actively works against you".to_string(),
            Self::TypingPenalty { time_reduction } => format!("-{:.1}s on typing challenges near faction", time_reduction),
            Self::ReputationDamage { to_faction, amount } => 
                format!("Faction spreading lies to {} (-{} rep)", to_faction.name(), amount),
        }
    }
}

/// Get benefits for a faction at a given rank
pub fn get_faction_benefits(faction: &Faction, rank: FactionRank) -> Vec<FactionBenefit> {
    let mut benefits = Vec::new();
    
    // Base benefits for all members
    benefits.push(FactionBenefit::QuestAccess);
    benefits.push(FactionBenefit::RestAccess);
    
    match faction {
        Faction::MagesGuild => {
            benefits.push(FactionBenefit::SafePassage { 
                regions: vec!["haven".to_string(), "athenaeum".to_string()] 
            });
            benefits.push(FactionBenefit::StatBonus { 
                stat: "accuracy".to_string(), 
                amount: 5 
            });
            
            if rank as u8 >= FactionRank::Trusted as u8 {
                benefits.push(FactionBenefit::ExclusiveSpell { 
                    spell_name: "Word of Power".to_string() 
                });
                benefits.push(FactionBenefit::ShopDiscount { percent: 0.15 });
            }
            if rank as u8 >= FactionRank::Elite as u8 {
                benefits.push(FactionBenefit::TrainingAccess { 
                    skill_tree: "Precision".to_string() 
                });
                benefits.push(FactionBenefit::IntelAccess);
            }
            if rank as u8 >= FactionRank::InnerCircle as u8 {
                benefits.push(FactionBenefit::UniqueItem { 
                    item_name: "Scribe's Sacred Quill".to_string() 
                });
            }
        }
        
        Faction::TempleOfDawn => {
            benefits.push(FactionBenefit::SafePassage { 
                regions: vec!["mechanist_fortress".to_string()] 
            });
            benefits.push(FactionBenefit::TypingBonus { time_extension: 0.5 });
            
            if rank as u8 >= FactionRank::Trusted as u8 {
                benefits.push(FactionBenefit::ExclusiveSpell { 
                    spell_name: "Overclock".to_string() 
                });
                benefits.push(FactionBenefit::StatBonus { 
                    stat: "speed".to_string(), 
                    amount: 10 
                });
            }
            if rank as u8 >= FactionRank::Elite as u8 {
                benefits.push(FactionBenefit::TrainingAccess { 
                    skill_tree: "Speed".to_string() 
                });
                benefits.push(FactionBenefit::CombatSupport { ally_strength: 3 });
            }
            if rank as u8 >= FactionRank::InnerCircle as u8 {
                benefits.push(FactionBenefit::UniqueItem { 
                    item_name: "Mechanical Keyboard of Precision".to_string() 
                });
            }
        }
        
        Faction::RangersOfTheWild => {
            benefits.push(FactionBenefit::SafePassage { 
                regions: vec!["sacred_grove".to_string()] 
            });
            benefits.push(FactionBenefit::StatBonus { 
                stat: "hp_regen".to_string(), 
                amount: 2 
            });
            
            if rank as u8 >= FactionRank::Trusted as u8 {
                benefits.push(FactionBenefit::ExclusiveSpell { 
                    spell_name: "Natural Flow".to_string() 
                });
                benefits.push(FactionBenefit::DangerWarnings);
            }
            if rank as u8 >= FactionRank::Elite as u8 {
                benefits.push(FactionBenefit::TrainingAccess { 
                    skill_tree: "Endurance".to_string() 
                });
                benefits.push(FactionBenefit::RestAccess);
            }
            if rank as u8 >= FactionRank::InnerCircle as u8 {
                benefits.push(FactionBenefit::UniqueItem { 
                    item_name: "Living Wood Keyboard".to_string() 
                });
            }
        }
        
        Faction::ShadowGuild => {
            benefits.push(FactionBenefit::SafePassage { 
                regions: vec!["shadow_quarter".to_string()] 
            });
            benefits.push(FactionBenefit::IntelAccess);
            
            if rank as u8 >= FactionRank::Trusted as u8 {
                benefits.push(FactionBenefit::ExclusiveSpell { 
                    spell_name: "Cipher Strike".to_string() 
                });
                benefits.push(FactionBenefit::ShopDiscount { percent: 0.20 }); // Black market
            }
            if rank as u8 >= FactionRank::Elite as u8 {
                benefits.push(FactionBenefit::TrainingAccess { 
                    skill_tree: "Shadow".to_string() 
                });
                benefits.push(FactionBenefit::DangerWarnings);
            }
            if rank as u8 >= FactionRank::InnerCircle as u8 {
                benefits.push(FactionBenefit::UniqueItem { 
                    item_name: "Invisible Ink Set".to_string() 
                });
            }
        }
        
        Faction::MerchantConsortium => {
            benefits.push(FactionBenefit::SafePassage { 
                regions: vec!["athenaeum".to_string(), "first_library".to_string()] 
            });
            benefits.push(FactionBenefit::IntelAccess);
            
            if rank as u8 >= FactionRank::Trusted as u8 {
                benefits.push(FactionBenefit::ExclusiveSpell { 
                    spell_name: "Recall".to_string() 
                });
                benefits.push(FactionBenefit::TrainingAccess { 
                    skill_tree: "Wisdom".to_string() 
                });
            }
            if rank as u8 >= FactionRank::Elite as u8 {
                benefits.push(FactionBenefit::ShopDiscount { percent: 0.10 });
                benefits.push(FactionBenefit::StatBonus { 
                    stat: "xp_gain".to_string(), 
                    amount: 15 
                });
            }
            if rank as u8 >= FactionRank::InnerCircle as u8 {
                benefits.push(FactionBenefit::UniqueItem { 
                    item_name: "Chronicler's Codex".to_string() 
                });
            }
        }
    }
    
    benefits
}

/// Get penalties for negative standing with a faction
pub fn get_faction_penalties(faction: &Faction, standing: i32) -> Vec<FactionPenalty> {
    let mut penalties = Vec::new();
    
    // Mild penalties
    if standing < -25 {
        penalties.push(FactionPenalty::PriceGouging { multiplier: 1.5 });
        penalties.push(FactionPenalty::SocialPariah);
    }
    
    // Moderate penalties
    if standing < -50 {
        penalties.push(FactionPenalty::TerritoryBan { 
            regions: get_faction_regions(faction) 
        });
        penalties.push(FactionPenalty::PriceGouging { multiplier: 2.0 });
    }
    
    // Severe penalties
    if standing < -75 {
        penalties.push(FactionPenalty::AttackOnSight { 
            regions: get_faction_regions(faction) 
        });
        penalties.push(FactionPenalty::BountyHunters { 
            frequency: 0.15, 
            difficulty: 3 
        });
        penalties.push(FactionPenalty::ActiveSabotage);
    }
    
    // Blood enemy penalties
    if standing <= -90 {
        penalties.push(FactionPenalty::BountyHunters { 
            frequency: 0.30, 
            difficulty: 5 
        });
        
        // Faction tries to damage reputation with others
        for other in get_allied_factions(faction) {
            penalties.push(FactionPenalty::ReputationDamage { 
                to_faction: other, 
                amount: 5 
            });
        }
    }
    
    penalties
}

/// Get regions controlled by a faction
pub fn get_faction_regions(faction: &Faction) -> Vec<String> {
    match faction {
        Faction::MagesGuild => vec!["haven".to_string()],
        Faction::TempleOfDawn => vec!["mechanist_fortress".to_string()],
        Faction::RangersOfTheWild => vec!["sacred_grove".to_string()],
        Faction::ShadowGuild => vec!["shadow_quarter".to_string()],
        Faction::MerchantConsortium => vec!["athenaeum".to_string(), "first_library".to_string()],
    }
}

/// Get factions that are enemies of this one
pub fn get_enemy_factions(faction: &Faction) -> Vec<Faction> {
    match faction {
        Faction::MagesGuild => vec![Faction::TempleOfDawn], // Traditional vs Modern
        Faction::TempleOfDawn => vec![Faction::MagesGuild, Faction::RangersOfTheWild], // Efficiency vs others
        Faction::RangersOfTheWild => vec![Faction::TempleOfDawn], // Organic vs Mechanical
        Faction::ShadowGuild => vec![], // Enemies with none officially (underground)
        Faction::MerchantConsortium => vec![], // Neutral to all
    }
}

/// Get factions allied to this one
pub fn get_allied_factions(faction: &Faction) -> Vec<Faction> {
    match faction {
        Faction::MagesGuild => vec![Faction::RangersOfTheWild], // Both value tradition
        Faction::TempleOfDawn => vec![], // Too cold for allies
        Faction::RangersOfTheWild => vec![Faction::MagesGuild],
        Faction::ShadowGuild => vec![], // Trust no one
        Faction::MerchantConsortium => vec![], // Neutral
    }
}

/// Get how factions react to actions involving other factions
/// Returns: HashMap<Faction, HashMap<OtherFaction, reaction_multiplier>>
/// positive multiplier = helping faction helps standing with other
/// negative multiplier = helping faction hurts standing with other
pub fn get_interfaction_relations() -> HashMap<Faction, HashMap<Faction, f32>> {
    let mut relations = HashMap::new();
    
    // Scribes relations
    let mut scribe_relations = HashMap::new();
    scribe_relations.insert(Faction::TempleOfDawn, -0.3); // Helping scribes hurts mechanist rep
    scribe_relations.insert(Faction::RangersOfTheWild, 0.2);  // Helping scribes helps naturalist rep
    scribe_relations.insert(Faction::MerchantConsortium, 0.1);
    scribe_relations.insert(Faction::ShadowGuild, 0.0);
    relations.insert(Faction::MagesGuild, scribe_relations);
    
    // Mechanists relations
    let mut mech_relations = HashMap::new();
    mech_relations.insert(Faction::MagesGuild, -0.3);
    mech_relations.insert(Faction::RangersOfTheWild, -0.4); // Strong opposition
    mech_relations.insert(Faction::MerchantConsortium, 0.0);
    mech_relations.insert(Faction::ShadowGuild, -0.1);
    relations.insert(Faction::TempleOfDawn, mech_relations);
    
    // Naturalists relations  
    let mut nat_relations = HashMap::new();
    nat_relations.insert(Faction::MagesGuild, 0.2);
    nat_relations.insert(Faction::TempleOfDawn, -0.4);
    nat_relations.insert(Faction::MerchantConsortium, 0.1);
    nat_relations.insert(Faction::ShadowGuild, 0.1);
    relations.insert(Faction::RangersOfTheWild, nat_relations);
    
    // Shadow Writers relations
    let mut shadow_relations = HashMap::new();
    shadow_relations.insert(Faction::MagesGuild, 0.0);
    shadow_relations.insert(Faction::TempleOfDawn, -0.1);
    shadow_relations.insert(Faction::RangersOfTheWild, 0.1);
    shadow_relations.insert(Faction::MerchantConsortium, 0.0);
    relations.insert(Faction::ShadowGuild, shadow_relations);
    
    // Archivists relations - mostly neutral
    let mut arch_relations = HashMap::new();
    arch_relations.insert(Faction::MagesGuild, 0.1);
    arch_relations.insert(Faction::TempleOfDawn, 0.0);
    arch_relations.insert(Faction::RangersOfTheWild, 0.1);
    arch_relations.insert(Faction::ShadowGuild, 0.0);
    relations.insert(Faction::MerchantConsortium, arch_relations);
    
    relations
}

/// Special faction-exclusive spells
pub fn get_faction_spell(faction: &Faction, rank: FactionRank) -> Option<Spell> {
    use super::spells::{SpellEffect, SpellElement, SpellTarget};
    
    match (faction, rank) {
        (Faction::MagesGuild, FactionRank::Trusted) => Some(Spell {
            name: "Word of Power".to_string(),
            description: "Speak a word of ancient power. Damage scales with word length.".to_string(),
            element: SpellElement::Arcane,
            target: SpellTarget::Enemy,
            mp_cost: 15,
            base_power: 25,
            incantation: "AUTHORITY".to_string(),
            cast_time: 8.0,
            effect: SpellEffect::Damage(25),
        }),
        
        (Faction::TempleOfDawn, FactionRank::Trusted) => Some(Spell {
            name: "Overclock".to_string(),
            description: "Push beyond limits. +50% typing speed for one challenge.".to_string(),
            element: SpellElement::Lightning,
            target: SpellTarget::Self_,
            mp_cost: 20,
            base_power: 0,
            incantation: "ACCELERATE".to_string(),
            cast_time: 5.0,
            effect: SpellEffect::Buff { stat: "speed".to_string(), amount: 50, duration: 1 },
        }),
        
        (Faction::RangersOfTheWild, FactionRank::Trusted) => Some(Spell {
            name: "Natural Flow".to_string(),
            description: "Enter a state of flow. Typos don't break combo.".to_string(),
            element: SpellElement::Nature,
            target: SpellTarget::Self_,
            mp_cost: 12,
            base_power: 0,
            incantation: "breathe".to_string(),
            cast_time: 6.0,
            effect: SpellEffect::Buff { stat: "flow".to_string(), amount: 100, duration: 3 },
        }),
        
        (Faction::ShadowGuild, FactionRank::Trusted) => Some(Spell {
            name: "Cipher Strike".to_string(),
            description: "Attack with encrypted words. Enemy can't predict your next move.".to_string(),
            element: SpellElement::Dark,
            target: SpellTarget::Enemy,
            mp_cost: 18,
            base_power: 30,
            incantation: "ENCRYPTED".to_string(),
            cast_time: 7.0,
            effect: SpellEffect::Multi { hits: 3, damage_per_hit: 10 },
        }),
        
        (Faction::MerchantConsortium, FactionRank::Trusted) => Some(Spell {
            name: "Recall".to_string(),
            description: "Remember perfectly. See the next word before it appears.".to_string(),
            element: SpellElement::Arcane,
            target: SpellTarget::Self_,
            mp_cost: 10,
            base_power: 0,
            incantation: "remember".to_string(),
            cast_time: 5.0,
            effect: SpellEffect::Buff { stat: "foresight".to_string(), amount: 100, duration: 5 },
        }),
        
        _ => None,
    }
}
