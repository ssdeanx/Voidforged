//! Procedural dungeon generation with actual corridors, room templates,
//! boss encounters, miniboss rooms, treasure rooms, and depth-based scaling.

use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

// ============================================================================
// Marker Components
// ============================================================================

/// Marker for dungeon floor tiles.
#[derive(Component)]
pub struct DungeonFloor;

/// Marker for dungeon walls.
#[derive(Component)]
pub struct DungeonWall;

/// Marker for the dungeon exit.
#[derive(Component)]
pub struct DungeonExit;

/// Marker for any entity spawned inside the dungeon — used for cleanup on exit.
#[derive(Component)]
pub struct DungeonEntity;

/// Marker for the boss enemy entity — used to check if the boss is alive.
#[derive(Component)]
pub struct BossMarker;

/// Marker for boss arena exit — exit only activates when boss is defeated.
#[derive(Component)]
pub struct BossArenaExit;

/// Marker for the loot chest in miniboss rooms.
#[derive(Component)]
pub struct LootChest;

/// Marker for treasure rooms — green-tinted floor tiles, no enemies.
#[derive(Component)]
pub struct TreasureRoomMarker;

/// Marker for decorative pillars in boss arena and pillar rooms.
#[derive(Component)]
pub struct PillarEntity;

// ============================================================================
// Constants
// ============================================================================

const TILE: f32 = 2.0;
const MIN_ROOMS: u32 = 2; // entrance + boss

// ============================================================================
// Room Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RoomKind {
    Entrance,
    Combat,
    Miniboss,
    Treasure,
    Boss,
}

// ============================================================================
// Room Template Definitions
// ============================================================================

/// A room template defines floor tiles, pillar positions, and interior walls.
pub(crate) struct RoomLayout {
    /// Relative tile positions (tx, tz) within the room's bounding box.
    pub(crate) tiles: Vec<(i32, i32)>,
    pub(crate) width: i32,
    pub(crate) height: i32,
    /// Pillar positions relative to room origin.
    pillars: Vec<(i32, i32)>,
    /// Interior wall segments: ((tx, tz), is_z_axis) where tx/tz is the tile
    /// coordinate at which the wall center is placed. is_z_axis=true means the
    /// wall's long axis runs along Z (matching north/south room walls).
    interior_walls: Vec<((i32, i32), bool)>,
}

fn template_square() -> RoomLayout {
    let mut tiles = Vec::new();
    for tx in 0..5 {
        for tz in 0..5 {
            tiles.push((tx, tz));
        }
    }
    RoomLayout {
        tiles,
        width: 5,
        height: 5,
        pillars: Vec::new(),
        interior_walls: Vec::new(),
    }
}

fn template_pillars() -> RoomLayout {
    let mut tiles = Vec::new();
    for tx in 0..5 {
        for tz in 0..5 {
            tiles.push((tx, tz));
        }
    }
    RoomLayout {
        tiles,
        width: 5,
        height: 5,
        pillars: vec![(1, 1), (1, 3), (3, 1), (3, 3)],
        interior_walls: Vec::new(),
    }
}

/// L-Shape (7×7 bounding box): tiles where x >= 3 || z >= 3, creating an L
/// in the bottom-right of the bounding box. All door centers (x=3 on north/south
/// edges, z=3 on east/west edges) have tiles behind them.
fn template_lshape() -> RoomLayout {
    let mut tiles = Vec::new();
    for tx in 0..7 {
        for tz in 0..7 {
            if tx >= 3 || tz >= 3 {
                tiles.push((tx, tz));
            }
        }
    }
    RoomLayout {
        tiles,
        width: 7,
        height: 7,
        pillars: Vec::new(),
        interior_walls: Vec::new(),
    }
}

/// Split room (6×5): two halves divided by an interior wall with a gap at z=2.
fn template_split() -> RoomLayout {
    let mut tiles = Vec::new();
    for tx in 0..6 {
        for tz in 0..5 {
            tiles.push((tx, tz));
        }
    }
    // Interior wall along x=3 (between columns 2 and 3), z=0..4, gap at z=2
    let mut interior_walls = Vec::new();
    for iz in 0..5 {
        if iz != 2 {
            // Wall at edge between tile (2, iz) and (3, iz): placed at x=3, z=iz
            // Runs along Z axis (north-south) → is_z_axis = true
            interior_walls.push(((3, iz), true));
        }
    }
    RoomLayout {
        tiles,
        width: 6,
        height: 5,
        pillars: Vec::new(),
        interior_walls,
    }
}

/// Arena (7×7): large open room with decorative corner pillars.
fn template_arena() -> RoomLayout {
    let mut tiles = Vec::new();
    for tx in 0..7 {
        for tz in 0..7 {
            tiles.push((tx, tz));
        }
    }
    RoomLayout {
        tiles,
        width: 7,
        height: 7,
        pillars: vec![(1, 1), (1, 5), (5, 1), (5, 5)],
        interior_walls: Vec::new(),
    }
}

/// Select a room template weighted by depth and room kind.
pub(crate) fn pick_template(depth: u32, kind: RoomKind, rng: &mut impl Rng) -> RoomLayout {
    match kind {
        RoomKind::Boss => template_arena(),
        RoomKind::Miniboss => {
            // Miniboss rooms use pillars or square, weighted by depth
            if depth >= 4 && rng.gen_bool(0.5) {
                template_pillars()
            } else {
                template_square()
            }
        }
        RoomKind::Treasure => {
            // Treasure rooms are always square 5×5
            template_square()
        }
        RoomKind::Entrance => template_square(),
        RoomKind::Combat => {
            // Weight by depth: deeper = more complex templates
            let roll = rng.gen_range(0.0..1.0);
            if depth <= 3 {
                // Shallow: mostly square and pillars
                if roll < 0.6 {
                    template_square()
                } else if roll < 0.85 {
                    template_pillars()
                } else {
                    template_split()
                }
            } else if depth <= 6 {
                // Mid: mix of all templates
                if roll < 0.25 {
                    template_square()
                } else if roll < 0.5 {
                    template_pillars()
                } else if roll < 0.7 {
                    template_lshape()
                } else if roll < 0.85 {
                    template_split()
                } else {
                    template_arena()
                }
            } else {
                // Deep: more complex rooms
                if roll < 0.15 {
                    template_square()
                } else if roll < 0.35 {
                    template_pillars()
                } else if roll < 0.55 {
                    template_lshape()
                } else if roll < 0.75 {
                    template_split()
                } else {
                    template_arena()
                }
            }
        }
    }
}

// ============================================================================
// Grid & Room Assignment
// ============================================================================

/// Determine the grid size and which rooms to generate based on dungeon depth.
/// Returns (grid_size, Vec of (rx, rz) positions in fill order).
fn build_room_grid(depth: u32) -> (i32, Vec<(i32, i32)>) {
    let grid_size = if depth <= 5 {
        3i32
    } else if depth <= 9 {
        4i32
    } else {
        5i32
    };

    let total = depth as usize;
    if total == 0 {
        return (grid_size, vec![(0, 0)]);
    }

    // Generate all room positions in the grid reading order.
    let mut positions: Vec<(i32, i32)> = Vec::new();
    for rz in 0..grid_size {
        for rx in 0..grid_size {
            positions.push((rx, rz));
        }
    }

    // We MUST have: entrance at (0,0), boss at (grid_size-1, grid_size-1),
    // miniboss at (grid_size-1, 0) and (0, grid_size-1) when depth is sufficient.
    // Reorder the fill list to guarantee these.
    let entrance = (0, 0);
    let boss = (grid_size - 1, grid_size - 1);
    let miniboss_a = (grid_size - 1, 0);
    let miniboss_b = (0, grid_size - 1);

    // Build fill list guaranteeing required rooms
    let mut fill: Vec<(i32, i32)> = Vec::new();

    // 1. Entrance always first
    fill.push(entrance);

    if total >= 2 {
        // 2. Boss always last assigned room
        // We'll add it at the end
    }

    // 3. Miniboss rooms if we have enough rooms
    if total >= 3 {
        fill.push(miniboss_a);
    }
    if total >= 4 {
        fill.push(miniboss_b);
    }

    // 4. Fill remaining with other positions from reading order
    // that are not entrance, boss, miniboss_a, or miniboss_b
    let special = [entrance, boss, miniboss_a, miniboss_b];
    for &pos in &positions {
        if special.contains(&pos) {
            continue;
        }
        if fill.len() >= total {
            break;
        }
        fill.push(pos);
    }

    // 5. Add boss at the end if depth >= 2
    if total >= 2 {
        // Insert boss at the last position
        let flen = fill.len();
        if flen >= total {
            // Replace last element with boss, displace the old last
            // to just before boss
            let last = fill[flen - 1];
            fill[flen - 1] = boss;
            let insert_at = fill.len().saturating_sub(1);
            fill.insert(insert_at, last);
        } else {
            fill.push(boss);
        }
    }

    // Trim to exact count
    fill.truncate(total);

    // Handle edge case: if boss wasn't placed (shouldn't happen with total >= 2)
    if total >= 2 && !fill.contains(&boss) {
        fill.pop();
        fill.push(boss);
    }

    (grid_size, fill)
}

/// Assign room kind based on position and total room settings.
fn assign_room_kind(pos: (i32, i32), grid_size: i32, total_rooms: usize, room_index: usize, depth: u32, rng: &mut impl Rng) -> RoomKind {
    let boss = (grid_size - 1, grid_size - 1);
    let entrance = (0, 0);
    let miniboss_a = (grid_size - 1, 0);
    let miniboss_b = (0, grid_size - 1);

    if pos == entrance {
        return RoomKind::Entrance;
    }
    if pos == boss && room_index == total_rooms - 1 {
        return RoomKind::Boss;
    }
    // Miniboss rooms: at grid corners (far from entrance)
    if total_rooms >= 3 && pos == miniboss_a {
        return RoomKind::Miniboss;
    }
    if total_rooms >= 4 && pos == miniboss_b {
        return RoomKind::Miniboss;
    }

    // Treasure room: 20% chance for remaining rooms (non-entrance, non-boss, non-miniboss)
    if depth >= 4 && rng.gen_range(0.0..1.0) < 0.20 {
        return RoomKind::Treasure;
    }

    RoomKind::Combat
}

// ============================================================================
// Wall Generation Helpers
// ============================================================================

/// Returns true if (tx, tz) is a valid door position on this room's perimeter.
fn is_door_position(tx: i32, tz: i32, width: i32, height: i32) -> bool {
    let cx = width / 2;
    let cz = height / 2;
    // Door at center of each wall:
    // North: (cx, 0)
    // South: (cx, height - 1)
    // West:  (0, cz)
    // East:  (width - 1, cz)
    (tz == 0 && tx == cx)
        || (tz == height - 1 && tx == cx)
        || (tx == 0 && tz == cz)
        || (tx == width - 1 && tz == cz)
}

/// Check if a tile exists in the set. Used for perimeter wall detection.
fn has_tile(tiles: &[(i32, i32)], tx: i32, tz: i32) -> bool {
    tiles.contains(&(tx, tz))
}

/// Spawn perimeter walls for a room, skipping door positions.
fn spawn_perimeter_walls(
    commands: &mut Commands,
    wall_mesh: Handle<Mesh>,
    wall_mat: Handle<StandardMaterial>,
    base_x: f32,
    base_z: f32,
    layout: &RoomLayout,
) {
    let tile_set: Vec<(i32, i32)> = layout.tiles.clone();
    let width = layout.width;
    let height = layout.height;

    for &(tx, tz) in &layout.tiles {
        // North edge (z-)
        if !has_tile(&tile_set, tx, tz - 1) && !is_door_position(tx, tz, width, height) {
            let wx = base_x + tx as f32 * TILE;
            let wz = base_z + tz as f32 * TILE - TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wx, 0.5, wz).with_rotation(Quat::IDENTITY),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
        // South edge (z+)
        if !has_tile(&tile_set, tx, tz + 1) && !is_door_position(tx, tz, width, height) {
            let wx = base_x + tx as f32 * TILE;
            let wz = base_z + tz as f32 * TILE + TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wx, 0.5, wz).with_rotation(Quat::IDENTITY),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
        // West edge (x-)
        if !has_tile(&tile_set, tx - 1, tz) && !is_door_position(tx, tz, width, height) {
            let wx = base_x + tx as f32 * TILE - TILE * 0.5;
            let wz = base_z + tz as f32 * TILE;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wx, 0.5, wz)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
        // East edge (x+)
        if !has_tile(&tile_set, tx + 1, tz) && !is_door_position(tx, tz, width, height) {
            let wx = base_x + tx as f32 * TILE + TILE * 0.5;
            let wz = base_z + tz as f32 * TILE;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wx, 0.5, wz)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
    }
}

// ============================================================================
// Corridor Generation
// ============================================================================

/// Generate corridor geometry between two adjacent rooms.
/// The corridor runs along the X axis (east-west) or Z axis (north-south)
/// connecting the door positions of two rooms.
#[allow(clippy::too_many_arguments)]
fn generate_corridor(
    commands: &mut Commands,
    floor_mesh: Handle<Mesh>,
    floor_mat: Handle<StandardMaterial>,
    wall_mesh: Handle<Mesh>,
    wall_mat: Handle<StandardMaterial>,
    room_a: (i32, i32),
    room_b: (i32, i32),
    room_step: i32,
    door_z_room_a: f32,
    door_x_room_a: f32,
) {
    let corridor_floor_mat = floor_mat; // slightly darker would be nice but we use the same for now
    let base_a_x = room_a.0 as f32 * room_step as f32 * TILE;
    let base_a_z = room_a.1 as f32 * room_step as f32 * TILE;
    let base_b_x = room_b.0 as f32 * room_step as f32 * TILE;
    let base_b_z = room_b.1 as f32 * room_step as f32 * TILE;

    // Determine corridor direction
    let dx = room_b.0 - room_a.0;
    let dz = room_b.1 - room_a.1;

    if dx != 0 {
        // East-west corridor
        let corr_z = base_a_z + door_z_room_a;
        // Starting x (east edge of room a's east door) to ending x (west edge of room b's west door)
        let start_x = base_a_x + (room_step - 1) as f32 * TILE;
        let end_x = base_b_x;
        // Fill corridor floor tiles along x between start and end
        let num_tiles = (end_x - start_x) / TILE;
        for i in 0..num_tiles as i32 {
            let cx = start_x + i as f32 * TILE + TILE * 0.5;
            // Corridor floor
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(corridor_floor_mat.clone()),
                Transform::from_xyz(cx, -0.5, corr_z),
                DungeonFloor,
                RoomEntity,
                DungeonEntity,
            ));

            // South corridor wall (z-) — is_z_axis=true for default orientation
            // Wait: corridor runs east-west, so the walls run east-west (along X).
            // Wall mesh extends in Z at default rotation. We want wall length
            // along X, so we need to rotate (is_z_axis=false → east/west wall style).
            let wall_z_south = corr_z - TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(cx, 0.5, wall_z_south)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));

            // North corridor wall (z+)
            let wall_z_north = corr_z + TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(cx, 0.5, wall_z_north)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
    } else if dz != 0 {
        // North-south corridor
        let corr_x = base_a_x + door_x_room_a;
        let start_z = base_a_z + (room_step - 1) as f32 * TILE;
        let end_z = base_b_z;

        let num_tiles = (end_z - start_z) / TILE;
        for i in 0..num_tiles as i32 {
            let cz = start_z + i as f32 * TILE + TILE * 0.5;
            // Corridor floor
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(corridor_floor_mat.clone()),
                Transform::from_xyz(corr_x, -0.5, cz),
                DungeonFloor,
                RoomEntity,
                DungeonEntity,
            ));

            // West corridor wall (x-)
            let wall_x_west = corr_x - TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wall_x_west, 0.5, cz)
                    .with_rotation(Quat::IDENTITY),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));

            // East corridor wall (x+)
            let wall_x_east = corr_x + TILE * 0.5;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wall_x_east, 0.5, cz)
                    .with_rotation(Quat::IDENTITY),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }
    }
}

// ============================================================================
// Main Dungeon Generation
// ============================================================================

/// Generates dungeon rooms, corridors, and exit when entering a dungeon.
#[allow(clippy::too_many_arguments)]
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

    let depth = instance.depth.max(MIN_ROOMS);
    info!(
        "Dungeon: {} (tier {}, depth {} rooms)",
        instance.name, instance.tier, depth
    );

    // ── Meshes and materials ──────────────────────────────────────────
    let floor_mesh = meshes.add(Cuboid::new(TILE * 0.9, 0.05, TILE * 0.9));
    let floor_mat = materials.add(Color::srgb(0.15, 0.12, 0.1));
    let corridor_mat = materials.add(Color::srgb(0.12, 0.10, 0.08)); // slightly darker
    let treasure_floor_mat = materials.add(Color::srgb(0.1, 0.18, 0.1)); // green-tinted
    let exit_mat = materials.add(Color::srgb(0.0, 0.5, 0.0));
    let wall_mesh = meshes.add(Cuboid::new(0.3, 2.0, TILE * 0.9));
    let wall_mat = materials.add(Color::srgb(0.3, 0.25, 0.2));
    let pillar_mesh = meshes.add(Cuboid::new(0.6, 1.5, 0.6));
    let pillar_mat = materials.add(Color::srgb(0.35, 0.3, 0.25));
    let chest_mesh = meshes.add(Cuboid::new(0.8, 0.6, 0.8));
    let chest_mat = materials.add(Color::srgb(0.9, 0.75, 0.1)); // gold

    // Enemy meshes/materials
    let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.8, 0.5));
    let enemy_mat_grunt = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let enemy_mat_ranged = materials.add(Color::srgb(0.9, 0.5, 0.1));
    let enemy_mat_elite = materials.add(Color::srgb(0.6, 0.1, 0.8));
    let enemy_mat_boss = materials.add(Color::srgb(1.0, 0.15, 0.15));

    // Health pickup mesh for treasure rooms
    let health_pickup_mesh = meshes.add(Cuboid::new(0.4, 0.4, 0.4));
    let health_pickup_mat = materials.add(Color::srgb(0.0, 1.0, 0.15));

    // ── Build room grid ───────────────────────────────────────────────
    let (grid_size, room_positions) = build_room_grid(depth);
    let total_rooms = room_positions.len();
    let mut rng = rand::thread_rng();

    // Calculate room_step based on the largest template we might use.
    // We use a generous room_step to accommodate all templates.
    // For templates up to 7×7, room_step=7+2=9 is safe.
    let room_step = 9i32;

    // Store room base positions and layouts for corridor generation
    struct RoomBuildInfo {
        pos: (i32, i32),
        kind: RoomKind,
        layout: RoomLayout,
        base_x: f32,
        base_z: f32,
        door_center_x: f32, // relative to base
        door_center_z: f32, // relative to base
        boss_entity: Option<Entity>,
    }

    let mut room_infos: Vec<RoomBuildInfo> = Vec::new();

    // ── First pass: build room info ───────────────────────────────────
    for (idx, &(rx, rz)) in room_positions.iter().enumerate() {
        let kind = assign_room_kind((rx, rz), grid_size, total_rooms, idx, depth, &mut rng);
        let layout = pick_template(depth, kind, &mut rng);

        let base_x = rx as f32 * room_step as f32 * TILE;
        let base_z = rz as f32 * room_step as f32 * TILE;

        // Door centers (relative to room base)
        let door_center_x = (layout.width / 2) as f32 * TILE;
        let door_center_z = (layout.height / 2) as f32 * TILE;

        room_infos.push(RoomBuildInfo {
            pos: (rx, rz),
            kind,
            layout,
            base_x,
            base_z,
            door_center_x,
            door_center_z,
            boss_entity: None,
        });
    }

    // ── Second pass: spawn geometry and entities ──────────────────────
    for (_idx, info) in room_infos.iter_mut().enumerate() {
        let (_rx, _rz) = info.pos;
        let base_x = info.base_x;
        let base_z = info.base_z;

        // Determine floor material based on room kind
        let room_floor_mat = match info.kind {
            RoomKind::Treasure => treasure_floor_mat.clone(),
            _ => floor_mat.clone(),
        };

        // ── Floor tiles ──────────────────────────────────────────────────
        for &(tx, tz) in &info.layout.tiles {
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(room_floor_mat.clone()),
                Transform::from_xyz(
                    base_x + tx as f32 * TILE,
                    -0.5,
                    base_z + tz as f32 * TILE,
                ),
                DungeonFloor,
                RoomEntity,
                DungeonEntity,
            ));
        }

        // ── Perimeter walls ──────────────────────────────────────────────
        spawn_perimeter_walls(
            &mut commands,
            wall_mesh.clone(),
            wall_mat.clone(),
            base_x,
            base_z,
            &info.layout,
        );

        // ── Interior walls (e.g., Split room divider) ────────────────────
        for &((wx_tile, wz_tile), is_z_axis) in &info.layout.interior_walls {
            let wall_wx = base_x + wx_tile as f32 * TILE;
            let wall_wz = base_z + wz_tile as f32 * TILE;
            commands.spawn((
                Mesh3d(wall_mesh.clone()),
                MeshMaterial3d(wall_mat.clone()),
                Transform::from_xyz(wall_wx, 0.5, wall_wz).with_rotation(if is_z_axis {
                    Quat::IDENTITY
                } else {
                    Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                }),
                DungeonWall,
                RoomEntity,
                DungeonEntity,
            ));
        }

        // ── Pillars (decorative or cover) ────────────────────────────────
        for &(ptx, ptz) in &info.layout.pillars {
            commands.spawn((
                Mesh3d(pillar_mesh.clone()),
                MeshMaterial3d(pillar_mat.clone()),
                Transform::from_xyz(
                    base_x + ptx as f32 * TILE,
                    0.0,
                    base_z + ptz as f32 * TILE,
                ),
                PillarEntity,
                RoomEntity,
                DungeonEntity,
            ));
        }

        // ── Boss room: exit + boss enemy ─────────────────────────────────
        if info.kind == RoomKind::Boss {
            // Boss arena exit tile
            let center_x = (info.layout.width / 2) as f32 * TILE;
            let center_z = (info.layout.height / 2) as f32 * TILE;
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(exit_mat.clone()),
                Transform::from_xyz(base_x + center_x, -0.45, base_z + center_z),
                DungeonExit,
                BossArenaExit, // marker: only activates when boss is dead
                RoomEntity,
                DungeonEntity,
            ));

            // Boss enemy in center
            let tier_mult = instance.tier as f32;
            let boss_entity = commands.spawn((
                Mesh3d(enemy_mesh.clone()),
                MeshMaterial3d(enemy_mat_boss.clone()),
                Transform::from_xyz(base_x + center_x, 0.0, base_z + center_z),
                Enemy {
                    variant: EnemyVariant::Boss,
                    tier: instance.tier,
                    xp_reward: (200.0 * tier_mult) as u64,
                },
                Health::new(150.0 * tier_mult),
                CombatStats {
                    move_speed: 3.0 + tier_mult * 0.2,
                    damage_bonus: tier_mult * 3.0,
                    armor: 10.0 + tier_mult * 2.0,
                    ..default()
                },
                Velocity(Vec3::ZERO),
                Team::Enemy,
                AttackCooldown::default(),
                BossMarker,
                RoomEntity,
                DungeonEntity,
            )).id();
            info.boss_entity = Some(boss_entity);
        }

        // ── Miniboss room: elite enemy + loot chest ──────────────────────
        if info.kind == RoomKind::Miniboss {
            let tier_mult = instance.tier as f32;
            let cx = (info.layout.width / 2) as f32 * TILE;
            let cz = (info.layout.height / 2) as f32 * TILE;

            // Elite enemy in center-left area
            commands.spawn((
                Mesh3d(enemy_mesh.clone()),
                MeshMaterial3d(enemy_mat_elite.clone()),
                Transform::from_xyz(base_x + cx - TILE, 0.0, base_z + cz),
                Enemy {
                    variant: EnemyVariant::Elite,
                    tier: instance.tier,
                    xp_reward: (50.0 * tier_mult) as u64,
                },
                Health::new(80.0 * tier_mult),
                CombatStats {
                    move_speed: 4.0 + tier_mult * 0.3,
                    damage_bonus: tier_mult * 4.0,
                    armor: 5.0 + tier_mult * 1.5,
                    ..default()
                },
                Velocity(Vec3::ZERO),
                Team::Enemy,
                AttackCooldown::default(),
                RoomEntity,
                DungeonEntity,
            ));

            // Loot chest in center-right area
            commands.spawn((
                Mesh3d(chest_mesh.clone()),
                MeshMaterial3d(chest_mat.clone()),
                Transform::from_xyz(base_x + cx + TILE, 0.0, base_z + cz),
                LootChest,
                RoomEntity,
                DungeonEntity,
            ));
        }

        // ── Treasure room: loot items, no enemies ────────────────────────
        if info.kind == RoomKind::Treasure {
            let cx = (info.layout.width / 2) as f32 * TILE;
            let cz = (info.layout.height / 2) as f32 * TILE;
            let count = rng.gen_range(2..=3);
            for i in 0..count {
                let offset_x = (i as f32 - (count as f32 - 1.0) / 2.0) * TILE;
                let offset_z = 0.0;
                commands.spawn((
                    Mesh3d(health_pickup_mesh.clone()),
                    MeshMaterial3d(health_pickup_mat.clone()),
                    Transform::from_xyz(
                        base_x + cx + offset_x,
                        0.0,
                        base_z + cz + offset_z,
                    ),
                    Pickup { kind: PickupKind::Health },
                    RoomEntity,
                    DungeonEntity,
                ));
            }
        }

        // ── Combat enemies (non-entrance, non-boss, non-treasure) ────────
        let spawn_enemies = match info.kind {
            RoomKind::Entrance | RoomKind::Boss | RoomKind::Treasure => false,
            _ => true,
        };
        if spawn_enemies {
            let tier_mult = instance.tier as f32;
            let enemy_count = match info.kind {
                RoomKind::Miniboss => 0, // Elite spawned above
                RoomKind::Combat => {
                    let layout_area = info.layout.tiles.len();
                    if layout_area > 30 {
                        // Arena-sized
                        rng.gen_range(3..=5)
                    } else if layout_area > 20 {
                        // Medium (L-Shape, Split)
                        rng.gen_range(2..=4)
                    } else {
                        // Standard (Square, Pillars)
                        rng.gen_range(1..=3)
                    }
                }
                _ => 0,
            };
            for _ in 0..enemy_count {
                // Pick a random floor tile that's not at the very edge
                let tile = loop {
                    let candidate = info.layout.tiles[rng.gen_range(0..info.layout.tiles.len())];
                    let (tx, tz) = candidate;
                    // Avoid spawning right on door positions
                    if is_door_position(tx, tz, info.layout.width, info.layout.height) {
                        continue;
                    }
                    break candidate;
                };
                let is_ranged = rng.gen_bool(0.3);
                let (mat, variant) = if is_ranged {
                    (enemy_mat_ranged.clone(), EnemyVariant::Ranged)
                } else {
                    (enemy_mat_grunt.clone(), EnemyVariant::Grunt)
                };
                commands.spawn((
                    Mesh3d(enemy_mesh.clone()),
                    MeshMaterial3d(mat),
                    Transform::from_xyz(
                        base_x + tile.0 as f32 * TILE,
                        0.0,
                        base_z + tile.1 as f32 * TILE,
                    ),
                    Enemy {
                        variant,
                        tier: instance.tier,
                        xp_reward: (10.0 * tier_mult) as u64,
                    },
                    Health::new(30.0 * tier_mult),
                    CombatStats {
                        move_speed: 3.5 + tier_mult * 0.3,
                        damage_bonus: tier_mult * 2.0,
                        ..default()
                    },
                    Velocity(Vec3::ZERO),
                    Team::Enemy,
                    AttackCooldown::default(),
                    RoomEntity,
                    DungeonEntity,
                ));
            }
        }
    }

    // ── Corridors between adjacent rooms ──────────────────────────────
    for i in 0..room_infos.len() {
        let info_a = &room_infos[i];
        // Connect to right neighbor (rx+1, rz)
        let right_pos = (info_a.pos.0 + 1, info_a.pos.1);
        if let Some(info_b) = room_infos.iter().find(|ri| ri.pos == right_pos) {
            generate_corridor(
                &mut commands,
                floor_mesh.clone(),
                corridor_mat.clone(),
                wall_mesh.clone(),
                wall_mat.clone(),
                info_a.pos,
                info_b.pos,
                room_step,
                info_a.door_center_z,
                info_a.door_center_x,
            );
        }
        // Connect to bottom neighbor (rx, rz+1)
        let bottom_pos = (info_a.pos.0, info_a.pos.1 + 1);
        if let Some(info_b) = room_infos.iter().find(|ri| ri.pos == bottom_pos) {
            generate_corridor(
                &mut commands,
                floor_mesh.clone(),
                corridor_mat.clone(),
                wall_mesh.clone(),
                wall_mat.clone(),
                info_a.pos,
                info_b.pos,
                room_step,
                info_a.door_center_z,
                info_a.door_center_x,
            );
        }
    }

    info!("Dungeon ready with {} rooms.", total_rooms);
}

// ============================================================================
// Dungeon Exit Check
// ============================================================================

/// Exits dungeon when player reaches the exit marker.
/// For boss rooms, the exit only activates when the boss is dead.
pub fn check_dungeon_exit(
    player_query: Query<&Transform, With<Player>>,
    exits: Query<(Entity, &Transform), With<DungeonExit>>,
    _boss_exits: Query<&Transform, (With<BossArenaExit>, With<DungeonExit>)>,
    boss_query: Query<(), (With<BossMarker>, With<Enemy>)>,
    mut dungeon_state: ResMut<DungeonState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    // Check if any boss is still alive — if so, block ALL exits
    let boss_alive = !boss_query.is_empty();
    if boss_alive {
        return;
    }

    // All exits now usable
    let cleared = exits.iter().any(|(_, exit_tf)| {
        player_pos.distance(exit_tf.translation) < 1.5
    });

    if cleared {
        info!("Dungeon cleared! Returning to world.");
        dungeon_state.current = None;
        next_state.set(AppState::World);
    }
}
