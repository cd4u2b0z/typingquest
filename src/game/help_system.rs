//! Help System - Context-aware in-game guidance
//!
//! Provides non-intrusive help overlays accessible at any time via ? or H.
//! Features:
//! - Context-sensitive tips based on current scene
//! - Full keybinding reference
//! - Current objectives and hints
//! - Tutorial reminders
//! - Tabbed navigation

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use super::state::Scene;

// ============================================================================
// HELP CONTEXT
// ============================================================================

/// What context the player is currently in (determines which tips to show)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HelpContext {
    Title,
    ClassSelect,
    Combat,
    Exploration,
    Shop,
    Rest,
    Event,
    Inventory,
    Stats,
    GameOver,
    Victory,
    Dialogue,
    Tutorial,
}

impl From<Scene> for HelpContext {
    fn from(scene: Scene) -> Self {
        match scene {
            Scene::Title => HelpContext::Title,
            Scene::ClassSelect => HelpContext::ClassSelect,
            Scene::Dungeon => HelpContext::Exploration,
            Scene::Combat => HelpContext::Combat,
            Scene::Shop => HelpContext::Shop,
            Scene::Rest => HelpContext::Rest,
            Scene::Event => HelpContext::Event,
            Scene::Inventory => HelpContext::Inventory,
            Scene::Stats => HelpContext::Stats,
            Scene::GameOver => HelpContext::GameOver,
            Scene::Victory => HelpContext::Victory,
            Scene::Tutorial => HelpContext::Tutorial,
            Scene::Lore => HelpContext::Event, // Lore is similar to events
            Scene::Milestone => HelpContext::Event, // Milestones are similar to events
            Scene::Upgrades => HelpContext::Shop, // Upgrades is like a shop
            Scene::BattleSummary => HelpContext::GameOver,
        }
    }
}

// ============================================================================
// HELP TABS
// ============================================================================

/// Available tabs in the help overlay
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HelpTab {
    #[default]
    Contextual,   // Context-sensitive tips
    Keybindings,  // Full key reference
    Objectives,   // Current goals
    Mechanics,    // Game systems explained
}

impl HelpTab {
    pub fn all() -> &'static [HelpTab] {
        &[
            HelpTab::Contextual,
            HelpTab::Keybindings,
            HelpTab::Objectives,
            HelpTab::Mechanics,
        ]
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            HelpTab::Contextual => "󰋖 Context",
            HelpTab::Keybindings => "󰌌 Keys",
            HelpTab::Objectives => "󰓥 Goals",
            HelpTab::Mechanics => "󰏗 Systems",
        }
    }
    
    pub fn index(&self) -> usize {
        match self {
            HelpTab::Contextual => 0,
            HelpTab::Keybindings => 1,
            HelpTab::Objectives => 2,
            HelpTab::Mechanics => 3,
        }
    }
    
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => HelpTab::Contextual,
            1 => HelpTab::Keybindings,
            2 => HelpTab::Objectives,
            3 => HelpTab::Mechanics,
            _ => HelpTab::Contextual,
        }
    }
}

// ============================================================================
// HELP TIP
// ============================================================================

/// Priority level for help tips
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TipPriority {
    Essential,  // Must know to play
    Important,  // Significantly helps
    Advanced,   // For optimization
    Secret,     // Hidden mechanics
}

/// A single help tip with icon and text
#[derive(Debug, Clone)]
pub struct HelpTip {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub priority: TipPriority,
}

impl HelpTip {
    pub const fn new(
        icon: &'static str,
        title: &'static str,
        description: &'static str,
        priority: TipPriority,
    ) -> Self {
        Self {
            icon,
            title,
            description,
            priority,
        }
    }
}

// ============================================================================
// KEYBINDING
// ============================================================================

/// A keybinding entry for the help screen
#[derive(Debug, Clone)]
pub struct Keybinding {
    pub key: &'static str,
    pub action: &'static str,
    pub context: Option<HelpContext>,  // None = global
}

impl Keybinding {
    pub const fn new(key: &'static str, action: &'static str) -> Self {
        Self {
            key,
            action,
            context: None,
        }
    }
    
    pub const fn with_context(
        key: &'static str,
        action: &'static str,
        context: HelpContext,
    ) -> Self {
        Self {
            key,
            action,
            context: Some(context),
        }
    }
}

// ============================================================================
// HELP SYSTEM
// ============================================================================

/// The main help system that manages overlay state and content
#[derive(Debug, Clone)]
pub struct HelpSystem {
    /// Whether the help overlay is currently visible
    pub visible: bool,
    
    /// Currently active tab
    pub active_tab: HelpTab,
    
    /// Current game context (auto-detected from scene)
    pub context: HelpContext,
    
    /// Scroll offset for long content
    pub scroll_offset: usize,
    
    /// Tips the player has seen (for progressive disclosure)
    pub tips_seen: HashSet<String>,
    
    /// Whether this is the player's first time seeing help
    pub first_time: bool,
}

impl Default for HelpSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpSystem {
    pub fn new() -> Self {
        Self {
            visible: false,
            active_tab: HelpTab::Contextual,
            context: HelpContext::Title,
            scroll_offset: 0,
            tips_seen: HashSet::new(),
            first_time: true,
        }
    }
    
    /// Toggle help overlay visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.scroll_offset = 0;
            self.first_time = false;
        }
    }
    
    /// Show help overlay
    pub fn show(&mut self) {
        self.visible = true;
        self.scroll_offset = 0;
        self.first_time = false;
    }
    
    /// Hide help overlay
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// Update context based on current scene
    pub fn update_context(&mut self, scene: Scene) {
        self.context = HelpContext::from(scene);
    }
    
    /// Move to next tab
    pub fn next_tab(&mut self) {
        let current = self.active_tab.index();
        let next = (current + 1) % HelpTab::all().len();
        self.active_tab = HelpTab::from_index(next);
        self.scroll_offset = 0;
    }
    
    /// Move to previous tab
    pub fn prev_tab(&mut self) {
        let current = self.active_tab.index();
        let prev = if current == 0 {
            HelpTab::all().len() - 1
        } else {
            current - 1
        };
        self.active_tab = HelpTab::from_index(prev);
        self.scroll_offset = 0;
    }
    
    /// Select tab by number (1-4)
    pub fn select_tab(&mut self, num: usize) {
        if num > 0 && num <= HelpTab::all().len() {
            self.active_tab = HelpTab::from_index(num - 1);
            self.scroll_offset = 0;
        }
    }
    
    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }
    
    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
    
    /// Mark a tip as seen
    pub fn mark_seen(&mut self, tip_id: &str) {
        self.tips_seen.insert(tip_id.to_string());
    }
    
    /// Check if a tip has been seen
    pub fn has_seen(&self, tip_id: &str) -> bool {
        self.tips_seen.contains(tip_id)
    }
    
    /// Get contextual tips for the current context
    pub fn get_contextual_tips(&self) -> Vec<HelpTip> {
        match self.context {
            HelpContext::Title => vec![
                HelpTip::new("󰒔", "Navigate", "Use j/k or ↑/↓ to move selection", TipPriority::Essential),
                HelpTip::new("󰌑", "Confirm", "Press Enter to select", TipPriority::Essential),
                HelpTip::new("󰗼", "Quit", "Press q to exit the game", TipPriority::Important),
            ],
            
            HelpContext::ClassSelect => vec![
                HelpTip::new("󰆥", "Choose Wisely", "Each class has unique abilities and playstyles", TipPriority::Essential),
                HelpTip::new("󰓥", "Wordsmith", "Balanced fighter with +10% damage", TipPriority::Important),
                HelpTip::new("󰏫", "Scribe", "Double XP, starts with Analyze", TipPriority::Important),
                HelpTip::new("󰄀", "Spellweaver", "Magic focus with +20% spell damage", TipPriority::Important),
                HelpTip::new("󰣥", "Barbarian", "High HP with +30% crit chance", TipPriority::Important),
                HelpTip::new("󰏇", "Trickster", "Combo master with +50% combo bonus", TipPriority::Important),
            ],
            
            HelpContext::Combat => vec![
                HelpTip::new("󰌌", "Type to Attack", "Type the displayed word to deal damage", TipPriority::Essential),
                HelpTip::new("󰁮", "Correct Mistakes", "Use Backspace to fix typos", TipPriority::Essential),
                HelpTip::new("󰈸", "Combos", "Chain words without mistakes for bonus damage", TipPriority::Important),
                HelpTip::new("󰄀", "Perfect Words", "No backspaces = 1.5x damage multiplier", TipPriority::Important),
                HelpTip::new("󰔚", "Speed Bonus", "Type faster for extra damage", TipPriority::Advanced),
                HelpTip::new("󰒔", "Flow State", "Consistent rhythm increases critical chance", TipPriority::Advanced),
                HelpTip::new("󰈆", "Flee", "Press Esc to attempt escape", TipPriority::Important),
            ],
            
            HelpContext::Exploration => vec![
                HelpTip::new("󰊗", "Explore", "Press e or Enter to enter the next room", TipPriority::Essential),
                HelpTip::new("󰆧", "Inventory", "Press i to view and use items", TipPriority::Important),
                HelpTip::new("󰄪", "Stats", "Press s to view your statistics", TipPriority::Important),
                HelpTip::new("󰓥", "Combat Rooms", "Fight enemies to progress", TipPriority::Important),
                HelpTip::new("󰚌", "Elite Rooms", "Harder enemies with better rewards", TipPriority::Important),
                HelpTip::new("󰒲", "Rest Sites", "Heal and recover between battles", TipPriority::Important),
            ],
            
            HelpContext::Shop => vec![
                HelpTip::new("󰒍", "Browse", "Use j/k to navigate items", TipPriority::Essential),
                HelpTip::new("󰆧", "Purchase", "Press Enter to buy selected item", TipPriority::Essential),
                HelpTip::new("󰈆", "Leave", "Press Esc to exit the shop", TipPriority::Important),
                HelpTip::new("󰒖", "Faction Prices", "Merchant reputation affects costs", TipPriority::Advanced),
            ],
            
            HelpContext::Rest => vec![
                HelpTip::new("󰒲", "Rest", "Choose an action to recover", TipPriority::Essential),
                HelpTip::new("󰣏", "Heal", "Restore HP by resting", TipPriority::Important),
                HelpTip::new("󰋖", "Train", "Improve skills through practice", TipPriority::Advanced),
                HelpTip::new("󰈆", "Continue", "Press Esc to leave", TipPriority::Important),
            ],
            
            HelpContext::Event => vec![
                HelpTip::new("󰋗", "Choose", "Read carefully and select an option", TipPriority::Essential),
                HelpTip::new("󰒔", "Navigate", "Use j/k to highlight choices", TipPriority::Essential),
                HelpTip::new("󰌑", "Confirm", "Press Enter to make your choice", TipPriority::Essential),
                HelpTip::new("󰛓", "Consequences", "Choices affect faction standing and story", TipPriority::Important),
            ],
            
            HelpContext::Inventory => vec![
                HelpTip::new("󰆧", "Select", "Use j/k to navigate items", TipPriority::Essential),
                HelpTip::new("󰌑", "Use", "Press Enter to use consumables", TipPriority::Essential),
                HelpTip::new("󰈆", "Close", "Press Esc to return", TipPriority::Important),
            ],
            
            HelpContext::Stats => vec![
                HelpTip::new("󰄪", "Statistics", "View your run performance", TipPriority::Essential),
                HelpTip::new("󰈆", "Close", "Press Esc to return", TipPriority::Important),
            ],
            
            HelpContext::GameOver | HelpContext::Victory | HelpContext::Tutorial => vec![
                HelpTip::new("󰑓", "Try Again", "Press Enter for a new run", TipPriority::Essential),
                HelpTip::new("󰐀", "Ink Earned", "Currency persists between runs", TipPriority::Important),
                HelpTip::new("󰗼", "Quit", "Press q to exit", TipPriority::Important),
            ],
            
            HelpContext::Dialogue => vec![
                HelpTip::new("󰍪", "Listen", "NPCs reveal lore and secrets", TipPriority::Essential),
                HelpTip::new("󰌌", "Respond", "Type dialogue options to speak", TipPriority::Important),
                HelpTip::new("󰒖", "Reputation", "Responses affect faction standing", TipPriority::Advanced),
            ],
        }
    }
    
    /// Get all keybindings, optionally filtered by context
    pub fn get_keybindings(&self, filter_context: bool) -> Vec<Keybinding> {
        let mut bindings = vec![
            // Global
            Keybinding::new("?/H", "Toggle help"),
            Keybinding::new("Esc", "Cancel/Back/Close"),
            Keybinding::new("q", "Quit game"),
            Keybinding::new("j/↓", "Navigate down"),
            Keybinding::new("k/↑", "Navigate up"),
            Keybinding::new("Enter", "Confirm selection"),
            
            // Combat
            Keybinding::with_context("a-z", "Type characters", HelpContext::Combat),
            Keybinding::with_context("Backspace", "Delete character", HelpContext::Combat),
            Keybinding::with_context("Tab", "Cycle targets", HelpContext::Combat),
            
            // Exploration
            Keybinding::with_context("e", "Explore/Enter room", HelpContext::Exploration),
            Keybinding::with_context("i", "Open inventory", HelpContext::Exploration),
            Keybinding::with_context("s", "View stats", HelpContext::Exploration),
            Keybinding::with_context("m", "View map", HelpContext::Exploration),
            
            // Help navigation
            Keybinding::new("1-4", "Switch help tabs"),
            Keybinding::new("Tab", "Next help tab"),
            Keybinding::new("Shift+Tab", "Previous help tab"),
        ];
        
        if filter_context {
            bindings.retain(|b| {
                b.context.is_none() || b.context == Some(self.context)
            });
        }
        
        bindings
    }
    
    /// Get current objectives based on game state
    pub fn get_objectives(&self, floor: i32, enemies_defeated: i32, has_boss: bool) -> Vec<String> {
        let mut objectives = Vec::new();
        
        match self.context {
            HelpContext::Combat => {
                objectives.push("󰓥 Defeat the enemy".to_string());
                objectives.push("󰈸 Build combos for bonus damage".to_string());
            }
            HelpContext::Exploration => {
                objectives.push(format!("󰏗 Floor {}/10 - Explore and survive", floor));
                if !has_boss {
                    objectives.push("󰮇 Find and defeat the floor boss".to_string());
                }
                objectives.push(format!("󰓥 Enemies defeated: {}", enemies_defeated));
            }
            HelpContext::Shop => {
                objectives.push("󰒍 Purchase useful items".to_string());
                objectives.push("󰆧 Manage your gold wisely".to_string());
            }
            HelpContext::Rest => {
                objectives.push("󰒲 Recover before continuing".to_string());
            }
            _ => {
                objectives.push("󰋗 Press ? for context-sensitive help".to_string());
            }
        }
        
        objectives
    }
    
    /// Get game mechanics explanations
    pub fn get_mechanics(&self) -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
        vec![
            (
                "󰈸 Combo System",
                "Chain words to multiply damage",
                vec![
                    "• Each word without mistakes adds to combo",
                    "• Combo decays after 2 seconds of no typing",
                    "• Max 3x damage multiplier at high combos",
                    "• Perfect words (no backspace) give bonus",
                ],
            ),
            (
                "󰒔 Flow States",
                "Typing rhythm affects critical hits",
                vec![
                    "• Building: Normal gameplay",
                    "• Flowing: Steady rhythm detected (+10% crit)",
                    "• Transcendent: Perfect rhythm (+30% crit)",
                ],
            ),
            (
                "󰐀 Meta-Progression",
                "Progress persists across runs",
                vec![
                    "• Earn Ink currency from every run",
                    "• Unlock permanent bonuses",
                    "• Lore fragments saved to Codex",
                    "• NPC bonds deepen over time",
                ],
            ),
            (
                "󰒖 Factions",
                "Five groups with different philosophies",
                vec![
                    "• Silent Order: Knowledge through observation",
                    "• Echoing Choir: Truth through prophecy",
                    "• Gilded Merchants: Power through commerce",
                    "• Threshold Wardens: Protection at any cost",
                    "• Void Touched: Those who embraced dissolution",
                ],
            ),
            (
                "󰛓 Mystery Progress",
                "Uncover the truth across runs",
                vec![
                    "• 5 tiers of revelation",
                    "• Lore fragments provide clues",
                    "• Faction standing unlocks secrets",
                    "• 12 possible endings",
                ],
            ),
        ]
    }
    
    /// Generate the help hint for the bottom bar (always visible)
    pub fn get_persistent_hint(&self) -> &'static str {
        if self.first_time {
            "󰋗 Press ? or H for help"
        } else {
            "? Help"
        }
    }
}

// ============================================================================
// CONTEXTUAL HINTS (Bottom Bar)
// ============================================================================

/// Triggers for contextual hints that appear in the bottom bar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HintTrigger {
    FirstCombat,
    LowHealth,
    HighCombo,
    MissedPerfect,
    FirstShop,
    FirstElite,
    FirstBoss,
    FirstLoreFragment,
    FirstRest,
    StuckTooLong,
    FlowStateReached,
}

/// A hint that appears in the bottom bar during gameplay
#[derive(Debug, Clone)]
pub struct BottomBarHint {
    pub icon: &'static str,
    pub message: &'static str,
    pub trigger: HintTrigger,
    pub show_once: bool,
    pub duration_secs: f32,
}

impl BottomBarHint {
    pub const fn new(
        icon: &'static str,
        message: &'static str,
        trigger: HintTrigger,
        show_once: bool,
    ) -> Self {
        Self {
            icon,
            message,
            trigger,
            show_once,
            duration_secs: 5.0,
        }
    }
}

/// Predefined hints for various game situations
pub const CONTEXTUAL_HINTS: &[BottomBarHint] = &[
    BottomBarHint::new(
        "󰌌",
        "Type the word to attack! Speed = bonus damage.",
        HintTrigger::FirstCombat,
        true,
    ),
    BottomBarHint::new(
        "󰒲",
        "Health low! Find a rest site to heal.",
        HintTrigger::LowHealth,
        false,
    ),
    BottomBarHint::new(
        "󰈸",
        "Nice combo! Keep it going for bonus damage!",
        HintTrigger::HighCombo,
        true,
    ),
    BottomBarHint::new(
        "󰋖",
        "TIP: No backspaces = PERFECT bonus (+50% damage)",
        HintTrigger::MissedPerfect,
        true,
    ),
    BottomBarHint::new(
        "󰒍",
        "Welcome to the shop! Use j/k to browse, Enter to buy.",
        HintTrigger::FirstShop,
        true,
    ),
    BottomBarHint::new(
        "󰚌",
        "Elite enemy! Harder than normal, but better rewards.",
        HintTrigger::FirstElite,
        true,
    ),
    BottomBarHint::new(
        "󰮇",
        "BOSS BATTLE! Defeat it to advance to the next floor.",
        HintTrigger::FirstBoss,
        true,
    ),
    BottomBarHint::new(
        "󰂺",
        "Lore fragment discovered! Check your Codex.",
        HintTrigger::FirstLoreFragment,
        true,
    ),
    BottomBarHint::new(
        "󰒲",
        "Rest sites let you heal, train, or meditate.",
        HintTrigger::FirstRest,
        true,
    ),
    BottomBarHint::new(
        "󰒔",
        "FLOW STATE! Your rhythm is perfect. +30% crit chance!",
        HintTrigger::FlowStateReached,
        false,
    ),
];

/// Manages the bottom bar hint display
#[derive(Debug, Clone, Default)]
pub struct HintManager {
    /// Hints that have been shown (for show_once hints)
    pub shown_hints: HashSet<HintTrigger>,
    
    /// Currently active hint (if any)
    pub active_hint: Option<(BottomBarHint, f32)>,  // (hint, remaining_time)
}

impl HintManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Try to show a hint for the given trigger
    pub fn try_show(&mut self, trigger: HintTrigger) {
        // Don't interrupt existing hint
        if self.active_hint.is_some() {
            return;
        }
        
        // Find matching hint
        if let Some(hint) = CONTEXTUAL_HINTS.iter().find(|h| h.trigger == trigger) {
            // Check if show_once and already shown
            if hint.show_once && self.shown_hints.contains(&trigger) {
                return;
            }
            
            self.active_hint = Some((hint.clone(), hint.duration_secs));
            
            if hint.show_once {
                self.shown_hints.insert(trigger);
            }
        }
    }
    
    /// Update hint timer
    pub fn tick(&mut self, delta_secs: f32) {
        if let Some((_, ref mut remaining)) = self.active_hint {
            *remaining -= delta_secs;
            if *remaining <= 0.0 {
                self.active_hint = None;
            }
        }
    }
    
    /// Get the current hint message (if any)
    pub fn current_message(&self) -> Option<(&str, &str)> {
        self.active_hint.as_ref().map(|(hint, _)| (hint.icon, hint.message))
    }
    
    /// Clear the active hint
    pub fn clear(&mut self) {
        self.active_hint = None;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_help_system_toggle() {
        let mut help = HelpSystem::new();
        assert!(!help.visible);
        
        help.toggle();
        assert!(help.visible);
        
        help.toggle();
        assert!(!help.visible);
    }
    
    #[test]
    fn test_tab_navigation() {
        let mut help = HelpSystem::new();
        assert_eq!(help.active_tab, HelpTab::Contextual);
        
        help.next_tab();
        assert_eq!(help.active_tab, HelpTab::Keybindings);
        
        help.next_tab();
        assert_eq!(help.active_tab, HelpTab::Objectives);
        
        help.prev_tab();
        assert_eq!(help.active_tab, HelpTab::Keybindings);
    }
    
    #[test]
    fn test_context_conversion() {
        assert_eq!(HelpContext::from(Scene::Combat), HelpContext::Combat);
        assert_eq!(HelpContext::from(Scene::Dungeon), HelpContext::Exploration);
        assert_eq!(HelpContext::from(Scene::Shop), HelpContext::Shop);
    }
    
    #[test]
    fn test_hint_manager() {
        let mut hints = HintManager::new();
        
        hints.try_show(HintTrigger::FirstCombat);
        assert!(hints.active_hint.is_some());
        
        // Can't interrupt
        hints.try_show(HintTrigger::LowHealth);
        assert!(hints.current_message().unwrap().1.contains("Type"));
        
        // Tick down
        hints.tick(6.0);
        assert!(hints.active_hint.is_none());
        
        // show_once already shown
        hints.try_show(HintTrigger::FirstCombat);
        assert!(hints.active_hint.is_none());
    }
}
