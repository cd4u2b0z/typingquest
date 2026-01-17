
#![allow(dead_code, unused_imports, unused_variables)]
//! Game logic modules - Deep RPG system with narrative coherence
//!
//! Core systems for TypingQuest - a typing-based roguelike RPG

// Core game state
pub mod state;
pub mod player;
pub mod enemy;

// Combat system
pub mod combat;
pub mod combat_events;
pub mod combat_engine;

// Character progression
pub mod spells;
pub mod items;
pub mod skills;

// World and narrative
pub mod dungeon;
pub mod events;
pub mod narrative;
pub mod quests;
pub mod characters;
pub mod world;

// New deep systems
pub mod narrative_seed;
pub mod faction_system;
pub mod typing_context;
pub mod event_bus;
pub mod run_modifiers;
pub mod voice_system;

// Persistence and configuration
pub mod save;
pub mod config;
pub mod stats;

pub mod world_engine;
