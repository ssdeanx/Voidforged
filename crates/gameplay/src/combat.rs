use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

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
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
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
                let hit_pos = enemy_transform.translation + Vec3::Y * 1.0;
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: proj_entity,
                    amount: projectile.damage,
                    is_critical: false,
                    damage_type: DamageType::Physical,
                });
                dmg_num_events.send(DamageNumberEvent {
                    position: hit_pos,
                    amount: projectile.damage as i32,
                    is_crit: false,
                });
                impact_events.send(SpawnImpactEvent {
                    position: hit_pos,
                    color: None,
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
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let (player_entity, player_transform) = match player_query.get_single() {
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
                target: player_entity,
                source: proj_entity,
                amount: projectile.damage,
                is_critical: false,
                damage_type: DamageType::Physical,
            });
            dmg_num_events.send(DamageNumberEvent {
                position: player_transform.translation + Vec3::Y * 1.5,
                amount: projectile.damage as i32,
                is_crit: false,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}

/// Processes damage events. Applies crit, armor, lifesteal, sends DeathEvent.
pub fn apply_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut death_events: EventWriter<DeathEvent>,
    player_query: Query<(Entity, &CombatStats), With<Player>>,
    enemy_query: Query<&CombatStats, With<Enemy>>,
    mut shake: ResMut<ScreenShake>,
    time: Res<Time>,
) {
    let (player_entity, player_stats) = player_query
        .get_single()
        .map(|(e, s)| (Some(e), s.clone()))
        .unwrap_or((None, CombatStats::default()));

    let mut rng = rand::thread_rng();
    let mut dead_this_frame: Vec<Entity> = Vec::new();

    for event in damage_events.read() {
        let target = event.target;
        if dead_this_frame.contains(&target) {
            continue;
        }
        let mut amount = event.amount;
        let source = event.source;

        // Crit roll for player damage
        let is_crit = if Some(source) == player_entity {
            rng.gen::<f32>() < player_stats.crit_chance
        } else {
            event.is_critical
        };
        if is_crit {
            amount *= player_stats.crit_multiplier;
        }

        // Armor reduction for targets with CombatStats
        let mitigated = if let Ok(target_stats) = enemy_query.get(target) {
            let reduction = target_stats.armor / (target_stats.armor + 100.0);
            amount * (1.0 - reduction)
        } else if Some(target) == player_entity {
            amount * player_stats.damage_taken_multiplier
        } else {
            amount
        };

        let raw_dmg = mitigated.max(1.0); // minimum 1 damage
                                          // Screen shake on player hit
        if Some(target) == player_entity {
            shake.trauma = (shake.trauma + raw_dmg / 100.0).min(1.0);
        }
        let is_dead = health_query
            .get_mut(target)
            .ok()
            .map(|mut health| {
                let died = health.take_damage(raw_dmg, time.elapsed_secs() as f32);
                died && !health.is_alive()
            })
            .unwrap_or(false);

        // Lifesteal (separate borrow)
        if Some(source) == player_entity && player_stats.lifesteal > 0.0 {
            if let Some(player_entity) = player_entity {
                if let Ok(mut player_health) = health_query.get_mut(player_entity) {
                    player_health.heal(raw_dmg * player_stats.lifesteal);
                }
            }
        }

        if is_dead {
            dead_this_frame.push(target);
            death_events.send(DeathEvent {
                entity: target,
                killer: Some(source),
                enemy_variant: None,
            });
        }
    }
}

/// Handles death events — spawns loot, despawns enemies.
pub fn handle_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    enemy_query: Query<(&Enemy, &Transform)>,
    assets: Res<GameAssets>,
) {
    for event in death_events.read() {
        if let Ok((enemy, transform)) = enemy_query.get(event.entity) {
            let pos = transform.translation + Vec3::Y * 0.5;
            ir_procedural::loot::spawn_loot(&mut commands, &assets, pos, &enemy.variant, enemy.tier);
            commands.entity(event.entity).despawn();
        }
    }
}
