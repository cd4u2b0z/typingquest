//! Enemy data structures and definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database of all enemy types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyDatabase {
    pub enemies: HashMap<String, EnemyTemplate>,
    pub bosses: HashMap<String, BossTemplate>,
}

/// Template for spawning enemies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub base_hp: i32,
    pub base_damage: i32,
    pub base_defense: i32,
    pub xp_reward: u64,
    pub gold_reward: i32,
    pub difficulty_tier: u32,  // 1-10
    pub typing_theme: String,  // Which word theme to use
    pub ascii_art: String,
    pub attack_messages: Vec<String>,
    pub death_message: String,
    pub special_ability: Option<SpecialAbility>,
}

/// Boss-specific template with phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub lore: String,
    pub base_hp: i32,
    pub base_damage: i32,
    pub base_defense: i32,
    pub xp_reward: u64,
    pub gold_reward: i32,
    pub phases: Vec<BossPhase>,
    pub ascii_art: String,
    pub intro_dialogue: Vec<String>,
    pub phase_transition_dialogue: Vec<String>,
    pub death_dialogue: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    pub hp_threshold: f32,  // Triggers when HP drops below this percentage
    pub name: String,
    pub damage_modifier: f32,
    pub speed_modifier: f32,
    pub special_ability: Option<SpecialAbility>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialAbility {
    /// Scrambles the letters in the current word
    WordScramble,
    /// Reduces time limit
    TimeWarp { reduction: f32 },
    /// Heals a percentage of max HP
    Regenerate { percent: f32 },
    /// Adds extra characters to type
    Corruption { extra_chars: u32 },
    /// Temporarily blinds (hides) part of the word
    Blind { duration: f32 },
    /// Reverses the word
    Mirror,
    /// Summons minion enemies
    Summon { enemy_id: String, count: u32 },
    /// Increases own damage temporarily
    Enrage { damage_mult: f32, duration: f32 },
}

impl Default for EnemyDatabase {
    fn default() -> Self {
        Self::embedded()
    }
}

impl EnemyDatabase {
    pub fn get_enemy(&self, id: &str) -> Option<&EnemyTemplate> {
        self.enemies.get(id)
    }
    
    pub fn get_boss(&self, id: &str) -> Option<&BossTemplate> {
        self.bosses.get(id)
    }
    
    pub fn get_enemies_by_tier(&self, tier: u32) -> Vec<&EnemyTemplate> {
        self.enemies.values()
            .filter(|e| e.difficulty_tier == tier)
            .collect()
    }
    
    pub fn embedded() -> Self {
        let mut enemies = HashMap::new();
        let mut bosses = HashMap::new();
        
        // === TIER 1: Tutorial/Easy ===
        enemies.insert("typo_gremlin".to_string(), EnemyTemplate {
            id: "typo_gremlin".to_string(),
            name: "Goblin Lurker".to_string(),
            description: "A small, wretched creature that lurks in dark corners.".to_string(),
            base_hp: 20,
            base_damage: 5,
            base_defense: 0,
            xp_reward: 10,
            gold_reward: 5,
            difficulty_tier: 1,
            typing_theme: "easy".to_string(),
            ascii_art: r#"
   \o/
    |
   / \
"#.to_string(),
            attack_messages: vec![
                "The gremlin giggles and throws a typo at you!".to_string(),
                "It scribbles errors in the air!".to_string(),
            ],
            death_message: "The goblin falls with a pitiful screech.".to_string(),
            special_ability: None,
        });
        
        enemies.insert("word_wisp".to_string(), EnemyTemplate {
            id: "word_wisp".to_string(),
            name: "Spectral Wisp".to_string(),
            description: "A shimmering spirit of the restless dead.".to_string(),
            base_hp: 15,
            base_damage: 8,
            base_defense: 0,
            xp_reward: 12,
            gold_reward: 7,
            difficulty_tier: 1,
            typing_theme: "magic".to_string(),
            ascii_art: r#"
  * . *
 . o .
  * *
"#.to_string(),
            attack_messages: vec![
                "The wisp flickers with malevolent light!".to_string(),
                "Ghostly letters swirl around you!".to_string(),
            ],
            death_message: "The wisp dissipates into ethereal mist.".to_string(),
            special_ability: None,
        });
        
        // === TIER 2-3: Early Game ===
        enemies.insert("syntax_spider".to_string(), EnemyTemplate {
            id: "syntax_spider".to_string(),
            name: "Venomous Spider".to_string(),
            description: "A giant arachnid with venom-dripping fangs.".to_string(),
            base_hp: 35,
            base_damage: 12,
            base_defense: 2,
            xp_reward: 25,
            gold_reward: 15,
            difficulty_tier: 2,
            typing_theme: "technology".to_string(),
            ascii_art: r#"
  /\  /\
 /  \/  \
 \  /\  /
  \/  \/
"#.to_string(),
            attack_messages: vec![
                "The spider shoots a web of semicolons!".to_string(),
                "It tangles you in nested parentheses!".to_string(),
            ],
            death_message: "The spider curls and goes still.".to_string(),
            special_ability: Some(SpecialAbility::Corruption { extra_chars: 2 }),
        });
        
        enemies.insert("vowel_vampire".to_string(), EnemyTemplate {
            id: "vowel_vampire".to_string(),
            name: "Lesser Vampire".to_string(),
            description: "An undead creature that thirsts for mortal essence.".to_string(),
            base_hp: 45,
            base_damage: 15,
            base_defense: 3,
            xp_reward: 35,
            gold_reward: 20,
            difficulty_tier: 3,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
   ___
  /   \
 | O O |
  \   /
   \_/
"#.to_string(),
            attack_messages: vec![
                "The vampire hisses, stealing your vowels!".to_string(),
                "It bites into your text hungrily!".to_string(),
            ],
            death_message: "The vampire crumbles to ash and bone.".to_string(),
            special_ability: Some(SpecialAbility::Blind { duration: 2.0 }),
        });
        
        // === TIER 4-5: Mid Game ===
        enemies.insert("corrupted_typer".to_string(), EnemyTemplate {
            id: "corrupted_typer".to_string(),
            name: "Blighted Thrall".to_string(),
            description: "A shambling corpse corrupted by dark magic.".to_string(),
            base_hp: 60,
            base_damage: 18,
            base_defense: 5,
            xp_reward: 50,
            gold_reward: 30,
            difficulty_tier: 4,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
   _____
  /     \
 |  x x  |
 |  ~~~  |
  \_____/
   |   |
"#.to_string(),
            attack_messages: vec![
                "The corrupted typer hammers keys randomly!".to_string(),
                "Gibberish streams from their broken fingers!".to_string(),
            ],
            death_message: "The thrall crumbles, finally at peace".to_string(),
            special_ability: Some(SpecialAbility::WordScramble),
        });
        
        enemies.insert("meaning_eater".to_string(), EnemyTemplate {
            id: "meaning_eater".to_string(),
            name: "Soul Devourer".to_string(),
            description: "A fiend that feeds on the souls of the fallen.".to_string(),
            base_hp: 75,
            base_damage: 20,
            base_defense: 6,
            xp_reward: 65,
            gold_reward: 40,
            difficulty_tier: 5,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
  @@@@@@@
 @  O O  @
 @ \___/ @
  @@@@@@@
    VVV
"#.to_string(),
            attack_messages: vec![
                "The creature opens its maw to consume your words!".to_string(),
                "Your sentences feel hollow as meaning drains away!".to_string(),
            ],
            death_message: "The devourer releases its stolen souls in a blinding flash.".to_string(),
            special_ability: Some(SpecialAbility::TimeWarp { reduction: 3.0 }),
        });
        
        // === TIER 6-7: Late Game ===
        enemies.insert("grammar_golem".to_string(), EnemyTemplate {
            id: "grammar_golem".to_string(),
            name: "Stone Golem".to_string(),
            description: "A massive construct of animated stone and ancient magic.".to_string(),
            base_hp: 100,
            base_damage: 25,
            base_defense: 10,
            xp_reward: 85,
            gold_reward: 55,
            difficulty_tier: 6,
            typing_theme: "ancient".to_string(),
            ascii_art: r#"
   [===]
  /|   |\
 / | O | \
   |===|
   /   \
"#.to_string(),
            attack_messages: vec![
                "The golem swings a fist of fossilized footnotes!".to_string(),
                "Ancient grammatical rules crash down upon you!".to_string(),
            ],
            death_message: "The golem crumbles into inert rubble.".to_string(),
            special_ability: Some(SpecialAbility::Enrage { damage_mult: 1.5, duration: 5.0 }),
        });
        
        enemies.insert("void_scribe".to_string(), EnemyTemplate {
            id: "void_scribe".to_string(),
            name: "Void Walker".to_string(),
            description: "A traveler between worlds, touched by the Void.".to_string(),
            base_hp: 90,
            base_damage: 30,
            base_defense: 8,
            xp_reward: 100,
            gold_reward: 65,
            difficulty_tier: 7,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
    ___
   /   \
  | . . |
   \___/
  /|   |\
 / |   | \
"#.to_string(),
            attack_messages: vec![
                "The scribe writes your doom in invisible ink!".to_string(),
                "Words appear and vanish simultaneously!".to_string(),
            ],
            death_message: "The walker fades back into the darkness.".to_string(),
            special_ability: Some(SpecialAbility::Mirror),
        });
        
        // === TIER 8-10: Endgame ===
        enemies.insert("entropy_weaver".to_string(), EnemyTemplate {
            id: "entropy_weaver".to_string(),
            name: "Shadow Weaver".to_string(),
            description: "A dark sorcerer that weaves deadly shadow magic.".to_string(),
            base_hp: 120,
            base_damage: 35,
            base_defense: 12,
            xp_reward: 120,
            gold_reward: 80,
            difficulty_tier: 8,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
   \~/\~/
    \  /
     \/
    /  \
   /~/\~\
"#.to_string(),
            attack_messages: vec![
                "Reality unravels at the seams!".to_string(),
                "Your words tangle into meaningless threads!".to_string(),
            ],
            death_message: "The weaver's shadows disperse into nothing.".to_string(),
            special_ability: Some(SpecialAbility::WordScramble),
        });
        
        enemies.insert("paragraph_phantom".to_string(), EnemyTemplate {
            id: "paragraph_phantom".to_string(),
            name: "Wailing Wraith".to_string(),
            description: "The tormented spirit of one who died in anguish.".to_string(),
            base_hp: 110,
            base_damage: 28,
            base_defense: 15,
            xp_reward: 110,
            gold_reward: 75,
            difficulty_tier: 8,
            typing_theme: "ancient".to_string(),
            ascii_art: r#"
  ╔═════╗
  ║~~~~~║
  ║~~~~~║
  ║~~~~~║
  ╚═════╝
"#.to_string(),
            attack_messages: vec![
                "Ghostly paragraphs surge toward you!".to_string(),
                "The phantom rewrites your fate!".to_string(),
            ],
            death_message: "The wraith fades with a final mournful wail.".to_string(),
            special_ability: Some(SpecialAbility::Blind { duration: 3.0 }),
        });
        
        enemies.insert("lexicon_leviathan".to_string(), EnemyTemplate {
            id: "lexicon_leviathan".to_string(),
            name: "Ancient Wyrm".to_string(),
            description: "An ancient dragon-kin of immense power.".to_string(),
            base_hp: 150,
            base_damage: 40,
            base_defense: 18,
            xp_reward: 150,
            gold_reward: 100,
            difficulty_tier: 9,
            typing_theme: "ancient".to_string(),
            ascii_art: r#"
    ___===___
   /  WORDS  \
  | WORDS WORDS|
   \__WORDS__/
      |  |
"#.to_string(),
            attack_messages: vec![
                "The leviathan speaks in tongues long dead!".to_string(),
                "A tidal wave of definitions crashes down!".to_string(),
            ],
            death_message: "The wyrm crashes down, its reign ended.".to_string(),
            special_ability: Some(SpecialAbility::Summon { enemy_id: "word_wisp".to_string(), count: 2 }),
        });
        
        enemies.insert("silence_incarnate".to_string(), EnemyTemplate {
            id: "silence_incarnate".to_string(),
            name: "Death Knight".to_string(),
            description: "An undead warrior of terrible skill and cold resolve.".to_string(),
            base_hp: 140,
            base_damage: 45,
            base_defense: 20,
            xp_reward: 180,
            gold_reward: 120,
            difficulty_tier: 10,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
      
   [     ]
   [     ]
   [     ]
      
"#.to_string(),
            attack_messages: vec![
                "...".to_string(),
                "The silence is deafening.".to_string(),
            ],
            death_message: "The knight falls, armor clattering.".to_string(),
            special_ability: Some(SpecialAbility::TimeWarp { reduction: 5.0 }),
        });
        
        // === BOSSES ===
        bosses.insert("corruption_elemental".to_string(), BossTemplate {
            id: "corruption_elemental".to_string(),
            name: "Blight Elemental".to_string(),
            title: "Herald of the Unwriting".to_string(),
            description: "A being of pure corruption and decay.".to_string(),
            lore: "Born from the first fragments of the Unwriting, this elemental \
                   seeks to spread corruption through all written language.".to_string(),
            base_hp: 200,
            base_damage: 25,
            base_defense: 10,
            xp_reward: 300,
            gold_reward: 150,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Gathering".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: None,
                },
                BossPhase {
                    hp_threshold: 0.5,
                    name: "Frenzied".to_string(),
                    damage_modifier: 1.5,
                    speed_modifier: 1.3,
                    special_ability: Some(SpecialAbility::WordScramble),
                },
                BossPhase {
                    hp_threshold: 0.25,
                    name: "Desperate".to_string(),
                    damage_modifier: 2.0,
                    speed_modifier: 1.5,
                    special_ability: Some(SpecialAbility::Corruption { extra_chars: 3 }),
                },
            ],
            ascii_art: r#"
      ╔═══╗
    ╔═╝   ╚═╗
   ═╝ ◊   ◊ ╚═
    ║   ▼   ║
   ═╗ ~~~~~ ╔═
    ╚═╗   ╔═╝
      ╚═══╝
"#.to_string(),
            intro_dialogue: vec![
                "Yooouur woooords... wiiiill... faaaaade...".to_string(),
                "The silence... it hungers... for meaning...".to_string(),
            ],
            phase_transition_dialogue: vec![
                "YESSS... THE VOID... GROWS STRONGER...".to_string(),
                "YOU CANNOT... TYPE... FAST ENOUGH...".to_string(),
            ],
            death_dialogue: vec![
                "The... words... they... return...".to_string(),
                "*The corruption dissipates, meaning restored*".to_string(),
            ],
        });
        
        bosses.insert("the_unwriter".to_string(), BossTemplate {
            id: "the_unwriter".to_string(),
            name: "The Void Herald".to_string(),
            title: "Entropy's Avatar".to_string(),
            description: "The harbinger of the Void, speaker of the end times.".to_string(),
            lore: "In the beginning was the Word. The Unwriter seeks the end. \
                   It is not evil—it is simply the cessation of all meaning, \
                   the final period at the end of reality's sentence.".to_string(),
            base_hp: 500,
            base_damage: 40,
            base_defense: 20,
            xp_reward: 1000,
            gold_reward: 500,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Manifesting".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: Some(SpecialAbility::TimeWarp { reduction: 2.0 }),
                },
                BossPhase {
                    hp_threshold: 0.75,
                    name: "Awakened".to_string(),
                    damage_modifier: 1.3,
                    speed_modifier: 1.2,
                    special_ability: Some(SpecialAbility::WordScramble),
                },
                BossPhase {
                    hp_threshold: 0.5,
                    name: "Ascendant".to_string(),
                    damage_modifier: 1.6,
                    speed_modifier: 1.4,
                    special_ability: Some(SpecialAbility::Blind { duration: 3.0 }),
                },
                BossPhase {
                    hp_threshold: 0.25,
                    name: "Absolute".to_string(),
                    damage_modifier: 2.0,
                    speed_modifier: 1.8,
                    special_ability: Some(SpecialAbility::Corruption { extra_chars: 5 }),
                },
            ],
            ascii_art: r#"
          ████████
        ██        ██
      ██   ◆    ◆   ██
     ██              ██
    ██    ╔══════╗    ██
    ██    ║VOID  ║    ██
    ██    ╚══════╝    ██
     ██     ~~~~     ██
      ██            ██
        ██        ██
          ████████
"#.to_string(),
            intro_dialogue: vec![
                "I am the silence between words.".to_string(),
                "I am the void where meaning dies.".to_string(),
                "I am the Unwriter. And you... are already forgotten.".to_string(),
            ],
            phase_transition_dialogue: vec![
                "Your words are hollow echoes in an empty universe.".to_string(),
                "Every keystroke feeds the void. Continue. Please.".to_string(),
                "There is no victory here. Only delayed entropy.".to_string(),
            ],
            death_dialogue: vec![
                "You... have typed... the impossible...".to_string(),
                "But know this... I am inevitable...".to_string(),
                "The final word... will always... be... silence...".to_string(),
                "*Reality stabilizes. The First Library remembers.*".to_string(),
            ],
        });
        
        Self { enemies, bosses }
    }
}
