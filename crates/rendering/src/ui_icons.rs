//! UI Icon asset loading.
//!
//! Loads icon textures from `assets/textures/icons/` and stores
//! them in `UiIconAssets` for use by HUD panels and upgrade screens.
//! Includes both abilities, items, and UI HUD elements.

use bevy::prelude::*;
use std::collections::HashMap;

// ============================================================================
// Resource — all loaded UI icon handles
// ============================================================================

/// Holds handles to every UI icon used by the HUD and upgrade screens.
#[derive(Resource, Debug, Clone, Default)]
pub struct UiIconAssets {
    /// Maps icon IDs (e.g., "icon_hp_up") to their texture handles.
    pub icons: HashMap<&'static str, Handle<Image>>,
}

impl UiIconAssets {
    /// Look up an icon by its ID string. Returns `None` if not found.
    pub fn get(&self, icon_id: &str) -> Option<Handle<Image>> {
        self.icons.get(icon_id).cloned()
    }
}

// ============================================================================
// Startup system — loads icons
// ============================================================================

/// Startup system: loads all UI icons from `assets/textures/icons/`.
pub fn load_ui_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut icons = HashMap::new();
    
    // Categorized list of icons to load
    let icon_ids = vec![
        // Upgrades / Stats
        "icon_hp_up", "icon_dmg_up", "icon_armor_up",
        "icon_speed_up", "icon_crit_up", "icon_lifesteal",
        "icon_xp", "icon_gold", "icon_magnet",
        // Weapons
        "icon_dagger", "icon_bow", "icon_staff",
        // Abilities
        "icon_cleave", "icon_heal_strike", "icon_consecration",
        "icon_backstab", "icon_poison", "icon_trap",
        "icon_fireball", "icon_blink",
        // HUD Elements
        "ui_settings", "ui_map", "ui_inventory", "ui_quest",
        // Status Effects
        "status_burning", "status_poisoned", "status_slowed",
        "status_stunned", "status_chilled", "status_vulnerable",
    ];

    for id in icon_ids {
        let path = format!("textures/icons/{}.png", id);
        icons.insert(id, asset_server.load(path));
    }

    commands.insert_resource(UiIconAssets { icons });
}
