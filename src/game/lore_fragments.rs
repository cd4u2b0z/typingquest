//! Lore Fragment System - Environmental storytelling and discoverable history
//!
//! Players piece together the world's history through fragments found in the world.
//! Each fragment is carefully authored to reveal information indirectly, building
//! mystery and rewarding attentive players.
//!
//! Design principles:
//! - Gene Wolfe: Let readers work for understanding
//! - Dark Souls: Items tell stories through descriptions
//! - Morrowind: Books within books, contradicting sources
//! - Kentucky Route Zero: Poetic ambiguity, emotional truth over literal

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A fragment of discoverable lore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreFragment {
    /// Unique identifier
    pub id: String,
    /// Display title (what player sees in inventory/journal)
    pub title: String,
    /// The lore category for organization
    pub category: LoreCategory,
    /// The physical form this lore takes
    pub form: LoreForm,
    /// Where this fragment can be found
    pub location: String,
    /// How the player discovers it (search, quest reward, NPC gift, etc.)
    pub discovery_method: DiscoveryMethod,
    /// The actual text content
    pub content: LoreContent,
    /// What this fragment reveals about the world
    pub revelations: Vec<String>,
    /// Other fragments this connects to
    pub related_fragments: Vec<String>,
    /// Whether this fragment can be corrupted/changed
    pub mutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoreCategory {
    /// History of the world before the Sundering
    AncientHistory,
    /// The Sundering and its immediate aftermath
    TheSundering,
    /// Individual faction histories and philosophies
    FactionLore,
    /// Personal stories of NPCs and historical figures
    PersonalStories,
    /// Knowledge about the Blight and the Breach
    BlightStudies,
    /// Poetry, literature, and cultural artifacts
    Arts,
    /// Prophecies and predictions
    Prophecy,
    /// The player's forgotten past
    PlayerHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoreForm {
    /// A book or scroll
    Book { pages: u32, condition: TextCondition },
    /// A letter or message
    Letter { sender: String, recipient: String },
    /// Carved or engraved text
    Inscription { material: String },
    /// An item with a story
    Artifact { item_name: String },
    /// Spoken by an NPC
    Dialogue { speaker: String },
    /// A dream or vision
    Vision { trigger: String },
    /// Environmental text (signs, graffiti)
    Environmental { surface: String },
    /// A song or poem
    Song { verses: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextCondition {
    Pristine,
    Worn,
    Damaged,
    PartiallyCorrupted,
    HeavilyCorrupted,
    Fragmentary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Found through exploration
    Search { difficulty: u32 },
    /// Given by an NPC
    Gift { from: String, condition: Option<String> },
    /// Reward for completing something
    QuestReward { quest: String },
    /// Purchased from a vendor
    Purchase { cost: i32, vendor: String },
    /// Found on a defeated enemy
    EnemyDrop { enemy: String, chance: f32 },
    /// Appears in dreams
    Dream { chapter: u32 },
    /// Decoded from something else
    Decoded { source: String },
    /// Revealed by faction membership
    FactionSecret { faction: String, rank: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoreContent {
    /// The full text of the fragment
    pub full_text: String,
    /// A short excerpt for previews
    pub excerpt: String,
    /// Author attribution (may be "Unknown")
    pub attributed_to: String,
    /// Notes the player can read about what this reveals
    pub player_notes: Option<String>,
}

/// Build all lore fragments for the game
pub fn build_lore_fragments() -> HashMap<String, LoreFragment> {
    let mut fragments = HashMap::new();
    
    // ========================================================================
    // ANCIENT HISTORY FRAGMENTS
    // ========================================================================
    
    fragments.insert("founding_of_logos".to_string(), LoreFragment {
        id: "founding_of_logos".to_string(),
        title: "The Founding Charter of Logos Prime".to_string(),
        category: LoreCategory::AncientHistory,
        form: LoreForm::Book { pages: 12, condition: TextCondition::PartiallyCorrupted },
        location: "Athenaeum - Historical Section".to_string(),
        discovery_method: DiscoveryMethod::Search { difficulty: 2 },
        content: LoreContent {
            full_text: "In the three-hundred-and-seventh year of the Age of Writing, the \
                Conclave of Mages gathered at the meeting of three rivers to establish \
                a repository of all knowledge. Here, where water—the most mutable of \
                elements—becomes briefly still, we shall build permanence.

                Let it be known that Logos Prime is founded on three principles:
                First, that all words have weight.
                Second, that preservation is sacred.
                Third, that [text corrupted]

                We, the founding members, commit ourselves to maintaining the Eternal \
                Codex, the Name Registry, and the Tomorrow Texts. May our work outlast \
                our bodies. May our words outlast our work.

                [Several pages are missing or illegible]

                ...warned against pursuing the Final Grammar. 'Some words,' the First \
                Scribe cautioned, 'are better left unspoken. Some concepts better left \
                unnamed. We preserve knowledge, not all knowledge. The difference is \
                wisdom.'

                This warning was voted down. The pursuit of completeness was deemed \
                more important than caution.

                [Remainder corrupted]".to_string(),
            excerpt: "Let it be known that Logos Prime is founded on three principles...".to_string(),
            attributed_to: "The Founding Conclave of Logos Prime".to_string(),
            player_notes: Some("The third principle is corrupted. What did they hide?".to_string()),
        },
        revelations: vec![
            "Logos Prime was the center of the old world.".to_string(),
            "The founders were warned about pursuing 'the Final Grammar.'".to_string(),
            "They ignored the warning.".to_string(),
        ],
        related_fragments: vec!["first_scribe_warning".to_string(), "eternal_codex_index".to_string()],
        mutable: true,
    });
    
    fragments.insert("first_scribe_warning".to_string(), LoreFragment {
        id: "first_scribe_warning".to_string(),
        title: "On Boundaries (attributed to the First Scribe)".to_string(),
        category: LoreCategory::AncientHistory,
        form: LoreForm::Inscription { material: "Foundation stone of the Athenaeum".to_string() },
        location: "Athenaeum - Hidden basement".to_string(),
        discovery_method: DiscoveryMethod::Search { difficulty: 4 },
        content: LoreContent {
            full_text: "I discovered how to make words permanent.
                This is not a gift. It is a responsibility.

                Before my discovery, words were free.
                They could be forgotten, mistaken, transformed.
                Now they endure.
                Now they bind.

                I have seen the future in the pattern of my letters.
                Someone will try to use my gift to bind even death.
                They will fail.
                The failure will be worse than death.

                To whoever finds this:
                Some doors should remain locked.
                Some words should remain unwritten.
                The Final Grammar is not a goal. It is a warning.

                I will not write what 'death' looks like in text.
                I will not give anyone the tools to unbind it.
                Let this be my final lesson:
                The greatest power of the written word
                is knowing when not to write.".to_string(),
            excerpt: "Some doors should remain locked. Some words should remain unwritten.".to_string(),
            attributed_to: "The First Scribe".to_string(),
            player_notes: Some("The First Scribe predicted the First Silence.".to_string()),
        },
        revelations: vec![
            "The First Scribe knew what would happen.".to_string(),
            "They deliberately withheld knowledge of how to unbind death.".to_string(),
            "Someone found it anyway.".to_string(),
        ],
        related_fragments: vec!["founding_of_logos".to_string(), "first_speaker_journal_1".to_string()],
        mutable: false,
    });
    
    // ========================================================================
    // FIRST SILENCE FRAGMENTS
    // ========================================================================
    
    fragments.insert("first_speaker_journal_1".to_string(), LoreFragment {
        id: "first_speaker_journal_1".to_string(),
        title: "Personal Journal (First Volume)".to_string(),
        category: LoreCategory::TheSundering,
        form: LoreForm::Book { pages: 200, condition: TextCondition::Worn },
        location: "Shadow Quarter - Hidden cache".to_string(),
        discovery_method: DiscoveryMethod::FactionSecret { 
            faction: "ShadowWriters".to_string(), 
            rank: "Trusted".to_string() 
        },
        content: LoreContent {
            full_text: "Year 2987, Spring

                Today I achieved what the masters said was impossible. I typed a lie \
                that became true. I wrote 'the apple is blue' and watched it happen.

                This is terrifying.
                This is beautiful.
                I don't know which feeling is more appropriate.

                Master Verity says I have a gift. She says I must use it responsibly. \
                But how can I be responsible with the power to reshape reality? \
                Every word I type might change the world.

                Year 2987, Summer

                I've met someone. Their name is [deliberately obscured]. They're a \
                researcher studying the boundaries of language. They believe there \
                are words we haven't discovered yet—concepts waiting to be named.

                We talk for hours about the nature of meaning. About whether reality \
                shapes language or language shapes reality. I think it's both. I think \
                we're the needle stitching them together.

                Year 2988, Winter

                We were married today. I typed our vows into the Eternal Codex itself. \
                'As long as these words endure,' I wrote, 'so shall our bond.'

                I meant it as poetry.
                Now I understand it as prophecy.

                Year 2989, Autumn

                [The handwriting becomes erratic here]

                They're sick. Something in the Codex—something we touched in our research— \
                it's undoing them. Their name is harder to remember each day. I have to \
                keep fighting it. I have to keep making it true.

                Why can't I fix this? I can make apples blue. Why can't I make them well?".to_string(),
            excerpt: "Today I achieved what the masters said was impossible. I typed a lie that became true.".to_string(),
            attributed_to: "Unknown (handwriting analysis suggests the First Speaker)".to_string(),
            player_notes: Some("This is the First Speaker's journal. They had a spouse.".to_string()),
        },
        revelations: vec![
            "The First Speaker loved someone who got sick.".to_string(),
            "They married by writing in the Eternal Codex.".to_string(),
            "Their spouse's illness began from researching dangerous texts.".to_string(),
        ],
        related_fragments: vec!["first_speaker_journal_2".to_string(), "verity_private_letters".to_string()],
        mutable: false,
    });
    
    fragments.insert("first_speaker_journal_2".to_string(), LoreFragment {
        id: "first_speaker_journal_2".to_string(),
        title: "Personal Journal (Second Volume)".to_string(),
        category: LoreCategory::TheSundering,
        form: LoreForm::Book { pages: 150, condition: TextCondition::Damaged },
        location: "Shadow Quarter - Cipher's personal vault".to_string(),
        discovery_method: DiscoveryMethod::QuestReward { quest: "cipher_trust".to_string() },
        content: LoreContent {
            full_text: "Year 2990, Spring

                They died today.

                I typed 'live' over and over. I typed it until my fingers bled.
                Reality doesn't care. Reality has rules.
                But I make the rules. I write the rules.
                Who wrote the rule that says people die?

                Year 2990, Spring (three days later)

                I found it. In the Restricted Section. A text the founders hid.
                The First Scribe's notes on concepts they refused to formalize.

                Death is not a word. Death is a silence.
                The space between heartbeats.
                The pause at the end of a sentence.

                You cannot unwrite what was never written.
                But you can fill a silence.
                You can make it so there is no room for the pause.

                I know what I have to do.

                Year 2990, Summer

                Verity found out. She begged me to stop.
                'You can't unmake death,' she said. 'You'll unmake everything else.'

                She doesn't understand. She never lost anyone.
                When you lose someone, 'everything else' doesn't matter.

                I will see them again.
                I will type them back into existence.
                And if existence has to change to make room for them...
                Then existence was wrong.

                Year 2990, The Day With No Name

                It's done.

                I caused the Sundering. I filled the silence.
                And now

                [The rest of the journal is blank]".to_string(),
            excerpt: "Death is not a word. Death is a silence. You cannot unwrite what was never written.".to_string(),
            attributed_to: "The First Speaker".to_string(),
            player_notes: Some("The First Silence happened on 'The Day With No Name.'".to_string()),
        },
        revelations: vec![
            "The First Speaker deliberately caused the First Silence.".to_string(),
            "They were trying to resurrect their spouse.".to_string(),
            "Death is a 'silence,' not a word.".to_string(),
            "Verity knew and tried to stop them.".to_string(),
        ],
        related_fragments: vec!["first_speaker_journal_1".to_string(), "verity_testimony".to_string()],
        mutable: false,
    });
    
    // ========================================================================
    // PERSONAL STORIES
    // ========================================================================
    
    fragments.insert("verity_private_letters".to_string(), LoreFragment {
        id: "verity_private_letters".to_string(),
        title: "Letters Never Sent".to_string(),
        category: LoreCategory::PersonalStories,
        form: LoreForm::Letter { 
            sender: "Sister Verity".to_string(), 
            recipient: "The First Speaker".to_string() 
        },
        location: "Mages Guild Tower - Verity's sealed chambers".to_string(),
        discovery_method: DiscoveryMethod::FactionSecret { 
            faction: "MagesGuild".to_string(), 
            rank: "Voice Initiate".to_string() 
        },
        content: LoreContent {
            full_text: "My dearest friend,

                I write these knowing you will never read them. By the time anyone \
                finds this collection, you will be long dead or something worse, and \
                I will have joined you in whatever comes after.

                I should have told you about [name torn out]. I knew about the sickness \
                before you did. I knew it was my fault—something in the research I gave \
                you, something I should have checked more carefully. But I was afraid.

                I watched you type yourself hollow trying to save them. I watched you \
                disappear into the Restricted Section for days at a time. I knew what \
                you were looking for. I knew you would find it.

                I could have stopped you. I had the authority to seal that section \
                permanently. Instead, I did nothing.

                Was it because I was curious? Because part of me wanted to see if you \
                could do it? I have meditated on this for decades and I still don't \
                know. I don't know if I was a coward or a collaborator.

                All I know is this: when you caused the Sundering, I felt it. Every \
                scribe in the world felt it. We all knew, in that moment, that words \
                would never be trustworthy again.

                And we all knew it was my fault as much as yours.

                I'm sorry I wasn't stronger.
                I'm sorry I wasn't kinder.
                I'm sorry I was your teacher.

                May the words we failed preserve what we could not.

                — Verity".to_string(),
            excerpt: "I could have stopped you. Instead, I did nothing.".to_string(),
            attributed_to: "Sister Verity, The Unshaken Hand".to_string(),
            player_notes: Some("Verity blames herself as much as the First Speaker.".to_string()),
        },
        revelations: vec![
            "Verity accidentally caused the spouse's illness.".to_string(),
            "She could have prevented the First Silence but didn't.".to_string(),
            "She taught the First Speaker.".to_string(),
        ],
        related_fragments: vec!["first_speaker_journal_1".to_string(), "verity_testimony".to_string()],
        mutable: false,
    });
    
    fragments.insert("cipher_origin".to_string(), LoreFragment {
        id: "cipher_origin".to_string(),
        title: "The Name I Chose".to_string(),
        category: LoreCategory::PersonalStories,
        form: LoreForm::Inscription { material: "Hidden wall in the Shadow Quarter".to_string() },
        location: "Shadow Quarter - Original hideout".to_string(),
        discovery_method: DiscoveryMethod::Search { difficulty: 5 },
        content: LoreContent {
            full_text: "My name was [encrypted]. I was a librarian at Logos Prime.

                When the Sundering began, I was shelving books in the East Wing. \
                I heard the silence first—a silence that had weight, that had presence. \
                Then I heard the screaming.

                Books don't scream. Except when they do.

                I ran. Not toward the exit, like everyone else. I ran deeper. Toward \
                the Forbidden Section. I knew what was happening. I'd read the signs. \
                I knew the First Speaker had finally done it.

                And I knew what texts needed to be saved.

                I carried seven books out of Logos Prime. Seven books too dangerous \
                to exist, too important to lose. I carried them through corridors \
                that no longer believed in geometry. I carried them past the screaming \
                shelves and the bleeding pages.

                When I emerged, I was no longer [encrypted]. That person died in the \
                library. I chose a new name. A name that means 'hidden meaning.'

                I am Cipher now. I will be Cipher until someone decodes me.
                Or until the world ends.
                Whichever comes first.

                I have been watching the First Speaker's reincarnations for forty-seven years.
                They don't remember.
                That's how I know they're not yet ready to face what they did.
                When they remember, I'll decide whether to help them.
                Or stop them.".to_string(),
            excerpt: "I carried seven books out of Logos Prime. Seven books too dangerous to exist.".to_string(),
            attributed_to: "Cipher".to_string(),
            player_notes: Some("Cipher witnessed the First Silence. They've been watching the player.".to_string()),
        },
        revelations: vec![
            "Cipher was present at the First Silence.".to_string(),
            "The 'Forbidden Library' is seven specific books.".to_string(),
            "Cipher knows about the player's reincarnations.".to_string(),
            "They're waiting to see what the player chooses.".to_string(),
        ],
        related_fragments: vec!["forbidden_seven".to_string(), "first_speaker_journal_2".to_string()],
        mutable: false,
    });
    
    // ========================================================================
    // CORRUPTION STUDIES
    // ========================================================================
    
    fragments.insert("corruption_taxonomy".to_string(), LoreFragment {
        id: "corruption_taxonomy".to_string(),
        title: "A Classification of Linguistic Decay".to_string(),
        category: LoreCategory::BlightStudies,
        form: LoreForm::Book { pages: 78, condition: TextCondition::Worn },
        location: "Athenaeum - Research Section".to_string(),
        discovery_method: DiscoveryMethod::Gift { 
            from: "Archivist Vera".to_string(), 
            condition: Some("Complete research quest".to_string())
        },
        content: LoreContent {
            full_text: "A Classification of Linguistic Decay
                by Archivist Chen, Year 32 of the Sundering

                INTRODUCTION

                The phenomenon we call 'Corruption' is not singular. Through careful \
                observation, I have identified six distinct forms of linguistic decay, \
                each with unique characteristics and recommended countermeasures.

                TYPE ONE: SEMANTIC DECAY
                Words lose their meanings gradually. 'Love' might come to mean 'rock.' \
                'Safety' might mean 'danger.' This is the most common form and the \
                most insidious. Countermeasure: Constant repetition of correct definitions.

                TYPE TWO: LITERAL MANIFESTATION
                Written words become physically real in harmful ways. Write 'fire' and \
                flames appear. This type is rare but immediately dangerous. \
                Countermeasure: Avoid concrete nouns in affected areas.

                TYPE THREE: BABEL EFFECT
                Languages blend and fracture. Sentences contain words from tongues that \
                never existed. Communication becomes impossible. \
                Countermeasure: Speak slowly, use gestures, rely on context.

                TYPE FOUR: TRUTH INVERSION
                Everything written becomes false. Write 'the sky is blue' and it turns \
                green. This type can be exploited but is dangerous to rely on. \
                Countermeasure: Write what you don't want. (Risky.)

                TYPE FIVE: GRAPHEME PARASITISM
                Letters become predatory, consuming other letters. Words shorten, then \
                vanish entirely. Silence spreads. \
                Countermeasure: Constant vigilance to reinforce reality.

                TYPE SIX: LINGUISTIC ACCELERATION
                Time flows differently through text. Words age rapidly. Past tense \
                becomes present. Future becomes past. \
                Countermeasure: Type quickly, never pause, avoid temporal references.

                CONCLUSION

                I believe these six types represent fragments of a single phenomenon: \
                the Sundering. Each is a different symptom of reality trying to reject \
                the wound caused by [the next page is torn out]".to_string(),
            excerpt: "The phenomenon we call 'Corruption' is not singular.".to_string(),
            attributed_to: "Archivist Chen, Year 32 of the Sundering".to_string(),
            player_notes: Some("The six corruption types match the game's mechanical system.".to_string()),
        },
        revelations: vec![
            "The Corruption has six distinct forms.".to_string(),
            "Each type requires different countermeasures.".to_string(),
            "All types are symptoms of 'the wound' (the First Silence).".to_string(),
        ],
        related_fragments: vec!["corruption_patterns".to_string(), "first_speaker_journal_2".to_string()],
        mutable: true,
    });
    
    // ========================================================================
    // PROPHECY FRAGMENTS
    // ========================================================================
    
    fragments.insert("tomorrow_text_7".to_string(), LoreFragment {
        id: "tomorrow_text_7".to_string(),
        title: "Tomorrow Text, Fragment Seven".to_string(),
        category: LoreCategory::Prophecy,
        form: LoreForm::Book { pages: 1, condition: TextCondition::Fragmentary },
        location: "Mages Guild Tower - Prophecy vault".to_string(),
        discovery_method: DiscoveryMethod::QuestReward { quest: "scribe_prophecy".to_string() },
        content: LoreContent {
            full_text: "When silence speaks and speech falls silent,
                When the named become the nameless,
                When the written word forgets itself—

                One will come who is many.
                One who died yet lives.
                One who remembers by forgetting.

                They will stand at the wound's edge.
                They will hold the quill that cuts.
                They will face three doors:

                The door of ending.
                The door of beginning.
                The door that is not a door.

                Choose silence: all things sleep.
                Choose speech: all things wake.
                Choose neither: all things change.

                The quill awaits the hand.
                The hand awaits the heart.
                The heart awaits itself.".to_string(),
            excerpt: "One will come who is many. One who died yet lives. One who remembers by forgetting.".to_string(),
            attributed_to: "Unknown prophet, pre-Sundering".to_string(),
            player_notes: Some("This prophecy describes the player.".to_string()),
        },
        revelations: vec![
            "The player was prophesied before the Sundering.".to_string(),
            "There are three possible outcomes.".to_string(),
            "The 'door that is not a door' suggests a hidden path.".to_string(),
        ],
        related_fragments: vec!["tomorrow_text_3".to_string(), "first_archivist_note".to_string()],
        mutable: false,
    });
    
    // ========================================================================
    // ARTS & CULTURE
    // ========================================================================
    
    fragments.insert("last_poem_of_logos".to_string(), LoreFragment {
        id: "last_poem_of_logos".to_string(),
        title: "The Last Poem of Logos Prime".to_string(),
        category: LoreCategory::Arts,
        form: LoreForm::Song { verses: 4 },
        location: "Haven - Inn, sung by bard".to_string(),
        discovery_method: DiscoveryMethod::Gift { 
            from: "Traveling bard".to_string(), 
            condition: Some("Listen to their full performance".to_string())
        },
        content: LoreContent {
            full_text: "In halls where words once walked like lords
                And meaning held its court,
                A silence fell like winter snow
                Where summer once held sport.

                The library burned from inside out,
                The books wept ink like tears,
                The scribes who stayed to save their words
                Have not been seen in years.

                So raise a glass to Logos Prime,
                The city built of speech,
                Where someone loved too much, too long,
                And lost what they would teach.

                Type true, type steady, type with care,
                Remember what they lost:
                That some words cannot be unwritten
                No matter what the cost.".to_string(),
            excerpt: "Where someone loved too much, too long, and lost what they would teach.".to_string(),
            attributed_to: "Traditional, post-Sundering".to_string(),
            player_notes: Some("Folk memory of the First Silence. Mostly accurate.".to_string()),
        },
        revelations: vec![
            "Common people know the basic story.".to_string(),
            "The emotional truth ('loved too much') is remembered.".to_string(),
            "The warning ('some words cannot be unwritten') persists.".to_string(),
        ],
        related_fragments: vec!["founding_of_logos".to_string(), "first_speaker_journal_2".to_string()],
        mutable: false,
    });
    
    // ========================================================================
    // PLAYER HISTORY (discovered late game)
    // ========================================================================
    
    fragments.insert("player_previous_life".to_string(), LoreFragment {
        id: "player_previous_life".to_string(),
        title: "Index Entry: The One Who Returns".to_string(),
        category: LoreCategory::PlayerHistory,
        form: LoreForm::Book { pages: 1, condition: TextCondition::Pristine },
        location: "Athenaeum - Index of Everything".to_string(),
        discovery_method: DiscoveryMethod::FactionSecret { 
            faction: "Archivists".to_string(), 
            rank: "Inner Circle".to_string()
        },
        content: LoreContent {
            full_text: "ENTRY: THE ONE WHO RETURNS
                (also known as: The First Speaker, The Unwriter, The Grief-Mad, \
                The One Who Would Not Let Go)

                Current incarnation: [ENCRYPTED - KEY REQUIRED]
                Previous incarnations: 47 recorded, suspected 60+
                First recorded: Year 2890 of the Age of Writing
                Last recorded death: Year 46 of the Sundering

                SUMMARY:
                This individual is causally linked to the First Silence. Following \
                the event, they have repeatedly reincarnated, each time with no \
                memory of previous lives. The reincarnation appears to be a side \
                effect of their partial unwriting of death.

                PATTERN:
                Each incarnation eventually recovers their memories. Upon recovery, \
                they either:
                1. Attempt to complete the Sundering (13 times)
                2. Attempt to reverse the Sundering (8 times)
                3. Choose suicide to prevent either (26 times)

                None have succeeded. The wound persists.

                CURRENT STATUS:
                Active. Located in Haven. Memories suppressed.

                RECOMMENDATION:
                Observe. Do not interfere. Previous interference has accelerated \
                memory recovery without improving outcomes.

                NOTE FROM THE FIRST ARCHIVIST:
                Let them choose this time. That's all they ever wanted—to choose.
                We owe them that much.".to_string(),
            excerpt: "Current incarnation: [ENCRYPTED - KEY REQUIRED]".to_string(),
            attributed_to: "The Index of Everything".to_string(),
            player_notes: Some("I've lived this life before. Many times. I've never succeeded.".to_string()),
        },
        revelations: vec![
            "The player is the First Speaker, reincarnated.".to_string(),
            "They've tried to fix things 47+ times.".to_string(),
            "They keep choosing to forget.".to_string(),
            "The First Archivist has been watching all along.".to_string(),
        ],
        related_fragments: vec!["first_speaker_journal_2".to_string(), "cipher_origin".to_string()],
        mutable: false,
    });
    
    fragments
}

/// Lore collection progress tracking
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoreJournal {
    /// Fragments discovered this run
    pub discovered: HashMap<String, bool>,
    /// When each fragment was discovered
    pub discovery_order: Vec<String>,
    /// Notes the player has added
    pub player_notes: HashMap<String, String>,
    /// Connections the player has made
    pub connections_made: Vec<(String, String)>,
    /// Percentage of lore discovered
    pub completion_percent: f32,
}

impl LoreJournal {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn discover(&mut self, fragment_id: &str) {
        if !self.discovered.contains_key(fragment_id) {
            self.discovered.insert(fragment_id.to_string(), true);
            self.discovery_order.push(fragment_id.to_string());
            self.update_completion();
        }
    }
    
    pub fn has_discovered(&self, fragment_id: &str) -> bool {
        *self.discovered.get(fragment_id).unwrap_or(&false)
    }
    
    pub fn add_note(&mut self, fragment_id: &str, note: &str) {
        self.player_notes.insert(fragment_id.to_string(), note.to_string());
    }
    
    pub fn connect(&mut self, fragment_a: &str, fragment_b: &str) {
        self.connections_made.push((fragment_a.to_string(), fragment_b.to_string()));
    }
    
    fn update_completion(&mut self) {
        let total = build_lore_fragments().len() as f32;
        let discovered = self.discovered.len() as f32;
        self.completion_percent = (discovered / total) * 100.0;
    }
}
