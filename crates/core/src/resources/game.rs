//! General game configuration and runtime state.

use bevy::prelude::*;

/// Static game configuration loaded from assets.
#[derive(Resource, Debug, Clone)]
pub struct GameConfig {
    pub starting_weapon: String,
    pub max_enemies_on_screen: u32,
    pub camera_follow_speed: f32,
    pub xp_magnet_radius: f32,
    pub gem_attract_speed: f32,
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

/// Tracks the elapsed time of the current play session.
#[derive(Resource, Debug, Clone)]
pub struct PlayTimer(pub f32);

impl Default for PlayTimer {
    fn default() -> Self { Self(0.0) }
}

/// Mouse cursor position projected into 3D world space (y=0 plane).
#[derive(Resource, Default, Debug, Clone)]
pub struct CursorWorldPos(pub Vec3);

/// Camera's current world transform — shared so gameplay can do camera-relative math.
#[derive(Resource, Debug, Clone)]
pub struct CameraTransform(pub Vec3, pub Quat);

impl Default for CameraTransform {
    fn default() -> Self {
        Self(Vec3::new(0.0, 20.0, 20.0), Quat::IDENTITY)
    }
}

/// Screen shake intensity for hit feedback.
#[derive(Resource, Debug, Clone)]
pub struct ScreenShake {
    pub trauma: f32,
    pub decay: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self { trauma: 0.0, decay: 5.0 }
    }
}
