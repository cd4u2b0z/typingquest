# Immersion Overhaul Plan: Making Combat Matter

*"Every keystroke should tell a story. Every enemy death should feel earned."*

---

## ✅ IMPLEMENTATION COMPLETE — v0.5.1 (2026-01-18)

All planned systems have been implemented:

| System | Status | Lines | Module |
|--------|--------|-------|--------|
| Typing Impact | ✅ Complete | 372 | typing_impact.rs |
| Dialogue Engine | ✅ Complete | 384 | dialogue_engine.rs |
| Enemy Visuals | ✅ Complete | 412 | enemy_visuals.rs |
| Pacing System | ✅ Complete | 370 | pacing.rs |
| Player Avatar | ✅ Complete | 381 | player_avatar.rs |
| Combat Immersion | ✅ Complete | 498 | combat_immersion.rs |
| **Total** | **✅** | **2,417** | **6 modules** |

Integration into CombatState complete with readonly rendering support.

---

## Executive Summary

Your game has **excellent architectural bones** — 794 lines of voice system, 540 lines of writing guidelines, 550 lines of typing feel, a full event bus. But these systems whisper when they should shout. The player types into a void and numbers change.

This document provides a concrete plan to:
1. Make dialogue feel alive and contextual
2. Slow pacing to build atmosphere
3. Create visual enemy damage states
4. Fuse typing directly to combat impact
5. Put the player on screen with presence

---

## Part 1: Narrative & Dialogue Overhaul

### Problem Analysis

Your `voice_system.rs` (794 lines) and `writing_guidelines.rs` (540 lines) are **dormant gold**. You have:
- Faction-specific vocabulary, metaphors, idioms
- Gene Wolfe / Le Guin / Dark Souls writing principles coded
- Sentence patterns with contextual weights

But combat dialogue is currently:
```rust
// Current: Generic, disconnected
self.battle_log.push(format!("✦ {} deals {} damage!", spell.name, damage));
```

### Solution: Contextual Dialogue Engine

#### Step 1: Create `DialogueContext` Struct

```rust
// src/game/dialogue_engine.rs (new file)

use super::voice_system::FactionVoice;
use super::enemy::Enemy;
use super::writing_guidelines::WritingPrinciples;

/// Rich context for generating contextual dialogue
pub struct DialogueContext<'a> {
    /// The enemy being fought
    pub enemy: &'a Enemy,
    /// Enemy's current health percentage
    pub enemy_health_pct: f32,
    /// Combat phase (fresh, bloodied, desperate, dying)
    pub combat_phase: CombatMomentum,
    /// Player's current momentum
    pub player_momentum: PlayerMomentum,
    /// Floor/zone for environmental flavor
    pub zone: ZoneContext,
    /// How many turns have passed
    pub turn_count: i32,
    /// Player's combo level
    pub combo: i32,
    /// Player's recent typing performance
    pub typing_quality: TypingQuality,
}

#[derive(Debug, Clone, Copy)]
pub enum CombatMomentum {
    Fresh,      // 100-75% HP
    Bloodied,   // 75-50% HP
    Desperate,  // 50-25% HP
    Dying,      // <25% HP
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerMomentum {
    Dominant,   // Combo > 5, high accuracy
    Confident,  // Combo 2-5, decent accuracy
    Struggling, // Combo broken, errors
    Critical,   // Low HP, desperate
}

#[derive(Debug, Clone)]
pub struct TypingQuality {
    pub wpm: f32,
    pub accuracy: f32,
    pub perfect_words: i32,
    pub errors_this_turn: i32,
}

impl CombatMomentum {
    pub fn from_health(current: i32, max: i32) -> Self {
        let pct = current as f32 / max as f32;
        match pct {
            p if p > 0.75 => Self::Fresh,
            p if p > 0.50 => Self::Bloodied,
            p if p > 0.25 => Self::Desperate,
            _ => Self::Dying,
        }
    }
}
```

#### Step 2: Enemy-Specific Dialogue Banks

Each enemy type gets dialogue that reflects their personality:

```rust
// src/data/enemy_dialogue.rs (new file)

use std::collections::HashMap;

/// Dialogue banks organized by enemy and combat state
pub struct EnemyDialogueBanks {
    banks: HashMap<String, EnemyDialogueBank>,
}

pub struct EnemyDialogueBank {
    /// What enemy says when appearing
    pub intro: Vec<String>,
    /// Idle taunts during combat (by momentum)
    pub taunts_fresh: Vec<String>,
    pub taunts_bloodied: Vec<String>,
    pub taunts_desperate: Vec<String>,
    /// Reactions to being hit (by damage severity)
    pub hit_light: Vec<String>,      // <10% HP lost
    pub hit_medium: Vec<String>,     // 10-25% HP lost
    pub hit_heavy: Vec<String>,      // >25% HP lost
    pub hit_critical: Vec<String>,   // Critical hit
    /// Attack dialogue
    pub attack_normal: Vec<String>,
    pub attack_enraged: Vec<String>,
    /// Death dialogue (multiple for variety)
    pub death: Vec<String>,
    /// Spare dialogue (if sparable)
    pub spare: Vec<String>,
}

impl EnemyDialogueBanks {
    pub fn load() -> Self {
        let mut banks = HashMap::new();
        
        // === HOLLOW KNIGHT (Boss) ===
        banks.insert("The Hollow Knight".to_string(), EnemyDialogueBank {
            intro: vec![
                "* The knight's empty helm turns toward you. Silence speaks louder than words.".into(),
                "* An oath-bound shade. Still guarding a kingdom of dust.".into(),
            ],
            taunts_fresh: vec![
                "* The knight assumes a defensive stance, patient as stone.".into(),
                "* Its sword hums with old magic, waiting.".into(),
            ],
            taunts_bloodied: vec![
                "* The knight's movements grow sharper. Instinct over honor.".into(),
                "* Something flickers in that empty helm. Recognition?".into(),
            ],
            taunts_desperate: vec![
                "* The knight staggers, then steadies. Duty endures.".into(),
                "* \"...still...I...stand...\" The voice is rust and regret.".into(),
            ],
            hit_light: vec![
                "* The blade glances off ancient armor.".into(),
                "* The knight absorbs the blow without flinching.".into(),
            ],
            hit_medium: vec![
                "* Sparks fly. The knight recoils, briefly.".into(),
                "* A crack spreads across the knight's breastplate.".into(),
            ],
            hit_heavy: vec![
                "* The knight buckles! Dust pours from the wound.".into(),
                "* A sound like breaking glass. The knight's form wavers.".into(),
            ],
            hit_critical: vec![
                "* DEVASTATING BLOW! The knight crashes to one knee.".into(),
                "* The helm cracks. For an instant, you see nothing inside.".into(),
            ],
            attack_normal: vec![
                "* The knight's blade descends in a perfect arc.".into(),
                "* A thrust, economical and deadly.".into(),
            ],
            attack_enraged: vec![
                "* The knight ROARS — wordless, bottomless grief.".into(),
                "* Abandon replaces discipline. The blade becomes a storm.".into(),
            ],
            death: vec![
                "* The knight kneels. The sword plants in the ground — a gravestone.".into(),
                "* \"...my...oath...kept...\" The helm falls empty.".into(),
                "* Duty ends. The armor crumbles to reveal nothing at all.".into(),
            ],
            spare: vec![
                "* The knight lowers its sword. A nod — soldier to soldier.".into(),
                "* \"...the...next...guardian...\" It steps aside.".into(),
            ],
        });
        
        // === GOBLIN SCROUNGER (Common) ===
        banks.insert("Goblin Scrounger".to_string(), EnemyDialogueBank {
            intro: vec![
                "* A goblin leaps from the shadows! \"SHINY! MINE!\"".into(),
                "* Yellow eyes gleam with avarice. \"What you got, yes?\"".into(),
            ],
            taunts_fresh: vec![
                "* \"Gonna take your coins! All of 'em!\"".into(),
                "* The goblin bounces on its heels, blade twitching.".into(),
            ],
            taunts_bloodied: vec![
                "* \"Ow! OW! Not fair fighting!\"".into(),
                "* The goblin's confidence wavers. \"Maybe... we share?\"".into(),
            ],
            taunts_desperate: vec![
                "* \"NO! MY SHINIES!\" Panic edges into the voice.".into(),
                "* The goblin backs toward the exit, eyes darting.".into(),
            ],
            hit_light: vec![
                "* \"Ack!\" The goblin dances backward.".into(),
                "* A glancing blow. The goblin hisses.".into(),
            ],
            hit_medium: vec![
                "* \"AIEEE!\" The goblin clutches the wound.".into(),
                "* The goblin's stolen armor dents inward.".into(),
            ],
            hit_heavy: vec![
                "* The goblin FLIES backward, scattering coins.".into(),
                "* A yelp cut short. The goblin crumples.".into(),
            ],
            hit_critical: vec![
                "* DEVASTATING! The goblin spins twice before hitting the ground.".into(),
            ],
            attack_normal: vec![
                "* The goblin lunges with its rusty shiv!".into(),
                "* Quick fingers reach for your coin purse!".into(),
            ],
            attack_enraged: vec![
                "* \"GRAAAH!\" The goblin abandons greed for fury!".into(),
            ],
            death: vec![
                "* \"My... shinies...\" The goblin's grip loosens. Coins scatter.".into(),
                "* The goblin falls. A pitiful pile of stolen goods spills out.".into(),
            ],
            spare: vec![
                "* \"Fine! FINE! Take half!\" The goblin hurls coins at you and flees.".into(),
            ],
        });
        
        Self { banks }
    }
    
    pub fn get(&self, enemy_name: &str) -> Option<&EnemyDialogueBank> {
        self.banks.get(enemy_name)
    }
}
```

#### Step 3: Wire Dialogue to Combat Events

Modify `combat.rs` to emit contextual dialogue:

```rust
// In CombatState::on_word_complete() or equivalent
fn generate_hit_dialogue(&self, damage: i32, is_crit: bool) -> String {
    let banks = self.dialogue_banks.get(&self.enemy.name);
    let momentum = CombatMomentum::from_health(self.enemy.current_hp, self.enemy.max_hp);
    
    let damage_pct = damage as f32 / self.enemy.max_hp as f32;
    
    if let Some(bank) = banks {
        let pool = if is_crit {
            &bank.hit_critical
        } else if damage_pct > 0.25 {
            &bank.hit_heavy
        } else if damage_pct > 0.10 {
            &bank.hit_medium
        } else {
            &bank.hit_light
        };
        
        let mut rng = rand::thread_rng();
        pool.choose(&mut rng).cloned().unwrap_or_else(|| {
            format!("You deal {} damage!", damage)
        })
    } else {
        // Fallback for enemies without dialogue banks
        if is_crit {
            format!("CRITICAL! {} damage!", damage)
        } else {
            format!("You deal {} damage.", damage)
        }
    }
}
```

---

## Part 2: Story Pacing — The Breath Between Battles

### Problem

Combat → Combat → Combat → Shop → Combat

There's no time to absorb the world. Tension can't build if it never releases.

### Solution: Pacing Beats System

```rust
// src/game/pacing.rs (new file)

use serde::{Deserialize, Serialize};

/// Narrative pacing controller
#[derive(Debug, Clone)]
pub struct PacingController {
    /// Encounters since last rest
    pub tension_level: i32,
    /// Combat rounds this floor
    pub intensity_this_floor: i32,
    /// Current pacing phase
    pub phase: PacingPhase,
    /// Beats waiting to trigger
    pub pending_beats: Vec<PacingBeat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacingPhase {
    /// Slow moments of discovery
    Exploration,
    /// Building toward conflict
    RisingTension,
    /// Active combat/danger
    Confrontation,
    /// Recovery after conflict
    Resolution,
    /// Story/character moment
    Interlude,
}

#[derive(Debug, Clone)]
pub enum PacingBeat {
    /// Atmospheric description (no input needed)
    AtmosphericMoment { text: String, duration_ms: u32 },
    /// Environmental detail the player notices
    EnvironmentalDetail { text: String, examine_option: bool },
    /// Internal thought from the player character
    InternalThought { text: String },
    /// Distant sound or hint of danger
    OminousHint { text: String },
    /// Memory fragment (for the player's hidden past)
    MemoryFlash { text: String, triggers_lore: bool },
    /// NPC encounter (non-hostile)
    NPCMoment { npc_name: String, dialogue: Vec<String> },
}

impl PacingController {
    pub fn new() -> Self {
        Self {
            tension_level: 0,
            intensity_this_floor: 0,
            phase: PacingPhase::Exploration,
            pending_beats: Vec::new(),
        }
    }
    
    /// Called after combat ends
    pub fn on_combat_end(&mut self, was_victory: bool, was_boss: bool) {
        self.intensity_this_floor += if was_boss { 3 } else { 1 };
        
        // After intensity builds, insert a breather
        if self.intensity_this_floor >= 3 && was_victory {
            self.queue_breather_beat();
        }
    }
    
    /// Called when entering a new room
    pub fn on_room_enter(&mut self, room_type: &str, floor: u32) {
        // Don't always rush to combat — queue atmospheric moments
        if self.phase == PacingPhase::Exploration {
            self.queue_atmospheric_beat(floor);
        }
    }
    
    fn queue_breather_beat(&mut self) {
        self.pending_beats.push(PacingBeat::InternalThought {
            text: "You pause. Let your breathing slow. The silence after battle is its own kind of music.".into(),
        });
        self.phase = PacingPhase::Resolution;
    }
    
    fn queue_atmospheric_beat(&mut self, floor: u32) {
        let beat = match floor {
            1..=2 => PacingBeat::EnvironmentalDetail {
                text: "Dust motes drift through shafts of pale light. The Shattered Halls remember grandeur.".into(),
                examine_option: true,
            },
            3..=4 => PacingBeat::AtmosphericMoment {
                text: "Water drips somewhere in the darkness. The Sunken Archives hold their breath.".into(),
                duration_ms: 2000,
            },
            5..=6 => PacingBeat::OminousHint {
                text: "Something moves in the undergrowth. Not hostile. Not yet.".into(),
            },
            _ => PacingBeat::EnvironmentalDetail {
                text: "The air tastes of old magic and older regret.".into(),
                examine_option: false,
            },
        };
        self.pending_beats.push(beat);
    }
    
    /// Get the next pacing beat to display (if any)
    pub fn pop_beat(&mut self) -> Option<PacingBeat> {
        self.pending_beats.pop()
    }
}
```

### Implementation: Room Transitions with Breath

```rust
// Modify dungeon exploration to respect pacing
impl GameState {
    pub fn enter_room(&mut self) {
        // Notify pacing controller FIRST
        self.pacing.on_room_enter(&self.current_room_type(), self.floor);
        
        // Check for pending atmospheric beat
        if let Some(beat) = self.pacing.pop_beat() {
            match beat {
                PacingBeat::AtmosphericMoment { text, duration_ms } => {
                    // Display text, pause before proceeding
                    self.show_atmospheric_text(&text, duration_ms);
                    return; // Don't immediately spawn combat
                }
                PacingBeat::InternalThought { text } => {
                    self.battle_log.push(format!("* {}", text));
                }
                // ... handle other beat types
            }
        }
        
        // Then proceed with normal room logic
        self.spawn_room_content();
    }
}
```

---

## Part 3: Visual Enemy Damage States

### Problem

Enemy HP bar goes down. Nothing else changes. A 10% HP boss looks identical to a 100% HP boss.

### Solution: Progressive ASCII Art Deterioration

```rust
// src/game/enemy_visuals.rs (new file)

/// Visual damage state for enemies
#[derive(Debug, Clone)]
pub struct EnemyVisualState {
    /// Base ASCII art frames (pristine)
    pub base_art: Vec<String>,
    /// Damage overlay patterns
    pub damage_overlays: DamageOverlays,
    /// Current animation frame
    pub current_frame: usize,
    /// Current posture
    pub posture: EnemyPosture,
    /// Active visual effects
    pub active_effects: Vec<VisualEffect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyPosture {
    Confident,      // Full HP, aggressive stance
    Wary,           // 75% HP, defensive
    Staggered,      // 50% HP, off-balance
    Wounded,        // 25% HP, limping/hunched
    Dying,          // <10% HP, barely standing
}

#[derive(Debug, Clone)]
pub struct DamageOverlays {
    /// Wound markers (╳, /, \, etc.)
    pub wounds: Vec<WoundMarker>,
    /// Posture modifications
    pub posture_shifts: Vec<PostureShift>,
    /// Blood/damage effects
    pub blood_splatter: Vec<(usize, usize, char)>,
}

#[derive(Debug, Clone)]
pub struct WoundMarker {
    pub position: (usize, usize),  // Row, col in ASCII art
    pub severity: WoundSeverity,
    pub char_override: char,
}

#[derive(Debug, Clone, Copy)]
pub enum WoundSeverity {
    Scratch,    // Minor visual change
    Cut,        // Visible line
    Gash,       // Major wound
    Critical,   // Devastating mark
}

impl EnemyVisualState {
    pub fn apply_damage(&mut self, damage_percent: f32, hit_location: HitLocation) {
        // Add wound marker at hit location
        let severity = match damage_percent {
            d if d > 0.25 => WoundSeverity::Critical,
            d if d > 0.15 => WoundSeverity::Gash,
            d if d > 0.08 => WoundSeverity::Cut,
            _ => WoundSeverity::Scratch,
        };
        
        let wound_char = match severity {
            WoundSeverity::Critical => '╳',
            WoundSeverity::Gash => '/',
            WoundSeverity::Cut => '─',
            WoundSeverity::Scratch => '·',
        };
        
        let pos = self.hit_location_to_position(hit_location);
        self.damage_overlays.wounds.push(WoundMarker {
            position: pos,
            severity,
            char_override: wound_char,
        });
        
        // Update posture based on cumulative damage
        self.update_posture();
        
        // Add blood effect
        if severity as u8 >= WoundSeverity::Cut as u8 {
            self.add_blood_splatter(pos);
        }
    }
    
    fn update_posture(&mut self) {
        let wound_severity: u8 = self.damage_overlays.wounds
            .iter()
            .map(|w| w.severity as u8)
            .sum();
        
        self.posture = match wound_severity {
            0..=2 => EnemyPosture::Confident,
            3..=5 => EnemyPosture::Wary,
            6..=8 => EnemyPosture::Staggered,
            9..=12 => EnemyPosture::Wounded,
            _ => EnemyPosture::Dying,
        };
    }
    
    /// Render the current visual state with damage overlays
    pub fn render(&self) -> Vec<String> {
        let mut art = self.base_art.clone();
        
        // Apply posture shift
        art = self.apply_posture_shift(art);
        
        // Apply wound markers
        for wound in &self.damage_overlays.wounds {
            if wound.position.0 < art.len() {
                let row = &mut art[wound.position.0];
                if wound.position.1 < row.len() {
                    let mut chars: Vec<char> = row.chars().collect();
                    chars[wound.position.1] = wound.char_override;
                    *row = chars.into_iter().collect();
                }
            }
        }
        
        // Apply blood splatter
        for (row, col, ch) in &self.damage_overlays.blood_splatter {
            if *row < art.len() {
                let line = &mut art[*row];
                if *col < line.len() {
                    let mut chars: Vec<char> = line.chars().collect();
                    if chars[*col] == ' ' {
                        chars[*col] = *ch;
                        *line = chars.into_iter().collect();
                    }
                }
            }
        }
        
        art
    }
    
    fn apply_posture_shift(&self, mut art: Vec<String>) -> Vec<String> {
        match self.posture {
            EnemyPosture::Confident => art, // No change
            EnemyPosture::Wary => {
                // Slight lean backward (shift right by 1)
                for line in &mut art {
                    *line = format!(" {}", line);
                }
                art
            }
            EnemyPosture::Staggered => {
                // Asymmetric shift (lean)
                for (i, line) in art.iter_mut().enumerate() {
                    if i % 2 == 0 {
                        *line = format!(" {}", line);
                    }
                }
                art
            }
            EnemyPosture::Wounded => {
                // Hunched (compress vertically by removing top line if possible)
                if art.len() > 3 {
                    art.insert(0, String::new()); // Add empty line at top
                }
                art
            }
            EnemyPosture::Dying => {
                // Dramatic lean + trembling effect (shown via animation)
                for (i, line) in art.iter_mut().enumerate() {
                    let offset = if i > art.len() / 2 { i - art.len() / 2 } else { 0 };
                    *line = format!("{}{}", " ".repeat(offset), line);
                }
                art
            }
        }
    }
    
    fn add_blood_splatter(&mut self, near: (usize, usize)) {
        let blood_chars = ['·', ':', '.', ',', '*'];
        let mut rng = rand::thread_rng();
        
        // Add 2-5 blood particles near the wound
        let count = rng.gen_range(2..=5);
        for _ in 0..count {
            let offset_row = rng.gen_range(-2i32..=2) as i32;
            let offset_col = rng.gen_range(-3i32..=3) as i32;
            let new_row = (near.0 as i32 + offset_row).max(0) as usize;
            let new_col = (near.1 as i32 + offset_col).max(0) as usize;
            let ch = *blood_chars.choose(&mut rng).unwrap();
            self.damage_overlays.blood_splatter.push((new_row, new_col, ch));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HitLocation {
    Head,
    Torso,
    LeftArm,
    RightArm,
    Legs,
    Center,
}
```

### Example: Progressive Goblin Damage

```
FRESH (100% HP):           BLOODIED (50% HP):        DYING (15% HP):
                           
    ,---.                      ,---.                       ,---.  
   / o o \                    / o─o \                      /·o─x \ 
   \  >  /                    \  >  /                       \ < /  
    |   |                      |╳  |                          |╳ |
   /|   |\                    /| : |\                       ·/|   |\·
  / |   | \                  / | : | \                     :/ |╳  |·\
 /__|   |__\                /__|   |__\                   /__|   |__\
                                   ·                          : ·
```

---

## Part 4: Typing-Combat Fusion

### Problem

Typing currently:
1. Inputs text
2. Checks correctness
3. Calculates damage separately
4. Shows number

The typing and the impact feel disconnected.

### Solution: Frame-by-Frame Typing Impact

```rust
// src/game/typing_impact.rs (new file)

use std::time::Instant;

/// Tracks typing and translates it to combat impact frame-by-frame
#[derive(Debug, Clone)]
pub struct TypingImpact {
    /// Current attack being typed
    pub current_attack: AttackSequence,
    /// Pending damage to apply (builds with each keystroke)
    pub pending_damage: f32,
    /// Damage multiplier from typing quality
    pub quality_multiplier: f32,
    /// Visual impact intensity (0.0 - 1.0)
    pub impact_intensity: f32,
    /// Attack type based on word completion
    pub attack_type: AttackType,
}

#[derive(Debug, Clone)]
pub struct AttackSequence {
    pub word: String,
    pub typed: String,
    pub started_at: Instant,
    pub keystrokes: Vec<Keystroke>,
}

#[derive(Debug, Clone)]
pub struct Keystroke {
    pub char: char,
    pub correct: bool,
    pub timestamp: Instant,
    /// Time since last keystroke
    pub interval_ms: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AttackType {
    /// Slow, methodical — single heavy strike
    Deliberate,
    /// Fast, flowing — rapid combo
    Flurry,
    /// Perfect accuracy — precision strike
    Precision,
    /// Messy but fast — wild swings
    Frantic,
    /// Mixed performance — normal attack
    Standard,
}

impl TypingImpact {
    pub fn new() -> Self {
        Self {
            current_attack: AttackSequence {
                word: String::new(),
                typed: String::new(),
                started_at: Instant::now(),
                keystrokes: Vec::new(),
            },
            pending_damage: 0.0,
            quality_multiplier: 1.0,
            impact_intensity: 0.0,
            attack_type: AttackType::Standard,
        }
    }
    
    /// Called on each keystroke during combat
    pub fn on_keystroke(&mut self, ch: char, correct: bool) -> KeystrokeResult {
        let now = Instant::now();
        let interval = self.current_attack.keystrokes.last()
            .map(|k| now.duration_since(k.timestamp).as_millis() as u32)
            .unwrap_or(0);
        
        self.current_attack.keystrokes.push(Keystroke {
            char: ch,
            correct,
            timestamp: now,
            interval_ms: interval,
        });
        
        self.current_attack.typed.push(ch);
        
        // Calculate per-keystroke impact
        let keystroke_impact = self.calculate_keystroke_impact(correct, interval);
        self.pending_damage += keystroke_impact.damage;
        self.impact_intensity = keystroke_impact.visual_intensity;
        
        KeystrokeResult {
            damage_this_stroke: keystroke_impact.damage,
            visual_intensity: keystroke_impact.visual_intensity,
            sound_pitch: keystroke_impact.sound_pitch,
            screen_shake: keystroke_impact.screen_shake,
            rhythm_bonus: keystroke_impact.rhythm_bonus,
        }
    }
    
    fn calculate_keystroke_impact(&self, correct: bool, interval_ms: u32) -> KeystrokeImpact {
        if !correct {
            return KeystrokeImpact {
                damage: 0.0,
                visual_intensity: 0.8,  // Error flash
                sound_pitch: 0.5,       // Low, discordant
                screen_shake: 0.1,
                rhythm_bonus: 0.0,
            };
        }
        
        // Base damage per correct keystroke
        let base = 1.0;
        
        // Speed bonus: faster = more damage (up to 2x at 50ms intervals)
        let speed_mult = if interval_ms > 0 {
            (200.0 / interval_ms as f32).min(2.0).max(0.5)
        } else {
            1.0
        };
        
        // Rhythm bonus: consistent intervals feel better and do more
        let rhythm_mult = self.calculate_rhythm_bonus(interval_ms);
        
        let damage = base * speed_mult * rhythm_mult;
        
        KeystrokeImpact {
            damage,
            visual_intensity: (speed_mult * 0.5).min(1.0),
            sound_pitch: 0.8 + (speed_mult * 0.2),  // Higher pitch for fast typing
            screen_shake: damage * 0.05,
            rhythm_bonus: rhythm_mult - 1.0,
        }
    }
    
    fn calculate_rhythm_bonus(&self, current_interval: u32) -> f32 {
        // Compare to average interval of last 3 keystrokes
        let recent: Vec<u32> = self.current_attack.keystrokes
            .iter()
            .rev()
            .take(3)
            .map(|k| k.interval_ms)
            .collect();
        
        if recent.len() < 2 {
            return 1.0;
        }
        
        let avg: u32 = recent.iter().sum::<u32>() / recent.len() as u32;
        let variance = (current_interval as i32 - avg as i32).abs() as u32;
        
        // Low variance (consistent rhythm) = up to 50% bonus
        if variance < 30 {
            1.5
        } else if variance < 60 {
            1.25
        } else if variance < 100 {
            1.1
        } else {
            1.0
        }
    }
    
    /// Called when word is completed
    pub fn complete_word(&mut self) -> WordCompletionResult {
        let elapsed = self.current_attack.started_at.elapsed();
        let char_count = self.current_attack.typed.len();
        let correct_count = self.current_attack.keystrokes.iter().filter(|k| k.correct).count();
        
        let accuracy = correct_count as f32 / char_count.max(1) as f32;
        let wpm = (char_count as f32 / 5.0) / (elapsed.as_secs_f32() / 60.0);
        
        // Determine attack type
        self.attack_type = self.determine_attack_type(wpm, accuracy);
        
        // Apply attack type multiplier
        let type_mult = match self.attack_type {
            AttackType::Precision => 1.5,   // Perfect accuracy = bonus
            AttackType::Flurry => 1.3,      // Fast = combo potential
            AttackType::Deliberate => 1.2,  // Slow but accurate = solid
            AttackType::Frantic => 0.9,     // Fast but sloppy = penalty
            AttackType::Standard => 1.0,
        };
        
        let final_damage = (self.pending_damage * type_mult).round() as i32;
        
        WordCompletionResult {
            damage: final_damage,
            attack_type: self.attack_type,
            wpm,
            accuracy,
            perfect: accuracy >= 0.99,
            message: self.generate_attack_message(final_damage),
        }
    }
    
    fn determine_attack_type(&self, wpm: f32, accuracy: f32) -> AttackType {
        match (wpm, accuracy) {
            (w, a) if a >= 0.99 && w >= 80.0 => AttackType::Precision,
            (w, _) if w >= 100.0 => AttackType::Flurry,
            (w, a) if w < 40.0 && a >= 0.95 => AttackType::Deliberate,
            (w, a) if w >= 70.0 && a < 0.85 => AttackType::Frantic,
            _ => AttackType::Standard,
        }
    }
    
    fn generate_attack_message(&self, damage: i32) -> String {
        match self.attack_type {
            AttackType::Precision => format!("⚔ PRECISION STRIKE! {} damage!", damage),
            AttackType::Flurry => format!("⚡ FLURRY! {} damage!", damage),
            AttackType::Deliberate => format!("🗡 Heavy blow. {} damage.", damage),
            AttackType::Frantic => format!("💥 Wild swing! {} damage.", damage),
            AttackType::Standard => format!("You deal {} damage.", damage),
        }
    }
    
    /// Reset for next word
    pub fn reset(&mut self, new_word: String) {
        self.current_attack = AttackSequence {
            word: new_word,
            typed: String::new(),
            started_at: Instant::now(),
            keystrokes: Vec::new(),
        };
        self.pending_damage = 0.0;
        self.impact_intensity = 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct KeystrokeImpact {
    pub damage: f32,
    pub visual_intensity: f32,
    pub sound_pitch: f32,
    pub screen_shake: f32,
    pub rhythm_bonus: f32,
}

#[derive(Debug, Clone)]
pub struct KeystrokeResult {
    pub damage_this_stroke: f32,
    pub visual_intensity: f32,
    pub sound_pitch: f32,
    pub screen_shake: f32,
    pub rhythm_bonus: f32,
}

#[derive(Debug, Clone)]
pub struct WordCompletionResult {
    pub damage: i32,
    pub attack_type: AttackType,
    pub wpm: f32,
    pub accuracy: f32,
    pub perfect: bool,
    pub message: String,
}
```

### Visual Feedback: Per-Keystroke Rendering

```rust
// In render.rs, modify combat rendering

fn render_typing_line(f: &mut Frame, area: Rect, combat: &CombatState, impact: &TypingImpact) {
    let word = &combat.current_word;
    let typed = &combat.typed_input;
    
    let mut spans = Vec::new();
    
    for (i, ch) in word.chars().enumerate() {
        let style = if i < typed.len() {
            let typed_ch = typed.chars().nth(i).unwrap();
            if typed_ch == ch {
                // Correct character - intensity based on typing speed
                let intensity = impact.impact_intensity;
                let green = (150.0 + intensity * 105.0) as u8;
                Style::default()
                    .fg(Color::Rgb(0, green, 0))
                    .add_modifier(if intensity > 0.7 { Modifier::BOLD } else { Modifier::empty() })
            } else {
                // Error - red with shake indication
                Style::default()
                    .fg(Color::Rgb(255, 50, 50))
                    .add_modifier(Modifier::UNDERLINED)
            }
        } else if i == typed.len() {
            // Current position - cursor highlight
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::SLOW_BLINK)
        } else {
            // Upcoming characters
            Style::default().fg(Color::DarkGray)
        };
        
        spans.push(Span::styled(ch.to_string(), style));
    }
    
    // Add damage preview
    if impact.pending_damage > 0.0 {
        spans.push(Span::styled(
            format!("  [{:.0}]", impact.pending_damage),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        ));
    }
    
    let line = Line::from(spans);
    let para = Paragraph::new(line)
        .alignment(Alignment::Center);
    
    f.render_widget(para, area);
}
```

---

## Part 5: Player Presence — A Body in the World

### Problem

The player is an abstraction — a name, some stats, a cursor. There's no sense of physical presence.

### Solution: Player Avatar with Animation States

```rust
// src/game/player_avatar.rs (new file)

use std::time::{Duration, Instant};

/// Player's visual representation
#[derive(Debug, Clone)]
pub struct PlayerAvatar {
    /// Current animation state
    pub state: AvatarState,
    /// Animation frame index
    pub frame: usize,
    /// Time of last frame change
    pub last_frame_change: Instant,
    /// Player's class (affects appearance)
    pub class: PlayerClass,
    /// Active visual effects on player
    pub effects: Vec<PlayerVisualEffect>,
    /// Facing direction
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarState {
    Idle,
    Typing,          // Fingers moving
    AttackWindup,    // Preparing to strike
    AttackRelease,   // Strike lands
    Hit,             // Taking damage
    Dodge,           // Evading
    Victory,         // Fist pump / celebration
    Death,           // Falling
    Casting,         // Spell casting pose
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerClass {
    Warrior,
    Mage,
    Rogue,
    Cleric,
    Ranger,
}

impl PlayerAvatar {
    pub fn new(class: PlayerClass) -> Self {
        Self {
            state: AvatarState::Idle,
            frame: 0,
            last_frame_change: Instant::now(),
            class,
            effects: Vec::new(),
            facing: Facing::Right,
        }
    }
    
    /// Get the current ASCII art frame
    pub fn render(&self) -> Vec<String> {
        let frames = self.get_animation_frames();
        let frame_idx = self.frame % frames.len();
        frames[frame_idx].clone()
    }
    
    /// Advance animation based on elapsed time
    pub fn tick(&mut self) {
        let frame_duration = self.get_frame_duration();
        if self.last_frame_change.elapsed() >= frame_duration {
            self.frame += 1;
            self.last_frame_change = Instant::now();
            
            // Some states auto-return to idle
            if self.state == AvatarState::AttackRelease && self.frame >= 3 {
                self.state = AvatarState::Idle;
                self.frame = 0;
            }
            if self.state == AvatarState::Hit && self.frame >= 2 {
                self.state = AvatarState::Idle;
                self.frame = 0;
            }
        }
    }
    
    /// Transition to a new state
    pub fn set_state(&mut self, new_state: AvatarState) {
        if self.state != new_state {
            self.state = new_state;
            self.frame = 0;
            self.last_frame_change = Instant::now();
        }
    }
    
    fn get_frame_duration(&self) -> Duration {
        match self.state {
            AvatarState::Idle => Duration::from_millis(500),
            AvatarState::Typing => Duration::from_millis(100),
            AvatarState::AttackWindup => Duration::from_millis(80),
            AvatarState::AttackRelease => Duration::from_millis(60),
            AvatarState::Hit => Duration::from_millis(100),
            AvatarState::Victory => Duration::from_millis(200),
            _ => Duration::from_millis(150),
        }
    }
    
    fn get_animation_frames(&self) -> Vec<Vec<String>> {
        match (self.class, self.state) {
            (PlayerClass::Warrior, AvatarState::Idle) => vec![
                vec![
                    "   O   ".into(),
                    "  /|\\  ".into(),
                    "  / \\  ".into(),
                ],
                vec![
                    "   O   ".into(),
                    "  /|\\  ".into(),
                    "  / \\  ".into(),
                ],
                vec![
                    "   o   ".into(),  // Slight head bob
                    "  /|\\  ".into(),
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Warrior, AvatarState::Typing) => vec![
                vec![
                    "   O   ".into(),
                    "  /|-, ".into(),  // Hands forward
                    "  / \\  ".into(),
                ],
                vec![
                    "   O   ".into(),
                    "  ,-|/ ".into(),  // Alternating
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Warrior, AvatarState::AttackWindup) => vec![
                vec![
                    "   O_  ".into(),
                    "  /|   ".into(),
                    "  / \\  ".into(),
                ],
                vec![
                    "   O\\  ".into(),
                    "  /|   ".into(),
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Warrior, AvatarState::AttackRelease) => vec![
                vec![
                    " __O   ".into(),
                    "   |\\==".into(),  // Sword thrust
                    "  / \\  ".into(),
                ],
                vec![
                    "   O   ".into(),
                    "  /|\\==".into(),
                    "  / \\  ".into(),
                ],
                vec![
                    "   O   ".into(),
                    "  /|\\  ".into(),
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Warrior, AvatarState::Hit) => vec![
                vec![
                    "   O   ".into(),
                    "   |\\  ".into(),
                    "  /|   ".into(),  // Stagger
                ],
                vec![
                    "  \\O/  ".into(),  // Recoil
                    "   |   ".into(),
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Warrior, AvatarState::Victory) => vec![
                vec![
                    "  \\O/  ".into(),
                    "   |   ".into(),
                    "  / \\  ".into(),
                ],
                vec![
                    "   O/  ".into(),
                    "  /|   ".into(),
                    "  / \\  ".into(),
                ],
            ],
            (PlayerClass::Mage, AvatarState::Idle) => vec![
                vec![
                    "   O   ".into(),
                    "  /|\\ ~".into(),  // Staff with magic
                    "  / \\| ".into(),
                ],
            ],
            (PlayerClass::Mage, AvatarState::Casting) => vec![
                vec![
                    "  \\O/  ".into(),
                    "   |  *".into(),
                    "  / \\ |".into(),
                ],
                vec![
                    "  \\O/ *".into(),
                    "   | * ".into(),
                    "  / \\ |".into(),
                ],
                vec![
                    "  \\O/**".into(),
                    "   |***".into(),
                    "  / \\ |".into(),
                ],
            ],
            // ... more class/state combinations
            _ => vec![vec![
                "   O   ".into(),
                "  /|\\  ".into(),
                "  / \\  ".into(),
            ]],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerVisualEffect {
    pub effect_type: EffectType,
    pub duration_remaining: Duration,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Shield,      // Glowing outline
    Poison,      // Green tint
    Burning,     // Orange particles
    Blessed,     // Yellow glow
    Haste,       // Motion blur
}
```

### Combat Layout with Player

```rust
// In render.rs

fn render_combat(f: &mut Frame, state: &GameState) {
    let area = f.area();
    
    // Split into: Player | Battle Info | Enemy
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Player avatar
            Constraint::Percentage(50),  // Combat info
            Constraint::Percentage(25),  // Enemy
        ])
        .split(area);
    
    // Render player avatar (left side)
    render_player_avatar(f, chunks[0], &state.player_avatar);
    
    // Render combat info (center)
    render_combat_info(f, chunks[1], state);
    
    // Render enemy with damage states (right side)
    render_enemy_visual(f, chunks[2], &state.combat.as_ref().unwrap().enemy_visual);
}

fn render_player_avatar(f: &mut Frame, area: Rect, avatar: &PlayerAvatar) {
    let art = avatar.render();
    
    // Center the avatar in the area
    let art_height = art.len() as u16;
    let y_offset = (area.height.saturating_sub(art_height)) / 2;
    
    let mut lines: Vec<Line> = Vec::new();
    
    // Add empty lines for centering
    for _ in 0..y_offset {
        lines.push(Line::from(""));
    }
    
    // Add avatar art with coloring based on state
    let color = match avatar.state {
        AvatarState::Idle => Color::White,
        AvatarState::Typing => Color::Cyan,
        AvatarState::AttackWindup => Color::Yellow,
        AvatarState::AttackRelease => Color::Green,
        AvatarState::Hit => Color::Red,
        AvatarState::Victory => Color::Magenta,
        _ => Color::White,
    };
    
    for line in art {
        lines.push(Line::styled(line, Style::default().fg(color)));
    }
    
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" YOU "))
        .alignment(Alignment::Center);
    
    f.render_widget(para, area);
}
```

---

## Part 6: Architecture Integration

### How Systems Connect

```
┌─────────────────────────────────────────────────────────────────────┐
│                            EventBus                                  │
│   (Central nervous system - already wired in v0.5.0)                │
└───────┬─────────────────────────────────────────────────────────────┘
        │
        │ Events flow through
        ▼
┌───────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  TypingImpact     │────▶│  DialogueEngine  │────▶│  VoiceSystem    │
│  (NEW)            │     │  (NEW)           │     │  (existing)     │
│                   │     │                  │     │                 │
│ • Per-keystroke   │     │ • Context-aware  │     │ • Faction voice │
│ • Rhythm bonus    │     │ • Enemy state    │     │ • Vocabulary    │
│ • Attack types    │     │ • Momentum       │     │ • Idioms        │
└───────────────────┘     └──────────────────┘     └─────────────────┘
        │                         │
        ▼                         ▼
┌───────────────────┐     ┌──────────────────┐
│  EnemyVisualState │     │  PacingController │
│  (NEW)            │     │  (NEW)            │
│                   │     │                   │
│ • Damage overlays │     │ • Tension levels  │
│ • Posture shifts  │     │ • Breather beats  │
│ • Blood effects   │     │ • Atmosphere      │
└───────────────────┘     └───────────────────┘
        │
        ▼
┌───────────────────┐
│  PlayerAvatar     │
│  (NEW)            │
│                   │
│ • Animation state │
│ • Class visuals   │
│ • Hit reactions   │
└───────────────────┘
```

### Wiring Checklist

```rust
// In GameState, add new fields:
pub struct GameState {
    // ... existing fields ...
    
    /// Typing-to-damage impact tracker
    pub typing_impact: TypingImpact,
    /// Contextual dialogue engine
    pub dialogue_engine: DialogueEngine,
    /// Enemy dialogue banks
    pub enemy_dialogue_banks: EnemyDialogueBanks,
    /// Pacing controller
    pub pacing: PacingController,
    /// Player avatar for visual presence
    pub player_avatar: PlayerAvatar,
}

// In CombatState, add:
pub struct CombatState {
    // ... existing fields ...
    
    /// Enemy's visual damage state
    pub enemy_visual: EnemyVisualState,
}
```

### Event Flow Example

When player types a character:

```rust
// In main.rs or input handling

fn on_key_press(&mut self, ch: char) {
    if let Some(combat) = &mut self.game.combat {
        // 1. Record keystroke in typing impact system
        let impact_result = self.game.typing_impact.on_keystroke(ch, correct);
        
        // 2. Update player avatar animation
        self.game.player_avatar.set_state(AvatarState::Typing);
        
        // 3. Apply per-keystroke visual feedback
        if impact_result.screen_shake > 0.1 {
            self.game.typing_feel.screen_shake = impact_result.screen_shake;
        }
        
        // 4. Check if word complete
        if combat.typed_input == combat.current_word {
            let result = self.game.typing_impact.complete_word();
            
            // 5. Trigger attack animation
            self.game.player_avatar.set_state(AvatarState::AttackWindup);
            
            // 6. Apply damage to enemy visual state
            let damage_pct = result.damage as f32 / combat.enemy.max_hp as f32;
            combat.enemy_visual.apply_damage(damage_pct, HitLocation::Center);
            
            // 7. Generate contextual hit dialogue
            let dialogue = self.game.dialogue_engine.generate_hit_dialogue(
                &combat.enemy,
                result.damage,
                result.attack_type,
            );
            combat.battle_log.push(dialogue);
            
            // 8. Emit event for other systems
            self.game.event_bus.emit(GameEvent::DamageDealt {
                target: combat.enemy.name.clone(),
                amount: result.damage,
                attack_type: format!("{:?}", result.attack_type),
            });
        }
    }
}
```

---

## Part 7: Implementation Priority

### Phase 1: Core Feel (1-2 days)
1. ✅ Create `typing_impact.rs` — immediate tactile feedback
2. ✅ Modify combat to use per-keystroke damage buildup
3. ✅ Add attack types based on typing quality

### Phase 2: Enemy Life (1-2 days)
1. Create `enemy_visuals.rs` — damage states
2. Create `enemy_dialogue.rs` — dialogue banks for top 10 enemies
3. Wire visual degradation to damage events

### Phase 3: Player Presence (1 day)
1. Create `player_avatar.rs` — ASCII animations
2. Modify combat render to show player
3. Add attack/hit/idle animations

### Phase 4: Dialogue & Pacing (2-3 days)
1. Create `dialogue_engine.rs` — contextual generation
2. Create `pacing.rs` — breather beats
3. Create dialogue banks for all enemies
4. Add atmospheric moments between combats

### Phase 5: Polish (1-2 days)
1. Tune damage numbers
2. Add more animation frames
3. Write more enemy dialogue
4. Playtest pacing adjustments

---

## Appendix: Writing Tone Reference

From your `writing_guidelines.rs`, key principles for new dialogue:

### DO:
- **Show don't tell**: "The knight's blade arm trembles" not "The knight is tired"
- **Economy of language**: 12 words max for combat text
- **Layered meaning**: Hint at deeper truth
- **Character voice**: Each enemy type has personality

### DON'T:
- Use "very", "really", "epic", "awesome"
- State emotions directly ("The goblin is angry")
- Write generic video game dialogue
- Break the world's tone for mechanics

### Example Transformations:

**Before (generic)**:
```
"You deal 15 damage!"
"The enemy attacks!"
"Victory!"
```

**After (immersive)**:
```
"Your blade finds the gap between plates. The knight buckles. (15)"
"The knight's oath demands blood. Steel sings toward your throat."
"The armor falls empty. Duty, at last, released."
```

---

## Summary

Your game has the bones. This plan adds:
- **Muscle**: Typing directly drives impact
- **Nerves**: Systems react to each other
- **Skin**: Visual damage and player presence
- **Voice**: Enemies that speak their nature
- **Breath**: Pacing that builds and releases

Total estimated effort: **7-10 days** of focused work.

The result: A game where every keystroke *matters*.
