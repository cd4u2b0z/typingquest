//! Random events - Undertale-style encounters!

use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub name: String,
    pub description: String,
    pub choices: Vec<EventChoice>,
    pub ascii_art: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChoice {
    pub text: String,
    pub outcome: EventOutcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    GainGold(i32),
    LoseGold(i32),
    GainHP(i32),
    LoseHP(i32),
    GainXP(i32),
    GainMaxHP(i32),
    GainItem,
    Nothing,
    Combat,
}

impl GameEvent {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let events = Self::get_event_pool();
        events.choose(&mut rng).unwrap().clone()
    }

    fn get_event_pool() -> Vec<Self> {
        vec![
            GameEvent {
                name: "The Mysterious Keyboard".to_string(),
                description: "You find an ancient mechanical keyboard glowing softly in the darkness. Its keys seem to whisper secrets of typing mastery...".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Press a key gently".to_string(),
                        outcome: EventOutcome::GainXP(25),
                    },
                    EventChoice {
                        text: "Mash all the keys!".to_string(),
                        outcome: EventOutcome::Combat,
                    },
                    EventChoice {
                        text: "Walk away respectfully".to_string(),
                        outcome: EventOutcome::GainMaxHP(5),
                    },
                ],
                ascii_art: concat!(
                    "  +-----------------------+\n",
                    "  | [Q][W][E][R][T][Y][U] |\n",
                    "  | [A][S][D][F][G][H][J] |\n",
                    "  |  [Z][X][C][V][B][N]   |\n",
                    "  +-----------------------+"
                ).to_string(),
            },
            GameEvent {
                name: "The Coding Shrine".to_string(),
                description: "A small shrine dedicated to the Programming Gods stands before you. An offering bowl sits empty...".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Offer 20 gold".to_string(),
                        outcome: EventOutcome::LoseGold(20),
                    },
                    EventChoice {
                        text: "Pray for luck".to_string(),
                        outcome: EventOutcome::GainItem,
                    },
                    EventChoice {
                        text: "Steal from the shrine".to_string(),
                        outcome: EventOutcome::Combat,
                    },
                ],
                ascii_art: concat!(
                    "      /\\\n",
                    "     /  \\\n",
                    "    /    \\\n",
                    "   +------+\n",
                    "   | </>  |\n",
                    "   +------+"
                ).to_string(),
            },
            GameEvent {
                name: "The Lost Typist".to_string(),
                description: "A weary traveler sits by the path. They look up with tired eyes.".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Help them find their way".to_string(),
                        outcome: EventOutcome::GainGold(30),
                    },
                    EventChoice {
                        text: "Give them gold".to_string(),
                        outcome: EventOutcome::LoseGold(10),
                    },
                    EventChoice {
                        text: "Ignore them".to_string(),
                        outcome: EventOutcome::Nothing,
                    },
                ],
                ascii_art: "    o\n   /|\\\n   / \\".to_string(),
            },
            GameEvent {
                name: "Clippy Appears!".to_string(),
                description: "It looks like you are trying to complete a dungeon! Would you like help with that?".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Yes, please help!".to_string(),
                        outcome: EventOutcome::GainHP(20),
                    },
                    EventChoice {
                        text: "No thanks".to_string(),
                        outcome: EventOutcome::Nothing,
                    },
                    EventChoice {
                        text: "NEVER ASK ME AGAIN".to_string(),
                        outcome: EventOutcome::LoseHP(5),
                    },
                ],
                ascii_art: "   +---+\n   | ? |\n   +-+-+\n     |".to_string(),
            },
            GameEvent {
                name: "The Vending Machine".to_string(),
                description: "An old vending machine hums in the corner. The options look... interesting.".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Insert coin (10g)".to_string(),
                        outcome: EventOutcome::GainItem,
                    },
                    EventChoice {
                        text: "Shake it violently".to_string(),
                        outcome: EventOutcome::Combat,
                    },
                    EventChoice {
                        text: "Walk away".to_string(),
                        outcome: EventOutcome::Nothing,
                    },
                ],
                ascii_art: concat!(
                    "  +---------+\n",
                    "  | SNACKS! |\n",
                    "  | [A][B]  |\n",
                    "  | [C][D]  |\n",
                    "  +---------+"
                ).to_string(),
            },
            GameEvent {
                name: "Fountain of Syntax".to_string(),
                description: "A beautiful fountain flows with glowing blue liquid. Ancient runes read: Drink deep, or drink not at all.".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Take a small sip".to_string(),
                        outcome: EventOutcome::GainHP(10),
                    },
                    EventChoice {
                        text: "Drink deeply".to_string(),
                        outcome: EventOutcome::GainMaxHP(10),
                    },
                    EventChoice {
                        text: "Splash face only".to_string(),
                        outcome: EventOutcome::GainXP(15),
                    },
                ],
                ascii_art: "   \\|/\n  --*--\n   /|\\\n  /   \\".to_string(),
            },
            GameEvent {
                name: "The Stack Overflow".to_string(),
                description: "Books and scrolls tower precariously high. One wrong move and they will all come crashing down!".to_string(),
                choices: vec![
                    EventChoice {
                        text: "Carefully grab a book".to_string(),
                        outcome: EventOutcome::GainXP(30),
                    },
                    EventChoice {
                        text: "Push the stack over".to_string(),
                        outcome: EventOutcome::Combat,
                    },
                    EventChoice {
                        text: "Leave it alone".to_string(),
                        outcome: EventOutcome::Nothing,
                    },
                ],
                ascii_art: "    +=+\n   +===+\n  +=====+\n +=======+".to_string(),
            },
        ]
    }
}
