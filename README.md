# 󰓥 TypingQuest

**A roguelike typing RPG for the terminal — defeat enemies through the rhythm of your keystrokes.**

<p align="center">
  <img src="demo.gif" alt="TypingQuest Demo" width="800">
</p>

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.3.0-blue?style=flat)](CHANGELOG.md)
[![Status](https://img.shields.io/badge/Status-Experimental_🧪-yellow?style=flat)]()
[![TUI](https://img.shields.io/badge/TUI-ratatui-purple?style=flat)](https://github.com/ratatui-org/ratatui)

---

## ⚠️ Project Status

**This is a personal hobby project — a fun "fuck around and find out" experiment in terminal game development.**

It currently exists in a *largely disconnected state*. There's ~23,000 lines of code across 37 modules, but honestly? About 60% of that is dormant scaffolding waiting to be wired up. The vision is grand. The reality is messier.

### What Actually Works (v0.3.0)
- ✅ Full 10-floor dungeon progression
- ✅ Combat with typing, combos, and flow states
- ✅ 5 playable classes with distinct stats
- ✅ Basic spellcasting (Tab to toggle, 1-9 to select)
- ✅ Items, shops, rest sites, treasure rooms
- ✅ Tutorial system (5 phases)
- ✅ Help overlay (press `?`)
- ✅ Meta-progression with Ink shop (buy permanent upgrades!)
- ✅ Faction reputation system (displayed in Stats)

### What's Dormant (~15,000 Lines)
These systems exist but aren't connected to gameplay yet:
- 🔸 **Deep Lore** (853 lines) — World cosmology, lore fragments
- 🔸 **Narrative Seeds** (906 lines) — Story generation framework
- 🔸 **Voice System** (794 lines) — NPC personality/dialogue
- 🔸 **Skills** (490 lines) — Active/passive skill trees
- 🔸 **Typing Context** (754 lines) — Thematic word generation
- 🔸 **Encounter Writing** (783 lines) — Authored events
- 🔸 **Run Modifiers** (632 lines) — Challenge variants

### Known Issues
- 🐛 Balance is completely untested — you might breeze or get wrecked
- 🐛 Some events reference systems that don't exist yet
- 🐛 Spell damage calculations are placeholder
- 🐛 No save/load — death is permadeath, closing is also death
- 🐛 Item effects are minimal
- 🐛 The game "ending" is pretty anticlimactic

This is a learning project. It compiles. It runs. It's fun to hack on. Don't expect polish.

---

## Vision

TypingQuest is a typing game that *feels* like an RPG. Every keystroke has weight. Combos build momentum. Flow states reward consistency. The dungeon unfolds through your fingers.

Inspired by [ttyper](https://github.com/max-niederman/ttyper), *Undertale*, *Balatro*, and *Hades*.

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/cd4u2b0z/typingquest.git
cd typingquest
cargo build --release

# Run the game
./target/release/typingquest
```

**Requirements:**
- Rust 1.70+ 
- A terminal with Unicode support
- [Nerd Font](https://www.nerdfonts.com/) recommended for icons

---

## How It Plays

```
╭─────────────────────────────────────────────────────────────────────╮
│  󰈸 FLOOR 3 — The Whispering Archives           HP ████████░░ 42/50 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   󰟀  Shadow Scribe                                                  │
│      ████████░░░░░░░░ 34/80 HP                                      │
│                                                                     │
│   ┌─────────────────────────────────────────┐                       │
│   │  Type:  「 arcane manuscript 」          │                       │
│   │         ~~~~~~ ___________              │                       │
│   └─────────────────────────────────────────┘                       │
│                                                                     │
│   󰑴 Combo: 7x  ·  󰓅 Flow: BUILDING  ·  󰧺 89 WPM  ·  󰸞 96%         │
│                                                                     │
╰─────────────────────────────────────────────────────────────────────╯
    [Tab] Spells  [?] Help  [Esc] Flee
```

### Core Loop

1. **Explore** — Navigate a 10-floor dungeon with procedural rooms
2. **Fight** — Type words to attack enemies; accuracy and speed deal damage
3. **Grow** — Level up, find items, learn spells, build your character
4. **Die** — Roguelike permadeath with meta-progression between runs

### Combat

Words appear. You type them. Damage happens.

- **Correct characters** flash green; errors flash red
- **Combos** build with consecutive correct words (up to 3x damage)
- **Flow states** reward consistent typing rhythm
- **Time pressure** adds urgency without being punishing

### Exploration

Each floor contains rooms: combat encounters, elite enemies, shops, rest sites, treasure, and random events. Choose your path. Manage your resources. Reach the boss.

---

## Controls

| Key | Action |
|-----|--------|
| `j/k` or `↑/↓` | Navigate menus |
| `Enter` or `e` | Confirm / Explore |
| `Backspace` | Fix typing errors |
| `Esc` | Back / Flee combat |
| `?` | Toggle help overlay |
| `i` | Inventory |
| `s` | Character stats |
| `Tab` | Toggle spell mode (combat) |
| `1-9` | Select spell (spell mode) |

---

## Classes

| Class | Style | Strength |
|-------|-------|----------|
| 󰜁 **Wordsmith** | Balanced | +10% damage, starts with Heal |
| 󰯂 **Scribe** | Spellcaster | +25% MP, faster spell learning |
| 󰺝 **Spellweaver** | Glass cannon | +50% spell damage, -20% HP |
| 󰓥 **Barbarian** | Tank | +30% HP, +15% damage, no spells |
| 󰗎 **Trickster** | Luck-based | Random bonuses, critical hits |

---

## Features

### Working (v0.3.0)

**Core Gameplay**
- 5 playable classes with distinct stats
- 10-floor dungeon with procedural room generation
- Typing-based combat with real-time feedback
- Spell casting — Tab to toggle, 1-9 to select, type incantation
- Item system — equipment, consumables (effects are minimal)
- Shop, rest, treasure, and event encounters

**Combat Feel**
- Combo system with 10% damage bonus per combo (up to 3x at 20)
- Flow states: Building → Flowing → Transcendent
- WPM and accuracy tracking with live display
- Faction reputation tracking (displayed in Stats)

**UI/UX**
- Consistent visual theme with semantic colors
- 40+ Nerd Font icons throughout
- Contextual help system (`?` key)
- 5-phase interactive tutorial
- Meta-progression: Ink earned on death (doesn't persist yet)

### Dormant (written but not connected)

- Deep lore system, lore fragments, world cosmology
- NPC voice/personality system
- Authored encounter writing
- Skill trees (active/passive)
- Run modifiers and challenge variants
- Thematic word generation by context

See [Project Status](#️-project-status) for the full breakdown.

---

## Project Structure

```
typingquest/
├── src/
│   ├── main.rs                  # Game loop, input handling (772 lines)
│   │
│   ├── game/                    # Core game logic (~15,000 lines)
│   │   ├── state.rs             # Game state, scene management
│   │   ├── combat.rs            # Combat state, spell mode
│   │   ├── combat_engine.rs     # Damage calc, word generation
│   │   ├── player.rs            # Player, classes, leveling
│   │   ├── enemy.rs             # Enemy definitions
│   │   ├── dungeon.rs           # Floor/room generation
│   │   ├── items.rs             # Equipment, consumables
│   │   ├── spells.rs            # Spell definitions
│   │   ├── typing_feel.rs       # ✅ Combo, flow, rhythm (550 lines)
│   │   ├── tutorial.rs          # ✅ 5-phase tutorial (617 lines)
│   │   ├── help_system.rs       # ✅ Help overlay (749 lines)
│   │   ├── faction_system.rs    # ✅ Faction rep tracking (815 lines)
│   │   ├── meta_progression.rs  # ✅ Ink/unlocks framework (612 lines)
│   │   ├── deep_lore.rs         # 🔸 World cosmology (853 lines)
│   │   ├── narrative_seed.rs    # 🔸 Story generation (906 lines)
│   │   ├── voice_system.rs      # 🔸 NPC personality (794 lines)
│   │   ├── skills.rs            # 🔸 Skill trees (490 lines)
│   │   ├── typing_context.rs    # 🔸 Thematic words (754 lines)
│   │   ├── encounter_writing.rs # 🔸 Authored events (783 lines)
│   │   └── run_modifiers.rs     # 🔸 Challenge variants (632 lines)
│   │
│   ├── ui/                      # Rendering (~1,900 lines)
│   │   ├── render.rs            # All screen rendering (1,399 lines)
│   │   └── theme.rs             # Colors, icons, styles (424 lines)
│   │
│   └── data/                    # Static content (~1,350 lines)
│       ├── enemies.rs           # Enemy data (555 lines)
│       ├── sentences.rs         # Word lists (490 lines)
│       └── word_lists.rs        # More words (155 lines)
│
├── Cargo.toml
├── CHANGELOG.md
└── README.md

✅ = Wired up and working
🔸 = Written but dormant
```

**~23,700 lines of Rust** across 45 source files.

---

## Roadmap

### v0.3.0 — Gameplay Integration ✅
- [x] Wire up spell casting (Tab + 1-9 + incantation)
- [x] Wire up faction reputation system
- [x] Wire up combat feel (combos, flow states)
- [x] Meta-progression: Ink earned on death
- [x] Fix floor progression bugs

### v0.4.0 — Persistence (someday)
- [ ] Save/load game state
- [ ] Ink persistence between runs
- [ ] Settings/config file

### v0.5.0 — Connect Dormant Systems (aspirational)
- [ ] Wire up deep lore / lore fragments
- [ ] Wire up voice system for NPCs
- [ ] Wire up skill trees
- [ ] Wire up thematic word generation

### v1.0.0 — Probably Never
- [ ] Balance pass
- [ ] Multiple endings
- [ ] Full documentation
- [ ] Actually finish something for once

---

## Building

```bash
# Development build
cargo build

# Release build (optimized, ~1.5MB binary)
cargo build --release

# Run directly
cargo run --release

# Check for errors without building
cargo check
```

---

## Contributing

TypingQuest is a personal hobby project. It's not looking for contributors, but if you want to fork it and do something cool, go for it.

If you find a bug that's actually blocking gameplay (not just "this system isn't wired up"), feel free to open an issue.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Credits

**TypingQuest** — Original work by Dr. Baklava

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [Nerd Fonts](https://www.nerdfonts.com/) — Icons

Inspired by:
- [ttyper](https://github.com/max-niederman/ttyper) — Terminal typing
- *Undertale* — Personality and charm
- *Hades* — Meta-progression
- *Balatro* — Satisfying feedback

---

*󰩛 Dr. Baklava was here*
