//! Deep Lore System - The mythology, history, and secrets of the world
//!
//! This module establishes the foundational truth of the universe.
//! All other systems reference this as the source of coherent worldbuilding.
//!
//! Inspiration: Tolkien, Elder Scrolls, D&D Forgotten Realms, Dark Souls,
//! Earthbound's cosmic horror, Fallout's mysterious past.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===========================================================================
// THE COSMOLOGY - What is true about this universe
// ===========================================================================

/// The fundamental truth of the world: The Veil is thinning.
/// An ancient evil stirs beneath the mountains. Heroes are needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cosmology {
    /// The three ages of the world
    pub ages: WorldAges,
    /// The nature of the Blight/Corruption
    pub corruption_truth: CorruptionTruth,
    /// The Sundering that started everything
    pub the_sundering: TheSundering,
    /// What the world was like before
    pub before_memory: BeforeMemory,
}

impl Default for Cosmology {
    fn default() -> Self {
        Self::canonical()
    }
}

impl Cosmology {
    /// The canonical, true cosmology (what actually happened)
    pub fn canonical() -> Self {
        Self {
            ages: WorldAges::canonical(),
            corruption_truth: CorruptionTruth::canonical(),
            the_sundering: TheSundering::canonical(),
            before_memory: BeforeMemory::canonical(),
        }
    }
}

/// The three ages of the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldAges {
    pub age_of_dawn: AgeOfDawn,
    pub age_of_crowns: AgeOfCrowns,
    pub age_of_shadow: AgeOfShadow,
}

impl WorldAges {
    pub fn canonical() -> Self {
        Self {
            age_of_dawn: AgeOfDawn {
                description: "When the gods walked among mortals. \
                    The First Flame burned eternal in the heart of the world. \
                    Dragons ruled the skies and spoke prophecy. \
                    Magic flowed freely, and death was but a passage.".to_string(),
                key_event: "The Forging of the Elder Stones gave form to chaos.".to_string(),
                duration: "Unknown - before mortal reckoning".to_string(),
                ended_by: "The gods withdrew beyond the Veil after the War of Heavens.".to_string(),
            },
            age_of_crowns: AgeOfCrowns {
                description: "Mortals inherited the world. Kingdoms rose and fell. \
                    The great empires built wonders that still stand in ruin. \
                    Magic was codified into schools. The Orders were founded. \
                    But hubris grew, and some sought to pierce the Veil.".to_string(),
                key_event: "The founding of the Eternal Kingdom of Valdris.".to_string(),
                duration: "Three thousand years of recorded history.".to_string(),
                ended_by: "The Sundering - when the Archon tried to become a god.".to_string(),
                great_works: vec![
                    "The Spire of Eternity - a tower touching the heavens".to_string(),
                    "The Binding Stones - seals holding the Void at bay".to_string(),
                    "The Chronicle Eternal - all knowledge preserved in crystal".to_string(),
                ],
            },
            age_of_shadow: AgeOfShadow {
                description: "The current age. The Blight spreads from the wound in reality. \
                    Monsters crawl from the depths. Ancient evils stir. \
                    The kingdoms are fractured, the Orders scattered. \
                    Heroes are needed more than ever.".to_string(),
                key_event: "The Sundering tore a hole between worlds.".to_string(),
                duration: "Forty-seven years since the Sundering.".to_string(),
                current_state: "The Blight spreads. The Veil thins. Time grows short.".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfDawn {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub ended_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfCrowns {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub ended_by: String,
    pub great_works: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfShadow {
    pub description: String,
    pub key_event: String,
    pub duration: String,
    pub current_state: String,
}

/// The true nature of the Blight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorruptionTruth {
    /// What the factions believe (different theories)
    pub faction_theories: HashMap<String, String>,
    /// What is actually true (revealed gradually)
    pub actual_nature: String,
    /// The terrible secret
    pub hidden_truth: HiddenTruth,
}

impl CorruptionTruth {
    pub fn canonical() -> Self {
        let mut theories = HashMap::new();
        
        theories.insert("Mages Guild".to_string(), 
            "The Blight is raw chaos leaking from the Void. \
            Only through mastery of the arcane arts can we seal the breach.".to_string());
        
        theories.insert("Temple of Dawn".to_string(),
            "The Blight is divine punishment for the Archon's hubris. \
            Only through prayer and penance can we be forgiven.".to_string());
        
        theories.insert("Rangers of the Wild".to_string(),
            "The Blight is a sickness in the natural order. \
            The land itself must be healed, root and branch.".to_string());
        
        theories.insert("Shadow Guild".to_string(),
            "The Blight is a weapon. Someone is controlling it. \
            Find the puppeteer, and you find the cure.".to_string());
        
        theories.insert("Merchant Consortium".to_string(),
            "The Blight is an opportunity. Where there is chaos, \
            there is profit. Let others worry about causes.".to_string());
        
        Self {
            faction_theories: theories,
            actual_nature: "The Blight is the Void itself - the nothing that existed \
                before creation. The Sundering didn't just tear reality; \
                it remembered that reality could end.".to_string(),
            hidden_truth: HiddenTruth {
                surface_appearance: "Monsters and corruption spreading from the Breach.".to_string(),
                deeper_truth: "The Void is not invading - it is being called. \
                    Someone wants this world to end.".to_string(),
                deepest_secret: "The Archon did not fail. He succeeded in becoming a god. \
                    But the god he became was the God of Endings.".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenTruth {
    pub surface_appearance: String,
    pub deeper_truth: String,
    pub deepest_secret: String,
}

/// The Sundering - the catastrophe that ended the Age of Crowns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSundering {
    pub what_happened: String,
    pub who_caused_it: String,
    pub immediate_effects: Vec<String>,
    pub ongoing_consequences: Vec<String>,
    pub what_was_lost: Vec<String>,
    pub the_hidden_truth: String,
}

impl TheSundering {
    pub fn canonical() -> Self {
        Self {
            what_happened: "The Archon Malachar gathered the five Elder Stones \
                and attempted the Ritual of Ascension. He sought to pierce the Veil \
                and claim the power of the gods. The ritual succeeded—and failed. \
                The Veil tore. The Void poured through. Malachar vanished. \
                The world has been dying ever since.".to_string(),
            who_caused_it: "Archon Malachar, Last King of Valdris, Seeker of Divinity".to_string(),
            immediate_effects: vec![
                "The Breach opened beneath the Spire of Eternity".to_string(),
                "The Elder Stones shattered and scattered".to_string(),
                "The great wards failed across all kingdoms".to_string(),
                "Monsters poured forth from the depths".to_string(),
                "The gods fell silent".to_string(),
            ],
            ongoing_consequences: vec![
                "The Blight spreads further each year".to_string(),
                "Magic grows unstable near the Breach".to_string(),
                "The dead sometimes refuse to stay dead".to_string(),
                "Dreams are invaded by whispers from the Void".to_string(),
                "Ancient evils sealed away are breaking free".to_string(),
            ],
            what_was_lost: vec![
                "The Eternal Kingdom of Valdris".to_string(),
                "The accumulated wisdom of the Imperial Archives".to_string(),
                "The binding oaths between mortal and divine".to_string(),
                "Trust between the surviving kingdoms".to_string(),
                "Hope, for many".to_string(),
            ],
            the_hidden_truth: "Malachar did not die. He did not fail. \
                He became something else—something between god and void. \
                He watches. He waits. He calls his servants home.".to_string(),
        }
    }
}

/// What existed before recorded history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeMemory {
    pub fragments: Vec<AncientFragment>,
    pub the_truth_beneath: String,
}

impl BeforeMemory {
    pub fn canonical() -> Self {
        Self {
            fragments: vec![
                AncientFragment {
                    source: "The Prophecies of the First Seer".to_string(),
                    content: "Before the world was the Void. Before the Void was the Dream. \
                        The Dreamer sleeps beneath the mountain. Do not wake the Dreamer.".to_string(),
                    reliability: "Ancient but fragmentary".to_string(),
                },
                AncientFragment {
                    source: "Dwarven runes from the Deep Roads".to_string(),
                    content: "We delved too deep. We found the darkness that thinks. \
                        We sealed the doors and spoke not of what we saw.".to_string(),
                    reliability: "Dwarves do not lie about stone".to_string(),
                },
                AncientFragment {
                    source: "Elven creation songs".to_string(),
                    content: "The world is a song. We are but verses. \
                        When the song ends, the silence will swallow all.".to_string(),
                    reliability: "Elves remember, but speak in riddles".to_string(),
                },
                AncientFragment {
                    source: "Dragon-speech, translated by madmen".to_string(),
                    content: "We were here before the gods. We will be here after. \
                        The cycle turns. All burns. All returns.".to_string(),
                    reliability: "Dragons speak truth, but not for mortals to understand".to_string(),
                },
            ],
            the_truth_beneath: "The world is not the first world. It is not the last. \
                The Void consumes, but also preserves. Everything that has ever existed \
                still exists somewhere in the nothing. The Dreamer dreams of them all.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncientFragment {
    pub source: String,
    pub content: String,
    pub reliability: String,
}

// ===========================================================================
// THE FACTIONS - Who shapes the world
// ===========================================================================

/// The five major factions struggling for influence in the Age of Shadow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionLore {
    pub name: String,
    pub symbol: String,
    pub philosophy: String,
    pub history: String,
    pub current_state: String,
    pub goals: Vec<String>,
    pub methods: Vec<String>,
    pub secrets: Vec<String>,
    pub notable_members: Vec<NotableFigure>,
    pub relationship_to_player: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotableFigure {
    pub name: String,
    pub title: String,
    pub description: String,
    pub secret: String,
}

pub fn get_faction_lore() -> Vec<FactionLore> {
    vec![
        FactionLore {
            name: "The Mages Guild".to_string(),
            symbol: "󰊠".to_string(),
            philosophy: "Knowledge is power. Power must be controlled. \
                Through understanding of the arcane, we shall seal the Breach \
                and restore order to the world.".to_string(),
            history: "Founded in the Age of Crowns to regulate magic after \
                the Wild Magic Wars nearly destroyed the continent.".to_string(),
            current_state: "Fractured between those who would use any means \
                to stop the Blight and those who fear becoming what they fight.".to_string(),
            goals: vec![
                "Seal the Breach permanently".to_string(),
                "Preserve magical knowledge".to_string(),
                "Prevent another Sundering".to_string(),
            ],
            methods: vec![
                "Arcane research and experimentation".to_string(),
                "Training battlemages to fight the Blight".to_string(),
                "Seeking the scattered Elder Stone fragments".to_string(),
            ],
            secrets: vec![
                "The Archmage knows how to close the Breach but fears the cost".to_string(),
                "They have been experimenting with Void magic in secret".to_string(),
            ],
            notable_members: vec![
                NotableFigure {
                    name: "Archmage Thessaly".to_string(),
                    title: "Voice of the Council".to_string(),
                    description: "Ancient, wise, and burdened by terrible knowledge.".to_string(),
                    secret: "She was Malachar's apprentice. She helped him gather the Stones.".to_string(),
                },
            ],
            relationship_to_player: "Sees potential. Watches carefully.".to_string(),
        },
        FactionLore {
            name: "Temple of Dawn".to_string(),
            symbol: "󰖙".to_string(),
            philosophy: "Faith endures when all else fails. The gods have not abandoned us; \
                we abandoned them. Through devotion, we shall earn their return.".to_string(),
            history: "The oldest religious institution, dating back to the Age of Dawn \
                when priests spoke directly with the divine.".to_string(),
            current_state: "Growing in influence as people seek comfort. \
                But the silence of the gods weighs heavily on the faithful.".to_string(),
            goals: vec![
                "Restore contact with the gods".to_string(),
                "Provide sanctuary for refugees".to_string(),
                "Purify the Blighted lands through holy rites".to_string(),
            ],
            methods: vec![
                "Prayer, ritual, and pilgrimage".to_string(),
                "Healing the sick and wounded".to_string(),
                "Holy crusades against undead and demons".to_string(),
            ],
            secrets: vec![
                "Some priests have heard whispers—but not from the gods".to_string(),
                "The High Priest knows the gods are not silent. They are afraid.".to_string(),
            ],
            notable_members: vec![
                NotableFigure {
                    name: "High Priest Aldric".to_string(),
                    title: "Keeper of the Eternal Flame".to_string(),
                    description: "A man whose faith has been tested to breaking.".to_string(),
                    secret: "He no longer believes. He continues for the hope of others.".to_string(),
                },
            ],
            relationship_to_player: "Sees a lost soul in need of guidance.".to_string(),
        },
        FactionLore {
            name: "Rangers of the Wild".to_string(),
            symbol: "󰌪".to_string(),
            philosophy: "Civilization's hubris caused the Sundering. \
                The answer lies not in books or temples but in the living world.".to_string(),
            history: "Formed from survivors of the frontier kingdoms destroyed in the Sundering. \
                They know the wilds better than any map.".to_string(),
            current_state: "Stretched thin protecting trade routes and villages \
                from monsters. Respected but underfunded.".to_string(),
            goals: vec![
                "Protect the innocent from monsters".to_string(),
                "Find natural remedies for the Blight".to_string(),
                "Preserve the balance between civilization and wild".to_string(),
            ],
            methods: vec![
                "Scouting and monster hunting".to_string(),
                "Druidic nature magic".to_string(),
                "Alliance with beast and spirit".to_string(),
            ],
            secrets: vec![
                "They have found places where the Blight cannot spread".to_string(),
                "The elder druids can speak with something ancient in the deep forests".to_string(),
            ],
            notable_members: vec![
                NotableFigure {
                    name: "Warden Sylva".to_string(),
                    title: "Voice of the Wilds".to_string(),
                    description: "Half-elf who has walked every road and knows every trail.".to_string(),
                    secret: "She has seen what waits in the Void. She hunts it.".to_string(),
                },
            ],
            relationship_to_player: "Judges by actions, not words.".to_string(),
        },
        FactionLore {
            name: "Shadow Guild".to_string(),
            symbol: "󰘻".to_string(),
            philosophy: "Laws are chains for the weak. In darkness, we find freedom. \
                We take what we need and answer to no crown.".to_string(),
            history: "As old as civilization itself. Where there is wealth, there are thieves. \
                But they have codes, and they keep their word—for a price.".to_string(),
            current_state: "Thriving in the chaos. Information is more valuable than gold, \
                and they deal in both.".to_string(),
            goals: vec![
                "Profit from the chaos".to_string(),
                "Maintain their network of spies and contacts".to_string(),
                "Survive—whatever comes".to_string(),
            ],
            methods: vec![
                "Theft, smuggling, and assassination".to_string(),
                "Bribery and blackmail".to_string(),
                "Selling information to all sides".to_string(),
            ],
            secrets: vec![
                "They know who really controls the Blight".to_string(),
                "The Guildmaster has made deals with things from the Void".to_string(),
            ],
            notable_members: vec![
                NotableFigure {
                    name: "The Whisper".to_string(),
                    title: "Guildmaster".to_string(),
                    description: "No one knows their face. No one knows their voice.".to_string(),
                    secret: "There have been seven Whispers. The current one is not human.".to_string(),
                },
            ],
            relationship_to_player: "Everyone has a price. They want to know yours.".to_string(),
        },
        FactionLore {
            name: "Merchant Consortium".to_string(),
            symbol: "󰆼".to_string(),
            philosophy: "Gold makes the world turn, even when gods fall silent. \
                Trade routes must stay open. Civilization must continue.".to_string(),
            history: "United after the Sundering when old currencies collapsed. \
                They standardized trade and kept economies from total collapse.".to_string(),
            current_state: "Wealthy beyond measure. They fund armies and rebuild cities. \
                They also profit from war and scarcity.".to_string(),
            goals: vec![
                "Maintain profitable trade routes".to_string(),
                "Fund reconstruction of key cities".to_string(),
                "Ensure their influence over all factions".to_string(),
            ],
            methods: vec![
                "Investment and loans".to_string(),
                "Monopolizing essential goods".to_string(),
                "Political manipulation through wealth".to_string(),
            ],
            secrets: vec![
                "They have been buying up Elder Stone fragments".to_string(),
                "The Consortium's inner circle serves an older power".to_string(),
            ],
            notable_members: vec![
                NotableFigure {
                    name: "Guildmaster Venn".to_string(),
                    title: "Lord of Coin".to_string(),
                    description: "A jolly merchant prince who remembers every debt.".to_string(),
                    secret: "He has been alive for three hundred years. He remembers Malachar.".to_string(),
                },
            ],
            relationship_to_player: "A customer. A debtor. An opportunity.".to_string(),
        },
    ]
}

// ===========================================================================
// THE MYSTERY - The player's hidden past
// ===========================================================================

/// What the player gradually discovers about themselves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMystery {
    pub clues_by_chapter: HashMap<i32, Vec<Clue>>,
    pub the_truth: PlayerTruth,
    pub possible_endings: Vec<Ending>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clue {
    pub id: String,
    pub description: String,
    pub how_found: String,
    pub what_it_suggests: String,
    pub who_knows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTruth {
    pub who_they_were: String,
    pub what_they_did: String,
    pub why_they_forgot: String,
    pub what_they_must_choose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ending {
    pub name: String,
    pub requirements: Vec<String>,
    pub description: String,
    pub consequences: String,
}

pub fn create_player_mystery() -> PlayerMystery {
    let mut clues = HashMap::new();
    
    // Chapter 1 clues - something is wrong
    clues.insert(1, vec![
        Clue {
            id: "amnesia".to_string(),
            description: "You awaken with no memory of who you are or how you came to be here. \
                Only a burning drive to descend.".to_string(),
            how_found: "Game start".to_string(),
            what_it_suggests: "Memory loss this complete is not natural.".to_string(),
            who_knows: vec!["Perhaps no one".to_string()],
        },
        Clue {
            id: "strange_recognition".to_string(),
            description: "Enemies sometimes hesitate before attacking you. \
                'It can't be,' one whispers. 'You died.'".to_string(),
            how_found: "Random combat dialogue".to_string(),
            what_it_suggests: "You are known. You should not be alive.".to_string(),
            who_knows: vec!["The monsters remember".to_string()],
        },
    ]);
    
    // Chapter 2 clues - others know something
    clues.insert(2, vec![
        Clue {
            id: "mages_guild_file".to_string(),
            description: "The Archmage has a sealed file with your face on it. \
                It is marked 'DO NOT ENGAGE'.".to_string(),
            how_found: "Mages Guild reputation".to_string(),
            what_it_suggests: "You were important. You were dangerous.".to_string(),
            who_knows: vec!["The Mages Guild inner council".to_string()],
        },
        Clue {
            id: "dreams_of_fire".to_string(),
            description: "You dream of a great tower burning. You dream of a ritual. \
                You dream of reaching for something beyond the stars.".to_string(),
            how_found: "Rest events".to_string(),
            what_it_suggests: "You were there. At the Sundering.".to_string(),
            who_knows: vec!["The Temple seers have seen your dreams".to_string()],
        },
    ]);
    
    // Chapter 3 clues - identity narrowing
    clues.insert(3, vec![
        Clue {
            id: "malachar_portrait".to_string(),
            description: "You find a portrait of the Archon Malachar. \
                Your blood runs cold. He has your face.".to_string(),
            how_found: "Deep dungeon exploration".to_string(),
            what_it_suggests: "Impossible. Malachar died forty-seven years ago.".to_string(),
            who_knows: vec!["The Shadow Guild has been watching you".to_string()],
        },
        Clue {
            id: "elder_stone_resonance".to_string(),
            description: "When you touch an Elder Stone fragment, it sings. \
                It knows you. It welcomes you home.".to_string(),
            how_found: "Finding Stone fragments".to_string(),
            what_it_suggests: "You wielded them before.".to_string(),
            who_knows: vec!["The Stones themselves".to_string()],
        },
    ]);
    
    // Chapter 4 clues - confronting the truth
    clues.insert(4, vec![
        Clue {
            id: "void_recognition".to_string(),
            description: "A voice from the Breach speaks: 'Why do you fight yourself? \
                You opened this door. Come home.'".to_string(),
            how_found: "Approaching the final dungeon".to_string(),
            what_it_suggests: "You are connected to the Void.".to_string(),
            who_knows: vec!["All the factions suspect by now".to_string()],
        },
        Clue {
            id: "the_journal".to_string(),
            description: "You find a journal in your own handwriting. It details the ritual. \
                'Forgive me,' the final entry reads. 'I will make this right. \
                I will become what is needed, even if I must forget myself.'".to_string(),
            how_found: "Shadow Guild questline".to_string(),
            what_it_suggests: "You are Malachar. You chose to forget.".to_string(),
            who_knows: vec!["Now you know".to_string()],
        },
    ]);
    
    // Chapter 5 clue - acceptance
    clues.insert(5, vec![
        Clue {
            id: "memory_return".to_string(),
            description: "At the threshold of the Breach, your memories return. \
                You were Malachar. You sought to save your dying world. \
                You failed. The guilt broke you. You erased yourself and began again. \
                How many times have you descended? How many times must you try?".to_string(),
            how_found: "Reaching the final boss".to_string(),
            what_it_suggests: "The truth.".to_string(),
            who_knows: vec!["Everyone".to_string()],
        },
    ]);
    
    PlayerMystery {
        clues_by_chapter: clues,
        the_truth: PlayerTruth {
            who_they_were: "Malachar, the Archon—greatest mage in history, whose ambition \
                shattered the world.".to_string(),
            what_they_did: "Attempted to become a god to save his people from plague. \
                The ritual tore reality. He could not bear what he had become.".to_string(),
            why_they_forgot: "You chose to forget. You sealed your memories and cast yourself \
                into the mortal world, hoping to find redemption through ignorance. \
                But the Breach calls to you. It always does.".to_string(),
            what_they_must_choose: "Close the Breach and die forever. \
                Embrace your godhood and rule the Void. \
                Or find a third path—neither mortal nor god, but something new.".to_string(),
        },
        possible_endings: vec![
            Ending {
                name: "The Final Rest".to_string(),
                requirements: vec!["Gather all five Elder Stone fragments".to_string(),
                                   "Sacrifice yourself to seal the Breach".to_string()],
                description: "You give what remains of your divine power to close the wound. \
                    The Breach seals. The Blight recedes. You die, truly and finally.".to_string(),
                consequences: "The world heals slowly. You are remembered as both villain and savior. \
                    Your name becomes a prayer and a curse.".to_string(),
            },
            Ending {
                name: "The Dark Ascension".to_string(),
                requirements: vec!["Embrace your connection to the Void".to_string(),
                                   "Absorb the power of the Breach".to_string()],
                description: "You remember. You accept. You become what you were becoming \
                    before you flinched. The God of Endings rises.".to_string(),
                consequences: "The world ends. Not in fire, but in silence. \
                    In the silence, there is peace. In the peace, there is nothing. \
                    Is that not what you wanted?".to_string(),
            },
            Ending {
                name: "The Third Path".to_string(),
                requirements: vec!["Unite all five factions".to_string(),
                                   "Find the Dreamer beneath the mountain".to_string(),
                                   "Wake them with your choice".to_string()],
                description: "You discover a truth older than gods: the world dreams itself. \
                    You choose not to close the Breach or join it, but to walk through. \
                    On the other side, you find not the Void, but the Dreamer. \
                    You have a conversation.".to_string(),
                consequences: "What happens next is between you and the Dreamer. \
                    The world changes. Whether for better or worse depends on what you said. \
                    But the Breach becomes a door. And doors can be walked through, both ways.".to_string(),
            },
        ],
    }
}

// ===========================================================================
// THE DUNGEON - What lies beneath
// ===========================================================================

/// The truth about why the dungeon exists and what it contains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonLore {
    pub name: String,
    pub true_nature: String,
    pub zones: Vec<ZoneLore>,
    pub bosses: Vec<BossLore>,
    pub the_bottom: BottomLore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneLore {
    pub zone_name: String,
    pub floors: (i32, i32),
    pub surface_appearance: String,
    pub true_history: String,
    pub ambient_horror: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossLore {
    pub name: String,
    pub title: String,
    pub floor: i32,
    pub appearance: String,
    pub true_identity: String,
    pub last_words: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomLore {
    pub what_player_expects: String,
    pub what_is_actually_there: String,
    pub the_final_choice: String,
}

pub fn get_dungeon_lore() -> DungeonLore {
    DungeonLore {
        name: "The Spire of Eternity".to_string(),
        true_nature: "The Spire was built by Malachar to reach the heavens. \
            Now it descends into the earth—inverted by the Sundering. \
            What was the pinnacle is now the pit. The Breach waits at the bottom.".to_string(),
        zones: vec![
            ZoneLore {
                zone_name: "The Shattered Halls".to_string(),
                floors: (1, 2),
                surface_appearance: "Ruined corridors of what was once a grand palace. \
                    Debris and cobwebs. The occasional scavenger.".to_string(),
                true_history: "These were the throne rooms of Valdris. \
                    Here kings held court and heroes were honored.".to_string(),
                ambient_horror: "Whispers of old courtiers. The echo of music that stopped mid-note.".to_string(),
            },
            ZoneLore {
                zone_name: "The Sunken Archives".to_string(),
                floors: (3, 4),
                surface_appearance: "Flooded libraries full of rotting books. \
                    Knowledge lost to water and time.".to_string(),
                true_history: "The greatest collection of magical knowledge ever assembled. \
                    Malachar studied here for decades.".to_string(),
                ambient_horror: "Books that whisper when opened. Knowledge that hurts to know.".to_string(),
            },
            ZoneLore {
                zone_name: "The Blighted Gardens".to_string(),
                floors: (5, 6),
                surface_appearance: "Once-beautiful gardens now twisted by the Blight. \
                    Plants that move. Flowers with teeth.".to_string(),
                true_history: "The royal gardens, where Malachar's beloved walked. \
                    He tried to preserve her memory here.".to_string(),
                ambient_horror: "The scent of roses mixed with rot. Something beautiful that will kill you.".to_string(),
            },
            ZoneLore {
                zone_name: "The Clockwork Depths".to_string(),
                floors: (7, 8),
                surface_appearance: "Ancient mechanisms still grinding. \
                    Traps and automatons guarding forgotten vaults.".to_string(),
                true_history: "Malachar's workshops, where he built the apparatus for the ritual. \
                    Some machines still wait for commands.".to_string(),
                ambient_horror: "The ticking never stops. Something is counting down.".to_string(),
            },
            ZoneLore {
                zone_name: "The Void's Edge".to_string(),
                floors: (9, 10),
                surface_appearance: "Reality breaks down. Geometry fails. \
                    The Breach looms ahead, bleeding darkness.".to_string(),
                true_history: "The ritual chamber. Ground zero of the Sundering. \
                    This is where the world broke.".to_string(),
                ambient_horror: "Your own voice, from somewhere else, begging you to stop. Or to continue.".to_string(),
            },
        ],
        bosses: vec![
            BossLore {
                name: "The Hollow Knight".to_string(),
                title: "Guardian of the Fallen Throne".to_string(),
                floor: 5,
                appearance: "A suit of royal armor, walking without a body inside. \
                    Its sword still carries the oath of protection.".to_string(),
                true_identity: "Sir Aldric, captain of the royal guard. He refused to abandon his post \
                    even when his king became a monster. His loyalty bound him here.".to_string(),
                last_words: "My king... I failed you. I failed everyone. Rest now... please rest...".to_string(),
            },
            BossLore {
                name: "The Void Herald".to_string(),
                title: "Voice of the Breach".to_string(),
                floor: 10,
                appearance: "A figure of living shadow with too many eyes. \
                    It speaks with the voices of everyone who ever fell into the Breach.".to_string(),
                true_identity: "What remains of everyone Malachar sacrificed for the ritual—including \
                    his beloved. They are not angry. They are lonely.".to_string(),
                last_words: "We waited so long... we knew you would come... we forgive you... do you forgive yourself?".to_string(),
            },
        ],
        the_bottom: BottomLore {
            what_player_expects: "A final boss. A great evil to defeat. Closure.".to_string(),
            what_is_actually_there: "A mirror. The Breach. Your own reflection, waiting. \
                And a choice that cannot be taken back.".to_string(),
            the_final_choice: "You must choose: close the wound and die, \
                embrace the void and rule, or walk through and see what lies beyond. \
                There is no going back. There is no right answer. There is only you.".to_string(),
        },
    }
}

// ===========================================================================
// HELPER FUNCTIONS
// ===========================================================================

/// Get a random piece of deep lore appropriate for the player's progress
pub fn get_lore_hint(chapter: i32) -> Option<String> {
    let hints = match chapter {
        1 => vec![
            "The Sundering changed everything. Some say it was forty-seven years ago. Others insist it was yesterday.",
            "The gods fell silent after the Sundering. The priests say they are testing us. Others say they are dead.",
            "Before the Blight, the Spire of Eternity touched the sky. Now it reaches into the earth.",
        ],
        2 => vec![
            "The Archon Malachar sought to become a god. Some say he succeeded. Some say that was the problem.",
            "Five Elder Stones, five great powers. They shattered when the Veil tore. Some fragments remain.",
            "The factions all want something different. But they all fear the same thing.",
        ],
        3 => vec![
            "Have you noticed the monsters hesitate sometimes? As if they recognize you?",
            "The Mages Guild has files on everyone. Yours is sealed at the highest level.",
            "Your dreams are not just dreams. Someone is trying to tell you something.",
        ],
        4 => vec![
            "The Breach does not just consume. It remembers. Everything that falls in still exists, somehow.",
            "Malachar did not fail. That is the terrible truth.",
            "You have been here before. You will be here again.",
        ],
        5 => vec![
            "At the bottom, there is only you.",
            "The Dreamer sleeps. The Dreamer dreams. In the dream, you exist.",
            "Choose wisely. Or don't. The choice is the only thing that matters.",
        ],
        _ => vec!["The world is older than we know. It will outlast us all."],
    };
    
    use rand::seq::SliceRandom;
    hints.choose(&mut rand::thread_rng()).map(|s| s.to_string())
}

/// Get a cryptic inscription for environmental storytelling
pub fn get_inscription() -> String {
    let inscriptions = vec![
        "HERE FELL VALDRIS - MAY THE FLAMES REMEMBER",
        "THE KING SOUGHT THE SKY AND FOUND THE VOID",
        "WE WERE WARNED. WE DID NOT LISTEN.",
        "TURN BACK, STRANGER. THERE IS NOTHING HERE BUT ENDINGS.",
        "I CARVED THIS AS I DIED. REMEMBER ME.",
        "THE DREAMER STIRS. LET IT SLEEP.",
        "FIVE STONES. FIVE SEALS. ALL BROKEN NOW.",
        "HE DID NOT MEAN FOR THIS. FORGIVE HIM.",
    ];
    
    use rand::seq::SliceRandom;
    inscriptions.choose(&mut rand::thread_rng())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "The inscription is too worn to read.".to_string())
}

// ===========================================================================
// FACTION HISTORIES - Detailed backgrounds for each faction
// ===========================================================================

/// Complete history of a faction, including hidden agendas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionHistory {
    pub faction_name: String,
    pub founding_story: String,
    pub founder: HistoricalFigure,
    pub original_purpose: String,
    pub how_they_changed: String,
    pub current_leadership: String,
    pub public_agenda: String,
    pub hidden_agenda: String,
    pub internal_conflicts: Vec<String>,
    pub key_artifacts: Vec<Artifact>,
    pub relationship_to_corruption: String,
    pub what_they_know: Vec<String>,
    pub what_they_hide: Vec<String>,
}

/// A historical figure important to the lore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalFigure {
    pub name: String,
    pub title: String,
    pub era: String,
    pub legacy: String,
    pub dark_secret: Option<String>,
    pub connection_to_player: Option<String>,
}

/// An artifact with history and power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub description: String,
    pub origin_story: String,
    pub powers: Vec<String>,
    pub current_location: ArtifactLocation,
    pub who_wants_it: Vec<String>,
    pub hidden_truth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactLocation {
    Known(String),
    Rumored(String),
    Lost,
    HeldByPlayer,
    Destroyed,
    Corrupted,
}

/// Build complete faction histories
pub fn build_faction_histories() -> HashMap<String, FactionHistory> {
    let mut histories = HashMap::new();
    
    histories.insert("MagesGuild".to_string(), FactionHistory {
        faction_name: "The Mages Guild".to_string(),
        founding_story: "Founded during the Wild Magic Wars when unregulated sorcery \
            nearly tore the continent apart. The first Archmages created binding oaths \
            to prevent magical catastrophe. They failed to prevent the Sundering.".to_string(),
        founder: HistoricalFigure {
            name: "Archmage Valdris the Wise".to_string(),
            title: "The First Binder".to_string(),
            era: "Age of Crowns, Year 847".to_string(),
            legacy: "Created the Binding Oaths that all mages swear. Built the first \
                tower of the Guild. His statue still stands in the Grand Hall.".to_string(),
            dark_secret: Some("Valdris knew the binding oaths had a flaw. He never fixed it.".to_string()),
            connection_to_player: Some("Valdris was Malachar's great-grandfather. \
                The ambition runs in the bloodline.".to_string()),
        },
        original_purpose: "Regulate magic. Prevent catastrophe. Train mages safely.".to_string(),
        how_they_changed: "The Sundering proved their oaths insufficient. Now they seek \
            power at any cost—including studying the Void they swore to contain.".to_string(),
        current_leadership: "Archmage Thessaly leads the Council of Seven. She is \
            the only living person who knew Malachar personally.".to_string(),
        public_agenda: "Seal the Breach. Restore magical stability. Train new mages.".to_string(),
        hidden_agenda: "Thessaly believes the Breach can be controlled, not closed. \
            She wants to harness its power to remake the world.".to_string(),
        internal_conflicts: vec![
            "The Sealers want to close the Breach at any cost.".to_string(),
            "The Harvesters want to study and use Void magic.".to_string(),
            "Some apprentices have started hearing the Void's whispers.".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Staff of Binding".to_string(),
                description: "Valdris's original staff, used to create the binding oaths.".to_string(),
                origin_story: "Forged from a fallen star and bound with blood magic.".to_string(),
                powers: vec![
                    "Can enforce any oath spoken while holding it.".to_string(),
                    "Grants resistance to Void corruption.".to_string(),
                    "Burns those who break sworn promises.".to_string(),
                ],
                current_location: ArtifactLocation::Known("The Archmage's chamber.".to_string()),
                who_wants_it: vec!["Everyone who has broken an oath.".to_string()],
                hidden_truth: Some("The staff is cracked. It cannot fully bind anymore.".to_string()),
            },
        ],
        relationship_to_corruption: "They believe they can control it through study.".to_string(),
        what_they_know: vec![
            "Malachar was their most promising student before his fall.".to_string(),
            "The Breach responds to strong magical signatures.".to_string(),
            "Something is coming through from the other side.".to_string(),
        ],
        what_they_hide: vec![
            "Thessaly helped Malachar gather the Elder Stones.".to_string(),
            "Three Council members have been corrupted by Void exposure.".to_string(),
            "They know the player's true identity and are watching.".to_string(),
        ],
    });
    
    histories.insert("TempleOfDawn".to_string(), FactionHistory {
        faction_name: "Temple of Dawn".to_string(),
        founding_story: "In the Age of Dawn, when gods walked among mortals, the first \
            priests received divine instruction directly. They built the First Temple \
            where light first touched the world.".to_string(),
        founder: HistoricalFigure {
            name: "Saint Aurelia".to_string(),
            title: "The Dawn's First Light".to_string(),
            era: "Age of Dawn".to_string(),
            legacy: "Spoke directly with the gods. Wrote the Sacred Texts. \
                Her body never decayed and rests in the Temple's heart.".to_string(),
            dark_secret: Some("Aurelia's final prophecy was suppressed: 'The gods \
                will abandon you when you need them most.'".to_string()),
            connection_to_player: None,
        },
        original_purpose: "Serve as intermediaries between mortals and the divine.".to_string(),
        how_they_changed: "When the gods fell silent, they had to become the source of \
            hope themselves. Faith became performance. Many priests lost belief.".to_string(),
        current_leadership: "High Priest Aldric maintains appearances while secretly \
            searching for any way to restore divine contact.".to_string(),
        public_agenda: "Provide sanctuary. Heal the sick. Keep faith alive.".to_string(),
        hidden_agenda: "Aldric has been experimenting with forbidden rites to force \
            the gods to respond. Some have worked—but not as expected.".to_string(),
        internal_conflicts: vec![
            "The Orthodox insist the gods will return on their own.".to_string(),
            "The Seekers want to actively summon divine intervention.".to_string(),
            "Some priests have begun hearing voices—but not from the gods.".to_string(),
        ],
        key_artifacts: vec![
            Artifact {
                name: "The Eternal Flame".to_string(),
                description: "A fire that has burned since the Age of Dawn, lit by the gods themselves.".to_string(),
                origin_story: "The gods touched a candle and said 'Let this burn until we return.'".to_string(),
                powers: vec![
                    "Purifies Blight corruption in its presence.".to_string(),
                    "Those who gaze into it see glimpses of truth.".to_string(),
                    "Cannot be extinguished by any known means.".to_string(),
                ],
                current_location: ArtifactLocation::Known("The heart of the First Temple.".to_string()),
                who_wants_it: vec!["The Void—it is the last divine light.".to_string()],
                hidden_truth: Some("The flame has been flickering. Aldric hides this from everyone.".to_string()),
            },
        ],
        relationship_to_corruption: "Divine punishment for mortal hubris. Can be cleansed through faith.".to_string(),
        what_they_know: vec![
            "The gods did not abandon mortals—they fled from something.".to_string(),
            "Some prayers are being answered, but the voice is wrong.".to_string(),
            "The Eternal Flame reacts to the player's presence.".to_string(),
        ],
        what_they_hide: vec![
            "Aldric has lost his faith entirely.".to_string(),
            "Three priests who attempted the forbidden rites went mad.".to_string(),
            "Aurelia's suppressed prophecy specifically mentions the player.".to_string(),
        ],
    });
    
    histories
}
