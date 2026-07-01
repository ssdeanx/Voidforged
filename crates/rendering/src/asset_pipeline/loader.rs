use bevy::asset::LoadState;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::asset_pipeline::config::AssetPipelineConfig;
<<<<<<< HEAD
use crate::asset_pipeline::slots::ModelSlotRegistry;
=======
use crate::asset_pipeline::slots::{ModelCategory, ModelSlot, ModelSlotRegistry};

>>>>>>> origin/master
/// Tracks the loading progress of queued GLTF scenes.
#[derive(Resource, Debug, Default)]
pub struct ModelLoadQueue {
    /// Models still being loaded (pending asset handles).
    pub pending: HashMap<String, Handle<Scene>>,
    /// Models that have finished loading successfully.
    pub ready: HashMap<String, Handle<Scene>>,
    /// Models whose loading failed (key → error message).
    pub failed: HashMap<String, String>,
}

/// Queues all configured models for loading via `AssetServer`.
///
/// Reads from `AssetPipelineConfig` and creates load handles for each
/// entry across all categories. The handles are stored in `ModelLoadQueue`
/// and polled by `poll_model_loading`.
pub fn initiate_model_loading(
    asset_server: Res<AssetServer>,
    config: Option<Res<AssetPipelineConfig>>,
    mut load_queue: ResMut<ModelLoadQueue>,
) {
    let Some(config) = config else {
        bevy::log::info!("Asset pipeline: no config resource, skipping GLTF loading.");
        return;
    };
    if !config.enabled {
        bevy::log::info!("Asset pipeline: disabled, using placeholder quads.");
        return;
    }

    let mut all_models: HashMap<String, String> = HashMap::new();
    for (class, path) in &config.characters {
        all_models.insert(format!("character/{class}"), path.clone());
    }
    for (variant, path) in &config.enemies {
        all_models.insert(format!("enemy/{variant}"), path.clone());
    }
    for (kind, path) in &config.weapons {
        all_models.insert(format!("weapon/{kind}"), path.clone());
    }
    for (kind, path) in &config.pickups {
        all_models.insert(format!("pickup/{kind}"), path.clone());
    }
    for (name, path) in &config.environment {
        all_models.insert(format!("env/{name}"), path.clone());
    }

    for (key, path) in &all_models {
        let scene_path = if path.contains('#') {
            path.clone()
        } else {
            format!("{path}#Scene0")
        };
        bevy::log::info!("Asset pipeline: queuing '{key}' from '{scene_path}'");
        load_queue
            .pending
            .insert(key.clone(), asset_server.load(&scene_path));
    }

    if load_queue.pending.is_empty() {
        bevy::log::warn!("Asset pipeline: config loaded but no models configured.");
    }
}

/// Polls pending model loads and moves completed/failed handles.
///
/// Once all pending loads finish, scenes are transferred to
/// `ModelSlotRegistry` and the asset pipeline is ready for use.
pub fn poll_model_loading(
    asset_server: Res<AssetServer>,
    mut load_queue: ResMut<ModelLoadQueue>,
    mut registry: ResMut<ModelSlotRegistry>,
) {
    if load_queue.pending.is_empty() {
        return;
    }

    let mut ready_keys: Vec<String> = Vec::new();
    let mut failed_keys: Vec<String> = Vec::new();

    for (key, handle) in &load_queue.pending {
        match asset_server.load_state(handle.id()) {
            LoadState::Loaded => ready_keys.push(key.clone()),
            LoadState::Failed(err) => {
                bevy::log::warn!("Asset pipeline: failed to load '{key}': {err}");
                failed_keys.push(key.clone());
            }
            LoadState::Loading | LoadState::NotLoaded => {}
        }
    }

    for key in &ready_keys {
        if let Some(handle) = load_queue.pending.remove(key) {
            bevy::log::info!("Asset pipeline: '{key}' loaded successfully.");
            load_queue.ready.insert(key.clone(), handle);
        }
    }
    for key in &failed_keys {
        load_queue.pending.remove(key);
        load_queue.failed.insert(key.clone(), "load_failed".into());
    }

    if load_queue.pending.is_empty() {
        // Transfer all ready scenes to the registry
        for (key, handle) in load_queue.ready.drain() {
            registry.scenes.insert(key, handle);
        }
        bevy::log::info!(
            "Asset pipeline: all models loaded ({} ok, {} failed). Placeholder fallback active for missing entries.",
            registry.scenes.len(),
            load_queue.failed.len(),
        );
    }
}

/// Returns `true` when the model load queue is empty (all done or failed).
pub fn all_models_loaded(load_queue: Res<ModelLoadQueue>) -> bool {
    load_queue.pending.is_empty()
}

/// Drop-in replacement system: assigns visual components to entities that
/// have a `ModelSlot` tag but no visual representation yet.
///
/// **Priority order:**
/// 1. If `ModelSlotRegistry` has a loaded scene → adds `SceneRoot(handle)`
/// 2. Otherwise → no-op (the existing placeholder-assignment systems fill in
///    Mesh3d/MeshMaterial3d as before)
///
/// When the user configures real GLTF paths in `asset_pipeline.ron`, this
/// system automatically switches from colored quads to 3D scenes with no
/// other code changes — true drop-in replacement.
pub fn assign_scene_from_slot(
    mut commands: Commands,
    registry: Res<ModelSlotRegistry>,
    query: Query<(Entity, &ModelSlot), (Without<SceneRoot>, Without<Mesh3d>)>,
) {
    for (entity, slot) in query.iter() {
        let category = slot.category.as_str();
        let name = &slot.name;
        if let Some(handle) = registry.get(category, name) {
            commands.entity(entity).insert((
                SceneRoot(handle),
                Transform::from_scale(Vec3::splat(slot.scale)),
            ));
            bevy::log::trace!(
                "Asset pipeline: assigned SceneRoot for '{}/{}'",
                category,
                name
            );
        }
    }
}

/// Maps an `ir_core::CharacterClass` to the corresponding `ModelSlot`.
///
/// Used by `spawn_player()` to tag the entity so `assign_scene_from_slot`
/// can pick it up once the model is loaded.
pub fn slot_for_class(class: &ir_core::CharacterClass) -> ModelSlot {
    use ir_core::CharacterClass;
    let name = match class {
        CharacterClass::Warrior => "Warrior",
        CharacterClass::Paladin => "Paladin",
        CharacterClass::Rogue => "Rogue",
        CharacterClass::Hunter => "Hunter",
        CharacterClass::Mage => "Mage",
    };
    ModelSlot::new(ModelCategory::Character, name)
}

/// Maps an `ir_core::EnemyVariant` to the corresponding `ModelSlot`.
pub fn slot_for_enemy_variant(variant: &ir_core::EnemyVariant) -> ModelSlot {
    use ir_core::EnemyVariant;
    let name = match variant {
        EnemyVariant::Grunt => "Grunt",
        EnemyVariant::Ranged => "Ranged",
        EnemyVariant::Charger => "Charger",
        EnemyVariant::Elite => "Elite",
        EnemyVariant::Boss => "Boss",
    };
    ModelSlot::new(ModelCategory::Enemy, name)
}

/// Maps a `WeaponKind` to the corresponding `ModelSlot` for weapon attachments.
pub fn slot_for_weapon(kind: &ir_core::WeaponKind) -> ModelSlot {
    use ir_core::WeaponKind;
    let name = match kind {
        WeaponKind::Dagger => "Dagger",
        WeaponKind::Sword => "Sword",
        WeaponKind::Bow => "Bow",
        WeaponKind::Staff => "Staff",
        WeaponKind::Aura => "Aura",
        WeaponKind::Whip => "Whip",
        WeaponKind::MagicMissile => "MagicMissile",
    };
    ModelSlot::new(ModelCategory::Weapon, name)
}
