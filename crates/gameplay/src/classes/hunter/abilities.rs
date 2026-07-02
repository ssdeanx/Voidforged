//! Hunter ability implementations.

use bevy::prelude::*;
use ir_core::*;

// ── Primary: Aimed Shot ────────────────────────────────────────────

pub fn primary_aimed_shot(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let direction = (cursor.0 - transform.translation).normalize_or_zero();
    if direction.length_squared() < 0.1 { return; }
    let dmg = 15.0 + stats.damage_bonus;
    commands.spawn(ProjectileBundle::new(
        dmg, 18.0, 3.0, direction,
        transform.translation + Vec3::Y * 0.5, ProjectileOwner::Player,
    ));
}

// ── Secondary: Multi-Shot ─────────────────────────────────────────

pub fn secondary_multi_shot(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
) {
    let base_dir = (cursor.0 - transform.translation).normalize_or_zero();
    if base_dir.length_squared() < 0.1 { return; }
    let dmg = 8.0 + stats.damage_bonus * 0.5;
    let origin = transform.translation + Vec3::Y * 0.5;
    for spread in [-0.15, 0.0, 0.15] {
        let rotated = Quat::from_rotation_y(spread) * base_dir;
        commands.spawn(ProjectileBundle::new(dmg, 14.0, 2.0, rotated, origin, ProjectileOwner::Player));
    }
}

// ── Cast: Trap ────────────────────────────────────────────────────

#[derive(Component)]
pub struct SnareTrap { pub lifetime: f32, pub slow_amount: f32 }

pub fn cast_trap(commands: &mut Commands, transform: &Transform) {
    commands.spawn((
        SnareTrap { lifetime: 8.0, slow_amount: 0.5 },
        Transform::from_translation(transform.translation), Lifetime { remaining: 8.0 },
    ));
}

pub fn tick_trap_slow(
    traps: Query<(&SnareTrap, &Transform)>,
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
) {
    for (_trap, trap_tf) in traps.iter() {
        for (mut velocity, enemy_tf) in enemies.iter_mut() {
            if trap_tf.translation.distance(enemy_tf.translation) < 2.0 {
                velocity.0 *= 0.5;
            }
        }
    }
}

// ── Spec-based Utilities ───────────────────────────────────────────

pub fn utility_hunters_mark(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let Some((nearest, _)) = crate::classes::auto_attack::nearest_enemy(transform.translation, 12.0, enemies) else { return };
    info!("Hunter (Marksmanship): Hunter's Mark on target — +{} damage", stats.damage_bonus * 0.25);
    commands.entity(nearest).insert(BuffComponent {
        id: "hunters_mark".into(), remaining: 20.0, stat_boost: stats.damage_bonus * 0.25,
    });
}

pub fn utility_call_pet(commands: &mut Commands, transform: &Transform) {
    info!("Hunter (Survival): Call Pet — wolf summoned");
    commands.spawn((
        Transform::from_translation(transform.translation),
        Lifetime { remaining: 30.0 },
    ));
}

// ── Spec-based Ultimates ───────────────────────────────────────────

pub fn ultimate_rapid_fire(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dir = (cursor.0 - transform.translation).normalize_or_zero();
    if dir.length_squared() < 0.1 { return; }
    info!("Hunter (Marksmanship): Rapid Fire — 5 arrows");
    let origin = transform.translation + Vec3::Y * 0.5;
    for i in 0..5 {
        let spread = (i as f32 - 2.0) * 0.08;
        let rotated = Quat::from_rotation_y(spread) * dir;
        commands.spawn(ProjectileBundle::new(
            8.0 + stats.damage_bonus * 0.5, 20.0, 1.5, rotated, origin, ProjectileOwner::Player,
        ));
    }
}

pub fn ultimate_explosive_trap(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats,
    _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Hunter (Survival): Explosive Trap — AoE fire damage");
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Circle { radius: 4.0 },
            30.0 + stats.damage_bonus * 1.5, Entity::PLACEHOLDER,
            DamageType::Magic, 0.5, ProjectileOwner::Player, 4.0,
        ),
        Transform::from_translation(transform.translation), Lifetime { remaining: 0.5 },
    ));
}
