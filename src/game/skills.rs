//! Deep Skill Tree System - Meaningful progression with real choices

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub trees: HashMap<SkillTreeType, Vec<Skill>>,
    pub skill_points: u32,
    pub unlocked_skills: Vec<String>,
}

impl Default for SkillTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillTree {
    pub fn new() -> Self {
        let mut trees = HashMap::new();
        trees.insert(SkillTreeType::Precision, Self::precision_tree());
        trees.insert(SkillTreeType::Speed, Self::speed_tree());
        trees.insert(SkillTreeType::Endurance, Self::endurance_tree());
        trees.insert(SkillTreeType::Wisdom, Self::wisdom_tree());
        trees.insert(SkillTreeType::Shadow, Self::shadow_tree());
        
        Self {
            trees,
            skill_points: 0,
            unlocked_skills: Vec::new(),
        }
    }
    
    pub fn has_skill(&self, skill_id: &str) -> bool {
        self.unlocked_skills.contains(&skill_id.to_string())
    }
    
    pub fn can_unlock(&self, skill: &Skill) -> bool {
        if self.skill_points < skill.cost {
            return false;
        }
        
        // Check prerequisites
        for prereq in &skill.prerequisites {
            if !self.has_skill(prereq) {
                return false;
            }
        }
        
        true
    }
    
    pub fn unlock_skill(&mut self, skill_id: &str) -> bool {
        // Find the skill
        for skills in self.trees.values() {
            if let Some(skill) = skills.iter().find(|s| s.id == skill_id) {
                if self.can_unlock(skill) && !self.has_skill(skill_id) {
                    self.skill_points -= skill.cost;
                    self.unlocked_skills.push(skill_id.to_string());
                    return true;
                }
            }
        }
        false
    }
    
    fn precision_tree() -> Vec<Skill> {
        vec![
            Skill {
                id: "precision_1".to_string(),
                name: "Careful Keystroke".to_string(),
                description: "Your deliberate approach reduces errors. +5% accuracy bonus.".to_string(),
                tree: SkillTreeType::Precision,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::AccuracyBonus(0.05)],
                lore: "The Scribes teach that every keystroke is a commitment. \
                       To type carelessly is to think carelessly.".to_string(),
            },
            Skill {
                id: "precision_2".to_string(),
                name: "Proofreader's Eye".to_string(),
                description: "You spot errors before they happen. Backspace is 50% faster.".to_string(),
                tree: SkillTreeType::Precision,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::BackspaceSpeed(1.5)],
                lore: "In the old publishing houses, proofreaders were revered as the last \
                       line of defense against chaos.".to_string(),
            },
            Skill {
                id: "precision_3".to_string(),
                name: "Word Weaver".to_string(),
                description: "Completing words without errors grants +10% damage bonus.".to_string(),
                tree: SkillTreeType::Precision,
                tier: 2,
                cost: 2,
                prerequisites: vec!["precision_1".to_string()],
                effects: vec![SkillEffect::PerfectWordDamageBonus(0.10)],
                lore: "The ancient Wordsmiths could weave sentences that moved hearts and \
                       toppled kingdoms. You walk their path now.".to_string(),
            },
            Skill {
                id: "precision_4".to_string(),
                name: "Sentence Mastery".to_string(),
                description: "Perfect sentences deal double damage and restore 5 MP.".to_string(),
                tree: SkillTreeType::Precision,
                tier: 3,
                cost: 3,
                prerequisites: vec!["precision_3".to_string()],
                effects: vec![
                    SkillEffect::PerfectSentenceBonus(2.0),
                    SkillEffect::PerfectSentenceMPRestore(5),
                ],
                lore: "A perfect sentence is a blade forged from pure intention. \
                       It cuts through confusion and illuminates truth.".to_string(),
            },
            Skill {
                id: "precision_5".to_string(),
                name: "The Eternal Word".to_string(),
                description: "Your first word each combat cannot miss and deals triple damage.".to_string(),
                tree: SkillTreeType::Precision,
                tier: 4,
                cost: 4,
                prerequisites: vec!["precision_4".to_string()],
                effects: vec![SkillEffect::FirstWordPerfect, SkillEffect::FirstWordTripleDamage],
                lore: "In the beginning was the Word, and the Word was with Power, \
                       and the Word was Power. So say the Scribes.".to_string(),
            },
        ]
    }
    
    fn speed_tree() -> Vec<Skill> {
        vec![
            Skill {
                id: "speed_1".to_string(),
                name: "Quick Fingers".to_string(),
                description: "Base typing speed contributes +10% more to damage.".to_string(),
                tree: SkillTreeType::Speed,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::WPMDamageMultiplier(0.10)],
                lore: "The Mechanists believe that speed is the purest expression of \
                       mastery. Hesitation is failure.".to_string(),
            },
            Skill {
                id: "speed_2".to_string(),
                name: "Burst Mode".to_string(),
                description: "Typing above 60 WPM grants +15% damage for 3 seconds.".to_string(),
                tree: SkillTreeType::Speed,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::BurstModeThreshold(60.0, 0.15, 3.0)],
                lore: "There is a state beyond conscious thought where the fingers move \
                       faster than the mind can follow.".to_string(),
            },
            Skill {
                id: "speed_3".to_string(),
                name: "Overclock".to_string(),
                description: "Time limit increased by 2 seconds. +20% damage when under 2 seconds remain.".to_string(),
                tree: SkillTreeType::Speed,
                tier: 2,
                cost: 2,
                prerequisites: vec!["speed_1".to_string()],
                effects: vec![
                    SkillEffect::TimeLimitBonus(2.0),
                    SkillEffect::PressureDamageBonus(2.0, 0.20),
                ],
                lore: "The Mechanist mantra: 'Under pressure, carbon becomes diamond. \
                       Under deadline, typists become legends.'".to_string(),
            },
            Skill {
                id: "speed_4".to_string(),
                name: "Velocity State".to_string(),
                description: "Each consecutive word above 80 WPM adds stacking +5% damage.".to_string(),
                tree: SkillTreeType::Speed,
                tier: 3,
                cost: 3,
                prerequisites: vec!["speed_3".to_string()],
                effects: vec![SkillEffect::VelocityStacking(80.0, 0.05)],
                lore: "The fastest typists report a sensation of time slowing down. \
                       Their fingers exist in a different temporal frame.".to_string(),
            },
            Skill {
                id: "speed_5".to_string(),
                name: "Mechanical Transcendence".to_string(),
                description: "At 100+ WPM, you enter Transcendence: all damage doubled, immunity to timing out.".to_string(),
                tree: SkillTreeType::Speed,
                tier: 4,
                cost: 4,
                prerequisites: vec!["speed_4".to_string()],
                effects: vec![SkillEffect::Transcendence(100.0)],
                lore: "The ultimate Mechanist achievement. At this speed, you are no longer \
                       typing—you are becoming one with the machine itself.".to_string(),
            },
        ]
    }
    
    fn endurance_tree() -> Vec<Skill> {
        vec![
            Skill {
                id: "endurance_1".to_string(),
                name: "Marathon Typer".to_string(),
                description: "+10% max HP. Long battles favor the patient.".to_string(),
                tree: SkillTreeType::Endurance,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::MaxHPBonus(0.10)],
                lore: "The Naturalists say: 'The river does not tire. The wind does not rest. \
                       Be as they are.'".to_string(),
            },
            Skill {
                id: "endurance_2".to_string(),
                name: "Second Wind".to_string(),
                description: "Once per combat, dropping below 25% HP restores 30% HP.".to_string(),
                tree: SkillTreeType::Endurance,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::SecondWind(0.25, 0.30)],
                lore: "Near death, the body releases reserves it hoards jealously. \
                       Learn to access these reserves consciously.".to_string(),
            },
            Skill {
                id: "endurance_3".to_string(),
                name: "Flow State".to_string(),
                description: "Maintaining 90%+ accuracy for 30 seconds regenerates HP slowly.".to_string(),
                tree: SkillTreeType::Endurance,
                tier: 2,
                cost: 2,
                prerequisites: vec!["endurance_1".to_string()],
                effects: vec![SkillEffect::FlowStateRegen(0.90, 30.0, 1)],
                lore: "When movement becomes effortless, when thought becomes action, \
                       you enter the Flow. The Flow sustains.".to_string(),
            },
            Skill {
                id: "endurance_4".to_string(),
                name: "Unshakeable Focus".to_string(),
                description: "Damage taken reduced by 20%. Typing through pain builds character.".to_string(),
                tree: SkillTreeType::Endurance,
                tier: 3,
                cost: 3,
                prerequisites: vec!["endurance_3".to_string()],
                effects: vec![SkillEffect::DamageReduction(0.20)],
                lore: "The master typist's fingers continue even as the world burns. \
                       Focus is armor; determination is a shield.".to_string(),
            },
            Skill {
                id: "endurance_5".to_string(),
                name: "One With The Keyboard".to_string(),
                description: "HP regenerates 1 per perfect word. You and the keyboard are one.".to_string(),
                tree: SkillTreeType::Endurance,
                tier: 4,
                cost: 4,
                prerequisites: vec!["endurance_4".to_string()],
                effects: vec![SkillEffect::PerfectWordHeal(1)],
                lore: "The Naturalist masters speak of becoming the keyboard, the keys becoming \
                       fingers, the fingers becoming thought, thought becoming action, \
                       action becoming word, word becoming world.".to_string(),
            },
        ]
    }
    
    fn wisdom_tree() -> Vec<Skill> {
        vec![
            Skill {
                id: "wisdom_1".to_string(),
                name: "Scholar's Memory".to_string(),
                description: "+10% XP from all sources. Knowledge compounds.".to_string(),
                tree: SkillTreeType::Wisdom,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::XPBonus(0.10)],
                lore: "The Archivists record everything, forget nothing. Their memories \
                       are vaults of accumulated wisdom.".to_string(),
            },
            Skill {
                id: "wisdom_2".to_string(),
                name: "Pattern Recognition".to_string(),
                description: "Common words are highlighted, making them easier to type quickly.".to_string(),
                tree: SkillTreeType::Wisdom,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::CommonWordHighlight],
                lore: "Experienced readers don't read letters—they read patterns. \
                       Train your mind to see the shapes of words.".to_string(),
            },
            Skill {
                id: "wisdom_3".to_string(),
                name: "Linguistic Insight".to_string(),
                description: "Learn enemy weaknesses after 3 turns. Type their weakness words for +50% damage.".to_string(),
                tree: SkillTreeType::Wisdom,
                tier: 2,
                cost: 2,
                prerequisites: vec!["wisdom_1".to_string()],
                effects: vec![SkillEffect::EnemyWeaknessReveal(3)],
                lore: "Every creature of corruption has a weakness, a word they fear, \
                       a phrase that unravels their being.".to_string(),
            },
            Skill {
                id: "wisdom_4".to_string(),
                name: "Polyglot".to_string(),
                description: "Unlock bonus challenges in other languages for massive XP rewards.".to_string(),
                tree: SkillTreeType::Wisdom,
                tier: 3,
                cost: 3,
                prerequisites: vec!["wisdom_3".to_string()],
                effects: vec![SkillEffect::UnlockLanguages],
                lore: "The greatest typists can shift between languages as easily as \
                       others shift between keys. Each language opens new doors.".to_string(),
            },
            Skill {
                id: "wisdom_5".to_string(),
                name: "Logos".to_string(),
                description: "Understanding the Word grants power over it. All text damage +25%, comprehension of ancient texts.".to_string(),
                tree: SkillTreeType::Wisdom,
                tier: 4,
                cost: 4,
                prerequisites: vec!["wisdom_4".to_string()],
                effects: vec![SkillEffect::LogosPower(0.25)],
                lore: "In the beginning was Logos—Word, Reason, meaning itself. \
                       To understand Logos is to understand the foundation of reality.".to_string(),
            },
        ]
    }
    
    fn shadow_tree() -> Vec<Skill> {
        vec![
            Skill {
                id: "shadow_1".to_string(),
                name: "Subtle Keys".to_string(),
                description: "Enemies are less accurate against you. -10% enemy damage.".to_string(),
                tree: SkillTreeType::Shadow,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::EnemyDamageReduction(0.10)],
                lore: "The Shadow Writers move unseen, type unheard. Their fingers \
                       leave no trace on the keys.".to_string(),
            },
            Skill {
                id: "shadow_2".to_string(),
                name: "Misdirection".to_string(),
                description: "10% chance for enemy attacks to miss entirely.".to_string(),
                tree: SkillTreeType::Shadow,
                tier: 1,
                cost: 1,
                prerequisites: vec![],
                effects: vec![SkillEffect::EvasionChance(0.10)],
                lore: "The best defense is not being where the attack lands. \
                       Type in shadows, exist between keystrokes.".to_string(),
            },
            Skill {
                id: "shadow_3".to_string(),
                name: "Exploit Weakness".to_string(),
                description: "Critical hit chance +15%. Criticals deal double damage.".to_string(),
                tree: SkillTreeType::Shadow,
                tier: 2,
                cost: 2,
                prerequisites: vec!["shadow_1".to_string()],
                effects: vec![SkillEffect::CritChance(0.15), SkillEffect::CritDamage(2.0)],
                lore: "Every enemy has a gap in their defenses, a moment of vulnerability. \
                       The Shadow Writers are trained to find and exploit these moments.".to_string(),
            },
            Skill {
                id: "shadow_4".to_string(),
                name: "Ghost Protocol".to_string(),
                description: "First attack each combat is guaranteed critical and cannot be countered.".to_string(),
                tree: SkillTreeType::Shadow,
                tier: 3,
                cost: 3,
                prerequisites: vec!["shadow_3".to_string()],
                effects: vec![SkillEffect::GhostStrike],
                lore: "Strike first, strike true, vanish. The enemy should never know \
                       what hit them.".to_string(),
            },
            Skill {
                id: "shadow_5".to_string(),
                name: "Erasure".to_string(),
                description: "Once per combat, completely negate an enemy attack by typing a counter-phrase perfectly.".to_string(),
                tree: SkillTreeType::Shadow,
                tier: 4,
                cost: 4,
                prerequisites: vec!["shadow_4".to_string()],
                effects: vec![SkillEffect::Erasure],
                lore: "The ultimate Shadow Writer technique. They do not block attacks—\
                       they erase them from existence, as if they never were typed.".to_string(),
            },
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillTreeType {
    /// Focus on accuracy and meaningful typing
    Precision,
    /// Focus on raw typing speed
    Speed,
    /// Focus on survival and sustainability
    Endurance,
    /// Focus on knowledge and understanding
    Wisdom,
    /// Focus on critical hits and evasion
    Shadow,
}

impl SkillTreeType {
    pub fn name(&self) -> &'static str {
        match self {
            SkillTreeType::Precision => "Precision",
            SkillTreeType::Speed => "Velocity",
            SkillTreeType::Endurance => "Endurance",
            SkillTreeType::Wisdom => "Wisdom",
            SkillTreeType::Shadow => "Shadow",
        }
    }
    
    pub fn philosophy(&self) -> &'static str {
        match self {
            SkillTreeType::Precision => "Every keystroke matters. Quality over quantity.",
            SkillTreeType::Speed => "Time waits for no one. Type faster than thought.",
            SkillTreeType::Endurance => "The marathon, not the sprint. Outlast all opposition.",
            SkillTreeType::Wisdom => "Knowledge is power. Understanding is victory.",
            SkillTreeType::Shadow => "Strike unseen. Let them fear what they cannot predict.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tree: SkillTreeType,
    pub tier: u32,
    pub cost: u32,
    pub prerequisites: Vec<String>,
    pub effects: Vec<SkillEffect>,
    pub lore: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    // Precision tree
    AccuracyBonus(f32),
    BackspaceSpeed(f32),
    PerfectWordDamageBonus(f32),
    PerfectSentenceBonus(f32),
    PerfectSentenceMPRestore(i32),
    FirstWordPerfect,
    FirstWordTripleDamage,
    
    // Speed tree
    WPMDamageMultiplier(f32),
    BurstModeThreshold(f32, f32, f32), // wpm threshold, damage bonus, duration
    TimeLimitBonus(f32),
    PressureDamageBonus(f32, f32), // time remaining threshold, bonus
    VelocityStacking(f32, f32), // wpm threshold, stacking bonus per word
    Transcendence(f32), // wpm threshold for transcendence mode
    
    // Endurance tree
    MaxHPBonus(f32),
    SecondWind(f32, f32), // hp threshold, restore amount
    FlowStateRegen(f32, f32, i32), // accuracy threshold, duration, hp per tick
    DamageReduction(f32),
    PerfectWordHeal(i32),
    
    // Wisdom tree
    XPBonus(f32),
    CommonWordHighlight,
    EnemyWeaknessReveal(i32), // turns until reveal
    UnlockLanguages,
    LogosPower(f32),
    
    // Shadow tree
    EnemyDamageReduction(f32),
    EvasionChance(f32),
    CritChance(f32),
    CritDamage(f32),
    GhostStrike,
    Erasure,
}
