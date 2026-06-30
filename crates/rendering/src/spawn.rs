//! Spawns the floor plane and player, then transitions to Playing.

use bevy::prelude::*;
use ir_core::*;

/// Spawns the floor plane and player, then transitions to Playing.
pub fn spawn_game_world(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<AppState>>,
    mut run_start_events: EventWriter<RunStartEvent>,
) {
    // Floor
    commands.spawn((
        Mesh3d(assets.floor_mesh.clone()),
        MeshMaterial3d(assets.floor_material.clone()),
        Transform::from_xyz(0.0, -0.5, 0.0),
        RoomEntity,
    ));

    // Player
    commands.spawn((
        Mesh3d(assets.player_mesh.clone()),
        MeshMaterial3d(assets.player_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default(),
        Health::new(100.0),
        CombatStats::default(),
        Weapon::new(WeaponKind::MagicMissile, 10.0, 1.5, 15.0),
        Velocity(Vec3::ZERO),
        Team::Player,
        RoomEntity,
    ));

    // Fire event
    run_start_events.send(RunStartEvent);

    // Start the game!
    next_state.set(AppState::Playing);
}
