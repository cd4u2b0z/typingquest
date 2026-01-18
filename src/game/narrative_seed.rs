//! Narrative Seed System - Run-wide coherence through seeded generation
//! 
//! This is the heart of procedural storytelling. Every run generates a unique
//! but internally consistent world state that ALL other systems reference.
//! 
//! 󰩛 Dr. Baklava's design philosophy: "Randomness without coherence is noise.
//!    Coherence without randomness is boredom. The seed is the soul."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use super::narrative::Faction;

/// The master seed that determines everything about this run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeSeed {
    /// The raw seed value (can be shared for identical runs)
    pub seed_value: u64,
    /// World-level constants that shape ALL generation
    pub world_state: WorldSeedState,
    /// Characters generated this run and their origins
    pub character_seeds: HashMap<String, CharacterSeed>,
    /// Events that have happened, affecting future generation
    pub timeline: Vec<TimelineEvent>,
    /// Prophecies/foreshadowing planted at run start
    pub prophecies: Vec<Prophecy>,
    /// Recurring motifs that appear throughout the run
    pub motifs: Vec<NarrativeMotif>,
}

impl NarrativeSeed {
    /// Generate a new narrative seed from a random value
    pub fn generate(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let world_state = WorldSeedState::generate(&mut rng);
        let prophecies = Prophecy::generate_set(&mut rng, &world_state);
        let motifs = NarrativeMotif::generate_set(&mut rng, &world_state);
        
        Self {
            seed_value: seed,
            world_state,
            character_seeds: HashMap::new(),
            timeline: vec![TimelineEvent::RunBegins],
            prophecies,
            motifs,
        }
    }
    
    /// Generate from current system time
    pub fn generate_random() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self::generate(seed)
    }
    
    /// Record an event in the timeline
    pub fn record_event(&mut self, event: TimelineEvent) {
        self.timeline.push(event);
    }
    
    /// Check if a prophecy has been fulfilled
    pub fn check_prophecies(&mut self, event: &TimelineEvent) -> Vec<&Prophecy> {
        self.prophecies.iter()
            .filter(|p| p.is_fulfilled_by(event))
            .collect()
    }
    
    /// Get the current narrative "temperature" - how much tension has built up
    pub fn tension_level(&self) -> f32 {
        let base_tension = self.timeline.len() as f32 * 0.05;
        let crisis_tension = if self.world_state.faction_crisis.is_some() { 0.2 } else { 0.0 };
        let betrayal_tension = self.timeline.iter()
            .filter(|e| matches!(e, TimelineEvent::BetrayedFaction { .. }))
            .count() as f32 * 0.15;
        
        (base_tension + crisis_tension + betrayal_tension).min(1.0)
    }
    
    /// Get a seeded character - generates consistent traits for same name
    pub fn get_or_create_character(&mut self, name: &str) -> CharacterSeed {
        if let Some(seed) = self.character_seeds.get(name) {
            seed.clone()
        } else {
            let mut rng = StdRng::seed_from_u64(
                self.seed_value.wrapping_add(name.bytes().map(|b| b as u64).sum::<u64>())
            );
            let seed = CharacterSeed::generate(&mut rng, &self.world_state);
            self.character_seeds.insert(name.to_string(), seed.clone());
            seed
        }
    }
}

/// World-level state that affects everything
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeedState {
    /// Which faction currently holds power
    pub dominant_faction: Faction,
    /// Which faction is struggling/on the decline
    pub declining_faction: Faction,
    /// The nature of the corruption this run
    pub corruption_type: CorruptionType,
    /// The inciting incident that started current events
    pub inciting_incident: IncitingIncident,
    /// Current faction crisis (if any)
    pub faction_crisis: Option<FactionCrisis>,
    /// The weather/atmosphere of the world
    pub world_mood: WorldMood,
    /// A specific MacGuffin driving the plot
    pub central_artifact: CentralArtifact,
}

impl WorldSeedState {
    pub fn generate<R: Rng>(rng: &mut R) -> Self {
        let factions = [
            Faction::MagesGuild,
            Faction::TempleOfDawn,
            Faction::RangersOfTheWild,
            Faction::ShadowGuild,
            Faction::MerchantConsortium,
        ];
        
        let dominant_idx = rng.gen_range(0..factions.len());
        let mut declining_idx = rng.gen_range(0..factions.len());
        while declining_idx == dominant_idx {
            declining_idx = rng.gen_range(0..factions.len());
        }
        
        Self {
            dominant_faction: factions[dominant_idx],
            declining_faction: factions[declining_idx],
            corruption_type: CorruptionType::random(rng),
            inciting_incident: IncitingIncident::random(rng),
            faction_crisis: if rng.gen_bool(0.6) {
                Some(FactionCrisis::random(rng, factions[declining_idx]))
            } else {
                None
            },
            world_mood: WorldMood::random(rng),
            central_artifact: CentralArtifact::random(rng),
        }
    }
}

/// How the corruption manifests this run - affects ALL corruption encounters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorruptionType {
    /// Words lose their meaning - philosophical horror
    SemanticDecay,
    /// Written text manifests physically - words become real
    LiteralManifest,
    /// People can no longer understand each other - Tower of Babel
    BabelCurse,
    /// Written lies become truth, truth becomes lies
    TruthInversion,
    /// Text consumes the writer - parasitic words
    GraphemeParasite,
    /// Language evolves too fast to follow - accelerated drift
    LinguisticAcceleration,
}

impl CorruptionType {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::SemanticDecay,
            1 => Self::LiteralManifest,
            2 => Self::BabelCurse,
            3 => Self::TruthInversion,
            4 => Self::GraphemeParasite,
            _ => Self::LinguisticAcceleration,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::SemanticDecay => "The Meaningless",
            Self::LiteralManifest => "The Manifest Word",
            Self::BabelCurse => "The Scattering",
            Self::TruthInversion => "The Great Lie",
            Self::GraphemeParasite => "The Hungry Letters",
            Self::LinguisticAcceleration => "The Drift",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::SemanticDecay => 
                "Words lose their meaning. 'Love' becomes noise. 'Help' summons nothing. \
                 The corruption doesn't destroy text—it hollows it out, leaving only shapes.",
            Self::LiteralManifest => 
                "Type 'fire' and flames appear. Write 'death' and... best not to. \
                 The barrier between language and reality has crumbled. Every word is a spell.",
            Self::BabelCurse => 
                "You speak, and others hear gibberish. They reply in tongues unknown. \
                 Only the written word—typed with precision—can bridge the gap now.",
            Self::TruthInversion => 
                "The corruption rewrites history. Written lies become fact. \
                 Only those who remember the old truths can fight back.",
            Self::GraphemeParasite => 
                "The letters themselves are hungry. They crawl from page to mind, \
                 consuming memories and replacing them with text. Type carefully.",
            Self::LinguisticAcceleration => 
                "Language evolves a century per day. Yesterday's words are archaic. \
                 Tomorrow's are incomprehensible. Only the fastest learners survive.",
        }
    }
    
    /// How this corruption type affects typing challenges
    pub fn typing_modifier(&self) -> TypingModifier {
        match self {
            Self::SemanticDecay => TypingModifier::WordsScramble { frequency: 0.1 },
            Self::LiteralManifest => TypingModifier::MistakesDealDamage { damage_per_error: 2 },
            Self::BabelCurse => TypingModifier::LanguageMixing { foreign_word_chance: 0.15 },
            Self::TruthInversion => TypingModifier::InvertedWords { inversion_chance: 0.2 },
            Self::GraphemeParasite => TypingModifier::LettersDisappear { decay_rate: 0.05 },
            Self::LinguisticAcceleration => TypingModifier::TimePressure { time_reduction: 0.3 },
        }
    }
}

/// How the corruption affects typing mechanically
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingModifier {
    /// Words occasionally scramble mid-typing
    WordsScramble { frequency: f32 },
    /// Mistakes deal damage to player
    MistakesDealDamage { damage_per_error: i32 },
    /// Foreign/archaic words mixed in
    LanguageMixing { foreign_word_chance: f32 },
    /// Some words must be typed backwards
    InvertedWords { inversion_chance: f32 },
    /// Letters fade if not typed quickly
    LettersDisappear { decay_rate: f32 },
    /// Time limits are shortened
    TimePressure { time_reduction: f32 },
}

/// The event that kicked off this run's story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncitingIncident {
    /// The First Library was breached
    LibraryBreached { by_whom: Faction },
    /// A great typist was assassinated
    MasterAssassinated { name: String, faction: Faction },
    /// An ancient text was discovered
    TextDiscovered { name: String, contents_hint: String },
    /// Two factions went to war
    FactionWar { aggressor: Faction, defender: Faction },
    /// A new corruption outbreak
    CorruptionSurge { origin_region: String },
    /// The player themselves caused something
    PlayerOrigin { mystery: String },
}

impl IncitingIncident {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let factions = [
            Faction::MagesGuild,
            Faction::TempleOfDawn,
            Faction::RangersOfTheWild,
            Faction::ShadowGuild,
            Faction::MerchantConsortium,
        ];
        
        match rng.gen_range(0..6) {
            0 => Self::LibraryBreached { 
                by_whom: factions[rng.gen_range(0..5)] 
            },
            1 => Self::MasterAssassinated {
                name: random_name(rng),
                faction: factions[rng.gen_range(0..5)],
            },
            2 => Self::TextDiscovered {
                name: random_artifact_name(rng),
                contents_hint: random_secret(rng),
            },
            3 => {
                let aggressor = factions[rng.gen_range(0..5)];
                let mut defender = factions[rng.gen_range(0..5)];
                while defender == aggressor {
                    defender = factions[rng.gen_range(0..5)];
                }
                Self::FactionWar { aggressor, defender }
            }
            4 => Self::CorruptionSurge {
                origin_region: random_region(rng),
            },
            _ => Self::PlayerOrigin {
                mystery: random_player_mystery(rng),
            },
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            Self::LibraryBreached { by_whom } => 
                format!("Three months ago, {} agents breached the First Library's inner sanctum. \
                        What they found—or released—changed everything.", by_whom.name()),
            Self::MasterAssassinated { name, faction } => 
                format!("Master {} of {} was found dead, their final words typed in blood: \
                        'THE WORDS REMEMBER.' No one knows what it means.", name, faction.name()),
            Self::TextDiscovered { name, contents_hint } => 
                format!("The {} was unearthed from the Corrupted Wastes. \
                        Its pages speak of {}. Every faction wants it.", name, contents_hint),
            Self::FactionWar { aggressor, defender } => 
                format!("{} declared war on {} without warning. \
                        The typing halls run red with ink.", aggressor.name(), defender.name()),
            Self::CorruptionSurge { origin_region } => 
                format!("A new corruption bloom erupted from {}. \
                        It spreads faster than any before. Time is running out.", origin_region),
            Self::PlayerOrigin { mystery } => 
                format!("You awoke with no memory, only muscle memory. Your fingers know words \
                        your mind has forgotten. And somewhere, {}.", mystery),
        }
    }
}

/// A faction-specific crisis that shapes NPC dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionCrisis {
    /// Leadership vacuum
    SuccessionCrisis { faction: Faction, candidates: Vec<String> },
    /// Internal betrayal
    InternalBetrayal { faction: Faction, traitor: String, sold_to: Faction },
    /// Lost something important
    CriticalLoss { faction: Faction, lost_item: String },
    /// Ideological schism
    Schism { faction: Faction, splinter_name: String, issue: String },
    /// Under siege
    UnderSiege { faction: Faction, besieged_location: String },
    /// Corruption infiltration
    CorruptionInfiltration { faction: Faction, severity: u32 },
}

impl FactionCrisis {
    pub fn random<R: Rng>(rng: &mut R, affected: Faction) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::SuccessionCrisis {
                faction: affected,
                candidates: vec![random_name(rng), random_name(rng), random_name(rng)],
            },
            1 => {
                let other_factions: Vec<_> = [
                    Faction::MagesGuild, Faction::TempleOfDawn, Faction::RangersOfTheWild,
                    Faction::ShadowGuild, Faction::MerchantConsortium,
                ].into_iter().filter(|f| *f != affected).collect();
                
                Self::InternalBetrayal {
                    faction: affected,
                    traitor: random_name(rng),
                    sold_to: other_factions[rng.gen_range(0..other_factions.len())],
                }
            }
            2 => Self::CriticalLoss {
                faction: affected,
                lost_item: random_artifact_name(rng),
            },
            3 => Self::Schism {
                faction: affected,
                splinter_name: random_splinter_name(rng),
                issue: random_schism_issue(rng),
            },
            4 => Self::UnderSiege {
                faction: affected,
                besieged_location: random_region(rng),
            },
            _ => Self::CorruptionInfiltration {
                faction: affected,
                severity: rng.gen_range(1..10),
            },
        }
    }
    
    pub fn faction(&self) -> Faction {
        match self {
            Self::SuccessionCrisis { faction, .. } => *faction,
            Self::InternalBetrayal { faction, .. } => *faction,
            Self::CriticalLoss { faction, .. } => *faction,
            Self::Schism { faction, .. } => *faction,
            Self::UnderSiege { faction, .. } => *faction,
            Self::CorruptionInfiltration { faction, .. } => *faction,
        }
    }
    
    pub fn description(&self) -> String {
        match self {
            Self::SuccessionCrisis { faction, candidates } => 
                format!("{} is leaderless. {} vie for control. The typing halls echo with political maneuvering.",
                    faction.name(), candidates.join(", ")),
            Self::InternalBetrayal { faction, traitor, sold_to } => 
                format!("{} was betrayed by {}—now a double agent for {}. Trust is shattered.",
                    faction.name(), traitor, sold_to.name()),
            Self::CriticalLoss { faction, lost_item } => 
                format!("{} has lost the {}. Without it, their power wanes daily.",
                    faction.name(), lost_item),
            Self::Schism { faction, splinter_name, issue } => 
                format!("{} has fractured. The {} split off over {}. Former allies now clash.",
                    faction.name(), splinter_name, issue),
            Self::UnderSiege { faction, besieged_location } => 
                format!("{}'s stronghold in {} is besieged. They grow desperate.",
                    faction.name(), besieged_location),
            Self::CorruptionInfiltration { faction, severity } => 
                format!("{} is corrupted from within. Severity: {}/10. Some members type... wrong.",
                    faction.name(), severity),
        }
    }
}

/// The overall atmosphere of the world
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorldMood {
    /// Hope is fading but not gone
    Twilight,
    /// Open conflict everywhere
    Wartime,
    /// Uneasy peace, tension underneath
    ColdWar,
    /// Things are rebuilding
    Recovery,
    /// Apocalyptic desperation
    EndTimes,
    /// Strange calm before the storm
    EeriePeace,
}

impl WorldMood {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::Twilight,
            1 => Self::Wartime,
            2 => Self::ColdWar,
            3 => Self::Recovery,
            4 => Self::EndTimes,
            _ => Self::EeriePeace,
        }
    }
    
    pub fn ambient_descriptions(&self) -> &[&'static str] {
        match self {
            Self::Twilight => &[
                "The light is always fading here, never quite dark, never quite bright.",
                "People speak in hushed tones, as if afraid to disturb something sleeping.",
                "Hope is a luxury few can afford, but some still type prayers.",
            ],
            Self::Wartime => &[
                "The clash of keystrokes echoes like distant thunder.",
                "Recruitment posters plaster every wall. Every faction needs soldiers.",
                "The wounded limp through streets, their typing fingers bandaged.",
            ],
            Self::ColdWar => &[
                "Smiles don't reach the eyes here. Everyone is watching everyone.",
                "Faction emblems are worn prominently. Neutrality is suspicious.",
                "Whispered conversations stop when strangers approach.",
            ],
            Self::Recovery => &[
                "Construction sounds everywhere. Rebuilding what was lost.",
                "Children laugh in streets that were silent a year ago.",
                "The scars remain, but new words are being written over them.",
            ],
            Self::EndTimes => &[
                "The corruption is visible on the horizon. A wall of un-meaning.",
                "People type frantically, preserving what they can before the end.",
                "Some have given up. Others type harder than ever.",
            ],
            Self::EeriePeace => &[
                "It's too quiet. The peace feels like a held breath.",
                "Everyone goes about their business, pretending nothing is wrong.",
                "The silence between keystrokes stretches unnaturally long.",
            ],
        }
    }
}

/// The MacGuffin driving this run's plot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralArtifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub current_holder: ArtifactHolder,
    pub power: String,
    pub danger: String,
}

impl CentralArtifact {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let artifact_type = ArtifactType::random(rng);
        Self {
            name: random_artifact_name(rng),
            artifact_type,
            current_holder: ArtifactHolder::random(rng),
            power: artifact_type.power_description(),
            danger: artifact_type.danger_description(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArtifactType {
    /// A keyboard with special powers
    AncientKeyboard,
    /// A text that rewrites reality
    ForbiddenCodex,
    /// The last copy of a lost language
    DeadLanguage,
    /// A machine that types on its own
    AutoScribe,
    /// Ink that never fades
    EternalInk,
    /// A cipher key to everything
    MasterCipher,
}

impl ArtifactType {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::AncientKeyboard,
            1 => Self::ForbiddenCodex,
            2 => Self::DeadLanguage,
            3 => Self::AutoScribe,
            4 => Self::EternalInk,
            _ => Self::MasterCipher,
        }
    }
    
    pub fn power_description(&self) -> String {
        match self {
            Self::AncientKeyboard => "Types words that existed before language was corrupted".to_string(),
            Self::ForbiddenCodex => "Contains the true names of things—type them to command reality".to_string(),
            Self::DeadLanguage => "The last speakers are gone, but the words still hold power".to_string(),
            Self::AutoScribe => "Writes prophecy without a typist—but whose will guides it?".to_string(),
            Self::EternalInk => "What is written cannot be unwritten, not even by corruption".to_string(),
            Self::MasterCipher => "Decodes any text, including the corruption's source code".to_string(),
        }
    }
    
    pub fn danger_description(&self) -> String {
        match self {
            Self::AncientKeyboard => "Each use drains the typist's memories".to_string(),
            Self::ForbiddenCodex => "Reading it invites the attention of things best left unnoticed".to_string(),
            Self::DeadLanguage => "Speaking dead words wakes dead things".to_string(),
            Self::AutoScribe => "Its prophecies tend to be self-fulfilling—and dark".to_string(),
            Self::EternalInk => "Mistakes are also eternal".to_string(),
            Self::MasterCipher => "The corruption knows it exists and hunts whoever holds it".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactHolder {
    Faction(Faction),
    Individual(String),
    Lost,
    Contested,
    Player,
}

impl ArtifactHolder {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..5) {
            0 => Self::Faction(Faction::MagesGuild),
            1 => Self::Faction(Faction::MerchantConsortium),
            2 => Self::Individual(random_name(rng)),
            3 => Self::Lost,
            _ => Self::Contested,
        }
    }
}

/// Seeded traits for a generated character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSeed {
    /// Their secret motivation
    pub hidden_agenda: HiddenAgenda,
    /// How they relate to the current crisis
    pub crisis_stance: CrisisStance,
    /// A personal secret
    pub dark_secret: String,
    /// Their typing style (affects how their dialogue feels)
    pub typing_personality: TypingPersonality,
    /// Will they betray the player?
    pub betrayal_likelihood: f32,
}

impl CharacterSeed {
    pub fn generate<R: Rng>(rng: &mut R, world: &WorldSeedState) -> Self {
        Self {
            hidden_agenda: HiddenAgenda::random(rng),
            crisis_stance: CrisisStance::random(rng, world),
            dark_secret: random_secret(rng),
            typing_personality: TypingPersonality::random(rng),
            betrayal_likelihood: rng.gen_range(0.0..0.4),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HiddenAgenda {
    /// Wants power for themselves
    PowerHungry,
    /// Secretly working for another faction
    DoubleAgent { true_loyalty: Faction },
    /// Trying to protect someone
    Protector { protecting: String },
    /// Seeking revenge
    Vengeance { against: String },
    /// Genuinely altruistic (rare)
    TrueBeliever,
    /// Just wants to survive
    Survivalist,
    /// Wants to destroy the current order
    Anarchist,
    /// Seeking forbidden knowledge
    KnowledgeSeeker,
}

impl HiddenAgenda {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..8) {
            0 => Self::PowerHungry,
            1 => Self::DoubleAgent { 
                true_loyalty: [Faction::MagesGuild, Faction::TempleOfDawn, Faction::RangersOfTheWild,
                               Faction::ShadowGuild, Faction::MerchantConsortium][rng.gen_range(0..5)]
            },
            2 => Self::Protector { protecting: random_name(rng) },
            3 => Self::Vengeance { against: random_name(rng) },
            4 => Self::TrueBeliever,
            5 => Self::Survivalist,
            6 => Self::Anarchist,
            _ => Self::KnowledgeSeeker,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrisisStance {
    /// Actively trying to solve it
    Resolver,
    /// Profiting from the chaos
    Opportunist,
    /// Hiding and waiting it out
    Hider,
    /// Caused it (knowingly or not)
    Responsible,
    /// Doesn't believe it's real
    Denier,
    /// Has a theory about it
    Theorist { theory: String },
}

impl CrisisStance {
    pub fn random<R: Rng>(rng: &mut R, _world: &WorldSeedState) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::Resolver,
            1 => Self::Opportunist,
            2 => Self::Hider,
            3 => Self::Responsible,
            4 => Self::Denier,
            _ => Self::Theorist { theory: random_theory(rng) },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypingPersonality {
    /// Fast and sloppy
    Hasty,
    /// Slow and precise
    Methodical,
    /// Erratic rhythm
    Unpredictable,
    /// Smooth and flowing
    Graceful,
    /// Aggressive, pounding keys
    Forceful,
    /// Hesitant, lots of pauses
    Nervous,
}

impl TypingPersonality {
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0..6) {
            0 => Self::Hasty,
            1 => Self::Methodical,
            2 => Self::Unpredictable,
            3 => Self::Graceful,
            4 => Self::Forceful,
            _ => Self::Nervous,
        }
    }
}

/// Events that have occurred this run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineEvent {
    RunBegins,
    FactionJoined { faction: Faction },
    FactionLeft { faction: Faction },
    BetrayedFaction { faction: Faction },
    QuestCompleted { quest_id: String },
    QuestFailed { quest_id: String },
    BossDefeated { boss_id: String },
    NPCMet { npc_id: String },
    NPCKilled { npc_id: String },
    ArtifactFound { artifact_id: String },
    ArtifactLost { artifact_id: String },
    LocationDiscovered { location_id: String },
    MajorChoice { choice_id: String, description: String },
    ProphecyFulfilled { prophecy_id: String },
}

/// Foreshadowing planted at run start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prophecy {
    pub id: String,
    pub text: String,
    pub fulfilled: bool,
    pub fulfillment_condition: ProphecyCondition,
}

impl Prophecy {
    pub fn generate_set<R: Rng>(rng: &mut R, world: &WorldSeedState) -> Vec<Self> {
        let mut prophecies = Vec::new();
        
        // Always one about the corruption
        prophecies.push(Self {
            id: "prophecy_corruption".to_string(),
            text: format!(
                "When {} reaches the heart of Haven, the final chapter begins.",
                world.corruption_type.name()
            ),
            fulfilled: false,
            fulfillment_condition: ProphecyCondition::CorruptionReachesLocation("haven".to_string()),
        });
        
        // One about a faction
        prophecies.push(Self {
            id: "prophecy_faction".to_string(),
            text: format!(
                "The {} shall face their darkest hour when three moons align.",
                world.dominant_faction.name()
            ),
            fulfilled: false,
            fulfillment_condition: ProphecyCondition::FactionReachesStatus(
                world.dominant_faction, 
                FactionCrisisLevel::Critical
            ),
        });
        
        // One about the player
        let player_prophecies = [
            "The one who types without memory shall remember at the worst moment.",
            "Your fingers know what your mind forgot. When you remember, you will wish you hadn't.",
            "The amnesiac's last word will be the first word of a new age.",
        ];
        prophecies.push(Self {
            id: "prophecy_player".to_string(),
            text: player_prophecies[rng.gen_range(0..player_prophecies.len())].to_string(),
            fulfilled: false,
            fulfillment_condition: ProphecyCondition::PlayerReachesLevel(10),
        });
        
        prophecies
    }
    
    pub fn is_fulfilled_by(&self, event: &TimelineEvent) -> bool {
        match (&self.fulfillment_condition, event) {
            (ProphecyCondition::FactionReachesStatus(f1, _), TimelineEvent::FactionJoined { faction }) 
                if f1 == faction => true,
            (ProphecyCondition::BossDefeated(boss), TimelineEvent::BossDefeated { boss_id })
                if boss == boss_id => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProphecyCondition {
    FactionReachesStatus(Faction, FactionCrisisLevel),
    CorruptionReachesLocation(String),
    PlayerReachesLevel(u32),
    BossDefeated(String),
    ArtifactObtained(String),
    ChoiceMade(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FactionCrisisLevel {
    Stable,
    Troubled,
    Critical,
    Collapsed,
}

/// Recurring narrative elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeMotif {
    pub name: String,
    pub symbol: String,
    pub meaning: String,
}

impl NarrativeMotif {
    pub fn generate_set<R: Rng>(rng: &mut R, _world: &WorldSeedState) -> Vec<Self> {
        let motif_pool = [
            ("Broken Keys", "󰌐", "lost potential, silenced voices"),
            ("Bleeding Ink", "󰴓", "sacrifice, permanent consequences"),
            ("Mirrors", "󰍹", "self-reflection, hidden truths"),
            ("Chains", "󰌾", "obligation, connection, imprisonment"),
            ("Seeds", "󰘬", "hope, hidden growth, patience"),
            ("Masks", "󰕧", "deception, protection, identity"),
            ("Bridges", "󰤇", "connection, risk, transition"),
            ("Roots", "󰁴", "history, foundation, hidden support"),
        ];
        
        let count = rng.gen_range(2..=4);
        let mut selected = Vec::new();
        let mut available: Vec<_> = motif_pool.iter().collect();
        
        for _ in 0..count {
            if available.is_empty() { break; }
            let idx = rng.gen_range(0..available.len());
            let (name, symbol, meaning) = available.remove(idx);
            selected.push(Self {
                name: name.to_string(),
                symbol: symbol.to_string(),
                meaning: meaning.to_string(),
            });
        }
        
        selected
    }
}

// Helper functions for random generation
fn random_name<R: Rng>(rng: &mut R) -> String {
    let prefixes = ["Cor", "Vel", "Sar", "Mal", "Fen", "Dra", "Ash", "Tyr", "Mor", "Kel"];
    let suffixes = ["ius", "ana", "oth", "en", "ix", "ara", "on", "iel", "us", "ia"];
    format!("{}{}", prefixes[rng.gen_range(0..prefixes.len())], suffixes[rng.gen_range(0..suffixes.len())])
}

fn random_artifact_name<R: Rng>(rng: &mut R) -> String {
    let adjectives = ["Void", "First", "Last", "Broken", "Eternal", "Silent", "Burning", "Frozen"];
    let nouns = ["Codex", "Keyboard", "Scroll", "Tablet", "Ink", "Quill", "Page", "Cipher"];
    format!("The {} {}", adjectives[rng.gen_range(0..adjectives.len())], nouns[rng.gen_range(0..nouns.len())])
}

fn random_region<R: Rng>(rng: &mut R) -> String {
    let regions = ["the Corrupted Wastes", "Haven", "the Athenaeum", "the Mechanist Fortress", 
                   "the Shadow Quarter", "the Sacred Grove", "the First Library"];
    regions[rng.gen_range(0..regions.len())].to_string()
}

fn random_secret<R: Rng>(rng: &mut R) -> String {
    let secrets = [
        "the true origin of the corruption",
        "how to reverse the unwriting",
        "the name of the First Typist",
        "the location of the Original Keyboard",
        "why words have power",
        "what lies beyond the corruption",
    ];
    secrets[rng.gen_range(0..secrets.len())].to_string()
}

fn random_player_mystery<R: Rng>(rng: &mut R) -> String {
    let mysteries = [
        "something hunts for you specifically",
        "you typed something terrible before you forgot",
        "the corruption knows your name",
        "you were someone important—or infamous",
        "your muscle memory includes words that shouldn't exist",
    ];
    mysteries[rng.gen_range(0..mysteries.len())].to_string()
}

fn random_splinter_name<R: Rng>(rng: &mut R) -> String {
    let names = ["True Path", "Reformed Order", "Purists", "New Dawn", "Old Guard"];
    names[rng.gen_range(0..names.len())].to_string()
}

fn random_schism_issue<R: Rng>(rng: &mut R) -> String {
    let issues = [
        "whether to ally with the Corruption or fight it",
        "the treatment of non-typists",
        "the use of forbidden typing techniques",
        "leadership succession",
        "the interpretation of ancient texts",
    ];
    issues[rng.gen_range(0..issues.len())].to_string()
}

fn random_theory<R: Rng>(rng: &mut R) -> String {
    let theories = [
        "The corruption is alive and thinks",
        "Someone is controlling the corruption deliberately",
        "The corruption is a natural phenomenon, like a storm",
        "The corruption is a punishment for linguistic sins",
        "The corruption is actually a cure misunderstood",
    ];
    theories[rng.gen_range(0..theories.len())].to_string()
}
