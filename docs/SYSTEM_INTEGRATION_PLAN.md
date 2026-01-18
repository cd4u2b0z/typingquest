# System Integration Plan: Bringing Keyboard Warrior to Life

*"The scaffolding is ready. The world just needs bones, muscle, and a pulse."*

---

## ✅ COMPLETED - v0.5.0 (2026-01-18)

**All six phases have been implemented!** The dormant systems are now alive.

### Completion Summary

| Phase | System | Status | Commit |
|-------|--------|--------|--------|
| 1 | EventBus | ✅ Complete | 2c4507b |
| 2 | NarrativeSeed + Corruption | ✅ Complete | 2c4507b |
| 3 | SkillTree → Combat | ✅ Complete | 99f540d |
| 4 | VoiceSystem | ✅ Complete | daa18ea |
| 5 | EncounterWriting | ✅ Complete | bfa9ace |
| 6 | RunModifiers | ✅ Complete | f12b3de |

### What Was Wired
- **EventBus**: 60+ event types, central nervous system
- **NarrativeSeed**: Corruption effects modify typing patterns
- **SkillTree**: 10+ helper methods, combat damage/crit/evasion/reduction
- **VoiceSystem**: Faction-specific NPC dialogue in shops and rest sites
- **EncounterWriting**: Authored events trigger during exploration
- **RunModifiers**: Enemy scaling and reward multipliers

### Files Modified
- `src/game/state.rs` - Major hub for all integrations
- `src/game/combat.rs` - Skills and corruption integration
- `src/game/skills.rs` - 10+ new helper methods

---

## Original Plan (Reference)

## Executive Summary

You have **~24,500 lines of Rust** across 49 files. Roughly **40% is dormant** - beautifully designed systems that compile but never execute. This document provides a concrete plan to wire everything together.

### The Core Problem

```
CURRENT STATE:
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Combat    │     │  Exploration │     │   Dialogue  │
│  (active)   │     │   (active)   │     │  (minimal)  │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       └───────────────────┴───────────────────┘
                           │
                    ╔══════╧══════╗
                    ║   NOTHING   ║  ← No central nervous system
                    ╚═════════════╝

DESIRED STATE:
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Combat    │     │  Exploration │     │   Dialogue  │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       └─────────┬─────────┴─────────┬─────────┘
                 │                   │
          ┌──────▼──────┐     ┌──────▼──────┐
          │  EventBus   │◄───►│NarrativeSeed│
          │ (reactions) │     │ (coherence) │
          └──────┬──────┘     └─────────────┘
                 │
    ┌────────────┼────────────┐
    ▼            ▼            ▼
┌────────┐ ┌──────────┐ ┌──────────┐
│Factions│ │  Skills  │ │Modifiers │
└────────┘ └──────────┘ └──────────┘
```

---

## Phase 1: The Nervous System (EventBus)

**Priority: CRITICAL**  
**Estimated Effort: 2-3 hours**  
**Impact: Enables all other integrations**

### Why First?

The `EventBus` in `event_bus.rs` is your **decoupling layer**. Without it, every system needs to know about every other system. With it, systems emit events and react to events without tight coupling.

### Current State

`EventBus` exists with 60+ event types defined, but it's not instantiated or used anywhere.

### Wiring Steps

#### Step 1: Add EventBus to GameState

```rust
// src/game/state.rs
use crate::game::event_bus::EventBus;

pub struct GameState {
    // ... existing fields ...
    
    /// Central event messaging system
    pub event_bus: EventBus,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            // ... existing initialization ...
            event_bus: EventBus::new(),
        }
    }
}
```

#### Step 2: Emit Events from Combat

```rust
// src/game/combat.rs - in on_word_completed()
use crate::game::event_bus::GameEvent;

// After successful word completion:
self.event_bus.emit(GameEvent::Combat(CombatEvent::PerfectWord {
    word: word.clone(),
    wpm,
    combo: self.combo,
}));

// After damage dealt:
self.event_bus.emit(GameEvent::Combat(CombatEvent::DamageDealt {
    amount: damage,
    target: self.enemy.name.clone(),
    was_critical: crit,
}));

// After combat ends:
self.event_bus.emit(GameEvent::Combat(CombatEvent::Ended {
    victory,
    enemy_name: self.enemy.name.clone(),
    turns: self.turn,
}));
```

#### Step 3: React to Events in Game Loop

```rust
// src/main.rs - in game loop, after input handling
fn process_events(state: &mut GameState) {
    // Process all pending events
    for event in state.event_bus.drain() {
        match event {
            GameEvent::Combat(CombatEvent::Ended { victory, enemy_name, .. }) => {
                if victory {
                    // Update faction standings
                    state.faction_relations.on_enemy_defeated(&enemy_name);
                    // Check skill unlocks
                    state.check_skill_unlocks();
                }
            }
            GameEvent::Faction(FactionEvent::StandingChanged { faction, old, new }) => {
                // Trigger dialogue changes, quest availability, etc.
                state.refresh_available_quests();
            }
            // ... handle other events
            _ => {}
        }
    }
}
```

---

## Phase 2: Run Coherence (NarrativeSeed)

**Priority: HIGH**  
**Estimated Effort: 2-3 hours**  
**Impact: Every run feels unique and coherent**

### Why?

`NarrativeSeed` determines the **shape of each run** - which faction is dominant, what corruption type affects typing, what crisis is happening. Without it, the world feels random. With it, everything connects.

### Current State

`NarrativeSeed` exists with full corruption types, world moods, faction crises - but it's never generated or consulted.

### Wiring Steps

#### Step 1: Generate Seed at Run Start

```rust
// src/game/state.rs
use crate::game::narrative_seed::NarrativeSeed;

pub struct GameState {
    // ... existing fields ...
    
    /// The narrative seed for this run (determines world state)
    pub narrative_seed: Option<NarrativeSeed>,
}

impl GameState {
    pub fn start_new_run(&mut self) {
        // Generate fresh narrative seed
        let seed_value = rand::random::<u64>();
        self.narrative_seed = Some(NarrativeSeed::from_seed(seed_value));
        
        // Apply corruption type to typing mechanics
        if let Some(ref seed) = self.narrative_seed {
            self.apply_corruption_effects(seed.world_state.corruption_type);
        }
    }
}
```

#### Step 2: Corruption Types Affect Typing

```rust
// src/game/combat.rs
impl CombatState {
    pub fn apply_corruption(&mut self, corruption: CorruptionType) {
        match corruption {
            CorruptionType::SilentRot => {
                // Words occasionally lose letters mid-combat
                self.word_decay_enabled = true;
            }
            CorruptionType::EchoMadness => {
                // Mistakes deal damage to player
                self.mistake_damage = 2;
            }
            CorruptionType::BabelCurse => {
                // Random letters become different languages
                self.language_mixing = true;
            }
            CorruptionType::TruthInversion => {
                // Some words must be typed backwards
                self.inversion_chance = 0.15;
            }
            CorruptionType::MemoryPlague => {
                // Letters fade if not typed quickly
                self.letter_fade_rate = 0.5;
            }
            CorruptionType::ChronoStutter => {
                // Time pressure varies unpredictably
                self.time_pressure_variance = 2.0;
            }
        }
    }
}
```

#### Step 3: World Mood Affects Atmosphere

```rust
// src/ui/render.rs
fn get_ambient_color(world_mood: WorldMood) -> Color {
    match world_mood {
        WorldMood::Twilight => Color::Rgb(70, 70, 100),    // Muted purple
        WorldMood::Wartime => Color::Rgb(100, 50, 50),     // Blood red tint
        WorldMood::ColdWar => Color::Rgb(50, 50, 70),      // Steel grey
        WorldMood::Recovery => Color::Rgb(70, 90, 70),     // Hopeful green
        WorldMood::EndTimes => Color::Rgb(30, 0, 30),      // Dark void
        WorldMood::EeriePeace => Color::Rgb(80, 80, 60),   // Unsettling yellow
    }
}
```

---

## Phase 3: Combat Depth (TypingContext + Skills)

**Priority: HIGH**  
**Estimated Effort: 4-5 hours**  
**Impact: Combat feels dynamic and responsive**

### Why?

Right now combat is "type word, deal damage." With `TypingContext` and `Skills`, combat becomes "type word in darkness while managing stealth, deal damage boosted by your Precision tree while building toward Transcendence."

### Current State

- `TypingContext` has 8 typing modes defined but unused
- `Skills` has 5 complete trees with 25+ effects but nothing applies them

### Wiring Steps

#### Step 1: Apply Skill Effects to Damage

```rust
// src/game/combat_engine.rs
pub fn calculate_damage(
    base_damage: i32,
    wpm: f64,
    accuracy: f32,
    combo: i32,
    skills: &SkillTree,
) -> i32 {
    let mut damage = base_damage as f64;
    
    // Precision tree
    if skills.has_skill("precision_strike") {
        damage *= 1.0 + (accuracy as f64 - 0.9) * 2.0; // Bonus for >90% accuracy
    }
    if skills.has_skill("perfect_word_bonus") && accuracy == 1.0 {
        damage *= 1.5;
    }
    
    // Speed tree
    if skills.has_skill("velocity_stacking") {
        damage *= 1.0 + (wpm / 100.0) * 0.5; // 50% bonus at 100 WPM
    }
    if skills.has_skill("transcendence") && wpm >= 100.0 {
        damage *= 2.0; // Double damage at transcendent speed
    }
    
    // Shadow tree
    if skills.has_skill("critical_strike") {
        if rand::random::<f32>() < 0.15 {
            damage *= 2.0; // 15% crit chance
        }
    }
    
    // Combo multiplier (existing)
    damage *= 1.0 + (combo as f64 * 0.1).min(2.0);
    
    damage as i32
}
```

#### Step 2: Environmental Typing Contexts

```rust
// src/game/combat.rs
use crate::game::typing_context::{CombatEnvironment, TypingContext};

impl CombatState {
    pub fn apply_environment(&mut self, env: CombatEnvironment) {
        match env {
            CombatEnvironment::Rain => {
                // Words visually "drip" - characters shift down over time
                self.visual_effect = VisualEffect::Dripping;
            }
            CombatEnvironment::Darkness => {
                // Only 3 characters visible at a time
                self.visible_chars = 3;
            }
            CombatEnvironment::Wind => {
                // Words drift left/right
                self.visual_effect = VisualEffect::Drifting;
            }
            CombatEnvironment::Corrupted => {
                // Random letters replaced with symbols
                self.corruption_rate = 0.1;
            }
            CombatEnvironment::Library => {
                // Backspace makes noise - attracts enemies
                self.backspace_penalty = true;
            }
        }
    }
}
```

#### Step 3: Skill Tree UI

```rust
// src/ui/render.rs
fn render_skill_tree(frame: &mut Frame, area: Rect, tree: &SkillTree) {
    // Show 5 trees as tabs
    let tabs = Tabs::new(vec!["⚔️ Precision", "⚡ Speed", "🛡️ Endurance", "📚 Wisdom", "🌑 Shadow"])
        .select(tree.selected_tree as usize);
    
    // Show skills as a tree with connections
    for skill in tree.get_current_tree_skills() {
        let style = if skill.unlocked {
            Style::default().fg(Color::Green)
        } else if tree.can_unlock(&skill.id) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        
        // Render skill with lore tooltip on hover
    }
}
```

---

## Phase 4: Meaningful Dialogue (VoiceSystem)

**Priority: HIGH**  
**Estimated Effort: 3-4 hours**  
**Impact: NPCs feel alive, world feels coherent**

### Why?

Right now dialogue is generic. With `VoiceSystem`, a Scribe speaks differently than a Mechanist. Their metaphors, vocabulary, and speech patterns reflect their faction's worldview.

### Current State

`VoiceSystem` has complete faction voices with 5+ vocabulary sets, pattern templates, and tone modifiers - but nothing uses them.

### Wiring Steps

#### Step 1: Filter Dialogue Through Faction Voice

```rust
// src/game/voice_system.rs
impl VoiceSystem {
    /// Transform generic text into faction-flavored speech
    pub fn speak(&self, faction: Faction, context: DialogueContext, base_text: &str) -> String {
        let voice = self.get_faction_voice(faction);
        
        // Replace generic words with faction vocabulary
        let mut text = base_text.to_string();
        for (generic, specific) in &voice.vocabulary.substitutions {
            text = text.replace(generic, specific);
        }
        
        // Apply speech patterns
        text = voice.apply_pattern(context, &text);
        
        // Add faction-specific interjections
        if rand::random::<f32>() < 0.3 {
            text = format!("{} {}", voice.random_interjection(), text);
        }
        
        text
    }
}

// Example transformations:
// Scribe: "You should rest" → "Perhaps a moment of reflection would serve you well"
// Mechanist: "You should rest" → "Efficiency requires downtime. Recharge."
// Naturalist: "You should rest" → "Even the mightiest oak must drink from still waters"
```

#### Step 2: Character Personalities from Seed

```rust
// src/game/characters.rs
impl NPC {
    pub fn from_seed(name: &str, seed: &NarrativeSeed) -> Self {
        let char_seed = seed.get_character_seed(name);
        
        Self {
            name: name.to_string(),
            faction: char_seed.faction_alignment,
            personality: char_seed.typing_personality,
            hidden_agenda: char_seed.hidden_agenda,
            betrayal_likelihood: char_seed.betrayal_likelihood,
            crisis_stance: char_seed.crisis_stance,
            dark_secret: char_seed.dark_secret,
        }
    }
    
    pub fn get_greeting(&self, voice_system: &VoiceSystem, player_rep: i32) -> String {
        let base = if player_rep > 50 {
            "Welcome, friend"
        } else if player_rep < -50 {
            "What do you want"
        } else {
            "Greetings, traveler"
        };
        
        voice_system.speak(self.faction, DialogueContext::Greeting, base)
    }
}
```

#### Step 3: Dialogue Trees with Consequences

```rust
// src/game/dialogue.rs
pub struct DialogueNode {
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
    pub typing_challenge: Option<TypingChallenge>,
}

pub struct DialogueChoice {
    pub text: String,
    pub requires: Option<Requirement>,
    pub consequences: Vec<Consequence>,
    pub next_node: Option<String>,
}

pub enum Consequence {
    FactionRep { faction: Faction, delta: i32 },
    NPCOpinion { npc: String, delta: i32 },
    RevealLore(String),
    UnlockQuest(String),
    GiveItem(String),
    StartCombat(String),
}

// Typing challenges in dialogue - typos have social costs
pub struct TypingChallenge {
    pub prompt: String,
    pub difficulty: f32,
    pub typo_consequence: TypoConsequence,
}

pub enum TypoConsequence {
    SeemsStupid,      // NPC thinks less of you
    SeemsDeceptive,   // NPC becomes suspicious
    AngersNPC,        // NPC becomes hostile
    SeemsNervous,     // NPC notices your hesitation
}
```

---

## Phase 5: Exploration Depth (EncounterWriting + World)

**Priority: MEDIUM**  
**Estimated Effort: 4-5 hours**  
**Impact: World feels lived-in, exploration is rewarding**

### Why?

Right now rooms are "combat" or "shop." With authored encounters, you find "The Retired Scribe" who hints at the Unwriting, or "A Stranger Arrives" who might be corrupted.

### Current State

- `EncounterWriting` has fully authored encounters with requirements, choices, consequences
- `World` has locations, factions, and zone definitions
- Neither connects to dungeon generation

### Wiring Steps

#### Step 1: Spawn Authored Encounters

```rust
// src/game/dungeon.rs
use crate::game::encounter_writing::{AuthoredEncounter, EncounterLibrary};

impl Dungeon {
    pub fn generate_room(&self, floor: u32, state: &GameState) -> Room {
        // 30% chance of authored encounter if requirements met
        if rand::random::<f32>() < 0.3 {
            if let Some(encounter) = EncounterLibrary::find_valid_encounter(
                floor,
                &state.faction_relations,
                &state.discovered_lore,
                state.narrative_seed.as_ref(),
            ) {
                return Room::AuthoredEncounter(encounter);
            }
        }
        
        // Fall back to procedural room
        self.generate_procedural_room(floor)
    }
}
```

#### Step 2: Environmental Storytelling

```rust
// src/game/world_integration.rs
impl Zone {
    pub fn get_environmental_details(&self, floor: u32) -> Vec<String> {
        match self {
            Zone::ShatteredHalls => vec![
                "Broken throne fragments litter the floor.",
                "A faded banner shows the crest of fallen Valdris.",
                "Scorch marks on the walls speak of the Sundering.",
            ],
            Zone::SunkenArchives => vec![
                "Water drips onto moldering tomes.",
                "A preserved scroll reads: 'Malachar sought the Veil...'",
                "Ghostly whispers echo forbidden knowledge.",
            ],
            // ... other zones
        }
    }
    
    pub fn get_typing_words(&self) -> Vec<String> {
        // Already implemented in lore_words.rs!
        LoreWords::get_zone_words(self)
    }
}
```

#### Step 3: Interactive World Elements

```rust
// src/game/exploration.rs
pub enum InteractiveElement {
    LoreObject {
        name: String,
        description: String,
        typing_challenge: Option<String>,  // Type to decipher
        lore_fragment: String,
    },
    HiddenCache {
        typing_challenge: String,  // Type to unlock
        contents: Vec<Item>,
    },
    CorruptedTerminal {
        requires_purify: bool,
        faction_data: Option<String>,
    },
    GhostlyEcho {
        memory_fragment: String,
        speaker: String,  // Who said this before they died
    },
}

impl InteractiveElement {
    pub fn interact(&self, state: &mut GameState) -> InteractionResult {
        match self {
            Self::LoreObject { typing_challenge: Some(challenge), lore_fragment, .. } => {
                // Player must type to decipher
                state.start_typing_challenge(challenge);
                state.pending_lore = Some(lore_fragment.clone());
                InteractionResult::TypingChallenge
            }
            Self::GhostlyEcho { memory_fragment, speaker } => {
                // Reveals story without combat
                state.show_lore_popup(speaker, memory_fragment);
                InteractionResult::LoreDiscovered
            }
            // ...
        }
    }
}
```

---

## Phase 6: Challenge Scaling (RunModifiers)

**Priority: MEDIUM**  
**Estimated Effort: 2-3 hours**  
**Impact: Replayability, player agency over difficulty**

### Why?

`RunModifiers` is your Hades Pact of Punishment. Players choose harder challenges for better rewards.

### Wiring Steps

#### Step 1: Pre-Run Modifier Selection

```rust
// src/game/state.rs
impl GameState {
    pub fn show_modifier_select(&mut self) {
        self.scene = Scene::ModifierSelect;
        self.available_modifiers = RunModifiers::get_available(self.meta_progress.heat_unlocked);
    }
    
    pub fn apply_run_modifiers(&mut self, selected: Vec<Modifier>) {
        self.run_modifiers = RunModifiers::new();
        for modifier in selected {
            self.run_modifiers.add_modifier(modifier);
        }
        
        // Apply to all systems
        self.apply_typing_modifiers();
        self.apply_combat_modifiers();
        self.apply_world_modifiers();
        
        // Set reward multiplier
        self.reward_multiplier = self.run_modifiers.reward_multiplier;
    }
}
```

#### Step 2: Typing Modifiers

```rust
// Effect implementations
fn apply_typing_modifiers(&mut self) {
    if self.run_modifiers.has_modifier(Modifier::InvisibleLetters) {
        // Random letters don't render
        self.invisible_letter_rate = 0.2;
    }
    if self.run_modifiers.has_modifier(Modifier::ShiftingText) {
        // Words move while typing
        self.text_shift_speed = 1.0;
    }
    if self.run_modifiers.has_modifier(Modifier::NoBackspace) {
        // Mistakes are permanent
        self.backspace_disabled = true;
    }
    if self.run_modifiers.has_modifier(Modifier::Metronome) {
        // Must type in rhythm
        self.metronome_enabled = true;
        self.metronome_bpm = 60;
    }
}
```

---

## Code Architecture Improvements

### 1. Data-Driven Design

Move hardcoded values to configuration:

```rust
// src/data/config.toml (new file)
[combat]
base_time_limit = 5.0
combo_damage_multiplier = 0.1
max_combo_multiplier = 3.0

[typing]
perfect_word_threshold = 1.0
flow_state_threshold = 0.95

[progression]
xp_per_level = 100
level_scaling = 1.5
```

```rust
// src/game/config.rs
#[derive(Deserialize)]
pub struct GameConfig {
    pub combat: CombatConfig,
    pub typing: TypingConfig,
    pub progression: ProgressionConfig,
}

impl GameConfig {
    pub fn load() -> Self {
        // Load from embedded or external TOML
    }
}
```

### 2. System Trait for Clean Interfaces

```rust
// src/game/systems.rs
pub trait GameSystem {
    /// Called every frame
    fn update(&mut self, delta: f32, state: &mut GameState);
    
    /// Called when an event occurs
    fn on_event(&mut self, event: &GameEvent, state: &mut GameState);
    
    /// Returns events this system wants to emit
    fn drain_events(&mut self) -> Vec<GameEvent>;
}

// Implement for each system
impl GameSystem for CombatState { ... }
impl GameSystem for FactionRelations { ... }
impl GameSystem for SkillTree { ... }
```

### 3. Resource Manager for Shared Data

```rust
// src/game/resources.rs
pub struct Resources {
    pub game_data: Arc<GameData>,
    pub voice_system: Arc<VoiceSystem>,
    pub encounter_library: Arc<EncounterLibrary>,
    pub config: Arc<GameConfig>,
}

// Pass to systems instead of individual references
impl GameState {
    pub fn new(resources: Resources) -> Self { ... }
}
```

---

## Writing Direction: Inspirations

For the dialogue and narrative quality you want, study these:

### Tone & Voice
- **Disco Elysium** - Every line reveals character, subtext everywhere
- **Planescape: Torment** - Philosophy embedded in dialogue
- **Hades** - Warmth, wit, and world-building in every interaction

### Structure
- **Dark Souls** - Environmental storytelling, fragmented lore
- **Hollow Knight** - Melancholy beauty, ancient mysteries
- **Sunless Sea** - Prose that drips atmosphere

### Typing Integration
- **Undertale** - Mechanics reflect narrative (MERCY, FIGHT)
- **Papers Please** - Typing as moral weight
- **Return of the Obra Dinn** - Deduction through interaction

### Key Principles

1. **Every line does double duty** - Reveals character AND advances world-building
2. **Subtext over text** - What's NOT said matters more
3. **Consistent voice** - Factions speak differently, always
4. **Earned reveals** - Lore fragments reward attention
5. **Typed words matter** - What you type reflects who you are

---

## Implementation Priority

| Phase | System | Effort | Impact | Dependencies |
|-------|--------|--------|--------|--------------|
| 1 | EventBus | 2-3h | Critical | None |
| 2 | NarrativeSeed | 2-3h | High | EventBus |
| 3a | Skills | 3-4h | High | EventBus |
| 3b | TypingContext | 3-4h | High | EventBus |
| 4 | VoiceSystem | 3-4h | High | NarrativeSeed |
| 5 | EncounterWriting | 4-5h | Medium | VoiceSystem |
| 6 | RunModifiers | 2-3h | Medium | EventBus |

**Total estimated effort: 20-26 hours**

---

## Next Steps

Choose your path:

### A. Task Breakdown for GitHub Issues
I'll create ~15-20 specific, actionable GitHub issues with acceptance criteria.

### B. Technical Refactor Checklist  
I'll create a PR-ready checklist for each phase with specific file changes.

### C. Start Implementing Phase 1
I'll wire up the EventBus right now and show you the pattern for other systems.

---

*"The systems exist. The lore exists. The world exists. They just don't know about each other yet."*
