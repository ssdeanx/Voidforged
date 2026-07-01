//! In-game HUD layout — coordinator that spawns all WoW-style UI elements.
//!
//! Delegates to sub-modules for player frame, target frame, action bar,
//! nameplates, and the zone tracker.

use bevy::prelude::*;
use crate::hud::components::*;
use crate::hud::{player_frame, target_frame, ability_bar, inventory, equipment, tooltips, minimap, buffs};
use crate::ui_textures::UiTextureAssets;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}

/// Spawns the full in-game HUD — root container and all child elements.
pub fn spawn_hud(mut commands: Commands, assets: Res<UiTextureAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // ── Player Unit Frame (top-left) ────────────────────────
            player_frame::spawn_player_frame(parent, &assets);

            // ── Target Unit Frame (top-center) ──────────────────────
            target_frame::spawn_target_frame(parent, &assets);

            // ── XP Bar (bottom, above action bar) ───────────────────
            spawn_xp_bar(parent, &assets);

            // ── Zone Name / Minimap (pulsing text, bottom-center) ───
            spawn_zone_tracker(parent);

            // ── Interaction prompt (center) ─────────────────────────
            spawn_prompt(parent);

            // ── Action Bar (bottom center) ──────────────────────────
            ability_bar::spawn_action_bar(parent, &assets);

            // ── Buff/Debuff Bar (above XP bar) ─────────────────────
            buffs::spawn_buff_bar(parent);

            // ── Minimap (top-right corner) ──────────────────────────
            minimap::spawn_minimap(parent);

            // ── Inventory panel (right side, hidden by default) ─────
            inventory::spawn_inventory(parent, &assets);

            // ── Equipment screen (right side, hidden by default) ────
            equipment::spawn_equipment(parent, &assets);

            // ── Item tooltip (follows cursor) ──────────────────────
            tooltips::spawn_tooltip(parent);
        });
}

/// Spawns the XP bar — purple gradient bar with text.
fn spawn_xp_bar(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(16.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(76.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-200.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::srgb(0.25, 0.25, 0.35)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
            HudXpBar,
        ))
        .with_children(|xp_bg| {
            // Fill — use XP gradient texture
            xp_bg.spawn((
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ImageNode::new(assets.bar_xp.clone()),
                HudXpBarFill,
            ));
            // Text overlay
            xp_bg
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    HudXpBarText,
                ))
                .with_children(|xp_text| {
                    xp_text.spawn((
                        label("XP: 0/100", 11.0, Color::srgb(0.7, 0.8, 1.0)),
                        HudXpBarText,
                    ));
                });
        });
}

/// Spawns the zone name / minimap area — pulsing colored text.
fn spawn_zone_tracker(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(24.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            HudZoneFrame,
        ))
        .with_children(|zone| {
            zone.spawn((
                label("", 16.0, Color::srgb(0.5, 0.75, 0.35)),
                HudZoneText,
            ));
        });
}

/// Spawns the interaction prompt (center screen).
fn spawn_prompt(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(40.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                ..default()
            },
            HudRoot,
        ))
        .with_children(|prompt| {
            prompt.spawn((
                label("", 18.0, Color::srgb(1.0, 0.9, 0.4)),
                HudPromptText,
            ));
        });
}

/// Despawns the entire HUD.
pub fn despawn_hud(mut commands: Commands, hud: Query<Entity, With<HudRoot>>) {
    for entity in hud.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
