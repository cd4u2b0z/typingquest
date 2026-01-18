# Changelog

All notable changes to **Keyboard Warrior** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Save/load persistence
- Ink currency persistence between runs
- Balance tuning pass

---

## [0.5.1] - 2026-01-18

### 🎭 Immersion Overhaul

**Every Keystroke Tells a Story** — 2,500+ lines of new immersion systems that make combat feel visceral and alive.

### Added - Typing Impact System
- **Per-keystroke damage tracking**: Each keypress contributes incremental damage
- **Rhythm detection**: Fast, consistent typing triggers "Flurry" attacks
- **Attack types**: Light Jab, Quick Strike, Solid Hit, Power Strike, Critical Blow
- **Visual intensity scaling**: Damage feedback correlates with typing speed

### Added - Dialogue Engine
- **Context-aware enemy dialogue**: Enemies react based on their health, your combo, combat phase
- **6 personality themes**: Goblin (crude), Undead (nihilistic), Spectral (cryptic), etc.
- **Dynamic taunts and death rattles**: "Your words cut deeper than steel..."
- **Player momentum tracking**: Dominant/Confident/Struggling states affect dialogue

### Added - Enemy Visual States
- **Progressive ASCII damage**: Enemy art deteriorates as they take damage
- **5 posture states**: Confident -> Wary -> Staggered -> Wounded -> Dying
- **Wound overlays**: Slash marks, blood particles appear on enemy art
- **Health-synced animations**: Visual shifts at 75%, 50%, 25%, 10% HP

### Added - Pacing System
- **Tension/breather beats**: Combat flow alternates intensity naturally
- **Boss tension multipliers**: Boss fights maintain higher baseline tension
- **Atmospheric messages**: "The air grows thick..." during tension peaks
- **Recovery windows**: Brief pauses after defeating enemies

### Added - Player Avatar
- **4 class-based avatars**: Wordsmith, Chronicler, Codebreaker, Freelancer
- **Combat animation states**: Idle, Attacking, Hit, Victory, Defeat
- **Wounded variations**: Avatar art changes when HP is low
- **Visual presence**: Player now has on-screen representation

### Added - Combat Immersion Wrapper
- **ImmersiveCombat struct**: Unified API integrating all subsystems
- **Enemy theme inference**: Auto-detects goblin/undead/spectral from name
- **Read-only rendering**: Supports immutable render contexts
- **Optional integration**: Graceful degradation if not initialized

### Changed
- CombatState gains optional immersive field
- init_immersion(player_class) one-call setup
- Helper methods for keystroke/word/render feedback
- Maps game Class to avatar PlayerClass

---

## [0.5.0] - 2026-01-18

### 🔗 Deep System Integration

**The Great Awakening** — ~4,000+ lines of dormant code now actively wired into gameplay. Six major systems that existed in isolation now communicate through a unified event architecture.

### Added - EventBus System (Phase 1)
- **Central Event Architecture**: All game systems now communicate through a unified EventBus
- **60+ Event Types**: PlayerDamaged, EnemyDefeated, ItemAcquired, FloorChanged, SpellCast, ComboAchieved, etc.
- **Event Processing Pipeline**: `process_events()` and `handle_event()` in GameState
- Events trigger narrative responses, faction changes, and skill effects

### Added - NarrativeSeed & Corruption Effects (Phase 2)
- **Corruption-based Typing Modifiers**: Your narrative corruption now affects combat typing
  - Temporal Stutter: Words echo (h-h-hello w-w-world)
  - Void Whispers: Spaces become underscores (hello_world)
  - Reality Fracture: Characters scatter (hlelo wrold)
  - Memory Decay: Vowels fade (h.ll. w.rld)
  - Name Erosion: Corrupted player name echoes
- **Active Typing Modifier Field**: GameState tracks current corruption effect
- Combat word generation passes corruption state to typing engine

### Added - SkillTree Combat Integration (Phase 3)
- **10+ Helper Methods** on SkillTree:
  - `get_active_effects()` — returns all unlocked skill effects
  - `get_damage_multiplier()` — combines all damage bonuses
  - `get_crit_chance()` / `get_crit_multiplier()` — critical hit mechanics
  - `get_damage_reduction()` / `get_evasion_chance()` — defensive stats
  - `is_transcendent()` — checks for transcendence state
  - `get_max_hp_multiplier()` / `get_xp_multiplier()` — scaling bonuses
- **CombatState Skill Integration**:
  - Constructor accepts skill tree reference
  - Computes and stores skill modifiers at combat start
  - `calculate_damage()` applies multipliers, transcendence bonuses, critical hits
  - Enemy attacks check evasion chance and apply damage reduction

### Added - VoiceSystem for NPC Dialogue (Phase 4)
- **Faction-based Dialogue Generation**: NPCs speak according to their faction personality
- **5 Faction Voices** (794 lines):
  - Mages Guild: Scholarly, arcane terminology
  - Temple of Dawn: Reverent, light metaphors
  - Rangers of the Wild: Nature-focused, practical
  - Shadow Guild: Cryptic, information-as-currency
  - Merchant Consortium: Transactional, pragmatic
- **Shop Integration**: Merchant Consortium greeting when entering shops
- **Rest Site Integration**: Temple of Dawn healer dialogue at rest sites
- **Dynamic Dialogue**: `generate_npc_dialogue()` and `get_merchant_greeting()` methods

### Added - EncounterWriting for Authored Events (Phase 5)
- **783 Lines of Encounters** now triggerable during exploration
- **Encounter Tracking**: Prevents repeat encounters via `encounter_tracker` HashMap
- **Trigger Conditions**: Floor requirements, faction reputation, item possession
- **Consequence System**: `resolve_encounter()` applies:
  - Gold rewards/costs
  - Health changes
  - Item grants
  - Faction reputation shifts
- **`try_trigger_encounter()`**: Scans for valid encounters each floor

### Added - RunModifiers for Difficulty Scaling (Phase 6)
- **50+ Modifiers** (632 lines) now affect gameplay
- **Enemy Scaling Methods**:
  - `get_enemy_health_multiplier()` — scales enemy HP
  - `get_enemy_damage_multiplier()` — scales enemy damage
- **Reward Scaling**:
  - `get_gold_multiplier()` — affects all gold rewards (combat, treasure, encounters)
- **Run Configuration**:
  - `set_run_type()` — configure challenge modes
  - `get_heat_level()` — retrieve current difficulty tier
- **Applied to**: Enemy stats at combat start, gold from victories, treasure rewards

### Changed - Combat System
- CombatState now initializes with corruption and skill modifiers
- Damage calculation incorporates skill tree bonuses
- Critical hits roll against skill-based crit chance
- Transcendence state grants bonus damage at low HP
- Evasion rolls occur before damage application
- Damage reduction applies after evasion check fails

### Changed - Exploration Flow
- Entering shops triggers Merchant Consortium voice lines
- Rest sites feature Temple of Dawn healer dialogue
- Floor transitions check for authored encounters
- Gold rewards apply run modifier multipliers

### Technical Details
- **Files Modified**: state.rs, combat.rs, skills.rs
- **New Imports**: event_bus, narrative_seed, voice_system, encounter_writing, run_modifiers
- **New GameState Fields**: event_bus, narrative_seed, active_typing_modifier, skill_tree, faction_voices, current_npc_dialogue, encounters, encounter_tracker, current_encounter, run_modifiers
- **Build Status**: Release build passing

### Impact
- Game systems now react to player actions dynamically
- Combat feels more responsive with skill-based modifiers
- NPCs have personality through faction-specific dialogue
- Exploration has authored narrative moments
- Difficulty scaling works for challenge runs

---

## [0.4.1] - 2026-01-17

### 🎮 Lore-Integrated Typing System

The words you type during combat are no longer random - they are woven into the narrative!

### Added - Lore Words Module (src/data/lore_words.rs)
- **Zone-Specific Word Pools**: Each zone has thematic words
  - Shattered Halls: throne, crown, knight, oath, valdris, sundering
  - Sunken Archives: scroll, tome, codex, wisdom, forbidden, ritual
  - Blighted Gardens: blight, rot, decay, purify, cleanse, restore
  - Clockwork Depths: gear, cog, mechanism, sentinel, guardian
  - Voids Edge: void, nothing, breach, archon, herald, doom
  - The Breach: seal, hero, destiny, salvation, redemption

### Added - Zone-Specific Sentences
- Each area has ~10 unique sentences that tell the story as you fight:
  - "The throne sits empty, but the oaths still bind."
  - "Malachar studied here before his fall from grace."
  - "The Elder Stones pulse with power that predates creation."

### Added - Enemy-Specific Word Themes
- Goblin enemies use greedy words: shiny, mine, steal, hoard
- Undead enemies use hollow words: oath, duty, eternal, forgotten
- Spectral enemies use ethereal words: wisp, shimmer, essence, regret
- Corrupted enemies use twisted words: blight, wrong, torment, purify
- Mechanical enemies use precise words: gear, protocol, directive
- Void enemies use cosmic words: nothing, emptiness, oblivion, silence

### Added - Boss-Specific Dialogue
- The Hollow Knight: "I am the last defender of a kingdom that no longer exists."
- The Void Herald: "I speak with the voice of endings. Listen, and despair."

### Added - Narrative Progression
- Early game sentences establish the world
- Mid game reveals the truth about Malachar
- Late game builds to the final confrontation

### Changed - Combat System
- CombatState now tracks floor number
- Word/sentence generation uses lore-appropriate content
- Boss battles feature unique dialogue from that boss

### Impact
- Every word typed now connects to the lore
- Players learn the story through gameplay
- Typing becomes an act of narrative discovery

---

## [0.4.0] - 2026-01-17

### 🏰 Fantasy Lore Overhaul

Complete rewrite of all lore systems from keyboard/typing theme to **high fantasy D&D-style** worldbuilding. Inspired by Tolkien, Elder Scrolls, Dark Souls, and Forgotten Realms.

### Changed - World Cosmology
- **Three Ages**: Age of Dawn (gods walked among mortals), Age of Crowns (mortal kingdoms), Age of Shadow (current era)
- **The Sundering**: Central catastrophe when Archon Malachar tried to pierce the Veil and become a god
- **The Blight**: Raw chaos leaking from the Void through the wound in reality
- **Player Mystery**: You are Malachar reborn, seeking redemption or dark ascension

### Changed - Factions (complete rename)
- Scribes → **Mages Guild** (arcane knowledge seekers)
- Mechanists → **Temple of Dawn** (divine light, healing, undead hunters)
- Naturalists → **Rangers of the Wild** (nature's defenders, Blight fighters)
- ShadowWriters → **Shadow Guild** (information brokers, assassins)
- Archivists → **Merchant Consortium** (trade, neutrality, pragmatism)

### Changed - Dungeon Zones
- **Floors 1-2**: The Shattered Halls (ruined castle)
- **Floors 3-4**: The Sunken Archives (flooded library)
- **Floors 5-6**: The Blighted Gardens (corrupted greenhouse)
- **Floors 7-8**: The Clockwork Depths (ancient machinery)
- **Floors 9-10**: The Void's Edge (reality breaks down)
- **Floor 11+**: The Breach (where the Sundering occurred)

### Changed - Bosses
- "THE GREAT QWERTY" → **The Hollow Knight** (Floor 5)
- "CLIPPY THE FALLEN" → **The Void Herald** (Floor 10)

### Changed - Enemies (14 renamed with new descriptions)
- Typo Gremlin → **Goblin Lurker**
- Word Wisp → **Spectral Wisp**
- Syntax Spider → **Venomous Spider**
- Vowel Vampire → **Lesser Vampire**
- Corrupted Typer → **Blighted Thrall**
- Meaning Eater → **Soul Devourer**
- Grammar Golem → **Stone Golem**
- Void Scribe → **Void Walker**
- Entropy Weaver → **Shadow Weaver**
- Paragraph Phantom → **Wailing Wraith**
- Lexicon Leviathan → **Ancient Wyrm**
- Silence Incarnate → **Death Knight**
- Corruption Elemental → **Blight Elemental**
- The Unwriter → **The Void Herald**

### Changed - Lore References
- "Unwriting" → "Sundering" throughout codebase
- All typing/keyboard metaphors replaced with fantasy equivalents
- Updated ambient messages, event text, and zone descriptions

### Changed - Three Endings
- **Final Rest**: Seal the breach, sacrifice yourself, world saved
- **Dark Ascension**: Complete Malachar's ritual, become the new dark god
- **Third Path**: Find another way (requires all faction alliances)

### Technical
- `deep_lore.rs`: 853 → 1,016 lines (complete rewrite)
- Updated faction references in 12+ files
- All changes compile cleanly (`cargo check` passes)

### Documentation
- README project structure now lists all 48 source files
- Accurate line counts for all modules
- New roadmap section for v0.4.0 lore overhaul

---

## [0.3.0] - 2026-01-17

### Major Gameplay Integration

This release focuses on **actually connecting systems** that existed but were dormant. The game went from "vibes-based scaffolding" to "playable loop with real mechanics."

### Added
- **Spell Casting System**
  - Press `Tab` to toggle spell mode during combat
  - Press `1-9` to select a spell from your spellbook
  - Type the spell's incantation to cast it
  - Spells now actually deal damage and cost MP

- **Faction Reputation**
  - Five factions now tracked in game state: Silent Order, Echoing Choir, Merchant Guild, Wandering Wardens, Void Touched
  - Faction standings displayed in Stats screen (`s`)
  - Event outcomes can now modify faction reputation

- **Meta-Progression (Partial)**
  - Ink currency earned on death (10 base + bonuses for floors/enemies)
  - Ink total displayed on Game Over screen
  - Note: Ink doesn't persist yet (no save/load)

### Fixed
- **Floor 5 Tutorial Hardlock** — end_rest() now properly advances floors when floor_complete is true
- **Combat Feel Disconnected** — typing_feel system now receives keystrokes and word completions
- **Combos Felt Weak** — increased combo multiplier from 5% to 10% per combo (max 3x at 20 combo)

### Changed
- Combat help text updated to show spell controls
- Game Over screen shows Ink earned and breakdown

### Honest Assessment
~15,000 lines of code remain dormant:
- deep_lore.rs, narrative_seed.rs, voice_system.rs, skills.rs, typing_context.rs, encounter_writing.rs, run_modifiers.rs

These systems compile. They have cool designs. They just... aren't connected to anything yet. This is a hobby project, not a product.

---

## [0.2.1] - 2026-01-17

### Added
- **Phase 4: Visual Identity**
  - New theme system (`theme.rs`, ~350 lines)
  - Semantic color palette (PRIMARY, SECONDARY, ACCENT, SUCCESS, WARNING, DANGER)
  - 40+ Nerd Font icons for UI elements
  - Border style presets (SINGLE, DOUBLE, ROUNDED, HEAVY)
  - Style helpers: `hp_color()`, `combo_color()`, `wpm_color()`, `accuracy_color()`

### Changed
- Title screen enhanced with rounded borders and decorative elements
- All panel titles now include themed icons
- Menu items display with contextual icons
- Game over and victory screens polished with icons and styled buttons
- Consistent color theming applied to all 11 screens

### Fixed
- Help key (`h`) no longer triggers during combat typing
- Dungeon navigation hint now clearly shows `[Enter/e] EXPLORE`
- Combat help hint changed from `[h]` to `[?]` for consistency

---

## [0.2.0] - 2026-01-17

### Added
- **Phase 1: Help System** (`help_system.rs`, ~750 lines)
  - Four-tab help overlay: Contextual, Keybindings, Objectives, Mechanics
  - Context-aware tips that change based on current scene
  - Tip priority system (Essential, Important, Advanced, Secret)
  - Scrollable content with keyboard navigation
  - Hint manager for timed contextual messages

- **Phase 2: Tutorial System** (`tutorial.rs`, ~650 lines)
  - Five-phase interactive tutorial
  - Phases: Awakening, First Strike, The Combo, Choice, Discovery
  - Narrative-integrated teaching (not just instructions)
  - Progress tracking with visual feedback
  - Skippable steps for returning players

- **Phase 3: Game Feel** (`typing_feel.rs`, ~550 lines)
  - Flow states: Building → Flowing → Transcendent
  - Combo system with damage multipliers (up to 3x)
  - Rhythm detection analyzing keystroke cadence
  - WPM and accuracy tracking with visual display
  - Screen shake trigger on enemy attacks
  - Color flash and visual feedback systems

- **Narrative Systems** (multiple files, ~4000 lines)
  - Deep lore system with world cosmology
  - Five factions: Silent Order, Echoing Choir, Merchants, Wardens, Void Touched
  - Lore fragment discovery with rarity tiers
  - Authored NPC encounters with distinct voices
  - Mystery progression framework (5 tiers)
  - Chapter-based story progression

- **Meta-Progression Foundation** (`meta_progression.rs`, ~650 lines)
  - Ink currency system (framework)
  - Unlock tree structure
  - Achievement definitions
  - NPC bond tracking

### Changed
- Replaced all emojis with Nerd Font glyphs throughout codebase
- Key hints added to all major screens
- Combat typing feedback improved with clearer visual states

### Technical
- Codebase expanded to ~21,500 lines
- 35+ source files with clean module organization
- All systems compile cleanly with no warnings

---

## [0.1.0] - 2026-01-16

### Added
- **Core Game Loop**
  - Scene-based state management (Title, ClassSelect, Dungeon, Combat, etc.)
  - Main game loop with 50ms tick rate
  - Clean terminal setup/teardown with crossterm

- **Character System**
  - 5 playable classes: Wordsmith, Scribe, Spellweaver, Barbarian, Trickster
  - Stats: Strength, Intellect, Vitality, Dexterity, Luck
  - Level progression with XP requirements
  - HP/MP management

- **Combat System**
  - Typing-based attacks with real-time input
  - Combo tracking for bonus damage
  - Time pressure mechanic
  - Enemy turn after word completion or timeout
  - Victory/defeat conditions

- **Dungeon System**
  - 10-floor progression
  - Procedural room generation
  - Room types: Combat, Elite, Boss, Treasure, Shop, Rest, Event
  - Floor difficulty scaling

- **Item System**
  - Equipment slots (weapon, armor, accessory)
  - Consumable items (potions, scrolls)
  - Relics with passive effects
  - Rarity tiers: Common, Uncommon, Rare, Epic, Legendary

- **Spell System**
  - Elemental spells (Fire, Ice, Lightning, etc.)
  - MP costs and cooldowns
  - Damage calculation based on Intellect

- **Event System**
  - Random encounters with branching choices
  - Consequences affecting player state
  - ASCII art for event scenes

- **Shop System**
  - Purchasable items with gold currency
  - Dynamic inventory

- **Rest System**
  - HP restoration
  - MP restoration  
  - XP training option

- **UI Rendering**
  - Full TUI with ratatui
  - 11 distinct screens
  - HP/MP gauges
  - Battle log
  - ASCII art for enemies and events

### Technical
- Rust 2021 edition
- Dependencies: ratatui 0.28, crossterm 0.28, serde, rand, better_panic
- ~8,000 lines of Rust
- Release binary: ~1.4MB (optimized + stripped)

---

## Version Scheme

This project uses [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.x.x): Breaking changes or feature-complete release
- **MINOR** (0.x.0): New features, systems, or significant additions
- **PATCH** (0.0.x): Bug fixes, polish, and refinements

Current status: **Pre-release** (0.x.x) — Core systems functional, content and polish ongoing.

---

## Links

- [Repository](https://github.com/cd4u2b0z/keyboard-warrior)
- [README](README.md)
- [License](LICENSE)
