//! Map generation — creates world tiles, zone markers, dungeon entrances.

use bevy::prelude::*;
use ir_core::*;
use crate::zone::*;

/// Marker for world map tiles.
#[derive(Component)]
pub struct WorldTile;

/// Marker for the dungeon entrance visual.
#[derive(Component)]
pub struct EntranceMarker;

/// Player's current zone (None = between zones / in dungeon).
#[derive(Resource, Default)]
pub struct CurrentZone(pub Option<ZoneId>);

/// Generates all zone tiles and dungeon entrances in the world.
pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_mesh = meshes.add(Cuboid::new(1.9, 0.05, 1.9));
    let entrance_icon = meshes.add(Cuboid::new(0.6, 0.3, 0.6));
    let entrance_mat = materials.add(Color::srgb(0.8, 0.2, 0.8));

    for zone in all_zones() {
        let col_a = zone.id.ground_color();
        let col_b = zone.id.ground_color_alt();
        let mat_a = materials.add(col_a);
        let mat_b = materials.add(col_b);
        let base_x = zone.offset_x as f32 * 2.0;
        let base_z = zone.offset_z as f32 * 2.0;

        for tx in 0..zone.tile_w {
            for tz in 0..zone.tile_h {
                let alt = (tx + tz) % 2 == 0;
                let mat = if alt { mat_a.clone() } else { mat_b.clone() };
                commands.spawn((
                    Mesh3d(tile_mesh.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_xyz(
                        base_x + tx as f32 * 2.0,
                        -0.5,
                        base_z + tz as f32 * 2.0,
                    ),
                    WorldTile,
                    RoomEntity,
                ));
            }
        }

        // Spawn environment decorations (rocks, bushes)
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let decor_count = rng.gen_range(8..=15);
            let rock_mesh = meshes.add(Cuboid::new(0.3, 0.15, 0.3));
            let bush_mesh = meshes.add(Cuboid::new(0.5, 0.4, 0.5));
            let grass_mesh = meshes.add(Cuboid::new(0.15, 0.2, 0.15));
            for _ in 0..decor_count {
                let tx = rng.gen_range(2..zone.tile_w - 2);
                let tz = rng.gen_range(2..zone.tile_h - 2);
                let (mesh, color) = match rng.gen_range(0..3) {
                    0 => (rock_mesh.clone(), Color::srgb(0.35, 0.3, 0.25)),
                    1 => (bush_mesh.clone(), Color::srgb(0.14, 0.35, 0.1)),
                    _ => (grass_mesh.clone(), Color::srgb(0.1, 0.25, 0.07)),
                };
                let mat = materials.add(color);
                let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.4..0.4);
                let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.4..0.4);
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(mat),
                    Transform::from_xyz(wx, -0.35, wz),
                    RoomEntity,
                ));
            }
        }

        // Spawn dungeon entrance markers
        for (ex, ez, entrance) in &zone.dungeon_entrances {
            let wx = base_x + *ex as f32 * 2.0;
            let wz = base_z + *ez as f32 * 2.0;
            commands.spawn((
                Mesh3d(entrance_icon.clone()),
                MeshMaterial3d(entrance_mat.clone()),
                Transform::from_xyz(wx, 0.0, wz),
                EntranceMarker,
                entrance.clone(),
                RoomEntity,
            ));
        }

        // Spawn wandering enemies in this zone
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let count = rng.gen_range(3..=6);
            let colors = [
                Color::srgb(0.6, 0.15, 0.15),
                Color::srgb(0.5, 0.1, 0.1),
                Color::srgb(0.7, 0.2, 0.1),
            ];
            let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.7, 0.5));
            for _ in 0..count {
                let tx = rng.gen_range(1..zone.tile_w - 1);
                let tz = rng.gen_range(1..zone.tile_h - 1);
                let mat = materials.add(colors[rng.gen_range(0..colors.len())]);
                let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.3..0.3);
                let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.3..0.3);
                commands.spawn((
                    Mesh3d(enemy_mesh.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_xyz(wx, 0.0, wz),
                    ir_core::Enemy {
                        variant: ir_core::EnemyVariant::Grunt,
                        tier: 0,
                        xp_reward: 5,
                    },
                    ir_core::Health::new(15.0),
                    ir_core::CombatStats {
                        move_speed: 2.0,
                        damage_bonus: 0.0,
                        ..default()
                    },
                    ir_core::Velocity(Vec3::ZERO),
                    ir_core::Team::Enemy,
                    ir_core::AttackCooldown::default(),
                    RoomEntity,
                    crate::zone::WorldPos(tx as i32, tz as i32),
                ));
            }
        }
    }
}

/// Tracks which zone the player is currently in, updates every frame.
pub fn track_player_zone(
    player_query: Query<&Transform, With<ir_core::Player>>,
    mut current_zone: ResMut<CurrentZone>,
) {
    let pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for zone in all_zones() {
        let x_min = zone.offset_x as f32 * 2.0;
        let x_max = x_min + zone.tile_w as f32 * 2.0;
        let z_min = zone.offset_z as f32 * 2.0;
        let z_max = z_min + zone.tile_h as f32 * 2.0;
        if pos.x >= x_min && pos.x < x_max && pos.z >= z_min && pos.z < z_max {
            current_zone.0 = Some(zone.id.clone());
            return;
        }
    }
    current_zone.0 = None;
}
