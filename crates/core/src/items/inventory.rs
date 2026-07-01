//! Inventory component — slot-based item storage attached to entities.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::items::ItemInstance;

/// Bag/inventory attached to the player or a storage entity.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Inventory {
    /// Slot storage. `None` = empty slot.
    pub slots: Vec<Option<ItemInstance>>,
    /// Maximum number of slots.
    pub max_slots: usize,
    /// Currency held by this inventory.
    pub gold: u64,
}

impl Inventory {
    /// Creates an inventory with `max_slots` empty slots.
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: vec![None; max_slots],
            max_slots,
            gold: 0,
        }
    }

    /// Adds an item to the first empty slot. Returns `false` if inventory is full.
    pub fn add_item(&mut self, item: ItemInstance) -> bool {
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(item);
                return true;
            }
        }
        false
    }

    /// Removes and returns the item at `index`, if any.
    pub fn remove_item(&mut self, index: usize) -> Option<ItemInstance> {
        if index < self.slots.len() {
            self.slots[index].take()
        } else {
            None
        }
    }

    /// Returns a reference to the item at `index`, if any.
    pub fn get(&self, index: usize) -> Option<&ItemInstance> {
        self.slots.get(index).and_then(|s| s.as_ref())
    }

    /// Returns a mutable reference to the item at `index`, if any.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ItemInstance> {
        self.slots.get_mut(index).and_then(|s| s.as_mut())
    }

    /// Returns `true` if there's at least one empty slot.
    pub fn has_space(&self) -> bool {
        self.slots.iter().any(|s| s.is_none())
    }

    /// Returns the number of occupied slots.
    pub fn used_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Checks if a specific item exists in the inventory.
    pub fn contains_def(&self, def_id: &str) -> bool {
        self.slots.iter().any(|s| s.as_ref().is_some_and(|i| i.def_id == def_id))
    }

    // ── Gold operations ────────────────────────────────────────

    pub fn add_gold(&mut self, amount: u64) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn remove_gold(&mut self, amount: u64) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
}
