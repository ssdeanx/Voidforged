//! HUD component markers – all UI element identifiers.
//!
//! Split into logical groups: player frame, target frame, action bar,
//! xp/stamina bars, zone tracker, overlays, and nameplates.

use bevy::prelude::*;
use ir_core::CharacterClass;

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

// ============================================================================
// Action Bar (bottom center – 6 keybinded slots)
// ============================================================================

/// Marker for the action bar container.
#[derive(Component)]
pub struct HudActionBar;
/// Marker for an individual action bar slot.
#[derive(Component)]
pub struct HudActionBarSlot;
/// Marker for the cooldown overlay on an ability slot.
#[derive(Component)]
pub struct HudCooldownOverlay;

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
// Enemy Nameplates (3D world-space UI)
// ============================================================================

/// Links a nameplate entity to its enemy entity.
#[derive(Component)]
pub struct EnemyNameplate(pub Entity);

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

// ============================================================================
// Announcements
// ============================================================================

/// Marker for a wave announcement with remaining display time.
#[derive(Component)]
pub struct WaveAnnouncement(pub f32);

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
