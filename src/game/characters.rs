//! Characters and NPCs with personality and depth

use serde::{Deserialize, Serialize};
use super::narrative::{Faction, DialogueNode, DialogueResponse, Effect, Requirement, TypingChallenge, ChallengeDifficulty};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub title: String,
    pub faction: Option<Faction>,
    pub personality: Personality,
    pub backstory: String,
    pub appearance: String,
    pub ascii_portrait: String,
    pub dialogue_tree: HashMap<String, DialogueNode>,
    pub relationship: i32,
    pub met: bool,
    pub alive: bool,
    pub location: String,
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub traits: Vec<PersonalityTrait>,
    pub values: Vec<Value>,
    pub speaking_style: SpeakingStyle,
    pub quirks: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PersonalityTrait {
    Kind, Cruel, Wise, Foolish, Brave, Cowardly, Honest, Deceptive,
    Patient, Impulsive, Curious, Apathetic, Idealistic, Cynical, Humble, Arrogant,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Value {
    Knowledge, Power, Freedom, Order, Tradition, Progress, Family, Honor, Wealth, Faith, Nature, Art,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakingStyle {
    pub formality: Formality,
    pub verbosity: Verbosity,
    pub common_phrases: Vec<String>,
    pub accent_notes: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Formality { VeryFormal, Formal, Neutral, Casual, Crude }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Verbosity { Terse, Concise, Normal, Verbose, Rambling }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
    pub content: String,
    pub revealed: bool,
    pub reveal_requirement: SecretRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretRequirement {
    RelationshipThreshold(i32),
    QuestComplete(String),
    ItemPossessed(String),
    SkillUnlocked(String),
    FactionStanding(Faction, i32),
    Discovered,
}

impl Character {
    pub fn archivist_vera() -> Self {
        let mut dialogue = HashMap::new();
        
        dialogue.insert("greeting".to_string(), DialogueNode {
            id: "greeting".to_string(),
            text: "[Archivist Vera] Ah, a new face in the Athenaeum. I am Vera, Keeper of Lost Words. \
                   Tell me, child of the modern age—what brings you to these ancient halls \
                   where the written word still holds power?".to_string(),
            typing_challenge: None,
            responses: vec![
                DialogueResponse {
                    text: "I seek knowledge about the corruption spreading across the land.".to_string(),
                    next_node: Some("corruption_info".to_string()),
                    requirements: vec![],
                    effects: vec![Effect::ModifyFaction(Faction::Archivists, 5)],
                    requires_typing: false,
                },
                DialogueResponse {
                    text: "I want to become stronger. Teach me the power of words.".to_string(),
                    next_node: Some("power_seeker".to_string()),
                    requirements: vec![],
                    effects: vec![Effect::ModifyFaction(Faction::Archivists, -5)],
                    requires_typing: false,
                },
            ],
        });
        
        dialogue.insert("corruption_info".to_string(), DialogueNode {
            id: "corruption_info".to_string(),
            text: "[Archivist Vera] The Unwriting spreads—a force that devours meaning itself. \
                   We believe it originates from a catastrophic event in the First Library.".to_string(),
            typing_challenge: Some(TypingChallenge {
                text: "The corruption feeds on careless words and thoughtless phrases, \
                       growing stronger with each meaning it devours.".to_string(),
                source: "Archivist Vera".to_string(),
                difficulty: ChallengeDifficulty::Journeyman,
                time_limit: Some(45.0),
                minimum_accuracy: 0.90,
                minimum_wpm: None,
            }),
            responses: vec![
                DialogueResponse {
                    text: "How can I help fight the Unwriting?".to_string(),
                    next_node: Some("join_fight".to_string()),
                    requirements: vec![],
                    effects: vec![Effect::StartQuest("corruption_investigation".to_string())],
                    requires_typing: false,
                },
            ],
        });
        
        Character {
            id: "archivist_vera".to_string(),
            name: "Vera".to_string(),
            title: "Keeper of Lost Words".to_string(),
            faction: Some(Faction::Archivists),
            personality: Personality {
                traits: vec![PersonalityTrait::Wise, PersonalityTrait::Patient, PersonalityTrait::Curious],
                values: vec![Value::Knowledge, Value::Tradition, Value::Art],
                speaking_style: SpeakingStyle {
                    formality: Formality::Formal,
                    verbosity: Verbosity::Verbose,
                    common_phrases: vec!["As it is written...".to_string()],
                    accent_notes: "Speaks with measured cadence".to_string(),
                },
                quirks: vec!["Constantly references obscure texts".to_string()],
            },
            backstory: "A scholar who abandoned academia to preserve forgotten words.".to_string(),
            appearance: "Elderly woman with silver hair and milky eyes that see clearly.".to_string(),
            ascii_portrait: "     .-\"\"\"\"-.    \n    /        \\   \n   |  O    O  |  \n    \\  `--'  /   \n     '-.__.-'    ".to_string(),
            dialogue_tree: dialogue,
            relationship: 0,
            met: false,
            alive: true,
            location: "athenaeum".to_string(),
            secrets: vec![Secret {
                id: "vera_true_age".to_string(),
                content: "Vera is over 200 years old, kept alive by the words she protects.".to_string(),
                revealed: false,
                reveal_requirement: SecretRequirement::RelationshipThreshold(80),
            }],
        }
    }
    
    pub fn commander_steele() -> Self {
        let mut dialogue = HashMap::new();
        
        dialogue.insert("greeting".to_string(), DialogueNode {
            id: "greeting".to_string(),
            text: "[Commander Steele] State your business. The Mechanist Legion has no time for idle chatter.".to_string(),
            typing_challenge: Some(TypingChallenge {
                text: "Efficiency is the highest virtue. Speed separates the worthy from the weak.".to_string(),
                source: "Commander Steele".to_string(),
                difficulty: ChallengeDifficulty::Expert,
                time_limit: Some(8.0),
                minimum_accuracy: 0.85,
                minimum_wpm: Some(80.0),
            }),
            responses: vec![
                DialogueResponse {
                    text: "I want to join the Mechanist Legion.".to_string(),
                    next_node: Some("join_mechanists".to_string()),
                    requirements: vec![],
                    effects: vec![Effect::ModifyFaction(Faction::Mechanists, 10)],
                    requires_typing: false,
                },
            ],
        });
        
        Character {
            id: "commander_steele".to_string(),
            name: "Marcus Steele".to_string(),
            title: "Commander of the Mechanist Legion".to_string(),
            faction: Some(Faction::Mechanists),
            personality: Personality {
                traits: vec![PersonalityTrait::Brave, PersonalityTrait::Impulsive, PersonalityTrait::Arrogant],
                values: vec![Value::Power, Value::Progress, Value::Order],
                speaking_style: SpeakingStyle {
                    formality: Formality::Neutral,
                    verbosity: Verbosity::Terse,
                    common_phrases: vec!["Speed or death.".to_string()],
                    accent_notes: "Speaks in bursts".to_string(),
                },
                quirks: vec!["Constantly tapping fingers".to_string()],
            },
            backstory: "A champion typist who survived the Unwriting through speed alone.".to_string(),
            appearance: "Muscular man with calloused fingers from decades of typing.".to_string(),
            ascii_portrait: "     .------.    \n    / .----. \\   \n   | | -  - | |  \n    \\  `--'  /   ".to_string(),
            dialogue_tree: dialogue,
            relationship: 0,
            met: false,
            alive: true,
            location: "mechanist_fortress".to_string(),
            secrets: vec![Secret {
                id: "steele_fear".to_string(),
                content: "Steele's obsession masks a deep fear of being too slow.".to_string(),
                revealed: false,
                reveal_requirement: SecretRequirement::RelationshipThreshold(60),
            }],
        }
    }
    
    pub fn shadow_whisper() -> Self {
        let mut dialogue = HashMap::new();
        
        dialogue.insert("greeting".to_string(), DialogueNode {
            id: "greeting".to_string(),
            text: "[???] Interesting. Most don't notice me. I am Whisper.".to_string(),
            typing_challenge: None,
            responses: vec![
                DialogueResponse {
                    text: "What do you want from me?".to_string(),
                    next_node: Some("proposition".to_string()),
                    requirements: vec![],
                    effects: vec![],
                    requires_typing: false,
                },
            ],
        });
        
        Character {
            id: "shadow_whisper".to_string(),
            name: "Whisper".to_string(),
            title: "Voice of the Shadow Writers".to_string(),
            faction: Some(Faction::ShadowWriters),
            personality: Personality {
                traits: vec![PersonalityTrait::Deceptive, PersonalityTrait::Curious, PersonalityTrait::Cynical],
                values: vec![Value::Freedom, Value::Knowledge, Value::Power],
                speaking_style: SpeakingStyle {
                    formality: Formality::Casual,
                    verbosity: Verbosity::Concise,
                    common_phrases: vec!["Interesting...".to_string()],
                    accent_notes: "Speaks softly".to_string(),
                },
                quirks: vec!["Never stands in direct light".to_string()],
            },
            backstory: "No one knows Whisper's true identity.".to_string(),
            appearance: "Indistinct, forgettable.".to_string(),
            ascii_portrait: "      .-\"\"-.     \n     /      \\    \n    |  ?  ?  |   \n     \\  ~~  /    ".to_string(),
            dialogue_tree: dialogue,
            relationship: 0,
            met: false,
            alive: true,
            location: "varies".to_string(),
            secrets: vec![Secret {
                id: "whisper_identity".to_string(),
                content: "Whisper is multiple individuals sharing one identity.".to_string(),
                revealed: false,
                reveal_requirement: SecretRequirement::FactionStanding(Faction::ShadowWriters, 80),
            }],
        }
    }
    
    pub fn elder_root() -> Self {
        let mut dialogue = HashMap::new();
        
        dialogue.insert("greeting".to_string(), DialogueNode {
            id: "greeting".to_string(),
            text: "[Elder Root] Be still. Listen. The forest has been expecting you.".to_string(),
            typing_challenge: None,
            responses: vec![
                DialogueResponse {
                    text: "I seek harmony between typing and nature.".to_string(),
                    next_node: Some("seeker".to_string()),
                    requirements: vec![],
                    effects: vec![Effect::ModifyFaction(Faction::Naturalists, 10)],
                    requires_typing: false,
                },
            ],
        });
        
        Character {
            id: "elder_root".to_string(),
            name: "Elder Root".to_string(),
            title: "Voice of the Green Word".to_string(),
            faction: Some(Faction::Naturalists),
            personality: Personality {
                traits: vec![PersonalityTrait::Wise, PersonalityTrait::Patient, PersonalityTrait::Kind],
                values: vec![Value::Nature, Value::Tradition, Value::Faith],
                speaking_style: SpeakingStyle {
                    formality: Formality::Formal,
                    verbosity: Verbosity::Verbose,
                    common_phrases: vec!["As the forest teaches...".to_string()],
                    accent_notes: "Speaks slowly like wind through leaves".to_string(),
                },
                quirks: vec!["Touches plants while speaking".to_string()],
            },
            backstory: "A being who heard the Green Word and was transformed.".to_string(),
            appearance: "Skin like bark, hair like moss.".to_string(),
            ascii_portrait: "      ,@@@,      \n     ,@@@@@,     \n     @@ O O@     \n      \\ = /      ".to_string(),
            dialogue_tree: dialogue,
            relationship: 0,
            met: false,
            alive: true,
            location: "sacred_grove".to_string(),
            secrets: vec![Secret {
                id: "root_origin".to_string(),
                content: "Elder Root is a nature spirit in human form.".to_string(),
                revealed: false,
                reveal_requirement: SecretRequirement::FactionStanding(Faction::Naturalists, 90),
            }],
        }
    }
    
    pub fn get_all_characters() -> Vec<Character> {
        vec![
            Self::archivist_vera(),
            Self::commander_steele(),
            Self::shadow_whisper(),
            Self::elder_root(),
        ]
    }
}
