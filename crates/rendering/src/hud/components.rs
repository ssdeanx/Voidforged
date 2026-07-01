//! HUD component markers for UI queries.

use bevy::prelude::*;

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
