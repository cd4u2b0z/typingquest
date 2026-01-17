# 󰓥 TypingQuest

**A narrative roguelike typing RPG — discover ancient mysteries through the rhythm of your keystrokes.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.2.0-blue?style=flat)]()
[![Lines of Code](https://img.shields.io/badge/Lines-20k+-yellow?style=flat)]()
[![TUI](https://img.shields.io/badge/TUI-ratatui-purple?style=flat)](https://github.com/ratatui-org/ratatui)

TypingQuest combines the satisfying mechanics of typing trainers like [ttyper](https://github.com/max-niederman/ttyper) with deep RPG progression inspired by *Undertale*, *Earthbound*, *Balatro*, *Hades*, and classic roguelikes.

```
╔══════════════════════════════════════════════════════════════════════════╗
║  TypingQuest v0.2.0                          Chapter: Discovery          ║
║  ═══════════════════════════════════════════════════════════════════════ ║
║                                                                          ║
║   ╭───────────╮                    ╭───────────╮                         ║
║   │    YOU    │      ⚔️  vs ⚔️      │   󰚌 ELITE │                         ║
║   │  Wordsmith│                    │Silent Warden│                       ║
║   │ HP ████░░ │                    │ HP ██████░░ │                        ║
║   │ MP ██████ │                    │             │                        ║
║   ╰───────────╯                    ╰───────────╯                         ║
║                                                                          ║
║   ┌────────────────────────────────────────────────────────────────┐    ║
║   │  "The threshold remembers all who cross it."                   │    ║
║   │                                                                │    ║
║   │  Type: "incantation of binding"                                │    ║
║   │  >     incantation of b_                                       │    ║
║   └────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║   ╭─ FLOW ─────────╮  ╭─ COMBO ────────╮  ╭─ STATS ────────────────╮    ║
║   │ ✨ TRANSCENDENT │  │ 󰈸 12x STREAK  │  │ WPM: 94  ACC: 98%      │    ║
║   │ Crit +30%      │  │ DMG: 3.0x      │  │ 󰐀 Ink: 847            │    ║
║   ╰────────────────╯  ╰────────────────╯  ╰────────────────────────╯    ║
║                                                                          ║
║   [Silent Order: ████████░░ Ally]  [Mystery: ██████░░ Tier 3/5]         ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

## 󰧮 Table of Contents

- [Features](#-features)
- [Requirements](#-requirements)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [How to Play](#-how-to-play)
- [Controls](#-controls)
- [Classes](#-classes)
- [Factions](#-factions)
- [Architecture](#-architecture)
- [Configuration](#-configuration)
- [Roadmap](#-roadmap)
- [License](#-license)
- [Credits](#-credits)

---

## 󰓎 Features

### 󰴓 Deep Narrative Systems *(NEW in 0.2.0)*

| Feature | Description |
|---------|-------------|
| 󰂺 Lore Discovery | Uncover fragments of ancient history as you type |
| 󰒖 Five Factions | Silent Order, Echoing Choir, Merchants, Wardens, Void Touched |
| 󰛓 Mystery Progression | Five-tier revelation system with multiple endings |
| 󰘬 Character Bonds | Build relationships through repeated encounters |
| 󰖟 World State | Your choices reshape faction territories and alliances |
| 󰝚 Voiced Dialogue | 15+ authored NPC encounters with distinct personalities |

### 󰌌 Typing Feel Engine *(NEW in 0.2.0)*

| Feature | Description |
|---------|-------------|
| 󰒔 Flow States | Building → Flowing → Transcendent |
| 󰈸 Combo System | Chain words for up to 3x damage multiplier |
| 󰔊 Rhythm Detection | Cadence analysis affects critical hit chance |
| 󰋖 Visual Feedback | Screen shake, color flash, text ripple effects |
| 󰌓 Keystroke Feel | Every key press feels impactful and satisfying |

### 󰆼 Meta-Progression *(NEW in 0.2.0)*

| Feature | Description |
|---------|-------------|
| 󰐀 Ink Currency | Earn persistent currency from every run |
| 󰓦 Unlock Tree | HP bonus, damage bonus, word preview, map reveal |
| 󰄀 Lore Codex | Collected lore persists across deaths |
| 󰘆 Achievements | Speed Demon, Perfectionist, True Ending, 10+ more |
| 󰈅 Heat System | Hades-style difficulty modifiers for extra rewards |
| 󰖟 NPC Bonds | Relationships deepen across multiple runs |

### Core Features

| Feature | Description |
|---------|-------------|
| 󰌌 Type-to-Attack | Your WPM is your weapon. Type words to deal damage |
| 󰔊 Combo System | Build combos for multiplied damage |
| 󰆥 5 Classes | Wordsmith, Scribe, Spellweaver, Barbarian, Trickster |
| 󰙅 Roguelike | Procedural dungeons, permadeath tension |
| 󰒓 Adaptive Difficulty | Game adjusts to your typing skill |
| 󰆼 Deep Progression | Level up, learn skills, collect items |

### Room Types

| Type | Icon | Description |
|------|------|-------------|
| Combat | 󰓥 | Standard enemy encounters |
| Elite | 󰚌 | Harder enemies, better rewards |
| Boss | 󰮇 | Floor boss battles |
| Treasure | 󰆧 | Free items and gold |
| Shop | 󰒍 | Buy equipment and consumables |
| Rest | 󰒲 | Heal, train, or meditate |
| Event | 󰋗 | Random encounters with choices |

### Combat Mechanics

- **Speed Bonus** — Type faster for bonus damage
- **Perfect Words** — No backspaces = damage multiplier
- **Combo Streaks** — Chain words for escalating damage
- **Accuracy Tracking** — Mistyped characters reduce effectiveness

---

## 󰏖 Requirements

### System Requirements

| Requirement | Value |
|-------------|-------|
| OS | Linux, macOS, Windows |
| Rust | 1.70+ |
| Terminal | Unicode support required |
| Display | Minimum 80x24 (120x40 recommended) |

### Dependencies

```toml
ratatui = "0.28"      # TUI framework
crossterm = "0.28"    # Terminal handling
serde = "1.0"         # Serialization
ron = "0.8"           # RON config format
rand = "0.8"          # RNG
better-panic = "0.3"  # Panic handling
```

---

## 󰔎 Quick Start

```bash
# Clone and run
git clone https://github.com/cd4u2b0z/typingquest.git
cd typingquest
cargo run --release

# Or install globally
cargo install --path .
typingquest
```

---

## 󰏗 Installation

### From Source

```bash
git clone https://github.com/cd4u2b0z/typingquest.git
cd typingquest
cargo build --release
./target/release/typingquest
```

### From crates.io (coming soon)

```bash
cargo install typingquest
```

---

## 󰊗 How to Play

1. **Select a class** — Each has unique abilities and playstyles
2. **Descend the dungeon** — 10 procedurally generated floors
3. **Type to fight** — Words appear, type them quickly and accurately
4. **Manage resources** — HP, MP, gold, and items
5. **Level up** — Gain XP, unlock skills, find equipment
6. **Defeat the boss** — Each floor ends with a boss battle

---

## 󰌌 Controls

| Key | Action |
|-----|--------|
| `a-z` | Type characters |
| `Backspace` | Delete character |
| `Enter` | Confirm selection |
| `Esc` | Cancel/Back |
| `j/k` | Navigate menus |
| `i` | Open inventory |
| `s` | View stats |
| `q` | Quit game |

---

## 󰆥 Classes

| Class | HP | MP | Specialty |
|-------|----|----|-----------|
| **Wordsmith** | 100 | 50 | Balanced fighter, +10% damage |
| **Scribe** | 80 | 80 | Double XP, starts with Analyze |
| **Spellweaver** | 70 | 100 | Magic focus, +20% spell damage |
| **Barbarian** | 150 | 20 | High HP, +30% crit chance |
| **Trickster** | 90 | 60 | Combo master, +50% combo bonus |

---

## 󰒖 Factions

Your choices shape your standing with the five factions that control the realm:

| Faction | Philosophy | Ally | Enemy |
|---------|------------|------|-------|
| 󰂵 **Silent Order** | Knowledge through observation | Wardens | Choir |
| 󰋾 **Echoing Choir** | Truth through prophecy | Void Touched | Silent Order |
| 󰆧 **Gilded Merchants** | Power through commerce | — | — |
| 󰛡 **Threshold Wardens** | Protection at any cost | Silent Order | Void Touched |
| 󰚌 **Void Touched** | Embrace dissolution | Choir | Wardens |

**Reputation Effects:**
- **Ally (50+)**: Exclusive encounters, discounts, quest access
- **Neutral (0)**: Standard interactions
- **Hostile (-50)**: Ambushes, closed doors, harder negotiations

Your faction standings influence which of the **12 endings** you can achieve.

---

## 󰙅 Architecture

```
typingquest/
├── Cargo.toml                    # Dependencies & metadata
├── README.md                     # This file
├── CHANGELOG.md                  # Version history
├── data/
│   ├── enemies.toml              # Enemy definitions
│   └── config.ron                # Game configuration
│
└── src/
    ├── main.rs                   # Entry point (533 lines)
    │
    ├── game/                     # 󰓎 CORE SYSTEMS (~15,000 lines)
    │   ├── mod.rs                # Module exports
    │   │
    │   │── # 🎭 Narrative Layer
    │   ├── deep_lore.rs          # Cosmology, endings (1,200 lines)
    │   ├── lore_fragments.rs     # Discoverable lore (900 lines)
    │   ├── encounter_writing.rs  # Authored encounters (1,000 lines)
    │   ├── writing_guidelines.rs # Literary standards (650 lines)
    │   ├── narrative_integration.rs # Engine coordinator (600 lines)
    │   ├── narrative_seed.rs     # Procedural narrative (900 lines)
    │   ├── faction_system.rs     # 5 factions (815 lines)
    │   ├── voice_system.rs       # NPC personalities (800 lines)
    │   ├── narrative.rs          # Base narrative (600 lines)
    │   │
    │   │── # ⌨️ Typing Systems
    │   ├── typing_feel.rs        # Flow & combos (450 lines)
    │   ├── typing_context.rs     # Context analysis (650 lines)
    │   ├── combat.rs             # Typing combat (370 lines)
    │   ├── combat_engine.rs      # Event-driven (420 lines)
    │   ├── combat_events.rs      # Combat events (200 lines)
    │   │
    │   │── # 🏆 Progression
    │   ├── meta_progression.rs   # Hades-style unlocks (650 lines)
    │   ├── run_modifiers.rs      # Heat system (630 lines)
    │   ├── stats.rs              # Achievements (450 lines)
    │   ├── skills.rs             # Skill trees (550 lines)
    │   │
    │   │── # 🎮 Core Game
    │   ├── state.rs              # Game state machine (150 lines)
    │   ├── player.rs             # Player data (270 lines)
    │   ├── enemy.rs              # Enemy system (400 lines)
    │   ├── dungeon.rs            # Floor generation (220 lines)
    │   ├── items.rs              # Items & relics (480 lines)
    │   ├── spells.rs             # Magic system (260 lines)
    │   ├── events.rs             # Random events (320 lines)
    │   ├── quests.rs             # Quest system (420 lines)
    │   ├── characters.rs         # NPCs (370 lines)
    │   ├── world.rs              # World/locations (700 lines)
    │   ├── world_engine.rs       # World state (270 lines)
    │   ├── event_bus.rs          # Event system (500 lines)
    │   ├── save.rs               # Save/load (220 lines)
    │   └── config.rs             # Configuration (320 lines)
    │
    ├── data/                     # 󰆼 CONTENT (~2,500 lines)
    │   ├── mod.rs                # Data exports
    │   ├── word_lists.rs         # Typing pools (200 lines)
    │   ├── sentences.rs          # Boss phrases (600 lines)
    │   └── enemies.rs            # Enemy database (520 lines)
    │
    └── ui/                       # 󰍹 RENDERING (~700 lines)
        ├── mod.rs                # UI exports
        └── render.rs             # Ratatui TUI (690 lines)

Total: ~20,000 lines of Rust across 37 source files
```

### Key Systems

| System | File | LOC | Description |
|--------|------|-----|-------------|
| Narrative Engine | `narrative_integration.rs` | ~600 | Coordinates all story systems |
| Deep Lore | `deep_lore.rs` | ~1,200 | Cosmology, faction histories, endings |
| Typing Feel | `typing_feel.rs` | ~450 | Flow states, combos, visual effects |
| Meta Progression | `meta_progression.rs` | ~650 | Persistent unlocks, achievements |
| Faction System | `faction_system.rs` | ~815 | Five factions with relationships |
| Voice System | `voice_system.rs` | ~800 | NPC dialogue with personalities |
| Narrative Seed | `narrative_seed.rs` | ~900 | Procedural narrative generation |
| Event Bus | `event_bus.rs` | ~500 | Game-wide event system |
| Combat Engine | `combat_engine.rs` | ~420 | Event-driven typing combat |

### System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TYPINGQUEST v0.2.0                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────── NARRATIVE LAYER ───────────────────────┐         │
│  │                                                                │         │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │         │
│  │  │ DeepLore │  │ Factions │  │  Voice   │  │    Lore      │  │         │
│  │  │  System  │  │  System  │  │  System  │  │  Fragments   │  │         │
│  │  │ 12 ends  │  │ 5 groups │  │ 15+ NPCs │  │ 13+ pieces   │  │         │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └──────┬───────┘  │         │
│  │       └─────────────┴─────────────┴───────────────┘           │         │
│  │                           │                                    │         │
│  │              ┌────────────┴────────────┐                      │         │
│  │              │   NarrativeEngine       │                      │         │
│  │              │  Chapter • Mystery •    │                      │         │
│  │              │  Encounters • Bonds     │                      │         │
│  │              └────────────┬────────────┘                      │         │
│  └───────────────────────────┼───────────────────────────────────┘         │
│                              │                                              │
│  ┌───────────────────────────┴────────────── TYPING LAYER ──────┐         │
│  │                                                               │         │
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐  │         │
│  │  │   FlowState    │  │     Combo      │  │    Rhythm      │  │         │
│  │  │ Building →     │  │   1x → 3x      │  │   Cadence      │  │         │
│  │  │ Transcendent   │  │   multiplier   │  │   Analysis     │  │         │
│  │  └───────┬────────┘  └───────┬────────┘  └───────┬────────┘  │         │
│  │          └───────────────────┼───────────────────┘           │         │
│  │                              │                                │         │
│  │              ┌───────────────┴───────────────┐               │         │
│  │              │      TypingFeel Engine        │               │         │
│  │              │   Visual FX • Crit Chance     │               │         │
│  │              └───────────────┬───────────────┘               │         │
│  └──────────────────────────────┼───────────────────────────────┘         │
│                                 │                                          │
│  ┌──────────────────────────────┴─── PERSISTENCE LAYER ─────────┐         │
│  │                                                               │         │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐         │         │
│  │  │   Ink   │  │ Unlocks │  │  Codex  │  │  Heat   │         │         │
│  │  │ Currency│  │  Tree   │  │  Lore   │  │ System  │         │         │
│  │  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘         │         │
│  │       └────────────┴────────────┴────────────┘              │         │
│  │                           │                                  │         │
│  │              ┌────────────┴────────────┐                    │         │
│  │              │     MetaProgression     │                    │         │
│  │              │  Survives permadeath    │                    │         │
│  │              └─────────────────────────┘                    │         │
│  └──────────────────────────────────────────────────────────────┘         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 󰒓 Configuration

### Difficulty Presets

| Preset | Description |
|--------|-------------|
| **Story** | Relaxed mode for narrative enjoyment |
| **Normal** | Standard challenge with adaptive difficulty |
| **Hard** | For experienced typists seeking challenge |
| **Ironman** | Permadeath, no saves — true roguelike |

### Config Location

```
~/.config/typingquest/config.ron    # Linux
~/Library/Application Support/typingquest/config.ron  # macOS
```

---

## 󰋚 Roadmap

- [x] Core game loop
- [x] 5 playable classes
- [x] Combat with typing mechanics
- [x] Dungeon progression (10 floors)
- [x] Items, equipment, relics
- [x] Event system with choices
- [x] Save/load system
- [x] Configuration system
- [x] Statistics & achievements
- [x] **Deep narrative systems** *(0.2.0)*
- [x] **Faction reputation & relationships** *(0.2.0)*
- [x] **Typing feel engine** *(0.2.0)*
- [x] **Meta-progression (Hades-style)** *(0.2.0)*
- [x] **Lore codex & mystery system** *(0.2.0)*
- [x] **NPC bonds across runs** *(0.2.0)*
- [ ] Sound effects (rodio integration)
- [ ] External content files (JSON/RON)
- [ ] Multiplayer typing races
- [ ] Steam/itch.io release

---

## 󰈙 License

MIT License - See [LICENSE](LICENSE) for details.

---

## 󱗗 Credits

- [ratatui](https://github.com/ratatui-org/ratatui) — Terminal UI framework
- [ttyper](https://github.com/max-niederman/ttyper) — Typing test inspiration
- *Undertale*, *Earthbound*, *Balatro* — Gameplay & aesthetic inspiration

---

**Type fast. Fight hard. Discover the truth.** 󰓥

Original work by **Dr. Baklava** • [github.com/cd4u2b0z](https://github.com/cd4u2b0z) • 2025

---

<details>
<summary><strong>📊 Project Stats</strong></summary>

- **Total Lines of Code:** ~20,000
- **Rust Source Files:** 35+
- **Major Systems:** 9
- **Authored NPC Encounters:** 15+
- **Lore Fragments:** 13+
- **Playable Classes:** 5
- **Factions:** 5
- **Endings:** 12

</details>
