use bevy::prelude::*;
use bevy::pbr::MaterialPlugin;
use bevy_hanabi::HanabiPlugin;
use ir_core::{Lifetime, SpawnImpactEvent, CameraTransform};
use crate::{
    assets, camera,
    effects::{self, EffectsLibrary, GlowMaterial},
    hud::{self},
    lighting, spawn,
    asset_pipeline::{self, slots::{ModelSlot, ModelCategory, ModelSlotRegistry}},
};

/// Marker for billboard sprites that always face the camera.
#[derive(Component)]
pub struct BillboardSprite;

/// System set for HUD reactive updates — runs during any gameplay state
/// (World, Dungeon, Playing) instead of being triply-registered.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct HudUpdateSet;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HanabiPlugin,
                MaterialPlugin::<GlowMaterial>::default(),
                asset_pipeline::AssetPipelinePlugin,
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

            // ═══════════════════════════════════════════════════════════════
            // Shared HUD updates — registered once, runs in any active
            // gameplay state (World, Dungeon, Playing) via run condition.
            // ═══════════════════════════════════════════════════════════════
            .configure_sets(Update, HudUpdateSet.run_if(
                in_state(ir_core::AppState::World)
                    .or(in_state(ir_core::AppState::Dungeon))
                    .or(in_state(ir_core::AppState::Playing))
            ))
            .add_systems(Update, (
                hud::update_player_health,
                hud::update_resource_bar,
                hud::update_stamina_bar,
                hud::update_xp_bar,
                hud::update_target_frame,
                hud::update_zone_tracker,
                hud::update_player_frame_class,
                hud::update_gold_text,
            ).in_set(HudUpdateSet))
            // Prompt + dash text only in World and Dungeon (not Playing)
            .add_systems(Update, (
                hud::update_prompt_text,
                hud::update_dash_text,
            ).run_if(
                in_state(ir_core::AppState::World)
                    .or(in_state(ir_core::AppState::Dungeon))
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
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::World)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::World)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::World)),
                assign_shadow.run_if(in_state(ir_core::AppState::World)),
                rotate_billboards.run_if(in_state(ir_core::AppState::World)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::World)),
                spawn_model_slot_scenes.run_if(in_state(ir_core::AppState::World)),
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
                hud::spawn_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_shadow.run_if(in_state(ir_core::AppState::Dungeon)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Dungeon)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_model_slot_scenes.run_if(in_state(ir_core::AppState::Dungeon)),
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
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::Playing)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_shadow.run_if(in_state(ir_core::AppState::Playing)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Playing)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Playing)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::Playing)),
                spawn_model_slot_scenes.run_if(in_state(ir_core::AppState::Playing)),
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
fn assign_projectile_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    projectiles: Query<Entity, (With<ir_core::Projectile>, Without<Mesh3d>, Without<ModelSlot>)>,
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
/// Respects ModelSlot — entities with a ModelSlot let the GLTF spawner
/// handle them instead.
fn assign_enemy_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    enemies: Query<(Entity, &ir_core::Enemy), (Without<Mesh3d>, With<ir_core::Enemy>, Without<ModelSlot>)>,
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
/// Respects ModelSlot — entities with a ModelSlot let the GLTF spawner handle them.
fn assign_pickup_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    gems: Query<Entity, (With<ir_core::ExperienceGem>, Without<Mesh3d>, Without<ModelSlot>)>,
    health_pickups: Query<Entity, (With<ir_core::Pickup>, Without<Mesh3d>, Without<ir_core::ExperienceGem>, Without<ModelSlot>)>,
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

/// Spawns GLTF scene entities for entities that have a ModelSlot but no Mesh3d yet.
///
/// This is the bridge between the asset pipeline and the renderer. When an entity
/// has a ModelSlot assigned (by spawn code, auto-binding, or config), this system
/// looks up the loaded GLTF scene in `ModelSlotRegistry` and spawns it as a child
/// scene. If the model isn't loaded or the slot doesn't resolve, the entity is
/// left alone — the fallback placeholder system will eventually assign a colored quad.
///
/// If the slot model IS resolved, we insert a Mesh3d marker so the placeholder
/// assignment systems skips this entity (they filter on `Without<Mesh3d>`).
/// We also attach the `AnimationStateMachine` component so animation tick works.
fn spawn_model_slot_scenes(
    mut commands: Commands,
    registry: Res<ModelSlotRegistry>,
    entities: Query<(Entity, &ModelSlot, &Transform), (Without<Mesh3d>, Changed<ModelSlot>)>,
) {
    for (entity, slot, transform) in entities.iter() {
        let key = format!("{}/{}", slot.category, slot.name);

        // Skip if already spawned or if the registry doesn't have this model
        if slot.spawned {
            continue;
        }

        if let Some(scene) = registry.scenes.get(&key) {
            bevy::log::info!(
                "Asset pipeline: spawning model '{key}' for entity {:?}",
                entity
            );

            // Spawn the GLTF scene as a child of this entity
            commands.entity(entity).insert((
                SceneRoot(scene.clone()),
                // Mark as having a mesh so placeholder systems skip this entity
                // (but we use a SceneRoot, not Mesh3d, so we need a different marker)
            ));

            // Also tag the entity so billboard/placeholder systems understand it's handled
            commands.entity(entity).insert((
                SceneModelSpawned,
            ));

            // Attach animation state machine for animated models
            commands.entity(entity).insert((
                asset_pipeline::animation::AnimationStateMachine::default(),
            ));
        } else {
            // Model not yet loaded — leave for placeholder fallback
            // This entity will get a colored quad from the assign_*_mesh systems.
            // If the model is still loading, we'll be re-triggered by Changed<ModelSlot>
            // when the model becomes available (future: use asset events).
            bevy::log::trace!(
                "Asset pipeline: model '{key}' not ready yet for entity {:?}, falling back to placeholder.",
                entity
            );
        }
    }
}

/// Marker component for entities that have had their GLTF scene spawned.
/// Prevents the entity from also receiving a placeholder quad.
#[derive(Component)]
pub struct SceneModelSpawned;
