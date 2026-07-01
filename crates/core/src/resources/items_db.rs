//! Item database — central registry of all item definitions.

use bevy::prelude::*;
use crate::items::ItemDef;

/// Central registry of all item definitions. Built at startup.
///
/// Populated by [`init_item_database`] during the loading screen.
/// Gameplay systems look up items by ID string to instantiate or inspect them.
#[derive(Resource, Debug, Clone)]
pub struct ItemDatabase {
    /// All registered item definitions.
    pub items: Vec<ItemDef>,
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl ItemDatabase {
    /// Looks up an item definition by its string ID.
    ///
    /// Returns `None` if no item with the given ID is registered.
    pub fn get(&self, id: &str) -> Option<&ItemDef> {
        self.items.iter().find(|i| i.id == id)
    }

    /// Registers a new item definition.
    ///
    /// Called during startup by [`init_item_database`] and may also be
    /// used by mod / content systems at runtime.
    pub fn register(&mut self, def: ItemDef) {
        self.items.push(def);
    }
}

/// Startup system — populates the [`ItemDatabase`] with built-in item definitions.
pub fn init_item_database(mut db: ResMut<ItemDatabase>) {
    for def in crate::items::starter_item_defs() {
        db.register(def);
    }
    info!("Item database initialized with {} items", db.items.len());
}
