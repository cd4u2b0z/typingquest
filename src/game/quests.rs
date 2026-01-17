//! Quest System - Deep, meaningful objectives with real consequences

use serde::{Deserialize, Serialize};
use crate::game::narrative::{Faction, Effect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub giver: Option<String>,
    pub chapter: u32,
    pub quest_type: QuestType,
    pub stages: Vec<QuestStage>,
    pub current_stage: usize,
    pub status: QuestStatus,
    pub rewards: Vec<Effect>,
    pub consequences: QuestConsequences,
    pub is_main_quest: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestType {
    Main,
    Faction(Faction),
    Side,
    Radiant,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Completed,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestStage {
    pub id: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub on_complete: Vec<Effect>,
    pub journal_entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub description: String,
    pub objective_type: ObjectiveType,
    pub completed: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    /// Type a specific text with minimum accuracy
    TypeText { text: String, min_accuracy: f32 },
    /// Defeat a specific enemy
    DefeatEnemy { enemy_id: String },
    /// Talk to an NPC
    TalkTo { npc_id: String },
    /// Make a choice in dialogue
    MakeChoice { dialogue_id: String, choice_id: String },
    /// Reach a location
    ReachLocation { location_id: String },
    /// Collect an item
    CollectItem { item_id: String, count: i32 },
    /// Achieve typing metrics
    TypingChallenge { min_wpm: f32, min_accuracy: f32, word_count: i32 },
    /// Reach a certain level
    ReachLevel { level: u32 },
    /// Gain faction reputation
    FactionReputation { faction: Faction, amount: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestConsequences {
    pub success: Vec<ConsequenceDescription>,
    pub failure: Vec<ConsequenceDescription>,
    pub affects_factions: Vec<(Faction, i32, i32)>, // faction, success change, failure change
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceDescription {
    pub description: String,
    pub effects: Vec<Effect>,
}

/// The main story quests
pub fn get_main_quests() -> Vec<Quest> {
    vec![
        Quest {
            id: "main_001_awakening".to_string(),
            name: "Awakening".to_string(),
            description: "You have awoken with no memory, only the instinct to type. \
                         Find out who you are and why your fingers remember what your mind forgot.".to_string(),
            giver: None,
            chapter: 1,
            quest_type: QuestType::Main,
            stages: vec![
                QuestStage {
                    id: "stage_1".to_string(),
                    description: "Test your typing abilities".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Complete your first typing challenge".to_string(),
                            objective_type: ObjectiveType::TypingChallenge { 
                                min_wpm: 20.0, 
                                min_accuracy: 0.8, 
                                word_count: 10 
                            },
                            completed: false,
                            optional: false,
                        },
                    ],
                    on_complete: vec![Effect::GiveXP(50)],
                    journal_entry: "My fingers moved before my mind could catch up. \
                                   The words flowed from somewhere deep within me. \
                                   I am a Typer. But what does that mean?".to_string(),
                },
                QuestStage {
                    id: "stage_2".to_string(),
                    description: "Explore the Terminal".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Navigate through the starting area".to_string(),
                            objective_type: ObjectiveType::ReachLocation { 
                                location_id: "terminal_exit".to_string() 
                            },
                            completed: false,
                            optional: false,
                        },
                    ],
                    on_complete: vec![Effect::UnlockArea("hub_world".to_string())],
                    journal_entry: "The Terminal was just the beginning. Beyond its walls \
                                   lies a world transformed by the Corruption. I must find \
                                   answers.".to_string(),
                },
            ],
            current_stage: 0,
            status: QuestStatus::Active,
            rewards: vec![
                Effect::GiveXP(100),
                Effect::GiveGold(50),
            ],
            consequences: QuestConsequences {
                success: vec![
                    ConsequenceDescription {
                        description: "You have proven yourself capable. The journey truly begins.".to_string(),
                        effects: vec![Effect::SetFlag("awakening_complete".to_string(), true)],
                    }
                ],
                failure: vec![],
                affects_factions: vec![],
            },
            is_main_quest: true,
        },
        Quest {
            id: "main_002_first_contact".to_string(),
            name: "First Contact".to_string(),
            description: "The factions have noticed your awakening. Representatives from each \
                         wish to speak with you. Your choices will shape your destiny.".to_string(),
            giver: None,
            chapter: 1,
            quest_type: QuestType::Main,
            stages: vec![
                QuestStage {
                    id: "stage_1".to_string(),
                    description: "Meet the faction representatives".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Speak with the Scribe emissary".to_string(),
                            objective_type: ObjectiveType::TalkTo { 
                                npc_id: "scribe_emissary".to_string() 
                            },
                            completed: false,
                            optional: false,
                        },
                        Objective {
                            description: "Speak with the Mechanist emissary".to_string(),
                            objective_type: ObjectiveType::TalkTo { 
                                npc_id: "mechanist_emissary".to_string() 
                            },
                            completed: false,
                            optional: false,
                        },
                        Objective {
                            description: "Speak with the Naturalist emissary".to_string(),
                            objective_type: ObjectiveType::TalkTo { 
                                npc_id: "naturalist_emissary".to_string() 
                            },
                            completed: false,
                            optional: false,
                        },
                    ],
                    on_complete: vec![],
                    journal_entry: "Three factions, three philosophies, three paths forward. \
                                   The Scribes speak of tradition and meaning. The Mechanists \
                                   preach efficiency and progress. The Naturalists advocate \
                                   for harmony and flow. Each believes they hold the answer \
                                   to the Corruption.".to_string(),
                },
                QuestStage {
                    id: "stage_2".to_string(),
                    description: "Choose your initial allegiance (or remain independent)".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Make your choice known".to_string(),
                            objective_type: ObjectiveType::MakeChoice { 
                                dialogue_id: "faction_choice".to_string(),
                                choice_id: "any".to_string()
                            },
                            completed: false,
                            optional: false,
                        },
                    ],
                    on_complete: vec![],
                    journal_entry: "I have made my choice. Whether it is the right one, \
                                   only time will tell. But I cannot stand idle while \
                                   the Corruption spreads.".to_string(),
                },
            ],
            current_stage: 0,
            status: QuestStatus::NotStarted,
            rewards: vec![
                Effect::GiveXP(200),
            ],
            consequences: QuestConsequences {
                success: vec![
                    ConsequenceDescription {
                        description: "Your allegiance is noted. Allies and enemies are made.".to_string(),
                        effects: vec![Effect::SetFlag("faction_chosen".to_string(), true)],
                    }
                ],
                failure: vec![],
                affects_factions: vec![
                    // Joining one faction affects standing with others
                ],
            },
            is_main_quest: true,
        },
        Quest {
            id: "main_003_corruption_source".to_string(),
            name: "The Source of Corruption".to_string(),
            description: "Rumors speak of a place where the Corruption first emerged. \
                         Finding it might reveal how to stop it—or unleash something worse.".to_string(),
            giver: None,
            chapter: 2,
            quest_type: QuestType::Main,
            stages: vec![
                QuestStage {
                    id: "stage_1".to_string(),
                    description: "Gather information about the Corruption's origin".to_string(),
                    objectives: vec![
                        Objective {
                            description: "Search the Archives for records".to_string(),
                            objective_type: ObjectiveType::ReachLocation { 
                                location_id: "archives".to_string() 
                            },
                            completed: false,
                            optional: false,
                        },
                        Objective {
                            description: "Decode the encrypted files".to_string(),
                            objective_type: ObjectiveType::TypeText { 
                                text: "The first corruption emerged from a single mistyped character \
                                       in the central database. Patient zero was identified as \
                                       terminal 7-ALPHA. All attempts at containment failed.".to_string(),
                                min_accuracy: 0.95
                            },
                            completed: false,
                            optional: false,
                        },
                    ],
                    on_complete: vec![Effect::GiveXP(150)],
                    journal_entry: "The records are fragmented, corrupted themselves, but one \
                                   thing is clear: the Corruption is not random. It was started \
                                   by something—or someone.".to_string(),
                },
            ],
            current_stage: 0,
            status: QuestStatus::NotStarted,
            rewards: vec![
                Effect::GiveXP(500),
                Effect::GiveGold(200),
            ],
            consequences: QuestConsequences {
                success: vec![
                    ConsequenceDescription {
                        description: "The truth about the Corruption's origin changes everything.".to_string(),
                        effects: vec![Effect::SetFlag("corruption_origin_known".to_string(), true)],
                    }
                ],
                failure: vec![
                    ConsequenceDescription {
                        description: "The knowledge is lost. The Corruption grows stronger.".to_string(),
                        effects: vec![Effect::SetFlag("corruption_strengthened".to_string(), true)],
                    }
                ],
                affects_factions: vec![
                    (Faction::Archivists, 20, -30),
                    (Faction::ShadowWriters, 15, -10),
                ],
            },
            is_main_quest: true,
        },
    ]
}

/// Generate dynamic side quests
pub fn generate_side_quest(player_level: u32, faction_standings: &[(Faction, i32)]) -> Quest {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let quest_templates = vec![
        ("The Lost Manuscript", "An ancient text has been discovered, but it's partially corrupted. Type it perfectly to restore it."),
        ("Speed Trial", "The Mechanists challenge you to prove your typing speed. Show them what you can do."),
        ("Poetry Preservation", "A dying poet wishes their final work to be transcribed. Every word matters."),
        ("Encrypted Message", "The Shadow Writers need a coded message decoded through precise typing."),
        ("Teaching the Young", "A group of students needs guidance in the old ways of typing."),
    ];
    
    let (name, desc) = quest_templates[rng.gen_range(0..quest_templates.len())];
    
    Quest {
        id: format!("side_gen_{}", rng.gen::<u32>()),
        name: name.to_string(),
        description: desc.to_string(),
        giver: Some("Random NPC".to_string()),
        chapter: 1,
        quest_type: QuestType::Side,
        stages: vec![
            QuestStage {
                id: "stage_1".to_string(),
                description: "Complete the typing challenge".to_string(),
                objectives: vec![
                    Objective {
                        description: "Type the required text with sufficient accuracy".to_string(),
                        objective_type: ObjectiveType::TypingChallenge {
                            min_wpm: 30.0 + (player_level as f32 * 5.0),
                            min_accuracy: 0.85 + (player_level as f32 * 0.01),
                            word_count: 20 + (player_level as i32 * 5),
                        },
                        completed: false,
                        optional: false,
                    }
                ],
                on_complete: vec![],
                journal_entry: "Another challenge completed, another step on the path.".to_string(),
            }
        ],
        current_stage: 0,
        status: QuestStatus::NotStarted,
        rewards: vec![
            Effect::GiveXP(50 + (player_level as u64 * 25)),
            Effect::GiveGold(20 + (player_level as i32 * 10)),
        ],
        consequences: QuestConsequences {
            success: vec![],
            failure: vec![],
            affects_factions: vec![],
        },
        is_main_quest: false,
    }
}
