//! Player input resource.

use bevy::prelude::*;

/// Tracks directional input for player movement.
#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerInput {
    pub direction: Vec2,
    pub primary_attack: bool,
    pub secondary_attack: bool,
    pub dodge: bool,
    pub cast: bool,
    pub interact: bool,
    pub pause: bool,
}
