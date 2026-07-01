//! Dungeon plugin — wires dungeon generation and exit handling during the
//! `Dungeon` game state.

use bevy::prelude::*;
use ir_core::*;
use crate::rooms::*;

/// Bevy plugin for procedural dungeon generation and exit handling.
///
/// Registers `generate_dungeon` on entering the `Dungeon` state and
/// `check_dungeon_exit` during `Update` while in the `Dungeon` state.
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
