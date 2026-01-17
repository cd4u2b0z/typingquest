//! Tutorial System - Interactive onboarding through typing
//!
//! Teaches core mechanics through 5 short phases:
//! 1. Awakening - Basic typing
//! 2. First Strike - Combat basics  
//! 3. The Combo - Chaining words
//! 4. Choice - Navigation and decisions
//! 5. Discovery - Lore and exploration
//!
//! Philosophy: "Learn by typing" - every lesson uses the core mechanic

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::path::PathBuf;

// ============================================================================
// TUTORIAL PHASE
// ============================================================================

/// The five phases of the tutorial
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TutorialPhase {
    #[default]
    Awakening,      // Phase 1: Basic typing
    FirstStrike,    // Phase 2: Combat basics
    TheCombo,       // Phase 3: Chaining words
    Choice,         // Phase 4: Navigation
    Discovery,      // Phase 5: Lore system
    Complete,       // Tutorial finished
}

impl TutorialPhase {
    pub fn next(&self) -> TutorialPhase {
        match self {
            TutorialPhase::Awakening => TutorialPhase::FirstStrike,
            TutorialPhase::FirstStrike => TutorialPhase::TheCombo,
            TutorialPhase::TheCombo => TutorialPhase::Choice,
            TutorialPhase::Choice => TutorialPhase::Discovery,
            TutorialPhase::Discovery => TutorialPhase::Complete,
            TutorialPhase::Complete => TutorialPhase::Complete,
        }
    }
    
    pub fn title(&self) -> &'static str {
        match self {
            TutorialPhase::Awakening => "Awakening",
            TutorialPhase::FirstStrike => "First Strike",
            TutorialPhase::TheCombo => "The Combo",
            TutorialPhase::Choice => "Choice",
            TutorialPhase::Discovery => "Discovery",
            TutorialPhase::Complete => "Complete",
        }
    }
    
    pub fn number(&self) -> u8 {
        match self {
            TutorialPhase::Awakening => 1,
            TutorialPhase::FirstStrike => 2,
            TutorialPhase::TheCombo => 3,
            TutorialPhase::Choice => 4,
            TutorialPhase::Discovery => 5,
            TutorialPhase::Complete => 6,
        }
    }
}

// ============================================================================
// TUTORIAL STEP
// ============================================================================

/// A single step within a tutorial phase
#[derive(Debug, Clone)]
pub struct TutorialStep {
    /// Narrative text shown above the typing area
    pub narrative: &'static str,
    
    /// The word or phrase to type
    pub target_text: &'static str,
    
    /// Hint shown below the typing area
    pub hint: &'static str,
    
    /// Whether this step requires perfect typing (no backspace)
    pub require_perfect: bool,
    
    /// Optional time limit (None = no limit for tutorial)
    pub time_limit: Option<f32>,
}

impl TutorialStep {
    pub const fn new(
        narrative: &'static str,
        target_text: &'static str,
        hint: &'static str,
    ) -> Self {
        Self {
            narrative,
            target_text,
            hint,
            require_perfect: false,
            time_limit: None,
        }
    }
    
    pub const fn perfect(mut self) -> Self {
        self.require_perfect = true;
        self
    }
    
    pub const fn timed(mut self, seconds: f32) -> Self {
        self.time_limit = Some(seconds);
        self
    }
}

// ============================================================================
// TUTORIAL CONTENT
// ============================================================================

/// Get all steps for a given phase
pub fn get_phase_steps(phase: TutorialPhase) -> Vec<TutorialStep> {
    match phase {
        TutorialPhase::Awakening => vec![
            TutorialStep::new(
                "You stand at the threshold between worlds.\nThe ancient stones hum with forgotten power.\n\nTo cross over, speak the word of passage...",
                "awaken",
                "󰋖 Just type the word. Take your time.",
            ),
            TutorialStep::new(
                "The threshold recognizes you.\nMemories stir—fragments of a life before.\n\nSpeak your purpose...",
                "begin",
                "󰋖 Type naturally. The threshold is patient.",
            ),
            TutorialStep::new(
                "The way opens. Beyond lies danger and discovery.\nBut first, you must prove your resolve.\n\nDeclare your intent...",
                "journey",
                "󰋖 Longer words work the same way.",
            ),
        ],
        
        TutorialPhase::FirstStrike => vec![
            TutorialStep::new(
                "A shadow stirs in the darkness ahead.\nIt has no name, only hunger.\n\n╭───╮\n│ ░░ │  Training Shade\n╰─┬─╯   HP: █░░░░ 1/5\n  │\n\nStrike it down!",
                "strike",
                "󰋖 Type the word to attack! Speed = damage.",
            ),
            TutorialStep::new(
                "The shade recoils but does not fall.\nIt gathers itself for another assault.\n\n╭───╮\n│ ▒▒ │  Training Shade\n╰─┬─╯   HP: ░░░░░ 0/5\n  │\n\nFinish it!",
                "destroy",
                "󰈸 Faster typing deals more damage!",
            ),
            TutorialStep::new(
                "The shade dissolves into nothing.\n\n  ╭────────────────────╮\n  │  VICTORY!          │\n  │  +10 XP  +5 Gold   │\n  ╰────────────────────╯\n\nYou grow stronger with each victory.",
                "continue",
                "󰋖 Press on. Greater challenges await.",
            ),
        ],
        
        TutorialPhase::TheCombo => vec![
            TutorialStep::new(
                "True power comes from flow—the rhythm of combat.\nChain your attacks without pause.\n\n╭───╮\n│ ◉◉ │  Combo Target\n╰─┬─╯   Combo: 0x\n  │\n\nBegin the chain...",
                "hit",
                "󰈸 COMBO: Type quickly to build combos!",
            ),
            TutorialStep::new(
                "Good! The rhythm builds.\nDon't stop now!\n\n╭───╮\n│ ◉◉ │  Combo Target\n╰─┬─╯   Combo: 1x\n  │\n\nKeep it going!",
                "fast",
                "󰈸 COMBO 1x! Chain words for more damage!",
            ),
            TutorialStep::new(
                "Excellent! Feel the flow!\n\n╭───╮\n│ ◉◉ │  Combo Target\n╰─┬─╯   Combo: 2x  󰈸󰈸\n  │\n\nOne more!",
                "combo",
                "󰈸 COMBO 2x! You're getting it!",
            ),
            TutorialStep::new(
                "PERFECT CHAIN!\n\n  ╭────────────────────╮\n  │  3x COMBO!         │\n  │  Damage: 3.0x      │\n  ╰────────────────────╯\n\nCombos multiply your damage.\nNever stop typing!",
                "mastery",
                "󰄀 COMBO 3x! Maximum damage multiplier!",
            ),
        ],
        
        TutorialPhase::Choice => vec![
            TutorialStep::new(
                "The path branches before you.\nEach choice shapes your journey.\n\n  ┌─────────────────────────┐\n  │  󰓥 Left:  Combat Room   │\n  │  󰆧 Right: Treasure Room │\n  └─────────────────────────┘\n\nChoose your path by typing its name...",
                "treasure",
                "󰋖 Type the name of your choice to select it.",
            ),
            TutorialStep::new(
                "Wise choice. A glittering chest awaits.\n\n  ╭──────────╮\n  │  󰆧 ╭──╮ │\n  │    │██│ │  Ancient Chest\n  │    ╰──╯ │\n  ╰──────────╯\n\nOpen it...",
                "open",
                "󰋖 Type commands to interact with the world.",
            ),
            TutorialStep::new(
                "Inside, you find a worn journal.\nIts pages whisper of forgotten truths.\n\n  ╭────────────────────╮\n  │  LORE FRAGMENT      │\n  │  \"The Silence...\"   │\n  ╰────────────────────╯\n\nTake it with you.",
                "take",
                "󰂺 Lore fragments reveal the world's secrets.",
            ),
        ],
        
        TutorialPhase::Discovery => vec![
            TutorialStep::new(
                "The journal speaks of five factions\nthat once ruled the threshold:\n\n  󰂵 The Silent Order\n  󰋾 The Echoing Choir\n  󰆧 The Gilded Merchants\n  󰛡 The Threshold Wardens\n  󰚌 The Void Touched\n\nYour choices will align you with them...",
                "understand",
                "󰒖 Factions remember how you treat them.",
            ),
            TutorialStep::new(
                "Each run, you will die.\nBut death is not the end.\n\n  ╭────────────────────╮\n  │  󰐀 INK: Currency    │\n  │  Persists through   │\n  │  death. Unlock      │\n  │  permanent bonuses. │\n  ╰────────────────────╯\n\nEmbrace the cycle.",
                "persist",
                "󰐀 Ink and lore survive every death.",
            ),
            TutorialStep::new(
                "You are ready.\n\nThe threshold awaits.\nType fast. Fight hard.\nDiscover the truth.\n\n  ╭────────────────────────────╮\n  │   TUTORIAL COMPLETE        │\n  │                            │\n  │   Press [h] for help       │\n  │   anytime you need it.     │\n  ╰────────────────────────────╯",
                "begin quest",
                "󰓥 Your typing quest begins now!",
            ),
        ],
        
        TutorialPhase::Complete => vec![],
    }
}

// ============================================================================
// TUTORIAL STATE
// ============================================================================

/// Manages the current tutorial progress
#[derive(Debug, Clone)]
pub struct TutorialState {
    /// Current phase
    pub phase: TutorialPhase,
    
    /// Current step within the phase (0-indexed)
    pub step_index: usize,
    
    /// What the player has typed so far
    pub typed_input: String,
    
    /// Whether the current attempt used backspace
    pub used_backspace: bool,
    
    /// Time when current step started
    pub step_started: Option<Instant>,
    
    /// Combo counter for TheCombo phase
    pub combo_count: u32,
    
    /// Time between last word completion (for combo tracking)
    pub last_completion: Option<Instant>,
    
    /// Whether to show success animation
    pub show_success: bool,
    
    /// Success message to display
    pub success_message: Option<String>,
    
    /// Whether tutorial has been completed before (from save)
    pub previously_completed: bool,
}

impl Default for TutorialState {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialState {
    pub fn new() -> Self {
        Self {
            phase: TutorialPhase::Awakening,
            step_index: 0,
            typed_input: String::new(),
            used_backspace: false,
            step_started: None,
            combo_count: 0,
            last_completion: None,
            show_success: false,
            success_message: None,
            previously_completed: false,
        }
    }
    
    /// Start or restart the tutorial
    pub fn start(&mut self) {
        self.phase = TutorialPhase::Awakening;
        self.step_index = 0;
        self.typed_input.clear();
        self.used_backspace = false;
        self.step_started = Some(Instant::now());
        self.combo_count = 0;
        self.last_completion = None;
        self.show_success = false;
        self.success_message = None;
    }
    
    /// Get the current step
    pub fn current_step(&self) -> Option<TutorialStep> {
        let steps = get_phase_steps(self.phase);
        steps.get(self.step_index).cloned()
    }
    
    /// Handle a character being typed
    pub fn on_char(&mut self, c: char) {
        self.typed_input.push(c);
        self.show_success = false;
        
        // Check if word completed
        if let Some(step) = self.current_step() {
            if self.typed_input == step.target_text {
                self.complete_step();
            }
        }
    }
    
    /// Handle backspace
    pub fn on_backspace(&mut self) {
        self.typed_input.pop();
        self.used_backspace = true;
    }
    
    /// Complete the current step and advance
    fn complete_step(&mut self) {
        let was_perfect = !self.used_backspace;
        let elapsed = self.step_started
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(0.0);
        
        // Calculate WPM for this word
        let word_len = self.typed_input.len() as f32;
        let wpm = if elapsed > 0.0 {
            (word_len / 5.0) / (elapsed / 60.0)
        } else {
            0.0
        };
        
        // Update combo for TheCombo phase
        if self.phase == TutorialPhase::TheCombo {
            let combo_window = self.last_completion
                .map(|t| t.elapsed() < Duration::from_secs(2))
                .unwrap_or(true);
            
            if combo_window {
                self.combo_count += 1;
            } else {
                self.combo_count = 1;
            }
            self.last_completion = Some(Instant::now());
        }
        
        // Generate success message
        self.success_message = Some(self.generate_success_message(was_perfect, wpm));
        self.show_success = true;
        
        // Advance to next step
        let steps = get_phase_steps(self.phase);
        if self.step_index + 1 < steps.len() {
            self.step_index += 1;
        } else {
            // Phase complete, advance to next phase
            self.phase = self.phase.next();
            self.step_index = 0;
            self.combo_count = 0;
        }
        
        // Reset for next step
        self.typed_input.clear();
        self.used_backspace = false;
        self.step_started = Some(Instant::now());
    }
    
    /// Generate a success message based on performance
    fn generate_success_message(&self, perfect: bool, wpm: f32) -> String {
        let mut parts = Vec::new();
        
        if perfect {
            parts.push("󰄀 PERFECT!".to_string());
        } else {
            parts.push("󰄬 Good!".to_string());
        }
        
        if wpm > 80.0 {
            parts.push(" 󰈸 Lightning fast!".to_string());
        } else if wpm > 50.0 {
            parts.push(" 󰔚 Quick!".to_string());
        }
        
        if self.combo_count > 1 {
            parts.push(format!(" Combo: {}x", self.combo_count));
        }
        
        parts.join("")
    }
    
    /// Check if tutorial is complete
    pub fn is_complete(&self) -> bool {
        self.phase == TutorialPhase::Complete
    }
    
    /// Get progress as percentage
    pub fn progress_percent(&self) -> u8 {
        let total_steps: usize = [
            TutorialPhase::Awakening,
            TutorialPhase::FirstStrike,
            TutorialPhase::TheCombo,
            TutorialPhase::Choice,
            TutorialPhase::Discovery,
        ].iter()
            .map(|p| get_phase_steps(*p).len())
            .sum();
        
        let completed: usize = [
            TutorialPhase::Awakening,
            TutorialPhase::FirstStrike,
            TutorialPhase::TheCombo,
            TutorialPhase::Choice,
            TutorialPhase::Discovery,
        ].iter()
            .take_while(|&&p| p != self.phase)
            .map(|p| get_phase_steps(*p).len())
            .sum::<usize>() + self.step_index;
        
        ((completed as f32 / total_steps as f32) * 100.0) as u8
    }
    
    /// Skip the entire tutorial
    pub fn skip(&mut self) {
        self.phase = TutorialPhase::Complete;
        self.step_index = 0;
    }
    
    /// Get time remaining for timed steps
    pub fn time_remaining(&self) -> Option<f32> {
        self.current_step().and_then(|step| {
            step.time_limit.map(|limit| {
                let elapsed = self.step_started
                    .map(|t| t.elapsed().as_secs_f32())
                    .unwrap_or(0.0);
                (limit - elapsed).max(0.0)
            })
        })
    }
}

// ============================================================================
// TUTORIAL PROGRESS (Persistent)
// ============================================================================

/// Persistent tutorial progress saved between sessions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TutorialProgress {
    /// Whether the full tutorial has been completed
    pub completed: bool,
    
    /// Highest phase reached
    pub highest_phase: u8,
    
    /// Total words typed in tutorial
    pub words_typed: u32,
    
    /// Best WPM achieved in tutorial
    pub best_wpm: f32,
    
    /// Number of perfect words
    pub perfect_words: u32,
}

impl TutorialProgress {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update progress after tutorial completion
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.highest_phase = 5;
    }
    
    /// Check if player should see tutorial
    pub fn should_show_tutorial(&self) -> bool {
        !self.completed
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tutorial_phases() {
        let mut state = TutorialState::new();
        assert_eq!(state.phase, TutorialPhase::Awakening);
        
        // Complete all awakening steps
        let steps = get_phase_steps(TutorialPhase::Awakening);
        for step in &steps {
            for c in step.target_text.chars() {
                state.on_char(c);
            }
        }
        
        assert_eq!(state.phase, TutorialPhase::FirstStrike);
    }
    
    #[test]
    fn test_backspace_tracking() {
        let mut state = TutorialState::new();
        state.on_char('a');
        assert!(!state.used_backspace);
        
        state.on_backspace();
        assert!(state.used_backspace);
    }
    
    #[test]
    fn test_progress_percent() {
        let mut state = TutorialState::new();
        assert_eq!(state.progress_percent(), 0);
        
        state.phase = TutorialPhase::Complete;
        // After complete, should still calculate based on steps
    }
}

// Compatibility methods for integration
impl TutorialState {
    /// Reset to beginning (alias for start)
    pub fn reset(&mut self) {
        self.start();
    }
    
    /// Check if current step is complete (typed_input matches target)
    pub fn is_step_complete(&self) -> bool {
        if let Some(step) = self.current_step() {
            self.typed_input == step.target_text
        } else {
            false
        }
    }
    
    /// Advance to next step/phase. Returns true if tutorial is now complete.
    pub fn advance(&mut self) -> bool {
        let steps = get_phase_steps(self.phase);
        
        if self.step_index + 1 < steps.len() {
            self.step_index += 1;
        } else {
            // Phase complete, advance to next phase
            self.phase = self.phase.next();
            self.step_index = 0;
            self.combo_count = 0;
        }
        
        // Reset for next step
        self.typed_input.clear();
        self.used_backspace = false;
        self.step_started = Some(Instant::now());
        
        self.is_complete()
    }
    
    /// Type a character
    pub fn type_char(&mut self, c: char) {
        self.on_char(c);
    }
    
    /// Backspace
    pub fn backspace(&mut self) {
        self.on_backspace();
    }
    
    /// Get current phase (for render compatibility)
    pub fn current_phase(&self) -> TutorialPhase {
        self.phase
    }
    
    /// Get current combo
    pub fn current_combo(&self) -> u32 {
        self.combo_count
    }
}

impl TutorialProgress {
    /// Load progress from save file
    pub fn load() -> Self {
        let save_path = dirs::config_dir()
            .map(|p| p.join("typingquest").join("tutorial.json"))
            .unwrap_or_else(|| PathBuf::from("tutorial.json"));
        
        if save_path.exists() {
            if let Ok(data) = std::fs::read_to_string(&save_path) {
                if let Ok(progress) = serde_json::from_str(&data) {
                    return progress;
                }
            }
        }
        
        Self::new()
    }
    
    /// Save progress to file
    pub fn save(&self) {
        let save_path = dirs::config_dir()
            .map(|p| p.join("typingquest").join("tutorial.json"))
            .unwrap_or_else(|| PathBuf::from("tutorial.json"));
        
        if let Some(parent) = save_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&save_path, data);
        }
    }
}
