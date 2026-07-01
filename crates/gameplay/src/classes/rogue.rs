//! Rogue class — backstab, poison blade, vanish, shadowstep.
//! Resource: Energy (fast regen, capped pool).

use bevy::prelude::*;
use ir_core::*;
use crate::classes::abilities::ClassResource;

/// Resource config
pub fn resource_config() -> ClassResource {
    ClassResource::new(100.0, 20.0) // 100 Energy, regen 20/sec
}

/// Marker for vanish buff — next attack auto-crits.
#[derive(Component)]
pub struct VanishBuff {
    pub remaining: f32,
    pub guaranteed_crit: bool,
}

// ── Primary: Backstab ────────────────────────────────────────────────────

/// Narrow cone melee attack from behind the target (or from any direction).
pub fn primary_backstab(
    commands: &mut Commands,
    attacker: Entity,
    transform: &Transform,
    stats: &CombatStats,
    _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = (8.0 + stats.damage_bonus).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 2.5, half_angle: 0.3 },
            dmg,
            attacker,
            DamageType::Physical,
            0.12,
            ProjectileOwner::Player,
            0.5,
        ),
        Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            ..default()
        },
    ));
}

// ── Secondary: Poison Blade ─────────────────────────────────────────────

/// Applies a stacking poison DoT to the target.
#[derive(Component)]
pub struct PoisonDoT {
    pub damage: f32,
    pub remaining: f32,
    pub tick_timer: f32,
    pub stack: u32,
}

pub fn secondary_poison_blade(
    commands: &mut Commands,
    _attacker: Entity,
    transform: &Transform,
    stats: &CombatStats,
    enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = 4.0 + stats.damage_bonus * 0.3;
    let range = 2.5;
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
        commands.entity(target).insert(PoisonDoT {
            damage: dmg,
            remaining: 4.0,
            tick_timer: 1.0,
            stack: 1,
        });
    }
}

/// Ticks poison DoT on enemies.
pub fn tick_poison(
    time: Res<Time>,
    mut enemies: Query<&mut PoisonDoT>,
) {
    for mut poison in enemies.iter_mut() {
        poison.remaining -= time.delta_secs();
        poison.tick_timer -= time.delta_secs();
    }
}

/// Applies poison tick damage and removes expired poisons.
pub fn apply_poison_damage(
    mut commands: Commands,
    time: Res<Time>,
    mut poisoned: Query<(Entity, &mut PoisonDoT)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (entity, mut poison) in poisoned.iter_mut() {
        if poison.tick_timer <= 0.0 {
            poison.tick_timer = 1.0;
            damage_events.send(DamageEvent {
                target: entity,
                source: entity,
                amount: poison.damage,
                is_critical: false,
                damage_type: DamageType::True,
            });
        }
        if poison.remaining <= 0.0 {
            commands.entity(entity).remove::<PoisonDoT>();
        }
    }
}

// ── Cast: Vanish ──────────────────────────────────────────────────────────

/// Vanish — becomes invisible, next attack auto-crits.
pub fn cast_vanish(
    commands: &mut Commands,
    player: Entity,
) {
    commands.entity(player).insert(VanishBuff {
        remaining: 4.0,
        guaranteed_crit: true,
    });
    info!("Rogue vanished! Next attack guaranteed crit.");
}

// ── Dash: Shadowstep ──────────────────────────────────────────────────────

/// Teleports behind the nearest enemy within range.
pub fn dash_shadowstep(_commands: &mut Commands, _cursor: &CursorWorldPos) {
    // TODO: Find nearest enemy, teleport behind
    info!("Rogue shadowstep");
}
