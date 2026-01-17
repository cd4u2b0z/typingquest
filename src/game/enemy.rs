//! Enemy definitions - Data-driven with Undertale/Earthbound flair!

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;
use std::sync::Arc;
use crate::data::{GameData, enemies::EnemyTemplate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub max_hp: i32,
    pub current_hp: i32,
    pub attack_power: i32,
    pub defense: i32,
    pub xp_reward: i32,
    pub gold_reward: i32,
    pub enemy_type: EnemyType,
    pub ascii_art: String,
    pub battle_cry: String,
    pub defeat_message: String,
    pub spare_condition: Option<String>,
    pub is_boss: bool,
    pub typing_theme: String,
    pub attack_messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyType {
    Normal,
    Elite,
    Boss,
}

impl Enemy {
    /// Create an enemy from a data template, scaled for floor
    pub fn from_template(template: &EnemyTemplate, floor: i32) -> Self {
        let scale = 1.0 + (floor as f32 - 1.0) * 0.1;
        Self {
            name: template.name.clone(),
            max_hp: (template.base_hp as f32 * scale) as i32,
            current_hp: (template.base_hp as f32 * scale) as i32,
            attack_power: (template.base_damage as f32 * scale) as i32,
            defense: (template.base_defense as f32 * scale) as i32,
            xp_reward: (template.xp_reward as f32 * scale) as i32,
            gold_reward: (template.gold_reward as f32 * scale) as i32,
            enemy_type: EnemyType::Normal,
            ascii_art: template.ascii_art.clone(),
            battle_cry: format!("* {} blocks your path!", template.name),
            defeat_message: template.death_message.clone(),
            spare_condition: None,
            is_boss: false,
            typing_theme: template.typing_theme.clone(),
            attack_messages: template.attack_messages.clone(),
        }
    }

    /// Spawn a random enemy appropriate for the floor using GameData
    pub fn random_for_floor_data(game_data: &GameData, floor: i32) -> Self {
        let tier = ((floor - 1) / 2 + 1).clamp(1, 7) as u32;
        let enemies = game_data.enemies.get_enemies_by_tier(tier);
        
        if enemies.is_empty() {
            // Fallback to hardcoded if no data
            return Self::random_for_floor(floor);
        }
        
        let mut rng = rand::thread_rng();
        let template = enemies.choose(&mut rng).unwrap();
        Self::from_template(template, floor)
    }

    /// Spawn an elite enemy using GameData
    pub fn random_elite_data(game_data: &GameData, floor: i32) -> Self {
        let mut enemy = Self::random_for_floor_data(game_data, floor);
        enemy.name = format!("Elite {}", enemy.name);
        enemy.max_hp = (enemy.max_hp as f32 * 1.5) as i32;
        enemy.current_hp = enemy.max_hp;
        enemy.attack_power = (enemy.attack_power as f32 * 1.3) as i32;
        enemy.xp_reward = (enemy.xp_reward as f32 * 2.0) as i32;
        enemy.gold_reward = (enemy.gold_reward as f32 * 2.0) as i32;
        enemy.enemy_type = EnemyType::Elite;
        enemy
    }

    /// Spawn a boss using GameData
    pub fn random_boss_data(game_data: &GameData, floor: i32) -> Self {
        let bosses: Vec<_> = game_data.enemies.bosses.values().collect();
        
        if bosses.is_empty() {
            return Self::random_boss(floor);
        }
        
        let mut rng = rand::thread_rng();
        let boss = bosses.choose(&mut rng).unwrap();
        let scale = 1.0 + (floor as f32 - 1.0) * 0.15;
        
        Self {
            name: boss.name.clone(),
            max_hp: (boss.base_hp as f32 * scale) as i32,
            current_hp: (boss.base_hp as f32 * scale) as i32,
            attack_power: (boss.base_damage as f32 * scale) as i32,
            defense: (boss.base_defense as f32 * scale) as i32,
            xp_reward: (boss.xp_reward as f32 * scale) as i32,
            gold_reward: (boss.gold_reward as f32 * scale) as i32,
            enemy_type: EnemyType::Boss,
            ascii_art: boss.ascii_art.clone(),
            battle_cry: boss.intro_dialogue.first()
                .cloned()
                .unwrap_or_else(|| format!("* {} awakens!", boss.name)),
            defeat_message: boss.death_dialogue.last()
                .cloned()
                .unwrap_or_else(|| format!("* {} has been defeated!", boss.name)),
            spare_condition: None,
            is_boss: true,
            typing_theme: "corruption".to_string(),
            attack_messages: boss.phase_transition_dialogue.clone(),
        }
    }

    // === Legacy methods for backwards compatibility ===
    
    pub fn random_for_floor(floor: i32) -> Self {
        let mut rng = rand::thread_rng();
        let pool = Self::get_enemy_pool(floor);
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn random_elite(floor: i32) -> Self {
        let mut enemy = Self::random_for_floor(floor);
        enemy.name = format!("Elite {}", enemy.name);
        enemy.max_hp = (enemy.max_hp as f32 * 1.5) as i32;
        enemy.current_hp = enemy.max_hp;
        enemy.attack_power = (enemy.attack_power as f32 * 1.3) as i32;
        enemy.xp_reward = (enemy.xp_reward as f32 * 2.0) as i32;
        enemy.gold_reward = (enemy.gold_reward as f32 * 2.0) as i32;
        enemy.enemy_type = EnemyType::Elite;
        enemy
    }

    pub fn random_boss(floor: i32) -> Self {
        let mut rng = rand::thread_rng();
        let pool = Self::get_boss_pool(floor);
        pool.choose(&mut rng).unwrap().clone()
    }

    pub fn get_attack_message(&self) -> &str {
        if !self.attack_messages.is_empty() {
            let mut rng = rand::thread_rng();
            return self.attack_messages.choose(&mut rng)
                .map(|s| s.as_str())
                .unwrap_or("attacks");
        }
        
        let messages = [
            "attacks",
            "strikes",
            "hits you",
            "lunges at you",
        ];
        let mut rng = rand::thread_rng();
        messages.choose(&mut rng).unwrap()
    }

    fn get_enemy_pool(floor: i32) -> Vec<Self> {
        let base_enemies = vec![
            Enemy {
                name: "Typo Gremlin".to_string(),
                max_hp: 30,
                current_hp: 30,
                attack_power: 5,
                defense: 0,
                xp_reward: 15,
                gold_reward: 10,
                enemy_type: EnemyType::Normal,
                ascii_art: "  ╭─╮\n  (o.o)\n  /|_|\\".to_string(),
                battle_cry: "* Yuor wrods are mnie!".to_string(),
                defeat_message: "* The gremlin dissolves into misspelled letters...".to_string(),
                spare_condition: Some("Type 'sorry' to apologize for good typing".to_string()),
                is_boss: false,
                typing_theme: "easy".to_string(),
                attack_messages: vec!["throws a typo at you".to_string()],
            },
            Enemy {
                name: "Keyboard Slime".to_string(),
                max_hp: 40,
                current_hp: 40,
                attack_power: 4,
                defense: 2,
                xp_reward: 12,
                gold_reward: 8,
                enemy_type: EnemyType::Normal,
                ascii_art: "  ~~~~\n (o  o)\n  ~~~~".to_string(),
                battle_cry: "* squelch squelch".to_string(),
                defeat_message: "* The slime evaporates, leaving behind keycaps.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "easy".to_string(),
                attack_messages: vec!["oozes onto your fingers".to_string()],
            },
            Enemy {
                name: "Caps Lock Ghost".to_string(),
                max_hp: 25,
                current_hp: 25,
                attack_power: 7,
                defense: 0,
                xp_reward: 18,
                gold_reward: 12,
                enemy_type: EnemyType::Normal,
                ascii_art: " .---.\n | A |\n '---'".to_string(),
                battle_cry: "* I WILL HAUNT YOUR KEYBOARD FOREVER!".to_string(),
                defeat_message: "* The ghost fades... but caps lock is still on.".to_string(),
                spare_condition: Some("Turn off caps lock".to_string()),
                is_boss: false,
                typing_theme: "easy".to_string(),
                attack_messages: vec!["SCREAMS AT YOU".to_string()],
            },
        ];

        let mid_enemies = vec![
            Enemy {
                name: "Syntax Error".to_string(),
                max_hp: 50,
                current_hp: 50,
                attack_power: 8,
                defense: 3,
                xp_reward: 25,
                gold_reward: 18,
                enemy_type: EnemyType::Normal,
                ascii_art: "  ERROR\n  [!?!]\n  -----".to_string(),
                battle_cry: "* Expected semicolon, found DEATH.".to_string(),
                defeat_message: "* The error has been handled gracefully.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "technology".to_string(),
                attack_messages: vec!["throws an exception".to_string()],
            },
            Enemy {
                name: "Autocorrect Demon".to_string(),
                max_hp: 45,
                current_hp: 45,
                attack_power: 6,
                defense: 2,
                xp_reward: 22,
                gold_reward: 15,
                enemy_type: EnemyType::Normal,
                ascii_art: "  /\\_/\\\n ( -.- )\n  > ^ <".to_string(),
                battle_cry: "* Did you mean: SUFFER?".to_string(),
                defeat_message: "* *ducking finally*".to_string(),
                spare_condition: Some("Type exactly what it suggests".to_string()),
                is_boss: false,
                typing_theme: "corruption".to_string(),
                attack_messages: vec!["autocorrects your health to zero".to_string()],
            },
        ];

        let hard_enemies = vec![
            Enemy {
                name: "Buffer Overflow".to_string(),
                max_hp: 70,
                current_hp: 70,
                attack_power: 12,
                defense: 5,
                xp_reward: 40,
                gold_reward: 30,
                enemy_type: EnemyType::Normal,
                ascii_art: " [████]\n [████]\n [FULL]".to_string(),
                battle_cry: "* SEGMENTATION FAULT INCOMING!".to_string(),
                defeat_message: "* Memory has been freed.".to_string(),
                spare_condition: None,
                is_boss: false,
                typing_theme: "technology".to_string(),
                attack_messages: vec!["overflows into your HP".to_string()],
            },
            Enemy {
                name: "Regex Nightmare".to_string(),
                max_hp: 60,
                current_hp: 60,
                attack_power: 15,
                defense: 3,
                xp_reward: 45,
                gold_reward: 35,
                enemy_type: EnemyType::Normal,
                ascii_art: " /.*?/\n |[^]+|\n \\w+\\d".to_string(),
                battle_cry: "* ^(?=.*pain)(?=.*suffering).*$".to_string(),
                defeat_message: "* Pattern matched: DEFEAT".to_string(),
                spare_condition: Some("Match the impossible pattern".to_string()),
                is_boss: false,
                typing_theme: "technology".to_string(),
                attack_messages: vec!["matches your soul with a greedy quantifier".to_string()],
            },
        ];

        match floor {
            1..=2 => base_enemies,
            3..=4 => [base_enemies, mid_enemies].concat(),
            5..=6 => mid_enemies,
            _ => [mid_enemies, hard_enemies].concat(),
        }
    }

    fn get_boss_pool(floor: i32) -> Vec<Self> {
        vec![
            Enemy {
                name: "The Backspace".to_string(),
                max_hp: 150,
                current_hp: 150,
                attack_power: 15,
                defense: 5,
                xp_reward: 100,
                gold_reward: 75,
                enemy_type: EnemyType::Boss,
                ascii_art: "  ╔═══╗\n  ║ ← ║\n  ╚═══╝".to_string(),
                battle_cry: "* I will erase everything you've ever typed!".to_string(),
                defeat_message: "* Delete... delete... del...".to_string(),
                spare_condition: Some("Delete yourself from existence".to_string()),
                is_boss: true,
                typing_theme: "corruption".to_string(),
                attack_messages: vec![
                    "erases part of your soul".to_string(),
                    "deletes your confidence".to_string(),
                ],
            },
            Enemy {
                name: "CTRL+Z, The Undoer".to_string(),
                max_hp: 180,
                current_hp: 180,
                attack_power: 12,
                defense: 8,
                xp_reward: 120,
                gold_reward: 90,
                enemy_type: EnemyType::Boss,
                ascii_art: "  ↺↺↺\n [UNDO]\n  ↻↻↻".to_string(),
                battle_cry: "* Your progress means nothing! I will undo it ALL!".to_string(),
                defeat_message: "* Cannot undo... action... permanent...".to_string(),
                spare_condition: None,
                is_boss: true,
                typing_theme: "corruption".to_string(),
                attack_messages: vec![
                    "reverts your healing".to_string(),
                    "undoes your last attack".to_string(),
                ],
            },
            Enemy {
                name: "The Unwriter".to_string(),
                max_hp: 300,
                current_hp: 300,
                attack_power: 25,
                defense: 15,
                xp_reward: 500,
                gold_reward: 300,
                enemy_type: EnemyType::Boss,
                ascii_art: "    ████████\n  ██        ██\n ██   ◆  ◆   ██\n██     ▼     ██\n ██  ~~~~~  ██\n  ██      ██\n    ████████".to_string(),
                battle_cry: "* I am the silence between words. I am the void where meaning dies.".to_string(),
                defeat_message: "* The... words... they... return...".to_string(),
                spare_condition: None,
                is_boss: true,
                typing_theme: "corruption".to_string(),
                attack_messages: vec![
                    "erases meaning from the universe".to_string(),
                    "writes your doom in invisible ink".to_string(),
                    "speaks in the language of endings".to_string(),
                ],
            },
        ]
    }
}
