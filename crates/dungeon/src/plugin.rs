//! Dungeon plugin — wires dungeon generation and exit handling during the
//! `Dungeon` game state, and raid generation for the `Raid` state.

use crate::{raid, rooms::*};
use bevy::prelude::*;
use ir_core::*;

/// Bevy plugin for procedural dungeon generation and raid handling.
///
/// Registers dungeon systems on `AppState::Dungeon` and raid systems
/// on `AppState::Raid`. Also registers shared raid resources.
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app
            // ── Shared raid resources ─────────────────────────────────
            .init_resource::<raid::RaidState>()
            // ── Dungeon systems ───────────────────────────────────────
            .add_systems(OnEnter(AppState::Dungeon), (generate_dungeon,))
            .add_systems(OnExit(AppState::Dungeon), (cleanup_dungeon,))
            .add_systems(
                Update,
                (check_dungeon_exit.run_if(in_state(AppState::Dungeon)),),
            )
            // ── Raid systems ──────────────────────────────────────────
            .add_systems(OnEnter(AppState::Raid), (raid::generate_raid,))
            .add_systems(OnExit(AppState::Raid), (raid::cleanup_raid,))
            .add_systems(
                Update,
                (
                    raid::check_raid_exit.run_if(in_state(AppState::Raid)),
                    raid::ai_companion_ai.run_if(in_state(AppState::Raid)),
                ),
            );
    }
}

/// Despawns all entities marked with DungeonEntity on dungeon exit.
fn cleanup_dungeon(mut commands: Commands, entities: Query<Entity, With<DungeonEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
    info!(
        "Dungeon cleaned up ({} entities despawned)",
        entities.iter().count()
    );
}
