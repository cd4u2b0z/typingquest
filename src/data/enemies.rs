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

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: SILENT LIBRARY (Floors 1-5)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("dust_sprite".to_string(), EnemyTemplate {
            id: "dust_sprite".to_string(),
            name: "Dust Sprite".to_string(),
            description: "A tiny creature born from centuries of accumulated dust.".to_string(),
            base_hp: 12,
            base_damage: 4,
            base_defense: 0,
            xp_reward: 8,
            gold_reward: 3,
            difficulty_tier: 1,
            typing_theme: "library".to_string(),
            ascii_art: r#"
  · ∴ ·
   ◦
  · ∴ ·
"#.to_string(),
            attack_messages: vec![
                "The sprite scatters dust into your eyes!".to_string(),
                "A cloud of ancient particles swirls around you!".to_string(),
            ],
            death_message: "The sprite settles into stillness.".to_string(),
            special_ability: None,
        });

        enemies.insert("paper_phantom".to_string(), EnemyTemplate {
            id: "paper_phantom".to_string(),
            name: "Paper Phantom".to_string(),
            description: "A ghost formed from discarded manuscripts.".to_string(),
            base_hp: 18,
            base_damage: 7,
            base_defense: 0,
            xp_reward: 14,
            gold_reward: 8,
            difficulty_tier: 1,
            typing_theme: "library".to_string(),
            ascii_art: r#"
  ┌───┐
  │ ≋ │
  │≋≋≋│
  └─┬─┘
    │
"#.to_string(),
            attack_messages: vec![
                "Paper cuts slice through the air!".to_string(),
                "The phantom throws razor-sharp pages!".to_string(),
            ],
            death_message: "The phantom unfolds into blank pages.".to_string(),
            special_ability: None,
        });

        enemies.insert("ink_wraith".to_string(), EnemyTemplate {
            id: "ink_wraith".to_string(),
            name: "Ink Wraith".to_string(),
            description: "A malevolent spirit that oozes corrupted ink.".to_string(),
            base_hp: 25,
            base_damage: 10,
            base_defense: 1,
            xp_reward: 20,
            gold_reward: 12,
            difficulty_tier: 2,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
   ▓░▓
  ▓▓░▓▓
 ░▓▓▓▓▓░
  ▓▓▓▓▓
   ░▓░
"#.to_string(),
            attack_messages: vec![
                "Ink splatters across your vision!".to_string(),
                "The wraith smears darkness over your words!".to_string(),
            ],
            death_message: "The wraith dissolves into a puddle of ink.".to_string(),
            special_ability: Some(SpecialAbility::Blind { duration: 1.5 }),
        });

        enemies.insert("shelf_specter".to_string(), EnemyTemplate {
            id: "shelf_specter".to_string(),
            name: "Shelf Specter".to_string(),
            description: "The angry spirit of a librarian who died organizing books.".to_string(),
            base_hp: 30,
            base_damage: 8,
            base_defense: 3,
            xp_reward: 22,
            gold_reward: 15,
            difficulty_tier: 2,
            typing_theme: "library".to_string(),
            ascii_art: r#"
 ╔═══╗
 ║▒▒▒║
 ║≡≡≡║
 ╚═╦═╝
   ║
"#.to_string(),
            attack_messages: vec![
                "'QUIET!' The specter throws books at you!".to_string(),
                "The specter alphabetizes your pain!".to_string(),
            ],
            death_message: "'Return... your books...' it whispers, fading.".to_string(),
            special_ability: Some(SpecialAbility::WordScramble),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: FORGOTTEN ARCHIVES (Floors 6-10)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("frost_cipher".to_string(), EnemyTemplate {
            id: "frost_cipher".to_string(),
            name: "Frost Cipher".to_string(),
            description: "An ice elemental that speaks in encrypted cold.".to_string(),
            base_hp: 40,
            base_damage: 14,
            base_defense: 4,
            xp_reward: 35,
            gold_reward: 20,
            difficulty_tier: 3,
            typing_theme: "ice".to_string(),
            ascii_art: r#"
   ❄
  ╱▲╲
 ╱▲▲▲╲
 ╲▲▲▲╱
  ╲▼╱
"#.to_string(),
            attack_messages: vec![
                "Frozen letters shatter against you!".to_string(),
                "The cipher encodes your fingers in ice!".to_string(),
            ],
            death_message: "The cipher melts into cryptic puddles.".to_string(),
            special_ability: Some(SpecialAbility::TimeWarp { reduction: 1.0 }),
        });

        enemies.insert("sealed_secret".to_string(), EnemyTemplate {
            id: "sealed_secret".to_string(),
            name: "Sealed Secret".to_string(),
            description: "A sentient forbidden text that escaped its binding.".to_string(),
            base_hp: 50,
            base_damage: 18,
            base_defense: 5,
            xp_reward: 45,
            gold_reward: 30,
            difficulty_tier: 4,
            typing_theme: "forbidden".to_string(),
            ascii_art: r#"
 ╔═[X]═╗
 ║?????║
 ║█████║
 ║?????║
 ╚═════╝
"#.to_string(),
            attack_messages: vec![
                "Forbidden knowledge sears your mind!".to_string(),
                "The secret tries to rewrite your memories!".to_string(),
            ],
            death_message: "The secret reseals itself, dormant once more.".to_string(),
            special_ability: Some(SpecialAbility::Corruption { extra_chars: 3 }),
        });

        enemies.insert("archive_guardian".to_string(), EnemyTemplate {
            id: "archive_guardian".to_string(),
            name: "Archive Guardian".to_string(),
            description: "A stone construct that protects the sealed knowledge.".to_string(),
            base_hp: 70,
            base_damage: 20,
            base_defense: 8,
            xp_reward: 55,
            gold_reward: 35,
            difficulty_tier: 4,
            typing_theme: "ancient".to_string(),
            ascii_art: r#"
  ╔═══╗
  ║ ◊ ║
 ╔╩═══╩╗
 ║█████║
 ╚╦═══╦╝
  ║   ║
"#.to_string(),
            attack_messages: vec![
                "The guardian's stone fist descends!".to_string(),
                "Ancient wards crackle with energy!".to_string(),
            ],
            death_message: "The guardian crumbles, its duty finally ended.".to_string(),
            special_ability: Some(SpecialAbility::Regenerate { percent: 5.0 }),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: BURNING SCRIPTORIUM (Floors 11-15)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("ember_sprite".to_string(), EnemyTemplate {
            id: "ember_sprite".to_string(),
            name: "Ember Sprite".to_string(),
            description: "A living spark that dances on burning pages.".to_string(),
            base_hp: 35,
            base_damage: 16,
            base_defense: 2,
            xp_reward: 40,
            gold_reward: 22,
            difficulty_tier: 4,
            typing_theme: "fire".to_string(),
            ascii_art: r#"
   🔥
  ╱ ╲
 ( ◦ )
  ╲ ╱
"#.to_string(),
            attack_messages: vec![
                "The sprite hurls burning words!".to_string(),
                "Flames lick at your typing fingers!".to_string(),
            ],
            death_message: "The sprite flickers out with a sigh.".to_string(),
            special_ability: None,
        });

        enemies.insert("ash_wraith".to_string(), EnemyTemplate {
            id: "ash_wraith".to_string(),
            name: "Ash Wraith".to_string(),
            description: "The bitter remains of a scribe who burned with their work.".to_string(),
            base_hp: 55,
            base_damage: 22,
            base_defense: 4,
            xp_reward: 55,
            gold_reward: 35,
            difficulty_tier: 5,
            typing_theme: "fire".to_string(),
            ascii_art: r#"
  ░▒▓█▓▒░
   ▓███▓
  ░▓▓▓▓▓░
   ░▓▓▓░
    ░▓░
"#.to_string(),
            attack_messages: vec![
                "Ashes swirl into your lungs!".to_string(),
                "The wraith breathes cinders of lost knowledge!".to_string(),
            ],
            death_message: "The wraith finally finds rest in the flames.".to_string(),
            special_ability: Some(SpecialAbility::Blind { duration: 2.0 }),
        });

        enemies.insert("burning_tome".to_string(), EnemyTemplate {
            id: "burning_tome".to_string(),
            name: "Burning Tome".to_string(),
            description: "A forbidden book that set itself ablaze rather than be read.".to_string(),
            base_hp: 65,
            base_damage: 25,
            base_defense: 3,
            xp_reward: 60,
            gold_reward: 40,
            difficulty_tier: 5,
            typing_theme: "forbidden".to_string(),
            ascii_art: r#"
 🔥🔥🔥🔥🔥
 ╔═════╗
 ║~~~~~║
 ║█████║
 ╚═════╝
 🔥🔥🔥🔥🔥
"#.to_string(),
            attack_messages: vec![
                "Flaming pages fly at your face!".to_string(),
                "The tome screams secrets in burning ink!".to_string(),
            ],
            death_message: "The tome's fire finally consumes it entirely.".to_string(),
            special_ability: Some(SpecialAbility::Enrage { damage_mult: 1.5, duration: 3.0 }),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: CRYSTAL CODEX (Floors 16-20)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("crystal_guardian".to_string(), EnemyTemplate {
            id: "crystal_guardian".to_string(),
            name: "Crystal Guardian".to_string(),
            description: "A being of pure crystallized thought.".to_string(),
            base_hp: 80,
            base_damage: 24,
            base_defense: 10,
            xp_reward: 70,
            gold_reward: 45,
            difficulty_tier: 6,
            typing_theme: "ice".to_string(),
            ascii_art: r#"
    ◇
   ◇◆◇
  ◇◆◆◆◇
   ◇◆◇
    ◇
"#.to_string(),
            attack_messages: vec![
                "Crystal shards rain down!".to_string(),
                "The guardian refracts your attacks!".to_string(),
            ],
            death_message: "The guardian shatters into a thousand fragments.".to_string(),
            special_ability: Some(SpecialAbility::Mirror),
        });

        enemies.insert("frozen_thought".to_string(), EnemyTemplate {
            id: "frozen_thought".to_string(),
            name: "Frozen Thought".to_string(),
            description: "An idea that was never completed, trapped in crystalline stasis.".to_string(),
            base_hp: 60,
            base_damage: 28,
            base_defense: 6,
            xp_reward: 65,
            gold_reward: 42,
            difficulty_tier: 6,
            typing_theme: "philosophy".to_string(),
            ascii_art: r#"
   ❄?❄
  ╱???╲
 (?????)
  ╲???╱
   ❄?❄
"#.to_string(),
            attack_messages: vec![
                "Incomplete thoughts bombard your mind!".to_string(),
                "The thought freezes your concentration!".to_string(),
            ],
            death_message: "The thought finally crystallizes into understanding.".to_string(),
            special_ability: Some(SpecialAbility::TimeWarp { reduction: 2.0 }),
        });

        enemies.insert("time_shard".to_string(), EnemyTemplate {
            id: "time_shard".to_string(),
            name: "Time Shard".to_string(),
            description: "A fragment of frozen time that attacks from multiple moments.".to_string(),
            base_hp: 75,
            base_damage: 30,
            base_defense: 5,
            xp_reward: 75,
            gold_reward: 50,
            difficulty_tier: 7,
            typing_theme: "temporal".to_string(),
            ascii_art: r#"
  ⧗
 ╱│╲
◁─┼─▷
 ╲│╱
  ⧖
"#.to_string(),
            attack_messages: vec![
                "The shard attacks from yesterday AND tomorrow!".to_string(),
                "Time stutters and skips!".to_string(),
            ],
            death_message: "The shard collapses into the present moment.".to_string(),
            special_ability: Some(SpecialAbility::WordScramble),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: VOID BETWEEN PAGES (Floors 21-25)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("void_crawler".to_string(), EnemyTemplate {
            id: "void_crawler".to_string(),
            name: "Void Crawler".to_string(),
            description: "A thing that exists in the spaces between letters.".to_string(),
            base_hp: 90,
            base_damage: 32,
            base_defense: 8,
            xp_reward: 85,
            gold_reward: 55,
            difficulty_tier: 7,
            typing_theme: "void".to_string(),
            ascii_art: r#"
   ████
  █    █
 █ ░  ░ █
  █    █
   ████
"#.to_string(),
            attack_messages: vec![
                "The crawler emerges from the whitespace!".to_string(),
                "Void tendrils reach between your words!".to_string(),
            ],
            death_message: "The crawler retreats into the margins.".to_string(),
            special_ability: Some(SpecialAbility::Blind { duration: 2.5 }),
        });

        enemies.insert("null_word".to_string(), EnemyTemplate {
            id: "null_word".to_string(),
            name: "Null Word".to_string(),
            description: "A word that means nothing and everything simultaneously.".to_string(),
            base_hp: 85,
            base_damage: 35,
            base_defense: 7,
            xp_reward: 90,
            gold_reward: 60,
            difficulty_tier: 8,
            typing_theme: "void".to_string(),
            ascii_art: r#"
 ╔═════╗
 ║     ║
 ║ N/A ║
 ║     ║
 ╚═════╝
"#.to_string(),
            attack_messages: vec![
                "The null word negates your meaning!".to_string(),
                "Your typed words become meaningless!".to_string(),
            ],
            death_message: "The null word gains definition in death.".to_string(),
            special_ability: Some(SpecialAbility::Corruption { extra_chars: 4 }),
        });

        enemies.insert("entropy_wisp".to_string(), EnemyTemplate {
            id: "entropy_wisp".to_string(),
            name: "Entropy Wisp".to_string(),
            description: "Pure chaos given form, it unravels order wherever it goes.".to_string(),
            base_hp: 70,
            base_damage: 40,
            base_defense: 4,
            xp_reward: 95,
            gold_reward: 65,
            difficulty_tier: 8,
            typing_theme: "chaos".to_string(),
            ascii_art: r#"
  ?¿?
 ?¿?¿?
  ?¿?
 ¿?¿?¿
  ¿?¿
"#.to_string(),
            attack_messages: vec![
                "Chaos corrupts your keyboard!".to_string(),
                "The wisp scrambles reality around you!".to_string(),
            ],
            death_message: "The entropy disperses into random noise.".to_string(),
            special_ability: Some(SpecialAbility::WordScramble),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ZONE-SPECIFIC ENEMIES: GENESIS ARCHIVE (Floors 26-30)
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("prime_letter".to_string(), EnemyTemplate {
            id: "prime_letter".to_string(),
            name: "Prime Letter".to_string(),
            description: "One of the original letters, from which all words descended.".to_string(),
            base_hp: 100,
            base_damage: 38,
            base_defense: 12,
            xp_reward: 110,
            gold_reward: 75,
            difficulty_tier: 9,
            typing_theme: "genesis".to_string(),
            ascii_art: r#"
   ╔═╗
   ║Ω║
  ═╩═╩═
   ███
"#.to_string(),
            attack_messages: vec![
                "The Prime Letter speaks in absolute truth!".to_string(),
                "Fundamental grammar assaults your mind!".to_string(),
            ],
            death_message: "The letter echoes eternally, never truly gone.".to_string(),
            special_ability: Some(SpecialAbility::Enrage { damage_mult: 1.8, duration: 4.0 }),
        });

        enemies.insert("genesis_construct".to_string(), EnemyTemplate {
            id: "genesis_construct".to_string(),
            name: "Genesis Construct".to_string(),
            description: "A being made of the first words ever written.".to_string(),
            base_hp: 120,
            base_damage: 42,
            base_defense: 15,
            xp_reward: 130,
            gold_reward: 85,
            difficulty_tier: 9,
            typing_theme: "genesis".to_string(),
            ascii_art: r#"
  ╔═══╗
  ║☼☼☼║
 ═╩═══╩═
 ║█████║
 ║█████║
 ╚═════╝
"#.to_string(),
            attack_messages: vec![
                "The construct speaks the first language!".to_string(),
                "Original syntax rewrites your understanding!".to_string(),
            ],
            death_message: "The construct returns to the first silence.".to_string(),
            special_ability: Some(SpecialAbility::Regenerate { percent: 8.0 }),
        });

        enemies.insert("alpha_word".to_string(), EnemyTemplate {
            id: "alpha_word".to_string(),
            name: "Alpha Word".to_string(),
            description: "The word that came before all others. It IS meaning itself.".to_string(),
            base_hp: 150,
            base_damage: 50,
            base_defense: 18,
            xp_reward: 150,
            gold_reward: 100,
            difficulty_tier: 10,
            typing_theme: "genesis".to_string(),
            ascii_art: r#"
   ★★★
  ★ Α ★
 ★  │  ★
  ★ │ ★
   ★│★
    V
"#.to_string(),
            attack_messages: vec![
                "THE ALPHA WORD SPEAKS AND REALITY LISTENS!".to_string(),
                "Your words are shadows of its truth!".to_string(),
            ],
            death_message: "The Alpha Word falls silent... but meaning persists.".to_string(),
            special_ability: Some(SpecialAbility::Corruption { extra_chars: 5 }),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ELITE VARIANTS
        // ═══════════════════════════════════════════════════════════════════

        enemies.insert("corrupted_librarian".to_string(), EnemyTemplate {
            id: "corrupted_librarian".to_string(),
            name: "Corrupted Librarian".to_string(),
            description: "Once a keeper of knowledge, now a purveyor of chaos.".to_string(),
            base_hp: 100,
            base_damage: 28,
            base_defense: 10,
            xp_reward: 80,
            gold_reward: 50,
            difficulty_tier: 5,
            typing_theme: "corruption".to_string(),
            ascii_art: r#"
   ╔═══╗
  ╱▓▓▓▓▓╲
 ║ ◈   ◈ ║
 ║  ▼▼▼  ║
  ╲▓▓▓▓▓╱
    ███
"#.to_string(),
            attack_messages: vec![
                "The librarian hurls forbidden tomes!".to_string(),
                "'Your late fees are OVERDUE!' it screams!".to_string(),
            ],
            death_message: "The librarian's corruption fades, revealing peaceful features.".to_string(),
            special_ability: Some(SpecialAbility::Summon { enemy_id: "paper_phantom".to_string(), count: 2 }),
        });

        enemies.insert("word_devourer".to_string(), EnemyTemplate {
            id: "word_devourer".to_string(),
            name: "Word Devourer".to_string(),
            description: "A nightmarish creature that feeds on language itself.".to_string(),
            base_hp: 130,
            base_damage: 35,
            base_defense: 8,
            xp_reward: 100,
            gold_reward: 70,
            difficulty_tier: 7,
            typing_theme: "void".to_string(),
            ascii_art: r#"
  ╔═══════╗
 ╔╝▓▓▓▓▓▓▓╚╗
 ║◈▓▓▓▓▓▓▓◈║
 ╚╗▓▓███▓▓╔╝
  ╚══███══╝
"#.to_string(),
            attack_messages: vec![
                "The devourer consumes your vowels!".to_string(),
                "Words disappear into its maw!".to_string(),
            ],
            death_message: "The devourer regurgitates a fountain of lost words.".to_string(),
            special_ability: Some(SpecialAbility::Corruption { extra_chars: 6 }),
        });

        // ═══════════════════════════════════════════════════════════════════
        // ADDITIONAL BOSSES
        // ═══════════════════════════════════════════════════════════════════

        bosses.insert("librarian_shade".to_string(), BossTemplate {
            id: "librarian_shade".to_string(),
            name: "The Librarian Shade".to_string(),
            title: "Keeper of Silence".to_string(),
            description: "The first Librarian, who chose to stay when the Library fell.".to_string(),
            lore: "When the Sundering came, the Head Librarian refused to flee. \
                   She believed her duty was eternal, and so it became. \
                   Now she enforces silence with ghostly fury.".to_string(),
            base_hp: 200,
            base_damage: 18,
            base_defense: 8,
            xp_reward: 300,
            gold_reward: 150,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Shushing".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: Some(SpecialAbility::Blind { duration: 1.0 }),
                },
                BossPhase {
                    hp_threshold: 0.5,
                    name: "Furious".to_string(),
                    damage_modifier: 1.5,
                    speed_modifier: 1.3,
                    special_ability: Some(SpecialAbility::Summon { enemy_id: "paper_phantom".to_string(), count: 3 }),
                },
            ],
            ascii_art: r#"
      ╔═══╗
     ╱ ▓▓▓ ╲
    ║ ◈   ◈ ║
    ║  ━━━  ║
     ╲ ▓▓▓ ╱
      ║███║
     ╱║███║╲
"#.to_string(),
            intro_dialogue: vec![
                "Shhhhh... No words allowed here.".to_string(),
                "You disturb the eternal silence.".to_string(),
                "The Library does not forgive noise.".to_string(),
            ],
            phase_transition_dialogue: vec![
                "QUIET! QUIET! QUIET!".to_string(),
                "I said... SILENCE!".to_string(),
            ],
            death_dialogue: vec![
                "The silence... it was all I had...".to_string(),
                "Perhaps... some words... deserve to be heard...".to_string(),
            ],
        });

        bosses.insert("phoenix_chronicler".to_string(), BossTemplate {
            id: "phoenix_chronicler".to_string(),
            name: "The Phoenix Chronicler".to_string(),
            title: "Flame of Forbidden Knowledge".to_string(),
            description: "A scribe who burned with their collection, reborn in eternal flame.".to_string(),
            lore: "They say knowledge is power. The Chronicler believed knowledge \
                   was fire—meant to spread, to consume, to illuminate. \
                   In death, they became the very flames they worshipped.".to_string(),
            base_hp: 350,
            base_damage: 30,
            base_defense: 12,
            xp_reward: 500,
            gold_reward: 250,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Smoldering".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: Some(SpecialAbility::TimeWarp { reduction: 1.5 }),
                },
                BossPhase {
                    hp_threshold: 0.6,
                    name: "Blazing".to_string(),
                    damage_modifier: 1.4,
                    speed_modifier: 1.2,
                    special_ability: Some(SpecialAbility::Enrage { damage_mult: 1.5, duration: 5.0 }),
                },
                BossPhase {
                    hp_threshold: 0.3,
                    name: "Infernal".to_string(),
                    damage_modifier: 1.8,
                    speed_modifier: 1.5,
                    special_ability: Some(SpecialAbility::Corruption { extra_chars: 4 }),
                },
            ],
            ascii_art: r#"
      🔥🔥🔥
    🔥╔═══╗🔥
   🔥║ ◆ ◆ ║🔥
   🔥║ ▼▼▼ ║🔥
    🔥║███║🔥
      🔥🔥🔥
"#.to_string(),
            intro_dialogue: vec![
                "BURN! Let the forbidden knowledge BURN!".to_string(),
                "I am the flame that illuminates truth!".to_string(),
                "Your words will fuel my eternal pyre!".to_string(),
            ],
            phase_transition_dialogue: vec![
                "MORE FIRE! MORE KNOWLEDGE TO BURN!".to_string(),
                "The flames speak truths you cannot bear!".to_string(),
            ],
            death_dialogue: vec![
                "From ashes... I will rise... again...".to_string(),
                "The fire... never truly dies...".to_string(),
            ],
        });

        bosses.insert("chronoscribe".to_string(), BossTemplate {
            id: "chronoscribe".to_string(),
            name: "The Chronoscribe".to_string(),
            title: "Warden of Frozen Moments".to_string(),
            description: "A scribe who recorded time itself, becoming trapped within it.".to_string(),
            lore: "They sought to write the complete history of everything. \
                   In doing so, they became unstuck from time, existing in all \
                   moments simultaneously, yet belonging to none.".to_string(),
            base_hp: 400,
            base_damage: 35,
            base_defense: 15,
            xp_reward: 600,
            gold_reward: 300,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Present".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: Some(SpecialAbility::TimeWarp { reduction: 2.0 }),
                },
                BossPhase {
                    hp_threshold: 0.7,
                    name: "Past Echo".to_string(),
                    damage_modifier: 1.3,
                    speed_modifier: 0.8, // Slower but harder hitting
                    special_ability: Some(SpecialAbility::Mirror),
                },
                BossPhase {
                    hp_threshold: 0.4,
                    name: "Future Shadow".to_string(),
                    damage_modifier: 1.5,
                    speed_modifier: 1.5,
                    special_ability: Some(SpecialAbility::WordScramble),
                },
                BossPhase {
                    hp_threshold: 0.15,
                    name: "Temporal Collapse".to_string(),
                    damage_modifier: 2.0,
                    speed_modifier: 2.0,
                    special_ability: Some(SpecialAbility::Blind { duration: 3.0 }),
                },
            ],
            ascii_art: r#"
      ⧗⧗⧗
    ╔═════╗
   ║ ◐   ◑ ║
   ║ ═════ ║
   ║ ⧖ ⧖ ⧖ ║
    ╚═════╝
      ⧖⧖⧖
"#.to_string(),
            intro_dialogue: vec![
                "I have seen this moment a thousand times.".to_string(),
                "You always come. You always try.".to_string(),
                "Let me show you how this ends... again.".to_string(),
            ],
            phase_transition_dialogue: vec![
                "Was that past or future? Does it matter?".to_string(),
                "Time is a flat circle of words.".to_string(),
                "I remember your defeat. I remember your victory. Both are true.".to_string(),
            ],
            death_dialogue: vec![
                "Finally... a moment I haven't seen...".to_string(),
                "Is this... the present? It's beautiful...".to_string(),
                "Time... flows... again...".to_string(),
            ],
        });

        bosses.insert("author_of_all".to_string(), BossTemplate {
            id: "author_of_all".to_string(),
            name: "The Author of All".to_string(),
            title: "First Word, Last Word".to_string(),
            description: "The being that wrote the first word, and will write the last.".to_string(),
            lore: "Before the Library, before language, before meaning itself, \
                   there was the Author. They wrote reality into existence. \
                   Now they wait to write 'The End.'".to_string(),
            base_hp: 800,
            base_damage: 60,
            base_defense: 25,
            xp_reward: 2000,
            gold_reward: 1000,
            phases: vec![
                BossPhase {
                    hp_threshold: 1.0,
                    name: "Prologue".to_string(),
                    damage_modifier: 1.0,
                    speed_modifier: 1.0,
                    special_ability: Some(SpecialAbility::Summon { enemy_id: "prime_letter".to_string(), count: 1 }),
                },
                BossPhase {
                    hp_threshold: 0.75,
                    name: "Rising Action".to_string(),
                    damage_modifier: 1.3,
                    speed_modifier: 1.2,
                    special_ability: Some(SpecialAbility::WordScramble),
                },
                BossPhase {
                    hp_threshold: 0.5,
                    name: "Climax".to_string(),
                    damage_modifier: 1.6,
                    speed_modifier: 1.4,
                    special_ability: Some(SpecialAbility::Corruption { extra_chars: 6 }),
                },
                BossPhase {
                    hp_threshold: 0.25,
                    name: "Falling Action".to_string(),
                    damage_modifier: 1.8,
                    speed_modifier: 1.6,
                    special_ability: Some(SpecialAbility::Blind { duration: 4.0 }),
                },
                BossPhase {
                    hp_threshold: 0.1,
                    name: "Epilogue".to_string(),
                    damage_modifier: 2.5,
                    speed_modifier: 2.0,
                    special_ability: Some(SpecialAbility::TimeWarp { reduction: 3.0 }),
                },
            ],
            ascii_art: r#"
        ★ ★ ★
      ╔═══════╗
     ╔╝ ◆   ◆ ╚╗
    ║   ═════   ║
    ║  ╔═════╗  ║
    ║  ║LOGOS║  ║
    ║  ╚═════╝  ║
     ╚╗       ╔╝
      ╚═══════╝
"#.to_string(),
            intro_dialogue: vec![
                "Ah. The protagonist arrives.".to_string(),
                "I wrote you, you know. Every keystroke, every victory.".to_string(),
                "Now let us see if the character can surpass the Author.".to_string(),
            ],
            phase_transition_dialogue: vec![
                "Interesting. You deviate from my outline.".to_string(),
                "A plot twist? I didn't write that...".to_string(),
                "Perhaps... perhaps YOU are the author now.".to_string(),
                "No! I will not be rewritten!".to_string(),
            ],
            death_dialogue: vec![
                "You... have written a new ending...".to_string(),
                "I created all words... yet I have none left...".to_string(),
                "The story... continues... without me...".to_string(),
                "*The pen falls silent. The page turns.*".to_string(),
            ],
        });

        Self { enemies, bosses }
    }
}
