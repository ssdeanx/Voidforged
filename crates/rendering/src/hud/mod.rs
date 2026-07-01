//! HUD module — all UI screens, overlays, and HUD elements.
//!
//! Split into:
//! - `components`: UI component markers + class color helpers
//! - `menu`: MainMenu, GameOver, Pause screens
//! - `layout`: In-game HUD root coordinator
//! - `player_frame`: WoW-style player unit frame
//! - `target_frame`: Enemy target frame
//! - `ability_bar`: 6-slot action bar with class abilities + cooldowns
//! - `nameplates`: 3D world-space enemy nameplates
//! - `updates`: All HUD update systems (health, xp, resource, etc.)
//! - `notifications`: Damage numbers, wave announcements
//! - `character_select`: Class/name selection screen

pub mod components;
pub mod menu;
pub mod layout;
pub mod player_frame;
pub mod target_frame;
pub mod ability_bar;
pub mod nameplates;
pub mod updates;
pub mod notifications;
pub mod character_select;
pub mod upgrade_tree;
pub mod inventory;
pub mod equipment;
pub mod tooltips;
pub mod minimap;
pub mod buffs;
pub mod settings;
pub mod zone_transition;

// Re-exports for plugin references
pub use layout::{despawn_hud, spawn_hud};
pub use menu::{
    despawn_game_over, despawn_main_menu, despawn_pause_overlay,
    handle_main_menu_buttons,
    spawn_game_over_screen, spawn_main_menu_screen, spawn_pause_overlay,
};
pub use notifications::{
    spawn_damage_numbers, spawn_wave_announcements, spawn_wave_cleared,
    update_damage_numbers, update_wave_announcements,
    spawn_level_up_popup, update_level_up_popups,
};
pub use updates::{
    update_ability_bar, update_dash_text, update_gold_text, update_player_health,
    update_prompt_text, update_resource_bar, update_stamina_bar, update_target_frame,
    update_xp_bar, update_zone_tracker, update_player_frame_class,
};
pub use nameplates::update_enemy_nameplates;
pub use inventory::{
    handle_inventory_left_click, handle_inventory_right_click, toggle_inventory,
    update_inventory, update_inventory_gold, update_inventory_stack_text,
};
pub use equipment::{
    handle_equip_slot_click, update_equipment, update_gear_score,
};
pub use tooltips::update_tooltip;
pub use upgrade_tree::{
    close_upgrade_tree, despawn_upgrade_tree, handle_upgrade_card_clicks,
    process_purchase_events, spawn_upgrade_tree, PurchaseUpgradeEvent,
};

// Minimap
pub use minimap::{spawn_minimap, update_minimap};

// Buff/Debuff
pub use buffs::{spawn_buff_bar, update_buff_bar, tick_buff_timers};

// Settings
pub use settings::{
    despawn_settings_screen, handle_settings_clicks, spawn_settings_screen,
    toggle_settings_from_menu, update_settings_screen,
};

// Zone transition overlay
pub use zone_transition::{
    despawn_zone_transition, spawn_zone_transition_overlay, update_zone_transition,
};
