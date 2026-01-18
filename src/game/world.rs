//! Location and World Map System

use serde::{Deserialize, Serialize};
use super::narrative::Faction;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub regions: HashMap<String, Region>,
    pub current_location: String,
    pub discovered_locations: Vec<String>,
    pub world_events: Vec<WorldEvent>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        let mut regions = HashMap::new();
        
        // The starting area
        regions.insert("haven".to_string(), Region::haven());
        regions.insert("athenaeum".to_string(), Region::athenaeum());
        regions.insert("mechanist_fortress".to_string(), Region::mechanist_fortress());
        regions.insert("shadow_quarter".to_string(), Region::shadow_quarter());
        regions.insert("sacred_grove".to_string(), Region::sacred_grove());
        regions.insert("corrupted_wastes".to_string(), Region::corrupted_wastes());
        regions.insert("first_library".to_string(), Region::first_library());
        
        Self {
            regions,
            current_location: "haven".to_string(),
            discovered_locations: vec!["haven".to_string()],
            world_events: Vec::new(),
        }
    }
    
    pub fn discover_location(&mut self, location_id: &str) -> bool {
        if !self.discovered_locations.contains(&location_id.to_string()) {
            self.discovered_locations.push(location_id.to_string());
            true
        } else {
            false
        }
    }
    
    pub fn travel_to(&mut self, location_id: &str) -> Result<&Region, &'static str> {
        if !self.discovered_locations.contains(&location_id.to_string()) {
            return Err("You haven't discovered this location yet.");
        }
        
        if let Some(region) = self.regions.get(location_id) {
            if !region.accessible {
                return Err("This location is currently inaccessible.");
            }
            self.current_location = location_id.to_string();
            Ok(region)
        } else {
            Err("Unknown location.")
        }
    }
    
    pub fn current_region(&self) -> Option<&Region> {
        self.regions.get(&self.current_location)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub description: String,
    pub detailed_description: String,
    pub ascii_art: String,
    pub faction_territory: Option<Faction>,
    pub danger_level: u32, // 1-10
    pub locations: Vec<Location>,
    pub connected_regions: Vec<String>,
    pub ambient_sentences: Vec<String>,
    pub accessible: bool,
    pub discovery_requirement: Option<DiscoveryRequirement>,
}

impl Region {
    pub fn haven() -> Self {
        Region {
            id: "haven".to_string(),
            name: "Haven".to_string(),
            description: "A sanctuary for typists, protected by ancient wards.".to_string(),
            detailed_description: "Haven exists in the space between corrupted territories, \
                a miracle of preservation in a crumbling world. The town is built around \
                the Last Functional Terminal—a machine from before the Unwriting that still \
                produces clean, uncorrupted text. Here, refugees from all factions gather, \
                bound by a fragile peace. The streets are lined with practice boards where \
                children learn their first words, and the air hums with the constant clatter \
                of keystrokes. Haven's protective wards require constant maintenance through \
                communal typing rituals performed at dawn and dusk.".to_string(),
            ascii_art: r#"
                    ___
                   /   \
              ____/     \____
             /               \
            /   H A V E N    \
           /                   \
          |   []  []  []  []   |
          |   ||  ||  ||  ||   |
          |___||__||__||__||___|
         /                      \
        /________________________\
       |     SANCTUARY OF WORDS   |
       |__________________________|
"#.to_string(),
            faction_territory: None,
            danger_level: 1,
            locations: vec![
                Location {
                    id: "haven_square".to_string(),
                    name: "Central Square".to_string(),
                    description: "The heart of Haven, where the Last Terminal stands.".to_string(),
                    npcs: vec!["town_elder".to_string()],
                    enemies: vec![],
                    items: vec![],
                    typing_challenge: None,
                },
                Location {
                    id: "haven_inn".to_string(),
                    name: "The Quiet Keys Inn".to_string(),
                    description: "A warm tavern where typists share stories and rest.".to_string(),
                    npcs: vec!["innkeeper_mira".to_string()],
                    enemies: vec![],
                    items: vec!["minor_healing".to_string()],
                    typing_challenge: None,
                },
                Location {
                    id: "haven_training".to_string(),
                    name: "Training Grounds".to_string(),
                    description: "Practice keyboards line the walls. Perfect your skills here.".to_string(),
                    npcs: vec!["trainer_beck".to_string()],
                    enemies: vec![],
                    items: vec![],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Complete the typing drill".to_string(),
                        sentences: vec![
                            "The quick brown fox jumps over the lazy dog.".to_string(),
                            "Pack my box with five dozen liquor jugs.".to_string(),
                            "How vexingly quick daft zebras jump.".to_string(),
                        ],
                        reward: ChallengeReward::Experience(50),
                        repeatable: true,
                    }),
                },
            ],
            connected_regions: vec![
                "athenaeum".to_string(),
                "corrupted_wastes".to_string(),
            ],
            ambient_sentences: vec![
                "The sound of typing echoes through Haven's peaceful streets.".to_string(),
                "Children practice their letters on chalk boards, dreaming of adventure.".to_string(),
                "The protective wards shimmer faintly in the morning light.".to_string(),
            ],
            accessible: true,
            discovery_requirement: None,
        }
    }
    
    pub fn athenaeum() -> Self {
        Region {
            id: "athenaeum".to_string(),
            name: "The Athenaeum".to_string(),
            description: "The great library of the Archivists, repository of all knowledge.".to_string(),
            detailed_description: "The Athenaeum defies physical law. Its exterior appears as \
                a modest stone building, but within lies an infinite labyrinth of shelves \
                stretching into dimensions that human minds struggle to comprehend. \
                The Archivists who tend this place have mapped only a fraction of its depths. \
                Books here are not merely read—they are experienced. Some contain memories, \
                others prophecies, and a rare few hold living ideas that have taken textual form. \
                The deeper one ventures, the stranger the texts become, until finally one reaches \
                the Restricted Section, where words too dangerous to speak are kept in silent vaults.".to_string(),
            ascii_art: r#"
              ___________________
             |  THE ATHENAEUM   |
             |___________________|
            /|   |BOOKS|BOOKS|  |\
           / |   |_____|_____|  | \
          /  |   |BOOKS|BOOKS|  |  \
         /   |   |_____|_____|  |   \
        /    |   |BOOKS|BOOKS|  |    \
       /     |   |_____|_____|  |     \
      /      |___________________|      \
     /       |     KNOWLEDGE     |       \
    /_________|___________________|_______\
"#.to_string(),
            faction_territory: Some(Faction::MerchantConsortium),
            danger_level: 3,
            locations: vec![
                Location {
                    id: "main_hall".to_string(),
                    name: "Grand Reading Hall".to_string(),
                    description: "Thousands of books line walls that stretch beyond sight.".to_string(),
                    npcs: vec!["archivist_vera".to_string()],
                    enemies: vec![],
                    items: vec!["lore_fragment".to_string()],
                    typing_challenge: None,
                },
                Location {
                    id: "restricted_section".to_string(),
                    name: "Restricted Section".to_string(),
                    description: "Sealed texts too dangerous for casual reading.".to_string(),
                    npcs: vec![],
                    enemies: vec!["guardian_construct".to_string()],
                    items: vec!["forbidden_text".to_string()],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Type the unsealing incantation perfectly".to_string(),
                        sentences: vec![
                            "By the first word spoken and the last word written, \
                             I seek knowledge that was sealed for wisdom's sake.".to_string(),
                        ],
                        reward: ChallengeReward::UnlockArea("inner_sanctum".to_string()),
                        repeatable: false,
                    }),
                },
            ],
            connected_regions: vec![
                "haven".to_string(),
                "sacred_grove".to_string(),
            ],
            ambient_sentences: vec![
                "Dust motes dance in shafts of light between towering shelves.".to_string(),
                "The rustle of turning pages creates a constant whisper.".to_string(),
                "Somewhere in the depths, a book screams—then falls silent.".to_string(),
            ],
            accessible: true,
            discovery_requirement: None,
        }
    }
    
    pub fn mechanist_fortress() -> Self {
        Region {
            id: "mechanist_fortress".to_string(),
            name: "The Velocity Citadel".to_string(),
            description: "The Mechanist stronghold, where speed is worshipped.".to_string(),
            detailed_description: "The Velocity Citadel is a monument to efficiency. \
                Every surface is covered in keyboards—walls, floors, even ceilings have \
                typing stations. The air thrums with the percussion of ten thousand fingers \
                striking keys in precise rhythm. Mechanists train here day and night, \
                pushing themselves toward inhuman speeds. Giant displays show real-time \
                WPM rankings, and those who fall below quota face harsh retraining. \
                At the citadel's heart lies the Overclocked Core, a machine that \
                the Mechanists believe will one day type so fast it will rewrite reality itself.".to_string(),
            ascii_art: r#"
           |VELOCITY CITADEL|
          /||||||||||||||||||\
         / |||||||||||||||||| \
        /  ||  SPEED IS  ||   \
       |   ||   LIFE     ||    |
       |   ||============||    |
       |   || WPM: 9999  ||    |
       |   ||============||    |
       |   ||||||||||||||||||  |
       |_______________________|
      /  [KEY][KEY][KEY][KEY]  \
     /_________________________\
"#.to_string(),
            faction_territory: Some(Faction::TempleOfDawn),
            danger_level: 5,
            locations: vec![
                Location {
                    id: "training_floor".to_string(),
                    name: "Speed Training Floor".to_string(),
                    description: "Rows of typing stations where Mechanists hone their speed.".to_string(),
                    npcs: vec!["commander_steele".to_string()],
                    enemies: vec![],
                    items: vec![],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Pass the Mechanist speed trial".to_string(),
                        sentences: vec![
                            "Speed is life. Hesitation is death. Type or be forgotten.".to_string(),
                        ],
                        reward: ChallengeReward::FactionReputation(Faction::TempleOfDawn, 20),
                        repeatable: true,
                    }),
                },
            ],
            connected_regions: vec![
                "corrupted_wastes".to_string(),
                "shadow_quarter".to_string(),
            ],
            ambient_sentences: vec![
                "The rhythm of a thousand keyboards creates a mechanical heartbeat.".to_string(),
                "Leaderboards flash with updated WPM scores every second.".to_string(),
                "A recruit cries out—they've achieved personal best: 150 WPM.".to_string(),
            ],
            accessible: false,
            discovery_requirement: Some(DiscoveryRequirement::QuestComplete("first_contact".to_string())),
        }
    }
    
    pub fn shadow_quarter() -> Self {
        Region {
            id: "shadow_quarter".to_string(),
            name: "The Shadow Quarter".to_string(),
            description: "Where the Shadow Writers conduct their secret business.".to_string(),
            detailed_description: "The Shadow Quarter exists in the margins of other places. \
                It's not on any map because it exists between maps, in the spaces where \
                words trail off into ellipses. Those who know how to look can find its \
                entrances—a door that shouldn't be there, a stairway leading down from \
                a ground floor, a shadow that doesn't match its source. Within, the \
                Shadow Writers trade in secrets, favors, and unwritten words. The economy \
                runs on information, and the most valuable currency is a secret no one else knows.".to_string(),
            ascii_art: r#"
              . . . . . . .
            .   ?   ?   ?   .
          .    _________    .
         .    |  ???    |    .
        .     |   ???   |     .
        .     |    ???  |     .
        .     |_________|     .
         .    /  ??? ???\    .
          .  /___________\  .
            . . . . . . . .
              SHADOW QUARTER
"#.to_string(),
            faction_territory: Some(Faction::ShadowGuild),
            danger_level: 6,
            locations: vec![
                Location {
                    id: "whisper_market".to_string(),
                    name: "Whisper Market".to_string(),
                    description: "Trade secrets and forbidden knowledge here.".to_string(),
                    npcs: vec!["shadow_whisper".to_string()],
                    enemies: vec![],
                    items: vec!["shadow_cloak".to_string()],
                    typing_challenge: None,
                },
            ],
            connected_regions: vec![
                "mechanist_fortress".to_string(),
                "corrupted_wastes".to_string(),
            ],
            ambient_sentences: vec![
                "Whispers echo from walls that shouldn't have ears.".to_string(),
                "A figure watches you from the corner of your eye—then isn't there.".to_string(),
                "Somewhere, a deal is struck. Somewhere else, a secret dies.".to_string(),
            ],
            accessible: false,
            discovery_requirement: Some(DiscoveryRequirement::FactionReputation(Faction::ShadowGuild, 20)),
        }
    }
    
    pub fn sacred_grove() -> Self {
        Region {
            id: "sacred_grove".to_string(),
            name: "The Sacred Grove".to_string(),
            description: "Where the Naturalists commune with the Green Word.".to_string(),
            detailed_description: "The Sacred Grove predates human language. Here, trees \
                grow in formations that spell out words in a script older than alphabets. \
                Streams babble in patterns that, to trained ears, form coherent sentences. \
                The Naturalists tend this place, translating between humanity's typed words \
                and nature's organic texts. They believe that all languages evolved from \
                the Green Word—the original language of life itself—and that by understanding \
                it, they can heal the rift between technology and nature that made the \
                Unwriting possible.".to_string(),
            ascii_art: r#"
           ,@@@@@@@@@,
          @@@@@@@@@@@@@@
         @@@@@@@@@@@@@@@@@
        @@@@ SACRED @@@@@@
         @@@@ GROVE @@@@@
          @@@@@@@@@@@@@
           @@@@@@@@@@@
            ,@@@@@@@,
             /|||||\
            / ||||| \
           /  |||||  \
          /___|||||___\
             GREEN
              WORD
"#.to_string(),
            faction_territory: Some(Faction::RangersOfTheWild),
            danger_level: 4,
            locations: vec![
                Location {
                    id: "heart_tree".to_string(),
                    name: "The Heart Tree".to_string(),
                    description: "An ancient tree that speaks in the Green Word.".to_string(),
                    npcs: vec!["elder_root".to_string()],
                    enemies: vec![],
                    items: vec!["nature_essence".to_string()],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Transcribe the Green Word".to_string(),
                        sentences: vec![
                            "As the river flows to the sea, as the seed grows to the tree, \
                             so too does the word flow through all things, connecting, creating, becoming.".to_string(),
                        ],
                        reward: ChallengeReward::SkillPoint,
                        repeatable: false,
                    }),
                },
            ],
            connected_regions: vec![
                "athenaeum".to_string(),
                "corrupted_wastes".to_string(),
            ],
            ambient_sentences: vec![
                "The trees sway though there is no wind, writing in the air.".to_string(),
                "A deer watches you with knowing eyes, then bounds away.".to_string(),
                "The very soil seems to hum with unspoken words.".to_string(),
            ],
            accessible: false,
            discovery_requirement: Some(DiscoveryRequirement::QuestComplete("voice_of_nature".to_string())),
        }
    }
    
    pub fn corrupted_wastes() -> Self {
        Region {
            id: "corrupted_wastes".to_string(),
            name: "The Corrupted Wastes".to_string(),
            description: "Land devoured by the Unwriting. Meaning itself has died here.".to_string(),
            detailed_description: "The Corrupted Wastes are what remains when the Unwriting \
                finishes its work. Here, words have no power. Signs display gibberish. \
                Speech comes out garbled. Even thoughts become confused and fragmentary. \
                The landscape is a nightmare of half-formed shapes—buildings that forgot \
                how to be buildings, trees that lost the concept of growth, creatures that \
                exist only as corrupted data. Typists who venture here must type constantly \
                to maintain their own coherence, for the Unwriting hungers for all meaning.".to_string(),
            ascii_art: r#"
        ????????????????????
       ?                    ?
      ?  THE CORRUPTED      ?
     ?    W A S T E S        ?
    ?                          ?
   ?  ▓▒░ MEANING LOST ░▒▓    ?
  ?                            ?
 ?    ▓▒░░▒▓  ????  ▓▒░░▒▓    ?
?                              ?
 ??????????????????????????
"#.to_string(),
            faction_territory: None,
            danger_level: 8,
            locations: vec![
                Location {
                    id: "waste_edge".to_string(),
                    name: "Edge of Meaning".to_string(),
                    description: "The boundary where coherent reality meets corruption.".to_string(),
                    npcs: vec![],
                    enemies: vec!["corrupted_typer".to_string(), "meaning_eater".to_string()],
                    items: vec!["corrupted_fragment".to_string()],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Type to maintain your coherence".to_string(),
                        sentences: vec![
                            "I am. I think. I type. These words are mine. I will not be unwritten.".to_string(),
                        ],
                        reward: ChallengeReward::SurvivalCheck,
                        repeatable: true,
                    }),
                },
            ],
            connected_regions: vec![
                "haven".to_string(),
                "mechanist_fortress".to_string(),
                "shadow_quarter".to_string(),
                "sacred_grove".to_string(),
                "first_library".to_string(),
            ],
            ambient_sentences: vec![
                "The air tastes like forgotten words.".to_string(),
                "Something that was once a road stretches nowhere.".to_string(),
                "Your thoughts feel slippery, hard to hold onto.".to_string(),
            ],
            accessible: true,
            discovery_requirement: None,
        }
    }
    
    pub fn first_library() -> Self {
        Region {
            id: "first_library".to_string(),
            name: "The First Library".to_string(),
            description: "Where the Word was born, and where it began to die.".to_string(),
            detailed_description: "The First Library is a myth. The First Library is real. \
                Both statements are true. It exists at the center of the Corrupted Wastes, \
                preserved by a paradox—the place where the Unwriting began is also the only \
                place where the original Word still survives. Here, in ruins that remember \
                being temples, the True Name was attempted. Here, meaning first shattered. \
                And here, perhaps, meaning can be restored—or destroyed forever. \
                No one who has entered has ever returned. But the stories say that within \
                lies the Terminal of Origin, where the first word was ever typed.".to_string(),
            ascii_art: r#"
            THE FIRST LIBRARY
           __________________
          |  THE ORIGIN OF  |
          |    ALL WORDS    |
          |__________________|
         /|  ▓▓▓▓▓▓▓▓▓▓▓▓  |\
        / |  ▓ TERMINAL ▓  | \
       /  |  ▓    OF    ▓  |  \
      /   |  ▓  ORIGIN  ▓  |   \
     /    |  ▓▓▓▓▓▓▓▓▓▓▓▓  |    \
    /     |__________________|     \
   /______________________________ \
"#.to_string(),
            faction_territory: None,
            danger_level: 10,
            locations: vec![
                Location {
                    id: "terminal_chamber".to_string(),
                    name: "Terminal of Origin".to_string(),
                    description: "Where the first word was typed, and the last may be.".to_string(),
                    npcs: vec![],
                    enemies: vec!["the_unwriter".to_string()],
                    items: vec!["true_name_fragment".to_string()],
                    typing_challenge: Some(LocationChallenge {
                        prompt: "Type the Word that will decide all fates".to_string(),
                        sentences: vec![
                            "In the beginning was the Word, and the Word was with all things, \
                             and the Word was all things. Now, at the ending, the Word must be \
                             spoken again—not to create, but to choose: preservation or oblivion.".to_string(),
                        ],
                        reward: ChallengeReward::FinalChoice,
                        repeatable: false,
                    }),
                },
            ],
            connected_regions: vec![
                "corrupted_wastes".to_string(),
            ],
            ambient_sentences: vec![
                "Reality itself seems to hold its breath.".to_string(),
                "The walls remember every word ever typed.".to_string(),
                "You feel the weight of all language pressing upon you.".to_string(),
            ],
            accessible: false,
            discovery_requirement: Some(DiscoveryRequirement::QuestComplete("the_final_chapter".to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub npcs: Vec<String>,
    pub enemies: Vec<String>,
    pub items: Vec<String>,
    pub typing_challenge: Option<LocationChallenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationChallenge {
    pub prompt: String,
    pub sentences: Vec<String>,
    pub reward: ChallengeReward,
    pub repeatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeReward {
    Experience(u32),
    Item(String),
    SkillPoint,
    FactionReputation(Faction, i32),
    UnlockArea(String),
    SurvivalCheck, // Must complete to survive
    FinalChoice,   // End-game decision
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryRequirement {
    QuestComplete(String),
    FactionReputation(Faction, i32),
    ItemPossessed(String),
    CharacterMet(String),
    LoreDiscovered(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub affected_regions: Vec<String>,
    pub active: bool,
    pub consequences: Vec<EventConsequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventConsequence {
    RegionDangerChange(String, i32),
    RegionAccessChange(String, bool),
    FactionRelationChange(Faction, Faction, i32),
    NPCStateChange(String, NPCChange),
    GlobalModifier(String, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCChange {
    LocationChange(String),
    DeathState(bool),
    DialogueUnlock(String),
    RelationshipChange(i32),
}
