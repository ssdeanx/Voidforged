//! HUD component markers – all UI element identifiers.
//!
//! Split into logical groups: player frame, target frame, action bar,
//! xp/stamina bars, zone tracker, overlays, and nameplates.

use bevy::prelude::*;
use ir_core::CharacterClass;

// ============================================================================
// In-Game HUD Root
// ============================================================================
#[derive(Component)]
pub struct HudRoot;

// ============================================================================
// Player Unit Frame (top-left)
// ============================================================================
#[derive(Component)]
pub struct HudPlayerFrame;
#[derive(Component)]
pub struct HudPlayerPortrait;
#[derive(Component)]
pub struct HudPlayerNameText;
#[derive(Component)]
pub struct HudPlayerLevelText;

// Health bar
#[derive(Component)]
pub struct HudHealthBar; // fill
#[derive(Component)]
pub struct HudHealthBarBorder; // outer border (class-colored)
#[derive(Component)]
pub struct HudHealthBarText; // "1234/5678"

// Class resource bar
#[derive(Component)]
pub struct HudResourceBar;
#[derive(Component)]
pub struct HudResourceBarFill;
#[derive(Component)]
pub struct HudResourceBarText;

// Stamina bar
#[derive(Component)]
pub struct HudStaminaBar;
#[derive(Component)]
pub struct HudStaminaBarFill;
#[derive(Component)]
pub struct HudStaminaBarText;

// ============================================================================
// Target / Enemy Unit Frame
// ============================================================================
#[derive(Component)]
pub struct HudTargetFrame;
#[derive(Component)]
pub struct HudTargetNameText;
#[derive(Component)]
pub struct HudTargetLevelText;
#[derive(Component)]
pub struct HudTargetHealthBar;
#[derive(Component)]
pub struct HudTargetHealthBarFill;
#[derive(Component)]
pub struct HudTargetHealthPctText;

// ============================================================================
// Action Bar (bottom center – 6 keybinded slots)
// ============================================================================
#[derive(Component)]
pub struct HudActionBar;
#[derive(Component)]
pub struct HudActionBarSlot;
#[derive(Component)]
pub struct HudCooldownOverlay;

// ============================================================================
// XP Bar
// ============================================================================
#[derive(Component)]
pub struct HudXpBar;
#[derive(Component)]
pub struct HudXpBarFill;
#[derive(Component)]
pub struct HudXpBarText;

// ============================================================================
// Zone / Minimap Tracker
// ============================================================================
#[derive(Component)]
pub struct HudZoneFrame;
#[derive(Component)]
pub struct HudZoneText;

// ============================================================================
// Legacy / Misc HUD (kept for backward compat)
// ============================================================================
#[derive(Component)]
pub struct HudLevelText;
#[derive(Component)]
pub struct HudWaveText;
#[derive(Component)]
pub struct HudDashText;
#[derive(Component)]
pub struct HudGoldText;
#[derive(Component)]
pub struct HudPromptText;
#[derive(Component)]
pub struct HudHotbarSlot;
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
#[derive(Component)]
pub struct PauseOverlay;
#[derive(Component)]
pub struct MainMenuRoot;
#[derive(Component)]
pub struct GameOverRoot;

// ============================================================================
// Announcements
// ============================================================================
#[derive(Component)]
pub struct WaveAnnouncement(pub f32);

// ============================================================================
// Character Select (unchanged)
// ============================================================================
#[derive(Component)]
pub struct CharSelectRoot;
#[derive(Component)]
pub struct CharSelectClassCard(pub CharacterClass);
#[derive(Component)]
pub struct CharSelectClassList;
#[derive(Component)]
pub struct CharSelectNameInput;
#[derive(Component)]
pub struct CharSelectConfirmBtn;
#[derive(Component)]
pub struct CharSelectExistingList;
#[derive(Component)]
pub struct CharSelectExistingSlot(pub u32);
#[derive(Component)]
pub struct CharSelectStatsPreview;
#[derive(Component)]
pub struct CharSelectDeleteBtn(pub u32);
