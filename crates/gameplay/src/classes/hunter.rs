//! Hunter class — aimed shot, multi-shot, trap, disengage.
//! Resource: Focus (medium regen, spent on abilities).

use bevy::prelude::*;
use ir_core::*;
use crate::classes::abilities::ClassResource;

/// Resource config
pub fn resource_config() -> ClassResource {
    ClassResource::new(100.0, 8.0) // 100 Focus, regen 8/sec
}

// ── Primary: Aimed Shot ───────────────────────────────────────────────────

/// Fires a high-damage projectile toward the cursor.
pub fn primary_aimed_shot(
    commands: &mut Commands,
    transform: &Transform,
    stats: &CombatStats,
    cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let direction = (cursor.0 - transform.translation).normalize_or_zero();
    if direction.length_squared() < 0.1 {
        return;
    }
    let dmg = 15.0 + stats.damage_bonus;
    commands.spawn(ProjectileBundle::new(
        dmg, 18.0, 3.0, direction,
        transform.translation + Vec3::Y * 0.5,
        ProjectileOwner::Player,
    ));
}

// ── Secondary: Multi-Shot ─────────────────────────────────────────────────

/// Fires a spread of 3 arrows.
pub fn secondary_multi_shot(
    commands: &mut Commands,
    transform: &Transform,
    stats: &CombatStats,
    cursor: &CursorWorldPos,
) {
    let base_dir = (cursor.0 - transform.translation).normalize_or_zero();
    if base_dir.length_squared() < 0.1 {
        return;
    }
    let dmg = 8.0 + stats.damage_bonus * 0.5;
    let origin = transform.translation + Vec3::Y * 0.5;
    for spread in [-0.15, 0.0, 0.15] {
        let rotated = Quat::from_rotation_y(spread) * base_dir;
        commands.spawn(ProjectileBundle::new(
            dmg, 14.0, 2.0, rotated, origin, ProjectileOwner::Player,
        ));
    }
}

// ── Cast: Trap ────────────────────────────────────────────────────────────

/// Places a snare at the hunter's feet that slows enemies.
#[derive(Component)]
pub struct SnareTrap {
    pub lifetime: f32,
    pub slow_amount: f32, // 0.0-1.0 movement speed reduction
}

pub fn cast_trap(
    commands: &mut Commands,
    transform: &Transform,
) {
    commands.spawn((
        SnareTrap {
            lifetime: 8.0,
            slow_amount: 0.5,
        },
        Transform::from_translation(transform.translation),
        Lifetime { remaining: 8.0 },
    ));
    info!("Hunter trap placed");
}

/// Applies slow to enemies near traps.
pub fn tick_trap_slow(
    traps: Query<(&SnareTrap, &Transform)>,
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
) {
    for (_trap, trap_tf) in traps.iter() {
        for (mut velocity, enemy_tf) in enemies.iter_mut() {
            let dist = trap_tf.translation.distance(enemy_tf.translation);
            if dist < 2.0 {
                velocity.0 *= 0.5; // 50% slow while in trap
            }
        }
    }
}

// ── Dash: Disengage ───────────────────────────────────────────────────────

/// Leaps backward away from the cursor direction.
pub fn dash_disengage(
    _commands: &mut Commands,
    transform: &Transform,
    cursor: &CursorWorldPos,
) {
    let away = (transform.translation - cursor.0).normalize_or_zero();
    if away.length_squared() > 0.1 {
        // Velocity set backward — handled by dash system
        info!("Hunter disengage in direction {:?}", away);
    }
}
