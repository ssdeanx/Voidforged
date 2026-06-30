use bevy::prelude::*;
use ir_core::*;

/// Reads keyboard input into the PlayerInput resource.
pub fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    // Direction from WASD/arrows
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    input.direction = dir.clamp_length_max(1.0);
    input.primary_attack = keyboard.pressed(KeyCode::Space) || keyboard.pressed(KeyCode::Enter);
    input.dodge = keyboard.just_pressed(KeyCode::ShiftLeft) || keyboard.just_pressed(KeyCode::ShiftRight);
    input.pause = keyboard.just_pressed(KeyCode::Escape);
}

/// Handles player movement from input.
pub fn player_movement(
    _time: Res<Time>,
    input: Res<PlayerInput>,
    mut query: Query<(&mut Velocity, &CombatStats), With<Player>>,
) {
    for (mut velocity, stats) in query.iter_mut() {
        let dir = input.direction;
        if dir.length_squared() > 0.0 {
            let normalized = dir.normalize();
            velocity.0 = Vec3::new(normalized.x * stats.move_speed, 0.0, normalized.y * stats.move_speed);
        } else {
            velocity.0 = Vec3::ZERO;
        }
    }
}

/// Applies velocity to player position.
pub fn apply_player_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

/// Auto-attack system — fires projectiles toward nearest enemy on cooldown.
pub fn player_auto_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &mut Weapon, &CombatStats), With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let (_player_entity, player_transform, mut weapon, stats) = match player_query.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    // Find nearest enemy
    let target = enemy_query
        .iter()
        .map(|t| (t.translation.distance(player_transform.translation), t))
        .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Update cooldown
    let attack_interval = 1.0 / (weapon.attack_speed + stats.attack_speed_bonus);
    weapon.cooldown_timer -= time.delta_secs();

    if let Some((_dist, enemy_transform)) = target {
        if weapon.cooldown_timer <= 0.0 {
            let direction = (enemy_transform.translation - player_transform.translation).normalize_or_zero();
            let spawn_pos = player_transform.translation + Vec3::Y * 0.5;

            commands.spawn(ProjectileBundle::new(
                weapon.damage + stats.damage_bonus,
                15.0,
                2.0,
                direction,
                spawn_pos,
                ProjectileOwner::Player,
            ));

            weapon.cooldown_timer = attack_interval;
        }
    }
}
