//! Mage class — fireball, frostbolt, arcane blast, blink.
//! Resource: Mana (large pool, slow regen).

use bevy::prelude::*;
use ir_core::*;
use crate::classes::abilities::ClassResource;

/// Resource config
pub fn resource_config() -> ClassResource {
    ClassResource::new(200.0, 4.0) // 200 Mana, regen 4/sec
}

// ── Primary: Fireball ─────────────────────────────────────────────────────

/// Fires a projectile that explodes on impact (small AoE).
pub fn primary_fireball(
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
    let dmg = 18.0 + stats.damage_bonus;
    commands.spawn(ProjectileBundle::new(
        dmg, 14.0, 2.5, direction,
        transform.translation + Vec3::Y * 0.5,
        ProjectileOwner::Player,
    ));
}

// ── Secondary: Frostbolt ──────────────────────────────────────────────────

/// Slowing projectile (applies slow effect on hit).
pub fn secondary_frostbolt(
    commands: &mut Commands,
    transform: &Transform,
    stats: &CombatStats,
    cursor: &CursorWorldPos,
) {
    let direction = (cursor.0 - transform.translation).normalize_or_zero();
    if direction.length_squared() < 0.1 {
        return;
    }
    let dmg = 10.0 + stats.damage_bonus * 0.5;
    commands.spawn(ProjectileBundle::new(
        dmg, 12.0, 3.0, direction,
        transform.translation + Vec3::Y * 0.5,
        ProjectileOwner::Player,
    ));
}

// ── Cast: Arcane Blast ────────────────────────────────────────────────────

/// High single-target damage, short range. Hits nearest enemy.
pub fn cast_arcane_blast(
    _commands: &mut Commands,
    transform: &Transform,
    stats: &CombatStats,
    _cursor: &CursorWorldPos,
    enemies: &Query<(Entity, &Transform), With<Enemy>>,
    damage_events: &mut EventWriter<DamageEvent>,
    dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = 25.0 + stats.damage_bonus * 2.0;
    let range = 6.0;
    let mut nearest: Option<(Entity, f32)> = None;

    for (enemy_entity, enemy_tf) in enemies.iter() {
        let dist = transform.translation.distance(enemy_tf.translation);
        if dist < range {
            match nearest {
                Some((_, d)) if dist < d => nearest = Some((enemy_entity, dist)),
                None => nearest = Some((enemy_entity, dist)),
                _ => {}
            }
        }
    }

    if let Some((target, _)) = nearest {
        damage_events.send(DamageEvent {
            target,
            source: target, // placeholder — owner unknown in this context
            amount: dmg,
            is_critical: false,
            damage_type: DamageType::Magic,
        });
        dmg_num_events.send(DamageNumberEvent {
            position: Vec3::Y,
            amount: dmg as i32,
            is_crit: false,
        });
        info!("Mage arcane blast hits for {}", dmg);
    }
}

// ── Dash: Blink ───────────────────────────────────────────────────────────

/// Short teleport in the movement direction or toward cursor.
pub fn dash_blink(_commands: &mut Commands, transform: &Transform, cursor: &CursorWorldPos) {
    let blink_dir = (cursor.0 - transform.translation).normalize_or_zero();
    _ = &blink_dir; // placeholder
    if blink_dir.length_squared() > 0.1 {
        let blink_dist = 6.0;
        _ = blink_dist;
    }
}
