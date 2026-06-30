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
