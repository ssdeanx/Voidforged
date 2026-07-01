use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Game State
// ============================================================================

/// Top-level application states.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    World,       // ← NEW: open world exploration
    Dungeon,     // ← NEW: inside a dungeon instance
    Playing,     // kept for backward compat / combat
    Paused,
    GameOver,
}

/// Run state — lifecycle of a single run.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunState {
    #[default]
    Entering,
    Exploring,
    Combat,
    RoomTransition,
    Boss,
    Victory,
    Defeat,
}

// ============================================================================
// Wave & Encounter Resources
// ============================================================================

/// Tracks the current wave/encounter within a run.
#[derive(Resource, Debug, Clone)]
pub struct WaveState {
    pub wave_number: u32,
    pub enemies_spawned: u32,
    pub enemies_total: u32,
    pub enemies_remaining: u32,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub wave_cooldown: f32,          // time between waves
    pub difficulty_multiplier: f32,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            wave_number: 1,
            enemies_spawned: 0,
            enemies_total: 8,
            enemies_remaining: 0,
            spawn_timer: 0.0,
            spawn_interval: 1.5,
            wave_cooldown: 5.0,
            difficulty_multiplier: 1.0,
        }
    }
}

// ============================================================================
// Player Progression (per-run)
// ============================================================================

/// Tracks kills and damage per run so the game over screen can show stats.
#[derive(Resource, Debug, Clone)]
pub struct RunProgression {
    pub kills: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
    pub gold_collected: u64,
    pub xp_earned: u64,
    pub rooms_cleared: u32,
    pub run_time: f32,
    pub current_zone: u32,
}

impl Default for RunProgression {
    fn default() -> Self {
        Self {
            kills: 0,
            damage_dealt: 0.0,
            damage_taken: 0.0,
            gold_collected: 0,
            xp_earned: 0,
            rooms_cleared: 0,
            run_time: 0.0,
            current_zone: 1,
        }
    }
}

// ============================================================================
// Meta-Progression (persistent across runs)
// ============================================================================

/// Persistent meta-progression data saved between runs.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgression {
    /// Hard currency (earned from runs, spent on permanent upgrades)
    pub dark_essence: u64,
    /// Soft currency (spent during runs)
    pub gold: u64,
    /// Total runs attempted
    pub total_runs: u32,
    /// Total runs completed (reached victory/ending)
    pub completed_runs: u32,
    /// Highest wave reached
    pub highest_wave: u32,
    /// Unlocked weapon/ability IDs
    pub unlocks: Vec<String>,
    /// Purchased upgrade tiers
    pub upgrades: Vec<UpgradeTier>,
}

impl Default for MetaProgression {
    fn default() -> Self {
        Self {
            dark_essence: 0,
            gold: 0,
            total_runs: 0,
            completed_runs: 0,
            highest_wave: 0,
            unlocks: vec!["dagger".to_string()],
            upgrades: Vec::new(),
        }
    }
}

/// A purchased tier of a permanent upgrade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeTier {
    pub id: String,
    pub tier: u32,
    pub cost: u64,
}

// ============================================================================
// Input Resources
// ============================================================================

/// Tracks directional input for player movement.
#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerInput {
    pub direction: Vec2,
    pub primary_attack: bool,
    pub secondary_attack: bool,
    pub dodge: bool,
    pub cast: bool,
    pub interact: bool,
    pub pause: bool,
}

// ============================================================================
// Game Config
// ============================================================================

/// Tracks the elapsed time of the current play session.
#[derive(Resource, Debug, Clone)]
pub struct PlayTimer(pub f32);

impl Default for PlayTimer {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Tracks which dungeon the player is currently in.
#[derive(Resource, Debug, Clone, Default)]
pub struct DungeonState {
    pub current: Option<DungeonInstance>,
}

#[derive(Debug, Clone)]
pub struct DungeonInstance {
    pub name: String,
    pub tier: u32,
    pub depth: u32,
}

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
        Self {
            trauma: 0.0,
            decay: 5.0,
        }
    }
}

// ============================================================================
// Asset Resources
// ============================================================================

/// Holds handles to loaded game assets.
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub player_mesh: Handle<Mesh>,
    pub player_material: Handle<StandardMaterial>,
    pub enemy_meshes: Vec<Handle<Mesh>>,
    pub enemy_materials: Vec<Handle<StandardMaterial>>,
    pub projectile_mesh: Handle<Mesh>,
    pub projectile_material: Handle<StandardMaterial>,
    pub gem_mesh: Handle<Mesh>,
    pub gem_material: Handle<StandardMaterial>,
    pub health_pickup_mesh: Handle<Mesh>,
    pub health_pickup_material: Handle<StandardMaterial>,
    pub gold_pickup_mesh: Handle<Mesh>,
    pub gold_pickup_material: Handle<StandardMaterial>,
    pub floor_mesh: Handle<Mesh>,
    pub floor_material: Handle<StandardMaterial>,
    pub tile_mesh: Handle<Mesh>,
    pub tile_material: Handle<StandardMaterial>,
    pub tile_material_alt: Handle<StandardMaterial>,
    pub wall_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
    pub shadow_mesh: Handle<Mesh>,
    pub shadow_material: Handle<StandardMaterial>,
    pub environment_meshes: Vec<Handle<Mesh>>,
}
