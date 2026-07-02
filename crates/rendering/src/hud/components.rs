//! HUD component markers – all UI element identifiers.
//!
//! Split into logical groups: player frame, target frame, action bar,
//! xp/stamina bars, zone tracker, overlays, and nameplates.

use bevy::prelude::*;
use ir_core::CharacterClass;

// ============================================================================
// WoW-style Class Colors (as defined in the task spec)
// ============================================================================

/// WoW class colors matching the spec: matches the iconic class palette.
pub fn class_primary_color(class: CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.78, 0.61, 0.43),  // #C79C6E tan
        CharacterClass::Paladin => Color::srgb(0.96, 0.55, 0.73),  // #F58CBA pink
        CharacterClass::Rogue => Color::srgb(1.00, 0.96, 0.41),    // #FFF569 yellow
        CharacterClass::Hunter => Color::srgb(0.67, 0.83, 0.45),   // #ABD473 green
        CharacterClass::Mage => Color::srgb(0.41, 0.80, 0.94),     // #69CCF0 blue
    }
}

/// Border glow color — slightly darker version of the primary class color.
pub fn class_border_glow(class: CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.50, 0.35, 0.20),
        CharacterClass::Paladin => Color::srgb(0.70, 0.30, 0.50),
        CharacterClass::Rogue => Color::srgb(0.75, 0.70, 0.20),
        CharacterClass::Hunter => Color::srgb(0.40, 0.60, 0.25),
        CharacterClass::Mage => Color::srgb(0.20, 0.50, 0.70),
    }
}

/// Color for the class resource bar.
pub fn resource_bar_color(class: CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.75, 0.10, 0.10),  // Rage = red
        CharacterClass::Paladin => Color::srgb(1.00, 0.84, 0.00),  // Holy Power = gold
        CharacterClass::Rogue => Color::srgb(1.00, 0.84, 0.00),    // Energy = yellow
        CharacterClass::Hunter => Color::srgb(0.00, 0.75, 0.38),   // Focus = green
        CharacterClass::Mage => Color::srgb(0.00, 0.40, 0.80),     // Mana = blue
    }
}

// ============================================================================
// In-Game HUD Root
// ============================================================================

/// Root marker for the in-game HUD container.
#[derive(Component)]
pub struct HudRoot;

// ============================================================================
// Player Unit Frame (top-left)
// ============================================================================

/// Marker for the player unit frame container.
#[derive(Component)]
pub struct HudPlayerFrame;
/// Marker for the player portrait element.
#[derive(Component)]
pub struct HudPlayerPortrait;
/// Marker for the player name text element.
#[derive(Component)]
pub struct HudPlayerNameText;
/// Marker for the player level text element.
#[derive(Component)]
pub struct HudPlayerLevelText;

// Health bar
/// Marker for the health bar fill element.
#[derive(Component)]
pub struct HudHealthBar;
/// Marker for the health bar outer border (class-colored).
#[derive(Component)]
pub struct HudHealthBarBorder;
/// Marker for the health bar text ("1234/5678").
#[derive(Component)]
pub struct HudHealthBarText;

/// Tracks the current displayed health percentage for smooth lerping.
#[derive(Component)]
pub struct HudHealthDisplay {
    pub display_pct: f32,
}

impl Default for HudHealthDisplay {
    fn default() -> Self {
        Self { display_pct: 1.0 }
    }
}

// Class resource bar
/// Marker for the class resource bar container.
#[derive(Component)]
pub struct HudResourceBar;
/// Marker for the class resource bar fill element.
#[derive(Component)]
pub struct HudResourceBarFill;
/// Marker for the class resource bar text.
#[derive(Component)]
pub struct HudResourceBarText;

/// Tracks the current displayed resource percentage for smooth lerping.
#[derive(Component)]
pub struct HudResourceDisplay {
    pub display_pct: f32,
}

impl Default for HudResourceDisplay {
    fn default() -> Self {
        Self { display_pct: 1.0 }
    }
}

// Stamina bar
/// Marker for the stamina bar container.
#[derive(Component)]
pub struct HudStaminaBar;
/// Marker for the stamina bar fill element.
#[derive(Component)]
pub struct HudStaminaBarFill;
/// Marker for the stamina bar text.
#[derive(Component)]
pub struct HudStaminaBarText;

// ============================================================================
// Target / Enemy Unit Frame
// ============================================================================

/// Marker for the target unit frame container.
#[derive(Component)]
pub struct HudTargetFrame;
/// Marker for the target name text element.
#[derive(Component)]
pub struct HudTargetNameText;
/// Marker for the target level text element.
#[derive(Component)]
pub struct HudTargetLevelText;
/// Marker for the target health bar background.
#[derive(Component)]
pub struct HudTargetHealthBar;
/// Marker for the target health bar fill element.
#[derive(Component)]
pub struct HudTargetHealthBarFill;
/// Marker for the target health percentage text.
#[derive(Component)]
pub struct HudTargetHealthPctText;
#[derive(Component)]
pub struct HudTargetEliteBorder; // dragon/special border for elite mobs

/// Tracks current displayed target health % for smooth lerping.
#[derive(Component)]
pub struct HudTargetHealthDisplay {
    pub display_pct: f32,
}

impl Default for HudTargetHealthDisplay {
    fn default() -> Self {
        Self { display_pct: 1.0 }
    }
}

// ============================================================================
// Action Bar (bottom center – 6+ keybinded slots)
// ============================================================================

/// Marker for the action bar container.
#[derive(Component)]
pub struct HudActionBar;
/// Marker for an individual action bar slot.
#[derive(Component)]
pub struct HudActionBarSlot;
/// Keybind label on an action slot (e.g. "1", "2", "Q", "E")
#[derive(Component)]
pub struct HudKeybindLabel;
/// The icon/placeholder inside an action slot
#[derive(Component)]
pub struct HudActionBarIcon;
/// Cooldown sweep overlay — descends from top to bottom as ability recharges
#[derive(Component)]
pub struct HudCooldownOverlay {
    pub remaining: f32,
    pub max: f32,
}

// ============================================================================
// XP Bar
// ============================================================================

/// Marker for the XP bar container.
#[derive(Component)]
pub struct HudXpBar;
/// Marker for the XP bar fill element.
#[derive(Component)]
pub struct HudXpBarFill;
/// Marker for the XP bar text.
#[derive(Component)]
pub struct HudXpBarText;

// ============================================================================
// Zone / Minimap Tracker
// ============================================================================

/// Marker for the zone name / minimap frame.
#[derive(Component)]
pub struct HudZoneFrame;
/// Marker for the zone name text element.
#[derive(Component)]
pub struct HudZoneText;

// ============================================================================
// Legacy / Misc HUD (kept for backward compat)
// ============================================================================

/// Marker for legacy level text.
#[derive(Component)]
pub struct HudLevelText;
/// Marker for legacy wave count text.
#[derive(Component)]
pub struct HudWaveText;
/// Marker for the dash cooldown text.
#[derive(Component)]
pub struct HudDashText;
/// Marker for the gold count text.
#[derive(Component)]
pub struct HudGoldText;
/// Marker for the interaction prompt text element.
#[derive(Component)]
pub struct HudPromptText;
/// Marker for a legacy hotbar slot.
#[derive(Component)]
pub struct HudHotbarSlot;
/// Marker for the character panel container.
#[derive(Component)]
pub struct HudCharacterPanel;

// ============================================================================
// Inventory UI
// ============================================================================
/// Root container for the inventory grid panel.
#[derive(Component)]
pub struct HudInventory;
/// A single inventory slot. Stores its index for item lookups.
#[derive(Component)]
pub struct HudInventorySlot(pub usize);
/// Marked on the currently selected inventory slot.
#[derive(Component)]
pub struct HudInventorySelected;
/// Gold display text inside the inventory panel.
#[derive(Component)]
pub struct HudInventoryGold;

// ============================================================================
// Equipment Screen
// ============================================================================
/// Root container for the equipment paperdoll panel.
#[derive(Component)]
pub struct HudEquipment;
/// A single equipment slot. Stores which equip slot it represents.
#[derive(Component)]
pub struct HudEquipSlot(pub ir_core::EquipSlot);
/// GearScore total text.
#[derive(Component)]
pub struct HudGearScoreText;

// ============================================================================
// Item Tooltips
// ============================================================================
/// Root container for the item tooltip popup.
#[derive(Component)]
pub struct HudTooltip;
/// One line of tooltip text (item name, stats, etc.).
#[derive(Component)]
pub struct HudTooltipLine(pub usize);

// ============================================================================
// Enemy Nameplates (3D world-space UI)
// ============================================================================

/// Links a nameplate entity to its enemy entity.
#[derive(Component)]
pub struct EnemyNameplate(pub Entity);

/// Stores the spawned nameplate children for position updates.
#[derive(Component)]
pub struct NameplateChildren {
    pub name_entity: Entity,
    pub bar_bg_entity: Entity,
    pub bar_fill_entity: Entity,
    pub pct_entity: Entity,
}

// ============================================================================
// Damage Numbers
// ============================================================================
/// Animation state for a floating damage number.
#[derive(Component)]
pub struct DamageNumberAnim {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub is_crit: bool,
}

// ============================================================================
// Overlays & Menus
// ============================================================================

/// Marker for the pause overlay.
#[derive(Component)]
pub struct PauseOverlay;
/// Marker for the main menu root.
#[derive(Component)]
pub struct MainMenuRoot;
/// Marker for the game over screen root.
#[derive(Component)]
pub struct GameOverRoot;
#[derive(Component)]
pub struct UpgradeTreeRoot;

// ============================================================================
// Announcements
// ============================================================================

/// Marker for a wave announcement with remaining display time.
#[derive(Component)]
pub struct WaveAnnouncement(pub f32);

// ============================================================================
// Minimap
// ============================================================================
#[derive(Component)]
pub struct HudMinimap;
#[derive(Component)]
pub struct HudMinimapContainer;
#[derive(Component)]
pub struct HudMinimapPlayerDot;
#[derive(Component)]
pub struct HudMinimapEnemyDot;

// ============================================================================
// Buff/Debuff Indicators
// ============================================================================
#[derive(Component)]
pub struct HudBuffBar;
#[derive(Component)]
pub struct HudBuffIcon {
    pub kind: ir_core::AbilityKind,
    pub remaining: f32,
    pub max_duration: f32,
    pub is_debuff: bool,
}

// ============================================================================
// Level-Up Popup
// ============================================================================
#[derive(Component)]
pub struct HudLevelUpPopup {
    pub timer: f32,
}
#[derive(Component)]
pub struct HudLevelUpText;

// ============================================================================
// Settings Screen
// ============================================================================
#[derive(Component)]
pub struct HudSettings;
#[derive(Component)]
pub struct HudSettingsButton;
/// Tracks the master volume level (0.0–1.0).
#[derive(Resource, Debug, Clone)]
pub struct AudioVolume(pub f32);
impl Default for AudioVolume {
    fn default() -> Self { Self(0.5) }
}
/// Marker on the volume slider fill bar (width driven by AudioVolume).
#[derive(Component)]
pub struct HudVolumeFill;
/// Marker on the volume percentage text (e.g. "50%").
#[derive(Component)]
pub struct HudVolumeText;

// ============================================================================
// Character Select (unchanged)
// ============================================================================

/// Marker for the character select screen root.
#[derive(Component)]
pub struct CharSelectRoot;
/// Marker for a class selection card, tagged with the class.
#[derive(Component)]
pub struct CharSelectClassCard(pub CharacterClass);
/// Marker for the class selection list container.
#[derive(Component)]
pub struct CharSelectClassList;
/// Marker for the character name input field.
#[derive(Component)]
pub struct CharSelectNameInput;
/// Marker for the confirm button.
#[derive(Component)]
pub struct CharSelectConfirmBtn;
/// Marker for the existing characters list.
#[derive(Component)]
pub struct CharSelectExistingList;
/// Marker for an existing character slot, tagged with the profile id.
#[derive(Component)]
pub struct CharSelectExistingSlot(pub u32);
/// Marker for the stats preview panel.
#[derive(Component)]
pub struct CharSelectStatsPreview;
/// Marker for the delete character button, tagged with the profile id.
#[derive(Component)]
pub struct CharSelectDeleteBtn(pub u32);
/// Back button on character select screen — returns to main menu.
#[derive(Component)]
pub struct CharSelectBackBtn;
/// Marker set after populate_existing_characters runs once per spawn.
#[derive(Component)]
pub struct CharSelectPopulated;

// ============================================================================
// Main Menu Buttons
// ============================================================================
#[derive(Component)]
pub struct MainMenuSettingsBtn;
#[derive(Component)]
pub struct MainMenuQuitBtn;

// ============================================================================
// Main Menu Background Particles
// ============================================================================
/// Marker for the main menu background particle system entity.
#[derive(Component)]
pub struct MenuBackgroundParticles;
