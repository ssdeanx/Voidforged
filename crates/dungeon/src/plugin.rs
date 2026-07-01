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
            .add_systems(OnExit(AppState::Dungeon), (
                cleanup_dungeon,
            ))
            .add_systems(Update, (
                check_dungeon_exit.run_if(in_state(AppState::Dungeon)),
            ));
    }
}

/// Despawns all entities marked with DungeonEntity on dungeon exit.
fn cleanup_dungeon(
    mut commands: Commands,
    entities: Query<Entity, With<DungeonEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
    info!("Dungeon cleaned up ({} entities despawned)", entities.iter().count());
}
