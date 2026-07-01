//! Combat and run-progression resources.

use bevy::prelude::*;

/// Tracks the current wave/encounter within a run.
#[derive(Resource, Debug, Clone)]
pub struct WaveState {
    pub wave_number: u32,
    pub enemies_spawned: u32,
    pub enemies_total: u32,
    pub enemies_remaining: u32,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub wave_cooldown: f32,
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
