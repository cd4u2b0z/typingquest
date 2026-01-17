zr# Changelog

All notable changes to TypingQuest will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Save/load game state
- Wire up dormant narrative systems
- Balance pass (once things are connected)

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

- [Repository](https://github.com/cd4u2b0z/typingquest)
- [README](README.md)
- [License](LICENSE)
