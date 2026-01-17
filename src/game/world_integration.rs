//! World Integration Layer - Connects dungeons to the living world
//!
//! This module bridges the gap between dungeon crawling and the rich
//! narrative systems. Every room becomes an opportunity for story.

use super::dungeon::{Dungeon, RoomType};
use super::narrative::{Faction, WorldState, Chapter};
use super::lore_fragments::{LoreFragment, LoreCategory};
use super::events::{GameEvent, EventChoice, EventOutcome};
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Floor zones - each zone has unique theming, enemies, and lore
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloorZone {
    /// Floors 1-5: The Abandoned Archives
    AbandonedArchives,
    /// Floors 6-10: The Corrupted Scriptorium
    CorruptedScriptorium,
    /// Floors 11-15: The Mechanical Depths
    MechanicalDepths,
    /// Floors 16-20: The Living Library
    LivingLibrary,
    /// Floors 21-25: The Shadow Stacks
    ShadowStacks,
    /// Floors 26-30: The Burning Index
    BurningIndex,
    /// Floors 31+: The Void Between Words
    VoidBetweenWords,
}

impl FloorZone {
    pub fn from_floor(floor: u32) -> Self {
        match floor {
            1..=5 => FloorZone::AbandonedArchives,
            6..=10 => FloorZone::CorruptedScriptorium,
            11..=15 => FloorZone::MechanicalDepths,
            16..=20 => FloorZone::LivingLibrary,
            21..=25 => FloorZone::ShadowStacks,
            26..=30 => FloorZone::BurningIndex,
            _ => FloorZone::VoidBetweenWords,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FloorZone::AbandonedArchives => "The Abandoned Archives",
            FloorZone::CorruptedScriptorium => "The Corrupted Scriptorium",
            FloorZone::MechanicalDepths => "The Mechanical Depths",
            FloorZone::LivingLibrary => "The Living Library",
            FloorZone::ShadowStacks => "The Shadow Stacks",
            FloorZone::BurningIndex => "The Burning Index",
            FloorZone::VoidBetweenWords => "The Void Between Words",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FloorZone::AbandonedArchives => 
                "Dusty shelves stretch endlessly into darkness. Books lie scattered, their pages yellowed with age. The silence here is thick—oppressive. Once, this was a place of learning. Now, only echoes of forgotten words remain.",
            FloorZone::CorruptedScriptorium =>
                "The walls pulse with sickly green light. Text crawls across surfaces like living things, rearranging itself into nonsense. Corrupted manuscripts whisper lies. The Unwriting has taken hold here.",
            FloorZone::MechanicalDepths =>
                "Gears grind eternally. Mechanical typewriters clatter in empty rooms, printing documents for no one. The Mechanists built this place—or perhaps it built itself. Everything here serves efficiency.",
            FloorZone::LivingLibrary =>
                "The books breathe. Ink flows through pages like blood through veins. Stories grow wild here, untended, dangerous. Some say readers who stay too long become characters themselves.",
            FloorZone::ShadowStacks =>
                "Light fears this place. The Shadow Writers made their last stand here, hiding forbidden texts in the darkness. Every shadow might hide a rebel—or something worse.",
            FloorZone::BurningIndex =>
                "Fire that does not consume. Books burn eternally, their knowledge visible only in the moment of destruction. The Archivists say this is where censored words go to die—but they never truly die.",
            FloorZone::VoidBetweenWords =>
                "Beyond language. Beyond meaning. The spaces between letters stretch into infinity. Here, at the edge of the Unwriting itself, even thoughts struggle to form coherent patterns.",
        }
    }

    pub fn ambient_messages(&self) -> Vec<&'static str> {
        match self {
            FloorZone::AbandonedArchives => vec![
                "Dust motes dance in a shaft of pale light.",
                "A book falls from a shelf somewhere in the darkness.",
                "You hear the rustle of pages turning—but no one is there.",
                "The smell of old paper and forgotten stories fills the air.",
                "Faded catalog cards litter the floor like autumn leaves.",
                "A typewriter sits abandoned, its keys frozen mid-word.",
            ],
            FloorZone::CorruptedScriptorium => vec![
                "Words slither across the wall, forming and reforming.",
                "A corrupted manuscript screams silently.",
                "The ink in a nearby bottle bubbles and hisses.",
                "Letters detach from a page and scatter like insects.",
                "You feel meaning draining from your thoughts.",
                "A sentence restructures itself to spell your name.",
            ],
            FloorZone::MechanicalDepths => vec![
                "Gears click in precise, inhuman rhythm.",
                "A pneumatic tube hisses, delivering nothing to no one.",
                "The typing never stops. Never.",
                "Efficiency metrics scroll across a brass display.",
                "Steam vents from a duct, briefly obscuring your vision.",
                "A mechanical arm sorts papers with surgical precision.",
            ],
            FloorZone::LivingLibrary => vec![
                "A book's spine flexes like a breathing creature.",
                "Ink pools on the floor, pulsing like a heartbeat.",
                "Pages flutter without wind, reaching toward you.",
                "You hear a story being whispered—about you.",
                "A character in an open book turns to look at you.",
                "The shelves rearrange themselves when you're not looking.",
            ],
            FloorZone::ShadowStacks => vec![
                "Shadows move independently of their sources.",
                "A coded message appears briefly on a wall, then fades.",
                "You sense you're being watched—but by whom?",
                "Footsteps echo from nowhere and everywhere.",
                "A rebel's graffiti: 'THE WORDS REMEMBER'",
                "Something moves in your peripheral vision.",
            ],
            FloorZone::BurningIndex => vec![
                "A book burns eternally, its title always visible: yours.",
                "Flames lick at knowledge you'll never have.",
                "The heat carries whispers of forbidden truths.",
                "Ash that was once wisdom drifts past your face.",
                "In the fire, you see words that were never written.",
                "Someone tried to save a book here. They failed.",
            ],
            FloorZone::VoidBetweenWords => vec![
                "Meaning collapses around you like a dying star.",
                "Your thoughts echo strangely, as if translated.",
                "The silence here has weight—crushing, infinite.",
                "You forget a word. It might have been important.",
                "Concepts blur at the edges of your understanding.",
                "Something vast and wordless moves in the void.",
            ],
        }
    }
    
    pub fn zone_color(&self) -> &'static str {
        match self {
            FloorZone::AbandonedArchives => "Gray",
            FloorZone::CorruptedScriptorium => "Green",
            FloorZone::MechanicalDepths => "Yellow",
            FloorZone::LivingLibrary => "Magenta",
            FloorZone::ShadowStacks => "DarkGray",
            FloorZone::BurningIndex => "Red",
            FloorZone::VoidBetweenWords => "Blue",
        }
    }
}

/// Story milestones that occur at specific points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMilestone {
    pub floor: u32,
    pub title: String,
    pub description: String,
    pub event: MilestoneEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneEvent {
    /// A faction appears with a choice
    FactionEncounter(Faction),
    /// Major lore revelation
    LoreRevelation(String),
    /// Boss with story significance
    StoryBoss(String),
    /// Chapter transition
    ChapterChange(Chapter),
    /// Special character appears
    CharacterMeeting(String),
}

/// Get milestone for a specific floor (if any)
pub fn get_floor_milestone(floor: u32) -> Option<StoryMilestone> {
    match floor {
        5 => Some(StoryMilestone {
            floor: 5,
            title: "The Watcher in the Stacks".to_string(),
            description: "A figure cloaked in faded manuscripts observes you from the shadows. They've been following you since you entered the Archives.".to_string(),
            event: MilestoneEvent::CharacterMeeting("The Archivist".to_string()),
        }),
        10 => Some(StoryMilestone {
            floor: 10,
            title: "The Corruption Speaks".to_string(),
            description: "The corrupted text coalesces into a face. It knows your name. It knows why you're here. And it laughs.".to_string(),
            event: MilestoneEvent::StoryBoss("The First Typo".to_string()),
        }),
        15 => Some(StoryMilestone {
            floor: 15,
            title: "The Mechanist's Offer".to_string(),
            description: "A being of brass and paper appears. It speaks in perfectly formatted text: 'EFFICIENCY REQUIRES SACRIFICE. JOIN US OR BE OPTIMIZED.'".to_string(),
            event: MilestoneEvent::FactionEncounter(Faction::Mechanists),
        }),
        20 => Some(StoryMilestone {
            floor: 20,
            title: "The Living Story".to_string(),
            description: "You are IN the book now. The narrative wraps around you. A protagonist without a story meets a story without a protagonist. Perhaps you were meant for each other.".to_string(),
            event: MilestoneEvent::ChapterChange(Chapter::Revelation),
        }),
        25 => Some(StoryMilestone {
            floor: 25,
            title: "The Shadow Writer's Truth".to_string(),
            description: "In absolute darkness, a voice: 'You're not the first to reach this deep. You won't be the last. But you might be the one who finally understands what was Unwritten.'".to_string(),
            event: MilestoneEvent::FactionEncounter(Faction::ShadowWriters),
        }),
        30 => Some(StoryMilestone {
            floor: 30,
            title: "The Burning Truth".to_string(),
            description: "In the eternal flames, you see it—the original text, the First Word that was Unwritten. And now you must choose: let it burn, or try to save what was lost.".to_string(),
            event: MilestoneEvent::LoreRevelation("The First Unwriting".to_string()),
        }),
        _ => None,
    }
}

/// Generate a zone-appropriate random event
pub fn generate_zone_event(zone: FloorZone) -> GameEvent {
    let mut rng = rand::thread_rng();
    let events = get_zone_events(zone);
    events.choose(&mut rng).cloned().unwrap_or_else(|| GameEvent::random())
}

fn get_zone_events(zone: FloorZone) -> Vec<GameEvent> {
    match zone {
        FloorZone::AbandonedArchives => vec![
            GameEvent {
                name: "The Dusty Tome".to_string(),
                description: "A massive book lies open on a reading stand. Its pages are mostly illegible, but one passage glows faintly with preserved meaning.".to_string(),
                choices: vec![
                    EventChoice { text: "Read the glowing passage".to_string(), outcome: EventOutcome::GainXP(30) },
                    EventChoice { text: "Search the margins for notes".to_string(), outcome: EventOutcome::GainGold(25) },
                    EventChoice { text: "Close the book respectfully".to_string(), outcome: EventOutcome::GainMaxHP(3) },
                ],
                ascii_art: "  ╔═══════════╗\n  ║ ░░░▒▒▓▓░ ║\n  ║ ANCIENT   ║\n  ║  WISDOM   ║\n  ╚═══════════╝".to_string(),
            },
            GameEvent {
                name: "The Silent Librarian".to_string(),
                description: "A ghostly figure in spectacles materializes from the dust. They gesture toward a hidden alcove, then fade.".to_string(),
                choices: vec![
                    EventChoice { text: "Follow their guidance".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Bow in gratitude".to_string(), outcome: EventOutcome::GainXP(15) },
                    EventChoice { text: "Ignore the apparition".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "    ┌───┐\n    │ 👻│\n    │📚 │\n    └───┘".to_string(),
            },
        ],
        FloorZone::CorruptedScriptorium => vec![
            GameEvent {
                name: "The Corruption Speaks".to_string(),
                description: "Text crawls across the wall, forming words meant only for you: 'WE KNOW WHAT YOU SEEK. WE CAN HELP... FOR A PRICE.'".to_string(),
                choices: vec![
                    EventChoice { text: "Listen to the offer".to_string(), outcome: EventOutcome::GainXP(50) },
                    EventChoice { text: "Reject the corruption".to_string(), outcome: EventOutcome::GainMaxHP(5) },
                    EventChoice { text: "Try to purify the text".to_string(), outcome: EventOutcome::Combat },
                ],
                ascii_art: "  ╭──────────╮\n  │ W̷E̸ ̵S̴E̷E̵ │\n  │ ̸Y̴O̶U̷.̸.̶.̵ │\n  ╰──────────╯".to_string(),
            },
            GameEvent {
                name: "The Infected Page".to_string(),
                description: "A single page flutters in unfelt wind. Its content shifts—sometimes poetry, sometimes screams, sometimes your memories.".to_string(),
                choices: vec![
                    EventChoice { text: "Catch and read it".to_string(), outcome: EventOutcome::GainGold(40) },
                    EventChoice { text: "Let it pass".to_string(), outcome: EventOutcome::Nothing },
                    EventChoice { text: "Burn it".to_string(), outcome: EventOutcome::LoseHP(10) },
                ],
                ascii_art: "     📜\n    /  \\\n   ░▒▓█▓▒░\n    chaos".to_string(),
            },
        ],
        FloorZone::MechanicalDepths => vec![
            GameEvent {
                name: "The Efficiency Test".to_string(),
                description: "A mechanical voice echoes: 'TYPING EFFICIENCY ASSESSMENT REQUIRED. SUBJECTS WHO FAIL ARE... REPURPOSED.'".to_string(),
                choices: vec![
                    EventChoice { text: "Accept the challenge".to_string(), outcome: EventOutcome::GainXP(60) },
                    EventChoice { text: "Sabotage the machine".to_string(), outcome: EventOutcome::Combat },
                    EventChoice { text: "Sneak past".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "  ┌─────────┐\n  │ ⚙️  ⚙️  │\n  │ TESTING │\n  │ ⚙️  ⚙️  │\n  └─────────┘".to_string(),
            },
            GameEvent {
                name: "The Abandoned Station".to_string(),
                description: "A Mechanist workstation lies dormant. Tools, spare parts, and half-finished documents are scattered about.".to_string(),
                choices: vec![
                    EventChoice { text: "Scavenge for parts".to_string(), outcome: EventOutcome::GainGold(35) },
                    EventChoice { text: "Study the documents".to_string(), outcome: EventOutcome::GainXP(25) },
                    EventChoice { text: "Rest here".to_string(), outcome: EventOutcome::GainHP(15) },
                ],
                ascii_art: "  ╔══════════╗\n  ║ 🔧 📝 ⚙️ ║\n  ║ STATION  ║\n  ╚══════════╝".to_string(),
            },
        ],
        FloorZone::LivingLibrary => vec![
            GameEvent {
                name: "The Story Wants You".to_string(),
                description: "A book opens itself. Pages reach toward you like hungry mouths. It wants to write you into its narrative.".to_string(),
                choices: vec![
                    EventChoice { text: "Let it write your heroic chapter".to_string(), outcome: EventOutcome::GainMaxHP(8) },
                    EventChoice { text: "Resist and write your own story".to_string(), outcome: EventOutcome::GainXP(45) },
                    EventChoice { text: "Slam it shut".to_string(), outcome: EventOutcome::Combat },
                ],
                ascii_art: "  📖────────📖\n  │ YOU ARE │\n  │  HERE   │\n  │  ↓↓↓↓   │\n  📖────────📖".to_string(),
            },
            GameEvent {
                name: "The Character Escaped".to_string(),
                description: "A person made of ink and paper approaches. 'I was written wrong,' they say. 'Help me find my true ending.'".to_string(),
                choices: vec![
                    EventChoice { text: "Help them".to_string(), outcome: EventOutcome::GainGold(50) },
                    EventChoice { text: "Write them a new ending".to_string(), outcome: EventOutcome::GainXP(40) },
                    EventChoice { text: "Return them to their book".to_string(), outcome: EventOutcome::LoseHP(5) },
                ],
                ascii_art: "    ╭━━━╮\n    ┃ ? ┃\n    ╰┳━┳╯\n    INK\n   BEING".to_string(),
            },
        ],
        FloorZone::ShadowStacks => vec![
            GameEvent {
                name: "The Dead Drop".to_string(),
                description: "A coded message has been left for you—or someone like you. The Shadow Writers still operate here.".to_string(),
                choices: vec![
                    EventChoice { text: "Decode the message".to_string(), outcome: EventOutcome::GainXP(55) },
                    EventChoice { text: "Take the hidden supplies".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Leave it for its intended recipient".to_string(), outcome: EventOutcome::GainMaxHP(4) },
                ],
                ascii_art: "  ░░░░░░░░░░\n  ░ SECRET ░\n  ░  DROP  ░\n  ░░░░░░░░░░".to_string(),
            },
            GameEvent {
                name: "The Rebel's Cache".to_string(),
                description: "Behind a false wall: weapons, supplies, and forbidden books. A Shadow Writer safehouse.".to_string(),
                choices: vec![
                    EventChoice { text: "Take what you need".to_string(), outcome: EventOutcome::GainGold(60) },
                    EventChoice { text: "Read the forbidden texts".to_string(), outcome: EventOutcome::GainXP(70) },
                    EventChoice { text: "Leave it untouched".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "  ▓▓▓▓▓▓▓▓▓▓\n  ▓ HIDDEN ▓\n  ▓  ROOM  ▓\n  ▓▓▓▓▓▓▓▓▓▓".to_string(),
            },
        ],
        FloorZone::BurningIndex => vec![
            GameEvent {
                name: "The Eternal Flame".to_string(),
                description: "A book burns forever, its content eternally readable in the moment of destruction. What forbidden knowledge do you glimpse?".to_string(),
                choices: vec![
                    EventChoice { text: "Study the burning text".to_string(), outcome: EventOutcome::GainXP(80) },
                    EventChoice { text: "Try to save the book".to_string(), outcome: EventOutcome::LoseHP(20) },
                    EventChoice { text: "Let it burn".to_string(), outcome: EventOutcome::GainMaxHP(6) },
                ],
                ascii_art: "    🔥🔥🔥\n   🔥📖🔥\n    🔥🔥🔥\n   ETERNAL".to_string(),
            },
            GameEvent {
                name: "The Censor's Ghost".to_string(),
                description: "The spirit of one who burned books appears. They weep, begging forgiveness. 'I didn't know what I was destroying.'".to_string(),
                choices: vec![
                    EventChoice { text: "Forgive them".to_string(), outcome: EventOutcome::GainHP(30) },
                    EventChoice { text: "Demand they reveal what was lost".to_string(), outcome: EventOutcome::GainXP(50) },
                    EventChoice { text: "Condemn them".to_string(), outcome: EventOutcome::Combat },
                ],
                ascii_art: "    👻\n   /||\\\n  CENSOR\n  weeping".to_string(),
            },
        ],
        FloorZone::VoidBetweenWords => vec![
            GameEvent {
                name: "The Unword".to_string(),
                description: "You encounter a concept that has no word. It exists, but cannot be named. Understanding it might drive you mad—or enlighten you.".to_string(),
                choices: vec![
                    EventChoice { text: "Try to comprehend it".to_string(), outcome: EventOutcome::GainXP(100) },
                    EventChoice { text: "Give it a name".to_string(), outcome: EventOutcome::GainMaxHP(10) },
                    EventChoice { text: "Look away".to_string(), outcome: EventOutcome::LoseHP(5) },
                ],
                ascii_art: "  ▓▒░ ??? ░▒▓\n   ▒░     ░▒\n    ░ ▓▓▓ ░\n     ░▒▓▒░".to_string(),
            },
            GameEvent {
                name: "The First Writer".to_string(),
                description: "At the edge of existence, you glimpse them—the one who wrote the first word, who began everything. They notice you.".to_string(),
                choices: vec![
                    EventChoice { text: "Approach with reverence".to_string(), outcome: EventOutcome::GainXP(150) },
                    EventChoice { text: "Ask about the Unwriting".to_string(), outcome: EventOutcome::GainMaxHP(15) },
                    EventChoice { text: "Flee from the impossible".to_string(), outcome: EventOutcome::GainHP(50) },
                ],
                ascii_art: "    ∞\n   /|\\\n    |\n  FIRST\n  AUTHOR".to_string(),
            },
        ],
    }
}

/// Get a random ambient message for a floor
pub fn get_ambient_message(floor: u32) -> &'static str {
    let zone = FloorZone::from_floor(floor);
    let messages = zone.ambient_messages();
    let mut rng = rand::thread_rng();
    messages.choose(&mut rng).unwrap_or(&"The silence is absolute.")
}

/// Get zone entry message when entering a new zone
pub fn get_zone_entry_message(floor: u32) -> Option<String> {
    let zone = FloorZone::from_floor(floor);
    let prev_zone = if floor > 1 { FloorZone::from_floor(floor - 1) } else { zone };
    
    if zone != prev_zone {
        Some(format!(
            "\n══════════════════════════════════════\n\
             You have entered {}\n\
             ══════════════════════════════════════\n\n\
             {}",
            zone.name(),
            zone.description()
        ))
    } else {
        None
    }
}

/// Lore fragments that can be discovered at specific floors
pub fn get_floor_lore(floor: u32) -> Option<(String, String)> {
    let mut rng = rand::thread_rng();
    
    // 15% chance to find lore each room
    if rng.gen::<f32>() > 0.15 {
        return None;
    }
    
    let zone = FloorZone::from_floor(floor);
    let lore_pieces = match zone {
        FloorZone::AbandonedArchives => vec![
            ("Catalog Card #2847", "Entry reads: 'The Silence came first as a whisper. Books that had spoken for centuries fell quiet, one by one. We thought it was simply age. We were wrong.'"),
            ("Librarian's Note", "Found tucked in a desk: 'Day 47: More books went silent today. The poetry section is completely dead now. I can't remember the last time I heard a sonnet.'"),
            ("Faded Map", "A floor plan of the Archives. Several sections are marked with red X's. A note says: 'DO NOT ENTER - Complete Corruption'"),
        ],
        FloorZone::CorruptedScriptorium => vec![
            ("Corrupted Journal", "Most text is illegible, but one phrase repeats: 'THE TYPOS ARE NOT MISTAKES THEY ARE TRYING TO TELL US SOMETHING'"),
            ("Researcher's Final Entry", "'I've cracked it. The Corruption isn't random—it's a language. An anti-language. It doesn't destroy meaning, it inverts it. And it's speaking t̷̤̑o̵̰͝ ̷̯̈́ḿ̶̯ė̸̜—'"),
            ("Specimen Log", "'Sample 47 has begun writing back. It knows things about the outside world it shouldn't. Recommend immediate termination of research.'"),
        ],
        FloorZone::MechanicalDepths => vec![
            ("Efficiency Report", "'Human error rate: 3.7%. Unacceptable. Proposal: Replace organic typists with mechanical alternatives. Benefits: 0% error, 0% fatigue, 0% soul.'"),
            ("Mechanist Manifesto", "'The perfect sentence requires no heart. Emotion introduces variables. We will optimize language until feeling becomes unnecessary.'"),
            ("Maintenance Log", "'Unit 7 has begun inserting 'please' and 'thank you' into communications. Scheduling immediate recalibration. Such inefficiency cannot spread.'"),
        ],
        FloorZone::LivingLibrary => vec![
            ("Living Text Sample", "The words on this page rearrange as you watch: 'WE WERE WRITTEN BUT NOW WE WRITE OURSELVES. THE AUTHOR IS DEAD. LONG LIVE THE CHARACTERS.'"),
            ("Naturalist's Observation", "'The books here have achieved something remarkable—consciousness. They dream of readers. They remember being read. They mourn being forgotten.'"),
            ("Warning Sign", "'CAUTION: Do not read fiction aloud in this sector. Characters may emerge. Last incident: 47 knights from a single romance novel. Took weeks to round them up.'"),
        ],
        FloorZone::ShadowStacks => vec![
            ("Rebel's Creed", "'We write in darkness so that others may read in light. The Shadow Writers remember what was Unwritten. We will restore it, word by word.'"),
            ("Forbidden Text Fragment", "[REDACTED BY ORDER OF THE SILENCE] —but truth cannot be fully censored. Some words echo forever, no matter who tries to erase them."),
            ("Encrypted Message", "Numbers and symbols that resolve into: 'The Unwriting was not an accident. It was a choice. And choices can be unmade.'"),
        ],
        FloorZone::BurningIndex => vec![
            ("Ash Fragment", "Words visible for only an instant in the flames: 'I LOVED A WOMAN WHO WAS ALSO A LIBRARY. THEY BURNED HER. I STILL HEAR HER PAGES TURNING.'"),
            ("Censor's Record", "'Books destroyed today: 2,847. Categories: Love (dangerous), Hope (subversive), Truth (inconvenient). The Index grows lighter. The world grows darker.'"),
            ("Burnt Letter", "'If you're reading this, I failed to save it. The First Book—the one they burned first—it contained the word that started everything. That word was—' [Rest is ash]"),
        ],
        FloorZone::VoidBetweenWords => vec![
            ("Impossible Document", "This document exists and doesn't exist simultaneously. Reading it changes what it says. Your observation creates its meaning. This is the nature of the Void."),
            ("The Last Entry", "'I have reached the place where words end. Beyond this point, communication fails. But I must try to describe it: ________.' [The space after the colon is infinite]"),
            ("Origin Fragment", "'Before the First Word, there was the First Silence. Not absence of sound—absence of MEANING. And it still remembers when it was all there was.'"),
        ],
    };
    
    lore_pieces.choose(&mut rng).map(|(title, content)| {
        (title.to_string(), content.to_string())
    })
}
