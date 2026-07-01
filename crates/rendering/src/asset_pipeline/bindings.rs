use bevy::prelude::*;
use std::collections::HashMap;
use crate::asset_pipeline::slots::{ModelCategory, ModelSlot};

#[derive(Resource, Debug, Default)]
pub struct ModelBindingRegistry {
    /// Maps weapon kind names (e.g., "Sword", "Bow") to their model slots.
    pub weapon_bindings: HashMap<String, ModelSlot>,
    /// Maps pickup kind names (e.g., "Health", "Gold") to their model slots.
    pub pickup_bindings: HashMap<String, ModelSlot>,
    /// Per-instance overrides for specific item → model slot mappings.
    pub item_overrides: HashMap<String, ModelSlot>,
}

impl ModelBindingRegistry {
    pub fn bind_weapon(&mut self, weapon_kind: impl Into<String>, slot: ModelSlot) {
        self.weapon_bindings.insert(weapon_kind.into(), slot);
    }
    pub fn bind_pickup(&mut self, pickup_kind: impl Into<String>, slot: ModelSlot) {
        self.pickup_bindings.insert(pickup_kind.into(), slot);
    }
    pub fn weapon_slot(&self, weapon_kind: &str) -> Option<&ModelSlot> {
        self.weapon_bindings.get(weapon_kind)
    }
    pub fn pickup_slot(&self, pickup_kind: &str) -> Option<&ModelSlot> {
        self.pickup_bindings.get(pickup_kind)
    }
}

pub fn register_default_bindings(mut bindings: ResMut<ModelBindingRegistry>) {
    bindings.bind_weapon("Dagger", ModelSlot::new(ModelCategory::Weapon, "Dagger"));
    bindings.bind_weapon("Sword", ModelSlot::new(ModelCategory::Weapon, "Sword"));
    bindings.bind_weapon("Bow", ModelSlot::new(ModelCategory::Weapon, "Bow"));
    bindings.bind_weapon("Staff", ModelSlot::new(ModelCategory::Weapon, "Staff"));
    bindings.bind_weapon("Aura", ModelSlot::new(ModelCategory::Weapon, "Aura"));
    bindings.bind_weapon("Whip", ModelSlot::new(ModelCategory::Weapon, "Whip"));
    bindings.bind_weapon("MagicMissile", ModelSlot::new(ModelCategory::Projectile, "MagicMissile"));
    bindings.bind_pickup("Health", ModelSlot::new(ModelCategory::Pickup, "Health"));
    bindings.bind_pickup("Gold", ModelSlot::new(ModelCategory::Pickup, "Gold"));
    bindings.bind_pickup("TemporaryBoost", ModelSlot::new(ModelCategory::Pickup, "TemporaryBoost"));
}

pub fn auto_bind_item_models(
    mut commands: Commands,
    _bindings: Res<ModelBindingRegistry>,
    registry: Res<crate::asset_pipeline::slots::ModelSlotRegistry>,
    weapons_query: Query<Entity, (With<ir_core::Weapon>, Without<ModelSlot>)>,
    pickups_query: Query<Entity, (With<ir_core::Pickup>, Without<ModelSlot>)>,
) {
    if !registry.has_category(ModelCategory::Weapon.as_str()) {
        for entity in weapons_query.iter() {
            commands.entity(entity).insert(ModelSlot::new(ModelCategory::Weapon, "Sword"));
        }
    }
    if !registry.has_category(ModelCategory::Pickup.as_str()) {
        for entity in pickups_query.iter() {
            commands.entity(entity).insert(ModelSlot::new(ModelCategory::Pickup, "Health"));
        }
    }
}
