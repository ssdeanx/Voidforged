use bevy::asset::LoadState;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::asset_pipeline::config::AssetPipelineConfig;
use crate::asset_pipeline::slots::ModelSlotRegistry;
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

pub fn initiate_model_loading(
    asset_server: Res<AssetServer>,
    config: Option<Res<AssetPipelineConfig>>,
    mut load_queue: ResMut<ModelLoadQueue>,
) {
    let Some(config) = config else {
        bevy::log::info!("Asset pipeline: no config, skipping GLTF loading.");
        return;
    };
    if !config.enabled {
        bevy::log::info!("Asset pipeline: disabled, using placeholders.");
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
        let scene_path = if path.contains('#') { path.clone() } else { format!("{path}#Scene0") };
        bevy::log::info!("Asset pipeline: loading '{key}' from '{scene_path}'");
        load_queue.pending.insert(key.clone(), asset_server.load(&scene_path));
    }
    if load_queue.pending.is_empty() {
        bevy::log::warn!("Asset pipeline: config loaded but no models configured.");
    }
}

pub fn poll_model_loading(
    asset_server: Res<AssetServer>,
    mut load_queue: ResMut<ModelLoadQueue>,
    mut registry: ResMut<ModelSlotRegistry>,
) {
    if load_queue.pending.is_empty() { return; }
    let mut ready_keys: Vec<String> = Vec::new();
    let mut failed_keys: Vec<String> = Vec::new();
    for (key, handle) in &load_queue.pending {
        match asset_server.load_state(handle.id()) {
            LoadState::Loaded => ready_keys.push(key.clone()),
            LoadState::Failed(err) => {
                bevy::log::warn!("Asset pipeline: failed '{key}': {err}");
                failed_keys.push(key.clone());
            }
            LoadState::Loading | LoadState::NotLoaded => {}
        }
    }
    for key in ready_keys {
        if let Some(handle) = load_queue.pending.remove(&key) {
            bevy::log::info!("Asset pipeline: '{key}' loaded.");
            load_queue.ready.insert(key.clone(), handle);
        }
    }
    for key in failed_keys {
        let _ = load_queue.pending.remove(&key);
    }
    if load_queue.pending.is_empty() {
        for (key, handle) in load_queue.ready.drain() {
            registry.scenes.insert(key, handle);
        }
        bevy::log::info!(
            "Asset pipeline: all done ({} ok, {} failed).",
            registry.scenes.len(), load_queue.failed.len()
        );
    }
}

pub fn all_models_loaded(load_queue: Res<ModelLoadQueue>) -> bool {
    load_queue.pending.is_empty()
}
