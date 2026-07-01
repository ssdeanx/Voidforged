//! Dungeon state resources — tracks the current dungeon instance.

use bevy::prelude::*;

/// Tracks which dungeon the player is currently inside.
///
/// When `current` is `None` the player is in the open world.
/// Set on dungeon entry, cleared on exit or completion.
#[derive(Resource, Debug, Clone, Default)]
pub struct DungeonState {
    /// The active dungeon instance, if any.
    pub current: Option<DungeonInstance>,
}

/// Descriptor for a single dungeon instance the player has entered.
#[derive(Debug, Clone)]
pub struct DungeonInstance {
    /// Display name of the dungeon.
    pub name: String,
    /// Difficulty tier of this dungeon.
    pub tier: u32,
    /// Current floor / depth reached inside the dungeon.
    pub depth: u32,
}
