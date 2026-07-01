use bevy::prelude::*;
use ir_core::*;

/// Moves projectiles and checks lifetime.
pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &mut Transform, &Velocity)>,
) {
    for (entity, mut projectile, mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Player projectiles hit enemies.
pub fn projectile_hit(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (proj_entity, projectile, proj_transform) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Player {
            continue;
        }
        for (enemy_entity, enemy_transform) in enemies.iter() {
            let dist = proj_transform.translation.distance(enemy_transform.translation);
            if dist < 1.0 {
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: proj_entity,
                    amount: projectile.damage,
                    is_critical: false,
                    damage_type: DamageType::Physical,
                });
                if !projectile.piercing {
                    commands.entity(proj_entity).despawn();
                }
                break;
            }
        }
    }
}

/// Enemy projectiles hit the player.
pub fn projectile_hit_player(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    player_query: Query<(&Transform, &Health), With<Player>>,
) {
    let (player_transform, _player_health) = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    for (proj_entity, projectile, proj_transform) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Enemy {
            continue;
        }
        let dist = proj_transform.translation.distance(player_transform.translation);
        if dist < 0.8 {
            damage_events.send(DamageEvent {
                target: Entity::from_raw(0),
                source: proj_entity,
                amount: projectile.damage,
                is_critical: false,
                damage_type: DamageType::Physical,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}

/// Processes damage events.
pub fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut death_events: EventWriter<DeathEvent>,
    time: Res<Time>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if health.take_damage(event.amount, time.elapsed_secs() as f32) {
                if !health.is_alive() {
                    death_events.send(DeathEvent {
                        entity: event.target,
                        killer: Some(event.source),
                        enemy_variant: None,
                    });
                }
            }
        }
    }
}

/// Handles death events — despawns entities and spawns XP gems.
pub fn handle_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    enemy_query: Query<(&Enemy, &Transform)>,
) {
    for event in death_events.read() {
        if let Ok((enemy, transform)) = enemy_query.get(event.entity) {
            let gem_pos = transform.translation + Vec3::new(0.0, 0.5, 0.0);
            let _gem_entity = commands.spawn(ExperienceGemBundle::new(enemy.xp_reward, gem_pos));
            commands.entity(event.entity).despawn();
        }
    }
}
