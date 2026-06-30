use bevy::prelude::*;
use ir_core::*;
use crate::{leveling, upgrades};

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(upgrades::UpgradesPlugin)
            .add_systems(Update, (
                leveling::handle_xp_gain,
                leveling::apply_level_up,
            ).run_if(in_state(AppState::Playing)));
    }
}
