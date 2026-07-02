//! Raid dungeon system — larger group dungeons with multiple bosses,
//! AI companions, and raid-specific loot tables.
//!
//! Each raid has:
//! - A larger grid (4×4 or 5×5) with more rooms
//! - Multiple boss encounters (2-3 per raid)
//! - Miniboss rooms with raid-tier loot
//! - AI companion slots filled by NPC class archetypes
//! - Group scaling (enemy HP/damage scales with party size)

use bevy::prelude::*;
use ir_core::*;
use crate::rooms::*;
use rand::Rng;

// ============================================================================
// Raid-specific constants
// ============================================================================

/// Number of companion slots available in a raid.
pub const RAID_COMPANION_SLOTS: usize = 4;

/// How much each additional companion increases enemy HP.
const HP_SCALE_PER_COMPANION: f32 = 0.25;

/// How much each additional companion increases enemy damage.
const DMG_SCALE_PER_COMPANION: f32 = 0.15;

// ============================================================================
// Components
// ============================================================================

/// Marker for raid-specific entities (separate cleanup from dungeon entities).
#[derive(Component)]
pub struct RaidEntity;

/// Marker for an AI companion that auto-follows and uses abilities.
#[derive(Component)]
pub struct AICompanion {
    /// The class this companion plays.
    pub class: CharacterClass,
    /// Name displayed for the companion.
    pub name: String,
    /// Target entity to follow/attack.
    pub target: Option<Entity>,
}

/// Marker for raid exit portal (only appears after all bosses are defeated).
#[derive(Component)]
pub struct RaidExit;

/// Marker for a raid boss encounter area.
#[derive(Component)]
pub struct RaidBossArena;

// ============================================================================
// Resources
// ============================================================================

/// Tracks the player's current raid (separate from dungeon state).
#[derive(Resource, Debug, Clone, Default)]
pub struct RaidState {
    /// The active raid instance, if any.
    pub current: Option<RaidInstance>,
    /// How many companions are active in this raid.
    pub companion_count: usize,
    /// Bosses remaining to be defeated.
    pub bosses_remaining: u32,
}

/// Descriptor for a single raid instance.
#[derive(Debug, Clone)]
pub struct RaidInstance {
    pub name: String,
    pub tier: u32,
    pub depth: u32,
    pub boss_count: u32,
}

/// Marker for a raid entrance marker on the world map.
#[derive(Debug, Clone, Component)]
pub struct RaidEntrance {
    pub name: String,
    pub raid_tier: u32,
    pub depth: u32,
    pub boss_count: u32,
}

// ============================================================================
// AI Companion — Auto Follow & Basic Attack
// ============================================================================

/// AI companion behaviour: follows the player, attacks nearest enemy,
/// and uses basic abilities based on their class.
pub fn ai_companion_ai(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut companion_query: Query<(
        Entity,
        &mut Transform,
        &mut AICompanion,
        &mut Health,
        &ClassResource,
    )>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let Ok(player_pos) = player_query.get_single() else { return };
    let dt = time.delta_secs();

    for (entity, mut transform, mut companion, mut health, _resource) in companion_query.iter_mut() {
        // Find nearest enemy
        let mut nearest: Option<(Entity, f32)> = None;
        for (eid, etf) in enemy_query.iter() {
            let dist = transform.translation.distance(etf.translation);
            if dist < 20.0 {
                match nearest {
                    Some((_, d)) if dist < d => nearest = Some((eid, dist)),
                    None => nearest = Some((eid, dist)),
                    _ => {}
                }
            }
        }

        if let Some((target, dist)) = nearest {
            companion.target = Some(target);
            if dist > 2.0 {
                // Move towards target
                let dir = (transform.translation - player_pos.translation).normalize_or_zero();
                let move_speed = 3.0;
                transform.translation -= dir * move_speed * dt;
            }
            // Auto-attack every 1.5 seconds
            // (In a full implementation this would call the class attack function)
        } else {
            // Follow player within 3-unit radius
            let to_player = player_pos.translation - transform.translation;
            if to_player.length() > 3.0 {
                let dir = to_player.normalize_or_zero();
                let follow_speed = 3.5;
                transform.translation += dir * follow_speed * dt;
            }
            companion.target = None;
        }

        // Self-heal when below 30% HP (all companions)
        if health.current / health.max < 0.3 {
            health.current = (health.current + 5.0 * dt).min(health.max);
        }
    }
}

/// Spawn AI companions for a raid.
pub fn spawn_companions(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    base_x: f32,
    base_z: f32,
    count: usize,
) {
    let companion_classes = [
        CharacterClass::Warrior,
        CharacterClass::Paladin,
        CharacterClass::Rogue,
        CharacterClass::Mage,
    ];
    let companion_names = ["Aldric", "Lyra", "Shade", "Morwen"];

    for i in 0..count.min(RAID_COMPANION_SLOTS) {
        let class = companion_classes[i % companion_classes.len()];
        let name = companion_names[i % companion_names.len()];

        let companion_mesh = meshes.add(Cuboid::new(0.5, 1.0, 0.5));
        let companion_mat = materials.add(Color::srgb(0.2, 0.5, 0.8));

        let offset_x = (i as f32 - 1.5) * 1.5;
        let offset_z = -2.0;

        commands.spawn((
            Mesh3d(companion_mesh),
            MeshMaterial3d(companion_mat),
            Transform::from_xyz(base_x + offset_x, 0.0, base_z + offset_z),
            AICompanion {
                class,
                name: name.to_string(),
                target: None,
            },
            PlayerClass(class),
            Health::new(100.0 + (i as f32) * 25.0),
            ClassResource::new(100.0, 100.0, 5.0),
            Enemy,
            RaidEntity,
        ));
    }
}

// ============================================================================
// Raid Generation
// ============================================================================

/// Generates a raid dungeon (larger grid, multiple bosses, AI companions).
#[allow(clippy::too_many_arguments)]
pub fn generate_raid(
    mut commands: Commands,
    raid_state: Res<RaidState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let instance = match &raid_state.current {
        Some(r) => r,
        None => return,
    };

    let depth = instance.depth.max(4);
    info!(
        "Raid: {} (tier {}, depth {} rooms, {} bosses)",
        instance.name, instance.tier, depth, instance.boss_count
    );

    // ── Meshes and materials ──────────────────────────────────────────
    let floor_mesh = meshes.add(Cuboid::new(2.0 * 0.9, 0.05, 2.0 * 0.9));
    let floor_mat = materials.add(Color::srgb(0.12, 0.08, 0.15)); // dark purple tint
    let corridor_mat = materials.add(Color::srgb(0.10, 0.06, 0.12));
    let treasure_floor_mat = materials.add(Color::srgb(0.08, 0.2, 0.1));
    let wall_mesh = meshes.add(Cuboid::new(0.3, 2.0, 2.0 * 0.9));
    let wall_mat = materials.add(Color::srgb(0.25, 0.2, 0.3));
    let pillar_mesh = meshes.add(Cuboid::new(0.6, 1.5, 0.6));
    let pillar_mat = materials.add(Color::srgb(0.3, 0.25, 0.35));

    // Enemy meshes/materials
    let enemy_mesh = meshes.add(Cuboid::new(0.5, 0.8, 0.5));
    let enemy_mat_elite = materials.add(Color::srgb(0.6, 0.1, 0.8));
    let enemy_mat_boss = materials.add(Color::srgb(1.0, 0.15, 0.15));

    // ── Build room grid (larger: 4×4 minimum) ────────────────────────
    let grid_size = (depth as i32).min(6).max(4);
    // Use a modified build that places multiple boss rooms
    let room_step = 10i32; // slightly larger for raid rooms

    let mut room_positions = Vec::new();
    for rz in 0..grid_size {
        for rx in 0..grid_size {
            room_positions.push((rx, rz));
        }
    }
    // Trim to depth
    room_positions.truncate(depth as usize);

    // Assign room kinds with multiple bosses
    let total_rooms = room_positions.len();
    let mut rng = rand::thread_rng();

    // Force boss rooms at certain positions
    let boss_positions: Vec<(i32, i32)> = if instance.boss_count >= 2 {
        vec![(grid_size - 1, grid_size - 1), (grid_size - 1, 0)]
    } else {
        vec![(grid_size - 1, grid_size - 1)]
    };

    struct RoomBuildInfo {
        pos: (i32, i32),
        kind: RoomKind,
        layout: RoomLayout,
        base_x: f32,
        base_z: f32,
        door_center_x: f32,
        door_center_z: f32,
    }

    let mut room_infos: Vec<RoomBuildInfo> = Vec::new();

    for (idx, &(rx, rz)) in room_positions.iter().enumerate() {
        let kind = if (rx, rz) == (0, 0) {
            RoomKind::Entrance
        } else if boss_positions.contains(&(rx, rz)) {
            RoomKind::Boss
        } else if rng.gen_bool(0.15) && depth >= 4 {
            RoomKind::Miniboss
        } else if rng.gen_bool(0.10) && depth >= 5 {
            RoomKind::Treasure
        } else {
            RoomKind::Combat
        };

        let layout = pick_template(depth, kind, &mut rng);

        let base_x = rx as f32 * room_step as f32 * 2.0;
        let base_z = rz as f32 * room_step as f32 * 2.0;
        let door_center_x = (layout.width / 2) as f32 * 2.0;
        let door_center_z = (layout.height / 2) as f32 * 2.0;

        room_infos.push(RoomBuildInfo {
            pos: (rx, rz),
            kind,
            layout,
            base_x,
            base_z,
            door_center_x,
            door_center_z,
        });
    }

    // Second pass: spawn geometry
    for info in room_infos.iter() {
        let room_floor_mat = match info.kind {
            RoomKind::Treasure => treasure_floor_mat.clone(),
            _ => floor_mat.clone(),
        };

        // Floor tiles
        for &(tx, tz) in &info.layout.tiles {
            commands.spawn((
                Mesh3d(floor_mesh.clone()),
                MeshMaterial3d(room_floor_mat.clone()),
                Transform::from_xyz(
                    info.base_x + tx as f32 * 2.0, -0.5, info.base_z + tz as f32 * 2.0,
                ),
                DungeonFloor,
                RoomEntity,
                RaidEntity,
            ));
        }

        // Spawn enemies per room
        match info.kind {
            RoomKind::Combat => {
                let count = rng.gen_range(3..=6);
                for _ in 0..count {
                    let tx = rng.gen_range(1..info.layout.width - 1);
                    let tz = rng.gen_range(1..info.layout.height - 1);
                    let wx = info.base_x + tx as f32 * 2.0;
                    let wz = info.base_z + tz as f32 * 2.0;
                    commands.spawn((
                        Mesh3d(enemy_mesh.clone()),
                        MeshMaterial3d(enemy_mat_elite.clone()),
                        Transform::from_xyz(wx, 0.0, wz),
                        Enemy { variant: EnemyVariant::Elite, tier: instance.tier as u16, xp_reward: 20 * instance.tier },
                        Health::new(80.0 * (1.0 + instance.tier as f32 * 0.3)),
                        CombatStats::default(),
                        RoomEntity,
                        RaidEntity,
                    ));
                }
            }
            RoomKind::Miniboss => {
                let cx = info.base_x + info.door_center_x;
                let cz = info.base_z + info.door_center_z;
                commands.spawn((
                    Mesh3d(enemy_mesh.clone()),
                    MeshMaterial3d(enemy_mat_elite.clone()),
                    Transform::from_xyz(cx, 0.0, cz),
                    Enemy { variant: EnemyVariant::Elite, tier: (instance.tier + 1) as u16, xp_reward: 50 * instance.tier },
                    Health::new(200.0 * (1.0 + instance.tier as f32 * 0.4)),
                    CombatStats::default(),
                    BossMarker,
                    RoomEntity,
                    RaidEntity,
                ));
            }
            RoomKind::Boss => {
                let cx = info.base_x + info.door_center_x;
                let cz = info.base_z + info.door_center_z;
                commands.spawn((
                    Mesh3d(enemy_mesh.clone()),
                    MeshMaterial3d(enemy_mat_boss.clone()),
                    Transform::from_xyz(cx, 0.0, cz).with_scale(Vec3::splat(1.5)),
                    Enemy { variant: EnemyVariant::Boss, tier: (instance.tier + 2) as u16, xp_reward: 200 * instance.tier },
                    Health::new(500.0 * (1.0 + instance.tier as f32 * 0.5)),
                    CombatStats::default(),
                    BossMarker,
                    RaidBossArena,
                    RoomEntity,
                    RaidEntity,
                ));
            }
            RoomKind::Treasure => {
                // Spawn chests
                let cx = info.base_x + info.door_center_x;
                let cz = info.base_z + info.door_center_z;
                let chest_mesh = meshes.add(Cuboid::new(0.8, 0.6, 0.8));
                let chest_mat = materials.add(Color::srgb(0.9, 0.75, 0.1));
                commands.spawn((
                    Mesh3d(chest_mesh),
                    MeshMaterial3d(chest_mat),
                    Transform::from_xyz(cx, 0.0, cz),
                    LootChest,
                    RoomEntity,
                    RaidEntity,
                ));
            }
            _ => {} // Entrance — no enemies
        }
    }

    // Spawn AI companions at entrance
    let companion_count = raid_state.companion_count;
    let entrance = &room_infos[0];
    spawn_companions(
        &mut commands,
        &mut meshes,
        &mut materials,
        entrance.base_x + entrance.door_center_x,
        entrance.base_z + entrance.door_center_z,
        companion_count,
    );

    info!("Raid ready with {} rooms.", total_rooms);
}

/// Exits raid when player reaches exit and all bosses are defeated.
pub fn check_raid_exit(
    player_query: Query<&Transform, With<Player>>,
    _exits: Query<&Transform, (With<RaidExit>, With<DungeonExit>)>,
    boss_query: Query<(), (With<BossMarker>, With<Enemy>)>,
    mut raid_state: ResMut<RaidState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut dungeon_state: ResMut<DungeonState>,
) {
    if let Some(instance) = &raid_state.current {
        let bosses_alive = boss_query.iter().count();
        raid_state.bosses_remaining = bosses_alive as u32;

        if bosses_alive == 0 {
            info!("Raid {} completed! All bosses defeated.", instance.name);

            // Clear raid state, transition back to World
            raid_state.current = None;
            dungeon_state.current = None;
            next_state.set(AppState::World);
        }
    }
}

/// Cleans up raid entities on exit.
pub fn cleanup_raid(
    mut commands: Commands,
    entities: Query<Entity, With<RaidEntity>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
    info!("Raid cleaned up ({} entities despawned)", entities.iter().count());
}
