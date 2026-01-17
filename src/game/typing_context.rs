//! Contextual Typing System - Typing means different things in different contexts
//!
//! Combat typing is different from dialogue typing is different from ritual typing.
//! This makes the core mechanic feel fresh throughout the game.
//!
//! Inspired by: Undertale's combat/mercy system, Disco Elysium's skill checks

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::narrative::Faction;

/// The context determines HOW typing works
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingContext {
    /// Standard combat - speed and accuracy both matter
    Combat(CombatContext),
    /// Dialogue - accuracy matters most, mistakes have social cost
    Dialogue(DialogueContext),
    /// Ritual - must type at a specific rhythm/speed
    Ritual(RitualContext),
    /// Decryption - solve a puzzle while typing
    Decryption(DecryptionContext),
    /// Persuasion - type counter-arguments
    Persuasion(PersuasionContext),
    /// Transcription - copy exactly what you see
    Transcription(TranscriptionContext),
    /// Race - pure speed competition
    Race(RaceContext),
    /// Stealth - type quietly (certain letters are "loud")
    Stealth(StealthContext),
}

impl TypingContext {
    pub fn name(&self) -> &str {
        match self {
            Self::Combat(_) => "Combat",
            Self::Dialogue(_) => "Dialogue",
            Self::Ritual(_) => "Ritual",
            Self::Decryption(_) => "Decryption",
            Self::Persuasion(_) => "Persuasion",
            Self::Transcription(_) => "Transcription",
            Self::Race(_) => "Race",
            Self::Stealth(_) => "Stealth",
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            Self::Combat(c) => format!("Combat against {}. {} at stake.", c.enemy_name, c.stakes.description()),
            Self::Dialogue(d) => format!("Conversation with {}. Topic: {}.", d.npc, d.topic.name()),
            Self::Ritual(r) => format!("Performing {}. Maintain {:.0} WPM.", r.ritual_name, r.target_wpm),
            Self::Decryption(d) => format!("Decrypting: {}. Cipher: {}.", d.hint, d.cipher_type.name()),
            Self::Persuasion(p) => format!("Counter-argue: \"{}\"", p.npc_argument),
            Self::Transcription(t) => format!("Transcribe {} exactly.", t.source_name),
            Self::Race(r) => format!("Racing {}. Prize: {} gold.", r.opponent_name, r.prize),
            Self::Stealth(s) => format!("Stealth mode. Loud letters: {}", s.loud_letters.iter().collect::<String>()),
        }
    }
}

/// Combat typing context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatContext {
    pub enemy_name: String,
    pub stakes: CombatStakes,
    /// Modifiers that affect this specific combat
    pub modifiers: Vec<CombatModifier>,
    /// Is this a boss fight?
    pub is_boss: bool,
    /// Environmental effects
    pub environment: Option<CombatEnvironment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatStakes {
    /// Normal fight - lose HP on failure
    Normal,
    /// Death is permanent
    Lethal,
    /// Protect someone else
    Protection { target: String },
    /// The world itself at stake
    WorldEnding,
    /// Just practice
    Training,
    /// Duel for honor
    Honor { wager: String },
}

impl CombatStakes {
    pub fn description(&self) -> &str {
        match self {
            Self::Normal => "Your life",
            Self::Lethal => "PERMADEATH ENABLED",
            Self::Protection { target } => "Their life",
            Self::WorldEnding => "Everything",
            Self::Training => "Nothing (practice)",
            Self::Honor { wager } => "Your honor",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatModifier {
    /// Time pressure
    TimePressure { reduction_percent: f32 },
    /// Accuracy requirements
    AccuracyRequired { minimum: f32 },
    /// Speed requirements
    SpeedRequired { minimum_wpm: f32 },
    /// Words are partially obscured
    Obscured { obscure_percent: f32 },
    /// Letters shift positions
    Scrambled { shift_frequency: f32 },
    /// Certain letters deal damage when typed
    CursedLetters { letters: Vec<char>, damage: i32 },
    /// Healing on perfect words
    VampiricWords { heal_percent: f32 },
    /// Combo multiplier enhanced
    ComboEnhanced { multiplier: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatEnvironment {
    /// Normal indoor
    Indoor,
    /// Raining - words "drip" down
    Rain { drip_speed: f32 },
    /// Dark - only current letter visible
    Darkness { visibility_radius: usize },
    /// Windy - words drift
    Wind { drift_direction: String, drift_speed: f32 },
    /// Corrupted - random letters change
    Corrupted { corruption_rate: f32 },
    /// Library - must be quiet (no mistakes)
    Library { mistake_penalty: i32 },
}

/// Dialogue typing context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContext {
    pub npc: String,
    pub npc_faction: Option<Faction>,
    pub topic: ConversationTopic,
    /// How mistakes affect the conversation
    pub typo_penalty: DialoguePenalty,
    /// Relationship at stake
    pub relationship_stakes: i32,
    /// Is this a timed response?
    pub timed: Option<f32>,
    /// Available dialogue options
    pub response_options: Vec<DialogueOption>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConversationTopic {
    Greeting,
    Gossip,
    QuestInfo,
    FactionBusiness,
    PersonalSecret,
    Negotiation,
    Interrogation,
    Romance,
    Threat,
    Philosophy,
}

impl ConversationTopic {
    pub fn name(&self) -> &str {
        match self {
            Self::Greeting => "Greeting",
            Self::Gossip => "Gossip",
            Self::QuestInfo => "Quest Information",
            Self::FactionBusiness => "Faction Business",
            Self::PersonalSecret => "Personal Secret",
            Self::Negotiation => "Negotiation",
            Self::Interrogation => "Interrogation",
            Self::Romance => "Romance",
            Self::Threat => "Threat",
            Self::Philosophy => "Philosophy",
        }
    }
    
    pub fn accuracy_requirement(&self) -> f32 {
        match self {
            Self::Greeting => 0.80,
            Self::Gossip => 0.75,
            Self::QuestInfo => 0.85,
            Self::FactionBusiness => 0.90,
            Self::PersonalSecret => 0.95,
            Self::Negotiation => 0.90,
            Self::Interrogation => 0.85,
            Self::Romance => 0.92,
            Self::Threat => 0.80,
            Self::Philosophy => 0.95,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialoguePenalty {
    /// Typos make you seem foolish
    SeemsStupid { reputation_loss: i32 },
    /// Typos interpreted as lies
    SeemsDeceptive { trust_loss: i32 },
    /// Typos anger the NPC
    AngersNPC { anger_per_typo: i32 },
    /// Typos reveal you're nervous
    SeemsNervous { suspicion_gain: i32 },
    /// No penalty (casual conversation)
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    /// What the player types to say this
    pub typed_response: String,
    /// What it actually says (may be different)
    pub spoken_text: String,
    /// Effects of choosing this
    pub effects: Vec<DialogueEffect>,
    /// Skill check required?
    pub skill_check: Option<DialogueSkillCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffect {
    ChangeRelationship(i32),
    ChangeFactionStanding(Faction, i32),
    RevealInformation(String),
    StartQuest(String),
    GiveItem(String),
    TriggerCombat,
    EndConversation,
    UnlockDialogueOption(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSkillCheck {
    pub skill: String,
    pub difficulty: u32,
    /// What happens on success
    pub success_text: String,
    /// What happens on failure  
    pub failure_text: String,
}

/// Ritual typing - must maintain specific rhythm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RitualContext {
    pub ritual_name: String,
    /// Target WPM - must be close to this
    pub target_wpm: f32,
    /// How close to target you must be
    pub tolerance: f32,
    /// Consequences of failing
    pub failure_consequence: RitualFailure,
    /// What the ritual accomplishes
    pub purpose: RitualPurpose,
    /// The text being typed (often a chant or incantation)
    pub incantation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RitualFailure {
    /// Ritual just fails, try again
    SimpleFailure,
    /// Backlash damage
    Backlash { damage: i32 },
    /// Corruption spreads
    CorruptionSpread { amount: i32 },
    /// Attract unwanted attention
    AttractEnemy { enemy_type: String },
    /// Opposite effect occurs
    Backfire { opposite_effect: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RitualPurpose {
    /// Maintain Haven's wards
    WardMaintenance,
    /// Open a sealed door
    SealBreaking { seal_name: String },
    /// Communicate with distant ally
    LongDistance { recipient: String },
    /// Purify corrupted text
    Purification { text_name: String },
    /// Summon an ally
    Summoning { ally_type: String },
    /// Divine the future
    Divination,
}

/// Decryption - solve puzzles while typing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptionContext {
    pub cipher_type: CipherType,
    pub hint: String,
    /// The encrypted text shown
    pub encrypted_text: String,
    /// What to actually type (the solution)
    pub solution: String,
    /// Time limit (if any)
    pub time_limit: Option<f32>,
    /// What you're unlocking
    pub reward: DecryptionReward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CipherType {
    /// Letters shifted by N (type the decoded version)
    Caesar { shift: i32 },
    /// Words scrambled (unscramble while typing)
    Anagram,
    /// Vowels removed (fill them in)
    Vowelless,
    /// Text is backwards (type forwards)
    Reversed,
    /// Substitution cipher (key provided)
    Substitution { key: HashMap<char, char> },
    /// Only every Nth letter matters
    Skip { n: usize },
    /// First letter of each word spells message
    Acrostic,
    /// Numbers represent letters (1=A, etc)
    Numeric,
}

impl CipherType {
    pub fn name(&self) -> &str {
        match self {
            Self::Caesar { .. } => "Caesar Shift",
            Self::Anagram => "Anagram",
            Self::Vowelless => "Missing Vowels",
            Self::Reversed => "Reversed Text",
            Self::Substitution { .. } => "Substitution",
            Self::Skip { .. } => "Skip Cipher",
            Self::Acrostic => "Acrostic",
            Self::Numeric => "Numeric Code",
        }
    }
    
    pub fn difficulty(&self) -> u32 {
        match self {
            Self::Reversed => 1,
            Self::Caesar { shift } if *shift <= 3 => 2,
            Self::Caesar { .. } => 3,
            Self::Vowelless => 2,
            Self::Anagram => 3,
            Self::Acrostic => 3,
            Self::Numeric => 2,
            Self::Skip { .. } => 4,
            Self::Substitution { .. } => 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecryptionReward {
    /// Reveals story information
    LoreUnlock { lore_id: String },
    /// Opens a physical path
    PathUnlock { location: String },
    /// Gives an item
    ItemReward { item_id: String },
    /// Reveals NPC secret
    SecretReveal { npc: String, secret: String },
    /// Just XP
    Experience(u32),
}

/// Persuasion - debate the NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersuasionContext {
    pub npc: String,
    /// What the NPC is arguing
    pub npc_argument: String,
    /// Valid counter-arguments (type one to succeed)
    pub valid_counters: Vec<CounterArgument>,
    /// How many rounds of debate
    pub rounds: u32,
    /// Current round
    pub current_round: u32,
    /// Points scored by each side
    pub player_points: u32,
    pub npc_points: u32,
    /// What happens if player wins
    pub victory_effect: PersuasionEffect,
    /// What happens if NPC wins
    pub defeat_effect: PersuasionEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterArgument {
    /// What to type
    pub text: String,
    /// How effective (1-3)
    pub effectiveness: u32,
    /// Flavor text when used
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersuasionEffect {
    /// NPC changes their stance
    ChangeStance { new_stance: String },
    /// NPC gives something
    Concession { what: String },
    /// Reputation change
    ReputationChange { faction: Option<Faction>, amount: i32 },
    /// NPC attacks
    CombatTriggered,
    /// NPC leaves
    NPCLeaves,
    /// Nothing changes
    Stalemate,
}

/// Transcription - copy exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionContext {
    pub source_name: String,
    pub source_type: TranscriptionSource,
    /// The text to copy
    pub text: String,
    /// Must be 100% accurate?
    pub perfect_required: bool,
    /// Time limit per line
    pub time_per_line: Option<f32>,
    /// What you get for completing it
    pub reward: TranscriptionReward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptionSource {
    /// Ancient scroll
    AncientText { age: String, language: String },
    /// Spoken words
    Dictation { speaker: String },
    /// Fading text
    FadingText { fade_rate: f32 },
    /// Text in a mirror
    MirrorText,
    /// Moving text
    MovingText { speed: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptionReward {
    /// Learn the text as a spell
    LearnSpell { spell_name: String },
    /// Record added to journal
    JournalEntry { entry: String },
    /// Quest item created
    CreateItem { item_id: String },
    /// Preservation XP
    Experience(u32),
}

/// Race - pure speed competition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceContext {
    pub opponent_name: String,
    pub opponent_wpm: f32,
    /// Text both racers type
    pub race_text: Vec<String>,
    /// Prize for winning
    pub prize: i32,
    /// What you lose if you lose
    pub wager: Option<i32>,
    /// Is this a faction championship?
    pub championship: Option<Faction>,
}

/// Stealth typing - some keys are "loud"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthContext {
    /// Letters that make noise
    pub loud_letters: Vec<char>,
    /// How many loud letters allowed
    pub noise_threshold: u32,
    /// Current noise level
    pub current_noise: u32,
    /// What happens if too noisy
    pub detection_consequence: DetectionConsequence,
    /// What you're sneaking past
    pub sneaking_past: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionConsequence {
    /// Guards attack
    Combat { enemy_count: u32 },
    /// Alarm raised
    Alarm { reinforcement_time: f32 },
    /// Caught and imprisoned
    Capture,
    /// Just have to retry
    Retry,
}

/// Result of a typing challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingResult {
    pub context_type: String,
    pub words_typed: u32,
    pub characters_typed: u32,
    pub errors: u32,
    pub accuracy: f32,
    pub wpm: f32,
    pub time_taken: f32,
    pub perfect_words: u32,
    pub longest_streak: u32,
    pub success: bool,
    pub rewards: Vec<TypingReward>,
    pub penalties: Vec<TypingPenaltyResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingReward {
    Experience(u32),
    Gold(i32),
    Item(String),
    Reputation(Faction, i32),
    Relationship(String, i32),
    SpellLearned(String),
    SkillPoint,
    LoreUnlocked(String),
    AreaUnlocked(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingPenaltyResult {
    Damage(i32),
    ReputationLoss(Faction, i32),
    RelationshipLoss(String, i32),
    GoldLost(i32),
    Detected,
    QuestFailed(String),
}

/// Evaluate typing performance based on context
pub fn evaluate_typing(
    context: &TypingContext,
    wpm: f32,
    accuracy: f32,
    perfect_words: u32,
    errors: u32,
    time_taken: f32,
) -> TypingResult {
    let mut result = TypingResult {
        context_type: context.name().to_string(),
        words_typed: 0,
        characters_typed: 0,
        errors,
        accuracy,
        wpm,
        time_taken,
        perfect_words,
        longest_streak: 0,
        success: false,
        rewards: Vec::new(),
        penalties: Vec::new(),
    };
    
    match context {
        TypingContext::Combat(c) => {
            // Combat success based on damage dealt
            result.success = true;
            let base_damage = (wpm * accuracy * 0.5) as i32;
            result.rewards.push(TypingReward::Experience((wpm * 2.0) as u32));
        }
        
        TypingContext::Dialogue(d) => {
            // Dialogue success based on accuracy meeting topic requirement
            let required_accuracy = d.topic.accuracy_requirement();
            result.success = accuracy >= required_accuracy;
            
            if !result.success {
                match &d.typo_penalty {
                    DialoguePenalty::SeemsStupid { reputation_loss } => {
                        result.penalties.push(TypingPenaltyResult::RelationshipLoss(
                            d.npc.clone(), 
                            *reputation_loss
                        ));
                    }
                    DialoguePenalty::AngersNPC { anger_per_typo } => {
                        result.penalties.push(TypingPenaltyResult::RelationshipLoss(
                            d.npc.clone(),
                            errors as i32 * anger_per_typo
                        ));
                    }
                    _ => {}
                }
            } else {
                result.rewards.push(TypingReward::Relationship(
                    d.npc.clone(),
                    d.relationship_stakes
                ));
            }
        }
        
        TypingContext::Ritual(r) => {
            // Ritual success based on maintaining target WPM
            let wpm_diff = (wpm - r.target_wpm).abs();
            result.success = wpm_diff <= r.tolerance;
            
            if !result.success {
                match &r.failure_consequence {
                    RitualFailure::Backlash { damage } => {
                        result.penalties.push(TypingPenaltyResult::Damage(*damage));
                    }
                    RitualFailure::AttractEnemy { .. } => {
                        // Combat triggered
                    }
                    _ => {}
                }
            } else {
                result.rewards.push(TypingReward::Experience(100));
            }
        }
        
        TypingContext::Decryption(d) => {
            // Decryption success based on accuracy (must be very high)
            result.success = accuracy >= 0.95;
            
            if result.success {
                match &d.reward {
                    DecryptionReward::Experience(xp) => {
                        result.rewards.push(TypingReward::Experience(*xp));
                    }
                    DecryptionReward::LoreUnlock { lore_id } => {
                        result.rewards.push(TypingReward::LoreUnlocked(lore_id.clone()));
                    }
                    DecryptionReward::ItemReward { item_id } => {
                        result.rewards.push(TypingReward::Item(item_id.clone()));
                    }
                    _ => {}
                }
            }
        }
        
        TypingContext::Stealth(s) => {
            // Count loud letters typed
            result.success = s.current_noise <= s.noise_threshold;
            
            if !result.success {
                result.penalties.push(TypingPenaltyResult::Detected);
            } else {
                result.rewards.push(TypingReward::Experience(75));
            }
        }
        
        _ => {
            // Default evaluation
            result.success = accuracy >= 0.85 && wpm >= 30.0;
        }
    }
    
    result
}

/// Generate a cipher challenge
pub fn generate_cipher(cipher_type: &CipherType, plaintext: &str) -> String {
    match cipher_type {
        CipherType::Caesar { shift } => {
            plaintext.chars().map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let shifted = ((c as u8 - base + *shift as u8) % 26) + base;
                    shifted as char
                } else {
                    c
                }
            }).collect()
        }
        
        CipherType::Reversed => {
            plaintext.chars().rev().collect()
        }
        
        CipherType::Vowelless => {
            plaintext.chars().map(|c| {
                if "aeiouAEIOU".contains(c) {
                    '_'
                } else {
                    c
                }
            }).collect()
        }
        
        CipherType::Numeric => {
            plaintext.chars().map(|c| {
                if c.is_ascii_alphabetic() {
                    let num = (c.to_ascii_lowercase() as u8 - b'a' + 1).to_string();
                    format!("{} ", num)
                } else if c == ' ' {
                    "/ ".to_string()
                } else {
                    c.to_string()
                }
            }).collect()
        }
        
        _ => plaintext.to_string(), // Fallback
    }
}

/// Pre-made typing challenges for various situations
pub fn get_dialogue_challenge(topic: ConversationTopic) -> Vec<String> {
    match topic {
        ConversationTopic::Greeting => vec![
            "Hello, friend".to_string(),
            "Well met, traveler".to_string(),
            "Greetings and salutations".to_string(),
        ],
        ConversationTopic::Negotiation => vec![
            "I believe we can reach an agreement".to_string(),
            "Consider this counter-offer".to_string(),
            "Your terms are acceptable with modifications".to_string(),
        ],
        ConversationTopic::Interrogation => vec![
            "I have nothing to hide".to_string(),
            "I was elsewhere that night".to_string(),
            "You have no evidence".to_string(),
        ],
        _ => vec!["...".to_string()],
    }
}

pub fn get_ritual_incantations() -> HashMap<String, Vec<String>> {
    let mut rituals = HashMap::new();
    
    rituals.insert("ward_maintenance".to_string(), vec![
        "By word and will, the barrier holds".to_string(),
        "Against the dark, our letters shield".to_string(),
        "Type true, type steady, type without fear".to_string(),
    ]);
    
    rituals.insert("purification".to_string(), vec![
        "Corruption fade, meaning return".to_string(),
        "The true word beneath revealed".to_string(),
        "What was lost, now restored".to_string(),
    ]);
    
    rituals.insert("summoning".to_string(), vec![
        "From distant realm I call thee forth".to_string(),
        "By name and nature, come to me".to_string(),
        "The pact is typed, the bond is sealed".to_string(),
    ]);
    
    rituals
}
