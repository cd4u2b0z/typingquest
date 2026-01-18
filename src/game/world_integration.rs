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
    /// Floors 1-2: The Shattered Halls - Ruined throne rooms of Valdris
    ShatteredHalls,
    /// Floors 3-4: The Sunken Archives - Flooded libraries of lost knowledge
    SunkenArchives,
    /// Floors 5-6: The Blighted Gardens - Corrupted royal gardens
    BlightedGardens,
    /// Floors 7-8: The Clockwork Depths - Ancient mechanisms still grinding
    ClockworkDepths,
    /// Floors 9-10: The Void's Edge - Where reality breaks down
    VoidsEdge,
    /// Floors 11+: The Breach - The wound in reality itself
    TheBreach,
}

impl FloorZone {
    pub fn from_floor(floor: u32) -> Self {
        match floor {
            1..=2 => FloorZone::ShatteredHalls,
            3..=4 => FloorZone::SunkenArchives,
            5..=6 => FloorZone::BlightedGardens,
            7..=8 => FloorZone::ClockworkDepths,
            9..=10 => FloorZone::VoidsEdge,
            _ => FloorZone::TheBreach,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FloorZone::ShatteredHalls => "The Shattered Halls",
            FloorZone::SunkenArchives => "The Sunken Archives",
            FloorZone::BlightedGardens => "The Blighted Gardens",
            FloorZone::ClockworkDepths => "The Clockwork Depths",
            FloorZone::VoidsEdge => "The Void's Edge",
            FloorZone::TheBreach => "The Breach",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FloorZone::ShatteredHalls => 
                "Ruined corridors of what was once a grand palace. Debris and cobwebs fill the once-magnificent throne rooms where kings held court and heroes were honored. Whispers of old courtiers echo through the darkness.",
            FloorZone::SunkenArchives =>
                "Flooded libraries full of rotting books. The greatest collection of magical knowledge ever assembled lies drowned here. Malachar studied in these halls for decades before his fall.",
            FloorZone::BlightedGardens =>
                "Once-beautiful gardens now twisted by the Blight. Plants move with malevolent purpose. Flowers have teeth. The royal gardens, where Malachar's beloved once walked, now hunger for blood.",
            FloorZone::ClockworkDepths =>
                "Ancient mechanisms still grinding, traps and automatons guarding forgotten vaults. Malachar's workshops, where he built the apparatus for the Ritual of Ascension. Some machines still wait for commands.",
            FloorZone::VoidsEdge =>
                "Reality breaks down. Geometry fails. The Breach looms ahead, bleeding darkness. The ritual chamber—ground zero of the Sundering. This is where the world broke.",
            FloorZone::TheBreach =>
                "Beyond the Veil. The wound in reality bleeds the Void itself. Nothing here follows the laws of the world. Only the mad or the determined venture this deep.",
        }
    }

    pub fn ambient_messages(&self) -> Vec<&'static str> {
        match self {
            FloorZone::ShatteredHalls => vec![
                "Dust motes dance in a shaft of pale light from a crack above.",
                "A tapestry falls from the wall, revealing a hidden door.",
                "You hear the distant clang of armor—but no one is there.",
                "The smell of ancient stone and faded glory fills the air.",
                "Broken crown fragments litter the floor like fallen stars.",
                "A throne sits empty, its velvet rotted but its majesty intact.",
            ],
            FloorZone::SunkenArchives => vec![
                "Water drips from the ceiling onto moldering tomes.",
                "A fish swims past, somehow living in the stagnant water.",
                "Waterlogged scrolls float by, their ink bleeding into nothing.",
                "The water glows faintly where magic once preserved these halls.",
                "You hear whispers from submerged books—or is it the current?",
                "A skeleton sits at a reading desk, book still in bony hands.",
            ],
            FloorZone::BlightedGardens => vec![
                "A flower turns to track your movement.",
                "The scent of roses mixes with the stench of rot.",
                "Vines creep across the floor toward your feet.",
                "A statue of a beautiful woman stands overgrown—Malachar's beloved.",
                "The hedges form a maze that seems to shift when unobserved.",
                "Something beautiful blooms. It has far too many teeth.",
            ],
            FloorZone::ClockworkDepths => vec![
                "Gears click in precise, inhuman rhythm.",
                "Steam vents from a pipe, briefly obscuring your vision.",
                "A mechanical guardian stands dormant—for now.",
                "Blueprints on a workbench show impossible devices.",
                "The ticking never stops. Something is counting down.",
                "A brass eye tracks your movement from the wall.",
            ],
            FloorZone::VoidsEdge => vec![
                "Reality flickers like a guttering candle.",
                "You see your own shadow move independently.",
                "The walls bleed darkness that drips upward.",
                "Your own voice echoes back, saying things you didn't say.",
                "The floor beneath you shows stars—endless, hungry stars.",
                "Something vast moves at the edge of perception.",
            ],
            FloorZone::TheBreach => vec![
                "The Void whispers your name. Your true name.",
                "Time flows strangely here—was that a moment or an eon?",
                "You glimpse other worlds through tears in reality.",
                "The darkness here is not absence of light—it is presence of nothing.",
                "Malachar's voice echoes: 'You came. I knew you would.'",
                "At the heart of nothing, something waits.",
            ],
        }
    }
    
    pub fn zone_color(&self) -> &'static str {
        match self {
            FloorZone::ShatteredHalls => "Gray",
            FloorZone::SunkenArchives => "Cyan",
            FloorZone::BlightedGardens => "Green",
            FloorZone::ClockworkDepths => "Yellow",
            FloorZone::VoidsEdge => "Magenta",
            FloorZone::TheBreach => "Red",
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
        2 => Some(StoryMilestone {
            floor: 2,
            title: "The Ghost of the Guard".to_string(),
            description: "A spectral knight materializes from the shadows. His armor bears the crest of Valdris. 'Another hero come to fail,' he whispers. 'Or perhaps... to succeed where I could not.'".to_string(),
            event: MilestoneEvent::CharacterMeeting("Sir Aldric's Ghost".to_string()),
        }),
        5 => Some(StoryMilestone {
            floor: 5,
            title: "The Hollow Knight Awakens".to_string(),
            description: "The suit of royal armor stands. There is no body inside—only duty, binding it to this place forever. Its sword still carries the oath of protection.".to_string(),
            event: MilestoneEvent::StoryBoss("The Hollow Knight".to_string()),
        }),
        7 => Some(StoryMilestone {
            floor: 7,
            title: "The Mages Guild's Offer".to_string(),
            description: "A shimmering portal opens. Archmage Thessaly's voice echoes through: 'We know what you seek. We can help—for a price. The Guild remembers those who serve it.'".to_string(),
            event: MilestoneEvent::FactionEncounter(Faction::TempleOfDawn), // Reusing enum temporarily
        }),
        10 => Some(StoryMilestone {
            floor: 10,
            title: "The Void Herald Speaks".to_string(),
            description: "A figure of living shadow with too many eyes blocks your path. It speaks with the voices of everyone who ever fell into the Breach. 'We waited so long... we knew you would come.'".to_string(),
            event: MilestoneEvent::StoryBoss("The Void Herald".to_string()),
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
        FloorZone::ShatteredHalls => vec![
            GameEvent {
                name: "The Fallen Crown".to_string(),
                description: "A crown lies in the dust, its gems still glittering. It belonged to one of the kings of Valdris—but which one? And why is it here?".to_string(),
                choices: vec![
                    EventChoice { text: "Examine the crown carefully".to_string(), outcome: EventOutcome::GainXP(30) },
                    EventChoice { text: "Take the gems".to_string(), outcome: EventOutcome::GainGold(25) },
                    EventChoice { text: "Leave it as a memorial".to_string(), outcome: EventOutcome::GainMaxHP(3) },
                ],
                ascii_art: "  ╔═══════════╗\n  ║  👑      ║\n  ║  FALLEN  ║\n  ║  CROWN   ║\n  ╚═══════════╝".to_string(),
            },
            GameEvent {
                name: "The Knight's Ghost".to_string(),
                description: "A spectral warrior materializes from the shadows. They gesture toward a hidden alcove, then fade, pointing at something you cannot yet see.".to_string(),
                choices: vec![
                    EventChoice { text: "Follow their guidance".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Salute and honor them".to_string(), outcome: EventOutcome::GainXP(15) },
                    EventChoice { text: "Ignore the apparition".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "    ┌───┐\n    │ 👻│\n    │⚔️ │\n    └───┘".to_string(),
            },
        ],
        FloorZone::SunkenArchives => vec![
            GameEvent {
                name: "The Drowned Scholar".to_string(),
                description: "A skeleton sits at a submerged desk, still clutching a waterproof scroll. Whatever they died protecting, it survived the flood.".to_string(),
                choices: vec![
                    EventChoice { text: "Read the preserved scroll".to_string(), outcome: EventOutcome::GainXP(50) },
                    EventChoice { text: "Search their belongings".to_string(), outcome: EventOutcome::GainGold(40) },
                    EventChoice { text: "Give them a proper burial".to_string(), outcome: EventOutcome::GainMaxHP(5) },
                ],
                ascii_art: "  ╭──────────╮\n  │ 📄 LOST │\n  │ SCHOLAR │\n  ╰──────────╯".to_string(),
            },
            GameEvent {
                name: "The Glowing Waters".to_string(),
                description: "The water here pulses with residual magic. Drinking might empower you—or poison you. The magic is old and unpredictable.".to_string(),
                choices: vec![
                    EventChoice { text: "Drink deeply".to_string(), outcome: EventOutcome::GainMaxHP(8) },
                    EventChoice { text: "Bottle some for later".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Avoid the strange water".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "     💧\n    /  \\\n   ░▒▓█▓▒░\n    magic".to_string(),
            },
        ],
        FloorZone::BlightedGardens => vec![
            GameEvent {
                name: "The Hungry Roses".to_string(),
                description: "A patch of roses grows here, impossibly beautiful. They turn to face you as one. A skeleton lies among them, picked clean.".to_string(),
                choices: vec![
                    EventChoice { text: "Harvest the deadly blooms".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Burn the garden".to_string(), outcome: EventOutcome::Combat },
                    EventChoice { text: "Carefully skirt around them".to_string(), outcome: EventOutcome::Nothing },
                ],
                ascii_art: "  ┌─────────┐\n  │ 🌹  🌹  │\n  │ HUNGRY │\n  │ 🌹  🌹  │\n  └─────────┘".to_string(),
            },
            GameEvent {
                name: "The Statue of the Beloved".to_string(),
                description: "A marble statue of a beautiful woman stands overgrown. A plaque reads: 'For my love, who taught me that some things are worth any price.' —Malachar".to_string(),
                choices: vec![
                    EventChoice { text: "Pay respects".to_string(), outcome: EventOutcome::GainXP(35) },
                    EventChoice { text: "Search for hidden compartments".to_string(), outcome: EventOutcome::GainGold(45) },
                    EventChoice { text: "Rest in the statue's shade".to_string(), outcome: EventOutcome::GainHP(20) },
                ],
                ascii_art: "  ╔══════════╗\n  ║ 🧝‍♀️ STATUE ║\n  ║ BELOVED  ║\n  ╚══════════╝".to_string(),
            },
        ],
        FloorZone::ClockworkDepths => vec![
            GameEvent {
                name: "The Dormant Guardian".to_string(),
                description: "A massive automaton stands motionless, covered in dust. Its eyes are dark. A control panel nearby flickers with dying power.".to_string(),
                choices: vec![
                    EventChoice { text: "Try to reactivate it".to_string(), outcome: EventOutcome::Combat },
                    EventChoice { text: "Salvage its parts".to_string(), outcome: EventOutcome::GainGold(60) },
                    EventChoice { text: "Leave it in peace".to_string(), outcome: EventOutcome::GainXP(30) },
                ],
                ascii_art: "  🤖────────🤖\n  │ DORMANT │\n  │ GUARD   │\n  │  ↓↓↓↓   │\n  🤖────────🤖".to_string(),
            },
            GameEvent {
                name: "The Artificer's Workshop".to_string(),
                description: "Malachar's personal workshop. Blueprints for impossible devices cover every surface. One set of plans is for 'The Ritual Apparatus.'".to_string(),
                choices: vec![
                    EventChoice { text: "Study the blueprints".to_string(), outcome: EventOutcome::GainXP(50) },
                    EventChoice { text: "Take valuable components".to_string(), outcome: EventOutcome::GainGold(40) },
                    EventChoice { text: "Sabotage what remains".to_string(), outcome: EventOutcome::GainMaxHP(5) },
                ],
                ascii_art: "    ╭━━━╮\n    ┃ ? ┃\n    ╰┳━┳╯\n    PLANS\n   DEVICE".to_string(),
            },
        ],
        FloorZone::VoidsEdge => vec![
            GameEvent {
                name: "The Mirror of Truth".to_string(),
                description: "A mirror that should not exist here shows your reflection—but the reflection wears a crown and robes of state. It mouths words you cannot hear.".to_string(),
                choices: vec![
                    EventChoice { text: "Touch the mirror".to_string(), outcome: EventOutcome::GainXP(55) },
                    EventChoice { text: "Speak to your reflection".to_string(), outcome: EventOutcome::GainMaxHP(8) },
                    EventChoice { text: "Shatter it".to_string(), outcome: EventOutcome::Combat },
                ],
                ascii_art: "  ░░░░░░░░░░\n  ░ MIRROR ░\n  ░  SELF  ░\n  ░░░░░░░░░░".to_string(),
            },
            GameEvent {
                name: "The Last Letter".to_string(),
                description: "A letter preserved in crystal, addressed to 'My Future Self.' The handwriting is familiar—impossibly familiar. It is yours.".to_string(),
                choices: vec![
                    EventChoice { text: "Read the letter".to_string(), outcome: EventOutcome::GainXP(70) },
                    EventChoice { text: "Take it unopened".to_string(), outcome: EventOutcome::GainItem },
                    EventChoice { text: "Leave it for your future self".to_string(), outcome: EventOutcome::GainMaxHP(6) },
                ],
                ascii_art: "  ▓▓▓▓▓▓▓▓▓▓\n  ▓ LETTER ▓\n  ▓  FROM  ▓\n  ▓▓▓▓▓▓▓▓▓▓".to_string(),
            },
        ],
        FloorZone::TheBreach => vec![
            GameEvent {
                name: "The Voice of the Void".to_string(),
                description: "The Void itself speaks: 'You opened this door. You can close it. But the price is everything you are—and everything you were.'".to_string(),
                choices: vec![
                    EventChoice { text: "Listen to what it offers".to_string(), outcome: EventOutcome::GainXP(100) },
                    EventChoice { text: "Defy the Void".to_string(), outcome: EventOutcome::GainMaxHP(10) },
                    EventChoice { text: "Accept your role".to_string(), outcome: EventOutcome::GainHP(50) },
                ],
                ascii_art: "    🔥🔥🔥\n   🔥VOID🔥\n    🔥🔥🔥\n   SPEAKS".to_string(),
            },
            GameEvent {
                name: "Malachar's Memory".to_string(),
                description: "A vision: Malachar before the ritual, speaking to someone who looks exactly like you. 'Remember,' he says. 'When the time comes—remember why we did this.'".to_string(),
                choices: vec![
                    EventChoice { text: "Try to remember".to_string(), outcome: EventOutcome::GainXP(150) },
                    EventChoice { text: "Reject the memory".to_string(), outcome: EventOutcome::GainMaxHP(15) },
                    EventChoice { text: "Embrace who you were".to_string(), outcome: EventOutcome::GainHP(50) },
                ],
                ascii_art: "    👻\n   /||\\\n  MEMORY\n  returns".to_string(),
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
        FloorZone::ShatteredHalls => vec![
            ("Royal Chronicle Fragment", "Entry reads: 'The Sundering came without warning. One moment the king held court; the next, the sky tore open and darkness poured through. We fled, but the king... the king walked toward it.'"),
            ("Knight's Final Letter", "Found tucked in armor: 'My love, if you find this, know I stayed to the end. Sir Aldric commands us to hold the throne room. We will not abandon our king, even now.'"),
            ("Faded Tapestry", "A floor plan of the palace. Several sections are marked with blood. A note says: 'DO NOT ENTER - The Void has claimed these halls'"),
        ],
        FloorZone::SunkenArchives => vec![
            ("Waterlogged Journal", "Most text is illegible, but one phrase survives: 'Malachar studied here for decades. We should have seen the signs. His obsession with the Elder Stones consumed him.'"),
            ("Researcher's Final Entry", "'I've found it—Malachar's original notes on the Ritual of Ascension. Gods forgive me, he wasn't trying to destroy the world. He was trying to save it. But the price—'"),
            ("Preserved Scroll", "'The five Elder Stones contain the power to pierce the Veil. Gather them, and one might walk between worlds. But what walks back may not be what left.'"),
        ],
        FloorZone::BlightedGardens => vec![
            ("Love Letter", "'My dearest Malachar, I fear your experiments. Promise me you will not go too far. Promise me you will come back to our garden. —Forever yours'"),
            ("Gardener's Note", "'The roses have changed. They track movement now. I saw one bloom open and close around a bird. The Blight spreads even here, in her memory.'"),
            ("Withered Bouquet", "Dried flowers wrapped in silk, with a note: 'From our first meeting in this garden. I will find a way to bring you back. I promise. —M'"),
        ],
        FloorZone::ClockworkDepths => vec![
            ("Artificer's Blueprint", "'The Ritual Apparatus, Mark VII. This iteration should channel the Elder Stones without catastrophic feedback. Previous six prototypes resulted in dimensional instability.'"),
            ("Maintenance Log", "'Guardian Unit 7 has begun asking questions. Why are we here? What happened to the master? I told it Malachar would return. It asked: Which version of him?'"),
            ("Warning Placard", "'DANGER: Reality anchors unstable beyond this point. Time may not flow correctly. If you see yourself, DO NOT INTERACT.'"),
        ],
        FloorZone::VoidsEdge => vec![
            ("Malachar's Journal", "'I can feel the Veil thinning. So close now. The Stones resonate with something beyond. It calls to me. It knows my name. It says it can give her back.'"),
            ("Survivor's Account", "'I saw him at the moment of Ascension. He reached for the gods—and something reached back. The look on his face... it wasn't triumph. It was horror.'"),
            ("Void-Touched Note", "Text that shifts when you look away: 'YOU CAME BACK. YOU ALWAYS COME BACK. HOW MANY TIMES WILL YOU TRY? HOW MANY TIMES WILL YOU FAIL?'"),
        ],
        FloorZone::TheBreach => vec![
            ("The Final Truth", "'I am Malachar. I was Malachar. I will be Malachar. The cycle turns. The Breach remembers. And you—you are me, trying again.'"),
            ("Beyond the Veil", "'There is no death here. No life. Only the choice: seal the wound and end yourself forever, or embrace what you became and rule the nothing.'"),
            ("The Dreamer Stirs", "'Before the Void, before the gods, something dreamed the world. It sleeps still. The Breach is its opening eye. What will it see when it wakes?'"),
        ],
    };
    
    lore_pieces.choose(&mut rng).map(|(title, content)| {
        (title.to_string(), content.to_string())
    })
}
