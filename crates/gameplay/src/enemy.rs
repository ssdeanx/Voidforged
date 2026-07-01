//! Enemy AI — movement, formations, telegraphing, ranged/melee attacks, boss phases.
//! Each variant has distinct behavior:
//! - Grunt: surround player, charge directly
//! - Ranged: strafe at mid-range, flee when close, fire projectiles
//! - Charger: circle then charge, high knockback
//! - Elite: aggressive melee with periodic AoE
//! - Boss: phase-based attack patterns

use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

/// Aggro ranges per variant (max_dist, min_dist).
fn aggro_range(variant: &EnemyVariant) -> (f32, f32) {
    match variant {
        EnemyVariant::Grunt => (15.0, 0.0),
        EnemyVariant::Ranged => (22.0, 4.0),  // flee when < 4 units
        EnemyVariant::Charger => (25.0, 2.0),
        EnemyVariant::Elite => (18.0, 0.0),
        EnemyVariant::Boss => (30.0, 0.0),
    }
}

/// Enemy AI: movement with formation awareness and aggro ranges.
/// Skips enemies in windup (telegraphing) or stunned.
pub fn enemy_ai(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(
        &Enemy,
        &mut Velocity,
        &Transform,
        &CombatStats,
        &AttackCooldown,
        Option<&Stun>,
    )>,
    other_enemies: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();

    // Collect positions of all enemies for formation awareness
    let enemy_positions: Vec<Vec3> = other_enemies
        .iter()
        .map(|t| t.translation)
        .collect();

    for (enemy, mut velocity, transform, stats, cooldown, stun) in enemy_query.iter_mut() {
        // Don't move during windup or stun
        if cooldown.windup > 0.0 || stun.is_some() {
            velocity.0 = Vec3::ZERO;
            continue;
        }

        let to_player = player_pos - transform.translation;
        let dist = to_player.length();
        let (max_aggro, flee_dist) = aggro_range(&enemy.variant);

        // Aggro check: skip if outside range
        if dist > max_aggro {
            velocity.0 = Vec3::ZERO;
            continue;
        }

        let dir_to_player = if dist > 0.1 {
            (to_player / dist) * Vec3::new(1.0, 0.0, 1.0)
        } else {
            Vec3::ZERO
        };

        let behavior = match enemy.variant {
            EnemyVariant::Grunt => {
                // Surround player: approach directly
                // If too close to other grunts, spread out slightly
                let mut avoid = Vec3::ZERO;
                for other_pos in enemy_positions.iter() {
                    let to_other = transform.translation - *other_pos;
                    let other_dist = to_other.length();
                    if other_dist < 2.0 && other_dist > 0.1 {
                        avoid += to_other / other_dist;
                    }
                }
                let move_dir = (dir_to_player + avoid * 0.3).normalize_or_zero();
                move_dir * stats.move_speed
            }
            EnemyVariant::Ranged => {
                if dist < flee_dist {
                    // Flee when player is too close
                    -dir_to_player * stats.move_speed * 1.2
                } else if dist < 18.0 {
                    // Strafe around player at mid-range
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    // Alternate strafe direction for variety
                    let strafe_dir = if (transform.translation.x + transform.translation.z) as i32 % 2 == 0 {
                        strafe
                    } else {
                        -strafe
                    };
                    (dir_to_player * 0.2 + strafe_dir * 0.8) * stats.move_speed
                } else {
                    // Approach slowly
                    dir_to_player * stats.move_speed * 0.5
                }
            }
            EnemyVariant::Charger => {
                if dist < 20.0 && dist > 3.0 {
                    // Circle the player, then charge when angle is right
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    let circle_speed = stats.move_speed * 0.7;
                    let charge_dir = if dist > 10.0 {
                        dir_to_player * stats.move_speed
                    } else {
                        strafe * circle_speed
                    };
                    charge_dir
                } else if dist <= 3.0 {
                    // Too close, back off slightly
                    -dir_to_player * stats.move_speed * 0.3
                } else {
                    Vec3::ZERO
                }
            }
            EnemyVariant::Elite => {
                if dist < 18.0 {
                    // Aggressive pursuit with speed variation
                    let wobble = Vec3::new(
                        (rng.gen::<f32>() - 0.5) * 2.0,
                        0.0,
                        (rng.gen::<f32>() - 0.5) * 2.0,
                    );
                    (dir_to_player + wobble * 0.15).normalize_or_zero()
                        * stats.move_speed
                        * (0.8 + rng.gen::<f32>() * 0.4)
                } else {
                    dir_to_player * stats.move_speed
                }
            }
            EnemyVariant::Boss => {
                // Boss moves toward player but at varying speed
                // Boss phase behavior handled in boss_phase_ai
                dir_to_player * stats.move_speed * 0.5
            }
        };

        velocity.0 = Vec3::new(behavior.x, 0.0, behavior.z);
    }
}
pub fn boss_phase_ai(
    _time: Res<Time>,
    mut boss_query: Query<(
        Entity,
        &Enemy,
        &mut Velocity,
        &Transform,
        &Health,
        &CombatStats,
        &mut AttackCooldown,
    ), With<Enemy>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };

    for (boss_entity, enemy, mut velocity, transform, health, stats, mut cooldown) in boss_query.iter_mut() {
        if enemy.variant != EnemyVariant::Boss {
            continue;
        }

        let hp_pct = health.fraction();
        let to_player = player_pos - transform.translation;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);

        match hp_pct {
            // Phase 1: 100%–60% HP — standard approach
            p if p > 0.6 => {
                velocity.0 = dir * stats.move_speed * 0.5;
            }
            // Phase 2: 60%–30% HP — faster, more aggressive
            p if p > 0.3 => {
                velocity.0 = dir * stats.move_speed * 0.8;
                // Shorter windup
                if cooldown.windup > 0.0 && cooldown.windup > 0.3 {
                    cooldown.windup = 0.3;
                }
            }
            // Phase 3: < 30% HP — enrage, fast attacks, AoE pulses
            _ => {
                velocity.0 = dir * stats.move_speed * 1.1;
                // Boss periodically does a ground slam (AoE) in this phase
                if dist < 6.0 && cooldown.timer <= 0.0 {
                    // Direct damage to player
                    damage_events.send(DamageEvent {
                        target: player_entity,
                        source: boss_entity,
                        amount: 15.0 + stats.damage_bonus * 0.5,
                        is_critical: false,
                        damage_type: DamageType::Physical,
                    });
                    cooldown.timer = 2.0;
                }
            }
        }
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
/// Spawns telegraph visual indicators during windup.
pub fn enemy_melee_attack(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(
        Entity,
        &Enemy,
        &Transform,
        &mut AttackCooldown,
        &CombatStats,
    )>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };

    for (enemy_entity, enemy, transform, mut cooldown, stats) in enemy_query.iter_mut() {
        // Skip ranged enemies (handled by enemy_ranged_attack)
        if enemy.variant == EnemyVariant::Ranged {
            continue;
        }

        let dist = transform.translation.distance(player_pos);
        let melee_range = match enemy.variant {
            EnemyVariant::Boss => 3.5,
            EnemyVariant::Charger => 3.0,
            EnemyVariant::Elite => 3.0,
            _ => 2.0,
        };

        // If windup is active, tick it and deal damage when it expires
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                // Telegraph complete — deal damage
                let base_dmg = match enemy.variant {
                    EnemyVariant::Grunt => 8.0,
                    EnemyVariant::Charger => 18.0,
                    EnemyVariant::Elite => 25.0,
                    EnemyVariant::Boss => 45.0,
                    _ => 10.0,
                };
                let dmg = base_dmg + stats.damage_bonus;
                damage_events.send(DamageEvent {
                    target: player_entity,
                    source: enemy_entity,
                    amount: dmg,
                    is_critical: false,
                    damage_type: DamageType::Physical,
                });
                // Charger hits apply knockback and stun
                if enemy.variant == EnemyVariant::Charger {
                    let kb_dir = (player_pos - transform.translation)
                        .normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
                    if kb_dir.length_squared() > 0.1 {
                        commands.entity(player_entity).insert(Knockback::new(
                            kb_dir * 15.0,
                            4.0,
                        ));
                    }
                    commands.entity(player_entity).insert(Stun::new(0.5));
                }
                let interval = match enemy.variant {
                    EnemyVariant::Boss => 1.5,
                    EnemyVariant::Charger => 2.5,
                    EnemyVariant::Elite => 1.2,
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
            cooldown.windup = match enemy.variant {
                EnemyVariant::Charger => 0.6,
                EnemyVariant::Boss => 0.8,
                EnemyVariant::Elite => 0.4,
                _ => 0.3,
            };
            // Spawn telegraph indicator
            commands.spawn((
                TelegraphIndicator::new(cooldown.windup, enemy_entity),
                Transform::from_translation(transform.translation + Vec3::Y * 0.1),
            ));
        }
    }
}

/// Ranged enemies telegraph then fire projectiles.
pub fn enemy_ranged_attack(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform, &mut AttackCooldown)>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (enemy_entity, enemy, transform, mut cooldown) in enemy_query.iter_mut() {
        if enemy.variant != EnemyVariant::Ranged {
            continue;
        }

        let dist = transform.translation.distance(player_pos);
        let (_, flee_dist) = aggro_range(&EnemyVariant::Ranged);

        // If windup is active, tick it and fire when it expires
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                // Telegraph complete — fire
                let direction = (player_pos - transform.translation).normalize_or_zero();
                let spawn_pos = transform.translation + Vec3::Y * 0.5;
                // Enemy projectiles are slightly slower but have a visual marker
                let _proj = commands.spawn(ProjectileBundle::new(
                    8.0, 10.0, 3.0, direction, spawn_pos, ProjectileOwner::Enemy,
                )).insert(EnemyProjectileMarker);
                cooldown.timer = 2.5;
                cooldown.windup = 0.0;
            }
            continue;
        }

        // Tick normal cooldown
        cooldown.timer = (cooldown.timer - time.delta_secs()).max(0.0);

        // Start windup if in range (but not too close) and cooldown is ready
        if dist < 22.0 && dist > flee_dist && cooldown.timer <= 0.0 {
            cooldown.windup = 0.5; // 0.5s telegraph before ranged attack
            // Spawn telegraph indicator
            commands.spawn((
                TelegraphIndicator::new(cooldown.windup, enemy_entity),
                Transform::from_translation(transform.translation + Vec3::Y * 0.1),
            ));
        }
    }
}
