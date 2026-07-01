//! Plugin for meta-progression — experience gain, level-up, and upgrades.

use bevy::prelude::*;
use ir_core::*;
use crate::{leveling, upgrades};

/// Bevy plugin for meta-progression systems.
///
/// Registers XP handling and level-up systems that run during the
/// `Dungeon` and `Playing` game states. Also adds the upgrades sub-plugin.
pub struct ProgressionPlugin;

fn can_gain_xp(state: Res<State<AppState>>) -> bool {
    matches!(*state.get(), AppState::Dungeon | AppState::Playing)
}

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(upgrades::UpgradesPlugin)
            .add_systems(Update, (
                leveling::handle_xp_gain,
                leveling::apply_level_up,
            ).chain().run_if(can_gain_xp));
    }
}
