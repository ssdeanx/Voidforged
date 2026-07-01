//! Procedural dungeon room generation with corridors between rooms.

use bevy::prelude::*;
use ir_core::*;

/// Marker for dungeon floor tiles.
#[derive(Component)]
pub struct DungeonFloor;

/// Marker for dungeon walls.
#[derive(Component)]
pub struct DungeonWall;

/// Marker for the dungeon exit.
#[derive(Component)]
pub struct DungeonExit;

const TILE: f32 = 2.0; // tile spacing
const ROOM_TILES: i32 = 5;   // 5x5 tiles per room
const CORRIDOR_LEN: i32 = 2; // tiles between rooms (so rooms don't overlap walls)

/// Generates dungeon rooms, corridors, and exit when entering a dungeon.
pub fn generate_dungeon(
    mut commands: Commands,
    dungeon_state: Res<DungeonState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let instance = match &dungeon_state.current {
        Some(d) => d,
        None => return,
    };

    info!("Dungeon: {} (tier {}, {} rooms)", instance.name, instance.tier, instance.depth);

    let floor_mesh = meshes.add(Cuboid::new(TILE * 0.9, 0.05, TILE * 0.9));
    let floor_mat = materials.add(Color::srgb(0.15, 0.12, 0.1));
    let exit_mat = materials.add(Color::srgb(0.0, 0.5, 0.0));
    let wall_mesh = meshes.add(Cuboid::new(0.3, 2.0, TILE * 0.9));
    let wall_mat = materials.add(Color::srgb(0.3, 0.25, 0.2));

    // Enemy mesh/material (reuse from projectile as placeholder)
    let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.8, 0.5));
    let enemy_mat_grunt = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let enemy_mat_ranged = materials.add(Color::srgb(0.9, 0.5, 0.1));

    // 3×3 room grid (simplified depth = number of rooms to clear)
    let rooms_per_side = 3i32;
    let room_step = ROOM_TILES + CORRIDOR_LEN; // tiles between room centers

    for rx in 0..rooms_per_side {
        for rz in 0..rooms_per_side {
            let base_x = (rx * room_step) as f32 * TILE;
            let base_z = (rz * room_step) as f32 * TILE;
            let is_exit = rx == rooms_per_side - 1 && rz == rooms_per_side - 1;
            let is_entrance = rx == 0 && rz == 0;

            // Floor tiles for this room
            for tx in 0..ROOM_TILES {
                for tz in 0..ROOM_TILES {
                    commands.spawn((
                        Mesh3d(floor_mesh.clone()),
                        MeshMaterial3d(floor_mat.clone()),
                        Transform::from_xyz(
                            base_x + tx as f32 * TILE,
                            -0.5,
                            base_z + tz as f32 * TILE,
                        ),
                        DungeonFloor,
                        RoomEntity,
                    ));
                }
            }

            // Green exit marker tile in the exit room
            if is_exit {
                let center = (ROOM_TILES / 2) as f32 * TILE;
                commands.spawn((
                    Mesh3d(floor_mesh.clone()),
                    MeshMaterial3d(exit_mat.clone()),
                    Transform::from_xyz(base_x + center, -0.45, base_z + center),
                    DungeonExit,
                    RoomEntity,
                ));
            }

            // Walls around each room, skipping door positions
            for i in 0..ROOM_TILES {
                let i_f = i as f32 * TILE;
                for (wx, wz, is_z_axis) in [
                    // north wall (z-)
                    (base_x + i_f, base_z - TILE * 0.5, true),
                    // south wall (z+)
                    (base_x + i_f, base_z + (ROOM_TILES - 1) as f32 * TILE + TILE * 0.5, true),
                    // west wall (x-)
                    (base_x - TILE * 0.5, base_z + i_f, false),
                    // east wall (x+)
                    (base_x + (ROOM_TILES - 1) as f32 * TILE + TILE * 0.5, base_z + i_f, false),
                ] {
                    // Leave doorways at center of each wall
                    let is_door = i == ROOM_TILES / 2;
                    if is_door { continue; }
                    commands.spawn((
                        Mesh3d(wall_mesh.clone()),
                        MeshMaterial3d(wall_mat.clone()),
                        Transform::from_xyz(wx, 0.5, wz).with_rotation(if is_z_axis {
                            Quat::IDENTITY
                        } else {
                            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                        }),
                        DungeonWall,
                        RoomEntity,
                    ));
                }
            }

            // Spawn enemies in non-entrance rooms
            if !is_entrance {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let count = rng.gen_range(1..=3);
                let tier_mult = instance.tier as f32;
                for _ in 0..count {
                    let tx = rng.gen_range(1..ROOM_TILES - 1);
                    let tz = rng.gen_range(1..ROOM_TILES - 1);
                    let is_ranged = rng.gen_bool(0.3);
                    let (mat, variant) = if is_ranged {
                        (enemy_mat_ranged.clone(), ir_core::EnemyVariant::Ranged)
                    } else {
                        (enemy_mat_grunt.clone(), ir_core::EnemyVariant::Grunt)
                    };
                    commands.spawn((
                        Mesh3d(enemy_mesh.clone()),
                        MeshMaterial3d(mat),
                        Transform::from_xyz(
                            base_x + tx as f32 * TILE,
                            0.0,
                            base_z + tz as f32 * TILE,
                        ),
                        ir_core::Enemy {
                            variant,
                            tier: instance.tier,
                            xp_reward: (10.0 * tier_mult) as u64,
                        },
                        ir_core::Health::new(30.0 * tier_mult),
                        ir_core::CombatStats {
                            move_speed: 3.5 + tier_mult * 0.3,
                            damage_bonus: tier_mult * 2.0,
                            ..default()
                        },
                        ir_core::Velocity(Vec3::ZERO),
                        ir_core::Team::Enemy,
                        ir_core::AttackCooldown::default(),
                        RoomEntity,
                    ));
                }
            }
        }
    }

    info!("Dungeon ready with enemies.");
}

/// Exits dungeon when player reaches the exit marker.
pub fn check_dungeon_exit(
    player_query: Query<&Transform, With<Player>>,
    exits: Query<&Transform, (With<DungeonExit>, Without<Player>)>,
    mut dungeon_state: ResMut<DungeonState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let cleared = exits.iter().any(|exit_tf| {
        player_pos.distance(exit_tf.translation) < 1.5
    });

    if cleared {
        info!("Dungeon cleared! Returning to world.");
        dungeon_state.current = None;
        next_state.set(AppState::World);
    }
}
