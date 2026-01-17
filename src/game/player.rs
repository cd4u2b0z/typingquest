//! Player character and progression

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::items::Item;
use super::spells::Spell;

/// Character classes with unique abilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Class {
    Wordsmith,      // Balanced, bonus to word combos
    Scribe,         // High accuracy bonuses, defensive
    Spellweaver,    // Powerful spells, low HP
    Barbarian,      // High HP, raw damage, typos hurt less
    Trickster,      // Random bonuses, chaos magic
}

impl Class {
    pub fn name(&self) -> &'static str {
        match self {
            Class::Wordsmith => "Wordsmith",
            Class::Scribe => "Scribe",
            Class::Spellweaver => "Spellweaver",
            Class::Barbarian => "Barbarian",
            Class::Trickster => "Trickster",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Class::Wordsmith => "Master of the written word. Combo bonuses are doubled.",
            Class::Scribe => "Precise and methodical. Perfect accuracy grants shields.",
            Class::Spellweaver => "Channels raw typing into magic. Spells cost less but fragile.",
            Class::Barbarian => "Types with fury! High HP, typos deal less self-damage.",
            Class::Trickster => "Chaos incarnate. Random effects on every word completed.",
        }
    }

    pub fn ascii_art(&self) -> &'static str {
        match self {
            Class::Wordsmith => r#"
    ___
   /   \
  | o o |
   \ = /    ✒️
    |||
   /|||\
  / ||| \
"#,
            Class::Scribe => r#"
    _____
   /     \
  |  ◉ ◉  |
   \ ___ /   📜
    |   |
   /|   |\
  /_|   |_\
"#,
            Class::Spellweaver => r#"
     /\
    /  \
   | ✦✦ |
    \__/    ✨
     ||
    /||\
   ⚡ || ⚡
"#,
            Class::Barbarian => r#"
   \!!!!/
    \  /
   |◣◢|
    \/ ̄    ⚔️
   /||\
  //||\\
 // || \\
"#,
            Class::Trickster => r#"
    ????
   / ?? \
  | ?  ? |
   \ ?? /   🎲
    |~~|
   /|  |\
  ? |  | ?
"#,
        }
    }

    pub fn base_hp(&self) -> i32 {
        match self {
            Class::Wordsmith => 100,
            Class::Scribe => 90,
            Class::Spellweaver => 70,
            Class::Barbarian => 150,
            Class::Trickster => 85,
        }
    }

    pub fn base_mp(&self) -> i32 {
        match self {
            Class::Wordsmith => 50,
            Class::Scribe => 60,
            Class::Spellweaver => 100,
            Class::Barbarian => 30,
            Class::Trickster => 70,
        }
    }
}

/// Player stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,      // Damage multiplier
    pub intellect: i32,     // Spell power & MP
    pub vitality: i32,      // HP & defense
    pub dexterity: i32,     // Accuracy bonus & crit chance
    pub luck: i32,          // Item find & random events
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: 10,
            intellect: 10,
            vitality: 10,
            dexterity: 10,
            luck: 10,
        }
    }
}

/// The player character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub level: u32,
    pub experience: u64,
    pub gold: u64,
    
    // Vitals
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub shield: i32,
    
    // Stats
    pub stats: Stats,
    
    // Progression
    pub floor: u32,
    pub rooms_cleared: u32,
    pub enemies_defeated: u32,
    pub words_typed: u64,
    pub perfect_words: u64,
    pub total_keystrokes: u64,
    pub best_wpm: f64,
    pub best_combo: u32,
    
    // Equipment & Inventory
    pub inventory: Vec<Item>,
    pub equipped: HashMap<String, Item>,
    pub known_spells: Vec<Spell>,
    pub active_spell: Option<usize>,
    
    // Status effects
    pub buffs: Vec<StatusEffect>,
    pub debuffs: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub name: String,
    pub description: String,
    pub turns_remaining: i32,
    pub effect_type: EffectType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    DamageBoost(f32),
    DefenseBoost(f32),
    Regeneration(i32),
    Poison(i32),
    Haste,           // More time to type
    Slow,            // Less time to type
    Confusion,       // Letters scrambled
    Blindness,       // Can't see some letters
}

impl Player {
    pub fn new(name: String, class: Class) -> Self {
        let max_hp = class.base_hp();
        let max_mp = class.base_mp();
        
        Self {
            name,
            class,
            level: 1,
            experience: 0,
            gold: 0,
            hp: max_hp,
            max_hp,
            mp: max_mp,
            max_mp,
            shield: 0,
            stats: Stats::default(),
            floor: 1,
            rooms_cleared: 0,
            enemies_defeated: 0,
            words_typed: 0,
            perfect_words: 0,
            total_keystrokes: 0,
            best_wpm: 0.0,
            best_combo: 0,
            inventory: Vec::new(),
            equipped: HashMap::new(),
            known_spells: vec![Spell::basic_attack()],
            active_spell: Some(0),
            buffs: Vec::new(),
            debuffs: Vec::new(),
        }
    }

    pub fn experience_to_next_level(&self) -> u64 {
        // Earthbound-style exponential curve
        (self.level as u64).pow(2) * 100
    }

    pub fn gain_experience(&mut self, amount: u64) -> bool {
        self.experience += amount;
        
        let needed = self.experience_to_next_level();
        if self.experience >= needed {
            self.experience -= needed;
            self.level_up();
            true
        } else {
            false
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        
        // Stat gains based on class
        match self.class {
            Class::Wordsmith => {
                self.stats.strength += 2;
                self.stats.intellect += 2;
                self.stats.vitality += 2;
                self.stats.dexterity += 2;
                self.stats.luck += 2;
            }
            Class::Scribe => {
                self.stats.strength += 1;
                self.stats.intellect += 3;
                self.stats.vitality += 2;
                self.stats.dexterity += 3;
                self.stats.luck += 1;
            }
            Class::Spellweaver => {
                self.stats.strength += 1;
                self.stats.intellect += 4;
                self.stats.vitality += 1;
                self.stats.dexterity += 2;
                self.stats.luck += 2;
            }
            Class::Barbarian => {
                self.stats.strength += 4;
                self.stats.intellect += 1;
                self.stats.vitality += 3;
                self.stats.dexterity += 1;
                self.stats.luck += 1;
            }
            Class::Trickster => {
                self.stats.strength += 2;
                self.stats.intellect += 2;
                self.stats.vitality += 1;
                self.stats.dexterity += 2;
                self.stats.luck += 3;
            }
        }
        
        // Recalculate max HP/MP
        self.max_hp = self.class.base_hp() + (self.stats.vitality * 5);
        self.max_mp = self.class.base_mp() + (self.stats.intellect * 3);
        
        // Full heal on level up!
        self.hp = self.max_hp;
        self.mp = self.max_mp;
    }

    pub fn take_damage(&mut self, amount: i32) -> i32 {
        // Shield absorbs first
        if self.shield > 0 {
            if self.shield >= amount {
                self.shield -= amount;
                return 0;
            } else {
                let remaining = amount - self.shield;
                self.shield = 0;
                self.hp -= remaining;
                return remaining;
            }
        }
        
        self.hp -= amount;
        amount
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn restore_mp(&mut self, amount: i32) {
        self.mp = (self.mp + amount).min(self.max_mp);
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn damage_multiplier(&self) -> f32 {
        let base = 1.0 + (self.stats.strength as f32 * 0.05);
        let buff_mult: f32 = self.buffs.iter()
            .filter_map(|b| match &b.effect_type {
                EffectType::DamageBoost(mult) => Some(*mult),
                _ => None,
            })
            .product();
        base * buff_mult.max(1.0)
    }

    pub fn defense_multiplier(&self) -> f32 {
        let base = 1.0 + (self.stats.vitality as f32 * 0.03);
        let buff_mult: f32 = self.buffs.iter()
            .filter_map(|b| match &b.effect_type {
                EffectType::DefenseBoost(mult) => Some(*mult),
                _ => None,
            })
            .product();
        base * buff_mult.max(1.0)
    }

    pub fn update_effects(&mut self) {
        // Tick down buffs
        self.buffs.retain_mut(|buff| {
            buff.turns_remaining -= 1;
            buff.turns_remaining > 0
        });
        
        // Tick down debuffs and apply effects
        let mut poison_damage = 0;
        let mut regen_amount = 0;
        
        self.debuffs.retain_mut(|debuff| {
            match &debuff.effect_type {
                EffectType::Poison(dmg) => poison_damage += dmg,
                _ => {}
            }
            debuff.turns_remaining -= 1;
            debuff.turns_remaining > 0
        });
        
        for buff in &self.buffs {
            if let EffectType::Regeneration(amount) = &buff.effect_type {
                regen_amount += amount;
            }
        }
        
        if poison_damage > 0 {
            self.hp -= poison_damage;
        }
        if regen_amount > 0 {
            self.heal(regen_amount);
        }
    }
}
