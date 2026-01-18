//! Combat Immersion System - Wires together all immersion overhaul systems
//!
//! This module integrates:
//! - TypingImpact: Per-keystroke damage with attack types
//! - DialogueEngine: Contextual enemy dialogue
//! - EnemyVisuals: Progressive damage visualization
//! - PacingController: Tension and breather beats
//! - PlayerAvatar: Player visual presence
//!
//! Usage: Create ImmersiveCombat alongside CombatState for enhanced feedback

use super::typing_impact::{TypingImpact, AttackType, KeystrokeResult, WordCompletionResult};
use super::dialogue_engine::{DialogueEngine, DialogueContext, CombatMomentum, PlayerMomentum, ZoneContext};
use super::enemy_visuals::{EnemyVisualState, EnemyPosture, HitLocation};
use super::pacing::{PacingController, PacingPhase, PacingBeat};
use super::player_avatar::{PlayerAvatar, PlayerClass, AvatarState};
use rand::prelude::*;

/// Immersive combat wrapper - enhances standard CombatState with rich feedback
#[derive(Debug, Clone)]
pub struct ImmersiveCombat {
    /// Typing impact tracker
    pub typing: TypingImpact,
    /// Dialogue generation
    pub dialogue: DialogueEngine,
    /// Enemy visual state
    pub enemy_visuals: EnemyVisualState,
    /// Pacing controller
    pub pacing: PacingController,
    /// Player avatar
    pub player: PlayerAvatar,
    /// Current enemy theme for dialogue
    pub enemy_theme: String,
    /// Current enemy name
    pub enemy_name: String,
    /// Current floor
    pub floor: u32,
    /// Player health percent
    pub player_health_percent: i32,
    /// Player accuracy this combat
    pub accuracy: f32,
    /// Last keystroke feedback
    pub last_keystroke_feedback: Option<KeystrokeFeedback>,
    /// Last word completion feedback
    pub last_word_feedback: Option<WordFeedback>,
    /// Pending combat messages (dialogue, atmosphere, etc.)
    pub pending_messages: Vec<CombatMessage>,
    /// Is this a boss fight
    pub is_boss: bool,
    /// Current typing WPM
    pub current_wpm: f32,
}

/// Feedback for a single keystroke
#[derive(Debug, Clone)]
pub struct KeystrokeFeedback {
    pub character: char,
    pub correct: bool,
    pub damage_dealt: i32,
    pub rhythm_bonus: bool,
    pub speed_rating: SpeedRating,
}

/// How fast was that keystroke?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedRating {
    Blazing,  // < 50ms
    Fast,     // < 100ms
    Normal,   // < 200ms
    Slow,     // < 400ms
    Hesitant, // >= 400ms
}

impl SpeedRating {
    pub fn from_interval(ms: u64) -> Self {
        match ms {
            0..=50 => Self::Blazing,
            51..=100 => Self::Fast,
            101..=200 => Self::Normal,
            201..=400 => Self::Slow,
            _ => Self::Hesitant,
        }
    }
    
    pub fn color_hint(&self) -> &'static str {
        match self {
            Self::Blazing => "yellow",
            Self::Fast => "green",
            Self::Normal => "white",
            Self::Slow => "gray",
            Self::Hesitant => "dark_gray",
        }
    }
}

/// Feedback for completing a word
#[derive(Debug, Clone)]
pub struct WordFeedback {
    pub attack_type: AttackType,
    pub total_damage: i32,
    pub message: String,
    pub enemy_reaction: String,
    pub enemy_new_posture: EnemyPosture,
    pub was_kill: bool,
}

/// A combat message to display
#[derive(Debug, Clone)]
pub struct CombatMessage {
    pub text: String,
    pub style: MessageStyle,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStyle {
    PlayerAction,
    EnemyAction,
    EnemyDialogue,
    SystemInfo,
    Atmosphere,
    Critical,
    Victory,
}

impl Default for ImmersiveCombat {
    fn default() -> Self {
        Self::new("Unknown".to_string(), "unknown".to_string(), 1, false, PlayerClass::Freelancer)
    }
}

impl ImmersiveCombat {
    /// Create a new immersive combat instance
    pub fn new(
        enemy_name: String,
        enemy_theme: String,
        floor: u32,
        is_boss: bool,
        player_class: PlayerClass,
    ) -> Self {
        let mut pacing = PacingController::new();
        pacing.on_combat_start(is_boss);
        
        Self {
            typing: TypingImpact::new(),
            dialogue: DialogueEngine::new(),
            enemy_visuals: EnemyVisualState::new(vec![
                "  /\\_/\\  ".to_string(),
                " ( o.o ) ".to_string(),
                "  > ^ <  ".to_string(),
            ]), // Default art, should be set from actual enemy
            pacing,
            player: PlayerAvatar::new(player_class),
            enemy_theme,
            enemy_name,
            floor,
            player_health_percent: 100,
            accuracy: 1.0,
            last_keystroke_feedback: None,
            last_word_feedback: None,
            pending_messages: Vec::new(),
            is_boss,
            current_wpm: 0.0,
        }
    }
    
    /// Set enemy ASCII art
    pub fn set_enemy_art(&mut self, art: Vec<String>) {
        self.enemy_visuals = EnemyVisualState::new(art);
    }
    
    /// Called when player starts typing a new word
    pub fn start_word(&mut self, word: &str) {
        self.typing.start_word(word.to_string());
        self.last_keystroke_feedback = None;
        self.last_word_feedback = None;
    }
    
    /// Called on each keystroke - returns feedback
    pub fn on_keystroke(&mut self, c: char, correct: bool) -> KeystrokeFeedback {
        let result = self.typing.on_keystroke(c, correct);
        
        // Trigger player typing animation
        self.player.on_keystroke();
        
        // Estimate interval from visual intensity (higher = faster = shorter interval)
        let estimated_interval = ((1.0 - result.visual_intensity) * 400.0) as u64;
        let speed_rating = SpeedRating::from_interval(estimated_interval);
        
        let feedback = KeystrokeFeedback {
            character: c,
            correct,
            damage_dealt: result.damage_this_stroke as i32,
            rhythm_bonus: result.rhythm_bonus > 0.0,
            speed_rating,
        };
        
        self.last_keystroke_feedback = Some(feedback.clone());
        feedback
    }
    
    /// Called when word is completed - returns comprehensive feedback
    pub fn on_word_complete(&mut self, enemy_health_percent: i32, base_damage: i32, current_wpm: f32) -> WordFeedback {
        self.current_wpm = current_wpm;
        let completion = self.typing.complete_word(base_damage);
        
        // Update dialogue context
        let ctx = self.build_dialogue_context(enemy_health_percent);
        
        // Generate contextual hit message
        let message = self.dialogue.generate_hit_message(&ctx, completion.damage, &completion.attack_type);
        
        // Apply damage to visual state (convert damage to pct for visualization)
        let hit_location = self.random_hit_location();
        let damage_pct = (completion.damage as f32 / 100.0).min(1.0); // Normalize
        self.enemy_visuals.apply_damage(damage_pct, hit_location);
        
        // Update enemy posture from health
        self.enemy_visuals.update_from_health(enemy_health_percent as f32);
        let new_posture = self.enemy_visuals.get_posture();
        
        // Trigger player attack animation
        self.player.on_attack();
        
        // Check for kill
        let was_kill = enemy_health_percent <= 0;
        
        // Generate enemy reaction
        let enemy_reaction = if was_kill {
            self.dialogue.generate_death_message(&ctx)
        } else {
            // Maybe enemy taunts
            self.dialogue.generate_enemy_taunt(&ctx).unwrap_or_default()
        };
        
        // Add to pending messages
        self.pending_messages.push(CombatMessage {
            text: message.clone(),
            style: MessageStyle::PlayerAction,
            duration_ms: 2000,
        });
        
        if !enemy_reaction.is_empty() {
            self.pending_messages.push(CombatMessage {
                text: enemy_reaction.clone(),
                style: if was_kill { MessageStyle::Victory } else { MessageStyle::EnemyDialogue },
                duration_ms: if was_kill { 3000 } else { 2000 },
            });
        }
        
        // Victory animation
        if was_kill {
            self.player.on_victory();
            self.pacing.on_combat_end(true, self.is_boss);
        }
        
        let feedback = WordFeedback {
            attack_type: completion.attack_type,
            total_damage: completion.damage,
            message,
            enemy_reaction,
            enemy_new_posture: new_posture,
            was_kill,
        };
        
        self.last_word_feedback = Some(feedback.clone());
        feedback
    }
    
    /// Called when enemy attacks player
    pub fn on_enemy_attack(&mut self, damage: i32, enemy_health_percent: i32) -> String {
        let ctx = self.build_dialogue_context(enemy_health_percent);
        let message = self.dialogue.generate_enemy_attack(&ctx, damage);
        
        // Player takes hit
        self.player.on_hit();
        
        self.pending_messages.push(CombatMessage {
            text: message.clone(),
            style: MessageStyle::EnemyAction,
            duration_ms: 2000,
        });
        
        message
    }
    
    /// Called when player takes damage (update health for visuals)
    pub fn on_player_damaged(&mut self, health_percent: i32) {
        self.player_health_percent = health_percent;
        self.player.update_health(health_percent as u32);
    }
    
    /// Update accuracy tracking
    pub fn update_accuracy(&mut self, accuracy: f32) {
        self.accuracy = accuracy;
    }
    
    /// Generate combat intro message
    pub fn generate_intro(&mut self, enemy_health_percent: i32) -> String {
        let ctx = self.build_dialogue_context(enemy_health_percent);
        let intro = self.dialogue.generate_combat_intro(&ctx);
        
        self.pending_messages.push(CombatMessage {
            text: intro.clone(),
            style: MessageStyle::SystemInfo,
            duration_ms: 2500,
        });
        
        intro
    }
    
    /// Get rendered enemy art with damage overlays
    pub fn render_enemy(&mut self) -> Vec<String> {
        self.enemy_visuals.render()
    }
    
    /// Get enemy art without caching (for read-only rendering)
    pub fn render_enemy_readonly(&self) -> Vec<String> {
        self.enemy_visuals.render_readonly()
    }
    
    /// Get player avatar art
    pub fn render_player(&self) -> Vec<&'static str> {
        self.player.get_art()
    }
    
    /// Update animations (call each frame)
    pub fn update(&mut self, delta_ms: u32) {
        self.player.update(delta_ms);
    }
    
    /// Get any pending pacing beat
    pub fn get_pacing_beat(&mut self) -> Option<PacingBeat> {
        self.pacing.pop_beat()
    }
    
    /// Get current tension level (0-100)
    pub fn get_tension(&self) -> i32 {
        self.pacing.get_tension()
    }
    
    /// Pop next pending message
    pub fn pop_message(&mut self) -> Option<CombatMessage> {
        if self.pending_messages.is_empty() {
            None
        } else {
            Some(self.pending_messages.remove(0))
        }
    }
    
    /// Check if there are pending messages
    pub fn has_pending_messages(&self) -> bool {
        !self.pending_messages.is_empty()
    }
    
    /// Get attack type icon for display
    pub fn get_attack_icon(&self) -> &'static str {
        if let Some(ref feedback) = self.last_word_feedback {
            feedback.attack_type.icon()
        } else {
            ""
        }
    }
    
    /// Get a formatted damage number with style hints
    pub fn format_damage(&self, damage: i32) -> String {
        if damage >= 30 {
            format!("💥 {} 💥", damage)
        } else if damage >= 20 {
            format!("⚡ {} ⚡", damage)
        } else if damage >= 10 {
            format!("✦ {} ✦", damage)
        } else {
            format!("{}", damage)
        }
    }
    
    // Helper functions
    
    fn build_dialogue_context(&self, enemy_health_percent: i32) -> DialogueContext {
        DialogueContext {
            enemy_name: self.enemy_name.clone(),
            enemy_theme: self.enemy_theme.clone(),
            enemy_momentum: CombatMomentum::from_health_percent(enemy_health_percent),
            player_momentum: PlayerMomentum::from_health_and_accuracy(
                self.player_health_percent,
                self.accuracy,
            ),
            zone: ZoneContext::from_floor(self.floor),
            typing_speed: self.current_wpm,
            accuracy: self.accuracy,
        }
    }
    
    fn random_hit_location(&self) -> HitLocation {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => HitLocation::Head,
            1 => HitLocation::Torso,
            2 => HitLocation::LeftArm,
            3 => HitLocation::RightArm,
            _ => HitLocation::Legs,
        }
    }
}

/// Helper to determine enemy theme from name/type
pub fn infer_enemy_theme(name: &str) -> String {
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("goblin") || name_lower.contains("gremlin") {
        "goblin".to_string()
    } else if name_lower.contains("skeleton") || name_lower.contains("zombie") 
           || name_lower.contains("undead") || name_lower.contains("ghoul") {
        "undead".to_string()
    } else if name_lower.contains("ghost") || name_lower.contains("phantom") 
           || name_lower.contains("specter") || name_lower.contains("spirit")
           || name_lower.contains("wraith") {
        "spectral".to_string()
    } else if name_lower.contains("corrupt") || name_lower.contains("tainted")
           || name_lower.contains("blight") || name_lower.contains("fungal") {
        "corrupted".to_string()
    } else if name_lower.contains("construct") || name_lower.contains("automaton")
           || name_lower.contains("clockwork") || name_lower.contains("golem") {
        "mechanical".to_string()
    } else if name_lower.contains("void") || name_lower.contains("aberration")
           || name_lower.contains("eldritch") || name_lower.contains("horror") {
        "void".to_string()
    } else {
        "unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_immersive_combat_creation() {
        let combat = ImmersiveCombat::new(
            "Goblin Scout".to_string(),
            "goblin".to_string(),
            1,
            false,
            PlayerClass::Freelancer,
        );
        
        assert_eq!(combat.enemy_name, "Goblin Scout");
        assert_eq!(combat.floor, 1);
        assert!(!combat.is_boss);
    }
    
    #[test]
    fn test_keystroke_feedback() {
        let mut combat = ImmersiveCombat::new(
            "Test Enemy".to_string(),
            "goblin".to_string(),
            1,
            false,
            PlayerClass::Wordsmith,
        );
        
        combat.start_word("hello");
        let feedback = combat.on_keystroke('h', true);
        
        assert!(feedback.correct);
        assert!(feedback.damage_dealt >= 0);
    }
    
    #[test]
    fn test_word_completion_feedback() {
        let mut combat = ImmersiveCombat::new(
            "Skeleton".to_string(),
            "undead".to_string(),
            2,
            false,
            PlayerClass::Codebreaker,
        );
        
        combat.start_word("test");
        combat.on_keystroke('t', true);
        combat.on_keystroke('e', true);
        combat.on_keystroke('s', true);
        combat.on_keystroke('t', true);
        
        let feedback = combat.on_word_complete(50, 10, 60.0);
        
        assert!(feedback.total_damage > 0);
        assert!(!feedback.message.is_empty());
        assert!(!feedback.was_kill);
    }
    
    #[test]
    fn test_enemy_theme_inference() {
        assert_eq!(infer_enemy_theme("Goblin Scout"), "goblin");
        assert_eq!(infer_enemy_theme("Ancient Skeleton"), "undead");
        assert_eq!(infer_enemy_theme("Void Aberration"), "void");
        assert_eq!(infer_enemy_theme("Clockwork Guardian"), "mechanical");
        assert_eq!(infer_enemy_theme("Corrupted Treant"), "corrupted");
        assert_eq!(infer_enemy_theme("Phantom Warden"), "spectral");
    }
}
