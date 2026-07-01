//! Equipment component — currently equipped items on a character.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::items::{EquipSlot, ItemInstance, StatType};
use crate::resources::ItemDatabase;

/// Currently equipped items across all gear slots.
///
/// Each slot corresponds to an [`EquipSlot`] variant. Equipping replaces
/// the current item; unequipping moves it back to the inventory.
/// [`apply_stats`](Self::apply_stats) sums all equipped item modifiers
/// into a [`CombatStats`](crate::components::CombatStats) struct.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Equipment {
    /// Main-hand weapon slot.
    pub weapon: Option<ItemInstance>,
    /// Off-hand / shield slot.
    pub offhand: Option<ItemInstance>,
    /// Helmet armour slot.
    pub helmet: Option<ItemInstance>,
    /// Chest armour slot.
    pub chest: Option<ItemInstance>,
    /// Boots armour slot.
    pub boots: Option<ItemInstance>,
    /// Ring accessory slot.
    pub ring: Option<ItemInstance>,
    /// Amulet accessory slot.
    pub amulet: Option<ItemInstance>,
    /// Trinket accessory slot.
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
    /// Equips an item into the given slot.
    ///
    /// Returns the previously equipped item in that slot, if any.
    pub fn equip(&mut self, item: ItemInstance, slot: EquipSlot) -> Option<ItemInstance> {
        let target = self.slot_mut(slot);
        let old = target.take();
        *target = Some(item);
        old
    }

    /// Unequips the item in the given slot, returning it (if any).
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

    /// Sums all equipped item stat modifiers into the given `CombatStats`.
    ///
    /// Looks up each item's definition in the database, accumulates its
    /// `base_stats` onto the supplied stats struct, and returns a vector
    /// of human-readable change descriptions for logging / UI.
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
