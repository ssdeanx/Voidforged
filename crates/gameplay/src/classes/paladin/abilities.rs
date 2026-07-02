//! Paladin ability implementations.

use bevy::prelude::*;
use ir_core::*;


// ── Primary: Righteous Strike ───────────────────────────────────────

pub fn primary_righteous_strike(
    commands: &mut Commands, attacker: Entity, transform: &Transform,
    stats: &CombatStats, _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = (12.0 + stats.damage_bonus * 1.2).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 3.5, half_angle: 0.5 },
            dmg, attacker, DamageType::Physical, 0.15,
            ProjectileOwner::Player, 1.5,
        ).with_hit_reaction(0.1, 0.15, 0.05),
        Transform { translation: transform.translation, rotation: transform.rotation, ..default() },
    ));
}

// ── Secondary: Holy Light ──────────────────────────────────────────

#[derive(Component)]
pub struct HolyLight { pub heal_pct: f32 }

/// Heals the paladin by 30% of max HP.
pub fn secondary_holy_light(commands: &mut Commands, player: Entity) {
    commands.entity(player).insert(HolyLight { heal_pct: 0.3 });
}

/// Applies HolyLight heal to player.
pub fn apply_holy_light(
    mut commands: Commands, mut health_query: Query<&mut Health, With<Player>>,
    holy_light_query: Query<(Entity, &HolyLight)>,
) {
    for (entity, light) in holy_light_query.iter() {
        if let Ok(mut health) = health_query.get_mut(entity) {
            let max_hp = health.max;
            health.heal(max_hp * light.heal_pct);
        }
        commands.entity(entity).remove::<HolyLight>();
    }
}

// ── Cast: Consecration ─────────────────────────────────────────────

#[derive(Component)]
pub struct ConsecrationField {
    pub lifetime: f32, pub tick_timer: f32, pub damage: f32,
}

pub fn cast_consecration(commands: &mut Commands, transform: &Transform, _stats: &CombatStats) {
    commands.spawn((
        ConsecrationField { lifetime: 6.0, tick_timer: 0.0, damage: 8.0 },
        Transform { translation: transform.translation, rotation: transform.rotation, ..default() },
    ));
}

pub fn tick_consecration(
    mut commands: Commands, time: Res<Time>,
    mut fields: Query<(Entity, &mut ConsecrationField, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (field_entity, mut field, field_tf) in fields.iter_mut() {
        field.tick_timer -= time.delta_secs();
        if field.tick_timer > 0.0 { continue; }
        field.tick_timer = 1.0;
        field.lifetime -= time.delta_secs();
        for (enemy_entity, enemy_tf) in enemies.iter() {
            if field_tf.translation.distance(enemy_tf.translation) < 3.0 {
                damage_events.send(DamageEvent {
                    target: enemy_entity, source: field_entity,
                    amount: field.damage, is_critical: false,
                    damage_type: DamageType::Magic, hit_position: None,
                });
            }
        }
        if field.lifetime <= 0.0 { commands.entity(field_entity).despawn(); }
    }
}

// ── Spec-based Utilities ───────────────────────────────────────────

pub fn utility_blessing_of_might(commands: &mut Commands, entity: Entity, stats: &CombatStats) {
    info!("Paladin (Holy): Blessing of Might — +{} damage for 12s", stats.damage_bonus * 0.15);
    commands.entity(entity).insert(BuffComponent {
        id: "blessing_of_might".into(), remaining: 12.0, stat_boost: stats.damage_bonus * 0.15,
    });
}

pub fn utility_hammer_of_justice(
    commands: &mut Commands, entity: Entity, transform: &Transform,
    stats: &CombatStats, enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Paladin (Retribution): Hammer of Justice — stun");
    let Some((nearest, _)) = crate::classes::auto_attack::nearest_enemy(transform.translation, 4.0, enemies) else { return };
    commands.entity(entity).insert(BuffComponent {
        id: "hammer_of_justice".into(), remaining: 4.0, stat_boost: 0.0,
    });
    // Stun the target via a damage event
    _damage_events.send(DamageEvent {
        target: nearest, source: entity, amount: stats.damage_bonus * 2.0,
        is_critical: false, damage_type: DamageType::Physical, hit_position: None,
    });
}

// ── Spec-based Ultimates ───────────────────────────────────────────

pub fn ultimate_divine_intervention(
    commands: &mut Commands, entity: Entity, _transform: &Transform,
    stats: &CombatStats, _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Paladin (Holy): Divine Intervention — massive heal");
    commands.entity(entity).insert(HolyLight { heal_pct: 0.6 });
}

pub fn ultimate_avenging_wrath(
    commands: &mut Commands, entity: Entity, transform: &Transform,
    stats: &CombatStats, _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Paladin (Retribution): Avenging Wrath — +30% damage for 15s");
    commands.entity(entity).insert(BuffComponent {
        id: "avenging_wrath".into(), remaining: 15.0, stat_boost: stats.damage_bonus * 0.3,
    });
}
