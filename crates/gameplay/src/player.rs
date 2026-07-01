use bevy::prelude::*;
use ir_core::*;

/// Reads keyboard and mouse input into the PlayerInput resource.
pub fn read_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<PlayerInput>,
) {
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
    input.primary_attack = mouse.pressed(MouseButton::Left);
    input.secondary_attack = mouse.pressed(MouseButton::Right);
    input.cast = keyboard.just_pressed(KeyCode::KeyQ);
    input.dodge = keyboard.just_pressed(KeyCode::ShiftLeft)
        || keyboard.just_pressed(KeyCode::ShiftRight);
    input.pause = keyboard.just_pressed(KeyCode::Escape);
}

/// Applies Equipment bonuses to CombatStats when a new run starts.
pub fn apply_equipment(
    mut player_query: Query<(&Equipment, &mut CombatStats), With<Player>>,
) {
    let (_equip, mut stats) = match player_query.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    // TODO: Apply equipped item stats when ItemSystem is wired
    stats.damage_bonus = 0.0;
    stats.attack_speed_bonus = 0.0;
}

/// Handles player movement from input. Uses acceleration for weight.
/// Movement is camera-relative: W = up-on-screen, not world +Z.
/// Skips movement if stunned or hit-stunned.
pub fn player_movement(
    time: Res<Time>,
    input: Res<PlayerInput>,
    cam: Res<ir_core::CameraTransform>,
    mut query: Query<(
        &mut Velocity,
        &CombatStats,
        &DashCooldown,
        Option<&Stun>,
        Option<&HitStun>,
        Option<&Frozen>,
    ), With<Player>>,
) {
    // Get camera forward and right vectors projected onto the XZ plane
    let cam_rot = cam.1;
    let forward = -(cam_rot * Vec3::Z);
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = cam_rot * Vec3::X;
    let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    for (mut velocity, stats, dash, stun, hit_stun, frozen) in query.iter_mut() {
        // Don't allow manual movement during dash, stun, or hit-stun
        if dash.active || stun.is_some() || hit_stun.is_some() {
            velocity.0 = Vec3::ZERO;
            continue;
        }
        let dir = input.direction;
        let mut speed = stats.move_speed + stats.move_speed_bonus;

        // Apply frozen slow (60% reduction)
        if frozen.is_some() {
            speed *= 0.4;
        }

        let target = if dir.length_squared() > 0.0 {
            let n = dir.normalize();
            (forward_xz * n.y + right_xz * n.x) * speed
        } else {
            Vec3::ZERO
        };
        // Lerp velocity for smooth acceleration
        let accel = 10.0 * time.delta_secs();
        velocity.0 = velocity.0.lerp(target, accel.min(1.0));
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

/// Fires a projectile toward cursor. Shared by primary, secondary, and dash attacks.
fn fire_toward_cursor(
    commands: &mut Commands,
    origin: Vec3,
    cursor: Vec3,
    damage: f32,
    speed: f32,
    lifetime: f32,
    owner: ProjectileOwner,
) {
    let direction = (cursor - origin).normalize_or_zero();
    if direction.length_squared() < 0.1 {
        return;
    }
    commands.spawn(ProjectileBundle::new(
        damage, speed, lifetime, direction, origin + Vec3::Y * 0.5, owner,
    ));
}

/// Primary attack toward cursor. Hold left mouse for continuous fire on cooldown.
pub fn player_attack(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(&Transform, &mut Weapon, &CombatStats), With<Player>>,
) {
    let (transform, mut weapon, stats) = match player_query.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    weapon.cooldown_timer = (weapon.cooldown_timer - time.delta_secs()).max(0.0);

    if !input.primary_attack || weapon.cooldown_timer > 0.0 {
        return;
    }

    let interval = 1.0 / (weapon.attack_speed + stats.attack_speed_bonus);
    weapon.cooldown_timer = interval;
    let dmg = weapon.damage + stats.damage_bonus;
    fire_toward_cursor(&mut commands, transform.translation, cursor.0, dmg, 15.0, 2.0, ProjectileOwner::Player);
}

/// Secondary attack — right mouse. Fires a spread of 3 projectiles.
pub fn player_secondary_attack(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    player_query: Query<(&Transform, &CombatStats), With<Player>>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.secondary_attack || *cooldown > 0.0 {
        return;
    }
    let (transform, stats) = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    *cooldown = 0.8;
    let dmg = 8.0 + stats.damage_bonus * 0.5;

    let base_dir = (cursor.0 - transform.translation).normalize_or_zero();
    if base_dir.length_squared() < 0.1 {
        return;
    }
    let origin = transform.translation + Vec3::Y * 0.5;
    for spread in [-0.15, 0.0, 0.15] {
        let rotated = Quat::from_rotation_y(spread) * base_dir;
        commands.spawn(ProjectileBundle::new(
            dmg, 12.0, 1.5, rotated, origin, ProjectileOwner::Player,
        ));
    }
}

/// Cast ability — Q key. Fires a slow, high-damage piercing projectile.
pub fn player_cast(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    player_query: Query<(&Transform, &CombatStats), With<Player>>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.cast || *cooldown > 0.0 {
        return;
    }
    let (transform, stats) = match player_query.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    *cooldown = 3.0;

    let base_dir = (cursor.0 - transform.translation).normalize_or_zero();
    if base_dir.length_squared() < 0.1 {
        return;
    }
    let origin = transform.translation + Vec3::Y * 0.5;

    commands.spawn(ProjectileBundle {
        projectile: Projectile {
            damage: (20.0 + stats.damage_bonus * 2.0),
            speed: 8.0,
            lifetime: 4.0,
            max_lifetime: 4.0,
            piercing: true,
            owner: ProjectileOwner::Player,
        },
        position: Position(origin),
        velocity: Velocity(base_dir * 8.0),
        render_info: RenderInfo::default(),
        room_entity: RoomEntity,
    });
}
