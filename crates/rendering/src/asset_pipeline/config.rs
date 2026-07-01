use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug, Clone)]
pub struct AssetPipelineConfig {
    pub characters: HashMap<String, String>,
    pub enemies: HashMap<String, String>,
    pub weapons: HashMap<String, String>,
    pub pickups: HashMap<String, String>,
    pub environment: HashMap<String, String>,
    pub animations: AnimationConfig,
    pub fallback_model: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AnimationConfig {
    pub idle: HashMap<String, String>,
    pub run: HashMap<String, String>,
    pub attack: HashMap<String, String>,
    pub hit: HashMap<String, String>,
}

impl Default for AssetPipelineConfig {
    fn default() -> Self {
        Self {
            characters: HashMap::new(),
            enemies: HashMap::new(),
            weapons: HashMap::new(),
            pickups: HashMap::new(),
            environment: HashMap::new(),
            animations: AnimationConfig::default(),
            fallback_model: None,
            enabled: true,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            idle: HashMap::new(),
            run: HashMap::new(),
            attack: HashMap::new(),
            hit: HashMap::new(),
        }
    }
}

pub const PIPELINE_CONFIG_PATH: &str = "asset_pipeline.ron";

pub fn load_config() -> AssetPipelineConfig {
    bevy::log::info!("Asset pipeline: using default config (placeholders active).");
    AssetPipelineConfig::default()
}
