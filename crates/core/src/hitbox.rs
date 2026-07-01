//! Hitbox system — spatial damage zones with configurable shapes.
//! Components live in ir_core, processing systems in ir_gameplay.

use crate::components::ProjectileOwner;
use crate::DamageType;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Hitbox Shapes
// ============================================================================

/// The shape of a damage hitbox.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub enum HitboxShape {
    /// Cone in front of the origin (melee cleave).
    Cone { range: f32, half_angle: f32 },
    /// Circle around the origin (AoE explosion).
    Circle { radius: f32 },
    /// Rectangle in front (charge, line attack).
    Rect { width: f32, length: f32 },
    /// Single-target point check.
    Point { range: f32 },
}

// ============================================================================
// Damage Hitbox Component
// ============================================================================

/// A temporary entity that deals damage to enemies it overlaps.
/// Spawned by attacks, despawned after `lifetime` seconds.
#[derive(Debug, Clone, Component)]
pub struct DamageHitbox {
    pub shape: HitboxShape,
    pub damage: f32,
    pub source: Entity,
    pub damage_type: DamageType,
    pub lifetime: f32,
    pub max_lifetime: f32,
    /// Tracks enemies already hit (anti-double-tap).
    pub hit_enemies: Vec<Entity>,
    pub owner: ProjectileOwner,
    pub knockback: f32,
}

impl DamageHitbox {
    pub fn new(
        shape: HitboxShape,
        damage: f32,
        source: Entity,
        damage_type: DamageType,
        lifetime: f32,
        owner: ProjectileOwner,
        knockback: f32,
    ) -> Self {
        Self {
            shape,
            damage,
            source,
            damage_type,
            lifetime,
            max_lifetime: lifetime,
            hit_enemies: Vec::with_capacity(8),
            owner,
            knockback,
        }
    }
}

/// Marker for hitboxes that should damage the player (enemy attacks).
#[derive(Debug, Clone, Component)]
pub struct EnemyHitbox;
