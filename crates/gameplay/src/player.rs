use bevy::prelude::*;
use ir_core::*;
use ir_world::map::WorldTile;
use ir_dungeon::rooms::DungeonWall;

/// Reads keyboard and mouse input into the PlayerInput resource.
/// Supports Dvorak-position-independent layout (WASD + arrows).
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
    input.interact = keyboard.just_pressed(KeyCode::KeyF);
}

/// Applies Equipment bonuses to CombatStats when a new run starts.
/// Starts from class base stats (already set by spawn), then adds equipment on top.
/// Also inserts ClassResource + Stamina if the player entity is missing them.
pub fn apply_equipment(
    mut commands: Commands,
    item_db: Res<ir_core::ItemDatabase>,
    player_query: Query<(Entity, &PlayerClass), Added<Player>>,
    mut equip_query: Query<(&Equipment, &mut CombatStats), With<Player>>,
) {
    let Ok((entity, class)) = player_query.get_single() else { return };
    // Ensure ClassResource and Stamina are inserted on fresh spawn
    commands.entity(entity).insert(class.0.default_resource());
    commands.entity(entity).insert(Stamina::default());
    let Ok((equip, mut stats)) = equip_query.get_single_mut() else { return };
    crate::equipment::recalc_equipment_stats(&item_db, equip, &mut *stats);
}

/// Handles player movement from input. Uses acceleration + friction model.
/// Movement is camera-relative: W = up-on-screen, not world +Z.
/// During stun/hit-stun, player input is skipped but existing velocity
/// (knockback, etc.) is preserved. During dash, input is also skipped.
///
/// Acceleration model:
///   - Ground friction: 0.85 (quick stop, 85% velocity retained per frame @60fps)
///   - Air friction:    0.95 (slippery, 95% retained per frame @60fps)
///   - Acceleration:    60.0 units/s² toward target velocity
///   - Knockback velocity feeds into Velocity so it interacts with friction
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
        Option<&Knockback>,
    ), With<Player>>,
) {
    // Get camera forward and right vectors projected onto the XZ plane
    let cam_rot = cam.1;
    let forward = -(cam_rot * Vec3::Z);
    let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = cam_rot * Vec3::X;
    let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    for (mut velocity, stats, dash, stun, hit_stun, frozen, _knockback) in query.iter_mut() {
        // During dash: skip player input, but preserve current velocity
        // (dash sets velocity separately in dash_ability)
        if dash.active {
            continue;
        }

        // Apply friction to current velocity (frame-rate independent)
        // Ground friction = 0.85 (85% retained per frame at 60fps)
        // Air friction = 0.95
        let friction = 0.85;
        let friction_factor = f32::powf(friction, time.delta_secs() * 60.0);
        velocity.0 *= friction_factor;

        // During stun/hit-stun: skip player input, velocity decays naturally
        // via friction (knockback still applies through apply_knockback feeding Velocity)
        if stun.is_some() || hit_stun.is_some() {
            continue;
        }

        let dir = input.direction;
        let mut speed = stats.move_speed + stats.move_speed_bonus;

        // Apply frozen slow (60% reduction)
        if frozen.is_some() {
            speed *= 0.4;
        }

        if dir.length_squared() > 0.0 {
            let n = dir.normalize();
            let target = (forward_xz * n.y + right_xz * n.x) * speed;

            // Accelerate toward target at 60 units/s²
            let accel = 60.0 * time.delta_secs();
            velocity.0 = velocity.0.lerp(target, accel.min(1.0));
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

/// Collision: pushes the player out of WorldTile edges (keeps player in world bounds)
/// and DungeonWall entities.
///
/// Simple approach: clamp player position based on tile bounding box and
/// push out of walls using a basic overlap test.
pub fn player_world_collision(
    mut player_query: Query<&mut Transform, With<Player>>,
    world_tiles: Query<&Transform, (With<WorldTile>, Without<Player>)>,
    dungeon_walls: Query<&Transform, (With<DungeonWall>, Without<Player>)>,
) {
    let Ok(mut player_tf) = player_query.get_single_mut() else { return };
    let player_pos = player_tf.translation;

    // ── WorldTile bounds: keep player inside the world tile grid ──
    if !world_tiles.is_empty() {
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_z = f32::MAX;
        let mut max_z = f32::MIN;
        for tile_tf in world_tiles.iter() {
            let p = tile_tf.translation;
            // Each tile is 2.0 wide (spacing), centered at its position
            let half = 1.0; // tile half-size (2.0 spacing / 2)
            min_x = min_x.min(p.x - half);
            max_x = max_x.max(p.x + half);
            min_z = min_z.min(p.z - half);
            max_z = max_z.max(p.z + half);
        }
        // Clamp player position within tile bounds (with a small margin)
        let margin = 0.3;
        player_tf.translation.x = player_tf.translation.x.clamp(min_x + margin, max_x - margin);
        player_tf.translation.z = player_tf.translation.z.clamp(min_z + margin, max_z - margin);
    }

    // ── DungeonWall collision: push player out ──
    // Walls are 0.3 wide, 2.0 tall, and (TILE*0.9) ≈ 1.8 long
    let player_radius = 0.4;
    for wall_tf in dungeon_walls.iter() {
        let wall_pos = wall_tf.translation;
        // Wall size: width=0.3, length=1.8 (TILE*0.9 ≈ 1.8 with TILE=2.0)
        let wall_half_w = 0.15; // half of 0.3 wall width
        let wall_half_l = 0.9;  // half of ~1.8 wall length

        // Determine wall orientation from rotation
        let is_z_axis = wall_tf.rotation == Quat::IDENTITY;

        let (wx, wz) = if is_z_axis {
            // Wall faces along Z axis: wide in Z, narrow in X
            (wall_half_w, wall_half_l)
        } else {
            // Wall faces along X axis: wide in X, narrow in Z
            (wall_half_l, wall_half_w)
        };

        // Simple AABB overlap check
        let dx = player_pos.x - wall_pos.x;
        let dz = player_pos.z - wall_pos.z;

        let overlap_x = wx + player_radius - dx.abs();
        let overlap_z = wz + player_radius - dz.abs();

        if overlap_x > 0.0 && overlap_z > 0.0 {
            // Push out along the axis of least penetration
            if overlap_x < overlap_z {
                // Push in X
                player_tf.translation.x += overlap_x * dx.signum();
            } else {
                // Push in Z
                player_tf.translation.z += overlap_z * dz.signum();
            }
        }
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
