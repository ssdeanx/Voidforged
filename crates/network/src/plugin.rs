use bevy::prelude::*;

/// Stub plugin — enables networking features when the `multiplayer` feature flag is set.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: Initialize bevy_replicon or lightyear, set up ECS replication
    }
}
