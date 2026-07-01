//! HUD component markers for UI queries.

use bevy::prelude::*;
use ir_core::CharacterClass;

// ── In-game HUD ─────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct HudRoot;
#[derive(Component)]
pub struct HudHealthBar;
#[derive(Component)]
pub struct HudXpBar;
#[derive(Component)]
pub struct HudHpText;
#[derive(Component)]
pub struct HudLevelText;
#[derive(Component)]
pub struct HudWaveText;
#[derive(Component)]
pub struct HudDashText;
#[derive(Component)]
pub struct HudGoldText;
#[derive(Component)]
pub struct HudZoneText;
#[derive(Component)]
pub struct HudPromptText;
#[derive(Component)]
pub struct HudHotbarSlot;
#[derive(Component)]
pub struct HudCharacterPanel;

// ── Stamina ─────────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct HudStaminaBar;
#[derive(Component)]
pub struct HudStaminaText;

// ── Character Select ────────────────────────────────────────────────────────
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

// ── Overlays ────────────────────────────────────────────────────────────────
#[derive(Component)]
pub struct PauseOverlay;
#[derive(Component)]
pub struct MainMenuRoot;
#[derive(Component)]
pub struct GameOverRoot;

// ── Announcements ───────────────────────────────────────────────────────────
#[derive(Component)]
pub struct WaveAnnouncement(pub f32);
