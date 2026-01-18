//! Item data structures and definitions
//!
//! Equipment, consumables, and relics with typing-themed effects.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database of all items
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemDatabase {
    pub equipment: HashMap<String, Equipment>,
    pub consumables: HashMap<String, Consumable>,
    pub relics: HashMap<String, Relic>,
}

/// Equipment that can be worn for stat bonuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slot: EquipmentSlot,
    pub rarity: Rarity,
    pub stats: StatBonus,
    pub typing_effect: Option<TypingEffect>,
    pub flavor_text: String,
    pub gold_value: i32,
}

/// Consumable items used during combat or exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub effect: ConsumableEffect,
    pub stack_max: u32,
    pub gold_value: i32,
}

/// Relics - passive items with powerful effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub effect: RelicEffect,
    pub lore: String,
    pub gold_value: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Accessory,
    Gloves,
    Boots,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatBonus {
    pub max_hp: i32,
    pub damage: i32,
    pub defense: i32,
    pub crit_chance: f32,
    pub typing_speed_bonus: f32,
    pub gold_bonus: f32,
    pub xp_bonus: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingEffect {
    ExtendedTime { seconds: f32 },
    ReducedWordLength { chars: u32 },
    DoubleLetterBonus,
    AccuracyForgiveness { errors_forgiven: u32 },
    ComboMultiplier { bonus: f32 },
    WordPreview { words_ahead: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    Heal { amount: i32 },
    HealPercent { percent: f32 },
    TemporaryDamage { amount: i32, duration_words: u32 },
    TemporaryDefense { amount: i32, duration_words: u32 },
    TemporaryTypingBonus { bonus: f32, duration_words: u32 },
    ClearNegativeEffects,
    RevealWord,
    SkipWord,
    DoubleGold { duration_combats: u32 },
    DoubleXP { duration_combats: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelicEffect {
    PassiveStat(StatBonus),
    OnPerfectWord { effect: Box<RelicTriggerEffect> },
    OnCombo { threshold: u32, effect: Box<RelicTriggerEffect> },
    OnCombatStart { effect: Box<RelicTriggerEffect> },
    OnCombatEnd { effect: Box<RelicTriggerEffect> },
    OnLowHealth { threshold_percent: f32, effect: Box<RelicTriggerEffect> },
    OnBossKill { effect: Box<RelicTriggerEffect> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelicTriggerEffect {
    Heal { amount: i32 },
    DamageBoost { amount: i32, duration_words: u32 },
    FreeWord,
    ExtraGold { amount: i32 },
    ExtraXP { amount: u64 },
    Shield { amount: i32 },
}

impl ItemDatabase {
    pub fn embedded() -> Self {
        let mut equipment = HashMap::new();
        let mut consumables = HashMap::new();
        let mut relics = HashMap::new();

        // ═══════════════════════════════════════════════
        // WEAPONS
        // ═══════════════════════════════════════════════

        equipment.insert("quill_of_swiftness".into(), Equipment {
            id: "quill_of_swiftness".into(),
            name: "Quill of Swiftness".into(),
            description: "A magical quill that makes your fingers dance across keys.".into(),
            slot: EquipmentSlot::Weapon,
            rarity: Rarity::Common,
            stats: StatBonus { damage: 3, typing_speed_bonus: 0.05, ..Default::default() },
            typing_effect: None,
            flavor_text: "The pen is mightier than the sword, especially when enchanted.".into(),
            gold_value: 50,
        });

        equipment.insert("inkblade".into(), Equipment {
            id: "inkblade".into(),
            name: "Inkblade".into(),
            description: "A sword that drips with living ink, cutting words from existence.".into(),
            slot: EquipmentSlot::Weapon,
            rarity: Rarity::Uncommon,
            stats: StatBonus { damage: 8, crit_chance: 0.05, ..Default::default() },
            typing_effect: Some(TypingEffect::ReducedWordLength { chars: 1 }),
            flavor_text: "Each stroke writes a new chapter of your enemy's demise.".into(),
            gold_value: 150,
        });

        equipment.insert("tome_of_power".into(), Equipment {
            id: "tome_of_power".into(),
            name: "Tome of Power".into(),
            description: "An ancient book crackling with arcane energy.".into(),
            slot: EquipmentSlot::Weapon,
            rarity: Rarity::Rare,
            stats: StatBonus { damage: 12, max_hp: 10, crit_chance: 0.08, ..Default::default() },
            typing_effect: Some(TypingEffect::ComboMultiplier { bonus: 0.1 }),
            flavor_text: "Knowledge is power. This is a lot of knowledge.".into(),
            gold_value: 350,
        });

        equipment.insert("voidwriter".into(), Equipment {
            id: "voidwriter".into(),
            name: "Voidwriter".into(),
            description: "A pen that writes in the space between existence.".into(),
            slot: EquipmentSlot::Weapon,
            rarity: Rarity::Epic,
            stats: StatBonus { damage: 18, crit_chance: 0.12, typing_speed_bonus: 0.1, ..Default::default() },
            typing_effect: Some(TypingEffect::AccuracyForgiveness { errors_forgiven: 1 }),
            flavor_text: "It writes what was never meant to be written.".into(),
            gold_value: 750,
        });

        equipment.insert("word_of_creation".into(), Equipment {
            id: "word_of_creation".into(),
            name: "Word of Creation".into(),
            description: "The first word ever spoken, given form.".into(),
            slot: EquipmentSlot::Weapon,
            rarity: Rarity::Legendary,
            stats: StatBonus { damage: 30, max_hp: 25, crit_chance: 0.15, typing_speed_bonus: 0.15, ..Default::default() },
            typing_effect: Some(TypingEffect::DoubleLetterBonus),
            flavor_text: "In the beginning was the Word, and the Word was with you.".into(),
            gold_value: 2000,
        });

        // ═══════════════════════════════════════════════
        // ARMOR
        // ═══════════════════════════════════════════════

        equipment.insert("leather_jerkin".into(), Equipment {
            id: "leather_jerkin".into(),
            name: "Leather Jerkin".into(),
            description: "Simple but effective protection.".into(),
            slot: EquipmentSlot::Armor,
            rarity: Rarity::Common,
            stats: StatBonus { defense: 2, max_hp: 5, ..Default::default() },
            typing_effect: None,
            flavor_text: "It's not fancy, but it keeps the pointy bits out.".into(),
            gold_value: 40,
        });

        equipment.insert("scribes_robes".into(), Equipment {
            id: "scribes_robes".into(),
            name: "Scribe's Robes".into(),
            description: "Robes worn by the archivists of old.".into(),
            slot: EquipmentSlot::Armor,
            rarity: Rarity::Uncommon,
            stats: StatBonus { defense: 4, max_hp: 10, xp_bonus: 0.1, ..Default::default() },
            typing_effect: None,
            flavor_text: "The ink stains tell a thousand stories.".into(),
            gold_value: 120,
        });

        equipment.insert("chainmail_of_focus".into(), Equipment {
            id: "chainmail_of_focus".into(),
            name: "Chainmail of Focus".into(),
            description: "Armor that helps you concentrate amidst chaos.".into(),
            slot: EquipmentSlot::Armor,
            rarity: Rarity::Rare,
            stats: StatBonus { defense: 8, max_hp: 20, ..Default::default() },
            typing_effect: Some(TypingEffect::ExtendedTime { seconds: 1.0 }),
            flavor_text: "Each link is inscribed with a word of clarity.".into(),
            gold_value: 300,
        });

        equipment.insert("void_vestments".into(), Equipment {
            id: "void_vestments".into(),
            name: "Void Vestments".into(),
            description: "Robes woven from the fabric between realities.".into(),
            slot: EquipmentSlot::Armor,
            rarity: Rarity::Epic,
            stats: StatBonus { defense: 12, max_hp: 35, typing_speed_bonus: 0.05, ..Default::default() },
            typing_effect: Some(TypingEffect::AccuracyForgiveness { errors_forgiven: 1 }),
            flavor_text: "You can feel the nothing pressing against you, protecting you.".into(),
            gold_value: 700,
        });

        // ═══════════════════════════════════════════════
        // GLOVES
        // ═══════════════════════════════════════════════

        equipment.insert("typing_gloves".into(), Equipment {
            id: "typing_gloves".into(),
            name: "Typing Gloves".into(),
            description: "Fingerless gloves that enhance dexterity.".into(),
            slot: EquipmentSlot::Gloves,
            rarity: Rarity::Common,
            stats: StatBonus { typing_speed_bonus: 0.08, ..Default::default() },
            typing_effect: None,
            flavor_text: "Feel the keys beneath your fingertips.".into(),
            gold_value: 35,
        });

        equipment.insert("quicksilver_gauntlets".into(), Equipment {
            id: "quicksilver_gauntlets".into(),
            name: "Quicksilver Gauntlets".into(),
            description: "Gloves infused with liquid metal for lightning reflexes.".into(),
            slot: EquipmentSlot::Gloves,
            rarity: Rarity::Rare,
            stats: StatBonus { typing_speed_bonus: 0.15, crit_chance: 0.05, ..Default::default() },
            typing_effect: Some(TypingEffect::ReducedWordLength { chars: 1 }),
            flavor_text: "Your fingers flow like mercury across the keys.".into(),
            gold_value: 280,
        });

        // ═══════════════════════════════════════════════
        // ACCESSORIES
        // ═══════════════════════════════════════════════

        equipment.insert("copper_ring".into(), Equipment {
            id: "copper_ring".into(),
            name: "Copper Ring".into(),
            description: "A simple ring with minor enchantment.".into(),
            slot: EquipmentSlot::Accessory,
            rarity: Rarity::Common,
            stats: StatBonus { gold_bonus: 0.05, ..Default::default() },
            typing_effect: None,
            flavor_text: "Every copper counts.".into(),
            gold_value: 25,
        });

        equipment.insert("amulet_of_foresight".into(), Equipment {
            id: "amulet_of_foresight".into(),
            name: "Amulet of Foresight".into(),
            description: "Shows glimpses of what's to come.".into(),
            slot: EquipmentSlot::Accessory,
            rarity: Rarity::Rare,
            stats: StatBonus { max_hp: 15, ..Default::default() },
            typing_effect: Some(TypingEffect::WordPreview { words_ahead: 2 }),
            flavor_text: "See the words before they arrive.".into(),
            gold_value: 400,
        });

        equipment.insert("ring_of_perfection".into(), Equipment {
            id: "ring_of_perfection".into(),
            name: "Ring of Perfection".into(),
            description: "A ring that rewards flawless typing.".into(),
            slot: EquipmentSlot::Accessory,
            rarity: Rarity::Epic,
            stats: StatBonus { crit_chance: 0.2, damage: 5, ..Default::default() },
            typing_effect: Some(TypingEffect::ComboMultiplier { bonus: 0.2 }),
            flavor_text: "Perfection is not a goal, it's a standard.".into(),
            gold_value: 600,
        });

        // ═══════════════════════════════════════════════
        // CONSUMABLES
        // ═══════════════════════════════════════════════

        consumables.insert("health_potion".into(), Consumable {
            id: "health_potion".into(),
            name: "Health Potion".into(),
            description: "Restores 30 HP.".into(),
            rarity: Rarity::Common,
            effect: ConsumableEffect::Heal { amount: 30 },
            stack_max: 5,
            gold_value: 25,
        });

        consumables.insert("greater_health_potion".into(), Consumable {
            id: "greater_health_potion".into(),
            name: "Greater Health Potion".into(),
            description: "Restores 75 HP.".into(),
            rarity: Rarity::Uncommon,
            effect: ConsumableEffect::Heal { amount: 75 },
            stack_max: 3,
            gold_value: 60,
        });

        consumables.insert("elixir_of_restoration".into(), Consumable {
            id: "elixir_of_restoration".into(),
            name: "Elixir of Restoration".into(),
            description: "Restores 50% of max HP.".into(),
            rarity: Rarity::Rare,
            effect: ConsumableEffect::HealPercent { percent: 0.5 },
            stack_max: 2,
            gold_value: 150,
        });

        consumables.insert("focus_tonic".into(), Consumable {
            id: "focus_tonic".into(),
            name: "Focus Tonic".into(),
            description: "Increases typing speed for 5 words.".into(),
            rarity: Rarity::Common,
            effect: ConsumableEffect::TemporaryTypingBonus { bonus: 0.2, duration_words: 5 },
            stack_max: 5,
            gold_value: 30,
        });

        consumables.insert("clarity_draught".into(), Consumable {
            id: "clarity_draught".into(),
            name: "Clarity Draught".into(),
            description: "Removes all negative effects.".into(),
            rarity: Rarity::Uncommon,
            effect: ConsumableEffect::ClearNegativeEffects,
            stack_max: 3,
            gold_value: 50,
        });

        consumables.insert("revealing_scroll".into(), Consumable {
            id: "revealing_scroll".into(),
            name: "Revealing Scroll".into(),
            description: "Shows the next word clearly.".into(),
            rarity: Rarity::Common,
            effect: ConsumableEffect::RevealWord,
            stack_max: 10,
            gold_value: 15,
        });

        consumables.insert("skip_scroll".into(), Consumable {
            id: "skip_scroll".into(),
            name: "Skip Scroll".into(),
            description: "Skips the current word without penalty.".into(),
            rarity: Rarity::Rare,
            effect: ConsumableEffect::SkipWord,
            stack_max: 2,
            gold_value: 100,
        });

        consumables.insert("gold_incense".into(), Consumable {
            id: "gold_incense".into(),
            name: "Gold Incense".into(),
            description: "Double gold from the next 3 combats.".into(),
            rarity: Rarity::Rare,
            effect: ConsumableEffect::DoubleGold { duration_combats: 3 },
            stack_max: 2,
            gold_value: 80,
        });

        // ═══════════════════════════════════════════════
        // RELICS
        // ═══════════════════════════════════════════════

        relics.insert("lucky_coin".into(), Relic {
            id: "lucky_coin".into(),
            name: "Lucky Coin".into(),
            description: "Increases gold drops by 15%.".into(),
            rarity: Rarity::Common,
            effect: RelicEffect::PassiveStat(StatBonus { gold_bonus: 0.15, ..Default::default() }),
            lore: "Heads you win, tails... you also win.".into(),
            gold_value: 100,
        });

        relics.insert("scholars_lens".into(), Relic {
            id: "scholars_lens".into(),
            name: "Scholar's Lens".into(),
            description: "Increases XP gained by 20%.".into(),
            rarity: Rarity::Uncommon,
            effect: RelicEffect::PassiveStat(StatBonus { xp_bonus: 0.2, ..Default::default() }),
            lore: "See the lessons in every encounter.".into(),
            gold_value: 150,
        });

        relics.insert("heart_of_the_scribe".into(), Relic {
            id: "heart_of_the_scribe".into(),
            name: "Heart of the Scribe".into(),
            description: "Perfect words heal you for 2 HP.".into(),
            rarity: Rarity::Rare,
            effect: RelicEffect::OnPerfectWord { 
                effect: Box::new(RelicTriggerEffect::Heal { amount: 2 }) 
            },
            lore: "Every perfect keystroke strengthens the soul.".into(),
            gold_value: 350,
        });

        relics.insert("combo_crown".into(), Relic {
            id: "combo_crown".into(),
            name: "Combo Crown".into(),
            description: "At 10 combo, gain a damage boost.".into(),
            rarity: Rarity::Rare,
            effect: RelicEffect::OnCombo { 
                threshold: 10, 
                effect: Box::new(RelicTriggerEffect::DamageBoost { amount: 5, duration_words: 3 }) 
            },
            lore: "The crown rewards those who type without faltering.".into(),
            gold_value: 400,
        });

        relics.insert("survivors_medallion".into(), Relic {
            id: "survivors_medallion".into(),
            name: "Survivor's Medallion".into(),
            description: "Below 25% HP, gain a shield each word.".into(),
            rarity: Rarity::Epic,
            effect: RelicEffect::OnLowHealth { 
                threshold_percent: 0.25, 
                effect: Box::new(RelicTriggerEffect::Shield { amount: 3 }) 
            },
            lore: "Death's door becomes a fortress.".into(),
            gold_value: 500,
        });

        relics.insert("void_fragment".into(), Relic {
            id: "void_fragment".into(),
            name: "Void Fragment".into(),
            description: "Killing a boss grants extra gold and XP.".into(),
            rarity: Rarity::Epic,
            effect: RelicEffect::OnBossKill { 
                effect: Box::new(RelicTriggerEffect::ExtraGold { amount: 100 }) 
            },
            lore: "A piece of the void, hungry for power.".into(),
            gold_value: 600,
        });

        relics.insert("genesis_shard".into(), Relic {
            id: "genesis_shard".into(),
            name: "Genesis Shard".into(),
            description: "Start each combat with a free word typed.".into(),
            rarity: Rarity::Legendary,
            effect: RelicEffect::OnCombatStart { 
                effect: Box::new(RelicTriggerEffect::FreeWord) 
            },
            lore: "A fragment of the first word ever spoken.".into(),
            gold_value: 1000,
        });

        Self { equipment, consumables, relics }
    }

    pub fn get_equipment(&self, id: &str) -> Option<&Equipment> {
        self.equipment.get(id)
    }

    pub fn get_consumable(&self, id: &str) -> Option<&Consumable> {
        self.consumables.get(id)
    }

    pub fn get_relic(&self, id: &str) -> Option<&Relic> {
        self.relics.get(id)
    }

    pub fn get_equipment_by_slot(&self, slot: EquipmentSlot) -> Vec<&Equipment> {
        self.equipment.values().filter(|e| e.slot == slot).collect()
    }

    pub fn get_equipment_by_rarity(&self, rarity: Rarity) -> Vec<&Equipment> {
        self.equipment.values().filter(|e| e.rarity == rarity).collect()
    }
}
