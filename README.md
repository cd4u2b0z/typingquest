# ⌨️ Keyboard Warrior

**Finally, a reason to be one.**

*A high-fantasy roguelike where you defeat ancient evils through the rhythm of your keystrokes. Descend into the ruins of a shattered kingdom. Face corrupted knights, void walkers, and eldritch horrors. Type to survive. Die repeatedly.*

<p align="center">
  <img src="demo.gif" alt="Keyboard Warrior Demo" width="800">
</p>

[![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Version](https://img.shields.io/badge/Version-0.5.0-blue?style=flat)](CHANGELOG.md)
[![Status](https://img.shields.io/badge/Status-Active_Development_🔥-orange?style=flat)]()
[![TUI](https://img.shields.io/badge/TUI-ratatui-purple?style=flat)](https://github.com/ratatui-org/ratatui)

---

## ⚠️ Project Status

**This is a personal hobby project — a fun "fuck around and find out" experiment in terminal game development.**

### What Actually Works (v0.5.0)
- ✅ Full 10-floor dungeon progression with zone-themed encounters
- ✅ Combat with typing, combos, and flow states
- ✅ **Lore-integrated typing** — words match the zone, enemy, and story
- ✅ **Corruption effects on typing** — narrative state warps your words
- ✅ 5 playable classes with distinct stats
- ✅ **Skill tree combat integration** — damage, crits, evasion, damage reduction
- ✅ Basic spellcasting (Tab to toggle, 1-9 to select)
- ✅ Items, shops, rest sites, treasure rooms
- ✅ **Faction-voiced NPCs** — merchants and healers speak with personality
- ✅ **Authored encounters** — scripted narrative moments during exploration
- ✅ **Run modifiers** — difficulty scaling for challenge modes
- ✅ Tutorial system (5 phases)
- ✅ Help overlay (press `?`)
- ✅ Meta-progression with Ink shop (buy permanent upgrades!)
- ✅ Faction reputation system (displayed in Stats)
- ✅ Boss-specific dialogue during boss fights
- ✅ Narrative progression through typed sentences
- ✅ **EventBus architecture** — systems communicate dynamically

### Recently Integrated (v0.5.0) 🔗
These dormant systems are now ACTIVE and affecting gameplay:
- ✅ **EventBus** (60+ event types) — central nervous system for game events
- ✅ **NarrativeSeed** — corruption types that modify typing patterns
- ✅ **SkillTree** (5 trees, 25+ skills) — combat modifiers actually apply
- ✅ **VoiceSystem** (794 lines) — faction NPCs have dialogue
- ✅ **EncounterWriting** (783 lines) — authored events trigger
- ✅ **RunModifiers** (632 lines, 50+ modifiers) — difficulty scaling works

### Still Dormant (~5,000 Lines)
- 🔸 **Deep Lore** (1,016 lines) — Three Ages, The Sundering, full histories
- 🔸 **Lore Fragments** (752 lines) — Discoverable pieces of world history
- 🔸 **World System** (607 + 274 + 407 lines) — Location simulation
- 🔸 **Characters & Quests** (329 + 368 lines) — NPC relationships, quest chains

### Known Issues
- 🐛 Balance is completely untested — you might breeze or get wrecked
- 🐛 Some events reference systems that don't exist yet
- 🐛 Spell damage calculations are placeholder
- 🐛 No save/load — death is permadeath, closing is also death
- 🐛 Item effects are minimal
- 🐛 The game "ending" is pretty anticlimactic

This is a learning project. It compiles. It runs. It's fun to hack on. Don't expect polish. Expect to die.

---

## 🏰 World & Lore

*"Wait, there's actual lore?" — Yes. Thousands of lines of it. Most of it unused.*

Keyboard Warrior is set in a dark fantasy world recovering from **The Sundering** — a cataclysm caused when Archon Malachar tried to pierce the Veil between worlds and become a god.

### The Three Ages
| Age | Era | What Happened |
|-----|-----|---------------|
| **Age of Dawn** | Before time | Gods walked among mortals. Dragons spoke prophecy. |
| **Age of Crowns** | 3,000 years | Mortal kingdoms rose. The great Orders were founded. |
| **Age of Shadow** | Now (47 years) | The Sundering tore reality. The Blight spreads. |

### The Five Factions
| Faction | Philosophy |
|---------|------------|
| 🔮 **Mages Guild** | Arcane knowledge is the key to sealing the breach |
| ☀️ **Temple of Dawn** | Divine light will purge the corruption |
| 🌲 **Rangers of the Wild** | Nature's balance must be restored |
| 🗡️ **Shadow Guild** | Information is power; secrets are currency |
| 💰 **Merchant Consortium** | Trade continues; neutrality profits |

### The Dungeon Zones
- **Floors 1-2:** The Shattered Halls — ruined castle, echoes of the old kingdom
- **Floors 3-4:** The Sunken Archives — flooded library, forbidden knowledge
- **Floors 5-6:** The Blighted Gardens — corrupted greenhouse, twisted nature
- **Floors 7-8:** The Clockwork Depths — ancient machinery, dwarven ruins
- **Floors 9-10:** The Void's Edge — reality breaks down, glimpses of the beyond
- **Floor 11+:** The Breach — where The Sundering occurred

### The Mystery
*You are not who you think you are.* As you descend, fragments of memory surface. The truth waits at the bottom — and three possible endings: **Final Rest**, **Dark Ascension**, or **The Third Path**.

---

## Vision

Keyboard Warrior is a typing game that *feels* like an RPG. Every keystroke has weight. Combos build momentum. Flow states reward consistency. The dungeon unfolds through your fingers.

It's also a pun. You're literally a warrior. With a keyboard. Fighting things. In a terminal. Look, the name was available.

Inspired by [ttyper](https://github.com/max-niederman/ttyper), *Undertale*, *Balatro*, *Hades*, and *Dark Souls*.

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/cd4u2b0z/keyboard-warrior.git
cd keyboard-warrior
cargo build --release

# Run the game
./target/release/keyboard-warrior
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

### Working (v0.4.0)

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

- Deep lore system with high fantasy cosmology (Three Ages, The Sundering, Malachar the Archon)
- Five factions: Mages Guild, Temple of Dawn, Rangers of the Wild, Shadow Guild, Merchant Consortium
- NPC voice/personality system
- Authored encounter writing
- Skill trees (active/passive)
- Run modifiers and challenge variants
- Thematic word generation by context

See [Project Status](#️-project-status) for the full breakdown.

---

## Project Structure

```
keyboard-warrior/
├── src/
│   ├── main.rs                    # Game loop, input handling (820 lines)
│   │
│   ├── game/                      # Core game logic (~19,000 lines, 37 files)
│   │   ├── mod.rs                 # Module exports
│   │   │
│   │   │ # ─── CORE SYSTEMS (wired up) ───
│   │   ├── state.rs               # ✅ Game state, scene management (500+)
│   │   ├── combat.rs              # ✅ Combat state, skill integration (600+)
│   │   ├── combat_engine.rs       # ✅ Damage calc, word generation (510)
│   │   ├── combat_events.rs       # ✅ Combat event handling (282)
│   │   ├── player.rs              # ✅ Player, classes, leveling (381)
│   │   ├── enemy.rs               # ✅ Enemy definitions (366)
│   │   ├── dungeon.rs             # ✅ Floor/room generation (308)
│   │   ├── items.rs               # ✅ Equipment, consumables (455)
│   │   ├── spells.rs              # ✅ Spell definitions (281)
│   │   ├── events.rs              # ✅ Game events system (305)
│   │   ├── stats.rs               # ✅ Statistics tracking (559)
│   │   │
│   │   │ # ─── FEEL & UX (wired up) ───
│   │   ├── typing_feel.rs         # ✅ Combo, flow, rhythm (550)
│   │   ├── tutorial.rs            # ✅ 5-phase tutorial (617)
│   │   ├── help_system.rs         # ✅ Help overlay (750)
│   │   ├── config.rs              # ✅ Game configuration (434)
│   │   │
│   │   │ # ─── PROGRESSION (wired up) ───
│   │   ├── faction_system.rs      # ✅ Faction reputation (815)
│   │   ├── meta_progression.rs    # ✅ Ink/unlocks framework (612)
│   │   ├── save.rs                # ✅ Save/load system (299)
│   │   │
│   │   │ # ─── INTEGRATED SYSTEMS (v0.5.0) ───
│   │   ├── event_bus.rs           # ✅ Central event messaging (594)
│   │   ├── narrative_seed.rs      # ✅ Corruption effects on typing (906)
│   │   ├── skills.rs              # ✅ Skill trees → combat (490+)
│   │   ├── voice_system.rs        # ✅ Faction NPC dialogue (794)
│   │   ├── encounter_writing.rs   # ✅ Authored exploration events (783)
│   │   ├── run_modifiers.rs       # ✅ Difficulty scaling (632)
│   │   │
│   │   │ # ─── WORLD & LORE (content complete) ───
│   │   ├── deep_lore.rs           # 🔸 High fantasy cosmology (1,016)
│   │   ├── lore_fragments.rs      # 🔸 Discoverable lore pieces (752)
│   │   ├── narrative.rs           # 🔸 Core narrative structures (486)
│   │   ├── narrative_integration.rs # 🔸 Narrative hooks (649)
│   │   ├── world.rs               # 🔸 World locations (607)
│   │   ├── world_engine.rs        # 🔸 World simulation (274)
│   │   ├── world_integration.rs   # 🔸 Zone definitions (407)
│   │   ├── writing_guidelines.rs  # 🔸 Tone/style guide (540)
│   │   │
│   │   │ # ─── CHARACTERS & QUESTS (dormant) ───
│   │   ├── characters.rs          # 🔸 NPC definitions (329)
│   │   ├── quests.rs              # 🔸 Quest system (368)
│   │   │
│   │   │ # ─── LEGACY ───
│   │   ├── typing_context.rs      # 🔄 Replaced by lore_words.rs (754)
│   │   └── enemy_old.rs           # 🗑️ Legacy (240)
│   │
│   ├── ui/                        # Rendering (~2,000 lines)
│   │   ├── render.rs              # All screen rendering (1,502)
│   │   ├── theme.rs               # Colors, icons, styles (424)
│   │   └── lore_render.rs         # Lore display (80)
│   │
│   └── data/                      # Static content (~1,700 lines)
│       ├── mod.rs                 # Data exports, lore word methods (180)
│       ├── enemies.rs             # Enemy templates (555)
│       ├── sentences.rs           # Word lists (490)
│       ├── word_lists.rs          # More words (155)
│       └── lore_words.rs          # ✅ Zone/enemy/boss word pools (350)
│
├── docs/
│   └── SYSTEM_INTEGRATION_PLAN.md # Integration roadmap (completed)
│
├── .github/                       # GitHub configuration
│   ├── workflows/ci.yml           # CI pipeline (Rust checks, cross-build)
│   ├── ISSUE_TEMPLATE/            # Bug report, feature request templates
│   └── PULL_REQUEST_TEMPLATE.md   # PR template
│
├── Cargo.toml
├── CHANGELOG.md
└── README.md

✅ = Wired up and working
🔸 = Content written, not connected to gameplay
🔄 = Superseded
🗑️ = Legacy/deprecated
```

**~24,500 lines of Rust** across 49 source files.

---

## Roadmap

### v0.3.0 — Gameplay Integration ✅
- [x] Wire up spell casting (Tab + 1-9 + incantation)
- [x] Wire up faction reputation system
- [x] Wire up combat feel (combos, flow states)
- [x] Meta-progression: Ink earned on death
- [x] Fix floor progression bugs

### v0.4.0 — Fantasy Lore Overhaul ✅
- [x] Rewrite deep_lore.rs with high fantasy cosmology (Three Ages, The Sundering, Malachar)
- [x] Replace all factions (MagesGuild, TempleOfDawn, RangersOfTheWild, ShadowGuild, MerchantConsortium)
- [x] Fantasy-themed zones (ShatteredHalls, SunkenArchives, BlightedGardens, ClockworkDepths, VoidsEdge, TheBreach)
- [x] Replace bosses (The Hollow Knight, The Void Herald)
- [x] Fantasy enemy names and descriptions (14 enemies)
- [x] Complete world cosmology with player mystery and multiple endings

### v0.4.1 — Lore-Integrated Typing ✅
- [x] Zone-specific word pools (30+ words per zone)
- [x] Zone-specific sentences that tell the story
- [x] Enemy-type word themes (goblin, undead, spectral, corrupted, mechanical, void)
- [x] Boss-specific dialogue (Hollow Knight, Void Herald unique lines)
- [x] Narrative progression (early/mid/late game sentences)
- [x] Combat system wired to use lore words instead of generic words

### v0.5.0 — Persistence & Polish
- [ ] Save/load game state
- [ ] Ink persistence between runs
- [ ] Settings/config file
- [ ] Balance pass

### v0.6.0 — Connect Dormant Systems (aspirational)
- [ ] Wire up deep lore / lore fragments to gameplay
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

Keyboard Warrior is a personal hobby project. It's not looking for contributors, but if you want to fork it and do something cool, go for it.

If you find a bug that's actually blocking gameplay (not just "this system isn't wired up"), feel free to open an issue. I might even fix it.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

Do whatever you want with this code. I'm not your mom.

---

## Credits

**Keyboard Warrior** — Original work by Dr. Baklava

*"I spent hundreds of hours on this so you can type 'ancient prophecy' at a Goblin."*

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) — Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal manipulation
- [Nerd Fonts](https://www.nerdfonts.com/) — Icons

Inspired by:
- [ttyper](https://github.com/max-niederman/ttyper) — Terminal typing
- *Undertale* — Personality and charm
- *Hades* — Meta-progression
- *Balatro* — Satisfying feedback
- *Dark Souls* — The joy of repeated death

---

*⌨️ Type well, die often.*
