//! Visual Effects System - The "Juice" that makes combat feel impactful
//!
//! This module provides:
//! - Floating damage numbers
//! - Screen shake effects
//! - Hit flash overlays
//! - Combo pulse animations
//! - Combat message styling

use std::time::Instant;

/// A floating text element (damage numbers, status text, etc.)
#[derive(Debug, Clone)]
pub struct FloatingText {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,
    pub opacity: f32,
    pub color: TextColor,
    pub size: TextSize,
    pub created_at: Instant,
    pub lifetime_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextColor {
    Damage,      // Red
    Critical,    // Yellow/Gold
    Heal,        // Green
    Combo,       // Cyan
    Miss,        // Gray
    Perfect,     // White/Bright
    Bonus,       // Magenta
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    Small,
    Normal,
    Large,
    Huge,
}

impl FloatingText {
    pub fn damage(amount: i32, x: f32, y: f32) -> Self {
        Self {
            text: format!("-{}", amount),
            x,
            y,
            velocity_y: -2.0,
            opacity: 1.0,
            color: TextColor::Damage,
            size: if amount > 50 { TextSize::Large } else { TextSize::Normal },
            created_at: Instant::now(),
            lifetime_ms: 1200,
        }
    }

    pub fn critical(amount: i32, x: f32, y: f32) -> Self {
        Self {
            text: format!("💥 {} CRIT!", amount),
            x,
            y,
            velocity_y: -3.0,
            opacity: 1.0,
            color: TextColor::Critical,
            size: TextSize::Huge,
            created_at: Instant::now(),
            lifetime_ms: 1500,
        }
    }

    pub fn combo(combo: i32, x: f32, y: f32) -> Self {
        let text = match combo {
            1..=2 => format!("{}x", combo),
            3..=5 => format!("{}x COMBO!", combo),
            6..=10 => format!("🔥 {}x STREAK!", combo),
            _ => format!("⚡ {}x UNSTOPPABLE!", combo),
        };
        Self {
            text,
            x,
            y,
            velocity_y: -1.5,
            opacity: 1.0,
            color: TextColor::Combo,
            size: if combo > 5 { TextSize::Large } else { TextSize::Normal },
            created_at: Instant::now(),
            lifetime_ms: 800,
        }
    }

    pub fn perfect(x: f32, y: f32) -> Self {
        Self {
            text: "✨ PERFECT!".to_string(),
            x,
            y,
            velocity_y: -2.5,
            opacity: 1.0,
            color: TextColor::Perfect,
            size: TextSize::Large,
            created_at: Instant::now(),
            lifetime_ms: 1000,
        }
    }

    pub fn attack_name(name: &str, x: f32, y: f32) -> Self {
        Self {
            text: name.to_string(),
            x,
            y,
            velocity_y: -1.0,
            opacity: 1.0,
            color: TextColor::Bonus,
            size: TextSize::Normal,
            created_at: Instant::now(),
            lifetime_ms: 1000,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_millis() as u64 > self.lifetime_ms
    }

    pub fn progress(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_millis() as f32;
        (elapsed / self.lifetime_ms as f32).min(1.0)
    }

    pub fn current_opacity(&self) -> f32 {
        let progress = self.progress();
        // Fade out in last 30% of lifetime
        if progress > 0.7 {
            1.0 - ((progress - 0.7) / 0.3)
        } else {
            1.0
        }
    }

    pub fn current_y(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        self.y + (self.velocity_y * elapsed * 10.0)
    }
}

/// Screen shake effect
#[derive(Debug, Clone)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration_ms: u64,
    pub created_at: Instant,
}

impl ScreenShake {
    pub fn light() -> Self {
        Self {
            intensity: 1.0,
            duration_ms: 100,
            created_at: Instant::now(),
        }
    }

    pub fn medium() -> Self {
        Self {
            intensity: 2.0,
            duration_ms: 150,
            created_at: Instant::now(),
        }
    }

    pub fn heavy() -> Self {
        Self {
            intensity: 3.0,
            duration_ms: 200,
            created_at: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        (self.created_at.elapsed().as_millis() as u64) < self.duration_ms
    }

    /// Returns current offset for shake effect
    pub fn get_offset(&self) -> (i16, i16) {
        if !self.is_active() {
            return (0, 0);
        }

        let progress = self.created_at.elapsed().as_millis() as f32 / self.duration_ms as f32;
        let decay = 1.0 - progress;
        let shake = (self.intensity * decay) as i16;
        
        // Pseudo-random shake based on elapsed time
        let elapsed = self.created_at.elapsed().as_millis() as i16;
        let x = if elapsed % 4 < 2 { shake } else { -shake };
        let y = if elapsed % 3 < 1 { shake / 2 } else { -(shake / 2) };
        
        (x, y)
    }
}

/// Hit flash overlay effect
#[derive(Debug, Clone)]
pub struct HitFlash {
    pub color: FlashColor,
    pub duration_ms: u64,
    pub created_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashColor {
    Red,     // Player hit
    White,   // Enemy hit
    Yellow,  // Critical
    Green,   // Heal
}

impl HitFlash {
    pub fn enemy_hit() -> Self {
        Self {
            color: FlashColor::White,
            duration_ms: 80,
            created_at: Instant::now(),
        }
    }

    pub fn player_hit() -> Self {
        Self {
            color: FlashColor::Red,
            duration_ms: 120,
            created_at: Instant::now(),
        }
    }

    pub fn critical() -> Self {
        Self {
            color: FlashColor::Yellow,
            duration_ms: 150,
            created_at: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        (self.created_at.elapsed().as_millis() as u64) < self.duration_ms
    }

    pub fn intensity(&self) -> f32 {
        if !self.is_active() {
            return 0.0;
        }
        let progress = self.created_at.elapsed().as_millis() as f32 / self.duration_ms as f32;
        1.0 - progress
    }
}

/// Manages all active visual effects
#[derive(Debug, Clone, Default)]
pub struct EffectsManager {
    pub floating_texts: Vec<FloatingText>,
    pub screen_shake: Option<ScreenShake>,
    pub hit_flash: Option<HitFlash>,
    pub combo_pulse: Option<ComboPulse>,
    pub typing_ripple: Option<TypingRipple>,
}

/// Combo counter pulse animation
#[derive(Debug, Clone)]
pub struct ComboPulse {
    pub combo: i32,
    pub created_at: Instant,
}

impl ComboPulse {
    pub fn new(combo: i32) -> Self {
        Self {
            combo,
            created_at: Instant::now(),
        }
    }

    pub fn scale(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_millis() as f32;
        if elapsed < 100.0 {
            // Expand
            1.0 + (elapsed / 100.0) * 0.3
        } else if elapsed < 200.0 {
            // Contract
            1.3 - ((elapsed - 100.0) / 100.0) * 0.3
        } else {
            1.0
        }
    }

    pub fn is_active(&self) -> bool {
        self.created_at.elapsed().as_millis() < 200
    }
}

/// Ripple effect when typing
#[derive(Debug, Clone)]
pub struct TypingRipple {
    pub correct: bool,
    pub created_at: Instant,
}

impl TypingRipple {
    pub fn correct() -> Self {
        Self {
            correct: true,
            created_at: Instant::now(),
        }
    }

    pub fn incorrect() -> Self {
        Self {
            correct: false,
            created_at: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.created_at.elapsed().as_millis() < 150
    }
}

impl EffectsManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all effects, removing expired ones
    pub fn update(&mut self) {
        // Remove expired floating texts
        self.floating_texts.retain(|t| !t.is_expired());

        // Clear expired shake
        if let Some(ref shake) = self.screen_shake {
            if !shake.is_active() {
                self.screen_shake = None;
            }
        }

        // Clear expired flash
        if let Some(ref flash) = self.hit_flash {
            if !flash.is_active() {
                self.hit_flash = None;
            }
        }

        // Clear expired pulse
        if let Some(ref pulse) = self.combo_pulse {
            if !pulse.is_active() {
                self.combo_pulse = None;
            }
        }

        // Clear expired ripple
        if let Some(ref ripple) = self.typing_ripple {
            if !ripple.is_active() {
                self.typing_ripple = None;
            }
        }
    }

    /// Add a damage number
    pub fn add_damage(&mut self, amount: i32, is_crit: bool) {
        let x = 0.5; // Center
        let y = 0.3; // Upper area
        
        if is_crit {
            self.floating_texts.push(FloatingText::critical(amount, x, y));
            self.screen_shake = Some(ScreenShake::heavy());
            self.hit_flash = Some(HitFlash::critical());
        } else {
            self.floating_texts.push(FloatingText::damage(amount, x, y));
            self.screen_shake = Some(ScreenShake::light());
            self.hit_flash = Some(HitFlash::enemy_hit());
        }
    }

    /// Add a combo indicator
    pub fn add_combo(&mut self, combo: i32) {
        if combo > 1 {
            self.floating_texts.push(FloatingText::combo(combo, 0.8, 0.5));
            self.combo_pulse = Some(ComboPulse::new(combo));
        }
    }

    /// Add a perfect word indicator
    pub fn add_perfect(&mut self) {
        self.floating_texts.push(FloatingText::perfect(0.5, 0.4));
    }

    /// Add attack name display
    pub fn add_attack_name(&mut self, name: &str) {
        self.floating_texts.push(FloatingText::attack_name(name, 0.5, 0.35));
    }

    /// Player took damage
    pub fn player_hit(&mut self, amount: i32) {
        self.screen_shake = Some(ScreenShake::medium());
        self.hit_flash = Some(HitFlash::player_hit());
        self.floating_texts.push(FloatingText {
            text: format!("-{}", amount),
            x: 0.2,
            y: 0.7,
            velocity_y: -1.5,
            opacity: 1.0,
            color: TextColor::Damage,
            size: TextSize::Normal,
            created_at: Instant::now(),
            lifetime_ms: 1000,
        });
    }

    /// Keystroke feedback
    pub fn keystroke(&mut self, correct: bool) {
        self.typing_ripple = Some(if correct {
            TypingRipple::correct()
        } else {
            TypingRipple::incorrect()
        });
    }

    /// Check if any effects are active
    pub fn has_active_effects(&self) -> bool {
        !self.floating_texts.is_empty()
            || self.screen_shake.is_some()
            || self.hit_flash.is_some()
            || self.combo_pulse.is_some()
            || self.typing_ripple.is_some()
    }
}

/// Combat atmosphere text that appears contextually
#[derive(Debug, Clone)]
pub struct AtmosphereText {
    pub lines: Vec<String>,
    pub style: AtmosphereStyle,
    pub created_at: Instant,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtmosphereStyle {
    Tension,     // Combat is heating up
    Breather,    // Moment of calm
    Victory,     // Enemy defeated
    Danger,      // Low HP warning
    Boss,        // Boss encounter
}

impl AtmosphereText {
    pub fn tension(line: &str) -> Self {
        Self {
            lines: vec![line.to_string()],
            style: AtmosphereStyle::Tension,
            created_at: Instant::now(),
            duration_ms: 2000,
        }
    }

    pub fn boss_intro(name: &str, tagline: &str) -> Self {
        Self {
            lines: vec![
                format!("⚔️  {}  ⚔️", name),
                tagline.to_string(),
            ],
            style: AtmosphereStyle::Boss,
            created_at: Instant::now(),
            duration_ms: 3000,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_millis() as u64 > self.duration_ms
    }

    pub fn opacity(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_millis() as f32;
        let total = self.duration_ms as f32;
        
        // Fade in first 20%, hold, fade out last 20%
        if elapsed < total * 0.2 {
            elapsed / (total * 0.2)
        } else if elapsed > total * 0.8 {
            1.0 - ((elapsed - total * 0.8) / (total * 0.2))
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floating_text_damage() {
        let text = FloatingText::damage(25, 0.5, 0.5);
        assert_eq!(text.text, "-25");
        assert!(!text.is_expired());
    }

    #[test]
    fn test_screen_shake() {
        let shake = ScreenShake::heavy();
        assert!(shake.is_active());
        let (x, y) = shake.get_offset();
        assert!(x.abs() <= 3 && y.abs() <= 3);
    }

    #[test]
    fn test_effects_manager() {
        let mut mgr = EffectsManager::new();
        mgr.add_damage(50, false);
        assert!(!mgr.floating_texts.is_empty());
        assert!(mgr.screen_shake.is_some());
    }

    #[test]
    fn test_combo_pulse() {
        let pulse = ComboPulse::new(5);
        assert!(pulse.is_active());
        assert!(pulse.scale() >= 1.0);
    }
}
