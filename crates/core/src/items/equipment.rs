//! Equipment component — currently equipped items on a character.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::items::{EquipSlot, ItemInstance};

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
    pub fn apply_stats(&self, _stats: &mut crate::components::CombatStats) {
        // TODO: Iterate equipped items, sum StatMod values, apply to stats
    }
}
