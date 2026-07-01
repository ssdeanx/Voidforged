//! Spawns, cleans up game world entities, and handles game-over/restart.

use bevy::prelude::*;
use ir_core::*;

use crate::asset_pipeline::animation::AnimationStateMachine;
use crate::asset_pipeline::loader::slot_for_class;

/// Spawns the ground grid, player, and environment — everything for a new run.
pub fn spawn_game_world(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // ── Ground plane (dark base) ──────────────────────────────────────
    commands.spawn((
        Mesh3d(assets.floor_mesh.clone()),
        MeshMaterial3d(assets.floor_material.clone()),
        Transform::from_xyz(0.0, -0.5, 0.0),
        RoomEntity,
    ));

    // ── Ground tiles (grid for visual context) ────────────────────────
    let tile_size = 1.8;
    let half_extent = 9; // 9 tiles in each direction from center
    for x in -half_extent..=half_extent {
        for z in -half_extent..=half_extent {
            let alt = (x + z) % 2 == 0;
            let mat = if alt {
                assets.tile_material.clone()
            } else {
                assets.tile_material_alt.clone()
            };
            commands.spawn((
                Mesh3d(assets.tile_mesh.clone()),
                MeshMaterial3d(mat),
                Transform::from_xyz(
                    x as f32 * tile_size,
                    -0.45,
                    z as f32 * tile_size,
                ),
                RoomEntity,
            ));
        }
    }

    // ── Boundary walls (short boxes to show arena edge) ──────────────
    let wall_mat = assets.wall_material.clone();
    let wall_mesh = assets.wall_mesh.clone();
    let half = half_extent as f32 * tile_size;
    let wall_positions = [
        (0.0, half + 1.0),   // front
        (0.0, -half - 1.0),  // back
        (half + 1.0, 0.0),   // right
        (-half - 1.0, 0.0),  // left
    ];
    for (x, z) in wall_positions {
        let is_side = x.abs() > z.abs();
        commands.spawn((
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_mat.clone()),
            Transform::from_xyz(x, 0.0, z).with_rotation(if is_side {
                Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
            } else {
                Quat::IDENTITY
            }),
            RoomEntity,
        ));
    }

    // ── Player (with model slot + animation state machine) ──────────
    commands.spawn((
        Mesh3d(assets.player_mesh.clone()),
        MeshMaterial3d(assets.player_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default(),
        Health {
            current: 100.0,
            max: 100.0,
            invulnerable_until: 4.0, // ~2s of spawn immunity
        },
        CombatStats::default(),
        Weapon::new(WeaponKind::MagicMissile, 10.0, 1.5, 15.0),
        Velocity(Vec3::ZERO),
        DashCooldown::default(),
        Equipment::default(),
        Team::Player,
        RoomEntity,
        // Asset pipeline integration — enables GLTF model replacement
        slot_for_class(&CharacterClass::Warrior),
        AnimationStateMachine::default(),
    ));
}

/// Spawns the player entity at world origin with class data from character creation.
///
/// Tags the entity with `ModelSlot` (for GLTF scene replacement) and
/// `AnimationStateMachine` (for state-driven animation). When the asset
/// pipeline has the matching 3D model loaded, the placeholder quad is
/// replaced automatically by `assign_scene_from_slot`.
pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    creation_state: Res<CharacterCreationState>,
) {
    let class = creation_state.selected_class.unwrap_or(CharacterClass::Warrior);
    let name = if creation_state.player_name.is_empty() {
        class.display_name().to_string()
    } else {
        creation_state.player_name.clone()
    };
    let stats = class.base_stats();
    let weapon = class.starting_weapon();

    commands.spawn((
        Mesh3d(assets.player_mesh.clone()),
        MeshMaterial3d(assets.player_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default(),
        PlayerClass(class),
        PlayerName(name),
        Health {
            current: class.base_max_hp(),
            max: class.base_max_hp(),
            invulnerable_until: time.elapsed_secs_f64() as f32 + 4.0,
        },
        stats,
        weapon,
        Velocity(Vec3::ZERO),
        DashCooldown::default(),
        Team::Player,
        // Asset pipeline integration
        slot_for_class(&class),
        AnimationStateMachine::default(),
    ))
    .insert(Equipment::default())
    .insert(Inventory::new(20));
}

/// Despawns all RoomEntity-marked entities — used on GameOver and state transitions.
pub fn cleanup_world(mut commands: Commands, entities: Query<Entity, With<RoomEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

/// Despawns the player entity — used during GameOver cleanup.
pub fn despawn_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    for entity in player.iter() {
        commands.entity(entity).despawn();
    }
}

/// Resets per-run resources to defaults when starting a new run.
pub fn reset_run_resources(
    mut wave_state: ResMut<WaveState>,
    mut progression: ResMut<RunProgression>,
    mut input: ResMut<PlayerInput>,
) {
    *wave_state = WaveState::default();
    *progression = RunProgression::default();
    *input = PlayerInput::default();
}

/// Reads Enter key to transition from MainMenu → CharacterSelect.
pub fn start_game_from_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::CharacterSelect);
    }
}

/// Reads Enter/Space to restart from GameOver → Playing directly.
pub fn restart_from_game_over(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}
