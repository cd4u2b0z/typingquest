# Changelog

All notable changes to TypingQuest will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Event-driven combat engine (`combat_engine.rs`, `combat_events.rs`)
- Save/load system with RON serialization (`save.rs`)
- Configuration system with difficulty presets (`config.rs`)
- Statistics and achievement tracking (`stats.rs`)
- Comprehensive word/sentence databases (`data/`)
- Enemy template system for data-driven content
- Adaptive difficulty based on player performance
- Combo system with streak bonuses
- Perfect word bonuses and accuracy tracking

### Changed
- Removed unused `tokio` dependency
- Improved code documentation throughout

## [0.1.0] - 2025-01-17

### Added
- Initial release
- Core game loop with scene-based state management
- 5 playable classes: Wordsmith, Scribe, Spellweaver, Barbarian, Trickster
- Combat system with typing-based attacks
- Dungeon exploration with procedural room generation
- Room types: Combat, Elite, Boss, Treasure, Shop, Rest, Event
- Item system with consumables, equipment, and relics
- Spell system with elemental magic
- Event system with branching choices
- Skill trees and character progression
- Narrative system with factions and dialogue
- Quest system
- TUI rendering with ratatui

### Technical
- Rust 2021 edition
- ratatui 0.28 for terminal UI
- crossterm 0.28 for input/terminal control
- serde for serialization
- Release binary: ~1.4MB (optimized + stripped)
