//! Enemy definitions - Undertale/Earthbound inspired!

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyType {
    Normal,
    Elite,
    Boss,
}

impl Enemy {
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
                spare_condition: Some("Turn off caps lock (it is already off)".to_string()),
                is_boss: false,
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
            },
        ];

        let hard_enemies = vec![
            Enemy {
                name: "Buffer Overflow".to_string(),
                max_hp: 60,
                current_hp: 60,
                attack_power: 10,
                defense: 5,
                xp_reward: 35,
                gold_reward: 25,
                enemy_type: EnemyType::Normal,
                ascii_art: "[[[[[[\n BOOM \n]]]]]]".to_string(),
                battle_cry: "* AAAAAAAAAAAAAAAAAAAAAA".to_string(),
                defeat_message: "* Segmentation fault (core dumped)".to_string(),
                spare_condition: None,
                is_boss: false,
            },
            Enemy {
                name: "Legacy Code Lich".to_string(),
                max_hp: 70,
                current_hp: 70,
                attack_power: 9,
                defense: 6,
                xp_reward: 40,
                gold_reward: 30,
                enemy_type: EnemyType::Normal,
                ascii_art: "  ╭─────╮\n  │ OLD │\n  ╰─────╯".to_string(),
                battle_cry: "* This code has been here since 1987...".to_string(),
                defeat_message: "* // TODO: remove this in next refactor".to_string(),
                spare_condition: Some("Comment it out instead of deleting".to_string()),
                is_boss: false,
            },
        ];

        // Scale enemies based on floor
        match floor {
            1..=2 => base_enemies,
            3..=4 => [base_enemies, mid_enemies].concat(),
            5..=6 => mid_enemies,
            7..=8 => [mid_enemies, hard_enemies].concat(),
            _ => hard_enemies,
        }
    }

    fn get_boss_pool(_floor: i32) -> Vec<Self> {
        vec![
            Enemy {
                name: "THE GREAT QWERTY".to_string(),
                max_hp: 200,
                current_hp: 200,
                attack_power: 15,
                defense: 8,
                xp_reward: 150,
                gold_reward: 100,
                enemy_type: EnemyType::Boss,
                ascii_art: "╔═══════════╗\n║ Q W E R T ║\n╚═══════════╝".to_string(),
                battle_cry: "* THE KEYBOARD DEITY HAS AWAKENED!".to_string(),
                defeat_message: "* The ancient keyboard crumbles...".to_string(),
                spare_condition: Some("Type the entire alphabet perfectly".to_string()),
                is_boss: true,
            },
            Enemy {
                name: "CLIPPY THE FALLEN".to_string(),
                max_hp: 250,
                current_hp: 250,
                attack_power: 18,
                defense: 5,
                xp_reward: 200,
                gold_reward: 150,
                enemy_type: EnemyType::Boss,
                ascii_art: "  ╭───╮\n  │ ? │\n  ╰┬─┬╯\n   │ │".to_string(),
                battle_cry: "* Hi! I'm Clippy! Would you like help dying?".to_string(),
                defeat_message: "* Clippy has been... uninstalled.".to_string(),
                spare_condition: Some("Click 'Don't show me this again'".to_string()),
                is_boss: true,
            },
            Enemy {
                name: "THE FINAL COMPILE".to_string(),
                max_hp: 300,
                current_hp: 300,
                attack_power: 20,
                defense: 10,
                xp_reward: 300,
                gold_reward: 200,
                enemy_type: EnemyType::Boss,
                ascii_art: "╔══════════════╗\n║ COMPILING... ║\n║   99%...     ║\n╚══════════════╝".to_string(),
                battle_cry: "* Error on line 1... of your LIFE!".to_string(),
                defeat_message: "* Build successful. Finally.".to_string(),
                spare_condition: Some("Fix all 999 compiler errors".to_string()),
                is_boss: true,
            },
        ]
    }
}
