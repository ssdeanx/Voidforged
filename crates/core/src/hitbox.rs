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
    /// Knockback magnitude (direction computed from hitbox to target).
    pub knockback: f32,
    /// Duration of hit-stun (stagger) applied to the target on hit.
    pub hit_stun_duration: f32,
    /// Duration of visual hit-flash on the target.
    pub hit_flash_duration: f32,
    /// Duration of hit-stop (local time freeze on the hit entity).
    pub hit_stop_duration: f32,
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
            hit_stun_duration: 0.1,
            hit_flash_duration: 0.15,
            hit_stop_duration: 0.05,
        }
    }

    /// Convenience builder for hit-stop configuration.
    pub fn with_hit_reaction(mut self, stun: f32, flash: f32, stop: f32) -> Self {
        self.hit_stun_duration = stun;
        self.hit_flash_duration = flash;
        self.hit_stop_duration = stop;
        self
    }
}

/// Marker for hitboxes that should damage the player (enemy attacks).
#[derive(Debug, Clone, Component)]
pub struct EnemyHitbox;
