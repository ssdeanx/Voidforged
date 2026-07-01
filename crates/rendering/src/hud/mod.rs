//! HUD module — all UI screens, overlays, and HUD elements.
//!
//! Split into:
//! - `components`: UI component markers
//! - `menu`: MainMenu, GameOver, Pause screens
//! - `layout`: In-game HUD root coordinator
//! - `player_frame`: WoW-style player unit frame
//! - `target_frame`: Enemy target frame
//! - `ability_bar`: 6-slot action bar
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

// Re-exports for plugin references
pub use layout::{despawn_hud, spawn_hud};
pub use menu::{
    despawn_game_over, despawn_main_menu, despawn_pause_overlay,
    spawn_game_over_screen, spawn_main_menu_screen, spawn_pause_overlay,
};
pub use notifications::{
    spawn_damage_numbers, spawn_wave_announcements, spawn_wave_cleared,
    update_damage_numbers, update_wave_announcements,
};
pub use updates::{
    update_dash_text, update_gold_text, update_player_health, update_prompt_text,
    update_resource_bar, update_stamina_bar, update_target_frame, update_xp_bar,
    update_zone_tracker, update_player_frame_class,
};
pub use nameplates::update_enemy_nameplates;
