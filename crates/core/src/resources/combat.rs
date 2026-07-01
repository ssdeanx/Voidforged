//! Combat and run-progression resources — wave tracking and run statistics.

use bevy::prelude::*;

/// Tracks the current wave or encounter within a run.
///
/// Managed by the dungeon/encounter systems to spawn enemies in waves,
/// track remaining enemies, and scale difficulty over time.
#[derive(Resource, Debug, Clone)]
pub struct WaveState {
    /// Current wave number (1-indexed).
    pub wave_number: u32,
    /// Number of enemies spawned so far this wave.
    pub enemies_spawned: u32,
    /// Total enemies to spawn this wave.
    pub enemies_total: u32,
    /// Enemies still alive this wave.
    pub enemies_remaining: u32,
    /// Timer before the next enemy spawns (seconds).
    pub spawn_timer: f32,
    /// Interval between individual enemy spawns (seconds).
    pub spawn_interval: f32,
    /// Cooldown between waves (seconds).
    pub wave_cooldown: f32,
    /// Difficulty scaling multiplier (applied to enemy stats per wave).
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

/// Tracks kills, damage, and other statistics for the current run.
///
/// Used by the game-over screen and end-of-run summary to display stats.
/// Updated by combat systems throughout the run.
#[derive(Resource, Debug, Clone)]
pub struct RunProgression {
    /// Total enemies killed this run.
    pub kills: u32,
    /// Total damage dealt to enemies this run.
    pub damage_dealt: f32,
    /// Total damage taken by the player this run.
    pub damage_taken: f32,
    /// Total gold collected this run.
    pub gold_collected: u64,
    /// Total XP earned this run.
    pub xp_earned: u64,
    /// Number of rooms cleared this run.
    pub rooms_cleared: u32,
    /// Total elapsed run time in seconds.
    pub run_time: f32,
    /// Current depth / zone number reached.
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
