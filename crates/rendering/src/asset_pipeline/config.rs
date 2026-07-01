use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime asset pipeline configuration — holds resolved model→path mappings.
///
/// Populated from `assets/asset_pipeline.ron` at load time. If the file is
/// missing a default config is used (all categories empty, placeholders active).
/// Edit the RON file to point to your Blender GLTF exports.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct AssetPipelineConfig {
    /// Map of character class name → GLTF path, e.g. "Warrior" → "models/warrior.glb"
    #[serde(default)]
    pub characters: HashMap<String, String>,
    /// Map of enemy variant name → GLTF path, e.g. "Grunt" → "models/grunt.glb"
    #[serde(default)]
    pub enemies: HashMap<String, String>,
    /// Map of weapon kind → GLTF path, e.g. "Sword" → "models/sword.glb"
    #[serde(default)]
    pub weapons: HashMap<String, String>,
    /// Map of pickup kind → GLTF path, e.g. "Health" → "models/health_pickup.glb"
    #[serde(default)]
    pub pickups: HashMap<String, String>,
    /// Map of environment name → GLTF path, e.g. "tree" → "models/tree.glb"
    #[serde(default)]
    pub environment: HashMap<String, String>,
    /// Animation clip paths per state
    #[serde(default)]
    pub animations: AnimationConfig,
    /// Fallback model path when a specific model is missing
    pub fallback_model: Option<String>,
    /// Set to `false` to keep using placeholder quads even when models are configured
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool { true }

/// Animation clip mapping — each state maps animation name → GLTF animation path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Animation clips keyed by name for idle state, e.g. "Warrior" → "models/warrior.glb#Animation0"
    #[serde(default)]
    pub idle: HashMap<String, String>,
    /// Animation clips for run state
    #[serde(default)]
    pub run: HashMap<String, String>,
    /// Animation clips for attack state
    #[serde(default)]
    pub attack: HashMap<String, String>,
    /// Animation clips for hit/react state
    #[serde(default)]
    pub hit: HashMap<String, String>,
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

/// Relative path (from project root / assets dir) to the pipeline RON config.
pub const PIPELINE_CONFIG_PATH: &str = "asset_pipeline.ron";

/// Tries to load `assets/{PIPELINE_CONFIG_PATH}` as RON, writing a default
/// template if the file doesn't exist so the user can edit it.
///
/// Called once during `OnEnter(AppState::Loading)` before model loading begins.
pub fn load_or_create_config(
    mut config: ResMut<AssetPipelineConfig>,
) {
    let asset_path = format!("assets/{PIPELINE_CONFIG_PATH}");
    match std::fs::read_to_string(&asset_path) {
        Ok(contents) => {
            match ron::from_str::<AssetPipelineConfig>(&contents) {
                Ok(parsed) => {
                    *config = parsed;
                    bevy::log::info!(
                        "Asset pipeline: loaded config from '{asset_path}' ({} entries).",
                        config.characters.len()
                            + config.enemies.len()
                            + config.weapons.len()
                            + config.pickups.len()
                            + config.environment.len()
                    );
                }
                Err(e) => {
                    bevy::log::warn!(
                        "Asset pipeline: failed to parse '{asset_path}': {e}. Using defaults."
                    );
                }
            }
        }
        Err(_) => {
            // File doesn't exist — write a template for the user
            let default_config = generate_template_config();
            if let Ok(ron_str) = ron::ser::to_string_pretty(
                &default_config,
                ron::ser::PrettyConfig::new().struct_names(true),
            ) {
                if std::fs::write(&asset_path, &ron_str).is_ok() {
                    bevy::log::info!(
                        "Asset pipeline: no config found — wrote default template to '{asset_path}'."
                    );
                } else {
                    bevy::log::info!(
                        "Asset pipeline: no config found (could not write template), using defaults."
                    );
                }
            }
            *config = default_config;
        }
    }
}

/// Generates a documented template showing the expected structure.
fn generate_template_config() -> AssetPipelineConfig {
    let mut characters = HashMap::new();
    characters.insert("Warrior".into(), "models/warrior.glb".into());
    characters.insert("Paladin".into(), "models/paladin.glb".into());
    characters.insert("Rogue".into(), "models/rogue.glb".into());
    characters.insert("Hunter".into(), "models/hunter.glb".into());
    characters.insert("Mage".into(), "models/mage.glb".into());

    let mut enemies = HashMap::new();
    enemies.insert("Grunt".into(), "models/enemies/grunt.glb".into());
    enemies.insert("Ranged".into(), "models/enemies/ranged.glb".into());
    enemies.insert("Charger".into(), "models/enemies/charger.glb".into());
    enemies.insert("Elite".into(), "models/enemies/elite.glb".into());
    enemies.insert("Boss".into(), "models/enemies/boss.glb".into());

    let mut weapons = HashMap::new();
    weapons.insert("Dagger".into(), "models/weapons/dagger.glb".into());
    weapons.insert("Sword".into(), "models/weapons/sword.glb".into());
    weapons.insert("Bow".into(), "models/weapons/bow.glb".into());
    weapons.insert("Staff".into(), "models/weapons/staff.glb".into());
    weapons.insert("Aura".into(), "models/weapons/aura.glb".into());
    weapons.insert("Whip".into(), "models/weapons/whip.glb".into());
    weapons.insert("MagicMissile".into(), "models/weapons/magic_missile.glb".into());

    let mut pickups = HashMap::new();
    pickups.insert("Health".into(), "models/pickups/health.glb".into());
    pickups.insert("Gold".into(), "models/pickups/gold.glb".into());
    pickups.insert("TemporaryBoost".into(), "models/pickups/temp_boost.glb".into());

    let mut idle_anims = HashMap::new();
    idle_anims.insert("Warrior".into(), "models/warrior.glb#Animation0".into());
    idle_anims.insert("Mage".into(), "models/mage.glb#Animation0".into());
    let mut run_anims = HashMap::new();
    run_anims.insert("Warrior".into(), "models/warrior.glb#Animation1".into());
    run_anims.insert("Mage".into(), "models/mage.glb#Animation1".into());
    let mut attack_anims = HashMap::new();
    attack_anims.insert("Sword".into(), "models/weapons/sword.glb#Animation0".into());
    let mut hit_anims = HashMap::new();
    hit_anims.insert("default".into(), "models/shared/hit.glb#Animation0".into());

    AssetPipelineConfig {
        characters,
        enemies,
        weapons,
        pickups,
        environment: HashMap::new(),
        animations: AnimationConfig {
            idle: idle_anims,
            run: run_anims,
            attack: attack_anims,
            hit: hit_anims,
        },
        fallback_model: Some("models/fallback.glb".into()),
        enabled: true,
    }
}
