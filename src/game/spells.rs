//! Spells and magical abilities - cast by typing!

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellElement {
    Physical,
    Fire,
    Ice,
    Lightning,
    Arcane,
    Holy,
    Dark,
    Nature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellTarget {
    Enemy,
    Self_,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub element: SpellElement,
    pub target: SpellTarget,
    pub mp_cost: i32,
    pub base_power: i32,
    pub incantation: String,  // Word(s) to type to cast
    pub cast_time: f32,       // Time limit in seconds
    pub effect: SpellEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellEffect {
    Damage(i32),
    Heal(i32),
    Shield(i32),
    Buff { stat: String, amount: i32, duration: i32 },
    Debuff { stat: String, amount: i32, duration: i32 },
    Drain { damage: i32, heal_percent: i32 },
    Multi { hits: i32, damage_per_hit: i32 },
    Poison { damage: i32, duration: i32 },
    Stun { duration: i32 },
}

impl Spell {
    pub fn basic_attack() -> Self {
        Self {
            name: "Strike".to_string(),
            description: "A basic attack. Type the word to deal damage.".to_string(),
            element: SpellElement::Physical,
            target: SpellTarget::Enemy,
            mp_cost: 0,
            base_power: 10,
            incantation: "strike".to_string(),
            cast_time: 5.0,
            effect: SpellEffect::Damage(10),
        }
    }

    /// Get all learnable spells
    pub fn spell_book() -> Vec<Self> {
        vec![
            // Tier 1 - Basic
            Self {
                name: "Strike".to_string(),
                description: "A basic attack.".to_string(),
                element: SpellElement::Physical,
                target: SpellTarget::Enemy,
                mp_cost: 0,
                base_power: 10,
                incantation: "strike".to_string(),
                cast_time: 5.0,
                effect: SpellEffect::Damage(10),
            },
            Self {
                name: "Mend".to_string(),
                description: "Restore a small amount of HP.".to_string(),
                element: SpellElement::Holy,
                target: SpellTarget::Self_,
                mp_cost: 5,
                base_power: 15,
                incantation: "heal".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Heal(15),
            },
            Self {
                name: "Spark".to_string(),
                description: "A small bolt of lightning.".to_string(),
                element: SpellElement::Lightning,
                target: SpellTarget::Enemy,
                mp_cost: 3,
                base_power: 12,
                incantation: "zap".to_string(),
                cast_time: 3.0,
                effect: SpellEffect::Damage(12),
            },

            // Tier 2 - Intermediate
            Self {
                name: "Fireball".to_string(),
                description: "Hurl a ball of flame.".to_string(),
                element: SpellElement::Fire,
                target: SpellTarget::Enemy,
                mp_cost: 8,
                base_power: 25,
                incantation: "fireball".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Damage(25),
            },
            Self {
                name: "Frost".to_string(),
                description: "Freeze the enemy with bitter cold.".to_string(),
                element: SpellElement::Ice,
                target: SpellTarget::Enemy,
                mp_cost: 7,
                base_power: 18,
                incantation: "freeze".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Damage(18),
            },
            Self {
                name: "Barrier".to_string(),
                description: "Create a protective shield.".to_string(),
                element: SpellElement::Arcane,
                target: SpellTarget::Self_,
                mp_cost: 10,
                base_power: 20,
                incantation: "shield".to_string(),
                cast_time: 3.5,
                effect: SpellEffect::Shield(20),
            },
            Self {
                name: "Drain".to_string(),
                description: "Steal life from the enemy.".to_string(),
                element: SpellElement::Dark,
                target: SpellTarget::Enemy,
                mp_cost: 12,
                base_power: 15,
                incantation: "drain".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Drain { damage: 15, heal_percent: 50 },
            },

            // Tier 3 - Advanced
            Self {
                name: "Thunder".to_string(),
                description: "Call down a devastating lightning bolt.".to_string(),
                element: SpellElement::Lightning,
                target: SpellTarget::Enemy,
                mp_cost: 15,
                base_power: 40,
                incantation: "thunderbolt".to_string(),
                cast_time: 3.5,
                effect: SpellEffect::Damage(40),
            },
            Self {
                name: "Inferno".to_string(),
                description: "Engulf the enemy in flames.".to_string(),
                element: SpellElement::Fire,
                target: SpellTarget::Enemy,
                mp_cost: 18,
                base_power: 35,
                incantation: "incinerate".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Multi { hits: 3, damage_per_hit: 15 },
            },
            Self {
                name: "Restore".to_string(),
                description: "Fully restore your HP.".to_string(),
                element: SpellElement::Holy,
                target: SpellTarget::Self_,
                mp_cost: 25,
                base_power: 100,
                incantation: "restoration".to_string(),
                cast_time: 5.0,
                effect: SpellEffect::Heal(100),
            },
            Self {
                name: "Toxic".to_string(),
                description: "Poison the enemy over time.".to_string(),
                element: SpellElement::Nature,
                target: SpellTarget::Enemy,
                mp_cost: 10,
                base_power: 8,
                incantation: "poison".to_string(),
                cast_time: 3.5,
                effect: SpellEffect::Poison { damage: 8, duration: 5 },
            },

            // Tier 4 - Legendary
            Self {
                name: "Meteor".to_string(),
                description: "Call down a meteor from the heavens.".to_string(),
                element: SpellElement::Fire,
                target: SpellTarget::All,
                mp_cost: 40,
                base_power: 80,
                incantation: "apocalypse".to_string(),
                cast_time: 3.0,
                effect: SpellEffect::Damage(80),
            },
            Self {
                name: "Holy Light".to_string(),
                description: "Bathe yourself in divine light.".to_string(),
                element: SpellElement::Holy,
                target: SpellTarget::Self_,
                mp_cost: 30,
                base_power: 50,
                incantation: "sanctuary".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Heal(50),
            },
            Self {
                name: "Void".to_string(),
                description: "Tear a hole in reality itself.".to_string(),
                element: SpellElement::Dark,
                target: SpellTarget::Enemy,
                mp_cost: 35,
                base_power: 70,
                incantation: "oblivion".to_string(),
                cast_time: 3.0,
                effect: SpellEffect::Damage(70),
            },

            // Secret spells
            Self {
                name: "QWERTY Strike".to_string(),
                description: "Channel the power of the keyboard.".to_string(),
                element: SpellElement::Arcane,
                target: SpellTarget::Enemy,
                mp_cost: 20,
                base_power: 50,
                incantation: "qwertyuiop".to_string(),
                cast_time: 4.0,
                effect: SpellEffect::Multi { hits: 10, damage_per_hit: 5 },
            },
            Self {
                name: "Home Row".to_string(),
                description: "The true path of the typist.".to_string(),
                element: SpellElement::Holy,
                target: SpellTarget::Self_,
                mp_cost: 15,
                base_power: 30,
                incantation: "asdfghjkl".to_string(),
                cast_time: 3.5,
                effect: SpellEffect::Buff { stat: "dexterity".to_string(), amount: 10, duration: 5 },
            },
        ]
    }

    pub fn get_element_color(&self) -> &'static str {
        match self.element {
            SpellElement::Physical => "white",
            SpellElement::Fire => "red",
            SpellElement::Ice => "cyan",
            SpellElement::Lightning => "yellow",
            SpellElement::Arcane => "magenta",
            SpellElement::Holy => "white",
            SpellElement::Dark => "gray",
            SpellElement::Nature => "green",
        }
    }

    pub fn get_element_symbol(&self) -> &'static str {
        match self.element {
            SpellElement::Physical => "⚔",
            SpellElement::Fire => "🔥",
            SpellElement::Ice => "❄",
            SpellElement::Lightning => "⚡",
            SpellElement::Arcane => "✨",
            SpellElement::Holy => "✝",
            SpellElement::Dark => "☠",
            SpellElement::Nature => "🌿",
        }
    }
}
