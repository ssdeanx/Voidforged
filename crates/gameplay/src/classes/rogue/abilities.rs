//! Rogue ability implementations.

use bevy::prelude::*;
use ir_core::*;


// ── Primary: Backstab ──────────────────────────────────────────────

pub fn primary_backstab(
    commands: &mut Commands, attacker: Entity, transform: &Transform,
    stats: &CombatStats, _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = (8.0 + stats.damage_bonus).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 2.5, half_angle: 0.3 },
            dmg, attacker, DamageType::Physical, 0.12,
            ProjectileOwner::Player, 0.5,
        ),
        Transform { translation: transform.translation, rotation: transform.rotation, ..default() },
    ));
}

// ── Secondary: Poison Blade ────────────────────────────────────────

#[derive(Component)]
pub struct PoisonDoT { pub damage: f32, pub remaining: f32, pub tick_timer: f32, pub stack: u32 }

pub fn secondary_poison_blade(
    commands: &mut Commands, _attacker: Entity, transform: &Transform,
    stats: &CombatStats, enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = 4.0 + stats.damage_bonus * 0.3;
    let mut nearest: Option<(Entity, f32)> = None;
    for (enemy_entity, enemy_tf) in enemies.iter() {
        let dist = transform.translation.distance(enemy_tf.translation);
        if dist < 2.5 { match nearest { Some((_, d)) if dist < d => nearest = Some((enemy_entity, dist)), None => nearest = Some((enemy_entity, dist)), _ => {} } }
    }
    if let Some((target, _)) = nearest {
        commands.entity(target).insert(PoisonDoT { damage: dmg, remaining: 4.0, tick_timer: 1.0, stack: 1 });
    }
}

pub fn tick_poison(time: Res<Time>, mut enemies: Query<&mut PoisonDoT>) {
    for mut poison in enemies.iter_mut() {
        poison.remaining -= time.delta_secs();
        poison.tick_timer -= time.delta_secs();
    }
}

pub fn apply_poison_damage(
    mut commands: Commands, _time: Res<Time>,
    mut poisoned: Query<(Entity, &mut PoisonDoT)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (entity, mut poison) in poisoned.iter_mut() {
        if poison.tick_timer <= 0.0 {
            poison.tick_timer = 1.0;
            damage_events.send(DamageEvent {
                target: entity, source: entity, amount: poison.damage,
                is_critical: false, damage_type: DamageType::True, hit_position: None,
            });
        }
        if poison.remaining <= 0.0 { commands.entity(entity).remove::<PoisonDoT>(); }
    }
}

// ── Cast: Vanish ──────────────────────────────────────────────────

#[derive(Component)]
pub struct VanishBuff { pub remaining: f32, pub guaranteed_crit: bool }

pub fn cast_vanish(commands: &mut Commands, player: Entity) {
    commands.entity(player).insert(VanishBuff { remaining: 4.0, guaranteed_crit: true });
}

// ── Spec-based Utilities ───────────────────────────────────────────

pub fn utility_deadly_poison(
    commands: &mut Commands, entity: Entity, stats: &CombatStats,
    _damage_events: &mut EventWriter<DamageEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Rogue (Assassination): Deadly Poison — +{} poison damage", stats.damage_bonus * 0.3);
    commands.entity(entity).insert(BuffComponent {
        id: "deadly_poison".into(), remaining: 300.0, stat_boost: stats.damage_bonus * 0.3,
    });
}

pub fn utility_smoke_bomb(commands: &mut Commands, transform: &Transform) {
    info!("Rogue (Outlaw): Smoke Bomb — blind area");
    commands.spawn((
        Lifetime { remaining: 3.0 },
        Transform::from_translation(transform.translation),
    ));
}

// ── Spec-based Ultimates ───────────────────────────────────────────

pub fn ultimate_rupture(
    commands: &mut Commands, entity: Entity, transform: &Transform,
    stats: &CombatStats, enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Rogue (Assassination): Rupture — massive bleed");
    let Some((nearest, _)) = crate::classes::auto_attack::nearest_enemy(transform.translation, 4.0, enemies) else { return };
    commands.entity(nearest).insert(PoisonDoT {
        damage: 12.0 + stats.damage_bonus, remaining: 8.0, tick_timer: 1.0, stack: 5,
    });
}

pub fn ultimate_blade_flurry(
    commands: &mut Commands, entity: Entity, transform: &Transform,
    stats: &CombatStats, _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    info!("Rogue (Outlaw): Blade Flurry — AoE burst");
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Circle { radius: 4.0 },
            20.0 + stats.damage_bonus * 2.0, entity, DamageType::Physical, 0.3,
            ProjectileOwner::Player, 3.0,
        ),
        Transform { translation: transform.translation, ..default() },
    ));
}
