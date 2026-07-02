//! Map generation — creates world tiles, zone markers, dungeon entrances.
//! Enemy respawn, zone-based scaling, decoration variety, ambient life.

use bevy::prelude::*;
use ir_core::*;
use crate::zone::*;

/// Marker for world map tiles.
#[derive(Component)]
pub struct WorldTile;

/// Marker for the dungeon entrance visual.
#[derive(Component)]
pub struct EntranceMarker;

/// Player's current zone (`None` = between zones / in dungeon).
#[derive(Resource, Default)]
pub struct CurrentZone(
    /// The zone the player is currently inside, or `None` if outside all defined zones.
    pub Option<ZoneId>,
);

/// Tracks which zones have had all enemies cleared.
#[derive(Resource, Default)]
pub struct ZoneCleared {
    pub cleared: std::collections::HashSet<ZoneId>,
}

/// Timer for enemy respawn after a zone is cleared.
#[derive(Resource)]
pub struct WorldEnemyRespawnTimer {
    /// Tracks per-zone timers: map from zone to time remaining before respawn.
    pub timers: std::collections::HashMap<ZoneId, f32>,
}

impl Default for WorldEnemyRespawnTimer {
    fn default() -> Self {
        Self {
            timers: std::collections::HashMap::new(),
        }
    }
}

/// Zone transition state for the loading screen overlay.
#[derive(Resource)]
pub struct ZoneTransitionState {
    pub active: bool,
    pub zone_name: String,
    pub zone_label: String,
    pub timer: f32,
    pub phase: TransitionPhase,
    pub previous_zone: Option<ZoneId>,
}

impl Default for ZoneTransitionState {
    fn default() -> Self {
        Self {
            active: false,
            zone_name: String::new(),
            zone_label: String::new(),
            timer: 0.0,
            phase: TransitionPhase::None,
            previous_zone: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionPhase {
    None,
    FadeIn,
    Hold,
    FadeOut,
}

/// Marker component for grass blade ambient entities (for sway animation).
#[derive(Component)]
pub struct GrassBlade {
    pub base_rotation: Quat,
    pub sway_offset: f32,
    pub sway_speed: f32,
    pub sway_amount: f32,
}

/// Marker component for ambient pebble entities.
#[derive(Component)]
pub struct Pebble;

/// Marker component for ground variation patch entities.
#[derive(Component)]
pub struct GroundPatch;

/// Fetches the ZoneDef for a given ZoneId by scanning all_zones().
pub fn zone_def_by_id(id: &ZoneId) -> Option<ZoneDef> {
    all_zones().into_iter().find(|z| z.id == *id)
}

/// Generates all zone tiles and dungeon entrances in the world.
pub fn generate_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
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

        // ── Ground tiles ──────────────────────────────────────────────
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

        // ── Zone-specific environment decorations ─────────────────────
        spawn_zone_decorations(&mut commands, &game_assets, &mut materials, &zone, base_x, base_z);

        // ── Ambient ground variation patches ──────────────────────────
        spawn_ground_patches(&mut commands, &mut meshes, &mut materials, &zone, base_x, base_z);

        // ── Ambient pebbles ───────────────────────────────────────────
        spawn_pebbles(&mut commands, &mut meshes, &mut materials, &zone, base_x, base_z);

        // ── Ambient grass blades (with sway) ──────────────────────────
        spawn_grass_blades(&mut commands, &mut meshes, &mut materials, &zone, base_x, base_z);

        // ── Dungeon entrance markers ──────────────────────────────────
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

        // ── Zone-specific enemies ──────────────────────────────────────
        spawn_zone_enemies(&mut commands, &mut meshes, &mut materials, &zone.id, base_x, base_z);
    }
}

/// Resolve a mesh tag string to the actual `Handle<Mesh>` from GameAssets.
fn mesh_for_tag(assets: &GameAssets, tag: &str) -> Handle<Mesh> {
    match tag {
        "bush" => assets.bush_mesh.clone(),
        "tree" => assets.tree_mesh.clone(),
        "rock" => assets.rock_mesh.clone(),
        "grass_blade" => assets.grass_blade_mesh.clone(),
        "flower" => assets.flower_mesh.clone(),
        "cactus" => assets.cactus_mesh.clone(),
        "mushroom" => assets.mushroom_mesh.clone(),
        "crystal" => assets.crystal_mesh.clone(),
        "pillar" => assets.pillar_mesh.clone(),
        _ => {
            warn!("Unknown mesh tag '{}', falling back to rock mesh", tag);
            assets.rock_mesh.clone()
        }
    }
}

/// Spawns zone-specific environment decorations with procedural mesh and material handles.
pub fn spawn_zone_decorations(
    commands: &mut Commands,
    game_assets: &GameAssets,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    zone: &ZoneDef,
    base_x: f32,
    base_z: f32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let defs = zone.id.decor_definitions();
    if defs.is_empty() {
        return;
    }

    let decor_count = rng.gen_range(12..=20);
    for _ in 0..decor_count {
        let tx = rng.gen_range(2..zone.tile_w - 2);
        let tz = rng.gen_range(2..zone.tile_h - 2);
        let idx = rng.gen_range(0..defs.len());
        let decor = &defs[idx];
        let mesh = mesh_for_tag(game_assets, decor.mesh_tag);
        let mat = materials.add(decor.color);
        let y_offset = 0.2; // approximate vertical offset for decor objects
        let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.6..0.6);
        let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.6..0.6);
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_xyz(wx, -0.5 + y_offset, wz),
            RoomEntity,
        ));
    }
}

/// Spawns ground color variation patches for ambient life.
fn spawn_ground_patches(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    zone: &ZoneDef,
    base_x: f32,
    base_z: f32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let colors = zone.id.ground_patch_colors();
    if colors.is_empty() {
        return;
    }

    let patch_mesh = meshes.add(Cuboid::new(0.3, 0.01, 0.3));
    let patch_count = rng.gen_range(15..=25);
    for _ in 0..patch_count {
        let tx = rng.gen_range(0..zone.tile_w);
        let tz = rng.gen_range(0..zone.tile_h);
        let color = colors[rng.gen_range(0..colors.len())];
        let mat = materials.add(color);
        let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.8..0.8);
        let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.8..0.8);
        commands.spawn((
            Mesh3d(patch_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(wx, -0.48, wz),
            GroundPatch,
            RoomEntity,
        ));
    }
}

/// Spawns tiny pebble ambient objects scattered across tiles.
fn spawn_pebbles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    zone: &ZoneDef,
    base_x: f32,
    base_z: f32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let pebble_mesh = meshes.add(Cuboid::new(0.04, 0.02, 0.04));
    let pebble_colors = [
        Color::srgb(0.35, 0.32, 0.28),
        Color::srgb(0.4, 0.38, 0.33),
        Color::srgb(0.3, 0.28, 0.25),
        Color::srgb(0.38, 0.35, 0.3),
    ];
    let pebble_count = rng.gen_range(20..=35);
    for _ in 0..pebble_count {
        let tx = rng.gen_range(0..zone.tile_w);
        let tz = rng.gen_range(0..zone.tile_h);
        let color = pebble_colors[rng.gen_range(0..pebble_colors.len())];
        let mat = materials.add(color);
        let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.9..0.9);
        let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.9..0.9);
        let rot = Quat::from_rotation_z(rng.gen::<f32>() * std::f32::consts::TAU);
        commands.spawn((
            Mesh3d(pebble_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_xyz(wx, -0.48, wz).with_rotation(rot),
            Pebble,
            RoomEntity,
        ));
    }
}

/// Spawns tall thin grass blade ambient objects with sway animation parameters.
fn spawn_grass_blades(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    zone: &ZoneDef,
    base_x: f32,
    base_z: f32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Only natural zones get grass blades
    let (blade_mesh, blade_color) = match zone.id {
        ZoneId::Grasslands => (
            meshes.add(Cuboid::new(0.03, 0.2, 0.03)),
            Color::srgb(0.12, 0.35, 0.08),
        ),
        ZoneId::Desert => (
            meshes.add(Cuboid::new(0.02, 0.12, 0.02)),
            Color::srgb(0.5, 0.4, 0.15),
        ),
        ZoneId::Forest => (
            meshes.add(Cuboid::new(0.03, 0.25, 0.03)),
            Color::srgb(0.08, 0.3, 0.06),
        ),
        ZoneId::Swamp => (
            meshes.add(Cuboid::new(0.02, 0.18, 0.02)),
            Color::srgb(0.2, 0.25, 0.08),
        ),
        _ => return, // No grass blades in Tundra or Void
    };
    let blade_mat = materials.add(blade_color);
    let blade_count = rng.gen_range(15..=30);
    for _ in 0..blade_count {
        let tx = rng.gen_range(0..zone.tile_w);
        let tz = rng.gen_range(0..zone.tile_h);
        let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.9..0.9);
        let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.9..0.9);
        let half_h = 0.2 / 2.0; // half height of the cuboid
        let rot = Quat::from_rotation_z(rng.gen::<f32>() * std::f32::consts::TAU);
        let sway_speed = rng.gen_range(0.8..1.5);
        let sway_amount = rng.gen_range(0.02..0.06);
        commands.spawn((
            Mesh3d(blade_mesh.clone()),
            MeshMaterial3d(blade_mat.clone()),
            Transform::from_xyz(wx, -0.5 + half_h, wz).with_rotation(rot),
            GrassBlade {
                base_rotation: rot,
                sway_offset: rng.gen::<f32>() * std::f32::consts::TAU,
                sway_speed,
                sway_amount,
            },
            RoomEntity,
        ));
    }
}

/// Spawns zone-appropriate enemies. Public so the respawn system can also call it.
pub fn spawn_zone_enemies(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    zone_id: &ZoneId,
    base_x: f32,
    base_z: f32,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let zone_def = zone_def_by_id(zone_id).unwrap_or_else(|| {
        // Fallback for zones not in all_zones() — just use Grasslands def
        all_zones().into_iter().next().unwrap()
    });

    // Determine spawn count
    let count = match zone_id {
        ZoneId::Grasslands => rng.gen_range(3..=6),
        ZoneId::Desert => rng.gen_range(4..=7),
        ZoneId::Forest => rng.gen_range(4..=6),
        ZoneId::Tundra => rng.gen_range(5..=8),
        ZoneId::Swamp => rng.gen_range(4..=7),
        _ => rng.gen_range(3..=5),
    };

    // Compute tier range and composition
    let (tier_min, tier_max) = zone_id.enemy_tier_range();
    let composition = zone_id.enemy_composition();
    let total_weight: u32 = composition.iter().map(|(_, w)| w).sum();
    let colors = zone_id.enemy_colors();
    let hp_mult = zone_id.enemy_hp_multiplier();

    // Pre-generate meshes per variant for reuse
    let mut variant_meshes = std::collections::HashMap::new();
    for variant in &[EnemyVariant::Grunt, EnemyVariant::Ranged, EnemyVariant::Charger, EnemyVariant::Elite, EnemyVariant::Boss, EnemyVariant::Caster, EnemyVariant::Healer, EnemyVariant::Summoner, EnemyVariant::Assassin, EnemyVariant::Brute] {
        let mesh = match variant {
            EnemyVariant::Grunt => meshes.add(Cuboid::new(0.5, 0.7, 0.5)),
            EnemyVariant::Ranged => meshes.add(Cuboid::new(0.4, 0.8, 0.4)),
            EnemyVariant::Charger => meshes.add(Cuboid::new(0.6, 0.6, 0.6)),
            EnemyVariant::Elite => meshes.add(Cuboid::new(0.5, 0.85, 0.5)),
            EnemyVariant::Boss => meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
            EnemyVariant::Caster => meshes.add(Cuboid::new(0.4, 0.9, 0.4)),
            EnemyVariant::Healer => meshes.add(Cuboid::new(0.5, 0.8, 0.5)),
            EnemyVariant::Summoner => meshes.add(Cuboid::new(0.5, 0.8, 0.5)),
            EnemyVariant::Assassin => meshes.add(Cuboid::new(0.3, 0.6, 0.3)),
            EnemyVariant::Brute => meshes.add(Cuboid::new(0.8, 0.9, 0.8)),
        };
        variant_meshes.insert(variant.clone(), mesh);
    }

    for _ in 0..count {
        // Pick variant based on weighted composition
        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;
        let mut selected_variant = EnemyVariant::Grunt;
        for (variant, weight) in &composition {
            cumulative += weight;
            if roll < cumulative {
                selected_variant = variant.clone();
                break;
            }
        }

        // Determine tier within zone range
        let tier = if tier_min == tier_max {
            tier_min
        } else {
            rng.gen_range(tier_min..=tier_max)
        };

        // Determine position
        let tx = rng.gen_range(1..zone_def.tile_w - 1);
        let tz = rng.gen_range(1..zone_def.tile_h - 1);
        let wx = base_x + tx as f32 * 2.0 + rng.gen_range(-0.3..0.3);
        let wz = base_z + tz as f32 * 2.0 + rng.gen_range(-0.3..0.3);

        // Get zone-appropriate color for this variant
        let color_idx = match selected_variant {
            EnemyVariant::Grunt => 0usize,
            EnemyVariant::Ranged => 1,
            EnemyVariant::Charger => 2,
            EnemyVariant::Elite => 3,
            EnemyVariant::Boss => 4,
            _ => 0,
        };
        let color = colors[color_idx.min(4)];
        let mat = materials.add(color);

        // Base HP scales with tier
        let base_hp = 15.0 + tier as f32 * 10.0;
        let hp = base_hp * hp_mult;

        // XP scales with tier
        let xp = (5 + tier * 5) as u64;

        // Move speed varies by variant
        let move_speed = match selected_variant {
            EnemyVariant::Grunt => 2.0 + tier as f32 * 0.2,
            EnemyVariant::Ranged => 2.5 + tier as f32 * 0.2,
            EnemyVariant::Charger => 3.5 + tier as f32 * 0.3,
            EnemyVariant::Elite => 3.0 + tier as f32 * 0.25,
            EnemyVariant::Boss => 1.5,
            _ => 2.5,
        };

        let mesh = variant_meshes.get(&selected_variant).cloned()
            .unwrap_or_else(|| variant_meshes.get(&EnemyVariant::Grunt).cloned().unwrap());

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_xyz(wx, 0.0, wz),
            Enemy {
                variant: selected_variant,
                tier,
                xp_reward: xp,
            },
            Health::new(hp),
            CombatStats {
                move_speed,
                damage_bonus: tier as f32 * 2.0,
                ..default()
            },
            Velocity(Vec3::ZERO),
            Team::Enemy,
            AttackCooldown::default(),
            RoomEntity,
            crate::zone::WorldPos(tx as i32, tz as i32),
        ));
    }
}

/// Tracks which zone the player is currently in, updates every frame.
pub fn track_player_zone(
    player_query: Query<&Transform, With<Player>>,
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

/// Detects zone boundary crossing and sets the transition state.
pub fn detect_zone_change(
    current_zone: Res<CurrentZone>,
    mut transition: ResMut<ZoneTransitionState>,
) {
    // Detect change from previous_zone to current_zone
    if transition.previous_zone != current_zone.0 {
        // If going from None to Some (first entry or leaving void), or changing zones
        if let Some(ref zone_id) = current_zone.0 {
            // Only trigger if we actually moved (skip on first load if we want)
            // Trigger transition when entering a new zone
            transition.active = true;
            transition.zone_name = zone_id.display_name().to_string();
            transition.zone_label = zone_id.zone_label().to_string();
            transition.timer = 0.0;
            transition.phase = TransitionPhase::FadeIn;
        }
        // If going from Some to None (entering void), we don't show the transition
        transition.previous_zone = current_zone.0.clone();
    }
}

/// Checks if a zone has been fully cleared of enemies and starts respawn timer.
pub fn check_zone_cleared(
    enemy_query: Query<(&Enemy, &Transform)>,
    mut cleared: ResMut<ZoneCleared>,
    mut respawn_timer: ResMut<WorldEnemyRespawnTimer>,
) {
    for zone in all_zones() {
        let zone_id = zone.id.clone();

        // Skip void
        if zone_id == ZoneId::Void {
            continue;
        }

        // If already cleared and timer already running, skip check
        if respawn_timer.timers.contains_key(&zone_id) {
            continue;
        }

        // Count enemies in this zone
        let x_min = zone.offset_x as f32 * 2.0;
        let x_max = x_min + zone.tile_w as f32 * 2.0;
        let z_min = zone.offset_z as f32 * 2.0;
        let z_max = z_min + zone.tile_h as f32 * 2.0;

        let enemy_count = enemy_query.iter().filter(|(_, tf)| {
            tf.translation.x >= x_min
                && tf.translation.x < x_max
                && tf.translation.z >= z_min
                && tf.translation.z < z_max
        }).count();

        if enemy_count == 0 && !cleared.cleared.contains(&zone_id) {
            // Zone just got cleared — start respawn timer
            cleared.cleared.insert(zone_id.clone());
            respawn_timer.timers.insert(zone_id.clone(), 30.0);
            info!("Zone {:?} cleared — respawn in 30s", zone_id);
        }
    }
}

/// Ticks the enemy respawn timer and respawns enemies when timer expires.
pub fn enemy_respawn_tick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut respawn_timer: ResMut<WorldEnemyRespawnTimer>,
    mut cleared: ResMut<ZoneCleared>,
) {
    let mut to_respawn = Vec::new();

    // Tick timers
    for (zone_id, timer) in respawn_timer.timers.iter_mut() {
        *timer -= time.delta_secs();
        if *timer <= 0.0 {
            to_respawn.push(zone_id.clone());
        }
    }

    // Respawn for expired timers
    for zone_id in to_respawn {
        respawn_timer.timers.remove(&zone_id);
        cleared.cleared.remove(&zone_id);

        // Find zone definition
        if let Some(zone) = zone_def_by_id(&zone_id) {
            let base_x = zone.offset_x as f32 * 2.0;
            let base_z = zone.offset_z as f32 * 2.0;
            info!("Respawning enemies in zone {:?}", zone_id);
            spawn_zone_enemies(
                &mut commands,
                &mut meshes,
                &mut materials,
                &zone_id,
                base_x,
                base_z,
            );
        }
    }
}

/// Animates grass blade sway.
pub fn sway_grass(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &GrassBlade)>,
) {
    for (mut transform, grass) in query.iter_mut() {
        let angle = (time.elapsed_secs() * grass.sway_speed + grass.sway_offset).sin() * grass.sway_amount;
        let sway_rot = Quat::from_rotation_x(angle);
        transform.rotation = grass.base_rotation * sway_rot;
    }
}
