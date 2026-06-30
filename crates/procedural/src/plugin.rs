use bevy::prelude::*;
use ir_core::*;
use crate::waves;

pub struct ProceduralPlugin;

impl Plugin for ProceduralPlugin {
    fn build(&self, app: &mut App) {
        app
            // Wave spawning
            .add_systems(Update, (
                waves::spawn_wave,
                waves::check_wave_cleared,
            ).run_if(in_state(AppState::Playing)));
    }
}
