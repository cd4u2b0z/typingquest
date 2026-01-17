//! World Engine - Integrates all systems for cohesive gameplay
//!
//! This is the "brain" that connects all deep systems together.

use std::collections::HashMap;
use rand::prelude::*;
use rand::SeedableRng;

use super::narrative::Faction;
use super::narrative_seed::{NarrativeSeed, CorruptionType};
use super::faction_system::{FactionRelations, FactionBenefit, FactionPenalty};
use super::event_bus::{EventBus, GameEvent, CombatOutcome};
use super::typing_context::{TypingContext, CombatContext};
use super::run_modifiers::{RunModifiers, Modifier, RunType};
use super::voice_system::{FactionVoice, build_faction_voices, generate_faction_dialogue, DialogueContext as VoiceDialogueContext};

/// The central game state that integrates all systems
#[derive(Debug)]
pub struct WorldEngine {
    pub narrative_seed: NarrativeSeed,
    pub faction_relations: FactionRelations,
    pub event_bus: EventBus,
    pub run_modifiers: RunModifiers,
    pub faction_voices: HashMap<Faction, FactionVoice>,
    pub corruption_level: f32,
    pub current_chapter: u32,
    pub days_passed: u32,
    pub rng: rand::rngs::StdRng,
    typing_modifiers: Vec<TypingModifierEffect>,
}

/// Effect a modifier has on typing
#[derive(Debug, Clone)]
pub struct TypingModifierEffect {
    pub name: String,
    pub wpm_modifier: f32,
    pub accuracy_modifier: f32,
    pub special_effect: Option<TypingSpecialEffect>,
}

#[derive(Debug, Clone)]
pub enum TypingSpecialEffect {
    ScrambleWords { delay_ms: u64 },
    MistakeDamage { damage: i32 },
    ForeignWords { frequency: f32 },
    ReversedWords { frequency: f32 },
    FadingLetters { fade_rate: f32 },
    WPMPressure { min_wpm: f32 },
}

impl WorldEngine {
    pub fn new(seed: u64) -> Self {
        let rng = rand::rngs::StdRng::seed_from_u64(seed);
        let narrative_seed = NarrativeSeed::generate(seed);
        let run_modifiers = super::run_modifiers::generate_from_seed(seed);
        let faction_voices = build_faction_voices();
        
        let mut faction_relations = FactionRelations::new();
        // Apply world state to initial faction standings
        faction_relations.modify_standing(narrative_seed.world_state.dominant_faction, 10);
        faction_relations.modify_standing(narrative_seed.world_state.declining_faction, -5);
        
        let typing_modifiers = Self::calculate_typing_modifiers(&narrative_seed, &run_modifiers);
        
        Self {
            narrative_seed,
            faction_relations,
            event_bus: EventBus::new(),
            run_modifiers,
            faction_voices,
            corruption_level: 0.15,
            current_chapter: 1,
            days_passed: 0,
            rng,
            typing_modifiers,
        }
    }
    
    pub fn with_run_type(seed: u64, run_type: RunType) -> Self {
        let mut engine = Self::new(seed);
        engine.run_modifiers.set_run_type(run_type);
        engine.typing_modifiers = Self::calculate_typing_modifiers(
            &engine.narrative_seed, 
            &engine.run_modifiers
        );
        engine
    }
    
    fn calculate_typing_modifiers(seed: &NarrativeSeed, modifiers: &RunModifiers) -> Vec<TypingModifierEffect> {
        let mut effects = Vec::new();
        
        // Map corruption type to typing effects
        let (wpm_mod, acc_mod, special) = match seed.world_state.corruption_type {
            CorruptionType::SemanticDecay => (0.9, 1.0, Some(TypingSpecialEffect::ScrambleWords { delay_ms: 3000 })),
            CorruptionType::LiteralManifest => (1.0, 0.95, Some(TypingSpecialEffect::MistakeDamage { damage: 2 })),
            CorruptionType::BabelCurse => (0.95, 0.95, Some(TypingSpecialEffect::ForeignWords { frequency: 0.1 })),
            CorruptionType::TruthInversion => (0.85, 1.0, Some(TypingSpecialEffect::ReversedWords { frequency: 0.1 })),
            CorruptionType::GraphemeParasite => (1.0, 0.9, Some(TypingSpecialEffect::FadingLetters { fade_rate: 0.5 })),
            CorruptionType::LinguisticAcceleration => (1.1, 1.0, Some(TypingSpecialEffect::WPMPressure { min_wpm: 45.0 })),
        };
        
        effects.push(TypingModifierEffect {
            name: format!("{} Corruption", seed.world_state.corruption_type.name()),
            wpm_modifier: wpm_mod,
            accuracy_modifier: acc_mod,
            special_effect: special,
        });
        
        // Add run modifier effects
        for active_mod in &modifiers.active {
            if active_mod.modifier.affects_typing() {
                if let Modifier::SpeedPressure { min_wpm } = &active_mod.modifier {
                    effects.push(TypingModifierEffect {
                        name: "Speed Pressure".to_string(),
                        wpm_modifier: 1.0,
                        accuracy_modifier: 1.0,
                        special_effect: Some(TypingSpecialEffect::WPMPressure { 
                            min_wpm: min_wpm * active_mod.level as f32 
                        }),
                    });
                }
                if let Modifier::MistakeDamage { damage_per_error } = &active_mod.modifier {
                    effects.push(TypingModifierEffect {
                        name: "Punishing Errors".to_string(),
                        wpm_modifier: 1.0,
                        accuracy_modifier: 1.0,
                        special_effect: Some(TypingSpecialEffect::MistakeDamage { 
                            damage: damage_per_error * active_mod.level as i32 
                        }),
                    });
                }
            }
        }
        
        effects
    }
    
    pub fn process_events(&mut self) {
        while let Some(event) = self.event_bus.poll() {
            self.handle_event(event);
        }
    }
    
    fn handle_event(&mut self, event: GameEvent) {
        match &event {
            GameEvent::CombatEnded { enemy, outcome } => {
                if let CombatOutcome::Victory { xp_gained, .. } = outcome {
                    self.event_bus.emit(GameEvent::ExperienceGained {
                        amount: *xp_gained,
                        source: format!("Defeated {}", enemy),
                    });
                }
            }
            GameEvent::WorldCorruptionChanged { old_level, new_level, .. } => {
                self.corruption_level = *new_level;
                if (new_level - old_level).abs() > 0.1 {
                    self.typing_modifiers = Self::calculate_typing_modifiers(
                        &self.narrative_seed,
                        &self.run_modifiers
                    );
                }
            }
            GameEvent::TimePassedDays { days } => {
                self.days_passed += days;
                if self.days_passed % 7 == 0 {
                    let spread = if self.run_modifiers.has_modifier(&Modifier::AcceleratedCorruption) {
                        0.05
                    } else {
                        0.02
                    };
                    self.event_bus.emit(GameEvent::WorldCorruptionChanged {
                        old_level: self.corruption_level,
                        new_level: (self.corruption_level + spread).min(1.0),
                        location: None,
                    });
                }
            }
            _ => {}
        }
    }
    
    pub fn generate_dialogue(&mut self, faction: Faction, context: VoiceDialogueContext) -> String {
        if let Some(voice) = self.faction_voices.get(&faction) {
            generate_faction_dialogue(voice, context, &mut self.rng)
        } else {
            "...".to_string()
        }
    }
    
    pub fn get_typing_context(&self, _context_type: &str) -> Option<TypingContext> {
        Some(TypingContext::Combat(CombatContext {
            enemy_name: "Enemy".to_string(),
            stakes: super::typing_context::CombatStakes::Normal,
            modifiers: Vec::new(),
            is_boss: false,
            environment: None,
        }))
    }
    
    pub fn get_typing_modifiers(&self) -> &[TypingModifierEffect] {
        &self.typing_modifiers
    }
    
    pub fn get_current_benefits(&self) -> Vec<FactionBenefit> {
        self.faction_relations.current_benefits()
    }
    
    pub fn get_current_penalties(&self) -> Vec<FactionPenalty> {
        self.faction_relations.current_penalties()
    }
    
    pub fn get_run_info(&self) -> RunInfo {
        RunInfo {
            seed: self.narrative_seed.seed_value,
            run_type: self.run_modifiers.run_type.clone(),
            heat: self.run_modifiers.total_heat,
            reward_multiplier: self.run_modifiers.reward_multiplier,
            corruption_type: self.narrative_seed.world_state.corruption_type.name().to_string(),
            dominant_faction: self.narrative_seed.world_state.dominant_faction.name().to_string(),
            inciting_incident: self.narrative_seed.world_state.inciting_incident.description().to_string(),
            days_passed: self.days_passed,
            corruption_level: self.corruption_level,
            chapter: self.current_chapter,
        }
    }
    
    pub fn advance_chapter(&mut self) {
        self.current_chapter += 1;
        self.event_bus.emit(GameEvent::ChapterStarted {
            chapter: self.current_chapter,
            title: format!("Chapter {}", self.current_chapter),
        });
    }
    
    pub fn emit_event(&mut self, event: GameEvent) {
        self.event_bus.emit(event);
    }
    
    pub fn tick(&mut self) {
        self.event_bus.tick();
        self.process_events();
    }
}

#[derive(Debug, Clone)]
pub struct RunInfo {
    pub seed: u64,
    pub run_type: RunType,
    pub heat: u32,
    pub reward_multiplier: f32,
    pub corruption_type: String,
    pub dominant_faction: String,
    pub inciting_incident: String,
    pub days_passed: u32,
    pub corruption_level: f32,
    pub chapter: u32,
}

/// Helper to create a new run with default settings
pub fn new_run() -> WorldEngine {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    WorldEngine::new(seed)
}

/// Helper to create a run with a specific seed (for sharing/replaying)
pub fn seeded_run(seed: u64) -> WorldEngine {
    WorldEngine::new(seed)
}

/// Helper to create a challenge run
pub fn challenge_run(seed: u64, run_type: RunType) -> WorldEngine {
    WorldEngine::with_run_type(seed, run_type)
}
