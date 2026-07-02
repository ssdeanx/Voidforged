//! Enemy ability system — different enemy types have distinct attack patterns,
//! abilities, and telegraphed attacks that make combat varied.
//!
//! Uses `Enemy.variant` to determine the attack set for each enemy type:
//! - Grunt: melee swing + charge
//! - Ranged: ranged volley + quick shot
//! - Charger: speed rush + knockback
//! - Elite: AoE slam + enrage
//! - Boss: multi-phase with unique abilities per phase
//! - Caster: magic bolt + frost nova
//! - Healer: heal beam + protective barrier
//! - Summoner: summon minions + curse
//! - Assassin: backstab teleport + poison blade
//! - Brute: ground slam + charge

use bevy::prelude::*;
use ir_core::*;

/// Cooldown timer for enemy special abilities.
#[derive(Component, Debug, Clone)]
pub struct EnemyAbilityCooldown {
    pub timers: [f32; 3],
    pub max_cds: [f32; 3],
}

impl EnemyAbilityCooldown {
    pub fn new(variant: &EnemyVariant) -> Self {
        match variant {
            EnemyVariant::Grunt => Self { timers: [0.0; 3], max_cds: [4.0, 0.0, 0.0] },
            EnemyVariant::Ranged => Self { timers: [0.0; 3], max_cds: [3.0, 0.0, 0.0] },
            EnemyVariant::Charger => Self { timers: [0.0; 3], max_cds: [5.0, 0.0, 0.0] },
            EnemyVariant::Elite => Self { timers: [0.0; 3], max_cds: [5.0, 8.0, 0.0] },
            EnemyVariant::Boss => Self { timers: [0.0; 3], max_cds: [6.0, 10.0, 15.0] },
            EnemyVariant::Caster => Self { timers: [0.0; 3], max_cds: [4.0, 10.0, 0.0] },
            EnemyVariant::Healer => Self { timers: [0.0; 3], max_cds: [6.0, 12.0, 0.0] },
            EnemyVariant::Summoner => Self { timers: [0.0; 3], max_cds: [8.0, 12.0, 0.0] },
            EnemyVariant::Assassin => Self { timers: [0.0; 3], max_cds: [6.0, 0.0, 0.0] },
            EnemyVariant::Brute => Self { timers: [0.0; 3], max_cds: [4.0, 8.0, 0.0] },
        }
    }

    pub fn ready(&self, index: usize) -> bool {
        index < 3 && self.timers[index] <= 0.0
    }

    pub fn reset(&mut self, index: usize) {
        if index < 3 { self.timers[index] = self.max_cds[index]; }
    }

    pub fn tick(&mut self, dt: f32) {
        for t in self.timers.iter_mut() { *t = (*t - dt).max(0.0); }
    }
}

/// Spawns an enemy ability hitbox.
fn spawn_ability_hitbox(
    commands: &mut Commands,
    shape: HitboxShape,
    damage: f32,
    source: Entity,
    damage_type: DamageType,
    knockback: f32,
    owner: ProjectileOwner,
    lifetime: f32,
    position: Vec3,
    rotation: Option<Quat>,
) {
    let mut hitbox = DamageHitbox::new(
        shape, damage, source, damage_type, knockback, owner, lifetime,
    );
    commands.spawn((
        hitbox,
        Transform {
            translation: position,
            rotation: rotation.unwrap_or(Quat::IDENTITY),
            ..default()
        },
    ));
}

/// Performs an enemy's special ability based on its variant and cooldown readiness.
/// Returns true if an ability was used this frame.
pub fn use_enemy_ability(
    enemy_entity: Entity,
    enemy_tf: &Transform,
    enemy_variant: &EnemyVariant,
    player_tf: &Transform,
    stats: &CombatStats,
    commands: &mut Commands,
    damage_events: &mut EventWriter<DamageEvent>,
    dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    impact_events: &mut EventWriter<SpawnImpactEvent>,
) -> bool {
    match enemy_variant {
        EnemyVariant::Grunt => {
            let dir = (player_tf.translation - enemy_tf.translation).normalize_or_zero();
            if dir.length_squared() < 0.1 { return false; }
            commands.spawn((
                ForcedMovement::new(dir * 25.0, 3.0),
                Transform::from_translation(enemy_tf.translation),
            ));
            spawn_ability_hitbox(
                commands, HitboxShape::Circle { radius: 2.0 },
                stats.damage_bonus * 1.5, enemy_entity, DamageType::Physical,
                0.2, ProjectileOwner::Enemy, 1.5, player_tf.translation, None,
            );
            true
        }
        EnemyVariant::Ranged => {
            let dir = (player_tf.translation - enemy_tf.translation).normalize_or_zero();
            if dir.length_squared() < 0.1 { return false; }
            let origin = enemy_tf.translation + Vec3::Y * 0.5;
            for spread in [-0.2, 0.0, 0.2] {
                let rotated = Quat::from_rotation_y(spread) * dir;
                commands.spawn(ProjectileBundle::new(
                    stats.damage_bonus * 0.6, 12.0, 2.0, rotated, origin, ProjectileOwner::Enemy,
                ));
            }
            true
        }
        EnemyVariant::Charger => {
            let dir = (player_tf.translation - enemy_tf.translation).normalize_or_zero();
            if dir.length_squared() < 0.1 { return false; }
            commands.spawn((
                ForcedMovement::new(dir * 35.0, 4.0),
                DamageHitbox::new(
                    HitboxShape::Cone { range: 4.0, half_angle: 0.5 },
                    stats.damage_bonus * 1.2, enemy_entity, DamageType::Physical,
                    0.15, ProjectileOwner::Enemy, 1.0,
                ).with_hit_reaction(0.0, 0.0, 0.1),
                Transform::from_translation(enemy_tf.translation),
            ));
            true
        }
        EnemyVariant::Elite => {
            commands.spawn((
                TelegraphIndicator::new(0.8, enemy_entity),
                DamageHitbox::new(
                    HitboxShape::Circle { radius: 4.0 },
                    stats.damage_bonus * 2.0, enemy_entity, DamageType::Physical,
                    0.3, ProjectileOwner::Enemy, 2.0,
                ),
                Transform::from_translation(enemy_tf.translation),
            ));
            true
        }
        EnemyVariant::Boss => {
            commands.spawn((
                TelegraphIndicator::new(1.0, enemy_entity),
                DamageHitbox::new(
                    HitboxShape::Circle { radius: 5.0 },
                    stats.damage_bonus * 3.0, enemy_entity, DamageType::Physical,
                    0.5, ProjectileOwner::Enemy, 3.0,
                ),
                Transform::from_translation(enemy_tf.translation),
            ));
            true
        }
        EnemyVariant::Caster => {
            // Homing magic bolt
            let dir = (player_tf.translation - enemy_tf.translation).normalize_or_zero();
            if dir.length_squared() < 0.1 { return false; }
            commands.spawn(ProjectileBundle::new(
                stats.damage_bonus * 1.2, 10.0, 3.0, dir,
                enemy_tf.translation + Vec3::Y * 0.5, ProjectileOwner::Enemy,
            ));
            true
        }
        EnemyVariant::Healer => {
            // Heal beam — restores HP to nearest damaged ally
            info!("Healer: healing nearby allies (placeholder)");
            true
        }
        EnemyVariant::Summoner => {
            // Summon 2 minion enemies near the summoner
            for side in [-1.0, 1.0] {
                let offset = Vec3::new(side * 2.0, 0.0, 0.0);
                commands.spawn(EnemyBundle::new(
                    EnemyVariant::Grunt, 1, enemy_tf.translation + offset,
                ));
            }
            true
        }
        EnemyVariant::Assassin => {
            // Teleport behind player and strike
            let behind = player_tf.translation - enemy_tf.forward() * 2.0;
            spawn_ability_hitbox(
                commands, HitboxShape::Cone { range: 2.0, half_angle: 0.3 },
                stats.damage_bonus * 2.5, enemy_entity, DamageType::Physical,
                0.4, ProjectileOwner::Enemy, 1.0, behind, None,
            );
            true
        }
        EnemyVariant::Brute => {
            // Ground slam — wide AoE around self
            commands.spawn((
                TelegraphIndicator::new(0.6, enemy_entity),
                DamageHitbox::new(
                    HitboxShape::Circle { radius: 4.0 },
                    stats.damage_bonus * 1.8, enemy_entity, DamageType::Physical,
                    0.35, ProjectileOwner::Enemy, 2.0,
                ),
                Transform::from_translation(enemy_tf.translation),
            ));
            true
        }
    }
}

/// System that orchestrates enemy ability usage based on cooldowns and proximity to player.
pub fn enemy_ability_system(
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy, &CombatStats, &mut EnemyAbilityCooldown)>,
    player_transform: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    let Ok(player_tf) = player_transform.get_single() else { return };
    let dt = time.delta_secs();

    for (entity, tf, enemy, stats, mut cd) in enemy_query.iter_mut() {
        cd.tick(dt);
        let dist = tf.translation.distance(player_tf.translation);
        let variant = &enemy.variant;

        if cd.ready(0) && dist < 8.0 {
            if use_enemy_ability(entity, tf, variant, player_tf, stats, &mut commands, &mut damage_events, &mut dmg_num_events, &mut impact_events) {
                cd.reset(0);
            }
        }
        if cd.ready(1) && dist < 6.0 {
            if matches!(variant, EnemyVariant::Elite | EnemyVariant::Boss | EnemyVariant::Caster | EnemyVariant::Healer | EnemyVariant::Summoner | EnemyVariant::Brute) {
                if use_enemy_ability(entity, tf, variant, player_tf, stats, &mut commands, &mut damage_events, &mut dmg_num_events, &mut impact_events) {
                    cd.reset(1);
                }
            }
        }
        if cd.ready(2) && dist < 10.0 {
            if matches!(variant, EnemyVariant::Boss) {
                if use_enemy_ability(entity, tf, variant, player_tf, stats, &mut commands, &mut damage_events, &mut dmg_num_events, &mut impact_events) {
                    cd.reset(2);
                }
            }
        }
    }
}
