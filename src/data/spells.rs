//! Spell data structures and definitions
//!
//! Combat abilities with typing-based activation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database of all spells
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpellDatabase {
    pub spells: HashMap<String, Spell>,
}

/// A castable spell with typing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub element: Element,
    pub tier: SpellTier,
    pub mana_cost: i32,
    pub cooldown_turns: u32,
    pub effect: SpellEffect,
    pub cast_words: Vec<String>,
    pub flavor_text: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Holy,
    Shadow,
    Arcane,
    Nature,
    Void,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellTier {
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellEffect {
    Damage { amount: i32, piercing: bool },
    DamageOverTime { amount: i32, turns: u32 },
    Heal { amount: i32 },
    HealOverTime { amount: i32, turns: u32 },
    Shield { amount: i32, duration_turns: u32 },
    Buff { stat: BuffStat, amount: i32, duration_turns: u32 },
    Debuff { stat: BuffStat, amount: i32, duration_turns: u32 },
    WordManipulation(WordManipEffect),
    MultiEffect(Vec<SpellEffect>),
    LifeSteal { damage: i32, heal_percent: f32 },
    Execute { threshold_percent: f32, damage: i32 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuffStat {
    Damage,
    Defense,
    TypingSpeed,
    CritChance,
    TimeLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WordManipEffect {
    SimplifyNextWord,
    RevealWord,
    SkipWord,
    SlowEnemy { duration_turns: u32 },
    ScrambleEnemyWord,
}

impl SpellDatabase {
    pub fn embedded() -> Self {
        let mut spells = HashMap::new();

        // ═══════════════════════════════════════════════
        // FIRE SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("ember".into(), Spell {
            id: "ember".into(),
            name: "Ember".into(),
            description: "A small burst of flame.".into(),
            element: Element::Fire,
            tier: SpellTier::Novice,
            mana_cost: 5,
            cooldown_turns: 0,
            effect: SpellEffect::Damage { amount: 10, piercing: false },
            cast_words: vec!["fire".into(), "burn".into(), "heat".into()],
            flavor_text: "Even the smallest spark can start a wildfire.".into(),
        });

        spells.insert("fireball".into(), Spell {
            id: "fireball".into(),
            name: "Fireball".into(),
            description: "A classic ball of fire that explodes on impact.".into(),
            element: Element::Fire,
            tier: SpellTier::Apprentice,
            mana_cost: 15,
            cooldown_turns: 1,
            effect: SpellEffect::Damage { amount: 30, piercing: false },
            cast_words: vec!["fireball".into(), "explosion".into(), "blaze".into()],
            flavor_text: "The answer to most problems, according to wizards.".into(),
        });

        spells.insert("immolate".into(), Spell {
            id: "immolate".into(),
            name: "Immolate".into(),
            description: "Sets the target ablaze, dealing damage over time.".into(),
            element: Element::Fire,
            tier: SpellTier::Journeyman,
            mana_cost: 20,
            cooldown_turns: 2,
            effect: SpellEffect::DamageOverTime { amount: 8, turns: 4 },
            cast_words: vec!["immolate".into(), "incinerate".into(), "conflagration".into()],
            flavor_text: "Some say the world will end in fire...".into(),
        });

        spells.insert("inferno".into(), Spell {
            id: "inferno".into(),
            name: "Inferno".into(),
            description: "A devastating wave of fire.".into(),
            element: Element::Fire,
            tier: SpellTier::Master,
            mana_cost: 40,
            cooldown_turns: 4,
            effect: SpellEffect::Damage { amount: 75, piercing: true },
            cast_words: vec!["inferno".into(), "apocalypse".into(), "pyroclasm".into()],
            flavor_text: "When words themselves burn, only ash remains.".into(),
        });

        // ═══════════════════════════════════════════════
        // ICE SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("frost_touch".into(), Spell {
            id: "frost_touch".into(),
            name: "Frost Touch".into(),
            description: "A chilling touch that slows enemies.".into(),
            element: Element::Ice,
            tier: SpellTier::Novice,
            mana_cost: 5,
            cooldown_turns: 0,
            effect: SpellEffect::MultiEffect(vec![
                SpellEffect::Damage { amount: 6, piercing: false },
                SpellEffect::Debuff { stat: BuffStat::TypingSpeed, amount: 10, duration_turns: 2 },
            ]),
            cast_words: vec!["frost".into(), "cold".into(), "chill".into()],
            flavor_text: "Cold as the void between stars.".into(),
        });

        spells.insert("ice_shard".into(), Spell {
            id: "ice_shard".into(),
            name: "Ice Shard".into(),
            description: "A sharp projectile of frozen water.".into(),
            element: Element::Ice,
            tier: SpellTier::Apprentice,
            mana_cost: 12,
            cooldown_turns: 1,
            effect: SpellEffect::Damage { amount: 25, piercing: false },
            cast_words: vec!["icicle".into(), "glacier".into(), "freeze".into()],
            flavor_text: "Nature's dagger, honed by winter.".into(),
        });

        spells.insert("blizzard".into(), Spell {
            id: "blizzard".into(),
            name: "Blizzard".into(),
            description: "A howling storm of ice and snow.".into(),
            element: Element::Ice,
            tier: SpellTier::Expert,
            mana_cost: 30,
            cooldown_turns: 3,
            effect: SpellEffect::MultiEffect(vec![
                SpellEffect::Damage { amount: 40, piercing: false },
                SpellEffect::WordManipulation(WordManipEffect::SlowEnemy { duration_turns: 3 }),
            ]),
            cast_words: vec!["blizzard".into(), "avalanche".into(), "permafrost".into()],
            flavor_text: "Winter comes for all who type too slowly.".into(),
        });

        // ═══════════════════════════════════════════════
        // LIGHTNING SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("spark".into(), Spell {
            id: "spark".into(),
            name: "Spark".into(),
            description: "A quick jolt of electricity.".into(),
            element: Element::Lightning,
            tier: SpellTier::Novice,
            mana_cost: 4,
            cooldown_turns: 0,
            effect: SpellEffect::Damage { amount: 8, piercing: false },
            cast_words: vec!["spark".into(), "zap".into(), "jolt".into()],
            flavor_text: "Quick as thought, sharp as pain.".into(),
        });

        spells.insert("lightning_bolt".into(), Spell {
            id: "lightning_bolt".into(),
            name: "Lightning Bolt".into(),
            description: "A powerful bolt from the heavens.".into(),
            element: Element::Lightning,
            tier: SpellTier::Journeyman,
            mana_cost: 20,
            cooldown_turns: 2,
            effect: SpellEffect::Damage { amount: 45, piercing: true },
            cast_words: vec!["lightning".into(), "thunder".into(), "storm".into()],
            flavor_text: "The sky writes in letters of light.".into(),
        });

        spells.insert("chain_lightning".into(), Spell {
            id: "chain_lightning".into(),
            name: "Chain Lightning".into(),
            description: "Lightning that jumps between targets.".into(),
            element: Element::Lightning,
            tier: SpellTier::Expert,
            mana_cost: 35,
            cooldown_turns: 3,
            effect: SpellEffect::Damage { amount: 60, piercing: true },
            cast_words: vec!["tempest".into(), "electrocute".into(), "conductor".into()],
            flavor_text: "One word, infinite consequences.".into(),
        });

        // ═══════════════════════════════════════════════
        // HOLY SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("minor_heal".into(), Spell {
            id: "minor_heal".into(),
            name: "Minor Heal".into(),
            description: "A simple healing light.".into(),
            element: Element::Holy,
            tier: SpellTier::Novice,
            mana_cost: 8,
            cooldown_turns: 1,
            effect: SpellEffect::Heal { amount: 20 },
            cast_words: vec!["heal".into(), "mend".into(), "restore".into()],
            flavor_text: "The first word of comfort.".into(),
        });

        spells.insert("divine_shield".into(), Spell {
            id: "divine_shield".into(),
            name: "Divine Shield".into(),
            description: "A protective barrier of holy light.".into(),
            element: Element::Holy,
            tier: SpellTier::Apprentice,
            mana_cost: 15,
            cooldown_turns: 2,
            effect: SpellEffect::Shield { amount: 30, duration_turns: 3 },
            cast_words: vec!["shield".into(), "protect".into(), "barrier".into()],
            flavor_text: "Words of protection written in light.".into(),
        });

        spells.insert("greater_heal".into(), Spell {
            id: "greater_heal".into(),
            name: "Greater Heal".into(),
            description: "Powerful restorative magic.".into(),
            element: Element::Holy,
            tier: SpellTier::Journeyman,
            mana_cost: 25,
            cooldown_turns: 2,
            effect: SpellEffect::Heal { amount: 50 },
            cast_words: vec!["restoration".into(), "rejuvenate".into(), "salvation".into()],
            flavor_text: "The word that rebuilds what was broken.".into(),
        });

        spells.insert("smite".into(), Spell {
            id: "smite".into(),
            name: "Smite".into(),
            description: "Holy damage that pierces darkness.".into(),
            element: Element::Holy,
            tier: SpellTier::Journeyman,
            mana_cost: 18,
            cooldown_turns: 1,
            effect: SpellEffect::Damage { amount: 35, piercing: true },
            cast_words: vec!["smite".into(), "purify".into(), "judgment".into()],
            flavor_text: "Light writes its verdict upon the wicked.".into(),
        });

        // ═══════════════════════════════════════════════
        // SHADOW SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("shadow_bolt".into(), Spell {
            id: "shadow_bolt".into(),
            name: "Shadow Bolt".into(),
            description: "A bolt of pure darkness.".into(),
            element: Element::Shadow,
            tier: SpellTier::Apprentice,
            mana_cost: 12,
            cooldown_turns: 1,
            effect: SpellEffect::Damage { amount: 28, piercing: false },
            cast_words: vec!["shadow".into(), "darkness".into(), "umbra".into()],
            flavor_text: "Words written in the absence of light.".into(),
        });

        spells.insert("life_drain".into(), Spell {
            id: "life_drain".into(),
            name: "Life Drain".into(),
            description: "Steals life force from the target.".into(),
            element: Element::Shadow,
            tier: SpellTier::Journeyman,
            mana_cost: 20,
            cooldown_turns: 2,
            effect: SpellEffect::LifeSteal { damage: 25, heal_percent: 0.5 },
            cast_words: vec!["drain".into(), "siphon".into(), "leech".into()],
            flavor_text: "What is taken is also given.".into(),
        });

        spells.insert("execute".into(), Spell {
            id: "execute".into(),
            name: "Execute".into(),
            description: "Massive damage to weakened enemies.".into(),
            element: Element::Shadow,
            tier: SpellTier::Expert,
            mana_cost: 25,
            cooldown_turns: 3,
            effect: SpellEffect::Execute { threshold_percent: 0.3, damage: 80 },
            cast_words: vec!["execute".into(), "terminate".into(), "annihilate".into()],
            flavor_text: "The final word in any argument.".into(),
        });

        // ═══════════════════════════════════════════════
        // ARCANE SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("arcane_missile".into(), Spell {
            id: "arcane_missile".into(),
            name: "Arcane Missile".into(),
            description: "Pure magical energy.".into(),
            element: Element::Arcane,
            tier: SpellTier::Novice,
            mana_cost: 6,
            cooldown_turns: 0,
            effect: SpellEffect::Damage { amount: 12, piercing: false },
            cast_words: vec!["arcane".into(), "magic".into(), "power".into()],
            flavor_text: "The simplest expression of magical will.".into(),
        });

        spells.insert("clarity".into(), Spell {
            id: "clarity".into(),
            name: "Clarity".into(),
            description: "Reveals the next word clearly.".into(),
            element: Element::Arcane,
            tier: SpellTier::Apprentice,
            mana_cost: 10,
            cooldown_turns: 2,
            effect: SpellEffect::WordManipulation(WordManipEffect::RevealWord),
            cast_words: vec!["clarity".into(), "insight".into(), "vision".into()],
            flavor_text: "See the words as they truly are.".into(),
        });

        spells.insert("time_warp".into(), Spell {
            id: "time_warp".into(),
            name: "Time Warp".into(),
            description: "Extends your typing time.".into(),
            element: Element::Arcane,
            tier: SpellTier::Journeyman,
            mana_cost: 18,
            cooldown_turns: 3,
            effect: SpellEffect::Buff { stat: BuffStat::TimeLimit, amount: 30, duration_turns: 3 },
            cast_words: vec!["time".into(), "temporal".into(), "chronos".into()],
            flavor_text: "Time bends to those who master its words.".into(),
        });

        spells.insert("word_simplify".into(), Spell {
            id: "word_simplify".into(),
            name: "Word Simplify".into(),
            description: "Makes the next word easier to type.".into(),
            element: Element::Arcane,
            tier: SpellTier::Apprentice,
            mana_cost: 12,
            cooldown_turns: 2,
            effect: SpellEffect::WordManipulation(WordManipEffect::SimplifyNextWord),
            cast_words: vec!["simplify".into(), "reduce".into(), "shorten".into()],
            flavor_text: "Complexity yields to the focused mind.".into(),
        });

        // ═══════════════════════════════════════════════
        // VOID SPELLS
        // ═══════════════════════════════════════════════

        spells.insert("void_touch".into(), Spell {
            id: "void_touch".into(),
            name: "Void Touch".into(),
            description: "Damage that ignores all defenses.".into(),
            element: Element::Void,
            tier: SpellTier::Expert,
            mana_cost: 30,
            cooldown_turns: 3,
            effect: SpellEffect::Damage { amount: 50, piercing: true },
            cast_words: vec!["void".into(), "nothing".into(), "null".into()],
            flavor_text: "Where the void touches, nothing remains.".into(),
        });

        spells.insert("entropy".into(), Spell {
            id: "entropy".into(),
            name: "Entropy".into(),
            description: "Scrambles the enemy's next word.".into(),
            element: Element::Void,
            tier: SpellTier::Expert,
            mana_cost: 22,
            cooldown_turns: 3,
            effect: SpellEffect::WordManipulation(WordManipEffect::ScrambleEnemyWord),
            cast_words: vec!["entropy".into(), "chaos".into(), "disorder".into()],
            flavor_text: "Order is merely a temporary state.".into(),
        });

        spells.insert("oblivion".into(), Spell {
            id: "oblivion".into(),
            name: "Oblivion".into(),
            description: "The ultimate void spell.".into(),
            element: Element::Void,
            tier: SpellTier::Master,
            mana_cost: 50,
            cooldown_turns: 5,
            effect: SpellEffect::MultiEffect(vec![
                SpellEffect::Damage { amount: 100, piercing: true },
                SpellEffect::WordManipulation(WordManipEffect::SkipWord),
            ]),
            cast_words: vec!["oblivion".into(), "annihilation".into(), "nonexistence".into()],
            flavor_text: "The last word. The only word. Silence.".into(),
        });

        Self { spells }
    }

    pub fn get_spell(&self, id: &str) -> Option<&Spell> {
        self.spells.get(id)
    }

    pub fn get_spells_by_element(&self, element: Element) -> Vec<&Spell> {
        self.spells.values().filter(|s| s.element == element).collect()
    }

    pub fn get_spells_by_tier(&self, tier: SpellTier) -> Vec<&Spell> {
        self.spells.values().filter(|s| s.tier == tier).collect()
    }

    pub fn get_affordable_spells(&self, mana: i32) -> Vec<&Spell> {
        self.spells.values().filter(|s| s.mana_cost <= mana).collect()
    }
}
