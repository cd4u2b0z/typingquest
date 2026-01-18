//! Word list data structures and loaders
//!
//! Comprehensive word database for typing combat.
//! Words are organized by difficulty (letter count) and theme.

use serde::{Deserialize, Serialize};

/// Word lists organized by difficulty and theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDatabase {
    pub easy: Vec<String>,      // 3-4 letters
    pub medium: Vec<String>,    // 5-7 letters
    pub hard: Vec<String>,      // 8-10 letters
    pub expert: Vec<String>,    // 11+ letters
    pub themed: ThemeWords,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeWords {
    pub magic: Vec<String>,
    pub combat: Vec<String>,
    pub nature: Vec<String>,
    pub technology: Vec<String>,
    pub corruption: Vec<String>,
    pub ancient: Vec<String>,
    pub undead: Vec<String>,
    pub void: Vec<String>,
    pub fire: Vec<String>,
    pub water: Vec<String>,
    pub shadow: Vec<String>,
    pub holy: Vec<String>,
}

impl Default for WordDatabase {
    fn default() -> Self {
        Self::embedded()
    }
}

impl WordDatabase {
    /// Get words by difficulty level (1-10)
    pub fn get_by_difficulty(&self, level: u32) -> Vec<&String> {
        match level {
            1..=2 => self.easy.iter().collect(),
            3..=4 => self.easy.iter().chain(self.medium.iter()).collect(),
            5..=6 => self.medium.iter().collect(),
            7..=8 => self.medium.iter().chain(self.hard.iter()).collect(),
            9..=10 => self.hard.iter().chain(self.expert.iter()).collect(),
            _ => self.medium.iter().collect(),
        }
    }
    
    /// Get themed words for special encounters
    pub fn get_themed(&self, theme: &str) -> Vec<&String> {
        match theme {
            "magic" => self.themed.magic.iter().collect(),
            "combat" => self.themed.combat.iter().collect(),
            "nature" => self.themed.nature.iter().collect(),
            "technology" => self.themed.technology.iter().collect(),
            "corruption" => self.themed.corruption.iter().collect(),
            "ancient" => self.themed.ancient.iter().collect(),
            "undead" => self.themed.undead.iter().collect(),
            "void" => self.themed.void.iter().collect(),
            "fire" => self.themed.fire.iter().collect(),
            "water" => self.themed.water.iter().collect(),
            "shadow" => self.themed.shadow.iter().collect(),
            "holy" => self.themed.holy.iter().collect(),
            _ => self.medium.iter().collect(),
        }
    }
    
    /// Embedded default database
    pub fn embedded() -> Self {
        Self {
            // =========================================
            // EASY WORDS (3-4 letters) - ~150 words
            // =========================================
            easy: vec![
                // Common words
                "the", "and", "for", "are", "but", "not", "you", "all",
                "can", "had", "her", "was", "one", "our", "out", "day",
                "get", "has", "him", "his", "how", "its", "may", "new",
                "now", "old", "see", "two", "way", "who", "boy", "did",
                "let", "put", "say", "she", "too", "use", "man", "any",
                // Action words
                "key", "run", "hit", "cut", "aim", "try", "win", "fly",
                "die", "ask", "eat", "sit", "end", "fix", "mix", "set",
                // Fantasy themed
                "type", "word", "fast", "slow", "book", "read", "mind",
                "soul", "fire", "wind", "dark", "glow", "echo", "void",
                "rune", "mage", "sage", "lore", "fate", "doom", "bane",
                "bone", "fang", "claw", "horn", "wing", "tail", "hide",
                // Elements
                "ice", "ash", "mud", "fog", "dew", "ray", "orb", "gem",
                "gold", "iron", "rust", "dust", "mist", "haze", "gale",
                // Combat
                "war", "foe", "ally", "helm", "mail", "bow", "axe", "rod",
                "wand", "orb", "tome", "seal", "mark", "sign", "ward",
                // Places
                "hall", "gate", "wall", "door", "room", "path", "road",
                "cave", "den", "lair", "pit", "tomb", "crypt", "keep",
                // States
                "calm", "wild", "bold", "meek", "grim", "pale", "cold",
                "warm", "hot", "wet", "dry", "soft", "hard", "weak",
                // More fantasy
                "hex", "jinx", "boon", "pact", "vow", "oath", "rite",
                "cult", "sect", "clan", "king", "lord", "duke", "rage",
            ].into_iter().map(String::from).collect(),
            
            // =========================================
            // MEDIUM WORDS (5-7 letters) - ~200 words
            // =========================================
            medium: vec![
                // Typing/text themed
                "typing", "script", "letter", "symbol", "cipher", "riddle",
                "written", "spoken", "scroll", "codex", "text", "words",
                "reader", "writer", "scribe", "author", "chapter", "verse",
                // Fantasy core
                "wisdom", "battle", "ancient", "mystic", "shadow", "forest",
                "temple", "shrine", "sacred", "memory", "keeper", "warden",
                "master", "novice", "warrior", "silence", "whisper", "thunder",
                "crystal", "archive", "essence", "energy", "spirit", "phantom",
                "specter", "wraith", "corrupt", "tainted", "blessed", "cursed",
                "enchant", "dreams", "visions", "reality", "fiction", "legends",
                "stories", "history", "future", "present", "eternal", "mortal",
                "divine", "demonic", "angelic", "fallen", "risen", "reborn",
                // Combat
                "weapon", "shield", "armor", "helmet", "blade", "dagger",
                "sword", "hammer", "lance", "staff", "strike", "defend",
                "attack", "parry", "dodge", "thrust", "cleave", "smash",
                "pierce", "slash", "block", "evade", "counter", "charge",
                // Magic
                "arcane", "mana", "power", "spell", "ritual", "invoke",
                "summon", "conjure", "channel", "focus", "dispel", "banish",
                "bind", "seal", "unlock", "cursed", "blessed", "enchant",
                // Creatures
                "dragon", "griffin", "phoenix", "serpent", "spider", "wolf",
                "beast", "demon", "angel", "ghost", "zombie", "goblin",
                "troll", "ogre", "giant", "vampire", "wraith", "specter",
                // Environment
                "dungeon", "castle", "tower", "bridge", "valley", "river",
                "ocean", "desert", "tundra", "jungle", "swamp", "mountain",
                "ruins", "village", "city", "kingdom", "realm", "domain",
                // States/qualities
                "mighty", "cunning", "savage", "noble", "wicked", "primal",
                "burning", "frozen", "rotting", "healing", "growing", "fading",
                // Abstract
                "honor", "glory", "shame", "fear", "hope", "despair",
                "hatred", "love", "rage", "peace", "chaos", "order",
                "destiny", "fortune", "doom", "fate", "chance", "choice",
            ].into_iter().map(String::from).collect(),
            
            // =========================================
            // HARD WORDS (8-10 letters) - ~150 words
            // =========================================
            hard: vec![
                // Core gameplay
                "corruption", "unwriting", "keystroke", "precision",
                "transcend", "sanctuary", "illuminate", "obliterate",
                "manuscript", "literature", "philosophy", "meditation",
                "dedication", "persevere", "resilience", "determined",
                "typography", "vocabulary", "linguistic", "alphabetic",
                "encryption", "decryption", "mysterious", "revelation",
                // Fantasy
                "forbidden", "knowledge", "apocalypse", "cataclysm",
                "prophecy", "chronicle", "testament", "scripture",
                "anthology", "compendium", "enchanted", "bewitched",
                "possessed", "corrupted", "purified", "sanctified",
                "desecrated", "hallowed", "accursed", "vengeance",
                // Combat
                "skirmish", "onslaught", "ambush", "fortress",
                "stronghold", "citadel", "barricade", "vanguard",
                "rearguard", "flanking", "offensive", "defensive",
                "strategic", "tactical", "guerrilla", "conqueror",
                // Magic system
                "incantation", "evocation", "conjuration", "divination",
                "necromancy", "pyromancy", "cryomancy", "geomancy",
                "enchantment", "illusion", "abjuration", "transmute",
                "sorcerous", "wizardry", "witchcraft", "shamanism",
                // Creatures
                "abomination", "aberration", "monstrous", "grotesque",
                "nightmarish", "horrific", "eldritch", "unholy",
                "celestial", "infernal", "primordial", "elemental",
                // Environment
                "labyrinth", "catacomb", "mausoleum", "sepulcher",
                "threshold", "precipice", "wasteland", "wilderness",
                "underworld", "overworld", "cathedral", "colosseum",
                // States
                "triumphant", "victorious", "defeated", "vanquished",
                "ascending", "descending", "emerging", "submerging",
                "awakening", "slumbering", "wandering", "searching",
                // Abstract
                "redemption", "damnation", "salvation", "oblivion",
                "eternity", "infinity", "mortality", "sacrifice",
                "betrayal", "loyalty", "treachery", "vengeance",
            ].into_iter().map(String::from).collect(),
            
            // =========================================
            // EXPERT WORDS (11+ letters) - ~100 words
            // =========================================
            expert: vec![
                // Core concepts
                "transcendence", "consciousness", "determination",
                "extraordinary", "philosophical", "understanding",
                "enlightenment", "manifestation", "transformation",
                "disintegration", "reconstitution", "interpretation",
                "communication", "authentication", "configuration",
                "comprehension", "representation", "implementation",
                "documentation", "pronunciation", "disambiguation",
                // Fantasy/magic
                "incantations", "transmutation", "teleportation",
                "resurrection", "reincarnation", "materialization",
                "transfiguration", "metamorphosis", "necromantic",
                "pyrotechnic", "electromagnetic", "crystallization",
                // Combat/war
                "confrontation", "annihilation", "extermination",
                "assassination", "fortification", "reinforcement",
                "counterattack", "overwhelming", "insurmountable",
                "impenetrable", "indestructible", "invulnerable",
                // States/qualities
                "unforgettable", "unforgivable", "irreversible",
                "incomprehensible", "indescribable", "unimaginable",
                "unprecedented", "supernatural", "preternatural",
                "otherworldly", "interdimensional", "multiversal",
                // Abstract
                "subconscious", "omnipotence", "omniscience",
                "omnipresence", "transcendental", "metaphysical",
                "eschatological", "apocalyptic", "cataclysmic",
                // Story/narrative
                "foreshadowing", "retrospective", "introspective",
                "psychological", "mythological", "archaeological",
                "anthropological", "etymological", "cosmological",
            ].into_iter().map(String::from).collect(),
            
            // =========================================
            // THEMED WORD CATEGORIES
            // =========================================
            themed: ThemeWords {
                // MAGIC - Arcane and mystical
                magic: vec![
                    "spell", "enchant", "conjure", "summon", "invoke",
                    "arcane", "mystic", "eldritch", "sorcery", "wizardry",
                    "incantation", "transmutation", "evocation", "divination",
                    "rune", "glyph", "sigil", "ward", "hex", "curse", "blessing",
                    "mana", "aether", "essence", "focus", "channel", "conduit",
                    "grimoire", "spellbook", "scroll", "wand", "staff", "orb",
                    "ritual", "ceremony", "invocation", "banishment", "binding",
                    "illusion", "phantasm", "mirage", "glamour", "enchantment",
                    "telekinesis", "telepathy", "clairvoyance", "precognition",
                ].into_iter().map(String::from).collect(),
                
                // COMBAT - Battle and warfare
                combat: vec![
                    "strike", "parry", "dodge", "block", "thrust",
                    "slash", "pierce", "shatter", "crush", "cleave",
                    "assault", "defend", "counter", "riposte", "feint",
                    "victory", "defeat", "battle", "skirmish", "duel",
                    "warfare", "conquest", "siege", "raid", "ambush",
                    "vanguard", "flank", "retreat", "charge", "advance",
                    "sword", "shield", "armor", "helm", "gauntlet",
                    "blade", "edge", "hilt", "pommel", "scabbard",
                    "archer", "knight", "soldier", "warrior", "champion",
                    "berserker", "paladin", "assassin", "ranger", "guard",
                ].into_iter().map(String::from).collect(),
                
                // NATURE - Wild and natural
                nature: vec![
                    "forest", "river", "mountain", "valley", "meadow",
                    "blossom", "willow", "verdant", "serene", "ancient",
                    "wilderness", "sanctuary", "grove", "glade", "thicket",
                    "stream", "cascade", "canopy", "undergrowth", "roots",
                    "oak", "pine", "birch", "ash", "elm", "maple", "cedar",
                    "fern", "moss", "lichen", "mushroom", "toadstool",
                    "deer", "wolf", "bear", "eagle", "owl", "fox", "rabbit",
                    "spring", "summer", "autumn", "winter", "season",
                    "bloom", "wither", "grow", "decay", "renew", "cycle",
                    "predator", "prey", "hunter", "hunted", "survival",
                ].into_iter().map(String::from).collect(),
                
                // TECHNOLOGY - Mechanical and constructed
                technology: vec![
                    "keyboard", "terminal", "circuit", "binary", "digital",
                    "process", "execute", "compile", "runtime", "protocol",
                    "algorithm", "interface", "mechanism", "automation",
                    "efficiency", "optimize", "overclock", "bandwidth",
                    "clockwork", "gears", "cogs", "springs", "pistons",
                    "engine", "machine", "device", "apparatus", "construct",
                    "steam", "pressure", "valve", "turbine", "generator",
                    "forge", "anvil", "hammer", "bellows", "furnace",
                    "blueprint", "schematic", "diagram", "design", "patent",
                    "inventor", "engineer", "artificer", "tinker", "smith",
                ].into_iter().map(String::from).collect(),
                
                // CORRUPTION - Decay and taint
                corruption: vec![
                    "taint", "decay", "rot", "wither", "corrupt",
                    "unwrite", "erase", "dissolve", "fragment", "scatter",
                    "distortion", "aberration", "mutation", "entropy",
                    "void", "null", "empty", "hollow", "broken", "shattered",
                    "blight", "plague", "pestilence", "disease", "infection",
                    "madness", "insanity", "delirium", "chaos", "disorder",
                    "twisted", "warped", "malformed", "deformed", "grotesque",
                    "festering", "putrid", "necrotic", "gangrenous", "septic",
                    "contaminate", "pollute", "defile", "desecrate", "profane",
                    "unmaking", "unraveling", "collapsing", "dissolving",
                ].into_iter().map(String::from).collect(),
                
                // ANCIENT - Old and timeless
                ancient: vec![
                    "primordial", "forgotten", "eternal", "timeless",
                    "ancestral", "prehistoric", "mythical", "legendary",
                    "chronicle", "testament", "scripture", "prophecy",
                    "relic", "artifact", "remnant", "vestige", "monument",
                    "elder", "archaic", "antique", "venerable", "aged",
                    "dynasty", "empire", "kingdom", "civilization", "epoch",
                    "ruins", "temple", "tomb", "crypt", "catacomb",
                    "hieroglyph", "inscription", "tablet", "obelisk", "monolith",
                    "ancestor", "forefather", "progenitor", "patriarch",
                    "legacy", "heritage", "tradition", "custom", "ritual",
                ].into_iter().map(String::from).collect(),
                
                // UNDEAD - Death and reanimation
                undead: vec![
                    "skeleton", "zombie", "ghoul", "wraith", "specter",
                    "ghost", "phantom", "shade", "revenant", "lich",
                    "vampire", "nosferatu", "draugr", "wight", "banshee",
                    "necropolis", "graveyard", "cemetery", "mausoleum", "crypt",
                    "coffin", "casket", "shroud", "burial", "interment",
                    "resurrection", "reanimation", "undeath", "unlife",
                    "decay", "decompose", "putrefy", "rot", "wither",
                    "soul", "spirit", "essence", "anima", "lifeforce",
                    "haunted", "cursed", "damned", "forsaken", "condemned",
                    "necromancer", "gravekeep", "undertaker", "deathknight",
                ].into_iter().map(String::from).collect(),
                
                // VOID - Emptiness and nothingness
                void: vec![
                    "void", "null", "nothing", "empty", "hollow",
                    "abyss", "chasm", "rift", "breach", "tear",
                    "darkness", "blackness", "shadow", "umbra", "penumbra",
                    "endless", "infinite", "boundless", "limitless", "vast",
                    "silent", "stillness", "calm", "peace", "serenity",
                    "consume", "devour", "absorb", "engulf", "swallow",
                    "erasure", "deletion", "removal", "absence", "lack",
                    "forgotten", "lost", "missing", "vanished", "gone",
                    "entropy", "dissolution", "dispersal", "scattering", "fading",
                    "cosmic", "stellar", "astral", "ethereal", "dimensional",
                ].into_iter().map(String::from).collect(),
                
                // FIRE - Flame and heat
                fire: vec![
                    "flame", "blaze", "inferno", "conflagration", "pyre",
                    "ember", "spark", "cinder", "ash", "smoke",
                    "burning", "scorching", "searing", "blazing", "incendiary",
                    "heat", "warmth", "thermal", "radiant", "glowing",
                    "phoenix", "salamander", "ifrit", "firedrake", "hellhound",
                    "magma", "lava", "molten", "volcanic", "eruption",
                    "combustion", "ignition", "kindling", "fuel", "tinder",
                    "torch", "lantern", "candle", "brazier", "furnace",
                    "dragonfire", "hellfire", "wildfire", "balefire", "sunfire",
                    "pyromancer", "firebrand", "arsonist", "pyromaniac",
                ].into_iter().map(String::from).collect(),
                
                // WATER - Ocean and flow
                water: vec![
                    "ocean", "sea", "river", "stream", "brook",
                    "wave", "tide", "current", "flow", "cascade",
                    "rain", "storm", "tempest", "hurricane", "typhoon",
                    "ice", "frost", "snow", "glacier", "frozen",
                    "mist", "fog", "vapor", "steam", "condensation",
                    "flood", "deluge", "torrent", "surge", "overflow",
                    "drown", "submerge", "immerse", "baptize", "cleanse",
                    "depths", "abyss", "trench", "fathom", "league",
                    "leviathan", "kraken", "serpent", "merfolk", "siren",
                    "hydromancer", "watermage", "seawitch", "tidecaller",
                ].into_iter().map(String::from).collect(),
                
                // SHADOW - Darkness and stealth
                shadow: vec![
                    "shadow", "shade", "darkness", "gloom", "murk",
                    "stealth", "silence", "unseen", "hidden", "concealed",
                    "assassin", "rogue", "thief", "spy", "infiltrator",
                    "dagger", "poison", "garrote", "trap", "ambush",
                    "midnight", "dusk", "twilight", "nightfall", "nocturnal",
                    "whisper", "murmur", "hush", "quiet", "still",
                    "cloak", "hood", "mask", "veil", "shroud",
                    "secret", "mystery", "enigma", "riddle", "puzzle",
                    "phantom", "specter", "wraith", "apparition", "ghost",
                    "shadowmancer", "nightblade", "shadewalker", "darkstalker",
                ].into_iter().map(String::from).collect(),
                
                // HOLY - Divine and sacred
                holy: vec![
                    "holy", "sacred", "divine", "blessed", "sanctified",
                    "light", "radiance", "glory", "brilliance", "luminous",
                    "angel", "seraph", "cherub", "archangel", "celestial",
                    "heaven", "paradise", "eden", "elysium", "nirvana",
                    "prayer", "blessing", "benediction", "sacrament", "rite",
                    "temple", "church", "cathedral", "chapel", "shrine",
                    "priest", "cleric", "paladin", "templar", "crusader",
                    "miracle", "wonder", "marvel", "portent", "omen",
                    "redemption", "salvation", "absolution", "atonement", "grace",
                    "smite", "purify", "consecrate", "anoint", "exorcise",
                ].into_iter().map(String::from).collect(),
            },
        }
    }
}
