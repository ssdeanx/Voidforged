//! Mage ability implementations.

use bevy::prelude::*;
use ir_core::*;

// ── Primary: Fireball ──────────────────────────────────────────────

pub fn primary_fireball(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let direction = (cursor.0 - transform.translation).normalize_or_zero();
    if direction.length_squared() < 0.1 { return; }
    let dmg = 18.0 + stats.damage_bonus;
    commands.spawn(ProjectileBundle::new(
        dmg, 14.0, 2.5, direction,
        transform.translation + Vec3::Y * 0.5, ProjectileOwner::Player,
    ));
}

// ── Secondary: Frostbolt ───────────────────────────────────────────

pub fn secondary_frostbolt(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
) {
    let direction = (cursor.0 - transform.translation).normalize_or_zero();
    if direction.length_squared() < 0.1 { return; }
    let dmg = 10.0 + stats.damage_bonus * 0.5;
    commands.spawn(ProjectileBundle::new(
        dmg, 12.0, 3.0, direction,
        transform.translation + Vec3::Y * 0.5, ProjectileOwner::Player,
    ));
}

// ── Cast: Arcane Blast ─────────────────────────────────────────────

pub fn cast_arcane_blast(
    _commands: &mut Commands, player: Entity, transform: &Transform,
    stats: &CombatStats, _cursor: &CursorWorldPos,
    enemies: &Query<(Entity, &Transform), With<Enemy>>,
    damage_events: &mut EventWriter<DamageEvent>,
    dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = 25.0 + stats.damage_bonus * 2.0;
    let mut nearest: Option<(Entity, f32)> = None;
    for (enemy_entity, enemy_tf) in enemies.iter() {
        let dist = transform.translation.distance(enemy_tf.translation);
        if dist < 6.0 { match nearest { Some((_, d)) if dist < d => nearest = Some((enemy_entity, dist)), None => nearest = Some((enemy_entity, dist)), _ => {} } }
    }
    if let Some((target, _)) = nearest {
        damage_events.send(DamageEvent { target, source: player, amount: dmg, is_critical: false, damage_type: DamageType::Magic, hit_position: None });
        dmg_num_events.send(DamageNumberEvent { position: Vec3::Y, amount: dmg as i32, is_crit: false, damage_type: DamageType::Magic });
    }
}

// ── Spec-based Utilities ───────────────────────────────────────────

pub fn utility_blizzard(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, _cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Mage (Frost): Blizzard — AoE ice storm");
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Circle { radius: 5.0 },
            5.0 + stats.damage_bonus * 0.3, Entity::PLACEHOLDER,
            DamageType::Magic, 4.0, ProjectileOwner::Player, 0.5,
        ),
        Transform::from_translation(transform.translation), Lifetime { remaining: 4.0 },
    ));
}

pub fn utility_combustion(commands: &mut Commands, entity: Entity, stats: &CombatStats) {
    info!("Mage (Fire): Combustion — +{} spell power for 15s", stats.damage_bonus * 0.2);
    commands.entity(entity).insert(BuffComponent {
        id: "combustion".into(), remaining: 15.0, stat_boost: stats.damage_bonus * 0.2,
    });
}

// ── Spec-based Ultimates ───────────────────────────────────────────

pub fn ultimate_water_elemental(commands: &mut Commands, transform: &Transform) {
    info!("Mage (Frost): Water Elemental — frost pet summoned");
    commands.spawn((
        Transform::from_translation(transform.translation),
        Lifetime { remaining: 30.0 },
    ));
}

pub fn ultimate_meteor(
    commands: &mut Commands, transform: &Transform,
    stats: &CombatStats, cursor: &CursorWorldPos,
    _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Mage (Fire): Meteor — massive AoE at cursor");
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Circle { radius: 6.0 },
            50.0 + stats.damage_bonus * 3.0, Entity::PLACEHOLDER,
            DamageType::Magic, 0.8, ProjectileOwner::Player, 6.0,
        ),
        Transform::from_translation(cursor.0), Lifetime { remaining: 0.8 },
    ));
}
