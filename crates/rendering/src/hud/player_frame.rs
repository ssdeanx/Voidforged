//! Player unit frame – WoW-style class-colored portrait + name + level
//! + smooth health bar (lerp) + class resource bar + stamina bar.
//!
//! Spawned inside the HUD root at top-left. Updated every frame by
//! the update systems in updates.rs.

use bevy::prelude::*;
use ir_core::CharacterClass;
use crate::hud::components::*;
use crate::ui_textures::UiTextureAssets;

/// Helper to build a text label bundle for UI text.
fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

/// Spawns the complete WoW-style player unit frame at top-left of the HUD.
pub fn spawn_player_frame(parent: &mut ChildBuilder, assets: &UiTextureAssets) {
    let class = CharacterClass::Warrior; // will be overridden by update system

    // Outer frame container – class-colored border with 9-slice border texture
    parent
        .spawn((
            Node {
                width: Val::Px(310.0),
                height: Val::Px(110.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(3.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(class_border_glow(class)),
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
            HudPlayerFrame,
        ))
        .with_children(|frame| {
            // ── Portrait (class-colored square) ─────────────────────┐
            frame.spawn((
                Node {
                    width: Val::Px(52.0),
                    height: Val::Px(52.0),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::right(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(class_primary_color(class)),
                BorderColor(class_border_glow(class)),
                HudPlayerPortrait,
            ));

            // ── Info column (name, level, bars) ─────────────────────┐
            frame
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(2.0),
                        width: Val::Px(240.0),
                        ..default()
                    },
                    HudPlayerFrame,
                ))
                .with_children(|info| {
                    // Name + Level row
                    info.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        HudPlayerFrame,
                    )).with_children(|nl| {
                        nl.spawn((label("Player", 15.0, Color::WHITE), HudPlayerNameText));
                        nl.spawn((label("Lv. 1", 13.0, Color::srgb(0.7, 0.5, 1.0)), HudPlayerLevelText));
                    });

                    // ── Health bar (WoW-style smooth fill) ──────────┐
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(18.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                        BackgroundColor(Color::srgb(0.15, 0.04, 0.04)),
                        HudHealthBarBorder,
                    )).with_children(|hp_bg| {
                        // Fill bar – uses health gradient texture, width driven by update system
                        hp_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ImageNode::new(assets.bar_health.clone()),
                            HudHealthBar,
                            HudHealthDisplay::default(),
                        ));
                        // HP text overlay (centered)
                        hp_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudHealthBarText,
                        )).with_children(|hp_text| {
                            hp_text.spawn((
                                label("100/100", 13.0, Color::WHITE),
                                HudHealthBarText,
                            ));
                        });
                    });

                    // ── Class resource bar ───────────────────────────┐
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(10.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.25, 0.25, 0.3)),
                        BackgroundColor(Color::srgb(0.08, 0.08, 0.1)),
                        HudResourceBar,
                    )).with_children(|res_bg| {
                        res_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.75, 0.10, 0.10)), // Rage default
                            HudResourceBarFill,
                            HudResourceDisplay::default(),
                        ));
                        res_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudResourceBarText,
                        )).with_children(|res_text| {
                            res_text.spawn((
                                label("Rage 0/100", 9.0, Color::WHITE),
                                HudResourceBarText,
                            ));
                        });
                    });

                    // ── Stamina bar ──────────────────────────────────┐
                    info.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(8.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor(Color::srgb(0.3, 0.3, 0.25)),
                        BackgroundColor(Color::srgb(0.1, 0.08, 0.03)),
                        HudStaminaBar,
                    )).with_children(|stam_bg| {
                        stam_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ImageNode::new(assets.bar_stamina.clone()),
                            HudStaminaBarFill,
                        ));
                        stam_bg.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            HudStaminaBarText,
                        )).with_children(|stam_text| {
                            stam_text.spawn((
                                label("100/100", 8.0, Color::WHITE),
                                HudStaminaBarText,
                            ));
                        });
                    });
                });
        });
}
