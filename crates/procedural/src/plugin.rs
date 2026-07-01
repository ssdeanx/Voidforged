//! Plugin that registers procedural generation systems — wave spawning and checking.

use bevy::prelude::*;
use ir_core::*;
use crate::waves;

/// Bevy plugin for procedural wave-based enemy spawning.
///
/// Registers `spawn_wave` and `check_wave_cleared` systems that run during
/// the `Playing` game state.
pub struct ProceduralPlugin;

impl Plugin for ProceduralPlugin {
    fn build(&self, app: &mut App) {
        app
            // Wave spawning
            .add_systems(Update, (
                waves::spawn_wave,
                waves::check_wave_cleared,
            ).run_if(in_state(AppState::Playing).or(in_state(AppState::Dungeon))));
    }
}
