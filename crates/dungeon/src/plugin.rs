//! Dungeon plugin.

use bevy::prelude::*;
use ir_core::*;
use crate::rooms::*;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Dungeon), (
                generate_dungeon,
            ))
            .add_systems(Update, (
                check_dungeon_exit.run_if(in_state(AppState::Dungeon)),
            ));
    }
}
