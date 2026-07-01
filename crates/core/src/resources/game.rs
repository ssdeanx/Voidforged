//! General game configuration and runtime state.

use bevy::prelude::*;

/// Static game configuration loaded from assets.
///
/// Controls tuning parameters such as starting equipment, enemy caps, camera behaviour,
/// and XP magnet mechanics. Populated by the loading systems and read by gameplay systems.
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    /// Identifier of the starting weapon entity definition.
    pub starting_weapon: String,
    /// Maximum number of enemies allowed on screen at once.
    pub max_enemies_on_screen: u32,
    /// Speed at which the camera follows the player.
    pub camera_follow_speed: f32,
    /// Radius at which the player attracts nearby XP gems.
    pub xp_magnet_radius: f32,
    /// Speed of gem attraction toward the player.
    pub gem_attract_speed: f32,
    /// Whether floating damage numbers are shown on hit.
    pub damage_numbers: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            starting_weapon: "magic_missile".to_string(),
            max_enemies_on_screen: 100,
            camera_follow_speed: 8.0,
            xp_magnet_radius: 6.0,
            gem_attract_speed: 12.0,
            damage_numbers: true,
        }
    }
}

/// Tracks the elapsed time (in seconds) of the current play session.
///
/// Updated each frame by the `update_play_timer` system in [`CorePlugin`](crate::plugin::CorePlugin)
/// when the app is in a gameplay state.
#[derive(Resource, Debug, Clone)]
pub struct PlayTimer(pub f32);

impl Default for PlayTimer {
    fn default() -> Self { Self(0.0) }
}

/// Mouse cursor position projected into 3D world space on the y=0 plane.
///
/// Updated each frame by the input system so gameplay code can read the
/// player's aimed target without querying the camera every time.
#[derive(Resource, Default, Debug, Clone)]
pub struct CursorWorldPos(pub Vec3);

/// Camera's current world transform (position and rotation).
///
/// Shared resource so gameplay systems can perform camera-relative math
/// (e.g. spawning projectiles in front of the camera) without querying
/// the camera entity directly.
#[derive(Resource, Debug, Clone)]
pub struct CameraTransform(pub Vec3, pub Quat);

impl Default for CameraTransform {
    fn default() -> Self {
        Self(Vec3::new(0.0, 20.0, 20.0), Quat::IDENTITY)
    }
}

/// Screen shake intensity for hit feedback and explosions.
///
/// `trauma` is reduced each frame by `decay`; camera systems read this to
/// apply a random offset proportional to the current trauma level.
#[derive(Resource, Debug, Clone)]
pub struct ScreenShake {
    /// Current shake intensity (0.0–1.0+).
    pub trauma: f32,
    /// Trauma decay per second.
    pub decay: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self { trauma: 0.0, decay: 5.0 }
    }
}
