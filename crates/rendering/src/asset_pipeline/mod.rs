//! Asset pipeline — configuration, loading, binding, animation, and slot management for 3D models.
//!
//! This module coordinates the full asset lifecycle:
//! - [`config`] — RON-based pipeline configuration (which GLTF files to load)
//! - [`loader`] — queuing, polling, and assigning loaded scenes to entities
//! - [`slots`] — model slot registry and category types for scene lookups
//! - [`bindings`] — weapon/pickup → model slot binding registry
//! - [`animation`] — animation state machine and clip playback systems

pub mod animation;
pub mod bindings;
pub mod config;
pub mod loader;
pub mod slots;

use bevy::prelude::*;
use self::animation::tick_animation_state_machine;
use self::bindings::{register_default_bindings, ModelBindingRegistry};
use self::config::AssetPipelineConfig;
use self::loader::{initiate_model_loading, poll_model_loading, ModelLoadQueue};
use self::slots::ModelSlotRegistry;

pub struct AssetPipelinePlugin;

impl Plugin for AssetPipelinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ModelSlotRegistry>()
            .init_resource::<ModelLoadQueue>()
            .init_resource::<ModelBindingRegistry>()
            .init_resource::<AssetPipelineConfig>()
            .add_systems(Startup, (register_default_bindings,))
            .add_systems(OnEnter(ir_core::AppState::Loading), (initiate_model_loading,))
            .add_systems(Update, (poll_model_loading.run_if(in_state(ir_core::AppState::Loading)),))
            .add_systems(Update, (tick_animation_state_machine.run_if(
                in_state(ir_core::AppState::World)
                    .or(in_state(ir_core::AppState::Dungeon))
                    .or(in_state(ir_core::AppState::Playing)),
            ),));
    }
}
