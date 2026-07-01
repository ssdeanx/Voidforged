//! Item instance — a runtime copy of an item with durability and stack tracking.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::items::ItemRarity;

/// A runtime instance of an item with mutable state.
///
/// References an [`ItemDef`](super::ItemDef) by `def_id` for template data.
/// While the definition is read-only and shared, each `ItemInstance` tracks
/// its own stack count, durability, and rarity override. Used in inventories,
/// equipment slots, and as dropped items in the world.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct ItemInstance {
    /// Matches `ItemDef.id` for template lookup in the database.
    pub def_id: String,
    /// Stack count for consumables and materials (1 for equipment).
    pub stack_count: u16,
    /// Current durability (0.0 = broken, item is unusable).
    pub durability: f32,
    /// Maximum durability (full repair restores to this value).
    pub max_durability: f32,
    /// Instance-level rarity override. Use `None` semantics — set to
    /// the template's rarity by default, but may differ for rolled loot.
    pub rarity: ItemRarity,
}

impl ItemInstance {
    /// Creates a new item instance from a definition ID.
    ///
    /// Initialises with 1 stack, full durability, and Common rarity.
    /// The caller should look up the actual rarity from the definition
    /// if a proper rarity override is needed.
    pub fn new(def_id: &str) -> Self {
        Self {
            def_id: def_id.to_string(),
            stack_count: 1,
            durability: 1.0,
            max_durability: 1.0,
            rarity: ItemRarity::Common,
        }
    }

    /// Creates a stacked instance for consumables and materials.
    pub fn stacked(def_id: &str, count: u16) -> Self {
        let mut item = Self::new(def_id);
        item.stack_count = count;
        item
    }

    /// Returns `true` if the item still has durability above zero.
    pub fn is_usable(&self) -> bool {
        self.durability > 0.0
    }

    /// Reduces durability by the given amount, clamped at zero.
    pub fn damage(&mut self, amount: f32) {
        self.durability = (self.durability - amount).max(0.0);
    }
}
