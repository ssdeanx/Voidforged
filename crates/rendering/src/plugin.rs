use bevy::prelude::*;
use crate::{
    assets,
    camera::{self},
    lighting,
    spawn,
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup — camera + lights
            .add_systems(Startup, (
                camera::spawn_isometric_camera,
                lighting::setup_lighting,
            ))

            // Loading state — generate placeholders, then go to MainMenu
            .add_systems(OnEnter(ir_core::AppState::Loading), (
                assets::generate_placeholder_assets,
            ))

            // MainMenu — spawn world, then auto-advance to Playing
            .add_systems(OnEnter(ir_core::AppState::MainMenu), (
                spawn::spawn_game_world,
            ))

            // Playing — camera follows player
            .add_systems(Update, (
                camera::follow_player.run_if(in_state(ir_core::AppState::Playing)),
            ));
    }
}
