//! Item database — central registry of all item definitions.

use bevy::prelude::*;
use crate::items::ItemDef;

/// Central registry of all item definitions. Built at startup.
#[derive(Resource, Debug, Clone)]
pub struct ItemDatabase {
    pub items: Vec<ItemDef>,
}

impl Default for ItemDatabase {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl ItemDatabase {
    pub fn get(&self, id: &str) -> Option<&ItemDef> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn register(&mut self, def: ItemDef) {
        self.items.push(def);
    }
}

/// Startup system — populates ItemDatabase with built-in item definitions.
pub fn init_item_database(mut db: ResMut<ItemDatabase>) {
    for def in crate::items::starter_item_defs() {
        db.register(def);
    }
    info!("Item database initialized with {} items", db.items.len());
}
