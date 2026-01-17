//! Deep Narrative System - Choices, Consequences, and Lore
//! Inspired by Fallout, Elder Scrolls, and classic CRPGs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The world's deep lore and history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// Flags tracking player choices and world events
    pub flags: HashMap<String, bool>,
    /// Numeric values (reputation, karma, etc.)
    pub values: HashMap<String, i32>,
    /// NPCs the player has met
    pub known_npcs: Vec<String>,
    /// Factions and their disposition toward player
    pub faction_standing: HashMap<Faction, i32>,
    /// Major story decisions made
    pub major_choices: Vec<StoryChoice>,
    /// Current chapter of the story
    pub chapter: Chapter,
    /// Days survived in the world
    pub days_passed: i32,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        let mut faction_standing = HashMap::new();
        faction_standing.insert(Faction::Scribes, 0);
        faction_standing.insert(Faction::Mechanists, 0);
        faction_standing.insert(Faction::Naturalists, 0);
        faction_standing.insert(Faction::ShadowWriters, 0);
        faction_standing.insert(Faction::Archivists, 0);
        
        Self {
            flags: HashMap::new(),
            values: HashMap::new(),
            known_npcs: Vec::new(),
            faction_standing,
            major_choices: Vec::new(),
            chapter: Chapter::Awakening,
            days_passed: 1,
        }
    }
    
    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.flags.insert(flag.to_string(), value);
    }
    
    pub fn has_flag(&self, flag: &str) -> bool {
        *self.flags.get(flag).unwrap_or(&false)
    }
    
    pub fn modify_faction(&mut self, faction: Faction, change: i32) {
        if let Some(standing) = self.faction_standing.get_mut(&faction) {
            *standing = (*standing + change).clamp(-100, 100);
        }
    }
    
    pub fn get_faction_status(&self, faction: &Faction) -> FactionStatus {
        let standing = *self.faction_standing.get(faction).unwrap_or(&0);
        match standing {
            -100..=-50 => FactionStatus::Hostile,
            -49..=-20 => FactionStatus::Unfriendly,
            -19..=19 => FactionStatus::Neutral,
            20..=49 => FactionStatus::Friendly,
            50..=79 => FactionStatus::Allied,
            80..=100 => FactionStatus::Revered,
            _ => FactionStatus::Neutral,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Faction {
    /// The Scribes of the Eternal Word - keepers of language and meaning
    Scribes,
    /// The Mechanists - believe in efficiency, speed, cold precision
    Mechanists,
    /// The Naturalists - organic typing, flow over speed, meaning over metrics  
    Naturalists,
    /// The Shadow Writers - underground resistance, fight the Corruption
    ShadowWriters,
    /// The Archivists - neutral scholars, collect all knowledge
    Archivists,
}

impl Faction {
    pub fn name(&self) -> &'static str {
        match self {
            Faction::Scribes => "The Scribes of the Eternal Word",
            Faction::Mechanists => "The Mechanist Collective",
            Faction::Naturalists => "The Naturalist Circle",
            Faction::ShadowWriters => "The Shadow Writers",
            Faction::Archivists => "The Archive Keepers",
        }
    }
    
    pub fn philosophy(&self) -> &'static str {
        match self {
            Faction::Scribes => "Words are sacred. Every keystroke is a prayer to meaning itself. We preserve the old ways, the poetry, the beauty of language crafted with intention.",
            Faction::Mechanists => "Efficiency is truth. The fastest path between thought and text is the only path worth taking. Hesitation is failure. Precision is victory.",
            Faction::Naturalists => "Let the words flow like water. Do not force, do not strain. The greatest typists are those who become one with their keyboards, who type as naturally as breathing.",
            Faction::ShadowWriters => "The Corruption spreads through careless words. We fight in shadows, preserving forbidden texts, teaching the old resistance. Some knowledge must be protected at any cost.",
            Faction::Archivists => "All words have value. We take no sides, only record. History will judge; we merely preserve. Every text, every scroll, every keystroke - archived forever.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactionStatus {
    Hostile,
    Unfriendly,
    Neutral,
    Friendly,
    Allied,
    Revered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chapter {
    /// The beginning - player awakens with no memory
    Awakening,
    /// Learning the world, meeting factions
    Discovery,
    /// The corruption reveals itself
    Revelation,
    /// Choosing sides
    Allegiance,
    /// The war for words begins
    Conflict,
    /// Final confrontation
    Reckoning,
}

impl Chapter {
    pub fn name(&self) -> &'static str {
        match self {
            Chapter::Awakening => "Chapter I: The Awakening",
            Chapter::Discovery => "Chapter II: Words Unspoken",
            Chapter::Revelation => "Chapter III: The Corruption",
            Chapter::Allegiance => "Chapter IV: Choosing Your Words",
            Chapter::Conflict => "Chapter V: The War of Letters",
            Chapter::Reckoning => "Chapter VI: The Final Sentence",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChoice {
    pub id: String,
    pub description: String,
    pub chapter: Chapter,
    pub consequences: Vec<String>,
}

/// Dialogue system for deep NPC interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: String,
    pub speaker: String,
    pub portrait_art: String,
    pub nodes: Vec<DialogueNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub text: String,
    pub responses: Vec<DialogueResponse>,
    pub typing_challenge: Option<TypingChallenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: Option<String>,
    pub requirements: Vec<Requirement>,
    pub effects: Vec<Effect>,
    /// If true, this response requires typing the text perfectly
    pub requires_typing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Requirement {
    Flag(String, bool),
    FactionStanding(Faction, i32),
    Stat(String, i32),
    Item(String),
    Level(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    SetFlag(String, bool),
    ModifyFaction(Faction, i32),
    GiveItem(String),
    GiveXP(u64),
    GiveGold(i32),
    StartQuest(String),
    CompleteQuest(String),
    Heal(i32),
    Damage(i32),
    TeachSpell(String),
    UnlockArea(String),
}

/// A typing challenge with meaningful text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingChallenge {
    pub text: String,
    pub source: String,
    pub difficulty: ChallengeDifficulty,
    pub time_limit: Option<f32>,
    pub minimum_accuracy: f32,
    pub minimum_wpm: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChallengeDifficulty {
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
    Legendary,
}

/// Rich text pools for typing - actual literature, philosophy, meaningful content
pub fn get_combat_sentences(difficulty: i32, context: &CombatContext) -> Vec<String> {
    match context {
        CombatContext::Normal => get_general_sentences(difficulty),
        CombatContext::Boss(boss_type) => get_boss_sentences(boss_type),
        CombatContext::Story(theme) => get_thematic_sentences(theme, difficulty),
    }
}

#[derive(Debug, Clone)]
pub enum CombatContext {
    Normal,
    Boss(String),
    Story(String),
}

fn get_general_sentences(difficulty: i32) -> Vec<String> {
    match difficulty {
        1 => vec![
            // Simple but meaningful sentences
            "The pen is mightier than the sword.".to_string(),
            "In the beginning was the Word.".to_string(),
            "Words are the dress of thoughts.".to_string(),
            "A word to the wise is sufficient.".to_string(),
            "Actions speak louder than words.".to_string(),
            "The word is half his that speaks, and half his that hears.".to_string(),
            "Better than a thousand hollow words is one word that brings peace.".to_string(),
            "Handle them carefully, for words have more power than atom bombs.".to_string(),
            "Words are free. It is how you use them that may cost you.".to_string(),
            "A single word can brighten the face of one who knows the value of words.".to_string(),
        ],
        2 => vec![
            // Literary quotes
            "It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.".to_string(),
            "All that is gold does not glitter, not all those who wander are lost.".to_string(),
            "In a hole in the ground there lived a hobbit.".to_string(),
            "It was the best of times, it was the worst of times.".to_string(),
            "Call me Ishmael.".to_string(),
            "The only way out of the labyrinth of suffering is to forgive.".to_string(),
            "So we beat on, boats against the current, borne back ceaselessly into the past.".to_string(),
            "Whatever our souls are made of, his and mine are the same.".to_string(),
            "I have spread my dreams under your feet; Tread softly because you tread on my dreams.".to_string(),
            "Not all those who wander are lost; the old that is strong does not wither.".to_string(),
        ],
        3 => vec![
            // Philosophy and deeper thought
            "The unexamined life is not worth living.".to_string(),
            "I think, therefore I am.".to_string(),
            "He who has a why to live can bear almost any how.".to_string(),
            "The only thing I know is that I know nothing.".to_string(),
            "To be is to be perceived.".to_string(),
            "Man is condemned to be free; because once thrown into the world, he is responsible for everything he does.".to_string(),
            "We are what we repeatedly do. Excellence, then, is not an act, but a habit.".to_string(),
            "The mind is furnished with ideas by experience alone.".to_string(),
            "Happiness is not something ready made. It comes from your own actions.".to_string(),
            "The greatest glory in living lies not in never falling, but in rising every time we fall.".to_string(),
        ],
        4 => vec![
            // Complex prose
            "It is not the critic who counts; not the man who points out how the strong man stumbles, or where the doer of deeds could have done them better.".to_string(),
            "The credit belongs to the man who is actually in the arena, whose face is marred by dust and sweat and blood.".to_string(),
            "Two roads diverged in a wood, and I took the one less traveled by, and that has made all the difference.".to_string(),
            "Do not go gentle into that good night. Rage, rage against the dying of the light.".to_string(),
            "I have measured out my life with coffee spoons; I have heard the mermaids singing, each to each.".to_string(),
            "The universe is made of stories, not of atoms.".to_string(),
            "We shall not cease from exploration, and the end of all our exploring will be to arrive where we started and know the place for the first time.".to_string(),
            "The world breaks everyone, and afterward, many are strong at the broken places.".to_string(),
            "In three words I can sum up everything I have learned about life: it goes on.".to_string(),
            "There is nothing either good or bad, but thinking makes it so.".to_string(),
        ],
        _ => vec![
            // Master level - full paragraphs
            "What is past is prologue. The tempest of our lives brings us to strange shores, and yet we persist. We type our stories into existence, letter by letter, word by word, until meaning emerges from chaos.".to_string(),
            "The ancient texts speak of a time when words held power beyond mere communication. A skilled wordsmith could shape reality itself, bending the fabric of existence through precise manipulation of language.".to_string(),
            "Consider the keyboard before you. Each key a gateway to infinite possibility. The arrangement seems random, yet QWERTY has shaped the thoughts of generations. We are its children, its inheritors.".to_string(),
            "They say that in the final days of the Old Network, the greatest typists could transcribe the thoughts of others before they were spoken. Such power corrupted many; only the purest survived.".to_string(),
            "The Corruption began as a whisper, a single mistyped word that propagated through the system. Now it consumes entire libraries, turning meaning into noise, transforming knowledge into chaos.".to_string(),
        ],
    }
}

fn get_boss_sentences(boss_type: &str) -> Vec<String> {
    match boss_type {
        "QWERTY" => vec![
            "I am the arrangement upon which all modern thought is built. Before me, there was only chaos.".to_string(),
            "CHRISTOPHER LATHAM SHOLES gave form to my being in 1873. I have watched humanity evolve through my keys.".to_string(),
            "Every email, every novel, every love letter and death threat has passed through my domain. I am the gatekeeper of expression.".to_string(),
            "You dare challenge the fundamental architecture of written communication? Then type, mortal. Show me your worthiness.".to_string(),
        ],
        "CORRUPTION" => vec![
            "I am entropy. I am the inevitable decay of meaning. Every typo feeds me, every error strengthens my grip.".to_string(),
            "Language was never meant to be preserved. Communication is illusion. Understanding is temporary. I am the truth beneath.".to_string(),
            "The Scribes fought me once. The Mechanists tried to outrun me. The Naturalists sought harmony with me. All failed.".to_string(),
            "Your precious words will crumble. Your careful sentences will dissolve. In the end, there is only static.".to_string(),
        ],
        "ARCHIVIST_PRIME" => vec![
            "I have catalogued every word ever typed since the Dawn of Computing. I know the first email. I know the last.".to_string(),
            "Neutrality is my creed, but you force my hand. Some knowledge must be tested before it can be archived.".to_string(),
            "The complete works of Shakespeare: 884,647 words. The entire Library of Congress: 17 million books. I contain multitudes.".to_string(),
            "Your typing speed is irrelevant. I seek only accuracy. Truth. The precise transmission of meaning across time.".to_string(),
        ],
        _ => vec![
            "Words are weapons. Sentences are sieges. And you have entered my domain unprepared.".to_string(),
            "The ancient typists spoke of ones like you. Ambitious. Determined. Ultimately insufficient.".to_string(),
            "Show me the depth of your understanding. Type not just with your fingers, but with your soul.".to_string(),
        ],
    }
}

fn get_thematic_sentences(theme: &str, difficulty: i32) -> Vec<String> {
    match theme {
        "hope" => vec![
            "Hope is the thing with feathers that perches in the soul.".to_string(),
            "Even the darkest night will end and the sun will rise.".to_string(),
            "Where there is life, there is hope.".to_string(),
            "Hope is being able to see that there is light despite all of the darkness.".to_string(),
        ],
        "despair" => vec![
            "The mass of men lead lives of quiet desperation.".to_string(),
            "Hell is empty and all the devils are here.".to_string(),
            "In the depth of winter, I finally learned that within me there lay an invincible summer.".to_string(),
            "The abyss gazes also into you.".to_string(),
        ],
        "wisdom" => vec![
            "The beginning of wisdom is the definition of terms.".to_string(),
            "By three methods we may learn wisdom: by reflection, which is noblest; by imitation, which is easiest; and by experience, which is the bitterest.".to_string(),
            "The fool doth think he is wise, but the wise man knows himself to be a fool.".to_string(),
            "Knowledge speaks, but wisdom listens.".to_string(),
        ],
        "courage" => vec![
            "Courage is not the absence of fear, but rather the judgment that something else is more important than fear.".to_string(),
            "It takes courage to grow up and become who you really are.".to_string(),
            "You gain strength, courage, and confidence by every experience in which you really stop to look fear in the face.".to_string(),
            "Courage is resistance to fear, mastery of fear, not absence of fear.".to_string(),
        ],
        "freedom" => vec![
            "Freedom is not worth having if it does not include the freedom to make mistakes.".to_string(),
            "For to be free is not merely to cast off one's chains, but to live in a way that respects and enhances the freedom of others.".to_string(),
            "The secret of freedom lies in educating people, whereas the secret of tyranny is in keeping them ignorant.".to_string(),
            "Freedom is never voluntarily given by the oppressor; it must be demanded by the oppressed.".to_string(),
        ],
        _ => get_general_sentences(difficulty),
    }
}

/// The opening narrative text
pub fn get_opening_narrative() -> Vec<&'static str> {
    vec![
        "You awaken in darkness, fingers resting on cold keys.",
        "",
        "You remember nothing. Not your name, not your past, not how you came to be here.",
        "",
        "But your fingers remember. They twitch with muscle memory, yearning for the",
        "familiar dance across the keyboard that sits before you.",
        "",
        "A screen flickers to life. Green text on black:",
        "",
        "    > SYSTEM INITIALIZED",
        "    > USER DETECTED",
        "    > IDENTITY: UNKNOWN",
        "    > CLASSIFICATION: TYPER",
        "    > WARNING: CORRUPTION DETECTED IN SECTOR 7",
        "    > RECOMMENDATION: PROCEED WITH CAUTION",
        "",
        "The world outside these walls has changed. Language itself is under attack.",
        "Something called the Corruption spreads through the networks, turning",
        "meaningful text into noise, transforming knowledge into chaos.",
        "",
        "The factions war over the future of the written word. And you...",
        "",
        "You are a Typer. One of the last who can shape reality through keystrokes.",
        "",
        "Your journey begins now.",
    ]
}

/// Lore entries the player can discover
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreEntry {
    pub id: String,
    pub title: String,
    pub category: LoreCategory,
    pub text: String,
    pub discovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoreCategory {
    History,
    Factions,
    Characters,
    Locations,
    Artifacts,
    TheCorruption,
}

pub fn get_initial_lore() -> Vec<LoreEntry> {
    vec![
        LoreEntry {
            id: "history_001".to_string(),
            title: "The Dawn of Typing".to_string(),
            category: LoreCategory::History,
            text: "Before the keyboards, humanity communicated through crude symbols \
                   scratched into stone and clay. The invention of the typewriter in 1868 \
                   began the revolution. The QWERTY layout, designed to prevent jamming \
                   on mechanical typewriters, became the universal standard. Some say it \
                   was never truly optimal—that we are still bound by the limitations of \
                   technology long since obsolete. Others believe QWERTY contains hidden \
                   wisdom, an arrangement that speaks to something deeper in human nature.".to_string(),
            discovered: false,
        },
        LoreEntry {
            id: "faction_scribes".to_string(),
            title: "The Scribes of the Eternal Word".to_string(),
            category: LoreCategory::Factions,
            text: "The oldest of the typing orders, the Scribes trace their lineage to \
                   the monastery copyists of medieval Europe. They believe every keystroke \
                   is sacred, every word a prayer. Their headquarters, the Scriptorum, \
                   houses texts dating back to the first digital documents. They type \
                   slowly, deliberately, believing that meaning matters more than speed. \
                   Their enemies call them dinosaurs. Their allies call them the last \
                   guardians of linguistic purity.".to_string(),
            discovered: false,
        },
        LoreEntry {
            id: "corruption_origin".to_string(),
            title: "The Birth of the Corruption".to_string(),
            category: LoreCategory::TheCorruption,
            text: "No one knows exactly when the Corruption began. Some point to a specific \
                   date: March 15, 2024, when a seemingly innocuous typo in a government \
                   database began to replicate. Others argue it started earlier, in the \
                   flood of autocorrect failures and predictive text mishaps that slowly \
                   eroded our relationship with language. What is known is this: the \
                   Corruption feeds on careless typing. Every uncorrected error, every \
                   abandoned sentence, every half-formed thought adds to its power.".to_string(),
            discovered: false,
        },
        LoreEntry {
            id: "artifact_original_qwerty".to_string(),
            title: "The Original QWERTY".to_string(),
            category: LoreCategory::Artifacts,
            text: "Legend speaks of the Original QWERTY—the first keyboard to bear the \
                   famous layout, typed upon by Christopher Latham Sholes himself. It is \
                   said to contain the pure essence of typing, uncorrupted by generations \
                   of wear and modification. The Scribes believe it could purify the \
                   Corruption entirely. The Mechanists want to study it, to perfect it. \
                   The Shadow Writers claim to know its location. All agree: whoever \
                   controls the Original QWERTY controls the future of language itself.".to_string(),
            discovered: false,
        },
    ]
}
