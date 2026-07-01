//! Dungeon state resources.

use bevy::prelude::*;

/// Tracks which dungeon the player is currently in.
#[derive(Resource, Debug, Clone, Default)]
pub struct DungeonState {
    pub current: Option<DungeonInstance>,
}

#[derive(Debug, Clone)]
pub struct DungeonInstance {
    pub name: String,
    pub tier: u32,
    pub depth: u32,
}
