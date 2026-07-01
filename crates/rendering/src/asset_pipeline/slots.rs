use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct ModelSlotRegistry {
    pub scenes: HashMap<String, Handle<Scene>>,
}

impl ModelSlotRegistry {
    pub fn get(&self, category: &str, name: &str) -> Option<Handle<Scene>> {
        self.scenes.get(&format!("{category}/{name}")).cloned()
    }
    pub fn has(&self, category: &str, name: &str) -> bool {
        self.scenes.contains_key(&format!("{category}/{name}"))
    }
    pub fn has_category(&self, category: &str) -> bool {
        self.scenes.keys().any(|k| k.starts_with(category))
    }
    pub fn count(&self) -> usize { self.scenes.len() }
}

#[derive(Component, Debug, Clone)]
pub struct ModelSlot {
    pub category: ModelCategory,
    pub name: String,
    pub scale: f32,
    pub spawned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelCategory {
    Character, Enemy, Weapon, Pickup, Environment, Projectile,
}

impl ModelCategory {
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
    pub fn new(category: ModelCategory, name: impl Into<String>) -> Self {
        Self { category, name: name.into(), scale: 1.0, spawned: false }
    }
    pub fn with_scale(mut self, scale: f32) -> Self { self.scale = scale; self }
}

pub enum CharacterModel {
    Warrior, Paladin, Rogue, Hunter, Mage,
}

impl From<CharacterModel> for ModelSlot {
    fn from(cm: CharacterModel) -> Self {
        let name = match cm {
            CharacterModel::Warrior => "Warrior",
            CharacterModel::Paladin => "Paladin",
            CharacterModel::Rogue => "Rogue",
            CharacterModel::Hunter => "Hunter",
            CharacterModel::Mage => "Mage",
        };
        ModelSlot::new(ModelCategory::Character, name)
    }
}

pub enum EnemyModel {
    Grunt, Ranged, Charger, Elite, Boss,
}

impl From<EnemyModel> for ModelSlot {
    fn from(em: EnemyModel) -> Self {
        let name = match em {
            EnemyModel::Grunt => "Grunt",
            EnemyModel::Ranged => "Ranged",
            EnemyModel::Charger => "Charger",
            EnemyModel::Elite => "Elite",
            EnemyModel::Boss => "Boss",
        };
        ModelSlot::new(ModelCategory::Enemy, name)
    }
}
