use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

/// Enemy AI: move toward player when within aggro range.
pub fn enemy_ai(
    _time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Velocity, &Transform, &CombatStats), With<Enemy>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();

    for (enemy, mut velocity, transform, stats) in enemy_query.iter_mut() {
        let to_player = player_pos - transform.translation;
        let dist = to_player.length();

        let behavior = match enemy.variant {
            EnemyVariant::Grunt => {
                // Simple chase
                if dist < 15.0 {
                    to_player.normalize_or_zero() * stats.move_speed
                } else {
                    Vec3::ZERO
                }
            }
            EnemyVariant::Ranged => {
                // Keep distance, strafe
                if dist < 8.0 {
                    // Too close — back away
                    -to_player.normalize_or_zero() * stats.move_speed
                } else if dist < 18.0 {
                    // Ideal range — strafe slightly
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    (to_player.normalize_or_zero() * 0.3 + strafe * 0.7) * stats.move_speed
                } else {
                    // Too far — approach
                    to_player.normalize_or_zero() * stats.move_speed * 0.5
                }
            }
            EnemyVariant::Charger => {
                // Fast charge toward player, slight randomness
                if dist < 20.0 && dist > 3.0 {
                    let wobble = Vec3::new(
                        (rng.gen::<f32>() - 0.5) * 2.0,
                        0.0,
                        (rng.gen::<f32>() - 0.5) * 2.0,
                    );
                    (to_player.normalize_or_zero() + wobble * 0.2).normalize_or_zero() * stats.move_speed
                } else {
                    Vec3::ZERO
                }
            }
            EnemyVariant::Elite => {
                // Steady advance with variable speed
                if dist < 18.0 {
                    to_player.normalize_or_zero() * stats.move_speed * (0.7 + rng.gen::<f32>() * 0.3)
                } else {
                    to_player.normalize_or_zero() * stats.move_speed
                }
            }
            EnemyVariant::Boss => {
                // Always advances, slow but relentless
                to_player.normalize_or_zero() * stats.move_speed * 0.6
            }
        };

        velocity.0 = Vec3::new(behavior.x, 0.0, behavior.z);
    }
}

/// Applies velocity to enemy positions.
pub fn apply_enemy_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, Option<&Enemy>)>,
) {
    for (mut transform, velocity, _enemy) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

/// Melee enemies deal damage to the player on contact.
pub fn enemy_melee_attack(
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut AttackCooldown, &CombatStats), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };

    for (enemy, transform, mut cooldown, stats) in enemy_query.iter_mut() {
        cooldown.timer -= time.delta_secs();
        if cooldown.timer > 0.0 {
            continue;
        }

        let dist = transform.translation.distance(player_pos);
        let melee_range = if enemy.variant == EnemyVariant::Boss { 3.0 } else { 1.8 };

        if dist < melee_range {
            let base_dmg = match enemy.variant {
                EnemyVariant::Grunt => 8.0,
                EnemyVariant::Ranged => 5.0,
                EnemyVariant::Charger => 15.0,
                EnemyVariant::Elite => 20.0,
                EnemyVariant::Boss => 40.0,
            };
            damage_events.send(DamageEvent {
                target: player_entity,
                source: Entity::from_raw(0),
                amount: base_dmg + stats.damage_bonus,
                is_critical: false,
                damage_type: DamageType::Physical,
            });
            let interval = match enemy.variant {
                EnemyVariant::Boss => 1.5,
                EnemyVariant::Charger => 2.0,
                _ => 1.0,
            };
            cooldown.timer = interval;
        }
    }
}

/// Ranged enemies fire projectiles at the player.
pub fn enemy_ranged_attack(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Enemy, &Transform, &mut AttackCooldown), With<Enemy>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (enemy, transform, mut cooldown) in enemy_query.iter_mut() {
        if enemy.variant != EnemyVariant::Ranged {
            continue;
        }

        cooldown.timer -= time.delta_secs();
        if cooldown.timer > 0.0 {
            continue;
        }

        let dist = transform.translation.distance(player_pos);
        if dist < 20.0 && dist > 3.0 {
            let direction = (player_pos - transform.translation).normalize_or_zero();
            let spawn_pos = transform.translation + Vec3::Y * 0.5;
            commands.spawn(ProjectileBundle::new(
                8.0,
                10.0,
                3.0,
                direction,
                spawn_pos,
                ProjectileOwner::Enemy,
            ));
            cooldown.timer = 2.0;
        }
    }
}
