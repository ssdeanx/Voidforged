use bevy::prelude::*;
use bevy::pbr::MaterialPlugin;
use bevy_hanabi::HanabiPlugin;
use ir_core::{Lifetime, SpawnImpactEvent, SpawnDeathEffectEvent, CameraTransform, HitFlash, Projectile, TrailSegment};
use crate::{
    assets, audio, camera,
    effects::{self, EffectsLibrary, GlowMaterial},
    hud::{self},
    lighting, spawn,
};

/// Marker for billboard sprites that always face the camera.
#[derive(Component)]
pub struct BillboardSprite;

/// Helper component that stores the original material handle before a hit-flash swap.
#[derive(Component)]
pub struct FlashMaterial {
    pub original: Handle<StandardMaterial>,
}

/// Resource for trail effect mesh and material.
#[derive(Resource)]
pub struct TrailAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HanabiPlugin,
                MaterialPlugin::<GlowMaterial>::default(),
                audio::AudioPlugin,
            ))

            // Register custom events
            .add_event::<hud::PurchaseUpgradeEvent>()

            // Resources
            .insert_resource(hud::components::AudioVolume(0.5))

            // Startup — camera + lights
            .add_systems(Startup, (
                camera::spawn_isometric_camera,
                lighting::setup_lighting,
            ))

            // Loading — generate placeholder assets + effects library + trail assets + UI textures
            .add_systems(OnEnter(ir_core::AppState::Loading), (
                assets::generate_placeholder_assets,
                effects::build_effects_library,
                setup_trail_assets,
                crate::ui_textures::generate_ui_textures,
            ))

            // MainMenu — clean up, show menu
            .add_systems(OnEnter(ir_core::AppState::MainMenu), (
                hud::despawn_hud,
                hud::despawn_pause_overlay,
                hud::despawn_game_over,
                hud::despawn_upgrade_tree,
                spawn::cleanup_world,
                hud::spawn_main_menu_screen,
                effects::spawn_menu_bg_particles_system,
            ))
            .add_systems(OnExit(ir_core::AppState::MainMenu), (
                effects::despawn_menu_bg_particles_system,
            ))
            .add_systems(Update, (
                spawn::start_game_from_menu.run_if(in_state(ir_core::AppState::MainMenu)),
                hud::handle_main_menu_buttons.run_if(in_state(ir_core::AppState::MainMenu)),
                hud::upgrade_tree::toggle_upgrade_tree_from_menu
                    .run_if(in_state(ir_core::AppState::MainMenu)),
                hud::upgrade_tree::handle_upgrade_card_clicks
                    .run_if(in_state(ir_core::AppState::MainMenu)),
                hud::upgrade_tree::process_purchase_events
                    .run_if(in_state(ir_core::AppState::MainMenu)),
            ))

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
                hud::character_select::handle_char_select_back,
            ).run_if(in_state(ir_core::AppState::CharacterSelect)))
            .add_systems(OnExit(ir_core::AppState::CharacterSelect), (
                hud::character_select::despawn_character_select,
            ))

            // Settings — overlay screen
            .add_systems(OnEnter(ir_core::AppState::Settings), (
                hud::despawn_main_menu,
                hud::spawn_settings_screen,
            ))
            .add_systems(Update, (
                hud::handle_settings_clicks,
                hud::update_settings_screen,
            ).run_if(in_state(ir_core::AppState::Settings)))
            .add_systems(OnExit(ir_core::AppState::Settings), (
                hud::despawn_settings_screen,
            ))

            // World — open world exploration
            .add_systems(OnEnter(ir_core::AppState::World), (
                hud::despawn_main_menu,
                hud::despawn_game_over,
                hud::despawn_pause_overlay,
                hud::despawn_zone_transition,
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
                hud::update_ability_bar.run_if(in_state(ir_core::AppState::World)),
                hud::update_dash_text.run_if(in_state(ir_core::AppState::World)),
                hud::update_gold_text.run_if(in_state(ir_core::AppState::World)),
            ))
            .add_systems(Update, (
                hud::update_inventory.run_if(in_state(ir_core::AppState::World)),
                hud::update_inventory_stack_text.run_if(in_state(ir_core::AppState::World)),
                hud::update_inventory_gold.run_if(in_state(ir_core::AppState::World)),
                hud::toggle_inventory.run_if(in_state(ir_core::AppState::World)),
                hud::handle_inventory_left_click.run_if(in_state(ir_core::AppState::World)),
                hud::handle_inventory_right_click.run_if(in_state(ir_core::AppState::World)),
                hud::update_equipment.run_if(in_state(ir_core::AppState::World)),
                hud::update_gear_score.run_if(in_state(ir_core::AppState::World)),
                hud::handle_equip_slot_click.run_if(in_state(ir_core::AppState::World)),
                hud::update_tooltip.run_if(in_state(ir_core::AppState::World)),
            ))
            .add_systems(Update, (
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::World)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::World)),
                hud::update_minimap.run_if(in_state(ir_core::AppState::World)),
                hud::update_buff_bar.run_if(in_state(ir_core::AppState::World)),
                hud::tick_buff_timers.run_if(in_state(ir_core::AppState::World)),
                hud::spawn_level_up_popup.run_if(in_state(ir_core::AppState::World)),
                hud::update_level_up_popups.run_if(in_state(ir_core::AppState::World)),
                // Zone transition overlay
                hud::spawn_zone_transition_overlay.run_if(in_state(ir_core::AppState::World)),
                hud::update_zone_transition.run_if(in_state(ir_core::AppState::World)),
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
                hud::update_ability_bar.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_dash_text.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_gold_text.run_if(in_state(ir_core::AppState::Dungeon)),
            ))
            .add_systems(Update, (
                hud::update_inventory.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_inventory_stack_text.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_inventory_gold.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::toggle_inventory.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::handle_inventory_left_click.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::handle_inventory_right_click.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_equipment.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_gear_score.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::handle_equip_slot_click.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_tooltip.run_if(in_state(ir_core::AppState::Dungeon)),
            ))
            .add_systems(Update, (
                hud::spawn_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_damage_numbers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_enemy_nameplates.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_minimap.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_buff_bar.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::tick_buff_timers.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::spawn_level_up_popup.run_if(in_state(ir_core::AppState::Dungeon)),
                hud::update_level_up_popups.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Dungeon)),
                assign_shadow.run_if(in_state(ir_core::AppState::Dungeon)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_death_effect.run_if(in_state(ir_core::AppState::Dungeon)),
                spawn_projectile_trail.run_if(in_state(ir_core::AppState::Dungeon)),
                apply_hit_flash_visual.run_if(in_state(ir_core::AppState::Dungeon)),
                restore_hit_flash_visual.run_if(in_state(ir_core::AppState::Dungeon)),
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
                hud::update_minimap.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_buff_bar.run_if(in_state(ir_core::AppState::Playing)),
                hud::tick_buff_timers.run_if(in_state(ir_core::AppState::Playing)),
                hud::spawn_level_up_popup.run_if(in_state(ir_core::AppState::Playing)),
                hud::update_level_up_popups.run_if(in_state(ir_core::AppState::Playing)),
                assign_projectile_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_enemy_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_pickup_mesh.run_if(in_state(ir_core::AppState::Playing)),
                assign_shadow.run_if(in_state(ir_core::AppState::Playing)),
                rotate_billboards.run_if(in_state(ir_core::AppState::Playing)),
                spawn_impact_effect.run_if(in_state(ir_core::AppState::Playing)),
                spawn_death_effect.run_if(in_state(ir_core::AppState::Playing)),
                spawn_projectile_trail.run_if(in_state(ir_core::AppState::Playing)),
                apply_hit_flash_visual.run_if(in_state(ir_core::AppState::Playing)),
                restore_hit_flash_visual.run_if(in_state(ir_core::AppState::Playing)),
                cleanup_lifetime.run_if(in_state(ir_core::AppState::Playing)),
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
                hud::despawn_zone_transition,
                spawn::cleanup_world,
                spawn::despawn_player,
                hud::spawn_game_over_screen,
            ))
            .add_systems(Update, spawn::restart_from_game_over
                .run_if(in_state(ir_core::AppState::GameOver)))

            // Universal tween system — runs in all states
            .add_systems(Update, advance_tweens_system)
            // Button hover highlight — runs in all UI states
            .add_systems(Update, crate::ui_textures::button_hover_system);
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

/// Reads SpawnDeathEffectEvent and spawns a death explosion at the position.
fn spawn_death_effect(
    mut commands: Commands,
    library: Res<EffectsLibrary>,
    mut events: EventReader<SpawnDeathEffectEvent>,
) {
    for event in events.read() {
        effects::spawn_death_explosion(&mut commands, &library, event.position);
    }
}

/// Spawns trail segments behind projectiles every ~50ms.
fn spawn_projectile_trail(
    mut commands: Commands,
    time: Res<Time>,
    trail_assets: Res<TrailAssets>,
    projectiles: Query<&Transform, With<Projectile>>,
    mut accum: Local<f32>,
) {
    *accum += time.delta_secs();
    if *accum < 0.05 {
        return;
    }
    *accum = 0.0;

    for tf in projectiles.iter() {
        commands.spawn((
            Mesh3d(trail_assets.mesh.clone()),
            MeshMaterial3d(trail_assets.material.clone()),
            Transform::from_translation(tf.translation),
            GlobalTransform::default(),
            BillboardSprite,
            Lifetime { remaining: 0.3 },
            TrailSegment,
        ));
    }
}

/// Initializes trail assets (small mesh + material).
fn setup_trail_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Circle::new(0.06));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.8, 1.0),
        emissive: LinearRgba::rgb(0.1, 0.4, 0.6),
        perceptual_roughness: 0.1,
        ..default()
    });
    commands.insert_resource(TrailAssets { mesh, material });
}

/// Applies a bright white emissive to entities that just received a HitFlash.
/// Stores the original material in a FlashMaterial component for later restoration.
fn apply_hit_flash_visual(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &MeshMaterial3d<StandardMaterial>), (With<HitFlash>, Without<FlashMaterial>)>,
) {
    for (entity, mat_handle) in query.iter() {
        if let Some(mat) = materials.get(&mat_handle.0) {
            let mut flash_mat = mat.clone();
            flash_mat.emissive = LinearRgba::rgb(1.0, 1.0, 1.0);
            let flash_handle = materials.add(flash_mat);
            commands.entity(entity).insert((
                MeshMaterial3d(flash_handle),
                FlashMaterial { original: mat_handle.0.clone() },
            ));
        }
    }
}

/// Restores original material when HitFlash has expired.
fn restore_hit_flash_visual(
    mut commands: Commands,
    query: Query<(Entity, &FlashMaterial), (Without<HitFlash>, With<FlashMaterial>)>,
) {
    for (entity, flash_mat) in query.iter() {
        commands.entity(entity).insert(MeshMaterial3d(flash_mat.original.clone()));
        commands.entity(entity).remove::<FlashMaterial>();
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
    projectiles: Query<Entity, (With<ir_core::Projectile>, Without<Mesh3d>)>,
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
fn assign_enemy_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    enemies: Query<(Entity, &ir_core::Enemy), (Without<Mesh3d>, With<ir_core::Enemy>)>,
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
fn assign_pickup_mesh(
    mut commands: Commands,
    assets: Res<ir_core::GameAssets>,
    gems: Query<Entity, (With<ir_core::ExperienceGem>, Without<Mesh3d>)>,
    health_pickups: Query<Entity, (With<ir_core::Pickup>, Without<Mesh3d>, Without<ir_core::ExperienceGem>)>,
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

// ── Tween system ────────────────────────────────────────────────────────────

/// Advances all active Tween components each frame (runs in all states).
pub fn advance_tweens_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut ir_core::tween::Tween,
        Option<&mut Transform>,
        Option<&mut BackgroundColor>,
    )>,
) {
    let dt = time.delta_secs();
    for (entity, mut tween, transform, bg_color) in query.iter_mut() {
        if !tween.playing {
            continue;
        }

        tween.timer += dt;
        let progress = (tween.timer / tween.duration).min(1.0);
        let eased = (tween.easing)(progress);

        match &tween.mode {
            ir_core::tween::TweenMode::Scale { from, to } => {
                if let Some(mut tf) = transform {
                    let scale = from + (to - from) * eased;
                    tf.scale = Vec3::splat(scale);
                }
            }
            ir_core::tween::TweenMode::Fade { from, to } => {
                if let Some(mut bg) = bg_color {
                    let alpha = from + (to - from) * eased;
                    let c = bg.0.to_srgba();
                    bg.0 = Color::srgba(c.red, c.green, c.blue, alpha);
                }
            }
            ir_core::tween::TweenMode::Translate { from, to } => {
                if let Some(mut tf) = transform {
                    tf.translation = *from + (*to - *from) * eased;
                }
            }
            ir_core::tween::TweenMode::Color { from, to } => {
                if let Some(mut bg) = bg_color {
                    let f = from.to_srgba();
                    let t = to.to_srgba();
                    bg.0 = Color::srgba(
                        f.red + (t.red - f.red) * eased,
                        f.green + (t.green - f.green) * eased,
                        f.blue + (t.blue - f.blue) * eased,
                        f.alpha + (t.alpha - f.alpha) * eased,
                    );
                }
            }
            ir_core::tween::TweenMode::Float { .. } => {}
        }

        if progress >= 1.0 {
            if tween.repeat {
                tween.timer = 0.0;
            } else {
                tween.playing = false;
                if matches!(&tween.mode, ir_core::tween::TweenMode::Fade { to, .. } if *to == 0.0) {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
