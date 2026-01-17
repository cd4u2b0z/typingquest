//! Sentence data structures and loaders

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A collection of sentences organized by category and difficulty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceDatabase {
    pub literature: Vec<SentenceEntry>,
    pub philosophy: Vec<SentenceEntry>,
    pub poetry: Vec<SentenceEntry>,
    pub technical: Vec<SentenceEntry>,
    pub nature: Vec<SentenceEntry>,
    pub combat: Vec<SentenceEntry>,
    pub boss_specific: HashMap<String, Vec<SentenceEntry>>,
    pub faction_specific: HashMap<String, Vec<SentenceEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceEntry {
    pub text: String,
    pub source: String,
    pub difficulty: u32,  // 1-10
    pub tags: Vec<String>,
}

impl Default for SentenceDatabase {
    fn default() -> Self {
        Self::embedded()
    }
}

impl SentenceDatabase {
    /// Get sentences by difficulty range
    pub fn get_by_difficulty(&self, min: u32, max: u32) -> Vec<&SentenceEntry> {
        let mut results = Vec::new();
        
        for entry in self.literature.iter()
            .chain(self.philosophy.iter())
            .chain(self.poetry.iter())
            .chain(self.technical.iter())
            .chain(self.nature.iter())
            .chain(self.combat.iter())
        {
            if entry.difficulty >= min && entry.difficulty <= max {
                results.push(entry);
            }
        }
        
        results
    }
    
    /// Get sentences for a specific boss
    pub fn get_boss_sentences(&self, boss_id: &str) -> Vec<&SentenceEntry> {
        self.boss_specific
            .get(boss_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get faction-flavored sentences
    pub fn get_faction_sentences(&self, faction: &str) -> Vec<&SentenceEntry> {
        self.faction_specific
            .get(faction)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    
    /// Embedded default database - used when no external file exists
    pub fn embedded() -> Self {
        let mut boss_specific = HashMap::new();
        let mut faction_specific = HashMap::new();
        
        // Boss: The Unwriter
        boss_specific.insert("the_unwriter".to_string(), vec![
            SentenceEntry {
                text: "I am the silence between words, the void where meaning dies.".to_string(),
                source: "The Unwriter".to_string(),
                difficulty: 7,
                tags: vec!["boss".to_string(), "dark".to_string()],
            },
            SentenceEntry {
                text: "Every word you type feeds my hunger. Every sentence extends my reach.".to_string(),
                source: "The Unwriter".to_string(),
                difficulty: 8,
                tags: vec!["boss".to_string(), "threat".to_string()],
            },
            SentenceEntry {
                text: "You cannot defeat entropy with keystrokes. Oblivion is patient.".to_string(),
                source: "The Unwriter".to_string(),
                difficulty: 8,
                tags: vec!["boss".to_string(), "philosophy".to_string()],
            },
        ]);
        
        // Boss: Corruption Elemental
        boss_specific.insert("corruption_elemental".to_string(), vec![
            SentenceEntry {
                text: "Yooouur woooords... beeecome... miiiine...".to_string(),
                source: "Corruption Elemental".to_string(),
                difficulty: 6,
                tags: vec!["boss".to_string(), "corrupted".to_string()],
            },
            SentenceEntry {
                text: "The meaning seeps away like water through cracked stone.".to_string(),
                source: "Corruption Elemental".to_string(),
                difficulty: 7,
                tags: vec!["boss".to_string()],
            },
        ]);
        
        // Faction: Scribes (formal, precise)
        faction_specific.insert("scribes".to_string(), vec![
            SentenceEntry {
                text: "Precision in typing reflects precision in thought. Let every keystroke be deliberate.".to_string(),
                source: "Scribe Doctrine".to_string(),
                difficulty: 6,
                tags: vec!["faction".to_string(), "scribes".to_string()],
            },
            SentenceEntry {
                text: "We do not race against time. We master it through measured, accurate strokes.".to_string(),
                source: "Grand Scribe Aldric".to_string(),
                difficulty: 7,
                tags: vec!["faction".to_string(), "scribes".to_string()],
            },
        ]);
        
        // Faction: Mechanists (speed, efficiency)
        faction_specific.insert("mechanists".to_string(), vec![
            SentenceEntry {
                text: "Speed is survival. Hesitation is extinction. Type or be forgotten.".to_string(),
                source: "Mechanist Creed".to_string(),
                difficulty: 6,
                tags: vec!["faction".to_string(), "mechanists".to_string()],
            },
            SentenceEntry {
                text: "The keyboard is a weapon. Your fingers are soldiers. Every millisecond counts.".to_string(),
                source: "Commander Steele".to_string(),
                difficulty: 7,
                tags: vec!["faction".to_string(), "mechanists".to_string()],
            },
        ]);
        
        // Faction: Naturalists (organic, flowing)
        faction_specific.insert("naturalists".to_string(), vec![
            SentenceEntry {
                text: "Type as the river flows, as the wind breathes, as the seasons turn.".to_string(),
                source: "Elder Root".to_string(),
                difficulty: 5,
                tags: vec!["faction".to_string(), "naturalists".to_string()],
            },
            SentenceEntry {
                text: "The Green Word predates all alphabets. We merely translate its ancient wisdom.".to_string(),
                source: "Naturalist Teaching".to_string(),
                difficulty: 6,
                tags: vec!["faction".to_string(), "naturalists".to_string()],
            },
        ]);
        
        // Faction: Shadow Writers (cryptic, mysterious)
        faction_specific.insert("shadow_writers".to_string(), vec![
            SentenceEntry {
                text: "The most powerful words are those never typed. Silence speaks volumes.".to_string(),
                source: "Shadow Writer Proverb".to_string(),
                difficulty: 6,
                tags: vec!["faction".to_string(), "shadow_writers".to_string()],
            },
            SentenceEntry {
                text: "Information is currency. Secrets are power. Type carefully what you reveal.".to_string(),
                source: "Whisper".to_string(),
                difficulty: 7,
                tags: vec!["faction".to_string(), "shadow_writers".to_string()],
            },
        ]);
        
        // Faction: Archivists (scholarly, preserving)
        faction_specific.insert("archivists".to_string(), vec![
            SentenceEntry {
                text: "Every word preserved is a victory against the Unwriting. We are memory's guardians.".to_string(),
                source: "Archivist Vera".to_string(),
                difficulty: 7,
                tags: vec!["faction".to_string(), "archivists".to_string()],
            },
            SentenceEntry {
                text: "The Athenaeum contains texts older than nations. Handle each word with reverence.".to_string(),
                source: "Archivist Training Manual".to_string(),
                difficulty: 7,
                tags: vec!["faction".to_string(), "archivists".to_string()],
            },
        ]);
        
        Self {
            literature: vec![
                // Shakespeare
                SentenceEntry {
                    text: "To be, or not to be, that is the question.".to_string(),
                    source: "Shakespeare, Hamlet".to_string(),
                    difficulty: 3,
                    tags: vec!["classic".to_string(), "shakespeare".to_string()],
                },
                SentenceEntry {
                    text: "All the world's a stage, and all the men and women merely players.".to_string(),
                    source: "Shakespeare, As You Like It".to_string(),
                    difficulty: 5,
                    tags: vec!["classic".to_string(), "shakespeare".to_string()],
                },
                SentenceEntry {
                    text: "The fault, dear Brutus, is not in our stars, but in ourselves.".to_string(),
                    source: "Shakespeare, Julius Caesar".to_string(),
                    difficulty: 5,
                    tags: vec!["classic".to_string(), "shakespeare".to_string()],
                },
                // Tolkien
                SentenceEntry {
                    text: "Not all those who wander are lost.".to_string(),
                    source: "J.R.R. Tolkien, The Fellowship of the Ring".to_string(),
                    difficulty: 3,
                    tags: vec!["fantasy".to_string(), "tolkien".to_string()],
                },
                SentenceEntry {
                    text: "Even the smallest person can change the course of the future.".to_string(),
                    source: "J.R.R. Tolkien, The Fellowship of the Ring".to_string(),
                    difficulty: 5,
                    tags: vec!["fantasy".to_string(), "tolkien".to_string()],
                },
                SentenceEntry {
                    text: "It is not despair, for despair is only for those who see the end beyond all doubt.".to_string(),
                    source: "J.R.R. Tolkien, The Fellowship of the Ring".to_string(),
                    difficulty: 7,
                    tags: vec!["fantasy".to_string(), "tolkien".to_string()],
                },
                // Orwell
                SentenceEntry {
                    text: "War is peace. Freedom is slavery. Ignorance is strength.".to_string(),
                    source: "George Orwell, 1984".to_string(),
                    difficulty: 4,
                    tags: vec!["dystopia".to_string(), "orwell".to_string()],
                },
                SentenceEntry {
                    text: "Who controls the past controls the future. Who controls the present controls the past.".to_string(),
                    source: "George Orwell, 1984".to_string(),
                    difficulty: 7,
                    tags: vec!["dystopia".to_string(), "orwell".to_string()],
                },
                // Dostoevsky
                SentenceEntry {
                    text: "Pain and suffering are always inevitable for a large intelligence and a deep heart.".to_string(),
                    source: "Fyodor Dostoevsky, Crime and Punishment".to_string(),
                    difficulty: 7,
                    tags: vec!["russian".to_string(), "dostoevsky".to_string()],
                },
                // Hemingway
                SentenceEntry {
                    text: "The world breaks everyone, and afterward, some are strong at the broken places.".to_string(),
                    source: "Ernest Hemingway, A Farewell to Arms".to_string(),
                    difficulty: 6,
                    tags: vec!["american".to_string(), "hemingway".to_string()],
                },
                // Camus
                SentenceEntry {
                    text: "In the depth of winter, I finally learned that within me there lay an invincible summer.".to_string(),
                    source: "Albert Camus".to_string(),
                    difficulty: 7,
                    tags: vec!["existentialism".to_string(), "camus".to_string()],
                },
            ],
            
            philosophy: vec![
                // Marcus Aurelius
                SentenceEntry {
                    text: "You have power over your mind, not outside events. Realize this, and you will find strength.".to_string(),
                    source: "Marcus Aurelius, Meditations".to_string(),
                    difficulty: 7,
                    tags: vec!["stoicism".to_string(), "aurelius".to_string()],
                },
                SentenceEntry {
                    text: "The happiness of your life depends upon the quality of your thoughts.".to_string(),
                    source: "Marcus Aurelius, Meditations".to_string(),
                    difficulty: 5,
                    tags: vec!["stoicism".to_string(), "aurelius".to_string()],
                },
                SentenceEntry {
                    text: "Waste no more time arguing about what a good man should be. Be one.".to_string(),
                    source: "Marcus Aurelius, Meditations".to_string(),
                    difficulty: 5,
                    tags: vec!["stoicism".to_string(), "aurelius".to_string()],
                },
                // Nietzsche
                SentenceEntry {
                    text: "He who has a why to live can bear almost any how.".to_string(),
                    source: "Friedrich Nietzsche".to_string(),
                    difficulty: 4,
                    tags: vec!["existentialism".to_string(), "nietzsche".to_string()],
                },
                SentenceEntry {
                    text: "Without music, life would be a mistake.".to_string(),
                    source: "Friedrich Nietzsche, Twilight of the Idols".to_string(),
                    difficulty: 3,
                    tags: vec!["existentialism".to_string(), "nietzsche".to_string()],
                },
                SentenceEntry {
                    text: "That which does not kill us makes us stronger.".to_string(),
                    source: "Friedrich Nietzsche".to_string(),
                    difficulty: 4,
                    tags: vec!["existentialism".to_string(), "nietzsche".to_string()],
                },
                // Seneca
                SentenceEntry {
                    text: "We suffer more often in imagination than in reality.".to_string(),
                    source: "Seneca".to_string(),
                    difficulty: 4,
                    tags: vec!["stoicism".to_string(), "seneca".to_string()],
                },
                SentenceEntry {
                    text: "Luck is what happens when preparation meets opportunity.".to_string(),
                    source: "Seneca".to_string(),
                    difficulty: 4,
                    tags: vec!["stoicism".to_string(), "seneca".to_string()],
                },
                // Epictetus
                SentenceEntry {
                    text: "It is not things that disturb us, but our judgments about those things.".to_string(),
                    source: "Epictetus, Enchiridion".to_string(),
                    difficulty: 6,
                    tags: vec!["stoicism".to_string(), "epictetus".to_string()],
                },
                // Plato
                SentenceEntry {
                    text: "The measure of a man is what he does with power.".to_string(),
                    source: "Plato".to_string(),
                    difficulty: 4,
                    tags: vec!["greek".to_string(), "plato".to_string()],
                },
                // Aristotle
                SentenceEntry {
                    text: "We are what we repeatedly do. Excellence, then, is not an act, but a habit.".to_string(),
                    source: "Aristotle".to_string(),
                    difficulty: 6,
                    tags: vec!["greek".to_string(), "aristotle".to_string()],
                },
            ],
            
            poetry: vec![
                // Dylan Thomas
                SentenceEntry {
                    text: "Do not go gentle into that good night. Rage, rage against the dying of the light.".to_string(),
                    source: "Dylan Thomas".to_string(),
                    difficulty: 6,
                    tags: vec!["poetry".to_string(), "thomas".to_string()],
                },
                // Robert Frost
                SentenceEntry {
                    text: "Two roads diverged in a wood, and I took the one less traveled by.".to_string(),
                    source: "Robert Frost, The Road Not Taken".to_string(),
                    difficulty: 5,
                    tags: vec!["poetry".to_string(), "frost".to_string()],
                },
                SentenceEntry {
                    text: "In three words I can sum up everything I have learned about life: it goes on.".to_string(),
                    source: "Robert Frost".to_string(),
                    difficulty: 6,
                    tags: vec!["poetry".to_string(), "frost".to_string()],
                },
                // T.S. Eliot
                SentenceEntry {
                    text: "This is the way the world ends, not with a bang but a whimper.".to_string(),
                    source: "T.S. Eliot, The Hollow Men".to_string(),
                    difficulty: 5,
                    tags: vec!["poetry".to_string(), "eliot".to_string()],
                },
                // Emily Dickinson
                SentenceEntry {
                    text: "Hope is the thing with feathers that perches in the soul.".to_string(),
                    source: "Emily Dickinson".to_string(),
                    difficulty: 5,
                    tags: vec!["poetry".to_string(), "dickinson".to_string()],
                },
                // Rumi
                SentenceEntry {
                    text: "The wound is the place where the Light enters you.".to_string(),
                    source: "Rumi".to_string(),
                    difficulty: 4,
                    tags: vec!["poetry".to_string(), "sufi".to_string()],
                },
                SentenceEntry {
                    text: "What you seek is seeking you.".to_string(),
                    source: "Rumi".to_string(),
                    difficulty: 2,
                    tags: vec!["poetry".to_string(), "sufi".to_string()],
                },
            ],
            
            technical: vec![
                SentenceEntry {
                    text: "The keyboard is mightier than the sword, but only in skilled hands.".to_string(),
                    source: "Mechanist Proverb".to_string(),
                    difficulty: 5,
                    tags: vec!["typing".to_string()],
                },
                SentenceEntry {
                    text: "Precision beats speed when speed lacks direction.".to_string(),
                    source: "Typing Wisdom".to_string(),
                    difficulty: 4,
                    tags: vec!["typing".to_string()],
                },
                SentenceEntry {
                    text: "The quick brown fox jumps over the lazy dog.".to_string(),
                    source: "Pangram".to_string(),
                    difficulty: 3,
                    tags: vec!["pangram".to_string(), "practice".to_string()],
                },
                SentenceEntry {
                    text: "Pack my box with five dozen liquor jugs.".to_string(),
                    source: "Pangram".to_string(),
                    difficulty: 4,
                    tags: vec!["pangram".to_string(), "practice".to_string()],
                },
                SentenceEntry {
                    text: "How vexingly quick daft zebras jump!".to_string(),
                    source: "Pangram".to_string(),
                    difficulty: 4,
                    tags: vec!["pangram".to_string(), "practice".to_string()],
                },
            ],
            
            nature: vec![
                SentenceEntry {
                    text: "In every walk with nature, one receives far more than he seeks.".to_string(),
                    source: "John Muir".to_string(),
                    difficulty: 5,
                    tags: vec!["nature".to_string(), "muir".to_string()],
                },
                SentenceEntry {
                    text: "The clearest way into the Universe is through a forest wilderness.".to_string(),
                    source: "John Muir".to_string(),
                    difficulty: 5,
                    tags: vec!["nature".to_string(), "muir".to_string()],
                },
                SentenceEntry {
                    text: "Look deep into nature, and then you will understand everything better.".to_string(),
                    source: "Albert Einstein".to_string(),
                    difficulty: 5,
                    tags: vec!["nature".to_string(), "einstein".to_string()],
                },
                SentenceEntry {
                    text: "The earth has music for those who listen.".to_string(),
                    source: "William Shakespeare".to_string(),
                    difficulty: 4,
                    tags: vec!["nature".to_string(), "shakespeare".to_string()],
                },
            ],
            
            combat: vec![
                SentenceEntry {
                    text: "Steel yourself. The enemy approaches with malice aforethought.".to_string(),
                    source: "Combat Tutorial".to_string(),
                    difficulty: 5,
                    tags: vec!["combat".to_string()],
                },
                SentenceEntry {
                    text: "Victory belongs to those whose fingers never falter.".to_string(),
                    source: "Warrior's Creed".to_string(),
                    difficulty: 4,
                    tags: vec!["combat".to_string()],
                },
                SentenceEntry {
                    text: "In battle, hesitation is death. Type with conviction.".to_string(),
                    source: "Combat Wisdom".to_string(),
                    difficulty: 4,
                    tags: vec!["combat".to_string()],
                },
                SentenceEntry {
                    text: "The corruption writhes before you. Each keystroke is a blade.".to_string(),
                    source: "Battle Narrator".to_string(),
                    difficulty: 5,
                    tags: vec!["combat".to_string(), "corruption".to_string()],
                },
                SentenceEntry {
                    text: "Your words are weapons. Your accuracy is armor. Fight!".to_string(),
                    source: "Battle Cry".to_string(),
                    difficulty: 4,
                    tags: vec!["combat".to_string()],
                },
            ],
            
            boss_specific,
            faction_specific,
        }
    }
}
