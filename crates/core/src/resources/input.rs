//! Player input resource — unified input state read by gameplay systems.

use bevy::prelude::*;

/// Tracks directional input for player movement and action buttons.
///
/// This resource is populated each frame by the input handling system
/// and consumed by movement, combat, and UI systems. Separating input
/// into a resource decoupling input collection from game logic.
#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerInput {
    /// Normalized 2D movement direction from keyboard or gamepad.
    pub direction: Vec2,
    /// Primary attack triggered (LMB or equivalent).
    pub primary_attack: bool,
    /// Secondary attack triggered (RMB or equivalent).
    pub secondary_attack: bool,
    /// Dodge roll triggered (spacebar or equivalent).
    pub dodge: bool,
    /// Cast / special ability triggered (Q or equivalent).
    pub cast: bool,
    /// Utility ability triggered (F or equivalent, slot 5).
    pub utility: bool,
    /// Ultimate ability triggered (R or equivalent, slot 6).
    pub ultimate: bool,
    /// Interact with objects or NPCs (E or equivalent).
    pub interact: bool,
    /// Toggle pause menu (Escape or equivalent).
    pub pause: bool,
}
