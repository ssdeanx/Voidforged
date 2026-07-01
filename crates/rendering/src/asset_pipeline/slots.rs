use bevy::prelude::*;
use std::collections::HashMap;

/// Registry of loaded 3D scenes (organized by category/name key).
///
/// Populated by `ModelLoadQueue` once all assets finish loading.
/// Downstream systems check this registry to decide whether to
/// spawn a GLTF scene or fall back to placeholder quads.
#[derive(Resource, Debug, Default)]
pub struct ModelSlotRegistry {
    /// Loaded scene handles, keyed by `"category/name"`.
    pub scenes: HashMap<String, Handle<Scene>>,
}

impl ModelSlotRegistry {
    /// Look up a loaded scene by category + name.
    /// Returns `None` if the key isn't loaded yet (still pending or failed).
    pub fn get(&self, category: &str, name: &str) -> Option<Handle<Scene>> {
        self.scenes.get(&format!("{category}/{name}")).cloned()
    }

    /// Returns true if a specific key is loaded.
    pub fn has(&self, category: &str, name: &str) -> bool {
        self.scenes.contains_key(&format!("{category}/{name}"))
    }

    /// Returns true if any scenes in a category are loaded.
    pub fn has_category(&self, category: &str) -> bool {
        self.scenes.keys().any(|k| k.starts_with(category))
    }

    /// Number of loaded scene entries across all categories.
    pub fn count(&self) -> usize {
        self.scenes.len()
    }

    /// Spawn a GLTF scene from the registry as a child of `commands`.
    ///
    /// Returns `Some(entity)` if the scene handle exists, `None` otherwise.
    /// The caller can match on the result to decide whether to spawn a
    /// placeholder quad instead.
    pub fn spawn_scene(
        &self,
        commands: &mut Commands,
        category: &str,
        name: &str,
        transform: Transform,
    ) -> Option<Entity> {
        let handle = self.get(category, name)?;
        Some(
            commands
                .spawn((
                    SceneRoot(handle),
                    transform,
                    Visibility::default(),
                ))
                .id(),
        )
    }
}

/// Component that marks an entity as having a specific model slot.
///
/// When `ModelSlotRegistry` has a matching entry, the visual system
/// spawns a `SceneRoot` instead of a placeholder mesh. This makes
/// the slot system a drop-in replacement — remove the placeholder
/// and the GLTF model appears automatically.
#[derive(Component, Debug, Clone)]
pub struct ModelSlot {
    /// High-level category (Character, Enemy, Weapon, etc.).
    pub category: ModelCategory,
    /// Specific model name within the category (e.g., "Warrior", "Sword").
    pub name: String,
    /// Uniform scale applied to the loaded scene.
    pub scale: f32,
    /// Whether the scene has already been spawned for this slot.
    pub spawned: bool,
}

/// High-level model categories — maps one-to-one to config sections.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelCategory {
    /// Playable character models.
    Character,
    /// Enemy variant models.
    Enemy,
    /// Weapon models (attached to characters).
    Weapon,
    /// Pickup item models (health, gold, etc.).
    Pickup,
    /// Environment / scenery models.
    Environment,
    /// Projectile models (magic missiles, arrows, etc.).
    Projectile,
}

impl ModelCategory {
    /// Returns the config key string used in `ModelSlotRegistry` lookups.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Character => "character",
            Self::Enemy => "enemy",
            Self::Weapon => "weapon",
            Self::Pickup => "pickup",
            Self::Environment => "env",
            Self::Projectile => "projectile",
        }
    }
}

impl std::fmt::Display for ModelCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ModelSlot {
    /// Creates a new `ModelSlot` with default `scale: 1.0` and `spawned: false`.
    pub fn new(category: ModelCategory, name: impl Into<String>) -> Self {
        Self {
            category,
            name: name.into(),
            scale: 1.0,
            spawned: false,
        }
    }

    /// Sets a custom scale factor for this slot (builder pattern).
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// Character model enum — converts to `ModelSlot`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterModel {
    /// Warrior class (sword-and-board melee).
    Warrior,
    /// Paladin class (holy warrior).
    Paladin,
    /// Rogue class (stealthy melee).
    Rogue,
    /// Hunter class (ranged bow user).
    Hunter,
    /// Mage class (spellcaster).
    Mage,
}

impl CharacterModel {
    /// All character class variants.
    pub fn all() -> &'static [CharacterModel] {
        &[
            CharacterModel::Warrior,
            CharacterModel::Paladin,
            CharacterModel::Rogue,
            CharacterModel::Hunter,
            CharacterModel::Mage,
        ]
    }

    /// Human-readable display name (same as config key).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Warrior => "Warrior",
            Self::Paladin => "Paladin",
            Self::Rogue => "Rogue",
            Self::Hunter => "Hunter",
            Self::Mage => "Mage",
        }
    }
}

impl From<CharacterModel> for ModelSlot {
    fn from(cm: CharacterModel) -> Self {
        ModelSlot::new(ModelCategory::Character, cm.as_str())
    }
}

/// Enemy model enum — converts to `ModelSlot`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyModel {
    /// Basic melee enemy.
    Grunt,
    /// Ranged / projectile enemy.
    Ranged,
    /// Fast charge-attack enemy.
    Charger,
    /// High-stat elite enemy.
    Elite,
    /// Boss enemy (unique mechanics).
    Boss,
}

impl EnemyModel {
    /// All enemy variant names.
    pub fn all() -> &'static [EnemyModel] {
        &[
            EnemyModel::Grunt,
            EnemyModel::Ranged,
            EnemyModel::Charger,
            EnemyModel::Elite,
            EnemyModel::Boss,
        ]
    }

    /// Human-readable display name (same as config key).
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Grunt => "Grunt",
            Self::Ranged => "Ranged",
            Self::Charger => "Charger",
            Self::Elite => "Elite",
            Self::Boss => "Boss",
        }
    }
}

impl From<EnemyModel> for ModelSlot {
    fn from(em: EnemyModel) -> Self {
        ModelSlot::new(ModelCategory::Enemy, em.as_str())
    }
}
