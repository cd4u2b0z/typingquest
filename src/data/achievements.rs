//! Achievement system - Unlockable rewards and milestones
//!
//! Tracks player accomplishments across runs with permanent rewards.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database of all achievements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchievementDatabase {
    pub achievements: HashMap<String, Achievement>,
}

/// An unlockable achievement with reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hint: String,
    pub category: AchievementCategory,
    pub tier: AchievementTier,
    pub requirement: AchievementRequirement,
    pub reward: AchievementReward,
    pub icon: char,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AchievementCategory {
    Typing,
    Combat,
    Exploration,
    Collection,
    Challenge,
    Story,
    Meta,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Secret,
}

impl AchievementTier {
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            AchievementTier::Bronze => (205, 127, 50),
            AchievementTier::Silver => (192, 192, 192),
            AchievementTier::Gold => (255, 215, 0),
            AchievementTier::Platinum => (229, 228, 226),
            AchievementTier::Secret => (138, 43, 226),
        }
    }
    
    pub fn symbol(&self) -> char {
        match self {
            AchievementTier::Bronze => '◉',
            AchievementTier::Silver => '◈',
            AchievementTier::Gold => '★',
            AchievementTier::Platinum => '✦',
            AchievementTier::Secret => '✧',
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRequirement {
    // Typing achievements
    WordsTyped(u64),
    PerfectWords(u64),
    TotalKeystrokes(u64),
    WpmReached(u32),
    WpmSustained { wpm: u32, words: u32 },
    AccuracyRun { min_percent: f32 },
    ComboReached(u32),
    ComboSustained { combo: u32, seconds: f32 },
    NoMistakesWords(u32),
    
    // Combat achievements
    EnemiesDefeated(u32),
    BossesDefeated(u32),
    SpecificBossDefeated(String),
    FlawlessVictories(u32),
    SpellsCast(u32),
    CriticalHits(u32),
    DamageDealt(u64),
    DamageTaken(u64),
    HealingDone(u64),
    CombatsWonInRow(u32),
    
    // Exploration achievements
    FloorsReached(u32),
    ZonesCleared(u32),
    RoomsExplored(u32),
    SecretsFound(u32),
    
    // Collection achievements
    ItemsCollected(u32),
    RelicsCollected(u32),
    SpellsLearned(u32),
    GoldEarned(u64),
    GoldSpent(u64),
    
    // Challenge achievements
    SpeedrunFloor { floor: u32, max_seconds: f32 },
    NoDamageBoss(String),
    LowLevelBoss { boss: String, max_level: u32 },
    NoItemsUsed { floors: u32 },
    
    // Story achievements
    LoreDiscovered(u32),
    DialoguesCompleted(u32),
    FactionMaxRep(String),
    EndingReached(String),
    
    // Meta achievements
    RunsCompleted(u32),
    TotalPlaytime { hours: u32 },
    DeathCount(u32),
    AchievementsUnlocked(u32),
    AllInCategory(AchievementCategory),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementReward {
    Cosmetic(CosmeticReward),
    Unlock(UnlockReward),
    StatBonus(StatBonusReward),
    StartingItem(String),
    Title(String),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmeticReward {
    pub reward_type: CosmeticType,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CosmeticType {
    BorderStyle,
    TypewriterSound,
    CombatEffect,
    MenuTheme,
    WordHighlight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockReward {
    pub unlock_type: UnlockType,
    pub id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnlockType {
    Class,
    GameMode,
    Difficulty,
    StartingSpell,
    ShopItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBonusReward {
    pub stat: String,
    pub amount: f32,
    pub permanent: bool,
}

impl AchievementDatabase {
    pub fn embedded() -> Self {
        let mut achievements = HashMap::new();

        // ═══════════════════════════════════════════════════════════════
        // TYPING ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("first_words".into(), Achievement {
            id: "first_words".into(),
            name: "First Words".into(),
            description: "Type your first 100 words.".into(),
            hint: "Just keep typing!".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::WordsTyped(100),
            reward: AchievementReward::None,
            icon: '📝',
            hidden: false,
        });

        achievements.insert("wordsmith_apprentice".into(), Achievement {
            id: "wordsmith_apprentice".into(),
            name: "Wordsmith Apprentice".into(),
            description: "Type 1,000 words total.".into(),
            hint: "Practice makes perfect.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::WordsTyped(1_000),
            reward: AchievementReward::Title("Apprentice".into()),
            icon: '✏',
            hidden: false,
        });

        achievements.insert("wordsmith_journeyman".into(), Achievement {
            id: "wordsmith_journeyman".into(),
            name: "Wordsmith Journeyman".into(),
            description: "Type 10,000 words total.".into(),
            hint: "Dedication pays off.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::WordsTyped(10_000),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "typing_damage".into(),
                amount: 5.0,
                permanent: true,
            }),
            icon: '✒',
            hidden: false,
        });

        achievements.insert("wordsmith_master".into(), Achievement {
            id: "wordsmith_master".into(),
            name: "Wordsmith Master".into(),
            description: "Type 100,000 words total.".into(),
            hint: "You've written a novel's worth!".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::WordsTyped(100_000),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::Class,
                id: "grand_scribe".into(),
            }),
            icon: '📜',
            hidden: false,
        });

        achievements.insert("speed_demon".into(), Achievement {
            id: "speed_demon".into(),
            name: "Speed Demon".into(),
            description: "Reach 80 WPM in combat.".into(),
            hint: "Type faster!".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::WpmReached(80),
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::CombatEffect,
                id: "flame_trail".into(),
                name: "Flame Trail".into(),
            }),
            icon: '🔥',
            hidden: false,
        });

        achievements.insert("lightning_fingers".into(), Achievement {
            id: "lightning_fingers".into(),
            name: "Lightning Fingers".into(),
            description: "Reach 100 WPM in combat.".into(),
            hint: "Faster than thought.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::WpmReached(100),
            reward: AchievementReward::StartingItem("spark_gloves".into()),
            icon: '⚡',
            hidden: false,
        });

        achievements.insert("transcendent_typist".into(), Achievement {
            id: "transcendent_typist".into(),
            name: "Transcendent Typist".into(),
            description: "Reach 150 WPM in combat.".into(),
            hint: "Beyond mortal limits.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Platinum,
            requirement: AchievementRequirement::WpmReached(150),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::GameMode,
                id: "transcendence_mode".into(),
            }),
            icon: '✦',
            hidden: false,
        });

        achievements.insert("perfectionist".into(), Achievement {
            id: "perfectionist".into(),
            name: "Perfectionist".into(),
            description: "Type 100 words with no mistakes.".into(),
            hint: "Accuracy is key.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::NoMistakesWords(100),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "accuracy_bonus".into(),
                amount: 3.0,
                permanent: true,
            }),
            icon: '✓',
            hidden: false,
        });

        achievements.insert("flawless_run".into(), Achievement {
            id: "flawless_run".into(),
            name: "Flawless Run".into(),
            description: "Complete an entire run with 99%+ accuracy.".into(),
            hint: "Not a single wasted keystroke.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::AccuracyRun { min_percent: 99.0 },
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::WordHighlight,
                id: "golden_glow".into(),
                name: "Golden Glow".into(),
            }),
            icon: '💯',
            hidden: false,
        });

        achievements.insert("combo_starter".into(), Achievement {
            id: "combo_starter".into(),
            name: "Combo Starter".into(),
            description: "Reach a 10-word combo.".into(),
            hint: "Chain those perfect words!".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::ComboReached(10),
            reward: AchievementReward::None,
            icon: '🔗',
            hidden: false,
        });

        achievements.insert("combo_master".into(), Achievement {
            id: "combo_master".into(),
            name: "Combo Master".into(),
            description: "Reach a 50-word combo.".into(),
            hint: "The rhythm of battle.".into(),
            category: AchievementCategory::Typing,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::ComboReached(50),
            reward: AchievementReward::StartingItem("combo_crown".into()),
            icon: '👑',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // COMBAT ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("first_blood".into(), Achievement {
            id: "first_blood".into(),
            name: "First Blood".into(),
            description: "Defeat your first enemy.".into(),
            hint: "Every journey begins with a single step.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::EnemiesDefeated(1),
            reward: AchievementReward::None,
            icon: '⚔',
            hidden: false,
        });

        achievements.insert("monster_slayer".into(), Achievement {
            id: "monster_slayer".into(),
            name: "Monster Slayer".into(),
            description: "Defeat 100 enemies.".into(),
            hint: "Building a reputation.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::EnemiesDefeated(100),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "damage".into(),
                amount: 5.0,
                permanent: true,
            }),
            icon: '🗡',
            hidden: false,
        });

        achievements.insert("legend".into(), Achievement {
            id: "legend".into(),
            name: "Legend".into(),
            description: "Defeat 1,000 enemies.".into(),
            hint: "Tales are told of your deeds.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::EnemiesDefeated(1000),
            reward: AchievementReward::Title("Legend".into()),
            icon: '🏆',
            hidden: false,
        });

        achievements.insert("boss_hunter".into(), Achievement {
            id: "boss_hunter".into(),
            name: "Boss Hunter".into(),
            description: "Defeat 10 bosses.".into(),
            hint: "The bigger they are...".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::BossesDefeated(10),
            reward: AchievementReward::StartingItem("boss_bane".into()),
            icon: '👹',
            hidden: false,
        });

        achievements.insert("librarian_slayer".into(), Achievement {
            id: "librarian_slayer".into(),
            name: "Silencer of Stories".into(),
            description: "Defeat the Librarian Shade.".into(),
            hint: "The first gatekeeper falls.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::SpecificBossDefeated("librarian_shade".into()),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::StartingSpell,
                id: "silence".into(),
            }),
            icon: '📚',
            hidden: false,
        });

        achievements.insert("void_conqueror".into(), Achievement {
            id: "void_conqueror".into(),
            name: "Void Conqueror".into(),
            description: "Defeat the Void Herald.".into(),
            hint: "Stare into the abyss and win.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::SpecificBossDefeated("void_herald".into()),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::Class,
                id: "void_touched".into(),
            }),
            icon: '🌑',
            hidden: false,
        });

        achievements.insert("untouchable".into(), Achievement {
            id: "untouchable".into(),
            name: "Untouchable".into(),
            description: "Win 5 combats without taking damage.".into(),
            hint: "They can't hit what they can't catch.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::FlawlessVictories(5),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "evasion".into(),
                amount: 5.0,
                permanent: true,
            }),
            icon: '💨',
            hidden: false,
        });

        achievements.insert("glass_cannon".into(), Achievement {
            id: "glass_cannon".into(),
            name: "Glass Cannon".into(),
            description: "Deal 10,000 total damage.".into(),
            hint: "Offense is the best defense.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::DamageDealt(10_000),
            reward: AchievementReward::StartingItem("berserker_quill".into()),
            icon: '💥',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // EXPLORATION ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("delver".into(), Achievement {
            id: "delver".into(),
            name: "Delver".into(),
            description: "Reach floor 5.".into(),
            hint: "The depths await.".into(),
            category: AchievementCategory::Exploration,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::FloorsReached(5),
            reward: AchievementReward::None,
            icon: '🔦',
            hidden: false,
        });

        achievements.insert("deep_explorer".into(), Achievement {
            id: "deep_explorer".into(),
            name: "Deep Explorer".into(),
            description: "Reach floor 15.".into(),
            hint: "Few venture this far.".into(),
            category: AchievementCategory::Exploration,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::FloorsReached(15),
            reward: AchievementReward::StartingItem("explorers_lantern".into()),
            icon: '🗺',
            hidden: false,
        });

        achievements.insert("abyssal_walker".into(), Achievement {
            id: "abyssal_walker".into(),
            name: "Abyssal Walker".into(),
            description: "Reach floor 25.".into(),
            hint: "The void knows your name.".into(),
            category: AchievementCategory::Exploration,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::FloorsReached(25),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::Difficulty,
                id: "nightmare".into(),
            }),
            icon: '🌊',
            hidden: false,
        });

        achievements.insert("genesis_reached".into(), Achievement {
            id: "genesis_reached".into(),
            name: "Genesis Reached".into(),
            description: "Reach floor 30 - The Genesis Archive.".into(),
            hint: "Where it all began.".into(),
            category: AchievementCategory::Exploration,
            tier: AchievementTier::Platinum,
            requirement: AchievementRequirement::FloorsReached(30),
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::MenuTheme,
                id: "genesis_theme".into(),
                name: "Genesis Theme".into(),
            }),
            icon: '✧',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // COLLECTION ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("collector".into(), Achievement {
            id: "collector".into(),
            name: "Collector".into(),
            description: "Collect 25 different items.".into(),
            hint: "Shiny things everywhere!".into(),
            category: AchievementCategory::Collection,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::ItemsCollected(25),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "item_find".into(),
                amount: 10.0,
                permanent: true,
            }),
            icon: '🎒',
            hidden: false,
        });

        achievements.insert("relic_hunter".into(), Achievement {
            id: "relic_hunter".into(),
            name: "Relic Hunter".into(),
            description: "Collect 10 relics.".into(),
            hint: "Ancient power awaits.".into(),
            category: AchievementCategory::Collection,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::RelicsCollected(10),
            reward: AchievementReward::StartingItem("lucky_coin".into()),
            icon: '🏺',
            hidden: false,
        });

        achievements.insert("wealthy".into(), Achievement {
            id: "wealthy".into(),
            name: "Wealthy".into(),
            description: "Earn 10,000 gold total.".into(),
            hint: "Money makes the world go round.".into(),
            category: AchievementCategory::Collection,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::GoldEarned(10_000),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "gold_find".into(),
                amount: 10.0,
                permanent: true,
            }),
            icon: '💰',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // CHALLENGE ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("speedrunner".into(), Achievement {
            id: "speedrunner".into(),
            name: "Speedrunner".into(),
            description: "Clear floor 5 in under 5 minutes.".into(),
            hint: "Gotta go fast.".into(),
            category: AchievementCategory::Challenge,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::SpeedrunFloor { floor: 5, max_seconds: 300.0 },
            reward: AchievementReward::StartingItem("chronos_quill".into()),
            icon: '⏱',
            hidden: false,
        });

        achievements.insert("no_hit_boss".into(), Achievement {
            id: "no_hit_boss".into(),
            name: "Perfect Victory".into(),
            description: "Defeat any boss without taking damage.".into(),
            hint: "Flawless execution.".into(),
            category: AchievementCategory::Challenge,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::NoDamageBoss("any".into()),
            reward: AchievementReward::Title("The Untouched".into()),
            icon: '🛡',
            hidden: false,
        });

        achievements.insert("minimalist".into(), Achievement {
            id: "minimalist".into(),
            name: "Minimalist".into(),
            description: "Clear 10 floors without using any items.".into(),
            hint: "Rely on skill alone.".into(),
            category: AchievementCategory::Challenge,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::NoItemsUsed { floors: 10 },
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::GameMode,
                id: "purist_mode".into(),
            }),
            icon: '🎯',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // STORY ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("lore_seeker".into(), Achievement {
            id: "lore_seeker".into(),
            name: "Lore Seeker".into(),
            description: "Discover 20 lore fragments.".into(),
            hint: "The world has stories to tell.".into(),
            category: AchievementCategory::Story,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::LoreDiscovered(20),
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::BorderStyle,
                id: "ancient_border".into(),
                name: "Ancient Border".into(),
            }),
            icon: '📖',
            hidden: false,
        });

        achievements.insert("historian".into(), Achievement {
            id: "historian".into(),
            name: "Historian".into(),
            description: "Discover all lore fragments.".into(),
            hint: "The complete picture emerges.".into(),
            category: AchievementCategory::Story,
            tier: AchievementTier::Platinum,
            requirement: AchievementRequirement::LoreDiscovered(100),
            reward: AchievementReward::Title("The Historian".into()),
            icon: '🎓',
            hidden: false,
        });

        // ═══════════════════════════════════════════════════════════════
        // META ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("persistent".into(), Achievement {
            id: "persistent".into(),
            name: "Persistent".into(),
            description: "Complete 10 runs (win or lose).".into(),
            hint: "Every attempt teaches something.".into(),
            category: AchievementCategory::Meta,
            tier: AchievementTier::Bronze,
            requirement: AchievementRequirement::RunsCompleted(10),
            reward: AchievementReward::StatBonus(StatBonusReward {
                stat: "starting_gold".into(),
                amount: 25.0,
                permanent: true,
            }),
            icon: '🔄',
            hidden: false,
        });

        achievements.insert("die_hard".into(), Achievement {
            id: "die_hard".into(),
            name: "Die Hard".into(),
            description: "Die 50 times.".into(),
            hint: "Death is just another teacher.".into(),
            category: AchievementCategory::Meta,
            tier: AchievementTier::Silver,
            requirement: AchievementRequirement::DeathCount(50),
            reward: AchievementReward::StartingItem("survivors_medallion".into()),
            icon: '💀',
            hidden: false,
        });

        achievements.insert("achievement_hunter".into(), Achievement {
            id: "achievement_hunter".into(),
            name: "Achievement Hunter".into(),
            description: "Unlock 25 achievements.".into(),
            hint: "Collecting the collectors.".into(),
            category: AchievementCategory::Meta,
            tier: AchievementTier::Gold,
            requirement: AchievementRequirement::AchievementsUnlocked(25),
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::BorderStyle,
                id: "golden_border".into(),
                name: "Golden Border".into(),
            }),
            icon: '🎖',
            hidden: false,
        });

        achievements.insert("completionist".into(), Achievement {
            id: "completionist".into(),
            name: "Completionist".into(),
            description: "Unlock all achievements.".into(),
            hint: "The ultimate challenge.".into(),
            category: AchievementCategory::Meta,
            tier: AchievementTier::Platinum,
            requirement: AchievementRequirement::AchievementsUnlocked(50),
            reward: AchievementReward::Title("The Completionist".into()),
            icon: '🌟',
            hidden: true,
        });

        // ═══════════════════════════════════════════════════════════════
        // SECRET ACHIEVEMENTS
        // ═══════════════════════════════════════════════════════════════

        achievements.insert("secret_konami".into(), Achievement {
            id: "secret_konami".into(),
            name: "Classic Code".into(),
            description: "???".into(),
            hint: "Up, up, down, down...".into(),
            category: AchievementCategory::Meta,
            tier: AchievementTier::Secret,
            requirement: AchievementRequirement::RunsCompleted(1), // Placeholder
            reward: AchievementReward::Cosmetic(CosmeticReward {
                reward_type: CosmeticType::TypewriterSound,
                id: "8bit_sounds".into(),
                name: "8-Bit Sounds".into(),
            }),
            icon: '🎮',
            hidden: true,
        });

        achievements.insert("secret_author".into(), Achievement {
            id: "secret_author".into(),
            name: "The Author".into(),
            description: "???".into(),
            hint: "Defeat what created you.".into(),
            category: AchievementCategory::Combat,
            tier: AchievementTier::Secret,
            requirement: AchievementRequirement::SpecificBossDefeated("author_of_all".into()),
            reward: AchievementReward::Unlock(UnlockReward {
                unlock_type: UnlockType::GameMode,
                id: "new_game_plus".into(),
            }),
            icon: '✍',
            hidden: true,
        });

        Self { achievements }
    }

    pub fn get(&self, id: &str) -> Option<&Achievement> {
        self.achievements.get(id)
    }

    pub fn by_category(&self, category: AchievementCategory) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.category == category)
            .collect()
    }

    pub fn by_tier(&self, tier: AchievementTier) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| a.tier == tier)
            .collect()
    }

    pub fn visible(&self) -> Vec<&Achievement> {
        self.achievements
            .values()
            .filter(|a| !a.hidden)
            .collect()
    }

    pub fn all_ids(&self) -> Vec<&str> {
        self.achievements.keys().map(|s| s.as_str()).collect()
    }
}

/// Player's achievement progress tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchievementProgress {
    pub unlocked: HashMap<String, UnlockedAchievement>,
    pub stats: AchievementStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockedAchievement {
    pub id: String,
    pub unlocked_at: String, // ISO timestamp
    pub run_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchievementStats {
    pub words_typed: u64,
    pub perfect_words: u64,
    pub total_keystrokes: u64,
    pub best_wpm: u32,
    pub best_combo: u32,
    pub enemies_defeated: u32,
    pub bosses_defeated: u32,
    pub bosses_defeated_list: Vec<String>,
    pub flawless_victories: u32,
    pub floors_reached: u32,
    pub items_collected: u32,
    pub relics_collected: u32,
    pub gold_earned: u64,
    pub lore_discovered: u32,
    pub runs_completed: u32,
    pub deaths: u32,
}

impl AchievementProgress {
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains_key(id)
    }

    pub fn unlock(&mut self, id: String, run_number: u32) -> bool {
        if self.unlocked.contains_key(&id) {
            return false;
        }
        
        self.unlocked.insert(id.clone(), UnlockedAchievement {
            id,
            unlocked_at: chrono_lite_now(),
            run_number,
        });
        true
    }

    pub fn unlocked_count(&self) -> usize {
        self.unlocked.len()
    }

    pub fn check_requirements(&self, db: &AchievementDatabase) -> Vec<String> {
        let mut newly_unlocked = Vec::new();
        
        for (id, achievement) in &db.achievements {
            if self.is_unlocked(id) {
                continue;
            }
            
            let met = match &achievement.requirement {
                AchievementRequirement::WordsTyped(n) => self.stats.words_typed >= *n,
                AchievementRequirement::PerfectWords(n) => self.stats.perfect_words >= *n,
                AchievementRequirement::WpmReached(wpm) => self.stats.best_wpm >= *wpm,
                AchievementRequirement::ComboReached(combo) => self.stats.best_combo >= *combo,
                AchievementRequirement::EnemiesDefeated(n) => self.stats.enemies_defeated >= *n,
                AchievementRequirement::BossesDefeated(n) => self.stats.bosses_defeated >= *n,
                AchievementRequirement::SpecificBossDefeated(boss) => {
                    self.stats.bosses_defeated_list.contains(boss)
                }
                AchievementRequirement::FlawlessVictories(n) => self.stats.flawless_victories >= *n,
                AchievementRequirement::FloorsReached(n) => self.stats.floors_reached >= *n,
                AchievementRequirement::ItemsCollected(n) => self.stats.items_collected >= *n,
                AchievementRequirement::RelicsCollected(n) => self.stats.relics_collected >= *n,
                AchievementRequirement::GoldEarned(n) => self.stats.gold_earned >= *n,
                AchievementRequirement::LoreDiscovered(n) => self.stats.lore_discovered >= *n,
                AchievementRequirement::RunsCompleted(n) => self.stats.runs_completed >= *n,
                AchievementRequirement::DeathCount(n) => self.stats.deaths >= *n,
                AchievementRequirement::AchievementsUnlocked(n) => self.unlocked.len() >= *n as usize,
                _ => false, // Complex requirements need special handling
            };
            
            if met {
                newly_unlocked.push(id.clone());
            }
        }
        
        newly_unlocked
    }
}

/// Simple timestamp without heavy chrono dependency
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}
