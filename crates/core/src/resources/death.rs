//! Death and respawn resources.

use bevy::prelude::*;

/// Tracks player death penalty state for open-world respawn.
#[derive(Resource, Debug, Clone)]
pub struct DeathPenalty {
    pub xp_loss_pct: f32,
    pub gold_loss_pct: f32,
}

impl Default for DeathPenalty {
    fn default() -> Self {
        Self { xp_loss_pct: 0.10, gold_loss_pct: 0.15 }
    }
}

/// Respawn point for open-world death (graveyard).
#[derive(Resource, Debug, Clone)]
pub struct Graveyard {
    pub position: Vec3,
}

impl Default for Graveyard {
    fn default() -> Self {
        Self { position: Vec3::new(0.0, 0.0, 0.0) }
    }
}

/// Tracking system for applying class resource on spawn.
#[derive(Resource)]
pub struct PendingClassSpawn {
    pub class: Option<crate::components::CharacterClass>,
}
