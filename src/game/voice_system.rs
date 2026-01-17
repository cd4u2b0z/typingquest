//! Voice Consistency System - Faction-specific dialogue generation
//!
//! Each faction has a distinct "voice": vocabulary, sentence structure, 
//! metaphors, and topics they care about. NPCs should sound like they
//! belong to their faction.
//!
//! Inspired by: Disco Elysium's writing, Planescape Torment's factions

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use rand::prelude::*;
use super::narrative::Faction;

/// Voice profile for a faction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionVoice {
    /// The faction this voice belongs to
    pub faction: Faction,
    /// Core vocabulary - words this faction uses often
    pub vocabulary: VocabularySet,
    /// Sentence patterns this faction prefers
    pub sentence_patterns: Vec<SentencePattern>,
    /// Metaphors and imagery this faction uses
    pub metaphors: MetaphorSet,
    /// Topics this faction cares about
    pub preferred_topics: Vec<Topic>,
    /// Topics this faction avoids
    pub taboo_topics: Vec<Topic>,
    /// Emotional register (formal, casual, archaic, etc.)
    pub register: SpeechRegister,
    /// Common phrases and idioms
    pub idioms: Vec<String>,
    /// How they address the player
    pub player_addresses: Vec<String>,
    /// Greeting styles
    pub greetings: Vec<String>,
    /// Farewell styles
    pub farewells: Vec<String>,
}

impl FactionVoice {
    /// Generate a greeting appropriate to this faction
    pub fn generate_greeting<R: Rng>(&self, rng: &mut R) -> String {
        self.greetings.choose(rng).cloned().unwrap_or_else(|| "Hello.".to_string())
    }
    
    /// Generate a farewell appropriate to this faction
    pub fn generate_farewell<R: Rng>(&self, rng: &mut R) -> String {
        self.farewells.choose(rng).cloned().unwrap_or_else(|| "Goodbye.".to_string())
    }
    
    /// Generate how this faction would address the player
    pub fn address_player<R: Rng>(&self, rng: &mut R) -> String {
        self.player_addresses.choose(rng).cloned().unwrap_or_else(|| "you".to_string())
    }
    
    /// Get a random idiom
    pub fn random_idiom<R: Rng>(&self, rng: &mut R) -> Option<&str> {
        self.idioms.choose(rng).map(|s| s.as_str())
    }
}

/// Set of vocabulary words organized by type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularySet {
    /// Nouns this faction prefers
    pub nouns: Vec<String>,
    /// Verbs this faction prefers
    pub verbs: Vec<String>,
    /// Adjectives this faction prefers
    pub adjectives: Vec<String>,
    /// Adverbs this faction prefers
    pub adverbs: Vec<String>,
    /// Technical jargon
    pub jargon: Vec<String>,
    /// Words this faction never uses
    pub forbidden_words: HashSet<String>,
}

impl VocabularySet {
    pub fn new() -> Self {
        Self {
            nouns: Vec::new(),
            verbs: Vec::new(),
            adjectives: Vec::new(),
            adverbs: Vec::new(),
            jargon: Vec::new(),
            forbidden_words: HashSet::new(),
        }
    }
    
    pub fn random_noun<R: Rng>(&self, rng: &mut R) -> Option<&str> {
        self.nouns.choose(rng).map(|s| s.as_str())
    }
    
    pub fn random_verb<R: Rng>(&self, rng: &mut R) -> Option<&str> {
        self.verbs.choose(rng).map(|s| s.as_str())
    }
    
    pub fn random_adjective<R: Rng>(&self, rng: &mut R) -> Option<&str> {
        self.adjectives.choose(rng).map(|s| s.as_str())
    }
    
    pub fn random_jargon<R: Rng>(&self, rng: &mut R) -> Option<&str> {
        self.jargon.choose(rng).map(|s| s.as_str())
    }
}

impl Default for VocabularySet {
    fn default() -> Self {
        Self::new()
    }
}

/// Sentence patterns a faction tends to use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentencePattern {
    pub name: String,
    /// Template with placeholders: {noun}, {verb}, {adjective}, {jargon}, {player}
    pub template: String,
    /// Weight for random selection
    pub weight: f32,
    /// Context when this pattern is used
    pub context: Vec<DialogueContext>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueContext {
    Greeting,
    Farewell,
    QuestGive,
    QuestComplete,
    Trading,
    Gossip,
    Warning,
    Gratitude,
    Anger,
    Fear,
    Neutral,
}

/// Set of metaphors and imagery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphorSet {
    /// Source domains (what they compare things TO)
    pub source_domains: Vec<String>,
    /// Target domains (what they tend to talk ABOUT)
    pub target_domains: Vec<String>,
    /// Pre-made metaphorical expressions
    pub expressions: Vec<MetaphorExpression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphorExpression {
    /// The metaphor itself
    pub text: String,
    /// What it means
    pub meaning: String,
    /// When it's appropriate
    pub context: Vec<DialogueContext>,
}

/// Topics of conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topic {
    // Scholarly
    Knowledge,
    History,
    Magic,
    Research,
    // Political
    Power,
    Faction,
    War,
    Peace,
    // Personal
    Family,
    Love,
    Death,
    Memory,
    // Practical
    Trade,
    Travel,
    Survival,
    Combat,
    // Philosophical
    Truth,
    Meaning,
    Time,
    Fate,
    // World
    Corruption,
    Nature,
    Technology,
    Language,
}

/// Speech register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechRegister {
    /// Formal, educated speech
    Formal,
    /// Casual, everyday speech
    Casual,
    /// Old-fashioned, archaic speech
    Archaic,
    /// Technical, jargon-heavy speech
    Technical,
    /// Poetic, flowery speech
    Poetic,
    /// Blunt, direct speech
    Blunt,
    /// Mysterious, cryptic speech
    Cryptic,
    /// Zealous, passionate speech
    Zealous,
}

/// Build voice profiles for all factions
pub fn build_faction_voices() -> HashMap<Faction, FactionVoice> {
    let mut voices = HashMap::new();
    
    // Scribes of the Eternal Word
    voices.insert(Faction::Scribes, FactionVoice {
        faction: Faction::Scribes,
        vocabulary: VocabularySet {
            nouns: vec![
                "text", "scroll", "knowledge", "wisdom", "archive", "tome",
                "manuscript", "inscription", "chronicle", "doctrine", "verse",
                "meaning", "truth", "preservation", "legacy", "word",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "inscribe", "preserve", "illuminate", "transcribe", "decode",
                "interpret", "catalog", "restore", "sanctify", "protect",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "sacred", "eternal", "ancient", "true", "preserved",
                "illuminated", "canonical", "orthodox", "profound", "timeless",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "carefully", "reverently", "dutifully", "truly", "eternally",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "the Original Text", "Word-perfect", "the Great Library",
                "orthographic integrity", "semantic preservation",
            ].into_iter().map(String::from).collect(),
            forbidden_words: vec![
                "destroy", "burn", "forget", "abandon",
            ].into_iter().map(String::from).collect(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Knowledge blessing".to_string(),
                template: "May the {adjective} {noun} guide your path.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::Farewell, DialogueContext::Greeting],
            },
            SentencePattern {
                name: "Duty reminder".to_string(),
                template: "We must {verb} the {noun} against corruption.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::Warning, DialogueContext::QuestGive],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["writing", "scrolls", "ink", "light", "preservation"].into_iter().map(String::from).collect(),
            target_domains: vec!["truth", "history", "meaning", "salvation"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "The ink of history never truly dries.".to_string(),
                    meaning: "The past always affects the present.".to_string(),
                    context: vec![DialogueContext::Neutral, DialogueContext::Gossip],
                },
                MetaphorExpression {
                    text: "A single smudged word can corrupt an entire text.".to_string(),
                    meaning: "Small mistakes can have large consequences.".to_string(),
                    context: vec![DialogueContext::Warning],
                },
            ],
        },
        preferred_topics: vec![Topic::Knowledge, Topic::History, Topic::Truth, Topic::Corruption, Topic::Language],
        taboo_topics: vec![Topic::Trade, Topic::Power],
        register: SpeechRegister::Formal,
        idioms: vec![
            "By the First Word".to_string(),
            "Word-perfect and true".to_string(),
            "May your text remain uncorrupted".to_string(),
            "The Library remembers".to_string(),
            "Written in eternity".to_string(),
        ],
        player_addresses: vec![
            "seeker".to_string(),
            "student".to_string(),
            "reader".to_string(),
            "truth-seeker".to_string(),
        ],
        greetings: vec![
            "The Eternal Word welcomes you.".to_string(),
            "May your mind be open and your hand steady.".to_string(),
            "Ah, another seeker of knowledge.".to_string(),
            "The texts have much to teach you.".to_string(),
        ],
        farewells: vec![
            "May the words guide you.".to_string(),
            "Preserve what you have learned.".to_string(),
            "The Library awaits your return.".to_string(),
            "Go forth and transcribe truth.".to_string(),
        ],
    });
    
    // Mechanist Syndicate
    voices.insert(Faction::Mechanists, FactionVoice {
        faction: Faction::Mechanists,
        vocabulary: VocabularySet {
            nouns: vec![
                "system", "algorithm", "process", "efficiency", "output",
                "mechanism", "protocol", "interface", "data", "optimization",
                "structure", "framework", "module", "parameter", "function",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "optimize", "compute", "process", "iterate", "implement",
                "debug", "calibrate", "execute", "analyze", "synthesize",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "efficient", "optimal", "systematic", "precise", "functional",
                "logical", "streamlined", "automated", "calculated", "modular",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "efficiently", "precisely", "systematically", "logically", "optimally",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "keystroke efficiency", "typing throughput", "error correction protocol",
                "input-output ratio", "cognitive overhead",
            ].into_iter().map(String::from).collect(),
            forbidden_words: vec![
                "faith", "feeling", "intuition", "spiritual",
            ].into_iter().map(String::from).collect(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Efficiency statement".to_string(),
                template: "The {adjective} approach would be to {verb} the {noun}.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::QuestGive, DialogueContext::Trading],
            },
            SentencePattern {
                name: "System analysis".to_string(),
                template: "Our {jargon} indicates suboptimal {noun} parameters.".to_string(),
                weight: 0.8,
                context: vec![DialogueContext::Warning, DialogueContext::Neutral],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["machines", "circuits", "algorithms", "gears"].into_iter().map(String::from).collect(),
            target_domains: vec!["society", "people", "problems", "efficiency"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "Every gear must turn for the machine to function.".to_string(),
                    meaning: "Everyone must contribute.".to_string(),
                    context: vec![DialogueContext::Neutral],
                },
                MetaphorExpression {
                    text: "A system is only as strong as its weakest subroutine.".to_string(),
                    meaning: "Address vulnerabilities.".to_string(),
                    context: vec![DialogueContext::Warning],
                },
            ],
        },
        preferred_topics: vec![Topic::Technology, Topic::Power, Topic::Survival, Topic::Trade],
        taboo_topics: vec![Topic::Magic, Topic::Fate],
        register: SpeechRegister::Technical,
        idioms: vec![
            "By the Prime Algorithm".to_string(),
            "Optimize or obsolete".to_string(),
            "Processing...".to_string(),
            "That computes".to_string(),
            "Run the numbers".to_string(),
        ],
        player_addresses: vec![
            "user".to_string(),
            "operator".to_string(),
            "entity".to_string(),
            "process".to_string(),
        ],
        greetings: vec![
            "Initiating conversation protocol.".to_string(),
            "Your presence is logged.".to_string(),
            "Input accepted.".to_string(),
            "Greetings, operator.".to_string(),
        ],
        farewells: vec![
            "Process terminated.".to_string(),
            "May your algorithms run smoothly.".to_string(),
            "Efficiency be with you.".to_string(),
            "End of line.".to_string(),
        ],
    });
    
    // Naturalist Circle
    voices.insert(Faction::Naturalists, FactionVoice {
        faction: Faction::Naturalists,
        vocabulary: VocabularySet {
            nouns: vec![
                "flow", "rhythm", "nature", "balance", "harmony",
                "root", "growth", "cycle", "seed", "bloom",
                "river", "wind", "earth", "spirit", "essence",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "grow", "flow", "breathe", "nurture", "restore",
                "bloom", "root", "cycle", "balance", "renew",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "natural", "organic", "flowing", "balanced", "rooted",
                "wild", "ancient", "primal", "cyclical", "harmonious",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "naturally", "gently", "freely", "organically", "harmoniously",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "the Great Flow", "natural keystroke", "organic rhythm",
                "word-breath cycle", "typing meditation",
            ].into_iter().map(String::from).collect(),
            forbidden_words: vec![
                "artificial", "machine", "synthetic", "forced",
            ].into_iter().map(String::from).collect(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Nature wisdom".to_string(),
                template: "Like the {noun}, we must {verb} with the {noun}.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::Neutral, DialogueContext::Gossip],
            },
            SentencePattern {
                name: "Flow guidance".to_string(),
                template: "Let the {adjective} {noun} guide your fingers.".to_string(),
                weight: 0.9,
                context: vec![DialogueContext::QuestGive],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["rivers", "trees", "seasons", "wind", "roots"].into_iter().map(String::from).collect(),
            target_domains: vec!["typing", "life", "wisdom", "learning"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "The river does not fight the rocks, it flows around them.".to_string(),
                    meaning: "Adapt rather than force.".to_string(),
                    context: vec![DialogueContext::Neutral],
                },
                MetaphorExpression {
                    text: "Even the mightiest tree began as a seed.".to_string(),
                    meaning: "All skills start small.".to_string(),
                    context: vec![DialogueContext::Gratitude, DialogueContext::QuestComplete],
                },
            ],
        },
        preferred_topics: vec![Topic::Nature, Topic::Truth, Topic::Meaning, Topic::Time, Topic::Memory],
        taboo_topics: vec![Topic::Technology, Topic::War, Topic::Power],
        register: SpeechRegister::Poetic,
        idioms: vec![
            "May the Flow be with you".to_string(),
            "Rooted and true".to_string(),
            "As the wind wills".to_string(),
            "In the cycle of all things".to_string(),
            "Breathe with the words".to_string(),
        ],
        player_addresses: vec![
            "young sapling".to_string(),
            "wanderer".to_string(),
            "child of the wind".to_string(),
            "seeker of balance".to_string(),
        ],
        greetings: vec![
            "The wind brings you to us.".to_string(),
            "Your roots are welcome here.".to_string(),
            "Breathe deep, friend.".to_string(),
            "The Flow acknowledges you.".to_string(),
        ],
        farewells: vec![
            "May the wind guide your path.".to_string(),
            "Grow strong, little seed.".to_string(),
            "Flow freely, always.".to_string(),
            "The cycle continues.".to_string(),
        ],
    });
    
    // Shadow Writers Guild
    voices.insert(Faction::ShadowWriters, FactionVoice {
        faction: Faction::ShadowWriters,
        vocabulary: VocabularySet {
            nouns: vec![
                "shadow", "secret", "cipher", "code", "silence",
                "darkness", "whisper", "hidden", "mask", "truth",
                "night", "blade", "contract", "mark", "debt",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "hide", "encode", "obscure", "reveal", "silence",
                "decrypt", "vanish", "strike", "observe", "infiltrate",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "hidden", "secret", "dark", "silent", "encrypted",
                "invisible", "cloaked", "unseen", "veiled", "sharp",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "silently", "secretly", "carefully", "unseen", "swiftly",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "dead drop", "cipher key", "shadow mark", "silent strike",
                "the Deep Code", "encrypted contract",
            ].into_iter().map(String::from).collect(),
            forbidden_words: vec![
                "obvious", "loud", "public", "trust",
            ].into_iter().map(String::from).collect(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Cryptic warning".to_string(),
                template: "The {adjective} {noun} speaks to those who listen.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::Warning, DialogueContext::Gossip],
            },
            SentencePattern {
                name: "Business".to_string(),
                template: "Perhaps a {noun} needs to {verb}... for a price.".to_string(),
                weight: 0.8,
                context: vec![DialogueContext::Trading, DialogueContext::QuestGive],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["shadows", "night", "blades", "secrets", "ciphers"].into_iter().map(String::from).collect(),
            target_domains: vec!["truth", "survival", "information", "power"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "In the darkness, all truths are equal.".to_string(),
                    meaning: "Information has no morality.".to_string(),
                    context: vec![DialogueContext::Neutral],
                },
                MetaphorExpression {
                    text: "The sharpest blade is the one you never see.".to_string(),
                    meaning: "Subtlety is power.".to_string(),
                    context: vec![DialogueContext::Warning],
                },
            ],
        },
        preferred_topics: vec![Topic::Power, Topic::Trade, Topic::Survival, Topic::Truth],
        taboo_topics: vec![Topic::Family, Topic::Love],
        register: SpeechRegister::Cryptic,
        idioms: vec![
            "Shadows guide you".to_string(),
            "Trust is for fools".to_string(),
            "The night knows".to_string(),
            "Silence buys survival".to_string(),
            "A secret for a secret".to_string(),
        ],
        player_addresses: vec![
            "outsider".to_string(),
            "prospect".to_string(),
            "mark".to_string(),
            "contact".to_string(),
        ],
        greetings: vec![
            "I didn't hear you come in.".to_string(),
            "... You seek us. Why?".to_string(),
            "*emerges from shadow* Speak.".to_string(),
            "We know why you're here.".to_string(),
        ],
        farewells: vec![
            "We were never here.".to_string(),
            "The shadows will find you.".to_string(),
            "Forget this conversation.".to_string(),
            "*vanishes*".to_string(),
        ],
    });
    
    // Archivists of the Lost
    voices.insert(Faction::Archivists, FactionVoice {
        faction: Faction::Archivists,
        vocabulary: VocabularySet {
            nouns: vec![
                "memory", "loss", "fragment", "echo", "remnant",
                "past", "forgotten", "relic", "ghost", "trace",
                "void", "absence", "record", "salvage", "grief",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "remember", "recover", "salvage", "mourn", "restore",
                "piece", "recall", "preserve", "honor", "seek",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "lost", "forgotten", "faded", "ancient", "recovered",
                "fragmented", "partial", "echoing", "ghostly", "precious",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "solemnly", "carefully", "reverently", "sadly", "hopefully",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "Memory Fragment", "the Forgotten Stacks", "Void-touched",
                "echo-pattern", "salvage priority", "grief-work",
            ].into_iter().map(String::from).collect(),
            forbidden_words: vec![
                "forget", "discard", "irrelevant", "worthless",
            ].into_iter().map(String::from).collect(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Memory musing".to_string(),
                template: "The {adjective} {noun} still echoes in the void.".to_string(),
                weight: 1.0,
                context: vec![DialogueContext::Neutral, DialogueContext::Gossip],
            },
            SentencePattern {
                name: "Recovery mission".to_string(),
                template: "We must {verb} the {noun} before it fades entirely.".to_string(),
                weight: 0.9,
                context: vec![DialogueContext::QuestGive, DialogueContext::Warning],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["ghosts", "echoes", "voids", "memories", "fragments"].into_iter().map(String::from).collect(),
            target_domains: vec!["loss", "meaning", "identity", "history"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "Even the void remembers what once filled it.".to_string(),
                    meaning: "Nothing is ever truly forgotten.".to_string(),
                    context: vec![DialogueContext::Neutral],
                },
                MetaphorExpression {
                    text: "We are all fragments seeking our whole.".to_string(),
                    meaning: "We all have gaps in our understanding.".to_string(),
                    context: vec![DialogueContext::Gratitude],
                },
            ],
        },
        preferred_topics: vec![Topic::Memory, Topic::History, Topic::Death, Topic::Meaning, Topic::Time],
        taboo_topics: vec![Topic::Trade, Topic::Power],
        register: SpeechRegister::Archaic,
        idioms: vec![
            "The Forgotten remember".to_string(),
            "Lost but not gone".to_string(),
            "In the echo of all things".to_string(),
            "The void gives back".to_string(),
            "Memory persists".to_string(),
        ],
        player_addresses: vec![
            "seeker of the lost".to_string(),
            "memory-walker".to_string(),
            "fragment".to_string(),
            "echo".to_string(),
        ],
        greetings: vec![
            "Another soul touched by loss.".to_string(),
            "The Forgotten acknowledge you.".to_string(),
            "What memories bring you here?".to_string(),
            "The void has whispered your name.".to_string(),
        ],
        farewells: vec![
            "May your memories endure.".to_string(),
            "The Forgotten watch over you.".to_string(),
            "We shall remember you.".to_string(),
            "Return when the void calls.".to_string(),
        ],
    });
    
    voices
}

/// Generate dialogue based on faction voice
pub fn generate_faction_dialogue<R: Rng>(
    voice: &FactionVoice,
    context: DialogueContext,
    rng: &mut R,
) -> String {
    // Find appropriate sentence patterns
    let applicable_patterns: Vec<_> = voice.sentence_patterns
        .iter()
        .filter(|p| p.context.contains(&context) || p.context.is_empty())
        .collect();
    
    if let Some(pattern) = applicable_patterns.choose(rng) {
        let mut text = pattern.template.clone();
        
        // Replace placeholders
        if text.contains("{noun}") {
            if let Some(noun) = voice.vocabulary.random_noun(rng) {
                text = text.replacen("{noun}", noun, 1);
            }
            // Second noun if still present
            if text.contains("{noun}") {
                if let Some(noun) = voice.vocabulary.random_noun(rng) {
                    text = text.replace("{noun}", noun);
                }
            }
        }
        if text.contains("{verb}") {
            if let Some(verb) = voice.vocabulary.random_verb(rng) {
                text = text.replace("{verb}", verb);
            }
        }
        if text.contains("{adjective}") {
            if let Some(adj) = voice.vocabulary.random_adjective(rng) {
                text = text.replace("{adjective}", adj);
            }
        }
        if text.contains("{jargon}") {
            if let Some(jargon) = voice.vocabulary.random_jargon(rng) {
                text = text.replace("{jargon}", jargon);
            }
        }
        if text.contains("{player}") {
            text = text.replace("{player}", &voice.address_player(rng));
        }
        
        text
    } else {
        // Fallback
        match context {
            DialogueContext::Greeting => voice.generate_greeting(rng),
            DialogueContext::Farewell => voice.generate_farewell(rng),
            _ => "...".to_string(),
        }
    }
}

/// Dr. Baklava's unique voice (easter egg)
pub fn dr_baklava_voice() -> FactionVoice {
    FactionVoice {
        faction: Faction::Scribes, // Default, doesn't really matter
        vocabulary: VocabularySet {
            nouns: vec![
                "pastry", "honey", "phyllo", "baklava", "sweetness",
                "layers", "nuts", "syrup", "dessert", "perfection",
                "butter", "crunch", "golden", "kitchen", "recipe",
            ].into_iter().map(String::from).collect(),
            verbs: vec![
                "bake", "layer", "sweeten", "drizzle", "savor",
                "taste", "caramelize", "crisp", "fold", "glaze",
            ].into_iter().map(String::from).collect(),
            adjectives: vec![
                "flaky", "honeyed", "golden", "crispy", "sweet",
                "layered", "buttery", "delicious", "perfect", "nutty",
            ].into_iter().map(String::from).collect(),
            adverbs: vec![
                "deliciously", "sweetly", "perfectly", "lovingly", "patiently",
            ].into_iter().map(String::from).collect(),
            jargon: vec![
                "the Secret Recipe", "phyllo dimension", "infinite layers",
                "the Golden Ratio", "syrup velocity",
            ].into_iter().map(String::from).collect(),
            forbidden_words: HashSet::new(),
        },
        sentence_patterns: vec![
            SentencePattern {
                name: "Pastry wisdom".to_string(),
                template: "Life, like {noun}, requires {adjective} {noun}.".to_string(),
                weight: 1.0,
                context: vec![],
            },
        ],
        metaphors: MetaphorSet {
            source_domains: vec!["pastry", "baking", "honey", "layers"].into_iter().map(String::from).collect(),
            target_domains: vec!["life", "wisdom", "adventure", "typing"].into_iter().map(String::from).collect(),
            expressions: vec![
                MetaphorExpression {
                    text: "Every keystroke is a layer in the baklava of destiny.".to_string(),
                    meaning: "Small actions compound.".to_string(),
                    context: vec![],
                },
            ],
        },
        preferred_topics: vec![Topic::Truth, Topic::Meaning],
        taboo_topics: vec![],
        register: SpeechRegister::Cryptic,
        idioms: vec![
            "May your layers be many".to_string(),
            "The phyllo knows all".to_string(),
            "Drizzled with fate".to_string(),
            "In the oven of time".to_string(),
        ],
        player_addresses: vec![
            "hungry one".to_string(),
            "seeker of sweetness".to_string(),
            "unbaked soul".to_string(),
        ],
        greetings: vec![
            "*the scent of honey fills the air* Ah, a visitor.".to_string(),
            "The baklava anticipated your arrival.".to_string(),
            "Come, taste wisdom.".to_string(),
        ],
        farewells: vec![
            "May your crusts never burn.".to_string(),
            "*vanishes in a cloud of powdered sugar*".to_string(),
            "The recipe... continues.".to_string(),
        ],
    }
}
