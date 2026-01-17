# ⚔️ TypingQuest

**A roguelike RPG typing adventure — type to cast spells, defeat enemies, and save the realm.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.1.0-blue?style=flat)]()
[![TUI](https://img.shields.io/badge/TUI-ratatui-purple?style=flat)](https://github.com/ratatui-org/ratatui)

TypingQuest combines the satisfying mechanics of typing trainers like [ttyper](https://github.com/max-niederman/ttyper) with deep RPG progression inspired by *Undertale*, *Earthbound*, *Balatro*, and classic roguelikes.

```
╔══════════════════════════════════════════════════════════════════╗
║  TypingQuest                                    Floor: 3         ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                  ║
║       ╭─────╮      vs      ╭─────╮                              ║
║       │ YOU │              │ ☠️  │  Shadow Wraith               ║
║       │ ███ │              │ ░░░ │  HP: ████████░░ 80/100       ║
║       ╰─────╯              ╰─────╯                              ║
║                                                                  ║
║  ┌────────────────────────────────────────────────────────────┐  ║
║  │  Type: "incantation"                                       │  ║
║  │  >     incan_                          ⏱️ 3.2s  🔥 5x      │  ║
║  └────────────────────────────────────────────────────────────┘  ║
║                                                                  ║
║  [WPM: 78]  [Accuracy: 96%]  [Combo: 5x]  [Streak: 🔥🔥🔥]     ║
╚══════════════════════════════════════════════════════════════════╝
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
- [Architecture](#-architecture)
- [Configuration](#-configuration)
- [Roadmap](#-roadmap)
- [License](#-license)
- [Credits](#-credits)

---

## 󰓎 Features

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
| Combat | ⚔️ | Standard enemy encounters |
| Elite | 💀 | Harder enemies, better rewards |
| Boss | 👑 | Floor boss battles |
| Treasure | 📦 | Free items and gold |
| Shop | 🛒 | Buy equipment and consumables |
| Rest | 🏕️ | Heal, train, or meditate |
| Event | ❓ | Random encounters with choices |

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
rand = "0.8"          # Random generation
```

---

## 󰑣 Quick Start

### One-Line Install (Linux/macOS)

```bash
git clone https://github.com/cd4u2b0z/typingquest.git ~/typingquest
cd ~/typingquest && cargo build --release && ./target/release/typingquest
```

### Run from Source

```bash
git clone https://github.com/cd4u2b0z/typingquest.git
cd typingquest
cargo run --release
```

---

## 󰏗 Installation

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Step 2: Clone Repository

```bash
git clone https://github.com/cd4u2b0z/typingquest.git ~/typingquest
cd ~/typingquest
```

### Step 3: Build Release

```bash
cargo build --release
```

### Step 4: Run

```bash
./target/release/typingquest
```

### Optional: Add to PATH

```bash
# Add to ~/.zshrc or ~/.bashrc
alias typingquest='~/typingquest/target/release/typingquest'
```

---

## 󰈈 How to Play

### 1. Choose Your Class

Each class has unique strengths — see [Classes](#-classes) section below.

### 2. Explore the Dungeon

Navigate through procedurally generated floors. Each room presents different challenges:

- **Combat** — Type words to attack enemies
- **Shops** — Spend gold on upgrades
- **Rest Sites** — Recover HP/MP or train
- **Events** — Make choices that affect your run

### 3. Type to Fight

When combat begins:

1. A word appears on screen
2. Type it correctly as fast as possible
3. Your speed and accuracy determine damage
4. Build combos for massive multipliers
5. Watch your timer — enemies attack when it runs out!

### 4. Survive & Progress

Clear 10 floors to achieve victory. Death means starting over (it's a roguelike!).

---

## 󰌌 Controls

### Menu Navigation

| Key | Action |
|-----|--------|
| `↑` / `k` | Menu up |
| `↓` / `j` | Menu down |
| `Enter` | Confirm / Select |
| `Esc` | Back / Cancel |

### Dungeon

| Key | Action |
|-----|--------|
| `e` / `Enter` | Explore (next room) |
| `i` | Open inventory |
| `s` | View stats |
| `q` | Quit game |

### Combat

| Key | Action |
|-----|--------|
| `[a-z]` | Type characters |
| `Backspace` | Delete last character |
| `Esc` | Attempt to flee |

---

## 󰆥 Classes

| Class | Specialty | Playstyle |
|-------|-----------|-----------|
| 󰊠 **Wordsmith** | Balanced | Jack of all trades, forgiving for beginners |
| 󰏫 **Scribe** | Accuracy | Bonus damage for perfect words |
| 󱐋 **Spellweaver** | Magic | MP-based abilities, elemental attacks |
| 󰞇 **Barbarian** | Damage | High attack, lower defense |
| 󰊤 **Trickster** | Critical | High crit chance, risky but rewarding |

---

## 󰙅 Architecture

```
src/
├── main.rs              # Game loop & input handling
├── game/
│   ├── state.rs         # GameState, Scene management
│   ├── player.rs        # Player stats, classes, leveling
│   ├── enemy.rs         # Enemy types, AI, spawning
│   ├── combat.rs        # Combat state machine
│   ├── combat_engine.rs # Event-driven combat system
│   ├── combat_events.rs # Combat event types
│   ├── dungeon.rs       # Floor generation, room types
│   ├── items.rs         # Equipment, consumables, relics
│   ├── spells.rs        # Magic system
│   ├── events.rs        # Random encounter events
│   ├── skills.rs        # Skill trees, abilities
│   ├── narrative.rs     # Story, dialogue, factions
│   ├── quests.rs        # Quest system
│   ├── save.rs          # Save/load functionality
│   ├── config.rs        # Game configuration
│   └── stats.rs         # Statistics & achievements
├── data/
│   ├── sentences.rs     # Typing content database
│   ├── word_lists.rs    # Categorized word pools
│   └── enemies.rs       # Enemy templates
└── ui/
    └── render.rs        # TUI rendering with ratatui
```

### Module Overview

| Module | Purpose |
|--------|---------|
| `state.rs` | Core game state, scene transitions |
| `combat_engine.rs` | Event-driven combat (returns events, no side effects) |
| `config.rs` | Difficulty presets, typing parameters |
| `stats.rs` | Achievement tracking, performance metrics |
| `save.rs` | RON-based save/load system |

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

**Type fast. Fight hard. Save the realm.** ⚔️

Original work by Dr. Baklava • [github.com/cd4u2b0z](https://github.com/cd4u2b0z) • 2026
