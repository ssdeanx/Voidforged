use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

/// Enemy AI: move toward player when within aggro range.
/// Skips enemies that are in windup (telegraphing).
pub fn enemy_ai(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Velocity, &Transform, &CombatStats, &AttackCooldown), With<Enemy>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();

    for (enemy, mut velocity, transform, stats, cooldown) in enemy_query.iter_mut() {
        // Don't move during windup (telegraphing)
        if cooldown.windup > 0.0 {
            velocity.0 = Vec3::ZERO;
            continue;
        }

        let to_player = player_pos - transform.translation;
        let dist = to_player.length();

        let behavior = match enemy.variant {
            EnemyVariant::Grunt => {
                if dist < 15.0 {
                    to_player.normalize_or_zero() * stats.move_speed
                } else {
                    Vec3::ZERO
                }
            }
            EnemyVariant::Ranged => {
                if dist < 8.0 {
                    -to_player.normalize_or_zero() * stats.move_speed
                } else if dist < 18.0 {
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    (to_player.normalize_or_zero() * 0.3 + strafe * 0.7) * stats.move_speed
                } else {
                    to_player.normalize_or_zero() * stats.move_speed * 0.5
                }
            }
            EnemyVariant::Charger => {
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
                if dist < 18.0 {
                    to_player.normalize_or_zero() * stats.move_speed * (0.7 + rng.gen::<f32>() * 0.3)
                } else {
                    to_player.normalize_or_zero() * stats.move_speed
                }
            }
            EnemyVariant::Boss => {
                to_player.normalize_or_zero() * stats.move_speed * 0.6
            }
        };

        velocity.0 = Vec3::new(behavior.x, 0.0, behavior.z);
    }
}

/// Applies velocity to enemy positions.
pub fn apply_enemy_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Enemy>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

/// Melee enemies telegraph then deal damage on contact.
pub fn enemy_melee_attack(
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform, &mut AttackCooldown, &CombatStats), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };

    for (enemy_entity, enemy, transform, mut cooldown, stats) in enemy_query.iter_mut() {
        let dist = transform.translation.distance(player_pos);
        let melee_range = if enemy.variant == EnemyVariant::Boss { 3.5 } else { 2.0 };

        // If windup is active, tick it and deal damage when it expires
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                // Telegraph complete — deal damage
                let base_dmg = match enemy.variant {
                    EnemyVariant::Grunt => 8.0,
                    EnemyVariant::Ranged => 5.0,
                    EnemyVariant::Charger => 15.0,
                    EnemyVariant::Elite => 20.0,
                    EnemyVariant::Boss => 40.0,
                };
                damage_events.send(DamageEvent {
                    target: player_entity,
                    source: enemy_entity,
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
                cooldown.windup = 0.0;
            }
            continue;
        }

        // Tick normal cooldown
        cooldown.timer = (cooldown.timer - time.delta_secs()).max(0.0);

        // Start windup if in range and cooldown is ready
        if dist < melee_range && cooldown.timer <= 0.0 {
            // Charger has longer telegraph, others shorter
            cooldown.windup = match enemy.variant {
                EnemyVariant::Charger => 0.5,
                EnemyVariant::Boss => 0.6,
                _ => 0.3,
            };
        }
    }
}

/// Ranged enemies telegraph then fire projectiles.
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

        let dist = transform.translation.distance(player_pos);

        // If windup is active, tick it and fire when it expires
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                // Telegraph complete — fire
                let direction = (player_pos - transform.translation).normalize_or_zero();
                let spawn_pos = transform.translation + Vec3::Y * 0.5;
                commands.spawn(ProjectileBundle::new(
                    8.0, 10.0, 3.0, direction, spawn_pos, ProjectileOwner::Enemy,
                ));
                cooldown.timer = 2.0;
                cooldown.windup = 0.0;
            }
            continue;
        }

        // Tick normal cooldown
        cooldown.timer = (cooldown.timer - time.delta_secs()).max(0.0);

        // Start windup if in range and cooldown is ready
        if dist < 20.0 && dist > 3.0 && cooldown.timer <= 0.0 {
            cooldown.windup = 0.4; // 0.4s telegraph before ranged attack
        }
    }
}
