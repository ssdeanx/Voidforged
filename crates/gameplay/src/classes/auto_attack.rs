//! Auto-attack system — weapon-based basic attacks that trigger automatically
//! when the player is in combat and within range of an enemy.
//!
//! Each class has its own auto-attack implementation (melee swing, ranged shot, magic bolt)
//! that fires automatically based on weapon attack speed.
//! Cooldown is stored on the player via `AutoAttackCooldown`.

use bevy::prelude::*;
use ir_core::*;

/// Per-player cooldown for auto-attacks. Runs on weapon speed.
#[derive(Component, Debug, Clone)]
pub struct AutoAttackCooldown {
    /// Time remaining before the next auto-attack can fire.
    pub timer: f32,
    /// Base cooldown derived from weapon speed (seconds between attacks).
    pub base_cd: f32,
}

impl Default for AutoAttackCooldown {
    fn default() -> Self {
        Self {
            timer: 0.0,
            base_cd: 1.0, // 1 attack per second default
        }
    }
}

impl AutoAttackCooldown {
    pub fn new(weapon_speed: f32) -> Self {
        let base_cd = if weapon_speed > 0.0 { 1.0 / weapon_speed } else { 1.0 };
        Self { timer: 0.0, base_cd }
    }

    pub fn tick(&mut self, dt: f32) {
        self.timer = (self.timer - dt).max(0.0);
    }

    pub fn ready(&self) -> bool {
        self.timer <= 0.0
    }

    pub fn reset(&mut self) {
        self.timer = self.base_cd;
    }
}

/// Finds the nearest enemy to a position within `range`.
/// Returns `None` if no enemy is within range.
pub fn nearest_enemy(
    position: Vec3,
    range: f32,
    enemies: &Query<(Entity, &Transform), With<Enemy>>,
) -> Option<(Entity, Vec3)> {
    let mut best: Option<(Entity, f32, Vec3)> = None;
    for (entity, tf) in enemies.iter() {
        let dist = position.distance(tf.translation);
        if dist <= range {
            match best {
                Some((_, d, _)) if dist < d => best = Some((entity, dist, tf.translation)),
                None => best = Some((entity, dist, tf.translation)),
                _ => {}
            }
        }
    }
    best.map(|(e, _, p)| (e, p))
}

/// Direction from `from` toward `target` on the XZ plane.
pub fn direction_to(from: Vec3, target: Vec3) -> Vec3 {
    let delta = target - from;
    let flat = Vec3::new(delta.x, 0.0, delta.z);
    flat.normalize_or_zero()
}
