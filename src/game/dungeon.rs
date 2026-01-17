//! Dungeon and floor generation - roguelike exploration!

use serde::{Deserialize, Serialize};
use rand::Rng;
use super::enemy::Enemy;
use super::items::Item;
use super::world_integration::{FloorZone, get_ambient_message, get_zone_entry_message, get_floor_lore};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub name: String,
    pub current_floor: i32,
    pub max_floor: i32,
    pub rooms_cleared: i32,
    pub rooms_per_floor: i32,
    pub current_room: Room,
    pub floor_complete: bool,
    /// Current zone based on floor depth
    pub zone_name: String,
    /// Pending zone entry message to display
    pub zone_message: Option<String>,
    /// Pending lore discovery
    pub pending_lore: Option<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub room_type: RoomType,
    pub cleared: bool,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomType {
    Combat,
    Elite,
    Boss,
    Treasure,
    Rest,
    Shop,
    Event,
    Start,
}

impl Dungeon {
    pub fn new() -> Self {
        let zone = FloorZone::from_floor(1);
        Self {
            name: "The Infinite Archives".to_string(),
            current_floor: 1,
            max_floor: 100, // Infinite-ish
            rooms_cleared: 0,
            rooms_per_floor: 4,
            current_room: Room {
                room_type: RoomType::Start,
                cleared: true,
                description: format!(
                    "You stand at the entrance of {}...\n\n{}",
                    zone.name(),
                    zone.description()
                ),
            },
            floor_complete: false,
            zone_name: zone.name().to_string(),
            zone_message: None,
            pending_lore: None,
        }
    }

    pub fn generate_next_room(&mut self) -> Room {
        let mut rng = rand::thread_rng();
        
        // Check for boss room
        if self.rooms_cleared >= self.rooms_per_floor - 1 && self.current_floor % 5 == 0 {
            return Room {
                room_type: RoomType::Boss,
                cleared: false,
                description: self.get_boss_room_description(),
            };
        }
        
        // Check for floor complete
        if self.rooms_cleared >= self.rooms_per_floor {
            self.floor_complete = true;
            let zone = FloorZone::from_floor(self.current_floor as u32);
            return Room {
                room_type: RoomType::Rest,
                cleared: false,
                description: format!(
                    "A stairway leads deeper into the {}...\n\n{}",
                    zone.name(),
                    get_ambient_message(self.current_floor as u32)
                ),
            };
        }
        
        // Check for lore discovery (15% chance per room)
        self.pending_lore = get_floor_lore(self.current_floor as u32);
        
        // Random room type
        let roll: f32 = rng.gen();
        let room_type = if roll < 0.50 {
            RoomType::Combat
        } else if roll < 0.65 {
            RoomType::Event
        } else if roll < 0.75 {
            RoomType::Treasure
        } else if roll < 0.85 {
            RoomType::Rest
        } else if roll < 0.92 {
            RoomType::Shop
        } else {
            RoomType::Elite
        };
        
        Room {
            room_type,
            cleared: false,
            description: self.get_room_description(room_type),
        }
    }

    fn get_room_description(&self, room_type: RoomType) -> String {
        let mut rng = rand::thread_rng();
        match room_type {
            RoomType::Combat => {
                let descriptions = [
                    "Something stirs in the shadows...",
                    "You hear the clacking of keys ahead.",
                    "A hostile presence blocks your path!",
                    "The air crackles with static electricity.",
                    "Error messages flash on distant screens.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Elite => {
                let descriptions = [
                    "A powerful enemy awaits!",
                    "The ground trembles with each keystroke.",
                    "A formidable foe guards this chamber.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Treasure => {
                let descriptions = [
                    "A glittering chest catches your eye!",
                    "Something valuable lies ahead.",
                    "You spot a mysterious package.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Rest => {
                let descriptions = [
                    "A peaceful campfire flickers invitingly.",
                    "A comfortable keyboard rest awaits.",
                    "A save point glows softly in the darkness.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Shop => {
                let descriptions = [
                    "A mysterious merchant has set up shop.",
                    "\"Welcome, weary typist...\"",
                    "Goods and wares line the walls.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Event => {
                let descriptions = [
                    "Something unusual catches your attention...",
                    "An interesting situation presents itself.",
                    "The unexpected awaits around the corner.",
                ];
                descriptions[rng.gen_range(0..descriptions.len())].to_string()
            }
            RoomType::Start => "Your journey begins here.".to_string(),
            RoomType::Boss => self.get_boss_room_description(),
        }
    }

    fn get_boss_room_description(&self) -> String {
        match self.current_floor {
            5 => "THE GREAT QWERTY awaits in their throne room...".to_string(),
            10 => "CLIPPY THE FALLEN lurks in the corrupted office...".to_string(),
            _ => "A powerful boss blocks your path!".to_string(),
        }
    }

    pub fn advance_floor(&mut self) {
        self.current_floor += 1;
        self.rooms_cleared = 0;
        self.floor_complete = false;
        
        // Check for zone transition
        let zone = FloorZone::from_floor(self.current_floor as u32);
        let zone_changed = self.zone_name != zone.name();
        self.zone_name = zone.name().to_string();
        
        // Set zone message if we entered a new zone
        if zone_changed {
            self.zone_message = get_zone_entry_message(self.current_floor as u32);
        }
        
        let description = if zone_changed {
            format!(
                "Floor {} — {}\n\n{}",
                self.current_floor,
                zone.name(),
                get_ambient_message(self.current_floor as u32)
            )
        } else {
            format!(
                "Floor {} — {}\n\n{}",
                self.current_floor,
                zone.name(),
                get_ambient_message(self.current_floor as u32)
            )
        };
        
        self.current_room = Room {
            room_type: RoomType::Start,
            cleared: true,
            description,
        };
    }

    pub fn get_floor_name(&self) -> &'static str {
        match self.current_floor {
            1..=2 => "Tutorial",
            3..=4 => "Mechanical",
            5 => "QWERTY Domain",
            6..=7 => "Digital",
            8..=9 => "Legacy Code",
            10 => "Clippy's Lair",
            _ => "Unknown",
        }
    }

    pub fn get_difficulty(&self) -> i32 {
        match self.current_floor {
            1..=2 => 1,
            3..=4 => 2,
            5..=6 => 3,
            7..=8 => 4,
            _ => 5,
        }
    }

    pub fn room_cleared(&mut self) {
        self.current_room.cleared = true;
        self.rooms_cleared += 1;
    }

    pub fn get_ascii_map(&self) -> String {
        let total_rooms = self.rooms_per_floor;
        let cleared = self.rooms_cleared;
        
        let mut map = String::new();
        map.push_str(&format!("╔═══════════════════════════════╗\n"));
        map.push_str(&format!("║  FLOOR {} - {}  \n", 
            self.current_floor, 
            self.get_floor_name()
        ));
        map.push_str(&format!("╠═══════════════════════════════╣\n"));
        map.push_str("║ Progress: ");
        
        for i in 0..total_rooms {
            if i < cleared {
                map.push_str("█ ");
            } else if i == cleared {
                map.push_str("▓ ");
            } else {
                map.push_str("░ ");
            }
        }
        map.push_str("║\n");
        map.push_str(&format!("║ Rooms: {}/{}", cleared, total_rooms));
        if self.current_floor % 5 == 0 {
            map.push_str(" [BOSS FLOOR]");
        }
        map.push_str(" ║\n");
        map.push_str(&format!("╚═══════════════════════════════╝\n"));
        
        map
    }
}

impl Room {
    pub fn get_icon(&self) -> &'static str {
        match self.room_type {
            RoomType::Combat => "⚔",
            RoomType::Elite => "󰚌",
            RoomType::Boss => "👑",
            RoomType::Treasure => "󰆧",
            RoomType::Rest => "󰒲",
            RoomType::Shop => "🛒",
            RoomType::Event => "❓",
            RoomType::Start => "🚪",
        }
    }
}
