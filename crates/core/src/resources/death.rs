//! Death and respawn resources — open-world death penalties and graveyards.

use bevy::prelude::*;

/// Tracks player death penalty percentages for open-world respawn.
///
/// When the player dies in the open world, they lose a percentage of
/// their XP and gold as a penalty. These rates are configured here.
#[derive(Resource, Debug, Clone)]
pub struct DeathPenalty {
    /// Percentage of current XP lost on death (0.0–1.0).
    pub xp_loss_pct: f32,
    /// Percentage of current gold lost on death (0.0–1.0).
    pub gold_loss_pct: f32,
}

impl Default for DeathPenalty {
    fn default() -> Self {
        Self { xp_loss_pct: 0.10, gold_loss_pct: 0.15 }
    }
}

/// Respawn point for open-world death (graveyard location).
///
/// When the player dies outside a dungeon, they respawn at this location
/// with death penalties applied. Set by the world-generation systems.
#[derive(Resource, Debug, Clone)]
pub struct Graveyard {
    /// World position of the respawn point.
    pub position: Vec3,
}

impl Default for Graveyard {
    fn default() -> Self {
        Self { position: Vec3::new(0.0, 0.0, 0.0) }
    }
}

/// One-shot resource used to apply the correct class resource on character spawn.
///
/// Set when transitioning from character creation to gameplay so that
/// the spawning system knows which class-specific resource (Rage, Mana, etc.)
/// to attach to the new player entity.
#[derive(Resource)]
pub struct PendingClassSpawn {
    /// The character class to use for resource generation on spawn.
    pub class: Option<crate::components::CharacterClass>,
}
