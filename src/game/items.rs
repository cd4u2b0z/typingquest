//! Items and loot system - Balatro-inspired card/joker mechanics!

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    pub fn color(&self) -> &'static str {
        match self {
            ItemRarity::Common => "white",
            ItemRarity::Uncommon => "green",
            ItemRarity::Rare => "blue",
            ItemRarity::Epic => "magenta",
            ItemRarity::Legendary => "yellow",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            ItemRarity::Common => "○",
            ItemRarity::Uncommon => "◐",
            ItemRarity::Rare => "●",
            ItemRarity::Epic => "◆",
            ItemRarity::Legendary => "★",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    Consumable,
    Equipment,
    Joker,      // Balatro-style passive effect cards!
    Relic,      // Permanent upgrades
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub flavor_text: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub effect: ItemEffect,
    pub price: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemEffect {
    // Consumables
    HealHP(i32),
    HealMP(i32),
    HealBoth { hp: i32, mp: i32 },
    BuffStat { stat: String, amount: i32, duration: i32 },
    CureStatus,
    Escape,     // Flee from battle

    // Equipment bonuses
    StatBonus { hp: i32, mp: i32, str_: i32, dex: i32, int: i32 },
    
    // Jokers - Balatro style!
    TypingBonus { wpm_threshold: i32, bonus_damage: i32 },
    AccuracyBonus { accuracy_threshold: i32, bonus_damage: i32 },
    ComboBonus { combo_threshold: i32, multiplier: f32 },
    CritChance(i32),        // % chance for 2x damage
    LifeSteal(i32),         // % of damage healed
    ManaSteal(i32),         // % of damage restored as MP
    TimeExtend(f32),        // Extra seconds for typing
    ErrorForgive(i32),      // Forgive N typos per word
    DoubleLetters,          // Repeated letters count as 2
    HomeRowBonus(i32),      // Bonus damage for home row words
    GoldMultiplier(f32),    // Extra gold from battles
    XPMultiplier(f32),      // Extra XP from battles
    LuckyDrop(i32),         // % bonus to item drop rate
    
    // Relics - permanent upgrades
    MaxHPBonus(i32),
    MaxMPBonus(i32),
    StartingShield(i32),
    BossKiller(i32),        // % bonus damage to bosses
    SpeedDemon(f32),        // Time limit reduced but damage up
}

impl Item {
    pub fn consumable_pool() -> Vec<Self> {
        vec![
            Item {
                name: "Health Potion".to_string(),
                description: "Restores 30 HP.".to_string(),
                flavor_text: "Tastes like cherry... and regret.".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Common,
                effect: ItemEffect::HealHP(30),
                price: 25,
            },
            Item {
                name: "Mana Potion".to_string(),
                description: "Restores 20 MP.".to_string(),
                flavor_text: "Suspiciously blue.".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Common,
                effect: ItemEffect::HealMP(20),
                price: 20,
            },
            Item {
                name: "Greater Health Potion".to_string(),
                description: "Restores 75 HP.".to_string(),
                flavor_text: "Now with 50% more health!".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::HealHP(75),
                price: 60,
            },
            Item {
                name: "Full Restore".to_string(),
                description: "Restores all HP and MP.".to_string(),
                flavor_text: "The good stuff.".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::HealBoth { hp: 999, mp: 999 },
                price: 150,
            },
            Item {
                name: "Smoke Bomb".to_string(),
                description: "Escape from any non-boss battle.".to_string(),
                flavor_text: "NINJA VANISH!".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::Escape,
                price: 40,
            },
            Item {
                name: "Antidote".to_string(),
                description: "Cures all status effects.".to_string(),
                flavor_text: "Side effects may include clarity.".to_string(),
                item_type: ItemType::Consumable,
                rarity: ItemRarity::Common,
                effect: ItemEffect::CureStatus,
                price: 15,
            },
        ]
    }

    pub fn joker_pool() -> Vec<Self> {
        vec![
            // Speed Jokers
            Item {
                name: "The Speedster".to_string(),
                description: "Deal +15 damage when typing above 60 WPM.".to_string(),
                flavor_text: "Gotta go fast!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::TypingBonus { wpm_threshold: 60, bonus_damage: 15 },
                price: 100,
            },
            Item {
                name: "The Flash Typist".to_string(),
                description: "Deal +30 damage when typing above 80 WPM.".to_string(),
                flavor_text: "My keyboard is smoking.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::TypingBonus { wpm_threshold: 80, bonus_damage: 30 },
                price: 180,
            },
            Item {
                name: "The Blur".to_string(),
                description: "Deal +50 damage when typing above 100 WPM.".to_string(),
                flavor_text: "They called me mad. MAD!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Epic,
                effect: ItemEffect::TypingBonus { wpm_threshold: 100, bonus_damage: 50 },
                price: 300,
            },

            // Accuracy Jokers
            Item {
                name: "The Perfectionist".to_string(),
                description: "Deal +20 damage with 100% accuracy.".to_string(),
                flavor_text: "Not a single mistake.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::AccuracyBonus { accuracy_threshold: 100, bonus_damage: 20 },
                price: 120,
            },
            Item {
                name: "Good Enough".to_string(),
                description: "Deal +10 damage with 90%+ accuracy.".to_string(),
                flavor_text: "Close enough!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Common,
                effect: ItemEffect::AccuracyBonus { accuracy_threshold: 90, bonus_damage: 10 },
                price: 60,
            },

            // Combo Jokers
            Item {
                name: "Combo Master".to_string(),
                description: "1.5x damage at 5+ combo.".to_string(),
                flavor_text: "C-C-C-COMBO!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::ComboBonus { combo_threshold: 5, multiplier: 1.5 },
                price: 200,
            },
            Item {
                name: "Combo God".to_string(),
                description: "2x damage at 10+ combo.".to_string(),
                flavor_text: "UNSTOPPABLE!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Legendary,
                effect: ItemEffect::ComboBonus { combo_threshold: 10, multiplier: 2.0 },
                price: 400,
            },

            // Utility Jokers
            Item {
                name: "Lucky Crit".to_string(),
                description: "15% chance for critical hit (2x damage).".to_string(),
                flavor_text: "Feeling lucky?".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::CritChance(15),
                price: 100,
            },
            Item {
                name: "Vampiric Touch".to_string(),
                description: "Heal 10% of damage dealt.".to_string(),
                flavor_text: "Your pain is my gain.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::LifeSteal(10),
                price: 180,
            },
            Item {
                name: "Mana Siphon".to_string(),
                description: "Restore 5% of damage as MP.".to_string(),
                flavor_text: "Magic flows back.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::ManaSteal(5),
                price: 120,
            },
            Item {
                name: "Time Lord".to_string(),
                description: "+2 seconds for all typing challenges.".to_string(),
                flavor_text: "Time is on your side.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::TimeExtend(2.0),
                price: 150,
            },
            Item {
                name: "Forgiving Keys".to_string(),
                description: "First 2 typos per word are forgiven.".to_string(),
                flavor_text: "Everyone makes misteaks.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::ErrorForgive(2),
                price: 200,
            },
            Item {
                name: "Home Row Hero".to_string(),
                description: "+25% damage for words using only home row.".to_string(),
                flavor_text: "A S D F J K L ;".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::HomeRowBonus(25),
                price: 100,
            },

            // Economy Jokers
            Item {
                name: "Gold Digger".to_string(),
                description: "+50% gold from all sources.".to_string(),
                flavor_text: "Money money money!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::GoldMultiplier(1.5),
                price: 250,
            },
            Item {
                name: "Wisdom Seeker".to_string(),
                description: "+25% XP from all sources.".to_string(),
                flavor_text: "Knowledge is power.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::XPMultiplier(1.25),
                price: 200,
            },
            Item {
                name: "Lucky Charm".to_string(),
                description: "+20% item drop rate.".to_string(),
                flavor_text: "Found a four-leaf clover!".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Uncommon,
                effect: ItemEffect::LuckyDrop(20),
                price: 150,
            },

            // Legendary Jokers
            Item {
                name: "The One".to_string(),
                description: "All bonuses: +20 damage at 80+ WPM, 95%+ acc, 5+ combo.".to_string(),
                flavor_text: "I know keyboard-fu.".to_string(),
                item_type: ItemType::Joker,
                rarity: ItemRarity::Legendary,
                effect: ItemEffect::TypingBonus { wpm_threshold: 80, bonus_damage: 20 },
                price: 500,
            },
        ]
    }

    pub fn relic_pool() -> Vec<Self> {
        vec![
            Item {
                name: "Heart Container".to_string(),
                description: "Permanently +25 Max HP.".to_string(),
                flavor_text: "Your heart grows stronger.".to_string(),
                item_type: ItemType::Relic,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::MaxHPBonus(25),
                price: 200,
            },
            Item {
                name: "Mana Crystal".to_string(),
                description: "Permanently +15 Max MP.".to_string(),
                flavor_text: "Magic resonates within.".to_string(),
                item_type: ItemType::Relic,
                rarity: ItemRarity::Rare,
                effect: ItemEffect::MaxMPBonus(15),
                price: 180,
            },
            Item {
                name: "Guardian Angel".to_string(),
                description: "Start each battle with 20 shield.".to_string(),
                flavor_text: "Protected from above.".to_string(),
                item_type: ItemType::Relic,
                rarity: ItemRarity::Epic,
                effect: ItemEffect::StartingShield(20),
                price: 300,
            },
            Item {
                name: "Giant Slayer".to_string(),
                description: "+30% damage against bosses.".to_string(),
                flavor_text: "The bigger they are...".to_string(),
                item_type: ItemType::Relic,
                rarity: ItemRarity::Epic,
                effect: ItemEffect::BossKiller(30),
                price: 350,
            },
            Item {
                name: "Coffee IV Drip".to_string(),
                description: "Time limits -20% but damage +40%.".to_string(),
                flavor_text: "MAXIMUM CAFFEINE.".to_string(),
                item_type: ItemType::Relic,
                rarity: ItemRarity::Legendary,
                effect: ItemEffect::SpeedDemon(0.4),
                price: 400,
            },
        ]
    }

    pub fn random_consumable() -> Self {
        let mut rng = rand::thread_rng();
        Self::consumable_pool().choose(&mut rng).unwrap().clone()
    }

    pub fn random_joker() -> Self {
        let mut rng = rand::thread_rng();
        Self::joker_pool().choose(&mut rng).unwrap().clone()
    }

    pub fn random_relic() -> Self {
        let mut rng = rand::thread_rng();
        Self::relic_pool().choose(&mut rng).unwrap().clone()
    }

    pub fn random_by_rarity(rarity: ItemRarity) -> Option<Self> {
        let mut rng = rand::thread_rng();
        let all_items: Vec<Self> = [
            Self::consumable_pool(),
            Self::joker_pool(),
            Self::relic_pool(),
        ].concat();
        
        let filtered: Vec<Self> = all_items.into_iter()
            .filter(|i| i.rarity == rarity)
            .collect();
        
        filtered.choose(&mut rng).cloned()
    }
}
