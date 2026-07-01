//! HUD module — all UI screens, overlays, and HUD elements.
//!
//! Split into:
//! - `components`: UI component markers
//! - `menu`: MainMenu, GameOver, Pause screens
//! - `layout`: In-game HUD + ability hotbar
//! - `updates`: HUD update systems (health, xp, level, etc.)
//! - `notifications`: Damage numbers, wave announcements, enemy HP bars

pub mod components;
pub mod menu;
pub mod layout;
pub mod updates;
pub mod notifications;

// Re-exports for backward compatibility with plugin references
pub use layout::{despawn_hud, spawn_hotbar, spawn_hud};
pub use menu::{
    despawn_game_over, despawn_main_menu, despawn_pause_overlay,
    spawn_game_over_screen, spawn_main_menu_screen, spawn_pause_overlay,
};
pub use notifications::{
    spawn_damage_numbers, spawn_wave_announcements, spawn_wave_cleared,
    update_enemy_health_bars, update_wave_announcements,
};
pub use updates::{
    update_dash_text, update_gold_text, update_health_bar, update_level_text,
    update_prompt_text, update_xp_bar, update_zone_text,
};
