use bevy::prelude::*;
use bevy::pbr::MaterialPlugin;
use bevy_hanabi::HanabiPlugin;
use ir_core::{Lifetime, SpawnImpactEvent, CameraTransform};
use crate::{
    assets, camera,
    effects::{self, EffectsLibrary, GlowMaterial},
    hud::{self},
    lighting, spawn,
};
use crate::asset_pipeline::{
    animation::tick_animation_clips,
    loader::assign_scene_from_slot,
    AssetPipelinePlugin,
};

/// Marker for billboard sprites that always face the camera.
#[derive(Component)]
pub struct BillboardSprite;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HanabiPlugin,
                MaterialPlugin::<GlowMaterial>::default(),
                AssetPipelinePlugin,   // GLTF asset pipeline (config, loading, slots)
            ))

            // Startup — camera + lights
            .add_systems(Startup, (
                camera::spawn_isometric_camera,
                lighting::setup_lighting,
            ))

            // Loading — generate placeholder assets + effects library
            .add_systems(OnEnter(ir_core::AppState::Loading), (
                assets::generate_placeholder_assets,
                effects::build_effects_library,
            ))

            // MainMenu — clean up, show menu
            .add_systems(OnEnter(ir_core::AppState::MainMenu), (
                hud::despawn_hud,
                hud::despawn_pause_overlay,
                hud::despawn_game_over,
                spawn::cleanup_world,
                hud::spawn_main_menu_screen,
            ))
            .add_systems(Update, spawn::start_game_from_menu
                .run_if(in_state(ir_core::AppState::MainMenu)))

            // CharacterSelect — choose class + name before entering world
            .add_systems(OnEnter(ir_core::AppState::CharacterSelect), (
                hud::despawn_main_menu,
                hud::character_select::spawn_character_select,
            ))
            .add_systems(Update, (
                hud::character_select::handle_class_selection,
                hud::character_select::handle_name_input,
                hud::character_select::confirm_character,
                hud::character_select::play_existing_character,
                hud::character_select::delete_character,
                hud::character_select::populate_existing_characters,
            ).run_if(in_state(ir_core::AppState::CharacterSelect)))
            .add_systems(OnExit(ir_core::AppState::CharacterSelect), (
                hud::character_select::despawn_character_select,
            ))

            // World — open world exploration
            .add_systems(OnEnter(ir_core::AppState::World), (
                hud::despawn_main_menu,
                hud::despawn_game_over,
                hud::despawn_pause_overlay,
                spawn::cleanup_world,
                spawn::spawn_player,
                hud::spawn_hud,
            ))
            .add_systems(Update, (
                camera::follow_player.run_if(in_state(ir_core::AppState::World)),
                camera::cursor_to_world.run_if(in_state(ir_core::AppState::World)),
            ))
            .add_systems(Update, (
                hud::update_player_health.run_if(in_state(ir_core::AppState::World)),
                hud::update_resource_bar.run_if(in_state(ir_core::AppState::World)),
                hud::update_stamina_bar.run_if(in_state(ir_core::AppState::World)),
                hud::update_xp_bar.run_if(in_state(ir_core::AppState::World)),
                hud::update_target_frame.run_if(in_state(ir_core::AppState::World)),
                hud::update_zone_tracker.run_if(in_state(ir_core::AppState::World)),
                hud::update_prompt_text.run_if(in_state(ir_core::AppState::World)),
                hud::update_player_frame_class.run_if(in_state(ir_core::AppState::World)),
                hud::update_dash_text.run_if(in_state(ir_core::AppState::World)),
                hud::update_gold_text.run_if(in_state(ir_core::AppState::World)),
            ))
            .add_systems(Update, (
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::World)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::World)),
                assign_scene_from_slot.run_if(in_state(ir_core::AppState::World)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_shadow.run_if(in_state(ir_core::AppState::World)),
                rotate_billboards.run_if(in_state(ir_core::AppState::World)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::World)),

            ))
            // Dungeon — combat instances
            .add_systems(OnEnter(ir_core::AppState::Dungeon), (
                spawn::cleanup_world,
                hud::spawn_hud,
            ))
            .add_systems(Update, (
                camera::follow_player.run_if(in_state(ir_core::AppState::Dungeon)),
                camera::cursor_to_world.run_if(in_state(ir_core::AppState::Dungeon)),
                camera::apply_screen_shake.run_if(in_state(ir_core::AppState::Dungeon)),
            ))
            .add_systems(Update, (
                hud::update_player_health.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_resource_bar.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_stamina_bar.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_xp_bar.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_target_frame.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_zone_tracker.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_prompt_text.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_player_frame_class.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_dash_text.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_gold_text.run_if(in_state(ir_core::AppState::Dungeon)),
            ))
            .add_systems(Update, (
                hud::spawn_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_scene_from_slot.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_shadow.run_if(in_state(ir_core::AppState::Dungeon)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Dungeon)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::Dungeon)),

            ))

            // Playing — spawns player and HUD (used after GameOver→restart)
            .add_systems(OnEnter(ir_core::AppState::Playing), (
                hud::despawn_main_menu,
                hud::despawn_game_over,
                spawn::spawn_player,
                hud::spawn_hud,
            ))
            .add_systems(Update, (
                camera::follow_player.run_if(in_state(ir_core::AppState::Playing)),
                camera::cursor_to_world.run_if(in_state(ir_core::AppState::Playing)),
                camera::apply_screen_shake.run_if(in_state(ir_core::AppState::Playing)),
            ))
            .add_systems(Update, (
                hud::update_player_health.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_resource_bar.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_stamina_bar.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_xp_bar.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_target_frame.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_zone_tracker.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_player_frame_class.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_gold_text.run_if(in_state(ir_core::AppState::Playing)),
            ))
            .add_systems(Update, (
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::Playing)),
                assign_scene_from_slot.run_if(in_state(ir_core::AppState::Playing)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_shadow.run_if(in_state(ir_core::AppState::Playing)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Playing)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Playing)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::Playing)),
                tick_animation_clips.run_if(in_state(ir_core::AppState::Playing)),
            ))

            // Paused — overlay
            .add_systems(OnEnter(ir_core::AppState::Paused), (
                hud::spawn_pause_overlay,
            ))
            .add_systems(OnExit(ir_core::AppState::Paused), (
                hud::despawn_pause_overlay,
            ))

            // GameOver — clean up, despawn player, show stats
            .add_systems(OnEnter(ir_core::AppState::GameOver), (
                hud::despawn_hud,
                hud::despawn_pause_overlay,
                spawn::cleanup_world,
                spawn::despawn_player,
                hud::spawn_game_over_screen,
            ))
            .add_systems(Update, spawn::restart_from_game_over
                .run_if(in_state(ir_core::AppState::GameOver)));
    }
}

/// Reads SpawnImpactEvent and spawns a particle burst at the position.
fn spawn_impact_effect(
    mut commands: Commands,
    library: Res<EffectsLibrary>,
    mut events: EventReader<SpawnImpactEvent>,
) {
    for event in events.read() {
        effects::spawn_impact(&mut commands, &library, event.position);
    }
}

/// Despawns entities whose Lifetime has expired.
fn cleanup_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.remaining -= time.delta_secs();
        if lifetime.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Assigns a default visible mesh/material to projectile entities that lack one.
/// Skips entities that already have a `SceneRoot` from the asset pipeline.
fn assign_projectile_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    projectiles: Query<Entity, (With<ir_core::Projectile>, Without<Mesh3d>, Without<SceneRoot>)>,
) {
    for entity in projectiles.iter() {
        commands.entity(entity).insert((
            Mesh3d(assets.projectile_mesh.clone()),
            MeshMaterial3d(assets.projectile_material.clone()),
            BillboardSprite,
        ));
    }
}

/// Assigns visible mesh/material to enemy entities based on their variant.
/// Skips enemies that already have a `SceneRoot` from the asset pipeline
/// (i.e., a GLTF model was loaded for them).
fn assign_enemy_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    enemies: Query<(Entity, &ir_core::Enemy), (Without<Mesh3d>, With<ir_core::Enemy>, Without<SceneRoot>)>,
) {
    for (entity, enemy) in enemies.iter() {
        let idx = match enemy.variant {
            ir_core::EnemyVariant::Grunt => 0,
            ir_core::EnemyVariant::Ranged => 1,
            ir_core::EnemyVariant::Charger => 2,
            ir_core::EnemyVariant::Elite => 3,
            ir_core::EnemyVariant::Boss => 4,
        };
        let mesh = assets.enemy_meshes.get(idx).cloned()
            .unwrap_or_else(|| assets.enemy_meshes[0].clone());
        let mat = assets.enemy_materials.get(idx).cloned()
            .unwrap_or_else(|| assets.enemy_materials[0].clone());
        commands.entity(entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            BillboardSprite,
        ));
    }
}

/// Assigns visible mesh/material to health/gold pickups, XP gems that lack them.
/// Skips entities with a `SceneRoot` from the asset pipeline.
fn assign_pickup_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    gems: Query<Entity, (With<ir_core::ExperienceGem>, Without<Mesh3d>, Without<SceneRoot>)>,
    health_pickups: Query<Entity, (With<ir_core::Pickup>, Without<Mesh3d>, Without<ir_core::ExperienceGem>, Without<SceneRoot>)>,
) {
    for entity in gems.iter() {
        commands.entity(entity).insert((
            Mesh3d(assets.gem_mesh.clone()),
            MeshMaterial3d(assets.gem_material.clone()),
            BillboardSprite,
        ));
    }
    for entity in health_pickups.iter() {
        commands.entity(entity).insert((
            Mesh3d(assets.health_pickup_mesh.clone()),
            MeshMaterial3d(assets.health_pickup_material.clone()),
            BillboardSprite,
        ));
    }
}

/// Adds a shadow sprite under character-like entities that lack one.
fn assign_shadow(
    mut commands: Commands,
    _assets: Res<ir_core::GameAssets>,
    shadows: Query<Entity, (With<ir_core::Enemy>, Without<ShadowSprite>)>,
    player_shadows: Query<Entity, (With<ir_core::Player>, Without<ShadowSprite>)>,
) {
    for entity in shadows.iter().chain(player_shadows.iter()) {
        commands.entity(entity).insert(ShadowSprite);
    }
}

#[derive(Component)]
pub struct ShadowSprite;

/// Rotates BillboardSprite entities to always face the camera.
fn rotate_billboards(
    cam: Res<CameraTransform>,
    mut query: Query<&mut Transform, (With<BillboardSprite>, Without<ir_core::Player>)>,
    mut player_query: Query<&mut Transform, (With<ir_core::Player>, With<BillboardSprite>)>,
) {
    let cam_pos = cam.0;
    for mut transform in query.iter_mut() {
        let dir = cam_pos - transform.translation;
        if dir.length_squared() > 0.01 {
            transform.look_at(cam_pos, Vec3::Y);
        }
    }
    // Player sprite always faces camera too
    for mut transform in player_query.iter_mut() {
        let dir = cam_pos - transform.translation;
        if dir.length_squared() > 0.01 {
            transform.look_at(cam_pos, Vec3::Y);
        }
    }
}
