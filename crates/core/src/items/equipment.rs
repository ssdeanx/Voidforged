//! Equipment component — currently equipped items on a character.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::items::{EquipSlot, ItemInstance, StatType};
use crate::resources::ItemDatabase;

/// Currently equipped items across 8 gear slots.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: Option<ItemInstance>,
    pub offhand: Option<ItemInstance>,
    pub helmet: Option<ItemInstance>,
    pub chest: Option<ItemInstance>,
    pub boots: Option<ItemInstance>,
    pub ring: Option<ItemInstance>,
    pub amulet: Option<ItemInstance>,
    pub trinket: Option<ItemInstance>,
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            weapon: None, offhand: None,
            helmet: None, chest: None, boots: None,
            ring: None, amulet: None, trinket: None,
        }
    }
}

impl Equipment {
    /// Equip an item into the given slot. Returns the previously equipped item, if any.
    pub fn equip(&mut self, item: ItemInstance, slot: EquipSlot) -> Option<ItemInstance> {
        let target = self.slot_mut(slot);
        let old = target.take();
        *target = Some(item);
        old
    }

    /// Unequip the item in the given slot. Returns the item, if any.
    pub fn unequip(&mut self, slot: EquipSlot) -> Option<ItemInstance> {
        self.slot_mut(slot).take()
    }

    /// Returns a reference to the item in the given slot, if any.
    pub fn get(&self, slot: EquipSlot) -> Option<&ItemInstance> {
        self.slot_ref(slot).as_ref()
    }

    fn slot_mut(&mut self, slot: EquipSlot) -> &mut Option<ItemInstance> {
        match slot {
            EquipSlot::MainHand => &mut self.weapon,
            EquipSlot::OffHand => &mut self.offhand,
            EquipSlot::Helmet => &mut self.helmet,
            EquipSlot::Chest => &mut self.chest,
            EquipSlot::Boots => &mut self.boots,
            EquipSlot::Ring => &mut self.ring,
            EquipSlot::Amulet => &mut self.amulet,
            EquipSlot::Trinket => &mut self.trinket,
        }
    }

    fn slot_ref(&self, slot: EquipSlot) -> &Option<ItemInstance> {
        match slot {
            EquipSlot::MainHand => &self.weapon,
            EquipSlot::OffHand => &self.offhand,
            EquipSlot::Helmet => &self.helmet,
            EquipSlot::Chest => &self.chest,
            EquipSlot::Boots => &self.boots,
            EquipSlot::Ring => &self.ring,
            EquipSlot::Amulet => &self.amulet,
            EquipSlot::Trinket => &self.trinket,
        }
    }

    /// Applies all equipped item stats to a CombatStats struct.
    /// Looks up each ItemInstance's def_id in ItemDatabase and sums StatMod values.
    /// Returns a vec of applied stat changes for logging.
    pub fn apply_stats(&self, db: &ItemDatabase, stats: &mut crate::components::CombatStats) -> Vec<String> {
        let mut changes: Vec<String> = Vec::new();
        let slots = [
            &self.weapon, &self.offhand, &self.helmet, &self.chest,
            &self.boots, &self.ring, &self.amulet, &self.trinket,
        ];

        for item_opt in slots {
            if let Some(item) = item_opt {
                if let Some(def) = db.get(&item.def_id) {
                    for mod_ in &def.base_stats {
                        match mod_.stat {
                            StatType::DamageBonus => stats.damage_bonus += mod_.value,
                            StatType::AttackSpeedBonus => stats.attack_speed_bonus += mod_.value,
                            StatType::Armor => stats.armor += mod_.value,
                            StatType::MaxHealth => stats.max_health_bonus += mod_.value,
                            StatType::MoveSpeed => stats.move_speed_bonus += mod_.value,
                            StatType::CritChance => stats.crit_chance += mod_.value,
                            StatType::CritMultiplier => stats.crit_multiplier += mod_.value,
                            StatType::DodgeChance => stats.dodge_chance += mod_.value,
                            StatType::Lifesteal => stats.lifesteal += mod_.value,
                            StatType::PickupRadius => stats.pickup_radius += mod_.value,
                            StatType::ManaRegen => {} // Not present in CombatStats
                            StatType::ArmorPenetration => stats.armor_penetration += mod_.value,
                        }
                        changes.push(format!("{} +{}", mod_.stat.display_name(), mod_.value));
                    }
                }
            }
        }
        changes
    }
}
