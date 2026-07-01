//! Hitbox system — spatial damage zones with configurable shapes.
//! Components live in ir_core, processing systems in ir_gameplay.

use crate::components::ProjectileOwner;
use crate::DamageType;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Hitbox Shapes
// ============================================================================

/// The shape of a damage hitbox used for area-of-effect and melee attacks.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub enum HitboxShape {
    /// Cone projecting forward from the origin (melee cleaves, breath attacks).
    Cone {
        /// Maximum distance from origin.
        range: f32,
        /// Half-angle of the cone (radians) — full spread = 2× half_angle.
        half_angle: f32,
    },
    /// Circular area around the origin (explosions, shockwaves).
    Circle {
        /// Radius of the circle in world units.
        radius: f32,
    },
    /// Rectangular area in front of the origin (charge attacks, line AoEs).
    Rect {
        /// Width of the rectangle.
        width: f32,
        /// Length of the rectangle (depth).
        length: f32,
    },
    /// Single-target point check within range (stab, projectile impact).
    Point {
        /// Maximum distance from origin to check.
        range: f32,
    },
}

// ============================================================================
// Damage Hitbox Component
// ============================================================================

/// A temporary entity that deals damage to entities it overlaps.
///
/// Spawned by attacks and abilities, despawned after `lifetime` seconds.
/// The actual overlap detection and damage application happens in
/// `ir_gameplay` systems. Contains the damage payload, hit-reaction
/// parameters (stun, flash, stop), and an anti-double-tap hit list.
#[derive(Debug, Clone, Component)]
pub struct DamageHitbox {
    /// Spatial shape used for overlap detection.
    pub shape: HitboxShape,
    /// Base damage dealt on hit.
    pub damage: f32,
    /// The entity that created this hitbox (for source-of-damage tracking).
    pub source: Entity,
    /// Physical, Magic, or True — affects resistance calculation.
    pub damage_type: DamageType,
    /// Remaining lifetime before despawn (seconds).
    pub lifetime: f32,
    /// Maximum lifetime (set on creation, used for progress VFX).
    pub max_lifetime: f32,
    /// Entities already hit by this hitbox (prevents multi-hit on the same target).
    pub hit_enemies: Vec<Entity>,
    /// Whether the hitbox harms players or enemies.
    pub owner: ProjectileOwner,
    /// Knockback magnitude applied to hit targets (direction computed from hitbox to target).
    pub knockback: f32,
    /// Duration of hit-stun (stagger) applied to the target on hit.
    pub hit_stun_duration: f32,
    /// Duration of visual hit-flash on the target.
    pub hit_flash_duration: f32,
    /// Duration of hit-stop (local time freeze on the hit entity).
    pub hit_stop_duration: f32,
}

impl DamageHitbox {
    /// Creates a new damage hitbox with default hit-reaction values.
    ///
    /// Default reactions: 0.1s stun, 0.15s flash, 0.05s stop.
    /// Adjust with [`with_hit_reaction`](Self::with_hit_reaction).
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

    /// Convenience builder to override hit-reaction timings.
    ///
    /// - `stun`: stagger duration (seconds).
    /// - `flash`: visual impact flash duration.
    /// - `stop`: hit-stop / local time freeze duration.
    pub fn with_hit_reaction(mut self, stun: f32, flash: f32, stop: f32) -> Self {
        self.hit_stun_duration = stun;
        self.hit_flash_duration = flash;
        self.hit_stop_duration = stop;
        self
    }
}

/// Marker component for hitboxes that deal damage to the player (enemy attacks).
#[derive(Debug, Clone, Component)]
pub struct EnemyHitbox;
